

// mod ::core::QJsonDocument
// package qtcore
// /usr/include/qt/QtCore/qjsondocument.h
// #include <qjsondocument.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 1
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
#[derive(Default)] // class sizeof(QJsonDocument)=8
pub struct QJsonDocument {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QJsonDocument_ITF interface {
//    QJsonDocument_PTR() *QJsonDocument
//}
//func (ptr *QJsonDocument) QJsonDocument_PTR() *QJsonDocument { return ptr }

impl /*struct*/ QJsonDocument {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QJsonDocument {
    return QJsonDocument{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QJsonDocument {
//  type Target = QJsonDocumentBASE;
//
//  fn deref(&self) -> &QJsonDocumentBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QJsonDocumentBASE> for QJsonDocument {
//  fn as_ref(& self) -> & QJsonDocumentBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qjsondocument.h:88
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QJsonDocument()

/*
Constructs an empty and invalid document.
*/
// QJsonDocument() ctx.fn_proto_cpp
impl /*struct*/ QJsonDocument {
  pub fn QJsonDocument_0<T: QJsonDocument_QJsonDocument_0>(value: T) -> QJsonDocument {
    let rsthis = value.QJsonDocument_0();
    return rsthis;
    // return 1;
  }
}

pub trait QJsonDocument_QJsonDocument_0 {
  fn QJsonDocument_0(self) -> QJsonDocument;
}
// QJsonDocument() ctx.fn_proto_cpp
impl<'a> /*trait*/ QJsonDocument_QJsonDocument_0 for () {
  fn QJsonDocument_0(self) -> QJsonDocument {
    // unsafe{_ZN13QJsonDocumentC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN13QJsonDocumentC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QJsonDocument{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qjsondocument.h:89
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QJsonDocument(const QJsonObject &)

/*
Constructs an empty and invalid document.
*/
// QJsonDocument(const QJsonObject &) ctx.fn_proto_cpp
impl /*struct*/ QJsonDocument {
  pub fn QJsonDocument_1<T: QJsonDocument_QJsonDocument_1>(value: T) -> QJsonDocument {
    let rsthis = value.QJsonDocument_1();
    return rsthis;
    // return 1;
  }
}

pub trait QJsonDocument_QJsonDocument_1 {
  fn QJsonDocument_1(self) -> QJsonDocument;
}
// QJsonDocument(const QJsonObject &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QJsonDocument_QJsonDocument_1 for (usize) {
  fn QJsonDocument_1(self) -> QJsonDocument {
    // unsafe{_ZN13QJsonDocumentC2ERK11QJsonObject()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN13QJsonDocumentC2ERK11QJsonObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QJsonDocument{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qjsondocument.h:90
// index:2
// Public Visibility=Default Availability=Available
// [-2] void QJsonDocument(const QJsonArray &)

/*
Constructs an empty and invalid document.
*/
// QJsonDocument(const QJsonArray &) ctx.fn_proto_cpp
impl /*struct*/ QJsonDocument {
  pub fn QJsonDocument_2<T: QJsonDocument_QJsonDocument_2>(value: T) -> QJsonDocument {
    let rsthis = value.QJsonDocument_2();
    return rsthis;
    // return 1;
  }
}

pub trait QJsonDocument_QJsonDocument_2 {
  fn QJsonDocument_2(self) -> QJsonDocument;
}
// QJsonDocument(const QJsonArray &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QJsonDocument_QJsonDocument_2 for (usize) {
  fn QJsonDocument_2(self) -> QJsonDocument {
    // unsafe{_ZN13QJsonDocumentC2ERK10QJsonArray()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN13QJsonDocumentC2ERK10QJsonArray", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QJsonDocument{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qjsondocument.h:91
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ~QJsonDocument()

/*

*/
pub fn DeleteQJsonDocument(this :*mut QJsonDocument) {
    // let rv = qtrt::InvokeQtFunc6("_ZN13QJsonDocumentD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 8)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qjsondocument.h:94
// index:0
// Public Visibility=Default Availability=Available
// [8] QJsonDocument & operator=(const QJsonDocument &)

/*

*/
impl /*struct*/ QJsonDocument {
  pub fn operator_equal_0<RetType, T: QJsonDocument_operator_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_0(self);
    // return 1;
  }
}
pub trait QJsonDocument_operator_equal_0<RetType> {
  fn operator_equal_0(self , rsthis: & QJsonDocument) -> RetType;
}
impl<'a> /*trait*/ QJsonDocument_operator_equal_0<usize> for (usize) {
  fn operator_equal_0(self , rsthis: & QJsonDocument) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN13QJsonDocumentaSERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsondocument.h:102
// index:1
// Public inline Visibility=Default Availability=Available
// [8] QJsonDocument & operator=(QJsonDocument &&)

/*

*/
impl /*struct*/ QJsonDocument {
  pub fn operator_equal_1<RetType, T: QJsonDocument_operator_equal_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_1(self);
    // return 1;
  }
}
pub trait QJsonDocument_operator_equal_1<RetType> {
  fn operator_equal_1(self , rsthis: & QJsonDocument) -> RetType;
}
impl<'a> /*trait*/ QJsonDocument_operator_equal_1<usize> for (usize) {
  fn operator_equal_1(self , rsthis: & QJsonDocument) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN13QJsonDocumentaSEOS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsondocument.h:108
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void swap(QJsonDocument &)

/*
Swaps the document other with this. This operation is very fast and never fails.

This function was introduced in  Qt 5.10.
*/
impl /*struct*/ QJsonDocument {
  pub fn swap_0<RetType, T: QJsonDocument_swap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.swap_0(self);
    // return 1;
  }
}
pub trait QJsonDocument_swap_0<RetType> {
  fn swap_0(self , rsthis: & QJsonDocument) -> RetType;
}
impl<'a> /*trait*/ QJsonDocument_swap_0<(/*void*/)> for (usize) {
  fn swap_0(self , rsthis: & QJsonDocument) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QJsonDocument4swapERS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qjsondocument.h:118
// index:0
// Public static Visibility=Default Availability=Available
// [8] QJsonDocument fromRawData(const char *, int, QJsonDocument::DataValidation)

/*
Creates a QJsonDocument that uses the first size bytes from data. It assumes data contains a binary encoded JSON document. The created document does not take ownership of data and the caller has to guarantee that data will not be deleted or modified as long as any QJsonDocument, QJsonObject or QJsonArray still references the data.

data has to be aligned to a 4 byte boundary.

validation decides whether the data is checked for validity before being used. By default the data is validated. If the data is not valid, the method returns a null document.

Returns a QJsonDocument representing the data.

See also rawData(), fromBinaryData(), isNull(), and DataValidation.
*/
impl /*struct*/ QJsonDocument {
  pub fn fromRawData_0<RetType, T: QJsonDocument_fromRawData_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromRawData_0();
    // return 1;
  }
}
pub trait QJsonDocument_fromRawData_0<RetType> {
  fn fromRawData_0(self ) -> RetType;
}
impl<'a> /*trait*/ QJsonDocument_fromRawData_0<usize> for (usize,i32,i32) {
  fn fromRawData_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN13QJsonDocument11fromRawDataEPKciNS_14DataValidationE", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsondocument.h:119
// index:0
// Public Visibility=Default Availability=Available
// [8] const char * rawData(int *) const

/*
Returns the raw binary representation of the data size will contain the size of the returned data.

This method is useful to e.g. stream the JSON document in it's binary form to a file.
*/
impl /*struct*/ QJsonDocument {
  pub fn rawData_0<RetType, T: QJsonDocument_rawData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rawData_0(self);
    // return 1;
  }
}
pub trait QJsonDocument_rawData_0<RetType> {
  fn rawData_0(self , rsthis: & QJsonDocument) -> RetType;
}
impl<'a> /*trait*/ QJsonDocument_rawData_0<usize> for (usize) {
  fn rawData_0(self , rsthis: & QJsonDocument) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QJsonDocument7rawDataEPi", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsondocument.h:121
// index:0
// Public static Visibility=Default Availability=Available
// [8] QJsonDocument fromBinaryData(const QByteArray &, QJsonDocument::DataValidation)

/*
Creates a QJsonDocument from data.

validation decides whether the data is checked for validity before being used. By default the data is validated. If the data is not valid, the method returns a null document.

See also toBinaryData(), fromRawData(), isNull(), and DataValidation.
*/
impl /*struct*/ QJsonDocument {
  pub fn fromBinaryData_0<RetType, T: QJsonDocument_fromBinaryData_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromBinaryData_0();
    // return 1;
  }
}
pub trait QJsonDocument_fromBinaryData_0<RetType> {
  fn fromBinaryData_0(self ) -> RetType;
}
impl<'a> /*trait*/ QJsonDocument_fromBinaryData_0<usize> for (usize,i32) {
  fn fromBinaryData_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN13QJsonDocument14fromBinaryDataERK10QByteArrayNS_14DataValidationE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsondocument.h:122
// index:0
// Public Visibility=Default Availability=Available
// [8] QByteArray toBinaryData() const

/*
Returns a binary representation of the document.

The binary representation is also the native format used internally in Qt, and is very efficient and fast to convert to and from.

The binary format can be stored on disk and interchanged with other applications or computers. fromBinaryData() can be used to convert it back into a JSON document.

See also fromBinaryData().
*/
impl /*struct*/ QJsonDocument {
  pub fn toBinaryData_0<RetType, T: QJsonDocument_toBinaryData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toBinaryData_0(self);
    // return 1;
  }
}
pub trait QJsonDocument_toBinaryData_0<RetType> {
  fn toBinaryData_0(self , rsthis: & QJsonDocument) -> RetType;
}
impl<'a> /*trait*/ QJsonDocument_toBinaryData_0<usize> for () {
  fn toBinaryData_0(self , rsthis: & QJsonDocument) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QJsonDocument12toBinaryDataEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsondocument.h:124
// index:0
// Public static Visibility=Default Availability=Available
// [8] QJsonDocument fromVariant(const QVariant &)

/*
Creates a QJsonDocument from the QVariant variant.

If the variant contains any other type than a QVariantMap, QVariantHash, QVariantList or QStringList, the returned document is invalid.

See also toVariant().
*/
impl /*struct*/ QJsonDocument {
  pub fn fromVariant_0<RetType, T: QJsonDocument_fromVariant_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromVariant_0();
    // return 1;
  }
}
pub trait QJsonDocument_fromVariant_0<RetType> {
  fn fromVariant_0(self ) -> RetType;
}
impl<'a> /*trait*/ QJsonDocument_fromVariant_0<usize> for (usize) {
  fn fromVariant_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN13QJsonDocument11fromVariantERK8QVariant", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsondocument.h:125
// index:0
// Public Visibility=Default Availability=Available
// [16] QVariant toVariant() const

/*
Returns a QVariant representing the Json document.

The returned variant will be a QVariantList if the document is a QJsonArray and a QVariantMap if the document is a QJsonObject.

See also fromVariant() and QJsonValue::toVariant().
*/
impl /*struct*/ QJsonDocument {
  pub fn toVariant_0<RetType, T: QJsonDocument_toVariant_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toVariant_0(self);
    // return 1;
  }
}
pub trait QJsonDocument_toVariant_0<RetType> {
  fn toVariant_0(self , rsthis: & QJsonDocument) -> RetType;
}
impl<'a> /*trait*/ QJsonDocument_toVariant_0<usize> for () {
  fn toVariant_0(self , rsthis: & QJsonDocument) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QJsonDocument9toVariantEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsondocument.h:137
// index:0
// Public Visibility=Default Availability=Available
// [8] QByteArray toJson() const

/*
Converts the QJsonDocument to a UTF-8 encoded JSON document in the provided format.

See also fromJson() and JsonFormat.
*/
impl /*struct*/ QJsonDocument {
  pub fn toJson_0<RetType, T: QJsonDocument_toJson_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toJson_0(self);
    // return 1;
  }
}
pub trait QJsonDocument_toJson_0<RetType> {
  fn toJson_0(self , rsthis: & QJsonDocument) -> RetType;
}
impl<'a> /*trait*/ QJsonDocument_toJson_0<usize> for () {
  fn toJson_0(self , rsthis: & QJsonDocument) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QJsonDocument6toJsonEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsondocument.h:138
