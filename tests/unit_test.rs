use std::cmp::max;
use std::cell::RefCell;

const BONUS_UPPER_MATCH: i64 = 10;
const BONUS_ADJACENCY: i64 = 10;
const BONUS_SEPARATOR: i64 = 8;
const PENALTY_CASE_UNMATCHED: i64 = -1;
const PENALTY_LEADING: i64 = -6; // penalty applied for every letter before the first match
const PENALTY_MAX_LEADING: i64 = -18; // maxing penalty for leading letters
const PENALTY_UNMATCHED: i64 = -2;

// judge how many scores the current index should get
fn fuzzy_score(string: &[char], index: usize, pattern: &[char], pattern_idx: usize) -> i64 {
    let mut score = 0;

    let pattern_char = pattern[pattern_idx];
    let cur = string[index];

    if pattern_char.is_uppercase() && cur.is_uppercase() && pattern_char == cur {
        score += BONUS_UPPER_MATCH;
    } else {
        score += PENALTY_CASE_UNMATCHED;
    }

    let prev = string[index - 1];

    // apply bonus for matches after a separator
    if prev == ' ' || prev == '_' || prev == '-' || prev == '/' || prev == '\\' {
        score += BONUS_SEPARATOR;
    }

    if pattern_idx == 0 {
        score += max((index as i64) * PENALTY_LEADING, PENALTY_MAX_LEADING);
    }

    score
}

#[derive(Clone, Copy, Debug)]
struct MatchingStatus {
    pub idx: usize,
    pub score: i64,
    pub final_score: i64,
    pub adj_num: usize,
    pub back_ref: usize,
}

impl MatchingStatus {
    pub fn empty() -> Self {
        MatchingStatus {
            idx: 0,
            score: 0,
            final_score: 0,
            adj_num: 1,
            back_ref: 0,
        }
    }
}

pub fn fuzzy_match(choice: &[char], pattern: &[char]) -> Option<(i64, Vec<usize>)> {
    if pattern.is_empty() {
        return Some((0, Vec::new()));
    }

    let mut scores = vec![];
    let mut picked = vec![];

    let mut prev_matched_idx = -1; // to ensure that the pushed char are able to match the pattern
    for (pattern_idx, pattern_char) in pattern.iter().map(|c| c.to_ascii_lowercase()).enumerate() {
        let vec_cell = RefCell::new(vec![]);
        {
            let mut vec = vec_cell.borrow_mut();
            for (idx, ch) in choice.iter().map(|c| c.to_ascii_lowercase()).enumerate() {
                if ch == pattern_char && (idx as i64) > prev_matched_idx {
                    let score = fuzzy_score(choice, idx, pattern, pattern_idx);
                    vec.push(MatchingStatus {
                        idx,
                        score,
                        final_score: score,
                        adj_num: 1,
                        back_ref: 0,
                    });
                }
            }

            if vec.is_empty() {
                // not matched
                return None;
            }
            prev_matched_idx = vec[0].idx as i64;
        }
        scores.push(vec_cell);
    }

    for pattern_idx in 0..pattern.len() - 1 {
        let cur_row = scores[pattern_idx].borrow();
        let mut next_row = scores[pattern_idx + 1].borrow_mut();

        for idx in 0..next_row.len() {
            let next = next_row[idx];
            let prev = if idx > 0 {
                next_row[idx - 1]
            } else {
                MatchingStatus::empty()
            };
            let score_before_idx = prev.final_score - prev.score + next.score
                + PENALTY_UNMATCHED * ((next.idx - prev.idx) as i64)
                - if prev.adj_num == 0 { BONUS_ADJACENCY } else { 0 };

            let (back_ref, score, adj_num) = cur_row
                .iter()
                .enumerate()
                .take_while(|&(_, &MatchingStatus { idx, .. })| idx < next.idx)
                .skip_while(|&(_, &MatchingStatus { idx, .. })| idx < prev.idx)
                .map(|(back_ref, cur)| {
                    let adj_num = next.idx - cur.idx - 1;
                    let final_score = cur.final_score + next.score + if adj_num == 0 {
                        BONUS_ADJACENCY
                    } else {
                        PENALTY_UNMATCHED * adj_num as i64
                    };
                    (back_ref, final_score, adj_num)
                })
                .max_by_key(|&(_, x, _)| x)
                .unwrap_or((prev.back_ref, score_before_idx, prev.adj_num));

            next_row[idx] = if idx > 0 && score < score_before_idx {
                MatchingStatus {
                    final_score: score_before_idx,
                    back_ref: prev.back_ref,
                    adj_num,
                    ..next
                }
            } else {
                MatchingStatus {
                    final_score: score,
                    back_ref,
                    adj_num,
                    ..next
                }
            };
        }
    }

    let last_row = scores[pattern.len() - 1].borrow();
    let (mut next_col, &MatchingStatus { final_score, .. }) = last_row
        .iter()
        .enumerate()
        .max_by_key(|&(_, x)| x.final_score)
        .expect("score:fuzzy_match: failed to iterate over last_row");
    let mut pattern_idx = pattern.len() as i64 - 1;
    while pattern_idx >= 0 {
        let status = scores[pattern_idx as usize].borrow()[next_col];
        next_col = status.back_ref;
        picked.push(status.idx);
        pattern_idx -= 1;
    }
    picked.reverse();
    Some((final_score, picked))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn teset_fuzzy_match() {
    let choice_1: Vec<char> = "London".chars().collect();
    let query_1: Vec<char> = "London".chars().collect();
    assert_eq!(fuzzy_match(&choice_1[..], &query_1[..]), Some((100, vec![0,0])));
    }
}
