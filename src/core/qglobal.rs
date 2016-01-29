// auto generated, do not modify.
// created: Thu Jan 28 22:38:45 2016
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
  fn QIntegerForSizeL4G_Class_Size() -> c_int;
  fn QIntegerForSizeL8G_Class_Size() -> c_int;
  fn QIntegerForSizeL2G_Class_Size() -> c_int;
  fn QIntegerForSizeL1G_Class_Size() -> c_int;
} // <= ext block end

// body block begin =>
// class sizeof(QIntegerForSizeL4G)=1
#[derive(Default)]
pub struct QIntegerForSizeL4G {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QIntegerForSizeL8G)=1
#[derive(Default)]
pub struct QIntegerForSizeL8G {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QIntegerForSizeL2G)=1
#[derive(Default)]
pub struct QIntegerForSizeL2G {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QIntegerForSizeL1G)=1
#[derive(Default)]
pub struct QIntegerForSizeL1G {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QIntegerForSizeL4G {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QIntegerForSizeL4G {
    return QIntegerForSizeL4G{qclsinst: qthis, ..Default::default()};
  }
}
impl /*struct*/ QIntegerForSizeL8G {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QIntegerForSizeL8G {
    return QIntegerForSizeL8G{qclsinst: qthis, ..Default::default()};
  }
}
impl /*struct*/ QIntegerForSizeL2G {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QIntegerForSizeL2G {
    return QIntegerForSizeL2G{qclsinst: qthis, ..Default::default()};
  }
}
impl /*struct*/ QIntegerForSizeL1G {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QIntegerForSizeL1G {
    return QIntegerForSizeL1G{qclsinst: qthis, ..Default::default()};
  }
}
// <= body block end

