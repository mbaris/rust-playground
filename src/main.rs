fn main() {
    
    let mut n:i16 = 0;

    loop {
        if n == 95{
            n += 1;
            continue;
        }
        println!("n is {}",n);
        

        n += 1;
        

        if n > 100{
            break;
        }
    }

    println!("finished loop");

    while n < 200 {
        n+=1;
        println!("n is {}",n );
    }

    println!("finished while");

}