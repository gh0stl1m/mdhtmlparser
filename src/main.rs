use clap::{Command, arg, Arg};
use pulldown_cmark::{html::push_html, Parser};
use maud::html;

fn wrap_html(s: &str, css: Option<&str>) -> String {

    let res = html!{
        (maud::DOCTYPE)
        html {
            head {
                meta charset = "utf-8";
                @if let Some(s) = css {
                    link rel = "stylesheet" type = "text/css" href=(s) {}
                }
            }
            body {
                (maud::PreEscaped(s))
            }
        }
    };

    res.into_string()
}

fn main() {

    let command = Command::new("mrend")
        .about("A CLI to render markdown")
        .author("Santiago Sanchez Taborda")
        .subcommand(
            Command::new("parse")
                    .about("Parse markdown files into html")
                    .arg(arg!(<file> "File to be parsed"))
                    .arg(
                        Arg::new("wrap")
                           .short('w') 
                           .long("wrap")
                           .help("Wrap in a html file")
                    )
                    .arg(
                        Arg::new("css")
                           .long("css") 
                           .takes_value(true) 
                           .help("Link to CSS file")
                    )

        )
        .get_matches();

    match command.subcommand() {
        Some(("parse", sub_matches)) => {

            let infile = std::fs::read_to_string(sub_matches.value_of("file").unwrap()).expect("Could not read file");

            let mut html_string = String::new();
            let string_parsed= Parser::new(&infile);
            push_html(&mut html_string, string_parsed.into_iter());
            
            if sub_matches.is_present("wrap") {

                html_string = wrap_html(&html_string, sub_matches.value_of("css"))
            }

            println!("{}", html_string);
        },
        _ => println!("Command not found")
    }
}
