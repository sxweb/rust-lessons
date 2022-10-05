fn main(){
    let x = "hello rust".to_string();

    let data = x;

    //we cant use this var because the owner changed
    //println!("It's incorrect: {}", x);
    println!("we can use this var: {}", data);

    //and we can use clone func to copy the value
    let new_data = data.clone();
    println!("some data: {}", new_data);

    //if we use the value in the function we delegate the ownership to it
    //and we cant use this var in future
    let x = "some string".to_string();
    print( x);
    //and we cant use this:
    //println!("cant use: {}", x);

    //but we can also make a ref for variable
    let x = "value for x".to_string();
    let ref_x = &x;
    print_ref(ref_x);
    //and we steel can use x variable
    print(x);
}

fn print(value: String){
    println!("Now the func own this value: {}", value);
}

fn print_ref(value: &String){
    println!("Now the func use ref: {}", value);
}