#[test]
fn roundtrip_u32() {
    for i in 0..=u32::max_value() {
        let encoded = vint64::encode(i as u64);
        let out = vint64::decode(&mut encoded.as_ref()).unwrap_or_else(|e| {
            panic!(
                "error while decoding {}: {:?} (bytes: {:x?})",
                i,
                e,
                encoded.as_ref()
            );
        }) as u32;
        assert_eq!(out, i);
    }
}
