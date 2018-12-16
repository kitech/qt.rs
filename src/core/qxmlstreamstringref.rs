

// mod ::core::QXmlStreamStringRef
// package qtcore
// /usr/include/qt/QtCore/qxmlstream.h
// #include <qxmlstream.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 8
*/
// import "C"
// import "unsafe"
// import "reflect"
// import "fmt"
// import "log"
// import "github.com/kitech/qt.go/qtrt"
use std::default::Default;
use std::ops::Deref;
use super::super::qtrt;
use super::*;
//  ext block end

//  body block begin



/*

*/
#[derive(Default)] // class sizeof(QXmlStreamStringRef)=16
pub struct QXmlStreamStringRef {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QXmlStreamStringRef_ITF interface {
//    QXmlStreamStringRef_PTR() *QXmlStreamStringRef
//}
//func (ptr *QXmlStreamStringRef) QXmlStreamStringRef_PTR() *QXmlStreamStringRef { return ptr }

impl /*struct*/ QXmlStreamStringRef {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QXmlStreamStringRef {
    return QXmlStreamStringRef{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QXmlStreamStringRef {
//  type Target = QXmlStreamStringRefBASE;
//
//  fn deref(&self) -> &QXmlStreamStringRefBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QXmlStreamStringRefBASE> for QXmlStreamStringRef {
//  fn as_ref(& self) -> & QXmlStreamStringRefBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qxmlstream.h:58
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void QXmlStreamStringRef()

/*

*/
// QXmlStreamStringRef() ctx.fn_proto_cpp
impl /*struct*/ QXmlStreamStringRef {
  pub fn QXmlStreamStringRef_0<T: QXmlStreamStringRef_QXmlStreamStringRef_0>(value: T) -> QXmlStreamStringRef {
    let rsthis = value.QXmlStreamStringRef_0();
    return rsthis;
    // return 1;
  }
}

pub trait QXmlStreamStringRef_QXmlStreamStringRef_0 {
  fn QXmlStreamStringRef_0(self) -> QXmlStreamStringRef;
}
// QXmlStreamStringRef() ctx.fn_proto_cpp
impl<'a> /*trait*/ QXmlStreamStringRef_QXmlStreamStringRef_0 for () {
  fn QXmlStreamStringRef_0(self) -> QXmlStreamStringRef {
    // unsafe{_ZN19QXmlStreamStringRefC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN19QXmlStreamStringRefC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QXmlStreamStringRef{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:59
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void QXmlStreamStringRef(const QStringRef &)

/*

*/
// QXmlStreamStringRef(const QStringRef &) ctx.fn_proto_cpp
impl /*struct*/ QXmlStreamStringRef {
  pub fn QXmlStreamStringRef_1<T: QXmlStreamStringRef_QXmlStreamStringRef_1>(value: T) -> QXmlStreamStringRef {
    let rsthis = value.QXmlStreamStringRef_1();
    return rsthis;
    // return 1;
  }
}

pub trait QXmlStreamStringRef_QXmlStreamStringRef_1 {
  fn QXmlStreamStringRef_1(self) -> QXmlStreamStringRef;
}
// QXmlStreamStringRef(const QStringRef &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QXmlStreamStringRef_QXmlStreamStringRef_1 for (usize) {
  fn QXmlStreamStringRef_1(self) -> QXmlStreamStringRef {
    // unsafe{_ZN19QXmlStreamStringRefC2ERK10QStringRef()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN19QXmlStreamStringRefC2ERK10QStringRef", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QXmlStreamStringRef{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:61
// index:2
// Public inline Visibility=Default Availability=Available
// [-2] void QXmlStreamStringRef(const QString &)

/*

*/
// QXmlStreamStringRef(const QString &) ctx.fn_proto_cpp
impl /*struct*/ QXmlStreamStringRef {
  pub fn QXmlStreamStringRef_2<T: QXmlStreamStringRef_QXmlStreamStringRef_2>(value: T) -> QXmlStreamStringRef {
    let rsthis = value.QXmlStreamStringRef_2();
    return rsthis;
    // return 1;
  }
}

pub trait QXmlStreamStringRef_QXmlStreamStringRef_2 {
  fn QXmlStreamStringRef_2(self) -> QXmlStreamStringRef;
}
// QXmlStreamStringRef(const QString &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QXmlStreamStringRef_QXmlStreamStringRef_2 for (usize) {
  fn QXmlStreamStringRef_2(self) -> QXmlStreamStringRef {
    // unsafe{_ZN19QXmlStreamStringRefC2ERK7QString()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN19QXmlStreamStringRefC2ERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QXmlStreamStringRef{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:63
// index:3
// Public inline Visibility=Default Availability=Available
// [-2] void QXmlStreamStringRef(QString &&)

/*

*/
// QXmlStreamStringRef(QString &&) ctx.fn_proto_cpp
impl /*struct*/ QXmlStreamStringRef {
  pub fn QXmlStreamStringRef_3<T: QXmlStreamStringRef_QXmlStreamStringRef_3>(value: T) -> QXmlStreamStringRef {
    let rsthis = value.QXmlStreamStringRef_3();
    return rsthis;
    // return 1;
  }
}

pub trait QXmlStreamStringRef_QXmlStreamStringRef_3 {
  fn QXmlStreamStringRef_3(self) -> QXmlStreamStringRef;
}
// QXmlStreamStringRef(QString &&) ctx.fn_proto_cpp
impl<'a> /*trait*/ QXmlStreamStringRef_QXmlStreamStringRef_3 for (usize) {
  fn QXmlStreamStringRef_3(self) -> QXmlStreamStringRef {
    // unsafe{_ZN19QXmlStreamStringRefC2EO7QString()};
    let arg0 = (&self) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN19QXmlStreamStringRefC2EO7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QXmlStreamStringRef{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:72
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QXmlStreamStringRef & operator=(QXmlStreamStringRef &&)

/*

*/
impl /*struct*/ QXmlStreamStringRef {
  pub fn operator_equal_0<RetType, T: QXmlStreamStringRef_operator_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_0(self);
    // return 1;
  }
}
pub trait QXmlStreamStringRef_operator_equal_0<RetType> {
  fn operator_equal_0(self , rsthis: & QXmlStreamStringRef) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamStringRef_operator_equal_0<usize> for (usize) {
  fn operator_equal_0(self , rsthis: & QXmlStreamStringRef) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN19QXmlStreamStringRefaSEOS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:75
// index:1
// Public inline Visibility=Default Availability=Available
// [16] QXmlStreamStringRef & operator=(const QXmlStreamStringRef &)

/*

*/
impl /*struct*/ QXmlStreamStringRef {
  pub fn operator_equal_1<RetType, T: QXmlStreamStringRef_operator_equal_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_1(self);
    // return 1;
  }
}
pub trait QXmlStreamStringRef_operator_equal_1<RetType> {
  fn operator_equal_1(self , rsthis: & QXmlStreamStringRef) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamStringRef_operator_equal_1<usize> for (usize) {
  fn operator_equal_1(self , rsthis: & QXmlStreamStringRef) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN19QXmlStreamStringRefaSERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:77
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void ~QXmlStreamStringRef()

/*

*/
pub fn DeleteQXmlStreamStringRef(this :*mut QXmlStreamStringRef) {
    // let rv = qtrt::InvokeQtFunc6("_ZN19QXmlStreamStringRefD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qxmlstream.h:82
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void swap(QXmlStreamStringRef &)

/*

*/
impl /*struct*/ QXmlStreamStringRef {
  pub fn swap_0<RetType, T: QXmlStreamStringRef_swap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.swap_0(self);
    // return 1;
  }
}
pub trait QXmlStreamStringRef_swap_0<RetType> {
  fn swap_0(self , rsthis: & QXmlStreamStringRef) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamStringRef_swap_0<(/*void*/)> for (usize) {
  fn swap_0(self , rsthis: & QXmlStreamStringRef) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN19QXmlStreamStringRef4swapERS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:89
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void clear()

/*

*/
impl /*struct*/ QXmlStreamStringRef {
  pub fn clear_0<RetType, T: QXmlStreamStringRef_clear_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.clear_0(self);
    // return 1;
  }
}
pub trait QXmlStreamStringRef_clear_0<RetType> {
  fn clear_0(self , rsthis: & QXmlStreamStringRef) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamStringRef_clear_0<(/*void*/)> for () {
  fn clear_0(self , rsthis: & QXmlStreamStringRef) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN19QXmlStreamStringRef5clearEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:91
// index:0
// Public inline Visibility=Default Availability=Available
// [8] const QString * string() const

/*

*/
impl /*struct*/ QXmlStreamStringRef {
  pub fn string_0<RetType, T: QXmlStreamStringRef_string_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.string_0(self);
    // return 1;
  }
}
pub trait QXmlStreamStringRef_string_0<RetType> {
  fn string_0(self , rsthis: & QXmlStreamStringRef) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamStringRef_string_0<usize> for () {
  fn string_0(self , rsthis: & QXmlStreamStringRef) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QXmlStreamStringRef6stringEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:92
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int position() const

/*

*/
impl /*struct*/ QXmlStreamStringRef {
  pub fn position_0<RetType, T: QXmlStreamStringRef_position_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.position_0(self);
    // return 1;
  }
}
pub trait QXmlStreamStringRef_position_0<RetType> {
  fn position_0(self , rsthis: & QXmlStreamStringRef) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamStringRef_position_0<i32> for () {
  fn position_0(self , rsthis: & QXmlStreamStringRef) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QXmlStreamStringRef8positionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:93
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int size() const

/*

*/
impl /*struct*/ QXmlStreamStringRef {
  pub fn size_0<RetType, T: QXmlStreamStringRef_size_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.size_0(self);
    // return 1;
  }
}
pub trait QXmlStreamStringRef_size_0<RetType> {
  fn size_0(self , rsthis: & QXmlStreamStringRef) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamStringRef_size_0<i32> for () {
  fn size_0(self , rsthis: & QXmlStreamStringRef) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QXmlStreamStringRef4sizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
