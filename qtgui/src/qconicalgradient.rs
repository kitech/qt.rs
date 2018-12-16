

// mod ::gui::QConicalGradient
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
// extern C begin: 19
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
#[derive(Default)] // class sizeof(QConicalGradient)=64
pub struct QConicalGradient {
  qbase: QGradient,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QConicalGradient_ITF interface {
//    QGradient_ITF
//    QConicalGradient_PTR() *QConicalGradient
//}
//func (ptr *QConicalGradient) QConicalGradient_PTR() *QConicalGradient { return ptr }

impl /*struct*/ QConicalGradient {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QConicalGradient {
    return QConicalGradient{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QConicalGradient {
//  type Target = QConicalGradientBASE;
//
//  fn deref(&self) -> &QConicalGradientBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QConicalGradientBASE> for QConicalGradient {
//  fn as_ref(& self) -> & QConicalGradientBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qbrush.h:306
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QConicalGradient()

/*

*/
// QConicalGradient() ctx.fn_proto_cpp
impl /*struct*/ QConicalGradient {
  pub fn QConicalGradient_0<T: QConicalGradient_QConicalGradient_0>(value: T) -> QConicalGradient {
    let rsthis = value.QConicalGradient_0();
    return rsthis;
    // return 1;
  }
}

pub trait QConicalGradient_QConicalGradient_0 {
  fn QConicalGradient_0(self) -> QConicalGradient;
}
// QConicalGradient() ctx.fn_proto_cpp
impl<'a> /*trait*/ QConicalGradient_QConicalGradient_0 for () {
  fn QConicalGradient_0(self) -> QConicalGradient {
    // unsafe{_ZN16QConicalGradientC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN16QConicalGradientC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QConicalGradient{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qbrush.h:307
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QConicalGradient(const QPointF &, qreal)

/*

*/
// QConicalGradient(const QPointF &, qreal) ctx.fn_proto_cpp
impl /*struct*/ QConicalGradient {
  pub fn QConicalGradient_1<T: QConicalGradient_QConicalGradient_1>(value: T) -> QConicalGradient {
    let rsthis = value.QConicalGradient_1();
    return rsthis;
    // return 1;
  }
}

pub trait QConicalGradient_QConicalGradient_1 {
  fn QConicalGradient_1(self) -> QConicalGradient;
}
// QConicalGradient(const QPointF &, qreal) ctx.fn_proto_cpp
impl<'a> /*trait*/ QConicalGradient_QConicalGradient_1 for (usize,f64) {
  fn QConicalGradient_1(self) -> QConicalGradient {
    // unsafe{_ZN16QConicalGradientC2ERK7QPointFd()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN16QConicalGradientC2ERK7QPointFd", 2,qtrt::FFITY_POINTER,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QConicalGradient{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qbrush.h:308
// index:2
// Public Visibility=Default Availability=Available
// [-2] void QConicalGradient(qreal, qreal, qreal)

/*

*/
// QConicalGradient(qreal, qreal, qreal) ctx.fn_proto_cpp
impl /*struct*/ QConicalGradient {
  pub fn QConicalGradient_2<T: QConicalGradient_QConicalGradient_2>(value: T) -> QConicalGradient {
    let rsthis = value.QConicalGradient_2();
    return rsthis;
    // return 1;
  }
}

pub trait QConicalGradient_QConicalGradient_2 {
  fn QConicalGradient_2(self) -> QConicalGradient;
}
// QConicalGradient(qreal, qreal, qreal) ctx.fn_proto_cpp
impl<'a> /*trait*/ QConicalGradient_QConicalGradient_2 for (f64,f64,f64) {
  fn QConicalGradient_2(self) -> QConicalGradient {
    // unsafe{_ZN16QConicalGradientC2Eddd()};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let arg2 = (&self.2) as *const f64 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN16QConicalGradientC2Eddd", 3,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QConicalGradient{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qbrush.h:310
// index:0
// Public Visibility=Default Availability=Available
// [16] QPointF center() const

/*

*/
impl /*struct*/ QConicalGradient {
  pub fn center_0<RetType, T: QConicalGradient_center_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.center_0(self);
    // return 1;
  }
}
pub trait QConicalGradient_center_0<RetType> {
  fn center_0(self , rsthis: & QConicalGradient) -> RetType;
}
impl<'a> /*trait*/ QConicalGradient_center_0<usize> for () {
  fn center_0(self , rsthis: & QConicalGradient) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QConicalGradient6centerEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qbrush.h:311
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCenter(const QPointF &)

/*

*/
impl /*struct*/ QConicalGradient {
  pub fn setCenter_0<RetType, T: QConicalGradient_setCenter_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCenter_0(self);
    // return 1;
  }
}
pub trait QConicalGradient_setCenter_0<RetType> {
  fn setCenter_0(self , rsthis: & QConicalGradient) -> RetType;
}
impl<'a> /*trait*/ QConicalGradient_setCenter_0<(/*void*/)> for (usize) {
  fn setCenter_0(self , rsthis: & QConicalGradient) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN16QConicalGradient9setCenterERK7QPointF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qbrush.h:312
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void setCenter(qreal, qreal)

/*

*/
impl /*struct*/ QConicalGradient {
  pub fn setCenter_1<RetType, T: QConicalGradient_setCenter_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCenter_1(self);
    // return 1;
  }
}
pub trait QConicalGradient_setCenter_1<RetType> {
  fn setCenter_1(self , rsthis: & QConicalGradient) -> RetType;
}
impl<'a> /*trait*/ QConicalGradient_setCenter_1<(/*void*/)> for (f64,f64) {
  fn setCenter_1(self , rsthis: & QConicalGradient) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN16QConicalGradient9setCenterEdd", 2,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qbrush.h:314
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal angle() const

/*

*/
impl /*struct*/ QConicalGradient {
  pub fn angle_0<RetType, T: QConicalGradient_angle_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.angle_0(self);
    // return 1;
  }
}
pub trait QConicalGradient_angle_0<RetType> {
  fn angle_0(self , rsthis: & QConicalGradient) -> RetType;
}
impl<'a> /*trait*/ QConicalGradient_angle_0<f64> for () {
  fn angle_0(self , rsthis: & QConicalGradient) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QConicalGradient5angleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qbrush.h:315
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setAngle(qreal)

/*

*/
impl /*struct*/ QConicalGradient {
  pub fn setAngle_0<RetType, T: QConicalGradient_setAngle_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setAngle_0(self);
    // return 1;
  }
}
pub trait QConicalGradient_setAngle_0<RetType> {
  fn setAngle_0(self , rsthis: & QConicalGradient) -> RetType;
}
impl<'a> /*trait*/ QConicalGradient_setAngle_0<(/*void*/)> for (f64) {
  fn setAngle_0(self , rsthis: & QConicalGradient) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN16QConicalGradient8setAngleEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}


pub fn DeleteQConicalGradient(this :*mut QConicalGradient) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN16QConicalGradientD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
