use yew::prelude::*;
use yew::{function_component, html, Html};
use yew::{Callback, Properties};

fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component]
fn App() -> Html {
    html! {
        <div>
        <h1>
        // <Banner selected={false}/>
        </h1>
            {"garbage"}
        </div>
    }
}

#[function_component]
fn Banner(props: &Props) -> Html {
    let home_click_handler: Callback<_> = Callback::from(move |id: String| {
        web_sys::console::log_1(&id.into());
        // _props.last_clicked = id;
    });
    let blog_click_handler: Callback<_> = Callback::from(move |id: String| {
        web_sys::console::log_1(&id.into());
    });
    let music_click_handler: Callback<_> = Callback::from(move |id: String| {
        web_sys::console::log_1(&id.into());
    });
    let resume_click_handler: Callback<_> = Callback::from(move |id: String| {
        web_sys::console::log_1(&id.into());
        // props.selected =
    });
    let counter = use_state(|| 0);
    let onclick: Callback<_> = {
        let counter = counter.clone();
        Callback::from(move |id: String| counter.set(*counter + 1))
    };

    html! {
    <section>
        <ul>
            // <li onclick={move|_|{home_click_handler.emit("home".to_string());}}>{"Home"}</li>
            <li onclick={move|_|{home_click_handler.emit("home".to_string());}}>{"Home"}</li>
            <li onclick={move|_|{blog_click_handler.emit("blog".to_string());}}>{"Blog"}</li>
            <li onclick={move|_|{music_click_handler.emit("music".to_string());}}>{"Music"}</li>
            <li onclick={move|_|{resume_click_handler.emit("resume".to_string());}}>{"Resume"}</li>
            <div>{props.selected}</div>
            <div>{props.last_clicked}</div>
        </ul>
    </section>
        }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Page {
    Home,
    Blog,
    Music,
    Resume,
}

impl ToString for Page {
    fn to_string(&self) -> String {
        match self {
            Page::Home => "Home".to_string(),
            Page::Blog => "Blog".to_string(),
            Page::Music => "Music".to_string(),
            Page::Resume => "Resume".to_string(),
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub selected: bool,
    #[prop_or(Page::Home)]
    pub last_clicked: Page,
}
