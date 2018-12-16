

// mod ::widgets::QPinchGesture
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
// extern C begin: 10
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
#[derive(Default)] // class sizeof(QPinchGesture)=16
pub struct QPinchGesture {
  qbase: QGesture,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QPinchGesture_ITF interface {
//    QGesture_ITF
//    QPinchGesture_PTR() *QPinchGesture
//}
//func (ptr *QPinchGesture) QPinchGesture_PTR() *QPinchGesture { return ptr }

impl /*struct*/ QPinchGesture {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QPinchGesture {
    return QPinchGesture{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QPinchGesture {
//  type Target = QPinchGestureBASE;
//
//  fn deref(&self) -> &QPinchGestureBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QPinchGestureBASE> for QPinchGesture {
//  fn as_ref(& self) -> & QPinchGestureBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qgesture.h:136
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QPinchGesture {
  pub fn metaObject_0<RetType, T: QPinchGesture_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QPinchGesture_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QPinchGesture) -> RetType;
}
impl<'a> /*trait*/ QPinchGesture_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QPinchGesture) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QPinchGesture10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgesture.h:165
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QPinchGesture(QObject *)

/*

*/
// QPinchGesture(QObject *) ctx.fn_proto_cpp
impl /*struct*/ QPinchGesture {
  pub fn QPinchGesture_0<T: QPinchGesture_QPinchGesture_0>(value: T) -> QPinchGesture {
    let rsthis = value.QPinchGesture_0();
    return rsthis;
    // return 1;
  }
}

pub trait QPinchGesture_QPinchGesture_0 {
  fn QPinchGesture_0(self) -> QPinchGesture;
}
// QPinchGesture(QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QPinchGesture_QPinchGesture_0 for (usize) {
  fn QPinchGesture_0(self) -> QPinchGesture {
    // unsafe{_ZN13QPinchGestureC2EP7QObject()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN13QPinchGestureC2EP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QPinchGesture{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgesture.h:166
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QPinchGesture()

/*

*/
pub fn DeleteQPinchGesture(this :*mut QPinchGesture) {
    // let rv = qtrt::InvokeQtFunc6("_ZN13QPinchGestureD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qgesture.h:168
// index:0
// Public Visibility=Default Availability=Available
// [4] QPinchGesture::ChangeFlags totalChangeFlags() const

/*

*/
impl /*struct*/ QPinchGesture {
  pub fn totalChangeFlags_0<RetType, T: QPinchGesture_totalChangeFlags_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.totalChangeFlags_0(self);
    // return 1;
  }
}
pub trait QPinchGesture_totalChangeFlags_0<RetType> {
  fn totalChangeFlags_0(self , rsthis: & QPinchGesture) -> RetType;
}
impl<'a> /*trait*/ QPinchGesture_totalChangeFlags_0<i32> for () {
  fn totalChangeFlags_0(self , rsthis: & QPinchGesture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QPinchGesture16totalChangeFlagsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgesture.h:169
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTotalChangeFlags(QPinchGesture::ChangeFlags)

/*

*/
impl /*struct*/ QPinchGesture {
  pub fn setTotalChangeFlags_0<RetType, T: QPinchGesture_setTotalChangeFlags_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTotalChangeFlags_0(self);
    // return 1;
  }
}
pub trait QPinchGesture_setTotalChangeFlags_0<RetType> {
  fn setTotalChangeFlags_0(self , rsthis: & QPinchGesture) -> RetType;
}
impl<'a> /*trait*/ QPinchGesture_setTotalChangeFlags_0<(/*void*/)> for (i32) {
  fn setTotalChangeFlags_0(self , rsthis: & QPinchGesture) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN13QPinchGesture19setTotalChangeFlagsE6QFlagsINS_10ChangeFlagEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgesture.h:171
// index:0
// Public Visibility=Default Availability=Available
// [4] QPinchGesture::ChangeFlags changeFlags() const

/*

*/
impl /*struct*/ QPinchGesture {
  pub fn changeFlags_0<RetType, T: QPinchGesture_changeFlags_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.changeFlags_0(self);
    // return 1;
  }
}
pub trait QPinchGesture_changeFlags_0<RetType> {
  fn changeFlags_0(self , rsthis: & QPinchGesture) -> RetType;
}
impl<'a> /*trait*/ QPinchGesture_changeFlags_0<i32> for () {
  fn changeFlags_0(self , rsthis: & QPinchGesture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QPinchGesture11changeFlagsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgesture.h:172
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setChangeFlags(QPinchGesture::ChangeFlags)

/*

*/
impl /*struct*/ QPinchGesture {
  pub fn setChangeFlags_0<RetType, T: QPinchGesture_setChangeFlags_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setChangeFlags_0(self);
    // return 1;
  }
}
pub trait QPinchGesture_setChangeFlags_0<RetType> {
  fn setChangeFlags_0(self , rsthis: & QPinchGesture) -> RetType;
}
impl<'a> /*trait*/ QPinchGesture_setChangeFlags_0<(/*void*/)> for (i32) {
  fn setChangeFlags_0(self , rsthis: & QPinchGesture) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN13QPinchGesture14setChangeFlagsE6QFlagsINS_10ChangeFlagEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgesture.h:174
// index:0
// Public Visibility=Default Availability=Available
// [16] QPointF startCenterPoint() const

/*

*/
impl /*struct*/ QPinchGesture {
  pub fn startCenterPoint_0<RetType, T: QPinchGesture_startCenterPoint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.startCenterPoint_0(self);
    // return 1;
  }
}
pub trait QPinchGesture_startCenterPoint_0<RetType> {
  fn startCenterPoint_0(self , rsthis: & QPinchGesture) -> RetType;
}
impl<'a> /*trait*/ QPinchGesture_startCenterPoint_0<usize> for () {
  fn startCenterPoint_0(self , rsthis: & QPinchGesture) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QPinchGesture16startCenterPointEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgesture.h:175
// index:0
// Public Visibility=Default Availability=Available
// [16] QPointF lastCenterPoint() const

/*

*/
impl /*struct*/ QPinchGesture {
  pub fn lastCenterPoint_0<RetType, T: QPinchGesture_lastCenterPoint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.lastCenterPoint_0(self);
    // return 1;
  }
}
pub trait QPinchGesture_lastCenterPoint_0<RetType> {
  fn lastCenterPoint_0(self , rsthis: & QPinchGesture) -> RetType;
}
impl<'a> /*trait*/ QPinchGesture_lastCenterPoint_0<usize> for () {
  fn lastCenterPoint_0(self , rsthis: & QPinchGesture) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QPinchGesture15lastCenterPointEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgesture.h:176
// index:0
// Public Visibility=Default Availability=Available
// [16] QPointF centerPoint() const

/*

*/
impl /*struct*/ QPinchGesture {
  pub fn centerPoint_0<RetType, T: QPinchGesture_centerPoint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.centerPoint_0(self);
    // return 1;
  }
}
pub trait QPinchGesture_centerPoint_0<RetType> {
  fn centerPoint_0(self , rsthis: & QPinchGesture) -> RetType;
}
impl<'a> /*trait*/ QPinchGesture_centerPoint_0<usize> for () {
  fn centerPoint_0(self , rsthis: & QPinchGesture) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QPinchGesture11centerPointEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgesture.h:177
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setStartCenterPoint(const QPointF &)

/*

*/
impl /*struct*/ QPinchGesture {
  pub fn setStartCenterPoint_0<RetType, T: QPinchGesture_setStartCenterPoint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setStartCenterPoint_0(self);
    // return 1;
  }
}
pub trait QPinchGesture_setStartCenterPoint_0<RetType> {
  fn setStartCenterPoint_0(self , rsthis: & QPinchGesture) -> RetType;
}
impl<'a> /*trait*/ QPinchGesture_setStartCenterPoint_0<(/*void*/)> for (usize) {
  fn setStartCenterPoint_0(self , rsthis: & QPinchGesture) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QPinchGesture19setStartCenterPointERK7QPointF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgesture.h:178
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setLastCenterPoint(const QPointF &)

/*

*/
impl /*struct*/ QPinchGesture {
  pub fn setLastCenterPoint_0<RetType, T: QPinchGesture_setLastCenterPoint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setLastCenterPoint_0(self);
    // return 1;
  }
}
pub trait QPinchGesture_setLastCenterPoint_0<RetType> {
  fn setLastCenterPoint_0(self , rsthis: & QPinchGesture) -> RetType;
}
impl<'a> /*trait*/ QPinchGesture_setLastCenterPoint_0<(/*void*/)> for (usize) {
  fn setLastCenterPoint_0(self , rsthis: & QPinchGesture) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QPinchGesture18setLastCenterPointERK7QPointF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgesture.h:179
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCenterPoint(const QPointF &)

