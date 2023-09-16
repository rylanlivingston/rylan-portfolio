use router::{switch, Route};
use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod router;

#[function_component(Navbar)]
pub fn navbar() -> Html {
    html! {
        <div class="navbar">
            <ul>
                <div class="nav_item">
                    <li><Link<Route> to={Route::Contact}>{ "Contact" }</Link<Route>></li>
                    <li><Link<Route> to={Route::Resume}>{ "Resume" }</Link<Route>></li>
                    <li><Link<Route> to={Route::Home}>{ "Home" }</Link<Route>></li>
                </div>
            </ul>
        </div>
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Navbar />
            <main>
                <Switch<Route>render={switch} />
            </main>
        </BrowserRouter>
    }
}
