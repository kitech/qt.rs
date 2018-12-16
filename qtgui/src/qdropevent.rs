

// mod ::gui::QDropEvent
// package qtgui
// /usr/include/qt/QtGui/qevent.h
// #include <qevent.h>
// #include <QtGui>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 5
*/
// import "C"
// import "unsafe"
// import "reflect"
// import "fmt"
// import "log"
// import "github.com/kitech/qt.go/qtrt"
// import "github.com/kitech/qt.go/qtcore"
use qtcore::*; // super::super::%!s(MISSING)::*;
use std::default::Default;
use std::ops::Deref;
use qtrt; // super::super::qtrt;
use super::*;
//  ext block end

//  body block begin



/*

*/
#[derive(Default)] // class sizeof(QDropEvent)=72
pub struct QDropEvent {
  qbase: QEvent,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QDropEvent_ITF interface {
//    qtcore.QEvent_ITF
//    QDropEvent_PTR() *QDropEvent
//}
//func (ptr *QDropEvent) QDropEvent_PTR() *QDropEvent { return ptr }

impl /*struct*/ QDropEvent {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QDropEvent {
    return QDropEvent{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QDropEvent {
//  type Target = QDropEventBASE;
//
//  fn deref(&self) -> &QDropEventBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QDropEventBASE> for QDropEvent {
//  fn as_ref(& self) -> & QDropEventBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qevent.h:608
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QDropEvent(const QPointF &, Qt::DropActions, const QMimeData *, Qt::MouseButtons, Qt::KeyboardModifiers, QEvent::Type)

/*

*/
// QDropEvent(const QPointF &, Qt::DropActions, const QMimeData *, Qt::MouseButtons, Qt::KeyboardModifiers, QEvent::Type) ctx.fn_proto_cpp
impl /*struct*/ QDropEvent {
  pub fn QDropEvent_0<T: QDropEvent_QDropEvent_0>(value: T) -> QDropEvent {
    let rsthis = value.QDropEvent_0();
    return rsthis;
    // return 1;
  }
}

pub trait QDropEvent_QDropEvent_0 {
  fn QDropEvent_0(self) -> QDropEvent;
}
// QDropEvent(const QPointF &, Qt::DropActions, const QMimeData *, Qt::MouseButtons, Qt::KeyboardModifiers, QEvent::Type) ctx.fn_proto_cpp
impl<'a> /*trait*/ QDropEvent_QDropEvent_0 for (usize,i32,usize,i32,i32,i32) {
  fn QDropEvent_0(self) -> QDropEvent {
    // unsafe{_ZN10QDropEventC2ERK7QPointF6QFlagsIN2Qt10DropActionEEPK9QMimeDataS3_INS4_11MouseButtonEES3_INS4_16KeyboardModifierEEN6QEvent4TypeE()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let arg4 = (&self.4) as *const i32 as usize;
    let arg5 = (&self.5) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN10QDropEventC2ERK7QPointF6QFlagsIN2Qt10DropActionEEPK9QMimeDataS3_INS4_11MouseButtonEES3_INS4_16KeyboardModifierEEN6QEvent4TypeE", 6,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,arg5,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QDropEvent{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qevent.h:610
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QDropEvent()

/*

*/
pub fn DeleteQDropEvent(this :*mut QDropEvent) {
    // let rv = qtrt::InvokeQtFunc6("_ZN10QDropEventD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 72)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qevent.h:612
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QPoint pos() const

/*

*/
impl /*struct*/ QDropEvent {
  pub fn pos_0<RetType, T: QDropEvent_pos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.pos_0(self);
    // return 1;
  }
}
pub trait QDropEvent_pos_0<RetType> {
  fn pos_0(self , rsthis: & QDropEvent) -> RetType;
}
impl<'a> /*trait*/ QDropEvent_pos_0<usize> for () {
  fn pos_0(self , rsthis: & QDropEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QDropEvent3posEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:613
// index:0
// Public inline Visibility=Default Availability=Available
// [16] const QPointF & posF() const

/*

*/
impl /*struct*/ QDropEvent {
  pub fn posF_0<RetType, T: QDropEvent_posF_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.posF_0(self);
    // return 1;
  }
}
pub trait QDropEvent_posF_0<RetType> {
  fn posF_0(self , rsthis: & QDropEvent) -> RetType;
}
impl<'a> /*trait*/ QDropEvent_posF_0<usize> for () {
  fn posF_0(self , rsthis: & QDropEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QDropEvent4posFEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:614
// index:0
// Public inline Visibility=Default Availability=Available
// [4] Qt::MouseButtons mouseButtons() const

/*

*/
impl /*struct*/ QDropEvent {
  pub fn mouseButtons_0<RetType, T: QDropEvent_mouseButtons_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mouseButtons_0(self);
    // return 1;
  }
}
pub trait QDropEvent_mouseButtons_0<RetType> {
  fn mouseButtons_0(self , rsthis: & QDropEvent) -> RetType;
}
impl<'a> /*trait*/ QDropEvent_mouseButtons_0<i32> for () {
  fn mouseButtons_0(self , rsthis: & QDropEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QDropEvent12mouseButtonsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:615
// index:0
// Public inline Visibility=Default Availability=Available
// [4] Qt::KeyboardModifiers keyboardModifiers() const

/*

*/
impl /*struct*/ QDropEvent {
  pub fn keyboardModifiers_0<RetType, T: QDropEvent_keyboardModifiers_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.keyboardModifiers_0(self);
    // return 1;
  }
}
pub trait QDropEvent_keyboardModifiers_0<RetType> {
  fn keyboardModifiers_0(self , rsthis: & QDropEvent) -> RetType;
}
impl<'a> /*trait*/ QDropEvent_keyboardModifiers_0<i32> for () {
  fn keyboardModifiers_0(self , rsthis: & QDropEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QDropEvent17keyboardModifiersEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:617
// index:0
// Public inline Visibility=Default Availability=Available
// [4] Qt::DropActions possibleActions() const

/*

*/
impl /*struct*/ QDropEvent {
  pub fn possibleActions_0<RetType, T: QDropEvent_possibleActions_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.possibleActions_0(self);
    // return 1;
  }
}
pub trait QDropEvent_possibleActions_0<RetType> {
  fn possibleActions_0(self , rsthis: & QDropEvent) -> RetType;
}
impl<'a> /*trait*/ QDropEvent_possibleActions_0<i32> for () {
  fn possibleActions_0(self , rsthis: & QDropEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QDropEvent15possibleActionsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:618
// index:0
// Public inline Visibility=Default Availability=Available
// [4] Qt::DropAction proposedAction() const

/*

*/
impl /*struct*/ QDropEvent {
  pub fn proposedAction_0<RetType, T: QDropEvent_proposedAction_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.proposedAction_0(self);
    // return 1;
  }
}
pub trait QDropEvent_proposedAction_0<RetType> {
  fn proposedAction_0(self , rsthis: & QDropEvent) -> RetType;
}
impl<'a> /*trait*/ QDropEvent_proposedAction_0<i32> for () {
  fn proposedAction_0(self , rsthis: & QDropEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QDropEvent14proposedActionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:619
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void acceptProposedAction()

/*

*/
impl /*struct*/ QDropEvent {
  pub fn acceptProposedAction_0<RetType, T: QDropEvent_acceptProposedAction_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.acceptProposedAction_0(self);
    // return 1;
  }
}
pub trait QDropEvent_acceptProposedAction_0<RetType> {
  fn acceptProposedAction_0(self , rsthis: & QDropEvent) -> RetType;
}
impl<'a> /*trait*/ QDropEvent_acceptProposedAction_0<(/*void*/)> for () {
  fn acceptProposedAction_0(self , rsthis: & QDropEvent) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN10QDropEvent20acceptProposedActionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qevent.h:621
// index:0
// Public inline Visibility=Default Availability=Available
// [4] Qt::DropAction dropAction() const

/*

*/
impl /*struct*/ QDropEvent {
  pub fn dropAction_0<RetType, T: QDropEvent_dropAction_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dropAction_0(self);
    // return 1;
  }
}
pub trait QDropEvent_dropAction_0<RetType> {
  fn dropAction_0(self , rsthis: & QDropEvent) -> RetType;
}
impl<'a> /*trait*/ QDropEvent_dropAction_0<i32> for () {
  fn dropAction_0(self , rsthis: & QDropEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QDropEvent10dropActionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:622
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDropAction(Qt::DropAction)

/*

*/
impl /*struct*/ QDropEvent {
  pub fn setDropAction_0<RetType, T: QDropEvent_setDropAction_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDropAction_0(self);
    // return 1;
  }
}
pub trait QDropEvent_setDropAction_0<RetType> {
  fn setDropAction_0(self , rsthis: & QDropEvent) -> RetType;
}
impl<'a> /*trait*/ QDropEvent_setDropAction_0<(/*void*/)> for (i32) {
  fn setDropAction_0(self , rsthis: & QDropEvent) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QDropEvent13setDropActionEN2Qt10DropActionE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qevent.h:624
// index:0
// Public Visibility=Default Availability=Available
// [8] QObject * source() const

/*

*/
impl /*struct*/ QDropEvent {
  pub fn source_0<RetType, T: QDropEvent_source_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.source_0(self);
    // return 1;
  }
}
pub trait QDropEvent_source_0<RetType> {
  fn source_0(self , rsthis: & QDropEvent) -> RetType;
}
impl<'a> /*trait*/ QDropEvent_source_0<usize> for () {
  fn source_0(self , rsthis: & QDropEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QDropEvent6sourceEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:625
// index:0
// Public inline Visibility=Default Availability=Available
// [8] const QMimeData * mimeData() const

/*

*/
impl /*struct*/ QDropEvent {
  pub fn mimeData_0<RetType, T: QDropEvent_mimeData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mimeData_0(self);
    // return 1;
  }
}
pub trait QDropEvent_mimeData_0<RetType> {
  fn mimeData_0(self , rsthis: & QDropEvent) -> RetType;
}
impl<'a> /*trait*/ QDropEvent_mimeData_0<usize> for () {
  fn mimeData_0(self , rsthis: & QDropEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QDropEvent8mimeDataEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
