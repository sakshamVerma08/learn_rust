fn main(){
    
    let mut s1 = String::from("NeoVim Motions are amazing !!");
    
    // immutable references 
    let i1 = &s1;
    let i2 = &s1;
    let i3 = &s1;


    // mutable reference to the same value : s1
    let mut_ref: &mut String = &mut s1;
    println!("i1 = {}", i1);
}



