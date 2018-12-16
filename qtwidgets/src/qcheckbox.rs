

// mod ::widgets::QCheckBox
// package qtwidgets
// /usr/include/qt/QtWidgets/qcheckbox.h
// #include <qcheckbox.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 55
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
// func (this *QCheckBox) InheritEvent(f func(e *qtcore.QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "event", f)
// }

// bool hitButton(const QPoint &)
// func (this *QCheckBox) InheritHitButton(f func(pos *qtcore.QPoint) bool) {
//  qtrt.SetAllInheritCallback(this, "hitButton", f)
// }

// void checkStateSet()
// func (this *QCheckBox) InheritCheckStateSet(f func() /*void*/) {
//  qtrt.SetAllInheritCallback(this, "checkStateSet", f)
// }

// void nextCheckState()
// func (this *QCheckBox) InheritNextCheckState(f func() /*void*/) {
//  qtrt.SetAllInheritCallback(this, "nextCheckState", f)
// }

// void paintEvent(QPaintEvent *)
// func (this *QCheckBox) InheritPaintEvent(f func(arg0 *qtgui.QPaintEvent/*777 QPaintEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "paintEvent", f)
// }

// void mouseMoveEvent(QMouseEvent *)
// func (this *QCheckBox) InheritMouseMoveEvent(f func(arg0 *qtgui.QMouseEvent/*777 QMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mouseMoveEvent", f)
// }

