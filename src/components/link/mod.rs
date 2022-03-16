use std::rc::Rc;
use yew::prelude::*;
use yewdux::dispatch::Dispatch;
use yewdux::prelude::BasicStore;
use crate::api::get_links;
use crate::dto::LinkDto;
use crate::store::UserStore;

pub enum LinkMessage {
    UserState(Rc<UserStore>),
    Success(Vec<LinkDto>)
}

pub struct LinkComponent {
    _dispatch: Dispatch<BasicStore<UserStore>>,
    state: Rc<UserStore>,
    links: Vec<LinkDto>,
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
            },
            LinkMessage::Success(data) => {
                self.links = data;
                true
            }
        }
    }


    fn view(&self, _ctx: &Context<Self>) -> Html {
        html!(
            <div class="container-links">
                <div class="links-header-container">
                    <i class="fa-solid fa-link link-icon"></i><div class="links-header">{"links"}</div>
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
}