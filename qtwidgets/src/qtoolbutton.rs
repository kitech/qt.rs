

// mod ::widgets::QToolButton
// package qtwidgets
// /usr/include/qt/QtWidgets/qtoolbutton.h
// #include <qtoolbutton.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 29
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
// func (this *QToolButton) InheritEvent(f func(e *qtcore.QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "event", f)
// }

// void mousePressEvent(QMouseEvent *)
// func (this *QToolButton) InheritMousePressEvent(f func(arg0 *qtgui.QMouseEvent/*777 QMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mousePressEvent", f)
// }

// void mouseReleaseEvent(QMouseEvent *)
// func (this *QToolButton) InheritMouseReleaseEvent(f func(arg0 *qtgui.QMouseEvent/*777 QMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mouseReleaseEvent", f)
// }

// void paintEvent(QPaintEvent *)
// func (this *QToolButton) InheritPaintEvent(f func(arg0 *qtgui.QPaintEvent/*777 QPaintEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "paintEvent", f)
// }

// void actionEvent(QActionEvent *)
// func (this *QToolButton) InheritActionEvent(f func(arg0 *qtgui.QActionEvent/*777 QActionEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "actionEvent", f)
// }

// void enterEvent(QEvent *)
// func (this *QToolButton) InheritEnterEvent(f func(arg0 *qtcore.QEvent/*777 QEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "enterEvent", f)
// }

// void leaveEvent(QEvent *)
// func (this *QToolButton) InheritLeaveEvent(f func(arg0 *qtcore.QEvent/*777 QEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "leaveEvent", f)
// }

// void timerEvent(QTimerEvent *)
// func (this *QToolButton) InheritTimerEvent(f func(arg0 *qtcore.QTimerEvent/*777 QTimerEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "timerEvent", f)
// }

// void changeEvent(QEvent *)
// func (this *QToolButton) InheritChangeEvent(f func(arg0 *qtcore.QEvent/*777 QEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "changeEvent", f)
// }

// bool hitButton(const QPoint &)
// func (this *QToolButton) InheritHitButton(f func(pos *qtcore.QPoint) bool) {
//  qtrt.SetAllInheritCallback(this, "hitButton", f)
// }

// void nextCheckState()
// func (this *QToolButton) InheritNextCheckState(f func() /*void*/) {
//  qtrt.SetAllInheritCallback(this, "nextCheckState", f)
// }

