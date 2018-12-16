

// mod ::gui::QMatrix
// package qtgui
// /usr/include/qt/QtGui/qmatrix.h
// #include <qmatrix.h>
// #include <QtGui>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 20
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
#[derive(Default)] // class sizeof(QMatrix)=48
pub struct QMatrix {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QMatrix_ITF interface {
//    QMatrix_PTR() *QMatrix
//}
//func (ptr *QMatrix) QMatrix_PTR() *QMatrix { return ptr }

impl /*struct*/ QMatrix {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QMatrix {
    return QMatrix{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QMatrix {
//  type Target = QMatrixBASE;
//
//  fn deref(&self) -> &QMatrixBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QMatrixBASE> for QMatrix {
//  fn as_ref(& self) -> & QMatrixBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qmatrix.h:60
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void QMatrix(Qt::Initialization)

/*
Constructs an identity matrix.

All elements are set to zero except m11 and m22 (specifying the scale), which are set to 1.

See also reset().
*/
// QMatrix(Qt::Initialization) ctx.fn_proto_cpp
impl /*struct*/ QMatrix {
  pub fn QMatrix_0<T: QMatrix_QMatrix_0>(value: T) -> QMatrix {
    let rsthis = value.QMatrix_0();
    return rsthis;
    // return 1;
  }
}

pub trait QMatrix_QMatrix_0 {
  fn QMatrix_0(self) -> QMatrix;
}
// QMatrix(Qt::Initialization) ctx.fn_proto_cpp
impl<'a> /*trait*/ QMatrix_QMatrix_0 for (i32) {
  fn QMatrix_0(self) -> QMatrix {
    // unsafe{_ZN7QMatrixC2EN2Qt14InitializationE()};
    let arg0 = (&self) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN7QMatrixC2EN2Qt14InitializationE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QMatrix{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qmatrix.h:61
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QMatrix()

/*
Constructs an identity matrix.

All elements are set to zero except m11 and m22 (specifying the scale), which are set to 1.

See also reset().
*/
// QMatrix() ctx.fn_proto_cpp
impl /*struct*/ QMatrix {
  pub fn QMatrix_1<T: QMatrix_QMatrix_1>(value: T) -> QMatrix {
    let rsthis = value.QMatrix_1();
    return rsthis;
    // return 1;
  }
}

pub trait QMatrix_QMatrix_1 {
  fn QMatrix_1(self) -> QMatrix;
}
// QMatrix() ctx.fn_proto_cpp
impl<'a> /*trait*/ QMatrix_QMatrix_1 for () {
  fn QMatrix_1(self) -> QMatrix {
    // unsafe{_ZN7QMatrixC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN7QMatrixC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QMatrix{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qmatrix.h:62
// index:2
// Public Visibility=Default Availability=Available
// [-2] void QMatrix(qreal, qreal, qreal, qreal, qreal, qreal)

/*
Constructs an identity matrix.

All elements are set to zero except m11 and m22 (specifying the scale), which are set to 1.

See also reset().
*/
// QMatrix(qreal, qreal, qreal, qreal, qreal, qreal) ctx.fn_proto_cpp
impl /*struct*/ QMatrix {
  pub fn QMatrix_2<T: QMatrix_QMatrix_2>(value: T) -> QMatrix {
    let rsthis = value.QMatrix_2();
    return rsthis;
    // return 1;
  }
}

pub trait QMatrix_QMatrix_2 {
  fn QMatrix_2(self) -> QMatrix;
}
// QMatrix(qreal, qreal, qreal, qreal, qreal, qreal) ctx.fn_proto_cpp
impl<'a> /*trait*/ QMatrix_QMatrix_2 for (f64,f64,f64,f64,f64,f64) {
  fn QMatrix_2(self) -> QMatrix {
    // unsafe{_ZN7QMatrixC2Edddddd()};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let arg2 = (&self.2) as *const f64 as usize;
    let arg3 = (&self.3) as *const f64 as usize;
    let arg4 = (&self.4) as *const f64 as usize;
    let arg5 = (&self.5) as *const f64 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN7QMatrixC2Edddddd", 6,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,arg5,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QMatrix{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qmatrix.h:67
// index:0
// Public inline Visibility=Default Availability=Available
// [48] QMatrix & operator=(QMatrix &&)

/*

*/
impl /*struct*/ QMatrix {
  pub fn operator_equal_0<RetType, T: QMatrix_operator_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_0(self);
    // return 1;
  }
}
pub trait QMatrix_operator_equal_0<RetType> {
  fn operator_equal_0(self , rsthis: & QMatrix) -> RetType;
}
impl<'a> /*trait*/ QMatrix_operator_equal_0<usize> for (usize) {
  fn operator_equal_0(self , rsthis: & QMatrix) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QMatrixaSEOS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qmatrix.h:69
// index:1
// Public Visibility=Default Availability=Available
// [48] QMatrix & operator=(const QMatrix &)

/*

*/
impl /*struct*/ QMatrix {
  pub fn operator_equal_1<RetType, T: QMatrix_operator_equal_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_1(self);
    // return 1;
  }
}
pub trait QMatrix_operator_equal_1<RetType> {
  fn operator_equal_1(self , rsthis: & QMatrix) -> RetType;
}
impl<'a> /*trait*/ QMatrix_operator_equal_1<usize> for (usize) {
  fn operator_equal_1(self , rsthis: & QMatrix) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QMatrixaSERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qmatrix.h:75
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setMatrix(qreal, qreal, qreal, qreal, qreal, qreal)

/*
Sets the matrix elements to the specified values, m11, m12, m21, m22, dx and dy.

Note that this function replaces the previous values. QMatrix provide the translate(), rotate(), scale() and shear() convenience functions to manipulate the various matrix elements based on the currently defined coordinate system.

See also QMatrix().
*/
impl /*struct*/ QMatrix {
  pub fn setMatrix_0<RetType, T: QMatrix_setMatrix_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMatrix_0(self);
    // return 1;
  }
}
pub trait QMatrix_setMatrix_0<RetType> {
  fn setMatrix_0(self , rsthis: & QMatrix) -> RetType;
}
impl<'a> /*trait*/ QMatrix_setMatrix_0<(/*void*/)> for (f64,f64,f64,f64,f64,f64) {
  fn setMatrix_0(self , rsthis: & QMatrix) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let arg2 = (&self.2) as *const f64 as usize;
    let arg3 = (&self.3) as *const f64 as usize;
    let arg4 = (&self.4) as *const f64 as usize;
    let arg5 = (&self.5) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN7QMatrix9setMatrixEdddddd", 6,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,arg5,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qmatrix.h:78
// index:0
// Public inline Visibility=Default Availability=Available
// [8] qreal m11() const

/*
Returns the horizontal scaling factor.

See also scale() and Basic Matrix Operations.
*/
impl /*struct*/ QMatrix {
  pub fn m11_0<RetType, T: QMatrix_m11_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.m11_0(self);
    // return 1;
  }
}
pub trait QMatrix_m11_0<RetType> {
  fn m11_0(self , rsthis: & QMatrix) -> RetType;
}
impl<'a> /*trait*/ QMatrix_m11_0<f64> for () {
  fn m11_0(self , rsthis: & QMatrix) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QMatrix3m11Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qmatrix.h:79
// index:0
// Public inline Visibility=Default Availability=Available
// [8] qreal m12() const

/*
Returns the vertical shearing factor.

See also shear() and Basic Matrix Operations.
*/
impl /*struct*/ QMatrix {
  pub fn m12_0<RetType, T: QMatrix_m12_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.m12_0(self);
    // return 1;
  }
}
pub trait QMatrix_m12_0<RetType> {
  fn m12_0(self , rsthis: & QMatrix) -> RetType;
}
impl<'a> /*trait*/ QMatrix_m12_0<f64> for () {
  fn m12_0(self , rsthis: & QMatrix) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QMatrix3m12Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qmatrix.h:80
// index:0
// Public inline Visibility=Default Availability=Available
// [8] qreal m21() const

/*
Returns the horizontal shearing factor.

See also shear() and Basic Matrix Operations.
*/
impl /*struct*/ QMatrix {
  pub fn m21_0<RetType, T: QMatrix_m21_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.m21_0(self);
    // return 1;
  }
}
pub trait QMatrix_m21_0<RetType> {
  fn m21_0(self , rsthis: & QMatrix) -> RetType;
}
impl<'a> /*trait*/ QMatrix_m21_0<f64> for () {
  fn m21_0(self , rsthis: & QMatrix) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QMatrix3m21Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qmatrix.h:81
// index:0
// Public inline Visibility=Default Availability=Available
// [8] qreal m22() const

/*
Returns the vertical scaling factor.

See also scale() and Basic Matrix Operations.
*/
impl /*struct*/ QMatrix {
  pub fn m22_0<RetType, T: QMatrix_m22_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.m22_0(self);
    // return 1;
  }
}
pub trait QMatrix_m22_0<RetType> {
  fn m22_0(self , rsthis: & QMatrix) -> RetType;
}
impl<'a> /*trait*/ QMatrix_m22_0<f64> for () {
  fn m22_0(self , rsthis: & QMatrix) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QMatrix3m22Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qmatrix.h:82
// index:0
// Public inline Visibility=Default Availability=Available
// [8] qreal dx() const

/*
Returns the horizontal translation factor.

See also translate() and Basic Matrix Operations.
*/
impl /*struct*/ QMatrix {
  pub fn dx_0<RetType, T: QMatrix_dx_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dx_0(self);
    // return 1;
  }
}
pub trait QMatrix_dx_0<RetType> {
  fn dx_0(self , rsthis: & QMatrix) -> RetType;
}
impl<'a> /*trait*/ QMatrix_dx_0<f64> for () {
  fn dx_0(self , rsthis: & QMatrix) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QMatrix2dxEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qmatrix.h:83
// index:0
// Public inline Visibility=Default Availability=Available
// [8] qreal dy() const

/*
Returns the vertical translation factor.

See also translate() and Basic Matrix Operations.
*/
impl /*struct*/ QMatrix {
  pub fn dy_0<RetType, T: QMatrix_dy_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dy_0(self);
    // return 1;
  }
}
pub trait QMatrix_dy_0<RetType> {
  fn dy_0(self , rsthis: & QMatrix) -> RetType;
}
impl<'a> /*trait*/ QMatrix_dy_0<f64> for () {
  fn dy_0(self , rsthis: & QMatrix) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QMatrix2dyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qmatrix.h:85
// index:0
// Public Visibility=Default Availability=Available
// [-2] void map(int, int, int *, int *) const

/*
Maps the given coordinates x and y into the coordinate system defined by this matrix. The resulting values are put in *tx and *ty, respectively.

The coordinates are transformed using the following formulas:


  x' = m11*x + m21*y + dx
  y' = m22*y + m12*x + dy



The point (x, y) is the original point, and (x', y') is the transformed point.

See also Basic Matrix Operations.
*/
impl /*struct*/ QMatrix {
  pub fn map__0<RetType, T: QMatrix_map__0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.map__0(self);
    // return 1;
  }
}
pub trait QMatrix_map__0<RetType> {
  fn map__0(self , rsthis: & QMatrix) -> RetType;
}
impl<'a> /*trait*/ QMatrix_map__0<(/*void*/)> for (i32,i32,usize,usize) {
  fn map__0(self , rsthis: & QMatrix) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const usize as usize;
    let arg3 = (&self.3) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK7QMatrix3mapEiiPiS0_", 4,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qmatrix.h:86
