use std::{collections::HashMap, fmt::Debug, hash::Hash};

use lumi::{Currency, UnitCost};
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
    pub time: Option<String>,
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct JournalItem<C: Hash + Eq, T> {
    pub txn: T,
    pub balance: HashMap<C, Decimal>,
    pub changes: HashMap<C, Decimal>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RefreshTime {
    pub timestamp: i64,
}
