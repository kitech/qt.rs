

// mod ::core::QSequentialAnimationGroup
// package qtcore
// /usr/include/qt/QtCore/qsequentialanimationgroup.h
// #include <qsequentialanimationgroup.h>
// #include <QtCore>

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
use std::default::Default;
use std::ops::Deref;
use qtrt; // super::super::qtrt;
use super::*;
//  ext block end

//  body block begin

// bool event(QEvent *)
// func (this *QSequentialAnimationGroup) InheritEvent(f func(event *QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "event", f)
// }

// void updateCurrentTime(int)
// func (this *QSequentialAnimationGroup) InheritUpdateCurrentTime(f func(arg0 int) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "updateCurrentTime", f)
// }

// void updateState(QAbstractAnimation::State, QAbstractAnimation::State)
// func (this *QSequentialAnimationGroup) InheritUpdateState(f func(newState int, oldState int) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "updateState", f)
// }

// void updateDirection(QAbstractAnimation::Direction)
// func (this *QSequentialAnimationGroup) InheritUpdateDirection(f func(direction int) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "updateDirection", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QSequentialAnimationGroup)=16
pub struct QSequentialAnimationGroup {
  qbase: QAnimationGroup,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QSequentialAnimationGroup_ITF interface {
//    QAnimationGroup_ITF
//    QSequentialAnimationGroup_PTR() *QSequentialAnimationGroup
//}
//func (ptr *QSequentialAnimationGroup) QSequentialAnimationGroup_PTR() *QSequentialAnimationGroup { return ptr }

impl /*struct*/ QSequentialAnimationGroup {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QSequentialAnimationGroup {
    return QSequentialAnimationGroup{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QSequentialAnimationGroup {
//  type Target = QSequentialAnimationGroupBASE;
//
//  fn deref(&self) -> &QSequentialAnimationGroupBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QSequentialAnimationGroupBASE> for QSequentialAnimationGroup {
//  fn as_ref(& self) -> & QSequentialAnimationGroupBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qsequentialanimationgroup.h:55
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QSequentialAnimationGroup {
  pub fn metaObject_0<RetType, T: QSequentialAnimationGroup_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QSequentialAnimationGroup_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QSequentialAnimationGroup) -> RetType;
}
impl<'a> /*trait*/ QSequentialAnimationGroup_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QSequentialAnimationGroup) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK25QSequentialAnimationGroup10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsequentialanimationgroup.h:59
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QSequentialAnimationGroup(QObject *)

/*
Constructs a QSequentialAnimationGroup. parent is passed to QObject's constructor.
*/
// QSequentialAnimationGroup(QObject *) ctx.fn_proto_cpp
impl /*struct*/ QSequentialAnimationGroup {
  pub fn QSequentialAnimationGroup_0<T: QSequentialAnimationGroup_QSequentialAnimationGroup_0>(value: T) -> QSequentialAnimationGroup {
    let rsthis = value.QSequentialAnimationGroup_0();
    return rsthis;
    // return 1;
  }
}

pub trait QSequentialAnimationGroup_QSequentialAnimationGroup_0 {
  fn QSequentialAnimationGroup_0(self) -> QSequentialAnimationGroup;
}
// QSequentialAnimationGroup(QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QSequentialAnimationGroup_QSequentialAnimationGroup_0 for (usize) {
  fn QSequentialAnimationGroup_0(self) -> QSequentialAnimationGroup {
    // unsafe{_ZN25QSequentialAnimationGroupC2EP7QObject()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN25QSequentialAnimationGroupC2EP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QSequentialAnimationGroup{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsequentialanimationgroup.h:60
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QSequentialAnimationGroup()

/*

*/
pub fn DeleteQSequentialAnimationGroup(this :*mut QSequentialAnimationGroup) {
    // let rv = qtrt::InvokeQtFunc6("_ZN25QSequentialAnimationGroupD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qsequentialanimationgroup.h:62
// index:0
// Public Visibility=Default Availability=Available
// [8] QPauseAnimation * addPause(int)

/*
Adds a pause of msecs to this animation group. The pause is considered as a special type of animation, thus animationCount will be increased by one.

See also insertPause() and QAnimationGroup::addAnimation().
*/
impl /*struct*/ QSequentialAnimationGroup {
  pub fn addPause_0<RetType, T: QSequentialAnimationGroup_addPause_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addPause_0(self);
    // return 1;
  }
}
pub trait QSequentialAnimationGroup_addPause_0<RetType> {
  fn addPause_0(self , rsthis: & QSequentialAnimationGroup) -> RetType;
}
impl<'a> /*trait*/ QSequentialAnimationGroup_addPause_0<usize> for (i32) {
  fn addPause_0(self , rsthis: & QSequentialAnimationGroup) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN25QSequentialAnimationGroup8addPauseEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsequentialanimationgroup.h:63
// index:0
// Public Visibility=Default Availability=Available
// [8] QPauseAnimation * insertPause(int, int)

/*
Inserts a pause of msecs milliseconds at index in this animation group.

See also addPause() and QAnimationGroup::insertAnimation().
*/
impl /*struct*/ QSequentialAnimationGroup {
  pub fn insertPause_0<RetType, T: QSequentialAnimationGroup_insertPause_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.insertPause_0(self);
    // return 1;
  }
}
pub trait QSequentialAnimationGroup_insertPause_0<RetType> {
  fn insertPause_0(self , rsthis: & QSequentialAnimationGroup) -> RetType;
}
impl<'a> /*trait*/ QSequentialAnimationGroup_insertPause_0<usize> for (i32,i32) {
  fn insertPause_0(self , rsthis: & QSequentialAnimationGroup) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN25QSequentialAnimationGroup11insertPauseEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsequentialanimationgroup.h:65
// index:0
// Public Visibility=Default Availability=Available
// [8] QAbstractAnimation * currentAnimation() const

/*

*/
impl /*struct*/ QSequentialAnimationGroup {
  pub fn currentAnimation_0<RetType, T: QSequentialAnimationGroup_currentAnimation_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.currentAnimation_0(self);
    // return 1;
  }
}
pub trait QSequentialAnimationGroup_currentAnimation_0<RetType> {
  fn currentAnimation_0(self , rsthis: & QSequentialAnimationGroup) -> RetType;
}
impl<'a> /*trait*/ QSequentialAnimationGroup_currentAnimation_0<usize> for () {
  fn currentAnimation_0(self , rsthis: & QSequentialAnimationGroup) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK25QSequentialAnimationGroup16currentAnimationEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsequentialanimationgroup.h:66
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] int duration() const

/*
Reimplemented from QAbstractAnimation::duration().
*/
impl /*struct*/ QSequentialAnimationGroup {
  pub fn duration_0<RetType, T: QSequentialAnimationGroup_duration_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.duration_0(self);
    // return 1;
  }
}
pub trait QSequentialAnimationGroup_duration_0<RetType> {
  fn duration_0(self , rsthis: & QSequentialAnimationGroup) -> RetType;
}
impl<'a> /*trait*/ QSequentialAnimationGroup_duration_0<i32> for () {
  fn duration_0(self , rsthis: & QSequentialAnimationGroup) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK25QSequentialAnimationGroup8durationEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsequentialanimationgroup.h:69
// index:0
// Public Visibility=Default Availability=Available
// [-2] void currentAnimationChanged(QAbstractAnimation *)

/*
QSequentialAnimationGroup emits this signal when currentAnimation has been changed. current is the current animation.

Note: Notifier signal for property currentAnimation. 

See also currentAnimation().
*/
impl /*struct*/ QSequentialAnimationGroup {
  pub fn currentAnimationChanged_0<RetType, T: QSequentialAnimationGroup_currentAnimationChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.currentAnimationChanged_0(self);
    // return 1;
  }
}
pub trait QSequentialAnimationGroup_currentAnimationChanged_0<RetType> {
  fn currentAnimationChanged_0(self , rsthis: & QSequentialAnimationGroup) -> RetType;
}
impl<'a> /*trait*/ QSequentialAnimationGroup_currentAnimationChanged_0<(/*void*/)> for (usize) {
  fn currentAnimationChanged_0(self , rsthis: & QSequentialAnimationGroup) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN25QSequentialAnimationGroup23currentAnimationChangedEP18QAbstractAnimation", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsequentialanimationgroup.h:73
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool event(QEvent *)

/*
Reimplemented from QObject::event().
*/
impl /*struct*/ QSequentialAnimationGroup {
  pub fn event_0<RetType, T: QSequentialAnimationGroup_event_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.event_0(self);
    // return 1;
  }
}
pub trait QSequentialAnimationGroup_event_0<RetType> {
  fn event_0(self , rsthis: & QSequentialAnimationGroup) -> RetType;
}
impl<'a> /*trait*/ QSequentialAnimationGroup_event_0<bool> for (usize) {
  fn event_0(self , rsthis: & QSequentialAnimationGroup) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN25QSequentialAnimationGroup5eventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsequentialanimationgroup.h:75
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void updateCurrentTime(int)

