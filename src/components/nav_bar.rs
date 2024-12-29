use yew::{function_component, html, Html};

#[function_component]
pub fn NavBar() -> Html {
    html! {
        <nav class="fixed top-0 w-full bg-black backdrop-blur-sm border-b border-white z-50">
            <div class="max-w-6xl mx-auto px-4 py-3 flex justify-between items-center">
                <p class="text-2xl font-bold text-gray-700 tracking-widest">{ "rcodr." }</p>
                <div class="flex gap-6">
                    <a href="#about" class="hover:text-emerald-400 transition-colors">
                        <i class="fa-brands fa-github" aria-hidden="true" />
                    </a>
                    <a href="#projects" class="hover:text-emerald-400 transition-colors">
                        <i class="fa-solid fa-envelope" aria-hidden="true" />
                    </a>
                    <a href="#skills" class="hover:text-emerald-400 transition-colors">
                        <i class="fa-brands fa-linkedin" aria-hidden="true" />
                    </a>
                    <a href="#contact" class="hover:text-emerald-400 transition-colors">
                        <i class="fa-brands fa-x-twitter" aria-hidden="true"/>
                    </a>
                </div>
            </div>
        </nav>
    }
}
