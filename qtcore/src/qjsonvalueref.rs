

// mod ::core::QJsonValueRef
// package qtcore
// /usr/include/qt/QtCore/qjsonvalue.h
// #include <qjsonvalue.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 38
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
#[derive(Default)] // class sizeof(QJsonValueRef)=16
pub struct QJsonValueRef {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QJsonValueRef_ITF interface {
//    QJsonValueRef_PTR() *QJsonValueRef
//}
//func (ptr *QJsonValueRef) QJsonValueRef_PTR() *QJsonValueRef { return ptr }

impl /*struct*/ QJsonValueRef {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QJsonValueRef {
    return QJsonValueRef{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QJsonValueRef {
//  type Target = QJsonValueRefBASE;
//
//  fn deref(&self) -> &QJsonValueRefBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QJsonValueRefBASE> for QJsonValueRef {
//  fn as_ref(& self) -> & QJsonValueRefBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qjsonvalue.h:174
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void QJsonValueRef(QJsonArray *, int)

/*

*/
// QJsonValueRef(QJsonArray *, int) ctx.fn_proto_cpp
impl /*struct*/ QJsonValueRef {
  pub fn QJsonValueRef_0<T: QJsonValueRef_QJsonValueRef_0>(value: T) -> QJsonValueRef {
    let rsthis = value.QJsonValueRef_0();
    return rsthis;
    // return 1;
  }
}

pub trait QJsonValueRef_QJsonValueRef_0 {
  fn QJsonValueRef_0(self) -> QJsonValueRef;
}
// QJsonValueRef(QJsonArray *, int) ctx.fn_proto_cpp
impl<'a> /*trait*/ QJsonValueRef_QJsonValueRef_0 for (usize,i32) {
  fn QJsonValueRef_0(self) -> QJsonValueRef {
    // unsafe{_ZN13QJsonValueRefC2EP10QJsonArrayi()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN13QJsonValueRefC2EP10QJsonArrayi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QJsonValueRef{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qjsonvalue.h:176
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void QJsonValueRef(QJsonObject *, int)

/*

*/
// QJsonValueRef(QJsonObject *, int) ctx.fn_proto_cpp
impl /*struct*/ QJsonValueRef {
  pub fn QJsonValueRef_1<T: QJsonValueRef_QJsonValueRef_1>(value: T) -> QJsonValueRef {
    let rsthis = value.QJsonValueRef_1();
    return rsthis;
    // return 1;
  }
}

pub trait QJsonValueRef_QJsonValueRef_1 {
  fn QJsonValueRef_1(self) -> QJsonValueRef;
}
// QJsonValueRef(QJsonObject *, int) ctx.fn_proto_cpp
impl<'a> /*trait*/ QJsonValueRef_QJsonValueRef_1 for (usize,i32) {
  fn QJsonValueRef_1(self) -> QJsonValueRef {
    // unsafe{_ZN13QJsonValueRefC2EP11QJsonObjecti()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN13QJsonValueRefC2EP11QJsonObjecti", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QJsonValueRef{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qjsonvalue.h:180
// index:0
// Public Visibility=Default Availability=Available
// [16] QJsonValueRef & operator=(const QJsonValue &)

/*

*/
impl /*struct*/ QJsonValueRef {
  pub fn operator_equal_0<RetType, T: QJsonValueRef_operator_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_0(self);
    // return 1;
  }
}
pub trait QJsonValueRef_operator_equal_0<RetType> {
  fn operator_equal_0(self , rsthis: & QJsonValueRef) -> RetType;
}
impl<'a> /*trait*/ QJsonValueRef_operator_equal_0<usize> for (usize) {
  fn operator_equal_0(self , rsthis: & QJsonValueRef) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN13QJsonValueRefaSERK10QJsonValue", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonvalue.h:181
// index:1
// Public Visibility=Default Availability=Available
// [16] QJsonValueRef & operator=(const QJsonValueRef &)

/*

*/
impl /*struct*/ QJsonValueRef {
  pub fn operator_equal_1<RetType, T: QJsonValueRef_operator_equal_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_1(self);
    // return 1;
  }
}
pub trait QJsonValueRef_operator_equal_1<RetType> {
  fn operator_equal_1(self , rsthis: & QJsonValueRef) -> RetType;
}
impl<'a> /*trait*/ QJsonValueRef_operator_equal_1<usize> for (usize) {
  fn operator_equal_1(self , rsthis: & QJsonValueRef) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN13QJsonValueRefaSERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonvalue.h:183
// index:0
// Public Visibility=Default Availability=Available
// [16] QVariant toVariant() const

/*
Converts the value to a QVariant().

The QJsonValue types will be converted as follows:

ConstantDescription
NullQMetaType::Nullptr
BoolQMetaType::Bool
DoubleQMetaType::Double
StringQString
ArrayQVariantList
ObjectQVariantMap
UndefinedQVariant()


See also fromVariant().
*/
impl /*struct*/ QJsonValueRef {
  pub fn toVariant_0<RetType, T: QJsonValueRef_toVariant_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toVariant_0(self);
    // return 1;
  }
}
pub trait QJsonValueRef_toVariant_0<RetType> {
  fn toVariant_0(self , rsthis: & QJsonValueRef) -> RetType;
}
impl<'a> /*trait*/ QJsonValueRef_toVariant_0<usize> for () {
  fn toVariant_0(self , rsthis: & QJsonValueRef) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QJsonValueRef9toVariantEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonvalue.h:184
// index:0
// Public inline Visibility=Default Availability=Available
// [4] QJsonValue::Type type() const

/*
Returns the type of the value.

See also QJsonValue::Type.
*/
impl /*struct*/ QJsonValueRef {
  pub fn type__0<RetType, T: QJsonValueRef_type__0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.type__0(self);
    // return 1;
  }
}
pub trait QJsonValueRef_type__0<RetType> {
  fn type__0(self , rsthis: & QJsonValueRef) -> RetType;
}
impl<'a> /*trait*/ QJsonValueRef_type__0<i32> for () {
  fn type__0(self , rsthis: & QJsonValueRef) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QJsonValueRef4typeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonvalue.h:185
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isNull() const

/*
Returns true if the value is null.
*/
impl /*struct*/ QJsonValueRef {
  pub fn isNull_0<RetType, T: QJsonValueRef_isNull_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isNull_0(self);
    // return 1;
  }
}
pub trait QJsonValueRef_isNull_0<RetType> {
  fn isNull_0(self , rsthis: & QJsonValueRef) -> RetType;
}
impl<'a> /*trait*/ QJsonValueRef_isNull_0<bool> for () {
  fn isNull_0(self , rsthis: & QJsonValueRef) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QJsonValueRef6isNullEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonvalue.h:186
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isBool() const

/*
Returns true if the value contains a boolean.

See also toBool().
*/
impl /*struct*/ QJsonValueRef {
  pub fn isBool_0<RetType, T: QJsonValueRef_isBool_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isBool_0(self);
    // return 1;
  }
}
pub trait QJsonValueRef_isBool_0<RetType> {
  fn isBool_0(self , rsthis: & QJsonValueRef) -> RetType;
}
impl<'a> /*trait*/ QJsonValueRef_isBool_0<bool> for () {
  fn isBool_0(self , rsthis: & QJsonValueRef) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QJsonValueRef6isBoolEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonvalue.h:187
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isDouble() const

/*
Returns true if the value contains a double.

See also toDouble().
*/
impl /*struct*/ QJsonValueRef {
  pub fn isDouble_0<RetType, T: QJsonValueRef_isDouble_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isDouble_0(self);
    // return 1;
  }
}
pub trait QJsonValueRef_isDouble_0<RetType> {
  fn isDouble_0(self , rsthis: & QJsonValueRef) -> RetType;
}
impl<'a> /*trait*/ QJsonValueRef_isDouble_0<bool> for () {
  fn isDouble_0(self , rsthis: & QJsonValueRef) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QJsonValueRef8isDoubleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonvalue.h:188
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isString() const

/*
Returns true if the value contains a string.

See also toString().
*/
impl /*struct*/ QJsonValueRef {
  pub fn isString_0<RetType, T: QJsonValueRef_isString_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isString_0(self);
    // return 1;
  }
}
pub trait QJsonValueRef_isString_0<RetType> {
  fn isString_0(self , rsthis: & QJsonValueRef) -> RetType;
}
impl<'a> /*trait*/ QJsonValueRef_isString_0<bool> for () {
  fn isString_0(self , rsthis: & QJsonValueRef) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QJsonValueRef8isStringEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonvalue.h:189
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isArray() const

