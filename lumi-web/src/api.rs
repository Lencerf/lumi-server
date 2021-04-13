use anyhow::Error;
use lumi::Transaction;
use lumi_server_defs::{FilterOptions, JournalItem, Position, TrieTable};
use std::collections::HashMap;
use std::rc::Rc;
use yew::callback::Callback;
use yew::format::{Json, Nothing};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

pub type FetchResponse<T> = Response<Json<Result<T, Error>>>;
type FetchCallback<T> = Callback<FetchResponse<T>>;

pub type Errors = Vec<lumi::Error>;
pub fn get_errors(callback: FetchCallback<Errors>) -> FetchTask {
    let req = Request::get("/api/errors").body(Nothing).unwrap();
    FetchService::fetch(req, callback).unwrap()
}

pub type Trie = TrieTable<String>;
pub fn get_trie(root: &str, callback: FetchCallback<Trie>) -> FetchTask {
    let req = Request::get(format!("/api/trie/{}", root))
        .body(Nothing)
        .unwrap();
    FetchService::fetch(req, callback).unwrap()
}

pub fn get_balances(callback: FetchCallback<HashMap<String, Vec<Position>>>) -> FetchTask {
    let req = Request::get("/api/balances").body(Nothing).unwrap();

    FetchService::fetch(req, callback).unwrap()
}

pub type Journal = Vec<JournalItem<String, Rc<Transaction>>>;
pub fn get_account_journal(
    account: &str,
    options: &FilterOptions,
    callback: FetchCallback<(Journal, usize)>,
) -> FetchTask {
    let query = serde_urlencoded::to_string(&options).unwrap();
    let req = if account.len() > 0 {
        Request::get(format!("/api/account/{}?{}", account, query))
            .body(Nothing)
            .unwrap()
    } else {
        Request::get(format!("/api/journal/?{}", query))
            .body(Nothing)
            .unwrap()
    };
    FetchService::fetch(req, callback).unwrap()
}
