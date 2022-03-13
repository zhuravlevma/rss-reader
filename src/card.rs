use crate::UserState;
use gloo_timers::callback::Interval;
use reqwasm::http::Request;
use std::error::Error;
use std::rc::Rc;
use yew::{html, Component, Context, Html};
use yewdux::dispatch::Dispatch;
use yewdux::prelude::BasicStore;

pub enum Msg {
    Test,
    Success(String),
    Tick,
    State(Rc<UserState>),
}

pub struct Card {
    content: String,
    interval: Interval,
    dispatch: Dispatch<BasicStore<UserState>>,
    state: Rc<UserState>,
}

pub async fn http_get() -> Result<String, Box<dyn Error>> {
    let res = Request::get("https://jsonplaceholder.typicode.com/todos/1")
        .send()
        .await
        .unwrap();
    let res = res.text().await.unwrap();
    Ok(res)
}

impl Component for Card {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let callback = ctx.link().callback(|_| Msg::Tick);
        let interval = Interval::new(200, move || callback.emit(()));
        let dispatch = Dispatch::bridge_state(ctx.link().callback(Msg::State));
        Self {
            content: "".to_string(),
            dispatch,
            interval,
            state: Default::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Test => {
                ctx.link().send_future(async {
                    match http_get().await {
                        Ok(data) => Msg::Success(data),
                        Err(_) => Msg::Success("error".to_string()),
                    }
                });
                false
            }
            Msg::Success(data) => {
                self.content = data;
                true
            }
            Msg::State(state) => {
                self.state = state;
                true
            }
            _ => true,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html!(
            <div>
                <button onclick={ctx.link().callback(|_| Msg::Test)}>
                        { "Get" }
                </button>
                <div>{self.content.clone()}</div>
            </div>
        )
    }
}
