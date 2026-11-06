use std::collections::HashMap;
use std::fmt::Write;

pub type Result<T> = std::result::Result<T, String>;

pub type Mesh = (Vec<[f64; 3]>, Vec<Vec<usize>>);

fn number(out: &mut String, value: f64) -> Result<()> {
    if !value.is_finite() {
        return Err(format!("a coordinate must be finite, not {value}."));
    }
    let value = if value == 0.0 { 0.0 } else { value };
    write!(out, "{value}").map_err(|e| e.to_string())
}

fn check(vertices: usize, faces: &[Vec<usize>]) -> Result<()> {
    for (at, face) in faces.iter().enumerate() {
        if face.len() < 3 {
            return Err(format!("face {at} has {} corners, under 3.", face.len()));
        }
        if let Some(&bad) = face.iter().find(|&&corner| corner >= vertices) {
            return Err(format!("face {at} names vertex {bad} of {vertices}."));
        }
    }
    Ok(())
}

pub fn encode(vertices: &[[f64; 3]], faces: &[Vec<usize>]) -> Result<String> {
    check(vertices.len(), faces)?;
    let mut out = String::with_capacity(vertices.len() * 12 + faces.len() * 12);
    for vertex in vertices {
        out.push('v');
        for &value in vertex {
            out.push(' ');
            number(&mut out, value)?;
        }
        out.push('\n');
    }
    for face in faces {
        out.push('f');
        for &corner in face {
            write!(out, " {}", corner + 1).map_err(|e| e.to_string())?;
        }
        out.push('\n');
    }
    Ok(out)
}

fn corner(word: &str, seen: usize, line: usize) -> Result<usize> {
    let head = word.split('/').next().unwrap_or("");
    let index: i64 = head
        .parse()
        .map_err(|_| format!("line {line}: {word} is not a face corner."))?;
    let at = if index < 0 {
        seen as i64 + index
    } else {
        index - 1
    };
    usize::try_from(at).map_err(|_| {
        format!("line {line}: corner {index} is outside the {seen} vertices read so far.")
    })
}

fn joined(text: &str) -> Vec<(usize, String)> {
    let mut out: Vec<(usize, String)> = Vec::new();
    let mut open = false;
    let text = text.strip_prefix('\u{feff}').unwrap_or(text);
    for (at, raw) in text.lines().enumerate() {
        let raw = raw.split('#').next().unwrap_or("");
        let (body, more) = match raw.trim_end().strip_suffix('\\') {
            Some(body) => (body, true),
            None => (raw, false),
        };
        match out.last_mut() {
            Some((_, line)) if open => {
                line.push(' ');
                line.push_str(body);
            }
            _ => out.push((at + 1, body.to_string())),
        }
        open = more;
    }
    out
}

pub fn decode(text: &str) -> Result<Mesh> {
    let mut vertices: Vec<[f64; 3]> = Vec::new();
    let mut faces: Vec<Vec<usize>> = Vec::new();
    for (line, body) in joined(text) {
        let mut words = body.split_whitespace();
        match words.next() {
            Some("v") => {
                let mut vertex = [0.0; 3];
                for slot in &mut vertex {
                    let word = words
                        .next()
                        .ok_or(format!("line {line}: a vertex needs three numbers."))?;
                    *slot = word
                        .parse::<f64>()
                        .ok()
                        .filter(|value| value.is_finite())
                        .ok_or(format!("line {line}: {word} is not a finite number."))?;
                }
                vertices.push(vertex);
            }
            Some("f") => {
                let face = words
                    .map(|word| corner(word, vertices.len(), line))
                    .collect::<Result<Vec<usize>>>()?;
                if face.len() < 3 {
                    return Err(format!("line {line}: a face needs at least three corners."));
                }
                faces.push(face);
            }
            _ => {}
        }
    }
    check(vertices.len(), &faces)?;
    Ok((vertices, faces))
}

fn key(vertex: &[f64; 3]) -> [u64; 3] {
    vertex.map(|value| if value == 0.0 { 0 } else { value.to_bits() })
}

pub fn weld(vertices: &[[f64; 3]], faces: &[Vec<usize>]) -> Result<Mesh> {
    check(vertices.len(), faces)?;
    let mut seen: HashMap<[u64; 3], usize> = HashMap::new();
    let mut kept: Vec<[f64; 3]> = Vec::new();
    let mut moved = vec![usize::MAX; vertices.len()];
    let mut out = Vec::with_capacity(faces.len());
    for face in faces {
        let mut next: Vec<usize> = Vec::with_capacity(face.len());
        for &corner in face {
            if moved[corner] == usize::MAX {
                moved[corner] = *seen.entry(key(&vertices[corner])).or_insert_with(|| {
                    kept.push(vertices[corner]);
                    kept.len() - 1
                });
            }
            if next.last() != Some(&moved[corner]) {
                next.push(moved[corner]);
            }
        }
        if next.len() > 1 && next.first() == next.last() {
            next.pop();
        }
        if next.len() >= 3 {
            out.push(next);
        }
    }
    Ok((kept, out))
}

pub fn soup(corners: &[[f64; 3]], sides: usize) -> Result<Mesh> {
    if sides < 3 {
        return Err(format!("a face has at least 3 sides, not {sides}."));
    }
    if !corners.len().is_multiple_of(sides) {
        return Err(format!(
            "{} corners do not fill whole faces of {sides}.",
            corners.len()
        ));
    }
    let faces: Vec<Vec<usize>> = (0..corners.len() / sides)
        .map(|face| (face * sides..(face + 1) * sides).collect())
        .collect();
    weld(corners, &faces)
}
