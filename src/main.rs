
fn main(){
    let mut lang = ["C#", "Rust", "Python"];
    lang[0] = "Java Script";

    for i in lang.iter(){
        println!("Lang: {}", i);
    }
}
