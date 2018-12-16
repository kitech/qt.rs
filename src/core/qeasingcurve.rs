

// mod ::core::QEasingCurve
// package qtcore
// /usr/include/qt/QtCore/qeasingcurve.h
// #include <qeasingcurve.h>
// #include <QtCore>

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
use std::default::Default;
use std::ops::Deref;
use super::super::qtrt;
use super::*;
//  ext block end

//  body block begin



/*

*/
#[derive(Default)] // class sizeof(QEasingCurve)=8
pub struct QEasingCurve {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QEasingCurve_ITF interface {
//    QEasingCurve_PTR() *QEasingCurve
//}
//func (ptr *QEasingCurve) QEasingCurve_PTR() *QEasingCurve { return ptr }

impl /*struct*/ QEasingCurve {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QEasingCurve {
    return QEasingCurve{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QEasingCurve {
//  type Target = QEasingCurveBASE;
//
//  fn deref(&self) -> &QEasingCurveBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QEasingCurveBASE> for QEasingCurve {
//  fn as_ref(& self) -> & QEasingCurveBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qeasingcurve.h:77
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QEasingCurve(QEasingCurve::Type)

/*
Constructs an easing curve of the given type.
*/
// QEasingCurve(QEasingCurve::Type) ctx.fn_proto_cpp
impl /*struct*/ QEasingCurve {
  pub fn QEasingCurve_0<T: QEasingCurve_QEasingCurve_0>(value: T) -> QEasingCurve {
    let rsthis = value.QEasingCurve_0();
    return rsthis;
    // return 1;
  }
}

pub trait QEasingCurve_QEasingCurve_0 {
  fn QEasingCurve_0(self) -> QEasingCurve;
}
// QEasingCurve(QEasingCurve::Type) ctx.fn_proto_cpp
impl<'a> /*trait*/ QEasingCurve_QEasingCurve_0 for (i32) {
  fn QEasingCurve_0(self) -> QEasingCurve {
    // unsafe{_ZN12QEasingCurveC2ENS_4TypeE()};
    let arg0 = (&self) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN12QEasingCurveC2ENS_4TypeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QEasingCurve{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qeasingcurve.h:79
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ~QEasingCurve()

/*

*/
pub fn DeleteQEasingCurve(this :*mut QEasingCurve) {
    // let rv = qtrt::InvokeQtFunc6("_ZN12QEasingCurveD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 8)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qeasingcurve.h:81
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QEasingCurve & operator=(const QEasingCurve &)

/*

*/
impl /*struct*/ QEasingCurve {
  pub fn operator_equal_0<RetType, T: QEasingCurve_operator_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_0(self);
    // return 1;
  }
}
pub trait QEasingCurve_operator_equal_0<RetType> {
  fn operator_equal_0(self , rsthis: & QEasingCurve) -> RetType;
}
impl<'a> /*trait*/ QEasingCurve_operator_equal_0<usize> for (usize) {
  fn operator_equal_0(self , rsthis: & QEasingCurve) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN12QEasingCurveaSERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qeasingcurve.h:85
// index:1
// Public inline Visibility=Default Availability=Available
// [8] QEasingCurve & operator=(QEasingCurve &&)

/*

*/
impl /*struct*/ QEasingCurve {
  pub fn operator_equal_1<RetType, T: QEasingCurve_operator_equal_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_1(self);
    // return 1;
  }
}
pub trait QEasingCurve_operator_equal_1<RetType> {
  fn operator_equal_1(self , rsthis: & QEasingCurve) -> RetType;
}
impl<'a> /*trait*/ QEasingCurve_operator_equal_1<usize> for (usize) {
  fn operator_equal_1(self , rsthis: & QEasingCurve) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN12QEasingCurveaSEOS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qeasingcurve.h:89
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void swap(QEasingCurve &)

/*
Swaps curve other with this curve. This operation is very fast and never fails.

This function was introduced in  Qt 5.0.
*/
impl /*struct*/ QEasingCurve {
  pub fn swap_0<RetType, T: QEasingCurve_swap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.swap_0(self);
    // return 1;
  }
}
pub trait QEasingCurve_swap_0<RetType> {
  fn swap_0(self , rsthis: & QEasingCurve) -> RetType;
}
impl<'a> /*trait*/ QEasingCurve_swap_0<(/*void*/)> for (usize) {
  fn swap_0(self , rsthis: & QEasingCurve) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QEasingCurve4swapERS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qeasingcurve.h:91
// index:0
// Public Visibility=Default Availability=Available
// [1] bool operator==(const QEasingCurve &) const

/*

*/
impl /*struct*/ QEasingCurve {
  pub fn operator_equal_equal_0<RetType, T: QEasingCurve_operator_equal_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_equal_0(self);
    // return 1;
  }
}
pub trait QEasingCurve_operator_equal_equal_0<RetType> {
  fn operator_equal_equal_0(self , rsthis: & QEasingCurve) -> RetType;
}
impl<'a> /*trait*/ QEasingCurve_operator_equal_equal_0<bool> for (usize) {
  fn operator_equal_equal_0(self , rsthis: & QEasingCurve) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QEasingCurveeqERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qeasingcurve.h:92
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator!=(const QEasingCurve &) const

/*

*/
impl /*struct*/ QEasingCurve {
  pub fn operator_not_equal_0<RetType, T: QEasingCurve_operator_not_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_not_equal_0(self);
    // return 1;
  }
}
pub trait QEasingCurve_operator_not_equal_0<RetType> {
  fn operator_not_equal_0(self , rsthis: & QEasingCurve) -> RetType;
}
impl<'a> /*trait*/ QEasingCurve_operator_not_equal_0<bool> for (usize) {
  fn operator_not_equal_0(self , rsthis: & QEasingCurve) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QEasingCurveneERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qeasingcurve.h:95
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal amplitude() const

/*
Returns the amplitude. This is not applicable for all curve types. It is only applicable for bounce and elastic curves (curves of type() QEasingCurve::InBounce, QEasingCurve::OutBounce, QEasingCurve::InOutBounce, QEasingCurve::OutInBounce, QEasingCurve::InElastic, QEasingCurve::OutElastic, QEasingCurve::InOutElastic or QEasingCurve::OutInElastic).

See also setAmplitude().
*/
impl /*struct*/ QEasingCurve {
  pub fn amplitude_0<RetType, T: QEasingCurve_amplitude_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.amplitude_0(self);
    // return 1;
  }
}
pub trait QEasingCurve_amplitude_0<RetType> {
  fn amplitude_0(self , rsthis: & QEasingCurve) -> RetType;
}
impl<'a> /*trait*/ QEasingCurve_amplitude_0<f64> for () {
  fn amplitude_0(self , rsthis: & QEasingCurve) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QEasingCurve9amplitudeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qeasingcurve.h:96
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setAmplitude(qreal)

/*
Sets the amplitude to amplitude.

This will set the amplitude of the bounce or the amplitude of the elastic "spring" effect. The higher the number, the higher the amplitude.

See also amplitude().
*/
impl /*struct*/ QEasingCurve {
  pub fn setAmplitude_0<RetType, T: QEasingCurve_setAmplitude_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setAmplitude_0(self);
    // return 1;
  }
}
pub trait QEasingCurve_setAmplitude_0<RetType> {
  fn setAmplitude_0(self , rsthis: & QEasingCurve) -> RetType;
}
impl<'a> /*trait*/ QEasingCurve_setAmplitude_0<(/*void*/)> for (f64) {
  fn setAmplitude_0(self , rsthis: & QEasingCurve) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN12QEasingCurve12setAmplitudeEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qeasingcurve.h:98
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal period() const

/*
Returns the period. This is not applicable for all curve types. It is only applicable if type() is QEasingCurve::InElastic, QEasingCurve::OutElastic, QEasingCurve::InOutElastic or QEasingCurve::OutInElastic.

See also setPeriod().
*/
impl /*struct*/ QEasingCurve {
  pub fn period_0<RetType, T: QEasingCurve_period_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.period_0(self);
    // return 1;
  }
}
pub trait QEasingCurve_period_0<RetType> {
  fn period_0(self , rsthis: & QEasingCurve) -> RetType;
}
impl<'a> /*trait*/ QEasingCurve_period_0<f64> for () {
  fn period_0(self , rsthis: & QEasingCurve) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QEasingCurve6periodEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qeasingcurve.h:99
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setPeriod(qreal)

/*
Sets the period to period. Setting a small period value will give a high frequency of the curve. A large period will give it a small frequency.

See also period().
*/
impl /*struct*/ QEasingCurve {
  pub fn setPeriod_0<RetType, T: QEasingCurve_setPeriod_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPeriod_0(self);
    // return 1;
  }
}
pub trait QEasingCurve_setPeriod_0<RetType> {
  fn setPeriod_0(self , rsthis: & QEasingCurve) -> RetType;
}
impl<'a> /*trait*/ QEasingCurve_setPeriod_0<(/*void*/)> for (f64) {
  fn setPeriod_0(self , rsthis: & QEasingCurve) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN12QEasingCurve9setPeriodEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qeasingcurve.h:101
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal overshoot() const

