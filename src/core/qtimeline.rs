

// mod ::core::QTimeLine
// package qtcore
// /usr/include/qt/QtCore/qtimeline.h
// #include <qtimeline.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 4
*/
// import "C"
// import "unsafe"
// import "reflect"
// import "fmt"
// import "log"
// import "github.com/kitech/qt.go/qtrt"
use std::default::Default;
use std::ops::Deref;
use super::super::qtrt;
use super::*;
//  ext block end

//  body block begin

// void timerEvent(QTimerEvent *)
// func (this *QTimeLine) InheritTimerEvent(f func(event *QTimerEvent/*777 QTimerEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "timerEvent", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QTimeLine)=16
pub struct QTimeLine {
  qbase: QObject,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QTimeLine_ITF interface {
//    QObject_ITF
//    QTimeLine_PTR() *QTimeLine
//}
//func (ptr *QTimeLine) QTimeLine_PTR() *QTimeLine { return ptr }

impl /*struct*/ QTimeLine {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QTimeLine {
    return QTimeLine{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QTimeLine {
//  type Target = QTimeLineBASE;
//
//  fn deref(&self) -> &QTimeLineBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QTimeLineBASE> for QTimeLine {
//  fn as_ref(& self) -> & QTimeLineBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qtimeline.h:52
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QTimeLine {
  pub fn metaObject_0<RetType, T: QTimeLine_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QTimeLine_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QTimeLine) -> RetType;
}
impl<'a> /*trait*/ QTimeLine_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QTimeLine) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTimeLine10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtimeline.h:79
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QTimeLine(int, QObject *)

/*
Constructs a timeline with a duration of duration milliseconds. parent is passed to QObject's constructor. The default duration is 1000 milliseconds.
*/
// QTimeLine(int, QObject *) ctx.fn_proto_cpp
impl /*struct*/ QTimeLine {
  pub fn QTimeLine_0<T: QTimeLine_QTimeLine_0>(value: T) -> QTimeLine {
    let rsthis = value.QTimeLine_0();
    return rsthis;
    // return 1;
  }
}

pub trait QTimeLine_QTimeLine_0 {
  fn QTimeLine_0(self) -> QTimeLine;
}
// QTimeLine(int, QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QTimeLine_QTimeLine_0 for (i32,usize) {
  fn QTimeLine_0(self) -> QTimeLine {
    // unsafe{_ZN9QTimeLineC2EiP7QObject()};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QTimeLineC2EiP7QObject", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTimeLine{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qtimeline.h:80
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QTimeLine()

/*

*/
pub fn DeleteQTimeLine(this :*mut QTimeLine) {
    // let rv = qtrt::InvokeQtFunc6("_ZN9QTimeLineD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qtimeline.h:82
// index:0
// Public Visibility=Default Availability=Available
// [4] QTimeLine::State state() const

/*
Returns the state of the timeline.

See also start(), setPaused(), and stop().
*/
impl /*struct*/ QTimeLine {
  pub fn state_0<RetType, T: QTimeLine_state_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.state_0(self);
    // return 1;
  }
}
pub trait QTimeLine_state_0<RetType> {
  fn state_0(self , rsthis: & QTimeLine) -> RetType;
}
impl<'a> /*trait*/ QTimeLine_state_0<i32> for () {
  fn state_0(self , rsthis: & QTimeLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTimeLine5stateEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtimeline.h:84
// index:0
// Public Visibility=Default Availability=Available
// [4] int loopCount() const

/*

*/
impl /*struct*/ QTimeLine {
  pub fn loopCount_0<RetType, T: QTimeLine_loopCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.loopCount_0(self);
    // return 1;
  }
}
pub trait QTimeLine_loopCount_0<RetType> {
  fn loopCount_0(self , rsthis: & QTimeLine) -> RetType;
}
impl<'a> /*trait*/ QTimeLine_loopCount_0<i32> for () {
  fn loopCount_0(self , rsthis: & QTimeLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTimeLine9loopCountEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtimeline.h:85
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setLoopCount(int)

/*

*/
impl /*struct*/ QTimeLine {
  pub fn setLoopCount_0<RetType, T: QTimeLine_setLoopCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setLoopCount_0(self);
    // return 1;
  }
}
pub trait QTimeLine_setLoopCount_0<RetType> {
  fn setLoopCount_0(self , rsthis: & QTimeLine) -> RetType;
}
impl<'a> /*trait*/ QTimeLine_setLoopCount_0<(/*void*/)> for (i32) {
  fn setLoopCount_0(self , rsthis: & QTimeLine) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QTimeLine12setLoopCountEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qtimeline.h:87
// index:0
// Public Visibility=Default Availability=Available
// [4] QTimeLine::Direction direction() const

/*

*/
impl /*struct*/ QTimeLine {
  pub fn direction_0<RetType, T: QTimeLine_direction_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.direction_0(self);
    // return 1;
  }
}
pub trait QTimeLine_direction_0<RetType> {
  fn direction_0(self , rsthis: & QTimeLine) -> RetType;
}
impl<'a> /*trait*/ QTimeLine_direction_0<i32> for () {
  fn direction_0(self , rsthis: & QTimeLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTimeLine9directionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtimeline.h:88
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDirection(QTimeLine::Direction)

/*

*/
impl /*struct*/ QTimeLine {
  pub fn setDirection_0<RetType, T: QTimeLine_setDirection_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDirection_0(self);
    // return 1;
  }
}
pub trait QTimeLine_setDirection_0<RetType> {
  fn setDirection_0(self , rsthis: & QTimeLine) -> RetType;
}
impl<'a> /*trait*/ QTimeLine_setDirection_0<(/*void*/)> for (i32) {
  fn setDirection_0(self , rsthis: & QTimeLine) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QTimeLine12setDirectionENS_9DirectionE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qtimeline.h:90
// index:0
// Public Visibility=Default Availability=Available
// [4] int duration() const

/*

*/
impl /*struct*/ QTimeLine {
  pub fn duration_0<RetType, T: QTimeLine_duration_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.duration_0(self);
    // return 1;
  }
}
pub trait QTimeLine_duration_0<RetType> {
  fn duration_0(self , rsthis: & QTimeLine) -> RetType;
}
impl<'a> /*trait*/ QTimeLine_duration_0<i32> for () {
  fn duration_0(self , rsthis: & QTimeLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTimeLine8durationEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtimeline.h:91
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDuration(int)

/*

*/
impl /*struct*/ QTimeLine {
  pub fn setDuration_0<RetType, T: QTimeLine_setDuration_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDuration_0(self);
    // return 1;
  }
}
pub trait QTimeLine_setDuration_0<RetType> {
  fn setDuration_0(self , rsthis: & QTimeLine) -> RetType;
}
impl<'a> /*trait*/ QTimeLine_setDuration_0<(/*void*/)> for (i32) {
  fn setDuration_0(self , rsthis: & QTimeLine) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QTimeLine11setDurationEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qtimeline.h:93
// index:0
// Public Visibility=Default Availability=Available
// [4] int startFrame() const

/*
Returns the start frame, which is the frame corresponding to the start of the timeline (i.e., the frame for which the current value is 0).

See also setStartFrame() and setFrameRange().
*/
impl /*struct*/ QTimeLine {
  pub fn startFrame_0<RetType, T: QTimeLine_startFrame_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.startFrame_0(self);
    // return 1;
  }
}
pub trait QTimeLine_startFrame_0<RetType> {
  fn startFrame_0(self , rsthis: & QTimeLine) -> RetType;
}
impl<'a> /*trait*/ QTimeLine_startFrame_0<i32> for () {
  fn startFrame_0(self , rsthis: & QTimeLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTimeLine10startFrameEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtimeline.h:94
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setStartFrame(int)

/*
Sets the start frame, which is the frame corresponding to the start of the timeline (i.e., the frame for which the current value is 0), to frame.

See also startFrame(), endFrame(), and setFrameRange().
*/
impl /*struct*/ QTimeLine {
  pub fn setStartFrame_0<RetType, T: QTimeLine_setStartFrame_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setStartFrame_0(self);
    // return 1;
  }
}
pub trait QTimeLine_setStartFrame_0<RetType> {
  fn setStartFrame_0(self , rsthis: & QTimeLine) -> RetType;
}
impl<'a> /*trait*/ QTimeLine_setStartFrame_0<(/*void*/)> for (i32) {
  fn setStartFrame_0(self , rsthis: & QTimeLine) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QTimeLine13setStartFrameEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qtimeline.h:95
// index:0
// Public Visibility=Default Availability=Available
// [4] int endFrame() const

/*
Returns the end frame, which is the frame corresponding to the end of the timeline (i.e., the frame for which the current value is 1).

See also setEndFrame() and setFrameRange().
*/
impl /*struct*/ QTimeLine {
  pub fn endFrame_0<RetType, T: QTimeLine_endFrame_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.endFrame_0(self);
    // return 1;
  }
}
pub trait QTimeLine_endFrame_0<RetType> {
  fn endFrame_0(self , rsthis: & QTimeLine) -> RetType;
}
impl<'a> /*trait*/ QTimeLine_endFrame_0<i32> for () {
  fn endFrame_0(self , rsthis: & QTimeLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTimeLine8endFrameEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtimeline.h:96
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setEndFrame(int)

/*
Sets the end frame, which is the frame corresponding to the end of the timeline (i.e., the frame for which the current value is 1), to frame.

See also endFrame(), startFrame(), and setFrameRange().
*/
impl /*struct*/ QTimeLine {
  pub fn setEndFrame_0<RetType, T: QTimeLine_setEndFrame_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setEndFrame_0(self);
    // return 1;
  }
}
pub trait QTimeLine_setEndFrame_0<RetType> {
  fn setEndFrame_0(self , rsthis: & QTimeLine) -> RetType;
}
impl<'a> /*trait*/ QTimeLine_setEndFrame_0<(/*void*/)> for (i32) {
  fn setEndFrame_0(self , rsthis: & QTimeLine) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QTimeLine11setEndFrameEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qtimeline.h:97
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFrameRange(int, int)

/*
Sets the timeline's frame counter to start at startFrame, and end and endFrame. For each time value, QTimeLine will find the corresponding frame when you call currentFrame() or frameForTime() by interpolating, using the return value of valueForTime().

When in Running state, QTimeLine also emits the frameChanged() signal when the frame changes.

See also startFrame(), endFrame(), start(), and currentFrame().
*/
impl /*struct*/ QTimeLine {
  pub fn setFrameRange_0<RetType, T: QTimeLine_setFrameRange_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFrameRange_0(self);
    // return 1;
  }
}
pub trait QTimeLine_setFrameRange_0<RetType> {
  fn setFrameRange_0(self , rsthis: & QTimeLine) -> RetType;
}
impl<'a> /*trait*/ QTimeLine_setFrameRange_0<(/*void*/)> for (i32,i32) {
  fn setFrameRange_0(self , rsthis: & QTimeLine) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QTimeLine13setFrameRangeEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qtimeline.h:99
// index:0
// Public Visibility=Default Availability=Available
// [4] int updateInterval() const

/*

*/
impl /*struct*/ QTimeLine {
  pub fn updateInterval_0<RetType, T: QTimeLine_updateInterval_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.updateInterval_0(self);
    // return 1;
  }
}
pub trait QTimeLine_updateInterval_0<RetType> {
  fn updateInterval_0(self , rsthis: & QTimeLine) -> RetType;
}
impl<'a> /*trait*/ QTimeLine_updateInterval_0<i32> for () {
  fn updateInterval_0(self , rsthis: & QTimeLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTimeLine14updateIntervalEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtimeline.h:100
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setUpdateInterval(int)

/*

*/
impl /*struct*/ QTimeLine {
  pub fn setUpdateInterval_0<RetType, T: QTimeLine_setUpdateInterval_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setUpdateInterval_0(self);
    // return 1;
  }
}
pub trait QTimeLine_setUpdateInterval_0<RetType> {
  fn setUpdateInterval_0(self , rsthis: & QTimeLine) -> RetType;
}
impl<'a> /*trait*/ QTimeLine_setUpdateInterval_0<(/*void*/)> for (i32) {
  fn setUpdateInterval_0(self , rsthis: & QTimeLine) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QTimeLine17setUpdateIntervalEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qtimeline.h:102
