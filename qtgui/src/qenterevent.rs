

// mod ::gui::QEnterEvent
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
#[derive(Default)] // class sizeof(QEnterEvent)=72
pub struct QEnterEvent {
  qbase: QEvent,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QEnterEvent_ITF interface {
//    qtcore.QEvent_ITF
//    QEnterEvent_PTR() *QEnterEvent
//}
//func (ptr *QEnterEvent) QEnterEvent_PTR() *QEnterEvent { return ptr }

impl /*struct*/ QEnterEvent {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QEnterEvent {
    return QEnterEvent{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QEnterEvent {
//  type Target = QEnterEventBASE;
//
//  fn deref(&self) -> &QEnterEventBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QEnterEventBASE> for QEnterEvent {
//  fn as_ref(& self) -> & QEnterEventBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qevent.h:85
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QEnterEvent(const QPointF &, const QPointF &, const QPointF &)

/*

*/
// QEnterEvent(const QPointF &, const QPointF &, const QPointF &) ctx.fn_proto_cpp
impl /*struct*/ QEnterEvent {
  pub fn QEnterEvent_0<T: QEnterEvent_QEnterEvent_0>(value: T) -> QEnterEvent {
    let rsthis = value.QEnterEvent_0();
    return rsthis;
    // return 1;
  }
}

pub trait QEnterEvent_QEnterEvent_0 {
  fn QEnterEvent_0(self) -> QEnterEvent;
}
// QEnterEvent(const QPointF &, const QPointF &, const QPointF &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QEnterEvent_QEnterEvent_0 for (usize,usize,usize) {
  fn QEnterEvent_0(self) -> QEnterEvent {
    // unsafe{_ZN11QEnterEventC2ERK7QPointFS2_S2_()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QEnterEventC2ERK7QPointFS2_S2_", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QEnterEvent{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qevent.h:86
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QEnterEvent()

/*

*/
pub fn DeleteQEnterEvent(this :*mut QEnterEvent) {
    // let rv = qtrt::InvokeQtFunc6("_ZN11QEnterEventD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 72)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qevent.h:89
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QPoint pos() const

/*

*/
impl /*struct*/ QEnterEvent {
  pub fn pos_0<RetType, T: QEnterEvent_pos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.pos_0(self);
    // return 1;
  }
}
pub trait QEnterEvent_pos_0<RetType> {
  fn pos_0(self , rsthis: & QEnterEvent) -> RetType;
}
impl<'a> /*trait*/ QEnterEvent_pos_0<usize> for () {
  fn pos_0(self , rsthis: & QEnterEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QEnterEvent3posEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:90
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QPoint globalPos() const

/*

*/
impl /*struct*/ QEnterEvent {
  pub fn globalPos_0<RetType, T: QEnterEvent_globalPos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.globalPos_0(self);
    // return 1;
  }
}
pub trait QEnterEvent_globalPos_0<RetType> {
  fn globalPos_0(self , rsthis: & QEnterEvent) -> RetType;
}
impl<'a> /*trait*/ QEnterEvent_globalPos_0<usize> for () {
  fn globalPos_0(self , rsthis: & QEnterEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QEnterEvent9globalPosEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:91
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int x() const

/*

*/
impl /*struct*/ QEnterEvent {
  pub fn x_0<RetType, T: QEnterEvent_x_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.x_0(self);
    // return 1;
  }
}
pub trait QEnterEvent_x_0<RetType> {
  fn x_0(self , rsthis: & QEnterEvent) -> RetType;
}
impl<'a> /*trait*/ QEnterEvent_x_0<i32> for () {
  fn x_0(self , rsthis: & QEnterEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QEnterEvent1xEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:92
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int y() const

/*

*/
impl /*struct*/ QEnterEvent {
  pub fn y_0<RetType, T: QEnterEvent_y_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.y_0(self);
    // return 1;
  }
}
pub trait QEnterEvent_y_0<RetType> {
  fn y_0(self , rsthis: & QEnterEvent) -> RetType;
}
impl<'a> /*trait*/ QEnterEvent_y_0<i32> for () {
  fn y_0(self , rsthis: & QEnterEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QEnterEvent1yEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:93
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int globalX() const

/*

*/
impl /*struct*/ QEnterEvent {
  pub fn globalX_0<RetType, T: QEnterEvent_globalX_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.globalX_0(self);
    // return 1;
  }
}
pub trait QEnterEvent_globalX_0<RetType> {
  fn globalX_0(self , rsthis: & QEnterEvent) -> RetType;
}
impl<'a> /*trait*/ QEnterEvent_globalX_0<i32> for () {
  fn globalX_0(self , rsthis: & QEnterEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QEnterEvent7globalXEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:94
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int globalY() const

/*

*/
impl /*struct*/ QEnterEvent {
  pub fn globalY_0<RetType, T: QEnterEvent_globalY_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.globalY_0(self);
    // return 1;
  }
}
pub trait QEnterEvent_globalY_0<RetType> {
  fn globalY_0(self , rsthis: & QEnterEvent) -> RetType;
}
impl<'a> /*trait*/ QEnterEvent_globalY_0<i32> for () {
  fn globalY_0(self , rsthis: & QEnterEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QEnterEvent7globalYEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:96
// index:0
// Public inline Visibility=Default Availability=Available
// [16] const QPointF & localPos() const

/*

*/
impl /*struct*/ QEnterEvent {
  pub fn localPos_0<RetType, T: QEnterEvent_localPos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.localPos_0(self);
    // return 1;
  }
}
pub trait QEnterEvent_localPos_0<RetType> {
  fn localPos_0(self , rsthis: & QEnterEvent) -> RetType;
}
impl<'a> /*trait*/ QEnterEvent_localPos_0<usize> for () {
  fn localPos_0(self , rsthis: & QEnterEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QEnterEvent8localPosEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:97
// index:0
// Public inline Visibility=Default Availability=Available
// [16] const QPointF & windowPos() const

/*

*/
impl /*struct*/ QEnterEvent {
  pub fn windowPos_0<RetType, T: QEnterEvent_windowPos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.windowPos_0(self);
    // return 1;
  }
}
pub trait QEnterEvent_windowPos_0<RetType> {
  fn windowPos_0(self , rsthis: & QEnterEvent) -> RetType;
}
impl<'a> /*trait*/ QEnterEvent_windowPos_0<usize> for () {
  fn windowPos_0(self , rsthis: & QEnterEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QEnterEvent9windowPosEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:98
// index:0
// Public inline Visibility=Default Availability=Available
// [16] const QPointF & screenPos() const

/*

*/
impl /*struct*/ QEnterEvent {
  pub fn screenPos_0<RetType, T: QEnterEvent_screenPos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.screenPos_0(self);
    // return 1;
  }
}
pub trait QEnterEvent_screenPos_0<RetType> {
  fn screenPos_0(self , rsthis: & QEnterEvent) -> RetType;
}
impl<'a> /*trait*/ QEnterEvent_screenPos_0<usize> for () {
  fn screenPos_0(self , rsthis: & QEnterEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QEnterEvent9screenPosEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
