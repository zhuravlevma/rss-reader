use crate::api::{get_links, Link};
use crate::UserState;
use gloo_timers::callback::Interval;
use std::rc::Rc;
use yew::{html, Component, Context, Html};
use yewdux::dispatch::Dispatch;
use yewdux::prelude::BasicStore;

pub enum ContentMessage {
    Tick,
    UserState(Rc<UserState>),
    Success(Vec<Link>),
}
pub struct Content {
    dispatch: Dispatch<BasicStore<UserState>>,
    state: Rc<UserState>,
    _interval: Interval,
    links: Vec<Link>,
}
impl Component for Content {
    type Message = ContentMessage;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let callback = ctx.link().callback(|_| ContentMessage::Tick);
        let _interval = Interval::new(200, move || callback.emit(()));
        let dispatch = Dispatch::bridge_state(ctx.link().callback(ContentMessage::UserState));
        Self {
            dispatch,
            _interval,
            state: Default::default(),
            links: vec![],
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ContentMessage::UserState(state) => {
                self.state = state;
                if self.state.token.is_empty() || self.state.token.eq("error") {
                    return true;
                }
                let token = self.state.token.clone();
                ctx.link().send_future(async {
                    match get_links(token).await {
                        Ok(data) => ContentMessage::Success(data),
                        Err(_) => ContentMessage::Success(vec![]),
                    }
                });
                true
            }
            ContentMessage::Success(links) => {
                self.links = links;
                true
            }
            _ => false,
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>{self.html_list()}</div>
        }
    }
}

impl Content {
    fn html_list(&self) -> Html {
        self.links
            .iter()
            .map(|el| {
                html!(
                    <div>{el.link.clone()}</div>
                )
            })
            .collect::<Html>()
    }
}
