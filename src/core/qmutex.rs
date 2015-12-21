// auto generated, do not modify.
// created: Mon Dec 21 22:54:38 2015
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
// use super::qmutex::QBasicMutex; // 773
// use super::qmutex::QMutex; // 773
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  void QMutexLocker::QMutexLocker(QBasicMutex * m);
  fn _ZN12QMutexLockerC1EP11QBasicMutex(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QMutex * QMutexLocker::mutex();
  fn _ZNK12QMutexLocker5mutexEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QMutexLocker::QMutexLocker(const QMutexLocker & );
  fn _ZN12QMutexLockerC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QMutexLocker::relock();
  fn _ZN12QMutexLocker6relockEv(qthis: *mut c_void);
  // proto:  void QMutexLocker::unlock();
  fn _ZN12QMutexLocker6unlockEv(qthis: *mut c_void);
  // proto:  void QMutexLocker::~QMutexLocker();
  fn _ZN12QMutexLockerD0Ev(qthis: *mut c_void);
  // proto:  void QBasicMutex::lock();
  fn _ZN11QBasicMutex4lockEv(qthis: *mut c_void);
  // proto:  bool QBasicMutex::tryLock();
  fn _ZN11QBasicMutex7tryLockEv(qthis: *mut c_void) -> c_char;
  // proto:  bool QBasicMutex::isRecursive();
  fn _ZN11QBasicMutex11isRecursiveEv(qthis: *mut c_void) -> c_char;
  // proto:  void QBasicMutex::unlock();
  fn _ZN11QBasicMutex6unlockEv(qthis: *mut c_void);
  // proto:  void QMutex::~QMutex();
  fn _ZN6QMutexD0Ev(qthis: *mut c_void);
  // proto:  bool QMutex::tryLock(int timeout);
  fn _ZN6QMutex7tryLockEi(qthis: *mut c_void, arg0: c_int) -> c_char;
  // proto:  void QMutex::QMutex(const QMutex & );
  fn _ZN6QMutexC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QMutex::lock();
  fn _ZN6QMutex4lockEv(qthis: *mut c_void);
  // proto:  void QMutex::unlock();
  fn _ZN6QMutex6unlockEv(qthis: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QMutexLocker)=4
pub struct QMutexLocker {
  pub qclsinst: *mut c_void,
}

// class sizeof(QBasicMutex)=1
pub struct QBasicMutex {
  pub qclsinst: *mut c_void,
}

// class sizeof(QMutex)=1
pub struct QMutex {
  pub qclsinst: *mut c_void,
}

  // proto:  void QMutexLocker::QMutexLocker(QBasicMutex * m);
impl /*struct*/ QMutexLocker {
  pub fn NewQMutexLocker<T: QMutexLocker_NewQMutexLocker>(value: T) -> QMutexLocker {
    let rsthis = value.NewQMutexLocker();
    return rsthis;
    // return 1;
  }
}

pub trait QMutexLocker_NewQMutexLocker {
  fn NewQMutexLocker(self) -> QMutexLocker;
}

  // proto:  void QMutexLocker::QMutexLocker(QBasicMutex * m);
impl<'a> /*trait*/ QMutexLocker_NewQMutexLocker for (QBasicMutex) {
  fn NewQMutexLocker(self) -> QMutexLocker {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QMutexLockerC1EP11QBasicMutex()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN12QMutexLockerC1EP11QBasicMutex(qthis, arg0)};
    let rsthis = QMutexLocker{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QMutex * QMutexLocker::mutex();
impl /*struct*/ QMutexLocker {
  pub fn mutex<RetType, T: QMutexLocker_mutex<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.mutex(self);
    // return 1;
  }
}

pub trait QMutexLocker_mutex<RetType> {
  fn mutex(self , rsthis: &mut QMutexLocker) -> RetType;
}

  // proto:  QMutex * QMutexLocker::mutex();
impl<'a> /*trait*/ QMutexLocker_mutex<QMutex> for () {
  fn mutex(self , rsthis: &mut QMutexLocker) -> QMutex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QMutexLocker5mutexEv()};
    let mut ret = unsafe {_ZNK12QMutexLocker5mutexEv(rsthis.qclsinst)};
    let mut ret1 = QMutex{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QMutexLocker::QMutexLocker(const QMutexLocker & );
impl<'a> /*trait*/ QMutexLocker_NewQMutexLocker for (QMutexLocker) {
  fn NewQMutexLocker(self) -> QMutexLocker {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QMutexLockerC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN12QMutexLockerC1ERKS_(qthis, arg0)};
    let rsthis = QMutexLocker{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QMutexLocker::relock();
impl /*struct*/ QMutexLocker {
  pub fn relock<RetType, T: QMutexLocker_relock<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.relock(self);
    // return 1;
  }
}

pub trait QMutexLocker_relock<RetType> {
  fn relock(self , rsthis: &mut QMutexLocker) -> RetType;
}

  // proto:  void QMutexLocker::relock();
impl<'a> /*trait*/ QMutexLocker_relock<()> for () {
  fn relock(self , rsthis: &mut QMutexLocker) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QMutexLocker6relockEv()};
     unsafe {_ZN12QMutexLocker6relockEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QMutexLocker::unlock();
impl /*struct*/ QMutexLocker {
  pub fn unlock<RetType, T: QMutexLocker_unlock<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.unlock(self);
    // return 1;
  }
}

