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
  fn _ZN16QCollatorSortKeyD0Ev() -> i32;
  fn _ZN16QCollatorSortKey4swapERS_(arg0: *mut c_void) -> i32;
  fn _ZNK16QCollatorSortKey7compareERKS_(arg0: *const c_void) -> i32;
  fn _ZN16QCollatorSortKeyC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZN16QCollatorSortKeyC1Ev(qthis: *mut c_void) -> i32;
}

// body block begin
// class sizeof(QCollatorSortKey)=1
pub struct QCollatorSortKey {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QCollatorSortKey {
  pub fn FreeQCollatorSortKey<T: QCollatorSortKey_FreeQCollatorSortKey>(&mut self, value: T) -> i32 {
    value.FreeQCollatorSortKey(self);
    return 1;
  }
}

pub trait QCollatorSortKey_FreeQCollatorSortKey {
  fn FreeQCollatorSortKey(self, this: &mut QCollatorSortKey) -> i32;
}

// proto: void QCollatorSortKey::FreeQCollatorSortKey();
impl<'a> /*trait*/ QCollatorSortKey_FreeQCollatorSortKey for () {
  fn FreeQCollatorSortKey(self, this: &mut QCollatorSortKey) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCollatorSortKeyD0Ev()};
    unsafe {_ZN16QCollatorSortKeyD0Ev()};
    return 1;
  }
}

impl /*struct*/ QCollatorSortKey {
  pub fn swap<T: QCollatorSortKey_swap>(&mut self, value: T) -> i32 {
    value.swap(self);
    return 1;
  }
}

pub trait QCollatorSortKey_swap {
  fn swap(self, this: &mut QCollatorSortKey) -> i32;
}

// proto: void QCollatorSortKey::swap(QCollatorSortKey & other);
impl<'a> /*trait*/ QCollatorSortKey_swap for (&'a mut QCollatorSortKey) {
  fn swap(self, this: &mut QCollatorSortKey) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCollatorSortKey4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN16QCollatorSortKey4swapERS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QCollatorSortKey {
  pub fn compare<T: QCollatorSortKey_compare>(&mut self, value: T) -> i32 {
    value.compare(self);
    return 1;
  }
}

pub trait QCollatorSortKey_compare {
  fn compare(self, this: &mut QCollatorSortKey) -> i32;
}

// proto: int QCollatorSortKey::compare(const QCollatorSortKey & key);
impl<'a> /*trait*/ QCollatorSortKey_compare for (&'a  QCollatorSortKey) {
  fn compare(self, this: &mut QCollatorSortKey) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QCollatorSortKey7compareERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK16QCollatorSortKey7compareERKS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QCollatorSortKey {
  pub fn NewQCollatorSortKey<T: QCollatorSortKey_NewQCollatorSortKey>(value: T) -> QCollatorSortKey {
    let rsthis = value.NewQCollatorSortKey();
    return rsthis;
    // return 1;
  }
}

pub trait QCollatorSortKey_NewQCollatorSortKey {
  fn NewQCollatorSortKey(self) -> QCollatorSortKey;
}

// proto: void QCollatorSortKey::NewQCollatorSortKey(const QCollatorSortKey & other);
impl<'a> /*trait*/ QCollatorSortKey_NewQCollatorSortKey for (&'a  QCollatorSortKey) {
  fn NewQCollatorSortKey(self) -> QCollatorSortKey {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCollatorSortKeyC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN16QCollatorSortKeyC1ERKS_(qthis, arg0)};
    let rsthis = QCollatorSortKey{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QCollatorSortKey::NewQCollatorSortKey();
impl<'a> /*trait*/ QCollatorSortKey_NewQCollatorSortKey for () {
  fn NewQCollatorSortKey(self) -> QCollatorSortKey {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCollatorSortKeyC1Ev()};
    unsafe {_ZN16QCollatorSortKeyC1Ev(qthis)};
    let rsthis = QCollatorSortKey{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

