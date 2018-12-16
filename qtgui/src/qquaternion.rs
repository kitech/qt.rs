

// mod ::gui::QQuaternion
// package qtgui
// /usr/include/qt/QtGui/qquaternion.h
// #include <qquaternion.h>
// #include <QtGui>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 37
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
#[derive(Default)] // class sizeof(QQuaternion)=16
pub struct QQuaternion {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QQuaternion_ITF interface {
//    QQuaternion_PTR() *QQuaternion
//}
//func (ptr *QQuaternion) QQuaternion_PTR() *QQuaternion { return ptr }

impl /*struct*/ QQuaternion {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QQuaternion {
    return QQuaternion{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QQuaternion {
//  type Target = QQuaternionBASE;
//
//  fn deref(&self) -> &QQuaternionBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QQuaternionBASE> for QQuaternion {
//  fn as_ref(& self) -> & QQuaternionBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qquaternion.h:59
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QQuaternion()

/*
Constructs an identity quaternion (1, 0, 0, 0), i.e. with the vector (0, 0, 0) and scalar 1.
*/
// QQuaternion() ctx.fn_proto_cpp
impl /*struct*/ QQuaternion {
  pub fn QQuaternion_0<T: QQuaternion_QQuaternion_0>(value: T) -> QQuaternion {
    let rsthis = value.QQuaternion_0();
    return rsthis;
    // return 1;
  }
}

pub trait QQuaternion_QQuaternion_0 {
  fn QQuaternion_0(self) -> QQuaternion;
}
// QQuaternion() ctx.fn_proto_cpp
impl<'a> /*trait*/ QQuaternion_QQuaternion_0 for () {
  fn QQuaternion_0(self) -> QQuaternion {
    // unsafe{_ZN11QQuaternionC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QQuaternionC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QQuaternion{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qquaternion.h:60
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void QQuaternion(Qt::Initialization)

/*
Constructs an identity quaternion (1, 0, 0, 0), i.e. with the vector (0, 0, 0) and scalar 1.
*/
// QQuaternion(Qt::Initialization) ctx.fn_proto_cpp
impl /*struct*/ QQuaternion {
  pub fn QQuaternion_1<T: QQuaternion_QQuaternion_1>(value: T) -> QQuaternion {
    let rsthis = value.QQuaternion_1();
    return rsthis;
    // return 1;
  }
}

pub trait QQuaternion_QQuaternion_1 {
  fn QQuaternion_1(self) -> QQuaternion;
}
// QQuaternion(Qt::Initialization) ctx.fn_proto_cpp
impl<'a> /*trait*/ QQuaternion_QQuaternion_1 for (i32) {
  fn QQuaternion_1(self) -> QQuaternion {
    // unsafe{_ZN11QQuaternionC2EN2Qt14InitializationE()};
    let arg0 = (&self) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QQuaternionC2EN2Qt14InitializationE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QQuaternion{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qquaternion.h:61
// index:2
// Public Visibility=Default Availability=Available
// [-2] void QQuaternion(float, float, float, float)

/*
Constructs an identity quaternion (1, 0, 0, 0), i.e. with the vector (0, 0, 0) and scalar 1.
*/
// QQuaternion(float, float, float, float) ctx.fn_proto_cpp
impl /*struct*/ QQuaternion {
  pub fn QQuaternion_2<T: QQuaternion_QQuaternion_2>(value: T) -> QQuaternion {
    let rsthis = value.QQuaternion_2();
    return rsthis;
    // return 1;
  }
}

pub trait QQuaternion_QQuaternion_2 {
  fn QQuaternion_2(self) -> QQuaternion;
}
// QQuaternion(float, float, float, float) ctx.fn_proto_cpp
impl<'a> /*trait*/ QQuaternion_QQuaternion_2 for (f32,f32,f32,f32) {
  fn QQuaternion_2(self) -> QQuaternion {
    // unsafe{_ZN11QQuaternionC2Effff()};
    let arg0 = (&self.0) as *const f32 as usize;
    let arg1 = (&self.1) as *const f32 as usize;
    let arg2 = (&self.2) as *const f32 as usize;
    let arg3 = (&self.3) as *const f32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QQuaternionC2Effff", 4,qtrt::FFITY_FLOAT,qtrt::FFITY_FLOAT,qtrt::FFITY_FLOAT,qtrt::FFITY_FLOAT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QQuaternion{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qquaternion.h:63
// index:3
// Public Visibility=Default Availability=Available
// [-2] void QQuaternion(float, const QVector3D &)

/*
Constructs an identity quaternion (1, 0, 0, 0), i.e. with the vector (0, 0, 0) and scalar 1.
*/
// QQuaternion(float, const QVector3D &) ctx.fn_proto_cpp
impl /*struct*/ QQuaternion {
  pub fn QQuaternion_3<T: QQuaternion_QQuaternion_3>(value: T) -> QQuaternion {
    let rsthis = value.QQuaternion_3();
    return rsthis;
    // return 1;
  }
}

pub trait QQuaternion_QQuaternion_3 {
  fn QQuaternion_3(self) -> QQuaternion;
}
// QQuaternion(float, const QVector3D &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QQuaternion_QQuaternion_3 for (f32,usize) {
  fn QQuaternion_3(self) -> QQuaternion {
    // unsafe{_ZN11QQuaternionC2EfRK9QVector3D()};
    let arg0 = (&self.0) as *const f32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QQuaternionC2EfRK9QVector3D", 2,qtrt::FFITY_FLOAT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QQuaternion{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qquaternion.h:66
// index:4
// Public Visibility=Default Availability=Available
// [-2] void QQuaternion(const QVector4D &)

/*
Constructs an identity quaternion (1, 0, 0, 0), i.e. with the vector (0, 0, 0) and scalar 1.
*/
// QQuaternion(const QVector4D &) ctx.fn_proto_cpp
impl /*struct*/ QQuaternion {
  pub fn QQuaternion_4<T: QQuaternion_QQuaternion_4>(value: T) -> QQuaternion {
    let rsthis = value.QQuaternion_4();
    return rsthis;
    // return 1;
  }
}

pub trait QQuaternion_QQuaternion_4 {
  fn QQuaternion_4(self) -> QQuaternion;
}
// QQuaternion(const QVector4D &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QQuaternion_QQuaternion_4 for (usize) {
  fn QQuaternion_4(self) -> QQuaternion {
    // unsafe{_ZN11QQuaternionC2ERK9QVector4D()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QQuaternionC2ERK9QVector4D", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QQuaternion{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qquaternion.h:69
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isNull() const

/*
Returns true if the x, y, z, and scalar components of this quaternion are set to 0.0; otherwise returns false.
*/
impl /*struct*/ QQuaternion {
  pub fn isNull_0<RetType, T: QQuaternion_isNull_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isNull_0(self);
    // return 1;
  }
}
pub trait QQuaternion_isNull_0<RetType> {
  fn isNull_0(self , rsthis: & QQuaternion) -> RetType;
}
impl<'a> /*trait*/ QQuaternion_isNull_0<bool> for () {
  fn isNull_0(self , rsthis: & QQuaternion) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QQuaternion6isNullEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qquaternion.h:70
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isIdentity() const

/*
Returns true if the x, y, and z components of this quaternion are set to 0.0, and the scalar component is set to 1.0; otherwise returns false.
*/
impl /*struct*/ QQuaternion {
  pub fn isIdentity_0<RetType, T: QQuaternion_isIdentity_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isIdentity_0(self);
    // return 1;
  }
}
pub trait QQuaternion_isIdentity_0<RetType> {
  fn isIdentity_0(self , rsthis: & QQuaternion) -> RetType;
}
impl<'a> /*trait*/ QQuaternion_isIdentity_0<bool> for () {
  fn isIdentity_0(self , rsthis: & QQuaternion) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QQuaternion10isIdentityEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qquaternion.h:73
// index:0
// Public Visibility=Default Availability=Available
// [12] QVector3D vector() const

/*
Returns the vector component of this quaternion.

See also setVector() and scalar().
*/
impl /*struct*/ QQuaternion {
  pub fn vector_0<RetType, T: QQuaternion_vector_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.vector_0(self);
    // return 1;
  }
}
pub trait QQuaternion_vector_0<RetType> {
  fn vector_0(self , rsthis: & QQuaternion) -> RetType;
}
impl<'a> /*trait*/ QQuaternion_vector_0<usize> for () {
  fn vector_0(self , rsthis: & QQuaternion) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QQuaternion6vectorEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qquaternion.h:74
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setVector(const QVector3D &)

/*
Sets the vector component of this quaternion to vector.

See also vector() and setScalar().
*/
impl /*struct*/ QQuaternion {
  pub fn setVector_0<RetType, T: QQuaternion_setVector_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setVector_0(self);
    // return 1;
  }
}
pub trait QQuaternion_setVector_0<RetType> {
  fn setVector_0(self , rsthis: & QQuaternion) -> RetType;
}
impl<'a> /*trait*/ QQuaternion_setVector_0<(/*void*/)> for (usize) {
  fn setVector_0(self , rsthis: & QQuaternion) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QQuaternion9setVectorERK9QVector3D", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qquaternion.h:76
// index:1
// Public Visibility=Default Availability=Available
// [-2] void setVector(float, float, float)

/*
Sets the vector component of this quaternion to vector.

See also vector() and setScalar().
*/
impl /*struct*/ QQuaternion {
  pub fn setVector_1<RetType, T: QQuaternion_setVector_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setVector_1(self);
    // return 1;
  }
}
pub trait QQuaternion_setVector_1<RetType> {
  fn setVector_1(self , rsthis: & QQuaternion) -> RetType;
}
impl<'a> /*trait*/ QQuaternion_setVector_1<(/*void*/)> for (f32,f32,f32) {
  fn setVector_1(self , rsthis: & QQuaternion) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f32 as usize;
    let arg1 = (&self.1) as *const f32 as usize;
    let arg2 = (&self.2) as *const f32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QQuaternion9setVectorEfff", 3,qtrt::FFITY_FLOAT,qtrt::FFITY_FLOAT,qtrt::FFITY_FLOAT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qquaternion.h:78
// index:0
// Public Visibility=Default Availability=Available
// [4] float x() const

/*
Returns the x coordinate of this quaternion's vector.

See also setX(), y(), z(), and scalar().
*/
impl /*struct*/ QQuaternion {
  pub fn x_0<RetType, T: QQuaternion_x_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.x_0(self);
    // return 1;
  }
}
pub trait QQuaternion_x_0<RetType> {
  fn x_0(self , rsthis: & QQuaternion) -> RetType;
}
impl<'a> /*trait*/ QQuaternion_x_0<f32> for () {
  fn x_0(self , rsthis: & QQuaternion) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QQuaternion1xEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qquaternion.h:79
// index:0
// Public Visibility=Default Availability=Available
// [4] float y() const

/*
Returns the y coordinate of this quaternion's vector.

See also setY(), x(), z(), and scalar().
*/
impl /*struct*/ QQuaternion {
  pub fn y_0<RetType, T: QQuaternion_y_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.y_0(self);
    // return 1;
  }
}
pub trait QQuaternion_y_0<RetType> {
  fn y_0(self , rsthis: & QQuaternion) -> RetType;
}
impl<'a> /*trait*/ QQuaternion_y_0<f32> for () {
  fn y_0(self , rsthis: & QQuaternion) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QQuaternion1yEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qquaternion.h:80
// index:0
// Public Visibility=Default Availability=Available
// [4] float z() const

/*
Returns the z coordinate of this quaternion's vector.

See also setZ(), x(), y(), and scalar().
*/
impl /*struct*/ QQuaternion {
  pub fn z_0<RetType, T: QQuaternion_z_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.z_0(self);
    // return 1;
  }
}
pub trait QQuaternion_z_0<RetType> {
  fn z_0(self , rsthis: & QQuaternion) -> RetType;
}
impl<'a> /*trait*/ QQuaternion_z_0<f32> for () {
  fn z_0(self , rsthis: & QQuaternion) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QQuaternion1zEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qquaternion.h:81
// index:0
// Public Visibility=Default Availability=Available
// [4] float scalar() const

/*
Returns the scalar component of this quaternion.

See also setScalar(), x(), y(), and z().
*/
impl /*struct*/ QQuaternion {
  pub fn scalar_0<RetType, T: QQuaternion_scalar_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.scalar_0(self);
    // return 1;
  }
}
pub trait QQuaternion_scalar_0<RetType> {
  fn scalar_0(self , rsthis: & QQuaternion) -> RetType;
}
impl<'a> /*trait*/ QQuaternion_scalar_0<f32> for () {
  fn scalar_0(self , rsthis: & QQuaternion) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QQuaternion6scalarEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qquaternion.h:83
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setX(float)

/*
Sets the x coordinate of this quaternion's vector to the given x coordinate.

See also x(), setY(), setZ(), and setScalar().
*/
impl /*struct*/ QQuaternion {
  pub fn setX_0<RetType, T: QQuaternion_setX_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setX_0(self);
    // return 1;
  }
}
pub trait QQuaternion_setX_0<RetType> {
  fn setX_0(self , rsthis: & QQuaternion) -> RetType;
}
impl<'a> /*trait*/ QQuaternion_setX_0<(/*void*/)> for (f32) {
  fn setX_0(self , rsthis: & QQuaternion) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QQuaternion4setXEf", 1,qtrt::FFITY_FLOAT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qquaternion.h:84
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setY(float)

/*
Sets the y coordinate of this quaternion's vector to the given y coordinate.

See also y(), setX(), setZ(), and setScalar().
*/
impl /*struct*/ QQuaternion {
  pub fn setY_0<RetType, T: QQuaternion_setY_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setY_0(self);
    // return 1;
  }
}
pub trait QQuaternion_setY_0<RetType> {
  fn setY_0(self , rsthis: & QQuaternion) -> RetType;
}
impl<'a> /*trait*/ QQuaternion_setY_0<(/*void*/)> for (f32) {
  fn setY_0(self , rsthis: & QQuaternion) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QQuaternion4setYEf", 1,qtrt::FFITY_FLOAT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qquaternion.h:85
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setZ(float)

/*
Sets the z coordinate of this quaternion's vector to the given z coordinate.

See also z(), setX(), setY(), and setScalar().
*/
impl /*struct*/ QQuaternion {
  pub fn setZ_0<RetType, T: QQuaternion_setZ_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setZ_0(self);
    // return 1;
  }
}
pub trait QQuaternion_setZ_0<RetType> {
  fn setZ_0(self , rsthis: & QQuaternion) -> RetType;
}
impl<'a> /*trait*/ QQuaternion_setZ_0<(/*void*/)> for (f32) {
  fn setZ_0(self , rsthis: & QQuaternion) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QQuaternion4setZEf", 1,qtrt::FFITY_FLOAT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qquaternion.h:86
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setScalar(float)

