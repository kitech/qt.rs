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

// proto:  const QPointF & QNativeGestureEvent::localPos();
impl /*struct*/ QNativeGestureEvent {
  pub fn localPos<RetType, T: QNativeGestureEvent_localPos<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.localPos(self);
    // return 1;
  }
}

pub trait QNativeGestureEvent_localPos<RetType> {
  fn localPos(self , rsthis: &mut QNativeGestureEvent) -> RetType;
}

// proto:  const QPointF & QNativeGestureEvent::localPos();
impl<'a> /*trait*/ QNativeGestureEvent_localPos<QPointF> for () {
  fn localPos(self , rsthis: &mut QNativeGestureEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QNativeGestureEvent8localPosEv()};
    let mut ret = unsafe {_ZNK19QNativeGestureEvent8localPosEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  const QPointF & QNativeGestureEvent::screenPos();
impl /*struct*/ QNativeGestureEvent {
  pub fn screenPos<RetType, T: QNativeGestureEvent_screenPos<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.screenPos(self);
    // return 1;
  }
}

pub trait QNativeGestureEvent_screenPos<RetType> {
  fn screenPos(self , rsthis: &mut QNativeGestureEvent) -> RetType;
}

// proto:  const QPointF & QNativeGestureEvent::screenPos();
impl<'a> /*trait*/ QNativeGestureEvent_screenPos<QPointF> for () {
  fn screenPos(self , rsthis: &mut QNativeGestureEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QNativeGestureEvent9screenPosEv()};
    let mut ret = unsafe {_ZNK19QNativeGestureEvent9screenPosEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  const QPoint QNativeGestureEvent::pos();
impl /*struct*/ QNativeGestureEvent {
  pub fn pos<RetType, T: QNativeGestureEvent_pos<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.pos(self);
    // return 1;
  }
}

pub trait QNativeGestureEvent_pos<RetType> {
  fn pos(self , rsthis: &mut QNativeGestureEvent) -> RetType;
}

// proto:  const QPoint QNativeGestureEvent::pos();
impl<'a> /*trait*/ QNativeGestureEvent_pos<QPoint> for () {
  fn pos(self , rsthis: &mut QNativeGestureEvent) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QNativeGestureEvent3posEv()};
    let mut ret = unsafe {_ZNK19QNativeGestureEvent3posEv(rsthis.qclsinst)};
    let mut ret1 = QPoint{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  const QPoint QNativeGestureEvent::globalPos();
impl /*struct*/ QNativeGestureEvent {
  pub fn globalPos<RetType, T: QNativeGestureEvent_globalPos<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.globalPos(self);
    // return 1;
  }
}

pub trait QNativeGestureEvent_globalPos<RetType> {
  fn globalPos(self , rsthis: &mut QNativeGestureEvent) -> RetType;
}

// proto:  const QPoint QNativeGestureEvent::globalPos();
impl<'a> /*trait*/ QNativeGestureEvent_globalPos<QPoint> for () {
  fn globalPos(self , rsthis: &mut QNativeGestureEvent) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QNativeGestureEvent9globalPosEv()};
    let mut ret = unsafe {_ZNK19QNativeGestureEvent9globalPosEv(rsthis.qclsinst)};
    let mut ret1 = QPoint{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  double QNativeGestureEvent::value();
impl /*struct*/ QNativeGestureEvent {
  pub fn value<RetType, T: QNativeGestureEvent_value<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.value(self);
    // return 1;
  }
}

pub trait QNativeGestureEvent_value<RetType> {
  fn value(self , rsthis: &mut QNativeGestureEvent) -> RetType;
}

// proto:  double QNativeGestureEvent::value();
impl<'a> /*trait*/ QNativeGestureEvent_value<f64> for () {
  fn value(self , rsthis: &mut QNativeGestureEvent) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QNativeGestureEvent5valueEv()};
    let mut ret = unsafe {_ZNK19QNativeGestureEvent5valueEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

// proto:  const QPointF & QNativeGestureEvent::windowPos();
impl /*struct*/ QNativeGestureEvent {
  pub fn windowPos<RetType, T: QNativeGestureEvent_windowPos<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.windowPos(self);
    // return 1;
  }
}

pub trait QNativeGestureEvent_windowPos<RetType> {
  fn windowPos(self , rsthis: &mut QNativeGestureEvent) -> RetType;
}

// proto:  const QPointF & QNativeGestureEvent::windowPos();
impl<'a> /*trait*/ QNativeGestureEvent_windowPos<QPointF> for () {
  fn windowPos(self , rsthis: &mut QNativeGestureEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QNativeGestureEvent9windowPosEv()};
    let mut ret = unsafe {_ZNK19QNativeGestureEvent9windowPosEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

