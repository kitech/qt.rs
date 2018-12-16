

// mod ::widgets::QTextEdit
// package qtwidgets
// /usr/include/qt/QtWidgets/qtextedit.h
// #include <qtextedit.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 12
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
// func (this *QTextEdit) InheritEvent(f func(e *qtcore.QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "event", f)
// }

// void timerEvent(QTimerEvent *)
// func (this *QTextEdit) InheritTimerEvent(f func(e *qtcore.QTimerEvent/*777 QTimerEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "timerEvent", f)
// }

// void keyPressEvent(QKeyEvent *)
// func (this *QTextEdit) InheritKeyPressEvent(f func(e *qtgui.QKeyEvent/*777 QKeyEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "keyPressEvent", f)
// }

// void keyReleaseEvent(QKeyEvent *)
// func (this *QTextEdit) InheritKeyReleaseEvent(f func(e *qtgui.QKeyEvent/*777 QKeyEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "keyReleaseEvent", f)
// }

// void resizeEvent(QResizeEvent *)
// func (this *QTextEdit) InheritResizeEvent(f func(e *qtgui.QResizeEvent/*777 QResizeEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "resizeEvent", f)
// }

// void paintEvent(QPaintEvent *)
// func (this *QTextEdit) InheritPaintEvent(f func(e *qtgui.QPaintEvent/*777 QPaintEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "paintEvent", f)
// }

// void mousePressEvent(QMouseEvent *)
// func (this *QTextEdit) InheritMousePressEvent(f func(e *qtgui.QMouseEvent/*777 QMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mousePressEvent", f)
// }

// void mouseMoveEvent(QMouseEvent *)
// func (this *QTextEdit) InheritMouseMoveEvent(f func(e *qtgui.QMouseEvent/*777 QMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mouseMoveEvent", f)
// }

// void mouseReleaseEvent(QMouseEvent *)
// func (this *QTextEdit) InheritMouseReleaseEvent(f func(e *qtgui.QMouseEvent/*777 QMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mouseReleaseEvent", f)
// }

// void mouseDoubleClickEvent(QMouseEvent *)
// func (this *QTextEdit) InheritMouseDoubleClickEvent(f func(e *qtgui.QMouseEvent/*777 QMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mouseDoubleClickEvent", f)
// }

// bool focusNextPrevChild(bool)
// func (this *QTextEdit) InheritFocusNextPrevChild(f func(next bool) bool) {
//  qtrt.SetAllInheritCallback(this, "focusNextPrevChild", f)
// }

// void contextMenuEvent(QContextMenuEvent *)
// func (this *QTextEdit) InheritContextMenuEvent(f func(e *qtgui.QContextMenuEvent/*777 QContextMenuEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "contextMenuEvent", f)
// }

// void dragEnterEvent(QDragEnterEvent *)
// func (this *QTextEdit) InheritDragEnterEvent(f func(e *qtgui.QDragEnterEvent/*777 QDragEnterEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "dragEnterEvent", f)
// }

// void dragLeaveEvent(QDragLeaveEvent *)
// func (this *QTextEdit) InheritDragLeaveEvent(f func(e *qtgui.QDragLeaveEvent/*777 QDragLeaveEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "dragLeaveEvent", f)
// }

// void dragMoveEvent(QDragMoveEvent *)
// func (this *QTextEdit) InheritDragMoveEvent(f func(e *qtgui.QDragMoveEvent/*777 QDragMoveEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "dragMoveEvent", f)
// }

// void dropEvent(QDropEvent *)
// func (this *QTextEdit) InheritDropEvent(f func(e *qtgui.QDropEvent/*777 QDropEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "dropEvent", f)
// }

// void focusInEvent(QFocusEvent *)
// func (this *QTextEdit) InheritFocusInEvent(f func(e *qtgui.QFocusEvent/*777 QFocusEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "focusInEvent", f)
// }

// void focusOutEvent(QFocusEvent *)
// func (this *QTextEdit) InheritFocusOutEvent(f func(e *qtgui.QFocusEvent/*777 QFocusEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "focusOutEvent", f)
// }

// void showEvent(QShowEvent *)
// func (this *QTextEdit) InheritShowEvent(f func(arg0 *qtgui.QShowEvent/*777 QShowEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "showEvent", f)
// }

// void changeEvent(QEvent *)
// func (this *QTextEdit) InheritChangeEvent(f func(e *qtcore.QEvent/*777 QEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "changeEvent", f)
// }

// void wheelEvent(QWheelEvent *)
// func (this *QTextEdit) InheritWheelEvent(f func(e *qtgui.QWheelEvent/*777 QWheelEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "wheelEvent", f)
// }

// QMimeData * createMimeDataFromSelection()
// func (this *QTextEdit) InheritCreateMimeDataFromSelection(f func() unsafe.Pointer/*666*/) {
//  qtrt.SetAllInheritCallback(this, "createMimeDataFromSelection", f)
// }

// bool canInsertFromMimeData(const QMimeData *)
// func (this *QTextEdit) InheritCanInsertFromMimeData(f func(source *qtcore.QMimeData/*777 const QMimeData **/) bool) {
//  qtrt.SetAllInheritCallback(this, "canInsertFromMimeData", f)
// }

// void insertFromMimeData(const QMimeData *)
// func (this *QTextEdit) InheritInsertFromMimeData(f func(source *qtcore.QMimeData/*777 const QMimeData **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "insertFromMimeData", f)
// }

// void inputMethodEvent(QInputMethodEvent *)
// func (this *QTextEdit) InheritInputMethodEvent(f func(arg0 *qtgui.QInputMethodEvent/*777 QInputMethodEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "inputMethodEvent", f)
// }

// void scrollContentsBy(int, int)
// func (this *QTextEdit) InheritScrollContentsBy(f func(dx int, dy int) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "scrollContentsBy", f)
// }

// void doSetTextCursor(const QTextCursor &)
// func (this *QTextEdit) InheritDoSetTextCursor(f func(cursor *qtgui.QTextCursor) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "doSetTextCursor", f)
// }

