

// mod ::gui::QTabletEvent
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
use std::default::Default;
use std::ops::Deref;
use qtrt; // super::super::qtrt;
use super::*;
//  ext block end

//  body block begin



/*

*/
#[derive(Default)] // class sizeof(QTabletEvent)=128
pub struct QTabletEvent {
  qbase: QInputEvent,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QTabletEvent_ITF interface {
//    QInputEvent_ITF
//    QTabletEvent_PTR() *QTabletEvent
//}
//func (ptr *QTabletEvent) QTabletEvent_PTR() *QTabletEvent { return ptr }

impl /*struct*/ QTabletEvent {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QTabletEvent {
    return QTabletEvent{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QTabletEvent {
//  type Target = QTabletEventBASE;
//
//  fn deref(&self) -> &QTabletEventBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QTabletEventBASE> for QTabletEvent {
//  fn as_ref(& self) -> & QTabletEventBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qevent.h:250
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QTabletEvent(QEvent::Type, const QPointF &, const QPointF &, int, int, qreal, int, int, qreal, qreal, int, Qt::KeyboardModifiers, qint64)

/*

*/
// QTabletEvent(QEvent::Type, const QPointF &, const QPointF &, int, int, qreal, int, int, qreal, qreal, int, Qt::KeyboardModifiers, qint64) ctx.fn_proto_cpp
impl /*struct*/ QTabletEvent {
  pub fn QTabletEvent_0<T: QTabletEvent_QTabletEvent_0>(value: T) -> QTabletEvent {
    let rsthis = value.QTabletEvent_0();
    return rsthis;
    // return 1;
  }
}

pub trait QTabletEvent_QTabletEvent_0 {
  fn QTabletEvent_0(self) -> QTabletEvent;
}
// QTabletEvent(QEvent::Type, const QPointF &, const QPointF &, int, int, qreal, int, int, qreal, qreal, int, Qt::KeyboardModifiers, qint64) ctx.fn_proto_cpp
impl<'a> /*trait*/ QTabletEvent_QTabletEvent_0 for (i32,usize,usize,i32,i32,f64,i32,i32,f64,f64,i32,i32,i64) {
  fn QTabletEvent_0(self) -> QTabletEvent {
    // unsafe{_ZN12QTabletEventC2EN6QEvent4TypeERK7QPointFS4_iidiiddi6QFlagsIN2Qt16KeyboardModifierEEx()};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let arg4 = (&self.4) as *const i32 as usize;
    let arg5 = (&self.5) as *const f64 as usize;
    let arg6 = (&self.6) as *const i32 as usize;
    let arg7 = (&self.7) as *const i32 as usize;
    let arg8 = (&self.8) as *const f64 as usize;
    let arg9 = (&self.9) as *const f64 as usize;
    let arg10 = (&self.10) as *const i32 as usize;
    let arg11 = (&self.11) as *const i32 as usize;
    let arg12 = (&self.12) as *const i64 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN12QTabletEventC2EN6QEvent4TypeERK7QPointFS4_iidiiddi6QFlagsIN2Qt16KeyboardModifierEEx", 13,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_DOUBLE,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_SINT64,0,0,0,arg0,arg1,arg2,arg3,arg4,arg5,arg6,arg7,arg8,arg9,arg10,arg11,arg12,0,0,0);
    let rsthis = QTabletEvent{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qevent.h:254
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QTabletEvent(QEvent::Type, const QPointF &, const QPointF &, int, int, qreal, int, int, qreal, qreal, int, Qt::KeyboardModifiers, qint64, Qt::MouseButton, Qt::MouseButtons)

/*

*/
// QTabletEvent(QEvent::Type, const QPointF &, const QPointF &, int, int, qreal, int, int, qreal, qreal, int, Qt::KeyboardModifiers, qint64, Qt::MouseButton, Qt::MouseButtons) ctx.fn_proto_cpp
impl /*struct*/ QTabletEvent {
  pub fn QTabletEvent_1<T: QTabletEvent_QTabletEvent_1>(value: T) -> QTabletEvent {
    let rsthis = value.QTabletEvent_1();
    return rsthis;
    // return 1;
  }
}

pub trait QTabletEvent_QTabletEvent_1 {
  fn QTabletEvent_1(self) -> QTabletEvent;
}
// QTabletEvent(QEvent::Type, const QPointF &, const QPointF &, int, int, qreal, int, int, qreal, qreal, int, Qt::KeyboardModifiers, qint64, Qt::MouseButton, Qt::MouseButtons) ctx.fn_proto_cpp
impl<'a> /*trait*/ QTabletEvent_QTabletEvent_1 for (i32,usize,usize,i32,i32,f64,i32,i32,f64,f64,i32,i32,i64,i32,i32) {
  fn QTabletEvent_1(self) -> QTabletEvent {
    // unsafe{_ZN12QTabletEventC2EN6QEvent4TypeERK7QPointFS4_iidiiddi6QFlagsIN2Qt16KeyboardModifierEExNS6_11MouseButtonES5_IS9_E()};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let arg4 = (&self.4) as *const i32 as usize;
    let arg5 = (&self.5) as *const f64 as usize;
    let arg6 = (&self.6) as *const i32 as usize;
    let arg7 = (&self.7) as *const i32 as usize;
    let arg8 = (&self.8) as *const f64 as usize;
    let arg9 = (&self.9) as *const f64 as usize;
    let arg10 = (&self.10) as *const i32 as usize;
    let arg11 = (&self.11) as *const i32 as usize;
    let arg12 = (&self.12) as *const i64 as usize;
    let arg13 = (&self.13) as *const i32 as usize;
    let arg14 = (&self.14) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN12QTabletEventC2EN6QEvent4TypeERK7QPointFS4_iidiiddi6QFlagsIN2Qt16KeyboardModifierEExNS6_11MouseButtonES5_IS9_E", 15,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_DOUBLE,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_SINT64,qtrt::FFITY_INT,qtrt::FFITY_INT,0,arg0,arg1,arg2,arg3,arg4,arg5,arg6,arg7,arg8,arg9,arg10,arg11,arg12,arg13,arg14,0);
    let rsthis = QTabletEvent{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qevent.h:259
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QTabletEvent()

/*

*/
pub fn DeleteQTabletEvent(this :*mut QTabletEvent) {
    // let rv = qtrt::InvokeQtFunc6("_ZN12QTabletEventD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 128)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qevent.h:261
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QPoint pos() const

/*

*/
impl /*struct*/ QTabletEvent {
  pub fn pos_0<RetType, T: QTabletEvent_pos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.pos_0(self);
    // return 1;
  }
}
pub trait QTabletEvent_pos_0<RetType> {
  fn pos_0(self , rsthis: & QTabletEvent) -> RetType;
}
impl<'a> /*trait*/ QTabletEvent_pos_0<usize> for () {
  fn pos_0(self , rsthis: & QTabletEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QTabletEvent3posEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:262
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QPoint globalPos() const

/*

*/
impl /*struct*/ QTabletEvent {
  pub fn globalPos_0<RetType, T: QTabletEvent_globalPos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.globalPos_0(self);
    // return 1;
  }
}
pub trait QTabletEvent_globalPos_0<RetType> {
  fn globalPos_0(self , rsthis: & QTabletEvent) -> RetType;
}
impl<'a> /*trait*/ QTabletEvent_globalPos_0<usize> for () {
  fn globalPos_0(self , rsthis: & QTabletEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QTabletEvent9globalPosEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:267
// index:0
// Public inline Visibility=Default Availability=Available
// [16] const QPointF & posF() const

/*

*/
impl /*struct*/ QTabletEvent {
  pub fn posF_0<RetType, T: QTabletEvent_posF_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.posF_0(self);
    // return 1;
  }
}
pub trait QTabletEvent_posF_0<RetType> {
  fn posF_0(self , rsthis: & QTabletEvent) -> RetType;
}
impl<'a> /*trait*/ QTabletEvent_posF_0<usize> for () {
  fn posF_0(self , rsthis: & QTabletEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QTabletEvent4posFEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:268
// index:0
// Public inline Visibility=Default Availability=Available
// [16] const QPointF & globalPosF() const

/*

*/
impl /*struct*/ QTabletEvent {
  pub fn globalPosF_0<RetType, T: QTabletEvent_globalPosF_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.globalPosF_0(self);
    // return 1;
  }
}
pub trait QTabletEvent_globalPosF_0<RetType> {
  fn globalPosF_0(self , rsthis: & QTabletEvent) -> RetType;
}
impl<'a> /*trait*/ QTabletEvent_globalPosF_0<usize> for () {
  fn globalPosF_0(self , rsthis: & QTabletEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QTabletEvent10globalPosFEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:270
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int x() const

/*

*/
impl /*struct*/ QTabletEvent {
  pub fn x_0<RetType, T: QTabletEvent_x_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.x_0(self);
    // return 1;
  }
}
pub trait QTabletEvent_x_0<RetType> {
  fn x_0(self , rsthis: & QTabletEvent) -> RetType;
}
impl<'a> /*trait*/ QTabletEvent_x_0<i32> for () {
  fn x_0(self , rsthis: & QTabletEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QTabletEvent1xEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:271
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int y() const

/*

*/
impl /*struct*/ QTabletEvent {
  pub fn y_0<RetType, T: QTabletEvent_y_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.y_0(self);
    // return 1;
  }
}
pub trait QTabletEvent_y_0<RetType> {
  fn y_0(self , rsthis: & QTabletEvent) -> RetType;
}
impl<'a> /*trait*/ QTabletEvent_y_0<i32> for () {
  fn y_0(self , rsthis: & QTabletEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QTabletEvent1yEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:272
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int globalX() const

/*

*/
impl /*struct*/ QTabletEvent {
  pub fn globalX_0<RetType, T: QTabletEvent_globalX_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.globalX_0(self);
    // return 1;
  }
}
pub trait QTabletEvent_globalX_0<RetType> {
  fn globalX_0(self , rsthis: & QTabletEvent) -> RetType;
}
impl<'a> /*trait*/ QTabletEvent_globalX_0<i32> for () {
  fn globalX_0(self , rsthis: & QTabletEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QTabletEvent7globalXEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:273
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int globalY() const

/*

*/
impl /*struct*/ QTabletEvent {
  pub fn globalY_0<RetType, T: QTabletEvent_globalY_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.globalY_0(self);
    // return 1;
  }
}
pub trait QTabletEvent_globalY_0<RetType> {
  fn globalY_0(self , rsthis: & QTabletEvent) -> RetType;
}
impl<'a> /*trait*/ QTabletEvent_globalY_0<i32> for () {
  fn globalY_0(self , rsthis: & QTabletEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QTabletEvent7globalYEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:274
// index:0
// Public inline Visibility=Default Availability=Available
// [8] qreal hiResGlobalX() const

/*

*/
impl /*struct*/ QTabletEvent {
  pub fn hiResGlobalX_0<RetType, T: QTabletEvent_hiResGlobalX_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hiResGlobalX_0(self);
    // return 1;
  }
}
pub trait QTabletEvent_hiResGlobalX_0<RetType> {
  fn hiResGlobalX_0(self , rsthis: & QTabletEvent) -> RetType;
}
impl<'a> /*trait*/ QTabletEvent_hiResGlobalX_0<f64> for () {
  fn hiResGlobalX_0(self , rsthis: & QTabletEvent) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QTabletEvent12hiResGlobalXEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:275
// index:0
// Public inline Visibility=Default Availability=Available
// [8] qreal hiResGlobalY() const

/*

*/
impl /*struct*/ QTabletEvent {
  pub fn hiResGlobalY_0<RetType, T: QTabletEvent_hiResGlobalY_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hiResGlobalY_0(self);
    // return 1;
  }
}
pub trait QTabletEvent_hiResGlobalY_0<RetType> {
  fn hiResGlobalY_0(self , rsthis: & QTabletEvent) -> RetType;
}
impl<'a> /*trait*/ QTabletEvent_hiResGlobalY_0<f64> for () {
  fn hiResGlobalY_0(self , rsthis: & QTabletEvent) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QTabletEvent12hiResGlobalYEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:276
// index:0
// Public inline Visibility=Default Availability=Available
// [4] QTabletEvent::TabletDevice device() const

/*

*/
impl /*struct*/ QTabletEvent {
  pub fn device_0<RetType, T: QTabletEvent_device_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.device_0(self);
    // return 1;
  }
}
pub trait QTabletEvent_device_0<RetType> {
  fn device_0(self , rsthis: & QTabletEvent) -> RetType;
}
impl<'a> /*trait*/ QTabletEvent_device_0<i32> for () {
  fn device_0(self , rsthis: & QTabletEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QTabletEvent6deviceEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:277
// index:0
// Public inline Visibility=Default Availability=Available
// [4] QTabletEvent::PointerType pointerType() const

/*

*/
impl /*struct*/ QTabletEvent {
  pub fn pointerType_0<RetType, T: QTabletEvent_pointerType_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.pointerType_0(self);
    // return 1;
  }
}
pub trait QTabletEvent_pointerType_0<RetType> {
  fn pointerType_0(self , rsthis: & QTabletEvent) -> RetType;
}
impl<'a> /*trait*/ QTabletEvent_pointerType_0<i32> for () {
  fn pointerType_0(self , rsthis: & QTabletEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QTabletEvent11pointerTypeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:278
// index:0
// Public inline Visibility=Default Availability=Available
// [8] qint64 uniqueId() const

/*

*/
impl /*struct*/ QTabletEvent {
  pub fn uniqueId_0<RetType, T: QTabletEvent_uniqueId_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.uniqueId_0(self);
    // return 1;
  }
}
pub trait QTabletEvent_uniqueId_0<RetType> {
  fn uniqueId_0(self , rsthis: & QTabletEvent) -> RetType;
}
impl<'a> /*trait*/ QTabletEvent_uniqueId_0<i64> for () {
  fn uniqueId_0(self , rsthis: & QTabletEvent) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QTabletEvent8uniqueIdEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:279
// index:0
// Public inline Visibility=Default Availability=Available
// [8] qreal pressure() const

/*

*/
impl /*struct*/ QTabletEvent {
  pub fn pressure_0<RetType, T: QTabletEvent_pressure_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.pressure_0(self);
    // return 1;
  }
}
pub trait QTabletEvent_pressure_0<RetType> {
  fn pressure_0(self , rsthis: & QTabletEvent) -> RetType;
}
impl<'a> /*trait*/ QTabletEvent_pressure_0<f64> for () {
  fn pressure_0(self , rsthis: & QTabletEvent) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QTabletEvent8pressureEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:280
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int z() const

/*

*/
impl /*struct*/ QTabletEvent {
  pub fn z_0<RetType, T: QTabletEvent_z_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.z_0(self);
    // return 1;
  }
}
pub trait QTabletEvent_z_0<RetType> {
  fn z_0(self , rsthis: & QTabletEvent) -> RetType;
}
impl<'a> /*trait*/ QTabletEvent_z_0<i32> for () {
  fn z_0(self , rsthis: & QTabletEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QTabletEvent1zEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:281
// index:0
// Public inline Visibility=Default Availability=Available
// [8] qreal tangentialPressure() const

/*

*/
impl /*struct*/ QTabletEvent {
  pub fn tangentialPressure_0<RetType, T: QTabletEvent_tangentialPressure_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.tangentialPressure_0(self);
    // return 1;
  }
}
pub trait QTabletEvent_tangentialPressure_0<RetType> {
  fn tangentialPressure_0(self , rsthis: & QTabletEvent) -> RetType;
}
impl<'a> /*trait*/ QTabletEvent_tangentialPressure_0<f64> for () {
  fn tangentialPressure_0(self , rsthis: & QTabletEvent) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QTabletEvent18tangentialPressureEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:282
// index:0
// Public inline Visibility=Default Availability=Available
// [8] qreal rotation() const

/*

*/
impl /*struct*/ QTabletEvent {
  pub fn rotation_0<RetType, T: QTabletEvent_rotation_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rotation_0(self);
    // return 1;
  }
}
pub trait QTabletEvent_rotation_0<RetType> {
  fn rotation_0(self , rsthis: & QTabletEvent) -> RetType;
}
impl<'a> /*trait*/ QTabletEvent_rotation_0<f64> for () {
  fn rotation_0(self , rsthis: & QTabletEvent) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QTabletEvent8rotationEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:283
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int xTilt() const

/*

*/
impl /*struct*/ QTabletEvent {
  pub fn xTilt_0<RetType, T: QTabletEvent_xTilt_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.xTilt_0(self);
    // return 1;
  }
}
pub trait QTabletEvent_xTilt_0<RetType> {
  fn xTilt_0(self , rsthis: & QTabletEvent) -> RetType;
}
impl<'a> /*trait*/ QTabletEvent_xTilt_0<i32> for () {
  fn xTilt_0(self , rsthis: & QTabletEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QTabletEvent5xTiltEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:284
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int yTilt() const

/*

*/
impl /*struct*/ QTabletEvent {
  pub fn yTilt_0<RetType, T: QTabletEvent_yTilt_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.yTilt_0(self);
    // return 1;
  }
}
pub trait QTabletEvent_yTilt_0<RetType> {
  fn yTilt_0(self , rsthis: & QTabletEvent) -> RetType;
}
impl<'a> /*trait*/ QTabletEvent_yTilt_0<i32> for () {
  fn yTilt_0(self , rsthis: & QTabletEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QTabletEvent5yTiltEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:285
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::MouseButton button() const

/*

*/
impl /*struct*/ QTabletEvent {
  pub fn button_0<RetType, T: QTabletEvent_button_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.button_0(self);
    // return 1;
  }
}
pub trait QTabletEvent_button_0<RetType> {
  fn button_0(self , rsthis: & QTabletEvent) -> RetType;
}
impl<'a> /*trait*/ QTabletEvent_button_0<i32> for () {
  fn button_0(self , rsthis: & QTabletEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QTabletEvent6buttonEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:286
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::MouseButtons buttons() const

/*

*/
impl /*struct*/ QTabletEvent {
  pub fn buttons_0<RetType, T: QTabletEvent_buttons_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.buttons_0(self);
    // return 1;
  }
}
pub trait QTabletEvent_buttons_0<RetType> {
  fn buttons_0(self , rsthis: & QTabletEvent) -> RetType;
}
impl<'a> /*trait*/ QTabletEvent_buttons_0<i32> for () {
  fn buttons_0(self , rsthis: & QTabletEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QTabletEvent7buttonsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}


/*


*/
pub type QTabletEvent__TabletDevice = i32;
// 
pub const QTabletEvent__NoDevice :QTabletEvent__TabletDevice = 0;
// 
pub const QTabletEvent__Puck :QTabletEvent__TabletDevice = 1;
// 
pub const QTabletEvent__Stylus :QTabletEvent__TabletDevice = 2;
// 
pub const QTabletEvent__Airbrush :QTabletEvent__TabletDevice = 3;
// 
pub const QTabletEvent__FourDMouse :QTabletEvent__TabletDevice = 4;
// 
pub const QTabletEvent__XFreeEraser :QTabletEvent__TabletDevice = 5;
// 
pub const QTabletEvent__RotationStylus :QTabletEvent__TabletDevice = 6;
pub fn QTabletEvent_TabletDeviceItemName(val: i32) ->String {
  match val {
     QTabletEvent__NoDevice => // 0
     {return String::from("NoDevice");}
     QTabletEvent__Puck => // 1
     {return String::from("Puck");}
     QTabletEvent__Stylus => // 2
     {return String::from("Stylus");}
     QTabletEvent__Airbrush => // 3
     {return String::from("Airbrush");}
     QTabletEvent__FourDMouse => // 4
     {return String::from("FourDMouse");}
     QTabletEvent__XFreeEraser => // 5
     {return String::from("XFreeEraser");}
     QTabletEvent__RotationStylus => // 6
     {return String::from("RotationStylus");}
  _ => {return format!("{}", val);}
}
}
pub fn QTabletEvent_TabletDeviceItemName_s(val: i32) ->String {
  //var nilthis *QTabletEvent
  //return nilthis.TabletDeviceItemName(val);
  return QTabletEvent_TabletDeviceItemName(val);
}


/*


*/
pub type QTabletEvent__PointerType = i32;
// 
pub const QTabletEvent__UnknownPointer :QTabletEvent__PointerType = 0;
// 
pub const QTabletEvent__Pen :QTabletEvent__PointerType = 1;
// 
pub const QTabletEvent__Cursor :QTabletEvent__PointerType = 2;
// 
pub const QTabletEvent__Eraser :QTabletEvent__PointerType = 3;
pub fn QTabletEvent_PointerTypeItemName(val: i32) ->String {
  match val {
     QTabletEvent__UnknownPointer => // 0
     {return String::from("UnknownPointer");}
     QTabletEvent__Pen => // 1
     {return String::from("Pen");}
     QTabletEvent__Cursor => // 2
     {return String::from("Cursor");}
     QTabletEvent__Eraser => // 3
     {return String::from("Eraser");}
  _ => {return format!("{}", val);}
}
}
pub fn QTabletEvent_PointerTypeItemName_s(val: i32) ->String {
  //var nilthis *QTabletEvent
  //return nilthis.PointerTypeItemName(val);
  return QTabletEvent_PointerTypeItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
