pub type Image = Vec<Vec<[u8; 3]>>;

/// アダプティとなるレガシーコード
pub struct LegacyCV {
    height: u32,
    width: u32,
}

impl LegacyCV {
    pub fn new(width: u32, height: u32) -> Self {
        Self { height, width }
    }
    pub fn detect_multi_face(&self, _image: Image) -> Vec<[u32; 4]> {
        vec![[100, 100, 200, 200]]
    }
    pub fn detect_multi_person(&self, _image: Image) -> Vec<[u32; 4]> {
        vec![
            [100, 100, 200, 200],
            [120, 150, 100, 100],
            [200, 200, 100, 150],
        ]
    }
}
