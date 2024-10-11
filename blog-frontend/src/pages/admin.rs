use crate::components::header::Header;
use crate::components::menu::Menu;
use crate::components::post_list_admin::PostListAdmin;
use yew::prelude::*;

#[function_component(Admin)]
pub fn admin() -> Html {
    let loading = html! {
        <img src="/rust-logo.png" alt="loading" />
    };

    html! {

        <div class="container">
                <div class="row">
                    <div class="col-sm-auto">
                        <Menu/>
                            <Header />
                                <Suspense fallback={loading.clone()}>
                                <PostListAdmin />
                                 </Suspense>

                        </div>
                    </div>
         </div>

    }
}
