use pi_fixed_scale_int::{ArithmeticError, FixedI64, FixedU64};

pub fn unsigned_raw_and_whole_two_have_distinct_counts() {
    let raw = FixedU64::<100>::from_raw(2);
    let whole = FixedU64::<100>::try_from_integer(2).expect("2 * 100 fits in u64");
    assert_eq!(raw.raw(), 2);
    assert_eq!(whole.raw(), 200);
    assert!(raw < whole);
}

pub fn signed_whole_integers_scale_with_sign() {
    for (integer, expected) in [(-2, -200), (0, 0), (2, 200)] {
        let value = FixedI64::<100>::try_from_integer(integer).expect("small integer fits");
        assert_eq!(value.raw(), expected);
    }
}

pub fn unsigned_whole_integer_at_scaling_limit_succeeds() {
    let integer = u64::MAX / 100;
    let value = FixedU64::<100>::try_from_integer(integer).expect("product fits in u64");
    assert_eq!(value.raw(), integer * 100);
}

pub fn unsigned_whole_integer_above_scaling_limit_overflows() {
    let integer = u64::MAX / 100 + 1;
    assert_eq!(
        FixedU64::<100>::try_from_integer(integer).map(|value| value.raw()),
        Err(ArithmeticError::Overflow)
    );
}

pub fn signed_whole_integers_at_scaling_limits_succeed() {
    for integer in [i64::MAX / 100, i64::MIN / 100] {
        let value = FixedI64::<100>::try_from_integer(integer).expect("product fits in i64");
        assert_eq!(value.raw(), integer * 100);
    }
}

pub fn signed_whole_integers_outside_scaling_limits_overflow() {
    for integer in [i64::MAX / 100 + 1, i64::MIN / 100 - 1] {
        assert_eq!(
            FixedI64::<100>::try_from_integer(integer).map(|value| value.raw()),
            Err(ArithmeticError::Overflow)
        );
    }
}

pub fn same_scale_values_compare_and_sort_by_count() {
    let two = FixedU64::<100>::from_raw(2);
    assert_eq!(two, FixedU64::<100>::from_raw(2));
    assert_ne!(two, FixedU64::<100>::from_raw(200));
    let mut unsigned = [FixedU64::<100>::from_raw(200), two, two];
    unsigned.sort();
    assert_eq!(unsigned.map(|value| value.raw()), [2, 2, 200]);

    let negative_two = FixedI64::<100>::from_raw(-2);
    assert_eq!(negative_two, FixedI64::<100>::from_raw(-2));
    assert_ne!(negative_two, FixedI64::<100>::from_raw(2));
    let mut signed = [2, -2, 0, -200].map(FixedI64::<100>::from_raw);
    signed.sort();
    assert_eq!(signed.map(|value| value.raw()), [-200, -2, 0, 2]);
}

pub fn different_scales_cannot_be_mixed_directly() {
    use std::io::Write;
    use std::process::{Command, Stdio};

    let dir = std::env::temp_dir().join(format!(
        "pi-fixed-scale-ticket-2-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system clock must be after epoch")
            .as_nanos()
    ));
    std::fs::create_dir(&dir).expect("create compiler output directory");
    let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let library = Command::new("rustc")
        .current_dir(manifest)
        .args(["--edition=2024", "--crate-name=pi_fixed_scale_int", "--crate-type=rlib"])
        .arg(manifest.join("src/lib.rs"))
        .arg("--out-dir")
        .arg(&dir)
        .output()
        .expect("rustc must be available to compile the public library");
    assert!(
        library.status.success(),
        "library compilation failed: {}",
        String::from_utf8_lossy(&library.stderr)
    );
    let extern_arg = format!(
        "pi_fixed_scale_int={}",
        dir.join("libpi_fixed_scale_int.rlib").display()
    );

    for (name, operation) in [
        ("unsigned_add", "FixedU64::<10>::from_raw(1).checked_add(FixedU64::<100>::from_raw(1))"),
        ("signed_add", "FixedI64::<10>::from_raw(-1).checked_add(FixedI64::<100>::from_raw(-1))"),
        ("unsigned_compare", "FixedU64::<10>::from_raw(1) < FixedU64::<100>::from_raw(1)"),
        ("signed_compare", "FixedI64::<10>::from_raw(-1) == FixedI64::<100>::from_raw(-1)"),
    ] {
        let source = format!(
            "use pi_fixed_scale_int::{{FixedU64, FixedI64}}; fn main() {{ let _ = {operation}; }}"
        );
        let mut child = Command::new("rustc")
            .args(["--edition=2024", "--crate-name=scale_mismatch_case", "--crate-type=bin", "--extern"])
            .arg(&extern_arg)
            .arg("--out-dir")
            .arg(&dir)
            .arg("-")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("start rustc for public API usage");
        child
            .stdin
            .take()
            .expect("rustc stdin")
            .write_all(source.as_bytes())
            .expect("write usage sample");
        let result = child.wait_with_output().expect("wait for rustc");
        let diagnostics = String::from_utf8_lossy(&result.stderr);
        assert!(!result.status.success(), "{name} unexpectedly compiled");
        assert!(
            diagnostics.contains("error[E0308]")
                && diagnostics.contains("expected")
                && diagnostics.contains("found"),
            "{name} failed for an unrelated reason: {diagnostics}"
        );
    }
    std::fs::remove_dir_all(&dir).expect("remove temporary compiler output directory");
}

pub fn zero_denominator_whole_integer_input_fails_to_compile() {
    use std::io::Write;
    use std::process::{Command, Stdio};

    let dir = std::env::temp_dir().join(format!(
        "pi-fixed-scale-ticket-2-zero-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system clock must be after epoch")
            .as_nanos()
    ));
    std::fs::create_dir(&dir).expect("create compiler output directory");
    let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let library = Command::new("rustc")
        .current_dir(manifest)
        .args(["--edition=2024", "--crate-name=pi_fixed_scale_int", "--crate-type=rlib"])
        .arg(manifest.join("src/lib.rs"))
        .arg("--out-dir")
        .arg(&dir)
        .output()
        .expect("rustc must be available to compile the public library");
    assert!(
        library.status.success(),
        "library compilation failed: {}",
        String::from_utf8_lossy(&library.stderr)
    );
    let extern_arg = format!(
        "pi_fixed_scale_int={}",
        dir.join("libpi_fixed_scale_int.rlib").display()
    );

    for (name, operation) in [
        ("unsigned", "FixedU64::<0>::try_from_integer(2)"),
        ("signed", "FixedI64::<0>::try_from_integer(2)"),
    ] {
        let source = format!(
            "use pi_fixed_scale_int::{{FixedU64, FixedI64}}; fn main() {{ let _ = {operation}; }}"
        );
        let mut child = Command::new("rustc")
            .args(["--edition=2024", "--crate-name=zero_integer_case", "--crate-type=bin", "--extern"])
            .arg(&extern_arg)
            .arg("--out-dir")
            .arg(&dir)
            .arg("-")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("start rustc for public API usage");
        child
            .stdin
            .take()
            .expect("rustc stdin")
            .write_all(source.as_bytes())
            .expect("write usage sample");
        let result = child.wait_with_output().expect("wait for rustc");
        let diagnostics = String::from_utf8_lossy(&result.stderr);
        assert!(!result.status.success(), "{name} unexpectedly compiled");
        assert!(
            diagnostics.contains("denominator must be positive"),
            "{name} failed for an unrelated reason: {diagnostics}"
        );
    }
    std::fs::remove_dir_all(&dir).expect("remove temporary compiler output directory");
}