// void initStyleOption(QStyleOptionToolButton *)
// func (this *QToolButton) InheritInitStyleOption(f func(option *QStyleOptionToolButton/*777 QStyleOptionToolButton **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "initStyleOption", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QToolButton)=48
pub struct QToolButton {
  qbase: QAbstractButton,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QToolButton_ITF interface {
//    QAbstractButton_ITF
//    QToolButton_PTR() *QToolButton
//}
//func (ptr *QToolButton) QToolButton_PTR() *QToolButton { return ptr }

impl /*struct*/ QToolButton {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QToolButton {
    return QToolButton{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QToolButton {
//  type Target = QToolButtonBASE;
//
//  fn deref(&self) -> &QToolButtonBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QToolButtonBASE> for QToolButton {
//  fn as_ref(& self) -> & QToolButtonBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qtoolbutton.h:57
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QToolButton {
  pub fn metaObject_0<RetType, T: QToolButton_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QToolButton_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QToolButton) -> RetType;
}
impl<'a> /*trait*/ QToolButton_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QToolButton) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QToolButton10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtoolbutton.h:74
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QToolButton(QWidget *)

/*
Constructs an empty tool button with parent parent.
*/
// QToolButton(QWidget *) ctx.fn_proto_cpp
impl /*struct*/ QToolButton {
  pub fn QToolButton_0<T: QToolButton_QToolButton_0>(value: T) -> QToolButton {
    let rsthis = value.QToolButton_0();
    return rsthis;
    // return 1;
  }
}

pub trait QToolButton_QToolButton_0 {
  fn QToolButton_0(self) -> QToolButton;
}
// QToolButton(QWidget *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QToolButton_QToolButton_0 for (usize) {
  fn QToolButton_0(self) -> QToolButton {
    // unsafe{_ZN11QToolButtonC2EP7QWidget()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QToolButtonC2EP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QToolButton{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtoolbutton.h:75
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QToolButton()

/*

*/
pub fn DeleteQToolButton(this :*mut QToolButton) {
    // let rv = qtrt::InvokeQtFunc6("_ZN11QToolButtonD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 48)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qtoolbutton.h:77
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QSize sizeHint() const

/*
Reimplemented from QWidget::sizeHint().
*/
impl /*struct*/ QToolButton {
  pub fn sizeHint_0<RetType, T: QToolButton_sizeHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sizeHint_0(self);
    // return 1;
  }
}
pub trait QToolButton_sizeHint_0<RetType> {
  fn sizeHint_0(self , rsthis: & QToolButton) -> RetType;
}
impl<'a> /*trait*/ QToolButton_sizeHint_0<usize> for () {
  fn sizeHint_0(self , rsthis: & QToolButton) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QToolButton8sizeHintEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtoolbutton.h:78
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QSize minimumSizeHint() const

/*
Reimplemented from QWidget::minimumSizeHint().
*/
impl /*struct*/ QToolButton {
  pub fn minimumSizeHint_0<RetType, T: QToolButton_minimumSizeHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.minimumSizeHint_0(self);
    // return 1;
  }
}
pub trait QToolButton_minimumSizeHint_0<RetType> {
  fn minimumSizeHint_0(self , rsthis: & QToolButton) -> RetType;
}
impl<'a> /*trait*/ QToolButton_minimumSizeHint_0<usize> for () {
  fn minimumSizeHint_0(self , rsthis: & QToolButton) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QToolButton15minimumSizeHintEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtoolbutton.h:80
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::ToolButtonStyle toolButtonStyle() const

/*

*/
impl /*struct*/ QToolButton {
  pub fn toolButtonStyle_0<RetType, T: QToolButton_toolButtonStyle_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toolButtonStyle_0(self);
    // return 1;
  }
}
pub trait QToolButton_toolButtonStyle_0<RetType> {
  fn toolButtonStyle_0(self , rsthis: & QToolButton) -> RetType;
}
impl<'a> /*trait*/ QToolButton_toolButtonStyle_0<i32> for () {
  fn toolButtonStyle_0(self , rsthis: & QToolButton) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QToolButton15toolButtonStyleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtoolbutton.h:82
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::ArrowType arrowType() const

/*

*/
impl /*struct*/ QToolButton {
  pub fn arrowType_0<RetType, T: QToolButton_arrowType_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.arrowType_0(self);
    // return 1;
  }
}
pub trait QToolButton_arrowType_0<RetType> {
  fn arrowType_0(self , rsthis: & QToolButton) -> RetType;
}
impl<'a> /*trait*/ QToolButton_arrowType_0<i32> for () {
  fn arrowType_0(self , rsthis: & QToolButton) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QToolButton9arrowTypeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtoolbutton.h:83
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setArrowType(Qt::ArrowType)

/*

*/
impl /*struct*/ QToolButton {
  pub fn setArrowType_0<RetType, T: QToolButton_setArrowType_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setArrowType_0(self);
    // return 1;
  }
}
pub trait QToolButton_setArrowType_0<RetType> {
  fn setArrowType_0(self , rsthis: & QToolButton) -> RetType;
}
impl<'a> /*trait*/ QToolButton_setArrowType_0<(/*void*/)> for (i32) {
  fn setArrowType_0(self , rsthis: & QToolButton) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QToolButton12setArrowTypeEN2Qt9ArrowTypeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtoolbutton.h:86
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setMenu(QMenu *)

/*
Associates the given menu with this tool button.

The menu will be shown according to the button's popupMode.

Ownership of the menu is not transferred to the tool button.

See also menu().
*/
impl /*struct*/ QToolButton {
  pub fn setMenu_0<RetType, T: QToolButton_setMenu_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMenu_0(self);
    // return 1;
  }
}
pub trait QToolButton_setMenu_0<RetType> {
  fn setMenu_0(self , rsthis: & QToolButton) -> RetType;
}
impl<'a> /*trait*/ QToolButton_setMenu_0<(/*void*/)> for (usize) {
  fn setMenu_0(self , rsthis: & QToolButton) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QToolButton7setMenuEP5QMenu", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtoolbutton.h:87
// index:0
// Public Visibility=Default Availability=Available
// [8] QMenu * menu() const

/*
Returns the associated menu, or 0 if no menu has been defined.

See also setMenu().
*/
impl /*struct*/ QToolButton {
  pub fn menu_0<RetType, T: QToolButton_menu_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.menu_0(self);
    // return 1;
  }
}
pub trait QToolButton_menu_0<RetType> {
  fn menu_0(self , rsthis: & QToolButton) -> RetType;
}
impl<'a> /*trait*/ QToolButton_menu_0<usize> for () {
  fn menu_0(self , rsthis: & QToolButton) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QToolButton4menuEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtoolbutton.h:89
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setPopupMode(QToolButton::ToolButtonPopupMode)

