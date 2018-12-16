

// mod ::gui::QTextFrameFormat
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
// extern C begin: 9
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
#[derive(Default)] // class sizeof(QTextFrameFormat)=16
pub struct QTextFrameFormat {
  qbase: QTextFormat,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QTextFrameFormat_ITF interface {
//    QTextFormat_ITF
//    QTextFrameFormat_PTR() *QTextFrameFormat
//}
//func (ptr *QTextFrameFormat) QTextFrameFormat_PTR() *QTextFrameFormat { return ptr }

impl /*struct*/ QTextFrameFormat {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QTextFrameFormat {
    return QTextFrameFormat{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QTextFrameFormat {
//  type Target = QTextFrameFormatBASE;
//
//  fn deref(&self) -> &QTextFrameFormatBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QTextFrameFormatBASE> for QTextFrameFormat {
//  fn as_ref(& self) -> & QTextFrameFormatBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qtextformat.h:770
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QTextFrameFormat()

/*

*/
// QTextFrameFormat() ctx.fn_proto_cpp
impl /*struct*/ QTextFrameFormat {
  pub fn QTextFrameFormat_0<T: QTextFrameFormat_QTextFrameFormat_0>(value: T) -> QTextFrameFormat {
    let rsthis = value.QTextFrameFormat_0();
    return rsthis;
    // return 1;
  }
}

pub trait QTextFrameFormat_QTextFrameFormat_0 {
  fn QTextFrameFormat_0(self) -> QTextFrameFormat;
}
// QTextFrameFormat() ctx.fn_proto_cpp
impl<'a> /*trait*/ QTextFrameFormat_QTextFrameFormat_0 for () {
  fn QTextFrameFormat_0(self) -> QTextFrameFormat {
    // unsafe{_ZN16QTextFrameFormatC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN16QTextFrameFormatC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTextFrameFormat{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:852
// index:1
// Protected Visibility=Default Availability=Available
// [-2] void QTextFrameFormat(const QTextFormat &)

/*

*/
// QTextFrameFormat(const QTextFormat &) ctx.fn_proto_cpp
impl /*struct*/ QTextFrameFormat {
  pub fn QTextFrameFormat_1<T: QTextFrameFormat_QTextFrameFormat_1>(value: T) -> QTextFrameFormat {
    let rsthis = value.QTextFrameFormat_1();
    return rsthis;
    // return 1;
  }
}

pub trait QTextFrameFormat_QTextFrameFormat_1 {
  fn QTextFrameFormat_1(self) -> QTextFrameFormat;
}
// QTextFrameFormat(const QTextFormat &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QTextFrameFormat_QTextFrameFormat_1 for (usize) {
  fn QTextFrameFormat_1(self) -> QTextFrameFormat {
    // unsafe{_ZN16QTextFrameFormatC2ERK11QTextFormat()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN16QTextFrameFormatC2ERK11QTextFormat", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTextFrameFormat{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:772
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isValid() const

/*
Returns true if the format is valid (i.e. is not InvalidFormat); otherwise returns false.
*/
impl /*struct*/ QTextFrameFormat {
  pub fn isValid_0<RetType, T: QTextFrameFormat_isValid_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isValid_0(self);
    // return 1;
  }
}
pub trait QTextFrameFormat_isValid_0<RetType> {
  fn isValid_0(self , rsthis: & QTextFrameFormat) -> RetType;
}
impl<'a> /*trait*/ QTextFrameFormat_isValid_0<bool> for () {
  fn isValid_0(self , rsthis: & QTextFrameFormat) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QTextFrameFormat7isValidEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:796
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setPosition(QTextFrameFormat::Position)

/*

*/
impl /*struct*/ QTextFrameFormat {
  pub fn setPosition_0<RetType, T: QTextFrameFormat_setPosition_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPosition_0(self);
    // return 1;
  }
}
pub trait QTextFrameFormat_setPosition_0<RetType> {
  fn setPosition_0(self , rsthis: & QTextFrameFormat) -> RetType;
}
impl<'a> /*trait*/ QTextFrameFormat_setPosition_0<(/*void*/)> for (i32) {
  fn setPosition_0(self , rsthis: & QTextFrameFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN16QTextFrameFormat11setPositionENS_8PositionE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:798
// index:0
// Public inline Visibility=Default Availability=Available
// [4] QTextFrameFormat::Position position() const

/*

*/
impl /*struct*/ QTextFrameFormat {
  pub fn position_0<RetType, T: QTextFrameFormat_position_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.position_0(self);
    // return 1;
  }
}
pub trait QTextFrameFormat_position_0<RetType> {
  fn position_0(self , rsthis: & QTextFrameFormat) -> RetType;
}
impl<'a> /*trait*/ QTextFrameFormat_position_0<i32> for () {
  fn position_0(self , rsthis: & QTextFrameFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QTextFrameFormat8positionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:801
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setBorder(qreal)

/*

*/
impl /*struct*/ QTextFrameFormat {
  pub fn setBorder_0<RetType, T: QTextFrameFormat_setBorder_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setBorder_0(self);
    // return 1;
  }
}
pub trait QTextFrameFormat_setBorder_0<RetType> {
  fn setBorder_0(self , rsthis: & QTextFrameFormat) -> RetType;
}
impl<'a> /*trait*/ QTextFrameFormat_setBorder_0<(/*void*/)> for (f64) {
  fn setBorder_0(self , rsthis: & QTextFrameFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN16QTextFrameFormat9setBorderEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:802
// index:0
// Public inline Visibility=Default Availability=Available
// [8] qreal border() const

/*

*/
impl /*struct*/ QTextFrameFormat {
  pub fn border_0<RetType, T: QTextFrameFormat_border_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.border_0(self);
    // return 1;
  }
}
pub trait QTextFrameFormat_border_0<RetType> {
  fn border_0(self , rsthis: & QTextFrameFormat) -> RetType;
}
impl<'a> /*trait*/ QTextFrameFormat_border_0<f64> for () {
  fn border_0(self , rsthis: & QTextFrameFormat) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QTextFrameFormat6borderEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:805
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setBorderBrush(const QBrush &)

/*

*/
impl /*struct*/ QTextFrameFormat {
  pub fn setBorderBrush_0<RetType, T: QTextFrameFormat_setBorderBrush_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setBorderBrush_0(self);
    // return 1;
  }
}
pub trait QTextFrameFormat_setBorderBrush_0<RetType> {
  fn setBorderBrush_0(self , rsthis: & QTextFrameFormat) -> RetType;
}
impl<'a> /*trait*/ QTextFrameFormat_setBorderBrush_0<(/*void*/)> for (usize) {
  fn setBorderBrush_0(self , rsthis: & QTextFrameFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN16QTextFrameFormat14setBorderBrushERK6QBrush", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:807
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QBrush borderBrush() const

/*

*/
impl /*struct*/ QTextFrameFormat {
  pub fn borderBrush_0<RetType, T: QTextFrameFormat_borderBrush_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.borderBrush_0(self);
    // return 1;
  }
}
pub trait QTextFrameFormat_borderBrush_0<RetType> {
  fn borderBrush_0(self , rsthis: & QTextFrameFormat) -> RetType;
}
impl<'a> /*trait*/ QTextFrameFormat_borderBrush_0<usize> for () {
  fn borderBrush_0(self , rsthis: & QTextFrameFormat) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QTextFrameFormat11borderBrushEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:810
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setBorderStyle(QTextFrameFormat::BorderStyle)

/*

*/
impl /*struct*/ QTextFrameFormat {
  pub fn setBorderStyle_0<RetType, T: QTextFrameFormat_setBorderStyle_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setBorderStyle_0(self);
    // return 1;
  }
}
pub trait QTextFrameFormat_setBorderStyle_0<RetType> {
  fn setBorderStyle_0(self , rsthis: & QTextFrameFormat) -> RetType;
}
impl<'a> /*trait*/ QTextFrameFormat_setBorderStyle_0<(/*void*/)> for (i32) {
  fn setBorderStyle_0(self , rsthis: & QTextFrameFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN16QTextFrameFormat14setBorderStyleENS_11BorderStyleE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:812
// index:0
// Public inline Visibility=Default Availability=Available
// [4] QTextFrameFormat::BorderStyle borderStyle() const

/*

*/
impl /*struct*/ QTextFrameFormat {
  pub fn borderStyle_0<RetType, T: QTextFrameFormat_borderStyle_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.borderStyle_0(self);
    // return 1;
  }
}
pub trait QTextFrameFormat_borderStyle_0<RetType> {
  fn borderStyle_0(self , rsthis: & QTextFrameFormat) -> RetType;
}
impl<'a> /*trait*/ QTextFrameFormat_borderStyle_0<i32> for () {
  fn borderStyle_0(self , rsthis: & QTextFrameFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QTextFrameFormat11borderStyleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:815
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setMargin(qreal)

/*

*/
impl /*struct*/ QTextFrameFormat {
  pub fn setMargin_0<RetType, T: QTextFrameFormat_setMargin_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMargin_0(self);
    // return 1;
  }
}
pub trait QTextFrameFormat_setMargin_0<RetType> {
  fn setMargin_0(self , rsthis: & QTextFrameFormat) -> RetType;
}
impl<'a> /*trait*/ QTextFrameFormat_setMargin_0<(/*void*/)> for (f64) {
  fn setMargin_0(self , rsthis: & QTextFrameFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN16QTextFrameFormat9setMarginEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:816
// index:0
// Public inline Visibility=Default Availability=Available
// [8] qreal margin() const

/*

*/
impl /*struct*/ QTextFrameFormat {
  pub fn margin_0<RetType, T: QTextFrameFormat_margin_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.margin_0(self);
    // return 1;
  }
}
pub trait QTextFrameFormat_margin_0<RetType> {
  fn margin_0(self , rsthis: & QTextFrameFormat) -> RetType;
}
impl<'a> /*trait*/ QTextFrameFormat_margin_0<f64> for () {
  fn margin_0(self , rsthis: & QTextFrameFormat) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QTextFrameFormat6marginEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:819
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setTopMargin(qreal)

/*

*/
impl /*struct*/ QTextFrameFormat {
  pub fn setTopMargin_0<RetType, T: QTextFrameFormat_setTopMargin_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTopMargin_0(self);
    // return 1;
  }
}
pub trait QTextFrameFormat_setTopMargin_0<RetType> {
  fn setTopMargin_0(self , rsthis: & QTextFrameFormat) -> RetType;
}
impl<'a> /*trait*/ QTextFrameFormat_setTopMargin_0<(/*void*/)> for (f64) {
  fn setTopMargin_0(self , rsthis: & QTextFrameFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN16QTextFrameFormat12setTopMarginEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:820
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal topMargin() const

/*

*/
impl /*struct*/ QTextFrameFormat {
  pub fn topMargin_0<RetType, T: QTextFrameFormat_topMargin_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.topMargin_0(self);
    // return 1;
  }
}
pub trait QTextFrameFormat_topMargin_0<RetType> {
  fn topMargin_0(self , rsthis: & QTextFrameFormat) -> RetType;
}
impl<'a> /*trait*/ QTextFrameFormat_topMargin_0<f64> for () {
  fn topMargin_0(self , rsthis: & QTextFrameFormat) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QTextFrameFormat9topMarginEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:822
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setBottomMargin(qreal)

/*

*/
impl /*struct*/ QTextFrameFormat {
  pub fn setBottomMargin_0<RetType, T: QTextFrameFormat_setBottomMargin_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setBottomMargin_0(self);
    // return 1;
  }
}
pub trait QTextFrameFormat_setBottomMargin_0<RetType> {
  fn setBottomMargin_0(self , rsthis: & QTextFrameFormat) -> RetType;
}
impl<'a> /*trait*/ QTextFrameFormat_setBottomMargin_0<(/*void*/)> for (f64) {
  fn setBottomMargin_0(self , rsthis: & QTextFrameFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN16QTextFrameFormat15setBottomMarginEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:823
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal bottomMargin() const

/*

*/
impl /*struct*/ QTextFrameFormat {
  pub fn bottomMargin_0<RetType, T: QTextFrameFormat_bottomMargin_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.bottomMargin_0(self);
    // return 1;
  }
}
pub trait QTextFrameFormat_bottomMargin_0<RetType> {
  fn bottomMargin_0(self , rsthis: & QTextFrameFormat) -> RetType;
}
impl<'a> /*trait*/ QTextFrameFormat_bottomMargin_0<f64> for () {
  fn bottomMargin_0(self , rsthis: & QTextFrameFormat) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QTextFrameFormat12bottomMarginEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:825
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setLeftMargin(qreal)

/*

*/
impl /*struct*/ QTextFrameFormat {
  pub fn setLeftMargin_0<RetType, T: QTextFrameFormat_setLeftMargin_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setLeftMargin_0(self);
    // return 1;
  }
}
pub trait QTextFrameFormat_setLeftMargin_0<RetType> {
  fn setLeftMargin_0(self , rsthis: & QTextFrameFormat) -> RetType;
}
impl<'a> /*trait*/ QTextFrameFormat_setLeftMargin_0<(/*void*/)> for (f64) {
  fn setLeftMargin_0(self , rsthis: & QTextFrameFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN16QTextFrameFormat13setLeftMarginEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:826
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal leftMargin() const

/*

*/
impl /*struct*/ QTextFrameFormat {
  pub fn leftMargin_0<RetType, T: QTextFrameFormat_leftMargin_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.leftMargin_0(self);
    // return 1;
  }
}
pub trait QTextFrameFormat_leftMargin_0<RetType> {
  fn leftMargin_0(self , rsthis: & QTextFrameFormat) -> RetType;
}
impl<'a> /*trait*/ QTextFrameFormat_leftMargin_0<f64> for () {
  fn leftMargin_0(self , rsthis: & QTextFrameFormat) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QTextFrameFormat10leftMarginEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:828
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setRightMargin(qreal)

/*

*/
impl /*struct*/ QTextFrameFormat {
  pub fn setRightMargin_0<RetType, T: QTextFrameFormat_setRightMargin_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setRightMargin_0(self);
    // return 1;
  }
}
pub trait QTextFrameFormat_setRightMargin_0<RetType> {
  fn setRightMargin_0(self , rsthis: & QTextFrameFormat) -> RetType;
}
impl<'a> /*trait*/ QTextFrameFormat_setRightMargin_0<(/*void*/)> for (f64) {
  fn setRightMargin_0(self , rsthis: & QTextFrameFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN16QTextFrameFormat14setRightMarginEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:829
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal rightMargin() const

/*

*/
impl /*struct*/ QTextFrameFormat {
  pub fn rightMargin_0<RetType, T: QTextFrameFormat_rightMargin_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rightMargin_0(self);
    // return 1;
  }
}
pub trait QTextFrameFormat_rightMargin_0<RetType> {
  fn rightMargin_0(self , rsthis: & QTextFrameFormat) -> RetType;
}
impl<'a> /*trait*/ QTextFrameFormat_rightMargin_0<f64> for () {
  fn rightMargin_0(self , rsthis: & QTextFrameFormat) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QTextFrameFormat11rightMarginEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:831
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setPadding(qreal)

