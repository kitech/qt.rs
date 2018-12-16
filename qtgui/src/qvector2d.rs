

// mod ::gui::QVector2D
// package qtgui
// /usr/include/qt/QtGui/qvector2d.h
// #include <qvector2d.h>
// #include <QtGui>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 22
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
#[derive(Default)] // class sizeof(QVector2D)=8
pub struct QVector2D {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QVector2D_ITF interface {
//    QVector2D_PTR() *QVector2D
//}
//func (ptr *QVector2D) QVector2D_PTR() *QVector2D { return ptr }

impl /*struct*/ QVector2D {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QVector2D {
    return QVector2D{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QVector2D {
//  type Target = QVector2DBASE;
//
//  fn deref(&self) -> &QVector2DBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QVector2DBASE> for QVector2D {
//  fn as_ref(& self) -> & QVector2DBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qvector2d.h:59
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void QVector2D()

/*
Constructs a null vector, i.e. with coordinates (0, 0).
*/
// QVector2D() ctx.fn_proto_cpp
impl /*struct*/ QVector2D {
  pub fn QVector2D_0<T: QVector2D_QVector2D_0>(value: T) -> QVector2D {
    let rsthis = value.QVector2D_0();
    return rsthis;
    // return 1;
  }
}

pub trait QVector2D_QVector2D_0 {
  fn QVector2D_0(self) -> QVector2D;
}
// QVector2D() ctx.fn_proto_cpp
impl<'a> /*trait*/ QVector2D_QVector2D_0 for () {
  fn QVector2D_0(self) -> QVector2D {
    // unsafe{_ZN9QVector2DC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QVector2DC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QVector2D{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qvector2d.h:60
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void QVector2D(Qt::Initialization)

/*
Constructs a null vector, i.e. with coordinates (0, 0).
*/
// QVector2D(Qt::Initialization) ctx.fn_proto_cpp
impl /*struct*/ QVector2D {
  pub fn QVector2D_1<T: QVector2D_QVector2D_1>(value: T) -> QVector2D {
    let rsthis = value.QVector2D_1();
    return rsthis;
    // return 1;
  }
}

pub trait QVector2D_QVector2D_1 {
  fn QVector2D_1(self) -> QVector2D;
}
// QVector2D(Qt::Initialization) ctx.fn_proto_cpp
impl<'a> /*trait*/ QVector2D_QVector2D_1 for (i32) {
  fn QVector2D_1(self) -> QVector2D {
    // unsafe{_ZN9QVector2DC2EN2Qt14InitializationE()};
    let arg0 = (&self) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QVector2DC2EN2Qt14InitializationE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QVector2D{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qvector2d.h:61
// index:2
// Public inline Visibility=Default Availability=Available
// [-2] void QVector2D(float, float)

/*
Constructs a null vector, i.e. with coordinates (0, 0).
*/
// QVector2D(float, float) ctx.fn_proto_cpp
impl /*struct*/ QVector2D {
  pub fn QVector2D_2<T: QVector2D_QVector2D_2>(value: T) -> QVector2D {
    let rsthis = value.QVector2D_2();
    return rsthis;
    // return 1;
  }
}

pub trait QVector2D_QVector2D_2 {
  fn QVector2D_2(self) -> QVector2D;
}
// QVector2D(float, float) ctx.fn_proto_cpp
impl<'a> /*trait*/ QVector2D_QVector2D_2 for (f32,f32) {
  fn QVector2D_2(self) -> QVector2D {
    // unsafe{_ZN9QVector2DC2Eff()};
    let arg0 = (&self.0) as *const f32 as usize;
    let arg1 = (&self.1) as *const f32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QVector2DC2Eff", 2,qtrt::FFITY_FLOAT,qtrt::FFITY_FLOAT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QVector2D{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qvector2d.h:62
// index:3
// Public inline Visibility=Default Availability=Available
// [-2] void QVector2D(const QPoint &)

/*
Constructs a null vector, i.e. with coordinates (0, 0).
*/
// QVector2D(const QPoint &) ctx.fn_proto_cpp
impl /*struct*/ QVector2D {
  pub fn QVector2D_3<T: QVector2D_QVector2D_3>(value: T) -> QVector2D {
    let rsthis = value.QVector2D_3();
    return rsthis;
    // return 1;
  }
}

pub trait QVector2D_QVector2D_3 {
  fn QVector2D_3(self) -> QVector2D;
}
// QVector2D(const QPoint &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QVector2D_QVector2D_3 for (usize) {
  fn QVector2D_3(self) -> QVector2D {
    // unsafe{_ZN9QVector2DC2ERK6QPoint()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QVector2DC2ERK6QPoint", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QVector2D{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qvector2d.h:63
// index:4
// Public inline Visibility=Default Availability=Available
// [-2] void QVector2D(const QPointF &)

/*
Constructs a null vector, i.e. with coordinates (0, 0).
*/
// QVector2D(const QPointF &) ctx.fn_proto_cpp
impl /*struct*/ QVector2D {
  pub fn QVector2D_4<T: QVector2D_QVector2D_4>(value: T) -> QVector2D {
    let rsthis = value.QVector2D_4();
    return rsthis;
    // return 1;
  }
}

pub trait QVector2D_QVector2D_4 {
  fn QVector2D_4(self) -> QVector2D;
}
// QVector2D(const QPointF &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QVector2D_QVector2D_4 for (usize) {
  fn QVector2D_4(self) -> QVector2D {
    // unsafe{_ZN9QVector2DC2ERK7QPointF()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QVector2DC2ERK7QPointF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QVector2D{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qvector2d.h:65
// index:5
// Public Visibility=Default Availability=Available
// [-2] void QVector2D(const QVector3D &)

/*
Constructs a null vector, i.e. with coordinates (0, 0).
*/
// QVector2D(const QVector3D &) ctx.fn_proto_cpp
impl /*struct*/ QVector2D {
  pub fn QVector2D_5<T: QVector2D_QVector2D_5>(value: T) -> QVector2D {
    let rsthis = value.QVector2D_5();
    return rsthis;
    // return 1;
  }
}

pub trait QVector2D_QVector2D_5 {
  fn QVector2D_5(self) -> QVector2D;
}
// QVector2D(const QVector3D &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QVector2D_QVector2D_5 for (usize) {
  fn QVector2D_5(self) -> QVector2D {
    // unsafe{_ZN9QVector2DC2ERK9QVector3D()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QVector2DC2ERK9QVector3D", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QVector2D{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qvector2d.h:68
// index:6
// Public Visibility=Default Availability=Available
// [-2] void QVector2D(const QVector4D &)

/*
Constructs a null vector, i.e. with coordinates (0, 0).
*/
// QVector2D(const QVector4D &) ctx.fn_proto_cpp
impl /*struct*/ QVector2D {
  pub fn QVector2D_6<T: QVector2D_QVector2D_6>(value: T) -> QVector2D {
    let rsthis = value.QVector2D_6();
    return rsthis;
    // return 1;
  }
}

pub trait QVector2D_QVector2D_6 {
  fn QVector2D_6(self) -> QVector2D;
}
// QVector2D(const QVector4D &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QVector2D_QVector2D_6 for (usize) {
  fn QVector2D_6(self) -> QVector2D {
    // unsafe{_ZN9QVector2DC2ERK9QVector4D()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QVector2DC2ERK9QVector4D", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QVector2D{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qvector2d.h:71
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isNull() const

/*
Returns true if the x and y coordinates are set to 0.0, otherwise returns false.
*/
impl /*struct*/ QVector2D {
  pub fn isNull_0<RetType, T: QVector2D_isNull_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isNull_0(self);
    // return 1;
  }
}
pub trait QVector2D_isNull_0<RetType> {
  fn isNull_0(self , rsthis: & QVector2D) -> RetType;
}
impl<'a> /*trait*/ QVector2D_isNull_0<bool> for () {
  fn isNull_0(self , rsthis: & QVector2D) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QVector2D6isNullEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qvector2d.h:73
// index:0
// Public inline Visibility=Default Availability=Available
// [4] float x() const

/*
Returns the x coordinate of this point.

See also setX() and y().
*/
impl /*struct*/ QVector2D {
  pub fn x_0<RetType, T: QVector2D_x_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.x_0(self);
    // return 1;
  }
}
pub trait QVector2D_x_0<RetType> {
  fn x_0(self , rsthis: & QVector2D) -> RetType;
}
impl<'a> /*trait*/ QVector2D_x_0<f32> for () {
  fn x_0(self , rsthis: & QVector2D) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QVector2D1xEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qvector2d.h:74
// index:0
// Public inline Visibility=Default Availability=Available
// [4] float y() const

/*
Returns the y coordinate of this point.

See also setY() and x().
*/
impl /*struct*/ QVector2D {
  pub fn y_0<RetType, T: QVector2D_y_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.y_0(self);
    // return 1;
  }
}
pub trait QVector2D_y_0<RetType> {
  fn y_0(self , rsthis: & QVector2D) -> RetType;
}
impl<'a> /*trait*/ QVector2D_y_0<f32> for () {
  fn y_0(self , rsthis: & QVector2D) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QVector2D1yEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qvector2d.h:76
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setX(float)

/*
Sets the x coordinate of this point to the given x coordinate.

See also x() and setY().
*/
impl /*struct*/ QVector2D {
  pub fn setX_0<RetType, T: QVector2D_setX_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setX_0(self);
    // return 1;
  }
}
pub trait QVector2D_setX_0<RetType> {
  fn setX_0(self , rsthis: & QVector2D) -> RetType;
}
impl<'a> /*trait*/ QVector2D_setX_0<(/*void*/)> for (f32) {
  fn setX_0(self , rsthis: & QVector2D) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QVector2D4setXEf", 1,qtrt::FFITY_FLOAT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qvector2d.h:77
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setY(float)

/*
Sets the y coordinate of this point to the given y coordinate.

See also y() and setX().
*/
impl /*struct*/ QVector2D {
  pub fn setY_0<RetType, T: QVector2D_setY_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setY_0(self);
    // return 1;
  }
}
pub trait QVector2D_setY_0<RetType> {
  fn setY_0(self , rsthis: & QVector2D) -> RetType;
}
impl<'a> /*trait*/ QVector2D_setY_0<(/*void*/)> for (f32) {
  fn setY_0(self , rsthis: & QVector2D) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QVector2D4setYEf", 1,qtrt::FFITY_FLOAT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qvector2d.h:79
// index:0
// Public Visibility=Default Availability=Available
// [4] float & operator[](int)

/*

*/
impl /*struct*/ QVector2D {
  pub fn operator_get_index_0<RetType, T: QVector2D_operator_get_index_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_get_index_0(self);
    // return 1;
  }
}
pub trait QVector2D_operator_get_index_0<RetType> {
  fn operator_get_index_0(self , rsthis: & QVector2D) -> RetType;
}
impl<'a> /*trait*/ QVector2D_operator_get_index_0<usize> for (i32) {
  fn operator_get_index_0(self , rsthis: & QVector2D) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QVector2DixEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qvector2d.h:80
// index:1
// Public Visibility=Default Availability=Available
// [4] float operator[](int) const

/*

*/
impl /*struct*/ QVector2D {
  pub fn operator_get_index_1<RetType, T: QVector2D_operator_get_index_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_get_index_1(self);
    // return 1;
  }
}
pub trait QVector2D_operator_get_index_1<RetType> {
  fn operator_get_index_1(self , rsthis: & QVector2D) -> RetType;
}
impl<'a> /*trait*/ QVector2D_operator_get_index_1<f32> for (i32) {
  fn operator_get_index_1(self , rsthis: & QVector2D) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QVector2DixEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qvector2d.h:82
// index:0
// Public Visibility=Default Availability=Available
// [4] float length() const

/*
Returns the length of the vector from the origin.

See also lengthSquared() and normalized().
*/
impl /*struct*/ QVector2D {
  pub fn length_0<RetType, T: QVector2D_length_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.length_0(self);
    // return 1;
  }
}
pub trait QVector2D_length_0<RetType> {
  fn length_0(self , rsthis: & QVector2D) -> RetType;
}
impl<'a> /*trait*/ QVector2D_length_0<f32> for () {
  fn length_0(self , rsthis: & QVector2D) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QVector2D6lengthEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qvector2d.h:83
// index:0
// Public Visibility=Default Availability=Available
// [4] float lengthSquared() const

/*
Returns the squared length of the vector from the origin. This is equivalent to the dot product of the vector with itself.

See also length() and dotProduct().
*/
impl /*struct*/ QVector2D {
  pub fn lengthSquared_0<RetType, T: QVector2D_lengthSquared_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.lengthSquared_0(self);
    // return 1;
  }
}
pub trait QVector2D_lengthSquared_0<RetType> {
  fn lengthSquared_0(self , rsthis: & QVector2D) -> RetType;
}
impl<'a> /*trait*/ QVector2D_lengthSquared_0<f32> for () {
  fn lengthSquared_0(self , rsthis: & QVector2D) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QVector2D13lengthSquaredEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qvector2d.h:85
// index:0
// Public Visibility=Default Availability=Available
// [8] QVector2D normalized() const

/*
Returns the normalized unit vector form of this vector.

If this vector is null, then a null vector is returned. If the length of the vector is very close to 1, then the vector will be returned as-is. Otherwise the normalized form of the vector of length 1 will be returned.

See also length() and normalize().
*/
impl /*struct*/ QVector2D {
  pub fn normalized_0<RetType, T: QVector2D_normalized_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.normalized_0(self);
    // return 1;
  }
}
pub trait QVector2D_normalized_0<RetType> {
  fn normalized_0(self , rsthis: & QVector2D) -> RetType;
}
impl<'a> /*trait*/ QVector2D_normalized_0<usize> for () {
  fn normalized_0(self , rsthis: & QVector2D) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QVector2D10normalizedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qvector2d.h:86
// index:0
// Public Visibility=Default Availability=Available
// [-2] void normalize()

/*
Normalizes the currect vector in place. Nothing happens if this vector is a null vector or the length of the vector is very close to 1.

See also length() and normalized().
*/
impl /*struct*/ QVector2D {
  pub fn normalize_0<RetType, T: QVector2D_normalize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.normalize_0(self);
    // return 1;
  }
}
pub trait QVector2D_normalize_0<RetType> {
  fn normalize_0(self , rsthis: & QVector2D) -> RetType;
}
impl<'a> /*trait*/ QVector2D_normalize_0<(/*void*/)> for () {
  fn normalize_0(self , rsthis: & QVector2D) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN9QVector2D9normalizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qvector2d.h:88
// index:0
// Public Visibility=Default Availability=Available
// [4] float distanceToPoint(const QVector2D &) const

/*
Returns the distance from this vertex to a point defined by the vertex point.

This function was introduced in  Qt 5.1.

See also distanceToLine().
*/
impl /*struct*/ QVector2D {
  pub fn distanceToPoint_0<RetType, T: QVector2D_distanceToPoint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.distanceToPoint_0(self);
    // return 1;
  }
}
pub trait QVector2D_distanceToPoint_0<RetType> {
  fn distanceToPoint_0(self , rsthis: & QVector2D) -> RetType;
}
impl<'a> /*trait*/ QVector2D_distanceToPoint_0<f32> for (usize) {
  fn distanceToPoint_0(self , rsthis: & QVector2D) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QVector2D15distanceToPointERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qvector2d.h:89
// index:0
// Public Visibility=Default Availability=Available
// [4] float distanceToLine(const QVector2D &, const QVector2D &) const

/*
Returns the distance that this vertex is from a line defined by point and the unit vector direction.

If direction is a null vector, then it does not define a line. In that case, the distance from point to this vertex is returned.

This function was introduced in  Qt 5.1.

See also distanceToPoint().
*/
impl /*struct*/ QVector2D {
  pub fn distanceToLine_0<RetType, T: QVector2D_distanceToLine_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.distanceToLine_0(self);
    // return 1;
  }
}
pub trait QVector2D_distanceToLine_0<RetType> {
  fn distanceToLine_0(self , rsthis: & QVector2D) -> RetType;
}
impl<'a> /*trait*/ QVector2D_distanceToLine_0<f32> for (usize,usize) {
  fn distanceToLine_0(self , rsthis: & QVector2D) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QVector2D14distanceToLineERKS_S1_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qvector2d.h:91
// index:0
// Public Visibility=Default Availability=Available
// [8] QVector2D & operator+=(const QVector2D &)

/*

*/
impl /*struct*/ QVector2D {
  pub fn operator_add_equal_0<RetType, T: QVector2D_operator_add_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_add_equal_0(self);
    // return 1;
  }
}
pub trait QVector2D_operator_add_equal_0<RetType> {
  fn operator_add_equal_0(self , rsthis: & QVector2D) -> RetType;
}
impl<'a> /*trait*/ QVector2D_operator_add_equal_0<usize> for (usize) {
  fn operator_add_equal_0(self , rsthis: & QVector2D) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QVector2DpLERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qvector2d.h:92
