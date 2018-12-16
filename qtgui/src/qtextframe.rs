

// mod ::gui::QTextFrame
// package qtgui
// /usr/include/qt/QtGui/qtextobject.h
// #include <qtextobject.h>
// #include <QtGui>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 1
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
#[derive(Default)] // class sizeof(QTextFrame)=16
pub struct QTextFrame {
  qbase: QTextObject,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QTextFrame_ITF interface {
//    QTextObject_ITF
//    QTextFrame_PTR() *QTextFrame
//}
//func (ptr *QTextFrame) QTextFrame_PTR() *QTextFrame { return ptr }

impl /*struct*/ QTextFrame {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QTextFrame {
    return QTextFrame{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QTextFrame {
//  type Target = QTextFrameBASE;
//
//  fn deref(&self) -> &QTextFrameBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QTextFrameBASE> for QTextFrame {
//  fn as_ref(& self) -> & QTextFrameBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qtextobject.h:120
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QTextFrame {
  pub fn metaObject_0<RetType, T: QTextFrame_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QTextFrame_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QTextFrame) -> RetType;
}
impl<'a> /*trait*/ QTextFrame_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QTextFrame) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTextFrame10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextobject.h:123
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QTextFrame(QTextDocument *)

/*

*/
// QTextFrame(QTextDocument *) ctx.fn_proto_cpp
impl /*struct*/ QTextFrame {
  pub fn QTextFrame_0<T: QTextFrame_QTextFrame_0>(value: T) -> QTextFrame {
    let rsthis = value.QTextFrame_0();
    return rsthis;
    // return 1;
  }
}

pub trait QTextFrame_QTextFrame_0 {
  fn QTextFrame_0(self) -> QTextFrame;
}
// QTextFrame(QTextDocument *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QTextFrame_QTextFrame_0 for (usize) {
  fn QTextFrame_0(self) -> QTextFrame {
    // unsafe{_ZN10QTextFrameC2EP13QTextDocument()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN10QTextFrameC2EP13QTextDocument", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTextFrame{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextobject.h:124
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QTextFrame()

/*

*/
pub fn DeleteQTextFrame(this :*mut QTextFrame) {
    // let rv = qtrt::InvokeQtFunc6("_ZN10QTextFrameD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qtextobject.h:126
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setFrameFormat(const QTextFrameFormat &)

/*

*/
impl /*struct*/ QTextFrame {
  pub fn setFrameFormat_0<RetType, T: QTextFrame_setFrameFormat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFrameFormat_0(self);
    // return 1;
  }
}
pub trait QTextFrame_setFrameFormat_0<RetType> {
  fn setFrameFormat_0(self , rsthis: & QTextFrame) -> RetType;
}
impl<'a> /*trait*/ QTextFrame_setFrameFormat_0<(/*void*/)> for (usize) {
  fn setFrameFormat_0(self , rsthis: & QTextFrame) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN10QTextFrame14setFrameFormatERK16QTextFrameFormat", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextobject.h:127
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QTextFrameFormat frameFormat() const

/*

*/
impl /*struct*/ QTextFrame {
  pub fn frameFormat_0<RetType, T: QTextFrame_frameFormat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.frameFormat_0(self);
    // return 1;
  }
}
pub trait QTextFrame_frameFormat_0<RetType> {
  fn frameFormat_0(self , rsthis: & QTextFrame) -> RetType;
}
impl<'a> /*trait*/ QTextFrame_frameFormat_0<usize> for () {
  fn frameFormat_0(self , rsthis: & QTextFrame) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTextFrame11frameFormatEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextobject.h:129
// index:0
// Public Visibility=Default Availability=Available
// [8] QTextCursor firstCursorPosition() const

/*

*/
impl /*struct*/ QTextFrame {
  pub fn firstCursorPosition_0<RetType, T: QTextFrame_firstCursorPosition_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.firstCursorPosition_0(self);
    // return 1;
  }
}
pub trait QTextFrame_firstCursorPosition_0<RetType> {
  fn firstCursorPosition_0(self , rsthis: & QTextFrame) -> RetType;
}
impl<'a> /*trait*/ QTextFrame_firstCursorPosition_0<usize> for () {
  fn firstCursorPosition_0(self , rsthis: & QTextFrame) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTextFrame19firstCursorPositionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextobject.h:130
// index:0
// Public Visibility=Default Availability=Available
// [8] QTextCursor lastCursorPosition() const

/*

*/
impl /*struct*/ QTextFrame {
  pub fn lastCursorPosition_0<RetType, T: QTextFrame_lastCursorPosition_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.lastCursorPosition_0(self);
    // return 1;
  }
}
pub trait QTextFrame_lastCursorPosition_0<RetType> {
  fn lastCursorPosition_0(self , rsthis: & QTextFrame) -> RetType;
}
impl<'a> /*trait*/ QTextFrame_lastCursorPosition_0<usize> for () {
  fn lastCursorPosition_0(self , rsthis: & QTextFrame) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTextFrame18lastCursorPositionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextobject.h:131
// index:0
// Public Visibility=Default Availability=Available
// [4] int firstPosition() const

/*

*/
impl /*struct*/ QTextFrame {
  pub fn firstPosition_0<RetType, T: QTextFrame_firstPosition_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.firstPosition_0(self);
    // return 1;
  }
}
pub trait QTextFrame_firstPosition_0<RetType> {
  fn firstPosition_0(self , rsthis: & QTextFrame) -> RetType;
}
impl<'a> /*trait*/ QTextFrame_firstPosition_0<i32> for () {
  fn firstPosition_0(self , rsthis: & QTextFrame) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTextFrame13firstPositionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextobject.h:132
// index:0
// Public Visibility=Default Availability=Available
// [4] int lastPosition() const

/*

*/
impl /*struct*/ QTextFrame {
  pub fn lastPosition_0<RetType, T: QTextFrame_lastPosition_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.lastPosition_0(self);
    // return 1;
  }
}
pub trait QTextFrame_lastPosition_0<RetType> {
  fn lastPosition_0(self , rsthis: & QTextFrame) -> RetType;
}
impl<'a> /*trait*/ QTextFrame_lastPosition_0<i32> for () {
  fn lastPosition_0(self , rsthis: & QTextFrame) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTextFrame12lastPositionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextobject.h:134
// index:0
// Public Visibility=Default Availability=Available
// [8] QTextFrameLayoutData * layoutData() const

/*

*/
impl /*struct*/ QTextFrame {
  pub fn layoutData_0<RetType, T: QTextFrame_layoutData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.layoutData_0(self);
    // return 1;
  }
}
pub trait QTextFrame_layoutData_0<RetType> {
  fn layoutData_0(self , rsthis: & QTextFrame) -> RetType;
}
impl<'a> /*trait*/ QTextFrame_layoutData_0<usize> for () {
  fn layoutData_0(self , rsthis: & QTextFrame) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTextFrame10layoutDataEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextobject.h:135
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setLayoutData(QTextFrameLayoutData *)

