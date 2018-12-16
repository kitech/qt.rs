

// mod ::widgets::QPushButton
// package qtwidgets
// /usr/include/qt/QtWidgets/qpushbutton.h
// #include <qpushbutton.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 96
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
// func (this *QPushButton) InheritEvent(f func(e *qtcore.QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "event", f)
// }

// void paintEvent(QPaintEvent *)
// func (this *QPushButton) InheritPaintEvent(f func(arg0 *qtgui.QPaintEvent/*777 QPaintEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "paintEvent", f)
// }

// void keyPressEvent(QKeyEvent *)
// func (this *QPushButton) InheritKeyPressEvent(f func(arg0 *qtgui.QKeyEvent/*777 QKeyEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "keyPressEvent", f)
// }

// void focusInEvent(QFocusEvent *)
// func (this *QPushButton) InheritFocusInEvent(f func(arg0 *qtgui.QFocusEvent/*777 QFocusEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "focusInEvent", f)
// }

// void focusOutEvent(QFocusEvent *)
// func (this *QPushButton) InheritFocusOutEvent(f func(arg0 *qtgui.QFocusEvent/*777 QFocusEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "focusOutEvent", f)
// }

// void initStyleOption(QStyleOptionButton *)
// func (this *QPushButton) InheritInitStyleOption(f func(option *QStyleOptionButton/*777 QStyleOptionButton **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "initStyleOption", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QPushButton)=48
pub struct QPushButton {
  qbase: QAbstractButton,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QPushButton_ITF interface {
//    QAbstractButton_ITF
//    QPushButton_PTR() *QPushButton
//}
//func (ptr *QPushButton) QPushButton_PTR() *QPushButton { return ptr }

impl /*struct*/ QPushButton {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QPushButton {
    return QPushButton{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QPushButton {
//  type Target = QPushButtonBASE;
//
//  fn deref(&self) -> &QPushButtonBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QPushButtonBASE> for QPushButton {
//  fn as_ref(& self) -> & QPushButtonBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qpushbutton.h:57
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QPushButton {
  pub fn metaObject_0<RetType, T: QPushButton_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QPushButton_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QPushButton) -> RetType;
}
impl<'a> /*trait*/ QPushButton_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QPushButton) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QPushButton10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qpushbutton.h:64
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QPushButton(QWidget *)

/*
Constructs a push button with no text and a parent.
*/
// QPushButton(QWidget *) ctx.fn_proto_cpp
impl /*struct*/ QPushButton {
  pub fn QPushButton_0<T: QPushButton_QPushButton_0>(value: T) -> QPushButton {
    let rsthis = value.QPushButton_0();
    return rsthis;
    // return 1;
  }
}

pub trait QPushButton_QPushButton_0 {
  fn QPushButton_0(self) -> QPushButton;
}
// QPushButton(QWidget *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QPushButton_QPushButton_0 for (usize) {
  fn QPushButton_0(self) -> QPushButton {
    // unsafe{_ZN11QPushButtonC2EP7QWidget()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QPushButtonC2EP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QPushButton{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qpushbutton.h:65
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QPushButton(const QString &, QWidget *)

/*
Constructs a push button with no text and a parent.
*/
// QPushButton(const QString &, QWidget *) ctx.fn_proto_cpp
impl /*struct*/ QPushButton {
  pub fn QPushButton_1<T: QPushButton_QPushButton_1>(value: T) -> QPushButton {
    let rsthis = value.QPushButton_1();
    return rsthis;
    // return 1;
  }
}

pub trait QPushButton_QPushButton_1 {
  fn QPushButton_1(self) -> QPushButton;
}
// QPushButton(const QString &, QWidget *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QPushButton_QPushButton_1 for (usize,usize) {
  fn QPushButton_1(self) -> QPushButton {
    // unsafe{_ZN11QPushButtonC2ERK7QStringP7QWidget()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QPushButtonC2ERK7QStringP7QWidget", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QPushButton{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qpushbutton.h:66
// index:2
// Public Visibility=Default Availability=Available
// [-2] void QPushButton(const QIcon &, const QString &, QWidget *)

/*
Constructs a push button with no text and a parent.
*/
// QPushButton(const QIcon &, const QString &, QWidget *) ctx.fn_proto_cpp
impl /*struct*/ QPushButton {
  pub fn QPushButton_2<T: QPushButton_QPushButton_2>(value: T) -> QPushButton {
    let rsthis = value.QPushButton_2();
    return rsthis;
    // return 1;
  }
}

pub trait QPushButton_QPushButton_2 {
  fn QPushButton_2(self) -> QPushButton;
}
// QPushButton(const QIcon &, const QString &, QWidget *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QPushButton_QPushButton_2 for (usize,usize,usize) {
  fn QPushButton_2(self) -> QPushButton {
    // unsafe{_ZN11QPushButtonC2ERK5QIconRK7QStringP7QWidget()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QPushButtonC2ERK5QIconRK7QStringP7QWidget", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QPushButton{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qpushbutton.h:67
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QPushButton()

/*

*/
pub fn DeleteQPushButton(this :*mut QPushButton) {
    // let rv = qtrt::InvokeQtFunc6("_ZN11QPushButtonD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 48)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qpushbutton.h:69
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QSize sizeHint() const

/*
Reimplemented from QWidget::sizeHint().
*/
impl /*struct*/ QPushButton {
  pub fn sizeHint_0<RetType, T: QPushButton_sizeHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sizeHint_0(self);
    // return 1;
  }
}
pub trait QPushButton_sizeHint_0<RetType> {
  fn sizeHint_0(self , rsthis: & QPushButton) -> RetType;
}
impl<'a> /*trait*/ QPushButton_sizeHint_0<usize> for () {
  fn sizeHint_0(self , rsthis: & QPushButton) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QPushButton8sizeHintEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qpushbutton.h:70
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QSize minimumSizeHint() const

/*
Reimplemented from QWidget::minimumSizeHint().
*/
impl /*struct*/ QPushButton {
  pub fn minimumSizeHint_0<RetType, T: QPushButton_minimumSizeHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.minimumSizeHint_0(self);
    // return 1;
  }
}
pub trait QPushButton_minimumSizeHint_0<RetType> {
  fn minimumSizeHint_0(self , rsthis: & QPushButton) -> RetType;
}
impl<'a> /*trait*/ QPushButton_minimumSizeHint_0<usize> for () {
  fn minimumSizeHint_0(self , rsthis: & QPushButton) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QPushButton15minimumSizeHintEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qpushbutton.h:72
// index:0
// Public Visibility=Default Availability=Available
// [1] bool autoDefault() const

/*

*/
impl /*struct*/ QPushButton {
  pub fn autoDefault_0<RetType, T: QPushButton_autoDefault_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.autoDefault_0(self);
    // return 1;
  }
}
pub trait QPushButton_autoDefault_0<RetType> {
  fn autoDefault_0(self , rsthis: & QPushButton) -> RetType;
}
impl<'a> /*trait*/ QPushButton_autoDefault_0<bool> for () {
  fn autoDefault_0(self , rsthis: & QPushButton) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QPushButton11autoDefaultEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qpushbutton.h:73
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setAutoDefault(bool)

/*

*/
impl /*struct*/ QPushButton {
  pub fn setAutoDefault_0<RetType, T: QPushButton_setAutoDefault_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setAutoDefault_0(self);
    // return 1;
  }
}
pub trait QPushButton_setAutoDefault_0<RetType> {
  fn setAutoDefault_0(self , rsthis: & QPushButton) -> RetType;
}
impl<'a> /*trait*/ QPushButton_setAutoDefault_0<(/*void*/)> for (bool) {
  fn setAutoDefault_0(self , rsthis: & QPushButton) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN11QPushButton14setAutoDefaultEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qpushbutton.h:74
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isDefault() const

/*

*/
impl /*struct*/ QPushButton {
  pub fn isDefault_0<RetType, T: QPushButton_isDefault_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isDefault_0(self);
    // return 1;
  }
}
pub trait QPushButton_isDefault_0<RetType> {
  fn isDefault_0(self , rsthis: & QPushButton) -> RetType;
}
impl<'a> /*trait*/ QPushButton_isDefault_0<bool> for () {
  fn isDefault_0(self , rsthis: & QPushButton) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QPushButton9isDefaultEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qpushbutton.h:75
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDefault(bool)

/*

*/
impl /*struct*/ QPushButton {
  pub fn setDefault_0<RetType, T: QPushButton_setDefault_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDefault_0(self);
    // return 1;
  }
}
pub trait QPushButton_setDefault_0<RetType> {
  fn setDefault_0(self , rsthis: & QPushButton) -> RetType;
}
impl<'a> /*trait*/ QPushButton_setDefault_0<(/*void*/)> for (bool) {
  fn setDefault_0(self , rsthis: & QPushButton) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN11QPushButton10setDefaultEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qpushbutton.h:78
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setMenu(QMenu *)

/*
Associates the popup menu menu with this push button. This turns the button into a menu button, which in some styles will produce a small triangle to the right of the button's text.

Ownership of the menu is not transferred to the push button.



A push button with popup menus shown in the Fusion widget style.

See also menu().
*/
impl /*struct*/ QPushButton {
  pub fn setMenu_0<RetType, T: QPushButton_setMenu_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMenu_0(self);
    // return 1;
  }
}
pub trait QPushButton_setMenu_0<RetType> {
  fn setMenu_0(self , rsthis: & QPushButton) -> RetType;
}
impl<'a> /*trait*/ QPushButton_setMenu_0<(/*void*/)> for (usize) {
  fn setMenu_0(self , rsthis: & QPushButton) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QPushButton7setMenuEP5QMenu", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qpushbutton.h:79
// index:0
// Public Visibility=Default Availability=Available
// [8] QMenu * menu() const

/*
Returns the button's associated popup menu or 0 if no popup menu has been set.

See also setMenu().
*/
impl /*struct*/ QPushButton {
  pub fn menu_0<RetType, T: QPushButton_menu_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.menu_0(self);
    // return 1;
  }
}
pub trait QPushButton_menu_0<RetType> {
  fn menu_0(self , rsthis: & QPushButton) -> RetType;
}
impl<'a> /*trait*/ QPushButton_menu_0<usize> for () {
  fn menu_0(self , rsthis: & QPushButton) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QPushButton4menuEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qpushbutton.h:82
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFlat(bool)

/*

*/
impl /*struct*/ QPushButton {
  pub fn setFlat_0<RetType, T: QPushButton_setFlat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFlat_0(self);
    // return 1;
  }
}
pub trait QPushButton_setFlat_0<RetType> {
  fn setFlat_0(self , rsthis: & QPushButton) -> RetType;
}
impl<'a> /*trait*/ QPushButton_setFlat_0<(/*void*/)> for (bool) {
  fn setFlat_0(self , rsthis: & QPushButton) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN11QPushButton7setFlatEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qpushbutton.h:83
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isFlat() const

