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
  // proto:  unsigned char QPixelFormat::blackSize();
  fn _ZNK12QPixelFormat9blackSizeEv(qthis: *mut c_void) -> c_uchar;
  // proto:  void QPixelFormat::NewQPixelFormat();
  fn _ZN12QPixelFormatC1Ev(qthis: *mut c_void) ;
  // proto:  unsigned char QPixelFormat::subEnum();
  fn _ZNK12QPixelFormat7subEnumEv(qthis: *mut c_void) -> c_uchar;
  // proto:  unsigned char QPixelFormat::greenSize();
  fn _ZNK12QPixelFormat9greenSizeEv(qthis: *mut c_void) -> c_uchar;
  // proto:  unsigned char QPixelFormat::lightnessSize();
  fn _ZNK12QPixelFormat13lightnessSizeEv(qthis: *mut c_void) -> c_uchar;
  // proto:  unsigned char QPixelFormat::bitsPerPixel();
  fn _ZNK12QPixelFormat12bitsPerPixelEv(qthis: *mut c_void) -> c_uchar;
  // proto:  unsigned char QPixelFormat::alphaSize();
  fn _ZNK12QPixelFormat9alphaSizeEv(qthis: *mut c_void) -> c_uchar;
  // proto:  unsigned char QPixelFormat::magentaSize();
  fn _ZNK12QPixelFormat11magentaSizeEv(qthis: *mut c_void) -> c_uchar;
  // proto:  unsigned char QPixelFormat::hueSize();
  fn _ZNK12QPixelFormat7hueSizeEv(qthis: *mut c_void) -> c_uchar;
  // proto:  unsigned char QPixelFormat::saturationSize();
  fn _ZNK12QPixelFormat14saturationSizeEv(qthis: *mut c_void) -> c_uchar;
  // proto:  unsigned char QPixelFormat::brightnessSize();
  fn _ZNK12QPixelFormat14brightnessSizeEv(qthis: *mut c_void) -> c_uchar;
  // proto:  unsigned char QPixelFormat::yellowSize();
  fn _ZNK12QPixelFormat10yellowSizeEv(qthis: *mut c_void) -> c_uchar;
  // proto:  unsigned char QPixelFormat::redSize();
  fn _ZNK12QPixelFormat7redSizeEv(qthis: *mut c_void) -> c_uchar;
  // proto:  unsigned char QPixelFormat::blueSize();
  fn _ZNK12QPixelFormat8blueSizeEv(qthis: *mut c_void) -> c_uchar;
  // proto:  unsigned char QPixelFormat::cyanSize();
  fn _ZNK12QPixelFormat8cyanSizeEv(qthis: *mut c_void) -> c_uchar;
  // proto:  unsigned char QPixelFormat::channelCount();
  fn _ZNK12QPixelFormat12channelCountEv(qthis: *mut c_void) -> c_uchar;
}

// body block begin
// class sizeof(QPixelFormat)=8
pub struct QPixelFormat {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QPixelFormat {
  pub fn blackSize<T: QPixelFormat_blackSize>(&mut self, value: T) -> u8 {
    return value.blackSize(self);
    // return 1;
  }
}

pub trait QPixelFormat_blackSize {
  fn blackSize(self, rsthis: &mut QPixelFormat) -> u8;
}

// proto:  unsigned char QPixelFormat::blackSize();
impl<'a> /*trait*/ QPixelFormat_blackSize for () {
  fn blackSize(self, rsthis: &mut QPixelFormat) -> u8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPixelFormat9blackSizeEv()};
    let mut ret = unsafe {_ZNK12QPixelFormat9blackSizeEv(rsthis.qclsinst)};
    return ret as u8;
    // return 1;
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
  pub fn subEnum<T: QPixelFormat_subEnum>(&mut self, value: T) -> u8 {
    return value.subEnum(self);
    // return 1;
  }
}

pub trait QPixelFormat_subEnum {
  fn subEnum(self, rsthis: &mut QPixelFormat) -> u8;
}

// proto:  unsigned char QPixelFormat::subEnum();
impl<'a> /*trait*/ QPixelFormat_subEnum for () {
  fn subEnum(self, rsthis: &mut QPixelFormat) -> u8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPixelFormat7subEnumEv()};
    let mut ret = unsafe {_ZNK12QPixelFormat7subEnumEv(rsthis.qclsinst)};
    return ret as u8;
    // return 1;
  }
}

impl /*struct*/ QPixelFormat {
  pub fn greenSize<T: QPixelFormat_greenSize>(&mut self, value: T) -> u8 {
    return value.greenSize(self);
    // return 1;
  }
}

pub trait QPixelFormat_greenSize {
  fn greenSize(self, rsthis: &mut QPixelFormat) -> u8;
}

