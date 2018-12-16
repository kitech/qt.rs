

// mod ::gui::QVector3D
// package qtgui
// /usr/include/qt/QtGui/qvector3d.h
// #include <qvector3d.h>
// #include <QtGui>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 32
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
#[derive(Default)] // class sizeof(QVector3D)=12
pub struct QVector3D {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QVector3D_ITF interface {
//    QVector3D_PTR() *QVector3D
//}
//func (ptr *QVector3D) QVector3D_PTR() *QVector3D { return ptr }

impl /*struct*/ QVector3D {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QVector3D {
    return QVector3D{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QVector3D {
//  type Target = QVector3DBASE;
//
//  fn deref(&self) -> &QVector3DBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QVector3DBASE> for QVector3D {
//  fn as_ref(& self) -> & QVector3DBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qvector3d.h:60
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void QVector3D()

/*
Constructs a null vector, i.e. with coordinates (0, 0, 0).
*/
// QVector3D() ctx.fn_proto_cpp
impl /*struct*/ QVector3D {
  pub fn QVector3D_0<T: QVector3D_QVector3D_0>(value: T) -> QVector3D {
    let rsthis = value.QVector3D_0();
    return rsthis;
    // return 1;
  }
}

pub trait QVector3D_QVector3D_0 {
  fn QVector3D_0(self) -> QVector3D;
}
// QVector3D() ctx.fn_proto_cpp
impl<'a> /*trait*/ QVector3D_QVector3D_0 for () {
  fn QVector3D_0(self) -> QVector3D {
    // unsafe{_ZN9QVector3DC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QVector3DC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QVector3D{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qvector3d.h:61
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void QVector3D(Qt::Initialization)

/*
Constructs a null vector, i.e. with coordinates (0, 0, 0).
*/
// QVector3D(Qt::Initialization) ctx.fn_proto_cpp
impl /*struct*/ QVector3D {
  pub fn QVector3D_1<T: QVector3D_QVector3D_1>(value: T) -> QVector3D {
    let rsthis = value.QVector3D_1();
    return rsthis;
    // return 1;
  }
}

pub trait QVector3D_QVector3D_1 {
  fn QVector3D_1(self) -> QVector3D;
}
// QVector3D(Qt::Initialization) ctx.fn_proto_cpp
impl<'a> /*trait*/ QVector3D_QVector3D_1 for (i32) {
  fn QVector3D_1(self) -> QVector3D {
    // unsafe{_ZN9QVector3DC2EN2Qt14InitializationE()};
    let arg0 = (&self) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QVector3DC2EN2Qt14InitializationE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QVector3D{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qvector3d.h:62
// index:2
// Public inline Visibility=Default Availability=Available
// [-2] void QVector3D(float, float, float)

/*
Constructs a null vector, i.e. with coordinates (0, 0, 0).
*/
// QVector3D(float, float, float) ctx.fn_proto_cpp
impl /*struct*/ QVector3D {
  pub fn QVector3D_2<T: QVector3D_QVector3D_2>(value: T) -> QVector3D {
    let rsthis = value.QVector3D_2();
    return rsthis;
    // return 1;
  }
}

pub trait QVector3D_QVector3D_2 {
  fn QVector3D_2(self) -> QVector3D;
}
// QVector3D(float, float, float) ctx.fn_proto_cpp
impl<'a> /*trait*/ QVector3D_QVector3D_2 for (f32,f32,f32) {
  fn QVector3D_2(self) -> QVector3D {
    // unsafe{_ZN9QVector3DC2Efff()};
    let arg0 = (&self.0) as *const f32 as usize;
    let arg1 = (&self.1) as *const f32 as usize;
    let arg2 = (&self.2) as *const f32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QVector3DC2Efff", 3,qtrt::FFITY_FLOAT,qtrt::FFITY_FLOAT,qtrt::FFITY_FLOAT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QVector3D{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qvector3d.h:64
// index:3
// Public inline Visibility=Default Availability=Available
// [-2] void QVector3D(const QPoint &)

/*
Constructs a null vector, i.e. with coordinates (0, 0, 0).
*/
// QVector3D(const QPoint &) ctx.fn_proto_cpp
impl /*struct*/ QVector3D {
  pub fn QVector3D_3<T: QVector3D_QVector3D_3>(value: T) -> QVector3D {
    let rsthis = value.QVector3D_3();
    return rsthis;
    // return 1;
  }
}

pub trait QVector3D_QVector3D_3 {
  fn QVector3D_3(self) -> QVector3D;
}
// QVector3D(const QPoint &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QVector3D_QVector3D_3 for (usize) {
  fn QVector3D_3(self) -> QVector3D {
    // unsafe{_ZN9QVector3DC2ERK6QPoint()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QVector3DC2ERK6QPoint", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QVector3D{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qvector3d.h:65
// index:4
// Public inline Visibility=Default Availability=Available
// [-2] void QVector3D(const QPointF &)

/*
Constructs a null vector, i.e. with coordinates (0, 0, 0).
*/
// QVector3D(const QPointF &) ctx.fn_proto_cpp
impl /*struct*/ QVector3D {
  pub fn QVector3D_4<T: QVector3D_QVector3D_4>(value: T) -> QVector3D {
    let rsthis = value.QVector3D_4();
    return rsthis;
    // return 1;
  }
}

pub trait QVector3D_QVector3D_4 {
  fn QVector3D_4(self) -> QVector3D;
}
// QVector3D(const QPointF &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QVector3D_QVector3D_4 for (usize) {
  fn QVector3D_4(self) -> QVector3D {
    // unsafe{_ZN9QVector3DC2ERK7QPointF()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QVector3DC2ERK7QPointF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QVector3D{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qvector3d.h:67
// index:5
// Public Visibility=Default Availability=Available
// [-2] void QVector3D(const QVector2D &)

/*
Constructs a null vector, i.e. with coordinates (0, 0, 0).
*/
// QVector3D(const QVector2D &) ctx.fn_proto_cpp
impl /*struct*/ QVector3D {
  pub fn QVector3D_5<T: QVector3D_QVector3D_5>(value: T) -> QVector3D {
    let rsthis = value.QVector3D_5();
    return rsthis;
    // return 1;
  }
}

pub trait QVector3D_QVector3D_5 {
  fn QVector3D_5(self) -> QVector3D;
}
// QVector3D(const QVector2D &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QVector3D_QVector3D_5 for (usize) {
  fn QVector3D_5(self) -> QVector3D {
    // unsafe{_ZN9QVector3DC2ERK9QVector2D()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QVector3DC2ERK9QVector2D", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QVector3D{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qvector3d.h:68
// index:6
// Public Visibility=Default Availability=Available
// [-2] void QVector3D(const QVector2D &, float)

/*
Constructs a null vector, i.e. with coordinates (0, 0, 0).
*/
// QVector3D(const QVector2D &, float) ctx.fn_proto_cpp
impl /*struct*/ QVector3D {
  pub fn QVector3D_6<T: QVector3D_QVector3D_6>(value: T) -> QVector3D {
    let rsthis = value.QVector3D_6();
    return rsthis;
    // return 1;
  }
}

pub trait QVector3D_QVector3D_6 {
  fn QVector3D_6(self) -> QVector3D;
}
// QVector3D(const QVector2D &, float) ctx.fn_proto_cpp
impl<'a> /*trait*/ QVector3D_QVector3D_6 for (usize,f32) {
  fn QVector3D_6(self) -> QVector3D {
    // unsafe{_ZN9QVector3DC2ERK9QVector2Df()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const f32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QVector3DC2ERK9QVector2Df", 2,qtrt::FFITY_POINTER,qtrt::FFITY_FLOAT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QVector3D{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qvector3d.h:71
// index:7
// Public Visibility=Default Availability=Available
// [-2] void QVector3D(const QVector4D &)

/*
Constructs a null vector, i.e. with coordinates (0, 0, 0).
*/
// QVector3D(const QVector4D &) ctx.fn_proto_cpp
impl /*struct*/ QVector3D {
  pub fn QVector3D_7<T: QVector3D_QVector3D_7>(value: T) -> QVector3D {
    let rsthis = value.QVector3D_7();
    return rsthis;
    // return 1;
  }
}

pub trait QVector3D_QVector3D_7 {
  fn QVector3D_7(self) -> QVector3D;
}
// QVector3D(const QVector4D &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QVector3D_QVector3D_7 for (usize) {
  fn QVector3D_7(self) -> QVector3D {
    // unsafe{_ZN9QVector3DC2ERK9QVector4D()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QVector3DC2ERK9QVector4D", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QVector3D{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qvector3d.h:74
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isNull() const

/*
Returns true if the x, y, and z coordinates are set to 0.0, otherwise returns false.
*/
impl /*struct*/ QVector3D {
  pub fn isNull_0<RetType, T: QVector3D_isNull_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isNull_0(self);
    // return 1;
  }
}
pub trait QVector3D_isNull_0<RetType> {
  fn isNull_0(self , rsthis: & QVector3D) -> RetType;
}
impl<'a> /*trait*/ QVector3D_isNull_0<bool> for () {
  fn isNull_0(self , rsthis: & QVector3D) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QVector3D6isNullEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qvector3d.h:76
// index:0
// Public inline Visibility=Default Availability=Available
// [4] float x() const

/*
Returns the x coordinate of this point.

See also setX(), y(), and z().
*/
impl /*struct*/ QVector3D {
  pub fn x_0<RetType, T: QVector3D_x_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.x_0(self);
    // return 1;
  }
}
pub trait QVector3D_x_0<RetType> {
  fn x_0(self , rsthis: & QVector3D) -> RetType;
}
impl<'a> /*trait*/ QVector3D_x_0<f32> for () {
  fn x_0(self , rsthis: & QVector3D) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QVector3D1xEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qvector3d.h:77
// index:0
// Public inline Visibility=Default Availability=Available
// [4] float y() const

/*
Returns the y coordinate of this point.

See also setY(), x(), and z().
*/
impl /*struct*/ QVector3D {
  pub fn y_0<RetType, T: QVector3D_y_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.y_0(self);
    // return 1;
  }
}
pub trait QVector3D_y_0<RetType> {
  fn y_0(self , rsthis: & QVector3D) -> RetType;
}
impl<'a> /*trait*/ QVector3D_y_0<f32> for () {
  fn y_0(self , rsthis: & QVector3D) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QVector3D1yEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qvector3d.h:78
// index:0
// Public inline Visibility=Default Availability=Available
// [4] float z() const

/*
Returns the z coordinate of this point.

See also setZ(), x(), and y().
*/
impl /*struct*/ QVector3D {
  pub fn z_0<RetType, T: QVector3D_z_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.z_0(self);
    // return 1;
  }
}
pub trait QVector3D_z_0<RetType> {
  fn z_0(self , rsthis: & QVector3D) -> RetType;
}
impl<'a> /*trait*/ QVector3D_z_0<f32> for () {
  fn z_0(self , rsthis: & QVector3D) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QVector3D1zEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qvector3d.h:80
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setX(float)

/*
Sets the x coordinate of this point to the given x coordinate.

See also x(), setY(), and setZ().
*/
impl /*struct*/ QVector3D {
  pub fn setX_0<RetType, T: QVector3D_setX_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setX_0(self);
    // return 1;
  }
}
pub trait QVector3D_setX_0<RetType> {
  fn setX_0(self , rsthis: & QVector3D) -> RetType;
}
impl<'a> /*trait*/ QVector3D_setX_0<(/*void*/)> for (f32) {
  fn setX_0(self , rsthis: & QVector3D) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QVector3D4setXEf", 1,qtrt::FFITY_FLOAT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qvector3d.h:81
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setY(float)

/*
Sets the y coordinate of this point to the given y coordinate.

See also y(), setX(), and setZ().
*/
impl /*struct*/ QVector3D {
  pub fn setY_0<RetType, T: QVector3D_setY_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setY_0(self);
    // return 1;
  }
}
pub trait QVector3D_setY_0<RetType> {
  fn setY_0(self , rsthis: & QVector3D) -> RetType;
}
impl<'a> /*trait*/ QVector3D_setY_0<(/*void*/)> for (f32) {
  fn setY_0(self , rsthis: & QVector3D) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QVector3D4setYEf", 1,qtrt::FFITY_FLOAT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qvector3d.h:82
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setZ(float)

/*
Sets the z coordinate of this point to the given z coordinate.

See also z(), setX(), and setY().
*/
impl /*struct*/ QVector3D {
  pub fn setZ_0<RetType, T: QVector3D_setZ_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setZ_0(self);
    // return 1;
  }
}
pub trait QVector3D_setZ_0<RetType> {
  fn setZ_0(self , rsthis: & QVector3D) -> RetType;
}
impl<'a> /*trait*/ QVector3D_setZ_0<(/*void*/)> for (f32) {
  fn setZ_0(self , rsthis: & QVector3D) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QVector3D4setZEf", 1,qtrt::FFITY_FLOAT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qvector3d.h:84
// index:0
// Public Visibility=Default Availability=Available
// [4] float & operator[](int)

/*

*/
impl /*struct*/ QVector3D {
  pub fn operator_get_index_0<RetType, T: QVector3D_operator_get_index_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_get_index_0(self);
    // return 1;
  }
}
pub trait QVector3D_operator_get_index_0<RetType> {
  fn operator_get_index_0(self , rsthis: & QVector3D) -> RetType;
}
impl<'a> /*trait*/ QVector3D_operator_get_index_0<usize> for (i32) {
  fn operator_get_index_0(self , rsthis: & QVector3D) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QVector3DixEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qvector3d.h:85
// index:1
// Public Visibility=Default Availability=Available
// [4] float operator[](int) const

/*

*/
impl /*struct*/ QVector3D {
  pub fn operator_get_index_1<RetType, T: QVector3D_operator_get_index_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_get_index_1(self);
    // return 1;
  }
}
pub trait QVector3D_operator_get_index_1<RetType> {
  fn operator_get_index_1(self , rsthis: & QVector3D) -> RetType;
}
impl<'a> /*trait*/ QVector3D_operator_get_index_1<f32> for (i32) {
  fn operator_get_index_1(self , rsthis: & QVector3D) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QVector3DixEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qvector3d.h:87
// index:0
// Public Visibility=Default Availability=Available
// [4] float length() const

/*
Returns the length of the vector from the origin.

See also lengthSquared() and normalized().
*/
impl /*struct*/ QVector3D {
  pub fn length_0<RetType, T: QVector3D_length_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.length_0(self);
    // return 1;
  }
}
pub trait QVector3D_length_0<RetType> {
  fn length_0(self , rsthis: & QVector3D) -> RetType;
}
impl<'a> /*trait*/ QVector3D_length_0<f32> for () {
  fn length_0(self , rsthis: & QVector3D) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QVector3D6lengthEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qvector3d.h:88
// index:0
// Public Visibility=Default Availability=Available
// [4] float lengthSquared() const

/*
Returns the squared length of the vector from the origin. This is equivalent to the dot product of the vector with itself.

See also length() and dotProduct().
*/
impl /*struct*/ QVector3D {
  pub fn lengthSquared_0<RetType, T: QVector3D_lengthSquared_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.lengthSquared_0(self);
    // return 1;
  }
}
pub trait QVector3D_lengthSquared_0<RetType> {
  fn lengthSquared_0(self , rsthis: & QVector3D) -> RetType;
}
impl<'a> /*trait*/ QVector3D_lengthSquared_0<f32> for () {
  fn lengthSquared_0(self , rsthis: & QVector3D) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QVector3D13lengthSquaredEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qvector3d.h:90
// index:0
// Public Visibility=Default Availability=Available
// [12] QVector3D normalized() const

/*
Returns the normalized unit vector form of this vector.

If this vector is null, then a null vector is returned. If the length of the vector is very close to 1, then the vector will be returned as-is. Otherwise the normalized form of the vector of length 1 will be returned.

See also length() and normalize().
*/
impl /*struct*/ QVector3D {
  pub fn normalized_0<RetType, T: QVector3D_normalized_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.normalized_0(self);
    // return 1;
  }
}
pub trait QVector3D_normalized_0<RetType> {
  fn normalized_0(self , rsthis: & QVector3D) -> RetType;
}
impl<'a> /*trait*/ QVector3D_normalized_0<usize> for () {
  fn normalized_0(self , rsthis: & QVector3D) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QVector3D10normalizedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qvector3d.h:91
// index:0
// Public Visibility=Default Availability=Available
// [-2] void normalize()

/*
Normalizes the currect vector in place. Nothing happens if this vector is a null vector or the length of the vector is very close to 1.

See also length() and normalized().
*/
impl /*struct*/ QVector3D {
  pub fn normalize_0<RetType, T: QVector3D_normalize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.normalize_0(self);
    // return 1;
  }
}
pub trait QVector3D_normalize_0<RetType> {
  fn normalize_0(self , rsthis: & QVector3D) -> RetType;
}
impl<'a> /*trait*/ QVector3D_normalize_0<(/*void*/)> for () {
  fn normalize_0(self , rsthis: & QVector3D) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN9QVector3D9normalizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qvector3d.h:93
// index:0
// Public Visibility=Default Availability=Available
// [12] QVector3D & operator+=(const QVector3D &)

/*

*/
impl /*struct*/ QVector3D {
  pub fn operator_add_equal_0<RetType, T: QVector3D_operator_add_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_add_equal_0(self);
    // return 1;
  }
}
pub trait QVector3D_operator_add_equal_0<RetType> {
  fn operator_add_equal_0(self , rsthis: & QVector3D) -> RetType;
}
impl<'a> /*trait*/ QVector3D_operator_add_equal_0<usize> for (usize) {
  fn operator_add_equal_0(self , rsthis: & QVector3D) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QVector3DpLERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qvector3d.h:94
// index:0
// Public Visibility=Default Availability=Available
// [12] QVector3D & operator-=(const QVector3D &)

/*

*/
impl /*struct*/ QVector3D {
  pub fn operator_minus_equal_0<RetType, T: QVector3D_operator_minus_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_minus_equal_0(self);
    // return 1;
  }
}
pub trait QVector3D_operator_minus_equal_0<RetType> {
  fn operator_minus_equal_0(self , rsthis: & QVector3D) -> RetType;
}
impl<'a> /*trait*/ QVector3D_operator_minus_equal_0<usize> for (usize) {
  fn operator_minus_equal_0(self , rsthis: & QVector3D) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QVector3DmIERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qvector3d.h:95
