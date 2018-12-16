

// mod ::widgets::QPlainTextDocumentLayout
// package qtwidgets
// /usr/include/qt/QtWidgets/qplaintextedit.h
// #include <qplaintextedit.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 114
*/
// import "C"
// import "unsafe"
// import "reflect"
// import "fmt"
// import "log"
// import "github.com/kitech/qt.go/qtrt"
// import "github.com/kitech/qt.go/qtcore"
use qtcore::*; // super::super::%!s(MISSING)::*;
// import "github.com/kitech/qt.go/qtgui"
use qtgui::*; // super::super::%!s(MISSING)::*;
use std::default::Default;
use std::ops::Deref;
use qtrt; // super::super::qtrt;
use super::*;
//  ext block end

//  body block begin

// void documentChanged(int, int, int)
// func (this *QPlainTextDocumentLayout) InheritDocumentChanged(f func(from int, arg1 int, charsAdded int) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "documentChanged", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QPlainTextDocumentLayout)=16
pub struct QPlainTextDocumentLayout {
  qbase: QAbstractTextDocumentLayout,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QPlainTextDocumentLayout_ITF interface {
//    qtgui.QAbstractTextDocumentLayout_ITF
//    QPlainTextDocumentLayout_PTR() *QPlainTextDocumentLayout
//}
//func (ptr *QPlainTextDocumentLayout) QPlainTextDocumentLayout_PTR() *QPlainTextDocumentLayout { return ptr }

impl /*struct*/ QPlainTextDocumentLayout {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QPlainTextDocumentLayout {
    return QPlainTextDocumentLayout{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QPlainTextDocumentLayout {
//  type Target = QPlainTextDocumentLayoutBASE;
//
//  fn deref(&self) -> &QPlainTextDocumentLayoutBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QPlainTextDocumentLayoutBASE> for QPlainTextDocumentLayout {
//  fn as_ref(& self) -> & QPlainTextDocumentLayoutBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qplaintextedit.h:297
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QPlainTextDocumentLayout {
  pub fn metaObject_0<RetType, T: QPlainTextDocumentLayout_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QPlainTextDocumentLayout_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QPlainTextDocumentLayout) -> RetType;
}
impl<'a> /*trait*/ QPlainTextDocumentLayout_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QPlainTextDocumentLayout) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK24QPlainTextDocumentLayout10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:302
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QPlainTextDocumentLayout(QTextDocument *)

/*

*/
// QPlainTextDocumentLayout(QTextDocument *) ctx.fn_proto_cpp
impl /*struct*/ QPlainTextDocumentLayout {
  pub fn QPlainTextDocumentLayout_0<T: QPlainTextDocumentLayout_QPlainTextDocumentLayout_0>(value: T) -> QPlainTextDocumentLayout {
    let rsthis = value.QPlainTextDocumentLayout_0();
    return rsthis;
    // return 1;
  }
}

pub trait QPlainTextDocumentLayout_QPlainTextDocumentLayout_0 {
  fn QPlainTextDocumentLayout_0(self) -> QPlainTextDocumentLayout;
}
// QPlainTextDocumentLayout(QTextDocument *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QPlainTextDocumentLayout_QPlainTextDocumentLayout_0 for (usize) {
  fn QPlainTextDocumentLayout_0(self) -> QPlainTextDocumentLayout {
    // unsafe{_ZN24QPlainTextDocumentLayoutC2EP13QTextDocument()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN24QPlainTextDocumentLayoutC2EP13QTextDocument", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QPlainTextDocumentLayout{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:303
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QPlainTextDocumentLayout()

/*

*/
pub fn DeleteQPlainTextDocumentLayout(this :*mut QPlainTextDocumentLayout) {
    // let rv = qtrt::InvokeQtFunc6("_ZN24QPlainTextDocumentLayoutD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qplaintextedit.h:306
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] int hitTest(const QPointF &, Qt::HitTestAccuracy) const

/*

*/
impl /*struct*/ QPlainTextDocumentLayout {
  pub fn hitTest_0<RetType, T: QPlainTextDocumentLayout_hitTest_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hitTest_0(self);
    // return 1;
  }
}
pub trait QPlainTextDocumentLayout_hitTest_0<RetType> {
  fn hitTest_0(self , rsthis: & QPlainTextDocumentLayout) -> RetType;
}
impl<'a> /*trait*/ QPlainTextDocumentLayout_hitTest_0<i32> for (usize,i32) {
  fn hitTest_0(self , rsthis: & QPlainTextDocumentLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK24QPlainTextDocumentLayout7hitTestERK7QPointFN2Qt15HitTestAccuracyE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:308
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] int pageCount() const

/*

*/
impl /*struct*/ QPlainTextDocumentLayout {
  pub fn pageCount_0<RetType, T: QPlainTextDocumentLayout_pageCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.pageCount_0(self);
    // return 1;
  }
}
pub trait QPlainTextDocumentLayout_pageCount_0<RetType> {
  fn pageCount_0(self , rsthis: & QPlainTextDocumentLayout) -> RetType;
}
impl<'a> /*trait*/ QPlainTextDocumentLayout_pageCount_0<i32> for () {
  fn pageCount_0(self , rsthis: & QPlainTextDocumentLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK24QPlainTextDocumentLayout9pageCountEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:309
// index:0
// Public virtual Visibility=Default Availability=Available
// [16] QSizeF documentSize() const

/*

*/
impl /*struct*/ QPlainTextDocumentLayout {
  pub fn documentSize_0<RetType, T: QPlainTextDocumentLayout_documentSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.documentSize_0(self);
    // return 1;
  }
}
pub trait QPlainTextDocumentLayout_documentSize_0<RetType> {
  fn documentSize_0(self , rsthis: & QPlainTextDocumentLayout) -> RetType;
}
impl<'a> /*trait*/ QPlainTextDocumentLayout_documentSize_0<usize> for () {
  fn documentSize_0(self , rsthis: & QPlainTextDocumentLayout) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK24QPlainTextDocumentLayout12documentSizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:311
// index:0
// Public virtual Visibility=Default Availability=Available
// [32] QRectF frameBoundingRect(QTextFrame *) const

/*

*/
impl /*struct*/ QPlainTextDocumentLayout {
  pub fn frameBoundingRect_0<RetType, T: QPlainTextDocumentLayout_frameBoundingRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.frameBoundingRect_0(self);
    // return 1;
  }
}
pub trait QPlainTextDocumentLayout_frameBoundingRect_0<RetType> {
  fn frameBoundingRect_0(self , rsthis: & QPlainTextDocumentLayout) -> RetType;
}
impl<'a> /*trait*/ QPlainTextDocumentLayout_frameBoundingRect_0<usize> for (usize) {
  fn frameBoundingRect_0(self , rsthis: & QPlainTextDocumentLayout) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK24QPlainTextDocumentLayout17frameBoundingRectEP10QTextFrame", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:312
// index:0
// Public virtual Visibility=Default Availability=Available
// [32] QRectF blockBoundingRect(const QTextBlock &) const

/*
Returns the bounding rectangle of the text block in the block's own coordinates.

See also blockBoundingGeometry().
*/
impl /*struct*/ QPlainTextDocumentLayout {
  pub fn blockBoundingRect_0<RetType, T: QPlainTextDocumentLayout_blockBoundingRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.blockBoundingRect_0(self);
    // return 1;
  }
}
pub trait QPlainTextDocumentLayout_blockBoundingRect_0<RetType> {
  fn blockBoundingRect_0(self , rsthis: & QPlainTextDocumentLayout) -> RetType;
}
impl<'a> /*trait*/ QPlainTextDocumentLayout_blockBoundingRect_0<usize> for (usize) {
  fn blockBoundingRect_0(self , rsthis: & QPlainTextDocumentLayout) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK24QPlainTextDocumentLayout17blockBoundingRectERK10QTextBlock", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:314
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ensureBlockLayout(const QTextBlock &) const

/*

*/
impl /*struct*/ QPlainTextDocumentLayout {
  pub fn ensureBlockLayout_0<RetType, T: QPlainTextDocumentLayout_ensureBlockLayout_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.ensureBlockLayout_0(self);
    // return 1;
  }
}
pub trait QPlainTextDocumentLayout_ensureBlockLayout_0<RetType> {
  fn ensureBlockLayout_0(self , rsthis: & QPlainTextDocumentLayout) -> RetType;
}
impl<'a> /*trait*/ QPlainTextDocumentLayout_ensureBlockLayout_0<(/*void*/)> for (usize) {
  fn ensureBlockLayout_0(self , rsthis: & QPlainTextDocumentLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK24QPlainTextDocumentLayout17ensureBlockLayoutERK10QTextBlock", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:316
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCursorWidth(int)

/*

*/
impl /*struct*/ QPlainTextDocumentLayout {
  pub fn setCursorWidth_0<RetType, T: QPlainTextDocumentLayout_setCursorWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCursorWidth_0(self);
    // return 1;
  }
}
pub trait QPlainTextDocumentLayout_setCursorWidth_0<RetType> {
  fn setCursorWidth_0(self , rsthis: & QPlainTextDocumentLayout) -> RetType;
}
impl<'a> /*trait*/ QPlainTextDocumentLayout_setCursorWidth_0<(/*void*/)> for (i32) {
  fn setCursorWidth_0(self , rsthis: & QPlainTextDocumentLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN24QPlainTextDocumentLayout14setCursorWidthEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:317
// index:0
// Public Visibility=Default Availability=Available
// [4] int cursorWidth() const

/*

*/
impl /*struct*/ QPlainTextDocumentLayout {
  pub fn cursorWidth_0<RetType, T: QPlainTextDocumentLayout_cursorWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cursorWidth_0(self);
    // return 1;
  }
}
pub trait QPlainTextDocumentLayout_cursorWidth_0<RetType> {
  fn cursorWidth_0(self , rsthis: & QPlainTextDocumentLayout) -> RetType;
}
impl<'a> /*trait*/ QPlainTextDocumentLayout_cursorWidth_0<i32> for () {
  fn cursorWidth_0(self , rsthis: & QPlainTextDocumentLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK24QPlainTextDocumentLayout11cursorWidthEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:319
// index:0
// Public Visibility=Default Availability=Available
// [-2] void requestUpdate()

/*

*/
impl /*struct*/ QPlainTextDocumentLayout {
  pub fn requestUpdate_0<RetType, T: QPlainTextDocumentLayout_requestUpdate_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.requestUpdate_0(self);
    // return 1;
  }
}
pub trait QPlainTextDocumentLayout_requestUpdate_0<RetType> {
  fn requestUpdate_0(self , rsthis: & QPlainTextDocumentLayout) -> RetType;
}
impl<'a> /*trait*/ QPlainTextDocumentLayout_requestUpdate_0<(/*void*/)> for () {
  fn requestUpdate_0(self , rsthis: & QPlainTextDocumentLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN24QPlainTextDocumentLayout13requestUpdateEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:322
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void documentChanged(int, int, int)

/*

*/
impl /*struct*/ QPlainTextDocumentLayout {
  pub fn documentChanged_0<RetType, T: QPlainTextDocumentLayout_documentChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.documentChanged_0(self);
    // return 1;
  }
}
pub trait QPlainTextDocumentLayout_documentChanged_0<RetType> {
  fn documentChanged_0(self , rsthis: & QPlainTextDocumentLayout) -> RetType;
}
impl<'a> /*trait*/ QPlainTextDocumentLayout_documentChanged_0<(/*void*/)> for (i32,i32,i32) {
  fn documentChanged_0(self , rsthis: & QPlainTextDocumentLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN24QPlainTextDocumentLayout15documentChangedEiii", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

//  body block end

//  keep block begin

//  keep block end
