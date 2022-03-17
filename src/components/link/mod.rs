use crate::api::get_links;
use crate::dto::LinkDto;
use crate::store::UserStore;
use log::info;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;
use yewdux::dispatch::Dispatch;
use yewdux::prelude::BasicStore;

pub enum LinkMessage {
    UserState(Rc<UserStore>),
    Success(Vec<LinkDto>),
    InputLink(String),
    Add,
}

pub struct LinkComponent {
    _dispatch: Dispatch<BasicStore<UserStore>>,
    state: Rc<UserStore>,
    links: Vec<LinkDto>,
    link: String,
}
impl Component for LinkComponent {
    type Message = LinkMessage;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let dispatch = Dispatch::bridge_state(ctx.link().callback(LinkMessage::UserState));
        Self {
            _dispatch: dispatch,
            state: Default::default(),
            links: vec![],
            link: "".to_string(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            LinkMessage::UserState(state) => {
                self.state = state;
                if self.state.token.is_empty() || self.state.token.eq("error") {
                    return true;
                }
                let token = self.state.token.clone();
                ctx.link().send_future(async {
                    match get_links(token).await {
                        Ok(data) => LinkMessage::Success(data),
                        Err(_) => LinkMessage::Success(vec![]),
                    }
                });
                true
            }
            LinkMessage::Success(data) => {
                self.links = data;
                true
            }
            LinkMessage::InputLink(data) => {
                self.link = data;
                true
            }
            LinkMessage::Add => {
                info!("{}", self.link);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let change = |e: FocusEvent| e.prevent_default();
        html!(
            <div class="container-links">
                <div class="links-header-container">
                    <i class="fa-solid fa-link link-icon"></i><div class="links-header">{"links"}</div>
                </div>
                <div class="form-container-link">
                    <form class="form-link" onsubmit={change}>
                        {self.html_input_link(ctx)}
                        {self.html_button_login(ctx)}
                    </form>
                </div>
                <ul>{self.html_list()}</ul>
            </div>
        )
    }
}

impl LinkComponent {
    fn html_list(&self) -> Html {
        self.links
            .iter()
            .map(|el| {
                html!(
                    <li class = "link">
                        <div class = "link-main">
                            <div class = "link-info">
                                <button><i class="fa-regular fa-trash-can link-trash"></i></button>
                                <label class="link-name-content" for="checkbox">{el.name.clone()}</label>
                                <div class = "link-description">
                                    <a target = "_blank" class = "link-href-content" href={el.link.clone()}>{el.link.clone()}</a>
                                </div>
                            </div>
                        </div>
                    </li>
                )
            })
            .collect::<Html>()
    }

    fn html_input_link(&self, ctx: &Context<Self>) -> Html {
        let change: Callback<Event> = ctx.link().batch_callback(|e: Event| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            input.map(|input| LinkMessage::InputLink(input.value()))
        });
        html! {
            <div class="form-element-link">
                <label class="form-element-link-label" for="username-input">
                    { "Link url" }
                </label>
                <input class="form-element-link-input" onchange={change}
                        id="username-input"
                        type="text"
                />
            </div>
        }
    }

    fn html_button_login(&self, ctx: &Context<Self>) -> Html {
        html!(
            <div class="form-element-link">
                <button class="form-element-link-button" onclick={ctx.link().callback(|_| LinkMessage::Add)}>
                    {"add "}<i class="fa-solid fa-plus"></i>
                </button>
            </div>
        )
    }
}
