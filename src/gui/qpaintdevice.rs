// auto generated, do not modify.
// created: Mon Dec 21 22:54:38 2015
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
use super::qpaintengine::QPaintEngine; // 773
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  int QPaintDevice::physicalDpiY();
  fn _ZNK12QPaintDevice12physicalDpiYEv(qthis: *mut c_void) -> c_int;
  // proto:  int QPaintDevice::heightMM();
  fn _ZNK12QPaintDevice8heightMMEv(qthis: *mut c_void) -> c_int;
  // proto:  int QPaintDevice::colorCount();
  fn _ZNK12QPaintDevice10colorCountEv(qthis: *mut c_void) -> c_int;
  // proto:  int QPaintDevice::physicalDpiX();
  fn _ZNK12QPaintDevice12physicalDpiXEv(qthis: *mut c_void) -> c_int;
  // proto:  int QPaintDevice::widthMM();
  fn _ZNK12QPaintDevice7widthMMEv(qthis: *mut c_void) -> c_int;
  // proto:  int QPaintDevice::devType();
  fn _ZNK12QPaintDevice7devTypeEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QPaintDevice::paintingActive();
  fn _ZNK12QPaintDevice14paintingActiveEv(qthis: *mut c_void) -> c_char;
  // proto:  int QPaintDevice::width();
  fn _ZNK12QPaintDevice5widthEv(qthis: *mut c_void) -> c_int;
  // proto:  void QPaintDevice::QPaintDevice(const QPaintDevice & );
  fn _ZN12QPaintDeviceC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QPaintDevice::QPaintDevice();
  fn _ZN12QPaintDeviceC1Ev(qthis: *mut c_void);
  // proto:  int QPaintDevice::devicePixelRatio();
  fn _ZNK12QPaintDevice16devicePixelRatioEv(qthis: *mut c_void) -> c_int;
  // proto:  int QPaintDevice::height();
  fn _ZNK12QPaintDevice6heightEv(qthis: *mut c_void) -> c_int;
  // proto:  int QPaintDevice::depth();
  fn _ZNK12QPaintDevice5depthEv(qthis: *mut c_void) -> c_int;
  // proto:  QPaintEngine * QPaintDevice::paintEngine();
  fn _ZNK12QPaintDevice11paintEngineEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QPaintDevice::logicalDpiY();
  fn _ZNK12QPaintDevice11logicalDpiYEv(qthis: *mut c_void) -> c_int;
  // proto:  void QPaintDevice::~QPaintDevice();
  fn _ZN12QPaintDeviceD0Ev(qthis: *mut c_void);
  // proto:  int QPaintDevice::logicalDpiX();
  fn _ZNK12QPaintDevice11logicalDpiXEv(qthis: *mut c_void) -> c_int;
} // <= ext block end

// body block begin =>
// class sizeof(QPaintDevice)=24
pub struct QPaintDevice {
  pub qclsinst: *mut c_void,
}

  // proto:  int QPaintDevice::physicalDpiY();
impl /*struct*/ QPaintDevice {
  pub fn physicalDpiY<RetType, T: QPaintDevice_physicalDpiY<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.physicalDpiY(self);
    // return 1;
  }
}

pub trait QPaintDevice_physicalDpiY<RetType> {
  fn physicalDpiY(self , rsthis: &mut QPaintDevice) -> RetType;
}

  // proto:  int QPaintDevice::physicalDpiY();
impl<'a> /*trait*/ QPaintDevice_physicalDpiY<i32> for () {
  fn physicalDpiY(self , rsthis: &mut QPaintDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPaintDevice12physicalDpiYEv()};
    let mut ret = unsafe {_ZNK12QPaintDevice12physicalDpiYEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QPaintDevice::heightMM();
impl /*struct*/ QPaintDevice {
  pub fn heightMM<RetType, T: QPaintDevice_heightMM<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.heightMM(self);
    // return 1;
  }
}

pub trait QPaintDevice_heightMM<RetType> {
  fn heightMM(self , rsthis: &mut QPaintDevice) -> RetType;
}

  // proto:  int QPaintDevice::heightMM();
impl<'a> /*trait*/ QPaintDevice_heightMM<i32> for () {
  fn heightMM(self , rsthis: &mut QPaintDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPaintDevice8heightMMEv()};
    let mut ret = unsafe {_ZNK12QPaintDevice8heightMMEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QPaintDevice::colorCount();
impl /*struct*/ QPaintDevice {
  pub fn colorCount<RetType, T: QPaintDevice_colorCount<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.colorCount(self);
    // return 1;
  }
}

pub trait QPaintDevice_colorCount<RetType> {
  fn colorCount(self , rsthis: &mut QPaintDevice) -> RetType;
}

  // proto:  int QPaintDevice::colorCount();
