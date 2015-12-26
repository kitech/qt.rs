// auto generated, do not modify.
// created: Sat Dec 26 10:16:52 2015
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
  // proto:  void QBasicTimer::start(int msec, QObject * obj);
  fn _ZN11QBasicTimer5startEiP7QObject(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void);
  // proto:  void QBasicTimer::stop();
  fn _ZN11QBasicTimer4stopEv(qthis: *mut c_void);
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

// <= body block end

