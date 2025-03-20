fn main() {
    println!("{:?}", two_sum(vec![3, 3], 9));
}
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut returning_indexes: Vec<i32> = Vec::new();

    for index in 0..nums.len() {
        for (values_index, values) in nums.iter().enumerate() {
            // Aynı indekse sahip öğeleri toplama
            if index != values_index {
                let sum = nums[index] + values;
                if sum == target {
                    returning_indexes.push(index as i32);
                    returning_indexes.push(values_index as i32);
                    return returning_indexes; // Sonucu bulunca hemen döndür
                }
            }
        }
    }
    returning_indexes
}
