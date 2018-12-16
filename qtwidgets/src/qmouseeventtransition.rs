

// mod ::widgets::QMouseEventTransition
// package qtwidgets
// /usr/include/qt/QtWidgets/qmouseeventtransition.h
// #include <qmouseeventtransition.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 68
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
// func (this *QMouseEventTransition) InheritOnTransition(f func(event *qtcore.QEvent/*777 QEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "onTransition", f)
// }

// bool eventTest(QEvent *)
// func (this *QMouseEventTransition) InheritEventTest(f func(event *qtcore.QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "eventTest", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QMouseEventTransition)=16
pub struct QMouseEventTransition {
  qbase: QEventTransition,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QMouseEventTransition_ITF interface {
//    qtcore.QEventTransition_ITF
//    QMouseEventTransition_PTR() *QMouseEventTransition
//}
//func (ptr *QMouseEventTransition) QMouseEventTransition_PTR() *QMouseEventTransition { return ptr }

impl /*struct*/ QMouseEventTransition {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QMouseEventTransition {
    return QMouseEventTransition{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QMouseEventTransition {
//  type Target = QMouseEventTransitionBASE;
//
//  fn deref(&self) -> &QMouseEventTransitionBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QMouseEventTransitionBASE> for QMouseEventTransition {
//  fn as_ref(& self) -> & QMouseEventTransitionBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qmouseeventtransition.h:54
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QMouseEventTransition {
  pub fn metaObject_0<RetType, T: QMouseEventTransition_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QMouseEventTransition_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QMouseEventTransition) -> RetType;
}
impl<'a> /*trait*/ QMouseEventTransition_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QMouseEventTransition) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK21QMouseEventTransition10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmouseeventtransition.h:58
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QMouseEventTransition(QState *)

/*
Constructs a new mouse event transition with the given sourceState.
*/
// QMouseEventTransition(QState *) ctx.fn_proto_cpp
impl /*struct*/ QMouseEventTransition {
  pub fn QMouseEventTransition_0<T: QMouseEventTransition_QMouseEventTransition_0>(value: T) -> QMouseEventTransition {
    let rsthis = value.QMouseEventTransition_0();
    return rsthis;
    // return 1;
  }
}

pub trait QMouseEventTransition_QMouseEventTransition_0 {
  fn QMouseEventTransition_0(self) -> QMouseEventTransition;
}
// QMouseEventTransition(QState *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QMouseEventTransition_QMouseEventTransition_0 for (usize) {
  fn QMouseEventTransition_0(self) -> QMouseEventTransition {
    // unsafe{_ZN21QMouseEventTransitionC2EP6QState()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN21QMouseEventTransitionC2EP6QState", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QMouseEventTransition{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmouseeventtransition.h:59
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QMouseEventTransition(QObject *, QEvent::Type, Qt::MouseButton, QState *)

/*
Constructs a new mouse event transition with the given sourceState.
*/
// QMouseEventTransition(QObject *, QEvent::Type, Qt::MouseButton, QState *) ctx.fn_proto_cpp
impl /*struct*/ QMouseEventTransition {
  pub fn QMouseEventTransition_1<T: QMouseEventTransition_QMouseEventTransition_1>(value: T) -> QMouseEventTransition {
    let rsthis = value.QMouseEventTransition_1();
    return rsthis;
    // return 1;
  }
}

pub trait QMouseEventTransition_QMouseEventTransition_1 {
  fn QMouseEventTransition_1(self) -> QMouseEventTransition;
}
// QMouseEventTransition(QObject *, QEvent::Type, Qt::MouseButton, QState *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QMouseEventTransition_QMouseEventTransition_1 for (usize,i32,i32,usize) {
  fn QMouseEventTransition_1(self) -> QMouseEventTransition {
    // unsafe{_ZN21QMouseEventTransitionC2EP7QObjectN6QEvent4TypeEN2Qt11MouseButtonEP6QState()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN21QMouseEventTransitionC2EP7QObjectN6QEvent4TypeEN2Qt11MouseButtonEP6QState", 4,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QMouseEventTransition{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmouseeventtransition.h:61
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QMouseEventTransition()

/*

*/
pub fn DeleteQMouseEventTransition(this :*mut QMouseEventTransition) {
    // let rv = qtrt::InvokeQtFunc6("_ZN21QMouseEventTransitionD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qmouseeventtransition.h:63
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::MouseButton button() const

/*
Returns the button that this mouse event transition checks for.

Note: Getter function for property button. 

See also setButton().
*/
impl /*struct*/ QMouseEventTransition {
  pub fn button_0<RetType, T: QMouseEventTransition_button_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.button_0(self);
    // return 1;
  }
}
pub trait QMouseEventTransition_button_0<RetType> {
  fn button_0(self , rsthis: & QMouseEventTransition) -> RetType;
}
impl<'a> /*trait*/ QMouseEventTransition_button_0<i32> for () {
  fn button_0(self , rsthis: & QMouseEventTransition) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK21QMouseEventTransition6buttonEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmouseeventtransition.h:64
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setButton(Qt::MouseButton)

/*
Sets the button that this mouse event transition will check for.

Note: Setter function for property button. 

See also button().
*/
impl /*struct*/ QMouseEventTransition {
  pub fn setButton_0<RetType, T: QMouseEventTransition_setButton_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setButton_0(self);
    // return 1;
  }
}
pub trait QMouseEventTransition_setButton_0<RetType> {
  fn setButton_0(self , rsthis: & QMouseEventTransition) -> RetType;
}
impl<'a> /*trait*/ QMouseEventTransition_setButton_0<(/*void*/)> for (i32) {
  fn setButton_0(self , rsthis: & QMouseEventTransition) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN21QMouseEventTransition9setButtonEN2Qt11MouseButtonE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmouseeventtransition.h:66
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::KeyboardModifiers modifierMask() const

/*
Returns the keyboard modifier mask that this mouse event transition checks for.

Note: Getter function for property modifierMask. 

See also setModifierMask().
*/
impl /*struct*/ QMouseEventTransition {
  pub fn modifierMask_0<RetType, T: QMouseEventTransition_modifierMask_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.modifierMask_0(self);
    // return 1;
  }
}
pub trait QMouseEventTransition_modifierMask_0<RetType> {
  fn modifierMask_0(self , rsthis: & QMouseEventTransition) -> RetType;
}
impl<'a> /*trait*/ QMouseEventTransition_modifierMask_0<i32> for () {
  fn modifierMask_0(self , rsthis: & QMouseEventTransition) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK21QMouseEventTransition12modifierMaskEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmouseeventtransition.h:67
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setModifierMask(Qt::KeyboardModifiers)

/*
Sets the keyboard modifier mask that this mouse event transition will check for to modifierMask.

Note: Setter function for property modifierMask. 

See also modifierMask().
*/
impl /*struct*/ QMouseEventTransition {
  pub fn setModifierMask_0<RetType, T: QMouseEventTransition_setModifierMask_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setModifierMask_0(self);
    // return 1;
  }
}
pub trait QMouseEventTransition_setModifierMask_0<RetType> {
  fn setModifierMask_0(self , rsthis: & QMouseEventTransition) -> RetType;
}
impl<'a> /*trait*/ QMouseEventTransition_setModifierMask_0<(/*void*/)> for (i32) {
  fn setModifierMask_0(self , rsthis: & QMouseEventTransition) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN21QMouseEventTransition15setModifierMaskE6QFlagsIN2Qt16KeyboardModifierEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmouseeventtransition.h:69
// index:0
// Public Visibility=Default Availability=Available
// [8] QPainterPath hitTestPath() const

/*
Returns the hit test path for this mouse event transition.

See also setHitTestPath().
*/
impl /*struct*/ QMouseEventTransition {
  pub fn hitTestPath_0<RetType, T: QMouseEventTransition_hitTestPath_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hitTestPath_0(self);
    // return 1;
  }
}
pub trait QMouseEventTransition_hitTestPath_0<RetType> {
  fn hitTestPath_0(self , rsthis: & QMouseEventTransition) -> RetType;
}
impl<'a> /*trait*/ QMouseEventTransition_hitTestPath_0<usize> for () {
  fn hitTestPath_0(self , rsthis: & QMouseEventTransition) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK21QMouseEventTransition11hitTestPathEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmouseeventtransition.h:70
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setHitTestPath(const QPainterPath &)

/*
Sets the hit test path for this mouse event transition to path. If a valid path has been set, the transition will only trigger if the mouse event position (QMouseEvent::pos()) is inside the path.

See also hitTestPath() and QPainterPath::contains().
*/
impl /*struct*/ QMouseEventTransition {
  pub fn setHitTestPath_0<RetType, T: QMouseEventTransition_setHitTestPath_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setHitTestPath_0(self);
    // return 1;
  }
}
pub trait QMouseEventTransition_setHitTestPath_0<RetType> {
  fn setHitTestPath_0(self , rsthis: & QMouseEventTransition) -> RetType;
}
impl<'a> /*trait*/ QMouseEventTransition_setHitTestPath_0<(/*void*/)> for (usize) {
  fn setHitTestPath_0(self , rsthis: & QMouseEventTransition) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN21QMouseEventTransition14setHitTestPathERK12QPainterPath", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmouseeventtransition.h:73
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void onTransition(QEvent *)

/*
Reimplemented from QAbstractTransition::onTransition().
*/
impl /*struct*/ QMouseEventTransition {
  pub fn onTransition_0<RetType, T: QMouseEventTransition_onTransition_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.onTransition_0(self);
    // return 1;
  }
}
pub trait QMouseEventTransition_onTransition_0<RetType> {
  fn onTransition_0(self , rsthis: & QMouseEventTransition) -> RetType;
}
impl<'a> /*trait*/ QMouseEventTransition_onTransition_0<(/*void*/)> for (usize) {
  fn onTransition_0(self , rsthis: & QMouseEventTransition) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN21QMouseEventTransition12onTransitionEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmouseeventtransition.h:74
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool eventTest(QEvent *)

/*
Reimplemented from QAbstractTransition::eventTest().
*/
impl /*struct*/ QMouseEventTransition {
  pub fn eventTest_0<RetType, T: QMouseEventTransition_eventTest_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.eventTest_0(self);
    // return 1;
  }
}
pub trait QMouseEventTransition_eventTest_0<RetType> {
  fn eventTest_0(self , rsthis: & QMouseEventTransition) -> RetType;
}
impl<'a> /*trait*/ QMouseEventTransition_eventTest_0<bool> for (usize) {
  fn eventTest_0(self , rsthis: & QMouseEventTransition) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN21QMouseEventTransition9eventTestEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
