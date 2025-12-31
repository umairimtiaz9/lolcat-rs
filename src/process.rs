use std::io::{BufRead, BufWriter, Write};
use crate::color::ColorGenerator;

pub fn process_input<R: BufRead, W: Write>(
    mut reader: R,
    writer: &mut BufWriter<W>,
    generator: &mut ColorGenerator,
    is_atty: bool,
) -> std::io::Result<()> {
    let mut buffer = Vec::new();

    loop {
        buffer.clear();
        let bytes_read = reader.read_until(b'\n', &mut buffer)?;
        if bytes_read == 0 {
            break;
        }

        // Decode bytes to string, escaping invalid sequences as \xNN (matching Python's backslashreplace)
        let line_string = decode_backslashreplace(&buffer);

        // Python's lolcat uses rstrip() which removes trailing whitespace (including \n)
        let trimmed = line_string.trim_end();

        generator.next_line(); // Advance the rainbow state per line

        for (char_idx, c) in trimmed.chars().enumerate() {
            if is_atty {
                write!(writer, "{}", generator.format_char(c, char_idx))?;
            } else {
                write!(writer, "{}", c)?;
            }
        }
        write!(writer, "\x1b[0m\n")?; // Reset color at end of line
    }
    writer.flush()
}

/// Decodes bytes to String, replacing invalid UTF-8 with \xNN escapes
fn decode_backslashreplace(v: &[u8]) -> String {
    let mut res = String::with_capacity(v.len());
    let mut i = 0;
    while i < v.len() {
        match std::str::from_utf8(&v[i..]) {
            Ok(s) => {
                res.push_str(s);
                break;
            }
            Err(e) => {
                let valid = e.valid_up_to();
                if valid > 0 {
                    // Safe because validation passed up to `valid`
                    res.push_str(unsafe { std::str::from_utf8_unchecked(&v[i..i + valid]) });
                    i += valid;
                }
                // The byte at i is invalid (or start of invalid seq)
                // We escape it as hex
                if i < v.len() {
                    use std::fmt::Write;
                    let _ = write!(res, "\\x{:02x}", v[i]);
                    i += 1;
                }
            }
        }
    }
    res
}
