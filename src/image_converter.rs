
pub mod png;

struct RawImage {
    width: u32,
    height: u32,
    image_data: Vec<Vec<i32>>,
}

enum ImageTypes {
    PNG,
    JPG
}

trait FileConverter {
    fn decode(&self) -> anyhow::Result<RawImage>;
}