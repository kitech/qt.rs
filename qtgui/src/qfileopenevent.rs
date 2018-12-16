

// mod ::gui::QFileOpenEvent
// package qtgui
// /usr/include/qt/QtGui/qevent.h
// #include <qevent.h>
// #include <QtGui>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 4
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



/*

*/
#[derive(Default)] // class sizeof(QFileOpenEvent)=40
pub struct QFileOpenEvent {
  qbase: QEvent,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QFileOpenEvent_ITF interface {
//    qtcore.QEvent_ITF
//    QFileOpenEvent_PTR() *QFileOpenEvent
//}
//func (ptr *QFileOpenEvent) QFileOpenEvent_PTR() *QFileOpenEvent { return ptr }

impl /*struct*/ QFileOpenEvent {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QFileOpenEvent {
    return QFileOpenEvent{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QFileOpenEvent {
//  type Target = QFileOpenEventBASE;
//
//  fn deref(&self) -> &QFileOpenEventBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QFileOpenEventBASE> for QFileOpenEvent {
//  fn as_ref(& self) -> & QFileOpenEventBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qevent.h:738
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QFileOpenEvent(const QString &)

/*

*/
// QFileOpenEvent(const QString &) ctx.fn_proto_cpp
impl /*struct*/ QFileOpenEvent {
  pub fn QFileOpenEvent_0<T: QFileOpenEvent_QFileOpenEvent_0>(value: T) -> QFileOpenEvent {
    let rsthis = value.QFileOpenEvent_0();
    return rsthis;
    // return 1;
  }
}

pub trait QFileOpenEvent_QFileOpenEvent_0 {
  fn QFileOpenEvent_0(self) -> QFileOpenEvent;
}
// QFileOpenEvent(const QString &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QFileOpenEvent_QFileOpenEvent_0 for (usize) {
  fn QFileOpenEvent_0(self) -> QFileOpenEvent {
    // unsafe{_ZN14QFileOpenEventC2ERK7QString()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN14QFileOpenEventC2ERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QFileOpenEvent{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qevent.h:739
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QFileOpenEvent(const QUrl &)

/*

*/
// QFileOpenEvent(const QUrl &) ctx.fn_proto_cpp
impl /*struct*/ QFileOpenEvent {
  pub fn QFileOpenEvent_1<T: QFileOpenEvent_QFileOpenEvent_1>(value: T) -> QFileOpenEvent {
    let rsthis = value.QFileOpenEvent_1();
    return rsthis;
    // return 1;
  }
}

pub trait QFileOpenEvent_QFileOpenEvent_1 {
  fn QFileOpenEvent_1(self) -> QFileOpenEvent;
}
// QFileOpenEvent(const QUrl &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QFileOpenEvent_QFileOpenEvent_1 for (usize) {
  fn QFileOpenEvent_1(self) -> QFileOpenEvent {
    // unsafe{_ZN14QFileOpenEventC2ERK4QUrl()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN14QFileOpenEventC2ERK4QUrl", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QFileOpenEvent{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qevent.h:740
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QFileOpenEvent()

/*

*/
pub fn DeleteQFileOpenEvent(this :*mut QFileOpenEvent) {
    // let rv = qtrt::InvokeQtFunc6("_ZN14QFileOpenEventD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 40)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qevent.h:742
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QString file() const

/*

*/
impl /*struct*/ QFileOpenEvent {
  pub fn file_0<RetType, T: QFileOpenEvent_file_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.file_0(self);
    // return 1;
  }
}
pub trait QFileOpenEvent_file_0<RetType> {
  fn file_0(self , rsthis: & QFileOpenEvent) -> RetType;
}
impl<'a> /*trait*/ QFileOpenEvent_file_0<usize> for () {
  fn file_0(self , rsthis: & QFileOpenEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QFileOpenEvent4fileEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:743
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QUrl url() const

/*

*/
impl /*struct*/ QFileOpenEvent {
  pub fn url_0<RetType, T: QFileOpenEvent_url_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.url_0(self);
    // return 1;
  }
}
pub trait QFileOpenEvent_url_0<RetType> {
  fn url_0(self , rsthis: & QFileOpenEvent) -> RetType;
}
impl<'a> /*trait*/ QFileOpenEvent_url_0<usize> for () {
  fn url_0(self , rsthis: & QFileOpenEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QFileOpenEvent3urlEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:744
// index:0
// Public Visibility=Default Availability=Available
// [1] bool openFile(QFile &, QIODevice::OpenMode) const

/*

*/
impl /*struct*/ QFileOpenEvent {
  pub fn openFile_0<RetType, T: QFileOpenEvent_openFile_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.openFile_0(self);
    // return 1;
  }
}
pub trait QFileOpenEvent_openFile_0<RetType> {
  fn openFile_0(self , rsthis: & QFileOpenEvent) -> RetType;
}
impl<'a> /*trait*/ QFileOpenEvent_openFile_0<bool> for (usize,i32) {
  fn openFile_0(self , rsthis: & QFileOpenEvent) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QFileOpenEvent8openFileER5QFile6QFlagsIN9QIODevice12OpenModeFlagEE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
