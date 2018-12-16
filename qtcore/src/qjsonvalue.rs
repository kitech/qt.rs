

// mod ::core::QJsonValue
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
// extern C begin: 7
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
#[derive(Default)] // class sizeof(QJsonValue)=24
pub struct QJsonValue {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QJsonValue_ITF interface {
//    QJsonValue_PTR() *QJsonValue
//}
//func (ptr *QJsonValue) QJsonValue_PTR() *QJsonValue { return ptr }

impl /*struct*/ QJsonValue {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QJsonValue {
    return QJsonValue{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QJsonValue {
//  type Target = QJsonValueBASE;
//
//  fn deref(&self) -> &QJsonValueBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QJsonValueBASE> for QJsonValue {
//  fn as_ref(& self) -> & QJsonValueBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qjsonvalue.h:76
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QJsonValue(QJsonValue::Type)

/*
Creates a QJsonValue of type type.

The default is to create a Null value.
*/
// QJsonValue(QJsonValue::Type) ctx.fn_proto_cpp
impl /*struct*/ QJsonValue {
  pub fn QJsonValue_0<T: QJsonValue_QJsonValue_0>(value: T) -> QJsonValue {
    let rsthis = value.QJsonValue_0();
    return rsthis;
    // return 1;
  }
}

pub trait QJsonValue_QJsonValue_0 {
  fn QJsonValue_0(self) -> QJsonValue;
}
// QJsonValue(QJsonValue::Type) ctx.fn_proto_cpp
impl<'a> /*trait*/ QJsonValue_QJsonValue_0 for (i32) {
  fn QJsonValue_0(self) -> QJsonValue {
    // unsafe{_ZN10QJsonValueC2ENS_4TypeE()};
    let arg0 = (&self) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN10QJsonValueC2ENS_4TypeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QJsonValue{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qjsonvalue.h:77
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QJsonValue(bool)

/*
Creates a QJsonValue of type type.

The default is to create a Null value.
*/
// QJsonValue(bool) ctx.fn_proto_cpp
impl /*struct*/ QJsonValue {
  pub fn QJsonValue_1<T: QJsonValue_QJsonValue_1>(value: T) -> QJsonValue {
    let rsthis = value.QJsonValue_1();
    return rsthis;
    // return 1;
  }
}

pub trait QJsonValue_QJsonValue_1 {
  fn QJsonValue_1(self) -> QJsonValue;
}
// QJsonValue(bool) ctx.fn_proto_cpp
impl<'a> /*trait*/ QJsonValue_QJsonValue_1 for (bool) {
  fn QJsonValue_1(self) -> QJsonValue {
    // unsafe{_ZN10QJsonValueC2Eb()};
    let arg0 = (&self) as *const bool as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN10QJsonValueC2Eb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QJsonValue{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qjsonvalue.h:78
// index:2
// Public Visibility=Default Availability=Available
// [-2] void QJsonValue(double)

/*
Creates a QJsonValue of type type.

The default is to create a Null value.
*/
// QJsonValue(double) ctx.fn_proto_cpp
impl /*struct*/ QJsonValue {
  pub fn QJsonValue_2<T: QJsonValue_QJsonValue_2>(value: T) -> QJsonValue {
    let rsthis = value.QJsonValue_2();
    return rsthis;
    // return 1;
  }
}

pub trait QJsonValue_QJsonValue_2 {
  fn QJsonValue_2(self) -> QJsonValue;
}
// QJsonValue(double) ctx.fn_proto_cpp
impl<'a> /*trait*/ QJsonValue_QJsonValue_2 for (f64) {
  fn QJsonValue_2(self) -> QJsonValue {
    // unsafe{_ZN10QJsonValueC2Ed()};
    let arg0 = (&self) as *const f64 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN10QJsonValueC2Ed", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QJsonValue{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qjsonvalue.h:79
// index:3
// Public Visibility=Default Availability=Available
// [-2] void QJsonValue(int)

/*
Creates a QJsonValue of type type.

The default is to create a Null value.
*/
// QJsonValue(int) ctx.fn_proto_cpp
impl /*struct*/ QJsonValue {
  pub fn QJsonValue_3<T: QJsonValue_QJsonValue_3>(value: T) -> QJsonValue {
    let rsthis = value.QJsonValue_3();
    return rsthis;
    // return 1;
  }
}

pub trait QJsonValue_QJsonValue_3 {
  fn QJsonValue_3(self) -> QJsonValue;
}
// QJsonValue(int) ctx.fn_proto_cpp
impl<'a> /*trait*/ QJsonValue_QJsonValue_3 for (i32) {
  fn QJsonValue_3(self) -> QJsonValue {
    // unsafe{_ZN10QJsonValueC2Ei()};
    let arg0 = (&self) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN10QJsonValueC2Ei", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QJsonValue{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qjsonvalue.h:80
// index:4
// Public Visibility=Default Availability=Available
// [-2] void QJsonValue(qint64)

/*
Creates a QJsonValue of type type.

The default is to create a Null value.
*/
// QJsonValue(qint64) ctx.fn_proto_cpp
impl /*struct*/ QJsonValue {
  pub fn QJsonValue_4<T: QJsonValue_QJsonValue_4>(value: T) -> QJsonValue {
    let rsthis = value.QJsonValue_4();
    return rsthis;
    // return 1;
  }
}

pub trait QJsonValue_QJsonValue_4 {
  fn QJsonValue_4(self) -> QJsonValue;
}
// QJsonValue(qint64) ctx.fn_proto_cpp
impl<'a> /*trait*/ QJsonValue_QJsonValue_4 for (i64) {
  fn QJsonValue_4(self) -> QJsonValue {
    // unsafe{_ZN10QJsonValueC2Ex()};
    let arg0 = (&self) as *const i64 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN10QJsonValueC2Ex", 1,qtrt::FFITY_SINT64,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QJsonValue{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qjsonvalue.h:81
// index:5
// Public Visibility=Default Availability=Available
// [-2] void QJsonValue(const QString &)

/*
Creates a QJsonValue of type type.

The default is to create a Null value.
*/
// QJsonValue(const QString &) ctx.fn_proto_cpp
impl /*struct*/ QJsonValue {
  pub fn QJsonValue_5<T: QJsonValue_QJsonValue_5>(value: T) -> QJsonValue {
    let rsthis = value.QJsonValue_5();
    return rsthis;
    // return 1;
  }
}

pub trait QJsonValue_QJsonValue_5 {
  fn QJsonValue_5(self) -> QJsonValue;
}
// QJsonValue(const QString &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QJsonValue_QJsonValue_5 for (usize) {
  fn QJsonValue_5(self) -> QJsonValue {
    // unsafe{_ZN10QJsonValueC2ERK7QString()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN10QJsonValueC2ERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QJsonValue{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qjsonvalue.h:82
// index:6
// Public Visibility=Default Availability=Available
// [-2] void QJsonValue(QLatin1String)

/*
Creates a QJsonValue of type type.

The default is to create a Null value.
*/
// QJsonValue(QLatin1String) ctx.fn_proto_cpp
impl /*struct*/ QJsonValue {
  pub fn QJsonValue_6<T: QJsonValue_QJsonValue_6>(value: T) -> QJsonValue {
    let rsthis = value.QJsonValue_6();
    return rsthis;
    // return 1;
  }
}

pub trait QJsonValue_QJsonValue_6 {
  fn QJsonValue_6(self) -> QJsonValue;
}
// QJsonValue(QLatin1String) ctx.fn_proto_cpp
impl<'a> /*trait*/ QJsonValue_QJsonValue_6 for (usize) {
  fn QJsonValue_6(self) -> QJsonValue {
    // unsafe{_ZN10QJsonValueC2E13QLatin1String()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN10QJsonValueC2E13QLatin1String", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QJsonValue{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qjsonvalue.h:84
// index:7
// Public inline Visibility=Default Availability=Available
// [-2] void QJsonValue(const char *)

/*
Creates a QJsonValue of type type.

The default is to create a Null value.
*/
// QJsonValue(const char *) ctx.fn_proto_cpp
impl /*struct*/ QJsonValue {
  pub fn QJsonValue_7<T: QJsonValue_QJsonValue_7>(value: T) -> QJsonValue {
    let rsthis = value.QJsonValue_7();
    return rsthis;
    // return 1;
  }
}

pub trait QJsonValue_QJsonValue_7 {
  fn QJsonValue_7(self) -> QJsonValue;
}
// QJsonValue(const char *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QJsonValue_QJsonValue_7 for (usize) {
  fn QJsonValue_7(self) -> QJsonValue {
    // unsafe{_ZN10QJsonValueC2EPKc()};
    let arg0 = (self) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN10QJsonValueC2EPKc", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QJsonValue{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qjsonvalue.h:87
// index:8
// Public Visibility=Default Availability=Available
// [-2] void QJsonValue(const QJsonArray &)

/*
Creates a QJsonValue of type type.

The default is to create a Null value.
*/
// QJsonValue(const QJsonArray &) ctx.fn_proto_cpp
impl /*struct*/ QJsonValue {
  pub fn QJsonValue_8<T: QJsonValue_QJsonValue_8>(value: T) -> QJsonValue {
    let rsthis = value.QJsonValue_8();
    return rsthis;
    // return 1;
  }
}

pub trait QJsonValue_QJsonValue_8 {
  fn QJsonValue_8(self) -> QJsonValue;
}
// QJsonValue(const QJsonArray &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QJsonValue_QJsonValue_8 for (usize) {
  fn QJsonValue_8(self) -> QJsonValue {
    // unsafe{_ZN10QJsonValueC2ERK10QJsonArray()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN10QJsonValueC2ERK10QJsonArray", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QJsonValue{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qjsonvalue.h:88
// index:9
// Public Visibility=Default Availability=Available
// [-2] void QJsonValue(const QJsonObject &)

/*
Creates a QJsonValue of type type.

The default is to create a Null value.
*/
// QJsonValue(const QJsonObject &) ctx.fn_proto_cpp
impl /*struct*/ QJsonValue {
  pub fn QJsonValue_9<T: QJsonValue_QJsonValue_9>(value: T) -> QJsonValue {
    let rsthis = value.QJsonValue_9();
    return rsthis;
    // return 1;
  }
}

pub trait QJsonValue_QJsonValue_9 {
  fn QJsonValue_9(self) -> QJsonValue;
}
// QJsonValue(const QJsonObject &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QJsonValue_QJsonValue_9 for (usize) {
  fn QJsonValue_9(self) -> QJsonValue {
    // unsafe{_ZN10QJsonValueC2ERK11QJsonObject()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN10QJsonValueC2ERK11QJsonObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QJsonValue{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qjsonvalue.h:90
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ~QJsonValue()

/*

*/
pub fn DeleteQJsonValue(this :*mut QJsonValue) {
    // let rv = qtrt::InvokeQtFunc6("_ZN10QJsonValueD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 24)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qjsonvalue.h:93
// index:0
// Public Visibility=Default Availability=Available
// [24] QJsonValue & operator=(const QJsonValue &)

/*

*/
impl /*struct*/ QJsonValue {
  pub fn operator_equal_0<RetType, T: QJsonValue_operator_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_0(self);
    // return 1;
  }
}
pub trait QJsonValue_operator_equal_0<RetType> {
  fn operator_equal_0(self , rsthis: & QJsonValue) -> RetType;
}
impl<'a> /*trait*/ QJsonValue_operator_equal_0<usize> for (usize) {
  fn operator_equal_0(self , rsthis: & QJsonValue) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QJsonValueaSERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonvalue.h:105
// index:1
// Public inline Visibility=Default Availability=Available
// [24] QJsonValue & operator=(QJsonValue &&)

/*

*/
impl /*struct*/ QJsonValue {
  pub fn operator_equal_1<RetType, T: QJsonValue_operator_equal_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_1(self);
    // return 1;
  }
}
pub trait QJsonValue_operator_equal_1<RetType> {
  fn operator_equal_1(self , rsthis: & QJsonValue) -> RetType;
}
impl<'a> /*trait*/ QJsonValue_operator_equal_1<usize> for (usize) {
  fn operator_equal_1(self , rsthis: & QJsonValue) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QJsonValueaSEOS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonvalue.h:111
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void swap(QJsonValue &)

/*
Swaps the value other with this. This operation is very fast and never fails.

This function was introduced in  Qt 5.10.
*/
impl /*struct*/ QJsonValue {
  pub fn swap_0<RetType, T: QJsonValue_swap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.swap_0(self);
    // return 1;
  }
}
pub trait QJsonValue_swap_0<RetType> {
  fn swap_0(self , rsthis: & QJsonValue) -> RetType;
}
impl<'a> /*trait*/ QJsonValue_swap_0<(/*void*/)> for (usize) {
  fn swap_0(self , rsthis: & QJsonValue) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN10QJsonValue4swapERS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qjsonvalue.h:118
// index:0
// Public static Visibility=Default Availability=Available
// [24] QJsonValue fromVariant(const QVariant &)

/*
Converts variant to a QJsonValue and returns it.

The conversion will convert QVariant types as follows:


 Source typeDestination type

QMetaType::Nullptr

QJsonValue::Null

QMetaType::Bool

QJsonValue::Bool

QMetaType::Int
QMetaType::UInt
QMetaType::LongLong
QMetaType::ULongLong
QMetaType::Float
QMetaType::Double

QJsonValue::Double

QMetaType::QString

QJsonValue::String

QMetaType::QStringList
QMetaType::QVariantList

QJsonValue::Array

QMetaType::QVariantMap
QMetaType::QVariantHash

QJsonValue::Object


For all other QVariant types a conversion to a QString will be attempted. If the returned string is empty, a Null QJsonValue will be stored, otherwise a String value using the returned QString.

See also toVariant().
*/
impl /*struct*/ QJsonValue {
  pub fn fromVariant_0<RetType, T: QJsonValue_fromVariant_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromVariant_0();
    // return 1;
  }
}
pub trait QJsonValue_fromVariant_0<RetType> {
  fn fromVariant_0(self ) -> RetType;
}
impl<'a> /*trait*/ QJsonValue_fromVariant_0<usize> for (usize) {
  fn fromVariant_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QJsonValue11fromVariantERK8QVariant", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonvalue.h:119
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
impl /*struct*/ QJsonValue {
  pub fn toVariant_0<RetType, T: QJsonValue_toVariant_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toVariant_0(self);
    // return 1;
  }
}
pub trait QJsonValue_toVariant_0<RetType> {
  fn toVariant_0(self , rsthis: & QJsonValue) -> RetType;
}
impl<'a> /*trait*/ QJsonValue_toVariant_0<usize> for () {
  fn toVariant_0(self , rsthis: & QJsonValue) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QJsonValue9toVariantEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonvalue.h:121
// index:0
// Public Visibility=Default Availability=Available
// [4] QJsonValue::Type type() const

/*
Returns the type of the value.

See also QJsonValue::Type.
*/
impl /*struct*/ QJsonValue {
  pub fn type__0<RetType, T: QJsonValue_type__0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.type__0(self);
    // return 1;
  }
}
pub trait QJsonValue_type__0<RetType> {
  fn type__0(self , rsthis: & QJsonValue) -> RetType;
}
impl<'a> /*trait*/ QJsonValue_type__0<i32> for () {
  fn type__0(self , rsthis: & QJsonValue) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QJsonValue4typeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonvalue.h:122
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isNull() const

/*
Returns true if the value is null.
*/
impl /*struct*/ QJsonValue {
  pub fn isNull_0<RetType, T: QJsonValue_isNull_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isNull_0(self);
    // return 1;
  }
}
pub trait QJsonValue_isNull_0<RetType> {
  fn isNull_0(self , rsthis: & QJsonValue) -> RetType;
}
impl<'a> /*trait*/ QJsonValue_isNull_0<bool> for () {
  fn isNull_0(self , rsthis: & QJsonValue) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QJsonValue6isNullEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonvalue.h:123
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isBool() const

/*
Returns true if the value contains a boolean.

See also toBool().
*/
impl /*struct*/ QJsonValue {
  pub fn isBool_0<RetType, T: QJsonValue_isBool_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isBool_0(self);
    // return 1;
  }
}
pub trait QJsonValue_isBool_0<RetType> {
  fn isBool_0(self , rsthis: & QJsonValue) -> RetType;
}
impl<'a> /*trait*/ QJsonValue_isBool_0<bool> for () {
  fn isBool_0(self , rsthis: & QJsonValue) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QJsonValue6isBoolEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonvalue.h:124
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isDouble() const

/*
Returns true if the value contains a double.

See also toDouble().
*/
impl /*struct*/ QJsonValue {
  pub fn isDouble_0<RetType, T: QJsonValue_isDouble_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isDouble_0(self);
    // return 1;
  }
}
pub trait QJsonValue_isDouble_0<RetType> {
  fn isDouble_0(self , rsthis: & QJsonValue) -> RetType;
}
impl<'a> /*trait*/ QJsonValue_isDouble_0<bool> for () {
  fn isDouble_0(self , rsthis: & QJsonValue) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QJsonValue8isDoubleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonvalue.h:125
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isString() const

/*
Returns true if the value contains a string.

See also toString().
*/
impl /*struct*/ QJsonValue {
  pub fn isString_0<RetType, T: QJsonValue_isString_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isString_0(self);
    // return 1;
  }
}
pub trait QJsonValue_isString_0<RetType> {
  fn isString_0(self , rsthis: & QJsonValue) -> RetType;
}
impl<'a> /*trait*/ QJsonValue_isString_0<bool> for () {
  fn isString_0(self , rsthis: & QJsonValue) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QJsonValue8isStringEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonvalue.h:126
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isArray() const

/*
Returns true if the value contains an array.

See also toArray().
*/
impl /*struct*/ QJsonValue {
  pub fn isArray_0<RetType, T: QJsonValue_isArray_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isArray_0(self);
    // return 1;
  }
}
pub trait QJsonValue_isArray_0<RetType> {
  fn isArray_0(self , rsthis: & QJsonValue) -> RetType;
}
impl<'a> /*trait*/ QJsonValue_isArray_0<bool> for () {
  fn isArray_0(self , rsthis: & QJsonValue) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QJsonValue7isArrayEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonvalue.h:127
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isObject() const

/*
Returns true if the value contains an object.

See also toObject().
*/
impl /*struct*/ QJsonValue {
  pub fn isObject_0<RetType, T: QJsonValue_isObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isObject_0(self);
    // return 1;
  }
}
pub trait QJsonValue_isObject_0<RetType> {
  fn isObject_0(self , rsthis: & QJsonValue) -> RetType;
}
impl<'a> /*trait*/ QJsonValue_isObject_0<bool> for () {
  fn isObject_0(self , rsthis: & QJsonValue) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QJsonValue8isObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonvalue.h:128
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isUndefined() const

/*
Returns true if the value is undefined. This can happen in certain error cases as e.g. accessing a non existing key in a QJsonObject.
*/
impl /*struct*/ QJsonValue {
  pub fn isUndefined_0<RetType, T: QJsonValue_isUndefined_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isUndefined_0(self);
    // return 1;
  }
}
pub trait QJsonValue_isUndefined_0<RetType> {
  fn isUndefined_0(self , rsthis: & QJsonValue) -> RetType;
}
impl<'a> /*trait*/ QJsonValue_isUndefined_0<bool> for () {
  fn isUndefined_0(self , rsthis: & QJsonValue) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QJsonValue11isUndefinedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonvalue.h:130
// index:0
// Public Visibility=Default Availability=Available
// [1] bool toBool(bool) const

/*
Converts the value to a bool and returns it.

If type() is not bool, the defaultValue will be returned.
*/
impl /*struct*/ QJsonValue {
  pub fn toBool_0<RetType, T: QJsonValue_toBool_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toBool_0(self);
    // return 1;
  }
}
pub trait QJsonValue_toBool_0<RetType> {
  fn toBool_0(self , rsthis: & QJsonValue) -> RetType;
}
impl<'a> /*trait*/ QJsonValue_toBool_0<bool> for (bool) {
  fn toBool_0(self , rsthis: & QJsonValue) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QJsonValue6toBoolEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonvalue.h:131
// index:0
// Public Visibility=Default Availability=Available
// [4] int toInt(int) const

/*
Converts the value to an int and returns it.

If type() is not Double or the value is not a whole number, the defaultValue will be returned.
*/
impl /*struct*/ QJsonValue {
  pub fn toInt_0<RetType, T: QJsonValue_toInt_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toInt_0(self);
    // return 1;
  }
}
pub trait QJsonValue_toInt_0<RetType> {
  fn toInt_0(self , rsthis: & QJsonValue) -> RetType;
}
impl<'a> /*trait*/ QJsonValue_toInt_0<i32> for (i32) {
  fn toInt_0(self , rsthis: & QJsonValue) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QJsonValue5toIntEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonvalue.h:132
// index:0
// Public Visibility=Default Availability=Available
// [8] double toDouble(double) const

/*
Converts the value to a double and returns it.

If type() is not Double, the defaultValue will be returned.
*/
impl /*struct*/ QJsonValue {
  pub fn toDouble_0<RetType, T: QJsonValue_toDouble_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toDouble_0(self);
    // return 1;
  }
}
pub trait QJsonValue_toDouble_0<RetType> {
  fn toDouble_0(self , rsthis: & QJsonValue) -> RetType;
}
impl<'a> /*trait*/ QJsonValue_toDouble_0<f64> for (f64) {
  fn toDouble_0(self , rsthis: & QJsonValue) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QJsonValue8toDoubleEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonvalue.h:133
// index:0
// Public Visibility=Default Availability=Available
// [8] QString toString() const

/*
Converts the value to a QString and returns it.

If type() is not String, a null QString will be returned.

See also QString::isNull().
*/
impl /*struct*/ QJsonValue {
  pub fn toString_0<RetType, T: QJsonValue_toString_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toString_0(self);
    // return 1;
  }
}
pub trait QJsonValue_toString_0<RetType> {
  fn toString_0(self , rsthis: & QJsonValue) -> RetType;
}
impl<'a> /*trait*/ QJsonValue_toString_0<usize> for () {
  fn toString_0(self , rsthis: & QJsonValue) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QJsonValue8toStringEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonvalue.h:134
// index:1
// Public Visibility=Default Availability=Available
// [8] QString toString(const QString &) const

/*
Converts the value to a QString and returns it.

If type() is not String, a null QString will be returned.

See also QString::isNull().
*/
impl /*struct*/ QJsonValue {
  pub fn toString_1<RetType, T: QJsonValue_toString_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toString_1(self);
    // return 1;
  }
}
pub trait QJsonValue_toString_1<RetType> {
  fn toString_1(self , rsthis: & QJsonValue) -> RetType;
}
impl<'a> /*trait*/ QJsonValue_toString_1<usize> for (usize) {
  fn toString_1(self , rsthis: & QJsonValue) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QJsonValue8toStringERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonvalue.h:135
// index:0
// Public Visibility=Default Availability=Available
// [16] QJsonArray toArray() const

/*
Converts the value to an array and returns it.

If type() is not Array, the defaultValue will be returned.
*/
impl /*struct*/ QJsonValue {
  pub fn toArray_0<RetType, T: QJsonValue_toArray_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toArray_0(self);
    // return 1;
  }
}
pub trait QJsonValue_toArray_0<RetType> {
  fn toArray_0(self , rsthis: & QJsonValue) -> RetType;
}
impl<'a> /*trait*/ QJsonValue_toArray_0<usize> for () {
  fn toArray_0(self , rsthis: & QJsonValue) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QJsonValue7toArrayEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonvalue.h:136
