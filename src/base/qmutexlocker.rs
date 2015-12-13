// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qbasicmutex::QBasicMutex;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  fn _ZN12QMutexLockerC1EP11QBasicMutex(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  fn _ZNK12QMutexLocker5mutexEv() -> i32;
  fn _ZN12QMutexLockerC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZN12QMutexLocker6relockEv() -> i32;
  fn _ZN12QMutexLocker6unlockEv() -> i32;
  fn _ZN12QMutexLockerD0Ev() -> i32;
}

// body block begin
// class sizeof(QMutexLocker)=4
pub struct QMutexLocker {
  pub qclsinst: *mut c_void,
}

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

// proto: void QMutexLocker::NewQMutexLocker(QBasicMutex * m);
impl<'a> /*trait*/ QMutexLocker_NewQMutexLocker for (&'a mut QBasicMutex) {
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

impl /*struct*/ QMutexLocker {
  pub fn mutex<T: QMutexLocker_mutex>(&mut self, value: T) -> i32 {
    value.mutex(self);
    return 1;
  }
}

pub trait QMutexLocker_mutex {
  fn mutex(self, this: &mut QMutexLocker) -> i32;
}

// proto: QMutex * QMutexLocker::mutex();
impl<'a> /*trait*/ QMutexLocker_mutex for () {
  fn mutex(self, this: &mut QMutexLocker) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QMutexLocker5mutexEv()};
    unsafe {_ZNK12QMutexLocker5mutexEv()};
    return 1;
  }
}

// proto: void QMutexLocker::NewQMutexLocker(const QMutexLocker & );
impl<'a> /*trait*/ QMutexLocker_NewQMutexLocker for (&'a  QMutexLocker) {
  fn NewQMutexLocker(self) -> QMutexLocker {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QMutexLockerC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN12QMutexLockerC1ERKS_(qthis, arg0)};
    let rsthis = QMutexLocker{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QMutexLocker {
  pub fn relock<T: QMutexLocker_relock>(&mut self, value: T) -> i32 {
    value.relock(self);
    return 1;
  }
}

pub trait QMutexLocker_relock {
  fn relock(self, this: &mut QMutexLocker) -> i32;
}

// proto: void QMutexLocker::relock();
impl<'a> /*trait*/ QMutexLocker_relock for () {
  fn relock(self, this: &mut QMutexLocker) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QMutexLocker6relockEv()};
    unsafe {_ZN12QMutexLocker6relockEv()};
    return 1;
  }
}

impl /*struct*/ QMutexLocker {
  pub fn unlock<T: QMutexLocker_unlock>(&mut self, value: T) -> i32 {
    value.unlock(self);
    return 1;
  }
}

pub trait QMutexLocker_unlock {
  fn unlock(self, this: &mut QMutexLocker) -> i32;
}

// proto: void QMutexLocker::unlock();
impl<'a> /*trait*/ QMutexLocker_unlock for () {
  fn unlock(self, this: &mut QMutexLocker) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QMutexLocker6unlockEv()};
    unsafe {_ZN12QMutexLocker6unlockEv()};
    return 1;
  }
}

impl /*struct*/ QMutexLocker {
  pub fn FreeQMutexLocker<T: QMutexLocker_FreeQMutexLocker>(&mut self, value: T) -> i32 {
    value.FreeQMutexLocker(self);
    return 1;
  }
}

pub trait QMutexLocker_FreeQMutexLocker {
  fn FreeQMutexLocker(self, this: &mut QMutexLocker) -> i32;
}

// proto: void QMutexLocker::FreeQMutexLocker();
impl<'a> /*trait*/ QMutexLocker_FreeQMutexLocker for () {
  fn FreeQMutexLocker(self, this: &mut QMutexLocker) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QMutexLockerD0Ev()};
    unsafe {_ZN12QMutexLockerD0Ev()};
    return 1;
  }
}

