use pi_fixed_scale_int::{ArithmeticError, FixedI64, FixedU64};

pub fn rescale_half_ties() {
    let unsigned = FixedU64::<100>::from_raw(125);
    assert_eq!(unsigned.rescale::<10>().expect("fits").raw(), 12);
    assert_eq!(unsigned.rescale_rounded::<10>().expect("fits").raw(), 13);
    assert_eq!(unsigned.raw(), 125);
    for (raw, truncated, rounded) in [(125, 12, 13), (-125, -12, -13), (-5, 0, -1)] {
        let signed = FixedI64::<100>::from_raw(raw);
        assert_eq!(signed.rescale::<10>().expect("fits").raw(), truncated);
        assert_eq!(signed.rescale_rounded::<10>().expect("fits").raw(), rounded);
        assert_eq!(signed.raw(), raw);
    }
}

pub fn rescale_around_half_and_exact() {
    for (raw, truncated, rounded) in [(124, 12, 12), (126, 12, 13), (120, 12, 12)] {
        let unsigned = FixedU64::<100>::from_raw(raw);
        assert_eq!(unsigned.rescale::<10>().expect("fits").raw(), truncated);
        assert_eq!(unsigned.rescale_rounded::<10>().expect("fits").raw(), rounded);
        assert_eq!(unsigned.raw(), raw);
    }
    for (raw, truncated, rounded) in [(-124, -12, -12), (-126, -12, -13), (-120, -12, -12)] {
        let signed = FixedI64::<100>::from_raw(raw);
        assert_eq!(signed.rescale::<10>().expect("fits").raw(), truncated);
        assert_eq!(signed.rescale_rounded::<10>().expect("fits").raw(), rounded);
        assert_eq!(signed.raw(), raw);
    }
    let unsigned = FixedU64::<{ u32::MAX }>::from_raw(1);
    assert_eq!(unsigned.rescale::<{ u32::MAX }>().expect("fits").raw(), 1);
    assert_eq!(unsigned.rescale_rounded::<{ u32::MAX }>().expect("fits").raw(), 1);
    let signed = FixedI64::<{ u32::MAX }>::from_raw(-1);
    assert_eq!(signed.rescale::<{ u32::MAX }>().expect("fits").raw(), -1);
    assert_eq!(signed.rescale_rounded::<{ u32::MAX }>().expect("fits").raw(), -1);
    assert_eq!((unsigned.raw(), signed.raw()), (1, -1));
}

pub fn rescale_to_larger_denominator() {
    let unsigned = FixedU64::<10>::from_raw(12);
    assert_eq!(unsigned.rescale::<100>().expect("fits").raw(), 120);
    assert_eq!(unsigned.rescale_rounded::<100>().expect("fits").raw(), 120);
    assert_eq!(unsigned.raw(), 12);

    for (raw, expected) in [(12, 120), (-12, -120)] {
        let signed = FixedI64::<10>::from_raw(raw);
        assert_eq!(signed.rescale::<100>().expect("fits").raw(), expected);
        assert_eq!(signed.rescale_rounded::<100>().expect("fits").raw(), expected);
        assert_eq!(signed.raw(), raw);
    }
}

pub fn rescale_overflows() {
    let unsigned = FixedU64::<1>::from_raw(u64::MAX);
    assert_eq!(unsigned.rescale::<2>().map(|v| v.raw()), Err(ArithmeticError::Overflow));
    assert_eq!(unsigned.rescale_rounded::<2>().map(|v| v.raw()), Err(ArithmeticError::Overflow));
    assert_eq!(unsigned.raw(), u64::MAX);
    for raw in [i64::MAX, i64::MIN] {
        let signed = FixedI64::<1>::from_raw(raw);
        assert_eq!(signed.rescale::<2>().map(|v| v.raw()), Err(ArithmeticError::Overflow));
        assert_eq!(signed.rescale_rounded::<2>().map(|v| v.raw()), Err(ArithmeticError::Overflow));
        assert_eq!(signed.raw(), raw);
    }
}

pub fn rescale_rounding_crosses_bound() {
    let unsigned = FixedU64::<2>::from_raw(1_190_112_520_884_487_201);
    assert_eq!(unsigned.rescale::<31>().expect("truncation fits").raw(), u64::MAX);
    assert_eq!(unsigned.rescale_rounded::<31>().map(|v| v.raw()), Err(ArithmeticError::Overflow));
    assert_eq!(unsigned.raw(), 1_190_112_520_884_487_201);

    let positive = FixedI64::<2>::from_raw(6_148_914_691_236_517_205);
    assert_eq!(positive.rescale::<3>().expect("truncation fits").raw(), i64::MAX);
    assert_eq!(positive.rescale_rounded::<3>().map(|v| v.raw()), Err(ArithmeticError::Overflow));
    assert_eq!(positive.raw(), 6_148_914_691_236_517_205);

    let negative = FixedI64::<4>::from_raw(-7_378_697_629_483_820_647);
    assert_eq!(negative.rescale::<5>().expect("truncation fits").raw(), i64::MIN);
    assert_eq!(negative.rescale_rounded::<5>().map(|v| v.raw()), Err(ArithmeticError::Overflow));
    assert_eq!(negative.raw(), -7_378_697_629_483_820_647);
}

pub fn zero_target_denominator_fails_to_compile() {
    assert_zero_denominator_compile_failure(&[
        ("unsigned truncate", "FixedU64::<10>::from_raw(1).rescale::<0>()", "denominators must be positive"),
        ("unsigned rounded", "FixedU64::<10>::from_raw(1).rescale_rounded::<0>()", "denominators must be positive"),
        ("signed truncate", "FixedI64::<10>::from_raw(-1).rescale::<0>()", "denominators must be positive"),
        ("signed rounded", "FixedI64::<10>::from_raw(-1).rescale_rounded::<0>()", "denominators must be positive"),
    ]);
}

