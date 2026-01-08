use yew::{function_component, html, Html};

#[derive(Clone)]
struct Skill {
    name: &'static str,
    level: u8, // 0-100
    category: &'static str,
    corrupted: bool,
}

#[function_component]
pub fn MissionArchive() -> Html {
    let skills = vec![
        Skill { name: "Rust", level: 80, category: "LANG", corrupted: false },
        Skill { name: "TypeScript", level: 95, category: "LANG", corrupted: false },
        Skill { name: "Bash", level: 90, category: "LANG", corrupted: false },
        Skill { name: "Java", level: 70, category: "LANG", corrupted: true },
        Skill { name: "Lua", level: 75, category: "LANG", corrupted: false },
        Skill { name: "Python", level: 85, category: "LANG", corrupted: true },
        Skill { name: "Go", level: 40, category: "LANG", corrupted: true },
        
        Skill { name: "Yew", level: 82, category: "FRMW", corrupted: false },
        Skill { name: "Axum", level: 78, category: "FRMW", corrupted: false },
        Skill { name: "React", level: 95, category: "FRMW", corrupted: false },
        Skill { name: "Next.js", level: 95, category: "FRMW", corrupted: false },
        Skill { name: "NestJS", level: 95, category: "FRMW", corrupted: false },
        Skill { name: "Tauri", level: 79, category: "FRMW", corrupted: false },
        Skill { name: "Tanstack Start", level: 92, category: "FRMW", corrupted: false },
        Skill { name: "Diesel ORM", level: 85, category: "FRMW", corrupted: false },
        Skill { name: "TypeORM", level: 90, category: "FRMW", corrupted: false },
        
        Skill { name: "Linux", level: 95, category: "SYS", corrupted: false },
        Skill { name: "Docker", level: 88, category: "SYS", corrupted: false },
        Skill { name: "Kubernetes", level: 75, category: "SYS", corrupted: true },
        Skill { name: "GCP", level: 82, category: "SYS", corrupted: false },
        Skill { name: "AWS", level: 75, category: "SYS", corrupted: false },
        Skill { name: "Garage Object Storage", level: 80, category: "SYS", corrupted: false },
        Skill { name: "RabbitMQ", level: 85, category: "SYS", corrupted: false },
        Skill { name: "Coolify", level: 90, category: "SYS", corrupted: false },
        
        Skill { name: "PostgreSQL", level: 90, category: "DB", corrupted: false },
        Skill { name: "Oracle", level: 85, category: "DB", corrupted: false },
        Skill { name: "MySQL", level: 80, category: "DB", corrupted: false },
        Skill { name: "PocketBase", level: 90, category: "DB", corrupted: false },
        Skill { name: "Redis", level: 95, category: "DB", corrupted: false },
        
        Skill { name: "Git", level: 95, category: "TOOL", corrupted: false },
        Skill { name: "Neovim", level: 88, category: "TOOL", corrupted: false },
        Skill { name: "Vim", level: 90, category: "TOOL", corrupted: false },
        Skill { name: "WASM", level: 85, category: "TOOL", corrupted: false },
    ];

    let render_progress_bar = |level: u8| -> String {
        let filled = (level as f32 / 100.0 * 20.0) as usize;
        let empty = 20 - filled;
        format!("{}{}", "█".repeat(filled), "░".repeat(empty))
    };

    html! {
        <section class="min-h-screen bg-black text-gray-100 py-20 px-4 overflow-x-hidden">
            <div class="max-w-6xl mx-auto">
                <div class="mb-12 text-center">
                    <div class="font-mono text-green-400 text-sm mb-4">
                        <p class="animate-pulse">{ "> system_scan --neural-implant --verbose" }</p>
                        <p class="text-gray-600 mt-1">{ "[ SCANNING PROTOCOL MATRIX... ]" }</p>
                    </div>
                    <div class="hidden sm:block">
                        <h2 class="text-xl sm:text-3xl font-mono font-bold text-red-700 mb-2 tracking-wider overflow-x-auto">
                            { "╔═══════════════════════════════════════════╗" }
                        </h2>
                        <h2 class="text-xl sm:text-3xl font-mono font-bold text-red-700 mb-2 tracking-wider overflow-x-auto">
                            { "║  NEURAL IMPLANT :: PROTOCOL MATRIX v2.0  ║" }
                        </h2>
                        <h2 class="text-xl sm:text-3xl font-mono font-bold text-red-700 mb-4 tracking-wider overflow-x-auto">
                            { "╚═══════════════════════════════════════════╝" }
                        </h2>
                    </div>
                    <div class="sm:hidden">
                        <h2 class="text-2xl font-mono font-bold text-red-700 mb-4">
                            { "NEURAL IMPLANT v2.0" }
                        </h2>
                    </div>
                    <p class="text-gray-500 font-mono text-xs">
                        { "[ STATUS: ACTIVE ] [ CORRUPTION: 12% ] [ LAST UPDATE: NOW ]" }
                    </p>
                </div>

                <div class="space-y-6 font-mono text-sm">
                    { for skills.iter().map(|skill| {
                        let progress = render_progress_bar(skill.level);
                        let level_color = if skill.level >= 85 {
                            "text-green-400"
                        } else if skill.level >= 70 {
                            "text-yellow-400"
                        } else {
                            "text-orange-400"
                        };
                        
                        html! {
                            <div class="border border-[#262626] bg-[#0a0a0a] p-4 rounded hover:border-green-500/50 transition-all">
                                <div class="flex items-center justify-between mb-2">
                                    <div class="flex items-center gap-3">
                                        <span class="text-red-700 text-xs font-bold">
                                            { format!("[{}]", skill.category) }
                                        </span>
                                        <span class={format!("{} font-bold", if skill.corrupted { "text-red-500" } else { "text-green-400" })}>
                                            { if skill.corrupted {
                                                format!("{} [CORRUPTED]", skill.name)
                                            } else {
                                                skill.name.to_string()
                                            }}
                                        </span>
                                        { if skill.corrupted {
                                            html! {
                                                <span class="text-red-700 text-xs animate-pulse">{ "⚠" }</span>
                                            }
                                        } else {
                                            html! {}
                                        }}
                                    </div>
                                    <span class={format!("{} text-xs", level_color)}>
                                        { format!("{}%", skill.level) }
                                    </span>
                                </div>
                                <div class="flex items-center gap-2">
                                    <span class="text-green-400 text-xs">{ "[" }</span>
                                    <span class="text-green-400 font-bold text-xs">
                                        { progress }
                                    </span>
                                    <span class="text-green-400 text-xs">{ "]" }</span>
                                </div>
                            </div>
                        }
                    })}
                </div>

                <div class="mt-12 border border-[#262626] bg-[#0a0a0a] p-6 rounded">
                    <div class="font-mono text-xs space-y-2">
                        <p class="text-green-400">
                            { "> cat /proc/neural/status" }
                        </p>
                        <p class="text-gray-400 ml-4">
                            { "TOTAL_PROTOCOLS: 33" }
                        </p>
                        <p class="text-gray-400 ml-4">
                            { "ACTIVE_CONNECTIONS: 29" }
                        </p>
                        <p class="text-red-500 ml-4">
                            { "CORRUPTED_SECTORS: 4" }
                        </p>
                        <p class="text-green-400 ml-4">
                            { "SYSTEM_INTEGRITY: 88%" }
                        </p>
                        <p class="text-gray-600 mt-4">
                            { "[ END OF SCAN ] [ VERIFIED: TRUE ]" }
                        </p>
                    </div>
                </div>

                <div class="mt-8 text-center">
                    <p class="text-gray-600 font-mono text-xs animate-pulse">
                        { "███ WARNING: UNAUTHORIZED ACCESS DETECTED ███" }
                    </p>
                </div>
            </div>
        </section>
    }
}

