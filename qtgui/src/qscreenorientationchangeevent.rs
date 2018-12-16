

// mod ::gui::QScreenOrientationChangeEvent
// package qtgui
// /usr/include/qt/QtGui/qevent.h
// #include <qevent.h>
// #include <QtGui>

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
// import "github.com/kitech/qt.go/qtcore"
use qtcore::*; // super::super::%!s(MISSING)::*;
use std::default::Default;
use std::ops::Deref;
use qtrt; // super::super::qtrt;
use super::*;
//  ext block end

//  body block begin



/*

*/
#[derive(Default)] // class sizeof(QScreenOrientationChangeEvent)=40
pub struct QScreenOrientationChangeEvent {
  qbase: QEvent,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QScreenOrientationChangeEvent_ITF interface {
//    qtcore.QEvent_ITF
//    QScreenOrientationChangeEvent_PTR() *QScreenOrientationChangeEvent
//}
//func (ptr *QScreenOrientationChangeEvent) QScreenOrientationChangeEvent_PTR() *QScreenOrientationChangeEvent { return ptr }

impl /*struct*/ QScreenOrientationChangeEvent {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QScreenOrientationChangeEvent {
    return QScreenOrientationChangeEvent{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QScreenOrientationChangeEvent {
//  type Target = QScreenOrientationChangeEventBASE;
//
//  fn deref(&self) -> &QScreenOrientationChangeEventBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QScreenOrientationChangeEventBASE> for QScreenOrientationChangeEvent {
//  fn as_ref(& self) -> & QScreenOrientationChangeEventBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qevent.h:1038
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QScreenOrientationChangeEvent(QScreen *, Qt::ScreenOrientation)

/*

*/
// QScreenOrientationChangeEvent(QScreen *, Qt::ScreenOrientation) ctx.fn_proto_cpp
impl /*struct*/ QScreenOrientationChangeEvent {
  pub fn QScreenOrientationChangeEvent_0<T: QScreenOrientationChangeEvent_QScreenOrientationChangeEvent_0>(value: T) -> QScreenOrientationChangeEvent {
    let rsthis = value.QScreenOrientationChangeEvent_0();
    return rsthis;
    // return 1;
  }
}

pub trait QScreenOrientationChangeEvent_QScreenOrientationChangeEvent_0 {
  fn QScreenOrientationChangeEvent_0(self) -> QScreenOrientationChangeEvent;
}
// QScreenOrientationChangeEvent(QScreen *, Qt::ScreenOrientation) ctx.fn_proto_cpp
impl<'a> /*trait*/ QScreenOrientationChangeEvent_QScreenOrientationChangeEvent_0 for (usize,i32) {
  fn QScreenOrientationChangeEvent_0(self) -> QScreenOrientationChangeEvent {
    // unsafe{_ZN29QScreenOrientationChangeEventC2EP7QScreenN2Qt17ScreenOrientationE()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN29QScreenOrientationChangeEventC2EP7QScreenN2Qt17ScreenOrientationE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QScreenOrientationChangeEvent{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qevent.h:1039
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QScreenOrientationChangeEvent()

/*

*/
pub fn DeleteQScreenOrientationChangeEvent(this :*mut QScreenOrientationChangeEvent) {
    // let rv = qtrt::InvokeQtFunc6("_ZN29QScreenOrientationChangeEventD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 40)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qevent.h:1041
// index:0
// Public Visibility=Default Availability=Available
// [8] QScreen * screen() const

/*

*/
impl /*struct*/ QScreenOrientationChangeEvent {
  pub fn screen_0<RetType, T: QScreenOrientationChangeEvent_screen_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.screen_0(self);
    // return 1;
  }
}
pub trait QScreenOrientationChangeEvent_screen_0<RetType> {
  fn screen_0(self , rsthis: & QScreenOrientationChangeEvent) -> RetType;
}
impl<'a> /*trait*/ QScreenOrientationChangeEvent_screen_0<usize> for () {
  fn screen_0(self , rsthis: & QScreenOrientationChangeEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK29QScreenOrientationChangeEvent6screenEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:1042
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::ScreenOrientation orientation() const

/*

*/
impl /*struct*/ QScreenOrientationChangeEvent {
  pub fn orientation_0<RetType, T: QScreenOrientationChangeEvent_orientation_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.orientation_0(self);
    // return 1;
  }
}
pub trait QScreenOrientationChangeEvent_orientation_0<RetType> {
  fn orientation_0(self , rsthis: & QScreenOrientationChangeEvent) -> RetType;
}
impl<'a> /*trait*/ QScreenOrientationChangeEvent_orientation_0<i32> for () {
  fn orientation_0(self , rsthis: & QScreenOrientationChangeEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK29QScreenOrientationChangeEvent11orientationEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
