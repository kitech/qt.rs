

// mod ::gui::QPaintEngine
// package qtgui
// /usr/include/qt/QtGui/qpaintengine.h
// #include <qpaintengine.h>
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
#[derive(Default)] // class sizeof(QPaintEngine)=32
pub struct QPaintEngine {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QPaintEngine_ITF interface {
//    QPaintEngine_PTR() *QPaintEngine
//}
//func (ptr *QPaintEngine) QPaintEngine_PTR() *QPaintEngine { return ptr }

impl /*struct*/ QPaintEngine {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QPaintEngine {
    return QPaintEngine{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QPaintEngine {
//  type Target = QPaintEngineBASE;
//
//  fn deref(&self) -> &QPaintEngineBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QPaintEngineBASE> for QPaintEngine {
//  fn as_ref(& self) -> & QPaintEngineBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qpaintengine.h:147
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QPaintEngine(QPaintEngine::PaintEngineFeatures)

/*
Creates a paint engine with the featureset specified by caps.
*/
// QPaintEngine(QPaintEngine::PaintEngineFeatures) ctx.fn_proto_cpp
impl /*struct*/ QPaintEngine {
  pub fn QPaintEngine_0<T: QPaintEngine_QPaintEngine_0>(value: T) -> QPaintEngine {
    let rsthis = value.QPaintEngine_0();
    return rsthis;
    // return 1;
  }
}

pub trait QPaintEngine_QPaintEngine_0 {
  fn QPaintEngine_0(self) -> QPaintEngine;
}
// QPaintEngine(QPaintEngine::PaintEngineFeatures) ctx.fn_proto_cpp
impl<'a> /*trait*/ QPaintEngine_QPaintEngine_0 for (i32) {
  fn QPaintEngine_0(self) -> QPaintEngine {
    // unsafe{_ZN12QPaintEngineC2E6QFlagsINS_18PaintEngineFeatureEE()};
    let arg0 = (&self) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN12QPaintEngineC2E6QFlagsINS_18PaintEngineFeatureEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QPaintEngine{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpaintengine.h:148
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QPaintEngine()

/*

*/
pub fn DeleteQPaintEngine(this :*mut QPaintEngine) {
    // let rv = qtrt::InvokeQtFunc6("_ZN12QPaintEngineD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 32)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qpaintengine.h:150
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isActive() const

/*
Returns true if the paint engine is actively drawing; otherwise returns false.

See also setActive().
*/
impl /*struct*/ QPaintEngine {
  pub fn isActive_0<RetType, T: QPaintEngine_isActive_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isActive_0(self);
    // return 1;
  }
}
pub trait QPaintEngine_isActive_0<RetType> {
  fn isActive_0(self , rsthis: & QPaintEngine) -> RetType;
}
impl<'a> /*trait*/ QPaintEngine_isActive_0<bool> for () {
  fn isActive_0(self , rsthis: & QPaintEngine) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QPaintEngine8isActiveEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpaintengine.h:151
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setActive(bool)

/*
Sets the active state of the paint engine to state.

See also isActive().
*/
impl /*struct*/ QPaintEngine {
  pub fn setActive_0<RetType, T: QPaintEngine_setActive_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setActive_0(self);
    // return 1;
  }
}
pub trait QPaintEngine_setActive_0<RetType> {
  fn setActive_0(self , rsthis: & QPaintEngine) -> RetType;
}
impl<'a> /*trait*/ QPaintEngine_setActive_0<(/*void*/)> for (bool) {
  fn setActive_0(self , rsthis: & QPaintEngine) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN12QPaintEngine9setActiveEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpaintengine.h:153
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [1] bool begin(QPaintDevice *)

/*
Reimplement this function to initialise your paint engine when painting is to start on the paint device pdev. Return true if the initialization was successful; otherwise return false.

See also end() and isActive().
*/
impl /*struct*/ QPaintEngine {
  pub fn begin_0<RetType, T: QPaintEngine_begin_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.begin_0(self);
    // return 1;
  }
}
pub trait QPaintEngine_begin_0<RetType> {
  fn begin_0(self , rsthis: & QPaintEngine) -> RetType;
}
impl<'a> /*trait*/ QPaintEngine_begin_0<bool> for (usize) {
  fn begin_0(self , rsthis: & QPaintEngine) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN12QPaintEngine5beginEP12QPaintDevice", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpaintengine.h:154
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [1] bool end()

/*
Reimplement this function to finish painting on the current paint device. Return true if painting was finished successfully; otherwise return false.

See also begin() and isActive().
*/
impl /*struct*/ QPaintEngine {
  pub fn end_0<RetType, T: QPaintEngine_end_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.end_0(self);
    // return 1;
  }
}
pub trait QPaintEngine_end_0<RetType> {
  fn end_0(self , rsthis: & QPaintEngine) -> RetType;
}
impl<'a> /*trait*/ QPaintEngine_end_0<bool> for () {
  fn end_0(self , rsthis: & QPaintEngine) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN12QPaintEngine3endEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpaintengine.h:156
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [-2] void updateState(const QPaintEngineState &)

/*
Reimplement this function to update the state of a paint engine.

When implemented, this function is responsible for checking the paint engine's current state and update the properties that are changed. Use the QPaintEngineState::state() function to find out which properties that must be updated, then use the corresponding get function to retrieve the current values for the given properties.

See also QPaintEngineState.
*/
impl /*struct*/ QPaintEngine {
  pub fn updateState_0<RetType, T: QPaintEngine_updateState_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.updateState_0(self);
    // return 1;
  }
}
pub trait QPaintEngine_updateState_0<RetType> {
  fn updateState_0(self , rsthis: & QPaintEngine) -> RetType;
}
impl<'a> /*trait*/ QPaintEngine_updateState_0<(/*void*/)> for (usize) {
  fn updateState_0(self , rsthis: & QPaintEngine) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QPaintEngine11updateStateERK17QPaintEngineState", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpaintengine.h:158
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void drawRects(const QRect *, int)

/*
Draws the first rectCount rectangles in the buffer rects. The default implementation of this function calls drawPath() or drawPolygon() depending on the feature set of the paint engine.
*/
impl /*struct*/ QPaintEngine {
  pub fn drawRects_0<RetType, T: QPaintEngine_drawRects_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawRects_0(self);
    // return 1;
  }
}
pub trait QPaintEngine_drawRects_0<RetType> {
  fn drawRects_0(self , rsthis: & QPaintEngine) -> RetType;
}
impl<'a> /*trait*/ QPaintEngine_drawRects_0<(/*void*/)> for (usize,i32) {
  fn drawRects_0(self , rsthis: & QPaintEngine) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN12QPaintEngine9drawRectsEPK5QRecti", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpaintengine.h:159
// index:1
// Public virtual Visibility=Default Availability=Available
// [-2] void drawRects(const QRectF *, int)

/*
Draws the first rectCount rectangles in the buffer rects. The default implementation of this function calls drawPath() or drawPolygon() depending on the feature set of the paint engine.
*/
impl /*struct*/ QPaintEngine {
  pub fn drawRects_1<RetType, T: QPaintEngine_drawRects_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawRects_1(self);
    // return 1;
  }
}
pub trait QPaintEngine_drawRects_1<RetType> {
  fn drawRects_1(self , rsthis: & QPaintEngine) -> RetType;
}
impl<'a> /*trait*/ QPaintEngine_drawRects_1<(/*void*/)> for (usize,i32) {
  fn drawRects_1(self , rsthis: & QPaintEngine) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN12QPaintEngine9drawRectsEPK6QRectFi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpaintengine.h:161
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void drawLines(const QLine *, int)

/*
The default implementation splits the list of lines in lines into lineCount separate calls to drawPath() or drawPolygon() depending on the feature set of the paint engine.
*/
impl /*struct*/ QPaintEngine {
  pub fn drawLines_0<RetType, T: QPaintEngine_drawLines_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawLines_0(self);
    // return 1;
  }
}
pub trait QPaintEngine_drawLines_0<RetType> {
  fn drawLines_0(self , rsthis: & QPaintEngine) -> RetType;
}
impl<'a> /*trait*/ QPaintEngine_drawLines_0<(/*void*/)> for (usize,i32) {
  fn drawLines_0(self , rsthis: & QPaintEngine) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN12QPaintEngine9drawLinesEPK5QLinei", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpaintengine.h:162
// index:1
// Public virtual Visibility=Default Availability=Available
// [-2] void drawLines(const QLineF *, int)

/*
The default implementation splits the list of lines in lines into lineCount separate calls to drawPath() or drawPolygon() depending on the feature set of the paint engine.
*/
impl /*struct*/ QPaintEngine {
  pub fn drawLines_1<RetType, T: QPaintEngine_drawLines_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawLines_1(self);
    // return 1;
  }
}
pub trait QPaintEngine_drawLines_1<RetType> {
  fn drawLines_1(self , rsthis: & QPaintEngine) -> RetType;
}
impl<'a> /*trait*/ QPaintEngine_drawLines_1<(/*void*/)> for (usize,i32) {
  fn drawLines_1(self , rsthis: & QPaintEngine) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN12QPaintEngine9drawLinesEPK6QLineFi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpaintengine.h:164
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void drawEllipse(const QRectF &)

