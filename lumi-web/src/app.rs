use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::{ErrorList, HoldingTable, JournalTable, Sidebar, TrieTable};
use crate::route::Route;

struct State {
    show_sidebar: bool,
}

pub struct App {
    state: State,
    link: ComponentLink<Self>,
}

pub enum Msg {
    ShowSidebar(bool),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            state: State {
                show_sidebar: false,
            },
            link,
        }
    }

    fn update(&mut self, message: Self::Message) -> ShouldRender {
        match message {
            Msg::ShowSidebar(v) => {
                self.state.show_sidebar = v;
            }
        }
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let show_sidebar = self.state.show_sidebar;
        let show_hide_handle = self.link.callback(|show: bool| Msg::ShowSidebar(show));
        let show_sidebar_click = self.link.callback(|_| Msg::ShowSidebar(true));

        let render = Router::render(move |switch: Route| {
            let qs = RouteService::<()>::new().get_query();
            let mut chars = qs.chars();
            chars.next();
            let qs = chars.as_str();
            let show_button = html! {
                <span onclick={show_sidebar_click.clone()} id="show_sidebar">{"☰"}</span>
            };

            let title = match switch {
                Route::Balance | Route::Index => "Balance Sheet",
                Route::Holdings => "Holdings",
                Route::Journal => "Journal",
                Route::Income => "Income",
                Route::Account(ref account) => account.as_str(),
                Route::Errors => "Errors",
            };
            let title_bar = html! {<header>{show_button.clone()}{title}</header>};
            let right_content = match switch {
                Route::Balance | Route::Index => {
                    html! {
                        <>
                            <div class="column">
                                <TrieTable root="Assets" options={qs}/>
                            </div>
                            <div class="column">
                                <TrieTable root="Liabilities" options={qs}/>
                                <TrieTable root="Equity" options={qs}/>
                            </div>
                        </>
                    }
                }
                Route::Income => {
                    html! {
                        <>
                            <div class="column">
                                <TrieTable root="Income" options={qs}/>
                            </div>
                            <div class="column">
                                <TrieTable root="Expenses" options={qs}/>
                            </div>
                        </>
                    }
                }
                Route::Journal => {
                    html! {
                        <>
                        <JournalTable account={""}, options={qs}/>
                        </>
                    }
                }
                Route::Holdings => {
                    html! {
                        <>
                        <HoldingTable />
                        </>
                    }
                }
                Route::Account(ref account) => {
                    html! {
                        <>
                        <JournalTable account={account}, options={qs}/>
                        </>
                    }
                }
                Route::Errors => {
                    html! {
                        <ErrorList/>
                    }
                }
            };
            html! {
                <>
                    <Sidebar current={switch.clone()} always_show={show_sidebar} on_hide={show_hide_handle.clone()} />
                    <div class="right-wrap">
                        {title_bar}
                        <main>
                            {right_content}
                        </main>
                    </div>

                </>
            }
        });

        html! {
            <>
                <Router<Route, ()> render=render/>
            </>
        }
    }
}
