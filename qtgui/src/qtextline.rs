

// mod ::gui::QTextLine
// package qtgui
// /usr/include/qt/QtGui/qtextlayout.h
// #include <qtextlayout.h>
// #include <QtGui>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 41
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
#[derive(Default)] // class sizeof(QTextLine)=16
pub struct QTextLine {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QTextLine_ITF interface {
//    QTextLine_PTR() *QTextLine
//}
//func (ptr *QTextLine) QTextLine_PTR() *QTextLine { return ptr }

impl /*struct*/ QTextLine {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QTextLine {
    return QTextLine{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QTextLine {
//  type Target = QTextLineBASE;
//
//  fn deref(&self) -> &QTextLineBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QTextLineBASE> for QTextLine {
//  fn as_ref(& self) -> & QTextLineBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qtextlayout.h:213
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void QTextLine()

/*

*/
// QTextLine() ctx.fn_proto_cpp
impl /*struct*/ QTextLine {
  pub fn QTextLine_0<T: QTextLine_QTextLine_0>(value: T) -> QTextLine {
    let rsthis = value.QTextLine_0();
    return rsthis;
    // return 1;
  }
}

pub trait QTextLine_QTextLine_0 {
  fn QTextLine_0(self) -> QTextLine;
}
// QTextLine() ctx.fn_proto_cpp
impl<'a> /*trait*/ QTextLine_QTextLine_0 for () {
  fn QTextLine_0(self) -> QTextLine {
    // unsafe{_ZN9QTextLineC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QTextLineC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTextLine{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextlayout.h:214
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isValid() const

/*

*/
impl /*struct*/ QTextLine {
  pub fn isValid_0<RetType, T: QTextLine_isValid_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isValid_0(self);
    // return 1;
  }
}
pub trait QTextLine_isValid_0<RetType> {
  fn isValid_0(self , rsthis: & QTextLine) -> RetType;
}
impl<'a> /*trait*/ QTextLine_isValid_0<bool> for () {
  fn isValid_0(self , rsthis: & QTextLine) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTextLine7isValidEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextlayout.h:216
// index:0
// Public Visibility=Default Availability=Available
// [32] QRectF rect() const

/*

*/
impl /*struct*/ QTextLine {
  pub fn rect_0<RetType, T: QTextLine_rect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rect_0(self);
    // return 1;
  }
}
pub trait QTextLine_rect_0<RetType> {
  fn rect_0(self , rsthis: & QTextLine) -> RetType;
}
impl<'a> /*trait*/ QTextLine_rect_0<usize> for () {
  fn rect_0(self , rsthis: & QTextLine) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTextLine4rectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextlayout.h:217
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal x() const

/*

*/
impl /*struct*/ QTextLine {
  pub fn x_0<RetType, T: QTextLine_x_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.x_0(self);
    // return 1;
  }
}
pub trait QTextLine_x_0<RetType> {
  fn x_0(self , rsthis: & QTextLine) -> RetType;
}
impl<'a> /*trait*/ QTextLine_x_0<f64> for () {
  fn x_0(self , rsthis: & QTextLine) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTextLine1xEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextlayout.h:218
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal y() const

/*

*/
impl /*struct*/ QTextLine {
  pub fn y_0<RetType, T: QTextLine_y_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.y_0(self);
    // return 1;
  }
}
pub trait QTextLine_y_0<RetType> {
  fn y_0(self , rsthis: & QTextLine) -> RetType;
}
impl<'a> /*trait*/ QTextLine_y_0<f64> for () {
  fn y_0(self , rsthis: & QTextLine) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTextLine1yEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextlayout.h:219
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal width() const

/*

*/
impl /*struct*/ QTextLine {
  pub fn width_0<RetType, T: QTextLine_width_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.width_0(self);
    // return 1;
  }
}
pub trait QTextLine_width_0<RetType> {
  fn width_0(self , rsthis: & QTextLine) -> RetType;
}
impl<'a> /*trait*/ QTextLine_width_0<f64> for () {
  fn width_0(self , rsthis: & QTextLine) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTextLine5widthEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextlayout.h:220
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal ascent() const

/*

*/
impl /*struct*/ QTextLine {
  pub fn ascent_0<RetType, T: QTextLine_ascent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.ascent_0(self);
    // return 1;
  }
}
pub trait QTextLine_ascent_0<RetType> {
  fn ascent_0(self , rsthis: & QTextLine) -> RetType;
}
impl<'a> /*trait*/ QTextLine_ascent_0<f64> for () {
  fn ascent_0(self , rsthis: & QTextLine) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTextLine6ascentEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextlayout.h:221
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal descent() const

/*

*/
impl /*struct*/ QTextLine {
  pub fn descent_0<RetType, T: QTextLine_descent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.descent_0(self);
    // return 1;
  }
}
pub trait QTextLine_descent_0<RetType> {
  fn descent_0(self , rsthis: & QTextLine) -> RetType;
}
impl<'a> /*trait*/ QTextLine_descent_0<f64> for () {
  fn descent_0(self , rsthis: & QTextLine) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTextLine7descentEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextlayout.h:222
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal height() const

