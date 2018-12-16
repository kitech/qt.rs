

// mod ::widgets::QAbstractButton
// package qtwidgets
// /usr/include/qt/QtWidgets/qabstractbutton.h
// #include <qabstractbutton.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 288
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

// void paintEvent(QPaintEvent *)
// func (this *QAbstractButton) InheritPaintEvent(f func(e *qtgui.QPaintEvent/*777 QPaintEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "paintEvent", f)
// }

// bool hitButton(const QPoint &)
// func (this *QAbstractButton) InheritHitButton(f func(pos *qtcore.QPoint) bool) {
//  qtrt.SetAllInheritCallback(this, "hitButton", f)
// }

// void checkStateSet()
// func (this *QAbstractButton) InheritCheckStateSet(f func() /*void*/) {
//  qtrt.SetAllInheritCallback(this, "checkStateSet", f)
// }

// void nextCheckState()
// func (this *QAbstractButton) InheritNextCheckState(f func() /*void*/) {
//  qtrt.SetAllInheritCallback(this, "nextCheckState", f)
// }

// bool event(QEvent *)
// func (this *QAbstractButton) InheritEvent(f func(e *qtcore.QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "event", f)
// }

// void keyPressEvent(QKeyEvent *)
// func (this *QAbstractButton) InheritKeyPressEvent(f func(e *qtgui.QKeyEvent/*777 QKeyEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "keyPressEvent", f)
// }

// void keyReleaseEvent(QKeyEvent *)
// func (this *QAbstractButton) InheritKeyReleaseEvent(f func(e *qtgui.QKeyEvent/*777 QKeyEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "keyReleaseEvent", f)
// }

// void mousePressEvent(QMouseEvent *)
// func (this *QAbstractButton) InheritMousePressEvent(f func(e *qtgui.QMouseEvent/*777 QMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mousePressEvent", f)
// }

// void mouseReleaseEvent(QMouseEvent *)
// func (this *QAbstractButton) InheritMouseReleaseEvent(f func(e *qtgui.QMouseEvent/*777 QMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mouseReleaseEvent", f)
// }

// void mouseMoveEvent(QMouseEvent *)
// func (this *QAbstractButton) InheritMouseMoveEvent(f func(e *qtgui.QMouseEvent/*777 QMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mouseMoveEvent", f)
// }

// void focusInEvent(QFocusEvent *)
// func (this *QAbstractButton) InheritFocusInEvent(f func(e *qtgui.QFocusEvent/*777 QFocusEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "focusInEvent", f)
// }

// void focusOutEvent(QFocusEvent *)
// func (this *QAbstractButton) InheritFocusOutEvent(f func(e *qtgui.QFocusEvent/*777 QFocusEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "focusOutEvent", f)
// }

// void changeEvent(QEvent *)
// func (this *QAbstractButton) InheritChangeEvent(f func(e *qtcore.QEvent/*777 QEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "changeEvent", f)
// }

