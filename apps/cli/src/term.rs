use std::io::Write;

const SHADOW: char = '\0';

// RAW

pub struct Raw {
    saved: libc::termios,
}

impl Raw {
    pub fn enter() -> Raw {
        let mut saved: libc::termios = unsafe { std::mem::zeroed() };
        unsafe { libc::tcgetattr(0, &mut saved) };
        hook(saved);
        let mut raw = saved;
        unsafe {
            libc::cfmakeraw(&mut raw);
            libc::tcsetattr(0, libc::TCSANOW, &raw);
        }
        emit("\x1b[?1049h\x1b[?25l");
        Raw { saved }
    }
}

impl Drop for Raw {
    fn drop(&mut self) {
        restore(&self.saved);
    }
}

fn restore(saved: &libc::termios) {
    emit("\x1b[0m\x1b[?25h\x1b[?1049l");
    unsafe { libc::tcsetattr(0, libc::TCSANOW, saved) };
}

fn hook(saved: libc::termios) {
    let prev = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        restore(&saved);
        prev(info);
    }));
}

fn emit(text: &str) {
    let mut out = std::io::stdout();
    out.write_all(text.as_bytes()).ok();
    out.flush().ok();
}

pub fn size() -> (usize, usize) {
    let mut win: libc::winsize = unsafe { std::mem::zeroed() };
    let ok = unsafe { libc::ioctl(1, libc::TIOCGWINSZ, &mut win) };
    if ok == 0 && win.ws_col > 0 && win.ws_row > 0 {
        (win.ws_col as usize, win.ws_row as usize)
    } else {
        (80, 24)
    }
}

pub fn tty() -> bool {
    unsafe { libc::isatty(0) == 1 && libc::isatty(1) == 1 }
}

// KEYS

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Key {
    Up,
    Down,
    Left,
    Right,
    Char(char),
    Enter,
    Backspace,
    Esc,
    Ctrl(char),
}

pub fn parse(bytes: &[u8]) -> Vec<Key> {
    let mut keys = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        let byte = bytes[i];
        if byte == 0x1b {
            i = escape(bytes, i, &mut keys);
        } else if byte == 0x0d || byte == 0x0a {
            keys.push(Key::Enter);
            i += 1;
        } else if byte == 0x7f || byte == 0x08 {
            keys.push(Key::Backspace);
            i += 1;
        } else if (0x01..=0x1a).contains(&byte) {
            keys.push(Key::Ctrl((byte - 1 + b'a') as char));
            i += 1;
        } else if byte >= 0x20 {
            match decode(bytes, i) {
                Some((ch, len)) => {
                    keys.push(Key::Char(ch));
                    i += len;
                }
                None => i += 1,
            }
        } else {
            i += 1;
        }
    }
    keys
}

fn escape(bytes: &[u8], i: usize, keys: &mut Vec<Key>) -> usize {
    match bytes.get(i + 1) {
        None => {
            keys.push(Key::Esc);
            i + 1
        }
        Some(b'[') => {
            let mut j = i + 2;
            while j < bytes.len() && !(0x40..=0x7e).contains(&bytes[j]) {
                j += 1;
            }
            if j >= bytes.len() {
                return bytes.len();
            }
            if j == i + 2 {
                if let Some(key) = arrow(bytes[j]) {
                    keys.push(key);
                }
            }
            j + 1
        }
        Some(b'O') => match bytes.get(i + 2).copied().and_then(arrow) {
            Some(key) => {
                keys.push(key);
                i + 3
            }
            None => i + 2,
        },
        Some(_) => {
            keys.push(Key::Esc);
            i + 1
        }
    }
}

fn arrow(byte: u8) -> Option<Key> {
    match byte {
        b'A' => Some(Key::Up),
        b'B' => Some(Key::Down),
        b'C' => Some(Key::Right),
        b'D' => Some(Key::Left),
        _ => None,
    }
}

