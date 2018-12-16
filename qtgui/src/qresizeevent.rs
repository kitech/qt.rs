

// mod ::gui::QResizeEvent
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
// extern C begin: 3
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
#[derive(Default)] // class sizeof(QResizeEvent)=40
pub struct QResizeEvent {
  qbase: QEvent,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QResizeEvent_ITF interface {
//    qtcore.QEvent_ITF
//    QResizeEvent_PTR() *QResizeEvent
//}
//func (ptr *QResizeEvent) QResizeEvent_PTR() *QResizeEvent { return ptr }

impl /*struct*/ QResizeEvent {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QResizeEvent {
    return QResizeEvent{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QResizeEvent {
//  type Target = QResizeEventBASE;
//
//  fn deref(&self) -> &QResizeEventBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QResizeEventBASE> for QResizeEvent {
//  fn as_ref(& self) -> & QResizeEventBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qevent.h:463
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QResizeEvent(const QSize &, const QSize &)

/*

*/
// QResizeEvent(const QSize &, const QSize &) ctx.fn_proto_cpp
impl /*struct*/ QResizeEvent {
  pub fn QResizeEvent_0<T: QResizeEvent_QResizeEvent_0>(value: T) -> QResizeEvent {
    let rsthis = value.QResizeEvent_0();
    return rsthis;
    // return 1;
  }
}

pub trait QResizeEvent_QResizeEvent_0 {
  fn QResizeEvent_0(self) -> QResizeEvent;
}
// QResizeEvent(const QSize &, const QSize &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QResizeEvent_QResizeEvent_0 for (usize,usize) {
  fn QResizeEvent_0(self) -> QResizeEvent {
    // unsafe{_ZN12QResizeEventC2ERK5QSizeS2_()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN12QResizeEventC2ERK5QSizeS2_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QResizeEvent{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qevent.h:464
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QResizeEvent()

/*

*/
pub fn DeleteQResizeEvent(this :*mut QResizeEvent) {
    // let rv = qtrt::InvokeQtFunc6("_ZN12QResizeEventD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 40)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qevent.h:466
// index:0
// Public inline Visibility=Default Availability=Available
// [8] const QSize & size() const

/*

*/
impl /*struct*/ QResizeEvent {
  pub fn size_0<RetType, T: QResizeEvent_size_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.size_0(self);
    // return 1;
  }
}
pub trait QResizeEvent_size_0<RetType> {
  fn size_0(self , rsthis: & QResizeEvent) -> RetType;
}
impl<'a> /*trait*/ QResizeEvent_size_0<usize> for () {
  fn size_0(self , rsthis: & QResizeEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QResizeEvent4sizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:467
// index:0
// Public inline Visibility=Default Availability=Available
// [8] const QSize & oldSize() const

/*

*/
impl /*struct*/ QResizeEvent {
  pub fn oldSize_0<RetType, T: QResizeEvent_oldSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.oldSize_0(self);
    // return 1;
  }
}
pub trait QResizeEvent_oldSize_0<RetType> {
  fn oldSize_0(self , rsthis: & QResizeEvent) -> RetType;
}
impl<'a> /*trait*/ QResizeEvent_oldSize_0<usize> for () {
  fn oldSize_0(self , rsthis: & QResizeEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QResizeEvent7oldSizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
