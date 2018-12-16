

// mod ::gui::QPainterPath
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
// extern C begin: 38
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
#[derive(Default)] // class sizeof(QPainterPath)=8
pub struct QPainterPath {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QPainterPath_ITF interface {
//    QPainterPath_PTR() *QPainterPath
//}
//func (ptr *QPainterPath) QPainterPath_PTR() *QPainterPath { return ptr }

impl /*struct*/ QPainterPath {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QPainterPath {
    return QPainterPath{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QPainterPath {
//  type Target = QPainterPathBASE;
//
//  fn deref(&self) -> &QPainterPathBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QPainterPathBASE> for QPainterPath {
//  fn as_ref(& self) -> & QPainterPathBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qpainterpath.h:91
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QPainterPath()

/*
Constructs an empty QPainterPath object.
*/
// QPainterPath() ctx.fn_proto_cpp
impl /*struct*/ QPainterPath {
  pub fn QPainterPath_0<T: QPainterPath_QPainterPath_0>(value: T) -> QPainterPath {
    let rsthis = value.QPainterPath_0();
    return rsthis;
    // return 1;
  }
}

pub trait QPainterPath_QPainterPath_0 {
  fn QPainterPath_0(self) -> QPainterPath;
}
// QPainterPath() ctx.fn_proto_cpp
impl<'a> /*trait*/ QPainterPath_QPainterPath_0 for () {
  fn QPainterPath_0(self) -> QPainterPath {
    // unsafe{_ZN12QPainterPathC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN12QPainterPathC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QPainterPath{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainterpath.h:92
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QPainterPath(const QPointF &)

/*
Constructs an empty QPainterPath object.
*/
// QPainterPath(const QPointF &) ctx.fn_proto_cpp
impl /*struct*/ QPainterPath {
  pub fn QPainterPath_1<T: QPainterPath_QPainterPath_1>(value: T) -> QPainterPath {
    let rsthis = value.QPainterPath_1();
    return rsthis;
    // return 1;
  }
}

pub trait QPainterPath_QPainterPath_1 {
  fn QPainterPath_1(self) -> QPainterPath;
}
// QPainterPath(const QPointF &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QPainterPath_QPainterPath_1 for (usize) {
  fn QPainterPath_1(self) -> QPainterPath {
    // unsafe{_ZN12QPainterPathC2ERK7QPointF()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN12QPainterPathC2ERK7QPointF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QPainterPath{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainterpath.h:94
// index:0
// Public Visibility=Default Availability=Available
// [8] QPainterPath & operator=(const QPainterPath &)

/*

*/
impl /*struct*/ QPainterPath {
  pub fn operator_equal_0<RetType, T: QPainterPath_operator_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_0(self);
    // return 1;
  }
}
pub trait QPainterPath_operator_equal_0<RetType> {
  fn operator_equal_0(self , rsthis: & QPainterPath) -> RetType;
}
impl<'a> /*trait*/ QPainterPath_operator_equal_0<usize> for (usize) {
  fn operator_equal_0(self , rsthis: & QPainterPath) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN12QPainterPathaSERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpainterpath.h:96
// index:1
// Public inline Visibility=Default Availability=Available
// [8] QPainterPath & operator=(QPainterPath &&)

/*

*/
impl /*struct*/ QPainterPath {
  pub fn operator_equal_1<RetType, T: QPainterPath_operator_equal_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_1(self);
    // return 1;
  }
}
pub trait QPainterPath_operator_equal_1<RetType> {
  fn operator_equal_1(self , rsthis: & QPainterPath) -> RetType;
}
impl<'a> /*trait*/ QPainterPath_operator_equal_1<usize> for (usize) {
  fn operator_equal_1(self , rsthis: & QPainterPath) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN12QPainterPathaSEOS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpainterpath.h:99
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ~QPainterPath()

/*

*/
pub fn DeleteQPainterPath(this :*mut QPainterPath) {
    // let rv = qtrt::InvokeQtFunc6("_ZN12QPainterPathD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 8)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qpainterpath.h:100
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void swap(QPainterPath &)

/*
Swaps painter path other with this painter path. This operation is very fast and never fails.

This function was introduced in  Qt 4.8.
*/
impl /*struct*/ QPainterPath {
  pub fn swap_0<RetType, T: QPainterPath_swap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.swap_0(self);
    // return 1;
  }
}
pub trait QPainterPath_swap_0<RetType> {
  fn swap_0(self , rsthis: & QPainterPath) -> RetType;
}
impl<'a> /*trait*/ QPainterPath_swap_0<(/*void*/)> for (usize) {
  fn swap_0(self , rsthis: & QPainterPath) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QPainterPath4swapERS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainterpath.h:102
// index:0
// Public Visibility=Default Availability=Available
// [-2] void closeSubpath()

/*
Closes the current subpath by drawing a line to the beginning of the subpath, automatically starting a new path. The current point of the new path is (0, 0).

If the subpath does not contain any elements, this function does nothing.

See also moveTo() and Composing a QPainterPath.
*/
impl /*struct*/ QPainterPath {
  pub fn closeSubpath_0<RetType, T: QPainterPath_closeSubpath_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.closeSubpath_0(self);
    // return 1;
  }
}
pub trait QPainterPath_closeSubpath_0<RetType> {
  fn closeSubpath_0(self , rsthis: & QPainterPath) -> RetType;
}
impl<'a> /*trait*/ QPainterPath_closeSubpath_0<(/*void*/)> for () {
  fn closeSubpath_0(self , rsthis: & QPainterPath) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN12QPainterPath12closeSubpathEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainterpath.h:104
// index:0
// Public Visibility=Default Availability=Available
// [-2] void moveTo(const QPointF &)

/*
Moves the current point to the given point, implicitly starting a new subpath and closing the previous one.

See also closeSubpath() and Composing a QPainterPath.
*/
impl /*struct*/ QPainterPath {
  pub fn moveTo_0<RetType, T: QPainterPath_moveTo_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.moveTo_0(self);
    // return 1;
  }
}
pub trait QPainterPath_moveTo_0<RetType> {
  fn moveTo_0(self , rsthis: & QPainterPath) -> RetType;
}
impl<'a> /*trait*/ QPainterPath_moveTo_0<(/*void*/)> for (usize) {
  fn moveTo_0(self , rsthis: & QPainterPath) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QPainterPath6moveToERK7QPointF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainterpath.h:105
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void moveTo(qreal, qreal)

/*
Moves the current point to the given point, implicitly starting a new subpath and closing the previous one.

See also closeSubpath() and Composing a QPainterPath.
*/
impl /*struct*/ QPainterPath {
  pub fn moveTo_1<RetType, T: QPainterPath_moveTo_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.moveTo_1(self);
    // return 1;
  }
}
pub trait QPainterPath_moveTo_1<RetType> {
  fn moveTo_1(self , rsthis: & QPainterPath) -> RetType;
}
impl<'a> /*trait*/ QPainterPath_moveTo_1<(/*void*/)> for (f64,f64) {
  fn moveTo_1(self , rsthis: & QPainterPath) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN12QPainterPath6moveToEdd", 2,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainterpath.h:107
// index:0
// Public Visibility=Default Availability=Available
// [-2] void lineTo(const QPointF &)

/*
Adds a straight line from the current position to the given endPoint. After the line is drawn, the current position is updated to be at the end point of the line.

See also addPolygon(), addRect(), and Composing a QPainterPath.
*/
impl /*struct*/ QPainterPath {
  pub fn lineTo_0<RetType, T: QPainterPath_lineTo_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.lineTo_0(self);
    // return 1;
  }
}
pub trait QPainterPath_lineTo_0<RetType> {
  fn lineTo_0(self , rsthis: & QPainterPath) -> RetType;
}
impl<'a> /*trait*/ QPainterPath_lineTo_0<(/*void*/)> for (usize) {
  fn lineTo_0(self , rsthis: & QPainterPath) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QPainterPath6lineToERK7QPointF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainterpath.h:108
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void lineTo(qreal, qreal)

/*
Adds a straight line from the current position to the given endPoint. After the line is drawn, the current position is updated to be at the end point of the line.

See also addPolygon(), addRect(), and Composing a QPainterPath.
*/
impl /*struct*/ QPainterPath {
  pub fn lineTo_1<RetType, T: QPainterPath_lineTo_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.lineTo_1(self);
    // return 1;
  }
}
pub trait QPainterPath_lineTo_1<RetType> {
  fn lineTo_1(self , rsthis: & QPainterPath) -> RetType;
}
impl<'a> /*trait*/ QPainterPath_lineTo_1<(/*void*/)> for (f64,f64) {
  fn lineTo_1(self , rsthis: & QPainterPath) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN12QPainterPath6lineToEdd", 2,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainterpath.h:110
// index:0
// Public Visibility=Default Availability=Available
// [-2] void arcMoveTo(const QRectF &, qreal)

/*
Creates a move to that lies on the arc that occupies the given rectangle at angle.

Angles are specified in degrees. Clockwise arcs can be specified using negative angles.

This function was introduced in  Qt 4.2.

See also moveTo() and arcTo().
*/
impl /*struct*/ QPainterPath {
  pub fn arcMoveTo_0<RetType, T: QPainterPath_arcMoveTo_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.arcMoveTo_0(self);
    // return 1;
  }
}
pub trait QPainterPath_arcMoveTo_0<RetType> {
  fn arcMoveTo_0(self , rsthis: & QPainterPath) -> RetType;
}
impl<'a> /*trait*/ QPainterPath_arcMoveTo_0<(/*void*/)> for (usize,f64) {
  fn arcMoveTo_0(self , rsthis: & QPainterPath) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN12QPainterPath9arcMoveToERK6QRectFd", 2,qtrt::FFITY_POINTER,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainterpath.h:111
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void arcMoveTo(qreal, qreal, qreal, qreal, qreal)

