use super::sidebar_item::SidebarItem;
use crate::route::Route;
use yew::prelude::*;

#[derive(Properties, Clone, Debug)]
pub struct Props {
    #[prop_or(false)]
    pub always_show: bool,
    pub current: Route,
    pub on_hide: Callback<bool>,
}
pub struct Sidebar {
    props: Props,
    link: ComponentLink<Self>,
}

pub enum Msg {
    MayHideSelf,
}

impl Component for Sidebar {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::MayHideSelf => self.props.on_hide.emit(false),
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.always_show == props.always_show && self.props.current == props.current {
            self.props.on_hide = props.on_hide;
            false
        } else {
            self.props = props;
            true
        }
    }

    fn view(&self) -> Html {
        let items = vec![
            html! {<SidebarItem  dest=Route::Balance current={self.props.current.clone()} title={"Balance Sheet"}/>},
            html! {<SidebarItem  dest=Route::Income current={self.props.current.clone()} title={"Income Statement"}/>},
            html! {<SidebarItem  dest=Route::Journal current={self.props.current.clone()} title={"Journal"}/>},
            html! {<SidebarItem  dest=Route::Holdings current={self.props.current.clone()} title={"Holdings"}/>},
            html! {<SidebarItem  dest=Route::Errors current={self.props.current.clone()} title={"Errors"}/>},
        ];
        let ul = html! {
            <ul>
                {items}
            </ul>
        };
        let class_name = if self.props.always_show {
            "sidebar show"
        } else {
            "sidebar"
        };
        let onclick = self.link.callback(|_| Msg::MayHideSelf);
        html! {
            <div class={class_name}>
                <div class="title">
                    <h1>{"Lumi"}</h1>
                    <span onclick={&onclick} id="hide_sidebar">{"←"}</span>
                </div>
                <nav onclick={&onclick}>{ul}
                </nav>
            </div>
        }
    }
}
