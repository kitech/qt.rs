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
  fn _ZNK11QWheelEvent1xEv(qthis: *mut c_void) ;
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
  fn _ZNK11QWheelEvent1yEv(qthis: *mut c_void) ;
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
  pub fn x<RetType, T: QWheelEvent_x<RetType>>(&mut self, value: T) -> RetType {
    return value.x(self);
    // return 1;
  }
}

pub trait QWheelEvent_x<RetType> {
  fn x(self, rsthis: &mut QWheelEvent) -> RetType;
}

// proto:  int QWheelEvent::x();
impl<'a> /*trait*/ QWheelEvent_x<()> for () {
  fn x(self, rsthis: &mut QWheelEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWheelEvent1xEv()};
     unsafe {_ZNK11QWheelEvent1xEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QWheelEvent {
  pub fn angleDelta<RetType, T: QWheelEvent_angleDelta<RetType>>(&mut self, value: T) -> RetType {
    return value.angleDelta(self);
    // return 1;
  }
}

pub trait QWheelEvent_angleDelta<RetType> {
  fn angleDelta(self, rsthis: &mut QWheelEvent) -> RetType;
}

// proto:  QPoint QWheelEvent::angleDelta();
impl<'a> /*trait*/ QWheelEvent_angleDelta<QPoint> for () {
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
  pub fn pos<RetType, T: QWheelEvent_pos<RetType>>(&mut self, value: T) -> RetType {
    return value.pos(self);
    // return 1;
  }
}

pub trait QWheelEvent_pos<RetType> {
  fn pos(self, rsthis: &mut QWheelEvent) -> RetType;
}

// proto:  QPoint QWheelEvent::pos();
impl<'a> /*trait*/ QWheelEvent_pos<QPoint> for () {
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
  pub fn globalY<RetType, T: QWheelEvent_globalY<RetType>>(&mut self, value: T) -> RetType {
    return value.globalY(self);
    // return 1;
  }
}

pub trait QWheelEvent_globalY<RetType> {
  fn globalY(self, rsthis: &mut QWheelEvent) -> RetType;
}

// proto:  int QWheelEvent::globalY();
impl<'a> /*trait*/ QWheelEvent_globalY<i32> for () {
  fn globalY(self, rsthis: &mut QWheelEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWheelEvent7globalYEv()};
    let mut ret = unsafe {_ZNK11QWheelEvent7globalYEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QWheelEvent {
  pub fn posF<RetType, T: QWheelEvent_posF<RetType>>(&mut self, value: T) -> RetType {
    return value.posF(self);
    // return 1;
  }
}

pub trait QWheelEvent_posF<RetType> {
  fn posF(self, rsthis: &mut QWheelEvent) -> RetType;
}

// proto:  const QPointF & QWheelEvent::posF();
impl<'a> /*trait*/ QWheelEvent_posF<QPointF> for () {
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
  pub fn globalX<RetType, T: QWheelEvent_globalX<RetType>>(&mut self, value: T) -> RetType {
    return value.globalX(self);
    // return 1;
  }
}

pub trait QWheelEvent_globalX<RetType> {
  fn globalX(self, rsthis: &mut QWheelEvent) -> RetType;
}

// proto:  int QWheelEvent::globalX();
impl<'a> /*trait*/ QWheelEvent_globalX<i32> for () {
  fn globalX(self, rsthis: &mut QWheelEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWheelEvent7globalXEv()};
    let mut ret = unsafe {_ZNK11QWheelEvent7globalXEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QWheelEvent {
  pub fn y<RetType, T: QWheelEvent_y<RetType>>(&mut self, value: T) -> RetType {
    return value.y(self);
    // return 1;
  }
}

pub trait QWheelEvent_y<RetType> {
  fn y(self, rsthis: &mut QWheelEvent) -> RetType;
}

// proto:  int QWheelEvent::y();
impl<'a> /*trait*/ QWheelEvent_y<()> for () {
  fn y(self, rsthis: &mut QWheelEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWheelEvent1yEv()};
     unsafe {_ZNK11QWheelEvent1yEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QWheelEvent {
  pub fn FreeQWheelEvent<RetType, T: QWheelEvent_FreeQWheelEvent<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQWheelEvent(self);
    // return 1;
  }
}

pub trait QWheelEvent_FreeQWheelEvent<RetType> {
  fn FreeQWheelEvent(self, rsthis: &mut QWheelEvent) -> RetType;
}

// proto:  void QWheelEvent::FreeQWheelEvent();
impl<'a> /*trait*/ QWheelEvent_FreeQWheelEvent<()> for () {
  fn FreeQWheelEvent(self, rsthis: &mut QWheelEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QWheelEventD0Ev()};
     unsafe {_ZN11QWheelEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QWheelEvent {
  pub fn pixelDelta<RetType, T: QWheelEvent_pixelDelta<RetType>>(&mut self, value: T) -> RetType {
    return value.pixelDelta(self);
    // return 1;
  }
}

pub trait QWheelEvent_pixelDelta<RetType> {
  fn pixelDelta(self, rsthis: &mut QWheelEvent) -> RetType;
}

// proto:  QPoint QWheelEvent::pixelDelta();
impl<'a> /*trait*/ QWheelEvent_pixelDelta<QPoint> for () {
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
  pub fn delta<RetType, T: QWheelEvent_delta<RetType>>(&mut self, value: T) -> RetType {
    return value.delta(self);
    // return 1;
  }
}

pub trait QWheelEvent_delta<RetType> {
  fn delta(self, rsthis: &mut QWheelEvent) -> RetType;
}

// proto:  int QWheelEvent::delta();
impl<'a> /*trait*/ QWheelEvent_delta<i32> for () {
  fn delta(self, rsthis: &mut QWheelEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWheelEvent5deltaEv()};
    let mut ret = unsafe {_ZNK11QWheelEvent5deltaEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QWheelEvent {
  pub fn globalPos<RetType, T: QWheelEvent_globalPos<RetType>>(&mut self, value: T) -> RetType {
    return value.globalPos(self);
    // return 1;
  }
}

pub trait QWheelEvent_globalPos<RetType> {
  fn globalPos(self, rsthis: &mut QWheelEvent) -> RetType;
}

// proto:  QPoint QWheelEvent::globalPos();
impl<'a> /*trait*/ QWheelEvent_globalPos<QPoint> for () {
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
  pub fn globalPosF<RetType, T: QWheelEvent_globalPosF<RetType>>(&mut self, value: T) -> RetType {
    return value.globalPosF(self);
    // return 1;
  }
}

pub trait QWheelEvent_globalPosF<RetType> {
  fn globalPosF(self, rsthis: &mut QWheelEvent) -> RetType;
}

// proto:  const QPointF & QWheelEvent::globalPosF();
impl<'a> /*trait*/ QWheelEvent_globalPosF<QPointF> for () {
  fn globalPosF(self, rsthis: &mut QWheelEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWheelEvent10globalPosFEv()};
    let mut ret = unsafe {_ZNK11QWheelEvent10globalPosFEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

