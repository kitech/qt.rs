

// mod ::gui::QFont
// package qtgui
// /usr/include/qt/QtGui/qfont.h
// #include <qfont.h>
// #include <QtGui>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 2
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
#[derive(Default)] // class sizeof(QFont)=16
pub struct QFont {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QFont_ITF interface {
//    QFont_PTR() *QFont
//}
//func (ptr *QFont) QFont_PTR() *QFont { return ptr }

impl /*struct*/ QFont {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QFont {
    return QFont{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QFont {
//  type Target = QFontBASE;
//
//  fn deref(&self) -> &QFontBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QFontBASE> for QFont {
//  fn as_ref(& self) -> & QFontBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qfont.h:170
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QFont()

/*
Constructs a font object that uses the application's default font.

See also QGuiApplication::setFont() and QGuiApplication::font().
*/
// QFont() ctx.fn_proto_cpp
impl /*struct*/ QFont {
  pub fn QFont_0<T: QFont_QFont_0>(value: T) -> QFont {
    let rsthis = value.QFont_0();
    return rsthis;
    // return 1;
  }
}

pub trait QFont_QFont_0 {
  fn QFont_0(self) -> QFont;
}
// QFont() ctx.fn_proto_cpp
impl<'a> /*trait*/ QFont_QFont_0 for () {
  fn QFont_0(self) -> QFont {
    // unsafe{_ZN5QFontC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN5QFontC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QFont{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qfont.h:171
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QFont(const QString &, int, int, bool)

/*
Constructs a font object that uses the application's default font.

See also QGuiApplication::setFont() and QGuiApplication::font().
*/
// QFont(const QString &, int, int, bool) ctx.fn_proto_cpp
impl /*struct*/ QFont {
  pub fn QFont_1<T: QFont_QFont_1>(value: T) -> QFont {
    let rsthis = value.QFont_1();
    return rsthis;
    // return 1;
  }
}

pub trait QFont_QFont_1 {
  fn QFont_1(self) -> QFont;
}
// QFont(const QString &, int, int, bool) ctx.fn_proto_cpp
impl<'a> /*trait*/ QFont_QFont_1 for (usize,i32,i32,bool) {
  fn QFont_1(self) -> QFont {
    // unsafe{_ZN5QFontC2ERK7QStringiib()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const bool as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN5QFontC2ERK7QStringiib", 4,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QFont{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qfont.h:172
// index:2
// Public Visibility=Default Availability=Available
// [-2] void QFont(const QFont &, QPaintDevice *)

/*
Constructs a font object that uses the application's default font.

See also QGuiApplication::setFont() and QGuiApplication::font().
*/
// QFont(const QFont &, QPaintDevice *) ctx.fn_proto_cpp
impl /*struct*/ QFont {
  pub fn QFont_2<T: QFont_QFont_2>(value: T) -> QFont {
    let rsthis = value.QFont_2();
    return rsthis;
    // return 1;
  }
}

pub trait QFont_QFont_2 {
  fn QFont_2(self) -> QFont;
}
// QFont(const QFont &, QPaintDevice *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QFont_QFont_2 for (usize,usize) {
  fn QFont_2(self) -> QFont {
    // unsafe{_ZN5QFontC2ERKS_P12QPaintDevice()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN5QFontC2ERKS_P12QPaintDevice", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QFont{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qfont.h:174
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ~QFont()

/*

*/
pub fn DeleteQFont(this :*mut QFont) {
    // let rv = qtrt::InvokeQtFunc6("_ZN5QFontD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qfont.h:176
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void swap(QFont &)

/*
Swaps this font instance with other. This function is very fast and never fails.

This function was introduced in  Qt 5.0.
*/
impl /*struct*/ QFont {
  pub fn swap_0<RetType, T: QFont_swap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.swap_0(self);
    // return 1;
  }
}
pub trait QFont_swap_0<RetType> {
  fn swap_0(self , rsthis: & QFont) -> RetType;
}
impl<'a> /*trait*/ QFont_swap_0<(/*void*/)> for (usize) {
  fn swap_0(self , rsthis: & QFont) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN5QFont4swapERS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qfont.h:179
// index:0
// Public Visibility=Default Availability=Available
// [8] QString family() const

/*
Returns the requested font family name, i.e. the name set in the constructor or the last setFont() call.

See also setFamily(), substitutes(), and substitute().
*/
impl /*struct*/ QFont {
  pub fn family_0<RetType, T: QFont_family_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.family_0(self);
    // return 1;
  }
}
pub trait QFont_family_0<RetType> {
  fn family_0(self , rsthis: & QFont) -> RetType;
}
impl<'a> /*trait*/ QFont_family_0<usize> for () {
  fn family_0(self , rsthis: & QFont) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QFont6familyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfont.h:180
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFamily(const QString &)

/*
Sets the family name of the font. The name is case insensitive and may include a foundry name.

The family name may optionally also include a foundry name, e.g. "Helvetica [Cronyx]". If the family is available from more than one foundry and the foundry isn't specified, an arbitrary foundry is chosen. If the family isn't available a family will be set using the font matching algorithm.

See also family(), setStyleHint(), and QFontInfo.
*/
impl /*struct*/ QFont {
  pub fn setFamily_0<RetType, T: QFont_setFamily_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFamily_0(self);
    // return 1;
  }
}
pub trait QFont_setFamily_0<RetType> {
  fn setFamily_0(self , rsthis: & QFont) -> RetType;
}
impl<'a> /*trait*/ QFont_setFamily_0<(/*void*/)> for (usize) {
  fn setFamily_0(self , rsthis: & QFont) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN5QFont9setFamilyERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qfont.h:182
// index:0
// Public Visibility=Default Availability=Available
// [8] QString styleName() const

/*
Returns the requested font style name. This can be used to match the font with irregular styles (that can't be normalized in other style properties).

This function was introduced in  Qt 4.8.

See also setStyleName(), setFamily(), and setStyle().
*/
impl /*struct*/ QFont {
  pub fn styleName_0<RetType, T: QFont_styleName_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.styleName_0(self);
    // return 1;
  }
}
pub trait QFont_styleName_0<RetType> {
  fn styleName_0(self , rsthis: & QFont) -> RetType;
}
impl<'a> /*trait*/ QFont_styleName_0<usize> for () {
  fn styleName_0(self , rsthis: & QFont) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QFont9styleNameEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfont.h:183
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setStyleName(const QString &)

/*
Sets the style name of the font to styleName. When set, other style properties like style() and weight() will be ignored for font matching, though they may be simulated afterwards if supported by the platform's font engine.

Due to the lower quality of artificially simulated styles, and the lack of full cross platform support, it is not recommended to use matching by style name together with matching by style properties

This function was introduced in  Qt 4.8.

See also styleName().
*/
impl /*struct*/ QFont {
  pub fn setStyleName_0<RetType, T: QFont_setStyleName_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setStyleName_0(self);
    // return 1;
  }
}
pub trait QFont_setStyleName_0<RetType> {
  fn setStyleName_0(self , rsthis: & QFont) -> RetType;
}
impl<'a> /*trait*/ QFont_setStyleName_0<(/*void*/)> for (usize) {
  fn setStyleName_0(self , rsthis: & QFont) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN5QFont12setStyleNameERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qfont.h:185
// index:0
// Public Visibility=Default Availability=Available
// [4] int pointSize() const

/*
Returns the point size of the font. Returns -1 if the font size was specified in pixels.

See also setPointSize() and pointSizeF().
*/
impl /*struct*/ QFont {
  pub fn pointSize_0<RetType, T: QFont_pointSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.pointSize_0(self);
    // return 1;
  }
}
pub trait QFont_pointSize_0<RetType> {
  fn pointSize_0(self , rsthis: & QFont) -> RetType;
}
impl<'a> /*trait*/ QFont_pointSize_0<i32> for () {
  fn pointSize_0(self , rsthis: & QFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QFont9pointSizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfont.h:186
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setPointSize(int)

/*
Sets the point size to pointSize. The point size must be greater than zero.

See also pointSize() and setPointSizeF().
*/
impl /*struct*/ QFont {
  pub fn setPointSize_0<RetType, T: QFont_setPointSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPointSize_0(self);
    // return 1;
  }
}
pub trait QFont_setPointSize_0<RetType> {
  fn setPointSize_0(self , rsthis: & QFont) -> RetType;
}
impl<'a> /*trait*/ QFont_setPointSize_0<(/*void*/)> for (i32) {
  fn setPointSize_0(self , rsthis: & QFont) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN5QFont12setPointSizeEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qfont.h:187
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal pointSizeF() const

/*
Returns the point size of the font. Returns -1 if the font size was specified in pixels.

See also pointSize(), setPointSizeF(), pixelSize(), QFontInfo::pointSize(), and QFontInfo::pixelSize().
*/
impl /*struct*/ QFont {
  pub fn pointSizeF_0<RetType, T: QFont_pointSizeF_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.pointSizeF_0(self);
    // return 1;
  }
}
pub trait QFont_pointSizeF_0<RetType> {
  fn pointSizeF_0(self , rsthis: & QFont) -> RetType;
}
impl<'a> /*trait*/ QFont_pointSizeF_0<f64> for () {
  fn pointSizeF_0(self , rsthis: & QFont) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QFont10pointSizeFEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfont.h:188
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setPointSizeF(qreal)

/*
Sets the point size to pointSize. The point size must be greater than zero. The requested precision may not be achieved on all platforms.

See also pointSizeF(), setPointSize(), and setPixelSize().
*/
impl /*struct*/ QFont {
  pub fn setPointSizeF_0<RetType, T: QFont_setPointSizeF_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPointSizeF_0(self);
    // return 1;
  }
}
pub trait QFont_setPointSizeF_0<RetType> {
  fn setPointSizeF_0(self , rsthis: & QFont) -> RetType;
}
impl<'a> /*trait*/ QFont_setPointSizeF_0<(/*void*/)> for (f64) {
  fn setPointSizeF_0(self , rsthis: & QFont) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN5QFont13setPointSizeFEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qfont.h:190
// index:0
// Public Visibility=Default Availability=Available
// [4] int pixelSize() const

/*
Returns the pixel size of the font if it was set with setPixelSize(). Returns -1 if the size was set with setPointSize() or setPointSizeF().

See also setPixelSize(), pointSize(), QFontInfo::pointSize(), and QFontInfo::pixelSize().
*/
impl /*struct*/ QFont {
  pub fn pixelSize_0<RetType, T: QFont_pixelSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.pixelSize_0(self);
    // return 1;
  }
}
pub trait QFont_pixelSize_0<RetType> {
  fn pixelSize_0(self , rsthis: & QFont) -> RetType;
}
impl<'a> /*trait*/ QFont_pixelSize_0<i32> for () {
  fn pixelSize_0(self , rsthis: & QFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QFont9pixelSizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfont.h:191
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setPixelSize(int)

/*
Sets the font size to pixelSize pixels.

Using this function makes the font device dependent. Use setPointSize() or setPointSizeF() to set the size of the font in a device independent manner.

See also pixelSize().
*/
impl /*struct*/ QFont {
  pub fn setPixelSize_0<RetType, T: QFont_setPixelSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPixelSize_0(self);
    // return 1;
  }
}
pub trait QFont_setPixelSize_0<RetType> {
  fn setPixelSize_0(self , rsthis: & QFont) -> RetType;
}
impl<'a> /*trait*/ QFont_setPixelSize_0<(/*void*/)> for (i32) {
  fn setPixelSize_0(self , rsthis: & QFont) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN5QFont12setPixelSizeEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qfont.h:193
// index:0
// Public Visibility=Default Availability=Available
// [4] int weight() const

/*
Returns the weight of the font, using the same scale as the QFont::Weight enumeration.

See also setWeight(), Weight, and QFontInfo.
*/
impl /*struct*/ QFont {
  pub fn weight_0<RetType, T: QFont_weight_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.weight_0(self);
    // return 1;
  }
}
pub trait QFont_weight_0<RetType> {
  fn weight_0(self , rsthis: & QFont) -> RetType;
}
impl<'a> /*trait*/ QFont_weight_0<i32> for () {
  fn weight_0(self , rsthis: & QFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QFont6weightEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfont.h:194
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setWeight(int)

/*
Sets the weight of the font to weight, using the scale defined by QFont::Weight enumeration.

Note: If styleName() is set, this value may be ignored for font selection.

See also weight() and QFontInfo.
*/
impl /*struct*/ QFont {
  pub fn setWeight_0<RetType, T: QFont_setWeight_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setWeight_0(self);
    // return 1;
  }
}
pub trait QFont_setWeight_0<RetType> {
  fn setWeight_0(self , rsthis: & QFont) -> RetType;
}
impl<'a> /*trait*/ QFont_setWeight_0<(/*void*/)> for (i32) {
  fn setWeight_0(self , rsthis: & QFont) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN5QFont9setWeightEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qfont.h:196
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool bold() const

