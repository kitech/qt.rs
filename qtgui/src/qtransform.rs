

// mod ::gui::QTransform
// package qtgui
// /usr/include/qt/QtGui/qtransform.h
// #include <qtransform.h>
// #include <QtGui>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 17
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
#[derive(Default)] // class sizeof(QTransform)=88
pub struct QTransform {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QTransform_ITF interface {
//    QTransform_PTR() *QTransform
//}
//func (ptr *QTransform) QTransform_PTR() *QTransform { return ptr }

impl /*struct*/ QTransform {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QTransform {
    return QTransform{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QTransform {
//  type Target = QTransformBASE;
//
//  fn deref(&self) -> &QTransformBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QTransformBASE> for QTransform {
//  fn as_ref(& self) -> & QTransformBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qtransform.h:69
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void QTransform(Qt::Initialization)

/*
Constructs an identity matrix.

All elements are set to zero except m11 and m22 (specifying the scale) and m33 which are set to 1.

See also reset().
*/
// QTransform(Qt::Initialization) ctx.fn_proto_cpp
impl /*struct*/ QTransform {
  pub fn QTransform_0<T: QTransform_QTransform_0>(value: T) -> QTransform {
    let rsthis = value.QTransform_0();
    return rsthis;
    // return 1;
  }
}

pub trait QTransform_QTransform_0 {
  fn QTransform_0(self) -> QTransform;
}
// QTransform(Qt::Initialization) ctx.fn_proto_cpp
impl<'a> /*trait*/ QTransform_QTransform_0 for (i32) {
  fn QTransform_0(self) -> QTransform {
    // unsafe{_ZN10QTransformC2EN2Qt14InitializationE()};
    let arg0 = (&self) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN10QTransformC2EN2Qt14InitializationE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTransform{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtransform.h:70
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QTransform()

/*
Constructs an identity matrix.

All elements are set to zero except m11 and m22 (specifying the scale) and m33 which are set to 1.

See also reset().
*/
// QTransform() ctx.fn_proto_cpp
impl /*struct*/ QTransform {
  pub fn QTransform_1<T: QTransform_QTransform_1>(value: T) -> QTransform {
    let rsthis = value.QTransform_1();
    return rsthis;
    // return 1;
  }
}

pub trait QTransform_QTransform_1 {
  fn QTransform_1(self) -> QTransform;
}
// QTransform() ctx.fn_proto_cpp
impl<'a> /*trait*/ QTransform_QTransform_1 for () {
  fn QTransform_1(self) -> QTransform {
    // unsafe{_ZN10QTransformC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN10QTransformC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTransform{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtransform.h:71
// index:2
// Public Visibility=Default Availability=Available
// [-2] void QTransform(qreal, qreal, qreal, qreal, qreal, qreal, qreal, qreal, qreal)

/*
Constructs an identity matrix.

All elements are set to zero except m11 and m22 (specifying the scale) and m33 which are set to 1.

See also reset().
*/
// QTransform(qreal, qreal, qreal, qreal, qreal, qreal, qreal, qreal, qreal) ctx.fn_proto_cpp
impl /*struct*/ QTransform {
  pub fn QTransform_2<T: QTransform_QTransform_2>(value: T) -> QTransform {
    let rsthis = value.QTransform_2();
    return rsthis;
    // return 1;
  }
}

pub trait QTransform_QTransform_2 {
  fn QTransform_2(self) -> QTransform;
}
// QTransform(qreal, qreal, qreal, qreal, qreal, qreal, qreal, qreal, qreal) ctx.fn_proto_cpp
impl<'a> /*trait*/ QTransform_QTransform_2 for (f64,f64,f64,f64,f64,f64,f64,f64,f64) {
  fn QTransform_2(self) -> QTransform {
    // unsafe{_ZN10QTransformC2Eddddddddd()};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let arg2 = (&self.2) as *const f64 as usize;
    let arg3 = (&self.3) as *const f64 as usize;
    let arg4 = (&self.4) as *const f64 as usize;
    let arg5 = (&self.5) as *const f64 as usize;
    let arg6 = (&self.6) as *const f64 as usize;
    let arg7 = (&self.7) as *const f64 as usize;
    let arg8 = (&self.8) as *const f64 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN10QTransformC2Eddddddddd", 9,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,arg5,arg6,arg7,arg8,0,0,0,0,0,0,0);
    let rsthis = QTransform{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtransform.h:74
// index:3
// Public Visibility=Default Availability=Available
// [-2] void QTransform(qreal, qreal, qreal, qreal, qreal, qreal)

/*
Constructs an identity matrix.

All elements are set to zero except m11 and m22 (specifying the scale) and m33 which are set to 1.

See also reset().
*/
// QTransform(qreal, qreal, qreal, qreal, qreal, qreal) ctx.fn_proto_cpp
impl /*struct*/ QTransform {
  pub fn QTransform_3<T: QTransform_QTransform_3>(value: T) -> QTransform {
    let rsthis = value.QTransform_3();
    return rsthis;
    // return 1;
  }
}

pub trait QTransform_QTransform_3 {
  fn QTransform_3(self) -> QTransform;
}
// QTransform(qreal, qreal, qreal, qreal, qreal, qreal) ctx.fn_proto_cpp
impl<'a> /*trait*/ QTransform_QTransform_3 for (f64,f64,f64,f64,f64,f64) {
  fn QTransform_3(self) -> QTransform {
    // unsafe{_ZN10QTransformC2Edddddd()};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let arg2 = (&self.2) as *const f64 as usize;
    let arg3 = (&self.3) as *const f64 as usize;
    let arg4 = (&self.4) as *const f64 as usize;
    let arg5 = (&self.5) as *const f64 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN10QTransformC2Edddddd", 6,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,arg5,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTransform{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtransform.h:76
// index:4
// Public Visibility=Default Availability=Available
// [-2] void QTransform(const QMatrix &)

/*
Constructs an identity matrix.

All elements are set to zero except m11 and m22 (specifying the scale) and m33 which are set to 1.

See also reset().
*/
// QTransform(const QMatrix &) ctx.fn_proto_cpp
impl /*struct*/ QTransform {
  pub fn QTransform_4<T: QTransform_QTransform_4>(value: T) -> QTransform {
    let rsthis = value.QTransform_4();
    return rsthis;
    // return 1;
  }
}

pub trait QTransform_QTransform_4 {
  fn QTransform_4(self) -> QTransform;
}
// QTransform(const QMatrix &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QTransform_QTransform_4 for (usize) {
  fn QTransform_4(self) -> QTransform {
    // unsafe{_ZN10QTransformC2ERK7QMatrix()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN10QTransformC2ERK7QMatrix", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTransform{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtransform.h:80
// index:0
// Public inline Visibility=Default Availability=Available
// [88] QTransform & operator=(QTransform &&)

/*

*/
impl /*struct*/ QTransform {
  pub fn operator_equal_0<RetType, T: QTransform_operator_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_0(self);
    // return 1;
  }
}
pub trait QTransform_operator_equal_0<RetType> {
  fn operator_equal_0(self , rsthis: & QTransform) -> RetType;
}
impl<'a> /*trait*/ QTransform_operator_equal_0<usize> for (usize) {
  fn operator_equal_0(self , rsthis: & QTransform) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QTransformaSEOS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtransform.h:82
// index:1
// Public Visibility=Default Availability=Available
// [88] QTransform & operator=(const QTransform &)

/*

*/
impl /*struct*/ QTransform {
  pub fn operator_equal_1<RetType, T: QTransform_operator_equal_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_1(self);
    // return 1;
  }
}
pub trait QTransform_operator_equal_1<RetType> {
  fn operator_equal_1(self , rsthis: & QTransform) -> RetType;
}
impl<'a> /*trait*/ QTransform_operator_equal_1<usize> for (usize) {
  fn operator_equal_1(self , rsthis: & QTransform) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QTransformaSERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtransform.h:91
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isAffine() const

/*
Returns true if the matrix represent an affine transformation, otherwise returns false.
*/
impl /*struct*/ QTransform {
  pub fn isAffine_0<RetType, T: QTransform_isAffine_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isAffine_0(self);
    // return 1;
  }
}
pub trait QTransform_isAffine_0<RetType> {
  fn isAffine_0(self , rsthis: & QTransform) -> RetType;
}
impl<'a> /*trait*/ QTransform_isAffine_0<bool> for () {
  fn isAffine_0(self , rsthis: & QTransform) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTransform8isAffineEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtransform.h:92
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isIdentity() const

/*
Returns true if the matrix is the identity matrix, otherwise returns false.

See also reset().
*/
impl /*struct*/ QTransform {
  pub fn isIdentity_0<RetType, T: QTransform_isIdentity_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isIdentity_0(self);
    // return 1;
  }
}
pub trait QTransform_isIdentity_0<RetType> {
  fn isIdentity_0(self , rsthis: & QTransform) -> RetType;
}
impl<'a> /*trait*/ QTransform_isIdentity_0<bool> for () {
  fn isIdentity_0(self , rsthis: & QTransform) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTransform10isIdentityEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtransform.h:93
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isInvertible() const

/*
Returns true if the matrix is invertible, otherwise returns false.

See also inverted().
*/
impl /*struct*/ QTransform {
  pub fn isInvertible_0<RetType, T: QTransform_isInvertible_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isInvertible_0(self);
    // return 1;
  }
}
pub trait QTransform_isInvertible_0<RetType> {
  fn isInvertible_0(self , rsthis: & QTransform) -> RetType;
}
impl<'a> /*trait*/ QTransform_isInvertible_0<bool> for () {
  fn isInvertible_0(self , rsthis: & QTransform) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTransform12isInvertibleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtransform.h:94
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isScaling() const

/*
Returns true if the matrix represents a scaling transformation, otherwise returns false.

See also reset().
*/
impl /*struct*/ QTransform {
  pub fn isScaling_0<RetType, T: QTransform_isScaling_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isScaling_0(self);
    // return 1;
  }
}
pub trait QTransform_isScaling_0<RetType> {
  fn isScaling_0(self , rsthis: & QTransform) -> RetType;
}
impl<'a> /*trait*/ QTransform_isScaling_0<bool> for () {
  fn isScaling_0(self , rsthis: & QTransform) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTransform9isScalingEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtransform.h:95
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isRotating() const

/*
Returns true if the matrix represents some kind of a rotating transformation, otherwise returns false.

Note: A rotation transformation of 180 degrees and/or 360 degrees is treated as a scaling transformation.

See also reset().
*/
impl /*struct*/ QTransform {
  pub fn isRotating_0<RetType, T: QTransform_isRotating_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isRotating_0(self);
    // return 1;
  }
}
pub trait QTransform_isRotating_0<RetType> {
  fn isRotating_0(self , rsthis: & QTransform) -> RetType;
}
impl<'a> /*trait*/ QTransform_isRotating_0<bool> for () {
  fn isRotating_0(self , rsthis: & QTransform) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTransform10isRotatingEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtransform.h:96
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isTranslating() const

/*
Returns true if the matrix represents a translating transformation, otherwise returns false.

See also reset().
*/
impl /*struct*/ QTransform {
  pub fn isTranslating_0<RetType, T: QTransform_isTranslating_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isTranslating_0(self);
    // return 1;
  }
}
pub trait QTransform_isTranslating_0<RetType> {
  fn isTranslating_0(self , rsthis: & QTransform) -> RetType;
}
impl<'a> /*trait*/ QTransform_isTranslating_0<bool> for () {
  fn isTranslating_0(self , rsthis: & QTransform) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTransform13isTranslatingEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtransform.h:98
// index:0
// Public Visibility=Default Availability=Available
// [4] QTransform::TransformationType type() const

/*
Returns the transformation type of this matrix.

The transformation type is the highest enumeration value capturing all of the matrix's transformations. For example, if the matrix both scales and shears, the type would be TxShear, because TxShear has a higher enumeration value than TxScale.

Knowing the transformation type of a matrix is useful for optimization: you can often handle specific types more optimally than handling the generic case.
*/
impl /*struct*/ QTransform {
  pub fn type__0<RetType, T: QTransform_type__0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.type__0(self);
    // return 1;
  }
}
pub trait QTransform_type__0<RetType> {
  fn type__0(self , rsthis: & QTransform) -> RetType;
}
impl<'a> /*trait*/ QTransform_type__0<i32> for () {
  fn type__0(self , rsthis: & QTransform) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTransform4typeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtransform.h:100
// index:0
// Public inline Visibility=Default Availability=Available
// [8] qreal determinant() const

/*
Returns the matrix's determinant.
*/
impl /*struct*/ QTransform {
  pub fn determinant_0<RetType, T: QTransform_determinant_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.determinant_0(self);
    // return 1;
  }
}
pub trait QTransform_determinant_0<RetType> {
  fn determinant_0(self , rsthis: & QTransform) -> RetType;
}
impl<'a> /*trait*/ QTransform_determinant_0<f64> for () {
  fn determinant_0(self , rsthis: & QTransform) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTransform11determinantEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtransform.h:101
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal det() const

/*

*/
impl /*struct*/ QTransform {
  pub fn det_0<RetType, T: QTransform_det_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.det_0(self);
    // return 1;
  }
}
pub trait QTransform_det_0<RetType> {
  fn det_0(self , rsthis: & QTransform) -> RetType;
}
impl<'a> /*trait*/ QTransform_det_0<f64> for () {
  fn det_0(self , rsthis: & QTransform) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTransform3detEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtransform.h:103
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal m11() const

/*
Returns the horizontal scaling factor.

See also scale() and Basic Matrix Operations.
*/
impl /*struct*/ QTransform {
  pub fn m11_0<RetType, T: QTransform_m11_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.m11_0(self);
    // return 1;
  }
}
pub trait QTransform_m11_0<RetType> {
  fn m11_0(self , rsthis: & QTransform) -> RetType;
}
impl<'a> /*trait*/ QTransform_m11_0<f64> for () {
  fn m11_0(self , rsthis: & QTransform) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTransform3m11Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtransform.h:104
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal m12() const

/*
Returns the vertical shearing factor.

See also shear() and Basic Matrix Operations.
*/
impl /*struct*/ QTransform {
  pub fn m12_0<RetType, T: QTransform_m12_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.m12_0(self);
    // return 1;
  }
}
pub trait QTransform_m12_0<RetType> {
  fn m12_0(self , rsthis: & QTransform) -> RetType;
}
impl<'a> /*trait*/ QTransform_m12_0<f64> for () {
  fn m12_0(self , rsthis: & QTransform) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTransform3m12Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtransform.h:105
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal m13() const

