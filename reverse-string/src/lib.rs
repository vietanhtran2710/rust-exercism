use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    // input.chars().rev().collect::<String>()
    input.graphemes(true).into_iter().rev().collect()
}
