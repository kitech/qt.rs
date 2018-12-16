

// mod ::widgets::QGraphicsSceneWheelEvent
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
// extern C begin: 30
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
#[derive(Default)] // class sizeof(QGraphicsSceneWheelEvent)=32
pub struct QGraphicsSceneWheelEvent {
  qbase: QGraphicsSceneEvent,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QGraphicsSceneWheelEvent_ITF interface {
//    QGraphicsSceneEvent_ITF
//    QGraphicsSceneWheelEvent_PTR() *QGraphicsSceneWheelEvent
//}
//func (ptr *QGraphicsSceneWheelEvent) QGraphicsSceneWheelEvent_PTR() *QGraphicsSceneWheelEvent { return ptr }

impl /*struct*/ QGraphicsSceneWheelEvent {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QGraphicsSceneWheelEvent {
    return QGraphicsSceneWheelEvent{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QGraphicsSceneWheelEvent {
//  type Target = QGraphicsSceneWheelEventBASE;
//
//  fn deref(&self) -> &QGraphicsSceneWheelEventBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QGraphicsSceneWheelEventBASE> for QGraphicsSceneWheelEvent {
//  fn as_ref(& self) -> & QGraphicsSceneWheelEventBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:139
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QGraphicsSceneWheelEvent(QEvent::Type)

/*

*/
// QGraphicsSceneWheelEvent(QEvent::Type) ctx.fn_proto_cpp
impl /*struct*/ QGraphicsSceneWheelEvent {
  pub fn QGraphicsSceneWheelEvent_0<T: QGraphicsSceneWheelEvent_QGraphicsSceneWheelEvent_0>(value: T) -> QGraphicsSceneWheelEvent {
    let rsthis = value.QGraphicsSceneWheelEvent_0();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsSceneWheelEvent_QGraphicsSceneWheelEvent_0 {
  fn QGraphicsSceneWheelEvent_0(self) -> QGraphicsSceneWheelEvent;
}
// QGraphicsSceneWheelEvent(QEvent::Type) ctx.fn_proto_cpp
impl<'a> /*trait*/ QGraphicsSceneWheelEvent_QGraphicsSceneWheelEvent_0 for (i32) {
  fn QGraphicsSceneWheelEvent_0(self) -> QGraphicsSceneWheelEvent {
    // unsafe{_ZN24QGraphicsSceneWheelEventC2EN6QEvent4TypeE()};
    let arg0 = (&self) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN24QGraphicsSceneWheelEventC2EN6QEvent4TypeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QGraphicsSceneWheelEvent{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:140
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QGraphicsSceneWheelEvent()

/*

*/
pub fn DeleteQGraphicsSceneWheelEvent(this :*mut QGraphicsSceneWheelEvent) {
    // let rv = qtrt::InvokeQtFunc6("_ZN24QGraphicsSceneWheelEventD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 32)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:142
// index:0
// Public Visibility=Default Availability=Available
// [16] QPointF pos() const

/*

*/
impl /*struct*/ QGraphicsSceneWheelEvent {
  pub fn pos_0<RetType, T: QGraphicsSceneWheelEvent_pos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.pos_0(self);
    // return 1;
  }
}
pub trait QGraphicsSceneWheelEvent_pos_0<RetType> {
  fn pos_0(self , rsthis: & QGraphicsSceneWheelEvent) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSceneWheelEvent_pos_0<usize> for () {
  fn pos_0(self , rsthis: & QGraphicsSceneWheelEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK24QGraphicsSceneWheelEvent3posEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:143
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setPos(const QPointF &)

/*

*/
impl /*struct*/ QGraphicsSceneWheelEvent {
  pub fn setPos_0<RetType, T: QGraphicsSceneWheelEvent_setPos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPos_0(self);
    // return 1;
  }
}
pub trait QGraphicsSceneWheelEvent_setPos_0<RetType> {
  fn setPos_0(self , rsthis: & QGraphicsSceneWheelEvent) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSceneWheelEvent_setPos_0<(/*void*/)> for (usize) {
  fn setPos_0(self , rsthis: & QGraphicsSceneWheelEvent) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN24QGraphicsSceneWheelEvent6setPosERK7QPointF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:145
// index:0
// Public Visibility=Default Availability=Available
// [16] QPointF scenePos() const

/*

*/
impl /*struct*/ QGraphicsSceneWheelEvent {
  pub fn scenePos_0<RetType, T: QGraphicsSceneWheelEvent_scenePos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.scenePos_0(self);
    // return 1;
  }
}
pub trait QGraphicsSceneWheelEvent_scenePos_0<RetType> {
  fn scenePos_0(self , rsthis: & QGraphicsSceneWheelEvent) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSceneWheelEvent_scenePos_0<usize> for () {
  fn scenePos_0(self , rsthis: & QGraphicsSceneWheelEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK24QGraphicsSceneWheelEvent8scenePosEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:146
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setScenePos(const QPointF &)

/*

*/
impl /*struct*/ QGraphicsSceneWheelEvent {
  pub fn setScenePos_0<RetType, T: QGraphicsSceneWheelEvent_setScenePos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setScenePos_0(self);
    // return 1;
  }
}
pub trait QGraphicsSceneWheelEvent_setScenePos_0<RetType> {
  fn setScenePos_0(self , rsthis: & QGraphicsSceneWheelEvent) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSceneWheelEvent_setScenePos_0<(/*void*/)> for (usize) {
  fn setScenePos_0(self , rsthis: & QGraphicsSceneWheelEvent) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN24QGraphicsSceneWheelEvent11setScenePosERK7QPointF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:148
// index:0
// Public Visibility=Default Availability=Available
// [8] QPoint screenPos() const

/*

*/
impl /*struct*/ QGraphicsSceneWheelEvent {
  pub fn screenPos_0<RetType, T: QGraphicsSceneWheelEvent_screenPos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.screenPos_0(self);
    // return 1;
  }
}
pub trait QGraphicsSceneWheelEvent_screenPos_0<RetType> {
  fn screenPos_0(self , rsthis: & QGraphicsSceneWheelEvent) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSceneWheelEvent_screenPos_0<usize> for () {
  fn screenPos_0(self , rsthis: & QGraphicsSceneWheelEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK24QGraphicsSceneWheelEvent9screenPosEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:149
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setScreenPos(const QPoint &)

/*

*/
impl /*struct*/ QGraphicsSceneWheelEvent {
  pub fn setScreenPos_0<RetType, T: QGraphicsSceneWheelEvent_setScreenPos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setScreenPos_0(self);
    // return 1;
  }
}
pub trait QGraphicsSceneWheelEvent_setScreenPos_0<RetType> {
  fn setScreenPos_0(self , rsthis: & QGraphicsSceneWheelEvent) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSceneWheelEvent_setScreenPos_0<(/*void*/)> for (usize) {
  fn setScreenPos_0(self , rsthis: & QGraphicsSceneWheelEvent) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN24QGraphicsSceneWheelEvent12setScreenPosERK6QPoint", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:151
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::MouseButtons buttons() const

/*

*/
impl /*struct*/ QGraphicsSceneWheelEvent {
  pub fn buttons_0<RetType, T: QGraphicsSceneWheelEvent_buttons_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.buttons_0(self);
    // return 1;
  }
}
pub trait QGraphicsSceneWheelEvent_buttons_0<RetType> {
  fn buttons_0(self , rsthis: & QGraphicsSceneWheelEvent) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSceneWheelEvent_buttons_0<i32> for () {
  fn buttons_0(self , rsthis: & QGraphicsSceneWheelEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK24QGraphicsSceneWheelEvent7buttonsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:152
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setButtons(Qt::MouseButtons)

/*

*/
impl /*struct*/ QGraphicsSceneWheelEvent {
  pub fn setButtons_0<RetType, T: QGraphicsSceneWheelEvent_setButtons_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setButtons_0(self);
    // return 1;
  }
}
pub trait QGraphicsSceneWheelEvent_setButtons_0<RetType> {
  fn setButtons_0(self , rsthis: & QGraphicsSceneWheelEvent) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSceneWheelEvent_setButtons_0<(/*void*/)> for (i32) {
  fn setButtons_0(self , rsthis: & QGraphicsSceneWheelEvent) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN24QGraphicsSceneWheelEvent10setButtonsE6QFlagsIN2Qt11MouseButtonEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:154
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::KeyboardModifiers modifiers() const

/*

*/
impl /*struct*/ QGraphicsSceneWheelEvent {
  pub fn modifiers_0<RetType, T: QGraphicsSceneWheelEvent_modifiers_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.modifiers_0(self);
    // return 1;
  }
}
pub trait QGraphicsSceneWheelEvent_modifiers_0<RetType> {
  fn modifiers_0(self , rsthis: & QGraphicsSceneWheelEvent) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSceneWheelEvent_modifiers_0<i32> for () {
  fn modifiers_0(self , rsthis: & QGraphicsSceneWheelEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK24QGraphicsSceneWheelEvent9modifiersEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:155
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setModifiers(Qt::KeyboardModifiers)

/*

*/
impl /*struct*/ QGraphicsSceneWheelEvent {
  pub fn setModifiers_0<RetType, T: QGraphicsSceneWheelEvent_setModifiers_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setModifiers_0(self);
    // return 1;
  }
}
pub trait QGraphicsSceneWheelEvent_setModifiers_0<RetType> {
  fn setModifiers_0(self , rsthis: & QGraphicsSceneWheelEvent) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSceneWheelEvent_setModifiers_0<(/*void*/)> for (i32) {
  fn setModifiers_0(self , rsthis: & QGraphicsSceneWheelEvent) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN24QGraphicsSceneWheelEvent12setModifiersE6QFlagsIN2Qt16KeyboardModifierEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:157
// index:0
// Public Visibility=Default Availability=Available
// [4] int delta() const

/*

*/
impl /*struct*/ QGraphicsSceneWheelEvent {
  pub fn delta_0<RetType, T: QGraphicsSceneWheelEvent_delta_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.delta_0(self);
    // return 1;
  }
}
pub trait QGraphicsSceneWheelEvent_delta_0<RetType> {
  fn delta_0(self , rsthis: & QGraphicsSceneWheelEvent) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSceneWheelEvent_delta_0<i32> for () {
  fn delta_0(self , rsthis: & QGraphicsSceneWheelEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK24QGraphicsSceneWheelEvent5deltaEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:158
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDelta(int)

/*

*/
impl /*struct*/ QGraphicsSceneWheelEvent {
  pub fn setDelta_0<RetType, T: QGraphicsSceneWheelEvent_setDelta_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDelta_0(self);
    // return 1;
  }
}
pub trait QGraphicsSceneWheelEvent_setDelta_0<RetType> {
  fn setDelta_0(self , rsthis: & QGraphicsSceneWheelEvent) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSceneWheelEvent_setDelta_0<(/*void*/)> for (i32) {
  fn setDelta_0(self , rsthis: & QGraphicsSceneWheelEvent) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN24QGraphicsSceneWheelEvent8setDeltaEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:160
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::Orientation orientation() const

/*

*/
impl /*struct*/ QGraphicsSceneWheelEvent {
  pub fn orientation_0<RetType, T: QGraphicsSceneWheelEvent_orientation_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.orientation_0(self);
    // return 1;
  }
}
pub trait QGraphicsSceneWheelEvent_orientation_0<RetType> {
  fn orientation_0(self , rsthis: & QGraphicsSceneWheelEvent) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSceneWheelEvent_orientation_0<i32> for () {
  fn orientation_0(self , rsthis: & QGraphicsSceneWheelEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK24QGraphicsSceneWheelEvent11orientationEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:161
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setOrientation(Qt::Orientation)

/*

*/
impl /*struct*/ QGraphicsSceneWheelEvent {
  pub fn setOrientation_0<RetType, T: QGraphicsSceneWheelEvent_setOrientation_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setOrientation_0(self);
    // return 1;
  }
}
pub trait QGraphicsSceneWheelEvent_setOrientation_0<RetType> {
  fn setOrientation_0(self , rsthis: & QGraphicsSceneWheelEvent) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSceneWheelEvent_setOrientation_0<(/*void*/)> for (i32) {
  fn setOrientation_0(self , rsthis: & QGraphicsSceneWheelEvent) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN24QGraphicsSceneWheelEvent14setOrientationEN2Qt11OrientationE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

//  body block end

//  keep block begin

//  keep block end
