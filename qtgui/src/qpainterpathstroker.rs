

// mod ::gui::QPainterPathStroker
// package qtgui
// /usr/include/qt/QtGui/qpainterpath.h
// #include <qpainterpath.h>
// #include <QtGui>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 77
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
#[derive(Default)] // class sizeof(QPainterPathStroker)=8
pub struct QPainterPathStroker {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QPainterPathStroker_ITF interface {
//    QPainterPathStroker_PTR() *QPainterPathStroker
//}
//func (ptr *QPainterPathStroker) QPainterPathStroker_PTR() *QPainterPathStroker { return ptr }

impl /*struct*/ QPainterPathStroker {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QPainterPathStroker {
    return QPainterPathStroker{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QPainterPathStroker {
//  type Target = QPainterPathStrokerBASE;
//
//  fn deref(&self) -> &QPainterPathStrokerBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QPainterPathStrokerBASE> for QPainterPathStroker {
//  fn as_ref(& self) -> & QPainterPathStrokerBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qpainterpath.h:246
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QPainterPathStroker()

/*

*/
// QPainterPathStroker() ctx.fn_proto_cpp
impl /*struct*/ QPainterPathStroker {
  pub fn QPainterPathStroker_0<T: QPainterPathStroker_QPainterPathStroker_0>(value: T) -> QPainterPathStroker {
    let rsthis = value.QPainterPathStroker_0();
    return rsthis;
    // return 1;
  }
}

pub trait QPainterPathStroker_QPainterPathStroker_0 {
  fn QPainterPathStroker_0(self) -> QPainterPathStroker;
}
// QPainterPathStroker() ctx.fn_proto_cpp
impl<'a> /*trait*/ QPainterPathStroker_QPainterPathStroker_0 for () {
  fn QPainterPathStroker_0(self) -> QPainterPathStroker {
    // unsafe{_ZN19QPainterPathStrokerC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN19QPainterPathStrokerC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QPainterPathStroker{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainterpath.h:247
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QPainterPathStroker(const QPen &)

/*

*/
// QPainterPathStroker(const QPen &) ctx.fn_proto_cpp
impl /*struct*/ QPainterPathStroker {
  pub fn QPainterPathStroker_1<T: QPainterPathStroker_QPainterPathStroker_1>(value: T) -> QPainterPathStroker {
    let rsthis = value.QPainterPathStroker_1();
    return rsthis;
    // return 1;
  }
}

pub trait QPainterPathStroker_QPainterPathStroker_1 {
  fn QPainterPathStroker_1(self) -> QPainterPathStroker;
}
// QPainterPathStroker(const QPen &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QPainterPathStroker_QPainterPathStroker_1 for (usize) {
  fn QPainterPathStroker_1(self) -> QPainterPathStroker {
    // unsafe{_ZN19QPainterPathStrokerC2ERK4QPen()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN19QPainterPathStrokerC2ERK4QPen", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QPainterPathStroker{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainterpath.h:248
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ~QPainterPathStroker()

/*

*/
pub fn DeleteQPainterPathStroker(this :*mut QPainterPathStroker) {
    // let rv = qtrt::InvokeQtFunc6("_ZN19QPainterPathStrokerD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 8)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qpainterpath.h:250
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setWidth(qreal)

/*

*/
impl /*struct*/ QPainterPathStroker {
  pub fn setWidth_0<RetType, T: QPainterPathStroker_setWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setWidth_0(self);
    // return 1;
  }
}
pub trait QPainterPathStroker_setWidth_0<RetType> {
  fn setWidth_0(self , rsthis: & QPainterPathStroker) -> RetType;
}
impl<'a> /*trait*/ QPainterPathStroker_setWidth_0<(/*void*/)> for (f64) {
  fn setWidth_0(self , rsthis: & QPainterPathStroker) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN19QPainterPathStroker8setWidthEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainterpath.h:251
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal width() const

/*

*/
impl /*struct*/ QPainterPathStroker {
  pub fn width_0<RetType, T: QPainterPathStroker_width_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.width_0(self);
    // return 1;
  }
}
pub trait QPainterPathStroker_width_0<RetType> {
  fn width_0(self , rsthis: & QPainterPathStroker) -> RetType;
}
impl<'a> /*trait*/ QPainterPathStroker_width_0<f64> for () {
  fn width_0(self , rsthis: & QPainterPathStroker) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QPainterPathStroker5widthEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpainterpath.h:253
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCapStyle(Qt::PenCapStyle)

/*

*/
impl /*struct*/ QPainterPathStroker {
  pub fn setCapStyle_0<RetType, T: QPainterPathStroker_setCapStyle_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCapStyle_0(self);
    // return 1;
  }
}
pub trait QPainterPathStroker_setCapStyle_0<RetType> {
  fn setCapStyle_0(self , rsthis: & QPainterPathStroker) -> RetType;
}
impl<'a> /*trait*/ QPainterPathStroker_setCapStyle_0<(/*void*/)> for (i32) {
  fn setCapStyle_0(self , rsthis: & QPainterPathStroker) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN19QPainterPathStroker11setCapStyleEN2Qt11PenCapStyleE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainterpath.h:254
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::PenCapStyle capStyle() const

/*

*/
impl /*struct*/ QPainterPathStroker {
  pub fn capStyle_0<RetType, T: QPainterPathStroker_capStyle_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.capStyle_0(self);
    // return 1;
  }
}
pub trait QPainterPathStroker_capStyle_0<RetType> {
  fn capStyle_0(self , rsthis: & QPainterPathStroker) -> RetType;
}
impl<'a> /*trait*/ QPainterPathStroker_capStyle_0<i32> for () {
  fn capStyle_0(self , rsthis: & QPainterPathStroker) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QPainterPathStroker8capStyleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpainterpath.h:256
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setJoinStyle(Qt::PenJoinStyle)

/*

*/
impl /*struct*/ QPainterPathStroker {
  pub fn setJoinStyle_0<RetType, T: QPainterPathStroker_setJoinStyle_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setJoinStyle_0(self);
    // return 1;
  }
}
pub trait QPainterPathStroker_setJoinStyle_0<RetType> {
  fn setJoinStyle_0(self , rsthis: & QPainterPathStroker) -> RetType;
}
impl<'a> /*trait*/ QPainterPathStroker_setJoinStyle_0<(/*void*/)> for (i32) {
  fn setJoinStyle_0(self , rsthis: & QPainterPathStroker) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN19QPainterPathStroker12setJoinStyleEN2Qt12PenJoinStyleE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainterpath.h:257
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::PenJoinStyle joinStyle() const

/*

*/
impl /*struct*/ QPainterPathStroker {
  pub fn joinStyle_0<RetType, T: QPainterPathStroker_joinStyle_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.joinStyle_0(self);
    // return 1;
  }
}
pub trait QPainterPathStroker_joinStyle_0<RetType> {
  fn joinStyle_0(self , rsthis: & QPainterPathStroker) -> RetType;
}
impl<'a> /*trait*/ QPainterPathStroker_joinStyle_0<i32> for () {
  fn joinStyle_0(self , rsthis: & QPainterPathStroker) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QPainterPathStroker9joinStyleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpainterpath.h:259
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setMiterLimit(qreal)

/*

*/
impl /*struct*/ QPainterPathStroker {
  pub fn setMiterLimit_0<RetType, T: QPainterPathStroker_setMiterLimit_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMiterLimit_0(self);
    // return 1;
  }
}
pub trait QPainterPathStroker_setMiterLimit_0<RetType> {
  fn setMiterLimit_0(self , rsthis: & QPainterPathStroker) -> RetType;
}
impl<'a> /*trait*/ QPainterPathStroker_setMiterLimit_0<(/*void*/)> for (f64) {
  fn setMiterLimit_0(self , rsthis: & QPainterPathStroker) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN19QPainterPathStroker13setMiterLimitEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainterpath.h:260
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal miterLimit() const

/*

*/
impl /*struct*/ QPainterPathStroker {
  pub fn miterLimit_0<RetType, T: QPainterPathStroker_miterLimit_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.miterLimit_0(self);
    // return 1;
  }
}
pub trait QPainterPathStroker_miterLimit_0<RetType> {
  fn miterLimit_0(self , rsthis: & QPainterPathStroker) -> RetType;
}
impl<'a> /*trait*/ QPainterPathStroker_miterLimit_0<f64> for () {
  fn miterLimit_0(self , rsthis: & QPainterPathStroker) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QPainterPathStroker10miterLimitEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpainterpath.h:262
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCurveThreshold(qreal)

/*

*/
impl /*struct*/ QPainterPathStroker {
  pub fn setCurveThreshold_0<RetType, T: QPainterPathStroker_setCurveThreshold_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCurveThreshold_0(self);
    // return 1;
  }
}
pub trait QPainterPathStroker_setCurveThreshold_0<RetType> {
  fn setCurveThreshold_0(self , rsthis: & QPainterPathStroker) -> RetType;
}
impl<'a> /*trait*/ QPainterPathStroker_setCurveThreshold_0<(/*void*/)> for (f64) {
  fn setCurveThreshold_0(self , rsthis: & QPainterPathStroker) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN19QPainterPathStroker17setCurveThresholdEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainterpath.h:263
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal curveThreshold() const

