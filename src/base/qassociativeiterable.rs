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
  fn _ZNK20QAssociativeIterable4sizeEv() -> i32;
  fn _ZNK20QAssociativeIterable5valueERK8QVariant(arg0: *const c_void) -> i32;
}

// body block begin
// class sizeof(QAssociativeIterable)=112
pub struct QAssociativeIterable {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QAssociativeIterable {
  pub fn size<T: QAssociativeIterable_size>(&mut self, value: T) -> i32 {
    value.size(self);
    return 1;
  }
}

pub trait QAssociativeIterable_size {
  fn size(self, this: &mut QAssociativeIterable) -> i32;
}

// proto: int QAssociativeIterable::size();
impl<'a> /*trait*/ QAssociativeIterable_size for () {
  fn size(self, this: &mut QAssociativeIterable) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 112)};
    // unsafe{_ZNK20QAssociativeIterable4sizeEv()};
    unsafe {_ZNK20QAssociativeIterable4sizeEv()};
    return 1;
  }
}

impl /*struct*/ QAssociativeIterable {
  pub fn value<T: QAssociativeIterable_value>(&mut self, value: T) -> i32 {
    value.value(self);
    return 1;
  }
}

pub trait QAssociativeIterable_value {
  fn value(self, this: &mut QAssociativeIterable) -> i32;
}

// proto: QVariant QAssociativeIterable::value(const QVariant & key);
impl<'a> /*trait*/ QAssociativeIterable_value for (&'a  QVariant) {
  fn value(self, this: &mut QAssociativeIterable) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 112)};
    // unsafe{_ZNK20QAssociativeIterable5valueERK8QVariant()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK20QAssociativeIterable5valueERK8QVariant(arg0)};
    return 1;
  }
}

