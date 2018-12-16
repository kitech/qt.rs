

// mod ::widgets::QSwipeGesture
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
// extern C begin: 25
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
#[derive(Default)] // class sizeof(QSwipeGesture)=16
pub struct QSwipeGesture {
  qbase: QGesture,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QSwipeGesture_ITF interface {
//    QGesture_ITF
//    QSwipeGesture_PTR() *QSwipeGesture
//}
//func (ptr *QSwipeGesture) QSwipeGesture_PTR() *QSwipeGesture { return ptr }

impl /*struct*/ QSwipeGesture {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QSwipeGesture {
    return QSwipeGesture{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QSwipeGesture {
//  type Target = QSwipeGestureBASE;
//
//  fn deref(&self) -> &QSwipeGestureBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QSwipeGestureBASE> for QSwipeGesture {
//  fn as_ref(& self) -> & QSwipeGestureBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qgesture.h:209
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QSwipeGesture {
  pub fn metaObject_0<RetType, T: QSwipeGesture_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QSwipeGesture_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QSwipeGesture) -> RetType;
}
impl<'a> /*trait*/ QSwipeGesture_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QSwipeGesture) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QSwipeGesture10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgesture.h:221
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QSwipeGesture(QObject *)

/*

*/
// QSwipeGesture(QObject *) ctx.fn_proto_cpp
impl /*struct*/ QSwipeGesture {
  pub fn QSwipeGesture_0<T: QSwipeGesture_QSwipeGesture_0>(value: T) -> QSwipeGesture {
    let rsthis = value.QSwipeGesture_0();
    return rsthis;
    // return 1;
  }
}

pub trait QSwipeGesture_QSwipeGesture_0 {
  fn QSwipeGesture_0(self) -> QSwipeGesture;
}
// QSwipeGesture(QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QSwipeGesture_QSwipeGesture_0 for (usize) {
  fn QSwipeGesture_0(self) -> QSwipeGesture {
    // unsafe{_ZN13QSwipeGestureC2EP7QObject()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN13QSwipeGestureC2EP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QSwipeGesture{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgesture.h:222
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QSwipeGesture()

/*

*/
pub fn DeleteQSwipeGesture(this :*mut QSwipeGesture) {
    // let rv = qtrt::InvokeQtFunc6("_ZN13QSwipeGestureD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qgesture.h:224
// index:0
// Public Visibility=Default Availability=Available
// [4] QSwipeGesture::SwipeDirection horizontalDirection() const

/*

*/
impl /*struct*/ QSwipeGesture {
  pub fn horizontalDirection_0<RetType, T: QSwipeGesture_horizontalDirection_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.horizontalDirection_0(self);
    // return 1;
  }
}
pub trait QSwipeGesture_horizontalDirection_0<RetType> {
  fn horizontalDirection_0(self , rsthis: & QSwipeGesture) -> RetType;
}
impl<'a> /*trait*/ QSwipeGesture_horizontalDirection_0<i32> for () {
  fn horizontalDirection_0(self , rsthis: & QSwipeGesture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QSwipeGesture19horizontalDirectionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgesture.h:225
// index:0
// Public Visibility=Default Availability=Available
// [4] QSwipeGesture::SwipeDirection verticalDirection() const

/*

*/
impl /*struct*/ QSwipeGesture {
  pub fn verticalDirection_0<RetType, T: QSwipeGesture_verticalDirection_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.verticalDirection_0(self);
    // return 1;
  }
}
pub trait QSwipeGesture_verticalDirection_0<RetType> {
  fn verticalDirection_0(self , rsthis: & QSwipeGesture) -> RetType;
}
impl<'a> /*trait*/ QSwipeGesture_verticalDirection_0<i32> for () {
  fn verticalDirection_0(self , rsthis: & QSwipeGesture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QSwipeGesture17verticalDirectionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgesture.h:227
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal swipeAngle() const

/*

*/
impl /*struct*/ QSwipeGesture {
  pub fn swipeAngle_0<RetType, T: QSwipeGesture_swipeAngle_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.swipeAngle_0(self);
    // return 1;
  }
}
pub trait QSwipeGesture_swipeAngle_0<RetType> {
  fn swipeAngle_0(self , rsthis: & QSwipeGesture) -> RetType;
}
impl<'a> /*trait*/ QSwipeGesture_swipeAngle_0<f64> for () {
  fn swipeAngle_0(self , rsthis: & QSwipeGesture) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QSwipeGesture10swipeAngleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgesture.h:228
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setSwipeAngle(qreal)

/*

*/
impl /*struct*/ QSwipeGesture {
  pub fn setSwipeAngle_0<RetType, T: QSwipeGesture_setSwipeAngle_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSwipeAngle_0(self);
    // return 1;
  }
}
pub trait QSwipeGesture_setSwipeAngle_0<RetType> {
  fn setSwipeAngle_0(self , rsthis: & QSwipeGesture) -> RetType;
}
impl<'a> /*trait*/ QSwipeGesture_setSwipeAngle_0<(/*void*/)> for (f64) {
  fn setSwipeAngle_0(self , rsthis: & QSwipeGesture) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN13QSwipeGesture13setSwipeAngleEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}


/*


*/
pub type QSwipeGesture__SwipeDirection = i32;
// 
pub const QSwipeGesture__NoDirection :QSwipeGesture__SwipeDirection = 0;
// 
pub const QSwipeGesture__Left :QSwipeGesture__SwipeDirection = 1;
// 
pub const QSwipeGesture__Right :QSwipeGesture__SwipeDirection = 2;
// 
pub const QSwipeGesture__Up :QSwipeGesture__SwipeDirection = 3;
// 
pub const QSwipeGesture__Down :QSwipeGesture__SwipeDirection = 4;
pub fn QSwipeGesture_SwipeDirectionItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QSwipeGesture", val);
}
pub fn QSwipeGesture_SwipeDirectionItemName_s(val: i32) ->String {
  //var nilthis *QSwipeGesture
  //return nilthis.SwipeDirectionItemName(val);
  return QSwipeGesture_SwipeDirectionItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
