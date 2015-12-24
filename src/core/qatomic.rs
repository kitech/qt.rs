// auto generated, do not modify.
// created: Thu Dec 24 23:00:39 2015
// src-file: /QtCore/qatomic.h
// dst-file: /src/core/qatomic.rs
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
  // proto:  void QAtomicInt::QAtomicInt(int value);
  fn _ZN10QAtomicIntC1Ei(qthis: *mut c_void, arg0: c_int);
} // <= ext block end

// body block begin =>
// class sizeof(QAtomicInt)=1
pub struct QAtomicInt {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QAtomicInt {
  pub fn inheritFrom(qthis: *mut c_void) -> QAtomicInt {
    return QAtomicInt{qclsinst: qthis};
  }
}
  // proto:  void QAtomicInt::QAtomicInt(int value);
impl /*struct*/ QAtomicInt {
  pub fn New<T: QAtomicInt_New>(value: T) -> QAtomicInt {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QAtomicInt_New {
  fn New(self) -> QAtomicInt;
}

  // proto:  void QAtomicInt::QAtomicInt(int value);
impl<'a> /*trait*/ QAtomicInt_New for (i32) {
  fn New(self) -> QAtomicInt {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QAtomicIntC1Ei()};
    let arg0 = self  as c_int;
    unsafe {_ZN10QAtomicIntC1Ei(qthis, arg0)};
    let rsthis = QAtomicInt{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// <= body block end

