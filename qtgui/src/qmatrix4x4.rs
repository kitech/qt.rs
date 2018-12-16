

// mod ::gui::QMatrix4x4
// package qtgui
// /usr/include/qt/QtGui/qmatrix4x4.h
// #include <qmatrix4x4.h>
// #include <QtGui>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 47
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
#[derive(Default)] // class sizeof(QMatrix4x4)=68
pub struct QMatrix4x4 {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QMatrix4x4_ITF interface {
//    QMatrix4x4_PTR() *QMatrix4x4
//}
//func (ptr *QMatrix4x4) QMatrix4x4_PTR() *QMatrix4x4 { return ptr }

impl /*struct*/ QMatrix4x4 {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QMatrix4x4 {
    return QMatrix4x4{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QMatrix4x4 {
//  type Target = QMatrix4x4BASE;
//
//  fn deref(&self) -> &QMatrix4x4BASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QMatrix4x4BASE> for QMatrix4x4 {
//  fn as_ref(& self) -> & QMatrix4x4BASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qmatrix4x4.h:62
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void QMatrix4x4()

/*
Constructs an identity matrix.
*/
// QMatrix4x4() ctx.fn_proto_cpp
impl /*struct*/ QMatrix4x4 {
  pub fn QMatrix4x4_0<T: QMatrix4x4_QMatrix4x4_0>(value: T) -> QMatrix4x4 {
    let rsthis = value.QMatrix4x4_0();
    return rsthis;
    // return 1;
  }
}

pub trait QMatrix4x4_QMatrix4x4_0 {
  fn QMatrix4x4_0(self) -> QMatrix4x4;
}
// QMatrix4x4() ctx.fn_proto_cpp
impl<'a> /*trait*/ QMatrix4x4_QMatrix4x4_0 for () {
  fn QMatrix4x4_0(self) -> QMatrix4x4 {
    // unsafe{_ZN10QMatrix4x4C2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN10QMatrix4x4C2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QMatrix4x4{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qmatrix4x4.h:63
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void QMatrix4x4(Qt::Initialization)

/*
Constructs an identity matrix.
*/
// QMatrix4x4(Qt::Initialization) ctx.fn_proto_cpp
impl /*struct*/ QMatrix4x4 {
  pub fn QMatrix4x4_1<T: QMatrix4x4_QMatrix4x4_1>(value: T) -> QMatrix4x4 {
    let rsthis = value.QMatrix4x4_1();
    return rsthis;
    // return 1;
  }
}

pub trait QMatrix4x4_QMatrix4x4_1 {
  fn QMatrix4x4_1(self) -> QMatrix4x4;
}
// QMatrix4x4(Qt::Initialization) ctx.fn_proto_cpp
impl<'a> /*trait*/ QMatrix4x4_QMatrix4x4_1 for (i32) {
  fn QMatrix4x4_1(self) -> QMatrix4x4 {
    // unsafe{_ZN10QMatrix4x4C2EN2Qt14InitializationE()};
    let arg0 = (&self) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN10QMatrix4x4C2EN2Qt14InitializationE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QMatrix4x4{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qmatrix4x4.h:64
// index:2
// Public Visibility=Default Availability=Available
// [-2] void QMatrix4x4(const float *)

/*
Constructs an identity matrix.
*/
// QMatrix4x4(const float *) ctx.fn_proto_cpp
impl /*struct*/ QMatrix4x4 {
  pub fn QMatrix4x4_2<T: QMatrix4x4_QMatrix4x4_2>(value: T) -> QMatrix4x4 {
    let rsthis = value.QMatrix4x4_2();
    return rsthis;
    // return 1;
  }
}

pub trait QMatrix4x4_QMatrix4x4_2 {
  fn QMatrix4x4_2(self) -> QMatrix4x4;
}
// QMatrix4x4(const float *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QMatrix4x4_QMatrix4x4_2 for (usize) {
  fn QMatrix4x4_2(self) -> QMatrix4x4 {
    // unsafe{_ZN10QMatrix4x4C2EPKf()};
    let arg0 = (&self) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN10QMatrix4x4C2EPKf", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QMatrix4x4{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qmatrix4x4.h:65
// index:3
// Public inline Visibility=Default Availability=Available
// [-2] void QMatrix4x4(float, float, float, float, float, float, float, float, float, float, float, float, float, float, float, float)

/*
Constructs an identity matrix.
*/
// QMatrix4x4(float, float, float, float, float, float, float, float, float, float, float, float, float, float, float, float) ctx.fn_proto_cpp
impl /*struct*/ QMatrix4x4 {
  pub fn QMatrix4x4_3<T: QMatrix4x4_QMatrix4x4_3>(value: T) -> QMatrix4x4 {
    let rsthis = value.QMatrix4x4_3();
    return rsthis;
    // return 1;
  }
}

pub trait QMatrix4x4_QMatrix4x4_3 {
  fn QMatrix4x4_3(self) -> QMatrix4x4;
}
// QMatrix4x4(float, float, float, float, float, float, float, float, float, float, float, float, float, float, float, float) ctx.fn_proto_cpp
impl<'a> /*trait*/ QMatrix4x4_QMatrix4x4_3 for (f32,f32,f32,f32,f32,f32,f32,f32,f32,f32,f32,f32,f32,f32,f32,f32) {
  fn QMatrix4x4_3(self) -> QMatrix4x4 {
    // unsafe{_ZN10QMatrix4x4C2Effffffffffffffff()};
    let arg0 = (&self.0) as *const f32 as usize;
    let arg1 = (&self.1) as *const f32 as usize;
    let arg2 = (&self.2) as *const f32 as usize;
    let arg3 = (&self.3) as *const f32 as usize;
    let arg4 = (&self.4) as *const f32 as usize;
    let arg5 = (&self.5) as *const f32 as usize;
    let arg6 = (&self.6) as *const f32 as usize;
    let arg7 = (&self.7) as *const f32 as usize;
    let arg8 = (&self.8) as *const f32 as usize;
    let arg9 = (&self.9) as *const f32 as usize;
    let arg10 = (&self.10) as *const f32 as usize;
    let arg11 = (&self.11) as *const f32 as usize;
    let arg12 = (&self.12) as *const f32 as usize;
    let arg13 = (&self.13) as *const f32 as usize;
    let arg14 = (&self.14) as *const f32 as usize;
    let arg15 = (&self.15) as *const f32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN10QMatrix4x4C2Effffffffffffffff", 16,qtrt::FFITY_FLOAT,qtrt::FFITY_FLOAT,qtrt::FFITY_FLOAT,qtrt::FFITY_FLOAT,qtrt::FFITY_FLOAT,qtrt::FFITY_FLOAT,qtrt::FFITY_FLOAT,qtrt::FFITY_FLOAT,qtrt::FFITY_FLOAT,qtrt::FFITY_FLOAT,qtrt::FFITY_FLOAT,qtrt::FFITY_FLOAT,qtrt::FFITY_FLOAT,qtrt::FFITY_FLOAT,qtrt::FFITY_FLOAT,qtrt::FFITY_FLOAT,arg0,arg1,arg2,arg3,arg4,arg5,arg6,arg7,arg8,arg9,arg10,arg11,arg12,arg13,arg14,arg15);
    let rsthis = QMatrix4x4{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qmatrix4x4.h:73
// index:4
// Public Visibility=Default Availability=Available
// [-2] void QMatrix4x4(const float *, int, int)

/*
Constructs an identity matrix.
*/
// QMatrix4x4(const float *, int, int) ctx.fn_proto_cpp
impl /*struct*/ QMatrix4x4 {
  pub fn QMatrix4x4_4<T: QMatrix4x4_QMatrix4x4_4>(value: T) -> QMatrix4x4 {
    let rsthis = value.QMatrix4x4_4();
    return rsthis;
    // return 1;
  }
}

pub trait QMatrix4x4_QMatrix4x4_4 {
  fn QMatrix4x4_4(self) -> QMatrix4x4;
}
// QMatrix4x4(const float *, int, int) ctx.fn_proto_cpp
impl<'a> /*trait*/ QMatrix4x4_QMatrix4x4_4 for (usize,i32,i32) {
  fn QMatrix4x4_4(self) -> QMatrix4x4 {
    // unsafe{_ZN10QMatrix4x4C2EPKfii()};
    let arg0 = (&self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN10QMatrix4x4C2EPKfii", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QMatrix4x4{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qmatrix4x4.h:74
// index:5
// Public Visibility=Default Availability=Available
// [-2] void QMatrix4x4(const QTransform &)

/*
Constructs an identity matrix.
*/
// QMatrix4x4(const QTransform &) ctx.fn_proto_cpp
impl /*struct*/ QMatrix4x4 {
  pub fn QMatrix4x4_5<T: QMatrix4x4_QMatrix4x4_5>(value: T) -> QMatrix4x4 {
    let rsthis = value.QMatrix4x4_5();
    return rsthis;
    // return 1;
  }
}

pub trait QMatrix4x4_QMatrix4x4_5 {
  fn QMatrix4x4_5(self) -> QMatrix4x4;
}
// QMatrix4x4(const QTransform &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QMatrix4x4_QMatrix4x4_5 for (usize) {
  fn QMatrix4x4_5(self) -> QMatrix4x4 {
    // unsafe{_ZN10QMatrix4x4C2ERK10QTransform()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN10QMatrix4x4C2ERK10QTransform", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QMatrix4x4{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qmatrix4x4.h:75
// index:6
// Public Visibility=Default Availability=Available
// [-2] void QMatrix4x4(const QMatrix &)

/*
Constructs an identity matrix.
*/
// QMatrix4x4(const QMatrix &) ctx.fn_proto_cpp
impl /*struct*/ QMatrix4x4 {
  pub fn QMatrix4x4_6<T: QMatrix4x4_QMatrix4x4_6>(value: T) -> QMatrix4x4 {
    let rsthis = value.QMatrix4x4_6();
    return rsthis;
    // return 1;
  }
}

pub trait QMatrix4x4_QMatrix4x4_6 {
  fn QMatrix4x4_6(self) -> QMatrix4x4;
}
// QMatrix4x4(const QMatrix &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QMatrix4x4_QMatrix4x4_6 for (usize) {
  fn QMatrix4x4_6(self) -> QMatrix4x4 {
    // unsafe{_ZN10QMatrix4x4C2ERK7QMatrix()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN10QMatrix4x4C2ERK7QMatrix", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QMatrix4x4{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qmatrix4x4.h:77
// index:0
// Public inline Visibility=Default Availability=Available
// [4] const float & operator()(int, int) const

/*

*/
impl /*struct*/ QMatrix4x4 {
  pub fn operator_fncall_0<RetType, T: QMatrix4x4_operator_fncall_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_fncall_0(self);
    // return 1;
  }
}
pub trait QMatrix4x4_operator_fncall_0<RetType> {
  fn operator_fncall_0(self , rsthis: & QMatrix4x4) -> RetType;
}
impl<'a> /*trait*/ QMatrix4x4_operator_fncall_0<usize> for (i32,i32) {
  fn operator_fncall_0(self , rsthis: & QMatrix4x4) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QMatrix4x4clEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qmatrix4x4.h:78
// index:1
// Public inline Visibility=Default Availability=Available
// [4] float & operator()(int, int)

/*

*/
impl /*struct*/ QMatrix4x4 {
  pub fn operator_fncall_1<RetType, T: QMatrix4x4_operator_fncall_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_fncall_1(self);
    // return 1;
  }
}
pub trait QMatrix4x4_operator_fncall_1<RetType> {
  fn operator_fncall_1(self , rsthis: & QMatrix4x4) -> RetType;
}
impl<'a> /*trait*/ QMatrix4x4_operator_fncall_1<usize> for (i32,i32) {
  fn operator_fncall_1(self , rsthis: & QMatrix4x4) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QMatrix4x4clEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qmatrix4x4.h:81
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QVector4D column(int) const

/*
Returns the elements of column index as a 4D vector.

See also setColumn() and row().
*/
impl /*struct*/ QMatrix4x4 {
  pub fn column_0<RetType, T: QMatrix4x4_column_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.column_0(self);
    // return 1;
  }
}
pub trait QMatrix4x4_column_0<RetType> {
  fn column_0(self , rsthis: & QMatrix4x4) -> RetType;
}
impl<'a> /*trait*/ QMatrix4x4_column_0<usize> for (i32) {
  fn column_0(self , rsthis: & QMatrix4x4) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QMatrix4x46columnEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qmatrix4x4.h:82
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setColumn(int, const QVector4D &)

/*
Sets the elements of column index to the components of value.

See also column() and setRow().
*/
impl /*struct*/ QMatrix4x4 {
  pub fn setColumn_0<RetType, T: QMatrix4x4_setColumn_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setColumn_0(self);
    // return 1;
  }
}
pub trait QMatrix4x4_setColumn_0<RetType> {
  fn setColumn_0(self , rsthis: & QMatrix4x4) -> RetType;
}
impl<'a> /*trait*/ QMatrix4x4_setColumn_0<(/*void*/)> for (i32,usize) {
  fn setColumn_0(self , rsthis: & QMatrix4x4) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN10QMatrix4x49setColumnEiRK9QVector4D", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qmatrix4x4.h:84
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QVector4D row(int) const

/*
Returns the elements of row index as a 4D vector.

See also setRow() and column().
*/
impl /*struct*/ QMatrix4x4 {
  pub fn row_0<RetType, T: QMatrix4x4_row_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.row_0(self);
    // return 1;
  }
}
pub trait QMatrix4x4_row_0<RetType> {
  fn row_0(self , rsthis: & QMatrix4x4) -> RetType;
}
impl<'a> /*trait*/ QMatrix4x4_row_0<usize> for (i32) {
  fn row_0(self , rsthis: & QMatrix4x4) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QMatrix4x43rowEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qmatrix4x4.h:85
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setRow(int, const QVector4D &)

/*
Sets the elements of row index to the components of value.

See also row() and setColumn().
*/
impl /*struct*/ QMatrix4x4 {
  pub fn setRow_0<RetType, T: QMatrix4x4_setRow_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setRow_0(self);
    // return 1;
  }
}
pub trait QMatrix4x4_setRow_0<RetType> {
  fn setRow_0(self , rsthis: & QMatrix4x4) -> RetType;
}
impl<'a> /*trait*/ QMatrix4x4_setRow_0<(/*void*/)> for (i32,usize) {
  fn setRow_0(self , rsthis: & QMatrix4x4) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN10QMatrix4x46setRowEiRK9QVector4D", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qmatrix4x4.h:88
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isAffine() const

/*
Returns true if this matrix is affine matrix; false otherwise.

An affine matrix is a 4x4 matrix with row 3 equal to (0, 0, 0, 1), e.g. no projective coefficients.

This function was introduced in  Qt 5.5.

See also isIdentity().
*/
impl /*struct*/ QMatrix4x4 {
  pub fn isAffine_0<RetType, T: QMatrix4x4_isAffine_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isAffine_0(self);
    // return 1;
  }
}
pub trait QMatrix4x4_isAffine_0<RetType> {
  fn isAffine_0(self , rsthis: & QMatrix4x4) -> RetType;
}
impl<'a> /*trait*/ QMatrix4x4_isAffine_0<bool> for () {
  fn isAffine_0(self , rsthis: & QMatrix4x4) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QMatrix4x48isAffineEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qmatrix4x4.h:90
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isIdentity() const

/*
Returns true if this matrix is the identity; false otherwise.

See also setToIdentity().
*/
impl /*struct*/ QMatrix4x4 {
  pub fn isIdentity_0<RetType, T: QMatrix4x4_isIdentity_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isIdentity_0(self);
    // return 1;
  }
}
pub trait QMatrix4x4_isIdentity_0<RetType> {
  fn isIdentity_0(self , rsthis: & QMatrix4x4) -> RetType;
}
impl<'a> /*trait*/ QMatrix4x4_isIdentity_0<bool> for () {
  fn isIdentity_0(self , rsthis: & QMatrix4x4) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QMatrix4x410isIdentityEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qmatrix4x4.h:91
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setToIdentity()

/*
Sets this matrix to the identity.

See also isIdentity().
*/
impl /*struct*/ QMatrix4x4 {
  pub fn setToIdentity_0<RetType, T: QMatrix4x4_setToIdentity_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setToIdentity_0(self);
    // return 1;
  }
}
pub trait QMatrix4x4_setToIdentity_0<RetType> {
  fn setToIdentity_0(self , rsthis: & QMatrix4x4) -> RetType;
}
impl<'a> /*trait*/ QMatrix4x4_setToIdentity_0<(/*void*/)> for () {
  fn setToIdentity_0(self , rsthis: & QMatrix4x4) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN10QMatrix4x413setToIdentityEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qmatrix4x4.h:93
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void fill(float)

/*
Fills all elements of this matrx with value.
*/
impl /*struct*/ QMatrix4x4 {
  pub fn fill_0<RetType, T: QMatrix4x4_fill_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fill_0(self);
    // return 1;
  }
}
pub trait QMatrix4x4_fill_0<RetType> {
  fn fill_0(self , rsthis: & QMatrix4x4) -> RetType;
}
impl<'a> /*trait*/ QMatrix4x4_fill_0<(/*void*/)> for (f32) {
  fn fill_0(self , rsthis: & QMatrix4x4) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QMatrix4x44fillEf", 1,qtrt::FFITY_FLOAT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qmatrix4x4.h:95
// index:0
// Public Visibility=Default Availability=Available
// [8] double determinant() const

/*
Returns the determinant of this matrix.
*/
impl /*struct*/ QMatrix4x4 {
  pub fn determinant_0<RetType, T: QMatrix4x4_determinant_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.determinant_0(self);
    // return 1;
  }
}
pub trait QMatrix4x4_determinant_0<RetType> {
  fn determinant_0(self , rsthis: & QMatrix4x4) -> RetType;
}
impl<'a> /*trait*/ QMatrix4x4_determinant_0<f64> for () {
  fn determinant_0(self , rsthis: & QMatrix4x4) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QMatrix4x411determinantEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qmatrix4x4.h:96
// index:0
// Public Visibility=Default Availability=Available
// [68] QMatrix4x4 inverted(bool *) const

/*
Returns the inverse of this matrix. Returns the identity if this matrix cannot be inverted; i.e. determinant() is zero. If invertible is not null, then true will be written to that location if the matrix can be inverted; false otherwise.

If the matrix is recognized as the identity or an orthonormal matrix, then this function will quickly invert the matrix using optimized routines.

See also determinant() and normalMatrix().
*/
impl /*struct*/ QMatrix4x4 {
  pub fn inverted_0<RetType, T: QMatrix4x4_inverted_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.inverted_0(self);
    // return 1;
  }
}
pub trait QMatrix4x4_inverted_0<RetType> {
  fn inverted_0(self , rsthis: & QMatrix4x4) -> RetType;
}
impl<'a> /*trait*/ QMatrix4x4_inverted_0<usize> for (usize) {
  fn inverted_0(self , rsthis: & QMatrix4x4) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QMatrix4x48invertedEPb", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qmatrix4x4.h:97
// index:0
// Public Visibility=Default Availability=Available
// [68] QMatrix4x4 transposed() const

/*
Returns this matrix, transposed about its diagonal.
*/
impl /*struct*/ QMatrix4x4 {
  pub fn transposed_0<RetType, T: QMatrix4x4_transposed_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.transposed_0(self);
    // return 1;
  }
}
pub trait QMatrix4x4_transposed_0<RetType> {
  fn transposed_0(self , rsthis: & QMatrix4x4) -> RetType;
}
impl<'a> /*trait*/ QMatrix4x4_transposed_0<usize> for () {
  fn transposed_0(self , rsthis: & QMatrix4x4) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QMatrix4x410transposedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qmatrix4x4.h:100
// index:0
// Public inline Visibility=Default Availability=Available
// [68] QMatrix4x4 & operator+=(const QMatrix4x4 &)

/*

*/
impl /*struct*/ QMatrix4x4 {
  pub fn operator_add_equal_0<RetType, T: QMatrix4x4_operator_add_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_add_equal_0(self);
    // return 1;
  }
}
pub trait QMatrix4x4_operator_add_equal_0<RetType> {
  fn operator_add_equal_0(self , rsthis: & QMatrix4x4) -> RetType;
}
impl<'a> /*trait*/ QMatrix4x4_operator_add_equal_0<usize> for (usize) {
  fn operator_add_equal_0(self , rsthis: & QMatrix4x4) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QMatrix4x4pLERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qmatrix4x4.h:101
// index:0
// Public inline Visibility=Default Availability=Available
// [68] QMatrix4x4 & operator-=(const QMatrix4x4 &)

/*

*/
impl /*struct*/ QMatrix4x4 {
  pub fn operator_minus_equal_0<RetType, T: QMatrix4x4_operator_minus_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_minus_equal_0(self);
    // return 1;
  }
}
pub trait QMatrix4x4_operator_minus_equal_0<RetType> {
  fn operator_minus_equal_0(self , rsthis: & QMatrix4x4) -> RetType;
}
impl<'a> /*trait*/ QMatrix4x4_operator_minus_equal_0<usize> for (usize) {
  fn operator_minus_equal_0(self , rsthis: & QMatrix4x4) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QMatrix4x4mIERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qmatrix4x4.h:102
// index:0
// Public inline Visibility=Default Availability=Available
// [68] QMatrix4x4 & operator*=(const QMatrix4x4 &)

/*

*/
impl /*struct*/ QMatrix4x4 {
  pub fn operator_mul_equal_0<RetType, T: QMatrix4x4_operator_mul_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_mul_equal_0(self);
    // return 1;
  }
}
pub trait QMatrix4x4_operator_mul_equal_0<RetType> {
  fn operator_mul_equal_0(self , rsthis: & QMatrix4x4) -> RetType;
}
impl<'a> /*trait*/ QMatrix4x4_operator_mul_equal_0<usize> for (usize) {
  fn operator_mul_equal_0(self , rsthis: & QMatrix4x4) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QMatrix4x4mLERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qmatrix4x4.h:103
