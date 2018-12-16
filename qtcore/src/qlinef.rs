

// mod ::core::QLineF
// package qtcore
// /usr/include/qt/QtCore/qline.h
// #include <qline.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 23
*/
// import "C"
// import "unsafe"
// import "reflect"
// import "fmt"
// import "log"
// import "github.com/kitech/qt.go/qtrt"
use std::default::Default;
use std::ops::Deref;
use qtrt; // super::super::qtrt;
use super::*;
//  ext block end

//  body block begin



/*

*/
#[derive(Default)] // class sizeof(QLineF)=32
pub struct QLineF {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QLineF_ITF interface {
//    QLineF_PTR() *QLineF
//}
//func (ptr *QLineF) QLineF_PTR() *QLineF { return ptr }

impl /*struct*/ QLineF {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QLineF {
    return QLineF{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QLineF {
//  type Target = QLineFBASE;
//
//  fn deref(&self) -> &QLineFBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QLineFBASE> for QLineF {
//  fn as_ref(& self) -> & QLineFBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qline.h:219
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void QLineF()

/*

*/
// QLineF() ctx.fn_proto_cpp
impl /*struct*/ QLineF {
  pub fn QLineF_0<T: QLineF_QLineF_0>(value: T) -> QLineF {
    let rsthis = value.QLineF_0();
    return rsthis;
    // return 1;
  }
}

pub trait QLineF_QLineF_0 {
  fn QLineF_0(self) -> QLineF;
}
// QLineF() ctx.fn_proto_cpp
impl<'a> /*trait*/ QLineF_QLineF_0 for () {
  fn QLineF_0(self) -> QLineF {
    // unsafe{_ZN6QLineFC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN6QLineFC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QLineF{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qline.h:220
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void QLineF(const QPointF &, const QPointF &)

/*

*/
// QLineF(const QPointF &, const QPointF &) ctx.fn_proto_cpp
impl /*struct*/ QLineF {
  pub fn QLineF_1<T: QLineF_QLineF_1>(value: T) -> QLineF {
    let rsthis = value.QLineF_1();
    return rsthis;
    // return 1;
  }
}

pub trait QLineF_QLineF_1 {
  fn QLineF_1(self) -> QLineF;
}
// QLineF(const QPointF &, const QPointF &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QLineF_QLineF_1 for (usize,usize) {
  fn QLineF_1(self) -> QLineF {
    // unsafe{_ZN6QLineFC2ERK7QPointFS2_()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN6QLineFC2ERK7QPointFS2_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QLineF{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qline.h:221
// index:2
// Public inline Visibility=Default Availability=Available
// [-2] void QLineF(qreal, qreal, qreal, qreal)

/*

*/
// QLineF(qreal, qreal, qreal, qreal) ctx.fn_proto_cpp
impl /*struct*/ QLineF {
  pub fn QLineF_2<T: QLineF_QLineF_2>(value: T) -> QLineF {
    let rsthis = value.QLineF_2();
    return rsthis;
    // return 1;
  }
}

pub trait QLineF_QLineF_2 {
  fn QLineF_2(self) -> QLineF;
}
// QLineF(qreal, qreal, qreal, qreal) ctx.fn_proto_cpp
impl<'a> /*trait*/ QLineF_QLineF_2 for (f64,f64,f64,f64) {
  fn QLineF_2(self) -> QLineF {
    // unsafe{_ZN6QLineFC2Edddd()};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let arg2 = (&self.2) as *const f64 as usize;
    let arg3 = (&self.3) as *const f64 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN6QLineFC2Edddd", 4,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QLineF{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qline.h:222
// index:3
// Public inline Visibility=Default Availability=Available
// [-2] void QLineF(const QLine &)

/*

*/
// QLineF(const QLine &) ctx.fn_proto_cpp
impl /*struct*/ QLineF {
  pub fn QLineF_3<T: QLineF_QLineF_3>(value: T) -> QLineF {
    let rsthis = value.QLineF_3();
    return rsthis;
    // return 1;
  }
}

pub trait QLineF_QLineF_3 {
  fn QLineF_3(self) -> QLineF;
}
// QLineF(const QLine &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QLineF_QLineF_3 for (usize) {
  fn QLineF_3(self) -> QLineF {
    // unsafe{_ZN6QLineFC2ERK5QLine()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN6QLineFC2ERK5QLine", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QLineF{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qline.h:224
// index:0
// Public static Visibility=Default Availability=Available
// [32] QLineF fromPolar(qreal, qreal)

/*

*/
impl /*struct*/ QLineF {
  pub fn fromPolar_0<RetType, T: QLineF_fromPolar_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromPolar_0();
    // return 1;
  }
}
pub trait QLineF_fromPolar_0<RetType> {
  fn fromPolar_0(self ) -> RetType;
}
impl<'a> /*trait*/ QLineF_fromPolar_0<usize> for (f64,f64) {
  fn fromPolar_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN6QLineF9fromPolarEdd", 2,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qline.h:226
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isNull() const

/*
Returns true if the line is not set up with valid start and end point; otherwise returns false.
*/
impl /*struct*/ QLineF {
  pub fn isNull_0<RetType, T: QLineF_isNull_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isNull_0(self);
    // return 1;
  }
}
pub trait QLineF_isNull_0<RetType> {
  fn isNull_0(self , rsthis: & QLineF) -> RetType;
}
impl<'a> /*trait*/ QLineF_isNull_0<bool> for () {
  fn isNull_0(self , rsthis: & QLineF) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QLineF6isNullEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qline.h:228
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QPointF p1() const

/*
Returns the line's start point.

See also setP1(), x1(), y1(), and p2().
*/
impl /*struct*/ QLineF {
  pub fn p1_0<RetType, T: QLineF_p1_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.p1_0(self);
    // return 1;
  }
}
pub trait QLineF_p1_0<RetType> {
  fn p1_0(self , rsthis: & QLineF) -> RetType;
}
impl<'a> /*trait*/ QLineF_p1_0<usize> for () {
  fn p1_0(self , rsthis: & QLineF) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QLineF2p1Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qline.h:229
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QPointF p2() const

/*
Returns the line's end point.

See also setP2(), x2(), y2(), and p1().
*/
impl /*struct*/ QLineF {
  pub fn p2_0<RetType, T: QLineF_p2_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.p2_0(self);
    // return 1;
  }
}
pub trait QLineF_p2_0<RetType> {
  fn p2_0(self , rsthis: & QLineF) -> RetType;
}
impl<'a> /*trait*/ QLineF_p2_0<usize> for () {
  fn p2_0(self , rsthis: & QLineF) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QLineF2p2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qline.h:231
// index:0
// Public inline Visibility=Default Availability=Available
// [8] qreal x1() const

/*
Returns the x-coordinate of the line's start point.

See also p1().
*/
impl /*struct*/ QLineF {
  pub fn x1_0<RetType, T: QLineF_x1_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.x1_0(self);
    // return 1;
  }
}
pub trait QLineF_x1_0<RetType> {
  fn x1_0(self , rsthis: & QLineF) -> RetType;
}
impl<'a> /*trait*/ QLineF_x1_0<f64> for () {
  fn x1_0(self , rsthis: & QLineF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QLineF2x1Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qline.h:232
// index:0
// Public inline Visibility=Default Availability=Available
// [8] qreal y1() const

/*
Returns the y-coordinate of the line's start point.

See also p1().
*/
impl /*struct*/ QLineF {
  pub fn y1_0<RetType, T: QLineF_y1_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.y1_0(self);
    // return 1;
  }
}
pub trait QLineF_y1_0<RetType> {
  fn y1_0(self , rsthis: & QLineF) -> RetType;
}
impl<'a> /*trait*/ QLineF_y1_0<f64> for () {
  fn y1_0(self , rsthis: & QLineF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QLineF2y1Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qline.h:234
// index:0
// Public inline Visibility=Default Availability=Available
// [8] qreal x2() const

/*
Returns the x-coordinate of the line's end point.

See also p2().
*/
impl /*struct*/ QLineF {
  pub fn x2_0<RetType, T: QLineF_x2_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.x2_0(self);
    // return 1;
  }
}
pub trait QLineF_x2_0<RetType> {
  fn x2_0(self , rsthis: & QLineF) -> RetType;
}
impl<'a> /*trait*/ QLineF_x2_0<f64> for () {
  fn x2_0(self , rsthis: & QLineF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QLineF2x2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qline.h:235
// index:0
// Public inline Visibility=Default Availability=Available
// [8] qreal y2() const

/*
Returns the y-coordinate of the line's end point.

See also p2().
*/
impl /*struct*/ QLineF {
  pub fn y2_0<RetType, T: QLineF_y2_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.y2_0(self);
    // return 1;
  }
}
pub trait QLineF_y2_0<RetType> {
  fn y2_0(self , rsthis: & QLineF) -> RetType;
}
impl<'a> /*trait*/ QLineF_y2_0<f64> for () {
  fn y2_0(self , rsthis: & QLineF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QLineF2y2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qline.h:237
// index:0
// Public inline Visibility=Default Availability=Available
// [8] qreal dx() const

/*
Returns the horizontal component of the line's vector.

See also dy().
*/
impl /*struct*/ QLineF {
  pub fn dx_0<RetType, T: QLineF_dx_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dx_0(self);
    // return 1;
  }
}
pub trait QLineF_dx_0<RetType> {
  fn dx_0(self , rsthis: & QLineF) -> RetType;
}
impl<'a> /*trait*/ QLineF_dx_0<f64> for () {
  fn dx_0(self , rsthis: & QLineF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QLineF2dxEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qline.h:238
// index:0
// Public inline Visibility=Default Availability=Available
// [8] qreal dy() const

/*
Returns the vertical component of the line's vector.

See also dx().
*/
impl /*struct*/ QLineF {
  pub fn dy_0<RetType, T: QLineF_dy_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dy_0(self);
    // return 1;
  }
}
pub trait QLineF_dy_0<RetType> {
  fn dy_0(self , rsthis: & QLineF) -> RetType;
}
impl<'a> /*trait*/ QLineF_dy_0<f64> for () {
  fn dy_0(self , rsthis: & QLineF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QLineF2dyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qline.h:240
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal length() const

/*

*/
impl /*struct*/ QLineF {
  pub fn length_0<RetType, T: QLineF_length_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.length_0(self);
    // return 1;
  }
}
pub trait QLineF_length_0<RetType> {
  fn length_0(self , rsthis: & QLineF) -> RetType;
}
impl<'a> /*trait*/ QLineF_length_0<f64> for () {
  fn length_0(self , rsthis: & QLineF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QLineF6lengthEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qline.h:241
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setLength(qreal)

/*

*/
impl /*struct*/ QLineF {
  pub fn setLength_0<RetType, T: QLineF_setLength_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setLength_0(self);
    // return 1;
  }
}
pub trait QLineF_setLength_0<RetType> {
  fn setLength_0(self , rsthis: & QLineF) -> RetType;
}
impl<'a> /*trait*/ QLineF_setLength_0<(/*void*/)> for (f64) {
  fn setLength_0(self , rsthis: & QLineF) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN6QLineF9setLengthEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qline.h:243
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal angle() const

/*

*/
impl /*struct*/ QLineF {
  pub fn angle_0<RetType, T: QLineF_angle_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.angle_0(self);
    // return 1;
  }
}
pub trait QLineF_angle_0<RetType> {
  fn angle_0(self , rsthis: & QLineF) -> RetType;
}
impl<'a> /*trait*/ QLineF_angle_0<f64> for () {
  fn angle_0(self , rsthis: & QLineF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QLineF5angleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qline.h:254
// index:1
// Public Visibility=Default Availability=Available
// [8] qreal angle(const QLineF &) const

/*

*/
impl /*struct*/ QLineF {
  pub fn angle_1<RetType, T: QLineF_angle_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.angle_1(self);
    // return 1;
  }
}
pub trait QLineF_angle_1<RetType> {
  fn angle_1(self , rsthis: & QLineF) -> RetType;
}
impl<'a> /*trait*/ QLineF_angle_1<f64> for (usize) {
  fn angle_1(self , rsthis: & QLineF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QLineF5angleERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qline.h:244
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setAngle(qreal)

/*

*/
impl /*struct*/ QLineF {
  pub fn setAngle_0<RetType, T: QLineF_setAngle_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setAngle_0(self);
    // return 1;
  }
}
pub trait QLineF_setAngle_0<RetType> {
  fn setAngle_0(self , rsthis: & QLineF) -> RetType;
}
impl<'a> /*trait*/ QLineF_setAngle_0<(/*void*/)> for (f64) {
  fn setAngle_0(self , rsthis: & QLineF) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN6QLineF8setAngleEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qline.h:246
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal angleTo(const QLineF &) const

/*

*/
impl /*struct*/ QLineF {
  pub fn angleTo_0<RetType, T: QLineF_angleTo_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.angleTo_0(self);
    // return 1;
  }
}
pub trait QLineF_angleTo_0<RetType> {
  fn angleTo_0(self , rsthis: & QLineF) -> RetType;
}
impl<'a> /*trait*/ QLineF_angleTo_0<f64> for (usize) {
  fn angleTo_0(self , rsthis: & QLineF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QLineF7angleToERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qline.h:248
// index:0
// Public Visibility=Default Availability=Available
// [32] QLineF unitVector() const

/*

*/
impl /*struct*/ QLineF {
  pub fn unitVector_0<RetType, T: QLineF_unitVector_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.unitVector_0(self);
    // return 1;
  }
}
pub trait QLineF_unitVector_0<RetType> {
  fn unitVector_0(self , rsthis: & QLineF) -> RetType;
}
impl<'a> /*trait*/ QLineF_unitVector_0<usize> for () {
  fn unitVector_0(self , rsthis: & QLineF) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QLineF10unitVectorEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qline.h:249
// index:0
// Public inline Visibility=Default Availability=Available
// [32] QLineF normalVector() const

/*

*/
impl /*struct*/ QLineF {
  pub fn normalVector_0<RetType, T: QLineF_normalVector_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.normalVector_0(self);
    // return 1;
  }
}
pub trait QLineF_normalVector_0<RetType> {
  fn normalVector_0(self , rsthis: & QLineF) -> RetType;
}
impl<'a> /*trait*/ QLineF_normalVector_0<usize> for () {
  fn normalVector_0(self , rsthis: & QLineF) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QLineF12normalVectorEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qline.h:252
// index:0
// Public Visibility=Default Availability=Available
// [4] QLineF::IntersectType intersect(const QLineF &, QPointF *) const

/*

*/
impl /*struct*/ QLineF {
  pub fn intersect_0<RetType, T: QLineF_intersect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.intersect_0(self);
    // return 1;
  }
}
pub trait QLineF_intersect_0<RetType> {
  fn intersect_0(self , rsthis: & QLineF) -> RetType;
}
impl<'a> /*trait*/ QLineF_intersect_0<i32> for (usize,usize) {
  fn intersect_0(self , rsthis: & QLineF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QLineF9intersectERKS_P7QPointF", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qline.h:256
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QPointF pointAt(qreal) const

/*

*/
impl /*struct*/ QLineF {
  pub fn pointAt_0<RetType, T: QLineF_pointAt_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.pointAt_0(self);
    // return 1;
  }
}
pub trait QLineF_pointAt_0<RetType> {
  fn pointAt_0(self , rsthis: & QLineF) -> RetType;
}
impl<'a> /*trait*/ QLineF_pointAt_0<usize> for (f64) {
  fn pointAt_0(self , rsthis: & QLineF) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QLineF7pointAtEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qline.h:257
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void translate(const QPointF &)

/*
Translates this line by the given offset.
*/
impl /*struct*/ QLineF {
  pub fn translate_0<RetType, T: QLineF_translate_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.translate_0(self);
    // return 1;
  }
}
pub trait QLineF_translate_0<RetType> {
  fn translate_0(self , rsthis: & QLineF) -> RetType;
}
impl<'a> /*trait*/ QLineF_translate_0<(/*void*/)> for (usize) {
  fn translate_0(self , rsthis: & QLineF) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN6QLineF9translateERK7QPointF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qline.h:258
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void translate(qreal, qreal)