/*
Returns true if the value contains an array.

See also toArray().
*/
impl /*struct*/ QJsonValueRef {
  pub fn isArray_0<RetType, T: QJsonValueRef_isArray_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isArray_0(self);
    // return 1;
  }
}
pub trait QJsonValueRef_isArray_0<RetType> {
  fn isArray_0(self , rsthis: & QJsonValueRef) -> RetType;
}
impl<'a> /*trait*/ QJsonValueRef_isArray_0<bool> for () {
  fn isArray_0(self , rsthis: & QJsonValueRef) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QJsonValueRef7isArrayEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonvalue.h:190
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isObject() const

/*
Returns true if the value contains an object.

See also toObject().
*/
impl /*struct*/ QJsonValueRef {
  pub fn isObject_0<RetType, T: QJsonValueRef_isObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isObject_0(self);
    // return 1;
  }
}
pub trait QJsonValueRef_isObject_0<RetType> {
  fn isObject_0(self , rsthis: & QJsonValueRef) -> RetType;
}
impl<'a> /*trait*/ QJsonValueRef_isObject_0<bool> for () {
  fn isObject_0(self , rsthis: & QJsonValueRef) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QJsonValueRef8isObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonvalue.h:191
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isUndefined() const

/*
Returns true if the value is undefined. This can happen in certain error cases as e.g. accessing a non existing key in a QJsonObject.
*/
impl /*struct*/ QJsonValueRef {
  pub fn isUndefined_0<RetType, T: QJsonValueRef_isUndefined_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isUndefined_0(self);
    // return 1;
  }
}
pub trait QJsonValueRef_isUndefined_0<RetType> {
  fn isUndefined_0(self , rsthis: & QJsonValueRef) -> RetType;
}
impl<'a> /*trait*/ QJsonValueRef_isUndefined_0<bool> for () {
  fn isUndefined_0(self , rsthis: & QJsonValueRef) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QJsonValueRef11isUndefinedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonvalue.h:193
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool toBool() const