// index:0
// Public Visibility=Default Availability=Available
// [12] QVector3D & operator*=(float)

/*

*/
impl /*struct*/ QVector3D {
  pub fn operator_mul_equal_0<RetType, T: QVector3D_operator_mul_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_mul_equal_0(self);
    // return 1;
  }
}
pub trait QVector3D_operator_mul_equal_0<RetType> {
  fn operator_mul_equal_0(self , rsthis: & QVector3D) -> RetType;
}
impl<'a> /*trait*/ QVector3D_operator_mul_equal_0<usize> for (f32) {
  fn operator_mul_equal_0(self , rsthis: & QVector3D) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QVector3DmLEf", 1,qtrt::FFITY_FLOAT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qvector3d.h:96
// index:1
// Public Visibility=Default Availability=Available
// [12] QVector3D & operator*=(const QVector3D &)

/*

*/
impl /*struct*/ QVector3D {
  pub fn operator_mul_equal_1<RetType, T: QVector3D_operator_mul_equal_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_mul_equal_1(self);
    // return 1;
  }
}
pub trait QVector3D_operator_mul_equal_1<RetType> {
  fn operator_mul_equal_1(self , rsthis: & QVector3D) -> RetType;
}
impl<'a> /*trait*/ QVector3D_operator_mul_equal_1<usize> for (usize) {
  fn operator_mul_equal_1(self , rsthis: & QVector3D) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QVector3DmLERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qvector3d.h:97
