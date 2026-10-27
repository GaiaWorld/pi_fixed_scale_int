use pi_fixed_scale_int::{FixedI64, FixedU64, MathError};

// @api:src/lib.rs
// @impl:src/math_inner.rs
// @test:tests/ticket_13_test.rs
// @design-doc:design/pi-fixed-scale-int/07-design/call-chain.md 切片13
// Expected values use exact identities and binary-exact rational powers, not another math tier.

pub fn each_fast_method_rejects_zero_denominator() {
    use std::{fs, process::Command};
    struct Scratch(std::path::PathBuf);
    impl Drop for Scratch {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }
    let dir = Scratch(std::env::temp_dir().join(format!(
            "pi-fast-zero-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        )));
    fs::create_dir_all(dir.0.join("src")).unwrap();
    let root = env!("CARGO_MANIFEST_DIR").replace('\\', "/");
    fs::write(dir.0.join("Cargo.toml"), format!(
        "[package]\nname = \"fast_zero_probe\"\nversion = \"0.0.0\"\nedition = \"2024\"\n[dependencies]\npi_fixed_scale_int = {{ path = \"{root}\", features = [\"math\"] }}\n"
    )).unwrap();
    for category in ["FixedI64", "FixedU64"] {
        for call in [
            "fast_exp()",
            "fast_exp2()",
            "fast_ln()",
            "fast_log2()",
            "fast_log10()",
            "fast_log(x)",
            "fast_powi(-1)",
        ] {
            for d in [1, 0] {
                // These types store an integer only: zeroed is a valid representation.
                // Do not call from_raw/raw: the method under test must reject D itself.
                let source = format!(
                    "use pi_fixed_scale_int::{category}; fn main() {{ let x: {category}<{d}> = unsafe {{ std::mem::zeroed() }}; let _ = x.{call}; }}"
                );
                fs::write(dir.0.join("src/main.rs"), source).unwrap();
                let output = Command::new(env!("CARGO"))
                    .args(["build", "--offline", "--quiet", "--manifest-path"])
                    .arg(dir.0.join("Cargo.toml"))
                    .arg("--target-dir")
                    .arg(dir.0.join("target"))
                    .output()
                    .expect("run compiler probe");
                let diagnostics = String::from_utf8_lossy(&output.stderr);
                if d == 1 {
                    assert!(
                        output.status.success(),
                        "positive control {category}.{call}: {diagnostics}"
                    );
                } else {
                    assert!(!output.status.success(), "{category}.{call} accepted D0");
                    assert!(
                        diagnostics.contains("E0080")
                            && diagnostics.contains("denominator must be positive"),
                        "{category}.{call} failed for an unrelated reason: {diagnostics}"
                    );
                }
            }
        }
    }
}

pub fn seven_methods_preserve_type_and_scale() {
    macro_rules! check {
        ($ty:ident) => {{
            let z = $ty::<100>::from_raw(0);
            let one = $ty::<100>::from_raw(100);
            let two = $ty::<100>::from_raw(200);
            let eight = $ty::<100>::from_raw(800);
            let ten = $ty::<100>::from_raw(1000);
            let hundred = $ty::<100>::from_raw(10000);
            let results: [Result<$ty<100>, MathError>; 7] = [
                z.fast_exp(),
                z.fast_exp2(),
                one.fast_ln(),
                eight.fast_log2(),
                hundred.fast_log10(),
                hundred.fast_log(ten),
                two.fast_powi(-1),
            ];
            for (result, expected) in results.into_iter().zip([100, 100, 0, 300, 200, 200, 50]) {
                assert_eq!(result.map(|v| v.raw()), Ok(expected));
            }
            assert_eq!(
                [
                    z.raw(),
                    one.raw(),
                    two.raw(),
                    eight.raw(),
                    ten.raw(),
                    hundred.raw()
                ],
                [0, 100, 200, 800, 1000, 10000]
            );
        }};
    }
    check!(FixedI64);
    check!(FixedU64);
}

pub fn nondecimal_scales_and_maximum_denominator() {
    macro_rules! check {
        ($ty:ident) => {{
            assert_eq!($ty::<3>::from_raw(0).fast_exp().map(|v| v.raw()), Ok(3));
            assert_eq!($ty::<3>::from_raw(1).fast_exp().map(|v| v.raw()), Ok(4));
            // 1/2 * 3 = 1.5 exactly: final half-away quantization yields 2.
            assert_eq!($ty::<3>::from_raw(6).fast_powi(-1).map(|v| v.raw()), Ok(2));
            assert_eq!(
                $ty::<{ u32::MAX }>::from_raw(0).fast_exp().map(|v| v.raw()),
                Ok(u32::MAX as _)
            );
        }};
    }
    check!(FixedI64);
    check!(FixedU64);
}

pub fn binary_exact_half_ties_round_away() {
    for (raw, n, expected) in [
        (15, 2, 23),
        (-15, 2, 23),
        (15, 3, 34),
        (-15, 3, -34),
        (20, -2, 3),
        (-20, -2, 3),
        (20, -3, 1),
        (-20, -3, -1),
    ] {
        let x = FixedI64::<10>::from_raw(raw);
        assert_eq!(
            x.fast_powi(n).map(|v| v.raw()),
            Ok(expected),
            "raw={raw}, n={n}"
        );
        assert_eq!(x.raw(), raw);
        if raw > 0 {
            assert_eq!(
                FixedU64::<10>::from_raw(raw as u64)
                    .fast_powi(n)
                    .map(|v| v.raw()),
                Ok(expected as u64)
            );
        }
    }
}

pub fn zero_powers_and_minimum_exponent() {
    for raw in [0, 1, 100, 200, u64::MAX] {
        let x = FixedU64::<100>::from_raw(raw);
        assert_eq!(x.fast_powi(0).map(|v| v.raw()), Ok(100));
        assert_eq!(x.raw(), raw);
    }
    for raw in [i64::MIN, -100, 0, 100, i64::MAX] {
        let x = FixedI64::<100>::from_raw(raw);
        assert_eq!(x.fast_powi(0).map(|v| v.raw()), Ok(100));
        assert_eq!(x.raw(), raw);
    }
    for raw in [-100, 100] {
        assert_eq!(
            FixedI64::<100>::from_raw(raw)
                .fast_powi(i32::MIN)
                .map(|v| v.raw()),
            Ok(100)
        );
    }
    assert_eq!(
        FixedU64::<100>::from_raw(100)
            .fast_powi(i32::MIN)
            .map(|v| v.raw()),
        Ok(100)
    );
    assert_eq!(
        FixedI64::<100>::from_raw(-100)
            .fast_powi(i32::MAX)
            .map(|v| v.raw()),
        Ok(-100)
    );
}

pub fn invalid_log_inputs_are_domain_errors() {
    for raw in [-100, 0] {
        let x = FixedI64::<100>::from_raw(raw);
        for result in [
            x.fast_ln(),
            x.fast_log2(),
            x.fast_log10(),
            x.fast_log(FixedI64::from_raw(200)),
        ] {
            assert_eq!(result, Err(MathError::Domain));
        }
        assert_eq!(x.raw(), raw);
    }
    let x = FixedU64::<100>::from_raw(0);
    for result in [
        x.fast_ln(),
        x.fast_log2(),
        x.fast_log10(),
        x.fast_log(FixedU64::from_raw(200)),
    ] {
        assert_eq!(result, Err(MathError::Domain));
    }
    assert_eq!(x.raw(), 0);
}

pub fn invalid_bases_are_domain_errors() {
    for raw in [-200, 0, 100] {
        let x = FixedI64::<100>::from_raw(200);
        let base = FixedI64::from_raw(raw);
        assert_eq!(x.fast_log(base), Err(MathError::Domain));
        assert_eq!((x.raw(), base.raw()), (200, raw));
    }
    for raw in [0, 100] {
        let x = FixedU64::<100>::from_raw(200);
        let base = FixedU64::from_raw(raw);
        assert_eq!(x.fast_log(base), Err(MathError::Domain));
        assert_eq!((x.raw(), base.raw()), (200, raw));
    }
}

pub fn zero_negative_powers_are_domain_errors() {
    for n in [-1, i32::MIN] {
        let s = FixedI64::<100>::from_raw(0);
        let u = FixedU64::<100>::from_raw(0);
        assert_eq!(s.fast_powi(n), Err(MathError::Domain));
        assert_eq!(u.fast_powi(n), Err(MathError::Domain));
        assert_eq!((s.raw(), u.raw()), (0, 0));
    }
}

pub fn signed_logarithms_cover_base_quadrants() {
    for (raw, base_raw, expected) in [
        (400, 200, 200),
        (400, 50, -200),
        (25, 200, -200),
        (25, 50, 200),
    ] {
        let x = FixedI64::<100>::from_raw(raw);
        let base = FixedI64::from_raw(base_raw);
        assert_eq!(x.fast_log(base).map(|v| v.raw()), Ok(expected));
        assert_eq!((x.raw(), base.raw()), (raw, base_raw));
    }
}

pub fn unsigned_negative_quantizes_before_range_check() {
    let x = FixedU64::<2>::from_raw(1);
    let base = FixedU64::<2>::from_raw(200);
    assert_eq!(x.fast_log(base).map(|v| v.raw()), Ok(0));
    assert_eq!((x.raw(), base.raw()), (1, 200));
    let half = FixedU64::<100>::from_raw(50);
    assert_eq!(half.fast_ln(), Err(MathError::Overflow));
    assert_eq!(half.raw(), 50);
}

pub fn storage_edges_survive_identity_power() {
    fn check<const D: u32>() {
        for raw in [i64::MIN, i64::MAX] {
            let x = FixedI64::<D>::from_raw(raw);
            assert_eq!(x.fast_powi(1).map(|v| v.raw()), Ok(raw));
            assert_eq!(x.raw(), raw);
        }
        let x = FixedU64::<D>::from_raw(u64::MAX);
        assert_eq!(x.fast_powi(1).map(|v| v.raw()), Ok(u64::MAX));
        assert_eq!(x.raw(), u64::MAX);
    }
    check::<1>();
    check::<{ u32::MAX }>();
}

pub fn proven_exponential_extremes() {
    let s = FixedI64::<1>::from_raw(101);
    let u = FixedU64::<1>::from_raw(101);
    assert_eq!(s.fast_exp(), Err(MathError::Overflow));
    assert_eq!(s.fast_exp2(), Err(MathError::Overflow));
    assert_eq!(u.fast_exp(), Err(MathError::Overflow));
    assert_eq!(u.fast_exp2(), Err(MathError::Overflow));
    let neg = FixedI64::<1>::from_raw(-101);
    assert_eq!(neg.fast_exp().map(|v| v.raw()), Ok(0));
    assert_eq!(neg.fast_exp2().map(|v| v.raw()), Ok(0));
    assert_eq!((s.raw(), u.raw(), neg.raw()), (101, 101, -101));
}

pub fn finite_powers_outside_storage_overflow() {
    let s = FixedI64::<1>::from_raw(2);
    let u = FixedU64::<1>::from_raw(2);
    let neg = FixedI64::<1>::from_raw(-2);
    assert_eq!(s.fast_powi(64), Err(MathError::Overflow));
    assert_eq!(u.fast_powi(64), Err(MathError::Overflow));
    assert_eq!(neg.fast_powi(65), Err(MathError::Overflow));
    assert_eq!((s.raw(), u.raw(), neg.raw()), (2, 2, -2));
}

pub fn invalid_large_intermediate_is_not_reciprocated_to_zero() {
    // (2^63-1)^(2^31) and (2^64-1)^(2^31) exceed the backend exponent range.
    let s = FixedI64::<1>::from_raw(i64::MAX);
    let u = FixedU64::<1>::from_raw(u64::MAX);
    assert_eq!(s.fast_powi(i32::MIN), Err(MathError::ComputationFailed));
    assert_eq!(u.fast_powi(i32::MIN), Err(MathError::ComputationFailed));
    assert_eq!((s.raw(), u.raw()), (i64::MAX, u64::MAX));
}
