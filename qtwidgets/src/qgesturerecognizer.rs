

// mod ::widgets::QGestureRecognizer
// package qtwidgets
// /usr/include/qt/QtWidgets/qgesturerecognizer.h
// #include <qgesturerecognizer.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 13
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
#[derive(Default)] // class sizeof(QGestureRecognizer)=8
pub struct QGestureRecognizer {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QGestureRecognizer_ITF interface {
//    QGestureRecognizer_PTR() *QGestureRecognizer
//}
//func (ptr *QGestureRecognizer) QGestureRecognizer_PTR() *QGestureRecognizer { return ptr }

impl /*struct*/ QGestureRecognizer {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QGestureRecognizer {
    return QGestureRecognizer{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QGestureRecognizer {
//  type Target = QGestureRecognizerBASE;
//
//  fn deref(&self) -> &QGestureRecognizerBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QGestureRecognizerBASE> for QGestureRecognizer {
//  fn as_ref(& self) -> & QGestureRecognizerBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qgesturerecognizer.h:77
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QGestureRecognizer()

/*
Constructs a new gesture recognizer object.
*/
// QGestureRecognizer() ctx.fn_proto_cpp
impl /*struct*/ QGestureRecognizer {
  pub fn QGestureRecognizer_0<T: QGestureRecognizer_QGestureRecognizer_0>(value: T) -> QGestureRecognizer {
    let rsthis = value.QGestureRecognizer_0();
    return rsthis;
    // return 1;
  }
}

pub trait QGestureRecognizer_QGestureRecognizer_0 {
  fn QGestureRecognizer_0(self) -> QGestureRecognizer;
}
// QGestureRecognizer() ctx.fn_proto_cpp
impl<'a> /*trait*/ QGestureRecognizer_QGestureRecognizer_0 for () {
  fn QGestureRecognizer_0(self) -> QGestureRecognizer {
    // unsafe{_ZN18QGestureRecognizerC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN18QGestureRecognizerC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QGestureRecognizer{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgesturerecognizer.h:78
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QGestureRecognizer()

/*

*/
pub fn DeleteQGestureRecognizer(this :*mut QGestureRecognizer) {
    // let rv = qtrt::InvokeQtFunc6("_ZN18QGestureRecognizerD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 8)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qgesturerecognizer.h:80
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QGesture * create(QObject *)

/*
This function is called by Qt to create a new QGesture object for the given target (QWidget or QGraphicsObject).

Reimplement this function to create a custom QGesture-derived gesture object if necessary.

The application takes ownership of the created gesture object.
*/
impl /*struct*/ QGestureRecognizer {
  pub fn create_0<RetType, T: QGestureRecognizer_create_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.create_0(self);
    // return 1;
  }
}
pub trait QGestureRecognizer_create_0<RetType> {
  fn create_0(self , rsthis: & QGestureRecognizer) -> RetType;
}
impl<'a> /*trait*/ QGestureRecognizer_create_0<usize> for (usize) {
  fn create_0(self , rsthis: & QGestureRecognizer) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN18QGestureRecognizer6createEP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgesturerecognizer.h:81
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [4] QGestureRecognizer::Result recognize(QGesture *, QObject *, QEvent *)

/*
Handles the given event for the watched object, updating the state of the gesture object as required, and returns a suitable result for the current recognition step.

This function is called by the framework to allow the recognizer to filter input events dispatched to QWidget or QGraphicsObject instances that it is monitoring.

The result reflects how much of the gesture has been recognized. The state of the gesture object is set depending on the result.

See also QGestureRecognizer::Result.
*/
impl /*struct*/ QGestureRecognizer {
  pub fn recognize_0<RetType, T: QGestureRecognizer_recognize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.recognize_0(self);
    // return 1;
  }
}
pub trait QGestureRecognizer_recognize_0<RetType> {
  fn recognize_0(self , rsthis: & QGestureRecognizer) -> RetType;
}
impl<'a> /*trait*/ QGestureRecognizer_recognize_0<i32> for (usize,usize,usize) {
  fn recognize_0(self , rsthis: & QGestureRecognizer) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN18QGestureRecognizer9recognizeEP8QGestureP7QObjectP6QEvent", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgesturerecognizer.h:83
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void reset(QGesture *)

/*
This function is called by the framework to reset a given gesture.

Reimplement this function to implement additional requirements for custom QGesture objects. This may be necessary if you implement a custom QGesture whose properties need special handling when the gesture is reset.
*/
impl /*struct*/ QGestureRecognizer {
  pub fn reset_0<RetType, T: QGestureRecognizer_reset_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.reset_0(self);
    // return 1;
  }
}
pub trait QGestureRecognizer_reset_0<RetType> {
  fn reset_0(self , rsthis: & QGestureRecognizer) -> RetType;
}
impl<'a> /*trait*/ QGestureRecognizer_reset_0<(/*void*/)> for (usize) {
  fn reset_0(self , rsthis: & QGestureRecognizer) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN18QGestureRecognizer5resetEP8QGesture", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgesturerecognizer.h:85
// index:0
// Public static Visibility=Default Availability=Available
// [4] Qt::GestureType registerRecognizer(QGestureRecognizer *)

/*
Registers the given recognizer in the gesture framework and returns a gesture ID for it.

The application takes ownership of the recognizer and returns the gesture type ID associated with it. For gesture recognizers which handle custom QGesture objects (i.e., those which return Qt::CustomGesture in a QGesture::gestureType() function) the return value is a generated gesture ID with the Qt::CustomGesture flag set.

See also unregisterRecognizer(), QGestureRecognizer::create(), and QGesture.
*/
impl /*struct*/ QGestureRecognizer {
  pub fn registerRecognizer_0<RetType, T: QGestureRecognizer_registerRecognizer_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.registerRecognizer_0();
    // return 1;
  }
}
pub trait QGestureRecognizer_registerRecognizer_0<RetType> {
  fn registerRecognizer_0(self ) -> RetType;
}
impl<'a> /*trait*/ QGestureRecognizer_registerRecognizer_0<i32> for (usize) {
  fn registerRecognizer_0(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN18QGestureRecognizer18registerRecognizerEPS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgesturerecognizer.h:86
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void unregisterRecognizer(Qt::GestureType)

/*
Unregisters all gesture recognizers of the specified type.

See also registerRecognizer().
*/
impl /*struct*/ QGestureRecognizer {
  pub fn unregisterRecognizer_0<RetType, T: QGestureRecognizer_unregisterRecognizer_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.unregisterRecognizer_0();
    // return 1;
  }
}
pub trait QGestureRecognizer_unregisterRecognizer_0<RetType> {
  fn unregisterRecognizer_0(self ) -> RetType;
}
impl<'a> /*trait*/ QGestureRecognizer_unregisterRecognizer_0<(/*void*/)> for (i32) {
  fn unregisterRecognizer_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN18QGestureRecognizer20unregisterRecognizerEN2Qt11GestureTypeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}


/*


*/
pub type QGestureRecognizer__ResultFlag = i32;
// 
pub const QGestureRecognizer__Ignore :QGestureRecognizer__ResultFlag = 1;
// 
pub const QGestureRecognizer__MayBeGesture :QGestureRecognizer__ResultFlag = 2;
// 
pub const QGestureRecognizer__TriggerGesture :QGestureRecognizer__ResultFlag = 4;
// 
pub const QGestureRecognizer__FinishGesture :QGestureRecognizer__ResultFlag = 8;
// 
pub const QGestureRecognizer__CancelGesture :QGestureRecognizer__ResultFlag = 16;
// 
pub const QGestureRecognizer__ResultState_Mask :QGestureRecognizer__ResultFlag = 255;
// 
pub const QGestureRecognizer__ConsumeEventHint :QGestureRecognizer__ResultFlag = 256;
// 
pub const QGestureRecognizer__ResultHint_Mask :QGestureRecognizer__ResultFlag = 65280;
pub fn QGestureRecognizer_ResultFlagItemName(val: i32) ->String {
  match val {
     QGestureRecognizer__Ignore => // 1
     {return String::from("Ignore");}
     QGestureRecognizer__MayBeGesture => // 2
     {return String::from("MayBeGesture");}
     QGestureRecognizer__TriggerGesture => // 4
     {return String::from("TriggerGesture");}
     QGestureRecognizer__FinishGesture => // 8
     {return String::from("FinishGesture");}
     QGestureRecognizer__CancelGesture => // 16
     {return String::from("CancelGesture");}
     QGestureRecognizer__ResultState_Mask => // 255
     {return String::from("ResultState_Mask");}
     QGestureRecognizer__ConsumeEventHint => // 256
     {return String::from("ConsumeEventHint");}
     QGestureRecognizer__ResultHint_Mask => // 65280
     {return String::from("ResultHint_Mask");}
  _ => {return format!("{}", val);}
}
}
pub fn QGestureRecognizer_ResultFlagItemName_s(val: i32) ->String {
  //var nilthis *QGestureRecognizer
  //return nilthis.ResultFlagItemName(val);
  return QGestureRecognizer_ResultFlagItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
