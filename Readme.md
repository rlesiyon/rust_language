# Summary of Rust Language Concepts based on my understanding.

## 1.  Rust Language.

Rust is a static language, which implies that it must know the type of all variables at compile time.

Compiler infers the types based on how the value of the variable and how is used.

## 2. Common Progamming Concepts

### Variables and Mutability:

Variables are immutable by nature in rust; for example declared variable below is immutable.

```
let x = 5
x = 6 # this operation will give an error.
```

You can make a variable mutable by adding ***mut*** in front of it as show below. 
```
let mut x = 5
x = 6 # this operation is acceptable.
```

#### Constants

Convention of defining a constant is by use of capital letters as shown below. Constants are always immutable. 

```
const THREE_HOURS_IN_SECONDS = 60*60*3
```

Constants are valid for the entire time a progam runs, within the scope it was defined. 

#### Shadowing
A variable is shadowed when the same variable is redefined in a smaller scope inside the current scope. An example is shown below; 

```
fn main() {
  let x = 5;
  let x = x + 1;

  # scope for shadowing; scope2
  {
    let x = x *2
  }

}
```

Any operation done in scope2, uses the double value of x. And any operation done outside scope2, regardless is it before or after, will use the originally assigned value of x. 

### Data Types

There are two subsets of data types: scalar and compound. 

Scalar type represents a single value. Rust has four primary scalar types.

    - integers 
    - floating
    - boolean
    - char

Compound types involves combination of the scalar types to form a composite type. Rust has two compound types which are 

    - tuples
    - arrays.

### Functions

**fn** - functions keyword.
**main** - the enty point of main programs.

snake case is the conventional style for functions and variables.

Example of a function definition

```
fn even_number(number: i32) -> i32 {
  number%2 == 0
}
```
In function signatures, you must define the type of each parameter. Additionally the return type must be specified by the use of arrow (->).

The return value of a function is synonymous with the value of the final expression in the block of a function.

**Notes**

- Statements - are instructions that perform some actions are does not return a value.
- expressions - evaluate to a resultant value

- Understanding the distinction between statements and expressions helps in specification of the final return value in a function where the return is specified in its signature.

Examples
```
let y = 6 is a statement
y + 1 is an expression
```

### Control flows

#### if expressions; 

The condition must be a bool in order for a if expression to run. 

Usages: 

- control flow 
- if in a let statement

  ```
  let condition = true;
  let number = if condition { 3} else {5} 
  ```
#### Loops

keyword: **loop**

- Execute the code forever or until explicitly told to stop.

```
loop {
  do something
}

while condition {
  do something
}

for i in (1..4) {
  do something
}

```

**(1..4)** is a ***Range*** provided by standard library

For loop is mostly used due to its safety. For instance, if you want to loop through an array; using a while loop might results in out of bound index. Additionally, Rust will execute the runtime check of indexing which will slow the program.

## 3. Understanding ownership. 

Most programming languages uses pointers or garbage collectors to allocate memories. For pointers the programmer allocated and free the memory explicitly. In gargabe collectors, it looks for no-longer-used variables as the programs runs and free the memory according. 

Rust memory is managed by system of ownership that followers specific rules at compile time. If the rules are violated the program does not compile. 

**Ownership rules**
 
  - Each value in rust has a owner. 
  - There can only be one owner at a time. 
  - When the owner goes out of scope, the value is dropped.

### Variable scope. 

Defined by the curly braces.

Example: 

This example illustrate when the variable comes into scope and when it goes out of scope.
```
fn main() {

  {                      // s is not valid here, it’s not yet declared
      let s = "hello";   // s is valid from this point forward

      // do stuff with s
  }                      // this scope is now over, and s is no longer valid
}
```
#### Move

***Note***

- Trivial data types with a known size at compile time are stored in stack memory. 

- Complex data types, with unknown size during compile, example literal string are store in hep memory. 

