use super::frames::{merging, writing};
use super::HOLD;

pub fn animation() -> Vec<Vec<usize>> {
    let writing = writing();
    let merging = merging();
    let mut out: Vec<Vec<usize>> = Vec::new();
    let hold = |frame: &Vec<usize>, out: &mut Vec<Vec<usize>>| {
        for _ in 0..HOLD {
            out.push(frame.clone());
        }
    };
    out.extend(writing.iter().cloned());
    hold(writing.last().unwrap(), &mut out);
    out.extend(merging.iter().cloned());
    hold(merging.last().unwrap(), &mut out);
    out.extend(merging.iter().rev().cloned());
    hold(merging.first().unwrap(), &mut out);
    out.extend(writing.iter().rev().cloned());
    hold(writing.first().unwrap(), &mut out);
    out
}