// index:1
// Public Visibility=Default Availability=Available
// [16] QJsonArray toArray(const QJsonArray &) const

/*
Converts the value to an array and returns it.

If type() is not Array, the defaultValue will be returned.
*/
impl /*struct*/ QJsonValue {
  pub fn toArray_1<RetType, T: QJsonValue_toArray_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toArray_1(self);
    // return 1;
  }
}
pub trait QJsonValue_toArray_1<RetType> {
  fn toArray_1(self , rsthis: & QJsonValue) -> RetType;
}
impl<'a> /*trait*/ QJsonValue_toArray_1<usize> for (usize) {
  fn toArray_1(self , rsthis: & QJsonValue) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QJsonValue7toArrayERK10QJsonArray", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonvalue.h:137
// index:0
// Public Visibility=Default Availability=Available
// [16] QJsonObject toObject() const

/*
Converts the value to an object and returns it.

If type() is not Object, the defaultValue will be returned.
*/
impl /*struct*/ QJsonValue {
  pub fn toObject_0<RetType, T: QJsonValue_toObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toObject_0(self);
    // return 1;
  }
}
pub trait QJsonValue_toObject_0<RetType> {
  fn toObject_0(self , rsthis: & QJsonValue) -> RetType;
}
impl<'a> /*trait*/ QJsonValue_toObject_0<usize> for () {
  fn toObject_0(self , rsthis: & QJsonValue) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QJsonValue8toObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonvalue.h:138
