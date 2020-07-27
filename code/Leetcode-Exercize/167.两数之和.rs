use std::collections::BTreeMap;
struct Solution;
impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: BTreeMap<i32, usize> = BTreeMap::new();
        let mut r: Vec<i32> = vec![-1; 2];
        for (i, &v) in numbers.iter().enumerate() {
            let key = &(target - v);
            if map.contains_key(key) {
                let j = *map.get(key).unwrap();
                r[0] = j as i32 + 1;
                r[1] = i as i32 + 1;
                break;
            }
            map.insert(v, i);
        }
        return r;
    }
}
fn main() {}
