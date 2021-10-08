// 1710. Maximum Units on a Truck, Easy
// https://leetcode.com/problems/maximum-units-on-a-truck/
impl Solution {
    pub fn maximum_units(mut box_types: Vec<Vec<i32>>, mut truck_size: i32) -> i32 {
        box_types.sort_unstable_by(|a, b| b[1].cmp(&a[1]));

        let mut loaded_boxes = 0;

        for box_type in box_types {
            if truck_size > box_type[0] {
                truck_size -= box_type[0];
                loaded_boxes += box_type[0] * box_type[1];
            } else {
                loaded_boxes += truck_size * box_type[1];
                return loaded_boxes;
            }
        }

        loaded_boxes
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_vec_i32;

    #[test]
    fn test_maximum_units() {
        assert_eq!(Solution::maximum_units(vec_vec_i32![[1, 3], [2, 2], [3, 1]], 4), 8);
    }

    #[test]
    fn test_maximum_units2() {
        assert_eq!(Solution::maximum_units(vec_vec_i32![[5, 10], [2, 5], [4, 7], [3, 9]], 10), 91);
    }
}