/*
Reimplemented from QAbstractAnimation::updateCurrentTime().
*/
impl /*struct*/ QSequentialAnimationGroup {
  pub fn updateCurrentTime_0<RetType, T: QSequentialAnimationGroup_updateCurrentTime_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.updateCurrentTime_0(self);
    // return 1;
  }
}
pub trait QSequentialAnimationGroup_updateCurrentTime_0<RetType> {
  fn updateCurrentTime_0(self , rsthis: & QSequentialAnimationGroup) -> RetType;
}
impl<'a> /*trait*/ QSequentialAnimationGroup_updateCurrentTime_0<(/*void*/)> for (i32) {
  fn updateCurrentTime_0(self , rsthis: & QSequentialAnimationGroup) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN25QSequentialAnimationGroup17updateCurrentTimeEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsequentialanimationgroup.h:76
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void updateState(QAbstractAnimation::State, QAbstractAnimation::State)

/*
Reimplemented from QAbstractAnimation::updateState().
*/
impl /*struct*/ QSequentialAnimationGroup {
  pub fn updateState_0<RetType, T: QSequentialAnimationGroup_updateState_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.updateState_0(self);
    // return 1;
  }
}
pub trait QSequentialAnimationGroup_updateState_0<RetType> {
  fn updateState_0(self , rsthis: & QSequentialAnimationGroup) -> RetType;
}
impl<'a> /*trait*/ QSequentialAnimationGroup_updateState_0<(/*void*/)> for (i32,i32) {
  fn updateState_0(self , rsthis: & QSequentialAnimationGroup) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN25QSequentialAnimationGroup11updateStateEN18QAbstractAnimation5StateES1_", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsequentialanimationgroup.h:77
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void updateDirection(QAbstractAnimation::Direction)

/*
Reimplemented from QAbstractAnimation::updateDirection().
*/
impl /*struct*/ QSequentialAnimationGroup {
  pub fn updateDirection_0<RetType, T: QSequentialAnimationGroup_updateDirection_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.updateDirection_0(self);
    // return 1;
  }
}
pub trait QSequentialAnimationGroup_updateDirection_0<RetType> {
  fn updateDirection_0(self , rsthis: & QSequentialAnimationGroup) -> RetType;
}
impl<'a> /*trait*/ QSequentialAnimationGroup_updateDirection_0<(/*void*/)> for (i32) {
  fn updateDirection_0(self , rsthis: & QSequentialAnimationGroup) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN25QSequentialAnimationGroup15updateDirectionEN18QAbstractAnimation9DirectionE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

//  body block end

//  keep block begin

//  keep block end
