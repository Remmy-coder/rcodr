use gloo_timers::callback::Timeout;
use web_sys::HtmlVideoElement;
use yew::{function_component, html, use_effect_with, use_node_ref, Html};

#[function_component]
pub fn AnimeFight() -> Html {
    let video_ref = use_node_ref();

    {
        let video_ref = video_ref.clone();
        use_effect_with((), move |_| {
            let video_ref_clone = video_ref.clone();
            Timeout::new(100, move || {
                if let Some(video) = video_ref_clone.cast::<HtmlVideoElement>() {
                    let _ = video.play();
                }
            })
            .forget();
            || ()
        });
    }
    html! {
        <section class="min-h-screen bg-black text-gray-100 py-20 px-4">
            <div class="max-w-6xl mx-auto">
                <div class="mb-12 text-center">
                    <div class="font-mono text-green-400 text-sm mb-4">
                        <p class="animate-pulse">{ "> neural_interface --playback --combat_data" }</p>
                        <p class="text-gray-600 mt-1">{ "[ ACCESSING PEAK ARCHIVE... ]" }</p>
                    </div>
                    <h2 class="text-3xl font-mono font-bold text-red-700 mb-2 tracking-wider">
                        { "╔═══════════════════════════════════════════╗" }
                    </h2>
                    <h2 class="text-3xl font-mono font-bold text-red-700 mb-2 tracking-wider">
                        { "║  PEAK SIMULATION :: NEURAL PLAYBACK   ║" }
                    </h2>
                    <h2 class="text-3xl font-mono font-bold text-red-700 mb-4 tracking-wider">
                        { "╚═══════════════════════════════════════════╝" }
                    </h2>
                    <p class="text-gray-500 font-mono text-xs">
                        { "[ STATUS: LOADING ] [ SOURCE: CLASSIFIED ] [ QUALITY: HD ]" }
                    </p>
                </div>

                <div class="border border-[#262626] bg-[#0a0a0a] p-6 rounded-lg shadow-[0_0_20px_rgba(0,255,0,0.1)]">
                    <div class="font-mono text-xs text-gray-500 mb-4">
                        <p>{ "> video_player --file combat_archive.mp4 --loop --hd" }</p>
                    </div>
                    
                    <div class="relative aspect-video bg-[#1a1a1a] border border-[#333] rounded overflow-hidden">
                        <video
                            ref={video_ref.clone()}
                            class="w-full h-full object-cover"
                            controls={true}
                            autoplay={true}
                            loop={true}
                            muted={true}
                            playsinline={true}
                            preload="auto"
                        >
                            <source src="/static/combat_archive.mp4" type="video/mp4" />
                            { "Your browser does not support the video tag." }
                        </video>
                    </div>

                    <div class="mt-4 font-mono text-xs text-gray-600 space-y-1">
                        <p>{ "[ SOURCE: LOCAL ] [ FILE: combat_archive.mp4 ] [ QUALITY: HD ]" }</p>
                        <p>{ "[ Add your video file to /static/combat_archive.mp4 ]" }</p>
                        <p class="text-red-500">{ "⚠ WARNING: COMBAT DATA MAY CONTAIN DISTURBING CONTENT" }</p>
                    </div>
                </div>

                <div class="mt-8 text-center">
                    <p class="text-gray-600 font-mono text-xs animate-pulse">
                        { "[ NEURAL INTERFACE ACTIVE ] [ PLAYBACK READY ]" }
                    </p>
                </div>
            </div>
        </section>
    }
}

