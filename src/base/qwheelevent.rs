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
  // proto: int QWheelEvent::x();
  fn _ZNK11QWheelEvent1xEv() -> i32;
  // proto: QPoint QWheelEvent::angleDelta();
  fn _ZNK11QWheelEvent10angleDeltaEv() -> i32;
  // proto: QPoint QWheelEvent::pos();
  fn _ZNK11QWheelEvent3posEv() -> i32;
  // proto: int QWheelEvent::globalY();
  fn _ZNK11QWheelEvent7globalYEv() -> i32;
  // proto: const QPointF & QWheelEvent::posF();
  fn _ZNK11QWheelEvent4posFEv() -> i32;
  // proto: int QWheelEvent::globalX();
  fn _ZNK11QWheelEvent7globalXEv() -> i32;
  // proto: int QWheelEvent::y();
  fn _ZNK11QWheelEvent1yEv() -> i32;
  // proto: void QWheelEvent::FreeQWheelEvent();
  fn _ZN11QWheelEventD0Ev() -> i32;
  // proto: QPoint QWheelEvent::pixelDelta();
  fn _ZNK11QWheelEvent10pixelDeltaEv() -> i32;
  // proto: int QWheelEvent::delta();
  fn _ZNK11QWheelEvent5deltaEv() -> i32;
  // proto: QPoint QWheelEvent::globalPos();
  fn _ZNK11QWheelEvent9globalPosEv() -> i32;
  // proto: const QPointF & QWheelEvent::globalPosF();
  fn _ZNK11QWheelEvent10globalPosFEv() -> i32;
}

// body block begin
// class sizeof(QWheelEvent)=1
pub struct QWheelEvent {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QWheelEvent {
  pub fn x<T: QWheelEvent_x>(&mut self, value: T) -> i32 {
    value.x(self);
    return 1;
  }
}

pub trait QWheelEvent_x {
  fn x(self, this: &mut QWheelEvent) -> i32;
}

// proto: int QWheelEvent::x();
impl<'a> /*trait*/ QWheelEvent_x for () {
  fn x(self, this: &mut QWheelEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWheelEvent1xEv()};
    unsafe {_ZNK11QWheelEvent1xEv()};
    return 1;
  }
}

impl /*struct*/ QWheelEvent {
  pub fn angleDelta<T: QWheelEvent_angleDelta>(&mut self, value: T) -> i32 {
    value.angleDelta(self);
    return 1;
  }
}

pub trait QWheelEvent_angleDelta {
  fn angleDelta(self, this: &mut QWheelEvent) -> i32;
}

// proto: QPoint QWheelEvent::angleDelta();
impl<'a> /*trait*/ QWheelEvent_angleDelta for () {
  fn angleDelta(self, this: &mut QWheelEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWheelEvent10angleDeltaEv()};
    unsafe {_ZNK11QWheelEvent10angleDeltaEv()};
    return 1;
  }
}

impl /*struct*/ QWheelEvent {
  pub fn pos<T: QWheelEvent_pos>(&mut self, value: T) -> i32 {
    value.pos(self);
    return 1;
  }
}

pub trait QWheelEvent_pos {
  fn pos(self, this: &mut QWheelEvent) -> i32;
}

// proto: QPoint QWheelEvent::pos();
impl<'a> /*trait*/ QWheelEvent_pos for () {
  fn pos(self, this: &mut QWheelEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWheelEvent3posEv()};
    unsafe {_ZNK11QWheelEvent3posEv()};
    return 1;
  }
}

impl /*struct*/ QWheelEvent {
  pub fn globalY<T: QWheelEvent_globalY>(&mut self, value: T) -> i32 {
    value.globalY(self);
    return 1;
  }
}

pub trait QWheelEvent_globalY {
  fn globalY(self, this: &mut QWheelEvent) -> i32;
}

// proto: int QWheelEvent::globalY();
impl<'a> /*trait*/ QWheelEvent_globalY for () {
  fn globalY(self, this: &mut QWheelEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWheelEvent7globalYEv()};
    unsafe {_ZNK11QWheelEvent7globalYEv()};
    return 1;
  }
}

impl /*struct*/ QWheelEvent {
  pub fn posF<T: QWheelEvent_posF>(&mut self, value: T) -> i32 {
    value.posF(self);
    return 1;
  }
}

pub trait QWheelEvent_posF {
  fn posF(self, this: &mut QWheelEvent) -> i32;
}

// proto: const QPointF & QWheelEvent::posF();
impl<'a> /*trait*/ QWheelEvent_posF for () {
  fn posF(self, this: &mut QWheelEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWheelEvent4posFEv()};
    unsafe {_ZNK11QWheelEvent4posFEv()};
    return 1;
  }
}