/*

*/
impl /*struct*/ QToolButton {
  pub fn setPopupMode_0<RetType, T: QToolButton_setPopupMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPopupMode_0(self);
    // return 1;
  }
}
pub trait QToolButton_setPopupMode_0<RetType> {
  fn setPopupMode_0(self , rsthis: & QToolButton) -> RetType;
}
impl<'a> /*trait*/ QToolButton_setPopupMode_0<(/*void*/)> for (i32) {
  fn setPopupMode_0(self , rsthis: & QToolButton) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QToolButton12setPopupModeENS_19ToolButtonPopupModeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtoolbutton.h:90
// index:0
// Public Visibility=Default Availability=Available
// [4] QToolButton::ToolButtonPopupMode popupMode() const

/*

*/
impl /*struct*/ QToolButton {
  pub fn popupMode_0<RetType, T: QToolButton_popupMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.popupMode_0(self);
    // return 1;
  }
}
pub trait QToolButton_popupMode_0<RetType> {
  fn popupMode_0(self , rsthis: & QToolButton) -> RetType;
}
impl<'a> /*trait*/ QToolButton_popupMode_0<i32> for () {
  fn popupMode_0(self , rsthis: & QToolButton) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QToolButton9popupModeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtoolbutton.h:93
// index:0
// Public Visibility=Default Availability=Available
// [8] QAction * defaultAction() const

/*
Returns the default action.

See also setDefaultAction().
*/
impl /*struct*/ QToolButton {
  pub fn defaultAction_0<RetType, T: QToolButton_defaultAction_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.defaultAction_0(self);
    // return 1;
  }
}
pub trait QToolButton_defaultAction_0<RetType> {
  fn defaultAction_0(self , rsthis: & QToolButton) -> RetType;
}
impl<'a> /*trait*/ QToolButton_defaultAction_0<usize> for () {
  fn defaultAction_0(self , rsthis: & QToolButton) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QToolButton13defaultActionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtoolbutton.h:95
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setAutoRaise(bool)

/*

*/
impl /*struct*/ QToolButton {
  pub fn setAutoRaise_0<RetType, T: QToolButton_setAutoRaise_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setAutoRaise_0(self);
    // return 1;
  }
}
pub trait QToolButton_setAutoRaise_0<RetType> {
  fn setAutoRaise_0(self , rsthis: & QToolButton) -> RetType;
}
impl<'a> /*trait*/ QToolButton_setAutoRaise_0<(/*void*/)> for (bool) {
  fn setAutoRaise_0(self , rsthis: & QToolButton) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN11QToolButton12setAutoRaiseEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtoolbutton.h:96
// index:0
// Public Visibility=Default Availability=Available
// [1] bool autoRaise() const

/*

*/
impl /*struct*/ QToolButton {
  pub fn autoRaise_0<RetType, T: QToolButton_autoRaise_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.autoRaise_0(self);
    // return 1;
  }
}
pub trait QToolButton_autoRaise_0<RetType> {
  fn autoRaise_0(self , rsthis: & QToolButton) -> RetType;
}
impl<'a> /*trait*/ QToolButton_autoRaise_0<bool> for () {
  fn autoRaise_0(self , rsthis: & QToolButton) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QToolButton9autoRaiseEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtoolbutton.h:100
// index:0
// Public Visibility=Default Availability=Available
// [-2] void showMenu()

