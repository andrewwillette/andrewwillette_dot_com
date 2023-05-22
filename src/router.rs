use yew::{function_component, html, Html};
use yew_router::prelude::*;

#[function_component(Music)]
fn music_page() -> Html {
    html! {
    <section>
        <ul>
            // <li onclick={move|_|{home_click_handler.emit("home".to_string());}}>{"Home"}</li>
            <div>{"listen to music"}</div>
        </ul>
    </section>
        }
}

#[derive(Clone, Routable, PartialEq, Debug)]
pub enum RootRoutes {
    #[at("/home")]
    Home,
    #[at("/music")]
    Music,
    #[at("/resume")]
    Resume,
    #[not_found]
    #[at("/404")]
    NotFound,
}

use crate::pages::music::*;

fn switch(routes: &RootRoutes) -> Html {
    let test = crate::pages::music::music_page();
    // match routes {
    //     RootRoutes::Music => html! { <crate::pages::music::Music /> },
    // }
}

#[function_component(RouteOutlet)]
pub fn route_outlet() -> Html {
    html! {
        <BrowserRouter>
            <BaseLayout>
                <Switch<RootRoutes> render={Switch::render(switch)} />
            </BaseLayout>
        </BrowserRouter>
    }
}
