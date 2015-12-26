// auto generated, do not modify.
// created: Sat Dec 26 10:16:52 2015
// src-file: /QtCore/qflags.h
// dst-file: /src/core/qflags.rs
//

// header block begin =>
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;


// <= header block end

// main block begin =>
// <= main block end

// use block begin =>
use std::ops::Deref;
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QIncompatibleFlag_Class_Size() -> c_int;
  // proto:  void QIncompatibleFlag::QIncompatibleFlag(int i);
  fn dector_ZN17QIncompatibleFlagC1Ei(arg0: c_int) -> *mut c_void;
  fn _ZN17QIncompatibleFlagC1Ei(qthis: *mut c_void, arg0: c_int);
  fn QFlag_Class_Size() -> c_int;
} // <= ext block end

// body block begin =>
// class sizeof(QIncompatibleFlag)=4
pub struct QIncompatibleFlag {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

// class sizeof(QFlag)=4
pub struct QFlag {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QIncompatibleFlag {
  pub fn inheritFrom(qthis: *mut c_void) -> QIncompatibleFlag {
    return QIncompatibleFlag{qclsinst: qthis};
  }
}
  // proto:  void QIncompatibleFlag::QIncompatibleFlag(int i);
impl /*struct*/ QIncompatibleFlag {
  pub fn New<T: QIncompatibleFlag_New>(value: T) -> QIncompatibleFlag {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QIncompatibleFlag_New {
  fn New(self) -> QIncompatibleFlag;
}

  // proto:  void QIncompatibleFlag::QIncompatibleFlag(int i);
impl<'a> /*trait*/ QIncompatibleFlag_New for (i32) {
  fn New(self) -> QIncompatibleFlag {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QIncompatibleFlagC1Ei()};
    let ctysz: c_int = unsafe{QIncompatibleFlag_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self  as c_int;
    // unsafe {_ZN17QIncompatibleFlagC1Ei(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN17QIncompatibleFlagC1Ei(arg0)};
    let rsthis = QIncompatibleFlag{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QFlag {
  pub fn inheritFrom(qthis: *mut c_void) -> QFlag {
    return QFlag{qclsinst: qthis};
  }
}
// <= body block end

