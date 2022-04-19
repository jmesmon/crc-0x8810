use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::RngCore;

pub fn crc_xmodem(c: &mut Criterion) {
    let mut group = c.benchmark_group("xmodem");
    let mut rng = rand::thread_rng();
    let mut buf = [0u8; 4096];
    rng.fill_bytes(&mut buf);
    group.bench_function("crc-ccitt XMODEM", |b| {
        b.iter(|| crc_ccitt::CRC_16_XMODEM.checksum(black_box(&buf)))
    });

    group.bench_function("crc-any xmodem", |b| {
        b.iter(|| {
            let mut crc_any = crc_any::CRCu16::crc16xmodem();
            crc_any.digest(black_box(&buf));
            black_box(crc_any.get_crc())
        })
    });

    let crc = crc::Crc::<u16>::new(&crc::CRC_16_XMODEM);
    group.bench_function("crc xmodem", |b| {
        b.iter(|| {
            let mut digest = crc.digest();
            digest.update(black_box(&buf));
            black_box(digest.finalize());
        })
    });
}

criterion_group!(benches, crc_xmodem);
criterion_main!(benches);