// index:0
// Public Visibility=Default Availability=Available
// [12] QVector3D & operator/=(float)

/*

*/
impl /*struct*/ QVector3D {
  pub fn operator_div_equal_0<RetType, T: QVector3D_operator_div_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_div_equal_0(self);
    // return 1;
  }
}
pub trait QVector3D_operator_div_equal_0<RetType> {
  fn operator_div_equal_0(self , rsthis: & QVector3D) -> RetType;
}
impl<'a> /*trait*/ QVector3D_operator_div_equal_0<usize> for (f32) {
  fn operator_div_equal_0(self , rsthis: & QVector3D) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QVector3DdVEf", 1,qtrt::FFITY_FLOAT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qvector3d.h:98
// index:1
// Public inline Visibility=Default Availability=Available
// [12] QVector3D & operator/=(const QVector3D &)

/*

*/
impl /*struct*/ QVector3D {
  pub fn operator_div_equal_1<RetType, T: QVector3D_operator_div_equal_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_div_equal_1(self);
    // return 1;
  }
}
pub trait QVector3D_operator_div_equal_1<RetType> {
  fn operator_div_equal_1(self , rsthis: & QVector3D) -> RetType;
}
impl<'a> /*trait*/ QVector3D_operator_div_equal_1<usize> for (usize) {
  fn operator_div_equal_1(self , rsthis: & QVector3D) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QVector3DdVERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qvector3d.h:100
