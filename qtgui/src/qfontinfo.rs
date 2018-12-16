

// mod ::gui::QFontInfo
// package qtgui
// /usr/include/qt/QtGui/qfontinfo.h
// #include <qfontinfo.h>
// #include <QtGui>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 21
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
#[derive(Default)] // class sizeof(QFontInfo)=8
pub struct QFontInfo {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QFontInfo_ITF interface {
//    QFontInfo_PTR() *QFontInfo
//}
//func (ptr *QFontInfo) QFontInfo_PTR() *QFontInfo { return ptr }

impl /*struct*/ QFontInfo {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QFontInfo {
    return QFontInfo{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QFontInfo {
//  type Target = QFontInfoBASE;
//
//  fn deref(&self) -> &QFontInfoBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QFontInfoBASE> for QFontInfo {
//  fn as_ref(& self) -> & QFontInfoBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qfontinfo.h:53
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QFontInfo(const QFont &)

/*
Constructs a font info object for font.

The font must be screen-compatible, i.e. a font you use when drawing text in widgets or pixmaps, not QPicture or QPrinter.

The font info object holds the information for the font that is passed in the constructor at the time it is created, and is not updated if the font's attributes are changed later.

Use QPainter::fontInfo() to get the font info when painting. This will give correct results also when painting on paint device that is not screen-compatible.
*/
// QFontInfo(const QFont &) ctx.fn_proto_cpp
impl /*struct*/ QFontInfo {
  pub fn QFontInfo_0<T: QFontInfo_QFontInfo_0>(value: T) -> QFontInfo {
    let rsthis = value.QFontInfo_0();
    return rsthis;
    // return 1;
  }
}

pub trait QFontInfo_QFontInfo_0 {
  fn QFontInfo_0(self) -> QFontInfo;
}
// QFontInfo(const QFont &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QFontInfo_QFontInfo_0 for (usize) {
  fn QFontInfo_0(self) -> QFontInfo {
    // unsafe{_ZN9QFontInfoC2ERK5QFont()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QFontInfoC2ERK5QFont", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QFontInfo{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qfontinfo.h:55
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ~QFontInfo()

/*

*/
pub fn DeleteQFontInfo(this :*mut QFontInfo) {
    // let rv = qtrt::InvokeQtFunc6("_ZN9QFontInfoD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 8)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qfontinfo.h:57
// index:0
// Public Visibility=Default Availability=Available
// [8] QFontInfo & operator=(const QFontInfo &)

/*

*/
impl /*struct*/ QFontInfo {
  pub fn operator_equal_0<RetType, T: QFontInfo_operator_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_0(self);
    // return 1;
  }
}
pub trait QFontInfo_operator_equal_0<RetType> {
  fn operator_equal_0(self , rsthis: & QFontInfo) -> RetType;
}
impl<'a> /*trait*/ QFontInfo_operator_equal_0<usize> for (usize) {
  fn operator_equal_0(self , rsthis: & QFontInfo) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QFontInfoaSERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfontinfo.h:59
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void swap(QFontInfo &)

/*
Swaps this font info instance with other. This function is very fast and never fails.

This function was introduced in  Qt 5.0.
*/
impl /*struct*/ QFontInfo {
  pub fn swap_0<RetType, T: QFontInfo_swap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.swap_0(self);
    // return 1;
  }
}
pub trait QFontInfo_swap_0<RetType> {
  fn swap_0(self , rsthis: & QFontInfo) -> RetType;
}
impl<'a> /*trait*/ QFontInfo_swap_0<(/*void*/)> for (usize) {
  fn swap_0(self , rsthis: & QFontInfo) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QFontInfo4swapERS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qfontinfo.h:61
// index:0
// Public Visibility=Default Availability=Available
// [8] QString family() const

/*
Returns the family name of the matched window system font.

See also QFont::family().
*/
impl /*struct*/ QFontInfo {
  pub fn family_0<RetType, T: QFontInfo_family_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.family_0(self);
    // return 1;
  }
}
pub trait QFontInfo_family_0<RetType> {
  fn family_0(self , rsthis: & QFontInfo) -> RetType;
}
impl<'a> /*trait*/ QFontInfo_family_0<usize> for () {
  fn family_0(self , rsthis: & QFontInfo) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QFontInfo6familyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfontinfo.h:62
// index:0
// Public Visibility=Default Availability=Available
// [8] QString styleName() const

/*
Returns the style name of the matched window system font on systems that support it.

This function was introduced in  Qt 4.8.

See also QFont::styleName().
*/
impl /*struct*/ QFontInfo {
  pub fn styleName_0<RetType, T: QFontInfo_styleName_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.styleName_0(self);
    // return 1;
  }
}
pub trait QFontInfo_styleName_0<RetType> {
  fn styleName_0(self , rsthis: & QFontInfo) -> RetType;
}
impl<'a> /*trait*/ QFontInfo_styleName_0<usize> for () {
  fn styleName_0(self , rsthis: & QFontInfo) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QFontInfo9styleNameEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfontinfo.h:63
// index:0
// Public Visibility=Default Availability=Available
// [4] int pixelSize() const

/*
Returns the pixel size of the matched window system font.

See also QFont::pointSize().
*/
impl /*struct*/ QFontInfo {
  pub fn pixelSize_0<RetType, T: QFontInfo_pixelSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.pixelSize_0(self);
    // return 1;
  }
}
pub trait QFontInfo_pixelSize_0<RetType> {
  fn pixelSize_0(self , rsthis: & QFontInfo) -> RetType;
}
impl<'a> /*trait*/ QFontInfo_pixelSize_0<i32> for () {
  fn pixelSize_0(self , rsthis: & QFontInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QFontInfo9pixelSizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfontinfo.h:64
// index:0
// Public Visibility=Default Availability=Available
// [4] int pointSize() const

/*
Returns the point size of the matched window system font.

See also pointSizeF() and QFont::pointSize().
*/
impl /*struct*/ QFontInfo {
  pub fn pointSize_0<RetType, T: QFontInfo_pointSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.pointSize_0(self);
    // return 1;
  }
}
pub trait QFontInfo_pointSize_0<RetType> {
  fn pointSize_0(self , rsthis: & QFontInfo) -> RetType;
}
impl<'a> /*trait*/ QFontInfo_pointSize_0<i32> for () {
  fn pointSize_0(self , rsthis: & QFontInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QFontInfo9pointSizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfontinfo.h:65
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal pointSizeF() const

/*
Returns the point size of the matched window system font.

See also QFont::pointSizeF().
*/
impl /*struct*/ QFontInfo {
  pub fn pointSizeF_0<RetType, T: QFontInfo_pointSizeF_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.pointSizeF_0(self);
    // return 1;
  }
}
pub trait QFontInfo_pointSizeF_0<RetType> {
  fn pointSizeF_0(self , rsthis: & QFontInfo) -> RetType;
}
impl<'a> /*trait*/ QFontInfo_pointSizeF_0<f64> for () {
  fn pointSizeF_0(self , rsthis: & QFontInfo) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QFontInfo10pointSizeFEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfontinfo.h:66
// index:0
// Public Visibility=Default Availability=Available
// [1] bool italic() const

/*
Returns the italic value of the matched window system font.

See also QFont::italic().
*/
impl /*struct*/ QFontInfo {
  pub fn italic_0<RetType, T: QFontInfo_italic_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.italic_0(self);
    // return 1;
  }
}
pub trait QFontInfo_italic_0<RetType> {
  fn italic_0(self , rsthis: & QFontInfo) -> RetType;
}
impl<'a> /*trait*/ QFontInfo_italic_0<bool> for () {
  fn italic_0(self , rsthis: & QFontInfo) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QFontInfo6italicEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfontinfo.h:67
// index:0
// Public Visibility=Default Availability=Available
// [4] QFont::Style style() const

/*
Returns the style value of the matched window system font.

See also QFont::style().
*/
impl /*struct*/ QFontInfo {
  pub fn style_0<RetType, T: QFontInfo_style_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.style_0(self);
    // return 1;
  }
}
pub trait QFontInfo_style_0<RetType> {
  fn style_0(self , rsthis: & QFontInfo) -> RetType;
}
impl<'a> /*trait*/ QFontInfo_style_0<i32> for () {
  fn style_0(self , rsthis: & QFontInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QFontInfo5styleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfontinfo.h:68
// index:0
// Public Visibility=Default Availability=Available
// [4] int weight() const

/*
Returns the weight of the matched window system font.

See also QFont::weight() and bold().
*/
impl /*struct*/ QFontInfo {
  pub fn weight_0<RetType, T: QFontInfo_weight_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.weight_0(self);
    // return 1;
  }
}
pub trait QFontInfo_weight_0<RetType> {
  fn weight_0(self , rsthis: & QFontInfo) -> RetType;
}
impl<'a> /*trait*/ QFontInfo_weight_0<i32> for () {
  fn weight_0(self , rsthis: & QFontInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QFontInfo6weightEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfontinfo.h:69
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool bold() const

/*
Returns true if weight() would return a value greater than QFont::Normal; otherwise returns false.

See also weight() and QFont::bold().
*/
impl /*struct*/ QFontInfo {
  pub fn bold_0<RetType, T: QFontInfo_bold_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.bold_0(self);
    // return 1;
  }
}
pub trait QFontInfo_bold_0<RetType> {
  fn bold_0(self , rsthis: & QFontInfo) -> RetType;
}
impl<'a> /*trait*/ QFontInfo_bold_0<bool> for () {
  fn bold_0(self , rsthis: & QFontInfo) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QFontInfo4boldEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfontinfo.h:70
// index:0
// Public Visibility=Default Availability=Available
// [1] bool underline() const

/*

*/
impl /*struct*/ QFontInfo {
  pub fn underline_0<RetType, T: QFontInfo_underline_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.underline_0(self);
    // return 1;
  }
}
pub trait QFontInfo_underline_0<RetType> {
  fn underline_0(self , rsthis: & QFontInfo) -> RetType;
}
impl<'a> /*trait*/ QFontInfo_underline_0<bool> for () {
  fn underline_0(self , rsthis: & QFontInfo) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QFontInfo9underlineEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfontinfo.h:71
