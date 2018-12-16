

// mod ::gui::QVector4D
// package qtgui
// /usr/include/qt/QtGui/qvector4d.h
// #include <qvector4d.h>
// #include <QtGui>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 41
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
#[derive(Default)] // class sizeof(QVector4D)=16
pub struct QVector4D {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QVector4D_ITF interface {
//    QVector4D_PTR() *QVector4D
//}
//func (ptr *QVector4D) QVector4D_PTR() *QVector4D { return ptr }

impl /*struct*/ QVector4D {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QVector4D {
    return QVector4D{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QVector4D {
//  type Target = QVector4DBASE;
//
//  fn deref(&self) -> &QVector4DBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QVector4DBASE> for QVector4D {
//  fn as_ref(& self) -> & QVector4DBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qvector4d.h:59
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void QVector4D()

/*
Constructs a null vector, i.e. with coordinates (0, 0, 0, 0).
*/
// QVector4D() ctx.fn_proto_cpp
impl /*struct*/ QVector4D {
  pub fn QVector4D_0<T: QVector4D_QVector4D_0>(value: T) -> QVector4D {
    let rsthis = value.QVector4D_0();
    return rsthis;
    // return 1;
  }
}

pub trait QVector4D_QVector4D_0 {
  fn QVector4D_0(self) -> QVector4D;
}
// QVector4D() ctx.fn_proto_cpp
impl<'a> /*trait*/ QVector4D_QVector4D_0 for () {
  fn QVector4D_0(self) -> QVector4D {
    // unsafe{_ZN9QVector4DC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QVector4DC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QVector4D{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qvector4d.h:60
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void QVector4D(Qt::Initialization)

/*
Constructs a null vector, i.e. with coordinates (0, 0, 0, 0).
*/
// QVector4D(Qt::Initialization) ctx.fn_proto_cpp
impl /*struct*/ QVector4D {
  pub fn QVector4D_1<T: QVector4D_QVector4D_1>(value: T) -> QVector4D {
    let rsthis = value.QVector4D_1();
    return rsthis;
    // return 1;
  }
}

pub trait QVector4D_QVector4D_1 {
  fn QVector4D_1(self) -> QVector4D;
}
// QVector4D(Qt::Initialization) ctx.fn_proto_cpp
impl<'a> /*trait*/ QVector4D_QVector4D_1 for (i32) {
  fn QVector4D_1(self) -> QVector4D {
    // unsafe{_ZN9QVector4DC2EN2Qt14InitializationE()};
    let arg0 = (&self) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QVector4DC2EN2Qt14InitializationE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QVector4D{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qvector4d.h:61
// index:2
// Public inline Visibility=Default Availability=Available
// [-2] void QVector4D(float, float, float, float)

/*
Constructs a null vector, i.e. with coordinates (0, 0, 0, 0).
*/
// QVector4D(float, float, float, float) ctx.fn_proto_cpp
impl /*struct*/ QVector4D {
  pub fn QVector4D_2<T: QVector4D_QVector4D_2>(value: T) -> QVector4D {
    let rsthis = value.QVector4D_2();
    return rsthis;
    // return 1;
  }
}

pub trait QVector4D_QVector4D_2 {
  fn QVector4D_2(self) -> QVector4D;
}
// QVector4D(float, float, float, float) ctx.fn_proto_cpp
impl<'a> /*trait*/ QVector4D_QVector4D_2 for (f32,f32,f32,f32) {
  fn QVector4D_2(self) -> QVector4D {
    // unsafe{_ZN9QVector4DC2Effff()};
    let arg0 = (&self.0) as *const f32 as usize;
    let arg1 = (&self.1) as *const f32 as usize;
    let arg2 = (&self.2) as *const f32 as usize;
    let arg3 = (&self.3) as *const f32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QVector4DC2Effff", 4,qtrt::FFITY_FLOAT,qtrt::FFITY_FLOAT,qtrt::FFITY_FLOAT,qtrt::FFITY_FLOAT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QVector4D{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qvector4d.h:62
// index:3
// Public inline Visibility=Default Availability=Available
// [-2] void QVector4D(const QPoint &)

/*
Constructs a null vector, i.e. with coordinates (0, 0, 0, 0).
*/
// QVector4D(const QPoint &) ctx.fn_proto_cpp
impl /*struct*/ QVector4D {
  pub fn QVector4D_3<T: QVector4D_QVector4D_3>(value: T) -> QVector4D {
    let rsthis = value.QVector4D_3();
    return rsthis;
    // return 1;
  }
}

pub trait QVector4D_QVector4D_3 {
  fn QVector4D_3(self) -> QVector4D;
}
// QVector4D(const QPoint &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QVector4D_QVector4D_3 for (usize) {
  fn QVector4D_3(self) -> QVector4D {
    // unsafe{_ZN9QVector4DC2ERK6QPoint()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QVector4DC2ERK6QPoint", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QVector4D{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qvector4d.h:63
// index:4
// Public inline Visibility=Default Availability=Available
// [-2] void QVector4D(const QPointF &)

/*
Constructs a null vector, i.e. with coordinates (0, 0, 0, 0).
*/
// QVector4D(const QPointF &) ctx.fn_proto_cpp
impl /*struct*/ QVector4D {
  pub fn QVector4D_4<T: QVector4D_QVector4D_4>(value: T) -> QVector4D {
    let rsthis = value.QVector4D_4();
    return rsthis;
    // return 1;
  }
}

pub trait QVector4D_QVector4D_4 {
  fn QVector4D_4(self) -> QVector4D;
}
// QVector4D(const QPointF &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QVector4D_QVector4D_4 for (usize) {
  fn QVector4D_4(self) -> QVector4D {
    // unsafe{_ZN9QVector4DC2ERK7QPointF()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QVector4DC2ERK7QPointF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QVector4D{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qvector4d.h:65
// index:5
// Public Visibility=Default Availability=Available
// [-2] void QVector4D(const QVector2D &)

/*
Constructs a null vector, i.e. with coordinates (0, 0, 0, 0).
*/
// QVector4D(const QVector2D &) ctx.fn_proto_cpp
impl /*struct*/ QVector4D {
  pub fn QVector4D_5<T: QVector4D_QVector4D_5>(value: T) -> QVector4D {
    let rsthis = value.QVector4D_5();
    return rsthis;
    // return 1;
  }
}

pub trait QVector4D_QVector4D_5 {
  fn QVector4D_5(self) -> QVector4D;
}
// QVector4D(const QVector2D &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QVector4D_QVector4D_5 for (usize) {
  fn QVector4D_5(self) -> QVector4D {
    // unsafe{_ZN9QVector4DC2ERK9QVector2D()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QVector4DC2ERK9QVector2D", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QVector4D{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qvector4d.h:66
// index:6
// Public Visibility=Default Availability=Available
// [-2] void QVector4D(const QVector2D &, float, float)

/*
Constructs a null vector, i.e. with coordinates (0, 0, 0, 0).
*/
// QVector4D(const QVector2D &, float, float) ctx.fn_proto_cpp
impl /*struct*/ QVector4D {
  pub fn QVector4D_6<T: QVector4D_QVector4D_6>(value: T) -> QVector4D {
    let rsthis = value.QVector4D_6();
    return rsthis;
    // return 1;
  }
}

pub trait QVector4D_QVector4D_6 {
  fn QVector4D_6(self) -> QVector4D;
}
// QVector4D(const QVector2D &, float, float) ctx.fn_proto_cpp
impl<'a> /*trait*/ QVector4D_QVector4D_6 for (usize,f32,f32) {
  fn QVector4D_6(self) -> QVector4D {
    // unsafe{_ZN9QVector4DC2ERK9QVector2Dff()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const f32 as usize;
    let arg2 = (&self.2) as *const f32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QVector4DC2ERK9QVector2Dff", 3,qtrt::FFITY_POINTER,qtrt::FFITY_FLOAT,qtrt::FFITY_FLOAT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QVector4D{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qvector4d.h:69
// index:7
// Public Visibility=Default Availability=Available
// [-2] void QVector4D(const QVector3D &)

/*
Constructs a null vector, i.e. with coordinates (0, 0, 0, 0).
*/
// QVector4D(const QVector3D &) ctx.fn_proto_cpp
impl /*struct*/ QVector4D {
  pub fn QVector4D_7<T: QVector4D_QVector4D_7>(value: T) -> QVector4D {
    let rsthis = value.QVector4D_7();
    return rsthis;
    // return 1;
  }
}

pub trait QVector4D_QVector4D_7 {
  fn QVector4D_7(self) -> QVector4D;
}
// QVector4D(const QVector3D &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QVector4D_QVector4D_7 for (usize) {
  fn QVector4D_7(self) -> QVector4D {
    // unsafe{_ZN9QVector4DC2ERK9QVector3D()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QVector4DC2ERK9QVector3D", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QVector4D{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qvector4d.h:70
// index:8
// Public Visibility=Default Availability=Available
// [-2] void QVector4D(const QVector3D &, float)

/*
Constructs a null vector, i.e. with coordinates (0, 0, 0, 0).
*/
// QVector4D(const QVector3D &, float) ctx.fn_proto_cpp
impl /*struct*/ QVector4D {
  pub fn QVector4D_8<T: QVector4D_QVector4D_8>(value: T) -> QVector4D {
    let rsthis = value.QVector4D_8();
    return rsthis;
    // return 1;
  }
}

pub trait QVector4D_QVector4D_8 {
  fn QVector4D_8(self) -> QVector4D;
}
// QVector4D(const QVector3D &, float) ctx.fn_proto_cpp
impl<'a> /*trait*/ QVector4D_QVector4D_8 for (usize,f32) {
  fn QVector4D_8(self) -> QVector4D {
    // unsafe{_ZN9QVector4DC2ERK9QVector3Df()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const f32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QVector4DC2ERK9QVector3Df", 2,qtrt::FFITY_POINTER,qtrt::FFITY_FLOAT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QVector4D{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qvector4d.h:73
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isNull() const

/*
Returns true if the x, y, z, and w coordinates are set to 0.0, otherwise returns false.
*/
impl /*struct*/ QVector4D {
  pub fn isNull_0<RetType, T: QVector4D_isNull_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isNull_0(self);
    // return 1;
  }
}
pub trait QVector4D_isNull_0<RetType> {
  fn isNull_0(self , rsthis: & QVector4D) -> RetType;
}
impl<'a> /*trait*/ QVector4D_isNull_0<bool> for () {
  fn isNull_0(self , rsthis: & QVector4D) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QVector4D6isNullEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qvector4d.h:75
// index:0
// Public inline Visibility=Default Availability=Available
// [4] float x() const

/*
Returns the x coordinate of this point.

See also setX(), y(), z(), and w().
*/
impl /*struct*/ QVector4D {
  pub fn x_0<RetType, T: QVector4D_x_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.x_0(self);
    // return 1;
  }
}
pub trait QVector4D_x_0<RetType> {
  fn x_0(self , rsthis: & QVector4D) -> RetType;
}
impl<'a> /*trait*/ QVector4D_x_0<f32> for () {
  fn x_0(self , rsthis: & QVector4D) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QVector4D1xEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qvector4d.h:76
// index:0
// Public inline Visibility=Default Availability=Available
// [4] float y() const

/*
Returns the y coordinate of this point.

See also setY(), x(), z(), and w().
*/
impl /*struct*/ QVector4D {
  pub fn y_0<RetType, T: QVector4D_y_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.y_0(self);
    // return 1;
  }
}
pub trait QVector4D_y_0<RetType> {
  fn y_0(self , rsthis: & QVector4D) -> RetType;
}
impl<'a> /*trait*/ QVector4D_y_0<f32> for () {
  fn y_0(self , rsthis: & QVector4D) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QVector4D1yEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qvector4d.h:77
// index:0
// Public inline Visibility=Default Availability=Available
// [4] float z() const

/*
Returns the z coordinate of this point.

See also setZ(), x(), y(), and w().
*/
impl /*struct*/ QVector4D {
  pub fn z_0<RetType, T: QVector4D_z_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.z_0(self);
    // return 1;
  }
}
pub trait QVector4D_z_0<RetType> {
  fn z_0(self , rsthis: & QVector4D) -> RetType;
}
impl<'a> /*trait*/ QVector4D_z_0<f32> for () {
  fn z_0(self , rsthis: & QVector4D) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QVector4D1zEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qvector4d.h:78
// index:0
// Public inline Visibility=Default Availability=Available
// [4] float w() const

/*
Returns the w coordinate of this point.

See also setW(), x(), y(), and z().
*/
impl /*struct*/ QVector4D {
  pub fn w_0<RetType, T: QVector4D_w_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.w_0(self);
    // return 1;
  }
}
pub trait QVector4D_w_0<RetType> {
  fn w_0(self , rsthis: & QVector4D) -> RetType;
}
impl<'a> /*trait*/ QVector4D_w_0<f32> for () {
  fn w_0(self , rsthis: & QVector4D) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QVector4D1wEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qvector4d.h:80
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setX(float)

/*
Sets the x coordinate of this point to the given x coordinate.

See also x(), setY(), setZ(), and setW().
*/
impl /*struct*/ QVector4D {
  pub fn setX_0<RetType, T: QVector4D_setX_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setX_0(self);
    // return 1;
  }
}
pub trait QVector4D_setX_0<RetType> {
  fn setX_0(self , rsthis: & QVector4D) -> RetType;
}
impl<'a> /*trait*/ QVector4D_setX_0<(/*void*/)> for (f32) {
  fn setX_0(self , rsthis: & QVector4D) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QVector4D4setXEf", 1,qtrt::FFITY_FLOAT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qvector4d.h:81
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setY(float)

/*
Sets the y coordinate of this point to the given y coordinate.

See also y(), setX(), setZ(), and setW().
*/
impl /*struct*/ QVector4D {
  pub fn setY_0<RetType, T: QVector4D_setY_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setY_0(self);
    // return 1;
  }
}
pub trait QVector4D_setY_0<RetType> {
  fn setY_0(self , rsthis: & QVector4D) -> RetType;
}
impl<'a> /*trait*/ QVector4D_setY_0<(/*void*/)> for (f32) {
  fn setY_0(self , rsthis: & QVector4D) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QVector4D4setYEf", 1,qtrt::FFITY_FLOAT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qvector4d.h:82
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setZ(float)

/*
Sets the z coordinate of this point to the given z coordinate.

See also z(), setX(), setY(), and setW().
*/
impl /*struct*/ QVector4D {
  pub fn setZ_0<RetType, T: QVector4D_setZ_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setZ_0(self);
    // return 1;
  }
}
pub trait QVector4D_setZ_0<RetType> {
  fn setZ_0(self , rsthis: & QVector4D) -> RetType;
}
impl<'a> /*trait*/ QVector4D_setZ_0<(/*void*/)> for (f32) {
  fn setZ_0(self , rsthis: & QVector4D) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QVector4D4setZEf", 1,qtrt::FFITY_FLOAT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qvector4d.h:83
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setW(float)

/*
Sets the w coordinate of this point to the given w coordinate.

See also w(), setX(), setY(), and setZ().
*/
impl /*struct*/ QVector4D {
  pub fn setW_0<RetType, T: QVector4D_setW_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setW_0(self);
    // return 1;
  }
}
pub trait QVector4D_setW_0<RetType> {
  fn setW_0(self , rsthis: & QVector4D) -> RetType;
}
impl<'a> /*trait*/ QVector4D_setW_0<(/*void*/)> for (f32) {
  fn setW_0(self , rsthis: & QVector4D) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QVector4D4setWEf", 1,qtrt::FFITY_FLOAT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qvector4d.h:85
// index:0
// Public Visibility=Default Availability=Available
// [4] float & operator[](int)

/*

*/
impl /*struct*/ QVector4D {
  pub fn operator_get_index_0<RetType, T: QVector4D_operator_get_index_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_get_index_0(self);
    // return 1;
  }
}
pub trait QVector4D_operator_get_index_0<RetType> {
  fn operator_get_index_0(self , rsthis: & QVector4D) -> RetType;
}
impl<'a> /*trait*/ QVector4D_operator_get_index_0<usize> for (i32) {
  fn operator_get_index_0(self , rsthis: & QVector4D) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QVector4DixEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qvector4d.h:86
// index:1
// Public Visibility=Default Availability=Available
// [4] float operator[](int) const

/*

*/
impl /*struct*/ QVector4D {
  pub fn operator_get_index_1<RetType, T: QVector4D_operator_get_index_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_get_index_1(self);
    // return 1;
  }
}
pub trait QVector4D_operator_get_index_1<RetType> {
  fn operator_get_index_1(self , rsthis: & QVector4D) -> RetType;
}
impl<'a> /*trait*/ QVector4D_operator_get_index_1<f32> for (i32) {
  fn operator_get_index_1(self , rsthis: & QVector4D) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QVector4DixEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qvector4d.h:88
// index:0
// Public Visibility=Default Availability=Available
// [4] float length() const

/*
Returns the length of the vector from the origin.

See also lengthSquared() and normalized().
*/
impl /*struct*/ QVector4D {
  pub fn length_0<RetType, T: QVector4D_length_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.length_0(self);
    // return 1;
  }
}
pub trait QVector4D_length_0<RetType> {
  fn length_0(self , rsthis: & QVector4D) -> RetType;
}
impl<'a> /*trait*/ QVector4D_length_0<f32> for () {
  fn length_0(self , rsthis: & QVector4D) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QVector4D6lengthEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qvector4d.h:89
// index:0
// Public Visibility=Default Availability=Available
// [4] float lengthSquared() const

/*
Returns the squared length of the vector from the origin. This is equivalent to the dot product of the vector with itself.

See also length() and dotProduct().
*/
impl /*struct*/ QVector4D {
  pub fn lengthSquared_0<RetType, T: QVector4D_lengthSquared_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.lengthSquared_0(self);
    // return 1;
  }
}
pub trait QVector4D_lengthSquared_0<RetType> {
  fn lengthSquared_0(self , rsthis: & QVector4D) -> RetType;
}
impl<'a> /*trait*/ QVector4D_lengthSquared_0<f32> for () {
  fn lengthSquared_0(self , rsthis: & QVector4D) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QVector4D13lengthSquaredEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qvector4d.h:91
// index:0
// Public Visibility=Default Availability=Available
// [16] QVector4D normalized() const

/*
Returns the normalized unit vector form of this vector.

If this vector is null, then a null vector is returned. If the length of the vector is very close to 1, then the vector will be returned as-is. Otherwise the normalized form of the vector of length 1 will be returned.

See also length() and normalize().
*/
impl /*struct*/ QVector4D {
  pub fn normalized_0<RetType, T: QVector4D_normalized_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.normalized_0(self);
    // return 1;
  }
}
pub trait QVector4D_normalized_0<RetType> {
  fn normalized_0(self , rsthis: & QVector4D) -> RetType;
}
impl<'a> /*trait*/ QVector4D_normalized_0<usize> for () {
  fn normalized_0(self , rsthis: & QVector4D) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QVector4D10normalizedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qvector4d.h:92
// index:0
// Public Visibility=Default Availability=Available
// [-2] void normalize()

/*
Normalizes the currect vector in place. Nothing happens if this vector is a null vector or the length of the vector is very close to 1.

See also length() and normalized().
*/
impl /*struct*/ QVector4D {
  pub fn normalize_0<RetType, T: QVector4D_normalize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.normalize_0(self);
    // return 1;
  }
}
pub trait QVector4D_normalize_0<RetType> {
  fn normalize_0(self , rsthis: & QVector4D) -> RetType;
}
impl<'a> /*trait*/ QVector4D_normalize_0<(/*void*/)> for () {
  fn normalize_0(self , rsthis: & QVector4D) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN9QVector4D9normalizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qvector4d.h:94
// index:0
// Public Visibility=Default Availability=Available
// [16] QVector4D & operator+=(const QVector4D &)

/*

*/
impl /*struct*/ QVector4D {
  pub fn operator_add_equal_0<RetType, T: QVector4D_operator_add_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_add_equal_0(self);
    // return 1;
  }
}
pub trait QVector4D_operator_add_equal_0<RetType> {
  fn operator_add_equal_0(self , rsthis: & QVector4D) -> RetType;
}
impl<'a> /*trait*/ QVector4D_operator_add_equal_0<usize> for (usize) {
  fn operator_add_equal_0(self , rsthis: & QVector4D) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QVector4DpLERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qvector4d.h:95
