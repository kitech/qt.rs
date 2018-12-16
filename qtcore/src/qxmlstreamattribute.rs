

// mod ::core::QXmlStreamAttribute
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
// extern C begin: 12
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
#[derive(Default)] // class sizeof(QXmlStreamAttribute)=80
pub struct QXmlStreamAttribute {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QXmlStreamAttribute_ITF interface {
//    QXmlStreamAttribute_PTR() *QXmlStreamAttribute
//}
//func (ptr *QXmlStreamAttribute) QXmlStreamAttribute_PTR() *QXmlStreamAttribute { return ptr }

impl /*struct*/ QXmlStreamAttribute {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QXmlStreamAttribute {
    return QXmlStreamAttribute{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QXmlStreamAttribute {
//  type Target = QXmlStreamAttributeBASE;
//
//  fn deref(&self) -> &QXmlStreamAttributeBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QXmlStreamAttributeBASE> for QXmlStreamAttribute {
//  fn as_ref(& self) -> & QXmlStreamAttributeBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qxmlstream.h:109
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QXmlStreamAttribute()

/*

*/
// QXmlStreamAttribute() ctx.fn_proto_cpp
impl /*struct*/ QXmlStreamAttribute {
  pub fn QXmlStreamAttribute_0<T: QXmlStreamAttribute_QXmlStreamAttribute_0>(value: T) -> QXmlStreamAttribute {
    let rsthis = value.QXmlStreamAttribute_0();
    return rsthis;
    // return 1;
  }
}

pub trait QXmlStreamAttribute_QXmlStreamAttribute_0 {
  fn QXmlStreamAttribute_0(self) -> QXmlStreamAttribute;
}
// QXmlStreamAttribute() ctx.fn_proto_cpp
impl<'a> /*trait*/ QXmlStreamAttribute_QXmlStreamAttribute_0 for () {
  fn QXmlStreamAttribute_0(self) -> QXmlStreamAttribute {
    // unsafe{_ZN19QXmlStreamAttributeC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN19QXmlStreamAttributeC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QXmlStreamAttribute{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:110
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QXmlStreamAttribute(const QString &, const QString &)

/*

*/
// QXmlStreamAttribute(const QString &, const QString &) ctx.fn_proto_cpp
impl /*struct*/ QXmlStreamAttribute {
  pub fn QXmlStreamAttribute_1<T: QXmlStreamAttribute_QXmlStreamAttribute_1>(value: T) -> QXmlStreamAttribute {
    let rsthis = value.QXmlStreamAttribute_1();
    return rsthis;
    // return 1;
  }
}

pub trait QXmlStreamAttribute_QXmlStreamAttribute_1 {
  fn QXmlStreamAttribute_1(self) -> QXmlStreamAttribute;
}
// QXmlStreamAttribute(const QString &, const QString &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QXmlStreamAttribute_QXmlStreamAttribute_1 for (usize,usize) {
  fn QXmlStreamAttribute_1(self) -> QXmlStreamAttribute {
    // unsafe{_ZN19QXmlStreamAttributeC2ERK7QStringS2_()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN19QXmlStreamAttributeC2ERK7QStringS2_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QXmlStreamAttribute{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:112
// index:2
// Public Visibility=Default Availability=Available
// [-2] void QXmlStreamAttribute(const QString &, const QString &, const QString &)

/*

*/
// QXmlStreamAttribute(const QString &, const QString &, const QString &) ctx.fn_proto_cpp
impl /*struct*/ QXmlStreamAttribute {
  pub fn QXmlStreamAttribute_2<T: QXmlStreamAttribute_QXmlStreamAttribute_2>(value: T) -> QXmlStreamAttribute {
    let rsthis = value.QXmlStreamAttribute_2();
    return rsthis;
    // return 1;
  }
}

pub trait QXmlStreamAttribute_QXmlStreamAttribute_2 {
  fn QXmlStreamAttribute_2(self) -> QXmlStreamAttribute;
}
// QXmlStreamAttribute(const QString &, const QString &, const QString &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QXmlStreamAttribute_QXmlStreamAttribute_2 for (usize,usize,usize) {
  fn QXmlStreamAttribute_2(self) -> QXmlStreamAttribute {
    // unsafe{_ZN19QXmlStreamAttributeC2ERK7QStringS2_S2_()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN19QXmlStreamAttributeC2ERK7QStringS2_S2_", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QXmlStreamAttribute{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:125
// index:0
// Public inline Visibility=Default Availability=Available
// [80] QXmlStreamAttribute & operator=(QXmlStreamAttribute &&)

/*

*/
impl /*struct*/ QXmlStreamAttribute {
  pub fn operator_equal_0<RetType, T: QXmlStreamAttribute_operator_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_0(self);
    // return 1;
  }
}
pub trait QXmlStreamAttribute_operator_equal_0<RetType> {
  fn operator_equal_0(self , rsthis: & QXmlStreamAttribute) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamAttribute_operator_equal_0<usize> for (usize) {
  fn operator_equal_0(self , rsthis: & QXmlStreamAttribute) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN19QXmlStreamAttributeaSEOS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:136
// index:1
// Public Visibility=Default Availability=Available
// [80] QXmlStreamAttribute & operator=(const QXmlStreamAttribute &)

/*

*/
impl /*struct*/ QXmlStreamAttribute {
  pub fn operator_equal_1<RetType, T: QXmlStreamAttribute_operator_equal_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_1(self);
    // return 1;
  }
}
pub trait QXmlStreamAttribute_operator_equal_1<RetType> {
  fn operator_equal_1(self , rsthis: & QXmlStreamAttribute) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamAttribute_operator_equal_1<usize> for (usize) {
  fn operator_equal_1(self , rsthis: & QXmlStreamAttribute) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN19QXmlStreamAttributeaSERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:137
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ~QXmlStreamAttribute()

/*

*/
pub fn DeleteQXmlStreamAttribute(this :*mut QXmlStreamAttribute) {
    // let rv = qtrt::InvokeQtFunc6("_ZN19QXmlStreamAttributeD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 80)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qxmlstream.h:140
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QStringRef namespaceUri() const

/*

*/
impl /*struct*/ QXmlStreamAttribute {
  pub fn namespaceUri_0<RetType, T: QXmlStreamAttribute_namespaceUri_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.namespaceUri_0(self);
    // return 1;
  }
}
pub trait QXmlStreamAttribute_namespaceUri_0<RetType> {
  fn namespaceUri_0(self , rsthis: & QXmlStreamAttribute) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamAttribute_namespaceUri_0<usize> for () {
  fn namespaceUri_0(self , rsthis: & QXmlStreamAttribute) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QXmlStreamAttribute12namespaceUriEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:141
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QStringRef name() const

/*

*/
impl /*struct*/ QXmlStreamAttribute {
  pub fn name_0<RetType, T: QXmlStreamAttribute_name_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.name_0(self);
    // return 1;
  }
}
pub trait QXmlStreamAttribute_name_0<RetType> {
  fn name_0(self , rsthis: & QXmlStreamAttribute) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamAttribute_name_0<usize> for () {
  fn name_0(self , rsthis: & QXmlStreamAttribute) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QXmlStreamAttribute4nameEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:142
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QStringRef qualifiedName() const

/*

*/
impl /*struct*/ QXmlStreamAttribute {
  pub fn qualifiedName_0<RetType, T: QXmlStreamAttribute_qualifiedName_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.qualifiedName_0(self);
    // return 1;
  }
}
pub trait QXmlStreamAttribute_qualifiedName_0<RetType> {
  fn qualifiedName_0(self , rsthis: & QXmlStreamAttribute) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamAttribute_qualifiedName_0<usize> for () {
  fn qualifiedName_0(self , rsthis: & QXmlStreamAttribute) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QXmlStreamAttribute13qualifiedNameEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:143
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QStringRef prefix() const

/*

*/
impl /*struct*/ QXmlStreamAttribute {
  pub fn prefix_0<RetType, T: QXmlStreamAttribute_prefix_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.prefix_0(self);
    // return 1;
  }
}
pub trait QXmlStreamAttribute_prefix_0<RetType> {
  fn prefix_0(self , rsthis: & QXmlStreamAttribute) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamAttribute_prefix_0<usize> for () {
  fn prefix_0(self , rsthis: & QXmlStreamAttribute) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QXmlStreamAttribute6prefixEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:148
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QStringRef value() const

/*

*/
impl /*struct*/ QXmlStreamAttribute {
  pub fn value_0<RetType, T: QXmlStreamAttribute_value_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.value_0(self);
    // return 1;
  }
}
pub trait QXmlStreamAttribute_value_0<RetType> {
  fn value_0(self , rsthis: & QXmlStreamAttribute) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamAttribute_value_0<usize> for () {
  fn value_0(self , rsthis: & QXmlStreamAttribute) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QXmlStreamAttribute5valueEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:149
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isDefault() const

/*

*/
impl /*struct*/ QXmlStreamAttribute {
  pub fn isDefault_0<RetType, T: QXmlStreamAttribute_isDefault_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isDefault_0(self);
    // return 1;
  }
}
pub trait QXmlStreamAttribute_isDefault_0<RetType> {
  fn isDefault_0(self , rsthis: & QXmlStreamAttribute) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamAttribute_isDefault_0<bool> for () {
  fn isDefault_0(self , rsthis: & QXmlStreamAttribute) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QXmlStreamAttribute9isDefaultEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:150
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator==(const QXmlStreamAttribute &) const

/*

*/
impl /*struct*/ QXmlStreamAttribute {
  pub fn operator_equal_equal_0<RetType, T: QXmlStreamAttribute_operator_equal_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_equal_0(self);
    // return 1;
  }
}
pub trait QXmlStreamAttribute_operator_equal_equal_0<RetType> {
  fn operator_equal_equal_0(self , rsthis: & QXmlStreamAttribute) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamAttribute_operator_equal_equal_0<bool> for (usize) {
  fn operator_equal_equal_0(self , rsthis: & QXmlStreamAttribute) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QXmlStreamAttributeeqERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:155
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator!=(const QXmlStreamAttribute &) const

/*

*/
impl /*struct*/ QXmlStreamAttribute {
  pub fn operator_not_equal_0<RetType, T: QXmlStreamAttribute_operator_not_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_not_equal_0(self);
    // return 1;
  }
}
pub trait QXmlStreamAttribute_operator_not_equal_0<RetType> {
  fn operator_not_equal_0(self , rsthis: & QXmlStreamAttribute) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamAttribute_operator_not_equal_0<bool> for (usize) {
  fn operator_not_equal_0(self , rsthis: & QXmlStreamAttribute) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QXmlStreamAttributeneERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
