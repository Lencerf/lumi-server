use lumi::{Error, Ledger, Transaction, TxnFlag};
use lumi_server_defs::{FilterOptions, JournalItem, TrieOptions, balance_sheet_to_list, build_trie_table};
use rust_decimal::Decimal;
use std::sync::Arc;
use std::{collections::HashMap, convert::Infallible};
use tokio::sync::RwLock;

pub async fn trie(
    root_account: String,
    options: TrieOptions,
    ledger: Arc<RwLock<Ledger>>,
) -> Result<impl warp::Reply, Infallible> {
    let ledger = ledger.read().await;
    let trie_table = build_trie_table(&ledger, &root_account, options);
    let result = trie_table.unwrap_or_default();
    Ok(warp::reply::json(&result))
}

pub async fn errors(errors: Arc<RwLock<Vec<Error>>>) -> Result<impl warp::Reply, Infallible> {
    let errors = errors.read().await;
    Ok(warp::reply::json(&*errors))
}

pub async fn balances(ledger: Arc<RwLock<Ledger>>) -> Result<impl warp::Reply, Infallible> {
    let ledger = ledger.read().await;
    Ok(warp::reply::json(&balance_sheet_to_list(
        ledger.balance_sheet(),
    )))
}

fn filter_account(txn: &Transaction, account: &str) -> bool {
    for posting in txn.postings() {
        if posting.account.starts_with(account) {
            return true;
        }
    }
    false
}

fn update_balance<'t>(
    txn: &'t Transaction,
    account: &str,
    running_balance: &mut HashMap<&'t str, Decimal>,
) -> HashMap<&'t str, Decimal> {
    if txn.flag() == TxnFlag::Balance {
        return HashMap::new();
    }
    let mut changes: HashMap<&str, Decimal> = HashMap::new();
    for posting in txn.postings().iter() {
        if posting.cost.is_none() && posting.account.starts_with(&account) {
            *changes.entry(posting.amount.currency.as_str()).or_default() += posting.amount.number;
        }
    }
    for (c, n) in changes.iter() {
        *running_balance.entry(c).or_default() += n;
    }
    changes
}

pub async fn account_journal(
    account: Option<String>,
    options: FilterOptions,
    ledger: Arc<RwLock<Ledger>>,
) -> Result<impl warp::Reply, Infallible> {
    let ledger = ledger.read().await;
    let mut filters: Vec<Box<dyn Fn(&Transaction) -> bool>> = Vec::new();
    if let Some(ref account) = account {
        filters.push(Box::new(move |txn: &Transaction| {
            filter_account(txn, account)
        }));
    }
    if let Some(account) = &options.account {
        filters.push(Box::new(move |txn: &Transaction| {
            filter_account(txn, account)
        }));
    };
    let txns: Vec<_> = ledger
        .txns()
        .iter()
        .filter(|t| {
            for filter in filters.iter() {
                if !filter(t) {
                    return false;
                }
            }
            true
        })
        .collect();
    let total_number = txns.len();
    let page = std::cmp::max(options.page.unwrap_or(1), 1);
    let entries = std::cmp::max(options.entries.unwrap_or(50), 1);
    let old_first = options.old_first.unwrap_or(false);
    if (page - 1) * entries >= txns.len() {
        Ok(warp::reply::json(&(
            Vec::<Transaction>::new(),
            total_number,
        )))
    } else {
        let num_skip = if old_first {
            (page - 1) * entries
        } else {
            if page * entries >= txns.len() {
                0
            } else {
                txns.len() - page * entries
            }
        };
        let mut running_balance: HashMap<&str, Decimal> = HashMap::new();
        if let Some(ref account) = account {
            for txn in txns.iter().take(num_skip) {
                let _ = update_balance(txn, &account, &mut running_balance);
            }
        }
        let num_take = if old_first {
            std::cmp::min(entries, txns.len() - entries * (page - 1))
        } else {
            (txns.len() - entries * (page - 1)) - num_skip
        };
        let mut items: Vec<_> = txns
            .into_iter()
            .skip(num_skip)
            .take(num_take)
            .map(|txn| {
                if let Some(ref account) = account {
                    let changes = update_balance(txn, &account, &mut running_balance);
                    JournalItem {
                        txn,
                        balance: running_balance.clone(),
                        changes,
                    }
                } else {
                    JournalItem {
                        txn,
                        balance: HashMap::new(),
                        changes: HashMap::new(),
                    }
                }
            })
            .collect();
        if !old_first {
            items.reverse();
        }
        Ok(warp::reply::json(&(items, total_number)))
    }
}
