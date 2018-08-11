extern crate reqwest;
extern crate regex;
extern crate clap;

use regex::Regex;

const LINE_CHAR_LIMIT: i32 = 75;

use std::process;
use clap::{Arg, App};

fn main() {

    let cfg = App::new("urbandict")
                    .version("1.1")
                    .author("izumariu")
                    .about("Look up cool slang words in the terminal!")
                    .arg(Arg::with_name("entries")
                        .short("n")
                        .long("entries")
                        .value_name("N")
                        .help("Show N entries maximum. NOT IMPLEMENTED YET!") //TODO!!!
                        .required(false)
                    )
                    .arg(Arg::with_name("word")
                        //.short("q")
                        //.long("query")
                        .value_name("WORD")
                        .help("Term to look up")
                        .required(true)
                        .index(1)
                    )
                    /*
                    .arg(Arg::with_name("word_of_the_day") // TODO
                        .help("Look up the word of the day. Will nullify all other arguments.")
                    )*/
                    .get_matches();



    let word = cfg.value_of("word").unwrap();

    eprintln!("Searching for '{}'", word);

    match find_word(&cfg) {

        Some(expl) => {
            print_with_readability( Explanation { word: String::from(word) , expl: String::from(expl) } )
        },

        None => {
            eprintln!("Sorry, we couldn't find '{}' {}", word, r"¯\_(ツ)_/¯");
            process::exit(1);
        },

    }

}


struct Explanation {
    word: String,
    expl: String,
}

fn print_with_readability(cfg: Explanation) {

    let word = cfg.word;
    let expl = cfg.expl;

    let mut header = String::from("\n");

    {
        let headline = format!(" {} ", String::from(word).to_ascii_uppercase());
        header.push_str(&format!("{}\n", headline));
        for _ in 0..headline.len() {
            header.push('-');
        }
        header.push('\n');
        header.push_str("from urbandictionary.com\n");
    }

    println!("{}", header);

    let mut line = String::new();
    for word in expl.split_whitespace() {
        let newlen = (line.len() + word.len() + 1) as i32;
        if newlen > LINE_CHAR_LIMIT {
            println!("{}", line);
            line = String::new();
        }
        if line != String::new() {
            line.push(' ');
        }
        line.push_str(&format!("{}", xmlesc(&word)));
    }
    println!("{}", line);

}

fn xmlesc(text: &str) -> String {
    let mut out = String::from(text);
    out = out.replace("&apos;", "'");
    out = out.replace("&quot;", "\"");
    out = out.replace("&lt;", "<");
    out = out.replace("&gt;", ">");
    out = out.replace("&amp;", "&");
    out
}

fn urlencode(arg: &str) -> String {
    let mut out = String::new();
    for val in arg.bytes() {
        if val >= 16 {
            out.push_str(&format!("%{:x}", val));
        } else {
            out.push_str(&format!("%0{:x}", val));
        }
    }
    out
}

fn find_word(cfg: &clap::ArgMatches) -> Option<String> {

    let word = cfg.value_of("word").unwrap();

    let response = match reqwest::get(&format!("https://www.urbandictionary.com/define.php?term={}", urlencode(&word))) {

        Ok(mut resp) => {

            match resp.text() {

                Ok(content) => content,

                Err(_) => {
                    eprintln!("Error: Could not extract body");
                    process::exit(1);
                },

            }

        },

        Err(_) => {
            eprintln!("Error: No response from server");
            process::exit(1);
        },

    };

    let error_rx = Regex::new(r"Sorry, we couldn't find").unwrap();
    let found_rx = Regex::new("<meta[^<>]+name=\"Description\"[^<>]*>").unwrap();
    let descr_rx = Regex::new("content=\"([^\"]+)\"").unwrap();

    if let Some(_) = error_rx.captures(&response) {
        // word does not exist
        return None;
    }

    if let Some(metatag) = found_rx.captures(&response) {
        // word does seem to exist
        if let Some(content) = descr_rx.captures(&metatag.get(0).unwrap().as_str()) {
            if let Some(caps_1) = content.get(1) {
                return Some(caps_1.as_str().to_string());
            }
        }
    }

    None

}
