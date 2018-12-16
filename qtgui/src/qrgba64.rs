

// mod ::gui::QRgba64
// package qtgui
// /usr/include/qt/QtGui/qrgba64.h
// #include <qrgba64.h>
// #include <QtGui>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 39
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
#[derive(Default)] // class sizeof(QRgba64)=8
pub struct QRgba64 {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QRgba64_ITF interface {
//    QRgba64_PTR() *QRgba64
//}
//func (ptr *QRgba64) QRgba64_PTR() *QRgba64 { return ptr }

impl /*struct*/ QRgba64 {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QRgba64 {
    return QRgba64{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QRgba64 {
//  type Target = QRgba64BASE;
//
//  fn deref(&self) -> &QRgba64BASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QRgba64BASE> for QRgba64 {
//  fn as_ref(& self) -> & QRgba64BASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qrgba64.h:69
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void QRgba64()

/*
Default constructs an instance of QRgba64.
*/
// QRgba64() ctx.fn_proto_cpp
impl /*struct*/ QRgba64 {
  pub fn QRgba64_0<T: QRgba64_QRgba64_0>(value: T) -> QRgba64 {
    let rsthis = value.QRgba64_0();
    return rsthis;
    // return 1;
  }
}

pub trait QRgba64_QRgba64_0 {
  fn QRgba64_0(self) -> QRgba64;
}
// QRgba64() ctx.fn_proto_cpp
impl<'a> /*trait*/ QRgba64_QRgba64_0 for () {
  fn QRgba64_0(self) -> QRgba64 {
    // unsafe{_ZN7QRgba64C2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN7QRgba64C2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QRgba64{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qrgba64.h:72
// index:0
// Public static inline Visibility=Default Availability=Available
// [8] QRgba64 fromRgba64(quint64)

/*
Returns c as a QRgba64 struct.

See also fromArgb32().
*/
impl /*struct*/ QRgba64 {
  pub fn fromRgba64_0<RetType, T: QRgba64_fromRgba64_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromRgba64_0();
    // return 1;
  }
}
pub trait QRgba64_fromRgba64_0<RetType> {
  fn fromRgba64_0(self ) -> RetType;
}
impl<'a> /*trait*/ QRgba64_fromRgba64_0<usize> for (u64) {
  fn fromRgba64_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const u64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QRgba6410fromRgba64Ey", 1,qtrt::FFITY_UINT64,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qrgba64.h:77
// index:1
// Public static inline Visibility=Default Availability=Available
// [8] QRgba64 fromRgba64(quint16, quint16, quint16, quint16)

/*
Returns c as a QRgba64 struct.

See also fromArgb32().
*/
impl /*struct*/ QRgba64 {
  pub fn fromRgba64_1<RetType, T: QRgba64_fromRgba64_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromRgba64_1();
    // return 1;
  }
}
pub trait QRgba64_fromRgba64_1<RetType> {
  fn fromRgba64_1(self ) -> RetType;
}
impl<'a> /*trait*/ QRgba64_fromRgba64_1<usize> for (u16,u16,u16,u16) {
  fn fromRgba64_1(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const u16 as usize;
    let arg1 = (&self.1) as *const u16 as usize;
    let arg2 = (&self.2) as *const u16 as usize;
    let arg3 = (&self.3) as *const u16 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QRgba6410fromRgba64Etttt", 4,qtrt::FFITY_UINT16,qtrt::FFITY_UINT16,qtrt::FFITY_UINT16,qtrt::FFITY_UINT16,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qrgba64.h:84
// index:0
// Public static inline Visibility=Default Availability=Available
// [8] QRgba64 fromRgba(quint8, quint8, quint8, quint8)

/*
Constructs a QRgba64 value from the four 8-bit color channels red, green, blue and alpha.

See also fromArgb32().
*/
impl /*struct*/ QRgba64 {
  pub fn fromRgba_0<RetType, T: QRgba64_fromRgba_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromRgba_0();
    // return 1;
  }
}
pub trait QRgba64_fromRgba_0<RetType> {
  fn fromRgba_0(self ) -> RetType;
}
impl<'a> /*trait*/ QRgba64_fromRgba_0<usize> for (u8,u8,u8,u8) {
  fn fromRgba_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const u8 as usize;
    let arg1 = (&self.1) as *const u8 as usize;
    let arg2 = (&self.2) as *const u8 as usize;
    let arg3 = (&self.3) as *const u8 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QRgba648fromRgbaEhhhh", 4,qtrt::FFITY_UINT8,qtrt::FFITY_UINT8,qtrt::FFITY_UINT8,qtrt::FFITY_UINT8,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qrgba64.h:92
// index:0
// Public static inline Visibility=Default Availability=Available
// [8] QRgba64 fromArgb32(uint)

/*
Constructs a QRgba64 value from the 32bit ARGB value rgb.

See also fromRgba().
*/
impl /*struct*/ QRgba64 {
  pub fn fromArgb32_0<RetType, T: QRgba64_fromArgb32_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromArgb32_0();
    // return 1;
  }
}
pub trait QRgba64_fromArgb32_0<RetType> {
  fn fromArgb32_0(self ) -> RetType;
}
impl<'a> /*trait*/ QRgba64_fromArgb32_0<usize> for (u32) {
  fn fromArgb32_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const u32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QRgba6410fromArgb32Ej", 1,qtrt::FFITY_UINT32,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qrgba64.h:97
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isOpaque() const

/*
Returns whether the color is fully opaque.

See also isTransparent() and alpha().
*/
impl /*struct*/ QRgba64 {
  pub fn isOpaque_0<RetType, T: QRgba64_isOpaque_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isOpaque_0(self);
    // return 1;
  }
}
pub trait QRgba64_isOpaque_0<RetType> {
  fn isOpaque_0(self , rsthis: & QRgba64) -> RetType;
}
impl<'a> /*trait*/ QRgba64_isOpaque_0<bool> for () {
  fn isOpaque_0(self , rsthis: & QRgba64) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QRgba648isOpaqueEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qrgba64.h:101
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isTransparent() const

/*
Returns whether the color is transparent.

See also isOpaque() and alpha().
*/
impl /*struct*/ QRgba64 {
  pub fn isTransparent_0<RetType, T: QRgba64_isTransparent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isTransparent_0(self);
    // return 1;
  }
}
pub trait QRgba64_isTransparent_0<RetType> {
  fn isTransparent_0(self , rsthis: & QRgba64) -> RetType;
}
impl<'a> /*trait*/ QRgba64_isTransparent_0<bool> for () {
  fn isTransparent_0(self , rsthis: & QRgba64) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QRgba6413isTransparentEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qrgba64.h:106
// index:0
// Public inline Visibility=Default Availability=Available
// [2] quint16 red() const

/*
Returns the 16-bit red color component.

See also setRed().
*/
impl /*struct*/ QRgba64 {
  pub fn red_0<RetType, T: QRgba64_red_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.red_0(self);
    // return 1;
  }
}
pub trait QRgba64_red_0<RetType> {
  fn red_0(self , rsthis: & QRgba64) -> RetType;
}
impl<'a> /*trait*/ QRgba64_red_0<u16> for () {
  fn red_0(self , rsthis: & QRgba64) -> u16 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QRgba643redEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u16 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qrgba64.h:107
// index:0
// Public inline Visibility=Default Availability=Available
// [2] quint16 green() const

/*
Returns the 16-bit green color component.

See also setGreen().
*/
impl /*struct*/ QRgba64 {
  pub fn green_0<RetType, T: QRgba64_green_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.green_0(self);
    // return 1;
  }
}
pub trait QRgba64_green_0<RetType> {
  fn green_0(self , rsthis: & QRgba64) -> RetType;
}
impl<'a> /*trait*/ QRgba64_green_0<u16> for () {
  fn green_0(self , rsthis: & QRgba64) -> u16 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QRgba645greenEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u16 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qrgba64.h:108
