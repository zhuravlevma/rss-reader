use reqwasm::http::{ReadableStream, Request, RequestMode};
use std::error::Error;
use yew::{html, Component, Context, Html};

pub enum Msg {
    Test,
    Success(String),
}

pub struct Card {
    content: String,
}

pub async fn http_get() -> Result<String, Box<dyn Error>> {
    let res = Request::get("https://habr.com/ru/rss/flows/management/all/?fl=ru")
        .mode(RequestMode::NoCors)
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
        Self {
            content: "".to_string(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        println!("{}", "click");
        match msg {
            Msg::Test => {
                ctx.link().send_future(async {
                    match http_get().await {
                        Ok(data) => Msg::Success(data),
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
