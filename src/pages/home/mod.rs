use crate::components::nav::NavComponent;
use crate::UserState;
use content::Content;
use gloo_timers::callback::Interval;
use std::rc::Rc;
use yew::{html, Component, Context, Html};
use yewdux::dispatch::Dispatch;
use yewdux::prelude::BasicStore;
pub enum Stages {
    Auth,
    UnAuth,
}

pub enum HomeMessage {
    Tick,
    UserState(Rc<UserState>),
}
pub struct HomePage {
    dispatch: Dispatch<BasicStore<UserState>>,
    state: Rc<UserState>,
    _interval: Interval,
    stage: Stages,
}
impl Component for HomePage {
    type Message = HomeMessage;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let callback = ctx.link().callback(|_| HomeMessage::Tick);
        let _interval = Interval::new(200, move || callback.emit(()));
        let dispatch = Dispatch::bridge_state(ctx.link().callback(HomeMessage::UserState));
        Self {
            dispatch,
            _interval,
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
            }
            _ => false,
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        match self.stage {
            Stages::Auth => {
                html! (
                    <main>
                        <NavComponent/>
                        <Content />
                    </main>
                )
            }
            Stages::UnAuth => {
                html!(
                    <main>
                        <NavComponent/>
                        <div>{"UnAuth"}</div>
                    </main>
                )
            }
        }
    }
}

mod content;