/*
Returns the horizontal projection factor.

See also translate() and Basic Matrix Operations.
*/
impl /*struct*/ QTransform {
  pub fn m13_0<RetType, T: QTransform_m13_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.m13_0(self);
    // return 1;
  }
}
pub trait QTransform_m13_0<RetType> {
  fn m13_0(self , rsthis: & QTransform) -> RetType;
}
impl<'a> /*trait*/ QTransform_m13_0<f64> for () {
  fn m13_0(self , rsthis: & QTransform) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTransform3m13Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtransform.h:106
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal m21() const

/*
Returns the horizontal shearing factor.

See also shear() and Basic Matrix Operations.
*/
impl /*struct*/ QTransform {
  pub fn m21_0<RetType, T: QTransform_m21_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.m21_0(self);
    // return 1;
  }
}
pub trait QTransform_m21_0<RetType> {
  fn m21_0(self , rsthis: & QTransform) -> RetType;
}
impl<'a> /*trait*/ QTransform_m21_0<f64> for () {
  fn m21_0(self , rsthis: & QTransform) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTransform3m21Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtransform.h:107
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal m22() const

/*
Returns the vertical scaling factor.

See also scale() and Basic Matrix Operations.
*/
impl /*struct*/ QTransform {
  pub fn m22_0<RetType, T: QTransform_m22_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.m22_0(self);
    // return 1;
  }
}
pub trait QTransform_m22_0<RetType> {
  fn m22_0(self , rsthis: & QTransform) -> RetType;
}
impl<'a> /*trait*/ QTransform_m22_0<f64> for () {
  fn m22_0(self , rsthis: & QTransform) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTransform3m22Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtransform.h:108
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal m23() const