/*
Returns the overshoot. This is not applicable for all curve types. It is only applicable if type() is QEasingCurve::InBack, QEasingCurve::OutBack, QEasingCurve::InOutBack or QEasingCurve::OutInBack.

See also setOvershoot().
*/
impl /*struct*/ QEasingCurve {
  pub fn overshoot_0<RetType, T: QEasingCurve_overshoot_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.overshoot_0(self);
    // return 1;
  }
}
pub trait QEasingCurve_overshoot_0<RetType> {
  fn overshoot_0(self , rsthis: & QEasingCurve) -> RetType;
}
impl<'a> /*trait*/ QEasingCurve_overshoot_0<f64> for () {
  fn overshoot_0(self , rsthis: & QEasingCurve) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QEasingCurve9overshootEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qeasingcurve.h:102
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setOvershoot(qreal)

/*
Sets the overshoot to overshoot.

0 produces no overshoot, and the default value of 1.70158 produces an overshoot of 10 percent.

See also overshoot().
*/
impl /*struct*/ QEasingCurve {
  pub fn setOvershoot_0<RetType, T: QEasingCurve_setOvershoot_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setOvershoot_0(self);
    // return 1;
  }
}
pub trait QEasingCurve_setOvershoot_0<RetType> {
  fn setOvershoot_0(self , rsthis: & QEasingCurve) -> RetType;
}
impl<'a> /*trait*/ QEasingCurve_setOvershoot_0<(/*void*/)> for (f64) {
  fn setOvershoot_0(self , rsthis: & QEasingCurve) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN12QEasingCurve12setOvershootEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qeasingcurve.h:104
