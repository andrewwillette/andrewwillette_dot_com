use yew::prelude::*;
pub struct Music;

impl Component for Music {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
        <section>
            <ul>
                <div>{"listen to music"}</div>
            </ul>
        </section>
            }
    }
}
