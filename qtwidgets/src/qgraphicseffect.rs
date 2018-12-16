

// mod ::widgets::QGraphicsEffect
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

// void draw(QPainter *)
// func (this *QGraphicsEffect) InheritDraw(f func(painter *qtgui.QPainter/*777 QPainter **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "draw", f)
// }

// void sourceChanged(QGraphicsEffect::ChangeFlags)
// func (this *QGraphicsEffect) InheritSourceChanged(f func(flags int) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "sourceChanged", f)
// }

// void updateBoundingRect()
// func (this *QGraphicsEffect) InheritUpdateBoundingRect(f func() /*void*/) {
//  qtrt.SetAllInheritCallback(this, "updateBoundingRect", f)
// }

// bool sourceIsPixmap()
// func (this *QGraphicsEffect) InheritSourceIsPixmap(f func() bool) {
//  qtrt.SetAllInheritCallback(this, "sourceIsPixmap", f)
// }

// QRectF sourceBoundingRect(Qt::CoordinateSystem)
// func (this *QGraphicsEffect) InheritSourceBoundingRect(f func(system int) unsafe.Pointer) {
//  qtrt.SetAllInheritCallback(this, "sourceBoundingRect", f)
// }

// void drawSource(QPainter *)
// func (this *QGraphicsEffect) InheritDrawSource(f func(painter *qtgui.QPainter/*777 QPainter **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "drawSource", f)
// }

