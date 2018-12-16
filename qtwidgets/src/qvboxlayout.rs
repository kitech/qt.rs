

// mod ::widgets::QVBoxLayout
// package qtwidgets
// /usr/include/qt/QtWidgets/qboxlayout.h
// #include <qboxlayout.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 4
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
#[derive(Default)] // class sizeof(QVBoxLayout)=32
pub struct QVBoxLayout {
  qbase: QBoxLayout,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QVBoxLayout_ITF interface {
//    QBoxLayout_ITF
//    QVBoxLayout_PTR() *QVBoxLayout
//}
//func (ptr *QVBoxLayout) QVBoxLayout_PTR() *QVBoxLayout { return ptr }

impl /*struct*/ QVBoxLayout {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QVBoxLayout {
    return QVBoxLayout{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QVBoxLayout {
//  type Target = QVBoxLayoutBASE;
//
//  fn deref(&self) -> &QVBoxLayoutBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QVBoxLayoutBASE> for QVBoxLayout {
//  fn as_ref(& self) -> & QVBoxLayoutBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qboxlayout.h:128
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QVBoxLayout {
  pub fn metaObject_0<RetType, T: QVBoxLayout_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QVBoxLayout_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QVBoxLayout) -> RetType;
}
impl<'a> /*trait*/ QVBoxLayout_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QVBoxLayout) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QVBoxLayout10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qboxlayout.h:130
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QVBoxLayout()

/*

*/
// QVBoxLayout() ctx.fn_proto_cpp
impl /*struct*/ QVBoxLayout {
  pub fn QVBoxLayout_0<T: QVBoxLayout_QVBoxLayout_0>(value: T) -> QVBoxLayout {
    let rsthis = value.QVBoxLayout_0();
    return rsthis;
    // return 1;
  }
}

pub trait QVBoxLayout_QVBoxLayout_0 {
  fn QVBoxLayout_0(self) -> QVBoxLayout;
}
// QVBoxLayout() ctx.fn_proto_cpp
impl<'a> /*trait*/ QVBoxLayout_QVBoxLayout_0 for () {
  fn QVBoxLayout_0(self) -> QVBoxLayout {
    // unsafe{_ZN11QVBoxLayoutC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QVBoxLayoutC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QVBoxLayout{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qboxlayout.h:131
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QVBoxLayout(QWidget *)

/*

*/
// QVBoxLayout(QWidget *) ctx.fn_proto_cpp
impl /*struct*/ QVBoxLayout {
  pub fn QVBoxLayout_1<T: QVBoxLayout_QVBoxLayout_1>(value: T) -> QVBoxLayout {
    let rsthis = value.QVBoxLayout_1();
    return rsthis;
    // return 1;
  }
}

pub trait QVBoxLayout_QVBoxLayout_1 {
  fn QVBoxLayout_1(self) -> QVBoxLayout;
}
// QVBoxLayout(QWidget *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QVBoxLayout_QVBoxLayout_1 for (usize) {
  fn QVBoxLayout_1(self) -> QVBoxLayout {
    // unsafe{_ZN11QVBoxLayoutC2EP7QWidget()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QVBoxLayoutC2EP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QVBoxLayout{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qboxlayout.h:132
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QVBoxLayout()

/*

*/
pub fn DeleteQVBoxLayout(this :*mut QVBoxLayout) {
    // let rv = qtrt::InvokeQtFunc6("_ZN11QVBoxLayoutD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 32)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
