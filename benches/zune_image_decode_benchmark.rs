use criterion::{criterion_group, criterion_main, Criterion};
use zune_core::options::DecoderOptions;
use zune_image::image::Image;

pub(crate) const JPEG_IMAGE_DATA: &[u8] = include_bytes!("../resources/lenna.jpg");

fn load_image() -> Image {
    Image::read(JPEG_IMAGE_DATA, DecoderOptions::default()).unwrap()
}

fn benchmark_decode_qoi(c: &mut Criterion) {
    let image = load_image();
    let data = image
        .write_to_vec(zune_image::codecs::ImageFormat::QOI)
        .unwrap();

    c.bench_function("decode qoi", |b| {
        b.iter(|| {
            let _ = Image::read(&data, DecoderOptions::default()).unwrap();
        })
    });
}

fn benchmark_decode_jpeg(c: &mut Criterion) {
    let image = load_image();
    let data = image
        .write_to_vec(zune_image::codecs::ImageFormat::JPEG)
        .unwrap();

    c.bench_function("decode jpg", |b| {
        b.iter(|| {
            let _ = Image::read(&data, DecoderOptions::default()).unwrap();
        })
    });
}

fn benchmark_decode_png(c: &mut Criterion) {
    let image = load_image();
    let data = image
        .write_to_vec(zune_image::codecs::ImageFormat::PNG)
        .unwrap();

    c.bench_function("decode png", |b| {
        b.iter(|| {
            let _ = Image::read(&data, DecoderOptions::default()).unwrap();
        })
    });
}

criterion_group!(
    benches,
    benchmark_decode_qoi,
    benchmark_decode_jpeg,
    benchmark_decode_png
);
criterion_main!(benches);
