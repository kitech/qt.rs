

// mod ::gui::QRawFont
// package qtgui
// /usr/include/qt/QtGui/qrawfont.h
// #include <qrawfont.h>
// #include <QtGui>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 24
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
#[derive(Default)] // class sizeof(QRawFont)=8
pub struct QRawFont {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QRawFont_ITF interface {
//    QRawFont_PTR() *QRawFont
//}
//func (ptr *QRawFont) QRawFont_PTR() *QRawFont { return ptr }

impl /*struct*/ QRawFont {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QRawFont {
    return QRawFont{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QRawFont {
//  type Target = QRawFontBASE;
//
//  fn deref(&self) -> &QRawFontBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QRawFontBASE> for QRawFont {
//  fn as_ref(& self) -> & QRawFontBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qrawfont.h:74
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QRawFont()

/*
Constructs an invalid QRawFont.
*/
// QRawFont() ctx.fn_proto_cpp
impl /*struct*/ QRawFont {
  pub fn QRawFont_0<T: QRawFont_QRawFont_0>(value: T) -> QRawFont {
    let rsthis = value.QRawFont_0();
    return rsthis;
    // return 1;
  }
}

pub trait QRawFont_QRawFont_0 {
  fn QRawFont_0(self) -> QRawFont;
}
// QRawFont() ctx.fn_proto_cpp
impl<'a> /*trait*/ QRawFont_QRawFont_0 for () {
  fn QRawFont_0(self) -> QRawFont {
    // unsafe{_ZN8QRawFontC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN8QRawFontC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QRawFont{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qrawfont.h:75
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QRawFont(const QString &, qreal, QFont::HintingPreference)

/*
Constructs an invalid QRawFont.
*/
// QRawFont(const QString &, qreal, QFont::HintingPreference) ctx.fn_proto_cpp
impl /*struct*/ QRawFont {
  pub fn QRawFont_1<T: QRawFont_QRawFont_1>(value: T) -> QRawFont {
    let rsthis = value.QRawFont_1();
    return rsthis;
    // return 1;
  }
}

pub trait QRawFont_QRawFont_1 {
  fn QRawFont_1(self) -> QRawFont;
}
// QRawFont(const QString &, qreal, QFont::HintingPreference) ctx.fn_proto_cpp
impl<'a> /*trait*/ QRawFont_QRawFont_1 for (usize,f64,i32) {
  fn QRawFont_1(self) -> QRawFont {
    // unsafe{_ZN8QRawFontC2ERK7QStringdN5QFont17HintingPreferenceE()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN8QRawFontC2ERK7QStringdN5QFont17HintingPreferenceE", 3,qtrt::FFITY_POINTER,qtrt::FFITY_DOUBLE,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QRawFont{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qrawfont.h:78
// index:2
// Public Visibility=Default Availability=Available
// [-2] void QRawFont(const QByteArray &, qreal, QFont::HintingPreference)

/*
Constructs an invalid QRawFont.
*/
// QRawFont(const QByteArray &, qreal, QFont::HintingPreference) ctx.fn_proto_cpp
impl /*struct*/ QRawFont {
  pub fn QRawFont_2<T: QRawFont_QRawFont_2>(value: T) -> QRawFont {
    let rsthis = value.QRawFont_2();
    return rsthis;
    // return 1;
  }
}

pub trait QRawFont_QRawFont_2 {
  fn QRawFont_2(self) -> QRawFont;
}
// QRawFont(const QByteArray &, qreal, QFont::HintingPreference) ctx.fn_proto_cpp
impl<'a> /*trait*/ QRawFont_QRawFont_2 for (usize,f64,i32) {
  fn QRawFont_2(self) -> QRawFont {
    // unsafe{_ZN8QRawFontC2ERK10QByteArraydN5QFont17HintingPreferenceE()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN8QRawFontC2ERK10QByteArraydN5QFont17HintingPreferenceE", 3,qtrt::FFITY_POINTER,qtrt::FFITY_DOUBLE,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QRawFont{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qrawfont.h:83
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QRawFont & operator=(QRawFont &&)

/*

*/
impl /*struct*/ QRawFont {
  pub fn operator_equal_0<RetType, T: QRawFont_operator_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_0(self);
    // return 1;
  }
}
pub trait QRawFont_operator_equal_0<RetType> {
  fn operator_equal_0(self , rsthis: & QRawFont) -> RetType;
}
impl<'a> /*trait*/ QRawFont_operator_equal_0<usize> for (usize) {
  fn operator_equal_0(self , rsthis: & QRawFont) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QRawFontaSEOS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qrawfont.h:85
// index:1
// Public Visibility=Default Availability=Available
// [8] QRawFont & operator=(const QRawFont &)

/*

*/
impl /*struct*/ QRawFont {
  pub fn operator_equal_1<RetType, T: QRawFont_operator_equal_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_1(self);
    // return 1;
  }
}
pub trait QRawFont_operator_equal_1<RetType> {
  fn operator_equal_1(self , rsthis: & QRawFont) -> RetType;
}
impl<'a> /*trait*/ QRawFont_operator_equal_1<usize> for (usize) {
  fn operator_equal_1(self , rsthis: & QRawFont) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QRawFontaSERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qrawfont.h:86
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ~QRawFont()

/*

*/
pub fn DeleteQRawFont(this :*mut QRawFont) {
    // let rv = qtrt::InvokeQtFunc6("_ZN8QRawFontD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 8)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qrawfont.h:88
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void swap(QRawFont &)

/*
Swaps this raw font with other. This function is very fast and never fails.

This function was introduced in  Qt 5.0.
*/
impl /*struct*/ QRawFont {
  pub fn swap_0<RetType, T: QRawFont_swap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.swap_0(self);
    // return 1;
  }
}
pub trait QRawFont_swap_0<RetType> {
  fn swap_0(self , rsthis: & QRawFont) -> RetType;
}
impl<'a> /*trait*/ QRawFont_swap_0<(/*void*/)> for (usize) {
  fn swap_0(self , rsthis: & QRawFont) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QRawFont4swapERS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qrawfont.h:90
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isValid() const

/*
Returns true if the QRawFont is valid and false otherwise.
*/
impl /*struct*/ QRawFont {
  pub fn isValid_0<RetType, T: QRawFont_isValid_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isValid_0(self);
    // return 1;
  }
}
pub trait QRawFont_isValid_0<RetType> {
  fn isValid_0(self , rsthis: & QRawFont) -> RetType;
}
impl<'a> /*trait*/ QRawFont_isValid_0<bool> for () {
  fn isValid_0(self , rsthis: & QRawFont) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QRawFont7isValidEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qrawfont.h:92
// index:0
// Public Visibility=Default Availability=Available
// [1] bool operator==(const QRawFont &) const

/*

*/
impl /*struct*/ QRawFont {
  pub fn operator_equal_equal_0<RetType, T: QRawFont_operator_equal_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_equal_0(self);
    // return 1;
  }
}
pub trait QRawFont_operator_equal_equal_0<RetType> {
  fn operator_equal_equal_0(self , rsthis: & QRawFont) -> RetType;
}
impl<'a> /*trait*/ QRawFont_operator_equal_equal_0<bool> for (usize) {
  fn operator_equal_equal_0(self , rsthis: & QRawFont) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QRawFonteqERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qrawfont.h:93
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator!=(const QRawFont &) const

/*

*/
impl /*struct*/ QRawFont {
  pub fn operator_not_equal_0<RetType, T: QRawFont_operator_not_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_not_equal_0(self);
    // return 1;
  }
}
pub trait QRawFont_operator_not_equal_0<RetType> {
  fn operator_not_equal_0(self , rsthis: & QRawFont) -> RetType;
}
impl<'a> /*trait*/ QRawFont_operator_not_equal_0<bool> for (usize) {
  fn operator_not_equal_0(self , rsthis: & QRawFont) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QRawFontneERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qrawfont.h:96
// index:0
// Public Visibility=Default Availability=Available
// [8] QString familyName() const

/*
Returns the family name of this QRawFont.
*/
impl /*struct*/ QRawFont {
  pub fn familyName_0<RetType, T: QRawFont_familyName_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.familyName_0(self);
    // return 1;
  }
}
pub trait QRawFont_familyName_0<RetType> {
  fn familyName_0(self , rsthis: & QRawFont) -> RetType;
}
impl<'a> /*trait*/ QRawFont_familyName_0<usize> for () {
  fn familyName_0(self , rsthis: & QRawFont) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QRawFont10familyNameEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qrawfont.h:97
// index:0
// Public Visibility=Default Availability=Available
// [8] QString styleName() const

/*
Returns the style name of this QRawFont.

See also QFont::styleName().
*/
impl /*struct*/ QRawFont {
  pub fn styleName_0<RetType, T: QRawFont_styleName_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.styleName_0(self);
    // return 1;
  }
}
pub trait QRawFont_styleName_0<RetType> {
  fn styleName_0(self , rsthis: & QRawFont) -> RetType;
}
impl<'a> /*trait*/ QRawFont_styleName_0<usize> for () {
  fn styleName_0(self , rsthis: & QRawFont) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QRawFont9styleNameEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qrawfont.h:99
// index:0
// Public Visibility=Default Availability=Available
// [4] QFont::Style style() const

/*
Returns the style of this QRawFont.

See also QFont::style().
*/
impl /*struct*/ QRawFont {
  pub fn style_0<RetType, T: QRawFont_style_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.style_0(self);
    // return 1;
  }
}
pub trait QRawFont_style_0<RetType> {
  fn style_0(self , rsthis: & QRawFont) -> RetType;
}
impl<'a> /*trait*/ QRawFont_style_0<i32> for () {
  fn style_0(self , rsthis: & QRawFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QRawFont5styleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qrawfont.h:100
// index:0
// Public Visibility=Default Availability=Available
// [4] int weight() const

/*
Returns the weight of this QRawFont.

See also QFont::weight().
*/
impl /*struct*/ QRawFont {
  pub fn weight_0<RetType, T: QRawFont_weight_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.weight_0(self);
    // return 1;
  }
}
pub trait QRawFont_weight_0<RetType> {
  fn weight_0(self , rsthis: & QRawFont) -> RetType;
}
impl<'a> /*trait*/ QRawFont_weight_0<i32> for () {
  fn weight_0(self , rsthis: & QRawFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QRawFont6weightEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qrawfont.h:105
// index:0
// Public Visibility=Default Availability=Available
// [1] bool glyphIndexesForChars(const QChar *, int, quint32 *, int *) const

/*
Converts a string of unicode points to glyph indexes using the CMAP table in the underlying font. The function works like glyphIndexesForString() except it take an array (chars), the results will be returned though glyphIndexes array and number of glyphs will be set in numGlyphs. The size of glyphIndexes array must be at least numChars, if that's still not enough, this function will return false, then you can resize glyphIndexes from the size returned in numGlyphs.

See also glyphIndexesForString(), advancesForGlyphIndexes(), QGlyphRun, QTextLayout::glyphRuns(), and QTextFragment::glyphRuns().
*/
impl /*struct*/ QRawFont {
  pub fn glyphIndexesForChars_0<RetType, T: QRawFont_glyphIndexesForChars_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.glyphIndexesForChars_0(self);
    // return 1;
  }
}
pub trait QRawFont_glyphIndexesForChars_0<RetType> {
  fn glyphIndexesForChars_0(self , rsthis: & QRawFont) -> RetType;
}
impl<'a> /*trait*/ QRawFont_glyphIndexesForChars_0<bool> for (usize,i32,usize,usize) {
  fn glyphIndexesForChars_0(self , rsthis: & QRawFont) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const usize as usize;
    let arg3 = (&self.3) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QRawFont20glyphIndexesForCharsEPK5QChariPjPi", 4,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qrawfont.h:106
// index:0
// Public Visibility=Default Availability=Available
// [1] bool advancesForGlyphIndexes(const quint32 *, QPointF *, int) const

/*
Returns the QRawFont's advances for each of the glyphIndexes in pixel units. The advances give the distance from the position of a given glyph to where the next glyph should be drawn to make it appear as if the two glyphs are unspaced. How the advances are calculated is controlled by layoutFlags.

This function was introduced in  Qt 5.1.

See also QTextLine::horizontalAdvance() and QFontMetricsF::width().
*/
impl /*struct*/ QRawFont {
  pub fn advancesForGlyphIndexes_0<RetType, T: QRawFont_advancesForGlyphIndexes_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.advancesForGlyphIndexes_0(self);
    // return 1;
  }
}
pub trait QRawFont_advancesForGlyphIndexes_0<RetType> {
  fn advancesForGlyphIndexes_0(self , rsthis: & QRawFont) -> RetType;
}
impl<'a> /*trait*/ QRawFont_advancesForGlyphIndexes_0<bool> for (usize,usize,i32) {
  fn advancesForGlyphIndexes_0(self , rsthis: & QRawFont) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QRawFont23advancesForGlyphIndexesEPKjP7QPointFi", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qrawfont.h:107