/*
Creates a move to that lies on the arc that occupies the given rectangle at angle.

Angles are specified in degrees. Clockwise arcs can be specified using negative angles.

This function was introduced in  Qt 4.2.

See also moveTo() and arcTo().
*/
impl /*struct*/ QPainterPath {
  pub fn arcMoveTo_1<RetType, T: QPainterPath_arcMoveTo_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.arcMoveTo_1(self);
    // return 1;
  }
}
pub trait QPainterPath_arcMoveTo_1<RetType> {
  fn arcMoveTo_1(self , rsthis: & QPainterPath) -> RetType;
}
impl<'a> /*trait*/ QPainterPath_arcMoveTo_1<(/*void*/)> for (f64,f64,f64,f64,f64) {
  fn arcMoveTo_1(self , rsthis: & QPainterPath) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let arg2 = (&self.2) as *const f64 as usize;
    let arg3 = (&self.3) as *const f64 as usize;
    let arg4 = (&self.4) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN12QPainterPath9arcMoveToEddddd", 5,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainterpath.h:113
// index:0
// Public Visibility=Default Availability=Available
// [-2] void arcTo(const QRectF &, qreal, qreal)

/*
Creates an arc that occupies the given rectangle, beginning at the specified startAngle and extending sweepLength degrees counter-clockwise.

Angles are specified in degrees. Clockwise arcs can be specified using negative angles.

Note that this function connects the starting point of the arc to the current position if they are not already connected. After the arc has been added, the current position is the last point in arc. To draw a line back to the first point, use the closeSubpath() function.


 
  QLinearGradient myGradient;
  QPen myPen;

  QPointF center, startPoint;

  QPainterPath myPath;
  myPath.moveTo(center);
  myPath.arcTo(boundingRect, startAngle,
               sweepLength);

  QPainter painter(this);
  painter.setBrush(myGradient);
  painter.setPen(myPen);
  painter.drawPath(myPath);





See also arcMoveTo(), addEllipse(), QPainter::drawArc(), QPainter::drawPie(), and Composing a QPainterPath.
*/
impl /*struct*/ QPainterPath {
  pub fn arcTo_0<RetType, T: QPainterPath_arcTo_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.arcTo_0(self);
    // return 1;
  }
}
pub trait QPainterPath_arcTo_0<RetType> {
  fn arcTo_0(self , rsthis: & QPainterPath) -> RetType;
}
impl<'a> /*trait*/ QPainterPath_arcTo_0<(/*void*/)> for (usize,f64,f64) {
  fn arcTo_0(self , rsthis: & QPainterPath) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let arg2 = (&self.2) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN12QPainterPath5arcToERK6QRectFdd", 3,qtrt::FFITY_POINTER,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainterpath.h:114
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void arcTo(qreal, qreal, qreal, qreal, qreal, qreal)

/*
Creates an arc that occupies the given rectangle, beginning at the specified startAngle and extending sweepLength degrees counter-clockwise.

Angles are specified in degrees. Clockwise arcs can be specified using negative angles.

Note that this function connects the starting point of the arc to the current position if they are not already connected. After the arc has been added, the current position is the last point in arc. To draw a line back to the first point, use the closeSubpath() function.


 
  QLinearGradient myGradient;
  QPen myPen;

  QPointF center, startPoint;

  QPainterPath myPath;
  myPath.moveTo(center);
  myPath.arcTo(boundingRect, startAngle,
               sweepLength);

  QPainter painter(this);
  painter.setBrush(myGradient);
  painter.setPen(myPen);
  painter.drawPath(myPath);





See also arcMoveTo(), addEllipse(), QPainter::drawArc(), QPainter::drawPie(), and Composing a QPainterPath.
*/
impl /*struct*/ QPainterPath {
  pub fn arcTo_1<RetType, T: QPainterPath_arcTo_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.arcTo_1(self);
    // return 1;
  }
}
pub trait QPainterPath_arcTo_1<RetType> {
  fn arcTo_1(self , rsthis: & QPainterPath) -> RetType;
}
impl<'a> /*trait*/ QPainterPath_arcTo_1<(/*void*/)> for (f64,f64,f64,f64,f64,f64) {
  fn arcTo_1(self , rsthis: & QPainterPath) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let arg2 = (&self.2) as *const f64 as usize;
    let arg3 = (&self.3) as *const f64 as usize;
    let arg4 = (&self.4) as *const f64 as usize;
    let arg5 = (&self.5) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN12QPainterPath5arcToEdddddd", 6,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,arg5,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainterpath.h:116
// index:0
// Public Visibility=Default Availability=Available
// [-2] void cubicTo(const QPointF &, const QPointF &, const QPointF &)

/*
Adds a cubic Bezier curve between the current position and the given endPoint using the control points specified by c1, and c2.

After the curve is added, the current position is updated to be at the end point of the curve.


 
  QLinearGradient myGradient;
  QPen myPen;

  QPainterPath myPath;
  myPath.cubicTo(c1, c2, endPoint);

  QPainter painter(this);
  painter.setBrush(myGradient);
  painter.setPen(myPen);
  painter.drawPath(myPath);





See also quadTo() and Composing a QPainterPath.
*/
impl /*struct*/ QPainterPath {
  pub fn cubicTo_0<RetType, T: QPainterPath_cubicTo_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cubicTo_0(self);
    // return 1;
  }
}
pub trait QPainterPath_cubicTo_0<RetType> {
  fn cubicTo_0(self , rsthis: & QPainterPath) -> RetType;
}
impl<'a> /*trait*/ QPainterPath_cubicTo_0<(/*void*/)> for (usize,usize,usize) {
  fn cubicTo_0(self , rsthis: & QPainterPath) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QPainterPath7cubicToERK7QPointFS2_S2_", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainterpath.h:117
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void cubicTo(qreal, qreal, qreal, qreal, qreal, qreal)

/*
Adds a cubic Bezier curve between the current position and the given endPoint using the control points specified by c1, and c2.

After the curve is added, the current position is updated to be at the end point of the curve.


 
  QLinearGradient myGradient;
  QPen myPen;

  QPainterPath myPath;
  myPath.cubicTo(c1, c2, endPoint);

  QPainter painter(this);
  painter.setBrush(myGradient);
  painter.setPen(myPen);
  painter.drawPath(myPath);





See also quadTo() and Composing a QPainterPath.
*/
impl /*struct*/ QPainterPath {
  pub fn cubicTo_1<RetType, T: QPainterPath_cubicTo_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cubicTo_1(self);
    // return 1;
  }
}
pub trait QPainterPath_cubicTo_1<RetType> {
  fn cubicTo_1(self , rsthis: & QPainterPath) -> RetType;
}
impl<'a> /*trait*/ QPainterPath_cubicTo_1<(/*void*/)> for (f64,f64,f64,f64,f64,f64) {
  fn cubicTo_1(self , rsthis: & QPainterPath) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let arg2 = (&self.2) as *const f64 as usize;
    let arg3 = (&self.3) as *const f64 as usize;
    let arg4 = (&self.4) as *const f64 as usize;
    let arg5 = (&self.5) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN12QPainterPath7cubicToEdddddd", 6,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,arg5,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainterpath.h:119
// index:0
// Public Visibility=Default Availability=Available
// [-2] void quadTo(const QPointF &, const QPointF &)

/*
Adds a quadratic Bezier curve between the current position and the given endPoint with the control point specified by c.

After the curve is added, the current point is updated to be at the end point of the curve.

See also cubicTo() and Composing a QPainterPath.
*/
impl /*struct*/ QPainterPath {
  pub fn quadTo_0<RetType, T: QPainterPath_quadTo_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.quadTo_0(self);
    // return 1;
  }
}
pub trait QPainterPath_quadTo_0<RetType> {
  fn quadTo_0(self , rsthis: & QPainterPath) -> RetType;
}
impl<'a> /*trait*/ QPainterPath_quadTo_0<(/*void*/)> for (usize,usize) {
  fn quadTo_0(self , rsthis: & QPainterPath) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QPainterPath6quadToERK7QPointFS2_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainterpath.h:120
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void quadTo(qreal, qreal, qreal, qreal)

