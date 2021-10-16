// 302. Smallest Rectangle Enclosing Black Pixels, Hard
// https://leetcode.com/problems/smallest-rectangle-enclosing-black-pixels/
impl Solution {
    pub fn min_area(image: Vec<Vec<char>>, x: i32, y: i32) -> i32 {
        let [mut min_x, mut max_x, mut min_y, mut max_y] = [i32::MAX, i32::MIN, i32::MAX, i32::MIN];

        fn bfs(image: &mut Vec<Vec<char>>, x: i32, y: i32, min_x: &mut i32, max_x: &mut i32, min_y: &mut i32, max_y: &mut i32) {
            let [n, m] = [image.len() as i32, image[0].len() as i32];

            if x < 0 || x >= n || y < 0 || y >= m {
                return;
            }

            let directions = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];

            if image[x as usize][y as usize] == '1' {
                *min_x = i32::min(*min_x, x);
                *max_x = i32::max(*max_x, x);
                *min_y = i32::min(*min_y, y);
                *max_y = i32::max(*max_y, y);

                image[x as usize][y as usize] = '0';

                for &(dx, dy) in directions.iter() {
                    bfs(image, x + dx, y + dy, min_x, max_x, min_y, max_y);
                }
            }
        }

        let mut image = image;
        bfs(&mut image, x, y, &mut min_x, &mut max_x, &mut min_y, &mut max_y);

        (max_x - min_x + 1) * (max_y - min_y + 1) as i32
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_vec_char;

    #[test]
    fn test_island_perimeter() {
        assert_eq!(Solution::min_area(vec_vec_char![['0', '0', '1', '0'], ['0', '1', '1', '0'], ['0', '1', '0', '0']], 0, 2), 6);
    }
}
