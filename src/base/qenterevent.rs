// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qpointf::QPointF;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: int QEnterEvent::y();
  fn _ZNK11QEnterEvent1yEv() -> i32;
  // proto: QPoint QEnterEvent::pos();
  fn _ZNK11QEnterEvent3posEv() -> i32;
  // proto: void QEnterEvent::FreeQEnterEvent();
  fn _ZN11QEnterEventD0Ev() -> i32;
  // proto: const QPointF & QEnterEvent::screenPos();
  fn _ZNK11QEnterEvent9screenPosEv() -> i32;
  // proto: const QPointF & QEnterEvent::localPos();
  fn _ZNK11QEnterEvent8localPosEv() -> i32;
  // proto: const QPointF & QEnterEvent::windowPos();
  fn _ZNK11QEnterEvent9windowPosEv() -> i32;
  // proto: int QEnterEvent::globalX();
  fn _ZNK11QEnterEvent7globalXEv() -> i32;
  // proto: int QEnterEvent::x();
  fn _ZNK11QEnterEvent1xEv() -> i32;
  // proto: QPoint QEnterEvent::globalPos();
  fn _ZNK11QEnterEvent9globalPosEv() -> i32;
  // proto: int QEnterEvent::globalY();
  fn _ZNK11QEnterEvent7globalYEv() -> i32;
  // proto: void QEnterEvent::NewQEnterEvent(const QPointF & localPos, const QPointF & windowPos, const QPointF & screenPos);
  fn _ZN11QEnterEventC1ERK7QPointFS2_S2_(qthis: *mut c_void, arg0: *const c_void, arg1: *const c_void, arg2: *const c_void) -> i32;
}

// body block begin
// class sizeof(QEnterEvent)=72
pub struct QEnterEvent {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QEnterEvent {
  pub fn y<T: QEnterEvent_y>(&mut self, value: T) -> i32 {
    value.y(self);
    return 1;
  }
}

pub trait QEnterEvent_y {
  fn y(self, this: &mut QEnterEvent) -> i32;
}

// proto: int QEnterEvent::y();
impl<'a> /*trait*/ QEnterEvent_y for () {
  fn y(self, this: &mut QEnterEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 72)};
    // unsafe{_ZNK11QEnterEvent1yEv()};
    unsafe {_ZNK11QEnterEvent1yEv()};
    return 1;
  }
}

impl /*struct*/ QEnterEvent {
  pub fn pos<T: QEnterEvent_pos>(&mut self, value: T) -> i32 {
    value.pos(self);
    return 1;
  }
}

pub trait QEnterEvent_pos {
  fn pos(self, this: &mut QEnterEvent) -> i32;
}

// proto: QPoint QEnterEvent::pos();
impl<'a> /*trait*/ QEnterEvent_pos for () {
  fn pos(self, this: &mut QEnterEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 72)};
    // unsafe{_ZNK11QEnterEvent3posEv()};
    unsafe {_ZNK11QEnterEvent3posEv()};
    return 1;
  }
}

impl /*struct*/ QEnterEvent {
  pub fn FreeQEnterEvent<T: QEnterEvent_FreeQEnterEvent>(&mut self, value: T) -> i32 {
    value.FreeQEnterEvent(self);
    return 1;
  }
}

pub trait QEnterEvent_FreeQEnterEvent {
  fn FreeQEnterEvent(self, this: &mut QEnterEvent) -> i32;
}

// proto: void QEnterEvent::FreeQEnterEvent();
impl<'a> /*trait*/ QEnterEvent_FreeQEnterEvent for () {
  fn FreeQEnterEvent(self, this: &mut QEnterEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 72)};
    // unsafe{_ZN11QEnterEventD0Ev()};
    unsafe {_ZN11QEnterEventD0Ev()};
    return 1;
  }
}

impl /*struct*/ QEnterEvent {
  pub fn screenPos<T: QEnterEvent_screenPos>(&mut self, value: T) -> i32 {
    value.screenPos(self);
    return 1;
  }
}

pub trait QEnterEvent_screenPos {
  fn screenPos(self, this: &mut QEnterEvent) -> i32;
}

// proto: const QPointF & QEnterEvent::screenPos();
impl<'a> /*trait*/ QEnterEvent_screenPos for () {
  fn screenPos(self, this: &mut QEnterEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 72)};
    // unsafe{_ZNK11QEnterEvent9screenPosEv()};
    unsafe {_ZNK11QEnterEvent9screenPosEv()};
    return 1;
  }
}

