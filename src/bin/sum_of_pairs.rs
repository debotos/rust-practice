/* https://www.codewars.com/kata/54d81488b981293527000c8f */

use std::collections::HashMap;

const LIST: [i32; 6] = [10, 5, 2, 3, 7, 5];
const SUM: i32 = 10;

fn main() {
    let mut results: HashMap<usize, [i32; 2]> = HashMap::new();
    let mut outer_loop_count = 1;

    while outer_loop_count <= LIST.len() {
        let mut inner_loop_count: usize = outer_loop_count;
        let current_item = LIST[outer_loop_count - 1];

        while inner_loop_count < LIST.len() {
            let next_item = LIST[inner_loop_count];
            let is_ok = SUM == current_item + next_item;
            println!(
                "Checking {} + {} == {} | {:?}",
                current_item, next_item, SUM, is_ok
            );
            if is_ok {
                results.insert(inner_loop_count, [current_item, next_item]);
            }

            inner_loop_count += 1;
        }
        println!("");

        outer_loop_count += 1;
    }

    if results.is_empty() {
        println!("No result.");
    } else if results.len() > 1 {
        println!("\nAll Results:\n{:?}", results);
        let min_index = results.keys().min();
        if let Some(index) = min_index {
            println!("\n\nFinal Result:");
            if let Some(result) = results.get(index) {
                println!("{:?}", result);
            } else {
                println!("No result.");
            }
        } else {
            println!("No result.");
        }
    } else {
        println!("\n\nFinal Result:");
        for result in results.values() {
            println!("{:?}", result);
        }
    }
}
