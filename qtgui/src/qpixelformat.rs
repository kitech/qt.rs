

// mod ::gui::QPixelFormat
// package qtgui
// /usr/include/qt/QtGui/qpixelformat.h
// #include <qpixelformat.h>
// #include <QtGui>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 23
*/
// import "C"
// import "unsafe"
// import "reflect"
// import "fmt"
// import "log"
// import "github.com/kitech/qt.go/qtrt"
// import "github.com/kitech/qt.go/qtcore"
use qtcore::*; // super::super::%!s(MISSING)::*;
use std::default::Default;
use std::ops::Deref;
use qtrt; // super::super::qtrt;
use super::*;
//  ext block end

//  body block begin



/*

*/
#[derive(Default)] // class sizeof(QPixelFormat)=8
pub struct QPixelFormat {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QPixelFormat_ITF interface {
//    QPixelFormat_PTR() *QPixelFormat
//}
//func (ptr *QPixelFormat) QPixelFormat_PTR() *QPixelFormat { return ptr }

impl /*struct*/ QPixelFormat {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QPixelFormat {
    return QPixelFormat{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QPixelFormat {
//  type Target = QPixelFormatBASE;
//
//  fn deref(&self) -> &QPixelFormatBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QPixelFormatBASE> for QPixelFormat {
//  fn as_ref(& self) -> & QPixelFormatBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qpixelformat.h:163
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void QPixelFormat()

/*
Creates a null pixelformat. This format maps to QImage::Format_Invalid.
*/
// QPixelFormat() ctx.fn_proto_cpp
impl /*struct*/ QPixelFormat {
  pub fn QPixelFormat_0<T: QPixelFormat_QPixelFormat_0>(value: T) -> QPixelFormat {
    let rsthis = value.QPixelFormat_0();
    return rsthis;
    // return 1;
  }
}

pub trait QPixelFormat_QPixelFormat_0 {
  fn QPixelFormat_0(self) -> QPixelFormat;
}
// QPixelFormat() ctx.fn_proto_cpp
impl<'a> /*trait*/ QPixelFormat_QPixelFormat_0 for () {
  fn QPixelFormat_0(self) -> QPixelFormat {
    // unsafe{_ZN12QPixelFormatC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN12QPixelFormatC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QPixelFormat{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpixelformat.h:164
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void QPixelFormat(QPixelFormat::ColorModel, uchar, uchar, uchar, uchar, uchar, uchar, QPixelFormat::AlphaUsage, QPixelFormat::AlphaPosition, QPixelFormat::AlphaPremultiplied, QPixelFormat::TypeInterpretation, QPixelFormat::ByteOrder, uchar)

/*
Creates a null pixelformat. This format maps to QImage::Format_Invalid.
*/
// QPixelFormat(QPixelFormat::ColorModel, uchar, uchar, uchar, uchar, uchar, uchar, QPixelFormat::AlphaUsage, QPixelFormat::AlphaPosition, QPixelFormat::AlphaPremultiplied, QPixelFormat::TypeInterpretation, QPixelFormat::ByteOrder, uchar) ctx.fn_proto_cpp
impl /*struct*/ QPixelFormat {
  pub fn QPixelFormat_1<T: QPixelFormat_QPixelFormat_1>(value: T) -> QPixelFormat {
    let rsthis = value.QPixelFormat_1();
    return rsthis;
    // return 1;
  }
}

pub trait QPixelFormat_QPixelFormat_1 {
  fn QPixelFormat_1(self) -> QPixelFormat;
}
// QPixelFormat(QPixelFormat::ColorModel, uchar, uchar, uchar, uchar, uchar, uchar, QPixelFormat::AlphaUsage, QPixelFormat::AlphaPosition, QPixelFormat::AlphaPremultiplied, QPixelFormat::TypeInterpretation, QPixelFormat::ByteOrder, uchar) ctx.fn_proto_cpp
impl<'a> /*trait*/ QPixelFormat_QPixelFormat_1 for (i32,u8,u8,u8,u8,u8,u8,i32,i32,i32,i32,i32,u8) {
  fn QPixelFormat_1(self) -> QPixelFormat {
    // unsafe{_ZN12QPixelFormatC2ENS_10ColorModelEhhhhhhNS_10AlphaUsageENS_13AlphaPositionENS_18AlphaPremultipliedENS_18TypeInterpretationENS_9ByteOrderEh()};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const u8 as usize;
    let arg2 = (&self.2) as *const u8 as usize;
    let arg3 = (&self.3) as *const u8 as usize;
    let arg4 = (&self.4) as *const u8 as usize;
    let arg5 = (&self.5) as *const u8 as usize;
    let arg6 = (&self.6) as *const u8 as usize;
    let arg7 = (&self.7) as *const i32 as usize;
    let arg8 = (&self.8) as *const i32 as usize;
    let arg9 = (&self.9) as *const i32 as usize;
    let arg10 = (&self.10) as *const i32 as usize;
    let arg11 = (&self.11) as *const i32 as usize;
    let arg12 = (&self.12) as *const u8 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN12QPixelFormatC2ENS_10ColorModelEhhhhhhNS_10AlphaUsageENS_13AlphaPositionENS_18AlphaPremultipliedENS_18TypeInterpretationENS_9ByteOrderEh", 13,qtrt::FFITY_INT,qtrt::FFITY_UINT8,qtrt::FFITY_UINT8,qtrt::FFITY_UINT8,qtrt::FFITY_UINT8,qtrt::FFITY_UINT8,qtrt::FFITY_UINT8,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_UINT8,0,0,0,arg0,arg1,arg2,arg3,arg4,arg5,arg6,arg7,arg8,arg9,arg10,arg11,arg12,0,0,0);
    let rsthis = QPixelFormat{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpixelformat.h:178
// index:0
// Public inline Visibility=Default Availability=Available
// [4] QPixelFormat::ColorModel colorModel() const

/*
Accessor function for getting the colorModel.
*/
impl /*struct*/ QPixelFormat {
  pub fn colorModel_0<RetType, T: QPixelFormat_colorModel_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.colorModel_0(self);
    // return 1;
  }
}
pub trait QPixelFormat_colorModel_0<RetType> {
  fn colorModel_0(self , rsthis: & QPixelFormat) -> RetType;
}
impl<'a> /*trait*/ QPixelFormat_colorModel_0<i32> for () {
  fn colorModel_0(self , rsthis: & QPixelFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QPixelFormat10colorModelEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpixelformat.h:179
// index:0
// Public inline Visibility=Default Availability=Available
// [1] uchar channelCount() const

/*
Accessor function for getting the channelCount. Channel Count is deduced by color channels with a size > 0 and if the size of the alpha channel is > 0.
*/
impl /*struct*/ QPixelFormat {
  pub fn channelCount_0<RetType, T: QPixelFormat_channelCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.channelCount_0(self);
    // return 1;
  }
}
pub trait QPixelFormat_channelCount_0<RetType> {
  fn channelCount_0(self , rsthis: & QPixelFormat) -> RetType;
}
impl<'a> /*trait*/ QPixelFormat_channelCount_0<u8> for () {
  fn channelCount_0(self , rsthis: & QPixelFormat) -> u8 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QPixelFormat12channelCountEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u8 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpixelformat.h:186
// index:0
// Public inline Visibility=Default Availability=Available
// [1] uchar redSize() const

/*
Accessor function for the size of the red color channel.
*/
impl /*struct*/ QPixelFormat {
  pub fn redSize_0<RetType, T: QPixelFormat_redSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.redSize_0(self);
    // return 1;
  }
}
pub trait QPixelFormat_redSize_0<RetType> {
  fn redSize_0(self , rsthis: & QPixelFormat) -> RetType;
}
impl<'a> /*trait*/ QPixelFormat_redSize_0<u8> for () {
  fn redSize_0(self , rsthis: & QPixelFormat) -> u8 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QPixelFormat7redSizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u8 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpixelformat.h:187
// index:0
// Public inline Visibility=Default Availability=Available
// [1] uchar greenSize() const

/*
Accessor function for the size of the green color channel.
*/
impl /*struct*/ QPixelFormat {
  pub fn greenSize_0<RetType, T: QPixelFormat_greenSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.greenSize_0(self);
    // return 1;
  }
}
pub trait QPixelFormat_greenSize_0<RetType> {
  fn greenSize_0(self , rsthis: & QPixelFormat) -> RetType;
}
impl<'a> /*trait*/ QPixelFormat_greenSize_0<u8> for () {
  fn greenSize_0(self , rsthis: & QPixelFormat) -> u8 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QPixelFormat9greenSizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u8 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpixelformat.h:188
// index:0
// Public inline Visibility=Default Availability=Available
// [1] uchar blueSize() const

/*
Accessor function for the size of the blue color channel.
*/
impl /*struct*/ QPixelFormat {
  pub fn blueSize_0<RetType, T: QPixelFormat_blueSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.blueSize_0(self);
    // return 1;
  }
}
pub trait QPixelFormat_blueSize_0<RetType> {
  fn blueSize_0(self , rsthis: & QPixelFormat) -> RetType;
}
impl<'a> /*trait*/ QPixelFormat_blueSize_0<u8> for () {
  fn blueSize_0(self , rsthis: & QPixelFormat) -> u8 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QPixelFormat8blueSizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u8 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpixelformat.h:190
// index:0
// Public inline Visibility=Default Availability=Available
// [1] uchar cyanSize() const

/*
Accessor function for the cyan color channel.
*/
impl /*struct*/ QPixelFormat {
  pub fn cyanSize_0<RetType, T: QPixelFormat_cyanSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cyanSize_0(self);
    // return 1;
  }
}
pub trait QPixelFormat_cyanSize_0<RetType> {
  fn cyanSize_0(self , rsthis: & QPixelFormat) -> RetType;
}
impl<'a> /*trait*/ QPixelFormat_cyanSize_0<u8> for () {
  fn cyanSize_0(self , rsthis: & QPixelFormat) -> u8 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QPixelFormat8cyanSizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u8 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpixelformat.h:191
// index:0
// Public inline Visibility=Default Availability=Available
// [1] uchar magentaSize() const

/*
Accessor function for the megenta color channel.
*/
impl /*struct*/ QPixelFormat {
  pub fn magentaSize_0<RetType, T: QPixelFormat_magentaSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.magentaSize_0(self);
    // return 1;
  }
}
pub trait QPixelFormat_magentaSize_0<RetType> {
  fn magentaSize_0(self , rsthis: & QPixelFormat) -> RetType;
}
impl<'a> /*trait*/ QPixelFormat_magentaSize_0<u8> for () {
  fn magentaSize_0(self , rsthis: & QPixelFormat) -> u8 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QPixelFormat11magentaSizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u8 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpixelformat.h:192
// index:0
// Public inline Visibility=Default Availability=Available
// [1] uchar yellowSize() const

/*
Accessor function for the yellow color channel.
*/
impl /*struct*/ QPixelFormat {
  pub fn yellowSize_0<RetType, T: QPixelFormat_yellowSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.yellowSize_0(self);
    // return 1;
  }
}
pub trait QPixelFormat_yellowSize_0<RetType> {
  fn yellowSize_0(self , rsthis: & QPixelFormat) -> RetType;
}
impl<'a> /*trait*/ QPixelFormat_yellowSize_0<u8> for () {
  fn yellowSize_0(self , rsthis: & QPixelFormat) -> u8 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QPixelFormat10yellowSizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u8 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpixelformat.h:193
// index:0
// Public inline Visibility=Default Availability=Available
// [1] uchar blackSize() const

/*
Accessor function for the black/key color channel.
*/
impl /*struct*/ QPixelFormat {
  pub fn blackSize_0<RetType, T: QPixelFormat_blackSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.blackSize_0(self);
    // return 1;
  }
}
pub trait QPixelFormat_blackSize_0<RetType> {
  fn blackSize_0(self , rsthis: & QPixelFormat) -> RetType;
}
impl<'a> /*trait*/ QPixelFormat_blackSize_0<u8> for () {
  fn blackSize_0(self , rsthis: & QPixelFormat) -> u8 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QPixelFormat9blackSizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u8 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpixelformat.h:195
// index:0
// Public inline Visibility=Default Availability=Available
// [1] uchar hueSize() const

/*
Accessor function for the hue channel size.
*/
impl /*struct*/ QPixelFormat {
  pub fn hueSize_0<RetType, T: QPixelFormat_hueSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hueSize_0(self);
    // return 1;
  }
}
pub trait QPixelFormat_hueSize_0<RetType> {
  fn hueSize_0(self , rsthis: & QPixelFormat) -> RetType;
}
impl<'a> /*trait*/ QPixelFormat_hueSize_0<u8> for () {
  fn hueSize_0(self , rsthis: & QPixelFormat) -> u8 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QPixelFormat7hueSizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u8 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpixelformat.h:196
