// auto generated, do not modify.
// created: Thu Jan 28 22:38:45 2016
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
  fn QWriteLocker_Class_Size() -> c_int;
  // proto:  QReadWriteLock * QWriteLocker::readWriteLock();
  fn C_ZNK12QWriteLocker13readWriteLockEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QWriteLocker::QWriteLocker(QReadWriteLock * readWriteLock);
  fn C_ZN12QWriteLockerC2EP14QReadWriteLock(arg0: *mut c_void) -> u64;
  // proto:  void QWriteLocker::unlock();
  fn C_ZN12QWriteLocker6unlockEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QWriteLocker::~QWriteLocker();
  fn C_ZN12QWriteLockerD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QWriteLocker::relock();
  fn C_ZN12QWriteLocker6relockEv(qthis: u64 /* *mut c_void*/);
  fn QReadWriteLock_Class_Size() -> c_int;
  // proto:  void QReadWriteLock::~QReadWriteLock();
  fn C_ZN14QReadWriteLockD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  bool QReadWriteLock::tryLockForRead();
  fn C_ZN14QReadWriteLock14tryLockForReadEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QReadWriteLock::lockForWrite();
  fn C_ZN14QReadWriteLock12lockForWriteEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QReadWriteLock::tryLockForWrite();
  fn C_ZN14QReadWriteLock15tryLockForWriteEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QReadWriteLock::unlock();
  fn C_ZN14QReadWriteLock6unlockEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QReadWriteLock::tryLockForRead(int timeout);
  fn C_ZN14QReadWriteLock14tryLockForReadEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> c_char;
  // proto:  void QReadWriteLock::lockForRead();
  fn C_ZN14QReadWriteLock11lockForReadEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QReadWriteLock::tryLockForWrite(int timeout);
  fn C_ZN14QReadWriteLock15tryLockForWriteEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> c_char;
  fn QReadLocker_Class_Size() -> c_int;
  // proto:  QReadWriteLock * QReadLocker::readWriteLock();
  fn C_ZNK11QReadLocker13readWriteLockEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QReadLocker::~QReadLocker();
  fn C_ZN11QReadLockerD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QReadLocker::QReadLocker(QReadWriteLock * readWriteLock);
  fn C_ZN11QReadLockerC2EP14QReadWriteLock(arg0: *mut c_void) -> u64;
  // proto:  void QReadLocker::relock();
  fn C_ZN11QReadLocker6relockEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QReadLocker::unlock();
  fn C_ZN11QReadLocker6unlockEv(qthis: u64 /* *mut c_void*/);
} // <= ext block end