/*
Shows (pops up) the associated popup menu. If there is no such menu, this function does nothing. This function does not return until the popup menu has been closed by the user.
*/
impl /*struct*/ QToolButton {
  pub fn showMenu_0<RetType, T: QToolButton_showMenu_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.showMenu_0(self);
    // return 1;
  }
}
pub trait QToolButton_showMenu_0<RetType> {
  fn showMenu_0(self , rsthis: & QToolButton) -> RetType;
}
impl<'a> /*trait*/ QToolButton_showMenu_0<(/*void*/)> for () {
  fn showMenu_0(self , rsthis: & QToolButton) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN11QToolButton8showMenuEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtoolbutton.h:102
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setToolButtonStyle(Qt::ToolButtonStyle)

/*

*/
impl /*struct*/ QToolButton {
  pub fn setToolButtonStyle_0<RetType, T: QToolButton_setToolButtonStyle_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setToolButtonStyle_0(self);
    // return 1;
  }
}
pub trait QToolButton_setToolButtonStyle_0<RetType> {
  fn setToolButtonStyle_0(self , rsthis: & QToolButton) -> RetType;
}
impl<'a> /*trait*/ QToolButton_setToolButtonStyle_0<(/*void*/)> for (i32) {
  fn setToolButtonStyle_0(self , rsthis: & QToolButton) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QToolButton18setToolButtonStyleEN2Qt15ToolButtonStyleE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtoolbutton.h:103
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDefaultAction(QAction *)

/*
Sets the default action to action.

If a tool button has a default action, the action defines the button's properties like text, icon, tool tip, etc.

See also defaultAction().
*/
impl /*struct*/ QToolButton {
  pub fn setDefaultAction_0<RetType, T: QToolButton_setDefaultAction_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDefaultAction_0(self);
    // return 1;
  }
}
pub trait QToolButton_setDefaultAction_0<RetType> {
  fn setDefaultAction_0(self , rsthis: & QToolButton) -> RetType;
}
impl<'a> /*trait*/ QToolButton_setDefaultAction_0<(/*void*/)> for (usize) {
  fn setDefaultAction_0(self , rsthis: & QToolButton) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QToolButton16setDefaultActionEP7QAction", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtoolbutton.h:106
// index:0
// Public Visibility=Default Availability=Available
// [-2] void triggered(QAction *)

/*
This signal is emitted when the given action is triggered.

The action may also be associated with other parts of the user interface, such as menu items and keyboard shortcuts. Sharing actions in this way helps make the user interface more consistent and is often less work to implement.
*/
impl /*struct*/ QToolButton {
  pub fn triggered_0<RetType, T: QToolButton_triggered_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.triggered_0(self);
    // return 1;
  }
}
pub trait QToolButton_triggered_0<RetType> {
  fn triggered_0(self , rsthis: & QToolButton) -> RetType;
}
impl<'a> /*trait*/ QToolButton_triggered_0<(/*void*/)> for (usize) {
  fn triggered_0(self , rsthis: & QToolButton) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QToolButton9triggeredEP7QAction", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtoolbutton.h:109
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool event(QEvent *)

/*
Reimplemented from QObject::event().
*/
impl /*struct*/ QToolButton {
  pub fn event_0<RetType, T: QToolButton_event_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.event_0(self);
    // return 1;
  }
}
pub trait QToolButton_event_0<RetType> {
  fn event_0(self , rsthis: & QToolButton) -> RetType;
}
impl<'a> /*trait*/ QToolButton_event_0<bool> for (usize) {
  fn event_0(self , rsthis: & QToolButton) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QToolButton5eventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtoolbutton.h:110
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mousePressEvent(QMouseEvent *)

/*
Reimplemented from QWidget::mousePressEvent().
*/
impl /*struct*/ QToolButton {
  pub fn mousePressEvent_0<RetType, T: QToolButton_mousePressEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mousePressEvent_0(self);
    // return 1;
  }
}
pub trait QToolButton_mousePressEvent_0<RetType> {
  fn mousePressEvent_0(self , rsthis: & QToolButton) -> RetType;
}
impl<'a> /*trait*/ QToolButton_mousePressEvent_0<(/*void*/)> for (usize) {
  fn mousePressEvent_0(self , rsthis: & QToolButton) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QToolButton15mousePressEventEP11QMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtoolbutton.h:111
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mouseReleaseEvent(QMouseEvent *)

