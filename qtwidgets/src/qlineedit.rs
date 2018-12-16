

// mod ::widgets::QLineEdit
// package qtwidgets
// /usr/include/qt/QtWidgets/qlineedit.h
// #include <qlineedit.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 109
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

// void mousePressEvent(QMouseEvent *)
// func (this *QLineEdit) InheritMousePressEvent(f func(arg0 *qtgui.QMouseEvent/*777 QMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mousePressEvent", f)
// }

// void mouseMoveEvent(QMouseEvent *)
// func (this *QLineEdit) InheritMouseMoveEvent(f func(arg0 *qtgui.QMouseEvent/*777 QMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mouseMoveEvent", f)
// }

// void mouseReleaseEvent(QMouseEvent *)
// func (this *QLineEdit) InheritMouseReleaseEvent(f func(arg0 *qtgui.QMouseEvent/*777 QMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mouseReleaseEvent", f)
// }

// void mouseDoubleClickEvent(QMouseEvent *)
// func (this *QLineEdit) InheritMouseDoubleClickEvent(f func(arg0 *qtgui.QMouseEvent/*777 QMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mouseDoubleClickEvent", f)
// }

// void keyPressEvent(QKeyEvent *)
// func (this *QLineEdit) InheritKeyPressEvent(f func(arg0 *qtgui.QKeyEvent/*777 QKeyEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "keyPressEvent", f)
// }

// void focusInEvent(QFocusEvent *)
// func (this *QLineEdit) InheritFocusInEvent(f func(arg0 *qtgui.QFocusEvent/*777 QFocusEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "focusInEvent", f)
// }

// void focusOutEvent(QFocusEvent *)
// func (this *QLineEdit) InheritFocusOutEvent(f func(arg0 *qtgui.QFocusEvent/*777 QFocusEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "focusOutEvent", f)
// }

// void paintEvent(QPaintEvent *)
// func (this *QLineEdit) InheritPaintEvent(f func(arg0 *qtgui.QPaintEvent/*777 QPaintEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "paintEvent", f)
// }

// void dragEnterEvent(QDragEnterEvent *)
// func (this *QLineEdit) InheritDragEnterEvent(f func(arg0 *qtgui.QDragEnterEvent/*777 QDragEnterEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "dragEnterEvent", f)
// }

// void dragMoveEvent(QDragMoveEvent *)
// func (this *QLineEdit) InheritDragMoveEvent(f func(e *qtgui.QDragMoveEvent/*777 QDragMoveEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "dragMoveEvent", f)
// }

// void dragLeaveEvent(QDragLeaveEvent *)
// func (this *QLineEdit) InheritDragLeaveEvent(f func(e *qtgui.QDragLeaveEvent/*777 QDragLeaveEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "dragLeaveEvent", f)
// }

// void dropEvent(QDropEvent *)
// func (this *QLineEdit) InheritDropEvent(f func(arg0 *qtgui.QDropEvent/*777 QDropEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "dropEvent", f)
// }

// void changeEvent(QEvent *)
// func (this *QLineEdit) InheritChangeEvent(f func(arg0 *qtcore.QEvent/*777 QEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "changeEvent", f)
// }

// void contextMenuEvent(QContextMenuEvent *)
// func (this *QLineEdit) InheritContextMenuEvent(f func(arg0 *qtgui.QContextMenuEvent/*777 QContextMenuEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "contextMenuEvent", f)
// }

// void inputMethodEvent(QInputMethodEvent *)
// func (this *QLineEdit) InheritInputMethodEvent(f func(arg0 *qtgui.QInputMethodEvent/*777 QInputMethodEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "inputMethodEvent", f)
// }

// void initStyleOption(QStyleOptionFrame *)
// func (this *QLineEdit) InheritInitStyleOption(f func(option *QStyleOptionFrame/*777 QStyleOptionFrame **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "initStyleOption", f)
// }

