

// mod ::gui::QExposeEvent
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
#[derive(Default)] // class sizeof(QExposeEvent)=32
pub struct QExposeEvent {
  qbase: QEvent,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QExposeEvent_ITF interface {
//    qtcore.QEvent_ITF
//    QExposeEvent_PTR() *QExposeEvent
//}
//func (ptr *QExposeEvent) QExposeEvent_PTR() *QExposeEvent { return ptr }

impl /*struct*/ QExposeEvent {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QExposeEvent {
    return QExposeEvent{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QExposeEvent {
//  type Target = QExposeEventBASE;
//
//  fn deref(&self) -> &QExposeEventBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QExposeEventBASE> for QExposeEvent {
//  fn as_ref(& self) -> & QExposeEventBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qevent.h:434
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QExposeEvent(const QRegion &)

/*

*/
// QExposeEvent(const QRegion &) ctx.fn_proto_cpp
impl /*struct*/ QExposeEvent {
  pub fn QExposeEvent_0<T: QExposeEvent_QExposeEvent_0>(value: T) -> QExposeEvent {
    let rsthis = value.QExposeEvent_0();
    return rsthis;
    // return 1;
  }
}

pub trait QExposeEvent_QExposeEvent_0 {
  fn QExposeEvent_0(self) -> QExposeEvent;
}
// QExposeEvent(const QRegion &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QExposeEvent_QExposeEvent_0 for (usize) {
  fn QExposeEvent_0(self) -> QExposeEvent {
    // unsafe{_ZN12QExposeEventC2ERK7QRegion()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN12QExposeEventC2ERK7QRegion", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QExposeEvent{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qevent.h:435
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QExposeEvent()

/*

*/
pub fn DeleteQExposeEvent(this :*mut QExposeEvent) {
    // let rv = qtrt::InvokeQtFunc6("_ZN12QExposeEventD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 32)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qevent.h:437
// index:0
// Public inline Visibility=Default Availability=Available
// [8] const QRegion & region() const

/*

*/
impl /*struct*/ QExposeEvent {
  pub fn region_0<RetType, T: QExposeEvent_region_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.region_0(self);
    // return 1;
  }
}
pub trait QExposeEvent_region_0<RetType> {
  fn region_0(self , rsthis: & QExposeEvent) -> RetType;
}
impl<'a> /*trait*/ QExposeEvent_region_0<usize> for () {
  fn region_0(self , rsthis: & QExposeEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QExposeEvent6regionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
