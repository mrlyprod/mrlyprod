use mrly_pixels::folder::{Sized, run, save};
use mrly_pixels::{Entry, Grid, cells, gif, json, overview, png, print, quarters, view};
use std::path::Path;
use std::process::ExitCode;

const SIDES: [usize; 3] = [16, 32, 64];

fn measure(found: &[Sized], delay: Option<usize>, problems: &mut Vec<String>) {
    let Some((_, _, base)) = found.iter().find(|(factor, _, _)| *factor == 1) else {
        return;
    };
    if base.width != base.height || !SIDES.contains(&base.width) {
        problems.push(format!(
            "1x.txt is {} by {}; a source canvas is 16, 32 or 64 square.",
            base.width, base.height
        ));
    }
    for (factor, label, grid) in found {
        let (width, height) = (base.width * factor, base.height * factor);
        if (grid.width, grid.height) != (width, height) {
            problems.push(format!(
                "{label}.txt is {} by {}, not {width} by {height}.",
                grid.width, grid.height
            ));
        }
        if grid.frames.len() != base.frames.len() {
            problems.push(format!(
                "{label}.txt has {} frames, 1x.txt has {}.",
                grid.frames.len(),
                base.frames.len()
            ));
        }
    }
    match (base.frames.len() > 1, delay) {
        (true, Some(delay)) if delay < 20 || delay % 10 != 0 => problems.push(
            "an animation needs a delay in entry.toml: 20 ms or more, in steps of 10.".into(),
        ),
        (false, Some(delay)) if delay > 0 => {
            problems.push("a still takes no delay in entry.toml.".into());
        }
        _ => {}
    }
}

fn write(out: &Path, label: &str, grid: &Grid, entry: Option<&Entry>, problems: &mut Vec<String>) {
    let delay = entry.map_or(0, |entry| entry.delay);
    let side = if grid.frames.len() > 1 { 256 } else { 512 };
    save(out.join(format!("{label}.txt")), print(grid), problems);
    if let Some(entry) = entry {
        save(
            out.join(format!("{label}.json")),
            json(entry, grid, "height", "frames"),
            problems,
        );
    }
    let made = [
        ("png", png(grid, 1)),
        ("gif", gif(grid, delay, 1)),
        ("view.png", view(grid, side)),
    ];
    for (kind, bytes) in made {
        match bytes {
            Ok(bytes) => save(out.join(format!("{label}.{kind}")), bytes, problems),
            Err(error) => problems.push(format!("{label}.{kind}: {error}")),
        }
    }
    if grid.width > 64 {
        match quarters(grid, side) {
            Ok(parts) => {
                for (at, bytes) in parts.into_iter().enumerate() {
                    save(
                        out.join(format!("{label}.view.{}.png", at + 1)),
                        bytes,
                        problems,
                    );
                }
            }
            Err(error) => problems.push(format!("{label}.view.1.png: {error}")),
        }
    }
    match cells(grid) {
        Ok(found) => {
            for (at, cell) in found.iter().enumerate() {
                let name = if found.len() > 1 {
                    format!("{label}.cell.{at}.json")
                } else {
                    format!("{label}.cell.json")
                };
                save(out.join(name), mrly_two::to_json(cell), problems);
            }
        }
        Err(error) => problems.push(format!("{label}.cell.json: {error}")),
    }
}

fn build(out: &Path, entry: Option<&Entry>, found: &[Sized], problems: &mut Vec<String>) -> String {
    measure(found, entry.map(|entry| entry.delay), problems);
    for (_, label, grid) in found {
        write(out, label, grid, entry, problems);
    }
    let all: Vec<Grid> = found.iter().map(|(_, _, grid)| grid.clone()).collect();
    match overview(&all, 256) {
        Ok(bytes) => save(out.join("view.png"), bytes, problems),
        Err(error) => problems.push(format!("view.png: {error}")),
    }
    let sides: Vec<String> = all.iter().map(|grid| grid.width.to_string()).collect();
    let inks: Vec<String> = all
        .iter()
        .map(|grid| grid.palette.len().to_string())
        .collect();
    let frames = all.first().map_or(0, |grid| grid.frames.len());
    format!(
        "sides {}  frames {frames}  colors {}",
        sides.join(" "),
        inks.join(" ")
    )
}

fn main() -> ExitCode {
    run("pixels", build)
}