// index:1
// Public Visibility=Default Availability=Available
// [-2] void map(qreal, qreal, qreal *, qreal *) const

/*
Maps the given coordinates x and y into the coordinate system defined by this matrix. The resulting values are put in *tx and *ty, respectively.

The coordinates are transformed using the following formulas:


  x' = m11*x + m21*y + dx
  y' = m22*y + m12*x + dy



The point (x, y) is the original point, and (x', y') is the transformed point.

See also Basic Matrix Operations.
*/
impl /*struct*/ QMatrix {
  pub fn map__1<RetType, T: QMatrix_map__1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.map__1(self);
    // return 1;
  }
}
pub trait QMatrix_map__1<RetType> {
  fn map__1(self , rsthis: & QMatrix) -> RetType;
}
impl<'a> /*trait*/ QMatrix_map__1<(/*void*/)> for (f64,f64,usize,usize) {
  fn map__1(self , rsthis: & QMatrix) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let arg2 = (&self.2) as *const usize as usize;
    let arg3 = (&self.3) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK7QMatrix3mapEddPdS0_", 4,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qmatrix.h:90
// index:2
// Public Visibility=Default Availability=Available
// [8] QPoint map(const QPoint &) const

/*
Maps the given coordinates x and y into the coordinate system defined by this matrix. The resulting values are put in *tx and *ty, respectively.

The coordinates are transformed using the following formulas:


  x' = m11*x + m21*y + dx
  y' = m22*y + m12*x + dy



The point (x, y) is the original point, and (x', y') is the transformed point.

See also Basic Matrix Operations.
*/
impl /*struct*/ QMatrix {
  pub fn map__2<RetType, T: QMatrix_map__2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.map__2(self);
    // return 1;
  }
}
pub trait QMatrix_map__2<RetType> {
  fn map__2(self , rsthis: & QMatrix) -> RetType;
}
impl<'a> /*trait*/ QMatrix_map__2<usize> for (usize) {
  fn map__2(self , rsthis: & QMatrix) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QMatrix3mapERK6QPoint", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qmatrix.h:91