// index:1
// Public Visibility=Default Availability=Available
// [1] bool advancesForGlyphIndexes(const quint32 *, QPointF *, int, QRawFont::LayoutFlags) const

/*
Returns the QRawFont's advances for each of the glyphIndexes in pixel units. The advances give the distance from the position of a given glyph to where the next glyph should be drawn to make it appear as if the two glyphs are unspaced. How the advances are calculated is controlled by layoutFlags.

This function was introduced in  Qt 5.1.

See also QTextLine::horizontalAdvance() and QFontMetricsF::width().
*/
impl /*struct*/ QRawFont {
  pub fn advancesForGlyphIndexes_1<RetType, T: QRawFont_advancesForGlyphIndexes_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.advancesForGlyphIndexes_1(self);
    // return 1;
  }
}
pub trait QRawFont_advancesForGlyphIndexes_1<RetType> {
  fn advancesForGlyphIndexes_1(self , rsthis: & QRawFont) -> RetType;
}
impl<'a> /*trait*/ QRawFont_advancesForGlyphIndexes_1<bool> for (usize,usize,i32,i32) {
  fn advancesForGlyphIndexes_1(self , rsthis: & QRawFont) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QRawFont23advancesForGlyphIndexesEPKjP7QPointFi6QFlagsINS_10LayoutFlagEE", 4,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qrawfont.h:109
