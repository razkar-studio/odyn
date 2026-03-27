use farben::{cprintln, style};

pub(crate) fn init_styles() {
    style!("load", "[bold blue]");
    style!("success", "[load]");
    style!("warn", "[bold yellow]");
    style!("error", "[bold red]");
    style!("info", "[load]");
    style!("done", "[load]");
}

pub(crate) fn status(label: &str, style: &str, message: &str) {
    cprintln!("[{style}]{:>12}[/] {message}", label);
}
