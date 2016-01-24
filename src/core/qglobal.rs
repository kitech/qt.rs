// auto generated, do not modify.
// created: Sun Jan 24 17:41:38 2016
// src-file: /QtCore/qglobal.h
// dst-file: /src/core/qglobal.rs
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
  fn QIntegerForSize_4__Class_Size() -> c_int;
  fn QIntegerForSize_8__Class_Size() -> c_int;
  fn QIntegerForSize_2__Class_Size() -> c_int;
  fn QIntegerForSize_1__Class_Size() -> c_int;
} // <= ext block end

// body block begin =>
// class sizeof(QIntegerForSize_4_)=1
#[derive(Default)]
pub struct QIntegerForSize_4_ {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QIntegerForSize_8_)=1
#[derive(Default)]
pub struct QIntegerForSize_8_ {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QIntegerForSize_2_)=1
#[derive(Default)]
pub struct QIntegerForSize_2_ {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QIntegerForSize_1_)=1
#[derive(Default)]
pub struct QIntegerForSize_1_ {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QIntegerForSize_4_ {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QIntegerForSize_4_ {
    return QIntegerForSize_4_{qclsinst: qthis, ..Default::default()};
  }
}
impl /*struct*/ QIntegerForSize_8_ {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QIntegerForSize_8_ {
    return QIntegerForSize_8_{qclsinst: qthis, ..Default::default()};
  }
}
impl /*struct*/ QIntegerForSize_2_ {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QIntegerForSize_2_ {
    return QIntegerForSize_2_{qclsinst: qthis, ..Default::default()};
  }
}
impl /*struct*/ QIntegerForSize_1_ {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QIntegerForSize_1_ {
    return QIntegerForSize_1_{qclsinst: qthis, ..Default::default()};
  }
}
// <= body block end

