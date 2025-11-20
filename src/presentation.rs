pub mod log {
    use colored::*;
    use std::fmt::Display;
    use std::sync::atomic::{AtomicBool, Ordering};

    /// Global flag to enable verbose logging
    static VERBOSE_MODE: AtomicBool = AtomicBool::new(false);

    /// Enable or disable verbose mode
    pub fn set_verbose(enabled: bool) {
        VERBOSE_MODE.store(enabled, Ordering::Relaxed);
    }

    /// Check if verbose mode is enabled
    pub fn is_verbose() -> bool {
        VERBOSE_MODE.load(Ordering::Relaxed)
    }

    /// Toggles coloring based on environment.
    /// For instance, colors do not work for `cmd`on Windows.
    pub fn check_support_for_colors() {
        let term = term::stdout().unwrap();
        if !term.supports_color() {
            colored::control::set_override(false);
        }
    }

    /// Print a header. Includes a preliminary newline.
    pub fn header<S: AsRef<str>>(text: S) {
        println!(
            "\n{open_brace} {text} {close_brace}",
            open_brace = "[".green(),
            text = text.as_ref(),
            close_brace = "]".green()
        );
    }

    /// Print the text without any frills.
    pub fn basic<S: AsRef<str>>(text: S) {
        println!("{}", text.as_ref());
    }

    /// Print a step.
    pub fn step<A: Display, B: Display>(process: A, text: B) {
        println!(
            "{open_paren} {process} {close_paren} {text}",
            open_paren = "(".purple(),
            process = process,
            close_paren = ")".purple(),
            text = text
        )
    }

    /// Print a success message.
    pub fn success<S: AsRef<str>>(text: S) {
        println!("\n\t[ Success ]\n\t{}\n", text.as_ref().bright_green());
    }

    /// Print an error.
    pub fn error<S: AsRef<str>>(text: S) {
        println!("\n\t[ Error ]\n\t{}\n", text.as_ref().red());
    }

    /// Print a verbose message (only when verbose mode is enabled).
    pub fn verbose<S: AsRef<str>>(text: S) {
        if is_verbose() {
            println!(
                "{open_bracket} {verbose} {close_bracket} {text}",
                open_bracket = "[".bright_black(),
                verbose = "verbose".bright_black(),
                close_bracket = "]".bright_black(),
                text = text.as_ref().bright_black()
            );
        }
    }

    /// Print a verbose message with a category (only when verbose mode is enabled).
    pub fn verbose_with_category<A: Display, B: Display>(category: A, text: B) {
        if is_verbose() {
            println!(
                "{open_bracket} {verbose} {close_bracket} {open_paren}{category}{close_paren} {text}",
                open_bracket = "[".bright_black(),
                verbose = "verbose".bright_black(),
                close_bracket = "]".bright_black(),
                open_paren = "(".bright_black(),
                category = category.to_string().bright_black(),
                close_paren = ")".bright_black(),
                text = text.to_string().bright_black()
            );
        }
    }
}
