use crate::api::sign_in_api;
use crate::components::nav::NavComponent;
use crate::router::Route;
use crate::store::{AuthState, UserStore};
use log::info;
use reqwasm::Error;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, FocusEvent, HtmlInputElement};
use yew::{events::Event, html, Callback, Component, Context, Html};
use yew_router::prelude::*;
use yewdux::dispatch::{Dispatch, Dispatcher};
use yewdux::prelude::BasicStore;

pub enum SignInMessage {
    SignIn,
    Success(String),
    InputUsername(String),
    InputPassword(String),
    UserState(Rc<UserStore>),
    Error(Error),
}

pub enum Stages {
    SignUp,
    Success,
    Error(String),
}

pub struct SignInPage {
    username: String,
    password: String,
    dispatch: Dispatch<BasicStore<UserStore>>,
    state: Rc<UserStore>,
    stage: Stages,
}

impl Component for SignInPage {
    type Message = SignInMessage;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let dispatch = Dispatch::bridge_state(ctx.link().callback(SignInMessage::UserState));
        Self {
            username: "".to_string(),
            password: "".to_string(),
            dispatch,
            state: Default::default(),
            stage: Stages::SignUp,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            SignInMessage::InputUsername(username) => {
                self.username = username;
                true
            }
            SignInMessage::InputPassword(password) => {
                self.password = password;
                true
            }
            SignInMessage::SignIn => {
                let username = self.username.clone();
                let password = self.password.clone();
                info!("username: {}; password: {}", username, password);
                ctx.link().send_future(async {
                    match sign_in_api(username, password).await {
                        Ok(data) => SignInMessage::Success(data.access_token),
                        Err(error) => SignInMessage::Error(error),
                    }
                });
                false
            }
            SignInMessage::Success(token) => {
                self.dispatch.reduce(|s| s.token = token);
                self.dispatch.reduce(|s| s.auth_state = AuthState::Auth);
                self.stage = Stages::Success;
                true
            }
            SignInMessage::UserState(state) => {
                self.state = state;
                true
            }
            SignInMessage::Error(error) => {
                info!(
                    "name {} pwd {}",
                    self.username.clone(),
                    self.password.clone()
                );
                match error {
                    Error::JsError(error) => self.stage = Stages::Error(error.name),
                    Error::SerdeError(error) => self.stage = Stages::Error(error.to_string()),
                }
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let change = |e: FocusEvent| e.prevent_default();
        match &self.stage {
            Stages::SignUp => {
                html!(
                    <main>
                        <NavComponent/>
                        <div class="center form-container">
                            <form class="form form-auth" onsubmit={change}>
                                {self.get_header()}
                                {self.html_input_username(ctx)}
                                {self.html_input_password(ctx)}
                                {self.html_button_login(ctx)}
                            </form>
                        </div>
                    </main>
                )
            }
            Stages::Error(_error) => {
                html!(
                    <main>
                        <NavComponent/>
                        <div class="center form-container">
                            <form class="form form-auth" onsubmit={change}>
                                {self.get_header()}
                                {self.get_error_message()}
                                {self.html_input_username(ctx)}
                                {self.html_input_password(ctx)}
                                {self.html_button_login(ctx)}
                            </form>
                        </div>
                    </main>
                )
            }
            Stages::Success => {
                html!(
                    <Redirect<Route> to={Route::Home}/>
                )
            }
        }
    }
}

impl SignInPage {
    fn html_button_login(&self, ctx: &Context<Self>) -> Html {
        html!(
            <div class="form-element column-direction center">
                <button cursor="pointer" class="primary-button" onclick={ctx.link().callback(|_| SignInMessage::SignIn)}>
                    { "Login" }
                </button>
            </div>
        )
    }

    fn html_input_username(&self, ctx: &Context<Self>) -> Html {
        let change: Callback<Event> = ctx.link().batch_callback(|e: Event| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            input.map(|input| SignInMessage::InputUsername(input.value()))
        });
        html! {
            <div class="form-element column-direction center">
                <label class="primary-input-label" for="username-input">
                    { "Username" }
                </label>
                <input class="primary-input" onchange={change}
                        id="username-input"
                        value={self.username.clone()}
                        type="text"
                />
            </div>
        }
    }

    fn html_input_password(&self, ctx: &Context<Self>) -> Html {
        let change = ctx.link().batch_callback(|e: Event| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            input.map(|input| SignInMessage::InputPassword(input.value()))
        });
        html! {
            <div class="form-element column-direction center">
                <label class="primary-input-label" for="password-input">
                    { "Password" }
                </label>
                <input class="primary-input" onchange={change}
                    id="password-input"
                    type="password"
                    value={self.password.clone()}
                />
            </div>
        }
    }

    fn get_header(&self) -> Html {
        html!(
            <h3 class="form-element column-direction center form-header">{"Please, sign in"}</h3>
        )
    }

    fn get_error_message(&self) -> Html {
        html!(
            <h3 class="form-element column-direction center form-header error-message">{"Wrong credentials"}</h3>
        )
    }
}
