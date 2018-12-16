

// mod ::core::QParallelAnimationGroup
// package qtcore
// /usr/include/qt/QtCore/qparallelanimationgroup.h
// #include <qparallelanimationgroup.h>
// #include <QtCore>

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
use std::default::Default;
use std::ops::Deref;
use qtrt; // super::super::qtrt;
use super::*;
//  ext block end

//  body block begin

// bool event(QEvent *)
// func (this *QParallelAnimationGroup) InheritEvent(f func(event *QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "event", f)
// }

// void updateCurrentTime(int)
// func (this *QParallelAnimationGroup) InheritUpdateCurrentTime(f func(currentTime int) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "updateCurrentTime", f)
// }

// void updateState(QAbstractAnimation::State, QAbstractAnimation::State)
// func (this *QParallelAnimationGroup) InheritUpdateState(f func(newState int, oldState int) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "updateState", f)
// }

// void updateDirection(QAbstractAnimation::Direction)
// func (this *QParallelAnimationGroup) InheritUpdateDirection(f func(direction int) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "updateDirection", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QParallelAnimationGroup)=16
pub struct QParallelAnimationGroup {
  qbase: QAnimationGroup,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QParallelAnimationGroup_ITF interface {
//    QAnimationGroup_ITF
//    QParallelAnimationGroup_PTR() *QParallelAnimationGroup
//}
//func (ptr *QParallelAnimationGroup) QParallelAnimationGroup_PTR() *QParallelAnimationGroup { return ptr }

impl /*struct*/ QParallelAnimationGroup {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QParallelAnimationGroup {
    return QParallelAnimationGroup{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QParallelAnimationGroup {
//  type Target = QParallelAnimationGroupBASE;
//
//  fn deref(&self) -> &QParallelAnimationGroupBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QParallelAnimationGroupBASE> for QParallelAnimationGroup {
//  fn as_ref(& self) -> & QParallelAnimationGroupBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qparallelanimationgroup.h:53
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QParallelAnimationGroup {
  pub fn metaObject_0<RetType, T: QParallelAnimationGroup_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QParallelAnimationGroup_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QParallelAnimationGroup) -> RetType;
}
impl<'a> /*trait*/ QParallelAnimationGroup_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QParallelAnimationGroup) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK23QParallelAnimationGroup10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qparallelanimationgroup.h:56
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QParallelAnimationGroup(QObject *)

/*
Constructs a QParallelAnimationGroup. parent is passed to QObject's constructor.
*/
// QParallelAnimationGroup(QObject *) ctx.fn_proto_cpp
impl /*struct*/ QParallelAnimationGroup {
  pub fn QParallelAnimationGroup_0<T: QParallelAnimationGroup_QParallelAnimationGroup_0>(value: T) -> QParallelAnimationGroup {
    let rsthis = value.QParallelAnimationGroup_0();
    return rsthis;
    // return 1;
  }
}

pub trait QParallelAnimationGroup_QParallelAnimationGroup_0 {
  fn QParallelAnimationGroup_0(self) -> QParallelAnimationGroup;
}
// QParallelAnimationGroup(QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QParallelAnimationGroup_QParallelAnimationGroup_0 for (usize) {
  fn QParallelAnimationGroup_0(self) -> QParallelAnimationGroup {
    // unsafe{_ZN23QParallelAnimationGroupC2EP7QObject()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN23QParallelAnimationGroupC2EP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QParallelAnimationGroup{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qparallelanimationgroup.h:57
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QParallelAnimationGroup()

/*

*/
pub fn DeleteQParallelAnimationGroup(this :*mut QParallelAnimationGroup) {
    // let rv = qtrt::InvokeQtFunc6("_ZN23QParallelAnimationGroupD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qparallelanimationgroup.h:59
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] int duration() const

/*
Reimplemented from QAbstractAnimation::duration().
*/
impl /*struct*/ QParallelAnimationGroup {
  pub fn duration_0<RetType, T: QParallelAnimationGroup_duration_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.duration_0(self);
    // return 1;
  }
}
pub trait QParallelAnimationGroup_duration_0<RetType> {
  fn duration_0(self , rsthis: & QParallelAnimationGroup) -> RetType;
}
impl<'a> /*trait*/ QParallelAnimationGroup_duration_0<i32> for () {
  fn duration_0(self , rsthis: & QParallelAnimationGroup) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK23QParallelAnimationGroup8durationEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qparallelanimationgroup.h:63
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool event(QEvent *)

/*
Reimplemented from QObject::event().
*/
impl /*struct*/ QParallelAnimationGroup {
  pub fn event_0<RetType, T: QParallelAnimationGroup_event_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.event_0(self);
    // return 1;
  }
}
pub trait QParallelAnimationGroup_event_0<RetType> {
  fn event_0(self , rsthis: & QParallelAnimationGroup) -> RetType;
}
impl<'a> /*trait*/ QParallelAnimationGroup_event_0<bool> for (usize) {
  fn event_0(self , rsthis: & QParallelAnimationGroup) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN23QParallelAnimationGroup5eventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qparallelanimationgroup.h:65
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void updateCurrentTime(int)

/*
Reimplemented from QAbstractAnimation::updateCurrentTime().
*/
impl /*struct*/ QParallelAnimationGroup {
  pub fn updateCurrentTime_0<RetType, T: QParallelAnimationGroup_updateCurrentTime_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.updateCurrentTime_0(self);
    // return 1;
  }
}
pub trait QParallelAnimationGroup_updateCurrentTime_0<RetType> {
  fn updateCurrentTime_0(self , rsthis: & QParallelAnimationGroup) -> RetType;
}
impl<'a> /*trait*/ QParallelAnimationGroup_updateCurrentTime_0<(/*void*/)> for (i32) {
  fn updateCurrentTime_0(self , rsthis: & QParallelAnimationGroup) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN23QParallelAnimationGroup17updateCurrentTimeEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qparallelanimationgroup.h:66
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void updateState(QAbstractAnimation::State, QAbstractAnimation::State)

/*
Reimplemented from QAbstractAnimation::updateState().
*/
impl /*struct*/ QParallelAnimationGroup {
  pub fn updateState_0<RetType, T: QParallelAnimationGroup_updateState_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.updateState_0(self);
    // return 1;
  }
}
pub trait QParallelAnimationGroup_updateState_0<RetType> {
  fn updateState_0(self , rsthis: & QParallelAnimationGroup) -> RetType;
}
impl<'a> /*trait*/ QParallelAnimationGroup_updateState_0<(/*void*/)> for (i32,i32) {
  fn updateState_0(self , rsthis: & QParallelAnimationGroup) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN23QParallelAnimationGroup11updateStateEN18QAbstractAnimation5StateES1_", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qparallelanimationgroup.h:67
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void updateDirection(QAbstractAnimation::Direction)

/*
Reimplemented from QAbstractAnimation::updateDirection().
*/
impl /*struct*/ QParallelAnimationGroup {
  pub fn updateDirection_0<RetType, T: QParallelAnimationGroup_updateDirection_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.updateDirection_0(self);
    // return 1;
  }
}
pub trait QParallelAnimationGroup_updateDirection_0<RetType> {
  fn updateDirection_0(self , rsthis: & QParallelAnimationGroup) -> RetType;
}
impl<'a> /*trait*/ QParallelAnimationGroup_updateDirection_0<(/*void*/)> for (i32) {
  fn updateDirection_0(self , rsthis: & QParallelAnimationGroup) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN23QParallelAnimationGroup15updateDirectionEN18QAbstractAnimation9DirectionE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

//  body block end

//  keep block begin

//  keep block end
