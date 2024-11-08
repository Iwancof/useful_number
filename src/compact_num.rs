use paste::paste;

macro_rules! define_compact_num {
    ($type_name: ident, $inner: ty) => {
        #[derive(PartialEq, Eq, Clone, Copy, Hash)]
        #[allow(unused)]
        pub struct $type_name<const EXCEPT: $inner> {
            inner: $inner,
        }

        paste! {
            pub type [<$type_name Max>] = $type_name<{$inner::MAX}>;
        }

        impl<const EXCEPT: $inner> core::default::Default for $type_name<EXCEPT> {
            fn default() -> Self {
                Self::new()
            }
        }

        impl<const EXCEPT: $inner> core::fmt::Debug for $type_name<EXCEPT> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
                f.debug_struct(stringify!($type_name))
                    .field("inner", &&self.get())
                    .finish()
            }
        }

        impl<const EXCEPT: $inner> core::cmp::PartialOrd for $type_name<EXCEPT> {
            fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                self.get()?.partial_cmp(other.get()?)
            }
        }

        impl<const EXCEPT: $inner> $type_name<EXCEPT> {
            pub fn new() -> Self {
                Self { inner: EXCEPT }
            }
            pub fn new_with(inner: $inner) -> Self {
                Self { inner }
            }
            pub fn get(&self) -> Option<&$inner> {
                if self.inner == EXCEPT {
                    None
                } else {
                    Some(&self.inner)
                }
            }
            pub fn get_mut(&mut self) -> Option<&mut $inner> {
                if self.inner == EXCEPT {
                    None
                } else {
                    Some(&mut self.inner)
                }
            }
            pub fn set(&mut self, val: $inner) {
                if val == EXCEPT {
                    panic!("cannot set EXCEPT value via set. use instead of init");
                }
                self.inner = val;
            }
            pub fn init(&mut self) {
                *self = Self::new();
            }
        }
    };
}

define_compact_num!(CompactU8, u8);
define_compact_num!(CompactU16, u16);
define_compact_num!(CompactU32, u32);
define_compact_num!(CompactU64, u64);
define_compact_num!(CompactU128, u128);
define_compact_num!(CompactUsize, usize);
define_compact_num!(CompactI8, i8);
define_compact_num!(CompactI16, i16);
define_compact_num!(CompactI32, i32);
define_compact_num!(CompactI64, i64);
define_compact_num!(CompactI128, i128);
define_compact_num!(CompactIsize, isize);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_value() {
        let mut v = CompactU32::<10>::new();
        assert!(v.get().is_none());

        v.set(20);
        assert_eq!(v.get().unwrap(), &20);

        v.init();
        assert!(v.get().is_none());
    }

    #[test]
    #[should_panic]
    fn set_exception_value() {
        let mut v = CompactU32::<10>::new();
        v.set(10);
    }
}
