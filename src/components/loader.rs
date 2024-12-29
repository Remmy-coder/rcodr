use gloo_timers::callback::Timeout;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct LoaderProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or(true)]
    pub is_loading: bool,
    #[prop_or(2000)]
    pub timeout_ms: u32,
}

#[function_component]
pub fn Loader(props: &LoaderProps) -> Html {
    let loading = use_state(|| props.is_loading);
    let timeout_ms = props.timeout_ms;

    {
        let loading = loading.clone();
        use_effect_with(props.is_loading, move |_| {
            let loading = loading.clone();
            let timeout = Timeout::new(timeout_ms, move || {
                loading.set(false);
            });
            timeout.forget();
            || ()
        });
    }

    if *loading {
        html! {
            <div
                class="fixed inset-0 flex items-center justify-center bg-black/50 backdrop-blur-sm z-50"
            >
                <div
                    class="animate-spin rounded-full h-16 w-16 border-t-4 border-b-4 border-green-400"
                />
            </div>
        }
    } else {
        html! { <>{ for props.children.iter() }</> }
    }
}
