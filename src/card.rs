use gloo_timers::callback::Interval;
use reqwasm::http::{ReadableStream, Request, RequestMode};
use std::error::Error;
use yew::{html, Component, Context, Html};

pub enum Msg {
    Test,
    Success(String),
    Tick,
}

pub struct Card {
    content: String,
    interval: Interval,
}

pub async fn http_get() -> Result<String, Box<dyn Error>> {
    let res = Request::get("https://jsonplaceholder.typicode.com/todos/1")
        // .mode(RequestMode::NoCors)
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
        Self {
            content: "".to_string(),
            interval,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        println!("{}", "click");
        match msg {
            Msg::Test => {
                ctx.link().send_future(async {
                    match http_get().await {
                        Ok(data) => {
                            println!("{}", data);
                            Msg::Success(data)
                        }
                        Err(_) => Msg::Success("error".to_string()),
                    }
                });
                // ctx.link()
                //     .send_message(Msg::Fetching);
                false
            }
            Msg::Success(data) => {
                println!("{}", data);
                self.content = data;
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
