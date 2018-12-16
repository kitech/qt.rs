

// mod ::core::QPropertyAnimation
// package qtcore
// /usr/include/qt/QtCore/qpropertyanimation.h
// #include <qpropertyanimation.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 20
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
// func (this *QPropertyAnimation) InheritEvent(f func(event *QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "event", f)
// }

// void updateCurrentValue(const QVariant &)
// func (this *QPropertyAnimation) InheritUpdateCurrentValue(f func(value *QVariant) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "updateCurrentValue", f)
// }

// void updateState(QAbstractAnimation::State, QAbstractAnimation::State)
// func (this *QPropertyAnimation) InheritUpdateState(f func(newState int, oldState int) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "updateState", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QPropertyAnimation)=16
pub struct QPropertyAnimation {
  qbase: QVariantAnimation,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QPropertyAnimation_ITF interface {
//    QVariantAnimation_ITF
//    QPropertyAnimation_PTR() *QPropertyAnimation
//}
//func (ptr *QPropertyAnimation) QPropertyAnimation_PTR() *QPropertyAnimation { return ptr }

impl /*struct*/ QPropertyAnimation {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QPropertyAnimation {
    return QPropertyAnimation{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QPropertyAnimation {
//  type Target = QPropertyAnimationBASE;
//
//  fn deref(&self) -> &QPropertyAnimationBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QPropertyAnimationBASE> for QPropertyAnimation {
//  fn as_ref(& self) -> & QPropertyAnimationBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qpropertyanimation.h:53
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QPropertyAnimation {
  pub fn metaObject_0<RetType, T: QPropertyAnimation_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QPropertyAnimation_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QPropertyAnimation) -> RetType;
}
impl<'a> /*trait*/ QPropertyAnimation_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QPropertyAnimation) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QPropertyAnimation10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qpropertyanimation.h:58
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QPropertyAnimation(QObject *)

/*
Construct a QPropertyAnimation object. parent is passed to QObject's constructor.
*/
// QPropertyAnimation(QObject *) ctx.fn_proto_cpp
impl /*struct*/ QPropertyAnimation {
  pub fn QPropertyAnimation_0<T: QPropertyAnimation_QPropertyAnimation_0>(value: T) -> QPropertyAnimation {
    let rsthis = value.QPropertyAnimation_0();
    return rsthis;
    // return 1;
  }
}

pub trait QPropertyAnimation_QPropertyAnimation_0 {
  fn QPropertyAnimation_0(self) -> QPropertyAnimation;
}
// QPropertyAnimation(QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QPropertyAnimation_QPropertyAnimation_0 for (usize) {
  fn QPropertyAnimation_0(self) -> QPropertyAnimation {
    // unsafe{_ZN18QPropertyAnimationC2EP7QObject()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN18QPropertyAnimationC2EP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QPropertyAnimation{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qpropertyanimation.h:59
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QPropertyAnimation(QObject *, const QByteArray &, QObject *)

/*
Construct a QPropertyAnimation object. parent is passed to QObject's constructor.
*/
// QPropertyAnimation(QObject *, const QByteArray &, QObject *) ctx.fn_proto_cpp
impl /*struct*/ QPropertyAnimation {
  pub fn QPropertyAnimation_1<T: QPropertyAnimation_QPropertyAnimation_1>(value: T) -> QPropertyAnimation {
    let rsthis = value.QPropertyAnimation_1();
    return rsthis;
    // return 1;
  }
}

pub trait QPropertyAnimation_QPropertyAnimation_1 {
  fn QPropertyAnimation_1(self) -> QPropertyAnimation;
}
// QPropertyAnimation(QObject *, const QByteArray &, QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QPropertyAnimation_QPropertyAnimation_1 for (usize,usize,usize) {
  fn QPropertyAnimation_1(self) -> QPropertyAnimation {
    // unsafe{_ZN18QPropertyAnimationC2EP7QObjectRK10QByteArrayS1_()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN18QPropertyAnimationC2EP7QObjectRK10QByteArrayS1_", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QPropertyAnimation{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qpropertyanimation.h:60
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QPropertyAnimation()

/*

*/
pub fn DeleteQPropertyAnimation(this :*mut QPropertyAnimation) {
    // let rv = qtrt::InvokeQtFunc6("_ZN18QPropertyAnimationD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qpropertyanimation.h:62
// index:0
// Public Visibility=Default Availability=Available
// [8] QObject * targetObject() const

/*

*/
impl /*struct*/ QPropertyAnimation {
  pub fn targetObject_0<RetType, T: QPropertyAnimation_targetObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.targetObject_0(self);
    // return 1;
  }
}
pub trait QPropertyAnimation_targetObject_0<RetType> {
  fn targetObject_0(self , rsthis: & QPropertyAnimation) -> RetType;
}
impl<'a> /*trait*/ QPropertyAnimation_targetObject_0<usize> for () {
  fn targetObject_0(self , rsthis: & QPropertyAnimation) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QPropertyAnimation12targetObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qpropertyanimation.h:63
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTargetObject(QObject *)

/*

*/
impl /*struct*/ QPropertyAnimation {
  pub fn setTargetObject_0<RetType, T: QPropertyAnimation_setTargetObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTargetObject_0(self);
    // return 1;
  }
}
pub trait QPropertyAnimation_setTargetObject_0<RetType> {
  fn setTargetObject_0(self , rsthis: & QPropertyAnimation) -> RetType;
}
impl<'a> /*trait*/ QPropertyAnimation_setTargetObject_0<(/*void*/)> for (usize) {
  fn setTargetObject_0(self , rsthis: & QPropertyAnimation) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN18QPropertyAnimation15setTargetObjectEP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qpropertyanimation.h:65
// index:0
// Public Visibility=Default Availability=Available
// [8] QByteArray propertyName() const

/*

*/
impl /*struct*/ QPropertyAnimation {
  pub fn propertyName_0<RetType, T: QPropertyAnimation_propertyName_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.propertyName_0(self);
    // return 1;
  }
}
pub trait QPropertyAnimation_propertyName_0<RetType> {
  fn propertyName_0(self , rsthis: & QPropertyAnimation) -> RetType;
}
impl<'a> /*trait*/ QPropertyAnimation_propertyName_0<usize> for () {
  fn propertyName_0(self , rsthis: & QPropertyAnimation) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QPropertyAnimation12propertyNameEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qpropertyanimation.h:66
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setPropertyName(const QByteArray &)

/*

*/
impl /*struct*/ QPropertyAnimation {
  pub fn setPropertyName_0<RetType, T: QPropertyAnimation_setPropertyName_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPropertyName_0(self);
    // return 1;
  }
}
pub trait QPropertyAnimation_setPropertyName_0<RetType> {
  fn setPropertyName_0(self , rsthis: & QPropertyAnimation) -> RetType;
}
impl<'a> /*trait*/ QPropertyAnimation_setPropertyName_0<(/*void*/)> for (usize) {
  fn setPropertyName_0(self , rsthis: & QPropertyAnimation) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN18QPropertyAnimation15setPropertyNameERK10QByteArray", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qpropertyanimation.h:69
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool event(QEvent *)

/*
Reimplemented from QObject::event().
*/
impl /*struct*/ QPropertyAnimation {
  pub fn event_0<RetType, T: QPropertyAnimation_event_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.event_0(self);
    // return 1;
  }
}
pub trait QPropertyAnimation_event_0<RetType> {
  fn event_0(self , rsthis: & QPropertyAnimation) -> RetType;
}
impl<'a> /*trait*/ QPropertyAnimation_event_0<bool> for (usize) {
  fn event_0(self , rsthis: & QPropertyAnimation) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN18QPropertyAnimation5eventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qpropertyanimation.h:70
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void updateCurrentValue(const QVariant &)

/*
Reimplemented from QVariantAnimation::updateCurrentValue().

This virtual function is called by QVariantAnimation whenever the current value changes. value is the new, updated value. It updates the current value of the property on the target object.

See also currentValue and currentTime.
*/
impl /*struct*/ QPropertyAnimation {
  pub fn updateCurrentValue_0<RetType, T: QPropertyAnimation_updateCurrentValue_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.updateCurrentValue_0(self);
    // return 1;
  }
}
pub trait QPropertyAnimation_updateCurrentValue_0<RetType> {
  fn updateCurrentValue_0(self , rsthis: & QPropertyAnimation) -> RetType;
}
impl<'a> /*trait*/ QPropertyAnimation_updateCurrentValue_0<(/*void*/)> for (usize) {
  fn updateCurrentValue_0(self , rsthis: & QPropertyAnimation) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN18QPropertyAnimation18updateCurrentValueERK8QVariant", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qpropertyanimation.h:71
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void updateState(QAbstractAnimation::State, QAbstractAnimation::State)

/*
Reimplemented from QAbstractAnimation::updateState().

If the startValue is not defined when the state of the animation changes from Stopped to Running, the current property value is used as the initial value for the animation.
*/
impl /*struct*/ QPropertyAnimation {
  pub fn updateState_0<RetType, T: QPropertyAnimation_updateState_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.updateState_0(self);
    // return 1;
  }
}
pub trait QPropertyAnimation_updateState_0<RetType> {
  fn updateState_0(self , rsthis: & QPropertyAnimation) -> RetType;
}
impl<'a> /*trait*/ QPropertyAnimation_updateState_0<(/*void*/)> for (i32,i32) {
  fn updateState_0(self , rsthis: & QPropertyAnimation) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN18QPropertyAnimation11updateStateEN18QAbstractAnimation5StateES1_", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

//  body block end

//  keep block begin

//  keep block end
