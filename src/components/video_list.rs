use yew::prelude::*;

use crate::domain::models::video::Video;

#[derive(Clone, Properties, PartialEq)]
pub struct VideoListProps {
    pub videos: Vec<Video>,
    pub on_click: Callback<Video>,
}

#[function_component(VideoList)]
// なぜ、&VideoListPropsは & が必要なのか？
pub fn video_list(VideoListProps { videos, on_click }: &VideoListProps) -> Html {
    videos
        .iter()
        .map(|video| {
            let on_video_select = {
                let on_click = on_click.clone();
                let video = video.clone();
                Callback::from(move |_| on_click.emit(video.clone()))
            };

            html! {
                <p onclick={on_video_select}>{format!("{}: {}", video.speaker, video.title)}</p>
            }
        })
        .collect()
}
