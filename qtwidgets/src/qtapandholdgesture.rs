

// mod ::widgets::QTapAndHoldGesture
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
// extern C begin: 5
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
#[derive(Default)] // class sizeof(QTapAndHoldGesture)=16
pub struct QTapAndHoldGesture {
  qbase: QGesture,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QTapAndHoldGesture_ITF interface {
//    QGesture_ITF
//    QTapAndHoldGesture_PTR() *QTapAndHoldGesture
//}
//func (ptr *QTapAndHoldGesture) QTapAndHoldGesture_PTR() *QTapAndHoldGesture { return ptr }

impl /*struct*/ QTapAndHoldGesture {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QTapAndHoldGesture {
    return QTapAndHoldGesture{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QTapAndHoldGesture {
//  type Target = QTapAndHoldGestureBASE;
//
//  fn deref(&self) -> &QTapAndHoldGestureBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QTapAndHoldGestureBASE> for QTapAndHoldGesture {
//  fn as_ref(& self) -> & QTapAndHoldGestureBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qgesture.h:254
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QTapAndHoldGesture {
  pub fn metaObject_0<RetType, T: QTapAndHoldGesture_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QTapAndHoldGesture_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QTapAndHoldGesture) -> RetType;
}
impl<'a> /*trait*/ QTapAndHoldGesture_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QTapAndHoldGesture) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QTapAndHoldGesture10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgesture.h:260
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QTapAndHoldGesture(QObject *)

/*

*/
// QTapAndHoldGesture(QObject *) ctx.fn_proto_cpp
impl /*struct*/ QTapAndHoldGesture {
  pub fn QTapAndHoldGesture_0<T: QTapAndHoldGesture_QTapAndHoldGesture_0>(value: T) -> QTapAndHoldGesture {
    let rsthis = value.QTapAndHoldGesture_0();
    return rsthis;
    // return 1;
  }
}

pub trait QTapAndHoldGesture_QTapAndHoldGesture_0 {
  fn QTapAndHoldGesture_0(self) -> QTapAndHoldGesture;
}
// QTapAndHoldGesture(QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QTapAndHoldGesture_QTapAndHoldGesture_0 for (usize) {
  fn QTapAndHoldGesture_0(self) -> QTapAndHoldGesture {
    // unsafe{_ZN18QTapAndHoldGestureC2EP7QObject()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN18QTapAndHoldGestureC2EP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTapAndHoldGesture{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgesture.h:261
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QTapAndHoldGesture()

/*

*/
pub fn DeleteQTapAndHoldGesture(this :*mut QTapAndHoldGesture) {
    // let rv = qtrt::InvokeQtFunc6("_ZN18QTapAndHoldGestureD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qgesture.h:263
// index:0
// Public Visibility=Default Availability=Available
// [16] QPointF position() const

/*

*/
impl /*struct*/ QTapAndHoldGesture {
  pub fn position_0<RetType, T: QTapAndHoldGesture_position_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.position_0(self);
    // return 1;
  }
}
pub trait QTapAndHoldGesture_position_0<RetType> {
  fn position_0(self , rsthis: & QTapAndHoldGesture) -> RetType;
}
impl<'a> /*trait*/ QTapAndHoldGesture_position_0<usize> for () {
  fn position_0(self , rsthis: & QTapAndHoldGesture) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QTapAndHoldGesture8positionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgesture.h:264
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setPosition(const QPointF &)

/*

*/
impl /*struct*/ QTapAndHoldGesture {
  pub fn setPosition_0<RetType, T: QTapAndHoldGesture_setPosition_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPosition_0(self);
    // return 1;
  }
}
pub trait QTapAndHoldGesture_setPosition_0<RetType> {
  fn setPosition_0(self , rsthis: & QTapAndHoldGesture) -> RetType;
}
impl<'a> /*trait*/ QTapAndHoldGesture_setPosition_0<(/*void*/)> for (usize) {
  fn setPosition_0(self , rsthis: & QTapAndHoldGesture) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN18QTapAndHoldGesture11setPositionERK7QPointF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgesture.h:266
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void setTimeout(int)

/*

*/
impl /*struct*/ QTapAndHoldGesture {
  pub fn setTimeout_0<RetType, T: QTapAndHoldGesture_setTimeout_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.setTimeout_0();
    // return 1;
  }
}
pub trait QTapAndHoldGesture_setTimeout_0<RetType> {
  fn setTimeout_0(self ) -> RetType;
}
impl<'a> /*trait*/ QTapAndHoldGesture_setTimeout_0<(/*void*/)> for (i32) {
  fn setTimeout_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN18QTapAndHoldGesture10setTimeoutEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgesture.h:267
// index:0
// Public static Visibility=Default Availability=Available
// [4] int timeout()

/*

*/
impl /*struct*/ QTapAndHoldGesture {
  pub fn timeout_0<RetType, T: QTapAndHoldGesture_timeout_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.timeout_0();
    // return 1;
  }
}
pub trait QTapAndHoldGesture_timeout_0<RetType> {
  fn timeout_0(self ) -> RetType;
}
impl<'a> /*trait*/ QTapAndHoldGesture_timeout_0<i32> for () {
  fn timeout_0(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN18QTapAndHoldGesture7timeoutEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
