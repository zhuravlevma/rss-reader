use auth::login::Login;
use card::Card;
use user_list::UserList;
use yew::prelude::*;
use yewdux::prelude::*;

#[derive(Clone, Default)]
pub struct TokenState {
    token: String,
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
            <div>
                <Card></Card>
                <UserList></UserList>
                <Login/>
            </div>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<RootComponent>();
}

mod api;
mod auth;
mod card;
mod user_list;
