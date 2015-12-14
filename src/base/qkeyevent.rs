// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstring::QString;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  int QKeyEvent::count();
  fn _ZNK9QKeyEvent5countEv(qthis: *mut c_void) -> c_int;
  // proto:  void QKeyEvent::FreeQKeyEvent();
  fn _ZN9QKeyEventD0Ev(qthis: *mut c_void) ;
  // proto:  QString QKeyEvent::text();
  fn _ZNK9QKeyEvent4textEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  unsigned int QKeyEvent::nativeVirtualKey();
  fn _ZNK9QKeyEvent16nativeVirtualKeyEv(qthis: *mut c_void) -> c_uint;
  // proto:  bool QKeyEvent::isAutoRepeat();
  fn _ZNK9QKeyEvent12isAutoRepeatEv(qthis: *mut c_void) -> int8_t;
  // proto:  int QKeyEvent::key();
  fn _ZNK9QKeyEvent3keyEv(qthis: *mut c_void) -> c_int;
  // proto:  unsigned int QKeyEvent::nativeModifiers();
  fn _ZNK9QKeyEvent15nativeModifiersEv(qthis: *mut c_void) -> c_uint;
  // proto:  unsigned int QKeyEvent::nativeScanCode();
  fn _ZNK9QKeyEvent14nativeScanCodeEv(qthis: *mut c_void) -> c_uint;
}

// body block begin
// class sizeof(QKeyEvent)=1
pub struct QKeyEvent {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QKeyEvent {
  pub fn count<T: QKeyEvent_count>(&mut self, value: T) -> i32 {
    return value.count(self);
    // return 1;
  }
}

pub trait QKeyEvent_count {
  fn count(self, rsthis: &mut QKeyEvent) -> i32;
}

// proto:  int QKeyEvent::count();
impl<'a> /*trait*/ QKeyEvent_count for () {
  fn count(self, rsthis: &mut QKeyEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QKeyEvent5countEv()};
    let mut ret = unsafe {_ZNK9QKeyEvent5countEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QKeyEvent {
  pub fn FreeQKeyEvent<T: QKeyEvent_FreeQKeyEvent>(&mut self, value: T)  {
     value.FreeQKeyEvent(self);
    // return 1;
  }
}

pub trait QKeyEvent_FreeQKeyEvent {
  fn FreeQKeyEvent(self, rsthis: &mut QKeyEvent) ;
}

// proto:  void QKeyEvent::FreeQKeyEvent();
impl<'a> /*trait*/ QKeyEvent_FreeQKeyEvent for () {
  fn FreeQKeyEvent(self, rsthis: &mut QKeyEvent)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QKeyEventD0Ev()};
     unsafe {_ZN9QKeyEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QKeyEvent {
  pub fn text<T: QKeyEvent_text>(&mut self, value: T) -> QString {
    return value.text(self);
    // return 1;
  }
}

pub trait QKeyEvent_text {
  fn text(self, rsthis: &mut QKeyEvent) -> QString;
}

// proto:  QString QKeyEvent::text();
impl<'a> /*trait*/ QKeyEvent_text for () {
  fn text(self, rsthis: &mut QKeyEvent) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QKeyEvent4textEv()};
    let mut ret = unsafe {_ZNK9QKeyEvent4textEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QKeyEvent {
  pub fn nativeVirtualKey<T: QKeyEvent_nativeVirtualKey>(&mut self, value: T) -> u32 {
    return value.nativeVirtualKey(self);
    // return 1;
  }
}

pub trait QKeyEvent_nativeVirtualKey {
  fn nativeVirtualKey(self, rsthis: &mut QKeyEvent) -> u32;
}

// proto:  unsigned int QKeyEvent::nativeVirtualKey();
impl<'a> /*trait*/ QKeyEvent_nativeVirtualKey for () {
  fn nativeVirtualKey(self, rsthis: &mut QKeyEvent) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QKeyEvent16nativeVirtualKeyEv()};
    let mut ret = unsafe {_ZNK9QKeyEvent16nativeVirtualKeyEv(rsthis.qclsinst)};
    return ret as u32;
    // return 1;
  }
}

impl /*struct*/ QKeyEvent {
  pub fn isAutoRepeat<T: QKeyEvent_isAutoRepeat>(&mut self, value: T) -> i8 {
    return value.isAutoRepeat(self);
    // return 1;
  }
}

pub trait QKeyEvent_isAutoRepeat {
  fn isAutoRepeat(self, rsthis: &mut QKeyEvent) -> i8;
}

// proto:  bool QKeyEvent::isAutoRepeat();
impl<'a> /*trait*/ QKeyEvent_isAutoRepeat for () {
  fn isAutoRepeat(self, rsthis: &mut QKeyEvent) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QKeyEvent12isAutoRepeatEv()};
    let mut ret = unsafe {_ZNK9QKeyEvent12isAutoRepeatEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QKeyEvent {
  pub fn key<T: QKeyEvent_key>(&mut self, value: T) -> i32 {
    return value.key(self);
    // return 1;
  }
}

pub trait QKeyEvent_key {
  fn key(self, rsthis: &mut QKeyEvent) -> i32;
}

// proto:  int QKeyEvent::key();
impl<'a> /*trait*/ QKeyEvent_key for () {
  fn key(self, rsthis: &mut QKeyEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QKeyEvent3keyEv()};
    let mut ret = unsafe {_ZNK9QKeyEvent3keyEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QKeyEvent {
  pub fn nativeModifiers<T: QKeyEvent_nativeModifiers>(&mut self, value: T) -> u32 {
    return value.nativeModifiers(self);
    // return 1;
  }
}

pub trait QKeyEvent_nativeModifiers {
  fn nativeModifiers(self, rsthis: &mut QKeyEvent) -> u32;
}

// proto:  unsigned int QKeyEvent::nativeModifiers();
impl<'a> /*trait*/ QKeyEvent_nativeModifiers for () {
  fn nativeModifiers(self, rsthis: &mut QKeyEvent) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QKeyEvent15nativeModifiersEv()};
    let mut ret = unsafe {_ZNK9QKeyEvent15nativeModifiersEv(rsthis.qclsinst)};
    return ret as u32;
    // return 1;
  }
}

impl /*struct*/ QKeyEvent {
  pub fn nativeScanCode<T: QKeyEvent_nativeScanCode>(&mut self, value: T) -> u32 {
    return value.nativeScanCode(self);
    // return 1;
  }
}

pub trait QKeyEvent_nativeScanCode {
  fn nativeScanCode(self, rsthis: &mut QKeyEvent) -> u32;
}

// proto:  unsigned int QKeyEvent::nativeScanCode();
impl<'a> /*trait*/ QKeyEvent_nativeScanCode for () {
  fn nativeScanCode(self, rsthis: &mut QKeyEvent) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QKeyEvent14nativeScanCodeEv()};
    let mut ret = unsafe {_ZNK9QKeyEvent14nativeScanCodeEv(rsthis.qclsinst)};
    return ret as u32;
    // return 1;
  }
}

