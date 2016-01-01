// auto generated, do not modify.
// created: Fri Jan  1 15:54:32 2016
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
// use super::qmutex::QMutex; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QMutexLocker_Class_Size() -> c_int;
  // proto:  void QMutexLocker::QMutexLocker(QBasicMutex * m);
  fn dector_ZN12QMutexLockerC1EP11QBasicMutex(arg0: *mut c_void) -> *mut c_void;
  fn demth_ZN12QMutexLockerC1EP11QBasicMutex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QMutex * QMutexLocker::mutex();
  fn demth_ZNK12QMutexLocker5mutexEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QMutexLocker::QMutexLocker(const QMutexLocker & );
  fn dector_ZN12QMutexLockerC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN12QMutexLockerC1ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QMutexLocker::relock();
  fn demth_ZN12QMutexLocker6relockEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QMutexLocker::unlock();
  fn demth_ZN12QMutexLocker6unlockEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QMutexLocker::~QMutexLocker();
  fn demth_ZN12QMutexLockerD0Ev(qthis: u64 /* *mut c_void*/);
  fn QBasicMutex_Class_Size() -> c_int;
  // proto:  void QBasicMutex::lock();
  fn demth_ZN11QBasicMutex4lockEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QBasicMutex::tryLock();
  fn _ZN11QBasicMutex7tryLockEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QBasicMutex::isRecursive();
  fn _ZN11QBasicMutex11isRecursiveEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QBasicMutex::unlock();
  fn demth_ZN11QBasicMutex6unlockEv(qthis: u64 /* *mut c_void*/);
  fn QMutex_Class_Size() -> c_int;
  // proto:  void QMutex::~QMutex();
  fn _ZN6QMutexD0Ev(qthis: u64 /* *mut c_void*/);
  // proto:  bool QMutex::tryLock(int timeout);
  fn _ZN6QMutex7tryLockEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> c_char;
  // proto:  void QMutex::QMutex(const QMutex & );
  fn dector_ZN6QMutexC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN6QMutexC1ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QMutex::lock();
  fn _ZN6QMutex4lockEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QMutex::unlock();
  fn _ZN6QMutex6unlockEv(qthis: u64 /* *mut c_void*/);
} // <= ext block end

