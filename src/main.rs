use std::io;

fn main(){
    let mut a_str = String::new();
    let mut b_str = String::new();
    let mut c_str = String::new();

    loop{
        println!("Enter a value:");

        match io::stdin().read_line(&mut a_str){
            Ok(_) =>{},
            Err(e) => println!("some error: {}", e)
        }

        println!("Enter b value:");

        match io::stdin().read_line(&mut b_str){
            Ok(_) => {},
            Err(e) => println!("some error: {}", e)
        }

        println!("Enter c value:");

        match io::stdin().read_line(&mut c_str){
            Ok(_) => {},
            Err(e) => println!("some errors: {}", e)
        }

        let a: f64 = a_str.trim().parse().unwrap();
        let b: f64 = b_str.trim().parse().unwrap();
        let c: f64 = c_str.trim().parse().unwrap();

        let d : f64 = (b * b) - 4.0 *a *c;

        if d > 0.0{
            let x1 = ((-b) + d.sqrt()) / (2.0 * a);
            let x2 = ((-b) - d.sqrt()) / (2.0 * a);
            println!("The x1 and x2 values are: {}, {}", x1, x2);
        }
        if d == 0.0{
            let x = (-b) / (2.0 * a);
            println!("The value of x is: {}", x);
        }
        if d < 0.0{
            println!("No any resolves for this expression");
        }
    }

}