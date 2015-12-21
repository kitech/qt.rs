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
  // proto:  QPoint QMouseEvent::globalPos();
  fn _ZNK11QMouseEvent9globalPosEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QMouseEvent::y();
  fn _ZNK11QMouseEvent1yEv(qthis: *mut c_void);
  // proto:  const QPointF & QMouseEvent::screenPos();
  fn _ZNK11QMouseEvent9screenPosEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QMouseEvent::x();
  fn _ZNK11QMouseEvent1xEv(qthis: *mut c_void);
  // proto:  const QPointF & QMouseEvent::localPos();
  fn _ZNK11QMouseEvent8localPosEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QMouseEvent::globalX();
  fn _ZNK11QMouseEvent7globalXEv(qthis: *mut c_void) -> c_int;
  // proto:  const QPointF & QMouseEvent::windowPos();
  fn _ZNK11QMouseEvent9windowPosEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QMouseEvent::~QMouseEvent();
  fn _ZN11QMouseEventD0Ev(qthis: *mut c_void);
  // proto:  int QMouseEvent::globalY();
  fn _ZNK11QMouseEvent7globalYEv(qthis: *mut c_void) -> c_int;
  // proto:  QPoint QMouseEvent::pos();
  fn _ZNK11QMouseEvent3posEv(qthis: *mut c_void) -> *mut c_void;
}

// body block begin
// class sizeof(QMouseEvent)=1
pub struct QMouseEvent {
  pub qclsinst: *mut c_void,
}

  // proto:  QPoint QMouseEvent::globalPos();
impl /*struct*/ QMouseEvent {
  pub fn globalPos<RetType, T: QMouseEvent_globalPos<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.globalPos(self);
    // return 1;
  }
}

pub trait QMouseEvent_globalPos<RetType> {
  fn globalPos(self , rsthis: &mut QMouseEvent) -> RetType;
}

  // proto:  QPoint QMouseEvent::globalPos();
impl<'a> /*trait*/ QMouseEvent_globalPos<QPoint> for () {
  fn globalPos(self , rsthis: &mut QMouseEvent) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMouseEvent9globalPosEv()};
    let mut ret = unsafe {_ZNK11QMouseEvent9globalPosEv(rsthis.qclsinst)};
    let mut ret1 = QPoint{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  int QMouseEvent::y();
impl /*struct*/ QMouseEvent {
  pub fn y<RetType, T: QMouseEvent_y<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.y(self);
    // return 1;
  }
}

pub trait QMouseEvent_y<RetType> {
  fn y(self , rsthis: &mut QMouseEvent) -> RetType;
}

  // proto:  int QMouseEvent::y();
impl<'a> /*trait*/ QMouseEvent_y<()> for () {
  fn y(self , rsthis: &mut QMouseEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMouseEvent1yEv()};
     unsafe {_ZNK11QMouseEvent1yEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  const QPointF & QMouseEvent::screenPos();
impl /*struct*/ QMouseEvent {
  pub fn screenPos<RetType, T: QMouseEvent_screenPos<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.screenPos(self);
    // return 1;
  }
}

pub trait QMouseEvent_screenPos<RetType> {
  fn screenPos(self , rsthis: &mut QMouseEvent) -> RetType;
}

  // proto:  const QPointF & QMouseEvent::screenPos();
impl<'a> /*trait*/ QMouseEvent_screenPos<QPointF> for () {
  fn screenPos(self , rsthis: &mut QMouseEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMouseEvent9screenPosEv()};
    let mut ret = unsafe {_ZNK11QMouseEvent9screenPosEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  int QMouseEvent::x();
impl /*struct*/ QMouseEvent {
  pub fn x<RetType, T: QMouseEvent_x<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.x(self);
    // return 1;
  }
}

pub trait QMouseEvent_x<RetType> {
  fn x(self , rsthis: &mut QMouseEvent) -> RetType;
}

  // proto:  int QMouseEvent::x();
