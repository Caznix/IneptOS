macro_rules! arch_cond {
    ($($arch:ident: $str:literal),* $(,)?) => {$(
        #[cfg(target_arch = $str)]
        pub mod $arch;
        #[cfg(target_arch = $str)]
        pub use self::$arch::*;
    )*};
}

arch_cond!(
    aarch64: "aarch64",
    riscv64: "riscv64",
    x86_64: "x86_64",
);
