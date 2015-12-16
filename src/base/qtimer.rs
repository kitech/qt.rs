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
  // proto:  void QTimer::FreeQTimer();
  fn _ZN6QTimerD0Ev(qthis: *mut c_void) ;
  // proto:  void QTimer::stop();
  fn _ZN6QTimer4stopEv(qthis: *mut c_void) ;
  // proto:  int QTimer::timerId();
  fn _ZNK6QTimer7timerIdEv(qthis: *mut c_void) -> c_int;
  // proto:  void QTimer::setSingleShot(bool singleShot);
  fn _ZN6QTimer13setSingleShotEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto: static void QTimer::singleShot(int msec, const QObject * receiver, const char * member);
  fn _ZN6QTimer10singleShotEiPK7QObjectPKc(arg0: c_int, arg1: *mut c_void, arg2: *const c_char) ;
  // proto:  void QTimer::start();
  fn _ZN6QTimer5startEv(qthis: *mut c_void) ;
  // proto:  int QTimer::interval();
  fn _ZNK6QTimer8intervalEv(qthis: *mut c_void) -> c_int;
  // proto:  void QTimer::setInterval(int msec);
  fn _ZN6QTimer11setIntervalEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QTimer::start(int msec);
  fn _ZN6QTimer5startEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QTimer::NewQTimer(const QTimer & );
  fn _ZN6QTimerC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  int QTimer::remainingTime();
  fn _ZNK6QTimer13remainingTimeEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QTimer::isSingleShot();
  fn _ZNK6QTimer12isSingleShotEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QTimer::isActive();
  fn _ZNK6QTimer8isActiveEv(qthis: *mut c_void) -> int8_t;
  // proto:  const QMetaObject * QTimer::metaObject();
  fn _ZNK6QTimer10metaObjectEv(qthis: *mut c_void) ;
  // proto:  void QTimer::NewQTimer(QObject * parent);
  fn _ZN6QTimerC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) ;
}