// index:3
// Public Visibility=Default Availability=Available
// [16] QPointF map(const QPointF &) const

/*
Maps the given coordinates x and y into the coordinate system defined by this matrix. The resulting values are put in *tx and *ty, respectively.

The coordinates are transformed using the following formulas:


  x' = m11*x + m21*y + dx
  y' = m22*y + m12*x + dy



The point (x, y) is the original point, and (x', y') is the transformed point.

See also Basic Matrix Operations.
*/
impl /*struct*/ QMatrix {
  pub fn map__3<RetType, T: QMatrix_map__3<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.map__3(self);
    // return 1;
  }
}
pub trait QMatrix_map__3<RetType> {
  fn map__3(self , rsthis: & QMatrix) -> RetType;
}
impl<'a> /*trait*/ QMatrix_map__3<usize> for (usize) {
  fn map__3(self , rsthis: & QMatrix) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QMatrix3mapERK7QPointF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qmatrix.h:92
// index:4
// Public Visibility=Default Availability=Available
// [16] QLine map(const QLine &) const

/*
Maps the given coordinates x and y into the coordinate system defined by this matrix. The resulting values are put in *tx and *ty, respectively.

The coordinates are transformed using the following formulas:


  x' = m11*x + m21*y + dx
  y' = m22*y + m12*x + dy



The point (x, y) is the original point, and (x', y') is the transformed point.

See also Basic Matrix Operations.
*/
impl /*struct*/ QMatrix {
  pub fn map__4<RetType, T: QMatrix_map__4<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.map__4(self);
    // return 1;
  }
}
pub trait QMatrix_map__4<RetType> {
  fn map__4(self , rsthis: & QMatrix) -> RetType;
}
impl<'a> /*trait*/ QMatrix_map__4<usize> for (usize) {
  fn map__4(self , rsthis: & QMatrix) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QMatrix3mapERK5QLine", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qmatrix.h:93