impl<'a> /*trait*/ QPaintDevice_colorCount<i32> for () {
  fn colorCount(self , rsthis: &mut QPaintDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPaintDevice10colorCountEv()};
    let mut ret = unsafe {_ZNK12QPaintDevice10colorCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QPaintDevice::physicalDpiX();
impl /*struct*/ QPaintDevice {
  pub fn physicalDpiX<RetType, T: QPaintDevice_physicalDpiX<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.physicalDpiX(self);
    // return 1;
  }
}

pub trait QPaintDevice_physicalDpiX<RetType> {
  fn physicalDpiX(self , rsthis: &mut QPaintDevice) -> RetType;
}

  // proto:  int QPaintDevice::physicalDpiX();
impl<'a> /*trait*/ QPaintDevice_physicalDpiX<i32> for () {
  fn physicalDpiX(self , rsthis: &mut QPaintDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPaintDevice12physicalDpiXEv()};
    let mut ret = unsafe {_ZNK12QPaintDevice12physicalDpiXEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QPaintDevice::widthMM();
impl /*struct*/ QPaintDevice {
  pub fn widthMM<RetType, T: QPaintDevice_widthMM<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.widthMM(self);
    // return 1;
  }
}

pub trait QPaintDevice_widthMM<RetType> {
  fn widthMM(self , rsthis: &mut QPaintDevice) -> RetType;
}

  // proto:  int QPaintDevice::widthMM();
impl<'a> /*trait*/ QPaintDevice_widthMM<i32> for () {
  fn widthMM(self , rsthis: &mut QPaintDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPaintDevice7widthMMEv()};
    let mut ret = unsafe {_ZNK12QPaintDevice7widthMMEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QPaintDevice::devType();
impl /*struct*/ QPaintDevice {
  pub fn devType<RetType, T: QPaintDevice_devType<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.devType(self);
    // return 1;
  }
}

pub trait QPaintDevice_devType<RetType> {
  fn devType(self , rsthis: &mut QPaintDevice) -> RetType;
}

  // proto:  int QPaintDevice::devType();
impl<'a> /*trait*/ QPaintDevice_devType<i32> for () {
  fn devType(self , rsthis: &mut QPaintDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPaintDevice7devTypeEv()};
    let mut ret = unsafe {_ZNK12QPaintDevice7devTypeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  bool QPaintDevice::paintingActive();
impl /*struct*/ QPaintDevice {
  pub fn paintingActive<RetType, T: QPaintDevice_paintingActive<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.paintingActive(self);
    // return 1;
  }
}

pub trait QPaintDevice_paintingActive<RetType> {
  fn paintingActive(self , rsthis: &mut QPaintDevice) -> RetType;
}

  // proto:  bool QPaintDevice::paintingActive();
impl<'a> /*trait*/ QPaintDevice_paintingActive<i8> for () {
  fn paintingActive(self , rsthis: &mut QPaintDevice) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPaintDevice14paintingActiveEv()};
    let mut ret = unsafe {_ZNK12QPaintDevice14paintingActiveEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  int QPaintDevice::width();
impl /*struct*/ QPaintDevice {
  pub fn width<RetType, T: QPaintDevice_width<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.width(self);
    // return 1;
  }
}

pub trait QPaintDevice_width<RetType> {
  fn width(self , rsthis: &mut QPaintDevice) -> RetType;
}

  // proto:  int QPaintDevice::width();
