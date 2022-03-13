use crate::pages::{home::HomePage, sign_in::SignInPage, sign_up::SignUpPage};
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
        Route::Home => html! {
            <HomePage />
        },
        Route::SignIn => html! {
            <SignInPage />
        },
        Route::SignUp => html! {
            <SignUpPage />
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}
