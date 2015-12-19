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

// proto:  int QKeyEvent::count();
impl /*struct*/ QKeyEvent {
  pub fn count<RetType, T: QKeyEvent_count<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.count(self);
    // return 1;
  }
}

pub trait QKeyEvent_count<RetType> {
  fn count(self , rsthis: &mut QKeyEvent) -> RetType;
}

// proto:  int QKeyEvent::count();
impl<'a> /*trait*/ QKeyEvent_count<i32> for () {
  fn count(self , rsthis: &mut QKeyEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QKeyEvent5countEv()};
    let mut ret = unsafe {_ZNK9QKeyEvent5countEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto:  void QKeyEvent::FreeQKeyEvent();
impl /*struct*/ QKeyEvent {
  pub fn FreeQKeyEvent<RetType, T: QKeyEvent_FreeQKeyEvent<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.FreeQKeyEvent(self);
    // return 1;
  }
}

pub trait QKeyEvent_FreeQKeyEvent<RetType> {
  fn FreeQKeyEvent(self , rsthis: &mut QKeyEvent) -> RetType;
}

// proto:  void QKeyEvent::FreeQKeyEvent();
impl<'a> /*trait*/ QKeyEvent_FreeQKeyEvent<()> for () {
  fn FreeQKeyEvent(self , rsthis: &mut QKeyEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QKeyEventD0Ev()};
     unsafe {_ZN9QKeyEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  QString QKeyEvent::text();
impl /*struct*/ QKeyEvent {
  pub fn text<RetType, T: QKeyEvent_text<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.text(self);
    // return 1;
  }
}

pub trait QKeyEvent_text<RetType> {
  fn text(self , rsthis: &mut QKeyEvent) -> RetType;
}

// proto:  QString QKeyEvent::text();
impl<'a> /*trait*/ QKeyEvent_text<QString> for () {
  fn text(self , rsthis: &mut QKeyEvent) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QKeyEvent4textEv()};
    let mut ret = unsafe {_ZNK9QKeyEvent4textEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  unsigned int QKeyEvent::nativeVirtualKey();
impl /*struct*/ QKeyEvent {
  pub fn nativeVirtualKey<RetType, T: QKeyEvent_nativeVirtualKey<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.nativeVirtualKey(self);
    // return 1;
  }
}

pub trait QKeyEvent_nativeVirtualKey<RetType> {
  fn nativeVirtualKey(self , rsthis: &mut QKeyEvent) -> RetType;
}

// proto:  unsigned int QKeyEvent::nativeVirtualKey();
impl<'a> /*trait*/ QKeyEvent_nativeVirtualKey<u32> for () {
  fn nativeVirtualKey(self , rsthis: &mut QKeyEvent) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QKeyEvent16nativeVirtualKeyEv()};
    let mut ret = unsafe {_ZNK9QKeyEvent16nativeVirtualKeyEv(rsthis.qclsinst)};
    return ret as u32;
    // return 1;
  }
}

// proto:  bool QKeyEvent::isAutoRepeat();
impl /*struct*/ QKeyEvent {
  pub fn isAutoRepeat<RetType, T: QKeyEvent_isAutoRepeat<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.isAutoRepeat(self);
    // return 1;
  }
}

pub trait QKeyEvent_isAutoRepeat<RetType> {
  fn isAutoRepeat(self , rsthis: &mut QKeyEvent) -> RetType;
}

// proto:  bool QKeyEvent::isAutoRepeat();
impl<'a> /*trait*/ QKeyEvent_isAutoRepeat<i8> for () {
  fn isAutoRepeat(self , rsthis: &mut QKeyEvent) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QKeyEvent12isAutoRepeatEv()};
    let mut ret = unsafe {_ZNK9QKeyEvent12isAutoRepeatEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto:  int QKeyEvent::key();
impl /*struct*/ QKeyEvent {
  pub fn key<RetType, T: QKeyEvent_key<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.key(self);
    // return 1;
  }
}

pub trait QKeyEvent_key<RetType> {
  fn key(self , rsthis: &mut QKeyEvent) -> RetType;
}

// proto:  int QKeyEvent::key();
impl<'a> /*trait*/ QKeyEvent_key<i32> for () {
  fn key(self , rsthis: &mut QKeyEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QKeyEvent3keyEv()};
    let mut ret = unsafe {_ZNK9QKeyEvent3keyEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto:  unsigned int QKeyEvent::nativeModifiers();
impl /*struct*/ QKeyEvent {
  pub fn nativeModifiers<RetType, T: QKeyEvent_nativeModifiers<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.nativeModifiers(self);
    // return 1;
  }
}

pub trait QKeyEvent_nativeModifiers<RetType> {
  fn nativeModifiers(self , rsthis: &mut QKeyEvent) -> RetType;
}

// proto:  unsigned int QKeyEvent::nativeModifiers();
impl<'a> /*trait*/ QKeyEvent_nativeModifiers<u32> for () {
  fn nativeModifiers(self , rsthis: &mut QKeyEvent) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QKeyEvent15nativeModifiersEv()};
    let mut ret = unsafe {_ZNK9QKeyEvent15nativeModifiersEv(rsthis.qclsinst)};
    return ret as u32;
    // return 1;
  }
}

// proto:  unsigned int QKeyEvent::nativeScanCode();
impl /*struct*/ QKeyEvent {
  pub fn nativeScanCode<RetType, T: QKeyEvent_nativeScanCode<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.nativeScanCode(self);
    // return 1;
  }
}

pub trait QKeyEvent_nativeScanCode<RetType> {
  fn nativeScanCode(self , rsthis: &mut QKeyEvent) -> RetType;
}

// proto:  unsigned int QKeyEvent::nativeScanCode();
impl<'a> /*trait*/ QKeyEvent_nativeScanCode<u32> for () {
  fn nativeScanCode(self , rsthis: &mut QKeyEvent) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QKeyEvent14nativeScanCodeEv()};
    let mut ret = unsafe {_ZNK9QKeyEvent14nativeScanCodeEv(rsthis.qclsinst)};
    return ret as u32;
    // return 1;
  }
}

