// a simple Rust program for the collatz conjecture problem

fn main() {
    let mut counter: u128 = 1;
    let mut result :u128 = 1;
    loop {
        if result % 2 == 0 {
            result /= 2;
        } else {
            result *= 3;
            result += 1;
        }

        if result == 1 {
            println!("{counter}");
            counter += 1;
            result = counter;
        }

        if counter == std::u128::MAX {
            break;
        }
    }
}

