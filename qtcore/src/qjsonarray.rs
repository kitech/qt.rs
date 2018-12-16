

// mod ::core::QJsonArray
// package qtcore
// /usr/include/qt/QtCore/qjsonarray.h
// #include <qjsonarray.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 4
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
#[derive(Default)] // class sizeof(QJsonArray)=16
pub struct QJsonArray {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QJsonArray_ITF interface {
//    QJsonArray_PTR() *QJsonArray
//}
//func (ptr *QJsonArray) QJsonArray_PTR() *QJsonArray { return ptr }

impl /*struct*/ QJsonArray {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QJsonArray {
    return QJsonArray{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QJsonArray {
//  type Target = QJsonArrayBASE;
//
//  fn deref(&self) -> &QJsonArrayBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QJsonArrayBASE> for QJsonArray {
//  fn as_ref(& self) -> & QJsonArrayBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qjsonarray.h:59
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QJsonArray()

/*
Creates an empty array.
*/
// QJsonArray() ctx.fn_proto_cpp
impl /*struct*/ QJsonArray {
  pub fn QJsonArray_0<T: QJsonArray_QJsonArray_0>(value: T) -> QJsonArray {
    let rsthis = value.QJsonArray_0();
    return rsthis;
    // return 1;
  }
}

pub trait QJsonArray_QJsonArray_0 {
  fn QJsonArray_0(self) -> QJsonArray;
}
// QJsonArray() ctx.fn_proto_cpp
impl<'a> /*trait*/ QJsonArray_QJsonArray_0 for () {
  fn QJsonArray_0(self) -> QJsonArray {
    // unsafe{_ZN10QJsonArrayC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN10QJsonArrayC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QJsonArray{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qjsonarray.h:70
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ~QJsonArray()

/*

*/
pub fn DeleteQJsonArray(this :*mut QJsonArray) {
    // let rv = qtrt::InvokeQtFunc6("_ZN10QJsonArrayD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qjsonarray.h:73
// index:0
// Public Visibility=Default Availability=Available
// [16] QJsonArray & operator=(const QJsonArray &)

/*

*/
impl /*struct*/ QJsonArray {
  pub fn operator_equal_0<RetType, T: QJsonArray_operator_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_0(self);
    // return 1;
  }
}
pub trait QJsonArray_operator_equal_0<RetType> {
  fn operator_equal_0(self , rsthis: & QJsonArray) -> RetType;
}
impl<'a> /*trait*/ QJsonArray_operator_equal_0<usize> for (usize) {
  fn operator_equal_0(self , rsthis: & QJsonArray) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QJsonArrayaSERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonarray.h:83
// index:1
// Public inline Visibility=Default Availability=Available
// [16] QJsonArray & operator=(QJsonArray &&)

/*

*/
impl /*struct*/ QJsonArray {
  pub fn operator_equal_1<RetType, T: QJsonArray_operator_equal_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_1(self);
    // return 1;
  }
}
pub trait QJsonArray_operator_equal_1<RetType> {
  fn operator_equal_1(self , rsthis: & QJsonArray) -> RetType;
}
impl<'a> /*trait*/ QJsonArray_operator_equal_1<usize> for (usize) {
  fn operator_equal_1(self , rsthis: & QJsonArray) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QJsonArrayaSEOS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonarray.h:89
// index:0
// Public static Visibility=Default Availability=Available
// [16] QJsonArray fromStringList(const QStringList &)

/*
Converts the string list list to a QJsonArray.

The values in list will be converted to JSON values.

See also toVariantList() and QJsonValue::fromVariant().
*/
impl /*struct*/ QJsonArray {
  pub fn fromStringList_0<RetType, T: QJsonArray_fromStringList_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromStringList_0();
    // return 1;
  }
}
pub trait QJsonArray_fromStringList_0<RetType> {
  fn fromStringList_0(self ) -> RetType;
}
impl<'a> /*trait*/ QJsonArray_fromStringList_0<usize> for (usize) {
  fn fromStringList_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QJsonArray14fromStringListERK11QStringList", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonarray.h:91
// index:0
// Public Visibility=Default Availability=Available
// [8] QVariantList toVariantList() const

/*
Converts this object to a QVariantList.

Returns the created map.
*/
impl /*struct*/ QJsonArray {
  pub fn toVariantList_0<RetType, T: QJsonArray_toVariantList_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toVariantList_0(self);
    // return 1;
  }
}
pub trait QJsonArray_toVariantList_0<RetType> {
  fn toVariantList_0(self , rsthis: & QJsonArray) -> RetType;
}
impl<'a> /*trait*/ QJsonArray_toVariantList_0<usize> for () {
  fn toVariantList_0(self , rsthis: & QJsonArray) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QJsonArray13toVariantListEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonarray.h:93
// index:0
// Public Visibility=Default Availability=Available
// [4] int size() const

/*
Returns the number of values stored in the array.
*/
impl /*struct*/ QJsonArray {
  pub fn size_0<RetType, T: QJsonArray_size_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.size_0(self);
    // return 1;
  }
}
pub trait QJsonArray_size_0<RetType> {
  fn size_0(self , rsthis: & QJsonArray) -> RetType;
}
impl<'a> /*trait*/ QJsonArray_size_0<i32> for () {
  fn size_0(self , rsthis: & QJsonArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QJsonArray4sizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonarray.h:94
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int count() const

/*
Same as size().

See also size().
*/
impl /*struct*/ QJsonArray {
  pub fn count_0<RetType, T: QJsonArray_count_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.count_0(self);
    // return 1;
  }
}
pub trait QJsonArray_count_0<RetType> {
  fn count_0(self , rsthis: & QJsonArray) -> RetType;
}
impl<'a> /*trait*/ QJsonArray_count_0<i32> for () {
  fn count_0(self , rsthis: & QJsonArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QJsonArray5countEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonarray.h:96
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isEmpty() const

/*
Returns true if the object is empty. This is the same as size() == 0.

See also size().
*/
impl /*struct*/ QJsonArray {
  pub fn isEmpty_0<RetType, T: QJsonArray_isEmpty_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isEmpty_0(self);
    // return 1;
  }
}
pub trait QJsonArray_isEmpty_0<RetType> {
  fn isEmpty_0(self , rsthis: & QJsonArray) -> RetType;
}
impl<'a> /*trait*/ QJsonArray_isEmpty_0<bool> for () {
  fn isEmpty_0(self , rsthis: & QJsonArray) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QJsonArray7isEmptyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonarray.h:97
// index:0
// Public Visibility=Default Availability=Available
// [24] QJsonValue at(int) const

/*
Returns a QJsonValue representing the value for index i.

The returned QJsonValue is Undefined, if i is out of bounds.
*/
impl /*struct*/ QJsonArray {
  pub fn at_0<RetType, T: QJsonArray_at_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.at_0(self);
    // return 1;
  }
}
pub trait QJsonArray_at_0<RetType> {
  fn at_0(self , rsthis: & QJsonArray) -> RetType;
}
impl<'a> /*trait*/ QJsonArray_at_0<usize> for (i32) {
  fn at_0(self , rsthis: & QJsonArray) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QJsonArray2atEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonarray.h:98
// index:0
// Public Visibility=Default Availability=Available
// [24] QJsonValue first() const

/*
Returns the first value stored in the array.

Same as at(0).

See also at().
*/
impl /*struct*/ QJsonArray {
  pub fn first_0<RetType, T: QJsonArray_first_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.first_0(self);
    // return 1;
  }
}
pub trait QJsonArray_first_0<RetType> {
  fn first_0(self , rsthis: & QJsonArray) -> RetType;
}
impl<'a> /*trait*/ QJsonArray_first_0<usize> for () {
  fn first_0(self , rsthis: & QJsonArray) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QJsonArray5firstEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonarray.h:99
// index:0
// Public Visibility=Default Availability=Available
// [24] QJsonValue last() const

/*
Returns the last value stored in the array.

Same as at(size() - 1).

See also at().
*/
impl /*struct*/ QJsonArray {
  pub fn last_0<RetType, T: QJsonArray_last_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.last_0(self);
    // return 1;
  }
}
pub trait QJsonArray_last_0<RetType> {
  fn last_0(self , rsthis: & QJsonArray) -> RetType;
}
impl<'a> /*trait*/ QJsonArray_last_0<usize> for () {
  fn last_0(self , rsthis: & QJsonArray) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QJsonArray4lastEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonarray.h:103