// index:0
// Public Visibility=Default Availability=Available
// [4] QTimeLine::CurveShape curveShape() const

/*

*/
impl /*struct*/ QTimeLine {
  pub fn curveShape_0<RetType, T: QTimeLine_curveShape_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.curveShape_0(self);
    // return 1;
  }
}
pub trait QTimeLine_curveShape_0<RetType> {
  fn curveShape_0(self , rsthis: & QTimeLine) -> RetType;
}
impl<'a> /*trait*/ QTimeLine_curveShape_0<i32> for () {
  fn curveShape_0(self , rsthis: & QTimeLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTimeLine10curveShapeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtimeline.h:103
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCurveShape(QTimeLine::CurveShape)

/*

*/
impl /*struct*/ QTimeLine {
  pub fn setCurveShape_0<RetType, T: QTimeLine_setCurveShape_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCurveShape_0(self);
    // return 1;
  }
}
pub trait QTimeLine_setCurveShape_0<RetType> {
  fn setCurveShape_0(self , rsthis: & QTimeLine) -> RetType;
}
impl<'a> /*trait*/ QTimeLine_setCurveShape_0<(/*void*/)> for (i32) {
  fn setCurveShape_0(self , rsthis: & QTimeLine) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QTimeLine13setCurveShapeENS_10CurveShapeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qtimeline.h:105
