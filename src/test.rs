/// Defines a set of tests on a pairing engine / curve combination.
///
/// The set of tests is split in two. The first set between `[]` is for regular
/// tests that should not panic. The second set is for tests that should panic.

#[macro_export]
macro_rules! batch_test_field {
    ( [$($test_set:ident),*], [$($test_panic_set:ident),*] => ($engine:ty) ) => {
        paste::item! {
            $(
                #[test]
                #[allow(non_snake_case)]
                fn [< $test_set _on_ $engine>]() {
                    $test_set::<<$engine as ark_ec::PairingEngine>::Fr>()
                }
            )*
            $(
                #[test]
                #[should_panic]
                #[allow(non_snake_case)]
                fn [< $test_panic_set _on_ $engine>]() {
                    $test_panic_set::<<$engine as ark_ec::PairingEngine>::Fr>()
                }
            )*
        }
    }
}

#[macro_export]
macro_rules! batch_test_field_params {
    ( [$($test_set:ident),*], [$($test_panic_set:ident),*] => ($engine:ty, $params:ty) ) => {
        paste::item! {
            $(
                #[test]
                #[allow(non_snake_case)]
                fn [< $test_set _on_ $engine>]() {
                    $test_set::<<$engine as ark_ec::PairingEngine>::Fr, $params>()
                }
            )*
            $(
                #[test]
                #[should_panic]
                #[allow(non_snake_case)]
                fn [< $test_panic_set _on_ $engine>]() {
                    $test_panic_set::<<$engine as ark_ec::PairingEngine>::Fr, $params>()
                }
            )*
        }
    }
}

#[macro_export]
macro_rules! batch_test_engine {
    ( [$($test_set:ident),*], [$($test_panic_set:ident),*] => ($engine:ty) ) => {
        paste::item! {
            $(
                #[test]
                #[allow(non_snake_case)]
                fn [< $test_set _on_ $engine>]() {
                    $test_set::<$engine>()
                }
            )*
            $(
                #[test]
                #[should_panic]
                #[allow(non_snake_case)]
                fn [< $test_panic_set _on_ $engine>]() {
                    $test_panic_set::<$engine>()
                }
            )*
        }
    }
}

#[macro_export]
macro_rules! batch_test {
    ( [$($test_set:ident),*], [$($test_panic_set:ident),*] => ($engine:ty, $params:ty) ) => {
        paste::item! {
            $(
                #[test]
                #[allow(non_snake_case)]
                fn [< $test_set _on_ $engine>]() {
                    $test_set::<<$engine as PairingEngine>::Fr, $params, crate::commitment::KZG10<$engine>>()
                }
            )*
            $(
                #[test]
                #[should_panic]
                #[allow(non_snake_case)]
                fn [< $test_panic_set _on_ $engine>]() {
                    $test_panic_set::<<$engine as PairingEngine>::Fr, $params, crate::commitment::KZG10<$engine>>()
                }
            )*
        }
    }
}

#[macro_export]
macro_rules! batch_test_ipa {
    ( [$($test_set:ident),*], [$($test_panic_set:ident),*] => ($curve:ty, $params:ty) ) => {
        paste::item! {
            $(
                #[test]
                #[allow(non_snake_case)]
                fn [< $test_set _on_ $curve _ipa>]() {
                    $test_set::<<$curve as PairingEngine>::Fr, $params, ark_poly_commit::ipa_pc::InnerProductArgPC<<$curve as PairingEngine>::G1Affine, blake2::Blake2s, DensePolynomial<<$curve as PairingEngine>::Fr>>>()
                }
            )*
            $(
                #[test]
                #[should_panic]
                #[allow(non_snake_case)]
                fn [< $test_panic_set _on_ $curve _ipa>]() {
                    $test_panic_set::<<$curve as PairingEngine>::Fr, $params, ark_poly_commit::ipa_pc::InnerProductArgPC<<$curve as PairingEngine>::G1Affine, blake2::Blake2s, DensePolynomial<<$curve as PairingEngine>::Fr>>>()
                }
            )*
        }
    }
}
