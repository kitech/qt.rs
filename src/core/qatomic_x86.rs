// auto generated, do not modify.
// created: Sun Jan 24 17:41:38 2016
// src-file: /QtCore/qatomic_x86.h
// dst-file: /src/core/qatomic_x86.rs
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
  fn QAtomicOpsSupport_2__Class_Size() -> c_int;
  fn QAtomicOpsSupport_1__Class_Size() -> c_int;
  fn QAtomicOpsSupport_8__Class_Size() -> c_int;
} // <= ext block end

// body block begin =>
// class sizeof(QAtomicOpsSupport_2_)=1
#[derive(Default)]
pub struct QAtomicOpsSupport_2_ {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QAtomicOpsSupport_1_)=1
#[derive(Default)]
pub struct QAtomicOpsSupport_1_ {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QAtomicOpsSupport_8_)=1
#[derive(Default)]
pub struct QAtomicOpsSupport_8_ {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QAtomicOpsSupport_2_ {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QAtomicOpsSupport_2_ {
    return QAtomicOpsSupport_2_{qclsinst: qthis, ..Default::default()};
  }
}
impl /*struct*/ QAtomicOpsSupport_1_ {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QAtomicOpsSupport_1_ {
    return QAtomicOpsSupport_1_{qclsinst: qthis, ..Default::default()};
  }
}
impl /*struct*/ QAtomicOpsSupport_8_ {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QAtomicOpsSupport_8_ {
    return QAtomicOpsSupport_8_{qclsinst: qthis, ..Default::default()};
  }
}
// <= body block end