// index:0
// Public Visibility=Default Availability=Available
// [16] QVector4D & operator-=(const QVector4D &)

/*

*/
impl /*struct*/ QVector4D {
  pub fn operator_minus_equal_0<RetType, T: QVector4D_operator_minus_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_minus_equal_0(self);
    // return 1;
  }
}
pub trait QVector4D_operator_minus_equal_0<RetType> {
  fn operator_minus_equal_0(self , rsthis: & QVector4D) -> RetType;
}
impl<'a> /*trait*/ QVector4D_operator_minus_equal_0<usize> for (usize) {
  fn operator_minus_equal_0(self , rsthis: & QVector4D) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QVector4DmIERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qvector4d.h:96
// index:0
// Public Visibility=Default Availability=Available
// [16] QVector4D & operator*=(float)

/*

*/
impl /*struct*/ QVector4D {
  pub fn operator_mul_equal_0<RetType, T: QVector4D_operator_mul_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_mul_equal_0(self);
    // return 1;
  }
}
pub trait QVector4D_operator_mul_equal_0<RetType> {
  fn operator_mul_equal_0(self , rsthis: & QVector4D) -> RetType;
}
impl<'a> /*trait*/ QVector4D_operator_mul_equal_0<usize> for (f32) {
  fn operator_mul_equal_0(self , rsthis: & QVector4D) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QVector4DmLEf", 1,qtrt::FFITY_FLOAT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qvector4d.h:97