// index:0
// Public static Visibility=Default Availability=Available
// [4] float dotProduct(const QVector3D &, const QVector3D &)

/*
Returns the dot product of v1 and v2.
*/
impl /*struct*/ QVector3D {
  pub fn dotProduct_0<RetType, T: QVector3D_dotProduct_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.dotProduct_0();
    // return 1;
  }
}
pub trait QVector3D_dotProduct_0<RetType> {
  fn dotProduct_0(self ) -> RetType;
}
impl<'a> /*trait*/ QVector3D_dotProduct_0<f32> for (usize,usize) {
  fn dotProduct_0(self ) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QVector3D10dotProductERKS_S1_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qvector3d.h:101
// index:0
// Public static Visibility=Default Availability=Available
// [12] QVector3D crossProduct(const QVector3D &, const QVector3D &)

/*
Returns the cross-product of vectors v1 and v2, which corresponds to the normal vector of a plane defined by v1 and v2.

See also normal().
*/
impl /*struct*/ QVector3D {
  pub fn crossProduct_0<RetType, T: QVector3D_crossProduct_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.crossProduct_0();
    // return 1;
  }
}
pub trait QVector3D_crossProduct_0<RetType> {
  fn crossProduct_0(self ) -> RetType;
}
impl<'a> /*trait*/ QVector3D_crossProduct_0<usize> for (usize,usize) {
  fn crossProduct_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QVector3D12crossProductERKS_S1_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qvector3d.h:103