// index:0
// Public inline Visibility=Default Availability=Available
// [1] uchar saturationSize() const

/*
Accessor function for the saturation channel size.
*/
impl /*struct*/ QPixelFormat {
  pub fn saturationSize_0<RetType, T: QPixelFormat_saturationSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.saturationSize_0(self);
    // return 1;
  }
}
pub trait QPixelFormat_saturationSize_0<RetType> {
  fn saturationSize_0(self , rsthis: & QPixelFormat) -> RetType;
}
impl<'a> /*trait*/ QPixelFormat_saturationSize_0<u8> for () {
  fn saturationSize_0(self , rsthis: & QPixelFormat) -> u8 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QPixelFormat14saturationSizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u8 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpixelformat.h:197
// index:0
// Public inline Visibility=Default Availability=Available
// [1] uchar lightnessSize() const

/*
Accessor function for the lightness channel size.
*/
impl /*struct*/ QPixelFormat {
  pub fn lightnessSize_0<RetType, T: QPixelFormat_lightnessSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.lightnessSize_0(self);
    // return 1;
  }
}
pub trait QPixelFormat_lightnessSize_0<RetType> {
  fn lightnessSize_0(self , rsthis: & QPixelFormat) -> RetType;
}
impl<'a> /*trait*/ QPixelFormat_lightnessSize_0<u8> for () {
  fn lightnessSize_0(self , rsthis: & QPixelFormat) -> u8 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QPixelFormat13lightnessSizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u8 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpixelformat.h:198
