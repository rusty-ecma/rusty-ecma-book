#[macro_use]
extern crate serde_derive;
use ress::prelude::*;
use toml::from_str;
use std::{
    env::args,
    fs::read_to_string,
    io::{IsTerminal, Read},
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
            print_usage();
            std::process::exit(1);
        } else {
            js
        },
        Err(_) => {
            print_usage();
            std::process::exit(1);
        }
    };
    let finder = BannedFinder::new(&js, banned);
    for item in finder {
        match item {
            Ok(_) => (),
            Err(msg) => println!("{}", msg),
        }
    }
}

fn get_js() -> Result<String, ::std::io::Error> {
    let mut cmd_args = args();
    let _ = cmd_args.next(); //discard bin name
    let js = if let Some(file_name) = cmd_args.next() {
        let js = read_to_string(file_name)?;
        js
    } else {
        let mut std_in = ::std::io::stdin();
        let mut ret = String::new();
        if std_in.is_terminal() {
            return Ok(ret)
        }
        std_in.read_to_string(&mut ret)?;
        ret
    };
    Ok(js)
}

struct BannedFinder<'a> {
    scanner: Scanner<'a>,
    banned: BannedTokens,
}

impl<'a> BannedFinder<'a> {
    pub fn new(js: &'a str, banned: BannedTokens) -> Self {
        Self {
            scanner: Scanner::new(js),
            banned,
        }
    }
}

impl<'a> Iterator for BannedFinder<'a> {
    type Item = Result<(), BannedError>;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.scanner.next() {
            match item {
                Ok(item) => {
                    Some(match &item.token {
                        Token::Ident(ref id) => {
                            let id = id.to_string();
                            if self.banned.idents.contains(&id) {
                                Err(BannedError(format!("identifier {}", id), item.location.start.line, item.location.start.column))
                            } else {
                                Ok(())
                            }
                        },
                        Token::Keyword(ref key) => {
                            if self.banned.keywords.contains(&key.to_string()) {
                                Err(BannedError(format!("keyword {}", key.to_string()), item.location.start.line, item.location.start.column))
                            } else {
                                Ok(())
                            }
                        },
                        Token::Punct(ref punct) => {
                            if self.banned.puncts.contains(&punct.to_string()) {
                                Err(BannedError(format!("punct {}", punct.to_string()), item.location.start.line, item.location.start.column))
                            } else {
                                Ok(())
                            }
                        },
                        Token::String(ref lit) => {
                            match lit {
                                StringLit::Double(inner)
                                | StringLit::Single(inner) => {
                                    if self.banned.strings.contains(&inner.to_string()) {
                                        Err(BannedError(format!("string {}", lit.to_string()), item.location.start.line, item.location.start.column))
                                    } else {
                                        Ok(())
                                    }
                                }
                            }
                        },
                        _ => Ok(()),
                    })
                },
                Err(_) => {
                    None
                }
            }
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct BannedError(String, usize, usize);

impl ::std::error::Error for BannedError {

}

impl ::std::fmt::Display for BannedError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "Banned {} found at {}:{}", self.0, self.1, self.2)
    }
}

fn print_usage() {
    println!("banned_tokens <infile>
cat <path/to/file> | banned_tokens");
}
