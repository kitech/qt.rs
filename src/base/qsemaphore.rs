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
  fn _ZN10QSemaphore7acquireEi(arg0: c_int) -> i32;
  fn _ZN10QSemaphore7releaseEi(arg0: c_int) -> i32;
  fn _ZNK10QSemaphore9availableEv() -> i32;
  fn _ZN10QSemaphore10tryAcquireEii(arg0: c_int, arg1: c_int) -> i32;
  fn _ZN10QSemaphoreC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZN10QSemaphore10tryAcquireEi(arg0: c_int) -> i32;
  fn _ZN10QSemaphoreC1Ei(qthis: *mut c_void, arg0: c_int) -> i32;
  fn _ZN10QSemaphoreD0Ev() -> i32;
}

// body block begin
// class sizeof(QSemaphore)=8
pub struct QSemaphore {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QSemaphore {
  pub fn acquire<T: QSemaphore_acquire>(&mut self, value: T) -> i32 {
    value.acquire(self);
    return 1;
  }
}

pub trait QSemaphore_acquire {
  fn acquire(self, this: &mut QSemaphore) -> i32;
}

// proto: void QSemaphore::acquire(int n);
impl<'a> /*trait*/ QSemaphore_acquire for (i32) {
  fn acquire(self, this: &mut QSemaphore) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QSemaphore7acquireEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN10QSemaphore7acquireEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QSemaphore {
  pub fn release<T: QSemaphore_release>(&mut self, value: T) -> i32 {
    value.release(self);
    return 1;
  }
}

pub trait QSemaphore_release {
  fn release(self, this: &mut QSemaphore) -> i32;
}

// proto: void QSemaphore::release(int n);
impl<'a> /*trait*/ QSemaphore_release for (i32) {
  fn release(self, this: &mut QSemaphore) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QSemaphore7releaseEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN10QSemaphore7releaseEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QSemaphore {
  pub fn available<T: QSemaphore_available>(&mut self, value: T) -> i32 {
    value.available(self);
    return 1;
  }
}

pub trait QSemaphore_available {
  fn available(self, this: &mut QSemaphore) -> i32;
}

// proto: int QSemaphore::available();
impl<'a> /*trait*/ QSemaphore_available for () {
  fn available(self, this: &mut QSemaphore) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QSemaphore9availableEv()};
    unsafe {_ZNK10QSemaphore9availableEv()};
    return 1;
  }
}

impl /*struct*/ QSemaphore {
  pub fn tryAcquire<T: QSemaphore_tryAcquire>(&mut self, value: T) -> i32 {
    value.tryAcquire(self);
    return 1;
  }
}

pub trait QSemaphore_tryAcquire {
  fn tryAcquire(self, this: &mut QSemaphore) -> i32;
}

// proto: bool QSemaphore::tryAcquire(int n, int timeout);
impl<'a> /*trait*/ QSemaphore_tryAcquire for (i32, i32) {
  fn tryAcquire(self, this: &mut QSemaphore) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QSemaphore10tryAcquireEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZN10QSemaphore10tryAcquireEii(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QSemaphore {
  pub fn NewQSemaphore<T: QSemaphore_NewQSemaphore>(value: T) -> QSemaphore {
    let rsthis = value.NewQSemaphore();
    return rsthis;
    // return 1;
  }
}

pub trait QSemaphore_NewQSemaphore {
  fn NewQSemaphore(self) -> QSemaphore;
}

// proto: void QSemaphore::NewQSemaphore(const QSemaphore & );
impl<'a> /*trait*/ QSemaphore_NewQSemaphore for (&'a  QSemaphore) {
  fn NewQSemaphore(self) -> QSemaphore {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QSemaphoreC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN10QSemaphoreC1ERKS_(qthis, arg0)};
    let rsthis = QSemaphore{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: bool QSemaphore::tryAcquire(int n);
impl<'a> /*trait*/ QSemaphore_tryAcquire for (i32) {
  fn tryAcquire(self, this: &mut QSemaphore) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QSemaphore10tryAcquireEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN10QSemaphore10tryAcquireEi(arg0)};
    return 1;
  }
}

// proto: void QSemaphore::NewQSemaphore(int n);
impl<'a> /*trait*/ QSemaphore_NewQSemaphore for (i32) {
  fn NewQSemaphore(self) -> QSemaphore {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QSemaphoreC1Ei()};
    let arg0 = self  as c_int;
    unsafe {_ZN10QSemaphoreC1Ei(qthis, arg0)};
    let rsthis = QSemaphore{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QSemaphore {
  pub fn FreeQSemaphore<T: QSemaphore_FreeQSemaphore>(&mut self, value: T) -> i32 {
    value.FreeQSemaphore(self);
    return 1;
  }
}

pub trait QSemaphore_FreeQSemaphore {
  fn FreeQSemaphore(self, this: &mut QSemaphore) -> i32;
}

// proto: void QSemaphore::FreeQSemaphore();
impl<'a> /*trait*/ QSemaphore_FreeQSemaphore for () {
  fn FreeQSemaphore(self, this: &mut QSemaphore) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QSemaphoreD0Ev()};
    unsafe {_ZN10QSemaphoreD0Ev()};
    return 1;
  }
}

