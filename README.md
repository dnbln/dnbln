# Hello

```rust
pub fn me() -> Programmer<'static> {
    let present = chrono::Utc::now().year() as u16;

    Programmer {
        about_me: AboutMe::new()
            .name("Dinu Blanovschi")
            .interest("Programming language implementation and tooling development")
            .interest("Game development")
            .links(|l| l.github("https://github.com/dnbln")),
        languages: vec![
            Language::new("Rust").level(Level::Average),
            Language::new("C").level(Level::Average),
            Language::new("C++").level(Level::Average),
            Language::new("Python").level(Level::Average),
            Language::new("Kotlin").level(Level::Average),
        ],
        projects: vec![
            Project::new("jmex")
                .repo("https://github.com/dnbln/jmex")
                .description(indoc!(
                    r"
                    A non-standard stream-based JSON parser and utility, similar to jq.
                    "
                )),
            Project::new("DiscordPanel")
                .repo("https://github.com/dnbln/DiscordPanel")
                .link("https://plugins.jetbrains.com/plugin/14918-discord-panel")
                .description(indoc!(
                    r"
                    Plugin that adds a small discord client tool window to IntelliJ.
                    "
                )),
        ],

        ..Default::default()
    }
}
```
