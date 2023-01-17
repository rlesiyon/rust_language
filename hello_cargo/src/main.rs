fn main() {
    let x: u8 = 25;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

       // addition
    let sum = 5.1/10.5;

    println!("addition : {sum} ")
}

