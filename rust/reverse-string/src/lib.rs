extern crate unicode_segmentation;

use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    // todo!("Write a function to reverse {input}");

    // reverse string
    // Result::expect(
        // String::from_str(
            input
                .graphemes(true)
                .to_owned()
                .rfold(String::from(""), |prev, next| {
                    prev.to_owned() + next
                })
        // ),
    //     "invalid string",
    // )
}