// index:0
// Public Visibility=Default Availability=Available
// [1] bool overline() const

/*

*/
impl /*struct*/ QFontInfo {
  pub fn overline_0<RetType, T: QFontInfo_overline_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.overline_0(self);
    // return 1;
  }
}
pub trait QFontInfo_overline_0<RetType> {
  fn overline_0(self , rsthis: & QFontInfo) -> RetType;
}
impl<'a> /*trait*/ QFontInfo_overline_0<bool> for () {
  fn overline_0(self , rsthis: & QFontInfo) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QFontInfo8overlineEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfontinfo.h:72
// index:0
// Public Visibility=Default Availability=Available
// [1] bool strikeOut() const

/*

*/
impl /*struct*/ QFontInfo {
  pub fn strikeOut_0<RetType, T: QFontInfo_strikeOut_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.strikeOut_0(self);
    // return 1;
  }
}
pub trait QFontInfo_strikeOut_0<RetType> {
  fn strikeOut_0(self , rsthis: & QFontInfo) -> RetType;
}
impl<'a> /*trait*/ QFontInfo_strikeOut_0<bool> for () {
  fn strikeOut_0(self , rsthis: & QFontInfo) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QFontInfo9strikeOutEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfontinfo.h:73
// index:0
// Public Visibility=Default Availability=Available
// [1] bool fixedPitch() const

