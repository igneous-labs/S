#[macro_export]
macro_rules! prop_assert_diff_at_most {
    ($a: expr, $b: expr, $at_most: expr) => {{
        let absdiff = if $a > $b { $a - $b } else { $b - $a };
        proptest::prop_assert!(
            absdiff <= $at_most,
            "a = {:?}, b = {:?}, absdiff = {:?}",
            $a,
            $b,
            absdiff
        )
    }};
}
