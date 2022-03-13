use crate::components::nav::NavComponent;
use crate::routing::Route;
use yew::prelude::*;
use yew_router::prelude::*;

pub enum Msg {}
pub struct HomePage {}
impl Component for HomePage {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <main>
                <NavComponent/>
                <div>{"Home page"}</div>
            </main>
        }
    }
}
