struct Rectangle{
    w: i32,
    h: i32
}

fn main(){
    let anon_func = || {
        println!("I am anonymous");
    };

    let result = anon_func();
    let area = |r: Rectangle| -> i32{
        r.w * r.h
    };

    let sum = |a,b| a+b;
    sum(5,6);
}
