

// mod ::widgets::QPlainTextEdit
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
// extern C begin: 126
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

// bool event(QEvent *)
// func (this *QPlainTextEdit) InheritEvent(f func(e *qtcore.QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "event", f)
// }

// void timerEvent(QTimerEvent *)
// func (this *QPlainTextEdit) InheritTimerEvent(f func(e *qtcore.QTimerEvent/*777 QTimerEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "timerEvent", f)
// }

// void keyPressEvent(QKeyEvent *)
// func (this *QPlainTextEdit) InheritKeyPressEvent(f func(e *qtgui.QKeyEvent/*777 QKeyEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "keyPressEvent", f)
// }

// void keyReleaseEvent(QKeyEvent *)
// func (this *QPlainTextEdit) InheritKeyReleaseEvent(f func(e *qtgui.QKeyEvent/*777 QKeyEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "keyReleaseEvent", f)
// }

// void resizeEvent(QResizeEvent *)
// func (this *QPlainTextEdit) InheritResizeEvent(f func(e *qtgui.QResizeEvent/*777 QResizeEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "resizeEvent", f)
// }

// void paintEvent(QPaintEvent *)
// func (this *QPlainTextEdit) InheritPaintEvent(f func(e *qtgui.QPaintEvent/*777 QPaintEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "paintEvent", f)
// }

// void mousePressEvent(QMouseEvent *)
// func (this *QPlainTextEdit) InheritMousePressEvent(f func(e *qtgui.QMouseEvent/*777 QMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mousePressEvent", f)
// }

// void mouseMoveEvent(QMouseEvent *)
// func (this *QPlainTextEdit) InheritMouseMoveEvent(f func(e *qtgui.QMouseEvent/*777 QMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mouseMoveEvent", f)
// }

// void mouseReleaseEvent(QMouseEvent *)
// func (this *QPlainTextEdit) InheritMouseReleaseEvent(f func(e *qtgui.QMouseEvent/*777 QMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mouseReleaseEvent", f)
// }

// void mouseDoubleClickEvent(QMouseEvent *)
// func (this *QPlainTextEdit) InheritMouseDoubleClickEvent(f func(e *qtgui.QMouseEvent/*777 QMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mouseDoubleClickEvent", f)
// }

// bool focusNextPrevChild(bool)
// func (this *QPlainTextEdit) InheritFocusNextPrevChild(f func(next bool) bool) {
//  qtrt.SetAllInheritCallback(this, "focusNextPrevChild", f)
// }

// void contextMenuEvent(QContextMenuEvent *)
// func (this *QPlainTextEdit) InheritContextMenuEvent(f func(e *qtgui.QContextMenuEvent/*777 QContextMenuEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "contextMenuEvent", f)
// }

// void dragEnterEvent(QDragEnterEvent *)
// func (this *QPlainTextEdit) InheritDragEnterEvent(f func(e *qtgui.QDragEnterEvent/*777 QDragEnterEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "dragEnterEvent", f)
// }

// void dragLeaveEvent(QDragLeaveEvent *)
// func (this *QPlainTextEdit) InheritDragLeaveEvent(f func(e *qtgui.QDragLeaveEvent/*777 QDragLeaveEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "dragLeaveEvent", f)
// }

// void dragMoveEvent(QDragMoveEvent *)
// func (this *QPlainTextEdit) InheritDragMoveEvent(f func(e *qtgui.QDragMoveEvent/*777 QDragMoveEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "dragMoveEvent", f)
// }

// void dropEvent(QDropEvent *)
// func (this *QPlainTextEdit) InheritDropEvent(f func(e *qtgui.QDropEvent/*777 QDropEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "dropEvent", f)
// }

// void focusInEvent(QFocusEvent *)
// func (this *QPlainTextEdit) InheritFocusInEvent(f func(e *qtgui.QFocusEvent/*777 QFocusEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "focusInEvent", f)
// }

// void focusOutEvent(QFocusEvent *)
// func (this *QPlainTextEdit) InheritFocusOutEvent(f func(e *qtgui.QFocusEvent/*777 QFocusEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "focusOutEvent", f)
// }

// void showEvent(QShowEvent *)
// func (this *QPlainTextEdit) InheritShowEvent(f func(arg0 *qtgui.QShowEvent/*777 QShowEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "showEvent", f)
// }

// void changeEvent(QEvent *)
// func (this *QPlainTextEdit) InheritChangeEvent(f func(e *qtcore.QEvent/*777 QEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "changeEvent", f)
// }

// void wheelEvent(QWheelEvent *)
// func (this *QPlainTextEdit) InheritWheelEvent(f func(e *qtgui.QWheelEvent/*777 QWheelEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "wheelEvent", f)
// }

// QMimeData * createMimeDataFromSelection()
// func (this *QPlainTextEdit) InheritCreateMimeDataFromSelection(f func() unsafe.Pointer/*666*/) {
//  qtrt.SetAllInheritCallback(this, "createMimeDataFromSelection", f)
// }

// bool canInsertFromMimeData(const QMimeData *)
// func (this *QPlainTextEdit) InheritCanInsertFromMimeData(f func(source *qtcore.QMimeData/*777 const QMimeData **/) bool) {
//  qtrt.SetAllInheritCallback(this, "canInsertFromMimeData", f)
// }

// void insertFromMimeData(const QMimeData *)
// func (this *QPlainTextEdit) InheritInsertFromMimeData(f func(source *qtcore.QMimeData/*777 const QMimeData **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "insertFromMimeData", f)
// }

// void inputMethodEvent(QInputMethodEvent *)
// func (this *QPlainTextEdit) InheritInputMethodEvent(f func(arg0 *qtgui.QInputMethodEvent/*777 QInputMethodEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "inputMethodEvent", f)
// }

// void scrollContentsBy(int, int)
// func (this *QPlainTextEdit) InheritScrollContentsBy(f func(dx int, dy int) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "scrollContentsBy", f)
// }

// void doSetTextCursor(const QTextCursor &)
// func (this *QPlainTextEdit) InheritDoSetTextCursor(f func(cursor *qtgui.QTextCursor) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "doSetTextCursor", f)
// }

// QTextBlock firstVisibleBlock()
// func (this *QPlainTextEdit) InheritFirstVisibleBlock(f func() unsafe.Pointer) {
//  qtrt.SetAllInheritCallback(this, "firstVisibleBlock", f)
// }

// QPointF contentOffset()
// func (this *QPlainTextEdit) InheritContentOffset(f func() unsafe.Pointer) {
//  qtrt.SetAllInheritCallback(this, "contentOffset", f)
// }

// QRectF blockBoundingRect(const QTextBlock &)
// func (this *QPlainTextEdit) InheritBlockBoundingRect(f func(block *qtgui.QTextBlock) unsafe.Pointer) {
//  qtrt.SetAllInheritCallback(this, "blockBoundingRect", f)
// }

// QRectF blockBoundingGeometry(const QTextBlock &)
// func (this *QPlainTextEdit) InheritBlockBoundingGeometry(f func(block *qtgui.QTextBlock) unsafe.Pointer) {
//  qtrt.SetAllInheritCallback(this, "blockBoundingGeometry", f)
// }

// QAbstractTextDocumentLayout::PaintContext getPaintContext()
// func (this *QPlainTextEdit) InheritGetPaintContext(f func() int) {
//  qtrt.SetAllInheritCallback(this, "getPaintContext", f)
// }

