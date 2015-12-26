// auto generated, do not modify.
// created: Sat Dec 26 12:15:38 2015
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
use super::super::core::qstring::QString; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QTouchDevice_Class_Size() -> c_int;
  // proto:  void QTouchDevice::setName(const QString & name);
  fn _ZN12QTouchDevice7setNameERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QString QTouchDevice::name();
  fn _ZNK12QTouchDevice4nameEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTouchDevice::setMaximumTouchPoints(int max);
  fn _ZN12QTouchDevice21setMaximumTouchPointsEi(qthis: *mut c_void, arg0: c_int);
  // proto: static QList<const QTouchDevice *> QTouchDevice::devices();
  fn _ZN12QTouchDevice7devicesEv();
  // proto:  void QTouchDevice::QTouchDevice();
  fn dector_ZN12QTouchDeviceC1Ev() -> *mut c_void;
  fn _ZN12QTouchDeviceC1Ev(qthis: *mut c_void);
  // proto:  void QTouchDevice::~QTouchDevice();
  fn _ZN12QTouchDeviceD0Ev(qthis: *mut c_void);
  // proto:  int QTouchDevice::maximumTouchPoints();
  fn _ZNK12QTouchDevice18maximumTouchPointsEv(qthis: *mut c_void) -> c_int;
} // <= ext block end

// body block begin =>
// class sizeof(QTouchDevice)=8
pub struct QTouchDevice {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTouchDevice {
  pub fn inheritFrom(qthis: *mut c_void) -> QTouchDevice {
    return QTouchDevice{qclsinst: qthis};
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
     unsafe {_ZN12QTouchDevice7setNameERK7QString(rsthis.qclsinst, arg0)};
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
    let mut ret = unsafe {_ZNK12QTouchDevice4nameEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret);
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
     unsafe {_ZN12QTouchDevice21setMaximumTouchPointsEi(rsthis.qclsinst, arg0)};
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
impl<'a> /*trait*/ QTouchDevice_devices_s<()> for () {
  fn devices_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTouchDevice7devicesEv()};
     unsafe {_ZN12QTouchDevice7devicesEv()};
    // return 1;
  }
}

  // proto:  void QTouchDevice::QTouchDevice();
impl /*struct*/ QTouchDevice {
  pub fn New<T: QTouchDevice_New>(value: T) -> QTouchDevice {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QTouchDevice_New {
  fn New(self) -> QTouchDevice;
}

  // proto:  void QTouchDevice::QTouchDevice();
impl<'a> /*trait*/ QTouchDevice_New for () {
  fn New(self) -> QTouchDevice {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTouchDeviceC1Ev()};
    let ctysz: c_int = unsafe{QTouchDevice_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    // unsafe {_ZN12QTouchDeviceC1Ev(qthis)};
    let qthis: *mut c_void = unsafe {dector_ZN12QTouchDeviceC1Ev()};
    let rsthis = QTouchDevice{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QTouchDevice::~QTouchDevice();
impl /*struct*/ QTouchDevice {
  pub fn Free<RetType, T: QTouchDevice_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QTouchDevice_Free<RetType> {
  fn Free(self , rsthis: & QTouchDevice) -> RetType;
}

  // proto:  void QTouchDevice::~QTouchDevice();
impl<'a> /*trait*/ QTouchDevice_Free<()> for () {
  fn Free(self , rsthis: & QTouchDevice) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTouchDeviceD0Ev()};
     unsafe {_ZN12QTouchDeviceD0Ev(rsthis.qclsinst)};
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
    let mut ret = unsafe {_ZNK12QTouchDevice18maximumTouchPointsEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// <= body block end

