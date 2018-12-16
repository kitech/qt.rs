

// mod ::gui::QHideEvent
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
#[derive(Default)] // class sizeof(QHideEvent)=24
pub struct QHideEvent {
  qbase: QEvent,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QHideEvent_ITF interface {
//    qtcore.QEvent_ITF
//    QHideEvent_PTR() *QHideEvent
//}
//func (ptr *QHideEvent) QHideEvent_PTR() *QHideEvent { return ptr }

impl /*struct*/ QHideEvent {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QHideEvent {
    return QHideEvent{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QHideEvent {
//  type Target = QHideEventBASE;
//
//  fn deref(&self) -> &QHideEventBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QHideEventBASE> for QHideEvent {
//  fn as_ref(& self) -> & QHideEventBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qevent.h:501
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QHideEvent()

/*

*/
// QHideEvent() ctx.fn_proto_cpp
impl /*struct*/ QHideEvent {
  pub fn QHideEvent_0<T: QHideEvent_QHideEvent_0>(value: T) -> QHideEvent {
    let rsthis = value.QHideEvent_0();
    return rsthis;
    // return 1;
  }
}

pub trait QHideEvent_QHideEvent_0 {
  fn QHideEvent_0(self) -> QHideEvent;
}
// QHideEvent() ctx.fn_proto_cpp
impl<'a> /*trait*/ QHideEvent_QHideEvent_0 for () {
  fn QHideEvent_0(self) -> QHideEvent {
    // unsafe{_ZN10QHideEventC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN10QHideEventC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QHideEvent{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qevent.h:502
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QHideEvent()

/*

*/
pub fn DeleteQHideEvent(this :*mut QHideEvent) {
    // let rv = qtrt::InvokeQtFunc6("_ZN10QHideEventD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 24)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
