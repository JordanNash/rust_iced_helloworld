//! "Text" tab: live Markdown preview and a syntax-highlighted code editor.
//! Features used: `markdown`, `highlighter` (plus the built-in `text_editor`).

use iced::highlighter;
use iced::widget::{column, markdown, pick_list, row, scrollable, text, text_editor};
use iced::{Alignment, Element, Font, Length, Theme};

const MD_SAMPLE: &str = r#"# Markdown preview

Edit **this** text and the preview updates live.

- Bullet lists
- *Emphasis* and **strong**
- `inline code`

```rust
fn main() {
    println!("hello from a code block");
}
```

[The Rust language](https://www.rust-lang.org)
"#;

const CODE_SAMPLE: &str = r#"// Syntax highlighting comes from the `highlighter` feature.
fn fibonacci(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn main() {
    for i in 0..10 {
        println!("fib({i}) = {}", fibonacci(i));
    }
}
"#;

pub struct Editing {
    md_content: text_editor::Content,
    md_items: Vec<markdown::Item>,
    last_link: Option<String>,
    code_content: text_editor::Content,
    hl_theme: highlighter::Theme,
}

impl Default for Editing {
    fn default() -> Self {
        Editing {
            md_content: text_editor::Content::with_text(MD_SAMPLE),
            md_items: markdown::parse(MD_SAMPLE).collect(),
            last_link: None,
            code_content: text_editor::Content::with_text(CODE_SAMPLE),
            hl_theme: highlighter::Theme::SolarizedDark,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    MarkdownEdited(text_editor::Action),
    CodeEdited(text_editor::Action),
    LinkClicked(markdown::Url),
    HighlightThemeChanged(highlighter::Theme),
}

impl Editing {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::MarkdownEdited(action) => {
                let is_edit = action.is_edit();
                self.md_content.perform(action);
                if is_edit {
                    // Re-parse only when the text actually changed.
                    self.md_items = markdown::parse(&self.md_content.text()).collect();
                }
            }
            Message::CodeEdited(action) => self.code_content.perform(action),
            Message::LinkClicked(url) => {
                // macOS convenience: hand web links to the default browser.
                if matches!(url.scheme(), "http" | "https") {
                    let _ = std::process::Command::new("open").arg(url.as_str()).spawn();
                }
                self.last_link = Some(url.to_string());
            }
            Message::HighlightThemeChanged(theme) => self.hl_theme = theme,
        }
    }

    /// The parent passes the active theme so the Markdown colors match it.
    pub fn view(&self, theme: &Theme) -> Element<'_, Message> {
        let md_editor = text_editor(&self.md_content)
            .placeholder("Type some Markdown...")
            .on_action(Message::MarkdownEdited)
            .padding(10)
            .height(Length::Fixed(260.0));

        let preview = scrollable(
            markdown::view(
                &self.md_items,
                markdown::Settings::default(),
                markdown::Style::from_palette(theme.palette()),
            )
            .map(Message::LinkClicked),
        )
        .height(Length::Fixed(260.0));

        let code_editor = text_editor(&self.code_content)
            .on_action(Message::CodeEdited)
            .highlight("rs", self.hl_theme)
            .font(Font::MONOSPACE)
            .padding(10)
            .height(Length::Fixed(260.0));

        let link_status = match &self.last_link {
            Some(link) => format!("Last link clicked: {link}"),
            None => String::from("Click the link in the preview to open it."),
        };

        column![
            text("Text editing").size(28),
            text("Markdown: edit on the left, live preview on the right"),
            row![md_editor, preview].spacing(12),
            text(link_status),
            row![
                text("Code highlighting theme:"),
                pick_list(
                    highlighter::Theme::ALL,
                    Some(self.hl_theme),
                    Message::HighlightThemeChanged
                ),
            ]
            .spacing(10)
            .align_y(Alignment::Center),
            code_editor,
        ]
        .spacing(14)
        .into()
    }
}
