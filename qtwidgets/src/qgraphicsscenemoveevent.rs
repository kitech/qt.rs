

// mod ::widgets::QGraphicsSceneMoveEvent
// package qtwidgets
// /usr/include/qt/QtWidgets/qgraphicssceneevent.h
// #include <qgraphicssceneevent.h>
// #include <QtWidgets>

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
// import "github.com/kitech/qt.go/qtgui"
use qtgui::*; // super::super::%!s(MISSING)::*;
use std::default::Default;
use std::ops::Deref;
use qtrt; // super::super::qtrt;
use super::*;
//  ext block end

//  body block begin



/*

*/
#[derive(Default)] // class sizeof(QGraphicsSceneMoveEvent)=32
pub struct QGraphicsSceneMoveEvent {
  qbase: QGraphicsSceneEvent,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QGraphicsSceneMoveEvent_ITF interface {
//    QGraphicsSceneEvent_ITF
//    QGraphicsSceneMoveEvent_PTR() *QGraphicsSceneMoveEvent
//}
//func (ptr *QGraphicsSceneMoveEvent) QGraphicsSceneMoveEvent_PTR() *QGraphicsSceneMoveEvent { return ptr }

impl /*struct*/ QGraphicsSceneMoveEvent {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QGraphicsSceneMoveEvent {
    return QGraphicsSceneMoveEvent{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QGraphicsSceneMoveEvent {
//  type Target = QGraphicsSceneMoveEventBASE;
//
//  fn deref(&self) -> &QGraphicsSceneMoveEventBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QGraphicsSceneMoveEventBASE> for QGraphicsSceneMoveEvent {
//  fn as_ref(& self) -> & QGraphicsSceneMoveEventBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:313
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QGraphicsSceneMoveEvent()

/*

*/
// QGraphicsSceneMoveEvent() ctx.fn_proto_cpp
impl /*struct*/ QGraphicsSceneMoveEvent {
  pub fn QGraphicsSceneMoveEvent_0<T: QGraphicsSceneMoveEvent_QGraphicsSceneMoveEvent_0>(value: T) -> QGraphicsSceneMoveEvent {
    let rsthis = value.QGraphicsSceneMoveEvent_0();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsSceneMoveEvent_QGraphicsSceneMoveEvent_0 {
  fn QGraphicsSceneMoveEvent_0(self) -> QGraphicsSceneMoveEvent;
}
// QGraphicsSceneMoveEvent() ctx.fn_proto_cpp
impl<'a> /*trait*/ QGraphicsSceneMoveEvent_QGraphicsSceneMoveEvent_0 for () {
  fn QGraphicsSceneMoveEvent_0(self) -> QGraphicsSceneMoveEvent {
    // unsafe{_ZN23QGraphicsSceneMoveEventC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN23QGraphicsSceneMoveEventC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QGraphicsSceneMoveEvent{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:314
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QGraphicsSceneMoveEvent()

/*

*/
pub fn DeleteQGraphicsSceneMoveEvent(this :*mut QGraphicsSceneMoveEvent) {
    // let rv = qtrt::InvokeQtFunc6("_ZN23QGraphicsSceneMoveEventD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 32)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:316
// index:0
// Public Visibility=Default Availability=Available
// [16] QPointF oldPos() const

/*

*/
impl /*struct*/ QGraphicsSceneMoveEvent {
  pub fn oldPos_0<RetType, T: QGraphicsSceneMoveEvent_oldPos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.oldPos_0(self);
    // return 1;
  }
}
pub trait QGraphicsSceneMoveEvent_oldPos_0<RetType> {
  fn oldPos_0(self , rsthis: & QGraphicsSceneMoveEvent) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSceneMoveEvent_oldPos_0<usize> for () {
  fn oldPos_0(self , rsthis: & QGraphicsSceneMoveEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK23QGraphicsSceneMoveEvent6oldPosEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:317
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setOldPos(const QPointF &)

/*

*/
impl /*struct*/ QGraphicsSceneMoveEvent {
  pub fn setOldPos_0<RetType, T: QGraphicsSceneMoveEvent_setOldPos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setOldPos_0(self);
    // return 1;
  }
}
pub trait QGraphicsSceneMoveEvent_setOldPos_0<RetType> {
  fn setOldPos_0(self , rsthis: & QGraphicsSceneMoveEvent) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSceneMoveEvent_setOldPos_0<(/*void*/)> for (usize) {
  fn setOldPos_0(self , rsthis: & QGraphicsSceneMoveEvent) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN23QGraphicsSceneMoveEvent9setOldPosERK7QPointF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:319
// index:0
// Public Visibility=Default Availability=Available
// [16] QPointF newPos() const

/*

*/
impl /*struct*/ QGraphicsSceneMoveEvent {
  pub fn newPos_0<RetType, T: QGraphicsSceneMoveEvent_newPos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.newPos_0(self);
    // return 1;
  }
}
pub trait QGraphicsSceneMoveEvent_newPos_0<RetType> {
  fn newPos_0(self , rsthis: & QGraphicsSceneMoveEvent) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSceneMoveEvent_newPos_0<usize> for () {
  fn newPos_0(self , rsthis: & QGraphicsSceneMoveEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK23QGraphicsSceneMoveEvent6newPosEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:320
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setNewPos(const QPointF &)

/*

*/
impl /*struct*/ QGraphicsSceneMoveEvent {
  pub fn setNewPos_0<RetType, T: QGraphicsSceneMoveEvent_setNewPos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setNewPos_0(self);
    // return 1;
  }
}
pub trait QGraphicsSceneMoveEvent_setNewPos_0<RetType> {
  fn setNewPos_0(self , rsthis: & QGraphicsSceneMoveEvent) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSceneMoveEvent_setNewPos_0<(/*void*/)> for (usize) {
  fn setNewPos_0(self , rsthis: & QGraphicsSceneMoveEvent) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN23QGraphicsSceneMoveEvent9setNewPosERK7QPointF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

//  body block end

//  keep block begin

//  keep block end
