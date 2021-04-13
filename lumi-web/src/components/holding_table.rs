use crate::api;
use crate::components::AccountRef;
use anyhow::Error;
use lumi_server_defs::Position;
use rust_decimal::prelude::Zero;
use std::collections::HashMap;
use yew::format::Json;
use yew::{prelude::*, services::fetch::FetchTask};
#[derive(Properties, Clone)]
pub struct Props {}

type HoldingMap = HashMap<String, Vec<Position>>;

struct State {
    holdings: Option<HoldingMap>,
    get_holdings_error: Option<Error>,
    get_holdings_loaded: bool,
}

pub struct HoldingTable {
    _props: Props,
    state: State,
    link: ComponentLink<Self>,
    task: Option<FetchTask>,
}

pub enum Msg {
    GetHoldings,
    GetHoldingsSuccess(HoldingMap),
    GetHoldingsError(Error),
}

impl Component for HoldingTable {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Msg::GetHoldings);
        Self {
            _props: props,
            state: State {
                holdings: None,
                get_holdings_error: None,
                get_holdings_loaded: false,
            },
            link,
            task: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::GetHoldings => {
                let handler =
                    self.link
                        .callback(move |response: api::FetchResponse<HoldingMap>| {
                            let (_, Json(data)) = response.into_parts();
                            match data {
                                Ok(holdings) => Msg::GetHoldingsSuccess(holdings),
                                Err(err) => Msg::GetHoldingsError(err),
                            }
                        });
                self.task = Some(api::get_balances(handler));
            }
            Msg::GetHoldingsSuccess(holdings) => {
                self.state.holdings = Some(holdings);
                self.state.get_holdings_loaded = true;
            }
            Msg::GetHoldingsError(err) => {
                self.state.get_holdings_error = Some(err);
                self.state.get_holdings_loaded = true;
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        if let Some(ref holdings) = self.state.holdings {
            let mut rows: Vec<Html> = vec![html! {
                <tr>
                    <th class="left">{"Account"}</th>
                    <th class="right">{"Amount"}</th>
                    <th class="right">{"Cost"}</th>
                    <th class="right">{"Acquisition Date"}</th>
                    <th class="right">{"Book Value"}</th>
                </tr>
            }];
            let mut entries = holdings.iter().collect::<Vec<_>>();
            entries.sort_by_key(|t| t.0);
            for (account, account_map) in entries {
                if !account.starts_with("Assets") && !account.starts_with("Lia") {
                    continue;
                }
                let mut account_entries = account_map.iter().collect::<Vec<_>>();
                account_entries.sort_by_key(|p| (&p.currency));
                for position in account_entries {
                    if position.number.is_zero() {
                        continue;
                    }
                    if let Some(cost) = &position.cost {
                        rows.push(html!{
                            <tr>
                                <td class="left"><AccountRef account=account/></td>
                                <td class="mono right">{position.number}{" "}{&position.currency}</td>
                                <td class="mono right">{&cost.amount}</td>
                                <td class="mono right">{&cost.date}</td>
                                <td class="mono right">{position.number*cost.amount.number}{" "}{&cost.amount.currency}</td>
                            </tr>
                        })
                    } else {
                        rows.push(html!{
                            <tr>
                                <td class="left"><AccountRef account=account/></td>
                                <td class="mono right">{position.number}{" "}{&position.currency}</td>
                                <td class="mono right"></td>
                                <td class="mono right"></td>
                                <td class="mono right">{position.number}{" "}{&position.currency}</td>
                            </tr>
                        })
                    }
                }
            }
            html! {
                <div class="card">
                    <table class="holdings">{rows}</table>
                </div>
            }
        } else if let Some(_) = self.state.get_holdings_error {
            html! {
                <p>{"error"}</p>
            }
        } else {
            html! {<p>{"loading"}</p>}
        }
    }
}
