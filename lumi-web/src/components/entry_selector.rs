use lumi_server_defs::{FilterOptions, DEFAULT_ENTRIES_PER_PAGE};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, Clone, Debug, PartialEq, Eq)]
pub struct Props {
    #[prop_or(DEFAULT_ENTRIES_PER_PAGE)]
    pub entries: usize,
}

struct State {
    show_menu: bool,
}

pub struct EntrySelector {
    state: State,
    props: Props,
    link: ComponentLink<Self>,
}

pub enum Msg {
    ShowMenu,
}

impl Component for EntrySelector {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            state: State { show_menu: false },
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props == props {
            false
        } else {
            self.props = props;
            true
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ShowMenu => {
                self.state.show_menu = !self.state.show_menu;
            }
        }
        true
    }

    fn view(&self) -> Html {
        let show_menu = self.link.callback(|_| Msg::ShowMenu);
        let route_service = RouteService::<()>::new();
        let query = route_service.get_query();
        let mut chars = query.chars();
        chars.next();
        let query = chars.as_str();
        let current_option: FilterOptions = serde_urlencoded::from_str(&query).unwrap_or_default();
        let current_path = route_service.get_path();

        let menu_items: Vec<_> = [20, 50, 100]
            .iter()
            .map(|n| {
                let n = *n;
                let mut new_option = current_option.clone();
                if n == DEFAULT_ENTRIES_PER_PAGE {
                    new_option.entries = None;
                } else {
                    new_option.entries = Some(n);
                }
                let query = serde_urlencoded::to_string(&new_option).unwrap();
                let dest = if query.len() > 0 {
                    format!("{}?{}", &current_path, query)
                } else {
                    format!("{}", &current_path)
                };
                type Anchor = RouterAnchor<String>;
                let item_class = if new_option.entries == current_option.entries {
                    "entry-number button selected"
                } else {
                    "entry-number button"
                };
                html! {
                    <Anchor route={dest} classes={item_class}>{n}</Anchor>
                }
            })
            .collect();
        let menu_class = if self.state.show_menu {
            "entry-menu"
        } else {
            "entry-menu hide"
        };
        let menu_button = if self.state.show_menu {
            html! {
                <span onclick={show_menu} class="button selected">{self.props.entries}{" rows"}<div class="arrow-up"></div></span>
            }
        } else {
            html! {
                <span onclick={show_menu} class="button">{self.props.entries}{" rows"}<div class="arrow-down"></div></span>
            }
        };
        html! {
            <div class="select-entries">
                {menu_button}
                <div class={menu_class}>
                    {menu_items}
                </div>
            </div>
        }
    }
}
