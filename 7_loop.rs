fn main() {
        let mut count=0;
    // infinite loop
        loop{
            count +=1;
            if count==3{
                continue;
            }
            println!("{count}");
            if count >=5{
                break;
            }
        }

    //while loop
    println!("multiplication table:");
    let mut i=1;
    while i<=5{
        let mut j=1;
        println!("multiplication of {i}.");
        while j<5{
            let mul=i*j;
            println!("{i}*{j}={mul}");
            j+=1;
        }
        i+=1;
    }

    //for in loop

    let mut sum=0;

    //sum of natural number 1 to 10
    for i in 1..11{
        sum+=i;
    }
    println!("sum:{sum}");

}