/*

*/
impl /*struct*/ QPainterPathStroker {
  pub fn curveThreshold_0<RetType, T: QPainterPathStroker_curveThreshold_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.curveThreshold_0(self);
    // return 1;
  }
}
pub trait QPainterPathStroker_curveThreshold_0<RetType> {
  fn curveThreshold_0(self , rsthis: & QPainterPathStroker) -> RetType;
}
impl<'a> /*trait*/ QPainterPathStroker_curveThreshold_0<f64> for () {
  fn curveThreshold_0(self , rsthis: & QPainterPathStroker) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QPainterPathStroker14curveThresholdEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpainterpath.h:265
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDashPattern(Qt::PenStyle)

/*

*/
impl /*struct*/ QPainterPathStroker {
  pub fn setDashPattern_0<RetType, T: QPainterPathStroker_setDashPattern_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDashPattern_0(self);
    // return 1;
  }
}
pub trait QPainterPathStroker_setDashPattern_0<RetType> {
  fn setDashPattern_0(self , rsthis: & QPainterPathStroker) -> RetType;
}
impl<'a> /*trait*/ QPainterPathStroker_setDashPattern_0<(/*void*/)> for (i32) {
  fn setDashPattern_0(self , rsthis: & QPainterPathStroker) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN19QPainterPathStroker14setDashPatternEN2Qt8PenStyleE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainterpath.h:269
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDashOffset(qreal)