/*
Reimplement this function to draw the largest ellipse that can be contained within rectangle rect.

The default implementation calls drawPolygon().
*/
impl /*struct*/ QPaintEngine {
  pub fn drawEllipse_0<RetType, T: QPaintEngine_drawEllipse_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawEllipse_0(self);
    // return 1;
  }
}
pub trait QPaintEngine_drawEllipse_0<RetType> {
  fn drawEllipse_0(self , rsthis: & QPaintEngine) -> RetType;
}
impl<'a> /*trait*/ QPaintEngine_drawEllipse_0<(/*void*/)> for (usize) {
  fn drawEllipse_0(self , rsthis: & QPaintEngine) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QPaintEngine11drawEllipseERK6QRectF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpaintengine.h:165
// index:1
// Public virtual Visibility=Default Availability=Available
// [-2] void drawEllipse(const QRect &)

/*
Reimplement this function to draw the largest ellipse that can be contained within rectangle rect.

The default implementation calls drawPolygon().
*/
impl /*struct*/ QPaintEngine {
  pub fn drawEllipse_1<RetType, T: QPaintEngine_drawEllipse_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawEllipse_1(self);
    // return 1;
  }
}
pub trait QPaintEngine_drawEllipse_1<RetType> {
  fn drawEllipse_1(self , rsthis: & QPaintEngine) -> RetType;
}
impl<'a> /*trait*/ QPaintEngine_drawEllipse_1<(/*void*/)> for (usize) {
  fn drawEllipse_1(self , rsthis: & QPaintEngine) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QPaintEngine11drawEllipseERK5QRect", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpaintengine.h:167
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void drawPath(const QPainterPath &)

