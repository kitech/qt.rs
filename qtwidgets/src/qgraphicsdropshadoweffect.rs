

// mod ::widgets::QGraphicsDropShadowEffect
// package qtwidgets
// /usr/include/qt/QtWidgets/qgraphicseffect.h
// #include <qgraphicseffect.h>
// #include <QtWidgets>

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

// void draw(QPainter *)
// func (this *QGraphicsDropShadowEffect) InheritDraw(f func(painter *qtgui.QPainter/*777 QPainter **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "draw", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QGraphicsDropShadowEffect)=16
pub struct QGraphicsDropShadowEffect {
  qbase: QGraphicsEffect,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QGraphicsDropShadowEffect_ITF interface {
//    QGraphicsEffect_ITF
//    QGraphicsDropShadowEffect_PTR() *QGraphicsDropShadowEffect
//}
//func (ptr *QGraphicsDropShadowEffect) QGraphicsDropShadowEffect_PTR() *QGraphicsDropShadowEffect { return ptr }

impl /*struct*/ QGraphicsDropShadowEffect {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QGraphicsDropShadowEffect {
    return QGraphicsDropShadowEffect{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QGraphicsDropShadowEffect {
//  type Target = QGraphicsDropShadowEffectBASE;
//
//  fn deref(&self) -> &QGraphicsDropShadowEffectBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QGraphicsDropShadowEffectBASE> for QGraphicsDropShadowEffect {
//  fn as_ref(& self) -> & QGraphicsDropShadowEffectBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qgraphicseffect.h:198
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QGraphicsDropShadowEffect {
  pub fn metaObject_0<RetType, T: QGraphicsDropShadowEffect_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QGraphicsDropShadowEffect_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QGraphicsDropShadowEffect) -> RetType;
}
impl<'a> /*trait*/ QGraphicsDropShadowEffect_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QGraphicsDropShadowEffect) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK25QGraphicsDropShadowEffect10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicseffect.h:205
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QGraphicsDropShadowEffect(QObject *)

/*

*/
// QGraphicsDropShadowEffect(QObject *) ctx.fn_proto_cpp
impl /*struct*/ QGraphicsDropShadowEffect {
  pub fn QGraphicsDropShadowEffect_0<T: QGraphicsDropShadowEffect_QGraphicsDropShadowEffect_0>(value: T) -> QGraphicsDropShadowEffect {
    let rsthis = value.QGraphicsDropShadowEffect_0();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsDropShadowEffect_QGraphicsDropShadowEffect_0 {
  fn QGraphicsDropShadowEffect_0(self) -> QGraphicsDropShadowEffect;
}
// QGraphicsDropShadowEffect(QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QGraphicsDropShadowEffect_QGraphicsDropShadowEffect_0 for (usize) {
  fn QGraphicsDropShadowEffect_0(self) -> QGraphicsDropShadowEffect {
    // unsafe{_ZN25QGraphicsDropShadowEffectC2EP7QObject()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN25QGraphicsDropShadowEffectC2EP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QGraphicsDropShadowEffect{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicseffect.h:206
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QGraphicsDropShadowEffect()

/*

*/
pub fn DeleteQGraphicsDropShadowEffect(this :*mut QGraphicsDropShadowEffect) {
    // let rv = qtrt::InvokeQtFunc6("_ZN25QGraphicsDropShadowEffectD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qgraphicseffect.h:208
// index:0
// Public virtual Visibility=Default Availability=Available
// [32] QRectF boundingRectFor(const QRectF &) const

/*
Returns the effective bounding rectangle for this effect, given the provided rect in the device coordinates. When writing you own custom effect, you must call updateBoundingRect() whenever any parameters are changed that may cause this this function to return a different value.

See also sourceBoundingRect().
*/
impl /*struct*/ QGraphicsDropShadowEffect {
  pub fn boundingRectFor_0<RetType, T: QGraphicsDropShadowEffect_boundingRectFor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.boundingRectFor_0(self);
    // return 1;
  }
}
pub trait QGraphicsDropShadowEffect_boundingRectFor_0<RetType> {
  fn boundingRectFor_0(self , rsthis: & QGraphicsDropShadowEffect) -> RetType;
}
impl<'a> /*trait*/ QGraphicsDropShadowEffect_boundingRectFor_0<usize> for (usize) {
  fn boundingRectFor_0(self , rsthis: & QGraphicsDropShadowEffect) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK25QGraphicsDropShadowEffect15boundingRectForERK6QRectF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicseffect.h:209
// index:0
// Public Visibility=Default Availability=Available
// [16] QPointF offset() const

/*

*/
impl /*struct*/ QGraphicsDropShadowEffect {
  pub fn offset_0<RetType, T: QGraphicsDropShadowEffect_offset_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.offset_0(self);
    // return 1;
  }
}
pub trait QGraphicsDropShadowEffect_offset_0<RetType> {
  fn offset_0(self , rsthis: & QGraphicsDropShadowEffect) -> RetType;
}
impl<'a> /*trait*/ QGraphicsDropShadowEffect_offset_0<usize> for () {
  fn offset_0(self , rsthis: & QGraphicsDropShadowEffect) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK25QGraphicsDropShadowEffect6offsetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicseffect.h:211
// index:0
// Public inline Visibility=Default Availability=Available
// [8] qreal xOffset() const

/*

*/
impl /*struct*/ QGraphicsDropShadowEffect {
  pub fn xOffset_0<RetType, T: QGraphicsDropShadowEffect_xOffset_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.xOffset_0(self);
    // return 1;
  }
}
pub trait QGraphicsDropShadowEffect_xOffset_0<RetType> {
  fn xOffset_0(self , rsthis: & QGraphicsDropShadowEffect) -> RetType;
}
impl<'a> /*trait*/ QGraphicsDropShadowEffect_xOffset_0<f64> for () {
  fn xOffset_0(self , rsthis: & QGraphicsDropShadowEffect) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK25QGraphicsDropShadowEffect7xOffsetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicseffect.h:214
// index:0
// Public inline Visibility=Default Availability=Available
// [8] qreal yOffset() const

/*

*/
impl /*struct*/ QGraphicsDropShadowEffect {
  pub fn yOffset_0<RetType, T: QGraphicsDropShadowEffect_yOffset_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.yOffset_0(self);
    // return 1;
  }
}
pub trait QGraphicsDropShadowEffect_yOffset_0<RetType> {
  fn yOffset_0(self , rsthis: & QGraphicsDropShadowEffect) -> RetType;
}
impl<'a> /*trait*/ QGraphicsDropShadowEffect_yOffset_0<f64> for () {
  fn yOffset_0(self , rsthis: & QGraphicsDropShadowEffect) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK25QGraphicsDropShadowEffect7yOffsetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicseffect.h:217
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal blurRadius() const

/*

*/
impl /*struct*/ QGraphicsDropShadowEffect {
  pub fn blurRadius_0<RetType, T: QGraphicsDropShadowEffect_blurRadius_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.blurRadius_0(self);
    // return 1;
  }
}
pub trait QGraphicsDropShadowEffect_blurRadius_0<RetType> {
  fn blurRadius_0(self , rsthis: & QGraphicsDropShadowEffect) -> RetType;
}
impl<'a> /*trait*/ QGraphicsDropShadowEffect_blurRadius_0<f64> for () {
  fn blurRadius_0(self , rsthis: & QGraphicsDropShadowEffect) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK25QGraphicsDropShadowEffect10blurRadiusEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicseffect.h:218
// index:0
// Public Visibility=Default Availability=Available
// [16] QColor color() const

/*

*/
impl /*struct*/ QGraphicsDropShadowEffect {
  pub fn color_0<RetType, T: QGraphicsDropShadowEffect_color_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.color_0(self);
    // return 1;
  }
}
pub trait QGraphicsDropShadowEffect_color_0<RetType> {
  fn color_0(self , rsthis: & QGraphicsDropShadowEffect) -> RetType;
}
impl<'a> /*trait*/ QGraphicsDropShadowEffect_color_0<usize> for () {
  fn color_0(self , rsthis: & QGraphicsDropShadowEffect) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK25QGraphicsDropShadowEffect5colorEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicseffect.h:221
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setOffset(const QPointF &)

/*

*/
impl /*struct*/ QGraphicsDropShadowEffect {
  pub fn setOffset_0<RetType, T: QGraphicsDropShadowEffect_setOffset_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setOffset_0(self);
    // return 1;
  }
}
pub trait QGraphicsDropShadowEffect_setOffset_0<RetType> {
  fn setOffset_0(self , rsthis: & QGraphicsDropShadowEffect) -> RetType;
}
impl<'a> /*trait*/ QGraphicsDropShadowEffect_setOffset_0<(/*void*/)> for (usize) {
  fn setOffset_0(self , rsthis: & QGraphicsDropShadowEffect) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN25QGraphicsDropShadowEffect9setOffsetERK7QPointF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicseffect.h:223
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void setOffset(qreal, qreal)

/*

*/
impl /*struct*/ QGraphicsDropShadowEffect {
  pub fn setOffset_1<RetType, T: QGraphicsDropShadowEffect_setOffset_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setOffset_1(self);
    // return 1;
  }
}
pub trait QGraphicsDropShadowEffect_setOffset_1<RetType> {
  fn setOffset_1(self , rsthis: & QGraphicsDropShadowEffect) -> RetType;
}
impl<'a> /*trait*/ QGraphicsDropShadowEffect_setOffset_1<(/*void*/)> for (f64,f64) {
  fn setOffset_1(self , rsthis: & QGraphicsDropShadowEffect) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN25QGraphicsDropShadowEffect9setOffsetEdd", 2,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicseffect.h:226
// index:2
// Public inline Visibility=Default Availability=Available
// [-2] void setOffset(qreal)

/*

*/
impl /*struct*/ QGraphicsDropShadowEffect {
  pub fn setOffset_2<RetType, T: QGraphicsDropShadowEffect_setOffset_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setOffset_2(self);
    // return 1;
  }
}
pub trait QGraphicsDropShadowEffect_setOffset_2<RetType> {
  fn setOffset_2(self , rsthis: & QGraphicsDropShadowEffect) -> RetType;
}
impl<'a> /*trait*/ QGraphicsDropShadowEffect_setOffset_2<(/*void*/)> for (f64) {
  fn setOffset_2(self , rsthis: & QGraphicsDropShadowEffect) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN25QGraphicsDropShadowEffect9setOffsetEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicseffect.h:229
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setXOffset(qreal)

