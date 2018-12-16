

// mod ::gui::QAbstractTextDocumentLayout
// package qtgui
// /usr/include/qt/QtGui/qabstracttextdocumentlayout.h
// #include <qabstracttextdocumentlayout.h>
// #include <QtGui>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 51
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

// void documentChanged(int, int, int)
// func (this *QAbstractTextDocumentLayout) InheritDocumentChanged(f func(from int, charsRemoved int, charsAdded int) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "documentChanged", f)
// }

// void resizeInlineObject(QTextInlineObject, int, const QTextFormat &)
// func (this *QAbstractTextDocumentLayout) InheritResizeInlineObject(f func(item *QTextInlineObject/*123*/, posInDocument int, format *QTextFormat) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "resizeInlineObject", f)
// }

// void positionInlineObject(QTextInlineObject, int, const QTextFormat &)
// func (this *QAbstractTextDocumentLayout) InheritPositionInlineObject(f func(item *QTextInlineObject/*123*/, posInDocument int, format *QTextFormat) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "positionInlineObject", f)
// }

// void drawInlineObject(QPainter *, const QRectF &, QTextInlineObject, int, const QTextFormat &)
// func (this *QAbstractTextDocumentLayout) InheritDrawInlineObject(f func(painter *QPainter/*777 QPainter **/, rect *qtcore.QRectF, object *QTextInlineObject/*123*/, posInDocument int, format *QTextFormat) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "drawInlineObject", f)
// }

// int formatIndex(int)
// func (this *QAbstractTextDocumentLayout) InheritFormatIndex(f func(pos int) int) {
//  qtrt.SetAllInheritCallback(this, "formatIndex", f)
// }