// void zoomInF(float)
// func (this *QTextEdit) InheritZoomInF(f func(range_ float32) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "zoomInF", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QTextEdit)=48
pub struct QTextEdit {
  qbase: QAbstractScrollArea,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QTextEdit_ITF interface {
//    QAbstractScrollArea_ITF
//    QTextEdit_PTR() *QTextEdit
//}
//func (ptr *QTextEdit) QTextEdit_PTR() *QTextEdit { return ptr }

impl /*struct*/ QTextEdit {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QTextEdit {
    return QTextEdit{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QTextEdit {
//  type Target = QTextEditBASE;
//
//  fn deref(&self) -> &QTextEditBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QTextEditBASE> for QTextEdit {
//  fn as_ref(& self) -> & QTextEditBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qtextedit.h:63
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QTextEdit {
  pub fn metaObject_0<RetType, T: QTextEdit_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QTextEdit_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QTextEdit) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTextEdit10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:105
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QTextEdit(QWidget *)

/*
Constructs an empty QTextEdit with parent parent.
*/
// QTextEdit(QWidget *) ctx.fn_proto_cpp
impl /*struct*/ QTextEdit {
  pub fn QTextEdit_0<T: QTextEdit_QTextEdit_0>(value: T) -> QTextEdit {
    let rsthis = value.QTextEdit_0();
    return rsthis;
    // return 1;
  }
}

pub trait QTextEdit_QTextEdit_0 {
  fn QTextEdit_0(self) -> QTextEdit;
}
// QTextEdit(QWidget *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QTextEdit_QTextEdit_0 for (usize) {
  fn QTextEdit_0(self) -> QTextEdit {
    // unsafe{_ZN9QTextEditC2EP7QWidget()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QTextEditC2EP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTextEdit{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:106
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QTextEdit(const QString &, QWidget *)

/*
Constructs an empty QTextEdit with parent parent.
*/
// QTextEdit(const QString &, QWidget *) ctx.fn_proto_cpp
impl /*struct*/ QTextEdit {
  pub fn QTextEdit_1<T: QTextEdit_QTextEdit_1>(value: T) -> QTextEdit {
    let rsthis = value.QTextEdit_1();
    return rsthis;
    // return 1;
  }
}

pub trait QTextEdit_QTextEdit_1 {
  fn QTextEdit_1(self) -> QTextEdit;
}
// QTextEdit(const QString &, QWidget *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QTextEdit_QTextEdit_1 for (usize,usize) {
  fn QTextEdit_1(self) -> QTextEdit {
    // unsafe{_ZN9QTextEditC2ERK7QStringP7QWidget()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QTextEditC2ERK7QStringP7QWidget", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTextEdit{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:107
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QTextEdit()

/*

*/
pub fn DeleteQTextEdit(this :*mut QTextEdit) {
    // let rv = qtrt::InvokeQtFunc6("_ZN9QTextEditD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 48)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qtextedit.h:109
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDocument(QTextDocument *)

/*

*/
impl /*struct*/ QTextEdit {
  pub fn setDocument_0<RetType, T: QTextEdit_setDocument_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDocument_0(self);
    // return 1;
  }
}
pub trait QTextEdit_setDocument_0<RetType> {
  fn setDocument_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_setDocument_0<(/*void*/)> for (usize) {
  fn setDocument_0(self , rsthis: & QTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QTextEdit11setDocumentEP13QTextDocument", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:110
// index:0
// Public Visibility=Default Availability=Available
// [8] QTextDocument * document() const

/*

*/
impl /*struct*/ QTextEdit {
  pub fn document_0<RetType, T: QTextEdit_document_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.document_0(self);
    // return 1;
  }
}
pub trait QTextEdit_document_0<RetType> {
  fn document_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_document_0<usize> for () {
  fn document_0(self , rsthis: & QTextEdit) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTextEdit8documentEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:112
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setPlaceholderText(const QString &)

/*

*/
impl /*struct*/ QTextEdit {
  pub fn setPlaceholderText_0<RetType, T: QTextEdit_setPlaceholderText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPlaceholderText_0(self);
    // return 1;
  }
}
pub trait QTextEdit_setPlaceholderText_0<RetType> {
  fn setPlaceholderText_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_setPlaceholderText_0<(/*void*/)> for (usize) {
  fn setPlaceholderText_0(self , rsthis: & QTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QTextEdit18setPlaceholderTextERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:113
// index:0
// Public Visibility=Default Availability=Available
// [8] QString placeholderText() const

/*

*/
impl /*struct*/ QTextEdit {
  pub fn placeholderText_0<RetType, T: QTextEdit_placeholderText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.placeholderText_0(self);
    // return 1;
  }
}
pub trait QTextEdit_placeholderText_0<RetType> {
  fn placeholderText_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_placeholderText_0<usize> for () {
  fn placeholderText_0(self , rsthis: & QTextEdit) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTextEdit15placeholderTextEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:115
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTextCursor(const QTextCursor &)

/*
Sets the visible cursor.

See also textCursor().
*/
impl /*struct*/ QTextEdit {
  pub fn setTextCursor_0<RetType, T: QTextEdit_setTextCursor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTextCursor_0(self);
    // return 1;
  }
}
pub trait QTextEdit_setTextCursor_0<RetType> {
  fn setTextCursor_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_setTextCursor_0<(/*void*/)> for (usize) {
  fn setTextCursor_0(self , rsthis: & QTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QTextEdit13setTextCursorERK11QTextCursor", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:116
// index:0
// Public Visibility=Default Availability=Available
// [8] QTextCursor textCursor() const

/*
Returns a copy of the QTextCursor that represents the currently visible cursor. Note that changes on the returned cursor do not affect QTextEdit's cursor; use setTextCursor() to update the visible cursor.

See also setTextCursor().
*/
impl /*struct*/ QTextEdit {
  pub fn textCursor_0<RetType, T: QTextEdit_textCursor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.textCursor_0(self);
    // return 1;
  }
}
pub trait QTextEdit_textCursor_0<RetType> {
  fn textCursor_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_textCursor_0<usize> for () {
  fn textCursor_0(self , rsthis: & QTextEdit) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTextEdit10textCursorEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:118
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isReadOnly() const

/*

*/
impl /*struct*/ QTextEdit {
  pub fn isReadOnly_0<RetType, T: QTextEdit_isReadOnly_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isReadOnly_0(self);
    // return 1;
  }
}
pub trait QTextEdit_isReadOnly_0<RetType> {
  fn isReadOnly_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_isReadOnly_0<bool> for () {
  fn isReadOnly_0(self , rsthis: & QTextEdit) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTextEdit10isReadOnlyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:119
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setReadOnly(bool)

/*

*/
impl /*struct*/ QTextEdit {
  pub fn setReadOnly_0<RetType, T: QTextEdit_setReadOnly_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setReadOnly_0(self);
    // return 1;
  }
}
pub trait QTextEdit_setReadOnly_0<RetType> {
  fn setReadOnly_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_setReadOnly_0<(/*void*/)> for (bool) {
  fn setReadOnly_0(self , rsthis: & QTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN9QTextEdit11setReadOnlyEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:121
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTextInteractionFlags(Qt::TextInteractionFlags)

/*

*/
impl /*struct*/ QTextEdit {
  pub fn setTextInteractionFlags_0<RetType, T: QTextEdit_setTextInteractionFlags_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTextInteractionFlags_0(self);
    // return 1;
  }
}
pub trait QTextEdit_setTextInteractionFlags_0<RetType> {
  fn setTextInteractionFlags_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_setTextInteractionFlags_0<(/*void*/)> for (i32) {
  fn setTextInteractionFlags_0(self , rsthis: & QTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QTextEdit23setTextInteractionFlagsE6QFlagsIN2Qt19TextInteractionFlagEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:122
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::TextInteractionFlags textInteractionFlags() const

/*

*/
impl /*struct*/ QTextEdit {
  pub fn textInteractionFlags_0<RetType, T: QTextEdit_textInteractionFlags_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.textInteractionFlags_0(self);
    // return 1;
  }
}
pub trait QTextEdit_textInteractionFlags_0<RetType> {
  fn textInteractionFlags_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_textInteractionFlags_0<i32> for () {
  fn textInteractionFlags_0(self , rsthis: & QTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTextEdit20textInteractionFlagsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:124
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal fontPointSize() const

/*
Returns the point size of the font of the current format.

See also setFontFamily(), setCurrentFont(), and setFontPointSize().
*/
impl /*struct*/ QTextEdit {
  pub fn fontPointSize_0<RetType, T: QTextEdit_fontPointSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fontPointSize_0(self);
    // return 1;
  }
}
pub trait QTextEdit_fontPointSize_0<RetType> {
  fn fontPointSize_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_fontPointSize_0<f64> for () {
  fn fontPointSize_0(self , rsthis: & QTextEdit) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTextEdit13fontPointSizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:125
// index:0
// Public Visibility=Default Availability=Available
// [8] QString fontFamily() const

/*
Returns the font family of the current format.

See also setFontFamily(), setCurrentFont(), and setFontPointSize().
*/
impl /*struct*/ QTextEdit {
  pub fn fontFamily_0<RetType, T: QTextEdit_fontFamily_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fontFamily_0(self);
    // return 1;
  }
}
pub trait QTextEdit_fontFamily_0<RetType> {
  fn fontFamily_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_fontFamily_0<usize> for () {
  fn fontFamily_0(self , rsthis: & QTextEdit) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTextEdit10fontFamilyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:126
// index:0
// Public Visibility=Default Availability=Available
// [4] int fontWeight() const

/*
Returns the font weight of the current format.

See also setFontWeight(), setCurrentFont(), setFontPointSize(), and QFont::Weight.
*/
impl /*struct*/ QTextEdit {
  pub fn fontWeight_0<RetType, T: QTextEdit_fontWeight_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fontWeight_0(self);
    // return 1;
  }
}
pub trait QTextEdit_fontWeight_0<RetType> {
  fn fontWeight_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_fontWeight_0<i32> for () {
  fn fontWeight_0(self , rsthis: & QTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTextEdit10fontWeightEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:127
// index:0
// Public Visibility=Default Availability=Available
// [1] bool fontUnderline() const

/*
Returns true if the font of the current format is underlined; otherwise returns false.

See also setFontUnderline().
*/
impl /*struct*/ QTextEdit {
  pub fn fontUnderline_0<RetType, T: QTextEdit_fontUnderline_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fontUnderline_0(self);
    // return 1;
  }
}
pub trait QTextEdit_fontUnderline_0<RetType> {
  fn fontUnderline_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_fontUnderline_0<bool> for () {
  fn fontUnderline_0(self , rsthis: & QTextEdit) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTextEdit13fontUnderlineEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:128
// index:0
// Public Visibility=Default Availability=Available
// [1] bool fontItalic() const

/*
Returns true if the font of the current format is italic; otherwise returns false.

See also setFontItalic().
*/
impl /*struct*/ QTextEdit {
  pub fn fontItalic_0<RetType, T: QTextEdit_fontItalic_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fontItalic_0(self);
    // return 1;
  }
}
pub trait QTextEdit_fontItalic_0<RetType> {
  fn fontItalic_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_fontItalic_0<bool> for () {
  fn fontItalic_0(self , rsthis: & QTextEdit) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTextEdit10fontItalicEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:129
// index:0
// Public Visibility=Default Availability=Available
// [16] QColor textColor() const

/*
Returns the text color of the current format.

See also setTextColor().
*/
impl /*struct*/ QTextEdit {
  pub fn textColor_0<RetType, T: QTextEdit_textColor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.textColor_0(self);
    // return 1;
  }
}
pub trait QTextEdit_textColor_0<RetType> {
  fn textColor_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_textColor_0<usize> for () {
  fn textColor_0(self , rsthis: & QTextEdit) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTextEdit9textColorEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:130
// index:0
// Public Visibility=Default Availability=Available
// [16] QColor textBackgroundColor() const

/*
Returns the text background color of the current format.

This function was introduced in  Qt 4.4.

See also setTextBackgroundColor().
*/
impl /*struct*/ QTextEdit {
  pub fn textBackgroundColor_0<RetType, T: QTextEdit_textBackgroundColor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.textBackgroundColor_0(self);
    // return 1;
  }
}
pub trait QTextEdit_textBackgroundColor_0<RetType> {
  fn textBackgroundColor_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_textBackgroundColor_0<usize> for () {
  fn textBackgroundColor_0(self , rsthis: & QTextEdit) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTextEdit19textBackgroundColorEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:131
// index:0
// Public Visibility=Default Availability=Available
// [16] QFont currentFont() const

/*
Returns the font of the current format.

See also setCurrentFont(), setFontFamily(), and setFontPointSize().
*/
impl /*struct*/ QTextEdit {
  pub fn currentFont_0<RetType, T: QTextEdit_currentFont_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.currentFont_0(self);
    // return 1;
  }
}
pub trait QTextEdit_currentFont_0<RetType> {
  fn currentFont_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_currentFont_0<usize> for () {
  fn currentFont_0(self , rsthis: & QTextEdit) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTextEdit11currentFontEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:132
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::Alignment alignment() const

/*
Returns the alignment of the current paragraph.

See also setAlignment().
*/
impl /*struct*/ QTextEdit {
  pub fn alignment_0<RetType, T: QTextEdit_alignment_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.alignment_0(self);
    // return 1;
  }
}
pub trait QTextEdit_alignment_0<RetType> {
  fn alignment_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_alignment_0<i32> for () {
  fn alignment_0(self , rsthis: & QTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTextEdit9alignmentEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:134
// index:0
// Public Visibility=Default Availability=Available
// [-2] void mergeCurrentCharFormat(const QTextCharFormat &)

/*
Merges the properties specified in modifier into the current character format by calling QTextCursor::mergeCharFormat on the editor's cursor. If the editor has a selection then the properties of modifier are directly applied to the selection.

See also QTextCursor::mergeCharFormat().
*/
impl /*struct*/ QTextEdit {
  pub fn mergeCurrentCharFormat_0<RetType, T: QTextEdit_mergeCurrentCharFormat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mergeCurrentCharFormat_0(self);
    // return 1;
  }
}
pub trait QTextEdit_mergeCurrentCharFormat_0<RetType> {
  fn mergeCurrentCharFormat_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_mergeCurrentCharFormat_0<(/*void*/)> for (usize) {
  fn mergeCurrentCharFormat_0(self , rsthis: & QTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QTextEdit22mergeCurrentCharFormatERK15QTextCharFormat", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:136
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCurrentCharFormat(const QTextCharFormat &)

/*
Sets the char format that is be used when inserting new text to format by calling QTextCursor::setCharFormat() on the editor's cursor. If the editor has a selection then the char format is directly applied to the selection.

See also currentCharFormat().
*/
impl /*struct*/ QTextEdit {
  pub fn setCurrentCharFormat_0<RetType, T: QTextEdit_setCurrentCharFormat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCurrentCharFormat_0(self);
    // return 1;
  }
}
pub trait QTextEdit_setCurrentCharFormat_0<RetType> {
  fn setCurrentCharFormat_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_setCurrentCharFormat_0<(/*void*/)> for (usize) {
  fn setCurrentCharFormat_0(self , rsthis: & QTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QTextEdit20setCurrentCharFormatERK15QTextCharFormat", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:137
// index:0
// Public Visibility=Default Availability=Available
// [16] QTextCharFormat currentCharFormat() const

/*
Returns the char format that is used when inserting new text.

See also setCurrentCharFormat().
*/
impl /*struct*/ QTextEdit {
  pub fn currentCharFormat_0<RetType, T: QTextEdit_currentCharFormat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.currentCharFormat_0(self);
    // return 1;
  }
}
pub trait QTextEdit_currentCharFormat_0<RetType> {
  fn currentCharFormat_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_currentCharFormat_0<usize> for () {
  fn currentCharFormat_0(self , rsthis: & QTextEdit) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTextEdit17currentCharFormatEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:139
// index:0
// Public Visibility=Default Availability=Available
// [4] QTextEdit::AutoFormatting autoFormatting() const

/*

*/
impl /*struct*/ QTextEdit {
  pub fn autoFormatting_0<RetType, T: QTextEdit_autoFormatting_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.autoFormatting_0(self);
    // return 1;
  }
}
pub trait QTextEdit_autoFormatting_0<RetType> {
  fn autoFormatting_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_autoFormatting_0<i32> for () {
  fn autoFormatting_0(self , rsthis: & QTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTextEdit14autoFormattingEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:140
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setAutoFormatting(QTextEdit::AutoFormatting)

/*

*/
impl /*struct*/ QTextEdit {
  pub fn setAutoFormatting_0<RetType, T: QTextEdit_setAutoFormatting_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setAutoFormatting_0(self);
    // return 1;
  }
}
pub trait QTextEdit_setAutoFormatting_0<RetType> {
  fn setAutoFormatting_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_setAutoFormatting_0<(/*void*/)> for (i32) {
  fn setAutoFormatting_0(self , rsthis: & QTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QTextEdit17setAutoFormattingE6QFlagsINS_18AutoFormattingFlagEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:142
// index:0
// Public Visibility=Default Availability=Available
// [1] bool tabChangesFocus() const

/*

*/
impl /*struct*/ QTextEdit {
  pub fn tabChangesFocus_0<RetType, T: QTextEdit_tabChangesFocus_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.tabChangesFocus_0(self);
    // return 1;
  }
}
pub trait QTextEdit_tabChangesFocus_0<RetType> {
  fn tabChangesFocus_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_tabChangesFocus_0<bool> for () {
  fn tabChangesFocus_0(self , rsthis: & QTextEdit) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTextEdit15tabChangesFocusEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:143
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTabChangesFocus(bool)

/*

*/
impl /*struct*/ QTextEdit {
  pub fn setTabChangesFocus_0<RetType, T: QTextEdit_setTabChangesFocus_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTabChangesFocus_0(self);
    // return 1;
  }
}
pub trait QTextEdit_setTabChangesFocus_0<RetType> {
  fn setTabChangesFocus_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_setTabChangesFocus_0<(/*void*/)> for (bool) {
  fn setTabChangesFocus_0(self , rsthis: & QTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN9QTextEdit18setTabChangesFocusEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:145
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setDocumentTitle(const QString &)

/*

*/
impl /*struct*/ QTextEdit {
  pub fn setDocumentTitle_0<RetType, T: QTextEdit_setDocumentTitle_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDocumentTitle_0(self);
    // return 1;
  }
}
pub trait QTextEdit_setDocumentTitle_0<RetType> {
  fn setDocumentTitle_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_setDocumentTitle_0<(/*void*/)> for (usize) {
  fn setDocumentTitle_0(self , rsthis: & QTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QTextEdit16setDocumentTitleERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:147
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QString documentTitle() const

/*

*/
impl /*struct*/ QTextEdit {
  pub fn documentTitle_0<RetType, T: QTextEdit_documentTitle_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.documentTitle_0(self);
    // return 1;
  }
}
pub trait QTextEdit_documentTitle_0<RetType> {
  fn documentTitle_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_documentTitle_0<usize> for () {
  fn documentTitle_0(self , rsthis: & QTextEdit) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTextEdit13documentTitleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:150
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isUndoRedoEnabled() const

/*

*/
impl /*struct*/ QTextEdit {
  pub fn isUndoRedoEnabled_0<RetType, T: QTextEdit_isUndoRedoEnabled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isUndoRedoEnabled_0(self);
    // return 1;
  }
}
pub trait QTextEdit_isUndoRedoEnabled_0<RetType> {
  fn isUndoRedoEnabled_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_isUndoRedoEnabled_0<bool> for () {
  fn isUndoRedoEnabled_0(self , rsthis: & QTextEdit) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTextEdit17isUndoRedoEnabledEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:152
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setUndoRedoEnabled(bool)

/*

*/
impl /*struct*/ QTextEdit {
  pub fn setUndoRedoEnabled_0<RetType, T: QTextEdit_setUndoRedoEnabled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setUndoRedoEnabled_0(self);
    // return 1;
  }
}
pub trait QTextEdit_setUndoRedoEnabled_0<RetType> {
  fn setUndoRedoEnabled_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_setUndoRedoEnabled_0<(/*void*/)> for (bool) {
  fn setUndoRedoEnabled_0(self , rsthis: & QTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN9QTextEdit18setUndoRedoEnabledEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:155
// index:0
// Public Visibility=Default Availability=Available
// [4] QTextEdit::LineWrapMode lineWrapMode() const

/*

*/
impl /*struct*/ QTextEdit {
  pub fn lineWrapMode_0<RetType, T: QTextEdit_lineWrapMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.lineWrapMode_0(self);
    // return 1;
  }
}
pub trait QTextEdit_lineWrapMode_0<RetType> {
  fn lineWrapMode_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_lineWrapMode_0<i32> for () {
  fn lineWrapMode_0(self , rsthis: & QTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTextEdit12lineWrapModeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:156
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setLineWrapMode(QTextEdit::LineWrapMode)

/*

*/
impl /*struct*/ QTextEdit {
  pub fn setLineWrapMode_0<RetType, T: QTextEdit_setLineWrapMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setLineWrapMode_0(self);
    // return 1;
  }
}
pub trait QTextEdit_setLineWrapMode_0<RetType> {
  fn setLineWrapMode_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_setLineWrapMode_0<(/*void*/)> for (i32) {
  fn setLineWrapMode_0(self , rsthis: & QTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QTextEdit15setLineWrapModeENS_12LineWrapModeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:158
// index:0
// Public Visibility=Default Availability=Available
// [4] int lineWrapColumnOrWidth() const

/*

*/
impl /*struct*/ QTextEdit {
  pub fn lineWrapColumnOrWidth_0<RetType, T: QTextEdit_lineWrapColumnOrWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.lineWrapColumnOrWidth_0(self);
    // return 1;
  }
}
pub trait QTextEdit_lineWrapColumnOrWidth_0<RetType> {
  fn lineWrapColumnOrWidth_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_lineWrapColumnOrWidth_0<i32> for () {
  fn lineWrapColumnOrWidth_0(self , rsthis: & QTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTextEdit21lineWrapColumnOrWidthEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:159
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setLineWrapColumnOrWidth(int)

/*

*/
impl /*struct*/ QTextEdit {
  pub fn setLineWrapColumnOrWidth_0<RetType, T: QTextEdit_setLineWrapColumnOrWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setLineWrapColumnOrWidth_0(self);
    // return 1;
  }
}
pub trait QTextEdit_setLineWrapColumnOrWidth_0<RetType> {
  fn setLineWrapColumnOrWidth_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_setLineWrapColumnOrWidth_0<(/*void*/)> for (i32) {
  fn setLineWrapColumnOrWidth_0(self , rsthis: & QTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QTextEdit24setLineWrapColumnOrWidthEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:161
// index:0
// Public Visibility=Default Availability=Available
// [4] QTextOption::WrapMode wordWrapMode() const

/*

*/
impl /*struct*/ QTextEdit {
  pub fn wordWrapMode_0<RetType, T: QTextEdit_wordWrapMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.wordWrapMode_0(self);
    // return 1;
  }
}
pub trait QTextEdit_wordWrapMode_0<RetType> {
  fn wordWrapMode_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_wordWrapMode_0<i32> for () {
  fn wordWrapMode_0(self , rsthis: & QTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTextEdit12wordWrapModeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:162
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setWordWrapMode(QTextOption::WrapMode)

/*

*/
impl /*struct*/ QTextEdit {
  pub fn setWordWrapMode_0<RetType, T: QTextEdit_setWordWrapMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setWordWrapMode_0(self);
    // return 1;
  }
}
pub trait QTextEdit_setWordWrapMode_0<RetType> {
  fn setWordWrapMode_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_setWordWrapMode_0<(/*void*/)> for (i32) {
  fn setWordWrapMode_0(self , rsthis: & QTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QTextEdit15setWordWrapModeEN11QTextOption8WrapModeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:164
// index:0
// Public Visibility=Default Availability=Available
// [1] bool find(const QString &, QTextDocument::FindFlags)

/*
Finds the next occurrence of the string, exp, using the given options. Returns true if exp was found and changes the cursor to select the match; otherwise returns false.
*/
impl /*struct*/ QTextEdit {
  pub fn find_0<RetType, T: QTextEdit_find_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.find_0(self);
    // return 1;
  }
}
pub trait QTextEdit_find_0<RetType> {
  fn find_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_find_0<bool> for (usize,i32) {
  fn find_0(self , rsthis: & QTextEdit) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QTextEdit4findERK7QString6QFlagsIN13QTextDocument8FindFlagEE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:166
// index:1
// Public Visibility=Default Availability=Available
// [1] bool find(const QRegExp &, QTextDocument::FindFlags)

/*
Finds the next occurrence of the string, exp, using the given options. Returns true if exp was found and changes the cursor to select the match; otherwise returns false.
*/
impl /*struct*/ QTextEdit {
  pub fn find_1<RetType, T: QTextEdit_find_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.find_1(self);
    // return 1;
  }
}
pub trait QTextEdit_find_1<RetType> {
  fn find_1(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_find_1<bool> for (usize,i32) {
  fn find_1(self , rsthis: & QTextEdit) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QTextEdit4findERK7QRegExp6QFlagsIN13QTextDocument8FindFlagEE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:169
// index:0
// Public Visibility=Default Availability=Available
// [8] QString toPlainText() const

/*
QString QTextEdit::toPlainText() const

Returns the text of the text edit as plain text.

Note: Getter function for property plainText. 

See also QTextEdit::setPlainText().
*/
impl /*struct*/ QTextEdit {
  pub fn toPlainText_0<RetType, T: QTextEdit_toPlainText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toPlainText_0(self);
    // return 1;
  }
}
pub trait QTextEdit_toPlainText_0<RetType> {
  fn toPlainText_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_toPlainText_0<usize> for () {
  fn toPlainText_0(self , rsthis: & QTextEdit) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTextEdit11toPlainTextEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:171
// index:0
// Public Visibility=Default Availability=Available
// [8] QString toHtml() const

/*

*/
impl /*struct*/ QTextEdit {
  pub fn toHtml_0<RetType, T: QTextEdit_toHtml_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toHtml_0(self);
    // return 1;
  }
}
pub trait QTextEdit_toHtml_0<RetType> {
  fn toHtml_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_toHtml_0<usize> for () {
  fn toHtml_0(self , rsthis: & QTextEdit) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTextEdit6toHtmlEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:174
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ensureCursorVisible()

/*
Ensures that the cursor is visible by scrolling the text edit if necessary.
*/
impl /*struct*/ QTextEdit {
  pub fn ensureCursorVisible_0<RetType, T: QTextEdit_ensureCursorVisible_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.ensureCursorVisible_0(self);
    // return 1;
  }
}
pub trait QTextEdit_ensureCursorVisible_0<RetType> {
  fn ensureCursorVisible_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_ensureCursorVisible_0<(/*void*/)> for () {
  fn ensureCursorVisible_0(self , rsthis: & QTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN9QTextEdit19ensureCursorVisibleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:176
// index:0
// Public virtual Visibility=Default Availability=Available
// [16] QVariant loadResource(int, const QUrl &)

/*
Loads the resource specified by the given type and name.

This function is an extension of QTextDocument::loadResource().

See also QTextDocument::loadResource().
*/
impl /*struct*/ QTextEdit {
  pub fn loadResource_0<RetType, T: QTextEdit_loadResource_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.loadResource_0(self);
    // return 1;
  }
}
pub trait QTextEdit_loadResource_0<RetType> {
  fn loadResource_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_loadResource_0<usize> for (i32,usize) {
  fn loadResource_0(self , rsthis: & QTextEdit) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QTextEdit12loadResourceEiRK4QUrl", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:178
// index:0
// Public Visibility=Default Availability=Available
// [8] QMenu * createStandardContextMenu()

/*
This function creates the standard context menu which is shown when the user clicks on the text edit with the right mouse button. It is called from the default contextMenuEvent() handler. The popup menu's ownership is transferred to the caller.

We recommend that you use the createStandardContextMenu(QPoint) version instead which will enable the actions that are sensitive to where the user clicked.
*/
impl /*struct*/ QTextEdit {
  pub fn createStandardContextMenu_0<RetType, T: QTextEdit_createStandardContextMenu_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.createStandardContextMenu_0(self);
    // return 1;
  }
}
pub trait QTextEdit_createStandardContextMenu_0<RetType> {
  fn createStandardContextMenu_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_createStandardContextMenu_0<usize> for () {
  fn createStandardContextMenu_0(self , rsthis: & QTextEdit) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QTextEdit25createStandardContextMenuEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:179
// index:1
// Public Visibility=Default Availability=Available
// [8] QMenu * createStandardContextMenu(const QPoint &)

/*
This function creates the standard context menu which is shown when the user clicks on the text edit with the right mouse button. It is called from the default contextMenuEvent() handler. The popup menu's ownership is transferred to the caller.

We recommend that you use the createStandardContextMenu(QPoint) version instead which will enable the actions that are sensitive to where the user clicked.
*/
impl /*struct*/ QTextEdit {
  pub fn createStandardContextMenu_1<RetType, T: QTextEdit_createStandardContextMenu_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.createStandardContextMenu_1(self);
    // return 1;
  }
}
pub trait QTextEdit_createStandardContextMenu_1<RetType> {
  fn createStandardContextMenu_1(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_createStandardContextMenu_1<usize> for (usize) {
  fn createStandardContextMenu_1(self , rsthis: & QTextEdit) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QTextEdit25createStandardContextMenuERK6QPoint", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:182
// index:0
// Public Visibility=Default Availability=Available
// [8] QTextCursor cursorForPosition(const QPoint &) const

/*
returns a QTextCursor at position pos (in viewport coordinates).
*/
impl /*struct*/ QTextEdit {
  pub fn cursorForPosition_0<RetType, T: QTextEdit_cursorForPosition_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cursorForPosition_0(self);
    // return 1;
  }
}
pub trait QTextEdit_cursorForPosition_0<RetType> {
  fn cursorForPosition_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_cursorForPosition_0<usize> for (usize) {
  fn cursorForPosition_0(self , rsthis: & QTextEdit) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTextEdit17cursorForPositionERK6QPoint", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:183
// index:0
// Public Visibility=Default Availability=Available
// [16] QRect cursorRect(const QTextCursor &) const

/*
returns a rectangle (in viewport coordinates) that includes the cursor.
*/
impl /*struct*/ QTextEdit {
  pub fn cursorRect_0<RetType, T: QTextEdit_cursorRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cursorRect_0(self);
    // return 1;
  }
}
pub trait QTextEdit_cursorRect_0<RetType> {
  fn cursorRect_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_cursorRect_0<usize> for (usize) {
  fn cursorRect_0(self , rsthis: & QTextEdit) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTextEdit10cursorRectERK11QTextCursor", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:184
// index:1
// Public Visibility=Default Availability=Available
// [16] QRect cursorRect() const

/*
returns a rectangle (in viewport coordinates) that includes the cursor.
*/
impl /*struct*/ QTextEdit {
  pub fn cursorRect_1<RetType, T: QTextEdit_cursorRect_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cursorRect_1(self);
    // return 1;
  }
}
pub trait QTextEdit_cursorRect_1<RetType> {
  fn cursorRect_1(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_cursorRect_1<usize> for () {
  fn cursorRect_1(self , rsthis: & QTextEdit) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTextEdit10cursorRectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:186
// index:0
// Public Visibility=Default Availability=Available
// [8] QString anchorAt(const QPoint &) const

/*
Returns the reference of the anchor at position pos, or an empty string if no anchor exists at that point.
*/
impl /*struct*/ QTextEdit {
  pub fn anchorAt_0<RetType, T: QTextEdit_anchorAt_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.anchorAt_0(self);
    // return 1;
  }
}
pub trait QTextEdit_anchorAt_0<RetType> {
  fn anchorAt_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_anchorAt_0<usize> for (usize) {
  fn anchorAt_0(self , rsthis: & QTextEdit) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTextEdit8anchorAtERK6QPoint", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:188
// index:0
// Public Visibility=Default Availability=Available
// [1] bool overwriteMode() const

/*

*/
impl /*struct*/ QTextEdit {
  pub fn overwriteMode_0<RetType, T: QTextEdit_overwriteMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.overwriteMode_0(self);
    // return 1;
  }
}
pub trait QTextEdit_overwriteMode_0<RetType> {
  fn overwriteMode_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_overwriteMode_0<bool> for () {
  fn overwriteMode_0(self , rsthis: & QTextEdit) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTextEdit13overwriteModeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:189
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setOverwriteMode(bool)

/*

*/
impl /*struct*/ QTextEdit {
  pub fn setOverwriteMode_0<RetType, T: QTextEdit_setOverwriteMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setOverwriteMode_0(self);
    // return 1;
  }
}
pub trait QTextEdit_setOverwriteMode_0<RetType> {
  fn setOverwriteMode_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_setOverwriteMode_0<(/*void*/)> for (bool) {
  fn setOverwriteMode_0(self , rsthis: & QTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN9QTextEdit16setOverwriteModeEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:192
// index:0
// Public Visibility=Default Availability=Available
// [4] int tabStopWidth() const

/*

*/
impl /*struct*/ QTextEdit {
  pub fn tabStopWidth_0<RetType, T: QTextEdit_tabStopWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.tabStopWidth_0(self);
    // return 1;
  }
}
pub trait QTextEdit_tabStopWidth_0<RetType> {
  fn tabStopWidth_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_tabStopWidth_0<i32> for () {
  fn tabStopWidth_0(self , rsthis: & QTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTextEdit12tabStopWidthEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:193
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTabStopWidth(int)

/*

*/
impl /*struct*/ QTextEdit {
  pub fn setTabStopWidth_0<RetType, T: QTextEdit_setTabStopWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTabStopWidth_0(self);
    // return 1;
  }
}
pub trait QTextEdit_setTabStopWidth_0<RetType> {
  fn setTabStopWidth_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_setTabStopWidth_0<(/*void*/)> for (i32) {
  fn setTabStopWidth_0(self , rsthis: & QTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QTextEdit15setTabStopWidthEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:196
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal tabStopDistance() const

/*

*/
impl /*struct*/ QTextEdit {
  pub fn tabStopDistance_0<RetType, T: QTextEdit_tabStopDistance_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.tabStopDistance_0(self);
    // return 1;
  }
}
pub trait QTextEdit_tabStopDistance_0<RetType> {
  fn tabStopDistance_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_tabStopDistance_0<f64> for () {
  fn tabStopDistance_0(self , rsthis: & QTextEdit) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTextEdit15tabStopDistanceEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:197
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTabStopDistance(qreal)

/*

*/
impl /*struct*/ QTextEdit {
  pub fn setTabStopDistance_0<RetType, T: QTextEdit_setTabStopDistance_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTabStopDistance_0(self);
    // return 1;
  }
}
pub trait QTextEdit_setTabStopDistance_0<RetType> {
  fn setTabStopDistance_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_setTabStopDistance_0<(/*void*/)> for (f64) {
  fn setTabStopDistance_0(self , rsthis: & QTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN9QTextEdit18setTabStopDistanceEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:199
// index:0
// Public Visibility=Default Availability=Available
// [4] int cursorWidth() const

/*

*/
impl /*struct*/ QTextEdit {
  pub fn cursorWidth_0<RetType, T: QTextEdit_cursorWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cursorWidth_0(self);
    // return 1;
  }
}
pub trait QTextEdit_cursorWidth_0<RetType> {
  fn cursorWidth_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_cursorWidth_0<i32> for () {
  fn cursorWidth_0(self , rsthis: & QTextEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTextEdit11cursorWidthEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:200
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCursorWidth(int)

/*

*/
impl /*struct*/ QTextEdit {
  pub fn setCursorWidth_0<RetType, T: QTextEdit_setCursorWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCursorWidth_0(self);
    // return 1;
  }
}
pub trait QTextEdit_setCursorWidth_0<RetType> {
  fn setCursorWidth_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_setCursorWidth_0<(/*void*/)> for (i32) {
  fn setCursorWidth_0(self , rsthis: & QTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QTextEdit14setCursorWidthEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:202
// index:0
// Public Visibility=Default Availability=Available
// [1] bool acceptRichText() const

/*

*/
impl /*struct*/ QTextEdit {
  pub fn acceptRichText_0<RetType, T: QTextEdit_acceptRichText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.acceptRichText_0(self);
    // return 1;
  }
}
pub trait QTextEdit_acceptRichText_0<RetType> {
  fn acceptRichText_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_acceptRichText_0<bool> for () {
  fn acceptRichText_0(self , rsthis: & QTextEdit) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTextEdit14acceptRichTextEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:203
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setAcceptRichText(bool)

/*

*/
impl /*struct*/ QTextEdit {
  pub fn setAcceptRichText_0<RetType, T: QTextEdit_setAcceptRichText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setAcceptRichText_0(self);
    // return 1;
  }
}
pub trait QTextEdit_setAcceptRichText_0<RetType> {
  fn setAcceptRichText_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_setAcceptRichText_0<(/*void*/)> for (bool) {
  fn setAcceptRichText_0(self , rsthis: & QTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN9QTextEdit17setAcceptRichTextEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:213
// index:0
// Public Visibility=Default Availability=Available
// [-2] void moveCursor(QTextCursor::MoveOperation, QTextCursor::MoveMode)

/*
Moves the cursor by performing the given operation.

If mode is QTextCursor::KeepAnchor, the cursor selects the text it moves over. This is the same effect that the user achieves when they hold down the Shift key and move the cursor with the cursor keys.

This function was introduced in  Qt 4.2.

See also QTextCursor::movePosition().
*/
impl /*struct*/ QTextEdit {
  pub fn moveCursor_0<RetType, T: QTextEdit_moveCursor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.moveCursor_0(self);
    // return 1;
  }
}
pub trait QTextEdit_moveCursor_0<RetType> {
  fn moveCursor_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_moveCursor_0<(/*void*/)> for (i32,i32) {
  fn moveCursor_0(self , rsthis: & QTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QTextEdit10moveCursorEN11QTextCursor13MoveOperationENS0_8MoveModeE", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:215
// index:0
// Public Visibility=Default Availability=Available
// [1] bool canPaste() const

/*
Returns whether text can be pasted from the clipboard into the textedit.

This function was introduced in  Qt 4.2.
*/
impl /*struct*/ QTextEdit {
  pub fn canPaste_0<RetType, T: QTextEdit_canPaste_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.canPaste_0(self);
    // return 1;
  }
}
pub trait QTextEdit_canPaste_0<RetType> {
  fn canPaste_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_canPaste_0<bool> for () {
  fn canPaste_0(self , rsthis: & QTextEdit) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTextEdit8canPasteEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:217
// index:0
// Public Visibility=Default Availability=Available
// [-2] void print(QPagedPaintDevice *) const

/*
Convenience function to print the text edit's document to the given printer. This is equivalent to calling the print method on the document directly except that this function also supports QPrinter::Selection as print range.

This function was introduced in  Qt 4.3.

See also QTextDocument::print().
*/
impl /*struct*/ QTextEdit {
  pub fn print_0<RetType, T: QTextEdit_print_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.print_0(self);
    // return 1;
  }
}
pub trait QTextEdit_print_0<RetType> {
  fn print_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_print_0<(/*void*/)> for (usize) {
  fn print_0(self , rsthis: & QTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK9QTextEdit5printEP17QPagedPaintDevice", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:219
// index:0
// Public virtual Visibility=Default Availability=Available
// [16] QVariant inputMethodQuery(Qt::InputMethodQuery) const

/*
Reimplemented from QWidget::inputMethodQuery().
*/
impl /*struct*/ QTextEdit {
  pub fn inputMethodQuery_0<RetType, T: QTextEdit_inputMethodQuery_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.inputMethodQuery_0(self);
    // return 1;
  }
}
pub trait QTextEdit_inputMethodQuery_0<RetType> {
  fn inputMethodQuery_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_inputMethodQuery_0<usize> for (i32) {
  fn inputMethodQuery_0(self , rsthis: & QTextEdit) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTextEdit16inputMethodQueryEN2Qt16InputMethodQueryE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:220
// index:1
// Public Visibility=Default Availability=Available
// [16] QVariant inputMethodQuery(Qt::InputMethodQuery, QVariant) const

/*
Reimplemented from QWidget::inputMethodQuery().
*/
impl /*struct*/ QTextEdit {
  pub fn inputMethodQuery_1<RetType, T: QTextEdit_inputMethodQuery_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.inputMethodQuery_1(self);
    // return 1;
  }
}
pub trait QTextEdit_inputMethodQuery_1<RetType> {
  fn inputMethodQuery_1(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_inputMethodQuery_1<usize> for (i32,usize) {
  fn inputMethodQuery_1(self , rsthis: & QTextEdit) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTextEdit16inputMethodQueryEN2Qt16InputMethodQueryE8QVariant", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:223
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFontPointSize(qreal)

/*
Sets the point size of the current format to s.

Note that if s is zero or negative, the behavior of this function is not defined.

See also fontPointSize(), setCurrentFont(), and setFontFamily().
*/
impl /*struct*/ QTextEdit {
  pub fn setFontPointSize_0<RetType, T: QTextEdit_setFontPointSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFontPointSize_0(self);
    // return 1;
  }
}
pub trait QTextEdit_setFontPointSize_0<RetType> {
  fn setFontPointSize_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_setFontPointSize_0<(/*void*/)> for (f64) {
  fn setFontPointSize_0(self , rsthis: & QTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN9QTextEdit16setFontPointSizeEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:224
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFontFamily(const QString &)

/*
Sets the font family of the current format to fontFamily.

See also fontFamily() and setCurrentFont().
*/
impl /*struct*/ QTextEdit {
  pub fn setFontFamily_0<RetType, T: QTextEdit_setFontFamily_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFontFamily_0(self);
    // return 1;
  }
}
pub trait QTextEdit_setFontFamily_0<RetType> {
  fn setFontFamily_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_setFontFamily_0<(/*void*/)> for (usize) {
  fn setFontFamily_0(self , rsthis: & QTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QTextEdit13setFontFamilyERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:225
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFontWeight(int)

/*
Sets the font weight of the current format to the given weight, where the value used is in the range defined by the QFont::Weight enum.

See also fontWeight(), setCurrentFont(), and setFontFamily().
*/
impl /*struct*/ QTextEdit {
  pub fn setFontWeight_0<RetType, T: QTextEdit_setFontWeight_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFontWeight_0(self);
    // return 1;
  }
}
pub trait QTextEdit_setFontWeight_0<RetType> {
  fn setFontWeight_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_setFontWeight_0<(/*void*/)> for (i32) {
  fn setFontWeight_0(self , rsthis: & QTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QTextEdit13setFontWeightEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:226
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFontUnderline(bool)

/*
If underline is true, sets the current format to underline; otherwise sets the current format to non-underline.

See also fontUnderline().
*/
impl /*struct*/ QTextEdit {
  pub fn setFontUnderline_0<RetType, T: QTextEdit_setFontUnderline_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFontUnderline_0(self);
    // return 1;
  }
}
pub trait QTextEdit_setFontUnderline_0<RetType> {
  fn setFontUnderline_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_setFontUnderline_0<(/*void*/)> for (bool) {
  fn setFontUnderline_0(self , rsthis: & QTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN9QTextEdit16setFontUnderlineEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:227
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFontItalic(bool)

/*
If italic is true, sets the current format to italic; otherwise sets the current format to non-italic.

See also fontItalic().
*/
impl /*struct*/ QTextEdit {
  pub fn setFontItalic_0<RetType, T: QTextEdit_setFontItalic_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFontItalic_0(self);
    // return 1;
  }
}
pub trait QTextEdit_setFontItalic_0<RetType> {
  fn setFontItalic_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_setFontItalic_0<(/*void*/)> for (bool) {
  fn setFontItalic_0(self , rsthis: & QTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN9QTextEdit13setFontItalicEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:228
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTextColor(const QColor &)

/*
Sets the text color of the current format to c.

See also textColor().
*/
impl /*struct*/ QTextEdit {
  pub fn setTextColor_0<RetType, T: QTextEdit_setTextColor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTextColor_0(self);
    // return 1;
  }
}
pub trait QTextEdit_setTextColor_0<RetType> {
  fn setTextColor_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_setTextColor_0<(/*void*/)> for (usize) {
  fn setTextColor_0(self , rsthis: & QTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QTextEdit12setTextColorERK6QColor", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:229
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTextBackgroundColor(const QColor &)

/*
Sets the text background color of the current format to c.

This function was introduced in  Qt 4.4.

See also textBackgroundColor().
*/
impl /*struct*/ QTextEdit {
  pub fn setTextBackgroundColor_0<RetType, T: QTextEdit_setTextBackgroundColor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTextBackgroundColor_0(self);
    // return 1;
  }
}
pub trait QTextEdit_setTextBackgroundColor_0<RetType> {
  fn setTextBackgroundColor_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_setTextBackgroundColor_0<(/*void*/)> for (usize) {
  fn setTextBackgroundColor_0(self , rsthis: & QTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QTextEdit22setTextBackgroundColorERK6QColor", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:230
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCurrentFont(const QFont &)

/*
Sets the font of the current format to f.

See also currentFont(), setFontPointSize(), and setFontFamily().
*/
impl /*struct*/ QTextEdit {
  pub fn setCurrentFont_0<RetType, T: QTextEdit_setCurrentFont_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCurrentFont_0(self);
    // return 1;
  }
}
pub trait QTextEdit_setCurrentFont_0<RetType> {
  fn setCurrentFont_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_setCurrentFont_0<(/*void*/)> for (usize) {
  fn setCurrentFont_0(self , rsthis: & QTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QTextEdit14setCurrentFontERK5QFont", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:231
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setAlignment(Qt::Alignment)

/*
Sets the alignment of the current paragraph to a. Valid alignments are Qt::AlignLeft, Qt::AlignRight, Qt::AlignJustify and Qt::AlignCenter (which centers horizontally).

See also alignment().
*/
impl /*struct*/ QTextEdit {
  pub fn setAlignment_0<RetType, T: QTextEdit_setAlignment_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setAlignment_0(self);
    // return 1;
  }
}
pub trait QTextEdit_setAlignment_0<RetType> {
  fn setAlignment_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_setAlignment_0<(/*void*/)> for (i32) {
  fn setAlignment_0(self , rsthis: & QTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QTextEdit12setAlignmentE6QFlagsIN2Qt13AlignmentFlagEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:233
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
impl /*struct*/ QTextEdit {
  pub fn setPlainText_0<RetType, T: QTextEdit_setPlainText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPlainText_0(self);
    // return 1;
  }
}
pub trait QTextEdit_setPlainText_0<RetType> {
  fn setPlainText_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_setPlainText_0<(/*void*/)> for (usize) {
  fn setPlainText_0(self , rsthis: & QTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QTextEdit12setPlainTextERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:235
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setHtml(const QString &)

/*

*/
impl /*struct*/ QTextEdit {
  pub fn setHtml_0<RetType, T: QTextEdit_setHtml_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setHtml_0(self);
    // return 1;
  }
}
pub trait QTextEdit_setHtml_0<RetType> {
  fn setHtml_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_setHtml_0<(/*void*/)> for (usize) {
  fn setHtml_0(self , rsthis: & QTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QTextEdit7setHtmlERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:237
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setText(const QString &)

/*
Sets the text edit's text. The text can be plain text or HTML and the text edit will try to guess the right format.

Use setHtml() or setPlainText() directly to avoid text edit's guessing.

This function was introduced in  Qt 4.2.

See also toPlainText() and toHtml().
*/
impl /*struct*/ QTextEdit {
  pub fn setText_0<RetType, T: QTextEdit_setText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setText_0(self);
    // return 1;
  }
}
pub trait QTextEdit_setText_0<RetType> {
  fn setText_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_setText_0<(/*void*/)> for (usize) {
  fn setText_0(self , rsthis: & QTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QTextEdit7setTextERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:240
// index:0
// Public Visibility=Default Availability=Available
// [-2] void cut()

/*
Copies the selected text to the clipboard and deletes it from the text edit.

If there is no selected text nothing happens.

See also copy() and paste().
*/
impl /*struct*/ QTextEdit {
  pub fn cut_0<RetType, T: QTextEdit_cut_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cut_0(self);
    // return 1;
  }
}
pub trait QTextEdit_cut_0<RetType> {
  fn cut_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_cut_0<(/*void*/)> for () {
  fn cut_0(self , rsthis: & QTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN9QTextEdit3cutEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:241
// index:0
// Public Visibility=Default Availability=Available
// [-2] void copy()

/*
Copies any selected text to the clipboard.

See also copyAvailable().
*/
impl /*struct*/ QTextEdit {
  pub fn copy_0<RetType, T: QTextEdit_copy_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.copy_0(self);
    // return 1;
  }
}
pub trait QTextEdit_copy_0<RetType> {
  fn copy_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_copy_0<(/*void*/)> for () {
  fn copy_0(self , rsthis: & QTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN9QTextEdit4copyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:242
// index:0
// Public Visibility=Default Availability=Available
// [-2] void paste()

/*
Pastes the text from the clipboard into the text edit at the current cursor position.

If there is no text in the clipboard nothing happens.

To change the behavior of this function, i.e. to modify what QTextEdit can paste and how it is being pasted, reimplement the virtual canInsertFromMimeData() and insertFromMimeData() functions.

See also cut() and copy().
*/
impl /*struct*/ QTextEdit {
  pub fn paste_0<RetType, T: QTextEdit_paste_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.paste_0(self);
    // return 1;
  }
}
pub trait QTextEdit_paste_0<RetType> {
  fn paste_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_paste_0<(/*void*/)> for () {
  fn paste_0(self , rsthis: & QTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN9QTextEdit5pasteEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:245
// index:0
// Public Visibility=Default Availability=Available
// [-2] void undo()

/*
Undoes the last operation.

If there is no operation to undo, i.e. there is no undo step in the undo/redo history, nothing happens.

This function was introduced in  Qt 4.2.

See also redo().
*/
impl /*struct*/ QTextEdit {
  pub fn undo_0<RetType, T: QTextEdit_undo_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.undo_0(self);
    // return 1;
  }
}
pub trait QTextEdit_undo_0<RetType> {
  fn undo_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_undo_0<(/*void*/)> for () {
  fn undo_0(self , rsthis: & QTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN9QTextEdit4undoEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:246
// index:0
// Public Visibility=Default Availability=Available
// [-2] void redo()

/*
Redoes the last operation.

If there is no operation to redo, i.e. there is no redo step in the undo/redo history, nothing happens.

This function was introduced in  Qt 4.2.

See also undo().
*/
impl /*struct*/ QTextEdit {
  pub fn redo_0<RetType, T: QTextEdit_redo_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.redo_0(self);
    // return 1;
  }
}
pub trait QTextEdit_redo_0<RetType> {
  fn redo_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_redo_0<(/*void*/)> for () {
  fn redo_0(self , rsthis: & QTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN9QTextEdit4redoEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:248
// index:0
// Public Visibility=Default Availability=Available
// [-2] void clear()

/*
Deletes all the text in the text edit.

Note that the undo/redo history is cleared by this function.

See also cut(), setPlainText(), and setHtml().
*/
impl /*struct*/ QTextEdit {
  pub fn clear_0<RetType, T: QTextEdit_clear_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.clear_0(self);
    // return 1;
  }
}
pub trait QTextEdit_clear_0<RetType> {
  fn clear_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_clear_0<(/*void*/)> for () {
  fn clear_0(self , rsthis: & QTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN9QTextEdit5clearEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:249
// index:0
// Public Visibility=Default Availability=Available
// [-2] void selectAll()

/*
Selects all text.

See also copy(), cut(), and textCursor().
*/
impl /*struct*/ QTextEdit {
  pub fn selectAll_0<RetType, T: QTextEdit_selectAll_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.selectAll_0(self);
    // return 1;
  }
}
pub trait QTextEdit_selectAll_0<RetType> {
  fn selectAll_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_selectAll_0<(/*void*/)> for () {
  fn selectAll_0(self , rsthis: & QTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN9QTextEdit9selectAllEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:251
// index:0
// Public Visibility=Default Availability=Available
// [-2] void insertPlainText(const QString &)

/*
Convenience slot that inserts text at the current cursor position.

It is equivalent to


  edit->textCursor().insertText(text);
*/
impl /*struct*/ QTextEdit {
  pub fn insertPlainText_0<RetType, T: QTextEdit_insertPlainText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.insertPlainText_0(self);
    // return 1;
  }
}
pub trait QTextEdit_insertPlainText_0<RetType> {
  fn insertPlainText_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_insertPlainText_0<(/*void*/)> for (usize) {
  fn insertPlainText_0(self , rsthis: & QTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QTextEdit15insertPlainTextERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:253
// index:0
// Public Visibility=Default Availability=Available
// [-2] void insertHtml(const QString &)

/*
Convenience slot that inserts text which is assumed to be of html formatting at the current cursor position.

It is equivalent to:


  edit->textCursor().insertHtml(fragment);



Note: When using this function with a style sheet, the style sheet will only apply to the current block in the document. In order to apply a style sheet throughout a document, use QTextDocument::setDefaultStyleSheet() instead.
*/
impl /*struct*/ QTextEdit {
  pub fn insertHtml_0<RetType, T: QTextEdit_insertHtml_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.insertHtml_0(self);
    // return 1;
  }
}
pub trait QTextEdit_insertHtml_0<RetType> {
  fn insertHtml_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_insertHtml_0<(/*void*/)> for (usize) {
  fn insertHtml_0(self , rsthis: & QTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QTextEdit10insertHtmlERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:258
// index:0
// Public Visibility=Default Availability=Available
// [-2] void scrollToAnchor(const QString &)

/*
Scrolls the text edit so that the anchor with the given name is visible; does nothing if the name is empty, or is already visible, or isn't found.
*/
impl /*struct*/ QTextEdit {
  pub fn scrollToAnchor_0<RetType, T: QTextEdit_scrollToAnchor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.scrollToAnchor_0(self);
    // return 1;
  }
}
pub trait QTextEdit_scrollToAnchor_0<RetType> {
  fn scrollToAnchor_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_scrollToAnchor_0<(/*void*/)> for (usize) {
  fn scrollToAnchor_0(self , rsthis: & QTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QTextEdit14scrollToAnchorERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:260
// index:0
// Public Visibility=Default Availability=Available
// [-2] void zoomIn(int)

/*
Zooms in on the text by making the base font size range points larger and recalculating all font sizes to be the new size. This does not change the size of any images.

See also zoomOut().
*/
impl /*struct*/ QTextEdit {
  pub fn zoomIn_0<RetType, T: QTextEdit_zoomIn_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.zoomIn_0(self);
    // return 1;
  }
}
pub trait QTextEdit_zoomIn_0<RetType> {
  fn zoomIn_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_zoomIn_0<(/*void*/)> for (i32) {
  fn zoomIn_0(self , rsthis: & QTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QTextEdit6zoomInEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:261
// index:0
// Public Visibility=Default Availability=Available
// [-2] void zoomOut(int)

/*
This is an overloaded function.

Zooms out on the text by making the base font size range points smaller and recalculating all font sizes to be the new size. This does not change the size of any images.

See also zoomIn().
*/
impl /*struct*/ QTextEdit {
  pub fn zoomOut_0<RetType, T: QTextEdit_zoomOut_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.zoomOut_0(self);
    // return 1;
  }
}
pub trait QTextEdit_zoomOut_0<RetType> {
  fn zoomOut_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_zoomOut_0<(/*void*/)> for (i32) {
  fn zoomOut_0(self , rsthis: & QTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QTextEdit7zoomOutEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:264
// index:0
// Public Visibility=Default Availability=Available
// [-2] void textChanged()

/*
This signal is emitted whenever the document's content changes; for example, when text is inserted or deleted, or when formatting is applied.

Note: Notifier signal for property html.
*/
impl /*struct*/ QTextEdit {
  pub fn textChanged_0<RetType, T: QTextEdit_textChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.textChanged_0(self);
    // return 1;
  }
}
pub trait QTextEdit_textChanged_0<RetType> {
  fn textChanged_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_textChanged_0<(/*void*/)> for () {
  fn textChanged_0(self , rsthis: & QTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN9QTextEdit11textChangedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:265
// index:0
// Public Visibility=Default Availability=Available
// [-2] void undoAvailable(bool)

/*
This signal is emitted whenever undo operations become available (available is true) or unavailable (available is false).
*/
impl /*struct*/ QTextEdit {
  pub fn undoAvailable_0<RetType, T: QTextEdit_undoAvailable_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.undoAvailable_0(self);
    // return 1;
  }
}
pub trait QTextEdit_undoAvailable_0<RetType> {
  fn undoAvailable_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_undoAvailable_0<(/*void*/)> for (bool) {
  fn undoAvailable_0(self , rsthis: & QTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN9QTextEdit13undoAvailableEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:266
// index:0
// Public Visibility=Default Availability=Available
// [-2] void redoAvailable(bool)

/*
This signal is emitted whenever redo operations become available (available is true) or unavailable (available is false).
*/
impl /*struct*/ QTextEdit {
  pub fn redoAvailable_0<RetType, T: QTextEdit_redoAvailable_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.redoAvailable_0(self);
    // return 1;
  }
}
pub trait QTextEdit_redoAvailable_0<RetType> {
  fn redoAvailable_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_redoAvailable_0<(/*void*/)> for (bool) {
  fn redoAvailable_0(self , rsthis: & QTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN9QTextEdit13redoAvailableEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:267
// index:0
// Public Visibility=Default Availability=Available
// [-2] void currentCharFormatChanged(const QTextCharFormat &)

/*
This signal is emitted if the current character format has changed, for example caused by a change of the cursor position.

The new format is f.

See also setCurrentCharFormat().
*/
impl /*struct*/ QTextEdit {
  pub fn currentCharFormatChanged_0<RetType, T: QTextEdit_currentCharFormatChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.currentCharFormatChanged_0(self);
    // return 1;
  }
}
pub trait QTextEdit_currentCharFormatChanged_0<RetType> {
  fn currentCharFormatChanged_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_currentCharFormatChanged_0<(/*void*/)> for (usize) {
  fn currentCharFormatChanged_0(self , rsthis: & QTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QTextEdit24currentCharFormatChangedERK15QTextCharFormat", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:268
// index:0
// Public Visibility=Default Availability=Available
// [-2] void copyAvailable(bool)

/*
This signal is emitted when text is selected or de-selected in the text edit.

When text is selected this signal will be emitted with yes set to true. If no text has been selected or if the selected text is de-selected this signal is emitted with yes set to false.

If yes is true then copy() can be used to copy the selection to the clipboard. If yes is false then copy() does nothing.

See also selectionChanged().
*/
impl /*struct*/ QTextEdit {
  pub fn copyAvailable_0<RetType, T: QTextEdit_copyAvailable_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.copyAvailable_0(self);
    // return 1;
  }
}
pub trait QTextEdit_copyAvailable_0<RetType> {
  fn copyAvailable_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_copyAvailable_0<(/*void*/)> for (bool) {
  fn copyAvailable_0(self , rsthis: & QTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN9QTextEdit13copyAvailableEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:269
// index:0
// Public Visibility=Default Availability=Available
// [-2] void selectionChanged()

/*
This signal is emitted whenever the selection changes.

See also copyAvailable().
*/
impl /*struct*/ QTextEdit {
  pub fn selectionChanged_0<RetType, T: QTextEdit_selectionChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.selectionChanged_0(self);
    // return 1;
  }
}
pub trait QTextEdit_selectionChanged_0<RetType> {
  fn selectionChanged_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_selectionChanged_0<(/*void*/)> for () {
  fn selectionChanged_0(self , rsthis: & QTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN9QTextEdit16selectionChangedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:270
// index:0
// Public Visibility=Default Availability=Available
// [-2] void cursorPositionChanged()

/*
This signal is emitted whenever the position of the cursor changed.
*/
impl /*struct*/ QTextEdit {
  pub fn cursorPositionChanged_0<RetType, T: QTextEdit_cursorPositionChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cursorPositionChanged_0(self);
    // return 1;
  }
}
pub trait QTextEdit_cursorPositionChanged_0<RetType> {
  fn cursorPositionChanged_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_cursorPositionChanged_0<(/*void*/)> for () {
  fn cursorPositionChanged_0(self , rsthis: & QTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN9QTextEdit21cursorPositionChangedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:273
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool event(QEvent *)

/*

*/
impl /*struct*/ QTextEdit {
  pub fn event_0<RetType, T: QTextEdit_event_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.event_0(self);
    // return 1;
  }
}
pub trait QTextEdit_event_0<RetType> {
  fn event_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_event_0<bool> for (usize) {
  fn event_0(self , rsthis: & QTextEdit) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QTextEdit5eventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:274
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void timerEvent(QTimerEvent *)

/*

*/
impl /*struct*/ QTextEdit {
  pub fn timerEvent_0<RetType, T: QTextEdit_timerEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.timerEvent_0(self);
    // return 1;
  }
}
pub trait QTextEdit_timerEvent_0<RetType> {
  fn timerEvent_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_timerEvent_0<(/*void*/)> for (usize) {
  fn timerEvent_0(self , rsthis: & QTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QTextEdit10timerEventEP11QTimerEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:275
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void keyPressEvent(QKeyEvent *)

/*
Reimplemented from QWidget::keyPressEvent().
*/
impl /*struct*/ QTextEdit {
  pub fn keyPressEvent_0<RetType, T: QTextEdit_keyPressEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.keyPressEvent_0(self);
    // return 1;
  }
}
pub trait QTextEdit_keyPressEvent_0<RetType> {
  fn keyPressEvent_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_keyPressEvent_0<(/*void*/)> for (usize) {
  fn keyPressEvent_0(self , rsthis: & QTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QTextEdit13keyPressEventEP9QKeyEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:276
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void keyReleaseEvent(QKeyEvent *)

/*
Reimplemented from QWidget::keyReleaseEvent().
*/
impl /*struct*/ QTextEdit {
  pub fn keyReleaseEvent_0<RetType, T: QTextEdit_keyReleaseEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.keyReleaseEvent_0(self);
    // return 1;
  }
}
pub trait QTextEdit_keyReleaseEvent_0<RetType> {
  fn keyReleaseEvent_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_keyReleaseEvent_0<(/*void*/)> for (usize) {
  fn keyReleaseEvent_0(self , rsthis: & QTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QTextEdit15keyReleaseEventEP9QKeyEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:277
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void resizeEvent(QResizeEvent *)

/*
Reimplemented from QWidget::resizeEvent().
*/
impl /*struct*/ QTextEdit {
  pub fn resizeEvent_0<RetType, T: QTextEdit_resizeEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resizeEvent_0(self);
    // return 1;
  }
}
pub trait QTextEdit_resizeEvent_0<RetType> {
  fn resizeEvent_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_resizeEvent_0<(/*void*/)> for (usize) {
  fn resizeEvent_0(self , rsthis: & QTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QTextEdit11resizeEventEP12QResizeEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:278
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void paintEvent(QPaintEvent *)

/*
Reimplemented from QWidget::paintEvent().

This event handler can be reimplemented in a subclass to receive paint events passed in event. It is usually unnecessary to reimplement this function in a subclass of QTextEdit.

Warning: The underlying text document must not be modified from within a reimplementation of this function.
*/
impl /*struct*/ QTextEdit {
  pub fn paintEvent_0<RetType, T: QTextEdit_paintEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.paintEvent_0(self);
    // return 1;
  }
}
pub trait QTextEdit_paintEvent_0<RetType> {
  fn paintEvent_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_paintEvent_0<(/*void*/)> for (usize) {
  fn paintEvent_0(self , rsthis: & QTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QTextEdit10paintEventEP11QPaintEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:279
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mousePressEvent(QMouseEvent *)

/*
Reimplemented from QWidget::mousePressEvent().
*/
impl /*struct*/ QTextEdit {
  pub fn mousePressEvent_0<RetType, T: QTextEdit_mousePressEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mousePressEvent_0(self);
    // return 1;
  }
}
pub trait QTextEdit_mousePressEvent_0<RetType> {
  fn mousePressEvent_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_mousePressEvent_0<(/*void*/)> for (usize) {
  fn mousePressEvent_0(self , rsthis: & QTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QTextEdit15mousePressEventEP11QMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:280
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mouseMoveEvent(QMouseEvent *)

/*
Reimplemented from QWidget::mouseMoveEvent().
*/
impl /*struct*/ QTextEdit {
  pub fn mouseMoveEvent_0<RetType, T: QTextEdit_mouseMoveEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mouseMoveEvent_0(self);
    // return 1;
  }
}
pub trait QTextEdit_mouseMoveEvent_0<RetType> {
  fn mouseMoveEvent_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_mouseMoveEvent_0<(/*void*/)> for (usize) {
  fn mouseMoveEvent_0(self , rsthis: & QTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QTextEdit14mouseMoveEventEP11QMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:281
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mouseReleaseEvent(QMouseEvent *)

/*
Reimplemented from QWidget::mouseReleaseEvent().
*/
impl /*struct*/ QTextEdit {
  pub fn mouseReleaseEvent_0<RetType, T: QTextEdit_mouseReleaseEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mouseReleaseEvent_0(self);
    // return 1;
  }
}
pub trait QTextEdit_mouseReleaseEvent_0<RetType> {
  fn mouseReleaseEvent_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_mouseReleaseEvent_0<(/*void*/)> for (usize) {
  fn mouseReleaseEvent_0(self , rsthis: & QTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QTextEdit17mouseReleaseEventEP11QMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:282
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mouseDoubleClickEvent(QMouseEvent *)

/*
Reimplemented from QWidget::mouseDoubleClickEvent().
*/
impl /*struct*/ QTextEdit {
  pub fn mouseDoubleClickEvent_0<RetType, T: QTextEdit_mouseDoubleClickEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mouseDoubleClickEvent_0(self);
    // return 1;
  }
}
pub trait QTextEdit_mouseDoubleClickEvent_0<RetType> {
  fn mouseDoubleClickEvent_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_mouseDoubleClickEvent_0<(/*void*/)> for (usize) {
  fn mouseDoubleClickEvent_0(self , rsthis: & QTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QTextEdit21mouseDoubleClickEventEP11QMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:283
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool focusNextPrevChild(bool)

/*
Reimplemented from QWidget::focusNextPrevChild().
*/
impl /*struct*/ QTextEdit {
  pub fn focusNextPrevChild_0<RetType, T: QTextEdit_focusNextPrevChild_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.focusNextPrevChild_0(self);
    // return 1;
  }
}
pub trait QTextEdit_focusNextPrevChild_0<RetType> {
  fn focusNextPrevChild_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_focusNextPrevChild_0<bool> for (bool) {
  fn focusNextPrevChild_0(self , rsthis: & QTextEdit) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QTextEdit18focusNextPrevChildEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:285
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void contextMenuEvent(QContextMenuEvent *)

/*
Reimplemented from QWidget::contextMenuEvent().

Shows the standard context menu created with createStandardContextMenu().

If you do not want the text edit to have a context menu, you can set its contextMenuPolicy to Qt::NoContextMenu. If you want to customize the context menu, reimplement this function. If you want to extend the standard context menu, reimplement this function, call createStandardContextMenu() and extend the menu returned.

Information about the event is passed in the event object.


  void MyTextEdit::contextMenuEvent(QContextMenuEvent *event)
  {
      QMenu *menu = createStandardContextMenu();
      menu->addAction(tr("My Menu Item"));
      //...
      menu->exec(event->globalPos());
      delete menu;
  }
*/
impl /*struct*/ QTextEdit {
  pub fn contextMenuEvent_0<RetType, T: QTextEdit_contextMenuEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.contextMenuEvent_0(self);
    // return 1;
  }
}
pub trait QTextEdit_contextMenuEvent_0<RetType> {
  fn contextMenuEvent_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_contextMenuEvent_0<(/*void*/)> for (usize) {
  fn contextMenuEvent_0(self , rsthis: & QTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QTextEdit16contextMenuEventEP17QContextMenuEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:288
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void dragEnterEvent(QDragEnterEvent *)

/*
Reimplemented from QWidget::dragEnterEvent().
*/
impl /*struct*/ QTextEdit {
  pub fn dragEnterEvent_0<RetType, T: QTextEdit_dragEnterEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dragEnterEvent_0(self);
    // return 1;
  }
}
pub trait QTextEdit_dragEnterEvent_0<RetType> {
  fn dragEnterEvent_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_dragEnterEvent_0<(/*void*/)> for (usize) {
  fn dragEnterEvent_0(self , rsthis: & QTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QTextEdit14dragEnterEventEP15QDragEnterEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:289
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void dragLeaveEvent(QDragLeaveEvent *)

/*
Reimplemented from QWidget::dragLeaveEvent().
*/
impl /*struct*/ QTextEdit {
  pub fn dragLeaveEvent_0<RetType, T: QTextEdit_dragLeaveEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dragLeaveEvent_0(self);
    // return 1;
  }
}
pub trait QTextEdit_dragLeaveEvent_0<RetType> {
  fn dragLeaveEvent_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_dragLeaveEvent_0<(/*void*/)> for (usize) {
  fn dragLeaveEvent_0(self , rsthis: & QTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QTextEdit14dragLeaveEventEP15QDragLeaveEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:290
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void dragMoveEvent(QDragMoveEvent *)

/*
Reimplemented from QWidget::dragMoveEvent().
*/
impl /*struct*/ QTextEdit {
  pub fn dragMoveEvent_0<RetType, T: QTextEdit_dragMoveEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dragMoveEvent_0(self);
    // return 1;
  }
}
pub trait QTextEdit_dragMoveEvent_0<RetType> {
  fn dragMoveEvent_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_dragMoveEvent_0<(/*void*/)> for (usize) {
  fn dragMoveEvent_0(self , rsthis: & QTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QTextEdit13dragMoveEventEP14QDragMoveEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:291
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void dropEvent(QDropEvent *)

/*
Reimplemented from QWidget::dropEvent().
*/
impl /*struct*/ QTextEdit {
  pub fn dropEvent_0<RetType, T: QTextEdit_dropEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dropEvent_0(self);
    // return 1;
  }
}
pub trait QTextEdit_dropEvent_0<RetType> {
  fn dropEvent_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_dropEvent_0<(/*void*/)> for (usize) {
  fn dropEvent_0(self , rsthis: & QTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QTextEdit9dropEventEP10QDropEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:293
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void focusInEvent(QFocusEvent *)

/*
Reimplemented from QWidget::focusInEvent().
*/
impl /*struct*/ QTextEdit {
  pub fn focusInEvent_0<RetType, T: QTextEdit_focusInEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.focusInEvent_0(self);
    // return 1;
  }
}
pub trait QTextEdit_focusInEvent_0<RetType> {
  fn focusInEvent_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_focusInEvent_0<(/*void*/)> for (usize) {
  fn focusInEvent_0(self , rsthis: & QTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QTextEdit12focusInEventEP11QFocusEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:294
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void focusOutEvent(QFocusEvent *)

/*
Reimplemented from QWidget::focusOutEvent().
*/
impl /*struct*/ QTextEdit {
  pub fn focusOutEvent_0<RetType, T: QTextEdit_focusOutEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.focusOutEvent_0(self);
    // return 1;
  }
}
pub trait QTextEdit_focusOutEvent_0<RetType> {
  fn focusOutEvent_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_focusOutEvent_0<(/*void*/)> for (usize) {
  fn focusOutEvent_0(self , rsthis: & QTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QTextEdit13focusOutEventEP11QFocusEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:295
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void showEvent(QShowEvent *)

/*
Reimplemented from QWidget::showEvent().
*/
impl /*struct*/ QTextEdit {
  pub fn showEvent_0<RetType, T: QTextEdit_showEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.showEvent_0(self);
    // return 1;
  }
}
pub trait QTextEdit_showEvent_0<RetType> {
  fn showEvent_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_showEvent_0<(/*void*/)> for (usize) {
  fn showEvent_0(self , rsthis: & QTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QTextEdit9showEventEP10QShowEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:296
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void changeEvent(QEvent *)

/*
Reimplemented from QWidget::changeEvent().
*/
impl /*struct*/ QTextEdit {
  pub fn changeEvent_0<RetType, T: QTextEdit_changeEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.changeEvent_0(self);
    // return 1;
  }
}
pub trait QTextEdit_changeEvent_0<RetType> {
  fn changeEvent_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_changeEvent_0<(/*void*/)> for (usize) {
  fn changeEvent_0(self , rsthis: & QTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QTextEdit11changeEventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:298
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void wheelEvent(QWheelEvent *)

/*
Reimplemented from QWidget::wheelEvent().
*/
impl /*struct*/ QTextEdit {
  pub fn wheelEvent_0<RetType, T: QTextEdit_wheelEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.wheelEvent_0(self);
    // return 1;
  }
}
pub trait QTextEdit_wheelEvent_0<RetType> {
  fn wheelEvent_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_wheelEvent_0<(/*void*/)> for (usize) {
  fn wheelEvent_0(self , rsthis: & QTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QTextEdit10wheelEventEP11QWheelEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:301
// index:0
// Protected virtual Visibility=Default Availability=Available
// [8] QMimeData * createMimeDataFromSelection() const

/*
This function returns a new MIME data object to represent the contents of the text edit's current selection. It is called when the selection needs to be encapsulated into a new QMimeData object; for example, when a drag and drop operation is started, or when data is copied to the clipboard.

If you reimplement this function, note that the ownership of the returned QMimeData object is passed to the caller. The selection can be retrieved by using the textCursor() function.
*/
impl /*struct*/ QTextEdit {
  pub fn createMimeDataFromSelection_0<RetType, T: QTextEdit_createMimeDataFromSelection_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.createMimeDataFromSelection_0(self);
    // return 1;
  }
}
pub trait QTextEdit_createMimeDataFromSelection_0<RetType> {
  fn createMimeDataFromSelection_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_createMimeDataFromSelection_0<usize> for () {
  fn createMimeDataFromSelection_0(self , rsthis: & QTextEdit) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTextEdit27createMimeDataFromSelectionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:302
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool canInsertFromMimeData(const QMimeData *) const

/*
This function returns true if the contents of the MIME data object, specified by source, can be decoded and inserted into the document. It is called for example when during a drag operation the mouse enters this widget and it is necessary to determine whether it is possible to accept the drag and drop operation.

Reimplement this function to enable drag and drop support for additional MIME types.
*/
impl /*struct*/ QTextEdit {
  pub fn canInsertFromMimeData_0<RetType, T: QTextEdit_canInsertFromMimeData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.canInsertFromMimeData_0(self);
    // return 1;
  }
}
pub trait QTextEdit_canInsertFromMimeData_0<RetType> {
  fn canInsertFromMimeData_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_canInsertFromMimeData_0<bool> for (usize) {
  fn canInsertFromMimeData_0(self , rsthis: & QTextEdit) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTextEdit21canInsertFromMimeDataEPK9QMimeData", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:303
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void insertFromMimeData(const QMimeData *)

/*
This function inserts the contents of the MIME data object, specified by source, into the text edit at the current cursor position. It is called whenever text is inserted as the result of a clipboard paste operation, or when the text edit accepts data from a drag and drop operation.

Reimplement this function to enable drag and drop support for additional MIME types.
*/
impl /*struct*/ QTextEdit {
  pub fn insertFromMimeData_0<RetType, T: QTextEdit_insertFromMimeData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.insertFromMimeData_0(self);
    // return 1;
  }
}
pub trait QTextEdit_insertFromMimeData_0<RetType> {
  fn insertFromMimeData_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_insertFromMimeData_0<(/*void*/)> for (usize) {
  fn insertFromMimeData_0(self , rsthis: & QTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QTextEdit18insertFromMimeDataEPK9QMimeData", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:305
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void inputMethodEvent(QInputMethodEvent *)

/*
Reimplemented from QWidget::inputMethodEvent().
*/
impl /*struct*/ QTextEdit {
  pub fn inputMethodEvent_0<RetType, T: QTextEdit_inputMethodEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.inputMethodEvent_0(self);
    // return 1;
  }
}
pub trait QTextEdit_inputMethodEvent_0<RetType> {
  fn inputMethodEvent_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_inputMethodEvent_0<(/*void*/)> for (usize) {
  fn inputMethodEvent_0(self , rsthis: & QTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QTextEdit16inputMethodEventEP17QInputMethodEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:309
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void scrollContentsBy(int, int)

/*
Reimplemented from QAbstractScrollArea::scrollContentsBy().
*/
impl /*struct*/ QTextEdit {
  pub fn scrollContentsBy_0<RetType, T: QTextEdit_scrollContentsBy_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.scrollContentsBy_0(self);
    // return 1;
  }
}
pub trait QTextEdit_scrollContentsBy_0<RetType> {
  fn scrollContentsBy_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_scrollContentsBy_0<(/*void*/)> for (i32,i32) {
  fn scrollContentsBy_0(self , rsthis: & QTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QTextEdit16scrollContentsByEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:310
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void doSetTextCursor(const QTextCursor &)

/*

*/
impl /*struct*/ QTextEdit {
  pub fn doSetTextCursor_0<RetType, T: QTextEdit_doSetTextCursor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.doSetTextCursor_0(self);
    // return 1;
  }
}
pub trait QTextEdit_doSetTextCursor_0<RetType> {
  fn doSetTextCursor_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_doSetTextCursor_0<(/*void*/)> for (usize) {
  fn doSetTextCursor_0(self , rsthis: & QTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QTextEdit15doSetTextCursorERK11QTextCursor", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtextedit.h:312
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void zoomInF(float)

/*

*/
impl /*struct*/ QTextEdit {
  pub fn zoomInF_0<RetType, T: QTextEdit_zoomInF_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.zoomInF_0(self);
    // return 1;
  }
}
pub trait QTextEdit_zoomInF_0<RetType> {
  fn zoomInF_0(self , rsthis: & QTextEdit) -> RetType;
}
impl<'a> /*trait*/ QTextEdit_zoomInF_0<(/*void*/)> for (f32) {
  fn zoomInF_0(self , rsthis: & QTextEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QTextEdit7zoomInFEf", 1,qtrt::FFITY_FLOAT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}


/*
ConstantValue
QTextEdit::NoWrap0
QTextEdit::WidgetWidth1
QTextEdit::FixedPixelWidth2
QTextEdit::FixedColumnWidth3

*/
pub type QTextEdit__LineWrapMode = i32;
// 
pub const QTextEdit__NoWrap :QTextEdit__LineWrapMode = 0;
// 
pub const QTextEdit__WidgetWidth :QTextEdit__LineWrapMode = 1;
// 
pub const QTextEdit__FixedPixelWidth :QTextEdit__LineWrapMode = 2;
// 
pub const QTextEdit__FixedColumnWidth :QTextEdit__LineWrapMode = 3;
pub fn QTextEdit_LineWrapModeItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QTextEdit", val);
}
pub fn QTextEdit_LineWrapModeItemName_s(val: i32) ->String {
  //var nilthis *QTextEdit
  //return nilthis.LineWrapModeItemName(val);
  return QTextEdit_LineWrapModeItemName(val);
}


/*


*/
pub type QTextEdit__AutoFormattingFlag = i32;
// 
pub const QTextEdit__AutoNone :QTextEdit__AutoFormattingFlag = 0;
// 
pub const QTextEdit__AutoBulletList :QTextEdit__AutoFormattingFlag = 1;
// 
pub const QTextEdit__AutoAll :QTextEdit__AutoFormattingFlag = -1;
pub fn QTextEdit_AutoFormattingFlagItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QTextEdit", val);
}
pub fn QTextEdit_AutoFormattingFlagItemName_s(val: i32) ->String {
  //var nilthis *QTextEdit
  //return nilthis.AutoFormattingFlagItemName(val);
  return QTextEdit_AutoFormattingFlagItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
