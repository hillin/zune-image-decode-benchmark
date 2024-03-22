use criterion::{criterion_group, criterion_main, Criterion};
use image::RgbImage;
use zune_core::options::DecoderOptions;
use zune_image::{codecs::ImageFormat, image::Image};

pub(crate) const JPEG_IMAGE_DATA: &[u8] = include_bytes!("../resources/lenna.jpg");

fn zune_load_image() -> Image {
    Image::read(JPEG_IMAGE_DATA, DecoderOptions::default()).unwrap()
}

fn zune_encode(format: ImageFormat) -> Vec<u8> {
    let image = zune_load_image();
    image.write_to_vec(format).unwrap()
}

fn zune_decode(data: &[u8]) -> Image {
    Image::read(data, DecoderOptions::default()).unwrap()
}

pub fn irs_load_image() -> RgbImage {
    image::load_from_memory(JPEG_IMAGE_DATA).unwrap().to_rgb8()
}

fn irs_encode(format: image::ImageFormat) -> Vec<u8> {
    let image = irs_load_image();
    let mut data = Vec::new();
    let mut cursor = std::io::Cursor::new(&mut data);
    image.write_to(&mut cursor, format).unwrap();

    data
}

fn irs_decode(data: &[u8]) -> RgbImage {
    image::load_from_memory(data).unwrap().to_rgb8()
}

fn benchmark_zune_encode_qoi(c: &mut Criterion) {
    c.bench_function("zune encode qoi", |b| {
        b.iter(|| {
            let _ = zune_encode(ImageFormat::QOI);
        })
    });
}

fn benchmark_zune_decode_qoi(c: &mut Criterion) {
    let data = zune_encode(ImageFormat::QOI);
    c.bench_function("zune decode qoi", |b| {
        b.iter(|| {
            let _ = zune_decode(&data);
        })
    });
}

fn benchmark_irs_encode_qoi(c: &mut Criterion) {
    c.bench_function("image-rs encode qoi", |b| {
        b.iter(|| {
            let _ = irs_encode(image::ImageFormat::Qoi);
        })
    });
}

fn benchmark_irs_decode_qoi(c: &mut Criterion) {
    let data = irs_encode(image::ImageFormat::Qoi);
    c.bench_function("image-rs decode qoi", |b| {
        b.iter(|| {
            let _ = irs_decode(&data);
        })
    });
}

fn benchmark_encode_jpeg(c: &mut Criterion) {
    c.bench_function("zune encode jpeg", |b| {
        b.iter(|| {
            let _ = zune_encode(ImageFormat::JPEG);
        })
    });
}

fn benchmark_decode_jpeg(c: &mut Criterion) {
    let data = zune_encode(ImageFormat::JPEG);
    c.bench_function("zune decode jpeg", |b| {
        b.iter(|| {
            let _ = zune_decode(&data);
        })
    });
}

fn benchmark_encode_png(c: &mut Criterion) {
    c.bench_function("zune encode png", |b| {
        b.iter(|| {
            let _ = zune_encode(ImageFormat::PNG);
        })
    });
}

fn benchmark_decode_png(c: &mut Criterion) {
    let data = zune_encode(ImageFormat::PNG);
    c.bench_function("zune decode png", |b| {
        b.iter(|| {
            let _ = zune_decode(&data);
        })
    });
}

criterion_group!(
    benches,
    benchmark_zune_encode_qoi,
    benchmark_irs_encode_qoi,
    benchmark_encode_jpeg,
    benchmark_encode_png,
    benchmark_zune_decode_qoi,
    benchmark_irs_decode_qoi,
    benchmark_decode_jpeg,
    benchmark_decode_png,
);
criterion_main!(benches);
