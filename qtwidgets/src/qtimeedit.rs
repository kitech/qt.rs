

// mod ::widgets::QTimeEdit
// package qtwidgets
// /usr/include/qt/QtWidgets/qdatetimeedit.h
// #include <qdatetimeedit.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 70
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
#[derive(Default)] // class sizeof(QTimeEdit)=48
pub struct QTimeEdit {
  qbase: QDateTimeEdit,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QTimeEdit_ITF interface {
//    QDateTimeEdit_ITF
//    QTimeEdit_PTR() *QTimeEdit
//}
//func (ptr *QTimeEdit) QTimeEdit_PTR() *QTimeEdit { return ptr }

impl /*struct*/ QTimeEdit {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QTimeEdit {
    return QTimeEdit{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QTimeEdit {
//  type Target = QTimeEditBASE;
//
//  fn deref(&self) -> &QTimeEditBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QTimeEditBASE> for QTimeEdit {
//  fn as_ref(& self) -> & QTimeEditBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qdatetimeedit.h:204
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QTimeEdit {
  pub fn metaObject_0<RetType, T: QTimeEdit_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QTimeEdit_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QTimeEdit) -> RetType;
}
impl<'a> /*trait*/ QTimeEdit_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QTimeEdit) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QTimeEdit10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdatetimeedit.h:207
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QTimeEdit(QWidget *)

/*

*/
// QTimeEdit(QWidget *) ctx.fn_proto_cpp
impl /*struct*/ QTimeEdit {
  pub fn QTimeEdit_0<T: QTimeEdit_QTimeEdit_0>(value: T) -> QTimeEdit {
    let rsthis = value.QTimeEdit_0();
    return rsthis;
    // return 1;
  }
}

pub trait QTimeEdit_QTimeEdit_0 {
  fn QTimeEdit_0(self) -> QTimeEdit;
}
// QTimeEdit(QWidget *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QTimeEdit_QTimeEdit_0 for (usize) {
  fn QTimeEdit_0(self) -> QTimeEdit {
    // unsafe{_ZN9QTimeEditC2EP7QWidget()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QTimeEditC2EP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTimeEdit{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdatetimeedit.h:208
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QTimeEdit(const QTime &, QWidget *)

/*

*/
// QTimeEdit(const QTime &, QWidget *) ctx.fn_proto_cpp
impl /*struct*/ QTimeEdit {
  pub fn QTimeEdit_1<T: QTimeEdit_QTimeEdit_1>(value: T) -> QTimeEdit {
    let rsthis = value.QTimeEdit_1();
    return rsthis;
    // return 1;
  }
}

pub trait QTimeEdit_QTimeEdit_1 {
  fn QTimeEdit_1(self) -> QTimeEdit;
}
// QTimeEdit(const QTime &, QWidget *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QTimeEdit_QTimeEdit_1 for (usize,usize) {
  fn QTimeEdit_1(self) -> QTimeEdit {
    // unsafe{_ZN9QTimeEditC2ERK5QTimeP7QWidget()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QTimeEditC2ERK5QTimeP7QWidget", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTimeEdit{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdatetimeedit.h:209
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QTimeEdit()

/*

*/
pub fn DeleteQTimeEdit(this :*mut QTimeEdit) {
    // let rv = qtrt::InvokeQtFunc6("_ZN9QTimeEditD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 48)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qdatetimeedit.h:212
// index:0
// Public Visibility=Default Availability=Available
// [-2] void userTimeChanged(const QTime &)

/*

*/
impl /*struct*/ QTimeEdit {
  pub fn userTimeChanged_0<RetType, T: QTimeEdit_userTimeChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.userTimeChanged_0(self);
    // return 1;
  }
}
pub trait QTimeEdit_userTimeChanged_0<RetType> {
  fn userTimeChanged_0(self , rsthis: & QTimeEdit) -> RetType;
}
impl<'a> /*trait*/ QTimeEdit_userTimeChanged_0<(/*void*/)> for (usize) {
  fn userTimeChanged_0(self , rsthis: & QTimeEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QTimeEdit15userTimeChangedERK5QTime", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

//  body block end

//  keep block begin

//  keep block end
