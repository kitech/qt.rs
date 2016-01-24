// auto generated, do not modify.
// created: Sun Jan 24 17:41:38 2016
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
  fn QAtomicOpsSupport_4__Class_Size() -> c_int;
} // <= ext block end

// body block begin =>
// class sizeof(QAtomicOpsSupport_4_)=1
#[derive(Default)]
pub struct QAtomicOpsSupport_4_ {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QAtomicOpsSupport_4_ {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QAtomicOpsSupport_4_ {
    return QAtomicOpsSupport_4_{qclsinst: qthis, ..Default::default()};
  }
}
// <= body block end

