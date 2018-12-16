

// mod ::widgets::QHBoxLayout
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
// extern C begin: 36
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
#[derive(Default)] // class sizeof(QHBoxLayout)=32
pub struct QHBoxLayout {
  qbase: QBoxLayout,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QHBoxLayout_ITF interface {
//    QBoxLayout_ITF
//    QHBoxLayout_PTR() *QHBoxLayout
//}
//func (ptr *QHBoxLayout) QHBoxLayout_PTR() *QHBoxLayout { return ptr }

impl /*struct*/ QHBoxLayout {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QHBoxLayout {
    return QHBoxLayout{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QHBoxLayout {
//  type Target = QHBoxLayoutBASE;
//
//  fn deref(&self) -> &QHBoxLayoutBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QHBoxLayoutBASE> for QHBoxLayout {
//  fn as_ref(& self) -> & QHBoxLayoutBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qboxlayout.h:115
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QHBoxLayout {
  pub fn metaObject_0<RetType, T: QHBoxLayout_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QHBoxLayout_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QHBoxLayout) -> RetType;
}
impl<'a> /*trait*/ QHBoxLayout_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QHBoxLayout) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QHBoxLayout10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qboxlayout.h:117
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QHBoxLayout()

/*

*/
// QHBoxLayout() ctx.fn_proto_cpp
impl /*struct*/ QHBoxLayout {
  pub fn QHBoxLayout_0<T: QHBoxLayout_QHBoxLayout_0>(value: T) -> QHBoxLayout {
    let rsthis = value.QHBoxLayout_0();
    return rsthis;
    // return 1;
  }
}

pub trait QHBoxLayout_QHBoxLayout_0 {
  fn QHBoxLayout_0(self) -> QHBoxLayout;
}
// QHBoxLayout() ctx.fn_proto_cpp
impl<'a> /*trait*/ QHBoxLayout_QHBoxLayout_0 for () {
  fn QHBoxLayout_0(self) -> QHBoxLayout {
    // unsafe{_ZN11QHBoxLayoutC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QHBoxLayoutC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QHBoxLayout{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qboxlayout.h:118
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QHBoxLayout(QWidget *)

/*

*/
// QHBoxLayout(QWidget *) ctx.fn_proto_cpp
impl /*struct*/ QHBoxLayout {
  pub fn QHBoxLayout_1<T: QHBoxLayout_QHBoxLayout_1>(value: T) -> QHBoxLayout {
    let rsthis = value.QHBoxLayout_1();
    return rsthis;
    // return 1;
  }
}

pub trait QHBoxLayout_QHBoxLayout_1 {
  fn QHBoxLayout_1(self) -> QHBoxLayout;
}
// QHBoxLayout(QWidget *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QHBoxLayout_QHBoxLayout_1 for (usize) {
  fn QHBoxLayout_1(self) -> QHBoxLayout {
    // unsafe{_ZN11QHBoxLayoutC2EP7QWidget()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QHBoxLayoutC2EP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QHBoxLayout{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qboxlayout.h:119
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QHBoxLayout()

/*

*/
pub fn DeleteQHBoxLayout(this :*mut QHBoxLayout) {
    // let rv = qtrt::InvokeQtFunc6("_ZN11QHBoxLayoutD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 32)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
