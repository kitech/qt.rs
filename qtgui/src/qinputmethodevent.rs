

// mod ::gui::QInputMethodEvent
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
// extern C begin: 11
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
#[derive(Default)] // class sizeof(QInputMethodEvent)=56
pub struct QInputMethodEvent {
  qbase: QEvent,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QInputMethodEvent_ITF interface {
//    qtcore.QEvent_ITF
//    QInputMethodEvent_PTR() *QInputMethodEvent
//}
//func (ptr *QInputMethodEvent) QInputMethodEvent_PTR() *QInputMethodEvent { return ptr }

impl /*struct*/ QInputMethodEvent {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QInputMethodEvent {
    return QInputMethodEvent{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QInputMethodEvent {
//  type Target = QInputMethodEventBASE;
//
//  fn deref(&self) -> &QInputMethodEventBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QInputMethodEventBASE> for QInputMethodEvent {
//  fn as_ref(& self) -> & QInputMethodEventBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qevent.h:555
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QInputMethodEvent()

/*

*/
// QInputMethodEvent() ctx.fn_proto_cpp
impl /*struct*/ QInputMethodEvent {
  pub fn QInputMethodEvent_0<T: QInputMethodEvent_QInputMethodEvent_0>(value: T) -> QInputMethodEvent {
    let rsthis = value.QInputMethodEvent_0();
    return rsthis;
    // return 1;
  }
}

pub trait QInputMethodEvent_QInputMethodEvent_0 {
  fn QInputMethodEvent_0(self) -> QInputMethodEvent;
}
// QInputMethodEvent() ctx.fn_proto_cpp
impl<'a> /*trait*/ QInputMethodEvent_QInputMethodEvent_0 for () {
  fn QInputMethodEvent_0(self) -> QInputMethodEvent {
    // unsafe{_ZN17QInputMethodEventC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN17QInputMethodEventC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QInputMethodEvent{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qevent.h:557
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QInputMethodEvent()

/*

*/
pub fn DeleteQInputMethodEvent(this :*mut QInputMethodEvent) {
    // let rv = qtrt::InvokeQtFunc6("_ZN17QInputMethodEventD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 56)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qevent.h:559
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCommitString(const QString &, int, int)

/*

*/
impl /*struct*/ QInputMethodEvent {
  pub fn setCommitString_0<RetType, T: QInputMethodEvent_setCommitString_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCommitString_0(self);
    // return 1;
  }
}
pub trait QInputMethodEvent_setCommitString_0<RetType> {
  fn setCommitString_0(self , rsthis: & QInputMethodEvent) -> RetType;
}
impl<'a> /*trait*/ QInputMethodEvent_setCommitString_0<(/*void*/)> for (usize,i32,i32) {
  fn setCommitString_0(self , rsthis: & QInputMethodEvent) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN17QInputMethodEvent15setCommitStringERK7QStringii", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qevent.h:561
// index:0
// Public inline Visibility=Default Availability=Available
// [8] const QString & preeditString() const

/*

*/
impl /*struct*/ QInputMethodEvent {
  pub fn preeditString_0<RetType, T: QInputMethodEvent_preeditString_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.preeditString_0(self);
    // return 1;
  }
}
pub trait QInputMethodEvent_preeditString_0<RetType> {
  fn preeditString_0(self , rsthis: & QInputMethodEvent) -> RetType;
}
impl<'a> /*trait*/ QInputMethodEvent_preeditString_0<usize> for () {
  fn preeditString_0(self , rsthis: & QInputMethodEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QInputMethodEvent13preeditStringEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:563
// index:0
// Public inline Visibility=Default Availability=Available
// [8] const QString & commitString() const

/*

*/
impl /*struct*/ QInputMethodEvent {
  pub fn commitString_0<RetType, T: QInputMethodEvent_commitString_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.commitString_0(self);
    // return 1;
  }
}
pub trait QInputMethodEvent_commitString_0<RetType> {
  fn commitString_0(self , rsthis: & QInputMethodEvent) -> RetType;
}
impl<'a> /*trait*/ QInputMethodEvent_commitString_0<usize> for () {
  fn commitString_0(self , rsthis: & QInputMethodEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QInputMethodEvent12commitStringEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:564
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int replacementStart() const

/*

*/
impl /*struct*/ QInputMethodEvent {
  pub fn replacementStart_0<RetType, T: QInputMethodEvent_replacementStart_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.replacementStart_0(self);
    // return 1;
  }
}
pub trait QInputMethodEvent_replacementStart_0<RetType> {
  fn replacementStart_0(self , rsthis: & QInputMethodEvent) -> RetType;
}
impl<'a> /*trait*/ QInputMethodEvent_replacementStart_0<i32> for () {
  fn replacementStart_0(self , rsthis: & QInputMethodEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QInputMethodEvent16replacementStartEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:565
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int replacementLength() const

/*

*/
impl /*struct*/ QInputMethodEvent {
  pub fn replacementLength_0<RetType, T: QInputMethodEvent_replacementLength_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.replacementLength_0(self);
    // return 1;
  }
}
pub trait QInputMethodEvent_replacementLength_0<RetType> {
  fn replacementLength_0(self , rsthis: & QInputMethodEvent) -> RetType;
}
impl<'a> /*trait*/ QInputMethodEvent_replacementLength_0<i32> for () {
  fn replacementLength_0(self , rsthis: & QInputMethodEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QInputMethodEvent17replacementLengthEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}


/*


*/
pub type QInputMethodEvent__AttributeType = i32;
// 
pub const QInputMethodEvent__TextFormat :QInputMethodEvent__AttributeType = 0;
// 
pub const QInputMethodEvent__Cursor :QInputMethodEvent__AttributeType = 1;
// 
pub const QInputMethodEvent__Language :QInputMethodEvent__AttributeType = 2;
// 
pub const QInputMethodEvent__Ruby :QInputMethodEvent__AttributeType = 3;
// 
pub const QInputMethodEvent__Selection :QInputMethodEvent__AttributeType = 4;
pub fn QInputMethodEvent_AttributeTypeItemName(val: i32) ->String {
  match val {
     QInputMethodEvent__TextFormat => // 0
     {return String::from("TextFormat");}
     QInputMethodEvent__Cursor => // 1
     {return String::from("Cursor");}
     QInputMethodEvent__Language => // 2
     {return String::from("Language");}
     QInputMethodEvent__Ruby => // 3
     {return String::from("Ruby");}
     QInputMethodEvent__Selection => // 4
     {return String::from("Selection");}
  _ => {return format!("{}", val);}
}
}
pub fn QInputMethodEvent_AttributeTypeItemName_s(val: i32) ->String {
  //var nilthis *QInputMethodEvent
  //return nilthis.AttributeTypeItemName(val);
  return QInputMethodEvent_AttributeTypeItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
