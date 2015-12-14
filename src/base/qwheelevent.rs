// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qpoint::QPoint;
use super::qpointf::QPointF;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  int QWheelEvent::x();
  fn _ZNK11QWheelEvent1xEv(qthis: *mut c_void) -> c_int;
  // proto:  QPoint QWheelEvent::angleDelta();
  fn _ZNK11QWheelEvent10angleDeltaEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QPoint QWheelEvent::pos();
  fn _ZNK11QWheelEvent3posEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QWheelEvent::globalY();
  fn _ZNK11QWheelEvent7globalYEv(qthis: *mut c_void) -> c_int;
  // proto:  const QPointF & QWheelEvent::posF();
  fn _ZNK11QWheelEvent4posFEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QWheelEvent::globalX();
  fn _ZNK11QWheelEvent7globalXEv(qthis: *mut c_void) -> c_int;
  // proto:  int QWheelEvent::y();
  fn _ZNK11QWheelEvent1yEv(qthis: *mut c_void) -> c_int;
  // proto:  void QWheelEvent::FreeQWheelEvent();
  fn _ZN11QWheelEventD0Ev(qthis: *mut c_void) ;
  // proto:  QPoint QWheelEvent::pixelDelta();
  fn _ZNK11QWheelEvent10pixelDeltaEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QWheelEvent::delta();
  fn _ZNK11QWheelEvent5deltaEv(qthis: *mut c_void) -> c_int;
  // proto:  QPoint QWheelEvent::globalPos();
  fn _ZNK11QWheelEvent9globalPosEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  const QPointF & QWheelEvent::globalPosF();
  fn _ZNK11QWheelEvent10globalPosFEv(qthis: *mut c_void) -> *mut c_void;
}

