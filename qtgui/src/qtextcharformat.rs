

// mod ::gui::QTextCharFormat
// package qtgui
// /usr/include/qt/QtGui/qtextformat.h
// #include <qtextformat.h>
// #include <QtGui>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 50
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
#[derive(Default)] // class sizeof(QTextCharFormat)=16
pub struct QTextCharFormat {
  qbase: QTextFormat,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QTextCharFormat_ITF interface {
//    QTextFormat_ITF
//    QTextCharFormat_PTR() *QTextCharFormat
//}
//func (ptr *QTextCharFormat) QTextCharFormat_PTR() *QTextCharFormat { return ptr }

impl /*struct*/ QTextCharFormat {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QTextCharFormat {
    return QTextCharFormat{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QTextCharFormat {
//  type Target = QTextCharFormatBASE;
//
//  fn deref(&self) -> &QTextCharFormatBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QTextCharFormatBASE> for QTextCharFormat {
//  fn as_ref(& self) -> & QTextCharFormatBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qtextformat.h:412
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QTextCharFormat()

/*

*/
// QTextCharFormat() ctx.fn_proto_cpp
impl /*struct*/ QTextCharFormat {
  pub fn QTextCharFormat_0<T: QTextCharFormat_QTextCharFormat_0>(value: T) -> QTextCharFormat {
    let rsthis = value.QTextCharFormat_0();
    return rsthis;
    // return 1;
  }
}

pub trait QTextCharFormat_QTextCharFormat_0 {
  fn QTextCharFormat_0(self) -> QTextCharFormat;
}
// QTextCharFormat() ctx.fn_proto_cpp
impl<'a> /*trait*/ QTextCharFormat_QTextCharFormat_0 for () {
  fn QTextCharFormat_0(self) -> QTextCharFormat {
    // unsafe{_ZN15QTextCharFormatC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN15QTextCharFormatC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTextCharFormat{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:557
// index:1
// Protected Visibility=Default Availability=Available
// [-2] void QTextCharFormat(const QTextFormat &)

/*

*/
// QTextCharFormat(const QTextFormat &) ctx.fn_proto_cpp
impl /*struct*/ QTextCharFormat {
  pub fn QTextCharFormat_1<T: QTextCharFormat_QTextCharFormat_1>(value: T) -> QTextCharFormat {
    let rsthis = value.QTextCharFormat_1();
    return rsthis;
    // return 1;
  }
}

pub trait QTextCharFormat_QTextCharFormat_1 {
  fn QTextCharFormat_1(self) -> QTextCharFormat;
}
// QTextCharFormat(const QTextFormat &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QTextCharFormat_QTextCharFormat_1 for (usize) {
  fn QTextCharFormat_1(self) -> QTextCharFormat {
    // unsafe{_ZN15QTextCharFormatC2ERK11QTextFormat()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN15QTextCharFormatC2ERK11QTextFormat", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTextCharFormat{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:414
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isValid() const

/*
Returns true if the format is valid (i.e. is not InvalidFormat); otherwise returns false.
*/
impl /*struct*/ QTextCharFormat {
  pub fn isValid_0<RetType, T: QTextCharFormat_isValid_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isValid_0(self);
    // return 1;
  }
}
pub trait QTextCharFormat_isValid_0<RetType> {
  fn isValid_0(self , rsthis: & QTextCharFormat) -> RetType;
}
impl<'a> /*trait*/ QTextCharFormat_isValid_0<bool> for () {
  fn isValid_0(self , rsthis: & QTextCharFormat) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QTextCharFormat7isValidEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:420
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFont(const QFont &, QTextCharFormat::FontPropertiesInheritanceBehavior)

/*

*/
impl /*struct*/ QTextCharFormat {
  pub fn setFont_0<RetType, T: QTextCharFormat_setFont_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFont_0(self);
    // return 1;
  }
}
pub trait QTextCharFormat_setFont_0<RetType> {
  fn setFont_0(self , rsthis: & QTextCharFormat) -> RetType;
}
impl<'a> /*trait*/ QTextCharFormat_setFont_0<(/*void*/)> for (usize,i32) {
  fn setFont_0(self , rsthis: & QTextCharFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN15QTextCharFormat7setFontERK5QFontNS_33FontPropertiesInheritanceBehaviorE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:421
// index:1
// Public Visibility=Default Availability=Available
// [-2] void setFont(const QFont &)

/*

*/
impl /*struct*/ QTextCharFormat {
  pub fn setFont_1<RetType, T: QTextCharFormat_setFont_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFont_1(self);
    // return 1;
  }
}
pub trait QTextCharFormat_setFont_1<RetType> {
  fn setFont_1(self , rsthis: & QTextCharFormat) -> RetType;
}
impl<'a> /*trait*/ QTextCharFormat_setFont_1<(/*void*/)> for (usize) {
  fn setFont_1(self , rsthis: & QTextCharFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QTextCharFormat7setFontERK5QFont", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:422
// index:0
// Public Visibility=Default Availability=Available
// [16] QFont font() const

/*

*/
impl /*struct*/ QTextCharFormat {
  pub fn font_0<RetType, T: QTextCharFormat_font_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.font_0(self);
    // return 1;
  }
}
pub trait QTextCharFormat_font_0<RetType> {
  fn font_0(self , rsthis: & QTextCharFormat) -> RetType;
}
impl<'a> /*trait*/ QTextCharFormat_font_0<usize> for () {
  fn font_0(self , rsthis: & QTextCharFormat) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QTextCharFormat4fontEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:424
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setFontFamily(const QString &)

/*

*/
impl /*struct*/ QTextCharFormat {
  pub fn setFontFamily_0<RetType, T: QTextCharFormat_setFontFamily_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFontFamily_0(self);
    // return 1;
  }
}
pub trait QTextCharFormat_setFontFamily_0<RetType> {
  fn setFontFamily_0(self , rsthis: & QTextCharFormat) -> RetType;
}
impl<'a> /*trait*/ QTextCharFormat_setFontFamily_0<(/*void*/)> for (usize) {
  fn setFontFamily_0(self , rsthis: & QTextCharFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QTextCharFormat13setFontFamilyERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:426
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QString fontFamily() const

/*

*/
impl /*struct*/ QTextCharFormat {
  pub fn fontFamily_0<RetType, T: QTextCharFormat_fontFamily_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fontFamily_0(self);
    // return 1;
  }
}
pub trait QTextCharFormat_fontFamily_0<RetType> {
  fn fontFamily_0(self , rsthis: & QTextCharFormat) -> RetType;
}
impl<'a> /*trait*/ QTextCharFormat_fontFamily_0<usize> for () {
  fn fontFamily_0(self , rsthis: & QTextCharFormat) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QTextCharFormat10fontFamilyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:429
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setFontPointSize(qreal)

/*

*/
impl /*struct*/ QTextCharFormat {
  pub fn setFontPointSize_0<RetType, T: QTextCharFormat_setFontPointSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFontPointSize_0(self);
    // return 1;
  }
}
pub trait QTextCharFormat_setFontPointSize_0<RetType> {
  fn setFontPointSize_0(self , rsthis: & QTextCharFormat) -> RetType;
}
impl<'a> /*trait*/ QTextCharFormat_setFontPointSize_0<(/*void*/)> for (f64) {
  fn setFontPointSize_0(self , rsthis: & QTextCharFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN15QTextCharFormat16setFontPointSizeEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:431
// index:0
// Public inline Visibility=Default Availability=Available
// [8] qreal fontPointSize() const

/*

*/
impl /*struct*/ QTextCharFormat {
  pub fn fontPointSize_0<RetType, T: QTextCharFormat_fontPointSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fontPointSize_0(self);
    // return 1;
  }
}
pub trait QTextCharFormat_fontPointSize_0<RetType> {
  fn fontPointSize_0(self , rsthis: & QTextCharFormat) -> RetType;
}
impl<'a> /*trait*/ QTextCharFormat_fontPointSize_0<f64> for () {
  fn fontPointSize_0(self , rsthis: & QTextCharFormat) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QTextCharFormat13fontPointSizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:434
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setFontWeight(int)

/*

*/
impl /*struct*/ QTextCharFormat {
  pub fn setFontWeight_0<RetType, T: QTextCharFormat_setFontWeight_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFontWeight_0(self);
    // return 1;
  }
}
pub trait QTextCharFormat_setFontWeight_0<RetType> {
  fn setFontWeight_0(self , rsthis: & QTextCharFormat) -> RetType;
}
impl<'a> /*trait*/ QTextCharFormat_setFontWeight_0<(/*void*/)> for (i32) {
  fn setFontWeight_0(self , rsthis: & QTextCharFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN15QTextCharFormat13setFontWeightEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:436
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int fontWeight() const

/*

*/
impl /*struct*/ QTextCharFormat {
  pub fn fontWeight_0<RetType, T: QTextCharFormat_fontWeight_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fontWeight_0(self);
    // return 1;
  }
}
pub trait QTextCharFormat_fontWeight_0<RetType> {
  fn fontWeight_0(self , rsthis: & QTextCharFormat) -> RetType;
}
impl<'a> /*trait*/ QTextCharFormat_fontWeight_0<i32> for () {
  fn fontWeight_0(self , rsthis: & QTextCharFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QTextCharFormat10fontWeightEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:438
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setFontItalic(bool)

/*

*/
impl /*struct*/ QTextCharFormat {
  pub fn setFontItalic_0<RetType, T: QTextCharFormat_setFontItalic_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFontItalic_0(self);
    // return 1;
  }
}
pub trait QTextCharFormat_setFontItalic_0<RetType> {
  fn setFontItalic_0(self , rsthis: & QTextCharFormat) -> RetType;
}
impl<'a> /*trait*/ QTextCharFormat_setFontItalic_0<(/*void*/)> for (bool) {
  fn setFontItalic_0(self , rsthis: & QTextCharFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN15QTextCharFormat13setFontItalicEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:440
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool fontItalic() const

/*

*/
impl /*struct*/ QTextCharFormat {
  pub fn fontItalic_0<RetType, T: QTextCharFormat_fontItalic_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fontItalic_0(self);
    // return 1;
  }
}
pub trait QTextCharFormat_fontItalic_0<RetType> {
  fn fontItalic_0(self , rsthis: & QTextCharFormat) -> RetType;
}
impl<'a> /*trait*/ QTextCharFormat_fontItalic_0<bool> for () {
  fn fontItalic_0(self , rsthis: & QTextCharFormat) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QTextCharFormat10fontItalicEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:442
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setFontCapitalization(QFont::Capitalization)

/*

*/
impl /*struct*/ QTextCharFormat {
  pub fn setFontCapitalization_0<RetType, T: QTextCharFormat_setFontCapitalization_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFontCapitalization_0(self);
    // return 1;
  }
}
pub trait QTextCharFormat_setFontCapitalization_0<RetType> {
  fn setFontCapitalization_0(self , rsthis: & QTextCharFormat) -> RetType;
}
impl<'a> /*trait*/ QTextCharFormat_setFontCapitalization_0<(/*void*/)> for (i32) {
  fn setFontCapitalization_0(self , rsthis: & QTextCharFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN15QTextCharFormat21setFontCapitalizationEN5QFont14CapitalizationE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:444
// index:0
// Public inline Visibility=Default Availability=Available
// [4] QFont::Capitalization fontCapitalization() const

/*

*/
impl /*struct*/ QTextCharFormat {
  pub fn fontCapitalization_0<RetType, T: QTextCharFormat_fontCapitalization_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fontCapitalization_0(self);
    // return 1;
  }
}
pub trait QTextCharFormat_fontCapitalization_0<RetType> {
  fn fontCapitalization_0(self , rsthis: & QTextCharFormat) -> RetType;
}
impl<'a> /*trait*/ QTextCharFormat_fontCapitalization_0<i32> for () {
  fn fontCapitalization_0(self , rsthis: & QTextCharFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QTextCharFormat18fontCapitalizationEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:446
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setFontLetterSpacingType(QFont::SpacingType)

/*

*/
impl /*struct*/ QTextCharFormat {
  pub fn setFontLetterSpacingType_0<RetType, T: QTextCharFormat_setFontLetterSpacingType_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFontLetterSpacingType_0(self);
    // return 1;
  }
}
pub trait QTextCharFormat_setFontLetterSpacingType_0<RetType> {
  fn setFontLetterSpacingType_0(self , rsthis: & QTextCharFormat) -> RetType;
}
impl<'a> /*trait*/ QTextCharFormat_setFontLetterSpacingType_0<(/*void*/)> for (i32) {
  fn setFontLetterSpacingType_0(self , rsthis: & QTextCharFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN15QTextCharFormat24setFontLetterSpacingTypeEN5QFont11SpacingTypeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:448
// index:0
// Public inline Visibility=Default Availability=Available
// [4] QFont::SpacingType fontLetterSpacingType() const

/*

*/
impl /*struct*/ QTextCharFormat {
  pub fn fontLetterSpacingType_0<RetType, T: QTextCharFormat_fontLetterSpacingType_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fontLetterSpacingType_0(self);
    // return 1;
  }
}
pub trait QTextCharFormat_fontLetterSpacingType_0<RetType> {
  fn fontLetterSpacingType_0(self , rsthis: & QTextCharFormat) -> RetType;
}
impl<'a> /*trait*/ QTextCharFormat_fontLetterSpacingType_0<i32> for () {
  fn fontLetterSpacingType_0(self , rsthis: & QTextCharFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QTextCharFormat21fontLetterSpacingTypeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:450
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setFontLetterSpacing(qreal)

/*

*/
impl /*struct*/ QTextCharFormat {
  pub fn setFontLetterSpacing_0<RetType, T: QTextCharFormat_setFontLetterSpacing_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFontLetterSpacing_0(self);
    // return 1;
  }
}
pub trait QTextCharFormat_setFontLetterSpacing_0<RetType> {
  fn setFontLetterSpacing_0(self , rsthis: & QTextCharFormat) -> RetType;
}
impl<'a> /*trait*/ QTextCharFormat_setFontLetterSpacing_0<(/*void*/)> for (f64) {
  fn setFontLetterSpacing_0(self , rsthis: & QTextCharFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN15QTextCharFormat20setFontLetterSpacingEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:452
// index:0
// Public inline Visibility=Default Availability=Available
// [8] qreal fontLetterSpacing() const

/*

*/
impl /*struct*/ QTextCharFormat {
  pub fn fontLetterSpacing_0<RetType, T: QTextCharFormat_fontLetterSpacing_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fontLetterSpacing_0(self);
    // return 1;
  }
}
pub trait QTextCharFormat_fontLetterSpacing_0<RetType> {
  fn fontLetterSpacing_0(self , rsthis: & QTextCharFormat) -> RetType;
}
impl<'a> /*trait*/ QTextCharFormat_fontLetterSpacing_0<f64> for () {
  fn fontLetterSpacing_0(self , rsthis: & QTextCharFormat) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QTextCharFormat17fontLetterSpacingEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:454
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setFontWordSpacing(qreal)

/*

*/
impl /*struct*/ QTextCharFormat {
  pub fn setFontWordSpacing_0<RetType, T: QTextCharFormat_setFontWordSpacing_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFontWordSpacing_0(self);
    // return 1;
  }
}
pub trait QTextCharFormat_setFontWordSpacing_0<RetType> {
  fn setFontWordSpacing_0(self , rsthis: & QTextCharFormat) -> RetType;
}
impl<'a> /*trait*/ QTextCharFormat_setFontWordSpacing_0<(/*void*/)> for (f64) {
  fn setFontWordSpacing_0(self , rsthis: & QTextCharFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN15QTextCharFormat18setFontWordSpacingEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:456
// index:0
// Public inline Visibility=Default Availability=Available
// [8] qreal fontWordSpacing() const

/*

*/
impl /*struct*/ QTextCharFormat {
  pub fn fontWordSpacing_0<RetType, T: QTextCharFormat_fontWordSpacing_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fontWordSpacing_0(self);
    // return 1;
  }
}
pub trait QTextCharFormat_fontWordSpacing_0<RetType> {
  fn fontWordSpacing_0(self , rsthis: & QTextCharFormat) -> RetType;
}
impl<'a> /*trait*/ QTextCharFormat_fontWordSpacing_0<f64> for () {
  fn fontWordSpacing_0(self , rsthis: & QTextCharFormat) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QTextCharFormat15fontWordSpacingEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:459
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setFontUnderline(bool)

/*

*/
impl /*struct*/ QTextCharFormat {
  pub fn setFontUnderline_0<RetType, T: QTextCharFormat_setFontUnderline_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFontUnderline_0(self);
    // return 1;
  }
}
pub trait QTextCharFormat_setFontUnderline_0<RetType> {
  fn setFontUnderline_0(self , rsthis: & QTextCharFormat) -> RetType;
}
impl<'a> /*trait*/ QTextCharFormat_setFontUnderline_0<(/*void*/)> for (bool) {
  fn setFontUnderline_0(self , rsthis: & QTextCharFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN15QTextCharFormat16setFontUnderlineEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:461
// index:0
// Public Visibility=Default Availability=Available
// [1] bool fontUnderline() const

/*

*/
impl /*struct*/ QTextCharFormat {
  pub fn fontUnderline_0<RetType, T: QTextCharFormat_fontUnderline_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fontUnderline_0(self);
    // return 1;
  }
}
pub trait QTextCharFormat_fontUnderline_0<RetType> {
  fn fontUnderline_0(self , rsthis: & QTextCharFormat) -> RetType;
}
impl<'a> /*trait*/ QTextCharFormat_fontUnderline_0<bool> for () {
  fn fontUnderline_0(self , rsthis: & QTextCharFormat) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QTextCharFormat13fontUnderlineEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:463
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setFontOverline(bool)

