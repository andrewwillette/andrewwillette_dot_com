use yew::prelude::*;
pub struct Home;

const HOME_BIO: &str =
    " Hi! My name is Andrew Willette. I am a software developer based in Madison, Wisconsin.";

const HOME_BIO_CONT: &str = "I like playing violin and host some recordings on my site here.";
// I like playing violin and host some recordings on my site here.
impl Component for Home {
    type Message = ();
    type Properties = ();
    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="tile is-ancestor is-vertical">
                <div class="tile is-child hero">
                    <div class="home-container pb-0">
                        <img class="center" src="https://andrewwillette.s3.us-east-2.amazonaws.com/newdir/website_photo_2.jpeg" alt="Homepage Image" />
                        <p class="center">{ HOME_BIO }</p>
                        <p class="center">{ HOME_BIO_CONT }</p>
                    </div>
                </div>
            </div>
        }
    }
}