/*

*/
impl /*struct*/ QPushButton {
  pub fn isFlat_0<RetType, T: QPushButton_isFlat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isFlat_0(self);
    // return 1;
  }
}
pub trait QPushButton_isFlat_0<RetType> {
  fn isFlat_0(self , rsthis: & QPushButton) -> RetType;
}
impl<'a> /*trait*/ QPushButton_isFlat_0<bool> for () {
  fn isFlat_0(self , rsthis: & QPushButton) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QPushButton6isFlatEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qpushbutton.h:87
// index:0
// Public Visibility=Default Availability=Available
// [-2] void showMenu()

/*
Shows (pops up) the associated popup menu. If there is no such menu, this function does nothing. This function does not return until the popup menu has been closed by the user.
*/
impl /*struct*/ QPushButton {
  pub fn showMenu_0<RetType, T: QPushButton_showMenu_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.showMenu_0(self);
    // return 1;
  }
}
pub trait QPushButton_showMenu_0<RetType> {
  fn showMenu_0(self , rsthis: & QPushButton) -> RetType;
}
impl<'a> /*trait*/ QPushButton_showMenu_0<(/*void*/)> for () {
  fn showMenu_0(self , rsthis: & QPushButton) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN11QPushButton8showMenuEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qpushbutton.h:91
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool event(QEvent *)

