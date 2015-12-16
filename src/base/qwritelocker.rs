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

impl /*struct*/ QWriteLocker {
  pub fn readWriteLock<T: QWriteLocker_readWriteLock>(&mut self, value: T) -> QReadWriteLock {
    return value.readWriteLock(self);
    // return 1;
  }
}

pub trait QWriteLocker_readWriteLock {
  fn readWriteLock(self, rsthis: &mut QWriteLocker) -> QReadWriteLock;
}

// proto:  QReadWriteLock * QWriteLocker::readWriteLock();
impl<'a> /*trait*/ QWriteLocker_readWriteLock for () {
  fn readWriteLock(self, rsthis: &mut QWriteLocker) -> QReadWriteLock {
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

impl /*struct*/ QWriteLocker {
  pub fn unlock<T: QWriteLocker_unlock>(&mut self, value: T)  {
     value.unlock(self);
    // return 1;
  }
}

pub trait QWriteLocker_unlock {
  fn unlock(self, rsthis: &mut QWriteLocker) ;
}

// proto:  void QWriteLocker::unlock();
impl<'a> /*trait*/ QWriteLocker_unlock for () {
  fn unlock(self, rsthis: &mut QWriteLocker)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QWriteLocker6unlockEv()};
     unsafe {_ZN12QWriteLocker6unlockEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QWriteLocker {
  pub fn FreeQWriteLocker<T: QWriteLocker_FreeQWriteLocker>(&mut self, value: T)  {
     value.FreeQWriteLocker(self);
    // return 1;
  }
}

pub trait QWriteLocker_FreeQWriteLocker {
  fn FreeQWriteLocker(self, rsthis: &mut QWriteLocker) ;
}

// proto:  void QWriteLocker::FreeQWriteLocker();
impl<'a> /*trait*/ QWriteLocker_FreeQWriteLocker for () {
  fn FreeQWriteLocker(self, rsthis: &mut QWriteLocker)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QWriteLockerD0Ev()};
     unsafe {_ZN12QWriteLockerD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QWriteLocker {
  pub fn relock<T: QWriteLocker_relock>(&mut self, value: T)  {
     value.relock(self);
    // return 1;
  }
}

pub trait QWriteLocker_relock {
  fn relock(self, rsthis: &mut QWriteLocker) ;
}

// proto:  void QWriteLocker::relock();
impl<'a> /*trait*/ QWriteLocker_relock for () {
  fn relock(self, rsthis: &mut QWriteLocker)  {
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