In rust, heap data is not copied instead a pointer is used; a stack data is copied. And example illustration of this is shown below: 
<img src='https://doc.rust-lang.org/book/img/trpl04-02.svg' />

This scenario happens when the operation s2 = s1 is done.

***So what happens when s1 and s2 goes out of scope?***

  - Because s1 and s2 points to the same memory, this is called ***double memory freeing***. This is one of the memory bugs called ***double free error***.
  - To ensure safety, Rust consider s1 no longer valid after the s1 = s2 operation. In this case, the money is not freed when s1 goes out of scope

***Move*** - involves copying the pointer, length and capacity while also invalidating the first variable. 

After s2 = s1 operation in Rust, this is what actually happens, where the grayed image show s1 was invalidated. 
<img src = "https://doc.rust-lang.org/book/img/trpl04-04.svg"> 

#### Clone: 

- If we want to copy both the stack and heap data.
- This produces the behaviours shown below: 
<img src="https://doc.rust-lang.org/book/img/trpl04-03.svg">

- This can expensive operation at runtime.

#### Ownership and Functions

The mechanics of passing a variable to function is the same as move and copy. When a complex variable is passed to a function, the variable moves to the function scope and it becomes invalid at the current scope; it us move mechanisi.

For the native data types, with known size at compile time, the variable is valid after it moves to function scope. As it uses a copy mechanism.

#### Return value and scope. 

The return value becomes available at the scope calling it. 

### Reference
 
 Reference - is an address we can follow to obtain the data stored in another variable. In this way we access the data in a variable without taking ownership. 

 Reference can be mutable and immutable; which can be specified by the use of mut annotation. For mutable reference, only one reference is used at a time and any number of immutable reference. 

 ```
fn main(){
  let s= "immutable reference';
  let r1 = &s;
  let r2 = &s;
  println!("{}, {}", r1, r2)

}

 ```

 Dangling reference: is a situation in which a reference is given to another scope, in which the memory for the variable has been freed whereas the pointer is passed forward. 

```
fn main() {
  let reference_to_nothing = dangle();
}

fn dangle() -> &String {
  let s = String::from("hello");
  &s 
}

The dangle function return a reference to s, but s has already been dropped. 
```

### Slice Type 
- is a reference that gets a contiguous sequence of elements in a collection. 

Example.
```
let s = String::from("hello world");

let hello = &s[0..5];
let world = &s[6..11];

```

## 4. Using Structs to Structure Related Data

Structs - custom data types that enable grouping of multiple related values to make a meaningful group. Defined with the keyword ```struct```.

And example of struct is shown below. The field have an associated name. 
```
struct User {
  email: String, 
  username: String, 
  active: bool
}

//instance
let user1 = User{
  email: String::from("someone@example.com"),
  username: String::from("someone"),
  active: true
}
```

We can also extend the instanciating of a struct by the use of already existing struct. The syntax ```..user1``` used to autofill the remaining variables. For this example the active field of user2 is assigned to ```true```.

```
let user2 = User{
  email: String::from("someone1@example.com"), 
  username: String::from("someone1"),
  ..user1
}

let user3 = User{
  active: false,
  ..user2
}
```

This assignment affects of the ownership of the corresponding structs.

Since email and username are literal Strings this data is stored in a heap memory, whereas active which is a boolean is stored stack memory. 

- user2: will a copy since email and username are different.
- user3: user2 is moved to user3, since the email, and username are the same. Thus user2 is invalidated.


**Types of structs**

- struct: field have associated name.

        - struct User{
            name: String
          }; 

- Tuple structs: 

        - struct Color(i32, i32, i32); 

- Unit-Like Structs Without Any Fields

        - struct AlwaysEqual;

### Debugging structs.

To print a struct using println! macro, because a struct can be printed in different ways, rust compiler require the user to implement the Display traits. A simple way is to implement as shown below. 

```
#[derive(Debug)]
```

Now in println! macro, we can use the following specifiers

