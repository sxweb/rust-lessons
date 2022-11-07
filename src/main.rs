fn main(){
    let mut array = [1, 3, 4, 5, 8];
    println!("Array value is: {:?}", array);
    println!("{:?}", array[2]);

    array[2] = 7;
    println!("{:?}", array[2]);

    for i in array.iter(){
        println!("{}", i);
    }

    println!("The array length is: {}", array.len());

    for i in 0..array.len(){
        println!("the value is: {}", array[i]);
    }

    let mut i = 0;
    while i < array.len(){
        println!("the array value from while is: {}", array[i]);
        i += 1;
    }

    for i in array.iter(){
        if i % 2 == 0{
            println!("{}", i);
        }
    }

    let someArray = [4, 5, 2, 5, 6, 4, 9];

    let mut i = 0;
    while i < someArray.len(){
        let mut j = i + 1;
        while j < someArray.len(){
            if someArray[i] == someArray[j]{
                println!("some array douvbled value is: {}", someArray[i]);
            }
            j += 1;
        } 
        i += 1;
    }
}