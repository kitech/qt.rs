

// mod ::widgets::QGraphicsSceneResizeEvent
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
// extern C begin: 23
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
#[derive(Default)] // class sizeof(QGraphicsSceneResizeEvent)=32
pub struct QGraphicsSceneResizeEvent {
  qbase: QGraphicsSceneEvent,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QGraphicsSceneResizeEvent_ITF interface {
//    QGraphicsSceneEvent_ITF
//    QGraphicsSceneResizeEvent_PTR() *QGraphicsSceneResizeEvent
//}
//func (ptr *QGraphicsSceneResizeEvent) QGraphicsSceneResizeEvent_PTR() *QGraphicsSceneResizeEvent { return ptr }

impl /*struct*/ QGraphicsSceneResizeEvent {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QGraphicsSceneResizeEvent {
    return QGraphicsSceneResizeEvent{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QGraphicsSceneResizeEvent {
//  type Target = QGraphicsSceneResizeEventBASE;
//
//  fn deref(&self) -> &QGraphicsSceneResizeEventBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QGraphicsSceneResizeEventBASE> for QGraphicsSceneResizeEvent {
//  fn as_ref(& self) -> & QGraphicsSceneResizeEventBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:297
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QGraphicsSceneResizeEvent()

/*

*/
// QGraphicsSceneResizeEvent() ctx.fn_proto_cpp
impl /*struct*/ QGraphicsSceneResizeEvent {
  pub fn QGraphicsSceneResizeEvent_0<T: QGraphicsSceneResizeEvent_QGraphicsSceneResizeEvent_0>(value: T) -> QGraphicsSceneResizeEvent {
    let rsthis = value.QGraphicsSceneResizeEvent_0();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsSceneResizeEvent_QGraphicsSceneResizeEvent_0 {
  fn QGraphicsSceneResizeEvent_0(self) -> QGraphicsSceneResizeEvent;
}
// QGraphicsSceneResizeEvent() ctx.fn_proto_cpp
impl<'a> /*trait*/ QGraphicsSceneResizeEvent_QGraphicsSceneResizeEvent_0 for () {
  fn QGraphicsSceneResizeEvent_0(self) -> QGraphicsSceneResizeEvent {
    // unsafe{_ZN25QGraphicsSceneResizeEventC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN25QGraphicsSceneResizeEventC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QGraphicsSceneResizeEvent{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:298
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QGraphicsSceneResizeEvent()

/*

*/
pub fn DeleteQGraphicsSceneResizeEvent(this :*mut QGraphicsSceneResizeEvent) {
    // let rv = qtrt::InvokeQtFunc6("_ZN25QGraphicsSceneResizeEventD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 32)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:300
// index:0
// Public Visibility=Default Availability=Available
// [16] QSizeF oldSize() const

/*

*/
impl /*struct*/ QGraphicsSceneResizeEvent {
  pub fn oldSize_0<RetType, T: QGraphicsSceneResizeEvent_oldSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.oldSize_0(self);
    // return 1;
  }
}
pub trait QGraphicsSceneResizeEvent_oldSize_0<RetType> {
  fn oldSize_0(self , rsthis: & QGraphicsSceneResizeEvent) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSceneResizeEvent_oldSize_0<usize> for () {
  fn oldSize_0(self , rsthis: & QGraphicsSceneResizeEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK25QGraphicsSceneResizeEvent7oldSizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:301
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setOldSize(const QSizeF &)

/*

*/
impl /*struct*/ QGraphicsSceneResizeEvent {
  pub fn setOldSize_0<RetType, T: QGraphicsSceneResizeEvent_setOldSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setOldSize_0(self);
    // return 1;
  }
}
pub trait QGraphicsSceneResizeEvent_setOldSize_0<RetType> {
  fn setOldSize_0(self , rsthis: & QGraphicsSceneResizeEvent) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSceneResizeEvent_setOldSize_0<(/*void*/)> for (usize) {
  fn setOldSize_0(self , rsthis: & QGraphicsSceneResizeEvent) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN25QGraphicsSceneResizeEvent10setOldSizeERK6QSizeF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:303
// index:0
// Public Visibility=Default Availability=Available
// [16] QSizeF newSize() const

/*

*/
impl /*struct*/ QGraphicsSceneResizeEvent {
  pub fn newSize_0<RetType, T: QGraphicsSceneResizeEvent_newSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.newSize_0(self);
    // return 1;
  }
}
pub trait QGraphicsSceneResizeEvent_newSize_0<RetType> {
  fn newSize_0(self , rsthis: & QGraphicsSceneResizeEvent) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSceneResizeEvent_newSize_0<usize> for () {
  fn newSize_0(self , rsthis: & QGraphicsSceneResizeEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK25QGraphicsSceneResizeEvent7newSizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgraphicssceneevent.h:304
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setNewSize(const QSizeF &)

/*

*/
impl /*struct*/ QGraphicsSceneResizeEvent {
  pub fn setNewSize_0<RetType, T: QGraphicsSceneResizeEvent_setNewSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setNewSize_0(self);
    // return 1;
  }
}
pub trait QGraphicsSceneResizeEvent_setNewSize_0<RetType> {
  fn setNewSize_0(self , rsthis: & QGraphicsSceneResizeEvent) -> RetType;
}
impl<'a> /*trait*/ QGraphicsSceneResizeEvent_setNewSize_0<(/*void*/)> for (usize) {
  fn setNewSize_0(self , rsthis: & QGraphicsSceneResizeEvent) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN25QGraphicsSceneResizeEvent10setNewSizeERK6QSizeF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

//  body block end

//  keep block begin

//  keep block end
