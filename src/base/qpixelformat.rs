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
  // proto: unsigned char QPixelFormat::blackSize();
  fn _ZNK12QPixelFormat9blackSizeEv() -> i32;
  // proto: void QPixelFormat::NewQPixelFormat();
  fn _ZN12QPixelFormatC1Ev(qthis: *mut c_void) -> i32;
  // proto: unsigned char QPixelFormat::subEnum();
  fn _ZNK12QPixelFormat7subEnumEv() -> i32;
  // proto: unsigned char QPixelFormat::greenSize();
  fn _ZNK12QPixelFormat9greenSizeEv() -> i32;
  // proto: unsigned char QPixelFormat::lightnessSize();
  fn _ZNK12QPixelFormat13lightnessSizeEv() -> i32;
  // proto: unsigned char QPixelFormat::bitsPerPixel();
  fn _ZNK12QPixelFormat12bitsPerPixelEv() -> i32;
  // proto: unsigned char QPixelFormat::alphaSize();
  fn _ZNK12QPixelFormat9alphaSizeEv() -> i32;
  // proto: unsigned char QPixelFormat::magentaSize();
  fn _ZNK12QPixelFormat11magentaSizeEv() -> i32;
  // proto: unsigned char QPixelFormat::hueSize();
  fn _ZNK12QPixelFormat7hueSizeEv() -> i32;
  // proto: unsigned char QPixelFormat::saturationSize();
  fn _ZNK12QPixelFormat14saturationSizeEv() -> i32;
  // proto: unsigned char QPixelFormat::brightnessSize();
  fn _ZNK12QPixelFormat14brightnessSizeEv() -> i32;
  // proto: unsigned char QPixelFormat::yellowSize();
  fn _ZNK12QPixelFormat10yellowSizeEv() -> i32;
  // proto: unsigned char QPixelFormat::redSize();
  fn _ZNK12QPixelFormat7redSizeEv() -> i32;
  // proto: unsigned char QPixelFormat::blueSize();
  fn _ZNK12QPixelFormat8blueSizeEv() -> i32;
  // proto: unsigned char QPixelFormat::cyanSize();
  fn _ZNK12QPixelFormat8cyanSizeEv() -> i32;
  // proto: unsigned char QPixelFormat::channelCount();
  fn _ZNK12QPixelFormat12channelCountEv() -> i32;
}

// body block begin
// class sizeof(QPixelFormat)=8
pub struct QPixelFormat {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QPixelFormat {
  pub fn blackSize<T: QPixelFormat_blackSize>(&mut self, value: T) -> i32 {
    value.blackSize(self);
    return 1;
  }
}

pub trait QPixelFormat_blackSize {
  fn blackSize(self, this: &mut QPixelFormat) -> i32;
}

// proto: unsigned char QPixelFormat::blackSize();
impl<'a> /*trait*/ QPixelFormat_blackSize for () {
  fn blackSize(self, this: &mut QPixelFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPixelFormat9blackSizeEv()};
    unsafe {_ZNK12QPixelFormat9blackSizeEv()};
    return 1;
  }
}

impl /*struct*/ QPixelFormat {
  pub fn NewQPixelFormat<T: QPixelFormat_NewQPixelFormat>(value: T) -> QPixelFormat {
    let rsthis = value.NewQPixelFormat();
    return rsthis;
    // return 1;
  }
}

pub trait QPixelFormat_NewQPixelFormat {
  fn NewQPixelFormat(self) -> QPixelFormat;
}

// proto: void QPixelFormat::NewQPixelFormat();
impl<'a> /*trait*/ QPixelFormat_NewQPixelFormat for () {
  fn NewQPixelFormat(self) -> QPixelFormat {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPixelFormatC1Ev()};
    unsafe {_ZN12QPixelFormatC1Ev(qthis)};
    let rsthis = QPixelFormat{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QPixelFormat {
  pub fn subEnum<T: QPixelFormat_subEnum>(&mut self, value: T) -> i32 {
    value.subEnum(self);
    return 1;
  }
}

pub trait QPixelFormat_subEnum {
  fn subEnum(self, this: &mut QPixelFormat) -> i32;
}

// proto: unsigned char QPixelFormat::subEnum();
impl<'a> /*trait*/ QPixelFormat_subEnum for () {
  fn subEnum(self, this: &mut QPixelFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPixelFormat7subEnumEv()};
    unsafe {_ZNK12QPixelFormat7subEnumEv()};
    return 1;
  }
}