pub trait QMutexLocker_unlock<RetType> {
  fn unlock(self , rsthis: &mut QMutexLocker) -> RetType;
}

  // proto:  void QMutexLocker::unlock();
impl<'a> /*trait*/ QMutexLocker_unlock<()> for () {
  fn unlock(self , rsthis: &mut QMutexLocker) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QMutexLocker6unlockEv()};
     unsafe {_ZN12QMutexLocker6unlockEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QMutexLocker::~QMutexLocker();
impl /*struct*/ QMutexLocker {
  pub fn FreeQMutexLocker<RetType, T: QMutexLocker_FreeQMutexLocker<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQMutexLocker(self);
    // return 1;
  }
}

pub trait QMutexLocker_FreeQMutexLocker<RetType> {
  fn FreeQMutexLocker(self , rsthis: &mut QMutexLocker) -> RetType;
}

  // proto:  void QMutexLocker::~QMutexLocker();
impl<'a> /*trait*/ QMutexLocker_FreeQMutexLocker<()> for () {
  fn FreeQMutexLocker(self , rsthis: &mut QMutexLocker) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QMutexLockerD0Ev()};
     unsafe {_ZN12QMutexLockerD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QBasicMutex::lock();
impl /*struct*/ QBasicMutex {
  pub fn lock<RetType, T: QBasicMutex_lock<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.lock(self);
    // return 1;
  }
}

pub trait QBasicMutex_lock<RetType> {
  fn lock(self , rsthis: &mut QBasicMutex) -> RetType;
}

  // proto:  void QBasicMutex::lock();
impl<'a> /*trait*/ QBasicMutex_lock<()> for () {
  fn lock(self , rsthis: &mut QBasicMutex) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QBasicMutex4lockEv()};
     unsafe {_ZN11QBasicMutex4lockEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QBasicMutex::tryLock();
impl /*struct*/ QBasicMutex {
  pub fn tryLock<RetType, T: QBasicMutex_tryLock<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.tryLock(self);
    // return 1;
  }
}

pub trait QBasicMutex_tryLock<RetType> {
  fn tryLock(self , rsthis: &mut QBasicMutex) -> RetType;
}

  // proto:  bool QBasicMutex::tryLock();
impl<'a> /*trait*/ QBasicMutex_tryLock<i8> for () {
  fn tryLock(self , rsthis: &mut QBasicMutex) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QBasicMutex7tryLockEv()};
    let mut ret = unsafe {_ZN11QBasicMutex7tryLockEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QBasicMutex::isRecursive();
impl /*struct*/ QBasicMutex {
  pub fn isRecursive<RetType, T: QBasicMutex_isRecursive<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isRecursive(self);
    // return 1;
  }
}

