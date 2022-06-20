use image::{GenericImageView, DynamicImage};
use std::path::Path;
use std::fs::File;
use tiff::encoder::{TiffEncoder, colortype};
use tiff::encoder::compression;

use std::any::type_name;
fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

fn main() {
    let img = image::open("bess-axa.tif").unwrap();
    println!("dimensions {:?}", img.dimensions());
    println!("{:?}", img.color());

    let output_filename = String::from("test_deflate_best");
    println!("{}", type_of(&img));

    save_compressed_tiff(&img, &output_filename);
}

fn save_compressed_tiff(img: &DynamicImage, output_filename: &String)
{
    let output = &mut File::create(&Path::new(&format!("{}.tif", output_filename))).unwrap();
    let dims = img.dimensions();
    let mut bigtiff = TiffEncoder::new_big(output).unwrap();

    bigtiff.write_image_with_compression::<colortype::RGB8, compression::Lzw>(dims.0, dims.1, compression::Lzw, &img.as_bytes()).unwrap();
    // bigtiff.write_image_with_compression::<colortype::RGB8, compression::Deflate>(dims.0, dims.1, compression::Deflate::with_level(compression::DeflateLevel::Best), &img.as_bytes()).unwrap();
}
