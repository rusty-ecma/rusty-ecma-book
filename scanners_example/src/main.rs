use ress::{
    Scanner,
    refs::RefScanner,
};
fn main() {
    let js = include_str!("../example.js");
    let scanner = Scanner::new(js);
    let ref_scanner = RefScanner::new(js);
    for (i, (item, ref_item) )in scanner.zip(ref_scanner).enumerate() {
        let prefix = if i < 10 {
            format!(" {}", i)
        } else {
            format!("{}", i)
        };
        println!("{} token: {:?}", prefix, item.token);
        println!("     ref: {:?}", ref_item.token);
    }
}