// QPixmap sourcePixmap(Qt::CoordinateSystem, QPoint *, QGraphicsEffect::PixmapPadMode)
// func (this *QGraphicsEffect) InheritSourcePixmap(f func(system int, offset *qtcore.QPoint/*777 QPoint **/, mode int) unsafe.Pointer) {
//  qtrt.SetAllInheritCallback(this, "sourcePixmap", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QGraphicsEffect)=16
pub struct QGraphicsEffect {
  qbase: QObject,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QGraphicsEffect_ITF interface {
//    qtcore.QObject_ITF
//    QGraphicsEffect_PTR() *QGraphicsEffect
//}
//func (ptr *QGraphicsEffect) QGraphicsEffect_PTR() *QGraphicsEffect { return ptr }

impl /*struct*/ QGraphicsEffect {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QGraphicsEffect {
    return QGraphicsEffect{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QGraphicsEffect {
//  type Target = QGraphicsEffectBASE;
//
//  fn deref(&self) -> &QGraphicsEffectBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QGraphicsEffectBASE> for QGraphicsEffect {
//  fn as_ref(& self) -> & QGraphicsEffectBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qgraphicseffect.h:64
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QGraphicsEffect {
  pub fn metaObject_0<RetType, T: QGraphicsEffect_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QGraphicsEffect_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QGraphicsEffect) -> RetType;
}
impl<'a> /*trait*/ QGraphicsEffect_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QGraphicsEffect) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QGraphicsEffect10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicseffect.h:82
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QGraphicsEffect(QObject *)

/*
Constructs a new QGraphicsEffect instance having the specified parent.
*/
// QGraphicsEffect(QObject *) ctx.fn_proto_cpp
impl /*struct*/ QGraphicsEffect {
  pub fn QGraphicsEffect_0<T: QGraphicsEffect_QGraphicsEffect_0>(value: T) -> QGraphicsEffect {
    let rsthis = value.QGraphicsEffect_0();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsEffect_QGraphicsEffect_0 {
  fn QGraphicsEffect_0(self) -> QGraphicsEffect;
}
// QGraphicsEffect(QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QGraphicsEffect_QGraphicsEffect_0 for (usize) {
  fn QGraphicsEffect_0(self) -> QGraphicsEffect {
    // unsafe{_ZN15QGraphicsEffectC2EP7QObject()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN15QGraphicsEffectC2EP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QGraphicsEffect{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicseffect.h:83
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QGraphicsEffect()

/*

*/
pub fn DeleteQGraphicsEffect(this :*mut QGraphicsEffect) {
    // let rv = qtrt::InvokeQtFunc6("_ZN15QGraphicsEffectD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qgraphicseffect.h:85
// index:0
// Public virtual Visibility=Default Availability=Available
// [32] QRectF boundingRectFor(const QRectF &) const

/*
Returns the effective bounding rectangle for this effect, given the provided rect in the device coordinates. When writing you own custom effect, you must call updateBoundingRect() whenever any parameters are changed that may cause this this function to return a different value.

See also sourceBoundingRect().
*/
impl /*struct*/ QGraphicsEffect {
  pub fn boundingRectFor_0<RetType, T: QGraphicsEffect_boundingRectFor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.boundingRectFor_0(self);
    // return 1;
  }
}
pub trait QGraphicsEffect_boundingRectFor_0<RetType> {
  fn boundingRectFor_0(self , rsthis: & QGraphicsEffect) -> RetType;
}
impl<'a> /*trait*/ QGraphicsEffect_boundingRectFor_0<usize> for (usize) {
  fn boundingRectFor_0(self , rsthis: & QGraphicsEffect) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QGraphicsEffect15boundingRectForERK6QRectF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicseffect.h:86
// index:0
// Public Visibility=Default Availability=Available
// [32] QRectF boundingRect() const

/*
Returns the effective bounding rectangle for this effect, i.e., the bounding rectangle of the source in device coordinates, adjusted by any margins applied by the effect itself.

See also boundingRectFor() and updateBoundingRect().
*/
impl /*struct*/ QGraphicsEffect {
  pub fn boundingRect_0<RetType, T: QGraphicsEffect_boundingRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.boundingRect_0(self);
    // return 1;
  }
}
pub trait QGraphicsEffect_boundingRect_0<RetType> {
  fn boundingRect_0(self , rsthis: & QGraphicsEffect) -> RetType;
}
impl<'a> /*trait*/ QGraphicsEffect_boundingRect_0<usize> for () {
  fn boundingRect_0(self , rsthis: & QGraphicsEffect) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QGraphicsEffect12boundingRectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicseffect.h:88
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isEnabled() const

/*

*/
impl /*struct*/ QGraphicsEffect {
  pub fn isEnabled_0<RetType, T: QGraphicsEffect_isEnabled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isEnabled_0(self);
    // return 1;
  }
}
pub trait QGraphicsEffect_isEnabled_0<RetType> {
  fn isEnabled_0(self , rsthis: & QGraphicsEffect) -> RetType;
}
impl<'a> /*trait*/ QGraphicsEffect_isEnabled_0<bool> for () {
  fn isEnabled_0(self , rsthis: & QGraphicsEffect) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QGraphicsEffect9isEnabledEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicseffect.h:91
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setEnabled(bool)

/*

*/
impl /*struct*/ QGraphicsEffect {
  pub fn setEnabled_0<RetType, T: QGraphicsEffect_setEnabled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setEnabled_0(self);
    // return 1;
  }
}
pub trait QGraphicsEffect_setEnabled_0<RetType> {
  fn setEnabled_0(self , rsthis: & QGraphicsEffect) -> RetType;
}
impl<'a> /*trait*/ QGraphicsEffect_setEnabled_0<(/*void*/)> for (bool) {
  fn setEnabled_0(self , rsthis: & QGraphicsEffect) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN15QGraphicsEffect10setEnabledEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicseffect.h:92
// index:0
// Public Visibility=Default Availability=Available
// [-2] void update()

/*
Schedules a redraw of the effect. Call this function whenever the effect needs to be redrawn. This function does not trigger a redraw of the source.

See also updateBoundingRect().
*/
impl /*struct*/ QGraphicsEffect {
  pub fn update_0<RetType, T: QGraphicsEffect_update_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.update_0(self);
    // return 1;
  }
}
pub trait QGraphicsEffect_update_0<RetType> {
  fn update_0(self , rsthis: & QGraphicsEffect) -> RetType;
}
impl<'a> /*trait*/ QGraphicsEffect_update_0<(/*void*/)> for () {
  fn update_0(self , rsthis: & QGraphicsEffect) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN15QGraphicsEffect6updateEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicseffect.h:95
// index:0
// Public Visibility=Default Availability=Available
// [-2] void enabledChanged(bool)

/*
This signal is emitted whenever the effect is enabled or disabled. The enabled parameter holds the effects's new enabled state.

Note: Notifier signal for property enabled. 

See also isEnabled().
*/
impl /*struct*/ QGraphicsEffect {
  pub fn enabledChanged_0<RetType, T: QGraphicsEffect_enabledChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.enabledChanged_0(self);
    // return 1;
  }
}
pub trait QGraphicsEffect_enabledChanged_0<RetType> {
  fn enabledChanged_0(self , rsthis: & QGraphicsEffect) -> RetType;
}
impl<'a> /*trait*/ QGraphicsEffect_enabledChanged_0<(/*void*/)> for (bool) {
  fn enabledChanged_0(self , rsthis: & QGraphicsEffect) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN15QGraphicsEffect14enabledChangedEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicseffect.h:99
// index:0
// Protected purevirtual virtual Visibility=Default Availability=Available
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
impl /*struct*/ QGraphicsEffect {
  pub fn draw_0<RetType, T: QGraphicsEffect_draw_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.draw_0(self);
    // return 1;
  }
}
pub trait QGraphicsEffect_draw_0<RetType> {
  fn draw_0(self , rsthis: & QGraphicsEffect) -> RetType;
}
impl<'a> /*trait*/ QGraphicsEffect_draw_0<(/*void*/)> for (usize) {
  fn draw_0(self , rsthis: & QGraphicsEffect) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QGraphicsEffect4drawEP8QPainter", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicseffect.h:100
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void sourceChanged(QGraphicsEffect::ChangeFlags)

/*
This virtual function is called by QGraphicsEffect to notify the effect that the source has changed. If the effect applies any cache, then this cache must be purged in order to reflect the new appearance of the source.

The flags describes what has changed.
*/
impl /*struct*/ QGraphicsEffect {
  pub fn sourceChanged_0<RetType, T: QGraphicsEffect_sourceChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sourceChanged_0(self);
    // return 1;
  }
}
pub trait QGraphicsEffect_sourceChanged_0<RetType> {
  fn sourceChanged_0(self , rsthis: & QGraphicsEffect) -> RetType;
}
impl<'a> /*trait*/ QGraphicsEffect_sourceChanged_0<(/*void*/)> for (i32) {
  fn sourceChanged_0(self , rsthis: & QGraphicsEffect) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN15QGraphicsEffect13sourceChangedE6QFlagsINS_10ChangeFlagEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicseffect.h:101
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void updateBoundingRect()

/*
This function notifies the effect framework when the effect's bounding rectangle has changed. As a custom effect author, you must call this function whenever you change any parameters that will cause the virtual boundingRectFor() function to return a different value.

This function will call update() if this is necessary.

See also boundingRectFor(), boundingRect(), and sourceBoundingRect().
*/
impl /*struct*/ QGraphicsEffect {
  pub fn updateBoundingRect_0<RetType, T: QGraphicsEffect_updateBoundingRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.updateBoundingRect_0(self);
    // return 1;
  }
}
pub trait QGraphicsEffect_updateBoundingRect_0<RetType> {
  fn updateBoundingRect_0(self , rsthis: & QGraphicsEffect) -> RetType;
}
impl<'a> /*trait*/ QGraphicsEffect_updateBoundingRect_0<(/*void*/)> for () {
  fn updateBoundingRect_0(self , rsthis: & QGraphicsEffect) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN15QGraphicsEffect18updateBoundingRectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicseffect.h:103
