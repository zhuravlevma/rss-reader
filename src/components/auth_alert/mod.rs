use crate::router::Route;
use yew::prelude::*;
use yew_router::prelude::*;

pub enum Msg {}
pub struct AuthAlertComponent {}
impl Component for AuthAlertComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="auth-alert-container">
                <div class="auth-alert-container-info">
                    <img class="auth-alert-image" src="https://user-images.githubusercontent.com/44276887/158201708-b2a1a4a9-9190-43d9-b135-65a3e7879612.png"/>
                </div>
                <div class="auth-alert-container-links">
                    {"Welcome. Please"}<Link <Route> classes={"auth-alert-link"} to={Route::SignIn}>
                       {"  log in  "}
                    </Link<Route>>{"or"}<Link <Route> classes={"auth-alert-link"} to={Route::SignUp}>
                           {"  register  "}
                    </Link<Route>>
                </div>
            </div>
        }
    }
}
