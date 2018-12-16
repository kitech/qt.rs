

// mod ::core::QAnimationGroup
// package qtcore
// /usr/include/qt/QtCore/qanimationgroup.h
// #include <qanimationgroup.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 14
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
// func (this *QAnimationGroup) InheritEvent(f func(event *QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "event", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QAnimationGroup)=16
pub struct QAnimationGroup {
  qbase: QAbstractAnimation,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QAnimationGroup_ITF interface {
//    QAbstractAnimation_ITF
//    QAnimationGroup_PTR() *QAnimationGroup
//}
//func (ptr *QAnimationGroup) QAnimationGroup_PTR() *QAnimationGroup { return ptr }

impl /*struct*/ QAnimationGroup {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QAnimationGroup {
    return QAnimationGroup{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QAnimationGroup {
//  type Target = QAnimationGroupBASE;
//
//  fn deref(&self) -> &QAnimationGroupBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QAnimationGroupBASE> for QAnimationGroup {
//  fn as_ref(& self) -> & QAnimationGroupBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qanimationgroup.h:53
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QAnimationGroup {
  pub fn metaObject_0<RetType, T: QAnimationGroup_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QAnimationGroup_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QAnimationGroup) -> RetType;
}
impl<'a> /*trait*/ QAnimationGroup_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QAnimationGroup) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QAnimationGroup10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qanimationgroup.h:56
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QAnimationGroup(QObject *)

/*
Constructs a QAnimationGroup. parent is passed to QObject's constructor.
*/
// QAnimationGroup(QObject *) ctx.fn_proto_cpp
impl /*struct*/ QAnimationGroup {
  pub fn QAnimationGroup_0<T: QAnimationGroup_QAnimationGroup_0>(value: T) -> QAnimationGroup {
    let rsthis = value.QAnimationGroup_0();
    return rsthis;
    // return 1;
  }
}

pub trait QAnimationGroup_QAnimationGroup_0 {
  fn QAnimationGroup_0(self) -> QAnimationGroup;
}
// QAnimationGroup(QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QAnimationGroup_QAnimationGroup_0 for (usize) {
  fn QAnimationGroup_0(self) -> QAnimationGroup {
    // unsafe{_ZN15QAnimationGroupC2EP7QObject()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN15QAnimationGroupC2EP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QAnimationGroup{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qanimationgroup.h:57
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QAnimationGroup()

/*

*/
pub fn DeleteQAnimationGroup(this :*mut QAnimationGroup) {
    // let rv = qtrt::InvokeQtFunc6("_ZN15QAnimationGroupD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qanimationgroup.h:59
// index:0
// Public Visibility=Default Availability=Available
// [8] QAbstractAnimation * animationAt(int) const

/*
Returns a pointer to the animation at index in this group. This function is useful when you need access to a particular animation. index is between 0 and animationCount() - 1.

See also animationCount() and indexOfAnimation().
*/
impl /*struct*/ QAnimationGroup {
  pub fn animationAt_0<RetType, T: QAnimationGroup_animationAt_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.animationAt_0(self);
    // return 1;
  }
}
pub trait QAnimationGroup_animationAt_0<RetType> {
  fn animationAt_0(self , rsthis: & QAnimationGroup) -> RetType;
}
impl<'a> /*trait*/ QAnimationGroup_animationAt_0<usize> for (i32) {
  fn animationAt_0(self , rsthis: & QAnimationGroup) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QAnimationGroup11animationAtEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qanimationgroup.h:60
// index:0
// Public Visibility=Default Availability=Available
// [4] int animationCount() const

/*
Returns the number of animations managed by this group.

See also indexOfAnimation(), addAnimation(), and animationAt().
*/
impl /*struct*/ QAnimationGroup {
  pub fn animationCount_0<RetType, T: QAnimationGroup_animationCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.animationCount_0(self);
    // return 1;
  }
}
pub trait QAnimationGroup_animationCount_0<RetType> {
  fn animationCount_0(self , rsthis: & QAnimationGroup) -> RetType;
}
impl<'a> /*trait*/ QAnimationGroup_animationCount_0<i32> for () {
  fn animationCount_0(self , rsthis: & QAnimationGroup) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QAnimationGroup14animationCountEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qanimationgroup.h:61
// index:0
// Public Visibility=Default Availability=Available
// [4] int indexOfAnimation(QAbstractAnimation *) const

/*
Returns the index of animation. The returned index can be passed to the other functions that take an index as an argument.

See also insertAnimation(), animationAt(), and takeAnimation().
*/
impl /*struct*/ QAnimationGroup {
  pub fn indexOfAnimation_0<RetType, T: QAnimationGroup_indexOfAnimation_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.indexOfAnimation_0(self);
    // return 1;
  }
}
pub trait QAnimationGroup_indexOfAnimation_0<RetType> {
  fn indexOfAnimation_0(self , rsthis: & QAnimationGroup) -> RetType;
}
impl<'a> /*trait*/ QAnimationGroup_indexOfAnimation_0<i32> for (usize) {
  fn indexOfAnimation_0(self , rsthis: & QAnimationGroup) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QAnimationGroup16indexOfAnimationEP18QAbstractAnimation", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qanimationgroup.h:62
// index:0
// Public Visibility=Default Availability=Available
// [-2] void addAnimation(QAbstractAnimation *)

/*
Adds animation to this group. This will call insertAnimation with index equals to animationCount().

Note: The group takes ownership of the animation.

See also removeAnimation().
*/
impl /*struct*/ QAnimationGroup {
  pub fn addAnimation_0<RetType, T: QAnimationGroup_addAnimation_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addAnimation_0(self);
    // return 1;
  }
}
pub trait QAnimationGroup_addAnimation_0<RetType> {
  fn addAnimation_0(self , rsthis: & QAnimationGroup) -> RetType;
}
impl<'a> /*trait*/ QAnimationGroup_addAnimation_0<(/*void*/)> for (usize) {
  fn addAnimation_0(self , rsthis: & QAnimationGroup) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QAnimationGroup12addAnimationEP18QAbstractAnimation", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qanimationgroup.h:63
// index:0
// Public Visibility=Default Availability=Available
// [-2] void insertAnimation(int, QAbstractAnimation *)

/*
Inserts animation into this animation group at index. If index is 0 the animation is inserted at the beginning. If index is animationCount(), the animation is inserted at the end.

Note: The group takes ownership of the animation.

See also takeAnimation(), addAnimation(), indexOfAnimation(), and removeAnimation().
*/
impl /*struct*/ QAnimationGroup {
  pub fn insertAnimation_0<RetType, T: QAnimationGroup_insertAnimation_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.insertAnimation_0(self);
    // return 1;
  }
}
pub trait QAnimationGroup_insertAnimation_0<RetType> {
  fn insertAnimation_0(self , rsthis: & QAnimationGroup) -> RetType;
}
impl<'a> /*trait*/ QAnimationGroup_insertAnimation_0<(/*void*/)> for (i32,usize) {
  fn insertAnimation_0(self , rsthis: & QAnimationGroup) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QAnimationGroup15insertAnimationEiP18QAbstractAnimation", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qanimationgroup.h:64
// index:0
// Public Visibility=Default Availability=Available
// [-2] void removeAnimation(QAbstractAnimation *)

/*
Removes animation from this group. The ownership of animation is transferred to the caller.

See also takeAnimation(), insertAnimation(), and addAnimation().
*/
impl /*struct*/ QAnimationGroup {
  pub fn removeAnimation_0<RetType, T: QAnimationGroup_removeAnimation_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.removeAnimation_0(self);
    // return 1;
  }
}
pub trait QAnimationGroup_removeAnimation_0<RetType> {
  fn removeAnimation_0(self , rsthis: & QAnimationGroup) -> RetType;
}
impl<'a> /*trait*/ QAnimationGroup_removeAnimation_0<(/*void*/)> for (usize) {
  fn removeAnimation_0(self , rsthis: & QAnimationGroup) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QAnimationGroup15removeAnimationEP18QAbstractAnimation", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qanimationgroup.h:65
// index:0
// Public Visibility=Default Availability=Available
// [8] QAbstractAnimation * takeAnimation(int)

/*
Returns the animation at index and removes it from the animation group.

Note: The ownership of the animation is transferred to the caller.

See also removeAnimation(), addAnimation(), insertAnimation(), and indexOfAnimation().
*/
impl /*struct*/ QAnimationGroup {
  pub fn takeAnimation_0<RetType, T: QAnimationGroup_takeAnimation_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.takeAnimation_0(self);
    // return 1;
  }
}
pub trait QAnimationGroup_takeAnimation_0<RetType> {
  fn takeAnimation_0(self , rsthis: & QAnimationGroup) -> RetType;
}
impl<'a> /*trait*/ QAnimationGroup_takeAnimation_0<usize> for (i32) {
  fn takeAnimation_0(self , rsthis: & QAnimationGroup) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN15QAnimationGroup13takeAnimationEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qanimationgroup.h:66
