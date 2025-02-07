use yew::prelude::*;
use yewdux::prelude::*;

use crate::store::Store;

#[function_component]
pub fn FeedbackStats() -> Html {
    let (store, _) = use_store::<Store>();
    let count = store.feedbacks.len();

    html! {
        <div class="flex justify-between items-center mb-11">
            <h4 class="text-white">{count} {" "} {"Uuarios"}</h4>
        </div>
    }
}
