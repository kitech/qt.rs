

// mod ::widgets::QCommandLinkButton
// package qtwidgets
// /usr/include/qt/QtWidgets/qcommandlinkbutton.h
// #include <qcommandlinkbutton.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 22
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

// QSize sizeHint()
// func (this *QCommandLinkButton) InheritSizeHint(f func() unsafe.Pointer) {
//  qtrt.SetAllInheritCallback(this, "sizeHint", f)
// }

// int heightForWidth(int)
// func (this *QCommandLinkButton) InheritHeightForWidth(f func(arg0 int) int) {
//  qtrt.SetAllInheritCallback(this, "heightForWidth", f)
// }

// QSize minimumSizeHint()
// func (this *QCommandLinkButton) InheritMinimumSizeHint(f func() unsafe.Pointer) {
//  qtrt.SetAllInheritCallback(this, "minimumSizeHint", f)
// }

// bool event(QEvent *)
// func (this *QCommandLinkButton) InheritEvent(f func(e *qtcore.QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "event", f)
// }

// void paintEvent(QPaintEvent *)
// func (this *QCommandLinkButton) InheritPaintEvent(f func(arg0 *qtgui.QPaintEvent/*777 QPaintEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "paintEvent", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QCommandLinkButton)=48
pub struct QCommandLinkButton {
  qbase: QPushButton,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QCommandLinkButton_ITF interface {
//    QPushButton_ITF
//    QCommandLinkButton_PTR() *QCommandLinkButton
//}
//func (ptr *QCommandLinkButton) QCommandLinkButton_PTR() *QCommandLinkButton { return ptr }

impl /*struct*/ QCommandLinkButton {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QCommandLinkButton {
    return QCommandLinkButton{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QCommandLinkButton {
//  type Target = QCommandLinkButtonBASE;
//
//  fn deref(&self) -> &QCommandLinkButtonBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QCommandLinkButtonBASE> for QCommandLinkButton {
//  fn as_ref(& self) -> & QCommandLinkButtonBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qcommandlinkbutton.h:55
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QCommandLinkButton {
  pub fn metaObject_0<RetType, T: QCommandLinkButton_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QCommandLinkButton_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QCommandLinkButton) -> RetType;
}
impl<'a> /*trait*/ QCommandLinkButton_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QCommandLinkButton) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QCommandLinkButton10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcommandlinkbutton.h:61
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QCommandLinkButton(QWidget *)

/*
Constructs a command link with no text and a parent.
*/
// QCommandLinkButton(QWidget *) ctx.fn_proto_cpp
impl /*struct*/ QCommandLinkButton {
  pub fn QCommandLinkButton_0<T: QCommandLinkButton_QCommandLinkButton_0>(value: T) -> QCommandLinkButton {
    let rsthis = value.QCommandLinkButton_0();
    return rsthis;
    // return 1;
  }
}

pub trait QCommandLinkButton_QCommandLinkButton_0 {
  fn QCommandLinkButton_0(self) -> QCommandLinkButton;
}
// QCommandLinkButton(QWidget *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QCommandLinkButton_QCommandLinkButton_0 for (usize) {
  fn QCommandLinkButton_0(self) -> QCommandLinkButton {
    // unsafe{_ZN18QCommandLinkButtonC2EP7QWidget()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN18QCommandLinkButtonC2EP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QCommandLinkButton{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcommandlinkbutton.h:62
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QCommandLinkButton(const QString &, QWidget *)

/*
Constructs a command link with no text and a parent.
*/
// QCommandLinkButton(const QString &, QWidget *) ctx.fn_proto_cpp
impl /*struct*/ QCommandLinkButton {
  pub fn QCommandLinkButton_1<T: QCommandLinkButton_QCommandLinkButton_1>(value: T) -> QCommandLinkButton {
    let rsthis = value.QCommandLinkButton_1();
    return rsthis;
    // return 1;
  }
}

pub trait QCommandLinkButton_QCommandLinkButton_1 {
  fn QCommandLinkButton_1(self) -> QCommandLinkButton;
}
// QCommandLinkButton(const QString &, QWidget *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QCommandLinkButton_QCommandLinkButton_1 for (usize,usize) {
  fn QCommandLinkButton_1(self) -> QCommandLinkButton {
    // unsafe{_ZN18QCommandLinkButtonC2ERK7QStringP7QWidget()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN18QCommandLinkButtonC2ERK7QStringP7QWidget", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QCommandLinkButton{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcommandlinkbutton.h:63
// index:2
// Public Visibility=Default Availability=Available
// [-2] void QCommandLinkButton(const QString &, const QString &, QWidget *)

/*
Constructs a command link with no text and a parent.
*/
// QCommandLinkButton(const QString &, const QString &, QWidget *) ctx.fn_proto_cpp
impl /*struct*/ QCommandLinkButton {
  pub fn QCommandLinkButton_2<T: QCommandLinkButton_QCommandLinkButton_2>(value: T) -> QCommandLinkButton {
    let rsthis = value.QCommandLinkButton_2();
    return rsthis;
    // return 1;
  }
}

pub trait QCommandLinkButton_QCommandLinkButton_2 {
  fn QCommandLinkButton_2(self) -> QCommandLinkButton;
}
// QCommandLinkButton(const QString &, const QString &, QWidget *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QCommandLinkButton_QCommandLinkButton_2 for (usize,usize,usize) {
  fn QCommandLinkButton_2(self) -> QCommandLinkButton {
    // unsafe{_ZN18QCommandLinkButtonC2ERK7QStringS2_P7QWidget()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN18QCommandLinkButtonC2ERK7QStringS2_P7QWidget", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QCommandLinkButton{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcommandlinkbutton.h:64
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QCommandLinkButton()

/*

*/
pub fn DeleteQCommandLinkButton(this :*mut QCommandLinkButton) {
    // let rv = qtrt::InvokeQtFunc6("_ZN18QCommandLinkButtonD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 48)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qcommandlinkbutton.h:66
// index:0
// Public Visibility=Default Availability=Available
// [8] QString description() const

/*

*/
impl /*struct*/ QCommandLinkButton {
  pub fn description_0<RetType, T: QCommandLinkButton_description_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.description_0(self);
    // return 1;
  }
}
pub trait QCommandLinkButton_description_0<RetType> {
  fn description_0(self , rsthis: & QCommandLinkButton) -> RetType;
}
impl<'a> /*trait*/ QCommandLinkButton_description_0<usize> for () {
  fn description_0(self , rsthis: & QCommandLinkButton) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QCommandLinkButton11descriptionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcommandlinkbutton.h:67
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDescription(const QString &)

/*

*/
impl /*struct*/ QCommandLinkButton {
  pub fn setDescription_0<RetType, T: QCommandLinkButton_setDescription_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDescription_0(self);
    // return 1;
  }
}
pub trait QCommandLinkButton_setDescription_0<RetType> {
  fn setDescription_0(self , rsthis: & QCommandLinkButton) -> RetType;
}
impl<'a> /*trait*/ QCommandLinkButton_setDescription_0<(/*void*/)> for (usize) {
  fn setDescription_0(self , rsthis: & QCommandLinkButton) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN18QCommandLinkButton14setDescriptionERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcommandlinkbutton.h:70
// index:0
// Protected virtual Visibility=Default Availability=Available
// [8] QSize sizeHint() const

/*
Reimplemented from QWidget::sizeHint().
*/
impl /*struct*/ QCommandLinkButton {
  pub fn sizeHint_0<RetType, T: QCommandLinkButton_sizeHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sizeHint_0(self);
    // return 1;
  }
}
pub trait QCommandLinkButton_sizeHint_0<RetType> {
  fn sizeHint_0(self , rsthis: & QCommandLinkButton) -> RetType;
}
impl<'a> /*trait*/ QCommandLinkButton_sizeHint_0<usize> for () {
  fn sizeHint_0(self , rsthis: & QCommandLinkButton) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QCommandLinkButton8sizeHintEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcommandlinkbutton.h:71
// index:0
// Protected virtual Visibility=Default Availability=Available
// [4] int heightForWidth(int) const

/*
Reimplemented from QWidget::heightForWidth().
*/
impl /*struct*/ QCommandLinkButton {
  pub fn heightForWidth_0<RetType, T: QCommandLinkButton_heightForWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.heightForWidth_0(self);
    // return 1;
  }
}
pub trait QCommandLinkButton_heightForWidth_0<RetType> {
  fn heightForWidth_0(self , rsthis: & QCommandLinkButton) -> RetType;
}
impl<'a> /*trait*/ QCommandLinkButton_heightForWidth_0<i32> for (i32) {
  fn heightForWidth_0(self , rsthis: & QCommandLinkButton) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QCommandLinkButton14heightForWidthEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcommandlinkbutton.h:72
// index:0
// Protected virtual Visibility=Default Availability=Available
// [8] QSize minimumSizeHint() const

/*
Reimplemented from QWidget::minimumSizeHint().
*/
impl /*struct*/ QCommandLinkButton {
  pub fn minimumSizeHint_0<RetType, T: QCommandLinkButton_minimumSizeHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.minimumSizeHint_0(self);
    // return 1;
  }
}
pub trait QCommandLinkButton_minimumSizeHint_0<RetType> {
  fn minimumSizeHint_0(self , rsthis: & QCommandLinkButton) -> RetType;
}
impl<'a> /*trait*/ QCommandLinkButton_minimumSizeHint_0<usize> for () {
  fn minimumSizeHint_0(self , rsthis: & QCommandLinkButton) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QCommandLinkButton15minimumSizeHintEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcommandlinkbutton.h:73
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool event(QEvent *)

/*
Reimplemented from QObject::event().
*/
impl /*struct*/ QCommandLinkButton {
  pub fn event_0<RetType, T: QCommandLinkButton_event_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.event_0(self);
    // return 1;
  }
}
pub trait QCommandLinkButton_event_0<RetType> {
  fn event_0(self , rsthis: & QCommandLinkButton) -> RetType;
}
impl<'a> /*trait*/ QCommandLinkButton_event_0<bool> for (usize) {
  fn event_0(self , rsthis: & QCommandLinkButton) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN18QCommandLinkButton5eventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcommandlinkbutton.h:74
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void paintEvent(QPaintEvent *)

/*
Reimplemented from QWidget::paintEvent().
*/
impl /*struct*/ QCommandLinkButton {
  pub fn paintEvent_0<RetType, T: QCommandLinkButton_paintEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.paintEvent_0(self);
    // return 1;
  }
}
pub trait QCommandLinkButton_paintEvent_0<RetType> {
  fn paintEvent_0(self , rsthis: & QCommandLinkButton) -> RetType;
}
impl<'a> /*trait*/ QCommandLinkButton_paintEvent_0<(/*void*/)> for (usize) {
  fn paintEvent_0(self , rsthis: & QCommandLinkButton) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN18QCommandLinkButton10paintEventEP11QPaintEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

//  body block end

//  keep block begin

//  keep block end