/*
Sets the scalar component of this quaternion to scalar.

See also scalar(), setX(), setY(), and setZ().
*/
impl /*struct*/ QQuaternion {
  pub fn setScalar_0<RetType, T: QQuaternion_setScalar_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setScalar_0(self);
    // return 1;
  }
}
pub trait QQuaternion_setScalar_0<RetType> {
  fn setScalar_0(self , rsthis: & QQuaternion) -> RetType;
}
impl<'a> /*trait*/ QQuaternion_setScalar_0<(/*void*/)> for (f32) {
  fn setScalar_0(self , rsthis: & QQuaternion) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QQuaternion9setScalarEf", 1,qtrt::FFITY_FLOAT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qquaternion.h:88
// index:0
// Public static inline Visibility=Default Availability=Available
// [4] float dotProduct(const QQuaternion &, const QQuaternion &)

/*
Returns the dot product of q1 and q2.

This function was introduced in  Qt 5.5.

See also length().
*/
impl /*struct*/ QQuaternion {
  pub fn dotProduct_0<RetType, T: QQuaternion_dotProduct_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.dotProduct_0();
    // return 1;
  }
}
pub trait QQuaternion_dotProduct_0<RetType> {
  fn dotProduct_0(self ) -> RetType;
}
impl<'a> /*trait*/ QQuaternion_dotProduct_0<f32> for (usize,usize) {
  fn dotProduct_0(self ) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QQuaternion10dotProductERKS_S1_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qquaternion.h:90
