use lumi::{Transaction, TxnFlag};

use crate::components::AccountRef;
use std::rc::Rc;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq, Eq)]
pub struct Props {
    pub txn: Rc<Transaction>,
    #[prop_or(false)]
    pub show_postings: bool,
    #[prop_or_default]
    pub change_balance: Option<(String, String)>,
    pub index: usize,
}

struct State {
    show_postings: bool,
}

pub enum Msg {
    ShowHidePostings,
}

pub struct TxnCell {
    props: Props,
    state: State,
    link: ComponentLink<Self>,
}

impl TxnCell {
    fn flag_str(&self) -> &'static str {
        match self.props.txn.flag() {
            TxnFlag::Posted => "*",
            TxnFlag::Balance => "bal",
            TxnFlag::Pad => "pad",
            TxnFlag::Pending => "!",
        }
    }

    fn even_odd(&self) -> &'static str {
        if self.props.index % 2 == 0 {
            "even"
        } else {
            "odd"
        }
    }

    fn desc(&self) -> Html {
        let txn = &self.props.txn;
        if txn.payee().len() > 0 {
            if txn.narration().len() > 0 {
                html! {
                    <>
                        <strong>{txn.payee()}</strong>
                        {" "}
                        {txn.narration()}
                    </>
                }
            } else {
                html! {
                    <strong>{txn.payee()}</strong>
                }
            }
        } else {
            html! {
                {txn.narration()}
            }
        }
    }

    fn balance_view(&self) -> Vec<Html> {
        let mut result = Vec::new();
        for posting in self.props.txn.postings() {
            let desc_span = if self.props.change_balance.is_some() {
                "5"
            } else {
                "1"
            };
            let tr_class = format!("balance {}", self.even_odd());
            let extra_td = if self.props.change_balance.is_some() {
                html! {}
            } else {
                html! {<td colspan="2"></td>}
            };
            result.push(html! {
                <tr class={tr_class}>
                    <td class="left mono date">{self.props.txn.date()}</td>
                    <td class="center mono flag">{"bal"}</td>
                    <td class="left" colspan={desc_span}><AccountRef account=&*posting.account /></td>
                    <td class="right amount mono">{&posting.amount}</td>
                    {extra_td}
                </tr>
            });
        }
        return result;
    }

    fn posting_view(&self) -> Vec<Html> {
        let mut result = Vec::new();
        let onclick = self.link.callback(|_| Msg::ShowHidePostings);

        let indicators = "•".repeat(self.props.txn.postings().len());
        let desc = html! {
            <>
                <td class="left">
                    {self.desc()}
                </td>
                <td class="expand mono right">
                    <span onclick=onclick>{indicators}</span>
                </td>
            </>
        };

        let tr_class = format!("txn {}", self.even_odd());
        if let Some((change, balance)) = &self.props.change_balance {
            result.push(html! {
                <tr class={tr_class}>
                    <td class="left mono date">{self.props.txn.date()}</td>
                    <td class="center mono flag">{self.flag_str()}</td>
                    {desc}
                    <td colspan="2"></td>
                    <td class="right amount mono">{change}</td>
                    <td class="right amount mono">{balance}</td>
                </tr>
            })
        } else {
            result.push(html! {
                <tr class={tr_class}>
                    <td class="left mono date">{self.props.txn.date()}</td>
                    <td class="center mono flag">{self.flag_str()}</td>
                    {desc}
                    <td colspan="2"></td>
                </tr>
            })
        }
        if self.state.show_postings {
            let posting_class = format!("posting {}", self.even_odd());
            for posting in self.props.txn.postings() {
                let price = posting
                    .price
                    .as_ref()
                    .map(|p| p.to_string())
                    .unwrap_or_default();
                let cost = posting
                    .cost
                    .as_ref()
                    .map(|c| html! {<>{&c.amount}<br/>{c.date}</>})
                    .unwrap_or_default();
                let extra_td = if self.props.change_balance.is_some() {
                    html! {<td colspan="2"></td>}
                } else {
                    html! {}
                };
                result.push(html! {
                    <tr class={&posting_class}>
                        <td></td>
                        <td></td>
                        <td class="left"><AccountRef account=&*posting.account /></td>
                        <td class="right mono amount">{&posting.amount}</td>
                        <td class="right mono cost">{cost}</td>
                        <td class="right mono amount">{price}</td>
                        {extra_td}
                    </tr>
                })
            }
        }
        result
    }
}

impl Component for TxnCell {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            state: State {
                show_postings: props.show_postings,
            },
            props,
            link,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if props != self.props {
            self.state.show_postings = props.show_postings;
            self.props = props;
            true
        } else {
            false
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ShowHidePostings => {
                let new_state = self.props.show_postings || !self.state.show_postings;
                if new_state == self.state.show_postings {
                    false
                } else {
                    self.state.show_postings = new_state;
                    true
                }
            }
        }
    }

    fn view(&self) -> Html {
        if self.props.txn.flag() == TxnFlag::Balance {
            html! { <>{self.balance_view()}</>}
        } else {
            html! { <>{self.posting_view()}</>}
        }
    }
}