/*
Reimplemented from QObject::event().
*/
impl /*struct*/ QPushButton {
  pub fn event_0<RetType, T: QPushButton_event_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.event_0(self);
    // return 1;
  }
}
pub trait QPushButton_event_0<RetType> {
  fn event_0(self , rsthis: & QPushButton) -> RetType;
}
impl<'a> /*trait*/ QPushButton_event_0<bool> for (usize) {
  fn event_0(self , rsthis: & QPushButton) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QPushButton5eventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qpushbutton.h:92
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void paintEvent(QPaintEvent *)

/*
Reimplemented from QWidget::paintEvent().
*/
impl /*struct*/ QPushButton {
  pub fn paintEvent_0<RetType, T: QPushButton_paintEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.paintEvent_0(self);
    // return 1;
  }
}
pub trait QPushButton_paintEvent_0<RetType> {
  fn paintEvent_0(self , rsthis: & QPushButton) -> RetType;
}
impl<'a> /*trait*/ QPushButton_paintEvent_0<(/*void*/)> for (usize) {
  fn paintEvent_0(self , rsthis: & QPushButton) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QPushButton10paintEventEP11QPaintEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qpushbutton.h:93
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void keyPressEvent(QKeyEvent *)

/*
Reimplemented from QWidget::keyPressEvent().
*/
impl /*struct*/ QPushButton {
  pub fn keyPressEvent_0<RetType, T: QPushButton_keyPressEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.keyPressEvent_0(self);
    // return 1;
  }
}
pub trait QPushButton_keyPressEvent_0<RetType> {
  fn keyPressEvent_0(self , rsthis: & QPushButton) -> RetType;
}
impl<'a> /*trait*/ QPushButton_keyPressEvent_0<(/*void*/)> for (usize) {
  fn keyPressEvent_0(self , rsthis: & QPushButton) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QPushButton13keyPressEventEP9QKeyEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qpushbutton.h:94
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void focusInEvent(QFocusEvent *)