/*

*/
impl /*struct*/ QTextCharFormat {
  pub fn setFontOverline_0<RetType, T: QTextCharFormat_setFontOverline_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFontOverline_0(self);
    // return 1;
  }
}
pub trait QTextCharFormat_setFontOverline_0<RetType> {
  fn setFontOverline_0(self , rsthis: & QTextCharFormat) -> RetType;
}
impl<'a> /*trait*/ QTextCharFormat_setFontOverline_0<(/*void*/)> for (bool) {
  fn setFontOverline_0(self , rsthis: & QTextCharFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN15QTextCharFormat15setFontOverlineEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:465
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool fontOverline() const

/*

*/
impl /*struct*/ QTextCharFormat {
  pub fn fontOverline_0<RetType, T: QTextCharFormat_fontOverline_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fontOverline_0(self);
    // return 1;
  }
}
pub trait QTextCharFormat_fontOverline_0<RetType> {
  fn fontOverline_0(self , rsthis: & QTextCharFormat) -> RetType;
}
impl<'a> /*trait*/ QTextCharFormat_fontOverline_0<bool> for () {
  fn fontOverline_0(self , rsthis: & QTextCharFormat) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QTextCharFormat12fontOverlineEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:468
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setFontStrikeOut(bool)

/*

*/
impl /*struct*/ QTextCharFormat {
  pub fn setFontStrikeOut_0<RetType, T: QTextCharFormat_setFontStrikeOut_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFontStrikeOut_0(self);
    // return 1;
  }
}
pub trait QTextCharFormat_setFontStrikeOut_0<RetType> {
  fn setFontStrikeOut_0(self , rsthis: & QTextCharFormat) -> RetType;
}
impl<'a> /*trait*/ QTextCharFormat_setFontStrikeOut_0<(/*void*/)> for (bool) {
  fn setFontStrikeOut_0(self , rsthis: & QTextCharFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN15QTextCharFormat16setFontStrikeOutEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:470
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool fontStrikeOut() const

/*

*/
impl /*struct*/ QTextCharFormat {
  pub fn fontStrikeOut_0<RetType, T: QTextCharFormat_fontStrikeOut_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fontStrikeOut_0(self);
    // return 1;
  }
}
pub trait QTextCharFormat_fontStrikeOut_0<RetType> {
  fn fontStrikeOut_0(self , rsthis: & QTextCharFormat) -> RetType;
}
impl<'a> /*trait*/ QTextCharFormat_fontStrikeOut_0<bool> for () {
  fn fontStrikeOut_0(self , rsthis: & QTextCharFormat) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QTextCharFormat13fontStrikeOutEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:473
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setUnderlineColor(const QColor &)