// body block begin =>
// class sizeof(QWriteLocker)=4
#[derive(Default)]
pub struct QWriteLocker {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QReadWriteLock)=8
#[derive(Default)]
pub struct QReadWriteLock {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QReadLocker)=4
#[derive(Default)]
pub struct QReadLocker {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QWriteLocker {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QWriteLocker {
    return QWriteLocker{qclsinst: qthis, ..Default::default()};
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
    let mut ret = unsafe {C_ZNK12QWriteLocker13readWriteLockEv(rsthis.qclsinst)};
    let mut ret1 = QReadWriteLock::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QWriteLocker::QWriteLocker(QReadWriteLock * readWriteLock);
impl /*struct*/ QWriteLocker {
  pub fn new<T: QWriteLocker_new>(value: T) -> QWriteLocker {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QWriteLocker_new {
  fn new(self) -> QWriteLocker;
}

  // proto:  void QWriteLocker::QWriteLocker(QReadWriteLock * readWriteLock);
impl<'a> /*trait*/ QWriteLocker_new for (&'a QReadWriteLock) {
  fn new(self) -> QWriteLocker {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QWriteLockerC2EP14QReadWriteLock()};
    let ctysz: c_int = unsafe{QWriteLocker_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN12QWriteLockerC2EP14QReadWriteLock(arg0)};
    let rsthis = QWriteLocker{qclsinst: qthis, ..Default::default()};
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
     unsafe {C_ZN12QWriteLocker6unlockEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QWriteLocker::~QWriteLocker();
impl /*struct*/ QWriteLocker {
  pub fn free<RetType, T: QWriteLocker_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QWriteLocker_free<RetType> {
  fn free(self , rsthis: & QWriteLocker) -> RetType;
}

  // proto:  void QWriteLocker::~QWriteLocker();
impl<'a> /*trait*/ QWriteLocker_free<()> for () {
  fn free(self , rsthis: & QWriteLocker) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QWriteLockerD2Ev()};
     unsafe {C_ZN12QWriteLockerD2Ev(rsthis.qclsinst)};
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
     unsafe {C_ZN12QWriteLocker6relockEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QReadWriteLock {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QReadWriteLock {
    return QReadWriteLock{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void QReadWriteLock::~QReadWriteLock();
impl /*struct*/ QReadWriteLock {
  pub fn free<RetType, T: QReadWriteLock_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QReadWriteLock_free<RetType> {
  fn free(self , rsthis: & QReadWriteLock) -> RetType;
}

  // proto:  void QReadWriteLock::~QReadWriteLock();
impl<'a> /*trait*/ QReadWriteLock_free<()> for () {
  fn free(self , rsthis: & QReadWriteLock) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QReadWriteLockD2Ev()};
     unsafe {C_ZN14QReadWriteLockD2Ev(rsthis.qclsinst)};
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
    let mut ret = unsafe {C_ZN14QReadWriteLock14tryLockForReadEv(rsthis.qclsinst)};
    return ret as i8; // 1
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
     unsafe {C_ZN14QReadWriteLock12lockForWriteEv(rsthis.qclsinst)};
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
    let mut ret = unsafe {C_ZN14QReadWriteLock15tryLockForWriteEv(rsthis.qclsinst)};
    return ret as i8; // 1
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
     unsafe {C_ZN14QReadWriteLock6unlockEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QReadWriteLock::tryLockForRead(int timeout);
impl<'a> /*trait*/ QReadWriteLock_tryLockForRead<i8> for (i32) {
  fn tryLockForRead(self , rsthis: & QReadWriteLock) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QReadWriteLock14tryLockForReadEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {C_ZN14QReadWriteLock14tryLockForReadEi(rsthis.qclsinst, arg0)};
    return ret as i8; // 1
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
     unsafe {C_ZN14QReadWriteLock11lockForReadEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QReadWriteLock::tryLockForWrite(int timeout);
impl<'a> /*trait*/ QReadWriteLock_tryLockForWrite<i8> for (i32) {
  fn tryLockForWrite(self , rsthis: & QReadWriteLock) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QReadWriteLock15tryLockForWriteEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {C_ZN14QReadWriteLock15tryLockForWriteEi(rsthis.qclsinst, arg0)};
    return ret as i8; // 1
    // return 1;
  }
}

impl /*struct*/ QReadLocker {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QReadLocker {
    return QReadLocker{qclsinst: qthis, ..Default::default()};
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
    let mut ret = unsafe {C_ZNK11QReadLocker13readWriteLockEv(rsthis.qclsinst)};
    let mut ret1 = QReadWriteLock::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QReadLocker::~QReadLocker();
impl /*struct*/ QReadLocker {
  pub fn free<RetType, T: QReadLocker_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QReadLocker_free<RetType> {
  fn free(self , rsthis: & QReadLocker) -> RetType;
}

  // proto:  void QReadLocker::~QReadLocker();
impl<'a> /*trait*/ QReadLocker_free<()> for () {
  fn free(self , rsthis: & QReadLocker) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QReadLockerD2Ev()};
     unsafe {C_ZN11QReadLockerD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QReadLocker::QReadLocker(QReadWriteLock * readWriteLock);
impl /*struct*/ QReadLocker {
  pub fn new<T: QReadLocker_new>(value: T) -> QReadLocker {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QReadLocker_new {
  fn new(self) -> QReadLocker;
}

  // proto:  void QReadLocker::QReadLocker(QReadWriteLock * readWriteLock);
impl<'a> /*trait*/ QReadLocker_new for (&'a QReadWriteLock) {
  fn new(self) -> QReadLocker {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QReadLockerC2EP14QReadWriteLock()};
    let ctysz: c_int = unsafe{QReadLocker_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN11QReadLockerC2EP14QReadWriteLock(arg0)};
    let rsthis = QReadLocker{qclsinst: qthis, ..Default::default()};
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
     unsafe {C_ZN11QReadLocker6relockEv(rsthis.qclsinst)};
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
     unsafe {C_ZN11QReadLocker6unlockEv(rsthis.qclsinst)};
    // return 1;
  }
}

// <= body block end

