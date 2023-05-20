use yew::prelude::*;
use yew::{function_component, html, Html};
use yew::{Callback, Properties};

fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component]
fn App() -> Html {
    let current_page = use_state(String::new);
    let home_click_callback: Callback<_> = {
        let current_page = current_page.clone();
        Callback::from(move |_| {
            current_page.set("Home".to_string());
        })
    };
    let blog_click_callback: Callback<_> = {
        let current_page = current_page.clone();
        Callback::from(move |_: String| {
            current_page.set("Blog".to_string());
        })
    };
    let music_click_callback: Callback<_> = {
        let current_page = current_page.clone();
        Callback::from(move |_: String| {
            current_page.set("Music".to_string());
        })
    };
    let resume_click_callback: Callback<_> = {
        let current_page = current_page.clone();
        Callback::from(move |_: String| {
            current_page.set("Resume".to_string());
        })
    };
    html! {
        <div>
        <h1>
        <Banner selected={false} {home_click_callback} {blog_click_callback} {resume_click_callback} {music_click_callback}/>
        </h1>
            {"garbage"}
            <div>{&*current_page.to_string()}</div>
        </div>
    }
}

#[function_component(Banner)]
fn banner(props: &Props) -> Html {
    // let current_page = use_state(String::new);
    // let home_click_handler: Callback<_> = {
    //     let current_page = current_page.clone();
    //     Callback::from(move |_| {
    //         current_page.set("Home".to_string());
    //     })
    // };
    // let blog_click_handler: Callback<_> = {
    //     let current_page = current_page.clone();
    //     Callback::from(move |_| {
    //         current_page.set("Blog".to_string());
    //     })
    // };
    // let music_click_handler: Callback<_> = {
    //     let current_page = current_page.clone();
    //     Callback::from(move |_| {
    //         current_page.set("Music".to_string());
    //     })
    // };
    // let resume_click_handler: Callback<_> = {
    //     let current_page = current_page.clone();
    //     Callback::from(move |_| {
    //         current_page.set("Resume".to_string());
    //     })
    // };
    // let counter = use_state(|| 0);
    // let _onclick: Callback<_> = {
    //     let counter = counter.clone();
    //     Callback::from(move |_id: String| counter.set(*counter + 1))
    // };

    html! {
    <section>
        <ul>
            // <li onclick={move|_|{props.home_click_callback.emit("home".to_string());}}>{"Home"}</li>
            // <li onclick={move|_|{props.home_click_callback.emit("blog".to_string());}}>{"Blog"}</li>
            // <li onclick={move|_|{props.music_click_callback.emit("music".to_string());}}>{"Music"}</li>
            // <li onclick={move|_|{props.resume_click_callback.emit("resume".to_string());}}>{"Resume"}</li>

            // <div>{props.selected}</div>
            // <div>{props.last_clicked}</div>
            // <div>{&*current_page.to_string()}</div>
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
    pub home_click_callback: Callback<String>,
    pub resume_click_callback: Callback<String>,
    pub blog_click_callback: Callback<String>,
    pub music_click_callback: Callback<String>,
}

impl Props {
    pub fn new(
        selected: bool,
        last_clicked: Page,
        home_click_callback: Callback<String>,
        resume_click_callback: Callback<String>,
        blog_click_callback: Callback<String>,
        music_click_callback: Callback<String>,
    ) -> Self {
        Self {
            selected,
            last_clicked,
            home_click_callback,
            resume_click_callback,
            blog_click_callback,
            music_click_callback,
        }
    }
}

#[function_component(Music)]
fn music_page(props: &Props) -> Html {
    html! {
    <section>
        <ul>
            // <li onclick={move|_|{home_click_handler.emit("home".to_string());}}>{"Home"}</li>
            <div>{"listen to music"}</div>
        </ul>
    </section>
        }
}

#[function_component(CVPage)]
fn cv_page(props: &Props) -> Html {
    html! {
    <section>
        <ul>
            // <li onclick={move|_|{home_click_handler.emit("home".to_string());}}>{"Home"}</li>
            <div>{"listen to music"}</div>
        </ul>
    </section>
        }
}
