

// mod ::gui::QRadialGradient
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
// extern C begin: 9
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
#[derive(Default)] // class sizeof(QRadialGradient)=64
pub struct QRadialGradient {
  qbase: QGradient,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QRadialGradient_ITF interface {
//    QGradient_ITF
//    QRadialGradient_PTR() *QRadialGradient
//}
//func (ptr *QRadialGradient) QRadialGradient_PTR() *QRadialGradient { return ptr }

impl /*struct*/ QRadialGradient {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QRadialGradient {
    return QRadialGradient{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QRadialGradient {
//  type Target = QRadialGradientBASE;
//
//  fn deref(&self) -> &QRadialGradientBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QRadialGradientBASE> for QRadialGradient {
//  fn as_ref(& self) -> & QRadialGradientBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qbrush.h:274
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QRadialGradient()

/*

*/
// QRadialGradient() ctx.fn_proto_cpp
impl /*struct*/ QRadialGradient {
  pub fn QRadialGradient_0<T: QRadialGradient_QRadialGradient_0>(value: T) -> QRadialGradient {
    let rsthis = value.QRadialGradient_0();
    return rsthis;
    // return 1;
  }
}

pub trait QRadialGradient_QRadialGradient_0 {
  fn QRadialGradient_0(self) -> QRadialGradient;
}
// QRadialGradient() ctx.fn_proto_cpp
impl<'a> /*trait*/ QRadialGradient_QRadialGradient_0 for () {
  fn QRadialGradient_0(self) -> QRadialGradient {
    // unsafe{_ZN15QRadialGradientC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN15QRadialGradientC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QRadialGradient{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qbrush.h:275
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QRadialGradient(const QPointF &, qreal, const QPointF &)

/*

*/
// QRadialGradient(const QPointF &, qreal, const QPointF &) ctx.fn_proto_cpp
impl /*struct*/ QRadialGradient {
  pub fn QRadialGradient_1<T: QRadialGradient_QRadialGradient_1>(value: T) -> QRadialGradient {
    let rsthis = value.QRadialGradient_1();
    return rsthis;
    // return 1;
  }
}

pub trait QRadialGradient_QRadialGradient_1 {
  fn QRadialGradient_1(self) -> QRadialGradient;
}
// QRadialGradient(const QPointF &, qreal, const QPointF &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QRadialGradient_QRadialGradient_1 for (usize,f64,usize) {
  fn QRadialGradient_1(self) -> QRadialGradient {
    // unsafe{_ZN15QRadialGradientC2ERK7QPointFdS2_()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN15QRadialGradientC2ERK7QPointFdS2_", 3,qtrt::FFITY_POINTER,qtrt::FFITY_DOUBLE,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QRadialGradient{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qbrush.h:276
// index:2
// Public Visibility=Default Availability=Available
// [-2] void QRadialGradient(qreal, qreal, qreal, qreal, qreal)

/*

*/
// QRadialGradient(qreal, qreal, qreal, qreal, qreal) ctx.fn_proto_cpp
impl /*struct*/ QRadialGradient {
  pub fn QRadialGradient_2<T: QRadialGradient_QRadialGradient_2>(value: T) -> QRadialGradient {
    let rsthis = value.QRadialGradient_2();
    return rsthis;
    // return 1;
  }
}

pub trait QRadialGradient_QRadialGradient_2 {
  fn QRadialGradient_2(self) -> QRadialGradient;
}
// QRadialGradient(qreal, qreal, qreal, qreal, qreal) ctx.fn_proto_cpp
impl<'a> /*trait*/ QRadialGradient_QRadialGradient_2 for (f64,f64,f64,f64,f64) {
  fn QRadialGradient_2(self) -> QRadialGradient {
    // unsafe{_ZN15QRadialGradientC2Eddddd()};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let arg2 = (&self.2) as *const f64 as usize;
    let arg3 = (&self.3) as *const f64 as usize;
    let arg4 = (&self.4) as *const f64 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN15QRadialGradientC2Eddddd", 5,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QRadialGradient{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qbrush.h:278
// index:3
// Public Visibility=Default Availability=Available
// [-2] void QRadialGradient(const QPointF &, qreal)

/*

*/
// QRadialGradient(const QPointF &, qreal) ctx.fn_proto_cpp
impl /*struct*/ QRadialGradient {
  pub fn QRadialGradient_3<T: QRadialGradient_QRadialGradient_3>(value: T) -> QRadialGradient {
    let rsthis = value.QRadialGradient_3();
    return rsthis;
    // return 1;
  }
}

pub trait QRadialGradient_QRadialGradient_3 {
  fn QRadialGradient_3(self) -> QRadialGradient;
}
// QRadialGradient(const QPointF &, qreal) ctx.fn_proto_cpp
impl<'a> /*trait*/ QRadialGradient_QRadialGradient_3 for (usize,f64) {
  fn QRadialGradient_3(self) -> QRadialGradient {
    // unsafe{_ZN15QRadialGradientC2ERK7QPointFd()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN15QRadialGradientC2ERK7QPointFd", 2,qtrt::FFITY_POINTER,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QRadialGradient{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qbrush.h:279
// index:4
// Public Visibility=Default Availability=Available
// [-2] void QRadialGradient(qreal, qreal, qreal)

/*

*/
// QRadialGradient(qreal, qreal, qreal) ctx.fn_proto_cpp
impl /*struct*/ QRadialGradient {
  pub fn QRadialGradient_4<T: QRadialGradient_QRadialGradient_4>(value: T) -> QRadialGradient {
    let rsthis = value.QRadialGradient_4();
    return rsthis;
    // return 1;
  }
}

pub trait QRadialGradient_QRadialGradient_4 {
  fn QRadialGradient_4(self) -> QRadialGradient;
}
// QRadialGradient(qreal, qreal, qreal) ctx.fn_proto_cpp
impl<'a> /*trait*/ QRadialGradient_QRadialGradient_4 for (f64,f64,f64) {
  fn QRadialGradient_4(self) -> QRadialGradient {
    // unsafe{_ZN15QRadialGradientC2Eddd()};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let arg2 = (&self.2) as *const f64 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN15QRadialGradientC2Eddd", 3,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QRadialGradient{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qbrush.h:281
// index:5
// Public Visibility=Default Availability=Available
// [-2] void QRadialGradient(const QPointF &, qreal, const QPointF &, qreal)

/*

*/
// QRadialGradient(const QPointF &, qreal, const QPointF &, qreal) ctx.fn_proto_cpp
impl /*struct*/ QRadialGradient {
  pub fn QRadialGradient_5<T: QRadialGradient_QRadialGradient_5>(value: T) -> QRadialGradient {
    let rsthis = value.QRadialGradient_5();
    return rsthis;
    // return 1;
  }
}

pub trait QRadialGradient_QRadialGradient_5 {
  fn QRadialGradient_5(self) -> QRadialGradient;
}
// QRadialGradient(const QPointF &, qreal, const QPointF &, qreal) ctx.fn_proto_cpp
impl<'a> /*trait*/ QRadialGradient_QRadialGradient_5 for (usize,f64,usize,f64) {
  fn QRadialGradient_5(self) -> QRadialGradient {
    // unsafe{_ZN15QRadialGradientC2ERK7QPointFdS2_d()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3) as *const f64 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN15QRadialGradientC2ERK7QPointFdS2_d", 4,qtrt::FFITY_POINTER,qtrt::FFITY_DOUBLE,qtrt::FFITY_POINTER,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QRadialGradient{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qbrush.h:282
// index:6
// Public Visibility=Default Availability=Available
// [-2] void QRadialGradient(qreal, qreal, qreal, qreal, qreal, qreal)

/*

*/
// QRadialGradient(qreal, qreal, qreal, qreal, qreal, qreal) ctx.fn_proto_cpp
impl /*struct*/ QRadialGradient {
  pub fn QRadialGradient_6<T: QRadialGradient_QRadialGradient_6>(value: T) -> QRadialGradient {
    let rsthis = value.QRadialGradient_6();
    return rsthis;
    // return 1;
  }
}

pub trait QRadialGradient_QRadialGradient_6 {
  fn QRadialGradient_6(self) -> QRadialGradient;
}
// QRadialGradient(qreal, qreal, qreal, qreal, qreal, qreal) ctx.fn_proto_cpp
impl<'a> /*trait*/ QRadialGradient_QRadialGradient_6 for (f64,f64,f64,f64,f64,f64) {
  fn QRadialGradient_6(self) -> QRadialGradient {
    // unsafe{_ZN15QRadialGradientC2Edddddd()};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let arg2 = (&self.2) as *const f64 as usize;
    let arg3 = (&self.3) as *const f64 as usize;
    let arg4 = (&self.4) as *const f64 as usize;
    let arg5 = (&self.5) as *const f64 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN15QRadialGradientC2Edddddd", 6,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,arg5,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QRadialGradient{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qbrush.h:284
// index:0
// Public Visibility=Default Availability=Available
// [16] QPointF center() const

/*

*/
impl /*struct*/ QRadialGradient {
  pub fn center_0<RetType, T: QRadialGradient_center_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.center_0(self);
    // return 1;
  }
}
pub trait QRadialGradient_center_0<RetType> {
  fn center_0(self , rsthis: & QRadialGradient) -> RetType;
}
impl<'a> /*trait*/ QRadialGradient_center_0<usize> for () {
  fn center_0(self , rsthis: & QRadialGradient) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QRadialGradient6centerEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qbrush.h:285
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCenter(const QPointF &)

/*

*/
impl /*struct*/ QRadialGradient {
  pub fn setCenter_0<RetType, T: QRadialGradient_setCenter_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCenter_0(self);
    // return 1;
  }
}
pub trait QRadialGradient_setCenter_0<RetType> {
  fn setCenter_0(self , rsthis: & QRadialGradient) -> RetType;
}
impl<'a> /*trait*/ QRadialGradient_setCenter_0<(/*void*/)> for (usize) {
  fn setCenter_0(self , rsthis: & QRadialGradient) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QRadialGradient9setCenterERK7QPointF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qbrush.h:286
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void setCenter(qreal, qreal)

/*

*/
impl /*struct*/ QRadialGradient {
  pub fn setCenter_1<RetType, T: QRadialGradient_setCenter_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCenter_1(self);
    // return 1;
  }
}
pub trait QRadialGradient_setCenter_1<RetType> {
  fn setCenter_1(self , rsthis: & QRadialGradient) -> RetType;
}
impl<'a> /*trait*/ QRadialGradient_setCenter_1<(/*void*/)> for (f64,f64) {
  fn setCenter_1(self , rsthis: & QRadialGradient) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN15QRadialGradient9setCenterEdd", 2,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qbrush.h:288
// index:0
// Public Visibility=Default Availability=Available
// [16] QPointF focalPoint() const

/*

*/
impl /*struct*/ QRadialGradient {
  pub fn focalPoint_0<RetType, T: QRadialGradient_focalPoint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.focalPoint_0(self);
    // return 1;
  }
}
pub trait QRadialGradient_focalPoint_0<RetType> {
  fn focalPoint_0(self , rsthis: & QRadialGradient) -> RetType;
}
impl<'a> /*trait*/ QRadialGradient_focalPoint_0<usize> for () {
  fn focalPoint_0(self , rsthis: & QRadialGradient) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QRadialGradient10focalPointEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qbrush.h:289
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFocalPoint(const QPointF &)

/*

*/
impl /*struct*/ QRadialGradient {
  pub fn setFocalPoint_0<RetType, T: QRadialGradient_setFocalPoint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFocalPoint_0(self);
    // return 1;
  }
}
pub trait QRadialGradient_setFocalPoint_0<RetType> {
  fn setFocalPoint_0(self , rsthis: & QRadialGradient) -> RetType;
}
impl<'a> /*trait*/ QRadialGradient_setFocalPoint_0<(/*void*/)> for (usize) {
  fn setFocalPoint_0(self , rsthis: & QRadialGradient) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QRadialGradient13setFocalPointERK7QPointF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qbrush.h:290
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void setFocalPoint(qreal, qreal)

/*

*/
impl /*struct*/ QRadialGradient {
  pub fn setFocalPoint_1<RetType, T: QRadialGradient_setFocalPoint_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFocalPoint_1(self);
    // return 1;
  }
}
pub trait QRadialGradient_setFocalPoint_1<RetType> {
  fn setFocalPoint_1(self , rsthis: & QRadialGradient) -> RetType;
}
impl<'a> /*trait*/ QRadialGradient_setFocalPoint_1<(/*void*/)> for (f64,f64) {
  fn setFocalPoint_1(self , rsthis: & QRadialGradient) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN15QRadialGradient13setFocalPointEdd", 2,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qbrush.h:292
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal radius() const

/*

*/
impl /*struct*/ QRadialGradient {
  pub fn radius_0<RetType, T: QRadialGradient_radius_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.radius_0(self);
    // return 1;
  }
}
pub trait QRadialGradient_radius_0<RetType> {
  fn radius_0(self , rsthis: & QRadialGradient) -> RetType;
}
impl<'a> /*trait*/ QRadialGradient_radius_0<f64> for () {
  fn radius_0(self , rsthis: & QRadialGradient) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QRadialGradient6radiusEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qbrush.h:293
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setRadius(qreal)

/*

*/
impl /*struct*/ QRadialGradient {
  pub fn setRadius_0<RetType, T: QRadialGradient_setRadius_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setRadius_0(self);
    // return 1;
  }
}
pub trait QRadialGradient_setRadius_0<RetType> {
  fn setRadius_0(self , rsthis: & QRadialGradient) -> RetType;
}
impl<'a> /*trait*/ QRadialGradient_setRadius_0<(/*void*/)> for (f64) {
  fn setRadius_0(self , rsthis: & QRadialGradient) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN15QRadialGradient9setRadiusEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qbrush.h:295
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal centerRadius() const

/*

*/
impl /*struct*/ QRadialGradient {
  pub fn centerRadius_0<RetType, T: QRadialGradient_centerRadius_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.centerRadius_0(self);
    // return 1;
  }
}
pub trait QRadialGradient_centerRadius_0<RetType> {
  fn centerRadius_0(self , rsthis: & QRadialGradient) -> RetType;
}
impl<'a> /*trait*/ QRadialGradient_centerRadius_0<f64> for () {
  fn centerRadius_0(self , rsthis: & QRadialGradient) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QRadialGradient12centerRadiusEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qbrush.h:296
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCenterRadius(qreal)

/*

*/
impl /*struct*/ QRadialGradient {
  pub fn setCenterRadius_0<RetType, T: QRadialGradient_setCenterRadius_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCenterRadius_0(self);
    // return 1;
  }
}
pub trait QRadialGradient_setCenterRadius_0<RetType> {
  fn setCenterRadius_0(self , rsthis: & QRadialGradient) -> RetType;
}
impl<'a> /*trait*/ QRadialGradient_setCenterRadius_0<(/*void*/)> for (f64) {
  fn setCenterRadius_0(self , rsthis: & QRadialGradient) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN15QRadialGradient15setCenterRadiusEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qbrush.h:298
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal focalRadius() const

/*

*/
impl /*struct*/ QRadialGradient {
  pub fn focalRadius_0<RetType, T: QRadialGradient_focalRadius_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.focalRadius_0(self);
    // return 1;
  }
}
pub trait QRadialGradient_focalRadius_0<RetType> {
  fn focalRadius_0(self , rsthis: & QRadialGradient) -> RetType;
}
impl<'a> /*trait*/ QRadialGradient_focalRadius_0<f64> for () {
  fn focalRadius_0(self , rsthis: & QRadialGradient) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QRadialGradient11focalRadiusEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qbrush.h:299
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFocalRadius(qreal)

/*

*/
impl /*struct*/ QRadialGradient {
  pub fn setFocalRadius_0<RetType, T: QRadialGradient_setFocalRadius_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFocalRadius_0(self);
    // return 1;
  }
}
pub trait QRadialGradient_setFocalRadius_0<RetType> {
  fn setFocalRadius_0(self , rsthis: & QRadialGradient) -> RetType;
}
impl<'a> /*trait*/ QRadialGradient_setFocalRadius_0<(/*void*/)> for (f64) {
  fn setFocalRadius_0(self , rsthis: & QRadialGradient) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN15QRadialGradient14setFocalRadiusEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}


pub fn DeleteQRadialGradient(this :*mut QRadialGradient) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN15QRadialGradientD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
