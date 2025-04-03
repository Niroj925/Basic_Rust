fn main(){
//outer variable scope whole in main function
    let outer_var=45;

    {
       let inner_var=41;
       println!("inner_var:{}",inner_var);
       println!("outer_var:{}",outer_var);
    }

    // println!("{}",inner_var);//this inner block scope variable can not be access from outside
    println!("{}",outer_var);

    //variable shadowing 
    let var1=35;
    {
        println!("{}",var1);

        //this declaration shadow the outer var1 variable
        let var1="Nepal";
        println!("{}",var1);
    }
    println!("{}",var1);

    //variable freezing
    let mut age=22;
    {
        //shadow mutable variable  with immutable variable age;
        let age=age;

        //this gives error age varaible is freez in this scope
        // age=23;

        println!("age in inner block:{}",age);//age variable goes out of scope

    }

    age=23;//in this scope is not freez in outer scope
    println!("age:{}",age);

}