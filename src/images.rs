use image::imageops::FilterType::Triangle;
use image::GenericImageView;
use image::{io::Reader, DynamicImage, ImageFormat};
use std::fs::File;
use std::io::BufReader;

pub fn find_local_image_by_path(path: String) -> (DynamicImage, ImageFormat) {
    let reader: Reader<BufReader<File>> = Reader::open(path).unwrap();
    let format: ImageFormat = reader.format().unwrap();
    let image: DynamicImage = reader.decode().unwrap();

    (image, format)
}

#[derive(Debug)]
pub enum ImageDataErrors {
    DifferentImageFormats,
    BufferTooSmall,
}

pub fn ensure_format_compatibility(
    f1: ImageFormat,
    f2: ImageFormat,
) -> Result<(), ImageDataErrors> {
    if f1 != f2 {
        return Err(ImageDataErrors::DifferentImageFormats);
    }

    Ok(())
}

pub fn find_smallest_image_area(dimension_1: (u32, u32), dimension_2: (u32, u32)) -> (u32, u32) {
    let total_area_1 = dimension_1.0 * dimension_1.1;
    let total_area_2 = dimension_2.0 * dimension_2.1;

    return if total_area_1 < total_area_2 {
        dimension_1
    } else {
        dimension_2
    };
}

pub fn bring_images_to_the_same_size(
    image_1: DynamicImage,
    image_2: DynamicImage,
) -> (DynamicImage, DynamicImage) {
    let (smallest_width, smallest_height) =
        find_smallest_image_area(image_1.dimensions(), image_2.dimensions());

    if image_2.dimensions() == (smallest_height, smallest_height) {
        (
            image_1.resize_exact(smallest_width, smallest_height, Triangle),
            image_2,
        )
    } else {
        (
            image_1,
            image_2.resize_exact(smallest_width, smallest_height, Triangle),
        )
    }
}

pub struct FloatingImage {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
    pub name: String,
}

impl FloatingImage {
    pub fn new(width: u32, height: u32, name: String) -> Self {
        let buffer_capacity = 1024 * 1024 * 8;
        let buffer: Vec<u8> = Vec::with_capacity(buffer_capacity);

        Self {
            width,
            height,
            data: buffer,
            name,
        }
    }

    pub fn set_data(&mut self, data: Vec<u8>) -> Result<(), ImageDataErrors> {
        if data.len() > self.data.capacity() {
            return Err(ImageDataErrors::BufferTooSmall);
        }

        self.data = data;
        Ok(())
    }
}

fn set_rgba(vec: &Vec<u8>, start: usize, end: usize) -> Vec<u8> {
    let mut rgba = Vec::new();

    for i in start..=end {
        let val = match vec.get(i) {
            Some(d) => *d,
            None => panic!("Index is out of bound"),
        };

        rgba.push(val);
    }

    rgba
}

fn alternate_pixels(vec_1: Vec<u8>, vec_2: Vec<u8>) -> Vec<u8> {
    let mut combined_data = vec![0u8; vec_1.len()];

    let mut i = 0;

    while i < vec_1.len() {
        if i & 8 == 0 {
            combined_data.splice(i..=i + 3, set_rgba(&vec_1, i, i + 3));
        } else {
            combined_data.splice(i..=i + 3, set_rgba(&vec_2, i, i + 3));
        };
        i += 4;
    }
    combined_data
}

pub fn combine_images(image_1: DynamicImage, image_2: DynamicImage) -> Vec<u8> {
    let vec_1 = image_1.to_rgba8().into_vec();
    let vec_2 = image_2.to_rgba8().into_vec();

    alternate_pixels(vec_1, vec_2)
}