// index:0
// Public Visibility=Default Availability=Available
// [4] float length() const

/*
Returns the length of the quaternion. This is also called the "norm".

See also lengthSquared(), normalized(), and dotProduct().
*/
impl /*struct*/ QQuaternion {
  pub fn length_0<RetType, T: QQuaternion_length_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.length_0(self);
    // return 1;
  }
}
pub trait QQuaternion_length_0<RetType> {
  fn length_0(self , rsthis: & QQuaternion) -> RetType;
}
impl<'a> /*trait*/ QQuaternion_length_0<f32> for () {
  fn length_0(self , rsthis: & QQuaternion) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QQuaternion6lengthEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qquaternion.h:91
// index:0
// Public Visibility=Default Availability=Available
// [4] float lengthSquared() const

/*
Returns the squared length of the quaternion.

See also length() and dotProduct().
*/
impl /*struct*/ QQuaternion {
  pub fn lengthSquared_0<RetType, T: QQuaternion_lengthSquared_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.lengthSquared_0(self);
    // return 1;
  }
}
pub trait QQuaternion_lengthSquared_0<RetType> {
  fn lengthSquared_0(self , rsthis: & QQuaternion) -> RetType;
}
impl<'a> /*trait*/ QQuaternion_lengthSquared_0<f32> for () {
  fn lengthSquared_0(self , rsthis: & QQuaternion) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QQuaternion13lengthSquaredEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qquaternion.h:93
