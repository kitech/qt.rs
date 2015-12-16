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
  // proto:  void QBasicMutex::lock();
  fn _ZN11QBasicMutex4lockEv(qthis: *mut c_void) ;
  // proto:  bool QBasicMutex::tryLock();
  fn _ZN11QBasicMutex7tryLockEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QBasicMutex::isRecursive();
  fn _ZN11QBasicMutex11isRecursiveEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QBasicMutex::unlock();
  fn _ZN11QBasicMutex6unlockEv(qthis: *mut c_void) ;
}

// body block begin
// class sizeof(QBasicMutex)=1
pub struct QBasicMutex {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QBasicMutex {
  pub fn lock<T: QBasicMutex_lock>(&mut self, value: T)  {
     value.lock(self);
    // return 1;
  }
}

pub trait QBasicMutex_lock {
  fn lock(self, rsthis: &mut QBasicMutex) ;
}

// proto:  void QBasicMutex::lock();
impl<'a> /*trait*/ QBasicMutex_lock for () {
  fn lock(self, rsthis: &mut QBasicMutex)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QBasicMutex4lockEv()};
     unsafe {_ZN11QBasicMutex4lockEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QBasicMutex {
  pub fn tryLock<T: QBasicMutex_tryLock>(&mut self, value: T) -> i8 {
    return value.tryLock(self);
    // return 1;
  }
}

pub trait QBasicMutex_tryLock {
  fn tryLock(self, rsthis: &mut QBasicMutex) -> i8;
}

// proto:  bool QBasicMutex::tryLock();
impl<'a> /*trait*/ QBasicMutex_tryLock for () {
  fn tryLock(self, rsthis: &mut QBasicMutex) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QBasicMutex7tryLockEv()};
    let mut ret = unsafe {_ZN11QBasicMutex7tryLockEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QBasicMutex {
  pub fn isRecursive<T: QBasicMutex_isRecursive>(&mut self, value: T) -> i8 {
    return value.isRecursive(self);
    // return 1;
  }
}

pub trait QBasicMutex_isRecursive {
  fn isRecursive(self, rsthis: &mut QBasicMutex) -> i8;
}

// proto:  bool QBasicMutex::isRecursive();
impl<'a> /*trait*/ QBasicMutex_isRecursive for () {
  fn isRecursive(self, rsthis: &mut QBasicMutex) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QBasicMutex11isRecursiveEv()};
    let mut ret = unsafe {_ZN11QBasicMutex11isRecursiveEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QBasicMutex {
  pub fn unlock<T: QBasicMutex_unlock>(&mut self, value: T)  {
     value.unlock(self);
    // return 1;
  }
}

pub trait QBasicMutex_unlock {
  fn unlock(self, rsthis: &mut QBasicMutex) ;
}

// proto:  void QBasicMutex::unlock();
impl<'a> /*trait*/ QBasicMutex_unlock for () {
  fn unlock(self, rsthis: &mut QBasicMutex)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QBasicMutex6unlockEv()};
     unsafe {_ZN11QBasicMutex6unlockEv(rsthis.qclsinst)};
    // return 1;
  }
}

