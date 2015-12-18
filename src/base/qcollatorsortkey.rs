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
  // proto:  void QCollatorSortKey::FreeQCollatorSortKey();
  fn _ZN16QCollatorSortKeyD0Ev(qthis: *mut c_void) ;
  // proto:  void QCollatorSortKey::swap(QCollatorSortKey & other);
  fn _ZN16QCollatorSortKey4swapERS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  int QCollatorSortKey::compare(const QCollatorSortKey & key);
  fn _ZNK16QCollatorSortKey7compareERKS_(qthis: *mut c_void, arg0: *mut c_void) -> c_int;
  // proto:  void QCollatorSortKey::NewQCollatorSortKey(const QCollatorSortKey & other);
  fn _ZN16QCollatorSortKeyC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QCollatorSortKey::NewQCollatorSortKey();
  fn _ZN16QCollatorSortKeyC1Ev(qthis: *mut c_void) ;
}

// body block begin
// class sizeof(QCollatorSortKey)=1
pub struct QCollatorSortKey {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QCollatorSortKey {
  pub fn FreeQCollatorSortKey<RetType, T: QCollatorSortKey_FreeQCollatorSortKey<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQCollatorSortKey(self);
    // return 1;
  }
}

pub trait QCollatorSortKey_FreeQCollatorSortKey<RetType> {
  fn FreeQCollatorSortKey(self, rsthis: &mut QCollatorSortKey) -> RetType;
}

// proto:  void QCollatorSortKey::FreeQCollatorSortKey();
impl<'a> /*trait*/ QCollatorSortKey_FreeQCollatorSortKey<()> for () {
  fn FreeQCollatorSortKey(self, rsthis: &mut QCollatorSortKey) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCollatorSortKeyD0Ev()};
     unsafe {_ZN16QCollatorSortKeyD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QCollatorSortKey {
  pub fn swap<RetType, T: QCollatorSortKey_swap<RetType>>(&mut self, value: T) -> RetType {
    return value.swap(self);
    // return 1;
  }
}

pub trait QCollatorSortKey_swap<RetType> {
  fn swap(self, rsthis: &mut QCollatorSortKey) -> RetType;
}

// proto:  void QCollatorSortKey::swap(QCollatorSortKey & other);
impl<'a> /*trait*/ QCollatorSortKey_swap<()> for (&'a mut QCollatorSortKey) {
  fn swap(self, rsthis: &mut QCollatorSortKey) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCollatorSortKey4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QCollatorSortKey4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QCollatorSortKey {
  pub fn compare<RetType, T: QCollatorSortKey_compare<RetType>>(&mut self, value: T) -> RetType {
    return value.compare(self);
    // return 1;
  }
}

pub trait QCollatorSortKey_compare<RetType> {
  fn compare(self, rsthis: &mut QCollatorSortKey) -> RetType;
}

// proto:  int QCollatorSortKey::compare(const QCollatorSortKey & key);
impl<'a> /*trait*/ QCollatorSortKey_compare<i32> for (&'a  QCollatorSortKey) {
  fn compare(self, rsthis: &mut QCollatorSortKey) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QCollatorSortKey7compareERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK16QCollatorSortKey7compareERKS_(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
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
    let arg0 = self.qclsinst  as *mut c_void;
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