// index:1
// Public Visibility=Default Availability=Available
// [16] QVector4D & operator*=(const QVector4D &)

/*

*/
impl /*struct*/ QVector4D {
  pub fn operator_mul_equal_1<RetType, T: QVector4D_operator_mul_equal_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_mul_equal_1(self);
    // return 1;
  }
}
pub trait QVector4D_operator_mul_equal_1<RetType> {
  fn operator_mul_equal_1(self , rsthis: & QVector4D) -> RetType;
}
impl<'a> /*trait*/ QVector4D_operator_mul_equal_1<usize> for (usize) {
  fn operator_mul_equal_1(self , rsthis: & QVector4D) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QVector4DmLERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qvector4d.h:98
// index:0
// Public Visibility=Default Availability=Available
// [16] QVector4D & operator/=(float)

/*

*/
impl /*struct*/ QVector4D {
  pub fn operator_div_equal_0<RetType, T: QVector4D_operator_div_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_div_equal_0(self);
    // return 1;
  }
}
pub trait QVector4D_operator_div_equal_0<RetType> {
  fn operator_div_equal_0(self , rsthis: & QVector4D) -> RetType;
}
impl<'a> /*trait*/ QVector4D_operator_div_equal_0<usize> for (f32) {
  fn operator_div_equal_0(self , rsthis: & QVector4D) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QVector4DdVEf", 1,qtrt::FFITY_FLOAT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qvector4d.h:99