// index:1
// Public Visibility=Default Availability=Available
// [16] QJsonObject toObject(const QJsonObject &) const

/*
Converts the value to an object and returns it.

If type() is not Object, the defaultValue will be returned.
*/
impl /*struct*/ QJsonValue {
  pub fn toObject_1<RetType, T: QJsonValue_toObject_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toObject_1(self);
    // return 1;
  }
}
pub trait QJsonValue_toObject_1<RetType> {
  fn toObject_1(self , rsthis: & QJsonValue) -> RetType;
}
impl<'a> /*trait*/ QJsonValue_toObject_1<usize> for (usize) {
  fn toObject_1(self , rsthis: & QJsonValue) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QJsonValue8toObjectERK11QJsonObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonvalue.h:140
// index:0
// Public Visibility=Default Availability=Available
// [24] const QJsonValue operator[](const QString &) const

/*

*/
impl /*struct*/ QJsonValue {
  pub fn operator_get_index_0<RetType, T: QJsonValue_operator_get_index_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_get_index_0(self);
    // return 1;
  }
}
pub trait QJsonValue_operator_get_index_0<RetType> {
  fn operator_get_index_0(self , rsthis: & QJsonValue) -> RetType;
}
impl<'a> /*trait*/ QJsonValue_operator_get_index_0<usize> for (usize) {
  fn operator_get_index_0(self , rsthis: & QJsonValue) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QJsonValueixERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonvalue.h:141