// index:0
// Public Visibility=Default Availability=Available
// [16] QQuaternion normalized() const

/*
Returns the normalized unit form of this quaternion.

If this quaternion is null, then a null quaternion is returned. If the length of the quaternion is very close to 1, then the quaternion will be returned as-is. Otherwise the normalized form of the quaternion of length 1 will be returned.

See also normalize(), length(), and dotProduct().
*/
impl /*struct*/ QQuaternion {
  pub fn normalized_0<RetType, T: QQuaternion_normalized_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.normalized_0(self);
    // return 1;
  }
}
pub trait QQuaternion_normalized_0<RetType> {
  fn normalized_0(self , rsthis: & QQuaternion) -> RetType;
}
impl<'a> /*trait*/ QQuaternion_normalized_0<usize> for () {
  fn normalized_0(self , rsthis: & QQuaternion) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QQuaternion10normalizedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qquaternion.h:94
// index:0
// Public Visibility=Default Availability=Available
// [-2] void normalize()

/*
Normalizes the current quaternion in place. Nothing happens if this is a null quaternion or the length of the quaternion is very close to 1.

See also length() and normalized().
*/
impl /*struct*/ QQuaternion {
  pub fn normalize_0<RetType, T: QQuaternion_normalize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.normalize_0(self);
    // return 1;
  }
}
pub trait QQuaternion_normalize_0<RetType> {
  fn normalize_0(self , rsthis: & QQuaternion) -> RetType;
}
impl<'a> /*trait*/ QQuaternion_normalize_0<(/*void*/)> for () {
  fn normalize_0(self , rsthis: & QQuaternion) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN11QQuaternion9normalizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qquaternion.h:96
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QQuaternion inverted() const

/*
Returns the inverse of this quaternion. If this quaternion is null, then a null quaternion is returned.

This function was introduced in  Qt 5.5.

See also isNull() and length().
*/
impl /*struct*/ QQuaternion {
  pub fn inverted_0<RetType, T: QQuaternion_inverted_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.inverted_0(self);
    // return 1;
  }
}
pub trait QQuaternion_inverted_0<RetType> {
  fn inverted_0(self , rsthis: & QQuaternion) -> RetType;
}
impl<'a> /*trait*/ QQuaternion_inverted_0<usize> for () {
  fn inverted_0(self , rsthis: & QQuaternion) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QQuaternion8invertedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qquaternion.h:98
// index:0
// Public Visibility=Default Availability=Available
// [16] QQuaternion conjugated() const

