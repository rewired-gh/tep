#[derive(Debug)]
struct Rule {
    from: String,
    to: String,
}

fn main() {
    let left = [
        ["。", "."],
        ["，", ","],
        ["；", ";"],
        ["：", ":"],
        ["？", "?"],
        ["！", "!"],
        ["……", "..."],
        ["…", "..."],
    ];
    let right = [["《", "《"], ["〈", "〈"]];
    let mut list = Vec::new();

    for i in left {
        for j in right {
            list.push(Rule {
                from: format!("{}{}", i[0], j[0]),
                to: format!("{}{} {}", i[1], k[1], j[1]),
            });
        }
    }

    println!("{:?}", list);
}
