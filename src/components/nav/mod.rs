use crate::router::Route;
use yew::prelude::*;
use yew_router::prelude::*;

pub enum Msg {}
pub struct NavComponent {}
impl Component for NavComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
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
        }
    }
}