// index:0
// Public Visibility=Default Availability=Available
// [32] QImage alphaMapForGlyph(quint32, QRawFont::AntialiasingType, const QTransform &) const

/*
This function returns a rasterized image of the glyph at the given glyphIndex in the underlying font, using the transform specified. If the QRawFont is not valid, this function will return an invalid QImage.

If the font is a color font, then the resulting image will contain the rendered glyph at the current pixel size. In this case, the antialiasingType will be ignored.

Otherwise, if antialiasingType is set to QRawFont::SubPixelAntialiasing, then the resulting image will be in QImage::Format_RGB32 and the RGB values of each pixel will represent the subpixel opacities of the pixel in the rasterization of the glyph. Otherwise, the image will be in the format of QImage::Format_Indexed8 and each pixel will contain the opacity of the pixel in the rasterization.

See also pathForGlyph() and QPainter::drawGlyphRun().
*/
impl /*struct*/ QRawFont {
  pub fn alphaMapForGlyph_0<RetType, T: QRawFont_alphaMapForGlyph_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.alphaMapForGlyph_0(self);
    // return 1;
  }
}
pub trait QRawFont_alphaMapForGlyph_0<RetType> {
  fn alphaMapForGlyph_0(self , rsthis: & QRawFont) -> RetType;
}
impl<'a> /*trait*/ QRawFont_alphaMapForGlyph_0<usize> for (u32,i32,usize) {
  fn alphaMapForGlyph_0(self , rsthis: & QRawFont) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const u32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QRawFont16alphaMapForGlyphEjNS_16AntialiasingTypeERK10QTransform", 3,qtrt::FFITY_UINT32,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qrawfont.h:112