/*

*/
impl /*struct*/ QGraphicsDropShadowEffect {
  pub fn setXOffset_0<RetType, T: QGraphicsDropShadowEffect_setXOffset_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setXOffset_0(self);
    // return 1;
  }
}
pub trait QGraphicsDropShadowEffect_setXOffset_0<RetType> {
  fn setXOffset_0(self , rsthis: & QGraphicsDropShadowEffect) -> RetType;
}
impl<'a> /*trait*/ QGraphicsDropShadowEffect_setXOffset_0<(/*void*/)> for (f64) {
  fn setXOffset_0(self , rsthis: & QGraphicsDropShadowEffect) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN25QGraphicsDropShadowEffect10setXOffsetEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicseffect.h:232
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setYOffset(qreal)

/*

*/
impl /*struct*/ QGraphicsDropShadowEffect {
  pub fn setYOffset_0<RetType, T: QGraphicsDropShadowEffect_setYOffset_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setYOffset_0(self);
    // return 1;
  }
}
pub trait QGraphicsDropShadowEffect_setYOffset_0<RetType> {
  fn setYOffset_0(self , rsthis: & QGraphicsDropShadowEffect) -> RetType;
}
impl<'a> /*trait*/ QGraphicsDropShadowEffect_setYOffset_0<(/*void*/)> for (f64) {
  fn setYOffset_0(self , rsthis: & QGraphicsDropShadowEffect) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN25QGraphicsDropShadowEffect10setYOffsetEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicseffect.h:235
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setBlurRadius(qreal)

