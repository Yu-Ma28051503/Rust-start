fn main() {
    let mut count = 0;

    'counting_up: loop {
        println!("count: {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }

            remaining -= 1;
        }

        count += 1;
    }

    println!("End count: {}", count);
    println!("Hello, world!");

    /* while loop */
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    println!("LIFTOFF!!!");

    /* for loop */
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("a: {}", element);
    }

    for index in (0..5).rev() {
        println!("a[{}]: {}", index, a[index]);
    }

    for num in (1..4).rev() {
        println!("{}!", num);
    }

    println!("LIFTOFF!!!");
}
