// auto generated, do not modify.
// created: Tue Dec 22 23:21:28 2015
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
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  void QBasicTimer::~QBasicTimer();
  fn _ZN11QBasicTimerD0Ev(qthis: *mut c_void);
  // proto:  void QBasicTimer::stop();
  fn _ZN11QBasicTimer4stopEv(qthis: *mut c_void);
  // proto:  int QBasicTimer::timerId();
  fn _ZNK11QBasicTimer7timerIdEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QBasicTimer::isActive();
  fn _ZNK11QBasicTimer8isActiveEv(qthis: *mut c_void) -> c_char;
  // proto:  void QBasicTimer::QBasicTimer();
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
  pub fn FreeQBasicTimer<RetType, T: QBasicTimer_FreeQBasicTimer<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQBasicTimer(self);
    // return 1;
  }
}

pub trait QBasicTimer_FreeQBasicTimer<RetType> {
  fn FreeQBasicTimer(self , rsthis: &mut QBasicTimer) -> RetType;
}

  // proto:  void QBasicTimer::~QBasicTimer();
impl<'a> /*trait*/ QBasicTimer_FreeQBasicTimer<()> for () {
  fn FreeQBasicTimer(self , rsthis: &mut QBasicTimer) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QBasicTimerD0Ev()};
     unsafe {_ZN11QBasicTimerD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QBasicTimer::stop();
impl /*struct*/ QBasicTimer {
  pub fn stop<RetType, T: QBasicTimer_stop<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.stop(self);
    // return 1;
  }
}

pub trait QBasicTimer_stop<RetType> {
  fn stop(self , rsthis: &mut QBasicTimer) -> RetType;
}

  // proto:  void QBasicTimer::stop();
impl<'a> /*trait*/ QBasicTimer_stop<()> for () {
  fn stop(self , rsthis: &mut QBasicTimer) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QBasicTimer4stopEv()};
     unsafe {_ZN11QBasicTimer4stopEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QBasicTimer::timerId();
impl /*struct*/ QBasicTimer {
  pub fn timerId<RetType, T: QBasicTimer_timerId<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.timerId(self);
    // return 1;
  }
}

pub trait QBasicTimer_timerId<RetType> {
  fn timerId(self , rsthis: &mut QBasicTimer) -> RetType;
}

  // proto:  int QBasicTimer::timerId();
impl<'a> /*trait*/ QBasicTimer_timerId<i32> for () {
  fn timerId(self , rsthis: &mut QBasicTimer) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QBasicTimer7timerIdEv()};
    let mut ret = unsafe {_ZNK11QBasicTimer7timerIdEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  bool QBasicTimer::isActive();
impl /*struct*/ QBasicTimer {
  pub fn isActive<RetType, T: QBasicTimer_isActive<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isActive(self);
    // return 1;
  }
}

pub trait QBasicTimer_isActive<RetType> {
  fn isActive(self , rsthis: &mut QBasicTimer) -> RetType;
}

  // proto:  bool QBasicTimer::isActive();
impl<'a> /*trait*/ QBasicTimer_isActive<i8> for () {
  fn isActive(self , rsthis: &mut QBasicTimer) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QBasicTimer8isActiveEv()};
    let mut ret = unsafe {_ZNK11QBasicTimer8isActiveEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QBasicTimer::QBasicTimer();
impl /*struct*/ QBasicTimer {
  pub fn NewQBasicTimer<T: QBasicTimer_NewQBasicTimer>(value: T) -> QBasicTimer {
    let rsthis = value.NewQBasicTimer();
    return rsthis;
    // return 1;
  }
}

pub trait QBasicTimer_NewQBasicTimer {
  fn NewQBasicTimer(self) -> QBasicTimer;
}

  // proto:  void QBasicTimer::QBasicTimer();
impl<'a> /*trait*/ QBasicTimer_NewQBasicTimer for () {
  fn NewQBasicTimer(self) -> QBasicTimer {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QBasicTimerC1Ev()};
    unsafe {_ZN11QBasicTimerC1Ev(qthis)};
    let rsthis = QBasicTimer{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QBasicTimer::start(int msec, QObject * obj);
impl /*struct*/ QBasicTimer {
  pub fn start<RetType, T: QBasicTimer_start<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.start(self);
    // return 1;
  }
}

pub trait QBasicTimer_start<RetType> {
  fn start(self , rsthis: &mut QBasicTimer) -> RetType;
}

  // proto:  void QBasicTimer::start(int msec, QObject * obj);
impl<'a> /*trait*/ QBasicTimer_start<()> for (i32, QObject) {
  fn start(self , rsthis: &mut QBasicTimer) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QBasicTimer5startEiP7QObject()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN11QBasicTimer5startEiP7QObject(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// <= body block end

