

// mod ::widgets::QDateEdit
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
#[derive(Default)] // class sizeof(QDateEdit)=48
pub struct QDateEdit {
  qbase: QDateTimeEdit,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QDateEdit_ITF interface {
//    QDateTimeEdit_ITF
//    QDateEdit_PTR() *QDateEdit
//}
//func (ptr *QDateEdit) QDateEdit_PTR() *QDateEdit { return ptr }

impl /*struct*/ QDateEdit {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QDateEdit {
    return QDateEdit{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QDateEdit {
//  type Target = QDateEditBASE;
//
//  fn deref(&self) -> &QDateEditBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QDateEditBASE> for QDateEdit {
//  fn as_ref(& self) -> & QDateEditBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qdatetimeedit.h:217
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QDateEdit {
  pub fn metaObject_0<RetType, T: QDateEdit_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QDateEdit_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QDateEdit) -> RetType;
}
impl<'a> /*trait*/ QDateEdit_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QDateEdit) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QDateEdit10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdatetimeedit.h:220
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QDateEdit(QWidget *)

/*

*/
// QDateEdit(QWidget *) ctx.fn_proto_cpp
impl /*struct*/ QDateEdit {
  pub fn QDateEdit_0<T: QDateEdit_QDateEdit_0>(value: T) -> QDateEdit {
    let rsthis = value.QDateEdit_0();
    return rsthis;
    // return 1;
  }
}

pub trait QDateEdit_QDateEdit_0 {
  fn QDateEdit_0(self) -> QDateEdit;
}
// QDateEdit(QWidget *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QDateEdit_QDateEdit_0 for (usize) {
  fn QDateEdit_0(self) -> QDateEdit {
    // unsafe{_ZN9QDateEditC2EP7QWidget()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QDateEditC2EP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QDateEdit{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdatetimeedit.h:221
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QDateEdit(const QDate &, QWidget *)

/*

*/
// QDateEdit(const QDate &, QWidget *) ctx.fn_proto_cpp
impl /*struct*/ QDateEdit {
  pub fn QDateEdit_1<T: QDateEdit_QDateEdit_1>(value: T) -> QDateEdit {
    let rsthis = value.QDateEdit_1();
    return rsthis;
    // return 1;
  }
}

pub trait QDateEdit_QDateEdit_1 {
  fn QDateEdit_1(self) -> QDateEdit;
}
// QDateEdit(const QDate &, QWidget *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QDateEdit_QDateEdit_1 for (usize,usize) {
  fn QDateEdit_1(self) -> QDateEdit {
    // unsafe{_ZN9QDateEditC2ERK5QDateP7QWidget()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QDateEditC2ERK5QDateP7QWidget", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QDateEdit{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdatetimeedit.h:222
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QDateEdit()

/*

*/
pub fn DeleteQDateEdit(this :*mut QDateEdit) {
    // let rv = qtrt::InvokeQtFunc6("_ZN9QDateEditD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 48)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qdatetimeedit.h:225
// index:0
// Public Visibility=Default Availability=Available
// [-2] void userDateChanged(const QDate &)

/*

*/
impl /*struct*/ QDateEdit {
  pub fn userDateChanged_0<RetType, T: QDateEdit_userDateChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.userDateChanged_0(self);
    // return 1;
  }
}
pub trait QDateEdit_userDateChanged_0<RetType> {
  fn userDateChanged_0(self , rsthis: & QDateEdit) -> RetType;
}
impl<'a> /*trait*/ QDateEdit_userDateChanged_0<(/*void*/)> for (usize) {
  fn userDateChanged_0(self , rsthis: & QDateEdit) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QDateEdit15userDateChangedERK5QDate", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

//  body block end

//  keep block begin

//  keep block end