// QRect cursorRect()
// func (this *QLineEdit) InheritCursorRect(f func() unsafe.Pointer) {
//  qtrt.SetAllInheritCallback(this, "cursorRect", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QLineEdit)=48
pub struct QLineEdit {
  qbase: QWidget,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QLineEdit_ITF interface {
//    QWidget_ITF
//    QLineEdit_PTR() *QLineEdit
//}
//func (ptr *QLineEdit) QLineEdit_PTR() *QLineEdit { return ptr }

impl /*struct*/ QLineEdit {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QLineEdit {
    return QLineEdit{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QLineEdit {
//  type Target = QLineEditBASE;
//
//  fn deref(&self) -> &QLineEditBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QLineEditBASE> for QLineEdit {
//  fn as_ref(& self) -> & QLineEditBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qlineedit.h:65
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QLineEdit {
  pub fn metaObject_0<RetType, T: QLineEdit_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QLineEdit_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QLineEdit) -> RetType;
}
impl<'a> /*trait*/ QLineEdit_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QLineEdit) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QLineEdit10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlineedit.h:93
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QLineEdit(QWidget *)

/*
Constructs a line edit with no text.

The maximum text length is set to 32767 characters.

The parent argument is sent to the QWidget constructor.

See also setText() and setMaxLength().
*/
// QLineEdit(QWidget *) ctx.fn_proto_cpp
impl /*struct*/ QLineEdit {
  pub fn QLineEdit_0<T: QLineEdit_QLineEdit_0>(value: T) -> QLineEdit {
    let rsthis = value.QLineEdit_0();
    return rsthis;
    // return 1;
  }
}

pub trait QLineEdit_QLineEdit_0 {
  fn QLineEdit_0(self) -> QLineEdit;
}
// QLineEdit(QWidget *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QLineEdit_QLineEdit_0 for (usize) {
  fn QLineEdit_0(self) -> QLineEdit {
    // unsafe{_ZN9QLineEditC2EP7QWidget()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QLineEditC2EP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QLineEdit{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlineedit.h:94
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QLineEdit(const QString &, QWidget *)

/*
Constructs a line edit with no text.

The maximum text length is set to 32767 characters.

The parent argument is sent to the QWidget constructor.

See also setText() and setMaxLength().
*/
// QLineEdit(const QString &, QWidget *) ctx.fn_proto_cpp
impl /*struct*/ QLineEdit {
  pub fn QLineEdit_1<T: QLineEdit_QLineEdit_1>(value: T) -> QLineEdit {
    let rsthis = value.QLineEdit_1();
    return rsthis;
    // return 1;
  }
}

pub trait QLineEdit_QLineEdit_1 {
  fn QLineEdit_1(self) -> QLineEdit;
}
// QLineEdit(const QString &, QWidget *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QLineEdit_QLineEdit_1 for (usize,usize) {
  fn QLineEdit_1(self) -> QLineEdit {
    // unsafe{_ZN9QLineEditC2ERK7QStringP7QWidget()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QLineEditC2ERK7QStringP7QWidget", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QLineEdit{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlineedit.h:95
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QLineEdit()

/*

*/
pub fn DeleteQLineEdit(this :*mut QLineEdit) {
    // let rv = qtrt::InvokeQtFunc6("_ZN9QLineEditD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 48)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qlineedit.h:97
// index:0
// Public Visibility=Default Availability=Available
// [8] QString text() const

/*

*/
impl /*struct*/ QLineEdit {
  pub fn text_0<RetType, T: QLineEdit_text_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.text_0(self);
    // return 1;
  }
}
pub trait QLineEdit_text_0<RetType> {
  fn text_0(self , rsthis: & QLineEdit) -> RetType;
}
impl<'a> /*trait*/ QLineEdit_text_0<usize> for () {
  fn text_0(self , rsthis: & QLineEdit) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QLineEdit4textEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlineedit.h:99
// index:0
// Public Visibility=Default Availability=Available
// [8] QString displayText() const

/*

*/
impl /*struct*/ QLineEdit {
  pub fn displayText_0<RetType, T: QLineEdit_displayText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.displayText_0(self);
    // return 1;
  }
}
pub trait QLineEdit_displayText_0<RetType> {
  fn displayText_0(self , rsthis: & QLineEdit) -> RetType;
}
impl<'a> /*trait*/ QLineEdit_displayText_0<usize> for () {
  fn displayText_0(self , rsthis: & QLineEdit) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QLineEdit11displayTextEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlineedit.h:101
// index:0
// Public Visibility=Default Availability=Available
// [8] QString placeholderText() const

/*

*/
impl /*struct*/ QLineEdit {
  pub fn placeholderText_0<RetType, T: QLineEdit_placeholderText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.placeholderText_0(self);
    // return 1;
  }
}
pub trait QLineEdit_placeholderText_0<RetType> {
  fn placeholderText_0(self , rsthis: & QLineEdit) -> RetType;
}
impl<'a> /*trait*/ QLineEdit_placeholderText_0<usize> for () {
  fn placeholderText_0(self , rsthis: & QLineEdit) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QLineEdit15placeholderTextEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlineedit.h:102
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setPlaceholderText(const QString &)

/*

*/
impl /*struct*/ QLineEdit {
  pub fn setPlaceholderText_0<RetType, T: QLineEdit_setPlaceholderText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPlaceholderText_0(self);
    // return 1;
  }
}
pub trait QLineEdit_setPlaceholderText_0<RetType> {
  fn setPlaceholderText_0(self , rsthis: & QLineEdit) -> RetType;
}
impl<'a> /*trait*/ QLineEdit_setPlaceholderText_0<(/*void*/)> for (usize) {
  fn setPlaceholderText_0(self , rsthis: & QLineEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QLineEdit18setPlaceholderTextERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlineedit.h:104
// index:0
// Public Visibility=Default Availability=Available
// [4] int maxLength() const

/*

*/
impl /*struct*/ QLineEdit {
  pub fn maxLength_0<RetType, T: QLineEdit_maxLength_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.maxLength_0(self);
    // return 1;
  }
}
pub trait QLineEdit_maxLength_0<RetType> {
  fn maxLength_0(self , rsthis: & QLineEdit) -> RetType;
}
impl<'a> /*trait*/ QLineEdit_maxLength_0<i32> for () {
  fn maxLength_0(self , rsthis: & QLineEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QLineEdit9maxLengthEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlineedit.h:105
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setMaxLength(int)

/*

*/
impl /*struct*/ QLineEdit {
  pub fn setMaxLength_0<RetType, T: QLineEdit_setMaxLength_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMaxLength_0(self);
    // return 1;
  }
}
pub trait QLineEdit_setMaxLength_0<RetType> {
  fn setMaxLength_0(self , rsthis: & QLineEdit) -> RetType;
}
impl<'a> /*trait*/ QLineEdit_setMaxLength_0<(/*void*/)> for (i32) {
  fn setMaxLength_0(self , rsthis: & QLineEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QLineEdit12setMaxLengthEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlineedit.h:107
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFrame(bool)

/*

*/
impl /*struct*/ QLineEdit {
  pub fn setFrame_0<RetType, T: QLineEdit_setFrame_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFrame_0(self);
    // return 1;
  }
}
pub trait QLineEdit_setFrame_0<RetType> {
  fn setFrame_0(self , rsthis: & QLineEdit) -> RetType;
}
impl<'a> /*trait*/ QLineEdit_setFrame_0<(/*void*/)> for (bool) {
  fn setFrame_0(self , rsthis: & QLineEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN9QLineEdit8setFrameEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlineedit.h:108
// index:0
// Public Visibility=Default Availability=Available
// [1] bool hasFrame() const

/*

*/
impl /*struct*/ QLineEdit {
  pub fn hasFrame_0<RetType, T: QLineEdit_hasFrame_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hasFrame_0(self);
    // return 1;
  }
}
pub trait QLineEdit_hasFrame_0<RetType> {
  fn hasFrame_0(self , rsthis: & QLineEdit) -> RetType;
}
impl<'a> /*trait*/ QLineEdit_hasFrame_0<bool> for () {
  fn hasFrame_0(self , rsthis: & QLineEdit) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QLineEdit8hasFrameEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlineedit.h:110
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setClearButtonEnabled(bool)

/*

*/
impl /*struct*/ QLineEdit {
  pub fn setClearButtonEnabled_0<RetType, T: QLineEdit_setClearButtonEnabled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setClearButtonEnabled_0(self);
    // return 1;
  }
}
pub trait QLineEdit_setClearButtonEnabled_0<RetType> {
  fn setClearButtonEnabled_0(self , rsthis: & QLineEdit) -> RetType;
}
impl<'a> /*trait*/ QLineEdit_setClearButtonEnabled_0<(/*void*/)> for (bool) {
  fn setClearButtonEnabled_0(self , rsthis: & QLineEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN9QLineEdit21setClearButtonEnabledEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlineedit.h:111
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isClearButtonEnabled() const

/*

*/
impl /*struct*/ QLineEdit {
  pub fn isClearButtonEnabled_0<RetType, T: QLineEdit_isClearButtonEnabled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isClearButtonEnabled_0(self);
    // return 1;
  }
}
pub trait QLineEdit_isClearButtonEnabled_0<RetType> {
  fn isClearButtonEnabled_0(self , rsthis: & QLineEdit) -> RetType;
}
impl<'a> /*trait*/ QLineEdit_isClearButtonEnabled_0<bool> for () {
  fn isClearButtonEnabled_0(self , rsthis: & QLineEdit) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QLineEdit20isClearButtonEnabledEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlineedit.h:115
// index:0
// Public Visibility=Default Availability=Available
// [4] QLineEdit::EchoMode echoMode() const

/*

*/
impl /*struct*/ QLineEdit {
  pub fn echoMode_0<RetType, T: QLineEdit_echoMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.echoMode_0(self);
    // return 1;
  }
}
pub trait QLineEdit_echoMode_0<RetType> {
  fn echoMode_0(self , rsthis: & QLineEdit) -> RetType;
}
impl<'a> /*trait*/ QLineEdit_echoMode_0<i32> for () {
  fn echoMode_0(self , rsthis: & QLineEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QLineEdit8echoModeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlineedit.h:116
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setEchoMode(QLineEdit::EchoMode)

/*

*/
impl /*struct*/ QLineEdit {
  pub fn setEchoMode_0<RetType, T: QLineEdit_setEchoMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setEchoMode_0(self);
    // return 1;
  }
}
pub trait QLineEdit_setEchoMode_0<RetType> {
  fn setEchoMode_0(self , rsthis: & QLineEdit) -> RetType;
}
impl<'a> /*trait*/ QLineEdit_setEchoMode_0<(/*void*/)> for (i32) {
  fn setEchoMode_0(self , rsthis: & QLineEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QLineEdit11setEchoModeENS_8EchoModeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlineedit.h:118
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isReadOnly() const

/*

*/
impl /*struct*/ QLineEdit {
  pub fn isReadOnly_0<RetType, T: QLineEdit_isReadOnly_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isReadOnly_0(self);
    // return 1;
  }
}
pub trait QLineEdit_isReadOnly_0<RetType> {
  fn isReadOnly_0(self , rsthis: & QLineEdit) -> RetType;
}
impl<'a> /*trait*/ QLineEdit_isReadOnly_0<bool> for () {
  fn isReadOnly_0(self , rsthis: & QLineEdit) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QLineEdit10isReadOnlyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlineedit.h:119
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setReadOnly(bool)

/*

*/
impl /*struct*/ QLineEdit {
  pub fn setReadOnly_0<RetType, T: QLineEdit_setReadOnly_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setReadOnly_0(self);
    // return 1;
  }
}
pub trait QLineEdit_setReadOnly_0<RetType> {
  fn setReadOnly_0(self , rsthis: & QLineEdit) -> RetType;
}
impl<'a> /*trait*/ QLineEdit_setReadOnly_0<(/*void*/)> for (bool) {
  fn setReadOnly_0(self , rsthis: & QLineEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN9QLineEdit11setReadOnlyEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlineedit.h:122
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setValidator(const QValidator *)

/*
Sets this line edit to only accept input that the validator, v, will accept. This allows you to place any arbitrary constraints on the text which may be entered.

If v == 0, setValidator() removes the current input validator. The initial setting is to have no input validator (i.e. any input is accepted up to maxLength()).

See also validator(), hasAcceptableInput(), QIntValidator, QDoubleValidator, and QRegExpValidator.
*/
impl /*struct*/ QLineEdit {
  pub fn setValidator_0<RetType, T: QLineEdit_setValidator_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setValidator_0(self);
    // return 1;
  }
}
pub trait QLineEdit_setValidator_0<RetType> {
  fn setValidator_0(self , rsthis: & QLineEdit) -> RetType;
}
impl<'a> /*trait*/ QLineEdit_setValidator_0<(/*void*/)> for (usize) {
  fn setValidator_0(self , rsthis: & QLineEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QLineEdit12setValidatorEPK10QValidator", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlineedit.h:123
// index:0
// Public Visibility=Default Availability=Available
// [8] const QValidator * validator() const

/*
Returns a pointer to the current input validator, or 0 if no validator has been set.

See also setValidator().
*/
impl /*struct*/ QLineEdit {
  pub fn validator_0<RetType, T: QLineEdit_validator_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.validator_0(self);
    // return 1;
  }
}
pub trait QLineEdit_validator_0<RetType> {
  fn validator_0(self , rsthis: & QLineEdit) -> RetType;
}
impl<'a> /*trait*/ QLineEdit_validator_0<usize> for () {
  fn validator_0(self , rsthis: & QLineEdit) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QLineEdit9validatorEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlineedit.h:127
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCompleter(QCompleter *)

/*
Sets this line edit to provide auto completions from the completer, c. The completion mode is set using QCompleter::setCompletionMode().

To use a QCompleter with a QValidator or QLineEdit::inputMask, you need to ensure that the model provided to QCompleter contains valid entries. You can use the QSortFilterProxyModel to ensure that the QCompleter's model contains only valid entries.

If c == 0, setCompleter() removes the current completer, effectively disabling auto completion.

This function was introduced in  Qt 4.2.

See also completer() and QCompleter.
*/
impl /*struct*/ QLineEdit {
  pub fn setCompleter_0<RetType, T: QLineEdit_setCompleter_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCompleter_0(self);
    // return 1;
  }
}
pub trait QLineEdit_setCompleter_0<RetType> {
  fn setCompleter_0(self , rsthis: & QLineEdit) -> RetType;
}
impl<'a> /*trait*/ QLineEdit_setCompleter_0<(/*void*/)> for (usize) {
  fn setCompleter_0(self , rsthis: & QLineEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QLineEdit12setCompleterEP10QCompleter", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlineedit.h:128
// index:0
// Public Visibility=Default Availability=Available
// [8] QCompleter * completer() const

/*
Returns the current QCompleter that provides completions.

This function was introduced in  Qt 4.2.

See also setCompleter().
*/
impl /*struct*/ QLineEdit {
  pub fn completer_0<RetType, T: QLineEdit_completer_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.completer_0(self);
    // return 1;
  }
}
pub trait QLineEdit_completer_0<RetType> {
  fn completer_0(self , rsthis: & QLineEdit) -> RetType;
}
impl<'a> /*trait*/ QLineEdit_completer_0<usize> for () {
  fn completer_0(self , rsthis: & QLineEdit) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QLineEdit9completerEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlineedit.h:131
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QSize sizeHint() const

