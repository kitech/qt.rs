

// mod ::gui::QAccessibleValueChangeEvent
// package qtgui
// /usr/include/qt/QtGui/qaccessible.h
// #include <qaccessible.h>
// #include <QtGui>

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
#[derive(Default)] // class sizeof(QAccessibleValueChangeEvent)=48
pub struct QAccessibleValueChangeEvent {
  qbase: QAccessibleEvent,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QAccessibleValueChangeEvent_ITF interface {
//    QAccessibleEvent_ITF
//    QAccessibleValueChangeEvent_PTR() *QAccessibleValueChangeEvent
//}
//func (ptr *QAccessibleValueChangeEvent) QAccessibleValueChangeEvent_PTR() *QAccessibleValueChangeEvent { return ptr }

impl /*struct*/ QAccessibleValueChangeEvent {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QAccessibleValueChangeEvent {
    return QAccessibleValueChangeEvent{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QAccessibleValueChangeEvent {
//  type Target = QAccessibleValueChangeEventBASE;
//
//  fn deref(&self) -> &QAccessibleValueChangeEventBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QAccessibleValueChangeEventBASE> for QAccessibleValueChangeEvent {
//  fn as_ref(& self) -> & QAccessibleValueChangeEventBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qaccessible.h:898
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void QAccessibleValueChangeEvent(QObject *, const QVariant &)

/*

*/
// QAccessibleValueChangeEvent(QObject *, const QVariant &) ctx.fn_proto_cpp
impl /*struct*/ QAccessibleValueChangeEvent {
  pub fn QAccessibleValueChangeEvent_0<T: QAccessibleValueChangeEvent_QAccessibleValueChangeEvent_0>(value: T) -> QAccessibleValueChangeEvent {
    let rsthis = value.QAccessibleValueChangeEvent_0();
    return rsthis;
    // return 1;
  }
}

pub trait QAccessibleValueChangeEvent_QAccessibleValueChangeEvent_0 {
  fn QAccessibleValueChangeEvent_0(self) -> QAccessibleValueChangeEvent;
}
// QAccessibleValueChangeEvent(QObject *, const QVariant &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QAccessibleValueChangeEvent_QAccessibleValueChangeEvent_0 for (usize,usize) {
  fn QAccessibleValueChangeEvent_0(self) -> QAccessibleValueChangeEvent {
    // unsafe{_ZN27QAccessibleValueChangeEventC2EP7QObjectRK8QVariant()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN27QAccessibleValueChangeEventC2EP7QObjectRK8QVariant", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QAccessibleValueChangeEvent{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:904
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void QAccessibleValueChangeEvent(QAccessibleInterface *, const QVariant &)

/*

*/
// QAccessibleValueChangeEvent(QAccessibleInterface *, const QVariant &) ctx.fn_proto_cpp
impl /*struct*/ QAccessibleValueChangeEvent {
  pub fn QAccessibleValueChangeEvent_1<T: QAccessibleValueChangeEvent_QAccessibleValueChangeEvent_1>(value: T) -> QAccessibleValueChangeEvent {
    let rsthis = value.QAccessibleValueChangeEvent_1();
    return rsthis;
    // return 1;
  }
}

pub trait QAccessibleValueChangeEvent_QAccessibleValueChangeEvent_1 {
  fn QAccessibleValueChangeEvent_1(self) -> QAccessibleValueChangeEvent;
}
// QAccessibleValueChangeEvent(QAccessibleInterface *, const QVariant &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QAccessibleValueChangeEvent_QAccessibleValueChangeEvent_1 for (usize,usize) {
  fn QAccessibleValueChangeEvent_1(self) -> QAccessibleValueChangeEvent {
    // unsafe{_ZN27QAccessibleValueChangeEventC2EP20QAccessibleInterfaceRK8QVariant()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN27QAccessibleValueChangeEventC2EP20QAccessibleInterfaceRK8QVariant", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QAccessibleValueChangeEvent{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:911
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QAccessibleValueChangeEvent()

/*

*/
pub fn DeleteQAccessibleValueChangeEvent(this :*mut QAccessibleValueChangeEvent) {
    // let rv = qtrt::InvokeQtFunc6("_ZN27QAccessibleValueChangeEventD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 48)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qaccessible.h:913
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setValue(const QVariant &)

/*

*/
impl /*struct*/ QAccessibleValueChangeEvent {
  pub fn setValue_0<RetType, T: QAccessibleValueChangeEvent_setValue_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setValue_0(self);
    // return 1;
  }
}
pub trait QAccessibleValueChangeEvent_setValue_0<RetType> {
  fn setValue_0(self , rsthis: & QAccessibleValueChangeEvent) -> RetType;
}
impl<'a> /*trait*/ QAccessibleValueChangeEvent_setValue_0<(/*void*/)> for (usize) {
  fn setValue_0(self , rsthis: & QAccessibleValueChangeEvent) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN27QAccessibleValueChangeEvent8setValueERK8QVariant", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:914
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QVariant value() const

/*

*/
impl /*struct*/ QAccessibleValueChangeEvent {
  pub fn value_0<RetType, T: QAccessibleValueChangeEvent_value_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.value_0(self);
    // return 1;
  }
}
pub trait QAccessibleValueChangeEvent_value_0<RetType> {
  fn value_0(self , rsthis: & QAccessibleValueChangeEvent) -> RetType;
}
impl<'a> /*trait*/ QAccessibleValueChangeEvent_value_0<usize> for () {
  fn value_0(self , rsthis: & QAccessibleValueChangeEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK27QAccessibleValueChangeEvent5valueEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
