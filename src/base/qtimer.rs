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
  fn _ZN6QTimerD0Ev() -> i32;
  fn _ZN6QTimer4stopEv() -> i32;
  fn _ZNK6QTimer7timerIdEv() -> i32;
  fn _ZN6QTimer13setSingleShotEb(arg0: int8_t) -> i32;
  fn _ZN6QTimer10singleShotEiPK7QObjectPKc(arg0: c_int, arg1: *const c_void, arg2: *const c_char) -> i32;
  fn _ZN6QTimer5startEv() -> i32;
  fn _ZNK6QTimer8intervalEv() -> i32;
  fn _ZN6QTimer11setIntervalEi(arg0: c_int) -> i32;
  fn _ZN6QTimer5startEi(arg0: c_int) -> i32;
  fn _ZN6QTimerC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZNK6QTimer13remainingTimeEv() -> i32;
  fn _ZNK6QTimer12isSingleShotEv() -> i32;
  fn _ZNK6QTimer8isActiveEv() -> i32;
  fn _ZNK6QTimer10metaObjectEv() -> i32;
  fn _ZN6QTimerC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) -> i32;
}

// body block begin
// class sizeof(QTimer)=1
pub struct QTimer {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTimer {
  pub fn FreeQTimer<T: QTimer_FreeQTimer>(&mut self, value: T) -> i32 {
    value.FreeQTimer(self);
    return 1;
  }
}

pub trait QTimer_FreeQTimer {
  fn FreeQTimer(self, this: &mut QTimer) -> i32;
}

// proto: void QTimer::FreeQTimer();
impl<'a> /*trait*/ QTimer_FreeQTimer for () {
  fn FreeQTimer(self, this: &mut QTimer) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QTimerD0Ev()};
    unsafe {_ZN6QTimerD0Ev()};
    return 1;
  }
}

impl /*struct*/ QTimer {
  pub fn stop<T: QTimer_stop>(&mut self, value: T) -> i32 {
    value.stop(self);
    return 1;
  }
}

pub trait QTimer_stop {
  fn stop(self, this: &mut QTimer) -> i32;
}

// proto: void QTimer::stop();
impl<'a> /*trait*/ QTimer_stop for () {
  fn stop(self, this: &mut QTimer) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QTimer4stopEv()};
    unsafe {_ZN6QTimer4stopEv()};
    return 1;
  }
}

impl /*struct*/ QTimer {
  pub fn timerId<T: QTimer_timerId>(&mut self, value: T) -> i32 {
    value.timerId(self);
    return 1;
  }
}

pub trait QTimer_timerId {
  fn timerId(self, this: &mut QTimer) -> i32;
}

// proto: int QTimer::timerId();
impl<'a> /*trait*/ QTimer_timerId for () {
  fn timerId(self, this: &mut QTimer) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QTimer7timerIdEv()};
    unsafe {_ZNK6QTimer7timerIdEv()};
    return 1;
  }
}

impl /*struct*/ QTimer {
  pub fn setSingleShot<T: QTimer_setSingleShot>(&mut self, value: T) -> i32 {
    value.setSingleShot(self);
    return 1;
  }
}

pub trait QTimer_setSingleShot {
  fn setSingleShot(self, this: &mut QTimer) -> i32;
}

// proto: void QTimer::setSingleShot(bool singleShot);
impl<'a> /*trait*/ QTimer_setSingleShot for (i8) {
  fn setSingleShot(self, this: &mut QTimer) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QTimer13setSingleShotEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN6QTimer13setSingleShotEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QTimer {
  pub fn singleShot<T: QTimer_singleShot>(&mut self, value: T) -> i32 {
    value.singleShot(self);
    return 1;
  }
}

pub trait QTimer_singleShot {
  fn singleShot(self, this: &mut QTimer) -> i32;
}

// proto: void QTimer::singleShot(int msec, const QObject * receiver, const char * member);
impl<'a> /*trait*/ QTimer_singleShot for (i32, &'a  QObject, &'a  String) {
  fn singleShot(self, this: &mut QTimer) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QTimer10singleShotEiPK7QObjectPKc()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2.as_ptr()  as *const c_char;
    unsafe {_ZN6QTimer10singleShotEiPK7QObjectPKc(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QTimer {
  pub fn start<T: QTimer_start>(&mut self, value: T) -> i32 {
    value.start(self);
    return 1;
  }
}

pub trait QTimer_start {
  fn start(self, this: &mut QTimer) -> i32;
}

// proto: void QTimer::start();
impl<'a> /*trait*/ QTimer_start for () {
  fn start(self, this: &mut QTimer) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QTimer5startEv()};
    unsafe {_ZN6QTimer5startEv()};
    return 1;
  }
}