/*
The default implementation ignores the path and does nothing.
*/
impl /*struct*/ QPaintEngine {
  pub fn drawPath_0<RetType, T: QPaintEngine_drawPath_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawPath_0(self);
    // return 1;
  }
}
pub trait QPaintEngine_drawPath_0<RetType> {
  fn drawPath_0(self , rsthis: & QPaintEngine) -> RetType;
}
impl<'a> /*trait*/ QPaintEngine_drawPath_0<(/*void*/)> for (usize) {
  fn drawPath_0(self , rsthis: & QPaintEngine) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QPaintEngine8drawPathERK12QPainterPath", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpaintengine.h:169
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void drawPoints(const QPointF *, int)

/*
Draws the first pointCount points in the buffer points
*/
impl /*struct*/ QPaintEngine {
  pub fn drawPoints_0<RetType, T: QPaintEngine_drawPoints_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawPoints_0(self);
    // return 1;
  }
}
pub trait QPaintEngine_drawPoints_0<RetType> {
  fn drawPoints_0(self , rsthis: & QPaintEngine) -> RetType;
}
impl<'a> /*trait*/ QPaintEngine_drawPoints_0<(/*void*/)> for (usize,i32) {
  fn drawPoints_0(self , rsthis: & QPaintEngine) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN12QPaintEngine10drawPointsEPK7QPointFi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpaintengine.h:170
// index:1
// Public virtual Visibility=Default Availability=Available
// [-2] void drawPoints(const QPoint *, int)

/*
Draws the first pointCount points in the buffer points
*/
impl /*struct*/ QPaintEngine {
  pub fn drawPoints_1<RetType, T: QPaintEngine_drawPoints_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawPoints_1(self);
    // return 1;
  }
}
pub trait QPaintEngine_drawPoints_1<RetType> {
  fn drawPoints_1(self , rsthis: & QPaintEngine) -> RetType;
}
impl<'a> /*trait*/ QPaintEngine_drawPoints_1<(/*void*/)> for (usize,i32) {
  fn drawPoints_1(self , rsthis: & QPaintEngine) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN12QPaintEngine10drawPointsEPK6QPointi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpaintengine.h:172
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void drawPolygon(const QPointF *, int, QPaintEngine::PolygonDrawMode)

/*
Reimplement this virtual function to draw the polygon defined by the pointCount first points in points, using mode mode.

Note: At least one of the drawPolygon() functions must be reimplemented.
*/
impl /*struct*/ QPaintEngine {
  pub fn drawPolygon_0<RetType, T: QPaintEngine_drawPolygon_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawPolygon_0(self);
    // return 1;
  }
}
pub trait QPaintEngine_drawPolygon_0<RetType> {
  fn drawPolygon_0(self , rsthis: & QPaintEngine) -> RetType;
}
impl<'a> /*trait*/ QPaintEngine_drawPolygon_0<(/*void*/)> for (usize,i32,i32) {
  fn drawPolygon_0(self , rsthis: & QPaintEngine) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN12QPaintEngine11drawPolygonEPK7QPointFiNS_15PolygonDrawModeE", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpaintengine.h:173
// index:1
// Public virtual Visibility=Default Availability=Available
// [-2] void drawPolygon(const QPoint *, int, QPaintEngine::PolygonDrawMode)

/*
Reimplement this virtual function to draw the polygon defined by the pointCount first points in points, using mode mode.

Note: At least one of the drawPolygon() functions must be reimplemented.
*/
impl /*struct*/ QPaintEngine {
  pub fn drawPolygon_1<RetType, T: QPaintEngine_drawPolygon_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawPolygon_1(self);
    // return 1;
  }
}
pub trait QPaintEngine_drawPolygon_1<RetType> {
  fn drawPolygon_1(self , rsthis: & QPaintEngine) -> RetType;
}
impl<'a> /*trait*/ QPaintEngine_drawPolygon_1<(/*void*/)> for (usize,i32,i32) {
  fn drawPolygon_1(self , rsthis: & QPaintEngine) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN12QPaintEngine11drawPolygonEPK6QPointiNS_15PolygonDrawModeE", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpaintengine.h:175
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [-2] void drawPixmap(const QRectF &, const QPixmap &, const QRectF &)

/*
Reimplement this function to draw the part of the pm specified by the sr rectangle in the given r.
*/
impl /*struct*/ QPaintEngine {
  pub fn drawPixmap_0<RetType, T: QPaintEngine_drawPixmap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawPixmap_0(self);
    // return 1;
  }
}
pub trait QPaintEngine_drawPixmap_0<RetType> {
  fn drawPixmap_0(self , rsthis: & QPaintEngine) -> RetType;
}
impl<'a> /*trait*/ QPaintEngine_drawPixmap_0<(/*void*/)> for (usize,usize,usize) {
  fn drawPixmap_0(self , rsthis: & QPaintEngine) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QPaintEngine10drawPixmapERK6QRectFRK7QPixmapS2_", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpaintengine.h:176
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void drawTextItem(const QPointF &, const QTextItem &)

