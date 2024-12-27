pub struct FrameProcessor {
    ascii_chars: Vec<char>,
    width: u32,
    height: u32,
}

impl FrameProcessor {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            ascii_chars: vec![' ', '.', ':', '-', '=', '+', '*', '#', '%', '@'],
            width,
            height,
        }
    }

    pub fn process_frame(&self, frame: &[u8]) -> String {
        let mut result = String::with_capacity((self.width * self.height) as usize);

        for y in 0..self.height {
            for x in 0..self.width {
                let idx = (y * self.width + x) as usize;
                if idx < frame.len() {
                    let pixel = frame[idx];
                    let char_idx = (pixel as f32 / 255.0 * (self.ascii_chars.len() - 1) as f32)
                        .round() as usize;
                    result.push(self.ascii_chars[char_idx]);
                }
            }
            result.push('\n');
        }

        result
    }
}
