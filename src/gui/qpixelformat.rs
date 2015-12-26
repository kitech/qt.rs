// auto generated, do not modify.
// created: Sat Dec 26 10:52:38 2015
// src-file: /QtGui/qpixelformat.h
// dst-file: /src/gui/qpixelformat.rs
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
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QPixelFormat_Class_Size() -> c_int;
  // proto:  uchar QPixelFormat::blackSize();
  fn _ZNK12QPixelFormat9blackSizeEv(qthis: *mut c_void) -> c_uchar;
  // proto:  void QPixelFormat::QPixelFormat();
  fn dector_ZN12QPixelFormatC1Ev() -> *mut c_void;
  fn _ZN12QPixelFormatC1Ev(qthis: *mut c_void);
  // proto:  uchar QPixelFormat::subEnum();
  fn _ZNK12QPixelFormat7subEnumEv(qthis: *mut c_void) -> c_uchar;
  // proto:  uchar QPixelFormat::greenSize();
  fn _ZNK12QPixelFormat9greenSizeEv(qthis: *mut c_void) -> c_uchar;
  // proto:  uchar QPixelFormat::lightnessSize();
  fn _ZNK12QPixelFormat13lightnessSizeEv(qthis: *mut c_void) -> c_uchar;
  // proto:  uchar QPixelFormat::bitsPerPixel();
  fn _ZNK12QPixelFormat12bitsPerPixelEv(qthis: *mut c_void) -> c_uchar;
  // proto:  uchar QPixelFormat::alphaSize();
  fn _ZNK12QPixelFormat9alphaSizeEv(qthis: *mut c_void) -> c_uchar;
  // proto:  uchar QPixelFormat::magentaSize();
  fn _ZNK12QPixelFormat11magentaSizeEv(qthis: *mut c_void) -> c_uchar;
  // proto:  uchar QPixelFormat::hueSize();
  fn _ZNK12QPixelFormat7hueSizeEv(qthis: *mut c_void) -> c_uchar;
  // proto:  uchar QPixelFormat::saturationSize();
  fn _ZNK12QPixelFormat14saturationSizeEv(qthis: *mut c_void) -> c_uchar;
  // proto:  uchar QPixelFormat::brightnessSize();
  fn _ZNK12QPixelFormat14brightnessSizeEv(qthis: *mut c_void) -> c_uchar;
  // proto:  uchar QPixelFormat::yellowSize();
  fn _ZNK12QPixelFormat10yellowSizeEv(qthis: *mut c_void) -> c_uchar;
  // proto:  uchar QPixelFormat::redSize();
  fn _ZNK12QPixelFormat7redSizeEv(qthis: *mut c_void) -> c_uchar;
  // proto:  uchar QPixelFormat::blueSize();
  fn _ZNK12QPixelFormat8blueSizeEv(qthis: *mut c_void) -> c_uchar;
  // proto:  uchar QPixelFormat::cyanSize();
  fn _ZNK12QPixelFormat8cyanSizeEv(qthis: *mut c_void) -> c_uchar;
  // proto:  uchar QPixelFormat::channelCount();
  fn _ZNK12QPixelFormat12channelCountEv(qthis: *mut c_void) -> c_uchar;
} // <= ext block end

// body block begin =>
// class sizeof(QPixelFormat)=8
pub struct QPixelFormat {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QPixelFormat {
  pub fn inheritFrom(qthis: *mut c_void) -> QPixelFormat {
    return QPixelFormat{qclsinst: qthis};
  }
}
  // proto:  uchar QPixelFormat::blackSize();
impl /*struct*/ QPixelFormat {
  pub fn blackSize<RetType, T: QPixelFormat_blackSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.blackSize(self);
    // return 1;
  }
}

pub trait QPixelFormat_blackSize<RetType> {
  fn blackSize(self , rsthis: & QPixelFormat) -> RetType;
}

  // proto:  uchar QPixelFormat::blackSize();