/*
Returns true if weight() is a value greater than QFont::Medium; otherwise returns false.

See also weight(), setBold(), and QFontInfo::bold().
*/
impl /*struct*/ QFont {
  pub fn bold_0<RetType, T: QFont_bold_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.bold_0(self);
    // return 1;
  }
}
pub trait QFont_bold_0<RetType> {
  fn bold_0(self , rsthis: & QFont) -> RetType;
}
impl<'a> /*trait*/ QFont_bold_0<bool> for () {
  fn bold_0(self , rsthis: & QFont) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QFont4boldEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfont.h:197
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setBold(bool)

/*
If enable is true sets the font's weight to QFont::Bold; otherwise sets the weight to QFont::Normal.

For finer boldness control use setWeight().

Note: If styleName() is set, this value may be ignored, or if supported on the platform, the font artificially embolded.

See also bold() and setWeight().
*/
impl /*struct*/ QFont {
  pub fn setBold_0<RetType, T: QFont_setBold_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setBold_0(self);
    // return 1;
  }
}
pub trait QFont_setBold_0<RetType> {
  fn setBold_0(self , rsthis: & QFont) -> RetType;
}
impl<'a> /*trait*/ QFont_setBold_0<(/*void*/)> for (bool) {
  fn setBold_0(self , rsthis: & QFont) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN5QFont7setBoldEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qfont.h:199
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setStyle(QFont::Style)

/*
Sets the style of the font to style.

See also style(), italic(), and QFontInfo.
*/
impl /*struct*/ QFont {
  pub fn setStyle_0<RetType, T: QFont_setStyle_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setStyle_0(self);
    // return 1;
  }
}
pub trait QFont_setStyle_0<RetType> {
  fn setStyle_0(self , rsthis: & QFont) -> RetType;
}
impl<'a> /*trait*/ QFont_setStyle_0<(/*void*/)> for (i32) {
  fn setStyle_0(self , rsthis: & QFont) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN5QFont8setStyleENS_5StyleE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qfont.h:200
// index:0
// Public Visibility=Default Availability=Available
// [4] QFont::Style style() const

/*
Returns the style of the font.

See also setStyle().
*/
impl /*struct*/ QFont {
  pub fn style_0<RetType, T: QFont_style_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.style_0(self);
    // return 1;
  }
}
pub trait QFont_style_0<RetType> {
  fn style_0(self , rsthis: & QFont) -> RetType;
}
impl<'a> /*trait*/ QFont_style_0<i32> for () {
  fn style_0(self , rsthis: & QFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QFont5styleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfont.h:202
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool italic() const

/*
Returns true if the style() of the font is not QFont::StyleNormal

See also setItalic() and style().
*/
impl /*struct*/ QFont {
  pub fn italic_0<RetType, T: QFont_italic_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.italic_0(self);
    // return 1;
  }
}
pub trait QFont_italic_0<RetType> {
  fn italic_0(self , rsthis: & QFont) -> RetType;
}
impl<'a> /*trait*/ QFont_italic_0<bool> for () {
  fn italic_0(self , rsthis: & QFont) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QFont6italicEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfont.h:203
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setItalic(bool)

/*
Sets the style() of the font to QFont::StyleItalic if enable is true; otherwise the style is set to QFont::StyleNormal.

Note: If styleName() is set, this value may be ignored, or if supported on the platform, the font may be rendered tilted instead of picking a designed italic font-variant.

See also italic() and QFontInfo.
*/
impl /*struct*/ QFont {
  pub fn setItalic_0<RetType, T: QFont_setItalic_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setItalic_0(self);
    // return 1;
  }
}
pub trait QFont_setItalic_0<RetType> {
  fn setItalic_0(self , rsthis: & QFont) -> RetType;
}
impl<'a> /*trait*/ QFont_setItalic_0<(/*void*/)> for (bool) {
  fn setItalic_0(self , rsthis: & QFont) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN5QFont9setItalicEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qfont.h:205
// index:0
// Public Visibility=Default Availability=Available
// [1] bool underline() const

/*
Returns true if underline has been set; otherwise returns false.

See also setUnderline().
*/
impl /*struct*/ QFont {
  pub fn underline_0<RetType, T: QFont_underline_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.underline_0(self);
    // return 1;
  }
}
pub trait QFont_underline_0<RetType> {
  fn underline_0(self , rsthis: & QFont) -> RetType;
}
impl<'a> /*trait*/ QFont_underline_0<bool> for () {
  fn underline_0(self , rsthis: & QFont) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QFont9underlineEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfont.h:206
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setUnderline(bool)

/*
If enable is true, sets underline on; otherwise sets underline off.

See also underline() and QFontInfo.
*/
impl /*struct*/ QFont {
  pub fn setUnderline_0<RetType, T: QFont_setUnderline_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setUnderline_0(self);
    // return 1;
  }
}
pub trait QFont_setUnderline_0<RetType> {
  fn setUnderline_0(self , rsthis: & QFont) -> RetType;
}
impl<'a> /*trait*/ QFont_setUnderline_0<(/*void*/)> for (bool) {
  fn setUnderline_0(self , rsthis: & QFont) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN5QFont12setUnderlineEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qfont.h:208
// index:0
// Public Visibility=Default Availability=Available
// [1] bool overline() const

/*
Returns true if overline has been set; otherwise returns false.

See also setOverline().
*/
impl /*struct*/ QFont {
  pub fn overline_0<RetType, T: QFont_overline_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.overline_0(self);
    // return 1;
  }
}
pub trait QFont_overline_0<RetType> {
  fn overline_0(self , rsthis: & QFont) -> RetType;
}
impl<'a> /*trait*/ QFont_overline_0<bool> for () {
  fn overline_0(self , rsthis: & QFont) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QFont8overlineEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfont.h:209
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setOverline(bool)

