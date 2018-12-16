

// mod ::core::QJsonObject
// package qtcore
// /usr/include/qt/QtCore/qjsonobject.h
// #include <qjsonobject.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 28
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
#[derive(Default)] // class sizeof(QJsonObject)=16
pub struct QJsonObject {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QJsonObject_ITF interface {
//    QJsonObject_PTR() *QJsonObject
//}
//func (ptr *QJsonObject) QJsonObject_PTR() *QJsonObject { return ptr }

impl /*struct*/ QJsonObject {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QJsonObject {
    return QJsonObject{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QJsonObject {
//  type Target = QJsonObjectBASE;
//
//  fn deref(&self) -> &QJsonObjectBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QJsonObjectBASE> for QJsonObject {
//  fn as_ref(& self) -> & QJsonObjectBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qjsonobject.h:61
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QJsonObject()

/*
Constructs an empty JSON object.

See also isEmpty().
*/
// QJsonObject() ctx.fn_proto_cpp
impl /*struct*/ QJsonObject {
  pub fn QJsonObject_0<T: QJsonObject_QJsonObject_0>(value: T) -> QJsonObject {
    let rsthis = value.QJsonObject_0();
    return rsthis;
    // return 1;
  }
}

pub trait QJsonObject_QJsonObject_0 {
  fn QJsonObject_0(self) -> QJsonObject;
}
// QJsonObject() ctx.fn_proto_cpp
impl<'a> /*trait*/ QJsonObject_QJsonObject_0 for () {
  fn QJsonObject_0(self) -> QJsonObject {
    // unsafe{_ZN11QJsonObjectC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QJsonObjectC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QJsonObject{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qjsonobject.h:72
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ~QJsonObject()

/*

*/
pub fn DeleteQJsonObject(this :*mut QJsonObject) {
    // let rv = qtrt::InvokeQtFunc6("_ZN11QJsonObjectD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qjsonobject.h:75
// index:0
// Public Visibility=Default Availability=Available
// [16] QJsonObject & operator=(const QJsonObject &)

/*

*/
impl /*struct*/ QJsonObject {
  pub fn operator_equal_0<RetType, T: QJsonObject_operator_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_0(self);
    // return 1;
  }
}
pub trait QJsonObject_operator_equal_0<RetType> {
  fn operator_equal_0(self , rsthis: & QJsonObject) -> RetType;
}
impl<'a> /*trait*/ QJsonObject_operator_equal_0<usize> for (usize) {
  fn operator_equal_0(self , rsthis: & QJsonObject) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QJsonObjectaSERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonobject.h:84
// index:1
// Public inline Visibility=Default Availability=Available
// [16] QJsonObject & operator=(QJsonObject &&)

/*

*/
impl /*struct*/ QJsonObject {
  pub fn operator_equal_1<RetType, T: QJsonObject_operator_equal_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_1(self);
    // return 1;
  }
}
pub trait QJsonObject_operator_equal_1<RetType> {
  fn operator_equal_1(self , rsthis: & QJsonObject) -> RetType;
}
impl<'a> /*trait*/ QJsonObject_operator_equal_1<usize> for (usize) {
  fn operator_equal_1(self , rsthis: & QJsonObject) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QJsonObjectaSEOS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonobject.h:90
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void swap(QJsonObject &)

/*
Swaps the object other with this. This operation is very fast and never fails.

This function was introduced in  Qt 5.10.
*/
impl /*struct*/ QJsonObject {
  pub fn swap_0<RetType, T: QJsonObject_swap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.swap_0(self);
    // return 1;
  }
}
pub trait QJsonObject_swap_0<RetType> {
  fn swap_0(self , rsthis: & QJsonObject) -> RetType;
}
impl<'a> /*trait*/ QJsonObject_swap_0<(/*void*/)> for (usize) {
  fn swap_0(self , rsthis: & QJsonObject) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QJsonObject4swapERS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qjsonobject.h:101
// index:0
// Public Visibility=Default Availability=Available
// [8] QStringList keys() const

/*
Returns a list of all keys in this object.

The list is sorted lexographically.
*/
impl /*struct*/ QJsonObject {
  pub fn keys_0<RetType, T: QJsonObject_keys_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.keys_0(self);
    // return 1;
  }
}
pub trait QJsonObject_keys_0<RetType> {
  fn keys_0(self , rsthis: & QJsonObject) -> RetType;
}
impl<'a> /*trait*/ QJsonObject_keys_0<usize> for () {
  fn keys_0(self , rsthis: & QJsonObject) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QJsonObject4keysEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonobject.h:102
// index:0
// Public Visibility=Default Availability=Available
// [4] int size() const

/*
Returns the number of (key, value) pairs stored in the object.
*/
impl /*struct*/ QJsonObject {
  pub fn size_0<RetType, T: QJsonObject_size_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.size_0(self);
    // return 1;
  }
}
pub trait QJsonObject_size_0<RetType> {
  fn size_0(self , rsthis: & QJsonObject) -> RetType;
}
impl<'a> /*trait*/ QJsonObject_size_0<i32> for () {
  fn size_0(self , rsthis: & QJsonObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QJsonObject4sizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonobject.h:103
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int count() const

/*
This is an overloaded function.

Same as size().
*/
impl /*struct*/ QJsonObject {
  pub fn count_0<RetType, T: QJsonObject_count_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.count_0(self);
    // return 1;
  }
}
pub trait QJsonObject_count_0<RetType> {
  fn count_0(self , rsthis: & QJsonObject) -> RetType;
}
impl<'a> /*trait*/ QJsonObject_count_0<i32> for () {
  fn count_0(self , rsthis: & QJsonObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QJsonObject5countEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonobject.h:104
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int length() const

/*
This is an overloaded function.

Same as size().
*/
impl /*struct*/ QJsonObject {
  pub fn length_0<RetType, T: QJsonObject_length_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.length_0(self);
    // return 1;
  }
}
pub trait QJsonObject_length_0<RetType> {
  fn length_0(self , rsthis: & QJsonObject) -> RetType;
}
impl<'a> /*trait*/ QJsonObject_length_0<i32> for () {
  fn length_0(self , rsthis: & QJsonObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QJsonObject6lengthEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonobject.h:105
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isEmpty() const

/*
Returns true if the object is empty. This is the same as size() == 0.

See also size().
*/
impl /*struct*/ QJsonObject {
  pub fn isEmpty_0<RetType, T: QJsonObject_isEmpty_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isEmpty_0(self);
    // return 1;
  }
}
pub trait QJsonObject_isEmpty_0<RetType> {
  fn isEmpty_0(self , rsthis: & QJsonObject) -> RetType;
}
impl<'a> /*trait*/ QJsonObject_isEmpty_0<bool> for () {
  fn isEmpty_0(self , rsthis: & QJsonObject) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QJsonObject7isEmptyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonobject.h:107
// index:0
// Public Visibility=Default Availability=Available
// [24] QJsonValue value(const QString &) const

/*
Returns a QJsonValue representing the value for the key key.

The returned QJsonValue is QJsonValue::Undefined if the key does not exist.

See also QJsonValue and QJsonValue::isUndefined().
*/
impl /*struct*/ QJsonObject {
  pub fn value_0<RetType, T: QJsonObject_value_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.value_0(self);
    // return 1;
  }
}
pub trait QJsonObject_value_0<RetType> {
  fn value_0(self , rsthis: & QJsonObject) -> RetType;
}
impl<'a> /*trait*/ QJsonObject_value_0<usize> for (usize) {
  fn value_0(self , rsthis: & QJsonObject) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QJsonObject5valueERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonobject.h:108
// index:1
// Public Visibility=Default Availability=Available
// [24] QJsonValue value(QLatin1String) const

/*
Returns a QJsonValue representing the value for the key key.

The returned QJsonValue is QJsonValue::Undefined if the key does not exist.

See also QJsonValue and QJsonValue::isUndefined().
*/
impl /*struct*/ QJsonObject {
  pub fn value_1<RetType, T: QJsonObject_value_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.value_1(self);
    // return 1;
  }
}
pub trait QJsonObject_value_1<RetType> {
  fn value_1(self , rsthis: & QJsonObject) -> RetType;
}
impl<'a> /*trait*/ QJsonObject_value_1<usize> for (usize) {
  fn value_1(self , rsthis: & QJsonObject) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QJsonObject5valueE13QLatin1String", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonobject.h:109
// index:0
// Public Visibility=Default Availability=Available
// [24] QJsonValue operator[](const QString &) const

/*

*/
impl /*struct*/ QJsonObject {
  pub fn operator_get_index_0<RetType, T: QJsonObject_operator_get_index_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_get_index_0(self);
    // return 1;
  }
}
pub trait QJsonObject_operator_get_index_0<RetType> {
  fn operator_get_index_0(self , rsthis: & QJsonObject) -> RetType;
}
impl<'a> /*trait*/ QJsonObject_operator_get_index_0<usize> for (usize) {
  fn operator_get_index_0(self , rsthis: & QJsonObject) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QJsonObjectixERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonobject.h:110
