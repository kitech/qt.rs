

// mod ::gui::QToolBarChangeEvent
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
// extern C begin: 6
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
#[derive(Default)] // class sizeof(QToolBarChangeEvent)=24
pub struct QToolBarChangeEvent {
  qbase: QEvent,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QToolBarChangeEvent_ITF interface {
//    qtcore.QEvent_ITF
//    QToolBarChangeEvent_PTR() *QToolBarChangeEvent
//}
//func (ptr *QToolBarChangeEvent) QToolBarChangeEvent_PTR() *QToolBarChangeEvent { return ptr }

impl /*struct*/ QToolBarChangeEvent {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QToolBarChangeEvent {
    return QToolBarChangeEvent{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QToolBarChangeEvent {
//  type Target = QToolBarChangeEventBASE;
//
//  fn deref(&self) -> &QToolBarChangeEventBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QToolBarChangeEventBASE> for QToolBarChangeEvent {
//  fn as_ref(& self) -> & QToolBarChangeEventBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qevent.h:754
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QToolBarChangeEvent(bool)

/*

*/
// QToolBarChangeEvent(bool) ctx.fn_proto_cpp
impl /*struct*/ QToolBarChangeEvent {
  pub fn QToolBarChangeEvent_0<T: QToolBarChangeEvent_QToolBarChangeEvent_0>(value: T) -> QToolBarChangeEvent {
    let rsthis = value.QToolBarChangeEvent_0();
    return rsthis;
    // return 1;
  }
}

pub trait QToolBarChangeEvent_QToolBarChangeEvent_0 {
  fn QToolBarChangeEvent_0(self) -> QToolBarChangeEvent;
}
// QToolBarChangeEvent(bool) ctx.fn_proto_cpp
impl<'a> /*trait*/ QToolBarChangeEvent_QToolBarChangeEvent_0 for (bool) {
  fn QToolBarChangeEvent_0(self) -> QToolBarChangeEvent {
    // unsafe{_ZN19QToolBarChangeEventC2Eb()};
    let arg0 = (&self) as *const bool as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN19QToolBarChangeEventC2Eb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QToolBarChangeEvent{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qevent.h:755
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QToolBarChangeEvent()

/*

*/
pub fn DeleteQToolBarChangeEvent(this :*mut QToolBarChangeEvent) {
    // let rv = qtrt::InvokeQtFunc6("_ZN19QToolBarChangeEventD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 24)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qevent.h:757
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool toggle() const

/*

*/
impl /*struct*/ QToolBarChangeEvent {
  pub fn toggle_0<RetType, T: QToolBarChangeEvent_toggle_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toggle_0(self);
    // return 1;
  }
}
pub trait QToolBarChangeEvent_toggle_0<RetType> {
  fn toggle_0(self , rsthis: & QToolBarChangeEvent) -> RetType;
}
impl<'a> /*trait*/ QToolBarChangeEvent_toggle_0<bool> for () {
  fn toggle_0(self , rsthis: & QToolBarChangeEvent) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QToolBarChangeEvent6toggleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
