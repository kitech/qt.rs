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
  // proto:  int QAssociativeIterable::size();
  fn _ZNK20QAssociativeIterable4sizeEv(qthis: *mut c_void) -> c_int;
  // proto:  QVariant QAssociativeIterable::value(const QVariant & key);
  fn _ZNK20QAssociativeIterable5valueERK8QVariant(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
}

// body block begin
// class sizeof(QAssociativeIterable)=112
pub struct QAssociativeIterable {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QAssociativeIterable {
  pub fn size<RetType, T: QAssociativeIterable_size<RetType>>(&mut self, value: T) -> RetType {
    return value.size(self);
    // return 1;
  }
}

pub trait QAssociativeIterable_size<RetType> {
  fn size(self, rsthis: &mut QAssociativeIterable) -> RetType;
}

// proto:  int QAssociativeIterable::size();
impl<'a> /*trait*/ QAssociativeIterable_size<i32> for () {
  fn size(self, rsthis: &mut QAssociativeIterable) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 112)};
    // unsafe{_ZNK20QAssociativeIterable4sizeEv()};
    let mut ret = unsafe {_ZNK20QAssociativeIterable4sizeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QAssociativeIterable {
  pub fn value<RetType, T: QAssociativeIterable_value<RetType>>(&mut self, value: T) -> RetType {
    return value.value(self);
    // return 1;
  }
}

pub trait QAssociativeIterable_value<RetType> {
  fn value(self, rsthis: &mut QAssociativeIterable) -> RetType;
}

// proto:  QVariant QAssociativeIterable::value(const QVariant & key);
impl<'a> /*trait*/ QAssociativeIterable_value<QVariant> for (&'a  QVariant) {
  fn value(self, rsthis: &mut QAssociativeIterable) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 112)};
    // unsafe{_ZNK20QAssociativeIterable5valueERK8QVariant()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK20QAssociativeIterable5valueERK8QVariant(rsthis.qclsinst, arg0)};
    let mut ret1 = QVariant{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

