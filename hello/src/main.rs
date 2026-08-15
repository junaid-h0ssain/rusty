
fn main() {
    println!("Hello, nana!");
    
    let number: i32 = 69;
    println!("{number} is nice");

    let numbers: [i32;5] = [1,2,3,4,5];
    println!(" here is your numbers : {:?}", numbers);

    let human: (&str, i32, bool) = ("Alice", 27, false);
    println!("Human : {:?}", human);

    let slice: &[i32] = &[1,2,3];
    println!("sliced {:?}", slice);

    let fruits: [&str; 3] = ["apple", "banana", "cherry"];
    println!("fruits: {:?}", fruits);

    let mut counter = 0;
    loop {
        counter += 1;
        println!("counter: {}", counter);
        if counter == 10 {
            break;
        }
    } 

    let mut counter = 0;
    while counter < 10 {
        counter += 1;
        println!("counter: {}", counter);
    }

    let mut counter = 0;
    loop {
        counter += 1;
        println!("counter: {}", counter);
        if counter == 10 {
            break;
        }
    }

    for fruit in fruits {
        println!("fruit: {}", fruit);
    }

    for number in (0..10).rev() {
        println!("number: {}", number);
    }

    for number in 0..10 {
        println!("number: {}", number);

        if number == 5 {
            break;
        }
    }
    println!("done");

    let mut counter = 0;
    while counter < 10 {
        counter += 1;
        println!("counter: {}", counter);
        if counter == 5 {
            continue;
        }
    }
    println!("done");   

    let mut counter = 0;
    while counter < 10 {
        counter += 1;
        println!("counter: {}", counter);
        if counter == 5 {
            break;
        }
    }
    println!("done");   
}
