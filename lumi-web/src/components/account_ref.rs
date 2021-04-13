use crate::route::Route;
use yew::prelude::*;
use yew_router::components::RouterAnchor;

#[derive(Properties, Clone, PartialEq, Eq)]
pub struct Props {
    pub account: String,
}
pub struct AccountRef {
    props: Props,
}

impl Component for AccountRef {
    type Message = ();
    type Properties = Props;

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if props == self.props {
            false
        } else {
            self.props = props;
            true
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn view(&self) -> Html {
        type Anchor = RouterAnchor<Route>;
        let dest = Route::Account(self.props.account.clone());
        html! {
            <Anchor route=dest classes="account">{&self.props.account}
            </Anchor>
        }
    }
}