// index:0
// Protected Visibility=Default Availability=Available
// [1] bool sourceIsPixmap() const

/*
Returns true if the source effectively is a pixmap, e.g., a QGraphicsPixmapItem.

This function is useful for optimization purposes. For instance, there's no point in drawing the source in device coordinates to avoid pixmap scaling if this function returns true - the source pixmap will be scaled anyways.
*/
impl /*struct*/ QGraphicsEffect {
  pub fn sourceIsPixmap_0<RetType, T: QGraphicsEffect_sourceIsPixmap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sourceIsPixmap_0(self);
    // return 1;
  }
}
pub trait QGraphicsEffect_sourceIsPixmap_0<RetType> {
  fn sourceIsPixmap_0(self , rsthis: & QGraphicsEffect) -> RetType;
}
impl<'a> /*trait*/ QGraphicsEffect_sourceIsPixmap_0<bool> for () {
  fn sourceIsPixmap_0(self , rsthis: & QGraphicsEffect) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QGraphicsEffect14sourceIsPixmapEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicseffect.h:104
// index:0
// Protected Visibility=Default Availability=Available
// [32] QRectF sourceBoundingRect(Qt::CoordinateSystem) const

/*
Returns the bounding rectangle of the source mapped to the given system.

Calling this function with Qt::DeviceCoordinates outside of QGraphicsEffect::draw() will give undefined results, as there is no device context available.

See also draw().
*/
impl /*struct*/ QGraphicsEffect {
  pub fn sourceBoundingRect_0<RetType, T: QGraphicsEffect_sourceBoundingRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sourceBoundingRect_0(self);
    // return 1;
  }
}
pub trait QGraphicsEffect_sourceBoundingRect_0<RetType> {
  fn sourceBoundingRect_0(self , rsthis: & QGraphicsEffect) -> RetType;
}
impl<'a> /*trait*/ QGraphicsEffect_sourceBoundingRect_0<usize> for (i32) {
  fn sourceBoundingRect_0(self , rsthis: & QGraphicsEffect) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QGraphicsEffect18sourceBoundingRectEN2Qt16CoordinateSystemE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicseffect.h:105
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void drawSource(QPainter *)

