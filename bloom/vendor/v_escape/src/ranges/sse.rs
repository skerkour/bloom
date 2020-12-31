/// Generate ranges sse2 implementation
///
/// ## Following macros must be defined
/// - `fallback!()`
///     when length is less than 16
/// - `write_mask!(mut $mask: {integer}, $ptr: *const u8)`
///     when bit mask is non equal 0
/// - `write_forward(mut $mask: {integer}, $until: usize)`
///     when bit mask is non equal 0  and valid bits until
///
#[macro_export]
macro_rules! loop_range_switch_sse2  {
    (($len:ident, $ptr:ident, $start_ptr:ident, $end_ptr:ident) $($t:tt, )+) => {
        use std::arch::x86_64::{__m128i, _mm_load_si128, _mm_loadu_si128, _mm_movemask_epi8};

        const M128_VECTOR_SIZE: usize = std::mem::size_of::<__m128i>();
        const M128_VECTOR_ALIGN: usize = M128_VECTOR_SIZE - 1;

        if $len < M128_VECTOR_SIZE {
            fallback!();
        } else {
            $crate::translations_128!($($t, )+);

            // Write mask for unaligned elements from the start
            // of the vector and aligning pointer
            {
                // Calculating index of aligned pointer
                let align = M128_VECTOR_SIZE - ($start_ptr as usize & M128_VECTOR_ALIGN);
                if align < M128_VECTOR_SIZE {
                    let mut mask = {
                        let a = _mm_loadu_si128($ptr as *const __m128i);
                        _mm_movemask_epi8(masking!(a))
                    };
                    // Writing mask for unaligned elements
                    if mask != 0 {
                        write_forward!(mask, align);
                    }
                    // Aligning pointer
                    $ptr = $ptr.add(align);
                }
            }
            // Process all aligned slices with at least one set of length `M128_VECTOR_SIZE`
            while $ptr <= $end_ptr.sub(M128_VECTOR_SIZE) {
                debug_assert_eq!(0, ($ptr as usize) % M128_VECTOR_SIZE);
                let mut mask = {
                    let a = _mm_load_si128($ptr as *const __m128i);
                    _mm_movemask_epi8(masking!(a))
                };

                if mask != 0 {
                    write_mask!(mask, $ptr);
                }
                $ptr = $ptr.add(M128_VECTOR_SIZE);
            }

            debug_assert!($end_ptr.sub(M128_VECTOR_SIZE) < $ptr);

            // At this point at most there is less than `M128_VECTOR_SIZE` elements
            // so the macro `write_mask` is used to the last elements
            if $ptr < $end_ptr {
                let d = M128_VECTOR_SIZE - $crate::sub!($end_ptr, $ptr);

                let mut mask = ({
                    debug_assert_eq!(M128_VECTOR_SIZE, $crate::sub!($end_ptr, $ptr.sub(d)));
                    let a = _mm_loadu_si128($ptr.sub(d) as *const __m128i);
                    _mm_movemask_epi8(masking!(a))
                } as u16).wrapping_shr(d as u32);

                if mask != 0 {
                    write_mask!(mask, $ptr);
                }
            }
        }
    };
}