/*
Returns the vertical projection factor.

See also translate() and Basic Matrix Operations.
*/
impl /*struct*/ QTransform {
  pub fn m23_0<RetType, T: QTransform_m23_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.m23_0(self);
    // return 1;
  }
}
pub trait QTransform_m23_0<RetType> {
  fn m23_0(self , rsthis: & QTransform) -> RetType;
}
impl<'a> /*trait*/ QTransform_m23_0<f64> for () {
  fn m23_0(self , rsthis: & QTransform) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTransform3m23Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtransform.h:109
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal m31() const

/*
Returns the horizontal translation factor.

See also dx(), translate(), and Basic Matrix Operations.
*/
impl /*struct*/ QTransform {
  pub fn m31_0<RetType, T: QTransform_m31_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.m31_0(self);
    // return 1;
  }
}
pub trait QTransform_m31_0<RetType> {
  fn m31_0(self , rsthis: & QTransform) -> RetType;
}
impl<'a> /*trait*/ QTransform_m31_0<f64> for () {
  fn m31_0(self , rsthis: & QTransform) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTransform3m31Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtransform.h:110
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal m32() const

/*
Returns the vertical translation factor.

See also dy(), translate(), and Basic Matrix Operations.
*/
impl /*struct*/ QTransform {
  pub fn m32_0<RetType, T: QTransform_m32_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.m32_0(self);
    // return 1;
  }
}
pub trait QTransform_m32_0<RetType> {
  fn m32_0(self , rsthis: & QTransform) -> RetType;
}
impl<'a> /*trait*/ QTransform_m32_0<f64> for () {
  fn m32_0(self , rsthis: & QTransform) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTransform3m32Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtransform.h:111
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal m33() const

/*
Returns the division factor.

See also translate() and Basic Matrix Operations.
*/
impl /*struct*/ QTransform {
  pub fn m33_0<RetType, T: QTransform_m33_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.m33_0(self);
    // return 1;
  }
}
pub trait QTransform_m33_0<RetType> {
  fn m33_0(self , rsthis: & QTransform) -> RetType;
}
impl<'a> /*trait*/ QTransform_m33_0<f64> for () {
  fn m33_0(self , rsthis: & QTransform) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTransform3m33Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtransform.h:112
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal dx() const

/*
Returns the horizontal translation factor.

See also m31(), translate(), and Basic Matrix Operations.
*/
impl /*struct*/ QTransform {
  pub fn dx_0<RetType, T: QTransform_dx_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dx_0(self);
    // return 1;
  }
}
pub trait QTransform_dx_0<RetType> {
  fn dx_0(self , rsthis: & QTransform) -> RetType;
}
impl<'a> /*trait*/ QTransform_dx_0<f64> for () {
  fn dx_0(self , rsthis: & QTransform) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTransform2dxEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtransform.h:113
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal dy() const

/*
Returns the vertical translation factor.

See also translate() and Basic Matrix Operations.
*/
impl /*struct*/ QTransform {
  pub fn dy_0<RetType, T: QTransform_dy_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dy_0(self);
    // return 1;
  }
}
pub trait QTransform_dy_0<RetType> {
  fn dy_0(self , rsthis: & QTransform) -> RetType;
}
impl<'a> /*trait*/ QTransform_dy_0<f64> for () {
  fn dy_0(self , rsthis: & QTransform) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTransform2dyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtransform.h:115
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setMatrix(qreal, qreal, qreal, qreal, qreal, qreal, qreal, qreal, qreal)

