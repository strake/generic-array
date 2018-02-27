//! This crate implements a structure that can be used as a generic array type.use
//! Core Rust array types `[T; N]` can't be used generically with
//! respect to `N`, so for example this:
//!
//! ```{should_fail}
//! struct Foo<T, N> {
//!     data: [T; N]
//! }
//! ```
//!
//! won't work.
//!
//! **generic-array** exports a `GenericArray<T,N>` type, which lets
//! the above be implemented as:
//!
//! ```
//! # use generic_array::{ArrayLength, GenericArray};
//! struct Foo<T, N: ArrayLength<T>> {
//!     data: GenericArray<T,N>
//! }
//! ```
//!
//! The `ArrayLength<T>` trait is implemented by default for
//! [unsigned integer types](../typenum/uint/index.html) from
//! [typenum](../typenum/index.html).
//!
//! For ease of use, an `arr!` macro is provided - example below:
//!
//! ```
//! # #[macro_use]
//! # extern crate generic_array;
//! # extern crate typenum;
//! # fn main() {
//! let array = arr![u32; 1, 2, 3];
//! assert_eq!(array[2], 3);
//! # }
//! ```

//#![deny(missing_docs)]
#![no_std]

extern crate generic_array;

pub use generic_array::{ArrayLength, GenericArray, iter, transmute, typenum};
pub use iter::GenericArrayIter;

#[cfg_attr(test, macro_use)]
pub mod arr;

#[cfg(test)]
mod test {
    // Compile with:
    // cargo rustc --lib --profile test --release --
    //      -C target-cpu=native -C opt-level=3 --emit asm
    // and view the assembly to make sure test_assembly generates
    // SIMD instructions instead of a niave loop.

    #[inline(never)]
    pub fn black_box<T>(val: T) -> T {
        use core::{mem, ptr};

        let ret = unsafe { ptr::read_volatile(&val) };
        mem::forget(val);
        ret
    }

    #[test]
    fn test_assembly() {
        let a = black_box(arr![i32; 1, 3, 5, 7]);
        let b = black_box(arr![i32; 2, 4, 6, 8]);

        let c = a.zip_ref(&b, |l, r| l + r);

        assert_eq!(c, arr![i32; 3, 7, 11, 15]);
    }
}
