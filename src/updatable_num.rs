use paste::paste;

pub enum UpdateResult<T> {
    Updated,
    Equal(T),
    NotUpdated,
}

impl<T> UpdateResult<T> {
    pub fn map_equal<R, F>(self, f: F) -> UpdateResult<R>
    where
        F: Fn(T) -> R,
    {
        match self {
            Self::Updated => UpdateResult::Updated,
            Self::NotUpdated => UpdateResult::NotUpdated,
            Self::Equal(t) => UpdateResult::Equal(f(t)),
        }
    }
    pub fn is_notupdate(&self) -> bool {
        if let Self::NotUpdated = self {
            true
        } else {
            false
        }
    }
    pub fn is_update(&self) -> bool {
        if let Self::Updated = self {
            true
        } else {
            false
        }
    }
}

macro_rules! define_update_num {
    ($type_name: ident, $inner: ty, $initial_value: ident, $compare: ident) => {
        #[derive(PartialEq, Eq, Clone, Copy, Hash)]
        pub struct $type_name {
            inner: $inner,
        }

        impl core::fmt::Debug for $type_name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
                f.debug_struct(stringify!($type_name))
                    .field("inner", &&self.get())
                    .finish()
            }
        }

        impl core::cmp::PartialOrd for $type_name {
            fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                self.get()?.partial_cmp(other.get()?)
            }
        }

        impl $type_name {
            pub fn new() -> Self {
                Self {
                    inner: <$inner>::$initial_value,
                }
            }
            pub fn new_with(inner: $inner) -> Self {
                Self { inner }
            }
            pub fn update(&mut self, new: $inner) -> UpdateResult<&mut $inner> {
                if new == <$inner>::$initial_value {
                    panic!("cannot update to {}", <$inner>::$initial_value);
                }
                match new.cmp(&self.inner) {
                    core::cmp::Ordering::Equal => UpdateResult::Equal(self.get_mut().unwrap()),
                    core::cmp::Ordering::$compare => {
                        self.inner = new;
                        UpdateResult::Updated
                    }
                    _ => UpdateResult::NotUpdated,
                }
            }
            pub fn has_value(&self) -> bool {
                self.inner != <$inner>::$initial_value
            }
            pub fn get(&self) -> Option<&$inner> {
                if self.inner == <$inner>::$initial_value {
                    None
                } else {
                    Some(&self.inner)
                }
            }
            pub fn get_mut(&mut self) -> Option<&mut $inner> {
                if self.inner == <$inner>::$initial_value {
                    None
                } else {
                    Some(&mut self.inner)
                }
            }
            pub fn set(&mut self, val: $inner) {
                if val == <$inner>::$initial_value {
                    panic!("use instead of init");
                }
                self.inner = val; // unchecked
            }
            pub fn init(&mut self) {
                self.inner = <$inner>::$initial_value;
            }
        }
    };
}