/*

*/
impl /*struct*/ QPinchGesture {
  pub fn setCenterPoint_0<RetType, T: QPinchGesture_setCenterPoint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCenterPoint_0(self);
    // return 1;
  }
}
pub trait QPinchGesture_setCenterPoint_0<RetType> {
  fn setCenterPoint_0(self , rsthis: & QPinchGesture) -> RetType;
}
impl<'a> /*trait*/ QPinchGesture_setCenterPoint_0<(/*void*/)> for (usize) {
  fn setCenterPoint_0(self , rsthis: & QPinchGesture) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QPinchGesture14setCenterPointERK7QPointF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgesture.h:181
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal totalScaleFactor() const

/*

*/
impl /*struct*/ QPinchGesture {
  pub fn totalScaleFactor_0<RetType, T: QPinchGesture_totalScaleFactor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.totalScaleFactor_0(self);
    // return 1;
  }
}
pub trait QPinchGesture_totalScaleFactor_0<RetType> {
  fn totalScaleFactor_0(self , rsthis: & QPinchGesture) -> RetType;
}
impl<'a> /*trait*/ QPinchGesture_totalScaleFactor_0<f64> for () {
  fn totalScaleFactor_0(self , rsthis: & QPinchGesture) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QPinchGesture16totalScaleFactorEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgesture.h:182
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal lastScaleFactor() const

