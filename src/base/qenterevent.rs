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
  // proto:  int QEnterEvent::y();
  fn _ZNK11QEnterEvent1yEv(qthis: *mut c_void) ;
  // proto:  QPoint QEnterEvent::pos();
  fn _ZNK11QEnterEvent3posEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QEnterEvent::FreeQEnterEvent();
  fn _ZN11QEnterEventD0Ev(qthis: *mut c_void) ;
  // proto:  const QPointF & QEnterEvent::screenPos();
  fn _ZNK11QEnterEvent9screenPosEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  const QPointF & QEnterEvent::localPos();
  fn _ZNK11QEnterEvent8localPosEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  const QPointF & QEnterEvent::windowPos();
  fn _ZNK11QEnterEvent9windowPosEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QEnterEvent::globalX();
  fn _ZNK11QEnterEvent7globalXEv(qthis: *mut c_void) -> c_int;
  // proto:  int QEnterEvent::x();
  fn _ZNK11QEnterEvent1xEv(qthis: *mut c_void) ;
  // proto:  QPoint QEnterEvent::globalPos();
  fn _ZNK11QEnterEvent9globalPosEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QEnterEvent::globalY();
  fn _ZNK11QEnterEvent7globalYEv(qthis: *mut c_void) -> c_int;
  // proto:  void QEnterEvent::NewQEnterEvent(const QPointF & localPos, const QPointF & windowPos, const QPointF & screenPos);
  fn _ZN11QEnterEventC1ERK7QPointFS2_S2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) ;
}

// body block begin
// class sizeof(QEnterEvent)=72
pub struct QEnterEvent {
  pub qclsinst: *mut c_void,
}

// proto:  int QEnterEvent::y();
impl /*struct*/ QEnterEvent {
  pub fn y<RetType, T: QEnterEvent_y<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.y(self);
    // return 1;
  }
}

pub trait QEnterEvent_y<RetType> {
  fn y(self , rsthis: &mut QEnterEvent) -> RetType;
}

// proto:  int QEnterEvent::y();
impl<'a> /*trait*/ QEnterEvent_y<()> for () {
  fn y(self , rsthis: &mut QEnterEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 72)};
    // unsafe{_ZNK11QEnterEvent1yEv()};
     unsafe {_ZNK11QEnterEvent1yEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  QPoint QEnterEvent::pos();
impl /*struct*/ QEnterEvent {
  pub fn pos<RetType, T: QEnterEvent_pos<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.pos(self);
    // return 1;
  }
}

pub trait QEnterEvent_pos<RetType> {
  fn pos(self , rsthis: &mut QEnterEvent) -> RetType;
}

// proto:  QPoint QEnterEvent::pos();
impl<'a> /*trait*/ QEnterEvent_pos<QPoint> for () {
  fn pos(self , rsthis: &mut QEnterEvent) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 72)};
    // unsafe{_ZNK11QEnterEvent3posEv()};
    let mut ret = unsafe {_ZNK11QEnterEvent3posEv(rsthis.qclsinst)};
    let mut ret1 = QPoint{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QEnterEvent::FreeQEnterEvent();
impl /*struct*/ QEnterEvent {
  pub fn FreeQEnterEvent<RetType, T: QEnterEvent_FreeQEnterEvent<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.FreeQEnterEvent(self);
    // return 1;
  }
}

pub trait QEnterEvent_FreeQEnterEvent<RetType> {
  fn FreeQEnterEvent(self , rsthis: &mut QEnterEvent) -> RetType;
}

// proto:  void QEnterEvent::FreeQEnterEvent();
impl<'a> /*trait*/ QEnterEvent_FreeQEnterEvent<()> for () {
  fn FreeQEnterEvent(self , rsthis: &mut QEnterEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 72)};
    // unsafe{_ZN11QEnterEventD0Ev()};
     unsafe {_ZN11QEnterEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  const QPointF & QEnterEvent::screenPos();
impl /*struct*/ QEnterEvent {
  pub fn screenPos<RetType, T: QEnterEvent_screenPos<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.screenPos(self);
    // return 1;
  }
}

pub trait QEnterEvent_screenPos<RetType> {
  fn screenPos(self , rsthis: &mut QEnterEvent) -> RetType;
}

// proto:  const QPointF & QEnterEvent::screenPos();
impl<'a> /*trait*/ QEnterEvent_screenPos<QPointF> for () {
  fn screenPos(self , rsthis: &mut QEnterEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 72)};
    // unsafe{_ZNK11QEnterEvent9screenPosEv()};
    let mut ret = unsafe {_ZNK11QEnterEvent9screenPosEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  const QPointF & QEnterEvent::localPos();
impl /*struct*/ QEnterEvent {
  pub fn localPos<RetType, T: QEnterEvent_localPos<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.localPos(self);
    // return 1;
  }
}

pub trait QEnterEvent_localPos<RetType> {
  fn localPos(self , rsthis: &mut QEnterEvent) -> RetType;
}