impl<'a> /*trait*/ QPixelFormat_blackSize<u8> for () {
  fn blackSize(self , rsthis: & QPixelFormat) -> u8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPixelFormat9blackSizeEv()};
    let mut ret = unsafe {_ZNK12QPixelFormat9blackSizeEv(rsthis.qclsinst)};
    return ret as u8;
    // return 1;
  }
}

  // proto:  void QPixelFormat::QPixelFormat();
impl /*struct*/ QPixelFormat {
  pub fn New<T: QPixelFormat_New>(value: T) -> QPixelFormat {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QPixelFormat_New {
  fn New(self) -> QPixelFormat;
}

  // proto:  void QPixelFormat::QPixelFormat();
impl<'a> /*trait*/ QPixelFormat_New for () {
  fn New(self) -> QPixelFormat {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPixelFormatC1Ev()};
    let ctysz: c_int = unsafe{QPixelFormat_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    // unsafe {_ZN12QPixelFormatC1Ev(qthis)};
    let qthis: *mut c_void = unsafe {dector_ZN12QPixelFormatC1Ev()};
    let rsthis = QPixelFormat{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  uchar QPixelFormat::subEnum();
impl /*struct*/ QPixelFormat {
  pub fn subEnum<RetType, T: QPixelFormat_subEnum<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.subEnum(self);
    // return 1;
  }
}

pub trait QPixelFormat_subEnum<RetType> {
  fn subEnum(self , rsthis: & QPixelFormat) -> RetType;
}

  // proto:  uchar QPixelFormat::subEnum();
impl<'a> /*trait*/ QPixelFormat_subEnum<u8> for () {
  fn subEnum(self , rsthis: & QPixelFormat) -> u8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPixelFormat7subEnumEv()};
    let mut ret = unsafe {_ZNK12QPixelFormat7subEnumEv(rsthis.qclsinst)};
    return ret as u8;
    // return 1;
  }
}

  // proto:  uchar QPixelFormat::greenSize();
impl /*struct*/ QPixelFormat {
  pub fn greenSize<RetType, T: QPixelFormat_greenSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.greenSize(self);
    // return 1;
  }
}

pub trait QPixelFormat_greenSize<RetType> {
  fn greenSize(self , rsthis: & QPixelFormat) -> RetType;
}

  // proto:  uchar QPixelFormat::greenSize();
impl<'a> /*trait*/ QPixelFormat_greenSize<u8> for () {
  fn greenSize(self , rsthis: & QPixelFormat) -> u8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPixelFormat9greenSizeEv()};
    let mut ret = unsafe {_ZNK12QPixelFormat9greenSizeEv(rsthis.qclsinst)};
    return ret as u8;
    // return 1;
  }
}

  // proto:  uchar QPixelFormat::lightnessSize();
impl /*struct*/ QPixelFormat {
  pub fn lightnessSize<RetType, T: QPixelFormat_lightnessSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.lightnessSize(self);
    // return 1;
  }
}

pub trait QPixelFormat_lightnessSize<RetType> {
  fn lightnessSize(self , rsthis: & QPixelFormat) -> RetType;
}

  // proto:  uchar QPixelFormat::lightnessSize();
impl<'a> /*trait*/ QPixelFormat_lightnessSize<u8> for () {
  fn lightnessSize(self , rsthis: & QPixelFormat) -> u8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPixelFormat13lightnessSizeEv()};
    let mut ret = unsafe {_ZNK12QPixelFormat13lightnessSizeEv(rsthis.qclsinst)};
    return ret as u8;
    // return 1;
  }
}

  // proto:  uchar QPixelFormat::bitsPerPixel();
impl /*struct*/ QPixelFormat {
  pub fn bitsPerPixel<RetType, T: QPixelFormat_bitsPerPixel<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.bitsPerPixel(self);
    // return 1;
  }
}

pub trait QPixelFormat_bitsPerPixel<RetType> {
  fn bitsPerPixel(self , rsthis: & QPixelFormat) -> RetType;
}

  // proto:  uchar QPixelFormat::bitsPerPixel();
impl<'a> /*trait*/ QPixelFormat_bitsPerPixel<u8> for () {
  fn bitsPerPixel(self , rsthis: & QPixelFormat) -> u8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPixelFormat12bitsPerPixelEv()};
    let mut ret = unsafe {_ZNK12QPixelFormat12bitsPerPixelEv(rsthis.qclsinst)};
    return ret as u8;
    // return 1;
  }
}

  // proto:  uchar QPixelFormat::alphaSize();