// index:0
// Public inline Visibility=Default Availability=Available
// [1] uchar brightnessSize() const

/*
Accessor function for the brightness channel size.
*/
impl /*struct*/ QPixelFormat {
  pub fn brightnessSize_0<RetType, T: QPixelFormat_brightnessSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.brightnessSize_0(self);
    // return 1;
  }
}
pub trait QPixelFormat_brightnessSize_0<RetType> {
  fn brightnessSize_0(self , rsthis: & QPixelFormat) -> RetType;
}
impl<'a> /*trait*/ QPixelFormat_brightnessSize_0<u8> for () {
  fn brightnessSize_0(self , rsthis: & QPixelFormat) -> u8 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QPixelFormat14brightnessSizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u8 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpixelformat.h:200
// index:0
// Public inline Visibility=Default Availability=Available
// [1] uchar alphaSize() const

/*
Accessor function for the alpha channel size.
*/
impl /*struct*/ QPixelFormat {
  pub fn alphaSize_0<RetType, T: QPixelFormat_alphaSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.alphaSize_0(self);
    // return 1;
  }
}
pub trait QPixelFormat_alphaSize_0<RetType> {
  fn alphaSize_0(self , rsthis: & QPixelFormat) -> RetType;
}
impl<'a> /*trait*/ QPixelFormat_alphaSize_0<u8> for () {
  fn alphaSize_0(self , rsthis: & QPixelFormat) -> u8 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QPixelFormat9alphaSizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u8 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpixelformat.h:202
