/// Overview: Trim redundant empty characters.
use std::collections::HashSet;

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
    let mut state = TrimState::Ready(TrimKind::Break);

    for ch in raw.chars() {
        let mut should_change_state = true;
        let mut next_state = get_trim_state(&ch);
        match state {
            TrimState::Ready(kind) => {
                if matches!(next_state, TrimState::WhitespaceUnknown) {
                    next_state = match kind {
                        TrimKind::Soft => TrimState::Trimming(TrimKind::Soft),
                        TrimKind::Hard => TrimState::Trimming(TrimKind::Hard),
                        TrimKind::Break => TrimState::Trimming(TrimKind::Break),
                    };
                }
            }

            TrimState::Trimming(kind) => {
                if matches!(next_state, TrimState::WhitespaceUnknown) {
                    should_change_state = false;
                } else {
                    match kind {
                        TrimKind::Soft => {
                            if !matches!(next_state, TrimState::Ready(TrimKind::Break)) {
                                trimmed.push(' ');
                            }
                        }
                        TrimKind::Hard => {
                            if matches!(next_state, TrimState::Idle) {
                                trimmed.push(' ');
                            }
                        }
                        TrimKind::Break => (),
                    }
                }
            }

            TrimState::Idle => {
                if matches!(next_state, TrimState::WhitespaceUnknown) {
                    should_change_state = false;
                }
            }

            TrimState::WhitespaceUnknown => (),
        };

        if should_change_state {
            state = next_state;
        }

        if !matches!(state, TrimState::Trimming(_)) {
            trimmed.push(ch);
        }
    }

    trimmed.trim().to_string()
}

fn get_trim_state(ch: &char) -> TrimState {
    let soft_marks = HashSet::from(['.', ',', ';', ':', '?', '!', '-']);
    let breaks = HashSet::from(['\n']);
    let hard_marks = HashSet::from([
        '\'', '"', '(', ')', '{', '}', '[', ']', '《', '》', '〈', '〉',
    ]);

    if soft_marks.contains(ch) {
        TrimState::Ready(TrimKind::Soft)
    } else if breaks.contains(ch) {
        TrimState::Ready(TrimKind::Break)
    } else if hard_marks.contains(ch) {
        TrimState::Ready(TrimKind::Hard)
    } else if *ch == ' ' {
        TrimState::WhitespaceUnknown
    } else {
        TrimState::Idle
    }
}
