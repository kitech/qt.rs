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
  // proto:  void QTouchEvent::setDevice(QTouchDevice * adevice);
  fn _ZN11QTouchEvent9setDeviceEP12QTouchDevice(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QWindow * QTouchEvent::window();
  fn _ZNK11QTouchEvent6windowEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QTouchDevice * QTouchEvent::device();
  fn _ZNK11QTouchEvent6deviceEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QObject * QTouchEvent::target();
  fn _ZNK11QTouchEvent6targetEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTouchEvent::FreeQTouchEvent();
  fn _ZN11QTouchEventD0Ev(qthis: *mut c_void) ;
  // proto:  void QTouchEvent::setWindow(QWindow * awindow);
  fn _ZN11QTouchEvent9setWindowEP7QWindow(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTouchEvent::setTarget(QObject * atarget);
  fn _ZN11QTouchEvent9setTargetEP7QObject(qthis: *mut c_void, arg0: *mut c_void) ;
}

// body block begin
// class sizeof(QTouchEvent)=1
pub struct QTouchEvent {
  pub qclsinst: *mut c_void,
}

// proto:  void QTouchEvent::setDevice(QTouchDevice * adevice);
impl /*struct*/ QTouchEvent {
  pub fn setDevice<RetType, T: QTouchEvent_setDevice<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setDevice(self);
    // return 1;
  }
}

pub trait QTouchEvent_setDevice<RetType> {
  fn setDevice(self , rsthis: &mut QTouchEvent) -> RetType;
}

// proto:  void QTouchEvent::setDevice(QTouchDevice * adevice);
impl<'a> /*trait*/ QTouchEvent_setDevice<()> for (&'a mut QTouchDevice) {
  fn setDevice(self , rsthis: &mut QTouchEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTouchEvent9setDeviceEP12QTouchDevice()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QTouchEvent9setDeviceEP12QTouchDevice(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  QWindow * QTouchEvent::window();
impl /*struct*/ QTouchEvent {
  pub fn window<RetType, T: QTouchEvent_window<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.window(self);
    // return 1;
  }
}

pub trait QTouchEvent_window<RetType> {
  fn window(self , rsthis: &mut QTouchEvent) -> RetType;
}

// proto:  QWindow * QTouchEvent::window();
impl<'a> /*trait*/ QTouchEvent_window<QWindow> for () {
  fn window(self , rsthis: &mut QTouchEvent) -> QWindow {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTouchEvent6windowEv()};
    let mut ret = unsafe {_ZNK11QTouchEvent6windowEv(rsthis.qclsinst)};
    let mut ret1 = QWindow{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QTouchDevice * QTouchEvent::device();
impl /*struct*/ QTouchEvent {
  pub fn device<RetType, T: QTouchEvent_device<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.device(self);
    // return 1;
  }
}

pub trait QTouchEvent_device<RetType> {
  fn device(self , rsthis: &mut QTouchEvent) -> RetType;
}

// proto:  QTouchDevice * QTouchEvent::device();
impl<'a> /*trait*/ QTouchEvent_device<QTouchDevice> for () {
  fn device(self , rsthis: &mut QTouchEvent) -> QTouchDevice {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTouchEvent6deviceEv()};
    let mut ret = unsafe {_ZNK11QTouchEvent6deviceEv(rsthis.qclsinst)};
    let mut ret1 = QTouchDevice{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QObject * QTouchEvent::target();
impl /*struct*/ QTouchEvent {
  pub fn target<RetType, T: QTouchEvent_target<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.target(self);
    // return 1;
  }
}

pub trait QTouchEvent_target<RetType> {
  fn target(self , rsthis: &mut QTouchEvent) -> RetType;
}

// proto:  QObject * QTouchEvent::target();
impl<'a> /*trait*/ QTouchEvent_target<QObject> for () {
  fn target(self , rsthis: &mut QTouchEvent) -> QObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTouchEvent6targetEv()};
    let mut ret = unsafe {_ZNK11QTouchEvent6targetEv(rsthis.qclsinst)};
    let mut ret1 = QObject{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QTouchEvent::FreeQTouchEvent();
impl /*struct*/ QTouchEvent {
  pub fn FreeQTouchEvent<RetType, T: QTouchEvent_FreeQTouchEvent<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.FreeQTouchEvent(self);
    // return 1;
  }
}

pub trait QTouchEvent_FreeQTouchEvent<RetType> {
  fn FreeQTouchEvent(self , rsthis: &mut QTouchEvent) -> RetType;
}

// proto:  void QTouchEvent::FreeQTouchEvent();
impl<'a> /*trait*/ QTouchEvent_FreeQTouchEvent<()> for () {
  fn FreeQTouchEvent(self , rsthis: &mut QTouchEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTouchEventD0Ev()};
     unsafe {_ZN11QTouchEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  void QTouchEvent::setWindow(QWindow * awindow);
impl /*struct*/ QTouchEvent {
  pub fn setWindow<RetType, T: QTouchEvent_setWindow<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setWindow(self);
    // return 1;
  }
}

pub trait QTouchEvent_setWindow<RetType> {
  fn setWindow(self , rsthis: &mut QTouchEvent) -> RetType;
}

// proto:  void QTouchEvent::setWindow(QWindow * awindow);
impl<'a> /*trait*/ QTouchEvent_setWindow<()> for (&'a mut QWindow) {
  fn setWindow(self , rsthis: &mut QTouchEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTouchEvent9setWindowEP7QWindow()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QTouchEvent9setWindowEP7QWindow(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QTouchEvent::setTarget(QObject * atarget);
impl /*struct*/ QTouchEvent {
  pub fn setTarget<RetType, T: QTouchEvent_setTarget<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setTarget(self);
    // return 1;
  }
}

pub trait QTouchEvent_setTarget<RetType> {
  fn setTarget(self , rsthis: &mut QTouchEvent) -> RetType;
}

// proto:  void QTouchEvent::setTarget(QObject * atarget);
impl<'a> /*trait*/ QTouchEvent_setTarget<()> for (&'a mut QObject) {
  fn setTarget(self , rsthis: &mut QTouchEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTouchEvent9setTargetEP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QTouchEvent9setTargetEP7QObject(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

