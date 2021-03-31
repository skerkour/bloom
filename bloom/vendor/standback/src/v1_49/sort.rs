use core::{cmp, mem, ptr};

use crate::mem::MaybeUninit;

fn maybe_uninit_slice_as_mut_ptr<T: Copy>(this: &mut [MaybeUninit<T>]) -> *mut T {
    this as *mut [MaybeUninit<T>] as *mut T
}

struct CopyOnDrop<T> {
    src: *mut T,
    dest: *mut T,
}

impl<T> Drop for CopyOnDrop<T> {
    fn drop(&mut self) {
        unsafe {
            ptr::copy_nonoverlapping(self.src, self.dest, 1);
        }
    }
}

fn shift_tail<T, F>(v: &mut [T], is_less: &mut F)
where
    F: FnMut(&T, &T) -> bool,
{
    let len = v.len();
    unsafe {
        if len >= 2 && is_less(v.get_unchecked(len - 1), v.get_unchecked(len - 2)) {
            let mut tmp = mem::ManuallyDrop::new(ptr::read(v.get_unchecked(len - 1)));
            let mut hole = CopyOnDrop {
                src: &mut *tmp,
                dest: v.get_unchecked_mut(len - 2),
            };
            ptr::copy_nonoverlapping(v.get_unchecked(len - 2), v.get_unchecked_mut(len - 1), 1);

            for i in (0..len - 2).rev() {
                if !is_less(&*tmp, v.get_unchecked(i)) {
                    break;
                }

                ptr::copy_nonoverlapping(v.get_unchecked(i), v.get_unchecked_mut(i + 1), 1);
                hole.dest = v.get_unchecked_mut(i);
            }
        }
    }
}

fn insertion_sort<T, F>(v: &mut [T], is_less: &mut F)
where
    F: FnMut(&T, &T) -> bool,
{
    for i in 1..v.len() {
        shift_tail(&mut v[..i + 1], is_less);
    }
}

fn partition_in_blocks<T, F>(v: &mut [T], pivot: &T, is_less: &mut F) -> usize
where
    F: FnMut(&T, &T) -> bool,
{
    const BLOCK: usize = 128;

    let mut l = v.as_mut_ptr();
    let mut block_l = BLOCK;
    let mut start_l = ptr::null_mut();
    let mut end_l = ptr::null_mut();
    let mut offsets_l = [MaybeUninit::<u8>::uninit(); BLOCK];

    let mut r = unsafe { l.add(v.len()) };
    let mut block_r = BLOCK;
    let mut start_r = ptr::null_mut();
    let mut end_r = ptr::null_mut();
    let mut offsets_r = [MaybeUninit::<u8>::uninit(); BLOCK];

    fn width<T>(l: *mut T, r: *mut T) -> usize {
        assert!(mem::size_of::<T>() > 0);
        (r as usize - l as usize) / mem::size_of::<T>()
    }

    loop {
        let is_done = width(l, r) <= 2 * BLOCK;

        if is_done {
            let mut rem = width(l, r);
            if start_l < end_l || start_r < end_r {
                rem -= BLOCK;
            }

            if start_l < end_l {
                block_r = rem;
            } else if start_r < end_r {
                block_l = rem;
            } else {
                block_l = rem / 2;
                block_r = rem - block_l;
            }
            debug_assert!(block_l <= BLOCK && block_r <= BLOCK);
            debug_assert!(width(l, r) == block_l + block_r);
        }

        if start_l == end_l {
            start_l = maybe_uninit_slice_as_mut_ptr(&mut offsets_l);
            end_l = maybe_uninit_slice_as_mut_ptr(&mut offsets_l);
            let mut elem = l;

            for i in 0..block_l {
                unsafe {
                    *end_l = i as u8;
                    end_l = end_l.offset(!is_less(&*elem, pivot) as isize);
                    elem = elem.offset(1);
                }
            }
        }

        if start_r == end_r {
            start_r = maybe_uninit_slice_as_mut_ptr(&mut offsets_r);
            end_r = maybe_uninit_slice_as_mut_ptr(&mut offsets_r);
            let mut elem = r;

            for i in 0..block_r {
                unsafe {
                    elem = elem.offset(-1);
                    *end_r = i as u8;
                    end_r = end_r.offset(is_less(&*elem, pivot) as isize);
                }
            }
        }

        let count = cmp::min(width(start_l, end_l), width(start_r, end_r));

        if count > 0 {
            macro_rules! left {
                () => {
                    l.offset(*start_l as isize)
                };
            }
            macro_rules! right {
                () => {
                    r.offset(-(*start_r as isize) - 1)
                };
            }

            unsafe {
                let tmp = ptr::read(left!());
                ptr::copy_nonoverlapping(right!(), left!(), 1);

                for _ in 1..count {
                    start_l = start_l.offset(1);
                    ptr::copy_nonoverlapping(left!(), right!(), 1);
                    start_r = start_r.offset(1);
                    ptr::copy_nonoverlapping(right!(), left!(), 1);
                }

                ptr::copy_nonoverlapping(&tmp, right!(), 1);
                mem::forget(tmp);
                start_l = start_l.offset(1);
                start_r = start_r.offset(1);
            }
        }

        if start_l == end_l {
            l = unsafe { l.add(block_l) };
        }

        if start_r == end_r {
            r = unsafe { r.offset(-(block_r as isize)) };
        }

        if is_done {
            break;
        }
    }

    if start_l < end_l {
        debug_assert_eq!(width(l, r), block_l);
        while start_l < end_l {
            unsafe {
                end_l = end_l.offset(-1);
                ptr::swap(l.offset(*end_l as isize), r.offset(-1));
                r = r.offset(-1);
            }
        }
        width(v.as_mut_ptr(), r)
    } else if start_r < end_r {
        debug_assert_eq!(width(l, r), block_r);
        while start_r < end_r {
            unsafe {
                end_r = end_r.offset(-1);
                ptr::swap(l, r.offset(-(*end_r as isize) - 1));
                l = l.offset(1);
            }
        }
        width(v.as_mut_ptr(), l)
    } else {
        width(v.as_mut_ptr(), l)
    }
}

