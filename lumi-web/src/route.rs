use yew_router::prelude::*;

#[derive(Switch, Debug, Clone, PartialEq, Eq)]
pub enum Route {
    #[to = "/holdings"]
    Holdings,
    #[to = "/account/{name}"]
    Account(String),
    #[to = "/journal"]
    Journal,
    #[to = "/income"]
    Income,
    #[to = "/errors"]
    Errors,
    #[to = "/balance_sheet"]
    Balance,
    #[to = "/"]
    Index,
}
