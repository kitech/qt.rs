// auto generated, do not modify.
// created: Sat Dec 26 10:52:38 2015
// src-file: /QtCore/qbasictimer.h
// dst-file: /src/core/qbasictimer.rs
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
use super::qobject::QObject; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QBasicTimer_Class_Size() -> c_int;
  // proto:  void QBasicTimer::~QBasicTimer();
  fn _ZN11QBasicTimerD0Ev(qthis: *mut c_void);
  // proto:  void QBasicTimer::stop();
  fn _ZN11QBasicTimer4stopEv(qthis: *mut c_void);
  // proto:  int QBasicTimer::timerId();
  fn _ZNK11QBasicTimer7timerIdEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QBasicTimer::isActive();
  fn _ZNK11QBasicTimer8isActiveEv(qthis: *mut c_void) -> c_char;
  // proto:  void QBasicTimer::QBasicTimer();
  fn dector_ZN11QBasicTimerC1Ev() -> *mut c_void;
  fn _ZN11QBasicTimerC1Ev(qthis: *mut c_void);
  // proto:  void QBasicTimer::start(int msec, QObject * obj);
  fn _ZN11QBasicTimer5startEiP7QObject(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QBasicTimer)=4
pub struct QBasicTimer {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QBasicTimer {
  pub fn inheritFrom(qthis: *mut c_void) -> QBasicTimer {
    return QBasicTimer{qclsinst: qthis};
  }
}
  // proto:  void QBasicTimer::~QBasicTimer();
impl /*struct*/ QBasicTimer {
  pub fn Free<RetType, T: QBasicTimer_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QBasicTimer_Free<RetType> {
  fn Free(self , rsthis: & QBasicTimer) -> RetType;
}

  // proto:  void QBasicTimer::~QBasicTimer();
impl<'a> /*trait*/ QBasicTimer_Free<()> for () {
  fn Free(self , rsthis: & QBasicTimer) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QBasicTimerD0Ev()};
     unsafe {_ZN11QBasicTimerD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QBasicTimer::stop();
impl /*struct*/ QBasicTimer {
  pub fn stop<RetType, T: QBasicTimer_stop<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.stop(self);
    // return 1;
  }
}

pub trait QBasicTimer_stop<RetType> {
  fn stop(self , rsthis: & QBasicTimer) -> RetType;
}

  // proto:  void QBasicTimer::stop();
impl<'a> /*trait*/ QBasicTimer_stop<()> for () {
  fn stop(self , rsthis: & QBasicTimer) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QBasicTimer4stopEv()};
     unsafe {_ZN11QBasicTimer4stopEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QBasicTimer::timerId();
impl /*struct*/ QBasicTimer {
  pub fn timerId<RetType, T: QBasicTimer_timerId<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.timerId(self);
    // return 1;
  }
}

pub trait QBasicTimer_timerId<RetType> {
  fn timerId(self , rsthis: & QBasicTimer) -> RetType;
}

  // proto:  int QBasicTimer::timerId();
impl<'a> /*trait*/ QBasicTimer_timerId<i32> for () {
  fn timerId(self , rsthis: & QBasicTimer) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QBasicTimer7timerIdEv()};
    let mut ret = unsafe {_ZNK11QBasicTimer7timerIdEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  bool QBasicTimer::isActive();
impl /*struct*/ QBasicTimer {
  pub fn isActive<RetType, T: QBasicTimer_isActive<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isActive(self);
    // return 1;
  }
}

pub trait QBasicTimer_isActive<RetType> {
  fn isActive(self , rsthis: & QBasicTimer) -> RetType;
}

  // proto:  bool QBasicTimer::isActive();
impl<'a> /*trait*/ QBasicTimer_isActive<i8> for () {
  fn isActive(self , rsthis: & QBasicTimer) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QBasicTimer8isActiveEv()};
    let mut ret = unsafe {_ZNK11QBasicTimer8isActiveEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QBasicTimer::QBasicTimer();
impl /*struct*/ QBasicTimer {
  pub fn New<T: QBasicTimer_New>(value: T) -> QBasicTimer {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QBasicTimer_New {
  fn New(self) -> QBasicTimer;
}

  // proto:  void QBasicTimer::QBasicTimer();
impl<'a> /*trait*/ QBasicTimer_New for () {
  fn New(self) -> QBasicTimer {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QBasicTimerC1Ev()};
    let ctysz: c_int = unsafe{QBasicTimer_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    // unsafe {_ZN11QBasicTimerC1Ev(qthis)};
    let qthis: *mut c_void = unsafe {dector_ZN11QBasicTimerC1Ev()};
    let rsthis = QBasicTimer{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QBasicTimer::start(int msec, QObject * obj);
impl /*struct*/ QBasicTimer {
  pub fn start<RetType, T: QBasicTimer_start<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.start(self);
    // return 1;
  }
}

pub trait QBasicTimer_start<RetType> {
  fn start(self , rsthis: & QBasicTimer) -> RetType;
}

  // proto:  void QBasicTimer::start(int msec, QObject * obj);
impl<'a> /*trait*/ QBasicTimer_start<()> for (i32, &'a QObject) {
  fn start(self , rsthis: & QBasicTimer) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QBasicTimer5startEiP7QObject()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN11QBasicTimer5startEiP7QObject(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// <= body block end

