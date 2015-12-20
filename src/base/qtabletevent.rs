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
  fn _ZNK12QTabletEvent1xEv(qthis: *mut c_void);
  // proto:  int QTabletEvent::xTilt();
  fn _ZNK12QTabletEvent5xTiltEv(qthis: *mut c_void) -> c_int;
  // proto:  qint64 QTabletEvent::uniqueId();
  fn _ZNK12QTabletEvent8uniqueIdEv(qthis: *mut c_void) -> c_longlong;
  // proto:  const QPointF & QTabletEvent::globalPosF();
  fn _ZNK12QTabletEvent10globalPosFEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QTabletEvent::z();
  fn _ZNK12QTabletEvent1zEv(qthis: *mut c_void) -> c_int;
  // proto:  int QTabletEvent::y();
  fn _ZNK12QTabletEvent1yEv(qthis: *mut c_void);
  // proto:  QPoint QTabletEvent::pos();
  fn _ZNK12QTabletEvent3posEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  qreal QTabletEvent::rotation();
  fn _ZNK12QTabletEvent8rotationEv(qthis: *mut c_void) -> c_double;
  // proto:  QPoint QTabletEvent::globalPos();
  fn _ZNK12QTabletEvent9globalPosEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTabletEvent::~QTabletEvent();
  fn _ZN12QTabletEventD0Ev(qthis: *mut c_void);
  // proto:  qreal QTabletEvent::tangentialPressure();
  fn _ZNK12QTabletEvent18tangentialPressureEv(qthis: *mut c_void) -> c_double;
  // proto:  qreal QTabletEvent::hiResGlobalX();
  fn _ZNK12QTabletEvent12hiResGlobalXEv(qthis: *mut c_void) -> c_double;
  // proto:  int QTabletEvent::globalY();
  fn _ZNK12QTabletEvent7globalYEv(qthis: *mut c_void) -> c_int;
  // proto:  qreal QTabletEvent::hiResGlobalY();
  fn _ZNK12QTabletEvent12hiResGlobalYEv(qthis: *mut c_void) -> c_double;
  // proto:  int QTabletEvent::globalX();
  fn _ZNK12QTabletEvent7globalXEv(qthis: *mut c_void) -> c_int;
  // proto:  const QPointF & QTabletEvent::posF();
  fn _ZNK12QTabletEvent4posFEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  qreal QTabletEvent::pressure();
  fn _ZNK12QTabletEvent8pressureEv(qthis: *mut c_void) -> c_double;
  // proto:  int QTabletEvent::yTilt();
  fn _ZNK12QTabletEvent5yTiltEv(qthis: *mut c_void) -> c_int;
}

// body block begin
// class sizeof(QTabletEvent)=1
pub struct QTabletEvent {
  pub qclsinst: *mut c_void,
}

  // proto:  int QTabletEvent::x();
impl /*struct*/ QTabletEvent {
  pub fn x<RetType, T: QTabletEvent_x<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.x(self);
    // return 1;
  }
}

pub trait QTabletEvent_x<RetType> {
  fn x(self , rsthis: &mut QTabletEvent) -> RetType;
}

  // proto:  int QTabletEvent::x();
impl<'a> /*trait*/ QTabletEvent_x<()> for () {
  fn x(self , rsthis: &mut QTabletEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTabletEvent1xEv()};
     unsafe {_ZNK12QTabletEvent1xEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QTabletEvent::xTilt();
impl /*struct*/ QTabletEvent {
  pub fn xTilt<RetType, T: QTabletEvent_xTilt<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.xTilt(self);
    // return 1;
  }
}

pub trait QTabletEvent_xTilt<RetType> {
  fn xTilt(self , rsthis: &mut QTabletEvent) -> RetType;
}

  // proto:  int QTabletEvent::xTilt();
impl<'a> /*trait*/ QTabletEvent_xTilt<i32> for () {
  fn xTilt(self , rsthis: &mut QTabletEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTabletEvent5xTiltEv()};
    let mut ret = unsafe {_ZNK12QTabletEvent5xTiltEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  qint64 QTabletEvent::uniqueId();
impl /*struct*/ QTabletEvent {
  pub fn uniqueId<RetType, T: QTabletEvent_uniqueId<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.uniqueId(self);
    // return 1;
  }
}

pub trait QTabletEvent_uniqueId<RetType> {
  fn uniqueId(self , rsthis: &mut QTabletEvent) -> RetType;
}

  // proto:  qint64 QTabletEvent::uniqueId();
impl<'a> /*trait*/ QTabletEvent_uniqueId<i64> for () {
  fn uniqueId(self , rsthis: &mut QTabletEvent) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTabletEvent8uniqueIdEv()};
    let mut ret = unsafe {_ZNK12QTabletEvent8uniqueIdEv(rsthis.qclsinst)};
    return ret as i64;
    // return 1;
  }
}

  // proto:  const QPointF & QTabletEvent::globalPosF();
impl /*struct*/ QTabletEvent {
  pub fn globalPosF<RetType, T: QTabletEvent_globalPosF<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.globalPosF(self);
    // return 1;
  }
}