fn partition<T, F>(v: &mut [T], pivot: usize, is_less: &mut F) -> (usize, bool)
where
    F: FnMut(&T, &T) -> bool,
{
    let (mid, was_partitioned) = {
        v.swap(0, pivot);
        let (pivot, v) = v.split_at_mut(1);
        let pivot = &mut pivot[0];

        let mut tmp = mem::ManuallyDrop::new(unsafe { ptr::read(pivot) });
        let _pivot_guard = CopyOnDrop {
            src: &mut *tmp,
            dest: pivot,
        };
        let pivot = &*tmp;

        let mut l = 0;
        let mut r = v.len();

        unsafe {
            while l < r && is_less(v.get_unchecked(l), pivot) {
                l += 1;
            }

            while l < r && !is_less(v.get_unchecked(r - 1), pivot) {
                r -= 1;
            }
        }

        (
            l + partition_in_blocks(&mut v[l..r], pivot, is_less),
            l >= r,
        )
    };

    v.swap(0, mid);

    (mid, was_partitioned)
}

fn partition_equal<T, F>(v: &mut [T], pivot: usize, is_less: &mut F) -> usize
where
    F: FnMut(&T, &T) -> bool,
{
    v.swap(0, pivot);
    let (pivot, v) = v.split_at_mut(1);
    let pivot = &mut pivot[0];

    let mut tmp = mem::ManuallyDrop::new(unsafe { ptr::read(pivot) });
    let _pivot_guard = CopyOnDrop {
        src: &mut *tmp,
        dest: pivot,
    };
    let pivot = &*tmp;

    let mut l = 0;
    let mut r = v.len();
    loop {
        unsafe {
            while l < r && !is_less(pivot, v.get_unchecked(l)) {
                l += 1;
            }

            while l < r && is_less(pivot, v.get_unchecked(r - 1)) {
                r -= 1;
            }

            if l >= r {
                break;
            }

            r -= 1;
            ptr::swap(v.get_unchecked_mut(l), v.get_unchecked_mut(r));
            l += 1;
        }
    }

    l + 1
}

