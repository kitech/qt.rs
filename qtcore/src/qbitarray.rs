

// mod ::core::QBitArray
// package qtcore
// /usr/include/qt/QtCore/qbitarray.h
// #include <qbitarray.h>
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
use super::super::qtrt;
use super::*;
//  ext block end

//  body block begin



/*

*/
#[derive(Default)] // class sizeof(QBitArray)=8
pub struct QBitArray {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QBitArray_ITF interface {
//    QBitArray_PTR() *QBitArray
//}
//func (ptr *QBitArray) QBitArray_PTR() *QBitArray { return ptr }

impl /*struct*/ QBitArray {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QBitArray {
    return QBitArray{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QBitArray {
//  type Target = QBitArrayBASE;
//
//  fn deref(&self) -> &QBitArrayBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QBitArrayBASE> for QBitArray {
//  fn as_ref(& self) -> & QBitArrayBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qbitarray.h:57
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void QBitArray()

/*
Constructs an empty bit array.

See also isEmpty().
*/
// QBitArray() ctx.fn_proto_cpp
impl /*struct*/ QBitArray {
  pub fn QBitArray_0<T: QBitArray_QBitArray_0>(value: T) -> QBitArray {
    let rsthis = value.QBitArray_0();
    return rsthis;
    // return 1;
  }
}

pub trait QBitArray_QBitArray_0 {
  fn QBitArray_0(self) -> QBitArray;
}
// QBitArray() ctx.fn_proto_cpp
impl<'a> /*trait*/ QBitArray_QBitArray_0 for () {
  fn QBitArray_0(self) -> QBitArray {
    // unsafe{_ZN9QBitArrayC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QBitArrayC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QBitArray{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qbitarray.h:58
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QBitArray(int, bool)

/*
Constructs an empty bit array.

See also isEmpty().
*/
// QBitArray(int, bool) ctx.fn_proto_cpp
impl /*struct*/ QBitArray {
  pub fn QBitArray_1<T: QBitArray_QBitArray_1>(value: T) -> QBitArray {
    let rsthis = value.QBitArray_1();
    return rsthis;
    // return 1;
  }
}

pub trait QBitArray_QBitArray_1 {
  fn QBitArray_1(self) -> QBitArray;
}
// QBitArray(int, bool) ctx.fn_proto_cpp
impl<'a> /*trait*/ QBitArray_QBitArray_1 for (i32,bool) {
  fn QBitArray_1(self) -> QBitArray {
    // unsafe{_ZN9QBitArrayC2Eib()};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const bool as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QBitArrayC2Eib", 2,qtrt::FFITY_INT,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QBitArray{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qbitarray.h:60
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QBitArray & operator=(const QBitArray &)

/*

*/
impl /*struct*/ QBitArray {
  pub fn operator_equal_0<RetType, T: QBitArray_operator_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_0(self);
    // return 1;
  }
}
pub trait QBitArray_operator_equal_0<RetType> {
  fn operator_equal_0(self , rsthis: & QBitArray) -> RetType;
}
impl<'a> /*trait*/ QBitArray_operator_equal_0<usize> for (usize) {
  fn operator_equal_0(self , rsthis: & QBitArray) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QBitArrayaSERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbitarray.h:63
// index:1
// Public inline Visibility=Default Availability=Available
// [8] QBitArray & operator=(QBitArray &&)

/*

*/
impl /*struct*/ QBitArray {
  pub fn operator_equal_1<RetType, T: QBitArray_operator_equal_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_1(self);
    // return 1;
  }
}
pub trait QBitArray_operator_equal_1<RetType> {
  fn operator_equal_1(self , rsthis: & QBitArray) -> RetType;
}
impl<'a> /*trait*/ QBitArray_operator_equal_1<usize> for (usize) {
  fn operator_equal_1(self , rsthis: & QBitArray) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QBitArrayaSEOS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbitarray.h:67
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void swap(QBitArray &)

/*
Swaps bit array other with this bit array. This operation is very fast and never fails.

This function was introduced in  Qt 4.8.
*/
impl /*struct*/ QBitArray {
  pub fn swap_0<RetType, T: QBitArray_swap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.swap_0(self);
    // return 1;
  }
}
pub trait QBitArray_swap_0<RetType> {
  fn swap_0(self , rsthis: & QBitArray) -> RetType;
}
impl<'a> /*trait*/ QBitArray_swap_0<(/*void*/)> for (usize) {
  fn swap_0(self , rsthis: & QBitArray) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QBitArray4swapERS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qbitarray.h:69
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int size() const

/*
Returns the number of bits stored in the bit array.

See also resize().
*/
impl /*struct*/ QBitArray {
  pub fn size_0<RetType, T: QBitArray_size_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.size_0(self);
    // return 1;
  }
}
pub trait QBitArray_size_0<RetType> {
  fn size_0(self , rsthis: & QBitArray) -> RetType;
}
impl<'a> /*trait*/ QBitArray_size_0<i32> for () {
  fn size_0(self , rsthis: & QBitArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QBitArray4sizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbitarray.h:70
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int count() const

/*
Same as size().
*/
impl /*struct*/ QBitArray {
  pub fn count_0<RetType, T: QBitArray_count_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.count_0(self);
    // return 1;
  }
}
pub trait QBitArray_count_0<RetType> {
  fn count_0(self , rsthis: & QBitArray) -> RetType;
}
impl<'a> /*trait*/ QBitArray_count_0<i32> for () {
  fn count_0(self , rsthis: & QBitArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QBitArray5countEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbitarray.h:71
// index:1
// Public Visibility=Default Availability=Available
// [4] int count(bool) const

/*
Same as size().
*/
impl /*struct*/ QBitArray {
  pub fn count_1<RetType, T: QBitArray_count_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.count_1(self);
    // return 1;
  }
}
pub trait QBitArray_count_1<RetType> {
  fn count_1(self , rsthis: & QBitArray) -> RetType;
}
impl<'a> /*trait*/ QBitArray_count_1<i32> for (bool) {
  fn count_1(self , rsthis: & QBitArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QBitArray5countEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbitarray.h:73
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isEmpty() const

/*
Returns true if this bit array has size 0; otherwise returns false.

See also size().
*/
impl /*struct*/ QBitArray {
  pub fn isEmpty_0<RetType, T: QBitArray_isEmpty_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isEmpty_0(self);
    // return 1;
  }
}
pub trait QBitArray_isEmpty_0<RetType> {
  fn isEmpty_0(self , rsthis: & QBitArray) -> RetType;
}
impl<'a> /*trait*/ QBitArray_isEmpty_0<bool> for () {
  fn isEmpty_0(self , rsthis: & QBitArray) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QBitArray7isEmptyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbitarray.h:74
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isNull() const

/*
Returns true if this bit array is null; otherwise returns false.

Example:


  QBitArray().isNull();           // returns true
  QBitArray(0).isNull();          // returns false
  QBitArray(3).isNull();          // returns false



Qt makes a distinction between null bit arrays and empty bit arrays for historical reasons. For most applications, what matters is whether or not a bit array contains any data, and this can be determined using isEmpty().

See also isEmpty().
*/
impl /*struct*/ QBitArray {
  pub fn isNull_0<RetType, T: QBitArray_isNull_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isNull_0(self);
    // return 1;
  }
}
pub trait QBitArray_isNull_0<RetType> {
  fn isNull_0(self , rsthis: & QBitArray) -> RetType;
}
impl<'a> /*trait*/ QBitArray_isNull_0<bool> for () {
  fn isNull_0(self , rsthis: & QBitArray) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QBitArray6isNullEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbitarray.h:76
// index:0
// Public Visibility=Default Availability=Available
// [-2] void resize(int)

/*
Resizes the bit array to size bits.

If size is greater than the current size, the bit array is extended to make it size bits with the extra bits added to the end. The new bits are initialized to false (0).

If size is less than the current size, bits are removed from the end.

See also size().
*/
impl /*struct*/ QBitArray {
  pub fn resize_0<RetType, T: QBitArray_resize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resize_0(self);
    // return 1;
  }
}
pub trait QBitArray_resize_0<RetType> {
  fn resize_0(self , rsthis: & QBitArray) -> RetType;
}
impl<'a> /*trait*/ QBitArray_resize_0<(/*void*/)> for (i32) {
  fn resize_0(self , rsthis: & QBitArray) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QBitArray6resizeEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qbitarray.h:78
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void detach()

/*

*/
impl /*struct*/ QBitArray {
  pub fn detach_0<RetType, T: QBitArray_detach_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.detach_0(self);
    // return 1;
  }
}
pub trait QBitArray_detach_0<RetType> {
  fn detach_0(self , rsthis: & QBitArray) -> RetType;
}
impl<'a> /*trait*/ QBitArray_detach_0<(/*void*/)> for () {
  fn detach_0(self , rsthis: & QBitArray) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN9QBitArray6detachEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qbitarray.h:79
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isDetached() const

/*

*/
impl /*struct*/ QBitArray {
  pub fn isDetached_0<RetType, T: QBitArray_isDetached_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isDetached_0(self);
    // return 1;
  }
}
pub trait QBitArray_isDetached_0<RetType> {
  fn isDetached_0(self , rsthis: & QBitArray) -> RetType;
}
impl<'a> /*trait*/ QBitArray_isDetached_0<bool> for () {
  fn isDetached_0(self , rsthis: & QBitArray) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QBitArray10isDetachedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbitarray.h:80
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void clear()

