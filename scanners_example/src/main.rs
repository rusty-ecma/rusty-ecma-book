use ress::{
    Scanner
};
fn main() {
    let js = include_str!("../example.js");
    let scanner = Scanner::new(js);
    for (i, item) in scanner.enumerate() {
        let item = item.unwrap();
        let prefix = if i < 10 {
            format!(" {}", i)
        } else {
            format!("{}", i)
        };
        println!("{} token: {:?}", prefix, item.token);
    }
}

#[cfg(test)]
mod test {
    use ress::*;
    #[test]
    fn chapter_1_1() {
        let js = "var i = 0;";
        let scanner = Scanner::new(js);
        for token in scanner {
            println!("{:#?}", token.unwrap());
        }
    }
    use ressa::Parser;
    #[test]
    fn ressa_ex1() {
        static JS: &str = "
function Thing(stuff) {
    this.stuff = stuff;
}
";
        let parser = Parser::new(JS).expect("Failed to create parser");
        for part in parser {
            let part = part.expect("Failed to parse part");
            println!("{:#?}", part);
        }
    }
}