/*
If enable is true, sets overline on; otherwise sets overline off.

See also overline() and QFontInfo.
*/
impl /*struct*/ QFont {
  pub fn setOverline_0<RetType, T: QFont_setOverline_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setOverline_0(self);
    // return 1;
  }
}
pub trait QFont_setOverline_0<RetType> {
  fn setOverline_0(self , rsthis: & QFont) -> RetType;
}
impl<'a> /*trait*/ QFont_setOverline_0<(/*void*/)> for (bool) {
  fn setOverline_0(self , rsthis: & QFont) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN5QFont11setOverlineEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qfont.h:211
// index:0
// Public Visibility=Default Availability=Available
// [1] bool strikeOut() const

/*
Returns true if strikeout has been set; otherwise returns false.

See also setStrikeOut().
*/
impl /*struct*/ QFont {
  pub fn strikeOut_0<RetType, T: QFont_strikeOut_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.strikeOut_0(self);
    // return 1;
  }
}
pub trait QFont_strikeOut_0<RetType> {
  fn strikeOut_0(self , rsthis: & QFont) -> RetType;
}
impl<'a> /*trait*/ QFont_strikeOut_0<bool> for () {
  fn strikeOut_0(self , rsthis: & QFont) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QFont9strikeOutEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfont.h:212
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setStrikeOut(bool)

/*
If enable is true, sets strikeout on; otherwise sets strikeout off.

See also strikeOut() and QFontInfo.
*/
impl /*struct*/ QFont {
  pub fn setStrikeOut_0<RetType, T: QFont_setStrikeOut_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setStrikeOut_0(self);
    // return 1;
  }
}
pub trait QFont_setStrikeOut_0<RetType> {
  fn setStrikeOut_0(self , rsthis: & QFont) -> RetType;
}
impl<'a> /*trait*/ QFont_setStrikeOut_0<(/*void*/)> for (bool) {
  fn setStrikeOut_0(self , rsthis: & QFont) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN5QFont12setStrikeOutEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qfont.h:214
// index:0
// Public Visibility=Default Availability=Available
// [1] bool fixedPitch() const

/*
Returns true if fixed pitch has been set; otherwise returns false.

See also setFixedPitch() and QFontInfo::fixedPitch().
*/
impl /*struct*/ QFont {
  pub fn fixedPitch_0<RetType, T: QFont_fixedPitch_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fixedPitch_0(self);
    // return 1;
  }
}
pub trait QFont_fixedPitch_0<RetType> {
  fn fixedPitch_0(self , rsthis: & QFont) -> RetType;
}
impl<'a> /*trait*/ QFont_fixedPitch_0<bool> for () {
  fn fixedPitch_0(self , rsthis: & QFont) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QFont10fixedPitchEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfont.h:215
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFixedPitch(bool)

/*
If enable is true, sets fixed pitch on; otherwise sets fixed pitch off.

See also fixedPitch() and QFontInfo.
*/
impl /*struct*/ QFont {
  pub fn setFixedPitch_0<RetType, T: QFont_setFixedPitch_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFixedPitch_0(self);
    // return 1;
  }
}
pub trait QFont_setFixedPitch_0<RetType> {
  fn setFixedPitch_0(self , rsthis: & QFont) -> RetType;
}
impl<'a> /*trait*/ QFont_setFixedPitch_0<(/*void*/)> for (bool) {
  fn setFixedPitch_0(self , rsthis: & QFont) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN5QFont13setFixedPitchEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qfont.h:217
// index:0
// Public Visibility=Default Availability=Available
// [1] bool kerning() const

/*
Returns true if kerning should be used when drawing text with this font.

See also setKerning().
*/
impl /*struct*/ QFont {
  pub fn kerning_0<RetType, T: QFont_kerning_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.kerning_0(self);
    // return 1;
  }
}
pub trait QFont_kerning_0<RetType> {
  fn kerning_0(self , rsthis: & QFont) -> RetType;
}
impl<'a> /*trait*/ QFont_kerning_0<bool> for () {
  fn kerning_0(self , rsthis: & QFont) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QFont7kerningEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfont.h:218
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setKerning(bool)

/*
Enables kerning for this font if enable is true; otherwise disables it. By default, kerning is enabled.

When kerning is enabled, glyph metrics do not add up anymore, even for Latin text. In other words, the assumption that width('a') + width('b') is equal to width("ab") is not necessarily true.

See also kerning() and QFontMetrics.
*/
impl /*struct*/ QFont {
  pub fn setKerning_0<RetType, T: QFont_setKerning_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setKerning_0(self);
    // return 1;
  }
}
pub trait QFont_setKerning_0<RetType> {
  fn setKerning_0(self , rsthis: & QFont) -> RetType;
}
impl<'a> /*trait*/ QFont_setKerning_0<(/*void*/)> for (bool) {
  fn setKerning_0(self , rsthis: & QFont) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN5QFont10setKerningEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qfont.h:220
// index:0
// Public Visibility=Default Availability=Available
// [4] QFont::StyleHint styleHint() const

/*
Returns the StyleHint.

The style hint affects the font matching algorithm. See QFont::StyleHint for the list of available hints.

See also setStyleHint(), QFont::StyleStrategy, and QFontInfo::styleHint().
*/
impl /*struct*/ QFont {
  pub fn styleHint_0<RetType, T: QFont_styleHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.styleHint_0(self);
    // return 1;
  }
}
pub trait QFont_styleHint_0<RetType> {
  fn styleHint_0(self , rsthis: & QFont) -> RetType;
}
impl<'a> /*trait*/ QFont_styleHint_0<i32> for () {
  fn styleHint_0(self , rsthis: & QFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QFont9styleHintEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfont.h:221
// index:0
// Public Visibility=Default Availability=Available
// [4] QFont::StyleStrategy styleStrategy() const

/*
Returns the StyleStrategy.

The style strategy affects the font matching algorithm. See QFont::StyleStrategy for the list of available strategies.

See also setStyleStrategy(), setStyleHint(), and QFont::StyleHint.
*/
impl /*struct*/ QFont {
  pub fn styleStrategy_0<RetType, T: QFont_styleStrategy_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.styleStrategy_0(self);
    // return 1;
  }
}
pub trait QFont_styleStrategy_0<RetType> {
  fn styleStrategy_0(self , rsthis: & QFont) -> RetType;
}
impl<'a> /*trait*/ QFont_styleStrategy_0<i32> for () {
  fn styleStrategy_0(self , rsthis: & QFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QFont13styleStrategyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfont.h:222
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setStyleHint(QFont::StyleHint, QFont::StyleStrategy)

/*
Sets the style hint and strategy to hint and strategy, respectively.

If these aren't set explicitly the style hint will default to AnyStyle and the style strategy to PreferDefault.

Qt does not support style hints on X11 since this information is not provided by the window system.

See also StyleHint, styleHint(), StyleStrategy, styleStrategy(), and QFontInfo.
*/
impl /*struct*/ QFont {
  pub fn setStyleHint_0<RetType, T: QFont_setStyleHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setStyleHint_0(self);
    // return 1;
  }
}
pub trait QFont_setStyleHint_0<RetType> {
  fn setStyleHint_0(self , rsthis: & QFont) -> RetType;
}
impl<'a> /*trait*/ QFont_setStyleHint_0<(/*void*/)> for (i32,i32) {
  fn setStyleHint_0(self , rsthis: & QFont) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN5QFont12setStyleHintENS_9StyleHintENS_13StyleStrategyE", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qfont.h:223
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setStyleStrategy(QFont::StyleStrategy)

/*
Sets the style strategy for the font to s.

See also styleStrategy() and QFont::StyleStrategy.
*/
impl /*struct*/ QFont {
  pub fn setStyleStrategy_0<RetType, T: QFont_setStyleStrategy_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setStyleStrategy_0(self);
    // return 1;
  }
}
pub trait QFont_setStyleStrategy_0<RetType> {
  fn setStyleStrategy_0(self , rsthis: & QFont) -> RetType;
}
impl<'a> /*trait*/ QFont_setStyleStrategy_0<(/*void*/)> for (i32) {
  fn setStyleStrategy_0(self , rsthis: & QFont) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN5QFont16setStyleStrategyENS_13StyleStrategyE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qfont.h:225
// index:0
// Public Visibility=Default Availability=Available
// [4] int stretch() const

/*
Returns the stretch factor for the font.

See also setStretch().
*/
impl /*struct*/ QFont {
  pub fn stretch_0<RetType, T: QFont_stretch_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.stretch_0(self);
    // return 1;
  }
}
pub trait QFont_stretch_0<RetType> {
  fn stretch_0(self , rsthis: & QFont) -> RetType;
}
impl<'a> /*trait*/ QFont_stretch_0<i32> for () {
  fn stretch_0(self , rsthis: & QFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QFont7stretchEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfont.h:226
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setStretch(int)

/*
Sets the stretch factor for the font.

The stretch factor matches a condensed or expanded version of the font or applies a stretch transform that changes the width of all characters in the font by factor percent. For example, setting factor to 150 results in all characters in the font being 1.5 times (ie. 150%) wider. The minimum stretch factor is 1, and the maximum stretch factor is 4000. The default stretch factor is AnyStretch, which will accept any stretch factor and not apply any transform on the font.

The stretch factor is only applied to outline fonts. The stretch factor is ignored for bitmap fonts.

Note: When matching a font with a native non-default stretch factor, requesting a stretch of 100 will stretch it back to a medium width font.

See also stretch() and QFont::Stretch.
*/
impl /*struct*/ QFont {
  pub fn setStretch_0<RetType, T: QFont_setStretch_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setStretch_0(self);
    // return 1;
  }
}
pub trait QFont_setStretch_0<RetType> {
  fn setStretch_0(self , rsthis: & QFont) -> RetType;
}
impl<'a> /*trait*/ QFont_setStretch_0<(/*void*/)> for (i32) {
  fn setStretch_0(self , rsthis: & QFont) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN5QFont10setStretchEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qfont.h:228
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal letterSpacing() const

