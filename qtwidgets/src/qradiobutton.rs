

// mod ::widgets::QRadioButton
// package qtwidgets
// /usr/include/qt/QtWidgets/qradiobutton.h
// #include <qradiobutton.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 30
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
// func (this *QRadioButton) InheritEvent(f func(e *qtcore.QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "event", f)
// }

// bool hitButton(const QPoint &)
// func (this *QRadioButton) InheritHitButton(f func(arg0 *qtcore.QPoint) bool) {
//  qtrt.SetAllInheritCallback(this, "hitButton", f)
// }

// void paintEvent(QPaintEvent *)
// func (this *QRadioButton) InheritPaintEvent(f func(arg0 *qtgui.QPaintEvent/*777 QPaintEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "paintEvent", f)
// }

// void mouseMoveEvent(QMouseEvent *)
// func (this *QRadioButton) InheritMouseMoveEvent(f func(arg0 *qtgui.QMouseEvent/*777 QMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mouseMoveEvent", f)
// }

// void initStyleOption(QStyleOptionButton *)
// func (this *QRadioButton) InheritInitStyleOption(f func(button *QStyleOptionButton/*777 QStyleOptionButton **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "initStyleOption", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QRadioButton)=48
pub struct QRadioButton {
  qbase: QAbstractButton,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QRadioButton_ITF interface {
//    QAbstractButton_ITF
//    QRadioButton_PTR() *QRadioButton
//}
//func (ptr *QRadioButton) QRadioButton_PTR() *QRadioButton { return ptr }

impl /*struct*/ QRadioButton {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QRadioButton {
    return QRadioButton{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QRadioButton {
//  type Target = QRadioButtonBASE;
//
//  fn deref(&self) -> &QRadioButtonBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QRadioButtonBASE> for QRadioButton {
//  fn as_ref(& self) -> & QRadioButtonBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qradiobutton.h:56
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QRadioButton {
  pub fn metaObject_0<RetType, T: QRadioButton_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QRadioButton_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QRadioButton) -> RetType;
}
impl<'a> /*trait*/ QRadioButton_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QRadioButton) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QRadioButton10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qradiobutton.h:59
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QRadioButton(QWidget *)

/*
Constructs a radio button with the given parent, but with no text or pixmap.

The parent argument is passed on to the QAbstractButton constructor.
*/
// QRadioButton(QWidget *) ctx.fn_proto_cpp
impl /*struct*/ QRadioButton {
  pub fn QRadioButton_0<T: QRadioButton_QRadioButton_0>(value: T) -> QRadioButton {
    let rsthis = value.QRadioButton_0();
    return rsthis;
    // return 1;
  }
}

pub trait QRadioButton_QRadioButton_0 {
  fn QRadioButton_0(self) -> QRadioButton;
}
// QRadioButton(QWidget *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QRadioButton_QRadioButton_0 for (usize) {
  fn QRadioButton_0(self) -> QRadioButton {
    // unsafe{_ZN12QRadioButtonC2EP7QWidget()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN12QRadioButtonC2EP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QRadioButton{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qradiobutton.h:60
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QRadioButton(const QString &, QWidget *)

/*
Constructs a radio button with the given parent, but with no text or pixmap.

The parent argument is passed on to the QAbstractButton constructor.
*/
// QRadioButton(const QString &, QWidget *) ctx.fn_proto_cpp
impl /*struct*/ QRadioButton {
  pub fn QRadioButton_1<T: QRadioButton_QRadioButton_1>(value: T) -> QRadioButton {
    let rsthis = value.QRadioButton_1();
    return rsthis;
    // return 1;
  }
}

pub trait QRadioButton_QRadioButton_1 {
  fn QRadioButton_1(self) -> QRadioButton;
}
// QRadioButton(const QString &, QWidget *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QRadioButton_QRadioButton_1 for (usize,usize) {
  fn QRadioButton_1(self) -> QRadioButton {
    // unsafe{_ZN12QRadioButtonC2ERK7QStringP7QWidget()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN12QRadioButtonC2ERK7QStringP7QWidget", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QRadioButton{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qradiobutton.h:61
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QRadioButton()

/*

*/
pub fn DeleteQRadioButton(this :*mut QRadioButton) {
    // let rv = qtrt::InvokeQtFunc6("_ZN12QRadioButtonD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 48)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qradiobutton.h:63
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QSize sizeHint() const

/*
Reimplemented from QWidget::sizeHint().
*/
impl /*struct*/ QRadioButton {
  pub fn sizeHint_0<RetType, T: QRadioButton_sizeHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sizeHint_0(self);
    // return 1;
  }
}
pub trait QRadioButton_sizeHint_0<RetType> {
  fn sizeHint_0(self , rsthis: & QRadioButton) -> RetType;
}
impl<'a> /*trait*/ QRadioButton_sizeHint_0<usize> for () {
  fn sizeHint_0(self , rsthis: & QRadioButton) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QRadioButton8sizeHintEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qradiobutton.h:64
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QSize minimumSizeHint() const

/*
Reimplemented from QWidget::minimumSizeHint().
*/
impl /*struct*/ QRadioButton {
  pub fn minimumSizeHint_0<RetType, T: QRadioButton_minimumSizeHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.minimumSizeHint_0(self);
    // return 1;
  }
}
pub trait QRadioButton_minimumSizeHint_0<RetType> {
  fn minimumSizeHint_0(self , rsthis: & QRadioButton) -> RetType;
}
impl<'a> /*trait*/ QRadioButton_minimumSizeHint_0<usize> for () {
  fn minimumSizeHint_0(self , rsthis: & QRadioButton) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QRadioButton15minimumSizeHintEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qradiobutton.h:67
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool event(QEvent *)

/*
Reimplemented from QObject::event().
*/
impl /*struct*/ QRadioButton {
  pub fn event_0<RetType, T: QRadioButton_event_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.event_0(self);
    // return 1;
  }
}
pub trait QRadioButton_event_0<RetType> {
  fn event_0(self , rsthis: & QRadioButton) -> RetType;
}
impl<'a> /*trait*/ QRadioButton_event_0<bool> for (usize) {
  fn event_0(self , rsthis: & QRadioButton) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN12QRadioButton5eventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qradiobutton.h:68
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool hitButton(const QPoint &) const

/*
Reimplemented from QAbstractButton::hitButton().
*/
impl /*struct*/ QRadioButton {
  pub fn hitButton_0<RetType, T: QRadioButton_hitButton_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hitButton_0(self);
    // return 1;
  }
}
pub trait QRadioButton_hitButton_0<RetType> {
  fn hitButton_0(self , rsthis: & QRadioButton) -> RetType;
}
impl<'a> /*trait*/ QRadioButton_hitButton_0<bool> for (usize) {
  fn hitButton_0(self , rsthis: & QRadioButton) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QRadioButton9hitButtonERK6QPoint", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qradiobutton.h:69
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void paintEvent(QPaintEvent *)

/*
Reimplemented from QWidget::paintEvent().
*/
impl /*struct*/ QRadioButton {
  pub fn paintEvent_0<RetType, T: QRadioButton_paintEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.paintEvent_0(self);
    // return 1;
  }
}
pub trait QRadioButton_paintEvent_0<RetType> {
  fn paintEvent_0(self , rsthis: & QRadioButton) -> RetType;
}
impl<'a> /*trait*/ QRadioButton_paintEvent_0<(/*void*/)> for (usize) {
  fn paintEvent_0(self , rsthis: & QRadioButton) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QRadioButton10paintEventEP11QPaintEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qradiobutton.h:70
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mouseMoveEvent(QMouseEvent *)

/*
Reimplemented from QWidget::mouseMoveEvent().
*/
impl /*struct*/ QRadioButton {
  pub fn mouseMoveEvent_0<RetType, T: QRadioButton_mouseMoveEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mouseMoveEvent_0(self);
    // return 1;
  }
}
pub trait QRadioButton_mouseMoveEvent_0<RetType> {
  fn mouseMoveEvent_0(self , rsthis: & QRadioButton) -> RetType;
}
impl<'a> /*trait*/ QRadioButton_mouseMoveEvent_0<(/*void*/)> for (usize) {
  fn mouseMoveEvent_0(self , rsthis: & QRadioButton) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QRadioButton14mouseMoveEventEP11QMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qradiobutton.h:71
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void initStyleOption(QStyleOptionButton *) const

/*
Initialize option with the values from this QRadioButton. This method is useful for subclasses when they need a QStyleOptionButton, but don't want to fill in all the information themselves.

See also QStyleOption::initFrom().
*/
impl /*struct*/ QRadioButton {
  pub fn initStyleOption_0<RetType, T: QRadioButton_initStyleOption_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.initStyleOption_0(self);
    // return 1;
  }
}
pub trait QRadioButton_initStyleOption_0<RetType> {
  fn initStyleOption_0(self , rsthis: & QRadioButton) -> RetType;
}
impl<'a> /*trait*/ QRadioButton_initStyleOption_0<(/*void*/)> for (usize) {
  fn initStyleOption_0(self , rsthis: & QRadioButton) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK12QRadioButton15initStyleOptionEP18QStyleOptionButton", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

//  body block end

//  keep block begin

//  keep block end
