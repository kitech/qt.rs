

// mod ::core::QAbstractAnimation
// package qtcore
// /usr/include/qt/QtCore/qabstractanimation.h
// #include <qabstractanimation.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 5
*/
// import "C"
// import "unsafe"
// import "reflect"
// import "fmt"
// import "log"
// import "github.com/kitech/qt.go/qtrt"
use std::default::Default;
use std::ops::Deref;
use qtrt; // super::super::qtrt;
use super::*;
//  ext block end

//  body block begin

// bool event(QEvent *)
// func (this *QAbstractAnimation) InheritEvent(f func(event *QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "event", f)
// }

// void updateCurrentTime(int)
// func (this *QAbstractAnimation) InheritUpdateCurrentTime(f func(currentTime int) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "updateCurrentTime", f)
// }

// void updateState(QAbstractAnimation::State, QAbstractAnimation::State)
// func (this *QAbstractAnimation) InheritUpdateState(f func(newState int, oldState int) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "updateState", f)
// }

// void updateDirection(QAbstractAnimation::Direction)
// func (this *QAbstractAnimation) InheritUpdateDirection(f func(direction int) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "updateDirection", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QAbstractAnimation)=16
pub struct QAbstractAnimation {
  qbase: QObject,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QAbstractAnimation_ITF interface {
//    QObject_ITF
//    QAbstractAnimation_PTR() *QAbstractAnimation
//}
//func (ptr *QAbstractAnimation) QAbstractAnimation_PTR() *QAbstractAnimation { return ptr }

impl /*struct*/ QAbstractAnimation {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QAbstractAnimation {
    return QAbstractAnimation{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QAbstractAnimation {
//  type Target = QAbstractAnimationBASE;
//
//  fn deref(&self) -> &QAbstractAnimationBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QAbstractAnimationBASE> for QAbstractAnimation {
//  fn as_ref(& self) -> & QAbstractAnimationBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qabstractanimation.h:57
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QAbstractAnimation {
  pub fn metaObject_0<RetType, T: QAbstractAnimation_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QAbstractAnimation_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QAbstractAnimation) -> RetType;
}
impl<'a> /*trait*/ QAbstractAnimation_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QAbstractAnimation) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QAbstractAnimation10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractanimation.h:85
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QAbstractAnimation(QObject *)

/*
Constructs the QAbstractAnimation base class, and passes parent to QObject's constructor.

See also QVariantAnimation and QAnimationGroup.
*/
// QAbstractAnimation(QObject *) ctx.fn_proto_cpp
impl /*struct*/ QAbstractAnimation {
  pub fn QAbstractAnimation_0<T: QAbstractAnimation_QAbstractAnimation_0>(value: T) -> QAbstractAnimation {
    let rsthis = value.QAbstractAnimation_0();
    return rsthis;
    // return 1;
  }
}

pub trait QAbstractAnimation_QAbstractAnimation_0 {
  fn QAbstractAnimation_0(self) -> QAbstractAnimation;
}
// QAbstractAnimation(QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QAbstractAnimation_QAbstractAnimation_0 for (usize) {
  fn QAbstractAnimation_0(self) -> QAbstractAnimation {
    // unsafe{_ZN18QAbstractAnimationC2EP7QObject()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN18QAbstractAnimationC2EP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QAbstractAnimation{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qabstractanimation.h:86
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QAbstractAnimation()

/*

*/
pub fn DeleteQAbstractAnimation(this :*mut QAbstractAnimation) {
    // let rv = qtrt::InvokeQtFunc6("_ZN18QAbstractAnimationD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qabstractanimation.h:88
// index:0
// Public Visibility=Default Availability=Available
// [4] QAbstractAnimation::State state() const

/*

*/
impl /*struct*/ QAbstractAnimation {
  pub fn state_0<RetType, T: QAbstractAnimation_state_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.state_0(self);
    // return 1;
  }
}
pub trait QAbstractAnimation_state_0<RetType> {
  fn state_0(self , rsthis: & QAbstractAnimation) -> RetType;
}
impl<'a> /*trait*/ QAbstractAnimation_state_0<i32> for () {
  fn state_0(self , rsthis: & QAbstractAnimation) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QAbstractAnimation5stateEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractanimation.h:90
// index:0
// Public Visibility=Default Availability=Available
// [8] QAnimationGroup * group() const

/*
If this animation is part of a QAnimationGroup, this function returns a pointer to the group; otherwise, it returns 0.

See also QAnimationGroup::addAnimation().
*/
impl /*struct*/ QAbstractAnimation {
  pub fn group_0<RetType, T: QAbstractAnimation_group_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.group_0(self);
    // return 1;
  }
}
pub trait QAbstractAnimation_group_0<RetType> {
  fn group_0(self , rsthis: & QAbstractAnimation) -> RetType;
}
impl<'a> /*trait*/ QAbstractAnimation_group_0<usize> for () {
  fn group_0(self , rsthis: & QAbstractAnimation) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QAbstractAnimation5groupEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractanimation.h:92
// index:0
// Public Visibility=Default Availability=Available
// [4] QAbstractAnimation::Direction direction() const

/*

*/
impl /*struct*/ QAbstractAnimation {
  pub fn direction_0<RetType, T: QAbstractAnimation_direction_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.direction_0(self);
    // return 1;
  }
}
pub trait QAbstractAnimation_direction_0<RetType> {
  fn direction_0(self , rsthis: & QAbstractAnimation) -> RetType;
}
impl<'a> /*trait*/ QAbstractAnimation_direction_0<i32> for () {
  fn direction_0(self , rsthis: & QAbstractAnimation) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QAbstractAnimation9directionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractanimation.h:93
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDirection(QAbstractAnimation::Direction)

/*

*/
impl /*struct*/ QAbstractAnimation {
  pub fn setDirection_0<RetType, T: QAbstractAnimation_setDirection_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDirection_0(self);
    // return 1;
  }
}
pub trait QAbstractAnimation_setDirection_0<RetType> {
  fn setDirection_0(self , rsthis: & QAbstractAnimation) -> RetType;
}
impl<'a> /*trait*/ QAbstractAnimation_setDirection_0<(/*void*/)> for (i32) {
  fn setDirection_0(self , rsthis: & QAbstractAnimation) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN18QAbstractAnimation12setDirectionENS_9DirectionE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qabstractanimation.h:95
// index:0
// Public Visibility=Default Availability=Available
// [4] int currentTime() const

/*

*/
impl /*struct*/ QAbstractAnimation {
  pub fn currentTime_0<RetType, T: QAbstractAnimation_currentTime_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.currentTime_0(self);
    // return 1;
  }
}
pub trait QAbstractAnimation_currentTime_0<RetType> {
  fn currentTime_0(self , rsthis: & QAbstractAnimation) -> RetType;
}
impl<'a> /*trait*/ QAbstractAnimation_currentTime_0<i32> for () {
  fn currentTime_0(self , rsthis: & QAbstractAnimation) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QAbstractAnimation11currentTimeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractanimation.h:96
// index:0
// Public Visibility=Default Availability=Available
// [4] int currentLoopTime() const

/*
Returns the current time inside the current loop. It can go from 0 to duration().

See also duration() and currentTime.
*/
impl /*struct*/ QAbstractAnimation {
  pub fn currentLoopTime_0<RetType, T: QAbstractAnimation_currentLoopTime_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.currentLoopTime_0(self);
    // return 1;
  }
}
pub trait QAbstractAnimation_currentLoopTime_0<RetType> {
  fn currentLoopTime_0(self , rsthis: & QAbstractAnimation) -> RetType;
}
impl<'a> /*trait*/ QAbstractAnimation_currentLoopTime_0<i32> for () {
  fn currentLoopTime_0(self , rsthis: & QAbstractAnimation) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QAbstractAnimation15currentLoopTimeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractanimation.h:98
// index:0
// Public Visibility=Default Availability=Available
// [4] int loopCount() const

/*

*/
impl /*struct*/ QAbstractAnimation {
  pub fn loopCount_0<RetType, T: QAbstractAnimation_loopCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.loopCount_0(self);
    // return 1;
  }
}
pub trait QAbstractAnimation_loopCount_0<RetType> {
  fn loopCount_0(self , rsthis: & QAbstractAnimation) -> RetType;
}
impl<'a> /*trait*/ QAbstractAnimation_loopCount_0<i32> for () {
  fn loopCount_0(self , rsthis: & QAbstractAnimation) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QAbstractAnimation9loopCountEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractanimation.h:99
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setLoopCount(int)