impl /*struct*/ QWheelEvent {
  pub fn globalX<T: QWheelEvent_globalX>(&mut self, value: T) -> i32 {
    value.globalX(self);
    return 1;
  }
}

pub trait QWheelEvent_globalX {
  fn globalX(self, this: &mut QWheelEvent) -> i32;
}

// proto: int QWheelEvent::globalX();
impl<'a> /*trait*/ QWheelEvent_globalX for () {
  fn globalX(self, this: &mut QWheelEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWheelEvent7globalXEv()};
    unsafe {_ZNK11QWheelEvent7globalXEv()};
    return 1;
  }
}

impl /*struct*/ QWheelEvent {
  pub fn y<T: QWheelEvent_y>(&mut self, value: T) -> i32 {
    value.y(self);
    return 1;
  }
}

pub trait QWheelEvent_y {
  fn y(self, this: &mut QWheelEvent) -> i32;
}

// proto: int QWheelEvent::y();
impl<'a> /*trait*/ QWheelEvent_y for () {
  fn y(self, this: &mut QWheelEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWheelEvent1yEv()};
    unsafe {_ZNK11QWheelEvent1yEv()};
    return 1;
  }
}

impl /*struct*/ QWheelEvent {
  pub fn FreeQWheelEvent<T: QWheelEvent_FreeQWheelEvent>(&mut self, value: T) -> i32 {
    value.FreeQWheelEvent(self);
    return 1;
  }
}

pub trait QWheelEvent_FreeQWheelEvent {
  fn FreeQWheelEvent(self, this: &mut QWheelEvent) -> i32;
}

// proto: void QWheelEvent::FreeQWheelEvent();
impl<'a> /*trait*/ QWheelEvent_FreeQWheelEvent for () {
  fn FreeQWheelEvent(self, this: &mut QWheelEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QWheelEventD0Ev()};
    unsafe {_ZN11QWheelEventD0Ev()};
    return 1;
  }
}

impl /*struct*/ QWheelEvent {
  pub fn pixelDelta<T: QWheelEvent_pixelDelta>(&mut self, value: T) -> i32 {
    value.pixelDelta(self);
    return 1;
  }
}

pub trait QWheelEvent_pixelDelta {
  fn pixelDelta(self, this: &mut QWheelEvent) -> i32;
}

// proto: QPoint QWheelEvent::pixelDelta();
impl<'a> /*trait*/ QWheelEvent_pixelDelta for () {
  fn pixelDelta(self, this: &mut QWheelEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWheelEvent10pixelDeltaEv()};
    unsafe {_ZNK11QWheelEvent10pixelDeltaEv()};
    return 1;
  }
}

impl /*struct*/ QWheelEvent {
  pub fn delta<T: QWheelEvent_delta>(&mut self, value: T) -> i32 {
    value.delta(self);
    return 1;
  }
}

pub trait QWheelEvent_delta {
  fn delta(self, this: &mut QWheelEvent) -> i32;
}

// proto: int QWheelEvent::delta();
impl<'a> /*trait*/ QWheelEvent_delta for () {
  fn delta(self, this: &mut QWheelEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWheelEvent5deltaEv()};
    unsafe {_ZNK11QWheelEvent5deltaEv()};
    return 1;
  }
}

impl /*struct*/ QWheelEvent {
  pub fn globalPos<T: QWheelEvent_globalPos>(&mut self, value: T) -> i32 {
    value.globalPos(self);
    return 1;
  }
}

pub trait QWheelEvent_globalPos {
  fn globalPos(self, this: &mut QWheelEvent) -> i32;
}

// proto: QPoint QWheelEvent::globalPos();
impl<'a> /*trait*/ QWheelEvent_globalPos for () {
  fn globalPos(self, this: &mut QWheelEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWheelEvent9globalPosEv()};
    unsafe {_ZNK11QWheelEvent9globalPosEv()};
    return 1;
  }
}

impl /*struct*/ QWheelEvent {
  pub fn globalPosF<T: QWheelEvent_globalPosF>(&mut self, value: T) -> i32 {
    value.globalPosF(self);
    return 1;
  }
}

pub trait QWheelEvent_globalPosF {
  fn globalPosF(self, this: &mut QWheelEvent) -> i32;
}

// proto: const QPointF & QWheelEvent::globalPosF();
impl<'a> /*trait*/ QWheelEvent_globalPosF for () {
  fn globalPosF(self, this: &mut QWheelEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWheelEvent10globalPosFEv()};
    unsafe {_ZNK11QWheelEvent10globalPosFEv()};
    return 1;
  }
}

