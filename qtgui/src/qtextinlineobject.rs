

// mod ::gui::QTextInlineObject
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
// extern C begin: 80
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
#[derive(Default)] // class sizeof(QTextInlineObject)=16
pub struct QTextInlineObject {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QTextInlineObject_ITF interface {
//    QTextInlineObject_PTR() *QTextInlineObject
//}
//func (ptr *QTextInlineObject) QTextInlineObject_PTR() *QTextInlineObject { return ptr }

impl /*struct*/ QTextInlineObject {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QTextInlineObject {
    return QTextInlineObject{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QTextInlineObject {
//  type Target = QTextInlineObjectBASE;
//
//  fn deref(&self) -> &QTextInlineObjectBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QTextInlineObjectBASE> for QTextInlineObject {
//  fn as_ref(& self) -> & QTextInlineObjectBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qtextlayout.h:72
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void QTextInlineObject()

/*

*/
// QTextInlineObject() ctx.fn_proto_cpp
impl /*struct*/ QTextInlineObject {
  pub fn QTextInlineObject_0<T: QTextInlineObject_QTextInlineObject_0>(value: T) -> QTextInlineObject {
    let rsthis = value.QTextInlineObject_0();
    return rsthis;
    // return 1;
  }
}

pub trait QTextInlineObject_QTextInlineObject_0 {
  fn QTextInlineObject_0(self) -> QTextInlineObject;
}
// QTextInlineObject() ctx.fn_proto_cpp
impl<'a> /*trait*/ QTextInlineObject_QTextInlineObject_0 for () {
  fn QTextInlineObject_0(self) -> QTextInlineObject {
    // unsafe{_ZN17QTextInlineObjectC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN17QTextInlineObjectC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTextInlineObject{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextlayout.h:73
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isValid() const

/*

*/
impl /*struct*/ QTextInlineObject {
  pub fn isValid_0<RetType, T: QTextInlineObject_isValid_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isValid_0(self);
    // return 1;
  }
}
pub trait QTextInlineObject_isValid_0<RetType> {
  fn isValid_0(self , rsthis: & QTextInlineObject) -> RetType;
}
impl<'a> /*trait*/ QTextInlineObject_isValid_0<bool> for () {
  fn isValid_0(self , rsthis: & QTextInlineObject) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QTextInlineObject7isValidEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextlayout.h:75
// index:0
// Public Visibility=Default Availability=Available
// [32] QRectF rect() const

/*

*/
impl /*struct*/ QTextInlineObject {
  pub fn rect_0<RetType, T: QTextInlineObject_rect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rect_0(self);
    // return 1;
  }
}
pub trait QTextInlineObject_rect_0<RetType> {
  fn rect_0(self , rsthis: & QTextInlineObject) -> RetType;
}
impl<'a> /*trait*/ QTextInlineObject_rect_0<usize> for () {
  fn rect_0(self , rsthis: & QTextInlineObject) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QTextInlineObject4rectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextlayout.h:76
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal width() const

/*

*/
impl /*struct*/ QTextInlineObject {
  pub fn width_0<RetType, T: QTextInlineObject_width_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.width_0(self);
    // return 1;
  }
}
pub trait QTextInlineObject_width_0<RetType> {
  fn width_0(self , rsthis: & QTextInlineObject) -> RetType;
}
impl<'a> /*trait*/ QTextInlineObject_width_0<f64> for () {
  fn width_0(self , rsthis: & QTextInlineObject) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QTextInlineObject5widthEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextlayout.h:77
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal ascent() const

/*

*/
impl /*struct*/ QTextInlineObject {
  pub fn ascent_0<RetType, T: QTextInlineObject_ascent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.ascent_0(self);
    // return 1;
  }
}
pub trait QTextInlineObject_ascent_0<RetType> {
  fn ascent_0(self , rsthis: & QTextInlineObject) -> RetType;
}
impl<'a> /*trait*/ QTextInlineObject_ascent_0<f64> for () {
  fn ascent_0(self , rsthis: & QTextInlineObject) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QTextInlineObject6ascentEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextlayout.h:78
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal descent() const

/*

*/
impl /*struct*/ QTextInlineObject {
  pub fn descent_0<RetType, T: QTextInlineObject_descent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.descent_0(self);
    // return 1;
  }
}
pub trait QTextInlineObject_descent_0<RetType> {
  fn descent_0(self , rsthis: & QTextInlineObject) -> RetType;
}
impl<'a> /*trait*/ QTextInlineObject_descent_0<f64> for () {
  fn descent_0(self , rsthis: & QTextInlineObject) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QTextInlineObject7descentEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextlayout.h:79
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal height() const

/*

*/
impl /*struct*/ QTextInlineObject {
  pub fn height_0<RetType, T: QTextInlineObject_height_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.height_0(self);
    // return 1;
  }
}
pub trait QTextInlineObject_height_0<RetType> {
  fn height_0(self , rsthis: & QTextInlineObject) -> RetType;
}
impl<'a> /*trait*/ QTextInlineObject_height_0<f64> for () {
  fn height_0(self , rsthis: & QTextInlineObject) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QTextInlineObject6heightEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextlayout.h:81
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::LayoutDirection textDirection() const

/*

*/
impl /*struct*/ QTextInlineObject {
  pub fn textDirection_0<RetType, T: QTextInlineObject_textDirection_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.textDirection_0(self);
    // return 1;
  }
}
pub trait QTextInlineObject_textDirection_0<RetType> {
  fn textDirection_0(self , rsthis: & QTextInlineObject) -> RetType;
}
impl<'a> /*trait*/ QTextInlineObject_textDirection_0<i32> for () {
  fn textDirection_0(self , rsthis: & QTextInlineObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QTextInlineObject13textDirectionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextlayout.h:83
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setWidth(qreal)

/*

*/
impl /*struct*/ QTextInlineObject {
  pub fn setWidth_0<RetType, T: QTextInlineObject_setWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setWidth_0(self);
    // return 1;
  }
}
pub trait QTextInlineObject_setWidth_0<RetType> {
  fn setWidth_0(self , rsthis: & QTextInlineObject) -> RetType;
}
impl<'a> /*trait*/ QTextInlineObject_setWidth_0<(/*void*/)> for (f64) {
  fn setWidth_0(self , rsthis: & QTextInlineObject) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN17QTextInlineObject8setWidthEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextlayout.h:84
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setAscent(qreal)