// index:0
// Public static Visibility=Default Availability=Available
// [12] QVector3D normal(const QVector3D &, const QVector3D &)

/*
Returns the normal vector of a plane defined by vectors v1 and v2, normalized to be a unit vector.

Use crossProduct() to compute the cross-product of v1 and v2 if you do not need the result to be normalized to a unit vector.

See also crossProduct() and distanceToPlane().
*/
impl /*struct*/ QVector3D {
  pub fn normal_0<RetType, T: QVector3D_normal_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.normal_0();
    // return 1;
  }
}
pub trait QVector3D_normal_0<RetType> {
  fn normal_0(self ) -> RetType;
}
impl<'a> /*trait*/ QVector3D_normal_0<usize> for (usize,usize) {
  fn normal_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QVector3D6normalERKS_S1_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qvector3d.h:104
// index:1
// Public static Visibility=Default Availability=Available
// [12] QVector3D normal(const QVector3D &, const QVector3D &, const QVector3D &)

/*
Returns the normal vector of a plane defined by vectors v1 and v2, normalized to be a unit vector.

Use crossProduct() to compute the cross-product of v1 and v2 if you do not need the result to be normalized to a unit vector.

See also crossProduct() and distanceToPlane().
*/
impl /*struct*/ QVector3D {
  pub fn normal_1<RetType, T: QVector3D_normal_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.normal_1();
    // return 1;
  }
}
pub trait QVector3D_normal_1<RetType> {
  fn normal_1(self ) -> RetType;
}
impl<'a> /*trait*/ QVector3D_normal_1<usize> for (usize,usize,usize) {
  fn normal_1(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QVector3D6normalERKS_S1_S1_", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qvector3d.h:107
// index:0
// Public Visibility=Default Availability=Available
// [12] QVector3D project(const QMatrix4x4 &, const QMatrix4x4 &, const QRect &) const

/*
Returns the window coordinates of this vector initially in object/model coordinates using the model view matrix modelView, the projection matrix projection and the viewport dimensions viewport.

When transforming from clip to normalized space, a division by the w component on the vector components takes place. To prevent dividing by 0 if w equals to 0, it is set to 1.

Note: the returned y coordinates are in OpenGL orientation. OpenGL expects the bottom to be 0 whereas for Qt top is 0.

This function was introduced in  Qt 5.5.

See also unproject().
*/
impl /*struct*/ QVector3D {
  pub fn project_0<RetType, T: QVector3D_project_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.project_0(self);
    // return 1;
  }
}
pub trait QVector3D_project_0<RetType> {
  fn project_0(self , rsthis: & QVector3D) -> RetType;
}
impl<'a> /*trait*/ QVector3D_project_0<usize> for (usize,usize,usize) {
  fn project_0(self , rsthis: & QVector3D) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QVector3D7projectERK10QMatrix4x4S2_RK5QRect", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qvector3d.h:108