/*
Converts the value to a bool and returns it.

If type() is not bool, the defaultValue will be returned.
*/
impl /*struct*/ QJsonValueRef {
  pub fn toBool_0<RetType, T: QJsonValueRef_toBool_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toBool_0(self);
    // return 1;
  }
}
pub trait QJsonValueRef_toBool_0<RetType> {
  fn toBool_0(self , rsthis: & QJsonValueRef) -> RetType;
}
impl<'a> /*trait*/ QJsonValueRef_toBool_0<bool> for () {
  fn toBool_0(self , rsthis: & QJsonValueRef) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QJsonValueRef6toBoolEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonvalue.h:201
// index:1
// Public inline Visibility=Default Availability=Available
// [1] bool toBool(bool) const

/*
Converts the value to a bool and returns it.

If type() is not bool, the defaultValue will be returned.
*/
impl /*struct*/ QJsonValueRef {
  pub fn toBool_1<RetType, T: QJsonValueRef_toBool_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toBool_1(self);
    // return 1;
  }
}
pub trait QJsonValueRef_toBool_1<RetType> {
  fn toBool_1(self , rsthis: & QJsonValueRef) -> RetType;
}
impl<'a> /*trait*/ QJsonValueRef_toBool_1<bool> for (bool) {
  fn toBool_1(self , rsthis: & QJsonValueRef) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QJsonValueRef6toBoolEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonvalue.h:194
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int toInt() const