// index:1
// Public Visibility=Default Availability=Available
// [24] const QJsonValue operator[](QLatin1String) const

/*

*/
impl /*struct*/ QJsonValue {
  pub fn operator_get_index_1<RetType, T: QJsonValue_operator_get_index_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_get_index_1(self);
    // return 1;
  }
}
pub trait QJsonValue_operator_get_index_1<RetType> {
  fn operator_get_index_1(self , rsthis: & QJsonValue) -> RetType;
}
impl<'a> /*trait*/ QJsonValue_operator_get_index_1<usize> for (usize) {
  fn operator_get_index_1(self , rsthis: & QJsonValue) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QJsonValueixE13QLatin1String", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonvalue.h:142
// index:2
// Public Visibility=Default Availability=Available
// [24] const QJsonValue operator[](int) const

/*

*/
impl /*struct*/ QJsonValue {
  pub fn operator_get_index_2<RetType, T: QJsonValue_operator_get_index_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_get_index_2(self);
    // return 1;
  }
}
pub trait QJsonValue_operator_get_index_2<RetType> {
  fn operator_get_index_2(self , rsthis: & QJsonValue) -> RetType;
}
impl<'a> /*trait*/ QJsonValue_operator_get_index_2<usize> for (i32) {
  fn operator_get_index_2(self , rsthis: & QJsonValue) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QJsonValueixEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonvalue.h:144
