

// mod ::gui::QGradient
// package qtgui
// /usr/include/qt/QtGui/qbrush.h
// #include <qbrush.h>
// #include <QtGui>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 0
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
#[derive(Default)] // class sizeof(QGradient)=64
pub struct QGradient {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QGradient_ITF interface {
//    QGradient_PTR() *QGradient
//}
//func (ptr *QGradient) QGradient_PTR() *QGradient { return ptr }

impl /*struct*/ QGradient {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QGradient {
    return QGradient{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QGradient {
//  type Target = QGradientBASE;
//
//  fn deref(&self) -> &QGradientBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QGradientBASE> for QGradient {
//  fn as_ref(& self) -> & QGradientBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qbrush.h:206
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QGradient()

/*

*/
// QGradient() ctx.fn_proto_cpp
impl /*struct*/ QGradient {
  pub fn QGradient_0<T: QGradient_QGradient_0>(value: T) -> QGradient {
    let rsthis = value.QGradient_0();
    return rsthis;
    // return 1;
  }
}

pub trait QGradient_QGradient_0 {
  fn QGradient_0(self) -> QGradient;
}
// QGradient() ctx.fn_proto_cpp
impl<'a> /*trait*/ QGradient_QGradient_0 for () {
  fn QGradient_0(self) -> QGradient {
    // unsafe{_ZN9QGradientC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QGradientC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QGradient{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qbrush.h:208
// index:0
// Public inline Visibility=Default Availability=Available
// [4] QGradient::Type type() const

/*

*/
impl /*struct*/ QGradient {
  pub fn type__0<RetType, T: QGradient_type__0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.type__0(self);
    // return 1;
  }
}
pub trait QGradient_type__0<RetType> {
  fn type__0(self , rsthis: & QGradient) -> RetType;
}
impl<'a> /*trait*/ QGradient_type__0<i32> for () {
  fn type__0(self , rsthis: & QGradient) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QGradient4typeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qbrush.h:210
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setSpread(QGradient::Spread)

/*

*/
impl /*struct*/ QGradient {
  pub fn setSpread_0<RetType, T: QGradient_setSpread_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSpread_0(self);
    // return 1;
  }
}
pub trait QGradient_setSpread_0<RetType> {
  fn setSpread_0(self , rsthis: & QGradient) -> RetType;
}
impl<'a> /*trait*/ QGradient_setSpread_0<(/*void*/)> for (i32) {
  fn setSpread_0(self , rsthis: & QGradient) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QGradient9setSpreadENS_6SpreadE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qbrush.h:211
// index:0
// Public inline Visibility=Default Availability=Available
// [4] QGradient::Spread spread() const

/*

*/
impl /*struct*/ QGradient {
  pub fn spread_0<RetType, T: QGradient_spread_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.spread_0(self);
    // return 1;
  }
}
pub trait QGradient_spread_0<RetType> {
  fn spread_0(self , rsthis: & QGradient) -> RetType;
}
impl<'a> /*trait*/ QGradient_spread_0<i32> for () {
  fn spread_0(self , rsthis: & QGradient) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QGradient6spreadEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qbrush.h:213
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setColorAt(qreal, const QColor &)

/*

*/
impl /*struct*/ QGradient {
  pub fn setColorAt_0<RetType, T: QGradient_setColorAt_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setColorAt_0(self);
    // return 1;
  }
}
pub trait QGradient_setColorAt_0<RetType> {
  fn setColorAt_0(self , rsthis: & QGradient) -> RetType;
}
impl<'a> /*trait*/ QGradient_setColorAt_0<(/*void*/)> for (f64,usize) {
  fn setColorAt_0(self , rsthis: & QGradient) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QGradient10setColorAtEdRK6QColor", 2,qtrt::FFITY_DOUBLE,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qbrush.h:218
// index:0
// Public Visibility=Default Availability=Available
// [4] QGradient::CoordinateMode coordinateMode() const

/*

*/
impl /*struct*/ QGradient {
  pub fn coordinateMode_0<RetType, T: QGradient_coordinateMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.coordinateMode_0(self);
    // return 1;
  }
}
pub trait QGradient_coordinateMode_0<RetType> {
  fn coordinateMode_0(self , rsthis: & QGradient) -> RetType;
}
impl<'a> /*trait*/ QGradient_coordinateMode_0<i32> for () {
  fn coordinateMode_0(self , rsthis: & QGradient) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QGradient14coordinateModeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qbrush.h:219
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCoordinateMode(QGradient::CoordinateMode)

/*

*/
impl /*struct*/ QGradient {
  pub fn setCoordinateMode_0<RetType, T: QGradient_setCoordinateMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCoordinateMode_0(self);
    // return 1;
  }
}
pub trait QGradient_setCoordinateMode_0<RetType> {
  fn setCoordinateMode_0(self , rsthis: & QGradient) -> RetType;
}
impl<'a> /*trait*/ QGradient_setCoordinateMode_0<(/*void*/)> for (i32) {
  fn setCoordinateMode_0(self , rsthis: & QGradient) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QGradient17setCoordinateModeENS_14CoordinateModeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qbrush.h:221
// index:0
// Public Visibility=Default Availability=Available
// [4] QGradient::InterpolationMode interpolationMode() const

/*

*/
impl /*struct*/ QGradient {
  pub fn interpolationMode_0<RetType, T: QGradient_interpolationMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.interpolationMode_0(self);
    // return 1;
  }
}
pub trait QGradient_interpolationMode_0<RetType> {
  fn interpolationMode_0(self , rsthis: & QGradient) -> RetType;
}
impl<'a> /*trait*/ QGradient_interpolationMode_0<i32> for () {
  fn interpolationMode_0(self , rsthis: & QGradient) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QGradient17interpolationModeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qbrush.h:222
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setInterpolationMode(QGradient::InterpolationMode)