// index:0
// Public Visibility=Default Availability=Available
// [12] QVector3D unproject(const QMatrix4x4 &, const QMatrix4x4 &, const QRect &) const

/*
Returns the object/model coordinates of this vector initially in window coordinates using the model view matrix modelView, the projection matrix projection and the viewport dimensions viewport.

When transforming from clip to normalized space, a division by the w component of the vector components takes place. To prevent dividing by 0 if w equals to 0, it is set to 1.

Note: y coordinates in viewport should use OpenGL orientation. OpenGL expects the bottom to be 0 whereas for Qt top is 0.

This function was introduced in  Qt 5.5.

See also project().
*/
impl /*struct*/ QVector3D {
  pub fn unproject_0<RetType, T: QVector3D_unproject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.unproject_0(self);
    // return 1;
  }
}
pub trait QVector3D_unproject_0<RetType> {
  fn unproject_0(self , rsthis: & QVector3D) -> RetType;
}
impl<'a> /*trait*/ QVector3D_unproject_0<usize> for (usize,usize,usize) {
  fn unproject_0(self , rsthis: & QVector3D) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QVector3D9unprojectERK10QMatrix4x4S2_RK5QRect", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qvector3d.h:110
// index:0
// Public Visibility=Default Availability=Available
// [4] float distanceToPoint(const QVector3D &) const

