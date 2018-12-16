

// mod ::widgets::QGraphicsScale
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
// extern C begin: 5
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
#[derive(Default)] // class sizeof(QGraphicsScale)=16
pub struct QGraphicsScale {
  qbase: QGraphicsTransform,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QGraphicsScale_ITF interface {
//    QGraphicsTransform_ITF
//    QGraphicsScale_PTR() *QGraphicsScale
//}
//func (ptr *QGraphicsScale) QGraphicsScale_PTR() *QGraphicsScale { return ptr }

impl /*struct*/ QGraphicsScale {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QGraphicsScale {
    return QGraphicsScale{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QGraphicsScale {
//  type Target = QGraphicsScaleBASE;
//
//  fn deref(&self) -> &QGraphicsScaleBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QGraphicsScaleBASE> for QGraphicsScale {
//  fn as_ref(& self) -> & QGraphicsScaleBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qgraphicstransform.h:81
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QGraphicsScale {
  pub fn metaObject_0<RetType, T: QGraphicsScale_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QGraphicsScale_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QGraphicsScale) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScale_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QGraphicsScale) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QGraphicsScale10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicstransform.h:88
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QGraphicsScale(QObject *)

/*

*/
// QGraphicsScale(QObject *) ctx.fn_proto_cpp
impl /*struct*/ QGraphicsScale {
  pub fn QGraphicsScale_0<T: QGraphicsScale_QGraphicsScale_0>(value: T) -> QGraphicsScale {
    let rsthis = value.QGraphicsScale_0();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsScale_QGraphicsScale_0 {
  fn QGraphicsScale_0(self) -> QGraphicsScale;
}
// QGraphicsScale(QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QGraphicsScale_QGraphicsScale_0 for (usize) {
  fn QGraphicsScale_0(self) -> QGraphicsScale {
    // unsafe{_ZN14QGraphicsScaleC2EP7QObject()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN14QGraphicsScaleC2EP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QGraphicsScale{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicstransform.h:89
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QGraphicsScale()

/*

*/
pub fn DeleteQGraphicsScale(this :*mut QGraphicsScale) {
    // let rv = qtrt::InvokeQtFunc6("_ZN14QGraphicsScaleD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qgraphicstransform.h:91
// index:0
// Public Visibility=Default Availability=Available
// [12] QVector3D origin() const

/*

*/
impl /*struct*/ QGraphicsScale {
  pub fn origin_0<RetType, T: QGraphicsScale_origin_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.origin_0(self);
    // return 1;
  }
}
pub trait QGraphicsScale_origin_0<RetType> {
  fn origin_0(self , rsthis: & QGraphicsScale) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScale_origin_0<usize> for () {
  fn origin_0(self , rsthis: & QGraphicsScale) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QGraphicsScale6originEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicstransform.h:92
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setOrigin(const QVector3D &)

/*

*/
impl /*struct*/ QGraphicsScale {
  pub fn setOrigin_0<RetType, T: QGraphicsScale_setOrigin_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setOrigin_0(self);
    // return 1;
  }
}
pub trait QGraphicsScale_setOrigin_0<RetType> {
  fn setOrigin_0(self , rsthis: & QGraphicsScale) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScale_setOrigin_0<(/*void*/)> for (usize) {
  fn setOrigin_0(self , rsthis: & QGraphicsScale) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN14QGraphicsScale9setOriginERK9QVector3D", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicstransform.h:94
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal xScale() const

/*

*/
impl /*struct*/ QGraphicsScale {
  pub fn xScale_0<RetType, T: QGraphicsScale_xScale_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.xScale_0(self);
    // return 1;
  }
}
pub trait QGraphicsScale_xScale_0<RetType> {
  fn xScale_0(self , rsthis: & QGraphicsScale) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScale_xScale_0<f64> for () {
  fn xScale_0(self , rsthis: & QGraphicsScale) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QGraphicsScale6xScaleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicstransform.h:95
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setXScale(qreal)

/*

*/
impl /*struct*/ QGraphicsScale {
  pub fn setXScale_0<RetType, T: QGraphicsScale_setXScale_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setXScale_0(self);
    // return 1;
  }
}
pub trait QGraphicsScale_setXScale_0<RetType> {
  fn setXScale_0(self , rsthis: & QGraphicsScale) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScale_setXScale_0<(/*void*/)> for (f64) {
  fn setXScale_0(self , rsthis: & QGraphicsScale) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN14QGraphicsScale9setXScaleEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicstransform.h:97
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal yScale() const

/*

*/
impl /*struct*/ QGraphicsScale {
  pub fn yScale_0<RetType, T: QGraphicsScale_yScale_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.yScale_0(self);
    // return 1;
  }
}
pub trait QGraphicsScale_yScale_0<RetType> {
  fn yScale_0(self , rsthis: & QGraphicsScale) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScale_yScale_0<f64> for () {
  fn yScale_0(self , rsthis: & QGraphicsScale) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QGraphicsScale6yScaleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicstransform.h:98
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setYScale(qreal)

/*

*/
impl /*struct*/ QGraphicsScale {
  pub fn setYScale_0<RetType, T: QGraphicsScale_setYScale_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setYScale_0(self);
    // return 1;
  }
}
pub trait QGraphicsScale_setYScale_0<RetType> {
  fn setYScale_0(self , rsthis: & QGraphicsScale) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScale_setYScale_0<(/*void*/)> for (f64) {
  fn setYScale_0(self , rsthis: & QGraphicsScale) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN14QGraphicsScale9setYScaleEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicstransform.h:100
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal zScale() const

/*

*/
impl /*struct*/ QGraphicsScale {
  pub fn zScale_0<RetType, T: QGraphicsScale_zScale_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.zScale_0(self);
    // return 1;
  }
}
pub trait QGraphicsScale_zScale_0<RetType> {
  fn zScale_0(self , rsthis: & QGraphicsScale) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScale_zScale_0<f64> for () {
  fn zScale_0(self , rsthis: & QGraphicsScale) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QGraphicsScale6zScaleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicstransform.h:101
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setZScale(qreal)

/*

*/
impl /*struct*/ QGraphicsScale {
  pub fn setZScale_0<RetType, T: QGraphicsScale_setZScale_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setZScale_0(self);
    // return 1;
  }
}
pub trait QGraphicsScale_setZScale_0<RetType> {
  fn setZScale_0(self , rsthis: & QGraphicsScale) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScale_setZScale_0<(/*void*/)> for (f64) {
  fn setZScale_0(self , rsthis: & QGraphicsScale) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN14QGraphicsScale9setZScaleEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicstransform.h:103
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void applyTo(QMatrix4x4 *) const

/*
This pure virtual method has to be reimplemented in derived classes.

It applies this transformation to matrix.

See also QGraphicsItem::transform() and QMatrix4x4::toTransform().
*/
impl /*struct*/ QGraphicsScale {
  pub fn applyTo_0<RetType, T: QGraphicsScale_applyTo_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.applyTo_0(self);
    // return 1;
  }
}
pub trait QGraphicsScale_applyTo_0<RetType> {
  fn applyTo_0(self , rsthis: & QGraphicsScale) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScale_applyTo_0<(/*void*/)> for (usize) {
  fn applyTo_0(self , rsthis: & QGraphicsScale) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK14QGraphicsScale7applyToEP10QMatrix4x4", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicstransform.h:106
// index:0
// Public Visibility=Default Availability=Available
// [-2] void originChanged()

/*

*/
impl /*struct*/ QGraphicsScale {
  pub fn originChanged_0<RetType, T: QGraphicsScale_originChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.originChanged_0(self);
    // return 1;
  }
}
pub trait QGraphicsScale_originChanged_0<RetType> {
  fn originChanged_0(self , rsthis: & QGraphicsScale) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScale_originChanged_0<(/*void*/)> for () {
  fn originChanged_0(self , rsthis: & QGraphicsScale) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN14QGraphicsScale13originChangedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicstransform.h:107
// index:0
// Public Visibility=Default Availability=Available
// [-2] void xScaleChanged()

/*

*/
impl /*struct*/ QGraphicsScale {
  pub fn xScaleChanged_0<RetType, T: QGraphicsScale_xScaleChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.xScaleChanged_0(self);
    // return 1;
  }
}
pub trait QGraphicsScale_xScaleChanged_0<RetType> {
  fn xScaleChanged_0(self , rsthis: & QGraphicsScale) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScale_xScaleChanged_0<(/*void*/)> for () {
  fn xScaleChanged_0(self , rsthis: & QGraphicsScale) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN14QGraphicsScale13xScaleChangedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicstransform.h:108
// index:0
// Public Visibility=Default Availability=Available
// [-2] void yScaleChanged()

/*

*/
impl /*struct*/ QGraphicsScale {
  pub fn yScaleChanged_0<RetType, T: QGraphicsScale_yScaleChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.yScaleChanged_0(self);
    // return 1;
  }
}
pub trait QGraphicsScale_yScaleChanged_0<RetType> {
  fn yScaleChanged_0(self , rsthis: & QGraphicsScale) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScale_yScaleChanged_0<(/*void*/)> for () {
  fn yScaleChanged_0(self , rsthis: & QGraphicsScale) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN14QGraphicsScale13yScaleChangedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicstransform.h:109
// index:0
// Public Visibility=Default Availability=Available
// [-2] void zScaleChanged()

/*

*/
impl /*struct*/ QGraphicsScale {
  pub fn zScaleChanged_0<RetType, T: QGraphicsScale_zScaleChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.zScaleChanged_0(self);
    // return 1;
  }
}
pub trait QGraphicsScale_zScaleChanged_0<RetType> {
  fn zScaleChanged_0(self , rsthis: & QGraphicsScale) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScale_zScaleChanged_0<(/*void*/)> for () {
  fn zScaleChanged_0(self , rsthis: & QGraphicsScale) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN14QGraphicsScale13zScaleChangedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicstransform.h:110
// index:0
// Public Visibility=Default Availability=Available
// [-2] void scaleChanged()

/*

*/
impl /*struct*/ QGraphicsScale {
  pub fn scaleChanged_0<RetType, T: QGraphicsScale_scaleChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.scaleChanged_0(self);
    // return 1;
  }
}
pub trait QGraphicsScale_scaleChanged_0<RetType> {
  fn scaleChanged_0(self , rsthis: & QGraphicsScale) -> RetType;
}
impl<'a> /*trait*/ QGraphicsScale_scaleChanged_0<(/*void*/)> for () {
  fn scaleChanged_0(self , rsthis: & QGraphicsScale) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN14QGraphicsScale12scaleChangedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

//  body block end

//  keep block begin

//  keep block end
