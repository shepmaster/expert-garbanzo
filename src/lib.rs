#![allow(incomplete_features)]
#![feature(const_generics, const_evaluatable_checked)]

trait FromExactBytesLe {
    const N_BYTES: usize;

    fn from_bytes_le(raw_bytes: &[u8; Self::N_BYTES]) -> Self;
}

trait SplitFixed<T, const N: usize> {
    fn split_fixed<const M: usize>(&self) -> (&[T; M], &[T; N-M]);
}

impl<T, const N: usize> SplitFixed<T, N> for [T; N] {
    fn split_fixed<const M: usize>(&self) -> (&[T; M], &[T; N-M]) {
        // Safety: Arrays are always contiguous, so performing an
        // offset is acceptable. `pointer::add` handles zero-sized
        // types for us. The compiler ensures that the resulting types
        // are valid, so all that we need to ensure is that the
        // arithmetic around the lengths is valid.
        unsafe {
            let start = self.as_ptr();
            let x = start as *const [T; M];
            let y = start.add(M) as *const [T; N-M];
            (&*x, &*y)
        }
    }
}

macro_rules! integer_impls {
    ($($t:ty),* $(,)?) => {
        $(
            impl FromExactBytesLe for $t {
                const N_BYTES: usize = core::mem::size_of::<$t>();

                fn from_bytes_le(b: &[u8; Self::N_BYTES]) -> Self {
                    Self::from_le_bytes(*b)
                }
            }
        )*
    }
}

integer_impls!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128);

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

mod example {
    use crate as the_crate;

    struct Header {
        a: u8,
        b: u16,
        c: u32,
        d: u64,
    }

    // TODO: Derive this
    impl the_crate::FromExactBytesLe for Header {
        const N_BYTES: usize = {
            <u8 as the_crate::FromExactBytesLe>::N_BYTES +
                <u16 as the_crate::FromExactBytesLe>::N_BYTES +
                <u32 as the_crate::FromExactBytesLe>::N_BYTES +
                <u64 as the_crate::FromExactBytesLe>::N_BYTES
        };

        fn from_bytes_le(__remaining: &[u8; Self::N_BYTES]) -> Self {
            let (a, __remaining) = the_crate::SplitFixed::split_fixed(__remaining);
            let a = the_crate::FromExactBytesLe::from_bytes_le(a);
            let (b, __remaining) = the_crate::SplitFixed::split_fixed(__remaining);
            let b = the_crate::FromExactBytesLe::from_bytes_le(b);
            let (c, __remaining) = the_crate::SplitFixed::split_fixed(__remaining);
            let c = the_crate::FromExactBytesLe::from_bytes_le(c);
            let (d, __remaining) = the_crate::SplitFixed::split_fixed(__remaining);
            let d = the_crate::FromExactBytesLe::from_bytes_le(d);

            // Compile-time check we consumed all the bytes
            let _: &[u8; 0] = __remaining;

            Self { a, b, c, d }
        }
    }
}
