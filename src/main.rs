use image::{GenericImageView};
use std::path::Path;
use std::fs::File;
use tiff::encoder::{TiffEncoder, colortype};
use tiff::encoder::compression;

fn main() {
    let img = image::open("bess-axa.jpg").unwrap();
    println!("dimensions {:?}", img.dimensions());
    println!("{:?}", img.color());

    let dims = img.dimensions();

    let output_filename = String::from("test_deflate_best");
    let output = &mut File::create(&Path::new(&format!("{}.tif", output_filename))).unwrap();

    let mut bigtiff = TiffEncoder::new_big(output).unwrap();
    // bigtiff.write_image_with_compression::<colortype::RGB8, compression::Lzw>(dims.0, dims.1, compression::Lzw, &img.as_bytes()).unwrap();
    bigtiff.write_image_with_compression::<colortype::RGB8, compression::Deflate>(dims.0, dims.1, compression::Deflate::with_level(compression::DeflateLevel::Best), &img.as_bytes()).unwrap();
}
