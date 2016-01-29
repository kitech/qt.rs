// auto generated, do not modify.
// created: Thu Jan 28 22:38:45 2016
// src-file: /QtCore/qgenericatomic.h
// dst-file: /src/core/qgenericatomic.rs
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
  fn QAtomicOpsSupportL4G_Class_Size() -> c_int;
} // <= ext block end

// body block begin =>
// class sizeof(QAtomicOpsSupportL4G)=1
#[derive(Default)]
pub struct QAtomicOpsSupportL4G {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QAtomicOpsSupportL4G {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QAtomicOpsSupportL4G {
    return QAtomicOpsSupportL4G{qclsinst: qthis, ..Default::default()};
  }
}
// <= body block end

