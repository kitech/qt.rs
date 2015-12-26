// auto generated, do not modify.
// created: Sat Dec 26 10:16:52 2015
// src-file: /QtCore/qmutex.h
// dst-file: /src/core/qmutex.rs
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
// use super::qmutex::QBasicMutex; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QMutexLocker_Class_Size() -> c_int;
  // proto:  void QMutexLocker::QMutexLocker(const QMutexLocker & );
  fn dector_ZN12QMutexLockerC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN12QMutexLockerC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  fn QBasicMutex_Class_Size() -> c_int;
  // proto:  bool QBasicMutex::isRecursive();
  fn _ZN11QBasicMutex11isRecursiveEv(qthis: *mut c_void) -> c_char;
  fn QMutex_Class_Size() -> c_int;
  // proto:  void QMutex::~QMutex();
  fn _ZN6QMutexD0Ev(qthis: *mut c_void);
  // proto:  bool QMutex::tryLock(int timeout);
  fn _ZN6QMutex7tryLockEi(qthis: *mut c_void, arg0: c_int) -> c_char;
  // proto:  void QMutex::QMutex(const QMutex & );
  fn dector_ZN6QMutexC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN6QMutexC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QMutex::lock();
  fn _ZN6QMutex4lockEv(qthis: *mut c_void);
  // proto:  void QMutex::unlock();
  fn _ZN6QMutex6unlockEv(qthis: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QMutexLocker)=4
pub struct QMutexLocker {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

// class sizeof(QBasicMutex)=1
pub struct QBasicMutex {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

// class sizeof(QMutex)=1
pub struct QMutex {
  qbase: QBasicMutex,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QMutexLocker {
  pub fn inheritFrom(qthis: *mut c_void) -> QMutexLocker {
    return QMutexLocker{qclsinst: qthis};
  }
}
  // proto:  void QMutexLocker::QMutexLocker(const QMutexLocker & );
impl /*struct*/ QMutexLocker {
  pub fn New<T: QMutexLocker_New>(value: T) -> QMutexLocker {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QMutexLocker_New {
  fn New(self) -> QMutexLocker;
}

  // proto:  void QMutexLocker::QMutexLocker(const QMutexLocker & );
impl<'a> /*trait*/ QMutexLocker_New for (&'a QMutexLocker) {
  fn New(self) -> QMutexLocker {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QMutexLockerC1ERKS_()};
    let ctysz: c_int = unsafe{QMutexLocker_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN12QMutexLockerC1ERKS_(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN12QMutexLockerC1ERKS_(arg0)};
    let rsthis = QMutexLocker{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QBasicMutex {
  pub fn inheritFrom(qthis: *mut c_void) -> QBasicMutex {
    return QBasicMutex{qclsinst: qthis};
  }
}
  // proto:  bool QBasicMutex::isRecursive();
impl /*struct*/ QBasicMutex {
  pub fn isRecursive<RetType, T: QBasicMutex_isRecursive<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isRecursive(self);
    // return 1;
  }
}

pub trait QBasicMutex_isRecursive<RetType> {
  fn isRecursive(self , rsthis: & QBasicMutex) -> RetType;
}

  // proto:  bool QBasicMutex::isRecursive();
impl<'a> /*trait*/ QBasicMutex_isRecursive<i8> for () {
  fn isRecursive(self , rsthis: & QBasicMutex) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QBasicMutex11isRecursiveEv()};
    let mut ret = unsafe {_ZN11QBasicMutex11isRecursiveEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QMutex {
  pub fn inheritFrom(qthis: *mut c_void) -> QMutex {
    return QMutex{qbase: QBasicMutex::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QMutex {
  type Target = QBasicMutex;

  fn deref(&self) -> &QBasicMutex {
    return & self.qbase;
  }
}
impl AsRef<QBasicMutex> for QMutex {
  fn as_ref(& self) -> & QBasicMutex {
    return & self.qbase;
  }
}
  // proto:  void QMutex::~QMutex();
impl /*struct*/ QMutex {
  pub fn Free<RetType, T: QMutex_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QMutex_Free<RetType> {
  fn Free(self , rsthis: & QMutex) -> RetType;
}

  // proto:  void QMutex::~QMutex();
impl<'a> /*trait*/ QMutex_Free<()> for () {
  fn Free(self , rsthis: & QMutex) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QMutexD0Ev()};
     unsafe {_ZN6QMutexD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QMutex::tryLock(int timeout);
impl /*struct*/ QMutex {
  pub fn tryLock<RetType, T: QMutex_tryLock<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.tryLock(self);
    // return 1;
  }
}

pub trait QMutex_tryLock<RetType> {
  fn tryLock(self , rsthis: & QMutex) -> RetType;
}

  // proto:  bool QMutex::tryLock(int timeout);
impl<'a> /*trait*/ QMutex_tryLock<i8> for (i32) {
  fn tryLock(self , rsthis: & QMutex) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QMutex7tryLockEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN6QMutex7tryLockEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QMutex::QMutex(const QMutex & );
impl /*struct*/ QMutex {
  pub fn New<T: QMutex_New>(value: T) -> QMutex {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QMutex_New {
  fn New(self) -> QMutex;
}

  // proto:  void QMutex::QMutex(const QMutex & );
impl<'a> /*trait*/ QMutex_New for (&'a QMutex) {
  fn New(self) -> QMutex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QMutexC1ERKS_()};
    let ctysz: c_int = unsafe{QMutex_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN6QMutexC1ERKS_(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN6QMutexC1ERKS_(arg0)};
    let rsthis = QMutex{/**/qbase: QBasicMutex::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QMutex::lock();
impl /*struct*/ QMutex {
  pub fn lock<RetType, T: QMutex_lock<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.lock(self);
    // return 1;
  }
}

pub trait QMutex_lock<RetType> {
  fn lock(self , rsthis: & QMutex) -> RetType;
}

  // proto:  void QMutex::lock();
impl<'a> /*trait*/ QMutex_lock<()> for () {
  fn lock(self , rsthis: & QMutex) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QMutex4lockEv()};
     unsafe {_ZN6QMutex4lockEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QMutex::unlock();
impl /*struct*/ QMutex {
  pub fn unlock<RetType, T: QMutex_unlock<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.unlock(self);
    // return 1;
  }
}

pub trait QMutex_unlock<RetType> {
  fn unlock(self , rsthis: & QMutex) -> RetType;
}

  // proto:  void QMutex::unlock();
impl<'a> /*trait*/ QMutex_unlock<()> for () {
  fn unlock(self , rsthis: & QMutex) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QMutex6unlockEv()};
     unsafe {_ZN6QMutex6unlockEv(rsthis.qclsinst)};
    // return 1;
  }
}

// <= body block end

