
fn main(){
    let mut counter = 0;
    let mut lang = ["C#", "Rust", "Python"];
    lang[0] = "Java Script";

    for i in lang.iter(){
        println!("Lang: {}", i);
    }

    loop{        
        counter += 1;
        if counter >= 5{
            break;
        }
        println!("I love Rust");
    }
    let mut some = true;
    while some {
        println!("Value: {}", some);
        some = !some;
    }
}