pub trait QTabletEvent_globalPosF<RetType> {
  fn globalPosF(self , rsthis: &mut QTabletEvent) -> RetType;
}

  // proto:  const QPointF & QTabletEvent::globalPosF();
impl<'a> /*trait*/ QTabletEvent_globalPosF<QPointF> for () {
  fn globalPosF(self , rsthis: &mut QTabletEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTabletEvent10globalPosFEv()};
    let mut ret = unsafe {_ZNK12QTabletEvent10globalPosFEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  int QTabletEvent::z();
impl /*struct*/ QTabletEvent {
  pub fn z<RetType, T: QTabletEvent_z<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.z(self);
    // return 1;
  }
}

pub trait QTabletEvent_z<RetType> {
  fn z(self , rsthis: &mut QTabletEvent) -> RetType;
}

  // proto:  int QTabletEvent::z();
impl<'a> /*trait*/ QTabletEvent_z<i32> for () {
  fn z(self , rsthis: &mut QTabletEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTabletEvent1zEv()};
    let mut ret = unsafe {_ZNK12QTabletEvent1zEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QTabletEvent::y();
impl /*struct*/ QTabletEvent {
  pub fn y<RetType, T: QTabletEvent_y<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.y(self);
    // return 1;
  }
}

pub trait QTabletEvent_y<RetType> {
  fn y(self , rsthis: &mut QTabletEvent) -> RetType;
}

  // proto:  int QTabletEvent::y();
impl<'a> /*trait*/ QTabletEvent_y<()> for () {
  fn y(self , rsthis: &mut QTabletEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTabletEvent1yEv()};
     unsafe {_ZNK12QTabletEvent1yEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QPoint QTabletEvent::pos();
impl /*struct*/ QTabletEvent {
  pub fn pos<RetType, T: QTabletEvent_pos<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.pos(self);
    // return 1;
  }
}

pub trait QTabletEvent_pos<RetType> {
  fn pos(self , rsthis: &mut QTabletEvent) -> RetType;
}

  // proto:  QPoint QTabletEvent::pos();
impl<'a> /*trait*/ QTabletEvent_pos<QPoint> for () {
  fn pos(self , rsthis: &mut QTabletEvent) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTabletEvent3posEv()};
    let mut ret = unsafe {_ZNK12QTabletEvent3posEv(rsthis.qclsinst)};
    let mut ret1 = QPoint{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  qreal QTabletEvent::rotation();
impl /*struct*/ QTabletEvent {
  pub fn rotation<RetType, T: QTabletEvent_rotation<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.rotation(self);
    // return 1;
  }
}

pub trait QTabletEvent_rotation<RetType> {
  fn rotation(self , rsthis: &mut QTabletEvent) -> RetType;
}

  // proto:  qreal QTabletEvent::rotation();
impl<'a> /*trait*/ QTabletEvent_rotation<f64> for () {
  fn rotation(self , rsthis: &mut QTabletEvent) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTabletEvent8rotationEv()};
    let mut ret = unsafe {_ZNK12QTabletEvent8rotationEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  QPoint QTabletEvent::globalPos();
impl /*struct*/ QTabletEvent {
  pub fn globalPos<RetType, T: QTabletEvent_globalPos<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.globalPos(self);
    // return 1;
  }
}

pub trait QTabletEvent_globalPos<RetType> {
  fn globalPos(self , rsthis: &mut QTabletEvent) -> RetType;
}

  // proto:  QPoint QTabletEvent::globalPos();
impl<'a> /*trait*/ QTabletEvent_globalPos<QPoint> for () {
  fn globalPos(self , rsthis: &mut QTabletEvent) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTabletEvent9globalPosEv()};
    let mut ret = unsafe {_ZNK12QTabletEvent9globalPosEv(rsthis.qclsinst)};
    let mut ret1 = QPoint{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QTabletEvent::~QTabletEvent();
impl /*struct*/ QTabletEvent {
  pub fn FreeQTabletEvent<RetType, T: QTabletEvent_FreeQTabletEvent<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQTabletEvent(self);
    // return 1;
  }
}

pub trait QTabletEvent_FreeQTabletEvent<RetType> {
  fn FreeQTabletEvent(self , rsthis: &mut QTabletEvent) -> RetType;
}

  // proto:  void QTabletEvent::~QTabletEvent();
impl<'a> /*trait*/ QTabletEvent_FreeQTabletEvent<()> for () {
  fn FreeQTabletEvent(self , rsthis: &mut QTabletEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTabletEventD0Ev()};
     unsafe {_ZN12QTabletEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  qreal QTabletEvent::tangentialPressure();
impl /*struct*/ QTabletEvent {
  pub fn tangentialPressure<RetType, T: QTabletEvent_tangentialPressure<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.tangentialPressure(self);
    // return 1;
  }
}

pub trait QTabletEvent_tangentialPressure<RetType> {
  fn tangentialPressure(self , rsthis: &mut QTabletEvent) -> RetType;
}

  // proto:  qreal QTabletEvent::tangentialPressure();
