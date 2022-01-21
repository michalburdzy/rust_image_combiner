mod args;
mod images;
use args::Args;
use image::GenericImageView;
use images::ensure_format_compatibility;
use images::find_local_image_by_path;

use crate::images::bring_images_to_the_same_size;
use crate::images::combine_images;
use crate::images::FloatingImage;
use crate::images::ImageDataErrors;

fn main() -> Result<(), ImageDataErrors> {
    let args = Args::new();
    println!("{:?}", args);

    let (image_1, image_1_format) = find_local_image_by_path(args.image_1);
    let (image_2, image_2_format) = find_local_image_by_path(args.image_2);

    let compatibility_error = ensure_format_compatibility(image_1_format, image_2_format);
    println!("{:?}", compatibility_error);

    match compatibility_error {
        Ok(()) => (),
        Err(err) => {
            panic!("{:?}", err)
        }
    }

    let (image_1, image_2) = bring_images_to_the_same_size(image_1, image_2);

    let mut output = FloatingImage::new(image_1.width(), image_1.height(), args.output);

    let combined_data = combine_images(image_1, image_2);

    output.set_data(combined_data)?;

    image::save_buffer_with_format(
        output.name,
        &output.data,
        output.width,
        output.height,
        image::ColorType::Rgba8,
        image_1_format,
    )
    .unwrap();

    Ok(())
}
