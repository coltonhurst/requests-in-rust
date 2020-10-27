/*
    A very basic synchronous HTTP GET call with `reqwest`
*/

fn main() {
    let body = reqwest::blocking::get("https://www.rust-lang.org/")
        .unwrap()
        .text();
    println!("body = {:?}", body);
}