/*
Translates this line by the given offset.
*/
impl /*struct*/ QLineF {
  pub fn translate_1<RetType, T: QLineF_translate_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.translate_1(self);
    // return 1;
  }
}
pub trait QLineF_translate_1<RetType> {
  fn translate_1(self , rsthis: & QLineF) -> RetType;
}
impl<'a> /*trait*/ QLineF_translate_1<(/*void*/)> for (f64,f64) {
  fn translate_1(self , rsthis: & QLineF) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN6QLineF9translateEdd", 2,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qline.h:260
// index:0
// Public inline Visibility=Default Availability=Available
// [32] QLineF translated(const QPointF &) const

/*
Returns this line translated by the given offset.

This function was introduced in  Qt 4.4.
*/
impl /*struct*/ QLineF {
  pub fn translated_0<RetType, T: QLineF_translated_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.translated_0(self);
    // return 1;
  }
}
pub trait QLineF_translated_0<RetType> {
  fn translated_0(self , rsthis: & QLineF) -> RetType;
}
impl<'a> /*trait*/ QLineF_translated_0<usize> for (usize) {
  fn translated_0(self , rsthis: & QLineF) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QLineF10translatedERK7QPointF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qline.h:261
// index:1
// Public inline Visibility=Default Availability=Available
// [32] QLineF translated(qreal, qreal) const

