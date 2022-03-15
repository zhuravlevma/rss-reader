use yew::{Html, html};
use crate::pages::{home::HomePage, sign_in::SignInPage, sign_up::SignUpPage};
use crate::router::Route;

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