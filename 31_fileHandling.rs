use std::fs::File;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;
use std::fs;

fn main(){
    //open a file in read only mode
    let data_result=File::open("content.txt");

    //result file return a enum
    //return file may be error or file

    let data_file=match data_result{
       Ok(file)=>file,
       Err(error)=>panic!("error occure {:?}",error),
    };
    println!("{:?}",data_file);

    //read a file in rust 
    //read a file in local system
    let mut file_data=File::open("content.txt").unwrap();

    //create an empty mutable string
    let mut file_content=String::new();

    //copy content of file into mutable string
    file_data.read_to_string(&mut file_content).unwrap();

    //print the read file
    println!("{:?}",file_content);

    //create and write into file
    let mut new_file=File::create("data.txt").expect("file create failed");

    //write into file
    new_file.write("Namaste Nepal".as_bytes()).expect("writing failed");

    println!("File created");

    //remove file 
    // fs::remove_file("data.txt").expect("failed to remove file");
    // println!("File has been removed");

    //append to file 

    //open file with append option
    let mut data_f=OpenOptions::new()
    .append(true)
    .open("data.txt")
    .expect("can not open in append mode");

    //write to a file 
    data_f
    .write("K xa khabar".as_bytes())
    .expect("can not add string");
println!("append content to file");
}