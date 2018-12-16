

// mod ::gui::QAccessibleStateChangeEvent
// package qtgui
// /usr/include/qt/QtGui/qaccessible.h
// #include <qaccessible.h>
// #include <QtGui>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 9
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
#[derive(Default)] // class sizeof(QAccessibleStateChangeEvent)=40
pub struct QAccessibleStateChangeEvent {
  qbase: QAccessibleEvent,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QAccessibleStateChangeEvent_ITF interface {
//    QAccessibleEvent_ITF
//    QAccessibleStateChangeEvent_PTR() *QAccessibleStateChangeEvent
//}
//func (ptr *QAccessibleStateChangeEvent) QAccessibleStateChangeEvent_PTR() *QAccessibleStateChangeEvent { return ptr }

impl /*struct*/ QAccessibleStateChangeEvent {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QAccessibleStateChangeEvent {
    return QAccessibleStateChangeEvent{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QAccessibleStateChangeEvent {
//  type Target = QAccessibleStateChangeEventBASE;
//
//  fn deref(&self) -> &QAccessibleStateChangeEventBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QAccessibleStateChangeEventBASE> for QAccessibleStateChangeEvent {
//  fn as_ref(& self) -> & QAccessibleStateChangeEventBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qaccessible.h:733
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QAccessibleStateChangeEvent()

/*

*/
pub fn DeleteQAccessibleStateChangeEvent(this :*mut QAccessibleStateChangeEvent) {
    // let rv = qtrt::InvokeQtFunc6("_ZN27QAccessibleStateChangeEventD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 40)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qaccessible.h:735
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QAccessible::State changedStates() const

/*

*/
impl /*struct*/ QAccessibleStateChangeEvent {
  pub fn changedStates_0<RetType, T: QAccessibleStateChangeEvent_changedStates_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.changedStates_0(self);
    // return 1;
  }
}
pub trait QAccessibleStateChangeEvent_changedStates_0<RetType> {
  fn changedStates_0(self , rsthis: & QAccessibleStateChangeEvent) -> RetType;
}
impl<'a> /*trait*/ QAccessibleStateChangeEvent_changedStates_0<i32> for () {
  fn changedStates_0(self , rsthis: & QAccessibleStateChangeEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK27QAccessibleStateChangeEvent13changedStatesEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
