pub fn is_armstrong_number(mut num: u128) -> bool {
    num == 0
        || num == {
            let len = num.ilog10() + 1;
            let mut sum = (num % 10).pow(len);

            while num > 0 {
                num /= 10;
                sum += (num % 10).pow(len);
            }

            sum
        }
}
