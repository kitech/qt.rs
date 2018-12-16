

// mod ::core::QMessageLogger
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
// extern C begin: 3
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
#[derive(Default)] // class sizeof(QMessageLogger)=32
pub struct QMessageLogger {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QMessageLogger_ITF interface {
//    QMessageLogger_PTR() *QMessageLogger
//}
//func (ptr *QMessageLogger) QMessageLogger_PTR() *QMessageLogger { return ptr }

impl /*struct*/ QMessageLogger {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QMessageLogger {
    return QMessageLogger{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QMessageLogger {
//  type Target = QMessageLoggerBASE;
//
//  fn deref(&self) -> &QMessageLoggerBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QMessageLoggerBASE> for QMessageLogger {
//  fn as_ref(& self) -> & QMessageLoggerBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qlogging.h:90
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void QMessageLogger()

/*

*/
// QMessageLogger() ctx.fn_proto_cpp
impl /*struct*/ QMessageLogger {
  pub fn QMessageLogger_0<T: QMessageLogger_QMessageLogger_0>(value: T) -> QMessageLogger {
    let rsthis = value.QMessageLogger_0();
    return rsthis;
    // return 1;
  }
}

pub trait QMessageLogger_QMessageLogger_0 {
  fn QMessageLogger_0(self) -> QMessageLogger;
}
// QMessageLogger() ctx.fn_proto_cpp
impl<'a> /*trait*/ QMessageLogger_QMessageLogger_0 for () {
  fn QMessageLogger_0(self) -> QMessageLogger {
    // unsafe{_ZN14QMessageLoggerC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN14QMessageLoggerC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QMessageLogger{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qlogging.h:91
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void QMessageLogger(const char *, int, const char *)

/*

*/
// QMessageLogger(const char *, int, const char *) ctx.fn_proto_cpp
impl /*struct*/ QMessageLogger {
  pub fn QMessageLogger_1<T: QMessageLogger_QMessageLogger_1>(value: T) -> QMessageLogger {
    let rsthis = value.QMessageLogger_1();
    return rsthis;
    // return 1;
  }
}

pub trait QMessageLogger_QMessageLogger_1 {
  fn QMessageLogger_1(self) -> QMessageLogger;
}
// QMessageLogger(const char *, int, const char *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QMessageLogger_QMessageLogger_1 for (usize,i32,usize) {
  fn QMessageLogger_1(self) -> QMessageLogger {
    // unsafe{_ZN14QMessageLoggerC2EPKciS1_()};
    let arg0 = (self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (self.2) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN14QMessageLoggerC2EPKciS1_", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QMessageLogger{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qlogging.h:93
// index:2
// Public inline Visibility=Default Availability=Available
// [-2] void QMessageLogger(const char *, int, const char *, const char *)

/*

*/
// QMessageLogger(const char *, int, const char *, const char *) ctx.fn_proto_cpp
impl /*struct*/ QMessageLogger {
  pub fn QMessageLogger_2<T: QMessageLogger_QMessageLogger_2>(value: T) -> QMessageLogger {
    let rsthis = value.QMessageLogger_2();
    return rsthis;
    // return 1;
  }
}

pub trait QMessageLogger_QMessageLogger_2 {
  fn QMessageLogger_2(self) -> QMessageLogger;
}
// QMessageLogger(const char *, int, const char *, const char *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QMessageLogger_QMessageLogger_2 for (usize,i32,usize,usize) {
  fn QMessageLogger_2(self) -> QMessageLogger {
    // unsafe{_ZN14QMessageLoggerC2EPKciS1_S1_()};
    let arg0 = (self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (self.2) as *const usize as usize;
    let arg3 = (self.3) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN14QMessageLoggerC2EPKciS1_S1_", 4,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QMessageLogger{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}


pub fn DeleteQMessageLogger(this :*mut QMessageLogger) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN14QMessageLoggerD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
