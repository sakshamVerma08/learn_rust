fn main(){

    let mut count: u32 = 0;
    
    println!("Counting until 30");

    loop {

        if count == 30{
            break;
        }

        count+=1;
    }

    println!("Value of count = {}", count);
}