// body block begin
// class sizeof(QTimer)=1
pub struct QTimer {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTimer {
  pub fn FreeQTimer<T: QTimer_FreeQTimer>(&mut self, value: T)  {
     value.FreeQTimer(self);
    // return 1;
  }
}

pub trait QTimer_FreeQTimer {
  fn FreeQTimer(self, rsthis: &mut QTimer) ;
}

// proto:  void QTimer::FreeQTimer();
impl<'a> /*trait*/ QTimer_FreeQTimer for () {
  fn FreeQTimer(self, rsthis: &mut QTimer)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QTimerD0Ev()};
     unsafe {_ZN6QTimerD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTimer {
  pub fn stop<T: QTimer_stop>(&mut self, value: T)  {
     value.stop(self);
    // return 1;
  }
}

pub trait QTimer_stop {
  fn stop(self, rsthis: &mut QTimer) ;
}

// proto:  void QTimer::stop();
impl<'a> /*trait*/ QTimer_stop for () {
  fn stop(self, rsthis: &mut QTimer)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QTimer4stopEv()};
     unsafe {_ZN6QTimer4stopEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTimer {
  pub fn timerId<T: QTimer_timerId>(&mut self, value: T) -> i32 {
    return value.timerId(self);
    // return 1;
  }
}

pub trait QTimer_timerId {
  fn timerId(self, rsthis: &mut QTimer) -> i32;
}

// proto:  int QTimer::timerId();
impl<'a> /*trait*/ QTimer_timerId for () {
  fn timerId(self, rsthis: &mut QTimer) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QTimer7timerIdEv()};
    let mut ret = unsafe {_ZNK6QTimer7timerIdEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTimer {
  pub fn setSingleShot<T: QTimer_setSingleShot>(&mut self, value: T)  {
     value.setSingleShot(self);
    // return 1;
  }
}

pub trait QTimer_setSingleShot {
  fn setSingleShot(self, rsthis: &mut QTimer) ;
}

// proto:  void QTimer::setSingleShot(bool singleShot);
impl<'a> /*trait*/ QTimer_setSingleShot for (i8) {
  fn setSingleShot(self, rsthis: &mut QTimer)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QTimer13setSingleShotEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN6QTimer13setSingleShotEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTimer {
  pub fn singleShot<T: QTimer_singleShot>(&mut self, value: T)  {
     value.singleShot(self);
    // return 1;
  }
}

pub trait QTimer_singleShot {
  fn singleShot(self, rsthis: &mut QTimer) ;
}

// proto: static void QTimer::singleShot(int msec, const QObject * receiver, const char * member);
impl<'a> /*trait*/ QTimer_singleShot for (i32, &'a  QObject, &'a  String) {
  fn singleShot(self, rsthis: &mut QTimer)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QTimer10singleShotEiPK7QObjectPKc()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.as_ptr()  as *const c_char;
     unsafe {_ZN6QTimer10singleShotEiPK7QObjectPKc(arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QTimer {
  pub fn start<T: QTimer_start>(&mut self, value: T)  {
     value.start(self);
    // return 1;
  }
}

pub trait QTimer_start {
  fn start(self, rsthis: &mut QTimer) ;
}

// proto:  void QTimer::start();
impl<'a> /*trait*/ QTimer_start for () {
  fn start(self, rsthis: &mut QTimer)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QTimer5startEv()};
     unsafe {_ZN6QTimer5startEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTimer {
  pub fn interval<T: QTimer_interval>(&mut self, value: T) -> i32 {
    return value.interval(self);
    // return 1;
  }
}

pub trait QTimer_interval {
  fn interval(self, rsthis: &mut QTimer) -> i32;
}

// proto:  int QTimer::interval();
impl<'a> /*trait*/ QTimer_interval for () {
  fn interval(self, rsthis: &mut QTimer) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QTimer8intervalEv()};
    let mut ret = unsafe {_ZNK6QTimer8intervalEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTimer {
  pub fn setInterval<T: QTimer_setInterval>(&mut self, value: T)  {
     value.setInterval(self);
    // return 1;
  }
}

pub trait QTimer_setInterval {
  fn setInterval(self, rsthis: &mut QTimer) ;
}

// proto:  void QTimer::setInterval(int msec);
impl<'a> /*trait*/ QTimer_setInterval for (i32) {
  fn setInterval(self, rsthis: &mut QTimer)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QTimer11setIntervalEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN6QTimer11setIntervalEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QTimer::start(int msec);
impl<'a> /*trait*/ QTimer_start for (i32) {
  fn start(self, rsthis: &mut QTimer)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QTimer5startEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN6QTimer5startEi(rsthis.qclsinst, arg0)};
    // return 1;
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
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN6QTimerC1ERKS_(qthis, arg0)};
    let rsthis = QTimer{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTimer {
  pub fn remainingTime<T: QTimer_remainingTime>(&mut self, value: T) -> i32 {
    return value.remainingTime(self);
    // return 1;
  }
}

pub trait QTimer_remainingTime {
  fn remainingTime(self, rsthis: &mut QTimer) -> i32;
}

// proto:  int QTimer::remainingTime();
impl<'a> /*trait*/ QTimer_remainingTime for () {
  fn remainingTime(self, rsthis: &mut QTimer) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QTimer13remainingTimeEv()};
    let mut ret = unsafe {_ZNK6QTimer13remainingTimeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTimer {
  pub fn isSingleShot<T: QTimer_isSingleShot>(&mut self, value: T) -> i8 {
    return value.isSingleShot(self);
    // return 1;
  }
}

pub trait QTimer_isSingleShot {
  fn isSingleShot(self, rsthis: &mut QTimer) -> i8;
}

// proto:  bool QTimer::isSingleShot();
impl<'a> /*trait*/ QTimer_isSingleShot for () {
  fn isSingleShot(self, rsthis: &mut QTimer) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QTimer12isSingleShotEv()};
    let mut ret = unsafe {_ZNK6QTimer12isSingleShotEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTimer {
  pub fn isActive<T: QTimer_isActive>(&mut self, value: T) -> i8 {
    return value.isActive(self);
    // return 1;
  }
}

pub trait QTimer_isActive {
  fn isActive(self, rsthis: &mut QTimer) -> i8;
}

// proto:  bool QTimer::isActive();
impl<'a> /*trait*/ QTimer_isActive for () {
  fn isActive(self, rsthis: &mut QTimer) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QTimer8isActiveEv()};
    let mut ret = unsafe {_ZNK6QTimer8isActiveEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTimer {
  pub fn metaObject<T: QTimer_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QTimer_metaObject {
  fn metaObject(self, rsthis: &mut QTimer) ;
}

// proto:  const QMetaObject * QTimer::metaObject();
impl<'a> /*trait*/ QTimer_metaObject for () {
  fn metaObject(self, rsthis: &mut QTimer)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QTimer10metaObjectEv()};
     unsafe {_ZNK6QTimer10metaObjectEv(rsthis.qclsinst)};
    // return 1;
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