impl /*struct*/ QPixelFormat {
  pub fn greenSize<T: QPixelFormat_greenSize>(&mut self, value: T) -> i32 {
    value.greenSize(self);
    return 1;
  }
}

pub trait QPixelFormat_greenSize {
  fn greenSize(self, this: &mut QPixelFormat) -> i32;
}

// proto: unsigned char QPixelFormat::greenSize();
impl<'a> /*trait*/ QPixelFormat_greenSize for () {
  fn greenSize(self, this: &mut QPixelFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPixelFormat9greenSizeEv()};
    unsafe {_ZNK12QPixelFormat9greenSizeEv()};
    return 1;
  }
}

impl /*struct*/ QPixelFormat {
  pub fn lightnessSize<T: QPixelFormat_lightnessSize>(&mut self, value: T) -> i32 {
    value.lightnessSize(self);
    return 1;
  }
}

pub trait QPixelFormat_lightnessSize {
  fn lightnessSize(self, this: &mut QPixelFormat) -> i32;
}

// proto: unsigned char QPixelFormat::lightnessSize();
impl<'a> /*trait*/ QPixelFormat_lightnessSize for () {
  fn lightnessSize(self, this: &mut QPixelFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPixelFormat13lightnessSizeEv()};
    unsafe {_ZNK12QPixelFormat13lightnessSizeEv()};
    return 1;
  }
}

impl /*struct*/ QPixelFormat {
  pub fn bitsPerPixel<T: QPixelFormat_bitsPerPixel>(&mut self, value: T) -> i32 {
    value.bitsPerPixel(self);
    return 1;
  }
}

pub trait QPixelFormat_bitsPerPixel {
  fn bitsPerPixel(self, this: &mut QPixelFormat) -> i32;
}

// proto: unsigned char QPixelFormat::bitsPerPixel();
impl<'a> /*trait*/ QPixelFormat_bitsPerPixel for () {
  fn bitsPerPixel(self, this: &mut QPixelFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPixelFormat12bitsPerPixelEv()};
    unsafe {_ZNK12QPixelFormat12bitsPerPixelEv()};
    return 1;
  }
}

impl /*struct*/ QPixelFormat {
  pub fn alphaSize<T: QPixelFormat_alphaSize>(&mut self, value: T) -> i32 {
    value.alphaSize(self);
    return 1;
  }
}

pub trait QPixelFormat_alphaSize {
  fn alphaSize(self, this: &mut QPixelFormat) -> i32;
}

// proto: unsigned char QPixelFormat::alphaSize();
impl<'a> /*trait*/ QPixelFormat_alphaSize for () {
  fn alphaSize(self, this: &mut QPixelFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPixelFormat9alphaSizeEv()};
    unsafe {_ZNK12QPixelFormat9alphaSizeEv()};
    return 1;
  }
}

impl /*struct*/ QPixelFormat {
  pub fn magentaSize<T: QPixelFormat_magentaSize>(&mut self, value: T) -> i32 {
    value.magentaSize(self);
    return 1;
  }
}

pub trait QPixelFormat_magentaSize {
  fn magentaSize(self, this: &mut QPixelFormat) -> i32;
}

// proto: unsigned char QPixelFormat::magentaSize();
impl<'a> /*trait*/ QPixelFormat_magentaSize for () {
  fn magentaSize(self, this: &mut QPixelFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPixelFormat11magentaSizeEv()};
    unsafe {_ZNK12QPixelFormat11magentaSizeEv()};
    return 1;
  }
}

impl /*struct*/ QPixelFormat {
  pub fn hueSize<T: QPixelFormat_hueSize>(&mut self, value: T) -> i32 {
    value.hueSize(self);
    return 1;
  }
}

pub trait QPixelFormat_hueSize {
  fn hueSize(self, this: &mut QPixelFormat) -> i32;
}

// proto: unsigned char QPixelFormat::hueSize();
impl<'a> /*trait*/ QPixelFormat_hueSize for () {
  fn hueSize(self, this: &mut QPixelFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPixelFormat7hueSizeEv()};
    unsafe {_ZNK12QPixelFormat7hueSizeEv()};
    return 1;
  }
}

impl /*struct*/ QPixelFormat {
  pub fn saturationSize<T: QPixelFormat_saturationSize>(&mut self, value: T) -> i32 {
    value.saturationSize(self);
    return 1;
  }
}