// index:1
// Public inline Visibility=Default Availability=Available
// [24] QJsonValue operator[](QLatin1String) const

/*

*/
impl /*struct*/ QJsonObject {
  pub fn operator_get_index_1<RetType, T: QJsonObject_operator_get_index_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_get_index_1(self);
    // return 1;
  }
}
pub trait QJsonObject_operator_get_index_1<RetType> {
  fn operator_get_index_1(self , rsthis: & QJsonObject) -> RetType;
}
impl<'a> /*trait*/ QJsonObject_operator_get_index_1<usize> for (usize) {
  fn operator_get_index_1(self , rsthis: & QJsonObject) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QJsonObjectixE13QLatin1String", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonobject.h:111
// index:2
// Public Visibility=Default Availability=Available
// [16] QJsonValueRef operator[](const QString &)

/*

*/
impl /*struct*/ QJsonObject {
  pub fn operator_get_index_2<RetType, T: QJsonObject_operator_get_index_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_get_index_2(self);
    // return 1;
  }
}
pub trait QJsonObject_operator_get_index_2<RetType> {
  fn operator_get_index_2(self , rsthis: & QJsonObject) -> RetType;
}
impl<'a> /*trait*/ QJsonObject_operator_get_index_2<usize> for (usize) {
  fn operator_get_index_2(self , rsthis: & QJsonObject) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QJsonObjectixERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonobject.h:112
