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