// index:1
// Public inline Visibility=Default Availability=Available
// [68] QMatrix4x4 & operator*=(float)

/*

*/
impl /*struct*/ QMatrix4x4 {
  pub fn operator_mul_equal_1<RetType, T: QMatrix4x4_operator_mul_equal_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_mul_equal_1(self);
    // return 1;
  }
}
pub trait QMatrix4x4_operator_mul_equal_1<RetType> {
  fn operator_mul_equal_1(self , rsthis: & QMatrix4x4) -> RetType;
}
impl<'a> /*trait*/ QMatrix4x4_operator_mul_equal_1<usize> for (f32) {
  fn operator_mul_equal_1(self , rsthis: & QMatrix4x4) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QMatrix4x4mLEf", 1,qtrt::FFITY_FLOAT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qmatrix4x4.h:104
// index:0
// Public Visibility=Default Availability=Available
// [68] QMatrix4x4 & operator/=(float)

/*

*/
impl /*struct*/ QMatrix4x4 {
  pub fn operator_div_equal_0<RetType, T: QMatrix4x4_operator_div_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_div_equal_0(self);
    // return 1;
  }
}
pub trait QMatrix4x4_operator_div_equal_0<RetType> {
  fn operator_div_equal_0(self , rsthis: & QMatrix4x4) -> RetType;
}
impl<'a> /*trait*/ QMatrix4x4_operator_div_equal_0<usize> for (f32) {
  fn operator_div_equal_0(self , rsthis: & QMatrix4x4) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QMatrix4x4dVEf", 1,qtrt::FFITY_FLOAT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qmatrix4x4.h:105
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator==(const QMatrix4x4 &) const