/*
Returns this line translated by the given offset.

This function was introduced in  Qt 4.4.
*/
impl /*struct*/ QLineF {
  pub fn translated_1<RetType, T: QLineF_translated_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.translated_1(self);
    // return 1;
  }
}
pub trait QLineF_translated_1<RetType> {
  fn translated_1(self , rsthis: & QLineF) -> RetType;
}
impl<'a> /*trait*/ QLineF_translated_1<usize> for (f64,f64) {
  fn translated_1(self , rsthis: & QLineF) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QLineF10translatedEdd", 2,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qline.h:263
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QPointF center() const

/*
Returns the center point of this line. This is equivalent to (p1() + p2()) / 2, except it will never overflow.

This function was introduced in  Qt 5.8.
*/
impl /*struct*/ QLineF {
  pub fn center_0<RetType, T: QLineF_center_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.center_0(self);
    // return 1;
  }
}
pub trait QLineF_center_0<RetType> {
  fn center_0(self , rsthis: & QLineF) -> RetType;
}
impl<'a> /*trait*/ QLineF_center_0<usize> for () {
  fn center_0(self , rsthis: & QLineF) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QLineF6centerEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qline.h:265
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setP1(const QPointF &)

/*
Sets the starting point of this line to p1.

This function was introduced in  Qt 4.4.

See also setP2() and p1().
*/
impl /*struct*/ QLineF {
  pub fn setP1_0<RetType, T: QLineF_setP1_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setP1_0(self);
    // return 1;
  }
}
pub trait QLineF_setP1_0<RetType> {
  fn setP1_0(self , rsthis: & QLineF) -> RetType;
}
impl<'a> /*trait*/ QLineF_setP1_0<(/*void*/)> for (usize) {
  fn setP1_0(self , rsthis: & QLineF) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN6QLineF5setP1ERK7QPointF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qline.h:266
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setP2(const QPointF &)