/*

*/
impl /*struct*/ QTextFrameFormat {
  pub fn setPadding_0<RetType, T: QTextFrameFormat_setPadding_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPadding_0(self);
    // return 1;
  }
}
pub trait QTextFrameFormat_setPadding_0<RetType> {
  fn setPadding_0(self , rsthis: & QTextFrameFormat) -> RetType;
}
impl<'a> /*trait*/ QTextFrameFormat_setPadding_0<(/*void*/)> for (f64) {
  fn setPadding_0(self , rsthis: & QTextFrameFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN16QTextFrameFormat10setPaddingEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:832
// index:0
// Public inline Visibility=Default Availability=Available
// [8] qreal padding() const

/*

*/
impl /*struct*/ QTextFrameFormat {
  pub fn padding_0<RetType, T: QTextFrameFormat_padding_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.padding_0(self);
    // return 1;
  }
}
pub trait QTextFrameFormat_padding_0<RetType> {
  fn padding_0(self , rsthis: & QTextFrameFormat) -> RetType;
}
impl<'a> /*trait*/ QTextFrameFormat_padding_0<f64> for () {
  fn padding_0(self , rsthis: & QTextFrameFormat) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QTextFrameFormat7paddingEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:835
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setWidth(qreal)

/*

*/
impl /*struct*/ QTextFrameFormat {
  pub fn setWidth_0<RetType, T: QTextFrameFormat_setWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setWidth_0(self);
    // return 1;
  }
}
pub trait QTextFrameFormat_setWidth_0<RetType> {
  fn setWidth_0(self , rsthis: & QTextFrameFormat) -> RetType;
}
impl<'a> /*trait*/ QTextFrameFormat_setWidth_0<(/*void*/)> for (f64) {
  fn setWidth_0(self , rsthis: & QTextFrameFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN16QTextFrameFormat8setWidthEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:836
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void setWidth(const QTextLength &)

/*

*/
impl /*struct*/ QTextFrameFormat {
  pub fn setWidth_1<RetType, T: QTextFrameFormat_setWidth_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setWidth_1(self);
    // return 1;
  }
}
pub trait QTextFrameFormat_setWidth_1<RetType> {
  fn setWidth_1(self , rsthis: & QTextFrameFormat) -> RetType;
}
impl<'a> /*trait*/ QTextFrameFormat_setWidth_1<(/*void*/)> for (usize) {
  fn setWidth_1(self , rsthis: & QTextFrameFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN16QTextFrameFormat8setWidthERK11QTextLength", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:838
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QTextLength width() const

/*

*/
impl /*struct*/ QTextFrameFormat {
  pub fn width_0<RetType, T: QTextFrameFormat_width_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.width_0(self);
    // return 1;
  }
}
pub trait QTextFrameFormat_width_0<RetType> {
  fn width_0(self , rsthis: & QTextFrameFormat) -> RetType;
}
impl<'a> /*trait*/ QTextFrameFormat_width_0<usize> for () {
  fn width_0(self , rsthis: & QTextFrameFormat) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QTextFrameFormat5widthEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:841
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setHeight(qreal)

