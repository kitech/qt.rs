

// mod ::gui::QTextTableCellFormat
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
// extern C begin: 14
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
#[derive(Default)] // class sizeof(QTextTableCellFormat)=16
pub struct QTextTableCellFormat {
  qbase: QTextCharFormat,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QTextTableCellFormat_ITF interface {
//    QTextCharFormat_ITF
//    QTextTableCellFormat_PTR() *QTextTableCellFormat
//}
//func (ptr *QTextTableCellFormat) QTextTableCellFormat_PTR() *QTextTableCellFormat { return ptr }

impl /*struct*/ QTextTableCellFormat {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QTextTableCellFormat {
    return QTextTableCellFormat{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QTextTableCellFormat {
//  type Target = QTextTableCellFormatBASE;
//
//  fn deref(&self) -> &QTextTableCellFormatBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QTextTableCellFormatBASE> for QTextTableCellFormat {
//  fn as_ref(& self) -> & QTextTableCellFormatBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qtextformat.h:945
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QTextTableCellFormat()

/*

*/
// QTextTableCellFormat() ctx.fn_proto_cpp
impl /*struct*/ QTextTableCellFormat {
  pub fn QTextTableCellFormat_0<T: QTextTableCellFormat_QTextTableCellFormat_0>(value: T) -> QTextTableCellFormat {
    let rsthis = value.QTextTableCellFormat_0();
    return rsthis;
    // return 1;
  }
}

pub trait QTextTableCellFormat_QTextTableCellFormat_0 {
  fn QTextTableCellFormat_0(self) -> QTextTableCellFormat;
}
// QTextTableCellFormat() ctx.fn_proto_cpp
impl<'a> /*trait*/ QTextTableCellFormat_QTextTableCellFormat_0 for () {
  fn QTextTableCellFormat_0(self) -> QTextTableCellFormat {
    // unsafe{_ZN20QTextTableCellFormatC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN20QTextTableCellFormatC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTextTableCellFormat{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:964
// index:1
// Protected Visibility=Default Availability=Available
// [-2] void QTextTableCellFormat(const QTextFormat &)

/*

*/
// QTextTableCellFormat(const QTextFormat &) ctx.fn_proto_cpp
impl /*struct*/ QTextTableCellFormat {
  pub fn QTextTableCellFormat_1<T: QTextTableCellFormat_QTextTableCellFormat_1>(value: T) -> QTextTableCellFormat {
    let rsthis = value.QTextTableCellFormat_1();
    return rsthis;
    // return 1;
  }
}

pub trait QTextTableCellFormat_QTextTableCellFormat_1 {
  fn QTextTableCellFormat_1(self) -> QTextTableCellFormat;
}
// QTextTableCellFormat(const QTextFormat &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QTextTableCellFormat_QTextTableCellFormat_1 for (usize) {
  fn QTextTableCellFormat_1(self) -> QTextTableCellFormat {
    // unsafe{_ZN20QTextTableCellFormatC2ERK11QTextFormat()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN20QTextTableCellFormatC2ERK11QTextFormat", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTextTableCellFormat{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:947
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isValid() const

/*
Returns true if the format is valid (i.e. is not InvalidFormat); otherwise returns false.
*/
impl /*struct*/ QTextTableCellFormat {
  pub fn isValid_0<RetType, T: QTextTableCellFormat_isValid_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isValid_0(self);
    // return 1;
  }
}
pub trait QTextTableCellFormat_isValid_0<RetType> {
  fn isValid_0(self , rsthis: & QTextTableCellFormat) -> RetType;
}
impl<'a> /*trait*/ QTextTableCellFormat_isValid_0<bool> for () {
  fn isValid_0(self , rsthis: & QTextTableCellFormat) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK20QTextTableCellFormat7isValidEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:949
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setTopPadding(qreal)

/*

*/
impl /*struct*/ QTextTableCellFormat {
  pub fn setTopPadding_0<RetType, T: QTextTableCellFormat_setTopPadding_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTopPadding_0(self);
    // return 1;
  }
}
pub trait QTextTableCellFormat_setTopPadding_0<RetType> {
  fn setTopPadding_0(self , rsthis: & QTextTableCellFormat) -> RetType;
}
impl<'a> /*trait*/ QTextTableCellFormat_setTopPadding_0<(/*void*/)> for (f64) {
  fn setTopPadding_0(self , rsthis: & QTextTableCellFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN20QTextTableCellFormat13setTopPaddingEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:950
// index:0
// Public inline Visibility=Default Availability=Available
// [8] qreal topPadding() const

/*

*/
impl /*struct*/ QTextTableCellFormat {
  pub fn topPadding_0<RetType, T: QTextTableCellFormat_topPadding_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.topPadding_0(self);
    // return 1;
  }
}
pub trait QTextTableCellFormat_topPadding_0<RetType> {
  fn topPadding_0(self , rsthis: & QTextTableCellFormat) -> RetType;
}
impl<'a> /*trait*/ QTextTableCellFormat_topPadding_0<f64> for () {
  fn topPadding_0(self , rsthis: & QTextTableCellFormat) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK20QTextTableCellFormat10topPaddingEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:952
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setBottomPadding(qreal)

/*

*/
impl /*struct*/ QTextTableCellFormat {
  pub fn setBottomPadding_0<RetType, T: QTextTableCellFormat_setBottomPadding_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setBottomPadding_0(self);
    // return 1;
  }
}
pub trait QTextTableCellFormat_setBottomPadding_0<RetType> {
  fn setBottomPadding_0(self , rsthis: & QTextTableCellFormat) -> RetType;
}
impl<'a> /*trait*/ QTextTableCellFormat_setBottomPadding_0<(/*void*/)> for (f64) {
  fn setBottomPadding_0(self , rsthis: & QTextTableCellFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN20QTextTableCellFormat16setBottomPaddingEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:953
// index:0
// Public inline Visibility=Default Availability=Available
// [8] qreal bottomPadding() const

/*

*/
impl /*struct*/ QTextTableCellFormat {
  pub fn bottomPadding_0<RetType, T: QTextTableCellFormat_bottomPadding_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.bottomPadding_0(self);
    // return 1;
  }
}
pub trait QTextTableCellFormat_bottomPadding_0<RetType> {
  fn bottomPadding_0(self , rsthis: & QTextTableCellFormat) -> RetType;
}
impl<'a> /*trait*/ QTextTableCellFormat_bottomPadding_0<f64> for () {
  fn bottomPadding_0(self , rsthis: & QTextTableCellFormat) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK20QTextTableCellFormat13bottomPaddingEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:955
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setLeftPadding(qreal)

/*

*/
impl /*struct*/ QTextTableCellFormat {
  pub fn setLeftPadding_0<RetType, T: QTextTableCellFormat_setLeftPadding_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setLeftPadding_0(self);
    // return 1;
  }
}
pub trait QTextTableCellFormat_setLeftPadding_0<RetType> {
  fn setLeftPadding_0(self , rsthis: & QTextTableCellFormat) -> RetType;
}
impl<'a> /*trait*/ QTextTableCellFormat_setLeftPadding_0<(/*void*/)> for (f64) {
  fn setLeftPadding_0(self , rsthis: & QTextTableCellFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN20QTextTableCellFormat14setLeftPaddingEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:956
// index:0
// Public inline Visibility=Default Availability=Available
// [8] qreal leftPadding() const

/*

*/
impl /*struct*/ QTextTableCellFormat {
  pub fn leftPadding_0<RetType, T: QTextTableCellFormat_leftPadding_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.leftPadding_0(self);
    // return 1;
  }
}
pub trait QTextTableCellFormat_leftPadding_0<RetType> {
  fn leftPadding_0(self , rsthis: & QTextTableCellFormat) -> RetType;
}
impl<'a> /*trait*/ QTextTableCellFormat_leftPadding_0<f64> for () {
  fn leftPadding_0(self , rsthis: & QTextTableCellFormat) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK20QTextTableCellFormat11leftPaddingEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:958
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setRightPadding(qreal)

/*

*/
impl /*struct*/ QTextTableCellFormat {
  pub fn setRightPadding_0<RetType, T: QTextTableCellFormat_setRightPadding_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setRightPadding_0(self);
    // return 1;
  }
}
pub trait QTextTableCellFormat_setRightPadding_0<RetType> {
  fn setRightPadding_0(self , rsthis: & QTextTableCellFormat) -> RetType;
}
impl<'a> /*trait*/ QTextTableCellFormat_setRightPadding_0<(/*void*/)> for (f64) {
  fn setRightPadding_0(self , rsthis: & QTextTableCellFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN20QTextTableCellFormat15setRightPaddingEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:959
// index:0
// Public inline Visibility=Default Availability=Available
// [8] qreal rightPadding() const

/*

*/
impl /*struct*/ QTextTableCellFormat {
  pub fn rightPadding_0<RetType, T: QTextTableCellFormat_rightPadding_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rightPadding_0(self);
    // return 1;
  }
}
pub trait QTextTableCellFormat_rightPadding_0<RetType> {
  fn rightPadding_0(self , rsthis: & QTextTableCellFormat) -> RetType;
}
impl<'a> /*trait*/ QTextTableCellFormat_rightPadding_0<f64> for () {
  fn rightPadding_0(self , rsthis: & QTextTableCellFormat) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK20QTextTableCellFormat12rightPaddingEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:961
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setPadding(qreal)

/*

*/
impl /*struct*/ QTextTableCellFormat {
  pub fn setPadding_0<RetType, T: QTextTableCellFormat_setPadding_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPadding_0(self);
    // return 1;
  }
}
pub trait QTextTableCellFormat_setPadding_0<RetType> {
  fn setPadding_0(self , rsthis: & QTextTableCellFormat) -> RetType;
}
impl<'a> /*trait*/ QTextTableCellFormat_setPadding_0<(/*void*/)> for (f64) {
  fn setPadding_0(self , rsthis: & QTextTableCellFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN20QTextTableCellFormat10setPaddingEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}


pub fn DeleteQTextTableCellFormat(this :*mut QTextTableCellFormat) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN20QTextTableCellFormatD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
