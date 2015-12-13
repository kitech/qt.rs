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
  fn _ZN11QBasicTimerD0Ev() -> i32;
  fn _ZN11QBasicTimer4stopEv() -> i32;
  fn _ZNK11QBasicTimer7timerIdEv() -> i32;
  fn _ZNK11QBasicTimer8isActiveEv() -> i32;
  fn _ZN11QBasicTimerC1Ev(qthis: *mut c_void) -> i32;
  fn _ZN11QBasicTimer5startEiP7QObject(arg0: c_int, arg1: *mut c_void) -> i32;
}

// body block begin
// class sizeof(QBasicTimer)=4
pub struct QBasicTimer {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QBasicTimer {
  pub fn FreeQBasicTimer<T: QBasicTimer_FreeQBasicTimer>(&mut self, value: T) -> i32 {
    value.FreeQBasicTimer(self);
    return 1;
  }
}

pub trait QBasicTimer_FreeQBasicTimer {
  fn FreeQBasicTimer(self, this: &mut QBasicTimer) -> i32;
}

// proto: void QBasicTimer::FreeQBasicTimer();
impl<'a> /*trait*/ QBasicTimer_FreeQBasicTimer for () {
  fn FreeQBasicTimer(self, this: &mut QBasicTimer) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QBasicTimerD0Ev()};
    unsafe {_ZN11QBasicTimerD0Ev()};
    return 1;
  }
}

impl /*struct*/ QBasicTimer {
  pub fn stop<T: QBasicTimer_stop>(&mut self, value: T) -> i32 {
    value.stop(self);
    return 1;
  }
}

pub trait QBasicTimer_stop {
  fn stop(self, this: &mut QBasicTimer) -> i32;
}

// proto: void QBasicTimer::stop();
impl<'a> /*trait*/ QBasicTimer_stop for () {
  fn stop(self, this: &mut QBasicTimer) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QBasicTimer4stopEv()};
    unsafe {_ZN11QBasicTimer4stopEv()};
    return 1;
  }
}

impl /*struct*/ QBasicTimer {
  pub fn timerId<T: QBasicTimer_timerId>(&mut self, value: T) -> i32 {
    value.timerId(self);
    return 1;
  }
}

pub trait QBasicTimer_timerId {
  fn timerId(self, this: &mut QBasicTimer) -> i32;
}

// proto: int QBasicTimer::timerId();
impl<'a> /*trait*/ QBasicTimer_timerId for () {
  fn timerId(self, this: &mut QBasicTimer) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QBasicTimer7timerIdEv()};
    unsafe {_ZNK11QBasicTimer7timerIdEv()};
    return 1;
  }
}

impl /*struct*/ QBasicTimer {
  pub fn isActive<T: QBasicTimer_isActive>(&mut self, value: T) -> i32 {
    value.isActive(self);
    return 1;
  }
}

pub trait QBasicTimer_isActive {
  fn isActive(self, this: &mut QBasicTimer) -> i32;
}

// proto: bool QBasicTimer::isActive();
impl<'a> /*trait*/ QBasicTimer_isActive for () {
  fn isActive(self, this: &mut QBasicTimer) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QBasicTimer8isActiveEv()};
    unsafe {_ZNK11QBasicTimer8isActiveEv()};
    return 1;
  }
}

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

// proto: void QBasicTimer::NewQBasicTimer();
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

impl /*struct*/ QBasicTimer {
  pub fn start<T: QBasicTimer_start>(&mut self, value: T) -> i32 {
    value.start(self);
    return 1;
  }
}

pub trait QBasicTimer_start {
  fn start(self, this: &mut QBasicTimer) -> i32;
}

// proto: void QBasicTimer::start(int msec, QObject * obj);
impl<'a> /*trait*/ QBasicTimer_start for (i32, &'a mut QObject) {
  fn start(self, this: &mut QBasicTimer) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QBasicTimer5startEiP7QObject()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN11QBasicTimer5startEiP7QObject(arg0, arg1)};
    return 1;
  }
}