// index:1
// Public inline Visibility=Default Availability=Available
// [16] QVector4D & operator/=(const QVector4D &)

/*

*/
impl /*struct*/ QVector4D {
  pub fn operator_div_equal_1<RetType, T: QVector4D_operator_div_equal_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_div_equal_1(self);
    // return 1;
  }
}
pub trait QVector4D_operator_div_equal_1<RetType> {
  fn operator_div_equal_1(self , rsthis: & QVector4D) -> RetType;
}
impl<'a> /*trait*/ QVector4D_operator_div_equal_1<usize> for (usize) {
  fn operator_div_equal_1(self , rsthis: & QVector4D) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QVector4DdVERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qvector4d.h:101
// index:0
// Public static Visibility=Default Availability=Available
// [4] float dotProduct(const QVector4D &, const QVector4D &)

/*
Returns the dot product of v1 and v2.
*/
impl /*struct*/ QVector4D {
  pub fn dotProduct_0<RetType, T: QVector4D_dotProduct_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.dotProduct_0();
    // return 1;
  }
}
pub trait QVector4D_dotProduct_0<RetType> {
  fn dotProduct_0(self ) -> RetType;
}
impl<'a> /*trait*/ QVector4D_dotProduct_0<f32> for (usize,usize) {
  fn dotProduct_0(self ) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QVector4D10dotProductERKS_S1_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qvector4d.h:117