/*
Returns the conjugate of this quaternion, which is (-x, -y, -z, scalar).

This function was introduced in  Qt 5.5.
*/
impl /*struct*/ QQuaternion {
  pub fn conjugated_0<RetType, T: QQuaternion_conjugated_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.conjugated_0(self);
    // return 1;
  }
}
pub trait QQuaternion_conjugated_0<RetType> {
  fn conjugated_0(self , rsthis: & QQuaternion) -> RetType;
}
impl<'a> /*trait*/ QQuaternion_conjugated_0<usize> for () {
  fn conjugated_0(self , rsthis: & QQuaternion) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QQuaternion10conjugatedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qquaternion.h:100
// index:0
// Public Visibility=Default Availability=Available
// [16] QQuaternion conjugate() const

/*

*/
impl /*struct*/ QQuaternion {
  pub fn conjugate_0<RetType, T: QQuaternion_conjugate_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.conjugate_0(self);
    // return 1;
  }
}
pub trait QQuaternion_conjugate_0<RetType> {
  fn conjugate_0(self , rsthis: & QQuaternion) -> RetType;
}
impl<'a> /*trait*/ QQuaternion_conjugate_0<usize> for () {
  fn conjugate_0(self , rsthis: & QQuaternion) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QQuaternion9conjugateEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qquaternion.h:103
// index:0
// Public Visibility=Default Availability=Available
// [12] QVector3D rotatedVector(const QVector3D &) const

/*
Rotates vector with this quaternion to produce a new vector in 3D space. The following code:


  QVector3D result = q.rotatedVector(vector);



is equivalent to the following:


  QVector3D result = (q * QQuaternion(0, vector) * q.conjugated()).vector();
*/
impl /*struct*/ QQuaternion {
  pub fn rotatedVector_0<RetType, T: QQuaternion_rotatedVector_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rotatedVector_0(self);
    // return 1;
  }
}
pub trait QQuaternion_rotatedVector_0<RetType> {
  fn rotatedVector_0(self , rsthis: & QQuaternion) -> RetType;
}
impl<'a> /*trait*/ QQuaternion_rotatedVector_0<usize> for (usize) {
  fn rotatedVector_0(self , rsthis: & QQuaternion) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QQuaternion13rotatedVectorERK9QVector3D", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qquaternion.h:105
// index:0
// Public Visibility=Default Availability=Available
// [16] QQuaternion & operator+=(const QQuaternion &)

/*

*/
impl /*struct*/ QQuaternion {
  pub fn operator_add_equal_0<RetType, T: QQuaternion_operator_add_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_add_equal_0(self);
    // return 1;
  }
}
pub trait QQuaternion_operator_add_equal_0<RetType> {
  fn operator_add_equal_0(self , rsthis: & QQuaternion) -> RetType;
}
impl<'a> /*trait*/ QQuaternion_operator_add_equal_0<usize> for (usize) {
  fn operator_add_equal_0(self , rsthis: & QQuaternion) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QQuaternionpLERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qquaternion.h:106
// index:0
// Public Visibility=Default Availability=Available
// [16] QQuaternion & operator-=(const QQuaternion &)

/*

*/
impl /*struct*/ QQuaternion {
  pub fn operator_minus_equal_0<RetType, T: QQuaternion_operator_minus_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_minus_equal_0(self);
    // return 1;
  }
}
pub trait QQuaternion_operator_minus_equal_0<RetType> {
  fn operator_minus_equal_0(self , rsthis: & QQuaternion) -> RetType;
}
impl<'a> /*trait*/ QQuaternion_operator_minus_equal_0<usize> for (usize) {
  fn operator_minus_equal_0(self , rsthis: & QQuaternion) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QQuaternionmIERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qquaternion.h:107
// index:0
// Public Visibility=Default Availability=Available
// [16] QQuaternion & operator*=(float)

/*

*/
impl /*struct*/ QQuaternion {
  pub fn operator_mul_equal_0<RetType, T: QQuaternion_operator_mul_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_mul_equal_0(self);
    // return 1;
  }
}
pub trait QQuaternion_operator_mul_equal_0<RetType> {
  fn operator_mul_equal_0(self , rsthis: & QQuaternion) -> RetType;
}
impl<'a> /*trait*/ QQuaternion_operator_mul_equal_0<usize> for (f32) {
  fn operator_mul_equal_0(self , rsthis: & QQuaternion) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QQuaternionmLEf", 1,qtrt::FFITY_FLOAT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qquaternion.h:108
// index:1
// Public Visibility=Default Availability=Available
// [16] QQuaternion & operator*=(const QQuaternion &)

/*

*/
impl /*struct*/ QQuaternion {
  pub fn operator_mul_equal_1<RetType, T: QQuaternion_operator_mul_equal_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_mul_equal_1(self);
    // return 1;
  }
}
pub trait QQuaternion_operator_mul_equal_1<RetType> {
  fn operator_mul_equal_1(self , rsthis: & QQuaternion) -> RetType;
}
impl<'a> /*trait*/ QQuaternion_operator_mul_equal_1<usize> for (usize) {
  fn operator_mul_equal_1(self , rsthis: & QQuaternion) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QQuaternionmLERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qquaternion.h:109
// index:0
// Public Visibility=Default Availability=Available
// [16] QQuaternion & operator/=(float)