/*

*/
impl /*struct*/ QTextLine {
  pub fn height_0<RetType, T: QTextLine_height_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.height_0(self);
    // return 1;
  }
}
pub trait QTextLine_height_0<RetType> {
  fn height_0(self , rsthis: & QTextLine) -> RetType;
}
impl<'a> /*trait*/ QTextLine_height_0<f64> for () {
  fn height_0(self , rsthis: & QTextLine) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTextLine6heightEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextlayout.h:223
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal leading() const

/*

*/
impl /*struct*/ QTextLine {
  pub fn leading_0<RetType, T: QTextLine_leading_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.leading_0(self);
    // return 1;
  }
}
pub trait QTextLine_leading_0<RetType> {
  fn leading_0(self , rsthis: & QTextLine) -> RetType;
}
impl<'a> /*trait*/ QTextLine_leading_0<f64> for () {
  fn leading_0(self , rsthis: & QTextLine) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTextLine7leadingEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextlayout.h:225
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setLeadingIncluded(bool)

/*

*/
impl /*struct*/ QTextLine {
  pub fn setLeadingIncluded_0<RetType, T: QTextLine_setLeadingIncluded_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setLeadingIncluded_0(self);
    // return 1;
  }
}
pub trait QTextLine_setLeadingIncluded_0<RetType> {
  fn setLeadingIncluded_0(self , rsthis: & QTextLine) -> RetType;
}
impl<'a> /*trait*/ QTextLine_setLeadingIncluded_0<(/*void*/)> for (bool) {
  fn setLeadingIncluded_0(self , rsthis: & QTextLine) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN9QTextLine18setLeadingIncludedEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextlayout.h:226
// index:0
// Public Visibility=Default Availability=Available
// [1] bool leadingIncluded() const

/*

*/
impl /*struct*/ QTextLine {
  pub fn leadingIncluded_0<RetType, T: QTextLine_leadingIncluded_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.leadingIncluded_0(self);
    // return 1;
  }
}
pub trait QTextLine_leadingIncluded_0<RetType> {
  fn leadingIncluded_0(self , rsthis: & QTextLine) -> RetType;
}
impl<'a> /*trait*/ QTextLine_leadingIncluded_0<bool> for () {
  fn leadingIncluded_0(self , rsthis: & QTextLine) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTextLine15leadingIncludedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextlayout.h:228
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal naturalTextWidth() const

/*

*/
impl /*struct*/ QTextLine {
  pub fn naturalTextWidth_0<RetType, T: QTextLine_naturalTextWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.naturalTextWidth_0(self);
    // return 1;
  }
}
pub trait QTextLine_naturalTextWidth_0<RetType> {
  fn naturalTextWidth_0(self , rsthis: & QTextLine) -> RetType;
}
impl<'a> /*trait*/ QTextLine_naturalTextWidth_0<f64> for () {
  fn naturalTextWidth_0(self , rsthis: & QTextLine) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTextLine16naturalTextWidthEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextlayout.h:229
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal horizontalAdvance() const

/*

*/
impl /*struct*/ QTextLine {
  pub fn horizontalAdvance_0<RetType, T: QTextLine_horizontalAdvance_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.horizontalAdvance_0(self);
    // return 1;
  }
}
pub trait QTextLine_horizontalAdvance_0<RetType> {
  fn horizontalAdvance_0(self , rsthis: & QTextLine) -> RetType;
}
impl<'a> /*trait*/ QTextLine_horizontalAdvance_0<f64> for () {
  fn horizontalAdvance_0(self , rsthis: & QTextLine) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTextLine17horizontalAdvanceEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextlayout.h:230
// index:0
// Public Visibility=Default Availability=Available
// [32] QRectF naturalTextRect() const