// index:0
// Public Visibility=Default Availability=Available
// [8] QVector2D toVector2D() const

/*
Returns the 2D vector form of this 4D vector, dropping the z and w coordinates.

See also toVector2DAffine(), toVector3D(), and toPoint().
*/
impl /*struct*/ QVector4D {
  pub fn toVector2D_0<RetType, T: QVector4D_toVector2D_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toVector2D_0(self);
    // return 1;
  }
}
pub trait QVector4D_toVector2D_0<RetType> {
  fn toVector2D_0(self , rsthis: & QVector4D) -> RetType;
}
impl<'a> /*trait*/ QVector4D_toVector2D_0<usize> for () {
  fn toVector2D_0(self , rsthis: & QVector4D) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QVector4D10toVector2DEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qvector4d.h:118
// index:0
// Public Visibility=Default Availability=Available
// [8] QVector2D toVector2DAffine() const

/*
Returns the 2D vector form of this 4D vector, dividing the x and y coordinates by the w coordinate and dropping the z coordinate. Returns a null vector if w is zero.

See also toVector2D(), toVector3DAffine(), and toPoint().
*/
impl /*struct*/ QVector4D {
  pub fn toVector2DAffine_0<RetType, T: QVector4D_toVector2DAffine_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toVector2DAffine_0(self);
    // return 1;
  }
}
pub trait QVector4D_toVector2DAffine_0<RetType> {
  fn toVector2DAffine_0(self , rsthis: & QVector4D) -> RetType;
}
impl<'a> /*trait*/ QVector4D_toVector2DAffine_0<usize> for () {
  fn toVector2DAffine_0(self , rsthis: & QVector4D) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QVector4D16toVector2DAffineEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qvector4d.h:121
