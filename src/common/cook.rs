/// Overview: Process (cook) the source (raw) content.
use super::rule::Rule;
use super::tree::Tree;

pub fn cook(raw: &str, is_trim: bool) -> String {
    let raw_chars: Vec<char> = raw.chars().collect();
    let raw_len = raw_chars.len();
    let mut cooked = String::with_capacity(2 * raw.len());
    let mut index = 0usize;

    let search_tree = Tree::from(Rule::default_rules());

    while index < raw_len {
        match search_tree.query(&raw_chars, index, raw_len) {
            Some((to, depth)) => {
                cooked.push_str(to);
                index += depth;
            }
            None => {
                cooked.push(raw_chars[index]);
                index += 1;
            }
        }
    }

    if is_trim {
        cooked
            .lines()
            .map(|line| line.trim())
            .collect::<Vec<&str>>()
            .join("\n")
            .trim()
            .to_string()
    } else {
        cooked
    }
}
