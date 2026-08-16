fn main(){

    let a = 25.000; // no type declared     
    
    let b: f32 = 3.0;
    let c: f32 = 5.0;
    let d = b/c; // see it's size 
    println!("Size of 'd' = {} bytes", std::mem::size_of_val(&d));


    println!("Size of 'a' = {}", std::mem::size_of_val(&a));
}