/*
Clears the contents of the bit array and makes it empty.

See also resize() and isEmpty().
*/
impl /*struct*/ QBitArray {
  pub fn clear_0<RetType, T: QBitArray_clear_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.clear_0(self);
    // return 1;
  }
}
pub trait QBitArray_clear_0<RetType> {
  fn clear_0(self , rsthis: & QBitArray) -> RetType;
}
impl<'a> /*trait*/ QBitArray_clear_0<(/*void*/)> for () {
  fn clear_0(self , rsthis: & QBitArray) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN9QBitArray5clearEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qbitarray.h:82
// index:0
// Public Visibility=Default Availability=Available
// [1] bool testBit(int) const

/*
Returns true if the bit at index position i is 1; otherwise returns false.

i must be a valid index position in the bit array (i.e., 0 <= i < size()).

See also setBit() and clearBit().
*/
impl /*struct*/ QBitArray {
  pub fn testBit_0<RetType, T: QBitArray_testBit_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.testBit_0(self);
    // return 1;
  }
}
pub trait QBitArray_testBit_0<RetType> {
  fn testBit_0(self , rsthis: & QBitArray) -> RetType;
}
impl<'a> /*trait*/ QBitArray_testBit_0<bool> for (i32) {
  fn testBit_0(self , rsthis: & QBitArray) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QBitArray7testBitEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbitarray.h:83
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setBit(int)

