use crate::api::sign_up_api;
use gloo_timers::callback::Interval;
use log::info;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::{events::Event, html, Component, Context, Html};

pub enum SignUpMessage {
    Tick,
    SignUp,
    Success(String),
    InputUsername(String),
    InputPassword(String),
    InputPasswordRepeat(String),
}

pub struct SignUp {
    interval: Interval,
    username: String,
    password: String,
    password_repeat: String,
}

impl Component for SignUp {
    type Message = SignUpMessage;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let callback = ctx.link().callback(|_| SignUpMessage::Tick);
        let interval = Interval::new(200, move || callback.emit(()));
        Self {
            username: "".to_string(),
            password: "".to_string(),
            password_repeat: "".to_string(),
            interval,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            SignUpMessage::InputUsername(username) => {
                self.username = username;
                true
            }
            SignUpMessage::InputPassword(password) => {
                self.password = password;
                true
            }
            SignUpMessage::InputPasswordRepeat(password) => {
                self.password_repeat = password;
                true
            }
            SignUpMessage::SignUp => {
                let username = self.username.clone();
                let password = self.password.clone();
                let eq = password.eq(&self.password_repeat);
                if !eq {
                    return false;
                }
                info!("username: {}; password: {}", username, password);
                ctx.link().send_future(async {
                    match sign_up_api(username, password).await {
                        Ok(data) => SignUpMessage::Success(data.user_id),
                        Err(_) => SignUpMessage::Success("error".to_string()),
                    }
                });
                false
            }
            SignUpMessage::Success(user_id) => {
                info!("{}", user_id);
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
                {self.html_input_repeat_password(ctx)}
                {self.html_button_signup(ctx)}
            </div>
        )
    }
}

impl SignUp {
    fn html_button_signup(&self, ctx: &Context<Self>) -> Html {
        html!(
            <button onclick={ctx.link().callback(|_| SignUpMessage::SignUp)}>
                { "SignUp" }
            </button>
        )
    }

    fn html_input_username(&self, ctx: &Context<Self>) -> Html {
        let change = ctx.link().batch_callback(|e: Event| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            input.map(|input| SignUpMessage::InputUsername(input.value()))
        });
        html! {
            <div>
                <label for="username-input-signup">
                    { "Username:" }
                    <input onchange={change}
                        id="username-input-signup"
                        type="text"
                    />
                </label>
            </div>
        }
    }

    fn html_input_password(&self, ctx: &Context<Self>) -> Html {
        let change = ctx.link().batch_callback(|e: Event| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            input.map(|input| SignUpMessage::InputPassword(input.value()))
        });
        html! {
            <div>
                <label for="password-input-signup">
                    { "Password:" }
                    <input onchange={change}
                        id="password-input-signup"
                        type="text"
                    />
                </label>
            </div>
        }
    }

    fn html_input_repeat_password(&self, ctx: &Context<Self>) -> Html {
        let change = ctx.link().batch_callback(|e: Event| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            input.map(|input| SignUpMessage::InputPasswordRepeat(input.value()))
        });
        html! {
            <div>
                <label for="password-input-signup-repeat">
                    { "Password repeat:" }
                    <input onchange={change}
                        id="password-input-signup-repeat"
                        type="text"
                    />
                </label>
            </div>
        }
    }
}
