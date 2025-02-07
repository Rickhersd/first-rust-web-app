use yew::prelude::*;

#[derive(Debug, PartialEq, Properties)]
pub struct Props {
    #[prop_or(None)]
    pub label: Option<String>,

    #[prop_or(None)]
    pub placeholder: Option<String>,

    pub on_input: Callback<String>,

    #[prop_or(None)]
    pub input_type: Option<String>,

    pub input_ref: NodeRef,
}

#[function_component]
pub fn Input(props: &Props) -> Html {
    let handle_input: Callback<InputEvent> = {
        let on_input: Callback<String> = props.on_input.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement =
                e.target_unchecked_into::<web_sys::HtmlInputElement>();
            let value: String = input.value();
            on_input.emit(value);
        })
    };

    html! {
        <div
            class="flex flex-col gap-1.5"
        >
            { props.label.as_ref().map(|label| html! {
                <label>{ label }</label> })
            }
            <input
                type={props.input_type.clone()}
                ref={props.input_ref.clone()}
                oninput={handle_input}
                class="border px-2 py-2 border-black rounded-md"
                placeholder={props.placeholder.clone()}
            />
        </div>

    }
}
