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
        if atty::is(atty::Stream::Stdin) {
            return Ok(ret)
        }
        std_in.read_to_string(&mut ret)?;
        ret
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
                        let (row, column) = self.get_position(item.span.start);
                        Err(BannedError(format!("identifier {}", id), row, column))
                    } else {
                        Ok(())
                    }
                },
                Token::Keyword(ref key) => {
                    if self.banned.keywords.contains(&key.to_string()) {
                        let (row, column) = self.get_position(item.span.start);
                        Err(BannedError(format!("keyword {}", key.to_string()), row, column))
                    } else {
                        Ok(())
                    }
                },
                Token::Punct(ref punct) => {
                    if self.banned.puncts.contains(&punct.to_string()) {
                        let (row, column) = self.get_position(item.span.start);
                        Err(BannedError(format!("punct {}", punct.to_string()), row, column))
                    } else {
                        Ok(())
                    }
                },
                Token::String(ref lit) => {
                    if self.banned.strings.contains(&lit.no_quote()) {
                        let (row, column) = self.get_position(item.span.start);
                        Err(BannedError(format!("string {}", lit.to_string()), row, column))
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

impl BannedFinder {
    fn get_position(&self, idx: usize) -> (usize, usize) {
        let (row, line_start) = self.scanner.stream[..idx]
            .char_indices()
            .fold((1, 0), |(row, line_start), (i, c)| if is_js_new_line(c) {
                (row + 1, i)
            } else {
                (row, line_start)
            });
        let col = if line_start == 0 {
            idx
        } else {
            idx.saturating_sub(line_start)
        };
        (row, col)
    }
}

fn is_js_new_line(c: char) -> bool {
    c == '\n'
    || c == '\u{2028}'
    || c == '\u{2029}'
}