// QTextCharFormat format(int)
// func (this *QAbstractTextDocumentLayout) InheritFormat(f func(pos int) unsafe.Pointer) {
//  qtrt.SetAllInheritCallback(this, "format", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QAbstractTextDocumentLayout)=16
pub struct QAbstractTextDocumentLayout {
  qbase: QObject,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QAbstractTextDocumentLayout_ITF interface {
//    qtcore.QObject_ITF
//    QAbstractTextDocumentLayout_PTR() *QAbstractTextDocumentLayout
//}
//func (ptr *QAbstractTextDocumentLayout) QAbstractTextDocumentLayout_PTR() *QAbstractTextDocumentLayout { return ptr }

impl /*struct*/ QAbstractTextDocumentLayout {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QAbstractTextDocumentLayout {
    return QAbstractTextDocumentLayout{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QAbstractTextDocumentLayout {
//  type Target = QAbstractTextDocumentLayoutBASE;
//
//  fn deref(&self) -> &QAbstractTextDocumentLayoutBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QAbstractTextDocumentLayoutBASE> for QAbstractTextDocumentLayout {
//  fn as_ref(& self) -> & QAbstractTextDocumentLayoutBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qabstracttextdocumentlayout.h:60
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QAbstractTextDocumentLayout {
  pub fn metaObject_0<RetType, T: QAbstractTextDocumentLayout_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QAbstractTextDocumentLayout_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QAbstractTextDocumentLayout) -> RetType;
}
impl<'a> /*trait*/ QAbstractTextDocumentLayout_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QAbstractTextDocumentLayout) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK27QAbstractTextDocumentLayout10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qabstracttextdocumentlayout.h:64
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QAbstractTextDocumentLayout(QTextDocument *)

/*
Creates a new text document layout for the given document.
*/
// QAbstractTextDocumentLayout(QTextDocument *) ctx.fn_proto_cpp
impl /*struct*/ QAbstractTextDocumentLayout {
  pub fn QAbstractTextDocumentLayout_0<T: QAbstractTextDocumentLayout_QAbstractTextDocumentLayout_0>(value: T) -> QAbstractTextDocumentLayout {
    let rsthis = value.QAbstractTextDocumentLayout_0();
    return rsthis;
    // return 1;
  }
}

pub trait QAbstractTextDocumentLayout_QAbstractTextDocumentLayout_0 {
  fn QAbstractTextDocumentLayout_0(self) -> QAbstractTextDocumentLayout;
}
// QAbstractTextDocumentLayout(QTextDocument *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QAbstractTextDocumentLayout_QAbstractTextDocumentLayout_0 for (usize) {
  fn QAbstractTextDocumentLayout_0(self) -> QAbstractTextDocumentLayout {
    // unsafe{_ZN27QAbstractTextDocumentLayoutC2EP13QTextDocument()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN27QAbstractTextDocumentLayoutC2EP13QTextDocument", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QAbstractTextDocumentLayout{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qabstracttextdocumentlayout.h:65
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QAbstractTextDocumentLayout()

/*

*/
pub fn DeleteQAbstractTextDocumentLayout(this :*mut QAbstractTextDocumentLayout) {
    // let rv = qtrt::InvokeQtFunc6("_ZN27QAbstractTextDocumentLayoutD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qabstracttextdocumentlayout.h:85
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [4] int hitTest(const QPointF &, Qt::HitTestAccuracy) const

/*
Returns the cursor position for the given point with the specified accuracy. Returns -1 if no valid cursor position was found.
*/
impl /*struct*/ QAbstractTextDocumentLayout {
  pub fn hitTest_0<RetType, T: QAbstractTextDocumentLayout_hitTest_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hitTest_0(self);
    // return 1;
  }
}
pub trait QAbstractTextDocumentLayout_hitTest_0<RetType> {
  fn hitTest_0(self , rsthis: & QAbstractTextDocumentLayout) -> RetType;
}
impl<'a> /*trait*/ QAbstractTextDocumentLayout_hitTest_0<i32> for (usize,i32) {
  fn hitTest_0(self , rsthis: & QAbstractTextDocumentLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK27QAbstractTextDocumentLayout7hitTestERK7QPointFN2Qt15HitTestAccuracyE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qabstracttextdocumentlayout.h:87
// index:0
// Public Visibility=Default Availability=Available
// [8] QString anchorAt(const QPointF &) const

/*
Returns the reference of the anchor the given position, or an empty string if no anchor exists at that point.
*/
impl /*struct*/ QAbstractTextDocumentLayout {
  pub fn anchorAt_0<RetType, T: QAbstractTextDocumentLayout_anchorAt_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.anchorAt_0(self);
    // return 1;
  }
}
pub trait QAbstractTextDocumentLayout_anchorAt_0<RetType> {
  fn anchorAt_0(self , rsthis: & QAbstractTextDocumentLayout) -> RetType;
}
impl<'a> /*trait*/ QAbstractTextDocumentLayout_anchorAt_0<usize> for (usize) {
  fn anchorAt_0(self , rsthis: & QAbstractTextDocumentLayout) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK27QAbstractTextDocumentLayout8anchorAtERK7QPointF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qabstracttextdocumentlayout.h:88
// index:0
// Public Visibility=Default Availability=Available
// [8] QString imageAt(const QPointF &) const

/*
Returns the source of the image at the given position pos, or an empty string if no image exists at that point.

This function was introduced in  Qt 5.8.
*/
impl /*struct*/ QAbstractTextDocumentLayout {
  pub fn imageAt_0<RetType, T: QAbstractTextDocumentLayout_imageAt_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.imageAt_0(self);
    // return 1;
  }
}
pub trait QAbstractTextDocumentLayout_imageAt_0<RetType> {
  fn imageAt_0(self , rsthis: & QAbstractTextDocumentLayout) -> RetType;
}
impl<'a> /*trait*/ QAbstractTextDocumentLayout_imageAt_0<usize> for (usize) {
  fn imageAt_0(self , rsthis: & QAbstractTextDocumentLayout) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK27QAbstractTextDocumentLayout7imageAtERK7QPointF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qabstracttextdocumentlayout.h:89
// index:0
// Public Visibility=Default Availability=Available
// [16] QTextFormat formatAt(const QPointF &) const

/*
Returns the text format at the given position pos.

This function was introduced in  Qt 5.8.
*/
impl /*struct*/ QAbstractTextDocumentLayout {
  pub fn formatAt_0<RetType, T: QAbstractTextDocumentLayout_formatAt_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.formatAt_0(self);
    // return 1;
  }
}
pub trait QAbstractTextDocumentLayout_formatAt_0<RetType> {
  fn formatAt_0(self , rsthis: & QAbstractTextDocumentLayout) -> RetType;
}
impl<'a> /*trait*/ QAbstractTextDocumentLayout_formatAt_0<usize> for (usize) {
  fn formatAt_0(self , rsthis: & QAbstractTextDocumentLayout) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK27QAbstractTextDocumentLayout8formatAtERK7QPointF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qabstracttextdocumentlayout.h:91
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [4] int pageCount() const

/*
Returns the number of pages contained in the layout.

See also pageCountChanged().
*/
impl /*struct*/ QAbstractTextDocumentLayout {
  pub fn pageCount_0<RetType, T: QAbstractTextDocumentLayout_pageCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.pageCount_0(self);
    // return 1;
  }
}
pub trait QAbstractTextDocumentLayout_pageCount_0<RetType> {
  fn pageCount_0(self , rsthis: & QAbstractTextDocumentLayout) -> RetType;
}
impl<'a> /*trait*/ QAbstractTextDocumentLayout_pageCount_0<i32> for () {
  fn pageCount_0(self , rsthis: & QAbstractTextDocumentLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK27QAbstractTextDocumentLayout9pageCountEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qabstracttextdocumentlayout.h:92
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [16] QSizeF documentSize() const

/*
Returns the total size of the document's layout.

This information can be used by display widgets to update their scroll bars correctly.

See also documentSizeChanged() and QTextDocument::pageSize.
*/
impl /*struct*/ QAbstractTextDocumentLayout {
  pub fn documentSize_0<RetType, T: QAbstractTextDocumentLayout_documentSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.documentSize_0(self);
    // return 1;
  }
}
pub trait QAbstractTextDocumentLayout_documentSize_0<RetType> {
  fn documentSize_0(self , rsthis: & QAbstractTextDocumentLayout) -> RetType;
}
impl<'a> /*trait*/ QAbstractTextDocumentLayout_documentSize_0<usize> for () {
  fn documentSize_0(self , rsthis: & QAbstractTextDocumentLayout) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK27QAbstractTextDocumentLayout12documentSizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qabstracttextdocumentlayout.h:94
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [32] QRectF frameBoundingRect(QTextFrame *) const

/*
Returns the bounding rectangle of frame.
*/
impl /*struct*/ QAbstractTextDocumentLayout {
  pub fn frameBoundingRect_0<RetType, T: QAbstractTextDocumentLayout_frameBoundingRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.frameBoundingRect_0(self);
    // return 1;
  }
}
pub trait QAbstractTextDocumentLayout_frameBoundingRect_0<RetType> {
  fn frameBoundingRect_0(self , rsthis: & QAbstractTextDocumentLayout) -> RetType;
}
impl<'a> /*trait*/ QAbstractTextDocumentLayout_frameBoundingRect_0<usize> for (usize) {
  fn frameBoundingRect_0(self , rsthis: & QAbstractTextDocumentLayout) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK27QAbstractTextDocumentLayout17frameBoundingRectEP10QTextFrame", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qabstracttextdocumentlayout.h:95
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [32] QRectF blockBoundingRect(const QTextBlock &) const

/*
Returns the bounding rectangle of block.
*/
impl /*struct*/ QAbstractTextDocumentLayout {
  pub fn blockBoundingRect_0<RetType, T: QAbstractTextDocumentLayout_blockBoundingRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.blockBoundingRect_0(self);
    // return 1;
  }
}
pub trait QAbstractTextDocumentLayout_blockBoundingRect_0<RetType> {
  fn blockBoundingRect_0(self , rsthis: & QAbstractTextDocumentLayout) -> RetType;
}
impl<'a> /*trait*/ QAbstractTextDocumentLayout_blockBoundingRect_0<usize> for (usize) {
  fn blockBoundingRect_0(self , rsthis: & QAbstractTextDocumentLayout) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK27QAbstractTextDocumentLayout17blockBoundingRectERK10QTextBlock", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qabstracttextdocumentlayout.h:97
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setPaintDevice(QPaintDevice *)

/*
Sets the paint device used for rendering the document's layout to the given device.

See also paintDevice().
*/
impl /*struct*/ QAbstractTextDocumentLayout {
  pub fn setPaintDevice_0<RetType, T: QAbstractTextDocumentLayout_setPaintDevice_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPaintDevice_0(self);
    // return 1;
  }
}
pub trait QAbstractTextDocumentLayout_setPaintDevice_0<RetType> {
  fn setPaintDevice_0(self , rsthis: & QAbstractTextDocumentLayout) -> RetType;
}
impl<'a> /*trait*/ QAbstractTextDocumentLayout_setPaintDevice_0<(/*void*/)> for (usize) {
  fn setPaintDevice_0(self , rsthis: & QAbstractTextDocumentLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN27QAbstractTextDocumentLayout14setPaintDeviceEP12QPaintDevice", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qabstracttextdocumentlayout.h:98
// index:0
// Public Visibility=Default Availability=Available
// [8] QPaintDevice * paintDevice() const

/*
Returns the paint device used to render the document's layout.

See also setPaintDevice().
*/
impl /*struct*/ QAbstractTextDocumentLayout {
  pub fn paintDevice_0<RetType, T: QAbstractTextDocumentLayout_paintDevice_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.paintDevice_0(self);
    // return 1;
  }
}
pub trait QAbstractTextDocumentLayout_paintDevice_0<RetType> {
  fn paintDevice_0(self , rsthis: & QAbstractTextDocumentLayout) -> RetType;
}
impl<'a> /*trait*/ QAbstractTextDocumentLayout_paintDevice_0<usize> for () {
  fn paintDevice_0(self , rsthis: & QAbstractTextDocumentLayout) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK27QAbstractTextDocumentLayout11paintDeviceEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qabstracttextdocumentlayout.h:100