/*

*/
impl /*struct*/ QPinchGesture {
  pub fn lastScaleFactor_0<RetType, T: QPinchGesture_lastScaleFactor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.lastScaleFactor_0(self);
    // return 1;
  }
}
pub trait QPinchGesture_lastScaleFactor_0<RetType> {
  fn lastScaleFactor_0(self , rsthis: & QPinchGesture) -> RetType;
}
impl<'a> /*trait*/ QPinchGesture_lastScaleFactor_0<f64> for () {
  fn lastScaleFactor_0(self , rsthis: & QPinchGesture) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QPinchGesture15lastScaleFactorEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgesture.h:183
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal scaleFactor() const

/*

*/
impl /*struct*/ QPinchGesture {
  pub fn scaleFactor_0<RetType, T: QPinchGesture_scaleFactor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.scaleFactor_0(self);
    // return 1;
  }
}
pub trait QPinchGesture_scaleFactor_0<RetType> {
  fn scaleFactor_0(self , rsthis: & QPinchGesture) -> RetType;
}
impl<'a> /*trait*/ QPinchGesture_scaleFactor_0<f64> for () {
  fn scaleFactor_0(self , rsthis: & QPinchGesture) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QPinchGesture11scaleFactorEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgesture.h:184
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTotalScaleFactor(qreal)

/*

*/
impl /*struct*/ QPinchGesture {
  pub fn setTotalScaleFactor_0<RetType, T: QPinchGesture_setTotalScaleFactor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTotalScaleFactor_0(self);
    // return 1;
  }
}
pub trait QPinchGesture_setTotalScaleFactor_0<RetType> {
  fn setTotalScaleFactor_0(self , rsthis: & QPinchGesture) -> RetType;
}
impl<'a> /*trait*/ QPinchGesture_setTotalScaleFactor_0<(/*void*/)> for (f64) {
  fn setTotalScaleFactor_0(self , rsthis: & QPinchGesture) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN13QPinchGesture19setTotalScaleFactorEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgesture.h:185
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setLastScaleFactor(qreal)

/*

*/
impl /*struct*/ QPinchGesture {
  pub fn setLastScaleFactor_0<RetType, T: QPinchGesture_setLastScaleFactor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setLastScaleFactor_0(self);
    // return 1;
  }
}
pub trait QPinchGesture_setLastScaleFactor_0<RetType> {
  fn setLastScaleFactor_0(self , rsthis: & QPinchGesture) -> RetType;
}
impl<'a> /*trait*/ QPinchGesture_setLastScaleFactor_0<(/*void*/)> for (f64) {
  fn setLastScaleFactor_0(self , rsthis: & QPinchGesture) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN13QPinchGesture18setLastScaleFactorEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgesture.h:186
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setScaleFactor(qreal)

/*

*/
impl /*struct*/ QPinchGesture {
  pub fn setScaleFactor_0<RetType, T: QPinchGesture_setScaleFactor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setScaleFactor_0(self);
    // return 1;
  }
}
pub trait QPinchGesture_setScaleFactor_0<RetType> {
  fn setScaleFactor_0(self , rsthis: & QPinchGesture) -> RetType;
}
impl<'a> /*trait*/ QPinchGesture_setScaleFactor_0<(/*void*/)> for (f64) {
  fn setScaleFactor_0(self , rsthis: & QPinchGesture) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN13QPinchGesture14setScaleFactorEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgesture.h:188
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal totalRotationAngle() const

