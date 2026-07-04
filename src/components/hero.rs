use gloo_timers::callback::{Interval, Timeout};
use yew::{function_component, html, use_effect_with, use_state, Html, UseStateHandle};

const BOOT_SCRIPT: &str = "booting rcodr.sys...\nloading modules [rust, ts, wasm]...\nestablishing uplink... done\naccess granted_";

fn type_next(typed_len: UseStateHandle<usize>, booted: UseStateHandle<bool>, next: usize, total: usize) {
    Timeout::new(18, move || {
        typed_len.set(next);
        if next >= total {
            let booted = booted.clone();
            Timeout::new(500, move || booted.set(true)).forget();
        } else {
            type_next(typed_len, booted, next + 1, total);
        }
    })
    .forget();
}

#[function_component]
pub fn Hero() -> Html {
    let bg = use_state(|| "/static/bg.gif".to_string());
    let typed_len = use_state(|| 0usize);
    let booted = use_state(|| false);

    {
        let bg = bg.clone();
        use_effect_with((), move |_| {
            let choices = vec!["/static/bg.gif", "/static/gif.gif"];

            Interval::new(5000, move || {
                let idx =
                    js_sys::Math::floor(js_sys::Math::random() * choices.len() as f64) as usize;
                bg.set(choices[idx].to_string());
            })
            .forget();

            || ()
        });
    }

    {
        let typed_len = typed_len.clone();
        let booted = booted.clone();
        use_effect_with((), move |_| {
            let total = BOOT_SCRIPT.chars().count();
            type_next(typed_len, booted, 1, total);
            || ()
        });
    }

    if !*booted {
        let visible: String = BOOT_SCRIPT.chars().take(*typed_len).collect();
        return html! {
            <section class="min-h-screen flex items-center justify-center bg-black text-green-400 font-mono px-4">
                <pre class="text-sm sm:text-base whitespace-pre-wrap">
                    { visible }
                    <span class="animate-blink">{ "_" }</span>
                </pre>
            </section>
        };
    }

    html! {
        <section
            style={format!("background: url('{}') center/cover fixed no-repeat;", *bg)}
            class="min-h-screen flex justify-center items-center text-green-400"
        >
            <div
                class="animate-reveal max-w-4xl w-full sm:h-[200px] p-8 border border-[#262626] rounded-xl bg-[#0a0a0a] backdrop-blur-sm shadow-[0_0_10px_rgba(0,255,0,0.2)] font-mono flex flex-col sm:flex-row items-center sm:items-center justify-between gap-6"
            >
                <div class="flex flex-col gap-4">
                    <h1 class="text-3xl sm:text-4xl font-extrabold font-mono tracking-wider">
                        <span class="inline-block cursor-default text-red-700 hover:animate-glitch">{ "ＲＣＯＤＲ" }</span>
                        <span class="ml-1 animate-blink">{"_"}</span>
                    </h1>
                    <div class="text-sm text-gray-500 animate-pulse">
                        { "[ system@omeje_remmy ] :: uptime 6y | last commit: rustacean@github" }
                    </div>
                </div>

                <div class="flex-shrink-0">
                    <img
                        src="/static/avatar.png"
                        alt="Omeje Remmy"
                        class="w-48 h-38 sm:w-60 sm:h-38 rounded-full border-2 border-[#737373] shadow-md"
                    />
                </div>
            </div>
        </section>
    }
}