// index:0
// Public Visibility=Default Availability=Available
// [8] QVector2D & operator-=(const QVector2D &)

/*

*/
impl /*struct*/ QVector2D {
  pub fn operator_minus_equal_0<RetType, T: QVector2D_operator_minus_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_minus_equal_0(self);
    // return 1;
  }
}
pub trait QVector2D_operator_minus_equal_0<RetType> {
  fn operator_minus_equal_0(self , rsthis: & QVector2D) -> RetType;
}
impl<'a> /*trait*/ QVector2D_operator_minus_equal_0<usize> for (usize) {
  fn operator_minus_equal_0(self , rsthis: & QVector2D) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QVector2DmIERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qvector2d.h:93
// index:0
// Public Visibility=Default Availability=Available
// [8] QVector2D & operator*=(float)

/*

*/
impl /*struct*/ QVector2D {
  pub fn operator_mul_equal_0<RetType, T: QVector2D_operator_mul_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_mul_equal_0(self);
    // return 1;
  }
}
pub trait QVector2D_operator_mul_equal_0<RetType> {
  fn operator_mul_equal_0(self , rsthis: & QVector2D) -> RetType;
}
impl<'a> /*trait*/ QVector2D_operator_mul_equal_0<usize> for (f32) {
  fn operator_mul_equal_0(self , rsthis: & QVector2D) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QVector2DmLEf", 1,qtrt::FFITY_FLOAT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qvector2d.h:94
