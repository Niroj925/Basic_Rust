//struct are the different type of data

struct Person{
    name:String,
    age:u8,
    height:f32
}

fn main(){
//create an instance of Person
let p1=Person{
    //name:"niroj"//this is &str type and it is fixed and immutable
    //string literal can not be directly assigned 
    name:String::from("niroj"),//create a string and head allocate memory and growable
    age:23,
    height:6.4
};
println!("{}",p1.name);


//create mutable person struct instance
let mut p2=Person{
    name:String::from("thapa"),//create a string and head allocate memory and growable
    age:20,
    height:5.4
};

p2.age=24;
println!("{}",p2.age);

}