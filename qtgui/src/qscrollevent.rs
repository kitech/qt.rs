

// mod ::gui::QScrollEvent
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
// extern C begin: 9
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
#[derive(Default)] // class sizeof(QScrollEvent)=64
pub struct QScrollEvent {
  qbase: QEvent,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QScrollEvent_ITF interface {
//    qtcore.QEvent_ITF
//    QScrollEvent_PTR() *QScrollEvent
//}
//func (ptr *QScrollEvent) QScrollEvent_PTR() *QScrollEvent { return ptr }

impl /*struct*/ QScrollEvent {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QScrollEvent {
    return QScrollEvent{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QScrollEvent {
//  type Target = QScrollEventBASE;
//
//  fn deref(&self) -> &QScrollEventBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QScrollEventBASE> for QScrollEvent {
//  fn as_ref(& self) -> & QScrollEventBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qevent.h:1022
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QScrollEvent(const QPointF &, const QPointF &, QScrollEvent::ScrollState)

/*

*/
// QScrollEvent(const QPointF &, const QPointF &, QScrollEvent::ScrollState) ctx.fn_proto_cpp
impl /*struct*/ QScrollEvent {
  pub fn QScrollEvent_0<T: QScrollEvent_QScrollEvent_0>(value: T) -> QScrollEvent {
    let rsthis = value.QScrollEvent_0();
    return rsthis;
    // return 1;
  }
}

pub trait QScrollEvent_QScrollEvent_0 {
  fn QScrollEvent_0(self) -> QScrollEvent;
}
// QScrollEvent(const QPointF &, const QPointF &, QScrollEvent::ScrollState) ctx.fn_proto_cpp
impl<'a> /*trait*/ QScrollEvent_QScrollEvent_0 for (usize,usize,i32) {
  fn QScrollEvent_0(self) -> QScrollEvent {
    // unsafe{_ZN12QScrollEventC2ERK7QPointFS2_NS_11ScrollStateE()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN12QScrollEventC2ERK7QPointFS2_NS_11ScrollStateE", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QScrollEvent{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qevent.h:1023
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QScrollEvent()

/*

*/
pub fn DeleteQScrollEvent(this :*mut QScrollEvent) {
    // let rv = qtrt::InvokeQtFunc6("_ZN12QScrollEventD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 64)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qevent.h:1025
// index:0
// Public Visibility=Default Availability=Available
// [16] QPointF contentPos() const

/*

*/
impl /*struct*/ QScrollEvent {
  pub fn contentPos_0<RetType, T: QScrollEvent_contentPos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.contentPos_0(self);
    // return 1;
  }
}
pub trait QScrollEvent_contentPos_0<RetType> {
  fn contentPos_0(self , rsthis: & QScrollEvent) -> RetType;
}
impl<'a> /*trait*/ QScrollEvent_contentPos_0<usize> for () {
  fn contentPos_0(self , rsthis: & QScrollEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QScrollEvent10contentPosEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:1026
// index:0
// Public Visibility=Default Availability=Available
// [16] QPointF overshootDistance() const

/*

*/
impl /*struct*/ QScrollEvent {
  pub fn overshootDistance_0<RetType, T: QScrollEvent_overshootDistance_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.overshootDistance_0(self);
    // return 1;
  }
}
pub trait QScrollEvent_overshootDistance_0<RetType> {
  fn overshootDistance_0(self , rsthis: & QScrollEvent) -> RetType;
}
impl<'a> /*trait*/ QScrollEvent_overshootDistance_0<usize> for () {
  fn overshootDistance_0(self , rsthis: & QScrollEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QScrollEvent17overshootDistanceEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:1027
// index:0
// Public Visibility=Default Availability=Available
// [4] QScrollEvent::ScrollState scrollState() const

/*

*/
impl /*struct*/ QScrollEvent {
  pub fn scrollState_0<RetType, T: QScrollEvent_scrollState_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.scrollState_0(self);
    // return 1;
  }
}
pub trait QScrollEvent_scrollState_0<RetType> {
  fn scrollState_0(self , rsthis: & QScrollEvent) -> RetType;
}
impl<'a> /*trait*/ QScrollEvent_scrollState_0<i32> for () {
  fn scrollState_0(self , rsthis: & QScrollEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QScrollEvent11scrollStateEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}


/*


*/
pub type QScrollEvent__ScrollState = i32;
// 
pub const QScrollEvent__ScrollStarted :QScrollEvent__ScrollState = 0;
// 
pub const QScrollEvent__ScrollUpdated :QScrollEvent__ScrollState = 1;
// 
pub const QScrollEvent__ScrollFinished :QScrollEvent__ScrollState = 2;
pub fn QScrollEvent_ScrollStateItemName(val: i32) ->String {
  match val {
     QScrollEvent__ScrollStarted => // 0
     {return String::from("ScrollStarted");}
     QScrollEvent__ScrollUpdated => // 1
     {return String::from("ScrollUpdated");}
     QScrollEvent__ScrollFinished => // 2
     {return String::from("ScrollFinished");}
  _ => {return format!("{}", val);}
}
}
pub fn QScrollEvent_ScrollStateItemName_s(val: i32) ->String {
  //var nilthis *QScrollEvent
  //return nilthis.ScrollStateItemName(val);
  return QScrollEvent_ScrollStateItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