/*
Sets the bit at index position i to 1.

i must be a valid index position in the bit array (i.e., 0 <= i < size()).

See also clearBit() and toggleBit().
*/
impl /*struct*/ QBitArray {
  pub fn setBit_0<RetType, T: QBitArray_setBit_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setBit_0(self);
    // return 1;
  }
}
pub trait QBitArray_setBit_0<RetType> {
  fn setBit_0(self , rsthis: & QBitArray) -> RetType;
}
impl<'a> /*trait*/ QBitArray_setBit_0<(/*void*/)> for (i32) {
  fn setBit_0(self , rsthis: & QBitArray) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QBitArray6setBitEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qbitarray.h:84
// index:1
// Public Visibility=Default Availability=Available
// [-2] void setBit(int, bool)

/*
Sets the bit at index position i to 1.

i must be a valid index position in the bit array (i.e., 0 <= i < size()).

See also clearBit() and toggleBit().
*/
impl /*struct*/ QBitArray {
  pub fn setBit_1<RetType, T: QBitArray_setBit_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setBit_1(self);
    // return 1;
  }
}
pub trait QBitArray_setBit_1<RetType> {
  fn setBit_1(self , rsthis: & QBitArray) -> RetType;
}
impl<'a> /*trait*/ QBitArray_setBit_1<(/*void*/)> for (i32,bool) {
  fn setBit_1(self , rsthis: & QBitArray) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN9QBitArray6setBitEib", 2,qtrt::FFITY_INT,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qbitarray.h:85
