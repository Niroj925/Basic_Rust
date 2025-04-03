//rust is memeory saftey language

fn meme(){
    let x=455;
    let y=353;
    println!("{},{}",x,y);
}

fn main(){
    let z=255;
    meme();
    println!("{}",z);
//memory allocation in stack of above function in order
//z is binding to main function then others

//address is the memeory reference of RAM
//address name  value
//2       x     455
//1       y     353
//0       z     255

//after completly execute the meme() then memory deallocated 
 //0   z   255


 //rust automatically allocated and deallocated memory in and out

 //heap
 //it's opposite of the stack where memory is deallocated after execute the function 
 //but most of the time we need to pass the variable and keep alive for long time 
 //for that we can use heap
 //Box is used to allocate memory in heap

 let xx=Box::new(252);
 let yy=233;
 println!("xx={},yy={}",xx,yy);

 //memory allocation for above is
 //address name value
 //1       yy    233
 //0       xx     ??

 //xx is allocated in heap when call Box::new()
 //the actual value of x is pointer to the heap
 //now the memory looks like this 
 //address name  value
 //5487           100
 //...      ...    ...
 //1        yy    233 
 //0        xx   ->5487

 //the xx -variable hold pointer to the address ->5487
 //when xx goes away it's first frees the memory allocated on the heap
 
}
