use yew::{function_component, html, Html};

#[function_component]
pub fn NavBar() -> Html {
    html! {
        <nav class="fixed top-0 w-full bg-black backdrop-blur-sm z-50">
            <div class="max-w-6xl mx-auto px-4 py-3 flex justify-center gap-10 font-mono text-gray-400 tracking-widest">
                <a href="https://github.com/Remmy-coder" class="hover:text-emerald-400 transition-colors">{ "github" }</a>
                <a href="mailto:remmy.ro@gmail.com" class="hover:text-emerald-400 transition-colors">{ "mail" }</a>
                <a href="https://www.linkedin.com/in/remmy-omeje-5a519017a/" class="hover:text-emerald-400 transition-colors">{ "linkedin" }</a>
                <a href="https://x.com/laurentthegoat" class="hover:text-emerald-400 transition-colors">{ "x" }</a>
            </div>
        </nav>
    }
}

