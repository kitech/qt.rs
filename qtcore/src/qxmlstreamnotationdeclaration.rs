

// mod ::core::QXmlStreamNotationDeclaration
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
#[derive(Default)] // class sizeof(QXmlStreamNotationDeclaration)=56
pub struct QXmlStreamNotationDeclaration {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QXmlStreamNotationDeclaration_ITF interface {
//    QXmlStreamNotationDeclaration_PTR() *QXmlStreamNotationDeclaration
//}
//func (ptr *QXmlStreamNotationDeclaration) QXmlStreamNotationDeclaration_PTR() *QXmlStreamNotationDeclaration { return ptr }

impl /*struct*/ QXmlStreamNotationDeclaration {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QXmlStreamNotationDeclaration {
    return QXmlStreamNotationDeclaration{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QXmlStreamNotationDeclaration {
//  type Target = QXmlStreamNotationDeclarationBASE;
//
//  fn deref(&self) -> &QXmlStreamNotationDeclarationBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QXmlStreamNotationDeclarationBASE> for QXmlStreamNotationDeclaration {
//  fn as_ref(& self) -> & QXmlStreamNotationDeclarationBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qxmlstream.h:241
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QXmlStreamNotationDeclaration()

/*

*/
// QXmlStreamNotationDeclaration() ctx.fn_proto_cpp
impl /*struct*/ QXmlStreamNotationDeclaration {
  pub fn QXmlStreamNotationDeclaration_0<T: QXmlStreamNotationDeclaration_QXmlStreamNotationDeclaration_0>(value: T) -> QXmlStreamNotationDeclaration {
    let rsthis = value.QXmlStreamNotationDeclaration_0();
    return rsthis;
    // return 1;
  }
}

pub trait QXmlStreamNotationDeclaration_QXmlStreamNotationDeclaration_0 {
  fn QXmlStreamNotationDeclaration_0(self) -> QXmlStreamNotationDeclaration;
}
// QXmlStreamNotationDeclaration() ctx.fn_proto_cpp
impl<'a> /*trait*/ QXmlStreamNotationDeclaration_QXmlStreamNotationDeclaration_0 for () {
  fn QXmlStreamNotationDeclaration_0(self) -> QXmlStreamNotationDeclaration {
    // unsafe{_ZN29QXmlStreamNotationDeclarationC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN29QXmlStreamNotationDeclarationC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QXmlStreamNotationDeclaration{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:243
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ~QXmlStreamNotationDeclaration()

/*

*/
pub fn DeleteQXmlStreamNotationDeclaration(this :*mut QXmlStreamNotationDeclaration) {
    // let rv = qtrt::InvokeQtFunc6("_ZN29QXmlStreamNotationDeclarationD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 56)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qxmlstream.h:253
// index:0
// Public Visibility=Default Availability=Available
// [56] QXmlStreamNotationDeclaration & operator=(const QXmlStreamNotationDeclaration &)

/*

*/
impl /*struct*/ QXmlStreamNotationDeclaration {
  pub fn operator_equal_0<RetType, T: QXmlStreamNotationDeclaration_operator_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_0(self);
    // return 1;
  }
}
pub trait QXmlStreamNotationDeclaration_operator_equal_0<RetType> {
  fn operator_equal_0(self , rsthis: & QXmlStreamNotationDeclaration) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamNotationDeclaration_operator_equal_0<usize> for (usize) {
  fn operator_equal_0(self , rsthis: & QXmlStreamNotationDeclaration) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN29QXmlStreamNotationDeclarationaSERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:254
// index:1
// Public inline Visibility=Default Availability=Available
// [56] QXmlStreamNotationDeclaration & operator=(QXmlStreamNotationDeclaration &&)

/*

*/
impl /*struct*/ QXmlStreamNotationDeclaration {
  pub fn operator_equal_1<RetType, T: QXmlStreamNotationDeclaration_operator_equal_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_1(self);
    // return 1;
  }
}
pub trait QXmlStreamNotationDeclaration_operator_equal_1<RetType> {
  fn operator_equal_1(self , rsthis: & QXmlStreamNotationDeclaration) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamNotationDeclaration_operator_equal_1<usize> for (usize) {
  fn operator_equal_1(self , rsthis: & QXmlStreamNotationDeclaration) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN29QXmlStreamNotationDeclarationaSEOS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:264
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QStringRef name() const

/*

*/
impl /*struct*/ QXmlStreamNotationDeclaration {
  pub fn name_0<RetType, T: QXmlStreamNotationDeclaration_name_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.name_0(self);
    // return 1;
  }
}
pub trait QXmlStreamNotationDeclaration_name_0<RetType> {
  fn name_0(self , rsthis: & QXmlStreamNotationDeclaration) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamNotationDeclaration_name_0<usize> for () {
  fn name_0(self , rsthis: & QXmlStreamNotationDeclaration) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK29QXmlStreamNotationDeclaration4nameEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:265
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QStringRef systemId() const

/*

*/
impl /*struct*/ QXmlStreamNotationDeclaration {
  pub fn systemId_0<RetType, T: QXmlStreamNotationDeclaration_systemId_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.systemId_0(self);
    // return 1;
  }
}
pub trait QXmlStreamNotationDeclaration_systemId_0<RetType> {
  fn systemId_0(self , rsthis: & QXmlStreamNotationDeclaration) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamNotationDeclaration_systemId_0<usize> for () {
  fn systemId_0(self , rsthis: & QXmlStreamNotationDeclaration) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK29QXmlStreamNotationDeclaration8systemIdEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:266
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QStringRef publicId() const

/*

*/
impl /*struct*/ QXmlStreamNotationDeclaration {
  pub fn publicId_0<RetType, T: QXmlStreamNotationDeclaration_publicId_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.publicId_0(self);
    // return 1;
  }
}
pub trait QXmlStreamNotationDeclaration_publicId_0<RetType> {
  fn publicId_0(self , rsthis: & QXmlStreamNotationDeclaration) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamNotationDeclaration_publicId_0<usize> for () {
  fn publicId_0(self , rsthis: & QXmlStreamNotationDeclaration) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK29QXmlStreamNotationDeclaration8publicIdEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:267
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator==(const QXmlStreamNotationDeclaration &) const

/*

*/
impl /*struct*/ QXmlStreamNotationDeclaration {
  pub fn operator_equal_equal_0<RetType, T: QXmlStreamNotationDeclaration_operator_equal_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_equal_0(self);
    // return 1;
  }
}
pub trait QXmlStreamNotationDeclaration_operator_equal_equal_0<RetType> {
  fn operator_equal_equal_0(self , rsthis: & QXmlStreamNotationDeclaration) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamNotationDeclaration_operator_equal_equal_0<bool> for (usize) {
  fn operator_equal_equal_0(self , rsthis: & QXmlStreamNotationDeclaration) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK29QXmlStreamNotationDeclarationeqERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:271
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator!=(const QXmlStreamNotationDeclaration &) const

/*

*/
impl /*struct*/ QXmlStreamNotationDeclaration {
  pub fn operator_not_equal_0<RetType, T: QXmlStreamNotationDeclaration_operator_not_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_not_equal_0(self);
    // return 1;
  }
}
pub trait QXmlStreamNotationDeclaration_operator_not_equal_0<RetType> {
  fn operator_not_equal_0(self , rsthis: & QXmlStreamNotationDeclaration) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamNotationDeclaration_operator_not_equal_0<bool> for (usize) {
  fn operator_not_equal_0(self , rsthis: & QXmlStreamNotationDeclaration) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK29QXmlStreamNotationDeclarationneERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
