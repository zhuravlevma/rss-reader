use super::auth::{sign_in::SignIn, sign_up::SignUp};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    SignIn,
    #[at("/signup")]
    SignUp,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <h1>{ "Home" }</h1> },
        Route::SignIn => html! {
            <SignIn />
        },
        Route::SignUp => html! {
            <SignUp />
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}
