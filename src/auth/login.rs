use crate::api::{login, User};
use crate::user_list::UserListMessage;
use gloo_timers::callback::Interval;
use log::info;
use serde::de::Unexpected::Str;
use wasm_bindgen::JsCast;
use web_sys::console::info;
use web_sys::{EventTarget, HtmlInputElement};
use yew::{events::Event, html, Component, Context, Html};

pub enum LoginMessage {
    Tick,
    Login,
    Success(String),
    InputUsername(String),
    InputPassword(String),
}

pub struct Login {
    interval: Interval,
    token: String,
    username: String,
    password: String,
}

impl Component for Login {
    type Message = LoginMessage;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let callback = ctx.link().callback(|_| LoginMessage::Tick);
        let interval = Interval::new(200, move || callback.emit(()));
        Self {
            token: "undefined".to_string(),
            username: "".to_string(),
            password: "".to_string(),
            interval,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            LoginMessage::InputUsername(username) => {
                self.username = username;
                true
            }
            LoginMessage::InputPassword(password) => {
                self.password = password;
                true
            }
            LoginMessage::Login => {
                let username = self.username.clone();
                let password = self.password.clone();
                info!("username: {}; password: {}", username, password);
                ctx.link().send_future(async {
                    match login(username, password).await {
                        Ok(data) => LoginMessage::Success(data.access_token),
                        Err(_) => LoginMessage::Success("error".to_string()),
                    }
                });
                false
            }
            LoginMessage::Success(token) => {
                info!("{}", token);
                self.token = token;
                true
            }
            _ => true,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html!(
            <div>
                {self.html_input_username(ctx)}
                {self.html_input_password(ctx)}
                {self.html_button_login(ctx)}
            </div>
        )
    }
}

impl Login {
    fn html_button_login(&self, ctx: &Context<Self>) -> Html {
        html!(
            <button onclick={ctx.link().callback(|_| LoginMessage::Login)}>
                { "Login" }
            </button>
        )
    }

    fn html_input_username(&self, ctx: &Context<Self>) -> Html {
        let change = ctx.link().batch_callback(|e: Event| {
            // When events are created the target is undefined, it's only
            // when dispatched does the target get added.
            let target: Option<EventTarget> = e.target();
            // Events can bubble so this listener might catch events from child
            // elements which are not of type HtmlInputElement
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

            input.map(|input| LoginMessage::InputUsername(input.value()))
        });
        html! {
            <div>
                <label for="username-input">
                    { "Username:" }
                    <input onchange={change}
                        id="username-input"
                        type="text"
                    />
                </label>
            </div>
        }
    }

    fn html_input_password(&self, ctx: &Context<Self>) -> Html {
        let change = ctx.link().batch_callback(|e: Event| {
            // When events are created the target is undefined, it's only
            // when dispatched does the target get added.
            let target: Option<EventTarget> = e.target();
            // Events can bubble so this listener might catch events from child
            // elements which are not of type HtmlInputElement
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

            input.map(|input| LoginMessage::InputPassword(input.value()))
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
