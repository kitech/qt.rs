

// mod ::widgets::QGraphicsRotation
// package qtwidgets
// /usr/include/qt/QtWidgets/qgraphicstransform.h
// #include <qgraphicstransform.h>
// #include <QtWidgets>

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
// import "github.com/kitech/qt.go/qtgui"
use qtgui::*; // super::super::%!s(MISSING)::*;
use std::default::Default;
use std::ops::Deref;
use qtrt; // super::super::qtrt;
use super::*;
//  ext block end

//  body block begin



/*

*/
#[derive(Default)] // class sizeof(QGraphicsRotation)=16
pub struct QGraphicsRotation {
  qbase: QGraphicsTransform,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QGraphicsRotation_ITF interface {
//    QGraphicsTransform_ITF
//    QGraphicsRotation_PTR() *QGraphicsRotation
//}
//func (ptr *QGraphicsRotation) QGraphicsRotation_PTR() *QGraphicsRotation { return ptr }

impl /*struct*/ QGraphicsRotation {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QGraphicsRotation {
    return QGraphicsRotation{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QGraphicsRotation {
//  type Target = QGraphicsRotationBASE;
//
//  fn deref(&self) -> &QGraphicsRotationBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QGraphicsRotationBASE> for QGraphicsRotation {
//  fn as_ref(& self) -> & QGraphicsRotationBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qgraphicstransform.h:120
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QGraphicsRotation {
  pub fn metaObject_0<RetType, T: QGraphicsRotation_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QGraphicsRotation_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QGraphicsRotation) -> RetType;
}
impl<'a> /*trait*/ QGraphicsRotation_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QGraphicsRotation) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QGraphicsRotation10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicstransform.h:126
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QGraphicsRotation(QObject *)

/*

*/
// QGraphicsRotation(QObject *) ctx.fn_proto_cpp
impl /*struct*/ QGraphicsRotation {
  pub fn QGraphicsRotation_0<T: QGraphicsRotation_QGraphicsRotation_0>(value: T) -> QGraphicsRotation {
    let rsthis = value.QGraphicsRotation_0();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsRotation_QGraphicsRotation_0 {
  fn QGraphicsRotation_0(self) -> QGraphicsRotation;
}
// QGraphicsRotation(QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QGraphicsRotation_QGraphicsRotation_0 for (usize) {
  fn QGraphicsRotation_0(self) -> QGraphicsRotation {
    // unsafe{_ZN17QGraphicsRotationC2EP7QObject()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN17QGraphicsRotationC2EP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QGraphicsRotation{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicstransform.h:127
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QGraphicsRotation()

/*

*/
pub fn DeleteQGraphicsRotation(this :*mut QGraphicsRotation) {
    // let rv = qtrt::InvokeQtFunc6("_ZN17QGraphicsRotationD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qgraphicstransform.h:129
// index:0
// Public Visibility=Default Availability=Available
// [12] QVector3D origin() const

/*

*/
impl /*struct*/ QGraphicsRotation {
  pub fn origin_0<RetType, T: QGraphicsRotation_origin_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.origin_0(self);
    // return 1;
  }
}
pub trait QGraphicsRotation_origin_0<RetType> {
  fn origin_0(self , rsthis: & QGraphicsRotation) -> RetType;
}
impl<'a> /*trait*/ QGraphicsRotation_origin_0<usize> for () {
  fn origin_0(self , rsthis: & QGraphicsRotation) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QGraphicsRotation6originEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicstransform.h:130
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setOrigin(const QVector3D &)

/*

*/
impl /*struct*/ QGraphicsRotation {
  pub fn setOrigin_0<RetType, T: QGraphicsRotation_setOrigin_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setOrigin_0(self);
    // return 1;
  }
}
pub trait QGraphicsRotation_setOrigin_0<RetType> {
  fn setOrigin_0(self , rsthis: & QGraphicsRotation) -> RetType;
}
impl<'a> /*trait*/ QGraphicsRotation_setOrigin_0<(/*void*/)> for (usize) {
  fn setOrigin_0(self , rsthis: & QGraphicsRotation) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN17QGraphicsRotation9setOriginERK9QVector3D", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicstransform.h:132
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal angle() const

/*

*/
impl /*struct*/ QGraphicsRotation {
  pub fn angle_0<RetType, T: QGraphicsRotation_angle_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.angle_0(self);
    // return 1;
  }
}
pub trait QGraphicsRotation_angle_0<RetType> {
  fn angle_0(self , rsthis: & QGraphicsRotation) -> RetType;
}
impl<'a> /*trait*/ QGraphicsRotation_angle_0<f64> for () {
  fn angle_0(self , rsthis: & QGraphicsRotation) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QGraphicsRotation5angleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicstransform.h:133
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setAngle(qreal)

/*

*/
impl /*struct*/ QGraphicsRotation {
  pub fn setAngle_0<RetType, T: QGraphicsRotation_setAngle_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setAngle_0(self);
    // return 1;
  }
}
pub trait QGraphicsRotation_setAngle_0<RetType> {
  fn setAngle_0(self , rsthis: & QGraphicsRotation) -> RetType;
}
impl<'a> /*trait*/ QGraphicsRotation_setAngle_0<(/*void*/)> for (f64) {
  fn setAngle_0(self , rsthis: & QGraphicsRotation) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN17QGraphicsRotation8setAngleEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicstransform.h:135
// index:0
// Public Visibility=Default Availability=Available
// [12] QVector3D axis() const

/*

*/
impl /*struct*/ QGraphicsRotation {
  pub fn axis_0<RetType, T: QGraphicsRotation_axis_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.axis_0(self);
    // return 1;
  }
}
pub trait QGraphicsRotation_axis_0<RetType> {
  fn axis_0(self , rsthis: & QGraphicsRotation) -> RetType;
}
impl<'a> /*trait*/ QGraphicsRotation_axis_0<usize> for () {
  fn axis_0(self , rsthis: & QGraphicsRotation) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QGraphicsRotation4axisEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicstransform.h:136
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setAxis(const QVector3D &)

/*

*/
impl /*struct*/ QGraphicsRotation {
  pub fn setAxis_0<RetType, T: QGraphicsRotation_setAxis_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setAxis_0(self);
    // return 1;
  }
}
pub trait QGraphicsRotation_setAxis_0<RetType> {
  fn setAxis_0(self , rsthis: & QGraphicsRotation) -> RetType;
}
impl<'a> /*trait*/ QGraphicsRotation_setAxis_0<(/*void*/)> for (usize) {
  fn setAxis_0(self , rsthis: & QGraphicsRotation) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN17QGraphicsRotation7setAxisERK9QVector3D", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicstransform.h:137
// index:1
// Public Visibility=Default Availability=Available
// [-2] void setAxis(Qt::Axis)

/*

*/
impl /*struct*/ QGraphicsRotation {
  pub fn setAxis_1<RetType, T: QGraphicsRotation_setAxis_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setAxis_1(self);
    // return 1;
  }
}
pub trait QGraphicsRotation_setAxis_1<RetType> {
  fn setAxis_1(self , rsthis: & QGraphicsRotation) -> RetType;
}
impl<'a> /*trait*/ QGraphicsRotation_setAxis_1<(/*void*/)> for (i32) {
  fn setAxis_1(self , rsthis: & QGraphicsRotation) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN17QGraphicsRotation7setAxisEN2Qt4AxisE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicstransform.h:139
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void applyTo(QMatrix4x4 *) const

/*
This pure virtual method has to be reimplemented in derived classes.

It applies this transformation to matrix.

See also QGraphicsItem::transform() and QMatrix4x4::toTransform().
*/
impl /*struct*/ QGraphicsRotation {
  pub fn applyTo_0<RetType, T: QGraphicsRotation_applyTo_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.applyTo_0(self);
    // return 1;
  }
}
pub trait QGraphicsRotation_applyTo_0<RetType> {
  fn applyTo_0(self , rsthis: & QGraphicsRotation) -> RetType;
}
impl<'a> /*trait*/ QGraphicsRotation_applyTo_0<(/*void*/)> for (usize) {
  fn applyTo_0(self , rsthis: & QGraphicsRotation) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK17QGraphicsRotation7applyToEP10QMatrix4x4", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicstransform.h:142
// index:0
// Public Visibility=Default Availability=Available
// [-2] void originChanged()

/*

*/
impl /*struct*/ QGraphicsRotation {
  pub fn originChanged_0<RetType, T: QGraphicsRotation_originChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.originChanged_0(self);
    // return 1;
  }
}
pub trait QGraphicsRotation_originChanged_0<RetType> {
  fn originChanged_0(self , rsthis: & QGraphicsRotation) -> RetType;
}
impl<'a> /*trait*/ QGraphicsRotation_originChanged_0<(/*void*/)> for () {
  fn originChanged_0(self , rsthis: & QGraphicsRotation) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN17QGraphicsRotation13originChangedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicstransform.h:143