// index:0
// Public Visibility=Default Availability=Available
// [1] bool operator==(const QJsonValue &) const

/*

*/
impl /*struct*/ QJsonValue {
  pub fn operator_equal_equal_0<RetType, T: QJsonValue_operator_equal_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_equal_0(self);
    // return 1;
  }
}
pub trait QJsonValue_operator_equal_equal_0<RetType> {
  fn operator_equal_equal_0(self , rsthis: & QJsonValue) -> RetType;
}
impl<'a> /*trait*/ QJsonValue_operator_equal_equal_0<bool> for (usize) {
  fn operator_equal_equal_0(self , rsthis: & QJsonValue) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QJsonValueeqERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonvalue.h:145
// index:0
// Public Visibility=Default Availability=Available
// [1] bool operator!=(const QJsonValue &) const

/*

*/
impl /*struct*/ QJsonValue {
  pub fn operator_not_equal_0<RetType, T: QJsonValue_operator_not_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_not_equal_0(self);
    // return 1;
  }
}
pub trait QJsonValue_operator_not_equal_0<RetType> {
  fn operator_not_equal_0(self , rsthis: & QJsonValue) -> RetType;
}
impl<'a> /*trait*/ QJsonValue_operator_not_equal_0<bool> for (usize) {
  fn operator_not_equal_0(self , rsthis: & QJsonValue) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QJsonValueneERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}


