use crate::components::link::LinkComponent;
use crate::components::nav::NavComponent;
use crate::router::Route;
use crate::store::UserStore;
use std::rc::Rc;
use yew::{html, Component, Context, Html};
use yew_router::prelude::*;
use yewdux::dispatch::Dispatch;
use yewdux::prelude::BasicStore;

pub enum Stages {
    Auth,
    UnAuth,
}

pub enum SettingsMessage {
    UserState(Rc<UserStore>),
}

pub struct SettingsPage {
    _dispatch: Dispatch<BasicStore<UserStore>>,
    state: Rc<UserStore>,
    stage: Stages,
}

impl Component for SettingsPage {
    type Message = SettingsMessage;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let dispatch = Dispatch::bridge_state(ctx.link().callback(SettingsMessage::UserState));

        Self {
            _dispatch: dispatch,
            state: Rc::new(Default::default()),
            stage: Stages::Auth,
        }
    }
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            SettingsMessage::UserState(state) => {
                self.state = state;
                if self.state.token.is_empty() || self.state.token.eq("error") {
                    self.stage = Stages::UnAuth
                } else {
                    self.stage = Stages::Auth
                }
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        match self.stage {
            Stages::Auth => {
                html! (
                    <main>
                        <div class="settings-container">
                            <NavComponent/>
                            <LinkComponent />
                        </div>
                    </main>
                )
            }
            Stages::UnAuth => {
                html!(
                    <Redirect<Route> to={Route::Home}/>
                )
            }
        }
    }
}
