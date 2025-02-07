use crate::{
    api::api_register_user,
    store::{set_feedback, set_loading, set_show_alert, Store},
};

use super::input::Input;

use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component]
pub fn LoginForm() -> Html {
    let (store, dispatch) = use_store::<Store>();
    let loading = &store.loading;
    let username = use_state(String::new);
    let login = use_state(String::new);
    let password = use_state(String::new);

    let text_password_input_ref = use_node_ref();
    let text_username_input_ref = use_node_ref();
    let text_login_input_ref = use_node_ref();

    let handle_input_username: Callback<String> = {
        let username = username.clone();
        Callback::from(move |username_text: String| {
            let value: String = username_text.clone();
            username.set(value);
        })
    };

    let handle_input_password: Callback<String> = {
        let password: UseStateHandle<String> = password.clone();
        Callback::from(move |password_text: String| {
            let value: String = password_text.clone();
            password.set(value);
        })
    };

    let handle_input_login: Callback<String> = {
        let login: UseStateHandle<String> = login.clone();
        Callback::from(move |password_text: String| {
            let value: String = password_text.clone();
            login.set(value);
        })
    };

    let on_submit: Callback<SubmitEvent> = {
        let cloned_dispatch: Dispatch<Store> = dispatch.clone();
        let cloned_password_input_ref = text_password_input_ref.clone();
        let cloned_username_input_ref = text_username_input_ref.clone();
        let cloned_login_input_ref = text_login_input_ref.clone();

        let cloned_username = username.clone();
        let cloned_password = password.clone();
        let cloned_login = login.clone();

        Callback::from(move |event: SubmitEvent| {
            let username_input_ref = cloned_password_input_ref.clone();
            let password_input_ref = cloned_username_input_ref.clone();
            let login_input_ref = cloned_login_input_ref.clone();

            let password: UseStateHandle<String> = cloned_password.clone();
            let username: UseStateHandle<String> = cloned_username.clone();
            let dispatch = cloned_dispatch.clone();
            let login = cloned_login.clone();

            event.prevent_default();
            set_loading(true, dispatch.clone());

            let new_user_data = serde_json::json!({
                "name": username.to_string(),
                "password": password.to_string(),
                "login": login.to_string()
            });

            spawn_local(async move {
                set_loading(true, dispatch.clone());
                let username_input = username_input_ref.cast::<HtmlInputElement>().unwrap();
                let password_input = password_input_ref.cast::<HtmlInputElement>().unwrap();
                let login_input = login_input_ref.cast::<HtmlInputElement>().unwrap();

                username_input.set_value("");
                password_input.set_value("");
                login_input.set_value("");

                password.set(String::new());
                username.set(String::new());
                login.set(String::new());

                let response: Result<(), String> =
                    api_register_user(new_user_data.to_string().as_str()).await;

                match response {
                    Ok(_) => {
                        set_loading(false, dispatch.clone());
                        set_show_alert("Usuario Creado".to_string(), dispatch.clone());
                    }
                    Err(e) => {
                        set_loading(false, dispatch.clone());
                        set_show_alert(e.to_string(), dispatch);
                    }
                }
            });
        })
    };

    html! {
        <div class="bg-white mx-auto max-w-lg text-gray-700 rounded-lg p-8 my-5 relative">
            <header class="max-w-md mx-auto">
                <h2 class="text-center text-2xl font-bold">{"Registrate en una aplicación creada completamente en Rust!"}</h2>
            </header>
            <form class="mt-8 flex flex-col gap-2" onsubmit={on_submit}>
                <Input
                    input_ref={text_username_input_ref}
                    on_input={handle_input_username}
                    label="Nombre Completo"
                />
                <Input
                    input_ref={text_login_input_ref}
                    on_input={handle_input_login}
                    label="Nombre de usuario"
                />
                <Input
                    input_ref={text_password_input_ref}
                    on_input={handle_input_password}
                    label="Contraseña"
                    input_type="password"
                />
                <button
                    type="submit"
                    class={format!(
                        "border-0 rounded-md mt-6 h-12 cursor-pointer hover:bg-indigo-500 {}",
                        if *loading { "bg-[#ccc] text-gray-800"} else {"bg-indigo-600 text-white"}
                    )}
                >
                    {"Guardar"}
                </button>
            </form>
        </div>
    }
}
