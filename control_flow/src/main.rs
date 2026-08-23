use std::io;


fn main(){

    celsius_to_fahrenheit();
    fahrenheit_to_celsius();
}


fn celsius_to_fahrenheit(){

   let mut temp = String::new(); // defining a new string var to hold user's input value
    
   println!("Enter the temperature: ");

   io::stdin()
       .read_line(&mut temp)
       .expect("Enter a valid temperature with no decimals");
   
    // converting the temperature to i32
    let temp: i32 = temp.trim().parse().expect("unparsable temperature");


    // converting the temperature to f64
    let temp: f64 = f64::from(temp); // explicitly annotating the f64 type so that .into() can work.

    // finding out the equivlent fahrenheit output
    let temp: f64 = (temp * 1.8) + 32.0;


    // printing the final result.
    println!("Equivalent F temp = {}", temp);

}


fn fahrenheit_to_celsius(){

    let mut input = String::new();

    println!("Enter the temperature: ");

    io::stdin()
        .read_line(&mut input)
        .expect("Enter a valid temperature");

    // converting the input to i32
    let input: i32 = input.trim().parse().expect("unparsable temp");

    // converting the input to f64
    let input: f64 = f64::from(input);


    let output: f64 = (input - 32.0) * 0.556;

    println!("Equivalent C temp = {}", output);

}


fn _check_age(){

  
    let mut age = String::new(); // Defining a new string instance 
    
    println!("Enter your age: ");

    io::stdin()
        .read_line(&mut age) // appends the user input to the already existing string variable age
        .expect("Please enter a valid age!");

    let age: i32 = age.trim().parse().expect("Unparsable Age");

   
    if age < 10{
        println!("age is less than 10");
    }
    else if age > 10 && age < 15 {
        println!("age is between 10 and 15");

    }
    else if age > 15 && age < 20{
        println!("age is between 15 and 20");
    }
    else{
        println!("age is greater than 20");
    }
}
