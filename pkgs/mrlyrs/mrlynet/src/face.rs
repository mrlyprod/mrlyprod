use mrlyos::kernel::{Manifest, Os};
use mrlyui::face::{FaceInput, FaceVerb};

pub fn manifest(app: &str) -> Option<Manifest> {
    crate::registry::catalogue()
        .into_iter()
        .find(|a| a.route() == app)
        .map(|a| a.manifest())
}

fn input(os: &Os, app: &str) -> Result<FaceInput, &'static str> {
    let view = os.peek(app).ok_or("no such app")?;
    let title = manifest(app)
        .map(|m| m.title)
        .unwrap_or_else(|| app.to_string());
    let dark = os
        .peek("settings")
        .map(|v| v.state["darkmode"] == true)
        .unwrap_or(false);
    Ok(FaceInput {
        app: app.to_string(),
        title,
        params: view.params,
        state: view.state,
        actions: view
            .actions
            .into_iter()
            .map(|v| FaceVerb {
                name: v.name,
                args: v.args,
            })
            .collect(),
        beat: view.beat.map(|c| c.verb),
        dark,
    })
}

pub fn face_frame(os: &Os, app: &str) -> Result<mrlyui::frame::Frame, &'static str> {
    Ok(mrlyui::face::face(&input(os, app)?))
}

pub fn face_rgba(os: &Os, app: &str) -> Result<(usize, usize, Vec<u8>), &'static str> {
    let frame = face_frame(os, app)?;
    let colors = frame.composite().cell.colors.unwrap_or_default();
    let mut buf = Vec::with_capacity(colors.len() * 4);
    for c in colors {
        buf.extend_from_slice(&c);
    }
    Ok((frame.width, frame.height, buf))
}

pub fn face_png(os: &Os, app: &str) -> Result<Vec<u8>, &'static str> {
    mrlyui::face::face_png(&input(os, app)?).map_err(|_| "could not render face")
}

pub fn canvas_rgba(os: &Os, app: &str) -> Result<(usize, usize, Vec<u8>), &'static str> {
    let view = os.peek(app).ok_or("no such app")?;
    let fact = &view.state["frame"];
    if fact.is_null() {
        return Err("nothing to shoot here");
    }
    let (w, h, pixels) = mrlyui::face::decode(fact).ok_or("bad frame")?;
    let mut buf = Vec::with_capacity(pixels.len() * 4);
    for c in pixels {
        buf.extend_from_slice(&c);
    }
    Ok((w, h, buf))
}
