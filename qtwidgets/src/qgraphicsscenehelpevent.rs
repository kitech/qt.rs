

// mod ::widgets::QGraphicsSceneHelpEvent
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
// extern C begin: 16
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
#[derive(Default)] // class sizeof(QGraphicsSceneHelpEvent)=32
pub struct QGraphicsSceneHelpEvent {
  qbase: QGraphicsSceneEvent,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QGraphicsSceneHelpEvent_ITF interface {
//    QGraphicsSceneEvent_ITF
//    QGraphicsSceneHelpEvent_PTR() *QGraphicsSceneHelpEvent
//}
//func (ptr *QGraphicsSceneHelpEvent) QGraphicsSceneHelpEvent_PTR() *QGraphicsSceneHelpEvent { return ptr }

impl /*struct*/ QGraphicsSceneHelpEvent {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QGraphicsSceneHelpEvent {
    return QGraphicsSceneHelpEvent{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QGraphicsSceneHelpEvent {
//  type Target = QGraphicsSceneHelpEventBASE;
//
//  fn deref(&self) -> &QGraphicsSceneHelpEventBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QGraphicsSceneHelpEventBASE> for QGraphicsSceneHelpEvent {
//  fn as_ref(& self) -> & QGraphicsSceneHelpEventBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:234
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QGraphicsSceneHelpEvent(QEvent::Type)

/*

*/
// QGraphicsSceneHelpEvent(QEvent::Type) ctx.fn_proto_cpp
impl /*struct*/ QGraphicsSceneHelpEvent {
  pub fn QGraphicsSceneHelpEvent_0<T: QGraphicsSceneHelpEvent_QGraphicsSceneHelpEvent_0>(value: T) -> QGraphicsSceneHelpEvent {
    let rsthis = value.QGraphicsSceneHelpEvent_0();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsSceneHelpEvent_QGraphicsSceneHelpEvent_0 {
  fn QGraphicsSceneHelpEvent_0(self) -> QGraphicsSceneHelpEvent;
}
// QGraphicsSceneHelpEvent(QEvent::Type) ctx.fn_proto_cpp
impl<'a> /*trait*/ QGraphicsSceneHelpEvent_QGraphicsSceneHelpEvent_0 for (i32) {
  fn QGraphicsSceneHelpEvent_0(self) -> QGraphicsSceneHelpEvent {
    // unsafe{_ZN23QGraphicsSceneHelpEventC2EN6QEvent4TypeE()};
    let arg0 = (&self) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN23QGraphicsSceneHelpEventC2EN6QEvent4TypeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QGraphicsSceneHelpEvent{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:235
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QGraphicsSceneHelpEvent()

/*

*/
pub fn DeleteQGraphicsSceneHelpEvent(this :*mut QGraphicsSceneHelpEvent) {
    // let rv = qtrt::InvokeQtFunc6("_ZN23QGraphicsSceneHelpEventD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 32)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:237
// index:0
// Public Visibility=Default Availability=Available
// [16] QPointF scenePos() const

/*

*/
impl /*struct*/ QGraphicsSceneHelpEvent {
  pub fn scenePos_0<RetType, T: QGraphicsSceneHelpEvent_scenePos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.scenePos_0(self);
    // return 1;
  }
}
pub trait QGraphicsSceneHelpEvent_scenePos_0<RetType> {
  fn scenePos_0(self , rsthis: & QGraphicsSceneHelpEvent) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSceneHelpEvent_scenePos_0<usize> for () {
  fn scenePos_0(self , rsthis: & QGraphicsSceneHelpEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK23QGraphicsSceneHelpEvent8scenePosEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:238
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setScenePos(const QPointF &)

/*

*/
impl /*struct*/ QGraphicsSceneHelpEvent {
  pub fn setScenePos_0<RetType, T: QGraphicsSceneHelpEvent_setScenePos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setScenePos_0(self);
    // return 1;
  }
}
pub trait QGraphicsSceneHelpEvent_setScenePos_0<RetType> {
  fn setScenePos_0(self , rsthis: & QGraphicsSceneHelpEvent) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSceneHelpEvent_setScenePos_0<(/*void*/)> for (usize) {
  fn setScenePos_0(self , rsthis: & QGraphicsSceneHelpEvent) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN23QGraphicsSceneHelpEvent11setScenePosERK7QPointF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:240
// index:0
// Public Visibility=Default Availability=Available
// [8] QPoint screenPos() const

/*

*/
impl /*struct*/ QGraphicsSceneHelpEvent {
  pub fn screenPos_0<RetType, T: QGraphicsSceneHelpEvent_screenPos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.screenPos_0(self);
    // return 1;
  }
}
pub trait QGraphicsSceneHelpEvent_screenPos_0<RetType> {
  fn screenPos_0(self , rsthis: & QGraphicsSceneHelpEvent) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSceneHelpEvent_screenPos_0<usize> for () {
  fn screenPos_0(self , rsthis: & QGraphicsSceneHelpEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK23QGraphicsSceneHelpEvent9screenPosEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:241
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setScreenPos(const QPoint &)

/*

*/
impl /*struct*/ QGraphicsSceneHelpEvent {
  pub fn setScreenPos_0<RetType, T: QGraphicsSceneHelpEvent_setScreenPos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setScreenPos_0(self);
    // return 1;
  }
}
pub trait QGraphicsSceneHelpEvent_setScreenPos_0<RetType> {
  fn setScreenPos_0(self , rsthis: & QGraphicsSceneHelpEvent) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSceneHelpEvent_setScreenPos_0<(/*void*/)> for (usize) {
  fn setScreenPos_0(self , rsthis: & QGraphicsSceneHelpEvent) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN23QGraphicsSceneHelpEvent12setScreenPosERK6QPoint", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

//  body block end

//  keep block begin

//  keep block end
