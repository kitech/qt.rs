

// mod ::widgets::QTapGesture
// package qtwidgets
// /usr/include/qt/QtWidgets/qgesture.h
// #include <qgesture.h>
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



/*

*/
#[derive(Default)] // class sizeof(QTapGesture)=16
pub struct QTapGesture {
  qbase: QGesture,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QTapGesture_ITF interface {
//    QGesture_ITF
//    QTapGesture_PTR() *QTapGesture
//}
//func (ptr *QTapGesture) QTapGesture_PTR() *QTapGesture { return ptr }

impl /*struct*/ QTapGesture {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QTapGesture {
    return QTapGesture{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QTapGesture {
//  type Target = QTapGestureBASE;
//
//  fn deref(&self) -> &QTapGestureBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QTapGestureBASE> for QTapGesture {
//  fn as_ref(& self) -> & QTapGestureBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qgesture.h:236
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QTapGesture {
  pub fn metaObject_0<RetType, T: QTapGesture_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QTapGesture_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QTapGesture) -> RetType;
}
impl<'a> /*trait*/ QTapGesture_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QTapGesture) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTapGesture10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgesture.h:242
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QTapGesture(QObject *)

/*

*/
// QTapGesture(QObject *) ctx.fn_proto_cpp
impl /*struct*/ QTapGesture {
  pub fn QTapGesture_0<T: QTapGesture_QTapGesture_0>(value: T) -> QTapGesture {
    let rsthis = value.QTapGesture_0();
    return rsthis;
    // return 1;
  }
}

pub trait QTapGesture_QTapGesture_0 {
  fn QTapGesture_0(self) -> QTapGesture;
}
// QTapGesture(QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QTapGesture_QTapGesture_0 for (usize) {
  fn QTapGesture_0(self) -> QTapGesture {
    // unsafe{_ZN11QTapGestureC2EP7QObject()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QTapGestureC2EP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTapGesture{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qgesture.h:243
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QTapGesture()

/*

*/
pub fn DeleteQTapGesture(this :*mut QTapGesture) {
    // let rv = qtrt::InvokeQtFunc6("_ZN11QTapGestureD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qgesture.h:245
// index:0
// Public Visibility=Default Availability=Available
// [16] QPointF position() const

/*

*/
impl /*struct*/ QTapGesture {
  pub fn position_0<RetType, T: QTapGesture_position_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.position_0(self);
    // return 1;
  }
}
pub trait QTapGesture_position_0<RetType> {
  fn position_0(self , rsthis: & QTapGesture) -> RetType;
}
impl<'a> /*trait*/ QTapGesture_position_0<usize> for () {
  fn position_0(self , rsthis: & QTapGesture) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTapGesture8positionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qgesture.h:246
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setPosition(const QPointF &)

/*

*/
impl /*struct*/ QTapGesture {
  pub fn setPosition_0<RetType, T: QTapGesture_setPosition_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPosition_0(self);
    // return 1;
  }
}
pub trait QTapGesture_setPosition_0<RetType> {
  fn setPosition_0(self , rsthis: & QTapGesture) -> RetType;
}
impl<'a> /*trait*/ QTapGesture_setPosition_0<(/*void*/)> for (usize) {
  fn setPosition_0(self , rsthis: & QTapGesture) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QTapGesture11setPositionERK7QPointF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

//  body block end

//  keep block begin

//  keep block end
