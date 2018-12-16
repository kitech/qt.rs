

// mod ::widgets::QScroller
// package qtwidgets
// /usr/include/qt/QtWidgets/qscroller.h
// #include <qscroller.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 9
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
#[derive(Default)] // class sizeof(QScroller)=24
pub struct QScroller {
  qbase: QObject,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QScroller_ITF interface {
//    qtcore.QObject_ITF
//    QScroller_PTR() *QScroller
//}
//func (ptr *QScroller) QScroller_PTR() *QScroller { return ptr }

impl /*struct*/ QScroller {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QScroller {
    return QScroller{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QScroller {
//  type Target = QScrollerBASE;
//
//  fn deref(&self) -> &QScrollerBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QScrollerBASE> for QScroller {
//  fn as_ref(& self) -> & QScrollerBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qscroller.h:63
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QScroller {
  pub fn metaObject_0<RetType, T: QScroller_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QScroller_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QScroller) -> RetType;
}
impl<'a> /*trait*/ QScroller_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QScroller) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QScroller10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qscroller.h:92
// index:0
// Public static Visibility=Default Availability=Available
// [1] bool hasScroller(QObject *)

/*
Returns true if a QScroller object was already created for target; false otherwise.

See also scroller().
*/
impl /*struct*/ QScroller {
  pub fn hasScroller_0<RetType, T: QScroller_hasScroller_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.hasScroller_0();
    // return 1;
  }
}
pub trait QScroller_hasScroller_0<RetType> {
  fn hasScroller_0(self ) -> RetType;
}
impl<'a> /*trait*/ QScroller_hasScroller_0<bool> for (usize) {
  fn hasScroller_0(self ) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QScroller11hasScrollerEP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qscroller.h:94
// index:0
// Public static Visibility=Default Availability=Available
// [8] QScroller * scroller(QObject *)

/*
Returns the scroller for the given target. As long as the object exists this function will always return the same QScroller instance. If no QScroller exists for the target, one will implicitly be created. At no point more than one QScroller will be active on an object.

See also hasScroller() and target().
*/
impl /*struct*/ QScroller {
  pub fn scroller_0<RetType, T: QScroller_scroller_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.scroller_0();
    // return 1;
  }
}
pub trait QScroller_scroller_0<RetType> {
  fn scroller_0(self ) -> RetType;
}
impl<'a> /*trait*/ QScroller_scroller_0<usize> for (usize) {
  fn scroller_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QScroller8scrollerEP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qscroller.h:95
// index:1
// Public static Visibility=Default Availability=Available
// [8] const QScroller * scroller(const QObject *)

/*
Returns the scroller for the given target. As long as the object exists this function will always return the same QScroller instance. If no QScroller exists for the target, one will implicitly be created. At no point more than one QScroller will be active on an object.

See also hasScroller() and target().
*/
impl /*struct*/ QScroller {
  pub fn scroller_1<RetType, T: QScroller_scroller_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.scroller_1();
    // return 1;
  }
}
pub trait QScroller_scroller_1<RetType> {
  fn scroller_1(self ) -> RetType;
}
impl<'a> /*trait*/ QScroller_scroller_1<usize> for (usize) {
  fn scroller_1(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QScroller8scrollerEPK7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qscroller.h:98
// index:0
// Public static Visibility=Default Availability=Available
// [4] Qt::GestureType grabGesture(QObject *, QScroller::ScrollerGestureType)

/*
Registers a custom scroll gesture recognizer, grabs it for the target and returns the resulting gesture type. If scrollGestureType is set to TouchGesture the gesture triggers on touch events. If it is set to one of LeftMouseButtonGesture, RightMouseButtonGesture or MiddleMouseButtonGesture it triggers on mouse events of the corresponding button.

Only one scroll gesture can be active on a single object at the same time. If you call this function twice on the same object, it will ungrab the existing gesture before grabbing the new one.

Note: To avoid unwanted side-effects, mouse events are consumed while the gesture is triggered. Since the initial mouse press event is not consumed, the gesture sends a fake mouse release event at the global position (INT_MIN, INT_MIN). This ensures that internal states of the widget that received the original mouse press are consistent.

See also ungrabGesture() and grabbedGesture().
*/
impl /*struct*/ QScroller {
  pub fn grabGesture_0<RetType, T: QScroller_grabGesture_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.grabGesture_0();
    // return 1;
  }
}
pub trait QScroller_grabGesture_0<RetType> {
  fn grabGesture_0(self ) -> RetType;
}
impl<'a> /*trait*/ QScroller_grabGesture_0<i32> for (usize,i32) {
  fn grabGesture_0(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QScroller11grabGestureEP7QObjectNS_19ScrollerGestureTypeE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qscroller.h:99
// index:0
// Public static Visibility=Default Availability=Available
// [4] Qt::GestureType grabbedGesture(QObject *)

/*
Returns the gesture type currently grabbed for the target or 0 if no gesture is grabbed.

See also grabGesture() and ungrabGesture().
*/
impl /*struct*/ QScroller {
  pub fn grabbedGesture_0<RetType, T: QScroller_grabbedGesture_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.grabbedGesture_0();
    // return 1;
  }
}
pub trait QScroller_grabbedGesture_0<RetType> {
  fn grabbedGesture_0(self ) -> RetType;
}
impl<'a> /*trait*/ QScroller_grabbedGesture_0<i32> for (usize) {
  fn grabbedGesture_0(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QScroller14grabbedGestureEP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qscroller.h:100
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void ungrabGesture(QObject *)

/*
Ungrabs the gesture for the target. Does nothing if no gesture is grabbed.

See also grabGesture() and grabbedGesture().
*/
impl /*struct*/ QScroller {
  pub fn ungrabGesture_0<RetType, T: QScroller_ungrabGesture_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.ungrabGesture_0();
    // return 1;
  }
}
pub trait QScroller_ungrabGesture_0<RetType> {
  fn ungrabGesture_0(self ) -> RetType;
}
impl<'a> /*trait*/ QScroller_ungrabGesture_0<(/*void*/)> for (usize) {
  fn ungrabGesture_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QScroller13ungrabGestureEP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qscroller.h:105
// index:0
// Public Visibility=Default Availability=Available
// [8] QObject * target() const

/*
Returns the target object of this scroller.

See also hasScroller() and scroller().
*/
impl /*struct*/ QScroller {
  pub fn target_0<RetType, T: QScroller_target_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.target_0(self);
    // return 1;
  }
}
pub trait QScroller_target_0<RetType> {
  fn target_0(self , rsthis: & QScroller) -> RetType;
}
impl<'a> /*trait*/ QScroller_target_0<usize> for () {
  fn target_0(self , rsthis: & QScroller) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QScroller6targetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qscroller.h:107
// index:0
// Public Visibility=Default Availability=Available
// [4] QScroller::State state() const

/*

*/
impl /*struct*/ QScroller {
  pub fn state_0<RetType, T: QScroller_state_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.state_0(self);
    // return 1;
  }
}
pub trait QScroller_state_0<RetType> {
  fn state_0(self , rsthis: & QScroller) -> RetType;
}
impl<'a> /*trait*/ QScroller_state_0<i32> for () {
  fn state_0(self , rsthis: & QScroller) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QScroller5stateEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qscroller.h:109
