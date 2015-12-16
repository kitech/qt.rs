// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qvariant::QVariant;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  int QSequentialIterable::size();
  fn _ZNK19QSequentialIterable4sizeEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QSequentialIterable::canReverseIterate();
  fn _ZNK19QSequentialIterable17canReverseIterateEv(qthis: *mut c_void) -> int8_t;
  // proto:  QVariant QSequentialIterable::at(int idx);
  fn _ZNK19QSequentialIterable2atEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
}

// body block begin
// class sizeof(QSequentialIterable)=104
pub struct QSequentialIterable {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QSequentialIterable {
  pub fn size<T: QSequentialIterable_size>(&mut self, value: T) -> i32 {
    return value.size(self);
    // return 1;
  }
}

pub trait QSequentialIterable_size {
  fn size(self, rsthis: &mut QSequentialIterable) -> i32;
}

// proto:  int QSequentialIterable::size();
impl<'a> /*trait*/ QSequentialIterable_size for () {
  fn size(self, rsthis: &mut QSequentialIterable) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 104)};
    // unsafe{_ZNK19QSequentialIterable4sizeEv()};
    let mut ret = unsafe {_ZNK19QSequentialIterable4sizeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QSequentialIterable {
  pub fn canReverseIterate<T: QSequentialIterable_canReverseIterate>(&mut self, value: T) -> i8 {
    return value.canReverseIterate(self);
    // return 1;
  }
}

pub trait QSequentialIterable_canReverseIterate {
  fn canReverseIterate(self, rsthis: &mut QSequentialIterable) -> i8;
}

// proto:  bool QSequentialIterable::canReverseIterate();
impl<'a> /*trait*/ QSequentialIterable_canReverseIterate for () {
  fn canReverseIterate(self, rsthis: &mut QSequentialIterable) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 104)};
    // unsafe{_ZNK19QSequentialIterable17canReverseIterateEv()};
    let mut ret = unsafe {_ZNK19QSequentialIterable17canReverseIterateEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QSequentialIterable {
  pub fn at<T: QSequentialIterable_at>(&mut self, value: T) -> QVariant {
    return value.at(self);
    // return 1;
  }
}

pub trait QSequentialIterable_at {
  fn at(self, rsthis: &mut QSequentialIterable) -> QVariant;
}

// proto:  QVariant QSequentialIterable::at(int idx);
impl<'a> /*trait*/ QSequentialIterable_at for (i32) {
  fn at(self, rsthis: &mut QSequentialIterable) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 104)};
    // unsafe{_ZNK19QSequentialIterable2atEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK19QSequentialIterable2atEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QVariant{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

