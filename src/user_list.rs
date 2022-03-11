use crate::api::{get_users, User};
use gloo_timers::callback::Interval;
use log::info;
use yew::{html, Component, Context, Html};

pub enum UserListMessage {
    Tick,
    Show,
    Success(Vec<User>),
}

pub struct UserList {
    interval: Interval,
    users: Vec<User>,
}

impl Component for UserList {
    type Message = UserListMessage;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let callback = ctx.link().callback(|_| UserListMessage::Tick);
        let interval = Interval::new(200, move || callback.emit(()));
        Self {
            users: vec![],
            interval,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            UserListMessage::Show => {
                ctx.link().send_future(async {
                    match get_users().await {
                        Ok(data) => UserListMessage::Success(data),
                        Err(_) => UserListMessage::Success(vec![]),
                    }
                });
                false
            }
            UserListMessage::Success(data) => {
                info!("{:?}", data);
                self.users = data;
                true
            }
            _ => true,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html!(
            <div>
                {self.html_show_button(ctx)}
                {self.html_list()}
            </div>
        )
    }
}

impl UserList {
    fn html_list(&self) -> Html {
        self.users
            .iter()
            .map(|el| {
                html!(
                    <div>{el.username.clone()}</div>
                )
            })
            .collect::<Html>()
    }
    fn html_show_button(&self, ctx: &Context<Self>) -> Html {
        html!(
            <button onclick={ctx.link().callback(|_| UserListMessage::Show)}>
                        { "Show users" }
                </button>
        )
    }
}
