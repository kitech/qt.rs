

// mod ::gui::QWheelEvent
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
#[derive(Default)] // class sizeof(QWheelEvent)=96
pub struct QWheelEvent {
  qbase: QInputEvent,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QWheelEvent_ITF interface {
//    QInputEvent_ITF
//    QWheelEvent_PTR() *QWheelEvent
//}
//func (ptr *QWheelEvent) QWheelEvent_PTR() *QWheelEvent { return ptr }

impl /*struct*/ QWheelEvent {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QWheelEvent {
    return QWheelEvent{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QWheelEvent {
//  type Target = QWheelEventBASE;
//
//  fn deref(&self) -> &QWheelEventBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QWheelEventBASE> for QWheelEvent {
//  fn as_ref(& self) -> & QWheelEventBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qevent.h:178
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QWheelEvent(const QPointF &, int, Qt::MouseButtons, Qt::KeyboardModifiers, Qt::Orientation)

/*

*/
// QWheelEvent(const QPointF &, int, Qt::MouseButtons, Qt::KeyboardModifiers, Qt::Orientation) ctx.fn_proto_cpp
impl /*struct*/ QWheelEvent {
  pub fn QWheelEvent_0<T: QWheelEvent_QWheelEvent_0>(value: T) -> QWheelEvent {
    let rsthis = value.QWheelEvent_0();
    return rsthis;
    // return 1;
  }
}

pub trait QWheelEvent_QWheelEvent_0 {
  fn QWheelEvent_0(self) -> QWheelEvent;
}
// QWheelEvent(const QPointF &, int, Qt::MouseButtons, Qt::KeyboardModifiers, Qt::Orientation) ctx.fn_proto_cpp
impl<'a> /*trait*/ QWheelEvent_QWheelEvent_0 for (usize,i32,i32,i32,i32) {
  fn QWheelEvent_0(self) -> QWheelEvent {
    // unsafe{_ZN11QWheelEventC2ERK7QPointFi6QFlagsIN2Qt11MouseButtonEES3_INS4_16KeyboardModifierEENS4_11OrientationE()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let arg4 = (&self.4) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QWheelEventC2ERK7QPointFi6QFlagsIN2Qt11MouseButtonEES3_INS4_16KeyboardModifierEENS4_11OrientationE", 5,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QWheelEvent{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qevent.h:181
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QWheelEvent(const QPointF &, const QPointF &, int, Qt::MouseButtons, Qt::KeyboardModifiers, Qt::Orientation)

/*

*/
// QWheelEvent(const QPointF &, const QPointF &, int, Qt::MouseButtons, Qt::KeyboardModifiers, Qt::Orientation) ctx.fn_proto_cpp
impl /*struct*/ QWheelEvent {
  pub fn QWheelEvent_1<T: QWheelEvent_QWheelEvent_1>(value: T) -> QWheelEvent {
    let rsthis = value.QWheelEvent_1();
    return rsthis;
    // return 1;
  }
}

pub trait QWheelEvent_QWheelEvent_1 {
  fn QWheelEvent_1(self) -> QWheelEvent;
}
// QWheelEvent(const QPointF &, const QPointF &, int, Qt::MouseButtons, Qt::KeyboardModifiers, Qt::Orientation) ctx.fn_proto_cpp
impl<'a> /*trait*/ QWheelEvent_QWheelEvent_1 for (usize,usize,i32,i32,i32,i32) {
  fn QWheelEvent_1(self) -> QWheelEvent {
    // unsafe{_ZN11QWheelEventC2ERK7QPointFS2_i6QFlagsIN2Qt11MouseButtonEES3_INS4_16KeyboardModifierEENS4_11OrientationE()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let arg4 = (&self.4) as *const i32 as usize;
    let arg5 = (&self.5) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QWheelEventC2ERK7QPointFS2_i6QFlagsIN2Qt11MouseButtonEES3_INS4_16KeyboardModifierEENS4_11OrientationE", 6,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,arg5,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QWheelEvent{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qevent.h:184
// index:2
// Public Visibility=Default Availability=Available
// [-2] void QWheelEvent(const QPointF &, const QPointF &, QPoint, QPoint, int, Qt::Orientation, Qt::MouseButtons, Qt::KeyboardModifiers)

/*

*/
// QWheelEvent(const QPointF &, const QPointF &, QPoint, QPoint, int, Qt::Orientation, Qt::MouseButtons, Qt::KeyboardModifiers) ctx.fn_proto_cpp
impl /*struct*/ QWheelEvent {
  pub fn QWheelEvent_2<T: QWheelEvent_QWheelEvent_2>(value: T) -> QWheelEvent {
    let rsthis = value.QWheelEvent_2();
    return rsthis;
    // return 1;
  }
}

pub trait QWheelEvent_QWheelEvent_2 {
  fn QWheelEvent_2(self) -> QWheelEvent;
}
// QWheelEvent(const QPointF &, const QPointF &, QPoint, QPoint, int, Qt::Orientation, Qt::MouseButtons, Qt::KeyboardModifiers) ctx.fn_proto_cpp
impl<'a> /*trait*/ QWheelEvent_QWheelEvent_2 for (usize,usize,usize,usize,i32,i32,i32,i32) {
  fn QWheelEvent_2(self) -> QWheelEvent {
    // unsafe{_ZN11QWheelEventC2ERK7QPointFS2_6QPointS3_iN2Qt11OrientationE6QFlagsINS4_11MouseButtonEES6_INS4_16KeyboardModifierEE()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
    let arg4 = (&self.4) as *const i32 as usize;
    let arg5 = (&self.5) as *const i32 as usize;
    let arg6 = (&self.6) as *const i32 as usize;
    let arg7 = (&self.7) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QWheelEventC2ERK7QPointFS2_6QPointS3_iN2Qt11OrientationE6QFlagsINS4_11MouseButtonEES6_INS4_16KeyboardModifierEE", 8,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,arg5,arg6,arg7,0,0,0,0,0,0,0,0);
    let rsthis = QWheelEvent{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qevent.h:187
// index:3
// Public Visibility=Default Availability=Available
// [-2] void QWheelEvent(const QPointF &, const QPointF &, QPoint, QPoint, int, Qt::Orientation, Qt::MouseButtons, Qt::KeyboardModifiers, Qt::ScrollPhase)

/*

*/
// QWheelEvent(const QPointF &, const QPointF &, QPoint, QPoint, int, Qt::Orientation, Qt::MouseButtons, Qt::KeyboardModifiers, Qt::ScrollPhase) ctx.fn_proto_cpp
impl /*struct*/ QWheelEvent {
  pub fn QWheelEvent_3<T: QWheelEvent_QWheelEvent_3>(value: T) -> QWheelEvent {
    let rsthis = value.QWheelEvent_3();
    return rsthis;
    // return 1;
  }
}

pub trait QWheelEvent_QWheelEvent_3 {
  fn QWheelEvent_3(self) -> QWheelEvent;
}
// QWheelEvent(const QPointF &, const QPointF &, QPoint, QPoint, int, Qt::Orientation, Qt::MouseButtons, Qt::KeyboardModifiers, Qt::ScrollPhase) ctx.fn_proto_cpp
impl<'a> /*trait*/ QWheelEvent_QWheelEvent_3 for (usize,usize,usize,usize,i32,i32,i32,i32,i32) {
  fn QWheelEvent_3(self) -> QWheelEvent {
    // unsafe{_ZN11QWheelEventC2ERK7QPointFS2_6QPointS3_iN2Qt11OrientationE6QFlagsINS4_11MouseButtonEES6_INS4_16KeyboardModifierEENS4_11ScrollPhaseE()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
    let arg4 = (&self.4) as *const i32 as usize;
    let arg5 = (&self.5) as *const i32 as usize;
    let arg6 = (&self.6) as *const i32 as usize;
    let arg7 = (&self.7) as *const i32 as usize;
    let arg8 = (&self.8) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QWheelEventC2ERK7QPointFS2_6QPointS3_iN2Qt11OrientationE6QFlagsINS4_11MouseButtonEES6_INS4_16KeyboardModifierEENS4_11ScrollPhaseE", 9,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,arg5,arg6,arg7,arg8,0,0,0,0,0,0,0);
    let rsthis = QWheelEvent{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qevent.h:190
// index:4
// Public Visibility=Default Availability=Available
// [-2] void QWheelEvent(const QPointF &, const QPointF &, QPoint, QPoint, int, Qt::Orientation, Qt::MouseButtons, Qt::KeyboardModifiers, Qt::ScrollPhase, Qt::MouseEventSource)

/*

*/
// QWheelEvent(const QPointF &, const QPointF &, QPoint, QPoint, int, Qt::Orientation, Qt::MouseButtons, Qt::KeyboardModifiers, Qt::ScrollPhase, Qt::MouseEventSource) ctx.fn_proto_cpp
impl /*struct*/ QWheelEvent {
  pub fn QWheelEvent_4<T: QWheelEvent_QWheelEvent_4>(value: T) -> QWheelEvent {
    let rsthis = value.QWheelEvent_4();
    return rsthis;
    // return 1;
  }
}

pub trait QWheelEvent_QWheelEvent_4 {
  fn QWheelEvent_4(self) -> QWheelEvent;
}
// QWheelEvent(const QPointF &, const QPointF &, QPoint, QPoint, int, Qt::Orientation, Qt::MouseButtons, Qt::KeyboardModifiers, Qt::ScrollPhase, Qt::MouseEventSource) ctx.fn_proto_cpp
impl<'a> /*trait*/ QWheelEvent_QWheelEvent_4 for (usize,usize,usize,usize,i32,i32,i32,i32,i32,i32) {
  fn QWheelEvent_4(self) -> QWheelEvent {
    // unsafe{_ZN11QWheelEventC2ERK7QPointFS2_6QPointS3_iN2Qt11OrientationE6QFlagsINS4_11MouseButtonEES6_INS4_16KeyboardModifierEENS4_11ScrollPhaseENS4_16MouseEventSourceE()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
    let arg4 = (&self.4) as *const i32 as usize;
    let arg5 = (&self.5) as *const i32 as usize;
    let arg6 = (&self.6) as *const i32 as usize;
    let arg7 = (&self.7) as *const i32 as usize;
    let arg8 = (&self.8) as *const i32 as usize;
    let arg9 = (&self.9) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QWheelEventC2ERK7QPointFS2_6QPointS3_iN2Qt11OrientationE6QFlagsINS4_11MouseButtonEES6_INS4_16KeyboardModifierEENS4_11ScrollPhaseENS4_16MouseEventSourceE", 10,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,arg5,arg6,arg7,arg8,arg9,0,0,0,0,0,0);
    let rsthis = QWheelEvent{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qevent.h:193
// index:5
// Public Visibility=Default Availability=Available
// [-2] void QWheelEvent(const QPointF &, const QPointF &, QPoint, QPoint, int, Qt::Orientation, Qt::MouseButtons, Qt::KeyboardModifiers, Qt::ScrollPhase, Qt::MouseEventSource, bool)

/*

*/
// QWheelEvent(const QPointF &, const QPointF &, QPoint, QPoint, int, Qt::Orientation, Qt::MouseButtons, Qt::KeyboardModifiers, Qt::ScrollPhase, Qt::MouseEventSource, bool) ctx.fn_proto_cpp
impl /*struct*/ QWheelEvent {
  pub fn QWheelEvent_5<T: QWheelEvent_QWheelEvent_5>(value: T) -> QWheelEvent {
    let rsthis = value.QWheelEvent_5();
    return rsthis;
    // return 1;
  }
}

pub trait QWheelEvent_QWheelEvent_5 {
  fn QWheelEvent_5(self) -> QWheelEvent;
}
// QWheelEvent(const QPointF &, const QPointF &, QPoint, QPoint, int, Qt::Orientation, Qt::MouseButtons, Qt::KeyboardModifiers, Qt::ScrollPhase, Qt::MouseEventSource, bool) ctx.fn_proto_cpp
impl<'a> /*trait*/ QWheelEvent_QWheelEvent_5 for (usize,usize,usize,usize,i32,i32,i32,i32,i32,i32,bool) {
  fn QWheelEvent_5(self) -> QWheelEvent {
    // unsafe{_ZN11QWheelEventC2ERK7QPointFS2_6QPointS3_iN2Qt11OrientationE6QFlagsINS4_11MouseButtonEES6_INS4_16KeyboardModifierEENS4_11ScrollPhaseENS4_16MouseEventSourceEb()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
    let arg4 = (&self.4) as *const i32 as usize;
    let arg5 = (&self.5) as *const i32 as usize;
    let arg6 = (&self.6) as *const i32 as usize;
    let arg7 = (&self.7) as *const i32 as usize;
    let arg8 = (&self.8) as *const i32 as usize;
    let arg9 = (&self.9) as *const i32 as usize;
    let arg10 = (&self.10) as *const bool as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QWheelEventC2ERK7QPointFS2_6QPointS3_iN2Qt11OrientationE6QFlagsINS4_11MouseButtonEES6_INS4_16KeyboardModifierEENS4_11ScrollPhaseENS4_16MouseEventSourceEb", 11,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_SINT8,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,arg5,arg6,arg7,arg8,arg9,arg10,0,0,0,0,0);
    let rsthis = QWheelEvent{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qevent.h:196
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QWheelEvent()

/*

*/
pub fn DeleteQWheelEvent(this :*mut QWheelEvent) {
    // let rv = qtrt::InvokeQtFunc6("_ZN11QWheelEventD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 96)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qevent.h:199
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QPoint pixelDelta() const

/*

*/
impl /*struct*/ QWheelEvent {
  pub fn pixelDelta_0<RetType, T: QWheelEvent_pixelDelta_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.pixelDelta_0(self);
    // return 1;
  }
}
pub trait QWheelEvent_pixelDelta_0<RetType> {
  fn pixelDelta_0(self , rsthis: & QWheelEvent) -> RetType;
}
impl<'a> /*trait*/ QWheelEvent_pixelDelta_0<usize> for () {
  fn pixelDelta_0(self , rsthis: & QWheelEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QWheelEvent10pixelDeltaEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:200
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QPoint angleDelta() const

/*

*/
impl /*struct*/ QWheelEvent {
  pub fn angleDelta_0<RetType, T: QWheelEvent_angleDelta_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.angleDelta_0(self);
    // return 1;
  }
}
pub trait QWheelEvent_angleDelta_0<RetType> {
  fn angleDelta_0(self , rsthis: & QWheelEvent) -> RetType;
}
impl<'a> /*trait*/ QWheelEvent_angleDelta_0<usize> for () {
  fn angleDelta_0(self , rsthis: & QWheelEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QWheelEvent10angleDeltaEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:202
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int delta() const

/*

*/
impl /*struct*/ QWheelEvent {
  pub fn delta_0<RetType, T: QWheelEvent_delta_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.delta_0(self);
    // return 1;
  }
}
pub trait QWheelEvent_delta_0<RetType> {
  fn delta_0(self , rsthis: & QWheelEvent) -> RetType;
}
impl<'a> /*trait*/ QWheelEvent_delta_0<i32> for () {
  fn delta_0(self , rsthis: & QWheelEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QWheelEvent5deltaEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:203
// index:0
// Public inline Visibility=Default Availability=Available
// [4] Qt::Orientation orientation() const

/*

*/
impl /*struct*/ QWheelEvent {
  pub fn orientation_0<RetType, T: QWheelEvent_orientation_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.orientation_0(self);
    // return 1;
  }
}
pub trait QWheelEvent_orientation_0<RetType> {
  fn orientation_0(self , rsthis: & QWheelEvent) -> RetType;
}
impl<'a> /*trait*/ QWheelEvent_orientation_0<i32> for () {
  fn orientation_0(self , rsthis: & QWheelEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QWheelEvent11orientationEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:206
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QPoint pos() const

/*

*/
impl /*struct*/ QWheelEvent {
  pub fn pos_0<RetType, T: QWheelEvent_pos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.pos_0(self);
    // return 1;
  }
}
pub trait QWheelEvent_pos_0<RetType> {
  fn pos_0(self , rsthis: & QWheelEvent) -> RetType;
}
impl<'a> /*trait*/ QWheelEvent_pos_0<usize> for () {
  fn pos_0(self , rsthis: & QWheelEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QWheelEvent3posEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:207
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QPoint globalPos() const

/*

*/
impl /*struct*/ QWheelEvent {
  pub fn globalPos_0<RetType, T: QWheelEvent_globalPos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.globalPos_0(self);
    // return 1;
  }
}
pub trait QWheelEvent_globalPos_0<RetType> {
  fn globalPos_0(self , rsthis: & QWheelEvent) -> RetType;
}
impl<'a> /*trait*/ QWheelEvent_globalPos_0<usize> for () {
  fn globalPos_0(self , rsthis: & QWheelEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QWheelEvent9globalPosEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:208
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int x() const

/*

*/
impl /*struct*/ QWheelEvent {
  pub fn x_0<RetType, T: QWheelEvent_x_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.x_0(self);
    // return 1;
  }
}
pub trait QWheelEvent_x_0<RetType> {
  fn x_0(self , rsthis: & QWheelEvent) -> RetType;
}
impl<'a> /*trait*/ QWheelEvent_x_0<i32> for () {
  fn x_0(self , rsthis: & QWheelEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QWheelEvent1xEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:209
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int y() const

/*

*/
impl /*struct*/ QWheelEvent {
  pub fn y_0<RetType, T: QWheelEvent_y_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.y_0(self);
    // return 1;
  }
}
pub trait QWheelEvent_y_0<RetType> {
  fn y_0(self , rsthis: & QWheelEvent) -> RetType;
}
impl<'a> /*trait*/ QWheelEvent_y_0<i32> for () {
  fn y_0(self , rsthis: & QWheelEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QWheelEvent1yEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:210
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int globalX() const

/*

*/
impl /*struct*/ QWheelEvent {
  pub fn globalX_0<RetType, T: QWheelEvent_globalX_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.globalX_0(self);
    // return 1;
  }
}
pub trait QWheelEvent_globalX_0<RetType> {
  fn globalX_0(self , rsthis: & QWheelEvent) -> RetType;
}
impl<'a> /*trait*/ QWheelEvent_globalX_0<i32> for () {
  fn globalX_0(self , rsthis: & QWheelEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QWheelEvent7globalXEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:211
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int globalY() const

/*

*/
impl /*struct*/ QWheelEvent {
  pub fn globalY_0<RetType, T: QWheelEvent_globalY_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.globalY_0(self);
    // return 1;
  }
}
pub trait QWheelEvent_globalY_0<RetType> {
  fn globalY_0(self , rsthis: & QWheelEvent) -> RetType;
}
impl<'a> /*trait*/ QWheelEvent_globalY_0<i32> for () {
  fn globalY_0(self , rsthis: & QWheelEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QWheelEvent7globalYEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:213
// index:0
// Public inline Visibility=Default Availability=Available
// [16] const QPointF & posF() const

/*

*/
impl /*struct*/ QWheelEvent {
  pub fn posF_0<RetType, T: QWheelEvent_posF_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.posF_0(self);
    // return 1;
  }
}
pub trait QWheelEvent_posF_0<RetType> {
  fn posF_0(self , rsthis: & QWheelEvent) -> RetType;
}
impl<'a> /*trait*/ QWheelEvent_posF_0<usize> for () {
  fn posF_0(self , rsthis: & QWheelEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QWheelEvent4posFEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:214
// index:0
// Public inline Visibility=Default Availability=Available
// [16] const QPointF & globalPosF() const

/*

*/
impl /*struct*/ QWheelEvent {
  pub fn globalPosF_0<RetType, T: QWheelEvent_globalPosF_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.globalPosF_0(self);
    // return 1;
  }
}
pub trait QWheelEvent_globalPosF_0<RetType> {
  fn globalPosF_0(self , rsthis: & QWheelEvent) -> RetType;
}
impl<'a> /*trait*/ QWheelEvent_globalPosF_0<usize> for () {
  fn globalPosF_0(self , rsthis: & QWheelEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QWheelEvent10globalPosFEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:216
// index:0
// Public inline Visibility=Default Availability=Available
// [4] Qt::MouseButtons buttons() const

/*

*/
impl /*struct*/ QWheelEvent {
  pub fn buttons_0<RetType, T: QWheelEvent_buttons_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.buttons_0(self);
    // return 1;
  }
}
pub trait QWheelEvent_buttons_0<RetType> {
  fn buttons_0(self , rsthis: & QWheelEvent) -> RetType;
}
impl<'a> /*trait*/ QWheelEvent_buttons_0<i32> for () {
  fn buttons_0(self , rsthis: & QWheelEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QWheelEvent7buttonsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:218
// index:0
// Public inline Visibility=Default Availability=Available
// [4] Qt::ScrollPhase phase() const

/*

*/
impl /*struct*/ QWheelEvent {
  pub fn phase_0<RetType, T: QWheelEvent_phase_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.phase_0(self);
    // return 1;
  }
}
pub trait QWheelEvent_phase_0<RetType> {
  fn phase_0(self , rsthis: & QWheelEvent) -> RetType;
}
impl<'a> /*trait*/ QWheelEvent_phase_0<i32> for () {
  fn phase_0(self , rsthis: & QWheelEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QWheelEvent5phaseEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:219
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool inverted() const

/*

*/
impl /*struct*/ QWheelEvent {
  pub fn inverted_0<RetType, T: QWheelEvent_inverted_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.inverted_0(self);
    // return 1;
  }
}
pub trait QWheelEvent_inverted_0<RetType> {
  fn inverted_0(self , rsthis: & QWheelEvent) -> RetType;
}
impl<'a> /*trait*/ QWheelEvent_inverted_0<bool> for () {
  fn inverted_0(self , rsthis: & QWheelEvent) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QWheelEvent8invertedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:221
// index:0
// Public inline Visibility=Default Availability=Available
// [4] Qt::MouseEventSource source() const

/*

*/
impl /*struct*/ QWheelEvent {
  pub fn source_0<RetType, T: QWheelEvent_source_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.source_0(self);
    // return 1;
  }
}
pub trait QWheelEvent_source_0<RetType> {
  fn source_0(self , rsthis: & QWheelEvent) -> RetType;
}
impl<'a> /*trait*/ QWheelEvent_source_0<i32> for () {
  fn source_0(self , rsthis: & QWheelEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QWheelEvent6sourceEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}


/*


*/
pub type QWheelEvent__ = i32;
// 
pub const QWheelEvent__DefaultDeltasPerStep :QWheelEvent__ = 120;
pub fn QWheelEvent_ItemName(val: i32) ->String {
  match val {
     QWheelEvent__DefaultDeltasPerStep => // 120
     {return String::from("DefaultDeltasPerStep");}
  _ => {return format!("{}", val);}
}
}
pub fn QWheelEvent_ItemName_s(val: i32) ->String {
  //var nilthis *QWheelEvent
  //return nilthis.ItemName(val);
  return QWheelEvent_ItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