// void timerEvent(QTimerEvent *)
// func (this *QAbstractButton) InheritTimerEvent(f func(e *qtcore.QTimerEvent/*777 QTimerEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "timerEvent", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QAbstractButton)=48
pub struct QAbstractButton {
  qbase: QWidget,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QAbstractButton_ITF interface {
//    QWidget_ITF
//    QAbstractButton_PTR() *QAbstractButton
//}
//func (ptr *QAbstractButton) QAbstractButton_PTR() *QAbstractButton { return ptr }

impl /*struct*/ QAbstractButton {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QAbstractButton {
    return QAbstractButton{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QAbstractButton {
//  type Target = QAbstractButtonBASE;
//
//  fn deref(&self) -> &QAbstractButtonBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QAbstractButtonBASE> for QAbstractButton {
//  fn as_ref(& self) -> & QAbstractButtonBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qabstractbutton.h:58
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QAbstractButton {
  pub fn metaObject_0<RetType, T: QAbstractButton_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QAbstractButton_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QAbstractButton) -> RetType;
}
impl<'a> /*trait*/ QAbstractButton_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QAbstractButton) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QAbstractButton10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractbutton.h:75
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QAbstractButton(QWidget *)

/*
Constructs an abstract button with a parent.
*/
// QAbstractButton(QWidget *) ctx.fn_proto_cpp
impl /*struct*/ QAbstractButton {
  pub fn QAbstractButton_0<T: QAbstractButton_QAbstractButton_0>(value: T) -> QAbstractButton {
    let rsthis = value.QAbstractButton_0();
    return rsthis;
    // return 1;
  }
}

pub trait QAbstractButton_QAbstractButton_0 {
  fn QAbstractButton_0(self) -> QAbstractButton;
}
// QAbstractButton(QWidget *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QAbstractButton_QAbstractButton_0 for (usize) {
  fn QAbstractButton_0(self) -> QAbstractButton {
    // unsafe{_ZN15QAbstractButtonC2EP7QWidget()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN15QAbstractButtonC2EP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QAbstractButton{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractbutton.h:76
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QAbstractButton()

/*

*/
pub fn DeleteQAbstractButton(this :*mut QAbstractButton) {
    // let rv = qtrt::InvokeQtFunc6("_ZN15QAbstractButtonD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 48)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qabstractbutton.h:78
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setText(const QString &)

/*

*/
impl /*struct*/ QAbstractButton {
  pub fn setText_0<RetType, T: QAbstractButton_setText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setText_0(self);
    // return 1;
  }
}
pub trait QAbstractButton_setText_0<RetType> {
  fn setText_0(self , rsthis: & QAbstractButton) -> RetType;
}
impl<'a> /*trait*/ QAbstractButton_setText_0<(/*void*/)> for (usize) {
  fn setText_0(self , rsthis: & QAbstractButton) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QAbstractButton7setTextERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractbutton.h:79
// index:0
// Public Visibility=Default Availability=Available
// [8] QString text() const

/*

*/
impl /*struct*/ QAbstractButton {
  pub fn text_0<RetType, T: QAbstractButton_text_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.text_0(self);
    // return 1;
  }
}
pub trait QAbstractButton_text_0<RetType> {
  fn text_0(self , rsthis: & QAbstractButton) -> RetType;
}
impl<'a> /*trait*/ QAbstractButton_text_0<usize> for () {
  fn text_0(self , rsthis: & QAbstractButton) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QAbstractButton4textEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractbutton.h:81
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setIcon(const QIcon &)

/*

*/
impl /*struct*/ QAbstractButton {
  pub fn setIcon_0<RetType, T: QAbstractButton_setIcon_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setIcon_0(self);
    // return 1;
  }
}
pub trait QAbstractButton_setIcon_0<RetType> {
  fn setIcon_0(self , rsthis: & QAbstractButton) -> RetType;
}
impl<'a> /*trait*/ QAbstractButton_setIcon_0<(/*void*/)> for (usize) {
  fn setIcon_0(self , rsthis: & QAbstractButton) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QAbstractButton7setIconERK5QIcon", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractbutton.h:82
// index:0
// Public Visibility=Default Availability=Available
// [8] QIcon icon() const

/*

*/
impl /*struct*/ QAbstractButton {
  pub fn icon_0<RetType, T: QAbstractButton_icon_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.icon_0(self);
    // return 1;
  }
}
pub trait QAbstractButton_icon_0<RetType> {
  fn icon_0(self , rsthis: & QAbstractButton) -> RetType;
}
impl<'a> /*trait*/ QAbstractButton_icon_0<usize> for () {
  fn icon_0(self , rsthis: & QAbstractButton) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QAbstractButton4iconEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractbutton.h:84
// index:0
// Public Visibility=Default Availability=Available
// [8] QSize iconSize() const

/*

*/
impl /*struct*/ QAbstractButton {
  pub fn iconSize_0<RetType, T: QAbstractButton_iconSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.iconSize_0(self);
    // return 1;
  }
}
pub trait QAbstractButton_iconSize_0<RetType> {
  fn iconSize_0(self , rsthis: & QAbstractButton) -> RetType;
}
impl<'a> /*trait*/ QAbstractButton_iconSize_0<usize> for () {
  fn iconSize_0(self , rsthis: & QAbstractButton) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QAbstractButton8iconSizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractbutton.h:87
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setShortcut(const QKeySequence &)

/*

*/
impl /*struct*/ QAbstractButton {
  pub fn setShortcut_0<RetType, T: QAbstractButton_setShortcut_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setShortcut_0(self);
    // return 1;
  }
}
pub trait QAbstractButton_setShortcut_0<RetType> {
  fn setShortcut_0(self , rsthis: & QAbstractButton) -> RetType;
}
impl<'a> /*trait*/ QAbstractButton_setShortcut_0<(/*void*/)> for (usize) {
  fn setShortcut_0(self , rsthis: & QAbstractButton) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QAbstractButton11setShortcutERK12QKeySequence", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractbutton.h:88
// index:0
// Public Visibility=Default Availability=Available
// [8] QKeySequence shortcut() const

/*

*/
impl /*struct*/ QAbstractButton {
  pub fn shortcut_0<RetType, T: QAbstractButton_shortcut_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.shortcut_0(self);
    // return 1;
  }
}
pub trait QAbstractButton_shortcut_0<RetType> {
  fn shortcut_0(self , rsthis: & QAbstractButton) -> RetType;
}
impl<'a> /*trait*/ QAbstractButton_shortcut_0<usize> for () {
  fn shortcut_0(self , rsthis: & QAbstractButton) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QAbstractButton8shortcutEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractbutton.h:91
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCheckable(bool)

