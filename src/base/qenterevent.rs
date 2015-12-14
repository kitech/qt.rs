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
  fn _ZNK11QEnterEvent1yEv(qthis: *mut c_void) -> c_int;
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
  fn _ZNK11QEnterEvent1xEv(qthis: *mut c_void) -> c_int;
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

impl /*struct*/ QEnterEvent {
  pub fn y<T: QEnterEvent_y>(&mut self, value: T) -> i32 {
    return value.y(self);
    // return 1;
  }
}

pub trait QEnterEvent_y {
  fn y(self, rsthis: &mut QEnterEvent) -> i32;
}

// proto:  int QEnterEvent::y();
impl<'a> /*trait*/ QEnterEvent_y for () {
  fn y(self, rsthis: &mut QEnterEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 72)};
    // unsafe{_ZNK11QEnterEvent1yEv()};
    let mut ret = unsafe {_ZNK11QEnterEvent1yEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QEnterEvent {
  pub fn pos<T: QEnterEvent_pos>(&mut self, value: T) -> QPoint {
    return value.pos(self);
    // return 1;
  }
}

pub trait QEnterEvent_pos {
  fn pos(self, rsthis: &mut QEnterEvent) -> QPoint;
}

// proto:  QPoint QEnterEvent::pos();
impl<'a> /*trait*/ QEnterEvent_pos for () {
  fn pos(self, rsthis: &mut QEnterEvent) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 72)};
    // unsafe{_ZNK11QEnterEvent3posEv()};
    let mut ret = unsafe {_ZNK11QEnterEvent3posEv(rsthis.qclsinst)};
    let mut ret1 = QPoint{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QEnterEvent {
  pub fn FreeQEnterEvent<T: QEnterEvent_FreeQEnterEvent>(&mut self, value: T)  {
     value.FreeQEnterEvent(self);
    // return 1;
  }
}

pub trait QEnterEvent_FreeQEnterEvent {
  fn FreeQEnterEvent(self, rsthis: &mut QEnterEvent) ;
}

// proto:  void QEnterEvent::FreeQEnterEvent();
impl<'a> /*trait*/ QEnterEvent_FreeQEnterEvent for () {
  fn FreeQEnterEvent(self, rsthis: &mut QEnterEvent)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 72)};
    // unsafe{_ZN11QEnterEventD0Ev()};
     unsafe {_ZN11QEnterEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QEnterEvent {
  pub fn screenPos<T: QEnterEvent_screenPos>(&mut self, value: T) -> QPointF {
    return value.screenPos(self);
    // return 1;
  }
}

pub trait QEnterEvent_screenPos {
  fn screenPos(self, rsthis: &mut QEnterEvent) -> QPointF;
}

// proto:  const QPointF & QEnterEvent::screenPos();
impl<'a> /*trait*/ QEnterEvent_screenPos for () {
  fn screenPos(self, rsthis: &mut QEnterEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 72)};
    // unsafe{_ZNK11QEnterEvent9screenPosEv()};
    let mut ret = unsafe {_ZNK11QEnterEvent9screenPosEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QEnterEvent {
  pub fn localPos<T: QEnterEvent_localPos>(&mut self, value: T) -> QPointF {
    return value.localPos(self);
    // return 1;
  }
}

pub trait QEnterEvent_localPos {
  fn localPos(self, rsthis: &mut QEnterEvent) -> QPointF;
}

// proto:  const QPointF & QEnterEvent::localPos();
impl<'a> /*trait*/ QEnterEvent_localPos for () {
  fn localPos(self, rsthis: &mut QEnterEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 72)};
    // unsafe{_ZNK11QEnterEvent8localPosEv()};
    let mut ret = unsafe {_ZNK11QEnterEvent8localPosEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QEnterEvent {
  pub fn windowPos<T: QEnterEvent_windowPos>(&mut self, value: T) -> QPointF {
    return value.windowPos(self);
    // return 1;
  }
}

pub trait QEnterEvent_windowPos {
  fn windowPos(self, rsthis: &mut QEnterEvent) -> QPointF;
}

// proto:  const QPointF & QEnterEvent::windowPos();
impl<'a> /*trait*/ QEnterEvent_windowPos for () {
  fn windowPos(self, rsthis: &mut QEnterEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 72)};
    // unsafe{_ZNK11QEnterEvent9windowPosEv()};
    let mut ret = unsafe {_ZNK11QEnterEvent9windowPosEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QEnterEvent {
  pub fn globalX<T: QEnterEvent_globalX>(&mut self, value: T) -> i32 {
    return value.globalX(self);
    // return 1;
  }
}

pub trait QEnterEvent_globalX {
  fn globalX(self, rsthis: &mut QEnterEvent) -> i32;
}

// proto:  int QEnterEvent::globalX();
impl<'a> /*trait*/ QEnterEvent_globalX for () {
  fn globalX(self, rsthis: &mut QEnterEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 72)};
    // unsafe{_ZNK11QEnterEvent7globalXEv()};
    let mut ret = unsafe {_ZNK11QEnterEvent7globalXEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QEnterEvent {
  pub fn x<T: QEnterEvent_x>(&mut self, value: T) -> i32 {
    return value.x(self);
    // return 1;
  }
}

pub trait QEnterEvent_x {
  fn x(self, rsthis: &mut QEnterEvent) -> i32;
}

// proto:  int QEnterEvent::x();
impl<'a> /*trait*/ QEnterEvent_x for () {
  fn x(self, rsthis: &mut QEnterEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 72)};
    // unsafe{_ZNK11QEnterEvent1xEv()};
    let mut ret = unsafe {_ZNK11QEnterEvent1xEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QEnterEvent {
  pub fn globalPos<T: QEnterEvent_globalPos>(&mut self, value: T) -> QPoint {
    return value.globalPos(self);
    // return 1;
  }
}

pub trait QEnterEvent_globalPos {
  fn globalPos(self, rsthis: &mut QEnterEvent) -> QPoint;
}

// proto:  QPoint QEnterEvent::globalPos();
impl<'a> /*trait*/ QEnterEvent_globalPos for () {
  fn globalPos(self, rsthis: &mut QEnterEvent) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 72)};
    // unsafe{_ZNK11QEnterEvent9globalPosEv()};
    let mut ret = unsafe {_ZNK11QEnterEvent9globalPosEv(rsthis.qclsinst)};
    let mut ret1 = QPoint{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QEnterEvent {
  pub fn globalY<T: QEnterEvent_globalY>(&mut self, value: T) -> i32 {
    return value.globalY(self);
    // return 1;
  }
}

pub trait QEnterEvent_globalY {
  fn globalY(self, rsthis: &mut QEnterEvent) -> i32;
}

// proto:  int QEnterEvent::globalY();
impl<'a> /*trait*/ QEnterEvent_globalY for () {
  fn globalY(self, rsthis: &mut QEnterEvent) -> i32 {
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