/*
Adds a quadratic Bezier curve between the current position and the given endPoint with the control point specified by c.

After the curve is added, the current point is updated to be at the end point of the curve.

See also cubicTo() and Composing a QPainterPath.
*/
impl /*struct*/ QPainterPath {
  pub fn quadTo_1<RetType, T: QPainterPath_quadTo_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.quadTo_1(self);
    // return 1;
  }
}
pub trait QPainterPath_quadTo_1<RetType> {
  fn quadTo_1(self , rsthis: & QPainterPath) -> RetType;
}
impl<'a> /*trait*/ QPainterPath_quadTo_1<(/*void*/)> for (f64,f64,f64,f64) {
  fn quadTo_1(self , rsthis: & QPainterPath) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let arg2 = (&self.2) as *const f64 as usize;
    let arg3 = (&self.3) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN12QPainterPath6quadToEdddd", 4,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainterpath.h:122
// index:0
// Public Visibility=Default Availability=Available
// [16] QPointF currentPosition() const

/*
Returns the current position of the path.
*/
impl /*struct*/ QPainterPath {
  pub fn currentPosition_0<RetType, T: QPainterPath_currentPosition_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.currentPosition_0(self);
    // return 1;
  }
}
pub trait QPainterPath_currentPosition_0<RetType> {
  fn currentPosition_0(self , rsthis: & QPainterPath) -> RetType;
}
impl<'a> /*trait*/ QPainterPath_currentPosition_0<usize> for () {
  fn currentPosition_0(self , rsthis: & QPainterPath) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QPainterPath15currentPositionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpainterpath.h:124
// index:0
// Public Visibility=Default Availability=Available
// [-2] void addRect(const QRectF &)

/*
Adds the given rectangle to this path as a closed subpath.

The rectangle is added as a clockwise set of lines. The painter path's current position after the rectangle has been added is at the top-left corner of the rectangle.


 
  QLinearGradient myGradient;
  QPen myPen;
  QRectF myRectangle;

  QPainterPath myPath;
  myPath.addRect(myRectangle);

  QPainter painter(this);
  painter.setBrush(myGradient);
  painter.setPen(myPen);
  painter.drawPath(myPath);





See also addRegion(), lineTo(), and Composing a QPainterPath.
*/
impl /*struct*/ QPainterPath {
  pub fn addRect_0<RetType, T: QPainterPath_addRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addRect_0(self);
    // return 1;
  }
}
pub trait QPainterPath_addRect_0<RetType> {
  fn addRect_0(self , rsthis: & QPainterPath) -> RetType;
}
impl<'a> /*trait*/ QPainterPath_addRect_0<(/*void*/)> for (usize) {
  fn addRect_0(self , rsthis: & QPainterPath) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QPainterPath7addRectERK6QRectF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainterpath.h:125
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void addRect(qreal, qreal, qreal, qreal)

/*
Adds the given rectangle to this path as a closed subpath.

The rectangle is added as a clockwise set of lines. The painter path's current position after the rectangle has been added is at the top-left corner of the rectangle.


 
  QLinearGradient myGradient;
  QPen myPen;
  QRectF myRectangle;

  QPainterPath myPath;
  myPath.addRect(myRectangle);

  QPainter painter(this);
  painter.setBrush(myGradient);
  painter.setPen(myPen);
  painter.drawPath(myPath);





See also addRegion(), lineTo(), and Composing a QPainterPath.
*/
impl /*struct*/ QPainterPath {
  pub fn addRect_1<RetType, T: QPainterPath_addRect_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addRect_1(self);
    // return 1;
  }
}
pub trait QPainterPath_addRect_1<RetType> {
  fn addRect_1(self , rsthis: & QPainterPath) -> RetType;
}
impl<'a> /*trait*/ QPainterPath_addRect_1<(/*void*/)> for (f64,f64,f64,f64) {
  fn addRect_1(self , rsthis: & QPainterPath) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let arg2 = (&self.2) as *const f64 as usize;
    let arg3 = (&self.3) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN12QPainterPath7addRectEdddd", 4,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainterpath.h:126
// index:0
// Public Visibility=Default Availability=Available
// [-2] void addEllipse(const QRectF &)

/*
Creates an ellipse within the specified boundingRectangle and adds it to the painter path as a closed subpath.

The ellipse is composed of a clockwise curve, starting and finishing at zero degrees (the 3 o'clock position).


 
  QLinearGradient myGradient;
  QPen myPen;
  QRectF boundingRectangle;

  QPainterPath myPath;
  myPath.addEllipse(boundingRectangle);

  QPainter painter(this);
  painter.setBrush(myGradient);
  painter.setPen(myPen);
  painter.drawPath(myPath);





See also arcTo(), QPainter::drawEllipse(), and Composing a QPainterPath.
*/
impl /*struct*/ QPainterPath {
  pub fn addEllipse_0<RetType, T: QPainterPath_addEllipse_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addEllipse_0(self);
    // return 1;
  }
}
pub trait QPainterPath_addEllipse_0<RetType> {
  fn addEllipse_0(self , rsthis: & QPainterPath) -> RetType;
}
impl<'a> /*trait*/ QPainterPath_addEllipse_0<(/*void*/)> for (usize) {
  fn addEllipse_0(self , rsthis: & QPainterPath) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QPainterPath10addEllipseERK6QRectF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainterpath.h:127
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void addEllipse(qreal, qreal, qreal, qreal)

/*
Creates an ellipse within the specified boundingRectangle and adds it to the painter path as a closed subpath.

The ellipse is composed of a clockwise curve, starting and finishing at zero degrees (the 3 o'clock position).


 
  QLinearGradient myGradient;
  QPen myPen;
  QRectF boundingRectangle;

  QPainterPath myPath;
  myPath.addEllipse(boundingRectangle);

  QPainter painter(this);
  painter.setBrush(myGradient);
  painter.setPen(myPen);
  painter.drawPath(myPath);





See also arcTo(), QPainter::drawEllipse(), and Composing a QPainterPath.
*/
impl /*struct*/ QPainterPath {
  pub fn addEllipse_1<RetType, T: QPainterPath_addEllipse_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addEllipse_1(self);
    // return 1;
  }
}
pub trait QPainterPath_addEllipse_1<RetType> {
  fn addEllipse_1(self , rsthis: & QPainterPath) -> RetType;
}
impl<'a> /*trait*/ QPainterPath_addEllipse_1<(/*void*/)> for (f64,f64,f64,f64) {
  fn addEllipse_1(self , rsthis: & QPainterPath) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let arg2 = (&self.2) as *const f64 as usize;
    let arg3 = (&self.3) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN12QPainterPath10addEllipseEdddd", 4,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainterpath.h:128
// index:2
// Public inline Visibility=Default Availability=Available
// [-2] void addEllipse(const QPointF &, qreal, qreal)

/*
Creates an ellipse within the specified boundingRectangle and adds it to the painter path as a closed subpath.

The ellipse is composed of a clockwise curve, starting and finishing at zero degrees (the 3 o'clock position).


 
  QLinearGradient myGradient;
  QPen myPen;
  QRectF boundingRectangle;

  QPainterPath myPath;
  myPath.addEllipse(boundingRectangle);

  QPainter painter(this);
  painter.setBrush(myGradient);
  painter.setPen(myPen);
  painter.drawPath(myPath);





See also arcTo(), QPainter::drawEllipse(), and Composing a QPainterPath.
*/
impl /*struct*/ QPainterPath {
  pub fn addEllipse_2<RetType, T: QPainterPath_addEllipse_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addEllipse_2(self);
    // return 1;
  }
}
pub trait QPainterPath_addEllipse_2<RetType> {
  fn addEllipse_2(self , rsthis: & QPainterPath) -> RetType;
}
impl<'a> /*trait*/ QPainterPath_addEllipse_2<(/*void*/)> for (usize,f64,f64) {
  fn addEllipse_2(self , rsthis: & QPainterPath) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let arg2 = (&self.2) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN12QPainterPath10addEllipseERK7QPointFdd", 3,qtrt::FFITY_POINTER,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainterpath.h:129
// index:0
// Public Visibility=Default Availability=Available
// [-2] void addPolygon(const QPolygonF &)

/*
Adds the given polygon to the path as an (unclosed) subpath.

Note that the current position after the polygon has been added, is the last point in polygon. To draw a line back to the first point, use the closeSubpath() function.


 
  QLinearGradient myGradient;
  QPen myPen;
  QPolygonF myPolygon;

  QPainterPath myPath;
  myPath.addPolygon(myPolygon);

  QPainter painter(this);
  painter.setBrush(myGradient);
  painter.setPen(myPen);
  painter.drawPath(myPath);





See also lineTo() and Composing a QPainterPath.
*/
impl /*struct*/ QPainterPath {
  pub fn addPolygon_0<RetType, T: QPainterPath_addPolygon_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addPolygon_0(self);
    // return 1;
  }
}
pub trait QPainterPath_addPolygon_0<RetType> {
  fn addPolygon_0(self , rsthis: & QPainterPath) -> RetType;
}
impl<'a> /*trait*/ QPainterPath_addPolygon_0<(/*void*/)> for (usize) {
  fn addPolygon_0(self , rsthis: & QPainterPath) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QPainterPath10addPolygonERK9QPolygonF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainterpath.h:130
