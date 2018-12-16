

// mod ::gui::QDragLeaveEvent
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
// extern C begin: 2
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
#[derive(Default)] // class sizeof(QDragLeaveEvent)=24
pub struct QDragLeaveEvent {
  qbase: QEvent,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QDragLeaveEvent_ITF interface {
//    qtcore.QEvent_ITF
//    QDragLeaveEvent_PTR() *QDragLeaveEvent
//}
//func (ptr *QDragLeaveEvent) QDragLeaveEvent_PTR() *QDragLeaveEvent { return ptr }

impl /*struct*/ QDragLeaveEvent {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QDragLeaveEvent {
    return QDragLeaveEvent{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QDragLeaveEvent {
//  type Target = QDragLeaveEventBASE;
//
//  fn deref(&self) -> &QDragLeaveEventBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QDragLeaveEventBASE> for QDragLeaveEvent {
//  fn as_ref(& self) -> & QDragLeaveEventBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qevent.h:671
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QDragLeaveEvent()

/*

*/
// QDragLeaveEvent() ctx.fn_proto_cpp
impl /*struct*/ QDragLeaveEvent {
  pub fn QDragLeaveEvent_0<T: QDragLeaveEvent_QDragLeaveEvent_0>(value: T) -> QDragLeaveEvent {
    let rsthis = value.QDragLeaveEvent_0();
    return rsthis;
    // return 1;
  }
}

pub trait QDragLeaveEvent_QDragLeaveEvent_0 {
  fn QDragLeaveEvent_0(self) -> QDragLeaveEvent;
}
// QDragLeaveEvent() ctx.fn_proto_cpp
impl<'a> /*trait*/ QDragLeaveEvent_QDragLeaveEvent_0 for () {
  fn QDragLeaveEvent_0(self) -> QDragLeaveEvent {
    // unsafe{_ZN15QDragLeaveEventC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN15QDragLeaveEventC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QDragLeaveEvent{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qevent.h:672
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QDragLeaveEvent()

/*

*/
pub fn DeleteQDragLeaveEvent(this :*mut QDragLeaveEvent) {
    // let rv = qtrt::InvokeQtFunc6("_ZN15QDragLeaveEventD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 24)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
