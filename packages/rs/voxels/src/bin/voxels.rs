use mrly_pixels::folder::{Sized, run, save};
use mrly_pixels::{Entry, Grid, json, print};
use mrly_three::Cell3d;
use mrly_voxels::{cell, obj, overview, png, turntable, view};
use std::path::Path;
use std::process::ExitCode;

const SIDES: [usize; 3] = [8, 16, 32];
const STEPS: usize = 24;
const DELAY: usize = 80;

fn measure(found: &[Sized], delay: Option<usize>, problems: &mut Vec<String>) {
    if delay.is_some_and(|delay| delay > 0) {
        problems.push("voxels take no delay in entry.toml.".into());
    }
    let base = found.iter().find(|(factor, _, _)| *factor == 1);
    if let Some((_, _, grid)) = base
        && !SIDES.contains(&grid.width)
    {
        problems.push(format!(
            "1x.txt is {} wide; a source cube is 8, 16 or 32 a side.",
            grid.width
        ));
    }
    for (factor, label, grid) in found {
        let (wide, deep, tall) = (grid.width, grid.height, grid.frames.len());
        let side = base.map_or(wide, |(_, _, grid)| grid.width * factor);
        if (wide, deep, tall) != (side, side, side) {
            problems.push(format!(
                "{label}.txt is {wide} wide, {deep} deep and {tall} layers tall, not a cube of {side}."
            ));
        }
    }
}

fn write(
    out: &Path,
    label: &str,
    grid: &Grid,
    entry: Option<&Entry>,
    problems: &mut Vec<String>,
) -> Option<Cell3d> {
    save(out.join(format!("{label}.txt")), print(grid), problems);
    if let Some(entry) = entry {
        let text = json(entry, grid, "depth", "layers");
        save(out.join(format!("{label}.json")), text, problems);
    }
    let made = match cell(grid) {
        Ok(made) => made,
        Err(error) => {
            problems.push(format!("{label}.cell.json: {error}"));
            return None;
        }
    };
    save(
        out.join(format!("{label}.cell.json")),
        mrly_three::to_json(&made),
        problems,
    );
    save(out.join(format!("{label}.obj")), obj(&made), problems);
    let shots = [
        ("png", png(&made, 768)),
        ("view.png", view(&made, 384)),
        ("gif", turntable(&made, 256, STEPS, DELAY)),
    ];
    for (kind, bytes) in shots {
        match bytes {
            Ok(bytes) => save(out.join(format!("{label}.{kind}")), bytes, problems),
            Err(error) => problems.push(format!("{label}.{kind}: {error}")),
        }
    }
    Some(made)
}

fn build(out: &Path, entry: Option<&Entry>, found: &[Sized], problems: &mut Vec<String>) -> String {
    measure(found, entry.map(|entry| entry.delay), problems);
    let mut made = Vec::new();
    for (_, label, grid) in found {
        made.extend(write(out, label, grid, entry, problems));
    }
    match overview(&made, 256) {
        Ok(bytes) => save(out.join("view.png"), bytes, problems),
        Err(error) => problems.push(format!("view.png: {error}")),
    }
    let sides: Vec<String> = found
        .iter()
        .map(|(_, _, grid)| grid.width.to_string())
        .collect();
    let inks: Vec<String> = found
        .iter()
        .map(|(_, _, grid)| grid.palette.len().to_string())
        .collect();
    format!("sides {}  colors {}", sides.join(" "), inks.join(" "))
}

fn main() -> ExitCode {
    run("voxels", build)
}