// index:5
// Public Visibility=Default Availability=Available
// [32] QLineF map(const QLineF &) const

/*
Maps the given coordinates x and y into the coordinate system defined by this matrix. The resulting values are put in *tx and *ty, respectively.

The coordinates are transformed using the following formulas:


  x' = m11*x + m21*y + dx
  y' = m22*y + m12*x + dy



The point (x, y) is the original point, and (x', y') is the transformed point.

See also Basic Matrix Operations.
*/
impl /*struct*/ QMatrix {
  pub fn map__5<RetType, T: QMatrix_map__5<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.map__5(self);
    // return 1;
  }
}
pub trait QMatrix_map__5<RetType> {
  fn map__5(self , rsthis: & QMatrix) -> RetType;
}
impl<'a> /*trait*/ QMatrix_map__5<usize> for (usize) {
  fn map__5(self , rsthis: & QMatrix) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QMatrix3mapERK6QLineF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qmatrix.h:94
// index:6
// Public Visibility=Default Availability=Available
// [8] QPolygonF map(const QPolygonF &) const

/*
Maps the given coordinates x and y into the coordinate system defined by this matrix. The resulting values are put in *tx and *ty, respectively.

The coordinates are transformed using the following formulas:


  x' = m11*x + m21*y + dx
  y' = m22*y + m12*x + dy



The point (x, y) is the original point, and (x', y') is the transformed point.

See also Basic Matrix Operations.
*/
impl /*struct*/ QMatrix {
  pub fn map__6<RetType, T: QMatrix_map__6<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.map__6(self);
    // return 1;
  }
}
pub trait QMatrix_map__6<RetType> {
  fn map__6(self , rsthis: & QMatrix) -> RetType;
}
impl<'a> /*trait*/ QMatrix_map__6<usize> for (usize) {
  fn map__6(self , rsthis: & QMatrix) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QMatrix3mapERK9QPolygonF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qmatrix.h:95
