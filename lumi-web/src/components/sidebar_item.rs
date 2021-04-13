use crate::route::Route;
use yew::prelude::*;
use yew_router::components::RouterAnchor;
#[derive(Properties, Clone)]
pub struct Props {
    pub dest: Route,
    pub current: Route,
    pub title: &'static str,
}

pub struct SidebarItem {
    props: Props,
}

impl Component for SidebarItem {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        self.props = _props;
        true
    }

    fn view(&self) -> Html {
        type Anchor = RouterAnchor<Route>;
        if self.props.dest == self.props.current
            || (self.props.current == Route::Index && self.props.dest == Route::Balance)
        {
            html! {
                <li class="active">
                    <Anchor route=self.props.dest.clone()>
                        <span>{&self.props.title}</span>
                    </Anchor>
                </li>
            }
        } else {
            html! {
                <li>
                    <Anchor route=self.props.dest.clone()>
                        <span>{&self.props.title}</span>
                    </Anchor>
                </li>
            }
        }
    }
}