/*
Returns the fixed pitch value of the matched window system font.

See also QFont::fixedPitch().
*/
impl /*struct*/ QFontInfo {
  pub fn fixedPitch_0<RetType, T: QFontInfo_fixedPitch_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fixedPitch_0(self);
    // return 1;
  }
}
pub trait QFontInfo_fixedPitch_0<RetType> {
  fn fixedPitch_0(self , rsthis: & QFontInfo) -> RetType;
}
impl<'a> /*trait*/ QFontInfo_fixedPitch_0<bool> for () {
  fn fixedPitch_0(self , rsthis: & QFontInfo) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QFontInfo10fixedPitchEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfontinfo.h:74
// index:0
// Public Visibility=Default Availability=Available
// [4] QFont::StyleHint styleHint() const

/*
Returns the style of the matched window system font.

Currently only returns the style hint set in QFont.

See also QFont::styleHint() and QFont::StyleHint.
*/
impl /*struct*/ QFontInfo {
  pub fn styleHint_0<RetType, T: QFontInfo_styleHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.styleHint_0(self);
    // return 1;
  }
}
pub trait QFontInfo_styleHint_0<RetType> {
  fn styleHint_0(self , rsthis: & QFontInfo) -> RetType;
}
impl<'a> /*trait*/ QFontInfo_styleHint_0<i32> for () {
  fn styleHint_0(self , rsthis: & QFontInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QFontInfo9styleHintEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfontinfo.h:76
// index:0
// Public Visibility=Default Availability=Available
// [1] bool rawMode() const

/*

*/
impl /*struct*/ QFontInfo {
  pub fn rawMode_0<RetType, T: QFontInfo_rawMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rawMode_0(self);
    // return 1;
  }
}
pub trait QFontInfo_rawMode_0<RetType> {
  fn rawMode_0(self , rsthis: & QFontInfo) -> RetType;
}
impl<'a> /*trait*/ QFontInfo_rawMode_0<bool> for () {
  fn rawMode_0(self , rsthis: & QFontInfo) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QFontInfo7rawModeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfontinfo.h:79
// index:0
// Public Visibility=Default Availability=Available
// [1] bool exactMatch() const

/*
Returns true if the matched window system font is exactly the same as the one specified by the font; otherwise returns false.

See also QFont::exactMatch().
*/
impl /*struct*/ QFontInfo {
  pub fn exactMatch_0<RetType, T: QFontInfo_exactMatch_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.exactMatch_0(self);
    // return 1;
  }
}
pub trait QFontInfo_exactMatch_0<RetType> {
  fn exactMatch_0(self , rsthis: & QFontInfo) -> RetType;
}
impl<'a> /*trait*/ QFontInfo_exactMatch_0<bool> for () {
  fn exactMatch_0(self , rsthis: & QFontInfo) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QFontInfo10exactMatchEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
