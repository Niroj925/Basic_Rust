//rust hashmap data structure store the key-value pairs
//features->each key is assiociated with value,key is unique but value can duplicate,value can access by their key

//hashmap is a standard part of collection library

//import from collection
use std::collections::HashMap;

fn main(){
    //creating hashmap
    let mut student:HashMap<i32,String>=HashMap::new();
    println!("{:?}",student);//empty hashMap

    //HashMap operation

    //1.add elements to hashMap
    student.insert(1,String::from("Niroj"));
    student.insert(2,String::from("Thapa"));
    student.insert(3,String::from("Bishwo"));

    println!("{:?}",student);

    //2.access the value
    println!("first:{:?}",student.get(&1));//this return the reference value not actual value
    println!("first:{:?}",student.get(&3));//return none

    
    //3.update hashmap element
    student.insert(1,String::from("Nirajan"));
    
    //4.remove element from hashmap
    student.remove(&2);
    println!("{:?}",student);

    //other methods
    //loop over hashmap
     //keys
     for name in student.keys(){
        println!("{}",name);
    } 

    //values
    for student in student.values(){
        println!("Student Name:{}",student);
    };



    //len()
    println!("{}",student.len());

    //contains_key()
    if student.contains_key(&2){
        println!("student exist");
    }else{
        println!("student does not exist");
    }

    //iter
    for (roll,name) in student.iter(){
        println!("{}:{}",roll,name);
    }

   
}