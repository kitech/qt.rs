// auto generated, do not modify.
// created: Sat Dec 26 10:52:38 2015
// src-file: /QtCore/qrunnable.h
// dst-file: /src/core/qrunnable.rs
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
  fn QRunnable_Class_Size() -> c_int;
  // proto:  void QRunnable::~QRunnable();
  fn _ZN9QRunnableD0Ev(qthis: *mut c_void);
  // proto:  void QRunnable::setAutoDelete(bool _autoDelete);
  fn _ZN9QRunnable13setAutoDeleteEb(qthis: *mut c_void, arg0: c_char);
  // proto:  void QRunnable::QRunnable();
  fn dector_ZN9QRunnableC1Ev() -> *mut c_void;
  fn _ZN9QRunnableC1Ev(qthis: *mut c_void);
  // proto:  void QRunnable::run();
  fn _ZN9QRunnable3runEv(qthis: *mut c_void);
  // proto:  bool QRunnable::autoDelete();
  fn _ZNK9QRunnable10autoDeleteEv(qthis: *mut c_void) -> c_char;
} // <= ext block end

// body block begin =>
// class sizeof(QRunnable)=16
pub struct QRunnable {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QRunnable {
  pub fn inheritFrom(qthis: *mut c_void) -> QRunnable {
    return QRunnable{qclsinst: qthis};
  }
}
  // proto:  void QRunnable::~QRunnable();
impl /*struct*/ QRunnable {
  pub fn Free<RetType, T: QRunnable_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QRunnable_Free<RetType> {
  fn Free(self , rsthis: & QRunnable) -> RetType;
}

  // proto:  void QRunnable::~QRunnable();
impl<'a> /*trait*/ QRunnable_Free<()> for () {
  fn Free(self , rsthis: & QRunnable) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QRunnableD0Ev()};
     unsafe {_ZN9QRunnableD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QRunnable::setAutoDelete(bool _autoDelete);
impl /*struct*/ QRunnable {
  pub fn setAutoDelete<RetType, T: QRunnable_setAutoDelete<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setAutoDelete(self);
    // return 1;
  }
}

pub trait QRunnable_setAutoDelete<RetType> {
  fn setAutoDelete(self , rsthis: & QRunnable) -> RetType;
}

  // proto:  void QRunnable::setAutoDelete(bool _autoDelete);
impl<'a> /*trait*/ QRunnable_setAutoDelete<()> for (i8) {
  fn setAutoDelete(self , rsthis: & QRunnable) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QRunnable13setAutoDeleteEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN9QRunnable13setAutoDeleteEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QRunnable::QRunnable();
impl /*struct*/ QRunnable {
  pub fn New<T: QRunnable_New>(value: T) -> QRunnable {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QRunnable_New {
  fn New(self) -> QRunnable;
}

  // proto:  void QRunnable::QRunnable();
impl<'a> /*trait*/ QRunnable_New for () {
  fn New(self) -> QRunnable {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QRunnableC1Ev()};
    let ctysz: c_int = unsafe{QRunnable_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    // unsafe {_ZN9QRunnableC1Ev(qthis)};
    let qthis: *mut c_void = unsafe {dector_ZN9QRunnableC1Ev()};
    let rsthis = QRunnable{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QRunnable::run();
impl /*struct*/ QRunnable {
  pub fn run<RetType, T: QRunnable_run<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.run(self);
    // return 1;
  }
}

pub trait QRunnable_run<RetType> {
  fn run(self , rsthis: & QRunnable) -> RetType;
}

  // proto:  void QRunnable::run();
impl<'a> /*trait*/ QRunnable_run<()> for () {
  fn run(self , rsthis: & QRunnable) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QRunnable3runEv()};
     unsafe {_ZN9QRunnable3runEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QRunnable::autoDelete();
impl /*struct*/ QRunnable {
  pub fn autoDelete<RetType, T: QRunnable_autoDelete<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.autoDelete(self);
    // return 1;
  }
}

pub trait QRunnable_autoDelete<RetType> {
  fn autoDelete(self , rsthis: & QRunnable) -> RetType;
}

  // proto:  bool QRunnable::autoDelete();
impl<'a> /*trait*/ QRunnable_autoDelete<i8> for () {
  fn autoDelete(self , rsthis: & QRunnable) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QRunnable10autoDeleteEv()};
    let mut ret = unsafe {_ZNK9QRunnable10autoDeleteEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// <= body block end

