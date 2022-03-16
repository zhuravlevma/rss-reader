use crate::components::auth_alert::AuthAlertComponent;
use crate::components::nav::NavComponent;
use content::ContentPage;
use std::rc::Rc;
use yew::{html, Component, Context, Html};
use yewdux::dispatch::Dispatch;
use yewdux::prelude::BasicStore;
use crate::store::{UserStore, AuthState};

pub enum Stages {
    Auth,
    UnAuth,
}

pub enum HomeMessage {
    UserState(Rc<UserStore>),
}
pub struct HomePage {
    _dispatch: Dispatch<BasicStore<UserStore>>,
    state: Rc<UserStore>,
    stage: Stages,
}
impl Component for HomePage {
    type Message = HomeMessage;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let dispatch = Dispatch::bridge_state(ctx.link().callback(HomeMessage::UserState));
        Self {
            _dispatch: dispatch,
            state: Default::default(),
            stage: Stages::UnAuth,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            HomeMessage::UserState(state) => {
                self.state = state;
                if self.state.token.is_empty() || self.state.token.eq("error") {
                    self.stage = Stages::UnAuth
                } else {
                    self.stage = Stages::Auth
                }
                true
            },
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        match self.state.auth_state {
            AuthState::Auth => {
                html! (
                    <main>
                        <NavComponent/>
                        <ContentPage />
                    </main>
                )
            }
            AuthState::UnAuth => {
                html!(
                    <main>
                        <NavComponent/>
                        <AuthAlertComponent/>
                    </main>
                )
            }
            // AuthState::Auth => {}
            // AuthState::UnAuth => {}
        }
    }
}

mod content;
