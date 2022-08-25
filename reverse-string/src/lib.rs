// use unicode_reverse::reverse_grapheme_clusters_in_place;
use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    input.graphemes(true).rev().collect()
    // reverse_grapheme_clusters_in_place(&input);
    // input.chars().rev().collect::<String>()
    // unimplemented!("Write a function to reverse {}", input);
}