/*
Sets the end point of this line to p2.

This function was introduced in  Qt 4.4.

See also setP1() and p2().
*/
impl /*struct*/ QLineF {
  pub fn setP2_0<RetType, T: QLineF_setP2_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setP2_0(self);
    // return 1;
  }
}
pub trait QLineF_setP2_0<RetType> {
  fn setP2_0(self , rsthis: & QLineF) -> RetType;
}
impl<'a> /*trait*/ QLineF_setP2_0<(/*void*/)> for (usize) {
  fn setP2_0(self , rsthis: & QLineF) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN6QLineF5setP2ERK7QPointF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qline.h:267
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setPoints(const QPointF &, const QPointF &)

/*
Sets the start point of this line to p1 and the end point of this line to p2.

This function was introduced in  Qt 4.4.

See also setP1(), setP2(), p1(), and p2().
*/
impl /*struct*/ QLineF {
  pub fn setPoints_0<RetType, T: QLineF_setPoints_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPoints_0(self);
    // return 1;
  }
}
pub trait QLineF_setPoints_0<RetType> {
  fn setPoints_0(self , rsthis: & QLineF) -> RetType;
}
impl<'a> /*trait*/ QLineF_setPoints_0<(/*void*/)> for (usize,usize) {
  fn setPoints_0(self , rsthis: & QLineF) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN6QLineF9setPointsERK7QPointFS2_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qline.h:268
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setLine(qreal, qreal, qreal, qreal)