/*
Converts the value to an int and returns it.

If type() is not Double or the value is not a whole number, the defaultValue will be returned.
*/
impl /*struct*/ QJsonValueRef {
  pub fn toInt_0<RetType, T: QJsonValueRef_toInt_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toInt_0(self);
    // return 1;
  }
}
pub trait QJsonValueRef_toInt_0<RetType> {
  fn toInt_0(self , rsthis: & QJsonValueRef) -> RetType;
}
impl<'a> /*trait*/ QJsonValueRef_toInt_0<i32> for () {
  fn toInt_0(self , rsthis: & QJsonValueRef) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QJsonValueRef5toIntEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonvalue.h:202
// index:1
// Public inline Visibility=Default Availability=Available
// [4] int toInt(int) const

/*
Converts the value to an int and returns it.

If type() is not Double or the value is not a whole number, the defaultValue will be returned.
*/
impl /*struct*/ QJsonValueRef {
  pub fn toInt_1<RetType, T: QJsonValueRef_toInt_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toInt_1(self);
    // return 1;
  }
}
pub trait QJsonValueRef_toInt_1<RetType> {
  fn toInt_1(self , rsthis: & QJsonValueRef) -> RetType;
}
impl<'a> /*trait*/ QJsonValueRef_toInt_1<i32> for (i32) {
  fn toInt_1(self , rsthis: & QJsonValueRef) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QJsonValueRef5toIntEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonvalue.h:195
// index:0
// Public inline Visibility=Default Availability=Available
// [8] double toDouble() const

/*
Converts the value to a double and returns it.

If type() is not Double, the defaultValue will be returned.
*/
impl /*struct*/ QJsonValueRef {
  pub fn toDouble_0<RetType, T: QJsonValueRef_toDouble_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toDouble_0(self);
    // return 1;
  }
}
pub trait QJsonValueRef_toDouble_0<RetType> {
  fn toDouble_0(self , rsthis: & QJsonValueRef) -> RetType;
}
impl<'a> /*trait*/ QJsonValueRef_toDouble_0<f64> for () {
  fn toDouble_0(self , rsthis: & QJsonValueRef) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QJsonValueRef8toDoubleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonvalue.h:203
// index:1
// Public inline Visibility=Default Availability=Available
// [8] double toDouble(double) const

/*
Converts the value to a double and returns it.

If type() is not Double, the defaultValue will be returned.
*/
impl /*struct*/ QJsonValueRef {
  pub fn toDouble_1<RetType, T: QJsonValueRef_toDouble_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toDouble_1(self);
    // return 1;
  }
}
pub trait QJsonValueRef_toDouble_1<RetType> {
  fn toDouble_1(self , rsthis: & QJsonValueRef) -> RetType;
}
impl<'a> /*trait*/ QJsonValueRef_toDouble_1<f64> for (f64) {
  fn toDouble_1(self , rsthis: & QJsonValueRef) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QJsonValueRef8toDoubleEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonvalue.h:196
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QString toString() const

/*
Converts the value to a QString and returns it.

If type() is not String, a null QString will be returned.

See also QString::isNull().
*/
impl /*struct*/ QJsonValueRef {
  pub fn toString_0<RetType, T: QJsonValueRef_toString_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toString_0(self);
    // return 1;
  }
}
pub trait QJsonValueRef_toString_0<RetType> {
  fn toString_0(self , rsthis: & QJsonValueRef) -> RetType;
}
impl<'a> /*trait*/ QJsonValueRef_toString_0<usize> for () {
  fn toString_0(self , rsthis: & QJsonValueRef) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QJsonValueRef8toStringEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonvalue.h:204
