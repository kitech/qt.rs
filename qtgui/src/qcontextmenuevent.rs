

// mod ::gui::QContextMenuEvent
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
#[derive(Default)] // class sizeof(QContextMenuEvent)=56
pub struct QContextMenuEvent {
  qbase: QInputEvent,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QContextMenuEvent_ITF interface {
//    QInputEvent_ITF
//    QContextMenuEvent_PTR() *QContextMenuEvent
//}
//func (ptr *QContextMenuEvent) QContextMenuEvent_PTR() *QContextMenuEvent { return ptr }

impl /*struct*/ QContextMenuEvent {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QContextMenuEvent {
    return QContextMenuEvent{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QContextMenuEvent {
//  type Target = QContextMenuEventBASE;
//
//  fn deref(&self) -> &QContextMenuEventBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QContextMenuEventBASE> for QContextMenuEvent {
//  fn as_ref(& self) -> & QContextMenuEventBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qevent.h:511
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QContextMenuEvent(QContextMenuEvent::Reason, const QPoint &, const QPoint &, Qt::KeyboardModifiers)

/*

*/
// QContextMenuEvent(QContextMenuEvent::Reason, const QPoint &, const QPoint &, Qt::KeyboardModifiers) ctx.fn_proto_cpp
impl /*struct*/ QContextMenuEvent {
  pub fn QContextMenuEvent_0<T: QContextMenuEvent_QContextMenuEvent_0>(value: T) -> QContextMenuEvent {
    let rsthis = value.QContextMenuEvent_0();
    return rsthis;
    // return 1;
  }
}

pub trait QContextMenuEvent_QContextMenuEvent_0 {
  fn QContextMenuEvent_0(self) -> QContextMenuEvent;
}
// QContextMenuEvent(QContextMenuEvent::Reason, const QPoint &, const QPoint &, Qt::KeyboardModifiers) ctx.fn_proto_cpp
impl<'a> /*trait*/ QContextMenuEvent_QContextMenuEvent_0 for (i32,usize,usize,i32) {
  fn QContextMenuEvent_0(self) -> QContextMenuEvent {
    // unsafe{_ZN17QContextMenuEventC2ENS_6ReasonERK6QPointS3_6QFlagsIN2Qt16KeyboardModifierEE()};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN17QContextMenuEventC2ENS_6ReasonERK6QPointS3_6QFlagsIN2Qt16KeyboardModifierEE", 4,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QContextMenuEvent{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qevent.h:513
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QContextMenuEvent(QContextMenuEvent::Reason, const QPoint &, const QPoint &)

/*

*/
// QContextMenuEvent(QContextMenuEvent::Reason, const QPoint &, const QPoint &) ctx.fn_proto_cpp
impl /*struct*/ QContextMenuEvent {
  pub fn QContextMenuEvent_1<T: QContextMenuEvent_QContextMenuEvent_1>(value: T) -> QContextMenuEvent {
    let rsthis = value.QContextMenuEvent_1();
    return rsthis;
    // return 1;
  }
}

pub trait QContextMenuEvent_QContextMenuEvent_1 {
  fn QContextMenuEvent_1(self) -> QContextMenuEvent;
}
// QContextMenuEvent(QContextMenuEvent::Reason, const QPoint &, const QPoint &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QContextMenuEvent_QContextMenuEvent_1 for (i32,usize,usize) {
  fn QContextMenuEvent_1(self) -> QContextMenuEvent {
    // unsafe{_ZN17QContextMenuEventC2ENS_6ReasonERK6QPointS3_()};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN17QContextMenuEventC2ENS_6ReasonERK6QPointS3_", 3,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QContextMenuEvent{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qevent.h:514
// index:2
// Public Visibility=Default Availability=Available
// [-2] void QContextMenuEvent(QContextMenuEvent::Reason, const QPoint &)

/*

*/
// QContextMenuEvent(QContextMenuEvent::Reason, const QPoint &) ctx.fn_proto_cpp
impl /*struct*/ QContextMenuEvent {
  pub fn QContextMenuEvent_2<T: QContextMenuEvent_QContextMenuEvent_2>(value: T) -> QContextMenuEvent {
    let rsthis = value.QContextMenuEvent_2();
    return rsthis;
    // return 1;
  }
}

pub trait QContextMenuEvent_QContextMenuEvent_2 {
  fn QContextMenuEvent_2(self) -> QContextMenuEvent;
}
// QContextMenuEvent(QContextMenuEvent::Reason, const QPoint &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QContextMenuEvent_QContextMenuEvent_2 for (i32,usize) {
  fn QContextMenuEvent_2(self) -> QContextMenuEvent {
    // unsafe{_ZN17QContextMenuEventC2ENS_6ReasonERK6QPoint()};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN17QContextMenuEventC2ENS_6ReasonERK6QPoint", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QContextMenuEvent{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qevent.h:515
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QContextMenuEvent()

/*

*/
pub fn DeleteQContextMenuEvent(this :*mut QContextMenuEvent) {
    // let rv = qtrt::InvokeQtFunc6("_ZN17QContextMenuEventD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 56)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qevent.h:517
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int x() const

/*

*/
impl /*struct*/ QContextMenuEvent {
  pub fn x_0<RetType, T: QContextMenuEvent_x_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.x_0(self);
    // return 1;
  }
}
pub trait QContextMenuEvent_x_0<RetType> {
  fn x_0(self , rsthis: & QContextMenuEvent) -> RetType;
}
impl<'a> /*trait*/ QContextMenuEvent_x_0<i32> for () {
  fn x_0(self , rsthis: & QContextMenuEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QContextMenuEvent1xEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:518
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int y() const

/*

*/
impl /*struct*/ QContextMenuEvent {
  pub fn y_0<RetType, T: QContextMenuEvent_y_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.y_0(self);
    // return 1;
  }
}
pub trait QContextMenuEvent_y_0<RetType> {
  fn y_0(self , rsthis: & QContextMenuEvent) -> RetType;
}
impl<'a> /*trait*/ QContextMenuEvent_y_0<i32> for () {
  fn y_0(self , rsthis: & QContextMenuEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QContextMenuEvent1yEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:519
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int globalX() const

/*

*/
impl /*struct*/ QContextMenuEvent {
  pub fn globalX_0<RetType, T: QContextMenuEvent_globalX_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.globalX_0(self);
    // return 1;
  }
}
pub trait QContextMenuEvent_globalX_0<RetType> {
  fn globalX_0(self , rsthis: & QContextMenuEvent) -> RetType;
}
impl<'a> /*trait*/ QContextMenuEvent_globalX_0<i32> for () {
  fn globalX_0(self , rsthis: & QContextMenuEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QContextMenuEvent7globalXEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:520
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int globalY() const

/*

*/
impl /*struct*/ QContextMenuEvent {
  pub fn globalY_0<RetType, T: QContextMenuEvent_globalY_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.globalY_0(self);
    // return 1;
  }
}
pub trait QContextMenuEvent_globalY_0<RetType> {
  fn globalY_0(self , rsthis: & QContextMenuEvent) -> RetType;
}
impl<'a> /*trait*/ QContextMenuEvent_globalY_0<i32> for () {
  fn globalY_0(self , rsthis: & QContextMenuEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QContextMenuEvent7globalYEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:522
// index:0
// Public inline Visibility=Default Availability=Available
// [8] const QPoint & pos() const

/*

*/
impl /*struct*/ QContextMenuEvent {
  pub fn pos_0<RetType, T: QContextMenuEvent_pos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.pos_0(self);
    // return 1;
  }
}
pub trait QContextMenuEvent_pos_0<RetType> {
  fn pos_0(self , rsthis: & QContextMenuEvent) -> RetType;
}
impl<'a> /*trait*/ QContextMenuEvent_pos_0<usize> for () {
  fn pos_0(self , rsthis: & QContextMenuEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QContextMenuEvent3posEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:523
// index:0
// Public inline Visibility=Default Availability=Available
// [8] const QPoint & globalPos() const

/*

*/
impl /*struct*/ QContextMenuEvent {
  pub fn globalPos_0<RetType, T: QContextMenuEvent_globalPos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.globalPos_0(self);
    // return 1;
  }
}
pub trait QContextMenuEvent_globalPos_0<RetType> {
  fn globalPos_0(self , rsthis: & QContextMenuEvent) -> RetType;
}
impl<'a> /*trait*/ QContextMenuEvent_globalPos_0<usize> for () {
  fn globalPos_0(self , rsthis: & QContextMenuEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QContextMenuEvent9globalPosEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:525
// index:0
// Public inline Visibility=Default Availability=Available
// [4] QContextMenuEvent::Reason reason() const

/*

*/
impl /*struct*/ QContextMenuEvent {
  pub fn reason_0<RetType, T: QContextMenuEvent_reason_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.reason_0(self);
    // return 1;
  }
}
pub trait QContextMenuEvent_reason_0<RetType> {
  fn reason_0(self , rsthis: & QContextMenuEvent) -> RetType;
}
impl<'a> /*trait*/ QContextMenuEvent_reason_0<i32> for () {
  fn reason_0(self , rsthis: & QContextMenuEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QContextMenuEvent6reasonEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}


/*


*/
pub type QContextMenuEvent__Reason = i32;
// 
pub const QContextMenuEvent__Mouse :QContextMenuEvent__Reason = 0;
// 
pub const QContextMenuEvent__Keyboard :QContextMenuEvent__Reason = 1;
// 
pub const QContextMenuEvent__Other :QContextMenuEvent__Reason = 2;
pub fn QContextMenuEvent_ReasonItemName(val: i32) ->String {
  match val {
     QContextMenuEvent__Mouse => // 0
     {return String::from("Mouse");}
     QContextMenuEvent__Keyboard => // 1
     {return String::from("Keyboard");}
     QContextMenuEvent__Other => // 2
     {return String::from("Other");}
  _ => {return format!("{}", val);}
}
}
pub fn QContextMenuEvent_ReasonItemName_s(val: i32) ->String {
  //var nilthis *QContextMenuEvent
  //return nilthis.ReasonItemName(val);
  return QContextMenuEvent_ReasonItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