/*

*/
impl /*struct*/ QTextInlineObject {
  pub fn setAscent_0<RetType, T: QTextInlineObject_setAscent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setAscent_0(self);
    // return 1;
  }
}
pub trait QTextInlineObject_setAscent_0<RetType> {
  fn setAscent_0(self , rsthis: & QTextInlineObject) -> RetType;
}
impl<'a> /*trait*/ QTextInlineObject_setAscent_0<(/*void*/)> for (f64) {
  fn setAscent_0(self , rsthis: & QTextInlineObject) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN17QTextInlineObject9setAscentEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextlayout.h:85
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDescent(qreal)

/*

*/
impl /*struct*/ QTextInlineObject {
  pub fn setDescent_0<RetType, T: QTextInlineObject_setDescent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDescent_0(self);
    // return 1;
  }
}
pub trait QTextInlineObject_setDescent_0<RetType> {
  fn setDescent_0(self , rsthis: & QTextInlineObject) -> RetType;
}
impl<'a> /*trait*/ QTextInlineObject_setDescent_0<(/*void*/)> for (f64) {
  fn setDescent_0(self , rsthis: & QTextInlineObject) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN17QTextInlineObject10setDescentEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextlayout.h:87
// index:0
// Public Visibility=Default Availability=Available
// [4] int textPosition() const

/*

*/
impl /*struct*/ QTextInlineObject {
  pub fn textPosition_0<RetType, T: QTextInlineObject_textPosition_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.textPosition_0(self);
    // return 1;
  }
}
pub trait QTextInlineObject_textPosition_0<RetType> {
  fn textPosition_0(self , rsthis: & QTextInlineObject) -> RetType;
}
impl<'a> /*trait*/ QTextInlineObject_textPosition_0<i32> for () {
  fn textPosition_0(self , rsthis: & QTextInlineObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QTextInlineObject12textPositionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextlayout.h:89
// index:0
// Public Visibility=Default Availability=Available
// [4] int formatIndex() const

/*

*/
impl /*struct*/ QTextInlineObject {
  pub fn formatIndex_0<RetType, T: QTextInlineObject_formatIndex_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.formatIndex_0(self);
    // return 1;
  }
}
pub trait QTextInlineObject_formatIndex_0<RetType> {
  fn formatIndex_0(self , rsthis: & QTextInlineObject) -> RetType;
}
impl<'a> /*trait*/ QTextInlineObject_formatIndex_0<i32> for () {
  fn formatIndex_0(self , rsthis: & QTextInlineObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QTextInlineObject11formatIndexEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextlayout.h:90
// index:0
// Public Visibility=Default Availability=Available
// [16] QTextFormat format() const

/*

*/
impl /*struct*/ QTextInlineObject {
  pub fn format_0<RetType, T: QTextInlineObject_format_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.format_0(self);
    // return 1;
  }
}
pub trait QTextInlineObject_format_0<RetType> {
  fn format_0(self , rsthis: & QTextInlineObject) -> RetType;
}
impl<'a> /*trait*/ QTextInlineObject_format_0<usize> for () {
  fn format_0(self , rsthis: & QTextInlineObject) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QTextInlineObject6formatEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}


pub fn DeleteQTextInlineObject(this :*mut QTextInlineObject) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN17QTextInlineObjectD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
