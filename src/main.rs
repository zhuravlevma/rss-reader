use crate::pages::home::HomePage;
use routing::switch;
use routing::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Default)]
pub struct UserState {
    token: String,
    user_id: String,
}

enum Msg {}
struct RootComponent {}
impl Component for RootComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <BrowserRouter>
                <Switch<Route> render={Switch::render(switch)} />
            </BrowserRouter>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<RootComponent>();
}

mod api;
mod card;
mod components;
mod pages;
mod routing;
mod user_list;