/*

*/
impl /*struct*/ QGradient {
  pub fn setInterpolationMode_0<RetType, T: QGradient_setInterpolationMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setInterpolationMode_0(self);
    // return 1;
  }
}
pub trait QGradient_setInterpolationMode_0<RetType> {
  fn setInterpolationMode_0(self , rsthis: & QGradient) -> RetType;
}
impl<'a> /*trait*/ QGradient_setInterpolationMode_0<(/*void*/)> for (i32) {
  fn setInterpolationMode_0(self , rsthis: & QGradient) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QGradient20setInterpolationModeENS_17InterpolationModeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qbrush.h:224
// index:0
// Public Visibility=Default Availability=Available
// [1] bool operator==(const QGradient &) const

/*

*/
impl /*struct*/ QGradient {
  pub fn operator_equal_equal_0<RetType, T: QGradient_operator_equal_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_equal_0(self);
    // return 1;
  }
}
pub trait QGradient_operator_equal_equal_0<RetType> {
  fn operator_equal_equal_0(self , rsthis: & QGradient) -> RetType;
}
impl<'a> /*trait*/ QGradient_operator_equal_equal_0<bool> for (usize) {
  fn operator_equal_equal_0(self , rsthis: & QGradient) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QGradienteqERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qbrush.h:225
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator!=(const QGradient &) const

/*

*/
impl /*struct*/ QGradient {
  pub fn operator_not_equal_0<RetType, T: QGradient_operator_not_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_not_equal_0(self);
    // return 1;
  }
}
pub trait QGradient_operator_not_equal_0<RetType> {
  fn operator_not_equal_0(self , rsthis: & QGradient) -> RetType;
}
impl<'a> /*trait*/ QGradient_operator_not_equal_0<bool> for (usize) {
  fn operator_not_equal_0(self , rsthis: & QGradient) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QGradientneERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}


pub fn DeleteQGradient(this :*mut QGradient) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN9QGradientD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}

/*


*/
pub type QGradient__Type = i32;
// 
pub const QGradient__LinearGradient :QGradient__Type = 0;
// 
pub const QGradient__RadialGradient :QGradient__Type = 1;
// 
pub const QGradient__ConicalGradient :QGradient__Type = 2;
// 
pub const QGradient__NoGradient :QGradient__Type = 3;
pub fn QGradient_TypeItemName(val: i32) ->String {
  match val {
     QGradient__LinearGradient => // 0
     {return String::from("LinearGradient");}
     QGradient__RadialGradient => // 1
     {return String::from("RadialGradient");}
     QGradient__ConicalGradient => // 2
     {return String::from("ConicalGradient");}
     QGradient__NoGradient => // 3
     {return String::from("NoGradient");}
  _ => {return format!("{}", val);}
}
}
pub fn QGradient_TypeItemName_s(val: i32) ->String {
  //var nilthis *QGradient
  //return nilthis.TypeItemName(val);
  return QGradient_TypeItemName(val);
}


/*


*/
pub type QGradient__Spread = i32;
// 
pub const QGradient__PadSpread :QGradient__Spread = 0;
// 
pub const QGradient__ReflectSpread :QGradient__Spread = 1;
// 
pub const QGradient__RepeatSpread :QGradient__Spread = 2;
pub fn QGradient_SpreadItemName(val: i32) ->String {
  match val {
     QGradient__PadSpread => // 0
     {return String::from("PadSpread");}
     QGradient__ReflectSpread => // 1
     {return String::from("ReflectSpread");}
     QGradient__RepeatSpread => // 2
     {return String::from("RepeatSpread");}
  _ => {return format!("{}", val);}
}
}
pub fn QGradient_SpreadItemName_s(val: i32) ->String {
  //var nilthis *QGradient
  //return nilthis.SpreadItemName(val);
  return QGradient_SpreadItemName(val);
}


/*


*/
pub type QGradient__CoordinateMode = i32;
// 
pub const QGradient__LogicalMode :QGradient__CoordinateMode = 0;
// 
pub const QGradient__StretchToDeviceMode :QGradient__CoordinateMode = 1;
// 
pub const QGradient__ObjectBoundingMode :QGradient__CoordinateMode = 2;
pub fn QGradient_CoordinateModeItemName(val: i32) ->String {
  match val {
     QGradient__LogicalMode => // 0
     {return String::from("LogicalMode");}
     QGradient__StretchToDeviceMode => // 1
     {return String::from("StretchToDeviceMode");}
     QGradient__ObjectBoundingMode => // 2
     {return String::from("ObjectBoundingMode");}
  _ => {return format!("{}", val);}
}
}
pub fn QGradient_CoordinateModeItemName_s(val: i32) ->String {
  //var nilthis *QGradient
  //return nilthis.CoordinateModeItemName(val);
  return QGradient_CoordinateModeItemName(val);
}


/*


*/
pub type QGradient__InterpolationMode = i32;
// 
pub const QGradient__ColorInterpolation :QGradient__InterpolationMode = 0;
// 
pub const QGradient__ComponentInterpolation :QGradient__InterpolationMode = 1;
pub fn QGradient_InterpolationModeItemName(val: i32) ->String {
  match val {
     QGradient__ColorInterpolation => // 0
     {return String::from("ColorInterpolation");}
     QGradient__ComponentInterpolation => // 1
     {return String::from("ComponentInterpolation");}
  _ => {return format!("{}", val);}
}
}
pub fn QGradient_InterpolationModeItemName_s(val: i32) ->String {
  //var nilthis *QGradient
  //return nilthis.InterpolationModeItemName(val);
  return QGradient_InterpolationModeItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
