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
  // proto:  void QIncompatibleFlag::QIncompatibleFlag(int i);
  fn _ZN17QIncompatibleFlagC1Ei(qthis: *mut c_void, arg0: c_int);
}

// body block begin
// class sizeof(QIncompatibleFlag)=4
pub struct QIncompatibleFlag {
  pub qclsinst: *mut c_void,
}

  // proto:  void QIncompatibleFlag::QIncompatibleFlag(int i);
impl /*struct*/ QIncompatibleFlag {
  pub fn NewQIncompatibleFlag<T: QIncompatibleFlag_NewQIncompatibleFlag>(value: T) -> QIncompatibleFlag {
    let rsthis = value.NewQIncompatibleFlag();
    return rsthis;
    // return 1;
  }
}

pub trait QIncompatibleFlag_NewQIncompatibleFlag {
  fn NewQIncompatibleFlag(self) -> QIncompatibleFlag;
}

  // proto:  void QIncompatibleFlag::QIncompatibleFlag(int i);
impl<'a> /*trait*/ QIncompatibleFlag_NewQIncompatibleFlag for (i32) {
  fn NewQIncompatibleFlag(self) -> QIncompatibleFlag {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QIncompatibleFlagC1Ei()};
    let arg0 = self  as c_int;
    unsafe {_ZN17QIncompatibleFlagC1Ei(qthis, arg0)};
    let rsthis = QIncompatibleFlag{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

