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
  fn _ZN14QReadWriteLockD0Ev() -> i32;
  fn _ZN14QReadWriteLockC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZN14QReadWriteLock14tryLockForReadEv() -> i32;
  fn _ZN14QReadWriteLock12lockForWriteEv() -> i32;
  fn _ZN14QReadWriteLock15tryLockForWriteEv() -> i32;
  fn _ZN14QReadWriteLock6unlockEv() -> i32;
  fn _ZN14QReadWriteLock14tryLockForReadEi(arg0: c_int) -> i32;
  fn _ZN14QReadWriteLock11lockForReadEv() -> i32;
  fn _ZN14QReadWriteLock15tryLockForWriteEi(arg0: c_int) -> i32;
}

// body block begin
// class sizeof(QReadWriteLock)=8
pub struct QReadWriteLock {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QReadWriteLock {
  pub fn FreeQReadWriteLock<T: QReadWriteLock_FreeQReadWriteLock>(&mut self, value: T) -> i32 {
    value.FreeQReadWriteLock(self);
    return 1;
  }
}

pub trait QReadWriteLock_FreeQReadWriteLock {
  fn FreeQReadWriteLock(self, this: &mut QReadWriteLock) -> i32;
}

// proto: void QReadWriteLock::FreeQReadWriteLock();
impl<'a> /*trait*/ QReadWriteLock_FreeQReadWriteLock for () {
  fn FreeQReadWriteLock(self, this: &mut QReadWriteLock) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QReadWriteLockD0Ev()};
    unsafe {_ZN14QReadWriteLockD0Ev()};
    return 1;
  }
}

impl /*struct*/ QReadWriteLock {
  pub fn NewQReadWriteLock<T: QReadWriteLock_NewQReadWriteLock>(value: T) -> QReadWriteLock {
    let rsthis = value.NewQReadWriteLock();
    return rsthis;
    // return 1;
  }
}

pub trait QReadWriteLock_NewQReadWriteLock {
  fn NewQReadWriteLock(self) -> QReadWriteLock;
}

// proto: void QReadWriteLock::NewQReadWriteLock(const QReadWriteLock & );
impl<'a> /*trait*/ QReadWriteLock_NewQReadWriteLock for (&'a  QReadWriteLock) {
  fn NewQReadWriteLock(self) -> QReadWriteLock {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QReadWriteLockC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN14QReadWriteLockC1ERKS_(qthis, arg0)};
    let rsthis = QReadWriteLock{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QReadWriteLock {
  pub fn tryLockForRead<T: QReadWriteLock_tryLockForRead>(&mut self, value: T) -> i32 {
    value.tryLockForRead(self);
    return 1;
  }
}

pub trait QReadWriteLock_tryLockForRead {
  fn tryLockForRead(self, this: &mut QReadWriteLock) -> i32;
}

// proto: bool QReadWriteLock::tryLockForRead();
impl<'a> /*trait*/ QReadWriteLock_tryLockForRead for () {
  fn tryLockForRead(self, this: &mut QReadWriteLock) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QReadWriteLock14tryLockForReadEv()};
    unsafe {_ZN14QReadWriteLock14tryLockForReadEv()};
    return 1;
  }
}

impl /*struct*/ QReadWriteLock {
  pub fn lockForWrite<T: QReadWriteLock_lockForWrite>(&mut self, value: T) -> i32 {
    value.lockForWrite(self);
    return 1;
  }
}

pub trait QReadWriteLock_lockForWrite {
  fn lockForWrite(self, this: &mut QReadWriteLock) -> i32;
}

// proto: void QReadWriteLock::lockForWrite();
impl<'a> /*trait*/ QReadWriteLock_lockForWrite for () {
  fn lockForWrite(self, this: &mut QReadWriteLock) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QReadWriteLock12lockForWriteEv()};
    unsafe {_ZN14QReadWriteLock12lockForWriteEv()};
    return 1;
  }
}

impl /*struct*/ QReadWriteLock {
  pub fn tryLockForWrite<T: QReadWriteLock_tryLockForWrite>(&mut self, value: T) -> i32 {
    value.tryLockForWrite(self);
    return 1;
  }
}

pub trait QReadWriteLock_tryLockForWrite {
  fn tryLockForWrite(self, this: &mut QReadWriteLock) -> i32;
}

// proto: bool QReadWriteLock::tryLockForWrite();
impl<'a> /*trait*/ QReadWriteLock_tryLockForWrite for () {
  fn tryLockForWrite(self, this: &mut QReadWriteLock) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QReadWriteLock15tryLockForWriteEv()};
    unsafe {_ZN14QReadWriteLock15tryLockForWriteEv()};
    return 1;
  }
}

impl /*struct*/ QReadWriteLock {
  pub fn unlock<T: QReadWriteLock_unlock>(&mut self, value: T) -> i32 {
    value.unlock(self);
    return 1;
  }
}

pub trait QReadWriteLock_unlock {
  fn unlock(self, this: &mut QReadWriteLock) -> i32;
}

// proto: void QReadWriteLock::unlock();
impl<'a> /*trait*/ QReadWriteLock_unlock for () {
  fn unlock(self, this: &mut QReadWriteLock) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QReadWriteLock6unlockEv()};
    unsafe {_ZN14QReadWriteLock6unlockEv()};
    return 1;
  }
}

// proto: bool QReadWriteLock::tryLockForRead(int timeout);
impl<'a> /*trait*/ QReadWriteLock_tryLockForRead for (i32) {
  fn tryLockForRead(self, this: &mut QReadWriteLock) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QReadWriteLock14tryLockForReadEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN14QReadWriteLock14tryLockForReadEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QReadWriteLock {
  pub fn lockForRead<T: QReadWriteLock_lockForRead>(&mut self, value: T) -> i32 {
    value.lockForRead(self);
    return 1;
  }
}

pub trait QReadWriteLock_lockForRead {
  fn lockForRead(self, this: &mut QReadWriteLock) -> i32;
}

// proto: void QReadWriteLock::lockForRead();
impl<'a> /*trait*/ QReadWriteLock_lockForRead for () {
  fn lockForRead(self, this: &mut QReadWriteLock) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QReadWriteLock11lockForReadEv()};
    unsafe {_ZN14QReadWriteLock11lockForReadEv()};
    return 1;
  }
}

// proto: bool QReadWriteLock::tryLockForWrite(int timeout);
impl<'a> /*trait*/ QReadWriteLock_tryLockForWrite for (i32) {
  fn tryLockForWrite(self, this: &mut QReadWriteLock) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QReadWriteLock15tryLockForWriteEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN14QReadWriteLock15tryLockForWriteEi(arg0)};
    return 1;
  }
}