// index:0
// Public Visibility=Default Availability=Available
// [-2] void removeAt(int)

/*
Removes the value at index position i. i must be a valid index position in the array (i.e., 0 <= i < size()).

See also insert() and replace().
*/
impl /*struct*/ QJsonArray {
  pub fn removeAt_0<RetType, T: QJsonArray_removeAt_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.removeAt_0(self);
    // return 1;
  }
}
pub trait QJsonArray_removeAt_0<RetType> {
  fn removeAt_0(self , rsthis: & QJsonArray) -> RetType;
}
impl<'a> /*trait*/ QJsonArray_removeAt_0<(/*void*/)> for (i32) {
  fn removeAt_0(self , rsthis: & QJsonArray) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QJsonArray8removeAtEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qjsonarray.h:104
// index:0
// Public Visibility=Default Availability=Available
// [24] QJsonValue takeAt(int)

/*
Removes the item at index position i and returns it. i must be a valid index position in the array (i.e., 0 <= i < size()).

If you don't use the return value, removeAt() is more efficient.

See also removeAt().
*/
impl /*struct*/ QJsonArray {
  pub fn takeAt_0<RetType, T: QJsonArray_takeAt_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.takeAt_0(self);
    // return 1;
  }
}
pub trait QJsonArray_takeAt_0<RetType> {
  fn takeAt_0(self , rsthis: & QJsonArray) -> RetType;
}
impl<'a> /*trait*/ QJsonArray_takeAt_0<usize> for (i32) {
  fn takeAt_0(self , rsthis: & QJsonArray) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QJsonArray6takeAtEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonarray.h:105
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void removeFirst()