/*

*/
impl /*struct*/ QMatrix4x4 {
  pub fn operator_equal_equal_0<RetType, T: QMatrix4x4_operator_equal_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_equal_0(self);
    // return 1;
  }
}
pub trait QMatrix4x4_operator_equal_equal_0<RetType> {
  fn operator_equal_equal_0(self , rsthis: & QMatrix4x4) -> RetType;
}
impl<'a> /*trait*/ QMatrix4x4_operator_equal_equal_0<bool> for (usize) {
  fn operator_equal_equal_0(self , rsthis: & QMatrix4x4) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QMatrix4x4eqERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qmatrix4x4.h:106
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator!=(const QMatrix4x4 &) const

/*

*/
impl /*struct*/ QMatrix4x4 {
  pub fn operator_not_equal_0<RetType, T: QMatrix4x4_operator_not_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_not_equal_0(self);
    // return 1;
  }
}
pub trait QMatrix4x4_operator_not_equal_0<RetType> {
  fn operator_not_equal_0(self , rsthis: & QMatrix4x4) -> RetType;
}
impl<'a> /*trait*/ QMatrix4x4_operator_not_equal_0<bool> for (usize) {
  fn operator_not_equal_0(self , rsthis: & QMatrix4x4) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QMatrix4x4neERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qmatrix4x4.h:131