/*

*/
impl /*struct*/ QAbstractAnimation {
  pub fn setLoopCount_0<RetType, T: QAbstractAnimation_setLoopCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setLoopCount_0(self);
    // return 1;
  }
}
pub trait QAbstractAnimation_setLoopCount_0<RetType> {
  fn setLoopCount_0(self , rsthis: & QAbstractAnimation) -> RetType;
}
impl<'a> /*trait*/ QAbstractAnimation_setLoopCount_0<(/*void*/)> for (i32) {
  fn setLoopCount_0(self , rsthis: & QAbstractAnimation) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN18QAbstractAnimation12setLoopCountEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qabstractanimation.h:100
// index:0
// Public Visibility=Default Availability=Available
// [4] int currentLoop() const

/*

*/
impl /*struct*/ QAbstractAnimation {
  pub fn currentLoop_0<RetType, T: QAbstractAnimation_currentLoop_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.currentLoop_0(self);
    // return 1;
  }
}
pub trait QAbstractAnimation_currentLoop_0<RetType> {
  fn currentLoop_0(self , rsthis: & QAbstractAnimation) -> RetType;
}
impl<'a> /*trait*/ QAbstractAnimation_currentLoop_0<i32> for () {
  fn currentLoop_0(self , rsthis: & QAbstractAnimation) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QAbstractAnimation11currentLoopEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractanimation.h:102
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [4] int duration() const