// index:0
// Public Visibility=Default Availability=Available
// [-2] void addText(const QPointF &, const QFont &, const QString &)

/*
Adds the given text to this path as a set of closed subpaths created from the font supplied. The subpaths are positioned so that the left end of the text's baseline lies at the specified point.


 
  QLinearGradient myGradient;
  QPen myPen;
  QFont myFont;
  QPointF baseline(x, y);

  QPainterPath myPath;
  myPath.addText(baseline, myFont, tr("Qt"));

  QPainter painter(this);
  painter.setBrush(myGradient);
  painter.setPen(myPen);
  painter.drawPath(myPath);





See also QPainter::drawText() and Composing a QPainterPath.
*/
impl /*struct*/ QPainterPath {
  pub fn addText_0<RetType, T: QPainterPath_addText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addText_0(self);
    // return 1;
  }
}
pub trait QPainterPath_addText_0<RetType> {
  fn addText_0(self , rsthis: & QPainterPath) -> RetType;
}
impl<'a> /*trait*/ QPainterPath_addText_0<(/*void*/)> for (usize,usize,usize) {
  fn addText_0(self , rsthis: & QPainterPath) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QPainterPath7addTextERK7QPointFRK5QFontRK7QString", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainterpath.h:131
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void addText(qreal, qreal, const QFont &, const QString &)

/*
Adds the given text to this path as a set of closed subpaths created from the font supplied. The subpaths are positioned so that the left end of the text's baseline lies at the specified point.


 
  QLinearGradient myGradient;
  QPen myPen;
  QFont myFont;
  QPointF baseline(x, y);

  QPainterPath myPath;
  myPath.addText(baseline, myFont, tr("Qt"));

  QPainter painter(this);
  painter.setBrush(myGradient);
  painter.setPen(myPen);
  painter.drawPath(myPath);





See also QPainter::drawText() and Composing a QPainterPath.
*/
impl /*struct*/ QPainterPath {
  pub fn addText_1<RetType, T: QPainterPath_addText_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addText_1(self);
    // return 1;
  }
}
pub trait QPainterPath_addText_1<RetType> {
  fn addText_1(self , rsthis: & QPainterPath) -> RetType;
}
impl<'a> /*trait*/ QPainterPath_addText_1<(/*void*/)> for (f64,f64,usize,usize) {
  fn addText_1(self , rsthis: & QPainterPath) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QPainterPath7addTextEddRK5QFontRK7QString", 4,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainterpath.h:132
// index:0
// Public Visibility=Default Availability=Available
// [-2] void addPath(const QPainterPath &)

/*
Adds the given path to this path as a closed subpath.

See also connectPath() and Composing a QPainterPath.
*/
impl /*struct*/ QPainterPath {
  pub fn addPath_0<RetType, T: QPainterPath_addPath_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addPath_0(self);
    // return 1;
  }
}
pub trait QPainterPath_addPath_0<RetType> {
  fn addPath_0(self , rsthis: & QPainterPath) -> RetType;
}
impl<'a> /*trait*/ QPainterPath_addPath_0<(/*void*/)> for (usize) {
  fn addPath_0(self , rsthis: & QPainterPath) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QPainterPath7addPathERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainterpath.h:133
// index:0
// Public Visibility=Default Availability=Available
// [-2] void addRegion(const QRegion &)

/*
Adds the given region to the path by adding each rectangle in the region as a separate closed subpath.

See also addRect() and Composing a QPainterPath.
*/
impl /*struct*/ QPainterPath {
  pub fn addRegion_0<RetType, T: QPainterPath_addRegion_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addRegion_0(self);
    // return 1;
  }
}
pub trait QPainterPath_addRegion_0<RetType> {
  fn addRegion_0(self , rsthis: & QPainterPath) -> RetType;
}
impl<'a> /*trait*/ QPainterPath_addRegion_0<(/*void*/)> for (usize) {
  fn addRegion_0(self , rsthis: & QPainterPath) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QPainterPath9addRegionERK7QRegion", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainterpath.h:135
// index:0
// Public Visibility=Default Availability=Available
// [-2] void addRoundedRect(const QRectF &, qreal, qreal, Qt::SizeMode)

/*
Adds the given rectangle rect with rounded corners to the path.

The xRadius and yRadius arguments specify the radii of the ellipses defining the corners of the rounded rectangle. When mode is Qt::RelativeSize, xRadius and yRadius are specified in percentage of half the rectangle's width and height respectively, and should be in the range 0.0 to 100.0.

This function was introduced in  Qt 4.4.

See also addRect().
*/
impl /*struct*/ QPainterPath {
  pub fn addRoundedRect_0<RetType, T: QPainterPath_addRoundedRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addRoundedRect_0(self);
    // return 1;
  }
}
pub trait QPainterPath_addRoundedRect_0<RetType> {
  fn addRoundedRect_0(self , rsthis: & QPainterPath) -> RetType;
}
impl<'a> /*trait*/ QPainterPath_addRoundedRect_0<(/*void*/)> for (usize,f64,f64,i32) {
  fn addRoundedRect_0(self , rsthis: & QPainterPath) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let arg2 = (&self.2) as *const f64 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN12QPainterPath14addRoundedRectERK6QRectFddN2Qt8SizeModeE", 4,qtrt::FFITY_POINTER,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainterpath.h:137
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void addRoundedRect(qreal, qreal, qreal, qreal, qreal, qreal, Qt::SizeMode)

/*
Adds the given rectangle rect with rounded corners to the path.

The xRadius and yRadius arguments specify the radii of the ellipses defining the corners of the rounded rectangle. When mode is Qt::RelativeSize, xRadius and yRadius are specified in percentage of half the rectangle's width and height respectively, and should be in the range 0.0 to 100.0.

This function was introduced in  Qt 4.4.

See also addRect().
*/
impl /*struct*/ QPainterPath {
  pub fn addRoundedRect_1<RetType, T: QPainterPath_addRoundedRect_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addRoundedRect_1(self);
    // return 1;
  }
}
pub trait QPainterPath_addRoundedRect_1<RetType> {
  fn addRoundedRect_1(self , rsthis: & QPainterPath) -> RetType;
}
impl<'a> /*trait*/ QPainterPath_addRoundedRect_1<(/*void*/)> for (f64,f64,f64,f64,f64,f64,i32) {
  fn addRoundedRect_1(self , rsthis: & QPainterPath) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let arg2 = (&self.2) as *const f64 as usize;
    let arg3 = (&self.3) as *const f64 as usize;
    let arg4 = (&self.4) as *const f64 as usize;
    let arg5 = (&self.5) as *const f64 as usize;
    let arg6 = (&self.6) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN12QPainterPath14addRoundedRectEddddddN2Qt8SizeModeE", 7,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,arg5,arg6,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainterpath.h:141
// index:0
// Public Visibility=Default Availability=Available
// [-2] void addRoundRect(const QRectF &, int, int)

/*

*/
impl /*struct*/ QPainterPath {
  pub fn addRoundRect_0<RetType, T: QPainterPath_addRoundRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addRoundRect_0(self);
    // return 1;
  }
}
pub trait QPainterPath_addRoundRect_0<RetType> {
  fn addRoundRect_0(self , rsthis: & QPainterPath) -> RetType;
}
impl<'a> /*trait*/ QPainterPath_addRoundRect_0<(/*void*/)> for (usize,i32,i32) {
  fn addRoundRect_0(self , rsthis: & QPainterPath) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN12QPainterPath12addRoundRectERK6QRectFii", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainterpath.h:142
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void addRoundRect(qreal, qreal, qreal, qreal, int, int)

/*

*/
impl /*struct*/ QPainterPath {
  pub fn addRoundRect_1<RetType, T: QPainterPath_addRoundRect_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addRoundRect_1(self);
    // return 1;
  }
}
pub trait QPainterPath_addRoundRect_1<RetType> {
  fn addRoundRect_1(self , rsthis: & QPainterPath) -> RetType;
}
impl<'a> /*trait*/ QPainterPath_addRoundRect_1<(/*void*/)> for (f64,f64,f64,f64,i32,i32) {
  fn addRoundRect_1(self , rsthis: & QPainterPath) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let arg2 = (&self.2) as *const f64 as usize;
    let arg3 = (&self.3) as *const f64 as usize;
    let arg4 = (&self.4) as *const i32 as usize;
    let arg5 = (&self.5) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN12QPainterPath12addRoundRectEddddii", 6,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,arg5,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainterpath.h:144