// index:3
// Public Visibility=Default Availability=Available
// [16] QJsonValueRef operator[](QLatin1String)

/*

*/
impl /*struct*/ QJsonObject {
  pub fn operator_get_index_3<RetType, T: QJsonObject_operator_get_index_3<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_get_index_3(self);
    // return 1;
  }
}
pub trait QJsonObject_operator_get_index_3<RetType> {
  fn operator_get_index_3(self , rsthis: & QJsonObject) -> RetType;
}
impl<'a> /*trait*/ QJsonObject_operator_get_index_3<usize> for (usize) {
  fn operator_get_index_3(self , rsthis: & QJsonObject) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QJsonObjectixE13QLatin1String", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonobject.h:114
// index:0
// Public Visibility=Default Availability=Available
// [-2] void remove(const QString &)

/*
Removes key from the object.

See also insert() and take().
*/
impl /*struct*/ QJsonObject {
  pub fn remove_0<RetType, T: QJsonObject_remove_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.remove_0(self);
    // return 1;
  }
}
pub trait QJsonObject_remove_0<RetType> {
  fn remove_0(self , rsthis: & QJsonObject) -> RetType;
}
impl<'a> /*trait*/ QJsonObject_remove_0<(/*void*/)> for (usize) {
  fn remove_0(self , rsthis: & QJsonObject) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QJsonObject6removeERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qjsonobject.h:115
// index:0
// Public Visibility=Default Availability=Available
// [24] QJsonValue take(const QString &)