/*
Removes the first item in the array. Calling this function is equivalent to calling removeAt(0). The array must not be empty. If the array can be empty, call isEmpty() before calling this function.

See also removeAt() and removeLast().
*/
impl /*struct*/ QJsonArray {
  pub fn removeFirst_0<RetType, T: QJsonArray_removeFirst_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.removeFirst_0(self);
    // return 1;
  }
}
pub trait QJsonArray_removeFirst_0<RetType> {
  fn removeFirst_0(self , rsthis: & QJsonArray) -> RetType;
}
impl<'a> /*trait*/ QJsonArray_removeFirst_0<(/*void*/)> for () {
  fn removeFirst_0(self , rsthis: & QJsonArray) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN10QJsonArray11removeFirstEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qjsonarray.h:106
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void removeLast()

/*
Removes the last item in the array. Calling this function is equivalent to calling removeAt(size() - 1). The array must not be empty. If the array can be empty, call isEmpty() before calling this function.

See also removeAt() and removeFirst().
*/
impl /*struct*/ QJsonArray {
  pub fn removeLast_0<RetType, T: QJsonArray_removeLast_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.removeLast_0(self);
    // return 1;
  }
}
pub trait QJsonArray_removeLast_0<RetType> {
  fn removeLast_0(self , rsthis: & QJsonArray) -> RetType;
}
impl<'a> /*trait*/ QJsonArray_removeLast_0<(/*void*/)> for () {
  fn removeLast_0(self , rsthis: & QJsonArray) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN10QJsonArray10removeLastEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qjsonarray.h:109
// index:0
// Public Visibility=Default Availability=Available
// [-2] void replace(int, const QJsonValue &)

/*
Replaces the item at index position i with value. i must be a valid index position in the array (i.e., 0 <= i < size()).

See also operator[]() and removeAt().
*/
impl /*struct*/ QJsonArray {
  pub fn replace_0<RetType, T: QJsonArray_replace_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.replace_0(self);
    // return 1;
  }
}
pub trait QJsonArray_replace_0<RetType> {
  fn replace_0(self , rsthis: & QJsonArray) -> RetType;
}
impl<'a> /*trait*/ QJsonArray_replace_0<(/*void*/)> for (i32,usize) {
  fn replace_0(self , rsthis: & QJsonArray) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN10QJsonArray7replaceEiRK10QJsonValue", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qjsonarray.h:111
// index:0
// Public Visibility=Default Availability=Available
// [1] bool contains(const QJsonValue &) const

/*
Returns true if the array contains an occurrence of value, otherwise false.

See also count().
*/
impl /*struct*/ QJsonArray {
  pub fn contains_0<RetType, T: QJsonArray_contains_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.contains_0(self);
    // return 1;
  }
}
pub trait QJsonArray_contains_0<RetType> {
  fn contains_0(self , rsthis: & QJsonArray) -> RetType;
}
impl<'a> /*trait*/ QJsonArray_contains_0<bool> for (usize) {
  fn contains_0(self , rsthis: & QJsonArray) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QJsonArray8containsERK10QJsonValue", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonarray.h:112
