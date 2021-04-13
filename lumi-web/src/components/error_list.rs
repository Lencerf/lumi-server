use crate::api;
use anyhow::Error;
use lumi::ErrorLevel;
use yew::format::Json;
use yew::{prelude::*, services::fetch::FetchTask};

type Errors = api::Errors;

pub enum Msg {
    GetErrors,
    GetErrorsSuccess(Errors),
    GEtErrorsFail(Error),
}

struct State {
    get_errors_loaded: bool,
    get_errors_error: Option<Error>,
    errors: Option<Errors>,
}

pub struct ErrorList {
    link: ComponentLink<Self>,
    task: Option<FetchTask>,
    state: State,
}

impl Component for ErrorList {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Msg::GetErrors);
        Self {
            link,
            task: None,
            state: State {
                get_errors_loaded: false,
                get_errors_error: None,
                errors: None,
            },
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::GetErrorsSuccess(errors) => {
                self.state.get_errors_loaded = true;
                self.state.errors = Some(errors);
            }
            Msg::GEtErrorsFail(error) => {
                self.state.get_errors_loaded = true;
                self.state.get_errors_error = Some(error);
            }
            Msg::GetErrors => {
                let handler = self.link.callback(|response: api::FetchResponse<Errors>| {
                    let Json(data) = response.into_body();
                    match data {
                        Ok(errors) => Msg::GetErrorsSuccess(errors),
                        Err(error) => Msg::GEtErrorsFail(error),
                    }
                });
                self.task = Some(api::get_errors(handler));
            }
        }
        true
    }

    fn view(&self) -> Html {
        if !self.state.get_errors_loaded {
            return html! {<p>{"loading"}</p>};
        }
        if let Some(errors) = &self.state.errors {
            let error_list: Vec<_> = errors.iter().map(|error| {
                let error_type = match error.level {
                    ErrorLevel::Error => html!{<span class="error">{"Error"}</span>},
                    ErrorLevel::Info => html!{<span class="info">{"Info"}</span>},
                    ErrorLevel::Warning => html!{<span class="warning">{"Warning"}</span>},
                };
                html!{
                    <>
                        <p class="desc">{error_type}{": "}{&error.msg}</p>
                        <p class="src">{&error.src.file}{":"}{error.src.start.line}{":"}{error.src.start.col}</p>
                    </>
                }
            }).collect();
            html! {<>{error_list}</>}
        } else {
            html! {<p>{"error"}</p>}
        }
    }
}
