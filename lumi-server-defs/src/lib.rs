use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    hash::Hash,
};

use lumi::{BalanceSheet, Currency, Ledger, UnitCost};
use rust_decimal::prelude::Zero;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Position {
    pub currency: Currency,
    pub number: Decimal,
    pub cost: Option<UnitCost>,
}

pub const DEFAULT_ENTRIES_PER_PAGE: usize = 50;
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Default)]
#[serde(default)]
pub struct FilterOptions {
    pub entries: Option<usize>,
    pub page: Option<usize>,
    pub old_first: Option<bool>,
    pub account: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Default)]
#[serde(default)]
pub struct TrieOptions {
    pub show_closed: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct TrieNode<S: Eq + Hash> {
    pub numbers: HashMap<S, Decimal>,
    pub nodes: HashMap<S, TrieNode<S>>,
}

pub fn build_trie<'s>(
    ledger: &'s Ledger,
    root_account: &str,
    options: TrieOptions,
) -> (TrieNode<&'s str>, HashSet<&'s str>) {
    let show_closed = options.show_closed.unwrap_or(false);
    let mut root_node = TrieNode::default();
    let mut currencies = HashSet::new();
    for (account, account_map) in ledger.balance_sheet() {
        if ledger.accounts()[account].close().is_some() && !show_closed {
            continue;
        }
        let mut parts = account.split(':');
        if parts.next() != Some(&root_account) {
            continue;
        }
        let mut account_holdings: HashMap<&'s str, Decimal> = HashMap::new();
        for (currency, cost_map) in account_map {
            for (cost, number) in cost_map {
                if number.is_zero() {
                    continue;
                }
                if let Some(unit_cost) = cost {
                    let cost_currency = unit_cost.amount.currency.as_str();
                    *account_holdings.entry(cost_currency).or_default() +=
                        unit_cost.amount.number * number;
                    currencies.insert(cost_currency);
                } else {
                    *account_holdings.entry(currency.as_str()).or_default() += number;
                    currencies.insert(currency.as_str());
                }
            }
        }
        let mut leaf_node = &mut root_node;
        for key in account.split(':') {
            leaf_node = leaf_node.nodes.entry(key).or_default();
            for (currency, number) in account_holdings.iter() {
                *leaf_node.numbers.entry(currency).or_default() += number;
            }
        }
    }
    (root_node, currencies)
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct TrieTable<S> {
    pub rows: Vec<TrieTableRow<S>>,
    pub currencies: Vec<S>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrieTableRow<S> {
    pub level: usize,
    pub name: S,
    pub numbers: Vec<String>,
}

fn build_trie_table_helper<'s, 'r: 's>(
    root: &'r str,
    level: usize,
    node: &TrieNode<&'s str>,
    currencies: &Vec<&'s str>,
    rows: &mut Vec<TrieTableRow<&'s str>>,
) {
    let numbers = currencies
        .iter()
        .map(|c| {
            let number = node.numbers.get(*c).copied().unwrap_or_default();
            if number.is_zero() {
                String::new()
            } else {
                format!("{:.2}", number)
            }
        })
        .collect();
    let row = TrieTableRow {
        level,
        name: root,
        numbers,
    };
    rows.push(row);
    let mut sorted_kv: Vec<_> = node.nodes.iter().collect();
    sorted_kv.sort_by_key(|kv| kv.0);
    for (account, sub_trie) in sorted_kv {
        build_trie_table_helper(account, level + 1, sub_trie, currencies, rows);
    }
}

pub fn build_trie_table<'s, 'r: 's>(
    ledger: &'s Ledger,
    root_account: &'r str,
    options: TrieOptions,
) -> Option<TrieTable<&'s str>> {
    let (trie, currencies) = build_trie(ledger, root_account, options);
    if let Some(node) = trie.nodes.get(root_account) {
        let mut currencies: Vec<_> = currencies.into_iter().collect();
        currencies.sort();
        let mut rows = Vec::new();
        build_trie_table_helper(root_account, 0, node, &currencies, &mut rows);
        Some(TrieTable { rows, currencies })
    } else {
        None
    }
}

pub fn balance_sheet_to_list(sheet: &BalanceSheet) -> HashMap<String, Vec<Position>> {
    let mut result = HashMap::new();
    for (account, account_map) in sheet {
        let list = result.entry(account.to_string()).or_insert(Vec::new());
        for (currency, currency_map) in account_map {
            for (cost, number) in currency_map {
                list.push(Position {
                    number: *number,
                    currency: currency.clone(),
                    cost: cost.clone(),
                })
            }
        }
    }
    result
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct JournalItem<C: Hash + Eq, T> {
    pub txn: T,
    pub balance: HashMap<C, Decimal>,
    pub changes: HashMap<C, Decimal>,
}