impl /*struct*/ QPixelFormat {
  pub fn alphaSize<RetType, T: QPixelFormat_alphaSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.alphaSize(self);
    // return 1;
  }
}

pub trait QPixelFormat_alphaSize<RetType> {
  fn alphaSize(self , rsthis: & QPixelFormat) -> RetType;
}

  // proto:  uchar QPixelFormat::alphaSize();
impl<'a> /*trait*/ QPixelFormat_alphaSize<u8> for () {
  fn alphaSize(self , rsthis: & QPixelFormat) -> u8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPixelFormat9alphaSizeEv()};
    let mut ret = unsafe {_ZNK12QPixelFormat9alphaSizeEv(rsthis.qclsinst)};
    return ret as u8;
    // return 1;
  }
}

  // proto:  uchar QPixelFormat::magentaSize();
impl /*struct*/ QPixelFormat {
  pub fn magentaSize<RetType, T: QPixelFormat_magentaSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.magentaSize(self);
    // return 1;
  }
}

pub trait QPixelFormat_magentaSize<RetType> {
  fn magentaSize(self , rsthis: & QPixelFormat) -> RetType;
}

  // proto:  uchar QPixelFormat::magentaSize();
impl<'a> /*trait*/ QPixelFormat_magentaSize<u8> for () {
  fn magentaSize(self , rsthis: & QPixelFormat) -> u8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPixelFormat11magentaSizeEv()};
    let mut ret = unsafe {_ZNK12QPixelFormat11magentaSizeEv(rsthis.qclsinst)};
    return ret as u8;
    // return 1;
  }
}

  // proto:  uchar QPixelFormat::hueSize();
impl /*struct*/ QPixelFormat {
  pub fn hueSize<RetType, T: QPixelFormat_hueSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hueSize(self);
    // return 1;
  }
}

pub trait QPixelFormat_hueSize<RetType> {
  fn hueSize(self , rsthis: & QPixelFormat) -> RetType;
}

  // proto:  uchar QPixelFormat::hueSize();
impl<'a> /*trait*/ QPixelFormat_hueSize<u8> for () {
  fn hueSize(self , rsthis: & QPixelFormat) -> u8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPixelFormat7hueSizeEv()};
    let mut ret = unsafe {_ZNK12QPixelFormat7hueSizeEv(rsthis.qclsinst)};
    return ret as u8;
    // return 1;
  }
}

  // proto:  uchar QPixelFormat::saturationSize();
impl /*struct*/ QPixelFormat {
  pub fn saturationSize<RetType, T: QPixelFormat_saturationSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.saturationSize(self);
    // return 1;
  }
}

pub trait QPixelFormat_saturationSize<RetType> {
  fn saturationSize(self , rsthis: & QPixelFormat) -> RetType;
}

  // proto:  uchar QPixelFormat::saturationSize();
impl<'a> /*trait*/ QPixelFormat_saturationSize<u8> for () {
  fn saturationSize(self , rsthis: & QPixelFormat) -> u8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPixelFormat14saturationSizeEv()};
    let mut ret = unsafe {_ZNK12QPixelFormat14saturationSizeEv(rsthis.qclsinst)};
    return ret as u8;
    // return 1;
  }
}

  // proto:  uchar QPixelFormat::brightnessSize();
impl /*struct*/ QPixelFormat {
  pub fn brightnessSize<RetType, T: QPixelFormat_brightnessSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.brightnessSize(self);
    // return 1;
  }
}

pub trait QPixelFormat_brightnessSize<RetType> {
  fn brightnessSize(self , rsthis: & QPixelFormat) -> RetType;
}

  // proto:  uchar QPixelFormat::brightnessSize();
impl<'a> /*trait*/ QPixelFormat_brightnessSize<u8> for () {
  fn brightnessSize(self , rsthis: & QPixelFormat) -> u8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPixelFormat14brightnessSizeEv()};
    let mut ret = unsafe {_ZNK12QPixelFormat14brightnessSizeEv(rsthis.qclsinst)};
    return ret as u8;
    // return 1;
  }
}

  // proto:  uchar QPixelFormat::yellowSize();
