use crate::api::remove_link;
use yew::{html, Component, Context, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct LinkProps {
    pub link: LinkData,
}

pub enum LinkMessage {
    Remove,
    Success(bool),
}

#[derive(Clone, PartialEq)]
pub struct LinkData {
    pub link_id: String,
    pub link: String,
    pub token: String,
}

pub enum LinkState {
    Visible,
    None,
}

pub struct Link {
    state: LinkState,
}

impl Component for Link {
    type Message = LinkMessage;
    type Properties = LinkProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            state: LinkState::Visible,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            LinkMessage::Remove => {
                let token = ctx.props().link.token.clone();
                let link_id = ctx.props().link.link_id.clone();
                ctx.link().send_future(async {
                    match remove_link(token, link_id).await {
                        Ok(data) => LinkMessage::Success(data),
                        Err(_) => LinkMessage::Success(false),
                    }
                });
                true
            }
            LinkMessage::Success(data) => {
                if data {
                    self.state = LinkState::None
                }
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        match self.state {
            LinkState::Visible => {
                html! (
                    <li class = "link">
                        <div class = "link-main">
                            <div class = "link-info row-direction">
                               <button class="button-icon" onclick={ctx.link().callback(|_| LinkMessage::Remove)}>
                                    <i class="fa-regular fa-trash-can link-trash"></i>
                                </button>
                                <div class = "link-description">
                                    <a target = "_blank" class = "link-href-content" href={ctx.props().link.link.clone()}>{ctx.props().link.link.clone()}</a>
                                </div>
                            </div>
                        </div>
                    </li>
                )
            }
            LinkState::None => {
                html!()
            }
        }
    }
}