// index:2
// Public inline Visibility=Default Availability=Available
// [-2] void addRoundRect(const QRectF &, int)

/*

*/
impl /*struct*/ QPainterPath {
  pub fn addRoundRect_2<RetType, T: QPainterPath_addRoundRect_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addRoundRect_2(self);
    // return 1;
  }
}
pub trait QPainterPath_addRoundRect_2<RetType> {
  fn addRoundRect_2(self , rsthis: & QPainterPath) -> RetType;
}
impl<'a> /*trait*/ QPainterPath_addRoundRect_2<(/*void*/)> for (usize,i32) {
  fn addRoundRect_2(self , rsthis: & QPainterPath) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN12QPainterPath12addRoundRectERK6QRectFi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainterpath.h:145
// index:3
// Public inline Visibility=Default Availability=Available
// [-2] void addRoundRect(qreal, qreal, qreal, qreal, int)

/*

*/
impl /*struct*/ QPainterPath {
  pub fn addRoundRect_3<RetType, T: QPainterPath_addRoundRect_3<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addRoundRect_3(self);
    // return 1;
  }
}
pub trait QPainterPath_addRoundRect_3<RetType> {
  fn addRoundRect_3(self , rsthis: & QPainterPath) -> RetType;
}
impl<'a> /*trait*/ QPainterPath_addRoundRect_3<(/*void*/)> for (f64,f64,f64,f64,i32) {
  fn addRoundRect_3(self , rsthis: & QPainterPath) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let arg2 = (&self.2) as *const f64 as usize;
    let arg3 = (&self.3) as *const f64 as usize;
    let arg4 = (&self.4) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN12QPainterPath12addRoundRectEddddi", 5,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainterpath.h:148
// index:0
// Public Visibility=Default Availability=Available
// [-2] void connectPath(const QPainterPath &)

/*
Connects the given path to this path by adding a line from the last element of this path to the first element of the given path.

See also addPath() and Composing a QPainterPath.
*/
impl /*struct*/ QPainterPath {
  pub fn connectPath_0<RetType, T: QPainterPath_connectPath_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.connectPath_0(self);
    // return 1;
  }
}
pub trait QPainterPath_connectPath_0<RetType> {
  fn connectPath_0(self , rsthis: & QPainterPath) -> RetType;
}
impl<'a> /*trait*/ QPainterPath_connectPath_0<(/*void*/)> for (usize) {
  fn connectPath_0(self , rsthis: & QPainterPath) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QPainterPath11connectPathERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainterpath.h:150
// index:0
// Public Visibility=Default Availability=Available
// [1] bool contains(const QPointF &) const

/*
Returns true if the given point is inside the path, otherwise returns false.

See also intersects().
*/
impl /*struct*/ QPainterPath {
  pub fn contains_0<RetType, T: QPainterPath_contains_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.contains_0(self);
    // return 1;
  }
}
pub trait QPainterPath_contains_0<RetType> {
  fn contains_0(self , rsthis: & QPainterPath) -> RetType;
}
impl<'a> /*trait*/ QPainterPath_contains_0<bool> for (usize) {
  fn contains_0(self , rsthis: & QPainterPath) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QPainterPath8containsERK7QPointF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpainterpath.h:151
// index:1
// Public Visibility=Default Availability=Available
// [1] bool contains(const QRectF &) const

/*
Returns true if the given point is inside the path, otherwise returns false.

See also intersects().
*/
impl /*struct*/ QPainterPath {
  pub fn contains_1<RetType, T: QPainterPath_contains_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.contains_1(self);
    // return 1;
  }
}
pub trait QPainterPath_contains_1<RetType> {
  fn contains_1(self , rsthis: & QPainterPath) -> RetType;
}
impl<'a> /*trait*/ QPainterPath_contains_1<bool> for (usize) {
  fn contains_1(self , rsthis: & QPainterPath) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QPainterPath8containsERK6QRectF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpainterpath.h:187
// index:2
// Public Visibility=Default Availability=Available
// [1] bool contains(const QPainterPath &) const

/*
Returns true if the given point is inside the path, otherwise returns false.

See also intersects().
*/
impl /*struct*/ QPainterPath {
  pub fn contains_2<RetType, T: QPainterPath_contains_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.contains_2(self);
    // return 1;
  }
}
pub trait QPainterPath_contains_2<RetType> {
  fn contains_2(self , rsthis: & QPainterPath) -> RetType;
}
impl<'a> /*trait*/ QPainterPath_contains_2<bool> for (usize) {
  fn contains_2(self , rsthis: & QPainterPath) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QPainterPath8containsERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpainterpath.h:152
// index:0
// Public Visibility=Default Availability=Available
// [1] bool intersects(const QRectF &) const

/*
Returns true if any point in the given rectangle intersects the path; otherwise returns false.

There is an intersection if any of the lines making up the rectangle crosses a part of the path or if any part of the rectangle overlaps with any area enclosed by the path. This function respects the current fillRule to determine what is considered inside the path.

See also contains().
*/
impl /*struct*/ QPainterPath {
  pub fn intersects_0<RetType, T: QPainterPath_intersects_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.intersects_0(self);
    // return 1;
  }
}
pub trait QPainterPath_intersects_0<RetType> {
  fn intersects_0(self , rsthis: & QPainterPath) -> RetType;
}
impl<'a> /*trait*/ QPainterPath_intersects_0<bool> for (usize) {
  fn intersects_0(self , rsthis: & QPainterPath) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QPainterPath10intersectsERK6QRectF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpainterpath.h:186
// index:1
// Public Visibility=Default Availability=Available
// [1] bool intersects(const QPainterPath &) const

/*
Returns true if any point in the given rectangle intersects the path; otherwise returns false.

There is an intersection if any of the lines making up the rectangle crosses a part of the path or if any part of the rectangle overlaps with any area enclosed by the path. This function respects the current fillRule to determine what is considered inside the path.

See also contains().
*/
impl /*struct*/ QPainterPath {
  pub fn intersects_1<RetType, T: QPainterPath_intersects_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.intersects_1(self);
    // return 1;
  }
}
pub trait QPainterPath_intersects_1<RetType> {
  fn intersects_1(self , rsthis: & QPainterPath) -> RetType;
}
impl<'a> /*trait*/ QPainterPath_intersects_1<bool> for (usize) {
  fn intersects_1(self , rsthis: & QPainterPath) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QPainterPath10intersectsERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpainterpath.h:154
// index:0
// Public Visibility=Default Availability=Available
// [-2] void translate(qreal, qreal)

/*
Translates all elements in the path by (dx, dy).

This function was introduced in  Qt 4.6.

See also translated().
*/
impl /*struct*/ QPainterPath {
  pub fn translate_0<RetType, T: QPainterPath_translate_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.translate_0(self);
    // return 1;
  }
}
pub trait QPainterPath_translate_0<RetType> {
  fn translate_0(self , rsthis: & QPainterPath) -> RetType;
}
impl<'a> /*trait*/ QPainterPath_translate_0<(/*void*/)> for (f64,f64) {
  fn translate_0(self , rsthis: & QPainterPath) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN12QPainterPath9translateEdd", 2,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainterpath.h:155
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void translate(const QPointF &)

/*
Translates all elements in the path by (dx, dy).

This function was introduced in  Qt 4.6.

See also translated().
*/
impl /*struct*/ QPainterPath {
  pub fn translate_1<RetType, T: QPainterPath_translate_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.translate_1(self);
    // return 1;
  }
}
pub trait QPainterPath_translate_1<RetType> {
  fn translate_1(self , rsthis: & QPainterPath) -> RetType;
}
impl<'a> /*trait*/ QPainterPath_translate_1<(/*void*/)> for (usize) {
  fn translate_1(self , rsthis: & QPainterPath) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QPainterPath9translateERK7QPointF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainterpath.h:157
// index:0
// Public Visibility=Default Availability=Available
// [8] QPainterPath translated(qreal, qreal) const

/*
Returns a copy of the path that is translated by (dx, dy).

This function was introduced in  Qt 4.6.

See also translate().
*/
impl /*struct*/ QPainterPath {
  pub fn translated_0<RetType, T: QPainterPath_translated_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.translated_0(self);
    // return 1;
  }
}
pub trait QPainterPath_translated_0<RetType> {
  fn translated_0(self , rsthis: & QPainterPath) -> RetType;
}
impl<'a> /*trait*/ QPainterPath_translated_0<usize> for (f64,f64) {
  fn translated_0(self , rsthis: & QPainterPath) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QPainterPath10translatedEdd", 2,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpainterpath.h:158
// index:1
// Public inline Visibility=Default Availability=Available
// [8] QPainterPath translated(const QPointF &) const

