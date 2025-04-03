fn main(){
    //there are 4 types of operator 

    //arthmetic operator 
    let mut a=12;
    let b=4;

    let add=a+b;
    println!("{add}");

    let sub=a-b;
    println!("{sub}");

    let mul=a*b;
    println!("{mul}");
    
    a=15;//= is asssignment operator
    
    let div=a as f32/b as f32;
    println!("{div}");

    let x=13.0;
    let y=7.0;
    let div_val=x/y;
    println!("{div_val}");
    
    let remainder=a%b;
    println!("{remainder}");

   //compound assignment operator
   a+=3;
   println!("{a}");

   a*=2;
   println!("{a}");


   //comparision operator < > ==
   let c=a>b;
    println!("{a}>{b} is {c} ");

   //logical operator &&, ||, !
   let is_valid=(a==b )|| (b<10);
   println!("{is_valid}");
   
}
