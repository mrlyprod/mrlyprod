// Real WAV files carry extra chunks (titles, cue points) and odd sizes. The reader skips what it does not need.
// This file is built by hand: a LIST chunk of odd length before the format, and a trailing chunk after the sound.
// cargo run -p mrly-wav --example chunks
use mrly_wav::decode;

fn chunk(name: &[u8; 4], body: &[u8]) -> Vec<u8> {
    let mut out = name.to_vec();
    out.extend_from_slice(&(body.len() as u32).to_le_bytes());
    out.extend_from_slice(body);
    // Chunks start on even bytes: an odd body gets one pad byte.
    if body.len() % 2 == 1 {
        out.push(0);
    }
    out
}

fn main() -> Result<(), String> {
    let mut fmt = Vec::new();
    for field in [1u32, 2, 22050] {
        match field {
            22050 => fmt.extend_from_slice(&field.to_le_bytes()),
            _ => fmt.extend_from_slice(&(field as u16).to_le_bytes()),
        }
    }
    fmt.extend_from_slice(&(22050u32 * 6).to_le_bytes());
    fmt.extend_from_slice(&6u16.to_le_bytes());
    fmt.extend_from_slice(&24u16.to_le_bytes());

    // Two stereo frames of 24-bit sound: full up, full down, half up, silence.
    let sound = [
        0xFF, 0xFF, 0x7F, 0x00, 0x00, 0x80, 0x00, 0x00, 0x40, 0x00, 0x00, 0x00,
    ];
    let mut body = b"WAVE".to_vec();
    body.extend(chunk(b"LIST", b"INFOodd"));
    body.extend(chunk(b"fmt ", &fmt));
    body.extend(chunk(b"data", &sound));
    body.extend(chunk(b"cue ", &[0; 4]));
    let mut file = b"RIFF".to_vec();
    file.extend_from_slice(&(body.len() as u32).to_le_bytes());
    file.extend(body);

    let wav = decode(&file)?;
    println!(
        "{} Hz, {} channels, {} bits, float {}, {} frames",
        wav.rate,
        wav.channels,
        wav.bits,
        wav.float,
        wav.frames()
    );
    println!("samples {:?}", wav.samples);

    // Broken files are refused with a reason, never a crash.
    for (label, bytes) in [
        ("empty", &file[..0]),
        ("cut in the format", &file[..40]),
        ("not riff", b"OggS....WAVE".as_slice()),
    ] {
        println!("{label}: {}", decode(bytes).unwrap_err());
    }
    // A file cut inside the sound still gives the whole frames that arrived.
    println!(
        "cut in the sound: {} frame",
        decode(&file[..file.len() - 16])?.frames()
    );
    Ok(())
}