// index:0
// Public Visibility=Default Availability=Available
// [-2] void addCubicBezierSegment(const QPointF &, const QPointF &, const QPointF &)

/*
Adds a segment of a cubic bezier spline to define a custom easing curve. It is only applicable if type() is QEasingCurve::BezierSpline. Note that the spline implicitly starts at (0.0, 0.0) and has to end at (1.0, 1.0) to be a valid easing curve. c1 and c2 are the control points used for drawing the curve. endPoint is the endpoint of the curve.
*/
impl /*struct*/ QEasingCurve {
  pub fn addCubicBezierSegment_0<RetType, T: QEasingCurve_addCubicBezierSegment_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addCubicBezierSegment_0(self);
    // return 1;
  }
}
pub trait QEasingCurve_addCubicBezierSegment_0<RetType> {
  fn addCubicBezierSegment_0(self , rsthis: & QEasingCurve) -> RetType;
}
impl<'a> /*trait*/ QEasingCurve_addCubicBezierSegment_0<(/*void*/)> for (usize,usize,usize) {
  fn addCubicBezierSegment_0(self , rsthis: & QEasingCurve) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QEasingCurve21addCubicBezierSegmentERK7QPointFS2_S2_", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qeasingcurve.h:105
// index:0
// Public Visibility=Default Availability=Available
// [-2] void addTCBSegment(const QPointF &, qreal, qreal, qreal)

/*
Adds a segment of a TCB bezier spline to define a custom easing curve. It is only applicable if type() is QEasingCurve::TCBSpline. The spline has to start explitly at (0.0, 0.0) and has to end at (1.0, 1.0) to be a valid easing curve. The tension t changes the length of the tangent vector. The continuity c changes the sharpness in change between the tangents. The bias b changes the direction of the tangent vector. nextPoint is the sample position. All three parameters are valid between -1 and 1 and define the tangent of the control point. If all three parameters are 0 the resulting spline is a Catmull-Rom spline. The begin and endpoint always have a bias of -1 and 1, since the outer tangent is not defined.
*/
impl /*struct*/ QEasingCurve {
  pub fn addTCBSegment_0<RetType, T: QEasingCurve_addTCBSegment_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addTCBSegment_0(self);
    // return 1;
  }
}
pub trait QEasingCurve_addTCBSegment_0<RetType> {
  fn addTCBSegment_0(self , rsthis: & QEasingCurve) -> RetType;
}
impl<'a> /*trait*/ QEasingCurve_addTCBSegment_0<(/*void*/)> for (usize,f64,f64,f64) {
  fn addTCBSegment_0(self , rsthis: & QEasingCurve) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let arg2 = (&self.2) as *const f64 as usize;
    let arg3 = (&self.3) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN12QEasingCurve13addTCBSegmentERK7QPointFddd", 4,qtrt::FFITY_POINTER,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qeasingcurve.h:111
