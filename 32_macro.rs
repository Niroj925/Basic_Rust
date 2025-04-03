//macro is a piece of code that generate another code 
//macro generate the code based on input,simplify repetative code,make code more concise

//popular macro are println!,vec!,panic! etc 

//create a macro
macro_rules! hello_nepal {
    //() this macro does not take any argument
    //macro rules defines inside this block 
    ()=>{
        //macro will expand into the content in this block
        println!("Namaste Nepal");
    }
}

//creating macro with argument
macro_rules! print_message{
    //match the rules that take an argument expression
    ($message:expr)=>{//take argument with $ prefix and after semicolon : which is called designator (expr)
        println!("{}",$message);
    }

    // There are many designators that we can use inside a macro rule body:
    // stmt: a statement
    // pat: a pattern
    // expr: an expression
    // ty: a type
    // ident: an identifier ...
}

//macro repeatation
macro_rules! repeat_print{
    //match the rule which match the multiple expression as an argument
    ($($msg:expr),*)=>{
        $(
        println!("{}",$msg);
        )*
    }
}

fn main(){
    //this call will expand into println!("Namaste Nepal")
    hello_nepal!();

    //call macro with argument
    print_message!("Welcome to Nepal");

    //call maro with multiple argument
    repeat_print!(1,"hello",3);
}