// index:0
// Public inline Visibility=Default Availability=Available
// [2] quint16 blue() const

/*
Returns the 16-bit blue color component.

See also setBlue().
*/
impl /*struct*/ QRgba64 {
  pub fn blue_0<RetType, T: QRgba64_blue_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.blue_0(self);
    // return 1;
  }
}
pub trait QRgba64_blue_0<RetType> {
  fn blue_0(self , rsthis: & QRgba64) -> RetType;
}
impl<'a> /*trait*/ QRgba64_blue_0<u16> for () {
  fn blue_0(self , rsthis: & QRgba64) -> u16 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QRgba644blueEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u16 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qrgba64.h:109
// index:0
// Public inline Visibility=Default Availability=Available
// [2] quint16 alpha() const

/*
Returns the 16-bit alpha channel.

See also setAlpha().
*/
impl /*struct*/ QRgba64 {
  pub fn alpha_0<RetType, T: QRgba64_alpha_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.alpha_0(self);
    // return 1;
  }
}
pub trait QRgba64_alpha_0<RetType> {
  fn alpha_0(self , rsthis: & QRgba64) -> RetType;
}
impl<'a> /*trait*/ QRgba64_alpha_0<u16> for () {
  fn alpha_0(self , rsthis: & QRgba64) -> u16 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QRgba645alphaEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u16 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qrgba64.h:110
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setRed(quint16)

