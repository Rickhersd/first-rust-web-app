use crate::Route;
use gloo::timers::callback::Timeout;
use yew::prelude::*;

use crate::store::{set_hide_alert, Store};
use yew_router::prelude::*;

#[function_component]
pub fn Navbar() -> Html {
    let navigator = use_navigator().unwrap();
    let navigator1 = use_navigator().unwrap();
    let navigator2 = use_navigator().unwrap();
    let navigator3 = use_navigator().unwrap();

    let on_move_to_home = Callback::from(move |_| navigator.push(&Route::Home));
    let on_move_to_login = Callback::from(move |_| navigator1.push(&Route::Login));
    let on_move_to_signup = Callback::from(move |_| navigator2.push(&Route::SignUp));
    let on_move_to_users = Callback::from(move |_| navigator3.push(&Route::Users));

    html! {
    <div class={format!("bg-white w-full py-4")}>
        <p class="text-sm">
            <span class="mr-2 px-3 py-1 gap-4 flex flex-row rounded-full text-black font-extrabold">
                <button
                    class="cursor-pointer underline-2 hover:underline"
                    onclick={on_move_to_home}
                >
                    {"Home"}
                </button>
                <button
                    class="cursor-pointer underline-2 hover:underline"
                    onclick={on_move_to_login}
                >
                    {"Iniciar Sesión"}
                </button>
                <button
                    class="cursor-pointer underline-2 hover:underline"
                    onclick={on_move_to_signup}>
                    {"Registrarse"}
                </button>
                <button
                    class="cursor-pointer underline-2 hover:underline"
                    onclick={on_move_to_users}>
                    {"Usuarios"}
                </button>
            </span>
        </p>
    </div>
    }
}
