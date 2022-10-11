trait Vehicle{
    fn change_color(&mut self, color: String);
}

impl Vehicle for Car{
    fn change_color(&mut self, color: String){
        self.color = color;
    }
}

struct Car{
    model: String,
    year: i32,
    color: String,
    owner: String
}

fn main(){
    let mut car = Car{
        model: "Subaru".to_string(),
        year: 2015,
        color: "Blue".to_string(),
        owner: "Yuriy".to_string()
    };

    car.change_color("white".to_string());
    println!("The color is: {}", car.color);
}