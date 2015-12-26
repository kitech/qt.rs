// auto generated, do not modify.
// created: Sat Dec 26 10:16:52 2015
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
  // proto:  QStringList QFactoryInterface::keys();
  fn _ZNK17QFactoryInterface4keysEv(qthis: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QFactoryInterface)=8
pub struct QFactoryInterface {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QFactoryInterface {
  pub fn inheritFrom(qthis: *mut c_void) -> QFactoryInterface {
    return QFactoryInterface{qclsinst: qthis};
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

