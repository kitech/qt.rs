

// mod ::core::QSocketNotifier
// package qtcore
// /usr/include/qt/QtCore/qsocketnotifier.h
// #include <qsocketnotifier.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 11
*/
// import "C"
// import "unsafe"
// import "reflect"
// import "fmt"
// import "log"
// import "github.com/kitech/qt.go/qtrt"
use std::default::Default;
use std::ops::Deref;
use super::super::qtrt;
use super::*;
//  ext block end

//  body block begin

// bool event(QEvent *)
// func (this *QSocketNotifier) InheritEvent(f func(arg0 *QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "event", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QSocketNotifier)=16
pub struct QSocketNotifier {
  qbase: QObject,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QSocketNotifier_ITF interface {
//    QObject_ITF
//    QSocketNotifier_PTR() *QSocketNotifier
//}
//func (ptr *QSocketNotifier) QSocketNotifier_PTR() *QSocketNotifier { return ptr }

impl /*struct*/ QSocketNotifier {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QSocketNotifier {
    return QSocketNotifier{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QSocketNotifier {
//  type Target = QSocketNotifierBASE;
//
//  fn deref(&self) -> &QSocketNotifierBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QSocketNotifierBASE> for QSocketNotifier {
//  fn as_ref(& self) -> & QSocketNotifierBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qsocketnotifier.h:50
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QSocketNotifier {
  pub fn metaObject_0<RetType, T: QSocketNotifier_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QSocketNotifier_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QSocketNotifier) -> RetType;
}
impl<'a> /*trait*/ QSocketNotifier_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QSocketNotifier) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QSocketNotifier10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsocketnotifier.h:56
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QSocketNotifier(qintptr, QSocketNotifier::Type, QObject *)

/*
Constructs a socket notifier with the given parent. It enables the socket, and watches for events of the given type.

It is generally advisable to explicitly enable or disable the socket notifier, especially for write notifiers.

Note for Windows users: The socket passed to QSocketNotifier will become non-blocking, even if it was created as a blocking socket.

See also setEnabled() and isEnabled().
*/
// QSocketNotifier(qintptr, QSocketNotifier::Type, QObject *) ctx.fn_proto_cpp
impl /*struct*/ QSocketNotifier {
  pub fn QSocketNotifier_0<T: QSocketNotifier_QSocketNotifier_0>(value: T) -> QSocketNotifier {
    let rsthis = value.QSocketNotifier_0();
    return rsthis;
    // return 1;
  }
}

pub trait QSocketNotifier_QSocketNotifier_0 {
  fn QSocketNotifier_0(self) -> QSocketNotifier;
}
// QSocketNotifier(qintptr, QSocketNotifier::Type, QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QSocketNotifier_QSocketNotifier_0 for (i64,i32,usize) {
  fn QSocketNotifier_0(self) -> QSocketNotifier {
    // unsafe{_ZN15QSocketNotifierC2ExNS_4TypeEP7QObject()};
    let arg0 = (&self.0) as *const i64 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN15QSocketNotifierC2ExNS_4TypeEP7QObject", 3,qtrt::FFITY_SINT64,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QSocketNotifier{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsocketnotifier.h:57
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QSocketNotifier()

/*

*/
pub fn DeleteQSocketNotifier(this :*mut QSocketNotifier) {
    // let rv = qtrt::InvokeQtFunc6("_ZN15QSocketNotifierD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qsocketnotifier.h:59
// index:0
// Public Visibility=Default Availability=Available
// [8] qintptr socket() const

/*
Returns the socket identifier specified to the constructor.

See also type().
*/
impl /*struct*/ QSocketNotifier {
  pub fn socket_0<RetType, T: QSocketNotifier_socket_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.socket_0(self);
    // return 1;
  }
}
pub trait QSocketNotifier_socket_0<RetType> {
  fn socket_0(self , rsthis: & QSocketNotifier) -> RetType;
}
impl<'a> /*trait*/ QSocketNotifier_socket_0<i64> for () {
  fn socket_0(self , rsthis: & QSocketNotifier) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QSocketNotifier6socketEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsocketnotifier.h:60
// index:0
// Public Visibility=Default Availability=Available
// [4] QSocketNotifier::Type type() const

/*
Returns the socket event type specified to the constructor.

See also socket().
*/
impl /*struct*/ QSocketNotifier {
  pub fn type__0<RetType, T: QSocketNotifier_type__0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.type__0(self);
    // return 1;
  }
}
pub trait QSocketNotifier_type__0<RetType> {
  fn type__0(self , rsthis: & QSocketNotifier) -> RetType;
}
impl<'a> /*trait*/ QSocketNotifier_type__0<i32> for () {
  fn type__0(self , rsthis: & QSocketNotifier) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QSocketNotifier4typeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsocketnotifier.h:62
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isEnabled() const

/*
Returns true if the notifier is enabled; otherwise returns false.

See also setEnabled().
*/
impl /*struct*/ QSocketNotifier {
  pub fn isEnabled_0<RetType, T: QSocketNotifier_isEnabled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isEnabled_0(self);
    // return 1;
  }
}
pub trait QSocketNotifier_isEnabled_0<RetType> {
  fn isEnabled_0(self , rsthis: & QSocketNotifier) -> RetType;
}
impl<'a> /*trait*/ QSocketNotifier_isEnabled_0<bool> for () {
  fn isEnabled_0(self , rsthis: & QSocketNotifier) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QSocketNotifier9isEnabledEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsocketnotifier.h:65
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setEnabled(bool)

/*
If enable is true, the notifier is enabled; otherwise the notifier is disabled.

The notifier is enabled by default, i.e. it emits the activated() signal whenever a socket event corresponding to its type occurs. If it is disabled, it ignores socket events (the same effect as not creating the socket notifier).

Write notifiers should normally be disabled immediately after the activated() signal has been emitted

See also isEnabled() and activated().
*/
impl /*struct*/ QSocketNotifier {
  pub fn setEnabled_0<RetType, T: QSocketNotifier_setEnabled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setEnabled_0(self);
    // return 1;
  }
}
pub trait QSocketNotifier_setEnabled_0<RetType> {
  fn setEnabled_0(self , rsthis: & QSocketNotifier) -> RetType;
}
impl<'a> /*trait*/ QSocketNotifier_setEnabled_0<(/*void*/)> for (bool) {
  fn setEnabled_0(self , rsthis: & QSocketNotifier) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN15QSocketNotifier10setEnabledEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsocketnotifier.h:71
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool event(QEvent *)

/*
Reimplemented from QObject::event().
*/
impl /*struct*/ QSocketNotifier {
  pub fn event_0<RetType, T: QSocketNotifier_event_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.event_0(self);
    // return 1;
  }
}
pub trait QSocketNotifier_event_0<RetType> {
  fn event_0(self , rsthis: & QSocketNotifier) -> RetType;
}
impl<'a> /*trait*/ QSocketNotifier_event_0<bool> for (usize) {
  fn event_0(self , rsthis: & QSocketNotifier) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN15QSocketNotifier5eventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}


/*
This enum describes the various types of events that a socket notifier can recognize. The type must be specified when constructing the socket notifier.

Note that if you need to monitor both reads and writes for the same file descriptor, you must create two socket notifiers. Note also that it is not possible to install two socket notifiers of the same type (Read, Write, Exception) on the same socket.



See also QSocketNotifier() and type().

*/
pub type QSocketNotifier__Type = i32;
// There is data to be read.
pub const QSocketNotifier__Read :QSocketNotifier__Type = 0;
// Data can be written.
pub const QSocketNotifier__Write :QSocketNotifier__Type = 1;
// An exception has occurred. We recommend against using this.
pub const QSocketNotifier__Exception :QSocketNotifier__Type = 2;
pub fn QSocketNotifier_TypeItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QSocketNotifier", val);
}
pub fn QSocketNotifier_TypeItemName_s(val: i32) ->String {
  //var nilthis *QSocketNotifier
  //return nilthis.TypeItemName(val);
  return QSocketNotifier_TypeItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
