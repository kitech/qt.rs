

// mod ::widgets::QGestureEvent
// package qtwidgets
// /usr/include/qt/QtWidgets/qgesture.h
// #include <qgesture.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 7
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
#[derive(Default)] // class sizeof(QGestureEvent)=56
pub struct QGestureEvent {
  qbase: QEvent,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QGestureEvent_ITF interface {
//    qtcore.QEvent_ITF
//    QGestureEvent_PTR() *QGestureEvent
//}
//func (ptr *QGestureEvent) QGestureEvent_PTR() *QGestureEvent { return ptr }

impl /*struct*/ QGestureEvent {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QGestureEvent {
    return QGestureEvent{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QGestureEvent {
//  type Target = QGestureEventBASE;
//
//  fn deref(&self) -> &QGestureEventBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QGestureEventBASE> for QGestureEvent {
//  fn as_ref(& self) -> & QGestureEventBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qgesture.h:278
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QGestureEvent()

/*

*/
pub fn DeleteQGestureEvent(this :*mut QGestureEvent) {
    // let rv = qtrt::InvokeQtFunc6("_ZN13QGestureEventD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 56)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qgesture.h:281
// index:0
// Public Visibility=Default Availability=Available
// [8] QGesture * gesture(Qt::GestureType) const

/*

*/
impl /*struct*/ QGestureEvent {
  pub fn gesture_0<RetType, T: QGestureEvent_gesture_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.gesture_0(self);
    // return 1;
  }
}
pub trait QGestureEvent_gesture_0<RetType> {
  fn gesture_0(self , rsthis: & QGestureEvent) -> RetType;
}
impl<'a> /*trait*/ QGestureEvent_gesture_0<usize> for (i32) {
  fn gesture_0(self , rsthis: & QGestureEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGestureEvent7gestureEN2Qt11GestureTypeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgesture.h:291
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setAccepted(QGesture *, bool)

/*

*/
impl /*struct*/ QGestureEvent {
  pub fn setAccepted_0<RetType, T: QGestureEvent_setAccepted_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setAccepted_0(self);
    // return 1;
  }
}
pub trait QGestureEvent_setAccepted_0<RetType> {
  fn setAccepted_0(self , rsthis: & QGestureEvent) -> RetType;
}
impl<'a> /*trait*/ QGestureEvent_setAccepted_0<(/*void*/)> for (usize,bool) {
  fn setAccepted_0(self , rsthis: & QGestureEvent) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN13QGestureEvent11setAcceptedEP8QGestureb", 2,qtrt::FFITY_POINTER,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgesture.h:296
// index:1
// Public Visibility=Default Availability=Available
// [-2] void setAccepted(Qt::GestureType, bool)

/*

*/
impl /*struct*/ QGestureEvent {
  pub fn setAccepted_1<RetType, T: QGestureEvent_setAccepted_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setAccepted_1(self);
    // return 1;
  }
}
pub trait QGestureEvent_setAccepted_1<RetType> {
  fn setAccepted_1(self , rsthis: & QGestureEvent) -> RetType;
}
impl<'a> /*trait*/ QGestureEvent_setAccepted_1<(/*void*/)> for (i32,bool) {
  fn setAccepted_1(self , rsthis: & QGestureEvent) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN13QGestureEvent11setAcceptedEN2Qt11GestureTypeEb", 2,qtrt::FFITY_INT,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgesture.h:292
// index:0
// Public Visibility=Default Availability=Available
// [-2] void accept(QGesture *)

/*

*/
impl /*struct*/ QGestureEvent {
  pub fn accept_0<RetType, T: QGestureEvent_accept_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.accept_0(self);
    // return 1;
  }
}
pub trait QGestureEvent_accept_0<RetType> {
  fn accept_0(self , rsthis: & QGestureEvent) -> RetType;
}
impl<'a> /*trait*/ QGestureEvent_accept_0<(/*void*/)> for (usize) {
  fn accept_0(self , rsthis: & QGestureEvent) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QGestureEvent6acceptEP8QGesture", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgesture.h:297
// index:1
// Public Visibility=Default Availability=Available
// [-2] void accept(Qt::GestureType)

/*

*/
impl /*struct*/ QGestureEvent {
  pub fn accept_1<RetType, T: QGestureEvent_accept_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.accept_1(self);
    // return 1;
  }
}
pub trait QGestureEvent_accept_1<RetType> {
  fn accept_1(self , rsthis: & QGestureEvent) -> RetType;
}
impl<'a> /*trait*/ QGestureEvent_accept_1<(/*void*/)> for (i32) {
  fn accept_1(self , rsthis: & QGestureEvent) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN13QGestureEvent6acceptEN2Qt11GestureTypeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgesture.h:293
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ignore(QGesture *)

/*

*/
impl /*struct*/ QGestureEvent {
  pub fn ignore_0<RetType, T: QGestureEvent_ignore_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.ignore_0(self);
    // return 1;
  }
}
pub trait QGestureEvent_ignore_0<RetType> {
  fn ignore_0(self , rsthis: & QGestureEvent) -> RetType;
}
impl<'a> /*trait*/ QGestureEvent_ignore_0<(/*void*/)> for (usize) {
  fn ignore_0(self , rsthis: & QGestureEvent) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QGestureEvent6ignoreEP8QGesture", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgesture.h:298
// index:1
// Public Visibility=Default Availability=Available
// [-2] void ignore(Qt::GestureType)

/*

*/
impl /*struct*/ QGestureEvent {
  pub fn ignore_1<RetType, T: QGestureEvent_ignore_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.ignore_1(self);
    // return 1;
  }
}
pub trait QGestureEvent_ignore_1<RetType> {
  fn ignore_1(self , rsthis: & QGestureEvent) -> RetType;
}
impl<'a> /*trait*/ QGestureEvent_ignore_1<(/*void*/)> for (i32) {
  fn ignore_1(self , rsthis: & QGestureEvent) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN13QGestureEvent6ignoreEN2Qt11GestureTypeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgesture.h:294
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isAccepted(QGesture *) const

/*

*/
impl /*struct*/ QGestureEvent {
  pub fn isAccepted_0<RetType, T: QGestureEvent_isAccepted_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isAccepted_0(self);
    // return 1;
  }
}
pub trait QGestureEvent_isAccepted_0<RetType> {
  fn isAccepted_0(self , rsthis: & QGestureEvent) -> RetType;
}
impl<'a> /*trait*/ QGestureEvent_isAccepted_0<bool> for (usize) {
  fn isAccepted_0(self , rsthis: & QGestureEvent) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGestureEvent10isAcceptedEP8QGesture", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgesture.h:299