impl /*struct*/ QPixelFormat {
  pub fn yellowSize<RetType, T: QPixelFormat_yellowSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.yellowSize(self);
    // return 1;
  }
}

pub trait QPixelFormat_yellowSize<RetType> {
  fn yellowSize(self , rsthis: & QPixelFormat) -> RetType;
}

  // proto:  uchar QPixelFormat::yellowSize();
impl<'a> /*trait*/ QPixelFormat_yellowSize<u8> for () {
  fn yellowSize(self , rsthis: & QPixelFormat) -> u8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPixelFormat10yellowSizeEv()};
    let mut ret = unsafe {_ZNK12QPixelFormat10yellowSizeEv(rsthis.qclsinst)};
    return ret as u8;
    // return 1;
  }
}

  // proto:  uchar QPixelFormat::redSize();
impl /*struct*/ QPixelFormat {
  pub fn redSize<RetType, T: QPixelFormat_redSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.redSize(self);
    // return 1;
  }
}

pub trait QPixelFormat_redSize<RetType> {
  fn redSize(self , rsthis: & QPixelFormat) -> RetType;
}

  // proto:  uchar QPixelFormat::redSize();
impl<'a> /*trait*/ QPixelFormat_redSize<u8> for () {
  fn redSize(self , rsthis: & QPixelFormat) -> u8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPixelFormat7redSizeEv()};
    let mut ret = unsafe {_ZNK12QPixelFormat7redSizeEv(rsthis.qclsinst)};
    return ret as u8;
    // return 1;
  }
}

  // proto:  uchar QPixelFormat::blueSize();
impl /*struct*/ QPixelFormat {
  pub fn blueSize<RetType, T: QPixelFormat_blueSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.blueSize(self);
    // return 1;
  }
}

pub trait QPixelFormat_blueSize<RetType> {
  fn blueSize(self , rsthis: & QPixelFormat) -> RetType;
}

  // proto:  uchar QPixelFormat::blueSize();
impl<'a> /*trait*/ QPixelFormat_blueSize<u8> for () {
  fn blueSize(self , rsthis: & QPixelFormat) -> u8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPixelFormat8blueSizeEv()};
    let mut ret = unsafe {_ZNK12QPixelFormat8blueSizeEv(rsthis.qclsinst)};
    return ret as u8;
    // return 1;
  }
}

  // proto:  uchar QPixelFormat::cyanSize();
impl /*struct*/ QPixelFormat {
  pub fn cyanSize<RetType, T: QPixelFormat_cyanSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.cyanSize(self);
    // return 1;
  }
}

pub trait QPixelFormat_cyanSize<RetType> {
  fn cyanSize(self , rsthis: & QPixelFormat) -> RetType;
}

  // proto:  uchar QPixelFormat::cyanSize();
impl<'a> /*trait*/ QPixelFormat_cyanSize<u8> for () {
  fn cyanSize(self , rsthis: & QPixelFormat) -> u8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPixelFormat8cyanSizeEv()};
    let mut ret = unsafe {_ZNK12QPixelFormat8cyanSizeEv(rsthis.qclsinst)};
    return ret as u8;
    // return 1;
  }
}

  // proto:  uchar QPixelFormat::channelCount();
impl /*struct*/ QPixelFormat {
  pub fn channelCount<RetType, T: QPixelFormat_channelCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.channelCount(self);
    // return 1;
  }
}

pub trait QPixelFormat_channelCount<RetType> {
  fn channelCount(self , rsthis: & QPixelFormat) -> RetType;
}

  // proto:  uchar QPixelFormat::channelCount();
impl<'a> /*trait*/ QPixelFormat_channelCount<u8> for () {
  fn channelCount(self , rsthis: & QPixelFormat) -> u8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPixelFormat12channelCountEv()};
    let mut ret = unsafe {_ZNK12QPixelFormat12channelCountEv(rsthis.qclsinst)};
    return ret as u8;
    // return 1;
  }
}

// <= body block end