/*
Sets the matrix elements to the specified values, m11, m12, m13 m21, m22, m23 m31, m32 and m33. Note that this function replaces the previous values. QTransform provides the translate(), rotate(), scale() and shear() convenience functions to manipulate the various matrix elements based on the currently defined coordinate system.

See also QTransform().
*/
impl /*struct*/ QTransform {
  pub fn setMatrix_0<RetType, T: QTransform_setMatrix_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMatrix_0(self);
    // return 1;
  }
}
pub trait QTransform_setMatrix_0<RetType> {
  fn setMatrix_0(self , rsthis: & QTransform) -> RetType;
}
impl<'a> /*trait*/ QTransform_setMatrix_0<(/*void*/)> for (f64,f64,f64,f64,f64,f64,f64,f64,f64) {
  fn setMatrix_0(self , rsthis: & QTransform) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let arg2 = (&self.2) as *const f64 as usize;
    let arg3 = (&self.3) as *const f64 as usize;
    let arg4 = (&self.4) as *const f64 as usize;
    let arg5 = (&self.5) as *const f64 as usize;
    let arg6 = (&self.6) as *const f64 as usize;
    let arg7 = (&self.7) as *const f64 as usize;
    let arg8 = (&self.8) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN10QTransform9setMatrixEddddddddd", 9,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,arg5,arg6,arg7,arg8,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtransform.h:119
// index:0
// Public Visibility=Default Availability=Available
// [88] QTransform inverted(bool *) const

/*
Returns an inverted copy of this matrix.

If the matrix is singular (not invertible), the returned matrix is the identity matrix. If invertible is valid (i.e. not 0), its value is set to true if the matrix is invertible, otherwise it is set to false.

See also isInvertible().
*/
impl /*struct*/ QTransform {
  pub fn inverted_0<RetType, T: QTransform_inverted_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.inverted_0(self);
    // return 1;
  }
}
pub trait QTransform_inverted_0<RetType> {
  fn inverted_0(self , rsthis: & QTransform) -> RetType;
}
impl<'a> /*trait*/ QTransform_inverted_0<usize> for (usize) {
  fn inverted_0(self , rsthis: & QTransform) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTransform8invertedEPb", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtransform.h:120
// index:0
// Public Visibility=Default Availability=Available
// [88] QTransform adjoint() const

/*
Returns the adjoint of this matrix.
*/
impl /*struct*/ QTransform {
  pub fn adjoint_0<RetType, T: QTransform_adjoint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.adjoint_0(self);
    // return 1;
  }
}
pub trait QTransform_adjoint_0<RetType> {
  fn adjoint_0(self , rsthis: & QTransform) -> RetType;
}
impl<'a> /*trait*/ QTransform_adjoint_0<usize> for () {
  fn adjoint_0(self , rsthis: & QTransform) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTransform7adjointEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtransform.h:121
// index:0
// Public Visibility=Default Availability=Available
// [88] QTransform transposed() const

/*
Returns the transpose of this matrix.
*/
impl /*struct*/ QTransform {
  pub fn transposed_0<RetType, T: QTransform_transposed_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.transposed_0(self);
    // return 1;
  }
}
pub trait QTransform_transposed_0<RetType> {
  fn transposed_0(self , rsthis: & QTransform) -> RetType;
}
impl<'a> /*trait*/ QTransform_transposed_0<usize> for () {
  fn transposed_0(self , rsthis: & QTransform) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTransform10transposedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtransform.h:123
// index:0
// Public Visibility=Default Availability=Available
// [88] QTransform & translate(qreal, qreal)

/*
Moves the coordinate system dx along the x axis and dy along the y axis, and returns a reference to the matrix.

See also setMatrix().
*/
impl /*struct*/ QTransform {
  pub fn translate_0<RetType, T: QTransform_translate_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.translate_0(self);
    // return 1;
  }
}
pub trait QTransform_translate_0<RetType> {
  fn translate_0(self , rsthis: & QTransform) -> RetType;
}
impl<'a> /*trait*/ QTransform_translate_0<usize> for (f64,f64) {
  fn translate_0(self , rsthis: & QTransform) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QTransform9translateEdd", 2,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtransform.h:124
// index:0
// Public Visibility=Default Availability=Available
// [88] QTransform & scale(qreal, qreal)

/*
Scales the coordinate system by sx horizontally and sy vertically, and returns a reference to the matrix.

See also setMatrix().
*/
impl /*struct*/ QTransform {
  pub fn scale_0<RetType, T: QTransform_scale_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.scale_0(self);
    // return 1;
  }
}
pub trait QTransform_scale_0<RetType> {
  fn scale_0(self , rsthis: & QTransform) -> RetType;
}
impl<'a> /*trait*/ QTransform_scale_0<usize> for (f64,f64) {
  fn scale_0(self , rsthis: & QTransform) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QTransform5scaleEdd", 2,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtransform.h:125
// index:0
// Public Visibility=Default Availability=Available
// [88] QTransform & shear(qreal, qreal)

/*
Shears the coordinate system by sh horizontally and sv vertically, and returns a reference to the matrix.

See also setMatrix().
*/
impl /*struct*/ QTransform {
  pub fn shear_0<RetType, T: QTransform_shear_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.shear_0(self);
    // return 1;
  }
}
pub trait QTransform_shear_0<RetType> {
  fn shear_0(self , rsthis: & QTransform) -> RetType;
}
impl<'a> /*trait*/ QTransform_shear_0<usize> for (f64,f64) {
  fn shear_0(self , rsthis: & QTransform) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QTransform5shearEdd", 2,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtransform.h:126
// index:0
// Public Visibility=Default Availability=Available
// [88] QTransform & rotate(qreal, Qt::Axis)

/*
Rotates the coordinate system counterclockwise by the given angle about the specified axis and returns a reference to the matrix.

Note that if you apply a QTransform to a point defined in widget coordinates, the direction of the rotation will be clockwise because the y-axis points downwards.

The angle is specified in degrees.

See also setMatrix().
*/
impl /*struct*/ QTransform {
  pub fn rotate_0<RetType, T: QTransform_rotate_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rotate_0(self);
    // return 1;
  }
}
pub trait QTransform_rotate_0<RetType> {
  fn rotate_0(self , rsthis: & QTransform) -> RetType;
}
impl<'a> /*trait*/ QTransform_rotate_0<usize> for (f64,i32) {
  fn rotate_0(self , rsthis: & QTransform) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QTransform6rotateEdN2Qt4AxisE", 2,qtrt::FFITY_DOUBLE,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtransform.h:127
// index:0
// Public Visibility=Default Availability=Available
// [88] QTransform & rotateRadians(qreal, Qt::Axis)

/*
Rotates the coordinate system counterclockwise by the given angle about the specified axis and returns a reference to the matrix.

Note that if you apply a QTransform to a point defined in widget coordinates, the direction of the rotation will be clockwise because the y-axis points downwards.

The angle is specified in radians.

See also setMatrix().
*/
impl /*struct*/ QTransform {
  pub fn rotateRadians_0<RetType, T: QTransform_rotateRadians_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rotateRadians_0(self);
    // return 1;
  }
}
pub trait QTransform_rotateRadians_0<RetType> {
  fn rotateRadians_0(self , rsthis: & QTransform) -> RetType;
}
impl<'a> /*trait*/ QTransform_rotateRadians_0<usize> for (f64,i32) {
  fn rotateRadians_0(self , rsthis: & QTransform) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QTransform13rotateRadiansEdN2Qt4AxisE", 2,qtrt::FFITY_DOUBLE,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtransform.h:129