/*
Returns the distance from this vertex to a point defined by the vertex point.

This function was introduced in  Qt 5.1.

See also distanceToPlane() and distanceToLine().
*/
impl /*struct*/ QVector3D {
  pub fn distanceToPoint_0<RetType, T: QVector3D_distanceToPoint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.distanceToPoint_0(self);
    // return 1;
  }
}
pub trait QVector3D_distanceToPoint_0<RetType> {
  fn distanceToPoint_0(self , rsthis: & QVector3D) -> RetType;
}
impl<'a> /*trait*/ QVector3D_distanceToPoint_0<f32> for (usize) {
  fn distanceToPoint_0(self , rsthis: & QVector3D) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QVector3D15distanceToPointERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qvector3d.h:111
// index:0
// Public Visibility=Default Availability=Available
// [4] float distanceToPlane(const QVector3D &, const QVector3D &) const

/*
Returns the distance from this vertex to a plane defined by the vertex plane and a normal unit vector. The normal parameter is assumed to have been normalized to a unit vector.

The return value will be negative if the vertex is below the plane, or zero if it is on the plane.

See also normal() and distanceToLine().
*/
impl /*struct*/ QVector3D {
  pub fn distanceToPlane_0<RetType, T: QVector3D_distanceToPlane_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.distanceToPlane_0(self);
    // return 1;
  }
}
pub trait QVector3D_distanceToPlane_0<RetType> {
  fn distanceToPlane_0(self , rsthis: & QVector3D) -> RetType;
}
impl<'a> /*trait*/ QVector3D_distanceToPlane_0<f32> for (usize,usize) {
  fn distanceToPlane_0(self , rsthis: & QVector3D) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QVector3D15distanceToPlaneERKS_S1_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qvector3d.h:112
// index:1
// Public Visibility=Default Availability=Available
// [4] float distanceToPlane(const QVector3D &, const QVector3D &, const QVector3D &) const

/*
Returns the distance from this vertex to a plane defined by the vertex plane and a normal unit vector. The normal parameter is assumed to have been normalized to a unit vector.

The return value will be negative if the vertex is below the plane, or zero if it is on the plane.

See also normal() and distanceToLine().
*/
impl /*struct*/ QVector3D {
  pub fn distanceToPlane_1<RetType, T: QVector3D_distanceToPlane_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.distanceToPlane_1(self);
    // return 1;
  }
}
pub trait QVector3D_distanceToPlane_1<RetType> {
  fn distanceToPlane_1(self , rsthis: & QVector3D) -> RetType;
}
impl<'a> /*trait*/ QVector3D_distanceToPlane_1<f32> for (usize,usize,usize) {
  fn distanceToPlane_1(self , rsthis: & QVector3D) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QVector3D15distanceToPlaneERKS_S1_S1_", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qvector3d.h:113