// index:1
// Public Visibility=Default Availability=Available
// [8] QByteArray toJson(QJsonDocument::JsonFormat) const

/*
Converts the QJsonDocument to a UTF-8 encoded JSON document in the provided format.

See also fromJson() and JsonFormat.
*/
impl /*struct*/ QJsonDocument {
  pub fn toJson_1<RetType, T: QJsonDocument_toJson_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toJson_1(self);
    // return 1;
  }
}
pub trait QJsonDocument_toJson_1<RetType> {
  fn toJson_1(self , rsthis: & QJsonDocument) -> RetType;
}
impl<'a> /*trait*/ QJsonDocument_toJson_1<usize> for (i32) {
  fn toJson_1(self , rsthis: & QJsonDocument) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QJsonDocument6toJsonENS_10JsonFormatE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsondocument.h:141
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isEmpty() const

/*
Returns true if the document doesn't contain any data.
*/
impl /*struct*/ QJsonDocument {
  pub fn isEmpty_0<RetType, T: QJsonDocument_isEmpty_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isEmpty_0(self);
    // return 1;
  }
}
pub trait QJsonDocument_isEmpty_0<RetType> {
  fn isEmpty_0(self , rsthis: & QJsonDocument) -> RetType;
}
impl<'a> /*trait*/ QJsonDocument_isEmpty_0<bool> for () {
  fn isEmpty_0(self , rsthis: & QJsonDocument) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QJsonDocument7isEmptyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsondocument.h:142
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isArray() const

