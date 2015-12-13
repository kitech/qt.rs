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
  // proto: const QPointF & QNativeGestureEvent::localPos();
  fn _ZNK19QNativeGestureEvent8localPosEv() -> i32;
  // proto: const QPointF & QNativeGestureEvent::screenPos();
  fn _ZNK19QNativeGestureEvent9screenPosEv() -> i32;
  // proto: const QPoint QNativeGestureEvent::pos();
  fn _ZNK19QNativeGestureEvent3posEv() -> i32;
  // proto: const QPoint QNativeGestureEvent::globalPos();
  fn _ZNK19QNativeGestureEvent9globalPosEv() -> i32;
  // proto: double QNativeGestureEvent::value();
  fn _ZNK19QNativeGestureEvent5valueEv() -> i32;
  // proto: const QPointF & QNativeGestureEvent::windowPos();
  fn _ZNK19QNativeGestureEvent9windowPosEv() -> i32;
}

// body block begin
// class sizeof(QNativeGestureEvent)=1
pub struct QNativeGestureEvent {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QNativeGestureEvent {
  pub fn localPos<T: QNativeGestureEvent_localPos>(&mut self, value: T) -> i32 {
    value.localPos(self);
    return 1;
  }
}

pub trait QNativeGestureEvent_localPos {
  fn localPos(self, this: &mut QNativeGestureEvent) -> i32;
}

// proto: const QPointF & QNativeGestureEvent::localPos();
impl<'a> /*trait*/ QNativeGestureEvent_localPos for () {
  fn localPos(self, this: &mut QNativeGestureEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QNativeGestureEvent8localPosEv()};
    unsafe {_ZNK19QNativeGestureEvent8localPosEv()};
    return 1;
  }
}

impl /*struct*/ QNativeGestureEvent {
  pub fn screenPos<T: QNativeGestureEvent_screenPos>(&mut self, value: T) -> i32 {
    value.screenPos(self);
    return 1;
  }
}

pub trait QNativeGestureEvent_screenPos {
  fn screenPos(self, this: &mut QNativeGestureEvent) -> i32;
}

// proto: const QPointF & QNativeGestureEvent::screenPos();
impl<'a> /*trait*/ QNativeGestureEvent_screenPos for () {
  fn screenPos(self, this: &mut QNativeGestureEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QNativeGestureEvent9screenPosEv()};
    unsafe {_ZNK19QNativeGestureEvent9screenPosEv()};
    return 1;
  }
}

impl /*struct*/ QNativeGestureEvent {
  pub fn pos<T: QNativeGestureEvent_pos>(&mut self, value: T) -> i32 {
    value.pos(self);
    return 1;
  }
}

pub trait QNativeGestureEvent_pos {
  fn pos(self, this: &mut QNativeGestureEvent) -> i32;
}

// proto: const QPoint QNativeGestureEvent::pos();
impl<'a> /*trait*/ QNativeGestureEvent_pos for () {
  fn pos(self, this: &mut QNativeGestureEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QNativeGestureEvent3posEv()};
    unsafe {_ZNK19QNativeGestureEvent3posEv()};
    return 1;
  }
}

impl /*struct*/ QNativeGestureEvent {
  pub fn globalPos<T: QNativeGestureEvent_globalPos>(&mut self, value: T) -> i32 {
    value.globalPos(self);
    return 1;
  }
}

pub trait QNativeGestureEvent_globalPos {
  fn globalPos(self, this: &mut QNativeGestureEvent) -> i32;
}

// proto: const QPoint QNativeGestureEvent::globalPos();
impl<'a> /*trait*/ QNativeGestureEvent_globalPos for () {
  fn globalPos(self, this: &mut QNativeGestureEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QNativeGestureEvent9globalPosEv()};
    unsafe {_ZNK19QNativeGestureEvent9globalPosEv()};
    return 1;
  }
}

impl /*struct*/ QNativeGestureEvent {
  pub fn value<T: QNativeGestureEvent_value>(&mut self, value: T) -> i32 {
    value.value(self);
    return 1;
  }
}

pub trait QNativeGestureEvent_value {
  fn value(self, this: &mut QNativeGestureEvent) -> i32;
}

// proto: double QNativeGestureEvent::value();
impl<'a> /*trait*/ QNativeGestureEvent_value for () {
  fn value(self, this: &mut QNativeGestureEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QNativeGestureEvent5valueEv()};
    unsafe {_ZNK19QNativeGestureEvent5valueEv()};
    return 1;
  }
}

impl /*struct*/ QNativeGestureEvent {
  pub fn windowPos<T: QNativeGestureEvent_windowPos>(&mut self, value: T) -> i32 {
    value.windowPos(self);
    return 1;
  }
}

pub trait QNativeGestureEvent_windowPos {
  fn windowPos(self, this: &mut QNativeGestureEvent) -> i32;
}

// proto: const QPointF & QNativeGestureEvent::windowPos();
impl<'a> /*trait*/ QNativeGestureEvent_windowPos for () {
  fn windowPos(self, this: &mut QNativeGestureEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QNativeGestureEvent9windowPosEv()};
    unsafe {_ZNK19QNativeGestureEvent9windowPosEv()};
    return 1;
  }
}

