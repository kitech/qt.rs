

// mod ::gui::QNativeGestureEvent
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
// extern C begin: 24
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
#[derive(Default)] // class sizeof(QNativeGestureEvent)=112
pub struct QNativeGestureEvent {
  qbase: QInputEvent,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QNativeGestureEvent_ITF interface {
//    QInputEvent_ITF
//    QNativeGestureEvent_PTR() *QNativeGestureEvent
//}
//func (ptr *QNativeGestureEvent) QNativeGestureEvent_PTR() *QNativeGestureEvent { return ptr }

impl /*struct*/ QNativeGestureEvent {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QNativeGestureEvent {
    return QNativeGestureEvent{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QNativeGestureEvent {
//  type Target = QNativeGestureEventBASE;
//
//  fn deref(&self) -> &QNativeGestureEventBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QNativeGestureEventBASE> for QNativeGestureEvent {
//  fn as_ref(& self) -> & QNativeGestureEventBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qevent.h:305
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QNativeGestureEvent(Qt::NativeGestureType, const QPointF &, const QPointF &, const QPointF &, qreal, ulong, quint64)

/*

*/
// QNativeGestureEvent(Qt::NativeGestureType, const QPointF &, const QPointF &, const QPointF &, qreal, ulong, quint64) ctx.fn_proto_cpp
impl /*struct*/ QNativeGestureEvent {
  pub fn QNativeGestureEvent_0<T: QNativeGestureEvent_QNativeGestureEvent_0>(value: T) -> QNativeGestureEvent {
    let rsthis = value.QNativeGestureEvent_0();
    return rsthis;
    // return 1;
  }
}

pub trait QNativeGestureEvent_QNativeGestureEvent_0 {
  fn QNativeGestureEvent_0(self) -> QNativeGestureEvent;
}
// QNativeGestureEvent(Qt::NativeGestureType, const QPointF &, const QPointF &, const QPointF &, qreal, ulong, quint64) ctx.fn_proto_cpp
impl<'a> /*trait*/ QNativeGestureEvent_QNativeGestureEvent_0 for (i32,usize,usize,usize,f64,u64,u64) {
  fn QNativeGestureEvent_0(self) -> QNativeGestureEvent {
    // unsafe{_ZN19QNativeGestureEventC2EN2Qt17NativeGestureTypeERK7QPointFS4_S4_dmy()};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
    let arg4 = (&self.4) as *const f64 as usize;
    let arg5 = (&self.5) as *const u64 as usize;
    let arg6 = (&self.6) as *const u64 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN19QNativeGestureEventC2EN2Qt17NativeGestureTypeERK7QPointFS4_S4_dmy", 7,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_DOUBLE,qtrt::FFITY_UINT64,qtrt::FFITY_UINT64,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,arg5,arg6,0,0,0,0,0,0,0,0,0);
    let rsthis = QNativeGestureEvent{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qevent.h:308
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QNativeGestureEvent(Qt::NativeGestureType, const QTouchDevice *, const QPointF &, const QPointF &, const QPointF &, qreal, ulong, quint64)

/*

*/
// QNativeGestureEvent(Qt::NativeGestureType, const QTouchDevice *, const QPointF &, const QPointF &, const QPointF &, qreal, ulong, quint64) ctx.fn_proto_cpp
impl /*struct*/ QNativeGestureEvent {
  pub fn QNativeGestureEvent_1<T: QNativeGestureEvent_QNativeGestureEvent_1>(value: T) -> QNativeGestureEvent {
    let rsthis = value.QNativeGestureEvent_1();
    return rsthis;
    // return 1;
  }
}

pub trait QNativeGestureEvent_QNativeGestureEvent_1 {
  fn QNativeGestureEvent_1(self) -> QNativeGestureEvent;
}
// QNativeGestureEvent(Qt::NativeGestureType, const QTouchDevice *, const QPointF &, const QPointF &, const QPointF &, qreal, ulong, quint64) ctx.fn_proto_cpp
impl<'a> /*trait*/ QNativeGestureEvent_QNativeGestureEvent_1 for (i32,usize,usize,usize,usize,f64,u64,u64) {
  fn QNativeGestureEvent_1(self) -> QNativeGestureEvent {
    // unsafe{_ZN19QNativeGestureEventC2EN2Qt17NativeGestureTypeEPK12QTouchDeviceRK7QPointFS7_S7_dmy()};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
    let arg4 = (&self.4/*.qclsinst*/) as *const usize as usize;
    let arg5 = (&self.5) as *const f64 as usize;
    let arg6 = (&self.6) as *const u64 as usize;
    let arg7 = (&self.7) as *const u64 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN19QNativeGestureEventC2EN2Qt17NativeGestureTypeEPK12QTouchDeviceRK7QPointFS7_S7_dmy", 8,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_DOUBLE,qtrt::FFITY_UINT64,qtrt::FFITY_UINT64,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,arg5,arg6,arg7,0,0,0,0,0,0,0,0);
    let rsthis = QNativeGestureEvent{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qevent.h:310
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QNativeGestureEvent()

/*

*/
pub fn DeleteQNativeGestureEvent(this :*mut QNativeGestureEvent) {
    // let rv = qtrt::InvokeQtFunc6("_ZN19QNativeGestureEventD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 112)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qevent.h:311
// index:0
// Public inline Visibility=Default Availability=Available
// [4] Qt::NativeGestureType gestureType() const

/*

*/
impl /*struct*/ QNativeGestureEvent {
  pub fn gestureType_0<RetType, T: QNativeGestureEvent_gestureType_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.gestureType_0(self);
    // return 1;
  }
}
pub trait QNativeGestureEvent_gestureType_0<RetType> {
  fn gestureType_0(self , rsthis: & QNativeGestureEvent) -> RetType;
}
impl<'a> /*trait*/ QNativeGestureEvent_gestureType_0<i32> for () {
  fn gestureType_0(self , rsthis: & QNativeGestureEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QNativeGestureEvent11gestureTypeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:312
// index:0
// Public inline Visibility=Default Availability=Available
// [8] qreal value() const

/*

*/
impl /*struct*/ QNativeGestureEvent {
  pub fn value_0<RetType, T: QNativeGestureEvent_value_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.value_0(self);
    // return 1;
  }
}
pub trait QNativeGestureEvent_value_0<RetType> {
  fn value_0(self , rsthis: & QNativeGestureEvent) -> RetType;
}
impl<'a> /*trait*/ QNativeGestureEvent_value_0<f64> for () {
  fn value_0(self , rsthis: & QNativeGestureEvent) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QNativeGestureEvent5valueEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:315
// index:0
// Public inline Visibility=Default Availability=Available
// [8] const QPoint pos() const

/*

*/
impl /*struct*/ QNativeGestureEvent {
  pub fn pos_0<RetType, T: QNativeGestureEvent_pos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.pos_0(self);
    // return 1;
  }
}
pub trait QNativeGestureEvent_pos_0<RetType> {
  fn pos_0(self , rsthis: & QNativeGestureEvent) -> RetType;
}
impl<'a> /*trait*/ QNativeGestureEvent_pos_0<usize> for () {
  fn pos_0(self , rsthis: & QNativeGestureEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QNativeGestureEvent3posEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:316
// index:0
// Public inline Visibility=Default Availability=Available
// [8] const QPoint globalPos() const

/*

*/
impl /*struct*/ QNativeGestureEvent {
  pub fn globalPos_0<RetType, T: QNativeGestureEvent_globalPos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.globalPos_0(self);
    // return 1;
  }
}
pub trait QNativeGestureEvent_globalPos_0<RetType> {
  fn globalPos_0(self , rsthis: & QNativeGestureEvent) -> RetType;
}
impl<'a> /*trait*/ QNativeGestureEvent_globalPos_0<usize> for () {
  fn globalPos_0(self , rsthis: & QNativeGestureEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QNativeGestureEvent9globalPosEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:318
// index:0
// Public inline Visibility=Default Availability=Available
// [16] const QPointF & localPos() const

/*

*/
impl /*struct*/ QNativeGestureEvent {
  pub fn localPos_0<RetType, T: QNativeGestureEvent_localPos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.localPos_0(self);
    // return 1;
  }
}
pub trait QNativeGestureEvent_localPos_0<RetType> {
  fn localPos_0(self , rsthis: & QNativeGestureEvent) -> RetType;
}
impl<'a> /*trait*/ QNativeGestureEvent_localPos_0<usize> for () {
  fn localPos_0(self , rsthis: & QNativeGestureEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QNativeGestureEvent8localPosEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:319
// index:0
// Public inline Visibility=Default Availability=Available
// [16] const QPointF & windowPos() const

/*

*/
impl /*struct*/ QNativeGestureEvent {
  pub fn windowPos_0<RetType, T: QNativeGestureEvent_windowPos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.windowPos_0(self);
    // return 1;
  }
}
pub trait QNativeGestureEvent_windowPos_0<RetType> {
  fn windowPos_0(self , rsthis: & QNativeGestureEvent) -> RetType;
}
impl<'a> /*trait*/ QNativeGestureEvent_windowPos_0<usize> for () {
  fn windowPos_0(self , rsthis: & QNativeGestureEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QNativeGestureEvent9windowPosEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:320
// index:0
// Public inline Visibility=Default Availability=Available
// [16] const QPointF & screenPos() const

/*

*/
impl /*struct*/ QNativeGestureEvent {
  pub fn screenPos_0<RetType, T: QNativeGestureEvent_screenPos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.screenPos_0(self);
    // return 1;
  }
}
pub trait QNativeGestureEvent_screenPos_0<RetType> {
  fn screenPos_0(self , rsthis: & QNativeGestureEvent) -> RetType;
}
impl<'a> /*trait*/ QNativeGestureEvent_screenPos_0<usize> for () {
  fn screenPos_0(self , rsthis: & QNativeGestureEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QNativeGestureEvent9screenPosEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:322
// index:0
// Public Visibility=Default Availability=Available
// [8] const QTouchDevice * device() const

/*

*/
impl /*struct*/ QNativeGestureEvent {
  pub fn device_0<RetType, T: QNativeGestureEvent_device_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.device_0(self);
    // return 1;
  }
}
pub trait QNativeGestureEvent_device_0<RetType> {
  fn device_0(self , rsthis: & QNativeGestureEvent) -> RetType;
}
impl<'a> /*trait*/ QNativeGestureEvent_device_0<usize> for () {
  fn device_0(self , rsthis: & QNativeGestureEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QNativeGestureEvent6deviceEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
