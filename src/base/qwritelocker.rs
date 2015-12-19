// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qreadwritelock::QReadWriteLock;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  QReadWriteLock * QWriteLocker::readWriteLock();
  fn _ZNK12QWriteLocker13readWriteLockEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QWriteLocker::NewQWriteLocker(QReadWriteLock * readWriteLock);
  fn _ZN12QWriteLockerC1EP14QReadWriteLock(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QWriteLocker::unlock();
  fn _ZN12QWriteLocker6unlockEv(qthis: *mut c_void) ;
  // proto:  void QWriteLocker::FreeQWriteLocker();
  fn _ZN12QWriteLockerD0Ev(qthis: *mut c_void) ;
  // proto:  void QWriteLocker::relock();
  fn _ZN12QWriteLocker6relockEv(qthis: *mut c_void) ;
  // proto:  void QWriteLocker::NewQWriteLocker(const QWriteLocker & );
  fn _ZN12QWriteLockerC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
}

// body block begin
// class sizeof(QWriteLocker)=4
pub struct QWriteLocker {
  pub qclsinst: *mut c_void,
}

// proto:  QReadWriteLock * QWriteLocker::readWriteLock();
impl /*struct*/ QWriteLocker {
  pub fn readWriteLock<RetType, T: QWriteLocker_readWriteLock<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.readWriteLock(self);
    // return 1;
  }
}

pub trait QWriteLocker_readWriteLock<RetType> {
  fn readWriteLock(self , rsthis: &mut QWriteLocker) -> RetType;
}

// proto:  QReadWriteLock * QWriteLocker::readWriteLock();
impl<'a> /*trait*/ QWriteLocker_readWriteLock<QReadWriteLock> for () {
  fn readWriteLock(self , rsthis: &mut QWriteLocker) -> QReadWriteLock {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QWriteLocker13readWriteLockEv()};
    let mut ret = unsafe {_ZNK12QWriteLocker13readWriteLockEv(rsthis.qclsinst)};
    let mut ret1 = QReadWriteLock{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QWriteLocker {
  pub fn NewQWriteLocker<T: QWriteLocker_NewQWriteLocker>(value: T) -> QWriteLocker {
    let rsthis = value.NewQWriteLocker();
    return rsthis;
    // return 1;
  }
}

pub trait QWriteLocker_NewQWriteLocker {
  fn NewQWriteLocker(self) -> QWriteLocker;
}

// proto: void QWriteLocker::NewQWriteLocker(QReadWriteLock * readWriteLock);
impl<'a> /*trait*/ QWriteLocker_NewQWriteLocker for (&'a mut QReadWriteLock) {
  fn NewQWriteLocker(self) -> QWriteLocker {
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
  pub fn unlock<RetType, T: QWriteLocker_unlock<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.unlock(self);
    // return 1;
  }
}

pub trait QWriteLocker_unlock<RetType> {
  fn unlock(self , rsthis: &mut QWriteLocker) -> RetType;
}

// proto:  void QWriteLocker::unlock();
impl<'a> /*trait*/ QWriteLocker_unlock<()> for () {
  fn unlock(self , rsthis: &mut QWriteLocker) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QWriteLocker6unlockEv()};
     unsafe {_ZN12QWriteLocker6unlockEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  void QWriteLocker::FreeQWriteLocker();
impl /*struct*/ QWriteLocker {
  pub fn FreeQWriteLocker<RetType, T: QWriteLocker_FreeQWriteLocker<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.FreeQWriteLocker(self);
    // return 1;
  }
}

pub trait QWriteLocker_FreeQWriteLocker<RetType> {
  fn FreeQWriteLocker(self , rsthis: &mut QWriteLocker) -> RetType;
}

// proto:  void QWriteLocker::FreeQWriteLocker();
impl<'a> /*trait*/ QWriteLocker_FreeQWriteLocker<()> for () {
  fn FreeQWriteLocker(self , rsthis: &mut QWriteLocker) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QWriteLockerD0Ev()};
     unsafe {_ZN12QWriteLockerD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  void QWriteLocker::relock();
impl /*struct*/ QWriteLocker {
  pub fn relock<RetType, T: QWriteLocker_relock<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.relock(self);
    // return 1;
  }
}

pub trait QWriteLocker_relock<RetType> {
  fn relock(self , rsthis: &mut QWriteLocker) -> RetType;
}

// proto:  void QWriteLocker::relock();
impl<'a> /*trait*/ QWriteLocker_relock<()> for () {
  fn relock(self , rsthis: &mut QWriteLocker) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QWriteLocker6relockEv()};
     unsafe {_ZN12QWriteLocker6relockEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto: void QWriteLocker::NewQWriteLocker(const QWriteLocker & );
impl<'a> /*trait*/ QWriteLocker_NewQWriteLocker for (&'a  QWriteLocker) {
  fn NewQWriteLocker(self) -> QWriteLocker {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QWriteLockerC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN12QWriteLockerC1ERKS_(qthis, arg0)};
    let rsthis = QWriteLocker{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