// index:7
// Public Visibility=Default Availability=Available
// [8] QPolygon map(const QPolygon &) const

/*
Maps the given coordinates x and y into the coordinate system defined by this matrix. The resulting values are put in *tx and *ty, respectively.

The coordinates are transformed using the following formulas:


  x' = m11*x + m21*y + dx
  y' = m22*y + m12*x + dy



The point (x, y) is the original point, and (x', y') is the transformed point.

See also Basic Matrix Operations.
*/
impl /*struct*/ QMatrix {
  pub fn map__7<RetType, T: QMatrix_map__7<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.map__7(self);
    // return 1;
  }
}
pub trait QMatrix_map__7<RetType> {
  fn map__7(self , rsthis: & QMatrix) -> RetType;
}
impl<'a> /*trait*/ QMatrix_map__7<usize> for (usize) {
  fn map__7(self , rsthis: & QMatrix) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QMatrix3mapERK8QPolygon", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qmatrix.h:96
// index:8
// Public Visibility=Default Availability=Available
// [8] QRegion map(const QRegion &) const

/*
Maps the given coordinates x and y into the coordinate system defined by this matrix. The resulting values are put in *tx and *ty, respectively.

The coordinates are transformed using the following formulas:


  x' = m11*x + m21*y + dx
  y' = m22*y + m12*x + dy



The point (x, y) is the original point, and (x', y') is the transformed point.

See also Basic Matrix Operations.
*/
impl /*struct*/ QMatrix {
  pub fn map__8<RetType, T: QMatrix_map__8<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.map__8(self);
    // return 1;
  }
}
pub trait QMatrix_map__8<RetType> {
  fn map__8(self , rsthis: & QMatrix) -> RetType;
}
impl<'a> /*trait*/ QMatrix_map__8<usize> for (usize) {
  fn map__8(self , rsthis: & QMatrix) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QMatrix3mapERK7QRegion", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qmatrix.h:97
// index:9
// Public Visibility=Default Availability=Available
// [8] QPainterPath map(const QPainterPath &) const

/*
Maps the given coordinates x and y into the coordinate system defined by this matrix. The resulting values are put in *tx and *ty, respectively.

The coordinates are transformed using the following formulas:


  x' = m11*x + m21*y + dx
  y' = m22*y + m12*x + dy



The point (x, y) is the original point, and (x', y') is the transformed point.

See also Basic Matrix Operations.
*/
impl /*struct*/ QMatrix {
  pub fn map__9<RetType, T: QMatrix_map__9<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.map__9(self);
    // return 1;
  }
}
pub trait QMatrix_map__9<RetType> {
  fn map__9(self , rsthis: & QMatrix) -> RetType;
}
impl<'a> /*trait*/ QMatrix_map__9<usize> for (usize) {
  fn map__9(self , rsthis: & QMatrix) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QMatrix3mapERK12QPainterPath", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qmatrix.h:87
// index:0
// Public Visibility=Default Availability=Available
// [16] QRect mapRect(const QRect &) const