macro_rules! define_update_num_with_data {
    ($type_name: ident, $inner: ty, $initial_value: ident, $compare: ident) => {
        paste! {
            pub struct [< $type_name WithData>]<T>
            {
                inner: $inner,
                data: core::mem::MaybeUninit<T>,
            }

            impl<T> Clone for [<$type_name WithData>]<T>
            where
                T: Clone
            {
                fn clone(&self) -> Self {
                    self.get().map(| (inner, data) | Self::new_with(inner.clone(), data.clone())).unwrap_or(Self::new())
                }
            }

            impl<T> Drop for [<$type_name WithData>]<T>
            {
                fn drop(&mut self) {
                    if self.has_value() {
                        // safety. self.data had benn initialized.
                        unsafe { self.data.assume_init_drop() };
                    }
                }
            }

            impl<T> core::fmt::Debug for [<$type_name WithData>]<T>
            where
                T: core::fmt::Debug,
            {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
                    let mut ds = f.debug_struct(stringify!([<$type_name WithData>]<T>));
                    if let Some((inner, data)) = self.get() {
                        ds
                            .field("inner", inner)
                            .field("data", data);
                    } else {
                        ds.field("inner", &Option::<()>::None);
                    };
                    ds.finish()
                }
            }

            impl<T> core::cmp::PartialEq for [<$type_name WithData>]<T> {
                fn eq(&self, other: &Self) -> bool {
                    if let Some((left, _)) = self.get() {
                        if let Some((right, _)) = other.get() {
                            if left == right {
                                return true;
                            }
                        }
                    }
                    return false;
                }
            }

            impl<T> core::cmp::PartialOrd for [<$type_name WithData>]<T> {
                fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                    self.get()?.0.partial_cmp(other.get()?.0)
                }
            }

            impl<T> core::default::Default for [<$type_name WithData>]<T> {
                fn default() -> Self {
                    Self::new()
                }
            }

            impl<T> [<$type_name WithData>]<T> {
                pub fn new() -> Self {
                    Self {
                        inner: <$inner>::$initial_value,
                        data: core::mem::MaybeUninit::uninit(),
                    }
                }
                pub fn new_with(inner: $inner, data: T) -> Self {
                    Self {
                        inner,
                        data: core::mem::MaybeUninit::new(data),
                    }
                }
                pub fn update(&mut self, new: $inner, data: T) -> UpdateResult<(&mut $inner, &mut T)> {
                    if new == <$inner>::$initial_value {
                        panic!("cannot update to {}", <$inner>::$initial_value);
                    }
                    match new.cmp(&self.inner) {
                        core::cmp::Ordering::Equal => {
                            UpdateResult::Equal(self.get_mut().unwrap())
                        },
                        core::cmp::Ordering::$compare => {
                            self.set(new, data);
                            UpdateResult::Updated
                        },
                        _ => {
                            UpdateResult::NotUpdated
                        }
                    }
                }
                pub fn has_value(&self) -> bool {
                    self.inner != <$inner>::$initial_value
                }
                pub fn get(&self) -> Option<(&$inner, &T)> {
                    if self.inner == <$inner>::$initial_value {
                        None
                    } else {
                        Some((&self.inner, unsafe { self.data.assume_init_ref() }))
                    }
                }
                pub fn get_mut(&mut self) -> Option<(&mut $inner, &mut T)> {
                    if self.inner == <$inner>::$initial_value {
                        None
                    } else {
                        Some((&mut self.inner, unsafe { self.data.assume_init_mut() }))
                    }
                }

                // need to initialize self.inner
                unsafe fn drop_inner(&mut self) {
                    if self.has_value() {
                        // safety. self.data had been initialized.
                        self.data.assume_init_drop();
                    }
                }
                pub fn set(&mut self, val: $inner, t: T) {
                    if val == <$inner>::$initial_value {
                        panic!("use instead of init");
                    }
                    unsafe { self.drop_inner() };
                    self.data.write(t);
                    self.inner = val;
                }
                pub fn init(&mut self) {
                    unsafe { self.drop_inner() };
                    self.inner = <$inner>::$initial_value;
                    self.data = core::mem::MaybeUninit::uninit(); // not needed.
                }
                pub fn take(self) -> Option<($inner, T)> {
                    let mut manual = core::mem::ManuallyDrop::new(self);

                    if manual.has_value() {
                        let data = core::mem::replace(&mut manual.data, core::mem::MaybeUninit::uninit());
                        return Some((manual.inner, unsafe { data.assume_init() }));
                    } else {
                        None
                    }
                }
            }
        }
    };
}

macro_rules! define_wrap {
    ($type_name: ident, $inner: ty, $initial_value: ident, $compare: ident) => {
        define_update_num!($type_name, $inner, $initial_value, $compare);
        define_update_num_with_data!($type_name, $inner, $initial_value, $compare);
    };
}

define_wrap!(UpdateToMinU8, u8, MAX, Less);
define_wrap!(UpdateToMinU16, u16, MAX, Less);
define_wrap!(UpdateToMinU32, u32, MAX, Less);
define_wrap!(UpdateToMinU64, u64, MAX, Less);
define_wrap!(UpdateToMinU128, u128, MAX, Less);
define_wrap!(UpdateToMinUsize, usize, MAX, Less);
define_wrap!(UpdateToMinI8, i8, MAX, Less);
define_wrap!(UpdateToMinI16, i16, MAX, Less);
define_wrap!(UpdateToMinI32, i32, MAX, Less);
define_wrap!(UpdateToMinI64, i64, MAX, Less);
define_wrap!(UpdateToMinI128, i128, MAX, Less);
define_wrap!(UpdateToMinIsize, isize, MAX, Less);