fn decode(bytes: &[u8], i: usize) -> Option<(char, usize)> {
    let byte = bytes[i];
    let len = if byte < 0x80 {
        1
    } else if byte >> 5 == 0b110 {
        2
    } else if byte >> 4 == 0b1110 {
        3
    } else if byte >> 3 == 0b11110 {
        4
    } else {
        return None;
    };
    let text = std::str::from_utf8(bytes.get(i..i + len)?).ok()?;
    Some((text.chars().next()?, len))
}

// CELLS

#[derive(Clone, PartialEq)]
pub struct Cell {
    pub ch: char,
    pub fg: Option<(u8, u8, u8)>,
    pub bg: Option<(u8, u8, u8)>,
    pub dim: bool,
}

impl Default for Cell {
    fn default() -> Cell {
        Cell {
            ch: ' ',
            fg: None,
            bg: None,
            dim: false,
        }
    }
}

impl Cell {
    pub fn ink(ch: char, fg: Option<(u8, u8, u8)>, dim: bool) -> Cell {
        Cell {
            ch,
            fg,
            bg: None,
            dim,
        }
    }
    pub fn block(fg: (u8, u8, u8), bg: (u8, u8, u8)) -> Cell {
        Cell {
            ch: '\u{2580}',
            fg: Some(fg),
            bg: Some(bg),
            dim: false,
        }
    }
}

pub fn wide(ch: char) -> bool {
    let c = ch as u32;
    (0x1100..=0x115f).contains(&c)
        || (0x231a..=0x231b).contains(&c)
        || (0x23e9..=0x23fa).contains(&c)
        || (0x25fd..=0x25fe).contains(&c)
        || (0x2600..=0x27bf).contains(&c)
        || (0x2b00..=0x2bff).contains(&c)
        || (0x2e80..=0xa4cf).contains(&c)
        || (0xac00..=0xd7a3).contains(&c)
        || (0xf900..=0xfaff).contains(&c)
        || (0xfe30..=0xfe6f).contains(&c)
        || (0xff00..=0xff60).contains(&c)
        || (0x1f000..=0x1faff).contains(&c)
}

fn zero(ch: char) -> bool {
    matches!(ch, '\u{fe0e}' | '\u{fe0f}' | '\u{200d}')
}

pub struct Screen {
    pub w: usize,
    pub h: usize,
    cells: Vec<Cell>,
}

impl Screen {
    pub fn new(w: usize, h: usize) -> Screen {
        Screen {
            w,
            h,
            cells: vec![Cell::default(); w * h],
        }
    }
    pub fn put(&mut self, x: usize, y: usize, cell: Cell) {
        if x < self.w && y < self.h {
            self.cells[y * self.w + x] = cell;
        }
    }
    pub fn text(
        &mut self,
        x: usize,
        y: usize,
        text: &str,
        fg: Option<(u8, u8, u8)>,
        dim: bool,
    ) -> usize {
        let mut cx = x;
        for ch in text.chars() {
            if zero(ch) {
                continue;
            }
            if cx >= self.w {
                break;
            }
            self.put(cx, y, Cell::ink(ch, fg, dim));
            cx += 1;
            if wide(ch) {
                self.put(cx, y, Cell::ink(SHADOW, None, false));
                cx += 1;
            }
        }
        cx
    }
}

// DIFF

pub fn diff(prev: Option<&Screen>, next: &Screen) -> String {
    let stale = match prev {
        Some(old) => old.w != next.w || old.h != next.h,
        None => true,
    };
    let mut out = String::new();
    if stale {
        out.push_str("\x1b[2J");
    }
    let mut pen = Cell::default();
    for y in 0..next.h {
        let mut run = false;
        for x in 0..next.w {
            let cell = &next.cells[y * next.w + x];
            let same = !stale && prev.is_some_and(|old| &old.cells[y * old.w + x] == cell);
            if same || cell.ch == SHADOW {
                run = false;
                continue;
            }
            if !run {
                out.push_str(&format!("\x1b[{};{}H", y + 1, x + 1));
                run = true;
            }
            dress(&mut out, &mut pen, cell);
            out.push(cell.ch);
        }
    }
    if !out.is_empty() {
        out.push_str("\x1b[0m");
    }
    out
}

