// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  fn _ZNK19QSequentialIterable4sizeEv() -> i32;
  fn _ZNK19QSequentialIterable17canReverseIterateEv() -> i32;
  fn _ZNK19QSequentialIterable2atEi(arg0: c_int) -> i32;
}

// body block begin
// class sizeof(QSequentialIterable)=104
pub struct QSequentialIterable {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QSequentialIterable {
  pub fn size<T: QSequentialIterable_size>(&mut self, value: T) -> i32 {
    value.size(self);
    return 1;
  }
}

pub trait QSequentialIterable_size {
  fn size(self, this: &mut QSequentialIterable) -> i32;
}

// proto: int QSequentialIterable::size();
impl<'a> /*trait*/ QSequentialIterable_size for () {
  fn size(self, this: &mut QSequentialIterable) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 104)};
    // unsafe{_ZNK19QSequentialIterable4sizeEv()};
    unsafe {_ZNK19QSequentialIterable4sizeEv()};
    return 1;
  }
}

impl /*struct*/ QSequentialIterable {
  pub fn canReverseIterate<T: QSequentialIterable_canReverseIterate>(&mut self, value: T) -> i32 {
    value.canReverseIterate(self);
    return 1;
  }
}

pub trait QSequentialIterable_canReverseIterate {
  fn canReverseIterate(self, this: &mut QSequentialIterable) -> i32;
}

// proto: bool QSequentialIterable::canReverseIterate();
impl<'a> /*trait*/ QSequentialIterable_canReverseIterate for () {
  fn canReverseIterate(self, this: &mut QSequentialIterable) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 104)};
    // unsafe{_ZNK19QSequentialIterable17canReverseIterateEv()};
    unsafe {_ZNK19QSequentialIterable17canReverseIterateEv()};
    return 1;
  }
}

impl /*struct*/ QSequentialIterable {
  pub fn at<T: QSequentialIterable_at>(&mut self, value: T) -> i32 {
    value.at(self);
    return 1;
  }
}

pub trait QSequentialIterable_at {
  fn at(self, this: &mut QSequentialIterable) -> i32;
}

// proto: QVariant QSequentialIterable::at(int idx);
impl<'a> /*trait*/ QSequentialIterable_at for (i32) {
  fn at(self, this: &mut QSequentialIterable) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 104)};
    // unsafe{_ZNK19QSequentialIterable2atEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK19QSequentialIterable2atEi(arg0)};
    return 1;
  }
}

