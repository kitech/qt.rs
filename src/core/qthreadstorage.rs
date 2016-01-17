// auto generated, do not modify.
// created: Sun Jan 17 17:37:11 2016
// src-file: /QtCore/qthreadstorage.h
// dst-file: /src/core/qthreadstorage.rs
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
  fn QThreadStorageData_Class_Size() -> c_int;
  // proto:  void ** QThreadStorageData::set(void * p);
  fn _ZN18QThreadStorageData3setEPv(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void ** QThreadStorageData::get();
  fn _ZNK18QThreadStorageData3getEv(qthis: u64 /* *mut c_void*/);
  // proto: static void QThreadStorageData::finish(void ** );
  fn _ZN18QThreadStorageData6finishEPPv(arg0: *mut c_void);
  // proto:  void QThreadStorageData::~QThreadStorageData();
  fn _ZN18QThreadStorageDataD2Ev(qthis: u64 /* *mut c_void*/);
} // <= ext block end

// body block begin =>
// class sizeof(QThreadStorageData)=4
#[derive(Default)]
pub struct QThreadStorageData {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QThreadStorageData {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QThreadStorageData {
    return QThreadStorageData{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void ** QThreadStorageData::set(void * p);
impl /*struct*/ QThreadStorageData {
  pub fn set<RetType, T: QThreadStorageData_set<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.set(self);
    // return 1;
  }
}

pub trait QThreadStorageData_set<RetType> {
  fn set(self , rsthis: & QThreadStorageData) -> RetType;
}

  // proto:  void ** QThreadStorageData::set(void * p);
impl<'a> /*trait*/ QThreadStorageData_set<()> for (*mut c_void) {
  fn set(self , rsthis: & QThreadStorageData) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QThreadStorageData3setEPv()};
    let arg0 = self  as *mut c_void;
     unsafe {_ZN18QThreadStorageData3setEPv(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void ** QThreadStorageData::get();
impl /*struct*/ QThreadStorageData {
  pub fn get<RetType, T: QThreadStorageData_get<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.get(self);
    // return 1;
  }
}

pub trait QThreadStorageData_get<RetType> {
  fn get(self , rsthis: & QThreadStorageData) -> RetType;
}

  // proto:  void ** QThreadStorageData::get();
impl<'a> /*trait*/ QThreadStorageData_get<()> for () {
  fn get(self , rsthis: & QThreadStorageData) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QThreadStorageData3getEv()};
     unsafe {_ZNK18QThreadStorageData3getEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto: static void QThreadStorageData::finish(void ** );
impl /*struct*/ QThreadStorageData {
  pub fn finish_s<RetType, T: QThreadStorageData_finish_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.finish_s();
    // return 1;
  }
}

pub trait QThreadStorageData_finish_s<RetType> {
  fn finish_s(self ) -> RetType;
}

  // proto: static void QThreadStorageData::finish(void ** );
impl<'a> /*trait*/ QThreadStorageData_finish_s<()> for (*mut c_void) {
  fn finish_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QThreadStorageData6finishEPPv()};
    let arg0 = self  as *mut c_void;
     unsafe {_ZN18QThreadStorageData6finishEPPv(arg0)};
    // return 1;
  }
}

  // proto:  void QThreadStorageData::~QThreadStorageData();
impl /*struct*/ QThreadStorageData {
  pub fn free<RetType, T: QThreadStorageData_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QThreadStorageData_free<RetType> {
  fn free(self , rsthis: & QThreadStorageData) -> RetType;
}

  // proto:  void QThreadStorageData::~QThreadStorageData();
impl<'a> /*trait*/ QThreadStorageData_free<()> for () {
  fn free(self , rsthis: & QThreadStorageData) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QThreadStorageDataD2Ev()};
     unsafe {_ZN18QThreadStorageDataD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// <= body block end

