use crate::legacy::LegacyCV;

#[derive(Debug)]
pub struct Rectangle {
    pub top: u32,
    pub left: u32,
    pub width: u32,
    pub height: u32,
}

/// アダプターが実装すべきトレイト．クライアントはこのトレイトを用いる．
pub trait AdapterTarget {
    fn new(width: u32, height: u32) -> Self;
    fn detect_face(&self, image: Vec<[u8; 3]>) -> Vec<Rectangle>;
    fn detect_person(&self, image: Vec<[u8; 3]>) -> Vec<Rectangle>;
}

/// シンプルなアダプター
pub struct LegacyCVAdapter {
    adaptee: LegacyCV,
    width: u32,
}

impl AdapterTarget for LegacyCVAdapter {
    fn new(width: u32, height: u32) -> Self {
        LegacyCVAdapter {
            adaptee: LegacyCV::new(width, height),
            width,
        }
    }

    fn detect_person(&self, mut image: Vec<[u8; 3]>) -> Vec<Rectangle> {
        let mut image_2d: Vec<Vec<[u8; 3]>> = Vec::new();

        for chunk in image.chunks_exact_mut(self.width as usize) {
            let chunk_vec = chunk.to_vec();
            image_2d.push(chunk_vec);
        }

        self.adaptee
            .detect_multi_person(image_2d)
            .into_iter()
            .map(|[top, left, width, height]| Rectangle {
                top,
                left,
                width,
                height,
            })
            .collect()
    }

    fn detect_face(&self, mut image: Vec<[u8; 3]>) -> Vec<Rectangle> {
        let mut image_2d: Vec<Vec<[u8; 3]>> = Vec::new();

        for chunk in image.chunks_exact_mut(self.width as usize) {
            let chunk_vec = chunk.to_vec();
            image_2d.push(chunk_vec);
        }

        self.adaptee
            .detect_multi_face(image_2d)
            .into_iter()
            .map(|[top, left, width, height]| Rectangle {
                top,
                left,
                width,
                height,
            })
            .collect()
    }
}
