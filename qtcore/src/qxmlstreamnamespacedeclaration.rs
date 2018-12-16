

// mod ::core::QXmlStreamNamespaceDeclaration
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
#[derive(Default)] // class sizeof(QXmlStreamNamespaceDeclaration)=40
pub struct QXmlStreamNamespaceDeclaration {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QXmlStreamNamespaceDeclaration_ITF interface {
//    QXmlStreamNamespaceDeclaration_PTR() *QXmlStreamNamespaceDeclaration
//}
//func (ptr *QXmlStreamNamespaceDeclaration) QXmlStreamNamespaceDeclaration_PTR() *QXmlStreamNamespaceDeclaration { return ptr }

impl /*struct*/ QXmlStreamNamespaceDeclaration {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QXmlStreamNamespaceDeclaration {
    return QXmlStreamNamespaceDeclaration{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QXmlStreamNamespaceDeclaration {
//  type Target = QXmlStreamNamespaceDeclarationBASE;
//
//  fn deref(&self) -> &QXmlStreamNamespaceDeclarationBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QXmlStreamNamespaceDeclarationBASE> for QXmlStreamNamespaceDeclaration {
//  fn as_ref(& self) -> & QXmlStreamNamespaceDeclarationBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qxmlstream.h:199
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QXmlStreamNamespaceDeclaration()

/*

*/
// QXmlStreamNamespaceDeclaration() ctx.fn_proto_cpp
impl /*struct*/ QXmlStreamNamespaceDeclaration {
  pub fn QXmlStreamNamespaceDeclaration_0<T: QXmlStreamNamespaceDeclaration_QXmlStreamNamespaceDeclaration_0>(value: T) -> QXmlStreamNamespaceDeclaration {
    let rsthis = value.QXmlStreamNamespaceDeclaration_0();
    return rsthis;
    // return 1;
  }
}

pub trait QXmlStreamNamespaceDeclaration_QXmlStreamNamespaceDeclaration_0 {
  fn QXmlStreamNamespaceDeclaration_0(self) -> QXmlStreamNamespaceDeclaration;
}
// QXmlStreamNamespaceDeclaration() ctx.fn_proto_cpp
impl<'a> /*trait*/ QXmlStreamNamespaceDeclaration_QXmlStreamNamespaceDeclaration_0 for () {
  fn QXmlStreamNamespaceDeclaration_0(self) -> QXmlStreamNamespaceDeclaration {
    // unsafe{_ZN30QXmlStreamNamespaceDeclarationC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN30QXmlStreamNamespaceDeclarationC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QXmlStreamNamespaceDeclaration{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:216
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QXmlStreamNamespaceDeclaration(const QString &, const QString &)

/*

*/
// QXmlStreamNamespaceDeclaration(const QString &, const QString &) ctx.fn_proto_cpp
impl /*struct*/ QXmlStreamNamespaceDeclaration {
  pub fn QXmlStreamNamespaceDeclaration_1<T: QXmlStreamNamespaceDeclaration_QXmlStreamNamespaceDeclaration_1>(value: T) -> QXmlStreamNamespaceDeclaration {
    let rsthis = value.QXmlStreamNamespaceDeclaration_1();
    return rsthis;
    // return 1;
  }
}

pub trait QXmlStreamNamespaceDeclaration_QXmlStreamNamespaceDeclaration_1 {
  fn QXmlStreamNamespaceDeclaration_1(self) -> QXmlStreamNamespaceDeclaration;
}
// QXmlStreamNamespaceDeclaration(const QString &, const QString &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QXmlStreamNamespaceDeclaration_QXmlStreamNamespaceDeclaration_1 for (usize,usize) {
  fn QXmlStreamNamespaceDeclaration_1(self) -> QXmlStreamNamespaceDeclaration {
    // unsafe{_ZN30QXmlStreamNamespaceDeclarationC2ERK7QStringS2_()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN30QXmlStreamNamespaceDeclarationC2ERK7QStringS2_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QXmlStreamNamespaceDeclaration{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:209
// index:0
// Public inline Visibility=Default Availability=Available
// [40] QXmlStreamNamespaceDeclaration & operator=(QXmlStreamNamespaceDeclaration &&)

/*

*/
impl /*struct*/ QXmlStreamNamespaceDeclaration {
  pub fn operator_equal_0<RetType, T: QXmlStreamNamespaceDeclaration_operator_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_0(self);
    // return 1;
  }
}
pub trait QXmlStreamNamespaceDeclaration_operator_equal_0<RetType> {
  fn operator_equal_0(self , rsthis: & QXmlStreamNamespaceDeclaration) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamNamespaceDeclaration_operator_equal_0<usize> for (usize) {
  fn operator_equal_0(self , rsthis: & QXmlStreamNamespaceDeclaration) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN30QXmlStreamNamespaceDeclarationaSEOS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:218
// index:1
// Public Visibility=Default Availability=Available
// [40] QXmlStreamNamespaceDeclaration & operator=(const QXmlStreamNamespaceDeclaration &)

/*

*/
impl /*struct*/ QXmlStreamNamespaceDeclaration {
  pub fn operator_equal_1<RetType, T: QXmlStreamNamespaceDeclaration_operator_equal_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_1(self);
    // return 1;
  }
}
pub trait QXmlStreamNamespaceDeclaration_operator_equal_1<RetType> {
  fn operator_equal_1(self , rsthis: & QXmlStreamNamespaceDeclaration) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamNamespaceDeclaration_operator_equal_1<usize> for (usize) {
  fn operator_equal_1(self , rsthis: & QXmlStreamNamespaceDeclaration) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN30QXmlStreamNamespaceDeclarationaSERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:217
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ~QXmlStreamNamespaceDeclaration()

/*

*/
pub fn DeleteQXmlStreamNamespaceDeclaration(this :*mut QXmlStreamNamespaceDeclaration) {
    // let rv = qtrt::InvokeQtFunc6("_ZN30QXmlStreamNamespaceDeclarationD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 40)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qxmlstream.h:221
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QStringRef prefix() const

/*

*/
impl /*struct*/ QXmlStreamNamespaceDeclaration {
  pub fn prefix_0<RetType, T: QXmlStreamNamespaceDeclaration_prefix_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.prefix_0(self);
    // return 1;
  }
}
pub trait QXmlStreamNamespaceDeclaration_prefix_0<RetType> {
  fn prefix_0(self , rsthis: & QXmlStreamNamespaceDeclaration) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamNamespaceDeclaration_prefix_0<usize> for () {
  fn prefix_0(self , rsthis: & QXmlStreamNamespaceDeclaration) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK30QXmlStreamNamespaceDeclaration6prefixEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:222
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QStringRef namespaceUri() const

/*

*/
impl /*struct*/ QXmlStreamNamespaceDeclaration {
  pub fn namespaceUri_0<RetType, T: QXmlStreamNamespaceDeclaration_namespaceUri_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.namespaceUri_0(self);
    // return 1;
  }
}
pub trait QXmlStreamNamespaceDeclaration_namespaceUri_0<RetType> {
  fn namespaceUri_0(self , rsthis: & QXmlStreamNamespaceDeclaration) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamNamespaceDeclaration_namespaceUri_0<usize> for () {
  fn namespaceUri_0(self , rsthis: & QXmlStreamNamespaceDeclaration) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK30QXmlStreamNamespaceDeclaration12namespaceUriEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:223
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator==(const QXmlStreamNamespaceDeclaration &) const

/*

*/
impl /*struct*/ QXmlStreamNamespaceDeclaration {
  pub fn operator_equal_equal_0<RetType, T: QXmlStreamNamespaceDeclaration_operator_equal_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_equal_0(self);
    // return 1;
  }
}
pub trait QXmlStreamNamespaceDeclaration_operator_equal_equal_0<RetType> {
  fn operator_equal_equal_0(self , rsthis: & QXmlStreamNamespaceDeclaration) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamNamespaceDeclaration_operator_equal_equal_0<bool> for (usize) {
  fn operator_equal_equal_0(self , rsthis: & QXmlStreamNamespaceDeclaration) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK30QXmlStreamNamespaceDeclarationeqERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:226
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator!=(const QXmlStreamNamespaceDeclaration &) const

/*

*/
impl /*struct*/ QXmlStreamNamespaceDeclaration {
  pub fn operator_not_equal_0<RetType, T: QXmlStreamNamespaceDeclaration_operator_not_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_not_equal_0(self);
    // return 1;
  }
}
pub trait QXmlStreamNamespaceDeclaration_operator_not_equal_0<RetType> {
  fn operator_not_equal_0(self , rsthis: & QXmlStreamNamespaceDeclaration) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamNamespaceDeclaration_operator_not_equal_0<bool> for (usize) {
  fn operator_not_equal_0(self , rsthis: & QXmlStreamNamespaceDeclaration) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK30QXmlStreamNamespaceDeclarationneERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