// index:0
// Public Visibility=Default Availability=Available
// [8] QEasingCurve easingCurve() const

/*

*/
impl /*struct*/ QTimeLine {
  pub fn easingCurve_0<RetType, T: QTimeLine_easingCurve_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.easingCurve_0(self);
    // return 1;
  }
}
pub trait QTimeLine_easingCurve_0<RetType> {
  fn easingCurve_0(self , rsthis: & QTimeLine) -> RetType;
}
impl<'a> /*trait*/ QTimeLine_easingCurve_0<usize> for () {
  fn easingCurve_0(self , rsthis: & QTimeLine) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTimeLine11easingCurveEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtimeline.h:106
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setEasingCurve(const QEasingCurve &)

/*

*/
impl /*struct*/ QTimeLine {
  pub fn setEasingCurve_0<RetType, T: QTimeLine_setEasingCurve_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setEasingCurve_0(self);
    // return 1;
  }
}
pub trait QTimeLine_setEasingCurve_0<RetType> {
  fn setEasingCurve_0(self , rsthis: & QTimeLine) -> RetType;
}
impl<'a> /*trait*/ QTimeLine_setEasingCurve_0<(/*void*/)> for (usize) {
  fn setEasingCurve_0(self , rsthis: & QTimeLine) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QTimeLine14setEasingCurveERK12QEasingCurve", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qtimeline.h:108
