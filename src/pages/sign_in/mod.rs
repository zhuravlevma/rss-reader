use crate::api::sign_in_api;
use crate::components::nav::NavComponent;
use crate::router::Route;
use crate::store::{AuthState, UserStore};
use log::info;
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
}

pub enum Stages {
    SignUp,
    Success,
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
                        Err(_) => SignInMessage::Success("error".to_string()),
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
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let change = |e: FocusEvent| e.prevent_default();
        match self.stage {
            Stages::SignUp => {
                html!(
                    <main class="main-page">
                        <NavComponent/>
                        <div class="form-container">
                            <form class="form" onsubmit={change}>
                                {self.get_header()}
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
            <div class="form-element">
                <button cursor="pointer" class="form-element-button" onclick={ctx.link().callback(|_| SignInMessage::SignIn)}>
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
            <div class="form-element">
                <label class="form-element-label" for="username-input">
                    { "Username" }
                </label>
                <input class="form-element-input" onchange={change}
                        id="username-input"
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
            <div class="form-element">
                <label class="form-element-label" for="password-input">
                    { "Password" }
                </label>
                <input class="form-element-input" onchange={change}
                    id="password-input"
                    type="password"
                />
            </div>
        }
    }

    fn get_header(&self) -> Html {
        html!(
            <h3 class="form-element form-header">{"Please, sign in"}</h3>
        )
    }
}
