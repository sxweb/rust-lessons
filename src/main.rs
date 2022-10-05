struct Developer{
    name: String,
    age: i32,
    lang: String
}

impl Developer{
    fn get_age(&self)->i32{
        self.age
    }

    fn change_lang(&mut self, new_lang: String){
        self.lang = new_lang;
    }

    fn new(age: i32, name: String, lang: String) -> Developer{
        Developer{age, name, lang}
    }
}



fn main(){
    let mut dev = Developer{
        name: "Yuriy".to_string(),
        age: 37,
        lang: "Java Script".to_string()
    };

    let age = dev.get_age();
    println!("Dev age: {}", age);
    dev.change_lang("Rust".to_string());

    let new_dev = Developer::new(33, "John".to_string(), "C#".to_string());
}