/*

*/
impl /*struct*/ QTextLine {
  pub fn naturalTextRect_0<RetType, T: QTextLine_naturalTextRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.naturalTextRect_0(self);
    // return 1;
  }
}
pub trait QTextLine_naturalTextRect_0<RetType> {
  fn naturalTextRect_0(self , rsthis: & QTextLine) -> RetType;
}
impl<'a> /*trait*/ QTextLine_naturalTextRect_0<usize> for () {
  fn naturalTextRect_0(self , rsthis: & QTextLine) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTextLine15naturalTextRectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextlayout.h:242
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal cursorToX(int *, QTextLine::Edge) const

/*

*/
impl /*struct*/ QTextLine {
  pub fn cursorToX_0<RetType, T: QTextLine_cursorToX_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cursorToX_0(self);
    // return 1;
  }
}
pub trait QTextLine_cursorToX_0<RetType> {
  fn cursorToX_0(self , rsthis: & QTextLine) -> RetType;
}
impl<'a> /*trait*/ QTextLine_cursorToX_0<f64> for (usize,i32) {
  fn cursorToX_0(self , rsthis: & QTextLine) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTextLine9cursorToXEPiNS_4EdgeE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextlayout.h:243
// index:1
// Public inline Visibility=Default Availability=Available
// [8] qreal cursorToX(int, QTextLine::Edge) const

/*

*/
impl /*struct*/ QTextLine {
  pub fn cursorToX_1<RetType, T: QTextLine_cursorToX_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cursorToX_1(self);
    // return 1;
  }
}
pub trait QTextLine_cursorToX_1<RetType> {
  fn cursorToX_1(self , rsthis: & QTextLine) -> RetType;
}
impl<'a> /*trait*/ QTextLine_cursorToX_1<f64> for (i32,i32) {
  fn cursorToX_1(self , rsthis: & QTextLine) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTextLine9cursorToXEiNS_4EdgeE", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextlayout.h:244
// index:0
// Public Visibility=Default Availability=Available
// [4] int xToCursor(qreal, QTextLine::CursorPosition) const

/*

*/
impl /*struct*/ QTextLine {
  pub fn xToCursor_0<RetType, T: QTextLine_xToCursor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.xToCursor_0(self);
    // return 1;
  }
}
pub trait QTextLine_xToCursor_0<RetType> {
  fn xToCursor_0(self , rsthis: & QTextLine) -> RetType;
}
impl<'a> /*trait*/ QTextLine_xToCursor_0<i32> for (f64,i32) {
  fn xToCursor_0(self , rsthis: & QTextLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTextLine9xToCursorEdNS_14CursorPositionE", 2,qtrt::FFITY_DOUBLE,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextlayout.h:246
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setLineWidth(qreal)

/*

*/
impl /*struct*/ QTextLine {
  pub fn setLineWidth_0<RetType, T: QTextLine_setLineWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setLineWidth_0(self);
    // return 1;
  }
}
pub trait QTextLine_setLineWidth_0<RetType> {
  fn setLineWidth_0(self , rsthis: & QTextLine) -> RetType;
}
impl<'a> /*trait*/ QTextLine_setLineWidth_0<(/*void*/)> for (f64) {
  fn setLineWidth_0(self , rsthis: & QTextLine) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN9QTextLine12setLineWidthEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextlayout.h:247
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setNumColumns(int)

/*

*/
impl /*struct*/ QTextLine {
  pub fn setNumColumns_0<RetType, T: QTextLine_setNumColumns_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setNumColumns_0(self);
    // return 1;
  }
}
pub trait QTextLine_setNumColumns_0<RetType> {
  fn setNumColumns_0(self , rsthis: & QTextLine) -> RetType;
}
impl<'a> /*trait*/ QTextLine_setNumColumns_0<(/*void*/)> for (i32) {
  fn setNumColumns_0(self , rsthis: & QTextLine) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QTextLine13setNumColumnsEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextlayout.h:248
// index:1
// Public Visibility=Default Availability=Available
// [-2] void setNumColumns(int, qreal)

/*

*/
impl /*struct*/ QTextLine {
  pub fn setNumColumns_1<RetType, T: QTextLine_setNumColumns_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setNumColumns_1(self);
    // return 1;
  }
}
pub trait QTextLine_setNumColumns_1<RetType> {
  fn setNumColumns_1(self , rsthis: & QTextLine) -> RetType;
}
impl<'a> /*trait*/ QTextLine_setNumColumns_1<(/*void*/)> for (i32,f64) {
  fn setNumColumns_1(self , rsthis: & QTextLine) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN9QTextLine13setNumColumnsEid", 2,qtrt::FFITY_INT,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextlayout.h:250
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setPosition(const QPointF &)