// index:0
// Public static Visibility=Default Availability=Available
// [1] bool squareToQuad(const QPolygonF &, QTransform &)

/*
Creates a transformation matrix, trans, that maps a unit square to a four-sided polygon, quad. Returns true if the transformation is constructed or false if such a transformation does not exist.

See also quadToSquare() and quadToQuad().
*/
impl /*struct*/ QTransform {
  pub fn squareToQuad_0<RetType, T: QTransform_squareToQuad_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.squareToQuad_0();
    // return 1;
  }
}
pub trait QTransform_squareToQuad_0<RetType> {
  fn squareToQuad_0(self ) -> RetType;
}
impl<'a> /*trait*/ QTransform_squareToQuad_0<bool> for (usize,usize) {
  fn squareToQuad_0(self ) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QTransform12squareToQuadERK9QPolygonFRS_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtransform.h:130
// index:0
// Public static Visibility=Default Availability=Available
// [1] bool quadToSquare(const QPolygonF &, QTransform &)

/*
Creates a transformation matrix, trans, that maps a four-sided polygon, quad, to a unit square. Returns true if the transformation is constructed or false if such a transformation does not exist.

See also squareToQuad() and quadToQuad().
*/
impl /*struct*/ QTransform {
  pub fn quadToSquare_0<RetType, T: QTransform_quadToSquare_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.quadToSquare_0();
    // return 1;
  }
}
pub trait QTransform_quadToSquare_0<RetType> {
  fn quadToSquare_0(self ) -> RetType;
}
impl<'a> /*trait*/ QTransform_quadToSquare_0<bool> for (usize,usize) {
  fn quadToSquare_0(self ) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QTransform12quadToSquareERK9QPolygonFRS_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtransform.h:131
// index:0
// Public static Visibility=Default Availability=Available
// [1] bool quadToQuad(const QPolygonF &, const QPolygonF &, QTransform &)

/*
Creates a transformation matrix, trans, that maps a four-sided polygon, one, to another four-sided polygon, two. Returns true if the transformation is possible; otherwise returns false.

This is a convenience method combining quadToSquare() and squareToQuad() methods. It allows the input quad to be transformed into any other quad.

See also squareToQuad() and quadToSquare().
*/
impl /*struct*/ QTransform {
  pub fn quadToQuad_0<RetType, T: QTransform_quadToQuad_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.quadToQuad_0();
    // return 1;
  }
}
pub trait QTransform_quadToQuad_0<RetType> {
  fn quadToQuad_0(self ) -> RetType;
}
impl<'a> /*trait*/ QTransform_quadToQuad_0<bool> for (usize,usize,usize) {
  fn quadToQuad_0(self ) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QTransform10quadToQuadERK9QPolygonFS2_RS_", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtransform.h:135
// index:0
// Public Visibility=Default Availability=Available
// [1] bool operator==(const QTransform &) const

/*

*/
impl /*struct*/ QTransform {
  pub fn operator_equal_equal_0<RetType, T: QTransform_operator_equal_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_equal_0(self);
    // return 1;
  }
}
pub trait QTransform_operator_equal_equal_0<RetType> {
  fn operator_equal_equal_0(self , rsthis: & QTransform) -> RetType;
}
impl<'a> /*trait*/ QTransform_operator_equal_equal_0<bool> for (usize) {
  fn operator_equal_equal_0(self , rsthis: & QTransform) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTransformeqERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtransform.h:136
// index:0
// Public Visibility=Default Availability=Available
// [1] bool operator!=(const QTransform &) const

/*

*/
impl /*struct*/ QTransform {
  pub fn operator_not_equal_0<RetType, T: QTransform_operator_not_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_not_equal_0(self);
    // return 1;
  }
}
pub trait QTransform_operator_not_equal_0<RetType> {
  fn operator_not_equal_0(self , rsthis: & QTransform) -> RetType;
}
impl<'a> /*trait*/ QTransform_operator_not_equal_0<bool> for (usize) {
  fn operator_not_equal_0(self , rsthis: & QTransform) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTransformneERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtransform.h:138
// index:0
// Public Visibility=Default Availability=Available
// [88] QTransform & operator*=(const QTransform &)

/*

*/
impl /*struct*/ QTransform {
  pub fn operator_mul_equal_0<RetType, T: QTransform_operator_mul_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_mul_equal_0(self);
    // return 1;
  }
}
pub trait QTransform_operator_mul_equal_0<RetType> {
  fn operator_mul_equal_0(self , rsthis: & QTransform) -> RetType;
}
impl<'a> /*trait*/ QTransform_operator_mul_equal_0<usize> for (usize) {
  fn operator_mul_equal_0(self , rsthis: & QTransform) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QTransformmLERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtransform.h:160
// index:1
// Public Visibility=Default Availability=Available
// [88] QTransform & operator*=(qreal)

/*

*/
impl /*struct*/ QTransform {
  pub fn operator_mul_equal_1<RetType, T: QTransform_operator_mul_equal_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_mul_equal_1(self);
    // return 1;
  }
}
pub trait QTransform_operator_mul_equal_1<RetType> {
  fn operator_mul_equal_1(self , rsthis: & QTransform) -> RetType;
}
impl<'a> /*trait*/ QTransform_operator_mul_equal_1<usize> for (f64) {
  fn operator_mul_equal_1(self , rsthis: & QTransform) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QTransformmLEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtransform.h:139
// index:0
// Public Visibility=Default Availability=Available
// [88] QTransform operator*(const QTransform &) const

/*

*/
impl /*struct*/ QTransform {
  pub fn operator_mul_0<RetType, T: QTransform_operator_mul_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_mul_0(self);
    // return 1;
  }
}
pub trait QTransform_operator_mul_0<RetType> {
  fn operator_mul_0(self , rsthis: & QTransform) -> RetType;
}
impl<'a> /*trait*/ QTransform_operator_mul_0<usize> for (usize) {
  fn operator_mul_0(self , rsthis: & QTransform) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTransformmlERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtransform.h:143
// index:0
// Public Visibility=Default Availability=Available
// [-2] void reset()

/*
Resets the matrix to an identity matrix, i.e. all elements are set to zero, except m11 and m22 (specifying the scale) and m33 which are set to 1.

See also QTransform(), isIdentity(), and Basic Matrix Operations.
*/
impl /*struct*/ QTransform {
  pub fn reset_0<RetType, T: QTransform_reset_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.reset_0(self);
    // return 1;
  }
}
pub trait QTransform_reset_0<RetType> {
  fn reset_0(self , rsthis: & QTransform) -> RetType;
}
impl<'a> /*trait*/ QTransform_reset_0<(/*void*/)> for () {
  fn reset_0(self , rsthis: & QTransform) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN10QTransform5resetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtransform.h:144
