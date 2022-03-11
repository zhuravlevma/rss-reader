use card::Card;
use user_list::UserList;
use yew::prelude::*;

enum Msg {}
struct RootComponent {
    value: i64,
}
impl Component for RootComponent {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self { value: 0 }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <button>{ "+1" }</button>
                <Card></Card>
                <UserList></UserList>
            </div>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<RootComponent>();
}

mod api;
mod card;
mod user_list;