// index:0
// Public Visibility=Default Availability=Available
// [16] QJsonValueRef operator[](int)

/*

*/
impl /*struct*/ QJsonArray {
  pub fn operator_get_index_0<RetType, T: QJsonArray_operator_get_index_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_get_index_0(self);
    // return 1;
  }
}
pub trait QJsonArray_operator_get_index_0<RetType> {
  fn operator_get_index_0(self , rsthis: & QJsonArray) -> RetType;
}
impl<'a> /*trait*/ QJsonArray_operator_get_index_0<usize> for (i32) {
  fn operator_get_index_0(self , rsthis: & QJsonArray) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QJsonArrayixEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonarray.h:113
// index:1
// Public Visibility=Default Availability=Available
// [24] QJsonValue operator[](int) const

/*

*/
impl /*struct*/ QJsonArray {
  pub fn operator_get_index_1<RetType, T: QJsonArray_operator_get_index_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_get_index_1(self);
    // return 1;
  }
}
pub trait QJsonArray_operator_get_index_1<RetType> {
  fn operator_get_index_1(self , rsthis: & QJsonArray) -> RetType;
}
impl<'a> /*trait*/ QJsonArray_operator_get_index_1<usize> for (i32) {
  fn operator_get_index_1(self , rsthis: & QJsonArray) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QJsonArrayixEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonarray.h:115
// index:0
// Public Visibility=Default Availability=Available
// [1] bool operator==(const QJsonArray &) const

/*

*/
impl /*struct*/ QJsonArray {
  pub fn operator_equal_equal_0<RetType, T: QJsonArray_operator_equal_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_equal_0(self);
    // return 1;
  }
}
pub trait QJsonArray_operator_equal_equal_0<RetType> {
  fn operator_equal_equal_0(self , rsthis: & QJsonArray) -> RetType;
}
impl<'a> /*trait*/ QJsonArray_operator_equal_equal_0<bool> for (usize) {
  fn operator_equal_equal_0(self , rsthis: & QJsonArray) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QJsonArrayeqERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonarray.h:116
// index:0
// Public Visibility=Default Availability=Available
// [1] bool operator!=(const QJsonArray &) const

/*

*/
impl /*struct*/ QJsonArray {
  pub fn operator_not_equal_0<RetType, T: QJsonArray_operator_not_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_not_equal_0(self);
    // return 1;
  }
}
pub trait QJsonArray_operator_not_equal_0<RetType> {
  fn operator_not_equal_0(self , rsthis: & QJsonArray) -> RetType;
}
impl<'a> /*trait*/ QJsonArray_operator_not_equal_0<bool> for (usize) {
  fn operator_not_equal_0(self , rsthis: & QJsonArray) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QJsonArrayneERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonarray.h:118
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void swap(QJsonArray &)

/*
Swaps the array other with this. This operation is very fast and never fails.

This function was introduced in  Qt 5.10.
*/
impl /*struct*/ QJsonArray {
  pub fn swap_0<RetType, T: QJsonArray_swap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.swap_0(self);
    // return 1;
  }
}
pub trait QJsonArray_swap_0<RetType> {
  fn swap_0(self , rsthis: & QJsonArray) -> RetType;
}
impl<'a> /*trait*/ QJsonArray_swap_0<(/*void*/)> for (usize) {
  fn swap_0(self , rsthis: & QJsonArray) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN10QJsonArray4swapERS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qjsonarray.h:214
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QJsonArray::iterator begin()

/*
Returns an STL-style iterator pointing to the first item in the array.

See also constBegin() and end().
*/
impl /*struct*/ QJsonArray {
  pub fn begin_0<RetType, T: QJsonArray_begin_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.begin_0(self);
    // return 1;
  }
}
pub trait QJsonArray_begin_0<RetType> {
  fn begin_0(self , rsthis: & QJsonArray) -> RetType;
}
impl<'a> /*trait*/ QJsonArray_begin_0<usize> for () {
  fn begin_0(self , rsthis: & QJsonArray) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QJsonArray5beginEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonarray.h:215
// index:1
// Public inline Visibility=Default Availability=Available
// [16] QJsonArray::const_iterator begin() const

