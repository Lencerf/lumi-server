use std::collections::HashMap;
use yew_router::components::RouterAnchor;

use crate::api;
use crate::components::{EntrySelector, TxnCell};
use anyhow::Error;
use lumi_server_defs::{FilterOptions, DEFAULT_ENTRIES_PER_PAGE};
use rust_decimal::{prelude::Zero, Decimal};
use yew::format::Json;
use yew::{prelude::*, services::fetch::FetchTask};

#[derive(Properties, Clone, PartialEq, Eq)]
pub struct Props {
    pub account: String,
    pub options: String,
}

type Journal = api::Journal;

struct State {
    journal: Option<(Journal, usize)>,
    get_journal_error: Option<Error>,
    get_journal_loaded: bool,
    options: FilterOptions,
    expand_postings: bool,
}
pub struct JournalTable {
    props: Props,
    state: State,
    link: ComponentLink<Self>,
    task: Option<FetchTask>,
}

pub enum Msg {
    GetJournal,
    GetJournalError(Error),
    GetJournalSuccess(Journal, usize),
    ExpandPostings,
}

fn change_to_str(changes: &HashMap<String, Decimal>) -> String {
    let descriptions: Vec<String> = changes
        .iter()
        .filter(|(_, n)| !n.is_zero())
        .map(|(c, n)| format!("{} {}", n, c))
        .collect();
    descriptions.join("\n")
}

impl JournalTable {
    fn option_to_dest(&self, options: &FilterOptions) -> String {
        let mut dest = if self.props.account.len() > 0 {
            format!("/account/{}", &self.props.account)
        } else {
            format!("/journal")
        };
        let query = serde_urlencoded::to_string(options).unwrap();
        if query.len() > 0 {
            dest.push('?');
            dest.push_str(&query);
        }
        dest
    }
}

