// auto generated, do not modify.
// created: Tue Dec 29 22:57:40 2015
// src-file: /QtCore/qnamespace.h
// dst-file: /src/core/qnamespace.rs
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
  fn QInternal_Class_Size() -> c_int;
} // <= ext block end

// body block begin =>
// class sizeof(QInternal)=1
#[derive(Default)]
pub struct QInternal {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QInternal {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QInternal {
    return QInternal{qclsinst: qthis, ..Default::default()};
  }
}
// <= body block end