/*
Sets this line to the start in x1, y1 and end in x2, y2.

This function was introduced in  Qt 4.4.

See also setP1(), setP2(), p1(), and p2().
*/
impl /*struct*/ QLineF {
  pub fn setLine_0<RetType, T: QLineF_setLine_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setLine_0(self);
    // return 1;
  }
}
pub trait QLineF_setLine_0<RetType> {
  fn setLine_0(self , rsthis: & QLineF) -> RetType;
}
impl<'a> /*trait*/ QLineF_setLine_0<(/*void*/)> for (f64,f64,f64,f64) {
  fn setLine_0(self , rsthis: & QLineF) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let arg2 = (&self.2) as *const f64 as usize;
    let arg3 = (&self.3) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN6QLineF7setLineEdddd", 4,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qline.h:270
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator==(const QLineF &) const

/*

*/
impl /*struct*/ QLineF {
  pub fn operator_equal_equal_0<RetType, T: QLineF_operator_equal_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_equal_0(self);
    // return 1;
  }
}
pub trait QLineF_operator_equal_equal_0<RetType> {
  fn operator_equal_equal_0(self , rsthis: & QLineF) -> RetType;
}
impl<'a> /*trait*/ QLineF_operator_equal_equal_0<bool> for (usize) {
  fn operator_equal_equal_0(self , rsthis: & QLineF) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QLineFeqERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qline.h:271
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator!=(const QLineF &) const

