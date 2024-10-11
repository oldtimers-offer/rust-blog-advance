use crate::pages::post::single_post::SinglePost;

use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub post_id: i32,
}

#[function_component(PostPage)]
pub fn post_page(props: &Props) -> Html {
    let loading = html! {
        <img src="/rust-logo.png" alt="loading" />
    };

    html! {
        <div class="container">
                <div class="row">
                    <div class="col-sm-auto">
                        <Suspense fallback={loading.clone()}>
                            <SinglePost post_id= {props.post_id}/>
                        </Suspense>
                    </div>
                </div>
        </div>
    }
}