define_wrap!(UpdateToMaxU8, u8, MIN, Greater);
define_wrap!(UpdateToMaxU16, u16, MIN, Greater);
define_wrap!(UpdateToMaxU32, u32, MIN, Greater);
define_wrap!(UpdateToMaxU64, u64, MIN, Greater);
define_wrap!(UpdateToMaxU128, u128, MIN, Greater);
define_wrap!(UpdateToMaxUsize, usize, MIN, Greater);
define_wrap!(UpdateToMaxI8, i8, MIN, Greater);
define_wrap!(UpdateToMaxI16, i16, MIN, Greater);
define_wrap!(UpdateToMaxI32, i32, MIN, Greater);
define_wrap!(UpdateToMaxI64, i64, MIN, Greater);
define_wrap!(UpdateToMaxI128, i128, MIN, Greater);
define_wrap!(UpdateToMaxIsize, isize, MIN, Greater);

#[cfg(test)]
mod tests {
    use core::cmp::Ordering;

    use super::*;

    #[test]
    fn new_and_update_u32() {
        let mut v = UpdateToMinU32::new();

        assert_eq!(v.get(), None);

        v.update(10);
        assert_eq!(v.get(), Some(&10));

        v.update(20);
        assert_eq!(v.get(), Some(&10));

        v.update(5);
        assert_eq!(v.get(), Some(&5));

        *v.get_mut().unwrap() = 10;
        assert_eq!(v.get(), Some(&10));

        v.set(40);
        assert_eq!(v.get(), Some(&40));

        v.init();
        assert_eq!(v.get(), None);
    }

    #[test]
    fn new_and_update_i128() {
        let mut v = UpdateToMaxI128::new();

        assert_eq!(v.get(), None);

        v.update(10);
        assert_eq!(v.get(), Some(&10));

        v.update(5);
        assert_eq!(v.get(), Some(&10));

        v.update(20);
        assert_eq!(v.get(), Some(&20));

        *v.get_mut().unwrap() = 10;
        assert_eq!(v.get(), Some(&10));

        v.set(5);
        assert_eq!(v.get(), Some(&5));

        v.init();
        assert_eq!(v.get(), None);
    }

    #[test]
    fn compare_test() {
        let mut v1 = UpdateToMaxU128::new();
        let mut v2 = UpdateToMaxU128::new();

        assert!(v1.partial_cmp(&v2).is_none());

        v1.set(100);
        assert!(v1.partial_cmp(&v2).is_none());

        v1.init();
        v2.set(200);
        assert!(v1.partial_cmp(&v2).is_none());

        v1.set(100);
        // 100 < 200: Less
        assert_eq!(v1.partial_cmp(&v2).unwrap(), Ordering::Less);
    }

    #[test]
    fn drop_test() {
        static mut DROP_COUNTER: i32 = 0;

        struct DropMe;

        impl Drop for DropMe {
            fn drop(&mut self) {
                unsafe { DROP_COUNTER += 1 };
            }
        }

        let _ = UpdateToMaxU128WithData::new_with(100, DropMe);

        unsafe {
            assert_eq!(DROP_COUNTER, 1);
        }
    }

    #[test]
    fn drop_test_update() {
        static mut DROP_COUNTER: i32 = 0;

        struct DropMe;

        impl Drop for DropMe {
            fn drop(&mut self) {
                unsafe { DROP_COUNTER += 1 };
            }
        }

        let mut v = UpdateToMaxU128WithData::new_with(100, DropMe);
        v.update(200, DropMe);

        core::mem::forget(v);

        unsafe {
            assert_eq!(DROP_COUNTER, 1);
        }
    }

    #[test]
    fn test_take() {
        static mut DROP_COUNTER: i32 = 0;


        struct DropMe;

        impl Drop for DropMe {
            fn drop(&mut self) {
                unsafe { DROP_COUNTER += 1 };
            }
        }

        let v = UpdateToMaxU8WithData::new_with(100, DropMe);
        let (n, d) = v.take().unwrap();

        assert_eq!(n, 100);
        core::mem::forget(d);
    }
}