/*

*/
impl /*struct*/ QTextCharFormat {
  pub fn setUnderlineColor_0<RetType, T: QTextCharFormat_setUnderlineColor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setUnderlineColor_0(self);
    // return 1;
  }
}
pub trait QTextCharFormat_setUnderlineColor_0<RetType> {
  fn setUnderlineColor_0(self , rsthis: & QTextCharFormat) -> RetType;
}
impl<'a> /*trait*/ QTextCharFormat_setUnderlineColor_0<(/*void*/)> for (usize) {
  fn setUnderlineColor_0(self , rsthis: & QTextCharFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QTextCharFormat17setUnderlineColorERK6QColor", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:475
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QColor underlineColor() const

/*

*/
impl /*struct*/ QTextCharFormat {
  pub fn underlineColor_0<RetType, T: QTextCharFormat_underlineColor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.underlineColor_0(self);
    // return 1;
  }
}
pub trait QTextCharFormat_underlineColor_0<RetType> {
  fn underlineColor_0(self , rsthis: & QTextCharFormat) -> RetType;
}
impl<'a> /*trait*/ QTextCharFormat_underlineColor_0<usize> for () {
  fn underlineColor_0(self , rsthis: & QTextCharFormat) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QTextCharFormat14underlineColorEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:478
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setFontFixedPitch(bool)

