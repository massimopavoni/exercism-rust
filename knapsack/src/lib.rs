#[derive(Debug)]
pub struct Item {
    pub weight: u32,
    pub value: u32,
}

pub fn maximum_value(max_weight: u32, items: &[Item]) -> u32 {
    let mut best_values = vec![0; max_weight as usize + 1];
    
    for item in items {
        for weight_limit in (item.weight as usize..=max_weight as usize).rev() {
            best_values[weight_limit] = best_values[weight_limit]
                .max(best_values[weight_limit - item.weight as usize] + item.value);
        }
    }

    best_values[max_weight as usize]
}
