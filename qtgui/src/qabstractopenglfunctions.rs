

// mod ::gui::QAbstractOpenGLFunctions
// package qtgui
// /usr/include/qt/QtGui/qopenglversionfunctions.h
// #include <qopenglversionfunctions.h>
// #include <QtGui>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 17
*/
// import "C"
// import "unsafe"
// import "reflect"
// import "fmt"
// import "log"
// import "github.com/kitech/qt.go/qtrt"
// import "github.com/kitech/qt.go/qtcore"
use qtcore::*; // super::super::%!s(MISSING)::*;
use std::default::Default;
use std::ops::Deref;
use qtrt; // super::super::qtrt;
use super::*;
//  ext block end

//  body block begin

// bool isInitialized()
// func (this *QAbstractOpenGLFunctions) InheritIsInitialized(f func() bool) {
//  qtrt.SetAllInheritCallback(this, "isInitialized", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QAbstractOpenGLFunctions)=16
pub struct QAbstractOpenGLFunctions {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QAbstractOpenGLFunctions_ITF interface {
//    QAbstractOpenGLFunctions_PTR() *QAbstractOpenGLFunctions
//}
//func (ptr *QAbstractOpenGLFunctions) QAbstractOpenGLFunctions_PTR() *QAbstractOpenGLFunctions { return ptr }

impl /*struct*/ QAbstractOpenGLFunctions {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QAbstractOpenGLFunctions {
    return QAbstractOpenGLFunctions{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QAbstractOpenGLFunctions {
//  type Target = QAbstractOpenGLFunctionsBASE;
//
//  fn deref(&self) -> &QAbstractOpenGLFunctionsBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QAbstractOpenGLFunctionsBASE> for QAbstractOpenGLFunctions {
//  fn as_ref(& self) -> & QAbstractOpenGLFunctionsBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qopenglversionfunctions.h:213
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QAbstractOpenGLFunctions()

/*

*/
pub fn DeleteQAbstractOpenGLFunctions(this :*mut QAbstractOpenGLFunctions) {
    // let rv = qtrt::InvokeQtFunc6("_ZN24QAbstractOpenGLFunctionsD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qopenglversionfunctions.h:215
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool initializeOpenGLFunctions()

/*

*/
impl /*struct*/ QAbstractOpenGLFunctions {
  pub fn initializeOpenGLFunctions_0<RetType, T: QAbstractOpenGLFunctions_initializeOpenGLFunctions_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.initializeOpenGLFunctions_0(self);
    // return 1;
  }
}
pub trait QAbstractOpenGLFunctions_initializeOpenGLFunctions_0<RetType> {
  fn initializeOpenGLFunctions_0(self , rsthis: & QAbstractOpenGLFunctions) -> RetType;
}
impl<'a> /*trait*/ QAbstractOpenGLFunctions_initializeOpenGLFunctions_0<bool> for () {
  fn initializeOpenGLFunctions_0(self , rsthis: & QAbstractOpenGLFunctions) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN24QAbstractOpenGLFunctions25initializeOpenGLFunctionsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qopenglversionfunctions.h:220
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void QAbstractOpenGLFunctions()

/*

*/
// QAbstractOpenGLFunctions() ctx.fn_proto_cpp
impl /*struct*/ QAbstractOpenGLFunctions {
  pub fn QAbstractOpenGLFunctions_0<T: QAbstractOpenGLFunctions_QAbstractOpenGLFunctions_0>(value: T) -> QAbstractOpenGLFunctions {
    let rsthis = value.QAbstractOpenGLFunctions_0();
    return rsthis;
    // return 1;
  }
}

pub trait QAbstractOpenGLFunctions_QAbstractOpenGLFunctions_0 {
  fn QAbstractOpenGLFunctions_0(self) -> QAbstractOpenGLFunctions;
}
// QAbstractOpenGLFunctions() ctx.fn_proto_cpp
impl<'a> /*trait*/ QAbstractOpenGLFunctions_QAbstractOpenGLFunctions_0 for () {
  fn QAbstractOpenGLFunctions_0(self) -> QAbstractOpenGLFunctions {
    // unsafe{_ZN24QAbstractOpenGLFunctionsC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN24QAbstractOpenGLFunctionsC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QAbstractOpenGLFunctions{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qopenglversionfunctions.h:223
// index:0
// Protected Visibility=Default Availability=Available
// [1] bool isInitialized() const

/*

*/
impl /*struct*/ QAbstractOpenGLFunctions {
  pub fn isInitialized_0<RetType, T: QAbstractOpenGLFunctions_isInitialized_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isInitialized_0(self);
    // return 1;
  }
}
pub trait QAbstractOpenGLFunctions_isInitialized_0<RetType> {
  fn isInitialized_0(self , rsthis: & QAbstractOpenGLFunctions) -> RetType;
}
impl<'a> /*trait*/ QAbstractOpenGLFunctions_isInitialized_0<bool> for () {
  fn isInitialized_0(self , rsthis: & QAbstractOpenGLFunctions) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK24QAbstractOpenGLFunctions13isInitializedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
