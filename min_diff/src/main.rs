use std::collections::BinaryHeap;

/// `min_diff` takes two non-empty arrays of integers, finds the pair of
/// numbers (one from each array) who absolute difference is closest to zero,
/// and returns a pair containing these two numbers, with the first number from
/// array
/// 
/// Note: complexity O(NlogN)
/// 
fn min_diff(array1: &Vec<u32>, array2: &Vec<u32>) -> (u32, u32) {
    let mut heap1 = BinaryHeap::new();

    for &number in array1 {
        heap1.push(number);
    }

    let mut heap2 = BinaryHeap::new();
    for &number in array2 {
        heap2.push(number);
    }

    let mut p1 = *heap1.peek().unwrap();
    let mut p2 = *heap2.peek().unwrap();
    let mut min_diff = if p1 > p2 { p1 - p2 } else { p2 - p1 };
    while heap1.peek() != None && heap2.peek() != None {
        let &first = heap1.peek().unwrap();
        let &second = heap2.peek().unwrap();
        if first > second {
            heap1.pop();
            if first - second < min_diff {
                min_diff = first - second;
                p1 = first;
                p2 = second;
            }
        } else {
            heap2.pop();
            if second - first < min_diff {
                min_diff = second - first;
                p1 = first;
                p2 = second;
            }
        }
    }

    return (p1, p2);
}

fn main() {
    let array1 = vec![23, 5, 10, 17, 30];
    let array2 = vec![26, 134, 135, 14, 19];
    let (first, second) = min_diff(&array1, &array2);
    println!("pair = <{}, {}>", first, second);
}
