use gloo_timers::callback::Interval;
use yew::{function_component, html, use_effect_with, use_state, Html};

#[function_component]
pub fn Hero() -> Html {
    let bg = use_state(|| "/rcodr/static/bg.gif".to_string());

    {
        let bg = bg.clone();
        use_effect_with((), move |_| {
            let choices = vec!["/rcodr/static/bg.gif", "/rcodr/static/gif.gif"];

            Interval::new(5000, move || {
                let idx =
                    js_sys::Math::floor(js_sys::Math::random() * choices.len() as f64) as usize;
                bg.set(choices[idx].to_string());
            })
            .forget();

            || ()
        });
    }

    html! {
        <section
            style={format!("background: url('{}') center/cover fixed no-repeat;", *bg)}
            class="min-h-screen flex justify-center items-center text-green-400"
        >
            <div
                class="max-w-4xl w-full sm:h-[200px] p-8 border border-[#262626] rounded-xl bg-[#0a0a0a] backdrop-blur-sm shadow-[0_0_10px_rgba(0,255,0,0.2)] font-mono flex flex-col sm:flex-row items-center sm:items-center justify-between gap-6"
            >
                <div class="flex flex-col gap-4">
                    <h1 class="text-3xl sm:text-4xl font-extrabold font-mono tracking-wider">
                        <span class="text-red-700">{ "ＲＣＯＤＲ" }</span>
                        <span class="ml-1 animate-blink">{"_"}</span>
                    </h1>
                    <div class="text-sm text-gray-500 animate-pulse">
                        { "[ system@omeje_remmy ] :: uptime 5y 420d | last commit: rustacean@github" }
                    </div>
                </div>

                <div class="flex-shrink-0">
                    <img
                        src="/rcodr/static/avatar.png"
                        alt="Omeje Remmy"
                        class="w-48 h-38 sm:w-60 sm:h-38 rounded-full border-2 border-[#737373] shadow-md"
                    />
                </div>
            </div>
        </section>
    }
}
