fn main(){
    
    let mut s = String::from("s");

    let mut count = 0;

    while count <= 2{
        s = take_and_return(s);
        count+=1;

    }
}



fn take_and_return(mut str: String) -> String {
    
    str.push_str("-");
    str
}