// index:0
// Public Visibility=Default Availability=Available
// [12] QVector3D toVector3D() const

/*
Returns the 3D vector form of this 4D vector, dropping the w coordinate.

See also toVector3DAffine(), toVector2D(), and toPoint().
*/
impl /*struct*/ QVector4D {
  pub fn toVector3D_0<RetType, T: QVector4D_toVector3D_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toVector3D_0(self);
    // return 1;
  }
}
pub trait QVector4D_toVector3D_0<RetType> {
  fn toVector3D_0(self , rsthis: & QVector4D) -> RetType;
}
impl<'a> /*trait*/ QVector4D_toVector3D_0<usize> for () {
  fn toVector3D_0(self , rsthis: & QVector4D) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QVector4D10toVector3DEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qvector4d.h:122
// index:0
// Public Visibility=Default Availability=Available
// [12] QVector3D toVector3DAffine() const

/*
Returns the 3D vector form of this 4D vector, dividing the x, y, and z coordinates by the w coordinate. Returns a null vector if w is zero.

See also toVector3D(), toVector2DAffine(), and toPoint().
*/
impl /*struct*/ QVector4D {
  pub fn toVector3DAffine_0<RetType, T: QVector4D_toVector3DAffine_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toVector3DAffine_0(self);
    // return 1;
  }
}
pub trait QVector4D_toVector3DAffine_0<RetType> {
  fn toVector3DAffine_0(self , rsthis: & QVector4D) -> RetType;
}
impl<'a> /*trait*/ QVector4D_toVector3DAffine_0<usize> for () {
  fn toVector3DAffine_0(self , rsthis: & QVector4D) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QVector4D16toVector3DAffineEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qvector4d.h:125
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QPoint toPoint() const

