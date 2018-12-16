

// mod ::gui::QLinearGradient
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
// extern C begin: 11
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
#[derive(Default)] // class sizeof(QLinearGradient)=64
pub struct QLinearGradient {
  qbase: QGradient,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QLinearGradient_ITF interface {
//    QGradient_ITF
//    QLinearGradient_PTR() *QLinearGradient
//}
//func (ptr *QLinearGradient) QLinearGradient_PTR() *QLinearGradient { return ptr }

impl /*struct*/ QLinearGradient {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QLinearGradient {
    return QLinearGradient{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QLinearGradient {
//  type Target = QLinearGradientBASE;
//
//  fn deref(&self) -> &QLinearGradientBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QLinearGradientBASE> for QLinearGradient {
//  fn as_ref(& self) -> & QLinearGradientBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qbrush.h:257
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QLinearGradient()

/*

*/
// QLinearGradient() ctx.fn_proto_cpp
impl /*struct*/ QLinearGradient {
  pub fn QLinearGradient_0<T: QLinearGradient_QLinearGradient_0>(value: T) -> QLinearGradient {
    let rsthis = value.QLinearGradient_0();
    return rsthis;
    // return 1;
  }
}

pub trait QLinearGradient_QLinearGradient_0 {
  fn QLinearGradient_0(self) -> QLinearGradient;
}
// QLinearGradient() ctx.fn_proto_cpp
impl<'a> /*trait*/ QLinearGradient_QLinearGradient_0 for () {
  fn QLinearGradient_0(self) -> QLinearGradient {
    // unsafe{_ZN15QLinearGradientC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN15QLinearGradientC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QLinearGradient{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qbrush.h:258
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QLinearGradient(const QPointF &, const QPointF &)

/*

*/
// QLinearGradient(const QPointF &, const QPointF &) ctx.fn_proto_cpp
impl /*struct*/ QLinearGradient {
  pub fn QLinearGradient_1<T: QLinearGradient_QLinearGradient_1>(value: T) -> QLinearGradient {
    let rsthis = value.QLinearGradient_1();
    return rsthis;
    // return 1;
  }
}

pub trait QLinearGradient_QLinearGradient_1 {
  fn QLinearGradient_1(self) -> QLinearGradient;
}
// QLinearGradient(const QPointF &, const QPointF &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QLinearGradient_QLinearGradient_1 for (usize,usize) {
  fn QLinearGradient_1(self) -> QLinearGradient {
    // unsafe{_ZN15QLinearGradientC2ERK7QPointFS2_()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN15QLinearGradientC2ERK7QPointFS2_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QLinearGradient{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qbrush.h:259
// index:2
// Public Visibility=Default Availability=Available
// [-2] void QLinearGradient(qreal, qreal, qreal, qreal)

/*

*/
// QLinearGradient(qreal, qreal, qreal, qreal) ctx.fn_proto_cpp
impl /*struct*/ QLinearGradient {
  pub fn QLinearGradient_2<T: QLinearGradient_QLinearGradient_2>(value: T) -> QLinearGradient {
    let rsthis = value.QLinearGradient_2();
    return rsthis;
    // return 1;
  }
}

pub trait QLinearGradient_QLinearGradient_2 {
  fn QLinearGradient_2(self) -> QLinearGradient;
}
// QLinearGradient(qreal, qreal, qreal, qreal) ctx.fn_proto_cpp
impl<'a> /*trait*/ QLinearGradient_QLinearGradient_2 for (f64,f64,f64,f64) {
  fn QLinearGradient_2(self) -> QLinearGradient {
    // unsafe{_ZN15QLinearGradientC2Edddd()};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let arg2 = (&self.2) as *const f64 as usize;
    let arg3 = (&self.3) as *const f64 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN15QLinearGradientC2Edddd", 4,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QLinearGradient{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qbrush.h:261
// index:0
// Public Visibility=Default Availability=Available
// [16] QPointF start() const

/*

*/
impl /*struct*/ QLinearGradient {
  pub fn start_0<RetType, T: QLinearGradient_start_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.start_0(self);
    // return 1;
  }
}
pub trait QLinearGradient_start_0<RetType> {
  fn start_0(self , rsthis: & QLinearGradient) -> RetType;
}
impl<'a> /*trait*/ QLinearGradient_start_0<usize> for () {
  fn start_0(self , rsthis: & QLinearGradient) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QLinearGradient5startEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qbrush.h:262
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setStart(const QPointF &)

/*

*/
impl /*struct*/ QLinearGradient {
  pub fn setStart_0<RetType, T: QLinearGradient_setStart_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setStart_0(self);
    // return 1;
  }
}
pub trait QLinearGradient_setStart_0<RetType> {
  fn setStart_0(self , rsthis: & QLinearGradient) -> RetType;
}
impl<'a> /*trait*/ QLinearGradient_setStart_0<(/*void*/)> for (usize) {
  fn setStart_0(self , rsthis: & QLinearGradient) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QLinearGradient8setStartERK7QPointF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qbrush.h:263
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void setStart(qreal, qreal)

/*

*/
impl /*struct*/ QLinearGradient {
  pub fn setStart_1<RetType, T: QLinearGradient_setStart_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setStart_1(self);
    // return 1;
  }
}
pub trait QLinearGradient_setStart_1<RetType> {
  fn setStart_1(self , rsthis: & QLinearGradient) -> RetType;
}
impl<'a> /*trait*/ QLinearGradient_setStart_1<(/*void*/)> for (f64,f64) {
  fn setStart_1(self , rsthis: & QLinearGradient) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN15QLinearGradient8setStartEdd", 2,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qbrush.h:265
// index:0
// Public Visibility=Default Availability=Available
// [16] QPointF finalStop() const

/*

*/
impl /*struct*/ QLinearGradient {
  pub fn finalStop_0<RetType, T: QLinearGradient_finalStop_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.finalStop_0(self);
    // return 1;
  }
}
pub trait QLinearGradient_finalStop_0<RetType> {
  fn finalStop_0(self , rsthis: & QLinearGradient) -> RetType;
}
impl<'a> /*trait*/ QLinearGradient_finalStop_0<usize> for () {
  fn finalStop_0(self , rsthis: & QLinearGradient) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QLinearGradient9finalStopEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qbrush.h:266
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFinalStop(const QPointF &)

/*

*/
impl /*struct*/ QLinearGradient {
  pub fn setFinalStop_0<RetType, T: QLinearGradient_setFinalStop_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFinalStop_0(self);
    // return 1;
  }
}
pub trait QLinearGradient_setFinalStop_0<RetType> {
  fn setFinalStop_0(self , rsthis: & QLinearGradient) -> RetType;
}
impl<'a> /*trait*/ QLinearGradient_setFinalStop_0<(/*void*/)> for (usize) {
  fn setFinalStop_0(self , rsthis: & QLinearGradient) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QLinearGradient12setFinalStopERK7QPointF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qbrush.h:267
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void setFinalStop(qreal, qreal)

/*

*/
impl /*struct*/ QLinearGradient {
  pub fn setFinalStop_1<RetType, T: QLinearGradient_setFinalStop_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFinalStop_1(self);
    // return 1;
  }
}
pub trait QLinearGradient_setFinalStop_1<RetType> {
  fn setFinalStop_1(self , rsthis: & QLinearGradient) -> RetType;
}
impl<'a> /*trait*/ QLinearGradient_setFinalStop_1<(/*void*/)> for (f64,f64) {
  fn setFinalStop_1(self , rsthis: & QLinearGradient) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN15QLinearGradient12setFinalStopEdd", 2,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}


pub fn DeleteQLinearGradient(this :*mut QLinearGradient) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN15QLinearGradientD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
