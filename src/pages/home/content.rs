use crate::api::{get_content, get_links, Link, LinkWithContent};
use crate::UserState;
use gloo_timers::callback::Interval;
use log::info;
use std::error::Error;
use std::rc::Rc;
use yew::{html, Component, Context, Html};
use yewdux::dispatch::Dispatch;
use yewdux::prelude::BasicStore;

pub enum ContentMessage {
    Tick,
    UserState(Rc<UserState>),
    Success(Vec<Link>),
    SuccessContent(Vec<LinkWithContent>),
}
pub struct Content {
    dispatch: Dispatch<BasicStore<UserState>>,
    state: Rc<UserState>,
    _interval: Interval,
    links: Vec<Link>,
    content: Vec<LinkWithContent>,
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
            content: vec![],
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
                let token = self.state.token.clone();
                ctx.link().send_future(async {
                    match get_content(token).await {
                        Ok(data) => ContentMessage::SuccessContent(data),
                        Err(_) => ContentMessage::SuccessContent(vec![]),
                    }
                });
                true
            }
            ContentMessage::Success(links) => {
                self.links = links;
                true
            }
            ContentMessage::SuccessContent(content) => {
                self.content = content;
                true
            }
            _ => false,
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="content">
                <div class="container-links">
                    <div class="links-header">{"Links"}</div>
                    <ul>{self.html_list()}</ul>
                </div>
                <ul class="container-content">
                    <div class="content-header">{"Messages"}</div>
                    <ul>{self.get_content()}</ul>
                </ul>
            </div>
        }
    }
}

impl Content {
    fn html_list(&self) -> Html {
        self.links
            .iter()
            .map(|el| {
                html!(
                    <li class = "link">
                        <i class="fa-solid fa-link link-icon"></i>
                        <div class = "link-main">
                            <div class = "link-name">
                                <p class = "link-name-label">{"name:"}</p>
                                <p class = "link-name-content">{el.name.clone()}</p>
                            </div>
                            <div class = "link-href">
                                <p class = "link-href-label">{"link:"}</p>
                                <a target = "_blank" class = "link-href-content" href={el.link.clone()}>{el.link.clone()}</a>
                            </div>
                            <div class = "link-description">
                                <p class = "link-description-label">{"Description:"}</p>
                                <p class = "link-description-content">{el.description.clone()}</p>
                            </div>
                        </div>
                    </li>
                )
            })
            .collect::<Html>()
    }

    fn get_content(&self) -> Html {
        let mut messages = vec![];
        for elem in &self.content {
            for content in &elem.content {
                messages.push(content)
            }
        }
        messages
            .iter()
            .map(|el| {
                html!(
                    <li>
                        <div>{el.link_url.clone()}</div>
                        <div>{el.title.clone()}</div>
                    </li>
                )
            })
            .collect::<Html>()
    }
}
