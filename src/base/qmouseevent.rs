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
  fn _ZNK11QMouseEvent1yEv(qthis: *mut c_void) -> c_int;
  // proto:  const QPointF & QMouseEvent::screenPos();
  fn _ZNK11QMouseEvent9screenPosEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QMouseEvent::x();
  fn _ZNK11QMouseEvent1xEv(qthis: *mut c_void) -> c_int;
  // proto:  const QPointF & QMouseEvent::localPos();
  fn _ZNK11QMouseEvent8localPosEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QMouseEvent::globalX();
  fn _ZNK11QMouseEvent7globalXEv(qthis: *mut c_void) -> c_int;
  // proto:  const QPointF & QMouseEvent::windowPos();
  fn _ZNK11QMouseEvent9windowPosEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QMouseEvent::FreeQMouseEvent();
  fn _ZN11QMouseEventD0Ev(qthis: *mut c_void) ;
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

impl /*struct*/ QMouseEvent {
  pub fn globalPos<T: QMouseEvent_globalPos>(&mut self, value: T) -> QPoint {
    return value.globalPos(self);
    // return 1;
  }
}

pub trait QMouseEvent_globalPos {
  fn globalPos(self, rsthis: &mut QMouseEvent) -> QPoint;
}

// proto:  QPoint QMouseEvent::globalPos();
impl<'a> /*trait*/ QMouseEvent_globalPos for () {
  fn globalPos(self, rsthis: &mut QMouseEvent) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMouseEvent9globalPosEv()};
    let mut ret = unsafe {_ZNK11QMouseEvent9globalPosEv(rsthis.qclsinst)};
    let mut ret1 = QPoint{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QMouseEvent {
  pub fn y<T: QMouseEvent_y>(&mut self, value: T) -> i32 {
    return value.y(self);
    // return 1;
  }
}

pub trait QMouseEvent_y {
  fn y(self, rsthis: &mut QMouseEvent) -> i32;
}

// proto:  int QMouseEvent::y();
impl<'a> /*trait*/ QMouseEvent_y for () {
  fn y(self, rsthis: &mut QMouseEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMouseEvent1yEv()};
    let mut ret = unsafe {_ZNK11QMouseEvent1yEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QMouseEvent {
  pub fn screenPos<T: QMouseEvent_screenPos>(&mut self, value: T) -> QPointF {
    return value.screenPos(self);
    // return 1;
  }
}

pub trait QMouseEvent_screenPos {
  fn screenPos(self, rsthis: &mut QMouseEvent) -> QPointF;
}

// proto:  const QPointF & QMouseEvent::screenPos();
impl<'a> /*trait*/ QMouseEvent_screenPos for () {
  fn screenPos(self, rsthis: &mut QMouseEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMouseEvent9screenPosEv()};
    let mut ret = unsafe {_ZNK11QMouseEvent9screenPosEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QMouseEvent {
  pub fn x<T: QMouseEvent_x>(&mut self, value: T) -> i32 {
    return value.x(self);
    // return 1;
  }
}

pub trait QMouseEvent_x {
  fn x(self, rsthis: &mut QMouseEvent) -> i32;
}

// proto:  int QMouseEvent::x();
impl<'a> /*trait*/ QMouseEvent_x for () {
  fn x(self, rsthis: &mut QMouseEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMouseEvent1xEv()};
    let mut ret = unsafe {_ZNK11QMouseEvent1xEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QMouseEvent {
  pub fn localPos<T: QMouseEvent_localPos>(&mut self, value: T) -> QPointF {
    return value.localPos(self);
    // return 1;
  }
}

pub trait QMouseEvent_localPos {
  fn localPos(self, rsthis: &mut QMouseEvent) -> QPointF;
}

// proto:  const QPointF & QMouseEvent::localPos();
impl<'a> /*trait*/ QMouseEvent_localPos for () {
  fn localPos(self, rsthis: &mut QMouseEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMouseEvent8localPosEv()};
    let mut ret = unsafe {_ZNK11QMouseEvent8localPosEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QMouseEvent {
  pub fn globalX<T: QMouseEvent_globalX>(&mut self, value: T) -> i32 {
    return value.globalX(self);
    // return 1;
  }
}

pub trait QMouseEvent_globalX {
  fn globalX(self, rsthis: &mut QMouseEvent) -> i32;
}

// proto:  int QMouseEvent::globalX();
impl<'a> /*trait*/ QMouseEvent_globalX for () {
  fn globalX(self, rsthis: &mut QMouseEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMouseEvent7globalXEv()};
    let mut ret = unsafe {_ZNK11QMouseEvent7globalXEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QMouseEvent {
  pub fn windowPos<T: QMouseEvent_windowPos>(&mut self, value: T) -> QPointF {
    return value.windowPos(self);
    // return 1;
  }
}

pub trait QMouseEvent_windowPos {
  fn windowPos(self, rsthis: &mut QMouseEvent) -> QPointF;
}

// proto:  const QPointF & QMouseEvent::windowPos();
impl<'a> /*trait*/ QMouseEvent_windowPos for () {
  fn windowPos(self, rsthis: &mut QMouseEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMouseEvent9windowPosEv()};
    let mut ret = unsafe {_ZNK11QMouseEvent9windowPosEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QMouseEvent {
  pub fn FreeQMouseEvent<T: QMouseEvent_FreeQMouseEvent>(&mut self, value: T)  {
     value.FreeQMouseEvent(self);
    // return 1;
  }
}

pub trait QMouseEvent_FreeQMouseEvent {
  fn FreeQMouseEvent(self, rsthis: &mut QMouseEvent) ;
}

// proto:  void QMouseEvent::FreeQMouseEvent();
impl<'a> /*trait*/ QMouseEvent_FreeQMouseEvent for () {
  fn FreeQMouseEvent(self, rsthis: &mut QMouseEvent)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMouseEventD0Ev()};
     unsafe {_ZN11QMouseEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QMouseEvent {
  pub fn globalY<T: QMouseEvent_globalY>(&mut self, value: T) -> i32 {
    return value.globalY(self);
    // return 1;
  }
}

pub trait QMouseEvent_globalY {
  fn globalY(self, rsthis: &mut QMouseEvent) -> i32;
}

// proto:  int QMouseEvent::globalY();
impl<'a> /*trait*/ QMouseEvent_globalY for () {
  fn globalY(self, rsthis: &mut QMouseEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMouseEvent7globalYEv()};
    let mut ret = unsafe {_ZNK11QMouseEvent7globalYEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QMouseEvent {
  pub fn pos<T: QMouseEvent_pos>(&mut self, value: T) -> QPoint {
    return value.pos(self);
    // return 1;
  }
}

pub trait QMouseEvent_pos {
  fn pos(self, rsthis: &mut QMouseEvent) -> QPoint;
}

// proto:  QPoint QMouseEvent::pos();
impl<'a> /*trait*/ QMouseEvent_pos for () {
  fn pos(self, rsthis: &mut QMouseEvent) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMouseEvent3posEv()};
    let mut ret = unsafe {_ZNK11QMouseEvent3posEv(rsthis.qclsinst)};
    let mut ret1 = QPoint{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

