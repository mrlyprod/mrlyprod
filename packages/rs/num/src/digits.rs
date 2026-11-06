pub fn elements(base: u64, digits: &[u64], depth: usize) -> Vec<u64> {
    let lead: Vec<u64> = digits.iter().copied().filter(|&d| d > 0).collect();
    if depth == 0 || lead.is_empty() {
        return Vec::new();
    }
    let mut out = lead.clone();
    let mut level = lead;
    for _ in 1..depth {
        let mut next = Vec::with_capacity(level.len() * digits.len());
        for value in &level {
            for digit in digits {
                next.push(value * base + digit);
            }
        }
        out.extend_from_slice(&next);
        level = next;
    }
    out
}