```
:?  -> prints: 

      Rectangle { width: 30, height: 10}

:#? -> prints: 

      Rectangle { 
                  width: 30, 
                  height: 10}
```

### Methods

Methods are simillar to functions with the keyword ```fn```. It can have parameters and a return value. It is always defined within the context of the struct. The first parameter is ```self```, which represents the instance of the struct. 

Example
```
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
}

// usage
fn main(){
  let rect1 = Rectangle {
    width: 30, 
    height: 10, 
  }

  println!("The area of Rectangle: {}", rect1.area())
}
```

#### Associated functions
Functions inside the impl block are called associated functions. 

Associated functions that do not take self as the first parameters are not methods, but usually referred as constructors. It mainly used to return a new instance of a struct. 

```
impl Rectangle {

  fn square(size: u32) -> Self {
    Self {
      width: size,
      height: size,
    }
  }
}

// calling this associated functions. 
let square = Rectangle::square(4);
```

```Self``` is the aliases of the type that appears after the keyword ```impl```.


## 5. Enums

Enums - is used to list  variants for a specific type. For instances, as of now, we have two IP address, V4 and V6. Enums can be used to enumerate this as shown below. The keyword is ```enum```.

```
enum IpAddrKind {
  V4,
  V6,
}
```

Additionally, enum variants can take different types and associated data as shown below. The V4 take for additional arguments wheres V6 take a string literal

```
enum IpAddrKind {
  V4(u8, u8, u8, u8), 
  V6(String)
}
```

**Option**

This is specifial type of enum, that allows to handle None values, when the variable does not have a value. It takes generic types. 

```
enum Option<T> {
  None, 
  Some(T),
}
```

Rust does not use null reference, the Option enum is used to handle this case. ----

### Binding. 

To get the values in enums, the patterns matching, like match is used to get the value. 

#### match. 

It an exhaustive pattern matching where all the possible control flow must be matched. When the code is run, the coin is checked if it matches any of the expression, from top down. When an expression is satisfified, the ```=>``` specifies what is done. For the case of the coins, there respective values are assigned. 

```
enum Coin {
  Penny
  Nickel
  Dime
  Quarter(state)
}

let coin = Coin::Penny;
// error: 
// This raise an error, that we must compile all possible cases. 
match coin {
  Coin::Penny => 1, 
  Coin::Nickel => 5, 
}

// correct way
match coin {
  Coin::Penny => 1, 
  Coin::Nickel => 5, 
  Coin::Dime =>10, 
  Coin::Quarter => 25,
}

```

**if let**

A more concise control flow, that allow matching of only one pattern. For instance, if we are only concern of the penny coin only, the ```if let``` will enable us to specify this in the code. 

```
if let Coin::Penny = coin {
  println!("This coin: {:?}", coin)
}
```

## 6. Managing Growing Projects with Packages, Crates and Modules

### Packages and crates

**Crate**

  - smallest amount of code that rust compiler considers at a time. 
  - Can contain a module, which might contain multiple file, and get compiled together with the crate. 

  Crate come in two forms: 

    - ***Binary*** := program compiled to an executable. It must contain a main function.
    - ***Library*** := don't have a main function and don't compile to an executable. 

**Package**

  - bundle of one or more creates that provide a functionality. It contain a cargo.toml file that instruct how build those crates. 

  - It must contain as many binary crates as possible and utmost one library. (must contain atleast one crate wether is a binary or a library.)


**Understand cargo new**

- Creates

```
src/main.rs :=> the root binary file.
Cargo.toml  :=> providing it a package.
src/lib.rs  :=> where library crate is stored. 
src/bin     :=> multiple binary crate can be placed here. 
```

## 8. Collections

### **Vec<T>: vector**

A vector stores more than one value (in a single data structure) in a contagious memory (next to each other).It must store element of the same type.