/*
Returns true if the document contains an array.

See also array() and isObject().
*/
impl /*struct*/ QJsonDocument {
  pub fn isArray_0<RetType, T: QJsonDocument_isArray_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isArray_0(self);
    // return 1;
  }
}
pub trait QJsonDocument_isArray_0<RetType> {
  fn isArray_0(self , rsthis: & QJsonDocument) -> RetType;
}
impl<'a> /*trait*/ QJsonDocument_isArray_0<bool> for () {
  fn isArray_0(self , rsthis: & QJsonDocument) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QJsonDocument7isArrayEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsondocument.h:143
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isObject() const

/*
Returns true if the document contains an object.

See also object() and isArray().
*/
impl /*struct*/ QJsonDocument {
  pub fn isObject_0<RetType, T: QJsonDocument_isObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isObject_0(self);
    // return 1;
  }
}
pub trait QJsonDocument_isObject_0<RetType> {
  fn isObject_0(self , rsthis: & QJsonDocument) -> RetType;
}
impl<'a> /*trait*/ QJsonDocument_isObject_0<bool> for () {
  fn isObject_0(self , rsthis: & QJsonDocument) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QJsonDocument8isObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsondocument.h:145
// index:0
// Public Visibility=Default Availability=Available
// [16] QJsonObject object() const

