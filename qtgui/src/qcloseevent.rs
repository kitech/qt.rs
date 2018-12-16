

// mod ::gui::QCloseEvent
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
// extern C begin: 4
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
#[derive(Default)] // class sizeof(QCloseEvent)=24
pub struct QCloseEvent {
  qbase: QEvent,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QCloseEvent_ITF interface {
//    qtcore.QEvent_ITF
//    QCloseEvent_PTR() *QCloseEvent
//}
//func (ptr *QCloseEvent) QCloseEvent_PTR() *QCloseEvent { return ptr }

impl /*struct*/ QCloseEvent {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QCloseEvent {
    return QCloseEvent{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QCloseEvent {
//  type Target = QCloseEventBASE;
//
//  fn deref(&self) -> &QCloseEventBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QCloseEventBASE> for QCloseEvent {
//  fn as_ref(& self) -> & QCloseEventBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qevent.h:477
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QCloseEvent()

/*

*/
// QCloseEvent() ctx.fn_proto_cpp
impl /*struct*/ QCloseEvent {
  pub fn QCloseEvent_0<T: QCloseEvent_QCloseEvent_0>(value: T) -> QCloseEvent {
    let rsthis = value.QCloseEvent_0();
    return rsthis;
    // return 1;
  }
}

pub trait QCloseEvent_QCloseEvent_0 {
  fn QCloseEvent_0(self) -> QCloseEvent;
}
// QCloseEvent() ctx.fn_proto_cpp
impl<'a> /*trait*/ QCloseEvent_QCloseEvent_0 for () {
  fn QCloseEvent_0(self) -> QCloseEvent {
    // unsafe{_ZN11QCloseEventC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QCloseEventC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QCloseEvent{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qevent.h:478
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QCloseEvent()

/*

*/
pub fn DeleteQCloseEvent(this :*mut QCloseEvent) {
    // let rv = qtrt::InvokeQtFunc6("_ZN11QCloseEventD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 24)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