// index:0
// Public Visibility=Default Availability=Available
// [4] QEasingCurve::Type type() const

/*
Returns the type of the easing curve.

See also setType().
*/
impl /*struct*/ QEasingCurve {
  pub fn type__0<RetType, T: QEasingCurve_type__0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.type__0(self);
    // return 1;
  }
}
pub trait QEasingCurve_type__0<RetType> {
  fn type__0(self , rsthis: & QEasingCurve) -> RetType;
}
impl<'a> /*trait*/ QEasingCurve_type__0<i32> for () {
  fn type__0(self , rsthis: & QEasingCurve) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QEasingCurve4typeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qeasingcurve.h:112
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setType(QEasingCurve::Type)

/*
Sets the type of the easing curve to type.

See also type().
*/
impl /*struct*/ QEasingCurve {
  pub fn setType_0<RetType, T: QEasingCurve_setType_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setType_0(self);
    // return 1;
  }
}
pub trait QEasingCurve_setType_0<RetType> {
  fn setType_0(self , rsthis: & QEasingCurve) -> RetType;
}
impl<'a> /*trait*/ QEasingCurve_setType_0<(/*void*/)> for (i32) {
  fn setType_0(self , rsthis: & QEasingCurve) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN12QEasingCurve7setTypeENS_4TypeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qeasingcurve.h:115
// index:0
// Public Visibility=Default Availability=Available
// [8] QEasingCurve::EasingFunction customType() const

/*
Returns the function pointer to the custom easing curve. If type() does not return QEasingCurve::Custom, this function will return 0.

See also setCustomType().
*/
impl /*struct*/ QEasingCurve {
  pub fn customType_0<RetType, T: QEasingCurve_customType_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.customType_0(self);
    // return 1;
  }
}
pub trait QEasingCurve_customType_0<RetType> {
  fn customType_0(self , rsthis: & QEasingCurve) -> RetType;
}
impl<'a> /*trait*/ QEasingCurve_customType_0<usize> for () {
  fn customType_0(self , rsthis: & QEasingCurve) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QEasingCurve10customTypeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qeasingcurve.h:117
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal valueForProgress(qreal) const

/*
Return the effective progress for the easing curve at progress. Whereas progress must be between 0 and 1, the returned effective progress can be outside those bounds. For example, QEasingCurve::InBack will return negative values in the beginning of the function.
*/
impl /*struct*/ QEasingCurve {
  pub fn valueForProgress_0<RetType, T: QEasingCurve_valueForProgress_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.valueForProgress_0(self);
    // return 1;
  }
}
pub trait QEasingCurve_valueForProgress_0<RetType> {
  fn valueForProgress_0(self , rsthis: & QEasingCurve) -> RetType;
}
impl<'a> /*trait*/ QEasingCurve_valueForProgress_0<f64> for (f64) {
  fn valueForProgress_0(self , rsthis: & QEasingCurve) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QEasingCurve16valueForProgressEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}