/*
Creates and returns a QRectF object that is a copy of the given rectangle, mapped into the coordinate system defined by this matrix.

The rectangle's coordinates are transformed using the following formulas:


  x' = m11*x + m21*y + dx
  y' = m22*y + m12*x + dy



If rotation or shearing has been specified, this function returns the bounding rectangle. To retrieve the exact region the given rectangle maps to, use the mapToPolygon() function instead.

See also mapToPolygon() and Basic Matrix Operations.
*/
impl /*struct*/ QMatrix {
  pub fn mapRect_0<RetType, T: QMatrix_mapRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mapRect_0(self);
    // return 1;
  }
}
pub trait QMatrix_mapRect_0<RetType> {
  fn mapRect_0(self , rsthis: & QMatrix) -> RetType;
}
impl<'a> /*trait*/ QMatrix_mapRect_0<usize> for (usize) {
  fn mapRect_0(self , rsthis: & QMatrix) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QMatrix7mapRectERK5QRect", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qmatrix.h:88
// index:1
// Public Visibility=Default Availability=Available
// [32] QRectF mapRect(const QRectF &) const

/*
Creates and returns a QRectF object that is a copy of the given rectangle, mapped into the coordinate system defined by this matrix.

The rectangle's coordinates are transformed using the following formulas:


  x' = m11*x + m21*y + dx
  y' = m22*y + m12*x + dy



If rotation or shearing has been specified, this function returns the bounding rectangle. To retrieve the exact region the given rectangle maps to, use the mapToPolygon() function instead.

See also mapToPolygon() and Basic Matrix Operations.
*/
impl /*struct*/ QMatrix {
  pub fn mapRect_1<RetType, T: QMatrix_mapRect_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mapRect_1(self);
    // return 1;
  }
}
pub trait QMatrix_mapRect_1<RetType> {
  fn mapRect_1(self , rsthis: & QMatrix) -> RetType;
}
impl<'a> /*trait*/ QMatrix_mapRect_1<usize> for (usize) {
  fn mapRect_1(self , rsthis: & QMatrix) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QMatrix7mapRectERK6QRectF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qmatrix.h:98
// index:0
// Public Visibility=Default Availability=Available
// [8] QPolygon mapToPolygon(const QRect &) const

/*
Creates and returns a QPolygon representation of the given rectangle, mapped into the coordinate system defined by this matrix.

The rectangle's coordinates are transformed using the following formulas:


  x' = m11*x + m21*y + dx
  y' = m22*y + m12*x + dy



Polygons and rectangles behave slightly differently when transformed (due to integer rounding), so matrix.map(QPolygon(rectangle)) is not always the same as matrix.mapToPolygon(rectangle).

See also mapRect() and Basic Matrix Operations.
*/
impl /*struct*/ QMatrix {
  pub fn mapToPolygon_0<RetType, T: QMatrix_mapToPolygon_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mapToPolygon_0(self);
    // return 1;
  }
}
pub trait QMatrix_mapToPolygon_0<RetType> {
  fn mapToPolygon_0(self , rsthis: & QMatrix) -> RetType;
}
impl<'a> /*trait*/ QMatrix_mapToPolygon_0<usize> for (usize) {
  fn mapToPolygon_0(self , rsthis: & QMatrix) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QMatrix12mapToPolygonERK5QRect", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qmatrix.h:100
// index:0
// Public Visibility=Default Availability=Available
// [-2] void reset()

/*
Resets the matrix to an identity matrix, i.e. all elements are set to zero, except m11 and m22 (specifying the scale) which are set to 1.

See also QMatrix(), isIdentity(), and Basic Matrix Operations.
*/
impl /*struct*/ QMatrix {
  pub fn reset_0<RetType, T: QMatrix_reset_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.reset_0(self);
    // return 1;
  }
}
pub trait QMatrix_reset_0<RetType> {
  fn reset_0(self , rsthis: & QMatrix) -> RetType;
}
impl<'a> /*trait*/ QMatrix_reset_0<(/*void*/)> for () {
  fn reset_0(self , rsthis: & QMatrix) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN7QMatrix5resetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qmatrix.h:101
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isIdentity() const

/*
Returns true if the matrix is the identity matrix, otherwise returns false.

See also reset().
*/
impl /*struct*/ QMatrix {
  pub fn isIdentity_0<RetType, T: QMatrix_isIdentity_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isIdentity_0(self);
    // return 1;
  }
}
pub trait QMatrix_isIdentity_0<RetType> {
  fn isIdentity_0(self , rsthis: & QMatrix) -> RetType;
}
impl<'a> /*trait*/ QMatrix_isIdentity_0<bool> for () {
  fn isIdentity_0(self , rsthis: & QMatrix) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QMatrix10isIdentityEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qmatrix.h:103
