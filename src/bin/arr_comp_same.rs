/* https://www.codewars.com/kata/550498447451fbbd7600041c */

use num::integer::Roots;

const ARR1: [i32; 8] = [121, 144, 19, 161, 19, 144, 19, 11];
const ARR2: [i32; 8] = [121, 14641, 20736, 361, 25921, 361, 20736, 361];

fn checker() -> bool {
    let mut is_ok = false;

    for item in ARR1 {
        if ARR2.contains(&(item * item)) {
            is_ok = true;
        } else {
            is_ok = false;
            break;
        }
    }

    if is_ok {
        for item in ARR2 {
            if ARR1.contains(&(item.sqrt())) {
                is_ok = true;
            } else {
                is_ok = false;
                break;
            }
        }
    }

    is_ok
}

fn main() {
    println!("{}", checker());
}