/*

*/
impl /*struct*/ QGraphicsDropShadowEffect {
  pub fn setBlurRadius_0<RetType, T: QGraphicsDropShadowEffect_setBlurRadius_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setBlurRadius_0(self);
    // return 1;
  }
}
pub trait QGraphicsDropShadowEffect_setBlurRadius_0<RetType> {
  fn setBlurRadius_0(self , rsthis: & QGraphicsDropShadowEffect) -> RetType;
}
impl<'a> /*trait*/ QGraphicsDropShadowEffect_setBlurRadius_0<(/*void*/)> for (f64) {
  fn setBlurRadius_0(self , rsthis: & QGraphicsDropShadowEffect) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN25QGraphicsDropShadowEffect13setBlurRadiusEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicseffect.h:236
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setColor(const QColor &)

/*

*/
impl /*struct*/ QGraphicsDropShadowEffect {
  pub fn setColor_0<RetType, T: QGraphicsDropShadowEffect_setColor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setColor_0(self);
    // return 1;
  }
}
pub trait QGraphicsDropShadowEffect_setColor_0<RetType> {
  fn setColor_0(self , rsthis: & QGraphicsDropShadowEffect) -> RetType;
}
impl<'a> /*trait*/ QGraphicsDropShadowEffect_setColor_0<(/*void*/)> for (usize) {
  fn setColor_0(self , rsthis: & QGraphicsDropShadowEffect) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN25QGraphicsDropShadowEffect8setColorERK6QColor", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicseffect.h:239
