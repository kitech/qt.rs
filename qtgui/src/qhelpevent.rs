

// mod ::gui::QHelpEvent
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
#[derive(Default)] // class sizeof(QHelpEvent)=40
pub struct QHelpEvent {
  qbase: QEvent,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QHelpEvent_ITF interface {
//    qtcore.QEvent_ITF
//    QHelpEvent_PTR() *QHelpEvent
//}
//func (ptr *QHelpEvent) QHelpEvent_PTR() *QHelpEvent { return ptr }

impl /*struct*/ QHelpEvent {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QHelpEvent {
    return QHelpEvent{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QHelpEvent {
//  type Target = QHelpEventBASE;
//
//  fn deref(&self) -> &QHelpEventBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QHelpEventBASE> for QHelpEvent {
//  fn as_ref(& self) -> & QHelpEventBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qevent.h:680
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QHelpEvent(QEvent::Type, const QPoint &, const QPoint &)

/*

*/
// QHelpEvent(QEvent::Type, const QPoint &, const QPoint &) ctx.fn_proto_cpp
impl /*struct*/ QHelpEvent {
  pub fn QHelpEvent_0<T: QHelpEvent_QHelpEvent_0>(value: T) -> QHelpEvent {
    let rsthis = value.QHelpEvent_0();
    return rsthis;
    // return 1;
  }
}

pub trait QHelpEvent_QHelpEvent_0 {
  fn QHelpEvent_0(self) -> QHelpEvent;
}
// QHelpEvent(QEvent::Type, const QPoint &, const QPoint &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QHelpEvent_QHelpEvent_0 for (i32,usize,usize) {
  fn QHelpEvent_0(self) -> QHelpEvent {
    // unsafe{_ZN10QHelpEventC2EN6QEvent4TypeERK6QPointS4_()};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN10QHelpEventC2EN6QEvent4TypeERK6QPointS4_", 3,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QHelpEvent{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qevent.h:681
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QHelpEvent()

/*

*/
pub fn DeleteQHelpEvent(this :*mut QHelpEvent) {
    // let rv = qtrt::InvokeQtFunc6("_ZN10QHelpEventD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 40)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qevent.h:683
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int x() const

/*

*/
impl /*struct*/ QHelpEvent {
  pub fn x_0<RetType, T: QHelpEvent_x_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.x_0(self);
    // return 1;
  }
}
pub trait QHelpEvent_x_0<RetType> {
  fn x_0(self , rsthis: & QHelpEvent) -> RetType;
}
impl<'a> /*trait*/ QHelpEvent_x_0<i32> for () {
  fn x_0(self , rsthis: & QHelpEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QHelpEvent1xEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:684
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int y() const

/*

*/
impl /*struct*/ QHelpEvent {
  pub fn y_0<RetType, T: QHelpEvent_y_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.y_0(self);
    // return 1;
  }
}
pub trait QHelpEvent_y_0<RetType> {
  fn y_0(self , rsthis: & QHelpEvent) -> RetType;
}
impl<'a> /*trait*/ QHelpEvent_y_0<i32> for () {
  fn y_0(self , rsthis: & QHelpEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QHelpEvent1yEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:685
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int globalX() const

/*

*/
impl /*struct*/ QHelpEvent {
  pub fn globalX_0<RetType, T: QHelpEvent_globalX_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.globalX_0(self);
    // return 1;
  }
}
pub trait QHelpEvent_globalX_0<RetType> {
  fn globalX_0(self , rsthis: & QHelpEvent) -> RetType;
}
impl<'a> /*trait*/ QHelpEvent_globalX_0<i32> for () {
  fn globalX_0(self , rsthis: & QHelpEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QHelpEvent7globalXEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:686
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int globalY() const

/*

*/
impl /*struct*/ QHelpEvent {
  pub fn globalY_0<RetType, T: QHelpEvent_globalY_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.globalY_0(self);
    // return 1;
  }
}
pub trait QHelpEvent_globalY_0<RetType> {
  fn globalY_0(self , rsthis: & QHelpEvent) -> RetType;
}
impl<'a> /*trait*/ QHelpEvent_globalY_0<i32> for () {
  fn globalY_0(self , rsthis: & QHelpEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QHelpEvent7globalYEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:688
// index:0
// Public inline Visibility=Default Availability=Available
// [8] const QPoint & pos() const

/*

*/
impl /*struct*/ QHelpEvent {
  pub fn pos_0<RetType, T: QHelpEvent_pos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.pos_0(self);
    // return 1;
  }
}
pub trait QHelpEvent_pos_0<RetType> {
  fn pos_0(self , rsthis: & QHelpEvent) -> RetType;
}
impl<'a> /*trait*/ QHelpEvent_pos_0<usize> for () {
  fn pos_0(self , rsthis: & QHelpEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QHelpEvent3posEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:689
// index:0
// Public inline Visibility=Default Availability=Available
// [8] const QPoint & globalPos() const

/*

*/
impl /*struct*/ QHelpEvent {
  pub fn globalPos_0<RetType, T: QHelpEvent_globalPos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.globalPos_0(self);
    // return 1;
  }
}
pub trait QHelpEvent_globalPos_0<RetType> {
  fn globalPos_0(self , rsthis: & QHelpEvent) -> RetType;
}
impl<'a> /*trait*/ QHelpEvent_globalPos_0<usize> for () {
  fn globalPos_0(self , rsthis: & QHelpEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QHelpEvent9globalPosEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
