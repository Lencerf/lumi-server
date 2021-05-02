use anyhow::Error;
use yew::format::Json;
use yew::{prelude::*, services::fetch::FetchTask};

use crate::api;
use crate::route::Route;
use yew_router::components::RouterAnchor;
use lumi_server_defs::TrieOptions;

type Trie = api::Trie;

#[derive(Properties, Clone, PartialEq, Eq)]
pub struct Props {
    pub root: &'static str,
    pub options: String,
}

struct State {
    trie: Option<Trie>,
    options: TrieOptions,
    get_trie_error: Option<Error>,
    get_trie_loaded: bool,
}

pub enum Msg {
    GetTrie,
    GetTrieSuccess(Trie),
    GetTrieError(Error),
}
pub struct TrieTable {
    props: Props,
    link: ComponentLink<Self>,
    task: Option<FetchTask>,
    state: State,
}

impl Component for TrieTable {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Msg::GetTrie);
        let options = serde_urlencoded::from_str(&props.options).unwrap_or_default();
        Self {
            props,
            link,
            task: None,
            state: State {
                trie: None,
                options,
                get_trie_error: None,
                get_trie_loaded: false,
            },
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if props != self.props {
            self.state.get_trie_loaded = false;
            self.props = props;
            self.link.send_message(Msg::GetTrie);
            true
        } else {
            false
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::GetTrieError(error) => {
                self.state.get_trie_loaded = true;
                self.state.get_trie_error = Some(error);
            }
            Msg::GetTrieSuccess(trie) => {
                self.state.get_trie_loaded = true;
                self.state.trie = Some(trie);
            }
            Msg::GetTrie => {
                let handler = self.link.callback(|response: api::FetchResponse<Trie>| {
                    let (_, Json(data)) = response.into_parts();
                    match data {
                        Ok(trie) => Msg::GetTrieSuccess(trie),
                        Err(err) => Msg::GetTrieError(err),
                    }
                });
                self.task = Some(api::get_trie(&self.props.root, &self.state.options, handler));
            }
        }
        true
    }

    fn view(&self) -> Html {
        if !self.state.get_trie_loaded {
            return html! {<p>{"loading"}</p>};
        }
        if let Some(trie) = &self.state.trie {
            let mut heads = vec![html! {<th/>}];
            for currency in trie.currencies.iter() {
                heads.push(html! {<th class="mono right">{currency}</th>})
            }
            let mut stack: Vec<(&String, usize)> = Vec::new();
            let rows: Vec<_> = trie
                .rows
                .iter()
                .map(|row| {
                    let td_class = format!("l{}", row.level);
                    while let Some((_, last_level)) = stack.last() {
                        if *last_level < row.level {
                            break;
                        } else {
                            stack.pop();
                        }
                    }
                    stack.push((&row.name, row.level));
                    let full_account = stack
                        .iter()
                        .map(|(seg, _)| seg.as_str())
                        .collect::<Vec<_>>()
                        .join(":");
                    type Anchor = RouterAnchor<Route>;
                    let dest = Route::Account(full_account);
                    let mut cols = vec![html! {
                        <td class={td_class}>
                            <Anchor route=dest classes="account">
                                {&row.name}
                            </Anchor>
                        </td>
                    }];
                    for number in &row.numbers {
                        cols.push(html! {<td class="mono right">{number}</td>});
                    }
                    html! {<tr>{cols}</tr>}
                })
                .collect();

            html! {
                <div class="card inline-block">
                    <table class="trie">
                        <tr>
                            {heads}
                        </tr>
                        {rows}
                    </table>
                </div>
            }
        } else {
            html! {<p>{"error"}</p>}
        }
    }
}