/*
Returns a copy of the path that is translated by (dx, dy).

This function was introduced in  Qt 4.6.

See also translate().
*/
impl /*struct*/ QPainterPath {
  pub fn translated_1<RetType, T: QPainterPath_translated_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.translated_1(self);
    // return 1;
  }
}
pub trait QPainterPath_translated_1<RetType> {
  fn translated_1(self , rsthis: & QPainterPath) -> RetType;
}
impl<'a> /*trait*/ QPainterPath_translated_1<usize> for (usize) {
  fn translated_1(self , rsthis: & QPainterPath) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QPainterPath10translatedERK7QPointF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpainterpath.h:160
// index:0
// Public Visibility=Default Availability=Available
// [32] QRectF boundingRect() const

/*
Returns the bounding rectangle of this painter path as a rectangle with floating point precision.

See also controlPointRect().
*/
impl /*struct*/ QPainterPath {
  pub fn boundingRect_0<RetType, T: QPainterPath_boundingRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.boundingRect_0(self);
    // return 1;
  }
}
pub trait QPainterPath_boundingRect_0<RetType> {
  fn boundingRect_0(self , rsthis: & QPainterPath) -> RetType;
}
impl<'a> /*trait*/ QPainterPath_boundingRect_0<usize> for () {
  fn boundingRect_0(self , rsthis: & QPainterPath) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QPainterPath12boundingRectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpainterpath.h:161
// index:0
// Public Visibility=Default Availability=Available
// [32] QRectF controlPointRect() const

/*
Returns the rectangle containing all the points and control points in this path.

This function is significantly faster to compute than the exact boundingRect(), and the returned rectangle is always a superset of the rectangle returned by boundingRect().

See also boundingRect().
*/
impl /*struct*/ QPainterPath {
  pub fn controlPointRect_0<RetType, T: QPainterPath_controlPointRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.controlPointRect_0(self);
    // return 1;
  }
}
pub trait QPainterPath_controlPointRect_0<RetType> {
  fn controlPointRect_0(self , rsthis: & QPainterPath) -> RetType;
}
impl<'a> /*trait*/ QPainterPath_controlPointRect_0<usize> for () {
  fn controlPointRect_0(self , rsthis: & QPainterPath) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QPainterPath16controlPointRectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpainterpath.h:163
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::FillRule fillRule() const

/*
Returns the painter path's currently set fill rule.

See also setFillRule().
*/
impl /*struct*/ QPainterPath {
  pub fn fillRule_0<RetType, T: QPainterPath_fillRule_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fillRule_0(self);
    // return 1;
  }
}
pub trait QPainterPath_fillRule_0<RetType> {
  fn fillRule_0(self , rsthis: & QPainterPath) -> RetType;
}
impl<'a> /*trait*/ QPainterPath_fillRule_0<i32> for () {
  fn fillRule_0(self , rsthis: & QPainterPath) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QPainterPath8fillRuleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpainterpath.h:164
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFillRule(Qt::FillRule)

/*
Sets the fill rule of the painter path to the given fillRule. Qt provides two methods for filling paths:


 Qt::OddEvenFill (default)Qt::WindingFill



See also fillRule().
*/
impl /*struct*/ QPainterPath {
  pub fn setFillRule_0<RetType, T: QPainterPath_setFillRule_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFillRule_0(self);
    // return 1;
  }
}
pub trait QPainterPath_setFillRule_0<RetType> {
  fn setFillRule_0(self , rsthis: & QPainterPath) -> RetType;
}
impl<'a> /*trait*/ QPainterPath_setFillRule_0<(/*void*/)> for (i32) {
  fn setFillRule_0(self , rsthis: & QPainterPath) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN12QPainterPath11setFillRuleEN2Qt8FillRuleE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainterpath.h:166
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isEmpty() const

/*
Returns true if either there are no elements in this path, or if the only element is a MoveToElement; otherwise returns false.

See also elementCount().
*/
impl /*struct*/ QPainterPath {
  pub fn isEmpty_0<RetType, T: QPainterPath_isEmpty_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isEmpty_0(self);
    // return 1;
  }
}
pub trait QPainterPath_isEmpty_0<RetType> {
  fn isEmpty_0(self , rsthis: & QPainterPath) -> RetType;
}
impl<'a> /*trait*/ QPainterPath_isEmpty_0<bool> for () {
  fn isEmpty_0(self , rsthis: & QPainterPath) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QPainterPath7isEmptyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpainterpath.h:168
// index:0
// Public Visibility=Default Availability=Available
// [8] QPainterPath toReversed() const

/*
Creates and returns a reversed copy of the path.

It is the order of the elements that is reversed: If a QPainterPath is composed by calling the moveTo(), lineTo() and cubicTo() functions in the specified order, the reversed copy is composed by calling cubicTo(), lineTo() and moveTo().
*/
impl /*struct*/ QPainterPath {
  pub fn toReversed_0<RetType, T: QPainterPath_toReversed_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toReversed_0(self);
    // return 1;
  }
}
pub trait QPainterPath_toReversed_0<RetType> {
  fn toReversed_0(self , rsthis: & QPainterPath) -> RetType;
}
impl<'a> /*trait*/ QPainterPath_toReversed_0<usize> for () {
  fn toReversed_0(self , rsthis: & QPainterPath) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QPainterPath10toReversedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpainterpath.h:171
// index:0
// Public Visibility=Default Availability=Available
// [8] QPolygonF toFillPolygon(const QMatrix &) const

/*
Converts the path into a polygon using the QTransform matrix, and returns the polygon.

The polygon is created by first converting all subpaths to polygons, then using a rewinding technique to make sure that overlapping subpaths can be filled using the correct fill rule.

Note that rewinding inserts addition lines in the polygon so the outline of the fill polygon does not match the outline of the path.

See also toSubpathPolygons(), toFillPolygons(), and QPainterPath Conversion.
*/
impl /*struct*/ QPainterPath {
  pub fn toFillPolygon_0<RetType, T: QPainterPath_toFillPolygon_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toFillPolygon_0(self);
    // return 1;
  }
}
pub trait QPainterPath_toFillPolygon_0<RetType> {
  fn toFillPolygon_0(self , rsthis: & QPainterPath) -> RetType;
}
impl<'a> /*trait*/ QPainterPath_toFillPolygon_0<usize> for (usize) {
  fn toFillPolygon_0(self , rsthis: & QPainterPath) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QPainterPath13toFillPolygonERK7QMatrix", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpainterpath.h:174
// index:1
// Public Visibility=Default Availability=Available
// [8] QPolygonF toFillPolygon(const QTransform &) const

/*
Converts the path into a polygon using the QTransform matrix, and returns the polygon.

The polygon is created by first converting all subpaths to polygons, then using a rewinding technique to make sure that overlapping subpaths can be filled using the correct fill rule.

Note that rewinding inserts addition lines in the polygon so the outline of the fill polygon does not match the outline of the path.

See also toSubpathPolygons(), toFillPolygons(), and QPainterPath Conversion.
*/
impl /*struct*/ QPainterPath {
  pub fn toFillPolygon_1<RetType, T: QPainterPath_toFillPolygon_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toFillPolygon_1(self);
    // return 1;
  }
}
pub trait QPainterPath_toFillPolygon_1<RetType> {
  fn toFillPolygon_1(self , rsthis: & QPainterPath) -> RetType;
}
impl<'a> /*trait*/ QPainterPath_toFillPolygon_1<usize> for (usize) {
  fn toFillPolygon_1(self , rsthis: & QPainterPath) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QPainterPath13toFillPolygonERK10QTransform", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpainterpath.h:176
// index:0
// Public Visibility=Default Availability=Available
// [4] int elementCount() const

/*
Returns the number of path elements in the painter path.

See also ElementType, elementAt(), and isEmpty().
*/
impl /*struct*/ QPainterPath {
  pub fn elementCount_0<RetType, T: QPainterPath_elementCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.elementCount_0(self);
    // return 1;
  }
}
pub trait QPainterPath_elementCount_0<RetType> {
  fn elementCount_0(self , rsthis: & QPainterPath) -> RetType;
}
impl<'a> /*trait*/ QPainterPath_elementCount_0<i32> for () {
  fn elementCount_0(self , rsthis: & QPainterPath) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QPainterPath12elementCountEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpainterpath.h:177
// index:0
// Public Visibility=Default Availability=Available
// [24] QPainterPath::Element elementAt(int) const

/*
Returns the element at the given index in the painter path.

See also ElementType, elementCount(), and isEmpty().
*/
impl /*struct*/ QPainterPath {
  pub fn elementAt_0<RetType, T: QPainterPath_elementAt_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.elementAt_0(self);
    // return 1;
  }
}
pub trait QPainterPath_elementAt_0<RetType> {
  fn elementAt_0(self , rsthis: & QPainterPath) -> RetType;
}
impl<'a> /*trait*/ QPainterPath_elementAt_0<i32> for (i32) {
  fn elementAt_0(self , rsthis: & QPainterPath) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QPainterPath9elementAtEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpainterpath.h:178
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setElementPositionAt(int, qreal, qreal)

