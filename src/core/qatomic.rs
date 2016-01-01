// auto generated, do not modify.
// created: Fri Jan  1 12:13:41 2016
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
  fn dector_ZN10QAtomicIntC1Ei(arg0: c_int) -> *mut c_void;
  fn _ZN10QAtomicIntC1Ei(qthis: u64 /* *mut c_void*/, arg0: c_int);
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
    // unsafe{_ZN10QAtomicIntC1Ei()};
    let ctysz: c_int = unsafe{QAtomicInt_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self  as c_int;
    // unsafe {_ZN10QAtomicIntC1Ei(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN10QAtomicIntC1Ei(arg0)} as u64;
    let rsthis = QAtomicInt{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// <= body block end

