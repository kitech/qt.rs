// auto generated, do not modify.
// created: Thu Jan 28 22:38:45 2016
// src-file: /QtGui/qtouchdevice.h
// dst-file: /src/gui/qtouchdevice.rs
//

// header block begin =>
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;


// <= header block end

// main block begin =>
// <= main block end

// use block begin =>
use std::ops::Deref;
use super::super::core::qstring::*; // 771
// use super::qlist::*; // 775
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QTouchDevice_Class_Size() -> c_int;
  // proto:  void QTouchDevice::setName(const QString & name);
  fn C_ZN12QTouchDevice7setNameERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QString QTouchDevice::name();
  fn C_ZNK12QTouchDevice4nameEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QTouchDevice::setMaximumTouchPoints(int max);
  fn C_ZN12QTouchDevice21setMaximumTouchPointsEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto: static QList<const QTouchDevice *> QTouchDevice::devices();
  fn C_ZN12QTouchDevice7devicesEv() -> *mut c_void;
  // proto:  void QTouchDevice::QTouchDevice();
  fn C_ZN12QTouchDeviceC2Ev() -> u64;
  // proto:  void QTouchDevice::~QTouchDevice();
  fn C_ZN12QTouchDeviceD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  int QTouchDevice::maximumTouchPoints();
  fn C_ZNK12QTouchDevice18maximumTouchPointsEv(qthis: u64 /* *mut c_void*/) -> c_int;
} // <= ext block end

// body block begin =>
// class sizeof(QTouchDevice)=8
#[derive(Default)]
pub struct QTouchDevice {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QTouchDevice {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QTouchDevice {
    return QTouchDevice{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void QTouchDevice::setName(const QString & name);
impl /*struct*/ QTouchDevice {
  pub fn setName<RetType, T: QTouchDevice_setName<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setName(self);
    // return 1;
  }
}

pub trait QTouchDevice_setName<RetType> {
  fn setName(self , rsthis: & QTouchDevice) -> RetType;
}

  // proto:  void QTouchDevice::setName(const QString & name);
impl<'a> /*trait*/ QTouchDevice_setName<()> for (&'a QString) {
  fn setName(self , rsthis: & QTouchDevice) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTouchDevice7setNameERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN12QTouchDevice7setNameERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QString QTouchDevice::name();
impl /*struct*/ QTouchDevice {
  pub fn name<RetType, T: QTouchDevice_name<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.name(self);
    // return 1;
  }
}

pub trait QTouchDevice_name<RetType> {
  fn name(self , rsthis: & QTouchDevice) -> RetType;
}

  // proto:  QString QTouchDevice::name();
impl<'a> /*trait*/ QTouchDevice_name<QString> for () {
  fn name(self , rsthis: & QTouchDevice) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTouchDevice4nameEv()};
    let mut ret = unsafe {C_ZNK12QTouchDevice4nameEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTouchDevice::setMaximumTouchPoints(int max);
impl /*struct*/ QTouchDevice {
  pub fn setMaximumTouchPoints<RetType, T: QTouchDevice_setMaximumTouchPoints<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setMaximumTouchPoints(self);
    // return 1;
  }
}

pub trait QTouchDevice_setMaximumTouchPoints<RetType> {
  fn setMaximumTouchPoints(self , rsthis: & QTouchDevice) -> RetType;
}

  // proto:  void QTouchDevice::setMaximumTouchPoints(int max);
impl<'a> /*trait*/ QTouchDevice_setMaximumTouchPoints<()> for (i32) {
  fn setMaximumTouchPoints(self , rsthis: & QTouchDevice) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTouchDevice21setMaximumTouchPointsEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN12QTouchDevice21setMaximumTouchPointsEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto: static QList<const QTouchDevice *> QTouchDevice::devices();
impl /*struct*/ QTouchDevice {
  pub fn devices_s<RetType, T: QTouchDevice_devices_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.devices_s();
    // return 1;
  }
}

pub trait QTouchDevice_devices_s<RetType> {
  fn devices_s(self ) -> RetType;
}

  // proto: static QList<const QTouchDevice *> QTouchDevice::devices();
impl<'a> /*trait*/ QTouchDevice_devices_s<u64> for () {
  fn devices_s(self ) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTouchDevice7devicesEv()};
    let mut ret = unsafe {C_ZN12QTouchDevice7devicesEv()};
    return ret as u64; // 5
    // return 1;
  }
}

  // proto:  void QTouchDevice::QTouchDevice();
impl /*struct*/ QTouchDevice {
  pub fn new<T: QTouchDevice_new>(value: T) -> QTouchDevice {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QTouchDevice_new {
  fn new(self) -> QTouchDevice;
}

  // proto:  void QTouchDevice::QTouchDevice();
impl<'a> /*trait*/ QTouchDevice_new for () {
  fn new(self) -> QTouchDevice {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTouchDeviceC2Ev()};
    let ctysz: c_int = unsafe{QTouchDevice_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let qthis: u64 = unsafe {C_ZN12QTouchDeviceC2Ev()};
    let rsthis = QTouchDevice{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QTouchDevice::~QTouchDevice();
impl /*struct*/ QTouchDevice {
  pub fn free<RetType, T: QTouchDevice_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QTouchDevice_free<RetType> {
  fn free(self , rsthis: & QTouchDevice) -> RetType;
}

  // proto:  void QTouchDevice::~QTouchDevice();
impl<'a> /*trait*/ QTouchDevice_free<()> for () {
  fn free(self , rsthis: & QTouchDevice) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTouchDeviceD2Ev()};
     unsafe {C_ZN12QTouchDeviceD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QTouchDevice::maximumTouchPoints();
impl /*struct*/ QTouchDevice {
  pub fn maximumTouchPoints<RetType, T: QTouchDevice_maximumTouchPoints<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.maximumTouchPoints(self);
    // return 1;
  }
}

pub trait QTouchDevice_maximumTouchPoints<RetType> {
  fn maximumTouchPoints(self , rsthis: & QTouchDevice) -> RetType;
}

  // proto:  int QTouchDevice::maximumTouchPoints();
impl<'a> /*trait*/ QTouchDevice_maximumTouchPoints<i32> for () {
  fn maximumTouchPoints(self , rsthis: & QTouchDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTouchDevice18maximumTouchPointsEv()};
    let mut ret = unsafe {C_ZNK12QTouchDevice18maximumTouchPointsEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

// <= body block end

