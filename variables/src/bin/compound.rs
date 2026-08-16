fn main(){

    let tup1 : (i32,i32,i32) = (1,2,3);
    let x = tup1.0;
    let y = tup1.1;
    let z = tup1.2;
    
    println!("x+y+z = {}", x+y+z);

    let arr: [i32;5] = [1,2,3,4,5];

    println!("arr[0] = {}\narr[1] = {}\narr[2] = {}\narr[3] = {}\narr[4] = {}", arr[0],arr[1],arr[2],arr[3],arr[4]);

    let arr2: [f64;3] = [1.00, 2.23, 3.12];
    println!("arr2[0] = {}\narr2[1] = {}\narr[2] = {}", arr2[0], arr2[1], arr2[2]);
} 