/*
Returns the QJsonObject contained in the document.

Returns an empty object if the document contains an array.

See also isObject(), array(), and setObject().
*/
impl /*struct*/ QJsonDocument {
  pub fn object_0<RetType, T: QJsonDocument_object_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.object_0(self);
    // return 1;
  }
}
pub trait QJsonDocument_object_0<RetType> {
  fn object_0(self , rsthis: & QJsonDocument) -> RetType;
}
impl<'a> /*trait*/ QJsonDocument_object_0<usize> for () {
  fn object_0(self , rsthis: & QJsonDocument) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QJsonDocument6objectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsondocument.h:146
// index:0
// Public Visibility=Default Availability=Available
// [16] QJsonArray array() const

/*
Returns the QJsonArray contained in the document.

Returns an empty array if the document contains an object.

See also isArray(), object(), and setArray().
*/
impl /*struct*/ QJsonDocument {
  pub fn array_0<RetType, T: QJsonDocument_array_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.array_0(self);
    // return 1;
  }
}
pub trait QJsonDocument_array_0<RetType> {
  fn array_0(self , rsthis: & QJsonDocument) -> RetType;
}
impl<'a> /*trait*/ QJsonDocument_array_0<usize> for () {
  fn array_0(self , rsthis: & QJsonDocument) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QJsonDocument5arrayEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsondocument.h:148
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setObject(const QJsonObject &)