/*
Reimplemented from QWidget::mouseReleaseEvent().
*/
impl /*struct*/ QToolButton {
  pub fn mouseReleaseEvent_0<RetType, T: QToolButton_mouseReleaseEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mouseReleaseEvent_0(self);
    // return 1;
  }
}
pub trait QToolButton_mouseReleaseEvent_0<RetType> {
  fn mouseReleaseEvent_0(self , rsthis: & QToolButton) -> RetType;
}
impl<'a> /*trait*/ QToolButton_mouseReleaseEvent_0<(/*void*/)> for (usize) {
  fn mouseReleaseEvent_0(self , rsthis: & QToolButton) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QToolButton17mouseReleaseEventEP11QMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtoolbutton.h:112
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void paintEvent(QPaintEvent *)

/*
Reimplemented from QWidget::paintEvent().

Paints the button in response to the paint event.
*/
impl /*struct*/ QToolButton {
  pub fn paintEvent_0<RetType, T: QToolButton_paintEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.paintEvent_0(self);
    // return 1;
  }
}
pub trait QToolButton_paintEvent_0<RetType> {
  fn paintEvent_0(self , rsthis: & QToolButton) -> RetType;
}
impl<'a> /*trait*/ QToolButton_paintEvent_0<(/*void*/)> for (usize) {
  fn paintEvent_0(self , rsthis: & QToolButton) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QToolButton10paintEventEP11QPaintEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtoolbutton.h:113
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void actionEvent(QActionEvent *)

/*
Reimplemented from QWidget::actionEvent().
*/
impl /*struct*/ QToolButton {
  pub fn actionEvent_0<RetType, T: QToolButton_actionEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.actionEvent_0(self);
    // return 1;
  }
}
pub trait QToolButton_actionEvent_0<RetType> {
  fn actionEvent_0(self , rsthis: & QToolButton) -> RetType;
}
impl<'a> /*trait*/ QToolButton_actionEvent_0<(/*void*/)> for (usize) {
  fn actionEvent_0(self , rsthis: & QToolButton) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QToolButton11actionEventEP12QActionEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtoolbutton.h:115
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void enterEvent(QEvent *)

/*
Reimplemented from QWidget::enterEvent().
*/
impl /*struct*/ QToolButton {
  pub fn enterEvent_0<RetType, T: QToolButton_enterEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.enterEvent_0(self);
    // return 1;
  }
}
pub trait QToolButton_enterEvent_0<RetType> {
  fn enterEvent_0(self , rsthis: & QToolButton) -> RetType;
}
impl<'a> /*trait*/ QToolButton_enterEvent_0<(/*void*/)> for (usize) {
  fn enterEvent_0(self , rsthis: & QToolButton) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QToolButton10enterEventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtoolbutton.h:116
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void leaveEvent(QEvent *)

/*
Reimplemented from QWidget::leaveEvent().
*/
impl /*struct*/ QToolButton {
  pub fn leaveEvent_0<RetType, T: QToolButton_leaveEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.leaveEvent_0(self);
    // return 1;
  }
}
pub trait QToolButton_leaveEvent_0<RetType> {
  fn leaveEvent_0(self , rsthis: & QToolButton) -> RetType;
}
impl<'a> /*trait*/ QToolButton_leaveEvent_0<(/*void*/)> for (usize) {
  fn leaveEvent_0(self , rsthis: & QToolButton) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QToolButton10leaveEventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtoolbutton.h:117
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void timerEvent(QTimerEvent *)

/*
Reimplemented from QObject::timerEvent().
*/
impl /*struct*/ QToolButton {
  pub fn timerEvent_0<RetType, T: QToolButton_timerEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.timerEvent_0(self);
    // return 1;
  }
}
pub trait QToolButton_timerEvent_0<RetType> {
  fn timerEvent_0(self , rsthis: & QToolButton) -> RetType;
}
impl<'a> /*trait*/ QToolButton_timerEvent_0<(/*void*/)> for (usize) {
  fn timerEvent_0(self , rsthis: & QToolButton) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QToolButton10timerEventEP11QTimerEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtoolbutton.h:118
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void changeEvent(QEvent *)