fn choose_pivot<T, F>(v: &mut [T], is_less: &mut F) -> (usize, bool)
where
    F: FnMut(&T, &T) -> bool,
{
    const SHORTEST_MEDIAN_OF_MEDIANS: usize = 50;
    const MAX_SWAPS: usize = 4 * 3;

    let len = v.len();

    let mut a = len / 4;
    let mut b = len / 4 * 2;
    let mut c = len / 4 * 3;

    let mut swaps = 0;

    if len >= 8 {
        let mut sort2 = |a: &mut usize, b: &mut usize| unsafe {
            if is_less(v.get_unchecked(*b), v.get_unchecked(*a)) {
                ptr::swap(a, b);
                swaps += 1;
            }
        };

        let mut sort3 = |a: &mut usize, b: &mut usize, c: &mut usize| {
            sort2(a, b);
            sort2(b, c);
            sort2(a, b);
        };

        if len >= SHORTEST_MEDIAN_OF_MEDIANS {
            let mut sort_adjacent = |a: &mut usize| {
                let tmp = *a;
                sort3(&mut (tmp - 1), a, &mut (tmp + 1));
            };

            sort_adjacent(&mut a);
            sort_adjacent(&mut b);
            sort_adjacent(&mut c);
        }

        sort3(&mut a, &mut b, &mut c);
    }

    if swaps < MAX_SWAPS {
        (b, swaps == 0)
    } else {
        v.reverse();
        (len - 1 - b, true)
    }
}

fn partition_at_index_loop<'a, T, F>(
    mut v: &'a mut [T],
    mut index: usize,
    is_less: &mut F,
    mut pred: Option<&'a T>,
) where
    F: FnMut(&T, &T) -> bool,
{
    loop {
        const MAX_INSERTION: usize = 10;
        if v.len() <= MAX_INSERTION {
            insertion_sort(v, is_less);
            return;
        }

        let (pivot, _) = choose_pivot(v, is_less);

        if let Some(p) = pred {
            if !is_less(p, &v[pivot]) {
                let mid = partition_equal(v, pivot, is_less);

                if mid > index {
                    return;
                }

                v = &mut v[mid..];
                index -= mid;
                pred = None;
                continue;
            }
        }

        let (mid, _) = partition(v, pivot, is_less);

        let (left, right) = { v }.split_at_mut(mid);
        let (pivot, right) = right.split_at_mut(1);
        let pivot = &pivot[0];

        match mid.cmp(&index) {
            cmp::Ordering::Less => {
                v = right;
                index = index - mid - 1;
                pred = Some(pivot);
            }
            cmp::Ordering::Greater => {
                v = left;
            }
            cmp::Ordering::Equal => {
                return;
            }
        }
    }
}

pub(crate) fn partition_at_index<T, F>(
    v: &mut [T],
    index: usize,
    mut is_less: F,
) -> (&mut [T], &mut T, &mut [T])
where
    F: FnMut(&T, &T) -> bool,
{
    use self::cmp::Ordering::{Greater, Less};

    if index >= v.len() {
        panic!(
            "partition_at_index index {} greater than length of slice {}",
            index,
            v.len()
        );
    }

    if mem::size_of::<T>() == 0 {
    } else if index == v.len() - 1 {
        let (max_index, _) = v
            .iter()
            .enumerate()
            .max_by(|&(_, x), &(_, y)| if is_less(x, y) { Less } else { Greater })
            .unwrap();
        v.swap(max_index, index);
    } else if index == 0 {
        let (min_index, _) = v
            .iter()
            .enumerate()
            .min_by(|&(_, x), &(_, y)| if is_less(x, y) { Less } else { Greater })
            .unwrap();
        v.swap(min_index, index);
    } else {
        partition_at_index_loop(v, index, &mut is_less, None);
    }

    let (left, right) = v.split_at_mut(index);
    let (pivot, right) = right.split_at_mut(1);
    let pivot = &mut pivot[0];
    (left, pivot, right)
}
