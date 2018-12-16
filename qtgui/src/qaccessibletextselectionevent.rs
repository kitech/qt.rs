

// mod ::gui::QAccessibleTextSelectionEvent
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
// extern C begin: 5
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
#[derive(Default)] // class sizeof(QAccessibleTextSelectionEvent)=40
pub struct QAccessibleTextSelectionEvent {
  qbase: QAccessibleTextCursorEvent,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QAccessibleTextSelectionEvent_ITF interface {
//    QAccessibleTextCursorEvent_ITF
//    QAccessibleTextSelectionEvent_PTR() *QAccessibleTextSelectionEvent
//}
//func (ptr *QAccessibleTextSelectionEvent) QAccessibleTextSelectionEvent_PTR() *QAccessibleTextSelectionEvent { return ptr }

impl /*struct*/ QAccessibleTextSelectionEvent {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QAccessibleTextSelectionEvent {
    return QAccessibleTextSelectionEvent{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QAccessibleTextSelectionEvent {
//  type Target = QAccessibleTextSelectionEventBASE;
//
//  fn deref(&self) -> &QAccessibleTextSelectionEventBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QAccessibleTextSelectionEventBASE> for QAccessibleTextSelectionEvent {
//  fn as_ref(& self) -> & QAccessibleTextSelectionEventBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qaccessible.h:773
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void QAccessibleTextSelectionEvent(QObject *, int, int)

/*

*/
// QAccessibleTextSelectionEvent(QObject *, int, int) ctx.fn_proto_cpp
impl /*struct*/ QAccessibleTextSelectionEvent {
  pub fn QAccessibleTextSelectionEvent_0<T: QAccessibleTextSelectionEvent_QAccessibleTextSelectionEvent_0>(value: T) -> QAccessibleTextSelectionEvent {
    let rsthis = value.QAccessibleTextSelectionEvent_0();
    return rsthis;
    // return 1;
  }
}

pub trait QAccessibleTextSelectionEvent_QAccessibleTextSelectionEvent_0 {
  fn QAccessibleTextSelectionEvent_0(self) -> QAccessibleTextSelectionEvent;
}
// QAccessibleTextSelectionEvent(QObject *, int, int) ctx.fn_proto_cpp
impl<'a> /*trait*/ QAccessibleTextSelectionEvent_QAccessibleTextSelectionEvent_0 for (usize,i32,i32) {
  fn QAccessibleTextSelectionEvent_0(self) -> QAccessibleTextSelectionEvent {
    // unsafe{_ZN29QAccessibleTextSelectionEventC2EP7QObjectii()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN29QAccessibleTextSelectionEventC2EP7QObjectii", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QAccessibleTextSelectionEvent{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:779
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void QAccessibleTextSelectionEvent(QAccessibleInterface *, int, int)

/*

*/
// QAccessibleTextSelectionEvent(QAccessibleInterface *, int, int) ctx.fn_proto_cpp
impl /*struct*/ QAccessibleTextSelectionEvent {
  pub fn QAccessibleTextSelectionEvent_1<T: QAccessibleTextSelectionEvent_QAccessibleTextSelectionEvent_1>(value: T) -> QAccessibleTextSelectionEvent {
    let rsthis = value.QAccessibleTextSelectionEvent_1();
    return rsthis;
    // return 1;
  }
}

pub trait QAccessibleTextSelectionEvent_QAccessibleTextSelectionEvent_1 {
  fn QAccessibleTextSelectionEvent_1(self) -> QAccessibleTextSelectionEvent;
}
// QAccessibleTextSelectionEvent(QAccessibleInterface *, int, int) ctx.fn_proto_cpp
impl<'a> /*trait*/ QAccessibleTextSelectionEvent_QAccessibleTextSelectionEvent_1 for (usize,i32,i32) {
  fn QAccessibleTextSelectionEvent_1(self) -> QAccessibleTextSelectionEvent {
    // unsafe{_ZN29QAccessibleTextSelectionEventC2EP20QAccessibleInterfaceii()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN29QAccessibleTextSelectionEventC2EP20QAccessibleInterfaceii", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QAccessibleTextSelectionEvent{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:786
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QAccessibleTextSelectionEvent()

/*

*/
pub fn DeleteQAccessibleTextSelectionEvent(this :*mut QAccessibleTextSelectionEvent) {
    // let rv = qtrt::InvokeQtFunc6("_ZN29QAccessibleTextSelectionEventD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 40)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qaccessible.h:788
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setSelection(int, int)

/*

*/
impl /*struct*/ QAccessibleTextSelectionEvent {
  pub fn setSelection_0<RetType, T: QAccessibleTextSelectionEvent_setSelection_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSelection_0(self);
    // return 1;
  }
}
pub trait QAccessibleTextSelectionEvent_setSelection_0<RetType> {
  fn setSelection_0(self , rsthis: & QAccessibleTextSelectionEvent) -> RetType;
}
impl<'a> /*trait*/ QAccessibleTextSelectionEvent_setSelection_0<(/*void*/)> for (i32,i32) {
  fn setSelection_0(self , rsthis: & QAccessibleTextSelectionEvent) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN29QAccessibleTextSelectionEvent12setSelectionEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:793
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int selectionStart() const

/*

*/
impl /*struct*/ QAccessibleTextSelectionEvent {
  pub fn selectionStart_0<RetType, T: QAccessibleTextSelectionEvent_selectionStart_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.selectionStart_0(self);
    // return 1;
  }
}
pub trait QAccessibleTextSelectionEvent_selectionStart_0<RetType> {
  fn selectionStart_0(self , rsthis: & QAccessibleTextSelectionEvent) -> RetType;
}
impl<'a> /*trait*/ QAccessibleTextSelectionEvent_selectionStart_0<i32> for () {
  fn selectionStart_0(self , rsthis: & QAccessibleTextSelectionEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK29QAccessibleTextSelectionEvent14selectionStartEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:794
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int selectionEnd() const

/*

*/
impl /*struct*/ QAccessibleTextSelectionEvent {
  pub fn selectionEnd_0<RetType, T: QAccessibleTextSelectionEvent_selectionEnd_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.selectionEnd_0(self);
    // return 1;
  }
}
pub trait QAccessibleTextSelectionEvent_selectionEnd_0<RetType> {
  fn selectionEnd_0(self , rsthis: & QAccessibleTextSelectionEvent) -> RetType;
}
impl<'a> /*trait*/ QAccessibleTextSelectionEvent_selectionEnd_0<i32> for () {
  fn selectionEnd_0(self , rsthis: & QAccessibleTextSelectionEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK29QAccessibleTextSelectionEvent12selectionEndEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
