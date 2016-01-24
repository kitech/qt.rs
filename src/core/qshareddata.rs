// auto generated, do not modify.
// created: Sun Jan 24 17:41:38 2016
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
  // proto:  void QSharedData::QSharedData();
  fn C_ZN11QSharedDataC2Ev() -> u64;
  // proto:  void QSharedData::QSharedData(const QSharedData & );
  fn C_ZN11QSharedDataC2ERKS_(arg0: *mut c_void) -> u64;
} // <= ext block end

// body block begin =>
// class sizeof(QSharedData)=1
#[derive(Default)]
pub struct QSharedData {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QSharedData {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QSharedData {
    return QSharedData{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void QSharedData::QSharedData();
impl /*struct*/ QSharedData {
  pub fn new<T: QSharedData_new>(value: T) -> QSharedData {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QSharedData_new {
  fn new(self) -> QSharedData;
}

  // proto:  void QSharedData::QSharedData();
impl<'a> /*trait*/ QSharedData_new for () {
  fn new(self) -> QSharedData {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QSharedDataC2Ev()};
    let ctysz: c_int = unsafe{QSharedData_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let qthis: u64 = unsafe {C_ZN11QSharedDataC2Ev()};
    let rsthis = QSharedData{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QSharedData::QSharedData(const QSharedData & );
impl<'a> /*trait*/ QSharedData_new for (&'a QSharedData) {
  fn new(self) -> QSharedData {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QSharedDataC2ERKS_()};
    let ctysz: c_int = unsafe{QSharedData_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN11QSharedDataC2ERKS_(arg0)};
    let rsthis = QSharedData{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// <= body block end