/*
This pure virtual function returns the duration of the animation, and defines for how long QAbstractAnimation should update the current time. This duration is local, and does not include the loop count.

A return value of -1 indicates that the animation has no defined duration; the animation should run forever until stopped. This is useful for animations that are not time driven, or where you cannot easily predict its duration (e.g., event driven audio playback in a game).

If the animation is a parallel QAnimationGroup, the duration will be the longest duration of all its animations. If the animation is a sequential QAnimationGroup, the duration will be the sum of the duration of all its animations.

Note: Getter function for property duration. 

See also loopCount.
*/
impl /*struct*/ QAbstractAnimation {
  pub fn duration_0<RetType, T: QAbstractAnimation_duration_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.duration_0(self);
    // return 1;
  }
}
pub trait QAbstractAnimation_duration_0<RetType> {
  fn duration_0(self , rsthis: & QAbstractAnimation) -> RetType;
}
impl<'a> /*trait*/ QAbstractAnimation_duration_0<i32> for () {
  fn duration_0(self , rsthis: & QAbstractAnimation) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QAbstractAnimation8durationEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractanimation.h:103
// index:0
// Public Visibility=Default Availability=Available
// [4] int totalDuration() const

/*
Returns the total and effective duration of the animation, including the loop count.

See also duration() and currentTime.
*/
impl /*struct*/ QAbstractAnimation {
  pub fn totalDuration_0<RetType, T: QAbstractAnimation_totalDuration_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.totalDuration_0(self);
    // return 1;
  }
}
pub trait QAbstractAnimation_totalDuration_0<RetType> {
  fn totalDuration_0(self , rsthis: & QAbstractAnimation) -> RetType;
}
impl<'a> /*trait*/ QAbstractAnimation_totalDuration_0<i32> for () {
  fn totalDuration_0(self , rsthis: & QAbstractAnimation) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QAbstractAnimation13totalDurationEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractanimation.h:106
// index:0
// Public Visibility=Default Availability=Available
// [-2] void finished()

/*
QAbstractAnimation emits this signal after the animation has stopped and has reached the end.

This signal is emitted after stateChanged().

See also stateChanged().
*/
impl /*struct*/ QAbstractAnimation {
  pub fn finished_0<RetType, T: QAbstractAnimation_finished_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.finished_0(self);
    // return 1;
  }
}
pub trait QAbstractAnimation_finished_0<RetType> {
  fn finished_0(self , rsthis: & QAbstractAnimation) -> RetType;
}
impl<'a> /*trait*/ QAbstractAnimation_finished_0<(/*void*/)> for () {
  fn finished_0(self , rsthis: & QAbstractAnimation) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN18QAbstractAnimation8finishedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qabstractanimation.h:107
// index:0
// Public Visibility=Default Availability=Available
// [-2] void stateChanged(QAbstractAnimation::State, QAbstractAnimation::State)

/*
QAbstractAnimation emits this signal whenever the state of the animation has changed from oldState to newState. This signal is emitted after the virtual updateState() function is called.

Note: Notifier signal for property state. 

See also updateState().
*/
impl /*struct*/ QAbstractAnimation {
  pub fn stateChanged_0<RetType, T: QAbstractAnimation_stateChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.stateChanged_0(self);
    // return 1;
  }
}
pub trait QAbstractAnimation_stateChanged_0<RetType> {
  fn stateChanged_0(self , rsthis: & QAbstractAnimation) -> RetType;
}
impl<'a> /*trait*/ QAbstractAnimation_stateChanged_0<(/*void*/)> for (i32,i32) {
  fn stateChanged_0(self , rsthis: & QAbstractAnimation) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN18QAbstractAnimation12stateChangedENS_5StateES0_", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qabstractanimation.h:108
