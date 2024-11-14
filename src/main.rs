use clap::Parser;
use mrml::prelude::{parser::ParserOptions, render::RenderOptions};

#[derive(Parser, Debug)]
#[command(about, long_about = None)]
struct Args {
    #[arg(short, long)]
    template: String,
}

fn main() {
    let args = Args::parse();

    let parser_options = ParserOptions::default();
    let render_options = RenderOptions::default();
    match mrml::parse_with_options(&args.template, &parser_options) {
        Ok(mjml) => match mjml.render(&render_options) {
            Ok(html) => print!("{}", html),
            Err(err) => eprintln!("Failed to render to html: {err:?}"),
        },
        Err(err) => eprintln!("Failed to parse template: {err:?}"),
    };
}