/*
Draws the source directly using the given painter.

This function should only be called from QGraphicsEffect::draw().

For example:


  MyGraphicsOpacityEffect::draw(QPainter *painter)
  {
      // Fully opaque; draw directly without going through a pixmap.
      if (qFuzzyCompare(m_opacity, 1)) {
          drawSource(painter);
          return;
      }
      ...
  }



See also QGraphicsEffect::draw().
*/
impl /*struct*/ QGraphicsEffect {
  pub fn drawSource_0<RetType, T: QGraphicsEffect_drawSource_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.drawSource_0(self);
    // return 1;
  }
}
pub trait QGraphicsEffect_drawSource_0<RetType> {
  fn drawSource_0(self , rsthis: & QGraphicsEffect) -> RetType;
}
impl<'a> /*trait*/ QGraphicsEffect_drawSource_0<(/*void*/)> for (usize) {
  fn drawSource_0(self , rsthis: & QGraphicsEffect) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QGraphicsEffect10drawSourceEP8QPainter", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicseffect.h:106
// index:0
// Protected Visibility=Default Availability=Available
// [32] QPixmap sourcePixmap(Qt::CoordinateSystem, QPoint *, QGraphicsEffect::PixmapPadMode) const

/*
Returns a pixmap with the source painted into it.

The system specifies which coordinate system to be used for the source. The optional offset parameter returns the offset where the pixmap should be painted at using the current painter. For control on how the pixmap is padded use the mode parameter.

The returned pixmap is clipped to the current painter's device rectangle when system is Qt::DeviceCoordinates.

Calling this function with Qt::DeviceCoordinates outside of QGraphicsEffect::draw() will give undefined results, as there is no device context available.

See also draw() and boundingRect().
*/
impl /*struct*/ QGraphicsEffect {
  pub fn sourcePixmap_0<RetType, T: QGraphicsEffect_sourcePixmap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sourcePixmap_0(self);
    // return 1;
  }
}
pub trait QGraphicsEffect_sourcePixmap_0<RetType> {
  fn sourcePixmap_0(self , rsthis: & QGraphicsEffect) -> RetType;
}
impl<'a> /*trait*/ QGraphicsEffect_sourcePixmap_0<usize> for (i32,usize,i32) {
  fn sourcePixmap_0(self , rsthis: & QGraphicsEffect) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QGraphicsEffect12sourcePixmapEN2Qt16CoordinateSystemEP6QPointNS_13PixmapPadModeE", 3,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}


/*


*/
pub type QGraphicsEffect__ChangeFlag = i32;
// 
pub const QGraphicsEffect__SourceAttached :QGraphicsEffect__ChangeFlag = 1;
// 
pub const QGraphicsEffect__SourceDetached :QGraphicsEffect__ChangeFlag = 2;
// 
pub const QGraphicsEffect__SourceBoundingRectChanged :QGraphicsEffect__ChangeFlag = 4;
// 
pub const QGraphicsEffect__SourceInvalidated :QGraphicsEffect__ChangeFlag = 8;
pub fn QGraphicsEffect_ChangeFlagItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QGraphicsEffect", val);
}
pub fn QGraphicsEffect_ChangeFlagItemName_s(val: i32) ->String {
  //var nilthis *QGraphicsEffect
  //return nilthis.ChangeFlagItemName(val);
  return QGraphicsEffect_ChangeFlagItemName(val);
}


/*
This enum describes how the pixmap returned from sourcePixmap should be padded.


*/
pub type QGraphicsEffect__PixmapPadMode = i32;
// The pixmap should not receive any additional padding.
pub const QGraphicsEffect__NoPad :QGraphicsEffect__PixmapPadMode = 0;
// The pixmap should be padded to ensure it has a completely transparent border.
pub const QGraphicsEffect__PadToTransparentBorder :QGraphicsEffect__PixmapPadMode = 1;
// The pixmap should be padded to match the effective bounding rectangle of the effect.
pub const QGraphicsEffect__PadToEffectiveBoundingRect :QGraphicsEffect__PixmapPadMode = 2;
pub fn QGraphicsEffect_PixmapPadModeItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QGraphicsEffect", val);
}
pub fn QGraphicsEffect_PixmapPadModeItemName_s(val: i32) ->String {
  //var nilthis *QGraphicsEffect
  //return nilthis.PixmapPadModeItemName(val);
  return QGraphicsEffect_PixmapPadModeItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
