use text_colorizer::*;

fn main() {
}

fn print_usage() {
    eprintln!("{} - change occurences of one string into another",
              "quickreplace".green());
    eprintln!("Usgae: quickreplace <target> <replacement> <INPUT> <OUTPUT>");
}

#[derive(Debug)]
struct Arguments {
    target: String,
    replacement: String,
    filename: String,
    output: String,
}