// index:0
// Public Visibility=Default Availability=Available
// [1] bool handleInput(QScroller::Input, const QPointF &, qint64)

/*
This function is used by gesture recognizers to inform the scroller about a new input event. The scroller changes its internal state() according to the input event and its attached scroller properties. The scroller doesn't distinguish between the kind of input device the event came from. Therefore the event needs to be split into the input type, a position and a milli-second timestamp. The position needs to be in the target's coordinate system.

The return value is true if the event should be consumed by the calling filter or false if the event should be forwarded to the control.

Note: Using grabGesture() should be sufficient for most use cases.
*/
impl /*struct*/ QScroller {
  pub fn handleInput_0<RetType, T: QScroller_handleInput_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.handleInput_0(self);
    // return 1;
  }
}
pub trait QScroller_handleInput_0<RetType> {
  fn handleInput_0(self , rsthis: & QScroller) -> RetType;
}
impl<'a> /*trait*/ QScroller_handleInput_0<bool> for (i32,usize,i64) {
  fn handleInput_0(self , rsthis: & QScroller) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const i64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QScroller11handleInputENS_5InputERK7QPointFx", 3,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_SINT64,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qscroller.h:111
// index:0
// Public Visibility=Default Availability=Available
// [-2] void stop()

/*
Stops the scroller and resets its state back to Inactive.
*/
impl /*struct*/ QScroller {
  pub fn stop_0<RetType, T: QScroller_stop_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.stop_0(self);
    // return 1;
  }
}
pub trait QScroller_stop_0<RetType> {
  fn stop_0(self , rsthis: & QScroller) -> RetType;
}
impl<'a> /*trait*/ QScroller_stop_0<(/*void*/)> for () {
  fn stop_0(self , rsthis: & QScroller) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN9QScroller4stopEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qscroller.h:112
