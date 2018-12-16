

// mod ::gui::QIconDragEvent
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
#[derive(Default)] // class sizeof(QIconDragEvent)=24
pub struct QIconDragEvent {
  qbase: QEvent,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QIconDragEvent_ITF interface {
//    qtcore.QEvent_ITF
//    QIconDragEvent_PTR() *QIconDragEvent
//}
//func (ptr *QIconDragEvent) QIconDragEvent_PTR() *QIconDragEvent { return ptr }

impl /*struct*/ QIconDragEvent {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QIconDragEvent {
    return QIconDragEvent{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QIconDragEvent {
//  type Target = QIconDragEventBASE;
//
//  fn deref(&self) -> &QIconDragEventBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QIconDragEventBASE> for QIconDragEvent {
//  fn as_ref(& self) -> & QIconDragEventBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qevent.h:485
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QIconDragEvent()

/*

*/
// QIconDragEvent() ctx.fn_proto_cpp
impl /*struct*/ QIconDragEvent {
  pub fn QIconDragEvent_0<T: QIconDragEvent_QIconDragEvent_0>(value: T) -> QIconDragEvent {
    let rsthis = value.QIconDragEvent_0();
    return rsthis;
    // return 1;
  }
}

pub trait QIconDragEvent_QIconDragEvent_0 {
  fn QIconDragEvent_0(self) -> QIconDragEvent;
}
// QIconDragEvent() ctx.fn_proto_cpp
impl<'a> /*trait*/ QIconDragEvent_QIconDragEvent_0 for () {
  fn QIconDragEvent_0(self) -> QIconDragEvent {
    // unsafe{_ZN14QIconDragEventC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN14QIconDragEventC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QIconDragEvent{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qevent.h:486
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QIconDragEvent()

/*

*/
pub fn DeleteQIconDragEvent(this :*mut QIconDragEvent) {
    // let rv = qtrt::InvokeQtFunc6("_ZN14QIconDragEventD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 24)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