// index:0
// Public inline Visibility=Default Availability=Available
// [1] uchar bitsPerPixel() const

/*
Accessor function for the bits used per pixel. This function returns the sum of the color channels + the size of the alpha channel.
*/
impl /*struct*/ QPixelFormat {
  pub fn bitsPerPixel_0<RetType, T: QPixelFormat_bitsPerPixel_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.bitsPerPixel_0(self);
    // return 1;
  }
}
pub trait QPixelFormat_bitsPerPixel_0<RetType> {
  fn bitsPerPixel_0(self , rsthis: & QPixelFormat) -> RetType;
}
impl<'a> /*trait*/ QPixelFormat_bitsPerPixel_0<u8> for () {
  fn bitsPerPixel_0(self , rsthis: & QPixelFormat) -> u8 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QPixelFormat12bitsPerPixelEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u8 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpixelformat.h:209
// index:0
// Public inline Visibility=Default Availability=Available
// [4] QPixelFormat::AlphaUsage alphaUsage() const

/*
Accessor function for alphaUsage.
*/
impl /*struct*/ QPixelFormat {
  pub fn alphaUsage_0<RetType, T: QPixelFormat_alphaUsage_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.alphaUsage_0(self);
    // return 1;
  }
}
pub trait QPixelFormat_alphaUsage_0<RetType> {
  fn alphaUsage_0(self , rsthis: & QPixelFormat) -> RetType;
}
impl<'a> /*trait*/ QPixelFormat_alphaUsage_0<i32> for () {
  fn alphaUsage_0(self , rsthis: & QPixelFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QPixelFormat10alphaUsageEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpixelformat.h:210
// index:0
// Public inline Visibility=Default Availability=Available
// [4] QPixelFormat::AlphaPosition alphaPosition() const

/*
Accessor function for alphaPosition.
*/
impl /*struct*/ QPixelFormat {
  pub fn alphaPosition_0<RetType, T: QPixelFormat_alphaPosition_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.alphaPosition_0(self);
    // return 1;
  }
}
pub trait QPixelFormat_alphaPosition_0<RetType> {
  fn alphaPosition_0(self , rsthis: & QPixelFormat) -> RetType;
}
impl<'a> /*trait*/ QPixelFormat_alphaPosition_0<i32> for () {
  fn alphaPosition_0(self , rsthis: & QPixelFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QPixelFormat13alphaPositionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpixelformat.h:211
// index:0
// Public inline Visibility=Default Availability=Available
// [4] QPixelFormat::AlphaPremultiplied premultiplied() const

/*
Accessor function for the AlphaPremultiplied enum. This indicates if the alpha channel is multiplied in to the color channels.
*/
impl /*struct*/ QPixelFormat {
  pub fn premultiplied_0<RetType, T: QPixelFormat_premultiplied_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.premultiplied_0(self);
    // return 1;
  }
}
pub trait QPixelFormat_premultiplied_0<RetType> {
  fn premultiplied_0(self , rsthis: & QPixelFormat) -> RetType;
}
impl<'a> /*trait*/ QPixelFormat_premultiplied_0<i32> for () {
  fn premultiplied_0(self , rsthis: & QPixelFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QPixelFormat13premultipliedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpixelformat.h:212
// index:0
// Public inline Visibility=Default Availability=Available
// [4] QPixelFormat::TypeInterpretation typeInterpretation() const

/*
Accessor function for the type representation of a color channel or a pixel.

See also TypeInterpretation.
*/
impl /*struct*/ QPixelFormat {
  pub fn typeInterpretation_0<RetType, T: QPixelFormat_typeInterpretation_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.typeInterpretation_0(self);
    // return 1;
  }
}
pub trait QPixelFormat_typeInterpretation_0<RetType> {
  fn typeInterpretation_0(self , rsthis: & QPixelFormat) -> RetType;
}
impl<'a> /*trait*/ QPixelFormat_typeInterpretation_0<i32> for () {
  fn typeInterpretation_0(self , rsthis: & QPixelFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QPixelFormat18typeInterpretationEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpixelformat.h:213
// index:0
// Public inline Visibility=Default Availability=Available
// [4] QPixelFormat::ByteOrder byteOrder() const

/*
The byte order is almost always set the the byte order of the current system. However, it can be useful to describe some YUV formats. This function should never return QPixelFormat::CurrentSystemEndian as this value is translated to a endian value in the constructor.
*/
impl /*struct*/ QPixelFormat {
  pub fn byteOrder_0<RetType, T: QPixelFormat_byteOrder_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.byteOrder_0(self);
    // return 1;
  }
}
pub trait QPixelFormat_byteOrder_0<RetType> {
  fn byteOrder_0(self , rsthis: & QPixelFormat) -> RetType;
}
impl<'a> /*trait*/ QPixelFormat_byteOrder_0<i32> for () {
  fn byteOrder_0(self , rsthis: & QPixelFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QPixelFormat9byteOrderEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpixelformat.h:215
// index:0
// Public inline Visibility=Default Availability=Available
// [4] QPixelFormat::YUVLayout yuvLayout() const

/*
Accessor function for the YUVLayout. It is difficult to describe the color channels of a YUV pixel format since YUV color model uses macro pixels. Instead the layout of the pixels are stored as an enum.
*/
impl /*struct*/ QPixelFormat {
  pub fn yuvLayout_0<RetType, T: QPixelFormat_yuvLayout_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.yuvLayout_0(self);
    // return 1;
  }
}
pub trait QPixelFormat_yuvLayout_0<RetType> {
  fn yuvLayout_0(self , rsthis: & QPixelFormat) -> RetType;
}
impl<'a> /*trait*/ QPixelFormat_yuvLayout_0<i32> for () {
  fn yuvLayout_0(self , rsthis: & QPixelFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QPixelFormat9yuvLayoutEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpixelformat.h:216
// index:0
// Public inline Visibility=Default Availability=Available
// [1] uchar subEnum() const

/*

*/
impl /*struct*/ QPixelFormat {
  pub fn subEnum_0<RetType, T: QPixelFormat_subEnum_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.subEnum_0(self);
    // return 1;
  }
}
pub trait QPixelFormat_subEnum_0<RetType> {
  fn subEnum_0(self , rsthis: & QPixelFormat) -> RetType;
}
impl<'a> /*trait*/ QPixelFormat_subEnum_0<u8> for () {
  fn subEnum_0(self , rsthis: & QPixelFormat) -> u8 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QPixelFormat7subEnumEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u8 = Default::default(); return dret;
  }
}