// index:0
// Public Visibility=Default Availability=Available
// [8] QPoint map(const QPoint &) const

/*
Maps the given coordinates x and y into the coordinate system defined by this matrix. The resulting values are put in *tx and *ty, respectively.

The coordinates are transformed using the following formulas:


  x' = m11*x + m21*y + dx
  y' = m22*y + m12*x + dy
  if (is not affine) {
      w' = m13*x + m23*y + m33
      x' /= w'
      y' /= w'
  }



The point (x, y) is the original point, and (x', y') is the transformed point.

See also Basic Matrix Operations.
*/
impl /*struct*/ QTransform {
  pub fn map__0<RetType, T: QTransform_map__0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.map__0(self);
    // return 1;
  }
}
pub trait QTransform_map__0<RetType> {
  fn map__0(self , rsthis: & QTransform) -> RetType;
}
impl<'a> /*trait*/ QTransform_map__0<usize> for (usize) {
  fn map__0(self , rsthis: & QTransform) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTransform3mapERK6QPoint", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtransform.h:145
// index:1
// Public Visibility=Default Availability=Available
// [16] QPointF map(const QPointF &) const

/*
Maps the given coordinates x and y into the coordinate system defined by this matrix. The resulting values are put in *tx and *ty, respectively.

The coordinates are transformed using the following formulas:


  x' = m11*x + m21*y + dx
  y' = m22*y + m12*x + dy
  if (is not affine) {
      w' = m13*x + m23*y + m33
      x' /= w'
      y' /= w'
  }



The point (x, y) is the original point, and (x', y') is the transformed point.

See also Basic Matrix Operations.
*/
impl /*struct*/ QTransform {
  pub fn map__1<RetType, T: QTransform_map__1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.map__1(self);
    // return 1;
  }
}
pub trait QTransform_map__1<RetType> {
  fn map__1(self , rsthis: & QTransform) -> RetType;
}
impl<'a> /*trait*/ QTransform_map__1<usize> for (usize) {
  fn map__1(self , rsthis: & QTransform) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTransform3mapERK7QPointF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtransform.h:146
// index:2
// Public Visibility=Default Availability=Available
// [16] QLine map(const QLine &) const

/*
Maps the given coordinates x and y into the coordinate system defined by this matrix. The resulting values are put in *tx and *ty, respectively.

The coordinates are transformed using the following formulas:


  x' = m11*x + m21*y + dx
  y' = m22*y + m12*x + dy
  if (is not affine) {
      w' = m13*x + m23*y + m33
      x' /= w'
      y' /= w'
  }



The point (x, y) is the original point, and (x', y') is the transformed point.

See also Basic Matrix Operations.
*/
impl /*struct*/ QTransform {
  pub fn map__2<RetType, T: QTransform_map__2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.map__2(self);
    // return 1;
  }
}
pub trait QTransform_map__2<RetType> {
  fn map__2(self , rsthis: & QTransform) -> RetType;
}
impl<'a> /*trait*/ QTransform_map__2<usize> for (usize) {
  fn map__2(self , rsthis: & QTransform) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTransform3mapERK5QLine", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtransform.h:147
// index:3
// Public Visibility=Default Availability=Available
// [32] QLineF map(const QLineF &) const

/*
Maps the given coordinates x and y into the coordinate system defined by this matrix. The resulting values are put in *tx and *ty, respectively.

The coordinates are transformed using the following formulas:


  x' = m11*x + m21*y + dx
  y' = m22*y + m12*x + dy
  if (is not affine) {
      w' = m13*x + m23*y + m33
      x' /= w'
      y' /= w'
  }



The point (x, y) is the original point, and (x', y') is the transformed point.

See also Basic Matrix Operations.
*/
impl /*struct*/ QTransform {
  pub fn map__3<RetType, T: QTransform_map__3<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.map__3(self);
    // return 1;
  }
}
pub trait QTransform_map__3<RetType> {
  fn map__3(self , rsthis: & QTransform) -> RetType;
}
impl<'a> /*trait*/ QTransform_map__3<usize> for (usize) {
  fn map__3(self , rsthis: & QTransform) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTransform3mapERK6QLineF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtransform.h:148
// index:4
// Public Visibility=Default Availability=Available
// [8] QPolygonF map(const QPolygonF &) const

/*
Maps the given coordinates x and y into the coordinate system defined by this matrix. The resulting values are put in *tx and *ty, respectively.

The coordinates are transformed using the following formulas:


  x' = m11*x + m21*y + dx
  y' = m22*y + m12*x + dy
  if (is not affine) {
      w' = m13*x + m23*y + m33
      x' /= w'
      y' /= w'
  }



The point (x, y) is the original point, and (x', y') is the transformed point.

See also Basic Matrix Operations.
*/
impl /*struct*/ QTransform {
  pub fn map__4<RetType, T: QTransform_map__4<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.map__4(self);
    // return 1;
  }
}
pub trait QTransform_map__4<RetType> {
  fn map__4(self , rsthis: & QTransform) -> RetType;
}
impl<'a> /*trait*/ QTransform_map__4<usize> for (usize) {
  fn map__4(self , rsthis: & QTransform) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTransform3mapERK9QPolygonF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtransform.h:149
// index:5
// Public Visibility=Default Availability=Available
// [8] QPolygon map(const QPolygon &) const

/*
Maps the given coordinates x and y into the coordinate system defined by this matrix. The resulting values are put in *tx and *ty, respectively.

The coordinates are transformed using the following formulas:


  x' = m11*x + m21*y + dx
  y' = m22*y + m12*x + dy
  if (is not affine) {
      w' = m13*x + m23*y + m33
      x' /= w'
      y' /= w'
  }



The point (x, y) is the original point, and (x', y') is the transformed point.

See also Basic Matrix Operations.
*/
impl /*struct*/ QTransform {
  pub fn map__5<RetType, T: QTransform_map__5<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.map__5(self);
    // return 1;
  }
}
pub trait QTransform_map__5<RetType> {
  fn map__5(self , rsthis: & QTransform) -> RetType;
}
impl<'a> /*trait*/ QTransform_map__5<usize> for (usize) {
  fn map__5(self , rsthis: & QTransform) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTransform3mapERK8QPolygon", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtransform.h:150
// index:6
// Public Visibility=Default Availability=Available
// [8] QRegion map(const QRegion &) const

