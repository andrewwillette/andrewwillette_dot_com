use super::soundcloud_iframe::SoundCloudIframe;
use yew::prelude::*;
pub struct Music;

const SOUNDCLOUD_URLS: &'static [&'static str] = &[
    // swallowtail jig
    "https://w.soundcloud.com/player/?url=https%3A//api.soundcloud.com/tracks/1511471356&color=%23b0a472&auto_play=false&hide_related=true&show_comments=false&show_user=true&show_reposts=false&show_teaser=false&visual=true",
    // sherry
    "https://w.soundcloud.com/player/?url=https%3A//api.soundcloud.com/tracks/1412600335&color=%233799bb&auto_play=false&hide_related=true&show_comments=false&show_user=true&show_reposts=false&show_teaser=false&visual=true",
    // raggedy ann
    "https://w.soundcloud.com/player/?url=https%3A//api.soundcloud.com/tracks/1353192850&color=%23e25862&auto_play=false&hide_related=true&show_comments=false&show_user=true&show_reposts=false&show_teaser=false&visual=true",
    // leather britches
    "https://w.soundcloud.com/player/?url=https%3A//api.soundcloud.com/tracks/1340417752&color=%237c643d&auto_play=false&hide_related=true&show_comments=false&show_user=true&show_reposts=false&show_teaser=false&visual=true",
    // carrol county blues
    "https://w.soundcloud.com/player/?url=https%3A//api.soundcloud.com/tracks/1299581302&color=%23e25862&auto_play=false&hide_related=true&show_comments=false&show_user=true&show_reposts=false&show_teaser=false&visual=true",
    // flop eared mule
    "https://w.soundcloud.com/player/?url=https%3A//api.soundcloud.com/tracks/1279484290&color=%2356bb37&auto_play=false&hide_related=true&show_comments=false&show_user=true&show_reposts=false&show_teaser=false&visual=true",
    // dixie
    "https://w.soundcloud.com/player/?url=https%3A//api.soundcloud.com/tracks/1279435306&color=%23e193db&auto_play=false&hide_related=true&show_comments=false&show_user=true&show_reposts=false&show_teaser=false&visual=true",
    // polecat blues
    "https://w.soundcloud.com/player/?url=https%3A//api.soundcloud.com/tracks/1244352139&color=%23f3143a&auto_play=false&hide_related=true&show_comments=false&show_user=true&show_reposts=false&show_teaser=false&visual=true",
    // wednesday night waltz
    "https://w.soundcloud.com/player/?url=https%3A//api.soundcloud.com/tracks/1241630236&color=%23e6d5d5&auto_play=false&hide_related=true&show_comments=false&show_user=true&show_reposts=false&show_teaser=false&visual=true",
    // down the road somewhere
    "https://w.soundcloud.com/player/?url=https%3A//api.soundcloud.com/tracks/1206283588&color=%235a1c1c&auto_play=false&hide_related=true&show_comments=false&show_user=true&show_reposts=false&show_teaser=false&visual=true",
    // benton's dream
    "https://w.soundcloud.com/player/?url=https%3A//api.soundcloud.com/tracks/1202201206&color=%23ccd7c8&auto_play=false&hide_related=true&show_comments=false&show_user=true&show_reposts=false&show_teaser=false&visual=true",
    // sugar in the gourd
    "https://w.soundcloud.com/player/?url=https%3A//api.soundcloud.com/tracks/1177441174&color=%23c224c3&auto_play=false&hide_related=true&show_comments=false&show_user=true&show_reposts=false&show_teaser=false&visual=true",
    // walking in my sleep
    "https://w.soundcloud.com/player/?url=https%3A//api.soundcloud.com/tracks/1144703368&color=%23b0a472&auto_play=false&hide_related=true&show_comments=false&show_user=true&show_reposts=false&show_teaser=false&visual=true",
];
impl Component for Music {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <section class="music-page">
                <ul>
                { for SOUNDCLOUD_URLS.iter().map(|url| {
                    html! {
                        <SoundCloudIframe url={url.to_string()} />
                    }
                })}
                </ul>
            </section>
        }
    }
}
