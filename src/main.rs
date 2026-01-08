use components::{anime_fight::AnimeFight, hero::Hero, loader::Loader, mission_archive::MissionArchive, nav_bar::NavBar};
use yew::prelude::*;

mod components;

#[function_component]
fn App() -> Html {
    html! {
        <div class="min-h-screen bg-black text-gray-100">
            <Loader>
                <NavBar />
                <Hero />
                <MissionArchive />
                <AnimeFight />
            </Loader>
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