// body block begin
// class sizeof(QWheelEvent)=1
pub struct QWheelEvent {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QWheelEvent {
  pub fn x<T: QWheelEvent_x>(&mut self, value: T) -> i32 {
    return value.x(self);
    // return 1;
  }
}

pub trait QWheelEvent_x {
  fn x(self, rsthis: &mut QWheelEvent) -> i32;
}

// proto:  int QWheelEvent::x();
impl<'a> /*trait*/ QWheelEvent_x for () {
  fn x(self, rsthis: &mut QWheelEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWheelEvent1xEv()};
    let mut ret = unsafe {_ZNK11QWheelEvent1xEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QWheelEvent {
  pub fn angleDelta<T: QWheelEvent_angleDelta>(&mut self, value: T) -> QPoint {
    return value.angleDelta(self);
    // return 1;
  }
}

pub trait QWheelEvent_angleDelta {
  fn angleDelta(self, rsthis: &mut QWheelEvent) -> QPoint;
}

// proto:  QPoint QWheelEvent::angleDelta();
impl<'a> /*trait*/ QWheelEvent_angleDelta for () {
  fn angleDelta(self, rsthis: &mut QWheelEvent) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWheelEvent10angleDeltaEv()};
    let mut ret = unsafe {_ZNK11QWheelEvent10angleDeltaEv(rsthis.qclsinst)};
    let mut ret1 = QPoint{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QWheelEvent {
  pub fn pos<T: QWheelEvent_pos>(&mut self, value: T) -> QPoint {
    return value.pos(self);
    // return 1;
  }
}

pub trait QWheelEvent_pos {
  fn pos(self, rsthis: &mut QWheelEvent) -> QPoint;
}

// proto:  QPoint QWheelEvent::pos();
impl<'a> /*trait*/ QWheelEvent_pos for () {
  fn pos(self, rsthis: &mut QWheelEvent) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWheelEvent3posEv()};
    let mut ret = unsafe {_ZNK11QWheelEvent3posEv(rsthis.qclsinst)};
    let mut ret1 = QPoint{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QWheelEvent {
  pub fn globalY<T: QWheelEvent_globalY>(&mut self, value: T) -> i32 {
    return value.globalY(self);
    // return 1;
  }
}

pub trait QWheelEvent_globalY {
  fn globalY(self, rsthis: &mut QWheelEvent) -> i32;
}

// proto:  int QWheelEvent::globalY();
impl<'a> /*trait*/ QWheelEvent_globalY for () {
  fn globalY(self, rsthis: &mut QWheelEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWheelEvent7globalYEv()};
    let mut ret = unsafe {_ZNK11QWheelEvent7globalYEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QWheelEvent {
  pub fn posF<T: QWheelEvent_posF>(&mut self, value: T) -> QPointF {
    return value.posF(self);
    // return 1;
  }
}

pub trait QWheelEvent_posF {
  fn posF(self, rsthis: &mut QWheelEvent) -> QPointF;
}

// proto:  const QPointF & QWheelEvent::posF();
impl<'a> /*trait*/ QWheelEvent_posF for () {
  fn posF(self, rsthis: &mut QWheelEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWheelEvent4posFEv()};
    let mut ret = unsafe {_ZNK11QWheelEvent4posFEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QWheelEvent {
  pub fn globalX<T: QWheelEvent_globalX>(&mut self, value: T) -> i32 {
    return value.globalX(self);
    // return 1;
  }
}

pub trait QWheelEvent_globalX {
  fn globalX(self, rsthis: &mut QWheelEvent) -> i32;
}

// proto:  int QWheelEvent::globalX();
impl<'a> /*trait*/ QWheelEvent_globalX for () {
  fn globalX(self, rsthis: &mut QWheelEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWheelEvent7globalXEv()};
    let mut ret = unsafe {_ZNK11QWheelEvent7globalXEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QWheelEvent {
  pub fn y<T: QWheelEvent_y>(&mut self, value: T) -> i32 {
    return value.y(self);
    // return 1;
  }
}

pub trait QWheelEvent_y {
  fn y(self, rsthis: &mut QWheelEvent) -> i32;
}

// proto:  int QWheelEvent::y();
impl<'a> /*trait*/ QWheelEvent_y for () {
  fn y(self, rsthis: &mut QWheelEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWheelEvent1yEv()};
    let mut ret = unsafe {_ZNK11QWheelEvent1yEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QWheelEvent {
  pub fn FreeQWheelEvent<T: QWheelEvent_FreeQWheelEvent>(&mut self, value: T)  {
     value.FreeQWheelEvent(self);
    // return 1;
  }
}

pub trait QWheelEvent_FreeQWheelEvent {
  fn FreeQWheelEvent(self, rsthis: &mut QWheelEvent) ;
}

// proto:  void QWheelEvent::FreeQWheelEvent();
impl<'a> /*trait*/ QWheelEvent_FreeQWheelEvent for () {
  fn FreeQWheelEvent(self, rsthis: &mut QWheelEvent)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QWheelEventD0Ev()};
     unsafe {_ZN11QWheelEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QWheelEvent {
  pub fn pixelDelta<T: QWheelEvent_pixelDelta>(&mut self, value: T) -> QPoint {
    return value.pixelDelta(self);
    // return 1;
  }
}

pub trait QWheelEvent_pixelDelta {
  fn pixelDelta(self, rsthis: &mut QWheelEvent) -> QPoint;
}

// proto:  QPoint QWheelEvent::pixelDelta();
impl<'a> /*trait*/ QWheelEvent_pixelDelta for () {
  fn pixelDelta(self, rsthis: &mut QWheelEvent) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWheelEvent10pixelDeltaEv()};
    let mut ret = unsafe {_ZNK11QWheelEvent10pixelDeltaEv(rsthis.qclsinst)};
    let mut ret1 = QPoint{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QWheelEvent {
  pub fn delta<T: QWheelEvent_delta>(&mut self, value: T) -> i32 {
    return value.delta(self);
    // return 1;
  }
}

pub trait QWheelEvent_delta {
  fn delta(self, rsthis: &mut QWheelEvent) -> i32;
}

// proto:  int QWheelEvent::delta();
impl<'a> /*trait*/ QWheelEvent_delta for () {
  fn delta(self, rsthis: &mut QWheelEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWheelEvent5deltaEv()};
    let mut ret = unsafe {_ZNK11QWheelEvent5deltaEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QWheelEvent {
  pub fn globalPos<T: QWheelEvent_globalPos>(&mut self, value: T) -> QPoint {
    return value.globalPos(self);
    // return 1;
  }
}

pub trait QWheelEvent_globalPos {
  fn globalPos(self, rsthis: &mut QWheelEvent) -> QPoint;
}

// proto:  QPoint QWheelEvent::globalPos();
impl<'a> /*trait*/ QWheelEvent_globalPos for () {
  fn globalPos(self, rsthis: &mut QWheelEvent) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWheelEvent9globalPosEv()};
    let mut ret = unsafe {_ZNK11QWheelEvent9globalPosEv(rsthis.qclsinst)};
    let mut ret1 = QPoint{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QWheelEvent {
  pub fn globalPosF<T: QWheelEvent_globalPosF>(&mut self, value: T) -> QPointF {
    return value.globalPosF(self);
    // return 1;
  }
}

pub trait QWheelEvent_globalPosF {
  fn globalPosF(self, rsthis: &mut QWheelEvent) -> QPointF;
}

// proto:  const QPointF & QWheelEvent::globalPosF();
impl<'a> /*trait*/ QWheelEvent_globalPosF for () {
  fn globalPosF(self, rsthis: &mut QWheelEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWheelEvent10globalPosFEv()};
    let mut ret = unsafe {_ZNK11QWheelEvent10globalPosFEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

