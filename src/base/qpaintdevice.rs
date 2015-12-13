// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: int QPaintDevice::physicalDpiY();
  fn _ZNK12QPaintDevice12physicalDpiYEv() -> i32;
  // proto: int QPaintDevice::heightMM();
  fn _ZNK12QPaintDevice8heightMMEv() -> i32;
  // proto: int QPaintDevice::colorCount();
  fn _ZNK12QPaintDevice10colorCountEv() -> i32;
  // proto: int QPaintDevice::physicalDpiX();
  fn _ZNK12QPaintDevice12physicalDpiXEv() -> i32;
  // proto: int QPaintDevice::widthMM();
  fn _ZNK12QPaintDevice7widthMMEv() -> i32;
  // proto: int QPaintDevice::devType();
  fn _ZNK12QPaintDevice7devTypeEv() -> i32;
  // proto: bool QPaintDevice::paintingActive();
  fn _ZNK12QPaintDevice14paintingActiveEv() -> i32;
  // proto: int QPaintDevice::width();
  fn _ZNK12QPaintDevice5widthEv() -> i32;
  // proto: void QPaintDevice::NewQPaintDevice(const QPaintDevice & );
  fn _ZN12QPaintDeviceC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: void QPaintDevice::NewQPaintDevice();
  fn _ZN12QPaintDeviceC1Ev(qthis: *mut c_void) -> i32;
  // proto: int QPaintDevice::devicePixelRatio();
  fn _ZNK12QPaintDevice16devicePixelRatioEv() -> i32;
  // proto: int QPaintDevice::height();
  fn _ZNK12QPaintDevice6heightEv() -> i32;
  // proto: int QPaintDevice::depth();
  fn _ZNK12QPaintDevice5depthEv() -> i32;
  // proto: QPaintEngine * QPaintDevice::paintEngine();
  fn _ZNK12QPaintDevice11paintEngineEv() -> i32;
  // proto: int QPaintDevice::logicalDpiY();
  fn _ZNK12QPaintDevice11logicalDpiYEv() -> i32;
  // proto: void QPaintDevice::FreeQPaintDevice();
  fn _ZN12QPaintDeviceD0Ev() -> i32;
  // proto: int QPaintDevice::logicalDpiX();
  fn _ZNK12QPaintDevice11logicalDpiXEv() -> i32;
}

// body block begin
// class sizeof(QPaintDevice)=24
pub struct QPaintDevice {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QPaintDevice {
  pub fn physicalDpiY<T: QPaintDevice_physicalDpiY>(&mut self, value: T) -> i32 {
    value.physicalDpiY(self);
    return 1;
  }
}

pub trait QPaintDevice_physicalDpiY {
  fn physicalDpiY(self, this: &mut QPaintDevice) -> i32;
}

// proto: int QPaintDevice::physicalDpiY();
impl<'a> /*trait*/ QPaintDevice_physicalDpiY for () {
  fn physicalDpiY(self, this: &mut QPaintDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPaintDevice12physicalDpiYEv()};
    unsafe {_ZNK12QPaintDevice12physicalDpiYEv()};
    return 1;
  }
}

impl /*struct*/ QPaintDevice {
  pub fn heightMM<T: QPaintDevice_heightMM>(&mut self, value: T) -> i32 {
    value.heightMM(self);
    return 1;
  }
}

pub trait QPaintDevice_heightMM {
  fn heightMM(self, this: &mut QPaintDevice) -> i32;
}

// proto: int QPaintDevice::heightMM();
impl<'a> /*trait*/ QPaintDevice_heightMM for () {
  fn heightMM(self, this: &mut QPaintDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPaintDevice8heightMMEv()};
    unsafe {_ZNK12QPaintDevice8heightMMEv()};
    return 1;
  }
}

impl /*struct*/ QPaintDevice {
  pub fn colorCount<T: QPaintDevice_colorCount>(&mut self, value: T) -> i32 {
    value.colorCount(self);
    return 1;
  }
}

pub trait QPaintDevice_colorCount {
  fn colorCount(self, this: &mut QPaintDevice) -> i32;
}

