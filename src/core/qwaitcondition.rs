// auto generated, do not modify.
// created: Fri Jan  1 15:54:32 2016
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
use std::ops::Deref;
use super::qreadwritelock::QReadWriteLock; // 773
use super::qmutex::QMutex; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QWaitCondition_Class_Size() -> c_int;
  // proto:  bool QWaitCondition::wait(QReadWriteLock * lockedReadWriteLock, unsigned long time);
  fn _ZN14QWaitCondition4waitEP14QReadWriteLockm(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_ulong) -> c_char;
  // proto:  bool QWaitCondition::wait(QMutex * lockedMutex, unsigned long time);
  fn _ZN14QWaitCondition4waitEP6QMutexm(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_ulong) -> c_char;
  // proto:  void QWaitCondition::wakeAll();
  fn _ZN14QWaitCondition7wakeAllEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QWaitCondition::wakeOne();
  fn _ZN14QWaitCondition7wakeOneEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QWaitCondition::QWaitCondition(const QWaitCondition & );
  fn dector_ZN14QWaitConditionC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN14QWaitConditionC1ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QWaitCondition::~QWaitCondition();
  fn _ZN14QWaitConditionD0Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QWaitCondition::QWaitCondition();
  fn dector_ZN14QWaitConditionC1Ev() -> *mut c_void;
  fn _ZN14QWaitConditionC1Ev(qthis: u64 /* *mut c_void*/);
} // <= ext block end

// body block begin =>
// class sizeof(QWaitCondition)=8
#[derive(Default)]
pub struct QWaitCondition {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QWaitCondition {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QWaitCondition {
    return QWaitCondition{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  bool QWaitCondition::wait(QReadWriteLock * lockedReadWriteLock, unsigned long time);
impl /*struct*/ QWaitCondition {
  pub fn wait<RetType, T: QWaitCondition_wait<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.wait(self);
    // return 1;
  }
}

pub trait QWaitCondition_wait<RetType> {
  fn wait(self , rsthis: & QWaitCondition) -> RetType;
}

  // proto:  bool QWaitCondition::wait(QReadWriteLock * lockedReadWriteLock, unsigned long time);
impl<'a> /*trait*/ QWaitCondition_wait<i8> for (&'a QReadWriteLock, u64) {
  fn wait(self , rsthis: & QWaitCondition) -> i8 {
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
impl<'a> /*trait*/ QWaitCondition_wait<i8> for (&'a QMutex, u64) {
  fn wait(self , rsthis: & QWaitCondition) -> i8 {
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
  pub fn wakeAll<RetType, T: QWaitCondition_wakeAll<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.wakeAll(self);
    // return 1;
  }
}

pub trait QWaitCondition_wakeAll<RetType> {
  fn wakeAll(self , rsthis: & QWaitCondition) -> RetType;
}

  // proto:  void QWaitCondition::wakeAll();
impl<'a> /*trait*/ QWaitCondition_wakeAll<()> for () {
  fn wakeAll(self , rsthis: & QWaitCondition) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QWaitCondition7wakeAllEv()};
     unsafe {_ZN14QWaitCondition7wakeAllEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QWaitCondition::wakeOne();
impl /*struct*/ QWaitCondition {
  pub fn wakeOne<RetType, T: QWaitCondition_wakeOne<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.wakeOne(self);
    // return 1;
  }
}

pub trait QWaitCondition_wakeOne<RetType> {
  fn wakeOne(self , rsthis: & QWaitCondition) -> RetType;
}

  // proto:  void QWaitCondition::wakeOne();
impl<'a> /*trait*/ QWaitCondition_wakeOne<()> for () {
  fn wakeOne(self , rsthis: & QWaitCondition) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QWaitCondition7wakeOneEv()};
     unsafe {_ZN14QWaitCondition7wakeOneEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QWaitCondition::QWaitCondition(const QWaitCondition & );
impl /*struct*/ QWaitCondition {
  pub fn new<T: QWaitCondition_new>(value: T) -> QWaitCondition {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QWaitCondition_new {
  fn new(self) -> QWaitCondition;
}

  // proto:  void QWaitCondition::QWaitCondition(const QWaitCondition & );
impl<'a> /*trait*/ QWaitCondition_new for (&'a QWaitCondition) {
  fn new(self) -> QWaitCondition {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QWaitConditionC1ERKS_()};
    let ctysz: c_int = unsafe{QWaitCondition_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN14QWaitConditionC1ERKS_(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN14QWaitConditionC1ERKS_(arg0)} as u64;
    let rsthis = QWaitCondition{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QWaitCondition::~QWaitCondition();
impl /*struct*/ QWaitCondition {
  pub fn free<RetType, T: QWaitCondition_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QWaitCondition_free<RetType> {
  fn free(self , rsthis: & QWaitCondition) -> RetType;
}

  // proto:  void QWaitCondition::~QWaitCondition();
impl<'a> /*trait*/ QWaitCondition_free<()> for () {
  fn free(self , rsthis: & QWaitCondition) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QWaitConditionD0Ev()};
     unsafe {_ZN14QWaitConditionD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QWaitCondition::QWaitCondition();
impl<'a> /*trait*/ QWaitCondition_new for () {
  fn new(self) -> QWaitCondition {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QWaitConditionC1Ev()};
    let ctysz: c_int = unsafe{QWaitCondition_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    // unsafe {_ZN14QWaitConditionC1Ev(qthis)};
    let qthis: u64 = unsafe {dector_ZN14QWaitConditionC1Ev()} as u64;
    let rsthis = QWaitCondition{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// <= body block end