/*

*/
impl /*struct*/ QTextCharFormat {
  pub fn setFontFixedPitch_0<RetType, T: QTextCharFormat_setFontFixedPitch_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFontFixedPitch_0(self);
    // return 1;
  }
}
pub trait QTextCharFormat_setFontFixedPitch_0<RetType> {
  fn setFontFixedPitch_0(self , rsthis: & QTextCharFormat) -> RetType;
}
impl<'a> /*trait*/ QTextCharFormat_setFontFixedPitch_0<(/*void*/)> for (bool) {
  fn setFontFixedPitch_0(self , rsthis: & QTextCharFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN15QTextCharFormat17setFontFixedPitchEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:480
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool fontFixedPitch() const

/*

*/
impl /*struct*/ QTextCharFormat {
  pub fn fontFixedPitch_0<RetType, T: QTextCharFormat_fontFixedPitch_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fontFixedPitch_0(self);
    // return 1;
  }
}
pub trait QTextCharFormat_fontFixedPitch_0<RetType> {
  fn fontFixedPitch_0(self , rsthis: & QTextCharFormat) -> RetType;
}
impl<'a> /*trait*/ QTextCharFormat_fontFixedPitch_0<bool> for () {
  fn fontFixedPitch_0(self , rsthis: & QTextCharFormat) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QTextCharFormat14fontFixedPitchEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:483
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setFontStretch(int)