/*
Returns the letter spacing for the font.

This function was introduced in  Qt 4.4.

See also setLetterSpacing(), letterSpacingType(), and setWordSpacing().
*/
impl /*struct*/ QFont {
  pub fn letterSpacing_0<RetType, T: QFont_letterSpacing_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.letterSpacing_0(self);
    // return 1;
  }
}
pub trait QFont_letterSpacing_0<RetType> {
  fn letterSpacing_0(self , rsthis: & QFont) -> RetType;
}
impl<'a> /*trait*/ QFont_letterSpacing_0<f64> for () {
  fn letterSpacing_0(self , rsthis: & QFont) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QFont13letterSpacingEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfont.h:229
// index:0
// Public Visibility=Default Availability=Available
// [4] QFont::SpacingType letterSpacingType() const

/*
Returns the spacing type used for letter spacing.

This function was introduced in  Qt 4.4.

See also letterSpacing(), setLetterSpacing(), and setWordSpacing().
*/
impl /*struct*/ QFont {
  pub fn letterSpacingType_0<RetType, T: QFont_letterSpacingType_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.letterSpacingType_0(self);
    // return 1;
  }
}
pub trait QFont_letterSpacingType_0<RetType> {
  fn letterSpacingType_0(self , rsthis: & QFont) -> RetType;
}
impl<'a> /*trait*/ QFont_letterSpacingType_0<i32> for () {
  fn letterSpacingType_0(self , rsthis: & QFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QFont17letterSpacingTypeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfont.h:230
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setLetterSpacing(QFont::SpacingType, qreal)

/*
Sets the letter spacing for the font to spacing and the type of spacing to type.

Letter spacing changes the default spacing between individual letters in the font. The spacing between the letters can be made smaller as well as larger either in percentage of the character width or in pixels, depending on the selected spacing type.

This function was introduced in  Qt 4.4.

See also letterSpacing(), letterSpacingType(), and setWordSpacing().
*/
impl /*struct*/ QFont {
  pub fn setLetterSpacing_0<RetType, T: QFont_setLetterSpacing_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setLetterSpacing_0(self);
    // return 1;
  }
}
pub trait QFont_setLetterSpacing_0<RetType> {
  fn setLetterSpacing_0(self , rsthis: & QFont) -> RetType;
}
impl<'a> /*trait*/ QFont_setLetterSpacing_0<(/*void*/)> for (i32,f64) {
  fn setLetterSpacing_0(self , rsthis: & QFont) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN5QFont16setLetterSpacingENS_11SpacingTypeEd", 2,qtrt::FFITY_INT,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qfont.h:232
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal wordSpacing() const

/*
Returns the word spacing for the font.

This function was introduced in  Qt 4.4.

See also setWordSpacing() and setLetterSpacing().
*/
impl /*struct*/ QFont {
  pub fn wordSpacing_0<RetType, T: QFont_wordSpacing_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.wordSpacing_0(self);
    // return 1;
  }
}
pub trait QFont_wordSpacing_0<RetType> {
  fn wordSpacing_0(self , rsthis: & QFont) -> RetType;
}
impl<'a> /*trait*/ QFont_wordSpacing_0<f64> for () {
  fn wordSpacing_0(self , rsthis: & QFont) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QFont11wordSpacingEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfont.h:233
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setWordSpacing(qreal)

/*
Sets the word spacing for the font to spacing.

Word spacing changes the default spacing between individual words. A positive value increases the word spacing by a corresponding amount of pixels, while a negative value decreases the inter-word spacing accordingly.

Word spacing will not apply to writing systems, where indiviaul words are not separated by white space.

This function was introduced in  Qt 4.4.

See also wordSpacing() and setLetterSpacing().
*/
impl /*struct*/ QFont {
  pub fn setWordSpacing_0<RetType, T: QFont_setWordSpacing_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setWordSpacing_0(self);
    // return 1;
  }
}
pub trait QFont_setWordSpacing_0<RetType> {
  fn setWordSpacing_0(self , rsthis: & QFont) -> RetType;
}
impl<'a> /*trait*/ QFont_setWordSpacing_0<(/*void*/)> for (f64) {
  fn setWordSpacing_0(self , rsthis: & QFont) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN5QFont14setWordSpacingEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qfont.h:235
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCapitalization(QFont::Capitalization)

/*
Sets the capitalization of the text in this font to caps.

A font's capitalization makes the text appear in the selected capitalization mode.

This function was introduced in  Qt 4.4.

See also capitalization().
*/
impl /*struct*/ QFont {
  pub fn setCapitalization_0<RetType, T: QFont_setCapitalization_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCapitalization_0(self);
    // return 1;
  }
}
pub trait QFont_setCapitalization_0<RetType> {
  fn setCapitalization_0(self , rsthis: & QFont) -> RetType;
}
impl<'a> /*trait*/ QFont_setCapitalization_0<(/*void*/)> for (i32) {
  fn setCapitalization_0(self , rsthis: & QFont) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN5QFont17setCapitalizationENS_14CapitalizationE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qfont.h:236
// index:0
// Public Visibility=Default Availability=Available
// [4] QFont::Capitalization capitalization() const

/*
Returns the current capitalization type of the font.

This function was introduced in  Qt 4.4.

See also setCapitalization().
*/
impl /*struct*/ QFont {
  pub fn capitalization_0<RetType, T: QFont_capitalization_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.capitalization_0(self);
    // return 1;
  }
}
pub trait QFont_capitalization_0<RetType> {
  fn capitalization_0(self , rsthis: & QFont) -> RetType;
}
impl<'a> /*trait*/ QFont_capitalization_0<i32> for () {
  fn capitalization_0(self , rsthis: & QFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QFont14capitalizationEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfont.h:238
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setHintingPreference(QFont::HintingPreference)

/*
Set the preference for the hinting level of the glyphs to hintingPreference. This is a hint to the underlying font rendering system to use a certain level of hinting, and has varying support across platforms. See the table in the documentation for QFont::HintingPreference for more details.

The default hinting preference is QFont::PreferDefaultHinting.

This function was introduced in  Qt 4.8.

See also hintingPreference().
*/
impl /*struct*/ QFont {
  pub fn setHintingPreference_0<RetType, T: QFont_setHintingPreference_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setHintingPreference_0(self);
    // return 1;
  }
}
pub trait QFont_setHintingPreference_0<RetType> {
  fn setHintingPreference_0(self , rsthis: & QFont) -> RetType;
}
impl<'a> /*trait*/ QFont_setHintingPreference_0<(/*void*/)> for (i32) {
  fn setHintingPreference_0(self , rsthis: & QFont) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN5QFont20setHintingPreferenceENS_17HintingPreferenceE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qfont.h:239
// index:0
// Public Visibility=Default Availability=Available
// [4] QFont::HintingPreference hintingPreference() const

/*
Returns the currently preferred hinting level for glyphs rendered with this font.

This function was introduced in  Qt 4.8.

See also setHintingPreference().
*/
impl /*struct*/ QFont {
  pub fn hintingPreference_0<RetType, T: QFont_hintingPreference_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hintingPreference_0(self);
    // return 1;
  }
}
pub trait QFont_hintingPreference_0<RetType> {
  fn hintingPreference_0(self , rsthis: & QFont) -> RetType;
}
impl<'a> /*trait*/ QFont_hintingPreference_0<i32> for () {
  fn hintingPreference_0(self , rsthis: & QFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QFont17hintingPreferenceEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfont.h:242
// index:0
// Public Visibility=Default Availability=Available
// [1] bool rawMode() const

/*

*/
impl /*struct*/ QFont {
  pub fn rawMode_0<RetType, T: QFont_rawMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rawMode_0(self);
    // return 1;
  }
}
pub trait QFont_rawMode_0<RetType> {
  fn rawMode_0(self , rsthis: & QFont) -> RetType;
}
impl<'a> /*trait*/ QFont_rawMode_0<bool> for () {
  fn rawMode_0(self , rsthis: & QFont) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QFont7rawModeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfont.h:243
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setRawMode(bool)

/*

*/
impl /*struct*/ QFont {
  pub fn setRawMode_0<RetType, T: QFont_setRawMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setRawMode_0(self);
    // return 1;
  }
}
pub trait QFont_setRawMode_0<RetType> {
  fn setRawMode_0(self , rsthis: & QFont) -> RetType;
}
impl<'a> /*trait*/ QFont_setRawMode_0<(/*void*/)> for (bool) {
  fn setRawMode_0(self , rsthis: & QFont) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN5QFont10setRawModeEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qfont.h:247
// index:0
// Public Visibility=Default Availability=Available
// [1] bool exactMatch() const

/*
Returns true if a window system font exactly matching the settings of this font is available.

See also QFontInfo.
*/
impl /*struct*/ QFont {
  pub fn exactMatch_0<RetType, T: QFont_exactMatch_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.exactMatch_0(self);
    // return 1;
  }
}
pub trait QFont_exactMatch_0<RetType> {
  fn exactMatch_0(self , rsthis: & QFont) -> RetType;
}
impl<'a> /*trait*/ QFont_exactMatch_0<bool> for () {
  fn exactMatch_0(self , rsthis: & QFont) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QFont10exactMatchEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfont.h:249
