/// デコレートする側・される側で共通して実装するトレイト
pub trait Read {
    fn read(&mut self, buf: &mut [char]) -> usize;
    // 簡単にするために必要
    fn rest_length(&self) -> usize;
}

/// デコレートされる側のReader
pub struct StringReader {
    content: String,
    pos: usize,
}

impl StringReader {
    pub fn new(content: String) -> Self {
        Self { content, pos: 0 }
    }
}

impl Read for StringReader {
    fn read(&mut self, buf: &mut [char]) -> usize {
        let mut counter = 0_usize;

        for (buf_char, self_char) in buf.iter_mut().zip(self.content.chars().skip(self.pos)) {
            *buf_char = self_char;
            counter += 1;
        }

        self.pos += counter;
        self.pos
    }
    fn rest_length(&self) -> usize {
        self.content.len() - self.pos
    }
}