/*

*/
impl /*struct*/ QTextFrameFormat {
  pub fn setHeight_0<RetType, T: QTextFrameFormat_setHeight_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setHeight_0(self);
    // return 1;
  }
}
pub trait QTextFrameFormat_setHeight_0<RetType> {
  fn setHeight_0(self , rsthis: & QTextFrameFormat) -> RetType;
}
impl<'a> /*trait*/ QTextFrameFormat_setHeight_0<(/*void*/)> for (f64) {
  fn setHeight_0(self , rsthis: & QTextFrameFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN16QTextFrameFormat9setHeightEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:842
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void setHeight(const QTextLength &)

/*

*/
impl /*struct*/ QTextFrameFormat {
  pub fn setHeight_1<RetType, T: QTextFrameFormat_setHeight_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setHeight_1(self);
    // return 1;
  }
}
pub trait QTextFrameFormat_setHeight_1<RetType> {
  fn setHeight_1(self , rsthis: & QTextFrameFormat) -> RetType;
}
impl<'a> /*trait*/ QTextFrameFormat_setHeight_1<(/*void*/)> for (usize) {
  fn setHeight_1(self , rsthis: & QTextFrameFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN16QTextFrameFormat9setHeightERK11QTextLength", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:843
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QTextLength height() const

/*

*/
impl /*struct*/ QTextFrameFormat {
  pub fn height_0<RetType, T: QTextFrameFormat_height_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.height_0(self);
    // return 1;
  }
}
pub trait QTextFrameFormat_height_0<RetType> {
  fn height_0(self , rsthis: & QTextFrameFormat) -> RetType;
}
impl<'a> /*trait*/ QTextFrameFormat_height_0<usize> for () {
  fn height_0(self , rsthis: & QTextFrameFormat) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QTextFrameFormat6heightEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:846
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setPageBreakPolicy(QTextFormat::PageBreakFlags)