// index:0
// Public Visibility=Default Availability=Available
// [16] QFont & operator=(const QFont &)

/*

*/
impl /*struct*/ QFont {
  pub fn operator_equal_0<RetType, T: QFont_operator_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_0(self);
    // return 1;
  }
}
pub trait QFont_operator_equal_0<RetType> {
  fn operator_equal_0(self , rsthis: & QFont) -> RetType;
}
impl<'a> /*trait*/ QFont_operator_equal_0<usize> for (usize) {
  fn operator_equal_0(self , rsthis: & QFont) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QFontaSERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfont.h:256
// index:1
// Public inline Visibility=Default Availability=Available
// [16] QFont & operator=(QFont &&)

/*

*/
impl /*struct*/ QFont {
  pub fn operator_equal_1<RetType, T: QFont_operator_equal_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_1(self);
    // return 1;
  }
}
pub trait QFont_operator_equal_1<RetType> {
  fn operator_equal_1(self , rsthis: & QFont) -> RetType;
}
impl<'a> /*trait*/ QFont_operator_equal_1<usize> for (usize) {
  fn operator_equal_1(self , rsthis: & QFont) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QFontaSEOS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfont.h:250
// index:0
// Public Visibility=Default Availability=Available
// [1] bool operator==(const QFont &) const

/*

*/
impl /*struct*/ QFont {
  pub fn operator_equal_equal_0<RetType, T: QFont_operator_equal_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_equal_0(self);
    // return 1;
  }
}
pub trait QFont_operator_equal_equal_0<RetType> {
  fn operator_equal_equal_0(self , rsthis: & QFont) -> RetType;
}
impl<'a> /*trait*/ QFont_operator_equal_equal_0<bool> for (usize) {
  fn operator_equal_equal_0(self , rsthis: & QFont) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QFonteqERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfont.h:251
// index:0
// Public Visibility=Default Availability=Available
// [1] bool operator!=(const QFont &) const

/*

*/
impl /*struct*/ QFont {
  pub fn operator_not_equal_0<RetType, T: QFont_operator_not_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_not_equal_0(self);
    // return 1;
  }
}
pub trait QFont_operator_not_equal_0<RetType> {
  fn operator_not_equal_0(self , rsthis: & QFont) -> RetType;
}
impl<'a> /*trait*/ QFont_operator_not_equal_0<bool> for (usize) {
  fn operator_not_equal_0(self , rsthis: & QFont) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QFontneERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfont.h:252
// index:0
// Public Visibility=Default Availability=Available
// [1] bool operator<(const QFont &) const

/*

*/
impl /*struct*/ QFont {
  pub fn operator_less_than_0<RetType, T: QFont_operator_less_than_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_less_than_0(self);
    // return 1;
  }
}
pub trait QFont_operator_less_than_0<RetType> {
  fn operator_less_than_0(self , rsthis: & QFont) -> RetType;
}
impl<'a> /*trait*/ QFont_operator_less_than_0<bool> for (usize) {
  fn operator_less_than_0(self , rsthis: & QFont) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QFontltERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfont.h:254
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isCopyOf(const QFont &) const

/*
Returns true if this font and f are copies of each other, i.e. one of them was created as a copy of the other and neither has been modified since. This is much stricter than equality.

See also operator=() and operator==().
*/
impl /*struct*/ QFont {
  pub fn isCopyOf_0<RetType, T: QFont_isCopyOf_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isCopyOf_0(self);
    // return 1;
  }
}
pub trait QFont_isCopyOf_0<RetType> {
  fn isCopyOf_0(self , rsthis: & QFont) -> RetType;
}
impl<'a> /*trait*/ QFont_isCopyOf_0<bool> for (usize) {
  fn isCopyOf_0(self , rsthis: & QFont) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QFont8isCopyOfERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfont.h:262
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setRawName(const QString &)

/*

*/
impl /*struct*/ QFont {
  pub fn setRawName_0<RetType, T: QFont_setRawName_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setRawName_0(self);
    // return 1;
  }
}
pub trait QFont_setRawName_0<RetType> {
  fn setRawName_0(self , rsthis: & QFont) -> RetType;
}
impl<'a> /*trait*/ QFont_setRawName_0<(/*void*/)> for (usize) {
  fn setRawName_0(self , rsthis: & QFont) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN5QFont10setRawNameERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qfont.h:263
// index:0
// Public Visibility=Default Availability=Available
// [8] QString rawName() const

/*

*/
impl /*struct*/ QFont {
  pub fn rawName_0<RetType, T: QFont_rawName_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rawName_0(self);
    // return 1;
  }
}
pub trait QFont_rawName_0<RetType> {
  fn rawName_0(self , rsthis: & QFont) -> RetType;
}
impl<'a> /*trait*/ QFont_rawName_0<usize> for () {
  fn rawName_0(self , rsthis: & QFont) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QFont7rawNameEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfont.h:266
// index:0
// Public Visibility=Default Availability=Available
// [8] QString key() const

/*
Returns the font's key, a textual representation of a font. It is typically used as the key for a cache or dictionary of fonts.

See also QMap.
*/
impl /*struct*/ QFont {
  pub fn key_0<RetType, T: QFont_key_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.key_0(self);
    // return 1;
  }
}
pub trait QFont_key_0<RetType> {
  fn key_0(self , rsthis: & QFont) -> RetType;
}
impl<'a> /*trait*/ QFont_key_0<usize> for () {
  fn key_0(self , rsthis: & QFont) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QFont3keyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfont.h:268
// index:0
// Public Visibility=Default Availability=Available
// [8] QString toString() const

/*
Returns a description of the font. The description is a comma-separated list of the attributes, perfectly suited for use in QSettings.

See also fromString().
*/
impl /*struct*/ QFont {
  pub fn toString_0<RetType, T: QFont_toString_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toString_0(self);
    // return 1;
  }
}
pub trait QFont_toString_0<RetType> {
  fn toString_0(self , rsthis: & QFont) -> RetType;
}
impl<'a> /*trait*/ QFont_toString_0<usize> for () {
  fn toString_0(self , rsthis: & QFont) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QFont8toStringEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfont.h:269
// index:0
// Public Visibility=Default Availability=Available
// [1] bool fromString(const QString &)

/*
Sets this font to match the description descrip. The description is a comma-separated list of the font attributes, as returned by toString().

See also toString().
*/
impl /*struct*/ QFont {
  pub fn fromString_0<RetType, T: QFont_fromString_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fromString_0(self);
    // return 1;
  }
}
pub trait QFont_fromString_0<RetType> {
  fn fromString_0(self , rsthis: & QFont) -> RetType;
}
impl<'a> /*trait*/ QFont_fromString_0<bool> for (usize) {
  fn fromString_0(self , rsthis: & QFont) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QFont10fromStringERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfont.h:271
// index:0
// Public static Visibility=Default Availability=Available
// [8] QString substitute(const QString &)

/*
Returns the first family name to be used whenever familyName is specified. The lookup is case insensitive.

If there is no substitution for familyName, familyName is returned.

To obtain a list of substitutions use substitutes().

See also setFamily(), insertSubstitutions(), insertSubstitution(), and removeSubstitutions().
*/
impl /*struct*/ QFont {
  pub fn substitute_0<RetType, T: QFont_substitute_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.substitute_0();
    // return 1;
  }
}
pub trait QFont_substitute_0<RetType> {
  fn substitute_0(self ) -> RetType;
}
impl<'a> /*trait*/ QFont_substitute_0<usize> for (usize) {
  fn substitute_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QFont10substituteERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfont.h:272
// index:0
// Public static Visibility=Default Availability=Available
// [8] QStringList substitutes(const QString &)

/*
Returns a list of family names to be used whenever familyName is specified. The lookup is case insensitive.

If there is no substitution for familyName, an empty list is returned.

See also substitute(), insertSubstitutions(), insertSubstitution(), and removeSubstitutions().
*/
impl /*struct*/ QFont {
  pub fn substitutes_0<RetType, T: QFont_substitutes_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.substitutes_0();
    // return 1;
  }
}
pub trait QFont_substitutes_0<RetType> {
  fn substitutes_0(self ) -> RetType;
}
impl<'a> /*trait*/ QFont_substitutes_0<usize> for (usize) {
  fn substitutes_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QFont11substitutesERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfont.h:273
// index:0
// Public static Visibility=Default Availability=Available
// [8] QStringList substitutions()

/*
Returns a sorted list of substituted family names.

See also insertSubstitution(), removeSubstitution(), and substitute().
*/
impl /*struct*/ QFont {
  pub fn substitutions_0<RetType, T: QFont_substitutions_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.substitutions_0();
    // return 1;
  }
}
pub trait QFont_substitutions_0<RetType> {
  fn substitutions_0(self ) -> RetType;
}
impl<'a> /*trait*/ QFont_substitutions_0<usize> for () {
  fn substitutions_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QFont13substitutionsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfont.h:274
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void insertSubstitution(const QString &, const QString &)