/*
Sets the red color component of this color to red.

See also red().
*/
impl /*struct*/ QRgba64 {
  pub fn setRed_0<RetType, T: QRgba64_setRed_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setRed_0(self);
    // return 1;
  }
}
pub trait QRgba64_setRed_0<RetType> {
  fn setRed_0(self , rsthis: & QRgba64) -> RetType;
}
impl<'a> /*trait*/ QRgba64_setRed_0<(/*void*/)> for (u16) {
  fn setRed_0(self , rsthis: & QRgba64) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const u16 as usize;
     qtrt::InvokeQtFunc6("_ZN7QRgba646setRedEt", 1,qtrt::FFITY_UINT16,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qrgba64.h:111
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setGreen(quint16)

/*
Sets the green color component of this color to green.

See also green().
*/
impl /*struct*/ QRgba64 {
  pub fn setGreen_0<RetType, T: QRgba64_setGreen_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setGreen_0(self);
    // return 1;
  }
}
pub trait QRgba64_setGreen_0<RetType> {
  fn setGreen_0(self , rsthis: & QRgba64) -> RetType;
}
impl<'a> /*trait*/ QRgba64_setGreen_0<(/*void*/)> for (u16) {
  fn setGreen_0(self , rsthis: & QRgba64) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const u16 as usize;
     qtrt::InvokeQtFunc6("_ZN7QRgba648setGreenEt", 1,qtrt::FFITY_UINT16,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qrgba64.h:112
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setBlue(quint16)

/*
Sets the blue color component of this color to blue.

See also blue().
*/
impl /*struct*/ QRgba64 {
  pub fn setBlue_0<RetType, T: QRgba64_setBlue_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setBlue_0(self);
    // return 1;
  }
}
pub trait QRgba64_setBlue_0<RetType> {
  fn setBlue_0(self , rsthis: & QRgba64) -> RetType;
}
impl<'a> /*trait*/ QRgba64_setBlue_0<(/*void*/)> for (u16) {
  fn setBlue_0(self , rsthis: & QRgba64) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const u16 as usize;
     qtrt::InvokeQtFunc6("_ZN7QRgba647setBlueEt", 1,qtrt::FFITY_UINT16,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qrgba64.h:113
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setAlpha(quint16)

/*
Sets the alpha of this color to alpha.

See also alpha().
*/
impl /*struct*/ QRgba64 {
  pub fn setAlpha_0<RetType, T: QRgba64_setAlpha_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setAlpha_0(self);
    // return 1;
  }
}
pub trait QRgba64_setAlpha_0<RetType> {
  fn setAlpha_0(self , rsthis: & QRgba64) -> RetType;
}
impl<'a> /*trait*/ QRgba64_setAlpha_0<(/*void*/)> for (u16) {
  fn setAlpha_0(self , rsthis: & QRgba64) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const u16 as usize;
     qtrt::InvokeQtFunc6("_ZN7QRgba648setAlphaEt", 1,qtrt::FFITY_UINT16,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qrgba64.h:115
// index:0
// Public inline Visibility=Default Availability=Available
// [1] quint8 red8() const

/*
Returns the red color component as an 8-bit.
*/
impl /*struct*/ QRgba64 {
  pub fn red8_0<RetType, T: QRgba64_red8_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.red8_0(self);
    // return 1;
  }
}
pub trait QRgba64_red8_0<RetType> {
  fn red8_0(self , rsthis: & QRgba64) -> RetType;
}
impl<'a> /*trait*/ QRgba64_red8_0<u8> for () {
  fn red8_0(self , rsthis: & QRgba64) -> u8 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QRgba644red8Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u8 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qrgba64.h:116