impl<'a> /*trait*/ QMouseEvent_x<()> for () {
  fn x(self , rsthis: &mut QMouseEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMouseEvent1xEv()};
     unsafe {_ZNK11QMouseEvent1xEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  const QPointF & QMouseEvent::localPos();
impl /*struct*/ QMouseEvent {
  pub fn localPos<RetType, T: QMouseEvent_localPos<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.localPos(self);
    // return 1;
  }
}

pub trait QMouseEvent_localPos<RetType> {
  fn localPos(self , rsthis: &mut QMouseEvent) -> RetType;
}

  // proto:  const QPointF & QMouseEvent::localPos();
impl<'a> /*trait*/ QMouseEvent_localPos<QPointF> for () {
  fn localPos(self , rsthis: &mut QMouseEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMouseEvent8localPosEv()};
    let mut ret = unsafe {_ZNK11QMouseEvent8localPosEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  int QMouseEvent::globalX();
impl /*struct*/ QMouseEvent {
  pub fn globalX<RetType, T: QMouseEvent_globalX<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.globalX(self);
    // return 1;
  }
}

pub trait QMouseEvent_globalX<RetType> {
  fn globalX(self , rsthis: &mut QMouseEvent) -> RetType;
}

  // proto:  int QMouseEvent::globalX();
impl<'a> /*trait*/ QMouseEvent_globalX<i32> for () {
  fn globalX(self , rsthis: &mut QMouseEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMouseEvent7globalXEv()};
    let mut ret = unsafe {_ZNK11QMouseEvent7globalXEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  const QPointF & QMouseEvent::windowPos();
impl /*struct*/ QMouseEvent {
  pub fn windowPos<RetType, T: QMouseEvent_windowPos<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.windowPos(self);
    // return 1;
  }
}

pub trait QMouseEvent_windowPos<RetType> {
  fn windowPos(self , rsthis: &mut QMouseEvent) -> RetType;
}

  // proto:  const QPointF & QMouseEvent::windowPos();
impl<'a> /*trait*/ QMouseEvent_windowPos<QPointF> for () {
  fn windowPos(self , rsthis: &mut QMouseEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMouseEvent9windowPosEv()};
    let mut ret = unsafe {_ZNK11QMouseEvent9windowPosEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QMouseEvent::~QMouseEvent();
impl /*struct*/ QMouseEvent {
  pub fn FreeQMouseEvent<RetType, T: QMouseEvent_FreeQMouseEvent<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQMouseEvent(self);
    // return 1;
  }
}

pub trait QMouseEvent_FreeQMouseEvent<RetType> {
  fn FreeQMouseEvent(self , rsthis: &mut QMouseEvent) -> RetType;
}

  // proto:  void QMouseEvent::~QMouseEvent();
impl<'a> /*trait*/ QMouseEvent_FreeQMouseEvent<()> for () {
  fn FreeQMouseEvent(self , rsthis: &mut QMouseEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMouseEventD0Ev()};
     unsafe {_ZN11QMouseEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QMouseEvent::globalY();
impl /*struct*/ QMouseEvent {
  pub fn globalY<RetType, T: QMouseEvent_globalY<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.globalY(self);
    // return 1;
  }
}

pub trait QMouseEvent_globalY<RetType> {
  fn globalY(self , rsthis: &mut QMouseEvent) -> RetType;
}

  // proto:  int QMouseEvent::globalY();
impl<'a> /*trait*/ QMouseEvent_globalY<i32> for () {
  fn globalY(self , rsthis: &mut QMouseEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMouseEvent7globalYEv()};
    let mut ret = unsafe {_ZNK11QMouseEvent7globalYEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QPoint QMouseEvent::pos();
impl /*struct*/ QMouseEvent {
  pub fn pos<RetType, T: QMouseEvent_pos<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.pos(self);
    // return 1;
  }
}

pub trait QMouseEvent_pos<RetType> {
  fn pos(self , rsthis: &mut QMouseEvent) -> RetType;
}

  // proto:  QPoint QMouseEvent::pos();
impl<'a> /*trait*/ QMouseEvent_pos<QPoint> for () {
  fn pos(self , rsthis: &mut QMouseEvent) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMouseEvent3posEv()};
    let mut ret = unsafe {_ZNK11QMouseEvent3posEv(rsthis.qclsinst)};
    let mut ret1 = QPoint{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

