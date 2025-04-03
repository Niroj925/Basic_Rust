//ownership is a set of rules which ensure the memory saftey
//ownership model for memory management instead of garbage collector and manual memory management
//this makes rust differ from other language 
//allows program without memory leak and slowness

fn main(){

    //starting of block
{
    let name=String::from("Thapa Niroj.");
    println!("{}",name);
}//end of block

//name can not be access here 
//out of block the variable name can not access 
//when variable goes out of this scope then memory is free for name variable

//ownership rules in rust
//1.each value has a owner
//2.there can be only one ownership at a time 
//3.when owner goes out of scope then value will be dropped

//owner of the string value 
let fruit1=String::from("kera");//fruit1 is the owner of this string
//String stores data both in stack and heap
//when string variable binds to fruit1 
//string holds the pointer to the memory that holds content of string ,capacity and lenght 

//moves the ownership to other variable
//only ownership at a time
let fruit2=fruit1;
println!("{}",fruit2);

//can not print ownership has been moved
//error, out of scope,value dropped
// println!("{}",fruit1);

//the fruit2 variable pointer to the memory 


//data copy in 
//primitive types (integer, float, boolean) have none size at compile time 
//these all are stored in stack so these are cheap to copy

let x=10;

//copy 
let y=x;
println!("{},{}",x,y);
//here x is copy to y because intiger,float and boolean copied instead of copy
//implement copy trait by default 


//ownership of the fruit2 is moved to this function
print_fruit(fruit2);

//so fruit2 ownership is moved to function so no longer get fruit2
// println!("{}",fruit2);//gives error

print_num(x);//value copied here because x is primitive type

println!("num:{}",x);
}

//String type use heap memory
fn print_fruit(str:String){//str comes to this scope 
    println!("fruit:{}",str);
}//str goes out of scope and is dropped and memory free

fn print_num(num:i32){//value comes into this scope
println!("{}",num);
}//num value out of scope