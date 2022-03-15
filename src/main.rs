use router::{Route, switch};
use yew::prelude::*;
use yew_router::prelude::*;

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

mod components;
mod pages;
mod store;
mod router;
mod dto;
mod api;