/*
Returns the QPoint form of this 4D vector. The z and w coordinates are dropped.

See also toPointF() and toVector2D().
*/
impl /*struct*/ QVector4D {
  pub fn toPoint_0<RetType, T: QVector4D_toPoint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toPoint_0(self);
    // return 1;
  }
}
pub trait QVector4D_toPoint_0<RetType> {
  fn toPoint_0(self , rsthis: & QVector4D) -> RetType;
}
impl<'a> /*trait*/ QVector4D_toPoint_0<usize> for () {
  fn toPoint_0(self , rsthis: & QVector4D) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QVector4D7toPointEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qvector4d.h:126
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QPointF toPointF() const

/*
Returns the QPointF form of this 4D vector. The z and w coordinates are dropped.

See also toPoint() and toVector2D().
*/
impl /*struct*/ QVector4D {
  pub fn toPointF_0<RetType, T: QVector4D_toPointF_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toPointF_0(self);
    // return 1;
  }
}
pub trait QVector4D_toPointF_0<RetType> {
  fn toPointF_0(self , rsthis: & QVector4D) -> RetType;
}
impl<'a> /*trait*/ QVector4D_toPointF_0<usize> for () {
  fn toPointF_0(self , rsthis: & QVector4D) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QVector4D8toPointFEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}


pub fn DeleteQVector4D(this :*mut QVector4D) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN9QVector4DD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end