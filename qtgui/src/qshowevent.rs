

// mod ::gui::QShowEvent
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
#[derive(Default)] // class sizeof(QShowEvent)=24
pub struct QShowEvent {
  qbase: QEvent,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QShowEvent_ITF interface {
//    qtcore.QEvent_ITF
//    QShowEvent_PTR() *QShowEvent
//}
//func (ptr *QShowEvent) QShowEvent_PTR() *QShowEvent { return ptr }

impl /*struct*/ QShowEvent {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QShowEvent {
    return QShowEvent{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QShowEvent {
//  type Target = QShowEventBASE;
//
//  fn deref(&self) -> &QShowEventBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QShowEventBASE> for QShowEvent {
//  fn as_ref(& self) -> & QShowEventBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qevent.h:493
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QShowEvent()

/*

*/
// QShowEvent() ctx.fn_proto_cpp
impl /*struct*/ QShowEvent {
  pub fn QShowEvent_0<T: QShowEvent_QShowEvent_0>(value: T) -> QShowEvent {
    let rsthis = value.QShowEvent_0();
    return rsthis;
    // return 1;
  }
}

pub trait QShowEvent_QShowEvent_0 {
  fn QShowEvent_0(self) -> QShowEvent;
}
// QShowEvent() ctx.fn_proto_cpp
impl<'a> /*trait*/ QShowEvent_QShowEvent_0 for () {
  fn QShowEvent_0(self) -> QShowEvent {
    // unsafe{_ZN10QShowEventC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN10QShowEventC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QShowEvent{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qevent.h:494
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QShowEvent()

/*

*/
pub fn DeleteQShowEvent(this :*mut QShowEvent) {
    // let rv = qtrt::InvokeQtFunc6("_ZN10QShowEventD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 24)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