// index:0
// Public Visibility=Default Availability=Available
// [48] QMatrix & translate(qreal, qreal)

/*
Moves the coordinate system dx along the x axis and dy along the y axis, and returns a reference to the matrix.

See also setMatrix().
*/
impl /*struct*/ QMatrix {
  pub fn translate_0<RetType, T: QMatrix_translate_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.translate_0(self);
    // return 1;
  }
}
pub trait QMatrix_translate_0<RetType> {
  fn translate_0(self , rsthis: & QMatrix) -> RetType;
}
impl<'a> /*trait*/ QMatrix_translate_0<usize> for (f64,f64) {
  fn translate_0(self , rsthis: & QMatrix) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QMatrix9translateEdd", 2,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qmatrix.h:104
// index:0
// Public Visibility=Default Availability=Available
// [48] QMatrix & scale(qreal, qreal)

/*
Scales the coordinate system by sx horizontally and sy vertically, and returns a reference to the matrix.

See also setMatrix().
*/
impl /*struct*/ QMatrix {
  pub fn scale_0<RetType, T: QMatrix_scale_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.scale_0(self);
    // return 1;
  }
}
pub trait QMatrix_scale_0<RetType> {
  fn scale_0(self , rsthis: & QMatrix) -> RetType;
}
impl<'a> /*trait*/ QMatrix_scale_0<usize> for (f64,f64) {
  fn scale_0(self , rsthis: & QMatrix) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QMatrix5scaleEdd", 2,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qmatrix.h:105
// index:0
// Public Visibility=Default Availability=Available
// [48] QMatrix & shear(qreal, qreal)

/*
Shears the coordinate system by sh horizontally and sv vertically, and returns a reference to the matrix.

See also setMatrix().
*/
impl /*struct*/ QMatrix {
  pub fn shear_0<RetType, T: QMatrix_shear_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.shear_0(self);
    // return 1;
  }
}
pub trait QMatrix_shear_0<RetType> {
  fn shear_0(self , rsthis: & QMatrix) -> RetType;
}
impl<'a> /*trait*/ QMatrix_shear_0<usize> for (f64,f64) {
  fn shear_0(self , rsthis: & QMatrix) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QMatrix5shearEdd", 2,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qmatrix.h:106
// index:0
// Public Visibility=Default Availability=Available
// [48] QMatrix & rotate(qreal)

/*
Rotates the coordinate system the given degrees counterclockwise.

Note that if you apply a QMatrix to a point defined in widget coordinates, the direction of the rotation will be clockwise because the y-axis points downwards.

Returns a reference to the matrix.

See also setMatrix().
*/
impl /*struct*/ QMatrix {
  pub fn rotate_0<RetType, T: QMatrix_rotate_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rotate_0(self);
    // return 1;
  }
}
pub trait QMatrix_rotate_0<RetType> {
  fn rotate_0(self , rsthis: & QMatrix) -> RetType;
}
impl<'a> /*trait*/ QMatrix_rotate_0<usize> for (f64) {
  fn rotate_0(self , rsthis: & QMatrix) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QMatrix6rotateEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qmatrix.h:108
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isInvertible() const

/*
Returns true if the matrix is invertible, otherwise returns false.

See also inverted().
*/
impl /*struct*/ QMatrix {
  pub fn isInvertible_0<RetType, T: QMatrix_isInvertible_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isInvertible_0(self);
    // return 1;
  }
}
pub trait QMatrix_isInvertible_0<RetType> {
  fn isInvertible_0(self , rsthis: & QMatrix) -> RetType;
}
impl<'a> /*trait*/ QMatrix_isInvertible_0<bool> for () {
  fn isInvertible_0(self , rsthis: & QMatrix) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QMatrix12isInvertibleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qmatrix.h:109
// index:0
// Public inline Visibility=Default Availability=Available
// [8] qreal determinant() const