// void initStyleOption(QStyleOptionButton *)
// func (this *QCheckBox) InheritInitStyleOption(f func(option *QStyleOptionButton/*777 QStyleOptionButton **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "initStyleOption", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QCheckBox)=48
pub struct QCheckBox {
  qbase: QAbstractButton,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QCheckBox_ITF interface {
//    QAbstractButton_ITF
//    QCheckBox_PTR() *QCheckBox
//}
//func (ptr *QCheckBox) QCheckBox_PTR() *QCheckBox { return ptr }

impl /*struct*/ QCheckBox {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QCheckBox {
    return QCheckBox{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QCheckBox {
//  type Target = QCheckBoxBASE;
//
//  fn deref(&self) -> &QCheckBoxBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QCheckBoxBASE> for QCheckBox {
//  fn as_ref(& self) -> & QCheckBoxBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qcheckbox.h:56
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QCheckBox {
  pub fn metaObject_0<RetType, T: QCheckBox_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QCheckBox_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QCheckBox) -> RetType;
}
impl<'a> /*trait*/ QCheckBox_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QCheckBox) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QCheckBox10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcheckbox.h:61
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QCheckBox(QWidget *)

/*
Constructs a checkbox with the given parent, but with no text.

parent is passed on to the QAbstractButton constructor.
*/
// QCheckBox(QWidget *) ctx.fn_proto_cpp
impl /*struct*/ QCheckBox {
  pub fn QCheckBox_0<T: QCheckBox_QCheckBox_0>(value: T) -> QCheckBox {
    let rsthis = value.QCheckBox_0();
    return rsthis;
    // return 1;
  }
}

pub trait QCheckBox_QCheckBox_0 {
  fn QCheckBox_0(self) -> QCheckBox;
}
// QCheckBox(QWidget *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QCheckBox_QCheckBox_0 for (usize) {
  fn QCheckBox_0(self) -> QCheckBox {
    // unsafe{_ZN9QCheckBoxC2EP7QWidget()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QCheckBoxC2EP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QCheckBox{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcheckbox.h:62
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QCheckBox(const QString &, QWidget *)

/*
Constructs a checkbox with the given parent, but with no text.

parent is passed on to the QAbstractButton constructor.
*/
// QCheckBox(const QString &, QWidget *) ctx.fn_proto_cpp
impl /*struct*/ QCheckBox {
  pub fn QCheckBox_1<T: QCheckBox_QCheckBox_1>(value: T) -> QCheckBox {
    let rsthis = value.QCheckBox_1();
    return rsthis;
    // return 1;
  }
}

pub trait QCheckBox_QCheckBox_1 {
  fn QCheckBox_1(self) -> QCheckBox;
}
// QCheckBox(const QString &, QWidget *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QCheckBox_QCheckBox_1 for (usize,usize) {
  fn QCheckBox_1(self) -> QCheckBox {
    // unsafe{_ZN9QCheckBoxC2ERK7QStringP7QWidget()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QCheckBoxC2ERK7QStringP7QWidget", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QCheckBox{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcheckbox.h:63
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QCheckBox()

/*

*/
pub fn DeleteQCheckBox(this :*mut QCheckBox) {
    // let rv = qtrt::InvokeQtFunc6("_ZN9QCheckBoxD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 48)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qcheckbox.h:65
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QSize sizeHint() const

/*
Reimplemented from QWidget::sizeHint().
*/
impl /*struct*/ QCheckBox {
  pub fn sizeHint_0<RetType, T: QCheckBox_sizeHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sizeHint_0(self);
    // return 1;
  }
}
pub trait QCheckBox_sizeHint_0<RetType> {
  fn sizeHint_0(self , rsthis: & QCheckBox) -> RetType;
}
impl<'a> /*trait*/ QCheckBox_sizeHint_0<usize> for () {
  fn sizeHint_0(self , rsthis: & QCheckBox) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QCheckBox8sizeHintEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcheckbox.h:66
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QSize minimumSizeHint() const

/*
Reimplemented from QWidget::minimumSizeHint().
*/
impl /*struct*/ QCheckBox {
  pub fn minimumSizeHint_0<RetType, T: QCheckBox_minimumSizeHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.minimumSizeHint_0(self);
    // return 1;
  }
}
pub trait QCheckBox_minimumSizeHint_0<RetType> {
  fn minimumSizeHint_0(self , rsthis: & QCheckBox) -> RetType;
}
impl<'a> /*trait*/ QCheckBox_minimumSizeHint_0<usize> for () {
  fn minimumSizeHint_0(self , rsthis: & QCheckBox) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QCheckBox15minimumSizeHintEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcheckbox.h:68
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTristate(bool)

/*

*/
impl /*struct*/ QCheckBox {
  pub fn setTristate_0<RetType, T: QCheckBox_setTristate_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTristate_0(self);
    // return 1;
  }
}
pub trait QCheckBox_setTristate_0<RetType> {
  fn setTristate_0(self , rsthis: & QCheckBox) -> RetType;
}
impl<'a> /*trait*/ QCheckBox_setTristate_0<(/*void*/)> for (bool) {
  fn setTristate_0(self , rsthis: & QCheckBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN9QCheckBox11setTristateEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcheckbox.h:69
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isTristate() const

/*

*/
impl /*struct*/ QCheckBox {
  pub fn isTristate_0<RetType, T: QCheckBox_isTristate_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isTristate_0(self);
    // return 1;
  }
}
pub trait QCheckBox_isTristate_0<RetType> {
  fn isTristate_0(self , rsthis: & QCheckBox) -> RetType;
}
impl<'a> /*trait*/ QCheckBox_isTristate_0<bool> for () {
  fn isTristate_0(self , rsthis: & QCheckBox) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QCheckBox10isTristateEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcheckbox.h:71
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::CheckState checkState() const

/*
Returns the checkbox's check state. If you do not need tristate support, you can also use QAbstractButton::isChecked(), which returns a boolean.

See also setCheckState() and Qt::CheckState.
*/
impl /*struct*/ QCheckBox {
  pub fn checkState_0<RetType, T: QCheckBox_checkState_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.checkState_0(self);
    // return 1;
  }
}
pub trait QCheckBox_checkState_0<RetType> {
  fn checkState_0(self , rsthis: & QCheckBox) -> RetType;
}
impl<'a> /*trait*/ QCheckBox_checkState_0<i32> for () {
  fn checkState_0(self , rsthis: & QCheckBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QCheckBox10checkStateEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcheckbox.h:72
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCheckState(Qt::CheckState)

/*
Sets the checkbox's check state to state. If you do not need tristate support, you can also use QAbstractButton::setChecked(), which takes a boolean.

See also checkState() and Qt::CheckState.
*/
impl /*struct*/ QCheckBox {
  pub fn setCheckState_0<RetType, T: QCheckBox_setCheckState_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCheckState_0(self);
    // return 1;
  }
}
pub trait QCheckBox_setCheckState_0<RetType> {
  fn setCheckState_0(self , rsthis: & QCheckBox) -> RetType;
}
impl<'a> /*trait*/ QCheckBox_setCheckState_0<(/*void*/)> for (i32) {
  fn setCheckState_0(self , rsthis: & QCheckBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QCheckBox13setCheckStateEN2Qt10CheckStateE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcheckbox.h:75
// index:0
// Public Visibility=Default Availability=Available
// [-2] void stateChanged(int)

/*
This signal is emitted whenever the checkbox's state changes, i.e., whenever the user checks or unchecks it.

state contains the checkbox's new Qt::CheckState.
*/
impl /*struct*/ QCheckBox {
  pub fn stateChanged_0<RetType, T: QCheckBox_stateChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.stateChanged_0(self);
    // return 1;
  }
}
pub trait QCheckBox_stateChanged_0<RetType> {
  fn stateChanged_0(self , rsthis: & QCheckBox) -> RetType;
}
impl<'a> /*trait*/ QCheckBox_stateChanged_0<(/*void*/)> for (i32) {
  fn stateChanged_0(self , rsthis: & QCheckBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QCheckBox12stateChangedEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcheckbox.h:78
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool event(QEvent *)

/*
Reimplemented from QObject::event().
*/
impl /*struct*/ QCheckBox {
  pub fn event_0<RetType, T: QCheckBox_event_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.event_0(self);
    // return 1;
  }
}
pub trait QCheckBox_event_0<RetType> {
  fn event_0(self , rsthis: & QCheckBox) -> RetType;
}
impl<'a> /*trait*/ QCheckBox_event_0<bool> for (usize) {
  fn event_0(self , rsthis: & QCheckBox) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QCheckBox5eventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcheckbox.h:79
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool hitButton(const QPoint &) const

/*
Reimplemented from QAbstractButton::hitButton().
*/
impl /*struct*/ QCheckBox {
  pub fn hitButton_0<RetType, T: QCheckBox_hitButton_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hitButton_0(self);
    // return 1;
  }
}
pub trait QCheckBox_hitButton_0<RetType> {
  fn hitButton_0(self , rsthis: & QCheckBox) -> RetType;
}
impl<'a> /*trait*/ QCheckBox_hitButton_0<bool> for (usize) {
  fn hitButton_0(self , rsthis: & QCheckBox) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QCheckBox9hitButtonERK6QPoint", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcheckbox.h:80
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void checkStateSet()

/*
Reimplemented from QAbstractButton::checkStateSet().
*/
impl /*struct*/ QCheckBox {
  pub fn checkStateSet_0<RetType, T: QCheckBox_checkStateSet_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.checkStateSet_0(self);
    // return 1;
  }
}
pub trait QCheckBox_checkStateSet_0<RetType> {
  fn checkStateSet_0(self , rsthis: & QCheckBox) -> RetType;
}
impl<'a> /*trait*/ QCheckBox_checkStateSet_0<(/*void*/)> for () {
  fn checkStateSet_0(self , rsthis: & QCheckBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN9QCheckBox13checkStateSetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcheckbox.h:81
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void nextCheckState()

/*
Reimplemented from QAbstractButton::nextCheckState().
*/
impl /*struct*/ QCheckBox {
  pub fn nextCheckState_0<RetType, T: QCheckBox_nextCheckState_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.nextCheckState_0(self);
    // return 1;
  }
}
pub trait QCheckBox_nextCheckState_0<RetType> {
  fn nextCheckState_0(self , rsthis: & QCheckBox) -> RetType;
}
impl<'a> /*trait*/ QCheckBox_nextCheckState_0<(/*void*/)> for () {
  fn nextCheckState_0(self , rsthis: & QCheckBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN9QCheckBox14nextCheckStateEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcheckbox.h:82
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void paintEvent(QPaintEvent *)

/*
Reimplemented from QWidget::paintEvent().
*/
impl /*struct*/ QCheckBox {
  pub fn paintEvent_0<RetType, T: QCheckBox_paintEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.paintEvent_0(self);
    // return 1;
  }
}
pub trait QCheckBox_paintEvent_0<RetType> {
  fn paintEvent_0(self , rsthis: & QCheckBox) -> RetType;
}
impl<'a> /*trait*/ QCheckBox_paintEvent_0<(/*void*/)> for (usize) {
  fn paintEvent_0(self , rsthis: & QCheckBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QCheckBox10paintEventEP11QPaintEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcheckbox.h:83
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mouseMoveEvent(QMouseEvent *)

/*
Reimplemented from QWidget::mouseMoveEvent().
*/
impl /*struct*/ QCheckBox {
  pub fn mouseMoveEvent_0<RetType, T: QCheckBox_mouseMoveEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mouseMoveEvent_0(self);
    // return 1;
  }
}
pub trait QCheckBox_mouseMoveEvent_0<RetType> {
  fn mouseMoveEvent_0(self , rsthis: & QCheckBox) -> RetType;
}
impl<'a> /*trait*/ QCheckBox_mouseMoveEvent_0<(/*void*/)> for (usize) {
  fn mouseMoveEvent_0(self , rsthis: & QCheckBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QCheckBox14mouseMoveEventEP11QMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcheckbox.h:84
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void initStyleOption(QStyleOptionButton *) const

/*
Initializes option with the values from this QCheckBox. This method is useful for subclasses that require a QStyleOptionButton, but do not want to fill in all the information themselves.

See also QStyleOption::initFrom().
*/
impl /*struct*/ QCheckBox {
  pub fn initStyleOption_0<RetType, T: QCheckBox_initStyleOption_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.initStyleOption_0(self);
    // return 1;
  }
}
pub trait QCheckBox_initStyleOption_0<RetType> {
  fn initStyleOption_0(self , rsthis: & QCheckBox) -> RetType;
}
impl<'a> /*trait*/ QCheckBox_initStyleOption_0<(/*void*/)> for (usize) {
  fn initStyleOption_0(self , rsthis: & QCheckBox) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK9QCheckBox15initStyleOptionEP18QStyleOptionButton", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

//  body block end

//  keep block begin

//  keep block end
