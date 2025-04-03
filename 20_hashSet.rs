//it is just like set where values can not be duplicate

use std::collections::HashSet;

fn main(){
    let mut colors:HashSet<&str>=HashSet::new();

    //add 
    colors.insert("red");
    colors.insert("green");
    colors.insert("blue");
    colors.insert("blue");//this could not insert this already include

    println!("{:?}",colors);


    //check for value in hashSet

    if colors.contains("red"){
        println!("we have color red in color hashSet");
    }

    colors.remove("green");
    println!("{:?}",colors);

    for color in colors{
        println!("{}",color);
    }

    //create HashSet with defaulr value
    let mut age:HashSet<i32>=HashSet::from([6,2,8]);
    println!("{:?}",age);

    //other method
    println!("{}",age.len());

    if age.is_empty(){
        println!("empty age hashset");
    }else{
        println!("age hashset is not empty");
    }

    //return all iterator and clear
//   for age in  age.drain(){
//       println!("{}",age);
//   };
//   println!("{:?}",age);//return {} because of drain

//     //remove all element from hashset
//     age.clear();
//     println!("{:?}",age);

let mut roll:HashSet<i32>=HashSet::from([1,2,3]);

println!("age:{:?}",age);
println!("roll:{:?}",roll);

//union operation of two sets
let union_res:HashSet<_>=age.union(&roll).collect();
println!("union:{:?}",union_res);

//intersection operation
let intersec:HashSet<_>=age.intersection(&roll).collect();
println!("intersection:{:?}",intersec);

//differemce
let diff:HashSet<_>=age.difference(&roll).collect();
println!("difference(age,roll):{:?}",diff);

//symmetric differemce
let sym_diff:HashSet<_>=age.symmetric_difference(&roll).collect();
println!("symmetric difference(age,roll):{:?}",sym_diff);

}