

// mod ::widgets::QGraphicsOpacityEffect
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
// import "github.com/kitech/qt.go/qtgui"
use qtgui::*; // super::super::%!s(MISSING)::*;
use std::default::Default;
use std::ops::Deref;
use qtrt; // super::super::qtrt;
use super::*;
//  ext block end

//  body block begin

// void draw(QPainter *)
// func (this *QGraphicsOpacityEffect) InheritDraw(f func(painter *qtgui.QPainter/*777 QPainter **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "draw", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QGraphicsOpacityEffect)=16
pub struct QGraphicsOpacityEffect {
  qbase: QGraphicsEffect,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QGraphicsOpacityEffect_ITF interface {
//    QGraphicsEffect_ITF
//    QGraphicsOpacityEffect_PTR() *QGraphicsOpacityEffect
//}
//func (ptr *QGraphicsOpacityEffect) QGraphicsOpacityEffect_PTR() *QGraphicsOpacityEffect { return ptr }

impl /*struct*/ QGraphicsOpacityEffect {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QGraphicsOpacityEffect {
    return QGraphicsOpacityEffect{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QGraphicsOpacityEffect {
//  type Target = QGraphicsOpacityEffectBASE;
//
//  fn deref(&self) -> &QGraphicsOpacityEffectBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QGraphicsOpacityEffectBASE> for QGraphicsOpacityEffect {
//  fn as_ref(& self) -> & QGraphicsOpacityEffectBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qgraphicseffect.h:254
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QGraphicsOpacityEffect {
  pub fn metaObject_0<RetType, T: QGraphicsOpacityEffect_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QGraphicsOpacityEffect_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QGraphicsOpacityEffect) -> RetType;
}
impl<'a> /*trait*/ QGraphicsOpacityEffect_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QGraphicsOpacityEffect) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK22QGraphicsOpacityEffect10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicseffect.h:258
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QGraphicsOpacityEffect(QObject *)

/*

*/
// QGraphicsOpacityEffect(QObject *) ctx.fn_proto_cpp
impl /*struct*/ QGraphicsOpacityEffect {
  pub fn QGraphicsOpacityEffect_0<T: QGraphicsOpacityEffect_QGraphicsOpacityEffect_0>(value: T) -> QGraphicsOpacityEffect {
    let rsthis = value.QGraphicsOpacityEffect_0();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsOpacityEffect_QGraphicsOpacityEffect_0 {
  fn QGraphicsOpacityEffect_0(self) -> QGraphicsOpacityEffect;
}
// QGraphicsOpacityEffect(QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QGraphicsOpacityEffect_QGraphicsOpacityEffect_0 for (usize) {
  fn QGraphicsOpacityEffect_0(self) -> QGraphicsOpacityEffect {
    // unsafe{_ZN22QGraphicsOpacityEffectC2EP7QObject()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN22QGraphicsOpacityEffectC2EP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QGraphicsOpacityEffect{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicseffect.h:259
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QGraphicsOpacityEffect()

/*

*/
pub fn DeleteQGraphicsOpacityEffect(this :*mut QGraphicsOpacityEffect) {
    // let rv = qtrt::InvokeQtFunc6("_ZN22QGraphicsOpacityEffectD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qgraphicseffect.h:261
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal opacity() const

/*

*/
impl /*struct*/ QGraphicsOpacityEffect {
  pub fn opacity_0<RetType, T: QGraphicsOpacityEffect_opacity_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.opacity_0(self);
    // return 1;
  }
}
pub trait QGraphicsOpacityEffect_opacity_0<RetType> {
  fn opacity_0(self , rsthis: & QGraphicsOpacityEffect) -> RetType;
}
impl<'a> /*trait*/ QGraphicsOpacityEffect_opacity_0<f64> for () {
  fn opacity_0(self , rsthis: & QGraphicsOpacityEffect) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK22QGraphicsOpacityEffect7opacityEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicseffect.h:262
// index:0
// Public Visibility=Default Availability=Available
// [8] QBrush opacityMask() const

/*

*/
impl /*struct*/ QGraphicsOpacityEffect {
  pub fn opacityMask_0<RetType, T: QGraphicsOpacityEffect_opacityMask_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.opacityMask_0(self);
    // return 1;
  }
}
pub trait QGraphicsOpacityEffect_opacityMask_0<RetType> {
  fn opacityMask_0(self , rsthis: & QGraphicsOpacityEffect) -> RetType;
}
impl<'a> /*trait*/ QGraphicsOpacityEffect_opacityMask_0<usize> for () {
  fn opacityMask_0(self , rsthis: & QGraphicsOpacityEffect) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK22QGraphicsOpacityEffect11opacityMaskEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicseffect.h:265
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setOpacity(qreal)

/*

*/
impl /*struct*/ QGraphicsOpacityEffect {
  pub fn setOpacity_0<RetType, T: QGraphicsOpacityEffect_setOpacity_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setOpacity_0(self);
    // return 1;
  }
}
pub trait QGraphicsOpacityEffect_setOpacity_0<RetType> {
  fn setOpacity_0(self , rsthis: & QGraphicsOpacityEffect) -> RetType;
}
impl<'a> /*trait*/ QGraphicsOpacityEffect_setOpacity_0<(/*void*/)> for (f64) {
  fn setOpacity_0(self , rsthis: & QGraphicsOpacityEffect) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN22QGraphicsOpacityEffect10setOpacityEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicseffect.h:266
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setOpacityMask(const QBrush &)

/*

*/
impl /*struct*/ QGraphicsOpacityEffect {
  pub fn setOpacityMask_0<RetType, T: QGraphicsOpacityEffect_setOpacityMask_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setOpacityMask_0(self);
    // return 1;
  }
}
pub trait QGraphicsOpacityEffect_setOpacityMask_0<RetType> {
  fn setOpacityMask_0(self , rsthis: & QGraphicsOpacityEffect) -> RetType;
}
impl<'a> /*trait*/ QGraphicsOpacityEffect_setOpacityMask_0<(/*void*/)> for (usize) {
  fn setOpacityMask_0(self , rsthis: & QGraphicsOpacityEffect) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN22QGraphicsOpacityEffect14setOpacityMaskERK6QBrush", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicseffect.h:269
// index:0
// Public Visibility=Default Availability=Available
// [-2] void opacityChanged(qreal)

/*

*/
impl /*struct*/ QGraphicsOpacityEffect {
  pub fn opacityChanged_0<RetType, T: QGraphicsOpacityEffect_opacityChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.opacityChanged_0(self);
    // return 1;
  }
}
pub trait QGraphicsOpacityEffect_opacityChanged_0<RetType> {
  fn opacityChanged_0(self , rsthis: & QGraphicsOpacityEffect) -> RetType;
}
impl<'a> /*trait*/ QGraphicsOpacityEffect_opacityChanged_0<(/*void*/)> for (f64) {
  fn opacityChanged_0(self , rsthis: & QGraphicsOpacityEffect) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN22QGraphicsOpacityEffect14opacityChangedEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicseffect.h:270
// index:0
// Public Visibility=Default Availability=Available
// [-2] void opacityMaskChanged(const QBrush &)

/*

*/
impl /*struct*/ QGraphicsOpacityEffect {
  pub fn opacityMaskChanged_0<RetType, T: QGraphicsOpacityEffect_opacityMaskChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.opacityMaskChanged_0(self);
    // return 1;
  }
}
pub trait QGraphicsOpacityEffect_opacityMaskChanged_0<RetType> {
  fn opacityMaskChanged_0(self , rsthis: & QGraphicsOpacityEffect) -> RetType;
}
impl<'a> /*trait*/ QGraphicsOpacityEffect_opacityMaskChanged_0<(/*void*/)> for (usize) {
  fn opacityMaskChanged_0(self , rsthis: & QGraphicsOpacityEffect) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN22QGraphicsOpacityEffect18opacityMaskChangedERK6QBrush", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicseffect.h:273
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
impl /*struct*/ QGraphicsOpacityEffect {
  pub fn draw_0<RetType, T: QGraphicsOpacityEffect_draw_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.draw_0(self);
    // return 1;
  }
}
pub trait QGraphicsOpacityEffect_draw_0<RetType> {
  fn draw_0(self , rsthis: & QGraphicsOpacityEffect) -> RetType;
}
impl<'a> /*trait*/ QGraphicsOpacityEffect_draw_0<(/*void*/)> for (usize) {
  fn draw_0(self , rsthis: & QGraphicsOpacityEffect) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN22QGraphicsOpacityEffect4drawEP8QPainter", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

//  body block end

//  keep block begin

//  keep block end
