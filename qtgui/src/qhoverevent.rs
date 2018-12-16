

// mod ::gui::QHoverEvent
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
// extern C begin: 19
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
#[derive(Default)] // class sizeof(QHoverEvent)=64
pub struct QHoverEvent {
  qbase: QInputEvent,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QHoverEvent_ITF interface {
//    QInputEvent_ITF
//    QHoverEvent_PTR() *QHoverEvent
//}
//func (ptr *QHoverEvent) QHoverEvent_PTR() *QHoverEvent { return ptr }

impl /*struct*/ QHoverEvent {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QHoverEvent {
    return QHoverEvent{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QHoverEvent {
//  type Target = QHoverEventBASE;
//
//  fn deref(&self) -> &QHoverEventBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QHoverEventBASE> for QHoverEvent {
//  fn as_ref(& self) -> & QHoverEventBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qevent.h:157
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QHoverEvent(QEvent::Type, const QPointF &, const QPointF &, Qt::KeyboardModifiers)

/*

*/
// QHoverEvent(QEvent::Type, const QPointF &, const QPointF &, Qt::KeyboardModifiers) ctx.fn_proto_cpp
impl /*struct*/ QHoverEvent {
  pub fn QHoverEvent_0<T: QHoverEvent_QHoverEvent_0>(value: T) -> QHoverEvent {
    let rsthis = value.QHoverEvent_0();
    return rsthis;
    // return 1;
  }
}

pub trait QHoverEvent_QHoverEvent_0 {
  fn QHoverEvent_0(self) -> QHoverEvent;
}
// QHoverEvent(QEvent::Type, const QPointF &, const QPointF &, Qt::KeyboardModifiers) ctx.fn_proto_cpp
impl<'a> /*trait*/ QHoverEvent_QHoverEvent_0 for (i32,usize,usize,i32) {
  fn QHoverEvent_0(self) -> QHoverEvent {
    // unsafe{_ZN11QHoverEventC2EN6QEvent4TypeERK7QPointFS4_6QFlagsIN2Qt16KeyboardModifierEE()};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QHoverEventC2EN6QEvent4TypeERK7QPointFS4_6QFlagsIN2Qt16KeyboardModifierEE", 4,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QHoverEvent{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qevent.h:158
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QHoverEvent()

/*

*/
pub fn DeleteQHoverEvent(this :*mut QHoverEvent) {
    // let rv = qtrt::InvokeQtFunc6("_ZN11QHoverEventD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 64)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qevent.h:161
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QPoint pos() const

/*

*/
impl /*struct*/ QHoverEvent {
  pub fn pos_0<RetType, T: QHoverEvent_pos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.pos_0(self);
    // return 1;
  }
}
pub trait QHoverEvent_pos_0<RetType> {
  fn pos_0(self , rsthis: & QHoverEvent) -> RetType;
}
impl<'a> /*trait*/ QHoverEvent_pos_0<usize> for () {
  fn pos_0(self , rsthis: & QHoverEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QHoverEvent3posEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:162
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QPoint oldPos() const

/*

*/
impl /*struct*/ QHoverEvent {
  pub fn oldPos_0<RetType, T: QHoverEvent_oldPos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.oldPos_0(self);
    // return 1;
  }
}
pub trait QHoverEvent_oldPos_0<RetType> {
  fn oldPos_0(self , rsthis: & QHoverEvent) -> RetType;
}
impl<'a> /*trait*/ QHoverEvent_oldPos_0<usize> for () {
  fn oldPos_0(self , rsthis: & QHoverEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QHoverEvent6oldPosEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:165
// index:0
// Public inline Visibility=Default Availability=Available
// [16] const QPointF & posF() const

/*

*/
impl /*struct*/ QHoverEvent {
  pub fn posF_0<RetType, T: QHoverEvent_posF_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.posF_0(self);
    // return 1;
  }
}
pub trait QHoverEvent_posF_0<RetType> {
  fn posF_0(self , rsthis: & QHoverEvent) -> RetType;
}
impl<'a> /*trait*/ QHoverEvent_posF_0<usize> for () {
  fn posF_0(self , rsthis: & QHoverEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QHoverEvent4posFEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:166
// index:0
// Public inline Visibility=Default Availability=Available
// [16] const QPointF & oldPosF() const

/*

*/
impl /*struct*/ QHoverEvent {
  pub fn oldPosF_0<RetType, T: QHoverEvent_oldPosF_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.oldPosF_0(self);
    // return 1;
  }
}
pub trait QHoverEvent_oldPosF_0<RetType> {
  fn oldPosF_0(self , rsthis: & QHoverEvent) -> RetType;
}
impl<'a> /*trait*/ QHoverEvent_oldPosF_0<usize> for () {
  fn oldPosF_0(self , rsthis: & QHoverEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QHoverEvent7oldPosFEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