// index:0
// Public Visibility=Default Availability=Available
// [8] QTextDocument * document() const

/*
Returns the text document that this layout is operating on.
*/
impl /*struct*/ QAbstractTextDocumentLayout {
  pub fn document_0<RetType, T: QAbstractTextDocumentLayout_document_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.document_0(self);
    // return 1;
  }
}
pub trait QAbstractTextDocumentLayout_document_0<RetType> {
  fn document_0(self , rsthis: & QAbstractTextDocumentLayout) -> RetType;
}
impl<'a> /*trait*/ QAbstractTextDocumentLayout_document_0<usize> for () {
  fn document_0(self , rsthis: & QAbstractTextDocumentLayout) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK27QAbstractTextDocumentLayout8documentEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qabstracttextdocumentlayout.h:102
// index:0
// Public Visibility=Default Availability=Available
// [-2] void registerHandler(int, QObject *)

/*
Registers the given component as a handler for items of the given objectType.

Note: registerHandler() has to be called once for each object type. This means that there is only one handler for multiple replacement characters of the same object type.

The text document layout does not take ownership of component.
*/
impl /*struct*/ QAbstractTextDocumentLayout {
  pub fn registerHandler_0<RetType, T: QAbstractTextDocumentLayout_registerHandler_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.registerHandler_0(self);
    // return 1;
  }
}
pub trait QAbstractTextDocumentLayout_registerHandler_0<RetType> {
  fn registerHandler_0(self , rsthis: & QAbstractTextDocumentLayout) -> RetType;
}
impl<'a> /*trait*/ QAbstractTextDocumentLayout_registerHandler_0<(/*void*/)> for (i32,usize) {
  fn registerHandler_0(self , rsthis: & QAbstractTextDocumentLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN27QAbstractTextDocumentLayout15registerHandlerEiP7QObject", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qabstracttextdocumentlayout.h:103
// index:0
// Public Visibility=Default Availability=Available
// [-2] void unregisterHandler(int, QObject *)

/*
Unregisters the given component as a handler for items of the given objectType, or any handler if the component is not specified.

This function was introduced in  Qt 5.2.
*/
impl /*struct*/ QAbstractTextDocumentLayout {
  pub fn unregisterHandler_0<RetType, T: QAbstractTextDocumentLayout_unregisterHandler_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.unregisterHandler_0(self);
    // return 1;
  }
}
pub trait QAbstractTextDocumentLayout_unregisterHandler_0<RetType> {
  fn unregisterHandler_0(self , rsthis: & QAbstractTextDocumentLayout) -> RetType;
}
impl<'a> /*trait*/ QAbstractTextDocumentLayout_unregisterHandler_0<(/*void*/)> for (i32,usize) {
  fn unregisterHandler_0(self , rsthis: & QAbstractTextDocumentLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN27QAbstractTextDocumentLayout17unregisterHandlerEiP7QObject", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qabstracttextdocumentlayout.h:104
// index:0
// Public Visibility=Default Availability=Available
// [8] QTextObjectInterface * handlerForObject(int) const

/*
Returns a handler for objects of the given objectType.
*/
impl /*struct*/ QAbstractTextDocumentLayout {
  pub fn handlerForObject_0<RetType, T: QAbstractTextDocumentLayout_handlerForObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.handlerForObject_0(self);
    // return 1;
  }
}
pub trait QAbstractTextDocumentLayout_handlerForObject_0<RetType> {
  fn handlerForObject_0(self , rsthis: & QAbstractTextDocumentLayout) -> RetType;
}
impl<'a> /*trait*/ QAbstractTextDocumentLayout_handlerForObject_0<usize> for (i32) {
  fn handlerForObject_0(self , rsthis: & QAbstractTextDocumentLayout) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK27QAbstractTextDocumentLayout16handlerForObjectEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qabstracttextdocumentlayout.h:107