// index:0
// Public Visibility=Default Availability=Available
// [-2] void angleChanged()

/*

*/
impl /*struct*/ QGraphicsRotation {
  pub fn angleChanged_0<RetType, T: QGraphicsRotation_angleChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.angleChanged_0(self);
    // return 1;
  }
}
pub trait QGraphicsRotation_angleChanged_0<RetType> {
  fn angleChanged_0(self , rsthis: & QGraphicsRotation) -> RetType;
}
impl<'a> /*trait*/ QGraphicsRotation_angleChanged_0<(/*void*/)> for () {
  fn angleChanged_0(self , rsthis: & QGraphicsRotation) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN17QGraphicsRotation12angleChangedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicstransform.h:144
// index:0
// Public Visibility=Default Availability=Available
// [-2] void axisChanged()

/*

*/
impl /*struct*/ QGraphicsRotation {
  pub fn axisChanged_0<RetType, T: QGraphicsRotation_axisChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.axisChanged_0(self);
    // return 1;
  }
}
pub trait QGraphicsRotation_axisChanged_0<RetType> {
  fn axisChanged_0(self , rsthis: & QGraphicsRotation) -> RetType;
}
impl<'a> /*trait*/ QGraphicsRotation_axisChanged_0<(/*void*/)> for () {
  fn axisChanged_0(self , rsthis: & QGraphicsRotation) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN17QGraphicsRotation11axisChangedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

//  body block end

//  keep block begin

//  keep block end