/*
This function draws the text item textItem at position p. The default implementation of this function converts the text to a QPainterPath and paints the resulting path.
*/
impl /*struct*/ QPaintEngine {
  pub fn drawTextItem_0<RetType, T: QPaintEngine_drawTextItem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawTextItem_0(self);
    // return 1;
  }
}
pub trait QPaintEngine_drawTextItem_0<RetType> {
  fn drawTextItem_0(self , rsthis: & QPaintEngine) -> RetType;
}
impl<'a> /*trait*/ QPaintEngine_drawTextItem_0<(/*void*/)> for (usize,usize) {
  fn drawTextItem_0(self , rsthis: & QPaintEngine) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QPaintEngine12drawTextItemERK7QPointFRK9QTextItem", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpaintengine.h:177
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void drawTiledPixmap(const QRectF &, const QPixmap &, const QPointF &)

/*
Reimplement this function to draw the pixmap in the given rect, starting at the given p. The pixmap will be drawn repeatedly until the rect is filled.
*/
impl /*struct*/ QPaintEngine {
  pub fn drawTiledPixmap_0<RetType, T: QPaintEngine_drawTiledPixmap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawTiledPixmap_0(self);
    // return 1;
  }
}
pub trait QPaintEngine_drawTiledPixmap_0<RetType> {
  fn drawTiledPixmap_0(self , rsthis: & QPaintEngine) -> RetType;
}
impl<'a> /*trait*/ QPaintEngine_drawTiledPixmap_0<(/*void*/)> for (usize,usize,usize) {
  fn drawTiledPixmap_0(self , rsthis: & QPaintEngine) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QPaintEngine15drawTiledPixmapERK6QRectFRK7QPixmapRK7QPointF", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpaintengine.h:178
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void drawImage(const QRectF &, const QImage &, const QRectF &, Qt::ImageConversionFlags)

/*
Reimplement this function to draw the part of the image specified by the sr rectangle in the given rectangle using the given conversion flags flags, to convert it to a pixmap.
*/
impl /*struct*/ QPaintEngine {
  pub fn drawImage_0<RetType, T: QPaintEngine_drawImage_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawImage_0(self);
    // return 1;
  }
}
pub trait QPaintEngine_drawImage_0<RetType> {
  fn drawImage_0(self , rsthis: & QPaintEngine) -> RetType;
}
impl<'a> /*trait*/ QPaintEngine_drawImage_0<(/*void*/)> for (usize,usize,usize,i32) {
  fn drawImage_0(self , rsthis: & QPaintEngine) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN12QPaintEngine9drawImageERK6QRectFRK6QImageS2_6QFlagsIN2Qt19ImageConversionFlagEE", 4,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpaintengine.h:181
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setPaintDevice(QPaintDevice *)

/*

*/
impl /*struct*/ QPaintEngine {
  pub fn setPaintDevice_0<RetType, T: QPaintEngine_setPaintDevice_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPaintDevice_0(self);
    // return 1;
  }
}
pub trait QPaintEngine_setPaintDevice_0<RetType> {
  fn setPaintDevice_0(self , rsthis: & QPaintEngine) -> RetType;
}
impl<'a> /*trait*/ QPaintEngine_setPaintDevice_0<(/*void*/)> for (usize) {
  fn setPaintDevice_0(self , rsthis: & QPaintEngine) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QPaintEngine14setPaintDeviceEP12QPaintDevice", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpaintengine.h:182
// index:0
// Public Visibility=Default Availability=Available
// [8] QPaintDevice * paintDevice() const

/*
Returns the device that this engine is painting on, if painting is active; otherwise returns 0.
*/
impl /*struct*/ QPaintEngine {
  pub fn paintDevice_0<RetType, T: QPaintEngine_paintDevice_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.paintDevice_0(self);
    // return 1;
  }
}
pub trait QPaintEngine_paintDevice_0<RetType> {
  fn paintDevice_0(self , rsthis: & QPaintEngine) -> RetType;
}
impl<'a> /*trait*/ QPaintEngine_paintDevice_0<usize> for () {
  fn paintDevice_0(self , rsthis: & QPaintEngine) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QPaintEngine11paintDeviceEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpaintengine.h:184
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setSystemClip(const QRegion &)

/*

*/
impl /*struct*/ QPaintEngine {
  pub fn setSystemClip_0<RetType, T: QPaintEngine_setSystemClip_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSystemClip_0(self);
    // return 1;
  }
}
pub trait QPaintEngine_setSystemClip_0<RetType> {
  fn setSystemClip_0(self , rsthis: & QPaintEngine) -> RetType;
}
impl<'a> /*trait*/ QPaintEngine_setSystemClip_0<(/*void*/)> for (usize) {
  fn setSystemClip_0(self , rsthis: & QPaintEngine) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QPaintEngine13setSystemClipERK7QRegion", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpaintengine.h:185
// index:0
// Public Visibility=Default Availability=Available
// [8] QRegion systemClip() const

/*

*/
impl /*struct*/ QPaintEngine {
  pub fn systemClip_0<RetType, T: QPaintEngine_systemClip_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.systemClip_0(self);
    // return 1;
  }
}
pub trait QPaintEngine_systemClip_0<RetType> {
  fn systemClip_0(self , rsthis: & QPaintEngine) -> RetType;
}
impl<'a> /*trait*/ QPaintEngine_systemClip_0<usize> for () {
  fn systemClip_0(self , rsthis: & QPaintEngine) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QPaintEngine10systemClipEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpaintengine.h:187
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setSystemRect(const QRect &)

