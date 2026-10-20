use pi_fixed_scale_int::{
    ArithmeticError, FixedI64, FixedU64, SCALE_10, SCALE_100, SCALE_1000, SCALE_10000, SCALE_100000,
    SCALE_1000000, SCALE_10000000, SCALE_100000000, SCALE_1371600,
};

pub fn raw_123_at_scale_100_represents_1_23() {
    let value = FixedU64::<100>::from_raw(123);
    assert_eq!(value.raw(), 123);
    assert_eq!(value.raw() as f64 / 100.0, 1.23);
}

pub fn convenient_scales_match_custom_scales() {
    macro_rules! check_scale {
        ($scale:ident, $expected:literal) => {
            assert_eq!($scale, $expected);
            assert_eq!(
                FixedU64::<{ $scale }>::from_raw(123).raw(),
                FixedU64::<$expected>::from_raw(123).raw()
            );
        };
    }
    check_scale!(SCALE_10, 10);
    check_scale!(SCALE_100, 100);
    check_scale!(SCALE_1000, 1_000);
    check_scale!(SCALE_10000, 10_000);
    check_scale!(SCALE_100000, 100_000);
    check_scale!(SCALE_1000000, 1_000_000);
    check_scale!(SCALE_10000000, 10_000_000);
    check_scale!(SCALE_100000000, 100_000_000);
    check_scale!(SCALE_1371600, 1_371_600);
}

pub fn max_u32_denominator_is_usable() {
    assert_eq!(FixedU64::<{ u32::MAX }>::from_raw(123).raw(), 123);
    assert_eq!(FixedI64::<{ u32::MAX }>::from_raw(-123).raw(), -123);
}

pub fn unsigned_raw_count_boundaries_round_trip() {
    for raw in [0, u64::MAX] {
        assert_eq!(FixedU64::<100>::from_raw(raw).raw(), raw);
    }
}

pub fn signed_raw_count_boundaries_round_trip() {
    for raw in [i64::MIN, i64::MAX] {
        assert_eq!(FixedI64::<100>::from_raw(raw).raw(), raw);
    }
}

pub fn arithmetic_error_variants_are_public_and_distinct() {
    let overflow: ArithmeticError = ArithmeticError::Overflow;
    let division_by_zero: ArithmeticError = ArithmeticError::DivisionByZero;
    assert_ne!(overflow, division_by_zero);
}

pub fn zero_denominator_public_operations_fail_to_compile() {
    use std::io::Write;
    use std::process::{Command, Stdio};

    let dir = std::env::temp_dir().join(format!(
        "pi-fixed-scale-test-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system clock must be after epoch")
            .as_nanos()
    ));
    std::fs::create_dir(&dir).expect("create temporary compiler output directory");
    let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let library = Command::new("rustc")
        .current_dir(manifest)
        .args(["--edition=2024", "--crate-name=pi_fixed_scale_int", "--crate-type=rlib"])
        .arg(manifest.join("src/lib.rs"))
        .arg("--out-dir")
        .arg(&dir)
        .output()
        .expect("rustc must be available to compile the public library");
    assert!(library.status.success(), "library compilation failed: {}", String::from_utf8_lossy(&library.stderr));
    let extern_arg = format!(
        "pi_fixed_scale_int={}",
        dir.join("libpi_fixed_scale_int.rlib").display()
    );

    for (name, operation) in [
        ("construct", "let _ = FixedU64::<0>::from_raw(1);"),
        // A zeroed integer count is a valid representation for the frozen value types.
        // Bypass from_raw only in these samples so raw/eq must fail on their own.
        ("read", "let value: FixedI64<0> = unsafe { std::mem::zeroed() }; let _ = value.raw();"),
        ("compare", "let left: FixedU64<0> = unsafe { std::mem::zeroed() }; let right: FixedU64<0> = unsafe { std::mem::zeroed() }; let _ = left == right;"),
        ("rescale", "let _ = FixedU64::<10>::from_raw(1).rescale::<0>();"),
    ] {
        let source = format!(
            "use pi_fixed_scale_int::{{FixedI64, FixedU64}}; fn main() {{ {operation} }}"
        );
        let mut child = Command::new("rustc")
            .args(["--edition=2024", "--crate-name=zero_denominator_case", "--crate-type=bin", "--extern"])
            .arg(&extern_arg)
            .arg("--out-dir")
            .arg(&dir)
            .arg("-")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("start rustc for public API usage");
        child.stdin.take().expect("rustc stdin").write_all(source.as_bytes()).expect("write usage sample");
        let result = child.wait_with_output().expect("wait for rustc");
        let diagnostics = String::from_utf8_lossy(&result.stderr);
        assert!(!result.status.success(), "{name} unexpectedly compiled");
        assert!(diagnostics.contains("denominator must be positive") || diagnostics.contains("denominators must be positive"), "{name} failed for an unrelated reason: {diagnostics}");
    }
    std::fs::remove_dir_all(&dir).expect("remove temporary compiler output directory");
}