// index:0
// Public Visibility=Default Availability=Available
// [-2] void currentLoopChanged(int)

/*
QAbstractAnimation emits this signal whenever the current loop changes. currentLoop is the current loop.

Note: Notifier signal for property currentLoop. 

See also currentLoop() and loopCount().
*/
impl /*struct*/ QAbstractAnimation {
  pub fn currentLoopChanged_0<RetType, T: QAbstractAnimation_currentLoopChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.currentLoopChanged_0(self);
    // return 1;
  }
}
pub trait QAbstractAnimation_currentLoopChanged_0<RetType> {
  fn currentLoopChanged_0(self , rsthis: & QAbstractAnimation) -> RetType;
}
impl<'a> /*trait*/ QAbstractAnimation_currentLoopChanged_0<(/*void*/)> for (i32) {
  fn currentLoopChanged_0(self , rsthis: & QAbstractAnimation) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN18QAbstractAnimation18currentLoopChangedEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qabstractanimation.h:109
// index:0
// Public Visibility=Default Availability=Available
// [-2] void directionChanged(QAbstractAnimation::Direction)

/*
QAbstractAnimation emits this signal whenever the direction has been changed. newDirection is the new direction.

Note: Notifier signal for property direction. 

See also direction.
*/
impl /*struct*/ QAbstractAnimation {
  pub fn directionChanged_0<RetType, T: QAbstractAnimation_directionChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.directionChanged_0(self);
    // return 1;
  }
}
pub trait QAbstractAnimation_directionChanged_0<RetType> {
  fn directionChanged_0(self , rsthis: & QAbstractAnimation) -> RetType;
}
impl<'a> /*trait*/ QAbstractAnimation_directionChanged_0<(/*void*/)> for (i32) {
  fn directionChanged_0(self , rsthis: & QAbstractAnimation) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN18QAbstractAnimation16directionChangedENS_9DirectionE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qabstractanimation.h:112
// index:0
// Public Visibility=Default Availability=Available
// [-2] void start(QAbstractAnimation::DeletionPolicy)

/*
Starts the animation. The policy argument says whether or not the animation should be deleted when it's done. When the animation starts, the stateChanged() signal is emitted, and state() returns Running. When control reaches the event loop, the animation will run by itself, periodically calling updateCurrentTime() as the animation progresses.

If the animation is currently stopped or has already reached the end, calling start() will rewind the animation and start again from the beginning. When the animation reaches the end, the animation will either stop, or if the loop level is more than 1, it will rewind and continue from the beginning.

If the animation is already running, this function does nothing.

See also stop() and state().
*/
impl /*struct*/ QAbstractAnimation {
  pub fn start_0<RetType, T: QAbstractAnimation_start_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.start_0(self);
    // return 1;
  }
}
pub trait QAbstractAnimation_start_0<RetType> {
  fn start_0(self , rsthis: & QAbstractAnimation) -> RetType;
}
impl<'a> /*trait*/ QAbstractAnimation_start_0<(/*void*/)> for (i32) {
  fn start_0(self , rsthis: & QAbstractAnimation) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN18QAbstractAnimation5startENS_14DeletionPolicyE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qabstractanimation.h:113
// index:0
// Public Visibility=Default Availability=Available
// [-2] void pause()

/*
Pauses the animation. When the animation is paused, state() returns Paused. The value of currentTime will remain unchanged until resume() or start() is called. If you want to continue from the current time, call resume().

See also start(), state(), and resume().
*/
impl /*struct*/ QAbstractAnimation {
  pub fn pause_0<RetType, T: QAbstractAnimation_pause_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.pause_0(self);
    // return 1;
  }
}
pub trait QAbstractAnimation_pause_0<RetType> {
  fn pause_0(self , rsthis: & QAbstractAnimation) -> RetType;
}
impl<'a> /*trait*/ QAbstractAnimation_pause_0<(/*void*/)> for () {
  fn pause_0(self , rsthis: & QAbstractAnimation) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN18QAbstractAnimation5pauseEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qabstractanimation.h:114
// index:0
// Public Visibility=Default Availability=Available
// [-2] void resume()

