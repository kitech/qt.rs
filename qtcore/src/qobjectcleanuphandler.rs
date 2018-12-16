

// mod ::core::QObjectCleanupHandler
// package qtcore
// /usr/include/qt/QtCore/qobjectcleanuphandler.h
// #include <qobjectcleanuphandler.h>
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
use qtrt; // super::super::qtrt;
use super::*;
//  ext block end

//  body block begin



/*

*/
#[derive(Default)] // class sizeof(QObjectCleanupHandler)=24
pub struct QObjectCleanupHandler {
  qbase: QObject,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QObjectCleanupHandler_ITF interface {
//    QObject_ITF
//    QObjectCleanupHandler_PTR() *QObjectCleanupHandler
//}
//func (ptr *QObjectCleanupHandler) QObjectCleanupHandler_PTR() *QObjectCleanupHandler { return ptr }

impl /*struct*/ QObjectCleanupHandler {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QObjectCleanupHandler {
    return QObjectCleanupHandler{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QObjectCleanupHandler {
//  type Target = QObjectCleanupHandlerBASE;
//
//  fn deref(&self) -> &QObjectCleanupHandlerBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QObjectCleanupHandlerBASE> for QObjectCleanupHandler {
//  fn as_ref(& self) -> & QObjectCleanupHandlerBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qobjectcleanuphandler.h:50
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QObjectCleanupHandler {
  pub fn metaObject_0<RetType, T: QObjectCleanupHandler_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QObjectCleanupHandler_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QObjectCleanupHandler) -> RetType;
}
impl<'a> /*trait*/ QObjectCleanupHandler_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QObjectCleanupHandler) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK21QObjectCleanupHandler10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qobjectcleanuphandler.h:53
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QObjectCleanupHandler()

/*
Constructs an empty QObjectCleanupHandler.
*/
// QObjectCleanupHandler() ctx.fn_proto_cpp
impl /*struct*/ QObjectCleanupHandler {
  pub fn QObjectCleanupHandler_0<T: QObjectCleanupHandler_QObjectCleanupHandler_0>(value: T) -> QObjectCleanupHandler {
    let rsthis = value.QObjectCleanupHandler_0();
    return rsthis;
    // return 1;
  }
}

pub trait QObjectCleanupHandler_QObjectCleanupHandler_0 {
  fn QObjectCleanupHandler_0(self) -> QObjectCleanupHandler;
}
// QObjectCleanupHandler() ctx.fn_proto_cpp
impl<'a> /*trait*/ QObjectCleanupHandler_QObjectCleanupHandler_0 for () {
  fn QObjectCleanupHandler_0(self) -> QObjectCleanupHandler {
    // unsafe{_ZN21QObjectCleanupHandlerC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN21QObjectCleanupHandlerC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QObjectCleanupHandler{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qobjectcleanuphandler.h:54
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QObjectCleanupHandler()

/*

*/
pub fn DeleteQObjectCleanupHandler(this :*mut QObjectCleanupHandler) {
    // let rv = qtrt::InvokeQtFunc6("_ZN21QObjectCleanupHandlerD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 24)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qobjectcleanuphandler.h:56
// index:0
// Public Visibility=Default Availability=Available
// [8] QObject * add(QObject *)

/*
Adds object to this cleanup handler and returns the pointer to the object.

See also remove().
*/
impl /*struct*/ QObjectCleanupHandler {
  pub fn add_0<RetType, T: QObjectCleanupHandler_add_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.add_0(self);
    // return 1;
  }
}
pub trait QObjectCleanupHandler_add_0<RetType> {
  fn add_0(self , rsthis: & QObjectCleanupHandler) -> RetType;
}
impl<'a> /*trait*/ QObjectCleanupHandler_add_0<usize> for (usize) {
  fn add_0(self , rsthis: & QObjectCleanupHandler) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN21QObjectCleanupHandler3addEP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qobjectcleanuphandler.h:57
// index:0
// Public Visibility=Default Availability=Available
// [-2] void remove(QObject *)

/*
Removes the object from this cleanup handler. The object will not be destroyed.

See also add().
*/
impl /*struct*/ QObjectCleanupHandler {
  pub fn remove_0<RetType, T: QObjectCleanupHandler_remove_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.remove_0(self);
    // return 1;
  }
}
pub trait QObjectCleanupHandler_remove_0<RetType> {
  fn remove_0(self , rsthis: & QObjectCleanupHandler) -> RetType;
}
impl<'a> /*trait*/ QObjectCleanupHandler_remove_0<(/*void*/)> for (usize) {
  fn remove_0(self , rsthis: & QObjectCleanupHandler) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN21QObjectCleanupHandler6removeEP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qobjectcleanuphandler.h:58
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isEmpty() const

/*
Returns true if this cleanup handler is empty or if all objects in this cleanup handler have been destroyed; otherwise return false.

See also add(), remove(), and clear().
*/
impl /*struct*/ QObjectCleanupHandler {
  pub fn isEmpty_0<RetType, T: QObjectCleanupHandler_isEmpty_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isEmpty_0(self);
    // return 1;
  }
}
pub trait QObjectCleanupHandler_isEmpty_0<RetType> {
  fn isEmpty_0(self , rsthis: & QObjectCleanupHandler) -> RetType;
}
impl<'a> /*trait*/ QObjectCleanupHandler_isEmpty_0<bool> for () {
  fn isEmpty_0(self , rsthis: & QObjectCleanupHandler) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK21QObjectCleanupHandler7isEmptyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qobjectcleanuphandler.h:59
// index:0
// Public Visibility=Default Availability=Available
// [-2] void clear()

/*
Deletes all objects in this cleanup handler. The cleanup handler becomes empty.

See also isEmpty().
*/
impl /*struct*/ QObjectCleanupHandler {
  pub fn clear_0<RetType, T: QObjectCleanupHandler_clear_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.clear_0(self);
    // return 1;
  }
}
pub trait QObjectCleanupHandler_clear_0<RetType> {
  fn clear_0(self , rsthis: & QObjectCleanupHandler) -> RetType;
}
impl<'a> /*trait*/ QObjectCleanupHandler_clear_0<(/*void*/)> for () {
  fn clear_0(self , rsthis: & QObjectCleanupHandler) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN21QObjectCleanupHandler5clearEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

//  body block end

//  keep block begin

//  keep block end