/*
Sets object as the main object of this document.

See also setArray() and object().
*/
impl /*struct*/ QJsonDocument {
  pub fn setObject_0<RetType, T: QJsonDocument_setObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setObject_0(self);
    // return 1;
  }
}
pub trait QJsonDocument_setObject_0<RetType> {
  fn setObject_0(self , rsthis: & QJsonDocument) -> RetType;
}
impl<'a> /*trait*/ QJsonDocument_setObject_0<(/*void*/)> for (usize) {
  fn setObject_0(self , rsthis: & QJsonDocument) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QJsonDocument9setObjectERK11QJsonObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qjsondocument.h:149
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setArray(const QJsonArray &)

/*
Sets array as the main object of this document.

See also setObject() and array().
*/
impl /*struct*/ QJsonDocument {
  pub fn setArray_0<RetType, T: QJsonDocument_setArray_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setArray_0(self);
    // return 1;
  }
}
pub trait QJsonDocument_setArray_0<RetType> {
  fn setArray_0(self , rsthis: & QJsonDocument) -> RetType;
}
impl<'a> /*trait*/ QJsonDocument_setArray_0<(/*void*/)> for (usize) {
  fn setArray_0(self , rsthis: & QJsonDocument) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QJsonDocument8setArrayERK10QJsonArray", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qjsondocument.h:151
// index:0
// Public Visibility=Default Availability=Available
// [24] const QJsonValue operator[](const QString &) const

/*

*/
impl /*struct*/ QJsonDocument {
  pub fn operator_get_index_0<RetType, T: QJsonDocument_operator_get_index_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_get_index_0(self);
    // return 1;
  }
}
pub trait QJsonDocument_operator_get_index_0<RetType> {
  fn operator_get_index_0(self , rsthis: & QJsonDocument) -> RetType;
}
impl<'a> /*trait*/ QJsonDocument_operator_get_index_0<usize> for (usize) {
  fn operator_get_index_0(self , rsthis: & QJsonDocument) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QJsonDocumentixERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsondocument.h:152
// index:1
// Public Visibility=Default Availability=Available
// [24] const QJsonValue operator[](QLatin1String) const

/*

*/
impl /*struct*/ QJsonDocument {
  pub fn operator_get_index_1<RetType, T: QJsonDocument_operator_get_index_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_get_index_1(self);
    // return 1;
  }
}
pub trait QJsonDocument_operator_get_index_1<RetType> {
  fn operator_get_index_1(self , rsthis: & QJsonDocument) -> RetType;
}
impl<'a> /*trait*/ QJsonDocument_operator_get_index_1<usize> for (usize) {
  fn operator_get_index_1(self , rsthis: & QJsonDocument) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QJsonDocumentixE13QLatin1String", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsondocument.h:153
// index:2
// Public Visibility=Default Availability=Available
// [24] const QJsonValue operator[](int) const

/*

*/
impl /*struct*/ QJsonDocument {
  pub fn operator_get_index_2<RetType, T: QJsonDocument_operator_get_index_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_get_index_2(self);
    // return 1;
  }
}
pub trait QJsonDocument_operator_get_index_2<RetType> {
  fn operator_get_index_2(self , rsthis: & QJsonDocument) -> RetType;
}
impl<'a> /*trait*/ QJsonDocument_operator_get_index_2<usize> for (i32) {
  fn operator_get_index_2(self , rsthis: & QJsonDocument) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QJsonDocumentixEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsondocument.h:155
// index:0
// Public Visibility=Default Availability=Available
// [1] bool operator==(const QJsonDocument &) const