/*
Resumes the animation after it was paused. When the animation is resumed, it emits the resumed() and stateChanged() signals. The currenttime is not changed.

See also start(), pause(), and state().
*/
impl /*struct*/ QAbstractAnimation {
  pub fn resume_0<RetType, T: QAbstractAnimation_resume_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resume_0(self);
    // return 1;
  }
}
pub trait QAbstractAnimation_resume_0<RetType> {
  fn resume_0(self , rsthis: & QAbstractAnimation) -> RetType;
}
impl<'a> /*trait*/ QAbstractAnimation_resume_0<(/*void*/)> for () {
  fn resume_0(self , rsthis: & QAbstractAnimation) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN18QAbstractAnimation6resumeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qabstractanimation.h:115
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setPaused(bool)

/*
If paused is true, the animation is paused. If paused is false, the animation is resumed.

See also state(), pause(), and resume().
*/
impl /*struct*/ QAbstractAnimation {
  pub fn setPaused_0<RetType, T: QAbstractAnimation_setPaused_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPaused_0(self);
    // return 1;
  }
}
pub trait QAbstractAnimation_setPaused_0<RetType> {
  fn setPaused_0(self , rsthis: & QAbstractAnimation) -> RetType;
}
impl<'a> /*trait*/ QAbstractAnimation_setPaused_0<(/*void*/)> for (bool) {
  fn setPaused_0(self , rsthis: & QAbstractAnimation) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN18QAbstractAnimation9setPausedEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qabstractanimation.h:116
// index:0
// Public Visibility=Default Availability=Available
// [-2] void stop()

/*
Stops the animation. When the animation is stopped, it emits the stateChanged() signal, and state() returns Stopped. The current time is not changed.

If the animation stops by itself after reaching the end (i.e., currentLoopTime() == duration() and currentLoop() > loopCount() - 1), the finished() signal is emitted.

See also start() and state().
*/
impl /*struct*/ QAbstractAnimation {
  pub fn stop_0<RetType, T: QAbstractAnimation_stop_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.stop_0(self);
    // return 1;
  }
}
pub trait QAbstractAnimation_stop_0<RetType> {
  fn stop_0(self , rsthis: & QAbstractAnimation) -> RetType;
}
impl<'a> /*trait*/ QAbstractAnimation_stop_0<(/*void*/)> for () {
  fn stop_0(self , rsthis: & QAbstractAnimation) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN18QAbstractAnimation4stopEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qabstractanimation.h:117
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCurrentTime(int)

/*

*/
impl /*struct*/ QAbstractAnimation {
  pub fn setCurrentTime_0<RetType, T: QAbstractAnimation_setCurrentTime_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCurrentTime_0(self);
    // return 1;
  }
}
pub trait QAbstractAnimation_setCurrentTime_0<RetType> {
  fn setCurrentTime_0(self , rsthis: & QAbstractAnimation) -> RetType;
}
impl<'a> /*trait*/ QAbstractAnimation_setCurrentTime_0<(/*void*/)> for (i32) {
  fn setCurrentTime_0(self , rsthis: & QAbstractAnimation) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN18QAbstractAnimation14setCurrentTimeEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qabstractanimation.h:121
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool event(QEvent *)

/*
Reimplemented from QObject::event().
*/
impl /*struct*/ QAbstractAnimation {
  pub fn event_0<RetType, T: QAbstractAnimation_event_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.event_0(self);
    // return 1;
  }
}
pub trait QAbstractAnimation_event_0<RetType> {
  fn event_0(self , rsthis: & QAbstractAnimation) -> RetType;
}
impl<'a> /*trait*/ QAbstractAnimation_event_0<bool> for (usize) {
  fn event_0(self , rsthis: & QAbstractAnimation) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN18QAbstractAnimation5eventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractanimation.h:123
// index:0
// Protected purevirtual virtual Visibility=Default Availability=Available
// [-2] void updateCurrentTime(int)

/*
This pure virtual function is called every time the animation's currentTime changes.

See also updateState().
*/
impl /*struct*/ QAbstractAnimation {
  pub fn updateCurrentTime_0<RetType, T: QAbstractAnimation_updateCurrentTime_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.updateCurrentTime_0(self);
    // return 1;
  }
}
pub trait QAbstractAnimation_updateCurrentTime_0<RetType> {
  fn updateCurrentTime_0(self , rsthis: & QAbstractAnimation) -> RetType;
}
impl<'a> /*trait*/ QAbstractAnimation_updateCurrentTime_0<(/*void*/)> for (i32) {
  fn updateCurrentTime_0(self , rsthis: & QAbstractAnimation) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN18QAbstractAnimation17updateCurrentTimeEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qabstractanimation.h:124
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void updateState(QAbstractAnimation::State, QAbstractAnimation::State)