// proto:  const QPointF & QEnterEvent::localPos();
impl<'a> /*trait*/ QEnterEvent_localPos<QPointF> for () {
  fn localPos(self , rsthis: &mut QEnterEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 72)};
    // unsafe{_ZNK11QEnterEvent8localPosEv()};
    let mut ret = unsafe {_ZNK11QEnterEvent8localPosEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  const QPointF & QEnterEvent::windowPos();
impl /*struct*/ QEnterEvent {
  pub fn windowPos<RetType, T: QEnterEvent_windowPos<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.windowPos(self);
    // return 1;
  }
}

pub trait QEnterEvent_windowPos<RetType> {
  fn windowPos(self , rsthis: &mut QEnterEvent) -> RetType;
}

// proto:  const QPointF & QEnterEvent::windowPos();
impl<'a> /*trait*/ QEnterEvent_windowPos<QPointF> for () {
  fn windowPos(self , rsthis: &mut QEnterEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 72)};
    // unsafe{_ZNK11QEnterEvent9windowPosEv()};
    let mut ret = unsafe {_ZNK11QEnterEvent9windowPosEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  int QEnterEvent::globalX();
impl /*struct*/ QEnterEvent {
  pub fn globalX<RetType, T: QEnterEvent_globalX<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.globalX(self);
    // return 1;
  }
}

pub trait QEnterEvent_globalX<RetType> {
  fn globalX(self , rsthis: &mut QEnterEvent) -> RetType;
}

// proto:  int QEnterEvent::globalX();
impl<'a> /*trait*/ QEnterEvent_globalX<i32> for () {
  fn globalX(self , rsthis: &mut QEnterEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 72)};
    // unsafe{_ZNK11QEnterEvent7globalXEv()};
    let mut ret = unsafe {_ZNK11QEnterEvent7globalXEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto:  int QEnterEvent::x();
impl /*struct*/ QEnterEvent {
  pub fn x<RetType, T: QEnterEvent_x<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.x(self);
    // return 1;
  }
}

pub trait QEnterEvent_x<RetType> {
  fn x(self , rsthis: &mut QEnterEvent) -> RetType;
}

// proto:  int QEnterEvent::x();
impl<'a> /*trait*/ QEnterEvent_x<()> for () {
  fn x(self , rsthis: &mut QEnterEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 72)};
    // unsafe{_ZNK11QEnterEvent1xEv()};
     unsafe {_ZNK11QEnterEvent1xEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  QPoint QEnterEvent::globalPos();
impl /*struct*/ QEnterEvent {
  pub fn globalPos<RetType, T: QEnterEvent_globalPos<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.globalPos(self);
    // return 1;
  }
}

pub trait QEnterEvent_globalPos<RetType> {
  fn globalPos(self , rsthis: &mut QEnterEvent) -> RetType;
}

// proto:  QPoint QEnterEvent::globalPos();
impl<'a> /*trait*/ QEnterEvent_globalPos<QPoint> for () {
  fn globalPos(self , rsthis: &mut QEnterEvent) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 72)};
    // unsafe{_ZNK11QEnterEvent9globalPosEv()};
    let mut ret = unsafe {_ZNK11QEnterEvent9globalPosEv(rsthis.qclsinst)};
    let mut ret1 = QPoint{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  int QEnterEvent::globalY();
impl /*struct*/ QEnterEvent {
  pub fn globalY<RetType, T: QEnterEvent_globalY<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.globalY(self);
    // return 1;
  }
}

pub trait QEnterEvent_globalY<RetType> {
  fn globalY(self , rsthis: &mut QEnterEvent) -> RetType;
}

// proto:  int QEnterEvent::globalY();
impl<'a> /*trait*/ QEnterEvent_globalY<i32> for () {
  fn globalY(self , rsthis: &mut QEnterEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 72)};
    // unsafe{_ZNK11QEnterEvent7globalYEv()};
    let mut ret = unsafe {_ZNK11QEnterEvent7globalYEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QEnterEvent {
  pub fn NewQEnterEvent<T: QEnterEvent_NewQEnterEvent>(value: T) -> QEnterEvent {
    let rsthis = value.NewQEnterEvent();
    return rsthis;
    // return 1;
  }
}

pub trait QEnterEvent_NewQEnterEvent {
  fn NewQEnterEvent(self) -> QEnterEvent;
}

// proto: void QEnterEvent::NewQEnterEvent(const QPointF & localPos, const QPointF & windowPos, const QPointF & screenPos);
impl<'a> /*trait*/ QEnterEvent_NewQEnterEvent for (&'a  QPointF, &'a  QPointF, &'a  QPointF) {
  fn NewQEnterEvent(self) -> QEnterEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 72)};
    // unsafe{_ZN11QEnterEventC1ERK7QPointFS2_S2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    unsafe {_ZN11QEnterEventC1ERK7QPointFS2_S2_(qthis, arg0, arg1, arg2)};
    let rsthis = QEnterEvent{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

