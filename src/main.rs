use yew::prelude::*;
enum Msg {}
use card::Card;

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
            </div>
        }
    }
}

fn main() {
    yew::start_app::<RootComponent>();
}

mod card;