/*
Reimplemented from QWidget::changeEvent().
*/
impl /*struct*/ QToolButton {
  pub fn changeEvent_0<RetType, T: QToolButton_changeEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.changeEvent_0(self);
    // return 1;
  }
}
pub trait QToolButton_changeEvent_0<RetType> {
  fn changeEvent_0(self , rsthis: & QToolButton) -> RetType;
}
impl<'a> /*trait*/ QToolButton_changeEvent_0<(/*void*/)> for (usize) {
  fn changeEvent_0(self , rsthis: & QToolButton) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QToolButton11changeEventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtoolbutton.h:120
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool hitButton(const QPoint &) const

/*
Reimplemented from QAbstractButton::hitButton().
*/
impl /*struct*/ QToolButton {
  pub fn hitButton_0<RetType, T: QToolButton_hitButton_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hitButton_0(self);
    // return 1;
  }
}
pub trait QToolButton_hitButton_0<RetType> {
  fn hitButton_0(self , rsthis: & QToolButton) -> RetType;
}
impl<'a> /*trait*/ QToolButton_hitButton_0<bool> for (usize) {
  fn hitButton_0(self , rsthis: & QToolButton) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QToolButton9hitButtonERK6QPoint", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qtoolbutton.h:121
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void nextCheckState()

/*
Reimplemented from QAbstractButton::nextCheckState().
*/
impl /*struct*/ QToolButton {
  pub fn nextCheckState_0<RetType, T: QToolButton_nextCheckState_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.nextCheckState_0(self);
    // return 1;
  }
}
pub trait QToolButton_nextCheckState_0<RetType> {
  fn nextCheckState_0(self , rsthis: & QToolButton) -> RetType;
}
impl<'a> /*trait*/ QToolButton_nextCheckState_0<(/*void*/)> for () {
  fn nextCheckState_0(self , rsthis: & QToolButton) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN11QToolButton14nextCheckStateEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qtoolbutton.h:122
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void initStyleOption(QStyleOptionToolButton *) const

/*
Initialize option with the values from this QToolButton. This method is useful for subclasses when they need a QStyleOptionToolButton, but don't want to fill in all the information themselves.

See also QStyleOption::initFrom().
*/
impl /*struct*/ QToolButton {
  pub fn initStyleOption_0<RetType, T: QToolButton_initStyleOption_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.initStyleOption_0(self);
    // return 1;
  }
}
pub trait QToolButton_initStyleOption_0<RetType> {
  fn initStyleOption_0(self , rsthis: & QToolButton) -> RetType;
}
impl<'a> /*trait*/ QToolButton_initStyleOption_0<(/*void*/)> for (usize) {
  fn initStyleOption_0(self , rsthis: & QToolButton) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK11QToolButton15initStyleOptionEP22QStyleOptionToolButton", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}


/*
Describes how a menu should be popped up for tool buttons that has a menu set or contains a list of actions.


*/
pub type QToolButton__ToolButtonPopupMode = i32;
// After pressing and holding the tool button down for a certain amount of time (the timeout is style dependent, see QStyle::SH_ToolButton_PopupDelay), the menu is displayed. A typical application example is the "back" button in some web browsers's tool bars. If the user clicks it, the browser simply browses back to the previous page. If the user presses and holds the button down for a while, the tool button shows a menu containing the current history list
pub const QToolButton__DelayedPopup :QToolButton__ToolButtonPopupMode = 0;
// In this mode the tool button displays a special arrow to indicate that a menu is present. The menu is displayed when the arrow part of the button is pressed.
pub const QToolButton__MenuButtonPopup :QToolButton__ToolButtonPopupMode = 1;
// The menu is displayed, without delay, when the tool button is pressed. In this mode, the button's own action is not triggered.
pub const QToolButton__InstantPopup :QToolButton__ToolButtonPopupMode = 2;
pub fn QToolButton_ToolButtonPopupModeItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QToolButton", val);
}
pub fn QToolButton_ToolButtonPopupModeItemName_s(val: i32) ->String {
  //var nilthis *QToolButton
  //return nilthis.ToolButtonPopupModeItemName(val);
  return QToolButton_ToolButtonPopupModeItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