// index:0
// Public Visibility=Default Availability=Available
// [-2] void clear()

/*
Removes and deletes all animations in this animation group, and resets the current time to 0.

See also addAnimation() and removeAnimation().
*/
impl /*struct*/ QAnimationGroup {
  pub fn clear_0<RetType, T: QAnimationGroup_clear_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.clear_0(self);
    // return 1;
  }
}
pub trait QAnimationGroup_clear_0<RetType> {
  fn clear_0(self , rsthis: & QAnimationGroup) -> RetType;
}
impl<'a> /*trait*/ QAnimationGroup_clear_0<(/*void*/)> for () {
  fn clear_0(self , rsthis: & QAnimationGroup) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN15QAnimationGroup5clearEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qanimationgroup.h:70
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool event(QEvent *)

/*
Reimplemented from QObject::event().
*/
impl /*struct*/ QAnimationGroup {
  pub fn event_0<RetType, T: QAnimationGroup_event_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.event_0(self);
    // return 1;
  }
}
pub trait QAnimationGroup_event_0<RetType> {
  fn event_0(self , rsthis: & QAnimationGroup) -> RetType;
}
impl<'a> /*trait*/ QAnimationGroup_event_0<bool> for (usize) {
  fn event_0(self , rsthis: & QAnimationGroup) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN15QAnimationGroup5eventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