impl<'a> /*trait*/ QTabletEvent_tangentialPressure<f64> for () {
  fn tangentialPressure(self , rsthis: &mut QTabletEvent) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTabletEvent18tangentialPressureEv()};
    let mut ret = unsafe {_ZNK12QTabletEvent18tangentialPressureEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  qreal QTabletEvent::hiResGlobalX();
impl /*struct*/ QTabletEvent {
  pub fn hiResGlobalX<RetType, T: QTabletEvent_hiResGlobalX<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.hiResGlobalX(self);
    // return 1;
  }
}

pub trait QTabletEvent_hiResGlobalX<RetType> {
  fn hiResGlobalX(self , rsthis: &mut QTabletEvent) -> RetType;
}

  // proto:  qreal QTabletEvent::hiResGlobalX();
impl<'a> /*trait*/ QTabletEvent_hiResGlobalX<f64> for () {
  fn hiResGlobalX(self , rsthis: &mut QTabletEvent) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTabletEvent12hiResGlobalXEv()};
    let mut ret = unsafe {_ZNK12QTabletEvent12hiResGlobalXEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  int QTabletEvent::globalY();
impl /*struct*/ QTabletEvent {
  pub fn globalY<RetType, T: QTabletEvent_globalY<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.globalY(self);
    // return 1;
  }
}

pub trait QTabletEvent_globalY<RetType> {
  fn globalY(self , rsthis: &mut QTabletEvent) -> RetType;
}

  // proto:  int QTabletEvent::globalY();
impl<'a> /*trait*/ QTabletEvent_globalY<i32> for () {
  fn globalY(self , rsthis: &mut QTabletEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTabletEvent7globalYEv()};
    let mut ret = unsafe {_ZNK12QTabletEvent7globalYEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  qreal QTabletEvent::hiResGlobalY();
impl /*struct*/ QTabletEvent {
  pub fn hiResGlobalY<RetType, T: QTabletEvent_hiResGlobalY<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.hiResGlobalY(self);
    // return 1;
  }
}

pub trait QTabletEvent_hiResGlobalY<RetType> {
  fn hiResGlobalY(self , rsthis: &mut QTabletEvent) -> RetType;
}

  // proto:  qreal QTabletEvent::hiResGlobalY();
impl<'a> /*trait*/ QTabletEvent_hiResGlobalY<f64> for () {
  fn hiResGlobalY(self , rsthis: &mut QTabletEvent) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTabletEvent12hiResGlobalYEv()};
    let mut ret = unsafe {_ZNK12QTabletEvent12hiResGlobalYEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  int QTabletEvent::globalX();
impl /*struct*/ QTabletEvent {
  pub fn globalX<RetType, T: QTabletEvent_globalX<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.globalX(self);
    // return 1;
  }
}

pub trait QTabletEvent_globalX<RetType> {
  fn globalX(self , rsthis: &mut QTabletEvent) -> RetType;
}

  // proto:  int QTabletEvent::globalX();
impl<'a> /*trait*/ QTabletEvent_globalX<i32> for () {
  fn globalX(self , rsthis: &mut QTabletEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTabletEvent7globalXEv()};
    let mut ret = unsafe {_ZNK12QTabletEvent7globalXEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  const QPointF & QTabletEvent::posF();
impl /*struct*/ QTabletEvent {
  pub fn posF<RetType, T: QTabletEvent_posF<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.posF(self);
    // return 1;
  }
}

pub trait QTabletEvent_posF<RetType> {
  fn posF(self , rsthis: &mut QTabletEvent) -> RetType;
}

  // proto:  const QPointF & QTabletEvent::posF();
impl<'a> /*trait*/ QTabletEvent_posF<QPointF> for () {
  fn posF(self , rsthis: &mut QTabletEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTabletEvent4posFEv()};
    let mut ret = unsafe {_ZNK12QTabletEvent4posFEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  qreal QTabletEvent::pressure();
impl /*struct*/ QTabletEvent {
  pub fn pressure<RetType, T: QTabletEvent_pressure<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.pressure(self);
    // return 1;
  }
}

pub trait QTabletEvent_pressure<RetType> {
  fn pressure(self , rsthis: &mut QTabletEvent) -> RetType;
}

  // proto:  qreal QTabletEvent::pressure();
impl<'a> /*trait*/ QTabletEvent_pressure<f64> for () {
  fn pressure(self , rsthis: &mut QTabletEvent) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTabletEvent8pressureEv()};
    let mut ret = unsafe {_ZNK12QTabletEvent8pressureEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  int QTabletEvent::yTilt();
impl /*struct*/ QTabletEvent {
  pub fn yTilt<RetType, T: QTabletEvent_yTilt<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.yTilt(self);
    // return 1;
  }
}

pub trait QTabletEvent_yTilt<RetType> {
  fn yTilt(self , rsthis: &mut QTabletEvent) -> RetType;
}

  // proto:  int QTabletEvent::yTilt();
impl<'a> /*trait*/ QTabletEvent_yTilt<i32> for () {
  fn yTilt(self , rsthis: &mut QTabletEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTabletEvent5yTiltEv()};
    let mut ret = unsafe {_ZNK12QTabletEvent5yTiltEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

