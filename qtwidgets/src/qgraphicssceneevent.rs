

// mod ::widgets::QGraphicsSceneEvent
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
// extern C begin: 106
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
#[derive(Default)] // class sizeof(QGraphicsSceneEvent)=32
pub struct QGraphicsSceneEvent {
  qbase: QEvent,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QGraphicsSceneEvent_ITF interface {
//    qtcore.QEvent_ITF
//    QGraphicsSceneEvent_PTR() *QGraphicsSceneEvent
//}
//func (ptr *QGraphicsSceneEvent) QGraphicsSceneEvent_PTR() *QGraphicsSceneEvent { return ptr }

impl /*struct*/ QGraphicsSceneEvent {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QGraphicsSceneEvent {
    return QGraphicsSceneEvent{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QGraphicsSceneEvent {
//  type Target = QGraphicsSceneEventBASE;
//
//  fn deref(&self) -> &QGraphicsSceneEventBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QGraphicsSceneEventBASE> for QGraphicsSceneEvent {
//  fn as_ref(& self) -> & QGraphicsSceneEventBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:67
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QGraphicsSceneEvent(QEvent::Type)

/*

*/
// QGraphicsSceneEvent(QEvent::Type) ctx.fn_proto_cpp
impl /*struct*/ QGraphicsSceneEvent {
  pub fn QGraphicsSceneEvent_0<T: QGraphicsSceneEvent_QGraphicsSceneEvent_0>(value: T) -> QGraphicsSceneEvent {
    let rsthis = value.QGraphicsSceneEvent_0();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsSceneEvent_QGraphicsSceneEvent_0 {
  fn QGraphicsSceneEvent_0(self) -> QGraphicsSceneEvent;
}
// QGraphicsSceneEvent(QEvent::Type) ctx.fn_proto_cpp
impl<'a> /*trait*/ QGraphicsSceneEvent_QGraphicsSceneEvent_0 for (i32) {
  fn QGraphicsSceneEvent_0(self) -> QGraphicsSceneEvent {
    // unsafe{_ZN19QGraphicsSceneEventC2EN6QEvent4TypeE()};
    let arg0 = (&self) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN19QGraphicsSceneEventC2EN6QEvent4TypeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QGraphicsSceneEvent{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:68
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QGraphicsSceneEvent()

/*

*/
pub fn DeleteQGraphicsSceneEvent(this :*mut QGraphicsSceneEvent) {
    // let rv = qtrt::InvokeQtFunc6("_ZN19QGraphicsSceneEventD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 32)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:70
// index:0
// Public Visibility=Default Availability=Available
// [8] QWidget * widget() const

/*
Returns the widget where the event originated, or 0 if the event originates from another application.
*/
impl /*struct*/ QGraphicsSceneEvent {
  pub fn widget_0<RetType, T: QGraphicsSceneEvent_widget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.widget_0(self);
    // return 1;
  }
}
pub trait QGraphicsSceneEvent_widget_0<RetType> {
  fn widget_0(self , rsthis: & QGraphicsSceneEvent) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSceneEvent_widget_0<usize> for () {
  fn widget_0(self , rsthis: & QGraphicsSceneEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QGraphicsSceneEvent6widgetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:71
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setWidget(QWidget *)

/*

*/
impl /*struct*/ QGraphicsSceneEvent {
  pub fn setWidget_0<RetType, T: QGraphicsSceneEvent_setWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setWidget_0(self);
    // return 1;
  }
}
pub trait QGraphicsSceneEvent_setWidget_0<RetType> {
  fn setWidget_0(self , rsthis: & QGraphicsSceneEvent) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSceneEvent_setWidget_0<(/*void*/)> for (usize) {
  fn setWidget_0(self , rsthis: & QGraphicsSceneEvent) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN19QGraphicsSceneEvent9setWidgetEP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

//  body block end

//  keep block begin

//  keep block end