/*

*/
impl /*struct*/ QTextFrame {
  pub fn setLayoutData_0<RetType, T: QTextFrame_setLayoutData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setLayoutData_0(self);
    // return 1;
  }
}
pub trait QTextFrame_setLayoutData_0<RetType> {
  fn setLayoutData_0(self , rsthis: & QTextFrame) -> RetType;
}
impl<'a> /*trait*/ QTextFrame_setLayoutData_0<(/*void*/)> for (usize) {
  fn setLayoutData_0(self , rsthis: & QTextFrame) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN10QTextFrame13setLayoutDataEP20QTextFrameLayoutData", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextobject.h:138
// index:0
// Public Visibility=Default Availability=Available
// [8] QTextFrame * parentFrame() const

/*

*/
impl /*struct*/ QTextFrame {
  pub fn parentFrame_0<RetType, T: QTextFrame_parentFrame_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.parentFrame_0(self);
    // return 1;
  }
}
pub trait QTextFrame_parentFrame_0<RetType> {
  fn parentFrame_0(self , rsthis: & QTextFrame) -> RetType;
}
impl<'a> /*trait*/ QTextFrame_parentFrame_0<usize> for () {
  fn parentFrame_0(self , rsthis: & QTextFrame) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTextFrame11parentFrameEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextobject.h:181
// index:0
// Public Visibility=Default Availability=Available
// [32] QTextFrame::iterator begin() const

/*

*/
impl /*struct*/ QTextFrame {
  pub fn begin_0<RetType, T: QTextFrame_begin_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.begin_0(self);
    // return 1;
  }
}
pub trait QTextFrame_begin_0<RetType> {
  fn begin_0(self , rsthis: & QTextFrame) -> RetType;
}
impl<'a> /*trait*/ QTextFrame_begin_0<usize> for () {
  fn begin_0(self , rsthis: & QTextFrame) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTextFrame5beginEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextobject.h:182
// index:0
// Public Visibility=Default Availability=Available
// [32] QTextFrame::iterator end() const

/*

*/
impl /*struct*/ QTextFrame {
  pub fn end_0<RetType, T: QTextFrame_end_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.end_0(self);
    // return 1;
  }
}
pub trait QTextFrame_end_0<RetType> {
  fn end_0(self , rsthis: & QTextFrame) -> RetType;
}
impl<'a> /*trait*/ QTextFrame_end_0<usize> for () {
  fn end_0(self , rsthis: & QTextFrame) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTextFrame3endEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
