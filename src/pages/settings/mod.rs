use yew::{html, Component, Context, Html};

use crate::components::link::LinkComponent;
use crate::components::nav::NavComponent;

pub enum SettingsMessage {}

pub struct SettingsPage {}

impl Component for SettingsPage {
    type Message = SettingsMessage;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! (
            <div class="settings-container">
                <NavComponent/>
                <LinkComponent />
            </div>
        )
    }
}