/*
Reimplemented from QWidget::focusInEvent().
*/
impl /*struct*/ QPushButton {
  pub fn focusInEvent_0<RetType, T: QPushButton_focusInEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.focusInEvent_0(self);
    // return 1;
  }
}
pub trait QPushButton_focusInEvent_0<RetType> {
  fn focusInEvent_0(self , rsthis: & QPushButton) -> RetType;
}
impl<'a> /*trait*/ QPushButton_focusInEvent_0<(/*void*/)> for (usize) {
  fn focusInEvent_0(self , rsthis: & QPushButton) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QPushButton12focusInEventEP11QFocusEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qpushbutton.h:95
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void focusOutEvent(QFocusEvent *)

/*
Reimplemented from QWidget::focusOutEvent().
*/
impl /*struct*/ QPushButton {
  pub fn focusOutEvent_0<RetType, T: QPushButton_focusOutEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.focusOutEvent_0(self);
    // return 1;
  }
}
pub trait QPushButton_focusOutEvent_0<RetType> {
  fn focusOutEvent_0(self , rsthis: & QPushButton) -> RetType;
}
impl<'a> /*trait*/ QPushButton_focusOutEvent_0<(/*void*/)> for (usize) {
  fn focusOutEvent_0(self , rsthis: & QPushButton) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QPushButton13focusOutEventEP11QFocusEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qpushbutton.h:96
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void initStyleOption(QStyleOptionButton *) const

/*
Initialize option with the values from this QPushButton. This method is useful for subclasses when they need a QStyleOptionButton, but don't want to fill in all the information themselves.

See also QStyleOption::initFrom().
*/
impl /*struct*/ QPushButton {
  pub fn initStyleOption_0<RetType, T: QPushButton_initStyleOption_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.initStyleOption_0(self);
    // return 1;
  }
}
pub trait QPushButton_initStyleOption_0<RetType> {
  fn initStyleOption_0(self , rsthis: & QPushButton) -> RetType;
}
impl<'a> /*trait*/ QPushButton_initStyleOption_0<(/*void*/)> for (usize) {
  fn initStyleOption_0(self , rsthis: & QPushButton) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK11QPushButton15initStyleOptionEP18QStyleOptionButton", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

//  body block end

//  keep block begin

//  keep block end
