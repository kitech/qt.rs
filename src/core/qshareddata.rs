// auto generated, do not modify.
// created: Sat Dec 26 10:16:52 2015
// src-file: /QtCore/qshareddata.h
// dst-file: /src/core/qshareddata.rs
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
  fn QSharedData_Class_Size() -> c_int;
} // <= ext block end

// body block begin =>
// class sizeof(QSharedData)=1
pub struct QSharedData {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QSharedData {
  pub fn inheritFrom(qthis: *mut c_void) -> QSharedData {
    return QSharedData{qclsinst: qthis};
  }
}
// <= body block end

