

// mod ::core::QDynamicPropertyChangeEvent
// package qtcore
// /usr/include/qt/QtCore/qcoreevent.h
// #include <qcoreevent.h>
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
#[derive(Default)] // class sizeof(QDynamicPropertyChangeEvent)=32
pub struct QDynamicPropertyChangeEvent {
  qbase: QEvent,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QDynamicPropertyChangeEvent_ITF interface {
//    QEvent_ITF
//    QDynamicPropertyChangeEvent_PTR() *QDynamicPropertyChangeEvent
//}
//func (ptr *QDynamicPropertyChangeEvent) QDynamicPropertyChangeEvent_PTR() *QDynamicPropertyChangeEvent { return ptr }

impl /*struct*/ QDynamicPropertyChangeEvent {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QDynamicPropertyChangeEvent {
    return QDynamicPropertyChangeEvent{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QDynamicPropertyChangeEvent {
//  type Target = QDynamicPropertyChangeEventBASE;
//
//  fn deref(&self) -> &QDynamicPropertyChangeEventBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QDynamicPropertyChangeEventBASE> for QDynamicPropertyChangeEvent {
//  fn as_ref(& self) -> & QDynamicPropertyChangeEventBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qcoreevent.h:365
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QDynamicPropertyChangeEvent(const QByteArray &)

/*

*/
// QDynamicPropertyChangeEvent(const QByteArray &) ctx.fn_proto_cpp
impl /*struct*/ QDynamicPropertyChangeEvent {
  pub fn QDynamicPropertyChangeEvent_0<T: QDynamicPropertyChangeEvent_QDynamicPropertyChangeEvent_0>(value: T) -> QDynamicPropertyChangeEvent {
    let rsthis = value.QDynamicPropertyChangeEvent_0();
    return rsthis;
    // return 1;
  }
}

pub trait QDynamicPropertyChangeEvent_QDynamicPropertyChangeEvent_0 {
  fn QDynamicPropertyChangeEvent_0(self) -> QDynamicPropertyChangeEvent;
}
// QDynamicPropertyChangeEvent(const QByteArray &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QDynamicPropertyChangeEvent_QDynamicPropertyChangeEvent_0 for (usize) {
  fn QDynamicPropertyChangeEvent_0(self) -> QDynamicPropertyChangeEvent {
    // unsafe{_ZN27QDynamicPropertyChangeEventC2ERK10QByteArray()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN27QDynamicPropertyChangeEventC2ERK10QByteArray", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QDynamicPropertyChangeEvent{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qcoreevent.h:366
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QDynamicPropertyChangeEvent()

/*

*/
pub fn DeleteQDynamicPropertyChangeEvent(this :*mut QDynamicPropertyChangeEvent) {
    // let rv = qtrt::InvokeQtFunc6("_ZN27QDynamicPropertyChangeEventD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 32)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qcoreevent.h:368
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QByteArray propertyName() const

/*

*/
impl /*struct*/ QDynamicPropertyChangeEvent {
  pub fn propertyName_0<RetType, T: QDynamicPropertyChangeEvent_propertyName_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.propertyName_0(self);
    // return 1;
  }
}
pub trait QDynamicPropertyChangeEvent_propertyName_0<RetType> {
  fn propertyName_0(self , rsthis: & QDynamicPropertyChangeEvent) -> RetType;
}
impl<'a> /*trait*/ QDynamicPropertyChangeEvent_propertyName_0<usize> for () {
  fn propertyName_0(self , rsthis: & QDynamicPropertyChangeEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK27QDynamicPropertyChangeEvent12propertyNameEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