// proto:  unsigned char QPixelFormat::greenSize();
impl<'a> /*trait*/ QPixelFormat_greenSize for () {
  fn greenSize(self, rsthis: &mut QPixelFormat) -> u8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPixelFormat9greenSizeEv()};
    let mut ret = unsafe {_ZNK12QPixelFormat9greenSizeEv(rsthis.qclsinst)};
    return ret as u8;
    // return 1;
  }
}

impl /*struct*/ QPixelFormat {
  pub fn lightnessSize<T: QPixelFormat_lightnessSize>(&mut self, value: T) -> u8 {
    return value.lightnessSize(self);
    // return 1;
  }
}

pub trait QPixelFormat_lightnessSize {
  fn lightnessSize(self, rsthis: &mut QPixelFormat) -> u8;
}

// proto:  unsigned char QPixelFormat::lightnessSize();
impl<'a> /*trait*/ QPixelFormat_lightnessSize for () {
  fn lightnessSize(self, rsthis: &mut QPixelFormat) -> u8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPixelFormat13lightnessSizeEv()};
    let mut ret = unsafe {_ZNK12QPixelFormat13lightnessSizeEv(rsthis.qclsinst)};
    return ret as u8;
    // return 1;
  }
}

impl /*struct*/ QPixelFormat {
  pub fn bitsPerPixel<T: QPixelFormat_bitsPerPixel>(&mut self, value: T) -> u8 {
    return value.bitsPerPixel(self);
    // return 1;
  }
}

pub trait QPixelFormat_bitsPerPixel {
  fn bitsPerPixel(self, rsthis: &mut QPixelFormat) -> u8;
}

// proto:  unsigned char QPixelFormat::bitsPerPixel();
impl<'a> /*trait*/ QPixelFormat_bitsPerPixel for () {
  fn bitsPerPixel(self, rsthis: &mut QPixelFormat) -> u8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPixelFormat12bitsPerPixelEv()};
    let mut ret = unsafe {_ZNK12QPixelFormat12bitsPerPixelEv(rsthis.qclsinst)};
    return ret as u8;
    // return 1;
  }
}

impl /*struct*/ QPixelFormat {
  pub fn alphaSize<T: QPixelFormat_alphaSize>(&mut self, value: T) -> u8 {
    return value.alphaSize(self);
    // return 1;
  }
}

pub trait QPixelFormat_alphaSize {
  fn alphaSize(self, rsthis: &mut QPixelFormat) -> u8;
}

// proto:  unsigned char QPixelFormat::alphaSize();
impl<'a> /*trait*/ QPixelFormat_alphaSize for () {
  fn alphaSize(self, rsthis: &mut QPixelFormat) -> u8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPixelFormat9alphaSizeEv()};
    let mut ret = unsafe {_ZNK12QPixelFormat9alphaSizeEv(rsthis.qclsinst)};
    return ret as u8;
    // return 1;
  }
}

impl /*struct*/ QPixelFormat {
  pub fn magentaSize<T: QPixelFormat_magentaSize>(&mut self, value: T) -> u8 {
    return value.magentaSize(self);
    // return 1;
  }
}

pub trait QPixelFormat_magentaSize {
  fn magentaSize(self, rsthis: &mut QPixelFormat) -> u8;
}

// proto:  unsigned char QPixelFormat::magentaSize();
impl<'a> /*trait*/ QPixelFormat_magentaSize for () {
  fn magentaSize(self, rsthis: &mut QPixelFormat) -> u8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPixelFormat11magentaSizeEv()};
    let mut ret = unsafe {_ZNK12QPixelFormat11magentaSizeEv(rsthis.qclsinst)};
    return ret as u8;
    // return 1;
  }
}

impl /*struct*/ QPixelFormat {
  pub fn hueSize<T: QPixelFormat_hueSize>(&mut self, value: T) -> u8 {
    return value.hueSize(self);
    // return 1;
  }
}

pub trait QPixelFormat_hueSize {
  fn hueSize(self, rsthis: &mut QPixelFormat) -> u8;
}

// proto:  unsigned char QPixelFormat::hueSize();
impl<'a> /*trait*/ QPixelFormat_hueSize for () {
  fn hueSize(self, rsthis: &mut QPixelFormat) -> u8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPixelFormat7hueSizeEv()};
    let mut ret = unsafe {_ZNK12QPixelFormat7hueSizeEv(rsthis.qclsinst)};
    return ret as u8;
    // return 1;
  }
}

impl /*struct*/ QPixelFormat {
  pub fn saturationSize<T: QPixelFormat_saturationSize>(&mut self, value: T) -> u8 {
    return value.saturationSize(self);
    // return 1;
  }
}

pub trait QPixelFormat_saturationSize {
  fn saturationSize(self, rsthis: &mut QPixelFormat) -> u8;
}

// proto:  unsigned char QPixelFormat::saturationSize();
impl<'a> /*trait*/ QPixelFormat_saturationSize for () {
  fn saturationSize(self, rsthis: &mut QPixelFormat) -> u8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPixelFormat14saturationSizeEv()};
    let mut ret = unsafe {_ZNK12QPixelFormat14saturationSizeEv(rsthis.qclsinst)};
    return ret as u8;
    // return 1;
  }
}