/*

*/
impl /*struct*/ QLineF {
  pub fn operator_not_equal_0<RetType, T: QLineF_operator_not_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_not_equal_0(self);
    // return 1;
  }
}
pub trait QLineF_operator_not_equal_0<RetType> {
  fn operator_not_equal_0(self , rsthis: & QLineF) -> RetType;
}
impl<'a> /*trait*/ QLineF_operator_not_equal_0<bool> for (usize) {
  fn operator_not_equal_0(self , rsthis: & QLineF) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QLineFneERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qline.h:273
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QLine toLine() const

/*

*/
impl /*struct*/ QLineF {
  pub fn toLine_0<RetType, T: QLineF_toLine_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toLine_0(self);
    // return 1;
  }
}
pub trait QLineF_toLine_0<RetType> {
  fn toLine_0(self , rsthis: & QLineF) -> RetType;
}
impl<'a> /*trait*/ QLineF_toLine_0<usize> for () {
  fn toLine_0(self , rsthis: & QLineF) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QLineF6toLineEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}


pub fn DeleteQLineF(this :*mut QLineF) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN6QLineFD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}

/*


*/
pub type QLineF__IntersectType = i32;
// 
pub const QLineF__NoIntersection :QLineF__IntersectType = 0;
// 
pub const QLineF__BoundedIntersection :QLineF__IntersectType = 1;
// 
pub const QLineF__UnboundedIntersection :QLineF__IntersectType = 2;
pub fn QLineF_IntersectTypeItemName(val: i32) ->String {
  match val {
     QLineF__NoIntersection => // 0
     {return String::from("NoIntersection");}
     QLineF__BoundedIntersection => // 1
     {return String::from("BoundedIntersection");}
     QLineF__UnboundedIntersection => // 2
     {return String::from("UnboundedIntersection");}
  _ => {return format!("{}", val);}
}
}
pub fn QLineF_IntersectTypeItemName_s(val: i32) ->String {
  //var nilthis *QLineF
  //return nilthis.IntersectTypeItemName(val);
  return QLineF_IntersectTypeItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
