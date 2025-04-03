fn greet(){
    println!("Namaste Nepal.");
}

fn add(a:i32,b:i32)->i32{
    let sum=a+b;
    println!("sum:{}",sum);
    return sum;
}

//return multiple values
fn addsum(a:i32,b:i32)->(i32,i32){
    return (a+b,a-b);
}

//pass by reference
fn calclen(s:&String)->usize{
return s.len();
}

fn main(){
    greet();
 let sum= add(4,5);
 println!("{}",sum);

 let (sum,sub)=addsum(5,12);
 println!("sum:{},diff:{}",sum,sub);

 //pass by reference
 let word=String::from("Nepal");
 let len=calclen(&word);
 println!("word:{},length:{}",word,len);
}