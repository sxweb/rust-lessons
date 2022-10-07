struct Car{
    model: String,
    year: i32,
    color: String,
}

impl Car{
    fn get_year(&self)->&i32{
        &self.year
    }

    fn change_color(&mut self, color:String){
        self.color = color;
    }

    fn new(model: String, year: i32, color: String) -> Car{
        Car{
            model,
            year,
            color
        }
    }
}

fn main(){
    let mut car = Car{
        model: "Honda".to_string(),
        year: 2015,
        color: "Red".to_string()
    };

    println!("Year: {}", car.get_year().to_string());
    car.change_color("Black".to_string());

    let new_car = Car::new("BMW".to_string(), 2021, "Yellow".to_string());
    let year = new_car.get_year();
    println!("Year of new car: {}", year);
}