/*

*/
impl /*struct*/ QAbstractButton {
  pub fn setCheckable_0<RetType, T: QAbstractButton_setCheckable_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCheckable_0(self);
    // return 1;
  }
}
pub trait QAbstractButton_setCheckable_0<RetType> {
  fn setCheckable_0(self , rsthis: & QAbstractButton) -> RetType;
}
impl<'a> /*trait*/ QAbstractButton_setCheckable_0<(/*void*/)> for (bool) {
  fn setCheckable_0(self , rsthis: & QAbstractButton) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN15QAbstractButton12setCheckableEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractbutton.h:92
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isCheckable() const

/*

*/
impl /*struct*/ QAbstractButton {
  pub fn isCheckable_0<RetType, T: QAbstractButton_isCheckable_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isCheckable_0(self);
    // return 1;
  }
}
pub trait QAbstractButton_isCheckable_0<RetType> {
  fn isCheckable_0(self , rsthis: & QAbstractButton) -> RetType;
}
impl<'a> /*trait*/ QAbstractButton_isCheckable_0<bool> for () {
  fn isCheckable_0(self , rsthis: & QAbstractButton) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QAbstractButton11isCheckableEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractbutton.h:94
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isChecked() const

/*

*/
impl /*struct*/ QAbstractButton {
  pub fn isChecked_0<RetType, T: QAbstractButton_isChecked_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isChecked_0(self);
    // return 1;
  }
}
pub trait QAbstractButton_isChecked_0<RetType> {
  fn isChecked_0(self , rsthis: & QAbstractButton) -> RetType;
}
impl<'a> /*trait*/ QAbstractButton_isChecked_0<bool> for () {
  fn isChecked_0(self , rsthis: & QAbstractButton) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QAbstractButton9isCheckedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractbutton.h:96
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDown(bool)

/*

*/
impl /*struct*/ QAbstractButton {
  pub fn setDown_0<RetType, T: QAbstractButton_setDown_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDown_0(self);
    // return 1;
  }
}
pub trait QAbstractButton_setDown_0<RetType> {
  fn setDown_0(self , rsthis: & QAbstractButton) -> RetType;
}
impl<'a> /*trait*/ QAbstractButton_setDown_0<(/*void*/)> for (bool) {
  fn setDown_0(self , rsthis: & QAbstractButton) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN15QAbstractButton7setDownEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractbutton.h:97
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isDown() const

/*

*/
impl /*struct*/ QAbstractButton {
  pub fn isDown_0<RetType, T: QAbstractButton_isDown_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isDown_0(self);
    // return 1;
  }
}
pub trait QAbstractButton_isDown_0<RetType> {
  fn isDown_0(self , rsthis: & QAbstractButton) -> RetType;
}
impl<'a> /*trait*/ QAbstractButton_isDown_0<bool> for () {
  fn isDown_0(self , rsthis: & QAbstractButton) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QAbstractButton6isDownEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractbutton.h:99
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setAutoRepeat(bool)

/*

*/
impl /*struct*/ QAbstractButton {
  pub fn setAutoRepeat_0<RetType, T: QAbstractButton_setAutoRepeat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setAutoRepeat_0(self);
    // return 1;
  }
}
pub trait QAbstractButton_setAutoRepeat_0<RetType> {
  fn setAutoRepeat_0(self , rsthis: & QAbstractButton) -> RetType;
}
impl<'a> /*trait*/ QAbstractButton_setAutoRepeat_0<(/*void*/)> for (bool) {
  fn setAutoRepeat_0(self , rsthis: & QAbstractButton) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN15QAbstractButton13setAutoRepeatEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractbutton.h:100
// index:0
// Public Visibility=Default Availability=Available
// [1] bool autoRepeat() const

/*

*/
impl /*struct*/ QAbstractButton {
  pub fn autoRepeat_0<RetType, T: QAbstractButton_autoRepeat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.autoRepeat_0(self);
    // return 1;
  }
}
pub trait QAbstractButton_autoRepeat_0<RetType> {
  fn autoRepeat_0(self , rsthis: & QAbstractButton) -> RetType;
}
impl<'a> /*trait*/ QAbstractButton_autoRepeat_0<bool> for () {
  fn autoRepeat_0(self , rsthis: & QAbstractButton) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QAbstractButton10autoRepeatEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractbutton.h:102
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setAutoRepeatDelay(int)

