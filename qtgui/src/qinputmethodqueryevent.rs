

// mod ::gui::QInputMethodQueryEvent
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
// extern C begin: 7
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
#[derive(Default)] // class sizeof(QInputMethodQueryEvent)=32
pub struct QInputMethodQueryEvent {
  qbase: QEvent,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QInputMethodQueryEvent_ITF interface {
//    qtcore.QEvent_ITF
//    QInputMethodQueryEvent_PTR() *QInputMethodQueryEvent
//}
//func (ptr *QInputMethodQueryEvent) QInputMethodQueryEvent_PTR() *QInputMethodQueryEvent { return ptr }

impl /*struct*/ QInputMethodQueryEvent {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QInputMethodQueryEvent {
    return QInputMethodQueryEvent{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QInputMethodQueryEvent {
//  type Target = QInputMethodQueryEventBASE;
//
//  fn deref(&self) -> &QInputMethodQueryEventBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QInputMethodQueryEventBASE> for QInputMethodQueryEvent {
//  fn as_ref(& self) -> & QInputMethodQueryEventBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qevent.h:581
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QInputMethodQueryEvent(Qt::InputMethodQueries)

/*

*/
// QInputMethodQueryEvent(Qt::InputMethodQueries) ctx.fn_proto_cpp
impl /*struct*/ QInputMethodQueryEvent {
  pub fn QInputMethodQueryEvent_0<T: QInputMethodQueryEvent_QInputMethodQueryEvent_0>(value: T) -> QInputMethodQueryEvent {
    let rsthis = value.QInputMethodQueryEvent_0();
    return rsthis;
    // return 1;
  }
}

pub trait QInputMethodQueryEvent_QInputMethodQueryEvent_0 {
  fn QInputMethodQueryEvent_0(self) -> QInputMethodQueryEvent;
}
// QInputMethodQueryEvent(Qt::InputMethodQueries) ctx.fn_proto_cpp
impl<'a> /*trait*/ QInputMethodQueryEvent_QInputMethodQueryEvent_0 for (i32) {
  fn QInputMethodQueryEvent_0(self) -> QInputMethodQueryEvent {
    // unsafe{_ZN22QInputMethodQueryEventC2E6QFlagsIN2Qt16InputMethodQueryEE()};
    let arg0 = (&self) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN22QInputMethodQueryEventC2E6QFlagsIN2Qt16InputMethodQueryEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QInputMethodQueryEvent{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qevent.h:582
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QInputMethodQueryEvent()

/*

*/
pub fn DeleteQInputMethodQueryEvent(this :*mut QInputMethodQueryEvent) {
    // let rv = qtrt::InvokeQtFunc6("_ZN22QInputMethodQueryEventD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 32)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qevent.h:584
// index:0
// Public inline Visibility=Default Availability=Available
// [4] Qt::InputMethodQueries queries() const

/*

*/
impl /*struct*/ QInputMethodQueryEvent {
  pub fn queries_0<RetType, T: QInputMethodQueryEvent_queries_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.queries_0(self);
    // return 1;
  }
}
pub trait QInputMethodQueryEvent_queries_0<RetType> {
  fn queries_0(self , rsthis: & QInputMethodQueryEvent) -> RetType;
}
impl<'a> /*trait*/ QInputMethodQueryEvent_queries_0<i32> for () {
  fn queries_0(self , rsthis: & QInputMethodQueryEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK22QInputMethodQueryEvent7queriesEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:586
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setValue(Qt::InputMethodQuery, const QVariant &)

/*

*/
impl /*struct*/ QInputMethodQueryEvent {
  pub fn setValue_0<RetType, T: QInputMethodQueryEvent_setValue_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setValue_0(self);
    // return 1;
  }
}
pub trait QInputMethodQueryEvent_setValue_0<RetType> {
  fn setValue_0(self , rsthis: & QInputMethodQueryEvent) -> RetType;
}
impl<'a> /*trait*/ QInputMethodQueryEvent_setValue_0<(/*void*/)> for (i32,usize) {
  fn setValue_0(self , rsthis: & QInputMethodQueryEvent) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN22QInputMethodQueryEvent8setValueEN2Qt16InputMethodQueryERK8QVariant", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qevent.h:587
// index:0
// Public Visibility=Default Availability=Available
// [16] QVariant value(Qt::InputMethodQuery) const

/*

*/
impl /*struct*/ QInputMethodQueryEvent {
  pub fn value_0<RetType, T: QInputMethodQueryEvent_value_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.value_0(self);
    // return 1;
  }
}
pub trait QInputMethodQueryEvent_value_0<RetType> {
  fn value_0(self , rsthis: & QInputMethodQueryEvent) -> RetType;
}
impl<'a> /*trait*/ QInputMethodQueryEvent_value_0<usize> for (i32) {
  fn value_0(self , rsthis: & QInputMethodQueryEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK22QInputMethodQueryEvent5valueEN2Qt16InputMethodQueryE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
