

// mod ::widgets::QGraphicsSceneContextMenuEvent
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



/*

*/
#[derive(Default)] // class sizeof(QGraphicsSceneContextMenuEvent)=32
pub struct QGraphicsSceneContextMenuEvent {
  qbase: QGraphicsSceneEvent,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QGraphicsSceneContextMenuEvent_ITF interface {
//    QGraphicsSceneEvent_ITF
//    QGraphicsSceneContextMenuEvent_PTR() *QGraphicsSceneContextMenuEvent
//}
//func (ptr *QGraphicsSceneContextMenuEvent) QGraphicsSceneContextMenuEvent_PTR() *QGraphicsSceneContextMenuEvent { return ptr }

impl /*struct*/ QGraphicsSceneContextMenuEvent {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QGraphicsSceneContextMenuEvent {
    return QGraphicsSceneContextMenuEvent{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QGraphicsSceneContextMenuEvent {
//  type Target = QGraphicsSceneContextMenuEventBASE;
//
//  fn deref(&self) -> &QGraphicsSceneContextMenuEventBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QGraphicsSceneContextMenuEventBASE> for QGraphicsSceneContextMenuEvent {
//  fn as_ref(& self) -> & QGraphicsSceneContextMenuEventBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:174
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QGraphicsSceneContextMenuEvent(QEvent::Type)

/*

*/
// QGraphicsSceneContextMenuEvent(QEvent::Type) ctx.fn_proto_cpp
impl /*struct*/ QGraphicsSceneContextMenuEvent {
  pub fn QGraphicsSceneContextMenuEvent_0<T: QGraphicsSceneContextMenuEvent_QGraphicsSceneContextMenuEvent_0>(value: T) -> QGraphicsSceneContextMenuEvent {
    let rsthis = value.QGraphicsSceneContextMenuEvent_0();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsSceneContextMenuEvent_QGraphicsSceneContextMenuEvent_0 {
  fn QGraphicsSceneContextMenuEvent_0(self) -> QGraphicsSceneContextMenuEvent;
}
// QGraphicsSceneContextMenuEvent(QEvent::Type) ctx.fn_proto_cpp
impl<'a> /*trait*/ QGraphicsSceneContextMenuEvent_QGraphicsSceneContextMenuEvent_0 for (i32) {
  fn QGraphicsSceneContextMenuEvent_0(self) -> QGraphicsSceneContextMenuEvent {
    // unsafe{_ZN30QGraphicsSceneContextMenuEventC2EN6QEvent4TypeE()};
    let arg0 = (&self) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN30QGraphicsSceneContextMenuEventC2EN6QEvent4TypeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QGraphicsSceneContextMenuEvent{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:175
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QGraphicsSceneContextMenuEvent()

/*

*/
pub fn DeleteQGraphicsSceneContextMenuEvent(this :*mut QGraphicsSceneContextMenuEvent) {
    // let rv = qtrt::InvokeQtFunc6("_ZN30QGraphicsSceneContextMenuEventD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 32)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:177
// index:0
// Public Visibility=Default Availability=Available
// [16] QPointF pos() const

/*

*/
impl /*struct*/ QGraphicsSceneContextMenuEvent {
  pub fn pos_0<RetType, T: QGraphicsSceneContextMenuEvent_pos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.pos_0(self);
    // return 1;
  }
}
pub trait QGraphicsSceneContextMenuEvent_pos_0<RetType> {
  fn pos_0(self , rsthis: & QGraphicsSceneContextMenuEvent) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSceneContextMenuEvent_pos_0<usize> for () {
  fn pos_0(self , rsthis: & QGraphicsSceneContextMenuEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK30QGraphicsSceneContextMenuEvent3posEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:178
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setPos(const QPointF &)

/*

*/
impl /*struct*/ QGraphicsSceneContextMenuEvent {
  pub fn setPos_0<RetType, T: QGraphicsSceneContextMenuEvent_setPos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPos_0(self);
    // return 1;
  }
}
pub trait QGraphicsSceneContextMenuEvent_setPos_0<RetType> {
  fn setPos_0(self , rsthis: & QGraphicsSceneContextMenuEvent) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSceneContextMenuEvent_setPos_0<(/*void*/)> for (usize) {
  fn setPos_0(self , rsthis: & QGraphicsSceneContextMenuEvent) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN30QGraphicsSceneContextMenuEvent6setPosERK7QPointF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:180
// index:0
// Public Visibility=Default Availability=Available
// [16] QPointF scenePos() const

/*

*/
impl /*struct*/ QGraphicsSceneContextMenuEvent {
  pub fn scenePos_0<RetType, T: QGraphicsSceneContextMenuEvent_scenePos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.scenePos_0(self);
    // return 1;
  }
}
pub trait QGraphicsSceneContextMenuEvent_scenePos_0<RetType> {
  fn scenePos_0(self , rsthis: & QGraphicsSceneContextMenuEvent) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSceneContextMenuEvent_scenePos_0<usize> for () {
  fn scenePos_0(self , rsthis: & QGraphicsSceneContextMenuEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK30QGraphicsSceneContextMenuEvent8scenePosEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:181
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setScenePos(const QPointF &)

/*

*/
impl /*struct*/ QGraphicsSceneContextMenuEvent {
  pub fn setScenePos_0<RetType, T: QGraphicsSceneContextMenuEvent_setScenePos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setScenePos_0(self);
    // return 1;
  }
}
pub trait QGraphicsSceneContextMenuEvent_setScenePos_0<RetType> {
  fn setScenePos_0(self , rsthis: & QGraphicsSceneContextMenuEvent) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSceneContextMenuEvent_setScenePos_0<(/*void*/)> for (usize) {
  fn setScenePos_0(self , rsthis: & QGraphicsSceneContextMenuEvent) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN30QGraphicsSceneContextMenuEvent11setScenePosERK7QPointF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:183
// index:0
// Public Visibility=Default Availability=Available
// [8] QPoint screenPos() const

/*

*/
impl /*struct*/ QGraphicsSceneContextMenuEvent {
  pub fn screenPos_0<RetType, T: QGraphicsSceneContextMenuEvent_screenPos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.screenPos_0(self);
    // return 1;
  }
}
pub trait QGraphicsSceneContextMenuEvent_screenPos_0<RetType> {
  fn screenPos_0(self , rsthis: & QGraphicsSceneContextMenuEvent) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSceneContextMenuEvent_screenPos_0<usize> for () {
  fn screenPos_0(self , rsthis: & QGraphicsSceneContextMenuEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK30QGraphicsSceneContextMenuEvent9screenPosEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:184
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setScreenPos(const QPoint &)

/*

*/
impl /*struct*/ QGraphicsSceneContextMenuEvent {
  pub fn setScreenPos_0<RetType, T: QGraphicsSceneContextMenuEvent_setScreenPos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setScreenPos_0(self);
    // return 1;
  }
}
pub trait QGraphicsSceneContextMenuEvent_setScreenPos_0<RetType> {
  fn setScreenPos_0(self , rsthis: & QGraphicsSceneContextMenuEvent) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSceneContextMenuEvent_setScreenPos_0<(/*void*/)> for (usize) {
  fn setScreenPos_0(self , rsthis: & QGraphicsSceneContextMenuEvent) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN30QGraphicsSceneContextMenuEvent12setScreenPosERK6QPoint", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:186
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::KeyboardModifiers modifiers() const

/*

*/
impl /*struct*/ QGraphicsSceneContextMenuEvent {
  pub fn modifiers_0<RetType, T: QGraphicsSceneContextMenuEvent_modifiers_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.modifiers_0(self);
    // return 1;
  }
}
pub trait QGraphicsSceneContextMenuEvent_modifiers_0<RetType> {
  fn modifiers_0(self , rsthis: & QGraphicsSceneContextMenuEvent) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSceneContextMenuEvent_modifiers_0<i32> for () {
  fn modifiers_0(self , rsthis: & QGraphicsSceneContextMenuEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK30QGraphicsSceneContextMenuEvent9modifiersEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:187
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setModifiers(Qt::KeyboardModifiers)

/*

*/
impl /*struct*/ QGraphicsSceneContextMenuEvent {
  pub fn setModifiers_0<RetType, T: QGraphicsSceneContextMenuEvent_setModifiers_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setModifiers_0(self);
    // return 1;
  }
}
pub trait QGraphicsSceneContextMenuEvent_setModifiers_0<RetType> {
  fn setModifiers_0(self , rsthis: & QGraphicsSceneContextMenuEvent) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSceneContextMenuEvent_setModifiers_0<(/*void*/)> for (i32) {
  fn setModifiers_0(self , rsthis: & QGraphicsSceneContextMenuEvent) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN30QGraphicsSceneContextMenuEvent12setModifiersE6QFlagsIN2Qt16KeyboardModifierEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:189
// index:0
// Public Visibility=Default Availability=Available
// [4] QGraphicsSceneContextMenuEvent::Reason reason() const

/*

*/
impl /*struct*/ QGraphicsSceneContextMenuEvent {
  pub fn reason_0<RetType, T: QGraphicsSceneContextMenuEvent_reason_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.reason_0(self);
    // return 1;
  }
}
pub trait QGraphicsSceneContextMenuEvent_reason_0<RetType> {
  fn reason_0(self , rsthis: & QGraphicsSceneContextMenuEvent) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSceneContextMenuEvent_reason_0<i32> for () {
  fn reason_0(self , rsthis: & QGraphicsSceneContextMenuEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK30QGraphicsSceneContextMenuEvent6reasonEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:190
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setReason(QGraphicsSceneContextMenuEvent::Reason)

/*

*/
impl /*struct*/ QGraphicsSceneContextMenuEvent {
  pub fn setReason_0<RetType, T: QGraphicsSceneContextMenuEvent_setReason_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setReason_0(self);
    // return 1;
  }
}
pub trait QGraphicsSceneContextMenuEvent_setReason_0<RetType> {
  fn setReason_0(self , rsthis: & QGraphicsSceneContextMenuEvent) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSceneContextMenuEvent_setReason_0<(/*void*/)> for (i32) {
  fn setReason_0(self , rsthis: & QGraphicsSceneContextMenuEvent) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN30QGraphicsSceneContextMenuEvent9setReasonENS_6ReasonE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}


/*


*/
pub type QGraphicsSceneContextMenuEvent__Reason = i32;
// 
pub const QGraphicsSceneContextMenuEvent__Mouse :QGraphicsSceneContextMenuEvent__Reason = 0;
// 
pub const QGraphicsSceneContextMenuEvent__Keyboard :QGraphicsSceneContextMenuEvent__Reason = 1;
// 
pub const QGraphicsSceneContextMenuEvent__Other :QGraphicsSceneContextMenuEvent__Reason = 2;
pub fn QGraphicsSceneContextMenuEvent_ReasonItemName(val: i32) ->String {
  match val {
     QGraphicsSceneContextMenuEvent__Mouse => // 0
     {return String::from("Mouse");}
     QGraphicsSceneContextMenuEvent__Keyboard => // 1
     {return String::from("Keyboard");}
     QGraphicsSceneContextMenuEvent__Other => // 2
     {return String::from("Other");}
  _ => {return format!("{}", val);}
}
}
pub fn QGraphicsSceneContextMenuEvent_ReasonItemName_s(val: i32) ->String {
  //var nilthis *QGraphicsSceneContextMenuEvent
  //return nilthis.ReasonItemName(val);
  return QGraphicsSceneContextMenuEvent_ReasonItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
