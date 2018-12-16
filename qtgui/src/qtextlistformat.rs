

// mod ::gui::QTextListFormat
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
// extern C begin: 25
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
#[derive(Default)] // class sizeof(QTextListFormat)=16
pub struct QTextListFormat {
  qbase: QTextFormat,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QTextListFormat_ITF interface {
//    QTextFormat_ITF
//    QTextListFormat_PTR() *QTextListFormat
//}
//func (ptr *QTextListFormat) QTextListFormat_PTR() *QTextListFormat { return ptr }

impl /*struct*/ QTextListFormat {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QTextListFormat {
    return QTextListFormat{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QTextListFormat {
//  type Target = QTextListFormatBASE;
//
//  fn deref(&self) -> &QTextListFormatBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QTextListFormatBASE> for QTextListFormat {
//  fn as_ref(& self) -> & QTextListFormatBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qtextformat.h:681
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QTextListFormat()

/*

*/
// QTextListFormat() ctx.fn_proto_cpp
impl /*struct*/ QTextListFormat {
  pub fn QTextListFormat_0<T: QTextListFormat_QTextListFormat_0>(value: T) -> QTextListFormat {
    let rsthis = value.QTextListFormat_0();
    return rsthis;
    // return 1;
  }
}

pub trait QTextListFormat_QTextListFormat_0 {
  fn QTextListFormat_0(self) -> QTextListFormat;
}
// QTextListFormat() ctx.fn_proto_cpp
impl<'a> /*trait*/ QTextListFormat_QTextListFormat_0 for () {
  fn QTextListFormat_0(self) -> QTextListFormat {
    // unsafe{_ZN15QTextListFormatC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN15QTextListFormatC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTextListFormat{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:714
// index:1
// Protected Visibility=Default Availability=Available
// [-2] void QTextListFormat(const QTextFormat &)

/*

*/
// QTextListFormat(const QTextFormat &) ctx.fn_proto_cpp
impl /*struct*/ QTextListFormat {
  pub fn QTextListFormat_1<T: QTextListFormat_QTextListFormat_1>(value: T) -> QTextListFormat {
    let rsthis = value.QTextListFormat_1();
    return rsthis;
    // return 1;
  }
}

pub trait QTextListFormat_QTextListFormat_1 {
  fn QTextListFormat_1(self) -> QTextListFormat;
}
// QTextListFormat(const QTextFormat &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QTextListFormat_QTextListFormat_1 for (usize) {
  fn QTextListFormat_1(self) -> QTextListFormat {
    // unsafe{_ZN15QTextListFormatC2ERK11QTextFormat()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN15QTextListFormatC2ERK11QTextFormat", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTextListFormat{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:683
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isValid() const

/*
Returns true if the format is valid (i.e. is not InvalidFormat); otherwise returns false.
*/
impl /*struct*/ QTextListFormat {
  pub fn isValid_0<RetType, T: QTextListFormat_isValid_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isValid_0(self);
    // return 1;
  }
}
pub trait QTextListFormat_isValid_0<RetType> {
  fn isValid_0(self , rsthis: & QTextListFormat) -> RetType;
}
impl<'a> /*trait*/ QTextListFormat_isValid_0<bool> for () {
  fn isValid_0(self , rsthis: & QTextListFormat) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QTextListFormat7isValidEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:697
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setStyle(QTextListFormat::Style)

/*

*/
impl /*struct*/ QTextListFormat {
  pub fn setStyle_0<RetType, T: QTextListFormat_setStyle_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setStyle_0(self);
    // return 1;
  }
}
pub trait QTextListFormat_setStyle_0<RetType> {
  fn setStyle_0(self , rsthis: & QTextListFormat) -> RetType;
}
impl<'a> /*trait*/ QTextListFormat_setStyle_0<(/*void*/)> for (i32) {
  fn setStyle_0(self , rsthis: & QTextListFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN15QTextListFormat8setStyleENS_5StyleE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:698
// index:0
// Public inline Visibility=Default Availability=Available
// [4] QTextListFormat::Style style() const

/*

*/
impl /*struct*/ QTextListFormat {
  pub fn style_0<RetType, T: QTextListFormat_style_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.style_0(self);
    // return 1;
  }
}
pub trait QTextListFormat_style_0<RetType> {
  fn style_0(self , rsthis: & QTextListFormat) -> RetType;
}
impl<'a> /*trait*/ QTextListFormat_style_0<i32> for () {
  fn style_0(self , rsthis: & QTextListFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QTextListFormat5styleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:701
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setIndent(int)

/*

*/
impl /*struct*/ QTextListFormat {
  pub fn setIndent_0<RetType, T: QTextListFormat_setIndent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setIndent_0(self);
    // return 1;
  }
}
pub trait QTextListFormat_setIndent_0<RetType> {
  fn setIndent_0(self , rsthis: & QTextListFormat) -> RetType;
}
impl<'a> /*trait*/ QTextListFormat_setIndent_0<(/*void*/)> for (i32) {
  fn setIndent_0(self , rsthis: & QTextListFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN15QTextListFormat9setIndentEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:702
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int indent() const

/*

*/
impl /*struct*/ QTextListFormat {
  pub fn indent_0<RetType, T: QTextListFormat_indent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.indent_0(self);
    // return 1;
  }
}
pub trait QTextListFormat_indent_0<RetType> {
  fn indent_0(self , rsthis: & QTextListFormat) -> RetType;
}
impl<'a> /*trait*/ QTextListFormat_indent_0<i32> for () {
  fn indent_0(self , rsthis: & QTextListFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QTextListFormat6indentEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:705
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setNumberPrefix(const QString &)

/*

*/
impl /*struct*/ QTextListFormat {
  pub fn setNumberPrefix_0<RetType, T: QTextListFormat_setNumberPrefix_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setNumberPrefix_0(self);
    // return 1;
  }
}
pub trait QTextListFormat_setNumberPrefix_0<RetType> {
  fn setNumberPrefix_0(self , rsthis: & QTextListFormat) -> RetType;
}
impl<'a> /*trait*/ QTextListFormat_setNumberPrefix_0<(/*void*/)> for (usize) {
  fn setNumberPrefix_0(self , rsthis: & QTextListFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QTextListFormat15setNumberPrefixERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:706
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QString numberPrefix() const

/*

*/
impl /*struct*/ QTextListFormat {
  pub fn numberPrefix_0<RetType, T: QTextListFormat_numberPrefix_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.numberPrefix_0(self);
    // return 1;
  }
}
pub trait QTextListFormat_numberPrefix_0<RetType> {
  fn numberPrefix_0(self , rsthis: & QTextListFormat) -> RetType;
}
impl<'a> /*trait*/ QTextListFormat_numberPrefix_0<usize> for () {
  fn numberPrefix_0(self , rsthis: & QTextListFormat) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QTextListFormat12numberPrefixEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:709
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setNumberSuffix(const QString &)

/*

*/
impl /*struct*/ QTextListFormat {
  pub fn setNumberSuffix_0<RetType, T: QTextListFormat_setNumberSuffix_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setNumberSuffix_0(self);
    // return 1;
  }
}
pub trait QTextListFormat_setNumberSuffix_0<RetType> {
  fn setNumberSuffix_0(self , rsthis: & QTextListFormat) -> RetType;
}
impl<'a> /*trait*/ QTextListFormat_setNumberSuffix_0<(/*void*/)> for (usize) {
  fn setNumberSuffix_0(self , rsthis: & QTextListFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QTextListFormat15setNumberSuffixERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:710
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QString numberSuffix() const

/*

*/
impl /*struct*/ QTextListFormat {
  pub fn numberSuffix_0<RetType, T: QTextListFormat_numberSuffix_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.numberSuffix_0(self);
    // return 1;
  }
}
pub trait QTextListFormat_numberSuffix_0<RetType> {
  fn numberSuffix_0(self , rsthis: & QTextListFormat) -> RetType;
}
impl<'a> /*trait*/ QTextListFormat_numberSuffix_0<usize> for () {
  fn numberSuffix_0(self , rsthis: & QTextListFormat) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QTextListFormat12numberSuffixEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}


pub fn DeleteQTextListFormat(this :*mut QTextListFormat) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN15QTextListFormatD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}

/*


*/
pub type QTextListFormat__Style = i32;
// 
pub const QTextListFormat__ListDisc :QTextListFormat__Style = -1;
// 
pub const QTextListFormat__ListCircle :QTextListFormat__Style = -2;
// 
pub const QTextListFormat__ListSquare :QTextListFormat__Style = -3;
// 
pub const QTextListFormat__ListDecimal :QTextListFormat__Style = -4;
// 
pub const QTextListFormat__ListLowerAlpha :QTextListFormat__Style = -5;
// 
pub const QTextListFormat__ListUpperAlpha :QTextListFormat__Style = -6;
// 
pub const QTextListFormat__ListLowerRoman :QTextListFormat__Style = -7;
// 
pub const QTextListFormat__ListUpperRoman :QTextListFormat__Style = -8;
// 
pub const QTextListFormat__ListStyleUndefined :QTextListFormat__Style = 0;
pub fn QTextListFormat_StyleItemName(val: i32) ->String {
  match val {
     QTextListFormat__ListDisc => // -1
     {return String::from("ListDisc");}
     QTextListFormat__ListCircle => // -2
     {return String::from("ListCircle");}
     QTextListFormat__ListSquare => // -3
     {return String::from("ListSquare");}
     QTextListFormat__ListDecimal => // -4
     {return String::from("ListDecimal");}
     QTextListFormat__ListLowerAlpha => // -5
     {return String::from("ListLowerAlpha");}
     QTextListFormat__ListUpperAlpha => // -6
     {return String::from("ListUpperAlpha");}
     QTextListFormat__ListLowerRoman => // -7
     {return String::from("ListLowerRoman");}
     QTextListFormat__ListUpperRoman => // -8
     {return String::from("ListUpperRoman");}
     QTextListFormat__ListStyleUndefined => // 0
     {return String::from("ListStyleUndefined");}
  _ => {return format!("{}", val);}
}
}
pub fn QTextListFormat_StyleItemName_s(val: i32) ->String {
  //var nilthis *QTextListFormat
  //return nilthis.StyleItemName(val);
  return QTextListFormat_StyleItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
