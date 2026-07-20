pub const DESCENDERS: &[char] = &[
    '@', '$', '\u{00a9}', '\u{00ae}', '(', ')', '[', ']', '{', '}',
];

pub fn descends(c: char) -> bool {
    DESCENDERS.contains(&c)
}

pub fn trim(rows: &[String]) -> Vec<String> {
    let grid: Vec<Vec<char>> = rows.iter().map(|row| row.chars().collect()).collect();
    if grid.is_empty() || grid[0].is_empty() {
        return rows.to_vec();
    }
    let width = grid[0].len();
    let lit = |col: usize| grid.iter().any(|row| row[col] == '1');
    let Some(start) = (0..width).find(|&col| lit(col)) else {
        return grid.iter().map(|_| "0".to_string()).collect();
    };
    let end = (0..width).rev().find(|&col| lit(col)).unwrap();
    grid.iter()
        .map(|row| row[start..=end].iter().collect())
        .collect()
}