pub fn DeleteQPixelFormat(this :*mut QPixelFormat) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN12QPixelFormatD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}

/*


*/
pub type QPixelFormat__FieldWidth = i32;
// 
pub const QPixelFormat__ModelFieldWidth :QPixelFormat__FieldWidth = 4;
// 
pub const QPixelFormat__FirstFieldWidth :QPixelFormat__FieldWidth = 6;
// 
pub const QPixelFormat__SecondFieldWidth :QPixelFormat__FieldWidth = 6;
// 
pub const QPixelFormat__ThirdFieldWidth :QPixelFormat__FieldWidth = 6;
// 
pub const QPixelFormat__FourthFieldWidth :QPixelFormat__FieldWidth = 6;
// 
pub const QPixelFormat__FifthFieldWidth :QPixelFormat__FieldWidth = 6;
// 
pub const QPixelFormat__AlphaFieldWidth :QPixelFormat__FieldWidth = 6;
// 
pub const QPixelFormat__AlphaUsageFieldWidth :QPixelFormat__FieldWidth = 1;
// 
pub const QPixelFormat__AlphaPositionFieldWidth :QPixelFormat__FieldWidth = 1;
// 
pub const QPixelFormat__PremulFieldWidth :QPixelFormat__FieldWidth = 1;
// 
pub const QPixelFormat__TypeInterpretationFieldWidth :QPixelFormat__FieldWidth = 4;
// 
pub const QPixelFormat__ByteOrderFieldWidth :QPixelFormat__FieldWidth = 2;
// 
pub const QPixelFormat__SubEnumFieldWidth :QPixelFormat__FieldWidth = 6;
// 
pub const QPixelFormat__UnusedFieldWidth :QPixelFormat__FieldWidth = 9;
// 
pub const QPixelFormat__TotalFieldWidthByWidths :QPixelFormat__FieldWidth = 64;
pub fn QPixelFormat_FieldWidthItemName(val: i32) ->String {
  match val {
     QPixelFormat__ModelFieldWidth => // 4
     {return String::from("ModelFieldWidth,TypeInterpretationFieldWidth");}
     QPixelFormat__FirstFieldWidth => // 6
     {return String::from("FirstFieldWidth,SecondFieldWidth,ThirdFieldWidth,FourthFieldWidth,FifthFieldWidth,AlphaFieldWidth,SubEnumFieldWidth");}
    // QPixelFormat__SecondFieldWidth => // 6
    // {return String::from("");}
    // QPixelFormat__ThirdFieldWidth => // 6
    // {return String::from("");}
    // QPixelFormat__FourthFieldWidth => // 6
    // {return String::from("");}
    // QPixelFormat__FifthFieldWidth => // 6
    // {return String::from("");}
    // QPixelFormat__AlphaFieldWidth => // 6
    // {return String::from("");}
     QPixelFormat__AlphaUsageFieldWidth => // 1
     {return String::from("AlphaUsageFieldWidth,AlphaPositionFieldWidth,PremulFieldWidth");}
    // QPixelFormat__AlphaPositionFieldWidth => // 1
    // {return String::from("");}
    // QPixelFormat__PremulFieldWidth => // 1
    // {return String::from("");}
    // QPixelFormat__TypeInterpretationFieldWidth => // 4
    // {return String::from("");}
     QPixelFormat__ByteOrderFieldWidth => // 2
     {return String::from("ByteOrderFieldWidth");}
    // QPixelFormat__SubEnumFieldWidth => // 6
    // {return String::from("");}
     QPixelFormat__UnusedFieldWidth => // 9
     {return String::from("UnusedFieldWidth");}
     QPixelFormat__TotalFieldWidthByWidths => // 64
     {return String::from("TotalFieldWidthByWidths");}
  _ => {return format!("{}", val);}
}
}
pub fn QPixelFormat_FieldWidthItemName_s(val: i32) ->String {
  //var nilthis *QPixelFormat
  //return nilthis.FieldWidthItemName(val);
  return QPixelFormat_FieldWidthItemName(val);
}