// index:0
// Public Visibility=Default Availability=Available
// [8] QPainterPath pathForGlyph(quint32) const

/*
This function returns the shape of the glyph at a given glyphIndex in the underlying font if the QRawFont is valid. Otherwise, it returns an empty QPainterPath.

The returned glyph will always be unhinted.

See also alphaMapForGlyph() and QPainterPath::addText().
*/
impl /*struct*/ QRawFont {
  pub fn pathForGlyph_0<RetType, T: QRawFont_pathForGlyph_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.pathForGlyph_0(self);
    // return 1;
  }
}
pub trait QRawFont_pathForGlyph_0<RetType> {
  fn pathForGlyph_0(self , rsthis: & QRawFont) -> RetType;
}
impl<'a> /*trait*/ QRawFont_pathForGlyph_0<usize> for (u32) {
  fn pathForGlyph_0(self , rsthis: & QRawFont) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const u32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QRawFont12pathForGlyphEj", 1,qtrt::FFITY_UINT32,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qrawfont.h:113
// index:0
// Public Visibility=Default Availability=Available
// [32] QRectF boundingRect(quint32) const

/*
Returns the smallest rectangle containing the glyph with the given glyphIndex.

This function was introduced in  Qt 5.0.
*/
impl /*struct*/ QRawFont {
  pub fn boundingRect_0<RetType, T: QRawFont_boundingRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.boundingRect_0(self);
    // return 1;
  }
}
pub trait QRawFont_boundingRect_0<RetType> {
  fn boundingRect_0(self , rsthis: & QRawFont) -> RetType;
}
impl<'a> /*trait*/ QRawFont_boundingRect_0<usize> for (u32) {
  fn boundingRect_0(self , rsthis: & QRawFont) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const u32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QRawFont12boundingRectEj", 1,qtrt::FFITY_UINT32,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qrawfont.h:115
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setPixelSize(qreal)