pub trait QPixelFormat_saturationSize {
  fn saturationSize(self, this: &mut QPixelFormat) -> i32;
}

// proto: unsigned char QPixelFormat::saturationSize();
impl<'a> /*trait*/ QPixelFormat_saturationSize for () {
  fn saturationSize(self, this: &mut QPixelFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPixelFormat14saturationSizeEv()};
    unsafe {_ZNK12QPixelFormat14saturationSizeEv()};
    return 1;
  }
}

impl /*struct*/ QPixelFormat {
  pub fn brightnessSize<T: QPixelFormat_brightnessSize>(&mut self, value: T) -> i32 {
    value.brightnessSize(self);
    return 1;
  }
}

pub trait QPixelFormat_brightnessSize {
  fn brightnessSize(self, this: &mut QPixelFormat) -> i32;
}

// proto: unsigned char QPixelFormat::brightnessSize();
impl<'a> /*trait*/ QPixelFormat_brightnessSize for () {
  fn brightnessSize(self, this: &mut QPixelFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPixelFormat14brightnessSizeEv()};
    unsafe {_ZNK12QPixelFormat14brightnessSizeEv()};
    return 1;
  }
}

impl /*struct*/ QPixelFormat {
  pub fn yellowSize<T: QPixelFormat_yellowSize>(&mut self, value: T) -> i32 {
    value.yellowSize(self);
    return 1;
  }
}

pub trait QPixelFormat_yellowSize {
  fn yellowSize(self, this: &mut QPixelFormat) -> i32;
}

// proto: unsigned char QPixelFormat::yellowSize();
impl<'a> /*trait*/ QPixelFormat_yellowSize for () {
  fn yellowSize(self, this: &mut QPixelFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPixelFormat10yellowSizeEv()};
    unsafe {_ZNK12QPixelFormat10yellowSizeEv()};
    return 1;
  }
}

impl /*struct*/ QPixelFormat {
  pub fn redSize<T: QPixelFormat_redSize>(&mut self, value: T) -> i32 {
    value.redSize(self);
    return 1;
  }
}

pub trait QPixelFormat_redSize {
  fn redSize(self, this: &mut QPixelFormat) -> i32;
}

// proto: unsigned char QPixelFormat::redSize();
impl<'a> /*trait*/ QPixelFormat_redSize for () {
  fn redSize(self, this: &mut QPixelFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPixelFormat7redSizeEv()};
    unsafe {_ZNK12QPixelFormat7redSizeEv()};
    return 1;
  }
}

impl /*struct*/ QPixelFormat {
  pub fn blueSize<T: QPixelFormat_blueSize>(&mut self, value: T) -> i32 {
    value.blueSize(self);
    return 1;
  }
}

pub trait QPixelFormat_blueSize {
  fn blueSize(self, this: &mut QPixelFormat) -> i32;
}

// proto: unsigned char QPixelFormat::blueSize();
impl<'a> /*trait*/ QPixelFormat_blueSize for () {
  fn blueSize(self, this: &mut QPixelFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPixelFormat8blueSizeEv()};
    unsafe {_ZNK12QPixelFormat8blueSizeEv()};
    return 1;
  }
}

impl /*struct*/ QPixelFormat {
  pub fn cyanSize<T: QPixelFormat_cyanSize>(&mut self, value: T) -> i32 {
    value.cyanSize(self);
    return 1;
  }
}

pub trait QPixelFormat_cyanSize {
  fn cyanSize(self, this: &mut QPixelFormat) -> i32;
}

// proto: unsigned char QPixelFormat::cyanSize();
impl<'a> /*trait*/ QPixelFormat_cyanSize for () {
  fn cyanSize(self, this: &mut QPixelFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPixelFormat8cyanSizeEv()};
    unsafe {_ZNK12QPixelFormat8cyanSizeEv()};
    return 1;
  }
}

impl /*struct*/ QPixelFormat {
  pub fn channelCount<T: QPixelFormat_channelCount>(&mut self, value: T) -> i32 {
    value.channelCount(self);
    return 1;
  }
}

pub trait QPixelFormat_channelCount {
  fn channelCount(self, this: &mut QPixelFormat) -> i32;
}

// proto: unsigned char QPixelFormat::channelCount();
impl<'a> /*trait*/ QPixelFormat_channelCount for () {
  fn channelCount(self, this: &mut QPixelFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPixelFormat12channelCountEv()};
    unsafe {_ZNK12QPixelFormat12channelCountEv()};
    return 1;
  }
}