/*


*/
pub type QPixelFormat__Field = i32;
// 
pub const QPixelFormat__ModelField :QPixelFormat__Field = 0;
// 
pub const QPixelFormat__FirstField :QPixelFormat__Field = 4;
// 
pub const QPixelFormat__SecondField :QPixelFormat__Field = 10;
// 
pub const QPixelFormat__ThirdField :QPixelFormat__Field = 16;
// 
pub const QPixelFormat__FourthField :QPixelFormat__Field = 22;
// 
pub const QPixelFormat__FifthField :QPixelFormat__Field = 28;
// 
pub const QPixelFormat__AlphaField :QPixelFormat__Field = 34;
// 
pub const QPixelFormat__AlphaUsageField :QPixelFormat__Field = 40;
// 
pub const QPixelFormat__AlphaPositionField :QPixelFormat__Field = 41;
// 
pub const QPixelFormat__PremulField :QPixelFormat__Field = 42;
// 
pub const QPixelFormat__TypeInterpretationField :QPixelFormat__Field = 43;
// 
pub const QPixelFormat__ByteOrderField :QPixelFormat__Field = 47;
// 
pub const QPixelFormat__SubEnumField :QPixelFormat__Field = 49;
// 
pub const QPixelFormat__UnusedField :QPixelFormat__Field = 55;
// 
pub const QPixelFormat__TotalFieldWidthByOffsets :QPixelFormat__Field = 64;
pub fn QPixelFormat_FieldItemName(val: i32) ->String {
  match val {
     QPixelFormat__ModelField => // 0
     {return String::from("ModelField");}
     QPixelFormat__FirstField => // 4
     {return String::from("FirstField");}
     QPixelFormat__SecondField => // 10
     {return String::from("SecondField");}
     QPixelFormat__ThirdField => // 16
     {return String::from("ThirdField");}
     QPixelFormat__FourthField => // 22
     {return String::from("FourthField");}
     QPixelFormat__FifthField => // 28
     {return String::from("FifthField");}
     QPixelFormat__AlphaField => // 34
     {return String::from("AlphaField");}
     QPixelFormat__AlphaUsageField => // 40
     {return String::from("AlphaUsageField");}
     QPixelFormat__AlphaPositionField => // 41
     {return String::from("AlphaPositionField");}
     QPixelFormat__PremulField => // 42
     {return String::from("PremulField");}
     QPixelFormat__TypeInterpretationField => // 43
     {return String::from("TypeInterpretationField");}
     QPixelFormat__ByteOrderField => // 47
     {return String::from("ByteOrderField");}
     QPixelFormat__SubEnumField => // 49
     {return String::from("SubEnumField");}
     QPixelFormat__UnusedField => // 55
     {return String::from("UnusedField");}
     QPixelFormat__TotalFieldWidthByOffsets => // 64
     {return String::from("TotalFieldWidthByOffsets");}
  _ => {return format!("{}", val);}
}
}
pub fn QPixelFormat_FieldItemName_s(val: i32) ->String {
  //var nilthis *QPixelFormat
  //return nilthis.FieldItemName(val);
  return QPixelFormat_FieldItemName(val);
}


/*
This enum type is used to describe the color model of the pixelformat. Alpha was added in 5.5.


*/
pub type QPixelFormat__ColorModel = i32;
// The color model is RGB.
pub const QPixelFormat__RGB :QPixelFormat__ColorModel = 0;
// This is logically the opposite endian version of RGB. However, for ease of use it has its own model.
pub const QPixelFormat__BGR :QPixelFormat__ColorModel = 1;
// The color model uses a color palette.
pub const QPixelFormat__Indexed :QPixelFormat__ColorModel = 2;
// The color model is Grayscale.
pub const QPixelFormat__Grayscale :QPixelFormat__ColorModel = 3;
// The color model is CMYK.
pub const QPixelFormat__CMYK :QPixelFormat__ColorModel = 4;
// The color model is HSL.
pub const QPixelFormat__HSL :QPixelFormat__ColorModel = 5;
// The color model is HSV.
pub const QPixelFormat__HSV :QPixelFormat__ColorModel = 6;
// The color model is YUV.
pub const QPixelFormat__YUV :QPixelFormat__ColorModel = 7;
// There is no color model, only alpha is used.
pub const QPixelFormat__Alpha :QPixelFormat__ColorModel = 8;
pub fn QPixelFormat_ColorModelItemName(val: i32) ->String {
  match val {
     QPixelFormat__RGB => // 0
     {return String::from("RGB");}
     QPixelFormat__BGR => // 1
     {return String::from("BGR");}
     QPixelFormat__Indexed => // 2
     {return String::from("Indexed");}
     QPixelFormat__Grayscale => // 3
     {return String::from("Grayscale");}
     QPixelFormat__CMYK => // 4
     {return String::from("CMYK");}
     QPixelFormat__HSL => // 5
     {return String::from("HSL");}
     QPixelFormat__HSV => // 6
     {return String::from("HSV");}
     QPixelFormat__YUV => // 7
     {return String::from("YUV");}
     QPixelFormat__Alpha => // 8
     {return String::from("Alpha");}
  _ => {return format!("{}", val);}
}
}
pub fn QPixelFormat_ColorModelItemName_s(val: i32) ->String {
  //var nilthis *QPixelFormat
  //return nilthis.ColorModelItemName(val);
  return QPixelFormat_ColorModelItemName(val);
}


