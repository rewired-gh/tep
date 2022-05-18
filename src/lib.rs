pub mod common;

pub use common::cook::cook;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cook_test() {
        let input = r#"《阿波罗分！〈利窍〉——你（看看！）三三，》：
“芝加：‘他说了「谱」（起立）’哥，”土气。算法……
过去？来；自己拿“好的”，年【月】～【在】！
（费）

  "#;
        let expected_trimmed_output = r#"《阿波罗分! 〈利窍〉- 你 (看看!) 三三,》:
"芝加: '他说了 '谱'(起立)' 哥," 土气. 算法...
过去? 来; 自己拿 "好的", 年 [月] ~ [在]!
(费)"#;
        assert_eq!(expected_trimmed_output, cook(input));
    }
}