// index:0
// Public Visibility=Default Availability=Available
// [4] int currentTime() const

/*

*/
impl /*struct*/ QTimeLine {
  pub fn currentTime_0<RetType, T: QTimeLine_currentTime_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.currentTime_0(self);
    // return 1;
  }
}
pub trait QTimeLine_currentTime_0<RetType> {
  fn currentTime_0(self , rsthis: & QTimeLine) -> RetType;
}
impl<'a> /*trait*/ QTimeLine_currentTime_0<i32> for () {
  fn currentTime_0(self , rsthis: & QTimeLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTimeLine11currentTimeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtimeline.h:109
// index:0
// Public Visibility=Default Availability=Available
// [4] int currentFrame() const

/*
Returns the frame corresponding to the current time.

See also currentTime(), frameForTime(), and setFrameRange().
*/
impl /*struct*/ QTimeLine {
  pub fn currentFrame_0<RetType, T: QTimeLine_currentFrame_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.currentFrame_0(self);
    // return 1;
  }
}
pub trait QTimeLine_currentFrame_0<RetType> {
  fn currentFrame_0(self , rsthis: & QTimeLine) -> RetType;
}
impl<'a> /*trait*/ QTimeLine_currentFrame_0<i32> for () {
  fn currentFrame_0(self , rsthis: & QTimeLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTimeLine12currentFrameEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtimeline.h:110
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal currentValue() const

/*
Returns the value corresponding to the current time.

See also valueForTime() and currentFrame().
*/
impl /*struct*/ QTimeLine {
  pub fn currentValue_0<RetType, T: QTimeLine_currentValue_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.currentValue_0(self);
    // return 1;
  }
}
pub trait QTimeLine_currentValue_0<RetType> {
  fn currentValue_0(self , rsthis: & QTimeLine) -> RetType;
}
impl<'a> /*trait*/ QTimeLine_currentValue_0<f64> for () {
  fn currentValue_0(self , rsthis: & QTimeLine) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTimeLine12currentValueEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtimeline.h:112