/*

*/
impl /*struct*/ QQuaternion {
  pub fn operator_div_equal_0<RetType, T: QQuaternion_operator_div_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_div_equal_0(self);
    // return 1;
  }
}
pub trait QQuaternion_operator_div_equal_0<RetType> {
  fn operator_div_equal_0(self , rsthis: & QQuaternion) -> RetType;
}
impl<'a> /*trait*/ QQuaternion_operator_div_equal_0<usize> for (f32) {
  fn operator_div_equal_0(self , rsthis: & QQuaternion) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QQuaterniondVEf", 1,qtrt::FFITY_FLOAT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qquaternion.h:124
// index:0
// Public Visibility=Default Availability=Available
// [16] QVector4D toVector4D() const

/*
Returns this quaternion as a 4D vector.
*/
impl /*struct*/ QQuaternion {
  pub fn toVector4D_0<RetType, T: QQuaternion_toVector4D_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toVector4D_0(self);
    // return 1;
  }
}
pub trait QQuaternion_toVector4D_0<RetType> {
  fn toVector4D_0(self , rsthis: & QQuaternion) -> RetType;
}
impl<'a> /*trait*/ QQuaternion_toVector4D_0<usize> for () {
  fn toVector4D_0(self , rsthis: & QQuaternion) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QQuaternion10toVector4DEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qquaternion.h:130
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void getAxisAndAngle(QVector3D *, float *) const

/*
Extracts a 3D axis (x, y, z) and a rotating angle angle (in degrees) that corresponds to this quaternion.

This function was introduced in  Qt 5.5.

See also fromAxisAndAngle().
*/
impl /*struct*/ QQuaternion {
  pub fn getAxisAndAngle_0<RetType, T: QQuaternion_getAxisAndAngle_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.getAxisAndAngle_0(self);
    // return 1;
  }
}
pub trait QQuaternion_getAxisAndAngle_0<RetType> {
  fn getAxisAndAngle_0(self , rsthis: & QQuaternion) -> RetType;
}
impl<'a> /*trait*/ QQuaternion_getAxisAndAngle_0<(/*void*/)> for (usize,usize) {
  fn getAxisAndAngle_0(self , rsthis: & QQuaternion) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK11QQuaternion15getAxisAndAngleEP9QVector3DPf", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qquaternion.h:133
// index:1
// Public Visibility=Default Availability=Available
// [-2] void getAxisAndAngle(float *, float *, float *, float *) const

/*
Extracts a 3D axis (x, y, z) and a rotating angle angle (in degrees) that corresponds to this quaternion.

This function was introduced in  Qt 5.5.

See also fromAxisAndAngle().
*/
impl /*struct*/ QQuaternion {
  pub fn getAxisAndAngle_1<RetType, T: QQuaternion_getAxisAndAngle_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.getAxisAndAngle_1(self);
    // return 1;
  }
}
pub trait QQuaternion_getAxisAndAngle_1<RetType> {
  fn getAxisAndAngle_1(self , rsthis: & QQuaternion) -> RetType;
}
impl<'a> /*trait*/ QQuaternion_getAxisAndAngle_1<(/*void*/)> for (usize,usize,usize,usize) {
  fn getAxisAndAngle_1(self , rsthis: & QQuaternion) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const usize as usize;
    let arg2 = (&self.2) as *const usize as usize;
    let arg3 = (&self.3) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK11QQuaternion15getAxisAndAngleEPfS0_S0_S0_", 4,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qquaternion.h:131
// index:0
// Public static Visibility=Default Availability=Available
// [16] QQuaternion fromAxisAndAngle(const QVector3D &, float)

/*
Creates a normalized quaternion that corresponds to rotating through angle degrees about the specified 3D axis.

See also getAxisAndAngle().
*/
impl /*struct*/ QQuaternion {
  pub fn fromAxisAndAngle_0<RetType, T: QQuaternion_fromAxisAndAngle_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromAxisAndAngle_0();
    // return 1;
  }
}
pub trait QQuaternion_fromAxisAndAngle_0<RetType> {
  fn fromAxisAndAngle_0(self ) -> RetType;
}
impl<'a> /*trait*/ QQuaternion_fromAxisAndAngle_0<usize> for (usize,f32) {
  fn fromAxisAndAngle_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const f32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QQuaternion16fromAxisAndAngleERK9QVector3Df", 2,qtrt::FFITY_POINTER,qtrt::FFITY_FLOAT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qquaternion.h:134
// index:1
// Public static Visibility=Default Availability=Available
// [16] QQuaternion fromAxisAndAngle(float, float, float, float)

