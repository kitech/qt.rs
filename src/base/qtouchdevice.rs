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
  // proto:  void QTouchDevice::setName(const QString & name);
  fn _ZN12QTouchDevice7setNameERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QString QTouchDevice::name();
  fn _ZNK12QTouchDevice4nameEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTouchDevice::setMaximumTouchPoints(int max);
  fn _ZN12QTouchDevice21setMaximumTouchPointsEi(qthis: *mut c_void, arg0: c_int) ;
  // proto: static QList<const QTouchDevice *> QTouchDevice::devices();
  fn _ZN12QTouchDevice7devicesEv() ;
  // proto:  void QTouchDevice::NewQTouchDevice();
  fn _ZN12QTouchDeviceC1Ev(qthis: *mut c_void) ;
  // proto:  void QTouchDevice::FreeQTouchDevice();
  fn _ZN12QTouchDeviceD0Ev(qthis: *mut c_void) ;
  // proto:  int QTouchDevice::maximumTouchPoints();
  fn _ZNK12QTouchDevice18maximumTouchPointsEv(qthis: *mut c_void) -> c_int;
}

// body block begin
// class sizeof(QTouchDevice)=8
pub struct QTouchDevice {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTouchDevice {
  pub fn setName<RetType, T: QTouchDevice_setName<RetType>>(&mut self, value: T) -> RetType {
    return value.setName(self);
    // return 1;
  }
}

pub trait QTouchDevice_setName<RetType> {
  fn setName(self, rsthis: &mut QTouchDevice) -> RetType;
}

// proto:  void QTouchDevice::setName(const QString & name);
impl<'a> /*trait*/ QTouchDevice_setName<()> for (&'a  QString) {
  fn setName(self, rsthis: &mut QTouchDevice) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTouchDevice7setNameERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QTouchDevice7setNameERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTouchDevice {
  pub fn name<RetType, T: QTouchDevice_name<RetType>>(&mut self, value: T) -> RetType {
    return value.name(self);
    // return 1;
  }
}

pub trait QTouchDevice_name<RetType> {
  fn name(self, rsthis: &mut QTouchDevice) -> RetType;
}

// proto:  QString QTouchDevice::name();
impl<'a> /*trait*/ QTouchDevice_name<QString> for () {
  fn name(self, rsthis: &mut QTouchDevice) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTouchDevice4nameEv()};
    let mut ret = unsafe {_ZNK12QTouchDevice4nameEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTouchDevice {
  pub fn setMaximumTouchPoints<RetType, T: QTouchDevice_setMaximumTouchPoints<RetType>>(&mut self, value: T) -> RetType {
    return value.setMaximumTouchPoints(self);
    // return 1;
  }
}

pub trait QTouchDevice_setMaximumTouchPoints<RetType> {
  fn setMaximumTouchPoints(self, rsthis: &mut QTouchDevice) -> RetType;
}

// proto:  void QTouchDevice::setMaximumTouchPoints(int max);
impl<'a> /*trait*/ QTouchDevice_setMaximumTouchPoints<()> for (i32) {
  fn setMaximumTouchPoints(self, rsthis: &mut QTouchDevice) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTouchDevice21setMaximumTouchPointsEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN12QTouchDevice21setMaximumTouchPointsEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTouchDevice {
  pub fn devices<RetType, T: QTouchDevice_devices<RetType>>(&mut self, value: T) -> RetType {
    return value.devices(self);
    // return 1;
  }
}

pub trait QTouchDevice_devices<RetType> {
  fn devices(self, rsthis: &mut QTouchDevice) -> RetType;
}

// proto: static QList<const QTouchDevice *> QTouchDevice::devices();
impl<'a> /*trait*/ QTouchDevice_devices<()> for () {
  fn devices(self, rsthis: &mut QTouchDevice) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTouchDevice7devicesEv()};
     unsafe {_ZN12QTouchDevice7devicesEv()};
    // return 1;
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
  pub fn FreeQTouchDevice<RetType, T: QTouchDevice_FreeQTouchDevice<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQTouchDevice(self);
    // return 1;
  }
}

pub trait QTouchDevice_FreeQTouchDevice<RetType> {
  fn FreeQTouchDevice(self, rsthis: &mut QTouchDevice) -> RetType;
}

// proto:  void QTouchDevice::FreeQTouchDevice();
impl<'a> /*trait*/ QTouchDevice_FreeQTouchDevice<()> for () {
  fn FreeQTouchDevice(self, rsthis: &mut QTouchDevice) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTouchDeviceD0Ev()};
     unsafe {_ZN12QTouchDeviceD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTouchDevice {
  pub fn maximumTouchPoints<RetType, T: QTouchDevice_maximumTouchPoints<RetType>>(&mut self, value: T) -> RetType {
    return value.maximumTouchPoints(self);
    // return 1;
  }
}

pub trait QTouchDevice_maximumTouchPoints<RetType> {
  fn maximumTouchPoints(self, rsthis: &mut QTouchDevice) -> RetType;
}

// proto:  int QTouchDevice::maximumTouchPoints();
impl<'a> /*trait*/ QTouchDevice_maximumTouchPoints<i32> for () {
  fn maximumTouchPoints(self, rsthis: &mut QTouchDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTouchDevice18maximumTouchPointsEv()};
    let mut ret = unsafe {_ZNK12QTouchDevice18maximumTouchPointsEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