// index:0
// Public inline Visibility=Default Availability=Available
// [1] quint8 green8() const

/*
Returns the green color component as an 8-bit.
*/
impl /*struct*/ QRgba64 {
  pub fn green8_0<RetType, T: QRgba64_green8_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.green8_0(self);
    // return 1;
  }
}
pub trait QRgba64_green8_0<RetType> {
  fn green8_0(self , rsthis: & QRgba64) -> RetType;
}
impl<'a> /*trait*/ QRgba64_green8_0<u8> for () {
  fn green8_0(self , rsthis: & QRgba64) -> u8 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QRgba646green8Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u8 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qrgba64.h:117
// index:0
// Public inline Visibility=Default Availability=Available
// [1] quint8 blue8() const

/*
Returns the blue color component as an 8-bit.
*/
impl /*struct*/ QRgba64 {
  pub fn blue8_0<RetType, T: QRgba64_blue8_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.blue8_0(self);
    // return 1;
  }
}
pub trait QRgba64_blue8_0<RetType> {
  fn blue8_0(self , rsthis: & QRgba64) -> RetType;
}
impl<'a> /*trait*/ QRgba64_blue8_0<u8> for () {
  fn blue8_0(self , rsthis: & QRgba64) -> u8 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QRgba645blue8Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u8 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qrgba64.h:118
// index:0
// Public inline Visibility=Default Availability=Available
// [1] quint8 alpha8() const

/*
Returns the alpha channel as an 8-bit.
*/
impl /*struct*/ QRgba64 {
  pub fn alpha8_0<RetType, T: QRgba64_alpha8_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.alpha8_0(self);
    // return 1;
  }
}
pub trait QRgba64_alpha8_0<RetType> {
  fn alpha8_0(self , rsthis: & QRgba64) -> RetType;
}
impl<'a> /*trait*/ QRgba64_alpha8_0<u8> for () {
  fn alpha8_0(self , rsthis: & QRgba64) -> u8 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QRgba646alpha8Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u8 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qrgba64.h:119
// index:0
// Public inline Visibility=Default Availability=Available
// [4] uint toArgb32() const

/*
Returns the color as a 32-bit ARGB value.

See also fromArgb32().
*/
impl /*struct*/ QRgba64 {
  pub fn toArgb32_0<RetType, T: QRgba64_toArgb32_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toArgb32_0(self);
    // return 1;
  }
}
pub trait QRgba64_toArgb32_0<RetType> {
  fn toArgb32_0(self , rsthis: & QRgba64) -> RetType;
}
impl<'a> /*trait*/ QRgba64_toArgb32_0<u32> for () {
  fn toArgb32_0(self , rsthis: & QRgba64) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QRgba648toArgb32Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qrgba64.h:123
