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
  // proto:  void QFlag::QFlag(ushort ai);
  fn _ZN5QFlagC1Et(qthis: *mut c_void, arg0: c_ushort);
  // proto:  void QFlag::QFlag(int ai);
  fn _ZN5QFlagC1Ei(qthis: *mut c_void, arg0: c_int);
  // proto:  void QFlag::QFlag(short ai);
  fn _ZN5QFlagC1Es(qthis: *mut c_void, arg0: c_short);
  // proto:  void QFlag::QFlag(uint ai);
  fn _ZN5QFlagC1Ej(qthis: *mut c_void, arg0: c_uint);
}

// body block begin
// class sizeof(QFlag)=4
pub struct QFlag {
  pub qclsinst: *mut c_void,
}

  // proto:  void QFlag::QFlag(ushort ai);
impl /*struct*/ QFlag {
  pub fn NewQFlag<T: QFlag_NewQFlag>(value: T) -> QFlag {
    let rsthis = value.NewQFlag();
    return rsthis;
    // return 1;
  }
}

pub trait QFlag_NewQFlag {
  fn NewQFlag(self) -> QFlag;
}

  // proto:  void QFlag::QFlag(ushort ai);
impl<'a> /*trait*/ QFlag_NewQFlag for (u16) {
  fn NewQFlag(self) -> QFlag {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFlagC1Et()};
    let arg0 = self  as c_ushort;
    unsafe {_ZN5QFlagC1Et(qthis, arg0)};
    let rsthis = QFlag{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QFlag::QFlag(int ai);
impl<'a> /*trait*/ QFlag_NewQFlag for (i32) {
  fn NewQFlag(self) -> QFlag {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFlagC1Ei()};
    let arg0 = self  as c_int;
    unsafe {_ZN5QFlagC1Ei(qthis, arg0)};
    let rsthis = QFlag{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QFlag::QFlag(short ai);
impl<'a> /*trait*/ QFlag_NewQFlag for (i16) {
  fn NewQFlag(self) -> QFlag {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFlagC1Es()};
    let arg0 = self  as c_short;
    unsafe {_ZN5QFlagC1Es(qthis, arg0)};
    let rsthis = QFlag{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QFlag::QFlag(uint ai);
impl<'a> /*trait*/ QFlag_NewQFlag for (u32) {
  fn NewQFlag(self) -> QFlag {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFlagC1Ej()};
    let arg0 = self  as c_uint;
    unsafe {_ZN5QFlagC1Ej(qthis, arg0)};
    let rsthis = QFlag{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

