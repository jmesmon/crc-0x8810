use proptest::prelude::*;

#[test]
fn plain() {
    let crc_a = crc::Crc::<u16>::new(&crc::CRC_16_XMODEM);
    let crc_b = crc_0x8810::CRC_16_XMODEM;

    proptest!(move |(s: Vec<u8>)| {
        let a = {
            let mut d = crc_a.digest();
            d.update(&s[..]);
            d.finalize()
        };
        let b = crc_b.checksum(&s[..]);
        prop_assert_eq!(a, b);
    })
}
