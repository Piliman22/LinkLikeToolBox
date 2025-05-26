use console::{Style, Term};

pub struct ColorScheme {
    pub success: Style,
    pub error: Style,
    pub warning: Style,
    pub info: Style,
    pub highlight: Style,
}

impl Default for ColorScheme {
    fn default() -> Self {
        Self {
            success: Style::new().green(),
            error: Style::new().red(),
            warning: Style::new().yellow(),
            info: Style::new().cyan(),
            highlight: Style::new().magenta(),
        }
    }
}

pub struct Output {
    term: Term,
    colors: ColorScheme,
}

impl Output {
    pub fn new() -> Self {
        Self {
            term: Term::stdout(),
            colors: ColorScheme::default(),
        }
    }

    // Print
    pub fn print_success(&self, message: &str) {
        println!("{}", self.colors.success.apply_to(message));
    }

    pub fn print_error(&self, message: &str) {
        eprintln!("{}", self.colors.error.apply_to(message));
    }

    pub fn print_warning(&self, message: &str) {
        eprintln!("{}", self.colors.warning.apply_to(message));
    }

    pub fn print_info(&self, message: &str) {
        println!("{}", self.colors.info.apply_to(message));
    }

    pub fn print_highlight(&self, message: &str) {
        println!("{}", self.colors.highlight.apply_to(message));
    }
}