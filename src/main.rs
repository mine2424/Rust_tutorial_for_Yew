use reqwasm::http::Request;
use yew::prelude::*;

use crate::components::video_detail::VideoDetail;
use crate::components::video_list::VideoList;
use crate::domain::models::video::Video;

mod components;
mod domain;

// #[function_component(App)]で宣言された関数コンポーネントを定義する。
#[function_component(App)]
fn app() -> Html {
    // use_stateは、状態を保持するためのHookである。
    let videos = use_state(|| vec![]);
    {
        // cloneは、Hookの返り値をコピーするためのメソッドです。
        let videos = videos.clone();
        // use_effectは、Hookの返り値を変更するためのメソッド。
        // さらにこれは依存元が変更されたかどうか関係なく値を更新する。
        // ただ、認識さセルにはstructに**PartialEq**が必要だ。
        use_effect_with_deps(
            // moveクロージャーがいまいちよくわからん。
            move |_| {
                // state(videos)を変数にコピー。
                let videos = videos.clone();
                // rest apiを叩いて、データを取得する。
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_videos: Vec<Video> = Request::get("/tutorial/data.json")
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();
                    videos.set(fetched_videos);
                });
                || ()
            },
            (),
        );
    }

    let selected_video = use_state(|| None);

    let on_video_select = {
        let selected_video = selected_video.clone();
        // 何のためのコールバック？
        Callback::from(move |video: Video| selected_video.set(Some(video)))
    };

    let details = selected_video.as_ref().map(|video| {
        html! {
            <VideoDetail video={video.clone()} />
        }
    });

    html! {
        <>
            <h1>{ "RustConf Explorer" }</h1>
            <div>
                <h3>{"Videos to watch"}</h3>
                <VideoList videos={(*videos).clone()} on_click={on_video_select.clone()} />
            </div>
            { for details }
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
