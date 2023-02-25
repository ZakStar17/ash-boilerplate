use std::ffi::{c_char, CStr};

pub fn c_char_array_to_string(arr: &[c_char]) -> String {
  let raw_string = unsafe { CStr::from_ptr(arr.as_ptr()) };
  raw_string
    .to_str()
    .expect("Failed to convert raw string")
    .to_owned()
}

pub fn i8_array_to_string(arr: &[i8]) -> Result<String, std::string::FromUtf8Error> {
  let mut bytes = Vec::with_capacity(arr.len());
  for &b in arr {
    if b == 0 {
      break;
    }
    bytes.push(b as u8)
  }
  String::from_utf8(bytes)
}

pub fn contains_all<'a, T>(slice: &mut [T], other: &'a [T]) -> Result<(), &'a T>
where
  T: Eq + Ord,
{
  slice.sort();
  for item in other.iter() {
    if let Err(_) = slice.binary_search(item) {
      return Err(item);
    }
  }
  return Ok(());
}

pub unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
  std::slice::from_raw_parts((p as *const T) as *const u8, std::mem::size_of::<T>())
}

macro_rules! iter_into_array {
  ($x:expr, $size:expr) => {{
    let mut tmp: [MaybeUninit<_>; $size] = unsafe { MaybeUninit::uninit().assume_init() };
    for i in 0..$size {
      tmp[i] = MaybeUninit::new($x.next().unwrap());
    }
    unsafe { std::mem::transmute::<_, [_; $size]>(tmp) }
  }};
}

macro_rules! vec_to_array {
  ($x:expr, $size:expr) => {{
    let mut tmp: [MaybeUninit<_>; $size] = unsafe { MaybeUninit::uninit().assume_init() };
    for i in 0..$size {
      tmp[i] = MaybeUninit::new($x[i]);
    }
    unsafe { std::mem::transmute::<_, [_; $size]>(tmp) }
  }};
  ($x:expr, $size:expr, $offset:expr) => {{
    let mut tmp: [MaybeUninit<_>; $size] = unsafe { MaybeUninit::uninit().assume_init() };
    for i in 0..$size {
      tmp[i] = MaybeUninit::new($x[$offset + i]);
    }
    unsafe { std::mem::transmute::<_, [_; $size]>(tmp) }
  }};
}

pub(crate) use iter_into_array;
pub(crate) use vec_to_array;
