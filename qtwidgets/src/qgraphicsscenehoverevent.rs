

// mod ::widgets::QGraphicsSceneHoverEvent
// package qtwidgets
// /usr/include/qt/QtWidgets/qgraphicssceneevent.h
// #include <qgraphicssceneevent.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 12
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
#[derive(Default)] // class sizeof(QGraphicsSceneHoverEvent)=32
pub struct QGraphicsSceneHoverEvent {
  qbase: QGraphicsSceneEvent,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QGraphicsSceneHoverEvent_ITF interface {
//    QGraphicsSceneEvent_ITF
//    QGraphicsSceneHoverEvent_PTR() *QGraphicsSceneHoverEvent
//}
//func (ptr *QGraphicsSceneHoverEvent) QGraphicsSceneHoverEvent_PTR() *QGraphicsSceneHoverEvent { return ptr }

impl /*struct*/ QGraphicsSceneHoverEvent {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QGraphicsSceneHoverEvent {
    return QGraphicsSceneHoverEvent{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QGraphicsSceneHoverEvent {
//  type Target = QGraphicsSceneHoverEventBASE;
//
//  fn deref(&self) -> &QGraphicsSceneHoverEventBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QGraphicsSceneHoverEventBASE> for QGraphicsSceneHoverEvent {
//  fn as_ref(& self) -> & QGraphicsSceneHoverEventBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:201
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QGraphicsSceneHoverEvent(QEvent::Type)

/*

*/
// QGraphicsSceneHoverEvent(QEvent::Type) ctx.fn_proto_cpp
impl /*struct*/ QGraphicsSceneHoverEvent {
  pub fn QGraphicsSceneHoverEvent_0<T: QGraphicsSceneHoverEvent_QGraphicsSceneHoverEvent_0>(value: T) -> QGraphicsSceneHoverEvent {
    let rsthis = value.QGraphicsSceneHoverEvent_0();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsSceneHoverEvent_QGraphicsSceneHoverEvent_0 {
  fn QGraphicsSceneHoverEvent_0(self) -> QGraphicsSceneHoverEvent;
}
// QGraphicsSceneHoverEvent(QEvent::Type) ctx.fn_proto_cpp
impl<'a> /*trait*/ QGraphicsSceneHoverEvent_QGraphicsSceneHoverEvent_0 for (i32) {
  fn QGraphicsSceneHoverEvent_0(self) -> QGraphicsSceneHoverEvent {
    // unsafe{_ZN24QGraphicsSceneHoverEventC2EN6QEvent4TypeE()};
    let arg0 = (&self) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN24QGraphicsSceneHoverEventC2EN6QEvent4TypeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QGraphicsSceneHoverEvent{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:202
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QGraphicsSceneHoverEvent()

/*

*/
pub fn DeleteQGraphicsSceneHoverEvent(this :*mut QGraphicsSceneHoverEvent) {
    // let rv = qtrt::InvokeQtFunc6("_ZN24QGraphicsSceneHoverEventD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 32)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:204
// index:0
// Public Visibility=Default Availability=Available
// [16] QPointF pos() const

/*

*/
impl /*struct*/ QGraphicsSceneHoverEvent {
  pub fn pos_0<RetType, T: QGraphicsSceneHoverEvent_pos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.pos_0(self);
    // return 1;
  }
}
pub trait QGraphicsSceneHoverEvent_pos_0<RetType> {
  fn pos_0(self , rsthis: & QGraphicsSceneHoverEvent) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSceneHoverEvent_pos_0<usize> for () {
  fn pos_0(self , rsthis: & QGraphicsSceneHoverEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK24QGraphicsSceneHoverEvent3posEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:205
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setPos(const QPointF &)

/*

*/
impl /*struct*/ QGraphicsSceneHoverEvent {
  pub fn setPos_0<RetType, T: QGraphicsSceneHoverEvent_setPos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPos_0(self);
    // return 1;
  }
}
pub trait QGraphicsSceneHoverEvent_setPos_0<RetType> {
  fn setPos_0(self , rsthis: & QGraphicsSceneHoverEvent) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSceneHoverEvent_setPos_0<(/*void*/)> for (usize) {
  fn setPos_0(self , rsthis: & QGraphicsSceneHoverEvent) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN24QGraphicsSceneHoverEvent6setPosERK7QPointF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:207
// index:0
// Public Visibility=Default Availability=Available
// [16] QPointF scenePos() const

/*

*/
impl /*struct*/ QGraphicsSceneHoverEvent {
  pub fn scenePos_0<RetType, T: QGraphicsSceneHoverEvent_scenePos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.scenePos_0(self);
    // return 1;
  }
}
pub trait QGraphicsSceneHoverEvent_scenePos_0<RetType> {
  fn scenePos_0(self , rsthis: & QGraphicsSceneHoverEvent) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSceneHoverEvent_scenePos_0<usize> for () {
  fn scenePos_0(self , rsthis: & QGraphicsSceneHoverEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK24QGraphicsSceneHoverEvent8scenePosEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:208
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setScenePos(const QPointF &)

/*

*/
impl /*struct*/ QGraphicsSceneHoverEvent {
  pub fn setScenePos_0<RetType, T: QGraphicsSceneHoverEvent_setScenePos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setScenePos_0(self);
    // return 1;
  }
}
pub trait QGraphicsSceneHoverEvent_setScenePos_0<RetType> {
  fn setScenePos_0(self , rsthis: & QGraphicsSceneHoverEvent) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSceneHoverEvent_setScenePos_0<(/*void*/)> for (usize) {
  fn setScenePos_0(self , rsthis: & QGraphicsSceneHoverEvent) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN24QGraphicsSceneHoverEvent11setScenePosERK7QPointF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:210
// index:0
// Public Visibility=Default Availability=Available
// [8] QPoint screenPos() const

/*

*/
impl /*struct*/ QGraphicsSceneHoverEvent {
  pub fn screenPos_0<RetType, T: QGraphicsSceneHoverEvent_screenPos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.screenPos_0(self);
    // return 1;
  }
}
pub trait QGraphicsSceneHoverEvent_screenPos_0<RetType> {
  fn screenPos_0(self , rsthis: & QGraphicsSceneHoverEvent) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSceneHoverEvent_screenPos_0<usize> for () {
  fn screenPos_0(self , rsthis: & QGraphicsSceneHoverEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK24QGraphicsSceneHoverEvent9screenPosEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:211
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setScreenPos(const QPoint &)

/*

*/
impl /*struct*/ QGraphicsSceneHoverEvent {
  pub fn setScreenPos_0<RetType, T: QGraphicsSceneHoverEvent_setScreenPos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setScreenPos_0(self);
    // return 1;
  }
}
pub trait QGraphicsSceneHoverEvent_setScreenPos_0<RetType> {
  fn setScreenPos_0(self , rsthis: & QGraphicsSceneHoverEvent) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSceneHoverEvent_setScreenPos_0<(/*void*/)> for (usize) {
  fn setScreenPos_0(self , rsthis: & QGraphicsSceneHoverEvent) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN24QGraphicsSceneHoverEvent12setScreenPosERK6QPoint", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:213
// index:0
// Public Visibility=Default Availability=Available
// [16] QPointF lastPos() const

/*

*/
impl /*struct*/ QGraphicsSceneHoverEvent {
  pub fn lastPos_0<RetType, T: QGraphicsSceneHoverEvent_lastPos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.lastPos_0(self);
    // return 1;
  }
}
pub trait QGraphicsSceneHoverEvent_lastPos_0<RetType> {
  fn lastPos_0(self , rsthis: & QGraphicsSceneHoverEvent) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSceneHoverEvent_lastPos_0<usize> for () {
  fn lastPos_0(self , rsthis: & QGraphicsSceneHoverEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK24QGraphicsSceneHoverEvent7lastPosEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:214
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setLastPos(const QPointF &)

/*

*/
impl /*struct*/ QGraphicsSceneHoverEvent {
  pub fn setLastPos_0<RetType, T: QGraphicsSceneHoverEvent_setLastPos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setLastPos_0(self);
    // return 1;
  }
}
pub trait QGraphicsSceneHoverEvent_setLastPos_0<RetType> {
  fn setLastPos_0(self , rsthis: & QGraphicsSceneHoverEvent) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSceneHoverEvent_setLastPos_0<(/*void*/)> for (usize) {
  fn setLastPos_0(self , rsthis: & QGraphicsSceneHoverEvent) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN24QGraphicsSceneHoverEvent10setLastPosERK7QPointF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:216
// index:0
// Public Visibility=Default Availability=Available
// [16] QPointF lastScenePos() const

/*

*/
impl /*struct*/ QGraphicsSceneHoverEvent {
  pub fn lastScenePos_0<RetType, T: QGraphicsSceneHoverEvent_lastScenePos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.lastScenePos_0(self);
    // return 1;
  }
}
pub trait QGraphicsSceneHoverEvent_lastScenePos_0<RetType> {
  fn lastScenePos_0(self , rsthis: & QGraphicsSceneHoverEvent) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSceneHoverEvent_lastScenePos_0<usize> for () {
  fn lastScenePos_0(self , rsthis: & QGraphicsSceneHoverEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK24QGraphicsSceneHoverEvent12lastScenePosEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:217
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setLastScenePos(const QPointF &)

/*

*/
impl /*struct*/ QGraphicsSceneHoverEvent {
  pub fn setLastScenePos_0<RetType, T: QGraphicsSceneHoverEvent_setLastScenePos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setLastScenePos_0(self);
    // return 1;
  }
}
pub trait QGraphicsSceneHoverEvent_setLastScenePos_0<RetType> {
  fn setLastScenePos_0(self , rsthis: & QGraphicsSceneHoverEvent) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSceneHoverEvent_setLastScenePos_0<(/*void*/)> for (usize) {
  fn setLastScenePos_0(self , rsthis: & QGraphicsSceneHoverEvent) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN24QGraphicsSceneHoverEvent15setLastScenePosERK7QPointF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:219
// index:0
// Public Visibility=Default Availability=Available
// [8] QPoint lastScreenPos() const

/*

*/
impl /*struct*/ QGraphicsSceneHoverEvent {
  pub fn lastScreenPos_0<RetType, T: QGraphicsSceneHoverEvent_lastScreenPos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.lastScreenPos_0(self);
    // return 1;
  }
}
pub trait QGraphicsSceneHoverEvent_lastScreenPos_0<RetType> {
  fn lastScreenPos_0(self , rsthis: & QGraphicsSceneHoverEvent) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSceneHoverEvent_lastScreenPos_0<usize> for () {
  fn lastScreenPos_0(self , rsthis: & QGraphicsSceneHoverEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK24QGraphicsSceneHoverEvent13lastScreenPosEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:220
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setLastScreenPos(const QPoint &)

/*

*/
impl /*struct*/ QGraphicsSceneHoverEvent {
  pub fn setLastScreenPos_0<RetType, T: QGraphicsSceneHoverEvent_setLastScreenPos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setLastScreenPos_0(self);
    // return 1;
  }
}
pub trait QGraphicsSceneHoverEvent_setLastScreenPos_0<RetType> {
  fn setLastScreenPos_0(self , rsthis: & QGraphicsSceneHoverEvent) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSceneHoverEvent_setLastScreenPos_0<(/*void*/)> for (usize) {
  fn setLastScreenPos_0(self , rsthis: & QGraphicsSceneHoverEvent) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN24QGraphicsSceneHoverEvent16setLastScreenPosERK6QPoint", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:222
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::KeyboardModifiers modifiers() const

/*

*/
impl /*struct*/ QGraphicsSceneHoverEvent {
  pub fn modifiers_0<RetType, T: QGraphicsSceneHoverEvent_modifiers_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.modifiers_0(self);
    // return 1;
  }
}
pub trait QGraphicsSceneHoverEvent_modifiers_0<RetType> {
  fn modifiers_0(self , rsthis: & QGraphicsSceneHoverEvent) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSceneHoverEvent_modifiers_0<i32> for () {
  fn modifiers_0(self , rsthis: & QGraphicsSceneHoverEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK24QGraphicsSceneHoverEvent9modifiersEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:223
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setModifiers(Qt::KeyboardModifiers)

/*

*/
impl /*struct*/ QGraphicsSceneHoverEvent {
  pub fn setModifiers_0<RetType, T: QGraphicsSceneHoverEvent_setModifiers_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setModifiers_0(self);
    // return 1;
  }
}
pub trait QGraphicsSceneHoverEvent_setModifiers_0<RetType> {
  fn setModifiers_0(self , rsthis: & QGraphicsSceneHoverEvent) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSceneHoverEvent_setModifiers_0<(/*void*/)> for (i32) {
  fn setModifiers_0(self , rsthis: & QGraphicsSceneHoverEvent) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN24QGraphicsSceneHoverEvent12setModifiersE6QFlagsIN2Qt16KeyboardModifierEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

//  body block end

//  keep block begin

//  keep block end
