struct User{
    active: bool,
    username: String,
    email: String,
    password: String,
}

fn main(){

    let username1 = String::from("Userx");
    let email1 = String::from("x@x.com");
    let password1 = String::from("1234");

    let username2 = String::from("UserY");
    let email2 = String::from("y@x.com");
    let password2 = String::from("4567");

    let user1: User = build_user(true, username1, email1, password1);
    let user2: User = build_user(false, username2, email2, password2);

    println!("Email of user1 = {} ", user1.email);
    println!("Email of user2 = {}", user2.email);

   
}

fn build_user(active: bool, username: String, email: String, password: String) -> User {
    User{
        active: active,
        username,
        email,
        password
    }

}