// index:0
// Public Visibility=Default Availability=Available
// [4] float distanceToLine(const QVector3D &, const QVector3D &) const

/*
Returns the distance that this vertex is from a line defined by point and the unit vector direction.

If direction is a null vector, then it does not define a line. In that case, the distance from point to this vertex is returned.

See also distanceToPlane().
*/
impl /*struct*/ QVector3D {
  pub fn distanceToLine_0<RetType, T: QVector3D_distanceToLine_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.distanceToLine_0(self);
    // return 1;
  }
}
pub trait QVector3D_distanceToLine_0<RetType> {
  fn distanceToLine_0(self , rsthis: & QVector3D) -> RetType;
}
impl<'a> /*trait*/ QVector3D_distanceToLine_0<f32> for (usize,usize) {
  fn distanceToLine_0(self , rsthis: & QVector3D) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QVector3D14distanceToLineERKS_S1_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qvector3d.h:129
// index:0
// Public Visibility=Default Availability=Available
// [8] QVector2D toVector2D() const

/*
Returns the 2D vector form of this 3D vector, dropping the z coordinate.

See also toVector4D() and toPoint().
*/
impl /*struct*/ QVector3D {
  pub fn toVector2D_0<RetType, T: QVector3D_toVector2D_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toVector2D_0(self);
    // return 1;
  }
}
pub trait QVector3D_toVector2D_0<RetType> {
  fn toVector2D_0(self , rsthis: & QVector3D) -> RetType;
}
impl<'a> /*trait*/ QVector3D_toVector2D_0<usize> for () {
  fn toVector2D_0(self , rsthis: & QVector3D) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QVector3D10toVector2DEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qvector3d.h:132
// index:0
// Public Visibility=Default Availability=Available
// [16] QVector4D toVector4D() const

/*
Returns the 4D form of this 3D vector, with the w coordinate set to zero.

See also toVector2D() and toPoint().
*/
impl /*struct*/ QVector3D {
  pub fn toVector4D_0<RetType, T: QVector3D_toVector4D_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toVector4D_0(self);
    // return 1;
  }
}
pub trait QVector3D_toVector4D_0<RetType> {
  fn toVector4D_0(self , rsthis: & QVector3D) -> RetType;
}
impl<'a> /*trait*/ QVector3D_toVector4D_0<usize> for () {
  fn toVector4D_0(self , rsthis: & QVector3D) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QVector3D10toVector4DEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qvector3d.h:135
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QPoint toPoint() const

/*
Returns the QPoint form of this 3D vector. The z coordinate is dropped.

See also toPointF() and toVector2D().
*/
impl /*struct*/ QVector3D {
  pub fn toPoint_0<RetType, T: QVector3D_toPoint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toPoint_0(self);
    // return 1;
  }
}
pub trait QVector3D_toPoint_0<RetType> {
  fn toPoint_0(self , rsthis: & QVector3D) -> RetType;
}
impl<'a> /*trait*/ QVector3D_toPoint_0<usize> for () {
  fn toPoint_0(self , rsthis: & QVector3D) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QVector3D7toPointEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qvector3d.h:136
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QPointF toPointF() const

/*
Returns the QPointF form of this 3D vector. The z coordinate is dropped.

See also toPoint() and toVector2D().
*/
impl /*struct*/ QVector3D {
  pub fn toPointF_0<RetType, T: QVector3D_toPointF_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toPointF_0(self);
    // return 1;
  }
}
pub trait QVector3D_toPointF_0<RetType> {
  fn toPointF_0(self , rsthis: & QVector3D) -> RetType;
}
impl<'a> /*trait*/ QVector3D_toPointF_0<usize> for () {
  fn toPointF_0(self , rsthis: & QVector3D) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QVector3D8toPointFEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}


pub fn DeleteQVector3D(this :*mut QVector3D) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN9QVector3DD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
