// 1570. Dot Product of Two Sparse Vectors, Medium
// https://leetcode.com/problems/dot-product-of-two-sparse-vectors/
impl SparseVector {
    fn new(nums: Vec<i32>) -> Self {
        SparseVector { nums }
    }

    fn dot_product(&self, vec: SparseVector) -> i32 {
        let mut result = 0;

        for i in 0..self.nums.len() {
            result += self.nums[i] * vec.nums[i];
        }

        result
    }
}

struct SparseVector {
    nums: Vec<i32>,
}
