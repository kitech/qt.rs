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
  // proto:  bool QWaitCondition::wait(QReadWriteLock * lockedReadWriteLock, unsigned long time);
  fn _ZN14QWaitCondition4waitEP14QReadWriteLockm(qthis: *mut c_void, arg0: *mut c_void, arg1: c_ulong) -> int8_t;
  // proto:  bool QWaitCondition::wait(QMutex * lockedMutex, unsigned long time);
  fn _ZN14QWaitCondition4waitEP6QMutexm(qthis: *mut c_void, arg0: *mut c_void, arg1: c_ulong) -> int8_t;
  // proto:  void QWaitCondition::wakeAll();
  fn _ZN14QWaitCondition7wakeAllEv(qthis: *mut c_void) ;
  // proto:  void QWaitCondition::wakeOne();
  fn _ZN14QWaitCondition7wakeOneEv(qthis: *mut c_void) ;
  // proto:  void QWaitCondition::NewQWaitCondition(const QWaitCondition & );
  fn _ZN14QWaitConditionC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QWaitCondition::FreeQWaitCondition();
  fn _ZN14QWaitConditionD0Ev(qthis: *mut c_void) ;
  // proto:  void QWaitCondition::NewQWaitCondition();
  fn _ZN14QWaitConditionC1Ev(qthis: *mut c_void) ;
}

// body block begin
// class sizeof(QWaitCondition)=8
pub struct QWaitCondition {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QWaitCondition {
  pub fn wait<T: QWaitCondition_wait>(&mut self, value: T) -> i8 {
    return value.wait(self);
    // return 1;
  }
}

pub trait QWaitCondition_wait {
  fn wait(self, rsthis: &mut QWaitCondition) -> i8;
}

// proto:  bool QWaitCondition::wait(QReadWriteLock * lockedReadWriteLock, unsigned long time);
impl<'a> /*trait*/ QWaitCondition_wait for (&'a mut QReadWriteLock, u32) {
  fn wait(self, rsthis: &mut QWaitCondition) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QWaitCondition4waitEP14QReadWriteLockm()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_ulong;
    let mut ret = unsafe {_ZN14QWaitCondition4waitEP14QReadWriteLockm(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

// proto:  bool QWaitCondition::wait(QMutex * lockedMutex, unsigned long time);
impl<'a> /*trait*/ QWaitCondition_wait for (&'a mut QMutex, u32) {
  fn wait(self, rsthis: &mut QWaitCondition) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QWaitCondition4waitEP6QMutexm()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_ulong;
    let mut ret = unsafe {_ZN14QWaitCondition4waitEP6QMutexm(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QWaitCondition {
  pub fn wakeAll<T: QWaitCondition_wakeAll>(&mut self, value: T)  {
     value.wakeAll(self);
    // return 1;
  }
}

pub trait QWaitCondition_wakeAll {
  fn wakeAll(self, rsthis: &mut QWaitCondition) ;
}

// proto:  void QWaitCondition::wakeAll();
impl<'a> /*trait*/ QWaitCondition_wakeAll for () {
  fn wakeAll(self, rsthis: &mut QWaitCondition)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QWaitCondition7wakeAllEv()};
     unsafe {_ZN14QWaitCondition7wakeAllEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QWaitCondition {
  pub fn wakeOne<T: QWaitCondition_wakeOne>(&mut self, value: T)  {
     value.wakeOne(self);
    // return 1;
  }
}

pub trait QWaitCondition_wakeOne {
  fn wakeOne(self, rsthis: &mut QWaitCondition) ;
}

// proto:  void QWaitCondition::wakeOne();
impl<'a> /*trait*/ QWaitCondition_wakeOne for () {
  fn wakeOne(self, rsthis: &mut QWaitCondition)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QWaitCondition7wakeOneEv()};
     unsafe {_ZN14QWaitCondition7wakeOneEv(rsthis.qclsinst)};
    // return 1;
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
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN14QWaitConditionC1ERKS_(qthis, arg0)};
    let rsthis = QWaitCondition{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QWaitCondition {
  pub fn FreeQWaitCondition<T: QWaitCondition_FreeQWaitCondition>(&mut self, value: T)  {
     value.FreeQWaitCondition(self);
    // return 1;
  }
}

pub trait QWaitCondition_FreeQWaitCondition {
  fn FreeQWaitCondition(self, rsthis: &mut QWaitCondition) ;
}

// proto:  void QWaitCondition::FreeQWaitCondition();
impl<'a> /*trait*/ QWaitCondition_FreeQWaitCondition for () {
  fn FreeQWaitCondition(self, rsthis: &mut QWaitCondition)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QWaitConditionD0Ev()};
     unsafe {_ZN14QWaitConditionD0Ev(rsthis.qclsinst)};
    // return 1;
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

