//generics like hashMap type 
//HashMap<K,V> key,value

//genreics or genereics type uses a single character like T,U,K,V,E 
//to disnguish from actual concret types like String ,&str,&i32 

//as a convention
//T,U are used for arbitory types
//K,V for key-value types
//E is used for Error type

use std::collections::HashMap;

fn main(){
    //create a hashmap with types i32 and str 
    let mut user:HashMap<i32,&str>=HashMap::new();//Generics used in this 

    user.insert(1,"Niroj");
    user.insert(2,"Thapa");

    println!("{:?}",user);

    //Generics struct type

    #[derive(Debug)]
    struct Point<T>{
        x:T,
        y:T,
    }

    //init generics struct type with i32
    let int_point=Point{x:4,y:6};

    //init genrics struct type with f32
    let float_point=Point{x:2.35,y:5.134};

    println!("{:?}",int_point);
    println!("{:?}",float_point);

    #[derive(Debug)]
    struct Student<T,U>{
        name:T,
        roll:U,
    }

    //init generics struct type with i32
    let student1=Student{name:"Niroj",roll:25};
    println!("{:?}",student1);

    //Genereics function
    //we can create function with generics types as parameter

    fn min<T:PartialOrd>(a:T,b:T)->T{
       if(a>b){
        return a;
       }else{
        return b;
       }
    }

    //different types as parameter using generics in function
  let res1=  min(34,32);
  let res2=  min(2.7,3.2);

  println!("{:?}",res1);
  println!("{:?}",res2);
}