/*

*/
impl /*struct*/ QAbstractButton {
  pub fn setAutoRepeatDelay_0<RetType, T: QAbstractButton_setAutoRepeatDelay_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setAutoRepeatDelay_0(self);
    // return 1;
  }
}
pub trait QAbstractButton_setAutoRepeatDelay_0<RetType> {
  fn setAutoRepeatDelay_0(self , rsthis: & QAbstractButton) -> RetType;
}
impl<'a> /*trait*/ QAbstractButton_setAutoRepeatDelay_0<(/*void*/)> for (i32) {
  fn setAutoRepeatDelay_0(self , rsthis: & QAbstractButton) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN15QAbstractButton18setAutoRepeatDelayEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractbutton.h:103
// index:0
// Public Visibility=Default Availability=Available
// [4] int autoRepeatDelay() const

/*

*/
impl /*struct*/ QAbstractButton {
  pub fn autoRepeatDelay_0<RetType, T: QAbstractButton_autoRepeatDelay_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.autoRepeatDelay_0(self);
    // return 1;
  }
}
pub trait QAbstractButton_autoRepeatDelay_0<RetType> {
  fn autoRepeatDelay_0(self , rsthis: & QAbstractButton) -> RetType;
}
impl<'a> /*trait*/ QAbstractButton_autoRepeatDelay_0<i32> for () {
  fn autoRepeatDelay_0(self , rsthis: & QAbstractButton) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QAbstractButton15autoRepeatDelayEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractbutton.h:105
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setAutoRepeatInterval(int)

/*

*/
impl /*struct*/ QAbstractButton {
  pub fn setAutoRepeatInterval_0<RetType, T: QAbstractButton_setAutoRepeatInterval_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setAutoRepeatInterval_0(self);
    // return 1;
  }
}
pub trait QAbstractButton_setAutoRepeatInterval_0<RetType> {
  fn setAutoRepeatInterval_0(self , rsthis: & QAbstractButton) -> RetType;
}
impl<'a> /*trait*/ QAbstractButton_setAutoRepeatInterval_0<(/*void*/)> for (i32) {
  fn setAutoRepeatInterval_0(self , rsthis: & QAbstractButton) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN15QAbstractButton21setAutoRepeatIntervalEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractbutton.h:106
// index:0
// Public Visibility=Default Availability=Available
// [4] int autoRepeatInterval() const

/*

*/
impl /*struct*/ QAbstractButton {
  pub fn autoRepeatInterval_0<RetType, T: QAbstractButton_autoRepeatInterval_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.autoRepeatInterval_0(self);
    // return 1;
  }
}
pub trait QAbstractButton_autoRepeatInterval_0<RetType> {
  fn autoRepeatInterval_0(self , rsthis: & QAbstractButton) -> RetType;
}
impl<'a> /*trait*/ QAbstractButton_autoRepeatInterval_0<i32> for () {
  fn autoRepeatInterval_0(self , rsthis: & QAbstractButton) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QAbstractButton18autoRepeatIntervalEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractbutton.h:108
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setAutoExclusive(bool)

/*

*/
impl /*struct*/ QAbstractButton {
  pub fn setAutoExclusive_0<RetType, T: QAbstractButton_setAutoExclusive_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setAutoExclusive_0(self);
    // return 1;
  }
}
pub trait QAbstractButton_setAutoExclusive_0<RetType> {
  fn setAutoExclusive_0(self , rsthis: & QAbstractButton) -> RetType;
}
impl<'a> /*trait*/ QAbstractButton_setAutoExclusive_0<(/*void*/)> for (bool) {
  fn setAutoExclusive_0(self , rsthis: & QAbstractButton) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN15QAbstractButton16setAutoExclusiveEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractbutton.h:109
// index:0
// Public Visibility=Default Availability=Available
// [1] bool autoExclusive() const

/*

*/
impl /*struct*/ QAbstractButton {
  pub fn autoExclusive_0<RetType, T: QAbstractButton_autoExclusive_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.autoExclusive_0(self);
    // return 1;
  }
}
pub trait QAbstractButton_autoExclusive_0<RetType> {
  fn autoExclusive_0(self , rsthis: & QAbstractButton) -> RetType;
}
impl<'a> /*trait*/ QAbstractButton_autoExclusive_0<bool> for () {
  fn autoExclusive_0(self , rsthis: & QAbstractButton) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QAbstractButton13autoExclusiveEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractbutton.h:112
// index:0
// Public Visibility=Default Availability=Available
// [8] QButtonGroup * group() const