/*
Removes key from the object.

Returns a QJsonValue containing the value referenced by key. If key was not contained in the object, the returned QJsonValue is QJsonValue::Undefined.

See also insert(), remove(), and QJsonValue.
*/
impl /*struct*/ QJsonObject {
  pub fn take_0<RetType, T: QJsonObject_take_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.take_0(self);
    // return 1;
  }
}
pub trait QJsonObject_take_0<RetType> {
  fn take_0(self , rsthis: & QJsonObject) -> RetType;
}
impl<'a> /*trait*/ QJsonObject_take_0<usize> for (usize) {
  fn take_0(self , rsthis: & QJsonObject) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QJsonObject4takeERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonobject.h:116
// index:0
// Public Visibility=Default Availability=Available
// [1] bool contains(const QString &) const

/*
Returns true if the object contains key key.

See also insert(), remove(), and take().
*/
impl /*struct*/ QJsonObject {
  pub fn contains_0<RetType, T: QJsonObject_contains_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.contains_0(self);
    // return 1;
  }
}
pub trait QJsonObject_contains_0<RetType> {
  fn contains_0(self , rsthis: & QJsonObject) -> RetType;
}
impl<'a> /*trait*/ QJsonObject_contains_0<bool> for (usize) {
  fn contains_0(self , rsthis: & QJsonObject) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QJsonObject8containsERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonobject.h:117
// index:1
// Public Visibility=Default Availability=Available
// [1] bool contains(QLatin1String) const

/*
Returns true if the object contains key key.

See also insert(), remove(), and take().
*/
impl /*struct*/ QJsonObject {
  pub fn contains_1<RetType, T: QJsonObject_contains_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.contains_1(self);
    // return 1;
  }
}
pub trait QJsonObject_contains_1<RetType> {
  fn contains_1(self , rsthis: & QJsonObject) -> RetType;
}
impl<'a> /*trait*/ QJsonObject_contains_1<bool> for (usize) {
  fn contains_1(self , rsthis: & QJsonObject) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QJsonObject8containsE13QLatin1String", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonobject.h:119
// index:0
// Public Visibility=Default Availability=Available
// [1] bool operator==(const QJsonObject &) const

/*

*/
impl /*struct*/ QJsonObject {
  pub fn operator_equal_equal_0<RetType, T: QJsonObject_operator_equal_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_equal_0(self);
    // return 1;
  }
}
pub trait QJsonObject_operator_equal_equal_0<RetType> {
  fn operator_equal_equal_0(self , rsthis: & QJsonObject) -> RetType;
}
impl<'a> /*trait*/ QJsonObject_operator_equal_equal_0<bool> for (usize) {
  fn operator_equal_equal_0(self , rsthis: & QJsonObject) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QJsonObjecteqERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonobject.h:120
// index:0
// Public Visibility=Default Availability=Available
// [1] bool operator!=(const QJsonObject &) const

/*

*/
impl /*struct*/ QJsonObject {
  pub fn operator_not_equal_0<RetType, T: QJsonObject_operator_not_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_not_equal_0(self);
    // return 1;
  }
}
pub trait QJsonObject_operator_not_equal_0<RetType> {
  fn operator_not_equal_0(self , rsthis: & QJsonObject) -> RetType;
}
impl<'a> /*trait*/ QJsonObject_operator_not_equal_0<bool> for (usize) {
  fn operator_not_equal_0(self , rsthis: & QJsonObject) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QJsonObjectneERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonobject.h:214
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QJsonObject::iterator begin()

/*
Returns an STL-style iterator pointing to the first item in the object.

See also constBegin() and end().
*/
impl /*struct*/ QJsonObject {
  pub fn begin_0<RetType, T: QJsonObject_begin_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.begin_0(self);
    // return 1;
  }
}
pub trait QJsonObject_begin_0<RetType> {
  fn begin_0(self , rsthis: & QJsonObject) -> RetType;
}
impl<'a> /*trait*/ QJsonObject_begin_0<usize> for () {
  fn begin_0(self , rsthis: & QJsonObject) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QJsonObject5beginEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonobject.h:215
// index:1
// Public inline Visibility=Default Availability=Available
// [16] QJsonObject::const_iterator begin() const

