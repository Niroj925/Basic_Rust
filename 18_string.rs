fn main(){

//string is sequence of unicode character encoded in UTF-8
//"niroj"->'n','i','r'..

//string is heap allocated and dynamic size
//the size of string is unknown at compile time
//creating string
let str=String::from("Namaste Nepal.");
println!("{}",str);

let mut name=String::from("Thapa ");
name.push_str("kaji");
println!("{}",name);

//string slicing
let ss=&name[2..6];//pointer used to slice
println!("{}",ss);

//iterating over string
for char in name.chars(){
    println!("{}",char);
}

//creating empty string with string::new()
let mut pet=String::new();
pet.push_str("dog");
println!("{}",pet);

//the main difference between String and &str

//String
//1.is a data type allcates the memory in heap
//2.fixed size and can be modified

//&str
//1.a view of string store somewhere in memory
//2. also known as a string slice and can handle by using pointer as &str


//rust does not support string indexing
}