// index:0
// Public Visibility=Default Availability=Available
// [16] QPointF velocity() const

/*
Returns the current scrolling velocity in meter per second when the state is Scrolling or Dragging. Returns a zero velocity otherwise.

The velocity is reported for both the x and y axis separately by using a QPointF.

See also pixelPerMeter().
*/
impl /*struct*/ QScroller {
  pub fn velocity_0<RetType, T: QScroller_velocity_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.velocity_0(self);
    // return 1;
  }
}
pub trait QScroller_velocity_0<RetType> {
  fn velocity_0(self , rsthis: & QScroller) -> RetType;
}
impl<'a> /*trait*/ QScroller_velocity_0<usize> for () {
  fn velocity_0(self , rsthis: & QScroller) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QScroller8velocityEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qscroller.h:113
// index:0
// Public Visibility=Default Availability=Available
// [16] QPointF finalPosition() const

/*
Returns the estimated final position for the current scroll movement. Returns the current position if the scroller state is not Scrolling. The result is undefined when the scroller state is Inactive.

The target position is in pixel.

See also pixelPerMeter() and scrollTo().
*/
impl /*struct*/ QScroller {
  pub fn finalPosition_0<RetType, T: QScroller_finalPosition_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.finalPosition_0(self);
    // return 1;
  }
}
pub trait QScroller_finalPosition_0<RetType> {
  fn finalPosition_0(self , rsthis: & QScroller) -> RetType;
}
impl<'a> /*trait*/ QScroller_finalPosition_0<usize> for () {
  fn finalPosition_0(self , rsthis: & QScroller) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QScroller13finalPositionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qscroller.h:114
// index:0
// Public Visibility=Default Availability=Available
// [16] QPointF pixelPerMeter() const

/*
Returns the pixel per meter metric for the scrolled widget.

The value is reported for both the x and y axis separately by using a QPointF.

Note: Please note that this value should be physically correct. The actual DPI settings that Qt returns for the display may be reported wrongly on purpose by the underlying windowing system, for example on macOS.
*/
impl /*struct*/ QScroller {
  pub fn pixelPerMeter_0<RetType, T: QScroller_pixelPerMeter_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.pixelPerMeter_0(self);
    // return 1;
  }
}
pub trait QScroller_pixelPerMeter_0<RetType> {
  fn pixelPerMeter_0(self , rsthis: & QScroller) -> RetType;
}
impl<'a> /*trait*/ QScroller_pixelPerMeter_0<usize> for () {
  fn pixelPerMeter_0(self , rsthis: & QScroller) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QScroller13pixelPerMeterEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qscroller.h:116
// index:0
// Public Visibility=Default Availability=Available
// [16] QScrollerProperties scrollerProperties() const

/*

*/
impl /*struct*/ QScroller {
  pub fn scrollerProperties_0<RetType, T: QScroller_scrollerProperties_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.scrollerProperties_0(self);
    // return 1;
  }
}
pub trait QScroller_scrollerProperties_0<RetType> {
  fn scrollerProperties_0(self , rsthis: & QScroller) -> RetType;
}
impl<'a> /*trait*/ QScroller_scrollerProperties_0<usize> for () {
  fn scrollerProperties_0(self , rsthis: & QScroller) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QScroller18scrollerPropertiesEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qscroller.h:119
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setSnapPositionsX(qreal, qreal)

