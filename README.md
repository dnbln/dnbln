# Hello

```rust
fn me() -> Programmer<'static> {
    let present = chrono::Utc::now().year();

    Programmer {
        about_me: AboutMe::new()
            .name("Dinu Blanovschi")
            .email("dinu.blanovschi@criptext.com")
            .age(18)
            .years_since_started_programming(5)
            .interest("Programming language implementation and tooling development")
            .interest("Game development")
            .links(|l|
                l.github("https://github.com/dnbln")
            )
            .education(|e| e.high_school(
                HighSchool::new(r#""Tudor Vianu" National High School of Computer Science"#)
                    .from("Bucharest, Romania")
                    .period(2019..=present)
            )),
        languages: vec![
            Language::new("Rust").level(Level::Average),
            Language::new("C").level(Level::Average),
            Language::new("C++").level(Level::Average),
            Language::new("Python").level(Level::Average),
            Language::new("Java").level(Level::Average),
            Language::new("Kotlin").level(Level::Average),
            Language::new("Golang").level(Level::Average),
            Language::new("JavaScript").level(Level::Average),
            Language::new("TypeScript").level(Level::Average),

            // own languages
            Language::new("leafbuild").level(Level::Expert),
        ],
        technologies: vec![
            TechnologyCategory::new("Web")
                .subcategory(|s| s.name("Frontend")
                                .technology("ReactJS"))
                .subcategory(|s| s.name("Backend")
                                .in_language("Rust", |s| s.technology("actix")
                                                        .technology("actix-web")
                                                        .technology("rocket"))
                                .in_language("Golang", |s| s.technology("gin")
                                                        .technology("raw net/http"))
                                .technology("nginx")),
            TechnologyCategory::new("Graphics")
                .technology("OpenGL")
                .technology(Technology::new("Vulkan").level(Level::Beginner)),
            TechnologyCategory::new("Other")
                .technology("IntelliJ platform"),
        ],
        projects: vec![
            Project::new("robespierre")
                .repo("https://github.com/dnbln/robespierre")
                .kind(Kind::ApiWrapper)
                .language("Rust")
                .description("API wrapper for revolt.chat"),
            Project::new("thor*")
                .repo("https://github.com/dnbln/thor")
                .repo("https://github.com/dnbln/thorc")
                .repo("https://github.com/dnbln/thorc-index")
                .repo("https://github.com/dnbln/templates")
                .kind(Kind::Utils)
                .language("Nix")
                .language("Rust")
                .description(indoc!(r"
                    Useful nix expressions for programming projects,
                    and a tool to create a project from given templates
                ")),
            Project::new("jmex")
                .repo("https://github.com/dnbln/jmex")
                .kind(Kind::Utils)
                .language("Rust")
                .description(indoc!(r"
                    A non-standard stream-based JSON parser and utility, similar to jq.
                ")),
            Project::new("DiscordPanel")
                .repo("https://github.com/dnbln/DiscordPanel")
                .link("https://plugins.jetbrains.com/plugin/14918-discord-panel")
                .kind(Kind::Utils)
                .kind(Kind::IntelliJPlugin)
                .language("Kotlin")
                .description(indoc!(r#"
                    Plugin that adds a small discord client tool window to IntelliJ.
                "#)),
        ],
        old_projects: vec![
            Project::new("leafbuild")
                .repo("https://github.com/leafbuild/leafbuild")
                .kind(Kind::Utils)
                .language("Rust")
                .language("C")
                .language("C++")
                .language("leafbuild")
                .description("Build system for C/C++ projects, similar to cmake."),
        ],
        projects_done_for_fun: vec![
            Project::new("ascii-image")
                .repo("https://github.com/projects-graveyard/ascii-image")
                .kind(Kind::Fun)
                .language("Rust")
                .description(indoc!(r#"
                    Program to turn images into "ascii" art, with braille characters.
                "#)),
            Project::new("lalrdoc")
                .repo("https://github.com/projects-graveyard/twdiff")
                .kind(Kind::Fun)
                .language("C")
                .description(indoc!(r#"
                    Program to compare C/C++ source files, giving them
                    a "similarity" score.
                "#)),
            Project::new("PLC")
                .repo("https://github.com/projects-graveyard/PseudoLangCompiler")
                .repo("https://github.com/projects-graveyard/Pseudo_LANG_COMPILER_C")
                .kind(Kind::Fun)
                .language("Java")
                .language("C")
                .description(indoc!(r#"
                    A Java and C implementation of a compiler for
                    "standard" pseudocode, taught in 9th grade.
                "#)),
            Project::new("NS")
                .repo("https://github.com/projects-graveyard/NatureSimulator")
                .repo("https://github.com/projects-graveyard/NS_CPP")
                .kind(Kind::Fun)
                .language("Java")
                .language("C++")
                .description(indoc!(r#"
                    A 3D game-ish project made for fun, implemented in
                    both Java and C++.
                "#))
        ],
    }
}
```