Initializing a vector, since vectors are implemented as generics ```Vec<T>```, when initializing an empty vector, type annotation must be added. 
```
let v : Vec<i32> = Vec::new()
```

Rust can also infer the data types, when created with the initial values. Use ```macro``` to initial the vector provided with the initial values. 
```
let v = vec![1, 2, 3]
```

#### Updating a Vector

Updating a vector, we need to make it mutable. And then used push to update it. ```v.push()```.

#### Reading an Elements in a vector. 

The vector can be access in two different ways: 
  - indexing. using []
  - get
      
      - It returns an an Option<T>, where ```T``` is the data types of values in the vector. We can then use match pattern to get the exact value.  

When to uses indexing or get. 

  - Indexing out of bound, will make the program to panic, but maybe you don't want you program to panic, good when you program to fail
  - With the use of get method, it allows customizing of the errors, in case of out of bound.

**Mutable and Immutable reference**

The convention is we cannot have a mutable and immutable reference in the same scope.

Example: 
```
let mut v = vec![1, 2, 3]
let first = &v.get(0)

v.push(4)
```

For the example below, doing inserting an element to the vector after taking an immutable reference, will break the program. This is mainly due to the contagious storage of memories in vectors. When a push is done, vector capacity might be used up, resulting to allocating the new memory, and copying all the values to that memory. In this case, the reference to the first element is deallocated. The borrowing rules in rust prevent this behaviour from happening. 

#### Iteration of a vector

It is save to iterate through a vector by using a mutable and immutable reference, ensuring the ownership is not transfer will iterating through a vector. 

```
let mut v = vec![1, 2, 3]

//immutable

for i in &v {
  print!("{i}")
}

//mutable

for i in &mut v {
  print!("{i}")
}
```

#### Composite data types storage in Rust vector

Since vectors only store values of the same datatype, we can use enum to make composite data types. 

```
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
  ];
}
```

### Storing UTF-8 Encoded Text With Strings

Rust only has one string type in the core language, which is ***str***. 

String: is a growable, mutable, owned, UTF-8 encoded string type. 

#### Creating a New String: 

Ways of creating a string: 

    - String::new() 
    - String::from("initial")
    - "initial".to_string()

#### Updating a string: 

We update a string by using a push_str method. The method take a string slice as we don't need to take ownership of the parameter.

    - push_str: take a string slice. 
    - push: take a single char.

 #### Concatenation with +Operator or the format! Macro. 

 **+ Operator**

 The method signature requires an owned string ```fn add(self, s: &str)```, thus we will need to do addition as follows. Add takes the ownership of s1 and take a reference of s2, which then get appended to s1 and then returned.

 Sometimes, s2 is a ```String``` type and not ```str``` as specified in the method. When ```add``` is called Rust uses ```deref coercion``` which turns ```&s``` to ```&s[...]``` thus coercing a String type to a str.  

 ```
 let s1 = "hello"
 let s2 = "world"
 let s3 = s1 + &s2
 ```

 **format macro**

 ``` format!("{s1}-{s2}-{s3}")```

 #### Indexing a string. 

In Rust, strings are not reference using an index why? 

  - Under the hood, String is a wrapper over a Vec<u8>, meaning the vector stores the UTF-8 encoding of each character. A char can have more that 1 byte of encoding thus the number of the char does not correspond to the how long the vector is. 
  - Indexing operation of String is not ```O(1)```, as Rust will have to figure out the number of valid characters. 

**Slicing strings**
Use a slice with to create an string slice ```&s[1..4]```.

**Methods for iterating Over Strings**

The best way to iterate through a string is to be specific: iterate through chars or bytes. 


```
for c in "Зд".chars() {
    println!("{c}");
}

for b in "Зд".bytes() {
    println!("{b}");
}
```

### Storing Keys with Associated Values in Hash Maps

Like vectors, hash maps are homogeneous: all of the keys must have the same type as each other, and all of the values must have the same type.

## 9. Error Handling








 













 












