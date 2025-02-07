use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yewdux::prelude::*;

use crate::{
    api::api_delete_feedback,
    store::{delete_feedback, set_loading, set_show_alert, Store},
};
use common::Feedback;

#[derive(Debug, PartialEq, Properties)]
pub struct Props {
    pub feedback: Feedback,
}

fn confirm_delete(message: &str) -> bool {
    web_sys::Window::confirm_with_message(&web_sys::window().unwrap(), message).unwrap()
}

#[function_component]
pub fn FeedbackItem(props: &Props) -> Html {
    let (_, dispatch) = use_store::<Store>();

    // let on_delete = {
    //     let cloned_dispatch = dispatch.clone();
    //     let feedback_id = props.feedback.id.clone();
    //     Callback::from(move |_: MouseEvent| {
    //         let dispatch = cloned_dispatch.clone();
    //         let confirmed = confirm_delete("Do you really want to delete this item?");

    //         if confirmed {
    //             spawn_local(async move {
    //                 set_loading(true, dispatch.clone());
    //                 let response = api_delete_feedback(feedback_id.to_string().as_str()).await;
    //                 match response {
    //                     Ok(_) => {
    //                         set_loading(false, dispatch.clone());
    //                         set_show_alert(
    //                             "Feedback deleted successfully".to_string(),
    //                             dispatch.clone(),
    //                         );
    //                         // delete_feedback(feedback_id, dispatch);
    //                     }
    //                     Err(e) => {
    //                         set_loading(false, dispatch.clone());
    //                         set_show_alert(e.to_string(), dispatch);
    //                     }
    //                 }
    //             });
    //         }
    //     })
    // };

    html! {
        <div class="bg-white text-gray-700 rounded-lg px-4 py-3 my-5 relative">
            <p>
            {"ID: "}{props.feedback.id.clone()}
            </p>
            <p>
            {"Nombre: "}{props.feedback.name.clone()}
            </p>
            <p>
                {&props.feedback.role.clone()}
            </p>
        </div>
    }
}