/*
Returns the group that this button belongs to.

If the button is not a member of any QButtonGroup, this function returns 0.

See also QButtonGroup.
*/
impl /*struct*/ QAbstractButton {
  pub fn group_0<RetType, T: QAbstractButton_group_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.group_0(self);
    // return 1;
  }
}
pub trait QAbstractButton_group_0<RetType> {
  fn group_0(self , rsthis: & QAbstractButton) -> RetType;
}
impl<'a> /*trait*/ QAbstractButton_group_0<usize> for () {
  fn group_0(self , rsthis: & QAbstractButton) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QAbstractButton5groupEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractbutton.h:116
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setIconSize(const QSize &)

/*

*/
impl /*struct*/ QAbstractButton {
  pub fn setIconSize_0<RetType, T: QAbstractButton_setIconSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setIconSize_0(self);
    // return 1;
  }
}
pub trait QAbstractButton_setIconSize_0<RetType> {
  fn setIconSize_0(self , rsthis: & QAbstractButton) -> RetType;
}
impl<'a> /*trait*/ QAbstractButton_setIconSize_0<(/*void*/)> for (usize) {
  fn setIconSize_0(self , rsthis: & QAbstractButton) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QAbstractButton11setIconSizeERK5QSize", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractbutton.h:117
// index:0
// Public Visibility=Default Availability=Available
// [-2] void animateClick(int)

/*
Performs an animated click: the button is pressed immediately, and released msec milliseconds later (the default is 100 ms).

Calling this function again before the button is released resets the release timer.

All signals associated with a click are emitted as appropriate.

This function does nothing if the button is disabled.

See also click().
*/
impl /*struct*/ QAbstractButton {
  pub fn animateClick_0<RetType, T: QAbstractButton_animateClick_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.animateClick_0(self);
    // return 1;
  }
}
pub trait QAbstractButton_animateClick_0<RetType> {
  fn animateClick_0(self , rsthis: & QAbstractButton) -> RetType;
}
impl<'a> /*trait*/ QAbstractButton_animateClick_0<(/*void*/)> for (i32) {
  fn animateClick_0(self , rsthis: & QAbstractButton) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN15QAbstractButton12animateClickEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractbutton.h:118
// index:0
// Public Visibility=Default Availability=Available
// [-2] void click()

/*
Performs a click.

All the usual signals associated with a click are emitted as appropriate. If the button is checkable, the state of the button is toggled.

This function does nothing if the button is disabled.

See also animateClick().
*/
impl /*struct*/ QAbstractButton {
  pub fn click_0<RetType, T: QAbstractButton_click_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.click_0(self);
    // return 1;
  }
}
pub trait QAbstractButton_click_0<RetType> {
  fn click_0(self , rsthis: & QAbstractButton) -> RetType;
}
impl<'a> /*trait*/ QAbstractButton_click_0<(/*void*/)> for () {
  fn click_0(self , rsthis: & QAbstractButton) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN15QAbstractButton5clickEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractbutton.h:119
// index:0
// Public Visibility=Default Availability=Available
// [-2] void toggle()

/*
Toggles the state of a checkable button.

See also checked.
*/
impl /*struct*/ QAbstractButton {
  pub fn toggle_0<RetType, T: QAbstractButton_toggle_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toggle_0(self);
    // return 1;
  }
}
pub trait QAbstractButton_toggle_0<RetType> {
  fn toggle_0(self , rsthis: & QAbstractButton) -> RetType;
}
impl<'a> /*trait*/ QAbstractButton_toggle_0<(/*void*/)> for () {
  fn toggle_0(self , rsthis: & QAbstractButton) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN15QAbstractButton6toggleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractbutton.h:120
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setChecked(bool)

/*

*/
impl /*struct*/ QAbstractButton {
  pub fn setChecked_0<RetType, T: QAbstractButton_setChecked_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setChecked_0(self);
    // return 1;
  }
}
pub trait QAbstractButton_setChecked_0<RetType> {
  fn setChecked_0(self , rsthis: & QAbstractButton) -> RetType;
}
impl<'a> /*trait*/ QAbstractButton_setChecked_0<(/*void*/)> for (bool) {
  fn setChecked_0(self , rsthis: & QAbstractButton) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN15QAbstractButton10setCheckedEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractbutton.h:123
// index:0
// Public Visibility=Default Availability=Available
// [-2] void pressed()

/*
This signal is emitted when the button is pressed down.

See also released() and clicked().
*/
impl /*struct*/ QAbstractButton {
  pub fn pressed_0<RetType, T: QAbstractButton_pressed_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.pressed_0(self);
    // return 1;
  }
}
pub trait QAbstractButton_pressed_0<RetType> {
  fn pressed_0(self , rsthis: & QAbstractButton) -> RetType;
}
impl<'a> /*trait*/ QAbstractButton_pressed_0<(/*void*/)> for () {
  fn pressed_0(self , rsthis: & QAbstractButton) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN15QAbstractButton7pressedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractbutton.h:124
