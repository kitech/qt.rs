

// mod ::gui::QTextBlockFormat
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
// extern C begin: 62
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
#[derive(Default)] // class sizeof(QTextBlockFormat)=16
pub struct QTextBlockFormat {
  qbase: QTextFormat,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QTextBlockFormat_ITF interface {
//    QTextFormat_ITF
//    QTextBlockFormat_PTR() *QTextBlockFormat
//}
//func (ptr *QTextBlockFormat) QTextBlockFormat_PTR() *QTextBlockFormat { return ptr }

impl /*struct*/ QTextBlockFormat {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QTextBlockFormat {
    return QTextBlockFormat{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QTextBlockFormat {
//  type Target = QTextBlockFormatBASE;
//
//  fn deref(&self) -> &QTextBlockFormatBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QTextBlockFormatBASE> for QTextBlockFormat {
//  fn as_ref(& self) -> & QTextBlockFormatBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qtextformat.h:590
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QTextBlockFormat()

/*

*/
// QTextBlockFormat() ctx.fn_proto_cpp
impl /*struct*/ QTextBlockFormat {
  pub fn QTextBlockFormat_0<T: QTextBlockFormat_QTextBlockFormat_0>(value: T) -> QTextBlockFormat {
    let rsthis = value.QTextBlockFormat_0();
    return rsthis;
    // return 1;
  }
}

pub trait QTextBlockFormat_QTextBlockFormat_0 {
  fn QTextBlockFormat_0(self) -> QTextBlockFormat;
}
// QTextBlockFormat() ctx.fn_proto_cpp
impl<'a> /*trait*/ QTextBlockFormat_QTextBlockFormat_0 for () {
  fn QTextBlockFormat_0(self) -> QTextBlockFormat {
    // unsafe{_ZN16QTextBlockFormatC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN16QTextBlockFormatC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTextBlockFormat{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:649
// index:1
// Protected Visibility=Default Availability=Available
// [-2] void QTextBlockFormat(const QTextFormat &)

/*

*/
// QTextBlockFormat(const QTextFormat &) ctx.fn_proto_cpp
impl /*struct*/ QTextBlockFormat {
  pub fn QTextBlockFormat_1<T: QTextBlockFormat_QTextBlockFormat_1>(value: T) -> QTextBlockFormat {
    let rsthis = value.QTextBlockFormat_1();
    return rsthis;
    // return 1;
  }
}

pub trait QTextBlockFormat_QTextBlockFormat_1 {
  fn QTextBlockFormat_1(self) -> QTextBlockFormat;
}
// QTextBlockFormat(const QTextFormat &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QTextBlockFormat_QTextBlockFormat_1 for (usize) {
  fn QTextBlockFormat_1(self) -> QTextBlockFormat {
    // unsafe{_ZN16QTextBlockFormatC2ERK11QTextFormat()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN16QTextBlockFormatC2ERK11QTextFormat", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTextBlockFormat{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:592
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isValid() const

/*
Returns true if the format is valid (i.e. is not InvalidFormat); otherwise returns false.
*/
impl /*struct*/ QTextBlockFormat {
  pub fn isValid_0<RetType, T: QTextBlockFormat_isValid_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isValid_0(self);
    // return 1;
  }
}
pub trait QTextBlockFormat_isValid_0<RetType> {
  fn isValid_0(self , rsthis: & QTextBlockFormat) -> RetType;
}
impl<'a> /*trait*/ QTextBlockFormat_isValid_0<bool> for () {
  fn isValid_0(self , rsthis: & QTextBlockFormat) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QTextBlockFormat7isValidEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:594
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setAlignment(Qt::Alignment)

/*

*/
impl /*struct*/ QTextBlockFormat {
  pub fn setAlignment_0<RetType, T: QTextBlockFormat_setAlignment_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setAlignment_0(self);
    // return 1;
  }
}
pub trait QTextBlockFormat_setAlignment_0<RetType> {
  fn setAlignment_0(self , rsthis: & QTextBlockFormat) -> RetType;
}
impl<'a> /*trait*/ QTextBlockFormat_setAlignment_0<(/*void*/)> for (i32) {
  fn setAlignment_0(self , rsthis: & QTextBlockFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN16QTextBlockFormat12setAlignmentE6QFlagsIN2Qt13AlignmentFlagEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:595
// index:0
// Public inline Visibility=Default Availability=Available
// [4] Qt::Alignment alignment() const

/*

*/
impl /*struct*/ QTextBlockFormat {
  pub fn alignment_0<RetType, T: QTextBlockFormat_alignment_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.alignment_0(self);
    // return 1;
  }
}
pub trait QTextBlockFormat_alignment_0<RetType> {
  fn alignment_0(self , rsthis: & QTextBlockFormat) -> RetType;
}
impl<'a> /*trait*/ QTextBlockFormat_alignment_0<i32> for () {
  fn alignment_0(self , rsthis: & QTextBlockFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QTextBlockFormat9alignmentEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:598
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setTopMargin(qreal)

/*

*/
impl /*struct*/ QTextBlockFormat {
  pub fn setTopMargin_0<RetType, T: QTextBlockFormat_setTopMargin_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTopMargin_0(self);
    // return 1;
  }
}
pub trait QTextBlockFormat_setTopMargin_0<RetType> {
  fn setTopMargin_0(self , rsthis: & QTextBlockFormat) -> RetType;
}
impl<'a> /*trait*/ QTextBlockFormat_setTopMargin_0<(/*void*/)> for (f64) {
  fn setTopMargin_0(self , rsthis: & QTextBlockFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN16QTextBlockFormat12setTopMarginEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:600
// index:0
// Public inline Visibility=Default Availability=Available
// [8] qreal topMargin() const

/*

*/
impl /*struct*/ QTextBlockFormat {
  pub fn topMargin_0<RetType, T: QTextBlockFormat_topMargin_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.topMargin_0(self);
    // return 1;
  }
}
pub trait QTextBlockFormat_topMargin_0<RetType> {
  fn topMargin_0(self , rsthis: & QTextBlockFormat) -> RetType;
}
impl<'a> /*trait*/ QTextBlockFormat_topMargin_0<f64> for () {
  fn topMargin_0(self , rsthis: & QTextBlockFormat) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QTextBlockFormat9topMarginEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:603
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setBottomMargin(qreal)

/*

*/
impl /*struct*/ QTextBlockFormat {
  pub fn setBottomMargin_0<RetType, T: QTextBlockFormat_setBottomMargin_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setBottomMargin_0(self);
    // return 1;
  }
}
pub trait QTextBlockFormat_setBottomMargin_0<RetType> {
  fn setBottomMargin_0(self , rsthis: & QTextBlockFormat) -> RetType;
}
impl<'a> /*trait*/ QTextBlockFormat_setBottomMargin_0<(/*void*/)> for (f64) {
  fn setBottomMargin_0(self , rsthis: & QTextBlockFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN16QTextBlockFormat15setBottomMarginEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:605
// index:0
// Public inline Visibility=Default Availability=Available
// [8] qreal bottomMargin() const

/*

*/
impl /*struct*/ QTextBlockFormat {
  pub fn bottomMargin_0<RetType, T: QTextBlockFormat_bottomMargin_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.bottomMargin_0(self);
    // return 1;
  }
}
pub trait QTextBlockFormat_bottomMargin_0<RetType> {
  fn bottomMargin_0(self , rsthis: & QTextBlockFormat) -> RetType;
}
impl<'a> /*trait*/ QTextBlockFormat_bottomMargin_0<f64> for () {
  fn bottomMargin_0(self , rsthis: & QTextBlockFormat) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QTextBlockFormat12bottomMarginEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:608
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setLeftMargin(qreal)

/*

*/
impl /*struct*/ QTextBlockFormat {
  pub fn setLeftMargin_0<RetType, T: QTextBlockFormat_setLeftMargin_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setLeftMargin_0(self);
    // return 1;
  }
}
pub trait QTextBlockFormat_setLeftMargin_0<RetType> {
  fn setLeftMargin_0(self , rsthis: & QTextBlockFormat) -> RetType;
}
impl<'a> /*trait*/ QTextBlockFormat_setLeftMargin_0<(/*void*/)> for (f64) {
  fn setLeftMargin_0(self , rsthis: & QTextBlockFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN16QTextBlockFormat13setLeftMarginEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:610
// index:0
// Public inline Visibility=Default Availability=Available
// [8] qreal leftMargin() const

/*

*/
impl /*struct*/ QTextBlockFormat {
  pub fn leftMargin_0<RetType, T: QTextBlockFormat_leftMargin_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.leftMargin_0(self);
    // return 1;
  }
}
pub trait QTextBlockFormat_leftMargin_0<RetType> {
  fn leftMargin_0(self , rsthis: & QTextBlockFormat) -> RetType;
}
impl<'a> /*trait*/ QTextBlockFormat_leftMargin_0<f64> for () {
  fn leftMargin_0(self , rsthis: & QTextBlockFormat) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QTextBlockFormat10leftMarginEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:613
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setRightMargin(qreal)

/*

*/
impl /*struct*/ QTextBlockFormat {
  pub fn setRightMargin_0<RetType, T: QTextBlockFormat_setRightMargin_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setRightMargin_0(self);
    // return 1;
  }
}
pub trait QTextBlockFormat_setRightMargin_0<RetType> {
  fn setRightMargin_0(self , rsthis: & QTextBlockFormat) -> RetType;
}
impl<'a> /*trait*/ QTextBlockFormat_setRightMargin_0<(/*void*/)> for (f64) {
  fn setRightMargin_0(self , rsthis: & QTextBlockFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN16QTextBlockFormat14setRightMarginEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:615
// index:0
// Public inline Visibility=Default Availability=Available
// [8] qreal rightMargin() const

/*

*/
impl /*struct*/ QTextBlockFormat {
  pub fn rightMargin_0<RetType, T: QTextBlockFormat_rightMargin_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rightMargin_0(self);
    // return 1;
  }
}
pub trait QTextBlockFormat_rightMargin_0<RetType> {
  fn rightMargin_0(self , rsthis: & QTextBlockFormat) -> RetType;
}
impl<'a> /*trait*/ QTextBlockFormat_rightMargin_0<f64> for () {
  fn rightMargin_0(self , rsthis: & QTextBlockFormat) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QTextBlockFormat11rightMarginEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:618
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setTextIndent(qreal)

/*

*/
impl /*struct*/ QTextBlockFormat {
  pub fn setTextIndent_0<RetType, T: QTextBlockFormat_setTextIndent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTextIndent_0(self);
    // return 1;
  }
}
pub trait QTextBlockFormat_setTextIndent_0<RetType> {
  fn setTextIndent_0(self , rsthis: & QTextBlockFormat) -> RetType;
}
impl<'a> /*trait*/ QTextBlockFormat_setTextIndent_0<(/*void*/)> for (f64) {
  fn setTextIndent_0(self , rsthis: & QTextBlockFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN16QTextBlockFormat13setTextIndentEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:620
// index:0
// Public inline Visibility=Default Availability=Available
// [8] qreal textIndent() const

/*

*/
impl /*struct*/ QTextBlockFormat {
  pub fn textIndent_0<RetType, T: QTextBlockFormat_textIndent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.textIndent_0(self);
    // return 1;
  }
}
pub trait QTextBlockFormat_textIndent_0<RetType> {
  fn textIndent_0(self , rsthis: & QTextBlockFormat) -> RetType;
}
impl<'a> /*trait*/ QTextBlockFormat_textIndent_0<f64> for () {
  fn textIndent_0(self , rsthis: & QTextBlockFormat) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QTextBlockFormat10textIndentEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:623
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setIndent(int)

/*

*/
impl /*struct*/ QTextBlockFormat {
  pub fn setIndent_0<RetType, T: QTextBlockFormat_setIndent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setIndent_0(self);
    // return 1;
  }
}
pub trait QTextBlockFormat_setIndent_0<RetType> {
  fn setIndent_0(self , rsthis: & QTextBlockFormat) -> RetType;
}
impl<'a> /*trait*/ QTextBlockFormat_setIndent_0<(/*void*/)> for (i32) {
  fn setIndent_0(self , rsthis: & QTextBlockFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN16QTextBlockFormat9setIndentEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:624
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int indent() const

/*

*/
impl /*struct*/ QTextBlockFormat {
  pub fn indent_0<RetType, T: QTextBlockFormat_indent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.indent_0(self);
    // return 1;
  }
}
pub trait QTextBlockFormat_indent_0<RetType> {
  fn indent_0(self , rsthis: & QTextBlockFormat) -> RetType;
}
impl<'a> /*trait*/ QTextBlockFormat_indent_0<i32> for () {
  fn indent_0(self , rsthis: & QTextBlockFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QTextBlockFormat6indentEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:627
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setLineHeight(qreal, int)

/*

*/
impl /*struct*/ QTextBlockFormat {
  pub fn setLineHeight_0<RetType, T: QTextBlockFormat_setLineHeight_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setLineHeight_0(self);
    // return 1;
  }
}
pub trait QTextBlockFormat_setLineHeight_0<RetType> {
  fn setLineHeight_0(self , rsthis: & QTextBlockFormat) -> RetType;
}
impl<'a> /*trait*/ QTextBlockFormat_setLineHeight_0<(/*void*/)> for (f64,i32) {
  fn setLineHeight_0(self , rsthis: & QTextBlockFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN16QTextBlockFormat13setLineHeightEdi", 2,qtrt::FFITY_DOUBLE,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:629
// index:0
// Public inline Visibility=Default Availability=Available
// [8] qreal lineHeight(qreal, qreal) const

/*

*/
impl /*struct*/ QTextBlockFormat {
  pub fn lineHeight_0<RetType, T: QTextBlockFormat_lineHeight_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.lineHeight_0(self);
    // return 1;
  }
}
pub trait QTextBlockFormat_lineHeight_0<RetType> {
  fn lineHeight_0(self , rsthis: & QTextBlockFormat) -> RetType;
}
impl<'a> /*trait*/ QTextBlockFormat_lineHeight_0<f64> for (f64,f64) {
  fn lineHeight_0(self , rsthis: & QTextBlockFormat) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QTextBlockFormat10lineHeightEdd", 2,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:630
// index:1
// Public inline Visibility=Default Availability=Available
// [8] qreal lineHeight() const

/*

*/
impl /*struct*/ QTextBlockFormat {
  pub fn lineHeight_1<RetType, T: QTextBlockFormat_lineHeight_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.lineHeight_1(self);
    // return 1;
  }
}
pub trait QTextBlockFormat_lineHeight_1<RetType> {
  fn lineHeight_1(self , rsthis: & QTextBlockFormat) -> RetType;
}
impl<'a> /*trait*/ QTextBlockFormat_lineHeight_1<f64> for () {
  fn lineHeight_1(self , rsthis: & QTextBlockFormat) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QTextBlockFormat10lineHeightEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:632
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int lineHeightType() const