/*

*/
impl /*struct*/ QPinchGesture {
  pub fn totalRotationAngle_0<RetType, T: QPinchGesture_totalRotationAngle_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.totalRotationAngle_0(self);
    // return 1;
  }
}
pub trait QPinchGesture_totalRotationAngle_0<RetType> {
  fn totalRotationAngle_0(self , rsthis: & QPinchGesture) -> RetType;
}
impl<'a> /*trait*/ QPinchGesture_totalRotationAngle_0<f64> for () {
  fn totalRotationAngle_0(self , rsthis: & QPinchGesture) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QPinchGesture18totalRotationAngleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgesture.h:189
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal lastRotationAngle() const

/*

*/
impl /*struct*/ QPinchGesture {
  pub fn lastRotationAngle_0<RetType, T: QPinchGesture_lastRotationAngle_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.lastRotationAngle_0(self);
    // return 1;
  }
}
pub trait QPinchGesture_lastRotationAngle_0<RetType> {
  fn lastRotationAngle_0(self , rsthis: & QPinchGesture) -> RetType;
}
impl<'a> /*trait*/ QPinchGesture_lastRotationAngle_0<f64> for () {
  fn lastRotationAngle_0(self , rsthis: & QPinchGesture) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QPinchGesture17lastRotationAngleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgesture.h:190
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal rotationAngle() const

/*

*/
impl /*struct*/ QPinchGesture {
  pub fn rotationAngle_0<RetType, T: QPinchGesture_rotationAngle_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rotationAngle_0(self);
    // return 1;
  }
}
pub trait QPinchGesture_rotationAngle_0<RetType> {
  fn rotationAngle_0(self , rsthis: & QPinchGesture) -> RetType;
}
impl<'a> /*trait*/ QPinchGesture_rotationAngle_0<f64> for () {
  fn rotationAngle_0(self , rsthis: & QPinchGesture) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QPinchGesture13rotationAngleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgesture.h:191
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTotalRotationAngle(qreal)

/*

*/
impl /*struct*/ QPinchGesture {
  pub fn setTotalRotationAngle_0<RetType, T: QPinchGesture_setTotalRotationAngle_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTotalRotationAngle_0(self);
    // return 1;
  }
}
pub trait QPinchGesture_setTotalRotationAngle_0<RetType> {
  fn setTotalRotationAngle_0(self , rsthis: & QPinchGesture) -> RetType;
}
impl<'a> /*trait*/ QPinchGesture_setTotalRotationAngle_0<(/*void*/)> for (f64) {
  fn setTotalRotationAngle_0(self , rsthis: & QPinchGesture) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN13QPinchGesture21setTotalRotationAngleEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgesture.h:192
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setLastRotationAngle(qreal)

/*

*/
impl /*struct*/ QPinchGesture {
  pub fn setLastRotationAngle_0<RetType, T: QPinchGesture_setLastRotationAngle_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setLastRotationAngle_0(self);
    // return 1;
  }
}
pub trait QPinchGesture_setLastRotationAngle_0<RetType> {
  fn setLastRotationAngle_0(self , rsthis: & QPinchGesture) -> RetType;
}
impl<'a> /*trait*/ QPinchGesture_setLastRotationAngle_0<(/*void*/)> for (f64) {
  fn setLastRotationAngle_0(self , rsthis: & QPinchGesture) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN13QPinchGesture20setLastRotationAngleEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgesture.h:193
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setRotationAngle(qreal)

/*

*/
impl /*struct*/ QPinchGesture {
  pub fn setRotationAngle_0<RetType, T: QPinchGesture_setRotationAngle_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setRotationAngle_0(self);
    // return 1;
  }
}
pub trait QPinchGesture_setRotationAngle_0<RetType> {
  fn setRotationAngle_0(self , rsthis: & QPinchGesture) -> RetType;
}
impl<'a> /*trait*/ QPinchGesture_setRotationAngle_0<(/*void*/)> for (f64) {
  fn setRotationAngle_0(self , rsthis: & QPinchGesture) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN13QPinchGesture16setRotationAngleEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}


/*


*/
pub type QPinchGesture__ChangeFlag = i32;
// 
pub const QPinchGesture__ScaleFactorChanged :QPinchGesture__ChangeFlag = 1;
// 
pub const QPinchGesture__RotationAngleChanged :QPinchGesture__ChangeFlag = 2;
// 
pub const QPinchGesture__CenterPointChanged :QPinchGesture__ChangeFlag = 4;
pub fn QPinchGesture_ChangeFlagItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QPinchGesture", val);
}
pub fn QPinchGesture_ChangeFlagItemName_s(val: i32) ->String {
  //var nilthis *QPinchGesture
  //return nilthis.ChangeFlagItemName(val);
  return QPinchGesture_ChangeFlagItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
