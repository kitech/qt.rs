// auto generated, do not modify.
// created: Thu Dec 24 23:00:39 2015
// src-file: /QtCore/qreadwritelock.h
// dst-file: /src/core/qreadwritelock.rs
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
// use super::qreadwritelock::QReadWriteLock; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]

// #[link(name = "QtInline")]

extern {
  // proto:  QReadWriteLock * QWriteLocker::readWriteLock();
  fn _ZNK12QWriteLocker13readWriteLockEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QWriteLocker::QWriteLocker(QReadWriteLock * readWriteLock);
  fn _ZN12QWriteLockerC1EP14QReadWriteLock(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QWriteLocker::unlock();
  fn _ZN12QWriteLocker6unlockEv(qthis: *mut c_void);
  // proto:  void QWriteLocker::~QWriteLocker();
  fn _ZN12QWriteLockerD0Ev(qthis: *mut c_void);
  // proto:  void QWriteLocker::relock();
  fn _ZN12QWriteLocker6relockEv(qthis: *mut c_void);
  // proto:  void QWriteLocker::QWriteLocker(const QWriteLocker & );
  fn _ZN12QWriteLockerC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QReadWriteLock::~QReadWriteLock();
  fn _ZN14QReadWriteLockD0Ev(qthis: *mut c_void);
  // proto:  void QReadWriteLock::QReadWriteLock(const QReadWriteLock & );
  fn _ZN14QReadWriteLockC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QReadWriteLock::tryLockForRead();
  fn _ZN14QReadWriteLock14tryLockForReadEv(qthis: *mut c_void) -> c_char;
  // proto:  void QReadWriteLock::lockForWrite();
  fn _ZN14QReadWriteLock12lockForWriteEv(qthis: *mut c_void);
  // proto:  bool QReadWriteLock::tryLockForWrite();
  fn _ZN14QReadWriteLock15tryLockForWriteEv(qthis: *mut c_void) -> c_char;
  // proto:  void QReadWriteLock::unlock();
  fn _ZN14QReadWriteLock6unlockEv(qthis: *mut c_void);
  // proto:  bool QReadWriteLock::tryLockForRead(int timeout);
  fn _ZN14QReadWriteLock14tryLockForReadEi(qthis: *mut c_void, arg0: c_int) -> c_char;
  // proto:  void QReadWriteLock::lockForRead();
  fn _ZN14QReadWriteLock11lockForReadEv(qthis: *mut c_void);
  // proto:  bool QReadWriteLock::tryLockForWrite(int timeout);
  fn _ZN14QReadWriteLock15tryLockForWriteEi(qthis: *mut c_void, arg0: c_int) -> c_char;
  // proto:  QReadWriteLock * QReadLocker::readWriteLock();
  fn _ZNK11QReadLocker13readWriteLockEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QReadLocker::~QReadLocker();
  fn _ZN11QReadLockerD0Ev(qthis: *mut c_void);
  // proto:  void QReadLocker::QReadLocker(QReadWriteLock * readWriteLock);
  fn _ZN11QReadLockerC1EP14QReadWriteLock(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QReadLocker::QReadLocker(const QReadLocker & );
  fn _ZN11QReadLockerC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QReadLocker::relock();
  fn _ZN11QReadLocker6relockEv(qthis: *mut c_void);
  // proto:  void QReadLocker::unlock();
  fn _ZN11QReadLocker6unlockEv(qthis: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QWriteLocker)=4
pub struct QWriteLocker {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

// class sizeof(QReadWriteLock)=8
pub struct QReadWriteLock {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

// class sizeof(QReadLocker)=4
pub struct QReadLocker {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QWriteLocker {
  pub fn inheritFrom(qthis: *mut c_void) -> QWriteLocker {
    return QWriteLocker{qclsinst: qthis};
  }
}
  // proto:  QReadWriteLock * QWriteLocker::readWriteLock();
impl /*struct*/ QWriteLocker {
  pub fn readWriteLock<RetType, T: QWriteLocker_readWriteLock<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.readWriteLock(self);
    // return 1;
  }
}

pub trait QWriteLocker_readWriteLock<RetType> {
  fn readWriteLock(self , rsthis: & QWriteLocker) -> RetType;
}

  // proto:  QReadWriteLock * QWriteLocker::readWriteLock();
impl<'a> /*trait*/ QWriteLocker_readWriteLock<QReadWriteLock> for () {
  fn readWriteLock(self , rsthis: & QWriteLocker) -> QReadWriteLock {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QWriteLocker13readWriteLockEv()};
    let mut ret = unsafe {_ZNK12QWriteLocker13readWriteLockEv(rsthis.qclsinst)};
    let mut ret1 = QReadWriteLock::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QWriteLocker::QWriteLocker(QReadWriteLock * readWriteLock);
impl /*struct*/ QWriteLocker {
  pub fn New<T: QWriteLocker_New>(value: T) -> QWriteLocker {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QWriteLocker_New {
  fn New(self) -> QWriteLocker;
}

  // proto:  void QWriteLocker::QWriteLocker(QReadWriteLock * readWriteLock);
impl<'a> /*trait*/ QWriteLocker_New for (&'a QReadWriteLock) {
  fn New(self) -> QWriteLocker {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QWriteLockerC1EP14QReadWriteLock()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN12QWriteLockerC1EP14QReadWriteLock(qthis, arg0)};
    let rsthis = QWriteLocker{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QWriteLocker::unlock();
impl /*struct*/ QWriteLocker {
  pub fn unlock<RetType, T: QWriteLocker_unlock<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.unlock(self);
    // return 1;
  }
}

pub trait QWriteLocker_unlock<RetType> {
  fn unlock(self , rsthis: & QWriteLocker) -> RetType;
}

  // proto:  void QWriteLocker::unlock();
impl<'a> /*trait*/ QWriteLocker_unlock<()> for () {
  fn unlock(self , rsthis: & QWriteLocker) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QWriteLocker6unlockEv()};
     unsafe {_ZN12QWriteLocker6unlockEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QWriteLocker::~QWriteLocker();
impl /*struct*/ QWriteLocker {
  pub fn Free<RetType, T: QWriteLocker_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QWriteLocker_Free<RetType> {
  fn Free(self , rsthis: & QWriteLocker) -> RetType;
}

  // proto:  void QWriteLocker::~QWriteLocker();
impl<'a> /*trait*/ QWriteLocker_Free<()> for () {
  fn Free(self , rsthis: & QWriteLocker) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QWriteLockerD0Ev()};
     unsafe {_ZN12QWriteLockerD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QWriteLocker::relock();
impl /*struct*/ QWriteLocker {
  pub fn relock<RetType, T: QWriteLocker_relock<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.relock(self);
    // return 1;
  }
}

pub trait QWriteLocker_relock<RetType> {
  fn relock(self , rsthis: & QWriteLocker) -> RetType;
}

  // proto:  void QWriteLocker::relock();
impl<'a> /*trait*/ QWriteLocker_relock<()> for () {
  fn relock(self , rsthis: & QWriteLocker) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QWriteLocker6relockEv()};
     unsafe {_ZN12QWriteLocker6relockEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QWriteLocker::QWriteLocker(const QWriteLocker & );
impl<'a> /*trait*/ QWriteLocker_New for (&'a QWriteLocker) {
  fn New(self) -> QWriteLocker {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QWriteLockerC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN12QWriteLockerC1ERKS_(qthis, arg0)};
    let rsthis = QWriteLocker{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QReadWriteLock {
  pub fn inheritFrom(qthis: *mut c_void) -> QReadWriteLock {
    return QReadWriteLock{qclsinst: qthis};
  }
}
  // proto:  void QReadWriteLock::~QReadWriteLock();
impl /*struct*/ QReadWriteLock {
  pub fn Free<RetType, T: QReadWriteLock_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QReadWriteLock_Free<RetType> {
  fn Free(self , rsthis: & QReadWriteLock) -> RetType;
}

  // proto:  void QReadWriteLock::~QReadWriteLock();
impl<'a> /*trait*/ QReadWriteLock_Free<()> for () {
  fn Free(self , rsthis: & QReadWriteLock) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QReadWriteLockD0Ev()};
     unsafe {_ZN14QReadWriteLockD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QReadWriteLock::QReadWriteLock(const QReadWriteLock & );
impl /*struct*/ QReadWriteLock {
  pub fn New<T: QReadWriteLock_New>(value: T) -> QReadWriteLock {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QReadWriteLock_New {
  fn New(self) -> QReadWriteLock;
}

  // proto:  void QReadWriteLock::QReadWriteLock(const QReadWriteLock & );
impl<'a> /*trait*/ QReadWriteLock_New for (&'a QReadWriteLock) {
  fn New(self) -> QReadWriteLock {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QReadWriteLockC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN14QReadWriteLockC1ERKS_(qthis, arg0)};
    let rsthis = QReadWriteLock{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QReadWriteLock::tryLockForRead();
impl /*struct*/ QReadWriteLock {
  pub fn tryLockForRead<RetType, T: QReadWriteLock_tryLockForRead<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.tryLockForRead(self);
    // return 1;
  }
}

pub trait QReadWriteLock_tryLockForRead<RetType> {
  fn tryLockForRead(self , rsthis: & QReadWriteLock) -> RetType;
}

  // proto:  bool QReadWriteLock::tryLockForRead();
impl<'a> /*trait*/ QReadWriteLock_tryLockForRead<i8> for () {
  fn tryLockForRead(self , rsthis: & QReadWriteLock) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QReadWriteLock14tryLockForReadEv()};
    let mut ret = unsafe {_ZN14QReadWriteLock14tryLockForReadEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QReadWriteLock::lockForWrite();
impl /*struct*/ QReadWriteLock {
  pub fn lockForWrite<RetType, T: QReadWriteLock_lockForWrite<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.lockForWrite(self);
    // return 1;
  }
}

pub trait QReadWriteLock_lockForWrite<RetType> {
  fn lockForWrite(self , rsthis: & QReadWriteLock) -> RetType;
}

  // proto:  void QReadWriteLock::lockForWrite();
impl<'a> /*trait*/ QReadWriteLock_lockForWrite<()> for () {
  fn lockForWrite(self , rsthis: & QReadWriteLock) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QReadWriteLock12lockForWriteEv()};
     unsafe {_ZN14QReadWriteLock12lockForWriteEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QReadWriteLock::tryLockForWrite();
impl /*struct*/ QReadWriteLock {
  pub fn tryLockForWrite<RetType, T: QReadWriteLock_tryLockForWrite<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.tryLockForWrite(self);
    // return 1;
  }
}

pub trait QReadWriteLock_tryLockForWrite<RetType> {
  fn tryLockForWrite(self , rsthis: & QReadWriteLock) -> RetType;
}

  // proto:  bool QReadWriteLock::tryLockForWrite();
impl<'a> /*trait*/ QReadWriteLock_tryLockForWrite<i8> for () {
  fn tryLockForWrite(self , rsthis: & QReadWriteLock) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QReadWriteLock15tryLockForWriteEv()};
    let mut ret = unsafe {_ZN14QReadWriteLock15tryLockForWriteEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QReadWriteLock::unlock();
impl /*struct*/ QReadWriteLock {
  pub fn unlock<RetType, T: QReadWriteLock_unlock<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.unlock(self);
    // return 1;
  }
}

pub trait QReadWriteLock_unlock<RetType> {
  fn unlock(self , rsthis: & QReadWriteLock) -> RetType;
}

  // proto:  void QReadWriteLock::unlock();
impl<'a> /*trait*/ QReadWriteLock_unlock<()> for () {
  fn unlock(self , rsthis: & QReadWriteLock) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QReadWriteLock6unlockEv()};
     unsafe {_ZN14QReadWriteLock6unlockEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QReadWriteLock::tryLockForRead(int timeout);
impl<'a> /*trait*/ QReadWriteLock_tryLockForRead<i8> for (i32) {
  fn tryLockForRead(self , rsthis: & QReadWriteLock) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QReadWriteLock14tryLockForReadEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN14QReadWriteLock14tryLockForReadEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QReadWriteLock::lockForRead();
impl /*struct*/ QReadWriteLock {
  pub fn lockForRead<RetType, T: QReadWriteLock_lockForRead<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.lockForRead(self);
    // return 1;
  }
}

pub trait QReadWriteLock_lockForRead<RetType> {
  fn lockForRead(self , rsthis: & QReadWriteLock) -> RetType;
}

  // proto:  void QReadWriteLock::lockForRead();
impl<'a> /*trait*/ QReadWriteLock_lockForRead<()> for () {
  fn lockForRead(self , rsthis: & QReadWriteLock) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QReadWriteLock11lockForReadEv()};
     unsafe {_ZN14QReadWriteLock11lockForReadEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QReadWriteLock::tryLockForWrite(int timeout);
impl<'a> /*trait*/ QReadWriteLock_tryLockForWrite<i8> for (i32) {
  fn tryLockForWrite(self , rsthis: & QReadWriteLock) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QReadWriteLock15tryLockForWriteEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN14QReadWriteLock15tryLockForWriteEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QReadLocker {
  pub fn inheritFrom(qthis: *mut c_void) -> QReadLocker {
    return QReadLocker{qclsinst: qthis};
  }
}
  // proto:  QReadWriteLock * QReadLocker::readWriteLock();
impl /*struct*/ QReadLocker {
  pub fn readWriteLock<RetType, T: QReadLocker_readWriteLock<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.readWriteLock(self);
    // return 1;
  }
}

pub trait QReadLocker_readWriteLock<RetType> {
  fn readWriteLock(self , rsthis: & QReadLocker) -> RetType;
}

  // proto:  QReadWriteLock * QReadLocker::readWriteLock();
impl<'a> /*trait*/ QReadLocker_readWriteLock<QReadWriteLock> for () {
  fn readWriteLock(self , rsthis: & QReadLocker) -> QReadWriteLock {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QReadLocker13readWriteLockEv()};
    let mut ret = unsafe {_ZNK11QReadLocker13readWriteLockEv(rsthis.qclsinst)};
    let mut ret1 = QReadWriteLock::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QReadLocker::~QReadLocker();
impl /*struct*/ QReadLocker {
  pub fn Free<RetType, T: QReadLocker_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QReadLocker_Free<RetType> {
  fn Free(self , rsthis: & QReadLocker) -> RetType;
}

  // proto:  void QReadLocker::~QReadLocker();
impl<'a> /*trait*/ QReadLocker_Free<()> for () {
  fn Free(self , rsthis: & QReadLocker) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QReadLockerD0Ev()};
     unsafe {_ZN11QReadLockerD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QReadLocker::QReadLocker(QReadWriteLock * readWriteLock);
impl /*struct*/ QReadLocker {
  pub fn New<T: QReadLocker_New>(value: T) -> QReadLocker {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QReadLocker_New {
  fn New(self) -> QReadLocker;
}

  // proto:  void QReadLocker::QReadLocker(QReadWriteLock * readWriteLock);
impl<'a> /*trait*/ QReadLocker_New for (&'a QReadWriteLock) {
  fn New(self) -> QReadLocker {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QReadLockerC1EP14QReadWriteLock()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QReadLockerC1EP14QReadWriteLock(qthis, arg0)};
    let rsthis = QReadLocker{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QReadLocker::QReadLocker(const QReadLocker & );
impl<'a> /*trait*/ QReadLocker_New for (&'a QReadLocker) {
  fn New(self) -> QReadLocker {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QReadLockerC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QReadLockerC1ERKS_(qthis, arg0)};
    let rsthis = QReadLocker{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QReadLocker::relock();
impl /*struct*/ QReadLocker {
  pub fn relock<RetType, T: QReadLocker_relock<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.relock(self);
    // return 1;
  }
}

pub trait QReadLocker_relock<RetType> {
  fn relock(self , rsthis: & QReadLocker) -> RetType;
}

  // proto:  void QReadLocker::relock();
impl<'a> /*trait*/ QReadLocker_relock<()> for () {
  fn relock(self , rsthis: & QReadLocker) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QReadLocker6relockEv()};
     unsafe {_ZN11QReadLocker6relockEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QReadLocker::unlock();
impl /*struct*/ QReadLocker {
  pub fn unlock<RetType, T: QReadLocker_unlock<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.unlock(self);
    // return 1;
  }
}

pub trait QReadLocker_unlock<RetType> {
  fn unlock(self , rsthis: & QReadLocker) -> RetType;
}

  // proto:  void QReadLocker::unlock();
impl<'a> /*trait*/ QReadLocker_unlock<()> for () {
  fn unlock(self , rsthis: & QReadLocker) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QReadLocker6unlockEv()};
     unsafe {_ZN11QReadLocker6unlockEv(rsthis.qclsinst)};
    // return 1;
  }
}

// <= body block end