// body block begin =>
// class sizeof(QMutexLocker)=4
#[derive(Default)]
pub struct QMutexLocker {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QBasicMutex)=1
#[derive(Default)]
pub struct QBasicMutex {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QMutex)=1
#[derive(Default)]
pub struct QMutex {
  qbase: QBasicMutex,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QMutexLocker {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QMutexLocker {
    return QMutexLocker{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void QMutexLocker::QMutexLocker(QBasicMutex * m);
impl /*struct*/ QMutexLocker {
  pub fn new<T: QMutexLocker_new>(value: T) -> QMutexLocker {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QMutexLocker_new {
  fn new(self) -> QMutexLocker;
}

  // proto:  void QMutexLocker::QMutexLocker(QBasicMutex * m);
impl<'a> /*trait*/ QMutexLocker_new for (&'a QBasicMutex) {
  fn new(self) -> QMutexLocker {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QMutexLockerC1EP11QBasicMutex()};
    let ctysz: c_int = unsafe{QMutexLocker_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN12QMutexLockerC1EP11QBasicMutex(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN12QMutexLockerC1EP11QBasicMutex(arg0)} as u64;
    let rsthis = QMutexLocker{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QMutex * QMutexLocker::mutex();
impl /*struct*/ QMutexLocker {
  pub fn mutex<RetType, T: QMutexLocker_mutex<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.mutex(self);
    // return 1;
  }
}

pub trait QMutexLocker_mutex<RetType> {
  fn mutex(self , rsthis: & QMutexLocker) -> RetType;
}

  // proto:  QMutex * QMutexLocker::mutex();
impl<'a> /*trait*/ QMutexLocker_mutex<QMutex> for () {
  fn mutex(self , rsthis: & QMutexLocker) -> QMutex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QMutexLocker5mutexEv()};
    let mut ret = unsafe {demth_ZNK12QMutexLocker5mutexEv(rsthis.qclsinst)};
    let mut ret1 = QMutex::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QMutexLocker::QMutexLocker(const QMutexLocker & );
impl<'a> /*trait*/ QMutexLocker_new for (&'a QMutexLocker) {
  fn new(self) -> QMutexLocker {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QMutexLockerC1ERKS_()};
    let ctysz: c_int = unsafe{QMutexLocker_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN12QMutexLockerC1ERKS_(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN12QMutexLockerC1ERKS_(arg0)} as u64;
    let rsthis = QMutexLocker{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QMutexLocker::relock();
impl /*struct*/ QMutexLocker {
  pub fn relock<RetType, T: QMutexLocker_relock<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.relock(self);
    // return 1;
  }
}

pub trait QMutexLocker_relock<RetType> {
  fn relock(self , rsthis: & QMutexLocker) -> RetType;
}

  // proto:  void QMutexLocker::relock();
impl<'a> /*trait*/ QMutexLocker_relock<()> for () {
  fn relock(self , rsthis: & QMutexLocker) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QMutexLocker6relockEv()};
     unsafe {demth_ZN12QMutexLocker6relockEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QMutexLocker::unlock();
impl /*struct*/ QMutexLocker {
  pub fn unlock<RetType, T: QMutexLocker_unlock<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.unlock(self);
    // return 1;
  }
}

pub trait QMutexLocker_unlock<RetType> {
  fn unlock(self , rsthis: & QMutexLocker) -> RetType;
}

  // proto:  void QMutexLocker::unlock();
impl<'a> /*trait*/ QMutexLocker_unlock<()> for () {
  fn unlock(self , rsthis: & QMutexLocker) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QMutexLocker6unlockEv()};
     unsafe {demth_ZN12QMutexLocker6unlockEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QMutexLocker::~QMutexLocker();
impl /*struct*/ QMutexLocker {
  pub fn free<RetType, T: QMutexLocker_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QMutexLocker_free<RetType> {
  fn free(self , rsthis: & QMutexLocker) -> RetType;
}

  // proto:  void QMutexLocker::~QMutexLocker();
impl<'a> /*trait*/ QMutexLocker_free<()> for () {
  fn free(self , rsthis: & QMutexLocker) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QMutexLockerD0Ev()};
     unsafe {demth_ZN12QMutexLockerD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QBasicMutex {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QBasicMutex {
    return QBasicMutex{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void QBasicMutex::lock();
impl /*struct*/ QBasicMutex {
  pub fn lock<RetType, T: QBasicMutex_lock<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.lock(self);
    // return 1;
  }
}

pub trait QBasicMutex_lock<RetType> {
  fn lock(self , rsthis: & QBasicMutex) -> RetType;
}

  // proto:  void QBasicMutex::lock();
impl<'a> /*trait*/ QBasicMutex_lock<()> for () {
  fn lock(self , rsthis: & QBasicMutex) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QBasicMutex4lockEv()};
     unsafe {demth_ZN11QBasicMutex4lockEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QBasicMutex::tryLock();
impl /*struct*/ QBasicMutex {
  pub fn tryLock<RetType, T: QBasicMutex_tryLock<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.tryLock(self);
    // return 1;
  }
}

pub trait QBasicMutex_tryLock<RetType> {
  fn tryLock(self , rsthis: & QBasicMutex) -> RetType;
}

  // proto:  bool QBasicMutex::tryLock();
impl<'a> /*trait*/ QBasicMutex_tryLock<i8> for () {
  fn tryLock(self , rsthis: & QBasicMutex) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QBasicMutex7tryLockEv()};
    let mut ret = unsafe {_ZN11QBasicMutex7tryLockEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
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

  // proto:  void QBasicMutex::unlock();
impl /*struct*/ QBasicMutex {
  pub fn unlock<RetType, T: QBasicMutex_unlock<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.unlock(self);
    // return 1;
  }
}

pub trait QBasicMutex_unlock<RetType> {
  fn unlock(self , rsthis: & QBasicMutex) -> RetType;
}

  // proto:  void QBasicMutex::unlock();
impl<'a> /*trait*/ QBasicMutex_unlock<()> for () {
  fn unlock(self , rsthis: & QBasicMutex) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QBasicMutex6unlockEv()};
     unsafe {demth_ZN11QBasicMutex6unlockEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QMutex {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QMutex {
    return QMutex{qbase: QBasicMutex::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
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
  pub fn free<RetType, T: QMutex_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QMutex_free<RetType> {
  fn free(self , rsthis: & QMutex) -> RetType;
}

  // proto:  void QMutex::~QMutex();
impl<'a> /*trait*/ QMutex_free<()> for () {
  fn free(self , rsthis: & QMutex) -> () {
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
  pub fn new<T: QMutex_new>(value: T) -> QMutex {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QMutex_new {
  fn new(self) -> QMutex;
}

  // proto:  void QMutex::QMutex(const QMutex & );
impl<'a> /*trait*/ QMutex_new for (&'a QMutex) {
  fn new(self) -> QMutex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QMutexC1ERKS_()};
    let ctysz: c_int = unsafe{QMutex_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN6QMutexC1ERKS_(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN6QMutexC1ERKS_(arg0)} as u64;
    let rsthis = QMutex{qbase: QBasicMutex::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
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

