use yew::prelude::*;

pub struct SoundCloudIframe {
    url: String,
}

#[derive(Properties, Clone, PartialEq)]
pub struct SoundCloudIframeProps {
    pub url: String,
}

impl Component for SoundCloudIframe {
    type Message = ();
    type Properties = SoundCloudIframeProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            url: ctx.props().url.clone(),
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <iframe
                    width="100%"
                    height="300"
                    scrolling="no"
                    frameborder="no"
                    src={self.url.clone()}
                />
            </div>
        }
    }
}