/*
Reimplemented from QWidget::sizeHint().

Returns a recommended size for the widget.

The width returned, in pixels, is usually enough for about 15 to 20 characters.
*/
impl /*struct*/ QLineEdit {
  pub fn sizeHint_0<RetType, T: QLineEdit_sizeHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sizeHint_0(self);
    // return 1;
  }
}
pub trait QLineEdit_sizeHint_0<RetType> {
  fn sizeHint_0(self , rsthis: & QLineEdit) -> RetType;
}
impl<'a> /*trait*/ QLineEdit_sizeHint_0<usize> for () {
  fn sizeHint_0(self , rsthis: & QLineEdit) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QLineEdit8sizeHintEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlineedit.h:132
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QSize minimumSizeHint() const

/*
Reimplemented from QWidget::minimumSizeHint().

Returns a minimum size for the line edit.

The width returned is enough for at least one character.
*/
impl /*struct*/ QLineEdit {
  pub fn minimumSizeHint_0<RetType, T: QLineEdit_minimumSizeHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.minimumSizeHint_0(self);
    // return 1;
  }
}
pub trait QLineEdit_minimumSizeHint_0<RetType> {
  fn minimumSizeHint_0(self , rsthis: & QLineEdit) -> RetType;
}
impl<'a> /*trait*/ QLineEdit_minimumSizeHint_0<usize> for () {
  fn minimumSizeHint_0(self , rsthis: & QLineEdit) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QLineEdit15minimumSizeHintEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlineedit.h:134
// index:0
// Public Visibility=Default Availability=Available
// [4] int cursorPosition() const

/*

*/
impl /*struct*/ QLineEdit {
  pub fn cursorPosition_0<RetType, T: QLineEdit_cursorPosition_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cursorPosition_0(self);
    // return 1;
  }
}
pub trait QLineEdit_cursorPosition_0<RetType> {
  fn cursorPosition_0(self , rsthis: & QLineEdit) -> RetType;
}
impl<'a> /*trait*/ QLineEdit_cursorPosition_0<i32> for () {
  fn cursorPosition_0(self , rsthis: & QLineEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QLineEdit14cursorPositionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlineedit.h:135
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCursorPosition(int)

/*

*/
impl /*struct*/ QLineEdit {
  pub fn setCursorPosition_0<RetType, T: QLineEdit_setCursorPosition_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCursorPosition_0(self);
    // return 1;
  }
}
pub trait QLineEdit_setCursorPosition_0<RetType> {
  fn setCursorPosition_0(self , rsthis: & QLineEdit) -> RetType;
}
impl<'a> /*trait*/ QLineEdit_setCursorPosition_0<(/*void*/)> for (i32) {
  fn setCursorPosition_0(self , rsthis: & QLineEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QLineEdit17setCursorPositionEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlineedit.h:136
// index:0
// Public Visibility=Default Availability=Available
// [4] int cursorPositionAt(const QPoint &)

/*
Returns the cursor position under the point pos.
*/
impl /*struct*/ QLineEdit {
  pub fn cursorPositionAt_0<RetType, T: QLineEdit_cursorPositionAt_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cursorPositionAt_0(self);
    // return 1;
  }
}
pub trait QLineEdit_cursorPositionAt_0<RetType> {
  fn cursorPositionAt_0(self , rsthis: & QLineEdit) -> RetType;
}
impl<'a> /*trait*/ QLineEdit_cursorPositionAt_0<i32> for (usize) {
  fn cursorPositionAt_0(self , rsthis: & QLineEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QLineEdit16cursorPositionAtERK6QPoint", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlineedit.h:138
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setAlignment(Qt::Alignment)

/*

*/
impl /*struct*/ QLineEdit {
  pub fn setAlignment_0<RetType, T: QLineEdit_setAlignment_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setAlignment_0(self);
    // return 1;
  }
}
pub trait QLineEdit_setAlignment_0<RetType> {
  fn setAlignment_0(self , rsthis: & QLineEdit) -> RetType;
}
impl<'a> /*trait*/ QLineEdit_setAlignment_0<(/*void*/)> for (i32) {
  fn setAlignment_0(self , rsthis: & QLineEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QLineEdit12setAlignmentE6QFlagsIN2Qt13AlignmentFlagEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlineedit.h:139
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::Alignment alignment() const

/*

*/
impl /*struct*/ QLineEdit {
  pub fn alignment_0<RetType, T: QLineEdit_alignment_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.alignment_0(self);
    // return 1;
  }
}
pub trait QLineEdit_alignment_0<RetType> {
  fn alignment_0(self , rsthis: & QLineEdit) -> RetType;
}
impl<'a> /*trait*/ QLineEdit_alignment_0<i32> for () {
  fn alignment_0(self , rsthis: & QLineEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QLineEdit9alignmentEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlineedit.h:141
// index:0
// Public Visibility=Default Availability=Available
// [-2] void cursorForward(bool, int)

/*
Moves the cursor forward steps characters. If mark is true each character moved over is added to the selection; if mark is false the selection is cleared.

See also cursorBackward().
*/
impl /*struct*/ QLineEdit {
  pub fn cursorForward_0<RetType, T: QLineEdit_cursorForward_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cursorForward_0(self);
    // return 1;
  }
}
pub trait QLineEdit_cursorForward_0<RetType> {
  fn cursorForward_0(self , rsthis: & QLineEdit) -> RetType;
}
impl<'a> /*trait*/ QLineEdit_cursorForward_0<(/*void*/)> for (bool,i32) {
  fn cursorForward_0(self , rsthis: & QLineEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const bool as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QLineEdit13cursorForwardEbi", 2,qtrt::FFITY_SINT8,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlineedit.h:142
// index:0
// Public Visibility=Default Availability=Available
// [-2] void cursorBackward(bool, int)

/*
Moves the cursor back steps characters. If mark is true each character moved over is added to the selection; if mark is false the selection is cleared.

See also cursorForward().
*/
impl /*struct*/ QLineEdit {
  pub fn cursorBackward_0<RetType, T: QLineEdit_cursorBackward_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cursorBackward_0(self);
    // return 1;
  }
}
pub trait QLineEdit_cursorBackward_0<RetType> {
  fn cursorBackward_0(self , rsthis: & QLineEdit) -> RetType;
}
impl<'a> /*trait*/ QLineEdit_cursorBackward_0<(/*void*/)> for (bool,i32) {
  fn cursorBackward_0(self , rsthis: & QLineEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const bool as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QLineEdit14cursorBackwardEbi", 2,qtrt::FFITY_SINT8,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlineedit.h:143
// index:0
// Public Visibility=Default Availability=Available
// [-2] void cursorWordForward(bool)

/*
Moves the cursor one word forward. If mark is true, the word is also selected.

See also cursorWordBackward().
*/
impl /*struct*/ QLineEdit {
  pub fn cursorWordForward_0<RetType, T: QLineEdit_cursorWordForward_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cursorWordForward_0(self);
    // return 1;
  }
}
pub trait QLineEdit_cursorWordForward_0<RetType> {
  fn cursorWordForward_0(self , rsthis: & QLineEdit) -> RetType;
}
impl<'a> /*trait*/ QLineEdit_cursorWordForward_0<(/*void*/)> for (bool) {
  fn cursorWordForward_0(self , rsthis: & QLineEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN9QLineEdit17cursorWordForwardEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlineedit.h:144
// index:0
// Public Visibility=Default Availability=Available
// [-2] void cursorWordBackward(bool)

/*
Moves the cursor one word backward. If mark is true, the word is also selected.

See also cursorWordForward().
*/
impl /*struct*/ QLineEdit {
  pub fn cursorWordBackward_0<RetType, T: QLineEdit_cursorWordBackward_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cursorWordBackward_0(self);
    // return 1;
  }
}
pub trait QLineEdit_cursorWordBackward_0<RetType> {
  fn cursorWordBackward_0(self , rsthis: & QLineEdit) -> RetType;
}
impl<'a> /*trait*/ QLineEdit_cursorWordBackward_0<(/*void*/)> for (bool) {
  fn cursorWordBackward_0(self , rsthis: & QLineEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN9QLineEdit18cursorWordBackwardEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlineedit.h:145
// index:0
// Public Visibility=Default Availability=Available
// [-2] void backspace()

/*
If no text is selected, deletes the character to the left of the text cursor and moves the cursor one position to the left. If any text is selected, the cursor is moved to the beginning of the selected text and the selected text is deleted.

See also del().
*/
impl /*struct*/ QLineEdit {
  pub fn backspace_0<RetType, T: QLineEdit_backspace_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.backspace_0(self);
    // return 1;
  }
}
pub trait QLineEdit_backspace_0<RetType> {
  fn backspace_0(self , rsthis: & QLineEdit) -> RetType;
}
impl<'a> /*trait*/ QLineEdit_backspace_0<(/*void*/)> for () {
  fn backspace_0(self , rsthis: & QLineEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN9QLineEdit9backspaceEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlineedit.h:146
// index:0
// Public Visibility=Default Availability=Available
// [-2] void del()

/*
If no text is selected, deletes the character to the right of the text cursor. If any text is selected, the cursor is moved to the beginning of the selected text and the selected text is deleted.

See also backspace().
*/
impl /*struct*/ QLineEdit {
  pub fn del_0<RetType, T: QLineEdit_del_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.del_0(self);
    // return 1;
  }
}
pub trait QLineEdit_del_0<RetType> {
  fn del_0(self , rsthis: & QLineEdit) -> RetType;
}
impl<'a> /*trait*/ QLineEdit_del_0<(/*void*/)> for () {
  fn del_0(self , rsthis: & QLineEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN9QLineEdit3delEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlineedit.h:147
// index:0
// Public Visibility=Default Availability=Available
// [-2] void home(bool)

/*
Moves the text cursor to the beginning of the line unless it is already there. If mark is true, text is selected towards the first position; otherwise, any selected text is unselected if the cursor is moved.

See also end().
*/
impl /*struct*/ QLineEdit {
  pub fn home_0<RetType, T: QLineEdit_home_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.home_0(self);
    // return 1;
  }
}
pub trait QLineEdit_home_0<RetType> {
  fn home_0(self , rsthis: & QLineEdit) -> RetType;
}
impl<'a> /*trait*/ QLineEdit_home_0<(/*void*/)> for (bool) {
  fn home_0(self , rsthis: & QLineEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN9QLineEdit4homeEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlineedit.h:148
// index:0
// Public Visibility=Default Availability=Available
// [-2] void end(bool)

/*
Moves the text cursor to the end of the line unless it is already there. If mark is true, text is selected towards the last position; otherwise, any selected text is unselected if the cursor is moved.

See also home().
*/
impl /*struct*/ QLineEdit {
  pub fn end_0<RetType, T: QLineEdit_end_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.end_0(self);
    // return 1;
  }
}
pub trait QLineEdit_end_0<RetType> {
  fn end_0(self , rsthis: & QLineEdit) -> RetType;
}
impl<'a> /*trait*/ QLineEdit_end_0<(/*void*/)> for (bool) {
  fn end_0(self , rsthis: & QLineEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN9QLineEdit3endEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlineedit.h:150
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isModified() const

/*

*/
impl /*struct*/ QLineEdit {
  pub fn isModified_0<RetType, T: QLineEdit_isModified_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isModified_0(self);
    // return 1;
  }
}
pub trait QLineEdit_isModified_0<RetType> {
  fn isModified_0(self , rsthis: & QLineEdit) -> RetType;
}
impl<'a> /*trait*/ QLineEdit_isModified_0<bool> for () {
  fn isModified_0(self , rsthis: & QLineEdit) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QLineEdit10isModifiedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlineedit.h:151
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setModified(bool)

