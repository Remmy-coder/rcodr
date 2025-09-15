use gloo_timers::callback::Interval;
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
    let frame = use_state(|| 0);

    let timeout_ms = props.timeout_ms;

    {
        let loading = loading.clone();
        use_effect_with(props.is_loading, move |_| {
            let loading = loading.clone();
            let timeout = gloo_timers::callback::Timeout::new(timeout_ms, move || {
                loading.set(false);
            });
            timeout.forget();
            || ()
        });
    }

    {
        let frame = frame.clone();
        use_effect(move || {
            let interval = Interval::new(100, move || {
                frame.set((*frame + 1) % 4);
            });
            move || drop(interval)
        });
    }

    if *loading {
        let ascii_frames = ["R", "C", "O", "D", "R"];
        let current = ascii_frames[*frame as usize];

        html! {
            <div class="fixed inset-0 flex items-center justify-center bg-black/70 backdrop-blur-sm z-50">
                <pre class="text-red-700 text-4xl font-mono animate-pulse">
                    { current }
                </pre>
            </div>
        }
    } else {
        html! { <>{ for props.children.iter() }</> }
    }
}