// index:1
// Public Visibility=Default Availability=Available
// [1] bool isAccepted(Qt::GestureType) const

/*

*/
impl /*struct*/ QGestureEvent {
  pub fn isAccepted_1<RetType, T: QGestureEvent_isAccepted_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isAccepted_1(self);
    // return 1;
  }
}
pub trait QGestureEvent_isAccepted_1<RetType> {
  fn isAccepted_1(self , rsthis: & QGestureEvent) -> RetType;
}
impl<'a> /*trait*/ QGestureEvent_isAccepted_1<bool> for (i32) {
  fn isAccepted_1(self , rsthis: & QGestureEvent) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGestureEvent10isAcceptedEN2Qt11GestureTypeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgesture.h:301
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setWidget(QWidget *)

/*

*/
impl /*struct*/ QGestureEvent {
  pub fn setWidget_0<RetType, T: QGestureEvent_setWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setWidget_0(self);
    // return 1;
  }
}
pub trait QGestureEvent_setWidget_0<RetType> {
  fn setWidget_0(self , rsthis: & QGestureEvent) -> RetType;
}
impl<'a> /*trait*/ QGestureEvent_setWidget_0<(/*void*/)> for (usize) {
  fn setWidget_0(self , rsthis: & QGestureEvent) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QGestureEvent9setWidgetEP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgesture.h:302
// index:0
// Public Visibility=Default Availability=Available
// [8] QWidget * widget() const

/*

*/
impl /*struct*/ QGestureEvent {
  pub fn widget_0<RetType, T: QGestureEvent_widget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.widget_0(self);
    // return 1;
  }
}
pub trait QGestureEvent_widget_0<RetType> {
  fn widget_0(self , rsthis: & QGestureEvent) -> RetType;
}
impl<'a> /*trait*/ QGestureEvent_widget_0<usize> for () {
  fn widget_0(self , rsthis: & QGestureEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGestureEvent6widgetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgesture.h:305
// index:0
// Public Visibility=Default Availability=Available
// [16] QPointF mapToGraphicsScene(const QPointF &) const

/*

*/
impl /*struct*/ QGestureEvent {
  pub fn mapToGraphicsScene_0<RetType, T: QGestureEvent_mapToGraphicsScene_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mapToGraphicsScene_0(self);
    // return 1;
  }
}
pub trait QGestureEvent_mapToGraphicsScene_0<RetType> {
  fn mapToGraphicsScene_0(self , rsthis: & QGestureEvent) -> RetType;
}
impl<'a> /*trait*/ QGestureEvent_mapToGraphicsScene_0<usize> for (usize) {
  fn mapToGraphicsScene_0(self , rsthis: & QGestureEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QGestureEvent18mapToGraphicsSceneERK7QPointF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