/*

*/
impl /*struct*/ QLineEdit {
  pub fn setModified_0<RetType, T: QLineEdit_setModified_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setModified_0(self);
    // return 1;
  }
}
pub trait QLineEdit_setModified_0<RetType> {
  fn setModified_0(self , rsthis: & QLineEdit) -> RetType;
}
impl<'a> /*trait*/ QLineEdit_setModified_0<(/*void*/)> for (bool) {
  fn setModified_0(self , rsthis: & QLineEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN9QLineEdit11setModifiedEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlineedit.h:153
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setSelection(int, int)

/*
Selects text from position start and for length characters. Negative lengths are allowed.

See also deselect(), selectAll(), and selectedText().
*/
impl /*struct*/ QLineEdit {
  pub fn setSelection_0<RetType, T: QLineEdit_setSelection_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSelection_0(self);
    // return 1;
  }
}
pub trait QLineEdit_setSelection_0<RetType> {
  fn setSelection_0(self , rsthis: & QLineEdit) -> RetType;
}
impl<'a> /*trait*/ QLineEdit_setSelection_0<(/*void*/)> for (i32,i32) {
  fn setSelection_0(self , rsthis: & QLineEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QLineEdit12setSelectionEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlineedit.h:154
// index:0
// Public Visibility=Default Availability=Available
// [1] bool hasSelectedText() const

/*

*/
impl /*struct*/ QLineEdit {
  pub fn hasSelectedText_0<RetType, T: QLineEdit_hasSelectedText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hasSelectedText_0(self);
    // return 1;
  }
}
pub trait QLineEdit_hasSelectedText_0<RetType> {
  fn hasSelectedText_0(self , rsthis: & QLineEdit) -> RetType;
}
impl<'a> /*trait*/ QLineEdit_hasSelectedText_0<bool> for () {
  fn hasSelectedText_0(self , rsthis: & QLineEdit) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QLineEdit15hasSelectedTextEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlineedit.h:155
// index:0
// Public Visibility=Default Availability=Available
// [8] QString selectedText() const

/*

*/
impl /*struct*/ QLineEdit {
  pub fn selectedText_0<RetType, T: QLineEdit_selectedText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.selectedText_0(self);
    // return 1;
  }
}
pub trait QLineEdit_selectedText_0<RetType> {
  fn selectedText_0(self , rsthis: & QLineEdit) -> RetType;
}
impl<'a> /*trait*/ QLineEdit_selectedText_0<usize> for () {
  fn selectedText_0(self , rsthis: & QLineEdit) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QLineEdit12selectedTextEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlineedit.h:156
// index:0
// Public Visibility=Default Availability=Available
// [4] int selectionStart() const

/*
Returns the index of the first selected character in the line edit or -1 if no text is selected.

See also selectedText(), selectionEnd(), and selectionLength().
*/
impl /*struct*/ QLineEdit {
  pub fn selectionStart_0<RetType, T: QLineEdit_selectionStart_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.selectionStart_0(self);
    // return 1;
  }
}
pub trait QLineEdit_selectionStart_0<RetType> {
  fn selectionStart_0(self , rsthis: & QLineEdit) -> RetType;
}
impl<'a> /*trait*/ QLineEdit_selectionStart_0<i32> for () {
  fn selectionStart_0(self , rsthis: & QLineEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QLineEdit14selectionStartEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlineedit.h:157
// index:0
// Public Visibility=Default Availability=Available
// [4] int selectionEnd() const

/*
Returns the index of the character directly after the selection in the line edit or -1 if no text is selected.

This function was introduced in  Qt 5.10.

See also selectedText(), selectionStart(), and selectionLength().
*/
impl /*struct*/ QLineEdit {
  pub fn selectionEnd_0<RetType, T: QLineEdit_selectionEnd_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.selectionEnd_0(self);
    // return 1;
  }
}
pub trait QLineEdit_selectionEnd_0<RetType> {
  fn selectionEnd_0(self , rsthis: & QLineEdit) -> RetType;
}
impl<'a> /*trait*/ QLineEdit_selectionEnd_0<i32> for () {
  fn selectionEnd_0(self , rsthis: & QLineEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QLineEdit12selectionEndEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlineedit.h:158
// index:0
// Public Visibility=Default Availability=Available
// [4] int selectionLength() const

/*
Returns the length of the selection.

This function was introduced in  Qt 5.10.

See also selectedText(), selectionStart(), and selectionEnd().
*/
impl /*struct*/ QLineEdit {
  pub fn selectionLength_0<RetType, T: QLineEdit_selectionLength_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.selectionLength_0(self);
    // return 1;
  }
}
pub trait QLineEdit_selectionLength_0<RetType> {
  fn selectionLength_0(self , rsthis: & QLineEdit) -> RetType;
}
impl<'a> /*trait*/ QLineEdit_selectionLength_0<i32> for () {
  fn selectionLength_0(self , rsthis: & QLineEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QLineEdit15selectionLengthEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlineedit.h:160
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isUndoAvailable() const

/*

*/
impl /*struct*/ QLineEdit {
  pub fn isUndoAvailable_0<RetType, T: QLineEdit_isUndoAvailable_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isUndoAvailable_0(self);
    // return 1;
  }
}
pub trait QLineEdit_isUndoAvailable_0<RetType> {
  fn isUndoAvailable_0(self , rsthis: & QLineEdit) -> RetType;
}
impl<'a> /*trait*/ QLineEdit_isUndoAvailable_0<bool> for () {
  fn isUndoAvailable_0(self , rsthis: & QLineEdit) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QLineEdit15isUndoAvailableEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlineedit.h:161
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isRedoAvailable() const

/*

*/
impl /*struct*/ QLineEdit {
  pub fn isRedoAvailable_0<RetType, T: QLineEdit_isRedoAvailable_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isRedoAvailable_0(self);
    // return 1;
  }
}
pub trait QLineEdit_isRedoAvailable_0<RetType> {
  fn isRedoAvailable_0(self , rsthis: & QLineEdit) -> RetType;
}
impl<'a> /*trait*/ QLineEdit_isRedoAvailable_0<bool> for () {
  fn isRedoAvailable_0(self , rsthis: & QLineEdit) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QLineEdit15isRedoAvailableEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlineedit.h:163
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDragEnabled(bool)

/*

*/
impl /*struct*/ QLineEdit {
  pub fn setDragEnabled_0<RetType, T: QLineEdit_setDragEnabled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDragEnabled_0(self);
    // return 1;
  }
}
pub trait QLineEdit_setDragEnabled_0<RetType> {
  fn setDragEnabled_0(self , rsthis: & QLineEdit) -> RetType;
}
impl<'a> /*trait*/ QLineEdit_setDragEnabled_0<(/*void*/)> for (bool) {
  fn setDragEnabled_0(self , rsthis: & QLineEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN9QLineEdit14setDragEnabledEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlineedit.h:164
// index:0
// Public Visibility=Default Availability=Available
// [1] bool dragEnabled() const

/*

*/
impl /*struct*/ QLineEdit {
  pub fn dragEnabled_0<RetType, T: QLineEdit_dragEnabled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dragEnabled_0(self);
    // return 1;
  }
}
pub trait QLineEdit_dragEnabled_0<RetType> {
  fn dragEnabled_0(self , rsthis: & QLineEdit) -> RetType;
}
impl<'a> /*trait*/ QLineEdit_dragEnabled_0<bool> for () {
  fn dragEnabled_0(self , rsthis: & QLineEdit) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QLineEdit11dragEnabledEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlineedit.h:166
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCursorMoveStyle(Qt::CursorMoveStyle)