// index:0
// Public Visibility=Default Availability=Available
// [-2] void released()

/*
This signal is emitted when the button is released.

See also pressed(), clicked(), and toggled().
*/
impl /*struct*/ QAbstractButton {
  pub fn released_0<RetType, T: QAbstractButton_released_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.released_0(self);
    // return 1;
  }
}
pub trait QAbstractButton_released_0<RetType> {
  fn released_0(self , rsthis: & QAbstractButton) -> RetType;
}
impl<'a> /*trait*/ QAbstractButton_released_0<(/*void*/)> for () {
  fn released_0(self , rsthis: & QAbstractButton) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN15QAbstractButton8releasedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractbutton.h:125
// index:0
// Public Visibility=Default Availability=Available
// [-2] void clicked(bool)

/*
This signal is emitted when the button is activated (i.e., pressed down then released while the mouse cursor is inside the button), when the shortcut key is typed, or when click() or animateClick() is called. Notably, this signal is not emitted if you call setDown(), setChecked() or toggle().

If the button is checkable, checked is true if the button is checked, or false if the button is unchecked.

See also pressed(), released(), and toggled().
*/
impl /*struct*/ QAbstractButton {
  pub fn clicked_0<RetType, T: QAbstractButton_clicked_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.clicked_0(self);
    // return 1;
  }
}
pub trait QAbstractButton_clicked_0<RetType> {
  fn clicked_0(self , rsthis: & QAbstractButton) -> RetType;
}
impl<'a> /*trait*/ QAbstractButton_clicked_0<(/*void*/)> for (bool) {
  fn clicked_0(self , rsthis: & QAbstractButton) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN15QAbstractButton7clickedEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractbutton.h:126
// index:0
// Public Visibility=Default Availability=Available
// [-2] void toggled(bool)

/*
This signal is emitted whenever a checkable button changes its state. checked is true if the button is checked, or false if the button is unchecked.

This may be the result of a user action, click() slot activation, or because setChecked() is called.

The states of buttons in exclusive button groups are updated before this signal is emitted. This means that slots can act on either the "off" signal or the "on" signal emitted by the buttons in the group whose states have changed.

For example, a slot that reacts to signals emitted by newly checked buttons but which ignores signals from buttons that have been unchecked can be implemented using the following pattern:


  void MyWidget::reactToToggle(bool checked)
  {
     if (checked) {
        // Examine the new button states.
        ...
     }
  }



Button groups can be created using the QButtonGroup class, and updates to the button states monitored with the QButtonGroup::buttonClicked() signal.

Note: Notifier signal for property checked. 

See also checked and clicked().
*/
impl /*struct*/ QAbstractButton {
  pub fn toggled_0<RetType, T: QAbstractButton_toggled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toggled_0(self);
    // return 1;
  }
}
pub trait QAbstractButton_toggled_0<RetType> {
  fn toggled_0(self , rsthis: & QAbstractButton) -> RetType;
}
impl<'a> /*trait*/ QAbstractButton_toggled_0<(/*void*/)> for (bool) {
  fn toggled_0(self , rsthis: & QAbstractButton) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN15QAbstractButton7toggledEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractbutton.h:129
// index:0
// Protected purevirtual virtual Visibility=Default Availability=Available
// [-2] void paintEvent(QPaintEvent *)

/*
Reimplemented from QWidget::paintEvent().
*/
impl /*struct*/ QAbstractButton {
  pub fn paintEvent_0<RetType, T: QAbstractButton_paintEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.paintEvent_0(self);
    // return 1;
  }
}
pub trait QAbstractButton_paintEvent_0<RetType> {
  fn paintEvent_0(self , rsthis: & QAbstractButton) -> RetType;
}
impl<'a> /*trait*/ QAbstractButton_paintEvent_0<(/*void*/)> for (usize) {
  fn paintEvent_0(self , rsthis: & QAbstractButton) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QAbstractButton10paintEventEP11QPaintEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractbutton.h:130
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool hitButton(const QPoint &) const

/*
Returns true if pos is inside the clickable button rectangle; otherwise returns false.

By default, the clickable area is the entire widget. Subclasses may reimplement this function to provide support for clickable areas of different shapes and sizes.
*/
impl /*struct*/ QAbstractButton {
  pub fn hitButton_0<RetType, T: QAbstractButton_hitButton_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hitButton_0(self);
    // return 1;
  }
}
pub trait QAbstractButton_hitButton_0<RetType> {
  fn hitButton_0(self , rsthis: & QAbstractButton) -> RetType;
}
impl<'a> /*trait*/ QAbstractButton_hitButton_0<bool> for (usize) {
  fn hitButton_0(self , rsthis: & QAbstractButton) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QAbstractButton9hitButtonERK6QPoint", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractbutton.h:131
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void checkStateSet()