/*

*/
impl /*struct*/ QTextCharFormat {
  pub fn setFontStretch_0<RetType, T: QTextCharFormat_setFontStretch_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFontStretch_0(self);
    // return 1;
  }
}
pub trait QTextCharFormat_setFontStretch_0<RetType> {
  fn setFontStretch_0(self , rsthis: & QTextCharFormat) -> RetType;
}
impl<'a> /*trait*/ QTextCharFormat_setFontStretch_0<(/*void*/)> for (i32) {
  fn setFontStretch_0(self , rsthis: & QTextCharFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN15QTextCharFormat14setFontStretchEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:485
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int fontStretch() const

/*

*/
impl /*struct*/ QTextCharFormat {
  pub fn fontStretch_0<RetType, T: QTextCharFormat_fontStretch_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fontStretch_0(self);
    // return 1;
  }
}
pub trait QTextCharFormat_fontStretch_0<RetType> {
  fn fontStretch_0(self , rsthis: & QTextCharFormat) -> RetType;
}
impl<'a> /*trait*/ QTextCharFormat_fontStretch_0<i32> for () {
  fn fontStretch_0(self , rsthis: & QTextCharFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QTextCharFormat11fontStretchEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:488
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setFontStyleHint(QFont::StyleHint, QFont::StyleStrategy)

/*

*/
impl /*struct*/ QTextCharFormat {
  pub fn setFontStyleHint_0<RetType, T: QTextCharFormat_setFontStyleHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFontStyleHint_0(self);
    // return 1;
  }
}
pub trait QTextCharFormat_setFontStyleHint_0<RetType> {
  fn setFontStyleHint_0(self , rsthis: & QTextCharFormat) -> RetType;
}
impl<'a> /*trait*/ QTextCharFormat_setFontStyleHint_0<(/*void*/)> for (i32,i32) {
  fn setFontStyleHint_0(self , rsthis: & QTextCharFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN15QTextCharFormat16setFontStyleHintEN5QFont9StyleHintENS0_13StyleStrategyE", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:490
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setFontStyleStrategy(QFont::StyleStrategy)

/*

*/
impl /*struct*/ QTextCharFormat {
  pub fn setFontStyleStrategy_0<RetType, T: QTextCharFormat_setFontStyleStrategy_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFontStyleStrategy_0(self);
    // return 1;
  }
}
pub trait QTextCharFormat_setFontStyleStrategy_0<RetType> {
  fn setFontStyleStrategy_0(self , rsthis: & QTextCharFormat) -> RetType;
}
impl<'a> /*trait*/ QTextCharFormat_setFontStyleStrategy_0<(/*void*/)> for (i32) {
  fn setFontStyleStrategy_0(self , rsthis: & QTextCharFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN15QTextCharFormat20setFontStyleStrategyEN5QFont13StyleStrategyE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:492
// index:0
// Public inline Visibility=Default Availability=Available
// [4] QFont::StyleHint fontStyleHint() const

/*

*/
impl /*struct*/ QTextCharFormat {
  pub fn fontStyleHint_0<RetType, T: QTextCharFormat_fontStyleHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fontStyleHint_0(self);
    // return 1;
  }
}
pub trait QTextCharFormat_fontStyleHint_0<RetType> {
  fn fontStyleHint_0(self , rsthis: & QTextCharFormat) -> RetType;
}
impl<'a> /*trait*/ QTextCharFormat_fontStyleHint_0<i32> for () {
  fn fontStyleHint_0(self , rsthis: & QTextCharFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QTextCharFormat13fontStyleHintEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:494
// index:0
// Public inline Visibility=Default Availability=Available
// [4] QFont::StyleStrategy fontStyleStrategy() const

/*

*/
impl /*struct*/ QTextCharFormat {
  pub fn fontStyleStrategy_0<RetType, T: QTextCharFormat_fontStyleStrategy_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fontStyleStrategy_0(self);
    // return 1;
  }
}
pub trait QTextCharFormat_fontStyleStrategy_0<RetType> {
  fn fontStyleStrategy_0(self , rsthis: & QTextCharFormat) -> RetType;
}
impl<'a> /*trait*/ QTextCharFormat_fontStyleStrategy_0<i32> for () {
  fn fontStyleStrategy_0(self , rsthis: & QTextCharFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QTextCharFormat17fontStyleStrategyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:497
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setFontHintingPreference(QFont::HintingPreference)

/*

*/
impl /*struct*/ QTextCharFormat {
  pub fn setFontHintingPreference_0<RetType, T: QTextCharFormat_setFontHintingPreference_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFontHintingPreference_0(self);
    // return 1;
  }
}
pub trait QTextCharFormat_setFontHintingPreference_0<RetType> {
  fn setFontHintingPreference_0(self , rsthis: & QTextCharFormat) -> RetType;
}
impl<'a> /*trait*/ QTextCharFormat_setFontHintingPreference_0<(/*void*/)> for (i32) {
  fn setFontHintingPreference_0(self , rsthis: & QTextCharFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN15QTextCharFormat24setFontHintingPreferenceEN5QFont17HintingPreferenceE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:502
// index:0
// Public inline Visibility=Default Availability=Available
// [4] QFont::HintingPreference fontHintingPreference() const

/*

*/
impl /*struct*/ QTextCharFormat {
  pub fn fontHintingPreference_0<RetType, T: QTextCharFormat_fontHintingPreference_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fontHintingPreference_0(self);
    // return 1;
  }
}
pub trait QTextCharFormat_fontHintingPreference_0<RetType> {
  fn fontHintingPreference_0(self , rsthis: & QTextCharFormat) -> RetType;
}
impl<'a> /*trait*/ QTextCharFormat_fontHintingPreference_0<i32> for () {
  fn fontHintingPreference_0(self , rsthis: & QTextCharFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QTextCharFormat21fontHintingPreferenceEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:507
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setFontKerning(bool)

/*

*/
impl /*struct*/ QTextCharFormat {
  pub fn setFontKerning_0<RetType, T: QTextCharFormat_setFontKerning_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFontKerning_0(self);
    // return 1;
  }
}
pub trait QTextCharFormat_setFontKerning_0<RetType> {
  fn setFontKerning_0(self , rsthis: & QTextCharFormat) -> RetType;
}
impl<'a> /*trait*/ QTextCharFormat_setFontKerning_0<(/*void*/)> for (bool) {
  fn setFontKerning_0(self , rsthis: & QTextCharFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN15QTextCharFormat14setFontKerningEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:509
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool fontKerning() const

/*

*/
impl /*struct*/ QTextCharFormat {
  pub fn fontKerning_0<RetType, T: QTextCharFormat_fontKerning_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fontKerning_0(self);
    // return 1;
  }
}
pub trait QTextCharFormat_fontKerning_0<RetType> {
  fn fontKerning_0(self , rsthis: & QTextCharFormat) -> RetType;
}
impl<'a> /*trait*/ QTextCharFormat_fontKerning_0<bool> for () {
  fn fontKerning_0(self , rsthis: & QTextCharFormat) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QTextCharFormat11fontKerningEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:512
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setUnderlineStyle(QTextCharFormat::UnderlineStyle)

