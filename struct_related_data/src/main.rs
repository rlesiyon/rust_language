//normal struct
#[derive(Debug)]
struct User {
    active: bool, 
    username: String, 
    email: String, 
    sign_in_count: u64,
}

#[derive(Debug)] // allows you to use println! or print! macro.
struct Rectangle {
  width: u32, 
  height: u32,
}

// method definition
impl Rectangle {
  fn area(&self) -> u32 {
    self.width*self.height
  }

  fn set_width(&mut self, width: u32) {
    self.width = width;
  }

  fn can_hold(&self, other: &Rectangle) -> bool {
    self.width >= other.width && self.height >= other.height
  }
}


// tuple struct: 
#[derive(Debug)]
struct Color(i32, i32, i32);
#[derive(Debug)]
struct Point(i32, i32, i32);

//unit-like structs without fields; 
#[derive(Debug)]
struct AlwaysEqual();

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("someone1@example.com");
    println!("\n{:?}", user1);

    //move in structs:
    // user1 becomes invalid and cannot be used afterwards: upon usage it will result in a variable is moved error.
    let user2 = User {
        active: false, 
        sign_in_count: 4, 
        ..user1
    };
    println!("\n{:#?}", user2);

    // copy in structs: 
    //user2 is still valid and can be used afterwards.
    let user3 = User {
        username: String::from("user2"),
        email: String::from("user2@example.com"),
        ..user2 
    };
    println!("\n{:?}", user2);
    println!("{:?}", user3);

    let color = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("\ncolor: {:?} \norigin: {:?}", 
                color, origin);

    //unit-like struct
    let subject = AlwaysEqual;
    
    
    // using methods
    let mut rect1 = Rectangle{
        width: 30, 
        height: 10,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    println!("The area of the rectangle: {}", rect1.area());
    //change 
    rect1.set_width(40);
    println!("The area of the rectangle: {}", rect1.area());

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));

}
