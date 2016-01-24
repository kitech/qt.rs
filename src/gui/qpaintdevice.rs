// auto generated, do not modify.
// created: Sun Jan 24 17:41:38 2016
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
  // proto:  int QPaintDevice::physicalDpiY();
  fn C_ZNK12QPaintDevice12physicalDpiYEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  int QPaintDevice::heightMM();
  fn C_ZNK12QPaintDevice8heightMMEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  int QPaintDevice::colorCount();
  fn C_ZNK12QPaintDevice10colorCountEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  int QPaintDevice::physicalDpiX();
  fn C_ZNK12QPaintDevice12physicalDpiXEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  int QPaintDevice::widthMM();
  fn C_ZNK12QPaintDevice7widthMMEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  int QPaintDevice::devType();
  fn C_ZNK12QPaintDevice7devTypeEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  bool QPaintDevice::paintingActive();
  fn C_ZNK12QPaintDevice14paintingActiveEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  int QPaintDevice::width();
  fn C_ZNK12QPaintDevice5widthEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  int QPaintDevice::devicePixelRatio();
  fn C_ZNK12QPaintDevice16devicePixelRatioEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  int QPaintDevice::height();
  fn C_ZNK12QPaintDevice6heightEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  int QPaintDevice::depth();
  fn C_ZNK12QPaintDevice5depthEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  QPaintEngine * QPaintDevice::paintEngine();
  fn C_ZNK12QPaintDevice11paintEngineEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  int QPaintDevice::logicalDpiY();
  fn C_ZNK12QPaintDevice11logicalDpiYEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QPaintDevice::~QPaintDevice();
  fn C_ZN12QPaintDeviceD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  int QPaintDevice::logicalDpiX();
  fn C_ZNK12QPaintDevice11logicalDpiXEv(qthis: u64 /* *mut c_void*/) -> c_int;
} // <= ext block end

// body block begin =>
// class sizeof(QPaintDevice)=24
#[derive(Default)]
pub struct QPaintDevice {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QPaintDevice {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QPaintDevice {
    return QPaintDevice{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  int QPaintDevice::physicalDpiY();
impl /*struct*/ QPaintDevice {
  pub fn physicalDpiY<RetType, T: QPaintDevice_physicalDpiY<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.physicalDpiY(self);
    // return 1;
  }
}

pub trait QPaintDevice_physicalDpiY<RetType> {
  fn physicalDpiY(self , rsthis: & QPaintDevice) -> RetType;
}

  // proto:  int QPaintDevice::physicalDpiY();
impl<'a> /*trait*/ QPaintDevice_physicalDpiY<i32> for () {
  fn physicalDpiY(self , rsthis: & QPaintDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPaintDevice12physicalDpiYEv()};
    let mut ret = unsafe {C_ZNK12QPaintDevice12physicalDpiYEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QPaintDevice::heightMM();
impl /*struct*/ QPaintDevice {
  pub fn heightMM<RetType, T: QPaintDevice_heightMM<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.heightMM(self);
    // return 1;
  }
}

pub trait QPaintDevice_heightMM<RetType> {
  fn heightMM(self , rsthis: & QPaintDevice) -> RetType;
}

  // proto:  int QPaintDevice::heightMM();
impl<'a> /*trait*/ QPaintDevice_heightMM<i32> for () {
  fn heightMM(self , rsthis: & QPaintDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPaintDevice8heightMMEv()};
    let mut ret = unsafe {C_ZNK12QPaintDevice8heightMMEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QPaintDevice::colorCount();
impl /*struct*/ QPaintDevice {
  pub fn colorCount<RetType, T: QPaintDevice_colorCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.colorCount(self);
    // return 1;
  }
}

pub trait QPaintDevice_colorCount<RetType> {
  fn colorCount(self , rsthis: & QPaintDevice) -> RetType;
}

  // proto:  int QPaintDevice::colorCount();
impl<'a> /*trait*/ QPaintDevice_colorCount<i32> for () {
  fn colorCount(self , rsthis: & QPaintDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPaintDevice10colorCountEv()};
    let mut ret = unsafe {C_ZNK12QPaintDevice10colorCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QPaintDevice::physicalDpiX();
impl /*struct*/ QPaintDevice {
  pub fn physicalDpiX<RetType, T: QPaintDevice_physicalDpiX<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.physicalDpiX(self);
    // return 1;
  }
}

pub trait QPaintDevice_physicalDpiX<RetType> {
  fn physicalDpiX(self , rsthis: & QPaintDevice) -> RetType;
}

  // proto:  int QPaintDevice::physicalDpiX();
impl<'a> /*trait*/ QPaintDevice_physicalDpiX<i32> for () {
  fn physicalDpiX(self , rsthis: & QPaintDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPaintDevice12physicalDpiXEv()};
    let mut ret = unsafe {C_ZNK12QPaintDevice12physicalDpiXEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QPaintDevice::widthMM();
impl /*struct*/ QPaintDevice {
  pub fn widthMM<RetType, T: QPaintDevice_widthMM<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.widthMM(self);
    // return 1;
  }
}

pub trait QPaintDevice_widthMM<RetType> {
  fn widthMM(self , rsthis: & QPaintDevice) -> RetType;
}

