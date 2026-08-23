use std::io;


fn main(){

    simple_interest_calc();   

}


fn _celsius_to_fahrenheit(){

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


fn simple_interest_calc(){


    let mut principal = String::new();
    let mut roi = String::new();
    let mut time_in_years = String::new();
    
    println!("Enter Principal Amount:");
    io::stdin()
        .read_line(&mut principal)
        .expect("Enter a valid type for the Principal amount");
    
    
    println!("Enter rate of interest:");
    io::stdin()
        .read_line(&mut roi)
        .expect("Enter a valid type for rate of interest");
    
    println!("Enter the time (in years):");
    io::stdin()
        .read_line(&mut time_in_years)
        .expect("Enter a valid time");


    let principal: i32 = principal.trim().parse().expect("Invalid principal amount entered");
    let principal: f64 = f64::from(principal);
    

    let roi: i32 = roi.trim().parse().expect("invalid roi entered");
    let roi: f64 = f64::from(roi);
    
    let time_in_years: i32 = time_in_years.trim().parse().expect("Invalid time entered");
    let time_in_years: f64 = f64::from(time_in_years);

    let output: f64 = (principal * roi * time_in_years)/100.0;

    println!("Simple Interest = {}", output);

}




fn _fahrenheit_to_celsius(){

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
