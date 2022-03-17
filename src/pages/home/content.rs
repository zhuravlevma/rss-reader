use yew::{html, Component, Context, Html};

use crate::components::link::LinkComponent;
use crate::components::news::NewsComponent;

pub enum ContentMessage {}

pub struct ContentPage {}

impl Component for ContentPage {
    type Message = ContentMessage;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! (
            <div class="content">
                <LinkComponent />
                <NewsComponent />
            </div>
        )
    }
}