/*
Set the snap positions for the horizontal axis to a list of positions. This overwrites all previously set snap positions and also a previously set snapping interval. Snapping can be deactivated by setting an empty list of positions.
*/
impl /*struct*/ QScroller {
  pub fn setSnapPositionsX_0<RetType, T: QScroller_setSnapPositionsX_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSnapPositionsX_0(self);
    // return 1;
  }
}
pub trait QScroller_setSnapPositionsX_0<RetType> {
  fn setSnapPositionsX_0(self , rsthis: & QScroller) -> RetType;
}
impl<'a> /*trait*/ QScroller_setSnapPositionsX_0<(/*void*/)> for (f64,f64) {
  fn setSnapPositionsX_0(self , rsthis: & QScroller) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN9QScroller17setSnapPositionsXEdd", 2,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qscroller.h:121
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setSnapPositionsY(qreal, qreal)

/*
Set the snap positions for the vertical axis to a list of positions. This overwrites all previously set snap positions and also a previously set snapping interval. Snapping can be deactivated by setting an empty list of positions.
*/
impl /*struct*/ QScroller {
  pub fn setSnapPositionsY_0<RetType, T: QScroller_setSnapPositionsY_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSnapPositionsY_0(self);
    // return 1;
  }
}
pub trait QScroller_setSnapPositionsY_0<RetType> {
  fn setSnapPositionsY_0(self , rsthis: & QScroller) -> RetType;
}
impl<'a> /*trait*/ QScroller_setSnapPositionsY_0<(/*void*/)> for (f64,f64) {
  fn setSnapPositionsY_0(self , rsthis: & QScroller) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN9QScroller17setSnapPositionsYEdd", 2,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qscroller.h:124
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setScrollerProperties(const QScrollerProperties &)

/*

*/
impl /*struct*/ QScroller {
  pub fn setScrollerProperties_0<RetType, T: QScroller_setScrollerProperties_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setScrollerProperties_0(self);
    // return 1;
  }
}
pub trait QScroller_setScrollerProperties_0<RetType> {
  fn setScrollerProperties_0(self , rsthis: & QScroller) -> RetType;
}
impl<'a> /*trait*/ QScroller_setScrollerProperties_0<(/*void*/)> for (usize) {
  fn setScrollerProperties_0(self , rsthis: & QScroller) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QScroller21setScrollerPropertiesERK19QScrollerProperties", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qscroller.h:125
// index:0
// Public Visibility=Default Availability=Available
// [-2] void scrollTo(const QPointF &)

/*
Starts scrolling the widget so that point pos is at the top-left position in the viewport.

The behaviour when scrolling outside the valid scroll area is undefined. In this case the scroller might or might not overshoot.

The scrolling speed will be calculated so that the given position will be reached after a platform-defined time span.

pos is given in viewport coordinates.

See also ensureVisible().
*/
impl /*struct*/ QScroller {
  pub fn scrollTo_0<RetType, T: QScroller_scrollTo_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.scrollTo_0(self);
    // return 1;
  }
}
pub trait QScroller_scrollTo_0<RetType> {
  fn scrollTo_0(self , rsthis: & QScroller) -> RetType;
}
impl<'a> /*trait*/ QScroller_scrollTo_0<(/*void*/)> for (usize) {
  fn scrollTo_0(self , rsthis: & QScroller) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QScroller8scrollToERK7QPointF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qscroller.h:126
// index:1
// Public Visibility=Default Availability=Available
// [-2] void scrollTo(const QPointF &, int)

/*
Starts scrolling the widget so that point pos is at the top-left position in the viewport.

The behaviour when scrolling outside the valid scroll area is undefined. In this case the scroller might or might not overshoot.

The scrolling speed will be calculated so that the given position will be reached after a platform-defined time span.

pos is given in viewport coordinates.

See also ensureVisible().
*/
impl /*struct*/ QScroller {
  pub fn scrollTo_1<RetType, T: QScroller_scrollTo_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.scrollTo_1(self);
    // return 1;
  }
}
pub trait QScroller_scrollTo_1<RetType> {
  fn scrollTo_1(self , rsthis: & QScroller) -> RetType;
}
impl<'a> /*trait*/ QScroller_scrollTo_1<(/*void*/)> for (usize,i32) {
  fn scrollTo_1(self , rsthis: & QScroller) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QScroller8scrollToERK7QPointFi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qscroller.h:127
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ensureVisible(const QRectF &, qreal, qreal)

