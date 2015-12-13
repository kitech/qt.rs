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
  fn _ZN9QRunnableD0Ev() -> i32;
  fn _ZN9QRunnable13setAutoDeleteEb(arg0: int8_t) -> i32;
  fn _ZN9QRunnableC1Ev(qthis: *mut c_void) -> i32;
  fn _ZN9QRunnable3runEv() -> i32;
  fn _ZNK9QRunnable10autoDeleteEv() -> i32;
}

// body block begin
// class sizeof(QRunnable)=16
pub struct QRunnable {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QRunnable {
  pub fn FreeQRunnable<T: QRunnable_FreeQRunnable>(&mut self, value: T) -> i32 {
    value.FreeQRunnable(self);
    return 1;
  }
}

pub trait QRunnable_FreeQRunnable {
  fn FreeQRunnable(self, this: &mut QRunnable) -> i32;
}

// proto: void QRunnable::FreeQRunnable();
impl<'a> /*trait*/ QRunnable_FreeQRunnable for () {
  fn FreeQRunnable(self, this: &mut QRunnable) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QRunnableD0Ev()};
    unsafe {_ZN9QRunnableD0Ev()};
    return 1;
  }
}

impl /*struct*/ QRunnable {
  pub fn setAutoDelete<T: QRunnable_setAutoDelete>(&mut self, value: T) -> i32 {
    value.setAutoDelete(self);
    return 1;
  }
}

pub trait QRunnable_setAutoDelete {
  fn setAutoDelete(self, this: &mut QRunnable) -> i32;
}

// proto: void QRunnable::setAutoDelete(bool _autoDelete);
impl<'a> /*trait*/ QRunnable_setAutoDelete for (i8) {
  fn setAutoDelete(self, this: &mut QRunnable) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QRunnable13setAutoDeleteEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN9QRunnable13setAutoDeleteEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QRunnable {
  pub fn NewQRunnable<T: QRunnable_NewQRunnable>(value: T) -> QRunnable {
    let rsthis = value.NewQRunnable();
    return rsthis;
    // return 1;
  }
}

pub trait QRunnable_NewQRunnable {
  fn NewQRunnable(self) -> QRunnable;
}

// proto: void QRunnable::NewQRunnable();
impl<'a> /*trait*/ QRunnable_NewQRunnable for () {
  fn NewQRunnable(self) -> QRunnable {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QRunnableC1Ev()};
    unsafe {_ZN9QRunnableC1Ev(qthis)};
    let rsthis = QRunnable{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QRunnable {
  pub fn run<T: QRunnable_run>(&mut self, value: T) -> i32 {
    value.run(self);
    return 1;
  }
}

pub trait QRunnable_run {
  fn run(self, this: &mut QRunnable) -> i32;
}

// proto: void QRunnable::run();
impl<'a> /*trait*/ QRunnable_run for () {
  fn run(self, this: &mut QRunnable) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QRunnable3runEv()};
    unsafe {_ZN9QRunnable3runEv()};
    return 1;
  }
}

impl /*struct*/ QRunnable {
  pub fn autoDelete<T: QRunnable_autoDelete>(&mut self, value: T) -> i32 {
    value.autoDelete(self);
    return 1;
  }
}

pub trait QRunnable_autoDelete {
  fn autoDelete(self, this: &mut QRunnable) -> i32;
}

// proto: bool QRunnable::autoDelete();
impl<'a> /*trait*/ QRunnable_autoDelete for () {
  fn autoDelete(self, this: &mut QRunnable) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QRunnable10autoDeleteEv()};
    unsafe {_ZNK9QRunnable10autoDeleteEv()};
    return 1;
  }
}