/*
Sets the x and y coordinate of the element at index index to x and y.

This function was introduced in  Qt 4.2.
*/
impl /*struct*/ QPainterPath {
  pub fn setElementPositionAt_0<RetType, T: QPainterPath_setElementPositionAt_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setElementPositionAt_0(self);
    // return 1;
  }
}
pub trait QPainterPath_setElementPositionAt_0<RetType> {
  fn setElementPositionAt_0(self , rsthis: & QPainterPath) -> RetType;
}
impl<'a> /*trait*/ QPainterPath_setElementPositionAt_0<(/*void*/)> for (i32,f64,f64) {
  fn setElementPositionAt_0(self , rsthis: & QPainterPath) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let arg2 = (&self.2) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN12QPainterPath20setElementPositionAtEidd", 3,qtrt::FFITY_INT,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpainterpath.h:180
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal length() const

/*
Returns the length of the current path.
*/
impl /*struct*/ QPainterPath {
  pub fn length_0<RetType, T: QPainterPath_length_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.length_0(self);
    // return 1;
  }
}
pub trait QPainterPath_length_0<RetType> {
  fn length_0(self , rsthis: & QPainterPath) -> RetType;
}
impl<'a> /*trait*/ QPainterPath_length_0<f64> for () {
  fn length_0(self , rsthis: & QPainterPath) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QPainterPath6lengthEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpainterpath.h:181
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal percentAtLength(qreal) const

/*
Returns percentage of the whole path at the specified length len.

Note that similarly to other percent methods, the percentage measurement is not linear with regards to the length, if curves are present in the path. When curves are present the percentage argument is mapped to the t parameter of the Bezier equations.
*/
impl /*struct*/ QPainterPath {
  pub fn percentAtLength_0<RetType, T: QPainterPath_percentAtLength_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.percentAtLength_0(self);
    // return 1;
  }
}
pub trait QPainterPath_percentAtLength_0<RetType> {
  fn percentAtLength_0(self , rsthis: & QPainterPath) -> RetType;
}
impl<'a> /*trait*/ QPainterPath_percentAtLength_0<f64> for (f64) {
  fn percentAtLength_0(self , rsthis: & QPainterPath) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QPainterPath15percentAtLengthEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpainterpath.h:182
// index:0
// Public Visibility=Default Availability=Available
// [16] QPointF pointAtPercent(qreal) const

/*
Returns the point at at the percentage t of the current path. The argument t has to be between 0 and 1.

Note that similarly to other percent methods, the percentage measurement is not linear with regards to the length, if curves are present in the path. When curves are present the percentage argument is mapped to the t parameter of the Bezier equations.
*/
impl /*struct*/ QPainterPath {
  pub fn pointAtPercent_0<RetType, T: QPainterPath_pointAtPercent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.pointAtPercent_0(self);
    // return 1;
  }
}
pub trait QPainterPath_pointAtPercent_0<RetType> {
  fn pointAtPercent_0(self , rsthis: & QPainterPath) -> RetType;
}
impl<'a> /*trait*/ QPainterPath_pointAtPercent_0<usize> for (f64) {
  fn pointAtPercent_0(self , rsthis: & QPainterPath) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QPainterPath14pointAtPercentEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpainterpath.h:183
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal angleAtPercent(qreal) const

/*
Returns the angle of the path tangent at the percentage t. The argument t has to be between 0 and 1.

Positive values for the angles mean counter-clockwise while negative values mean the clockwise direction. Zero degrees is at the 3 o'clock position.

Note that similarly to the other percent methods, the percentage measurement is not linear with regards to the length if curves are present in the path. When curves are present the percentage argument is mapped to the t parameter of the Bezier equations.
*/
impl /*struct*/ QPainterPath {
  pub fn angleAtPercent_0<RetType, T: QPainterPath_angleAtPercent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.angleAtPercent_0(self);
    // return 1;
  }
}
pub trait QPainterPath_angleAtPercent_0<RetType> {
  fn angleAtPercent_0(self , rsthis: & QPainterPath) -> RetType;
}
impl<'a> /*trait*/ QPainterPath_angleAtPercent_0<f64> for (f64) {
  fn angleAtPercent_0(self , rsthis: & QPainterPath) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QPainterPath14angleAtPercentEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpainterpath.h:184
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal slopeAtPercent(qreal) const

/*
Returns the slope of the path at the percentage t. The argument t has to be between 0 and 1.

Note that similarly to other percent methods, the percentage measurement is not linear with regards to the length, if curves are present in the path. When curves are present the percentage argument is mapped to the t parameter of the Bezier equations.
*/
impl /*struct*/ QPainterPath {
  pub fn slopeAtPercent_0<RetType, T: QPainterPath_slopeAtPercent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.slopeAtPercent_0(self);
    // return 1;
  }
}
pub trait QPainterPath_slopeAtPercent_0<RetType> {
  fn slopeAtPercent_0(self , rsthis: & QPainterPath) -> RetType;
}
impl<'a> /*trait*/ QPainterPath_slopeAtPercent_0<f64> for (f64) {
  fn slopeAtPercent_0(self , rsthis: & QPainterPath) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QPainterPath14slopeAtPercentEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpainterpath.h:188
// index:0
// Public Visibility=Default Availability=Available
// [8] QPainterPath united(const QPainterPath &) const

/*
Returns a path which is the union of this path's fill area and p's fill area.

Set operations on paths will treat the paths as areas. Non-closed paths will be treated as implicitly closed. Bezier curves may be flattened to line segments due to numerical instability of doing bezier curve intersections.

This function was introduced in  Qt 4.3.

See also intersected() and subtracted().
*/
impl /*struct*/ QPainterPath {
  pub fn united_0<RetType, T: QPainterPath_united_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.united_0(self);
    // return 1;
  }
}
pub trait QPainterPath_united_0<RetType> {
  fn united_0(self , rsthis: & QPainterPath) -> RetType;
}
impl<'a> /*trait*/ QPainterPath_united_0<usize> for (usize) {
  fn united_0(self , rsthis: & QPainterPath) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QPainterPath6unitedERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpainterpath.h:189
// index:0
// Public Visibility=Default Availability=Available
// [8] QPainterPath intersected(const QPainterPath &) const

/*
Returns a path which is the intersection of this path's fill area and p's fill area. Bezier curves may be flattened to line segments due to numerical instability of doing bezier curve intersections.

This function was introduced in  Qt 4.3.
*/
impl /*struct*/ QPainterPath {
  pub fn intersected_0<RetType, T: QPainterPath_intersected_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.intersected_0(self);
    // return 1;
  }
}
pub trait QPainterPath_intersected_0<RetType> {
  fn intersected_0(self , rsthis: & QPainterPath) -> RetType;
}
impl<'a> /*trait*/ QPainterPath_intersected_0<usize> for (usize) {
  fn intersected_0(self , rsthis: & QPainterPath) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QPainterPath11intersectedERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpainterpath.h:190
// index:0
// Public Visibility=Default Availability=Available
// [8] QPainterPath subtracted(const QPainterPath &) const

/*
Returns a path which is p's fill area subtracted from this path's fill area.

Set operations on paths will treat the paths as areas. Non-closed paths will be treated as implicitly closed. Bezier curves may be flattened to line segments due to numerical instability of doing bezier curve intersections.

This function was introduced in  Qt 4.3.
*/
impl /*struct*/ QPainterPath {
  pub fn subtracted_0<RetType, T: QPainterPath_subtracted_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.subtracted_0(self);
    // return 1;
  }
}
pub trait QPainterPath_subtracted_0<RetType> {
  fn subtracted_0(self , rsthis: & QPainterPath) -> RetType;
}
impl<'a> /*trait*/ QPainterPath_subtracted_0<usize> for (usize) {
  fn subtracted_0(self , rsthis: & QPainterPath) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QPainterPath10subtractedERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpainterpath.h:191
// index:0
// Public Visibility=Default Availability=Available
// [8] QPainterPath subtractedInverted(const QPainterPath &) const

/*

*/
impl /*struct*/ QPainterPath {
  pub fn subtractedInverted_0<RetType, T: QPainterPath_subtractedInverted_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.subtractedInverted_0(self);
    // return 1;
  }
}
pub trait QPainterPath_subtractedInverted_0<RetType> {
  fn subtractedInverted_0(self , rsthis: & QPainterPath) -> RetType;
}
impl<'a> /*trait*/ QPainterPath_subtractedInverted_0<usize> for (usize) {
  fn subtractedInverted_0(self , rsthis: & QPainterPath) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QPainterPath18subtractedInvertedERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpainterpath.h:193
// index:0
// Public Visibility=Default Availability=Available
// [8] QPainterPath simplified() const

/*
Returns a simplified version of this path. This implies merging all subpaths that intersect, and returning a path containing no intersecting edges. Consecutive parallel lines will also be merged. The simplified path will always use the default fill rule, Qt::OddEvenFill. Bezier curves may be flattened to line segments due to numerical instability of doing bezier curve intersections.

This function was introduced in  Qt 4.4.
*/
impl /*struct*/ QPainterPath {
  pub fn simplified_0<RetType, T: QPainterPath_simplified_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.simplified_0(self);
    // return 1;
  }
}
pub trait QPainterPath_simplified_0<RetType> {
  fn simplified_0(self , rsthis: & QPainterPath) -> RetType;
}
impl<'a> /*trait*/ QPainterPath_simplified_0<usize> for () {
  fn simplified_0(self , rsthis: & QPainterPath) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QPainterPath10simplifiedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpainterpath.h:195
