mod api;
mod components;
mod store;

use components::{
    alert::{AlertComponent, Props as AlertProps},
    feedback_form::FeedbackForm,
    feedback_list::FeedbackList,
    feedback_stats::FeedbackStats,
    login_form::LoginForm,
    navbar::Navbar,
    register_form::RegisterForm,
};
use store::Store;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

#[function_component]
fn Login() -> Html {
    let (store, _) = use_store::<Store>();
    let message: String = store.alert_input.alert_message.clone();
    let show_alert = store.alert_input.show_alert;
    let loading = &store.loading;

    let alert_props = AlertProps {
        message,
        delay_ms: 5000,
    };

    html! {
        <>

            <main class="mt-24">
                <LoginForm />
                // <FeedbackForm />
                // <FeedbackStats />
                // <FeedbackList />
            </main>
            if *loading {
                <div
                    class="fixed top-5 left-5 inline-block h-8 w-8 animate-spin rounded-full border-4 border-solid border-yellow-400 border-r-transparent align-[-0.125em] text-warning motion-reduce:animate-[spin_1.5s_linear_infinite]"
                    role="status">
                    <span
                    class="!absolute !-m-px !h-px !w-px !overflow-hidden !whitespace-nowrap !border-0 !p-0 ![clip:rect(0,0,0,0)]"
                    >{"Loading..."}</span
                >
                </div>
            }
        </>
    }
}

#[function_component]
fn App() -> Html {
    let (store, _) = use_store::<Store>();
    let message = store.alert_input.alert_message.clone();
    let show_alert = store.alert_input.show_alert;
    let loading = &store.loading;

    let alert_props = AlertProps {
        message,
        delay_ms: 5000,
    };

    html! {
        <>
        if show_alert {
                    <AlertComponent
                        message={alert_props.message}
                        delay_ms={alert_props.delay_ms}
                     />
                }
        <BrowserRouter>
            <Navbar />
            <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
        </BrowserRouter>
        </>
    }
}

#[function_component(Secure)]
fn secure() -> Html {
    let navigator = use_navigator().unwrap();

    html! {
        <div>
            <FeedbackList />
        </div>
    }
}

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,

    #[at("/login")]
    Login,

    #[at("/signup")]
    SignUp,

    #[at("/users")]
    Users,

    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <h1>{ "Home" }</h1> },
        Route::SignUp => html! {
            <RegisterForm />
        },
        Route::Users => html! {
            <Secure />
        },
        Route::Login => html! { <Login />},
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