fn dress(out: &mut String, pen: &mut Cell, cell: &Cell) {
    if pen.fg == cell.fg && pen.bg == cell.bg && pen.dim == cell.dim {
        return;
    }
    let drops = (pen.fg.is_some() && cell.fg.is_none())
        || (pen.bg.is_some() && cell.bg.is_none())
        || (pen.dim && !cell.dim);
    if drops {
        out.push_str("\x1b[0m");
        pen.fg = None;
        pen.bg = None;
        pen.dim = false;
    }
    if let Some((r, g, b)) = cell.fg {
        if pen.fg != cell.fg {
            out.push_str(&format!("\x1b[38;2;{r};{g};{b}m"));
        }
    }
    if let Some((r, g, b)) = cell.bg {
        if pen.bg != cell.bg {
            out.push_str(&format!("\x1b[48;2;{r};{g};{b}m"));
        }
    }
    if cell.dim && !pen.dim {
        out.push_str("\x1b[2m");
    }
    pen.fg = cell.fg;
    pen.bg = cell.bg;
    pen.dim = cell.dim;
}

#[cfg(test)]
impl Screen {
    pub fn dump(&self) -> String {
        let mut out = String::new();
        for y in 0..self.h {
            for x in 0..self.w {
                let ch = self.cells[y * self.w + x].ch;
                out.push(if ch == SHADOW { ' ' } else { ch });
            }
            out.push('\n');
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arrows_decode() {
        assert_eq!(parse(b"\x1b[A"), vec![Key::Up]);
        assert_eq!(parse(b"\x1b[B"), vec![Key::Down]);
        assert_eq!(parse(b"\x1b[C\x1b[D"), vec![Key::Right, Key::Left]);
    }

    #[test]
    fn a_lone_escape_is_escape() {
        assert_eq!(parse(b"\x1b"), vec![Key::Esc]);
    }

    #[test]
    fn control_and_plain_keys_decode() {
        assert_eq!(parse(&[3]), vec![Key::Ctrl('c')]);
        assert_eq!(parse(b"q"), vec![Key::Char('q')]);
        assert_eq!(parse(b"\r"), vec![Key::Enter]);
        assert_eq!(parse(&[0x7f]), vec![Key::Backspace]);
        assert_eq!(parse("é".as_bytes()), vec![Key::Char('é')]);
    }

    #[test]
    fn unknown_sequences_leave_no_junk() {
        assert_eq!(parse(b"\x1b[200~"), Vec::new());
        assert_eq!(parse(b"\x1b[1;2Rx"), vec![Key::Char('x')]);
    }

    #[test]
    fn identical_screens_diff_to_nothing() {
        let a = Screen::new(10, 3);
        let b = Screen::new(10, 3);
        assert_eq!(diff(Some(&a), &b), "");
    }

    #[test]
    fn one_cell_moves_the_cursor_once() {
        let a = Screen::new(10, 3);
        let mut b = Screen::new(10, 3);
        b.put(4, 1, Cell::ink('x', None, false));
        let out = diff(Some(&a), &b);
        assert_eq!(out.matches('H').count(), 1);
        assert!(out.contains("\x1b[2;5H"));
        assert!(!out.contains("\x1b[2J"));
        assert!(out.ends_with("\x1b[0m"));
    }

    #[test]
    fn a_resize_repaints_everything() {
        let a = Screen::new(10, 3);
        let b = Screen::new(12, 3);
        assert!(diff(Some(&a), &b).contains("\x1b[2J"));
        assert!(diff(None, &b).contains("\x1b[2J"));
    }

    #[test]
    fn a_wide_char_shadows_the_next_cell() {
        let mut screen = Screen::new(10, 1);
        let x = screen.text(0, 0, "🐍 s", None, false);
        assert_eq!(x, 4);
        let out = diff(None, &screen);
        assert!(out.contains('\u{1f40d}'));
        assert_eq!(out.matches('H').count(), 2);
    }

    #[test]
    fn plane_zero_emoji_are_wide_and_selectors_vanish() {
        assert!(wide('✨'));
        assert!(wide('⏱'));
        let mut screen = Screen::new(10, 1);
        let x = screen.text(0, 0, "⏱️ x", None, false);
        assert_eq!(x, 4);
    }
}