impl /*struct*/ QTimer {
  pub fn interval<T: QTimer_interval>(&mut self, value: T) -> i32 {
    value.interval(self);
    return 1;
  }
}

pub trait QTimer_interval {
  fn interval(self, this: &mut QTimer) -> i32;
}

// proto: int QTimer::interval();
impl<'a> /*trait*/ QTimer_interval for () {
  fn interval(self, this: &mut QTimer) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QTimer8intervalEv()};
    unsafe {_ZNK6QTimer8intervalEv()};
    return 1;
  }
}

impl /*struct*/ QTimer {
  pub fn setInterval<T: QTimer_setInterval>(&mut self, value: T) -> i32 {
    value.setInterval(self);
    return 1;
  }
}

pub trait QTimer_setInterval {
  fn setInterval(self, this: &mut QTimer) -> i32;
}

// proto: void QTimer::setInterval(int msec);
impl<'a> /*trait*/ QTimer_setInterval for (i32) {
  fn setInterval(self, this: &mut QTimer) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QTimer11setIntervalEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN6QTimer11setIntervalEi(arg0)};
    return 1;
  }
}

// proto: void QTimer::start(int msec);
impl<'a> /*trait*/ QTimer_start for (i32) {
  fn start(self, this: &mut QTimer) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QTimer5startEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN6QTimer5startEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTimer {
  pub fn NewQTimer<T: QTimer_NewQTimer>(value: T) -> QTimer {
    let rsthis = value.NewQTimer();
    return rsthis;
    // return 1;
  }
}

pub trait QTimer_NewQTimer {
  fn NewQTimer(self) -> QTimer;
}

// proto: void QTimer::NewQTimer(const QTimer & );
impl<'a> /*trait*/ QTimer_NewQTimer for (&'a  QTimer) {
  fn NewQTimer(self) -> QTimer {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QTimerC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN6QTimerC1ERKS_(qthis, arg0)};
    let rsthis = QTimer{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTimer {
  pub fn remainingTime<T: QTimer_remainingTime>(&mut self, value: T) -> i32 {
    value.remainingTime(self);
    return 1;
  }
}

pub trait QTimer_remainingTime {
  fn remainingTime(self, this: &mut QTimer) -> i32;
}

// proto: int QTimer::remainingTime();
impl<'a> /*trait*/ QTimer_remainingTime for () {
  fn remainingTime(self, this: &mut QTimer) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QTimer13remainingTimeEv()};
    unsafe {_ZNK6QTimer13remainingTimeEv()};
    return 1;
  }
}

impl /*struct*/ QTimer {
  pub fn isSingleShot<T: QTimer_isSingleShot>(&mut self, value: T) -> i32 {
    value.isSingleShot(self);
    return 1;
  }
}

pub trait QTimer_isSingleShot {
  fn isSingleShot(self, this: &mut QTimer) -> i32;
}

// proto: bool QTimer::isSingleShot();
impl<'a> /*trait*/ QTimer_isSingleShot for () {
  fn isSingleShot(self, this: &mut QTimer) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QTimer12isSingleShotEv()};
    unsafe {_ZNK6QTimer12isSingleShotEv()};
    return 1;
  }
}

impl /*struct*/ QTimer {
  pub fn isActive<T: QTimer_isActive>(&mut self, value: T) -> i32 {
    value.isActive(self);
    return 1;
  }
}

pub trait QTimer_isActive {
  fn isActive(self, this: &mut QTimer) -> i32;
}

// proto: bool QTimer::isActive();
impl<'a> /*trait*/ QTimer_isActive for () {
  fn isActive(self, this: &mut QTimer) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QTimer8isActiveEv()};
    unsafe {_ZNK6QTimer8isActiveEv()};
    return 1;
  }
}

impl /*struct*/ QTimer {
  pub fn metaObject<T: QTimer_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QTimer_metaObject {
  fn metaObject(self, this: &mut QTimer) -> i32;
}

// proto: const QMetaObject * QTimer::metaObject();
impl<'a> /*trait*/ QTimer_metaObject for () {
  fn metaObject(self, this: &mut QTimer) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QTimer10metaObjectEv()};
    unsafe {_ZNK6QTimer10metaObjectEv()};
    return 1;
  }
}

// proto: void QTimer::NewQTimer(QObject * parent);
impl<'a> /*trait*/ QTimer_NewQTimer for (&'a mut QObject) {
  fn NewQTimer(self) -> QTimer {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QTimerC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN6QTimerC1EP7QObject(qthis, arg0)};
    let rsthis = QTimer{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

