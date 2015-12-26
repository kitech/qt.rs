// auto generated, do not modify.
// created: Sat Dec 26 12:15:38 2015
// src-file: /QtCore/qsemaphore.h
// dst-file: /src/core/qsemaphore.rs
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
  fn QSemaphore_Class_Size() -> c_int;
  // proto:  void QSemaphore::acquire(int n);
  fn _ZN10QSemaphore7acquireEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QSemaphore::release(int n);
  fn _ZN10QSemaphore7releaseEi(qthis: *mut c_void, arg0: c_int);
  // proto:  int QSemaphore::available();
  fn _ZNK10QSemaphore9availableEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QSemaphore::tryAcquire(int n, int timeout);
  fn _ZN10QSemaphore10tryAcquireEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) -> c_char;
  // proto:  void QSemaphore::QSemaphore(const QSemaphore & );
  fn dector_ZN10QSemaphoreC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN10QSemaphoreC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QSemaphore::tryAcquire(int n);
  fn _ZN10QSemaphore10tryAcquireEi(qthis: *mut c_void, arg0: c_int) -> c_char;
  // proto:  void QSemaphore::QSemaphore(int n);
  fn dector_ZN10QSemaphoreC1Ei(arg0: c_int) -> *mut c_void;
  fn _ZN10QSemaphoreC1Ei(qthis: *mut c_void, arg0: c_int);
  // proto:  void QSemaphore::~QSemaphore();
  fn _ZN10QSemaphoreD0Ev(qthis: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QSemaphore)=8
pub struct QSemaphore {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QSemaphore {
  pub fn inheritFrom(qthis: *mut c_void) -> QSemaphore {
    return QSemaphore{qclsinst: qthis};
  }
}
  // proto:  void QSemaphore::acquire(int n);
impl /*struct*/ QSemaphore {
  pub fn acquire<RetType, T: QSemaphore_acquire<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.acquire(self);
    // return 1;
  }
}

pub trait QSemaphore_acquire<RetType> {
  fn acquire(self , rsthis: & QSemaphore) -> RetType;
}

  // proto:  void QSemaphore::acquire(int n);
impl<'a> /*trait*/ QSemaphore_acquire<()> for (i32) {
  fn acquire(self , rsthis: & QSemaphore) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QSemaphore7acquireEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN10QSemaphore7acquireEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QSemaphore::release(int n);
impl /*struct*/ QSemaphore {
  pub fn release<RetType, T: QSemaphore_release<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.release(self);
    // return 1;
  }
}

pub trait QSemaphore_release<RetType> {
  fn release(self , rsthis: & QSemaphore) -> RetType;
}

  // proto:  void QSemaphore::release(int n);
impl<'a> /*trait*/ QSemaphore_release<()> for (i32) {
  fn release(self , rsthis: & QSemaphore) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QSemaphore7releaseEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN10QSemaphore7releaseEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QSemaphore::available();
impl /*struct*/ QSemaphore {
  pub fn available<RetType, T: QSemaphore_available<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.available(self);
    // return 1;
  }
}

pub trait QSemaphore_available<RetType> {
  fn available(self , rsthis: & QSemaphore) -> RetType;
}

  // proto:  int QSemaphore::available();
impl<'a> /*trait*/ QSemaphore_available<i32> for () {
  fn available(self , rsthis: & QSemaphore) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QSemaphore9availableEv()};
    let mut ret = unsafe {_ZNK10QSemaphore9availableEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  bool QSemaphore::tryAcquire(int n, int timeout);
impl /*struct*/ QSemaphore {
  pub fn tryAcquire<RetType, T: QSemaphore_tryAcquire<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.tryAcquire(self);
    // return 1;
  }
}

pub trait QSemaphore_tryAcquire<RetType> {
  fn tryAcquire(self , rsthis: & QSemaphore) -> RetType;
}

  // proto:  bool QSemaphore::tryAcquire(int n, int timeout);
impl<'a> /*trait*/ QSemaphore_tryAcquire<i8> for (i32, i32) {
  fn tryAcquire(self , rsthis: & QSemaphore) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QSemaphore10tryAcquireEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZN10QSemaphore10tryAcquireEii(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QSemaphore::QSemaphore(const QSemaphore & );
impl /*struct*/ QSemaphore {
  pub fn New<T: QSemaphore_New>(value: T) -> QSemaphore {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QSemaphore_New {
  fn New(self) -> QSemaphore;
}

  // proto:  void QSemaphore::QSemaphore(const QSemaphore & );
impl<'a> /*trait*/ QSemaphore_New for (&'a QSemaphore) {
  fn New(self) -> QSemaphore {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QSemaphoreC1ERKS_()};
    let ctysz: c_int = unsafe{QSemaphore_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN10QSemaphoreC1ERKS_(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN10QSemaphoreC1ERKS_(arg0)};
    let rsthis = QSemaphore{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QSemaphore::tryAcquire(int n);
impl<'a> /*trait*/ QSemaphore_tryAcquire<i8> for (i32) {
  fn tryAcquire(self , rsthis: & QSemaphore) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QSemaphore10tryAcquireEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN10QSemaphore10tryAcquireEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QSemaphore::QSemaphore(int n);
impl<'a> /*trait*/ QSemaphore_New for (i32) {
  fn New(self) -> QSemaphore {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QSemaphoreC1Ei()};
    let ctysz: c_int = unsafe{QSemaphore_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self  as c_int;
    // unsafe {_ZN10QSemaphoreC1Ei(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN10QSemaphoreC1Ei(arg0)};
    let rsthis = QSemaphore{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QSemaphore::~QSemaphore();
impl /*struct*/ QSemaphore {
  pub fn Free<RetType, T: QSemaphore_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QSemaphore_Free<RetType> {
  fn Free(self , rsthis: & QSemaphore) -> RetType;
}

  // proto:  void QSemaphore::~QSemaphore();
impl<'a> /*trait*/ QSemaphore_Free<()> for () {
  fn Free(self , rsthis: & QSemaphore) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QSemaphoreD0Ev()};
     unsafe {_ZN10QSemaphoreD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// <= body block end