/*
This enum describes the type of the JSON value.


*/
pub type QJsonValue__Type = i32;
// 
pub const QJsonValue__Null :QJsonValue__Type = 0;
// 
pub const QJsonValue__Bool :QJsonValue__Type = 1;
// 
pub const QJsonValue__Double :QJsonValue__Type = 2;
// 
pub const QJsonValue__String :QJsonValue__Type = 3;
// 
pub const QJsonValue__Array :QJsonValue__Type = 4;
// 
pub const QJsonValue__Object :QJsonValue__Type = 5;
// 
pub const QJsonValue__Undefined :QJsonValue__Type = 128;
pub fn QJsonValue_TypeItemName(val: i32) ->String {
  match val {
     QJsonValue__Null => // 0
     {return String::from("Null");}
     QJsonValue__Bool => // 1
     {return String::from("Bool");}
     QJsonValue__Double => // 2
     {return String::from("Double");}
     QJsonValue__String => // 3
     {return String::from("String");}
     QJsonValue__Array => // 4
     {return String::from("Array");}
     QJsonValue__Object => // 5
     {return String::from("Object");}
     QJsonValue__Undefined => // 128
     {return String::from("Undefined");}
  _ => {return format!("{}", val);}
}
}
pub fn QJsonValue_TypeItemName_s(val: i32) ->String {
  //var nilthis *QJsonValue
  //return nilthis.TypeItemName(val);
  return QJsonValue_TypeItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