/*
Maps the given coordinates x and y into the coordinate system defined by this matrix. The resulting values are put in *tx and *ty, respectively.

The coordinates are transformed using the following formulas:


  x' = m11*x + m21*y + dx
  y' = m22*y + m12*x + dy
  if (is not affine) {
      w' = m13*x + m23*y + m33
      x' /= w'
      y' /= w'
  }



The point (x, y) is the original point, and (x', y') is the transformed point.

See also Basic Matrix Operations.
*/
impl /*struct*/ QTransform {
  pub fn map__6<RetType, T: QTransform_map__6<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.map__6(self);
    // return 1;
  }
}
pub trait QTransform_map__6<RetType> {
  fn map__6(self , rsthis: & QTransform) -> RetType;
}
impl<'a> /*trait*/ QTransform_map__6<usize> for (usize) {
  fn map__6(self , rsthis: & QTransform) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTransform3mapERK7QRegion", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtransform.h:151
// index:7
// Public Visibility=Default Availability=Available
// [8] QPainterPath map(const QPainterPath &) const

/*
Maps the given coordinates x and y into the coordinate system defined by this matrix. The resulting values are put in *tx and *ty, respectively.

The coordinates are transformed using the following formulas:


  x' = m11*x + m21*y + dx
  y' = m22*y + m12*x + dy
  if (is not affine) {
      w' = m13*x + m23*y + m33
      x' /= w'
      y' /= w'
  }



The point (x, y) is the original point, and (x', y') is the transformed point.

See also Basic Matrix Operations.
*/
impl /*struct*/ QTransform {
  pub fn map__7<RetType, T: QTransform_map__7<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.map__7(self);
    // return 1;
  }
}
pub trait QTransform_map__7<RetType> {
  fn map__7(self , rsthis: & QTransform) -> RetType;
}
impl<'a> /*trait*/ QTransform_map__7<usize> for (usize) {
  fn map__7(self , rsthis: & QTransform) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTransform3mapERK12QPainterPath", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtransform.h:155
// index:8
// Public Visibility=Default Availability=Available
// [-2] void map(int, int, int *, int *) const

/*
Maps the given coordinates x and y into the coordinate system defined by this matrix. The resulting values are put in *tx and *ty, respectively.

The coordinates are transformed using the following formulas:


  x' = m11*x + m21*y + dx
  y' = m22*y + m12*x + dy
  if (is not affine) {
      w' = m13*x + m23*y + m33
      x' /= w'
      y' /= w'
  }



The point (x, y) is the original point, and (x', y') is the transformed point.

See also Basic Matrix Operations.
*/
impl /*struct*/ QTransform {
  pub fn map__8<RetType, T: QTransform_map__8<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.map__8(self);
    // return 1;
  }
}
pub trait QTransform_map__8<RetType> {
  fn map__8(self , rsthis: & QTransform) -> RetType;
}
impl<'a> /*trait*/ QTransform_map__8<(/*void*/)> for (i32,i32,usize,usize) {
  fn map__8(self , rsthis: & QTransform) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const usize as usize;
    let arg3 = (&self.3) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK10QTransform3mapEiiPiS0_", 4,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtransform.h:156
// index:9
// Public Visibility=Default Availability=Available
// [-2] void map(qreal, qreal, qreal *, qreal *) const

/*
Maps the given coordinates x and y into the coordinate system defined by this matrix. The resulting values are put in *tx and *ty, respectively.

The coordinates are transformed using the following formulas:


  x' = m11*x + m21*y + dx
  y' = m22*y + m12*x + dy
  if (is not affine) {
      w' = m13*x + m23*y + m33
      x' /= w'
      y' /= w'
  }



The point (x, y) is the original point, and (x', y') is the transformed point.

See also Basic Matrix Operations.
*/
impl /*struct*/ QTransform {
  pub fn map__9<RetType, T: QTransform_map__9<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.map__9(self);
    // return 1;
  }
}
pub trait QTransform_map__9<RetType> {
  fn map__9(self , rsthis: & QTransform) -> RetType;
}
impl<'a> /*trait*/ QTransform_map__9<(/*void*/)> for (f64,f64,usize,usize) {
  fn map__9(self , rsthis: & QTransform) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let arg2 = (&self.2) as *const usize as usize;
    let arg3 = (&self.3) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK10QTransform3mapEddPdS0_", 4,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtransform.h:152
// index:0
// Public Visibility=Default Availability=Available
// [8] QPolygon mapToPolygon(const QRect &) const

/*
Creates and returns a QPolygon representation of the given rectangle, mapped into the coordinate system defined by this matrix.

The rectangle's coordinates are transformed using the following formulas:


  x' = m11*x + m21*y + dx
  y' = m22*y + m12*x + dy
  if (is not affine) {
      w' = m13*x + m23*y + m33
      x' /= w'
      y' /= w'
  }



Polygons and rectangles behave slightly differently when transformed (due to integer rounding), so matrix.map(QPolygon(rectangle)) is not always the same as matrix.mapToPolygon(rectangle).

See also mapRect() and Basic Matrix Operations.
*/
impl /*struct*/ QTransform {
  pub fn mapToPolygon_0<RetType, T: QTransform_mapToPolygon_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mapToPolygon_0(self);
    // return 1;
  }
}
pub trait QTransform_mapToPolygon_0<RetType> {
  fn mapToPolygon_0(self , rsthis: & QTransform) -> RetType;
}
impl<'a> /*trait*/ QTransform_mapToPolygon_0<usize> for (usize) {
  fn mapToPolygon_0(self , rsthis: & QTransform) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTransform12mapToPolygonERK5QRect", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtransform.h:153
// index:0
// Public Visibility=Default Availability=Available
// [16] QRect mapRect(const QRect &) const

/*
Creates and returns a QRectF object that is a copy of the given rectangle, mapped into the coordinate system defined by this matrix.

The rectangle's coordinates are transformed using the following formulas:


  x' = m11*x + m21*y + dx
  y' = m22*y + m12*x + dy
  if (is not affine) {
      w' = m13*x + m23*y + m33
      x' /= w'
      y' /= w'
  }



If rotation or shearing has been specified, this function returns the bounding rectangle. To retrieve the exact region the given rectangle maps to, use the mapToPolygon() function instead.

See also mapToPolygon() and Basic Matrix Operations.
*/
impl /*struct*/ QTransform {
  pub fn mapRect_0<RetType, T: QTransform_mapRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mapRect_0(self);
    // return 1;
  }
}
pub trait QTransform_mapRect_0<RetType> {
  fn mapRect_0(self , rsthis: & QTransform) -> RetType;
}
impl<'a> /*trait*/ QTransform_mapRect_0<usize> for (usize) {
  fn mapRect_0(self , rsthis: & QTransform) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTransform7mapRectERK5QRect", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtransform.h:154
// index:1
// Public Visibility=Default Availability=Available
// [32] QRectF mapRect(const QRectF &) const