/*

*/
impl /*struct*/ QLineEdit {
  pub fn setCursorMoveStyle_0<RetType, T: QLineEdit_setCursorMoveStyle_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCursorMoveStyle_0(self);
    // return 1;
  }
}
pub trait QLineEdit_setCursorMoveStyle_0<RetType> {
  fn setCursorMoveStyle_0(self , rsthis: & QLineEdit) -> RetType;
}
impl<'a> /*trait*/ QLineEdit_setCursorMoveStyle_0<(/*void*/)> for (i32) {
  fn setCursorMoveStyle_0(self , rsthis: & QLineEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QLineEdit18setCursorMoveStyleEN2Qt15CursorMoveStyleE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlineedit.h:167
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::CursorMoveStyle cursorMoveStyle() const

/*

*/
impl /*struct*/ QLineEdit {
  pub fn cursorMoveStyle_0<RetType, T: QLineEdit_cursorMoveStyle_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cursorMoveStyle_0(self);
    // return 1;
  }
}
pub trait QLineEdit_cursorMoveStyle_0<RetType> {
  fn cursorMoveStyle_0(self , rsthis: & QLineEdit) -> RetType;
}
impl<'a> /*trait*/ QLineEdit_cursorMoveStyle_0<i32> for () {
  fn cursorMoveStyle_0(self , rsthis: & QLineEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QLineEdit15cursorMoveStyleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlineedit.h:169
// index:0
// Public Visibility=Default Availability=Available
// [8] QString inputMask() const

/*

*/
impl /*struct*/ QLineEdit {
  pub fn inputMask_0<RetType, T: QLineEdit_inputMask_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.inputMask_0(self);
    // return 1;
  }
}
pub trait QLineEdit_inputMask_0<RetType> {
  fn inputMask_0(self , rsthis: & QLineEdit) -> RetType;
}
impl<'a> /*trait*/ QLineEdit_inputMask_0<usize> for () {
  fn inputMask_0(self , rsthis: & QLineEdit) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QLineEdit9inputMaskEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlineedit.h:170
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setInputMask(const QString &)

/*

*/
impl /*struct*/ QLineEdit {
  pub fn setInputMask_0<RetType, T: QLineEdit_setInputMask_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setInputMask_0(self);
    // return 1;
  }
}
pub trait QLineEdit_setInputMask_0<RetType> {
  fn setInputMask_0(self , rsthis: & QLineEdit) -> RetType;
}
impl<'a> /*trait*/ QLineEdit_setInputMask_0<(/*void*/)> for (usize) {
  fn setInputMask_0(self , rsthis: & QLineEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QLineEdit12setInputMaskERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlineedit.h:171
// index:0
// Public Visibility=Default Availability=Available
// [1] bool hasAcceptableInput() const

/*

*/
impl /*struct*/ QLineEdit {
  pub fn hasAcceptableInput_0<RetType, T: QLineEdit_hasAcceptableInput_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hasAcceptableInput_0(self);
    // return 1;
  }
}
pub trait QLineEdit_hasAcceptableInput_0<RetType> {
  fn hasAcceptableInput_0(self , rsthis: & QLineEdit) -> RetType;
}
impl<'a> /*trait*/ QLineEdit_hasAcceptableInput_0<bool> for () {
  fn hasAcceptableInput_0(self , rsthis: & QLineEdit) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QLineEdit18hasAcceptableInputEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlineedit.h:173
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTextMargins(int, int, int, int)

/*
Sets the margins around the text inside the frame to have the sizes left, top, right, and bottom.

See also getTextMargins().

This function was introduced in  Qt 4.5.

See also textMargins().
*/
impl /*struct*/ QLineEdit {
  pub fn setTextMargins_0<RetType, T: QLineEdit_setTextMargins_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTextMargins_0(self);
    // return 1;
  }
}
pub trait QLineEdit_setTextMargins_0<RetType> {
  fn setTextMargins_0(self , rsthis: & QLineEdit) -> RetType;
}
impl<'a> /*trait*/ QLineEdit_setTextMargins_0<(/*void*/)> for (i32,i32,i32,i32) {
  fn setTextMargins_0(self , rsthis: & QLineEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QLineEdit14setTextMarginsEiiii", 4,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlineedit.h:174
// index:1
// Public Visibility=Default Availability=Available
// [-2] void setTextMargins(const QMargins &)

/*
Sets the margins around the text inside the frame to have the sizes left, top, right, and bottom.

See also getTextMargins().

This function was introduced in  Qt 4.5.

See also textMargins().
*/
impl /*struct*/ QLineEdit {
  pub fn setTextMargins_1<RetType, T: QLineEdit_setTextMargins_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTextMargins_1(self);
    // return 1;
  }
}
pub trait QLineEdit_setTextMargins_1<RetType> {
  fn setTextMargins_1(self , rsthis: & QLineEdit) -> RetType;
}
impl<'a> /*trait*/ QLineEdit_setTextMargins_1<(/*void*/)> for (usize) {
  fn setTextMargins_1(self , rsthis: & QLineEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QLineEdit14setTextMarginsERK8QMargins", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlineedit.h:175
// index:0
// Public Visibility=Default Availability=Available
// [-2] void getTextMargins(int *, int *, int *, int *) const

/*
Returns the widget's text margins for left, top, right, and bottom.

This function was introduced in  Qt 4.5.

See also setTextMargins().
*/
impl /*struct*/ QLineEdit {
  pub fn getTextMargins_0<RetType, T: QLineEdit_getTextMargins_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.getTextMargins_0(self);
    // return 1;
  }
}
pub trait QLineEdit_getTextMargins_0<RetType> {
  fn getTextMargins_0(self , rsthis: & QLineEdit) -> RetType;
}
impl<'a> /*trait*/ QLineEdit_getTextMargins_0<(/*void*/)> for (usize,usize,usize,usize) {
  fn getTextMargins_0(self , rsthis: & QLineEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const usize as usize;
    let arg2 = (&self.2) as *const usize as usize;
    let arg3 = (&self.3) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK9QLineEdit14getTextMarginsEPiS0_S0_S0_", 4,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlineedit.h:176
// index:0
// Public Visibility=Default Availability=Available
// [16] QMargins textMargins() const

/*
Returns the widget's text margins.

This function was introduced in  Qt 4.6.

See also setTextMargins().
*/
impl /*struct*/ QLineEdit {
  pub fn textMargins_0<RetType, T: QLineEdit_textMargins_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.textMargins_0(self);
    // return 1;
  }
}
pub trait QLineEdit_textMargins_0<RetType> {
  fn textMargins_0(self , rsthis: & QLineEdit) -> RetType;
}
impl<'a> /*trait*/ QLineEdit_textMargins_0<usize> for () {
  fn textMargins_0(self , rsthis: & QLineEdit) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QLineEdit11textMarginsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlineedit.h:180
// index:0
// Public Visibility=Default Availability=Available
// [-2] void addAction(QAction *, QLineEdit::ActionPosition)

/*
This is an overloaded function.

Adds the action to the list of actions at the position.

This function was introduced in  Qt 5.2.
*/
impl /*struct*/ QLineEdit {
  pub fn addAction_0<RetType, T: QLineEdit_addAction_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addAction_0(self);
    // return 1;
  }
}
pub trait QLineEdit_addAction_0<RetType> {
  fn addAction_0(self , rsthis: & QLineEdit) -> RetType;
}
impl<'a> /*trait*/ QLineEdit_addAction_0<(/*void*/)> for (usize,i32) {
  fn addAction_0(self , rsthis: & QLineEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QLineEdit9addActionEP7QActionNS_14ActionPositionE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlineedit.h:181
// index:1
// Public Visibility=Default Availability=Available
// [8] QAction * addAction(const QIcon &, QLineEdit::ActionPosition)

/*
This is an overloaded function.

Adds the action to the list of actions at the position.

This function was introduced in  Qt 5.2.
*/
impl /*struct*/ QLineEdit {
  pub fn addAction_1<RetType, T: QLineEdit_addAction_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addAction_1(self);
    // return 1;
  }
}
pub trait QLineEdit_addAction_1<RetType> {
  fn addAction_1(self , rsthis: & QLineEdit) -> RetType;
}
impl<'a> /*trait*/ QLineEdit_addAction_1<usize> for (usize,i32) {
  fn addAction_1(self , rsthis: & QLineEdit) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QLineEdit9addActionERK5QIconNS_14ActionPositionE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlineedit.h:185
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setText(const QString &)

