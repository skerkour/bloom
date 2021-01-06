fn main() {
    let ac = autocfg::new();

    for root in &["core", "std"] {
        for size in &[8, 16, 32, 64, 128] {
            ac.emit_expression_cfg(
                &format!("{}::sync::atomic::AtomicU{}::compare_exchange", root, size),
                &format!("has_atomic_u{}", size),
            );
            ac.emit_expression_cfg(
                &format!("{}::sync::atomic::AtomicI{}::compare_exchange", root, size),
                &format!("has_atomic_i{}", size),
            );
        }
    }
}