impl /*struct*/ QPixelFormat {
  pub fn brightnessSize<T: QPixelFormat_brightnessSize>(&mut self, value: T) -> u8 {
    return value.brightnessSize(self);
    // return 1;
  }
}

pub trait QPixelFormat_brightnessSize {
  fn brightnessSize(self, rsthis: &mut QPixelFormat) -> u8;
}

// proto:  unsigned char QPixelFormat::brightnessSize();
impl<'a> /*trait*/ QPixelFormat_brightnessSize for () {
  fn brightnessSize(self, rsthis: &mut QPixelFormat) -> u8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPixelFormat14brightnessSizeEv()};
    let mut ret = unsafe {_ZNK12QPixelFormat14brightnessSizeEv(rsthis.qclsinst)};
    return ret as u8;
    // return 1;
  }
}

impl /*struct*/ QPixelFormat {
  pub fn yellowSize<T: QPixelFormat_yellowSize>(&mut self, value: T) -> u8 {
    return value.yellowSize(self);
    // return 1;
  }
}

pub trait QPixelFormat_yellowSize {
  fn yellowSize(self, rsthis: &mut QPixelFormat) -> u8;
}

// proto:  unsigned char QPixelFormat::yellowSize();
impl<'a> /*trait*/ QPixelFormat_yellowSize for () {
  fn yellowSize(self, rsthis: &mut QPixelFormat) -> u8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPixelFormat10yellowSizeEv()};
    let mut ret = unsafe {_ZNK12QPixelFormat10yellowSizeEv(rsthis.qclsinst)};
    return ret as u8;
    // return 1;
  }
}

impl /*struct*/ QPixelFormat {
  pub fn redSize<T: QPixelFormat_redSize>(&mut self, value: T) -> u8 {
    return value.redSize(self);
    // return 1;
  }
}

pub trait QPixelFormat_redSize {
  fn redSize(self, rsthis: &mut QPixelFormat) -> u8;
}

// proto:  unsigned char QPixelFormat::redSize();
impl<'a> /*trait*/ QPixelFormat_redSize for () {
  fn redSize(self, rsthis: &mut QPixelFormat) -> u8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPixelFormat7redSizeEv()};
    let mut ret = unsafe {_ZNK12QPixelFormat7redSizeEv(rsthis.qclsinst)};
    return ret as u8;
    // return 1;
  }
}

impl /*struct*/ QPixelFormat {
  pub fn blueSize<T: QPixelFormat_blueSize>(&mut self, value: T) -> u8 {
    return value.blueSize(self);
    // return 1;
  }
}

pub trait QPixelFormat_blueSize {
  fn blueSize(self, rsthis: &mut QPixelFormat) -> u8;
}

// proto:  unsigned char QPixelFormat::blueSize();
impl<'a> /*trait*/ QPixelFormat_blueSize for () {
  fn blueSize(self, rsthis: &mut QPixelFormat) -> u8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPixelFormat8blueSizeEv()};
    let mut ret = unsafe {_ZNK12QPixelFormat8blueSizeEv(rsthis.qclsinst)};
    return ret as u8;
    // return 1;
  }
}

impl /*struct*/ QPixelFormat {
  pub fn cyanSize<T: QPixelFormat_cyanSize>(&mut self, value: T) -> u8 {
    return value.cyanSize(self);
    // return 1;
  }
}

pub trait QPixelFormat_cyanSize {
  fn cyanSize(self, rsthis: &mut QPixelFormat) -> u8;
}

// proto:  unsigned char QPixelFormat::cyanSize();
impl<'a> /*trait*/ QPixelFormat_cyanSize for () {
  fn cyanSize(self, rsthis: &mut QPixelFormat) -> u8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPixelFormat8cyanSizeEv()};
    let mut ret = unsafe {_ZNK12QPixelFormat8cyanSizeEv(rsthis.qclsinst)};
    return ret as u8;
    // return 1;
  }
}

impl /*struct*/ QPixelFormat {
  pub fn channelCount<T: QPixelFormat_channelCount>(&mut self, value: T) -> u8 {
    return value.channelCount(self);
    // return 1;
  }
}

pub trait QPixelFormat_channelCount {
  fn channelCount(self, rsthis: &mut QPixelFormat) -> u8;
}

// proto:  unsigned char QPixelFormat::channelCount();
impl<'a> /*trait*/ QPixelFormat_channelCount for () {
  fn channelCount(self, rsthis: &mut QPixelFormat) -> u8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPixelFormat12channelCountEv()};
    let mut ret = unsafe {_ZNK12QPixelFormat12channelCountEv(rsthis.qclsinst)};
    return ret as u8;
    // return 1;
  }
}

