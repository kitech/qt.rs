// auto generated, do not modify.
// created: Mon Dec 21 22:54:38 2015
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
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  void QFactoryInterface::~QFactoryInterface();
  fn _ZN17QFactoryInterfaceD0Ev(qthis: *mut c_void);
  // proto:  QStringList QFactoryInterface::keys();
  fn _ZNK17QFactoryInterface4keysEv(qthis: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QFactoryInterface)=8
pub struct QFactoryInterface {
  pub qclsinst: *mut c_void,
}

  // proto:  void QFactoryInterface::~QFactoryInterface();
impl /*struct*/ QFactoryInterface {
  pub fn FreeQFactoryInterface<RetType, T: QFactoryInterface_FreeQFactoryInterface<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQFactoryInterface(self);
    // return 1;
  }
}

pub trait QFactoryInterface_FreeQFactoryInterface<RetType> {
  fn FreeQFactoryInterface(self , rsthis: &mut QFactoryInterface) -> RetType;
}

  // proto:  void QFactoryInterface::~QFactoryInterface();
impl<'a> /*trait*/ QFactoryInterface_FreeQFactoryInterface<()> for () {
  fn FreeQFactoryInterface(self , rsthis: &mut QFactoryInterface) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QFactoryInterfaceD0Ev()};
     unsafe {_ZN17QFactoryInterfaceD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QStringList QFactoryInterface::keys();
impl /*struct*/ QFactoryInterface {
  pub fn keys<RetType, T: QFactoryInterface_keys<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.keys(self);
    // return 1;
  }
}

pub trait QFactoryInterface_keys<RetType> {
  fn keys(self , rsthis: &mut QFactoryInterface) -> RetType;
}

  // proto:  QStringList QFactoryInterface::keys();
impl<'a> /*trait*/ QFactoryInterface_keys<()> for () {
  fn keys(self , rsthis: &mut QFactoryInterface) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QFactoryInterface4keysEv()};
     unsafe {_ZNK17QFactoryInterface4keysEv(rsthis.qclsinst)};
    // return 1;
  }
}

// <= body block end

