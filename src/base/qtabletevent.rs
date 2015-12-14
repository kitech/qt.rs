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
  // proto:  int QTabletEvent::x();
  fn _ZNK12QTabletEvent1xEv(qthis: *mut c_void) -> c_int;
  // proto:  int QTabletEvent::xTilt();
  fn _ZNK12QTabletEvent5xTiltEv(qthis: *mut c_void) -> c_int;
  // proto:  long long QTabletEvent::uniqueId();
  fn _ZNK12QTabletEvent8uniqueIdEv(qthis: *mut c_void) -> c_longlong;
  // proto:  const QPointF & QTabletEvent::globalPosF();
  fn _ZNK12QTabletEvent10globalPosFEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QTabletEvent::z();
  fn _ZNK12QTabletEvent1zEv(qthis: *mut c_void) -> c_int;
  // proto:  int QTabletEvent::y();
  fn _ZNK12QTabletEvent1yEv(qthis: *mut c_void) -> c_int;
  // proto:  QPoint QTabletEvent::pos();
  fn _ZNK12QTabletEvent3posEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  double QTabletEvent::rotation();
  fn _ZNK12QTabletEvent8rotationEv(qthis: *mut c_void) -> c_double;
  // proto:  QPoint QTabletEvent::globalPos();
  fn _ZNK12QTabletEvent9globalPosEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTabletEvent::FreeQTabletEvent();
  fn _ZN12QTabletEventD0Ev(qthis: *mut c_void) ;
  // proto:  double QTabletEvent::tangentialPressure();
  fn _ZNK12QTabletEvent18tangentialPressureEv(qthis: *mut c_void) -> c_double;
  // proto:  double QTabletEvent::hiResGlobalX();
  fn _ZNK12QTabletEvent12hiResGlobalXEv(qthis: *mut c_void) -> c_double;
  // proto:  int QTabletEvent::globalY();
  fn _ZNK12QTabletEvent7globalYEv(qthis: *mut c_void) -> c_int;
  // proto:  double QTabletEvent::hiResGlobalY();
  fn _ZNK12QTabletEvent12hiResGlobalYEv(qthis: *mut c_void) -> c_double;
  // proto:  int QTabletEvent::globalX();
  fn _ZNK12QTabletEvent7globalXEv(qthis: *mut c_void) -> c_int;
  // proto:  const QPointF & QTabletEvent::posF();
  fn _ZNK12QTabletEvent4posFEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  double QTabletEvent::pressure();
  fn _ZNK12QTabletEvent8pressureEv(qthis: *mut c_void) -> c_double;
  // proto:  int QTabletEvent::yTilt();
  fn _ZNK12QTabletEvent5yTiltEv(qthis: *mut c_void) -> c_int;
}

// body block begin
// class sizeof(QTabletEvent)=1
pub struct QTabletEvent {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTabletEvent {
  pub fn x<T: QTabletEvent_x>(&mut self, value: T) -> i32 {
    return value.x(self);
    // return 1;
  }
}

pub trait QTabletEvent_x {
  fn x(self, rsthis: &mut QTabletEvent) -> i32;
}

// proto:  int QTabletEvent::x();
impl<'a> /*trait*/ QTabletEvent_x for () {
  fn x(self, rsthis: &mut QTabletEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTabletEvent1xEv()};
    let mut ret = unsafe {_ZNK12QTabletEvent1xEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTabletEvent {
  pub fn xTilt<T: QTabletEvent_xTilt>(&mut self, value: T) -> i32 {
    return value.xTilt(self);
    // return 1;
  }
}

pub trait QTabletEvent_xTilt {
  fn xTilt(self, rsthis: &mut QTabletEvent) -> i32;
}

// proto:  int QTabletEvent::xTilt();
impl<'a> /*trait*/ QTabletEvent_xTilt for () {
  fn xTilt(self, rsthis: &mut QTabletEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTabletEvent5xTiltEv()};
    let mut ret = unsafe {_ZNK12QTabletEvent5xTiltEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTabletEvent {
  pub fn uniqueId<T: QTabletEvent_uniqueId>(&mut self, value: T) -> i64 {
    return value.uniqueId(self);
    // return 1;
  }
}

pub trait QTabletEvent_uniqueId {
  fn uniqueId(self, rsthis: &mut QTabletEvent) -> i64;
}

// proto:  long long QTabletEvent::uniqueId();
impl<'a> /*trait*/ QTabletEvent_uniqueId for () {
  fn uniqueId(self, rsthis: &mut QTabletEvent) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTabletEvent8uniqueIdEv()};
    let mut ret = unsafe {_ZNK12QTabletEvent8uniqueIdEv(rsthis.qclsinst)};
    return ret as i64;
    // return 1;
  }
}

