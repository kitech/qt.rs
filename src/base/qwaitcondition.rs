// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qreadwritelock::QReadWriteLock;
use super::qmutex::QMutex;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  fn _ZN14QWaitCondition4waitEP14QReadWriteLockm(arg0: *mut c_void, arg1: c_ulong) -> i32;
  fn _ZN14QWaitCondition4waitEP6QMutexm(arg0: *mut c_void, arg1: c_ulong) -> i32;
  fn _ZN14QWaitCondition7wakeAllEv() -> i32;
  fn _ZN14QWaitCondition7wakeOneEv() -> i32;
  fn _ZN14QWaitConditionC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZN14QWaitConditionD0Ev() -> i32;
  fn _ZN14QWaitConditionC1Ev(qthis: *mut c_void) -> i32;
}

// body block begin
// class sizeof(QWaitCondition)=8
pub struct QWaitCondition {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QWaitCondition {
  pub fn wait<T: QWaitCondition_wait>(&mut self, value: T) -> i32 {
    value.wait(self);
    return 1;
  }
}

pub trait QWaitCondition_wait {
  fn wait(self, this: &mut QWaitCondition) -> i32;
}

// proto: bool QWaitCondition::wait(QReadWriteLock * lockedReadWriteLock, unsigned long time);
impl<'a> /*trait*/ QWaitCondition_wait for (&'a mut QReadWriteLock, u32) {
  fn wait(self, this: &mut QWaitCondition) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QWaitCondition4waitEP14QReadWriteLockm()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_ulong;
    unsafe {_ZN14QWaitCondition4waitEP14QReadWriteLockm(arg0, arg1)};
    return 1;
  }
}

// proto: bool QWaitCondition::wait(QMutex * lockedMutex, unsigned long time);
impl<'a> /*trait*/ QWaitCondition_wait for (&'a mut QMutex, u32) {
  fn wait(self, this: &mut QWaitCondition) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QWaitCondition4waitEP6QMutexm()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_ulong;
    unsafe {_ZN14QWaitCondition4waitEP6QMutexm(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QWaitCondition {
  pub fn wakeAll<T: QWaitCondition_wakeAll>(&mut self, value: T) -> i32 {
    value.wakeAll(self);
    return 1;
  }
}

pub trait QWaitCondition_wakeAll {
  fn wakeAll(self, this: &mut QWaitCondition) -> i32;
}

// proto: void QWaitCondition::wakeAll();
impl<'a> /*trait*/ QWaitCondition_wakeAll for () {
  fn wakeAll(self, this: &mut QWaitCondition) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QWaitCondition7wakeAllEv()};
    unsafe {_ZN14QWaitCondition7wakeAllEv()};
    return 1;
  }
}

impl /*struct*/ QWaitCondition {
  pub fn wakeOne<T: QWaitCondition_wakeOne>(&mut self, value: T) -> i32 {
    value.wakeOne(self);
    return 1;
  }
}

pub trait QWaitCondition_wakeOne {
  fn wakeOne(self, this: &mut QWaitCondition) -> i32;
}

// proto: void QWaitCondition::wakeOne();
impl<'a> /*trait*/ QWaitCondition_wakeOne for () {
  fn wakeOne(self, this: &mut QWaitCondition) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QWaitCondition7wakeOneEv()};
    unsafe {_ZN14QWaitCondition7wakeOneEv()};
    return 1;
  }
}

impl /*struct*/ QWaitCondition {
  pub fn NewQWaitCondition<T: QWaitCondition_NewQWaitCondition>(value: T) -> QWaitCondition {
    let rsthis = value.NewQWaitCondition();
    return rsthis;
    // return 1;
  }
}

pub trait QWaitCondition_NewQWaitCondition {
  fn NewQWaitCondition(self) -> QWaitCondition;
}

// proto: void QWaitCondition::NewQWaitCondition(const QWaitCondition & );
impl<'a> /*trait*/ QWaitCondition_NewQWaitCondition for (&'a  QWaitCondition) {
  fn NewQWaitCondition(self) -> QWaitCondition {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QWaitConditionC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN14QWaitConditionC1ERKS_(qthis, arg0)};
    let rsthis = QWaitCondition{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QWaitCondition {
  pub fn FreeQWaitCondition<T: QWaitCondition_FreeQWaitCondition>(&mut self, value: T) -> i32 {
    value.FreeQWaitCondition(self);
    return 1;
  }
}

pub trait QWaitCondition_FreeQWaitCondition {
  fn FreeQWaitCondition(self, this: &mut QWaitCondition) -> i32;
}

// proto: void QWaitCondition::FreeQWaitCondition();
impl<'a> /*trait*/ QWaitCondition_FreeQWaitCondition for () {
  fn FreeQWaitCondition(self, this: &mut QWaitCondition) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QWaitConditionD0Ev()};
    unsafe {_ZN14QWaitConditionD0Ev()};
    return 1;
  }
}

// proto: void QWaitCondition::NewQWaitCondition();
impl<'a> /*trait*/ QWaitCondition_NewQWaitCondition for () {
  fn NewQWaitCondition(self) -> QWaitCondition {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QWaitConditionC1Ev()};
    unsafe {_ZN14QWaitConditionC1Ev(qthis)};
    let rsthis = QWaitCondition{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