pub trait QBasicMutex_isRecursive<RetType> {
  fn isRecursive(self , rsthis: &mut QBasicMutex) -> RetType;
}

  // proto:  bool QBasicMutex::isRecursive();
impl<'a> /*trait*/ QBasicMutex_isRecursive<i8> for () {
  fn isRecursive(self , rsthis: &mut QBasicMutex) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QBasicMutex11isRecursiveEv()};
    let mut ret = unsafe {_ZN11QBasicMutex11isRecursiveEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QBasicMutex::unlock();
impl /*struct*/ QBasicMutex {
  pub fn unlock<RetType, T: QBasicMutex_unlock<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.unlock(self);
    // return 1;
  }
}

pub trait QBasicMutex_unlock<RetType> {
  fn unlock(self , rsthis: &mut QBasicMutex) -> RetType;
}

  // proto:  void QBasicMutex::unlock();
impl<'a> /*trait*/ QBasicMutex_unlock<()> for () {
  fn unlock(self , rsthis: &mut QBasicMutex) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QBasicMutex6unlockEv()};
     unsafe {_ZN11QBasicMutex6unlockEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QMutex::~QMutex();
impl /*struct*/ QMutex {
  pub fn FreeQMutex<RetType, T: QMutex_FreeQMutex<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQMutex(self);
    // return 1;
  }
}

pub trait QMutex_FreeQMutex<RetType> {
  fn FreeQMutex(self , rsthis: &mut QMutex) -> RetType;
}

  // proto:  void QMutex::~QMutex();
impl<'a> /*trait*/ QMutex_FreeQMutex<()> for () {
  fn FreeQMutex(self , rsthis: &mut QMutex) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QMutexD0Ev()};
     unsafe {_ZN6QMutexD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QMutex::tryLock(int timeout);
impl /*struct*/ QMutex {
  pub fn tryLock<RetType, T: QMutex_tryLock<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.tryLock(self);
    // return 1;
  }
}

pub trait QMutex_tryLock<RetType> {
  fn tryLock(self , rsthis: &mut QMutex) -> RetType;
}

  // proto:  bool QMutex::tryLock(int timeout);
impl<'a> /*trait*/ QMutex_tryLock<i8> for (i32) {
  fn tryLock(self , rsthis: &mut QMutex) -> i8 {
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
  pub fn NewQMutex<T: QMutex_NewQMutex>(value: T) -> QMutex {
    let rsthis = value.NewQMutex();
    return rsthis;
    // return 1;
  }
}

pub trait QMutex_NewQMutex {
  fn NewQMutex(self) -> QMutex;
}

  // proto:  void QMutex::QMutex(const QMutex & );
impl<'a> /*trait*/ QMutex_NewQMutex for (QMutex) {
  fn NewQMutex(self) -> QMutex {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QMutexC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN6QMutexC1ERKS_(qthis, arg0)};
    let rsthis = QMutex{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QMutex::lock();
impl /*struct*/ QMutex {
  pub fn lock<RetType, T: QMutex_lock<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.lock(self);
    // return 1;
  }
}

pub trait QMutex_lock<RetType> {
  fn lock(self , rsthis: &mut QMutex) -> RetType;
}

  // proto:  void QMutex::lock();
impl<'a> /*trait*/ QMutex_lock<()> for () {
  fn lock(self , rsthis: &mut QMutex) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QMutex4lockEv()};
     unsafe {_ZN6QMutex4lockEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QMutex::unlock();
impl /*struct*/ QMutex {
  pub fn unlock<RetType, T: QMutex_unlock<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.unlock(self);
    // return 1;
  }
}

pub trait QMutex_unlock<RetType> {
  fn unlock(self , rsthis: &mut QMutex) -> RetType;
}

  // proto:  void QMutex::unlock();
impl<'a> /*trait*/ QMutex_unlock<()> for () {
  fn unlock(self , rsthis: &mut QMutex) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QMutex6unlockEv()};
     unsafe {_ZN6QMutex6unlockEv(rsthis.qclsinst)};
    // return 1;
  }
}

// <= body block end
