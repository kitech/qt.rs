

// mod ::widgets::QGraphicsColorizeEffect
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
// extern C begin: 16
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
// func (this *QGraphicsColorizeEffect) InheritDraw(f func(painter *qtgui.QPainter/*777 QPainter **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "draw", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QGraphicsColorizeEffect)=16
pub struct QGraphicsColorizeEffect {
  qbase: QGraphicsEffect,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QGraphicsColorizeEffect_ITF interface {
//    QGraphicsEffect_ITF
//    QGraphicsColorizeEffect_PTR() *QGraphicsColorizeEffect
//}
//func (ptr *QGraphicsColorizeEffect) QGraphicsColorizeEffect_PTR() *QGraphicsColorizeEffect { return ptr }

impl /*struct*/ QGraphicsColorizeEffect {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QGraphicsColorizeEffect {
    return QGraphicsColorizeEffect{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QGraphicsColorizeEffect {
//  type Target = QGraphicsColorizeEffectBASE;
//
//  fn deref(&self) -> &QGraphicsColorizeEffectBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QGraphicsColorizeEffectBASE> for QGraphicsColorizeEffect {
//  fn as_ref(& self) -> & QGraphicsColorizeEffectBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qgraphicseffect.h:128
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QGraphicsColorizeEffect {
  pub fn metaObject_0<RetType, T: QGraphicsColorizeEffect_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QGraphicsColorizeEffect_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QGraphicsColorizeEffect) -> RetType;
}
impl<'a> /*trait*/ QGraphicsColorizeEffect_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QGraphicsColorizeEffect) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK23QGraphicsColorizeEffect10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicseffect.h:132
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QGraphicsColorizeEffect(QObject *)

/*

*/
// QGraphicsColorizeEffect(QObject *) ctx.fn_proto_cpp
impl /*struct*/ QGraphicsColorizeEffect {
  pub fn QGraphicsColorizeEffect_0<T: QGraphicsColorizeEffect_QGraphicsColorizeEffect_0>(value: T) -> QGraphicsColorizeEffect {
    let rsthis = value.QGraphicsColorizeEffect_0();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsColorizeEffect_QGraphicsColorizeEffect_0 {
  fn QGraphicsColorizeEffect_0(self) -> QGraphicsColorizeEffect;
}
// QGraphicsColorizeEffect(QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QGraphicsColorizeEffect_QGraphicsColorizeEffect_0 for (usize) {
  fn QGraphicsColorizeEffect_0(self) -> QGraphicsColorizeEffect {
    // unsafe{_ZN23QGraphicsColorizeEffectC2EP7QObject()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN23QGraphicsColorizeEffectC2EP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QGraphicsColorizeEffect{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicseffect.h:133
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QGraphicsColorizeEffect()

/*

*/
pub fn DeleteQGraphicsColorizeEffect(this :*mut QGraphicsColorizeEffect) {
    // let rv = qtrt::InvokeQtFunc6("_ZN23QGraphicsColorizeEffectD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qgraphicseffect.h:135
// index:0
// Public Visibility=Default Availability=Available
// [16] QColor color() const

/*

*/
impl /*struct*/ QGraphicsColorizeEffect {
  pub fn color_0<RetType, T: QGraphicsColorizeEffect_color_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.color_0(self);
    // return 1;
  }
}
pub trait QGraphicsColorizeEffect_color_0<RetType> {
  fn color_0(self , rsthis: & QGraphicsColorizeEffect) -> RetType;
}
impl<'a> /*trait*/ QGraphicsColorizeEffect_color_0<usize> for () {
  fn color_0(self , rsthis: & QGraphicsColorizeEffect) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK23QGraphicsColorizeEffect5colorEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicseffect.h:136
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal strength() const

/*

*/
impl /*struct*/ QGraphicsColorizeEffect {
  pub fn strength_0<RetType, T: QGraphicsColorizeEffect_strength_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.strength_0(self);
    // return 1;
  }
}
pub trait QGraphicsColorizeEffect_strength_0<RetType> {
  fn strength_0(self , rsthis: & QGraphicsColorizeEffect) -> RetType;
}
impl<'a> /*trait*/ QGraphicsColorizeEffect_strength_0<f64> for () {
  fn strength_0(self , rsthis: & QGraphicsColorizeEffect) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK23QGraphicsColorizeEffect8strengthEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicseffect.h:139
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setColor(const QColor &)

/*

*/
impl /*struct*/ QGraphicsColorizeEffect {
  pub fn setColor_0<RetType, T: QGraphicsColorizeEffect_setColor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setColor_0(self);
    // return 1;
  }
}
pub trait QGraphicsColorizeEffect_setColor_0<RetType> {
  fn setColor_0(self , rsthis: & QGraphicsColorizeEffect) -> RetType;
}
impl<'a> /*trait*/ QGraphicsColorizeEffect_setColor_0<(/*void*/)> for (usize) {
  fn setColor_0(self , rsthis: & QGraphicsColorizeEffect) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN23QGraphicsColorizeEffect8setColorERK6QColor", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicseffect.h:140
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setStrength(qreal)

/*

*/
impl /*struct*/ QGraphicsColorizeEffect {
  pub fn setStrength_0<RetType, T: QGraphicsColorizeEffect_setStrength_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setStrength_0(self);
    // return 1;
  }
}
pub trait QGraphicsColorizeEffect_setStrength_0<RetType> {
  fn setStrength_0(self , rsthis: & QGraphicsColorizeEffect) -> RetType;
}
impl<'a> /*trait*/ QGraphicsColorizeEffect_setStrength_0<(/*void*/)> for (f64) {
  fn setStrength_0(self , rsthis: & QGraphicsColorizeEffect) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN23QGraphicsColorizeEffect11setStrengthEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicseffect.h:143
// index:0
// Public Visibility=Default Availability=Available
// [-2] void colorChanged(const QColor &)

/*

*/
impl /*struct*/ QGraphicsColorizeEffect {
  pub fn colorChanged_0<RetType, T: QGraphicsColorizeEffect_colorChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.colorChanged_0(self);
    // return 1;
  }
}
pub trait QGraphicsColorizeEffect_colorChanged_0<RetType> {
  fn colorChanged_0(self , rsthis: & QGraphicsColorizeEffect) -> RetType;
}
impl<'a> /*trait*/ QGraphicsColorizeEffect_colorChanged_0<(/*void*/)> for (usize) {
  fn colorChanged_0(self , rsthis: & QGraphicsColorizeEffect) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN23QGraphicsColorizeEffect12colorChangedERK6QColor", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicseffect.h:144
// index:0
// Public Visibility=Default Availability=Available
// [-2] void strengthChanged(qreal)

/*

*/
impl /*struct*/ QGraphicsColorizeEffect {
  pub fn strengthChanged_0<RetType, T: QGraphicsColorizeEffect_strengthChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.strengthChanged_0(self);
    // return 1;
  }
}
pub trait QGraphicsColorizeEffect_strengthChanged_0<RetType> {
  fn strengthChanged_0(self , rsthis: & QGraphicsColorizeEffect) -> RetType;
}
impl<'a> /*trait*/ QGraphicsColorizeEffect_strengthChanged_0<(/*void*/)> for (f64) {
  fn strengthChanged_0(self , rsthis: & QGraphicsColorizeEffect) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN23QGraphicsColorizeEffect15strengthChangedEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicseffect.h:147
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
impl /*struct*/ QGraphicsColorizeEffect {
  pub fn draw_0<RetType, T: QGraphicsColorizeEffect_draw_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.draw_0(self);
    // return 1;
  }
}
pub trait QGraphicsColorizeEffect_draw_0<RetType> {
  fn draw_0(self , rsthis: & QGraphicsColorizeEffect) -> RetType;
}
impl<'a> /*trait*/ QGraphicsColorizeEffect_draw_0<(/*void*/)> for (usize) {
  fn draw_0(self , rsthis: & QGraphicsColorizeEffect) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN23QGraphicsColorizeEffect4drawEP8QPainter", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

//  body block end

//  keep block begin

//  keep block end
