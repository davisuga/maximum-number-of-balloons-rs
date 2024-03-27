// Given a string text, you want to use the characters of text to form as many instances of the word "balloon" as possible.

// You can use each character in text at most once. Return the maximum number of instances that can be formed.

// Example 1:

// Input: text = "nlaebolko"
// Output: 1
// Example 2:

// Input: text = "loonbalxballpoon"
// Output: 2
// Example 3:

// Input: text = "leetcode"
// Output: 0

// Constraints:

// 1 <= text.length <= 104
// text consists of lower case English letters only.

impl Solution {
    pub fn max_number_of_balloons(text: String) -> i32 {
        let mut state = [0; 5];
        let mut num_words = 0;
        &text.chars().for_each(|char| match &char {
            'b' => state[0] += 1,
            'a' => state[1] += 1,
            'l' => state[2] += 1,
            'o' => state[3] += 1,
            'n' => state[4] += 1,
            _ => {}
        });
        state[2] = state[2] / 2;
        state[3] = state[3] / 2;
        *state
            .iter()
            .reduce(|acc, current| if acc < current { acc } else { current })
            .unwrap_or(&0)
    }
}
