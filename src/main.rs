struct Rectangle{
    w: i32,
    h: i32
}

fn main(){
    let mut anon_func : fn() = || {
        println!("I am anonymous");
    };

    fn f(){
        println!("I am not anonymous");
    }

    anon_func = f;

    let result = anon_func();
    let area : fn(Rectangle) -> i32 = |r: Rectangle| -> i32{
        r.w * r.h
    };

    let sum : fn(i32, i32) -> i32 = |a,b| a+b;
    sum(5,6);

    fn func_on_successed(){
        println!("ok");
    }
    fn func_on_failed(){
        println!("error");
    }

    let variable_on_seccessed = func_on_successed;
    let variable_on_failed = func_on_failed;
    external::handler(variable_on_seccessed, variable_on_failed);
}

mod external{
    pub fn handler(on_successed : fn(), on_failed : fn()){
        let ret = do_http_call();
        if ret == true{
            on_successed();
        }else{
            on_failed();
        }
    }

    fn do_http_call() -> bool{
        false
    }
}