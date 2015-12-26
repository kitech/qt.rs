// auto generated, do not modify.
// created: Sat Dec 26 10:16:52 2015
// src-file: /QtGui/qpaintdevice.h
// dst-file: /src/gui/qpaintdevice.rs
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
use super::qpaintengine::QPaintEngine; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QPaintDevice_Class_Size() -> c_int;
  // proto:  int QPaintDevice::devType();
  fn _ZNK12QPaintDevice7devTypeEv(qthis: *mut c_void) -> c_int;
  // proto:  QPaintEngine * QPaintDevice::paintEngine();
  fn _ZNK12QPaintDevice11paintEngineEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QPaintDevice::QPaintDevice(const QPaintDevice & );
  fn dector_ZN12QPaintDeviceC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN12QPaintDeviceC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QPaintDevice::QPaintDevice();
  fn dector_ZN12QPaintDeviceC1Ev() -> *mut c_void;
  fn _ZN12QPaintDeviceC1Ev(qthis: *mut c_void);
  // proto:  bool QPaintDevice::paintingActive();
  fn _ZNK12QPaintDevice14paintingActiveEv(qthis: *mut c_void) -> c_char;
  // proto:  void QPaintDevice::~QPaintDevice();
  fn _ZN12QPaintDeviceD0Ev(qthis: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QPaintDevice)=24
pub struct QPaintDevice {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QPaintDevice {
  pub fn inheritFrom(qthis: *mut c_void) -> QPaintDevice {
    return QPaintDevice{qclsinst: qthis};
  }
}
  // proto:  int QPaintDevice::devType();
impl /*struct*/ QPaintDevice {
  pub fn devType<RetType, T: QPaintDevice_devType<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.devType(self);
    // return 1;
  }
}

pub trait QPaintDevice_devType<RetType> {
  fn devType(self , rsthis: & QPaintDevice) -> RetType;
}

  // proto:  int QPaintDevice::devType();
impl<'a> /*trait*/ QPaintDevice_devType<i32> for () {
  fn devType(self , rsthis: & QPaintDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPaintDevice7devTypeEv()};
    let mut ret = unsafe {_ZNK12QPaintDevice7devTypeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QPaintEngine * QPaintDevice::paintEngine();
impl /*struct*/ QPaintDevice {
  pub fn paintEngine<RetType, T: QPaintDevice_paintEngine<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.paintEngine(self);
    // return 1;
  }
}

pub trait QPaintDevice_paintEngine<RetType> {
  fn paintEngine(self , rsthis: & QPaintDevice) -> RetType;
}

  // proto:  QPaintEngine * QPaintDevice::paintEngine();
impl<'a> /*trait*/ QPaintDevice_paintEngine<QPaintEngine> for () {
  fn paintEngine(self , rsthis: & QPaintDevice) -> QPaintEngine {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPaintDevice11paintEngineEv()};
    let mut ret = unsafe {_ZNK12QPaintDevice11paintEngineEv(rsthis.qclsinst)};
    let mut ret1 = QPaintEngine::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QPaintDevice::QPaintDevice(const QPaintDevice & );
impl /*struct*/ QPaintDevice {
  pub fn New<T: QPaintDevice_New>(value: T) -> QPaintDevice {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QPaintDevice_New {
  fn New(self) -> QPaintDevice;
}

  // proto:  void QPaintDevice::QPaintDevice(const QPaintDevice & );
impl<'a> /*trait*/ QPaintDevice_New for (&'a QPaintDevice) {
  fn New(self) -> QPaintDevice {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPaintDeviceC1ERKS_()};
    let ctysz: c_int = unsafe{QPaintDevice_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN12QPaintDeviceC1ERKS_(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN12QPaintDeviceC1ERKS_(arg0)};
    let rsthis = QPaintDevice{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QPaintDevice::QPaintDevice();
impl<'a> /*trait*/ QPaintDevice_New for () {
  fn New(self) -> QPaintDevice {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPaintDeviceC1Ev()};
    let ctysz: c_int = unsafe{QPaintDevice_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    // unsafe {_ZN12QPaintDeviceC1Ev(qthis)};
    let qthis: *mut c_void = unsafe {dector_ZN12QPaintDeviceC1Ev()};
    let rsthis = QPaintDevice{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QPaintDevice::paintingActive();
impl /*struct*/ QPaintDevice {
  pub fn paintingActive<RetType, T: QPaintDevice_paintingActive<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.paintingActive(self);
    // return 1;
  }
}

pub trait QPaintDevice_paintingActive<RetType> {
  fn paintingActive(self , rsthis: & QPaintDevice) -> RetType;
}

  // proto:  bool QPaintDevice::paintingActive();
impl<'a> /*trait*/ QPaintDevice_paintingActive<i8> for () {
  fn paintingActive(self , rsthis: & QPaintDevice) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPaintDevice14paintingActiveEv()};
    let mut ret = unsafe {_ZNK12QPaintDevice14paintingActiveEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QPaintDevice::~QPaintDevice();
impl /*struct*/ QPaintDevice {
  pub fn Free<RetType, T: QPaintDevice_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QPaintDevice_Free<RetType> {
  fn Free(self , rsthis: & QPaintDevice) -> RetType;
}

  // proto:  void QPaintDevice::~QPaintDevice();
impl<'a> /*trait*/ QPaintDevice_Free<()> for () {
  fn Free(self , rsthis: & QPaintDevice) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPaintDeviceD0Ev()};
     unsafe {_ZN12QPaintDeviceD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// <= body block end

