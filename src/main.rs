trait Employee{
    fn do_work(&mut self);
    fn show_balance(&self);
}

impl Employee for Developer{
    fn do_work(&mut self){
        self.cash += self.sallary;
    }
    fn show_balance(&self){
        println!("The balance is: {}", self.cash);
    }
}

impl ToString for Developer{
    fn to_string(&self) -> std::string::String{
        format!("My name is {}, i'm {} yo my profession is developer", 
        self.name, self.age)
    }
}

struct Developer{
    name: String,
    age: i32,
    cash: i32,
    sallary: i32
}

fn main(){
    let mut dev = Developer{
        name: "Yuriy".to_string(),
        age: 37,
        cash: 30,
        sallary: 10
    };

    dev.do_work();
    dev.show_balance();
    println!("Introduce: {}", dev.to_string());
}