use yew::{function_component, html, Html};

#[function_component]
pub fn Hero() -> Html {
    html! {
        <section
            class="min-h-screen flex justify-center items-center bg-black text-green-400 bg-cover bg-center bg-fixed"
        >
            <div
                class="max-w-4xl w-full sm:h-[400px] p-8 border border-green-400 rounded-xl bg-black/80 backdrop-blur-sm shadow-[0_0_10px_rgba(0,255,0,0.2)] font-mono flex flex-col sm:flex-row items-center sm:items-center justify-between gap-6"
            >
                <div class="flex flex-col gap-4">
                    <h1 class="text-3xl sm:text-4xl tracking-wider">
                        <span class="text-red-700">{ "RCODR" }</span>
                        <span class="ml-1 animate-blink">{"_"}</span>
                    </h1>
                    <div class="text-sm text-gray-500 animate-pulse">
                        { "[ system@omeje_remmy ] :: uptime 5y 142d | last commit: rustacean@github" }
                    </div>
                </div>

                <div class="flex-shrink-0">
                    <img
                        src="/rcodr/static/avatar.png"
                        alt="Omeje Remmy"
                        class="w-48 h-38 sm:w-60 sm:h-40 rounded-full border-2 border-green-400 shadow-md"
                    />
                </div>
            </div>
        </section>
    }
}