/*
Sets the pixel size with which this font should be rendered to pixelSize.

See also pixelSize().
*/
impl /*struct*/ QRawFont {
  pub fn setPixelSize_0<RetType, T: QRawFont_setPixelSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPixelSize_0(self);
    // return 1;
  }
}
pub trait QRawFont_setPixelSize_0<RetType> {
  fn setPixelSize_0(self , rsthis: & QRawFont) -> RetType;
}
impl<'a> /*trait*/ QRawFont_setPixelSize_0<(/*void*/)> for (f64) {
  fn setPixelSize_0(self , rsthis: & QRawFont) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN8QRawFont12setPixelSizeEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qrawfont.h:116
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal pixelSize() const

/*
Returns the pixel size set for this QRawFont. The pixel size affects how glyphs are rasterized, the size of glyphs returned by pathForGlyph(), and is used to convert internal metrics from design units to logical pixel units.

See also setPixelSize().
*/
impl /*struct*/ QRawFont {
  pub fn pixelSize_0<RetType, T: QRawFont_pixelSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.pixelSize_0(self);
    // return 1;
  }
}
pub trait QRawFont_pixelSize_0<RetType> {
  fn pixelSize_0(self , rsthis: & QRawFont) -> RetType;
}
impl<'a> /*trait*/ QRawFont_pixelSize_0<f64> for () {
  fn pixelSize_0(self , rsthis: & QRawFont) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QRawFont9pixelSizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qrawfont.h:118
// index:0
// Public Visibility=Default Availability=Available
// [4] QFont::HintingPreference hintingPreference() const

/*
Returns the hinting preference used to construct this QRawFont.

See also QFont::hintingPreference().
*/
impl /*struct*/ QRawFont {
  pub fn hintingPreference_0<RetType, T: QRawFont_hintingPreference_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hintingPreference_0(self);
    // return 1;
  }
}
pub trait QRawFont_hintingPreference_0<RetType> {
  fn hintingPreference_0(self , rsthis: & QRawFont) -> RetType;
}
impl<'a> /*trait*/ QRawFont_hintingPreference_0<i32> for () {
  fn hintingPreference_0(self , rsthis: & QRawFont) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QRawFont17hintingPreferenceEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qrawfont.h:120
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal ascent() const

/*
Returns the ascent of this QRawFont in pixel units.

The ascent of a font is the distance from the baseline to the highest position characters extend to. In practice, some font designers break this rule, e.g. when they put more than one accent on top of a character, or to accommodate an unusual character in an exotic language, so it is possible (though rare) that this value will be too small.

See also QFontMetricsF::ascent().
*/
impl /*struct*/ QRawFont {
  pub fn ascent_0<RetType, T: QRawFont_ascent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.ascent_0(self);
    // return 1;
  }
}
pub trait QRawFont_ascent_0<RetType> {
  fn ascent_0(self , rsthis: & QRawFont) -> RetType;
}
impl<'a> /*trait*/ QRawFont_ascent_0<f64> for () {
  fn ascent_0(self , rsthis: & QRawFont) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QRawFont6ascentEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qrawfont.h:121
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal capHeight() const

/*
Returns the cap height of this QRawFont in pixel units.

The cap height of a font is the height of a capital letter above the baseline. It specifically is the height of capital letters that are flat - such as H or I - as opposed to round letters such as O, or pointed letters like A, both of which may display overshoot.

This function was introduced in  Qt 5.8.

See also QFontMetricsF::capHeight().
*/
impl /*struct*/ QRawFont {
  pub fn capHeight_0<RetType, T: QRawFont_capHeight_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.capHeight_0(self);
    // return 1;
  }
}
pub trait QRawFont_capHeight_0<RetType> {
  fn capHeight_0(self , rsthis: & QRawFont) -> RetType;
}
impl<'a> /*trait*/ QRawFont_capHeight_0<f64> for () {
  fn capHeight_0(self , rsthis: & QRawFont) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QRawFont9capHeightEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qrawfont.h:122
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal descent() const