// index:1
// Public inline Visibility=Default Availability=Available
// [8] QString toString(const QString &) const

/*
Converts the value to a QString and returns it.

If type() is not String, a null QString will be returned.

See also QString::isNull().
*/
impl /*struct*/ QJsonValueRef {
  pub fn toString_1<RetType, T: QJsonValueRef_toString_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toString_1(self);
    // return 1;
  }
}
pub trait QJsonValueRef_toString_1<RetType> {
  fn toString_1(self , rsthis: & QJsonValueRef) -> RetType;
}
impl<'a> /*trait*/ QJsonValueRef_toString_1<usize> for (usize) {
  fn toString_1(self , rsthis: & QJsonValueRef) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QJsonValueRef8toStringERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonvalue.h:197
// index:0
// Public Visibility=Default Availability=Available
// [16] QJsonArray toArray() const

/*
Converts the value to an array and returns it.

If type() is not Array, the defaultValue will be returned.
*/
impl /*struct*/ QJsonValueRef {
  pub fn toArray_0<RetType, T: QJsonValueRef_toArray_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toArray_0(self);
    // return 1;
  }
}
pub trait QJsonValueRef_toArray_0<RetType> {
  fn toArray_0(self , rsthis: & QJsonValueRef) -> RetType;
}
impl<'a> /*trait*/ QJsonValueRef_toArray_0<usize> for () {
  fn toArray_0(self , rsthis: & QJsonValueRef) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QJsonValueRef7toArrayEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonvalue.h:198
// index:0
// Public Visibility=Default Availability=Available
// [16] QJsonObject toObject() const

/*
Converts the value to an object and returns it.

If type() is not Object, the defaultValue will be returned.
*/
impl /*struct*/ QJsonValueRef {
  pub fn toObject_0<RetType, T: QJsonValueRef_toObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toObject_0(self);
    // return 1;
  }
}
pub trait QJsonValueRef_toObject_0<RetType> {
  fn toObject_0(self , rsthis: & QJsonValueRef) -> RetType;
}
impl<'a> /*trait*/ QJsonValueRef_toObject_0<usize> for () {
  fn toObject_0(self , rsthis: & QJsonValueRef) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QJsonValueRef8toObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonvalue.h:206
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator==(const QJsonValue &) const

/*

*/
impl /*struct*/ QJsonValueRef {
  pub fn operator_equal_equal_0<RetType, T: QJsonValueRef_operator_equal_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_equal_0(self);
    // return 1;
  }
}
pub trait QJsonValueRef_operator_equal_equal_0<RetType> {
  fn operator_equal_equal_0(self , rsthis: & QJsonValueRef) -> RetType;
}
impl<'a> /*trait*/ QJsonValueRef_operator_equal_equal_0<bool> for (usize) {
  fn operator_equal_equal_0(self , rsthis: & QJsonValueRef) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QJsonValueRefeqERK10QJsonValue", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonvalue.h:207
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator!=(const QJsonValue &) const

/*

*/
impl /*struct*/ QJsonValueRef {
  pub fn operator_not_equal_0<RetType, T: QJsonValueRef_operator_not_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_not_equal_0(self);
    // return 1;
  }
}
pub trait QJsonValueRef_operator_not_equal_0<RetType> {
  fn operator_not_equal_0(self , rsthis: & QJsonValueRef) -> RetType;
}
impl<'a> /*trait*/ QJsonValueRef_operator_not_equal_0<bool> for (usize) {
  fn operator_not_equal_0(self , rsthis: & QJsonValueRef) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QJsonValueRefneERK10QJsonValue", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}


pub fn DeleteQJsonValueRef(this :*mut QJsonValueRef) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN13QJsonValueRefD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
