// 1013. Partition Array Into Three Parts With Equal Sum, Easy
// https://leetcode.com/problems/partition-array-into-three-parts-with-equal-sum/
impl Solution {
    pub fn can_three_parts_equal_sum(arr: Vec<i32>) -> bool {
        let sum = arr.iter().sum::<i32>();

        if sum % 3 != 0 {
            return false;
        }

        let target = sum / 3;
        let mut buckets = [0, 0, 0];
        let mut buckets_size = [0, 0, 0];
        let mut i = 0;
        for num in arr.iter() {
            buckets[i % 3] += num;
            buckets_size[i % 3] += 1;
            if buckets[i % 3] == target {
                i += 1;
            }
        }
        println!("{:?}", buckets);
        buckets_size[0] > 0 && buckets_size[1] > 0 && buckets_size[2] > 0 && buckets[0] == buckets[1] && buckets[1] == buckets[2]
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_three_parts_equal_sum() {
        assert_eq!(Solution::can_three_parts_equal_sum(vec![0, 2, 1, -6, 6, -7, 9, 1, 2, 0, 1]), true);
    }

    #[test]
    fn test_can_three_parts_equal_sum2() {
        assert_eq!(Solution::can_three_parts_equal_sum(vec![0, 2, 1, -6, 6, -7, 9, 1, 2, 0, 1]), true);
    }

    #[test]
    fn test_can_three_parts_equal_sum3() {
        assert_eq!(Solution::can_three_parts_equal_sum(vec![0, 2, 1, -6, 6, -7, 9, 1, 2, 0, 1]), true);
    }

    #[test]
    fn test_can_three_parts_equal_sum4() {
        assert_eq!(Solution::can_three_parts_equal_sum(vec![1, -1, 1, -1]), false);
    }
}
