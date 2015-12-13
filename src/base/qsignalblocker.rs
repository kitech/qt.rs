// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qobject::QObject;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  fn _ZN14QSignalBlocker7unblockEv() -> i32;
  fn _ZN14QSignalBlockerC1ER7QObject(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  fn _ZN14QSignalBlockerC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  fn _ZN14QSignalBlockerC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZN14QSignalBlocker7reblockEv() -> i32;
  fn _ZN14QSignalBlockerD0Ev() -> i32;
}

// body block begin
// class sizeof(QSignalBlocker)=16
pub struct QSignalBlocker {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QSignalBlocker {
  pub fn unblock<T: QSignalBlocker_unblock>(&mut self, value: T) -> i32 {
    value.unblock(self);
    return 1;
  }
}

pub trait QSignalBlocker_unblock {
  fn unblock(self, this: &mut QSignalBlocker) -> i32;
}

// proto: void QSignalBlocker::unblock();
impl<'a> /*trait*/ QSignalBlocker_unblock for () {
  fn unblock(self, this: &mut QSignalBlocker) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QSignalBlocker7unblockEv()};
    unsafe {_ZN14QSignalBlocker7unblockEv()};
    return 1;
  }
}

impl /*struct*/ QSignalBlocker {
  pub fn NewQSignalBlocker<T: QSignalBlocker_NewQSignalBlocker>(value: T) -> QSignalBlocker {
    let rsthis = value.NewQSignalBlocker();
    return rsthis;
    // return 1;
  }
}

pub trait QSignalBlocker_NewQSignalBlocker {
  fn NewQSignalBlocker(self) -> QSignalBlocker;
}

// proto: void QSignalBlocker::NewQSignalBlocker(QObject & o);
impl<'a> /*trait*/ QSignalBlocker_NewQSignalBlocker for (&'a mut QObject) {
  fn NewQSignalBlocker(self) -> QSignalBlocker {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QSignalBlockerC1ER7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN14QSignalBlockerC1ER7QObject(qthis, arg0)};
    let rsthis = QSignalBlocker{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QSignalBlocker::NewQSignalBlocker(const QSignalBlocker & );
impl<'a> /*trait*/ QSignalBlocker_NewQSignalBlocker for (&'a  QSignalBlocker) {
  fn NewQSignalBlocker(self) -> QSignalBlocker {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QSignalBlockerC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN14QSignalBlockerC1ERKS_(qthis, arg0)};
    let rsthis = QSignalBlocker{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QSignalBlocker {
  pub fn reblock<T: QSignalBlocker_reblock>(&mut self, value: T) -> i32 {
    value.reblock(self);
    return 1;
  }
}

pub trait QSignalBlocker_reblock {
  fn reblock(self, this: &mut QSignalBlocker) -> i32;
}

// proto: void QSignalBlocker::reblock();
impl<'a> /*trait*/ QSignalBlocker_reblock for () {
  fn reblock(self, this: &mut QSignalBlocker) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QSignalBlocker7reblockEv()};
    unsafe {_ZN14QSignalBlocker7reblockEv()};
    return 1;
  }
}

impl /*struct*/ QSignalBlocker {
  pub fn FreeQSignalBlocker<T: QSignalBlocker_FreeQSignalBlocker>(&mut self, value: T) -> i32 {
    value.FreeQSignalBlocker(self);
    return 1;
  }
}

pub trait QSignalBlocker_FreeQSignalBlocker {
  fn FreeQSignalBlocker(self, this: &mut QSignalBlocker) -> i32;
}

// proto: void QSignalBlocker::FreeQSignalBlocker();
impl<'a> /*trait*/ QSignalBlocker_FreeQSignalBlocker for () {
  fn FreeQSignalBlocker(self, this: &mut QSignalBlocker) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QSignalBlockerD0Ev()};
    unsafe {_ZN14QSignalBlockerD0Ev()};
    return 1;
  }
}