/*

*/
impl /*struct*/ QTextBlockFormat {
  pub fn lineHeightType_0<RetType, T: QTextBlockFormat_lineHeightType_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.lineHeightType_0(self);
    // return 1;
  }
}
pub trait QTextBlockFormat_lineHeightType_0<RetType> {
  fn lineHeightType_0(self , rsthis: & QTextBlockFormat) -> RetType;
}
impl<'a> /*trait*/ QTextBlockFormat_lineHeightType_0<i32> for () {
  fn lineHeightType_0(self , rsthis: & QTextBlockFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QTextBlockFormat14lineHeightTypeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:635
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setNonBreakableLines(bool)

/*

*/
impl /*struct*/ QTextBlockFormat {
  pub fn setNonBreakableLines_0<RetType, T: QTextBlockFormat_setNonBreakableLines_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setNonBreakableLines_0(self);
    // return 1;
  }
}
pub trait QTextBlockFormat_setNonBreakableLines_0<RetType> {
  fn setNonBreakableLines_0(self , rsthis: & QTextBlockFormat) -> RetType;
}
impl<'a> /*trait*/ QTextBlockFormat_setNonBreakableLines_0<(/*void*/)> for (bool) {
  fn setNonBreakableLines_0(self , rsthis: & QTextBlockFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN16QTextBlockFormat20setNonBreakableLinesEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:637
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool nonBreakableLines() const

/*

*/
impl /*struct*/ QTextBlockFormat {
  pub fn nonBreakableLines_0<RetType, T: QTextBlockFormat_nonBreakableLines_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.nonBreakableLines_0(self);
    // return 1;
  }
}
pub trait QTextBlockFormat_nonBreakableLines_0<RetType> {
  fn nonBreakableLines_0(self , rsthis: & QTextBlockFormat) -> RetType;
}
impl<'a> /*trait*/ QTextBlockFormat_nonBreakableLines_0<bool> for () {
  fn nonBreakableLines_0(self , rsthis: & QTextBlockFormat) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QTextBlockFormat17nonBreakableLinesEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:640
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setPageBreakPolicy(QTextFormat::PageBreakFlags)