/*
Inserts substituteName into the substitution table for the family familyName.

See also insertSubstitutions(), removeSubstitutions(), substitutions(), substitute(), and substitutes().
*/
impl /*struct*/ QFont {
  pub fn insertSubstitution_0<RetType, T: QFont_insertSubstitution_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.insertSubstitution_0();
    // return 1;
  }
}
pub trait QFont_insertSubstitution_0<RetType> {
  fn insertSubstitution_0(self ) -> RetType;
}
impl<'a> /*trait*/ QFont_insertSubstitution_0<(/*void*/)> for (usize,usize) {
  fn insertSubstitution_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN5QFont18insertSubstitutionERK7QStringS2_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qfont.h:275
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void insertSubstitutions(const QString &, const QStringList &)

/*
Inserts the list of families substituteNames into the substitution list for familyName.

See also insertSubstitution(), removeSubstitutions(), substitutions(), and substitute().
*/
impl /*struct*/ QFont {
  pub fn insertSubstitutions_0<RetType, T: QFont_insertSubstitutions_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.insertSubstitutions_0();
    // return 1;
  }
}
pub trait QFont_insertSubstitutions_0<RetType> {
  fn insertSubstitutions_0(self ) -> RetType;
}
impl<'a> /*trait*/ QFont_insertSubstitutions_0<(/*void*/)> for (usize,usize) {
  fn insertSubstitutions_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN5QFont19insertSubstitutionsERK7QStringRK11QStringList", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qfont.h:276
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void removeSubstitutions(const QString &)

/*
Removes all the substitutions for familyName.

This function was introduced in  Qt 5.0.

See also insertSubstitutions(), insertSubstitution(), substitutions(), and substitute().
*/
impl /*struct*/ QFont {
  pub fn removeSubstitutions_0<RetType, T: QFont_removeSubstitutions_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.removeSubstitutions_0();
    // return 1;
  }
}
pub trait QFont_removeSubstitutions_0<RetType> {
  fn removeSubstitutions_0(self ) -> RetType;
}
impl<'a> /*trait*/ QFont_removeSubstitutions_0<(/*void*/)> for (usize) {
  fn removeSubstitutions_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN5QFont19removeSubstitutionsERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qfont.h:280
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void initialize()

/*

*/
impl /*struct*/ QFont {
  pub fn initialize_0<RetType, T: QFont_initialize_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.initialize_0();
    // return 1;
  }
}
pub trait QFont_initialize_0<RetType> {
  fn initialize_0(self ) -> RetType;
}
impl<'a> /*trait*/ QFont_initialize_0<(/*void*/)> for () {
  fn initialize_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN5QFont10initializeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qfont.h:281
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void cleanup()

/*

*/
impl /*struct*/ QFont {
  pub fn cleanup_0<RetType, T: QFont_cleanup_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.cleanup_0();
    // return 1;
  }
}
pub trait QFont_cleanup_0<RetType> {
  fn cleanup_0(self ) -> RetType;
}
impl<'a> /*trait*/ QFont_cleanup_0<(/*void*/)> for () {
  fn cleanup_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN5QFont7cleanupEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qfont.h:282
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void cacheStatistics()

/*

*/
impl /*struct*/ QFont {
  pub fn cacheStatistics_0<RetType, T: QFont_cacheStatistics_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.cacheStatistics_0();
    // return 1;
  }
}
pub trait QFont_cacheStatistics_0<RetType> {
  fn cacheStatistics_0(self ) -> RetType;
}
impl<'a> /*trait*/ QFont_cacheStatistics_0<(/*void*/)> for () {
  fn cacheStatistics_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN5QFont15cacheStatisticsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qfont.h:284
// index:0
// Public Visibility=Default Availability=Available
// [8] QString defaultFamily() const

/*
Returns the family name that corresponds to the current style hint.

See also StyleHint, styleHint(), and setStyleHint().
*/
impl /*struct*/ QFont {
  pub fn defaultFamily_0<RetType, T: QFont_defaultFamily_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.defaultFamily_0(self);
    // return 1;
  }
}
pub trait QFont_defaultFamily_0<RetType> {
  fn defaultFamily_0(self , rsthis: & QFont) -> RetType;
}
impl<'a> /*trait*/ QFont_defaultFamily_0<usize> for () {
  fn defaultFamily_0(self , rsthis: & QFont) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QFont13defaultFamilyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfont.h:285
// index:0
// Public Visibility=Default Availability=Available
// [8] QString lastResortFamily() const

/*
Returns the "last resort" font family name.

The current implementation tries a wide variety of common fonts, returning the first one it finds. Is is possible that no family is found in which case an empty string is returned.

See also lastResortFont().
*/
impl /*struct*/ QFont {
  pub fn lastResortFamily_0<RetType, T: QFont_lastResortFamily_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.lastResortFamily_0(self);
    // return 1;
  }
}
pub trait QFont_lastResortFamily_0<RetType> {
  fn lastResortFamily_0(self , rsthis: & QFont) -> RetType;
}
impl<'a> /*trait*/ QFont_lastResortFamily_0<usize> for () {
  fn lastResortFamily_0(self , rsthis: & QFont) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QFont16lastResortFamilyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfont.h:286
// index:0
// Public Visibility=Default Availability=Available
// [8] QString lastResortFont() const

/*
Returns a "last resort" font name for the font matching algorithm. This is used if the last resort family is not available. It will always return a name, if necessary returning something like "fixed" or "system".

The current implementation tries a wide variety of common fonts, returning the first one it finds. The implementation may change at any time, but this function will always return a string containing something.

It is theoretically possible that there really isn't a lastResortFont() in which case Qt will abort with an error message. We have not been able to identify a case where this happens. Please report it as a bug if it does, preferably with a list of the fonts you have installed.

See also lastResortFamily().
*/
impl /*struct*/ QFont {
  pub fn lastResortFont_0<RetType, T: QFont_lastResortFont_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.lastResortFont_0(self);
    // return 1;
  }
}
pub trait QFont_lastResortFont_0<RetType> {
  fn lastResortFont_0(self , rsthis: & QFont) -> RetType;
}
impl<'a> /*trait*/ QFont_lastResortFont_0<usize> for () {
  fn lastResortFont_0(self , rsthis: & QFont) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QFont14lastResortFontEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfont.h:288
// index:0
// Public Visibility=Default Availability=Available
// [16] QFont resolve(const QFont &) const

/*
Returns a new QFont that has attributes copied from other that have not been previously set on this font.
*/
impl /*struct*/ QFont {
  pub fn resolve_0<RetType, T: QFont_resolve_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resolve_0(self);
    // return 1;
  }
}
pub trait QFont_resolve_0<RetType> {
  fn resolve_0(self , rsthis: & QFont) -> RetType;
}
impl<'a> /*trait*/ QFont_resolve_0<usize> for (usize) {
  fn resolve_0(self , rsthis: & QFont) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QFont7resolveERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfont.h:289
// index:1
// Public inline Visibility=Default Availability=Available
// [4] uint resolve() const

/*
Returns a new QFont that has attributes copied from other that have not been previously set on this font.
*/
impl /*struct*/ QFont {
  pub fn resolve_1<RetType, T: QFont_resolve_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resolve_1(self);
    // return 1;
  }
}
pub trait QFont_resolve_1<RetType> {
  fn resolve_1(self , rsthis: & QFont) -> RetType;
}
impl<'a> /*trait*/ QFont_resolve_1<u32> for () {
  fn resolve_1(self , rsthis: & QFont) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QFont7resolveEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfont.h:290
// index:2
// Public inline Visibility=Default Availability=Available
// [-2] void resolve(uint)

/*
Returns a new QFont that has attributes copied from other that have not been previously set on this font.
*/
impl /*struct*/ QFont {
  pub fn resolve_2<RetType, T: QFont_resolve_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resolve_2(self);
    // return 1;
  }
}
pub trait QFont_resolve_2<RetType> {
  fn resolve_2(self , rsthis: & QFont) -> RetType;
}
impl<'a> /*trait*/ QFont_resolve_2<(/*void*/)> for (u32) {
  fn resolve_2(self , rsthis: & QFont) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const u32 as usize;
     qtrt::InvokeQtFunc6("_ZN5QFont7resolveEj", 1,qtrt::FFITY_UINT32,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}


