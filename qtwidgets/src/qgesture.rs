

// mod ::widgets::QGesture
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
// extern C begin: 56
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
#[derive(Default)] // class sizeof(QGesture)=16
pub struct QGesture {
  qbase: QObject,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QGesture_ITF interface {
//    qtcore.QObject_ITF
//    QGesture_PTR() *QGesture
//}
//func (ptr *QGesture) QGesture_PTR() *QGesture { return ptr }

impl /*struct*/ QGesture {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QGesture {
    return QGesture{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QGesture {
//  type Target = QGestureBASE;
//
//  fn deref(&self) -> &QGestureBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QGestureBASE> for QGesture {
//  fn as_ref(& self) -> & QGestureBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qgesture.h:63
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QGesture {
  pub fn metaObject_0<RetType, T: QGesture_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QGesture_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QGesture) -> RetType;
}
impl<'a> /*trait*/ QGesture_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QGesture) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QGesture10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgesture.h:73
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QGesture(QObject *)

/*
Constructs a new gesture object with the given parent.

QGesture objects are created by gesture recognizers in the QGestureRecognizer::create() function.
*/
// QGesture(QObject *) ctx.fn_proto_cpp
impl /*struct*/ QGesture {
  pub fn QGesture_0<T: QGesture_QGesture_0>(value: T) -> QGesture {
    let rsthis = value.QGesture_0();
    return rsthis;
    // return 1;
  }
}

pub trait QGesture_QGesture_0 {
  fn QGesture_0(self) -> QGesture;
}
// QGesture(QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QGesture_QGesture_0 for (usize) {
  fn QGesture_0(self) -> QGesture {
    // unsafe{_ZN8QGestureC2EP7QObject()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN8QGestureC2EP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QGesture{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgesture.h:74
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QGesture()

/*

*/
pub fn DeleteQGesture(this :*mut QGesture) {
    // let rv = qtrt::InvokeQtFunc6("_ZN8QGestureD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qgesture.h:76
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::GestureType gestureType() const

/*

*/
impl /*struct*/ QGesture {
  pub fn gestureType_0<RetType, T: QGesture_gestureType_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.gestureType_0(self);
    // return 1;
  }
}
pub trait QGesture_gestureType_0<RetType> {
  fn gestureType_0(self , rsthis: & QGesture) -> RetType;
}
impl<'a> /*trait*/ QGesture_gestureType_0<i32> for () {
  fn gestureType_0(self , rsthis: & QGesture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QGesture11gestureTypeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgesture.h:78
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::GestureState state() const

/*

*/
impl /*struct*/ QGesture {
  pub fn state_0<RetType, T: QGesture_state_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.state_0(self);
    // return 1;
  }
}
pub trait QGesture_state_0<RetType> {
  fn state_0(self , rsthis: & QGesture) -> RetType;
}
impl<'a> /*trait*/ QGesture_state_0<i32> for () {
  fn state_0(self , rsthis: & QGesture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QGesture5stateEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgesture.h:80
// index:0
// Public Visibility=Default Availability=Available
// [16] QPointF hotSpot() const

/*

*/
impl /*struct*/ QGesture {
  pub fn hotSpot_0<RetType, T: QGesture_hotSpot_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hotSpot_0(self);
    // return 1;
  }
}
pub trait QGesture_hotSpot_0<RetType> {
  fn hotSpot_0(self , rsthis: & QGesture) -> RetType;
}
impl<'a> /*trait*/ QGesture_hotSpot_0<usize> for () {
  fn hotSpot_0(self , rsthis: & QGesture) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QGesture7hotSpotEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgesture.h:81
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setHotSpot(const QPointF &)

/*

*/
impl /*struct*/ QGesture {
  pub fn setHotSpot_0<RetType, T: QGesture_setHotSpot_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setHotSpot_0(self);
    // return 1;
  }
}
pub trait QGesture_setHotSpot_0<RetType> {
  fn setHotSpot_0(self , rsthis: & QGesture) -> RetType;
}
impl<'a> /*trait*/ QGesture_setHotSpot_0<(/*void*/)> for (usize) {
  fn setHotSpot_0(self , rsthis: & QGesture) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QGesture10setHotSpotERK7QPointF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgesture.h:82
// index:0
// Public Visibility=Default Availability=Available
// [1] bool hasHotSpot() const

/*

*/
impl /*struct*/ QGesture {
  pub fn hasHotSpot_0<RetType, T: QGesture_hasHotSpot_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hasHotSpot_0(self);
    // return 1;
  }
}
pub trait QGesture_hasHotSpot_0<RetType> {
  fn hasHotSpot_0(self , rsthis: & QGesture) -> RetType;
}
impl<'a> /*trait*/ QGesture_hasHotSpot_0<bool> for () {
  fn hasHotSpot_0(self , rsthis: & QGesture) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QGesture10hasHotSpotEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgesture.h:83
// index:0
// Public Visibility=Default Availability=Available
// [-2] void unsetHotSpot()

/*

*/
impl /*struct*/ QGesture {
  pub fn unsetHotSpot_0<RetType, T: QGesture_unsetHotSpot_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.unsetHotSpot_0(self);
    // return 1;
  }
}
pub trait QGesture_unsetHotSpot_0<RetType> {
  fn unsetHotSpot_0(self , rsthis: & QGesture) -> RetType;
}
impl<'a> /*trait*/ QGesture_unsetHotSpot_0<(/*void*/)> for () {
  fn unsetHotSpot_0(self , rsthis: & QGesture) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN8QGesture12unsetHotSpotEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgesture.h:90
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setGestureCancelPolicy(QGesture::GestureCancelPolicy)

/*

*/
impl /*struct*/ QGesture {
  pub fn setGestureCancelPolicy_0<RetType, T: QGesture_setGestureCancelPolicy_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setGestureCancelPolicy_0(self);
    // return 1;
  }
}
pub trait QGesture_setGestureCancelPolicy_0<RetType> {
  fn setGestureCancelPolicy_0(self , rsthis: & QGesture) -> RetType;
}
impl<'a> /*trait*/ QGesture_setGestureCancelPolicy_0<(/*void*/)> for (i32) {
  fn setGestureCancelPolicy_0(self , rsthis: & QGesture) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QGesture22setGestureCancelPolicyENS_19GestureCancelPolicyE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgesture.h:91
// index:0
// Public Visibility=Default Availability=Available
// [4] QGesture::GestureCancelPolicy gestureCancelPolicy() const

/*

*/
impl /*struct*/ QGesture {
  pub fn gestureCancelPolicy_0<RetType, T: QGesture_gestureCancelPolicy_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.gestureCancelPolicy_0(self);
    // return 1;
  }
}
pub trait QGesture_gestureCancelPolicy_0<RetType> {
  fn gestureCancelPolicy_0(self , rsthis: & QGesture) -> RetType;
}
impl<'a> /*trait*/ QGesture_gestureCancelPolicy_0<i32> for () {
  fn gestureCancelPolicy_0(self , rsthis: & QGesture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QGesture19gestureCancelPolicyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}


/*
This enum describes how accepting a gesture can cancel other gestures automatically.


*/
pub type QGesture__GestureCancelPolicy = i32;
// On accepting this gesture no other gestures will be affected.
pub const QGesture__CancelNone :QGesture__GestureCancelPolicy = 0;
// On accepting this gesture all gestures that are active in the context (respecting the Qt::GestureFlag that were specified when subscribed to the gesture) will be cancelled.
pub const QGesture__CancelAllInContext :QGesture__GestureCancelPolicy = 1;
pub fn QGesture_GestureCancelPolicyItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QGesture", val);
}
pub fn QGesture_GestureCancelPolicyItemName_s(val: i32) ->String {
  //var nilthis *QGesture
  //return nilthis.GestureCancelPolicyItemName(val);
  return QGesture_GestureCancelPolicyItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