// index:0
// Public Visibility=Default Availability=Available
// [-2] void scale(const QVector3D &)

/*
Multiplies this matrix by another that scales coordinates by the components of vector.

See also translate() and rotate().
*/
impl /*struct*/ QMatrix4x4 {
  pub fn scale_0<RetType, T: QMatrix4x4_scale_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.scale_0(self);
    // return 1;
  }
}
pub trait QMatrix4x4_scale_0<RetType> {
  fn scale_0(self , rsthis: & QMatrix4x4) -> RetType;
}
impl<'a> /*trait*/ QMatrix4x4_scale_0<(/*void*/)> for (usize) {
  fn scale_0(self , rsthis: & QMatrix4x4) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN10QMatrix4x45scaleERK9QVector3D", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qmatrix4x4.h:135
// index:1
// Public Visibility=Default Availability=Available
// [-2] void scale(float, float)

/*
Multiplies this matrix by another that scales coordinates by the components of vector.

See also translate() and rotate().
*/
impl /*struct*/ QMatrix4x4 {
  pub fn scale_1<RetType, T: QMatrix4x4_scale_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.scale_1(self);
    // return 1;
  }
}
pub trait QMatrix4x4_scale_1<RetType> {
  fn scale_1(self , rsthis: & QMatrix4x4) -> RetType;
}
impl<'a> /*trait*/ QMatrix4x4_scale_1<(/*void*/)> for (f32,f32) {
  fn scale_1(self , rsthis: & QMatrix4x4) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f32 as usize;
    let arg1 = (&self.1) as *const f32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QMatrix4x45scaleEff", 2,qtrt::FFITY_FLOAT,qtrt::FFITY_FLOAT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qmatrix4x4.h:136
// index:2
// Public Visibility=Default Availability=Available
// [-2] void scale(float, float, float)

/*
Multiplies this matrix by another that scales coordinates by the components of vector.

See also translate() and rotate().
*/
impl /*struct*/ QMatrix4x4 {
  pub fn scale_2<RetType, T: QMatrix4x4_scale_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.scale_2(self);
    // return 1;
  }
}
pub trait QMatrix4x4_scale_2<RetType> {
  fn scale_2(self , rsthis: & QMatrix4x4) -> RetType;
}
impl<'a> /*trait*/ QMatrix4x4_scale_2<(/*void*/)> for (f32,f32,f32) {
  fn scale_2(self , rsthis: & QMatrix4x4) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f32 as usize;
    let arg1 = (&self.1) as *const f32 as usize;
    let arg2 = (&self.2) as *const f32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QMatrix4x45scaleEfff", 3,qtrt::FFITY_FLOAT,qtrt::FFITY_FLOAT,qtrt::FFITY_FLOAT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qmatrix4x4.h:137
// index:3
// Public Visibility=Default Availability=Available
// [-2] void scale(float)

/*
Multiplies this matrix by another that scales coordinates by the components of vector.

See also translate() and rotate().
*/
impl /*struct*/ QMatrix4x4 {
  pub fn scale_3<RetType, T: QMatrix4x4_scale_3<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.scale_3(self);
    // return 1;
  }
}
pub trait QMatrix4x4_scale_3<RetType> {
  fn scale_3(self , rsthis: & QMatrix4x4) -> RetType;
}
impl<'a> /*trait*/ QMatrix4x4_scale_3<(/*void*/)> for (f32) {
  fn scale_3(self , rsthis: & QMatrix4x4) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QMatrix4x45scaleEf", 1,qtrt::FFITY_FLOAT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qmatrix4x4.h:132
// index:0
// Public Visibility=Default Availability=Available
// [-2] void translate(const QVector3D &)

/*
Multiplies this matrix by another that translates coordinates by the components of vector.

See also scale() and rotate().
*/
impl /*struct*/ QMatrix4x4 {
  pub fn translate_0<RetType, T: QMatrix4x4_translate_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.translate_0(self);
    // return 1;
  }
}
pub trait QMatrix4x4_translate_0<RetType> {
  fn translate_0(self , rsthis: & QMatrix4x4) -> RetType;
}
impl<'a> /*trait*/ QMatrix4x4_translate_0<(/*void*/)> for (usize) {
  fn translate_0(self , rsthis: & QMatrix4x4) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN10QMatrix4x49translateERK9QVector3D", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qmatrix4x4.h:138
// index:1
// Public Visibility=Default Availability=Available
// [-2] void translate(float, float)

