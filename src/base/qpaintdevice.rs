// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qpaintengine::QPaintEngine;

// ext block begin
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
  fn _ZNK12QPaintDevice14paintingActiveEv(qthis: *mut c_void) -> int8_t;
  // proto:  int QPaintDevice::width();
  fn _ZNK12QPaintDevice5widthEv(qthis: *mut c_void) -> c_int;
  // proto:  void QPaintDevice::NewQPaintDevice(const QPaintDevice & );
  fn _ZN12QPaintDeviceC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QPaintDevice::NewQPaintDevice();
  fn _ZN12QPaintDeviceC1Ev(qthis: *mut c_void) ;
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
  // proto:  void QPaintDevice::FreeQPaintDevice();
  fn _ZN12QPaintDeviceD0Ev(qthis: *mut c_void) ;
  // proto:  int QPaintDevice::logicalDpiX();
  fn _ZNK12QPaintDevice11logicalDpiXEv(qthis: *mut c_void) -> c_int;
}

// body block begin
// class sizeof(QPaintDevice)=24
pub struct QPaintDevice {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QPaintDevice {
  pub fn physicalDpiY<T: QPaintDevice_physicalDpiY>(&mut self, value: T) -> i32 {
    return value.physicalDpiY(self);
    // return 1;
  }
}

pub trait QPaintDevice_physicalDpiY {
  fn physicalDpiY(self, rsthis: &mut QPaintDevice) -> i32;
}

// proto:  int QPaintDevice::physicalDpiY();
impl<'a> /*trait*/ QPaintDevice_physicalDpiY for () {
  fn physicalDpiY(self, rsthis: &mut QPaintDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPaintDevice12physicalDpiYEv()};
    let mut ret = unsafe {_ZNK12QPaintDevice12physicalDpiYEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QPaintDevice {
  pub fn heightMM<T: QPaintDevice_heightMM>(&mut self, value: T) -> i32 {
    return value.heightMM(self);
    // return 1;
  }
}

pub trait QPaintDevice_heightMM {
  fn heightMM(self, rsthis: &mut QPaintDevice) -> i32;
}

// proto:  int QPaintDevice::heightMM();
impl<'a> /*trait*/ QPaintDevice_heightMM for () {
  fn heightMM(self, rsthis: &mut QPaintDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPaintDevice8heightMMEv()};
    let mut ret = unsafe {_ZNK12QPaintDevice8heightMMEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QPaintDevice {
  pub fn colorCount<T: QPaintDevice_colorCount>(&mut self, value: T) -> i32 {
    return value.colorCount(self);
    // return 1;
  }
}

pub trait QPaintDevice_colorCount {
  fn colorCount(self, rsthis: &mut QPaintDevice) -> i32;
}

// proto:  int QPaintDevice::colorCount();
impl<'a> /*trait*/ QPaintDevice_colorCount for () {
  fn colorCount(self, rsthis: &mut QPaintDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPaintDevice10colorCountEv()};
    let mut ret = unsafe {_ZNK12QPaintDevice10colorCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QPaintDevice {
  pub fn physicalDpiX<T: QPaintDevice_physicalDpiX>(&mut self, value: T) -> i32 {
    return value.physicalDpiX(self);
    // return 1;
  }
}

pub trait QPaintDevice_physicalDpiX {
  fn physicalDpiX(self, rsthis: &mut QPaintDevice) -> i32;
}

// proto:  int QPaintDevice::physicalDpiX();
impl<'a> /*trait*/ QPaintDevice_physicalDpiX for () {
  fn physicalDpiX(self, rsthis: &mut QPaintDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPaintDevice12physicalDpiXEv()};
    let mut ret = unsafe {_ZNK12QPaintDevice12physicalDpiXEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QPaintDevice {
  pub fn widthMM<T: QPaintDevice_widthMM>(&mut self, value: T) -> i32 {
    return value.widthMM(self);
    // return 1;
  }
}

pub trait QPaintDevice_widthMM {
  fn widthMM(self, rsthis: &mut QPaintDevice) -> i32;
}

// proto:  int QPaintDevice::widthMM();
impl<'a> /*trait*/ QPaintDevice_widthMM for () {
  fn widthMM(self, rsthis: &mut QPaintDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPaintDevice7widthMMEv()};
    let mut ret = unsafe {_ZNK12QPaintDevice7widthMMEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QPaintDevice {
  pub fn devType<T: QPaintDevice_devType>(&mut self, value: T) -> i32 {
    return value.devType(self);
    // return 1;
  }
}

pub trait QPaintDevice_devType {
  fn devType(self, rsthis: &mut QPaintDevice) -> i32;
}

// proto:  int QPaintDevice::devType();
impl<'a> /*trait*/ QPaintDevice_devType for () {
  fn devType(self, rsthis: &mut QPaintDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPaintDevice7devTypeEv()};
    let mut ret = unsafe {_ZNK12QPaintDevice7devTypeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QPaintDevice {
  pub fn paintingActive<T: QPaintDevice_paintingActive>(&mut self, value: T) -> i8 {
    return value.paintingActive(self);
    // return 1;
  }
}

pub trait QPaintDevice_paintingActive {
  fn paintingActive(self, rsthis: &mut QPaintDevice) -> i8;
}

// proto:  bool QPaintDevice::paintingActive();
impl<'a> /*trait*/ QPaintDevice_paintingActive for () {
  fn paintingActive(self, rsthis: &mut QPaintDevice) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPaintDevice14paintingActiveEv()};
    let mut ret = unsafe {_ZNK12QPaintDevice14paintingActiveEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QPaintDevice {
  pub fn width<T: QPaintDevice_width>(&mut self, value: T) -> i32 {
    return value.width(self);
    // return 1;
  }
}

pub trait QPaintDevice_width {
  fn width(self, rsthis: &mut QPaintDevice) -> i32;
}

// proto:  int QPaintDevice::width();
impl<'a> /*trait*/ QPaintDevice_width for () {
  fn width(self, rsthis: &mut QPaintDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPaintDevice5widthEv()};
    let mut ret = unsafe {_ZNK12QPaintDevice5widthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

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

// proto: void QPaintDevice::NewQPaintDevice(const QPaintDevice & );
impl<'a> /*trait*/ QPaintDevice_NewQPaintDevice for (&'a  QPaintDevice) {
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

// proto: void QPaintDevice::NewQPaintDevice();
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

impl /*struct*/ QPaintDevice {
  pub fn devicePixelRatio<T: QPaintDevice_devicePixelRatio>(&mut self, value: T) -> i32 {
    return value.devicePixelRatio(self);
    // return 1;
  }
}

pub trait QPaintDevice_devicePixelRatio {
  fn devicePixelRatio(self, rsthis: &mut QPaintDevice) -> i32;
}

// proto:  int QPaintDevice::devicePixelRatio();
impl<'a> /*trait*/ QPaintDevice_devicePixelRatio for () {
  fn devicePixelRatio(self, rsthis: &mut QPaintDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPaintDevice16devicePixelRatioEv()};
    let mut ret = unsafe {_ZNK12QPaintDevice16devicePixelRatioEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QPaintDevice {
  pub fn height<T: QPaintDevice_height>(&mut self, value: T) -> i32 {
    return value.height(self);
    // return 1;
  }
}

pub trait QPaintDevice_height {
  fn height(self, rsthis: &mut QPaintDevice) -> i32;
}

// proto:  int QPaintDevice::height();
impl<'a> /*trait*/ QPaintDevice_height for () {
  fn height(self, rsthis: &mut QPaintDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPaintDevice6heightEv()};
    let mut ret = unsafe {_ZNK12QPaintDevice6heightEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QPaintDevice {
  pub fn depth<T: QPaintDevice_depth>(&mut self, value: T) -> i32 {
    return value.depth(self);
    // return 1;
  }
}

pub trait QPaintDevice_depth {
  fn depth(self, rsthis: &mut QPaintDevice) -> i32;
}

// proto:  int QPaintDevice::depth();
impl<'a> /*trait*/ QPaintDevice_depth for () {
  fn depth(self, rsthis: &mut QPaintDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPaintDevice5depthEv()};
    let mut ret = unsafe {_ZNK12QPaintDevice5depthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QPaintDevice {
  pub fn paintEngine<T: QPaintDevice_paintEngine>(&mut self, value: T) -> QPaintEngine {
    return value.paintEngine(self);
    // return 1;
  }
}

pub trait QPaintDevice_paintEngine {
  fn paintEngine(self, rsthis: &mut QPaintDevice) -> QPaintEngine;
}

// proto:  QPaintEngine * QPaintDevice::paintEngine();
impl<'a> /*trait*/ QPaintDevice_paintEngine for () {
  fn paintEngine(self, rsthis: &mut QPaintDevice) -> QPaintEngine {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPaintDevice11paintEngineEv()};
    let mut ret = unsafe {_ZNK12QPaintDevice11paintEngineEv(rsthis.qclsinst)};
    let mut ret1 = QPaintEngine{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPaintDevice {
  pub fn logicalDpiY<T: QPaintDevice_logicalDpiY>(&mut self, value: T) -> i32 {
    return value.logicalDpiY(self);
    // return 1;
  }
}

pub trait QPaintDevice_logicalDpiY {
  fn logicalDpiY(self, rsthis: &mut QPaintDevice) -> i32;
}

// proto:  int QPaintDevice::logicalDpiY();
impl<'a> /*trait*/ QPaintDevice_logicalDpiY for () {
  fn logicalDpiY(self, rsthis: &mut QPaintDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPaintDevice11logicalDpiYEv()};
    let mut ret = unsafe {_ZNK12QPaintDevice11logicalDpiYEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QPaintDevice {
  pub fn FreeQPaintDevice<T: QPaintDevice_FreeQPaintDevice>(&mut self, value: T)  {
     value.FreeQPaintDevice(self);
    // return 1;
  }
}

pub trait QPaintDevice_FreeQPaintDevice {
  fn FreeQPaintDevice(self, rsthis: &mut QPaintDevice) ;
}

// proto:  void QPaintDevice::FreeQPaintDevice();
impl<'a> /*trait*/ QPaintDevice_FreeQPaintDevice for () {
  fn FreeQPaintDevice(self, rsthis: &mut QPaintDevice)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPaintDeviceD0Ev()};
     unsafe {_ZN12QPaintDeviceD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QPaintDevice {
  pub fn logicalDpiX<T: QPaintDevice_logicalDpiX>(&mut self, value: T) -> i32 {
    return value.logicalDpiX(self);
    // return 1;
  }
}

pub trait QPaintDevice_logicalDpiX {
  fn logicalDpiX(self, rsthis: &mut QPaintDevice) -> i32;
}

// proto:  int QPaintDevice::logicalDpiX();
impl<'a> /*trait*/ QPaintDevice_logicalDpiX for () {
  fn logicalDpiX(self, rsthis: &mut QPaintDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPaintDevice11logicalDpiXEv()};
    let mut ret = unsafe {_ZNK12QPaintDevice11logicalDpiXEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