// index:0
// Public Visibility=Default Availability=Available
// [4] int frameForTime(int) const

/*
Returns the frame corresponding to the time msec. This value is calculated using a linear interpolation of the start and end frame, based on the value returned by valueForTime().

See also valueForTime() and setFrameRange().
*/
impl /*struct*/ QTimeLine {
  pub fn frameForTime_0<RetType, T: QTimeLine_frameForTime_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.frameForTime_0(self);
    // return 1;
  }
}
pub trait QTimeLine_frameForTime_0<RetType> {
  fn frameForTime_0(self , rsthis: & QTimeLine) -> RetType;
}
impl<'a> /*trait*/ QTimeLine_frameForTime_0<i32> for (i32) {
  fn frameForTime_0(self , rsthis: & QTimeLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTimeLine12frameForTimeEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtimeline.h:113
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] qreal valueForTime(int) const

/*
Returns the timeline value for the time msec. The returned value, which varies depending on the curve shape, is always between 0 and 1. If msec is 0, the default implementation always returns 0.

Reimplement this function to provide a custom curve shape for your timeline.

See also CurveShape and frameForTime().
*/
impl /*struct*/ QTimeLine {
  pub fn valueForTime_0<RetType, T: QTimeLine_valueForTime_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.valueForTime_0(self);
    // return 1;
  }
}
pub trait QTimeLine_valueForTime_0<RetType> {
  fn valueForTime_0(self , rsthis: & QTimeLine) -> RetType;
}
impl<'a> /*trait*/ QTimeLine_valueForTime_0<f64> for (i32) {
  fn valueForTime_0(self , rsthis: & QTimeLine) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTimeLine12valueForTimeEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtimeline.h:116
// index:0
// Public Visibility=Default Availability=Available
// [-2] void start()

/*
Starts the timeline. QTimeLine will enter Running state, and once it enters the event loop, it will update its current time, frame and value at regular intervals. The default interval is 40 ms (i.e., 25 times per second). You can change the update interval by calling setUpdateInterval().

The timeline will start from position 0, or the end if going backward. If you want to resume a stopped timeline without restarting, you can call resume() instead.

See also resume(), updateInterval(), frameChanged(), and valueChanged().
*/
impl /*struct*/ QTimeLine {
  pub fn start_0<RetType, T: QTimeLine_start_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.start_0(self);
    // return 1;
  }
}
pub trait QTimeLine_start_0<RetType> {
  fn start_0(self , rsthis: & QTimeLine) -> RetType;
}
impl<'a> /*trait*/ QTimeLine_start_0<(/*void*/)> for () {
  fn start_0(self , rsthis: & QTimeLine) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN9QTimeLine5startEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qtimeline.h:117
