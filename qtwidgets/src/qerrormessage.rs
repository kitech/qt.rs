

// mod ::widgets::QErrorMessage
// package qtwidgets
// /usr/include/qt/QtWidgets/qerrormessage.h
// #include <qerrormessage.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 2
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

// void done(int)
// func (this *QErrorMessage) InheritDone(f func(arg0 int) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "done", f)
// }

// void changeEvent(QEvent *)
// func (this *QErrorMessage) InheritChangeEvent(f func(e *qtcore.QEvent/*777 QEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "changeEvent", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QErrorMessage)=48
pub struct QErrorMessage {
  qbase: QDialog,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QErrorMessage_ITF interface {
//    QDialog_ITF
//    QErrorMessage_PTR() *QErrorMessage
//}
//func (ptr *QErrorMessage) QErrorMessage_PTR() *QErrorMessage { return ptr }

impl /*struct*/ QErrorMessage {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QErrorMessage {
    return QErrorMessage{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QErrorMessage {
//  type Target = QErrorMessageBASE;
//
//  fn deref(&self) -> &QErrorMessageBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QErrorMessageBASE> for QErrorMessage {
//  fn as_ref(& self) -> & QErrorMessageBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qerrormessage.h:55
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QErrorMessage {
  pub fn metaObject_0<RetType, T: QErrorMessage_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QErrorMessage_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QErrorMessage) -> RetType;
}
impl<'a> /*trait*/ QErrorMessage_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QErrorMessage) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QErrorMessage10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qerrormessage.h:58
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QErrorMessage(QWidget *)

/*
Constructs and installs an error handler window with the given parent.
*/
// QErrorMessage(QWidget *) ctx.fn_proto_cpp
impl /*struct*/ QErrorMessage {
  pub fn QErrorMessage_0<T: QErrorMessage_QErrorMessage_0>(value: T) -> QErrorMessage {
    let rsthis = value.QErrorMessage_0();
    return rsthis;
    // return 1;
  }
}

pub trait QErrorMessage_QErrorMessage_0 {
  fn QErrorMessage_0(self) -> QErrorMessage;
}
// QErrorMessage(QWidget *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QErrorMessage_QErrorMessage_0 for (usize) {
  fn QErrorMessage_0(self) -> QErrorMessage {
    // unsafe{_ZN13QErrorMessageC2EP7QWidget()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN13QErrorMessageC2EP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QErrorMessage{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qerrormessage.h:59
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QErrorMessage()

/*

*/
pub fn DeleteQErrorMessage(this :*mut QErrorMessage) {
    // let rv = qtrt::InvokeQtFunc6("_ZN13QErrorMessageD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 48)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qerrormessage.h:61
// index:0
// Public static Visibility=Default Availability=Available
// [8] QErrorMessage * qtHandler()

/*
Returns a pointer to a QErrorMessage object that outputs the default Qt messages. This function creates such an object, if there isn't one already.
*/
impl /*struct*/ QErrorMessage {
  pub fn qtHandler_0<RetType, T: QErrorMessage_qtHandler_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.qtHandler_0();
    // return 1;
  }
}
pub trait QErrorMessage_qtHandler_0<RetType> {
  fn qtHandler_0(self ) -> RetType;
}
impl<'a> /*trait*/ QErrorMessage_qtHandler_0<usize> for () {
  fn qtHandler_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN13QErrorMessage9qtHandlerEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qerrormessage.h:64
// index:0
// Public Visibility=Default Availability=Available
// [-2] void showMessage(const QString &)

/*
Shows the given message, message, and returns immediately. If the user has requested for the message not to be shown again, this function does nothing.

Normally, the message is displayed immediately. However, if there are pending messages, it will be queued to be displayed later.
*/
impl /*struct*/ QErrorMessage {
  pub fn showMessage_0<RetType, T: QErrorMessage_showMessage_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.showMessage_0(self);
    // return 1;
  }
}
pub trait QErrorMessage_showMessage_0<RetType> {
  fn showMessage_0(self , rsthis: & QErrorMessage) -> RetType;
}
impl<'a> /*trait*/ QErrorMessage_showMessage_0<(/*void*/)> for (usize) {
  fn showMessage_0(self , rsthis: & QErrorMessage) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QErrorMessage11showMessageERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qerrormessage.h:65
// index:1
// Public Visibility=Default Availability=Available
// [-2] void showMessage(const QString &, const QString &)

/*
Shows the given message, message, and returns immediately. If the user has requested for the message not to be shown again, this function does nothing.

Normally, the message is displayed immediately. However, if there are pending messages, it will be queued to be displayed later.
*/
impl /*struct*/ QErrorMessage {
  pub fn showMessage_1<RetType, T: QErrorMessage_showMessage_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.showMessage_1(self);
    // return 1;
  }
}
pub trait QErrorMessage_showMessage_1<RetType> {
  fn showMessage_1(self , rsthis: & QErrorMessage) -> RetType;
}
impl<'a> /*trait*/ QErrorMessage_showMessage_1<(/*void*/)> for (usize,usize) {
  fn showMessage_1(self , rsthis: & QErrorMessage) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QErrorMessage11showMessageERK7QStringS2_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qerrormessage.h:68
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void done(int)

/*
Reimplemented from QDialog::done().
*/
impl /*struct*/ QErrorMessage {
  pub fn done_0<RetType, T: QErrorMessage_done_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.done_0(self);
    // return 1;
  }
}
pub trait QErrorMessage_done_0<RetType> {
  fn done_0(self , rsthis: & QErrorMessage) -> RetType;
}
impl<'a> /*trait*/ QErrorMessage_done_0<(/*void*/)> for (i32) {
  fn done_0(self , rsthis: & QErrorMessage) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN13QErrorMessage4doneEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qerrormessage.h:69
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void changeEvent(QEvent *)

/*
Reimplemented from QWidget::changeEvent().
*/
impl /*struct*/ QErrorMessage {
  pub fn changeEvent_0<RetType, T: QErrorMessage_changeEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.changeEvent_0(self);
    // return 1;
  }
}
pub trait QErrorMessage_changeEvent_0<RetType> {
  fn changeEvent_0(self , rsthis: & QErrorMessage) -> RetType;
}
impl<'a> /*trait*/ QErrorMessage_changeEvent_0<(/*void*/)> for (usize) {
  fn changeEvent_0(self , rsthis: & QErrorMessage) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QErrorMessage11changeEventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

//  body block end

//  keep block begin

//  keep block end