/*
Returns an STL-style iterator pointing to the first item in the object.

See also constBegin() and end().
*/
impl /*struct*/ QJsonObject {
  pub fn begin_1<RetType, T: QJsonObject_begin_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.begin_1(self);
    // return 1;
  }
}
pub trait QJsonObject_begin_1<RetType> {
  fn begin_1(self , rsthis: & QJsonObject) -> RetType;
}
impl<'a> /*trait*/ QJsonObject_begin_1<usize> for () {
  fn begin_1(self , rsthis: & QJsonObject) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QJsonObject5beginEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonobject.h:216
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QJsonObject::const_iterator constBegin() const

/*
Returns a const STL-style iterator pointing to the first item in the object.

See also begin() and constEnd().
*/
impl /*struct*/ QJsonObject {
  pub fn constBegin_0<RetType, T: QJsonObject_constBegin_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.constBegin_0(self);
    // return 1;
  }
}
pub trait QJsonObject_constBegin_0<RetType> {
  fn constBegin_0(self , rsthis: & QJsonObject) -> RetType;
}
impl<'a> /*trait*/ QJsonObject_constBegin_0<usize> for () {
  fn constBegin_0(self , rsthis: & QJsonObject) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QJsonObject10constBeginEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonobject.h:217
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QJsonObject::iterator end()

/*
Returns an STL-style iterator pointing to the imaginary item after the last item in the object.

See also begin() and constEnd().
*/
impl /*struct*/ QJsonObject {
  pub fn end_0<RetType, T: QJsonObject_end_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.end_0(self);
    // return 1;
  }
}
pub trait QJsonObject_end_0<RetType> {
  fn end_0(self , rsthis: & QJsonObject) -> RetType;
}
impl<'a> /*trait*/ QJsonObject_end_0<usize> for () {
  fn end_0(self , rsthis: & QJsonObject) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QJsonObject3endEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonobject.h:218
// index:1
// Public inline Visibility=Default Availability=Available
// [16] QJsonObject::const_iterator end() const

/*
Returns an STL-style iterator pointing to the imaginary item after the last item in the object.

See also begin() and constEnd().
*/
impl /*struct*/ QJsonObject {
  pub fn end_1<RetType, T: QJsonObject_end_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.end_1(self);
    // return 1;
  }
}
pub trait QJsonObject_end_1<RetType> {
  fn end_1(self , rsthis: & QJsonObject) -> RetType;
}
impl<'a> /*trait*/ QJsonObject_end_1<usize> for () {
  fn end_1(self , rsthis: & QJsonObject) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QJsonObject3endEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonobject.h:219
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QJsonObject::const_iterator constEnd() const

/*
Returns a const STL-style iterator pointing to the imaginary item after the last item in the object.

See also constBegin() and end().
*/
impl /*struct*/ QJsonObject {
  pub fn constEnd_0<RetType, T: QJsonObject_constEnd_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.constEnd_0(self);
    // return 1;
  }
}
pub trait QJsonObject_constEnd_0<RetType> {
  fn constEnd_0(self , rsthis: & QJsonObject) -> RetType;
}
impl<'a> /*trait*/ QJsonObject_constEnd_0<usize> for () {
  fn constEnd_0(self , rsthis: & QJsonObject) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QJsonObject8constEndEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonobject.h:225
// index:0
// Public Visibility=Default Availability=Available
// [16] QJsonObject::iterator find(const QString &)

/*
Returns an iterator pointing to the item with key key in the map.

If the map contains no item with key key, the function returns end().
*/
impl /*struct*/ QJsonObject {
  pub fn find_0<RetType, T: QJsonObject_find_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.find_0(self);
    // return 1;
  }
}
pub trait QJsonObject_find_0<RetType> {
  fn find_0(self , rsthis: & QJsonObject) -> RetType;
}
impl<'a> /*trait*/ QJsonObject_find_0<usize> for (usize) {
  fn find_0(self , rsthis: & QJsonObject) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QJsonObject4findERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonobject.h:226
// index:1
// Public Visibility=Default Availability=Available
// [16] QJsonObject::iterator find(QLatin1String)

