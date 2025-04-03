//trait defines the share functionality for multiple types
//trait promote type-safty,prevent error at compile time
//act like interface in anoter language with some diff

//define traits 
trait MyTrait{
    fn method_one(&self);
    fn method_two(&mut self,arg:i32)->bool;
}

//we define MyTrait with two method signature 
//method signature defines the types hehaviour which implement this traits

//define trait Printable
trait Printable{
    fn print(&self);
}
//define struct to implement trait

struct Person{
    name:String,
    age:i32,
}

//implement trait Printable for Person struct
impl Printable for Person{
    fn print(&self){
        println!("Person{{name:{},age:{}}}",self.name,self.age);
    }
}

struct Car{
    brand:String,
    model:String,
}

impl Printable for Car{
    fn print(&self){
        println!("Car{{brand:{},model:{}}}",self.brand,self.model);
    }
}

//utility function to print any object that implement Printable trait
fn print_thing<T:Printable>(thing:&T){//it's a generic function that can accept reference to any object that implement Printable trait
    thing.print();
}


fn main(){
    //Instantialte Person and Car s
    let person=Person {name:"Niroj".to_string(),age:24};
    let car=Car{brand:"Tesla".to_string(),model:"Model3".to_string()};

    //call print things with reference of Person and Car
    print_thing(&person);
    print_thing(&car);
}

///in above we defined Printable trait for Struct Person and Car
/// Printable trait requires a print method for implementor 

//derive keyword 
//use derive keyword to implement copy and clone
#[derive(Copy,Clone)]
struct MyStruct{
    value:i32,
}

fn jptfn(){
    let x=MyStruct{value:54};
    let y=x;
    println!("{:?}",x);
    println!("{:?}",y);
}