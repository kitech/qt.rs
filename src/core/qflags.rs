// auto generated, do not modify.
// created: Thu Jan 28 22:38:45 2016
// src-file: /QtCore/qflags.h
// dst-file: /src/core/qflags.rs
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
  fn QIncompatibleFlag_Class_Size() -> c_int;
  // proto:  void QIncompatibleFlag::QIncompatibleFlag(int i);
  fn C_ZN17QIncompatibleFlagC2Ei(arg0: c_int) -> u64;
  fn QFlag_Class_Size() -> c_int;
  // proto:  void QFlag::QFlag(ushort ai);
  fn C_ZN5QFlagC2Et(arg0: c_ushort) -> u64;
  // proto:  void QFlag::QFlag(int ai);
  fn C_ZN5QFlagC2Ei(arg0: c_int) -> u64;
  // proto:  void QFlag::QFlag(short ai);
  fn C_ZN5QFlagC2Es(arg0: c_short) -> u64;
  // proto:  void QFlag::QFlag(uint ai);
  fn C_ZN5QFlagC2Ej(arg0: c_uint) -> u64;
} // <= ext block end

// body block begin =>
// class sizeof(QIncompatibleFlag)=4
#[derive(Default)]
pub struct QIncompatibleFlag {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QFlag)=4
#[derive(Default)]
pub struct QFlag {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QIncompatibleFlag {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QIncompatibleFlag {
    return QIncompatibleFlag{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void QIncompatibleFlag::QIncompatibleFlag(int i);
impl /*struct*/ QIncompatibleFlag {
  pub fn new<T: QIncompatibleFlag_new>(value: T) -> QIncompatibleFlag {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QIncompatibleFlag_new {
  fn new(self) -> QIncompatibleFlag;
}

  // proto:  void QIncompatibleFlag::QIncompatibleFlag(int i);
impl<'a> /*trait*/ QIncompatibleFlag_new for (i32) {
  fn new(self) -> QIncompatibleFlag {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QIncompatibleFlagC2Ei()};
    let ctysz: c_int = unsafe{QIncompatibleFlag_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self  as c_int;
    let qthis: u64 = unsafe {C_ZN17QIncompatibleFlagC2Ei(arg0)};
    let rsthis = QIncompatibleFlag{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QFlag {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QFlag {
    return QFlag{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void QFlag::QFlag(ushort ai);
impl /*struct*/ QFlag {
  pub fn new<T: QFlag_new>(value: T) -> QFlag {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QFlag_new {
  fn new(self) -> QFlag;
}

  // proto:  void QFlag::QFlag(ushort ai);
impl<'a> /*trait*/ QFlag_new for (u16) {
  fn new(self) -> QFlag {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFlagC2Et()};
    let ctysz: c_int = unsafe{QFlag_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self  as c_ushort;
    let qthis: u64 = unsafe {C_ZN5QFlagC2Et(arg0)};
    let rsthis = QFlag{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QFlag::QFlag(int ai);
impl<'a> /*trait*/ QFlag_new for (i32) {
  fn new(self) -> QFlag {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFlagC2Ei()};
    let ctysz: c_int = unsafe{QFlag_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self  as c_int;
    let qthis: u64 = unsafe {C_ZN5QFlagC2Ei(arg0)};
    let rsthis = QFlag{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QFlag::QFlag(short ai);
impl<'a> /*trait*/ QFlag_new for (i16) {
  fn new(self) -> QFlag {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFlagC2Es()};
    let ctysz: c_int = unsafe{QFlag_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self  as c_short;
    let qthis: u64 = unsafe {C_ZN5QFlagC2Es(arg0)};
    let rsthis = QFlag{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QFlag::QFlag(uint ai);
impl<'a> /*trait*/ QFlag_new for (u32) {
  fn new(self) -> QFlag {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFlagC2Ej()};
    let ctysz: c_int = unsafe{QFlag_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self  as c_uint;
    let qthis: u64 = unsafe {C_ZN5QFlagC2Ej(arg0)};
    let rsthis = QFlag{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// <= body block end