/*

*/
impl /*struct*/ QPainterPathStroker {
  pub fn setDashOffset_0<RetType, T: QPainterPathStroker_setDashOffset_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDashOffset_0(self);
    // return 1;
  }
}
pub trait QPainterPathStroker_setDashOffset_0<RetType> {
  fn setDashOffset_0(self , rsthis: & QPainterPathStroker) -> RetType;
}
impl<'a> /*trait*/ QPainterPathStroker_setDashOffset_0<(/*void*/)> for (f64) {
  fn setDashOffset_0(self , rsthis: & QPainterPathStroker) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN19QPainterPathStroker13setDashOffsetEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainterpath.h:270
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal dashOffset() const

/*

*/
impl /*struct*/ QPainterPathStroker {
  pub fn dashOffset_0<RetType, T: QPainterPathStroker_dashOffset_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dashOffset_0(self);
    // return 1;
  }
}
pub trait QPainterPathStroker_dashOffset_0<RetType> {
  fn dashOffset_0(self , rsthis: & QPainterPathStroker) -> RetType;
}
impl<'a> /*trait*/ QPainterPathStroker_dashOffset_0<f64> for () {
  fn dashOffset_0(self , rsthis: & QPainterPathStroker) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QPainterPathStroker10dashOffsetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpainterpath.h:272
// index:0
// Public Visibility=Default Availability=Available
// [8] QPainterPath createStroke(const QPainterPath &) const

/*

*/
impl /*struct*/ QPainterPathStroker {
  pub fn createStroke_0<RetType, T: QPainterPathStroker_createStroke_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.createStroke_0(self);
    // return 1;
  }
}
pub trait QPainterPathStroker_createStroke_0<RetType> {
  fn createStroke_0(self , rsthis: & QPainterPathStroker) -> RetType;
}
impl<'a> /*trait*/ QPainterPathStroker_createStroke_0<usize> for (usize) {
  fn createStroke_0(self , rsthis: & QPainterPathStroker) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QPainterPathStroker12createStrokeERK12QPainterPath", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