/*
Starts scrolling so that the rectangle rect is visible inside the viewport with additional margins specified in pixels by xmargin and ymargin around the rect.

In cases where it is not possible to fit the rect plus margins inside the viewport the contents are scrolled so that as much as possible is visible from rect.

The scrolling speed is calculated so that the given position is reached after a platform-defined time span.

This function performs the actual scrolling by calling scrollTo().

See also scrollTo().
*/
impl /*struct*/ QScroller {
  pub fn ensureVisible_0<RetType, T: QScroller_ensureVisible_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.ensureVisible_0(self);
    // return 1;
  }
}
pub trait QScroller_ensureVisible_0<RetType> {
  fn ensureVisible_0(self , rsthis: & QScroller) -> RetType;
}
impl<'a> /*trait*/ QScroller_ensureVisible_0<(/*void*/)> for (usize,f64,f64) {
  fn ensureVisible_0(self , rsthis: & QScroller) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let arg2 = (&self.2) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN9QScroller13ensureVisibleERK6QRectFdd", 3,qtrt::FFITY_POINTER,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qscroller.h:128
// index:1
// Public Visibility=Default Availability=Available
// [-2] void ensureVisible(const QRectF &, qreal, qreal, int)

/*
Starts scrolling so that the rectangle rect is visible inside the viewport with additional margins specified in pixels by xmargin and ymargin around the rect.

In cases where it is not possible to fit the rect plus margins inside the viewport the contents are scrolled so that as much as possible is visible from rect.

The scrolling speed is calculated so that the given position is reached after a platform-defined time span.

This function performs the actual scrolling by calling scrollTo().

See also scrollTo().
*/
impl /*struct*/ QScroller {
  pub fn ensureVisible_1<RetType, T: QScroller_ensureVisible_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.ensureVisible_1(self);
    // return 1;
  }
}
pub trait QScroller_ensureVisible_1<RetType> {
  fn ensureVisible_1(self , rsthis: & QScroller) -> RetType;
}
impl<'a> /*trait*/ QScroller_ensureVisible_1<(/*void*/)> for (usize,f64,f64,i32) {
  fn ensureVisible_1(self , rsthis: & QScroller) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let arg2 = (&self.2) as *const f64 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QScroller13ensureVisibleERK6QRectFddi", 4,qtrt::FFITY_POINTER,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qscroller.h:129
// index:0
// Public Visibility=Default Availability=Available
// [-2] void resendPrepareEvent()

/*
This function resends the QScrollPrepareEvent. Calling resendPrepareEvent triggers a QScrollPrepareEvent from the scroller. This allows the receiver to re-set content position and content size while scrolling. Calling this function while in the Inactive state is useless as the prepare event is sent again before scrolling starts.
*/
impl /*struct*/ QScroller {
  pub fn resendPrepareEvent_0<RetType, T: QScroller_resendPrepareEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resendPrepareEvent_0(self);
    // return 1;
  }
}
pub trait QScroller_resendPrepareEvent_0<RetType> {
  fn resendPrepareEvent_0(self , rsthis: & QScroller) -> RetType;
}
impl<'a> /*trait*/ QScroller_resendPrepareEvent_0<(/*void*/)> for () {
  fn resendPrepareEvent_0(self , rsthis: & QScroller) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN9QScroller18resendPrepareEventEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qscroller.h:132
// index:0
// Public Visibility=Default Availability=Available
// [-2] void stateChanged(QScroller::State)

/*
QScroller emits this signal whenever the state changes. newState is the new State.

Note: Notifier signal for property state. 

See also state.
*/
impl /*struct*/ QScroller {
  pub fn stateChanged_0<RetType, T: QScroller_stateChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.stateChanged_0(self);
    // return 1;
  }
}
pub trait QScroller_stateChanged_0<RetType> {
  fn stateChanged_0(self , rsthis: & QScroller) -> RetType;
}
impl<'a> /*trait*/ QScroller_stateChanged_0<(/*void*/)> for (i32) {
  fn stateChanged_0(self , rsthis: & QScroller) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QScroller12stateChangedENS_5StateE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qscroller.h:133
