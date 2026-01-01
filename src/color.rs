pub struct ColorGenerator {
    pub freq: f64,
    pub spread: f64,
    pub seed: f64,
    pub line_idx: usize,
}

impl ColorGenerator {
    /// Advances to the next line (simulating options.os += 1 in Python)
    pub fn next_line(&mut self) {
        self.line_idx += 1;
    }

    /// Generates the RGB values for a specific point in the rainbow
    pub fn get_rgb(&self, char_idx: usize) -> (u8, u8, u8) {
        // We use both char_idx and line_idx to ensure a diagonal gradient
        let i = self.seed + (self.line_idx as f64) + (char_idx as f64 / self.spread);

        // Saturation Boost:
        // By increasing the amplitude from 127.0 to 150.0 (or higher), we "clip" the sine wave.
        // This makes the peaks (pure colors) wider and the transitions (muddy colors) sharper.
        // This effectively removes the "dull" middle-ground colors.
        let r = ((self.freq * i).sin() * 150.0 + 128.0).clamp(0.0, 255.0);
        let g = ((self.freq * i + 2.0 * std::f64::consts::PI / 3.0).sin() * 150.0 + 128.0).clamp(0.0, 255.0);
        let b = ((self.freq * i + 4.0 * std::f64::consts::PI / 3.0).sin() * 150.0 + 128.0).clamp(0.0, 255.0);

        (r as u8, g as u8, b as u8)
    }

    /// Writes a character with TrueColor ANSI codes directly to a writer
    pub fn write_colored_char<W: std::io::Write>(
        &self,
        w: &mut W,
        c: char,
        char_idx: usize,
    ) -> std::io::Result<()> {
        let (r, g, b) = self.get_rgb(char_idx);
        write!(w, "\x1b[38;2;{};{};{}m{}", r, g, b, c)
    }
}
