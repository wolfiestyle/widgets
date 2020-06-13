use crate::geometry::Size;
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

#[cfg(feature = "image")]
use image::{DynamicImage, ImageResult};

pub type ImageRef = Arc<Image>;

#[derive(Debug, Eq)]
pub struct Image {
    pub data: ImageData,
    pub size: Size,
    pub format: PixelFormat,
    pub id: ImageId,
}

impl Image {
    pub fn new(data: impl Into<ImageData>, size: impl Into<Size>, format: PixelFormat) -> Self {
        let mut data = data.into();
        let size = size.into();
        // make sure the buffer is big enough
        if !matches!(data, ImageData::Empty) {
            let data_len = data.len();
            let expected_len = size.area() * format.num_components();
            if data_len != expected_len {
                data.resize(expected_len);
            }
        }

        Self {
            data,
            size,
            format,
            id: ImageId::new(),
        }
    }

    #[cfg(feature = "image")]
    #[inline]
    pub fn from_file(path: impl AsRef<Path>) -> ImageResult<Self> {
        image::open(path).map(Image::from)
    }

    #[cfg(feature = "image")]
    #[inline]
    pub fn from_bytes(buffer: &[u8]) -> ImageResult<Self> {
        image::load_from_memory(buffer).map(Image::from)
    }
}

#[cfg(feature = "image")]
impl From<DynamicImage> for Image {
    fn from(image: DynamicImage) -> Self {
        match image {
            DynamicImage::ImageLuma8(buf) => {
                let size = buf.dimensions();
                Self::new(buf.into_raw(), size, PixelFormat::Luma)
            }
            DynamicImage::ImageLumaA8(buf) => {
                let size = buf.dimensions();
                Self::new(buf.into_raw(), size, PixelFormat::LumaA)
            }
            DynamicImage::ImageRgb8(buf) => {
                let size = buf.dimensions();
                Self::new(buf.into_raw(), size, PixelFormat::Rgb)
            }
            DynamicImage::ImageRgba8(buf) => {
                let size = buf.dimensions();
                Self::new(buf.into_raw(), size, PixelFormat::Rgba)
            }
            DynamicImage::ImageBgr8(_) => {
                let buf = image.into_rgb();
                let size = buf.dimensions();
                Self::new(buf.into_raw(), size, PixelFormat::Rgb)
            }
            DynamicImage::ImageBgra8(_) => {
                let buf = image.into_rgba();
                let size = buf.dimensions();
                Self::new(buf.into_raw(), size, PixelFormat::Rgba)
            }
            DynamicImage::ImageLuma16(buf) => {
                let size = buf.dimensions();
                Self::new(buf.into_raw(), size, PixelFormat::Luma)
            }
            DynamicImage::ImageLumaA16(buf) => {
                let size = buf.dimensions();
                Self::new(buf.into_raw(), size, PixelFormat::LumaA)
            }
            DynamicImage::ImageRgb16(buf) => {
                let size = buf.dimensions();
                Self::new(buf.into_raw(), size, PixelFormat::Rgb)
            }
            DynamicImage::ImageRgba16(buf) => {
                let size = buf.dimensions();
                Self::new(buf.into_raw(), size, PixelFormat::Rgba)
            }
        }
    }
}

impl PartialEq for Image {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImageData {
    Empty,
    Bpp8(Vec<u8>),
    Bpp16(Vec<u16>),
}

impl ImageData {
    fn len(&self) -> usize {
        match self {
            ImageData::Empty => 0,
            ImageData::Bpp8(v) => v.len(),
            ImageData::Bpp16(v) => v.len(),
        }
    }

    fn resize(&mut self, new_len: usize) {
        match self {
            ImageData::Empty => (),
            ImageData::Bpp8(v) => v.resize(new_len, 0),
            ImageData::Bpp16(v) => v.resize(new_len, 0),
        }
    }
}

impl From<()> for ImageData {
    #[inline]
    fn from(_: ()) -> Self {
        ImageData::Empty
    }
}

impl From<Vec<u8>> for ImageData {
    #[inline]
    fn from(data: Vec<u8>) -> Self {
        ImageData::Bpp8(data)
    }
}

impl From<Vec<u16>> for ImageData {
    #[inline]
    fn from(data: Vec<u16>) -> Self {
        ImageData::Bpp16(data)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PixelFormat {
    Luma,
    LumaA,
    Rgb,
    Rgba,
}

impl PixelFormat {
    #[inline]
    pub fn num_components(self) -> usize {
        match self {
            PixelFormat::Luma => 1,
            PixelFormat::LumaA => 2,
            PixelFormat::Rgb => 3,
            PixelFormat::Rgba => 4,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ImageId(usize);

static IMAGE_ID: AtomicUsize = AtomicUsize::new(1);

impl ImageId {
    fn new() -> Self {
        let id = IMAGE_ID.fetch_add(1, Ordering::Relaxed);
        ImageId(id)
    }
}
