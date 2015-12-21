// auto generated, do not modify.
// created: Mon Dec 21 22:54:38 2015
// src-file: /QtCore/qwaitcondition.h
// dst-file: /src/core/qwaitcondition.rs
//

// header block begin =>
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;


// <= header block end

// main block begin =>
// <= main block end

// use block begin =>
use super::qreadwritelock::QReadWriteLock; // 773
use super::qmutex::QMutex; // 773
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  bool QWaitCondition::wait(QReadWriteLock * lockedReadWriteLock, unsigned long time);
  fn _ZN14QWaitCondition4waitEP14QReadWriteLockm(qthis: *mut c_void, arg0: *mut c_void, arg1: c_ulong) -> c_char;
  // proto:  bool QWaitCondition::wait(QMutex * lockedMutex, unsigned long time);
  fn _ZN14QWaitCondition4waitEP6QMutexm(qthis: *mut c_void, arg0: *mut c_void, arg1: c_ulong) -> c_char;
  // proto:  void QWaitCondition::wakeAll();
  fn _ZN14QWaitCondition7wakeAllEv(qthis: *mut c_void);
  // proto:  void QWaitCondition::wakeOne();
  fn _ZN14QWaitCondition7wakeOneEv(qthis: *mut c_void);
  // proto:  void QWaitCondition::QWaitCondition(const QWaitCondition & );
  fn _ZN14QWaitConditionC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QWaitCondition::~QWaitCondition();
  fn _ZN14QWaitConditionD0Ev(qthis: *mut c_void);
  // proto:  void QWaitCondition::QWaitCondition();
  fn _ZN14QWaitConditionC1Ev(qthis: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QWaitCondition)=8
pub struct QWaitCondition {
  pub qclsinst: *mut c_void,
}

  // proto:  bool QWaitCondition::wait(QReadWriteLock * lockedReadWriteLock, unsigned long time);
impl /*struct*/ QWaitCondition {
  pub fn wait<RetType, T: QWaitCondition_wait<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.wait(self);
    // return 1;
  }
}

pub trait QWaitCondition_wait<RetType> {
  fn wait(self , rsthis: &mut QWaitCondition) -> RetType;
}

  // proto:  bool QWaitCondition::wait(QReadWriteLock * lockedReadWriteLock, unsigned long time);
impl<'a> /*trait*/ QWaitCondition_wait<i8> for (QReadWriteLock, u64) {
  fn wait(self , rsthis: &mut QWaitCondition) -> i8 {
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
impl<'a> /*trait*/ QWaitCondition_wait<i8> for (QMutex, u64) {
  fn wait(self , rsthis: &mut QWaitCondition) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QWaitCondition4waitEP6QMutexm()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_ulong;
    let mut ret = unsafe {_ZN14QWaitCondition4waitEP6QMutexm(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QWaitCondition::wakeAll();
impl /*struct*/ QWaitCondition {
  pub fn wakeAll<RetType, T: QWaitCondition_wakeAll<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.wakeAll(self);
    // return 1;
  }
}

pub trait QWaitCondition_wakeAll<RetType> {
  fn wakeAll(self , rsthis: &mut QWaitCondition) -> RetType;
}

  // proto:  void QWaitCondition::wakeAll();
impl<'a> /*trait*/ QWaitCondition_wakeAll<()> for () {
  fn wakeAll(self , rsthis: &mut QWaitCondition) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QWaitCondition7wakeAllEv()};
     unsafe {_ZN14QWaitCondition7wakeAllEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QWaitCondition::wakeOne();
impl /*struct*/ QWaitCondition {
  pub fn wakeOne<RetType, T: QWaitCondition_wakeOne<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.wakeOne(self);
    // return 1;
  }
}

pub trait QWaitCondition_wakeOne<RetType> {
  fn wakeOne(self , rsthis: &mut QWaitCondition) -> RetType;
}

  // proto:  void QWaitCondition::wakeOne();
impl<'a> /*trait*/ QWaitCondition_wakeOne<()> for () {
  fn wakeOne(self , rsthis: &mut QWaitCondition) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QWaitCondition7wakeOneEv()};
     unsafe {_ZN14QWaitCondition7wakeOneEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QWaitCondition::QWaitCondition(const QWaitCondition & );
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

  // proto:  void QWaitCondition::QWaitCondition(const QWaitCondition & );
impl<'a> /*trait*/ QWaitCondition_NewQWaitCondition for (QWaitCondition) {
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

  // proto:  void QWaitCondition::~QWaitCondition();
impl /*struct*/ QWaitCondition {
  pub fn FreeQWaitCondition<RetType, T: QWaitCondition_FreeQWaitCondition<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQWaitCondition(self);
    // return 1;
  }
}

pub trait QWaitCondition_FreeQWaitCondition<RetType> {
  fn FreeQWaitCondition(self , rsthis: &mut QWaitCondition) -> RetType;
}

  // proto:  void QWaitCondition::~QWaitCondition();
impl<'a> /*trait*/ QWaitCondition_FreeQWaitCondition<()> for () {
  fn FreeQWaitCondition(self , rsthis: &mut QWaitCondition) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QWaitConditionD0Ev()};
     unsafe {_ZN14QWaitConditionD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QWaitCondition::QWaitCondition();
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

// <= body block end

