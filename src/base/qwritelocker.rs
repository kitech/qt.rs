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
  fn _ZNK12QWriteLocker13readWriteLockEv() -> i32;
  fn _ZN12QWriteLockerC1EP14QReadWriteLock(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  fn _ZN12QWriteLocker6unlockEv() -> i32;
  fn _ZN12QWriteLockerD0Ev() -> i32;
  fn _ZN12QWriteLocker6relockEv() -> i32;
  fn _ZN12QWriteLockerC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
}

// body block begin
// class sizeof(QWriteLocker)=4
pub struct QWriteLocker {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QWriteLocker {
  pub fn readWriteLock<T: QWriteLocker_readWriteLock>(&mut self, value: T) -> i32 {
    value.readWriteLock(self);
    return 1;
  }
}

pub trait QWriteLocker_readWriteLock {
  fn readWriteLock(self, this: &mut QWriteLocker) -> i32;
}

// proto: QReadWriteLock * QWriteLocker::readWriteLock();
impl<'a> /*trait*/ QWriteLocker_readWriteLock for () {
  fn readWriteLock(self, this: &mut QWriteLocker) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QWriteLocker13readWriteLockEv()};
    unsafe {_ZNK12QWriteLocker13readWriteLockEv()};
    return 1;
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
  pub fn unlock<T: QWriteLocker_unlock>(&mut self, value: T) -> i32 {
    value.unlock(self);
    return 1;
  }
}

pub trait QWriteLocker_unlock {
  fn unlock(self, this: &mut QWriteLocker) -> i32;
}

// proto: void QWriteLocker::unlock();
impl<'a> /*trait*/ QWriteLocker_unlock for () {
  fn unlock(self, this: &mut QWriteLocker) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QWriteLocker6unlockEv()};
    unsafe {_ZN12QWriteLocker6unlockEv()};
    return 1;
  }
}

impl /*struct*/ QWriteLocker {
  pub fn FreeQWriteLocker<T: QWriteLocker_FreeQWriteLocker>(&mut self, value: T) -> i32 {
    value.FreeQWriteLocker(self);
    return 1;
  }
}

pub trait QWriteLocker_FreeQWriteLocker {
  fn FreeQWriteLocker(self, this: &mut QWriteLocker) -> i32;
}

// proto: void QWriteLocker::FreeQWriteLocker();
impl<'a> /*trait*/ QWriteLocker_FreeQWriteLocker for () {
  fn FreeQWriteLocker(self, this: &mut QWriteLocker) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QWriteLockerD0Ev()};
    unsafe {_ZN12QWriteLockerD0Ev()};
    return 1;
  }
}

impl /*struct*/ QWriteLocker {
  pub fn relock<T: QWriteLocker_relock>(&mut self, value: T) -> i32 {
    value.relock(self);
    return 1;
  }
}

pub trait QWriteLocker_relock {
  fn relock(self, this: &mut QWriteLocker) -> i32;
}

// proto: void QWriteLocker::relock();
impl<'a> /*trait*/ QWriteLocker_relock for () {
  fn relock(self, this: &mut QWriteLocker) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QWriteLocker6relockEv()};
    unsafe {_ZN12QWriteLocker6relockEv()};
    return 1;
  }
}

// proto: void QWriteLocker::NewQWriteLocker(const QWriteLocker & );
impl<'a> /*trait*/ QWriteLocker_NewQWriteLocker for (&'a  QWriteLocker) {
  fn NewQWriteLocker(self) -> QWriteLocker {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QWriteLockerC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN12QWriteLockerC1ERKS_(qthis, arg0)};
    let rsthis = QWriteLocker{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