impl /*struct*/ QEnterEvent {
  pub fn localPos<T: QEnterEvent_localPos>(&mut self, value: T) -> i32 {
    value.localPos(self);
    return 1;
  }
}

pub trait QEnterEvent_localPos {
  fn localPos(self, this: &mut QEnterEvent) -> i32;
}

// proto: const QPointF & QEnterEvent::localPos();
impl<'a> /*trait*/ QEnterEvent_localPos for () {
  fn localPos(self, this: &mut QEnterEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 72)};
    // unsafe{_ZNK11QEnterEvent8localPosEv()};
    unsafe {_ZNK11QEnterEvent8localPosEv()};
    return 1;
  }
}

impl /*struct*/ QEnterEvent {
  pub fn windowPos<T: QEnterEvent_windowPos>(&mut self, value: T) -> i32 {
    value.windowPos(self);
    return 1;
  }
}

pub trait QEnterEvent_windowPos {
  fn windowPos(self, this: &mut QEnterEvent) -> i32;
}

// proto: const QPointF & QEnterEvent::windowPos();
impl<'a> /*trait*/ QEnterEvent_windowPos for () {
  fn windowPos(self, this: &mut QEnterEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 72)};
    // unsafe{_ZNK11QEnterEvent9windowPosEv()};
    unsafe {_ZNK11QEnterEvent9windowPosEv()};
    return 1;
  }
}

impl /*struct*/ QEnterEvent {
  pub fn globalX<T: QEnterEvent_globalX>(&mut self, value: T) -> i32 {
    value.globalX(self);
    return 1;
  }
}

pub trait QEnterEvent_globalX {
  fn globalX(self, this: &mut QEnterEvent) -> i32;
}

// proto: int QEnterEvent::globalX();
impl<'a> /*trait*/ QEnterEvent_globalX for () {
  fn globalX(self, this: &mut QEnterEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 72)};
    // unsafe{_ZNK11QEnterEvent7globalXEv()};
    unsafe {_ZNK11QEnterEvent7globalXEv()};
    return 1;
  }
}

impl /*struct*/ QEnterEvent {
  pub fn x<T: QEnterEvent_x>(&mut self, value: T) -> i32 {
    value.x(self);
    return 1;
  }
}

pub trait QEnterEvent_x {
  fn x(self, this: &mut QEnterEvent) -> i32;
}

// proto: int QEnterEvent::x();
impl<'a> /*trait*/ QEnterEvent_x for () {
  fn x(self, this: &mut QEnterEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 72)};
    // unsafe{_ZNK11QEnterEvent1xEv()};
    unsafe {_ZNK11QEnterEvent1xEv()};
    return 1;
  }
}

impl /*struct*/ QEnterEvent {
  pub fn globalPos<T: QEnterEvent_globalPos>(&mut self, value: T) -> i32 {
    value.globalPos(self);
    return 1;
  }
}

pub trait QEnterEvent_globalPos {
  fn globalPos(self, this: &mut QEnterEvent) -> i32;
}

// proto: QPoint QEnterEvent::globalPos();
impl<'a> /*trait*/ QEnterEvent_globalPos for () {
  fn globalPos(self, this: &mut QEnterEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 72)};
    // unsafe{_ZNK11QEnterEvent9globalPosEv()};
    unsafe {_ZNK11QEnterEvent9globalPosEv()};
    return 1;
  }
}

impl /*struct*/ QEnterEvent {
  pub fn globalY<T: QEnterEvent_globalY>(&mut self, value: T) -> i32 {
    value.globalY(self);
    return 1;
  }
}

pub trait QEnterEvent_globalY {
  fn globalY(self, this: &mut QEnterEvent) -> i32;
}

// proto: int QEnterEvent::globalY();
impl<'a> /*trait*/ QEnterEvent_globalY for () {
  fn globalY(self, this: &mut QEnterEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 72)};
    // unsafe{_ZNK11QEnterEvent7globalYEv()};
    unsafe {_ZNK11QEnterEvent7globalYEv()};
    return 1;
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
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2.qclsinst  as *const c_void;
    unsafe {_ZN11QEnterEventC1ERK7QPointFS2_S2_(qthis, arg0, arg1, arg2)};
    let rsthis = QEnterEvent{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

