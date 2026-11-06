fn merged(leaves: &[(u64, usize)]) -> Vec<u8> {
    let count = leaves.len();
    let mut weight: Vec<u64> = leaves.iter().map(|leaf| leaf.0).collect();
    let mut parent = vec![0usize; 2 * count - 1];
    let (mut leaf, mut node) = (0, count);
    for next in count..2 * count - 1 {
        let mut sum = 0;
        for _ in 0..2 {
            let pick = if leaf < count && (node == next || weight[leaf] <= weight[node]) {
                leaf += 1;
                leaf - 1
            } else {
                node += 1;
                node - 1
            };
            sum += weight[pick];
            parent[pick] = next;
        }
        weight.push(sum);
    }
    let mut depth = vec![0u8; 2 * count - 1];
    for at in (0..2 * count - 2).rev() {
        depth[at] = depth[parent[at]].saturating_add(1);
    }
    depth.truncate(count);
    depth
}

fn packaged(leaves: &[(u64, usize)], limit: usize) -> Vec<u8> {
    let most = 2 * leaves.len() - 2;
    let mut lists: Vec<Vec<(u64, bool)>> = Vec::with_capacity(limit);
    let mut last: Vec<(u64, bool)> = leaves.iter().map(|leaf| (leaf.0, true)).collect();
    for _ in 1..limit {
        let mut next = Vec::with_capacity(most);
        let mut pairs = last.chunks_exact(2).map(|pair| pair[0].0 + pair[1].0);
        let mut pair = pairs.next();
        let mut leaf = 0;
        while next.len() < most {
            match (pair, leaves.get(leaf)) {
                (Some(sum), Some(&(weight, _))) if sum < weight => {
                    next.push((sum, false));
                    pair = pairs.next();
                }
                (_, Some(&(weight, _))) => {
                    next.push((weight, true));
                    leaf += 1;
                }
                (Some(sum), None) => {
                    next.push((sum, false));
                    pair = pairs.next();
                }
                (None, None) => break,
            }
        }
        lists.push(last);
        last = next;
    }
    lists.push(last);
    let mut depth = vec![0u8; leaves.len()];
    let mut take = most;
    for list in lists.iter().rev() {
        let single = list[..take].iter().filter(|item| item.1).count();
        for slot in &mut depth[..single] {
            *slot += 1;
        }
        take = 2 * (take - single);
    }
    depth
}

pub(crate) fn lengths(counts: &[u32], limit: usize) -> Vec<u8> {
    let mut out = vec![0u8; counts.len()];
    let mut leaves: Vec<(u64, usize)> = counts
        .iter()
        .enumerate()
        .filter(|(_, count)| **count > 0)
        .map(|(symbol, &count)| (count as u64, symbol))
        .collect();
    if leaves.len() < 2 {
        let used = leaves.first().map(|leaf| leaf.1);
        let spare = if used == Some(0) { 1 } else { 0 };
        out[used.unwrap_or(1)] = 1;
        out[spare] = 1;
        return out;
    }
    leaves.sort_unstable();
    let mut depth = merged(&leaves);
    if depth.iter().any(|&bits| bits as usize > limit) {
        depth = packaged(&leaves, limit);
    }
    for (&(_, symbol), bits) in leaves.iter().zip(depth) {
        out[symbol] = bits;
    }
    out
}

pub(crate) fn codes(lengths: &[u8]) -> Vec<u16> {
    let mut count = [0u16; 16];
    for &len in lengths {
        count[len as usize] += 1;
    }
    count[0] = 0;
    let mut next = [0u16; 16];
    let mut code = 0u16;
    for bits in 1..16 {
        code = (code + count[bits - 1]) << 1;
        next[bits] = code;
    }
    lengths
        .iter()
        .map(|&len| {
            if len == 0 {
                return 0;
            }
            let code = next[len as usize];
            next[len as usize] += 1;
            code.reverse_bits() >> (16 - len)
        })
        .collect()
}
