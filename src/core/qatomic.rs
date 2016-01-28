// auto generated, do not modify.
// created: Thu Jan 28 22:38:45 2016
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
  fn QAtomicInt_Class_Size() -> c_int;
  // proto:  void QAtomicInt::QAtomicInt(int value);
  fn C_ZN10QAtomicIntC2Ei(arg0: c_int) -> u64;
} // <= ext block end

// body block begin =>
// class sizeof(QAtomicInt)=1
#[derive(Default)]
pub struct QAtomicInt {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QAtomicInt {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QAtomicInt {
    return QAtomicInt{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void QAtomicInt::QAtomicInt(int value);
impl /*struct*/ QAtomicInt {
  pub fn new<T: QAtomicInt_new>(value: T) -> QAtomicInt {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QAtomicInt_new {
  fn new(self) -> QAtomicInt;
}

  // proto:  void QAtomicInt::QAtomicInt(int value);
impl<'a> /*trait*/ QAtomicInt_new for (i32) {
  fn new(self) -> QAtomicInt {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QAtomicIntC2Ei()};
    let ctysz: c_int = unsafe{QAtomicInt_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self  as c_int;
    let qthis: u64 = unsafe {C_ZN10QAtomicIntC2Ei(arg0)};
    let rsthis = QAtomicInt{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// <= body block end