// index:0
// Public Visibility=Default Availability=Available
// [-2] void scrollerPropertiesChanged(const QScrollerProperties &)

/*
QScroller emits this signal whenever its scroller properties change. newProperties are the new scroller properties.

Note: Notifier signal for property scrollerProperties. 

See also scrollerProperties.
*/
impl /*struct*/ QScroller {
  pub fn scrollerPropertiesChanged_0<RetType, T: QScroller_scrollerPropertiesChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.scrollerPropertiesChanged_0(self);
    // return 1;
  }
}
pub trait QScroller_scrollerPropertiesChanged_0<RetType> {
  fn scrollerPropertiesChanged_0(self , rsthis: & QScroller) -> RetType;
}
impl<'a> /*trait*/ QScroller_scrollerPropertiesChanged_0<(/*void*/)> for (usize) {
  fn scrollerPropertiesChanged_0(self , rsthis: & QScroller) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QScroller25scrollerPropertiesChangedERK19QScrollerProperties", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}


pub fn DeleteQScroller(this :*mut QScroller) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN9QScrollerD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}

/*
This enum contains the different QScroller states.


*/
pub type QScroller__State = i32;
// The scroller is not scrolling and nothing is pressed.
pub const QScroller__Inactive :QScroller__State = 0;
// A touch event was received or the mouse button was pressed but the scroll area is currently not dragged.
pub const QScroller__Pressed :QScroller__State = 1;
// The scroll area is currently following the touch point or mouse.
pub const QScroller__Dragging :QScroller__State = 2;
// The scroll area is moving on it's own.
pub const QScroller__Scrolling :QScroller__State = 3;
pub fn QScroller_StateItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QScroller", val);
}
pub fn QScroller_StateItemName_s(val: i32) ->String {
  //var nilthis *QScroller
  //return nilthis.StateItemName(val);
  return QScroller_StateItemName(val);
}


/*
This enum contains the different gesture types that are supported by the QScroller gesture recognizer.


*/
pub type QScroller__ScrollerGestureType = i32;
// The gesture recognizer will only trigger on touch events. Specifically it will react on single touch points when using a touch screen and dual touch points when using a touchpad.
pub const QScroller__TouchGesture :QScroller__ScrollerGestureType = 0;
// The gesture recognizer will only trigger on left mouse button events.
pub const QScroller__LeftMouseButtonGesture :QScroller__ScrollerGestureType = 1;
// The gesture recognizer will only trigger on right mouse button events.
pub const QScroller__RightMouseButtonGesture :QScroller__ScrollerGestureType = 2;
// The gesture recognizer will only trigger on middle mouse button events.
pub const QScroller__MiddleMouseButtonGesture :QScroller__ScrollerGestureType = 3;
pub fn QScroller_ScrollerGestureTypeItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QScroller", val);
}
pub fn QScroller_ScrollerGestureTypeItemName_s(val: i32) ->String {
  //var nilthis *QScroller
  //return nilthis.ScrollerGestureTypeItemName(val);
  return QScroller_ScrollerGestureTypeItemName(val);
}


/*
This enum contains an input device agnostic view of input events that are relevant for QScroller.


*/
pub type QScroller__Input = i32;
// The user pressed the input device (e.g. QEvent::MouseButtonPress, QEvent::GraphicsSceneMousePress, QEvent::TouchBegin)
pub const QScroller__InputPress :QScroller__Input = 1;
// The user moved the input device (e.g. QEvent::MouseMove, QEvent::GraphicsSceneMouseMove, QEvent::TouchUpdate)
pub const QScroller__InputMove :QScroller__Input = 2;
// The user released the input device (e.g. QEvent::MouseButtonRelease, QEvent::GraphicsSceneMouseRelease, QEvent::TouchEnd)
pub const QScroller__InputRelease :QScroller__Input = 3;
pub fn QScroller_InputItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QScroller", val);
}
pub fn QScroller_InputItemName_s(val: i32) ->String {
  //var nilthis *QScroller
  //return nilthis.InputItemName(val);
  return QScroller_InputItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
