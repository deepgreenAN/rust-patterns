use crate::reader::Read;

/// デコレートする側のReader
pub struct BufReader<R: Read> {
    reader: R,
    inner_buffer: Vec<char>,
    pos: usize,
}

impl<R: Read> BufReader<R> {
    pub fn new(mut reader: R) -> Self {
        // 簡単のためコンストラクト時に全て読み込み
        let mut inner_buffer = (0..reader.rest_length())
            .map(|_| ' ')
            .collect::<Vec<char>>();
        reader.read(&mut inner_buffer);

        Self {
            reader,
            inner_buffer,
            pos: 0,
        }
    }
}

impl<R: Read> Read for BufReader<R> {
    fn read(&mut self, buf: &mut [char]) -> usize {
        let char_iter = self.inner_buffer.iter().skip(self.pos);
        let mut counter = 0_usize;

        for (buf_char, inner_buffer_char) in buf.iter_mut().zip(char_iter) {
            *buf_char = *inner_buffer_char;
            counter += 1;
        }

        self.pos += counter;
        self.pos
    }
    fn rest_length(&self) -> usize {
        self.reader.rest_length()
    }
}
