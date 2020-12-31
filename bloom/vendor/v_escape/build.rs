fn main() {
    enable_simd_optimizations();
}

fn enable_simd_optimizations() {
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("sse2") {
            println!("cargo:rustc-cfg=v_escape_sse");
        }

        if is_x86_feature_detected!("avx2") {
            println!("cargo:rustc-cfg=v_escape_avx");
        }
    }
}
