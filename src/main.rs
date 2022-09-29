fn main(){
    let message = "i love Rust";
    let func = ||{
        println!("message: {}", message);
    };

    func();

    let mut num = 0;
    let mut add_one = ||{
        num += 1;
    };

    add_one();
    add_one();
    add_one();
}