/*
Returns the descent of this QRawFont in pixel units.

The descent is the distance from the base line to the lowest point characters extend to. In practice, some font designers break this rule, e.g. to accommodate an unusual character in an exotic language, so it is possible (though rare) that this value will be too small.

See also QFontMetricsF::descent().
*/
impl /*struct*/ QRawFont {
  pub fn descent_0<RetType, T: QRawFont_descent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.descent_0(self);
    // return 1;
  }
}
pub trait QRawFont_descent_0<RetType> {
  fn descent_0(self , rsthis: & QRawFont) -> RetType;
}
impl<'a> /*trait*/ QRawFont_descent_0<f64> for () {
  fn descent_0(self , rsthis: & QRawFont) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QRawFont7descentEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qrawfont.h:123
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal leading() const

/*
Returns the leading of this QRawFont in pixel units.

This is the natural inter-line spacing.

See also QFontMetricsF::leading().
*/
impl /*struct*/ QRawFont {
  pub fn leading_0<RetType, T: QRawFont_leading_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.leading_0(self);
    // return 1;
  }
}
pub trait QRawFont_leading_0<RetType> {
  fn leading_0(self , rsthis: & QRawFont) -> RetType;
}
impl<'a> /*trait*/ QRawFont_leading_0<f64> for () {
  fn leading_0(self , rsthis: & QRawFont) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QRawFont7leadingEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qrawfont.h:124
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal xHeight() const

/*
Returns the xHeight of this QRawFont in pixel units.

This is often but not always the same as the height of the character 'x'.

See also QFontMetricsF::xHeight().
*/
impl /*struct*/ QRawFont {
  pub fn xHeight_0<RetType, T: QRawFont_xHeight_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.xHeight_0(self);
    // return 1;
  }
}
pub trait QRawFont_xHeight_0<RetType> {
  fn xHeight_0(self , rsthis: & QRawFont) -> RetType;
}
impl<'a> /*trait*/ QRawFont_xHeight_0<f64> for () {
  fn xHeight_0(self , rsthis: & QRawFont) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QRawFont7xHeightEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qrawfont.h:125
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal averageCharWidth() const

/*
Returns the average character width of this QRawFont in pixel units.

See also QFontMetricsF::averageCharWidth().
*/
impl /*struct*/ QRawFont {
  pub fn averageCharWidth_0<RetType, T: QRawFont_averageCharWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.averageCharWidth_0(self);
    // return 1;
  }
}
pub trait QRawFont_averageCharWidth_0<RetType> {
  fn averageCharWidth_0(self , rsthis: & QRawFont) -> RetType;
}
impl<'a> /*trait*/ QRawFont_averageCharWidth_0<f64> for () {
  fn averageCharWidth_0(self , rsthis: & QRawFont) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QRawFont16averageCharWidthEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qrawfont.h:126
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal maxCharWidth() const

/*
Returns the width of the widest character in the font.

See also QFontMetricsF::maxWidth().
*/
impl /*struct*/ QRawFont {
  pub fn maxCharWidth_0<RetType, T: QRawFont_maxCharWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.maxCharWidth_0(self);
    // return 1;
  }
}
pub trait QRawFont_maxCharWidth_0<RetType> {
  fn maxCharWidth_0(self , rsthis: & QRawFont) -> RetType;
}
impl<'a> /*trait*/ QRawFont_maxCharWidth_0<f64> for () {
  fn maxCharWidth_0(self , rsthis: & QRawFont) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QRawFont12maxCharWidthEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qrawfont.h:127
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal lineThickness() const

/*
Returns the thickness for drawing lines (underline, overline, etc.) along with text drawn in this font.
*/
impl /*struct*/ QRawFont {
  pub fn lineThickness_0<RetType, T: QRawFont_lineThickness_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.lineThickness_0(self);
    // return 1;
  }
}
pub trait QRawFont_lineThickness_0<RetType> {
  fn lineThickness_0(self , rsthis: & QRawFont) -> RetType;
}
impl<'a> /*trait*/ QRawFont_lineThickness_0<f64> for () {
  fn lineThickness_0(self , rsthis: & QRawFont) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QRawFont13lineThicknessEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qrawfont.h:128
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal underlinePosition() const

/*
Returns the position from baseline for drawing underlines below the text rendered with this font.
*/
impl /*struct*/ QRawFont {
  pub fn underlinePosition_0<RetType, T: QRawFont_underlinePosition_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.underlinePosition_0(self);
    // return 1;
  }
}
pub trait QRawFont_underlinePosition_0<RetType> {
  fn underlinePosition_0(self , rsthis: & QRawFont) -> RetType;
}
impl<'a> /*trait*/ QRawFont_underlinePosition_0<f64> for () {
  fn underlinePosition_0(self , rsthis: & QRawFont) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QRawFont17underlinePositionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qrawfont.h:130
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal unitsPerEm() const

