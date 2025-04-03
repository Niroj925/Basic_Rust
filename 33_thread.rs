//thread is the smallest executable unit of process
//thread allow split the computation in our program into multiple thread
//running multiple task at the same time improve performance but increase complexity

//create thread 

use std::thread;
use std::time::Duration;
use std::sync::mpsc;

fn main(){
    //creat a thread
 let handle= thread::spawn(||{
        //everything run here in seperate thread
        for i in 1..10{
            println!("{} from spawn thread.!",i);
            thread::sleep(Duration::from_millis(2));
        }
    });

    //if we do not handle this it's execution is stop after the completion of main thread 
    handle.join().unwrap();
    println!("spawned thread completed");

    //main thread
    for i in 1..5{
        println!("{} from main thread.",i);
        thread::sleep(Duration::from_millis(1));
    }

    //the main things is when main thread complete then all other thread are shut down whether they are running

    //to complete the execution we have a concept to join handles
    // println!("join handles to incomplete execution in spawn thread");
    // handle.join().unwrap();

    //in above spawn thread we handle the return and bind to handle variable 

    //two thread continuous altering each other 
    //the main thread waits because of handle.join() and does not end until spwaned thread completed


    //move thread with clousure 
    let str=String::from("Hello Nepal");
    //move the str value to seperate thread
    // let handle_str=thread::spawn(||{//without move
    let handle_str=thread::spawn(move||{
        //use move keyword to take the ownership of str
        // variable when value move to the thread the ownership is moved to thread
        println!("{}",str);
    });
    handle_str.join().unwrap();

    //sending messages between thread

    //thread can communicate each other by sending the message through channel
    //channel is way to send the values between thread 
    //it can be used to synchronized the communication and avoid data races

    //use channel function use std::sync::mpsc module


    //create a channel
    let (sender,receiver)=mpsc::channel();

    //spawn new thread
    let hndle=thread::spawn(move||{
        //received the message
        let message=receiver.recv().unwrap();
        println!("received message:{}",message);
    });

    let msg=String::from("Namaste Nepal");
    //send message to the channel
    sender.send(msg).unwrap();

    //wait for spwaned thread to complete
    hndle.join().unwrap();

}