/*
This enum describes if the alpha channel is used or not. Sometimes the pixelformat will have a size for the alpha channel, but the pixel format does actually not use the alpha channel. For example RGB32 is such a format. The RGB channels are 8 bits each, and there is no alpha channel. But the complete size for each pixel is 32. Therefore the alpha channel size is 8, but the alpha channel is ignored. Its important to note that in such situations the position of the alpha channel is significant.


*/
pub type QPixelFormat__AlphaUsage = i32;
// The alpha channel is used.
pub const QPixelFormat__UsesAlpha :QPixelFormat__AlphaUsage = 0;
// The alpha channel is not used.
pub const QPixelFormat__IgnoresAlpha :QPixelFormat__AlphaUsage = 1;
pub fn QPixelFormat_AlphaUsageItemName(val: i32) ->String {
  match val {
     QPixelFormat__UsesAlpha => // 0
     {return String::from("UsesAlpha");}
     QPixelFormat__IgnoresAlpha => // 1
     {return String::from("IgnoresAlpha");}
  _ => {return format!("{}", val);}
}
}
pub fn QPixelFormat_AlphaUsageItemName_s(val: i32) ->String {
  //var nilthis *QPixelFormat
  //return nilthis.AlphaUsageItemName(val);
  return QPixelFormat_AlphaUsageItemName(val);
}


/*
This enum type is used to describe the alpha channels position relative to the color channels.


*/
pub type QPixelFormat__AlphaPosition = i32;
// The alpha channel will be put in front of the color channels . E.g. ARGB.
pub const QPixelFormat__AtBeginning :QPixelFormat__AlphaPosition = 0;
// The alpha channel will be put in the back of the color channels. E.g. RGBA.
pub const QPixelFormat__AtEnd :QPixelFormat__AlphaPosition = 1;
pub fn QPixelFormat_AlphaPositionItemName(val: i32) ->String {
  match val {
     QPixelFormat__AtBeginning => // 0
     {return String::from("AtBeginning");}
     QPixelFormat__AtEnd => // 1
     {return String::from("AtEnd");}
  _ => {return format!("{}", val);}
}
}
pub fn QPixelFormat_AlphaPositionItemName_s(val: i32) ->String {
  //var nilthis *QPixelFormat
  //return nilthis.AlphaPositionItemName(val);
  return QPixelFormat_AlphaPositionItemName(val);
}


/*
This enum type describes the boolean state if the alpha channel is multiplied into the color channels or not.


*/
pub type QPixelFormat__AlphaPremultiplied = i32;
// The alpha channel is not multiplied into the color channels.
pub const QPixelFormat__NotPremultiplied :QPixelFormat__AlphaPremultiplied = 0;
// The alpha channel is multiplied into the color channels.
pub const QPixelFormat__Premultiplied :QPixelFormat__AlphaPremultiplied = 1;
pub fn QPixelFormat_AlphaPremultipliedItemName(val: i32) ->String {
  match val {
     QPixelFormat__NotPremultiplied => // 0
     {return String::from("NotPremultiplied");}
     QPixelFormat__Premultiplied => // 1
     {return String::from("Premultiplied");}
  _ => {return format!("{}", val);}
}
}
pub fn QPixelFormat_AlphaPremultipliedItemName_s(val: i32) ->String {
  //var nilthis *QPixelFormat
  //return nilthis.AlphaPremultipliedItemName(val);
  return QPixelFormat_AlphaPremultipliedItemName(val);
}


/*
This enum describes how each pixel is interpreted. If a pixel is read as a full 32 bit unsigned integer and then each channel is masked out, or if each byte is read as unsigned char values. Typically QImage formats interpret one pixel as an unsigned integer and then the color channels are masked out. OpenGL on the other hand typically interpreted pixels "one byte after the other", Ie. unsigned byte.

QImage also have the format Format_RGBA8888 (and its derivatives), where the pixels are interpreted as unsigned bytes. OpenGL has extensions that makes it possible to upload pixel buffers in an unsigned integer format.



The image above shows a ARGB pixel in memory read as an unsigned integer. However, if this pixel was read byte for byte on a little endian system the first byte would be the byte containing the B-channel. The next byte would be the G-channel, then the R-channel and finally the A-channel. This shows that on little endian systems, how each pixel is interpreted is significant for integer formats. This is not the case on big endian systems.

ConstantValue
QPixelFormat::UnsignedInteger0
QPixelFormat::UnsignedShort1
QPixelFormat::UnsignedByte2
QPixelFormat::FloatingPoint3

*/
pub type QPixelFormat__TypeInterpretation = i32;
// 
pub const QPixelFormat__UnsignedInteger :QPixelFormat__TypeInterpretation = 0;
// 
pub const QPixelFormat__UnsignedShort :QPixelFormat__TypeInterpretation = 1;
// 
pub const QPixelFormat__UnsignedByte :QPixelFormat__TypeInterpretation = 2;
// 
pub const QPixelFormat__FloatingPoint :QPixelFormat__TypeInterpretation = 3;
pub fn QPixelFormat_TypeInterpretationItemName(val: i32) ->String {
  match val {
     QPixelFormat__UnsignedInteger => // 0
     {return String::from("UnsignedInteger");}
     QPixelFormat__UnsignedShort => // 1
     {return String::from("UnsignedShort");}
     QPixelFormat__UnsignedByte => // 2
     {return String::from("UnsignedByte");}
     QPixelFormat__FloatingPoint => // 3
     {return String::from("FloatingPoint");}
  _ => {return format!("{}", val);}
}
}
pub fn QPixelFormat_TypeInterpretationItemName_s(val: i32) ->String {
  //var nilthis *QPixelFormat
  //return nilthis.TypeInterpretationItemName(val);
  return QPixelFormat_TypeInterpretationItemName(val);
}


