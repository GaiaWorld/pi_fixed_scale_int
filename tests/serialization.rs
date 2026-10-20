use pi_fixed_scale_int::{FixedI64, FixedU64};

#[test]
fn debug_shows_type_and_raw_count() {
    assert_eq!(
        format!("{:?}", FixedU64::<100>::from_raw(123)),
        "FixedU64 { raw: 123, denominator: 100 }"
    );
    assert_eq!(
        format!("{:?}", FixedI64::<10>::from_raw(-12)),
        "FixedI64 { raw: -12, denominator: 10 }"
    );
}

#[cfg(feature = "serde")]
#[test]
fn serde_round_trips_raw_counts() {
    let unsigned = FixedU64::<100>::from_raw(u64::MAX);
    let signed = FixedI64::<10>::from_raw(i64::MIN);
    assert_eq!(
        serde_json::from_str::<FixedU64<100>>(&serde_json::to_string(&unsigned).unwrap())
            .unwrap()
            .raw(),
        unsigned.raw()
    );
    assert_eq!(
        serde_json::from_str::<FixedI64<10>>(&serde_json::to_string(&signed).unwrap())
            .unwrap()
            .raw(),
        signed.raw()
    );
}

#[cfg(feature = "fory")]
#[test]
fn fory_round_trips_raw_counts() {
    let mut fory = fory::Fory::builder().xlang(false).build();
    fory.register::<FixedU64<100>>(100).unwrap();
    fory.register::<FixedI64<10>>(101).unwrap();

    let unsigned = FixedU64::<100>::from_raw(u64::MAX);
    let signed = FixedI64::<10>::from_raw(i64::MIN);
    let unsigned_bytes = fory.serialize(&unsigned).unwrap();
    let signed_bytes = fory.serialize(&signed).unwrap();
    let decoded_unsigned: FixedU64<100> = fory.deserialize(&unsigned_bytes).unwrap();
    let decoded_signed: FixedI64<10> = fory.deserialize(&signed_bytes).unwrap();
    assert_eq!(decoded_unsigned.raw(), unsigned.raw());
    assert_eq!(decoded_signed.raw(), signed.raw());
}
