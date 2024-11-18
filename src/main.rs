use std::env::args;

use mrml::prelude::{parser::ParserOptions, render::RenderOptions};

fn main() {
    if args().len() != 2 {
        eprintln!("This program takes 2 argument");
        return;
    }
    let template = args().skip(1).next().unwrap();

    let parser_options = ParserOptions::default();
    let render_options = RenderOptions::default();
    match mrml::parse_with_options(&template, &parser_options) {
        Ok(mjml) => match mjml.render(&render_options) {
            Ok(html) => print!("{}", html),
            Err(err) => eprintln!("Failed to render to html: {err:?}"),
        },
        Err(err) => eprintln!("Failed to parse template: {err:?}"),
    };
}