/*

*/
impl /*struct*/ QLineEdit {
  pub fn setText_0<RetType, T: QLineEdit_setText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setText_0(self);
    // return 1;
  }
}
pub trait QLineEdit_setText_0<RetType> {
  fn setText_0(self , rsthis: & QLineEdit) -> RetType;
}
impl<'a> /*trait*/ QLineEdit_setText_0<(/*void*/)> for (usize) {
  fn setText_0(self , rsthis: & QLineEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QLineEdit7setTextERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlineedit.h:186
// index:0
// Public Visibility=Default Availability=Available
// [-2] void clear()

/*
Clears the contents of the line edit.

See also setText() and insert().
*/
impl /*struct*/ QLineEdit {
  pub fn clear_0<RetType, T: QLineEdit_clear_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.clear_0(self);
    // return 1;
  }
}
pub trait QLineEdit_clear_0<RetType> {
  fn clear_0(self , rsthis: & QLineEdit) -> RetType;
}
impl<'a> /*trait*/ QLineEdit_clear_0<(/*void*/)> for () {
  fn clear_0(self , rsthis: & QLineEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN9QLineEdit5clearEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlineedit.h:187
// index:0
// Public Visibility=Default Availability=Available
// [-2] void selectAll()

/*
Selects all the text (i.e. highlights it) and moves the cursor to the end. This is useful when a default value has been inserted because if the user types before clicking on the widget, the selected text will be deleted.

See also setSelection() and deselect().
*/
impl /*struct*/ QLineEdit {
  pub fn selectAll_0<RetType, T: QLineEdit_selectAll_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.selectAll_0(self);
    // return 1;
  }
}
pub trait QLineEdit_selectAll_0<RetType> {
  fn selectAll_0(self , rsthis: & QLineEdit) -> RetType;
}
impl<'a> /*trait*/ QLineEdit_selectAll_0<(/*void*/)> for () {
  fn selectAll_0(self , rsthis: & QLineEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN9QLineEdit9selectAllEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlineedit.h:188
// index:0
// Public Visibility=Default Availability=Available
// [-2] void undo()

/*
Undoes the last operation if undo is available. Deselects any current selection, and updates the selection start to the current cursor position.
*/
impl /*struct*/ QLineEdit {
  pub fn undo_0<RetType, T: QLineEdit_undo_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.undo_0(self);
    // return 1;
  }
}
pub trait QLineEdit_undo_0<RetType> {
  fn undo_0(self , rsthis: & QLineEdit) -> RetType;
}
impl<'a> /*trait*/ QLineEdit_undo_0<(/*void*/)> for () {
  fn undo_0(self , rsthis: & QLineEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN9QLineEdit4undoEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlineedit.h:189
// index:0
// Public Visibility=Default Availability=Available
// [-2] void redo()

/*
Redoes the last operation if redo is available.
*/
impl /*struct*/ QLineEdit {
  pub fn redo_0<RetType, T: QLineEdit_redo_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.redo_0(self);
    // return 1;
  }
}
pub trait QLineEdit_redo_0<RetType> {
  fn redo_0(self , rsthis: & QLineEdit) -> RetType;
}
impl<'a> /*trait*/ QLineEdit_redo_0<(/*void*/)> for () {
  fn redo_0(self , rsthis: & QLineEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN9QLineEdit4redoEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlineedit.h:191
// index:0
// Public Visibility=Default Availability=Available
// [-2] void cut()

/*
Copies the selected text to the clipboard and deletes it, if there is any, and if echoMode() is Normal.

If the current validator disallows deleting the selected text, cut() will copy without deleting.

See also copy(), paste(), and setValidator().
*/
impl /*struct*/ QLineEdit {
  pub fn cut_0<RetType, T: QLineEdit_cut_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cut_0(self);
    // return 1;
  }
}
pub trait QLineEdit_cut_0<RetType> {
  fn cut_0(self , rsthis: & QLineEdit) -> RetType;
}
impl<'a> /*trait*/ QLineEdit_cut_0<(/*void*/)> for () {
  fn cut_0(self , rsthis: & QLineEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN9QLineEdit3cutEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlineedit.h:192
// index:0
// Public Visibility=Default Availability=Available
// [-2] void copy() const

/*
Copies the selected text to the clipboard, if there is any, and if echoMode() is Normal.

See also cut() and paste().
*/
impl /*struct*/ QLineEdit {
  pub fn copy_0<RetType, T: QLineEdit_copy_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.copy_0(self);
    // return 1;
  }
}
pub trait QLineEdit_copy_0<RetType> {
  fn copy_0(self , rsthis: & QLineEdit) -> RetType;
}
impl<'a> /*trait*/ QLineEdit_copy_0<(/*void*/)> for () {
  fn copy_0(self , rsthis: & QLineEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZNK9QLineEdit4copyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlineedit.h:193
// index:0
// Public Visibility=Default Availability=Available
// [-2] void paste()

/*
Inserts the clipboard's text at the cursor position, deleting any selected text, providing the line edit is not read-only.

If the end result would not be acceptable to the current validator, nothing happens.

See also copy() and cut().
*/
impl /*struct*/ QLineEdit {
  pub fn paste_0<RetType, T: QLineEdit_paste_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.paste_0(self);
    // return 1;
  }
}
pub trait QLineEdit_paste_0<RetType> {
  fn paste_0(self , rsthis: & QLineEdit) -> RetType;
}
impl<'a> /*trait*/ QLineEdit_paste_0<(/*void*/)> for () {
  fn paste_0(self , rsthis: & QLineEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN9QLineEdit5pasteEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlineedit.h:197
// index:0
// Public Visibility=Default Availability=Available
// [-2] void deselect()

/*
Deselects any selected text.

See also setSelection() and selectAll().
*/
impl /*struct*/ QLineEdit {
  pub fn deselect_0<RetType, T: QLineEdit_deselect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.deselect_0(self);
    // return 1;
  }
}
pub trait QLineEdit_deselect_0<RetType> {
  fn deselect_0(self , rsthis: & QLineEdit) -> RetType;
}
impl<'a> /*trait*/ QLineEdit_deselect_0<(/*void*/)> for () {
  fn deselect_0(self , rsthis: & QLineEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN9QLineEdit8deselectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlineedit.h:200
// index:0
// Public Visibility=Default Availability=Available
// [8] QMenu * createStandardContextMenu()

/*
This function creates the standard context menu which is shown when the user clicks on the line edit with the right mouse button. It is called from the default contextMenuEvent() handler. The popup menu's ownership is transferred to the caller.
*/
impl /*struct*/ QLineEdit {
  pub fn createStandardContextMenu_0<RetType, T: QLineEdit_createStandardContextMenu_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.createStandardContextMenu_0(self);
    // return 1;
  }
}
pub trait QLineEdit_createStandardContextMenu_0<RetType> {
  fn createStandardContextMenu_0(self , rsthis: & QLineEdit) -> RetType;
}
impl<'a> /*trait*/ QLineEdit_createStandardContextMenu_0<usize> for () {
  fn createStandardContextMenu_0(self , rsthis: & QLineEdit) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QLineEdit25createStandardContextMenuEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlineedit.h:204
// index:0
// Public Visibility=Default Availability=Available
// [-2] void textChanged(const QString &)

