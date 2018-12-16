

// mod ::core::QMessageLogContext
// package qtcore
// /usr/include/qt/QtCore/qlogging.h
// #include <qlogging.h>
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



/*

*/
#[derive(Default)] // class sizeof(QMessageLogContext)=32
pub struct QMessageLogContext {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QMessageLogContext_ITF interface {
//    QMessageLogContext_PTR() *QMessageLogContext
//}
//func (ptr *QMessageLogContext) QMessageLogContext_PTR() *QMessageLogContext { return ptr }

impl /*struct*/ QMessageLogContext {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QMessageLogContext {
    return QMessageLogContext{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QMessageLogContext {
//  type Target = QMessageLogContextBASE;
//
//  fn deref(&self) -> &QMessageLogContextBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QMessageLogContextBASE> for QMessageLogContext {
//  fn as_ref(& self) -> & QMessageLogContextBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qlogging.h:66
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void QMessageLogContext()

/*

*/
// QMessageLogContext() ctx.fn_proto_cpp
impl /*struct*/ QMessageLogContext {
  pub fn QMessageLogContext_0<T: QMessageLogContext_QMessageLogContext_0>(value: T) -> QMessageLogContext {
    let rsthis = value.QMessageLogContext_0();
    return rsthis;
    // return 1;
  }
}

pub trait QMessageLogContext_QMessageLogContext_0 {
  fn QMessageLogContext_0(self) -> QMessageLogContext;
}
// QMessageLogContext() ctx.fn_proto_cpp
impl<'a> /*trait*/ QMessageLogContext_QMessageLogContext_0 for () {
  fn QMessageLogContext_0(self) -> QMessageLogContext {
    // unsafe{_ZN18QMessageLogContextC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN18QMessageLogContextC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QMessageLogContext{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qlogging.h:68
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void QMessageLogContext(const char *, int, const char *, const char *)

/*

*/
// QMessageLogContext(const char *, int, const char *, const char *) ctx.fn_proto_cpp
impl /*struct*/ QMessageLogContext {
  pub fn QMessageLogContext_1<T: QMessageLogContext_QMessageLogContext_1>(value: T) -> QMessageLogContext {
    let rsthis = value.QMessageLogContext_1();
    return rsthis;
    // return 1;
  }
}

pub trait QMessageLogContext_QMessageLogContext_1 {
  fn QMessageLogContext_1(self) -> QMessageLogContext;
}
// QMessageLogContext(const char *, int, const char *, const char *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QMessageLogContext_QMessageLogContext_1 for (usize,i32,usize,usize) {
  fn QMessageLogContext_1(self) -> QMessageLogContext {
    // unsafe{_ZN18QMessageLogContextC2EPKciS1_S1_()};
    let arg0 = (self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (self.2) as *const usize as usize;
    let arg3 = (self.3) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN18QMessageLogContextC2EPKciS1_S1_", 4,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QMessageLogContext{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qlogging.h:71
// index:0
// Public Visibility=Default Availability=Available
// [-2] void copy(const QMessageLogContext &)

/*

*/
impl /*struct*/ QMessageLogContext {
  pub fn copy_0<RetType, T: QMessageLogContext_copy_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.copy_0(self);
    // return 1;
  }
}
pub trait QMessageLogContext_copy_0<RetType> {
  fn copy_0(self , rsthis: & QMessageLogContext) -> RetType;
}
impl<'a> /*trait*/ QMessageLogContext_copy_0<(/*void*/)> for (usize) {
  fn copy_0(self , rsthis: & QMessageLogContext) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN18QMessageLogContext4copyERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}


pub fn DeleteQMessageLogContext(this :*mut QMessageLogContext) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN18QMessageLogContextD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
