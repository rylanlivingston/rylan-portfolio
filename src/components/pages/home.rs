use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <>
            <div class="name">
                <h1>{"Rylan Livingston"}</h1>
                <h2>{"Student"}</h2>
            </div>
            <p>
                {"Interested in cybersecurity and software development. I enjoy reading books and playing the guitar!"}
            </p>
        </>
    }
}
