#[macro_use]
extern crate serde_derive;
use ress::{
    Scanner,
    Token,
};
use toml::from_str;
use std::{
    env::args,
    fs::read_to_string,
    io::Read,
};

#[derive(Deserialize)]
struct BannedTokens {
    idents: Vec<String>,
    keywords: Vec<String>,
    puncts: Vec<String>,
    strings: Vec<String>,
}

fn main() {
    let config_text = ::std::fs::read_to_string("banned_tokens.toml").expect("failed to read config");
    let banned: BannedTokens = from_str(&config_text).expect("Failed to deserialize banned tokens");
    let js = match get_js() {
        Ok(js) => if js.len() == 0 {
            println!("Unable to get javascript");
            print_usage();
            std::process::exit(1);
        } else {
            js
        },
        Err(e) => {
            println!("Error getting javascript {}", e);
            print_usage();
            std::process::exit(1);
        }
    }
    for item in finder {
        match item {
            Ok(_) => (),
            Err(msg) => println!("{}", msg),
        }
    }
}

fn get_js() -> Result<String, ::std::io::Error> {
    let args = args();
    let mut cmd_args = args();
    let _ = cmd_args.next(); //discard bin name
    let js = if let Some(file_name) = cmd_args.next() {
        let js = read_to_string(file_name)?;
        js
    } else {
        let mut std_in = ::std::io::stdin();
        let mut buf = String::new();
        std_in.read_to_string(&mut buf)?;
        buf
    };
    Ok(js)
}

struct BannedFinder {
    scanner: Scanner,
    banned: BannedTokens,
}

impl BannedFinder {
    pub fn new(js: &str, banned: BannedTokens) -> Self {
        Self {
            scanner: Scanner::new(js),
            banned,
        }
    }
}

impl Iterator for BannedFinder {
    type Item = Result<(), BannedError>;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.scanner.next() {
            Some(match &item.token {
                Token::Ident(ref id) => {
                    let id = id.to_string();
                    if self.banned.idents.contains(&id) {
                        Err(BannedError(id, item.span.start))
                    } else {
                        Ok(())
                    }
                },
                Token::Keyword(ref key) => {
                    if self.banned.keywords.contains(&key.to_string()) {
                        Err(BannedError(key.to_string(), item.span.start))
                    } else {
                        Ok(())
                    }
                },
                Token::Punct(ref punct) => {
                    if self.banned.puncts.contains(&punct.to_string()) {
                        Err(BannedError(punct.to_string(), item.span.start))
                    } else {
                        Ok(())
                    }
                },
                Token::String(ref lit) => {
                    if self.banned.strings.contains(&lit.no_quote()) {
                        Err(BannedError(lit.no_quote(), item.span.start))
                    } else {
                        Ok(())
                    }
                },
                _ => Ok(()),
            })
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct BannedError(String, usize);

impl ::std::error::Error for BannedError {

}

impl ::std::fmt::Display for BannedError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "Banned token {:?} found at {}", self.0, self.1)
    }
}

fn print_usage() {
    println!("banned_tokens <infile>
cat <path/to/file> | banned_tokens");
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn main() {
        let js = "'use strict'";

    }
}
