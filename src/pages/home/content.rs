use crate::api::{get_content, get_links, ContentModel, Link};
use crate::UserState;
use gloo_timers::callback::Interval;
use gloo_utils;
use std::rc::Rc;
use web_sys::window;
use yew::{function_component, html, Component, Context, Html, Properties};
use yewdux::dispatch::Dispatch;
use yewdux::prelude::BasicStore;

pub enum ContentMessage {
    Tick,
    UserState(Rc<UserState>),
    Success(Vec<Link>),
    SuccessContentNormal(Vec<ContentModel>),
    Next,
    Back,
}

pub struct ContentPage {
    _dispatch: Dispatch<BasicStore<UserState>>,
    state: Rc<UserState>,
    _interval: Interval,
    links: Vec<Link>,
    content: Vec<ContentModel>,
    start: u32,
    take: u32,
}
impl Component for ContentPage {
    type Message = ContentMessage;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let callback = ctx.link().callback(|_| ContentMessage::Tick);
        let _interval = Interval::new(200, move || callback.emit(()));
        let dispatch = Dispatch::bridge_state(ctx.link().callback(ContentMessage::UserState));
        Self {
            _dispatch: dispatch,
            _interval,
            state: Default::default(),
            links: vec![],
            content: vec![],
            start: 0,
            take: 15,
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
                let start = self.start;
                let take = self.take;
                ctx.link().send_future(async move {
                    match get_content(token, start, take).await {
                        Ok(data) => ContentMessage::SuccessContentNormal(data),
                        Err(_) => ContentMessage::SuccessContentNormal(vec![]),
                    }
                });
                true
            }
            ContentMessage::Next => {
                self.start += 15;
                let take = self.take;
                let start = self.start;
                let token = self.state.token.clone();
                ctx.link().send_future(async move {
                    match get_content(token, start, take).await {
                        Ok(data) => ContentMessage::SuccessContentNormal(data),
                        Err(_) => ContentMessage::SuccessContentNormal(vec![]),
                    }
                });
                false
            }
            ContentMessage::Back => {
                self.start -= 15;
                let take = self.take;
                let start = self.start;
                let token = self.state.token.clone();
                ctx.link().send_future(async move {
                    match get_content(token, start, take).await {
                        Ok(data) => ContentMessage::SuccessContentNormal(data),
                        Err(_) => ContentMessage::SuccessContentNormal(vec![]),
                    }
                });
                false
            }
            ContentMessage::Success(links) => {
                self.links = links;
                true
            }
            ContentMessage::SuccessContentNormal(content) => {
                window().unwrap().scroll_to_with_x_and_y(0.0, 0.0);
                self.content = content;
                true
            }
            _ => false,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="content">
                <div class="container-links">
                    <div class="links-header">{"Links"}</div>
                    <ul>{self.html_list()}</ul>
                </div>
                <div class="container-content">
                    <ul class="content-list">
                        <ul>{self.get_content()}</ul>
                    </ul>
                    <div class="content-paging">
                        <button onclick={ctx.link().callback(|_| ContentMessage::Back)} type="button" class="content-paging-button"><i class="fas fa-angle-left"></i></button>
                        <div class="content-paging-info">
                            <p>{self.start}</p>
                            <p>{"    ...   "}</p>
                            <p>{self.start + self.take}</p>
                        </div>
                        <button onclick={ctx.link().callback(|_| ContentMessage::Next)} type="button" class="content-paging-button"><i class="fas fa-angle-right"></i></button>
                    </div>
                </div>
            </div>
        }
    }
}

impl ContentPage {
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
        self.content
            .iter()
            .map(|el| {
                html!(
                    <li class="content-element">
                        <div class="content-title">
                                <a target="_blank" href={el.link_url.clone()}>{el.title.clone()}</a>
                        </div>
                        <div class="content-desc"><SafeHtml html={match &el.description {
                                Some(desc) => desc.to_string().clone(),
                                None => "".to_string()
                        }}/></div>
                    </li>
                )
            })
            .collect::<Html>()
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub html: String,
}

#[function_component(SafeHtml)]
pub fn safe_html(props: &Props) -> Html {
    let div = gloo_utils::document().create_element("div").unwrap();
    div.set_inner_html(&props.html.clone());
    Html::VRef(div.into())
}
