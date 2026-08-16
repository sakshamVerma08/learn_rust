fn main(){

    let sum = 5 + 10; // type should be i32 --> 4 bytes.
    let diff = 25 as f64 - 12.00; // check type 
    let mult = 25 * 25;
    let quotient = 4/2;
    let rem = 4%2;

    let s1 = std::mem::size_of_val(&sum);
    let s2 = std::mem::size_of_val(&diff);
    let s3 = std::mem::size_of_val(&mult);
    let s4 = std::mem::size_of_val(&quotient);
    let s5 = std::mem::size_of_val(&rem);

    println!("Sum = {}\nDiff={}\nMult={}\nQuotient={}\nRemainder={}", sum,diff,mult,quotient,rem);
    println!("\nsize of 'sum' = {}\nsize of 'diff' = {}\nsize of 'mult' = {}\nsize of quotient = {}\nsize of 'rem' = {}", s1,s2,s3,s4,s5);
}
