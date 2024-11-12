struct Solution;

impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        if (divisor == 1) {
            return dividend;;
        }
        let min = i32::MIN;
        if (dividend == min) && (divisor == -1) {
            return i32::MAX;
        }
        let sign = (dividend > 0 && divisor > 0) || (dividend < 0 && divisor < 0);

        let mut ans = 0;
        // 都变为负数，防止溢出
        let mut a = match dividend > 0 {
            true => { -dividend }
            false => { dividend }
        };

        let mut b = match divisor > 0 {
            true => { -divisor }
            false => {divisor }
        };


        while a <= b {
            let mut x = b;
            let mut cnt = 1;
            // b*2^n
            while (a <= (x << 1)) && (x >= min >> 1) {
                // 乘以2
                x = x << 1;
                // 数量翻倍
                cnt = cnt << 1;
            }

            a -= x;
            ans += cnt;
        }

        match sign {
            true => { ans }
            false => { -ans }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::two_number_divide::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::divide(10, 3), 3);
        assert_eq!(Solution::divide(7, -3), -2);
        assert_eq!(Solution::divide(21, -2), -10);
    }
}
