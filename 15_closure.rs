fn main(){
//clousure are the function without name 
let name=||println!("Danda bdr dangal");

//call the clousure
name();

let sum=|a:i32,b:i32|println!("sum:{}",a+b);
sum(4,3);

//multi line closure
let square_sum=|a:i32,b:i32|{
    let mut sum=a+b;

    let mut square=sum*sum;
    return square;
};
println!("sum and square:{}",square_sum(4,3));

//clousure environment capturing 
let age=16;
let showage=||println!("age:{}",age);
showage();

let name=String::from("Hamro");
//1.variable is not modified inside clousure
//immutable clousure
let print_name=|| {
    println!("name:{}",name);
};

//immutable borrow is possible outside clousure
println!("lenght of word:{},",name.len());
print_name();

//2.variable modified inside clousure
//mutable clousure
let mut word=String::from("Hamro");
let mut print_str=||{
    word.push_str(" Nepal.!!");
    println!("{}",word);
};

//can not immutable borrow because the varibale is borrow as mutable inside closure
// println!("length of word={}",name.len());

print_str();

//can immutable borrow because closure has already used
println!("word len:{}",word.len());


//the main difference between function and closure is that closure can capture value (environment capturing)

fn print_age(){
    println!("Namaste gaich.");
    // println!("your age:{}",age);//function can not capture the value from outside
}

let print_age_c=||println!("Your age inside closure:{}",age);//closure can capture the value from outside too

print_age();
print_age_c();

}