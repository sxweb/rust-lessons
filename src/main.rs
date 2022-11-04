use std::io;// i - input, o - output

fn main(){
    let mut name = String::new();

    println!("Input your name");

    match io::stdin().read_line(&mut name){
        Ok(_) =>{
            println!("Hello, {}", name);
        },
        Err(e) =>{
            println!("Some error");
        }
    }
}