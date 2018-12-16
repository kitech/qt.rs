

// mod ::widgets::QKeyEventTransition
// package qtwidgets
// /usr/include/qt/QtWidgets/qkeyeventtransition.h
// #include <qkeyeventtransition.h>
// #include <QtWidgets>

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

// void onTransition(QEvent *)
// func (this *QKeyEventTransition) InheritOnTransition(f func(event *qtcore.QEvent/*777 QEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "onTransition", f)
// }

// bool eventTest(QEvent *)
// func (this *QKeyEventTransition) InheritEventTest(f func(event *qtcore.QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "eventTest", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QKeyEventTransition)=16
pub struct QKeyEventTransition {
  qbase: QEventTransition,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QKeyEventTransition_ITF interface {
//    qtcore.QEventTransition_ITF
//    QKeyEventTransition_PTR() *QKeyEventTransition
//}
//func (ptr *QKeyEventTransition) QKeyEventTransition_PTR() *QKeyEventTransition { return ptr }

impl /*struct*/ QKeyEventTransition {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QKeyEventTransition {
    return QKeyEventTransition{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QKeyEventTransition {
//  type Target = QKeyEventTransitionBASE;
//
//  fn deref(&self) -> &QKeyEventTransitionBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QKeyEventTransitionBASE> for QKeyEventTransition {
//  fn as_ref(& self) -> & QKeyEventTransitionBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qkeyeventtransition.h:53
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QKeyEventTransition {
  pub fn metaObject_0<RetType, T: QKeyEventTransition_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QKeyEventTransition_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QKeyEventTransition) -> RetType;
}
impl<'a> /*trait*/ QKeyEventTransition_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QKeyEventTransition) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QKeyEventTransition10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qkeyeventtransition.h:57
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QKeyEventTransition(QState *)

/*
Constructs a new key event transition with the given sourceState.
*/
// QKeyEventTransition(QState *) ctx.fn_proto_cpp
impl /*struct*/ QKeyEventTransition {
  pub fn QKeyEventTransition_0<T: QKeyEventTransition_QKeyEventTransition_0>(value: T) -> QKeyEventTransition {
    let rsthis = value.QKeyEventTransition_0();
    return rsthis;
    // return 1;
  }
}

pub trait QKeyEventTransition_QKeyEventTransition_0 {
  fn QKeyEventTransition_0(self) -> QKeyEventTransition;
}
// QKeyEventTransition(QState *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QKeyEventTransition_QKeyEventTransition_0 for (usize) {
  fn QKeyEventTransition_0(self) -> QKeyEventTransition {
    // unsafe{_ZN19QKeyEventTransitionC2EP6QState()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN19QKeyEventTransitionC2EP6QState", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QKeyEventTransition{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qkeyeventtransition.h:58
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QKeyEventTransition(QObject *, QEvent::Type, int, QState *)

/*
Constructs a new key event transition with the given sourceState.
*/
// QKeyEventTransition(QObject *, QEvent::Type, int, QState *) ctx.fn_proto_cpp
impl /*struct*/ QKeyEventTransition {
  pub fn QKeyEventTransition_1<T: QKeyEventTransition_QKeyEventTransition_1>(value: T) -> QKeyEventTransition {
    let rsthis = value.QKeyEventTransition_1();
    return rsthis;
    // return 1;
  }
}

pub trait QKeyEventTransition_QKeyEventTransition_1 {
  fn QKeyEventTransition_1(self) -> QKeyEventTransition;
}
// QKeyEventTransition(QObject *, QEvent::Type, int, QState *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QKeyEventTransition_QKeyEventTransition_1 for (usize,i32,i32,usize) {
  fn QKeyEventTransition_1(self) -> QKeyEventTransition {
    // unsafe{_ZN19QKeyEventTransitionC2EP7QObjectN6QEvent4TypeEiP6QState()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN19QKeyEventTransitionC2EP7QObjectN6QEvent4TypeEiP6QState", 4,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QKeyEventTransition{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qkeyeventtransition.h:60
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QKeyEventTransition()

/*

*/
pub fn DeleteQKeyEventTransition(this :*mut QKeyEventTransition) {
    // let rv = qtrt::InvokeQtFunc6("_ZN19QKeyEventTransitionD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qkeyeventtransition.h:62
// index:0
// Public Visibility=Default Availability=Available
// [4] int key() const

/*
Returns the key that this key event transition checks for.

Note: Getter function for property key. 

See also setKey().
*/
impl /*struct*/ QKeyEventTransition {
  pub fn key_0<RetType, T: QKeyEventTransition_key_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.key_0(self);
    // return 1;
  }
}
pub trait QKeyEventTransition_key_0<RetType> {
  fn key_0(self , rsthis: & QKeyEventTransition) -> RetType;
}
impl<'a> /*trait*/ QKeyEventTransition_key_0<i32> for () {
  fn key_0(self , rsthis: & QKeyEventTransition) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QKeyEventTransition3keyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qkeyeventtransition.h:63
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setKey(int)

/*
Sets the key that this key event transition will check for.

Note: Setter function for property key. 

See also key().
*/
impl /*struct*/ QKeyEventTransition {
  pub fn setKey_0<RetType, T: QKeyEventTransition_setKey_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setKey_0(self);
    // return 1;
  }
}
pub trait QKeyEventTransition_setKey_0<RetType> {
  fn setKey_0(self , rsthis: & QKeyEventTransition) -> RetType;
}
impl<'a> /*trait*/ QKeyEventTransition_setKey_0<(/*void*/)> for (i32) {
  fn setKey_0(self , rsthis: & QKeyEventTransition) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN19QKeyEventTransition6setKeyEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qkeyeventtransition.h:65
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::KeyboardModifiers modifierMask() const

/*
Returns the keyboard modifier mask that this key event transition checks for.

Note: Getter function for property modifierMask. 

See also setModifierMask().
*/
impl /*struct*/ QKeyEventTransition {
  pub fn modifierMask_0<RetType, T: QKeyEventTransition_modifierMask_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.modifierMask_0(self);
    // return 1;
  }
}
pub trait QKeyEventTransition_modifierMask_0<RetType> {
  fn modifierMask_0(self , rsthis: & QKeyEventTransition) -> RetType;
}
impl<'a> /*trait*/ QKeyEventTransition_modifierMask_0<i32> for () {
  fn modifierMask_0(self , rsthis: & QKeyEventTransition) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QKeyEventTransition12modifierMaskEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qkeyeventtransition.h:66
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setModifierMask(Qt::KeyboardModifiers)

/*
Sets the keyboard modifier mask that this key event transition will check for to modifierMask.

Note: Setter function for property modifierMask. 

See also modifierMask().
*/
impl /*struct*/ QKeyEventTransition {
  pub fn setModifierMask_0<RetType, T: QKeyEventTransition_setModifierMask_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setModifierMask_0(self);
    // return 1;
  }
}
pub trait QKeyEventTransition_setModifierMask_0<RetType> {
  fn setModifierMask_0(self , rsthis: & QKeyEventTransition) -> RetType;
}
impl<'a> /*trait*/ QKeyEventTransition_setModifierMask_0<(/*void*/)> for (i32) {
  fn setModifierMask_0(self , rsthis: & QKeyEventTransition) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN19QKeyEventTransition15setModifierMaskE6QFlagsIN2Qt16KeyboardModifierEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qkeyeventtransition.h:69
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void onTransition(QEvent *)

/*
Reimplemented from QAbstractTransition::onTransition().
*/
impl /*struct*/ QKeyEventTransition {
  pub fn onTransition_0<RetType, T: QKeyEventTransition_onTransition_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.onTransition_0(self);
    // return 1;
  }
}
pub trait QKeyEventTransition_onTransition_0<RetType> {
  fn onTransition_0(self , rsthis: & QKeyEventTransition) -> RetType;
}
impl<'a> /*trait*/ QKeyEventTransition_onTransition_0<(/*void*/)> for (usize) {
  fn onTransition_0(self , rsthis: & QKeyEventTransition) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN19QKeyEventTransition12onTransitionEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qkeyeventtransition.h:70
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool eventTest(QEvent *)

/*
Reimplemented from QAbstractTransition::eventTest().
*/
impl /*struct*/ QKeyEventTransition {
  pub fn eventTest_0<RetType, T: QKeyEventTransition_eventTest_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.eventTest_0(self);
    // return 1;
  }
}
pub trait QKeyEventTransition_eventTest_0<RetType> {
  fn eventTest_0(self , rsthis: & QKeyEventTransition) -> RetType;
}
impl<'a> /*trait*/ QKeyEventTransition_eventTest_0<bool> for (usize) {
  fn eventTest_0(self , rsthis: & QKeyEventTransition) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN19QKeyEventTransition9eventTestEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
