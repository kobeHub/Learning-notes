/*Create struct and use it*/
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Twodim {
    color: Color,
    point: Point,
}
// tulpe struct with no named-filelds
#[derive(Debug)]
struct Color(i32, i32, i32);

#[derive(Debug)]
struct Point(i32, i32, i32);

#[derive(Debug)]
struct Account(User, String);

pub fn create(email: String, username: String) {
    let user1 = User{
        email: String::from("jdgets111@gamil.com"),
        username: String::from("Inno Jia"),
        sign_in_count: 12,
        active: true
    };
    println!("{:#?}", user1);
    println!("Create user with same name variable:{}, {}", email, username);
    let user2 = User{
        username,
        email,
        ..user1    // use the fields of user1
    };
    println!("{:?}", user2);

    let dim = Twodim{
        point: Point(1, 2, 3),
        color: Color(23, 78, 9)
    };
    println!("Struct with struct fileld:{:?}", dim);

    let acc = Account(user1, "Just a password".to_string());
    println!("{:?}", acc);

}

pub fn tuple_struct() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    // let test: Point = black; different type, not allowed
    println!("The black color:{:#?}", black);
    println!("The origin point:{:#?}", origin);
}
