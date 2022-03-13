use crate::routing::Route;
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
                        <img class="main-nav-logo" src ="https://user-images.githubusercontent.com/44276887/158073533-5cdf800e-446f-4516-9ba6-4516fc1d9456.png" alt="logo"/>
                    </Link<Route>>
                    <p>{"RSS reader"}</p>
                </div>
                <div class="main-nav-buttons">
                    <Link <Route> classes={"main-nav-link"} to={Route::SignIn}>{ "SignIn" }</Link<Route>>
                    <Link <Route> classes={"main-nav-link"} to={Route::SignUp}>{ "SignUp" }</Link<Route>>
                </div>
            </nav>
        }
    }
}
