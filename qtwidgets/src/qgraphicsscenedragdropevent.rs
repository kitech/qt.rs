

// mod ::widgets::QGraphicsSceneDragDropEvent
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
// extern C begin: 6
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
#[derive(Default)] // class sizeof(QGraphicsSceneDragDropEvent)=32
pub struct QGraphicsSceneDragDropEvent {
  qbase: QGraphicsSceneEvent,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QGraphicsSceneDragDropEvent_ITF interface {
//    QGraphicsSceneEvent_ITF
//    QGraphicsSceneDragDropEvent_PTR() *QGraphicsSceneDragDropEvent
//}
//func (ptr *QGraphicsSceneDragDropEvent) QGraphicsSceneDragDropEvent_PTR() *QGraphicsSceneDragDropEvent { return ptr }

impl /*struct*/ QGraphicsSceneDragDropEvent {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QGraphicsSceneDragDropEvent {
    return QGraphicsSceneDragDropEvent{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QGraphicsSceneDragDropEvent {
//  type Target = QGraphicsSceneDragDropEventBASE;
//
//  fn deref(&self) -> &QGraphicsSceneDragDropEventBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QGraphicsSceneDragDropEventBASE> for QGraphicsSceneDragDropEvent {
//  fn as_ref(& self) -> & QGraphicsSceneDragDropEventBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:252
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QGraphicsSceneDragDropEvent(QEvent::Type)

/*

*/
// QGraphicsSceneDragDropEvent(QEvent::Type) ctx.fn_proto_cpp
impl /*struct*/ QGraphicsSceneDragDropEvent {
  pub fn QGraphicsSceneDragDropEvent_0<T: QGraphicsSceneDragDropEvent_QGraphicsSceneDragDropEvent_0>(value: T) -> QGraphicsSceneDragDropEvent {
    let rsthis = value.QGraphicsSceneDragDropEvent_0();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsSceneDragDropEvent_QGraphicsSceneDragDropEvent_0 {
  fn QGraphicsSceneDragDropEvent_0(self) -> QGraphicsSceneDragDropEvent;
}
// QGraphicsSceneDragDropEvent(QEvent::Type) ctx.fn_proto_cpp
impl<'a> /*trait*/ QGraphicsSceneDragDropEvent_QGraphicsSceneDragDropEvent_0 for (i32) {
  fn QGraphicsSceneDragDropEvent_0(self) -> QGraphicsSceneDragDropEvent {
    // unsafe{_ZN27QGraphicsSceneDragDropEventC2EN6QEvent4TypeE()};
    let arg0 = (&self) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN27QGraphicsSceneDragDropEventC2EN6QEvent4TypeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QGraphicsSceneDragDropEvent{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:253
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QGraphicsSceneDragDropEvent()

/*

*/
pub fn DeleteQGraphicsSceneDragDropEvent(this :*mut QGraphicsSceneDragDropEvent) {
    // let rv = qtrt::InvokeQtFunc6("_ZN27QGraphicsSceneDragDropEventD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 32)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:255
// index:0
// Public Visibility=Default Availability=Available
// [16] QPointF pos() const

/*

*/
impl /*struct*/ QGraphicsSceneDragDropEvent {
  pub fn pos_0<RetType, T: QGraphicsSceneDragDropEvent_pos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.pos_0(self);
    // return 1;
  }
}
pub trait QGraphicsSceneDragDropEvent_pos_0<RetType> {
  fn pos_0(self , rsthis: & QGraphicsSceneDragDropEvent) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSceneDragDropEvent_pos_0<usize> for () {
  fn pos_0(self , rsthis: & QGraphicsSceneDragDropEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK27QGraphicsSceneDragDropEvent3posEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:256
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setPos(const QPointF &)

/*

*/
impl /*struct*/ QGraphicsSceneDragDropEvent {
  pub fn setPos_0<RetType, T: QGraphicsSceneDragDropEvent_setPos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPos_0(self);
    // return 1;
  }
}
pub trait QGraphicsSceneDragDropEvent_setPos_0<RetType> {
  fn setPos_0(self , rsthis: & QGraphicsSceneDragDropEvent) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSceneDragDropEvent_setPos_0<(/*void*/)> for (usize) {
  fn setPos_0(self , rsthis: & QGraphicsSceneDragDropEvent) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN27QGraphicsSceneDragDropEvent6setPosERK7QPointF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:258
// index:0
// Public Visibility=Default Availability=Available
// [16] QPointF scenePos() const

/*

*/
impl /*struct*/ QGraphicsSceneDragDropEvent {
  pub fn scenePos_0<RetType, T: QGraphicsSceneDragDropEvent_scenePos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.scenePos_0(self);
    // return 1;
  }
}
pub trait QGraphicsSceneDragDropEvent_scenePos_0<RetType> {
  fn scenePos_0(self , rsthis: & QGraphicsSceneDragDropEvent) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSceneDragDropEvent_scenePos_0<usize> for () {
  fn scenePos_0(self , rsthis: & QGraphicsSceneDragDropEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK27QGraphicsSceneDragDropEvent8scenePosEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:259
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setScenePos(const QPointF &)

/*

*/
impl /*struct*/ QGraphicsSceneDragDropEvent {
  pub fn setScenePos_0<RetType, T: QGraphicsSceneDragDropEvent_setScenePos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setScenePos_0(self);
    // return 1;
  }
}
pub trait QGraphicsSceneDragDropEvent_setScenePos_0<RetType> {
  fn setScenePos_0(self , rsthis: & QGraphicsSceneDragDropEvent) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSceneDragDropEvent_setScenePos_0<(/*void*/)> for (usize) {
  fn setScenePos_0(self , rsthis: & QGraphicsSceneDragDropEvent) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN27QGraphicsSceneDragDropEvent11setScenePosERK7QPointF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:261
// index:0
// Public Visibility=Default Availability=Available
// [8] QPoint screenPos() const

/*

*/
impl /*struct*/ QGraphicsSceneDragDropEvent {
  pub fn screenPos_0<RetType, T: QGraphicsSceneDragDropEvent_screenPos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.screenPos_0(self);
    // return 1;
  }
}
pub trait QGraphicsSceneDragDropEvent_screenPos_0<RetType> {
  fn screenPos_0(self , rsthis: & QGraphicsSceneDragDropEvent) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSceneDragDropEvent_screenPos_0<usize> for () {
  fn screenPos_0(self , rsthis: & QGraphicsSceneDragDropEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK27QGraphicsSceneDragDropEvent9screenPosEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:262
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setScreenPos(const QPoint &)

/*

*/
impl /*struct*/ QGraphicsSceneDragDropEvent {
  pub fn setScreenPos_0<RetType, T: QGraphicsSceneDragDropEvent_setScreenPos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setScreenPos_0(self);
    // return 1;
  }
}
pub trait QGraphicsSceneDragDropEvent_setScreenPos_0<RetType> {
  fn setScreenPos_0(self , rsthis: & QGraphicsSceneDragDropEvent) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSceneDragDropEvent_setScreenPos_0<(/*void*/)> for (usize) {
  fn setScreenPos_0(self , rsthis: & QGraphicsSceneDragDropEvent) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN27QGraphicsSceneDragDropEvent12setScreenPosERK6QPoint", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:264
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::MouseButtons buttons() const

/*

*/
impl /*struct*/ QGraphicsSceneDragDropEvent {
  pub fn buttons_0<RetType, T: QGraphicsSceneDragDropEvent_buttons_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.buttons_0(self);
    // return 1;
  }
}
pub trait QGraphicsSceneDragDropEvent_buttons_0<RetType> {
  fn buttons_0(self , rsthis: & QGraphicsSceneDragDropEvent) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSceneDragDropEvent_buttons_0<i32> for () {
  fn buttons_0(self , rsthis: & QGraphicsSceneDragDropEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK27QGraphicsSceneDragDropEvent7buttonsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:265
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setButtons(Qt::MouseButtons)

/*

*/
impl /*struct*/ QGraphicsSceneDragDropEvent {
  pub fn setButtons_0<RetType, T: QGraphicsSceneDragDropEvent_setButtons_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setButtons_0(self);
    // return 1;
  }
}
pub trait QGraphicsSceneDragDropEvent_setButtons_0<RetType> {
  fn setButtons_0(self , rsthis: & QGraphicsSceneDragDropEvent) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSceneDragDropEvent_setButtons_0<(/*void*/)> for (i32) {
  fn setButtons_0(self , rsthis: & QGraphicsSceneDragDropEvent) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN27QGraphicsSceneDragDropEvent10setButtonsE6QFlagsIN2Qt11MouseButtonEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:267
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::KeyboardModifiers modifiers() const

/*

*/
impl /*struct*/ QGraphicsSceneDragDropEvent {
  pub fn modifiers_0<RetType, T: QGraphicsSceneDragDropEvent_modifiers_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.modifiers_0(self);
    // return 1;
  }
}
pub trait QGraphicsSceneDragDropEvent_modifiers_0<RetType> {
  fn modifiers_0(self , rsthis: & QGraphicsSceneDragDropEvent) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSceneDragDropEvent_modifiers_0<i32> for () {
  fn modifiers_0(self , rsthis: & QGraphicsSceneDragDropEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK27QGraphicsSceneDragDropEvent9modifiersEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:268
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setModifiers(Qt::KeyboardModifiers)

/*

*/
impl /*struct*/ QGraphicsSceneDragDropEvent {
  pub fn setModifiers_0<RetType, T: QGraphicsSceneDragDropEvent_setModifiers_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setModifiers_0(self);
    // return 1;
  }
}
pub trait QGraphicsSceneDragDropEvent_setModifiers_0<RetType> {
  fn setModifiers_0(self , rsthis: & QGraphicsSceneDragDropEvent) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSceneDragDropEvent_setModifiers_0<(/*void*/)> for (i32) {
  fn setModifiers_0(self , rsthis: & QGraphicsSceneDragDropEvent) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN27QGraphicsSceneDragDropEvent12setModifiersE6QFlagsIN2Qt16KeyboardModifierEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:270
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::DropActions possibleActions() const

/*

*/
impl /*struct*/ QGraphicsSceneDragDropEvent {
  pub fn possibleActions_0<RetType, T: QGraphicsSceneDragDropEvent_possibleActions_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.possibleActions_0(self);
    // return 1;
  }
}
pub trait QGraphicsSceneDragDropEvent_possibleActions_0<RetType> {
  fn possibleActions_0(self , rsthis: & QGraphicsSceneDragDropEvent) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSceneDragDropEvent_possibleActions_0<i32> for () {
  fn possibleActions_0(self , rsthis: & QGraphicsSceneDragDropEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK27QGraphicsSceneDragDropEvent15possibleActionsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:271
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setPossibleActions(Qt::DropActions)

/*

*/
impl /*struct*/ QGraphicsSceneDragDropEvent {
  pub fn setPossibleActions_0<RetType, T: QGraphicsSceneDragDropEvent_setPossibleActions_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPossibleActions_0(self);
    // return 1;
  }
}
pub trait QGraphicsSceneDragDropEvent_setPossibleActions_0<RetType> {
  fn setPossibleActions_0(self , rsthis: & QGraphicsSceneDragDropEvent) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSceneDragDropEvent_setPossibleActions_0<(/*void*/)> for (i32) {
  fn setPossibleActions_0(self , rsthis: & QGraphicsSceneDragDropEvent) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN27QGraphicsSceneDragDropEvent18setPossibleActionsE6QFlagsIN2Qt10DropActionEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:273
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::DropAction proposedAction() const

/*

*/
impl /*struct*/ QGraphicsSceneDragDropEvent {
  pub fn proposedAction_0<RetType, T: QGraphicsSceneDragDropEvent_proposedAction_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.proposedAction_0(self);
    // return 1;
  }
}
pub trait QGraphicsSceneDragDropEvent_proposedAction_0<RetType> {
  fn proposedAction_0(self , rsthis: & QGraphicsSceneDragDropEvent) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSceneDragDropEvent_proposedAction_0<i32> for () {
  fn proposedAction_0(self , rsthis: & QGraphicsSceneDragDropEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK27QGraphicsSceneDragDropEvent14proposedActionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:274
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setProposedAction(Qt::DropAction)

/*

*/
impl /*struct*/ QGraphicsSceneDragDropEvent {
  pub fn setProposedAction_0<RetType, T: QGraphicsSceneDragDropEvent_setProposedAction_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setProposedAction_0(self);
    // return 1;
  }
}
pub trait QGraphicsSceneDragDropEvent_setProposedAction_0<RetType> {
  fn setProposedAction_0(self , rsthis: & QGraphicsSceneDragDropEvent) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSceneDragDropEvent_setProposedAction_0<(/*void*/)> for (i32) {
  fn setProposedAction_0(self , rsthis: & QGraphicsSceneDragDropEvent) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN27QGraphicsSceneDragDropEvent17setProposedActionEN2Qt10DropActionE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:275
// index:0
// Public Visibility=Default Availability=Available
// [-2] void acceptProposedAction()

/*

*/
impl /*struct*/ QGraphicsSceneDragDropEvent {
  pub fn acceptProposedAction_0<RetType, T: QGraphicsSceneDragDropEvent_acceptProposedAction_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.acceptProposedAction_0(self);
    // return 1;
  }
}
pub trait QGraphicsSceneDragDropEvent_acceptProposedAction_0<RetType> {
  fn acceptProposedAction_0(self , rsthis: & QGraphicsSceneDragDropEvent) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSceneDragDropEvent_acceptProposedAction_0<(/*void*/)> for () {
  fn acceptProposedAction_0(self , rsthis: & QGraphicsSceneDragDropEvent) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN27QGraphicsSceneDragDropEvent20acceptProposedActionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:277
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::DropAction dropAction() const

/*

*/
impl /*struct*/ QGraphicsSceneDragDropEvent {
  pub fn dropAction_0<RetType, T: QGraphicsSceneDragDropEvent_dropAction_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dropAction_0(self);
    // return 1;
  }
}
pub trait QGraphicsSceneDragDropEvent_dropAction_0<RetType> {
  fn dropAction_0(self , rsthis: & QGraphicsSceneDragDropEvent) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSceneDragDropEvent_dropAction_0<i32> for () {
  fn dropAction_0(self , rsthis: & QGraphicsSceneDragDropEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK27QGraphicsSceneDragDropEvent10dropActionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:278
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDropAction(Qt::DropAction)

/*

*/
impl /*struct*/ QGraphicsSceneDragDropEvent {
  pub fn setDropAction_0<RetType, T: QGraphicsSceneDragDropEvent_setDropAction_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDropAction_0(self);
    // return 1;
  }
}
pub trait QGraphicsSceneDragDropEvent_setDropAction_0<RetType> {
  fn setDropAction_0(self , rsthis: & QGraphicsSceneDragDropEvent) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSceneDragDropEvent_setDropAction_0<(/*void*/)> for (i32) {
  fn setDropAction_0(self , rsthis: & QGraphicsSceneDragDropEvent) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN27QGraphicsSceneDragDropEvent13setDropActionEN2Qt10DropActionE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:280
// index:0
// Public Visibility=Default Availability=Available
// [8] QWidget * source() const

/*

*/
impl /*struct*/ QGraphicsSceneDragDropEvent {
  pub fn source_0<RetType, T: QGraphicsSceneDragDropEvent_source_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.source_0(self);
    // return 1;
  }
}
pub trait QGraphicsSceneDragDropEvent_source_0<RetType> {
  fn source_0(self , rsthis: & QGraphicsSceneDragDropEvent) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSceneDragDropEvent_source_0<usize> for () {
  fn source_0(self , rsthis: & QGraphicsSceneDragDropEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK27QGraphicsSceneDragDropEvent6sourceEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:281
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setSource(QWidget *)

/*

*/
impl /*struct*/ QGraphicsSceneDragDropEvent {
  pub fn setSource_0<RetType, T: QGraphicsSceneDragDropEvent_setSource_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSource_0(self);
    // return 1;
  }
}
pub trait QGraphicsSceneDragDropEvent_setSource_0<RetType> {
  fn setSource_0(self , rsthis: & QGraphicsSceneDragDropEvent) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSceneDragDropEvent_setSource_0<(/*void*/)> for (usize) {
  fn setSource_0(self , rsthis: & QGraphicsSceneDragDropEvent) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN27QGraphicsSceneDragDropEvent9setSourceEP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:283
// index:0
// Public Visibility=Default Availability=Available
// [8] const QMimeData * mimeData() const

/*

*/
impl /*struct*/ QGraphicsSceneDragDropEvent {
  pub fn mimeData_0<RetType, T: QGraphicsSceneDragDropEvent_mimeData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mimeData_0(self);
    // return 1;
  }
}
pub trait QGraphicsSceneDragDropEvent_mimeData_0<RetType> {
  fn mimeData_0(self , rsthis: & QGraphicsSceneDragDropEvent) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSceneDragDropEvent_mimeData_0<usize> for () {
  fn mimeData_0(self , rsthis: & QGraphicsSceneDragDropEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK27QGraphicsSceneDragDropEvent8mimeDataEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:284
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setMimeData(const QMimeData *)

/*

*/
impl /*struct*/ QGraphicsSceneDragDropEvent {
  pub fn setMimeData_0<RetType, T: QGraphicsSceneDragDropEvent_setMimeData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMimeData_0(self);
    // return 1;
  }
}
pub trait QGraphicsSceneDragDropEvent_setMimeData_0<RetType> {
  fn setMimeData_0(self , rsthis: & QGraphicsSceneDragDropEvent) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSceneDragDropEvent_setMimeData_0<(/*void*/)> for (usize) {
  fn setMimeData_0(self , rsthis: & QGraphicsSceneDragDropEvent) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN27QGraphicsSceneDragDropEvent11setMimeDataEPK9QMimeData", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

//  body block end

//  keep block begin

//  keep block end