/*
The type of easing curve.

ConstantValue
QEasingCurve::Linear0




Easing curve for a linear (t) function: velocity is constant.

ConstantValue
QEasingCurve::InQuad1




Easing curve for a quadratic (t^2) function: accelerating from zero velocity.

ConstantValue
QEasingCurve::OutQuad2




Easing curve for a quadratic (t^2) function: decelerating to zero velocity.

ConstantValue
QEasingCurve::InOutQuad3




Easing curve for a quadratic (t^2) function: acceleration until halfway, then deceleration.

ConstantValue
QEasingCurve::OutInQuad4




Easing curve for a quadratic (t^2) function: deceleration until halfway, then acceleration.

ConstantValue
QEasingCurve::InCubic5




Easing curve for a cubic (t^3) function: accelerating from zero velocity.

ConstantValue
QEasingCurve::OutCubic6




Easing curve for a cubic (t^3) function: decelerating to zero velocity.

ConstantValue
QEasingCurve::InOutCubic7




Easing curve for a cubic (t^3) function: acceleration until halfway, then deceleration.

ConstantValue
QEasingCurve::OutInCubic8




Easing curve for a cubic (t^3) function: deceleration until halfway, then acceleration.

ConstantValue
QEasingCurve::InQuart9




Easing curve for a quartic (t^4) function: accelerating from zero velocity.

ConstantValue




Easing curve for a quartic (t^4) function: decelerating to zero velocity.

ConstantValue




Easing curve for a quartic (t^4) function: acceleration until halfway, then deceleration.

ConstantValue




Easing curve for a quartic (t^4) function: deceleration until halfway, then acceleration.

ConstantValue




Easing curve for a quintic (t^5) easing in: accelerating from zero velocity.

ConstantValue




Easing curve for a quintic (t^5) function: decelerating to zero velocity.

ConstantValue




Easing curve for a quintic (t^5) function: acceleration until halfway, then deceleration.

ConstantValue




Easing curve for a quintic (t^5) function: deceleration until halfway, then acceleration.

ConstantValue




Easing curve for a sinusoidal (sin(t)) function: accelerating from zero velocity.

ConstantValue




Easing curve for a sinusoidal (sin(t)) function: decelerating to zero velocity.

ConstantValue




Easing curve for a sinusoidal (sin(t)) function: acceleration until halfway, then deceleration.

ConstantValue




Easing curve for a sinusoidal (sin(t)) function: deceleration until halfway, then acceleration.

ConstantValue




Easing curve for an exponential (2^t) function: accelerating from zero velocity.

ConstantValue




Easing curve for an exponential (2^t) function: decelerating to zero velocity.

ConstantValue




Easing curve for an exponential (2^t) function: acceleration until halfway, then deceleration.

ConstantValue




Easing curve for an exponential (2^t) function: deceleration until halfway, then acceleration.

ConstantValue




Easing curve for a circular (sqrt(1-t^2)) function: accelerating from zero velocity.

ConstantValue




Easing curve for a circular (sqrt(1-t^2)) function: decelerating to zero velocity.

ConstantValue




Easing curve for a circular (sqrt(1-t^2)) function: acceleration until halfway, then deceleration.

ConstantValue




Easing curve for a circular (sqrt(1-t^2)) function: deceleration until halfway, then acceleration.

ConstantValue




Easing curve for an elastic (exponentially decaying sine wave) function: accelerating from zero velocity. The peak amplitude can be set with the amplitude parameter, and the period of decay by the period parameter.

ConstantValue




Easing curve for an elastic (exponentially decaying sine wave) function: decelerating to zero velocity. The peak amplitude can be set with the amplitude parameter, and the period of decay by the period parameter.

ConstantValue




Easing curve for an elastic (exponentially decaying sine wave) function: acceleration until halfway, then deceleration.

ConstantValue




Easing curve for an elastic (exponentially decaying sine wave) function: deceleration until halfway, then acceleration.

ConstantValue




Easing curve for a back (overshooting cubic function: (s+1)*t^3 - s*t^2) easing in: accelerating from zero velocity.

ConstantValue




Easing curve for a back (overshooting cubic function: (s+1)*t^3 - s*t^2) easing out: decelerating to zero velocity.

ConstantValue




Easing curve for a back (overshooting cubic function: (s+1)*t^3 - s*t^2) easing in/out: acceleration until halfway, then deceleration.

ConstantValue




Easing curve for a back (overshooting cubic easing: (s+1)*t^3 - s*t^2) easing out/in: deceleration until halfway, then acceleration.

ConstantValue




Easing curve for a bounce (exponentially decaying parabolic bounce) function: accelerating from zero velocity.

ConstantValue




Easing curve for a bounce (exponentially decaying parabolic bounce) function: decelerating from zero velocity.

ConstantValue




Easing curve for a bounce (exponentially decaying parabolic bounce) function easing in/out: acceleration until halfway, then deceleration.

ConstantValue




Easing curve for a bounce (exponentially decaying parabolic bounce) function easing out/in: deceleration until halfway, then acceleration.



See also addCubicBezierSegment() and addTCBSegment().

*/
pub type QEasingCurve__Type = i32;
// 
pub const QEasingCurve__Linear :QEasingCurve__Type = 0;
// 
pub const QEasingCurve__InQuad :QEasingCurve__Type = 1;
// 
pub const QEasingCurve__OutQuad :QEasingCurve__Type = 2;
// 
pub const QEasingCurve__InOutQuad :QEasingCurve__Type = 3;
// 
pub const QEasingCurve__OutInQuad :QEasingCurve__Type = 4;
// 
pub const QEasingCurve__InCubic :QEasingCurve__Type = 5;
// 
pub const QEasingCurve__OutCubic :QEasingCurve__Type = 6;
// 
pub const QEasingCurve__InOutCubic :QEasingCurve__Type = 7;
// 
pub const QEasingCurve__OutInCubic :QEasingCurve__Type = 8;
// 
pub const QEasingCurve__InQuart :QEasingCurve__Type = 9;
// 0
pub const QEasingCurve__OutQuart :QEasingCurve__Type = 10;
// 1
pub const QEasingCurve__InOutQuart :QEasingCurve__Type = 11;
// 2
pub const QEasingCurve__OutInQuart :QEasingCurve__Type = 12;
// 3
pub const QEasingCurve__InQuint :QEasingCurve__Type = 13;
// 4
pub const QEasingCurve__OutQuint :QEasingCurve__Type = 14;
// 5
pub const QEasingCurve__InOutQuint :QEasingCurve__Type = 15;
// 6
pub const QEasingCurve__OutInQuint :QEasingCurve__Type = 16;
// 7
pub const QEasingCurve__InSine :QEasingCurve__Type = 17;
// 8
pub const QEasingCurve__OutSine :QEasingCurve__Type = 18;
// 9
pub const QEasingCurve__InOutSine :QEasingCurve__Type = 19;
// 0
pub const QEasingCurve__OutInSine :QEasingCurve__Type = 20;
// 1
pub const QEasingCurve__InExpo :QEasingCurve__Type = 21;
// 2
pub const QEasingCurve__OutExpo :QEasingCurve__Type = 22;
// 3
pub const QEasingCurve__InOutExpo :QEasingCurve__Type = 23;
// 4
pub const QEasingCurve__OutInExpo :QEasingCurve__Type = 24;
// 5
pub const QEasingCurve__InCirc :QEasingCurve__Type = 25;
// 6
pub const QEasingCurve__OutCirc :QEasingCurve__Type = 26;
// 7
pub const QEasingCurve__InOutCirc :QEasingCurve__Type = 27;
// 8
pub const QEasingCurve__OutInCirc :QEasingCurve__Type = 28;
// 9
pub const QEasingCurve__InElastic :QEasingCurve__Type = 29;
// 0
pub const QEasingCurve__OutElastic :QEasingCurve__Type = 30;
// 1
pub const QEasingCurve__InOutElastic :QEasingCurve__Type = 31;
// 2
pub const QEasingCurve__OutInElastic :QEasingCurve__Type = 32;
// 3
pub const QEasingCurve__InBack :QEasingCurve__Type = 33;
// 4
pub const QEasingCurve__OutBack :QEasingCurve__Type = 34;
// 5
pub const QEasingCurve__InOutBack :QEasingCurve__Type = 35;
// 6
pub const QEasingCurve__OutInBack :QEasingCurve__Type = 36;
// 7
pub const QEasingCurve__InBounce :QEasingCurve__Type = 37;
// 8
pub const QEasingCurve__OutBounce :QEasingCurve__Type = 38;
// 9
pub const QEasingCurve__InOutBounce :QEasingCurve__Type = 39;
// 0
pub const QEasingCurve__OutInBounce :QEasingCurve__Type = 40;
// 
pub const QEasingCurve__InCurve :QEasingCurve__Type = 41;
// 
pub const QEasingCurve__OutCurve :QEasingCurve__Type = 42;
// 
pub const QEasingCurve__SineCurve :QEasingCurve__Type = 43;
// 
pub const QEasingCurve__CosineCurve :QEasingCurve__Type = 44;
// 
pub const QEasingCurve__BezierSpline :QEasingCurve__Type = 45;
// 
pub const QEasingCurve__TCBSpline :QEasingCurve__Type = 46;
// 
pub const QEasingCurve__Custom :QEasingCurve__Type = 47;
// 
pub const QEasingCurve__NCurveTypes :QEasingCurve__Type = 48;
pub fn QEasingCurve_TypeItemName(val: i32) ->String {
  match val {
     QEasingCurve__Linear => // 0
     {return String::from("Linear");}
     QEasingCurve__InQuad => // 1
     {return String::from("InQuad");}
     QEasingCurve__OutQuad => // 2
     {return String::from("OutQuad");}
     QEasingCurve__InOutQuad => // 3
     {return String::from("InOutQuad");}
     QEasingCurve__OutInQuad => // 4
     {return String::from("OutInQuad");}
     QEasingCurve__InCubic => // 5
     {return String::from("InCubic");}
     QEasingCurve__OutCubic => // 6
     {return String::from("OutCubic");}
     QEasingCurve__InOutCubic => // 7
     {return String::from("InOutCubic");}
     QEasingCurve__OutInCubic => // 8
     {return String::from("OutInCubic");}
     QEasingCurve__InQuart => // 9
     {return String::from("InQuart");}
     QEasingCurve__OutQuart => // 10
     {return String::from("OutQuart");}
     QEasingCurve__InOutQuart => // 11
     {return String::from("InOutQuart");}
     QEasingCurve__OutInQuart => // 12
     {return String::from("OutInQuart");}
     QEasingCurve__InQuint => // 13
     {return String::from("InQuint");}
     QEasingCurve__OutQuint => // 14
     {return String::from("OutQuint");}
     QEasingCurve__InOutQuint => // 15
     {return String::from("InOutQuint");}
     QEasingCurve__OutInQuint => // 16
     {return String::from("OutInQuint");}
     QEasingCurve__InSine => // 17
     {return String::from("InSine");}
     QEasingCurve__OutSine => // 18
     {return String::from("OutSine");}
     QEasingCurve__InOutSine => // 19
     {return String::from("InOutSine");}
     QEasingCurve__OutInSine => // 20
     {return String::from("OutInSine");}
     QEasingCurve__InExpo => // 21
     {return String::from("InExpo");}
     QEasingCurve__OutExpo => // 22
     {return String::from("OutExpo");}
     QEasingCurve__InOutExpo => // 23
     {return String::from("InOutExpo");}
     QEasingCurve__OutInExpo => // 24
     {return String::from("OutInExpo");}
     QEasingCurve__InCirc => // 25
     {return String::from("InCirc");}
     QEasingCurve__OutCirc => // 26
     {return String::from("OutCirc");}
     QEasingCurve__InOutCirc => // 27
     {return String::from("InOutCirc");}
     QEasingCurve__OutInCirc => // 28
     {return String::from("OutInCirc");}
     QEasingCurve__InElastic => // 29
     {return String::from("InElastic");}
     QEasingCurve__OutElastic => // 30
     {return String::from("OutElastic");}
     QEasingCurve__InOutElastic => // 31
     {return String::from("InOutElastic");}
     QEasingCurve__OutInElastic => // 32
     {return String::from("OutInElastic");}
     QEasingCurve__InBack => // 33
     {return String::from("InBack");}
     QEasingCurve__OutBack => // 34
     {return String::from("OutBack");}
     QEasingCurve__InOutBack => // 35
     {return String::from("InOutBack");}
     QEasingCurve__OutInBack => // 36
     {return String::from("OutInBack");}
     QEasingCurve__InBounce => // 37
     {return String::from("InBounce");}
     QEasingCurve__OutBounce => // 38
     {return String::from("OutBounce");}
     QEasingCurve__InOutBounce => // 39
     {return String::from("InOutBounce");}
     QEasingCurve__OutInBounce => // 40
     {return String::from("OutInBounce");}
     QEasingCurve__InCurve => // 41
     {return String::from("InCurve");}
     QEasingCurve__OutCurve => // 42
     {return String::from("OutCurve");}
     QEasingCurve__SineCurve => // 43
     {return String::from("SineCurve");}
     QEasingCurve__CosineCurve => // 44
     {return String::from("CosineCurve");}
     QEasingCurve__BezierSpline => // 45
     {return String::from("BezierSpline");}
     QEasingCurve__TCBSpline => // 46
     {return String::from("TCBSpline");}
     QEasingCurve__Custom => // 47
     {return String::from("Custom");}
     QEasingCurve__NCurveTypes => // 48
     {return String::from("NCurveTypes");}
  _ => {return format!("{}", val);}
}
}
pub fn QEasingCurve_TypeItemName_s(val: i32) ->String {
  //var nilthis *QEasingCurve
  //return nilthis.TypeItemName(val);
  return QEasingCurve_TypeItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
