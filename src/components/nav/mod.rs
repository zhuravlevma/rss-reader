use crate::router::Route;
use crate::store::{AuthState, UserStore};
use std::rc::Rc;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::dispatch::{Dispatch, Dispatcher};
use yewdux::prelude::BasicStore;

pub enum NavMessage {
    UserState(Rc<UserStore>),
    Exit,
}

pub struct NavComponent {
    dispatch: Dispatch<BasicStore<UserStore>>,
    state: Rc<UserStore>,
}
impl Component for NavComponent {
    type Message = NavMessage;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let dispatch = Dispatch::bridge_state(ctx.link().callback(NavMessage::UserState));
        Self {
            dispatch,
            state: Default::default(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            NavMessage::UserState(state) => {
                self.state = state;
                true
            }
            NavMessage::Exit => {
                self.dispatch.reduce(|s| s.auth_state = AuthState::UnAuth);
                self.dispatch.reduce(|s| s.token = "".to_string());
                self.dispatch.reduce(|s| s.user_id = "".to_string());
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        match self.state.auth_state {
            AuthState::UnAuth => {
                html! (
                    <nav class="main-nav">
                        <div class="main-nav-logo-container">
                            <Link <Route> classes={"main-nav-link-logo"} to={Route::Home}>
                                <i class="main-nav-logo fas fa-mail-bulk"></i>
                            </Link<Route>>
                        </div>
                                 <div class="main-nav-buttons">
                                    <Link <Route> classes={"main-nav-link"} to={Route::SignIn}><i class="fas fa-sign-in"></i></Link<Route>>
                                    <Link <Route> classes={"main-nav-link"} to={Route::SignUp}><i class="fas fa-user-plus"></i></Link<Route>>
                                </div>
                    </nav>
                )
            }
            AuthState::Auth => {
                html! (
                    <nav class="main-nav">
                        <div class="main-nav-logo-container">
                            <Link <Route> classes={"main-nav-link-logo"} to={Route::Home}>
                                <i class="main-nav-logo fas fa-mail-bulk"></i>
                            </Link<Route>>
                        </div>
                        <div class="main-nav-group">
                            <div class="nav-exit-button">
                                <Link <Route> classes={"main-nav-link"} to={Route::Settings}>
                                    <i class="fas fa-tools"></i>
                                </Link<Route>>
                             </div>
                            <div class="nav-exit-button">
                                <div class="main-nav-button-exit">
                                    <button onclick={ctx.link().callback(|_| NavMessage::Exit)} class="main-nav-link">
                                        <i class="exit-button fas fa-sign-out-alt"></i>
                                     </button>
                                </div>
                             </div>
                        </div>
                    </nav>
                )
            }
        }
    }
}