/*
Creates and returns a QRectF object that is a copy of the given rectangle, mapped into the coordinate system defined by this matrix.

The rectangle's coordinates are transformed using the following formulas:


  x' = m11*x + m21*y + dx
  y' = m22*y + m12*x + dy
  if (is not affine) {
      w' = m13*x + m23*y + m33
      x' /= w'
      y' /= w'
  }



If rotation or shearing has been specified, this function returns the bounding rectangle. To retrieve the exact region the given rectangle maps to, use the mapToPolygon() function instead.

See also mapToPolygon() and Basic Matrix Operations.
*/
impl /*struct*/ QTransform {
  pub fn mapRect_1<RetType, T: QTransform_mapRect_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mapRect_1(self);
    // return 1;
  }
}
pub trait QTransform_mapRect_1<RetType> {
  fn mapRect_1(self , rsthis: & QTransform) -> RetType;
}
impl<'a> /*trait*/ QTransform_mapRect_1<usize> for (usize) {
  fn mapRect_1(self , rsthis: & QTransform) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTransform7mapRectERK6QRectF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtransform.h:158
// index:0
// Public Visibility=Default Availability=Available
// [48] const QMatrix & toAffine() const

/*
Returns the QTransform as an affine matrix.

Warning: If a perspective transformation has been specified, then the conversion will cause loss of data.
*/
impl /*struct*/ QTransform {
  pub fn toAffine_0<RetType, T: QTransform_toAffine_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toAffine_0(self);
    // return 1;
  }
}
pub trait QTransform_toAffine_0<RetType> {
  fn toAffine_0(self , rsthis: & QTransform) -> RetType;
}
impl<'a> /*trait*/ QTransform_toAffine_0<usize> for () {
  fn toAffine_0(self , rsthis: & QTransform) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QTransform8toAffineEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtransform.h:161
// index:0
// Public Visibility=Default Availability=Available
// [88] QTransform & operator/=(qreal)

/*

*/
impl /*struct*/ QTransform {
  pub fn operator_div_equal_0<RetType, T: QTransform_operator_div_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_div_equal_0(self);
    // return 1;
  }
}
pub trait QTransform_operator_div_equal_0<RetType> {
  fn operator_div_equal_0(self , rsthis: & QTransform) -> RetType;
}
impl<'a> /*trait*/ QTransform_operator_div_equal_0<usize> for (f64) {
  fn operator_div_equal_0(self , rsthis: & QTransform) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QTransformdVEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtransform.h:162
// index:0
// Public Visibility=Default Availability=Available
// [88] QTransform & operator+=(qreal)

/*

*/
impl /*struct*/ QTransform {
  pub fn operator_add_equal_0<RetType, T: QTransform_operator_add_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_add_equal_0(self);
    // return 1;
  }
}
pub trait QTransform_operator_add_equal_0<RetType> {
  fn operator_add_equal_0(self , rsthis: & QTransform) -> RetType;
}
impl<'a> /*trait*/ QTransform_operator_add_equal_0<usize> for (f64) {
  fn operator_add_equal_0(self , rsthis: & QTransform) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QTransformpLEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtransform.h:163
// index:0
// Public Visibility=Default Availability=Available
// [88] QTransform & operator-=(qreal)

/*

*/
impl /*struct*/ QTransform {
  pub fn operator_minus_equal_0<RetType, T: QTransform_operator_minus_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_minus_equal_0(self);
    // return 1;
  }
}
pub trait QTransform_operator_minus_equal_0<RetType> {
  fn operator_minus_equal_0(self , rsthis: & QTransform) -> RetType;
}
impl<'a> /*trait*/ QTransform_operator_minus_equal_0<usize> for (f64) {
  fn operator_minus_equal_0(self , rsthis: & QTransform) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QTransformmIEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtransform.h:165
// index:0
// Public static Visibility=Default Availability=Available
// [88] QTransform fromTranslate(qreal, qreal)

/*
Creates a matrix which corresponds to a translation of dx along the x axis and dy along the y axis. This is the same as QTransform().translate(dx, dy) but slightly faster.

This function was introduced in  Qt 4.5.
*/
impl /*struct*/ QTransform {
  pub fn fromTranslate_0<RetType, T: QTransform_fromTranslate_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromTranslate_0();
    // return 1;
  }
}
pub trait QTransform_fromTranslate_0<RetType> {
  fn fromTranslate_0(self ) -> RetType;
}
impl<'a> /*trait*/ QTransform_fromTranslate_0<usize> for (f64,f64) {
  fn fromTranslate_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QTransform13fromTranslateEdd", 2,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtransform.h:166
// index:0
// Public static Visibility=Default Availability=Available
// [88] QTransform fromScale(qreal, qreal)

/*
Creates a matrix which corresponds to a scaling of sx horizontally and sy vertically. This is the same as QTransform().scale(sx, sy) but slightly faster.

This function was introduced in  Qt 4.5.
*/
impl /*struct*/ QTransform {
  pub fn fromScale_0<RetType, T: QTransform_fromScale_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromScale_0();
    // return 1;
  }
}
pub trait QTransform_fromScale_0<RetType> {
  fn fromScale_0(self ) -> RetType;
}
impl<'a> /*trait*/ QTransform_fromScale_0<usize> for (f64,f64) {
  fn fromScale_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QTransform9fromScaleEdd", 2,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}


pub fn DeleteQTransform(this :*mut QTransform) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN10QTransformD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}

/*
ConstantValue

*/
pub type QTransform__TransformationType = i32;
// 
pub const QTransform__TxNone :QTransform__TransformationType = 0;
// 
pub const QTransform__TxTranslate :QTransform__TransformationType = 1;
// 
pub const QTransform__TxScale :QTransform__TransformationType = 2;
// 
pub const QTransform__TxRotate :QTransform__TransformationType = 4;
// 
pub const QTransform__TxShear :QTransform__TransformationType = 8;
// 
pub const QTransform__TxProject :QTransform__TransformationType = 16;
pub fn QTransform_TransformationTypeItemName(val: i32) ->String {
  match val {
     QTransform__TxNone => // 0
     {return String::from("TxNone");}
     QTransform__TxTranslate => // 1
     {return String::from("TxTranslate");}
     QTransform__TxScale => // 2
     {return String::from("TxScale");}
     QTransform__TxRotate => // 4
     {return String::from("TxRotate");}
     QTransform__TxShear => // 8
     {return String::from("TxShear");}
     QTransform__TxProject => // 16
     {return String::from("TxProject");}
  _ => {return format!("{}", val);}
}
}
pub fn QTransform_TransformationTypeItemName_s(val: i32) ->String {
  //var nilthis *QTransform
  //return nilthis.TransformationTypeItemName(val);
  return QTransform_TransformationTypeItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
