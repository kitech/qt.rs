

// mod ::widgets::QGraphicsBlurEffect
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
// extern C begin: 10
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
// func (this *QGraphicsBlurEffect) InheritDraw(f func(painter *qtgui.QPainter/*777 QPainter **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "draw", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QGraphicsBlurEffect)=16
pub struct QGraphicsBlurEffect {
  qbase: QGraphicsEffect,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QGraphicsBlurEffect_ITF interface {
//    QGraphicsEffect_ITF
//    QGraphicsBlurEffect_PTR() *QGraphicsBlurEffect
//}
//func (ptr *QGraphicsBlurEffect) QGraphicsBlurEffect_PTR() *QGraphicsBlurEffect { return ptr }

impl /*struct*/ QGraphicsBlurEffect {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QGraphicsBlurEffect {
    return QGraphicsBlurEffect{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QGraphicsBlurEffect {
//  type Target = QGraphicsBlurEffectBASE;
//
//  fn deref(&self) -> &QGraphicsBlurEffectBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QGraphicsBlurEffectBASE> for QGraphicsBlurEffect {
//  fn as_ref(& self) -> & QGraphicsBlurEffectBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qgraphicseffect.h:157
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QGraphicsBlurEffect {
  pub fn metaObject_0<RetType, T: QGraphicsBlurEffect_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QGraphicsBlurEffect_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QGraphicsBlurEffect) -> RetType;
}
impl<'a> /*trait*/ QGraphicsBlurEffect_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QGraphicsBlurEffect) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QGraphicsBlurEffect10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicseffect.h:170
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QGraphicsBlurEffect(QObject *)

/*

*/
// QGraphicsBlurEffect(QObject *) ctx.fn_proto_cpp
impl /*struct*/ QGraphicsBlurEffect {
  pub fn QGraphicsBlurEffect_0<T: QGraphicsBlurEffect_QGraphicsBlurEffect_0>(value: T) -> QGraphicsBlurEffect {
    let rsthis = value.QGraphicsBlurEffect_0();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsBlurEffect_QGraphicsBlurEffect_0 {
  fn QGraphicsBlurEffect_0(self) -> QGraphicsBlurEffect;
}
// QGraphicsBlurEffect(QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QGraphicsBlurEffect_QGraphicsBlurEffect_0 for (usize) {
  fn QGraphicsBlurEffect_0(self) -> QGraphicsBlurEffect {
    // unsafe{_ZN19QGraphicsBlurEffectC2EP7QObject()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN19QGraphicsBlurEffectC2EP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QGraphicsBlurEffect{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicseffect.h:171
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QGraphicsBlurEffect()

/*

*/
pub fn DeleteQGraphicsBlurEffect(this :*mut QGraphicsBlurEffect) {
    // let rv = qtrt::InvokeQtFunc6("_ZN19QGraphicsBlurEffectD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qgraphicseffect.h:173
// index:0
// Public virtual Visibility=Default Availability=Available
// [32] QRectF boundingRectFor(const QRectF &) const

/*
Returns the effective bounding rectangle for this effect, given the provided rect in the device coordinates. When writing you own custom effect, you must call updateBoundingRect() whenever any parameters are changed that may cause this this function to return a different value.

See also sourceBoundingRect().
*/
impl /*struct*/ QGraphicsBlurEffect {
  pub fn boundingRectFor_0<RetType, T: QGraphicsBlurEffect_boundingRectFor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.boundingRectFor_0(self);
    // return 1;
  }
}
pub trait QGraphicsBlurEffect_boundingRectFor_0<RetType> {
  fn boundingRectFor_0(self , rsthis: & QGraphicsBlurEffect) -> RetType;
}
impl<'a> /*trait*/ QGraphicsBlurEffect_boundingRectFor_0<usize> for (usize) {
  fn boundingRectFor_0(self , rsthis: & QGraphicsBlurEffect) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QGraphicsBlurEffect15boundingRectForERK6QRectF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicseffect.h:174
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal blurRadius() const

/*

*/
impl /*struct*/ QGraphicsBlurEffect {
  pub fn blurRadius_0<RetType, T: QGraphicsBlurEffect_blurRadius_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.blurRadius_0(self);
    // return 1;
  }
}
pub trait QGraphicsBlurEffect_blurRadius_0<RetType> {
  fn blurRadius_0(self , rsthis: & QGraphicsBlurEffect) -> RetType;
}
impl<'a> /*trait*/ QGraphicsBlurEffect_blurRadius_0<f64> for () {
  fn blurRadius_0(self , rsthis: & QGraphicsBlurEffect) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QGraphicsBlurEffect10blurRadiusEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicseffect.h:175
// index:0
// Public Visibility=Default Availability=Available
// [4] QGraphicsBlurEffect::BlurHints blurHints() const

/*

*/
impl /*struct*/ QGraphicsBlurEffect {
  pub fn blurHints_0<RetType, T: QGraphicsBlurEffect_blurHints_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.blurHints_0(self);
    // return 1;
  }
}
pub trait QGraphicsBlurEffect_blurHints_0<RetType> {
  fn blurHints_0(self , rsthis: & QGraphicsBlurEffect) -> RetType;
}
impl<'a> /*trait*/ QGraphicsBlurEffect_blurHints_0<i32> for () {
  fn blurHints_0(self , rsthis: & QGraphicsBlurEffect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QGraphicsBlurEffect9blurHintsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicseffect.h:178
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setBlurRadius(qreal)

/*

*/
impl /*struct*/ QGraphicsBlurEffect {
  pub fn setBlurRadius_0<RetType, T: QGraphicsBlurEffect_setBlurRadius_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setBlurRadius_0(self);
    // return 1;
  }
}
pub trait QGraphicsBlurEffect_setBlurRadius_0<RetType> {
  fn setBlurRadius_0(self , rsthis: & QGraphicsBlurEffect) -> RetType;
}
impl<'a> /*trait*/ QGraphicsBlurEffect_setBlurRadius_0<(/*void*/)> for (f64) {
  fn setBlurRadius_0(self , rsthis: & QGraphicsBlurEffect) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN19QGraphicsBlurEffect13setBlurRadiusEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicseffect.h:179
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setBlurHints(QGraphicsBlurEffect::BlurHints)

/*

*/
impl /*struct*/ QGraphicsBlurEffect {
  pub fn setBlurHints_0<RetType, T: QGraphicsBlurEffect_setBlurHints_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setBlurHints_0(self);
    // return 1;
  }
}
pub trait QGraphicsBlurEffect_setBlurHints_0<RetType> {
  fn setBlurHints_0(self , rsthis: & QGraphicsBlurEffect) -> RetType;
}
impl<'a> /*trait*/ QGraphicsBlurEffect_setBlurHints_0<(/*void*/)> for (i32) {
  fn setBlurHints_0(self , rsthis: & QGraphicsBlurEffect) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN19QGraphicsBlurEffect12setBlurHintsE6QFlagsINS_8BlurHintEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicseffect.h:182
// index:0
// Public Visibility=Default Availability=Available
// [-2] void blurRadiusChanged(qreal)

/*

*/
impl /*struct*/ QGraphicsBlurEffect {
  pub fn blurRadiusChanged_0<RetType, T: QGraphicsBlurEffect_blurRadiusChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.blurRadiusChanged_0(self);
    // return 1;
  }
}
pub trait QGraphicsBlurEffect_blurRadiusChanged_0<RetType> {
  fn blurRadiusChanged_0(self , rsthis: & QGraphicsBlurEffect) -> RetType;
}
impl<'a> /*trait*/ QGraphicsBlurEffect_blurRadiusChanged_0<(/*void*/)> for (f64) {
  fn blurRadiusChanged_0(self , rsthis: & QGraphicsBlurEffect) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN19QGraphicsBlurEffect17blurRadiusChangedEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicseffect.h:183
// index:0
// Public Visibility=Default Availability=Available
// [-2] void blurHintsChanged(QGraphicsBlurEffect::BlurHints)

/*

*/
impl /*struct*/ QGraphicsBlurEffect {
  pub fn blurHintsChanged_0<RetType, T: QGraphicsBlurEffect_blurHintsChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.blurHintsChanged_0(self);
    // return 1;
  }
}
pub trait QGraphicsBlurEffect_blurHintsChanged_0<RetType> {
  fn blurHintsChanged_0(self , rsthis: & QGraphicsBlurEffect) -> RetType;
}
impl<'a> /*trait*/ QGraphicsBlurEffect_blurHintsChanged_0<(/*void*/)> for (i32) {
  fn blurHintsChanged_0(self , rsthis: & QGraphicsBlurEffect) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN19QGraphicsBlurEffect16blurHintsChangedE6QFlagsINS_8BlurHintEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicseffect.h:186
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
impl /*struct*/ QGraphicsBlurEffect {
  pub fn draw_0<RetType, T: QGraphicsBlurEffect_draw_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.draw_0(self);
    // return 1;
  }
}
pub trait QGraphicsBlurEffect_draw_0<RetType> {
  fn draw_0(self , rsthis: & QGraphicsBlurEffect) -> RetType;
}
impl<'a> /*trait*/ QGraphicsBlurEffect_draw_0<(/*void*/)> for (usize) {
  fn draw_0(self , rsthis: & QGraphicsBlurEffect) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN19QGraphicsBlurEffect4drawEP8QPainter", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}


/*


*/
pub type QGraphicsBlurEffect__BlurHint = i32;
// 
pub const QGraphicsBlurEffect__PerformanceHint :QGraphicsBlurEffect__BlurHint = 0;
// 
pub const QGraphicsBlurEffect__QualityHint :QGraphicsBlurEffect__BlurHint = 1;
// 
pub const QGraphicsBlurEffect__AnimationHint :QGraphicsBlurEffect__BlurHint = 2;
pub fn QGraphicsBlurEffect_BlurHintItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QGraphicsBlurEffect", val);
}
pub fn QGraphicsBlurEffect_BlurHintItemName_s(val: i32) ->String {
  //var nilthis *QGraphicsBlurEffect
  //return nilthis.BlurHintItemName(val);
  return QGraphicsBlurEffect_BlurHintItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