/*
Creates a normalized quaternion that corresponds to rotating through angle degrees about the specified 3D axis.

See also getAxisAndAngle().
*/
impl /*struct*/ QQuaternion {
  pub fn fromAxisAndAngle_1<RetType, T: QQuaternion_fromAxisAndAngle_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromAxisAndAngle_1();
    // return 1;
  }
}
pub trait QQuaternion_fromAxisAndAngle_1<RetType> {
  fn fromAxisAndAngle_1(self ) -> RetType;
}
impl<'a> /*trait*/ QQuaternion_fromAxisAndAngle_1<usize> for (f32,f32,f32,f32) {
  fn fromAxisAndAngle_1(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f32 as usize;
    let arg1 = (&self.1) as *const f32 as usize;
    let arg2 = (&self.2) as *const f32 as usize;
    let arg3 = (&self.3) as *const f32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QQuaternion16fromAxisAndAngleEffff", 4,qtrt::FFITY_FLOAT,qtrt::FFITY_FLOAT,qtrt::FFITY_FLOAT,qtrt::FFITY_FLOAT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qquaternion.h:138
// index:0
// Public inline Visibility=Default Availability=Available
// [12] QVector3D toEulerAngles() const

/*
This is an overloaded function.

Calculates roll, pitch, and yaw Euler angles (in degrees) that corresponds to this quaternion.

This function was introduced in  Qt 5.5.

See also fromEulerAngles().
*/
impl /*struct*/ QQuaternion {
  pub fn toEulerAngles_0<RetType, T: QQuaternion_toEulerAngles_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toEulerAngles_0(self);
    // return 1;
  }
}
pub trait QQuaternion_toEulerAngles_0<RetType> {
  fn toEulerAngles_0(self , rsthis: & QQuaternion) -> RetType;
}
impl<'a> /*trait*/ QQuaternion_toEulerAngles_0<usize> for () {
  fn toEulerAngles_0(self , rsthis: & QQuaternion) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QQuaternion13toEulerAnglesEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qquaternion.h:139
// index:0
// Public static inline Visibility=Default Availability=Available
// [16] QQuaternion fromEulerAngles(const QVector3D &)

/*
Creates a quaternion that corresponds to a rotation of roll degrees around the z axis, pitch degrees around the x axis, and yaw degrees around the y axis (in that order).

This function was introduced in  Qt 5.5.

See also getEulerAngles().
*/
impl /*struct*/ QQuaternion {
  pub fn fromEulerAngles_0<RetType, T: QQuaternion_fromEulerAngles_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromEulerAngles_0();
    // return 1;
  }
}
pub trait QQuaternion_fromEulerAngles_0<RetType> {
  fn fromEulerAngles_0(self ) -> RetType;
}
impl<'a> /*trait*/ QQuaternion_fromEulerAngles_0<usize> for (usize) {
  fn fromEulerAngles_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QQuaternion15fromEulerAnglesERK9QVector3D", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qquaternion.h:142
// index:1
// Public static Visibility=Default Availability=Available
// [16] QQuaternion fromEulerAngles(float, float, float)

/*
Creates a quaternion that corresponds to a rotation of roll degrees around the z axis, pitch degrees around the x axis, and yaw degrees around the y axis (in that order).

This function was introduced in  Qt 5.5.

See also getEulerAngles().
*/
impl /*struct*/ QQuaternion {
  pub fn fromEulerAngles_1<RetType, T: QQuaternion_fromEulerAngles_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromEulerAngles_1();
    // return 1;
  }
}
pub trait QQuaternion_fromEulerAngles_1<RetType> {
  fn fromEulerAngles_1(self ) -> RetType;
}
impl<'a> /*trait*/ QQuaternion_fromEulerAngles_1<usize> for (f32,f32,f32) {
  fn fromEulerAngles_1(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f32 as usize;
    let arg1 = (&self.1) as *const f32 as usize;
    let arg2 = (&self.2) as *const f32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QQuaternion15fromEulerAnglesEfff", 3,qtrt::FFITY_FLOAT,qtrt::FFITY_FLOAT,qtrt::FFITY_FLOAT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qquaternion.h:141
// index:0
// Public Visibility=Default Availability=Available
// [-2] void getEulerAngles(float *, float *, float *) const

/*
Calculates roll, pitch, and yaw Euler angles (in degrees) that corresponds to this quaternion.

This function was introduced in  Qt 5.5.

See also fromEulerAngles().
*/
impl /*struct*/ QQuaternion {
  pub fn getEulerAngles_0<RetType, T: QQuaternion_getEulerAngles_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.getEulerAngles_0(self);
    // return 1;
  }
}
pub trait QQuaternion_getEulerAngles_0<RetType> {
  fn getEulerAngles_0(self , rsthis: & QQuaternion) -> RetType;
}
impl<'a> /*trait*/ QQuaternion_getEulerAngles_0<(/*void*/)> for (usize,usize,usize) {
  fn getEulerAngles_0(self , rsthis: & QQuaternion) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const usize as usize;
    let arg2 = (&self.2) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK11QQuaternion14getEulerAnglesEPfS0_S0_", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qquaternion.h:148
// index:0
// Public Visibility=Default Availability=Available
// [-2] void getAxes(QVector3D *, QVector3D *, QVector3D *) const

/*
Returns the 3 orthonormal axes (xAxis, yAxis, zAxis) defining the quaternion.

This function was introduced in  Qt 5.5.

See also fromAxes() and toRotationMatrix().
*/
impl /*struct*/ QQuaternion {
  pub fn getAxes_0<RetType, T: QQuaternion_getAxes_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.getAxes_0(self);
    // return 1;
  }
}
pub trait QQuaternion_getAxes_0<RetType> {
  fn getAxes_0(self , rsthis: & QQuaternion) -> RetType;
}
impl<'a> /*trait*/ QQuaternion_getAxes_0<(/*void*/)> for (usize,usize,usize) {
  fn getAxes_0(self , rsthis: & QQuaternion) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK11QQuaternion7getAxesEP9QVector3DS1_S1_", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qquaternion.h:149
