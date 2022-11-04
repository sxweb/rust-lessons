fn main(){
    let num = 10;

    match num {
        15 => println!("the num is 15"),
        10 => {
            println!("the num is 10");
            println!("the num matched");
        },
        5..=50 =>{
            println!("The num is between 5 and 10");
        },
        _ => println!("no any matches")
    }
}