/*

*/
impl /*struct*/ QTextBlockFormat {
  pub fn setPageBreakPolicy_0<RetType, T: QTextBlockFormat_setPageBreakPolicy_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPageBreakPolicy_0(self);
    // return 1;
  }
}
pub trait QTextBlockFormat_setPageBreakPolicy_0<RetType> {
  fn setPageBreakPolicy_0(self , rsthis: & QTextBlockFormat) -> RetType;
}
impl<'a> /*trait*/ QTextBlockFormat_setPageBreakPolicy_0<(/*void*/)> for (i32) {
  fn setPageBreakPolicy_0(self , rsthis: & QTextBlockFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN16QTextBlockFormat18setPageBreakPolicyE6QFlagsIN11QTextFormat13PageBreakFlagEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:642
// index:0
// Public inline Visibility=Default Availability=Available
// [4] QTextFormat::PageBreakFlags pageBreakPolicy() const

/*

*/
impl /*struct*/ QTextBlockFormat {
  pub fn pageBreakPolicy_0<RetType, T: QTextBlockFormat_pageBreakPolicy_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.pageBreakPolicy_0(self);
    // return 1;
  }
}
pub trait QTextBlockFormat_pageBreakPolicy_0<RetType> {
  fn pageBreakPolicy_0(self , rsthis: & QTextBlockFormat) -> RetType;
}
impl<'a> /*trait*/ QTextBlockFormat_pageBreakPolicy_0<i32> for () {
  fn pageBreakPolicy_0(self , rsthis: & QTextBlockFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QTextBlockFormat15pageBreakPolicyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}


