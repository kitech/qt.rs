

// mod ::widgets::QGraphicsSceneMouseEvent
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
// extern C begin: 4
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
#[derive(Default)] // class sizeof(QGraphicsSceneMouseEvent)=32
pub struct QGraphicsSceneMouseEvent {
  qbase: QGraphicsSceneEvent,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QGraphicsSceneMouseEvent_ITF interface {
//    QGraphicsSceneEvent_ITF
//    QGraphicsSceneMouseEvent_PTR() *QGraphicsSceneMouseEvent
//}
//func (ptr *QGraphicsSceneMouseEvent) QGraphicsSceneMouseEvent_PTR() *QGraphicsSceneMouseEvent { return ptr }

impl /*struct*/ QGraphicsSceneMouseEvent {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QGraphicsSceneMouseEvent {
    return QGraphicsSceneMouseEvent{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QGraphicsSceneMouseEvent {
//  type Target = QGraphicsSceneMouseEventBASE;
//
//  fn deref(&self) -> &QGraphicsSceneMouseEventBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QGraphicsSceneMouseEventBASE> for QGraphicsSceneMouseEvent {
//  fn as_ref(& self) -> & QGraphicsSceneMouseEventBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:85
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QGraphicsSceneMouseEvent(QEvent::Type)

/*

*/
// QGraphicsSceneMouseEvent(QEvent::Type) ctx.fn_proto_cpp
impl /*struct*/ QGraphicsSceneMouseEvent {
  pub fn QGraphicsSceneMouseEvent_0<T: QGraphicsSceneMouseEvent_QGraphicsSceneMouseEvent_0>(value: T) -> QGraphicsSceneMouseEvent {
    let rsthis = value.QGraphicsSceneMouseEvent_0();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsSceneMouseEvent_QGraphicsSceneMouseEvent_0 {
  fn QGraphicsSceneMouseEvent_0(self) -> QGraphicsSceneMouseEvent;
}
// QGraphicsSceneMouseEvent(QEvent::Type) ctx.fn_proto_cpp
impl<'a> /*trait*/ QGraphicsSceneMouseEvent_QGraphicsSceneMouseEvent_0 for (i32) {
  fn QGraphicsSceneMouseEvent_0(self) -> QGraphicsSceneMouseEvent {
    // unsafe{_ZN24QGraphicsSceneMouseEventC2EN6QEvent4TypeE()};
    let arg0 = (&self) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN24QGraphicsSceneMouseEventC2EN6QEvent4TypeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QGraphicsSceneMouseEvent{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:86
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QGraphicsSceneMouseEvent()

/*

*/
pub fn DeleteQGraphicsSceneMouseEvent(this :*mut QGraphicsSceneMouseEvent) {
    // let rv = qtrt::InvokeQtFunc6("_ZN24QGraphicsSceneMouseEventD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 32)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:88
// index:0
// Public Visibility=Default Availability=Available
// [16] QPointF pos() const

/*

*/
impl /*struct*/ QGraphicsSceneMouseEvent {
  pub fn pos_0<RetType, T: QGraphicsSceneMouseEvent_pos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.pos_0(self);
    // return 1;
  }
}
pub trait QGraphicsSceneMouseEvent_pos_0<RetType> {
  fn pos_0(self , rsthis: & QGraphicsSceneMouseEvent) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSceneMouseEvent_pos_0<usize> for () {
  fn pos_0(self , rsthis: & QGraphicsSceneMouseEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK24QGraphicsSceneMouseEvent3posEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:89
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setPos(const QPointF &)

/*

*/
impl /*struct*/ QGraphicsSceneMouseEvent {
  pub fn setPos_0<RetType, T: QGraphicsSceneMouseEvent_setPos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPos_0(self);
    // return 1;
  }
}
pub trait QGraphicsSceneMouseEvent_setPos_0<RetType> {
  fn setPos_0(self , rsthis: & QGraphicsSceneMouseEvent) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSceneMouseEvent_setPos_0<(/*void*/)> for (usize) {
  fn setPos_0(self , rsthis: & QGraphicsSceneMouseEvent) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN24QGraphicsSceneMouseEvent6setPosERK7QPointF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:91
// index:0
// Public Visibility=Default Availability=Available
// [16] QPointF scenePos() const

/*

*/
impl /*struct*/ QGraphicsSceneMouseEvent {
  pub fn scenePos_0<RetType, T: QGraphicsSceneMouseEvent_scenePos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.scenePos_0(self);
    // return 1;
  }
}
pub trait QGraphicsSceneMouseEvent_scenePos_0<RetType> {
  fn scenePos_0(self , rsthis: & QGraphicsSceneMouseEvent) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSceneMouseEvent_scenePos_0<usize> for () {
  fn scenePos_0(self , rsthis: & QGraphicsSceneMouseEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK24QGraphicsSceneMouseEvent8scenePosEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:92
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setScenePos(const QPointF &)

/*

*/
impl /*struct*/ QGraphicsSceneMouseEvent {
  pub fn setScenePos_0<RetType, T: QGraphicsSceneMouseEvent_setScenePos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setScenePos_0(self);
    // return 1;
  }
}
pub trait QGraphicsSceneMouseEvent_setScenePos_0<RetType> {
  fn setScenePos_0(self , rsthis: & QGraphicsSceneMouseEvent) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSceneMouseEvent_setScenePos_0<(/*void*/)> for (usize) {
  fn setScenePos_0(self , rsthis: & QGraphicsSceneMouseEvent) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN24QGraphicsSceneMouseEvent11setScenePosERK7QPointF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:94
// index:0
// Public Visibility=Default Availability=Available
// [8] QPoint screenPos() const

/*

*/
impl /*struct*/ QGraphicsSceneMouseEvent {
  pub fn screenPos_0<RetType, T: QGraphicsSceneMouseEvent_screenPos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.screenPos_0(self);
    // return 1;
  }
}
pub trait QGraphicsSceneMouseEvent_screenPos_0<RetType> {
  fn screenPos_0(self , rsthis: & QGraphicsSceneMouseEvent) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSceneMouseEvent_screenPos_0<usize> for () {
  fn screenPos_0(self , rsthis: & QGraphicsSceneMouseEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK24QGraphicsSceneMouseEvent9screenPosEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:95
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setScreenPos(const QPoint &)

/*

*/
impl /*struct*/ QGraphicsSceneMouseEvent {
  pub fn setScreenPos_0<RetType, T: QGraphicsSceneMouseEvent_setScreenPos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setScreenPos_0(self);
    // return 1;
  }
}
pub trait QGraphicsSceneMouseEvent_setScreenPos_0<RetType> {
  fn setScreenPos_0(self , rsthis: & QGraphicsSceneMouseEvent) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSceneMouseEvent_setScreenPos_0<(/*void*/)> for (usize) {
  fn setScreenPos_0(self , rsthis: & QGraphicsSceneMouseEvent) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN24QGraphicsSceneMouseEvent12setScreenPosERK6QPoint", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:97
// index:0
// Public Visibility=Default Availability=Available
// [16] QPointF buttonDownPos(Qt::MouseButton) const

/*

*/
impl /*struct*/ QGraphicsSceneMouseEvent {
  pub fn buttonDownPos_0<RetType, T: QGraphicsSceneMouseEvent_buttonDownPos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.buttonDownPos_0(self);
    // return 1;
  }
}
pub trait QGraphicsSceneMouseEvent_buttonDownPos_0<RetType> {
  fn buttonDownPos_0(self , rsthis: & QGraphicsSceneMouseEvent) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSceneMouseEvent_buttonDownPos_0<usize> for (i32) {
  fn buttonDownPos_0(self , rsthis: & QGraphicsSceneMouseEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK24QGraphicsSceneMouseEvent13buttonDownPosEN2Qt11MouseButtonE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:98
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setButtonDownPos(Qt::MouseButton, const QPointF &)

/*

*/
impl /*struct*/ QGraphicsSceneMouseEvent {
  pub fn setButtonDownPos_0<RetType, T: QGraphicsSceneMouseEvent_setButtonDownPos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setButtonDownPos_0(self);
    // return 1;
  }
}
pub trait QGraphicsSceneMouseEvent_setButtonDownPos_0<RetType> {
  fn setButtonDownPos_0(self , rsthis: & QGraphicsSceneMouseEvent) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSceneMouseEvent_setButtonDownPos_0<(/*void*/)> for (i32,usize) {
  fn setButtonDownPos_0(self , rsthis: & QGraphicsSceneMouseEvent) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN24QGraphicsSceneMouseEvent16setButtonDownPosEN2Qt11MouseButtonERK7QPointF", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:100
// index:0
// Public Visibility=Default Availability=Available
// [16] QPointF buttonDownScenePos(Qt::MouseButton) const

/*

*/
impl /*struct*/ QGraphicsSceneMouseEvent {
  pub fn buttonDownScenePos_0<RetType, T: QGraphicsSceneMouseEvent_buttonDownScenePos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.buttonDownScenePos_0(self);
    // return 1;
  }
}
pub trait QGraphicsSceneMouseEvent_buttonDownScenePos_0<RetType> {
  fn buttonDownScenePos_0(self , rsthis: & QGraphicsSceneMouseEvent) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSceneMouseEvent_buttonDownScenePos_0<usize> for (i32) {
  fn buttonDownScenePos_0(self , rsthis: & QGraphicsSceneMouseEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK24QGraphicsSceneMouseEvent18buttonDownScenePosEN2Qt11MouseButtonE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:101
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setButtonDownScenePos(Qt::MouseButton, const QPointF &)

/*

*/
impl /*struct*/ QGraphicsSceneMouseEvent {
  pub fn setButtonDownScenePos_0<RetType, T: QGraphicsSceneMouseEvent_setButtonDownScenePos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setButtonDownScenePos_0(self);
    // return 1;
  }
}
pub trait QGraphicsSceneMouseEvent_setButtonDownScenePos_0<RetType> {
  fn setButtonDownScenePos_0(self , rsthis: & QGraphicsSceneMouseEvent) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSceneMouseEvent_setButtonDownScenePos_0<(/*void*/)> for (i32,usize) {
  fn setButtonDownScenePos_0(self , rsthis: & QGraphicsSceneMouseEvent) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN24QGraphicsSceneMouseEvent21setButtonDownScenePosEN2Qt11MouseButtonERK7QPointF", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:103
// index:0
// Public Visibility=Default Availability=Available
// [8] QPoint buttonDownScreenPos(Qt::MouseButton) const

/*

*/
impl /*struct*/ QGraphicsSceneMouseEvent {
  pub fn buttonDownScreenPos_0<RetType, T: QGraphicsSceneMouseEvent_buttonDownScreenPos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.buttonDownScreenPos_0(self);
    // return 1;
  }
}
pub trait QGraphicsSceneMouseEvent_buttonDownScreenPos_0<RetType> {
  fn buttonDownScreenPos_0(self , rsthis: & QGraphicsSceneMouseEvent) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSceneMouseEvent_buttonDownScreenPos_0<usize> for (i32) {
  fn buttonDownScreenPos_0(self , rsthis: & QGraphicsSceneMouseEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK24QGraphicsSceneMouseEvent19buttonDownScreenPosEN2Qt11MouseButtonE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:104
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setButtonDownScreenPos(Qt::MouseButton, const QPoint &)

/*

*/
impl /*struct*/ QGraphicsSceneMouseEvent {
  pub fn setButtonDownScreenPos_0<RetType, T: QGraphicsSceneMouseEvent_setButtonDownScreenPos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setButtonDownScreenPos_0(self);
    // return 1;
  }
}
pub trait QGraphicsSceneMouseEvent_setButtonDownScreenPos_0<RetType> {
  fn setButtonDownScreenPos_0(self , rsthis: & QGraphicsSceneMouseEvent) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSceneMouseEvent_setButtonDownScreenPos_0<(/*void*/)> for (i32,usize) {
  fn setButtonDownScreenPos_0(self , rsthis: & QGraphicsSceneMouseEvent) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN24QGraphicsSceneMouseEvent22setButtonDownScreenPosEN2Qt11MouseButtonERK6QPoint", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:106
// index:0
// Public Visibility=Default Availability=Available
// [16] QPointF lastPos() const

/*

*/
impl /*struct*/ QGraphicsSceneMouseEvent {
  pub fn lastPos_0<RetType, T: QGraphicsSceneMouseEvent_lastPos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.lastPos_0(self);
    // return 1;
  }
}
pub trait QGraphicsSceneMouseEvent_lastPos_0<RetType> {
  fn lastPos_0(self , rsthis: & QGraphicsSceneMouseEvent) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSceneMouseEvent_lastPos_0<usize> for () {
  fn lastPos_0(self , rsthis: & QGraphicsSceneMouseEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK24QGraphicsSceneMouseEvent7lastPosEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:107
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setLastPos(const QPointF &)

/*

*/
impl /*struct*/ QGraphicsSceneMouseEvent {
  pub fn setLastPos_0<RetType, T: QGraphicsSceneMouseEvent_setLastPos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setLastPos_0(self);
    // return 1;
  }
}
pub trait QGraphicsSceneMouseEvent_setLastPos_0<RetType> {
  fn setLastPos_0(self , rsthis: & QGraphicsSceneMouseEvent) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSceneMouseEvent_setLastPos_0<(/*void*/)> for (usize) {
  fn setLastPos_0(self , rsthis: & QGraphicsSceneMouseEvent) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN24QGraphicsSceneMouseEvent10setLastPosERK7QPointF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:109
// index:0
// Public Visibility=Default Availability=Available
// [16] QPointF lastScenePos() const

/*

*/
impl /*struct*/ QGraphicsSceneMouseEvent {
  pub fn lastScenePos_0<RetType, T: QGraphicsSceneMouseEvent_lastScenePos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.lastScenePos_0(self);
    // return 1;
  }
}
pub trait QGraphicsSceneMouseEvent_lastScenePos_0<RetType> {
  fn lastScenePos_0(self , rsthis: & QGraphicsSceneMouseEvent) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSceneMouseEvent_lastScenePos_0<usize> for () {
  fn lastScenePos_0(self , rsthis: & QGraphicsSceneMouseEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK24QGraphicsSceneMouseEvent12lastScenePosEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:110
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setLastScenePos(const QPointF &)

/*

*/
impl /*struct*/ QGraphicsSceneMouseEvent {
  pub fn setLastScenePos_0<RetType, T: QGraphicsSceneMouseEvent_setLastScenePos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setLastScenePos_0(self);
    // return 1;
  }
}
pub trait QGraphicsSceneMouseEvent_setLastScenePos_0<RetType> {
  fn setLastScenePos_0(self , rsthis: & QGraphicsSceneMouseEvent) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSceneMouseEvent_setLastScenePos_0<(/*void*/)> for (usize) {
  fn setLastScenePos_0(self , rsthis: & QGraphicsSceneMouseEvent) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN24QGraphicsSceneMouseEvent15setLastScenePosERK7QPointF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:112
// index:0
// Public Visibility=Default Availability=Available
// [8] QPoint lastScreenPos() const

/*

*/
impl /*struct*/ QGraphicsSceneMouseEvent {
  pub fn lastScreenPos_0<RetType, T: QGraphicsSceneMouseEvent_lastScreenPos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.lastScreenPos_0(self);
    // return 1;
  }
}
pub trait QGraphicsSceneMouseEvent_lastScreenPos_0<RetType> {
  fn lastScreenPos_0(self , rsthis: & QGraphicsSceneMouseEvent) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSceneMouseEvent_lastScreenPos_0<usize> for () {
  fn lastScreenPos_0(self , rsthis: & QGraphicsSceneMouseEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK24QGraphicsSceneMouseEvent13lastScreenPosEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:113
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setLastScreenPos(const QPoint &)

/*

*/
impl /*struct*/ QGraphicsSceneMouseEvent {
  pub fn setLastScreenPos_0<RetType, T: QGraphicsSceneMouseEvent_setLastScreenPos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setLastScreenPos_0(self);
    // return 1;
  }
}
pub trait QGraphicsSceneMouseEvent_setLastScreenPos_0<RetType> {
  fn setLastScreenPos_0(self , rsthis: & QGraphicsSceneMouseEvent) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSceneMouseEvent_setLastScreenPos_0<(/*void*/)> for (usize) {
  fn setLastScreenPos_0(self , rsthis: & QGraphicsSceneMouseEvent) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN24QGraphicsSceneMouseEvent16setLastScreenPosERK6QPoint", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:115
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::MouseButtons buttons() const

/*

*/
impl /*struct*/ QGraphicsSceneMouseEvent {
  pub fn buttons_0<RetType, T: QGraphicsSceneMouseEvent_buttons_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.buttons_0(self);
    // return 1;
  }
}
pub trait QGraphicsSceneMouseEvent_buttons_0<RetType> {
  fn buttons_0(self , rsthis: & QGraphicsSceneMouseEvent) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSceneMouseEvent_buttons_0<i32> for () {
  fn buttons_0(self , rsthis: & QGraphicsSceneMouseEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK24QGraphicsSceneMouseEvent7buttonsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:116
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setButtons(Qt::MouseButtons)

/*

*/
impl /*struct*/ QGraphicsSceneMouseEvent {
  pub fn setButtons_0<RetType, T: QGraphicsSceneMouseEvent_setButtons_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setButtons_0(self);
    // return 1;
  }
}
pub trait QGraphicsSceneMouseEvent_setButtons_0<RetType> {
  fn setButtons_0(self , rsthis: & QGraphicsSceneMouseEvent) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSceneMouseEvent_setButtons_0<(/*void*/)> for (i32) {
  fn setButtons_0(self , rsthis: & QGraphicsSceneMouseEvent) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN24QGraphicsSceneMouseEvent10setButtonsE6QFlagsIN2Qt11MouseButtonEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:118
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::MouseButton button() const

/*

*/
impl /*struct*/ QGraphicsSceneMouseEvent {
  pub fn button_0<RetType, T: QGraphicsSceneMouseEvent_button_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.button_0(self);
    // return 1;
  }
}
pub trait QGraphicsSceneMouseEvent_button_0<RetType> {
  fn button_0(self , rsthis: & QGraphicsSceneMouseEvent) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSceneMouseEvent_button_0<i32> for () {
  fn button_0(self , rsthis: & QGraphicsSceneMouseEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK24QGraphicsSceneMouseEvent6buttonEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:119
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setButton(Qt::MouseButton)

/*

*/
impl /*struct*/ QGraphicsSceneMouseEvent {
  pub fn setButton_0<RetType, T: QGraphicsSceneMouseEvent_setButton_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setButton_0(self);
    // return 1;
  }
}
pub trait QGraphicsSceneMouseEvent_setButton_0<RetType> {
  fn setButton_0(self , rsthis: & QGraphicsSceneMouseEvent) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSceneMouseEvent_setButton_0<(/*void*/)> for (i32) {
  fn setButton_0(self , rsthis: & QGraphicsSceneMouseEvent) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN24QGraphicsSceneMouseEvent9setButtonEN2Qt11MouseButtonE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:121
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::KeyboardModifiers modifiers() const

/*

*/
impl /*struct*/ QGraphicsSceneMouseEvent {
  pub fn modifiers_0<RetType, T: QGraphicsSceneMouseEvent_modifiers_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.modifiers_0(self);
    // return 1;
  }
}
pub trait QGraphicsSceneMouseEvent_modifiers_0<RetType> {
  fn modifiers_0(self , rsthis: & QGraphicsSceneMouseEvent) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSceneMouseEvent_modifiers_0<i32> for () {
  fn modifiers_0(self , rsthis: & QGraphicsSceneMouseEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK24QGraphicsSceneMouseEvent9modifiersEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:122
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setModifiers(Qt::KeyboardModifiers)

/*

*/
impl /*struct*/ QGraphicsSceneMouseEvent {
  pub fn setModifiers_0<RetType, T: QGraphicsSceneMouseEvent_setModifiers_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setModifiers_0(self);
    // return 1;
  }
}
pub trait QGraphicsSceneMouseEvent_setModifiers_0<RetType> {
  fn setModifiers_0(self , rsthis: & QGraphicsSceneMouseEvent) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSceneMouseEvent_setModifiers_0<(/*void*/)> for (i32) {
  fn setModifiers_0(self , rsthis: & QGraphicsSceneMouseEvent) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN24QGraphicsSceneMouseEvent12setModifiersE6QFlagsIN2Qt16KeyboardModifierEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:124
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::MouseEventSource source() const

/*

*/
impl /*struct*/ QGraphicsSceneMouseEvent {
  pub fn source_0<RetType, T: QGraphicsSceneMouseEvent_source_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.source_0(self);
    // return 1;
  }
}
pub trait QGraphicsSceneMouseEvent_source_0<RetType> {
  fn source_0(self , rsthis: & QGraphicsSceneMouseEvent) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSceneMouseEvent_source_0<i32> for () {
  fn source_0(self , rsthis: & QGraphicsSceneMouseEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK24QGraphicsSceneMouseEvent6sourceEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:125
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setSource(Qt::MouseEventSource)

/*

*/
impl /*struct*/ QGraphicsSceneMouseEvent {
  pub fn setSource_0<RetType, T: QGraphicsSceneMouseEvent_setSource_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSource_0(self);
    // return 1;
  }
}
pub trait QGraphicsSceneMouseEvent_setSource_0<RetType> {
  fn setSource_0(self , rsthis: & QGraphicsSceneMouseEvent) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSceneMouseEvent_setSource_0<(/*void*/)> for (i32) {
  fn setSource_0(self , rsthis: & QGraphicsSceneMouseEvent) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN24QGraphicsSceneMouseEvent9setSourceEN2Qt16MouseEventSourceE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:127
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::MouseEventFlags flags() const

/*

*/
impl /*struct*/ QGraphicsSceneMouseEvent {
  pub fn flags_0<RetType, T: QGraphicsSceneMouseEvent_flags_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.flags_0(self);
    // return 1;
  }
}
pub trait QGraphicsSceneMouseEvent_flags_0<RetType> {
  fn flags_0(self , rsthis: & QGraphicsSceneMouseEvent) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSceneMouseEvent_flags_0<i32> for () {
  fn flags_0(self , rsthis: & QGraphicsSceneMouseEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK24QGraphicsSceneMouseEvent5flagsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:128
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFlags(Qt::MouseEventFlags)

/*

*/
impl /*struct*/ QGraphicsSceneMouseEvent {
  pub fn setFlags_0<RetType, T: QGraphicsSceneMouseEvent_setFlags_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFlags_0(self);
    // return 1;
  }
}
pub trait QGraphicsSceneMouseEvent_setFlags_0<RetType> {
  fn setFlags_0(self , rsthis: & QGraphicsSceneMouseEvent) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSceneMouseEvent_setFlags_0<(/*void*/)> for (i32) {
  fn setFlags_0(self , rsthis: & QGraphicsSceneMouseEvent) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN24QGraphicsSceneMouseEvent8setFlagsE6QFlagsIN2Qt14MouseEventFlagEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

//  body block end

//  keep block begin

//  keep block end