/*

*/
impl /*struct*/ QTextFrameFormat {
  pub fn setPageBreakPolicy_0<RetType, T: QTextFrameFormat_setPageBreakPolicy_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPageBreakPolicy_0(self);
    // return 1;
  }
}
pub trait QTextFrameFormat_setPageBreakPolicy_0<RetType> {
  fn setPageBreakPolicy_0(self , rsthis: & QTextFrameFormat) -> RetType;
}
impl<'a> /*trait*/ QTextFrameFormat_setPageBreakPolicy_0<(/*void*/)> for (i32) {
  fn setPageBreakPolicy_0(self , rsthis: & QTextFrameFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN16QTextFrameFormat18setPageBreakPolicyE6QFlagsIN11QTextFormat13PageBreakFlagEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:848
// index:0
// Public inline Visibility=Default Availability=Available
// [4] QTextFormat::PageBreakFlags pageBreakPolicy() const

/*

*/
impl /*struct*/ QTextFrameFormat {
  pub fn pageBreakPolicy_0<RetType, T: QTextFrameFormat_pageBreakPolicy_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.pageBreakPolicy_0(self);
    // return 1;
  }
}
pub trait QTextFrameFormat_pageBreakPolicy_0<RetType> {
  fn pageBreakPolicy_0(self , rsthis: & QTextFrameFormat) -> RetType;
}
impl<'a> /*trait*/ QTextFrameFormat_pageBreakPolicy_0<i32> for () {
  fn pageBreakPolicy_0(self , rsthis: & QTextFrameFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QTextFrameFormat15pageBreakPolicyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}


