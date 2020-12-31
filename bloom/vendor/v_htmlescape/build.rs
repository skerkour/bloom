use std::env;

fn main() {
    enable_simd_optimizations();
}

fn enable_simd_optimizations() {
    if is_env_set("CARGO_CFG_HTMLESCAPE_DISABLE_AUTO_SIMD") {
        return;
    }

    println!("cargo:rustc-cfg=v_htmlescape_simd");
    println!("cargo:rustc-cfg=v_htmlescape_sse");

    if !is_env_set("CARGO_CFG_HTMLESCAPE_DISABLE_AUTO_AVX") {
        println!("cargo:rustc-cfg=v_htmlescape_avx");
    }
}

fn is_env_set(name: &str) -> bool {
    env::var(name).is_ok()
}
