// auto generated, do not modify.
// created: Wed Dec 30 23:22:52 2015
// src-file: /QtCore/qfactoryinterface.h
// dst-file: /src/core/qfactoryinterface.rs
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
  fn QFactoryInterface_Class_Size() -> c_int;
  // proto:  void QFactoryInterface::~QFactoryInterface();
  fn _ZN17QFactoryInterfaceD0Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QStringList QFactoryInterface::keys();
  fn _ZNK17QFactoryInterface4keysEv(qthis: u64 /* *mut c_void*/);
} // <= ext block end

// body block begin =>
// class sizeof(QFactoryInterface)=8
#[derive(Default)]
pub struct QFactoryInterface {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QFactoryInterface {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QFactoryInterface {
    return QFactoryInterface{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void QFactoryInterface::~QFactoryInterface();
impl /*struct*/ QFactoryInterface {
  pub fn Free<RetType, T: QFactoryInterface_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QFactoryInterface_Free<RetType> {
  fn Free(self , rsthis: & QFactoryInterface) -> RetType;
}

  // proto:  void QFactoryInterface::~QFactoryInterface();
impl<'a> /*trait*/ QFactoryInterface_Free<()> for () {
  fn Free(self , rsthis: & QFactoryInterface) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QFactoryInterfaceD0Ev()};
     unsafe {_ZN17QFactoryInterfaceD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QStringList QFactoryInterface::keys();
impl /*struct*/ QFactoryInterface {
  pub fn keys<RetType, T: QFactoryInterface_keys<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.keys(self);
    // return 1;
  }
}

pub trait QFactoryInterface_keys<RetType> {
  fn keys(self , rsthis: & QFactoryInterface) -> RetType;
}

  // proto:  QStringList QFactoryInterface::keys();
impl<'a> /*trait*/ QFactoryInterface_keys<()> for () {
  fn keys(self , rsthis: & QFactoryInterface) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QFactoryInterface4keysEv()};
     unsafe {_ZNK17QFactoryInterface4keysEv(rsthis.qclsinst)};
    // return 1;
  }
}

// <= body block end

