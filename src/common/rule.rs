/// Overview: Rule model and default rules set.
pub struct Rule {
    pub from: &'static str,
    pub to: &'static str,
}

impl Rule {
    pub fn default_rules() -> Vec<Rule> {
        Vec::from([
            Rule {
                from: "。”，",
                to: ".\", ",
            },
            Rule {
                from: "，”，",
                to: ",\", ",
            },
            Rule {
                from: "？”，",
                to: "?\", ",
            },
            Rule {
                from: "！”，",
                to: "!\", ",
            },
            Rule {
                from: "；”，",
                to: ";\", ",
            },
            Rule {
                from: "……”，",
                to: "...\", ",
            },
            Rule {
                from: "。’，",
                to: ".', ",
            },
            Rule {
                from: "，’，",
                to: ",', ",
            },
            Rule {
                from: "？’，",
                to: "?', ",
            },
            Rule {
                from: "！’，",
                to: "!', ",
            },
            Rule {
                from: "；’，",
                to: ";', ",
            },
            Rule {
                from: "……’，",
                to: "...', ",
            },
            Rule {
                from: "。」，",
                to: ".', ",
            },
            Rule {
                from: "，」，",
                to: ",', ",
            },
            Rule {
                from: "？」，",
                to: "?', ",
            },
            Rule {
                from: "！」，",
                to: "!', ",
            },
            Rule {
                from: "；」，",
                to: ";', ",
            },
            Rule {
                from: "……」，",
                to: "...', ",
            },
            Rule {
                from: "。』，",
                to: ".\" ",
            },
            Rule {
                from: "，』，",
                to: ",\" ",
            },
            Rule {
                from: "？』，",
                to: "?\" ",
            },
            Rule {
                from: "！』，",
                to: "!\" ",
            },
            Rule {
                from: "；』，",
                to: ";\" ",
            },
            Rule {
                from: "……』，",
                to: "...\" ",
            },
            Rule {
                from: "。”",
                to: ".\" ",
            },
            Rule {
                from: "，”",
                to: ",\" ",
            },
            Rule {
                from: "？”",
                to: "?\" ",
            },
            Rule {
                from: "！”",
                to: "!\" ",
            },
            Rule {
                from: "；”",
                to: ";\" ",
            },
            Rule {
                from: "……”",
                to: "...\" ",
            },
            Rule {
                from: "。’",
                to: ".' ",
            },
            Rule {
                from: "，’",
                to: ",' ",
            },
            Rule {
                from: "？’",
                to: "?' ",
            },
            Rule {
                from: "！’",
                to: "!' ",
            },
            Rule {
                from: "；’",
                to: ";' ",
            },
            Rule {
                from: "……’",
                to: "...' ",
            },
            Rule {
                from: "。」",
                to: ".' ",
            },
            Rule {
                from: "，」",
                to: ",' ",
            },
            Rule {
                from: "？」",
                to: "?' ",
            },
            Rule {
                from: "！」",
                to: "!' ",
            },
            Rule {
                from: "；」",
                to: ";' ",
            },
            Rule {
                from: "……」",
                to: "...' ",
            },
            Rule {
                from: "。』",
                to: ".\" ",
            },
            Rule {
                from: "，』",
                to: ",\" ",
            },
            Rule {
                from: "？』",
                to: "?\" ",
            },
            Rule {
                from: "！』",
                to: "!\" ",
            },
            Rule {
                from: "；』",
                to: ";\" ",
            },
            Rule {
                from: "……』",
                to: "...\" ",
            },
            Rule {
                from: "。“",
                to: ". \"",
            },
            Rule {
                from: "，“",
                to: ", \"",
            },
            Rule {
                from: "？“",
                to: "? \"",
            },
            Rule {
                from: "！“",
                to: "! \"",
            },
            Rule {
                from: "；“",
                to: "; \"",
            },
            Rule {
                from: "……“",
                to: "... \"",
            },
            Rule {
                from: "\n“",
                to: "\n\"",
            },
            Rule {
                from: "。‘",
                to: ". '",
            },
            Rule {
                from: "，‘",
                to: ", '",
            },
            Rule {
                from: "？‘",
                to: "? '",
            },
            Rule {
                from: "！‘",
                to: "! '",
            },
            Rule {
                from: "；‘",
                to: "; '",
            },
            Rule {
                from: "……‘",
                to: "... '",
            },
            Rule {
                from: "\n‘",
                to: "\n'",
            },
            Rule {
                from: "。『",
                to: ". \"",
            },
            Rule {
                from: "，『",
                to: ", \"",
            },
            Rule {
                from: "？『",
                to: "? \"",
            },
            Rule {
                from: "！『",
                to: "! \"",
            },
            Rule {
                from: "；『",
                to: "; \"",
            },
            Rule {
                from: "……『",
                to: "... \"",
            },
            Rule {
                from: "\n『",
                to: "\n\"",
            },
            Rule {
                from: "。「",
                to: ". '",
            },
            Rule {
                from: "，「",
                to: ", '",
            },
            Rule {
                from: "？「",
                to: "? '",
            },
            Rule {
                from: "！「",
                to: "! '",
            },
            Rule {
                from: "；「",
                to: "; '",
            },
            Rule {
                from: "……「",
                to: "... '",
            },
            Rule {
                from: "\n「",
                to: "\n'",
            },
            Rule {
                from: "“",
                to: " \"",
            },
            Rule {
                from: "‘",
                to: " '",
            },
            Rule {
                from: "「",
                to: " '",
            },
            Rule {
                from: "『",
                to: " \"",
            },
            Rule {
                from: "”",
                to: "\" ",
            },
            Rule {
                from: "’",
                to: "' ",
            },
            Rule {
                from: "」",
                to: "' ",
            },
            Rule {
                from: "』",
                to: "\" ",
            },
            Rule {
                from: "（",
                to: " (",
            },
            Rule {
                from: "）",
                to: ") ",
            },
            Rule {
                from: "【",
                to: " [",
            },
            Rule {
                from: "】",
                to: "] ",
            },
            Rule {
                from: "——",
                to: " - ",
            },
            Rule {
                from: "。",
                to: ". ",
            },
            Rule {
                from: "，",
                to: ", ",
            },
            Rule {
                from: "？",
                to: "? ",
            },
            Rule {
                from: "！",
                to: "! ",
            },
            Rule {
                from: "；",
                to: "; ",
            },
            Rule {
                from: "……",
                to: "... ",
            },
            Rule {
                from: "：",
                to: ": ",
            },
            Rule {
                from: "、",
                to: ", ",
            },
        ])
    }
}