// proto: int QPaintDevice::colorCount();
impl<'a> /*trait*/ QPaintDevice_colorCount for () {
  fn colorCount(self, this: &mut QPaintDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPaintDevice10colorCountEv()};
    unsafe {_ZNK12QPaintDevice10colorCountEv()};
    return 1;
  }
}

impl /*struct*/ QPaintDevice {
  pub fn physicalDpiX<T: QPaintDevice_physicalDpiX>(&mut self, value: T) -> i32 {
    value.physicalDpiX(self);
    return 1;
  }
}

pub trait QPaintDevice_physicalDpiX {
  fn physicalDpiX(self, this: &mut QPaintDevice) -> i32;
}

// proto: int QPaintDevice::physicalDpiX();
impl<'a> /*trait*/ QPaintDevice_physicalDpiX for () {
  fn physicalDpiX(self, this: &mut QPaintDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPaintDevice12physicalDpiXEv()};
    unsafe {_ZNK12QPaintDevice12physicalDpiXEv()};
    return 1;
  }
}

impl /*struct*/ QPaintDevice {
  pub fn widthMM<T: QPaintDevice_widthMM>(&mut self, value: T) -> i32 {
    value.widthMM(self);
    return 1;
  }
}

pub trait QPaintDevice_widthMM {
  fn widthMM(self, this: &mut QPaintDevice) -> i32;
}

// proto: int QPaintDevice::widthMM();
impl<'a> /*trait*/ QPaintDevice_widthMM for () {
  fn widthMM(self, this: &mut QPaintDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPaintDevice7widthMMEv()};
    unsafe {_ZNK12QPaintDevice7widthMMEv()};
    return 1;
  }
}

impl /*struct*/ QPaintDevice {
  pub fn devType<T: QPaintDevice_devType>(&mut self, value: T) -> i32 {
    value.devType(self);
    return 1;
  }
}

pub trait QPaintDevice_devType {
  fn devType(self, this: &mut QPaintDevice) -> i32;
}

// proto: int QPaintDevice::devType();
impl<'a> /*trait*/ QPaintDevice_devType for () {
  fn devType(self, this: &mut QPaintDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPaintDevice7devTypeEv()};
    unsafe {_ZNK12QPaintDevice7devTypeEv()};
    return 1;
  }
}

impl /*struct*/ QPaintDevice {
  pub fn paintingActive<T: QPaintDevice_paintingActive>(&mut self, value: T) -> i32 {
    value.paintingActive(self);
    return 1;
  }
}

pub trait QPaintDevice_paintingActive {
  fn paintingActive(self, this: &mut QPaintDevice) -> i32;
}

// proto: bool QPaintDevice::paintingActive();
impl<'a> /*trait*/ QPaintDevice_paintingActive for () {
  fn paintingActive(self, this: &mut QPaintDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPaintDevice14paintingActiveEv()};
    unsafe {_ZNK12QPaintDevice14paintingActiveEv()};
    return 1;
  }
}

impl /*struct*/ QPaintDevice {
  pub fn width<T: QPaintDevice_width>(&mut self, value: T) -> i32 {
    value.width(self);
    return 1;
  }
}

pub trait QPaintDevice_width {
  fn width(self, this: &mut QPaintDevice) -> i32;
}

// proto: int QPaintDevice::width();
impl<'a> /*trait*/ QPaintDevice_width for () {
  fn width(self, this: &mut QPaintDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPaintDevice5widthEv()};
    unsafe {_ZNK12QPaintDevice5widthEv()};
    return 1;
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
    let arg0 = self.qclsinst  as *const c_void;
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
    value.devicePixelRatio(self);
    return 1;
  }
}

pub trait QPaintDevice_devicePixelRatio {
  fn devicePixelRatio(self, this: &mut QPaintDevice) -> i32;
}

// proto: int QPaintDevice::devicePixelRatio();
impl<'a> /*trait*/ QPaintDevice_devicePixelRatio for () {
  fn devicePixelRatio(self, this: &mut QPaintDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPaintDevice16devicePixelRatioEv()};
    unsafe {_ZNK12QPaintDevice16devicePixelRatioEv()};
    return 1;
  }
}