/*
Returns an STL-style iterator pointing to the first item in the array.

See also constBegin() and end().
*/
impl /*struct*/ QJsonArray {
  pub fn begin_1<RetType, T: QJsonArray_begin_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.begin_1(self);
    // return 1;
  }
}
pub trait QJsonArray_begin_1<RetType> {
  fn begin_1(self , rsthis: & QJsonArray) -> RetType;
}
impl<'a> /*trait*/ QJsonArray_begin_1<usize> for () {
  fn begin_1(self , rsthis: & QJsonArray) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QJsonArray5beginEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonarray.h:216
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QJsonArray::const_iterator constBegin() const

/*
Returns a const STL-style iterator pointing to the first item in the array.

See also begin() and constEnd().
*/
impl /*struct*/ QJsonArray {
  pub fn constBegin_0<RetType, T: QJsonArray_constBegin_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.constBegin_0(self);
    // return 1;
  }
}
pub trait QJsonArray_constBegin_0<RetType> {
  fn constBegin_0(self , rsthis: & QJsonArray) -> RetType;
}
impl<'a> /*trait*/ QJsonArray_constBegin_0<usize> for () {
  fn constBegin_0(self , rsthis: & QJsonArray) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QJsonArray10constBeginEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonarray.h:217
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QJsonArray::iterator end()

/*
Returns an STL-style iterator pointing to the imaginary item after the last item in the array.

See also begin() and constEnd().
*/
impl /*struct*/ QJsonArray {
  pub fn end_0<RetType, T: QJsonArray_end_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.end_0(self);
    // return 1;
  }
}
pub trait QJsonArray_end_0<RetType> {
  fn end_0(self , rsthis: & QJsonArray) -> RetType;
}
impl<'a> /*trait*/ QJsonArray_end_0<usize> for () {
  fn end_0(self , rsthis: & QJsonArray) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QJsonArray3endEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonarray.h:218
// index:1
// Public inline Visibility=Default Availability=Available
// [16] QJsonArray::const_iterator end() const

/*
Returns an STL-style iterator pointing to the imaginary item after the last item in the array.

See also begin() and constEnd().
*/
impl /*struct*/ QJsonArray {
  pub fn end_1<RetType, T: QJsonArray_end_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.end_1(self);
    // return 1;
  }
}
pub trait QJsonArray_end_1<RetType> {
  fn end_1(self , rsthis: & QJsonArray) -> RetType;
}
impl<'a> /*trait*/ QJsonArray_end_1<usize> for () {
  fn end_1(self , rsthis: & QJsonArray) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QJsonArray3endEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonarray.h:219
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QJsonArray::const_iterator constEnd() const

/*
Returns a const STL-style iterator pointing to the imaginary item after the last item in the array.

See also constBegin() and end().
*/
impl /*struct*/ QJsonArray {
  pub fn constEnd_0<RetType, T: QJsonArray_constEnd_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.constEnd_0(self);
    // return 1;
  }
}
pub trait QJsonArray_constEnd_0<RetType> {
  fn constEnd_0(self , rsthis: & QJsonArray) -> RetType;
}
impl<'a> /*trait*/ QJsonArray_constEnd_0<usize> for () {
  fn constEnd_0(self , rsthis: & QJsonArray) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QJsonArray8constEndEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonarray.h:228
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QJsonArray operator+(const QJsonValue &) const

/*

*/
impl /*struct*/ QJsonArray {
  pub fn operator_add_0<RetType, T: QJsonArray_operator_add_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_add_0(self);
    // return 1;
  }
}
pub trait QJsonArray_operator_add_0<RetType> {
  fn operator_add_0(self , rsthis: & QJsonArray) -> RetType;
}
impl<'a> /*trait*/ QJsonArray_operator_add_0<usize> for (usize) {
  fn operator_add_0(self , rsthis: & QJsonArray) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QJsonArrayplERK10QJsonValue", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonarray.h:230
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QJsonArray & operator+=(const QJsonValue &)

/*

*/
impl /*struct*/ QJsonArray {
  pub fn operator_add_equal_0<RetType, T: QJsonArray_operator_add_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_add_equal_0(self);
    // return 1;
  }
}
pub trait QJsonArray_operator_add_equal_0<RetType> {
  fn operator_add_equal_0(self , rsthis: & QJsonArray) -> RetType;
}
impl<'a> /*trait*/ QJsonArray_operator_add_equal_0<usize> for (usize) {
  fn operator_add_equal_0(self , rsthis: & QJsonArray) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QJsonArraypLERK10QJsonValue", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonarray.h:232
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QJsonArray & operator<<(const QJsonValue &)

