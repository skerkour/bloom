/// Generate ranges avx2 implementation
///
/// ## Following macros must be defined
/// - `fallback!()`
///     when length is less than 32
/// - `write_mask!(mut $mask: {integer}, $ptr: *const u8)`
///     when bit mask is non equal 0
/// - `write_forward(mut $mask: {integer}, $until: usize)`
///     when bit mask is non equal 0  and valid bits until
///
#[macro_export]
macro_rules! loop_range_switch_avx2  {
    (($len:ident, $ptr:ident, $start_ptr:ident, $end_ptr:ident) $($t:tt, )+) => {
        use std::arch::x86_64::{
            __m256i, _mm256_load_si256, _mm256_loadu_si256, _mm256_movemask_epi8, _mm256_or_si256,
        };

        const M256_VECTOR_SIZE: usize = std::mem::size_of::<__m256i>();

        if $len < M256_VECTOR_SIZE {
            $crate::loop_range_switch_sse2!(($len, $ptr, $start_ptr, $end_ptr) $($t, )+);
        } else {
            $crate::translations_256!($($t, )+);

            // Aligning pointer by using `_mm256_loadu_si256` on unaligned bytes.
            {
                const M256_VECTOR_ALIGN: usize = M256_VECTOR_SIZE - 1;
                let align = M256_VECTOR_SIZE - ($start_ptr as usize & M256_VECTOR_ALIGN);
                if align < M256_VECTOR_SIZE {
                    let mut mask = {
                        let a = _mm256_loadu_si256($ptr as *const __m256i);
                        _mm256_movemask_epi8(masking!(a))
                    };

                    if mask != 0 {
                        write_forward!(mask, align);
                    }
                    // Aligning pointer
                    $ptr = $ptr.add(align);
                }
            }

            $crate::avx_main_loop!(($len, $ptr, $end_ptr) $($t, )+);

            // When the rest of string has a $length greater then `M256_VECTOR_SIZE`
            // but less than `LOOP_SIZE`, we process it `M256_VECTOR_SIZE` bits at
            // a time until there are left less then `M256_VECTOR_SIZE` elements
            while $ptr <= $end_ptr.sub(M256_VECTOR_SIZE) {
                debug_assert_eq!(0, ($ptr as usize) % M256_VECTOR_SIZE);

                let mut mask = {
                    let a = _mm256_load_si256($ptr as *const __m256i);
                    _mm256_movemask_epi8(masking!(a))
                };

                if mask != 0 {
                    write_mask!(mask, $ptr);
                }
                $ptr = $ptr.add(M256_VECTOR_SIZE);
            }

            debug_assert!($end_ptr.sub(M256_VECTOR_SIZE) < $ptr);

            // At this point at most there is less then `M256_VECTOR_SIZE` elements
            // so the macro `write_forward` is used to finalize de process
            if $ptr < $end_ptr {
                let d = M256_VECTOR_SIZE - $crate::sub!($end_ptr, $ptr);

                let mut mask = ({
                    debug_assert_eq!(M256_VECTOR_SIZE, $crate::sub!($end_ptr, $ptr.sub(d)), "Over runs");
                    let a = _mm256_loadu_si256($ptr.sub(d) as *const __m256i);
                    _mm256_movemask_epi8(masking!(a))
                } as u32).wrapping_shr(d as u32);

                if mask != 0 {
                    write_mask!(mask, $ptr);
                }
            }
        }
    };
}
