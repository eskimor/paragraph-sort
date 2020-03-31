use std::cmp::Ordering;

// Sort paragraphs by their first word (until the first space character!).
// Also note that the search is case sensitive (upper case is "smaller" than lower case).
pub fn paragraph_sort(raw: &str) -> String {
    let cleaned = raw.trim_start();
    let start = &raw[0..(raw.len() - cleaned.len())];

    let input = cleaned.trim_end();
    let end = &cleaned[input.len()..];

    let mut paragraphs: Vec<&str> = input.split("\n\n").collect();
    paragraphs.sort_by(&compare_first_word);
    let sorted: String = paragraphs.join("\n\n");

    let mut start = start.to_string();
    start.reserve(sorted.len() + end.len());
    start + &sorted + end
}

fn compare_first_word(first: &&str, second: &&str) -> Ordering {
    first
        .trim_start()
        .split(" ")
        .next()
        .cmp(&second.trim_start().split(" ").next())
}

#[cfg(test)]
mod tests {
    static TEST_INPUT1: &'static str = "

      Hello

      you x

      how x

      are x   
    ";
    #[test]
    fn start_whitespace_is_preserved() {
        assert_eq!(&crate::paragraph_sort(TEST_INPUT1)[0..2], "\n\n");
    }
    #[test]
    fn end_whitespace_is_preserved() {
        let result = &crate::paragraph_sort(TEST_INPUT1);
        assert_eq!(&result[result.len() - 3..], "   ");
    }
    #[test]
    fn result_is_sorted() {
        let result = &crate::paragraph_sort(TEST_INPUT1);
        assert_eq!(
            &result
                .split("\n")
                .map(|x| x.trim())
                .filter(|x| x.len() > 0)
                .collect::<Vec<_>>()
                .join(" "),
            "Hello are x how x you x"
        );
    }
}