/*
YUV is not represented by describing the size of the color channels. This is because YUV often use macro pixels, making the concept of sperate color channels invalid. Instead the different YUV layouts are described with this enum.

ConstantValue
QPixelFormat::UYVY6
QPixelFormat::YUYV7

*/
pub type QPixelFormat__YUVLayout = i32;
// 
pub const QPixelFormat__YUV444 :QPixelFormat__YUVLayout = 0;
// 
pub const QPixelFormat__YUV422 :QPixelFormat__YUVLayout = 1;
// 
pub const QPixelFormat__YUV411 :QPixelFormat__YUVLayout = 2;
// 
pub const QPixelFormat__YUV420P :QPixelFormat__YUVLayout = 3;
// 
pub const QPixelFormat__YUV420SP :QPixelFormat__YUVLayout = 4;
// 
pub const QPixelFormat__YV12 :QPixelFormat__YUVLayout = 5;
// 
pub const QPixelFormat__UYVY :QPixelFormat__YUVLayout = 6;
// 
pub const QPixelFormat__YUYV :QPixelFormat__YUVLayout = 7;
// 
pub const QPixelFormat__NV12 :QPixelFormat__YUVLayout = 8;
// 
pub const QPixelFormat__NV21 :QPixelFormat__YUVLayout = 9;
// 0
pub const QPixelFormat__IMC1 :QPixelFormat__YUVLayout = 10;
// 1
pub const QPixelFormat__IMC2 :QPixelFormat__YUVLayout = 11;
// 2
pub const QPixelFormat__IMC3 :QPixelFormat__YUVLayout = 12;
// 3
pub const QPixelFormat__IMC4 :QPixelFormat__YUVLayout = 13;
// 4
pub const QPixelFormat__Y8 :QPixelFormat__YUVLayout = 14;
// 5
pub const QPixelFormat__Y16 :QPixelFormat__YUVLayout = 15;
pub fn QPixelFormat_YUVLayoutItemName(val: i32) ->String {
  match val {
     QPixelFormat__YUV444 => // 0
     {return String::from("YUV444");}
     QPixelFormat__YUV422 => // 1
     {return String::from("YUV422");}
     QPixelFormat__YUV411 => // 2
     {return String::from("YUV411");}
     QPixelFormat__YUV420P => // 3
     {return String::from("YUV420P");}
     QPixelFormat__YUV420SP => // 4
     {return String::from("YUV420SP");}
     QPixelFormat__YV12 => // 5
     {return String::from("YV12");}
     QPixelFormat__UYVY => // 6
     {return String::from("UYVY");}
     QPixelFormat__YUYV => // 7
     {return String::from("YUYV");}
     QPixelFormat__NV12 => // 8
     {return String::from("NV12");}
     QPixelFormat__NV21 => // 9
     {return String::from("NV21");}
     QPixelFormat__IMC1 => // 10
     {return String::from("IMC1");}
     QPixelFormat__IMC2 => // 11
     {return String::from("IMC2");}
     QPixelFormat__IMC3 => // 12
     {return String::from("IMC3");}
     QPixelFormat__IMC4 => // 13
     {return String::from("IMC4");}
     QPixelFormat__Y8 => // 14
     {return String::from("Y8");}
     QPixelFormat__Y16 => // 15
     {return String::from("Y16");}
  _ => {return format!("{}", val);}
}
}
pub fn QPixelFormat_YUVLayoutItemName_s(val: i32) ->String {
  //var nilthis *QPixelFormat
  //return nilthis.YUVLayoutItemName(val);
  return QPixelFormat_YUVLayoutItemName(val);
}


/*
This enum describes the ByteOrder of the pixel format. This enum is mostly ignored but have some use cases for YUV formats. BGR formats have their own color model, and should not be described by using the opposite endianness on an RGB format.


*/
pub type QPixelFormat__ByteOrder = i32;
// The byte order is little endian.
pub const QPixelFormat__LittleEndian :QPixelFormat__ByteOrder = 0;
// The byte order is big endian.
pub const QPixelFormat__BigEndian :QPixelFormat__ByteOrder = 1;
// This enum will not be stored, but is converted in the constructor to the endian enum that matches the enum of the current system.
pub const QPixelFormat__CurrentSystemEndian :QPixelFormat__ByteOrder = 2;
pub fn QPixelFormat_ByteOrderItemName(val: i32) ->String {
  match val {
     QPixelFormat__LittleEndian => // 0
     {return String::from("LittleEndian");}
     QPixelFormat__BigEndian => // 1
     {return String::from("BigEndian");}
     QPixelFormat__CurrentSystemEndian => // 2
     {return String::from("CurrentSystemEndian");}
  _ => {return format!("{}", val);}
}
}
pub fn QPixelFormat_ByteOrderItemName_s(val: i32) ->String {
  //var nilthis *QPixelFormat
  //return nilthis.ByteOrderItemName(val);
  return QPixelFormat_ByteOrderItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