/*

*/
impl /*struct*/ QTextCharFormat {
  pub fn setUnderlineStyle_0<RetType, T: QTextCharFormat_setUnderlineStyle_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setUnderlineStyle_0(self);
    // return 1;
  }
}
pub trait QTextCharFormat_setUnderlineStyle_0<RetType> {
  fn setUnderlineStyle_0(self , rsthis: & QTextCharFormat) -> RetType;
}
impl<'a> /*trait*/ QTextCharFormat_setUnderlineStyle_0<(/*void*/)> for (i32) {
  fn setUnderlineStyle_0(self , rsthis: & QTextCharFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN15QTextCharFormat17setUnderlineStyleENS_14UnderlineStyleE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:513
// index:0
// Public inline Visibility=Default Availability=Available
// [4] QTextCharFormat::UnderlineStyle underlineStyle() const

/*

*/
impl /*struct*/ QTextCharFormat {
  pub fn underlineStyle_0<RetType, T: QTextCharFormat_underlineStyle_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.underlineStyle_0(self);
    // return 1;
  }
}
pub trait QTextCharFormat_underlineStyle_0<RetType> {
  fn underlineStyle_0(self , rsthis: & QTextCharFormat) -> RetType;
}
impl<'a> /*trait*/ QTextCharFormat_underlineStyle_0<i32> for () {
  fn underlineStyle_0(self , rsthis: & QTextCharFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QTextCharFormat14underlineStyleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:516
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setVerticalAlignment(QTextCharFormat::VerticalAlignment)

/*

*/
impl /*struct*/ QTextCharFormat {
  pub fn setVerticalAlignment_0<RetType, T: QTextCharFormat_setVerticalAlignment_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setVerticalAlignment_0(self);
    // return 1;
  }
}
pub trait QTextCharFormat_setVerticalAlignment_0<RetType> {
  fn setVerticalAlignment_0(self , rsthis: & QTextCharFormat) -> RetType;
}
impl<'a> /*trait*/ QTextCharFormat_setVerticalAlignment_0<(/*void*/)> for (i32) {
  fn setVerticalAlignment_0(self , rsthis: & QTextCharFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN15QTextCharFormat20setVerticalAlignmentENS_17VerticalAlignmentE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:518
// index:0
// Public inline Visibility=Default Availability=Available
// [4] QTextCharFormat::VerticalAlignment verticalAlignment() const

/*

*/
impl /*struct*/ QTextCharFormat {
  pub fn verticalAlignment_0<RetType, T: QTextCharFormat_verticalAlignment_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.verticalAlignment_0(self);
    // return 1;
  }
}
pub trait QTextCharFormat_verticalAlignment_0<RetType> {
  fn verticalAlignment_0(self , rsthis: & QTextCharFormat) -> RetType;
}
impl<'a> /*trait*/ QTextCharFormat_verticalAlignment_0<i32> for () {
  fn verticalAlignment_0(self , rsthis: & QTextCharFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QTextCharFormat17verticalAlignmentEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:521
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setTextOutline(const QPen &)

/*

*/
impl /*struct*/ QTextCharFormat {
  pub fn setTextOutline_0<RetType, T: QTextCharFormat_setTextOutline_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTextOutline_0(self);
    // return 1;
  }
}
pub trait QTextCharFormat_setTextOutline_0<RetType> {
  fn setTextOutline_0(self , rsthis: & QTextCharFormat) -> RetType;
}
impl<'a> /*trait*/ QTextCharFormat_setTextOutline_0<(/*void*/)> for (usize) {
  fn setTextOutline_0(self , rsthis: & QTextCharFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QTextCharFormat14setTextOutlineERK4QPen", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:523
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QPen textOutline() const

/*

*/
impl /*struct*/ QTextCharFormat {
  pub fn textOutline_0<RetType, T: QTextCharFormat_textOutline_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.textOutline_0(self);
    // return 1;
  }
}
pub trait QTextCharFormat_textOutline_0<RetType> {
  fn textOutline_0(self , rsthis: & QTextCharFormat) -> RetType;
}
impl<'a> /*trait*/ QTextCharFormat_textOutline_0<usize> for () {
  fn textOutline_0(self , rsthis: & QTextCharFormat) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QTextCharFormat11textOutlineEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:526
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setToolTip(const QString &)

/*

*/
impl /*struct*/ QTextCharFormat {
  pub fn setToolTip_0<RetType, T: QTextCharFormat_setToolTip_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setToolTip_0(self);
    // return 1;
  }
}
pub trait QTextCharFormat_setToolTip_0<RetType> {
  fn setToolTip_0(self , rsthis: & QTextCharFormat) -> RetType;
}
impl<'a> /*trait*/ QTextCharFormat_setToolTip_0<(/*void*/)> for (usize) {
  fn setToolTip_0(self , rsthis: & QTextCharFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QTextCharFormat10setToolTipERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:528
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QString toolTip() const

/*

*/
impl /*struct*/ QTextCharFormat {
  pub fn toolTip_0<RetType, T: QTextCharFormat_toolTip_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toolTip_0(self);
    // return 1;
  }
}
pub trait QTextCharFormat_toolTip_0<RetType> {
  fn toolTip_0(self , rsthis: & QTextCharFormat) -> RetType;
}
impl<'a> /*trait*/ QTextCharFormat_toolTip_0<usize> for () {
  fn toolTip_0(self , rsthis: & QTextCharFormat) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QTextCharFormat7toolTipEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:531
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setAnchor(bool)

/*

*/
impl /*struct*/ QTextCharFormat {
  pub fn setAnchor_0<RetType, T: QTextCharFormat_setAnchor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setAnchor_0(self);
    // return 1;
  }
}
pub trait QTextCharFormat_setAnchor_0<RetType> {
  fn setAnchor_0(self , rsthis: & QTextCharFormat) -> RetType;
}
impl<'a> /*trait*/ QTextCharFormat_setAnchor_0<(/*void*/)> for (bool) {
  fn setAnchor_0(self , rsthis: & QTextCharFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN15QTextCharFormat9setAnchorEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:533
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isAnchor() const

/*

*/
impl /*struct*/ QTextCharFormat {
  pub fn isAnchor_0<RetType, T: QTextCharFormat_isAnchor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isAnchor_0(self);
    // return 1;
  }
}
pub trait QTextCharFormat_isAnchor_0<RetType> {
  fn isAnchor_0(self , rsthis: & QTextCharFormat) -> RetType;
}
impl<'a> /*trait*/ QTextCharFormat_isAnchor_0<bool> for () {
  fn isAnchor_0(self , rsthis: & QTextCharFormat) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QTextCharFormat8isAnchorEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:536
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setAnchorHref(const QString &)

