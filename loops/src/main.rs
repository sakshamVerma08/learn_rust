fn main(){

    let mut counter = 0;

    let res = loop{
        counter += 1;

        if counter == 10{
            break counter *2;
        }
    };

    println!("The result of loop = {}",res);


}