pub fn DeleteQTextFrameFormat(this :*mut QTextFrameFormat) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN16QTextFrameFormatD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}

/*


*/
pub type QTextFrameFormat__Position = i32;
// 
pub const QTextFrameFormat__InFlow :QTextFrameFormat__Position = 0;
// 
pub const QTextFrameFormat__FloatLeft :QTextFrameFormat__Position = 1;
// 
pub const QTextFrameFormat__FloatRight :QTextFrameFormat__Position = 2;
pub fn QTextFrameFormat_PositionItemName(val: i32) ->String {
  match val {
     QTextFrameFormat__InFlow => // 0
     {return String::from("InFlow");}
     QTextFrameFormat__FloatLeft => // 1
     {return String::from("FloatLeft");}
     QTextFrameFormat__FloatRight => // 2
     {return String::from("FloatRight");}
  _ => {return format!("{}", val);}
}
}
pub fn QTextFrameFormat_PositionItemName_s(val: i32) ->String {
  //var nilthis *QTextFrameFormat
  //return nilthis.PositionItemName(val);
  return QTextFrameFormat_PositionItemName(val);
}


/*


*/
pub type QTextFrameFormat__BorderStyle = i32;
// 
pub const QTextFrameFormat__BorderStyle_None :QTextFrameFormat__BorderStyle = 0;
// 
pub const QTextFrameFormat__BorderStyle_Dotted :QTextFrameFormat__BorderStyle = 1;
// 
pub const QTextFrameFormat__BorderStyle_Dashed :QTextFrameFormat__BorderStyle = 2;
// 
pub const QTextFrameFormat__BorderStyle_Solid :QTextFrameFormat__BorderStyle = 3;
// 
pub const QTextFrameFormat__BorderStyle_Double :QTextFrameFormat__BorderStyle = 4;
// 
pub const QTextFrameFormat__BorderStyle_DotDash :QTextFrameFormat__BorderStyle = 5;
// 
pub const QTextFrameFormat__BorderStyle_DotDotDash :QTextFrameFormat__BorderStyle = 6;
// 
pub const QTextFrameFormat__BorderStyle_Groove :QTextFrameFormat__BorderStyle = 7;
// 
pub const QTextFrameFormat__BorderStyle_Ridge :QTextFrameFormat__BorderStyle = 8;
// 
pub const QTextFrameFormat__BorderStyle_Inset :QTextFrameFormat__BorderStyle = 9;
// 
pub const QTextFrameFormat__BorderStyle_Outset :QTextFrameFormat__BorderStyle = 10;
pub fn QTextFrameFormat_BorderStyleItemName(val: i32) ->String {
  match val {
     QTextFrameFormat__BorderStyle_None => // 0
     {return String::from("BorderStyle_None");}
     QTextFrameFormat__BorderStyle_Dotted => // 1
     {return String::from("BorderStyle_Dotted");}
     QTextFrameFormat__BorderStyle_Dashed => // 2
     {return String::from("BorderStyle_Dashed");}
     QTextFrameFormat__BorderStyle_Solid => // 3
     {return String::from("BorderStyle_Solid");}
     QTextFrameFormat__BorderStyle_Double => // 4
     {return String::from("BorderStyle_Double");}
     QTextFrameFormat__BorderStyle_DotDash => // 5
     {return String::from("BorderStyle_DotDash");}
     QTextFrameFormat__BorderStyle_DotDotDash => // 6
     {return String::from("BorderStyle_DotDotDash");}
     QTextFrameFormat__BorderStyle_Groove => // 7
     {return String::from("BorderStyle_Groove");}
     QTextFrameFormat__BorderStyle_Ridge => // 8
     {return String::from("BorderStyle_Ridge");}
     QTextFrameFormat__BorderStyle_Inset => // 9
     {return String::from("BorderStyle_Inset");}
     QTextFrameFormat__BorderStyle_Outset => // 10
     {return String::from("BorderStyle_Outset");}
  _ => {return format!("{}", val);}
}
}
pub fn QTextFrameFormat_BorderStyleItemName_s(val: i32) ->String {
  //var nilthis *QTextFrameFormat
  //return nilthis.BorderStyleItemName(val);
  return QTextFrameFormat_BorderStyleItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