/*
This virtual function is called by QAbstractAnimation when the state of the animation is changed from oldState to newState.

See also start(), stop(), pause(), and resume().
*/
impl /*struct*/ QAbstractAnimation {
  pub fn updateState_0<RetType, T: QAbstractAnimation_updateState_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.updateState_0(self);
    // return 1;
  }
}
pub trait QAbstractAnimation_updateState_0<RetType> {
  fn updateState_0(self , rsthis: & QAbstractAnimation) -> RetType;
}
impl<'a> /*trait*/ QAbstractAnimation_updateState_0<(/*void*/)> for (i32,i32) {
  fn updateState_0(self , rsthis: & QAbstractAnimation) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN18QAbstractAnimation11updateStateENS_5StateES0_", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qabstractanimation.h:125
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void updateDirection(QAbstractAnimation::Direction)

/*
This virtual function is called by QAbstractAnimation when the direction of the animation is changed. The direction argument is the new direction.

See also setDirection() and direction().
*/
impl /*struct*/ QAbstractAnimation {
  pub fn updateDirection_0<RetType, T: QAbstractAnimation_updateDirection_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.updateDirection_0(self);
    // return 1;
  }
}
pub trait QAbstractAnimation_updateDirection_0<RetType> {
  fn updateDirection_0(self , rsthis: & QAbstractAnimation) -> RetType;
}
impl<'a> /*trait*/ QAbstractAnimation_updateDirection_0<(/*void*/)> for (i32) {
  fn updateDirection_0(self , rsthis: & QAbstractAnimation) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN18QAbstractAnimation15updateDirectionENS_9DirectionE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}


/*
This enum describes the direction of the animation when in Running state.



See also direction.

*/
pub type QAbstractAnimation__Direction = i32;
// 
pub const QAbstractAnimation__Forward :QAbstractAnimation__Direction = 0;
// 
pub const QAbstractAnimation__Backward :QAbstractAnimation__Direction = 1;
pub fn QAbstractAnimation_DirectionItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QAbstractAnimation", val);
}
pub fn QAbstractAnimation_DirectionItemName_s(val: i32) ->String {
  //var nilthis *QAbstractAnimation
  //return nilthis.DirectionItemName(val);
  return QAbstractAnimation_DirectionItemName(val);
}


/*
This enum describes the state of the animation.



See also state() and stateChanged().

*/
pub type QAbstractAnimation__State = i32;
// The animation is not running. This is the initial state of QAbstractAnimation, and the state QAbstractAnimation reenters when finished. The current time remain unchanged until either setCurrentTime() is called, or the animation is started by calling start().
pub const QAbstractAnimation__Stopped :QAbstractAnimation__State = 0;
// The animation is paused (i.e., temporarily suspended). Calling resume() will resume animation activity.
pub const QAbstractAnimation__Paused :QAbstractAnimation__State = 1;
// The animation is running. While control is in the event loop, QAbstractAnimation will update its current time at regular intervals, calling updateCurrentTime() when appropriate.
pub const QAbstractAnimation__Running :QAbstractAnimation__State = 2;
pub fn QAbstractAnimation_StateItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QAbstractAnimation", val);
}
pub fn QAbstractAnimation_StateItemName_s(val: i32) ->String {
  //var nilthis *QAbstractAnimation
  //return nilthis.StateItemName(val);
  return QAbstractAnimation_StateItemName(val);
}


/*

*/
pub type QAbstractAnimation__DeletionPolicy = i32;
// The animation will not be deleted when stopped.
pub const QAbstractAnimation__KeepWhenStopped :QAbstractAnimation__DeletionPolicy = 0;
// The animation will be automatically deleted when stopped.
pub const QAbstractAnimation__DeleteWhenStopped :QAbstractAnimation__DeletionPolicy = 1;
pub fn QAbstractAnimation_DeletionPolicyItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QAbstractAnimation", val);
}
pub fn QAbstractAnimation_DeletionPolicyItemName_s(val: i32) ->String {
  //var nilthis *QAbstractAnimation
  //return nilthis.DeletionPolicyItemName(val);
  return QAbstractAnimation_DeletionPolicyItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
