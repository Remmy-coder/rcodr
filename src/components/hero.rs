use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct ProjectCardProps {
    pub name: String,
    pub description: String,
    pub tags: Vec<String>,
    pub link: String,
}

#[function_component]
pub fn ProjectCard(props: &ProjectCardProps) -> Html {
    html! {
        <a
            href={props.link.clone()}
            target="_blank"
            rel="noopener noreferrer"
            class="flex flex-col justify-between bg-black rounded-lg p-6 border border-gray-700 hover:bg-transparent transition duration-300"
        >
            <h3 class="text-xl text-gray-400 font-bold mb-2">{ &props.name }</h3>
            <p class="text-gray-400 mb-4">{ &props.description }</p>
            <div class="flex flex-wrap gap-2 mb-4">
                { for props.tags.iter().map(|tag| html! {
                    <span
                        class="px-2 py-1 bg-emerald-400/10 text-emerald-400 rounded text-sm"
                    >
                        { tag }
                    </span>
                }) }
            </div>
        </a>
    }
}

#[function_component]
pub fn Hero() -> Html {
    let projects = vec![
    ProjectCardProps {
        name: "In-Memory Cache".to_string(),
        description: "A versatile in-memory cache implementation with support for TTL (time-to-live) and size limits.".to_string(),
        tags: vec![
            "Rust".to_string(),
            "Async Rust".to_string(),
            "Caching".to_string(),
            "Data-Structures".to_string(),
        ],
        link: "https://github.com/Remmy-coder/rcodr.cache".to_string(),
    },
    ProjectCardProps {
        name: "Cadmus_RC".to_string(),
        description: "A comprehensive command-line tool for testing API endpoints, supporting various HTTP methods, token management, and response parsing.".to_string(),
        tags: vec![
            "Bash".to_string(),
            "CLI".to_string(),
        ],
        link: "https://github.com/Remmy-coder/cadmus_rc".to_string(),
    },
    ProjectCardProps {
        name: "[WIP] Native Product Tracker".to_string(),
        description: "A native desktop tool for tracking product batches with cryptographic authentication, written in Rust and built with the Tauri framework.".to_string(),
        tags: vec![
            "TypeScript".to_string(), 
            "Rust".to_string(), 
            "Diesel ORM".to_string(), 
            "Postgres".to_string(), 
            "Docker".to_string(), 
            "NextJs".to_string(), 
            "X-State".to_string(),
            "Shadcn-Ui".to_string(),
            "Recharts".to_string(),
        ],
        link: "https://github.com/Remmy-coder/native-product-tracker".to_string(),
    },
    ProjectCardProps {
        name: "Neovim NvChad Config".to_string(),
        description: "My custom Neovim configuration using NvChad, featuring Lazy.nvim, Mason, Nvim-Tree, Conform, DAP, Dbee, and Treesitter.".to_string(),
        tags: vec![
            "Nvim".to_string(), 
            "Vim".to_string(), 
            "Lua".to_string(),
        ],
        link: "https://github.com/Remmy-coder/neovim-config-nvchad".to_string(),
    },
    ProjectCardProps {
        name: "[WIP] Load Balancer".to_string(),
        description: "A load balancer implementation in Rust using the round-robin algorithm.".to_string(),
        tags: vec![
            "Rust".to_string(), 
        ],
        link: "https://github.com/Remmy-coder/load-balancer".to_string(),
    },
    ProjectCardProps {
        name: "[WIP] The Reach".to_string(),
        description: "A password manager command-line tool written in Rust.".to_string(),
        tags: vec![
            "Rust".to_string(), 
        ],
        link: "https://github.com/Remmy-coder/the_reach".to_string(),
    },
];

    html! {
        <>
            <section
                class="min-h-screen flex justify-center items-center bg-black text-green-400 bg-cover bg-center bg-fixed"
                style="background-image: url('static/herobg.png');"
            >
                <div
                    class="min-h-screen rounded-lg p-8 sm:p-12 border border-green-400  mt-14 shadow-[0_0_15px_rgba(0,255,0,0.2)] bg-black/70 backdrop-blur-sm"
                >
                <div class="h-2/4 bg-black text-gray-400 font-mono p-6 border-2 border-gray-600 rounded-md shadow-md shadow-gray-900">
                    <h1 class="text-left text-3xl sm:text-4xl font-bold mb-8 tracking-wider">
                        { "Hi, I'm " }
                        <span class="text-red-700">{ "[ Omeje Remmy ]" }</span>
                        <span class="ml-1 animate-blink">{ "_" }</span>
                    </h1>

                    <p class="text-lg sm:text-xl mb-4">{ "> Software engineer sometimes sword immortal" }</p>
                    <p class="text-lg sm:text-xl mb-4">{ "> Innovative senior engineer with 5+ years of experience delivering scalable software solutions." }</p>
                    <p class="text-lg sm:text-xl mb-4">{ "> Passionate about low-level and functional programming with professional experience in TypeScript, Rust, and Lua." }</p>
                    <p class="text-lg sm:text-xl mb-4">{ "> Arch Linux evangelist and Neovim preacher." }</p>
                    <p class="text-lg sm:text-xl mb-6">
                        { "> This entire website is written in " }
                        <span class="font-bold text-green-200">{ "Rust." }</span>
                    </p>

                    <div class="mt-6">
                        <a
                            href="static/omeje_cv.pdf"
                            download="omeje_remmy_cv.pdf"
                            class="inline-block px-6 py-3 border-2 border-red-700 font-bold uppercase tracking-widest rounded hover:bg-green-900 hover:text-green-100 transition"
                        >
                            <i class="mr-2 fa-solid fa-download"></i>
                            { "download cv" }
                            <i class="ml-2 fa-solid fa-file-pdf"></i>
                        </a>
                    </div>
                </div>
                    <div class="mt-20">
                        <h2 class="text-3xl font-bold mb-8 text-gray-400">
                            { "[ Projects & Editor Setup ]" }
                        </h2>
                        <div class="grid grid-cols-1 md:grid-cols-2 gap-6 mb-8">
                            { for projects.iter().map(|project| html! {
                                <ProjectCard
                                    name={project.name.clone()}
                                    description={project.description.clone()}
                                    tags={project.tags.clone()}
                                    link={project.link.clone()}
                                />
                            }) }
                        </div>
                        <span class="text-gray-400 text-sm">
                            { "Some projects cannot be displayed here due to NDA restrictions. " }
                            <a
                                href="https://github.com/Remmy-coder"
                                target="_blank"
                                rel="noopener noreferrer"
                                class="text-green-400 underline hover:text-green-500"
                            >
                                { "Visit my GitHub repository" }
                            </a>
                            { " for more!" }
                        </span>
                    </div>
                </div>
            </section>
        </>
    }
}