/*

*/
impl /*struct*/ QPaintEngine {
  pub fn setSystemRect_0<RetType, T: QPaintEngine_setSystemRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSystemRect_0(self);
    // return 1;
  }
}
pub trait QPaintEngine_setSystemRect_0<RetType> {
  fn setSystemRect_0(self , rsthis: & QPaintEngine) -> RetType;
}
impl<'a> /*trait*/ QPaintEngine_setSystemRect_0<(/*void*/)> for (usize) {
  fn setSystemRect_0(self , rsthis: & QPaintEngine) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QPaintEngine13setSystemRectERK5QRect", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpaintengine.h:188
// index:0
// Public Visibility=Default Availability=Available
// [16] QRect systemRect() const

/*

*/
impl /*struct*/ QPaintEngine {
  pub fn systemRect_0<RetType, T: QPaintEngine_systemRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.systemRect_0(self);
    // return 1;
  }
}
pub trait QPaintEngine_systemRect_0<RetType> {
  fn systemRect_0(self , rsthis: & QPaintEngine) -> RetType;
}
impl<'a> /*trait*/ QPaintEngine_systemRect_0<usize> for () {
  fn systemRect_0(self , rsthis: & QPaintEngine) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QPaintEngine10systemRectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpaintengine.h:191
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QPoint coordinateOffset() const

/*

*/
impl /*struct*/ QPaintEngine {
  pub fn coordinateOffset_0<RetType, T: QPaintEngine_coordinateOffset_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.coordinateOffset_0(self);
    // return 1;
  }
}
pub trait QPaintEngine_coordinateOffset_0<RetType> {
  fn coordinateOffset_0(self , rsthis: & QPaintEngine) -> RetType;
}
impl<'a> /*trait*/ QPaintEngine_coordinateOffset_0<usize> for () {
  fn coordinateOffset_0(self , rsthis: & QPaintEngine) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QPaintEngine16coordinateOffsetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpaintengine.h:214
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [4] QPaintEngine::Type type() const

/*
Reimplement this function to return the paint engine Type.
*/
impl /*struct*/ QPaintEngine {
  pub fn type__0<RetType, T: QPaintEngine_type__0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.type__0(self);
    // return 1;
  }
}
pub trait QPaintEngine_type__0<RetType> {
  fn type__0(self , rsthis: & QPaintEngine) -> RetType;
}
impl<'a> /*trait*/ QPaintEngine_type__0<i32> for () {
  fn type__0(self , rsthis: & QPaintEngine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QPaintEngine4typeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpaintengine.h:216
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void fix_neg_rect(int *, int *, int *, int *)

/*

*/
impl /*struct*/ QPaintEngine {
  pub fn fix_neg_rect_0<RetType, T: QPaintEngine_fix_neg_rect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fix_neg_rect_0(self);
    // return 1;
  }
}
pub trait QPaintEngine_fix_neg_rect_0<RetType> {
  fn fix_neg_rect_0(self , rsthis: & QPaintEngine) -> RetType;
}
impl<'a> /*trait*/ QPaintEngine_fix_neg_rect_0<(/*void*/)> for (usize,usize,usize,usize) {
  fn fix_neg_rect_0(self , rsthis: & QPaintEngine) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const usize as usize;
    let arg2 = (&self.2) as *const usize as usize;
    let arg3 = (&self.3) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QPaintEngine12fix_neg_rectEPiS0_S0_S0_", 4,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpaintengine.h:218
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool testDirty(QPaintEngine::DirtyFlags)

/*

*/
impl /*struct*/ QPaintEngine {
  pub fn testDirty_0<RetType, T: QPaintEngine_testDirty_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.testDirty_0(self);
    // return 1;
  }
}
pub trait QPaintEngine_testDirty_0<RetType> {
  fn testDirty_0(self , rsthis: & QPaintEngine) -> RetType;
}
impl<'a> /*trait*/ QPaintEngine_testDirty_0<bool> for (i32) {
  fn testDirty_0(self , rsthis: & QPaintEngine) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN12QPaintEngine9testDirtyE6QFlagsINS_9DirtyFlagEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpaintengine.h:219
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setDirty(QPaintEngine::DirtyFlags)

/*

*/
impl /*struct*/ QPaintEngine {
  pub fn setDirty_0<RetType, T: QPaintEngine_setDirty_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDirty_0(self);
    // return 1;
  }
}
pub trait QPaintEngine_setDirty_0<RetType> {
  fn setDirty_0(self , rsthis: & QPaintEngine) -> RetType;
}
impl<'a> /*trait*/ QPaintEngine_setDirty_0<(/*void*/)> for (i32) {
  fn setDirty_0(self , rsthis: & QPaintEngine) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN12QPaintEngine8setDirtyE6QFlagsINS_9DirtyFlagEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpaintengine.h:220
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void clearDirty(QPaintEngine::DirtyFlags)

/*

*/
impl /*struct*/ QPaintEngine {
  pub fn clearDirty_0<RetType, T: QPaintEngine_clearDirty_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.clearDirty_0(self);
    // return 1;
  }
}
pub trait QPaintEngine_clearDirty_0<RetType> {
  fn clearDirty_0(self , rsthis: & QPaintEngine) -> RetType;
}
impl<'a> /*trait*/ QPaintEngine_clearDirty_0<(/*void*/)> for (i32) {
  fn clearDirty_0(self , rsthis: & QPaintEngine) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN12QPaintEngine10clearDirtyE6QFlagsINS_9DirtyFlagEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpaintengine.h:222
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool hasFeature(QPaintEngine::PaintEngineFeatures) const

