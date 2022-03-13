use crate::api::sign_in_api;
use crate::components::nav::NavComponent;
use crate::{routing, UserState};
use gloo_timers::callback::Interval;
use log::info;
use routing::Route;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::{events::Event, html, Component, Context, Html};
use yew_router::prelude::*;
use yewdux::dispatch::{Dispatch, Dispatcher};
use yewdux::prelude::BasicStore;

pub enum SignInMessage {
    Tick,
    SignIn,
    Success(String),
    InputUsername(String),
    InputPassword(String),
    UserState(Rc<UserState>),
}

pub enum Stages {
    SignUp,
    Success,
}

pub struct SignInPage {
    interval: Interval,
    username: String,
    password: String,
    dispatch: Dispatch<BasicStore<UserState>>,
    state: Rc<UserState>,
    stage: Stages,
}

impl Component for SignInPage {
    type Message = SignInMessage;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let callback = ctx.link().callback(|_| SignInMessage::Tick);
        let interval = Interval::new(200, move || callback.emit(()));
        let dispatch = Dispatch::bridge_state(ctx.link().callback(SignInMessage::UserState));
        Self {
            username: "".to_string(),
            password: "".to_string(),
            dispatch,
            interval,
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
                self.stage = Stages::Success;
                true
            }
            SignInMessage::UserState(state) => {
                self.state = state;
                true
            }
            _ => true,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        match self.stage {
            Stages::SignUp => {
                html!(
                    <main>
                        <NavComponent/>
                        <div>
                            {self.html_input_username(ctx)}
                            {self.html_input_password(ctx)}
                            {self.html_button_login(ctx)}
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
            <button onclick={ctx.link().callback(|_| SignInMessage::SignIn)}>
                { "Login" }
            </button>
        )
    }

    fn html_input_username(&self, ctx: &Context<Self>) -> Html {
        let change = ctx.link().batch_callback(|e: Event| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            input.map(|input| SignInMessage::InputUsername(input.value()))
        });
        html! {
            <main>
                <label for="username-input">
                    { "Username:" }
                    <input onchange={change}
                        id="username-input"
                        type="text"
                    />
                </label>
            </main>
        }
    }

    fn html_input_password(&self, ctx: &Context<Self>) -> Html {
        let change = ctx.link().batch_callback(|e: Event| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            input.map(|input| SignInMessage::InputPassword(input.value()))
        });
        html! {
            <div>
                <label for="password-input">
                    { "Password:" }
                    <input onchange={change}
                        id="password-input"
                        type="text"
                    />
                </label>
            </div>
        }
    }
}
