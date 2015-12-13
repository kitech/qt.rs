// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: QPoint QMouseEvent::globalPos();
  fn _ZNK11QMouseEvent9globalPosEv() -> i32;
  // proto: int QMouseEvent::y();
  fn _ZNK11QMouseEvent1yEv() -> i32;
  // proto: const QPointF & QMouseEvent::screenPos();
  fn _ZNK11QMouseEvent9screenPosEv() -> i32;
  // proto: int QMouseEvent::x();
  fn _ZNK11QMouseEvent1xEv() -> i32;
  // proto: const QPointF & QMouseEvent::localPos();
  fn _ZNK11QMouseEvent8localPosEv() -> i32;
  // proto: int QMouseEvent::globalX();
  fn _ZNK11QMouseEvent7globalXEv() -> i32;
  // proto: const QPointF & QMouseEvent::windowPos();
  fn _ZNK11QMouseEvent9windowPosEv() -> i32;
  // proto: void QMouseEvent::FreeQMouseEvent();
  fn _ZN11QMouseEventD0Ev() -> i32;
  // proto: int QMouseEvent::globalY();
  fn _ZNK11QMouseEvent7globalYEv() -> i32;
  // proto: QPoint QMouseEvent::pos();
  fn _ZNK11QMouseEvent3posEv() -> i32;
}

// body block begin
// class sizeof(QMouseEvent)=1
pub struct QMouseEvent {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QMouseEvent {
  pub fn globalPos<T: QMouseEvent_globalPos>(&mut self, value: T) -> i32 {
    value.globalPos(self);
    return 1;
  }
}

pub trait QMouseEvent_globalPos {
  fn globalPos(self, this: &mut QMouseEvent) -> i32;
}

// proto: QPoint QMouseEvent::globalPos();
impl<'a> /*trait*/ QMouseEvent_globalPos for () {
  fn globalPos(self, this: &mut QMouseEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMouseEvent9globalPosEv()};
    unsafe {_ZNK11QMouseEvent9globalPosEv()};
    return 1;
  }
}

impl /*struct*/ QMouseEvent {
  pub fn y<T: QMouseEvent_y>(&mut self, value: T) -> i32 {
    value.y(self);
    return 1;
  }
}

pub trait QMouseEvent_y {
  fn y(self, this: &mut QMouseEvent) -> i32;
}

// proto: int QMouseEvent::y();
impl<'a> /*trait*/ QMouseEvent_y for () {
  fn y(self, this: &mut QMouseEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMouseEvent1yEv()};
    unsafe {_ZNK11QMouseEvent1yEv()};
    return 1;
  }
}

impl /*struct*/ QMouseEvent {
  pub fn screenPos<T: QMouseEvent_screenPos>(&mut self, value: T) -> i32 {
    value.screenPos(self);
    return 1;
  }
}

pub trait QMouseEvent_screenPos {
  fn screenPos(self, this: &mut QMouseEvent) -> i32;
}

// proto: const QPointF & QMouseEvent::screenPos();
impl<'a> /*trait*/ QMouseEvent_screenPos for () {
  fn screenPos(self, this: &mut QMouseEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMouseEvent9screenPosEv()};
    unsafe {_ZNK11QMouseEvent9screenPosEv()};
    return 1;
  }
}

impl /*struct*/ QMouseEvent {
  pub fn x<T: QMouseEvent_x>(&mut self, value: T) -> i32 {
    value.x(self);
    return 1;
  }
}

pub trait QMouseEvent_x {
  fn x(self, this: &mut QMouseEvent) -> i32;
}

// proto: int QMouseEvent::x();
impl<'a> /*trait*/ QMouseEvent_x for () {
  fn x(self, this: &mut QMouseEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMouseEvent1xEv()};
    unsafe {_ZNK11QMouseEvent1xEv()};
    return 1;
  }
}

impl /*struct*/ QMouseEvent {
  pub fn localPos<T: QMouseEvent_localPos>(&mut self, value: T) -> i32 {
    value.localPos(self);
    return 1;
  }
}

pub trait QMouseEvent_localPos {
  fn localPos(self, this: &mut QMouseEvent) -> i32;
}

// proto: const QPointF & QMouseEvent::localPos();
impl<'a> /*trait*/ QMouseEvent_localPos for () {
  fn localPos(self, this: &mut QMouseEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMouseEvent8localPosEv()};
    unsafe {_ZNK11QMouseEvent8localPosEv()};
    return 1;
  }
}

impl /*struct*/ QMouseEvent {
  pub fn globalX<T: QMouseEvent_globalX>(&mut self, value: T) -> i32 {
    value.globalX(self);
    return 1;
  }
}

pub trait QMouseEvent_globalX {
  fn globalX(self, this: &mut QMouseEvent) -> i32;
}

// proto: int QMouseEvent::globalX();
impl<'a> /*trait*/ QMouseEvent_globalX for () {
  fn globalX(self, this: &mut QMouseEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMouseEvent7globalXEv()};
    unsafe {_ZNK11QMouseEvent7globalXEv()};
    return 1;
  }
}

impl /*struct*/ QMouseEvent {
  pub fn windowPos<T: QMouseEvent_windowPos>(&mut self, value: T) -> i32 {
    value.windowPos(self);
    return 1;
  }
}

pub trait QMouseEvent_windowPos {
  fn windowPos(self, this: &mut QMouseEvent) -> i32;
}

// proto: const QPointF & QMouseEvent::windowPos();
impl<'a> /*trait*/ QMouseEvent_windowPos for () {
  fn windowPos(self, this: &mut QMouseEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMouseEvent9windowPosEv()};
    unsafe {_ZNK11QMouseEvent9windowPosEv()};
    return 1;
  }
}

impl /*struct*/ QMouseEvent {
  pub fn FreeQMouseEvent<T: QMouseEvent_FreeQMouseEvent>(&mut self, value: T) -> i32 {
    value.FreeQMouseEvent(self);
    return 1;
  }
}

pub trait QMouseEvent_FreeQMouseEvent {
  fn FreeQMouseEvent(self, this: &mut QMouseEvent) -> i32;
}

// proto: void QMouseEvent::FreeQMouseEvent();
impl<'a> /*trait*/ QMouseEvent_FreeQMouseEvent for () {
  fn FreeQMouseEvent(self, this: &mut QMouseEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMouseEventD0Ev()};
    unsafe {_ZN11QMouseEventD0Ev()};
    return 1;
  }
}

impl /*struct*/ QMouseEvent {
  pub fn globalY<T: QMouseEvent_globalY>(&mut self, value: T) -> i32 {
    value.globalY(self);
    return 1;
  }
}

pub trait QMouseEvent_globalY {
  fn globalY(self, this: &mut QMouseEvent) -> i32;
}

// proto: int QMouseEvent::globalY();
impl<'a> /*trait*/ QMouseEvent_globalY for () {
  fn globalY(self, this: &mut QMouseEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMouseEvent7globalYEv()};
    unsafe {_ZNK11QMouseEvent7globalYEv()};
    return 1;
  }
}

impl /*struct*/ QMouseEvent {
  pub fn pos<T: QMouseEvent_pos>(&mut self, value: T) -> i32 {
    value.pos(self);
    return 1;
  }
}

pub trait QMouseEvent_pos {
  fn pos(self, this: &mut QMouseEvent) -> i32;
}

// proto: QPoint QMouseEvent::pos();
impl<'a> /*trait*/ QMouseEvent_pos for () {
  fn pos(self, this: &mut QMouseEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMouseEvent3posEv()};
    unsafe {_ZNK11QMouseEvent3posEv()};
    return 1;
  }
}

