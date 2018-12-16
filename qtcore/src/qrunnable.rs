

// mod ::core::QRunnable
// package qtcore
// /usr/include/qt/QtCore/qrunnable.h
// #include <qrunnable.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 6
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
#[derive(Default)] // class sizeof(QRunnable)=16
pub struct QRunnable {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QRunnable_ITF interface {
//    QRunnable_PTR() *QRunnable
//}
//func (ptr *QRunnable) QRunnable_PTR() *QRunnable { return ptr }

impl /*struct*/ QRunnable {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QRunnable {
    return QRunnable{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QRunnable {
//  type Target = QRunnableBASE;
//
//  fn deref(&self) -> &QRunnableBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QRunnableBASE> for QRunnable {
//  fn as_ref(& self) -> & QRunnableBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qrunnable.h:58
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [-2] void run()

/*
Implement this pure virtual function in your subclass.
*/
impl /*struct*/ QRunnable {
  pub fn run_0<RetType, T: QRunnable_run_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.run_0(self);
    // return 1;
  }
}
pub trait QRunnable_run_0<RetType> {
  fn run_0(self , rsthis: & QRunnable) -> RetType;
}
impl<'a> /*trait*/ QRunnable_run_0<(/*void*/)> for () {
  fn run_0(self , rsthis: & QRunnable) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN9QRunnable3runEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qrunnable.h:60
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void QRunnable()

/*
Constructs a QRunnable. Auto-deletion is enabled by default.

See also autoDelete() and setAutoDelete().
*/
// QRunnable() ctx.fn_proto_cpp
impl /*struct*/ QRunnable {
  pub fn QRunnable_0<T: QRunnable_QRunnable_0>(value: T) -> QRunnable {
    let rsthis = value.QRunnable_0();
    return rsthis;
    // return 1;
  }
}

pub trait QRunnable_QRunnable_0 {
  fn QRunnable_0(self) -> QRunnable;
}
// QRunnable() ctx.fn_proto_cpp
impl<'a> /*trait*/ QRunnable_QRunnable_0 for () {
  fn QRunnable_0(self) -> QRunnable {
    // unsafe{_ZN9QRunnableC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QRunnableC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QRunnable{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qrunnable.h:61
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QRunnable()

/*

*/
pub fn DeleteQRunnable(this :*mut QRunnable) {
    // let rv = qtrt::InvokeQtFunc6("_ZN9QRunnableD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qrunnable.h:63
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool autoDelete() const

/*
Returns true is auto-deletion is enabled; false otherwise.

If auto-deletion is enabled, QThreadPool will automatically delete this runnable after calling run(); otherwise, ownership remains with the application programmer.

See also setAutoDelete() and QThreadPool.
*/
impl /*struct*/ QRunnable {
  pub fn autoDelete_0<RetType, T: QRunnable_autoDelete_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.autoDelete_0(self);
    // return 1;
  }
}
pub trait QRunnable_autoDelete_0<RetType> {
  fn autoDelete_0(self , rsthis: & QRunnable) -> RetType;
}
impl<'a> /*trait*/ QRunnable_autoDelete_0<bool> for () {
  fn autoDelete_0(self , rsthis: & QRunnable) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QRunnable10autoDeleteEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qrunnable.h:64
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setAutoDelete(bool)

/*
Enables auto-deletion if autoDelete is true; otherwise auto-deletion is disabled.

If auto-deletion is enabled, QThreadPool will automatically delete this runnable after calling run(); otherwise, ownership remains with the application programmer.

Note that this flag must be set before calling QThreadPool::start(). Calling this function after QThreadPool::start() results in undefined behavior.

See also autoDelete() and QThreadPool.
*/
impl /*struct*/ QRunnable {
  pub fn setAutoDelete_0<RetType, T: QRunnable_setAutoDelete_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setAutoDelete_0(self);
    // return 1;
  }
}
pub trait QRunnable_setAutoDelete_0<RetType> {
  fn setAutoDelete_0(self , rsthis: & QRunnable) -> RetType;
}
impl<'a> /*trait*/ QRunnable_setAutoDelete_0<(/*void*/)> for (bool) {
  fn setAutoDelete_0(self , rsthis: & QRunnable) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN9QRunnable13setAutoDeleteEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

//  body block end

//  keep block begin

//  keep block end
