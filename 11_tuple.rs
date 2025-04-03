//tuple store different type of data
fn main(){
    //tuple has fixed size and can not grow or shrink after create
    //use () to create tuple

    //tuple without data types
    let info=("wow",6 ,"c",3.6);
    println!("{:?}",info);

//tuple with data types
let student:(&str,u8,f32)=("Thapa",6,9.25);
println!("{:?}",student);

//accessing tuple element
println!("{:?}",student.0);
println!("{:?}",student.2);

//create a mutable tuple
let mut user:(&str,u8,f32)=("Niroj",25,6.35);
user.0="Kaji";
user.2=6.36;
println!("{:?}",user);

//destructuring tuple
let (name,_age,height)=user;//age is not used so user _
println!("{}",name);
println!("{}",height);

}