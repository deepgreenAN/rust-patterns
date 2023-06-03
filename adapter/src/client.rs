use crate::adapter::{AdapterTarget, LegacyCVAdapter};

// -------------------------------------------------------------------------------------------------
// 直接関係はない画像取得用の型

struct VideoImage {
    width: u32,
    height: u32,
}

impl IntoIterator for VideoImage {
    type Item = Vec<[u8; 3]>;
    type IntoIter = VideoImageIntoIter;

    fn into_iter(self) -> Self::IntoIter {
        VideoImageIntoIter { video_image: self }
    }
}

struct VideoImageIntoIter {
    video_image: VideoImage,
}

impl Iterator for VideoImageIntoIter {
    type Item = Vec<[u8; 3]>;
    fn next(&mut self) -> Option<Self::Item> {
        std::thread::sleep(std::time::Duration::from_secs(1));
        let length = (self.video_image.width * self.video_image.height) as usize;
        Some((0..length).map(|_| [0_u8, 0_u8, 0_u8]).collect())
    }
}

// -------------------------------------------------------------------------------------------------
// クライアント
pub struct SurveillanceCamera<T = LegacyCVAdapter>
where
    T: AdapterTarget,
{
    detector_adapter: T,
    video_image: VideoImage,
}

impl<T> SurveillanceCamera<T>
where
    T: AdapterTarget,
{
    pub fn new() -> Self {
        Self {
            detector_adapter: T::new(600, 400),
            video_image: VideoImage {
                width: 600,
                height: 400,
            },
        }
    }

    pub fn run_loop(self) {
        let SurveillanceCamera {
            detector_adapter,
            video_image,
        } = self;

        for image in video_image.into_iter() {
            let faces = detector_adapter.detect_face(image);
            println!("検出された顔: {:?}", faces);
        }
    }
}
