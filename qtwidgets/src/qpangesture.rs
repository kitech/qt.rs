

// mod ::widgets::QPanGesture
// package qtwidgets
// /usr/include/qt/QtWidgets/qgesture.h
// #include <qgesture.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 11
*/
// import "C"
// import "unsafe"
// import "reflect"
// import "fmt"
// import "log"
// import "github.com/kitech/qt.go/qtrt"
// import "github.com/kitech/qt.go/qtcore"
use qtcore::*; // super::super::%!s(MISSING)::*;
// import "github.com/kitech/qt.go/qtgui"
use qtgui::*; // super::super::%!s(MISSING)::*;
use std::default::Default;
use std::ops::Deref;
use qtrt; // super::super::qtrt;
use super::*;
//  ext block end

//  body block begin



/*

*/
#[derive(Default)] // class sizeof(QPanGesture)=16
pub struct QPanGesture {
  qbase: QGesture,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QPanGesture_ITF interface {
//    QGesture_ITF
//    QPanGesture_PTR() *QPanGesture
//}
//func (ptr *QPanGesture) QPanGesture_PTR() *QPanGesture { return ptr }

impl /*struct*/ QPanGesture {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QPanGesture {
    return QPanGesture{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QPanGesture {
//  type Target = QPanGestureBASE;
//
//  fn deref(&self) -> &QPanGestureBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QPanGestureBASE> for QPanGesture {
//  fn as_ref(& self) -> & QPanGestureBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qgesture.h:106
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QPanGesture {
  pub fn metaObject_0<RetType, T: QPanGesture_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QPanGesture_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QPanGesture) -> RetType;
}
impl<'a> /*trait*/ QPanGesture_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QPanGesture) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QPanGesture10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgesture.h:117
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QPanGesture(QObject *)

/*

*/
// QPanGesture(QObject *) ctx.fn_proto_cpp
impl /*struct*/ QPanGesture {
  pub fn QPanGesture_0<T: QPanGesture_QPanGesture_0>(value: T) -> QPanGesture {
    let rsthis = value.QPanGesture_0();
    return rsthis;
    // return 1;
  }
}

pub trait QPanGesture_QPanGesture_0 {
  fn QPanGesture_0(self) -> QPanGesture;
}
// QPanGesture(QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QPanGesture_QPanGesture_0 for (usize) {
  fn QPanGesture_0(self) -> QPanGesture {
    // unsafe{_ZN11QPanGestureC2EP7QObject()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QPanGestureC2EP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QPanGesture{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgesture.h:118
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QPanGesture()

/*

*/
pub fn DeleteQPanGesture(this :*mut QPanGesture) {
    // let rv = qtrt::InvokeQtFunc6("_ZN11QPanGestureD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qgesture.h:120
// index:0
// Public Visibility=Default Availability=Available
// [16] QPointF lastOffset() const

/*

*/
impl /*struct*/ QPanGesture {
  pub fn lastOffset_0<RetType, T: QPanGesture_lastOffset_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.lastOffset_0(self);
    // return 1;
  }
}
pub trait QPanGesture_lastOffset_0<RetType> {
  fn lastOffset_0(self , rsthis: & QPanGesture) -> RetType;
}
impl<'a> /*trait*/ QPanGesture_lastOffset_0<usize> for () {
  fn lastOffset_0(self , rsthis: & QPanGesture) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QPanGesture10lastOffsetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgesture.h:121
// index:0
// Public Visibility=Default Availability=Available
// [16] QPointF offset() const

/*

*/
impl /*struct*/ QPanGesture {
  pub fn offset_0<RetType, T: QPanGesture_offset_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.offset_0(self);
    // return 1;
  }
}
pub trait QPanGesture_offset_0<RetType> {
  fn offset_0(self , rsthis: & QPanGesture) -> RetType;
}
impl<'a> /*trait*/ QPanGesture_offset_0<usize> for () {
  fn offset_0(self , rsthis: & QPanGesture) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QPanGesture6offsetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgesture.h:122
// index:0
// Public Visibility=Default Availability=Available
// [16] QPointF delta() const

/*

*/
impl /*struct*/ QPanGesture {
  pub fn delta_0<RetType, T: QPanGesture_delta_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.delta_0(self);
    // return 1;
  }
}
pub trait QPanGesture_delta_0<RetType> {
  fn delta_0(self , rsthis: & QPanGesture) -> RetType;
}
impl<'a> /*trait*/ QPanGesture_delta_0<usize> for () {
  fn delta_0(self , rsthis: & QPanGesture) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QPanGesture5deltaEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgesture.h:123
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal acceleration() const

/*

*/
impl /*struct*/ QPanGesture {
  pub fn acceleration_0<RetType, T: QPanGesture_acceleration_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.acceleration_0(self);
    // return 1;
  }
}
pub trait QPanGesture_acceleration_0<RetType> {
  fn acceleration_0(self , rsthis: & QPanGesture) -> RetType;
}
impl<'a> /*trait*/ QPanGesture_acceleration_0<f64> for () {
  fn acceleration_0(self , rsthis: & QPanGesture) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QPanGesture12accelerationEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgesture.h:125
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setLastOffset(const QPointF &)

/*

*/
impl /*struct*/ QPanGesture {
  pub fn setLastOffset_0<RetType, T: QPanGesture_setLastOffset_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setLastOffset_0(self);
    // return 1;
  }
}
pub trait QPanGesture_setLastOffset_0<RetType> {
  fn setLastOffset_0(self , rsthis: & QPanGesture) -> RetType;
}
impl<'a> /*trait*/ QPanGesture_setLastOffset_0<(/*void*/)> for (usize) {
  fn setLastOffset_0(self , rsthis: & QPanGesture) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QPanGesture13setLastOffsetERK7QPointF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgesture.h:126
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setOffset(const QPointF &)

/*

*/
impl /*struct*/ QPanGesture {
  pub fn setOffset_0<RetType, T: QPanGesture_setOffset_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setOffset_0(self);
    // return 1;
  }
}
pub trait QPanGesture_setOffset_0<RetType> {
  fn setOffset_0(self , rsthis: & QPanGesture) -> RetType;
}
impl<'a> /*trait*/ QPanGesture_setOffset_0<(/*void*/)> for (usize) {
  fn setOffset_0(self , rsthis: & QPanGesture) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QPanGesture9setOffsetERK7QPointF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgesture.h:127
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setAcceleration(qreal)

/*

*/
impl /*struct*/ QPanGesture {
  pub fn setAcceleration_0<RetType, T: QPanGesture_setAcceleration_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setAcceleration_0(self);
    // return 1;
  }
}
pub trait QPanGesture_setAcceleration_0<RetType> {
  fn setAcceleration_0(self , rsthis: & QPanGesture) -> RetType;
}
impl<'a> /*trait*/ QPanGesture_setAcceleration_0<(/*void*/)> for (f64) {
  fn setAcceleration_0(self , rsthis: & QPanGesture) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN11QPanGesture15setAccelerationEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

//  body block end

//  keep block begin

//  keep block end
