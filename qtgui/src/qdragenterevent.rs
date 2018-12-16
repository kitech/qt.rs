

// mod ::gui::QDragEnterEvent
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
// extern C begin: 7
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
#[derive(Default)] // class sizeof(QDragEnterEvent)=88
pub struct QDragEnterEvent {
  qbase: QDragMoveEvent,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QDragEnterEvent_ITF interface {
//    QDragMoveEvent_ITF
//    QDragEnterEvent_PTR() *QDragEnterEvent
//}
//func (ptr *QDragEnterEvent) QDragEnterEvent_PTR() *QDragEnterEvent { return ptr }

impl /*struct*/ QDragEnterEvent {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QDragEnterEvent {
    return QDragEnterEvent{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QDragEnterEvent {
//  type Target = QDragEnterEventBASE;
//
//  fn deref(&self) -> &QDragEnterEventBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QDragEnterEventBASE> for QDragEnterEvent {
//  fn as_ref(& self) -> & QDragEnterEventBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qevent.h:662
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QDragEnterEvent(const QPoint &, Qt::DropActions, const QMimeData *, Qt::MouseButtons, Qt::KeyboardModifiers)

/*

*/
// QDragEnterEvent(const QPoint &, Qt::DropActions, const QMimeData *, Qt::MouseButtons, Qt::KeyboardModifiers) ctx.fn_proto_cpp
impl /*struct*/ QDragEnterEvent {
  pub fn QDragEnterEvent_0<T: QDragEnterEvent_QDragEnterEvent_0>(value: T) -> QDragEnterEvent {
    let rsthis = value.QDragEnterEvent_0();
    return rsthis;
    // return 1;
  }
}

pub trait QDragEnterEvent_QDragEnterEvent_0 {
  fn QDragEnterEvent_0(self) -> QDragEnterEvent;
}
// QDragEnterEvent(const QPoint &, Qt::DropActions, const QMimeData *, Qt::MouseButtons, Qt::KeyboardModifiers) ctx.fn_proto_cpp
impl<'a> /*trait*/ QDragEnterEvent_QDragEnterEvent_0 for (usize,i32,usize,i32,i32) {
  fn QDragEnterEvent_0(self) -> QDragEnterEvent {
    // unsafe{_ZN15QDragEnterEventC2ERK6QPoint6QFlagsIN2Qt10DropActionEEPK9QMimeDataS3_INS4_11MouseButtonEES3_INS4_16KeyboardModifierEE()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let arg4 = (&self.4) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN15QDragEnterEventC2ERK6QPoint6QFlagsIN2Qt10DropActionEEPK9QMimeDataS3_INS4_11MouseButtonEES3_INS4_16KeyboardModifierEE", 5,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QDragEnterEvent{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qevent.h:664
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QDragEnterEvent()

/*

*/
pub fn DeleteQDragEnterEvent(this :*mut QDragEnterEvent) {
    // let rv = qtrt::InvokeQtFunc6("_ZN15QDragEnterEventD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 88)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
