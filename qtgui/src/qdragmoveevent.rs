

// mod ::gui::QDragMoveEvent
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
// extern C begin: 13
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
#[derive(Default)] // class sizeof(QDragMoveEvent)=88
pub struct QDragMoveEvent {
  qbase: QDropEvent,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QDragMoveEvent_ITF interface {
//    QDropEvent_ITF
//    QDragMoveEvent_PTR() *QDragMoveEvent
//}
//func (ptr *QDragMoveEvent) QDragMoveEvent_PTR() *QDragMoveEvent { return ptr }

impl /*struct*/ QDragMoveEvent {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QDragMoveEvent {
    return QDragMoveEvent{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QDragMoveEvent {
//  type Target = QDragMoveEventBASE;
//
//  fn deref(&self) -> &QDragMoveEventBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QDragMoveEventBASE> for QDragMoveEvent {
//  fn as_ref(& self) -> & QDragMoveEventBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qevent.h:642
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QDragMoveEvent(const QPoint &, Qt::DropActions, const QMimeData *, Qt::MouseButtons, Qt::KeyboardModifiers, QEvent::Type)

/*

*/
// QDragMoveEvent(const QPoint &, Qt::DropActions, const QMimeData *, Qt::MouseButtons, Qt::KeyboardModifiers, QEvent::Type) ctx.fn_proto_cpp
impl /*struct*/ QDragMoveEvent {
  pub fn QDragMoveEvent_0<T: QDragMoveEvent_QDragMoveEvent_0>(value: T) -> QDragMoveEvent {
    let rsthis = value.QDragMoveEvent_0();
    return rsthis;
    // return 1;
  }
}

pub trait QDragMoveEvent_QDragMoveEvent_0 {
  fn QDragMoveEvent_0(self) -> QDragMoveEvent;
}
// QDragMoveEvent(const QPoint &, Qt::DropActions, const QMimeData *, Qt::MouseButtons, Qt::KeyboardModifiers, QEvent::Type) ctx.fn_proto_cpp
impl<'a> /*trait*/ QDragMoveEvent_QDragMoveEvent_0 for (usize,i32,usize,i32,i32,i32) {
  fn QDragMoveEvent_0(self) -> QDragMoveEvent {
    // unsafe{_ZN14QDragMoveEventC2ERK6QPoint6QFlagsIN2Qt10DropActionEEPK9QMimeDataS3_INS4_11MouseButtonEES3_INS4_16KeyboardModifierEEN6QEvent4TypeE()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let arg4 = (&self.4) as *const i32 as usize;
    let arg5 = (&self.5) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN14QDragMoveEventC2ERK6QPoint6QFlagsIN2Qt10DropActionEEPK9QMimeDataS3_INS4_11MouseButtonEES3_INS4_16KeyboardModifierEEN6QEvent4TypeE", 6,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,arg5,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QDragMoveEvent{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qevent.h:644
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QDragMoveEvent()

/*

*/
pub fn DeleteQDragMoveEvent(this :*mut QDragMoveEvent) {
    // let rv = qtrt::InvokeQtFunc6("_ZN14QDragMoveEventD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 88)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qevent.h:646
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QRect answerRect() const

/*

*/
impl /*struct*/ QDragMoveEvent {
  pub fn answerRect_0<RetType, T: QDragMoveEvent_answerRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.answerRect_0(self);
    // return 1;
  }
}
pub trait QDragMoveEvent_answerRect_0<RetType> {
  fn answerRect_0(self , rsthis: & QDragMoveEvent) -> RetType;
}
impl<'a> /*trait*/ QDragMoveEvent_answerRect_0<usize> for () {
  fn answerRect_0(self , rsthis: & QDragMoveEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QDragMoveEvent10answerRectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:648
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void accept()

/*

*/
impl /*struct*/ QDragMoveEvent {
  pub fn accept_0<RetType, T: QDragMoveEvent_accept_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.accept_0(self);
    // return 1;
  }
}
pub trait QDragMoveEvent_accept_0<RetType> {
  fn accept_0(self , rsthis: & QDragMoveEvent) -> RetType;
}
impl<'a> /*trait*/ QDragMoveEvent_accept_0<(/*void*/)> for () {
  fn accept_0(self , rsthis: & QDragMoveEvent) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN14QDragMoveEvent6acceptEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qevent.h:651
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void accept(const QRect &)

/*

*/
impl /*struct*/ QDragMoveEvent {
  pub fn accept_1<RetType, T: QDragMoveEvent_accept_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.accept_1(self);
    // return 1;
  }
}
pub trait QDragMoveEvent_accept_1<RetType> {
  fn accept_1(self , rsthis: & QDragMoveEvent) -> RetType;
}
impl<'a> /*trait*/ QDragMoveEvent_accept_1<(/*void*/)> for (usize) {
  fn accept_1(self , rsthis: & QDragMoveEvent) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN14QDragMoveEvent6acceptERK5QRect", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qevent.h:649
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void ignore()

/*

*/
impl /*struct*/ QDragMoveEvent {
  pub fn ignore_0<RetType, T: QDragMoveEvent_ignore_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.ignore_0(self);
    // return 1;
  }
}
pub trait QDragMoveEvent_ignore_0<RetType> {
  fn ignore_0(self , rsthis: & QDragMoveEvent) -> RetType;
}
impl<'a> /*trait*/ QDragMoveEvent_ignore_0<(/*void*/)> for () {
  fn ignore_0(self , rsthis: & QDragMoveEvent) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN14QDragMoveEvent6ignoreEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qevent.h:652
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void ignore(const QRect &)

/*

*/
impl /*struct*/ QDragMoveEvent {
  pub fn ignore_1<RetType, T: QDragMoveEvent_ignore_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.ignore_1(self);
    // return 1;
  }
}
pub trait QDragMoveEvent_ignore_1<RetType> {
  fn ignore_1(self , rsthis: & QDragMoveEvent) -> RetType;
}
impl<'a> /*trait*/ QDragMoveEvent_ignore_1<(/*void*/)> for (usize) {
  fn ignore_1(self , rsthis: & QDragMoveEvent) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN14QDragMoveEvent6ignoreERK5QRect", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

//  body block end

//  keep block begin

//  keep block end
