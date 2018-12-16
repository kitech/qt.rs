

// mod ::gui::QAccessibleEvent
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
#[derive(Default)] // class sizeof(QAccessibleEvent)=32
pub struct QAccessibleEvent {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QAccessibleEvent_ITF interface {
//    QAccessibleEvent_PTR() *QAccessibleEvent
//}
//func (ptr *QAccessibleEvent) QAccessibleEvent_PTR() *QAccessibleEvent { return ptr }

impl /*struct*/ QAccessibleEvent {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QAccessibleEvent {
    return QAccessibleEvent{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QAccessibleEvent {
//  type Target = QAccessibleEventBASE;
//
//  fn deref(&self) -> &QAccessibleEventBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QAccessibleEventBASE> for QAccessibleEvent {
//  fn as_ref(& self) -> & QAccessibleEventBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qaccessible.h:668
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void QAccessibleEvent(QObject *, QAccessible::Event)

/*

*/
// QAccessibleEvent(QObject *, QAccessible::Event) ctx.fn_proto_cpp
impl /*struct*/ QAccessibleEvent {
  pub fn QAccessibleEvent_0<T: QAccessibleEvent_QAccessibleEvent_0>(value: T) -> QAccessibleEvent {
    let rsthis = value.QAccessibleEvent_0();
    return rsthis;
    // return 1;
  }
}

pub trait QAccessibleEvent_QAccessibleEvent_0 {
  fn QAccessibleEvent_0(self) -> QAccessibleEvent;
}
// QAccessibleEvent(QObject *, QAccessible::Event) ctx.fn_proto_cpp
impl<'a> /*trait*/ QAccessibleEvent_QAccessibleEvent_0 for (usize,i32) {
  fn QAccessibleEvent_0(self) -> QAccessibleEvent {
    // unsafe{_ZN16QAccessibleEventC2EP7QObjectN11QAccessible5EventE()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN16QAccessibleEventC2EP7QObjectN11QAccessible5EventE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QAccessibleEvent{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:684
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void QAccessibleEvent(QAccessibleInterface *, QAccessible::Event)

/*

*/
// QAccessibleEvent(QAccessibleInterface *, QAccessible::Event) ctx.fn_proto_cpp
impl /*struct*/ QAccessibleEvent {
  pub fn QAccessibleEvent_1<T: QAccessibleEvent_QAccessibleEvent_1>(value: T) -> QAccessibleEvent {
    let rsthis = value.QAccessibleEvent_1();
    return rsthis;
    // return 1;
  }
}

pub trait QAccessibleEvent_QAccessibleEvent_1 {
  fn QAccessibleEvent_1(self) -> QAccessibleEvent;
}
// QAccessibleEvent(QAccessibleInterface *, QAccessible::Event) ctx.fn_proto_cpp
impl<'a> /*trait*/ QAccessibleEvent_QAccessibleEvent_1 for (usize,i32) {
  fn QAccessibleEvent_1(self) -> QAccessibleEvent {
    // unsafe{_ZN16QAccessibleEventC2EP20QAccessibleInterfaceN11QAccessible5EventE()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN16QAccessibleEventC2EP20QAccessibleInterfaceN11QAccessible5EventE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QAccessibleEvent{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:699
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QAccessibleEvent()

/*

*/
pub fn DeleteQAccessibleEvent(this :*mut QAccessibleEvent) {
    // let rv = qtrt::InvokeQtFunc6("_ZN16QAccessibleEventD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 32)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qaccessible.h:701
// index:0
// Public inline Visibility=Default Availability=Available
// [4] QAccessible::Event type() const

/*

*/
impl /*struct*/ QAccessibleEvent {
  pub fn type__0<RetType, T: QAccessibleEvent_type__0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.type__0(self);
    // return 1;
  }
}
pub trait QAccessibleEvent_type__0<RetType> {
  fn type__0(self , rsthis: & QAccessibleEvent) -> RetType;
}
impl<'a> /*trait*/ QAccessibleEvent_type__0<i32> for () {
  fn type__0(self , rsthis: & QAccessibleEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QAccessibleEvent4typeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:702
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QObject * object() const

/*

*/
impl /*struct*/ QAccessibleEvent {
  pub fn object_0<RetType, T: QAccessibleEvent_object_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.object_0(self);
    // return 1;
  }
}
pub trait QAccessibleEvent_object_0<RetType> {
  fn object_0(self , rsthis: & QAccessibleEvent) -> RetType;
}
impl<'a> /*trait*/ QAccessibleEvent_object_0<usize> for () {
  fn object_0(self , rsthis: & QAccessibleEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QAccessibleEvent6objectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:703
// index:0
// Public Visibility=Default Availability=Available
// [4] QAccessible::Id uniqueId() const

/*
Returns the unique ID for the QAccessibleInterface iface.
*/
impl /*struct*/ QAccessibleEvent {
  pub fn uniqueId_0<RetType, T: QAccessibleEvent_uniqueId_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.uniqueId_0(self);
    // return 1;
  }
}
pub trait QAccessibleEvent_uniqueId_0<RetType> {
  fn uniqueId_0(self , rsthis: & QAccessibleEvent) -> RetType;
}
impl<'a> /*trait*/ QAccessibleEvent_uniqueId_0<i32> for () {
  fn uniqueId_0(self , rsthis: & QAccessibleEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QAccessibleEvent8uniqueIdEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:705
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setChild(int)

/*

*/
impl /*struct*/ QAccessibleEvent {
  pub fn setChild_0<RetType, T: QAccessibleEvent_setChild_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setChild_0(self);
    // return 1;
  }
}
pub trait QAccessibleEvent_setChild_0<RetType> {
  fn setChild_0(self , rsthis: & QAccessibleEvent) -> RetType;
}
impl<'a> /*trait*/ QAccessibleEvent_setChild_0<(/*void*/)> for (i32) {
  fn setChild_0(self , rsthis: & QAccessibleEvent) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN16QAccessibleEvent8setChildEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:706
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int child() const

/*

*/
impl /*struct*/ QAccessibleEvent {
  pub fn child_0<RetType, T: QAccessibleEvent_child_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.child_0(self);
    // return 1;
  }
}
pub trait QAccessibleEvent_child_0<RetType> {
  fn child_0(self , rsthis: & QAccessibleEvent) -> RetType;
}
impl<'a> /*trait*/ QAccessibleEvent_child_0<i32> for () {
  fn child_0(self , rsthis: & QAccessibleEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QAccessibleEvent5childEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:708
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QAccessibleInterface * accessibleInterface() const

/*
Returns the QAccessibleInterface belonging to the id.

Returns 0 if the id is invalid.
*/
impl /*struct*/ QAccessibleEvent {
  pub fn accessibleInterface_0<RetType, T: QAccessibleEvent_accessibleInterface_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.accessibleInterface_0(self);
    // return 1;
  }
}
pub trait QAccessibleEvent_accessibleInterface_0<RetType> {
  fn accessibleInterface_0(self , rsthis: & QAccessibleEvent) -> RetType;
}
impl<'a> /*trait*/ QAccessibleEvent_accessibleInterface_0<usize> for () {
  fn accessibleInterface_0(self , rsthis: & QAccessibleEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QAccessibleEvent19accessibleInterfaceEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