// index:0
// Public Visibility=Default Availability=Available
// [-2] void clearBit(int)

/*
Sets the bit at index position i to 0.

i must be a valid index position in the bit array (i.e., 0 <= i < size()).

See also setBit() and toggleBit().
*/
impl /*struct*/ QBitArray {
  pub fn clearBit_0<RetType, T: QBitArray_clearBit_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.clearBit_0(self);
    // return 1;
  }
}
pub trait QBitArray_clearBit_0<RetType> {
  fn clearBit_0(self , rsthis: & QBitArray) -> RetType;
}
impl<'a> /*trait*/ QBitArray_clearBit_0<(/*void*/)> for (i32) {
  fn clearBit_0(self , rsthis: & QBitArray) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QBitArray8clearBitEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qbitarray.h:86
// index:0
// Public Visibility=Default Availability=Available
// [1] bool toggleBit(int)

/*
Inverts the value of the bit at index position i, returning the previous value of that bit as either true (if it was set) or false (if it was unset).

If the previous value was 0, the new value will be 1. If the previous value was 1, the new value will be 0.

i must be a valid index position in the bit array (i.e., 0 <= i < size()).

See also setBit() and clearBit().
*/
impl /*struct*/ QBitArray {
  pub fn toggleBit_0<RetType, T: QBitArray_toggleBit_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toggleBit_0(self);
    // return 1;
  }
}
pub trait QBitArray_toggleBit_0<RetType> {
  fn toggleBit_0(self , rsthis: & QBitArray) -> RetType;
}
impl<'a> /*trait*/ QBitArray_toggleBit_0<bool> for (i32) {
  fn toggleBit_0(self , rsthis: & QBitArray) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QBitArray9toggleBitEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbitarray.h:88
// index:0
// Public Visibility=Default Availability=Available
// [1] bool at(int) const

/*
Returns the value of the bit at index position i.

i must be a valid index position in the bit array (i.e., 0 <= i < size()).

See also operator[]().
*/
impl /*struct*/ QBitArray {
  pub fn at_0<RetType, T: QBitArray_at_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.at_0(self);
    // return 1;
  }
}
pub trait QBitArray_at_0<RetType> {
  fn at_0(self , rsthis: & QBitArray) -> RetType;
}
impl<'a> /*trait*/ QBitArray_at_0<bool> for (i32) {
  fn at_0(self , rsthis: & QBitArray) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QBitArray2atEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbitarray.h:89
// index:0
// Public Visibility=Default Availability=Available
// [16] QBitRef operator[](int)

/*

*/
impl /*struct*/ QBitArray {
  pub fn operator_get_index_0<RetType, T: QBitArray_operator_get_index_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_get_index_0(self);
    // return 1;
  }
}
pub trait QBitArray_operator_get_index_0<RetType> {
  fn operator_get_index_0(self , rsthis: & QBitArray) -> RetType;
}
impl<'a> /*trait*/ QBitArray_operator_get_index_0<usize> for (i32) {
  fn operator_get_index_0(self , rsthis: & QBitArray) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QBitArrayixEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbitarray.h:90
// index:1
// Public Visibility=Default Availability=Available
// [1] bool operator[](int) const

/*

*/
impl /*struct*/ QBitArray {
  pub fn operator_get_index_1<RetType, T: QBitArray_operator_get_index_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_get_index_1(self);
    // return 1;
  }
}
pub trait QBitArray_operator_get_index_1<RetType> {
  fn operator_get_index_1(self , rsthis: & QBitArray) -> RetType;
}
impl<'a> /*trait*/ QBitArray_operator_get_index_1<bool> for (i32) {
  fn operator_get_index_1(self , rsthis: & QBitArray) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QBitArrayixEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbitarray.h:91