/*
Style hints are used by the font matching algorithm to find an appropriate default family if a selected font family is not available.

QFont::AnyStyle?leaves the font matching algorithm to choose the family. This is the default.
QFont::SansSerifHelveticathe font matcher prefer sans serif fonts.
QFont::SerifTimesthe font matcher prefers serif fonts.
QFont::Times?is a synonym for Serif.
QFont::TypeWriterCourierthe font matcher prefers fixed pitch fonts.
QFont::Courier?a synonym for TypeWriter.
QFont::OldEnglish?the font matcher prefers decorative fonts.
QFont::DecorativeOldEnglishis a synonym for OldEnglish.
QFont::Monospace?the font matcher prefers fonts that map to the CSS generic font-family 'monospace'.
QFont::Fantasy?the font matcher prefers fonts that map to the CSS generic font-family 'fantasy'.
QFont::Cursive?the font matcher prefers fonts that map to the CSS generic font-family 'cursive'.
QFont::System?the font matcher prefers system fonts.

*/
pub type QFont__StyleHint = i32;
// is a synonym for SansSerif.
pub const QFont__Helvetica :QFont__StyleHint = 0;
// 
pub const QFont__SansSerif :QFont__StyleHint = 0;
// 
pub const QFont__Times :QFont__StyleHint = 1;
// 
pub const QFont__Serif :QFont__StyleHint = 1;
// 
pub const QFont__Courier :QFont__StyleHint = 2;
// 
pub const QFont__TypeWriter :QFont__StyleHint = 2;
// 
pub const QFont__OldEnglish :QFont__StyleHint = 3;
// 
pub const QFont__Decorative :QFont__StyleHint = 3;
// 
pub const QFont__System :QFont__StyleHint = 4;
// 
pub const QFont__AnyStyle :QFont__StyleHint = 5;
// 
pub const QFont__Cursive :QFont__StyleHint = 6;
// 
pub const QFont__Monospace :QFont__StyleHint = 7;
// 
pub const QFont__Fantasy :QFont__StyleHint = 8;
pub fn QFont_StyleHintItemName(val: i32) ->String {
  match val {
     QFont__Helvetica => // 0
     {return String::from("Helvetica,SansSerif");}
    // QFont__SansSerif => // 0
    // {return String::from("");}
     QFont__Times => // 1
     {return String::from("Times,Serif");}
    // QFont__Serif => // 1
    // {return String::from("");}
     QFont__Courier => // 2
     {return String::from("Courier,TypeWriter");}
    // QFont__TypeWriter => // 2
    // {return String::from("");}
     QFont__OldEnglish => // 3
     {return String::from("OldEnglish,Decorative");}
    // QFont__Decorative => // 3
    // {return String::from("");}
     QFont__System => // 4
     {return String::from("System");}
     QFont__AnyStyle => // 5
     {return String::from("AnyStyle");}
     QFont__Cursive => // 6
     {return String::from("Cursive");}
     QFont__Monospace => // 7
     {return String::from("Monospace");}
     QFont__Fantasy => // 8
     {return String::from("Fantasy");}
  _ => {return format!("{}", val);}
}
}
pub fn QFont_StyleHintItemName_s(val: i32) ->String {
  //var nilthis *QFont
  //return nilthis.StyleHintItemName(val);
  return QFont_StyleHintItemName(val);
}


/*
The style strategy tells the font matching algorithm what type of fonts should be used to find an appropriate default family.

The following strategies are available:



Any of these may be OR-ed with one of these flags:


*/
pub type QFont__StyleStrategy = i32;
// 
pub const QFont__PreferDefault :QFont__StyleStrategy = 1;
// 
pub const QFont__PreferBitmap :QFont__StyleStrategy = 2;
// 
pub const QFont__PreferDevice :QFont__StyleStrategy = 4;
// 
pub const QFont__PreferOutline :QFont__StyleStrategy = 8;
// 
pub const QFont__ForceOutline :QFont__StyleStrategy = 16;
// 
pub const QFont__PreferMatch :QFont__StyleStrategy = 32;
// 
pub const QFont__PreferQuality :QFont__StyleStrategy = 64;
// 
pub const QFont__PreferAntialias :QFont__StyleStrategy = 128;
// 
pub const QFont__NoAntialias :QFont__StyleStrategy = 256;
// 
pub const QFont__OpenGLCompatible :QFont__StyleStrategy = 512;
// 
pub const QFont__ForceIntegerMetrics :QFont__StyleStrategy = 1024;
// 
pub const QFont__NoSubpixelAntialias :QFont__StyleStrategy = 2048;
// 
pub const QFont__PreferNoShaping :QFont__StyleStrategy = 4096;
// 
pub const QFont__NoFontMerging :QFont__StyleStrategy = 32768;
pub fn QFont_StyleStrategyItemName(val: i32) ->String {
  match val {
     QFont__PreferDefault => // 1
     {return String::from("PreferDefault");}
     QFont__PreferBitmap => // 2
     {return String::from("PreferBitmap");}
     QFont__PreferDevice => // 4
     {return String::from("PreferDevice");}
     QFont__PreferOutline => // 8
     {return String::from("PreferOutline");}
     QFont__ForceOutline => // 16
     {return String::from("ForceOutline");}
     QFont__PreferMatch => // 32
     {return String::from("PreferMatch");}
     QFont__PreferQuality => // 64
     {return String::from("PreferQuality");}
     QFont__PreferAntialias => // 128
     {return String::from("PreferAntialias");}
     QFont__NoAntialias => // 256
     {return String::from("NoAntialias");}
     QFont__OpenGLCompatible => // 512
     {return String::from("OpenGLCompatible");}
     QFont__ForceIntegerMetrics => // 1024
     {return String::from("ForceIntegerMetrics");}
     QFont__NoSubpixelAntialias => // 2048
     {return String::from("NoSubpixelAntialias");}
     QFont__PreferNoShaping => // 4096
     {return String::from("PreferNoShaping");}
     QFont__NoFontMerging => // 32768
     {return String::from("NoFontMerging");}
  _ => {return format!("{}", val);}
}
}
pub fn QFont_StyleStrategyItemName_s(val: i32) ->String {
  //var nilthis *QFont
  //return nilthis.StyleStrategyItemName(val);
  return QFont_StyleStrategyItemName(val);
}


/*
This enum describes the different levels of hinting that can be applied to glyphs to improve legibility on displays where it might be warranted by the density of pixels.



Please note that this enum only describes a preference, as the full range of hinting levels are not supported on all of Qt's supported platforms. The following table details the effect of a given hinting preference on a selected set of target platforms.


 PreferDefaultHintingPreferNoHintingPreferVerticalHintingPreferFullHinting
Windows Vista (w/o Platform Update) and earlierFull hintingFull hintingFull hintingFull hinting
Windows 7 and Windows Vista (w/Platform Update) and DirectWrite enabled in QtFull hintingVertical hintingVertical hintingFull hinting
FreeTypeOperating System settingNo hintingVertical hinting (light)Full hinting
Cocoa on macOSNo hintingNo hintingNo hintingNo hinting


Note: Please be aware that altering the hinting preference on Windows is available through the DirectWrite font engine. This is available on Windows Vista after installing the platform update, and on Windows 7. In order to use this extension, configure Qt using -directwrite. The target application will then depend on the availability of DirectWrite on the target system.

This enum was introduced or modified in  Qt 4.8.

*/
pub type QFont__HintingPreference = i32;
// Use the default hinting level for the target platform.
pub const QFont__PreferDefaultHinting :QFont__HintingPreference = 0;
// If possible, render text without hinting the outlines of the glyphs. The text layout will be typographically accurate and scalable, using the same metrics as are used e.g. when printing.
pub const QFont__PreferNoHinting :QFont__HintingPreference = 1;
// If possible, render text with no horizontal hinting, but align glyphs to the pixel grid in the vertical direction. The text will appear crisper on displays where the density is too low to give an accurate rendering of the glyphs. But since the horizontal metrics of the glyphs are unhinted, the text's layout will be scalable to higher density devices (such as printers) without impacting details such as line breaks.
pub const QFont__PreferVerticalHinting :QFont__HintingPreference = 2;
// If possible, render text with hinting in both horizontal and vertical directions. The text will be altered to optimize legibility on the target device, but since the metrics will depend on the target size of the text, the positions of glyphs, line breaks, and other typographical detail will not scale, meaning that a text layout may look different on devices with different pixel densities.
pub const QFont__PreferFullHinting :QFont__HintingPreference = 3;
pub fn QFont_HintingPreferenceItemName(val: i32) ->String {
  match val {
     QFont__PreferDefaultHinting => // 0
     {return String::from("PreferDefaultHinting");}
     QFont__PreferNoHinting => // 1
     {return String::from("PreferNoHinting");}
     QFont__PreferVerticalHinting => // 2
     {return String::from("PreferVerticalHinting");}
     QFont__PreferFullHinting => // 3
     {return String::from("PreferFullHinting");}
  _ => {return format!("{}", val);}
}
}
pub fn QFont_HintingPreferenceItemName_s(val: i32) ->String {
  //var nilthis *QFont
  //return nilthis.HintingPreferenceItemName(val);
  return QFont_HintingPreferenceItemName(val);
}


/*
Qt uses a weighting scale from 0 to 99 similar to, but not the same as, the scales used in Windows or CSS. A weight of 0 will be thin, whilst 99 will be extremely black.

This enum contains the predefined font weights:


*/
pub type QFont__Weight = i32;
// 0
pub const QFont__Thin :QFont__Weight = 0;
// 
pub const QFont__ExtraLight :QFont__Weight = 12;
// 
pub const QFont__Light :QFont__Weight = 25;
// 
pub const QFont__Normal :QFont__Weight = 50;
// 
pub const QFont__Medium :QFont__Weight = 57;
// 
pub const QFont__DemiBold :QFont__Weight = 63;
// 
pub const QFont__Bold :QFont__Weight = 75;
// 
pub const QFont__ExtraBold :QFont__Weight = 81;
// 
pub const QFont__Black :QFont__Weight = 87;
pub fn QFont_WeightItemName(val: i32) ->String {
  match val {
     QFont__Thin => // 0
     {return String::from("Thin");}
     QFont__ExtraLight => // 12
     {return String::from("ExtraLight");}
     QFont__Light => // 25
     {return String::from("Light");}
     QFont__Normal => // 50
     {return String::from("Normal");}
     QFont__Medium => // 57
     {return String::from("Medium");}
     QFont__DemiBold => // 63
     {return String::from("DemiBold");}
     QFont__Bold => // 75
     {return String::from("Bold");}
     QFont__ExtraBold => // 81
     {return String::from("ExtraBold");}
     QFont__Black => // 87
     {return String::from("Black");}
  _ => {return format!("{}", val);}
}
}
pub fn QFont_WeightItemName_s(val: i32) ->String {
  //var nilthis *QFont
  //return nilthis.WeightItemName(val);
  return QFont_WeightItemName(val);
}


