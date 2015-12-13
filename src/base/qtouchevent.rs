// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qtouchdevice::QTouchDevice;
use super::qwindow::QWindow;
use super::qobject::QObject;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QTouchEvent::setDevice(QTouchDevice * adevice);
  fn _ZN11QTouchEvent9setDeviceEP12QTouchDevice(arg0: *mut c_void) -> i32;
  // proto: QWindow * QTouchEvent::window();
  fn _ZNK11QTouchEvent6windowEv() -> i32;
  // proto: QTouchDevice * QTouchEvent::device();
  fn _ZNK11QTouchEvent6deviceEv() -> i32;
  // proto: QObject * QTouchEvent::target();
  fn _ZNK11QTouchEvent6targetEv() -> i32;
  // proto: void QTouchEvent::FreeQTouchEvent();
  fn _ZN11QTouchEventD0Ev() -> i32;
  // proto: void QTouchEvent::setWindow(QWindow * awindow);
  fn _ZN11QTouchEvent9setWindowEP7QWindow(arg0: *mut c_void) -> i32;
  // proto: void QTouchEvent::setTarget(QObject * atarget);
  fn _ZN11QTouchEvent9setTargetEP7QObject(arg0: *mut c_void) -> i32;
}

// body block begin
// class sizeof(QTouchEvent)=1
pub struct QTouchEvent {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTouchEvent {
  pub fn setDevice<T: QTouchEvent_setDevice>(&mut self, value: T) -> i32 {
    value.setDevice(self);
    return 1;
  }
}

pub trait QTouchEvent_setDevice {
  fn setDevice(self, this: &mut QTouchEvent) -> i32;
}

// proto: void QTouchEvent::setDevice(QTouchDevice * adevice);
impl<'a> /*trait*/ QTouchEvent_setDevice for (&'a mut QTouchDevice) {
  fn setDevice(self, this: &mut QTouchEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTouchEvent9setDeviceEP12QTouchDevice()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QTouchEvent9setDeviceEP12QTouchDevice(arg0)};
    return 1;
  }
}

impl /*struct*/ QTouchEvent {
  pub fn window<T: QTouchEvent_window>(&mut self, value: T) -> i32 {
    value.window(self);
    return 1;
  }
}

pub trait QTouchEvent_window {
  fn window(self, this: &mut QTouchEvent) -> i32;
}

// proto: QWindow * QTouchEvent::window();
impl<'a> /*trait*/ QTouchEvent_window for () {
  fn window(self, this: &mut QTouchEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTouchEvent6windowEv()};
    unsafe {_ZNK11QTouchEvent6windowEv()};
    return 1;
  }
}

impl /*struct*/ QTouchEvent {
  pub fn device<T: QTouchEvent_device>(&mut self, value: T) -> i32 {
    value.device(self);
    return 1;
  }
}

pub trait QTouchEvent_device {
  fn device(self, this: &mut QTouchEvent) -> i32;
}

// proto: QTouchDevice * QTouchEvent::device();
impl<'a> /*trait*/ QTouchEvent_device for () {
  fn device(self, this: &mut QTouchEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTouchEvent6deviceEv()};
    unsafe {_ZNK11QTouchEvent6deviceEv()};
    return 1;
  }
}

impl /*struct*/ QTouchEvent {
  pub fn target<T: QTouchEvent_target>(&mut self, value: T) -> i32 {
    value.target(self);
    return 1;
  }
}

pub trait QTouchEvent_target {
  fn target(self, this: &mut QTouchEvent) -> i32;
}

// proto: QObject * QTouchEvent::target();
impl<'a> /*trait*/ QTouchEvent_target for () {
  fn target(self, this: &mut QTouchEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTouchEvent6targetEv()};
    unsafe {_ZNK11QTouchEvent6targetEv()};
    return 1;
  }
}

impl /*struct*/ QTouchEvent {
  pub fn FreeQTouchEvent<T: QTouchEvent_FreeQTouchEvent>(&mut self, value: T) -> i32 {
    value.FreeQTouchEvent(self);
    return 1;
  }
}

pub trait QTouchEvent_FreeQTouchEvent {
  fn FreeQTouchEvent(self, this: &mut QTouchEvent) -> i32;
}

// proto: void QTouchEvent::FreeQTouchEvent();
impl<'a> /*trait*/ QTouchEvent_FreeQTouchEvent for () {
  fn FreeQTouchEvent(self, this: &mut QTouchEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTouchEventD0Ev()};
    unsafe {_ZN11QTouchEventD0Ev()};
    return 1;
  }
}

impl /*struct*/ QTouchEvent {
  pub fn setWindow<T: QTouchEvent_setWindow>(&mut self, value: T) -> i32 {
    value.setWindow(self);
    return 1;
  }
}

pub trait QTouchEvent_setWindow {
  fn setWindow(self, this: &mut QTouchEvent) -> i32;
}

// proto: void QTouchEvent::setWindow(QWindow * awindow);
impl<'a> /*trait*/ QTouchEvent_setWindow for (&'a mut QWindow) {
  fn setWindow(self, this: &mut QTouchEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTouchEvent9setWindowEP7QWindow()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QTouchEvent9setWindowEP7QWindow(arg0)};
    return 1;
  }
}

impl /*struct*/ QTouchEvent {
  pub fn setTarget<T: QTouchEvent_setTarget>(&mut self, value: T) -> i32 {
    value.setTarget(self);
    return 1;
  }
}

pub trait QTouchEvent_setTarget {
  fn setTarget(self, this: &mut QTouchEvent) -> i32;
}

// proto: void QTouchEvent::setTarget(QObject * atarget);
impl<'a> /*trait*/ QTouchEvent_setTarget for (&'a mut QObject) {
  fn setTarget(self, this: &mut QTouchEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTouchEvent9setTargetEP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QTouchEvent9setTargetEP7QObject(arg0)};
    return 1;
  }
}