// index:0
// Public Visibility=Default Availability=Available
// [-2] void resume()

/*
Resumes the timeline from the current time. QTimeLine will reenter Running state, and once it enters the event loop, it will update its current time, frame and value at regular intervals.

In contrast to start(), this function does not restart the timeline before it resumes.

See also start(), updateInterval(), frameChanged(), and valueChanged().
*/
impl /*struct*/ QTimeLine {
  pub fn resume_0<RetType, T: QTimeLine_resume_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resume_0(self);
    // return 1;
  }
}
pub trait QTimeLine_resume_0<RetType> {
  fn resume_0(self , rsthis: & QTimeLine) -> RetType;
}
impl<'a> /*trait*/ QTimeLine_resume_0<(/*void*/)> for () {
  fn resume_0(self , rsthis: & QTimeLine) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN9QTimeLine6resumeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qtimeline.h:118
// index:0
// Public Visibility=Default Availability=Available
// [-2] void stop()

/*
Stops the timeline, causing QTimeLine to enter NotRunning state.

See also start().
*/
impl /*struct*/ QTimeLine {
  pub fn stop_0<RetType, T: QTimeLine_stop_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.stop_0(self);
    // return 1;
  }
}
pub trait QTimeLine_stop_0<RetType> {
  fn stop_0(self , rsthis: & QTimeLine) -> RetType;
}
impl<'a> /*trait*/ QTimeLine_stop_0<(/*void*/)> for () {
  fn stop_0(self , rsthis: & QTimeLine) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN9QTimeLine4stopEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qtimeline.h:119
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setPaused(bool)

/*
If paused is true, the timeline is paused, causing QTimeLine to enter Paused state. No updates will be signaled until either start() or setPaused(false) is called. If paused is false, the timeline is resumed and continues where it left.

See also state() and start().
*/
impl /*struct*/ QTimeLine {
  pub fn setPaused_0<RetType, T: QTimeLine_setPaused_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPaused_0(self);
    // return 1;
  }
}
pub trait QTimeLine_setPaused_0<RetType> {
  fn setPaused_0(self , rsthis: & QTimeLine) -> RetType;
}
impl<'a> /*trait*/ QTimeLine_setPaused_0<(/*void*/)> for (bool) {
  fn setPaused_0(self , rsthis: & QTimeLine) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN9QTimeLine9setPausedEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qtimeline.h:120
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCurrentTime(int)

/*

*/
impl /*struct*/ QTimeLine {
  pub fn setCurrentTime_0<RetType, T: QTimeLine_setCurrentTime_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCurrentTime_0(self);
    // return 1;
  }
}
pub trait QTimeLine_setCurrentTime_0<RetType> {
  fn setCurrentTime_0(self , rsthis: & QTimeLine) -> RetType;
}
impl<'a> /*trait*/ QTimeLine_setCurrentTime_0<(/*void*/)> for (i32) {
  fn setCurrentTime_0(self , rsthis: & QTimeLine) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QTimeLine14setCurrentTimeEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qtimeline.h:121
// index:0
// Public Visibility=Default Availability=Available
// [-2] void toggleDirection()

/*
Toggles the direction of the timeline. If the direction was Forward, it becomes Backward, and vice verca.

See also setDirection().
*/
impl /*struct*/ QTimeLine {
  pub fn toggleDirection_0<RetType, T: QTimeLine_toggleDirection_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toggleDirection_0(self);
    // return 1;
  }
}
pub trait QTimeLine_toggleDirection_0<RetType> {
  fn toggleDirection_0(self , rsthis: & QTimeLine) -> RetType;
}
impl<'a> /*trait*/ QTimeLine_toggleDirection_0<(/*void*/)> for () {
  fn toggleDirection_0(self , rsthis: & QTimeLine) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN9QTimeLine15toggleDirectionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qtimeline.h:130
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void timerEvent(QTimerEvent *)