/*
Returns true if the paint engine supports the specified feature; otherwise returns false.
*/
impl /*struct*/ QPaintEngine {
  pub fn hasFeature_0<RetType, T: QPaintEngine_hasFeature_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hasFeature_0(self);
    // return 1;
  }
}
pub trait QPaintEngine_hasFeature_0<RetType> {
  fn hasFeature_0(self , rsthis: & QPaintEngine) -> RetType;
}
impl<'a> /*trait*/ QPaintEngine_hasFeature_0<bool> for (i32) {
  fn hasFeature_0(self , rsthis: & QPaintEngine) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QPaintEngine10hasFeatureE6QFlagsINS_18PaintEngineFeatureEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpaintengine.h:224
// index:0
// Public Visibility=Default Availability=Available
// [8] QPainter * painter() const

/*
Returns the paint engine's painter.
*/
impl /*struct*/ QPaintEngine {
  pub fn painter_0<RetType, T: QPaintEngine_painter_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.painter_0(self);
    // return 1;
  }
}
pub trait QPaintEngine_painter_0<RetType> {
  fn painter_0(self , rsthis: & QPaintEngine) -> RetType;
}
impl<'a> /*trait*/ QPaintEngine_painter_0<usize> for () {
  fn painter_0(self , rsthis: & QPaintEngine) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QPaintEngine7painterEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpaintengine.h:226
// index:0
// Public Visibility=Default Availability=Available
// [-2] void syncState()

/*

*/
impl /*struct*/ QPaintEngine {
  pub fn syncState_0<RetType, T: QPaintEngine_syncState_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.syncState_0(self);
    // return 1;
  }
}
pub trait QPaintEngine_syncState_0<RetType> {
  fn syncState_0(self , rsthis: & QPaintEngine) -> RetType;
}
impl<'a> /*trait*/ QPaintEngine_syncState_0<(/*void*/)> for () {
  fn syncState_0(self , rsthis: & QPaintEngine) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN12QPaintEngine9syncStateEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpaintengine.h:227
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isExtended() const

/*

*/
impl /*struct*/ QPaintEngine {
  pub fn isExtended_0<RetType, T: QPaintEngine_isExtended_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isExtended_0(self);
    // return 1;
  }
}
pub trait QPaintEngine_isExtended_0<RetType> {
  fn isExtended_0(self , rsthis: & QPaintEngine) -> RetType;
}
impl<'a> /*trait*/ QPaintEngine_isExtended_0<bool> for () {
  fn isExtended_0(self , rsthis: & QPaintEngine) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QPaintEngine10isExtendedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}


/*


*/
pub type QPaintEngine__PaintEngineFeature = i32;
// 
pub const QPaintEngine__PrimitiveTransform :QPaintEngine__PaintEngineFeature = 1;
// 
pub const QPaintEngine__PatternTransform :QPaintEngine__PaintEngineFeature = 2;
// 
pub const QPaintEngine__PixmapTransform :QPaintEngine__PaintEngineFeature = 4;
// 
pub const QPaintEngine__PatternBrush :QPaintEngine__PaintEngineFeature = 8;
// 
pub const QPaintEngine__LinearGradientFill :QPaintEngine__PaintEngineFeature = 16;
// 
pub const QPaintEngine__RadialGradientFill :QPaintEngine__PaintEngineFeature = 32;
// 
pub const QPaintEngine__ConicalGradientFill :QPaintEngine__PaintEngineFeature = 64;
// 
pub const QPaintEngine__AlphaBlend :QPaintEngine__PaintEngineFeature = 128;
// 
pub const QPaintEngine__PorterDuff :QPaintEngine__PaintEngineFeature = 256;
// 
pub const QPaintEngine__PainterPaths :QPaintEngine__PaintEngineFeature = 512;
// 
pub const QPaintEngine__Antialiasing :QPaintEngine__PaintEngineFeature = 1024;
// 
pub const QPaintEngine__BrushStroke :QPaintEngine__PaintEngineFeature = 2048;
// 
pub const QPaintEngine__ConstantOpacity :QPaintEngine__PaintEngineFeature = 4096;
// 
pub const QPaintEngine__MaskedBrush :QPaintEngine__PaintEngineFeature = 8192;
// 
pub const QPaintEngine__PerspectiveTransform :QPaintEngine__PaintEngineFeature = 16384;
// 
pub const QPaintEngine__BlendModes :QPaintEngine__PaintEngineFeature = 32768;
// 
pub const QPaintEngine__ObjectBoundingModeGradients :QPaintEngine__PaintEngineFeature = 65536;
// 
pub const QPaintEngine__RasterOpModes :QPaintEngine__PaintEngineFeature = 131072;
// 
pub const QPaintEngine__PaintOutsidePaintEvent :QPaintEngine__PaintEngineFeature = 536870912;
// 
pub const QPaintEngine__AllFeatures :QPaintEngine__PaintEngineFeature = -1;
pub fn QPaintEngine_PaintEngineFeatureItemName(val: i32) ->String {
  match val {
     QPaintEngine__PrimitiveTransform => // 1
     {return String::from("PrimitiveTransform");}
     QPaintEngine__PatternTransform => // 2
     {return String::from("PatternTransform");}
     QPaintEngine__PixmapTransform => // 4
     {return String::from("PixmapTransform");}
     QPaintEngine__PatternBrush => // 8
     {return String::from("PatternBrush");}
     QPaintEngine__LinearGradientFill => // 16
     {return String::from("LinearGradientFill");}
     QPaintEngine__RadialGradientFill => // 32
     {return String::from("RadialGradientFill");}
     QPaintEngine__ConicalGradientFill => // 64
     {return String::from("ConicalGradientFill");}
     QPaintEngine__AlphaBlend => // 128
     {return String::from("AlphaBlend");}
     QPaintEngine__PorterDuff => // 256
     {return String::from("PorterDuff");}
     QPaintEngine__PainterPaths => // 512
     {return String::from("PainterPaths");}
     QPaintEngine__Antialiasing => // 1024
     {return String::from("Antialiasing");}
     QPaintEngine__BrushStroke => // 2048
     {return String::from("BrushStroke");}
     QPaintEngine__ConstantOpacity => // 4096
     {return String::from("ConstantOpacity");}
     QPaintEngine__MaskedBrush => // 8192
     {return String::from("MaskedBrush");}
     QPaintEngine__PerspectiveTransform => // 16384
     {return String::from("PerspectiveTransform");}
     QPaintEngine__BlendModes => // 32768
     {return String::from("BlendModes");}
     QPaintEngine__ObjectBoundingModeGradients => // 65536
     {return String::from("ObjectBoundingModeGradients");}
     QPaintEngine__RasterOpModes => // 131072
     {return String::from("RasterOpModes");}
     QPaintEngine__PaintOutsidePaintEvent => // 536870912
     {return String::from("PaintOutsidePaintEvent");}
     QPaintEngine__AllFeatures => // -1
     {return String::from("AllFeatures");}
  _ => {return format!("{}", val);}
}
}
pub fn QPaintEngine_PaintEngineFeatureItemName_s(val: i32) ->String {
  //var nilthis *QPaintEngine
  //return nilthis.PaintEngineFeatureItemName(val);
  return QPaintEngine_PaintEngineFeatureItemName(val);
}


