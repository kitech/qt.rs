

// mod ::gui::QMouseEvent
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
// extern C begin: 11
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
#[derive(Default)] // class sizeof(QMouseEvent)=104
pub struct QMouseEvent {
  qbase: QInputEvent,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QMouseEvent_ITF interface {
//    QInputEvent_ITF
//    QMouseEvent_PTR() *QMouseEvent
//}
//func (ptr *QMouseEvent) QMouseEvent_PTR() *QMouseEvent { return ptr }

impl /*struct*/ QMouseEvent {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QMouseEvent {
    return QMouseEvent{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QMouseEvent {
//  type Target = QMouseEventBASE;
//
//  fn deref(&self) -> &QMouseEventBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QMouseEventBASE> for QMouseEvent {
//  fn as_ref(& self) -> & QMouseEventBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qevent.h:107
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QMouseEvent(QEvent::Type, const QPointF &, Qt::MouseButton, Qt::MouseButtons, Qt::KeyboardModifiers)

/*

*/
// QMouseEvent(QEvent::Type, const QPointF &, Qt::MouseButton, Qt::MouseButtons, Qt::KeyboardModifiers) ctx.fn_proto_cpp
impl /*struct*/ QMouseEvent {
  pub fn QMouseEvent_0<T: QMouseEvent_QMouseEvent_0>(value: T) -> QMouseEvent {
    let rsthis = value.QMouseEvent_0();
    return rsthis;
    // return 1;
  }
}

pub trait QMouseEvent_QMouseEvent_0 {
  fn QMouseEvent_0(self) -> QMouseEvent;
}
// QMouseEvent(QEvent::Type, const QPointF &, Qt::MouseButton, Qt::MouseButtons, Qt::KeyboardModifiers) ctx.fn_proto_cpp
impl<'a> /*trait*/ QMouseEvent_QMouseEvent_0 for (i32,usize,i32,i32,i32) {
  fn QMouseEvent_0(self) -> QMouseEvent {
    // unsafe{_ZN11QMouseEventC2EN6QEvent4TypeERK7QPointFN2Qt11MouseButtonE6QFlagsIS6_ES7_INS5_16KeyboardModifierEE()};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let arg4 = (&self.4) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QMouseEventC2EN6QEvent4TypeERK7QPointFN2Qt11MouseButtonE6QFlagsIS6_ES7_INS5_16KeyboardModifierEE", 5,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QMouseEvent{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qevent.h:109
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QMouseEvent(QEvent::Type, const QPointF &, const QPointF &, Qt::MouseButton, Qt::MouseButtons, Qt::KeyboardModifiers)

/*

*/
// QMouseEvent(QEvent::Type, const QPointF &, const QPointF &, Qt::MouseButton, Qt::MouseButtons, Qt::KeyboardModifiers) ctx.fn_proto_cpp
impl /*struct*/ QMouseEvent {
  pub fn QMouseEvent_1<T: QMouseEvent_QMouseEvent_1>(value: T) -> QMouseEvent {
    let rsthis = value.QMouseEvent_1();
    return rsthis;
    // return 1;
  }
}

pub trait QMouseEvent_QMouseEvent_1 {
  fn QMouseEvent_1(self) -> QMouseEvent;
}
// QMouseEvent(QEvent::Type, const QPointF &, const QPointF &, Qt::MouseButton, Qt::MouseButtons, Qt::KeyboardModifiers) ctx.fn_proto_cpp
impl<'a> /*trait*/ QMouseEvent_QMouseEvent_1 for (i32,usize,usize,i32,i32,i32) {
  fn QMouseEvent_1(self) -> QMouseEvent {
    // unsafe{_ZN11QMouseEventC2EN6QEvent4TypeERK7QPointFS4_N2Qt11MouseButtonE6QFlagsIS6_ES7_INS5_16KeyboardModifierEE()};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let arg4 = (&self.4) as *const i32 as usize;
    let arg5 = (&self.5) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QMouseEventC2EN6QEvent4TypeERK7QPointFS4_N2Qt11MouseButtonE6QFlagsIS6_ES7_INS5_16KeyboardModifierEE", 6,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,arg5,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QMouseEvent{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qevent.h:112
// index:2
// Public Visibility=Default Availability=Available
// [-2] void QMouseEvent(QEvent::Type, const QPointF &, const QPointF &, const QPointF &, Qt::MouseButton, Qt::MouseButtons, Qt::KeyboardModifiers)

/*

*/
// QMouseEvent(QEvent::Type, const QPointF &, const QPointF &, const QPointF &, Qt::MouseButton, Qt::MouseButtons, Qt::KeyboardModifiers) ctx.fn_proto_cpp
impl /*struct*/ QMouseEvent {
  pub fn QMouseEvent_2<T: QMouseEvent_QMouseEvent_2>(value: T) -> QMouseEvent {
    let rsthis = value.QMouseEvent_2();
    return rsthis;
    // return 1;
  }
}

pub trait QMouseEvent_QMouseEvent_2 {
  fn QMouseEvent_2(self) -> QMouseEvent;
}
// QMouseEvent(QEvent::Type, const QPointF &, const QPointF &, const QPointF &, Qt::MouseButton, Qt::MouseButtons, Qt::KeyboardModifiers) ctx.fn_proto_cpp
impl<'a> /*trait*/ QMouseEvent_QMouseEvent_2 for (i32,usize,usize,usize,i32,i32,i32) {
  fn QMouseEvent_2(self) -> QMouseEvent {
    // unsafe{_ZN11QMouseEventC2EN6QEvent4TypeERK7QPointFS4_S4_N2Qt11MouseButtonE6QFlagsIS6_ES7_INS5_16KeyboardModifierEE()};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
    let arg4 = (&self.4) as *const i32 as usize;
    let arg5 = (&self.5) as *const i32 as usize;
    let arg6 = (&self.6) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QMouseEventC2EN6QEvent4TypeERK7QPointFS4_S4_N2Qt11MouseButtonE6QFlagsIS6_ES7_INS5_16KeyboardModifierEE", 7,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,arg5,arg6,0,0,0,0,0,0,0,0,0);
    let rsthis = QMouseEvent{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qevent.h:115
// index:3
// Public Visibility=Default Availability=Available
// [-2] void QMouseEvent(QEvent::Type, const QPointF &, const QPointF &, const QPointF &, Qt::MouseButton, Qt::MouseButtons, Qt::KeyboardModifiers, Qt::MouseEventSource)

/*

*/
// QMouseEvent(QEvent::Type, const QPointF &, const QPointF &, const QPointF &, Qt::MouseButton, Qt::MouseButtons, Qt::KeyboardModifiers, Qt::MouseEventSource) ctx.fn_proto_cpp
impl /*struct*/ QMouseEvent {
  pub fn QMouseEvent_3<T: QMouseEvent_QMouseEvent_3>(value: T) -> QMouseEvent {
    let rsthis = value.QMouseEvent_3();
    return rsthis;
    // return 1;
  }
}

pub trait QMouseEvent_QMouseEvent_3 {
  fn QMouseEvent_3(self) -> QMouseEvent;
}
// QMouseEvent(QEvent::Type, const QPointF &, const QPointF &, const QPointF &, Qt::MouseButton, Qt::MouseButtons, Qt::KeyboardModifiers, Qt::MouseEventSource) ctx.fn_proto_cpp
impl<'a> /*trait*/ QMouseEvent_QMouseEvent_3 for (i32,usize,usize,usize,i32,i32,i32,i32) {
  fn QMouseEvent_3(self) -> QMouseEvent {
    // unsafe{_ZN11QMouseEventC2EN6QEvent4TypeERK7QPointFS4_S4_N2Qt11MouseButtonE6QFlagsIS6_ES7_INS5_16KeyboardModifierEENS5_16MouseEventSourceE()};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
    let arg4 = (&self.4) as *const i32 as usize;
    let arg5 = (&self.5) as *const i32 as usize;
    let arg6 = (&self.6) as *const i32 as usize;
    let arg7 = (&self.7) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QMouseEventC2EN6QEvent4TypeERK7QPointFS4_S4_N2Qt11MouseButtonE6QFlagsIS6_ES7_INS5_16KeyboardModifierEENS5_16MouseEventSourceE", 8,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,arg5,arg6,arg7,0,0,0,0,0,0,0,0);
    let rsthis = QMouseEvent{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qevent.h:118
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QMouseEvent()

/*

*/
pub fn DeleteQMouseEvent(this :*mut QMouseEvent) {
    // let rv = qtrt::InvokeQtFunc6("_ZN11QMouseEventD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 104)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qevent.h:121
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QPoint pos() const

/*

*/
impl /*struct*/ QMouseEvent {
  pub fn pos_0<RetType, T: QMouseEvent_pos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.pos_0(self);
    // return 1;
  }
}
pub trait QMouseEvent_pos_0<RetType> {
  fn pos_0(self , rsthis: & QMouseEvent) -> RetType;
}
impl<'a> /*trait*/ QMouseEvent_pos_0<usize> for () {
  fn pos_0(self , rsthis: & QMouseEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QMouseEvent3posEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:122
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QPoint globalPos() const

/*

*/
impl /*struct*/ QMouseEvent {
  pub fn globalPos_0<RetType, T: QMouseEvent_globalPos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.globalPos_0(self);
    // return 1;
  }
}
pub trait QMouseEvent_globalPos_0<RetType> {
  fn globalPos_0(self , rsthis: & QMouseEvent) -> RetType;
}
impl<'a> /*trait*/ QMouseEvent_globalPos_0<usize> for () {
  fn globalPos_0(self , rsthis: & QMouseEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QMouseEvent9globalPosEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:123
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int x() const

/*

*/
impl /*struct*/ QMouseEvent {
  pub fn x_0<RetType, T: QMouseEvent_x_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.x_0(self);
    // return 1;
  }
}
pub trait QMouseEvent_x_0<RetType> {
  fn x_0(self , rsthis: & QMouseEvent) -> RetType;
}
impl<'a> /*trait*/ QMouseEvent_x_0<i32> for () {
  fn x_0(self , rsthis: & QMouseEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QMouseEvent1xEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:124
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int y() const

/*

*/
impl /*struct*/ QMouseEvent {
  pub fn y_0<RetType, T: QMouseEvent_y_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.y_0(self);
    // return 1;
  }
}
pub trait QMouseEvent_y_0<RetType> {
  fn y_0(self , rsthis: & QMouseEvent) -> RetType;
}
impl<'a> /*trait*/ QMouseEvent_y_0<i32> for () {
  fn y_0(self , rsthis: & QMouseEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QMouseEvent1yEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:125
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int globalX() const

/*

*/
impl /*struct*/ QMouseEvent {
  pub fn globalX_0<RetType, T: QMouseEvent_globalX_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.globalX_0(self);
    // return 1;
  }
}
pub trait QMouseEvent_globalX_0<RetType> {
  fn globalX_0(self , rsthis: & QMouseEvent) -> RetType;
}
impl<'a> /*trait*/ QMouseEvent_globalX_0<i32> for () {
  fn globalX_0(self , rsthis: & QMouseEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QMouseEvent7globalXEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:126
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int globalY() const

/*

*/
impl /*struct*/ QMouseEvent {
  pub fn globalY_0<RetType, T: QMouseEvent_globalY_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.globalY_0(self);
    // return 1;
  }
}
pub trait QMouseEvent_globalY_0<RetType> {
  fn globalY_0(self , rsthis: & QMouseEvent) -> RetType;
}
impl<'a> /*trait*/ QMouseEvent_globalY_0<i32> for () {
  fn globalY_0(self , rsthis: & QMouseEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QMouseEvent7globalYEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:128
// index:0
// Public inline Visibility=Default Availability=Available
// [16] const QPointF & localPos() const

/*

*/
impl /*struct*/ QMouseEvent {
  pub fn localPos_0<RetType, T: QMouseEvent_localPos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.localPos_0(self);
    // return 1;
  }
}
pub trait QMouseEvent_localPos_0<RetType> {
  fn localPos_0(self , rsthis: & QMouseEvent) -> RetType;
}
impl<'a> /*trait*/ QMouseEvent_localPos_0<usize> for () {
  fn localPos_0(self , rsthis: & QMouseEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QMouseEvent8localPosEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:129
// index:0
// Public inline Visibility=Default Availability=Available
// [16] const QPointF & windowPos() const

/*

*/
impl /*struct*/ QMouseEvent {
  pub fn windowPos_0<RetType, T: QMouseEvent_windowPos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.windowPos_0(self);
    // return 1;
  }
}
pub trait QMouseEvent_windowPos_0<RetType> {
  fn windowPos_0(self , rsthis: & QMouseEvent) -> RetType;
}
impl<'a> /*trait*/ QMouseEvent_windowPos_0<usize> for () {
  fn windowPos_0(self , rsthis: & QMouseEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QMouseEvent9windowPosEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:130
// index:0
// Public inline Visibility=Default Availability=Available
// [16] const QPointF & screenPos() const

/*

*/
impl /*struct*/ QMouseEvent {
  pub fn screenPos_0<RetType, T: QMouseEvent_screenPos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.screenPos_0(self);
    // return 1;
  }
}
pub trait QMouseEvent_screenPos_0<RetType> {
  fn screenPos_0(self , rsthis: & QMouseEvent) -> RetType;
}
impl<'a> /*trait*/ QMouseEvent_screenPos_0<usize> for () {
  fn screenPos_0(self , rsthis: & QMouseEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QMouseEvent9screenPosEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:132
// index:0
// Public inline Visibility=Default Availability=Available
// [4] Qt::MouseButton button() const

/*

*/
impl /*struct*/ QMouseEvent {
  pub fn button_0<RetType, T: QMouseEvent_button_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.button_0(self);
    // return 1;
  }
}
pub trait QMouseEvent_button_0<RetType> {
  fn button_0(self , rsthis: & QMouseEvent) -> RetType;
}
impl<'a> /*trait*/ QMouseEvent_button_0<i32> for () {
  fn button_0(self , rsthis: & QMouseEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QMouseEvent6buttonEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:133
// index:0
// Public inline Visibility=Default Availability=Available
// [4] Qt::MouseButtons buttons() const

/*

*/
impl /*struct*/ QMouseEvent {
  pub fn buttons_0<RetType, T: QMouseEvent_buttons_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.buttons_0(self);
    // return 1;
  }
}
pub trait QMouseEvent_buttons_0<RetType> {
  fn buttons_0(self , rsthis: & QMouseEvent) -> RetType;
}
impl<'a> /*trait*/ QMouseEvent_buttons_0<i32> for () {
  fn buttons_0(self , rsthis: & QMouseEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QMouseEvent7buttonsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:135
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setLocalPos(const QPointF &)

/*

*/
impl /*struct*/ QMouseEvent {
  pub fn setLocalPos_0<RetType, T: QMouseEvent_setLocalPos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setLocalPos_0(self);
    // return 1;
  }
}
pub trait QMouseEvent_setLocalPos_0<RetType> {
  fn setLocalPos_0(self , rsthis: & QMouseEvent) -> RetType;
}
impl<'a> /*trait*/ QMouseEvent_setLocalPos_0<(/*void*/)> for (usize) {
  fn setLocalPos_0(self , rsthis: & QMouseEvent) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QMouseEvent11setLocalPosERK7QPointF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qevent.h:141
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::MouseEventSource source() const

/*

*/
impl /*struct*/ QMouseEvent {
  pub fn source_0<RetType, T: QMouseEvent_source_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.source_0(self);
    // return 1;
  }
}
pub trait QMouseEvent_source_0<RetType> {
  fn source_0(self , rsthis: & QMouseEvent) -> RetType;
}
impl<'a> /*trait*/ QMouseEvent_source_0<i32> for () {
  fn source_0(self , rsthis: & QMouseEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QMouseEvent6sourceEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:142
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::MouseEventFlags flags() const

/*

*/
impl /*struct*/ QMouseEvent {
  pub fn flags_0<RetType, T: QMouseEvent_flags_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.flags_0(self);
    // return 1;
  }
}
pub trait QMouseEvent_flags_0<RetType> {
  fn flags_0(self , rsthis: & QMouseEvent) -> RetType;
}
impl<'a> /*trait*/ QMouseEvent_flags_0<i32> for () {
  fn flags_0(self , rsthis: & QMouseEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QMouseEvent5flagsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
