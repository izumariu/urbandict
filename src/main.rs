extern crate reqwest;
extern crate regex;

use regex::Regex;

const LINE_CHAR_LIMIT: i32 = 85;

use std::{env, process};

fn main() {

    let query = match get_query() {
        Some(q) => q,
        None => {
            let exe = env::args().nth(0).unwrap();
            eprintln!("USAGE: {} [QUERY]", exe);
            process::exit(1);
        },
    };

    eprintln!("Searching for '{}'", query);

    match find_word(&query) {

        Some(expl) => {
            //eprintln!("{:?}", expl);
            print_with_readability(&query, &expl)
        },

        None => {
            eprintln!("Sorry, we couldn't find '{}' {}", query, r"¯\_(ツ)_/¯");
            process::exit(1);
        },

    }

}

fn print_with_readability(word: &str, expl: &str) {

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

fn get_query() -> Option<String> {

    if let Some(arg) = env::args().nth(1) {
        return Some(arg.to_string());
    }

    None

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

fn find_word(word: &str) -> Option<String> {

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