// index:0
// Public static Visibility=Default Availability=Available
// [16] QQuaternion fromAxes(const QVector3D &, const QVector3D &, const QVector3D &)

/*
Constructs the quaternion using 3 axes (xAxis, yAxis, zAxis).

Note: The axes are assumed to be orthonormal.

This function was introduced in  Qt 5.5.

See also getAxes() and fromRotationMatrix().
*/
impl /*struct*/ QQuaternion {
  pub fn fromAxes_0<RetType, T: QQuaternion_fromAxes_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromAxes_0();
    // return 1;
  }
}
pub trait QQuaternion_fromAxes_0<RetType> {
  fn fromAxes_0(self ) -> RetType;
}
impl<'a> /*trait*/ QQuaternion_fromAxes_0<usize> for (usize,usize,usize) {
  fn fromAxes_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QQuaternion8fromAxesERK9QVector3DS2_S2_", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qquaternion.h:151
// index:0
// Public static Visibility=Default Availability=Available
// [16] QQuaternion fromDirection(const QVector3D &, const QVector3D &)

/*
Constructs the quaternion using specified forward direction direction and upward direction up. If the upward direction was not specified or the forward and upward vectors are collinear, a new orthonormal upward direction will be generated.

This function was introduced in  Qt 5.5.

See also fromAxes() and rotationTo().
*/
impl /*struct*/ QQuaternion {
  pub fn fromDirection_0<RetType, T: QQuaternion_fromDirection_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromDirection_0();
    // return 1;
  }
}
pub trait QQuaternion_fromDirection_0<RetType> {
  fn fromDirection_0(self ) -> RetType;
}
impl<'a> /*trait*/ QQuaternion_fromDirection_0<usize> for (usize,usize) {
  fn fromDirection_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QQuaternion13fromDirectionERK9QVector3DS2_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qquaternion.h:153
// index:0
// Public static Visibility=Default Availability=Available
// [16] QQuaternion rotationTo(const QVector3D &, const QVector3D &)

/*
Returns the shortest arc quaternion to rotate from the direction described by the vector from to the direction described by the vector to.

This function was introduced in  Qt 5.5.

See also fromDirection().
*/
impl /*struct*/ QQuaternion {
  pub fn rotationTo_0<RetType, T: QQuaternion_rotationTo_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.rotationTo_0();
    // return 1;
  }
}
pub trait QQuaternion_rotationTo_0<RetType> {
  fn rotationTo_0(self ) -> RetType;
}
impl<'a> /*trait*/ QQuaternion_rotationTo_0<usize> for (usize,usize) {
  fn rotationTo_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QQuaternion10rotationToERK9QVector3DS2_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qquaternion.h:156
// index:0
// Public static Visibility=Default Availability=Available
// [16] QQuaternion slerp(const QQuaternion &, const QQuaternion &, float)

/*
Interpolates along the shortest spherical path between the rotational positions q1 and q2. The value t should be between 0 and 1, indicating the spherical distance to travel between q1 and q2.

If t is less than or equal to 0, then q1 will be returned. If t is greater than or equal to 1, then q2 will be returned.

See also nlerp().
*/
impl /*struct*/ QQuaternion {
  pub fn slerp_0<RetType, T: QQuaternion_slerp_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.slerp_0();
    // return 1;
  }
}
pub trait QQuaternion_slerp_0<RetType> {
  fn slerp_0(self ) -> RetType;
}
impl<'a> /*trait*/ QQuaternion_slerp_0<usize> for (usize,usize,f32) {
  fn slerp_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const f32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QQuaternion5slerpERKS_S1_f", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_FLOAT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qquaternion.h:158
// index:0
// Public static Visibility=Default Availability=Available
// [16] QQuaternion nlerp(const QQuaternion &, const QQuaternion &, float)

/*
Interpolates along the shortest linear path between the rotational positions q1 and q2. The value t should be between 0 and 1, indicating the distance to travel between q1 and q2. The result will be normalized().

If t is less than or equal to 0, then q1 will be returned. If t is greater than or equal to 1, then q2 will be returned.

The nlerp() function is typically faster than slerp() and will give approximate results to spherical interpolation that are good enough for some applications.

See also slerp().
*/
impl /*struct*/ QQuaternion {
  pub fn nlerp_0<RetType, T: QQuaternion_nlerp_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.nlerp_0();
    // return 1;
  }
}
pub trait QQuaternion_nlerp_0<RetType> {
  fn nlerp_0(self ) -> RetType;
}
impl<'a> /*trait*/ QQuaternion_nlerp_0<usize> for (usize,usize,f32) {
  fn nlerp_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const f32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QQuaternion5nlerpERKS_S1_f", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_FLOAT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}


pub fn DeleteQQuaternion(this :*mut QQuaternion) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN11QQuaternionD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