/*
This enum describes the different styles of glyphs that are used to display text.



See also Weight.

*/
pub type QFont__Style = i32;
// Normal glyphs used in unstyled text.
pub const QFont__StyleNormal :QFont__Style = 0;
// Italic glyphs that are specifically designed for the purpose of representing italicized text.
pub const QFont__StyleItalic :QFont__Style = 1;
// Glyphs with an italic appearance that are typically based on the unstyled glyphs, but are not fine-tuned for the purpose of representing italicized text.
pub const QFont__StyleOblique :QFont__Style = 2;
pub fn QFont_StyleItemName(val: i32) ->String {
  match val {
     QFont__StyleNormal => // 0
     {return String::from("StyleNormal");}
     QFont__StyleItalic => // 1
     {return String::from("StyleItalic");}
     QFont__StyleOblique => // 2
     {return String::from("StyleOblique");}
  _ => {return format!("{}", val);}
}
}
pub fn QFont_StyleItemName_s(val: i32) ->String {
  //var nilthis *QFont
  //return nilthis.StyleItemName(val);
  return QFont_StyleItemName(val);
}


/*
Predefined stretch values that follow the CSS naming convention. The higher the value, the more stretched the text is.



See also setStretch() and stretch().

*/
pub type QFont__Stretch = i32;
// 
pub const QFont__AnyStretch :QFont__Stretch = 0;
// 
pub const QFont__UltraCondensed :QFont__Stretch = 50;
// 
pub const QFont__ExtraCondensed :QFont__Stretch = 62;
// 
pub const QFont__Condensed :QFont__Stretch = 75;
// 
pub const QFont__SemiCondensed :QFont__Stretch = 87;
// 
pub const QFont__Unstretched :QFont__Stretch = 100;
// 
pub const QFont__SemiExpanded :QFont__Stretch = 112;
// 
pub const QFont__Expanded :QFont__Stretch = 125;
// 
pub const QFont__ExtraExpanded :QFont__Stretch = 150;
// 
pub const QFont__UltraExpanded :QFont__Stretch = 200;
pub fn QFont_StretchItemName(val: i32) ->String {
  match val {
     QFont__AnyStretch => // 0
     {return String::from("AnyStretch");}
     QFont__UltraCondensed => // 50
     {return String::from("UltraCondensed");}
     QFont__ExtraCondensed => // 62
     {return String::from("ExtraCondensed");}
     QFont__Condensed => // 75
     {return String::from("Condensed");}
     QFont__SemiCondensed => // 87
     {return String::from("SemiCondensed");}
     QFont__Unstretched => // 100
     {return String::from("Unstretched");}
     QFont__SemiExpanded => // 112
     {return String::from("SemiExpanded");}
     QFont__Expanded => // 125
     {return String::from("Expanded");}
     QFont__ExtraExpanded => // 150
     {return String::from("ExtraExpanded");}
     QFont__UltraExpanded => // 200
     {return String::from("UltraExpanded");}
  _ => {return format!("{}", val);}
}
}
pub fn QFont_StretchItemName_s(val: i32) ->String {
  //var nilthis *QFont
  //return nilthis.StretchItemName(val);
  return QFont_StretchItemName(val);
}


/*
Rendering option for text this font applies to.



This enum was introduced or modified in  Qt 4.4.

*/
pub type QFont__Capitalization = i32;
// This is the normal text rendering option where no capitalization change is applied.
pub const QFont__MixedCase :QFont__Capitalization = 0;
// This alters the text to be rendered in all uppercase type.
pub const QFont__AllUppercase :QFont__Capitalization = 1;
// This alters the text to be rendered in all lowercase type.
pub const QFont__AllLowercase :QFont__Capitalization = 2;
// This alters the text to be rendered in small-caps type.
pub const QFont__SmallCaps :QFont__Capitalization = 3;
// This alters the text to be rendered with the first character of each word as an uppercase character.
pub const QFont__Capitalize :QFont__Capitalization = 4;
pub fn QFont_CapitalizationItemName(val: i32) ->String {
  match val {
     QFont__MixedCase => // 0
     {return String::from("MixedCase");}
     QFont__AllUppercase => // 1
     {return String::from("AllUppercase");}
     QFont__AllLowercase => // 2
     {return String::from("AllLowercase");}
     QFont__SmallCaps => // 3
     {return String::from("SmallCaps");}
     QFont__Capitalize => // 4
     {return String::from("Capitalize");}
  _ => {return format!("{}", val);}
}
}
pub fn QFont_CapitalizationItemName_s(val: i32) ->String {
  //var nilthis *QFont
  //return nilthis.CapitalizationItemName(val);
  return QFont_CapitalizationItemName(val);
}


/*


This enum was introduced or modified in  Qt 4.4.

*/
pub type QFont__SpacingType = i32;
// 
pub const QFont__PercentageSpacing :QFont__SpacingType = 0;
// A positive value increases the letter spacing by the corresponding pixels; a negative value decreases the spacing.
pub const QFont__AbsoluteSpacing :QFont__SpacingType = 1;
pub fn QFont_SpacingTypeItemName(val: i32) ->String {
  match val {
     QFont__PercentageSpacing => // 0
     {return String::from("PercentageSpacing");}
     QFont__AbsoluteSpacing => // 1
     {return String::from("AbsoluteSpacing");}
  _ => {return format!("{}", val);}
}
}
pub fn QFont_SpacingTypeItemName_s(val: i32) ->String {
  //var nilthis *QFont
  //return nilthis.SpacingTypeItemName(val);
  return QFont_SpacingTypeItemName(val);
}


/*


*/
pub type QFont__ResolveProperties = i32;
// 
pub const QFont__FamilyResolved :QFont__ResolveProperties = 1;
// 
pub const QFont__SizeResolved :QFont__ResolveProperties = 2;
// 
pub const QFont__StyleHintResolved :QFont__ResolveProperties = 4;
// 
pub const QFont__StyleStrategyResolved :QFont__ResolveProperties = 8;
// 
pub const QFont__WeightResolved :QFont__ResolveProperties = 16;
// 
pub const QFont__StyleResolved :QFont__ResolveProperties = 32;
// 
pub const QFont__UnderlineResolved :QFont__ResolveProperties = 64;
// 
pub const QFont__OverlineResolved :QFont__ResolveProperties = 128;
// 
pub const QFont__StrikeOutResolved :QFont__ResolveProperties = 256;
// 
pub const QFont__FixedPitchResolved :QFont__ResolveProperties = 512;
// 
pub const QFont__StretchResolved :QFont__ResolveProperties = 1024;
// 
pub const QFont__KerningResolved :QFont__ResolveProperties = 2048;
// 
pub const QFont__CapitalizationResolved :QFont__ResolveProperties = 4096;
// 
pub const QFont__LetterSpacingResolved :QFont__ResolveProperties = 8192;
// 
pub const QFont__WordSpacingResolved :QFont__ResolveProperties = 16384;
// 
pub const QFont__HintingPreferenceResolved :QFont__ResolveProperties = 32768;
// 
pub const QFont__StyleNameResolved :QFont__ResolveProperties = 65536;
// 
pub const QFont__AllPropertiesResolved :QFont__ResolveProperties = 131071;
pub fn QFont_ResolvePropertiesItemName(val: i32) ->String {
  match val {
     QFont__FamilyResolved => // 1
     {return String::from("FamilyResolved");}
     QFont__SizeResolved => // 2
     {return String::from("SizeResolved");}
     QFont__StyleHintResolved => // 4
     {return String::from("StyleHintResolved");}
     QFont__StyleStrategyResolved => // 8
     {return String::from("StyleStrategyResolved");}
     QFont__WeightResolved => // 16
     {return String::from("WeightResolved");}
     QFont__StyleResolved => // 32
     {return String::from("StyleResolved");}
     QFont__UnderlineResolved => // 64
     {return String::from("UnderlineResolved");}
     QFont__OverlineResolved => // 128
     {return String::from("OverlineResolved");}
     QFont__StrikeOutResolved => // 256
     {return String::from("StrikeOutResolved");}
     QFont__FixedPitchResolved => // 512
     {return String::from("FixedPitchResolved");}
     QFont__StretchResolved => // 1024
     {return String::from("StretchResolved");}
     QFont__KerningResolved => // 2048
     {return String::from("KerningResolved");}
     QFont__CapitalizationResolved => // 4096
     {return String::from("CapitalizationResolved");}
     QFont__LetterSpacingResolved => // 8192
     {return String::from("LetterSpacingResolved");}
     QFont__WordSpacingResolved => // 16384
     {return String::from("WordSpacingResolved");}
     QFont__HintingPreferenceResolved => // 32768
     {return String::from("HintingPreferenceResolved");}
     QFont__StyleNameResolved => // 65536
     {return String::from("StyleNameResolved");}
     QFont__AllPropertiesResolved => // 131071
     {return String::from("AllPropertiesResolved");}
  _ => {return format!("{}", val);}
}
}
pub fn QFont_ResolvePropertiesItemName_s(val: i32) ->String {
  //var nilthis *QFont
  //return nilthis.ResolvePropertiesItemName(val);
  return QFont_ResolvePropertiesItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
