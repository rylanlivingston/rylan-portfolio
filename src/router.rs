use crate::components::pages::{contact::Contact, home::Home, resume::Resume};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Debug, Clone, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/contact")]
    Contact,
    #[at("/resume")]
    Resume,
}

pub fn switch(route: Route)    -> Html {
    match route {   
        Route::Home => html!{<Home />},
        Route::Contact => html!{<Contact />},
        Route::Resume => html!{<Resume />},
    }
}
