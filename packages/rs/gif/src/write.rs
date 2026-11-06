pub(crate) struct Frame {
    pub(crate) left: usize,
    pub(crate) top: usize,
    pub(crate) width: usize,
    pub(crate) height: usize,
    pub(crate) delay: u16,
    pub(crate) dispose: u8,
    pub(crate) transparent: Option<u8>,
    pub(crate) min_size: u8,
    pub(crate) data: Vec<u8>,
}

fn word(out: &mut Vec<u8>, value: usize) {
    out.extend_from_slice(&(value as u16).to_le_bytes());
}

pub(crate) fn assemble(
    width: usize,
    height: usize,
    palette: &[[u8; 3]],
    frames: &[Frame],
) -> Vec<u8> {
    let moving = frames.len() > 1;
    let controlled = |frame: &Frame| moving || frame.transparent.is_some();
    let size = frames
        .iter()
        .map(|frame| frame.data.len() + 32)
        .sum::<usize>()
        + 1024;
    let mut out: Vec<u8> = Vec::with_capacity(size);
    out.extend_from_slice(if frames.iter().any(controlled) {
        b"GIF89a"
    } else {
        b"GIF87a"
    });
    word(&mut out, width);
    word(&mut out, height);
    let mut bits = 1;
    while 1 << bits < palette.len() {
        bits += 1;
    }
    out.extend_from_slice(&[0x80 | (bits - 1) << 4 | (bits - 1), 0, 0]);
    for color in palette {
        out.extend_from_slice(color);
    }
    out.resize(13 + (3 << bits), 0);
    if moving {
        out.extend_from_slice(b"\x21\xFF\x0BNETSCAPE2.0\x03\x01\x00\x00\x00");
    }
    for frame in frames {
        if controlled(frame) {
            let flags = frame.dispose << 2 | frame.transparent.is_some() as u8;
            out.extend_from_slice(&[0x21, 0xF9, 4, flags]);
            word(&mut out, frame.delay as usize);
            out.extend_from_slice(&[frame.transparent.unwrap_or(0), 0]);
        }
        out.push(0x2C);
        for value in [frame.left, frame.top, frame.width, frame.height] {
            word(&mut out, value);
        }
        out.extend_from_slice(&[0, frame.min_size]);
        for chunk in frame.data.chunks(255) {
            out.push(chunk.len() as u8);
            out.extend_from_slice(chunk);
        }
        out.push(0);
    }
    out.push(0x3B);
    out
}