// void zoomInF(float)
// func (this *QPlainTextEdit) InheritZoomInF(f func(range_ float32) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "zoomInF", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QPlainTextEdit)=48
pub struct QPlainTextEdit {
  qbase: QAbstractScrollArea,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QPlainTextEdit_ITF interface {
//    QAbstractScrollArea_ITF
//    QPlainTextEdit_PTR() *QPlainTextEdit
//}
//func (ptr *QPlainTextEdit) QPlainTextEdit_PTR() *QPlainTextEdit { return ptr }

impl /*struct*/ QPlainTextEdit {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QPlainTextEdit {
    return QPlainTextEdit{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QPlainTextEdit {
//  type Target = QPlainTextEditBASE;
//
//  fn deref(&self) -> &QPlainTextEditBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QPlainTextEditBASE> for QPlainTextEdit {
//  fn as_ref(& self) -> & QPlainTextEditBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qplaintextedit.h:66
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QPlainTextEdit {
  pub fn metaObject_0<RetType, T: QPlainTextEdit_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QPlainTextEdit) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QPlainTextEdit10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:94
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QPlainTextEdit(QWidget *)

/*
Constructs an empty QPlainTextEdit with parent parent.
*/
// QPlainTextEdit(QWidget *) ctx.fn_proto_cpp
impl /*struct*/ QPlainTextEdit {
  pub fn QPlainTextEdit_0<T: QPlainTextEdit_QPlainTextEdit_0>(value: T) -> QPlainTextEdit {
    let rsthis = value.QPlainTextEdit_0();
    return rsthis;
    // return 1;
  }
}

pub trait QPlainTextEdit_QPlainTextEdit_0 {
  fn QPlainTextEdit_0(self) -> QPlainTextEdit;
}
// QPlainTextEdit(QWidget *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QPlainTextEdit_QPlainTextEdit_0 for (usize) {
  fn QPlainTextEdit_0(self) -> QPlainTextEdit {
    // unsafe{_ZN14QPlainTextEditC2EP7QWidget()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN14QPlainTextEditC2EP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QPlainTextEdit{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:95
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QPlainTextEdit(const QString &, QWidget *)

/*
Constructs an empty QPlainTextEdit with parent parent.
*/
// QPlainTextEdit(const QString &, QWidget *) ctx.fn_proto_cpp
impl /*struct*/ QPlainTextEdit {
  pub fn QPlainTextEdit_1<T: QPlainTextEdit_QPlainTextEdit_1>(value: T) -> QPlainTextEdit {
    let rsthis = value.QPlainTextEdit_1();
    return rsthis;
    // return 1;
  }
}

pub trait QPlainTextEdit_QPlainTextEdit_1 {
  fn QPlainTextEdit_1(self) -> QPlainTextEdit;
}
// QPlainTextEdit(const QString &, QWidget *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QPlainTextEdit_QPlainTextEdit_1 for (usize,usize) {
  fn QPlainTextEdit_1(self) -> QPlainTextEdit {
    // unsafe{_ZN14QPlainTextEditC2ERK7QStringP7QWidget()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN14QPlainTextEditC2ERK7QStringP7QWidget", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QPlainTextEdit{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:96
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QPlainTextEdit()

/*

*/
pub fn DeleteQPlainTextEdit(this :*mut QPlainTextEdit) {
    // let rv = qtrt::InvokeQtFunc6("_ZN14QPlainTextEditD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 48)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qplaintextedit.h:98
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDocument(QTextDocument *)

/*
Makes document the new document of the text editor.

The parent QObject of the provided document remains the owner of the object. If the current document is a child of the text editor, then it is deleted.

The document must have a document layout that inherits QPlainTextDocumentLayout (see QTextDocument::setDocumentLayout()).

See also document().
*/
impl /*struct*/ QPlainTextEdit {
  pub fn setDocument_0<RetType, T: QPlainTextEdit_setDocument_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDocument_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_setDocument_0<RetType> {
  fn setDocument_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_setDocument_0<(/*void*/)> for (usize) {
  fn setDocument_0(self , rsthis: & QPlainTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN14QPlainTextEdit11setDocumentEP13QTextDocument", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:99
// index:0
// Public Visibility=Default Availability=Available
// [8] QTextDocument * document() const

/*
Returns a pointer to the underlying document.

See also setDocument().
*/
impl /*struct*/ QPlainTextEdit {
  pub fn document_0<RetType, T: QPlainTextEdit_document_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.document_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_document_0<RetType> {
  fn document_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_document_0<usize> for () {
  fn document_0(self , rsthis: & QPlainTextEdit) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QPlainTextEdit8documentEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:101
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setPlaceholderText(const QString &)

/*

*/
impl /*struct*/ QPlainTextEdit {
  pub fn setPlaceholderText_0<RetType, T: QPlainTextEdit_setPlaceholderText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPlaceholderText_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_setPlaceholderText_0<RetType> {
  fn setPlaceholderText_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_setPlaceholderText_0<(/*void*/)> for (usize) {
  fn setPlaceholderText_0(self , rsthis: & QPlainTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN14QPlainTextEdit18setPlaceholderTextERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:102
// index:0
// Public Visibility=Default Availability=Available
// [8] QString placeholderText() const

/*

*/
impl /*struct*/ QPlainTextEdit {
  pub fn placeholderText_0<RetType, T: QPlainTextEdit_placeholderText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.placeholderText_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_placeholderText_0<RetType> {
  fn placeholderText_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_placeholderText_0<usize> for () {
  fn placeholderText_0(self , rsthis: & QPlainTextEdit) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QPlainTextEdit15placeholderTextEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:104
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTextCursor(const QTextCursor &)

/*
Sets the visible cursor.

See also textCursor().
*/
impl /*struct*/ QPlainTextEdit {
  pub fn setTextCursor_0<RetType, T: QPlainTextEdit_setTextCursor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTextCursor_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_setTextCursor_0<RetType> {
  fn setTextCursor_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_setTextCursor_0<(/*void*/)> for (usize) {
  fn setTextCursor_0(self , rsthis: & QPlainTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN14QPlainTextEdit13setTextCursorERK11QTextCursor", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:105
// index:0
// Public Visibility=Default Availability=Available
// [8] QTextCursor textCursor() const

/*
Returns a copy of the QTextCursor that represents the currently visible cursor. Note that changes on the returned cursor do not affect QPlainTextEdit's cursor; use setTextCursor() to update the visible cursor.

See also setTextCursor().
*/
impl /*struct*/ QPlainTextEdit {
  pub fn textCursor_0<RetType, T: QPlainTextEdit_textCursor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.textCursor_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_textCursor_0<RetType> {
  fn textCursor_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_textCursor_0<usize> for () {
  fn textCursor_0(self , rsthis: & QPlainTextEdit) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QPlainTextEdit10textCursorEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:107
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isReadOnly() const

/*

*/
impl /*struct*/ QPlainTextEdit {
  pub fn isReadOnly_0<RetType, T: QPlainTextEdit_isReadOnly_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isReadOnly_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_isReadOnly_0<RetType> {
  fn isReadOnly_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_isReadOnly_0<bool> for () {
  fn isReadOnly_0(self , rsthis: & QPlainTextEdit) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QPlainTextEdit10isReadOnlyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:108
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setReadOnly(bool)

/*

*/
impl /*struct*/ QPlainTextEdit {
  pub fn setReadOnly_0<RetType, T: QPlainTextEdit_setReadOnly_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setReadOnly_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_setReadOnly_0<RetType> {
  fn setReadOnly_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_setReadOnly_0<(/*void*/)> for (bool) {
  fn setReadOnly_0(self , rsthis: & QPlainTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN14QPlainTextEdit11setReadOnlyEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:110
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTextInteractionFlags(Qt::TextInteractionFlags)

/*

*/
impl /*struct*/ QPlainTextEdit {
  pub fn setTextInteractionFlags_0<RetType, T: QPlainTextEdit_setTextInteractionFlags_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTextInteractionFlags_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_setTextInteractionFlags_0<RetType> {
  fn setTextInteractionFlags_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_setTextInteractionFlags_0<(/*void*/)> for (i32) {
  fn setTextInteractionFlags_0(self , rsthis: & QPlainTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN14QPlainTextEdit23setTextInteractionFlagsE6QFlagsIN2Qt19TextInteractionFlagEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:111
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::TextInteractionFlags textInteractionFlags() const

/*

*/
impl /*struct*/ QPlainTextEdit {
  pub fn textInteractionFlags_0<RetType, T: QPlainTextEdit_textInteractionFlags_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.textInteractionFlags_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_textInteractionFlags_0<RetType> {
  fn textInteractionFlags_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_textInteractionFlags_0<i32> for () {
  fn textInteractionFlags_0(self , rsthis: & QPlainTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QPlainTextEdit20textInteractionFlagsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:113
// index:0
// Public Visibility=Default Availability=Available
// [-2] void mergeCurrentCharFormat(const QTextCharFormat &)

/*
Merges the properties specified in modifier into the current character format by calling QTextCursor::mergeCharFormat on the editor's cursor. If the editor has a selection then the properties of modifier are directly applied to the selection.

See also QTextCursor::mergeCharFormat().
*/
impl /*struct*/ QPlainTextEdit {
  pub fn mergeCurrentCharFormat_0<RetType, T: QPlainTextEdit_mergeCurrentCharFormat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mergeCurrentCharFormat_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_mergeCurrentCharFormat_0<RetType> {
  fn mergeCurrentCharFormat_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_mergeCurrentCharFormat_0<(/*void*/)> for (usize) {
  fn mergeCurrentCharFormat_0(self , rsthis: & QPlainTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN14QPlainTextEdit22mergeCurrentCharFormatERK15QTextCharFormat", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:114
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCurrentCharFormat(const QTextCharFormat &)

/*
Sets the char format that is be used when inserting new text to format by calling QTextCursor::setCharFormat() on the editor's cursor. If the editor has a selection then the char format is directly applied to the selection.

See also currentCharFormat().
*/
impl /*struct*/ QPlainTextEdit {
  pub fn setCurrentCharFormat_0<RetType, T: QPlainTextEdit_setCurrentCharFormat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCurrentCharFormat_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_setCurrentCharFormat_0<RetType> {
  fn setCurrentCharFormat_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_setCurrentCharFormat_0<(/*void*/)> for (usize) {
  fn setCurrentCharFormat_0(self , rsthis: & QPlainTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN14QPlainTextEdit20setCurrentCharFormatERK15QTextCharFormat", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:115
// index:0
// Public Visibility=Default Availability=Available
// [16] QTextCharFormat currentCharFormat() const

/*
Returns the char format that is used when inserting new text.

See also setCurrentCharFormat().
*/
impl /*struct*/ QPlainTextEdit {
  pub fn currentCharFormat_0<RetType, T: QPlainTextEdit_currentCharFormat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.currentCharFormat_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_currentCharFormat_0<RetType> {
  fn currentCharFormat_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_currentCharFormat_0<usize> for () {
  fn currentCharFormat_0(self , rsthis: & QPlainTextEdit) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QPlainTextEdit17currentCharFormatEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:117
// index:0
// Public Visibility=Default Availability=Available
// [1] bool tabChangesFocus() const

/*

*/
impl /*struct*/ QPlainTextEdit {
  pub fn tabChangesFocus_0<RetType, T: QPlainTextEdit_tabChangesFocus_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.tabChangesFocus_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_tabChangesFocus_0<RetType> {
  fn tabChangesFocus_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_tabChangesFocus_0<bool> for () {
  fn tabChangesFocus_0(self , rsthis: & QPlainTextEdit) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QPlainTextEdit15tabChangesFocusEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:118
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTabChangesFocus(bool)

/*

*/
impl /*struct*/ QPlainTextEdit {
  pub fn setTabChangesFocus_0<RetType, T: QPlainTextEdit_setTabChangesFocus_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTabChangesFocus_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_setTabChangesFocus_0<RetType> {
  fn setTabChangesFocus_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_setTabChangesFocus_0<(/*void*/)> for (bool) {
  fn setTabChangesFocus_0(self , rsthis: & QPlainTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN14QPlainTextEdit18setTabChangesFocusEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:120
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setDocumentTitle(const QString &)

/*

*/
impl /*struct*/ QPlainTextEdit {
  pub fn setDocumentTitle_0<RetType, T: QPlainTextEdit_setDocumentTitle_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDocumentTitle_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_setDocumentTitle_0<RetType> {
  fn setDocumentTitle_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_setDocumentTitle_0<(/*void*/)> for (usize) {
  fn setDocumentTitle_0(self , rsthis: & QPlainTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN14QPlainTextEdit16setDocumentTitleERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:122
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QString documentTitle() const

/*

*/
impl /*struct*/ QPlainTextEdit {
  pub fn documentTitle_0<RetType, T: QPlainTextEdit_documentTitle_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.documentTitle_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_documentTitle_0<RetType> {
  fn documentTitle_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_documentTitle_0<usize> for () {
  fn documentTitle_0(self , rsthis: & QPlainTextEdit) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QPlainTextEdit13documentTitleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:125
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isUndoRedoEnabled() const

/*

*/
impl /*struct*/ QPlainTextEdit {
  pub fn isUndoRedoEnabled_0<RetType, T: QPlainTextEdit_isUndoRedoEnabled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isUndoRedoEnabled_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_isUndoRedoEnabled_0<RetType> {
  fn isUndoRedoEnabled_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_isUndoRedoEnabled_0<bool> for () {
  fn isUndoRedoEnabled_0(self , rsthis: & QPlainTextEdit) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QPlainTextEdit17isUndoRedoEnabledEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:127
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setUndoRedoEnabled(bool)

/*

*/
impl /*struct*/ QPlainTextEdit {
  pub fn setUndoRedoEnabled_0<RetType, T: QPlainTextEdit_setUndoRedoEnabled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setUndoRedoEnabled_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_setUndoRedoEnabled_0<RetType> {
  fn setUndoRedoEnabled_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_setUndoRedoEnabled_0<(/*void*/)> for (bool) {
  fn setUndoRedoEnabled_0(self , rsthis: & QPlainTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN14QPlainTextEdit18setUndoRedoEnabledEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:130
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setMaximumBlockCount(int)

/*

*/
impl /*struct*/ QPlainTextEdit {
  pub fn setMaximumBlockCount_0<RetType, T: QPlainTextEdit_setMaximumBlockCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMaximumBlockCount_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_setMaximumBlockCount_0<RetType> {
  fn setMaximumBlockCount_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_setMaximumBlockCount_0<(/*void*/)> for (i32) {
  fn setMaximumBlockCount_0(self , rsthis: & QPlainTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN14QPlainTextEdit20setMaximumBlockCountEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:132
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int maximumBlockCount() const

/*

*/
impl /*struct*/ QPlainTextEdit {
  pub fn maximumBlockCount_0<RetType, T: QPlainTextEdit_maximumBlockCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.maximumBlockCount_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_maximumBlockCount_0<RetType> {
  fn maximumBlockCount_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_maximumBlockCount_0<i32> for () {
  fn maximumBlockCount_0(self , rsthis: & QPlainTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QPlainTextEdit17maximumBlockCountEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:136
// index:0
// Public Visibility=Default Availability=Available
// [4] QPlainTextEdit::LineWrapMode lineWrapMode() const

/*

*/
impl /*struct*/ QPlainTextEdit {
  pub fn lineWrapMode_0<RetType, T: QPlainTextEdit_lineWrapMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.lineWrapMode_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_lineWrapMode_0<RetType> {
  fn lineWrapMode_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_lineWrapMode_0<i32> for () {
  fn lineWrapMode_0(self , rsthis: & QPlainTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QPlainTextEdit12lineWrapModeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:137
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setLineWrapMode(QPlainTextEdit::LineWrapMode)

/*

*/
impl /*struct*/ QPlainTextEdit {
  pub fn setLineWrapMode_0<RetType, T: QPlainTextEdit_setLineWrapMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setLineWrapMode_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_setLineWrapMode_0<RetType> {
  fn setLineWrapMode_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_setLineWrapMode_0<(/*void*/)> for (i32) {
  fn setLineWrapMode_0(self , rsthis: & QPlainTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN14QPlainTextEdit15setLineWrapModeENS_12LineWrapModeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:139
// index:0
// Public Visibility=Default Availability=Available
// [4] QTextOption::WrapMode wordWrapMode() const

/*

*/
impl /*struct*/ QPlainTextEdit {
  pub fn wordWrapMode_0<RetType, T: QPlainTextEdit_wordWrapMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.wordWrapMode_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_wordWrapMode_0<RetType> {
  fn wordWrapMode_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_wordWrapMode_0<i32> for () {
  fn wordWrapMode_0(self , rsthis: & QPlainTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QPlainTextEdit12wordWrapModeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:140
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setWordWrapMode(QTextOption::WrapMode)

/*

*/
impl /*struct*/ QPlainTextEdit {
  pub fn setWordWrapMode_0<RetType, T: QPlainTextEdit_setWordWrapMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setWordWrapMode_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_setWordWrapMode_0<RetType> {
  fn setWordWrapMode_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_setWordWrapMode_0<(/*void*/)> for (i32) {
  fn setWordWrapMode_0(self , rsthis: & QPlainTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN14QPlainTextEdit15setWordWrapModeEN11QTextOption8WrapModeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:142
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setBackgroundVisible(bool)

/*

*/
impl /*struct*/ QPlainTextEdit {
  pub fn setBackgroundVisible_0<RetType, T: QPlainTextEdit_setBackgroundVisible_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setBackgroundVisible_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_setBackgroundVisible_0<RetType> {
  fn setBackgroundVisible_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_setBackgroundVisible_0<(/*void*/)> for (bool) {
  fn setBackgroundVisible_0(self , rsthis: & QPlainTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN14QPlainTextEdit20setBackgroundVisibleEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:143
// index:0
// Public Visibility=Default Availability=Available
// [1] bool backgroundVisible() const

/*

*/
impl /*struct*/ QPlainTextEdit {
  pub fn backgroundVisible_0<RetType, T: QPlainTextEdit_backgroundVisible_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.backgroundVisible_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_backgroundVisible_0<RetType> {
  fn backgroundVisible_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_backgroundVisible_0<bool> for () {
  fn backgroundVisible_0(self , rsthis: & QPlainTextEdit) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QPlainTextEdit17backgroundVisibleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:145
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCenterOnScroll(bool)

/*

*/
impl /*struct*/ QPlainTextEdit {
  pub fn setCenterOnScroll_0<RetType, T: QPlainTextEdit_setCenterOnScroll_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCenterOnScroll_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_setCenterOnScroll_0<RetType> {
  fn setCenterOnScroll_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_setCenterOnScroll_0<(/*void*/)> for (bool) {
  fn setCenterOnScroll_0(self , rsthis: & QPlainTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN14QPlainTextEdit17setCenterOnScrollEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:146
// index:0
// Public Visibility=Default Availability=Available
// [1] bool centerOnScroll() const

/*

*/
impl /*struct*/ QPlainTextEdit {
  pub fn centerOnScroll_0<RetType, T: QPlainTextEdit_centerOnScroll_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.centerOnScroll_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_centerOnScroll_0<RetType> {
  fn centerOnScroll_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_centerOnScroll_0<bool> for () {
  fn centerOnScroll_0(self , rsthis: & QPlainTextEdit) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QPlainTextEdit14centerOnScrollEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:148
// index:0
// Public Visibility=Default Availability=Available
// [1] bool find(const QString &, QTextDocument::FindFlags)

/*
Finds the next occurrence of the string, exp, using the given options. Returns true if exp was found and changes the cursor to select the match; otherwise returns false.
*/
impl /*struct*/ QPlainTextEdit {
  pub fn find_0<RetType, T: QPlainTextEdit_find_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.find_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_find_0<RetType> {
  fn find_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_find_0<bool> for (usize,i32) {
  fn find_0(self , rsthis: & QPlainTextEdit) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN14QPlainTextEdit4findERK7QString6QFlagsIN13QTextDocument8FindFlagEE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:150
// index:1
// Public Visibility=Default Availability=Available
// [1] bool find(const QRegExp &, QTextDocument::FindFlags)

/*
Finds the next occurrence of the string, exp, using the given options. Returns true if exp was found and changes the cursor to select the match; otherwise returns false.
*/
impl /*struct*/ QPlainTextEdit {
  pub fn find_1<RetType, T: QPlainTextEdit_find_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.find_1(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_find_1<RetType> {
  fn find_1(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_find_1<bool> for (usize,i32) {
  fn find_1(self , rsthis: & QPlainTextEdit) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN14QPlainTextEdit4findERK7QRegExp6QFlagsIN13QTextDocument8FindFlagEE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:153
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QString toPlainText() const

/*
Returns the text of the text edit as plain text.

Note: Getter function for property plainText. 

See also QPlainTextEdit::setPlainText().
*/
impl /*struct*/ QPlainTextEdit {
  pub fn toPlainText_0<RetType, T: QPlainTextEdit_toPlainText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toPlainText_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_toPlainText_0<RetType> {
  fn toPlainText_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_toPlainText_0<usize> for () {
  fn toPlainText_0(self , rsthis: & QPlainTextEdit) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QPlainTextEdit11toPlainTextEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:156
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ensureCursorVisible()

/*
Ensures that the cursor is visible by scrolling the text edit if necessary.

See also centerCursor() and centerOnScroll.
*/
impl /*struct*/ QPlainTextEdit {
  pub fn ensureCursorVisible_0<RetType, T: QPlainTextEdit_ensureCursorVisible_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.ensureCursorVisible_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_ensureCursorVisible_0<RetType> {
  fn ensureCursorVisible_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_ensureCursorVisible_0<(/*void*/)> for () {
  fn ensureCursorVisible_0(self , rsthis: & QPlainTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN14QPlainTextEdit19ensureCursorVisibleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:158
// index:0
// Public virtual Visibility=Default Availability=Available
// [16] QVariant loadResource(int, const QUrl &)

/*
Loads the resource specified by the given type and name.

This function is an extension of QTextDocument::loadResource().

See also QTextDocument::loadResource().
*/
impl /*struct*/ QPlainTextEdit {
  pub fn loadResource_0<RetType, T: QPlainTextEdit_loadResource_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.loadResource_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_loadResource_0<RetType> {
  fn loadResource_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_loadResource_0<usize> for (i32,usize) {
  fn loadResource_0(self , rsthis: & QPlainTextEdit) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN14QPlainTextEdit12loadResourceEiRK4QUrl", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:160
// index:0
// Public Visibility=Default Availability=Available
// [8] QMenu * createStandardContextMenu()

/*
This function creates the standard context menu which is shown when the user clicks on the text edit with the right mouse button. It is called from the default contextMenuEvent() handler. The popup menu's ownership is transferred to the caller.

We recommend that you use the createStandardContextMenu(QPoint) version instead which will enable the actions that are sensitive to where the user clicked.
*/
impl /*struct*/ QPlainTextEdit {
  pub fn createStandardContextMenu_0<RetType, T: QPlainTextEdit_createStandardContextMenu_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.createStandardContextMenu_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_createStandardContextMenu_0<RetType> {
  fn createStandardContextMenu_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_createStandardContextMenu_0<usize> for () {
  fn createStandardContextMenu_0(self , rsthis: & QPlainTextEdit) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN14QPlainTextEdit25createStandardContextMenuEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:161
// index:1
// Public Visibility=Default Availability=Available
// [8] QMenu * createStandardContextMenu(const QPoint &)

/*
This function creates the standard context menu which is shown when the user clicks on the text edit with the right mouse button. It is called from the default contextMenuEvent() handler. The popup menu's ownership is transferred to the caller.

We recommend that you use the createStandardContextMenu(QPoint) version instead which will enable the actions that are sensitive to where the user clicked.
*/
impl /*struct*/ QPlainTextEdit {
  pub fn createStandardContextMenu_1<RetType, T: QPlainTextEdit_createStandardContextMenu_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.createStandardContextMenu_1(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_createStandardContextMenu_1<RetType> {
  fn createStandardContextMenu_1(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_createStandardContextMenu_1<usize> for (usize) {
  fn createStandardContextMenu_1(self , rsthis: & QPlainTextEdit) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN14QPlainTextEdit25createStandardContextMenuERK6QPoint", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:164
// index:0
// Public Visibility=Default Availability=Available
// [8] QTextCursor cursorForPosition(const QPoint &) const

/*
returns a QTextCursor at position pos (in viewport coordinates).
*/
impl /*struct*/ QPlainTextEdit {
  pub fn cursorForPosition_0<RetType, T: QPlainTextEdit_cursorForPosition_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cursorForPosition_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_cursorForPosition_0<RetType> {
  fn cursorForPosition_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_cursorForPosition_0<usize> for (usize) {
  fn cursorForPosition_0(self , rsthis: & QPlainTextEdit) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QPlainTextEdit17cursorForPositionERK6QPoint", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:165
// index:0
// Public Visibility=Default Availability=Available
// [16] QRect cursorRect(const QTextCursor &) const

/*
returns a rectangle (in viewport coordinates) that includes the cursor.
*/
impl /*struct*/ QPlainTextEdit {
  pub fn cursorRect_0<RetType, T: QPlainTextEdit_cursorRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cursorRect_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_cursorRect_0<RetType> {
  fn cursorRect_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_cursorRect_0<usize> for (usize) {
  fn cursorRect_0(self , rsthis: & QPlainTextEdit) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QPlainTextEdit10cursorRectERK11QTextCursor", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:166
// index:1
// Public Visibility=Default Availability=Available
// [16] QRect cursorRect() const

/*
returns a rectangle (in viewport coordinates) that includes the cursor.
*/
impl /*struct*/ QPlainTextEdit {
  pub fn cursorRect_1<RetType, T: QPlainTextEdit_cursorRect_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cursorRect_1(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_cursorRect_1<RetType> {
  fn cursorRect_1(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_cursorRect_1<usize> for () {
  fn cursorRect_1(self , rsthis: & QPlainTextEdit) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QPlainTextEdit10cursorRectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:168
// index:0
// Public Visibility=Default Availability=Available
// [8] QString anchorAt(const QPoint &) const

/*
Returns the reference of the anchor at position pos, or an empty string if no anchor exists at that point.

This function was introduced in  Qt 4.7.
*/
impl /*struct*/ QPlainTextEdit {
  pub fn anchorAt_0<RetType, T: QPlainTextEdit_anchorAt_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.anchorAt_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_anchorAt_0<RetType> {
  fn anchorAt_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_anchorAt_0<usize> for (usize) {
  fn anchorAt_0(self , rsthis: & QPlainTextEdit) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QPlainTextEdit8anchorAtERK6QPoint", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:170
// index:0
// Public Visibility=Default Availability=Available
// [1] bool overwriteMode() const

/*

*/
impl /*struct*/ QPlainTextEdit {
  pub fn overwriteMode_0<RetType, T: QPlainTextEdit_overwriteMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.overwriteMode_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_overwriteMode_0<RetType> {
  fn overwriteMode_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_overwriteMode_0<bool> for () {
  fn overwriteMode_0(self , rsthis: & QPlainTextEdit) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QPlainTextEdit13overwriteModeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:171
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setOverwriteMode(bool)

/*

*/
impl /*struct*/ QPlainTextEdit {
  pub fn setOverwriteMode_0<RetType, T: QPlainTextEdit_setOverwriteMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setOverwriteMode_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_setOverwriteMode_0<RetType> {
  fn setOverwriteMode_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_setOverwriteMode_0<(/*void*/)> for (bool) {
  fn setOverwriteMode_0(self , rsthis: & QPlainTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN14QPlainTextEdit16setOverwriteModeEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:174
// index:0
// Public Visibility=Default Availability=Available
// [4] int tabStopWidth() const

/*

*/
impl /*struct*/ QPlainTextEdit {
  pub fn tabStopWidth_0<RetType, T: QPlainTextEdit_tabStopWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.tabStopWidth_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_tabStopWidth_0<RetType> {
  fn tabStopWidth_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_tabStopWidth_0<i32> for () {
  fn tabStopWidth_0(self , rsthis: & QPlainTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QPlainTextEdit12tabStopWidthEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:175
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTabStopWidth(int)

/*

*/
impl /*struct*/ QPlainTextEdit {
  pub fn setTabStopWidth_0<RetType, T: QPlainTextEdit_setTabStopWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTabStopWidth_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_setTabStopWidth_0<RetType> {
  fn setTabStopWidth_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_setTabStopWidth_0<(/*void*/)> for (i32) {
  fn setTabStopWidth_0(self , rsthis: & QPlainTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN14QPlainTextEdit15setTabStopWidthEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:178
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal tabStopDistance() const

/*

*/
impl /*struct*/ QPlainTextEdit {
  pub fn tabStopDistance_0<RetType, T: QPlainTextEdit_tabStopDistance_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.tabStopDistance_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_tabStopDistance_0<RetType> {
  fn tabStopDistance_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_tabStopDistance_0<f64> for () {
  fn tabStopDistance_0(self , rsthis: & QPlainTextEdit) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QPlainTextEdit15tabStopDistanceEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:179
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTabStopDistance(qreal)

/*

*/
impl /*struct*/ QPlainTextEdit {
  pub fn setTabStopDistance_0<RetType, T: QPlainTextEdit_setTabStopDistance_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTabStopDistance_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_setTabStopDistance_0<RetType> {
  fn setTabStopDistance_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_setTabStopDistance_0<(/*void*/)> for (f64) {
  fn setTabStopDistance_0(self , rsthis: & QPlainTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN14QPlainTextEdit18setTabStopDistanceEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:181
// index:0
// Public Visibility=Default Availability=Available
// [4] int cursorWidth() const

/*

*/
impl /*struct*/ QPlainTextEdit {
  pub fn cursorWidth_0<RetType, T: QPlainTextEdit_cursorWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cursorWidth_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_cursorWidth_0<RetType> {
  fn cursorWidth_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_cursorWidth_0<i32> for () {
  fn cursorWidth_0(self , rsthis: & QPlainTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QPlainTextEdit11cursorWidthEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:182
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCursorWidth(int)

/*

*/
impl /*struct*/ QPlainTextEdit {
  pub fn setCursorWidth_0<RetType, T: QPlainTextEdit_setCursorWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCursorWidth_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_setCursorWidth_0<RetType> {
  fn setCursorWidth_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_setCursorWidth_0<(/*void*/)> for (i32) {
  fn setCursorWidth_0(self , rsthis: & QPlainTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN14QPlainTextEdit14setCursorWidthEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:187
// index:0
// Public Visibility=Default Availability=Available
// [-2] void moveCursor(QTextCursor::MoveOperation, QTextCursor::MoveMode)

/*
Moves the cursor by performing the given operation.

If mode is QTextCursor::KeepAnchor, the cursor selects the text it moves over. This is the same effect that the user achieves when they hold down the Shift key and move the cursor with the cursor keys.

See also QTextCursor::movePosition().
*/
impl /*struct*/ QPlainTextEdit {
  pub fn moveCursor_0<RetType, T: QPlainTextEdit_moveCursor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.moveCursor_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_moveCursor_0<RetType> {
  fn moveCursor_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_moveCursor_0<(/*void*/)> for (i32,i32) {
  fn moveCursor_0(self , rsthis: & QPlainTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN14QPlainTextEdit10moveCursorEN11QTextCursor13MoveOperationENS0_8MoveModeE", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:189
// index:0
// Public Visibility=Default Availability=Available
// [1] bool canPaste() const

/*
Returns whether text can be pasted from the clipboard into the textedit.
*/
impl /*struct*/ QPlainTextEdit {
  pub fn canPaste_0<RetType, T: QPlainTextEdit_canPaste_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.canPaste_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_canPaste_0<RetType> {
  fn canPaste_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_canPaste_0<bool> for () {
  fn canPaste_0(self , rsthis: & QPlainTextEdit) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QPlainTextEdit8canPasteEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:191
// index:0
// Public Visibility=Default Availability=Available
// [-2] void print(QPagedPaintDevice *) const

/*
Convenience function to print the text edit's document to the given printer. This is equivalent to calling the print method on the document directly except that this function also supports QPrinter::Selection as print range.

See also QTextDocument::print().
*/
impl /*struct*/ QPlainTextEdit {
  pub fn print_0<RetType, T: QPlainTextEdit_print_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.print_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_print_0<RetType> {
  fn print_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_print_0<(/*void*/)> for (usize) {
  fn print_0(self , rsthis: & QPlainTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK14QPlainTextEdit5printEP17QPagedPaintDevice", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:193
// index:0
// Public Visibility=Default Availability=Available
// [4] int blockCount() const

/*

*/
impl /*struct*/ QPlainTextEdit {
  pub fn blockCount_0<RetType, T: QPlainTextEdit_blockCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.blockCount_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_blockCount_0<RetType> {
  fn blockCount_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_blockCount_0<i32> for () {
  fn blockCount_0(self , rsthis: & QPlainTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QPlainTextEdit10blockCountEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:194
// index:0
// Public virtual Visibility=Default Availability=Available
// [16] QVariant inputMethodQuery(Qt::InputMethodQuery) const

/*
Reimplemented from QWidget::inputMethodQuery().
*/
impl /*struct*/ QPlainTextEdit {
  pub fn inputMethodQuery_0<RetType, T: QPlainTextEdit_inputMethodQuery_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.inputMethodQuery_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_inputMethodQuery_0<RetType> {
  fn inputMethodQuery_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_inputMethodQuery_0<usize> for (i32) {
  fn inputMethodQuery_0(self , rsthis: & QPlainTextEdit) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QPlainTextEdit16inputMethodQueryEN2Qt16InputMethodQueryE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:195
// index:1
// Public Visibility=Default Availability=Available
// [16] QVariant inputMethodQuery(Qt::InputMethodQuery, QVariant) const

/*
Reimplemented from QWidget::inputMethodQuery().
*/
impl /*struct*/ QPlainTextEdit {
  pub fn inputMethodQuery_1<RetType, T: QPlainTextEdit_inputMethodQuery_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.inputMethodQuery_1(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_inputMethodQuery_1<RetType> {
  fn inputMethodQuery_1(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_inputMethodQuery_1<usize> for (i32,usize) {
  fn inputMethodQuery_1(self , rsthis: & QPlainTextEdit) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QPlainTextEdit16inputMethodQueryEN2Qt16InputMethodQueryE8QVariant", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:199
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setPlainText(const QString &)

/*
Changes the text of the text edit to the string text. Any previous text is removed.

text is interpreted as plain text.

Note that the undo/redo history is cleared by this function.

Note: Setter function for property plainText. 

See also toPlainText().
*/
impl /*struct*/ QPlainTextEdit {
  pub fn setPlainText_0<RetType, T: QPlainTextEdit_setPlainText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPlainText_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_setPlainText_0<RetType> {
  fn setPlainText_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_setPlainText_0<(/*void*/)> for (usize) {
  fn setPlainText_0(self , rsthis: & QPlainTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN14QPlainTextEdit12setPlainTextERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:202
// index:0
// Public Visibility=Default Availability=Available
// [-2] void cut()

/*
Copies the selected text to the clipboard and deletes it from the text edit.

If there is no selected text nothing happens.

See also copy() and paste().
*/
impl /*struct*/ QPlainTextEdit {
  pub fn cut_0<RetType, T: QPlainTextEdit_cut_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cut_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_cut_0<RetType> {
  fn cut_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_cut_0<(/*void*/)> for () {
  fn cut_0(self , rsthis: & QPlainTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN14QPlainTextEdit3cutEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:203
// index:0
// Public Visibility=Default Availability=Available
// [-2] void copy()

/*
Copies any selected text to the clipboard.

See also copyAvailable().
*/
impl /*struct*/ QPlainTextEdit {
  pub fn copy_0<RetType, T: QPlainTextEdit_copy_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.copy_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_copy_0<RetType> {
  fn copy_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_copy_0<(/*void*/)> for () {
  fn copy_0(self , rsthis: & QPlainTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN14QPlainTextEdit4copyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:204
// index:0
// Public Visibility=Default Availability=Available
// [-2] void paste()

/*
Pastes the text from the clipboard into the text edit at the current cursor position.

If there is no text in the clipboard nothing happens.

To change the behavior of this function, i.e. to modify what QPlainTextEdit can paste and how it is being pasted, reimplement the virtual canInsertFromMimeData() and insertFromMimeData() functions.

See also cut() and copy().
*/
impl /*struct*/ QPlainTextEdit {
  pub fn paste_0<RetType, T: QPlainTextEdit_paste_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.paste_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_paste_0<RetType> {
  fn paste_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_paste_0<(/*void*/)> for () {
  fn paste_0(self , rsthis: & QPlainTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN14QPlainTextEdit5pasteEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:207
// index:0
// Public Visibility=Default Availability=Available
// [-2] void undo()

/*
Undoes the last operation.

If there is no operation to undo, i.e. there is no undo step in the undo/redo history, nothing happens.

See also redo().
*/
impl /*struct*/ QPlainTextEdit {
  pub fn undo_0<RetType, T: QPlainTextEdit_undo_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.undo_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_undo_0<RetType> {
  fn undo_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_undo_0<(/*void*/)> for () {
  fn undo_0(self , rsthis: & QPlainTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN14QPlainTextEdit4undoEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:208
// index:0
// Public Visibility=Default Availability=Available
// [-2] void redo()

/*
Redoes the last operation.

If there is no operation to redo, i.e. there is no redo step in the undo/redo history, nothing happens.

See also undo().
*/
impl /*struct*/ QPlainTextEdit {
  pub fn redo_0<RetType, T: QPlainTextEdit_redo_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.redo_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_redo_0<RetType> {
  fn redo_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_redo_0<(/*void*/)> for () {
  fn redo_0(self , rsthis: & QPlainTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN14QPlainTextEdit4redoEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:210
// index:0
// Public Visibility=Default Availability=Available
// [-2] void clear()

/*
Deletes all the text in the text edit.

Note that the undo/redo history is cleared by this function.

See also cut() and setPlainText().
*/
impl /*struct*/ QPlainTextEdit {
  pub fn clear_0<RetType, T: QPlainTextEdit_clear_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.clear_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_clear_0<RetType> {
  fn clear_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_clear_0<(/*void*/)> for () {
  fn clear_0(self , rsthis: & QPlainTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN14QPlainTextEdit5clearEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:211
// index:0
// Public Visibility=Default Availability=Available
// [-2] void selectAll()

/*
Selects all text.

See also copy(), cut(), and textCursor().
*/
impl /*struct*/ QPlainTextEdit {
  pub fn selectAll_0<RetType, T: QPlainTextEdit_selectAll_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.selectAll_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_selectAll_0<RetType> {
  fn selectAll_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_selectAll_0<(/*void*/)> for () {
  fn selectAll_0(self , rsthis: & QPlainTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN14QPlainTextEdit9selectAllEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:213
// index:0
// Public Visibility=Default Availability=Available
// [-2] void insertPlainText(const QString &)

/*
Convenience slot that inserts text at the current cursor position.

It is equivalent to


  edit->textCursor().insertText(text);
*/
impl /*struct*/ QPlainTextEdit {
  pub fn insertPlainText_0<RetType, T: QPlainTextEdit_insertPlainText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.insertPlainText_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_insertPlainText_0<RetType> {
  fn insertPlainText_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_insertPlainText_0<(/*void*/)> for (usize) {
  fn insertPlainText_0(self , rsthis: & QPlainTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN14QPlainTextEdit15insertPlainTextERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:215
// index:0
// Public Visibility=Default Availability=Available
// [-2] void appendPlainText(const QString &)

/*
Appends a new paragraph with text to the end of the text edit.

See also appendHtml().
*/
impl /*struct*/ QPlainTextEdit {
  pub fn appendPlainText_0<RetType, T: QPlainTextEdit_appendPlainText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.appendPlainText_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_appendPlainText_0<RetType> {
  fn appendPlainText_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_appendPlainText_0<(/*void*/)> for (usize) {
  fn appendPlainText_0(self , rsthis: & QPlainTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN14QPlainTextEdit15appendPlainTextERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:216
// index:0
// Public Visibility=Default Availability=Available
// [-2] void appendHtml(const QString &)

/*
Appends a new paragraph with html to the end of the text edit.

appendPlainText()
*/
impl /*struct*/ QPlainTextEdit {
  pub fn appendHtml_0<RetType, T: QPlainTextEdit_appendHtml_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.appendHtml_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_appendHtml_0<RetType> {
  fn appendHtml_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_appendHtml_0<(/*void*/)> for (usize) {
  fn appendHtml_0(self , rsthis: & QPlainTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN14QPlainTextEdit10appendHtmlERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:218
// index:0
// Public Visibility=Default Availability=Available
// [-2] void centerCursor()

/*
Scrolls the document in order to center the cursor vertically.

See also ensureCursorVisible() and centerOnScroll.
*/
impl /*struct*/ QPlainTextEdit {
  pub fn centerCursor_0<RetType, T: QPlainTextEdit_centerCursor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.centerCursor_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_centerCursor_0<RetType> {
  fn centerCursor_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_centerCursor_0<(/*void*/)> for () {
  fn centerCursor_0(self , rsthis: & QPlainTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN14QPlainTextEdit12centerCursorEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:220
// index:0
// Public Visibility=Default Availability=Available
// [-2] void zoomIn(int)

/*
Zooms in on the text by making the base font size range points larger and recalculating all font sizes to be the new size. This does not change the size of any images.

See also zoomOut().
*/
impl /*struct*/ QPlainTextEdit {
  pub fn zoomIn_0<RetType, T: QPlainTextEdit_zoomIn_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.zoomIn_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_zoomIn_0<RetType> {
  fn zoomIn_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_zoomIn_0<(/*void*/)> for (i32) {
  fn zoomIn_0(self , rsthis: & QPlainTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN14QPlainTextEdit6zoomInEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:221
// index:0
// Public Visibility=Default Availability=Available
// [-2] void zoomOut(int)

/*
This is an overloaded function.

Zooms out on the text by making the base font size range points smaller and recalculating all font sizes to be the new size. This does not change the size of any images.

See also zoomIn().
*/
impl /*struct*/ QPlainTextEdit {
  pub fn zoomOut_0<RetType, T: QPlainTextEdit_zoomOut_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.zoomOut_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_zoomOut_0<RetType> {
  fn zoomOut_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_zoomOut_0<(/*void*/)> for (i32) {
  fn zoomOut_0(self , rsthis: & QPlainTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN14QPlainTextEdit7zoomOutEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:224
// index:0
// Public Visibility=Default Availability=Available
// [-2] void textChanged()

/*
This signal is emitted whenever the document's content changes; for example, when text is inserted or deleted, or when formatting is applied.

Note: Notifier signal for property plainText.
*/
impl /*struct*/ QPlainTextEdit {
  pub fn textChanged_0<RetType, T: QPlainTextEdit_textChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.textChanged_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_textChanged_0<RetType> {
  fn textChanged_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_textChanged_0<(/*void*/)> for () {
  fn textChanged_0(self , rsthis: & QPlainTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN14QPlainTextEdit11textChangedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:225
// index:0
// Public Visibility=Default Availability=Available
// [-2] void undoAvailable(bool)

/*
This signal is emitted whenever undo operations become available (available is true) or unavailable (available is false).
*/
impl /*struct*/ QPlainTextEdit {
  pub fn undoAvailable_0<RetType, T: QPlainTextEdit_undoAvailable_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.undoAvailable_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_undoAvailable_0<RetType> {
  fn undoAvailable_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_undoAvailable_0<(/*void*/)> for (bool) {
  fn undoAvailable_0(self , rsthis: & QPlainTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN14QPlainTextEdit13undoAvailableEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:226
// index:0
// Public Visibility=Default Availability=Available
// [-2] void redoAvailable(bool)

/*
This signal is emitted whenever redo operations become available (available is true) or unavailable (available is false).
*/
impl /*struct*/ QPlainTextEdit {
  pub fn redoAvailable_0<RetType, T: QPlainTextEdit_redoAvailable_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.redoAvailable_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_redoAvailable_0<RetType> {
  fn redoAvailable_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_redoAvailable_0<(/*void*/)> for (bool) {
  fn redoAvailable_0(self , rsthis: & QPlainTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN14QPlainTextEdit13redoAvailableEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:227
// index:0
// Public Visibility=Default Availability=Available
// [-2] void copyAvailable(bool)

/*
This signal is emitted when text is selected or de-selected in the text edit.

When text is selected this signal will be emitted with yes set to true. If no text has been selected or if the selected text is de-selected this signal is emitted with yes set to false.

If yes is true then copy() can be used to copy the selection to the clipboard. If yes is false then copy() does nothing.

See also selectionChanged().
*/
impl /*struct*/ QPlainTextEdit {
  pub fn copyAvailable_0<RetType, T: QPlainTextEdit_copyAvailable_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.copyAvailable_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_copyAvailable_0<RetType> {
  fn copyAvailable_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_copyAvailable_0<(/*void*/)> for (bool) {
  fn copyAvailable_0(self , rsthis: & QPlainTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN14QPlainTextEdit13copyAvailableEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:228
// index:0
// Public Visibility=Default Availability=Available
// [-2] void selectionChanged()

/*
This signal is emitted whenever the selection changes.

See also copyAvailable().
*/
impl /*struct*/ QPlainTextEdit {
  pub fn selectionChanged_0<RetType, T: QPlainTextEdit_selectionChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.selectionChanged_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_selectionChanged_0<RetType> {
  fn selectionChanged_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_selectionChanged_0<(/*void*/)> for () {
  fn selectionChanged_0(self , rsthis: & QPlainTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN14QPlainTextEdit16selectionChangedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:229
// index:0
// Public Visibility=Default Availability=Available
// [-2] void cursorPositionChanged()

/*
This signal is emitted whenever the position of the cursor changed.
*/
impl /*struct*/ QPlainTextEdit {
  pub fn cursorPositionChanged_0<RetType, T: QPlainTextEdit_cursorPositionChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cursorPositionChanged_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_cursorPositionChanged_0<RetType> {
  fn cursorPositionChanged_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_cursorPositionChanged_0<(/*void*/)> for () {
  fn cursorPositionChanged_0(self , rsthis: & QPlainTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN14QPlainTextEdit21cursorPositionChangedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:231
// index:0
// Public Visibility=Default Availability=Available
// [-2] void updateRequest(const QRect &, int)

/*
This signal is emitted when the text document needs an update of the specified rect. If the text is scrolled, rect will cover the entire viewport area. If the text is scrolled vertically, dy carries the amount of pixels the viewport was scrolled.

The purpose of the signal is to support extra widgets in plain text edit subclasses that e.g. show line numbers, breakpoints, or other extra information.
*/
impl /*struct*/ QPlainTextEdit {
  pub fn updateRequest_0<RetType, T: QPlainTextEdit_updateRequest_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.updateRequest_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_updateRequest_0<RetType> {
  fn updateRequest_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_updateRequest_0<(/*void*/)> for (usize,i32) {
  fn updateRequest_0(self , rsthis: & QPlainTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN14QPlainTextEdit13updateRequestERK5QRecti", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:232
// index:0
// Public Visibility=Default Availability=Available
// [-2] void blockCountChanged(int)

/*
This signal is emitted whenever the block count changes. The new block count is passed in newBlockCount.
*/
impl /*struct*/ QPlainTextEdit {
  pub fn blockCountChanged_0<RetType, T: QPlainTextEdit_blockCountChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.blockCountChanged_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_blockCountChanged_0<RetType> {
  fn blockCountChanged_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_blockCountChanged_0<(/*void*/)> for (i32) {
  fn blockCountChanged_0(self , rsthis: & QPlainTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN14QPlainTextEdit17blockCountChangedEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:233
// index:0
// Public Visibility=Default Availability=Available
// [-2] void modificationChanged(bool)

/*
This signal is emitted whenever the content of the document changes in a way that affects the modification state. If changed is true, the document has been modified; otherwise it is false.

For example, calling setModified(false) on a document and then inserting text causes the signal to get emitted. If you undo that operation, causing the document to return to its original unmodified state, the signal will get emitted again.
*/
impl /*struct*/ QPlainTextEdit {
  pub fn modificationChanged_0<RetType, T: QPlainTextEdit_modificationChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.modificationChanged_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_modificationChanged_0<RetType> {
  fn modificationChanged_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_modificationChanged_0<(/*void*/)> for (bool) {
  fn modificationChanged_0(self , rsthis: & QPlainTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN14QPlainTextEdit19modificationChangedEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:236
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool event(QEvent *)

/*

*/
impl /*struct*/ QPlainTextEdit {
  pub fn event_0<RetType, T: QPlainTextEdit_event_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.event_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_event_0<RetType> {
  fn event_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_event_0<bool> for (usize) {
  fn event_0(self , rsthis: & QPlainTextEdit) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN14QPlainTextEdit5eventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:237
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void timerEvent(QTimerEvent *)

/*

*/
impl /*struct*/ QPlainTextEdit {
  pub fn timerEvent_0<RetType, T: QPlainTextEdit_timerEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.timerEvent_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_timerEvent_0<RetType> {
  fn timerEvent_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_timerEvent_0<(/*void*/)> for (usize) {
  fn timerEvent_0(self , rsthis: & QPlainTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN14QPlainTextEdit10timerEventEP11QTimerEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:238
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void keyPressEvent(QKeyEvent *)

/*
Reimplemented from QWidget::keyPressEvent().
*/
impl /*struct*/ QPlainTextEdit {
  pub fn keyPressEvent_0<RetType, T: QPlainTextEdit_keyPressEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.keyPressEvent_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_keyPressEvent_0<RetType> {
  fn keyPressEvent_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_keyPressEvent_0<(/*void*/)> for (usize) {
  fn keyPressEvent_0(self , rsthis: & QPlainTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN14QPlainTextEdit13keyPressEventEP9QKeyEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:239
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void keyReleaseEvent(QKeyEvent *)

/*
Reimplemented from QWidget::keyReleaseEvent().
*/
impl /*struct*/ QPlainTextEdit {
  pub fn keyReleaseEvent_0<RetType, T: QPlainTextEdit_keyReleaseEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.keyReleaseEvent_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_keyReleaseEvent_0<RetType> {
  fn keyReleaseEvent_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_keyReleaseEvent_0<(/*void*/)> for (usize) {
  fn keyReleaseEvent_0(self , rsthis: & QPlainTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN14QPlainTextEdit15keyReleaseEventEP9QKeyEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:240
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void resizeEvent(QResizeEvent *)

/*
Reimplemented from QWidget::resizeEvent().
*/
impl /*struct*/ QPlainTextEdit {
  pub fn resizeEvent_0<RetType, T: QPlainTextEdit_resizeEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resizeEvent_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_resizeEvent_0<RetType> {
  fn resizeEvent_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_resizeEvent_0<(/*void*/)> for (usize) {
  fn resizeEvent_0(self , rsthis: & QPlainTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN14QPlainTextEdit11resizeEventEP12QResizeEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:241
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void paintEvent(QPaintEvent *)

/*
Reimplemented from QWidget::paintEvent().
*/
impl /*struct*/ QPlainTextEdit {
  pub fn paintEvent_0<RetType, T: QPlainTextEdit_paintEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.paintEvent_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_paintEvent_0<RetType> {
  fn paintEvent_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_paintEvent_0<(/*void*/)> for (usize) {
  fn paintEvent_0(self , rsthis: & QPlainTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN14QPlainTextEdit10paintEventEP11QPaintEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:242
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mousePressEvent(QMouseEvent *)

/*
Reimplemented from QWidget::mousePressEvent().
*/
impl /*struct*/ QPlainTextEdit {
  pub fn mousePressEvent_0<RetType, T: QPlainTextEdit_mousePressEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mousePressEvent_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_mousePressEvent_0<RetType> {
  fn mousePressEvent_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_mousePressEvent_0<(/*void*/)> for (usize) {
  fn mousePressEvent_0(self , rsthis: & QPlainTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN14QPlainTextEdit15mousePressEventEP11QMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:243
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mouseMoveEvent(QMouseEvent *)

/*
Reimplemented from QWidget::mouseMoveEvent().
*/
impl /*struct*/ QPlainTextEdit {
  pub fn mouseMoveEvent_0<RetType, T: QPlainTextEdit_mouseMoveEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mouseMoveEvent_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_mouseMoveEvent_0<RetType> {
  fn mouseMoveEvent_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_mouseMoveEvent_0<(/*void*/)> for (usize) {
  fn mouseMoveEvent_0(self , rsthis: & QPlainTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN14QPlainTextEdit14mouseMoveEventEP11QMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:244
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mouseReleaseEvent(QMouseEvent *)

/*
Reimplemented from QWidget::mouseReleaseEvent().
*/
impl /*struct*/ QPlainTextEdit {
  pub fn mouseReleaseEvent_0<RetType, T: QPlainTextEdit_mouseReleaseEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mouseReleaseEvent_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_mouseReleaseEvent_0<RetType> {
  fn mouseReleaseEvent_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_mouseReleaseEvent_0<(/*void*/)> for (usize) {
  fn mouseReleaseEvent_0(self , rsthis: & QPlainTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN14QPlainTextEdit17mouseReleaseEventEP11QMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:245
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mouseDoubleClickEvent(QMouseEvent *)

/*
Reimplemented from QWidget::mouseDoubleClickEvent().
*/
impl /*struct*/ QPlainTextEdit {
  pub fn mouseDoubleClickEvent_0<RetType, T: QPlainTextEdit_mouseDoubleClickEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mouseDoubleClickEvent_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_mouseDoubleClickEvent_0<RetType> {
  fn mouseDoubleClickEvent_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_mouseDoubleClickEvent_0<(/*void*/)> for (usize) {
  fn mouseDoubleClickEvent_0(self , rsthis: & QPlainTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN14QPlainTextEdit21mouseDoubleClickEventEP11QMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:246
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool focusNextPrevChild(bool)

/*
Reimplemented from QWidget::focusNextPrevChild().
*/
impl /*struct*/ QPlainTextEdit {
  pub fn focusNextPrevChild_0<RetType, T: QPlainTextEdit_focusNextPrevChild_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.focusNextPrevChild_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_focusNextPrevChild_0<RetType> {
  fn focusNextPrevChild_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_focusNextPrevChild_0<bool> for (bool) {
  fn focusNextPrevChild_0(self , rsthis: & QPlainTextEdit) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN14QPlainTextEdit18focusNextPrevChildEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:248
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void contextMenuEvent(QContextMenuEvent *)

/*
Reimplemented from QWidget::contextMenuEvent().

Shows the standard context menu created with createStandardContextMenu().

If you do not want the text edit to have a context menu, you can set its contextMenuPolicy to Qt::NoContextMenu. If you want to customize the context menu, reimplement this function. If you want to extend the standard context menu, reimplement this function, call createStandardContextMenu() and extend the menu returned.

Information about the event is passed in the event object.


  void MyQPlainTextEdit::contextMenuEvent(QContextMenuEvent *event)
  {
      QMenu *menu = createStandardContextMenu();
      menu->addAction(tr("My Menu Item"));
      //...
      menu->exec(event->globalPos());
      delete menu;
  }
*/
impl /*struct*/ QPlainTextEdit {
  pub fn contextMenuEvent_0<RetType, T: QPlainTextEdit_contextMenuEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.contextMenuEvent_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_contextMenuEvent_0<RetType> {
  fn contextMenuEvent_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_contextMenuEvent_0<(/*void*/)> for (usize) {
  fn contextMenuEvent_0(self , rsthis: & QPlainTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN14QPlainTextEdit16contextMenuEventEP17QContextMenuEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:251
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void dragEnterEvent(QDragEnterEvent *)

/*
Reimplemented from QWidget::dragEnterEvent().
*/
impl /*struct*/ QPlainTextEdit {
  pub fn dragEnterEvent_0<RetType, T: QPlainTextEdit_dragEnterEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dragEnterEvent_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_dragEnterEvent_0<RetType> {
  fn dragEnterEvent_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_dragEnterEvent_0<(/*void*/)> for (usize) {
  fn dragEnterEvent_0(self , rsthis: & QPlainTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN14QPlainTextEdit14dragEnterEventEP15QDragEnterEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:252
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void dragLeaveEvent(QDragLeaveEvent *)

/*
Reimplemented from QWidget::dragLeaveEvent().
*/
impl /*struct*/ QPlainTextEdit {
  pub fn dragLeaveEvent_0<RetType, T: QPlainTextEdit_dragLeaveEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dragLeaveEvent_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_dragLeaveEvent_0<RetType> {
  fn dragLeaveEvent_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_dragLeaveEvent_0<(/*void*/)> for (usize) {
  fn dragLeaveEvent_0(self , rsthis: & QPlainTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN14QPlainTextEdit14dragLeaveEventEP15QDragLeaveEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:253
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void dragMoveEvent(QDragMoveEvent *)

/*
Reimplemented from QWidget::dragMoveEvent().
*/
impl /*struct*/ QPlainTextEdit {
  pub fn dragMoveEvent_0<RetType, T: QPlainTextEdit_dragMoveEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dragMoveEvent_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_dragMoveEvent_0<RetType> {
  fn dragMoveEvent_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_dragMoveEvent_0<(/*void*/)> for (usize) {
  fn dragMoveEvent_0(self , rsthis: & QPlainTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN14QPlainTextEdit13dragMoveEventEP14QDragMoveEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:254
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void dropEvent(QDropEvent *)

/*
Reimplemented from QWidget::dropEvent().
*/
impl /*struct*/ QPlainTextEdit {
  pub fn dropEvent_0<RetType, T: QPlainTextEdit_dropEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dropEvent_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_dropEvent_0<RetType> {
  fn dropEvent_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_dropEvent_0<(/*void*/)> for (usize) {
  fn dropEvent_0(self , rsthis: & QPlainTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN14QPlainTextEdit9dropEventEP10QDropEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:256
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void focusInEvent(QFocusEvent *)

/*
Reimplemented from QWidget::focusInEvent().
*/
impl /*struct*/ QPlainTextEdit {
  pub fn focusInEvent_0<RetType, T: QPlainTextEdit_focusInEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.focusInEvent_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_focusInEvent_0<RetType> {
  fn focusInEvent_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_focusInEvent_0<(/*void*/)> for (usize) {
  fn focusInEvent_0(self , rsthis: & QPlainTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN14QPlainTextEdit12focusInEventEP11QFocusEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:257
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void focusOutEvent(QFocusEvent *)

/*
Reimplemented from QWidget::focusOutEvent().
*/
impl /*struct*/ QPlainTextEdit {
  pub fn focusOutEvent_0<RetType, T: QPlainTextEdit_focusOutEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.focusOutEvent_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_focusOutEvent_0<RetType> {
  fn focusOutEvent_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_focusOutEvent_0<(/*void*/)> for (usize) {
  fn focusOutEvent_0(self , rsthis: & QPlainTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN14QPlainTextEdit13focusOutEventEP11QFocusEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:258
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void showEvent(QShowEvent *)

/*
Reimplemented from QWidget::showEvent().
*/
impl /*struct*/ QPlainTextEdit {
  pub fn showEvent_0<RetType, T: QPlainTextEdit_showEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.showEvent_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_showEvent_0<RetType> {
  fn showEvent_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_showEvent_0<(/*void*/)> for (usize) {
  fn showEvent_0(self , rsthis: & QPlainTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN14QPlainTextEdit9showEventEP10QShowEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:259
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void changeEvent(QEvent *)

/*
Reimplemented from QWidget::changeEvent().
*/
impl /*struct*/ QPlainTextEdit {
  pub fn changeEvent_0<RetType, T: QPlainTextEdit_changeEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.changeEvent_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_changeEvent_0<RetType> {
  fn changeEvent_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_changeEvent_0<(/*void*/)> for (usize) {
  fn changeEvent_0(self , rsthis: & QPlainTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN14QPlainTextEdit11changeEventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:261
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void wheelEvent(QWheelEvent *)

/*
Reimplemented from QWidget::wheelEvent().
*/
impl /*struct*/ QPlainTextEdit {
  pub fn wheelEvent_0<RetType, T: QPlainTextEdit_wheelEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.wheelEvent_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_wheelEvent_0<RetType> {
  fn wheelEvent_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_wheelEvent_0<(/*void*/)> for (usize) {
  fn wheelEvent_0(self , rsthis: & QPlainTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN14QPlainTextEdit10wheelEventEP11QWheelEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:264
// index:0
// Protected virtual Visibility=Default Availability=Available
// [8] QMimeData * createMimeDataFromSelection() const

/*
This function returns a new MIME data object to represent the contents of the text edit's current selection. It is called when the selection needs to be encapsulated into a new QMimeData object; for example, when a drag and drop operation is started, or when data is copied to the clipboard.

If you reimplement this function, note that the ownership of the returned QMimeData object is passed to the caller. The selection can be retrieved by using the textCursor() function.
*/
impl /*struct*/ QPlainTextEdit {
  pub fn createMimeDataFromSelection_0<RetType, T: QPlainTextEdit_createMimeDataFromSelection_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.createMimeDataFromSelection_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_createMimeDataFromSelection_0<RetType> {
  fn createMimeDataFromSelection_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_createMimeDataFromSelection_0<usize> for () {
  fn createMimeDataFromSelection_0(self , rsthis: & QPlainTextEdit) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QPlainTextEdit27createMimeDataFromSelectionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:265
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool canInsertFromMimeData(const QMimeData *) const

/*
This function returns true if the contents of the MIME data object, specified by source, can be decoded and inserted into the document. It is called for example when during a drag operation the mouse enters this widget and it is necessary to determine whether it is possible to accept the drag.
*/
impl /*struct*/ QPlainTextEdit {
  pub fn canInsertFromMimeData_0<RetType, T: QPlainTextEdit_canInsertFromMimeData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.canInsertFromMimeData_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_canInsertFromMimeData_0<RetType> {
  fn canInsertFromMimeData_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_canInsertFromMimeData_0<bool> for (usize) {
  fn canInsertFromMimeData_0(self , rsthis: & QPlainTextEdit) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QPlainTextEdit21canInsertFromMimeDataEPK9QMimeData", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:266
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void insertFromMimeData(const QMimeData *)

/*
This function inserts the contents of the MIME data object, specified by source, into the text edit at the current cursor position. It is called whenever text is inserted as the result of a clipboard paste operation, or when the text edit accepts data from a drag and drop operation.
*/
impl /*struct*/ QPlainTextEdit {
  pub fn insertFromMimeData_0<RetType, T: QPlainTextEdit_insertFromMimeData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.insertFromMimeData_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_insertFromMimeData_0<RetType> {
  fn insertFromMimeData_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_insertFromMimeData_0<(/*void*/)> for (usize) {
  fn insertFromMimeData_0(self , rsthis: & QPlainTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN14QPlainTextEdit18insertFromMimeDataEPK9QMimeData", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:268
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void inputMethodEvent(QInputMethodEvent *)

/*
Reimplemented from QWidget::inputMethodEvent().
*/
impl /*struct*/ QPlainTextEdit {
  pub fn inputMethodEvent_0<RetType, T: QPlainTextEdit_inputMethodEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.inputMethodEvent_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_inputMethodEvent_0<RetType> {
  fn inputMethodEvent_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_inputMethodEvent_0<(/*void*/)> for (usize) {
  fn inputMethodEvent_0(self , rsthis: & QPlainTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN14QPlainTextEdit16inputMethodEventEP17QInputMethodEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:272
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void scrollContentsBy(int, int)

/*
Reimplemented from QAbstractScrollArea::scrollContentsBy().
*/
impl /*struct*/ QPlainTextEdit {
  pub fn scrollContentsBy_0<RetType, T: QPlainTextEdit_scrollContentsBy_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.scrollContentsBy_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_scrollContentsBy_0<RetType> {
  fn scrollContentsBy_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_scrollContentsBy_0<(/*void*/)> for (i32,i32) {
  fn scrollContentsBy_0(self , rsthis: & QPlainTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN14QPlainTextEdit16scrollContentsByEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:273
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void doSetTextCursor(const QTextCursor &)

/*

*/
impl /*struct*/ QPlainTextEdit {
  pub fn doSetTextCursor_0<RetType, T: QPlainTextEdit_doSetTextCursor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.doSetTextCursor_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_doSetTextCursor_0<RetType> {
  fn doSetTextCursor_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_doSetTextCursor_0<(/*void*/)> for (usize) {
  fn doSetTextCursor_0(self , rsthis: & QPlainTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN14QPlainTextEdit15doSetTextCursorERK11QTextCursor", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:275
// index:0
// Protected Visibility=Default Availability=Available
// [16] QTextBlock firstVisibleBlock() const

/*
Returns the first visible block.

See also blockBoundingRect().
*/
impl /*struct*/ QPlainTextEdit {
  pub fn firstVisibleBlock_0<RetType, T: QPlainTextEdit_firstVisibleBlock_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.firstVisibleBlock_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_firstVisibleBlock_0<RetType> {
  fn firstVisibleBlock_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_firstVisibleBlock_0<usize> for () {
  fn firstVisibleBlock_0(self , rsthis: & QPlainTextEdit) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QPlainTextEdit17firstVisibleBlockEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:276
// index:0
// Protected Visibility=Default Availability=Available
// [16] QPointF contentOffset() const

/*
Returns the content's origin in viewport coordinates.

The origin of the content of a plain text edit is always the top left corner of the first visible text block. The content offset is different from (0,0) when the text has been scrolled horizontally, or when the first visible block has been scrolled partially off the screen, i.e. the visible text does not start with the first line of the first visible block, or when the first visible block is the very first block and the editor displays a margin.

See also firstVisibleBlock(), horizontalScrollBar(), and verticalScrollBar().
*/
impl /*struct*/ QPlainTextEdit {
  pub fn contentOffset_0<RetType, T: QPlainTextEdit_contentOffset_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.contentOffset_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_contentOffset_0<RetType> {
  fn contentOffset_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_contentOffset_0<usize> for () {
  fn contentOffset_0(self , rsthis: & QPlainTextEdit) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QPlainTextEdit13contentOffsetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:277
// index:0
// Protected Visibility=Default Availability=Available
// [32] QRectF blockBoundingRect(const QTextBlock &) const

/*
Returns the bounding rectangle of the text block in the block's own coordinates.

See also blockBoundingGeometry().
*/
impl /*struct*/ QPlainTextEdit {
  pub fn blockBoundingRect_0<RetType, T: QPlainTextEdit_blockBoundingRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.blockBoundingRect_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_blockBoundingRect_0<RetType> {
  fn blockBoundingRect_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_blockBoundingRect_0<usize> for (usize) {
  fn blockBoundingRect_0(self , rsthis: & QPlainTextEdit) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QPlainTextEdit17blockBoundingRectERK10QTextBlock", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:278
// index:0
// Protected Visibility=Default Availability=Available
// [32] QRectF blockBoundingGeometry(const QTextBlock &) const

/*
Returns the bounding rectangle of the text block in content coordinates. Translate the rectangle with the contentOffset() to get visual coordinates on the viewport.

See also firstVisibleBlock() and blockBoundingRect().
*/
impl /*struct*/ QPlainTextEdit {
  pub fn blockBoundingGeometry_0<RetType, T: QPlainTextEdit_blockBoundingGeometry_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.blockBoundingGeometry_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_blockBoundingGeometry_0<RetType> {
  fn blockBoundingGeometry_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_blockBoundingGeometry_0<usize> for (usize) {
  fn blockBoundingGeometry_0(self , rsthis: & QPlainTextEdit) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QPlainTextEdit21blockBoundingGeometryERK10QTextBlock", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:279
// index:0
// Protected Visibility=Default Availability=Available
// [64] QAbstractTextDocumentLayout::PaintContext getPaintContext() const

/*
Returns the paint context for the viewport(), useful only when reimplementing paintEvent().
*/
impl /*struct*/ QPlainTextEdit {
  pub fn getPaintContext_0<RetType, T: QPlainTextEdit_getPaintContext_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.getPaintContext_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_getPaintContext_0<RetType> {
  fn getPaintContext_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_getPaintContext_0<i32> for () {
  fn getPaintContext_0(self , rsthis: & QPlainTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QPlainTextEdit15getPaintContextEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qplaintextedit.h:281
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void zoomInF(float)

/*

*/
impl /*struct*/ QPlainTextEdit {
  pub fn zoomInF_0<RetType, T: QPlainTextEdit_zoomInF_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.zoomInF_0(self);
    // return 1;
  }
}
pub trait QPlainTextEdit_zoomInF_0<RetType> {
  fn zoomInF_0(self , rsthis: & QPlainTextEdit) -> RetType;
}
impl<'a> /*trait*/ QPlainTextEdit_zoomInF_0<(/*void*/)> for (f32) {
  fn zoomInF_0(self , rsthis: & QPlainTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f32 as usize;
     qtrt::InvokeQtFunc6("_ZN14QPlainTextEdit7zoomInFEf", 1,qtrt::FFITY_FLOAT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}


/*
ConstantValue
QPlainTextEdit::NoWrap0
QPlainTextEdit::WidgetWidth1

*/
pub type QPlainTextEdit__LineWrapMode = i32;
// 
pub const QPlainTextEdit__NoWrap :QPlainTextEdit__LineWrapMode = 0;
// 
pub const QPlainTextEdit__WidgetWidth :QPlainTextEdit__LineWrapMode = 1;
pub fn QPlainTextEdit_LineWrapModeItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QPlainTextEdit", val);
}
pub fn QPlainTextEdit_LineWrapModeItemName_s(val: i32) ->String {
  //var nilthis *QPlainTextEdit
  //return nilthis.LineWrapModeItemName(val);
  return QPlainTextEdit_LineWrapModeItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