// index:0
// Public Visibility=Default Availability=Available
// [-2] void update(const QRectF &)

/*
This signal is emitted when the rectangle rect has been updated.

Subclasses of QAbstractTextDocumentLayout should emit this signal when the layout of the contents change in order to repaint.
*/
impl /*struct*/ QAbstractTextDocumentLayout {
  pub fn update_0<RetType, T: QAbstractTextDocumentLayout_update_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.update_0(self);
    // return 1;
  }
}
pub trait QAbstractTextDocumentLayout_update_0<RetType> {
  fn update_0(self , rsthis: & QAbstractTextDocumentLayout) -> RetType;
}
impl<'a> /*trait*/ QAbstractTextDocumentLayout_update_0<(/*void*/)> for (usize) {
  fn update_0(self , rsthis: & QAbstractTextDocumentLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN27QAbstractTextDocumentLayout6updateERK6QRectF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qabstracttextdocumentlayout.h:108
// index:0
// Public Visibility=Default Availability=Available
// [-2] void updateBlock(const QTextBlock &)

/*
This signal is emitted when the specified block has been updated.

Subclasses of QAbstractTextDocumentLayout should emit this signal when the layout of block has changed in order to repaint.

This function was introduced in  Qt 4.4.
*/
impl /*struct*/ QAbstractTextDocumentLayout {
  pub fn updateBlock_0<RetType, T: QAbstractTextDocumentLayout_updateBlock_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.updateBlock_0(self);
    // return 1;
  }
}
pub trait QAbstractTextDocumentLayout_updateBlock_0<RetType> {
  fn updateBlock_0(self , rsthis: & QAbstractTextDocumentLayout) -> RetType;
}
impl<'a> /*trait*/ QAbstractTextDocumentLayout_updateBlock_0<(/*void*/)> for (usize) {
  fn updateBlock_0(self , rsthis: & QAbstractTextDocumentLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN27QAbstractTextDocumentLayout11updateBlockERK10QTextBlock", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qabstracttextdocumentlayout.h:109
// index:0
// Public Visibility=Default Availability=Available
// [-2] void documentSizeChanged(const QSizeF &)

/*
This signal is emitted when the size of the document layout changes to newSize.

Subclasses of QAbstractTextDocumentLayout should emit this signal when the document's entire layout size changes. This signal is useful for widgets that display text documents since it enables them to update their scroll bars correctly.

See also documentSize().
*/
impl /*struct*/ QAbstractTextDocumentLayout {
  pub fn documentSizeChanged_0<RetType, T: QAbstractTextDocumentLayout_documentSizeChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.documentSizeChanged_0(self);
    // return 1;
  }
}
pub trait QAbstractTextDocumentLayout_documentSizeChanged_0<RetType> {
  fn documentSizeChanged_0(self , rsthis: & QAbstractTextDocumentLayout) -> RetType;
}
impl<'a> /*trait*/ QAbstractTextDocumentLayout_documentSizeChanged_0<(/*void*/)> for (usize) {
  fn documentSizeChanged_0(self , rsthis: & QAbstractTextDocumentLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN27QAbstractTextDocumentLayout19documentSizeChangedERK6QSizeF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qabstracttextdocumentlayout.h:110
// index:0
// Public Visibility=Default Availability=Available
// [-2] void pageCountChanged(int)

/*
This signal is emitted when the number of pages in the layout changes; newPages is the updated page count.

Subclasses of QAbstractTextDocumentLayout should emit this signal when the number of pages in the layout has changed. Changes to the page count are caused by changes to the layout or the document content itself.

See also pageCount().
*/
impl /*struct*/ QAbstractTextDocumentLayout {
  pub fn pageCountChanged_0<RetType, T: QAbstractTextDocumentLayout_pageCountChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.pageCountChanged_0(self);
    // return 1;
  }
}
pub trait QAbstractTextDocumentLayout_pageCountChanged_0<RetType> {
  fn pageCountChanged_0(self , rsthis: & QAbstractTextDocumentLayout) -> RetType;
}
impl<'a> /*trait*/ QAbstractTextDocumentLayout_pageCountChanged_0<(/*void*/)> for (i32) {
  fn pageCountChanged_0(self , rsthis: & QAbstractTextDocumentLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN27QAbstractTextDocumentLayout16pageCountChangedEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qabstracttextdocumentlayout.h:115
// index:0
// Protected purevirtual virtual Visibility=Default Availability=Available
// [-2] void documentChanged(int, int, int)

/*
This function is called whenever the contents of the document change. A change occurs when text is inserted, removed, or a combination of these two. The change is specified by position, charsRemoved, and charsAdded corresponding to the starting character position of the change, the number of characters removed from the document, and the number of characters added.

For example, when inserting the text "Hello" into an empty document, charsRemoved would be 0 and charsAdded would be 5 (the length of the string).

Replacing text is a combination of removing and inserting. For example, if the text "Hello" gets replaced by "Hi", charsRemoved would be 5 and charsAdded would be 2.

For subclasses of QAbstractTextDocumentLayout, this is the central function where a large portion of the work to lay out and position document contents is done.

For example, in a subclass that only arranges blocks of text, an implementation of this function would have to do the following:


Determine the list of changed QTextBlock(s) using the parameters provided.
Each QTextBlock object's corresponding QTextLayout object needs to be processed. You can access the QTextBlock's layout using the QTextBlock::layout() function. This processing should take the document's page size into consideration.
If the total number of pages changed, the pageCountChanged() signal should be emitted.
If the total size changed, the documentSizeChanged() signal should be emitted.
The update() signal should be emitted to schedule a repaint of areas in the layout that require repainting.


See also QTextLayout.
*/
impl /*struct*/ QAbstractTextDocumentLayout {
  pub fn documentChanged_0<RetType, T: QAbstractTextDocumentLayout_documentChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.documentChanged_0(self);
    // return 1;
  }
}
pub trait QAbstractTextDocumentLayout_documentChanged_0<RetType> {
  fn documentChanged_0(self , rsthis: & QAbstractTextDocumentLayout) -> RetType;
}
impl<'a> /*trait*/ QAbstractTextDocumentLayout_documentChanged_0<(/*void*/)> for (i32,i32,i32) {
  fn documentChanged_0(self , rsthis: & QAbstractTextDocumentLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN27QAbstractTextDocumentLayout15documentChangedEiii", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qabstracttextdocumentlayout.h:117
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void resizeInlineObject(QTextInlineObject, int, const QTextFormat &)

/*
Sets the size of the inline object item corresponding to the text format.

posInDocument specifies the position of the object within the document.

The default implementation resizes the item to the size returned by the object handler's intrinsicSize() function. This function is called only within Qt. Subclasses can reimplement this function to customize the resizing of inline objects.
*/
impl /*struct*/ QAbstractTextDocumentLayout {
  pub fn resizeInlineObject_0<RetType, T: QAbstractTextDocumentLayout_resizeInlineObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resizeInlineObject_0(self);
    // return 1;
  }
}
pub trait QAbstractTextDocumentLayout_resizeInlineObject_0<RetType> {
  fn resizeInlineObject_0(self , rsthis: & QAbstractTextDocumentLayout) -> RetType;
}
impl<'a> /*trait*/ QAbstractTextDocumentLayout_resizeInlineObject_0<(/*void*/)> for (usize,i32,usize) {
  fn resizeInlineObject_0(self , rsthis: & QAbstractTextDocumentLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN27QAbstractTextDocumentLayout18resizeInlineObjectE17QTextInlineObjectiRK11QTextFormat", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qabstracttextdocumentlayout.h:118
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void positionInlineObject(QTextInlineObject, int, const QTextFormat &)

/*
Lays out the inline object item using the given text format.

posInDocument specifies the position of the object within the document.

The default implementation does nothing. This function is called only within Qt. Subclasses can reimplement this function to customize the position of inline objects.

See also drawInlineObject().
*/
impl /*struct*/ QAbstractTextDocumentLayout {
  pub fn positionInlineObject_0<RetType, T: QAbstractTextDocumentLayout_positionInlineObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.positionInlineObject_0(self);
    // return 1;
  }
}
pub trait QAbstractTextDocumentLayout_positionInlineObject_0<RetType> {
  fn positionInlineObject_0(self , rsthis: & QAbstractTextDocumentLayout) -> RetType;
}
impl<'a> /*trait*/ QAbstractTextDocumentLayout_positionInlineObject_0<(/*void*/)> for (usize,i32,usize) {
  fn positionInlineObject_0(self , rsthis: & QAbstractTextDocumentLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN27QAbstractTextDocumentLayout20positionInlineObjectE17QTextInlineObjectiRK11QTextFormat", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qabstracttextdocumentlayout.h:119
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void drawInlineObject(QPainter *, const QRectF &, QTextInlineObject, int, const QTextFormat &)

/*
This function is called to draw the inline object, object, with the given painter within the rectangle specified by rect using the specified text format.

posInDocument specifies the position of the object within the document.

The default implementation calls drawObject() on the object handlers. This function is called only within Qt. Subclasses can reimplement this function to customize the drawing of inline objects.

See also draw().
*/
impl /*struct*/ QAbstractTextDocumentLayout {
  pub fn drawInlineObject_0<RetType, T: QAbstractTextDocumentLayout_drawInlineObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawInlineObject_0(self);
    // return 1;
  }
}
pub trait QAbstractTextDocumentLayout_drawInlineObject_0<RetType> {
  fn drawInlineObject_0(self , rsthis: & QAbstractTextDocumentLayout) -> RetType;
}
impl<'a> /*trait*/ QAbstractTextDocumentLayout_drawInlineObject_0<(/*void*/)> for (usize,usize,usize,i32,usize) {
  fn drawInlineObject_0(self , rsthis: & QAbstractTextDocumentLayout) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let arg4 = (&self.4/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN27QAbstractTextDocumentLayout16drawInlineObjectEP8QPainterRK6QRectF17QTextInlineObjectiRK11QTextFormat", 5,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qabstracttextdocumentlayout.h:121
// index:0
// Protected Visibility=Default Availability=Available
// [4] int formatIndex(int)

/*

*/
impl /*struct*/ QAbstractTextDocumentLayout {
  pub fn formatIndex_0<RetType, T: QAbstractTextDocumentLayout_formatIndex_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.formatIndex_0(self);
    // return 1;
  }
}
pub trait QAbstractTextDocumentLayout_formatIndex_0<RetType> {
  fn formatIndex_0(self , rsthis: & QAbstractTextDocumentLayout) -> RetType;
}
impl<'a> /*trait*/ QAbstractTextDocumentLayout_formatIndex_0<i32> for (i32) {
  fn formatIndex_0(self , rsthis: & QAbstractTextDocumentLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN27QAbstractTextDocumentLayout11formatIndexEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qabstracttextdocumentlayout.h:122
// index:0
// Protected Visibility=Default Availability=Available
// [16] QTextCharFormat format(int)

/*
Returns the character format that is applicable at the given position.
*/
impl /*struct*/ QAbstractTextDocumentLayout {
  pub fn format_0<RetType, T: QAbstractTextDocumentLayout_format_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.format_0(self);
    // return 1;
  }
}
pub trait QAbstractTextDocumentLayout_format_0<RetType> {
  fn format_0(self , rsthis: & QAbstractTextDocumentLayout) -> RetType;
}
impl<'a> /*trait*/ QAbstractTextDocumentLayout_format_0<usize> for (i32) {
  fn format_0(self , rsthis: & QAbstractTextDocumentLayout) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN27QAbstractTextDocumentLayout6formatEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