/*
This virtual handler is called when setChecked() is used, unless it is called from within nextCheckState(). It allows subclasses to reset their intermediate button states.

See also nextCheckState().
*/
impl /*struct*/ QAbstractButton {
  pub fn checkStateSet_0<RetType, T: QAbstractButton_checkStateSet_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.checkStateSet_0(self);
    // return 1;
  }
}
pub trait QAbstractButton_checkStateSet_0<RetType> {
  fn checkStateSet_0(self , rsthis: & QAbstractButton) -> RetType;
}
impl<'a> /*trait*/ QAbstractButton_checkStateSet_0<(/*void*/)> for () {
  fn checkStateSet_0(self , rsthis: & QAbstractButton) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN15QAbstractButton13checkStateSetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractbutton.h:132
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void nextCheckState()

/*
This virtual handler is called when a button is clicked. The default implementation calls setChecked(!isChecked()) if the button isCheckable(). It allows subclasses to implement intermediate button states.

See also checkStateSet().
*/
impl /*struct*/ QAbstractButton {
  pub fn nextCheckState_0<RetType, T: QAbstractButton_nextCheckState_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.nextCheckState_0(self);
    // return 1;
  }
}
pub trait QAbstractButton_nextCheckState_0<RetType> {
  fn nextCheckState_0(self , rsthis: & QAbstractButton) -> RetType;
}
impl<'a> /*trait*/ QAbstractButton_nextCheckState_0<(/*void*/)> for () {
  fn nextCheckState_0(self , rsthis: & QAbstractButton) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN15QAbstractButton14nextCheckStateEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractbutton.h:134
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool event(QEvent *)

/*
Reimplemented from QObject::event().
*/
impl /*struct*/ QAbstractButton {
  pub fn event_0<RetType, T: QAbstractButton_event_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.event_0(self);
    // return 1;
  }
}
pub trait QAbstractButton_event_0<RetType> {
  fn event_0(self , rsthis: & QAbstractButton) -> RetType;
}
impl<'a> /*trait*/ QAbstractButton_event_0<bool> for (usize) {
  fn event_0(self , rsthis: & QAbstractButton) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN15QAbstractButton5eventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qabstractbutton.h:135
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void keyPressEvent(QKeyEvent *)

/*
Reimplemented from QWidget::keyPressEvent().
*/
impl /*struct*/ QAbstractButton {
  pub fn keyPressEvent_0<RetType, T: QAbstractButton_keyPressEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.keyPressEvent_0(self);
    // return 1;
  }
}
pub trait QAbstractButton_keyPressEvent_0<RetType> {
  fn keyPressEvent_0(self , rsthis: & QAbstractButton) -> RetType;
}
impl<'a> /*trait*/ QAbstractButton_keyPressEvent_0<(/*void*/)> for (usize) {
  fn keyPressEvent_0(self , rsthis: & QAbstractButton) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QAbstractButton13keyPressEventEP9QKeyEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractbutton.h:136
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void keyReleaseEvent(QKeyEvent *)

/*
Reimplemented from QWidget::keyReleaseEvent().
*/
impl /*struct*/ QAbstractButton {
  pub fn keyReleaseEvent_0<RetType, T: QAbstractButton_keyReleaseEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.keyReleaseEvent_0(self);
    // return 1;
  }
}
pub trait QAbstractButton_keyReleaseEvent_0<RetType> {
  fn keyReleaseEvent_0(self , rsthis: & QAbstractButton) -> RetType;
}
impl<'a> /*trait*/ QAbstractButton_keyReleaseEvent_0<(/*void*/)> for (usize) {
  fn keyReleaseEvent_0(self , rsthis: & QAbstractButton) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QAbstractButton15keyReleaseEventEP9QKeyEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractbutton.h:137
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mousePressEvent(QMouseEvent *)

/*
Reimplemented from QWidget::mousePressEvent().
*/
impl /*struct*/ QAbstractButton {
  pub fn mousePressEvent_0<RetType, T: QAbstractButton_mousePressEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mousePressEvent_0(self);
    // return 1;
  }
}
pub trait QAbstractButton_mousePressEvent_0<RetType> {
  fn mousePressEvent_0(self , rsthis: & QAbstractButton) -> RetType;
}
impl<'a> /*trait*/ QAbstractButton_mousePressEvent_0<(/*void*/)> for (usize) {
  fn mousePressEvent_0(self , rsthis: & QAbstractButton) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QAbstractButton15mousePressEventEP11QMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractbutton.h:138
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mouseReleaseEvent(QMouseEvent *)

