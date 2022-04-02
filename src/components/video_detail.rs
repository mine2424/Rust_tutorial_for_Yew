use yew::prelude::*;

use crate::domain::models::video::Video;

#[derive(Clone, Properties, PartialEq)]
pub struct VideosDetailProps {
    pub video: Video,
}

#[function_component(VideoDetail)]
pub fn video_detail(VideosDetailProps { video }: &VideosDetailProps) -> Html {
    html! {
        <div>
            <h3>{ video.title.clone() }</h3>
            <img src="https://via.placeholder.com/640x360.png?text=Video+Player+Placeholder" alt="video thumbnail" />
        </div>
    }
}