// index:1
// Public Visibility=Default Availability=Available
// [8] QVector2D & operator*=(const QVector2D &)

/*

*/
impl /*struct*/ QVector2D {
  pub fn operator_mul_equal_1<RetType, T: QVector2D_operator_mul_equal_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_mul_equal_1(self);
    // return 1;
  }
}
pub trait QVector2D_operator_mul_equal_1<RetType> {
  fn operator_mul_equal_1(self , rsthis: & QVector2D) -> RetType;
}
impl<'a> /*trait*/ QVector2D_operator_mul_equal_1<usize> for (usize) {
  fn operator_mul_equal_1(self , rsthis: & QVector2D) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QVector2DmLERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qvector2d.h:95
// index:0
// Public Visibility=Default Availability=Available
// [8] QVector2D & operator/=(float)

/*

*/
impl /*struct*/ QVector2D {
  pub fn operator_div_equal_0<RetType, T: QVector2D_operator_div_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_div_equal_0(self);
    // return 1;
  }
}
pub trait QVector2D_operator_div_equal_0<RetType> {
  fn operator_div_equal_0(self , rsthis: & QVector2D) -> RetType;
}
impl<'a> /*trait*/ QVector2D_operator_div_equal_0<usize> for (f32) {
  fn operator_div_equal_0(self , rsthis: & QVector2D) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QVector2DdVEf", 1,qtrt::FFITY_FLOAT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qvector2d.h:96