/*
Multiplies this matrix by another that translates coordinates by the components of vector.

See also scale() and rotate().
*/
impl /*struct*/ QMatrix4x4 {
  pub fn translate_1<RetType, T: QMatrix4x4_translate_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.translate_1(self);
    // return 1;
  }
}
pub trait QMatrix4x4_translate_1<RetType> {
  fn translate_1(self , rsthis: & QMatrix4x4) -> RetType;
}
impl<'a> /*trait*/ QMatrix4x4_translate_1<(/*void*/)> for (f32,f32) {
  fn translate_1(self , rsthis: & QMatrix4x4) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f32 as usize;
    let arg1 = (&self.1) as *const f32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QMatrix4x49translateEff", 2,qtrt::FFITY_FLOAT,qtrt::FFITY_FLOAT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qmatrix4x4.h:139
// index:2
// Public Visibility=Default Availability=Available
// [-2] void translate(float, float, float)

/*
Multiplies this matrix by another that translates coordinates by the components of vector.

See also scale() and rotate().
*/
impl /*struct*/ QMatrix4x4 {
  pub fn translate_2<RetType, T: QMatrix4x4_translate_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.translate_2(self);
    // return 1;
  }
}
pub trait QMatrix4x4_translate_2<RetType> {
  fn translate_2(self , rsthis: & QMatrix4x4) -> RetType;
}
impl<'a> /*trait*/ QMatrix4x4_translate_2<(/*void*/)> for (f32,f32,f32) {
  fn translate_2(self , rsthis: & QMatrix4x4) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f32 as usize;
    let arg1 = (&self.1) as *const f32 as usize;
    let arg2 = (&self.2) as *const f32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QMatrix4x49translateEfff", 3,qtrt::FFITY_FLOAT,qtrt::FFITY_FLOAT,qtrt::FFITY_FLOAT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qmatrix4x4.h:133
// index:0
// Public Visibility=Default Availability=Available
// [-2] void rotate(float, const QVector3D &)

/*
Multiples this matrix by another that rotates coordinates through angle degrees about vector.

See also scale() and translate().
*/
impl /*struct*/ QMatrix4x4 {
  pub fn rotate_0<RetType, T: QMatrix4x4_rotate_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rotate_0(self);
    // return 1;
  }
}
pub trait QMatrix4x4_rotate_0<RetType> {
  fn rotate_0(self , rsthis: & QMatrix4x4) -> RetType;
}
impl<'a> /*trait*/ QMatrix4x4_rotate_0<(/*void*/)> for (f32,usize) {
  fn rotate_0(self , rsthis: & QMatrix4x4) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN10QMatrix4x46rotateEfRK9QVector3D", 2,qtrt::FFITY_FLOAT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qmatrix4x4.h:140
// index:1
// Public Visibility=Default Availability=Available
// [-2] void rotate(float, float, float, float)

/*
Multiples this matrix by another that rotates coordinates through angle degrees about vector.

See also scale() and translate().
*/
impl /*struct*/ QMatrix4x4 {
  pub fn rotate_1<RetType, T: QMatrix4x4_rotate_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rotate_1(self);
    // return 1;
  }
}
pub trait QMatrix4x4_rotate_1<RetType> {
  fn rotate_1(self , rsthis: & QMatrix4x4) -> RetType;
}
impl<'a> /*trait*/ QMatrix4x4_rotate_1<(/*void*/)> for (f32,f32,f32,f32) {
  fn rotate_1(self , rsthis: & QMatrix4x4) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f32 as usize;
    let arg1 = (&self.1) as *const f32 as usize;
    let arg2 = (&self.2) as *const f32 as usize;
    let arg3 = (&self.3) as *const f32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QMatrix4x46rotateEffff", 4,qtrt::FFITY_FLOAT,qtrt::FFITY_FLOAT,qtrt::FFITY_FLOAT,qtrt::FFITY_FLOAT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qmatrix4x4.h:142
// index:2
// Public Visibility=Default Availability=Available
// [-2] void rotate(const QQuaternion &)

/*
Multiples this matrix by another that rotates coordinates through angle degrees about vector.

See also scale() and translate().
*/
impl /*struct*/ QMatrix4x4 {
  pub fn rotate_2<RetType, T: QMatrix4x4_rotate_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rotate_2(self);
    // return 1;
  }
}
pub trait QMatrix4x4_rotate_2<RetType> {
  fn rotate_2(self , rsthis: & QMatrix4x4) -> RetType;
}
impl<'a> /*trait*/ QMatrix4x4_rotate_2<(/*void*/)> for (usize) {
  fn rotate_2(self , rsthis: & QMatrix4x4) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN10QMatrix4x46rotateERK11QQuaternion", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qmatrix4x4.h:145
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ortho(const QRect &)

/*
Multiplies this matrix by another that applies an orthographic projection for a window with lower-left corner (left, bottom), upper-right corner (right, top), and the specified nearPlane and farPlane clipping planes.

See also frustum() and perspective().
*/
impl /*struct*/ QMatrix4x4 {
  pub fn ortho_0<RetType, T: QMatrix4x4_ortho_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.ortho_0(self);
    // return 1;
  }
}
pub trait QMatrix4x4_ortho_0<RetType> {
  fn ortho_0(self , rsthis: & QMatrix4x4) -> RetType;
}
impl<'a> /*trait*/ QMatrix4x4_ortho_0<(/*void*/)> for (usize) {
  fn ortho_0(self , rsthis: & QMatrix4x4) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN10QMatrix4x45orthoERK5QRect", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qmatrix4x4.h:146
// index:1
// Public Visibility=Default Availability=Available
// [-2] void ortho(const QRectF &)

/*
Multiplies this matrix by another that applies an orthographic projection for a window with lower-left corner (left, bottom), upper-right corner (right, top), and the specified nearPlane and farPlane clipping planes.

See also frustum() and perspective().
*/
impl /*struct*/ QMatrix4x4 {
  pub fn ortho_1<RetType, T: QMatrix4x4_ortho_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.ortho_1(self);
    // return 1;
  }
}
pub trait QMatrix4x4_ortho_1<RetType> {
  fn ortho_1(self , rsthis: & QMatrix4x4) -> RetType;
}
impl<'a> /*trait*/ QMatrix4x4_ortho_1<(/*void*/)> for (usize) {
  fn ortho_1(self , rsthis: & QMatrix4x4) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN10QMatrix4x45orthoERK6QRectF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qmatrix4x4.h:147
// index:2
// Public Visibility=Default Availability=Available
// [-2] void ortho(float, float, float, float, float, float)

/*
Multiplies this matrix by another that applies an orthographic projection for a window with lower-left corner (left, bottom), upper-right corner (right, top), and the specified nearPlane and farPlane clipping planes.

See also frustum() and perspective().
*/
impl /*struct*/ QMatrix4x4 {
  pub fn ortho_2<RetType, T: QMatrix4x4_ortho_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.ortho_2(self);
    // return 1;
  }
}
pub trait QMatrix4x4_ortho_2<RetType> {
  fn ortho_2(self , rsthis: & QMatrix4x4) -> RetType;
}
impl<'a> /*trait*/ QMatrix4x4_ortho_2<(/*void*/)> for (f32,f32,f32,f32,f32,f32) {
  fn ortho_2(self , rsthis: & QMatrix4x4) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f32 as usize;
    let arg1 = (&self.1) as *const f32 as usize;
    let arg2 = (&self.2) as *const f32 as usize;
    let arg3 = (&self.3) as *const f32 as usize;
    let arg4 = (&self.4) as *const f32 as usize;
    let arg5 = (&self.5) as *const f32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QMatrix4x45orthoEffffff", 6,qtrt::FFITY_FLOAT,qtrt::FFITY_FLOAT,qtrt::FFITY_FLOAT,qtrt::FFITY_FLOAT,qtrt::FFITY_FLOAT,qtrt::FFITY_FLOAT,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,arg5,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qmatrix4x4.h:148