/*
Returns the number of design units define the width and height of the em square for this QRawFont. This value is used together with the pixel size when converting design metrics to pixel units, as the internal metrics are specified in design units and the pixel size gives the size of 1 em in pixels.

See also pixelSize() and setPixelSize().
*/
impl /*struct*/ QRawFont {
  pub fn unitsPerEm_0<RetType, T: QRawFont_unitsPerEm_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.unitsPerEm_0(self);
    // return 1;
  }
}
pub trait QRawFont_unitsPerEm_0<RetType> {
  fn unitsPerEm_0(self , rsthis: & QRawFont) -> RetType;
}
impl<'a> /*trait*/ QRawFont_unitsPerEm_0<f64> for () {
  fn unitsPerEm_0(self , rsthis: & QRawFont) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QRawFont10unitsPerEmEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qrawfont.h:132
// index:0
// Public Visibility=Default Availability=Available
// [-2] void loadFromFile(const QString &, qreal, QFont::HintingPreference)

/*
Replaces the current QRawFont with the contents of the file referenced by fileName for the size (in pixels) given by pixelSize, and using the hinting preference specified by hintingPreference.

The file must reference a TrueType or OpenType font.

See also loadFromData().
*/
impl /*struct*/ QRawFont {
  pub fn loadFromFile_0<RetType, T: QRawFont_loadFromFile_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.loadFromFile_0(self);
    // return 1;
  }
}
pub trait QRawFont_loadFromFile_0<RetType> {
  fn loadFromFile_0(self , rsthis: & QRawFont) -> RetType;
}
impl<'a> /*trait*/ QRawFont_loadFromFile_0<(/*void*/)> for (usize,f64,i32) {
  fn loadFromFile_0(self , rsthis: & QRawFont) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QRawFont12loadFromFileERK7QStringdN5QFont17HintingPreferenceE", 3,qtrt::FFITY_POINTER,qtrt::FFITY_DOUBLE,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qrawfont.h:136
// index:0
// Public Visibility=Default Availability=Available
// [-2] void loadFromData(const QByteArray &, qreal, QFont::HintingPreference)

/*
Replaces the current QRawFont with the font contained in the supplied fontData for the size (in pixels) given by pixelSize, and using the hinting preference specified by hintingPreference.

The fontData must contain a TrueType or OpenType font.

See also loadFromFile().
*/
impl /*struct*/ QRawFont {
  pub fn loadFromData_0<RetType, T: QRawFont_loadFromData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.loadFromData_0(self);
    // return 1;
  }
}
pub trait QRawFont_loadFromData_0<RetType> {
  fn loadFromData_0(self , rsthis: & QRawFont) -> RetType;
}
impl<'a> /*trait*/ QRawFont_loadFromData_0<(/*void*/)> for (usize,f64,i32) {
  fn loadFromData_0(self , rsthis: & QRawFont) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QRawFont12loadFromDataERK10QByteArraydN5QFont17HintingPreferenceE", 3,qtrt::FFITY_POINTER,qtrt::FFITY_DOUBLE,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qrawfont.h:140
// index:0
// Public Visibility=Default Availability=Available
// [1] bool supportsCharacter(uint) const

/*
Returns true if the font has a glyph that corresponds to the given character.

See also supportedWritingSystems().
*/
impl /*struct*/ QRawFont {
  pub fn supportsCharacter_0<RetType, T: QRawFont_supportsCharacter_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.supportsCharacter_0(self);
    // return 1;
  }
}
pub trait QRawFont_supportsCharacter_0<RetType> {
  fn supportsCharacter_0(self , rsthis: & QRawFont) -> RetType;
}
impl<'a> /*trait*/ QRawFont_supportsCharacter_0<bool> for (u32) {
  fn supportsCharacter_0(self , rsthis: & QRawFont) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const u32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QRawFont17supportsCharacterEj", 1,qtrt::FFITY_UINT32,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qrawfont.h:141
// index:1
// Public Visibility=Default Availability=Available
// [1] bool supportsCharacter(QChar) const

