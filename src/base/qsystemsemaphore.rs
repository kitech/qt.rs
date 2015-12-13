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
  fn _ZNK16QSystemSemaphore3keyEv() -> i32;
  fn _ZN16QSystemSemaphore7releaseEi(arg0: c_int) -> i32;
  fn _ZNK16QSystemSemaphore11errorStringEv() -> i32;
  fn _ZN16QSystemSemaphoreC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZN16QSystemSemaphore7acquireEv() -> i32;
  fn _ZN16QSystemSemaphoreD0Ev() -> i32;
}

// body block begin
// class sizeof(QSystemSemaphore)=1
pub struct QSystemSemaphore {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QSystemSemaphore {
  pub fn key<T: QSystemSemaphore_key>(&mut self, value: T) -> i32 {
    value.key(self);
    return 1;
  }
}

pub trait QSystemSemaphore_key {
  fn key(self, this: &mut QSystemSemaphore) -> i32;
}

// proto: QString QSystemSemaphore::key();
impl<'a> /*trait*/ QSystemSemaphore_key for () {
  fn key(self, this: &mut QSystemSemaphore) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QSystemSemaphore3keyEv()};
    unsafe {_ZNK16QSystemSemaphore3keyEv()};
    return 1;
  }
}

impl /*struct*/ QSystemSemaphore {
  pub fn release<T: QSystemSemaphore_release>(&mut self, value: T) -> i32 {
    value.release(self);
    return 1;
  }
}

pub trait QSystemSemaphore_release {
  fn release(self, this: &mut QSystemSemaphore) -> i32;
}

// proto: bool QSystemSemaphore::release(int n);
impl<'a> /*trait*/ QSystemSemaphore_release for (i32) {
  fn release(self, this: &mut QSystemSemaphore) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QSystemSemaphore7releaseEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN16QSystemSemaphore7releaseEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QSystemSemaphore {
  pub fn errorString<T: QSystemSemaphore_errorString>(&mut self, value: T) -> i32 {
    value.errorString(self);
    return 1;
  }
}

pub trait QSystemSemaphore_errorString {
  fn errorString(self, this: &mut QSystemSemaphore) -> i32;
}

// proto: QString QSystemSemaphore::errorString();
impl<'a> /*trait*/ QSystemSemaphore_errorString for () {
  fn errorString(self, this: &mut QSystemSemaphore) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QSystemSemaphore11errorStringEv()};
    unsafe {_ZNK16QSystemSemaphore11errorStringEv()};
    return 1;
  }
}

impl /*struct*/ QSystemSemaphore {
  pub fn NewQSystemSemaphore<T: QSystemSemaphore_NewQSystemSemaphore>(value: T) -> QSystemSemaphore {
    let rsthis = value.NewQSystemSemaphore();
    return rsthis;
    // return 1;
  }
}

pub trait QSystemSemaphore_NewQSystemSemaphore {
  fn NewQSystemSemaphore(self) -> QSystemSemaphore;
}

// proto: void QSystemSemaphore::NewQSystemSemaphore(const QSystemSemaphore & );
impl<'a> /*trait*/ QSystemSemaphore_NewQSystemSemaphore for (&'a  QSystemSemaphore) {
  fn NewQSystemSemaphore(self) -> QSystemSemaphore {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QSystemSemaphoreC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN16QSystemSemaphoreC1ERKS_(qthis, arg0)};
    let rsthis = QSystemSemaphore{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QSystemSemaphore {
  pub fn acquire<T: QSystemSemaphore_acquire>(&mut self, value: T) -> i32 {
    value.acquire(self);
    return 1;
  }
}

pub trait QSystemSemaphore_acquire {
  fn acquire(self, this: &mut QSystemSemaphore) -> i32;
}

// proto: bool QSystemSemaphore::acquire();
impl<'a> /*trait*/ QSystemSemaphore_acquire for () {
  fn acquire(self, this: &mut QSystemSemaphore) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QSystemSemaphore7acquireEv()};
    unsafe {_ZN16QSystemSemaphore7acquireEv()};
    return 1;
  }
}

impl /*struct*/ QSystemSemaphore {
  pub fn FreeQSystemSemaphore<T: QSystemSemaphore_FreeQSystemSemaphore>(&mut self, value: T) -> i32 {
    value.FreeQSystemSemaphore(self);
    return 1;
  }
}

pub trait QSystemSemaphore_FreeQSystemSemaphore {
  fn FreeQSystemSemaphore(self, this: &mut QSystemSemaphore) -> i32;
}

// proto: void QSystemSemaphore::FreeQSystemSemaphore();
impl<'a> /*trait*/ QSystemSemaphore_FreeQSystemSemaphore for () {
  fn FreeQSystemSemaphore(self, this: &mut QSystemSemaphore) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QSystemSemaphoreD0Ev()};
    unsafe {_ZN16QSystemSemaphoreD0Ev()};
    return 1;
  }
}

