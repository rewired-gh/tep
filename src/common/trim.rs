/// Overview: Trim redundant empty characters.
use super::rule;

enum TrimState {
    Idle,
    Ready(TrimKind),
    Trimming(TrimKind),
    WhitespaceUnknown,
}

#[derive(Copy, Clone)]
enum TrimKind {
    Soft,
    Hard,
    Break,
}

pub fn trim(raw: &str) -> String {
    let mut trimmed = String::with_capacity(raw.len());

    use TrimKind::*;
    use TrimState::*;

    raw.chars().fold(Ready(Break), |trim_state, ch| {
        let next_state = match (trim_state, get_trim_state(&ch)) {
            (Ready(kind), WhitespaceUnknown) | (Trimming(kind), WhitespaceUnknown) => {
                Trimming(kind)
            }
            (Trimming(kind), state) => {
                match (kind, &state) {
                    (Soft, ts) if !matches!(ts, Ready(Break)) => trimmed.push(' '),
                    (Hard, Idle) => trimmed.push(' '),
                    _ => (),
                }
                state
            }
            (Idle, WhitespaceUnknown) => Idle,
            (_, trim_state) => trim_state,
        };
        if !matches!(next_state, Trimming(_)) {
            trimmed.push(ch);
        }
        next_state
    });

    trimmed.trim().to_string()
}

fn get_trim_state(ch: &char) -> TrimState {
    if rule::SOFT_MARKS.contains(ch) {
        TrimState::Ready(TrimKind::Soft)
    } else if rule::BREAKS.contains(ch) {
        TrimState::Ready(TrimKind::Break)
    } else if rule::HARD_MARKS.contains(ch) {
        TrimState::Ready(TrimKind::Hard)
    } else if *ch == ' ' {
        TrimState::WhitespaceUnknown
    } else {
        TrimState::Idle
    }
}
