// auto generated, do not modify.
// created: Wed Jan 20 00:44:03 2016
// src-file: /QtQml/qqmlparserstatus.h
// dst-file: /src/qml/qqmlparserstatus.rs
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
  fn QQmlParserStatus_Class_Size() -> c_int;
  // proto:  void QQmlParserStatus::QQmlParserStatus();
  fn _ZN16QQmlParserStatusC2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QQmlParserStatus::componentComplete();
  fn _ZN16QQmlParserStatus17componentCompleteEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QQmlParserStatus::~QQmlParserStatus();
  fn _ZN16QQmlParserStatusD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QQmlParserStatus::classBegin();
  fn _ZN16QQmlParserStatus10classBeginEv(qthis: u64 /* *mut c_void*/);
} // <= ext block end

// body block begin =>
// class sizeof(QQmlParserStatus)=16
#[derive(Default)]
pub struct QQmlParserStatus {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QQmlParserStatus {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QQmlParserStatus {
    return QQmlParserStatus{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void QQmlParserStatus::QQmlParserStatus();
impl /*struct*/ QQmlParserStatus {
  pub fn new<T: QQmlParserStatus_new>(value: T) -> QQmlParserStatus {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QQmlParserStatus_new {
  fn new(self) -> QQmlParserStatus;
}

  // proto:  void QQmlParserStatus::QQmlParserStatus();
impl<'a> /*trait*/ QQmlParserStatus_new for () {
  fn new(self) -> QQmlParserStatus {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QQmlParserStatusC2Ev()};
    let ctysz: c_int = unsafe{QQmlParserStatus_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    unsafe {_ZN16QQmlParserStatusC2Ev(qthis_ph)};
    let qthis: u64 = qthis_ph;
    let rsthis = QQmlParserStatus{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QQmlParserStatus::componentComplete();
impl /*struct*/ QQmlParserStatus {
  pub fn componentComplete<RetType, T: QQmlParserStatus_componentComplete<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.componentComplete(self);
    // return 1;
  }
}

pub trait QQmlParserStatus_componentComplete<RetType> {
  fn componentComplete(self , rsthis: & QQmlParserStatus) -> RetType;
}

  // proto:  void QQmlParserStatus::componentComplete();
impl<'a> /*trait*/ QQmlParserStatus_componentComplete<()> for () {
  fn componentComplete(self , rsthis: & QQmlParserStatus) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QQmlParserStatus17componentCompleteEv()};
     unsafe {_ZN16QQmlParserStatus17componentCompleteEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QQmlParserStatus::~QQmlParserStatus();
impl /*struct*/ QQmlParserStatus {
  pub fn free<RetType, T: QQmlParserStatus_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QQmlParserStatus_free<RetType> {
  fn free(self , rsthis: & QQmlParserStatus) -> RetType;
}

  // proto:  void QQmlParserStatus::~QQmlParserStatus();
impl<'a> /*trait*/ QQmlParserStatus_free<()> for () {
  fn free(self , rsthis: & QQmlParserStatus) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QQmlParserStatusD2Ev()};
     unsafe {_ZN16QQmlParserStatusD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QQmlParserStatus::classBegin();
impl /*struct*/ QQmlParserStatus {
  pub fn classBegin<RetType, T: QQmlParserStatus_classBegin<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.classBegin(self);
    // return 1;
  }
}

pub trait QQmlParserStatus_classBegin<RetType> {
  fn classBegin(self , rsthis: & QQmlParserStatus) -> RetType;
}

  // proto:  void QQmlParserStatus::classBegin();
impl<'a> /*trait*/ QQmlParserStatus_classBegin<()> for () {
  fn classBegin(self , rsthis: & QQmlParserStatus) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QQmlParserStatus10classBeginEv()};
     unsafe {_ZN16QQmlParserStatus10classBeginEv(rsthis.qclsinst)};
    // return 1;
  }
}

// <= body block end

