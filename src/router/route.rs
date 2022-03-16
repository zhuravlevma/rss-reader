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