/*
Moves the text layout to point p.

See also position().
*/
impl /*struct*/ QTextLine {
  pub fn setPosition_0<RetType, T: QTextLine_setPosition_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPosition_0(self);
    // return 1;
  }
}
pub trait QTextLine_setPosition_0<RetType> {
  fn setPosition_0(self , rsthis: & QTextLine) -> RetType;
}
impl<'a> /*trait*/ QTextLine_setPosition_0<(/*void*/)> for (usize) {
  fn setPosition_0(self , rsthis: & QTextLine) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QTextLine11setPositionERK7QPointF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextlayout.h:251
// index:0
// Public Visibility=Default Availability=Available
// [16] QPointF position() const

/*
The global position of the layout. This is independent of the bounding rectangle and of the layout process.

This function was introduced in  Qt 4.2.

See also setPosition().
*/
impl /*struct*/ QTextLine {
  pub fn position_0<RetType, T: QTextLine_position_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.position_0(self);
    // return 1;
  }
}
pub trait QTextLine_position_0<RetType> {
  fn position_0(self , rsthis: & QTextLine) -> RetType;
}
impl<'a> /*trait*/ QTextLine_position_0<usize> for () {
  fn position_0(self , rsthis: & QTextLine) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTextLine8positionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextlayout.h:253
// index:0
// Public Visibility=Default Availability=Available
// [4] int textStart() const

/*

*/
impl /*struct*/ QTextLine {
  pub fn textStart_0<RetType, T: QTextLine_textStart_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.textStart_0(self);
    // return 1;
  }
}
pub trait QTextLine_textStart_0<RetType> {
  fn textStart_0(self , rsthis: & QTextLine) -> RetType;
}
impl<'a> /*trait*/ QTextLine_textStart_0<i32> for () {
  fn textStart_0(self , rsthis: & QTextLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTextLine9textStartEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextlayout.h:254
// index:0
// Public Visibility=Default Availability=Available
// [4] int textLength() const

/*

*/
impl /*struct*/ QTextLine {
  pub fn textLength_0<RetType, T: QTextLine_textLength_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.textLength_0(self);
    // return 1;
  }
}
pub trait QTextLine_textLength_0<RetType> {
  fn textLength_0(self , rsthis: & QTextLine) -> RetType;
}
impl<'a> /*trait*/ QTextLine_textLength_0<i32> for () {
  fn textLength_0(self , rsthis: & QTextLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTextLine10textLengthEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextlayout.h:256
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int lineNumber() const

/*

*/
impl /*struct*/ QTextLine {
  pub fn lineNumber_0<RetType, T: QTextLine_lineNumber_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.lineNumber_0(self);
    // return 1;
  }
}
pub trait QTextLine_lineNumber_0<RetType> {
  fn lineNumber_0(self , rsthis: & QTextLine) -> RetType;
}
impl<'a> /*trait*/ QTextLine_lineNumber_0<i32> for () {
  fn lineNumber_0(self , rsthis: & QTextLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTextLine10lineNumberEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}


pub fn DeleteQTextLine(this :*mut QTextLine) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN9QTextLineD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}

/*


*/
pub type QTextLine__Edge = i32;
// 
pub const QTextLine__Leading :QTextLine__Edge = 0;
// 
pub const QTextLine__Trailing :QTextLine__Edge = 1;
pub fn QTextLine_EdgeItemName(val: i32) ->String {
  match val {
     QTextLine__Leading => // 0
     {return String::from("Leading");}
     QTextLine__Trailing => // 1
     {return String::from("Trailing");}
  _ => {return format!("{}", val);}
}
}
pub fn QTextLine_EdgeItemName_s(val: i32) ->String {
  //var nilthis *QTextLine
  //return nilthis.EdgeItemName(val);
  return QTextLine_EdgeItemName(val);
}


/*


*/
pub type QTextLine__CursorPosition = i32;
// 
pub const QTextLine__CursorBetweenCharacters :QTextLine__CursorPosition = 0;
// 
pub const QTextLine__CursorOnCharacter :QTextLine__CursorPosition = 1;
pub fn QTextLine_CursorPositionItemName(val: i32) ->String {
  match val {
     QTextLine__CursorBetweenCharacters => // 0
     {return String::from("CursorBetweenCharacters");}
     QTextLine__CursorOnCharacter => // 1
     {return String::from("CursorOnCharacter");}
  _ => {return format!("{}", val);}
}
}
pub fn QTextLine_CursorPositionItemName_s(val: i32) ->String {
  //var nilthis *QTextLine
  //return nilthis.CursorPositionItemName(val);
  return QTextLine_CursorPositionItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
