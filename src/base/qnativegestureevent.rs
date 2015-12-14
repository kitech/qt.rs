// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qpointf::QPointF;
use super::qpoint::QPoint;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  const QPointF & QNativeGestureEvent::localPos();
  fn _ZNK19QNativeGestureEvent8localPosEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  const QPointF & QNativeGestureEvent::screenPos();
  fn _ZNK19QNativeGestureEvent9screenPosEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  const QPoint QNativeGestureEvent::pos();
  fn _ZNK19QNativeGestureEvent3posEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  const QPoint QNativeGestureEvent::globalPos();
  fn _ZNK19QNativeGestureEvent9globalPosEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  double QNativeGestureEvent::value();
  fn _ZNK19QNativeGestureEvent5valueEv(qthis: *mut c_void) -> c_double;
  // proto:  const QPointF & QNativeGestureEvent::windowPos();
  fn _ZNK19QNativeGestureEvent9windowPosEv(qthis: *mut c_void) -> *mut c_void;
}

// body block begin
// class sizeof(QNativeGestureEvent)=1
pub struct QNativeGestureEvent {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QNativeGestureEvent {
  pub fn localPos<T: QNativeGestureEvent_localPos>(&mut self, value: T) -> QPointF {
    return value.localPos(self);
    // return 1;
  }
}

pub trait QNativeGestureEvent_localPos {
  fn localPos(self, rsthis: &mut QNativeGestureEvent) -> QPointF;
}

// proto:  const QPointF & QNativeGestureEvent::localPos();
impl<'a> /*trait*/ QNativeGestureEvent_localPos for () {
  fn localPos(self, rsthis: &mut QNativeGestureEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QNativeGestureEvent8localPosEv()};
    let mut ret = unsafe {_ZNK19QNativeGestureEvent8localPosEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QNativeGestureEvent {
  pub fn screenPos<T: QNativeGestureEvent_screenPos>(&mut self, value: T) -> QPointF {
    return value.screenPos(self);
    // return 1;
  }
}

pub trait QNativeGestureEvent_screenPos {
  fn screenPos(self, rsthis: &mut QNativeGestureEvent) -> QPointF;
}

// proto:  const QPointF & QNativeGestureEvent::screenPos();
impl<'a> /*trait*/ QNativeGestureEvent_screenPos for () {
  fn screenPos(self, rsthis: &mut QNativeGestureEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QNativeGestureEvent9screenPosEv()};
    let mut ret = unsafe {_ZNK19QNativeGestureEvent9screenPosEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QNativeGestureEvent {
  pub fn pos<T: QNativeGestureEvent_pos>(&mut self, value: T) -> QPoint {
    return value.pos(self);
    // return 1;
  }
}

pub trait QNativeGestureEvent_pos {
  fn pos(self, rsthis: &mut QNativeGestureEvent) -> QPoint;
}

// proto:  const QPoint QNativeGestureEvent::pos();
impl<'a> /*trait*/ QNativeGestureEvent_pos for () {
  fn pos(self, rsthis: &mut QNativeGestureEvent) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QNativeGestureEvent3posEv()};
    let mut ret = unsafe {_ZNK19QNativeGestureEvent3posEv(rsthis.qclsinst)};
    let mut ret1 = QPoint{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QNativeGestureEvent {
  pub fn globalPos<T: QNativeGestureEvent_globalPos>(&mut self, value: T) -> QPoint {
    return value.globalPos(self);
    // return 1;
  }
}

pub trait QNativeGestureEvent_globalPos {
  fn globalPos(self, rsthis: &mut QNativeGestureEvent) -> QPoint;
}

// proto:  const QPoint QNativeGestureEvent::globalPos();
impl<'a> /*trait*/ QNativeGestureEvent_globalPos for () {
  fn globalPos(self, rsthis: &mut QNativeGestureEvent) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QNativeGestureEvent9globalPosEv()};
    let mut ret = unsafe {_ZNK19QNativeGestureEvent9globalPosEv(rsthis.qclsinst)};
    let mut ret1 = QPoint{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QNativeGestureEvent {
  pub fn value<T: QNativeGestureEvent_value>(&mut self, value: T) -> f64 {
    return value.value(self);
    // return 1;
  }
}

pub trait QNativeGestureEvent_value {
  fn value(self, rsthis: &mut QNativeGestureEvent) -> f64;
}

// proto:  double QNativeGestureEvent::value();
impl<'a> /*trait*/ QNativeGestureEvent_value for () {
  fn value(self, rsthis: &mut QNativeGestureEvent) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QNativeGestureEvent5valueEv()};
    let mut ret = unsafe {_ZNK19QNativeGestureEvent5valueEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QNativeGestureEvent {
  pub fn windowPos<T: QNativeGestureEvent_windowPos>(&mut self, value: T) -> QPointF {
    return value.windowPos(self);
    // return 1;
  }
}

pub trait QNativeGestureEvent_windowPos {
  fn windowPos(self, rsthis: &mut QNativeGestureEvent) -> QPointF;
}

// proto:  const QPointF & QNativeGestureEvent::windowPos();
impl<'a> /*trait*/ QNativeGestureEvent_windowPos for () {
  fn windowPos(self, rsthis: &mut QNativeGestureEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QNativeGestureEvent9windowPosEv()};
    let mut ret = unsafe {_ZNK19QNativeGestureEvent9windowPosEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

