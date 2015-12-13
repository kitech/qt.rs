// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  fn _ZN11QBasicMutex4lockEv() -> i32;
  fn _ZN11QBasicMutex7tryLockEv() -> i32;
  fn _ZN11QBasicMutex11isRecursiveEv() -> i32;
  fn _ZN11QBasicMutex6unlockEv() -> i32;
}

// body block begin
// class sizeof(QBasicMutex)=1
pub struct QBasicMutex {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QBasicMutex {
  pub fn lock<T: QBasicMutex_lock>(&mut self, value: T) -> i32 {
    value.lock(self);
    return 1;
  }
}

pub trait QBasicMutex_lock {
  fn lock(self, this: &mut QBasicMutex) -> i32;
}

// proto: void QBasicMutex::lock();
impl<'a> /*trait*/ QBasicMutex_lock for () {
  fn lock(self, this: &mut QBasicMutex) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QBasicMutex4lockEv()};
    unsafe {_ZN11QBasicMutex4lockEv()};
    return 1;
  }
}

impl /*struct*/ QBasicMutex {
  pub fn tryLock<T: QBasicMutex_tryLock>(&mut self, value: T) -> i32 {
    value.tryLock(self);
    return 1;
  }
}

pub trait QBasicMutex_tryLock {
  fn tryLock(self, this: &mut QBasicMutex) -> i32;
}

// proto: bool QBasicMutex::tryLock();
impl<'a> /*trait*/ QBasicMutex_tryLock for () {
  fn tryLock(self, this: &mut QBasicMutex) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QBasicMutex7tryLockEv()};
    unsafe {_ZN11QBasicMutex7tryLockEv()};
    return 1;
  }
}

impl /*struct*/ QBasicMutex {
  pub fn isRecursive<T: QBasicMutex_isRecursive>(&mut self, value: T) -> i32 {
    value.isRecursive(self);
    return 1;
  }
}

pub trait QBasicMutex_isRecursive {
  fn isRecursive(self, this: &mut QBasicMutex) -> i32;
}

// proto: bool QBasicMutex::isRecursive();
impl<'a> /*trait*/ QBasicMutex_isRecursive for () {
  fn isRecursive(self, this: &mut QBasicMutex) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QBasicMutex11isRecursiveEv()};
    unsafe {_ZN11QBasicMutex11isRecursiveEv()};
    return 1;
  }
}

impl /*struct*/ QBasicMutex {
  pub fn unlock<T: QBasicMutex_unlock>(&mut self, value: T) -> i32 {
    value.unlock(self);
    return 1;
  }
}

pub trait QBasicMutex_unlock {
  fn unlock(self, this: &mut QBasicMutex) -> i32;
}

// proto: void QBasicMutex::unlock();
impl<'a> /*trait*/ QBasicMutex_unlock for () {
  fn unlock(self, this: &mut QBasicMutex) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QBasicMutex6unlockEv()};
    unsafe {_ZN11QBasicMutex6unlockEv()};
    return 1;
  }
}