// index:0
// Public Visibility=Default Availability=Available
// [1] bool operator==(const QPainterPath &) const

/*

*/
impl /*struct*/ QPainterPath {
  pub fn operator_equal_equal_0<RetType, T: QPainterPath_operator_equal_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_equal_0(self);
    // return 1;
  }
}
pub trait QPainterPath_operator_equal_equal_0<RetType> {
  fn operator_equal_equal_0(self , rsthis: & QPainterPath) -> RetType;
}
impl<'a> /*trait*/ QPainterPath_operator_equal_equal_0<bool> for (usize) {
  fn operator_equal_equal_0(self , rsthis: & QPainterPath) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QPainterPatheqERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpainterpath.h:196
// index:0
// Public Visibility=Default Availability=Available
// [1] bool operator!=(const QPainterPath &) const

/*

*/
impl /*struct*/ QPainterPath {
  pub fn operator_not_equal_0<RetType, T: QPainterPath_operator_not_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_not_equal_0(self);
    // return 1;
  }
}
pub trait QPainterPath_operator_not_equal_0<RetType> {
  fn operator_not_equal_0(self , rsthis: & QPainterPath) -> RetType;
}
impl<'a> /*trait*/ QPainterPath_operator_not_equal_0<bool> for (usize) {
  fn operator_not_equal_0(self , rsthis: & QPainterPath) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QPainterPathneERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpainterpath.h:198
// index:0
// Public Visibility=Default Availability=Available
// [8] QPainterPath operator&(const QPainterPath &) const

/*

*/
impl /*struct*/ QPainterPath {
  pub fn operator_and_0<RetType, T: QPainterPath_operator_and_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_and_0(self);
    // return 1;
  }
}
pub trait QPainterPath_operator_and_0<RetType> {
  fn operator_and_0(self , rsthis: & QPainterPath) -> RetType;
}
impl<'a> /*trait*/ QPainterPath_operator_and_0<usize> for (usize) {
  fn operator_and_0(self , rsthis: & QPainterPath) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QPainterPathanERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpainterpath.h:199
// index:0
// Public Visibility=Default Availability=Available
// [8] QPainterPath operator|(const QPainterPath &) const

/*

*/
impl /*struct*/ QPainterPath {
  pub fn operator_or_0<RetType, T: QPainterPath_operator_or_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_or_0(self);
    // return 1;
  }
}
pub trait QPainterPath_operator_or_0<RetType> {
  fn operator_or_0(self , rsthis: & QPainterPath) -> RetType;
}
impl<'a> /*trait*/ QPainterPath_operator_or_0<usize> for (usize) {
  fn operator_or_0(self , rsthis: & QPainterPath) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QPainterPathorERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpainterpath.h:200
// index:0
// Public Visibility=Default Availability=Available
// [8] QPainterPath operator+(const QPainterPath &) const

/*

*/
impl /*struct*/ QPainterPath {
  pub fn operator_add_0<RetType, T: QPainterPath_operator_add_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_add_0(self);
    // return 1;
  }
}
pub trait QPainterPath_operator_add_0<RetType> {
  fn operator_add_0(self , rsthis: & QPainterPath) -> RetType;
}
impl<'a> /*trait*/ QPainterPath_operator_add_0<usize> for (usize) {
  fn operator_add_0(self , rsthis: & QPainterPath) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QPainterPathplERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpainterpath.h:201
// index:0
// Public Visibility=Default Availability=Available
// [8] QPainterPath operator-(const QPainterPath &) const

/*
Subtracts the other path from a copy of this path, and returns the copy.

This function was introduced in  Qt 4.5.

See also subtracted(), operator-=(), and operator+().
*/
impl /*struct*/ QPainterPath {
  pub fn operator_minus_0<RetType, T: QPainterPath_operator_minus_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_minus_0(self);
    // return 1;
  }
}
pub trait QPainterPath_operator_minus_0<RetType> {
  fn operator_minus_0(self , rsthis: & QPainterPath) -> RetType;
}
impl<'a> /*trait*/ QPainterPath_operator_minus_0<usize> for (usize) {
  fn operator_minus_0(self , rsthis: & QPainterPath) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QPainterPathmiERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpainterpath.h:202
// index:0
// Public Visibility=Default Availability=Available
// [8] QPainterPath & operator&=(const QPainterPath &)

/*

*/
impl /*struct*/ QPainterPath {
  pub fn operator_and_equal_0<RetType, T: QPainterPath_operator_and_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_and_equal_0(self);
    // return 1;
  }
}
pub trait QPainterPath_operator_and_equal_0<RetType> {
  fn operator_and_equal_0(self , rsthis: & QPainterPath) -> RetType;
}
impl<'a> /*trait*/ QPainterPath_operator_and_equal_0<usize> for (usize) {
  fn operator_and_equal_0(self , rsthis: & QPainterPath) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN12QPainterPathaNERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpainterpath.h:203
// index:0
// Public Visibility=Default Availability=Available
// [8] QPainterPath & operator|=(const QPainterPath &)

/*

*/
impl /*struct*/ QPainterPath {
  pub fn operator_or_equal_0<RetType, T: QPainterPath_operator_or_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_or_equal_0(self);
    // return 1;
  }
}
pub trait QPainterPath_operator_or_equal_0<RetType> {
  fn operator_or_equal_0(self , rsthis: & QPainterPath) -> RetType;
}
impl<'a> /*trait*/ QPainterPath_operator_or_equal_0<usize> for (usize) {
  fn operator_or_equal_0(self , rsthis: & QPainterPath) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN12QPainterPathoRERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpainterpath.h:204
// index:0
// Public Visibility=Default Availability=Available
// [8] QPainterPath & operator+=(const QPainterPath &)

/*

*/
impl /*struct*/ QPainterPath {
  pub fn operator_add_equal_0<RetType, T: QPainterPath_operator_add_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_add_equal_0(self);
    // return 1;
  }
}
pub trait QPainterPath_operator_add_equal_0<RetType> {
  fn operator_add_equal_0(self , rsthis: & QPainterPath) -> RetType;
}
impl<'a> /*trait*/ QPainterPath_operator_add_equal_0<usize> for (usize) {
  fn operator_add_equal_0(self , rsthis: & QPainterPath) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN12QPainterPathpLERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpainterpath.h:205
// index:0
// Public Visibility=Default Availability=Available
// [8] QPainterPath & operator-=(const QPainterPath &)

/*

*/
impl /*struct*/ QPainterPath {
  pub fn operator_minus_equal_0<RetType, T: QPainterPath_operator_minus_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_minus_equal_0(self);
    // return 1;
  }
}
pub trait QPainterPath_operator_minus_equal_0<RetType> {
  fn operator_minus_equal_0(self , rsthis: & QPainterPath) -> RetType;
}
impl<'a> /*trait*/ QPainterPath_operator_minus_equal_0<usize> for (usize) {
  fn operator_minus_equal_0(self , rsthis: & QPainterPath) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN12QPainterPathmIERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}


/*
This enum describes the types of elements used to connect vertices in subpaths.

Note that elements added as closed subpaths using the addEllipse(), addPath(), addPolygon(), addRect(), addRegion() and addText() convenience functions, is actually added to the path as a collection of separate elements using the moveTo(), lineTo() and cubicTo() functions.



See also elementAt() and elementCount().

*/
pub type QPainterPath__ElementType = i32;
// A new subpath. See also moveTo().
pub const QPainterPath__MoveToElement :QPainterPath__ElementType = 0;
// A line. See also lineTo().
pub const QPainterPath__LineToElement :QPainterPath__ElementType = 1;
// A curve. See also cubicTo() and quadTo().
pub const QPainterPath__CurveToElement :QPainterPath__ElementType = 2;
// The extra data required to describe a curve in a CurveToElement element.
pub const QPainterPath__CurveToDataElement :QPainterPath__ElementType = 3;
pub fn QPainterPath_ElementTypeItemName(val: i32) ->String {
  match val {
     QPainterPath__MoveToElement => // 0
     {return String::from("MoveToElement");}
     QPainterPath__LineToElement => // 1
     {return String::from("LineToElement");}
     QPainterPath__CurveToElement => // 2
     {return String::from("CurveToElement");}
     QPainterPath__CurveToDataElement => // 3
     {return String::from("CurveToDataElement");}
  _ => {return format!("{}", val);}
}
}
pub fn QPainterPath_ElementTypeItemName_s(val: i32) ->String {
  //var nilthis *QPainterPath
  //return nilthis.ElementTypeItemName(val);
  return QPainterPath_ElementTypeItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