// index:2
// Public Visibility=Default Availability=Available
// [16] QBitRef operator[](uint)

/*

*/
impl /*struct*/ QBitArray {
  pub fn operator_get_index_2<RetType, T: QBitArray_operator_get_index_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_get_index_2(self);
    // return 1;
  }
}
pub trait QBitArray_operator_get_index_2<RetType> {
  fn operator_get_index_2(self , rsthis: & QBitArray) -> RetType;
}
impl<'a> /*trait*/ QBitArray_operator_get_index_2<usize> for (u32) {
  fn operator_get_index_2(self , rsthis: & QBitArray) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const u32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QBitArrayixEj", 1,qtrt::FFITY_UINT32,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbitarray.h:92
// index:3
// Public Visibility=Default Availability=Available
// [1] bool operator[](uint) const

/*

*/
impl /*struct*/ QBitArray {
  pub fn operator_get_index_3<RetType, T: QBitArray_operator_get_index_3<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_get_index_3(self);
    // return 1;
  }
}
pub trait QBitArray_operator_get_index_3<RetType> {
  fn operator_get_index_3(self , rsthis: & QBitArray) -> RetType;
}
impl<'a> /*trait*/ QBitArray_operator_get_index_3<bool> for (u32) {
  fn operator_get_index_3(self , rsthis: & QBitArray) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const u32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QBitArrayixEj", 1,qtrt::FFITY_UINT32,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbitarray.h:94
// index:0
// Public Visibility=Default Availability=Available
// [8] QBitArray & operator&=(const QBitArray &)

/*

*/
impl /*struct*/ QBitArray {
  pub fn operator_and_equal_0<RetType, T: QBitArray_operator_and_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_and_equal_0(self);
    // return 1;
  }
}
pub trait QBitArray_operator_and_equal_0<RetType> {
  fn operator_and_equal_0(self , rsthis: & QBitArray) -> RetType;
}
impl<'a> /*trait*/ QBitArray_operator_and_equal_0<usize> for (usize) {
  fn operator_and_equal_0(self , rsthis: & QBitArray) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QBitArrayaNERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbitarray.h:95
// index:0
// Public Visibility=Default Availability=Available
// [8] QBitArray & operator|=(const QBitArray &)

/*

*/
impl /*struct*/ QBitArray {
  pub fn operator_or_equal_0<RetType, T: QBitArray_operator_or_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_or_equal_0(self);
    // return 1;
  }
}
pub trait QBitArray_operator_or_equal_0<RetType> {
  fn operator_or_equal_0(self , rsthis: & QBitArray) -> RetType;
}
impl<'a> /*trait*/ QBitArray_operator_or_equal_0<usize> for (usize) {
  fn operator_or_equal_0(self , rsthis: & QBitArray) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QBitArrayoRERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbitarray.h:96
// index:0
// Public Visibility=Default Availability=Available
// [8] QBitArray & operator^=(const QBitArray &)

/*

*/
impl /*struct*/ QBitArray {
  pub fn operator_caret_equal_0<RetType, T: QBitArray_operator_caret_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_caret_equal_0(self);
    // return 1;
  }
}
pub trait QBitArray_operator_caret_equal_0<RetType> {
  fn operator_caret_equal_0(self , rsthis: & QBitArray) -> RetType;
}
impl<'a> /*trait*/ QBitArray_operator_caret_equal_0<usize> for (usize) {
  fn operator_caret_equal_0(self , rsthis: & QBitArray) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QBitArrayeOERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbitarray.h:97
// index:0
// Public Visibility=Default Availability=Available
// [8] QBitArray operator~() const

/*

*/
impl /*struct*/ QBitArray {
  pub fn operator_around_0<RetType, T: QBitArray_operator_around_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_around_0(self);
    // return 1;
  }
}
pub trait QBitArray_operator_around_0<RetType> {
  fn operator_around_0(self , rsthis: & QBitArray) -> RetType;
}
impl<'a> /*trait*/ QBitArray_operator_around_0<usize> for () {
  fn operator_around_0(self , rsthis: & QBitArray) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QBitArraycoEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbitarray.h:99
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator==(const QBitArray &) const

