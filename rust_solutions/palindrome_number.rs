/**
 * Given an integer x, return true if x is a palindrome, and false otherwise.



Example 1:

Input: x = 121
Output: true
Explanation: 121 reads as 121 from left to right and from right to left.
Example 2:

Input: x = -121
Output: false
Explanation: From left to right, it reads -121. From right to left, it becomes 121-. Therefore it is not a palindrome.
Example 3:

Input: x = 10
Output: false
Explanation: Reads 01 from right to left. Therefore it is not a palindrome.
 */

fn palindrome_number(x: i32) -> bool {
    if x > -1 && x < 10 {
        return true;
    }
    if x < 0 || x % 10 == 0 {
        return false;
    }

    let mut xx = x;
    let mut rr = 0;

    while xx > rr {
        rr = rr*10 + xx%10;
        xx = xx/10;
    }

    return xx == rr || xx == rr/10;
}
