fn main() {

    // Move: the stack data is coppied and previous variable in invalidated.

    // trivial data types with known size at compile time
    let s_array = ['h', 'e', 'l', 'l', 'o'];
    let s1_array = s_array;
    println!("{:?} {:?}", s_array, s1_array);

    // complex data types with unknown size at compile time
    let s1 = String::from("hello");
    let mut s2= s1;
    println!("{}", s2);

    // clone: deeply copies the stack and heap data.
    let s = String::from("hello");
    let s_ = s.clone();
    println!("{} {}", s, s_);

    // reference
    let len = calculate_string_length(&s2);
    println!("{} {}", s2, len);

    //immutable reference
    // main reference can be used as possible.
    let s = String::from("immutable reference");
    let r1 = &s;
    let r2 = &s;
    let r3 = &s;
    println!("{}, {} {}", r1, r2, r3); 

    //mutable reference
    change(&mut s2);
    println!("{}, {}", s2, 
        calculate_string_length(&s2));

    // only one of the mutable reference can be defined at a time before it used. 
    let mut s = String::from("mutable reference");
    let r1 = &mut s;
    println!("{}", r1);
    let r2 = &mut s;
    println!("{}", r2);   

    // The slice type
    // String slice:
    let word = first_word(&s2);
    // s2.clear() is forbidden by the compiler as this is using a mutable reference more than one at a time. 
    println!("{}, {}", word, s2);

    // Array slice; 
    let a = [1, 2, 3, 4, 5];
    let slice = &a[..3];
    println!("{:?} =? {:?} {:?}",slice, 
                &[1, 2, 3], 
                assert_eq!(slice, &[1, 2, 3]));
                
}

//reference; 
fn calculate_string_length(s: &String) -> usize{
    s.len()
}

//change
fn change(s: &mut String) {
    s.push_str(" world!");
}

// slices
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