/*
Returns the matrix's determinant.

This function was introduced in  Qt 4.6.
*/
impl /*struct*/ QMatrix {
  pub fn determinant_0<RetType, T: QMatrix_determinant_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.determinant_0(self);
    // return 1;
  }
}
pub trait QMatrix_determinant_0<RetType> {
  fn determinant_0(self , rsthis: & QMatrix) -> RetType;
}
impl<'a> /*trait*/ QMatrix_determinant_0<f64> for () {
  fn determinant_0(self , rsthis: & QMatrix) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QMatrix11determinantEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qmatrix.h:111
// index:0
// Public Visibility=Default Availability=Available
// [48] QMatrix inverted(bool *) const

/*
Returns an inverted copy of this matrix.

If the matrix is singular (not invertible), the returned matrix is the identity matrix. If invertible is valid (i.e. not 0), its value is set to true if the matrix is invertible, otherwise it is set to false.

See also isInvertible().
*/
impl /*struct*/ QMatrix {
  pub fn inverted_0<RetType, T: QMatrix_inverted_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.inverted_0(self);
    // return 1;
  }
}
pub trait QMatrix_inverted_0<RetType> {
  fn inverted_0(self , rsthis: & QMatrix) -> RetType;
}
impl<'a> /*trait*/ QMatrix_inverted_0<usize> for (usize) {
  fn inverted_0(self , rsthis: & QMatrix) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QMatrix8invertedEPb", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qmatrix.h:113
// index:0
// Public Visibility=Default Availability=Available
// [1] bool operator==(const QMatrix &) const

/*

*/
impl /*struct*/ QMatrix {
  pub fn operator_equal_equal_0<RetType, T: QMatrix_operator_equal_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_equal_0(self);
    // return 1;
  }
}
pub trait QMatrix_operator_equal_equal_0<RetType> {
  fn operator_equal_equal_0(self , rsthis: & QMatrix) -> RetType;
}
impl<'a> /*trait*/ QMatrix_operator_equal_equal_0<bool> for (usize) {
  fn operator_equal_equal_0(self , rsthis: & QMatrix) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QMatrixeqERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qmatrix.h:114
// index:0
// Public Visibility=Default Availability=Available
// [1] bool operator!=(const QMatrix &) const

/*

*/
impl /*struct*/ QMatrix {
  pub fn operator_not_equal_0<RetType, T: QMatrix_operator_not_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_not_equal_0(self);
    // return 1;
  }
}
pub trait QMatrix_operator_not_equal_0<RetType> {
  fn operator_not_equal_0(self , rsthis: & QMatrix) -> RetType;
}
impl<'a> /*trait*/ QMatrix_operator_not_equal_0<bool> for (usize) {
  fn operator_not_equal_0(self , rsthis: & QMatrix) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QMatrixneERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qmatrix.h:116
// index:0
// Public Visibility=Default Availability=Available
// [48] QMatrix & operator*=(const QMatrix &)

/*

*/
impl /*struct*/ QMatrix {
  pub fn operator_mul_equal_0<RetType, T: QMatrix_operator_mul_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_mul_equal_0(self);
    // return 1;
  }
}
pub trait QMatrix_operator_mul_equal_0<RetType> {
  fn operator_mul_equal_0(self , rsthis: & QMatrix) -> RetType;
}
impl<'a> /*trait*/ QMatrix_operator_mul_equal_0<usize> for (usize) {
  fn operator_mul_equal_0(self , rsthis: & QMatrix) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QMatrixmLERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qmatrix.h:117
// index:0
// Public Visibility=Default Availability=Available
// [48] QMatrix operator*(const QMatrix &) const

/*

*/
impl /*struct*/ QMatrix {
  pub fn operator_mul_0<RetType, T: QMatrix_operator_mul_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_mul_0(self);
    // return 1;
  }
}
pub trait QMatrix_operator_mul_0<RetType> {
  fn operator_mul_0(self , rsthis: & QMatrix) -> RetType;
}
impl<'a> /*trait*/ QMatrix_operator_mul_0<usize> for (usize) {
  fn operator_mul_0(self , rsthis: & QMatrix) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QMatrixmlERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}


pub fn DeleteQMatrix(this :*mut QMatrix) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN7QMatrixD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
