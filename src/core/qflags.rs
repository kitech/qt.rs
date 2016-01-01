// auto generated, do not modify.
// created: Fri Jan  1 12:13:41 2016
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
  fn dector_ZN17QIncompatibleFlagC1Ei(arg0: c_int) -> *mut c_void;
  fn _ZN17QIncompatibleFlagC1Ei(qthis: u64 /* *mut c_void*/, arg0: c_int);
  fn QFlag_Class_Size() -> c_int;
  // proto:  void QFlag::QFlag(ushort ai);
  fn dector_ZN5QFlagC1Et(arg0: c_ushort) -> *mut c_void;
  fn _ZN5QFlagC1Et(qthis: u64 /* *mut c_void*/, arg0: c_ushort);
  // proto:  void QFlag::QFlag(int ai);
  fn dector_ZN5QFlagC1Ei(arg0: c_int) -> *mut c_void;
  fn _ZN5QFlagC1Ei(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QFlag::QFlag(short ai);
  fn dector_ZN5QFlagC1Es(arg0: c_short) -> *mut c_void;
  fn _ZN5QFlagC1Es(qthis: u64 /* *mut c_void*/, arg0: c_short);
  // proto:  void QFlag::QFlag(uint ai);
  fn dector_ZN5QFlagC1Ej(arg0: c_uint) -> *mut c_void;
  fn _ZN5QFlagC1Ej(qthis: u64 /* *mut c_void*/, arg0: c_uint);
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
    // unsafe{_ZN17QIncompatibleFlagC1Ei()};
    let ctysz: c_int = unsafe{QIncompatibleFlag_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self  as c_int;
    // unsafe {_ZN17QIncompatibleFlagC1Ei(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN17QIncompatibleFlagC1Ei(arg0)} as u64;
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
    // unsafe{_ZN5QFlagC1Et()};
    let ctysz: c_int = unsafe{QFlag_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self  as c_ushort;
    // unsafe {_ZN5QFlagC1Et(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN5QFlagC1Et(arg0)} as u64;
    let rsthis = QFlag{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QFlag::QFlag(int ai);
impl<'a> /*trait*/ QFlag_new for (i32) {
  fn new(self) -> QFlag {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFlagC1Ei()};
    let ctysz: c_int = unsafe{QFlag_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self  as c_int;
    // unsafe {_ZN5QFlagC1Ei(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN5QFlagC1Ei(arg0)} as u64;
    let rsthis = QFlag{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QFlag::QFlag(short ai);
impl<'a> /*trait*/ QFlag_new for (i16) {
  fn new(self) -> QFlag {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFlagC1Es()};
    let ctysz: c_int = unsafe{QFlag_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self  as c_short;
    // unsafe {_ZN5QFlagC1Es(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN5QFlagC1Es(arg0)} as u64;
    let rsthis = QFlag{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QFlag::QFlag(uint ai);
impl<'a> /*trait*/ QFlag_new for (u32) {
  fn new(self) -> QFlag {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QFlagC1Ej()};
    let ctysz: c_int = unsafe{QFlag_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self  as c_uint;
    // unsafe {_ZN5QFlagC1Ej(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN5QFlagC1Ej(arg0)} as u64;
    let rsthis = QFlag{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// <= body block end

