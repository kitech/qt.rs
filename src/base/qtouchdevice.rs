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
  // proto: void QTouchDevice::setName(const QString & name);
  fn _ZN12QTouchDevice7setNameERK7QString(arg0: *const c_void) -> i32;
  // proto: QString QTouchDevice::name();
  fn _ZNK12QTouchDevice4nameEv() -> i32;
  // proto: void QTouchDevice::setMaximumTouchPoints(int max);
  fn _ZN12QTouchDevice21setMaximumTouchPointsEi(arg0: c_int) -> i32;
  // proto: QList<const QTouchDevice *> QTouchDevice::devices();
  fn _ZN12QTouchDevice7devicesEv() -> i32;
  // proto: void QTouchDevice::NewQTouchDevice();
  fn _ZN12QTouchDeviceC1Ev(qthis: *mut c_void) -> i32;
  // proto: void QTouchDevice::FreeQTouchDevice();
  fn _ZN12QTouchDeviceD0Ev() -> i32;
  // proto: int QTouchDevice::maximumTouchPoints();
  fn _ZNK12QTouchDevice18maximumTouchPointsEv() -> i32;
}

// body block begin
// class sizeof(QTouchDevice)=8
pub struct QTouchDevice {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTouchDevice {
  pub fn setName<T: QTouchDevice_setName>(&mut self, value: T) -> i32 {
    value.setName(self);
    return 1;
  }
}

pub trait QTouchDevice_setName {
  fn setName(self, this: &mut QTouchDevice) -> i32;
}

// proto: void QTouchDevice::setName(const QString & name);
impl<'a> /*trait*/ QTouchDevice_setName for (&'a  QString) {
  fn setName(self, this: &mut QTouchDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTouchDevice7setNameERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN12QTouchDevice7setNameERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QTouchDevice {
  pub fn name<T: QTouchDevice_name>(&mut self, value: T) -> i32 {
    value.name(self);
    return 1;
  }
}

pub trait QTouchDevice_name {
  fn name(self, this: &mut QTouchDevice) -> i32;
}

// proto: QString QTouchDevice::name();
impl<'a> /*trait*/ QTouchDevice_name for () {
  fn name(self, this: &mut QTouchDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTouchDevice4nameEv()};
    unsafe {_ZNK12QTouchDevice4nameEv()};
    return 1;
  }
}

impl /*struct*/ QTouchDevice {
  pub fn setMaximumTouchPoints<T: QTouchDevice_setMaximumTouchPoints>(&mut self, value: T) -> i32 {
    value.setMaximumTouchPoints(self);
    return 1;
  }
}

pub trait QTouchDevice_setMaximumTouchPoints {
  fn setMaximumTouchPoints(self, this: &mut QTouchDevice) -> i32;
}

// proto: void QTouchDevice::setMaximumTouchPoints(int max);
impl<'a> /*trait*/ QTouchDevice_setMaximumTouchPoints for (i32) {
  fn setMaximumTouchPoints(self, this: &mut QTouchDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTouchDevice21setMaximumTouchPointsEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN12QTouchDevice21setMaximumTouchPointsEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTouchDevice {
  pub fn devices<T: QTouchDevice_devices>(&mut self, value: T) -> i32 {
    value.devices(self);
    return 1;
  }
}

pub trait QTouchDevice_devices {
  fn devices(self, this: &mut QTouchDevice) -> i32;
}

// proto: QList<const QTouchDevice *> QTouchDevice::devices();
impl<'a> /*trait*/ QTouchDevice_devices for () {
  fn devices(self, this: &mut QTouchDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTouchDevice7devicesEv()};
    unsafe {_ZN12QTouchDevice7devicesEv()};
    return 1;
  }
}

impl /*struct*/ QTouchDevice {
  pub fn NewQTouchDevice<T: QTouchDevice_NewQTouchDevice>(value: T) -> QTouchDevice {
    let rsthis = value.NewQTouchDevice();
    return rsthis;
    // return 1;
  }
}

pub trait QTouchDevice_NewQTouchDevice {
  fn NewQTouchDevice(self) -> QTouchDevice;
}

// proto: void QTouchDevice::NewQTouchDevice();
impl<'a> /*trait*/ QTouchDevice_NewQTouchDevice for () {
  fn NewQTouchDevice(self) -> QTouchDevice {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTouchDeviceC1Ev()};
    unsafe {_ZN12QTouchDeviceC1Ev(qthis)};
    let rsthis = QTouchDevice{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTouchDevice {
  pub fn FreeQTouchDevice<T: QTouchDevice_FreeQTouchDevice>(&mut self, value: T) -> i32 {
    value.FreeQTouchDevice(self);
    return 1;
  }
}

pub trait QTouchDevice_FreeQTouchDevice {
  fn FreeQTouchDevice(self, this: &mut QTouchDevice) -> i32;
}

// proto: void QTouchDevice::FreeQTouchDevice();
impl<'a> /*trait*/ QTouchDevice_FreeQTouchDevice for () {
  fn FreeQTouchDevice(self, this: &mut QTouchDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTouchDeviceD0Ev()};
    unsafe {_ZN12QTouchDeviceD0Ev()};
    return 1;
  }
}

impl /*struct*/ QTouchDevice {
  pub fn maximumTouchPoints<T: QTouchDevice_maximumTouchPoints>(&mut self, value: T) -> i32 {
    value.maximumTouchPoints(self);
    return 1;
  }
}

pub trait QTouchDevice_maximumTouchPoints {
  fn maximumTouchPoints(self, this: &mut QTouchDevice) -> i32;
}

// proto: int QTouchDevice::maximumTouchPoints();
impl<'a> /*trait*/ QTouchDevice_maximumTouchPoints for () {
  fn maximumTouchPoints(self, this: &mut QTouchDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTouchDevice18maximumTouchPointsEv()};
    unsafe {_ZNK12QTouchDevice18maximumTouchPointsEv()};
    return 1;
  }
}

