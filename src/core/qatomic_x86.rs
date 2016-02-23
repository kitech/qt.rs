// auto generated, do not modify.
// created: Mon Feb 22 23:57:02 2016
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
  fn QAtomicOpsSupportL2G_Class_Size() -> c_int;
  fn QAtomicOpsSupportL1G_Class_Size() -> c_int;
  fn QAtomicOpsSupportL8G_Class_Size() -> c_int;
} // <= ext block end

// body block begin =>
// class sizeof(QAtomicOpsSupportL2G)=1
#[derive(Default)]
pub struct QAtomicOpsSupportL2G {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QAtomicOpsSupportL1G)=1
#[derive(Default)]
pub struct QAtomicOpsSupportL1G {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QAtomicOpsSupportL8G)=1
#[derive(Default)]
pub struct QAtomicOpsSupportL8G {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QAtomicOpsSupportL2G {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QAtomicOpsSupportL2G {
    return QAtomicOpsSupportL2G{qclsinst: qthis, ..Default::default()};
  }
}
impl /*struct*/ QAtomicOpsSupportL1G {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QAtomicOpsSupportL1G {
    return QAtomicOpsSupportL1G{qclsinst: qthis, ..Default::default()};
  }
}
impl /*struct*/ QAtomicOpsSupportL8G {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QAtomicOpsSupportL8G {
    return QAtomicOpsSupportL8G{qclsinst: qthis, ..Default::default()};
  }
}
// <= body block end