/*

*/
impl /*struct*/ QTextCharFormat {
  pub fn setAnchorHref_0<RetType, T: QTextCharFormat_setAnchorHref_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setAnchorHref_0(self);
    // return 1;
  }
}
pub trait QTextCharFormat_setAnchorHref_0<RetType> {
  fn setAnchorHref_0(self , rsthis: & QTextCharFormat) -> RetType;
}
impl<'a> /*trait*/ QTextCharFormat_setAnchorHref_0<(/*void*/)> for (usize) {
  fn setAnchorHref_0(self , rsthis: & QTextCharFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QTextCharFormat13setAnchorHrefERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:538
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QString anchorHref() const

/*

*/
impl /*struct*/ QTextCharFormat {
  pub fn anchorHref_0<RetType, T: QTextCharFormat_anchorHref_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.anchorHref_0(self);
    // return 1;
  }
}
pub trait QTextCharFormat_anchorHref_0<RetType> {
  fn anchorHref_0(self , rsthis: & QTextCharFormat) -> RetType;
}
impl<'a> /*trait*/ QTextCharFormat_anchorHref_0<usize> for () {
  fn anchorHref_0(self , rsthis: & QTextCharFormat) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QTextCharFormat10anchorHrefEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:541
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setAnchorName(const QString &)

/*

*/
impl /*struct*/ QTextCharFormat {
  pub fn setAnchorName_0<RetType, T: QTextCharFormat_setAnchorName_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setAnchorName_0(self);
    // return 1;
  }
}
pub trait QTextCharFormat_setAnchorName_0<RetType> {
  fn setAnchorName_0(self , rsthis: & QTextCharFormat) -> RetType;
}
impl<'a> /*trait*/ QTextCharFormat_setAnchorName_0<(/*void*/)> for (usize) {
  fn setAnchorName_0(self , rsthis: & QTextCharFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QTextCharFormat13setAnchorNameERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:543
// index:0
// Public Visibility=Default Availability=Available
// [8] QString anchorName() const

/*

*/
impl /*struct*/ QTextCharFormat {
  pub fn anchorName_0<RetType, T: QTextCharFormat_anchorName_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.anchorName_0(self);
    // return 1;
  }
}
pub trait QTextCharFormat_anchorName_0<RetType> {
  fn anchorName_0(self , rsthis: & QTextCharFormat) -> RetType;
}
impl<'a> /*trait*/ QTextCharFormat_anchorName_0<usize> for () {
  fn anchorName_0(self , rsthis: & QTextCharFormat) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QTextCharFormat10anchorNameEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:545
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setAnchorNames(const QStringList &)

/*

*/
impl /*struct*/ QTextCharFormat {
  pub fn setAnchorNames_0<RetType, T: QTextCharFormat_setAnchorNames_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setAnchorNames_0(self);
    // return 1;
  }
}
pub trait QTextCharFormat_setAnchorNames_0<RetType> {
  fn setAnchorNames_0(self , rsthis: & QTextCharFormat) -> RetType;
}
impl<'a> /*trait*/ QTextCharFormat_setAnchorNames_0<(/*void*/)> for (usize) {
  fn setAnchorNames_0(self , rsthis: & QTextCharFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QTextCharFormat14setAnchorNamesERK11QStringList", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:547
// index:0
// Public Visibility=Default Availability=Available
// [8] QStringList anchorNames() const

/*

*/
impl /*struct*/ QTextCharFormat {
  pub fn anchorNames_0<RetType, T: QTextCharFormat_anchorNames_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.anchorNames_0(self);
    // return 1;
  }
}
pub trait QTextCharFormat_anchorNames_0<RetType> {
  fn anchorNames_0(self , rsthis: & QTextCharFormat) -> RetType;
}
impl<'a> /*trait*/ QTextCharFormat_anchorNames_0<usize> for () {
  fn anchorNames_0(self , rsthis: & QTextCharFormat) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QTextCharFormat11anchorNamesEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:549
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setTableCellRowSpan(int)

/*

*/
impl /*struct*/ QTextCharFormat {
  pub fn setTableCellRowSpan_0<RetType, T: QTextCharFormat_setTableCellRowSpan_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTableCellRowSpan_0(self);
    // return 1;
  }
}
pub trait QTextCharFormat_setTableCellRowSpan_0<RetType> {
  fn setTableCellRowSpan_0(self , rsthis: & QTextCharFormat) -> RetType;
}
impl<'a> /*trait*/ QTextCharFormat_setTableCellRowSpan_0<(/*void*/)> for (i32) {
  fn setTableCellRowSpan_0(self , rsthis: & QTextCharFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN15QTextCharFormat19setTableCellRowSpanEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:550
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int tableCellRowSpan() const

/*

*/
impl /*struct*/ QTextCharFormat {
  pub fn tableCellRowSpan_0<RetType, T: QTextCharFormat_tableCellRowSpan_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.tableCellRowSpan_0(self);
    // return 1;
  }
}
pub trait QTextCharFormat_tableCellRowSpan_0<RetType> {
  fn tableCellRowSpan_0(self , rsthis: & QTextCharFormat) -> RetType;
}
impl<'a> /*trait*/ QTextCharFormat_tableCellRowSpan_0<i32> for () {
  fn tableCellRowSpan_0(self , rsthis: & QTextCharFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QTextCharFormat16tableCellRowSpanEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:552
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setTableCellColumnSpan(int)

/*

*/
impl /*struct*/ QTextCharFormat {
  pub fn setTableCellColumnSpan_0<RetType, T: QTextCharFormat_setTableCellColumnSpan_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTableCellColumnSpan_0(self);
    // return 1;
  }
}
pub trait QTextCharFormat_setTableCellColumnSpan_0<RetType> {
  fn setTableCellColumnSpan_0(self , rsthis: & QTextCharFormat) -> RetType;
}
impl<'a> /*trait*/ QTextCharFormat_setTableCellColumnSpan_0<(/*void*/)> for (i32) {
  fn setTableCellColumnSpan_0(self , rsthis: & QTextCharFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN15QTextCharFormat22setTableCellColumnSpanEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:553
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int tableCellColumnSpan() const

/*

*/
impl /*struct*/ QTextCharFormat {
  pub fn tableCellColumnSpan_0<RetType, T: QTextCharFormat_tableCellColumnSpan_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.tableCellColumnSpan_0(self);
    // return 1;
  }
}
pub trait QTextCharFormat_tableCellColumnSpan_0<RetType> {
  fn tableCellColumnSpan_0(self , rsthis: & QTextCharFormat) -> RetType;
}
impl<'a> /*trait*/ QTextCharFormat_tableCellColumnSpan_0<i32> for () {
  fn tableCellColumnSpan_0(self , rsthis: & QTextCharFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QTextCharFormat19tableCellColumnSpanEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}


