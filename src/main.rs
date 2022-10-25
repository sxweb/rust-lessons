struct Car{
    model: String,
    color: String,
    year: i32
}

struct Person{
    name: String,
    age: i32,
    car: Car
}

fn main(){
    let name = "Yuriy";
    println!("Hello, {}", name);
    let mut lada = Car{
        model: "Lada".to_string(),
        color: "Yellow".to_string(),
        year: 2008
    };
    let mut bmw = Car{
        model: "BMW".to_string(),
        color: "Black".to_string(),
        year: 2010
    };

    let mut yuriy = Person{
        name: "Yuriy".to_string(),
        age: 37,
        car: lada
    };

    buy_car(yuriy, bmw);
}

fn buy_car(person : Person, car : Car){
    person.car = car;
    println!("Congr, you got a {}", car.model.to_string());
}

