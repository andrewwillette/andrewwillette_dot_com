use yew::prelude::*;
pub struct Resume;

impl Component for Resume {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
        <section>
            <ul>
                <div>{"personal resume"}</div>
            </ul>
        </section>
            }
    }
}