/*

*/
impl /*struct*/ QJsonArray {
  pub fn operator_left_shift_0<RetType, T: QJsonArray_operator_left_shift_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_left_shift_0(self);
    // return 1;
  }
}
pub trait QJsonArray_operator_left_shift_0<RetType> {
  fn operator_left_shift_0(self , rsthis: & QJsonArray) -> RetType;
}
impl<'a> /*trait*/ QJsonArray_operator_left_shift_0<usize> for (usize) {
  fn operator_left_shift_0(self , rsthis: & QJsonArray) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QJsonArraylsERK10QJsonValue", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonarray.h:236
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void push_back(const QJsonValue &)

/*
This function is provided for STL compatibility. It is equivalent to append(value) and will append value to the array.
*/
impl /*struct*/ QJsonArray {
  pub fn push_back_0<RetType, T: QJsonArray_push_back_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.push_back_0(self);
    // return 1;
  }
}
pub trait QJsonArray_push_back_0<RetType> {
  fn push_back_0(self , rsthis: & QJsonArray) -> RetType;
}
impl<'a> /*trait*/ QJsonArray_push_back_0<(/*void*/)> for (usize) {
  fn push_back_0(self , rsthis: & QJsonArray) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN10QJsonArray9push_backERK10QJsonValue", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qjsonarray.h:237
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void push_front(const QJsonValue &)

/*
This function is provided for STL compatibility. It is equivalent to prepend(value) and will prepend value to the array.
*/
impl /*struct*/ QJsonArray {
  pub fn push_front_0<RetType, T: QJsonArray_push_front_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.push_front_0(self);
    // return 1;
  }
}
pub trait QJsonArray_push_front_0<RetType> {
  fn push_front_0(self , rsthis: & QJsonArray) -> RetType;
}
impl<'a> /*trait*/ QJsonArray_push_front_0<(/*void*/)> for (usize) {
  fn push_front_0(self , rsthis: & QJsonArray) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN10QJsonArray10push_frontERK10QJsonValue", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qjsonarray.h:238
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void pop_front()

/*
This function is provided for STL compatibility. It is equivalent to removeFirst(). The array must not be empty. If the array can be empty, call isEmpty() before calling this function.
*/
impl /*struct*/ QJsonArray {
  pub fn pop_front_0<RetType, T: QJsonArray_pop_front_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.pop_front_0(self);
    // return 1;
  }
}
pub trait QJsonArray_pop_front_0<RetType> {
  fn pop_front_0(self , rsthis: & QJsonArray) -> RetType;
}
impl<'a> /*trait*/ QJsonArray_pop_front_0<(/*void*/)> for () {
  fn pop_front_0(self , rsthis: & QJsonArray) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN10QJsonArray9pop_frontEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qjsonarray.h:239
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void pop_back()

/*
This function is provided for STL compatibility. It is equivalent to removeLast(). The array must not be empty. If the array can be empty, call isEmpty() before calling this function.
*/
impl /*struct*/ QJsonArray {
  pub fn pop_back_0<RetType, T: QJsonArray_pop_back_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.pop_back_0(self);
    // return 1;
  }
}
pub trait QJsonArray_pop_back_0<RetType> {
  fn pop_back_0(self , rsthis: & QJsonArray) -> RetType;
}
impl<'a> /*trait*/ QJsonArray_pop_back_0<(/*void*/)> for () {
  fn pop_back_0(self , rsthis: & QJsonArray) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN10QJsonArray8pop_backEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qjsonarray.h:240
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool empty() const

/*
This function is provided for STL compatibility. It is equivalent to isEmpty() and returns true if the array is empty.
*/
impl /*struct*/ QJsonArray {
  pub fn empty_0<RetType, T: QJsonArray_empty_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.empty_0(self);
    // return 1;
  }
}
pub trait QJsonArray_empty_0<RetType> {
  fn empty_0(self , rsthis: & QJsonArray) -> RetType;
}
impl<'a> /*trait*/ QJsonArray_empty_0<bool> for () {
  fn empty_0(self , rsthis: & QJsonArray) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QJsonArray5emptyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