// index:1
// Public inline Visibility=Default Availability=Available
// [8] QVector2D & operator/=(const QVector2D &)

/*

*/
impl /*struct*/ QVector2D {
  pub fn operator_div_equal_1<RetType, T: QVector2D_operator_div_equal_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_div_equal_1(self);
    // return 1;
  }
}
pub trait QVector2D_operator_div_equal_1<RetType> {
  fn operator_div_equal_1(self , rsthis: & QVector2D) -> RetType;
}
impl<'a> /*trait*/ QVector2D_operator_div_equal_1<usize> for (usize) {
  fn operator_div_equal_1(self , rsthis: & QVector2D) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QVector2DdVERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qvector2d.h:98
// index:0
// Public static Visibility=Default Availability=Available
// [4] float dotProduct(const QVector2D &, const QVector2D &)

/*
Returns the dot product of v1 and v2.
*/
impl /*struct*/ QVector2D {
  pub fn dotProduct_0<RetType, T: QVector2D_dotProduct_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.dotProduct_0();
    // return 1;
  }
}
pub trait QVector2D_dotProduct_0<RetType> {
  fn dotProduct_0(self ) -> RetType;
}
impl<'a> /*trait*/ QVector2D_dotProduct_0<f32> for (usize,usize) {
  fn dotProduct_0(self ) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QVector2D10dotProductERKS_S1_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qvector2d.h:114
