

// mod ::core::QByteArray
// package qtcore
// /usr/include/qt/QtCore/qbytearray.h
// #include <qbytearray.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 0
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
#[derive(Default)] // class sizeof(QByteArray)=8
pub struct QByteArray {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QByteArray_ITF interface {
//    QByteArray_PTR() *QByteArray
//}
//func (ptr *QByteArray) QByteArray_PTR() *QByteArray { return ptr }

impl /*struct*/ QByteArray {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QByteArray {
    return QByteArray{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QByteArray {
//  type Target = QByteArrayBASE;
//
//  fn deref(&self) -> &QByteArrayBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QByteArrayBASE> for QByteArray {
//  fn as_ref(& self) -> & QByteArrayBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qbytearray.h:170
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void QByteArray()

/*
Constructs an empty byte array.

See also isEmpty().
*/
// QByteArray() ctx.fn_proto_cpp
impl /*struct*/ QByteArray {
  pub fn QByteArray_0<T: QByteArray_QByteArray_0>(value: T) -> QByteArray {
    let rsthis = value.QByteArray_0();
    return rsthis;
    // return 1;
  }
}

pub trait QByteArray_QByteArray_0 {
  fn QByteArray_0(self) -> QByteArray;
}
// QByteArray() ctx.fn_proto_cpp
impl<'a> /*trait*/ QByteArray_QByteArray_0 for () {
  fn QByteArray_0(self) -> QByteArray {
    // unsafe{_ZN10QByteArrayC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN10QByteArrayC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QByteArray{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:171
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QByteArray(const char *, int)

/*
Constructs an empty byte array.

See also isEmpty().
*/
// QByteArray(const char *, int) ctx.fn_proto_cpp
impl /*struct*/ QByteArray {
  pub fn QByteArray_1<T: QByteArray_QByteArray_1>(value: T) -> QByteArray {
    let rsthis = value.QByteArray_1();
    return rsthis;
    // return 1;
  }
}

pub trait QByteArray_QByteArray_1 {
  fn QByteArray_1(self) -> QByteArray;
}
// QByteArray(const char *, int) ctx.fn_proto_cpp
impl<'a> /*trait*/ QByteArray_QByteArray_1 for (usize,i32) {
  fn QByteArray_1(self) -> QByteArray {
    // unsafe{_ZN10QByteArrayC2EPKci()};
    let arg0 = (self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN10QByteArrayC2EPKci", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QByteArray{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:172
// index:2
// Public Visibility=Default Availability=Available
// [-2] void QByteArray(int, char)

/*
Constructs an empty byte array.

See also isEmpty().
*/
// QByteArray(int, char) ctx.fn_proto_cpp
impl /*struct*/ QByteArray {
  pub fn QByteArray_2<T: QByteArray_QByteArray_2>(value: T) -> QByteArray {
    let rsthis = value.QByteArray_2();
    return rsthis;
    // return 1;
  }
}

pub trait QByteArray_QByteArray_2 {
  fn QByteArray_2(self) -> QByteArray;
}
// QByteArray(int, char) ctx.fn_proto_cpp
impl<'a> /*trait*/ QByteArray_QByteArray_2 for (i32,i8) {
  fn QByteArray_2(self) -> QByteArray {
    // unsafe{_ZN10QByteArrayC2Eic()};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i8 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN10QByteArrayC2Eic", 2,qtrt::FFITY_INT,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QByteArray{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:173
// index:3
// Public Visibility=Default Availability=Available
// [-2] void QByteArray(int, Qt::Initialization)

/*
Constructs an empty byte array.

See also isEmpty().
*/
// QByteArray(int, Qt::Initialization) ctx.fn_proto_cpp
impl /*struct*/ QByteArray {
  pub fn QByteArray_3<T: QByteArray_QByteArray_3>(value: T) -> QByteArray {
    let rsthis = value.QByteArray_3();
    return rsthis;
    // return 1;
  }
}

pub trait QByteArray_QByteArray_3 {
  fn QByteArray_3(self) -> QByteArray;
}
// QByteArray(int, Qt::Initialization) ctx.fn_proto_cpp
impl<'a> /*trait*/ QByteArray_QByteArray_3 for (i32,i32) {
  fn QByteArray_3(self) -> QByteArray {
    // unsafe{_ZN10QByteArrayC2EiN2Qt14InitializationE()};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN10QByteArrayC2EiN2Qt14InitializationE", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QByteArray{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:175
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void ~QByteArray()

/*

*/
pub fn DeleteQByteArray(this :*mut QByteArray) {
    // let rv = qtrt::InvokeQtFunc6("_ZN10QByteArrayD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 8)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qbytearray.h:177
// index:0
// Public Visibility=Default Availability=Available
// [8] QByteArray & operator=(const QByteArray &)

/*

*/
impl /*struct*/ QByteArray {
  pub fn operator_equal_0<RetType, T: QByteArray_operator_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_0(self);
    // return 1;
  }
}
pub trait QByteArray_operator_equal_0<RetType> {
  fn operator_equal_0(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_operator_equal_0<usize> for (usize) {
  fn operator_equal_0(self , rsthis: & QByteArray) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QByteArrayaSERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:178
// index:1
// Public Visibility=Default Availability=Available
// [8] QByteArray & operator=(const char *)

/*

*/
impl /*struct*/ QByteArray {
  pub fn operator_equal_1<RetType, T: QByteArray_operator_equal_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_1(self);
    // return 1;
  }
}
pub trait QByteArray_operator_equal_1<RetType> {
  fn operator_equal_1(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_operator_equal_1<usize> for (usize) {
  fn operator_equal_1(self , rsthis: & QByteArray) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QByteArrayaSEPKc", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:181
// index:2
// Public inline Visibility=Default Availability=Available
// [8] QByteArray & operator=(QByteArray &&)

/*

*/
impl /*struct*/ QByteArray {
  pub fn operator_equal_2<RetType, T: QByteArray_operator_equal_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_2(self);
    // return 1;
  }
}
pub trait QByteArray_operator_equal_2<RetType> {
  fn operator_equal_2(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_operator_equal_2<usize> for (usize) {
  fn operator_equal_2(self , rsthis: & QByteArray) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QByteArrayaSEOS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:185
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void swap(QByteArray &)

/*
Swaps byte array other with this byte array. This operation is very fast and never fails.

This function was introduced in  Qt 4.8.
*/
impl /*struct*/ QByteArray {
  pub fn swap_0<RetType, T: QByteArray_swap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.swap_0(self);
    // return 1;
  }
}
pub trait QByteArray_swap_0<RetType> {
  fn swap_0(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_swap_0<(/*void*/)> for (usize) {
  fn swap_0(self , rsthis: & QByteArray) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN10QByteArray4swapERS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:188
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int size() const

/*
Returns the number of bytes in this byte array.

The last byte in the byte array is at position size() - 1. In addition, QByteArray ensures that the byte at position size() is always '\0', so that you can use the return value of data() and constData() as arguments to functions that expect '\0'-terminated strings. If the QByteArray object was created from a raw data that didn't include the trailing null-termination character then QByteArray doesn't add it automaticall unless the deep copy is created.

Example:


  QByteArray ba("Hello");
  int n = ba.size();          // n == 5
  ba.data()[0];               // returns 'H'
  ba.data()[4];               // returns 'o'
  ba.data()[5];               // returns '\0'



See also isEmpty() and resize().
*/
impl /*struct*/ QByteArray {
  pub fn size_0<RetType, T: QByteArray_size_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.size_0(self);
    // return 1;
  }
}
pub trait QByteArray_size_0<RetType> {
  fn size_0(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_size_0<i32> for () {
  fn size_0(self , rsthis: & QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QByteArray4sizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:189
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isEmpty() const

/*
Returns true if the byte array has size 0; otherwise returns false.

Example:


  QByteArray().isEmpty();         // returns true
  QByteArray("").isEmpty();       // returns true
  QByteArray("abc").isEmpty();    // returns false



See also size().
*/
impl /*struct*/ QByteArray {
  pub fn isEmpty_0<RetType, T: QByteArray_isEmpty_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isEmpty_0(self);
    // return 1;
  }
}
pub trait QByteArray_isEmpty_0<RetType> {
  fn isEmpty_0(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_isEmpty_0<bool> for () {
  fn isEmpty_0(self , rsthis: & QByteArray) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QByteArray7isEmptyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:190
// index:0
// Public Visibility=Default Availability=Available
// [-2] void resize(int)

/*
Sets the size of the byte array to size bytes.

If size is greater than the current size, the byte array is extended to make it size bytes with the extra bytes added to the end. The new bytes are uninitialized.

If size is less than the current size, bytes are removed from the end.

See also size() and truncate().
*/
impl /*struct*/ QByteArray {
  pub fn resize_0<RetType, T: QByteArray_resize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resize_0(self);
    // return 1;
  }
}
pub trait QByteArray_resize_0<RetType> {
  fn resize_0(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_resize_0<(/*void*/)> for (i32) {
  fn resize_0(self , rsthis: & QByteArray) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QByteArray6resizeEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:192
// index:0
// Public Visibility=Default Availability=Available
// [8] QByteArray & fill(char, int)

/*
Sets every byte in the byte array to character ch. If size is different from -1 (the default), the byte array is resized to size size beforehand.

Example:


  QByteArray ba("Istambul");
  ba.fill('o');
  // ba == "oooooooo"

  ba.fill('X', 2);
  // ba == "XX"



See also resize().
*/
impl /*struct*/ QByteArray {
  pub fn fill_0<RetType, T: QByteArray_fill_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fill_0(self);
    // return 1;
  }
}
pub trait QByteArray_fill_0<RetType> {
  fn fill_0(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_fill_0<usize> for (i8,i32) {
  fn fill_0(self , rsthis: & QByteArray) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i8 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QByteArray4fillEci", 2,qtrt::FFITY_SINT8,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:194
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int capacity() const

/*
Returns the maximum number of bytes that can be stored in the byte array without forcing a reallocation.

The sole purpose of this function is to provide a means of fine tuning QByteArray's memory usage. In general, you will rarely ever need to call this function. If you want to know how many bytes are in the byte array, call size().

See also reserve() and squeeze().
*/
impl /*struct*/ QByteArray {
  pub fn capacity_0<RetType, T: QByteArray_capacity_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.capacity_0(self);
    // return 1;
  }
}
pub trait QByteArray_capacity_0<RetType> {
  fn capacity_0(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_capacity_0<i32> for () {
  fn capacity_0(self , rsthis: & QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QByteArray8capacityEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:195
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void reserve(int)

/*
Attempts to allocate memory for at least size bytes. If you know in advance how large the byte array will be, you can call this function, and if you call resize() often you are likely to get better performance. If size is an underestimate, the worst that will happen is that the QByteArray will be a bit slower.

The sole purpose of this function is to provide a means of fine tuning QByteArray's memory usage. In general, you will rarely ever need to call this function. If you want to change the size of the byte array, call resize().

See also squeeze() and capacity().
*/
impl /*struct*/ QByteArray {
  pub fn reserve_0<RetType, T: QByteArray_reserve_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.reserve_0(self);
    // return 1;
  }
}
pub trait QByteArray_reserve_0<RetType> {
  fn reserve_0(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_reserve_0<(/*void*/)> for (i32) {
  fn reserve_0(self , rsthis: & QByteArray) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QByteArray7reserveEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:196
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void squeeze()

/*
Releases any memory not required to store the array's data.

The sole purpose of this function is to provide a means of fine tuning QByteArray's memory usage. In general, you will rarely ever need to call this function.

See also reserve() and capacity().
*/
impl /*struct*/ QByteArray {
  pub fn squeeze_0<RetType, T: QByteArray_squeeze_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.squeeze_0(self);
    // return 1;
  }
}
pub trait QByteArray_squeeze_0<RetType> {
  fn squeeze_0(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_squeeze_0<(/*void*/)> for () {
  fn squeeze_0(self , rsthis: & QByteArray) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN10QByteArray7squeezeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:202
// index:0
// Public inline Visibility=Default Availability=Available
// [8] char * data()

/*
Returns a pointer to the data stored in the byte array. The pointer can be used to access and modify the bytes that compose the array. The data is '\0'-terminated, i.e. the number of bytes in the returned character string is size() + 1 for the '\0' terminator.

Example:


  QByteArray ba("Hello world");
  char *data = ba.data();
  while (*data) {
      cout << "[" << *data << "]" << endl;
      ++data;
  }



The pointer remains valid as long as the byte array isn't reallocated or destroyed. For read-only access, constData() is faster because it never causes a deep copy to occur.

This function is mostly useful to pass a byte array to a function that accepts a const char *.

The following example makes a copy of the char* returned by data(), but it will corrupt the heap and cause a crash because it does not allocate a byte for the '\0' at the end:


  QString tmp = "test";
  QByteArray text = tmp.toLocal8Bit();
  char *data = new char[text.size()];
  strcpy(data, text.data());
  delete [] data;



This one allocates the correct amount of space:


  QString tmp = "test";
  QByteArray text = tmp.toLocal8Bit();
  char *data = new char[text.size() + 1];
  strcpy(data, text.data());
  delete [] data;



Note: A QByteArray can store any byte values including '\0's, but most functions that take char * arguments assume that the data ends at the first '\0' they encounter.

See also constData() and operator[]().
*/
impl /*struct*/ QByteArray {
  pub fn data_0<RetType, T: QByteArray_data_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.data_0(self);
    // return 1;
  }
}
pub trait QByteArray_data_0<RetType> {
  fn data_0(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_data_0<usize> for () {
  fn data_0(self , rsthis: & QByteArray) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QByteArray4dataEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:203
// index:1
// Public inline Visibility=Default Availability=Available
// [8] const char * data() const

/*
Returns a pointer to the data stored in the byte array. The pointer can be used to access and modify the bytes that compose the array. The data is '\0'-terminated, i.e. the number of bytes in the returned character string is size() + 1 for the '\0' terminator.

Example:


  QByteArray ba("Hello world");
  char *data = ba.data();
  while (*data) {
      cout << "[" << *data << "]" << endl;
      ++data;
  }



The pointer remains valid as long as the byte array isn't reallocated or destroyed. For read-only access, constData() is faster because it never causes a deep copy to occur.

This function is mostly useful to pass a byte array to a function that accepts a const char *.

The following example makes a copy of the char* returned by data(), but it will corrupt the heap and cause a crash because it does not allocate a byte for the '\0' at the end:


  QString tmp = "test";
  QByteArray text = tmp.toLocal8Bit();
  char *data = new char[text.size()];
  strcpy(data, text.data());
  delete [] data;



This one allocates the correct amount of space:


  QString tmp = "test";
  QByteArray text = tmp.toLocal8Bit();
  char *data = new char[text.size() + 1];
  strcpy(data, text.data());
  delete [] data;



Note: A QByteArray can store any byte values including '\0's, but most functions that take char * arguments assume that the data ends at the first '\0' they encounter.

See also constData() and operator[]().
*/
impl /*struct*/ QByteArray {
  pub fn data_1<RetType, T: QByteArray_data_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.data_1(self);
    // return 1;
  }
}
pub trait QByteArray_data_1<RetType> {
  fn data_1(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_data_1<usize> for () {
  fn data_1(self , rsthis: & QByteArray) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QByteArray4dataEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:204
// index:0
// Public inline Visibility=Default Availability=Available
// [8] const char * constData() const

/*
Returns a pointer to the data stored in the byte array. The pointer can be used to access the bytes that compose the array. The data is '\0'-terminated unless the QByteArray object was created from raw data. The pointer remains valid as long as the byte array isn't reallocated or destroyed.

This function is mostly useful to pass a byte array to a function that accepts a const char *.

Note: A QByteArray can store any byte values including '\0's, but most functions that take char * arguments assume that the data ends at the first '\0' they encounter.

See also data(), operator[](), and fromRawData().
*/
impl /*struct*/ QByteArray {
  pub fn constData_0<RetType, T: QByteArray_constData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.constData_0(self);
    // return 1;
  }
}
pub trait QByteArray_constData_0<RetType> {
  fn constData_0(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_constData_0<usize> for () {
  fn constData_0(self , rsthis: & QByteArray) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QByteArray9constDataEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:205
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void detach()

/*

*/
impl /*struct*/ QByteArray {
  pub fn detach_0<RetType, T: QByteArray_detach_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.detach_0(self);
    // return 1;
  }
}
pub trait QByteArray_detach_0<RetType> {
  fn detach_0(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_detach_0<(/*void*/)> for () {
  fn detach_0(self , rsthis: & QByteArray) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN10QByteArray6detachEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:206
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isDetached() const

/*

*/
impl /*struct*/ QByteArray {
  pub fn isDetached_0<RetType, T: QByteArray_isDetached_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isDetached_0(self);
    // return 1;
  }
}
pub trait QByteArray_isDetached_0<RetType> {
  fn isDetached_0(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_isDetached_0<bool> for () {
  fn isDetached_0(self , rsthis: & QByteArray) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QByteArray10isDetachedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:207
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isSharedWith(const QByteArray &) const

/*

*/
impl /*struct*/ QByteArray {
  pub fn isSharedWith_0<RetType, T: QByteArray_isSharedWith_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isSharedWith_0(self);
    // return 1;
  }
}
pub trait QByteArray_isSharedWith_0<RetType> {
  fn isSharedWith_0(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_isSharedWith_0<bool> for (usize) {
  fn isSharedWith_0(self , rsthis: & QByteArray) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QByteArray12isSharedWithERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:208
// index:0
// Public Visibility=Default Availability=Available
// [-2] void clear()

/*
Clears the contents of the byte array and makes it null.

See also resize() and isNull().
*/
impl /*struct*/ QByteArray {
  pub fn clear_0<RetType, T: QByteArray_clear_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.clear_0(self);
    // return 1;
  }
}
pub trait QByteArray_clear_0<RetType> {
  fn clear_0(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_clear_0<(/*void*/)> for () {
  fn clear_0(self , rsthis: & QByteArray) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN10QByteArray5clearEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:210
// index:0
// Public inline Visibility=Default Availability=Available
// [1] char at(int) const

/*
Returns the character at index position i in the byte array.

i must be a valid index position in the byte array (i.e., 0 <= i < size()).

See also operator[]().
*/
impl /*struct*/ QByteArray {
  pub fn at_0<RetType, T: QByteArray_at_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.at_0(self);
    // return 1;
  }
}
pub trait QByteArray_at_0<RetType> {
  fn at_0(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_at_0<i8> for (i32) {
  fn at_0(self , rsthis: & QByteArray) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QByteArray2atEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i8 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:211
// index:0
// Public inline Visibility=Default Availability=Available
// [1] char operator[](int) const

/*

*/
impl /*struct*/ QByteArray {
  pub fn operator_get_index_0<RetType, T: QByteArray_operator_get_index_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_get_index_0(self);
    // return 1;
  }
}
pub trait QByteArray_operator_get_index_0<RetType> {
  fn operator_get_index_0(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_operator_get_index_0<i8> for (i32) {
  fn operator_get_index_0(self , rsthis: & QByteArray) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QByteArrayixEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i8 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:212
// index:1
// Public inline Visibility=Default Availability=Available
// [1] char operator[](uint) const

/*

*/
impl /*struct*/ QByteArray {
  pub fn operator_get_index_1<RetType, T: QByteArray_operator_get_index_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_get_index_1(self);
    // return 1;
  }
}
pub trait QByteArray_operator_get_index_1<RetType> {
  fn operator_get_index_1(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_operator_get_index_1<i8> for (u32) {
  fn operator_get_index_1(self , rsthis: & QByteArray) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const u32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QByteArrayixEj", 1,qtrt::FFITY_UINT32,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i8 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:213
// index:2
// Public inline Visibility=Default Availability=Available
// [16] QByteRef operator[](int)

/*

*/
impl /*struct*/ QByteArray {
  pub fn operator_get_index_2<RetType, T: QByteArray_operator_get_index_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_get_index_2(self);
    // return 1;
  }
}
pub trait QByteArray_operator_get_index_2<RetType> {
  fn operator_get_index_2(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_operator_get_index_2<usize> for (i32) {
  fn operator_get_index_2(self , rsthis: & QByteArray) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QByteArrayixEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:214
// index:3
// Public inline Visibility=Default Availability=Available
// [16] QByteRef operator[](uint)

/*

*/
impl /*struct*/ QByteArray {
  pub fn operator_get_index_3<RetType, T: QByteArray_operator_get_index_3<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_get_index_3(self);
    // return 1;
  }
}
pub trait QByteArray_operator_get_index_3<RetType> {
  fn operator_get_index_3(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_operator_get_index_3<usize> for (u32) {
  fn operator_get_index_3(self , rsthis: & QByteArray) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const u32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QByteArrayixEj", 1,qtrt::FFITY_UINT32,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:215
// index:0
// Public inline Visibility=Default Availability=Available
// [1] char front() const

/*
Returns the first character in the byte array. Same as at(0).

This function is provided for STL compatibility.

Warning: Calling this function on an empty byte array constitutes undefined behavior.

This function was introduced in  Qt 5.10.

See also back(), at(), and operator[]().
*/
impl /*struct*/ QByteArray {
  pub fn front_0<RetType, T: QByteArray_front_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.front_0(self);
    // return 1;
  }
}
pub trait QByteArray_front_0<RetType> {
  fn front_0(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_front_0<i8> for () {
  fn front_0(self , rsthis: & QByteArray) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QByteArray5frontEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i8 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:216
// index:1
// Public inline Visibility=Default Availability=Available
// [16] QByteRef front()

/*
Returns the first character in the byte array. Same as at(0).

This function is provided for STL compatibility.

Warning: Calling this function on an empty byte array constitutes undefined behavior.

This function was introduced in  Qt 5.10.

See also back(), at(), and operator[]().
*/
impl /*struct*/ QByteArray {
  pub fn front_1<RetType, T: QByteArray_front_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.front_1(self);
    // return 1;
  }
}
pub trait QByteArray_front_1<RetType> {
  fn front_1(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_front_1<usize> for () {
  fn front_1(self , rsthis: & QByteArray) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QByteArray5frontEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:217
// index:0
// Public inline Visibility=Default Availability=Available
// [1] char back() const

/*
Returns the last character in the byte array. Same as at(size() - 1).

This function is provided for STL compatibility.

Warning: Calling this function on an empty byte array constitutes undefined behavior.

This function was introduced in  Qt 5.10.

See also front(), at(), and operator[]().
*/
impl /*struct*/ QByteArray {
  pub fn back_0<RetType, T: QByteArray_back_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.back_0(self);
    // return 1;
  }
}
pub trait QByteArray_back_0<RetType> {
  fn back_0(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_back_0<i8> for () {
  fn back_0(self , rsthis: & QByteArray) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QByteArray4backEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i8 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:218
// index:1
// Public inline Visibility=Default Availability=Available
// [16] QByteRef back()

/*
Returns the last character in the byte array. Same as at(size() - 1).

This function is provided for STL compatibility.

Warning: Calling this function on an empty byte array constitutes undefined behavior.

This function was introduced in  Qt 5.10.

See also front(), at(), and operator[]().
*/
impl /*struct*/ QByteArray {
  pub fn back_1<RetType, T: QByteArray_back_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.back_1(self);
    // return 1;
  }
}
pub trait QByteArray_back_1<RetType> {
  fn back_1(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_back_1<usize> for () {
  fn back_1(self , rsthis: & QByteArray) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QByteArray4backEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:220
// index:0
// Public Visibility=Default Availability=Available
// [4] int indexOf(char, int) const

/*
Returns the index position of the first occurrence of the byte array ba in this byte array, searching forward from index position from. Returns -1 if ba could not be found.

Example:


  QByteArray x("sticky question");
  QByteArray y("sti");
  x.indexOf(y);               // returns 0
  x.indexOf(y, 1);            // returns 10
  x.indexOf(y, 10);           // returns 10
  x.indexOf(y, 11);           // returns -1



See also lastIndexOf(), contains(), and count().
*/
impl /*struct*/ QByteArray {
  pub fn indexOf_0<RetType, T: QByteArray_indexOf_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.indexOf_0(self);
    // return 1;
  }
}
pub trait QByteArray_indexOf_0<RetType> {
  fn indexOf_0(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_indexOf_0<i32> for (i8,i32) {
  fn indexOf_0(self , rsthis: & QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i8 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QByteArray7indexOfEci", 2,qtrt::FFITY_SINT8,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:221
// index:1
// Public Visibility=Default Availability=Available
// [4] int indexOf(const char *, int) const

/*
Returns the index position of the first occurrence of the byte array ba in this byte array, searching forward from index position from. Returns -1 if ba could not be found.

Example:


  QByteArray x("sticky question");
  QByteArray y("sti");
  x.indexOf(y);               // returns 0
  x.indexOf(y, 1);            // returns 10
  x.indexOf(y, 10);           // returns 10
  x.indexOf(y, 11);           // returns -1



See also lastIndexOf(), contains(), and count().
*/
impl /*struct*/ QByteArray {
  pub fn indexOf_1<RetType, T: QByteArray_indexOf_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.indexOf_1(self);
    // return 1;
  }
}
pub trait QByteArray_indexOf_1<RetType> {
  fn indexOf_1(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_indexOf_1<i32> for (usize,i32) {
  fn indexOf_1(self , rsthis: & QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QByteArray7indexOfEPKci", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:222
// index:2
// Public Visibility=Default Availability=Available
// [4] int indexOf(const QByteArray &, int) const

/*
Returns the index position of the first occurrence of the byte array ba in this byte array, searching forward from index position from. Returns -1 if ba could not be found.

Example:


  QByteArray x("sticky question");
  QByteArray y("sti");
  x.indexOf(y);               // returns 0
  x.indexOf(y, 1);            // returns 10
  x.indexOf(y, 10);           // returns 10
  x.indexOf(y, 11);           // returns -1



See also lastIndexOf(), contains(), and count().
*/
impl /*struct*/ QByteArray {
  pub fn indexOf_2<RetType, T: QByteArray_indexOf_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.indexOf_2(self);
    // return 1;
  }
}
pub trait QByteArray_indexOf_2<RetType> {
  fn indexOf_2(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_indexOf_2<i32> for (usize,i32) {
  fn indexOf_2(self , rsthis: & QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QByteArray7indexOfERKS_i", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:331
// index:3
// Public Visibility=Default Availability=Available
// [4] int indexOf(const QString &, int) const

/*
Returns the index position of the first occurrence of the byte array ba in this byte array, searching forward from index position from. Returns -1 if ba could not be found.

Example:


  QByteArray x("sticky question");
  QByteArray y("sti");
  x.indexOf(y);               // returns 0
  x.indexOf(y, 1);            // returns 10
  x.indexOf(y, 10);           // returns 10
  x.indexOf(y, 11);           // returns -1



See also lastIndexOf(), contains(), and count().
*/
impl /*struct*/ QByteArray {
  pub fn indexOf_3<RetType, T: QByteArray_indexOf_3<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.indexOf_3(self);
    // return 1;
  }
}
pub trait QByteArray_indexOf_3<RetType> {
  fn indexOf_3(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_indexOf_3<i32> for (usize,i32) {
  fn indexOf_3(self , rsthis: & QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QByteArray7indexOfERK7QStringi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:223
// index:0
// Public Visibility=Default Availability=Available
// [4] int lastIndexOf(char, int) const

/*
Returns the index position of the last occurrence of the byte array ba in this byte array, searching backward from index position from. If from is -1 (the default), the search starts at the last byte. Returns -1 if ba could not be found.

Example:


  QByteArray x("crazy azimuths");
  QByteArray y("az");
  x.lastIndexOf(y);           // returns 6
  x.lastIndexOf(y, 6);        // returns 6
  x.lastIndexOf(y, 5);        // returns 2
  x.lastIndexOf(y, 1);        // returns -1



See also indexOf(), contains(), and count().
*/
impl /*struct*/ QByteArray {
  pub fn lastIndexOf_0<RetType, T: QByteArray_lastIndexOf_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.lastIndexOf_0(self);
    // return 1;
  }
}
pub trait QByteArray_lastIndexOf_0<RetType> {
  fn lastIndexOf_0(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_lastIndexOf_0<i32> for (i8,i32) {
  fn lastIndexOf_0(self , rsthis: & QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i8 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QByteArray11lastIndexOfEci", 2,qtrt::FFITY_SINT8,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:224
// index:1
// Public Visibility=Default Availability=Available
// [4] int lastIndexOf(const char *, int) const

/*
Returns the index position of the last occurrence of the byte array ba in this byte array, searching backward from index position from. If from is -1 (the default), the search starts at the last byte. Returns -1 if ba could not be found.

Example:


  QByteArray x("crazy azimuths");
  QByteArray y("az");
  x.lastIndexOf(y);           // returns 6
  x.lastIndexOf(y, 6);        // returns 6
  x.lastIndexOf(y, 5);        // returns 2
  x.lastIndexOf(y, 1);        // returns -1



See also indexOf(), contains(), and count().
*/
impl /*struct*/ QByteArray {
  pub fn lastIndexOf_1<RetType, T: QByteArray_lastIndexOf_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.lastIndexOf_1(self);
    // return 1;
  }
}
pub trait QByteArray_lastIndexOf_1<RetType> {
  fn lastIndexOf_1(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_lastIndexOf_1<i32> for (usize,i32) {
  fn lastIndexOf_1(self , rsthis: & QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QByteArray11lastIndexOfEPKci", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:225
// index:2
// Public Visibility=Default Availability=Available
// [4] int lastIndexOf(const QByteArray &, int) const

/*
Returns the index position of the last occurrence of the byte array ba in this byte array, searching backward from index position from. If from is -1 (the default), the search starts at the last byte. Returns -1 if ba could not be found.

Example:


  QByteArray x("crazy azimuths");
  QByteArray y("az");
  x.lastIndexOf(y);           // returns 6
  x.lastIndexOf(y, 6);        // returns 6
  x.lastIndexOf(y, 5);        // returns 2
  x.lastIndexOf(y, 1);        // returns -1



See also indexOf(), contains(), and count().
*/
impl /*struct*/ QByteArray {
  pub fn lastIndexOf_2<RetType, T: QByteArray_lastIndexOf_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.lastIndexOf_2(self);
    // return 1;
  }
}
pub trait QByteArray_lastIndexOf_2<RetType> {
  fn lastIndexOf_2(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_lastIndexOf_2<i32> for (usize,i32) {
  fn lastIndexOf_2(self , rsthis: & QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QByteArray11lastIndexOfERKS_i", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:332
// index:3
// Public Visibility=Default Availability=Available
// [4] int lastIndexOf(const QString &, int) const

/*
Returns the index position of the last occurrence of the byte array ba in this byte array, searching backward from index position from. If from is -1 (the default), the search starts at the last byte. Returns -1 if ba could not be found.

Example:


  QByteArray x("crazy azimuths");
  QByteArray y("az");
  x.lastIndexOf(y);           // returns 6
  x.lastIndexOf(y, 6);        // returns 6
  x.lastIndexOf(y, 5);        // returns 2
  x.lastIndexOf(y, 1);        // returns -1



See also indexOf(), contains(), and count().
*/
impl /*struct*/ QByteArray {
  pub fn lastIndexOf_3<RetType, T: QByteArray_lastIndexOf_3<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.lastIndexOf_3(self);
    // return 1;
  }
}
pub trait QByteArray_lastIndexOf_3<RetType> {
  fn lastIndexOf_3(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_lastIndexOf_3<i32> for (usize,i32) {
  fn lastIndexOf_3(self , rsthis: & QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QByteArray11lastIndexOfERK7QStringi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:227
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool contains(char) const

/*
Returns true if the byte array contains an occurrence of the byte array ba; otherwise returns false.

See also indexOf() and count().
*/
impl /*struct*/ QByteArray {
  pub fn contains_0<RetType, T: QByteArray_contains_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.contains_0(self);
    // return 1;
  }
}
pub trait QByteArray_contains_0<RetType> {
  fn contains_0(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_contains_0<bool> for (i8) {
  fn contains_0(self , rsthis: & QByteArray) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i8 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QByteArray8containsEc", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:228
// index:1
// Public inline Visibility=Default Availability=Available
// [1] bool contains(const char *) const

/*
Returns true if the byte array contains an occurrence of the byte array ba; otherwise returns false.

See also indexOf() and count().
*/
impl /*struct*/ QByteArray {
  pub fn contains_1<RetType, T: QByteArray_contains_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.contains_1(self);
    // return 1;
  }
}
pub trait QByteArray_contains_1<RetType> {
  fn contains_1(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_contains_1<bool> for (usize) {
  fn contains_1(self , rsthis: & QByteArray) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QByteArray8containsEPKc", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:229
// index:2
// Public inline Visibility=Default Availability=Available
// [1] bool contains(const QByteArray &) const

/*
Returns true if the byte array contains an occurrence of the byte array ba; otherwise returns false.

See also indexOf() and count().
*/
impl /*struct*/ QByteArray {
  pub fn contains_2<RetType, T: QByteArray_contains_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.contains_2(self);
    // return 1;
  }
}
pub trait QByteArray_contains_2<RetType> {
  fn contains_2(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_contains_2<bool> for (usize) {
  fn contains_2(self , rsthis: & QByteArray) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QByteArray8containsERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:230
// index:0
// Public Visibility=Default Availability=Available
// [4] int count(char) const

/*
Returns the number of (potentially overlapping) occurrences of byte array ba in this byte array.

See also contains() and indexOf().
*/
impl /*struct*/ QByteArray {
  pub fn count_0<RetType, T: QByteArray_count_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.count_0(self);
    // return 1;
  }
}
pub trait QByteArray_count_0<RetType> {
  fn count_0(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_count_0<i32> for (i8) {
  fn count_0(self , rsthis: & QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i8 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QByteArray5countEc", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:231
// index:1
// Public Visibility=Default Availability=Available
// [4] int count(const char *) const

/*
Returns the number of (potentially overlapping) occurrences of byte array ba in this byte array.

See also contains() and indexOf().
*/
impl /*struct*/ QByteArray {
  pub fn count_1<RetType, T: QByteArray_count_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.count_1(self);
    // return 1;
  }
}
pub trait QByteArray_count_1<RetType> {
  fn count_1(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_count_1<i32> for (usize) {
  fn count_1(self , rsthis: & QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QByteArray5countEPKc", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:232
// index:2
// Public Visibility=Default Availability=Available
// [4] int count(const QByteArray &) const

/*
Returns the number of (potentially overlapping) occurrences of byte array ba in this byte array.

See also contains() and indexOf().
*/
impl /*struct*/ QByteArray {
  pub fn count_2<RetType, T: QByteArray_count_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.count_2(self);
    // return 1;
  }
}
pub trait QByteArray_count_2<RetType> {
  fn count_2(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_count_2<i32> for (usize) {
  fn count_2(self , rsthis: & QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QByteArray5countERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:433
// index:3
// Public inline Visibility=Default Availability=Available
// [4] int count() const

/*
Returns the number of (potentially overlapping) occurrences of byte array ba in this byte array.

See also contains() and indexOf().
*/
impl /*struct*/ QByteArray {
  pub fn count_3<RetType, T: QByteArray_count_3<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.count_3(self);
    // return 1;
  }
}
pub trait QByteArray_count_3<RetType> {
  fn count_3(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_count_3<i32> for () {
  fn count_3(self , rsthis: & QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QByteArray5countEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:234
// index:0
// Public Visibility=Default Availability=Available
// [8] QByteArray left(int) const

/*
Returns a byte array that contains the leftmost len bytes of this byte array.

The entire byte array is returned if len is greater than size().

Example:


  QByteArray x("Pineapple");
  QByteArray y = x.left(4);
  // y == "Pine"



See also startsWith(), right(), mid(), chopped(), chop(), and truncate().
*/
impl /*struct*/ QByteArray {
  pub fn left_0<RetType, T: QByteArray_left_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.left_0(self);
    // return 1;
  }
}
pub trait QByteArray_left_0<RetType> {
  fn left_0(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_left_0<usize> for (i32) {
  fn left_0(self , rsthis: & QByteArray) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QByteArray4leftEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:235
// index:0
// Public Visibility=Default Availability=Available
// [8] QByteArray right(int) const

/*
Returns a byte array that contains the rightmost len bytes of this byte array.

The entire byte array is returned if len is greater than size().

Example:


  QByteArray x("Pineapple");
  QByteArray y = x.right(5);
  // y == "apple"



See also endsWith(), left(), mid(), chopped(), chop(), and truncate().
*/
impl /*struct*/ QByteArray {
  pub fn right_0<RetType, T: QByteArray_right_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.right_0(self);
    // return 1;
  }
}
pub trait QByteArray_right_0<RetType> {
  fn right_0(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_right_0<usize> for (i32) {
  fn right_0(self , rsthis: & QByteArray) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QByteArray5rightEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:236
// index:0
// Public Visibility=Default Availability=Available
// [8] QByteArray mid(int, int) const

/*
Returns a byte array containing len bytes from this byte array, starting at position pos.

If len is -1 (the default), or pos + len >= size(), returns a byte array containing all bytes starting at position pos until the end of the byte array.

Example:


  QByteArray x("Five pineapples");
  QByteArray y = x.mid(5, 4);     // y == "pine"
  QByteArray z = x.mid(5);        // z == "pineapples"



See also left(), right(), chopped(), chop(), and truncate().
*/
impl /*struct*/ QByteArray {
  pub fn mid_0<RetType, T: QByteArray_mid_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mid_0(self);
    // return 1;
  }
}
pub trait QByteArray_mid_0<RetType> {
  fn mid_0(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_mid_0<usize> for (i32,i32) {
  fn mid_0(self , rsthis: & QByteArray) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QByteArray3midEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:237
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QByteArray chopped(int) const

/*
Returns a byte array that contains the leftmost size() - len bytes of this byte array.

Note: The behavior is undefined if len is negative or greater than size().

This function was introduced in  Qt 5.10.

See also endsWith(), left(), right(), mid(), chop(), and truncate().
*/
impl /*struct*/ QByteArray {
  pub fn chopped_0<RetType, T: QByteArray_chopped_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.chopped_0(self);
    // return 1;
  }
}
pub trait QByteArray_chopped_0<RetType> {
  fn chopped_0(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_chopped_0<usize> for (i32) {
  fn chopped_0(self , rsthis: & QByteArray) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QByteArray7choppedEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:240
// index:0
// Public Visibility=Default Availability=Available
// [1] bool startsWith(const QByteArray &) const

/*
Returns true if this byte array starts with byte array ba; otherwise returns false.

Example:


  QByteArray url("ftp://ftp.qt-project.org/");
  if (url.startsWith("ftp:"))
      ...



See also endsWith() and left().
*/
impl /*struct*/ QByteArray {
  pub fn startsWith_0<RetType, T: QByteArray_startsWith_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.startsWith_0(self);
    // return 1;
  }
}
pub trait QByteArray_startsWith_0<RetType> {
  fn startsWith_0(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_startsWith_0<bool> for (usize) {
  fn startsWith_0(self , rsthis: & QByteArray) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QByteArray10startsWithERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:241
// index:1
// Public Visibility=Default Availability=Available
// [1] bool startsWith(char) const

/*
Returns true if this byte array starts with byte array ba; otherwise returns false.

Example:


  QByteArray url("ftp://ftp.qt-project.org/");
  if (url.startsWith("ftp:"))
      ...



See also endsWith() and left().
*/
impl /*struct*/ QByteArray {
  pub fn startsWith_1<RetType, T: QByteArray_startsWith_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.startsWith_1(self);
    // return 1;
  }
}
pub trait QByteArray_startsWith_1<RetType> {
  fn startsWith_1(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_startsWith_1<bool> for (i8) {
  fn startsWith_1(self , rsthis: & QByteArray) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i8 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QByteArray10startsWithEc", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:242
// index:2
// Public Visibility=Default Availability=Available
// [1] bool startsWith(const char *) const

/*
Returns true if this byte array starts with byte array ba; otherwise returns false.

Example:


  QByteArray url("ftp://ftp.qt-project.org/");
  if (url.startsWith("ftp:"))
      ...



See also endsWith() and left().
*/
impl /*struct*/ QByteArray {
  pub fn startsWith_2<RetType, T: QByteArray_startsWith_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.startsWith_2(self);
    // return 1;
  }
}
pub trait QByteArray_startsWith_2<RetType> {
  fn startsWith_2(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_startsWith_2<bool> for (usize) {
  fn startsWith_2(self , rsthis: & QByteArray) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QByteArray10startsWithEPKc", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:244
// index:0
// Public Visibility=Default Availability=Available
// [1] bool endsWith(const QByteArray &) const

/*
Returns true if this byte array ends with byte array ba; otherwise returns false.

Example:


  QByteArray url("http://qt-project.org/doc/qt-5.0/qtdoc/index.html");
  if (url.endsWith(".html"))
      ...



See also startsWith() and right().
*/
impl /*struct*/ QByteArray {
  pub fn endsWith_0<RetType, T: QByteArray_endsWith_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.endsWith_0(self);
    // return 1;
  }
}
pub trait QByteArray_endsWith_0<RetType> {
  fn endsWith_0(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_endsWith_0<bool> for (usize) {
  fn endsWith_0(self , rsthis: & QByteArray) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QByteArray8endsWithERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:245
// index:1
// Public Visibility=Default Availability=Available
// [1] bool endsWith(char) const

/*
Returns true if this byte array ends with byte array ba; otherwise returns false.

Example:


  QByteArray url("http://qt-project.org/doc/qt-5.0/qtdoc/index.html");
  if (url.endsWith(".html"))
      ...



See also startsWith() and right().
*/
impl /*struct*/ QByteArray {
  pub fn endsWith_1<RetType, T: QByteArray_endsWith_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.endsWith_1(self);
    // return 1;
  }
}
pub trait QByteArray_endsWith_1<RetType> {
  fn endsWith_1(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_endsWith_1<bool> for (i8) {
  fn endsWith_1(self , rsthis: & QByteArray) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i8 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QByteArray8endsWithEc", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:246
// index:2
// Public Visibility=Default Availability=Available
// [1] bool endsWith(const char *) const

/*
Returns true if this byte array ends with byte array ba; otherwise returns false.

Example:


  QByteArray url("http://qt-project.org/doc/qt-5.0/qtdoc/index.html");
  if (url.endsWith(".html"))
      ...



See also startsWith() and right().
*/
impl /*struct*/ QByteArray {
  pub fn endsWith_2<RetType, T: QByteArray_endsWith_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.endsWith_2(self);
    // return 1;
  }
}
pub trait QByteArray_endsWith_2<RetType> {
  fn endsWith_2(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_endsWith_2<bool> for (usize) {
  fn endsWith_2(self , rsthis: & QByteArray) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QByteArray8endsWithEPKc", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:248
// index:0
// Public Visibility=Default Availability=Available
// [-2] void truncate(int)

/*
Truncates the byte array at index position pos.

If pos is beyond the end of the array, nothing happens.

Example:


  QByteArray ba("Stockholm");
  ba.truncate(5);             // ba == "Stock"



See also chop(), resize(), and left().
*/
impl /*struct*/ QByteArray {
  pub fn truncate_0<RetType, T: QByteArray_truncate_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.truncate_0(self);
    // return 1;
  }
}
pub trait QByteArray_truncate_0<RetType> {
  fn truncate_0(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_truncate_0<(/*void*/)> for (i32) {
  fn truncate_0(self , rsthis: & QByteArray) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QByteArray8truncateEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:249
// index:0
// Public Visibility=Default Availability=Available
// [-2] void chop(int)

/*
Removes n bytes from the end of the byte array.

If n is greater than size(), the result is an empty byte array.

Example:


  QByteArray ba("STARTTLS\r\n");
  ba.chop(2);                 // ba == "STARTTLS"



See also truncate(), resize(), and left().
*/
impl /*struct*/ QByteArray {
  pub fn chop_0<RetType, T: QByteArray_chop_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.chop_0(self);
    // return 1;
  }
}
pub trait QByteArray_chop_0<RetType> {
  fn chop_0(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_chop_0<(/*void*/)> for (i32) {
  fn chop_0(self , rsthis: & QByteArray) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QByteArray4chopEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:259
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QByteArray toLower() const

/*
Returns a lowercase copy of the byte array. The bytearray is interpreted as a Latin-1 encoded string.

Example:


  QByteArray x("Qt by THE QT COMPANY");
  QByteArray y = x.toLower();
  // y == "qt by the qt company"



See also toUpper() and 8-bit Character Comparisons.
*/
impl /*struct*/ QByteArray {
  pub fn toLower_0<RetType, T: QByteArray_toLower_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toLower_0(self);
    // return 1;
  }
}
pub trait QByteArray_toLower_0<RetType> {
  fn toLower_0(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_toLower_0<usize> for () {
  fn toLower_0(self , rsthis: & QByteArray) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNKR10QByteArray7toLowerEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:261
// index:1
// Public inline Visibility=Default Availability=Available
// [8] QByteArray toLower()

/*
Returns a lowercase copy of the byte array. The bytearray is interpreted as a Latin-1 encoded string.

Example:


  QByteArray x("Qt by THE QT COMPANY");
  QByteArray y = x.toLower();
  // y == "qt by the qt company"



See also toUpper() and 8-bit Character Comparisons.
*/
impl /*struct*/ QByteArray {
  pub fn toLower_1<RetType, T: QByteArray_toLower_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toLower_1(self);
    // return 1;
  }
}
pub trait QByteArray_toLower_1<RetType> {
  fn toLower_1(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_toLower_1<usize> for () {
  fn toLower_1(self , rsthis: & QByteArray) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNO10QByteArray7toLowerEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:263
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QByteArray toUpper() const

/*
Returns an uppercase copy of the byte array. The bytearray is interpreted as a Latin-1 encoded string.

Example:


  QByteArray x("Qt by THE QT COMPANY");
  QByteArray y = x.toUpper();
  // y == "QT BY THE QT COMPANY"



See also toLower() and 8-bit Character Comparisons.
*/
impl /*struct*/ QByteArray {
  pub fn toUpper_0<RetType, T: QByteArray_toUpper_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toUpper_0(self);
    // return 1;
  }
}
pub trait QByteArray_toUpper_0<RetType> {
  fn toUpper_0(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_toUpper_0<usize> for () {
  fn toUpper_0(self , rsthis: & QByteArray) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNKR10QByteArray7toUpperEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:265
// index:1
// Public inline Visibility=Default Availability=Available
// [8] QByteArray toUpper()

/*
Returns an uppercase copy of the byte array. The bytearray is interpreted as a Latin-1 encoded string.

Example:


  QByteArray x("Qt by THE QT COMPANY");
  QByteArray y = x.toUpper();
  // y == "QT BY THE QT COMPANY"



See also toLower() and 8-bit Character Comparisons.
*/
impl /*struct*/ QByteArray {
  pub fn toUpper_1<RetType, T: QByteArray_toUpper_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toUpper_1(self);
    // return 1;
  }
}
pub trait QByteArray_toUpper_1<RetType> {
  fn toUpper_1(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_toUpper_1<usize> for () {
  fn toUpper_1(self , rsthis: & QByteArray) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNO10QByteArray7toUpperEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:267
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QByteArray trimmed() const

/*
Returns a byte array that has whitespace removed from the start and the end.

Whitespace means any character for which the standard C++ isspace() function returns true in the C locale. This includes the ASCII characters '\t', '\n', '\v', '\f', '\r', and ' '.

Example:


  QByteArray ba("  lots\t of\nwhitespace\r\n ");
  ba = ba.trimmed();
  // ba == "lots\t of\nwhitespace";



Unlike simplified(), trimmed() leaves internal whitespace alone.

See also simplified().
*/
impl /*struct*/ QByteArray {
  pub fn trimmed_0<RetType, T: QByteArray_trimmed_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.trimmed_0(self);
    // return 1;
  }
}
pub trait QByteArray_trimmed_0<RetType> {
  fn trimmed_0(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_trimmed_0<usize> for () {
  fn trimmed_0(self , rsthis: & QByteArray) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNKR10QByteArray7trimmedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:269
// index:1
// Public inline Visibility=Default Availability=Available
// [8] QByteArray trimmed()

/*
Returns a byte array that has whitespace removed from the start and the end.

Whitespace means any character for which the standard C++ isspace() function returns true in the C locale. This includes the ASCII characters '\t', '\n', '\v', '\f', '\r', and ' '.

Example:


  QByteArray ba("  lots\t of\nwhitespace\r\n ");
  ba = ba.trimmed();
  // ba == "lots\t of\nwhitespace";



Unlike simplified(), trimmed() leaves internal whitespace alone.

See also simplified().
*/
impl /*struct*/ QByteArray {
  pub fn trimmed_1<RetType, T: QByteArray_trimmed_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.trimmed_1(self);
    // return 1;
  }
}
pub trait QByteArray_trimmed_1<RetType> {
  fn trimmed_1(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_trimmed_1<usize> for () {
  fn trimmed_1(self , rsthis: & QByteArray) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNO10QByteArray7trimmedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:271
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QByteArray simplified() const

/*
Returns a byte array that has whitespace removed from the start and the end, and which has each sequence of internal whitespace replaced with a single space.

Whitespace means any character for which the standard C++ isspace() function returns true in the C locale. This includes the ASCII isspace() function returns true in the C locale. This includes the ASCII characters '\t', '\n', '\v', '\f', '\r', and ' '.

Example:


  QByteArray ba("  lots\t of\nwhitespace\r\n ");
  ba = ba.simplified();
  // ba == "lots of whitespace";



See also trimmed().
*/
impl /*struct*/ QByteArray {
  pub fn simplified_0<RetType, T: QByteArray_simplified_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.simplified_0(self);
    // return 1;
  }
}
pub trait QByteArray_simplified_0<RetType> {
  fn simplified_0(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_simplified_0<usize> for () {
  fn simplified_0(self , rsthis: & QByteArray) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNKR10QByteArray10simplifiedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:273
// index:1
// Public inline Visibility=Default Availability=Available
// [8] QByteArray simplified()

/*
Returns a byte array that has whitespace removed from the start and the end, and which has each sequence of internal whitespace replaced with a single space.

Whitespace means any character for which the standard C++ isspace() function returns true in the C locale. This includes the ASCII isspace() function returns true in the C locale. This includes the ASCII characters '\t', '\n', '\v', '\f', '\r', and ' '.

Example:


  QByteArray ba("  lots\t of\nwhitespace\r\n ");
  ba = ba.simplified();
  // ba == "lots of whitespace";



See also trimmed().
*/
impl /*struct*/ QByteArray {
  pub fn simplified_1<RetType, T: QByteArray_simplified_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.simplified_1(self);
    // return 1;
  }
}
pub trait QByteArray_simplified_1<RetType> {
  fn simplified_1(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_simplified_1<usize> for () {
  fn simplified_1(self , rsthis: & QByteArray) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNO10QByteArray10simplifiedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:285
// index:0
// Public Visibility=Default Availability=Available
// [8] QByteArray leftJustified(int, char, bool) const

/*
Returns a byte array of size width that contains this byte array padded by the fill character.

If truncate is false and the size() of the byte array is more than width, then the returned byte array is a copy of this byte array.

If truncate is true and the size() of the byte array is more than width, then any bytes in a copy of the byte array after position width are removed, and the copy is returned.

Example:


  QByteArray x("apple");
  QByteArray y = x.leftJustified(8, '.');   // y == "apple..."



See also rightJustified().
*/
impl /*struct*/ QByteArray {
  pub fn leftJustified_0<RetType, T: QByteArray_leftJustified_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.leftJustified_0(self);
    // return 1;
  }
}
pub trait QByteArray_leftJustified_0<RetType> {
  fn leftJustified_0(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_leftJustified_0<usize> for (i32,i8,bool) {
  fn leftJustified_0(self , rsthis: & QByteArray) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i8 as usize;
    let arg2 = (&self.2) as *const bool as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QByteArray13leftJustifiedEicb", 3,qtrt::FFITY_INT,qtrt::FFITY_SINT8,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:286
// index:0
// Public Visibility=Default Availability=Available
// [8] QByteArray rightJustified(int, char, bool) const

/*
Returns a byte array of size width that contains the fill character followed by this byte array.

If truncate is false and the size of the byte array is more than width, then the returned byte array is a copy of this byte array.

If truncate is true and the size of the byte array is more than width, then the resulting byte array is truncated at position width.

Example:


  QByteArray x("apple");
  QByteArray y = x.rightJustified(8, '.');    // y == "...apple"



See also leftJustified().
*/
impl /*struct*/ QByteArray {
  pub fn rightJustified_0<RetType, T: QByteArray_rightJustified_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rightJustified_0(self);
    // return 1;
  }
}
pub trait QByteArray_rightJustified_0<RetType> {
  fn rightJustified_0(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_rightJustified_0<usize> for (i32,i8,bool) {
  fn rightJustified_0(self , rsthis: & QByteArray) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i8 as usize;
    let arg2 = (&self.2) as *const bool as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QByteArray14rightJustifiedEicb", 3,qtrt::FFITY_INT,qtrt::FFITY_SINT8,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:303
// index:0
// Public Visibility=Default Availability=Available
// [8] QByteArray & remove(int, int)

/*
Removes len bytes from the array, starting at index position pos, and returns a reference to the array.

If pos is out of range, nothing happens. If pos is valid, but pos + len is larger than the size of the array, the array is truncated at position pos.

Example:


  QByteArray ba("Montreal");
  ba.remove(1, 4);
  // ba == "Meal"



See also insert() and replace().
*/
impl /*struct*/ QByteArray {
  pub fn remove_0<RetType, T: QByteArray_remove_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.remove_0(self);
    // return 1;
  }
}
pub trait QByteArray_remove_0<RetType> {
  fn remove_0(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_remove_0<usize> for (i32,i32) {
  fn remove_0(self , rsthis: & QByteArray) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QByteArray6removeEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:304
// index:0
// Public Visibility=Default Availability=Available
// [8] QByteArray & replace(int, int, const char *)

/*
Replaces len bytes from index position pos with the byte array after, and returns a reference to this byte array.

Example:


  QByteArray x("Say yes!");
  QByteArray y("no");
  x.replace(4, 3, y);
  // x == "Say no!"



See also insert() and remove().
*/
impl /*struct*/ QByteArray {
  pub fn replace_0<RetType, T: QByteArray_replace_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.replace_0(self);
    // return 1;
  }
}
pub trait QByteArray_replace_0<RetType> {
  fn replace_0(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_replace_0<usize> for (i32,i32,usize) {
  fn replace_0(self , rsthis: & QByteArray) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (self.2) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QByteArray7replaceEiiPKc", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:305
// index:1
// Public Visibility=Default Availability=Available
// [8] QByteArray & replace(int, int, const char *, int)

/*
Replaces len bytes from index position pos with the byte array after, and returns a reference to this byte array.

Example:


  QByteArray x("Say yes!");
  QByteArray y("no");
  x.replace(4, 3, y);
  // x == "Say no!"



See also insert() and remove().
*/
impl /*struct*/ QByteArray {
  pub fn replace_1<RetType, T: QByteArray_replace_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.replace_1(self);
    // return 1;
  }
}
pub trait QByteArray_replace_1<RetType> {
  fn replace_1(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_replace_1<usize> for (i32,i32,usize,i32) {
  fn replace_1(self , rsthis: & QByteArray) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (self.2) as *const usize as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QByteArray7replaceEiiPKci", 4,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:306
// index:2
// Public Visibility=Default Availability=Available
// [8] QByteArray & replace(int, int, const QByteArray &)

/*
Replaces len bytes from index position pos with the byte array after, and returns a reference to this byte array.

Example:


  QByteArray x("Say yes!");
  QByteArray y("no");
  x.replace(4, 3, y);
  // x == "Say no!"



See also insert() and remove().
*/
impl /*struct*/ QByteArray {
  pub fn replace_2<RetType, T: QByteArray_replace_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.replace_2(self);
    // return 1;
  }
}
pub trait QByteArray_replace_2<RetType> {
  fn replace_2(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_replace_2<usize> for (i32,i32,usize) {
  fn replace_2(self , rsthis: & QByteArray) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QByteArray7replaceEiiRKS_", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:307
// index:3
// Public inline Visibility=Default Availability=Available
// [8] QByteArray & replace(char, const char *)

/*
Replaces len bytes from index position pos with the byte array after, and returns a reference to this byte array.

Example:


  QByteArray x("Say yes!");
  QByteArray y("no");
  x.replace(4, 3, y);
  // x == "Say no!"



See also insert() and remove().
*/
impl /*struct*/ QByteArray {
  pub fn replace_3<RetType, T: QByteArray_replace_3<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.replace_3(self);
    // return 1;
  }
}
pub trait QByteArray_replace_3<RetType> {
  fn replace_3(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_replace_3<usize> for (i8,usize) {
  fn replace_3(self , rsthis: & QByteArray) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i8 as usize;
    let arg1 = (self.1) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QByteArray7replaceEcPKc", 2,qtrt::FFITY_SINT8,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:308
// index:4
// Public Visibility=Default Availability=Available
// [8] QByteArray & replace(char, const QByteArray &)

/*
Replaces len bytes from index position pos with the byte array after, and returns a reference to this byte array.

Example:


  QByteArray x("Say yes!");
  QByteArray y("no");
  x.replace(4, 3, y);
  // x == "Say no!"



See also insert() and remove().
*/
impl /*struct*/ QByteArray {
  pub fn replace_4<RetType, T: QByteArray_replace_4<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.replace_4(self);
    // return 1;
  }
}
pub trait QByteArray_replace_4<RetType> {
  fn replace_4(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_replace_4<usize> for (i8,usize) {
  fn replace_4(self , rsthis: & QByteArray) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i8 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QByteArray7replaceEcRKS_", 2,qtrt::FFITY_SINT8,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:309
// index:5
// Public inline Visibility=Default Availability=Available
// [8] QByteArray & replace(const char *, const char *)

/*
Replaces len bytes from index position pos with the byte array after, and returns a reference to this byte array.

Example:


  QByteArray x("Say yes!");
  QByteArray y("no");
  x.replace(4, 3, y);
  // x == "Say no!"



See also insert() and remove().
*/
impl /*struct*/ QByteArray {
  pub fn replace_5<RetType, T: QByteArray_replace_5<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.replace_5(self);
    // return 1;
  }
}
pub trait QByteArray_replace_5<RetType> {
  fn replace_5(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_replace_5<usize> for (usize,usize) {
  fn replace_5(self , rsthis: & QByteArray) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self.0) as *const usize as usize;
    let arg1 = (self.1) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QByteArray7replaceEPKcS1_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:310
// index:6
// Public Visibility=Default Availability=Available
// [8] QByteArray & replace(const char *, int, const char *, int)

/*
Replaces len bytes from index position pos with the byte array after, and returns a reference to this byte array.

Example:


  QByteArray x("Say yes!");
  QByteArray y("no");
  x.replace(4, 3, y);
  // x == "Say no!"



See also insert() and remove().
*/
impl /*struct*/ QByteArray {
  pub fn replace_6<RetType, T: QByteArray_replace_6<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.replace_6(self);
    // return 1;
  }
}
pub trait QByteArray_replace_6<RetType> {
  fn replace_6(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_replace_6<usize> for (usize,i32,usize,i32) {
  fn replace_6(self , rsthis: & QByteArray) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (self.2) as *const usize as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QByteArray7replaceEPKciS1_i", 4,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:311
// index:7
// Public Visibility=Default Availability=Available
// [8] QByteArray & replace(const QByteArray &, const QByteArray &)

/*
Replaces len bytes from index position pos with the byte array after, and returns a reference to this byte array.

Example:


  QByteArray x("Say yes!");
  QByteArray y("no");
  x.replace(4, 3, y);
  // x == "Say no!"



See also insert() and remove().
*/
impl /*struct*/ QByteArray {
  pub fn replace_7<RetType, T: QByteArray_replace_7<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.replace_7(self);
    // return 1;
  }
}
pub trait QByteArray_replace_7<RetType> {
  fn replace_7(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_replace_7<usize> for (usize,usize) {
  fn replace_7(self , rsthis: & QByteArray) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QByteArray7replaceERKS_S1_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:312
// index:8
// Public inline Visibility=Default Availability=Available
// [8] QByteArray & replace(const QByteArray &, const char *)

/*
Replaces len bytes from index position pos with the byte array after, and returns a reference to this byte array.

Example:


  QByteArray x("Say yes!");
  QByteArray y("no");
  x.replace(4, 3, y);
  // x == "Say no!"



See also insert() and remove().
*/
impl /*struct*/ QByteArray {
  pub fn replace_8<RetType, T: QByteArray_replace_8<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.replace_8(self);
    // return 1;
  }
}
pub trait QByteArray_replace_8<RetType> {
  fn replace_8(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_replace_8<usize> for (usize,usize) {
  fn replace_8(self , rsthis: & QByteArray) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (self.1) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QByteArray7replaceERKS_PKc", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:313
// index:9
// Public Visibility=Default Availability=Available
// [8] QByteArray & replace(const char *, const QByteArray &)

/*
Replaces len bytes from index position pos with the byte array after, and returns a reference to this byte array.

Example:


  QByteArray x("Say yes!");
  QByteArray y("no");
  x.replace(4, 3, y);
  // x == "Say no!"



See also insert() and remove().
*/
impl /*struct*/ QByteArray {
  pub fn replace_9<RetType, T: QByteArray_replace_9<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.replace_9(self);
    // return 1;
  }
}
pub trait QByteArray_replace_9<RetType> {
  fn replace_9(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_replace_9<usize> for (usize,usize) {
  fn replace_9(self , rsthis: & QByteArray) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self.0) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QByteArray7replaceEPKcRKS_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:314
// index:10
// Public Visibility=Default Availability=Available
// [8] QByteArray & replace(char, char)

/*
Replaces len bytes from index position pos with the byte array after, and returns a reference to this byte array.

Example:


  QByteArray x("Say yes!");
  QByteArray y("no");
  x.replace(4, 3, y);
  // x == "Say no!"



See also insert() and remove().
*/
impl /*struct*/ QByteArray {
  pub fn replace_10<RetType, T: QByteArray_replace_10<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.replace_10(self);
    // return 1;
  }
}
pub trait QByteArray_replace_10<RetType> {
  fn replace_10(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_replace_10<usize> for (i8,i8) {
  fn replace_10(self , rsthis: & QByteArray) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i8 as usize;
    let arg1 = (&self.1) as *const i8 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QByteArray7replaceEcc", 2,qtrt::FFITY_SINT8,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:326
// index:11
// Public Visibility=Default Availability=Available
// [8] QByteArray & replace(const QString &, const char *)

/*
Replaces len bytes from index position pos with the byte array after, and returns a reference to this byte array.

Example:


  QByteArray x("Say yes!");
  QByteArray y("no");
  x.replace(4, 3, y);
  // x == "Say no!"



See also insert() and remove().
*/
impl /*struct*/ QByteArray {
  pub fn replace_11<RetType, T: QByteArray_replace_11<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.replace_11(self);
    // return 1;
  }
}
pub trait QByteArray_replace_11<RetType> {
  fn replace_11(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_replace_11<usize> for (usize,usize) {
  fn replace_11(self , rsthis: & QByteArray) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (self.1) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QByteArray7replaceERK7QStringPKc", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:327
// index:12
// Public Visibility=Default Availability=Available
// [8] QByteArray & replace(char, const QString &)

/*
Replaces len bytes from index position pos with the byte array after, and returns a reference to this byte array.

Example:


  QByteArray x("Say yes!");
  QByteArray y("no");
  x.replace(4, 3, y);
  // x == "Say no!"



See also insert() and remove().
*/
impl /*struct*/ QByteArray {
  pub fn replace_12<RetType, T: QByteArray_replace_12<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.replace_12(self);
    // return 1;
  }
}
pub trait QByteArray_replace_12<RetType> {
  fn replace_12(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_replace_12<usize> for (i8,usize) {
  fn replace_12(self , rsthis: & QByteArray) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i8 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QByteArray7replaceEcRK7QString", 2,qtrt::FFITY_SINT8,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:328
// index:13
// Public Visibility=Default Availability=Available
// [8] QByteArray & replace(const QString &, const QByteArray &)

/*
Replaces len bytes from index position pos with the byte array after, and returns a reference to this byte array.

Example:


  QByteArray x("Say yes!");
  QByteArray y("no");
  x.replace(4, 3, y);
  // x == "Say no!"



See also insert() and remove().
*/
impl /*struct*/ QByteArray {
  pub fn replace_13<RetType, T: QByteArray_replace_13<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.replace_13(self);
    // return 1;
  }
}
pub trait QByteArray_replace_13<RetType> {
  fn replace_13(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_replace_13<usize> for (usize,usize) {
  fn replace_13(self , rsthis: & QByteArray) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QByteArray7replaceERK7QStringRKS_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:315
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QByteArray & operator+=(char)

/*

*/
impl /*struct*/ QByteArray {
  pub fn operator_add_equal_0<RetType, T: QByteArray_operator_add_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_add_equal_0(self);
    // return 1;
  }
}
pub trait QByteArray_operator_add_equal_0<RetType> {
  fn operator_add_equal_0(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_operator_add_equal_0<usize> for (i8) {
  fn operator_add_equal_0(self , rsthis: & QByteArray) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i8 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QByteArraypLEc", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:316
// index:1
// Public inline Visibility=Default Availability=Available
// [8] QByteArray & operator+=(const char *)

/*

*/
impl /*struct*/ QByteArray {
  pub fn operator_add_equal_1<RetType, T: QByteArray_operator_add_equal_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_add_equal_1(self);
    // return 1;
  }
}
pub trait QByteArray_operator_add_equal_1<RetType> {
  fn operator_add_equal_1(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_operator_add_equal_1<usize> for (usize) {
  fn operator_add_equal_1(self , rsthis: & QByteArray) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QByteArraypLEPKc", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:317
// index:2
// Public inline Visibility=Default Availability=Available
// [8] QByteArray & operator+=(const QByteArray &)

/*

*/
impl /*struct*/ QByteArray {
  pub fn operator_add_equal_2<RetType, T: QByteArray_operator_add_equal_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_add_equal_2(self);
    // return 1;
  }
}
pub trait QByteArray_operator_add_equal_2<RetType> {
  fn operator_add_equal_2(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_operator_add_equal_2<usize> for (usize) {
  fn operator_add_equal_2(self , rsthis: & QByteArray) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QByteArraypLERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:330
// index:3
// Public Visibility=Default Availability=Available
// [8] QByteArray & operator+=(const QString &)

/*

*/
impl /*struct*/ QByteArray {
  pub fn operator_add_equal_3<RetType, T: QByteArray_operator_add_equal_3<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_add_equal_3(self);
    // return 1;
  }
}
pub trait QByteArray_operator_add_equal_3<RetType> {
  fn operator_add_equal_3(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_operator_add_equal_3<usize> for (usize) {
  fn operator_add_equal_3(self , rsthis: & QByteArray) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QByteArraypLERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:321
// index:0
// Public Visibility=Default Availability=Available
// [8] QByteArray repeated(int) const

/*
Returns a copy of this byte array repeated the specified number of times.

If times is less than 1, an empty byte array is returned.

Example:


  QByteArray ba("ab");
  ba.repeated(4);             // returns "abababab"



This function was introduced in  Qt 4.5.
*/
impl /*struct*/ QByteArray {
  pub fn repeated_0<RetType, T: QByteArray_repeated_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.repeated_0(self);
    // return 1;
  }
}
pub trait QByteArray_repeated_0<RetType> {
  fn repeated_0(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_repeated_0<usize> for (i32) {
  fn repeated_0(self , rsthis: & QByteArray) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QByteArray8repeatedEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:335
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator==(const QString &) const

/*

*/
impl /*struct*/ QByteArray {
  pub fn operator_equal_equal_0<RetType, T: QByteArray_operator_equal_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_equal_0(self);
    // return 1;
  }
}
pub trait QByteArray_operator_equal_equal_0<RetType> {
  fn operator_equal_equal_0(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_operator_equal_equal_0<bool> for (usize) {
  fn operator_equal_equal_0(self , rsthis: & QByteArray) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QByteArrayeqERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:336
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator!=(const QString &) const

/*

*/
impl /*struct*/ QByteArray {
  pub fn operator_not_equal_0<RetType, T: QByteArray_operator_not_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_not_equal_0(self);
    // return 1;
  }
}
pub trait QByteArray_operator_not_equal_0<RetType> {
  fn operator_not_equal_0(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_operator_not_equal_0<bool> for (usize) {
  fn operator_not_equal_0(self , rsthis: & QByteArray) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QByteArrayneERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:337
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator<(const QString &) const

/*

*/
impl /*struct*/ QByteArray {
  pub fn operator_less_than_0<RetType, T: QByteArray_operator_less_than_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_less_than_0(self);
    // return 1;
  }
}
pub trait QByteArray_operator_less_than_0<RetType> {
  fn operator_less_than_0(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_operator_less_than_0<bool> for (usize) {
  fn operator_less_than_0(self , rsthis: & QByteArray) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QByteArrayltERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:338
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator>(const QString &) const

/*

*/
impl /*struct*/ QByteArray {
  pub fn operator_greater_than_0<RetType, T: QByteArray_operator_greater_than_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_greater_than_0(self);
    // return 1;
  }
}
pub trait QByteArray_operator_greater_than_0<RetType> {
  fn operator_greater_than_0(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_operator_greater_than_0<bool> for (usize) {
  fn operator_greater_than_0(self , rsthis: & QByteArray) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QByteArraygtERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:339
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator<=(const QString &) const

/*

*/
impl /*struct*/ QByteArray {
  pub fn operator_less_than_equal_0<RetType, T: QByteArray_operator_less_than_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_less_than_equal_0(self);
    // return 1;
  }
}
pub trait QByteArray_operator_less_than_equal_0<RetType> {
  fn operator_less_than_equal_0(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_operator_less_than_equal_0<bool> for (usize) {
  fn operator_less_than_equal_0(self , rsthis: & QByteArray) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QByteArrayleERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:340
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator>=(const QString &) const

/*

*/
impl /*struct*/ QByteArray {
  pub fn operator_greater_than_equal_0<RetType, T: QByteArray_operator_greater_than_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_greater_than_equal_0(self);
    // return 1;
  }
}
pub trait QByteArray_operator_greater_than_equal_0<RetType> {
  fn operator_greater_than_equal_0(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_operator_greater_than_equal_0<bool> for (usize) {
  fn operator_greater_than_equal_0(self , rsthis: & QByteArray) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QByteArraygeERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:343
// index:0
// Public Visibility=Default Availability=Available
// [2] short toShort(bool *, int) const

/*
Returns the byte array converted to a short using base base, which is 10 by default and must be between 2 and 36, or 0.

If base is 0, the base is determined automatically using the following rules: If the byte array begins with "0x", it is assumed to be hexadecimal; if it begins with "0", it is assumed to be octal; otherwise it is assumed to be decimal.

Returns 0 if the conversion fails.

If ok is not 0: if a conversion error occurs, *ok is set to false; otherwise *ok is set to true.

Note: The conversion of the number is performed in the default C locale, irrespective of the user's locale.

See also number().
*/
impl /*struct*/ QByteArray {
  pub fn toShort_0<RetType, T: QByteArray_toShort_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toShort_0(self);
    // return 1;
  }
}
pub trait QByteArray_toShort_0<RetType> {
  fn toShort_0(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_toShort_0<i16> for (usize,i32) {
  fn toShort_0(self , rsthis: & QByteArray) -> i16 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QByteArray7toShortEPbi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i16 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:344
// index:0
// Public Visibility=Default Availability=Available
// [2] ushort toUShort(bool *, int) const

/*
Returns the byte array converted to an unsigned short using base base, which is 10 by default and must be between 2 and 36, or 0.

If base is 0, the base is determined automatically using the following rules: If the byte array begins with "0x", it is assumed to be hexadecimal; if it begins with "0", it is assumed to be octal; otherwise it is assumed to be decimal.

Returns 0 if the conversion fails.

If ok is not 0: if a conversion error occurs, *ok is set to false; otherwise *ok is set to true.

Note: The conversion of the number is performed in the default C locale, irrespective of the user's locale.

See also number().
*/
impl /*struct*/ QByteArray {
  pub fn toUShort_0<RetType, T: QByteArray_toUShort_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toUShort_0(self);
    // return 1;
  }
}
pub trait QByteArray_toUShort_0<RetType> {
  fn toUShort_0(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_toUShort_0<u16> for (usize,i32) {
  fn toUShort_0(self , rsthis: & QByteArray) -> u16 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QByteArray8toUShortEPbi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u16 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:345
// index:0
// Public Visibility=Default Availability=Available
// [4] int toInt(bool *, int) const

/*
Returns the byte array converted to an int using base base, which is 10 by default and must be between 2 and 36, or 0.

If base is 0, the base is determined automatically using the following rules: If the byte array begins with "0x", it is assumed to be hexadecimal; if it begins with "0", it is assumed to be octal; otherwise it is assumed to be decimal.

Returns 0 if the conversion fails.

If ok is not 0: if a conversion error occurs, *ok is set to false; otherwise *ok is set to true.


  QByteArray str("FF");
  bool ok;
  int hex = str.toInt(&ok, 16);     // hex == 255, ok == true
  int dec = str.toInt(&ok, 10);     // dec == 0, ok == false



Note: The conversion of the number is performed in the default C locale, irrespective of the user's locale.

See also number().
*/
impl /*struct*/ QByteArray {
  pub fn toInt_0<RetType, T: QByteArray_toInt_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toInt_0(self);
    // return 1;
  }
}
pub trait QByteArray_toInt_0<RetType> {
  fn toInt_0(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_toInt_0<i32> for (usize,i32) {
  fn toInt_0(self , rsthis: & QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QByteArray5toIntEPbi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:346
// index:0
// Public Visibility=Default Availability=Available
// [4] uint toUInt(bool *, int) const

/*
Returns the byte array converted to an unsigned int using base base, which is 10 by default and must be between 2 and 36, or 0.

If base is 0, the base is determined automatically using the following rules: If the byte array begins with "0x", it is assumed to be hexadecimal; if it begins with "0", it is assumed to be octal; otherwise it is assumed to be decimal.

Returns 0 if the conversion fails.

If ok is not 0: if a conversion error occurs, *ok is set to false; otherwise *ok is set to true.

Note: The conversion of the number is performed in the default C locale, irrespective of the user's locale.

See also number().
*/
impl /*struct*/ QByteArray {
  pub fn toUInt_0<RetType, T: QByteArray_toUInt_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toUInt_0(self);
    // return 1;
  }
}
pub trait QByteArray_toUInt_0<RetType> {
  fn toUInt_0(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_toUInt_0<u32> for (usize,i32) {
  fn toUInt_0(self , rsthis: & QByteArray) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QByteArray6toUIntEPbi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:347
// index:0
// Public Visibility=Default Availability=Available
// [8] long toLong(bool *, int) const

/*
Returns the byte array converted to a long int using base base, which is 10 by default and must be between 2 and 36, or 0.

If base is 0, the base is determined automatically using the following rules: If the byte array begins with "0x", it is assumed to be hexadecimal; if it begins with "0", it is assumed to be octal; otherwise it is assumed to be decimal.

Returns 0 if the conversion fails.

If ok is not 0: if a conversion error occurs, *ok is set to false; otherwise *ok is set to true.


  QByteArray str("FF");
  bool ok;
  long hex = str.toLong(&ok, 16);   // hex == 255, ok == true
  long dec = str.toLong(&ok, 10);   // dec == 0, ok == false



Note: The conversion of the number is performed in the default C locale, irrespective of the user's locale.

This function was introduced in  Qt 4.1.

See also number().
*/
impl /*struct*/ QByteArray {
  pub fn toLong_0<RetType, T: QByteArray_toLong_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toLong_0(self);
    // return 1;
  }
}
pub trait QByteArray_toLong_0<RetType> {
  fn toLong_0(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_toLong_0<i64> for (usize,i32) {
  fn toLong_0(self , rsthis: & QByteArray) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QByteArray6toLongEPbi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:348
// index:0
// Public Visibility=Default Availability=Available
// [8] ulong toULong(bool *, int) const

/*
Returns the byte array converted to an unsigned long int using base base, which is 10 by default and must be between 2 and 36, or 0.

If base is 0, the base is determined automatically using the following rules: If the byte array begins with "0x", it is assumed to be hexadecimal; if it begins with "0", it is assumed to be octal; otherwise it is assumed to be decimal.

Returns 0 if the conversion fails.

If ok is not 0: if a conversion error occurs, *ok is set to false; otherwise *ok is set to true.

Note: The conversion of the number is performed in the default C locale, irrespective of the user's locale.

This function was introduced in  Qt 4.1.

See also number().
*/
impl /*struct*/ QByteArray {
  pub fn toULong_0<RetType, T: QByteArray_toULong_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toULong_0(self);
    // return 1;
  }
}
pub trait QByteArray_toULong_0<RetType> {
  fn toULong_0(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_toULong_0<u64> for (usize,i32) {
  fn toULong_0(self , rsthis: & QByteArray) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QByteArray7toULongEPbi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:349
// index:0
// Public Visibility=Default Availability=Available
// [8] qlonglong toLongLong(bool *, int) const

/*
Returns the byte array converted to a long long using base base, which is 10 by default and must be between 2 and 36, or 0.

If base is 0, the base is determined automatically using the following rules: If the byte array begins with "0x", it is assumed to be hexadecimal; if it begins with "0", it is assumed to be octal; otherwise it is assumed to be decimal.

Returns 0 if the conversion fails.

If ok is not 0: if a conversion error occurs, *ok is set to false; otherwise *ok is set to true.

Note: The conversion of the number is performed in the default C locale, irrespective of the user's locale.

See also number().
*/
impl /*struct*/ QByteArray {
  pub fn toLongLong_0<RetType, T: QByteArray_toLongLong_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toLongLong_0(self);
    // return 1;
  }
}
pub trait QByteArray_toLongLong_0<RetType> {
  fn toLongLong_0(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_toLongLong_0<i64> for (usize,i32) {
  fn toLongLong_0(self , rsthis: & QByteArray) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QByteArray10toLongLongEPbi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:350
// index:0
// Public Visibility=Default Availability=Available
// [8] qulonglong toULongLong(bool *, int) const

/*
Returns the byte array converted to an unsigned long long using base base, which is 10 by default and must be between 2 and 36, or 0.

If base is 0, the base is determined automatically using the following rules: If the byte array begins with "0x", it is assumed to be hexadecimal; if it begins with "0", it is assumed to be octal; otherwise it is assumed to be decimal.

Returns 0 if the conversion fails.

If ok is not 0: if a conversion error occurs, *ok is set to false; otherwise *ok is set to true.

Note: The conversion of the number is performed in the default C locale, irrespective of the user's locale.

See also number().
*/
impl /*struct*/ QByteArray {
  pub fn toULongLong_0<RetType, T: QByteArray_toULongLong_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toULongLong_0(self);
    // return 1;
  }
}
pub trait QByteArray_toULongLong_0<RetType> {
  fn toULongLong_0(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_toULongLong_0<u64> for (usize,i32) {
  fn toULongLong_0(self , rsthis: & QByteArray) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QByteArray11toULongLongEPbi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:351
// index:0
// Public Visibility=Default Availability=Available
// [4] float toFloat(bool *) const

/*
Returns the byte array converted to a float value.

Returns 0.0 if the conversion fails.

If ok is not 0: if a conversion error occurs, *ok is set to false; otherwise *ok is set to true.

Note: The conversion of the number is performed in the default C locale, irrespective of the user's locale.

See also number().
*/
impl /*struct*/ QByteArray {
  pub fn toFloat_0<RetType, T: QByteArray_toFloat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toFloat_0(self);
    // return 1;
  }
}
pub trait QByteArray_toFloat_0<RetType> {
  fn toFloat_0(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_toFloat_0<f32> for (usize) {
  fn toFloat_0(self , rsthis: & QByteArray) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QByteArray7toFloatEPb", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:352
// index:0
// Public Visibility=Default Availability=Available
// [8] double toDouble(bool *) const

/*
Returns the byte array converted to a double value.

Returns 0.0 if the conversion fails.

If ok is not 0: if a conversion error occurs, *ok is set to false; otherwise *ok is set to true.


  QByteArray string("1234.56");
  double a = string.toDouble();   // a == 1234.56



Note: The conversion of the number is performed in the default C locale, irrespective of the user's locale.

See also number().
*/
impl /*struct*/ QByteArray {
  pub fn toDouble_0<RetType, T: QByteArray_toDouble_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toDouble_0(self);
    // return 1;
  }
}
pub trait QByteArray_toDouble_0<RetType> {
  fn toDouble_0(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_toDouble_0<f64> for (usize) {
  fn toDouble_0(self , rsthis: & QByteArray) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QByteArray8toDoubleEPb", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:353
// index:0
// Public Visibility=Default Availability=Available
// [8] QByteArray toBase64(QByteArray::Base64Options) const

/*
Returns a copy of the byte array, encoded as Base64.


  QByteArray text("Qt is great!");
  text.toBase64();        // returns "UXQgaXMgZ3JlYXQh"



The algorithm used to encode Base64-encoded data is defined in RFC 4648.

See also fromBase64().
*/
impl /*struct*/ QByteArray {
  pub fn toBase64_0<RetType, T: QByteArray_toBase64_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toBase64_0(self);
    // return 1;
  }
}
pub trait QByteArray_toBase64_0<RetType> {
  fn toBase64_0(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_toBase64_0<usize> for (i32) {
  fn toBase64_0(self , rsthis: & QByteArray) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QByteArray8toBase64E6QFlagsINS_12Base64OptionEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:354
// index:1
// Public Visibility=Default Availability=Available
// [8] QByteArray toBase64() const

/*
Returns a copy of the byte array, encoded as Base64.


  QByteArray text("Qt is great!");
  text.toBase64();        // returns "UXQgaXMgZ3JlYXQh"



The algorithm used to encode Base64-encoded data is defined in RFC 4648.

See also fromBase64().
*/
impl /*struct*/ QByteArray {
  pub fn toBase64_1<RetType, T: QByteArray_toBase64_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toBase64_1(self);
    // return 1;
  }
}
pub trait QByteArray_toBase64_1<RetType> {
  fn toBase64_1(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_toBase64_1<usize> for () {
  fn toBase64_1(self , rsthis: & QByteArray) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QByteArray8toBase64Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:355
// index:0
// Public Visibility=Default Availability=Available
// [8] QByteArray toHex() const

/*
Returns a hex encoded copy of the byte array. The hex encoding uses the numbers 0-9 and the letters a-f.

See also fromHex().
*/
impl /*struct*/ QByteArray {
  pub fn toHex_0<RetType, T: QByteArray_toHex_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toHex_0(self);
    // return 1;
  }
}
pub trait QByteArray_toHex_0<RetType> {
  fn toHex_0(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_toHex_0<usize> for () {
  fn toHex_0(self , rsthis: & QByteArray) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QByteArray5toHexEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:356
// index:1
// Public Visibility=Default Availability=Available
// [8] QByteArray toHex(char) const

/*
Returns a hex encoded copy of the byte array. The hex encoding uses the numbers 0-9 and the letters a-f.

See also fromHex().
*/
impl /*struct*/ QByteArray {
  pub fn toHex_1<RetType, T: QByteArray_toHex_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toHex_1(self);
    // return 1;
  }
}
pub trait QByteArray_toHex_1<RetType> {
  fn toHex_1(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_toHex_1<usize> for (i8) {
  fn toHex_1(self , rsthis: & QByteArray) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i8 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QByteArray5toHexEc", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:357
// index:0
// Public Visibility=Default Availability=Available
// [8] QByteArray toPercentEncoding(const QByteArray &, const QByteArray &, char) const

/*
Returns a URI/URL-style percent-encoded copy of this byte array. The percent parameter allows you to override the default '%' character for another.

By default, this function will encode all characters that are not one of the following:

ALPHA ("a" to "z" and "A" to "Z") / DIGIT (0 to 9) / "-" / "." / "_" / "~"

To prevent characters from being encoded pass them to exclude. To force characters to be encoded pass them to include. The percent character is always encoded.

Example:


  QByteArray text = "{a fishy string?}";
  QByteArray ba = text.toPercentEncoding("{}", "s");
  qDebug(ba.constData());
  // prints "{a fi%73hy %73tring%3F}"



The hex encoding uses the numbers 0-9 and the uppercase letters A-F.

This function was introduced in  Qt 4.4.

See also fromPercentEncoding() and QUrl::toPercentEncoding().
*/
impl /*struct*/ QByteArray {
  pub fn toPercentEncoding_0<RetType, T: QByteArray_toPercentEncoding_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toPercentEncoding_0(self);
    // return 1;
  }
}
pub trait QByteArray_toPercentEncoding_0<RetType> {
  fn toPercentEncoding_0(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_toPercentEncoding_0<usize> for (usize,usize,i8) {
  fn toPercentEncoding_0(self , rsthis: & QByteArray) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const i8 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QByteArray17toPercentEncodingERKS_S1_c", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:361
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QByteArray & setNum(short, int)

/*
Sets the byte array to the printed value of n in base base (10 by default) and returns a reference to the byte array. The base can be any value between 2 and 36. For bases other than 10, n is treated as an unsigned integer.

Example:


  QByteArray ba;
  int n = 63;
  ba.setNum(n);           // ba == "63"
  ba.setNum(n, 16);       // ba == "3f"



Note: The format of the number is not localized; the default C locale is used irrespective of the user's locale.

See also number() and toInt().
*/
impl /*struct*/ QByteArray {
  pub fn setNum_0<RetType, T: QByteArray_setNum_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setNum_0(self);
    // return 1;
  }
}
pub trait QByteArray_setNum_0<RetType> {
  fn setNum_0(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_setNum_0<usize> for (i16,i32) {
  fn setNum_0(self , rsthis: & QByteArray) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i16 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QByteArray6setNumEsi", 2,qtrt::FFITY_SINT16,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:362
// index:1
// Public inline Visibility=Default Availability=Available
// [8] QByteArray & setNum(ushort, int)

/*
Sets the byte array to the printed value of n in base base (10 by default) and returns a reference to the byte array. The base can be any value between 2 and 36. For bases other than 10, n is treated as an unsigned integer.

Example:


  QByteArray ba;
  int n = 63;
  ba.setNum(n);           // ba == "63"
  ba.setNum(n, 16);       // ba == "3f"



Note: The format of the number is not localized; the default C locale is used irrespective of the user's locale.

See also number() and toInt().
*/
impl /*struct*/ QByteArray {
  pub fn setNum_1<RetType, T: QByteArray_setNum_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setNum_1(self);
    // return 1;
  }
}
pub trait QByteArray_setNum_1<RetType> {
  fn setNum_1(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_setNum_1<usize> for (u16,i32) {
  fn setNum_1(self , rsthis: & QByteArray) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const u16 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QByteArray6setNumEti", 2,qtrt::FFITY_UINT16,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:363
// index:2
// Public inline Visibility=Default Availability=Available
// [8] QByteArray & setNum(int, int)

/*
Sets the byte array to the printed value of n in base base (10 by default) and returns a reference to the byte array. The base can be any value between 2 and 36. For bases other than 10, n is treated as an unsigned integer.

Example:


  QByteArray ba;
  int n = 63;
  ba.setNum(n);           // ba == "63"
  ba.setNum(n, 16);       // ba == "3f"



Note: The format of the number is not localized; the default C locale is used irrespective of the user's locale.

See also number() and toInt().
*/
impl /*struct*/ QByteArray {
  pub fn setNum_2<RetType, T: QByteArray_setNum_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setNum_2(self);
    // return 1;
  }
}
pub trait QByteArray_setNum_2<RetType> {
  fn setNum_2(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_setNum_2<usize> for (i32,i32) {
  fn setNum_2(self , rsthis: & QByteArray) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QByteArray6setNumEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:364
// index:3
// Public inline Visibility=Default Availability=Available
// [8] QByteArray & setNum(uint, int)

/*
Sets the byte array to the printed value of n in base base (10 by default) and returns a reference to the byte array. The base can be any value between 2 and 36. For bases other than 10, n is treated as an unsigned integer.

Example:


  QByteArray ba;
  int n = 63;
  ba.setNum(n);           // ba == "63"
  ba.setNum(n, 16);       // ba == "3f"



Note: The format of the number is not localized; the default C locale is used irrespective of the user's locale.

See also number() and toInt().
*/
impl /*struct*/ QByteArray {
  pub fn setNum_3<RetType, T: QByteArray_setNum_3<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setNum_3(self);
    // return 1;
  }
}
pub trait QByteArray_setNum_3<RetType> {
  fn setNum_3(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_setNum_3<usize> for (u32,i32) {
  fn setNum_3(self , rsthis: & QByteArray) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const u32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QByteArray6setNumEji", 2,qtrt::FFITY_UINT32,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:365
// index:4
// Public Visibility=Default Availability=Available
// [8] QByteArray & setNum(qlonglong, int)

/*
Sets the byte array to the printed value of n in base base (10 by default) and returns a reference to the byte array. The base can be any value between 2 and 36. For bases other than 10, n is treated as an unsigned integer.

Example:


  QByteArray ba;
  int n = 63;
  ba.setNum(n);           // ba == "63"
  ba.setNum(n, 16);       // ba == "3f"



Note: The format of the number is not localized; the default C locale is used irrespective of the user's locale.

See also number() and toInt().
*/
impl /*struct*/ QByteArray {
  pub fn setNum_4<RetType, T: QByteArray_setNum_4<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setNum_4(self);
    // return 1;
  }
}
pub trait QByteArray_setNum_4<RetType> {
  fn setNum_4(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_setNum_4<usize> for (i64,i32) {
  fn setNum_4(self , rsthis: & QByteArray) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i64 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QByteArray6setNumExi", 2,qtrt::FFITY_SINT64,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:366
// index:5
// Public Visibility=Default Availability=Available
// [8] QByteArray & setNum(qulonglong, int)

/*
Sets the byte array to the printed value of n in base base (10 by default) and returns a reference to the byte array. The base can be any value between 2 and 36. For bases other than 10, n is treated as an unsigned integer.

Example:


  QByteArray ba;
  int n = 63;
  ba.setNum(n);           // ba == "63"
  ba.setNum(n, 16);       // ba == "3f"



Note: The format of the number is not localized; the default C locale is used irrespective of the user's locale.

See also number() and toInt().
*/
impl /*struct*/ QByteArray {
  pub fn setNum_5<RetType, T: QByteArray_setNum_5<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setNum_5(self);
    // return 1;
  }
}
pub trait QByteArray_setNum_5<RetType> {
  fn setNum_5(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_setNum_5<usize> for (u64,i32) {
  fn setNum_5(self , rsthis: & QByteArray) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const u64 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QByteArray6setNumEyi", 2,qtrt::FFITY_UINT64,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:367
// index:6
// Public inline Visibility=Default Availability=Available
// [8] QByteArray & setNum(float, char, int)

/*
Sets the byte array to the printed value of n in base base (10 by default) and returns a reference to the byte array. The base can be any value between 2 and 36. For bases other than 10, n is treated as an unsigned integer.

Example:


  QByteArray ba;
  int n = 63;
  ba.setNum(n);           // ba == "63"
  ba.setNum(n, 16);       // ba == "3f"



Note: The format of the number is not localized; the default C locale is used irrespective of the user's locale.

See also number() and toInt().
*/
impl /*struct*/ QByteArray {
  pub fn setNum_6<RetType, T: QByteArray_setNum_6<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setNum_6(self);
    // return 1;
  }
}
pub trait QByteArray_setNum_6<RetType> {
  fn setNum_6(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_setNum_6<usize> for (f32,i8,i32) {
  fn setNum_6(self , rsthis: & QByteArray) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f32 as usize;
    let arg1 = (&self.1) as *const i8 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QByteArray6setNumEfci", 3,qtrt::FFITY_FLOAT,qtrt::FFITY_SINT8,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:368
// index:7
// Public Visibility=Default Availability=Available
// [8] QByteArray & setNum(double, char, int)

/*
Sets the byte array to the printed value of n in base base (10 by default) and returns a reference to the byte array. The base can be any value between 2 and 36. For bases other than 10, n is treated as an unsigned integer.

Example:


  QByteArray ba;
  int n = 63;
  ba.setNum(n);           // ba == "63"
  ba.setNum(n, 16);       // ba == "3f"



Note: The format of the number is not localized; the default C locale is used irrespective of the user's locale.

See also number() and toInt().
*/
impl /*struct*/ QByteArray {
  pub fn setNum_7<RetType, T: QByteArray_setNum_7<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setNum_7(self);
    // return 1;
  }
}
pub trait QByteArray_setNum_7<RetType> {
  fn setNum_7(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_setNum_7<usize> for (f64,i8,i32) {
  fn setNum_7(self , rsthis: & QByteArray) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const i8 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QByteArray6setNumEdci", 3,qtrt::FFITY_DOUBLE,qtrt::FFITY_SINT8,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:369
// index:0
// Public Visibility=Default Availability=Available
// [8] QByteArray & setRawData(const char *, uint)

/*
Resets the QByteArray to use the first size bytes of the data array. The bytes are not copied. The QByteArray will contain the data pointer. The caller guarantees that data will not be deleted or modified as long as this QByteArray and any copies of it exist that have not been modified.

This function can be used instead of fromRawData() to re-use existing QByteArray objects to save memory re-allocations.

This function was introduced in  Qt 4.7.

See also fromRawData(), data(), and constData().
*/
impl /*struct*/ QByteArray {
  pub fn setRawData_0<RetType, T: QByteArray_setRawData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setRawData_0(self);
    // return 1;
  }
}
pub trait QByteArray_setRawData_0<RetType> {
  fn setRawData_0(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_setRawData_0<usize> for (usize,u32) {
  fn setRawData_0(self , rsthis: & QByteArray) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const u32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QByteArray10setRawDataEPKcj", 2,qtrt::FFITY_POINTER,qtrt::FFITY_UINT32,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:371
// index:0
// Public static Visibility=Default Availability=Available
// [8] QByteArray number(int, int)

/*
Returns a byte array containing the string equivalent of the number n to base base (10 by default). The base can be any value between 2 and 36.

Example:


  int n = 63;
  QByteArray::number(n);              // returns "63"
  QByteArray::number(n, 16);          // returns "3f"
  QByteArray::number(n, 16).toUpper();  // returns "3F"



Note: The format of the number is not localized; the default C locale is used irrespective of the user's locale.

See also setNum() and toInt().
*/
impl /*struct*/ QByteArray {
  pub fn number_0<RetType, T: QByteArray_number_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.number_0();
    // return 1;
  }
}
pub trait QByteArray_number_0<RetType> {
  fn number_0(self ) -> RetType;
}
impl<'a> /*trait*/ QByteArray_number_0<usize> for (i32,i32) {
  fn number_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QByteArray6numberEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:372
// index:1
// Public static Visibility=Default Availability=Available
// [8] QByteArray number(uint, int)

/*
Returns a byte array containing the string equivalent of the number n to base base (10 by default). The base can be any value between 2 and 36.

Example:


  int n = 63;
  QByteArray::number(n);              // returns "63"
  QByteArray::number(n, 16);          // returns "3f"
  QByteArray::number(n, 16).toUpper();  // returns "3F"



Note: The format of the number is not localized; the default C locale is used irrespective of the user's locale.

See also setNum() and toInt().
*/
impl /*struct*/ QByteArray {
  pub fn number_1<RetType, T: QByteArray_number_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.number_1();
    // return 1;
  }
}
pub trait QByteArray_number_1<RetType> {
  fn number_1(self ) -> RetType;
}
impl<'a> /*trait*/ QByteArray_number_1<usize> for (u32,i32) {
  fn number_1(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const u32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QByteArray6numberEji", 2,qtrt::FFITY_UINT32,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:373
// index:2
// Public static Visibility=Default Availability=Available
// [8] QByteArray number(qlonglong, int)

/*
Returns a byte array containing the string equivalent of the number n to base base (10 by default). The base can be any value between 2 and 36.

Example:


  int n = 63;
  QByteArray::number(n);              // returns "63"
  QByteArray::number(n, 16);          // returns "3f"
  QByteArray::number(n, 16).toUpper();  // returns "3F"



Note: The format of the number is not localized; the default C locale is used irrespective of the user's locale.

See also setNum() and toInt().
*/
impl /*struct*/ QByteArray {
  pub fn number_2<RetType, T: QByteArray_number_2<RetType>>( overload_args: T) -> RetType {
    return overload_args.number_2();
    // return 1;
  }
}
pub trait QByteArray_number_2<RetType> {
  fn number_2(self ) -> RetType;
}
impl<'a> /*trait*/ QByteArray_number_2<usize> for (i64,i32) {
  fn number_2(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i64 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QByteArray6numberExi", 2,qtrt::FFITY_SINT64,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:374
// index:3
// Public static Visibility=Default Availability=Available
// [8] QByteArray number(qulonglong, int)

/*
Returns a byte array containing the string equivalent of the number n to base base (10 by default). The base can be any value between 2 and 36.

Example:


  int n = 63;
  QByteArray::number(n);              // returns "63"
  QByteArray::number(n, 16);          // returns "3f"
  QByteArray::number(n, 16).toUpper();  // returns "3F"



Note: The format of the number is not localized; the default C locale is used irrespective of the user's locale.

See also setNum() and toInt().
*/
impl /*struct*/ QByteArray {
  pub fn number_3<RetType, T: QByteArray_number_3<RetType>>( overload_args: T) -> RetType {
    return overload_args.number_3();
    // return 1;
  }
}
pub trait QByteArray_number_3<RetType> {
  fn number_3(self ) -> RetType;
}
impl<'a> /*trait*/ QByteArray_number_3<usize> for (u64,i32) {
  fn number_3(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const u64 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QByteArray6numberEyi", 2,qtrt::FFITY_UINT64,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:375
// index:4
// Public static Visibility=Default Availability=Available
// [8] QByteArray number(double, char, int)

/*
Returns a byte array containing the string equivalent of the number n to base base (10 by default). The base can be any value between 2 and 36.

Example:


  int n = 63;
  QByteArray::number(n);              // returns "63"
  QByteArray::number(n, 16);          // returns "3f"
  QByteArray::number(n, 16).toUpper();  // returns "3F"



Note: The format of the number is not localized; the default C locale is used irrespective of the user's locale.

See also setNum() and toInt().
*/
impl /*struct*/ QByteArray {
  pub fn number_4<RetType, T: QByteArray_number_4<RetType>>( overload_args: T) -> RetType {
    return overload_args.number_4();
    // return 1;
  }
}
pub trait QByteArray_number_4<RetType> {
  fn number_4(self ) -> RetType;
}
impl<'a> /*trait*/ QByteArray_number_4<usize> for (f64,i8,i32) {
  fn number_4(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const i8 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QByteArray6numberEdci", 3,qtrt::FFITY_DOUBLE,qtrt::FFITY_SINT8,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:376
// index:0
// Public static Visibility=Default Availability=Available
// [8] QByteArray fromRawData(const char *, int)

/*
Constructs a QByteArray that uses the first size bytes of the data array. The bytes are not copied. The QByteArray will contain the data pointer. The caller guarantees that data will not be deleted or modified as long as this QByteArray and any copies of it exist that have not been modified. In other words, because QByteArray is an implicitly shared class and the instance returned by this function contains the data pointer, the caller must not delete data or modify it directly as long as the returned QByteArray and any copies exist. However, QByteArray does not take ownership of data, so the QByteArray destructor will never delete the raw data, even when the last QByteArray referring to data is destroyed.

A subsequent attempt to modify the contents of the returned QByteArray or any copy made from it will cause it to create a deep copy of the data array before doing the modification. This ensures that the raw data array itself will never be modified by QByteArray.

Here is an example of how to read data using a QDataStream on raw data in memory without copying the raw data into a QByteArray:


   static const char mydata[] = {
      0x00, 0x00, 0x03, 0x84, 0x78, 0x9c, 0x3b, 0x76,
      0xec, 0x18, 0xc3, 0x31, 0x0a, 0xf1, 0xcc, 0x99,
      ...
      0x6d, 0x5b
  };

  QByteArray data = QByteArray::fromRawData(mydata, sizeof(mydata));
  QDataStream in(&data, QIODevice::ReadOnly);
  ...



Warning: A byte array created with fromRawData() is not null-terminated, unless the raw data contains a 0 character at position size. While that does not matter for QDataStream or functions like indexOf(), passing the byte array to a function accepting a const char * expected to be '\0'-terminated will fail.

See also setRawData(), data(), and constData().
*/
impl /*struct*/ QByteArray {
  pub fn fromRawData_0<RetType, T: QByteArray_fromRawData_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromRawData_0();
    // return 1;
  }
}
pub trait QByteArray_fromRawData_0<RetType> {
  fn fromRawData_0(self ) -> RetType;
}
impl<'a> /*trait*/ QByteArray_fromRawData_0<usize> for (usize,i32) {
  fn fromRawData_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QByteArray11fromRawDataEPKci", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:377
// index:0
// Public static Visibility=Default Availability=Available
// [8] QByteArray fromBase64(const QByteArray &, QByteArray::Base64Options)

/*
Returns a decoded copy of the Base64 array base64. Input is not checked for validity; invalid characters in the input are skipped, enabling the decoding process to continue with subsequent characters.

For example:


  QByteArray text = QByteArray::fromBase64("UXQgaXMgZ3JlYXQh");
  text.data();            // returns "Qt is great!"



The algorithm used to decode Base64-encoded data is defined in RFC 4648.

See also toBase64().
*/
impl /*struct*/ QByteArray {
  pub fn fromBase64_0<RetType, T: QByteArray_fromBase64_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromBase64_0();
    // return 1;
  }
}
pub trait QByteArray_fromBase64_0<RetType> {
  fn fromBase64_0(self ) -> RetType;
}
impl<'a> /*trait*/ QByteArray_fromBase64_0<usize> for (usize,i32) {
  fn fromBase64_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QByteArray10fromBase64ERKS_6QFlagsINS_12Base64OptionEE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:378
// index:1
// Public static Visibility=Default Availability=Available
// [8] QByteArray fromBase64(const QByteArray &)

/*
Returns a decoded copy of the Base64 array base64. Input is not checked for validity; invalid characters in the input are skipped, enabling the decoding process to continue with subsequent characters.

For example:


  QByteArray text = QByteArray::fromBase64("UXQgaXMgZ3JlYXQh");
  text.data();            // returns "Qt is great!"



The algorithm used to decode Base64-encoded data is defined in RFC 4648.

See also toBase64().
*/
impl /*struct*/ QByteArray {
  pub fn fromBase64_1<RetType, T: QByteArray_fromBase64_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromBase64_1();
    // return 1;
  }
}
pub trait QByteArray_fromBase64_1<RetType> {
  fn fromBase64_1(self ) -> RetType;
}
impl<'a> /*trait*/ QByteArray_fromBase64_1<usize> for (usize) {
  fn fromBase64_1(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QByteArray10fromBase64ERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:379
// index:0
// Public static Visibility=Default Availability=Available
// [8] QByteArray fromHex(const QByteArray &)

/*
Returns a decoded copy of the hex encoded array hexEncoded. Input is not checked for validity; invalid characters in the input are skipped, enabling the decoding process to continue with subsequent characters.

For example:


  QByteArray text = QByteArray::fromHex("517420697320677265617421");
  text.data();            // returns "Qt is great!"



See also toHex().
*/
impl /*struct*/ QByteArray {
  pub fn fromHex_0<RetType, T: QByteArray_fromHex_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromHex_0();
    // return 1;
  }
}
pub trait QByteArray_fromHex_0<RetType> {
  fn fromHex_0(self ) -> RetType;
}
impl<'a> /*trait*/ QByteArray_fromHex_0<usize> for (usize) {
  fn fromHex_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QByteArray7fromHexERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:380
// index:0
// Public static Visibility=Default Availability=Available
// [8] QByteArray fromPercentEncoding(const QByteArray &, char)

/*
Returns a decoded copy of the URI/URL-style percent-encoded input. The percent parameter allows you to replace the '%' character for another (for instance, '_' or '=').

For example:


  QByteArray text = QByteArray::fromPercentEncoding("Qt%20is%20great%33");
  text.data();            // returns "Qt is great!"



Note: Given invalid input (such as a string containing the sequence "%G5", which is not a valid hexadecimal number) the output will be invalid as well. As an example: the sequence "%G5" could be decoded to 'W'.

This function was introduced in  Qt 4.4.

See also toPercentEncoding() and QUrl::fromPercentEncoding().
*/
impl /*struct*/ QByteArray {
  pub fn fromPercentEncoding_0<RetType, T: QByteArray_fromPercentEncoding_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromPercentEncoding_0();
    // return 1;
  }
}
pub trait QByteArray_fromPercentEncoding_0<RetType> {
  fn fromPercentEncoding_0(self ) -> RetType;
}
impl<'a> /*trait*/ QByteArray_fromPercentEncoding_0<usize> for (usize,i8) {
  fn fromPercentEncoding_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i8 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QByteArray19fromPercentEncodingERKS_c", 2,qtrt::FFITY_POINTER,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:399
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QByteArray::iterator begin()

/*
Returns an STL-style iterator pointing to the first character in the byte-array.

See also constBegin() and end().
*/
impl /*struct*/ QByteArray {
  pub fn begin_0<RetType, T: QByteArray_begin_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.begin_0(self);
    // return 1;
  }
}
pub trait QByteArray_begin_0<RetType> {
  fn begin_0(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_begin_0<usize> for () {
  fn begin_0(self , rsthis: & QByteArray) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QByteArray5beginEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:400
// index:1
// Public inline Visibility=Default Availability=Available
// [8] QByteArray::const_iterator begin() const

/*
Returns an STL-style iterator pointing to the first character in the byte-array.

See also constBegin() and end().
*/
impl /*struct*/ QByteArray {
  pub fn begin_1<RetType, T: QByteArray_begin_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.begin_1(self);
    // return 1;
  }
}
pub trait QByteArray_begin_1<RetType> {
  fn begin_1(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_begin_1<usize> for () {
  fn begin_1(self , rsthis: & QByteArray) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QByteArray5beginEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:401
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QByteArray::const_iterator cbegin() const

/*
Returns a const STL-style iterator pointing to the first character in the byte-array.

This function was introduced in  Qt 5.0.

See also begin() and cend().
*/
impl /*struct*/ QByteArray {
  pub fn cbegin_0<RetType, T: QByteArray_cbegin_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cbegin_0(self);
    // return 1;
  }
}
pub trait QByteArray_cbegin_0<RetType> {
  fn cbegin_0(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_cbegin_0<usize> for () {
  fn cbegin_0(self , rsthis: & QByteArray) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QByteArray6cbeginEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:402
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QByteArray::const_iterator constBegin() const

/*
Returns a const STL-style iterator pointing to the first character in the byte-array.

See also begin() and constEnd().
*/
impl /*struct*/ QByteArray {
  pub fn constBegin_0<RetType, T: QByteArray_constBegin_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.constBegin_0(self);
    // return 1;
  }
}
pub trait QByteArray_constBegin_0<RetType> {
  fn constBegin_0(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_constBegin_0<usize> for () {
  fn constBegin_0(self , rsthis: & QByteArray) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QByteArray10constBeginEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:403
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QByteArray::iterator end()

/*
Returns an STL-style iterator pointing to the imaginary character after the last character in the byte-array.

See also begin() and constEnd().
*/
impl /*struct*/ QByteArray {
  pub fn end_0<RetType, T: QByteArray_end_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.end_0(self);
    // return 1;
  }
}
pub trait QByteArray_end_0<RetType> {
  fn end_0(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_end_0<usize> for () {
  fn end_0(self , rsthis: & QByteArray) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QByteArray3endEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:404
// index:1
// Public inline Visibility=Default Availability=Available
// [8] QByteArray::const_iterator end() const

/*
Returns an STL-style iterator pointing to the imaginary character after the last character in the byte-array.

See also begin() and constEnd().
*/
impl /*struct*/ QByteArray {
  pub fn end_1<RetType, T: QByteArray_end_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.end_1(self);
    // return 1;
  }
}
pub trait QByteArray_end_1<RetType> {
  fn end_1(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_end_1<usize> for () {
  fn end_1(self , rsthis: & QByteArray) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QByteArray3endEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:405
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QByteArray::const_iterator cend() const

/*
Returns a const STL-style iterator pointing to the imaginary character after the last character in the list.

This function was introduced in  Qt 5.0.

See also cbegin() and end().
*/
impl /*struct*/ QByteArray {
  pub fn cend_0<RetType, T: QByteArray_cend_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cend_0(self);
    // return 1;
  }
}
pub trait QByteArray_cend_0<RetType> {
  fn cend_0(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_cend_0<usize> for () {
  fn cend_0(self , rsthis: & QByteArray) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QByteArray4cendEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:406
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QByteArray::const_iterator constEnd() const

/*
Returns a const STL-style iterator pointing to the imaginary character after the last character in the list.

See also constBegin() and end().
*/
impl /*struct*/ QByteArray {
  pub fn constEnd_0<RetType, T: QByteArray_constEnd_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.constEnd_0(self);
    // return 1;
  }
}
pub trait QByteArray_constEnd_0<RetType> {
  fn constEnd_0(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_constEnd_0<usize> for () {
  fn constEnd_0(self , rsthis: & QByteArray) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QByteArray8constEndEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:422
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void push_back(char)

/*
This function is provided for STL compatibility. It is equivalent to append(other).
*/
impl /*struct*/ QByteArray {
  pub fn push_back_0<RetType, T: QByteArray_push_back_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.push_back_0(self);
    // return 1;
  }
}
pub trait QByteArray_push_back_0<RetType> {
  fn push_back_0(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_push_back_0<(/*void*/)> for (i8) {
  fn push_back_0(self , rsthis: & QByteArray) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i8 as usize;
     qtrt::InvokeQtFunc6("_ZN10QByteArray9push_backEc", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:423
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void push_back(const char *)

/*
This function is provided for STL compatibility. It is equivalent to append(other).
*/
impl /*struct*/ QByteArray {
  pub fn push_back_1<RetType, T: QByteArray_push_back_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.push_back_1(self);
    // return 1;
  }
}
pub trait QByteArray_push_back_1<RetType> {
  fn push_back_1(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_push_back_1<(/*void*/)> for (usize) {
  fn push_back_1(self , rsthis: & QByteArray) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN10QByteArray9push_backEPKc", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:424
// index:2
// Public inline Visibility=Default Availability=Available
// [-2] void push_back(const QByteArray &)

/*
This function is provided for STL compatibility. It is equivalent to append(other).
*/
impl /*struct*/ QByteArray {
  pub fn push_back_2<RetType, T: QByteArray_push_back_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.push_back_2(self);
    // return 1;
  }
}
pub trait QByteArray_push_back_2<RetType> {
  fn push_back_2(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_push_back_2<(/*void*/)> for (usize) {
  fn push_back_2(self , rsthis: & QByteArray) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN10QByteArray9push_backERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:425
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void push_front(char)

/*
This function is provided for STL compatibility. It is equivalent to prepend(other).
*/
impl /*struct*/ QByteArray {
  pub fn push_front_0<RetType, T: QByteArray_push_front_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.push_front_0(self);
    // return 1;
  }
}
pub trait QByteArray_push_front_0<RetType> {
  fn push_front_0(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_push_front_0<(/*void*/)> for (i8) {
  fn push_front_0(self , rsthis: & QByteArray) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i8 as usize;
     qtrt::InvokeQtFunc6("_ZN10QByteArray10push_frontEc", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:426
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void push_front(const char *)

/*
This function is provided for STL compatibility. It is equivalent to prepend(other).
*/
impl /*struct*/ QByteArray {
  pub fn push_front_1<RetType, T: QByteArray_push_front_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.push_front_1(self);
    // return 1;
  }
}
pub trait QByteArray_push_front_1<RetType> {
  fn push_front_1(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_push_front_1<(/*void*/)> for (usize) {
  fn push_front_1(self , rsthis: & QByteArray) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN10QByteArray10push_frontEPKc", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:427
// index:2
// Public inline Visibility=Default Availability=Available
// [-2] void push_front(const QByteArray &)

/*
This function is provided for STL compatibility. It is equivalent to prepend(other).
*/
impl /*struct*/ QByteArray {
  pub fn push_front_2<RetType, T: QByteArray_push_front_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.push_front_2(self);
    // return 1;
  }
}
pub trait QByteArray_push_front_2<RetType> {
  fn push_front_2(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_push_front_2<(/*void*/)> for (usize) {
  fn push_front_2(self , rsthis: & QByteArray) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN10QByteArray10push_frontERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:428
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void shrink_to_fit()

/*
This function is provided for STL compatibility. It is equivalent to squeeze().

This function was introduced in  Qt 5.10.
*/
impl /*struct*/ QByteArray {
  pub fn shrink_to_fit_0<RetType, T: QByteArray_shrink_to_fit_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.shrink_to_fit_0(self);
    // return 1;
  }
}
pub trait QByteArray_shrink_to_fit_0<RetType> {
  fn shrink_to_fit_0(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_shrink_to_fit_0<(/*void*/)> for () {
  fn shrink_to_fit_0(self , rsthis: & QByteArray) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN10QByteArray13shrink_to_fitEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:431
// index:0
// Public inline Visibility=Default Availability=Available
// [32] std::string toStdString() const

/*
Returns a std::string object with the data contained in this QByteArray.

This operator is mostly useful to pass a QByteArray to a function that accepts a std::string object.

This function was introduced in  Qt 5.4.

See also fromStdString() and QString::toStdString().
*/
impl /*struct*/ QByteArray {
  pub fn toStdString_0<RetType, T: QByteArray_toStdString_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toStdString_0(self);
    // return 1;
  }
}
pub trait QByteArray_toStdString_0<RetType> {
  fn toStdString_0(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_toStdString_0<i32> for () {
  fn toStdString_0(self , rsthis: & QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QByteArray11toStdStringB5cxx11Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:434
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int length() const

/*
Same as size().
*/
impl /*struct*/ QByteArray {
  pub fn length_0<RetType, T: QByteArray_length_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.length_0(self);
    // return 1;
  }
}
pub trait QByteArray_length_0<RetType> {
  fn length_0(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_length_0<i32> for () {
  fn length_0(self , rsthis: & QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QByteArray6lengthEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearray.h:435
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isNull() const

/*
Returns true if this byte array is null; otherwise returns false.

Example:


  QByteArray().isNull();          // returns true
  QByteArray("").isNull();        // returns false
  QByteArray("abc").isNull();     // returns false



Qt makes a distinction between null byte arrays and empty byte arrays for historical reasons. For most applications, what matters is whether or not a byte array contains any data, and this can be determined using isEmpty().

See also isEmpty().
*/
impl /*struct*/ QByteArray {
  pub fn isNull_0<RetType, T: QByteArray_isNull_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isNull_0(self);
    // return 1;
  }
}
pub trait QByteArray_isNull_0<RetType> {
  fn isNull_0(self , rsthis: & QByteArray) -> RetType;
}
impl<'a> /*trait*/ QByteArray_isNull_0<bool> for () {
  fn isNull_0(self , rsthis: & QByteArray) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QByteArray6isNullEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}


/*


*/
pub type QByteArray__Base64Option = i32;
// 
pub const QByteArray__Base64Encoding :QByteArray__Base64Option = 0;
// 
pub const QByteArray__Base64UrlEncoding :QByteArray__Base64Option = 1;
// 
pub const QByteArray__KeepTrailingEquals :QByteArray__Base64Option = 0;
// 
pub const QByteArray__OmitTrailingEquals :QByteArray__Base64Option = 2;
pub fn QByteArray_Base64OptionItemName(val: i32) ->String {
  match val {
     QByteArray__Base64Encoding => // 0
     {return String::from("Base64Encoding,KeepTrailingEquals");}
     QByteArray__Base64UrlEncoding => // 1
     {return String::from("Base64UrlEncoding");}
    // QByteArray__KeepTrailingEquals => // 0
    // {return String::from("");}
     QByteArray__OmitTrailingEquals => // 2
     {return String::from("OmitTrailingEquals");}
  _ => {return format!("{}", val);}
}
}
pub fn QByteArray_Base64OptionItemName_s(val: i32) ->String {
  //var nilthis *QByteArray
  //return nilthis.Base64OptionItemName(val);
  return QByteArray_Base64OptionItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
