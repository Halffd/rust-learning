struct Solution;

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;

        let init = match n % 4 {
            0 => n,
            1 => 1,
            2 => n + 1,
            _ => 0,
        } ^ 0;

        nums.into_iter().fold(init, |acc, e| acc ^ e)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn e1() {
        let input = vec![3, 0, 1];
        assert_eq!(Solution::missing_number(input), 2);
    }

    #[test]
    fn e2() {
        let input = vec![0, 1];
        assert_eq!(Solution::missing_number(input), 2);
    }

    #[test]
    fn e3() {
        let input = vec![9, 6, 4, 2, 3, 5, 7, 0, 1];
        assert_eq!(Solution::missing_number(input), 8);
    }

    #[test]
    fn within_constraints() {
        let n = 10000;
        let input = (0..n).collect();
        assert_eq!(Solution::missing_number(input), n);
    }

    #[test]
    fn overflow_safe() {
        let n = i32::MAX / 2;
        let input = (0..n).collect();
        assert_eq!(Solution::missing_number(input), n);
    }
}

fn main() {
    // Example usage
    let input = vec![3, 0, 1];
    let missing = Solution::missing_number(input);
    println!("The missing number is: {}", missing);
}