// index:0
// Public Visibility=Default Availability=Available
// [-2] void frustum(float, float, float, float, float, float)

/*
Multiplies this matrix by another that applies a perspective frustum projection for a window with lower-left corner (left, bottom), upper-right corner (right, top), and the specified nearPlane and farPlane clipping planes.

See also ortho() and perspective().
*/
impl /*struct*/ QMatrix4x4 {
  pub fn frustum_0<RetType, T: QMatrix4x4_frustum_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.frustum_0(self);
    // return 1;
  }
}
pub trait QMatrix4x4_frustum_0<RetType> {
  fn frustum_0(self , rsthis: & QMatrix4x4) -> RetType;
}
impl<'a> /*trait*/ QMatrix4x4_frustum_0<(/*void*/)> for (f32,f32,f32,f32,f32,f32) {
  fn frustum_0(self , rsthis: & QMatrix4x4) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f32 as usize;
    let arg1 = (&self.1) as *const f32 as usize;
    let arg2 = (&self.2) as *const f32 as usize;
    let arg3 = (&self.3) as *const f32 as usize;
    let arg4 = (&self.4) as *const f32 as usize;
    let arg5 = (&self.5) as *const f32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QMatrix4x47frustumEffffff", 6,qtrt::FFITY_FLOAT,qtrt::FFITY_FLOAT,qtrt::FFITY_FLOAT,qtrt::FFITY_FLOAT,qtrt::FFITY_FLOAT,qtrt::FFITY_FLOAT,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,arg5,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qmatrix4x4.h:149
// index:0
// Public Visibility=Default Availability=Available
// [-2] void perspective(float, float, float, float)

/*
Multiplies this matrix by another that applies a perspective projection. The vertical field of view will be verticalAngle degrees within a window with a given aspectRatio that determines the horizontal field of view. The projection will have the specified nearPlane and farPlane clipping planes which are the distances from the viewer to the corresponding planes.

See also ortho() and frustum().
*/
impl /*struct*/ QMatrix4x4 {
  pub fn perspective_0<RetType, T: QMatrix4x4_perspective_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.perspective_0(self);
    // return 1;
  }
}
pub trait QMatrix4x4_perspective_0<RetType> {
  fn perspective_0(self , rsthis: & QMatrix4x4) -> RetType;
}
impl<'a> /*trait*/ QMatrix4x4_perspective_0<(/*void*/)> for (f32,f32,f32,f32) {
  fn perspective_0(self , rsthis: & QMatrix4x4) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f32 as usize;
    let arg1 = (&self.1) as *const f32 as usize;
    let arg2 = (&self.2) as *const f32 as usize;
    let arg3 = (&self.3) as *const f32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QMatrix4x411perspectiveEffff", 4,qtrt::FFITY_FLOAT,qtrt::FFITY_FLOAT,qtrt::FFITY_FLOAT,qtrt::FFITY_FLOAT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qmatrix4x4.h:151
// index:0
// Public Visibility=Default Availability=Available
// [-2] void lookAt(const QVector3D &, const QVector3D &, const QVector3D &)

/*
Multiplies this matrix by a viewing matrix derived from an eye point. The center value indicates the center of the view that the eye is looking at. The up value indicates which direction should be considered up with respect to the eye.

Note: The up vector must not be parallel to the line of sight from eye to center.
*/
impl /*struct*/ QMatrix4x4 {
  pub fn lookAt_0<RetType, T: QMatrix4x4_lookAt_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.lookAt_0(self);
    // return 1;
  }
}
pub trait QMatrix4x4_lookAt_0<RetType> {
  fn lookAt_0(self , rsthis: & QMatrix4x4) -> RetType;
}
impl<'a> /*trait*/ QMatrix4x4_lookAt_0<(/*void*/)> for (usize,usize,usize) {
  fn lookAt_0(self , rsthis: & QMatrix4x4) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN10QMatrix4x46lookAtERK9QVector3DS2_S2_", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qmatrix4x4.h:153
// index:0
// Public Visibility=Default Availability=Available
// [-2] void viewport(const QRectF &)

/*
Multiplies this matrix by another that performs the scale and bias transformation used by OpenGL to transform from normalized device coordinates (NDC) to viewport (window) coordinates. That is it maps points from the cube ranging over [-1, 1] in each dimension to the viewport with it's near-lower-left corner at (left, bottom, nearPlane) and with size (width, height, farPlane - nearPlane).

This matches the transform used by the fixed function OpenGL viewport transform controlled by the functions glViewport() and glDepthRange().
*/
impl /*struct*/ QMatrix4x4 {
  pub fn viewport_0<RetType, T: QMatrix4x4_viewport_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.viewport_0(self);
    // return 1;
  }
}
pub trait QMatrix4x4_viewport_0<RetType> {
  fn viewport_0(self , rsthis: & QMatrix4x4) -> RetType;
}
impl<'a> /*trait*/ QMatrix4x4_viewport_0<(/*void*/)> for (usize) {
  fn viewport_0(self , rsthis: & QMatrix4x4) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN10QMatrix4x48viewportERK6QRectF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qmatrix4x4.h:154
// index:1
// Public Visibility=Default Availability=Available
// [-2] void viewport(float, float, float, float, float, float)

/*
Multiplies this matrix by another that performs the scale and bias transformation used by OpenGL to transform from normalized device coordinates (NDC) to viewport (window) coordinates. That is it maps points from the cube ranging over [-1, 1] in each dimension to the viewport with it's near-lower-left corner at (left, bottom, nearPlane) and with size (width, height, farPlane - nearPlane).

This matches the transform used by the fixed function OpenGL viewport transform controlled by the functions glViewport() and glDepthRange().
*/
impl /*struct*/ QMatrix4x4 {
  pub fn viewport_1<RetType, T: QMatrix4x4_viewport_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.viewport_1(self);
    // return 1;
  }
}
pub trait QMatrix4x4_viewport_1<RetType> {
  fn viewport_1(self , rsthis: & QMatrix4x4) -> RetType;
}
impl<'a> /*trait*/ QMatrix4x4_viewport_1<(/*void*/)> for (f32,f32,f32,f32,f32,f32) {
  fn viewport_1(self , rsthis: & QMatrix4x4) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f32 as usize;
    let arg1 = (&self.1) as *const f32 as usize;
    let arg2 = (&self.2) as *const f32 as usize;
    let arg3 = (&self.3) as *const f32 as usize;
    let arg4 = (&self.4) as *const f32 as usize;
    let arg5 = (&self.5) as *const f32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QMatrix4x48viewportEffffff", 6,qtrt::FFITY_FLOAT,qtrt::FFITY_FLOAT,qtrt::FFITY_FLOAT,qtrt::FFITY_FLOAT,qtrt::FFITY_FLOAT,qtrt::FFITY_FLOAT,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,arg5,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qmatrix4x4.h:155
// index:0
// Public Visibility=Default Availability=Available
// [-2] void flipCoordinates()