pub fn DeleteQTextCharFormat(this :*mut QTextCharFormat) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN15QTextCharFormatD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}

/*


*/
pub type QTextCharFormat__VerticalAlignment = i32;
// 
pub const QTextCharFormat__AlignNormal :QTextCharFormat__VerticalAlignment = 0;
// 
pub const QTextCharFormat__AlignSuperScript :QTextCharFormat__VerticalAlignment = 1;
// 
pub const QTextCharFormat__AlignSubScript :QTextCharFormat__VerticalAlignment = 2;
// 
pub const QTextCharFormat__AlignMiddle :QTextCharFormat__VerticalAlignment = 3;
// 
pub const QTextCharFormat__AlignTop :QTextCharFormat__VerticalAlignment = 4;
// 
pub const QTextCharFormat__AlignBottom :QTextCharFormat__VerticalAlignment = 5;
// 
pub const QTextCharFormat__AlignBaseline :QTextCharFormat__VerticalAlignment = 6;
pub fn QTextCharFormat_VerticalAlignmentItemName(val: i32) ->String {
  match val {
     QTextCharFormat__AlignNormal => // 0
     {return String::from("AlignNormal");}
     QTextCharFormat__AlignSuperScript => // 1
     {return String::from("AlignSuperScript");}
     QTextCharFormat__AlignSubScript => // 2
     {return String::from("AlignSubScript");}
     QTextCharFormat__AlignMiddle => // 3
     {return String::from("AlignMiddle");}
     QTextCharFormat__AlignTop => // 4
     {return String::from("AlignTop");}
     QTextCharFormat__AlignBottom => // 5
     {return String::from("AlignBottom");}
     QTextCharFormat__AlignBaseline => // 6
     {return String::from("AlignBaseline");}
  _ => {return format!("{}", val);}
}
}
pub fn QTextCharFormat_VerticalAlignmentItemName_s(val: i32) ->String {
  //var nilthis *QTextCharFormat
  //return nilthis.VerticalAlignmentItemName(val);
  return QTextCharFormat_VerticalAlignmentItemName(val);
}


/*


*/
pub type QTextCharFormat__UnderlineStyle = i32;
// 
pub const QTextCharFormat__NoUnderline :QTextCharFormat__UnderlineStyle = 0;
// 
pub const QTextCharFormat__SingleUnderline :QTextCharFormat__UnderlineStyle = 1;
// 
pub const QTextCharFormat__DashUnderline :QTextCharFormat__UnderlineStyle = 2;
// 
pub const QTextCharFormat__DotLine :QTextCharFormat__UnderlineStyle = 3;
// 
pub const QTextCharFormat__DashDotLine :QTextCharFormat__UnderlineStyle = 4;
// 
pub const QTextCharFormat__DashDotDotLine :QTextCharFormat__UnderlineStyle = 5;
// 
pub const QTextCharFormat__WaveUnderline :QTextCharFormat__UnderlineStyle = 6;
// 
pub const QTextCharFormat__SpellCheckUnderline :QTextCharFormat__UnderlineStyle = 7;
pub fn QTextCharFormat_UnderlineStyleItemName(val: i32) ->String {
  match val {
     QTextCharFormat__NoUnderline => // 0
     {return String::from("NoUnderline");}
     QTextCharFormat__SingleUnderline => // 1
     {return String::from("SingleUnderline");}
     QTextCharFormat__DashUnderline => // 2
     {return String::from("DashUnderline");}
     QTextCharFormat__DotLine => // 3
     {return String::from("DotLine");}
     QTextCharFormat__DashDotLine => // 4
     {return String::from("DashDotLine");}
     QTextCharFormat__DashDotDotLine => // 5
     {return String::from("DashDotDotLine");}
     QTextCharFormat__WaveUnderline => // 6
     {return String::from("WaveUnderline");}
     QTextCharFormat__SpellCheckUnderline => // 7
     {return String::from("SpellCheckUnderline");}
  _ => {return format!("{}", val);}
}
}
pub fn QTextCharFormat_UnderlineStyleItemName_s(val: i32) ->String {
  //var nilthis *QTextCharFormat
  //return nilthis.UnderlineStyleItemName(val);
  return QTextCharFormat_UnderlineStyleItemName(val);
}


/*


*/
pub type QTextCharFormat__FontPropertiesInheritanceBehavior = i32;
// 
pub const QTextCharFormat__FontPropertiesSpecifiedOnly :QTextCharFormat__FontPropertiesInheritanceBehavior = 0;
// 
pub const QTextCharFormat__FontPropertiesAll :QTextCharFormat__FontPropertiesInheritanceBehavior = 1;
pub fn QTextCharFormat_FontPropertiesInheritanceBehaviorItemName(val: i32) ->String {
  match val {
     QTextCharFormat__FontPropertiesSpecifiedOnly => // 0
     {return String::from("FontPropertiesSpecifiedOnly");}
     QTextCharFormat__FontPropertiesAll => // 1
     {return String::from("FontPropertiesAll");}
  _ => {return format!("{}", val);}
}
}
pub fn QTextCharFormat_FontPropertiesInheritanceBehaviorItemName_s(val: i32) ->String {
  //var nilthis *QTextCharFormat
  //return nilthis.FontPropertiesInheritanceBehaviorItemName(val);
  return QTextCharFormat_FontPropertiesInheritanceBehaviorItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
