// auto generated, do not modify.
// created: Sat Dec 26 10:16:52 2015
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
// <= body block end

