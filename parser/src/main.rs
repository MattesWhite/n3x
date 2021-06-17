use anyhow::Result;
use clap::{arg_enum, value_t, App, Arg};
use parser::{n3, n3x, turtle};
use pest::Parser;
use std::io::Read;

arg_enum! {
    #[derive(Debug, Clone, Copy)]
    enum Parsers {
        Turtle,
        N3,
        N3X,
    }
}

impl Default for Parsers {
    fn default() -> Self {
        Parsers::Turtle
    }
}

fn main() -> Result<()> {
    let app = App::new("Grammar Validator")
        .version("0.1")
        .author("Matthias Farnbauer-Schmidt <matthias.farnbauer-schmidt@fau.de>")
        .about("CL tool for parsing different serialization formats of RDF")
        .arg(Arg::with_name("format")
            .short("f")
            .long("format")
            .help("specify the format to parse")
            .possible_values(&Parsers::variants())
            .case_insensitive(true)
            .takes_value(true)
            .default_value("Turtle")
        )
        .arg(Arg::with_name("input")
            .short("i")
            .long("input")
            .help("Hand in a string to parse. If this option is not set the input is read from `stdin`")
            .takes_value(true)
        )
        .get_matches();

    // acquire input
    let input = if let Some(input) = app.value_of("input") {
        input.to_owned()
    } else {
        let mut buffer = String::new();
        std::io::stdin().read_to_string(&mut buffer).unwrap();
        buffer
    };

    // parse
    match value_t!(app.value_of("format"), Parsers).unwrap() {
        Parsers::Turtle => {
            turtle::Parser::parse(turtle::Rule::turtleDoc, &input).map(|pairs| {
                println!("XXX Parsed:\n{:#?}\nXXX", pairs);
            })?;
        }
        Parsers::N3 => {
            n3::Parser::parse(n3::Rule::document, &input).map(|pairs| {
                println!("XXX Parsed:\n{:#?}\nXXX", pairs);
            })?;
        }
        Parsers::N3X => n3x::prarse_and_print(&input)?,
    };

    Ok(())
}