/*
Returns true if the font has a glyph that corresponds to the given character.

See also supportedWritingSystems().
*/
impl /*struct*/ QRawFont {
  pub fn supportsCharacter_1<RetType, T: QRawFont_supportsCharacter_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.supportsCharacter_1(self);
    // return 1;
  }
}
pub trait QRawFont_supportsCharacter_1<RetType> {
  fn supportsCharacter_1(self , rsthis: & QRawFont) -> RetType;
}
impl<'a> /*trait*/ QRawFont_supportsCharacter_1<bool> for (usize) {
  fn supportsCharacter_1(self , rsthis: & QRawFont) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QRawFont17supportsCharacterE5QChar", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qrawfont.h:144
// index:0
// Public Visibility=Default Availability=Available
// [8] QByteArray fontTable(const char *) const

/*
Retrieves the sfnt table named tagName from the underlying physical font, or an empty byte array if no such table was found. The returned font table's byte order is Big Endian, like the sfnt format specifies. The tagName must be four characters long and should be formatted in the default endianness of the current platform.
*/
impl /*struct*/ QRawFont {
  pub fn fontTable_0<RetType, T: QRawFont_fontTable_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fontTable_0(self);
    // return 1;
  }
}
pub trait QRawFont_fontTable_0<RetType> {
  fn fontTable_0(self , rsthis: & QRawFont) -> RetType;
}
impl<'a> /*trait*/ QRawFont_fontTable_0<usize> for (usize) {
  fn fontTable_0(self , rsthis: & QRawFont) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QRawFont9fontTableEPKc", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qrawfont.h:146
// index:0
// Public static Visibility=Default Availability=Available
// [8] QRawFont fromFont(const QFont &, QFontDatabase::WritingSystem)

/*
Fetches the physical representation based on a font query. The physical font returned is the font that will be preferred by Qt in order to display text in the selected writingSystem.

Warning: This function is potentially expensive and should not be called in performance sensitive code.
*/
impl /*struct*/ QRawFont {
  pub fn fromFont_0<RetType, T: QRawFont_fromFont_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromFont_0();
    // return 1;
  }
}
pub trait QRawFont_fromFont_0<RetType> {
  fn fromFont_0(self ) -> RetType;
}
impl<'a> /*trait*/ QRawFont_fromFont_0<usize> for (usize,i32) {
  fn fromFont_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QRawFont8fromFontERK5QFontN13QFontDatabase13WritingSystemE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}


/*
This enum represents the different ways a glyph can be rasterized in the function alphaMapForGlyph().


*/
pub type QRawFont__AntialiasingType = i32;
// Will rasterize by measuring the coverage of the shape on whole pixels. The returned image contains the alpha values of each pixel based on the coverage of the glyph shape.
pub const QRawFont__PixelAntialiasing :QRawFont__AntialiasingType = 0;
// Will rasterize by measuring the coverage of each subpixel, returning a separate alpha value for each of the red, green and blue components of each pixel.
pub const QRawFont__SubPixelAntialiasing :QRawFont__AntialiasingType = 1;
pub fn QRawFont_AntialiasingTypeItemName(val: i32) ->String {
  match val {
     QRawFont__PixelAntialiasing => // 0
     {return String::from("PixelAntialiasing");}
     QRawFont__SubPixelAntialiasing => // 1
     {return String::from("SubPixelAntialiasing");}
  _ => {return format!("{}", val);}
}
}
pub fn QRawFont_AntialiasingTypeItemName_s(val: i32) ->String {
  //var nilthis *QRawFont
  //return nilthis.AntialiasingTypeItemName(val);
  return QRawFont_AntialiasingTypeItemName(val);
}


/*


*/
pub type QRawFont__LayoutFlag = i32;
// 
pub const QRawFont__SeparateAdvances :QRawFont__LayoutFlag = 0;
// 
pub const QRawFont__KernedAdvances :QRawFont__LayoutFlag = 1;
// 
pub const QRawFont__UseDesignMetrics :QRawFont__LayoutFlag = 2;
pub fn QRawFont_LayoutFlagItemName(val: i32) ->String {
  match val {
     QRawFont__SeparateAdvances => // 0
     {return String::from("SeparateAdvances");}
     QRawFont__KernedAdvances => // 1
     {return String::from("KernedAdvances");}
     QRawFont__UseDesignMetrics => // 2
     {return String::from("UseDesignMetrics");}
  _ => {return format!("{}", val);}
}
}
pub fn QRawFont_LayoutFlagItemName_s(val: i32) ->String {
  //var nilthis *QRawFont
  //return nilthis.LayoutFlagItemName(val);
  return QRawFont_LayoutFlagItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