pub fn DeleteQTextBlockFormat(this :*mut QTextBlockFormat) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN16QTextBlockFormatD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}

/*


*/
pub type QTextBlockFormat__LineHeightTypes = i32;
// 
pub const QTextBlockFormat__SingleHeight :QTextBlockFormat__LineHeightTypes = 0;
// 
pub const QTextBlockFormat__ProportionalHeight :QTextBlockFormat__LineHeightTypes = 1;
// 
pub const QTextBlockFormat__FixedHeight :QTextBlockFormat__LineHeightTypes = 2;
// 
pub const QTextBlockFormat__MinimumHeight :QTextBlockFormat__LineHeightTypes = 3;
// 
pub const QTextBlockFormat__LineDistanceHeight :QTextBlockFormat__LineHeightTypes = 4;
pub fn QTextBlockFormat_LineHeightTypesItemName(val: i32) ->String {
  match val {
     QTextBlockFormat__SingleHeight => // 0
     {return String::from("SingleHeight");}
     QTextBlockFormat__ProportionalHeight => // 1
     {return String::from("ProportionalHeight");}
     QTextBlockFormat__FixedHeight => // 2
     {return String::from("FixedHeight");}
     QTextBlockFormat__MinimumHeight => // 3
     {return String::from("MinimumHeight");}
     QTextBlockFormat__LineDistanceHeight => // 4
     {return String::from("LineDistanceHeight");}
  _ => {return format!("{}", val);}
}
}
pub fn QTextBlockFormat_LineHeightTypesItemName_s(val: i32) ->String {
  //var nilthis *QTextBlockFormat
  //return nilthis.LineHeightTypesItemName(val);
  return QTextBlockFormat_LineHeightTypesItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
