use aoc2024::input_file;
use std::{error::Error, io::BufRead};

fn main() -> Result<(), Box<dyn Error>> {
    let filename = input_file(false);
    let mut rules: Vec<(i32, i32)> = Vec::new();
    let mut input = std::io::BufReader::new(std::fs::File::open(filename)?)
        .lines()
        .into_iter();
    for line in &mut input {
        let line = line?;
        if line.is_empty() {
            break;
        }
        let mut line = line.split("|");
        let a = line.next().unwrap().parse()?;
        let b = line.next().unwrap().parse()?;
        assert_eq!(line.next(), None);
        rules.push((a, b));
    }

    let mut orders: Vec<Vec<i32>> = Vec::new();
    for line in input {
        orders.push(line?.split(",").map(|x| x.parse().unwrap()).collect());
    }
    //println!("{:?}", rules);
    //println!("{:?}", orders);

    let mut sum = 0;
    let mut reordered_sum = 0;
    for order in orders {
        if is_ordered(&order, &rules) {
            let len = order.len();
            assert!(len % 2 == 1);
            sum += order[len / 2];
        } else {
            let new_order = reorder(&order, &rules);
            let len = new_order.len();
            assert!(len % 2 == 1);
            reordered_sum += new_order[len / 2];
        }
    }
    println!("Sum: {}", sum);
    println!("Reordered sum: {}", reordered_sum);
    Ok(())
}

fn is_ordered(order: &[i32], rules: &[(i32, i32)]) -> bool {
    for (a, b) in rules {
        let a_index = order.iter().position(|x| x == a);
        let b_index = order.iter().position(|x| x == b);
        if let (Some(a_index), Some(b_index)) = (a_index, b_index) {
            if a_index > b_index {
                // println!("order {:?} breaks rule {}|{}", order, a, b);
                return false;
            }
        } else {
            continue;
        }
    }
    return true;
}

fn reorder(order: &[i32], rules: &[(i32, i32)]) -> Vec<i32> {
    let mut correct = Vec::new();
    'orderloop: for n in order {
        for i in 0..correct.len() + 1 {
            let mut new = correct.clone();
            new.insert(i, *n);
            if is_ordered(&new, rules) {
                correct = new;
                continue 'orderloop;
            }
        }
        unreachable!("No correct order found");
    }
    return correct;
}