impl /*struct*/ QPaintDevice {
  pub fn height<T: QPaintDevice_height>(&mut self, value: T) -> i32 {
    value.height(self);
    return 1;
  }
}

pub trait QPaintDevice_height {
  fn height(self, this: &mut QPaintDevice) -> i32;
}

// proto: int QPaintDevice::height();
impl<'a> /*trait*/ QPaintDevice_height for () {
  fn height(self, this: &mut QPaintDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPaintDevice6heightEv()};
    unsafe {_ZNK12QPaintDevice6heightEv()};
    return 1;
  }
}

impl /*struct*/ QPaintDevice {
  pub fn depth<T: QPaintDevice_depth>(&mut self, value: T) -> i32 {
    value.depth(self);
    return 1;
  }
}

pub trait QPaintDevice_depth {
  fn depth(self, this: &mut QPaintDevice) -> i32;
}

// proto: int QPaintDevice::depth();
impl<'a> /*trait*/ QPaintDevice_depth for () {
  fn depth(self, this: &mut QPaintDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPaintDevice5depthEv()};
    unsafe {_ZNK12QPaintDevice5depthEv()};
    return 1;
  }
}

impl /*struct*/ QPaintDevice {
  pub fn paintEngine<T: QPaintDevice_paintEngine>(&mut self, value: T) -> i32 {
    value.paintEngine(self);
    return 1;
  }
}

pub trait QPaintDevice_paintEngine {
  fn paintEngine(self, this: &mut QPaintDevice) -> i32;
}

// proto: QPaintEngine * QPaintDevice::paintEngine();
impl<'a> /*trait*/ QPaintDevice_paintEngine for () {
  fn paintEngine(self, this: &mut QPaintDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPaintDevice11paintEngineEv()};
    unsafe {_ZNK12QPaintDevice11paintEngineEv()};
    return 1;
  }
}

impl /*struct*/ QPaintDevice {
  pub fn logicalDpiY<T: QPaintDevice_logicalDpiY>(&mut self, value: T) -> i32 {
    value.logicalDpiY(self);
    return 1;
  }
}

pub trait QPaintDevice_logicalDpiY {
  fn logicalDpiY(self, this: &mut QPaintDevice) -> i32;
}

// proto: int QPaintDevice::logicalDpiY();
impl<'a> /*trait*/ QPaintDevice_logicalDpiY for () {
  fn logicalDpiY(self, this: &mut QPaintDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPaintDevice11logicalDpiYEv()};
    unsafe {_ZNK12QPaintDevice11logicalDpiYEv()};
    return 1;
  }
}

impl /*struct*/ QPaintDevice {
  pub fn FreeQPaintDevice<T: QPaintDevice_FreeQPaintDevice>(&mut self, value: T) -> i32 {
    value.FreeQPaintDevice(self);
    return 1;
  }
}

pub trait QPaintDevice_FreeQPaintDevice {
  fn FreeQPaintDevice(self, this: &mut QPaintDevice) -> i32;
}

// proto: void QPaintDevice::FreeQPaintDevice();
impl<'a> /*trait*/ QPaintDevice_FreeQPaintDevice for () {
  fn FreeQPaintDevice(self, this: &mut QPaintDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPaintDeviceD0Ev()};
    unsafe {_ZN12QPaintDeviceD0Ev()};
    return 1;
  }
}

impl /*struct*/ QPaintDevice {
  pub fn logicalDpiX<T: QPaintDevice_logicalDpiX>(&mut self, value: T) -> i32 {
    value.logicalDpiX(self);
    return 1;
  }
}

pub trait QPaintDevice_logicalDpiX {
  fn logicalDpiX(self, this: &mut QPaintDevice) -> i32;
}

// proto: int QPaintDevice::logicalDpiX();
impl<'a> /*trait*/ QPaintDevice_logicalDpiX for () {
  fn logicalDpiX(self, this: &mut QPaintDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPaintDevice11logicalDpiXEv()};
    unsafe {_ZNK12QPaintDevice11logicalDpiXEv()};
    return 1;
  }
}

