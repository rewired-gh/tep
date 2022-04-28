/// Overview: Rule model and default rules set.
use std::collections::HashSet;
use structopt::lazy_static::lazy_static;

pub struct Rule {
    pub from: &'static str,
    pub to: &'static str,
}

impl Rule {
    pub fn default_rules() -> Vec<Self> {
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
            // S2
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
            // S3
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
            // S4
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
            // S5
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

lazy_static! {
    pub static ref SOFT_MARKS: HashSet<char> = HashSet::from(['.', ',', ';', ':', '?', '!', '-']);
    pub static ref BREAKS: HashSet<char> = HashSet::from(['\n']);
    pub static ref HARD_MARKS: HashSet<char> =
        HashSet::from(['\'', '"', '(', ')', '{', '}', '[', ']', '《', '》', '〈', '〉',]);
}