/*
Reimplemented from QWidget::mouseReleaseEvent().
*/
impl /*struct*/ QAbstractButton {
  pub fn mouseReleaseEvent_0<RetType, T: QAbstractButton_mouseReleaseEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mouseReleaseEvent_0(self);
    // return 1;
  }
}
pub trait QAbstractButton_mouseReleaseEvent_0<RetType> {
  fn mouseReleaseEvent_0(self , rsthis: & QAbstractButton) -> RetType;
}
impl<'a> /*trait*/ QAbstractButton_mouseReleaseEvent_0<(/*void*/)> for (usize) {
  fn mouseReleaseEvent_0(self , rsthis: & QAbstractButton) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QAbstractButton17mouseReleaseEventEP11QMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractbutton.h:139
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mouseMoveEvent(QMouseEvent *)

/*
Reimplemented from QWidget::mouseMoveEvent().
*/
impl /*struct*/ QAbstractButton {
  pub fn mouseMoveEvent_0<RetType, T: QAbstractButton_mouseMoveEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mouseMoveEvent_0(self);
    // return 1;
  }
}
pub trait QAbstractButton_mouseMoveEvent_0<RetType> {
  fn mouseMoveEvent_0(self , rsthis: & QAbstractButton) -> RetType;
}
impl<'a> /*trait*/ QAbstractButton_mouseMoveEvent_0<(/*void*/)> for (usize) {
  fn mouseMoveEvent_0(self , rsthis: & QAbstractButton) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QAbstractButton14mouseMoveEventEP11QMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractbutton.h:140
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void focusInEvent(QFocusEvent *)

/*
Reimplemented from QWidget::focusInEvent().
*/
impl /*struct*/ QAbstractButton {
  pub fn focusInEvent_0<RetType, T: QAbstractButton_focusInEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.focusInEvent_0(self);
    // return 1;
  }
}
pub trait QAbstractButton_focusInEvent_0<RetType> {
  fn focusInEvent_0(self , rsthis: & QAbstractButton) -> RetType;
}
impl<'a> /*trait*/ QAbstractButton_focusInEvent_0<(/*void*/)> for (usize) {
  fn focusInEvent_0(self , rsthis: & QAbstractButton) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QAbstractButton12focusInEventEP11QFocusEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractbutton.h:141
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void focusOutEvent(QFocusEvent *)

/*
Reimplemented from QWidget::focusOutEvent().
*/
impl /*struct*/ QAbstractButton {
  pub fn focusOutEvent_0<RetType, T: QAbstractButton_focusOutEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.focusOutEvent_0(self);
    // return 1;
  }
}
pub trait QAbstractButton_focusOutEvent_0<RetType> {
  fn focusOutEvent_0(self , rsthis: & QAbstractButton) -> RetType;
}
impl<'a> /*trait*/ QAbstractButton_focusOutEvent_0<(/*void*/)> for (usize) {
  fn focusOutEvent_0(self , rsthis: & QAbstractButton) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QAbstractButton13focusOutEventEP11QFocusEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractbutton.h:142
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void changeEvent(QEvent *)

/*
Reimplemented from QWidget::changeEvent().
*/
impl /*struct*/ QAbstractButton {
  pub fn changeEvent_0<RetType, T: QAbstractButton_changeEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.changeEvent_0(self);
    // return 1;
  }
}
pub trait QAbstractButton_changeEvent_0<RetType> {
  fn changeEvent_0(self , rsthis: & QAbstractButton) -> RetType;
}
impl<'a> /*trait*/ QAbstractButton_changeEvent_0<(/*void*/)> for (usize) {
  fn changeEvent_0(self , rsthis: & QAbstractButton) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QAbstractButton11changeEventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qabstractbutton.h:143
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void timerEvent(QTimerEvent *)

/*
Reimplemented from QObject::timerEvent().
*/
impl /*struct*/ QAbstractButton {
  pub fn timerEvent_0<RetType, T: QAbstractButton_timerEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.timerEvent_0(self);
    // return 1;
  }
}
pub trait QAbstractButton_timerEvent_0<RetType> {
  fn timerEvent_0(self , rsthis: & QAbstractButton) -> RetType;
}
impl<'a> /*trait*/ QAbstractButton_timerEvent_0<(/*void*/)> for (usize) {
  fn timerEvent_0(self , rsthis: & QAbstractButton) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QAbstractButton10timerEventEP11QTimerEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

//  body block end

//  keep block begin

//  keep block end