/*


*/
pub type QPaintEngine__DirtyFlag = i32;
// 
pub const QPaintEngine__DirtyPen :QPaintEngine__DirtyFlag = 1;
// 
pub const QPaintEngine__DirtyBrush :QPaintEngine__DirtyFlag = 2;
// 
pub const QPaintEngine__DirtyBrushOrigin :QPaintEngine__DirtyFlag = 4;
// 
pub const QPaintEngine__DirtyFont :QPaintEngine__DirtyFlag = 8;
// 
pub const QPaintEngine__DirtyBackground :QPaintEngine__DirtyFlag = 16;
// 
pub const QPaintEngine__DirtyBackgroundMode :QPaintEngine__DirtyFlag = 32;
// 
pub const QPaintEngine__DirtyTransform :QPaintEngine__DirtyFlag = 64;
// 
pub const QPaintEngine__DirtyClipRegion :QPaintEngine__DirtyFlag = 128;
// 
pub const QPaintEngine__DirtyClipPath :QPaintEngine__DirtyFlag = 256;
// 
pub const QPaintEngine__DirtyHints :QPaintEngine__DirtyFlag = 512;
// 
pub const QPaintEngine__DirtyCompositionMode :QPaintEngine__DirtyFlag = 1024;
// 
pub const QPaintEngine__DirtyClipEnabled :QPaintEngine__DirtyFlag = 2048;
// 
pub const QPaintEngine__DirtyOpacity :QPaintEngine__DirtyFlag = 4096;
// 
pub const QPaintEngine__AllDirty :QPaintEngine__DirtyFlag = 65535;
pub fn QPaintEngine_DirtyFlagItemName(val: i32) ->String {
  match val {
     QPaintEngine__DirtyPen => // 1
     {return String::from("DirtyPen");}
     QPaintEngine__DirtyBrush => // 2
     {return String::from("DirtyBrush");}
     QPaintEngine__DirtyBrushOrigin => // 4
     {return String::from("DirtyBrushOrigin");}
     QPaintEngine__DirtyFont => // 8
     {return String::from("DirtyFont");}
     QPaintEngine__DirtyBackground => // 16
     {return String::from("DirtyBackground");}
     QPaintEngine__DirtyBackgroundMode => // 32
     {return String::from("DirtyBackgroundMode");}
     QPaintEngine__DirtyTransform => // 64
     {return String::from("DirtyTransform");}
     QPaintEngine__DirtyClipRegion => // 128
     {return String::from("DirtyClipRegion");}
     QPaintEngine__DirtyClipPath => // 256
     {return String::from("DirtyClipPath");}
     QPaintEngine__DirtyHints => // 512
     {return String::from("DirtyHints");}
     QPaintEngine__DirtyCompositionMode => // 1024
     {return String::from("DirtyCompositionMode");}
     QPaintEngine__DirtyClipEnabled => // 2048
     {return String::from("DirtyClipEnabled");}
     QPaintEngine__DirtyOpacity => // 4096
     {return String::from("DirtyOpacity");}
     QPaintEngine__AllDirty => // 65535
     {return String::from("AllDirty");}
  _ => {return format!("{}", val);}
}
}
pub fn QPaintEngine_DirtyFlagItemName_s(val: i32) ->String {
  //var nilthis *QPaintEngine
  //return nilthis.DirtyFlagItemName(val);
  return QPaintEngine_DirtyFlagItemName(val);
}