impl /*struct*/ QTabletEvent {
  pub fn globalPosF<T: QTabletEvent_globalPosF>(&mut self, value: T) -> QPointF {
    return value.globalPosF(self);
    // return 1;
  }
}

pub trait QTabletEvent_globalPosF {
  fn globalPosF(self, rsthis: &mut QTabletEvent) -> QPointF;
}

// proto:  const QPointF & QTabletEvent::globalPosF();
impl<'a> /*trait*/ QTabletEvent_globalPosF for () {
  fn globalPosF(self, rsthis: &mut QTabletEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTabletEvent10globalPosFEv()};
    let mut ret = unsafe {_ZNK12QTabletEvent10globalPosFEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTabletEvent {
  pub fn z<T: QTabletEvent_z>(&mut self, value: T) -> i32 {
    return value.z(self);
    // return 1;
  }
}

pub trait QTabletEvent_z {
  fn z(self, rsthis: &mut QTabletEvent) -> i32;
}

// proto:  int QTabletEvent::z();
impl<'a> /*trait*/ QTabletEvent_z for () {
  fn z(self, rsthis: &mut QTabletEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTabletEvent1zEv()};
    let mut ret = unsafe {_ZNK12QTabletEvent1zEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTabletEvent {
  pub fn y<T: QTabletEvent_y>(&mut self, value: T) -> i32 {
    return value.y(self);
    // return 1;
  }
}

pub trait QTabletEvent_y {
  fn y(self, rsthis: &mut QTabletEvent) -> i32;
}

// proto:  int QTabletEvent::y();
impl<'a> /*trait*/ QTabletEvent_y for () {
  fn y(self, rsthis: &mut QTabletEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTabletEvent1yEv()};
    let mut ret = unsafe {_ZNK12QTabletEvent1yEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTabletEvent {
  pub fn pos<T: QTabletEvent_pos>(&mut self, value: T) -> QPoint {
    return value.pos(self);
    // return 1;
  }
}

pub trait QTabletEvent_pos {
  fn pos(self, rsthis: &mut QTabletEvent) -> QPoint;
}

// proto:  QPoint QTabletEvent::pos();
impl<'a> /*trait*/ QTabletEvent_pos for () {
  fn pos(self, rsthis: &mut QTabletEvent) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTabletEvent3posEv()};
    let mut ret = unsafe {_ZNK12QTabletEvent3posEv(rsthis.qclsinst)};
    let mut ret1 = QPoint{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTabletEvent {
  pub fn rotation<T: QTabletEvent_rotation>(&mut self, value: T) -> f64 {
    return value.rotation(self);
    // return 1;
  }
}

pub trait QTabletEvent_rotation {
  fn rotation(self, rsthis: &mut QTabletEvent) -> f64;
}

// proto:  double QTabletEvent::rotation();
impl<'a> /*trait*/ QTabletEvent_rotation for () {
  fn rotation(self, rsthis: &mut QTabletEvent) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTabletEvent8rotationEv()};
    let mut ret = unsafe {_ZNK12QTabletEvent8rotationEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QTabletEvent {
  pub fn globalPos<T: QTabletEvent_globalPos>(&mut self, value: T) -> QPoint {
    return value.globalPos(self);
    // return 1;
  }
}

pub trait QTabletEvent_globalPos {
  fn globalPos(self, rsthis: &mut QTabletEvent) -> QPoint;
}

// proto:  QPoint QTabletEvent::globalPos();
impl<'a> /*trait*/ QTabletEvent_globalPos for () {
  fn globalPos(self, rsthis: &mut QTabletEvent) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTabletEvent9globalPosEv()};
    let mut ret = unsafe {_ZNK12QTabletEvent9globalPosEv(rsthis.qclsinst)};
    let mut ret1 = QPoint{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTabletEvent {
  pub fn FreeQTabletEvent<T: QTabletEvent_FreeQTabletEvent>(&mut self, value: T)  {
     value.FreeQTabletEvent(self);
    // return 1;
  }
}

pub trait QTabletEvent_FreeQTabletEvent {
  fn FreeQTabletEvent(self, rsthis: &mut QTabletEvent) ;
}

// proto:  void QTabletEvent::FreeQTabletEvent();
impl<'a> /*trait*/ QTabletEvent_FreeQTabletEvent for () {
  fn FreeQTabletEvent(self, rsthis: &mut QTabletEvent)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTabletEventD0Ev()};
     unsafe {_ZN12QTabletEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTabletEvent {
  pub fn tangentialPressure<T: QTabletEvent_tangentialPressure>(&mut self, value: T) -> f64 {
    return value.tangentialPressure(self);
    // return 1;
  }
}

pub trait QTabletEvent_tangentialPressure {
  fn tangentialPressure(self, rsthis: &mut QTabletEvent) -> f64;
}

// proto:  double QTabletEvent::tangentialPressure();
impl<'a> /*trait*/ QTabletEvent_tangentialPressure for () {
  fn tangentialPressure(self, rsthis: &mut QTabletEvent) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTabletEvent18tangentialPressureEv()};
    let mut ret = unsafe {_ZNK12QTabletEvent18tangentialPressureEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QTabletEvent {
  pub fn hiResGlobalX<T: QTabletEvent_hiResGlobalX>(&mut self, value: T) -> f64 {
    return value.hiResGlobalX(self);
    // return 1;
  }
}

pub trait QTabletEvent_hiResGlobalX {
  fn hiResGlobalX(self, rsthis: &mut QTabletEvent) -> f64;
}

// proto:  double QTabletEvent::hiResGlobalX();
impl<'a> /*trait*/ QTabletEvent_hiResGlobalX for () {
  fn hiResGlobalX(self, rsthis: &mut QTabletEvent) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTabletEvent12hiResGlobalXEv()};
    let mut ret = unsafe {_ZNK12QTabletEvent12hiResGlobalXEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QTabletEvent {
  pub fn globalY<T: QTabletEvent_globalY>(&mut self, value: T) -> i32 {
    return value.globalY(self);
    // return 1;
  }
}

pub trait QTabletEvent_globalY {
  fn globalY(self, rsthis: &mut QTabletEvent) -> i32;
}

// proto:  int QTabletEvent::globalY();
impl<'a> /*trait*/ QTabletEvent_globalY for () {
  fn globalY(self, rsthis: &mut QTabletEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTabletEvent7globalYEv()};
    let mut ret = unsafe {_ZNK12QTabletEvent7globalYEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTabletEvent {
  pub fn hiResGlobalY<T: QTabletEvent_hiResGlobalY>(&mut self, value: T) -> f64 {
    return value.hiResGlobalY(self);
    // return 1;
  }
}

pub trait QTabletEvent_hiResGlobalY {
  fn hiResGlobalY(self, rsthis: &mut QTabletEvent) -> f64;
}

// proto:  double QTabletEvent::hiResGlobalY();
impl<'a> /*trait*/ QTabletEvent_hiResGlobalY for () {
  fn hiResGlobalY(self, rsthis: &mut QTabletEvent) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTabletEvent12hiResGlobalYEv()};
    let mut ret = unsafe {_ZNK12QTabletEvent12hiResGlobalYEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QTabletEvent {
  pub fn globalX<T: QTabletEvent_globalX>(&mut self, value: T) -> i32 {
    return value.globalX(self);
    // return 1;
  }
}

pub trait QTabletEvent_globalX {
  fn globalX(self, rsthis: &mut QTabletEvent) -> i32;
}

// proto:  int QTabletEvent::globalX();
impl<'a> /*trait*/ QTabletEvent_globalX for () {
  fn globalX(self, rsthis: &mut QTabletEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTabletEvent7globalXEv()};
    let mut ret = unsafe {_ZNK12QTabletEvent7globalXEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTabletEvent {
  pub fn posF<T: QTabletEvent_posF>(&mut self, value: T) -> QPointF {
    return value.posF(self);
    // return 1;
  }
}

pub trait QTabletEvent_posF {
  fn posF(self, rsthis: &mut QTabletEvent) -> QPointF;
}

// proto:  const QPointF & QTabletEvent::posF();
impl<'a> /*trait*/ QTabletEvent_posF for () {
  fn posF(self, rsthis: &mut QTabletEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTabletEvent4posFEv()};
    let mut ret = unsafe {_ZNK12QTabletEvent4posFEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTabletEvent {
  pub fn pressure<T: QTabletEvent_pressure>(&mut self, value: T) -> f64 {
    return value.pressure(self);
    // return 1;
  }
}

pub trait QTabletEvent_pressure {
  fn pressure(self, rsthis: &mut QTabletEvent) -> f64;
}

// proto:  double QTabletEvent::pressure();
impl<'a> /*trait*/ QTabletEvent_pressure for () {
  fn pressure(self, rsthis: &mut QTabletEvent) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTabletEvent8pressureEv()};
    let mut ret = unsafe {_ZNK12QTabletEvent8pressureEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QTabletEvent {
  pub fn yTilt<T: QTabletEvent_yTilt>(&mut self, value: T) -> i32 {
    return value.yTilt(self);
    // return 1;
  }
}

pub trait QTabletEvent_yTilt {
  fn yTilt(self, rsthis: &mut QTabletEvent) -> i32;
}

// proto:  int QTabletEvent::yTilt();
impl<'a> /*trait*/ QTabletEvent_yTilt for () {
  fn yTilt(self, rsthis: &mut QTabletEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTabletEvent5yTiltEv()};
    let mut ret = unsafe {_ZNK12QTabletEvent5yTiltEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

