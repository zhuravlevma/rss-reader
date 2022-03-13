use crate::components::nav::NavComponent;
use crate::routing::Route;
use crate::UserState;
use gloo_timers::callback::Interval;
use std::rc::Rc;
use yew::prelude::*;
use yew_router::prelude::*;
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
    interval: Interval,
    stage: Stages,
}
impl Component for HomePage {
    type Message = HomeMessage;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let callback = ctx.link().callback(|_| HomeMessage::Tick);
        let interval = Interval::new(200, move || callback.emit(()));
        let dispatch = Dispatch::bridge_state(ctx.link().callback(HomeMessage::UserState));
        Self {
            dispatch,
            interval,
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

    fn view(&self, ctx: &Context<Self>) -> Html {
        match self.stage {
            Stages::Auth => {
                html! (
                    <main>
                        <NavComponent/>
                        <div>{"Home page"}</div>
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