/*
Returns an iterator pointing to the item with key key in the map.

If the map contains no item with key key, the function returns end().
*/
impl /*struct*/ QJsonObject {
  pub fn find_1<RetType, T: QJsonObject_find_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.find_1(self);
    // return 1;
  }
}
pub trait QJsonObject_find_1<RetType> {
  fn find_1(self , rsthis: & QJsonObject) -> RetType;
}
impl<'a> /*trait*/ QJsonObject_find_1<usize> for (usize) {
  fn find_1(self , rsthis: & QJsonObject) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QJsonObject4findE13QLatin1String", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonobject.h:227
// index:2
// Public inline Visibility=Default Availability=Available
// [16] QJsonObject::const_iterator find(const QString &) const

/*
Returns an iterator pointing to the item with key key in the map.

If the map contains no item with key key, the function returns end().
*/
impl /*struct*/ QJsonObject {
  pub fn find_2<RetType, T: QJsonObject_find_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.find_2(self);
    // return 1;
  }
}
pub trait QJsonObject_find_2<RetType> {
  fn find_2(self , rsthis: & QJsonObject) -> RetType;
}
impl<'a> /*trait*/ QJsonObject_find_2<usize> for (usize) {
  fn find_2(self , rsthis: & QJsonObject) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QJsonObject4findERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonobject.h:228
// index:3
// Public inline Visibility=Default Availability=Available
// [16] QJsonObject::const_iterator find(QLatin1String) const

/*
Returns an iterator pointing to the item with key key in the map.

If the map contains no item with key key, the function returns end().
*/
impl /*struct*/ QJsonObject {
  pub fn find_3<RetType, T: QJsonObject_find_3<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.find_3(self);
    // return 1;
  }
}
pub trait QJsonObject_find_3<RetType> {
  fn find_3(self , rsthis: & QJsonObject) -> RetType;
}
impl<'a> /*trait*/ QJsonObject_find_3<usize> for (usize) {
  fn find_3(self , rsthis: & QJsonObject) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QJsonObject4findE13QLatin1String", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonobject.h:229
// index:0
// Public Visibility=Default Availability=Available
// [16] QJsonObject::const_iterator constFind(const QString &) const

/*
Returns a const iterator pointing to the item with key key in the map.

If the map contains no item with key key, the function returns constEnd().
*/
impl /*struct*/ QJsonObject {
  pub fn constFind_0<RetType, T: QJsonObject_constFind_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.constFind_0(self);
    // return 1;
  }
}
pub trait QJsonObject_constFind_0<RetType> {
  fn constFind_0(self , rsthis: & QJsonObject) -> RetType;
}
impl<'a> /*trait*/ QJsonObject_constFind_0<usize> for (usize) {
  fn constFind_0(self , rsthis: & QJsonObject) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QJsonObject9constFindERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonobject.h:230
// index:1
// Public Visibility=Default Availability=Available
// [16] QJsonObject::const_iterator constFind(QLatin1String) const

/*
Returns a const iterator pointing to the item with key key in the map.

If the map contains no item with key key, the function returns constEnd().
*/
impl /*struct*/ QJsonObject {
  pub fn constFind_1<RetType, T: QJsonObject_constFind_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.constFind_1(self);
    // return 1;
  }
}
pub trait QJsonObject_constFind_1<RetType> {
  fn constFind_1(self , rsthis: & QJsonObject) -> RetType;
}
impl<'a> /*trait*/ QJsonObject_constFind_1<usize> for (usize) {
  fn constFind_1(self , rsthis: & QJsonObject) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QJsonObject9constFindE13QLatin1String", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonobject.h:238
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool empty() const

/*
This function is provided for STL compatibility. It is equivalent to isEmpty(), returning true if the object is empty; otherwise returning false.
*/
impl /*struct*/ QJsonObject {
  pub fn empty_0<RetType, T: QJsonObject_empty_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.empty_0(self);
    // return 1;
  }
}
pub trait QJsonObject_empty_0<RetType> {
  fn empty_0(self , rsthis: & QJsonObject) -> RetType;
}
impl<'a> /*trait*/ QJsonObject_empty_0<bool> for () {
  fn empty_0(self , rsthis: & QJsonObject) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QJsonObject5emptyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