/*
This signal is emitted whenever the text changes. The text argument is the new text.

Unlike textEdited(), this signal is also emitted when the text is changed programmatically, for example, by calling setText().

Note: Notifier signal for property text.
*/
impl /*struct*/ QLineEdit {
  pub fn textChanged_0<RetType, T: QLineEdit_textChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.textChanged_0(self);
    // return 1;
  }
}
pub trait QLineEdit_textChanged_0<RetType> {
  fn textChanged_0(self , rsthis: & QLineEdit) -> RetType;
}
impl<'a> /*trait*/ QLineEdit_textChanged_0<(/*void*/)> for (usize) {
  fn textChanged_0(self , rsthis: & QLineEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QLineEdit11textChangedERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlineedit.h:205
// index:0
// Public Visibility=Default Availability=Available
// [-2] void textEdited(const QString &)

/*
This signal is emitted whenever the text is edited. The text argument is the new text.

Unlike textChanged(), this signal is not emitted when the text is changed programmatically, for example, by calling setText().
*/
impl /*struct*/ QLineEdit {
  pub fn textEdited_0<RetType, T: QLineEdit_textEdited_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.textEdited_0(self);
    // return 1;
  }
}
pub trait QLineEdit_textEdited_0<RetType> {
  fn textEdited_0(self , rsthis: & QLineEdit) -> RetType;
}
impl<'a> /*trait*/ QLineEdit_textEdited_0<(/*void*/)> for (usize) {
  fn textEdited_0(self , rsthis: & QLineEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QLineEdit10textEditedERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlineedit.h:206
// index:0
// Public Visibility=Default Availability=Available
// [-2] void cursorPositionChanged(int, int)

/*
This signal is emitted whenever the cursor moves. The previous position is given by old, and the new position by new.

See also setCursorPosition() and cursorPosition().
*/
impl /*struct*/ QLineEdit {
  pub fn cursorPositionChanged_0<RetType, T: QLineEdit_cursorPositionChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cursorPositionChanged_0(self);
    // return 1;
  }
}
pub trait QLineEdit_cursorPositionChanged_0<RetType> {
  fn cursorPositionChanged_0(self , rsthis: & QLineEdit) -> RetType;
}
impl<'a> /*trait*/ QLineEdit_cursorPositionChanged_0<(/*void*/)> for (i32,i32) {
  fn cursorPositionChanged_0(self , rsthis: & QLineEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QLineEdit21cursorPositionChangedEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlineedit.h:207
// index:0
// Public Visibility=Default Availability=Available
// [-2] void returnPressed()

/*
This signal is emitted when the Return or Enter key is pressed. Note that if there is a validator() or inputMask() set on the line edit, the returnPressed() signal will only be emitted if the input follows the inputMask() and the validator() returns QValidator::Acceptable.
*/
impl /*struct*/ QLineEdit {
  pub fn returnPressed_0<RetType, T: QLineEdit_returnPressed_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.returnPressed_0(self);
    // return 1;
  }
}
pub trait QLineEdit_returnPressed_0<RetType> {
  fn returnPressed_0(self , rsthis: & QLineEdit) -> RetType;
}
impl<'a> /*trait*/ QLineEdit_returnPressed_0<(/*void*/)> for () {
  fn returnPressed_0(self , rsthis: & QLineEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN9QLineEdit13returnPressedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlineedit.h:208
// index:0
// Public Visibility=Default Availability=Available
// [-2] void editingFinished()

/*
This signal is emitted when the Return or Enter key is pressed or the line edit loses focus. Note that if there is a validator() or inputMask() set on the line edit and enter/return is pressed, the editingFinished() signal will only be emitted if the input follows the inputMask() and the validator() returns QValidator::Acceptable.
*/
impl /*struct*/ QLineEdit {
  pub fn editingFinished_0<RetType, T: QLineEdit_editingFinished_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.editingFinished_0(self);
    // return 1;
  }
}
pub trait QLineEdit_editingFinished_0<RetType> {
  fn editingFinished_0(self , rsthis: & QLineEdit) -> RetType;
}
impl<'a> /*trait*/ QLineEdit_editingFinished_0<(/*void*/)> for () {
  fn editingFinished_0(self , rsthis: & QLineEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN9QLineEdit15editingFinishedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlineedit.h:209
// index:0
// Public Visibility=Default Availability=Available
// [-2] void selectionChanged()

/*
This signal is emitted whenever the selection changes.

See also hasSelectedText() and selectedText().
*/
impl /*struct*/ QLineEdit {
  pub fn selectionChanged_0<RetType, T: QLineEdit_selectionChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.selectionChanged_0(self);
    // return 1;
  }
}
pub trait QLineEdit_selectionChanged_0<RetType> {
  fn selectionChanged_0(self , rsthis: & QLineEdit) -> RetType;
}
impl<'a> /*trait*/ QLineEdit_selectionChanged_0<(/*void*/)> for () {
  fn selectionChanged_0(self , rsthis: & QLineEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN9QLineEdit16selectionChangedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlineedit.h:212
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mousePressEvent(QMouseEvent *)

/*
Reimplemented from QWidget::mousePressEvent().
*/
impl /*struct*/ QLineEdit {
  pub fn mousePressEvent_0<RetType, T: QLineEdit_mousePressEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mousePressEvent_0(self);
    // return 1;
  }
}
pub trait QLineEdit_mousePressEvent_0<RetType> {
  fn mousePressEvent_0(self , rsthis: & QLineEdit) -> RetType;
}
impl<'a> /*trait*/ QLineEdit_mousePressEvent_0<(/*void*/)> for (usize) {
  fn mousePressEvent_0(self , rsthis: & QLineEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QLineEdit15mousePressEventEP11QMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlineedit.h:213
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mouseMoveEvent(QMouseEvent *)

/*
Reimplemented from QWidget::mouseMoveEvent().
*/
impl /*struct*/ QLineEdit {
  pub fn mouseMoveEvent_0<RetType, T: QLineEdit_mouseMoveEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mouseMoveEvent_0(self);
    // return 1;
  }
}
pub trait QLineEdit_mouseMoveEvent_0<RetType> {
  fn mouseMoveEvent_0(self , rsthis: & QLineEdit) -> RetType;
}
impl<'a> /*trait*/ QLineEdit_mouseMoveEvent_0<(/*void*/)> for (usize) {
  fn mouseMoveEvent_0(self , rsthis: & QLineEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QLineEdit14mouseMoveEventEP11QMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlineedit.h:214
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mouseReleaseEvent(QMouseEvent *)

/*
Reimplemented from QWidget::mouseReleaseEvent().
*/
impl /*struct*/ QLineEdit {
  pub fn mouseReleaseEvent_0<RetType, T: QLineEdit_mouseReleaseEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mouseReleaseEvent_0(self);
    // return 1;
  }
}
pub trait QLineEdit_mouseReleaseEvent_0<RetType> {
  fn mouseReleaseEvent_0(self , rsthis: & QLineEdit) -> RetType;
}
impl<'a> /*trait*/ QLineEdit_mouseReleaseEvent_0<(/*void*/)> for (usize) {
  fn mouseReleaseEvent_0(self , rsthis: & QLineEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QLineEdit17mouseReleaseEventEP11QMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlineedit.h:215
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mouseDoubleClickEvent(QMouseEvent *)

/*
Reimplemented from QWidget::mouseDoubleClickEvent().
*/
impl /*struct*/ QLineEdit {
  pub fn mouseDoubleClickEvent_0<RetType, T: QLineEdit_mouseDoubleClickEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mouseDoubleClickEvent_0(self);
    // return 1;
  }
}
pub trait QLineEdit_mouseDoubleClickEvent_0<RetType> {
  fn mouseDoubleClickEvent_0(self , rsthis: & QLineEdit) -> RetType;
}
impl<'a> /*trait*/ QLineEdit_mouseDoubleClickEvent_0<(/*void*/)> for (usize) {
  fn mouseDoubleClickEvent_0(self , rsthis: & QLineEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QLineEdit21mouseDoubleClickEventEP11QMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlineedit.h:216
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void keyPressEvent(QKeyEvent *)

/*
Reimplemented from QWidget::keyPressEvent().

Converts the given key press event into a line edit action.

If Return or Enter is pressed and the current text is valid (or can be made valid by the validator), the signal returnPressed() is emitted.

The default key bindings are listed in the class's detailed description.
*/
impl /*struct*/ QLineEdit {
  pub fn keyPressEvent_0<RetType, T: QLineEdit_keyPressEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.keyPressEvent_0(self);
    // return 1;
  }
}
pub trait QLineEdit_keyPressEvent_0<RetType> {
  fn keyPressEvent_0(self , rsthis: & QLineEdit) -> RetType;
}
impl<'a> /*trait*/ QLineEdit_keyPressEvent_0<(/*void*/)> for (usize) {
  fn keyPressEvent_0(self , rsthis: & QLineEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QLineEdit13keyPressEventEP9QKeyEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlineedit.h:217
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void focusInEvent(QFocusEvent *)

/*
Reimplemented from QWidget::focusInEvent().
*/
impl /*struct*/ QLineEdit {
  pub fn focusInEvent_0<RetType, T: QLineEdit_focusInEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.focusInEvent_0(self);
    // return 1;
  }
}
pub trait QLineEdit_focusInEvent_0<RetType> {
  fn focusInEvent_0(self , rsthis: & QLineEdit) -> RetType;
}
impl<'a> /*trait*/ QLineEdit_focusInEvent_0<(/*void*/)> for (usize) {
  fn focusInEvent_0(self , rsthis: & QLineEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QLineEdit12focusInEventEP11QFocusEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlineedit.h:218
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void focusOutEvent(QFocusEvent *)

/*
Reimplemented from QWidget::focusOutEvent().
*/
impl /*struct*/ QLineEdit {
  pub fn focusOutEvent_0<RetType, T: QLineEdit_focusOutEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.focusOutEvent_0(self);
    // return 1;
  }
}
pub trait QLineEdit_focusOutEvent_0<RetType> {
  fn focusOutEvent_0(self , rsthis: & QLineEdit) -> RetType;
}
impl<'a> /*trait*/ QLineEdit_focusOutEvent_0<(/*void*/)> for (usize) {
  fn focusOutEvent_0(self , rsthis: & QLineEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QLineEdit13focusOutEventEP11QFocusEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlineedit.h:219
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void paintEvent(QPaintEvent *)

/*
Reimplemented from QWidget::paintEvent().
*/
impl /*struct*/ QLineEdit {
  pub fn paintEvent_0<RetType, T: QLineEdit_paintEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.paintEvent_0(self);
    // return 1;
  }
}
pub trait QLineEdit_paintEvent_0<RetType> {
  fn paintEvent_0(self , rsthis: & QLineEdit) -> RetType;
}
impl<'a> /*trait*/ QLineEdit_paintEvent_0<(/*void*/)> for (usize) {
  fn paintEvent_0(self , rsthis: & QLineEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QLineEdit10paintEventEP11QPaintEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlineedit.h:221
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void dragEnterEvent(QDragEnterEvent *)

/*
Reimplemented from QWidget::dragEnterEvent().
*/
impl /*struct*/ QLineEdit {
  pub fn dragEnterEvent_0<RetType, T: QLineEdit_dragEnterEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dragEnterEvent_0(self);
    // return 1;
  }
}
pub trait QLineEdit_dragEnterEvent_0<RetType> {
  fn dragEnterEvent_0(self , rsthis: & QLineEdit) -> RetType;
}
impl<'a> /*trait*/ QLineEdit_dragEnterEvent_0<(/*void*/)> for (usize) {
  fn dragEnterEvent_0(self , rsthis: & QLineEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QLineEdit14dragEnterEventEP15QDragEnterEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlineedit.h:222
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void dragMoveEvent(QDragMoveEvent *)

/*
Reimplemented from QWidget::dragMoveEvent().
*/
impl /*struct*/ QLineEdit {
  pub fn dragMoveEvent_0<RetType, T: QLineEdit_dragMoveEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dragMoveEvent_0(self);
    // return 1;
  }
}
pub trait QLineEdit_dragMoveEvent_0<RetType> {
  fn dragMoveEvent_0(self , rsthis: & QLineEdit) -> RetType;
}
impl<'a> /*trait*/ QLineEdit_dragMoveEvent_0<(/*void*/)> for (usize) {
  fn dragMoveEvent_0(self , rsthis: & QLineEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QLineEdit13dragMoveEventEP14QDragMoveEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlineedit.h:223
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void dragLeaveEvent(QDragLeaveEvent *)

/*
Reimplemented from QWidget::dragLeaveEvent().
*/
impl /*struct*/ QLineEdit {
  pub fn dragLeaveEvent_0<RetType, T: QLineEdit_dragLeaveEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dragLeaveEvent_0(self);
    // return 1;
  }
}
pub trait QLineEdit_dragLeaveEvent_0<RetType> {
  fn dragLeaveEvent_0(self , rsthis: & QLineEdit) -> RetType;
}
impl<'a> /*trait*/ QLineEdit_dragLeaveEvent_0<(/*void*/)> for (usize) {
  fn dragLeaveEvent_0(self , rsthis: & QLineEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QLineEdit14dragLeaveEventEP15QDragLeaveEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlineedit.h:224
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void dropEvent(QDropEvent *)

/*
Reimplemented from QWidget::dropEvent().
*/
impl /*struct*/ QLineEdit {
  pub fn dropEvent_0<RetType, T: QLineEdit_dropEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dropEvent_0(self);
    // return 1;
  }
}
pub trait QLineEdit_dropEvent_0<RetType> {
  fn dropEvent_0(self , rsthis: & QLineEdit) -> RetType;
}
impl<'a> /*trait*/ QLineEdit_dropEvent_0<(/*void*/)> for (usize) {
  fn dropEvent_0(self , rsthis: & QLineEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QLineEdit9dropEventEP10QDropEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlineedit.h:226
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void changeEvent(QEvent *)

/*
Reimplemented from QWidget::changeEvent().
*/
impl /*struct*/ QLineEdit {
  pub fn changeEvent_0<RetType, T: QLineEdit_changeEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.changeEvent_0(self);
    // return 1;
  }
}
pub trait QLineEdit_changeEvent_0<RetType> {
  fn changeEvent_0(self , rsthis: & QLineEdit) -> RetType;
}
impl<'a> /*trait*/ QLineEdit_changeEvent_0<(/*void*/)> for (usize) {
  fn changeEvent_0(self , rsthis: & QLineEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QLineEdit11changeEventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlineedit.h:228
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void contextMenuEvent(QContextMenuEvent *)

/*
Reimplemented from QWidget::contextMenuEvent().

Shows the standard context menu created with createStandardContextMenu().

If you do not want the line edit to have a context menu, you can set its contextMenuPolicy to Qt::NoContextMenu. If you want to customize the context menu, reimplement this function. If you want to extend the standard context menu, reimplement this function, call createStandardContextMenu() and extend the menu returned.


  void LineEdit::contextMenuEvent(QContextMenuEvent *event)
  {
      QMenu *menu = createStandardContextMenu();
      menu->addAction(tr("My Menu Item"));
      //...
      menu->exec(event->globalPos());
      delete menu;
  }



The event parameter is used to obtain the position where the mouse cursor was when the event was generated.

See also setContextMenuPolicy().
*/
impl /*struct*/ QLineEdit {
  pub fn contextMenuEvent_0<RetType, T: QLineEdit_contextMenuEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.contextMenuEvent_0(self);
    // return 1;
  }
}
pub trait QLineEdit_contextMenuEvent_0<RetType> {
  fn contextMenuEvent_0(self , rsthis: & QLineEdit) -> RetType;
}
impl<'a> /*trait*/ QLineEdit_contextMenuEvent_0<(/*void*/)> for (usize) {
  fn contextMenuEvent_0(self , rsthis: & QLineEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QLineEdit16contextMenuEventEP17QContextMenuEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlineedit.h:231
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void inputMethodEvent(QInputMethodEvent *)

/*
Reimplemented from QWidget::inputMethodEvent().
*/
impl /*struct*/ QLineEdit {
  pub fn inputMethodEvent_0<RetType, T: QLineEdit_inputMethodEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.inputMethodEvent_0(self);
    // return 1;
  }
}
pub trait QLineEdit_inputMethodEvent_0<RetType> {
  fn inputMethodEvent_0(self , rsthis: & QLineEdit) -> RetType;
}
impl<'a> /*trait*/ QLineEdit_inputMethodEvent_0<(/*void*/)> for (usize) {
  fn inputMethodEvent_0(self , rsthis: & QLineEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QLineEdit16inputMethodEventEP17QInputMethodEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlineedit.h:232
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void initStyleOption(QStyleOptionFrame *) const

/*
Initialize option with the values from this QLineEdit. This method is useful for subclasses when they need a QStyleOptionFrame, but don't want to fill in all the information themselves.

See also QStyleOption::initFrom().
*/
impl /*struct*/ QLineEdit {
  pub fn initStyleOption_0<RetType, T: QLineEdit_initStyleOption_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.initStyleOption_0(self);
    // return 1;
  }
}
pub trait QLineEdit_initStyleOption_0<RetType> {
  fn initStyleOption_0(self , rsthis: & QLineEdit) -> RetType;
}
impl<'a> /*trait*/ QLineEdit_initStyleOption_0<(/*void*/)> for (usize) {
  fn initStyleOption_0(self , rsthis: & QLineEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK9QLineEdit15initStyleOptionEP17QStyleOptionFrame", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qlineedit.h:234
// index:0
// Public virtual Visibility=Default Availability=Available
// [16] QVariant inputMethodQuery(Qt::InputMethodQuery) const

/*
Reimplemented from QWidget::inputMethodQuery().
*/
impl /*struct*/ QLineEdit {
  pub fn inputMethodQuery_0<RetType, T: QLineEdit_inputMethodQuery_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.inputMethodQuery_0(self);
    // return 1;
  }
}
pub trait QLineEdit_inputMethodQuery_0<RetType> {
  fn inputMethodQuery_0(self , rsthis: & QLineEdit) -> RetType;
}
impl<'a> /*trait*/ QLineEdit_inputMethodQuery_0<usize> for (i32) {
  fn inputMethodQuery_0(self , rsthis: & QLineEdit) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QLineEdit16inputMethodQueryEN2Qt16InputMethodQueryE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlineedit.h:235