/*

*/
impl /*struct*/ QMatrix4x4 {
  pub fn flipCoordinates_0<RetType, T: QMatrix4x4_flipCoordinates_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.flipCoordinates_0(self);
    // return 1;
  }
}
pub trait QMatrix4x4_flipCoordinates_0<RetType> {
  fn flipCoordinates_0(self , rsthis: & QMatrix4x4) -> RetType;
}
impl<'a> /*trait*/ QMatrix4x4_flipCoordinates_0<(/*void*/)> for () {
  fn flipCoordinates_0(self , rsthis: & QMatrix4x4) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN10QMatrix4x415flipCoordinatesEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qmatrix4x4.h:157
// index:0
// Public Visibility=Default Availability=Available
// [-2] void copyDataTo(float *) const

/*
Retrieves the 16 items in this matrix and copies them to values in row-major order.
*/
impl /*struct*/ QMatrix4x4 {
  pub fn copyDataTo_0<RetType, T: QMatrix4x4_copyDataTo_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.copyDataTo_0(self);
    // return 1;
  }
}
pub trait QMatrix4x4_copyDataTo_0<RetType> {
  fn copyDataTo_0(self , rsthis: & QMatrix4x4) -> RetType;
}
impl<'a> /*trait*/ QMatrix4x4_copyDataTo_0<(/*void*/)> for (usize) {
  fn copyDataTo_0(self , rsthis: & QMatrix4x4) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK10QMatrix4x410copyDataToEPf", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qmatrix4x4.h:159
// index:0
// Public Visibility=Default Availability=Available
// [48] QMatrix toAffine() const

/*
Returns the conventional Qt 2D affine transformation matrix that corresponds to this matrix. It is assumed that this matrix only contains 2D affine transformation elements.

See also toTransform().
*/
impl /*struct*/ QMatrix4x4 {
  pub fn toAffine_0<RetType, T: QMatrix4x4_toAffine_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toAffine_0(self);
    // return 1;
  }
}
pub trait QMatrix4x4_toAffine_0<RetType> {
  fn toAffine_0(self , rsthis: & QMatrix4x4) -> RetType;
}
impl<'a> /*trait*/ QMatrix4x4_toAffine_0<usize> for () {
  fn toAffine_0(self , rsthis: & QMatrix4x4) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QMatrix4x48toAffineEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qmatrix4x4.h:160
// index:0
// Public Visibility=Default Availability=Available
// [88] QTransform toTransform() const

/*
Returns the conventional Qt 2D transformation matrix that corresponds to this matrix.

The returned QTransform is formed by simply dropping the third row and third column of the QMatrix4x4. This is suitable for implementing orthographic projections where the z co-ordinate should be dropped rather than projected.

See also toAffine().
*/
impl /*struct*/ QMatrix4x4 {
  pub fn toTransform_0<RetType, T: QMatrix4x4_toTransform_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toTransform_0(self);
    // return 1;
  }
}
pub trait QMatrix4x4_toTransform_0<RetType> {
  fn toTransform_0(self , rsthis: & QMatrix4x4) -> RetType;
}
impl<'a> /*trait*/ QMatrix4x4_toTransform_0<usize> for () {
  fn toTransform_0(self , rsthis: & QMatrix4x4) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QMatrix4x411toTransformEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qmatrix4x4.h:161
// index:1
// Public Visibility=Default Availability=Available
// [88] QTransform toTransform(float) const

/*
Returns the conventional Qt 2D transformation matrix that corresponds to this matrix.

The returned QTransform is formed by simply dropping the third row and third column of the QMatrix4x4. This is suitable for implementing orthographic projections where the z co-ordinate should be dropped rather than projected.

See also toAffine().
*/
impl /*struct*/ QMatrix4x4 {
  pub fn toTransform_1<RetType, T: QMatrix4x4_toTransform_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toTransform_1(self);
    // return 1;
  }
}
pub trait QMatrix4x4_toTransform_1<RetType> {
  fn toTransform_1(self , rsthis: & QMatrix4x4) -> RetType;
}
impl<'a> /*trait*/ QMatrix4x4_toTransform_1<usize> for (f32) {
  fn toTransform_1(self , rsthis: & QMatrix4x4) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QMatrix4x411toTransformEf", 1,qtrt::FFITY_FLOAT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qmatrix4x4.h:163
// index:0
// Public Visibility=Default Availability=Available
// [8] QPoint map(const QPoint &) const

/*
Maps point by multiplying this matrix by point.

See also mapRect().
*/
impl /*struct*/ QMatrix4x4 {
  pub fn map__0<RetType, T: QMatrix4x4_map__0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.map__0(self);
    // return 1;
  }
}
pub trait QMatrix4x4_map__0<RetType> {
  fn map__0(self , rsthis: & QMatrix4x4) -> RetType;
}
impl<'a> /*trait*/ QMatrix4x4_map__0<usize> for (usize) {
  fn map__0(self , rsthis: & QMatrix4x4) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QMatrix4x43mapERK6QPoint", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qmatrix4x4.h:164
// index:1
// Public Visibility=Default Availability=Available
// [16] QPointF map(const QPointF &) const

/*
Maps point by multiplying this matrix by point.

See also mapRect().
*/
impl /*struct*/ QMatrix4x4 {
  pub fn map__1<RetType, T: QMatrix4x4_map__1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.map__1(self);
    // return 1;
  }
}
pub trait QMatrix4x4_map__1<RetType> {
  fn map__1(self , rsthis: & QMatrix4x4) -> RetType;
}
impl<'a> /*trait*/ QMatrix4x4_map__1<usize> for (usize) {
  fn map__1(self , rsthis: & QMatrix4x4) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QMatrix4x43mapERK7QPointF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qmatrix4x4.h:166
// index:2
// Public Visibility=Default Availability=Available
// [12] QVector3D map(const QVector3D &) const

/*
Maps point by multiplying this matrix by point.

See also mapRect().
*/
impl /*struct*/ QMatrix4x4 {
  pub fn map__2<RetType, T: QMatrix4x4_map__2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.map__2(self);
    // return 1;
  }
}
pub trait QMatrix4x4_map__2<RetType> {
  fn map__2(self , rsthis: & QMatrix4x4) -> RetType;
}
impl<'a> /*trait*/ QMatrix4x4_map__2<usize> for (usize) {
  fn map__2(self , rsthis: & QMatrix4x4) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QMatrix4x43mapERK9QVector3D", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qmatrix4x4.h:170
// index:3
// Public Visibility=Default Availability=Available
// [16] QVector4D map(const QVector4D &) const

/*
Maps point by multiplying this matrix by point.

See also mapRect().
*/
impl /*struct*/ QMatrix4x4 {
  pub fn map__3<RetType, T: QMatrix4x4_map__3<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.map__3(self);
    // return 1;
  }
}
pub trait QMatrix4x4_map__3<RetType> {
  fn map__3(self , rsthis: & QMatrix4x4) -> RetType;
}
impl<'a> /*trait*/ QMatrix4x4_map__3<usize> for (usize) {
  fn map__3(self , rsthis: & QMatrix4x4) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QMatrix4x43mapERK9QVector4D", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qmatrix4x4.h:167
// index:0
// Public Visibility=Default Availability=Available
// [12] QVector3D mapVector(const QVector3D &) const