/*

*/
impl /*struct*/ QJsonDocument {
  pub fn operator_equal_equal_0<RetType, T: QJsonDocument_operator_equal_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_equal_0(self);
    // return 1;
  }
}
pub trait QJsonDocument_operator_equal_equal_0<RetType> {
  fn operator_equal_equal_0(self , rsthis: & QJsonDocument) -> RetType;
}
impl<'a> /*trait*/ QJsonDocument_operator_equal_equal_0<bool> for (usize) {
  fn operator_equal_equal_0(self , rsthis: & QJsonDocument) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QJsonDocumenteqERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsondocument.h:156
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator!=(const QJsonDocument &) const

/*

*/
impl /*struct*/ QJsonDocument {
  pub fn operator_not_equal_0<RetType, T: QJsonDocument_operator_not_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_not_equal_0(self);
    // return 1;
  }
}
pub trait QJsonDocument_operator_not_equal_0<RetType> {
  fn operator_not_equal_0(self , rsthis: & QJsonDocument) -> RetType;
}
impl<'a> /*trait*/ QJsonDocument_operator_not_equal_0<bool> for (usize) {
  fn operator_not_equal_0(self , rsthis: & QJsonDocument) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QJsonDocumentneERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsondocument.h:158
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isNull() const

/*
returns true if this document is null.

Null documents are documents created through the default constructor.

Documents created from UTF-8 encoded text or the binary format are validated during parsing. If validation fails, the returned document will also be null.
*/
impl /*struct*/ QJsonDocument {
  pub fn isNull_0<RetType, T: QJsonDocument_isNull_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isNull_0(self);
    // return 1;
  }
}
pub trait QJsonDocument_isNull_0<RetType> {
  fn isNull_0(self , rsthis: & QJsonDocument) -> RetType;
}
impl<'a> /*trait*/ QJsonDocument_isNull_0<bool> for () {
  fn isNull_0(self , rsthis: & QJsonDocument) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QJsonDocument6isNullEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}


/*
This value is used to tell QJsonDocument whether to validate the binary data when converting to a QJsonDocument using fromBinaryData() or fromRawData().


*/
pub type QJsonDocument__DataValidation = i32;
// Validate the data before using it. This is the default.
pub const QJsonDocument__Validate :QJsonDocument__DataValidation = 0;
// Bypasses data validation. Only use if you received the data from a trusted place and know it's valid, as using of invalid data can crash the application.
pub const QJsonDocument__BypassValidation :QJsonDocument__DataValidation = 1;
pub fn QJsonDocument_DataValidationItemName(val: i32) ->String {
  match val {
     QJsonDocument__Validate => // 0
     {return String::from("Validate");}
     QJsonDocument__BypassValidation => // 1
     {return String::from("BypassValidation");}
  _ => {return format!("{}", val);}
}
}
pub fn QJsonDocument_DataValidationItemName_s(val: i32) ->String {
  //var nilthis *QJsonDocument
  //return nilthis.DataValidationItemName(val);
  return QJsonDocument_DataValidationItemName(val);
}


/*
This value defines the format of the JSON byte array produced when converting to a QJsonDocument using toJson().

  {
      "Array": [
          true,
          999,
          "string"
      ],
      "Key": "Value",
      "null": null
  }



  {"Array":[true,999,"string"],"Key":"Value","null":null}

*/
pub type QJsonDocument__JsonFormat = i32;
// Defines human readable output as follows:
pub const QJsonDocument__Indented :QJsonDocument__JsonFormat = 0;
// Defines a compact output as follows:
pub const QJsonDocument__Compact :QJsonDocument__JsonFormat = 1;
pub fn QJsonDocument_JsonFormatItemName(val: i32) ->String {
  match val {
     QJsonDocument__Indented => // 0
     {return String::from("Indented");}
     QJsonDocument__Compact => // 1
     {return String::from("Compact");}
  _ => {return format!("{}", val);}
}
}
pub fn QJsonDocument_JsonFormatItemName_s(val: i32) ->String {
  //var nilthis *QJsonDocument
  //return nilthis.JsonFormatItemName(val);
  return QJsonDocument_JsonFormatItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