/*
Reimplemented from QObject::timerEvent().
*/
impl /*struct*/ QTimeLine {
  pub fn timerEvent_0<RetType, T: QTimeLine_timerEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.timerEvent_0(self);
    // return 1;
  }
}
pub trait QTimeLine_timerEvent_0<RetType> {
  fn timerEvent_0(self , rsthis: & QTimeLine) -> RetType;
}
impl<'a> /*trait*/ QTimeLine_timerEvent_0<(/*void*/)> for (usize) {
  fn timerEvent_0(self , rsthis: & QTimeLine) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QTimeLine10timerEventEP11QTimerEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}


/*
This enum describes the state of the timeline.



See also state() and stateChanged().

*/
pub type QTimeLine__State = i32;
// The timeline is not running. This is the initial state of QTimeLine, and the state QTimeLine reenters when finished. The current time, frame and value remain unchanged until either setCurrentTime() is called, or the timeline is started by calling start().
pub const QTimeLine__NotRunning :QTimeLine__State = 0;
// The timeline is paused (i.e., temporarily suspended). Calling setPaused(false) will resume timeline activity.
pub const QTimeLine__Paused :QTimeLine__State = 1;
// The timeline is running. While control is in the event loop, QTimeLine will update its current time at regular intervals, emitting valueChanged() and frameChanged() when appropriate.
pub const QTimeLine__Running :QTimeLine__State = 2;
pub fn QTimeLine_StateItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QTimeLine", val);
}
pub fn QTimeLine_StateItemName_s(val: i32) ->String {
  //var nilthis *QTimeLine
  //return nilthis.StateItemName(val);
  return QTimeLine_StateItemName(val);
}


/*
This enum describes the direction of the timeline when in Running state.



See also setDirection().

*/
pub type QTimeLine__Direction = i32;
// 
pub const QTimeLine__Forward :QTimeLine__Direction = 0;
// 
pub const QTimeLine__Backward :QTimeLine__Direction = 1;
pub fn QTimeLine_DirectionItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QTimeLine", val);
}
pub fn QTimeLine_DirectionItemName_s(val: i32) ->String {
  //var nilthis *QTimeLine
  //return nilthis.DirectionItemName(val);
  return QTimeLine_DirectionItemName(val);
}


/*
This enum describes the default shape of QTimeLine's value curve. The default, shape is EaseInOutCurve. The curve defines the relation between the value and the timeline.



See also setCurveShape().

*/
pub type QTimeLine__CurveShape = i32;
// The value starts growing slowly, then increases in speed.
pub const QTimeLine__EaseInCurve :QTimeLine__CurveShape = 0;
// The value starts growing steadily, then ends slowly.
pub const QTimeLine__EaseOutCurve :QTimeLine__CurveShape = 1;
// The value starts growing slowly, then runs steadily, then grows slowly again.
pub const QTimeLine__EaseInOutCurve :QTimeLine__CurveShape = 2;
// 
pub const QTimeLine__LinearCurve :QTimeLine__CurveShape = 3;
// The value grows sinusoidally.
pub const QTimeLine__SineCurve :QTimeLine__CurveShape = 4;
// The value grows cosinusoidally.
pub const QTimeLine__CosineCurve :QTimeLine__CurveShape = 5;
pub fn QTimeLine_CurveShapeItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QTimeLine", val);
}
pub fn QTimeLine_CurveShapeItemName_s(val: i32) ->String {
  //var nilthis *QTimeLine
  //return nilthis.CurveShapeItemName(val);
  return QTimeLine_CurveShapeItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