impl Component for JournalTable {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Msg::GetJournal);
        let options = serde_urlencoded::from_str(&props.options).unwrap_or_default();
        Self {
            props,
            state: State {
                journal: None,
                get_journal_error: None,
                get_journal_loaded: false,
                options,
                expand_postings: false,
            },
            link,
            task: None,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if props != self.props {
            self.state.get_journal_loaded = false;
            self.state.options = serde_urlencoded::from_str(&props.options).unwrap_or_default();
            self.props = props;
            self.link.send_message(Msg::GetJournal);
            true
        } else {
            false
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::GetJournalError(error) => {
                self.state.get_journal_loaded = true;
                self.state.get_journal_error = Some(error);
            }
            Msg::GetJournalSuccess(journal, total) => {
                self.state.get_journal_loaded = true;
                self.state.journal = Some((journal, total));
            }
            Msg::GetJournal => {
                let handler =
                    self.link
                        .callback(|response: api::FetchResponse<(Journal, usize)>| {
                            let (_, Json(data)) = response.into_parts();
                            match data {
                                Ok((journal, total)) => Msg::GetJournalSuccess(journal, total),
                                Err(error) => Msg::GetJournalError(error),
                            }
                        });
                self.task = Some(api::get_account_journal(
                    &self.props.account,
                    &self.state.options,
                    handler,
                ));
            }
            Msg::ExpandPostings => {
                self.state.expand_postings = !self.state.expand_postings;
            }
        }
        true
    }

    fn view(&self) -> Html {
        if !self.state.get_journal_loaded {
            return html! {<p>{"loading"}</p>};
        }
        if let Some((ref journal, total)) = self.state.journal {
            let mut rows = vec![];
            if self.props.account.len() > 0 {
                for (index, item) in journal.iter().enumerate() {
                    let change_str = change_to_str(&item.changes);
                    let balance_str = change_to_str(&item.balance);
                    rows.push(html!{
                        <TxnCell txn=item.txn.clone() change_balance=(change_str, balance_str) index=index show_postings={self.state.expand_postings} />
                    });
                }
            } else {
                for (index, item) in journal.iter().enumerate() {
                    rows.push(html!{
                        <TxnCell txn=item.txn.clone() index=index show_postings={self.state.expand_postings}/>
                    });
                }
            }
            type Anchor = RouterAnchor<String>;
            let mut options_change_order = self.state.options.clone();
            let order_indicator = if options_change_order.old_first == Some(true) {
                options_change_order.old_first = None;
                html! {
                    <Anchor route={self.option_to_dest(&options_change_order)}><div class="arrow-up"></div></Anchor>
                }
            } else {
                options_change_order.old_first = Some(true);
                html! {
                    <Anchor route={self.option_to_dest(&options_change_order)}><div class="arrow-down"></div></Anchor>
                }
            };
            let head = if self.props.account.len() > 0 {
                html! {
                    <tr class="head">
                        <th class="left date">{"Date"}{order_indicator}</th>
                        <th class="center flag">{"Flag"}</th>
                        <th class="left">{"Description"}</th>
                        <th class="right amount">{"Position"}</th>
                        <th class="right cost">{"Cost"}</th>
                        <th class="right amount">{"Price"}</th>
                        <th class="right amount">{"Change"}</th>
                        <th class="right amount">{"Balance"}</th>
                    </tr>
                }
            } else {
                html! {
                    <tr class="head">
                        <th class="left date">{"Date"}{order_indicator}</th>
                        <th class="center flag">{"Flag"}</th>
                        <th class="left">{"Description"}</th>
                        <th class="right amount">{"Position"}</th>
                        <th class="right cost">{"Cost"}</th>
                        <th class="right amount">{"Price"}</th>
                    </tr>
                }
            };
            let table = html! {
                <div class="card">
                    <table class="txn">
                        {head}
                        {rows}
                    </table>
                </div>
            };

            let entries = self
                .state
                .options
                .entries
                .unwrap_or(DEFAULT_ENTRIES_PER_PAGE);
            let current_page = self.state.options.page.unwrap_or(1);
            let total_pages = (total + entries - 1) / entries;
            let mut link_pages = vec![];
            if current_page > 0 && current_page <= total_pages {
                if current_page > 4 {
                    link_pages.extend(&[1, 0, current_page - 1]);
                } else {
                    for p in 1..current_page {
                        link_pages.push(p);
                    }
                }
                link_pages.push(current_page);
                if current_page + 3 < total_pages {
                    link_pages.extend(&[current_page + 1, 0, total_pages]);
                } else {
                    for p in current_page + 1..=total_pages {
                        link_pages.push(p);
                    }
                }
            } else {
                if total_pages > 4 {
                    link_pages.extend(&[1, 2, 3, 0, total_pages]);
                } else {
                    link_pages = (1..=total_pages).into_iter().collect();
                }
            }
            let mut page_buttons = vec![];
            if current_page > 1 {
                let mut options_prev = self.state.options.clone();
                match options_prev.page.as_mut() {
                    Some(n) if *n > 2 => *n -= 1,
                    _ => options_prev.page = None,
                };
                page_buttons.push(html!{
                    <Anchor route={self.option_to_dest(&options_prev)} classes="button">{"<"}</Anchor>
                })
            }
            for p in link_pages {
                let button = if p == 0 {
                    html! {
                        <a class="button">{"..."}</a>
                    }
                } else if p == current_page {
                    html! {
                        <a class="button selected">{current_page}</a>
                    }
                } else {
                    let mut option = self.state.options.clone();
                    let button_class: &str;
                    match p {
                        1 => {
                            option.page = None;
                            button_class = "button";
                        }
                        q if q == total_pages => {
                            option.page = Some(p);
                            button_class = "button";
                        }
                        _ => {
                            option.page = Some(p);
                            button_class = "button extra-page";
                        }
                    };
                    html! {
                        <Anchor route={self.option_to_dest(&option)} classes={button_class}>{p}</Anchor>
                    }
                };
                page_buttons.push(button);
            }

            if current_page < total_pages {
                let mut options_next = self.state.options.clone();
                match options_next.page.as_mut() {
                    Some(n) => *n += 1,
                    _ => options_next.page = Some(2),
                };
                page_buttons.push(html!{
                    <Anchor route={self.option_to_dest(&options_next)} classes="button">{">"}</Anchor>
                });
            }
            let current_entries = self
                .state
                .options
                .entries
                .unwrap_or(DEFAULT_ENTRIES_PER_PAGE);
            let row_selector = html! {
                <div class="row-selector">
                    <EntrySelector entries={current_entries}/>
                    <div class="buttons">
                        {page_buttons}
                    </div>
                </div>
            };
            let onclick_expand = self.link.callback(|_| Msg::ExpandPostings);
            let class_expand = if self.state.expand_postings {
                "button selected"
            } else {
                "button"
            };
            html! {
                <>
                    <div class="txn-table-head">
                        <span onclick={onclick_expand} class={class_expand}>{"Expand Positions"}</span>
                        {row_selector}
                    </div>
                    {table}
                </>
            }
        } else {
            html! {
                <p>{"error"}</p>
            }
        }
    }
}