// index:0
// Public Visibility=Default Availability=Available
// [-2] void offsetChanged(const QPointF &)

/*

*/
impl /*struct*/ QGraphicsDropShadowEffect {
  pub fn offsetChanged_0<RetType, T: QGraphicsDropShadowEffect_offsetChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.offsetChanged_0(self);
    // return 1;
  }
}
pub trait QGraphicsDropShadowEffect_offsetChanged_0<RetType> {
  fn offsetChanged_0(self , rsthis: & QGraphicsDropShadowEffect) -> RetType;
}
impl<'a> /*trait*/ QGraphicsDropShadowEffect_offsetChanged_0<(/*void*/)> for (usize) {
  fn offsetChanged_0(self , rsthis: & QGraphicsDropShadowEffect) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN25QGraphicsDropShadowEffect13offsetChangedERK7QPointF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicseffect.h:240
// index:0
// Public Visibility=Default Availability=Available
// [-2] void blurRadiusChanged(qreal)

/*

*/
impl /*struct*/ QGraphicsDropShadowEffect {
  pub fn blurRadiusChanged_0<RetType, T: QGraphicsDropShadowEffect_blurRadiusChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.blurRadiusChanged_0(self);
    // return 1;
  }
}
pub trait QGraphicsDropShadowEffect_blurRadiusChanged_0<RetType> {
  fn blurRadiusChanged_0(self , rsthis: & QGraphicsDropShadowEffect) -> RetType;
}
impl<'a> /*trait*/ QGraphicsDropShadowEffect_blurRadiusChanged_0<(/*void*/)> for (f64) {
  fn blurRadiusChanged_0(self , rsthis: & QGraphicsDropShadowEffect) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN25QGraphicsDropShadowEffect17blurRadiusChangedEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicseffect.h:241
// index:0
// Public Visibility=Default Availability=Available
// [-2] void colorChanged(const QColor &)

/*

*/
impl /*struct*/ QGraphicsDropShadowEffect {
  pub fn colorChanged_0<RetType, T: QGraphicsDropShadowEffect_colorChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.colorChanged_0(self);
    // return 1;
  }
}
pub trait QGraphicsDropShadowEffect_colorChanged_0<RetType> {
  fn colorChanged_0(self , rsthis: & QGraphicsDropShadowEffect) -> RetType;
}
impl<'a> /*trait*/ QGraphicsDropShadowEffect_colorChanged_0<(/*void*/)> for (usize) {
  fn colorChanged_0(self , rsthis: & QGraphicsDropShadowEffect) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN25QGraphicsDropShadowEffect12colorChangedERK6QColor", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicseffect.h:244
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void draw(QPainter *)

/*
This pure virtual function draws the effect and is called whenever the source needs to be drawn.

Reimplement this function in a QGraphicsEffect subclass to provide the effect's drawing implementation, using painter.

For example:


  MyGraphicsEffect::draw(QPainter *painter)
  {
      ...
      QPoint offset;
      if (sourceIsPixmap()) {
          // No point in drawing in device coordinates (pixmap will be scaled anyways).
          const QPixmap pixmap = sourcePixmap(Qt::LogicalCoordinates, &offset);
          ...
          painter->drawPixmap(offset, pixmap);
      } else {
          // Draw pixmap in device coordinates to avoid pixmap scaling;
          const QPixmap pixmap = sourcePixmap(Qt::DeviceCoordinates, &offset);
          painter->setWorldTransform(QTransform());
          ...
          painter->drawPixmap(offset, pixmap);
      }
      ...
  }



This function should not be called explicitly by the user, since it is meant for reimplementation purposes only.
*/
impl /*struct*/ QGraphicsDropShadowEffect {
  pub fn draw_0<RetType, T: QGraphicsDropShadowEffect_draw_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.draw_0(self);
    // return 1;
  }
}
pub trait QGraphicsDropShadowEffect_draw_0<RetType> {
  fn draw_0(self , rsthis: & QGraphicsDropShadowEffect) -> RetType;
}
impl<'a> /*trait*/ QGraphicsDropShadowEffect_draw_0<(/*void*/)> for (usize) {
  fn draw_0(self , rsthis: & QGraphicsDropShadowEffect) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN25QGraphicsDropShadowEffect4drawEP8QPainter", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

//  body block end

//  keep block begin

//  keep block end