pub fn zero_source_rescale_fails_to_compile() {
    assert_zero_denominator_compile_failure(&[
        ("unsigned truncate", "unsafe { std::mem::zeroed::<FixedU64<0>>() }.rescale::<10>()", "denominators must be positive"),
        ("unsigned rounded", "unsafe { std::mem::zeroed::<FixedU64<0>>() }.rescale_rounded::<10>()", "denominators must be positive"),
        ("signed truncate", "unsafe { std::mem::zeroed::<FixedI64<0>>() }.rescale::<10>()", "denominators must be positive"),
        ("signed rounded", "unsafe { std::mem::zeroed::<FixedI64<0>>() }.rescale_rounded::<10>()", "denominators must be positive"),
    ]);
}

pub fn zero_source_integer_output_fails_to_compile() {
    assert_zero_denominator_compile_failure(&[
        ("unsigned truncate", "unsafe { std::mem::zeroed::<FixedU64<0>>() }.to_integer()", "denominator must be positive"),
        ("unsigned rounded", "unsafe { std::mem::zeroed::<FixedU64<0>>() }.to_integer_rounded()", "denominator must be positive"),
        ("signed truncate", "unsafe { std::mem::zeroed::<FixedI64<0>>() }.to_integer()", "denominator must be positive"),
        ("signed rounded", "unsafe { std::mem::zeroed::<FixedI64<0>>() }.to_integer_rounded()", "denominator must be positive"),
    ]);
}

fn assert_zero_denominator_compile_failure(cases: &[(&str, &str, &str)]) {
    use std::io::Write;
    use std::process::{Command, Stdio};

    let dir = std::env::temp_dir().join(format!(
        "pi-fixed-scale-ticket-6-{}-{}",
        std::process::id(),
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)
            .expect("system clock must be after epoch").as_nanos()
    ));
    std::fs::create_dir(&dir).expect("create compiler output directory");
    let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let library = Command::new("rustc")
        .current_dir(manifest)
        .args(["--edition=2024", "--crate-name=pi_fixed_scale_int", "--crate-type=rlib"])
        .arg(manifest.join("src/lib.rs"))
        .arg("--out-dir").arg(&dir)
        .output().expect("compile public library");
    assert!(library.status.success(), "library compilation failed: {}", String::from_utf8_lossy(&library.stderr));
    let extern_arg = format!("pi_fixed_scale_int={}", dir.join("libpi_fixed_scale_int.rlib").display());
    for &(name, operation, expected_diagnostic) in cases {
        let source = format!("use pi_fixed_scale_int::{{FixedI64, FixedU64}}; fn main() {{ let _ = {operation}; }}");
        let mut child = Command::new("rustc")
            .args(["--edition=2024", "--crate-name=zero_target_case", "--crate-type=bin", "--extern"])
            .arg(&extern_arg).arg("--out-dir").arg(&dir).arg("-")
            .stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped())
            .spawn().expect("start rustc for public API usage");
        child.stdin.take().expect("rustc stdin").write_all(source.as_bytes()).expect("write usage sample");
        let result = child.wait_with_output().expect("wait for rustc");
        let diagnostics = String::from_utf8_lossy(&result.stderr);
        assert!(!result.status.success(), "{name} unexpectedly compiled");
        assert!(diagnostics.contains(expected_diagnostic), "{name} failed for an unrelated reason: {diagnostics}");
    }
    std::fs::remove_dir_all(&dir).expect("remove compiler output directory");
}

pub fn integer_output_half_ties() {
    let unsigned = FixedU64::<10>::from_raw(15);
    assert_eq!(unsigned.to_integer(), 1);
    assert_eq!(unsigned.to_integer_rounded(), 2);
    assert_eq!(unsigned.raw(), 15);
    for (raw, truncated, rounded) in [(15, 1, 2), (-15, -1, -2), (-5, 0, -1)] {
        let signed = FixedI64::<10>::from_raw(raw);
        assert_eq!(signed.to_integer(), truncated);
        assert_eq!(signed.to_integer_rounded(), rounded);
        assert_eq!(signed.raw(), raw);
    }
}

pub fn integer_output_around_half_and_exact() {
    for (raw, truncated, rounded) in [(14, 1, 1), (16, 1, 2), (20, 2, 2)] {
        let unsigned = FixedU64::<10>::from_raw(raw);
        assert_eq!(unsigned.to_integer(), truncated);
        assert_eq!(unsigned.to_integer_rounded(), rounded);
        assert_eq!(unsigned.raw(), raw);
    }
    for (raw, truncated, rounded) in [(-14, -1, -1), (-16, -1, -2), (-20, -2, -2)] {
        let signed = FixedI64::<10>::from_raw(raw);
        assert_eq!(signed.to_integer(), truncated);
        assert_eq!(signed.to_integer_rounded(), rounded);
        assert_eq!(signed.raw(), raw);
    }
}

pub fn integer_output_extremes() {
    let unsigned = FixedU64::<1>::from_raw(u64::MAX);
    assert_eq!(unsigned.to_integer(), u64::MAX);
    assert_eq!(unsigned.to_integer_rounded(), u64::MAX);
    assert_eq!(unsigned.raw(), u64::MAX);
    for raw in [i64::MIN, i64::MAX] {
        let signed = FixedI64::<1>::from_raw(raw);
        assert_eq!(signed.to_integer(), raw);
        assert_eq!(signed.to_integer_rounded(), raw);
        assert_eq!(signed.raw(), raw);
    }
}