// index:0
// Public Visibility=Default Availability=Available
// [12] QVector3D toVector3D() const

/*
Returns the 3D form of this 2D vector, with the z coordinate set to zero.

See also toVector4D() and toPoint().
*/
impl /*struct*/ QVector2D {
  pub fn toVector3D_0<RetType, T: QVector2D_toVector3D_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toVector3D_0(self);
    // return 1;
  }
}
pub trait QVector2D_toVector3D_0<RetType> {
  fn toVector3D_0(self , rsthis: & QVector2D) -> RetType;
}
impl<'a> /*trait*/ QVector2D_toVector3D_0<usize> for () {
  fn toVector3D_0(self , rsthis: & QVector2D) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QVector2D10toVector3DEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qvector2d.h:117
// index:0
// Public Visibility=Default Availability=Available
// [16] QVector4D toVector4D() const

/*
Returns the 4D form of this 2D vector, with the z and w coordinates set to zero.

See also toVector3D() and toPoint().
*/
impl /*struct*/ QVector2D {
  pub fn toVector4D_0<RetType, T: QVector2D_toVector4D_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toVector4D_0(self);
    // return 1;
  }
}
pub trait QVector2D_toVector4D_0<RetType> {
  fn toVector4D_0(self , rsthis: & QVector2D) -> RetType;
}
impl<'a> /*trait*/ QVector2D_toVector4D_0<usize> for () {
  fn toVector4D_0(self , rsthis: & QVector2D) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QVector2D10toVector4DEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qvector2d.h:120
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QPoint toPoint() const