/*

*/
pub type QPaintEngine__PolygonDrawMode = i32;
// The polygon should be drawn using OddEven fill rule.
pub const QPaintEngine__OddEvenMode :QPaintEngine__PolygonDrawMode = 0;
// The polygon should be drawn using Winding fill rule.
pub const QPaintEngine__WindingMode :QPaintEngine__PolygonDrawMode = 1;
// The polygon is a convex polygon and can be drawn using specialized algorithms where available.
pub const QPaintEngine__ConvexMode :QPaintEngine__PolygonDrawMode = 2;
// Only the outline of the polygon should be drawn.
pub const QPaintEngine__PolylineMode :QPaintEngine__PolygonDrawMode = 3;
pub fn QPaintEngine_PolygonDrawModeItemName(val: i32) ->String {
  match val {
     QPaintEngine__OddEvenMode => // 0
     {return String::from("OddEvenMode");}
     QPaintEngine__WindingMode => // 1
     {return String::from("WindingMode");}
     QPaintEngine__ConvexMode => // 2
     {return String::from("ConvexMode");}
     QPaintEngine__PolylineMode => // 3
     {return String::from("PolylineMode");}
  _ => {return format!("{}", val);}
}
}
pub fn QPaintEngine_PolygonDrawModeItemName_s(val: i32) ->String {
  //var nilthis *QPaintEngine
  //return nilthis.PolygonDrawModeItemName(val);
  return QPaintEngine_PolygonDrawModeItemName(val);
}


/*

*/
pub type QPaintEngine__Type = i32;
//  
pub const QPaintEngine__X11 :QPaintEngine__Type = 0;
//  
pub const QPaintEngine__Windows :QPaintEngine__Type = 1;
// macOS's QuickDraw
pub const QPaintEngine__QuickDraw :QPaintEngine__Type = 2;
// 
pub const QPaintEngine__CoreGraphics :QPaintEngine__Type = 3;
//  
pub const QPaintEngine__MacPrinter :QPaintEngine__Type = 4;
// Qt for Embedded Linux
pub const QPaintEngine__QWindowSystem :QPaintEngine__Type = 5;
// (No longer supported)
pub const QPaintEngine__PostScript :QPaintEngine__Type = 6;
//  
pub const QPaintEngine__OpenGL :QPaintEngine__Type = 7;
// QPicture format
pub const QPaintEngine__Picture :QPaintEngine__Type = 8;
// Scalable Vector Graphics XML format
pub const QPaintEngine__SVG :QPaintEngine__Type = 9;
// 
pub const QPaintEngine__Raster :QPaintEngine__Type = 10;
// 
pub const QPaintEngine__Direct3D :QPaintEngine__Type = 11;
// 
pub const QPaintEngine__Pdf :QPaintEngine__Type = 12;
// 
pub const QPaintEngine__OpenVG :QPaintEngine__Type = 13;
// 
pub const QPaintEngine__OpenGL2 :QPaintEngine__Type = 14;
// 
pub const QPaintEngine__PaintBuffer :QPaintEngine__Type = 15;
// 
pub const QPaintEngine__Blitter :QPaintEngine__Type = 16;
// 
pub const QPaintEngine__Direct2D :QPaintEngine__Type = 17;
// 
pub const QPaintEngine__User :QPaintEngine__Type = 50;
// 
pub const QPaintEngine__MaxUser :QPaintEngine__Type = 100;
pub fn QPaintEngine_TypeItemName(val: i32) ->String {
  match val {
     QPaintEngine__X11 => // 0
     {return String::from("X11");}
     QPaintEngine__Windows => // 1
     {return String::from("Windows");}
     QPaintEngine__QuickDraw => // 2
     {return String::from("QuickDraw");}
     QPaintEngine__CoreGraphics => // 3
     {return String::from("CoreGraphics");}
     QPaintEngine__MacPrinter => // 4
     {return String::from("MacPrinter");}
     QPaintEngine__QWindowSystem => // 5
     {return String::from("QWindowSystem");}
     QPaintEngine__PostScript => // 6
     {return String::from("PostScript");}
     QPaintEngine__OpenGL => // 7
     {return String::from("OpenGL");}
     QPaintEngine__Picture => // 8
     {return String::from("Picture");}
     QPaintEngine__SVG => // 9
     {return String::from("SVG");}
     QPaintEngine__Raster => // 10
     {return String::from("Raster");}
     QPaintEngine__Direct3D => // 11
     {return String::from("Direct3D");}
     QPaintEngine__Pdf => // 12
     {return String::from("Pdf");}
     QPaintEngine__OpenVG => // 13
     {return String::from("OpenVG");}
     QPaintEngine__OpenGL2 => // 14
     {return String::from("OpenGL2");}
     QPaintEngine__PaintBuffer => // 15
     {return String::from("PaintBuffer");}
     QPaintEngine__Blitter => // 16
     {return String::from("Blitter");}
     QPaintEngine__Direct2D => // 17
     {return String::from("Direct2D");}
     QPaintEngine__User => // 50
     {return String::from("User");}
     QPaintEngine__MaxUser => // 100
     {return String::from("MaxUser");}
  _ => {return format!("{}", val);}
}
}
pub fn QPaintEngine_TypeItemName_s(val: i32) ->String {
  //var nilthis *QPaintEngine
  //return nilthis.TypeItemName(val);
  return QPaintEngine_TypeItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
