//types of error 
//1.Unrecoverable //this stop the program
//2.Recoverable

use std::fs::File;

fn main(){
    println!("Hello Nepal.");

    //unrecoverable error
    //explicitely exit the program with unrecoverable error
    // panic!("Crash");

    let nums=[1,2,3,4];
    // println!("{}",nums[5]);

    //Recoverable error 
    //this error can not execution of program

    let file=File::open("content.txt");

    //1.Result<T,E> enum type
    //data file is Result<T,E>
    let data_file=match file{
        Ok(file)=>file,
        Err(err)=>panic!("error occure {:?}",err),
    };
   
   println!("{:?}",data_file);

   //2.Option <T> type
   let text="Hamro Nepal";
let char_option=text.chars().nth(10);

let char=match char_option{
    None=>"empty".to_string(),
    Some(c)=>c.to_string()
};

println!("character at 10 is {}.",char);

//difference between Option and Result
}