/*

*/
impl /*struct*/ QBitArray {
  pub fn operator_equal_equal_0<RetType, T: QBitArray_operator_equal_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_equal_0(self);
    // return 1;
  }
}
pub trait QBitArray_operator_equal_equal_0<RetType> {
  fn operator_equal_equal_0(self , rsthis: & QBitArray) -> RetType;
}
impl<'a> /*trait*/ QBitArray_operator_equal_equal_0<bool> for (usize) {
  fn operator_equal_equal_0(self , rsthis: & QBitArray) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QBitArrayeqERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbitarray.h:100
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator!=(const QBitArray &) const

/*

*/
impl /*struct*/ QBitArray {
  pub fn operator_not_equal_0<RetType, T: QBitArray_operator_not_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_not_equal_0(self);
    // return 1;
  }
}
pub trait QBitArray_operator_not_equal_0<RetType> {
  fn operator_not_equal_0(self , rsthis: & QBitArray) -> RetType;
}
impl<'a> /*trait*/ QBitArray_operator_not_equal_0<bool> for (usize) {
  fn operator_not_equal_0(self , rsthis: & QBitArray) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QBitArrayneERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbitarray.h:102
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool fill(bool, int)

/*
Sets every bit in the bit array to value, returning true if successful; otherwise returns false. If size is different from -1 (the default), the bit array is resized to size beforehand.

Example:


  QBitArray ba(8);
  ba.fill(true);
  // ba: [ 1, 1, 1, 1, 1, 1, 1, 1 ]

  ba.fill(false, 2);
  // ba: [ 0, 0 ]



See also resize().
*/
impl /*struct*/ QBitArray {
  pub fn fill_0<RetType, T: QBitArray_fill_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fill_0(self);
    // return 1;
  }
}
pub trait QBitArray_fill_0<RetType> {
  fn fill_0(self , rsthis: & QBitArray) -> RetType;
}
impl<'a> /*trait*/ QBitArray_fill_0<bool> for (bool,i32) {
  fn fill_0(self , rsthis: & QBitArray) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const bool as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QBitArray4fillEbi", 2,qtrt::FFITY_SINT8,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbitarray.h:103
// index:1
// Public Visibility=Default Availability=Available
// [-2] void fill(bool, int, int)

/*
Sets every bit in the bit array to value, returning true if successful; otherwise returns false. If size is different from -1 (the default), the bit array is resized to size beforehand.

Example:


  QBitArray ba(8);
  ba.fill(true);
  // ba: [ 1, 1, 1, 1, 1, 1, 1, 1 ]

  ba.fill(false, 2);
  // ba: [ 0, 0 ]



See also resize().
*/
impl /*struct*/ QBitArray {
  pub fn fill_1<RetType, T: QBitArray_fill_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fill_1(self);
    // return 1;
  }
}
pub trait QBitArray_fill_1<RetType> {
  fn fill_1(self , rsthis: & QBitArray) -> RetType;
}
impl<'a> /*trait*/ QBitArray_fill_1<(/*void*/)> for (bool,i32,i32) {
  fn fill_1(self , rsthis: & QBitArray) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const bool as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QBitArray4fillEbii", 3,qtrt::FFITY_SINT8,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qbitarray.h:105
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void truncate(int)

/*
Truncates the bit array at index position pos.

If pos is beyond the end of the array, nothing happens.

See also resize().
*/
impl /*struct*/ QBitArray {
  pub fn truncate_0<RetType, T: QBitArray_truncate_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.truncate_0(self);
    // return 1;
  }
}
pub trait QBitArray_truncate_0<RetType> {
  fn truncate_0(self , rsthis: & QBitArray) -> RetType;
}
impl<'a> /*trait*/ QBitArray_truncate_0<(/*void*/)> for (i32) {
  fn truncate_0(self , rsthis: & QBitArray) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QBitArray8truncateEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}


pub fn DeleteQBitArray(this :*mut QBitArray) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN9QBitArrayD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