// index:0
// Public inline Visibility=Default Availability=Available
// [2] ushort toRgb16() const

/*
Returns the color as a 16-bit RGB value.

See also toArgb32().
*/
impl /*struct*/ QRgba64 {
  pub fn toRgb16_0<RetType, T: QRgba64_toRgb16_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toRgb16_0(self);
    // return 1;
  }
}
pub trait QRgba64_toRgb16_0<RetType> {
  fn toRgb16_0(self , rsthis: & QRgba64) -> RetType;
}
impl<'a> /*trait*/ QRgba64_toRgb16_0<u16> for () {
  fn toRgb16_0(self , rsthis: & QRgba64) -> u16 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QRgba647toRgb16Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u16 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qrgba64.h:128
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QRgba64 premultiplied() const

/*
Returns the color with the alpha premultiplied.

See also unpremultiplied().
*/
impl /*struct*/ QRgba64 {
  pub fn premultiplied_0<RetType, T: QRgba64_premultiplied_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.premultiplied_0(self);
    // return 1;
  }
}
pub trait QRgba64_premultiplied_0<RetType> {
  fn premultiplied_0(self , rsthis: & QRgba64) -> RetType;
}
impl<'a> /*trait*/ QRgba64_premultiplied_0<usize> for () {
  fn premultiplied_0(self , rsthis: & QRgba64) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QRgba6413premultipliedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qrgba64.h:137
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QRgba64 unpremultiplied() const

/*
Returns the color with the alpha unpremultiplied.

See also premultiplied().
*/
impl /*struct*/ QRgba64 {
  pub fn unpremultiplied_0<RetType, T: QRgba64_unpremultiplied_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.unpremultiplied_0(self);
    // return 1;
  }
}
pub trait QRgba64_unpremultiplied_0<RetType> {
  fn unpremultiplied_0(self , rsthis: & QRgba64) -> RetType;
}
impl<'a> /*trait*/ QRgba64_unpremultiplied_0<usize> for () {
  fn unpremultiplied_0(self , rsthis: & QRgba64) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QRgba6415unpremultipliedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qrgba64.h:151
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QRgba64 operator=(quint64)

/*

*/
impl /*struct*/ QRgba64 {
  pub fn operator_equal_0<RetType, T: QRgba64_operator_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_0(self);
    // return 1;
  }
}
pub trait QRgba64_operator_equal_0<RetType> {
  fn operator_equal_0(self , rsthis: & QRgba64) -> RetType;
}
impl<'a> /*trait*/ QRgba64_operator_equal_0<usize> for (u64) {
  fn operator_equal_0(self , rsthis: & QRgba64) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const u64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QRgba64aSEy", 1,qtrt::FFITY_UINT64,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}


pub fn DeleteQRgba64(this :*mut QRgba64) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN7QRgba64D2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}

/*


*/
pub type QRgba64__Shifts = i32;
// 
pub const QRgba64__RedShift :QRgba64__Shifts = 0;
// 
pub const QRgba64__GreenShift :QRgba64__Shifts = 16;
// 
pub const QRgba64__BlueShift :QRgba64__Shifts = 32;
// 
pub const QRgba64__AlphaShift :QRgba64__Shifts = 48;
pub fn QRgba64_ShiftsItemName(val: i32) ->String {
  match val {
     QRgba64__RedShift => // 0
     {return String::from("RedShift");}
     QRgba64__GreenShift => // 16
     {return String::from("GreenShift");}
     QRgba64__BlueShift => // 32
     {return String::from("BlueShift");}
     QRgba64__AlphaShift => // 48
     {return String::from("AlphaShift");}
  _ => {return format!("{}", val);}
}
}
pub fn QRgba64_ShiftsItemName_s(val: i32) ->String {
  //var nilthis *QRgba64
  //return nilthis.ShiftsItemName(val);
  return QRgba64_ShiftsItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
