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
  // proto: int QKeyEvent::count();
  fn _ZNK9QKeyEvent5countEv() -> i32;
  // proto: void QKeyEvent::FreeQKeyEvent();
  fn _ZN9QKeyEventD0Ev() -> i32;
  // proto: QString QKeyEvent::text();
  fn _ZNK9QKeyEvent4textEv() -> i32;
  // proto: unsigned int QKeyEvent::nativeVirtualKey();
  fn _ZNK9QKeyEvent16nativeVirtualKeyEv() -> i32;
  // proto: bool QKeyEvent::isAutoRepeat();
  fn _ZNK9QKeyEvent12isAutoRepeatEv() -> i32;
  // proto: int QKeyEvent::key();
  fn _ZNK9QKeyEvent3keyEv() -> i32;
  // proto: unsigned int QKeyEvent::nativeModifiers();
  fn _ZNK9QKeyEvent15nativeModifiersEv() -> i32;
  // proto: unsigned int QKeyEvent::nativeScanCode();
  fn _ZNK9QKeyEvent14nativeScanCodeEv() -> i32;
}

// body block begin
// class sizeof(QKeyEvent)=1
pub struct QKeyEvent {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QKeyEvent {
  pub fn count<T: QKeyEvent_count>(&mut self, value: T) -> i32 {
    value.count(self);
    return 1;
  }
}

pub trait QKeyEvent_count {
  fn count(self, this: &mut QKeyEvent) -> i32;
}

// proto: int QKeyEvent::count();
impl<'a> /*trait*/ QKeyEvent_count for () {
  fn count(self, this: &mut QKeyEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QKeyEvent5countEv()};
    unsafe {_ZNK9QKeyEvent5countEv()};
    return 1;
  }
}

impl /*struct*/ QKeyEvent {
  pub fn FreeQKeyEvent<T: QKeyEvent_FreeQKeyEvent>(&mut self, value: T) -> i32 {
    value.FreeQKeyEvent(self);
    return 1;
  }
}

pub trait QKeyEvent_FreeQKeyEvent {
  fn FreeQKeyEvent(self, this: &mut QKeyEvent) -> i32;
}

// proto: void QKeyEvent::FreeQKeyEvent();
impl<'a> /*trait*/ QKeyEvent_FreeQKeyEvent for () {
  fn FreeQKeyEvent(self, this: &mut QKeyEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QKeyEventD0Ev()};
    unsafe {_ZN9QKeyEventD0Ev()};
    return 1;
  }
}

impl /*struct*/ QKeyEvent {
  pub fn text<T: QKeyEvent_text>(&mut self, value: T) -> i32 {
    value.text(self);
    return 1;
  }
}

pub trait QKeyEvent_text {
  fn text(self, this: &mut QKeyEvent) -> i32;
}

// proto: QString QKeyEvent::text();
impl<'a> /*trait*/ QKeyEvent_text for () {
  fn text(self, this: &mut QKeyEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QKeyEvent4textEv()};
    unsafe {_ZNK9QKeyEvent4textEv()};
    return 1;
  }
}

impl /*struct*/ QKeyEvent {
  pub fn nativeVirtualKey<T: QKeyEvent_nativeVirtualKey>(&mut self, value: T) -> i32 {
    value.nativeVirtualKey(self);
    return 1;
  }
}

pub trait QKeyEvent_nativeVirtualKey {
  fn nativeVirtualKey(self, this: &mut QKeyEvent) -> i32;
}

// proto: unsigned int QKeyEvent::nativeVirtualKey();
impl<'a> /*trait*/ QKeyEvent_nativeVirtualKey for () {
  fn nativeVirtualKey(self, this: &mut QKeyEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QKeyEvent16nativeVirtualKeyEv()};
    unsafe {_ZNK9QKeyEvent16nativeVirtualKeyEv()};
    return 1;
  }
}

impl /*struct*/ QKeyEvent {
  pub fn isAutoRepeat<T: QKeyEvent_isAutoRepeat>(&mut self, value: T) -> i32 {
    value.isAutoRepeat(self);
    return 1;
  }
}

pub trait QKeyEvent_isAutoRepeat {
  fn isAutoRepeat(self, this: &mut QKeyEvent) -> i32;
}

// proto: bool QKeyEvent::isAutoRepeat();
impl<'a> /*trait*/ QKeyEvent_isAutoRepeat for () {
  fn isAutoRepeat(self, this: &mut QKeyEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QKeyEvent12isAutoRepeatEv()};
    unsafe {_ZNK9QKeyEvent12isAutoRepeatEv()};
    return 1;
  }
}

impl /*struct*/ QKeyEvent {
  pub fn key<T: QKeyEvent_key>(&mut self, value: T) -> i32 {
    value.key(self);
    return 1;
  }
}

pub trait QKeyEvent_key {
  fn key(self, this: &mut QKeyEvent) -> i32;
}

// proto: int QKeyEvent::key();
impl<'a> /*trait*/ QKeyEvent_key for () {
  fn key(self, this: &mut QKeyEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QKeyEvent3keyEv()};
    unsafe {_ZNK9QKeyEvent3keyEv()};
    return 1;
  }
}

impl /*struct*/ QKeyEvent {
  pub fn nativeModifiers<T: QKeyEvent_nativeModifiers>(&mut self, value: T) -> i32 {
    value.nativeModifiers(self);
    return 1;
  }
}

pub trait QKeyEvent_nativeModifiers {
  fn nativeModifiers(self, this: &mut QKeyEvent) -> i32;
}

// proto: unsigned int QKeyEvent::nativeModifiers();
impl<'a> /*trait*/ QKeyEvent_nativeModifiers for () {
  fn nativeModifiers(self, this: &mut QKeyEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QKeyEvent15nativeModifiersEv()};
    unsafe {_ZNK9QKeyEvent15nativeModifiersEv()};
    return 1;
  }
}

impl /*struct*/ QKeyEvent {
  pub fn nativeScanCode<T: QKeyEvent_nativeScanCode>(&mut self, value: T) -> i32 {
    value.nativeScanCode(self);
    return 1;
  }
}

pub trait QKeyEvent_nativeScanCode {
  fn nativeScanCode(self, this: &mut QKeyEvent) -> i32;
}

// proto: unsigned int QKeyEvent::nativeScanCode();
impl<'a> /*trait*/ QKeyEvent_nativeScanCode for () {
  fn nativeScanCode(self, this: &mut QKeyEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QKeyEvent14nativeScanCodeEv()};
    unsafe {_ZNK9QKeyEvent14nativeScanCodeEv()};
    return 1;
  }
}

