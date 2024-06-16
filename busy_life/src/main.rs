/// `busy_life` finds maximum number of non-overlapping ranges
/// 
/// Note: Complexity O(N * logN)
/// 
fn busy_life(activities: &mut Vec<(u32, u32)>) -> u32 {
    activities.sort_by(|first, second| first.1.cmp(&second.1));
    let mut first = activities[0];
    let mut max_activities = 0;
    for i in 1..activities.len() {
        if activities[i].0 >= first.0 {
            max_activities += 1;
            first = activities[i];
        }
    }

    return max_activities;
}

fn main() {
    let mut activities = vec![(7, 9), (0, 10), (4, 5), (8, 9), (4, 10), (5, 7)];

    println!("max activities = {}", busy_life(&mut activities));
}
