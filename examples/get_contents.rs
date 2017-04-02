extern crate scrappy;

fn main() {
    let result = scrappy::get_content("www.google.lt");
    println!("{:?}", result);
    
    let result = scrappy::get_content("www.google.lt").unwrap().find("google");
    println!("{:?}", result);
}