// index:1
// Public Visibility=Default Availability=Available
// [16] QVariant inputMethodQuery(Qt::InputMethodQuery, QVariant) const

/*
Reimplemented from QWidget::inputMethodQuery().
*/
impl /*struct*/ QLineEdit {
  pub fn inputMethodQuery_1<RetType, T: QLineEdit_inputMethodQuery_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.inputMethodQuery_1(self);
    // return 1;
  }
}
pub trait QLineEdit_inputMethodQuery_1<RetType> {
  fn inputMethodQuery_1(self , rsthis: & QLineEdit) -> RetType;
}
impl<'a> /*trait*/ QLineEdit_inputMethodQuery_1<usize> for (i32,usize) {
  fn inputMethodQuery_1(self , rsthis: & QLineEdit) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QLineEdit16inputMethodQueryEN2Qt16InputMethodQueryE8QVariant", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlineedit.h:236
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool event(QEvent *)

/*
Reimplemented from QObject::event().
*/
impl /*struct*/ QLineEdit {
  pub fn event_0<RetType, T: QLineEdit_event_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.event_0(self);
    // return 1;
  }
}
pub trait QLineEdit_event_0<RetType> {
  fn event_0(self , rsthis: & QLineEdit) -> RetType;
}
impl<'a> /*trait*/ QLineEdit_event_0<bool> for (usize) {
  fn event_0(self , rsthis: & QLineEdit) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QLineEdit5eventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qlineedit.h:238
// index:0
// Protected Visibility=Default Availability=Available
// [16] QRect cursorRect() const

/*
Returns a rectangle that includes the lineedit cursor.

This function was introduced in  Qt 4.4.
*/
impl /*struct*/ QLineEdit {
  pub fn cursorRect_0<RetType, T: QLineEdit_cursorRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cursorRect_0(self);
    // return 1;
  }
}
pub trait QLineEdit_cursorRect_0<RetType> {
  fn cursorRect_0(self , rsthis: & QLineEdit) -> RetType;
}
impl<'a> /*trait*/ QLineEdit_cursorRect_0<usize> for () {
  fn cursorRect_0(self , rsthis: & QLineEdit) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QLineEdit10cursorRectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}


/*
This enum type describes how a line edit should display the action widgets to be added.



This enum was introduced or modified in  Qt 5.2.

See also addAction(), removeAction(), and QWidget::layoutDirection.

*/
pub type QLineEdit__ActionPosition = i32;
// The widget is displayed to the left of the text when using layout direction Qt::LeftToRight or to the right when using Qt::RightToLeft, respectively.
pub const QLineEdit__LeadingPosition :QLineEdit__ActionPosition = 0;
// The widget is displayed to the right of the text when using layout direction Qt::LeftToRight or to the left when using Qt::RightToLeft, respectively.
pub const QLineEdit__TrailingPosition :QLineEdit__ActionPosition = 1;
pub fn QLineEdit_ActionPositionItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QLineEdit", val);
}
pub fn QLineEdit_ActionPositionItemName_s(val: i32) ->String {
  //var nilthis *QLineEdit
  //return nilthis.ActionPositionItemName(val);
  return QLineEdit_ActionPositionItemName(val);
}


/*
This enum type describes how a line edit should display its contents.



See also setEchoMode() and echoMode().

*/
pub type QLineEdit__EchoMode = i32;
// Display characters as they are entered. This is the default.
pub const QLineEdit__Normal :QLineEdit__EchoMode = 0;
// Do not display anything. This may be appropriate for passwords where even the length of the password should be kept secret.
pub const QLineEdit__NoEcho :QLineEdit__EchoMode = 1;
// Display platform-dependent password mask characters instead of the characters actually entered.
pub const QLineEdit__Password :QLineEdit__EchoMode = 2;
// Display characters as they are entered while editing otherwise display characters as with Password.
pub const QLineEdit__PasswordEchoOnEdit :QLineEdit__EchoMode = 3;
pub fn QLineEdit_EchoModeItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QLineEdit", val);
}
pub fn QLineEdit_EchoModeItemName_s(val: i32) ->String {
  //var nilthis *QLineEdit
  //return nilthis.EchoModeItemName(val);
  return QLineEdit_EchoModeItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