/*
Maps vector by multiplying the top 3x3 portion of this matrix by vector. The translation and projection components of this matrix are ignored.

See also map().
*/
impl /*struct*/ QMatrix4x4 {
  pub fn mapVector_0<RetType, T: QMatrix4x4_mapVector_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mapVector_0(self);
    // return 1;
  }
}
pub trait QMatrix4x4_mapVector_0<RetType> {
  fn mapVector_0(self , rsthis: & QMatrix4x4) -> RetType;
}
impl<'a> /*trait*/ QMatrix4x4_mapVector_0<usize> for (usize) {
  fn mapVector_0(self , rsthis: & QMatrix4x4) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QMatrix4x49mapVectorERK9QVector3D", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qmatrix4x4.h:172
// index:0
// Public Visibility=Default Availability=Available
// [16] QRect mapRect(const QRect &) const

/*
Maps rect by multiplying this matrix by the corners of rect and then forming a new rectangle from the results. The returned rectangle will be an ordinary 2D rectangle with sides parallel to the horizontal and vertical axes.

See also map().
*/
impl /*struct*/ QMatrix4x4 {
  pub fn mapRect_0<RetType, T: QMatrix4x4_mapRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mapRect_0(self);
    // return 1;
  }
}
pub trait QMatrix4x4_mapRect_0<RetType> {
  fn mapRect_0(self , rsthis: & QMatrix4x4) -> RetType;
}
impl<'a> /*trait*/ QMatrix4x4_mapRect_0<usize> for (usize) {
  fn mapRect_0(self , rsthis: & QMatrix4x4) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QMatrix4x47mapRectERK5QRect", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qmatrix4x4.h:173
// index:1
// Public Visibility=Default Availability=Available
// [32] QRectF mapRect(const QRectF &) const

/*
Maps rect by multiplying this matrix by the corners of rect and then forming a new rectangle from the results. The returned rectangle will be an ordinary 2D rectangle with sides parallel to the horizontal and vertical axes.

See also map().
*/
impl /*struct*/ QMatrix4x4 {
  pub fn mapRect_1<RetType, T: QMatrix4x4_mapRect_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mapRect_1(self);
    // return 1;
  }
}
pub trait QMatrix4x4_mapRect_1<RetType> {
  fn mapRect_1(self , rsthis: & QMatrix4x4) -> RetType;
}
impl<'a> /*trait*/ QMatrix4x4_mapRect_1<usize> for (usize) {
  fn mapRect_1(self , rsthis: & QMatrix4x4) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QMatrix4x47mapRectERK6QRectF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qmatrix4x4.h:178
// index:0
// Public inline Visibility=Default Availability=Available
// [8] float * data()

/*
Returns a pointer to the raw data of this matrix.

See also constData() and optimize().
*/
impl /*struct*/ QMatrix4x4 {
  pub fn data_0<RetType, T: QMatrix4x4_data_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.data_0(self);
    // return 1;
  }
}
pub trait QMatrix4x4_data_0<RetType> {
  fn data_0(self , rsthis: & QMatrix4x4) -> RetType;
}
impl<'a> /*trait*/ QMatrix4x4_data_0<usize> for () {
  fn data_0(self , rsthis: & QMatrix4x4) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QMatrix4x44dataEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qmatrix4x4.h:179
// index:1
// Public inline Visibility=Default Availability=Available
// [8] const float * data() const

/*
Returns a pointer to the raw data of this matrix.

See also constData() and optimize().
*/
impl /*struct*/ QMatrix4x4 {
  pub fn data_1<RetType, T: QMatrix4x4_data_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.data_1(self);
    // return 1;
  }
}
pub trait QMatrix4x4_data_1<RetType> {
  fn data_1(self , rsthis: & QMatrix4x4) -> RetType;
}
impl<'a> /*trait*/ QMatrix4x4_data_1<usize> for () {
  fn data_1(self , rsthis: & QMatrix4x4) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QMatrix4x44dataEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qmatrix4x4.h:180
// index:0
// Public inline Visibility=Default Availability=Available
// [8] const float * constData() const

/*
Returns a constant pointer to the raw data of this matrix. This raw data is stored in column-major format.

See also data().
*/
impl /*struct*/ QMatrix4x4 {
  pub fn constData_0<RetType, T: QMatrix4x4_constData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.constData_0(self);
    // return 1;
  }
}
pub trait QMatrix4x4_constData_0<RetType> {
  fn constData_0(self , rsthis: & QMatrix4x4) -> RetType;
}
impl<'a> /*trait*/ QMatrix4x4_constData_0<usize> for () {
  fn constData_0(self , rsthis: & QMatrix4x4) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QMatrix4x49constDataEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qmatrix4x4.h:182
// index:0
// Public Visibility=Default Availability=Available
// [-2] void optimize()

/*
Optimize the usage of this matrix from its current elements.

Some operations such as translate(), scale(), and rotate() can be performed more efficiently if the matrix being modified is already known to be the identity, a previous translate(), a previous scale(), etc.

Normally the QMatrix4x4 class keeps track of this special type internally as operations are performed. However, if the matrix is modified directly with {QLoggingCategory::operator()}{operator()()} or data(), then QMatrix4x4 will lose track of the special type and will revert to the safest but least efficient operations thereafter.

By calling optimize() after directly modifying the matrix, the programmer can force QMatrix4x4 to recover the special type if the elements appear to conform to one of the known optimized types.

See also operator()(), data(), and translate().
*/
impl /*struct*/ QMatrix4x4 {
  pub fn optimize_0<RetType, T: QMatrix4x4_optimize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.optimize_0(self);
    // return 1;
  }
}
pub trait QMatrix4x4_optimize_0<RetType> {
  fn optimize_0(self , rsthis: & QMatrix4x4) -> RetType;
}
impl<'a> /*trait*/ QMatrix4x4_optimize_0<(/*void*/)> for () {
  fn optimize_0(self , rsthis: & QMatrix4x4) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN10QMatrix4x48optimizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}


pub fn DeleteQMatrix4x4(this :*mut QMatrix4x4) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN10QMatrix4x4D2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}

/*


*/
pub type QMatrix4x4__ = i32;
// 
pub const QMatrix4x4__Identity :QMatrix4x4__ = 0;
// 
pub const QMatrix4x4__Translation :QMatrix4x4__ = 1;
// 
pub const QMatrix4x4__Scale :QMatrix4x4__ = 2;
// 
pub const QMatrix4x4__Rotation2D :QMatrix4x4__ = 4;
// 
pub const QMatrix4x4__Rotation :QMatrix4x4__ = 8;
// 
pub const QMatrix4x4__Perspective :QMatrix4x4__ = 16;
// 
pub const QMatrix4x4__General :QMatrix4x4__ = 31;
pub fn QMatrix4x4_ItemName(val: i32) ->String {
  match val {
     QMatrix4x4__Identity => // 0
     {return String::from("Identity");}
     QMatrix4x4__Translation => // 1
     {return String::from("Translation");}
     QMatrix4x4__Scale => // 2
     {return String::from("Scale");}
     QMatrix4x4__Rotation2D => // 4
     {return String::from("Rotation2D");}
     QMatrix4x4__Rotation => // 8
     {return String::from("Rotation");}
     QMatrix4x4__Perspective => // 16
     {return String::from("Perspective");}
     QMatrix4x4__General => // 31
     {return String::from("General");}
  _ => {return format!("{}", val);}
}
}
pub fn QMatrix4x4_ItemName_s(val: i32) ->String {
  //var nilthis *QMatrix4x4
  //return nilthis.ItemName(val);
  return QMatrix4x4_ItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
