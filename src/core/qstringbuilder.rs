// auto generated, do not modify.
// created: Sat Dec 26 10:16:52 2015
// src-file: /QtCore/qstringbuilder.h
// dst-file: /src/core/qstringbuilder.rs
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
  fn QAbstractConcatenable_Class_Size() -> c_int;
} // <= ext block end

// body block begin =>
// class sizeof(QAbstractConcatenable)=1
pub struct QAbstractConcatenable {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QAbstractConcatenable {
  pub fn inheritFrom(qthis: *mut c_void) -> QAbstractConcatenable {
    return QAbstractConcatenable{qclsinst: qthis};
  }
}
// <= body block end

