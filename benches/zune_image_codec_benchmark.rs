use criterion::{criterion_group, criterion_main, Criterion};
use zune_core::options::DecoderOptions;
use zune_image::{codecs::ImageFormat, image::Image};

pub(crate) const JPEG_IMAGE_DATA: &[u8] = include_bytes!("../resources/lenna.jpg");

fn load_image() -> Image {
    Image::read(JPEG_IMAGE_DATA, DecoderOptions::default()).unwrap()
}

fn encode(format: ImageFormat) -> Vec<u8> {
    let image = load_image();
    image.write_to_vec(format).unwrap()
}

fn decode(data: &[u8]) -> Image {
    Image::read(data, DecoderOptions::default()).unwrap()
}

fn benchmark_encode_qoi(c: &mut Criterion) {
    c.bench_function("encode qoi", |b| {
        b.iter(|| {
            let _ = encode(ImageFormat::QOI);
        })
    });
}

fn benchmark_decode_qoi(c: &mut Criterion) {
    let data = encode(ImageFormat::QOI);
    c.bench_function("decode qoi", |b| {
        b.iter(|| {
            let _ = decode(&data);
        })
    });
}

fn benchmark_encode_jpeg(c: &mut Criterion) {
    c.bench_function("encode jpeg", |b| {
        b.iter(|| {
            let _ = encode(ImageFormat::JPEG);
        })
    });
}

fn benchmark_decode_jpeg(c: &mut Criterion) {
    let data = encode(ImageFormat::JPEG);
    c.bench_function("decode jpeg", |b| {
        b.iter(|| {
            let _ = decode(&data);
        })
    });
}

fn benchmark_encode_png(c: &mut Criterion) {
    c.bench_function("encode png", |b| {
        b.iter(|| {
            let _ = encode(ImageFormat::PNG);
        })
    });
}

fn benchmark_decode_png(c: &mut Criterion) {
    let data = encode(ImageFormat::PNG);
    c.bench_function("decode png", |b| {
        b.iter(|| {
            let _ = decode(&data);
        })
    });
}

criterion_group!(
    benches,
    benchmark_encode_qoi,
    benchmark_encode_jpeg,
    benchmark_encode_png,
    benchmark_decode_qoi,
    benchmark_decode_jpeg,
    benchmark_decode_png
);
criterion_main!(benches);