impl<'a> /*trait*/ QPaintDevice_width<i32> for () {
  fn width(self , rsthis: &mut QPaintDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPaintDevice5widthEv()};
    let mut ret = unsafe {_ZNK12QPaintDevice5widthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QPaintDevice::QPaintDevice(const QPaintDevice & );
impl /*struct*/ QPaintDevice {
  pub fn NewQPaintDevice<T: QPaintDevice_NewQPaintDevice>(value: T) -> QPaintDevice {
    let rsthis = value.NewQPaintDevice();
    return rsthis;
    // return 1;
  }
}

pub trait QPaintDevice_NewQPaintDevice {
  fn NewQPaintDevice(self) -> QPaintDevice;
}

  // proto:  void QPaintDevice::QPaintDevice(const QPaintDevice & );
impl<'a> /*trait*/ QPaintDevice_NewQPaintDevice for (QPaintDevice) {
  fn NewQPaintDevice(self) -> QPaintDevice {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPaintDeviceC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN12QPaintDeviceC1ERKS_(qthis, arg0)};
    let rsthis = QPaintDevice{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QPaintDevice::QPaintDevice();
impl<'a> /*trait*/ QPaintDevice_NewQPaintDevice for () {
  fn NewQPaintDevice(self) -> QPaintDevice {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPaintDeviceC1Ev()};
    unsafe {_ZN12QPaintDeviceC1Ev(qthis)};
    let rsthis = QPaintDevice{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  int QPaintDevice::devicePixelRatio();
impl /*struct*/ QPaintDevice {
  pub fn devicePixelRatio<RetType, T: QPaintDevice_devicePixelRatio<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.devicePixelRatio(self);
    // return 1;
  }
}

pub trait QPaintDevice_devicePixelRatio<RetType> {
  fn devicePixelRatio(self , rsthis: &mut QPaintDevice) -> RetType;
}

  // proto:  int QPaintDevice::devicePixelRatio();
impl<'a> /*trait*/ QPaintDevice_devicePixelRatio<i32> for () {
  fn devicePixelRatio(self , rsthis: &mut QPaintDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPaintDevice16devicePixelRatioEv()};
    let mut ret = unsafe {_ZNK12QPaintDevice16devicePixelRatioEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QPaintDevice::height();
impl /*struct*/ QPaintDevice {
  pub fn height<RetType, T: QPaintDevice_height<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.height(self);
    // return 1;
  }
}

pub trait QPaintDevice_height<RetType> {
  fn height(self , rsthis: &mut QPaintDevice) -> RetType;
}

  // proto:  int QPaintDevice::height();
impl<'a> /*trait*/ QPaintDevice_height<i32> for () {
  fn height(self , rsthis: &mut QPaintDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPaintDevice6heightEv()};
    let mut ret = unsafe {_ZNK12QPaintDevice6heightEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QPaintDevice::depth();
impl /*struct*/ QPaintDevice {
  pub fn depth<RetType, T: QPaintDevice_depth<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.depth(self);
    // return 1;
  }
}

pub trait QPaintDevice_depth<RetType> {
  fn depth(self , rsthis: &mut QPaintDevice) -> RetType;
}

  // proto:  int QPaintDevice::depth();
impl<'a> /*trait*/ QPaintDevice_depth<i32> for () {
  fn depth(self , rsthis: &mut QPaintDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPaintDevice5depthEv()};
    let mut ret = unsafe {_ZNK12QPaintDevice5depthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QPaintEngine * QPaintDevice::paintEngine();
impl /*struct*/ QPaintDevice {
  pub fn paintEngine<RetType, T: QPaintDevice_paintEngine<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.paintEngine(self);
    // return 1;
  }
}

pub trait QPaintDevice_paintEngine<RetType> {
  fn paintEngine(self , rsthis: &mut QPaintDevice) -> RetType;
}

  // proto:  QPaintEngine * QPaintDevice::paintEngine();
impl<'a> /*trait*/ QPaintDevice_paintEngine<QPaintEngine> for () {
  fn paintEngine(self , rsthis: &mut QPaintDevice) -> QPaintEngine {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPaintDevice11paintEngineEv()};
    let mut ret = unsafe {_ZNK12QPaintDevice11paintEngineEv(rsthis.qclsinst)};
    let mut ret1 = QPaintEngine{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  int QPaintDevice::logicalDpiY();
impl /*struct*/ QPaintDevice {
  pub fn logicalDpiY<RetType, T: QPaintDevice_logicalDpiY<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.logicalDpiY(self);
    // return 1;
  }
}

pub trait QPaintDevice_logicalDpiY<RetType> {
  fn logicalDpiY(self , rsthis: &mut QPaintDevice) -> RetType;
}

  // proto:  int QPaintDevice::logicalDpiY();
impl<'a> /*trait*/ QPaintDevice_logicalDpiY<i32> for () {
  fn logicalDpiY(self , rsthis: &mut QPaintDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPaintDevice11logicalDpiYEv()};
    let mut ret = unsafe {_ZNK12QPaintDevice11logicalDpiYEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QPaintDevice::~QPaintDevice();
impl /*struct*/ QPaintDevice {
  pub fn FreeQPaintDevice<RetType, T: QPaintDevice_FreeQPaintDevice<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQPaintDevice(self);
    // return 1;
  }
}

pub trait QPaintDevice_FreeQPaintDevice<RetType> {
  fn FreeQPaintDevice(self , rsthis: &mut QPaintDevice) -> RetType;
}

  // proto:  void QPaintDevice::~QPaintDevice();
impl<'a> /*trait*/ QPaintDevice_FreeQPaintDevice<()> for () {
  fn FreeQPaintDevice(self , rsthis: &mut QPaintDevice) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPaintDeviceD0Ev()};
     unsafe {_ZN12QPaintDeviceD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QPaintDevice::logicalDpiX();
impl /*struct*/ QPaintDevice {
  pub fn logicalDpiX<RetType, T: QPaintDevice_logicalDpiX<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.logicalDpiX(self);
    // return 1;
  }
}

pub trait QPaintDevice_logicalDpiX<RetType> {
  fn logicalDpiX(self , rsthis: &mut QPaintDevice) -> RetType;
}

  // proto:  int QPaintDevice::logicalDpiX();
impl<'a> /*trait*/ QPaintDevice_logicalDpiX<i32> for () {
  fn logicalDpiX(self , rsthis: &mut QPaintDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPaintDevice11logicalDpiXEv()};
    let mut ret = unsafe {_ZNK12QPaintDevice11logicalDpiXEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// <= body block end

