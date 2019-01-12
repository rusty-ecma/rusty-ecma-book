use ressa::*;

static JS: &str = "
function Thing(stuff) {
    this.stuff = stuff;
}
class Stuff {
    constructor(thing) {
        this.thing = thing;
    }
}
";

fn main() {
    let parser = Parser::new(JS).expect("Failed to create parser");
    for part in parser {
        let part = part.expect("Failed to parse part");
        println!("{:?}", part);
    }
}