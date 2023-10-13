use text_colorizer::Colorize;

pub fn print_usage() {
    eprintln!("{} - change occurrences of one string into another", "quick replace".green());
    eprintln!("Usage: quick_replace <target> <replacement> <INPUT> <OUTPUT>");
}