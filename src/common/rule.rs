/// Overview: Rule model and default rules set.
pub struct Rule {
    pub from: &'static str,
    pub to: &'static str,
}

impl Rule {
    pub fn default_rules() -> Vec<Rule> {
        Vec::from([
            // S1
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
                from: "…”",
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
                from: "…’",
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
                from: "…」",
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
                from: "…』",
                to: "...\" ",
            },
            Rule {
                from: "。）",
                to: ".) ",
            },
            Rule {
                from: "，）",
                to: ",) ",
            },
            Rule {
                from: "？）",
                to: "?) ",
            },
            Rule {
                from: "！）",
                to: "!) ",
            },
            Rule {
                from: "；）",
                to: ";) ",
            },
            Rule {
                from: "……）",
                to: "...) ",
            },
            Rule {
                from: "…）",
                to: "...) ",
            },
            Rule {
                from: "。】",
                to: ".] ",
            },
            Rule {
                from: "，】",
                to: ",] ",
            },
            Rule {
                from: "？】",
                to: "?] ",
            },
            Rule {
                from: "！】",
                to: "!] ",
            },
            Rule {
                from: "；】",
                to: ";] ",
            },
            Rule {
                from: "……】",
                to: "...] ",
            },
            Rule {
                from: "…】",
                to: "...] ",
            },
            // S3
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
                from: "（",
                to: " (",
            },
            Rule {
                from: "【",
                to: " [",
            },
            // S4
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
                from: "）",
                to: ") ",
            },
            Rule {
                from: "】",
                to: "] ",
            },
            // S5
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
                from: "…",
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
            Rule {
                from: "—",
                to: " - ",
            },
            Rule {
                from: "～",
                to: "~",
            },
            // S6
            Rule {
                from: "。》",
                to: ".》",
            },
            Rule {
                from: "，》",
                to: ",》",
            },
            Rule {
                from: "？》",
                to: "?》",
            },
            Rule {
                from: "！》",
                to: "!》",
            },
            Rule {
                from: "；》",
                to: ";》",
            },
            Rule {
                from: "……》",
                to: "...》",
            },
            Rule {
                from: "…》",
                to: "...》",
            },
            Rule {
                from: "。〉",
                to: ".〉",
            },
            Rule {
                from: "，〉",
                to: ",〉",
            },
            Rule {
                from: "？〉",
                to: "?〉",
            },
            Rule {
                from: "！〉",
                to: "!〉",
            },
            Rule {
                from: "；〉",
                to: ";〉",
            },
            Rule {
                from: "……〉",
                to: "...〉",
            },
            Rule {
                from: "…〉",
                to: "...〉",
            },
        ])
    }
}