/*
Returns the QPoint form of this 2D vector.

See also toPointF() and toVector3D().
*/
impl /*struct*/ QVector2D {
  pub fn toPoint_0<RetType, T: QVector2D_toPoint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toPoint_0(self);
    // return 1;
  }
}
pub trait QVector2D_toPoint_0<RetType> {
  fn toPoint_0(self , rsthis: & QVector2D) -> RetType;
}
impl<'a> /*trait*/ QVector2D_toPoint_0<usize> for () {
  fn toPoint_0(self , rsthis: & QVector2D) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QVector2D7toPointEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qvector2d.h:121
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QPointF toPointF() const

/*
Returns the QPointF form of this 2D vector.

See also toPoint() and toVector3D().
*/
impl /*struct*/ QVector2D {
  pub fn toPointF_0<RetType, T: QVector2D_toPointF_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toPointF_0(self);
    // return 1;
  }
}
pub trait QVector2D_toPointF_0<RetType> {
  fn toPointF_0(self , rsthis: & QVector2D) -> RetType;
}
impl<'a> /*trait*/ QVector2D_toPointF_0<usize> for () {
  fn toPointF_0(self , rsthis: & QVector2D) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QVector2D8toPointFEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}


pub fn DeleteQVector2D(this :*mut QVector2D) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN9QVector2DD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