  // proto:  int QPaintDevice::widthMM();
impl<'a> /*trait*/ QPaintDevice_widthMM<i32> for () {
  fn widthMM(self , rsthis: & QPaintDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPaintDevice7widthMMEv()};
    let mut ret = unsafe {C_ZNK12QPaintDevice7widthMMEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
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
    let mut ret = unsafe {C_ZNK12QPaintDevice7devTypeEv(rsthis.qclsinst)};
    return ret as i32;
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
    let mut ret = unsafe {C_ZNK12QPaintDevice14paintingActiveEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  int QPaintDevice::width();
impl /*struct*/ QPaintDevice {
  pub fn width<RetType, T: QPaintDevice_width<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.width(self);
    // return 1;
  }
}

pub trait QPaintDevice_width<RetType> {
  fn width(self , rsthis: & QPaintDevice) -> RetType;
}

  // proto:  int QPaintDevice::width();
impl<'a> /*trait*/ QPaintDevice_width<i32> for () {
  fn width(self , rsthis: & QPaintDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPaintDevice5widthEv()};
    let mut ret = unsafe {C_ZNK12QPaintDevice5widthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QPaintDevice::devicePixelRatio();
impl /*struct*/ QPaintDevice {
  pub fn devicePixelRatio<RetType, T: QPaintDevice_devicePixelRatio<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.devicePixelRatio(self);
    // return 1;
  }
}

pub trait QPaintDevice_devicePixelRatio<RetType> {
  fn devicePixelRatio(self , rsthis: & QPaintDevice) -> RetType;
}

  // proto:  int QPaintDevice::devicePixelRatio();
impl<'a> /*trait*/ QPaintDevice_devicePixelRatio<i32> for () {
  fn devicePixelRatio(self , rsthis: & QPaintDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPaintDevice16devicePixelRatioEv()};
    let mut ret = unsafe {C_ZNK12QPaintDevice16devicePixelRatioEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QPaintDevice::height();
impl /*struct*/ QPaintDevice {
  pub fn height<RetType, T: QPaintDevice_height<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.height(self);
    // return 1;
  }
}

pub trait QPaintDevice_height<RetType> {
  fn height(self , rsthis: & QPaintDevice) -> RetType;
}

  // proto:  int QPaintDevice::height();
impl<'a> /*trait*/ QPaintDevice_height<i32> for () {
  fn height(self , rsthis: & QPaintDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPaintDevice6heightEv()};
    let mut ret = unsafe {C_ZNK12QPaintDevice6heightEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QPaintDevice::depth();
impl /*struct*/ QPaintDevice {
  pub fn depth<RetType, T: QPaintDevice_depth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.depth(self);
    // return 1;
  }
}

pub trait QPaintDevice_depth<RetType> {
  fn depth(self , rsthis: & QPaintDevice) -> RetType;
}

  // proto:  int QPaintDevice::depth();
impl<'a> /*trait*/ QPaintDevice_depth<i32> for () {
  fn depth(self , rsthis: & QPaintDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPaintDevice5depthEv()};
    let mut ret = unsafe {C_ZNK12QPaintDevice5depthEv(rsthis.qclsinst)};
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
    let mut ret = unsafe {C_ZNK12QPaintDevice11paintEngineEv(rsthis.qclsinst)};
    let mut ret1 = QPaintEngine::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QPaintDevice::logicalDpiY();
impl /*struct*/ QPaintDevice {
  pub fn logicalDpiY<RetType, T: QPaintDevice_logicalDpiY<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.logicalDpiY(self);
    // return 1;
  }
}

pub trait QPaintDevice_logicalDpiY<RetType> {
  fn logicalDpiY(self , rsthis: & QPaintDevice) -> RetType;
}

  // proto:  int QPaintDevice::logicalDpiY();
impl<'a> /*trait*/ QPaintDevice_logicalDpiY<i32> for () {
  fn logicalDpiY(self , rsthis: & QPaintDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPaintDevice11logicalDpiYEv()};
    let mut ret = unsafe {C_ZNK12QPaintDevice11logicalDpiYEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QPaintDevice::~QPaintDevice();
impl /*struct*/ QPaintDevice {
  pub fn free<RetType, T: QPaintDevice_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QPaintDevice_free<RetType> {
  fn free(self , rsthis: & QPaintDevice) -> RetType;
}

  // proto:  void QPaintDevice::~QPaintDevice();
impl<'a> /*trait*/ QPaintDevice_free<()> for () {
  fn free(self , rsthis: & QPaintDevice) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPaintDeviceD2Ev()};
     unsafe {C_ZN12QPaintDeviceD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QPaintDevice::logicalDpiX();
impl /*struct*/ QPaintDevice {
  pub fn logicalDpiX<RetType, T: QPaintDevice_logicalDpiX<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.logicalDpiX(self);
    // return 1;
  }
}

pub trait QPaintDevice_logicalDpiX<RetType> {
  fn logicalDpiX(self , rsthis: & QPaintDevice) -> RetType;
}

  // proto:  int QPaintDevice::logicalDpiX();
impl<'a> /*trait*/ QPaintDevice_logicalDpiX<i32> for () {
  fn logicalDpiX(self , rsthis: & QPaintDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPaintDevice11logicalDpiXEv()};
    let mut ret = unsafe {C_ZNK12QPaintDevice11logicalDpiXEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// <= body block end

