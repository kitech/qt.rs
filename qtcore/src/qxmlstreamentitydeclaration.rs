

// mod ::core::QXmlStreamEntityDeclaration
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
// extern C begin: 9
*/
// import "C"
// import "unsafe"
// import "reflect"
// import "fmt"
// import "log"
// import "github.com/kitech/qt.go/qtrt"
use std::default::Default;
use std::ops::Deref;
use qtrt; // super::super::qtrt;
use super::*;
//  ext block end

//  body block begin



/*

*/
#[derive(Default)] // class sizeof(QXmlStreamEntityDeclaration)=88
pub struct QXmlStreamEntityDeclaration {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QXmlStreamEntityDeclaration_ITF interface {
//    QXmlStreamEntityDeclaration_PTR() *QXmlStreamEntityDeclaration
//}
//func (ptr *QXmlStreamEntityDeclaration) QXmlStreamEntityDeclaration_PTR() *QXmlStreamEntityDeclaration { return ptr }

impl /*struct*/ QXmlStreamEntityDeclaration {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QXmlStreamEntityDeclaration {
    return QXmlStreamEntityDeclaration{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QXmlStreamEntityDeclaration {
//  type Target = QXmlStreamEntityDeclarationBASE;
//
//  fn deref(&self) -> &QXmlStreamEntityDeclarationBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QXmlStreamEntityDeclarationBASE> for QXmlStreamEntityDeclaration {
//  fn as_ref(& self) -> & QXmlStreamEntityDeclarationBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qxmlstream.h:286
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QXmlStreamEntityDeclaration()

/*

*/
// QXmlStreamEntityDeclaration() ctx.fn_proto_cpp
impl /*struct*/ QXmlStreamEntityDeclaration {
  pub fn QXmlStreamEntityDeclaration_0<T: QXmlStreamEntityDeclaration_QXmlStreamEntityDeclaration_0>(value: T) -> QXmlStreamEntityDeclaration {
    let rsthis = value.QXmlStreamEntityDeclaration_0();
    return rsthis;
    // return 1;
  }
}

pub trait QXmlStreamEntityDeclaration_QXmlStreamEntityDeclaration_0 {
  fn QXmlStreamEntityDeclaration_0(self) -> QXmlStreamEntityDeclaration;
}
// QXmlStreamEntityDeclaration() ctx.fn_proto_cpp
impl<'a> /*trait*/ QXmlStreamEntityDeclaration_QXmlStreamEntityDeclaration_0 for () {
  fn QXmlStreamEntityDeclaration_0(self) -> QXmlStreamEntityDeclaration {
    // unsafe{_ZN27QXmlStreamEntityDeclarationC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN27QXmlStreamEntityDeclarationC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QXmlStreamEntityDeclaration{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:288
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ~QXmlStreamEntityDeclaration()

/*

*/
pub fn DeleteQXmlStreamEntityDeclaration(this :*mut QXmlStreamEntityDeclaration) {
    // let rv = qtrt::InvokeQtFunc6("_ZN27QXmlStreamEntityDeclarationD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 88)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qxmlstream.h:300
// index:0
// Public Visibility=Default Availability=Available
// [88] QXmlStreamEntityDeclaration & operator=(const QXmlStreamEntityDeclaration &)

/*

*/
impl /*struct*/ QXmlStreamEntityDeclaration {
  pub fn operator_equal_0<RetType, T: QXmlStreamEntityDeclaration_operator_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_0(self);
    // return 1;
  }
}
pub trait QXmlStreamEntityDeclaration_operator_equal_0<RetType> {
  fn operator_equal_0(self , rsthis: & QXmlStreamEntityDeclaration) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamEntityDeclaration_operator_equal_0<usize> for (usize) {
  fn operator_equal_0(self , rsthis: & QXmlStreamEntityDeclaration) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN27QXmlStreamEntityDeclarationaSERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:301
// index:1
// Public inline Visibility=Default Availability=Available
// [88] QXmlStreamEntityDeclaration & operator=(QXmlStreamEntityDeclaration &&)

/*

*/
impl /*struct*/ QXmlStreamEntityDeclaration {
  pub fn operator_equal_1<RetType, T: QXmlStreamEntityDeclaration_operator_equal_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_1(self);
    // return 1;
  }
}
pub trait QXmlStreamEntityDeclaration_operator_equal_1<RetType> {
  fn operator_equal_1(self , rsthis: & QXmlStreamEntityDeclaration) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamEntityDeclaration_operator_equal_1<usize> for (usize) {
  fn operator_equal_1(self , rsthis: & QXmlStreamEntityDeclaration) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN27QXmlStreamEntityDeclarationaSEOS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:313
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QStringRef name() const

/*

*/
impl /*struct*/ QXmlStreamEntityDeclaration {
  pub fn name_0<RetType, T: QXmlStreamEntityDeclaration_name_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.name_0(self);
    // return 1;
  }
}
pub trait QXmlStreamEntityDeclaration_name_0<RetType> {
  fn name_0(self , rsthis: & QXmlStreamEntityDeclaration) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamEntityDeclaration_name_0<usize> for () {
  fn name_0(self , rsthis: & QXmlStreamEntityDeclaration) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK27QXmlStreamEntityDeclaration4nameEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:314
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QStringRef notationName() const

/*

*/
impl /*struct*/ QXmlStreamEntityDeclaration {
  pub fn notationName_0<RetType, T: QXmlStreamEntityDeclaration_notationName_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.notationName_0(self);
    // return 1;
  }
}
pub trait QXmlStreamEntityDeclaration_notationName_0<RetType> {
  fn notationName_0(self , rsthis: & QXmlStreamEntityDeclaration) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamEntityDeclaration_notationName_0<usize> for () {
  fn notationName_0(self , rsthis: & QXmlStreamEntityDeclaration) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK27QXmlStreamEntityDeclaration12notationNameEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:315
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QStringRef systemId() const

/*

*/
impl /*struct*/ QXmlStreamEntityDeclaration {
  pub fn systemId_0<RetType, T: QXmlStreamEntityDeclaration_systemId_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.systemId_0(self);
    // return 1;
  }
}
pub trait QXmlStreamEntityDeclaration_systemId_0<RetType> {
  fn systemId_0(self , rsthis: & QXmlStreamEntityDeclaration) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamEntityDeclaration_systemId_0<usize> for () {
  fn systemId_0(self , rsthis: & QXmlStreamEntityDeclaration) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK27QXmlStreamEntityDeclaration8systemIdEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:316
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QStringRef publicId() const

/*

*/
impl /*struct*/ QXmlStreamEntityDeclaration {
  pub fn publicId_0<RetType, T: QXmlStreamEntityDeclaration_publicId_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.publicId_0(self);
    // return 1;
  }
}
pub trait QXmlStreamEntityDeclaration_publicId_0<RetType> {
  fn publicId_0(self , rsthis: & QXmlStreamEntityDeclaration) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamEntityDeclaration_publicId_0<usize> for () {
  fn publicId_0(self , rsthis: & QXmlStreamEntityDeclaration) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK27QXmlStreamEntityDeclaration8publicIdEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:317
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QStringRef value() const

/*

*/
impl /*struct*/ QXmlStreamEntityDeclaration {
  pub fn value_0<RetType, T: QXmlStreamEntityDeclaration_value_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.value_0(self);
    // return 1;
  }
}
pub trait QXmlStreamEntityDeclaration_value_0<RetType> {
  fn value_0(self , rsthis: & QXmlStreamEntityDeclaration) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamEntityDeclaration_value_0<usize> for () {
  fn value_0(self , rsthis: & QXmlStreamEntityDeclaration) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK27QXmlStreamEntityDeclaration5valueEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:318
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator==(const QXmlStreamEntityDeclaration &) const

/*

*/
impl /*struct*/ QXmlStreamEntityDeclaration {
  pub fn operator_equal_equal_0<RetType, T: QXmlStreamEntityDeclaration_operator_equal_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_equal_0(self);
    // return 1;
  }
}
pub trait QXmlStreamEntityDeclaration_operator_equal_equal_0<RetType> {
  fn operator_equal_equal_0(self , rsthis: & QXmlStreamEntityDeclaration) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamEntityDeclaration_operator_equal_equal_0<bool> for (usize) {
  fn operator_equal_equal_0(self , rsthis: & QXmlStreamEntityDeclaration) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK27QXmlStreamEntityDeclarationeqERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:325
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator!=(const QXmlStreamEntityDeclaration &) const

/*

*/
impl /*struct*/ QXmlStreamEntityDeclaration {
  pub fn operator_not_equal_0<RetType, T: QXmlStreamEntityDeclaration_operator_not_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_not_equal_0(self);
    // return 1;
  }
}
pub trait QXmlStreamEntityDeclaration_operator_not_equal_0<RetType> {
  fn operator_not_equal_0(self , rsthis: & QXmlStreamEntityDeclaration) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamEntityDeclaration_operator_not_equal_0<bool> for (usize) {
  fn operator_not_equal_0(self , rsthis: & QXmlStreamEntityDeclaration) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK27QXmlStreamEntityDeclarationneERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
