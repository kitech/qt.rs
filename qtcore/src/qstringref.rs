

// mod ::core::QStringRef
// package qtcore
// /usr/include/qt/QtCore/qstring.h
// #include <qstring.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 42
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
#[derive(Default)] // class sizeof(QStringRef)=16
pub struct QStringRef {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QStringRef_ITF interface {
//    QStringRef_PTR() *QStringRef
//}
//func (ptr *QStringRef) QStringRef_PTR() *QStringRef { return ptr }

impl /*struct*/ QStringRef {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QStringRef {
    return QStringRef{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QStringRef {
//  type Target = QStringRefBASE;
//
//  fn deref(&self) -> &QStringRefBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QStringRefBASE> for QStringRef {
//  fn as_ref(& self) -> & QStringRefBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qstring.h:1420
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void QStringRef()

/*

*/
// QStringRef() ctx.fn_proto_cpp
impl /*struct*/ QStringRef {
  pub fn QStringRef_0<T: QStringRef_QStringRef_0>(value: T) -> QStringRef {
    let rsthis = value.QStringRef_0();
    return rsthis;
    // return 1;
  }
}

pub trait QStringRef_QStringRef_0 {
  fn QStringRef_0(self) -> QStringRef;
}
// QStringRef() ctx.fn_proto_cpp
impl<'a> /*trait*/ QStringRef_QStringRef_0 for () {
  fn QStringRef_0(self) -> QStringRef {
    // unsafe{_ZN10QStringRefC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN10QStringRefC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QStringRef{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qstring.h:1421
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void QStringRef(const QString *, int, int)

/*

*/
// QStringRef(const QString *, int, int) ctx.fn_proto_cpp
impl /*struct*/ QStringRef {
  pub fn QStringRef_1<T: QStringRef_QStringRef_1>(value: T) -> QStringRef {
    let rsthis = value.QStringRef_1();
    return rsthis;
    // return 1;
  }
}

pub trait QStringRef_QStringRef_1 {
  fn QStringRef_1(self) -> QStringRef;
}
// QStringRef(const QString *, int, int) ctx.fn_proto_cpp
impl<'a> /*trait*/ QStringRef_QStringRef_1 for (usize,i32,i32) {
  fn QStringRef_1(self) -> QStringRef {
    // unsafe{_ZN10QStringRefC2EPK7QStringii()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN10QStringRefC2EPK7QStringii", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QStringRef{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qstring.h:1422
// index:2
// Public inline Visibility=Default Availability=Available
// [-2] void QStringRef(const QString *)

/*

*/
// QStringRef(const QString *) ctx.fn_proto_cpp
impl /*struct*/ QStringRef {
  pub fn QStringRef_2<T: QStringRef_QStringRef_2>(value: T) -> QStringRef {
    let rsthis = value.QStringRef_2();
    return rsthis;
    // return 1;
  }
}

pub trait QStringRef_QStringRef_2 {
  fn QStringRef_2(self) -> QStringRef;
}
// QStringRef(const QString *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QStringRef_QStringRef_2 for (usize) {
  fn QStringRef_2(self) -> QStringRef {
    // unsafe{_ZN10QStringRefC2EPK7QString()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN10QStringRefC2EPK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QStringRef{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qstring.h:1431
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QStringRef & operator=(QStringRef &&)

/*

*/
impl /*struct*/ QStringRef {
  pub fn operator_equal_0<RetType, T: QStringRef_operator_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_0(self);
    // return 1;
  }
}
pub trait QStringRef_operator_equal_0<RetType> {
  fn operator_equal_0(self , rsthis: & QStringRef) -> RetType;
}
impl<'a> /*trait*/ QStringRef_operator_equal_0<usize> for (usize) {
  fn operator_equal_0(self , rsthis: & QStringRef) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QStringRefaSEOS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1433
// index:1
// Public inline Visibility=Default Availability=Available
// [16] QStringRef & operator=(const QStringRef &)

/*

*/
impl /*struct*/ QStringRef {
  pub fn operator_equal_1<RetType, T: QStringRef_operator_equal_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_1(self);
    // return 1;
  }
}
pub trait QStringRef_operator_equal_1<RetType> {
  fn operator_equal_1(self , rsthis: & QStringRef) -> RetType;
}
impl<'a> /*trait*/ QStringRef_operator_equal_1<usize> for (usize) {
  fn operator_equal_1(self , rsthis: & QStringRef) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QStringRefaSERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1505
// index:2
// Public inline Visibility=Default Availability=Available
// [16] QStringRef & operator=(const QString *)

/*

*/
impl /*struct*/ QStringRef {
  pub fn operator_equal_2<RetType, T: QStringRef_operator_equal_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_2(self);
    // return 1;
  }
}
pub trait QStringRef_operator_equal_2<RetType> {
  fn operator_equal_2(self , rsthis: & QStringRef) -> RetType;
}
impl<'a> /*trait*/ QStringRef_operator_equal_2<usize> for (usize) {
  fn operator_equal_2(self , rsthis: & QStringRef) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QStringRefaSEPK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1438
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void ~QStringRef()

/*

*/
pub fn DeleteQStringRef(this :*mut QStringRef) {
    // let rv = qtrt::InvokeQtFunc6("_ZN10QStringRefD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qstring.h:1441
// index:0
// Public inline Visibility=Default Availability=Available
// [8] const QString * string() const

/*

*/
impl /*struct*/ QStringRef {
  pub fn string_0<RetType, T: QStringRef_string_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.string_0(self);
    // return 1;
  }
}
pub trait QStringRef_string_0<RetType> {
  fn string_0(self , rsthis: & QStringRef) -> RetType;
}
impl<'a> /*trait*/ QStringRef_string_0<usize> for () {
  fn string_0(self , rsthis: & QStringRef) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QStringRef6stringEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1442
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int position() const

/*

*/
impl /*struct*/ QStringRef {
  pub fn position_0<RetType, T: QStringRef_position_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.position_0(self);
    // return 1;
  }
}
pub trait QStringRef_position_0<RetType> {
  fn position_0(self , rsthis: & QStringRef) -> RetType;
}
impl<'a> /*trait*/ QStringRef_position_0<i32> for () {
  fn position_0(self , rsthis: & QStringRef) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QStringRef8positionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1443
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int size() const

/*
Returns the number of characters in this string.

The last character in the string is at position size() - 1.

Example:


  QString str = "World";
  int n = str.size();         // n == 5
  str.data()[0];              // returns 'W'
  str.data()[4];              // returns 'd'



See also isEmpty() and resize().
*/
impl /*struct*/ QStringRef {
  pub fn size_0<RetType, T: QStringRef_size_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.size_0(self);
    // return 1;
  }
}
pub trait QStringRef_size_0<RetType> {
  fn size_0(self , rsthis: & QStringRef) -> RetType;
}
impl<'a> /*trait*/ QStringRef_size_0<i32> for () {
  fn size_0(self , rsthis: & QStringRef) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QStringRef4sizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1444
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int count() const

/*
Returns the number of (potentially overlapping) occurrences of the string str in this string.

If cs is Qt::CaseSensitive (default), the search is case sensitive; otherwise the search is case insensitive.

See also contains() and indexOf().
*/
impl /*struct*/ QStringRef {
  pub fn count_0<RetType, T: QStringRef_count_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.count_0(self);
    // return 1;
  }
}
pub trait QStringRef_count_0<RetType> {
  fn count_0(self , rsthis: & QStringRef) -> RetType;
}
impl<'a> /*trait*/ QStringRef_count_0<i32> for () {
  fn count_0(self , rsthis: & QStringRef) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QStringRef5countEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1461
// index:1
// Public Visibility=Default Availability=Available
// [4] int count(const QString &, Qt::CaseSensitivity) const

/*
Returns the number of (potentially overlapping) occurrences of the string str in this string.

If cs is Qt::CaseSensitive (default), the search is case sensitive; otherwise the search is case insensitive.

See also contains() and indexOf().
*/
impl /*struct*/ QStringRef {
  pub fn count_1<RetType, T: QStringRef_count_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.count_1(self);
    // return 1;
  }
}
pub trait QStringRef_count_1<RetType> {
  fn count_1(self , rsthis: & QStringRef) -> RetType;
}
impl<'a> /*trait*/ QStringRef_count_1<i32> for (usize,i32) {
  fn count_1(self , rsthis: & QStringRef) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QStringRef5countERK7QStringN2Qt15CaseSensitivityE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1462
// index:2
// Public Visibility=Default Availability=Available
// [4] int count(QChar, Qt::CaseSensitivity) const

/*
Returns the number of (potentially overlapping) occurrences of the string str in this string.

If cs is Qt::CaseSensitive (default), the search is case sensitive; otherwise the search is case insensitive.

See also contains() and indexOf().
*/
impl /*struct*/ QStringRef {
  pub fn count_2<RetType, T: QStringRef_count_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.count_2(self);
    // return 1;
  }
}
pub trait QStringRef_count_2<RetType> {
  fn count_2(self , rsthis: & QStringRef) -> RetType;
}
impl<'a> /*trait*/ QStringRef_count_2<i32> for (usize,i32) {
  fn count_2(self , rsthis: & QStringRef) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QStringRef5countE5QCharN2Qt15CaseSensitivityE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1463
// index:3
// Public Visibility=Default Availability=Available
// [4] int count(const QStringRef &, Qt::CaseSensitivity) const

/*
Returns the number of (potentially overlapping) occurrences of the string str in this string.

If cs is Qt::CaseSensitive (default), the search is case sensitive; otherwise the search is case insensitive.

See also contains() and indexOf().
*/
impl /*struct*/ QStringRef {
  pub fn count_3<RetType, T: QStringRef_count_3<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.count_3(self);
    // return 1;
  }
}
pub trait QStringRef_count_3<RetType> {
  fn count_3(self , rsthis: & QStringRef) -> RetType;
}
impl<'a> /*trait*/ QStringRef_count_3<i32> for (usize,i32) {
  fn count_3(self , rsthis: & QStringRef) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QStringRef5countERKS_N2Qt15CaseSensitivityE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1445
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int length() const

/*
Returns the number of characters in this string. Equivalent to size().

See also resize().
*/
impl /*struct*/ QStringRef {
  pub fn length_0<RetType, T: QStringRef_length_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.length_0(self);
    // return 1;
  }
}
pub trait QStringRef_length_0<RetType> {
  fn length_0(self , rsthis: & QStringRef) -> RetType;
}
impl<'a> /*trait*/ QStringRef_length_0<i32> for () {
  fn length_0(self , rsthis: & QStringRef) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QStringRef6lengthEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1447
// index:0
// Public Visibility=Default Availability=Available
// [4] int indexOf(const QString &, int, Qt::CaseSensitivity) const

/*
Returns the index position of the first occurrence of the string str in this string, searching forward from index position from. Returns -1 if str is not found.

If cs is Qt::CaseSensitive (default), the search is case sensitive; otherwise the search is case insensitive.

Example:


  QString x = "sticky question";
  QString y = "sti";
  x.indexOf(y);               // returns 0
  x.indexOf(y, 1);            // returns 10
  x.indexOf(y, 10);           // returns 10
  x.indexOf(y, 11);           // returns -1



If from is -1, the search starts at the last character; if it is -2, at the next to last character and so on.

See also lastIndexOf(), contains(), and count().
*/
impl /*struct*/ QStringRef {
  pub fn indexOf_0<RetType, T: QStringRef_indexOf_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.indexOf_0(self);
    // return 1;
  }
}
pub trait QStringRef_indexOf_0<RetType> {
  fn indexOf_0(self , rsthis: & QStringRef) -> RetType;
}
impl<'a> /*trait*/ QStringRef_indexOf_0<i32> for (usize,i32,i32) {
  fn indexOf_0(self , rsthis: & QStringRef) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QStringRef7indexOfERK7QStringiN2Qt15CaseSensitivityE", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1448
// index:1
// Public Visibility=Default Availability=Available
// [4] int indexOf(QChar, int, Qt::CaseSensitivity) const

/*
Returns the index position of the first occurrence of the string str in this string, searching forward from index position from. Returns -1 if str is not found.

If cs is Qt::CaseSensitive (default), the search is case sensitive; otherwise the search is case insensitive.

Example:


  QString x = "sticky question";
  QString y = "sti";
  x.indexOf(y);               // returns 0
  x.indexOf(y, 1);            // returns 10
  x.indexOf(y, 10);           // returns 10
  x.indexOf(y, 11);           // returns -1



If from is -1, the search starts at the last character; if it is -2, at the next to last character and so on.

See also lastIndexOf(), contains(), and count().
*/
impl /*struct*/ QStringRef {
  pub fn indexOf_1<RetType, T: QStringRef_indexOf_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.indexOf_1(self);
    // return 1;
  }
}
pub trait QStringRef_indexOf_1<RetType> {
  fn indexOf_1(self , rsthis: & QStringRef) -> RetType;
}
impl<'a> /*trait*/ QStringRef_indexOf_1<i32> for (usize,i32,i32) {
  fn indexOf_1(self , rsthis: & QStringRef) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QStringRef7indexOfE5QChariN2Qt15CaseSensitivityE", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1449
// index:2
// Public Visibility=Default Availability=Available
// [4] int indexOf(QLatin1String, int, Qt::CaseSensitivity) const

/*
Returns the index position of the first occurrence of the string str in this string, searching forward from index position from. Returns -1 if str is not found.

If cs is Qt::CaseSensitive (default), the search is case sensitive; otherwise the search is case insensitive.

Example:


  QString x = "sticky question";
  QString y = "sti";
  x.indexOf(y);               // returns 0
  x.indexOf(y, 1);            // returns 10
  x.indexOf(y, 10);           // returns 10
  x.indexOf(y, 11);           // returns -1



If from is -1, the search starts at the last character; if it is -2, at the next to last character and so on.

See also lastIndexOf(), contains(), and count().
*/
impl /*struct*/ QStringRef {
  pub fn indexOf_2<RetType, T: QStringRef_indexOf_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.indexOf_2(self);
    // return 1;
  }
}
pub trait QStringRef_indexOf_2<RetType> {
  fn indexOf_2(self , rsthis: & QStringRef) -> RetType;
}
impl<'a> /*trait*/ QStringRef_indexOf_2<i32> for (usize,i32,i32) {
  fn indexOf_2(self , rsthis: & QStringRef) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QStringRef7indexOfE13QLatin1StringiN2Qt15CaseSensitivityE", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1450
// index:3
// Public Visibility=Default Availability=Available
// [4] int indexOf(const QStringRef &, int, Qt::CaseSensitivity) const

/*
Returns the index position of the first occurrence of the string str in this string, searching forward from index position from. Returns -1 if str is not found.

If cs is Qt::CaseSensitive (default), the search is case sensitive; otherwise the search is case insensitive.

Example:


  QString x = "sticky question";
  QString y = "sti";
  x.indexOf(y);               // returns 0
  x.indexOf(y, 1);            // returns 10
  x.indexOf(y, 10);           // returns 10
  x.indexOf(y, 11);           // returns -1



If from is -1, the search starts at the last character; if it is -2, at the next to last character and so on.

See also lastIndexOf(), contains(), and count().
*/
impl /*struct*/ QStringRef {
  pub fn indexOf_3<RetType, T: QStringRef_indexOf_3<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.indexOf_3(self);
    // return 1;
  }
}
pub trait QStringRef_indexOf_3<RetType> {
  fn indexOf_3(self , rsthis: & QStringRef) -> RetType;
}
impl<'a> /*trait*/ QStringRef_indexOf_3<i32> for (usize,i32,i32) {
  fn indexOf_3(self , rsthis: & QStringRef) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QStringRef7indexOfERKS_iN2Qt15CaseSensitivityE", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1451
// index:0
// Public Visibility=Default Availability=Available
// [4] int lastIndexOf(const QString &, int, Qt::CaseSensitivity) const

/*
Returns the index position of the last occurrence of the string str in this string, searching backward from index position from. If from is -1 (default), the search starts at the last character; if from is -2, at the next to last character and so on. Returns -1 if str is not found.

If cs is Qt::CaseSensitive (default), the search is case sensitive; otherwise the search is case insensitive.

Example:


  QString x = "crazy azimuths";
  QString y = "az";
  x.lastIndexOf(y);           // returns 6
  x.lastIndexOf(y, 6);        // returns 6
  x.lastIndexOf(y, 5);        // returns 2
  x.lastIndexOf(y, 1);        // returns -1



See also indexOf(), contains(), and count().
*/
impl /*struct*/ QStringRef {
  pub fn lastIndexOf_0<RetType, T: QStringRef_lastIndexOf_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.lastIndexOf_0(self);
    // return 1;
  }
}
pub trait QStringRef_lastIndexOf_0<RetType> {
  fn lastIndexOf_0(self , rsthis: & QStringRef) -> RetType;
}
impl<'a> /*trait*/ QStringRef_lastIndexOf_0<i32> for (usize,i32,i32) {
  fn lastIndexOf_0(self , rsthis: & QStringRef) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QStringRef11lastIndexOfERK7QStringiN2Qt15CaseSensitivityE", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1452
// index:1
// Public Visibility=Default Availability=Available
// [4] int lastIndexOf(QChar, int, Qt::CaseSensitivity) const

/*
Returns the index position of the last occurrence of the string str in this string, searching backward from index position from. If from is -1 (default), the search starts at the last character; if from is -2, at the next to last character and so on. Returns -1 if str is not found.

If cs is Qt::CaseSensitive (default), the search is case sensitive; otherwise the search is case insensitive.

Example:


  QString x = "crazy azimuths";
  QString y = "az";
  x.lastIndexOf(y);           // returns 6
  x.lastIndexOf(y, 6);        // returns 6
  x.lastIndexOf(y, 5);        // returns 2
  x.lastIndexOf(y, 1);        // returns -1



See also indexOf(), contains(), and count().
*/
impl /*struct*/ QStringRef {
  pub fn lastIndexOf_1<RetType, T: QStringRef_lastIndexOf_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.lastIndexOf_1(self);
    // return 1;
  }
}
pub trait QStringRef_lastIndexOf_1<RetType> {
  fn lastIndexOf_1(self , rsthis: & QStringRef) -> RetType;
}
impl<'a> /*trait*/ QStringRef_lastIndexOf_1<i32> for (usize,i32,i32) {
  fn lastIndexOf_1(self , rsthis: & QStringRef) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QStringRef11lastIndexOfE5QChariN2Qt15CaseSensitivityE", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1453
// index:2
// Public Visibility=Default Availability=Available
// [4] int lastIndexOf(QLatin1String, int, Qt::CaseSensitivity) const

/*
Returns the index position of the last occurrence of the string str in this string, searching backward from index position from. If from is -1 (default), the search starts at the last character; if from is -2, at the next to last character and so on. Returns -1 if str is not found.

If cs is Qt::CaseSensitive (default), the search is case sensitive; otherwise the search is case insensitive.

Example:


  QString x = "crazy azimuths";
  QString y = "az";
  x.lastIndexOf(y);           // returns 6
  x.lastIndexOf(y, 6);        // returns 6
  x.lastIndexOf(y, 5);        // returns 2
  x.lastIndexOf(y, 1);        // returns -1



See also indexOf(), contains(), and count().
*/
impl /*struct*/ QStringRef {
  pub fn lastIndexOf_2<RetType, T: QStringRef_lastIndexOf_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.lastIndexOf_2(self);
    // return 1;
  }
}
pub trait QStringRef_lastIndexOf_2<RetType> {
  fn lastIndexOf_2(self , rsthis: & QStringRef) -> RetType;
}
impl<'a> /*trait*/ QStringRef_lastIndexOf_2<i32> for (usize,i32,i32) {
  fn lastIndexOf_2(self , rsthis: & QStringRef) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QStringRef11lastIndexOfE13QLatin1StringiN2Qt15CaseSensitivityE", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1454
// index:3
// Public Visibility=Default Availability=Available
// [4] int lastIndexOf(const QStringRef &, int, Qt::CaseSensitivity) const

/*
Returns the index position of the last occurrence of the string str in this string, searching backward from index position from. If from is -1 (default), the search starts at the last character; if from is -2, at the next to last character and so on. Returns -1 if str is not found.

If cs is Qt::CaseSensitive (default), the search is case sensitive; otherwise the search is case insensitive.

Example:


  QString x = "crazy azimuths";
  QString y = "az";
  x.lastIndexOf(y);           // returns 6
  x.lastIndexOf(y, 6);        // returns 6
  x.lastIndexOf(y, 5);        // returns 2
  x.lastIndexOf(y, 1);        // returns -1



See also indexOf(), contains(), and count().
*/
impl /*struct*/ QStringRef {
  pub fn lastIndexOf_3<RetType, T: QStringRef_lastIndexOf_3<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.lastIndexOf_3(self);
    // return 1;
  }
}
pub trait QStringRef_lastIndexOf_3<RetType> {
  fn lastIndexOf_3(self , rsthis: & QStringRef) -> RetType;
}
impl<'a> /*trait*/ QStringRef_lastIndexOf_3<i32> for (usize,i32,i32) {
  fn lastIndexOf_3(self , rsthis: & QStringRef) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QStringRef11lastIndexOfERKS_iN2Qt15CaseSensitivityE", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1456
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool contains(const QString &, Qt::CaseSensitivity) const

/*
Returns true if this string contains an occurrence of the string str; otherwise returns false.

If cs is Qt::CaseSensitive (default), the search is case sensitive; otherwise the search is case insensitive.

Example:


  QString str = "Peter Pan";
  str.contains("peter", Qt::CaseInsensitive);    // returns true



See also indexOf() and count().
*/
impl /*struct*/ QStringRef {
  pub fn contains_0<RetType, T: QStringRef_contains_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.contains_0(self);
    // return 1;
  }
}
pub trait QStringRef_contains_0<RetType> {
  fn contains_0(self , rsthis: & QStringRef) -> RetType;
}
impl<'a> /*trait*/ QStringRef_contains_0<bool> for (usize,i32) {
  fn contains_0(self , rsthis: & QStringRef) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QStringRef8containsERK7QStringN2Qt15CaseSensitivityE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1457
// index:1
// Public inline Visibility=Default Availability=Available
// [1] bool contains(QChar, Qt::CaseSensitivity) const

/*
Returns true if this string contains an occurrence of the string str; otherwise returns false.

If cs is Qt::CaseSensitive (default), the search is case sensitive; otherwise the search is case insensitive.

Example:


  QString str = "Peter Pan";
  str.contains("peter", Qt::CaseInsensitive);    // returns true



See also indexOf() and count().
*/
impl /*struct*/ QStringRef {
  pub fn contains_1<RetType, T: QStringRef_contains_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.contains_1(self);
    // return 1;
  }
}
pub trait QStringRef_contains_1<RetType> {
  fn contains_1(self , rsthis: & QStringRef) -> RetType;
}
impl<'a> /*trait*/ QStringRef_contains_1<bool> for (usize,i32) {
  fn contains_1(self , rsthis: & QStringRef) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QStringRef8containsE5QCharN2Qt15CaseSensitivityE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1458
// index:2
// Public inline Visibility=Default Availability=Available
// [1] bool contains(QLatin1String, Qt::CaseSensitivity) const

/*
Returns true if this string contains an occurrence of the string str; otherwise returns false.

If cs is Qt::CaseSensitive (default), the search is case sensitive; otherwise the search is case insensitive.

Example:


  QString str = "Peter Pan";
  str.contains("peter", Qt::CaseInsensitive);    // returns true



See also indexOf() and count().
*/
impl /*struct*/ QStringRef {
  pub fn contains_2<RetType, T: QStringRef_contains_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.contains_2(self);
    // return 1;
  }
}
pub trait QStringRef_contains_2<RetType> {
  fn contains_2(self , rsthis: & QStringRef) -> RetType;
}
impl<'a> /*trait*/ QStringRef_contains_2<bool> for (usize,i32) {
  fn contains_2(self , rsthis: & QStringRef) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QStringRef8containsE13QLatin1StringN2Qt15CaseSensitivityE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1459
// index:3
// Public inline Visibility=Default Availability=Available
// [1] bool contains(const QStringRef &, Qt::CaseSensitivity) const

/*
Returns true if this string contains an occurrence of the string str; otherwise returns false.

If cs is Qt::CaseSensitive (default), the search is case sensitive; otherwise the search is case insensitive.

Example:


  QString str = "Peter Pan";
  str.contains("peter", Qt::CaseInsensitive);    // returns true



See also indexOf() and count().
*/
impl /*struct*/ QStringRef {
  pub fn contains_3<RetType, T: QStringRef_contains_3<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.contains_3(self);
    // return 1;
  }
}
pub trait QStringRef_contains_3<RetType> {
  fn contains_3(self , rsthis: & QStringRef) -> RetType;
}
impl<'a> /*trait*/ QStringRef_contains_3<bool> for (usize,i32) {
  fn contains_3(self , rsthis: & QStringRef) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QStringRef8containsERKS_N2Qt15CaseSensitivityE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1470
// index:0
// Public Visibility=Default Availability=Available
// [16] QStringRef left(int) const

/*
Returns a substring that contains the n leftmost characters of the string.

The entire string is returned if n is greater than or equal to size(), or less than zero.


  QString x = "Pineapple";
  QString y = x.left(4);      // y == "Pine"



See also right(), mid(), startsWith(), chopped(), chop(), and truncate().
*/
impl /*struct*/ QStringRef {
  pub fn left_0<RetType, T: QStringRef_left_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.left_0(self);
    // return 1;
  }
}
pub trait QStringRef_left_0<RetType> {
  fn left_0(self , rsthis: & QStringRef) -> RetType;
}
impl<'a> /*trait*/ QStringRef_left_0<usize> for (i32) {
  fn left_0(self , rsthis: & QStringRef) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QStringRef4leftEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1471
// index:0
// Public Visibility=Default Availability=Available
// [16] QStringRef right(int) const

/*
Returns a substring that contains the n rightmost characters of the string.

The entire string is returned if n is greater than or equal to size(), or less than zero.


  QString x = "Pineapple";
  QString y = x.right(5);      // y == "apple"



See also left(), mid(), endsWith(), chopped(), chop(), and truncate().
*/
impl /*struct*/ QStringRef {
  pub fn right_0<RetType, T: QStringRef_right_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.right_0(self);
    // return 1;
  }
}
pub trait QStringRef_right_0<RetType> {
  fn right_0(self , rsthis: & QStringRef) -> RetType;
}
impl<'a> /*trait*/ QStringRef_right_0<usize> for (i32) {
  fn right_0(self , rsthis: & QStringRef) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QStringRef5rightEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1472
// index:0
// Public Visibility=Default Availability=Available
// [16] QStringRef mid(int, int) const

/*
Returns a string that contains n characters of this string, starting at the specified position index.

Returns a null string if the position index exceeds the length of the string. If there are less than n characters available in the string starting at the given position, or if n is -1 (default), the function returns all characters that are available from the specified position.

Example:


  QString x = "Nine pineapples";
  QString y = x.mid(5, 4);            // y == "pine"
  QString z = x.mid(5);               // z == "pineapples"



See also left(), right(), chopped(), chop(), and truncate().
*/
impl /*struct*/ QStringRef {
  pub fn mid_0<RetType, T: QStringRef_mid_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mid_0(self);
    // return 1;
  }
}
pub trait QStringRef_mid_0<RetType> {
  fn mid_0(self , rsthis: & QStringRef) -> RetType;
}
impl<'a> /*trait*/ QStringRef_mid_0<usize> for (i32,i32) {
  fn mid_0(self , rsthis: & QStringRef) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QStringRef3midEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1473
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QStringRef chopped(int) const

/*
Returns a substring that contains the size() - len leftmost characters of this string.

Note: The behavior is undefined if len is negative or greater than size().

This function was introduced in  Qt 5.10.

See also endsWith(), left(), right(), mid(), chop(), and truncate().
*/
impl /*struct*/ QStringRef {
  pub fn chopped_0<RetType, T: QStringRef_chopped_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.chopped_0(self);
    // return 1;
  }
}
pub trait QStringRef_chopped_0<RetType> {
  fn chopped_0(self , rsthis: & QStringRef) -> RetType;
}
impl<'a> /*trait*/ QStringRef_chopped_0<usize> for (i32) {
  fn chopped_0(self , rsthis: & QStringRef) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QStringRef7choppedEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1476
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void truncate(int)

/*
Truncates the string at the given position index.

If the specified position index is beyond the end of the string, nothing happens.

Example:


  QString str = "Vladivostok";
  str.truncate(4);
  // str == "Vlad"



If position is negative, it is equivalent to passing zero.

See also chop(), resize(), left(), and QStringRef::truncate().
*/
impl /*struct*/ QStringRef {
  pub fn truncate_0<RetType, T: QStringRef_truncate_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.truncate_0(self);
    // return 1;
  }
}
pub trait QStringRef_truncate_0<RetType> {
  fn truncate_0(self , rsthis: & QStringRef) -> RetType;
}
impl<'a> /*trait*/ QStringRef_truncate_0<(/*void*/)> for (i32) {
  fn truncate_0(self , rsthis: & QStringRef) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QStringRef8truncateEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qstring.h:1477
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void chop(int)

/*
Removes n characters from the end of the string.

If n is greater than or equal to size(), the result is an empty string; if n is negative, it is equivalent to passing zero.

Example:


  QString str("LOGOUT\r\n");
  str.chop(2);
  // str == "LOGOUT"



If you want to remove characters from the beginning of the string, use remove() instead.

See also truncate(), resize(), remove(), and QStringRef::chop().
*/
impl /*struct*/ QStringRef {
  pub fn chop_0<RetType, T: QStringRef_chop_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.chop_0(self);
    // return 1;
  }
}
pub trait QStringRef_chop_0<RetType> {
  fn chop_0(self , rsthis: & QStringRef) -> RetType;
}
impl<'a> /*trait*/ QStringRef_chop_0<(/*void*/)> for (i32) {
  fn chop_0(self , rsthis: & QStringRef) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QStringRef4chopEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qstring.h:1485
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isRightToLeft() const

/*
Returns true if the string is read right to left.

See also QStringRef::isRightToLeft().
*/
impl /*struct*/ QStringRef {
  pub fn isRightToLeft_0<RetType, T: QStringRef_isRightToLeft_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isRightToLeft_0(self);
    // return 1;
  }
}
pub trait QStringRef_isRightToLeft_0<RetType> {
  fn isRightToLeft_0(self , rsthis: & QStringRef) -> RetType;
}
impl<'a> /*trait*/ QStringRef_isRightToLeft_0<bool> for () {
  fn isRightToLeft_0(self , rsthis: & QStringRef) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QStringRef13isRightToLeftEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1487
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool startsWith(QStringView, Qt::CaseSensitivity) const

/*
Returns true if the string starts with s; otherwise returns false.

If cs is Qt::CaseSensitive (default), the search is case sensitive; otherwise the search is case insensitive.


  QString str = "Bananas";
  str.startsWith("Ban");     // returns true
  str.startsWith("Car");     // returns false



See also endsWith().
*/
impl /*struct*/ QStringRef {
  pub fn startsWith_0<RetType, T: QStringRef_startsWith_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.startsWith_0(self);
    // return 1;
  }
}
pub trait QStringRef_startsWith_0<RetType> {
  fn startsWith_0(self , rsthis: & QStringRef) -> RetType;
}
impl<'a> /*trait*/ QStringRef_startsWith_0<bool> for (usize,i32) {
  fn startsWith_0(self , rsthis: & QStringRef) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QStringRef10startsWithE11QStringViewN2Qt15CaseSensitivityE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1489
// index:1
// Public Visibility=Default Availability=Available
// [1] bool startsWith(QLatin1String, Qt::CaseSensitivity) const

/*
Returns true if the string starts with s; otherwise returns false.

If cs is Qt::CaseSensitive (default), the search is case sensitive; otherwise the search is case insensitive.


  QString str = "Bananas";
  str.startsWith("Ban");     // returns true
  str.startsWith("Car");     // returns false



See also endsWith().
*/
impl /*struct*/ QStringRef {
  pub fn startsWith_1<RetType, T: QStringRef_startsWith_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.startsWith_1(self);
    // return 1;
  }
}
pub trait QStringRef_startsWith_1<RetType> {
  fn startsWith_1(self , rsthis: & QStringRef) -> RetType;
}
impl<'a> /*trait*/ QStringRef_startsWith_1<bool> for (usize,i32) {
  fn startsWith_1(self , rsthis: & QStringRef) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QStringRef10startsWithE13QLatin1StringN2Qt15CaseSensitivityE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1490
// index:2
// Public Visibility=Default Availability=Available
// [1] bool startsWith(QChar, Qt::CaseSensitivity) const

/*
Returns true if the string starts with s; otherwise returns false.

If cs is Qt::CaseSensitive (default), the search is case sensitive; otherwise the search is case insensitive.


  QString str = "Bananas";
  str.startsWith("Ban");     // returns true
  str.startsWith("Car");     // returns false



See also endsWith().
*/
impl /*struct*/ QStringRef {
  pub fn startsWith_2<RetType, T: QStringRef_startsWith_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.startsWith_2(self);
    // return 1;
  }
}
pub trait QStringRef_startsWith_2<RetType> {
  fn startsWith_2(self , rsthis: & QStringRef) -> RetType;
}
impl<'a> /*trait*/ QStringRef_startsWith_2<bool> for (usize,i32) {
  fn startsWith_2(self , rsthis: & QStringRef) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QStringRef10startsWithE5QCharN2Qt15CaseSensitivityE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1492
// index:3
// Public Visibility=Default Availability=Available
// [1] bool startsWith(const QString &, Qt::CaseSensitivity) const

/*
Returns true if the string starts with s; otherwise returns false.

If cs is Qt::CaseSensitive (default), the search is case sensitive; otherwise the search is case insensitive.


  QString str = "Bananas";
  str.startsWith("Ban");     // returns true
  str.startsWith("Car");     // returns false



See also endsWith().
*/
impl /*struct*/ QStringRef {
  pub fn startsWith_3<RetType, T: QStringRef_startsWith_3<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.startsWith_3(self);
    // return 1;
  }
}
pub trait QStringRef_startsWith_3<RetType> {
  fn startsWith_3(self , rsthis: & QStringRef) -> RetType;
}
impl<'a> /*trait*/ QStringRef_startsWith_3<bool> for (usize,i32) {
  fn startsWith_3(self , rsthis: & QStringRef) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QStringRef10startsWithERK7QStringN2Qt15CaseSensitivityE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1493
// index:4
// Public Visibility=Default Availability=Available
// [1] bool startsWith(const QStringRef &, Qt::CaseSensitivity) const

/*
Returns true if the string starts with s; otherwise returns false.

If cs is Qt::CaseSensitive (default), the search is case sensitive; otherwise the search is case insensitive.


  QString str = "Bananas";
  str.startsWith("Ban");     // returns true
  str.startsWith("Car");     // returns false



See also endsWith().
*/
impl /*struct*/ QStringRef {
  pub fn startsWith_4<RetType, T: QStringRef_startsWith_4<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.startsWith_4(self);
    // return 1;
  }
}
pub trait QStringRef_startsWith_4<RetType> {
  fn startsWith_4(self , rsthis: & QStringRef) -> RetType;
}
impl<'a> /*trait*/ QStringRef_startsWith_4<bool> for (usize,i32) {
  fn startsWith_4(self , rsthis: & QStringRef) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QStringRef10startsWithERKS_N2Qt15CaseSensitivityE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1496
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool endsWith(QStringView, Qt::CaseSensitivity) const

/*
Returns true if the string ends with s; otherwise returns false.

If cs is Qt::CaseSensitive (default), the search is case sensitive; otherwise the search is case insensitive.


  QString str = "Bananas";
  str.endsWith("anas");         // returns true
  str.endsWith("pple");         // returns false



See also startsWith().
*/
impl /*struct*/ QStringRef {
  pub fn endsWith_0<RetType, T: QStringRef_endsWith_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.endsWith_0(self);
    // return 1;
  }
}
pub trait QStringRef_endsWith_0<RetType> {
  fn endsWith_0(self , rsthis: & QStringRef) -> RetType;
}
impl<'a> /*trait*/ QStringRef_endsWith_0<bool> for (usize,i32) {
  fn endsWith_0(self , rsthis: & QStringRef) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QStringRef8endsWithE11QStringViewN2Qt15CaseSensitivityE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1498
// index:1
// Public Visibility=Default Availability=Available
// [1] bool endsWith(QLatin1String, Qt::CaseSensitivity) const

/*
Returns true if the string ends with s; otherwise returns false.

If cs is Qt::CaseSensitive (default), the search is case sensitive; otherwise the search is case insensitive.


  QString str = "Bananas";
  str.endsWith("anas");         // returns true
  str.endsWith("pple");         // returns false



See also startsWith().
*/
impl /*struct*/ QStringRef {
  pub fn endsWith_1<RetType, T: QStringRef_endsWith_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.endsWith_1(self);
    // return 1;
  }
}
pub trait QStringRef_endsWith_1<RetType> {
  fn endsWith_1(self , rsthis: & QStringRef) -> RetType;
}
impl<'a> /*trait*/ QStringRef_endsWith_1<bool> for (usize,i32) {
  fn endsWith_1(self , rsthis: & QStringRef) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QStringRef8endsWithE13QLatin1StringN2Qt15CaseSensitivityE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1499
// index:2
// Public Visibility=Default Availability=Available
// [1] bool endsWith(QChar, Qt::CaseSensitivity) const

/*
Returns true if the string ends with s; otherwise returns false.

If cs is Qt::CaseSensitive (default), the search is case sensitive; otherwise the search is case insensitive.


  QString str = "Bananas";
  str.endsWith("anas");         // returns true
  str.endsWith("pple");         // returns false



See also startsWith().
*/
impl /*struct*/ QStringRef {
  pub fn endsWith_2<RetType, T: QStringRef_endsWith_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.endsWith_2(self);
    // return 1;
  }
}
pub trait QStringRef_endsWith_2<RetType> {
  fn endsWith_2(self , rsthis: & QStringRef) -> RetType;
}
impl<'a> /*trait*/ QStringRef_endsWith_2<bool> for (usize,i32) {
  fn endsWith_2(self , rsthis: & QStringRef) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QStringRef8endsWithE5QCharN2Qt15CaseSensitivityE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1501
// index:3
// Public Visibility=Default Availability=Available
// [1] bool endsWith(const QString &, Qt::CaseSensitivity) const

/*
Returns true if the string ends with s; otherwise returns false.

If cs is Qt::CaseSensitive (default), the search is case sensitive; otherwise the search is case insensitive.


  QString str = "Bananas";
  str.endsWith("anas");         // returns true
  str.endsWith("pple");         // returns false



See also startsWith().
*/
impl /*struct*/ QStringRef {
  pub fn endsWith_3<RetType, T: QStringRef_endsWith_3<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.endsWith_3(self);
    // return 1;
  }
}
pub trait QStringRef_endsWith_3<RetType> {
  fn endsWith_3(self , rsthis: & QStringRef) -> RetType;
}
impl<'a> /*trait*/ QStringRef_endsWith_3<bool> for (usize,i32) {
  fn endsWith_3(self , rsthis: & QStringRef) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QStringRef8endsWithERK7QStringN2Qt15CaseSensitivityE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1502
// index:4
// Public Visibility=Default Availability=Available
// [1] bool endsWith(const QStringRef &, Qt::CaseSensitivity) const

/*
Returns true if the string ends with s; otherwise returns false.

If cs is Qt::CaseSensitive (default), the search is case sensitive; otherwise the search is case insensitive.


  QString str = "Bananas";
  str.endsWith("anas");         // returns true
  str.endsWith("pple");         // returns false



See also startsWith().
*/
impl /*struct*/ QStringRef {
  pub fn endsWith_4<RetType, T: QStringRef_endsWith_4<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.endsWith_4(self);
    // return 1;
  }
}
pub trait QStringRef_endsWith_4<RetType> {
  fn endsWith_4(self , rsthis: & QStringRef) -> RetType;
}
impl<'a> /*trait*/ QStringRef_endsWith_4<bool> for (usize,i32) {
  fn endsWith_4(self , rsthis: & QStringRef) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QStringRef8endsWithERKS_N2Qt15CaseSensitivityE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1507
// index:0
// Public inline Visibility=Default Availability=Available
// [8] const QChar * unicode() const

/*
Returns a Unicode representation of the string. The result remains valid until the string is modified.

Note: The returned string may not be '\0'-terminated. Use size() to determine the length of the array.

See also setUnicode(), utf16(), and fromRawData().
*/
impl /*struct*/ QStringRef {
  pub fn unicode_0<RetType, T: QStringRef_unicode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.unicode_0(self);
    // return 1;
  }
}
pub trait QStringRef_unicode_0<RetType> {
  fn unicode_0(self , rsthis: & QStringRef) -> RetType;
}
impl<'a> /*trait*/ QStringRef_unicode_0<usize> for () {
  fn unicode_0(self , rsthis: & QStringRef) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QStringRef7unicodeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1513
// index:0
// Public inline Visibility=Default Availability=Available
// [8] const QChar * data() const

/*
Returns a pointer to the data stored in the QString. The pointer can be used to access and modify the characters that compose the string.

Unlike constData() and unicode(), the returned data is always '\0'-terminated.

Example:


  QString str = "Hello world";
  QChar *data = str.data();
  while (!data->isNull()) {
      qDebug() << data->unicode();
      ++data;
  }



Note that the pointer remains valid only as long as the string is not modified by other means. For read-only access, constData() is faster because it never causes a deep copy to occur.

See also constData() and operator[]().
*/
impl /*struct*/ QStringRef {
  pub fn data_0<RetType, T: QStringRef_data_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.data_0(self);
    // return 1;
  }
}
pub trait QStringRef_data_0<RetType> {
  fn data_0(self , rsthis: & QStringRef) -> RetType;
}
impl<'a> /*trait*/ QStringRef_data_0<usize> for () {
  fn data_0(self , rsthis: & QStringRef) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QStringRef4dataEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1514
// index:0
// Public inline Visibility=Default Availability=Available
// [8] const QChar * constData() const

/*
Returns a pointer to the data stored in the QString. The pointer can be used to access the characters that compose the string.

Note that the pointer remains valid only as long as the string is not modified.

Note: The returned string may not be '\0'-terminated. Use size() to determine the length of the array.

See also data(), operator[](), and fromRawData().
*/
impl /*struct*/ QStringRef {
  pub fn constData_0<RetType, T: QStringRef_constData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.constData_0(self);
    // return 1;
  }
}
pub trait QStringRef_constData_0<RetType> {
  fn constData_0(self , rsthis: & QStringRef) -> RetType;
}
impl<'a> /*trait*/ QStringRef_constData_0<usize> for () {
  fn constData_0(self , rsthis: & QStringRef) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QStringRef9constDataEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1516
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QStringRef::const_iterator begin() const

/*
Returns an STL-style iterator pointing to the first character in the string.

See also constBegin() and end().
*/
impl /*struct*/ QStringRef {
  pub fn begin_0<RetType, T: QStringRef_begin_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.begin_0(self);
    // return 1;
  }
}
pub trait QStringRef_begin_0<RetType> {
  fn begin_0(self , rsthis: & QStringRef) -> RetType;
}
impl<'a> /*trait*/ QStringRef_begin_0<usize> for () {
  fn begin_0(self , rsthis: & QStringRef) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QStringRef5beginEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1517
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QStringRef::const_iterator cbegin() const

/*
Returns a const STL-style iterator pointing to the first character in the string.

This function was introduced in  Qt 5.0.

See also begin() and cend().
*/
impl /*struct*/ QStringRef {
  pub fn cbegin_0<RetType, T: QStringRef_cbegin_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cbegin_0(self);
    // return 1;
  }
}
pub trait QStringRef_cbegin_0<RetType> {
  fn cbegin_0(self , rsthis: & QStringRef) -> RetType;
}
impl<'a> /*trait*/ QStringRef_cbegin_0<usize> for () {
  fn cbegin_0(self , rsthis: & QStringRef) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QStringRef6cbeginEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1518
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QStringRef::const_iterator constBegin() const

/*
Returns a const STL-style iterator pointing to the first character in the string.

See also begin() and constEnd().
*/
impl /*struct*/ QStringRef {
  pub fn constBegin_0<RetType, T: QStringRef_constBegin_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.constBegin_0(self);
    // return 1;
  }
}
pub trait QStringRef_constBegin_0<RetType> {
  fn constBegin_0(self , rsthis: & QStringRef) -> RetType;
}
impl<'a> /*trait*/ QStringRef_constBegin_0<usize> for () {
  fn constBegin_0(self , rsthis: & QStringRef) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QStringRef10constBeginEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1519
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QStringRef::const_iterator end() const

/*
Returns an STL-style iterator pointing to the imaginary character after the last character in the string.

See also begin() and constEnd().
*/
impl /*struct*/ QStringRef {
  pub fn end_0<RetType, T: QStringRef_end_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.end_0(self);
    // return 1;
  }
}
pub trait QStringRef_end_0<RetType> {
  fn end_0(self , rsthis: & QStringRef) -> RetType;
}
impl<'a> /*trait*/ QStringRef_end_0<usize> for () {
  fn end_0(self , rsthis: & QStringRef) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QStringRef3endEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1520
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QStringRef::const_iterator cend() const

/*
Returns a const STL-style iterator pointing to the imaginary character after the last character in the list.

This function was introduced in  Qt 5.0.

See also cbegin() and end().
*/
impl /*struct*/ QStringRef {
  pub fn cend_0<RetType, T: QStringRef_cend_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cend_0(self);
    // return 1;
  }
}
pub trait QStringRef_cend_0<RetType> {
  fn cend_0(self , rsthis: & QStringRef) -> RetType;
}
impl<'a> /*trait*/ QStringRef_cend_0<usize> for () {
  fn cend_0(self , rsthis: & QStringRef) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QStringRef4cendEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1521
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QStringRef::const_iterator constEnd() const

/*
Returns a const STL-style iterator pointing to the imaginary character after the last character in the list.

See also constBegin() and end().
*/
impl /*struct*/ QStringRef {
  pub fn constEnd_0<RetType, T: QStringRef_constEnd_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.constEnd_0(self);
    // return 1;
  }
}
pub trait QStringRef_constEnd_0<RetType> {
  fn constEnd_0(self , rsthis: & QStringRef) -> RetType;
}
impl<'a> /*trait*/ QStringRef_constEnd_0<usize> for () {
  fn constEnd_0(self , rsthis: & QStringRef) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QStringRef8constEndEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1531
// index:0
// Public Visibility=Default Availability=Available
// [8] QByteArray toLatin1() const

/*
Returns a Latin-1 representation of the string as a QByteArray.

The returned byte array is undefined if the string contains non-Latin1 characters. Those characters may be suppressed or replaced with a question mark.

See also fromLatin1(), toUtf8(), toLocal8Bit(), QTextCodec, and qConvertToLatin1().
*/
impl /*struct*/ QStringRef {
  pub fn toLatin1_0<RetType, T: QStringRef_toLatin1_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toLatin1_0(self);
    // return 1;
  }
}
pub trait QStringRef_toLatin1_0<RetType> {
  fn toLatin1_0(self , rsthis: & QStringRef) -> RetType;
}
impl<'a> /*trait*/ QStringRef_toLatin1_0<usize> for () {
  fn toLatin1_0(self , rsthis: & QStringRef) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QStringRef8toLatin1Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1532
// index:0
// Public Visibility=Default Availability=Available
// [8] QByteArray toUtf8() const

/*
Returns a UTF-8 representation of the string as a QByteArray.

UTF-8 is a Unicode codec and can represent all characters in a Unicode string like QString.

See also fromUtf8(), toLatin1(), toLocal8Bit(), QTextCodec, and qConvertToUtf8().
*/
impl /*struct*/ QStringRef {
  pub fn toUtf8_0<RetType, T: QStringRef_toUtf8_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toUtf8_0(self);
    // return 1;
  }
}
pub trait QStringRef_toUtf8_0<RetType> {
  fn toUtf8_0(self , rsthis: & QStringRef) -> RetType;
}
impl<'a> /*trait*/ QStringRef_toUtf8_0<usize> for () {
  fn toUtf8_0(self , rsthis: & QStringRef) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QStringRef6toUtf8Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1533
// index:0
// Public Visibility=Default Availability=Available
// [8] QByteArray toLocal8Bit() const

/*
Returns the local 8-bit representation of the string as a QByteArray. The returned byte array is undefined if the string contains characters not supported by the local 8-bit encoding.

QTextCodec::codecForLocale() is used to perform the conversion from Unicode. If the locale encoding could not be determined, this function does the same as toLatin1().

If this string contains any characters that cannot be encoded in the locale, the returned byte array is undefined. Those characters may be suppressed or replaced by another.

See also fromLocal8Bit(), toLatin1(), toUtf8(), and QTextCodec.
*/
impl /*struct*/ QStringRef {
  pub fn toLocal8Bit_0<RetType, T: QStringRef_toLocal8Bit_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toLocal8Bit_0(self);
    // return 1;
  }
}
pub trait QStringRef_toLocal8Bit_0<RetType> {
  fn toLocal8Bit_0(self , rsthis: & QStringRef) -> RetType;
}
impl<'a> /*trait*/ QStringRef_toLocal8Bit_0<usize> for () {
  fn toLocal8Bit_0(self , rsthis: & QStringRef) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QStringRef11toLocal8BitEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1536
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void clear()

/*
Clears the contents of the string and makes it null.

See also resize() and isNull().
*/
impl /*struct*/ QStringRef {
  pub fn clear_0<RetType, T: QStringRef_clear_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.clear_0(self);
    // return 1;
  }
}
pub trait QStringRef_clear_0<RetType> {
  fn clear_0(self , rsthis: & QStringRef) -> RetType;
}
impl<'a> /*trait*/ QStringRef_clear_0<(/*void*/)> for () {
  fn clear_0(self , rsthis: & QStringRef) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN10QStringRef5clearEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qstring.h:1537
// index:0
// Public Visibility=Default Availability=Available
// [8] QString toString() const

/*

*/
impl /*struct*/ QStringRef {
  pub fn toString_0<RetType, T: QStringRef_toString_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toString_0(self);
    // return 1;
  }
}
pub trait QStringRef_toString_0<RetType> {
  fn toString_0(self , rsthis: & QStringRef) -> RetType;
}
impl<'a> /*trait*/ QStringRef_toString_0<usize> for () {
  fn toString_0(self , rsthis: & QStringRef) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QStringRef8toStringEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1538
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isEmpty() const

/*
Returns true if the string has no characters; otherwise returns false.

Example:


  QString().isEmpty();            // returns true
  QString("").isEmpty();          // returns true
  QString("x").isEmpty();         // returns false
  QString("abc").isEmpty();       // returns false



See also size().
*/
impl /*struct*/ QStringRef {
  pub fn isEmpty_0<RetType, T: QStringRef_isEmpty_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isEmpty_0(self);
    // return 1;
  }
}
pub trait QStringRef_isEmpty_0<RetType> {
  fn isEmpty_0(self , rsthis: & QStringRef) -> RetType;
}
impl<'a> /*trait*/ QStringRef_isEmpty_0<bool> for () {
  fn isEmpty_0(self , rsthis: & QStringRef) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QStringRef7isEmptyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1539
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isNull() const

/*
Returns true if this string is null; otherwise returns false.

Example:


  QString().isNull();             // returns true
  QString("").isNull();           // returns false
  QString("abc").isNull();        // returns false



Qt makes a distinction between null strings and empty strings for historical reasons. For most applications, what matters is whether or not a string contains any data, and this can be determined using the isEmpty() function.

See also isEmpty().
*/
impl /*struct*/ QStringRef {
  pub fn isNull_0<RetType, T: QStringRef_isNull_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isNull_0(self);
    // return 1;
  }
}
pub trait QStringRef_isNull_0<RetType> {
  fn isNull_0(self , rsthis: & QStringRef) -> RetType;
}
impl<'a> /*trait*/ QStringRef_isNull_0<bool> for () {
  fn isNull_0(self , rsthis: & QStringRef) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QStringRef6isNullEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1541
// index:0
// Public Visibility=Default Availability=Available
// [16] QStringRef appendTo(QString *) const

/*

*/
impl /*struct*/ QStringRef {
  pub fn appendTo_0<RetType, T: QStringRef_appendTo_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.appendTo_0(self);
    // return 1;
  }
}
pub trait QStringRef_appendTo_0<RetType> {
  fn appendTo_0(self , rsthis: & QStringRef) -> RetType;
}
impl<'a> /*trait*/ QStringRef_appendTo_0<usize> for (usize) {
  fn appendTo_0(self , rsthis: & QStringRef) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QStringRef8appendToEP7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1543
// index:0
// Public inline Visibility=Default Availability=Available
// [2] const QChar at(int) const

/*
Returns the character at the given index position in the string.

The position must be a valid index position in the string (i.e., 0 <= position < size()).

See also operator[]().
*/
impl /*struct*/ QStringRef {
  pub fn at_0<RetType, T: QStringRef_at_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.at_0(self);
    // return 1;
  }
}
pub trait QStringRef_at_0<RetType> {
  fn at_0(self , rsthis: & QStringRef) -> RetType;
}
impl<'a> /*trait*/ QStringRef_at_0<usize> for (i32) {
  fn at_0(self , rsthis: & QStringRef) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QStringRef2atEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1545
// index:0
// Public inline Visibility=Default Availability=Available
// [2] QChar operator[](int) const

/*

*/
impl /*struct*/ QStringRef {
  pub fn operator_get_index_0<RetType, T: QStringRef_operator_get_index_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_get_index_0(self);
    // return 1;
  }
}
pub trait QStringRef_operator_get_index_0<RetType> {
  fn operator_get_index_0(self , rsthis: & QStringRef) -> RetType;
}
impl<'a> /*trait*/ QStringRef_operator_get_index_0<usize> for (i32) {
  fn operator_get_index_0(self , rsthis: & QStringRef) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QStringRefixEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1546
// index:0
// Public inline Visibility=Default Availability=Available
// [2] QChar front() const

/*
Returns the first character in the string. Same as at(0).

This function is provided for STL compatibility.

Warning: Calling this function on an empty string constitutes undefined behavior.

This function was introduced in  Qt 5.10.

See also back(), at(), and operator[]().
*/
impl /*struct*/ QStringRef {
  pub fn front_0<RetType, T: QStringRef_front_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.front_0(self);
    // return 1;
  }
}
pub trait QStringRef_front_0<RetType> {
  fn front_0(self , rsthis: & QStringRef) -> RetType;
}
impl<'a> /*trait*/ QStringRef_front_0<usize> for () {
  fn front_0(self , rsthis: & QStringRef) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QStringRef5frontEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1547
// index:0
// Public inline Visibility=Default Availability=Available
// [2] QChar back() const

/*
Returns the last character in the string. Same as at(size() - 1).

This function is provided for STL compatibility.

Warning: Calling this function on an empty string constitutes undefined behavior.

This function was introduced in  Qt 5.10.

See also front(), at(), and operator[]().
*/
impl /*struct*/ QStringRef {
  pub fn back_0<RetType, T: QStringRef_back_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.back_0(self);
    // return 1;
  }
}
pub trait QStringRef_back_0<RetType> {
  fn back_0(self , rsthis: & QStringRef) -> RetType;
}
impl<'a> /*trait*/ QStringRef_back_0<usize> for () {
  fn back_0(self , rsthis: & QStringRef) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QStringRef4backEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1551
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator==(const char *) const

/*

*/
impl /*struct*/ QStringRef {
  pub fn operator_equal_equal_0<RetType, T: QStringRef_operator_equal_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_equal_0(self);
    // return 1;
  }
}
pub trait QStringRef_operator_equal_equal_0<RetType> {
  fn operator_equal_equal_0(self , rsthis: & QStringRef) -> RetType;
}
impl<'a> /*trait*/ QStringRef_operator_equal_equal_0<bool> for (usize) {
  fn operator_equal_equal_0(self , rsthis: & QStringRef) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QStringRefeqEPKc", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1552
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator!=(const char *) const

/*

*/
impl /*struct*/ QStringRef {
  pub fn operator_not_equal_0<RetType, T: QStringRef_operator_not_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_not_equal_0(self);
    // return 1;
  }
}
pub trait QStringRef_operator_not_equal_0<RetType> {
  fn operator_not_equal_0(self , rsthis: & QStringRef) -> RetType;
}
impl<'a> /*trait*/ QStringRef_operator_not_equal_0<bool> for (usize) {
  fn operator_not_equal_0(self , rsthis: & QStringRef) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QStringRefneEPKc", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1553
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator<(const char *) const

/*

*/
impl /*struct*/ QStringRef {
  pub fn operator_less_than_0<RetType, T: QStringRef_operator_less_than_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_less_than_0(self);
    // return 1;
  }
}
pub trait QStringRef_operator_less_than_0<RetType> {
  fn operator_less_than_0(self , rsthis: & QStringRef) -> RetType;
}
impl<'a> /*trait*/ QStringRef_operator_less_than_0<bool> for (usize) {
  fn operator_less_than_0(self , rsthis: & QStringRef) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QStringRefltEPKc", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1554
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator<=(const char *) const

/*

*/
impl /*struct*/ QStringRef {
  pub fn operator_less_than_equal_0<RetType, T: QStringRef_operator_less_than_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_less_than_equal_0(self);
    // return 1;
  }
}
pub trait QStringRef_operator_less_than_equal_0<RetType> {
  fn operator_less_than_equal_0(self , rsthis: & QStringRef) -> RetType;
}
impl<'a> /*trait*/ QStringRef_operator_less_than_equal_0<bool> for (usize) {
  fn operator_less_than_equal_0(self , rsthis: & QStringRef) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QStringRefleEPKc", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1555
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator>(const char *) const

/*

*/
impl /*struct*/ QStringRef {
  pub fn operator_greater_than_0<RetType, T: QStringRef_operator_greater_than_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_greater_than_0(self);
    // return 1;
  }
}
pub trait QStringRef_operator_greater_than_0<RetType> {
  fn operator_greater_than_0(self , rsthis: & QStringRef) -> RetType;
}
impl<'a> /*trait*/ QStringRef_operator_greater_than_0<bool> for (usize) {
  fn operator_greater_than_0(self , rsthis: & QStringRef) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QStringRefgtEPKc", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1556
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator>=(const char *) const

/*

*/
impl /*struct*/ QStringRef {
  pub fn operator_greater_than_equal_0<RetType, T: QStringRef_operator_greater_than_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_greater_than_equal_0(self);
    // return 1;
  }
}
pub trait QStringRef_operator_greater_than_equal_0<RetType> {
  fn operator_greater_than_equal_0(self , rsthis: & QStringRef) -> RetType;
}
impl<'a> /*trait*/ QStringRef_operator_greater_than_equal_0<bool> for (usize) {
  fn operator_greater_than_equal_0(self , rsthis: & QStringRef) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QStringRefgeEPKc", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1559
// index:0
// Public Visibility=Default Availability=Available
// [4] int compare(const QString &, Qt::CaseSensitivity) const

/*
Compares s1 with s2 and returns an integer less than, equal to, or greater than zero if s1 is less than, equal to, or greater than s2.

If cs is Qt::CaseSensitive, the comparison is case sensitive; otherwise the comparison is case insensitive.

Case sensitive comparison is based exclusively on the numeric Unicode values of the characters and is very fast, but is not what a human would expect. Consider sorting user-visible strings with localeAwareCompare().


  int x = QString::compare("aUtO", "AuTo", Qt::CaseInsensitive);  // x == 0
  int y = QString::compare("auto", "Car", Qt::CaseSensitive);     // y > 0
  int z = QString::compare("auto", "Car", Qt::CaseInsensitive);   // z < 0



This function was introduced in  Qt 4.2.

See also operator==(), operator<(), and operator>().
*/
impl /*struct*/ QStringRef {
  pub fn compare_0<RetType, T: QStringRef_compare_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.compare_0(self);
    // return 1;
  }
}
pub trait QStringRef_compare_0<RetType> {
  fn compare_0(self , rsthis: & QStringRef) -> RetType;
}
impl<'a> /*trait*/ QStringRef_compare_0<i32> for (usize,i32) {
  fn compare_0(self , rsthis: & QStringRef) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QStringRef7compareERK7QStringN2Qt15CaseSensitivityE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1560
// index:1
// Public Visibility=Default Availability=Available
// [4] int compare(const QStringRef &, Qt::CaseSensitivity) const

/*
Compares s1 with s2 and returns an integer less than, equal to, or greater than zero if s1 is less than, equal to, or greater than s2.

If cs is Qt::CaseSensitive, the comparison is case sensitive; otherwise the comparison is case insensitive.

Case sensitive comparison is based exclusively on the numeric Unicode values of the characters and is very fast, but is not what a human would expect. Consider sorting user-visible strings with localeAwareCompare().


  int x = QString::compare("aUtO", "AuTo", Qt::CaseInsensitive);  // x == 0
  int y = QString::compare("auto", "Car", Qt::CaseSensitive);     // y > 0
  int z = QString::compare("auto", "Car", Qt::CaseInsensitive);   // z < 0



This function was introduced in  Qt 4.2.

See also operator==(), operator<(), and operator>().
*/
impl /*struct*/ QStringRef {
  pub fn compare_1<RetType, T: QStringRef_compare_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.compare_1(self);
    // return 1;
  }
}
pub trait QStringRef_compare_1<RetType> {
  fn compare_1(self , rsthis: & QStringRef) -> RetType;
}
impl<'a> /*trait*/ QStringRef_compare_1<i32> for (usize,i32) {
  fn compare_1(self , rsthis: & QStringRef) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QStringRef7compareERKS_N2Qt15CaseSensitivityE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1561
// index:2
// Public Visibility=Default Availability=Available
// [4] int compare(QLatin1String, Qt::CaseSensitivity) const

/*
Compares s1 with s2 and returns an integer less than, equal to, or greater than zero if s1 is less than, equal to, or greater than s2.

If cs is Qt::CaseSensitive, the comparison is case sensitive; otherwise the comparison is case insensitive.

Case sensitive comparison is based exclusively on the numeric Unicode values of the characters and is very fast, but is not what a human would expect. Consider sorting user-visible strings with localeAwareCompare().


  int x = QString::compare("aUtO", "AuTo", Qt::CaseInsensitive);  // x == 0
  int y = QString::compare("auto", "Car", Qt::CaseSensitive);     // y > 0
  int z = QString::compare("auto", "Car", Qt::CaseInsensitive);   // z < 0



This function was introduced in  Qt 4.2.

See also operator==(), operator<(), and operator>().
*/
impl /*struct*/ QStringRef {
  pub fn compare_2<RetType, T: QStringRef_compare_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.compare_2(self);
    // return 1;
  }
}
pub trait QStringRef_compare_2<RetType> {
  fn compare_2(self , rsthis: & QStringRef) -> RetType;
}
impl<'a> /*trait*/ QStringRef_compare_2<i32> for (usize,i32) {
  fn compare_2(self , rsthis: & QStringRef) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QStringRef7compareE13QLatin1StringN2Qt15CaseSensitivityE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1563
// index:3
// Public inline Visibility=Default Availability=Available
// [4] int compare(const QByteArray &, Qt::CaseSensitivity) const

/*
Compares s1 with s2 and returns an integer less than, equal to, or greater than zero if s1 is less than, equal to, or greater than s2.

If cs is Qt::CaseSensitive, the comparison is case sensitive; otherwise the comparison is case insensitive.

Case sensitive comparison is based exclusively on the numeric Unicode values of the characters and is very fast, but is not what a human would expect. Consider sorting user-visible strings with localeAwareCompare().


  int x = QString::compare("aUtO", "AuTo", Qt::CaseInsensitive);  // x == 0
  int y = QString::compare("auto", "Car", Qt::CaseSensitive);     // y > 0
  int z = QString::compare("auto", "Car", Qt::CaseInsensitive);   // z < 0



This function was introduced in  Qt 4.2.

See also operator==(), operator<(), and operator>().
*/
impl /*struct*/ QStringRef {
  pub fn compare_3<RetType, T: QStringRef_compare_3<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.compare_3(self);
    // return 1;
  }
}
pub trait QStringRef_compare_3<RetType> {
  fn compare_3(self , rsthis: & QStringRef) -> RetType;
}
impl<'a> /*trait*/ QStringRef_compare_3<i32> for (usize,i32) {
  fn compare_3(self , rsthis: & QStringRef) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QStringRef7compareERK10QByteArrayN2Qt15CaseSensitivityE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1566
// index:4
// Public static Visibility=Default Availability=Available
// [4] int compare(const QStringRef &, const QString &, Qt::CaseSensitivity)

/*
Compares s1 with s2 and returns an integer less than, equal to, or greater than zero if s1 is less than, equal to, or greater than s2.

If cs is Qt::CaseSensitive, the comparison is case sensitive; otherwise the comparison is case insensitive.

Case sensitive comparison is based exclusively on the numeric Unicode values of the characters and is very fast, but is not what a human would expect. Consider sorting user-visible strings with localeAwareCompare().


  int x = QString::compare("aUtO", "AuTo", Qt::CaseInsensitive);  // x == 0
  int y = QString::compare("auto", "Car", Qt::CaseSensitive);     // y > 0
  int z = QString::compare("auto", "Car", Qt::CaseInsensitive);   // z < 0



This function was introduced in  Qt 4.2.

See also operator==(), operator<(), and operator>().
*/
impl /*struct*/ QStringRef {
  pub fn compare_4<RetType, T: QStringRef_compare_4<RetType>>( overload_args: T) -> RetType {
    return overload_args.compare_4();
    // return 1;
  }
}
pub trait QStringRef_compare_4<RetType> {
  fn compare_4(self ) -> RetType;
}
impl<'a> /*trait*/ QStringRef_compare_4<i32> for (usize,usize,i32) {
  fn compare_4(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QStringRef7compareERKS_RK7QStringN2Qt15CaseSensitivityE", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1568
// index:5
// Public static Visibility=Default Availability=Available
// [4] int compare(const QStringRef &, const QStringRef &, Qt::CaseSensitivity)

/*
Compares s1 with s2 and returns an integer less than, equal to, or greater than zero if s1 is less than, equal to, or greater than s2.

If cs is Qt::CaseSensitive, the comparison is case sensitive; otherwise the comparison is case insensitive.

Case sensitive comparison is based exclusively on the numeric Unicode values of the characters and is very fast, but is not what a human would expect. Consider sorting user-visible strings with localeAwareCompare().


  int x = QString::compare("aUtO", "AuTo", Qt::CaseInsensitive);  // x == 0
  int y = QString::compare("auto", "Car", Qt::CaseSensitive);     // y > 0
  int z = QString::compare("auto", "Car", Qt::CaseInsensitive);   // z < 0



This function was introduced in  Qt 4.2.

See also operator==(), operator<(), and operator>().
*/
impl /*struct*/ QStringRef {
  pub fn compare_5<RetType, T: QStringRef_compare_5<RetType>>( overload_args: T) -> RetType {
    return overload_args.compare_5();
    // return 1;
  }
}
pub trait QStringRef_compare_5<RetType> {
  fn compare_5(self ) -> RetType;
}
impl<'a> /*trait*/ QStringRef_compare_5<i32> for (usize,usize,i32) {
  fn compare_5(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QStringRef7compareERKS_S1_N2Qt15CaseSensitivityE", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1570
// index:6
// Public static Visibility=Default Availability=Available
// [4] int compare(const QStringRef &, QLatin1String, Qt::CaseSensitivity)

/*
Compares s1 with s2 and returns an integer less than, equal to, or greater than zero if s1 is less than, equal to, or greater than s2.

If cs is Qt::CaseSensitive, the comparison is case sensitive; otherwise the comparison is case insensitive.

Case sensitive comparison is based exclusively on the numeric Unicode values of the characters and is very fast, but is not what a human would expect. Consider sorting user-visible strings with localeAwareCompare().


  int x = QString::compare("aUtO", "AuTo", Qt::CaseInsensitive);  // x == 0
  int y = QString::compare("auto", "Car", Qt::CaseSensitive);     // y > 0
  int z = QString::compare("auto", "Car", Qt::CaseInsensitive);   // z < 0



This function was introduced in  Qt 4.2.

See also operator==(), operator<(), and operator>().
*/
impl /*struct*/ QStringRef {
  pub fn compare_6<RetType, T: QStringRef_compare_6<RetType>>( overload_args: T) -> RetType {
    return overload_args.compare_6();
    // return 1;
  }
}
pub trait QStringRef_compare_6<RetType> {
  fn compare_6(self ) -> RetType;
}
impl<'a> /*trait*/ QStringRef_compare_6<i32> for (usize,usize,i32) {
  fn compare_6(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QStringRef7compareERKS_13QLatin1StringN2Qt15CaseSensitivityE", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1573
// index:0
// Public Visibility=Default Availability=Available
// [4] int localeAwareCompare(const QString &) const

/*
Compares s1 with s2 and returns an integer less than, equal to, or greater than zero if s1 is less than, equal to, or greater than s2.

The comparison is performed in a locale- and also platform-dependent manner. Use this function to present sorted lists of strings to the user.

On macOS and iOS this function compares according the "Order for sorted lists" setting in the International preferences panel.

See also compare() and QLocale.
*/
impl /*struct*/ QStringRef {
  pub fn localeAwareCompare_0<RetType, T: QStringRef_localeAwareCompare_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.localeAwareCompare_0(self);
    // return 1;
  }
}
pub trait QStringRef_localeAwareCompare_0<RetType> {
  fn localeAwareCompare_0(self , rsthis: & QStringRef) -> RetType;
}
impl<'a> /*trait*/ QStringRef_localeAwareCompare_0<i32> for (usize) {
  fn localeAwareCompare_0(self , rsthis: & QStringRef) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QStringRef18localeAwareCompareERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1574
// index:1
// Public Visibility=Default Availability=Available
// [4] int localeAwareCompare(const QStringRef &) const

/*
Compares s1 with s2 and returns an integer less than, equal to, or greater than zero if s1 is less than, equal to, or greater than s2.

The comparison is performed in a locale- and also platform-dependent manner. Use this function to present sorted lists of strings to the user.

On macOS and iOS this function compares according the "Order for sorted lists" setting in the International preferences panel.

See also compare() and QLocale.
*/
impl /*struct*/ QStringRef {
  pub fn localeAwareCompare_1<RetType, T: QStringRef_localeAwareCompare_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.localeAwareCompare_1(self);
    // return 1;
  }
}
pub trait QStringRef_localeAwareCompare_1<RetType> {
  fn localeAwareCompare_1(self , rsthis: & QStringRef) -> RetType;
}
impl<'a> /*trait*/ QStringRef_localeAwareCompare_1<i32> for (usize) {
  fn localeAwareCompare_1(self , rsthis: & QStringRef) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QStringRef18localeAwareCompareERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1575
// index:2
// Public static Visibility=Default Availability=Available
// [4] int localeAwareCompare(const QStringRef &, const QString &)

/*
Compares s1 with s2 and returns an integer less than, equal to, or greater than zero if s1 is less than, equal to, or greater than s2.

The comparison is performed in a locale- and also platform-dependent manner. Use this function to present sorted lists of strings to the user.

On macOS and iOS this function compares according the "Order for sorted lists" setting in the International preferences panel.

See also compare() and QLocale.
*/
impl /*struct*/ QStringRef {
  pub fn localeAwareCompare_2<RetType, T: QStringRef_localeAwareCompare_2<RetType>>( overload_args: T) -> RetType {
    return overload_args.localeAwareCompare_2();
    // return 1;
  }
}
pub trait QStringRef_localeAwareCompare_2<RetType> {
  fn localeAwareCompare_2(self ) -> RetType;
}
impl<'a> /*trait*/ QStringRef_localeAwareCompare_2<i32> for (usize,usize) {
  fn localeAwareCompare_2(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QStringRef18localeAwareCompareERKS_RK7QString", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1576
// index:3
// Public static Visibility=Default Availability=Available
// [4] int localeAwareCompare(const QStringRef &, const QStringRef &)

/*
Compares s1 with s2 and returns an integer less than, equal to, or greater than zero if s1 is less than, equal to, or greater than s2.

The comparison is performed in a locale- and also platform-dependent manner. Use this function to present sorted lists of strings to the user.

On macOS and iOS this function compares according the "Order for sorted lists" setting in the International preferences panel.

See also compare() and QLocale.
*/
impl /*struct*/ QStringRef {
  pub fn localeAwareCompare_3<RetType, T: QStringRef_localeAwareCompare_3<RetType>>( overload_args: T) -> RetType {
    return overload_args.localeAwareCompare_3();
    // return 1;
  }
}
pub trait QStringRef_localeAwareCompare_3<RetType> {
  fn localeAwareCompare_3(self ) -> RetType;
}
impl<'a> /*trait*/ QStringRef_localeAwareCompare_3<i32> for (usize,usize) {
  fn localeAwareCompare_3(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QStringRef18localeAwareCompareERKS_S1_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1578
// index:0
// Public Visibility=Default Availability=Available
// [16] QStringRef trimmed() const

/*
Returns a string that has whitespace removed from the start and the end.

Whitespace means any character for which QChar::isSpace() returns true. This includes the ASCII characters '\t', '\n', '\v', '\f', '\r', and ' '.

Example:


  QString str = "  lots\t of\nwhitespace\r\n ";
  str = str.trimmed();
  // str == "lots\t of\nwhitespace"



Unlike simplified(), trimmed() leaves internal whitespace alone.

See also simplified().
*/
impl /*struct*/ QStringRef {
  pub fn trimmed_0<RetType, T: QStringRef_trimmed_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.trimmed_0(self);
    // return 1;
  }
}
pub trait QStringRef_trimmed_0<RetType> {
  fn trimmed_0(self , rsthis: & QStringRef) -> RetType;
}
impl<'a> /*trait*/ QStringRef_trimmed_0<usize> for () {
  fn trimmed_0(self , rsthis: & QStringRef) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QStringRef7trimmedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1579
// index:0
// Public Visibility=Default Availability=Available
// [2] short toShort(bool *, int) const

/*
Returns the string converted to a short using base base, which is 10 by default and must be between 2 and 36, or 0. Returns 0 if the conversion fails.

If a conversion error occurs, *ok is set to false; otherwise *ok is set to true.

If base is 0, the C language convention is used: If the string begins with "0x", base 16 is used; if the string begins with "0", base 8 is used; otherwise, base 10 is used.

The string conversion will always happen in the 'C' locale. For locale dependent conversion use QLocale::toShort()

Example:


  QString str = "FF";
  bool ok;

  short hex = str.toShort(&ok, 16);   // hex == 255, ok == true
  short dec = str.toShort(&ok, 10);   // dec == 0, ok == false



See also number(), toUShort(), toInt(), and QLocale::toShort().
*/
impl /*struct*/ QStringRef {
  pub fn toShort_0<RetType, T: QStringRef_toShort_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toShort_0(self);
    // return 1;
  }
}
pub trait QStringRef_toShort_0<RetType> {
  fn toShort_0(self , rsthis: & QStringRef) -> RetType;
}
impl<'a> /*trait*/ QStringRef_toShort_0<i16> for (usize,i32) {
  fn toShort_0(self , rsthis: & QStringRef) -> i16 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QStringRef7toShortEPbi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i16 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1580
// index:0
// Public Visibility=Default Availability=Available
// [2] ushort toUShort(bool *, int) const

/*
Returns the string converted to an unsigned short using base base, which is 10 by default and must be between 2 and 36, or 0. Returns 0 if the conversion fails.

If a conversion error occurs, *ok is set to false; otherwise *ok is set to true.

If base is 0, the C language convention is used: If the string begins with "0x", base 16 is used; if the string begins with "0", base 8 is used; otherwise, base 10 is used.

The string conversion will always happen in the 'C' locale. For locale dependent conversion use QLocale::toUShort()

Example:


  QString str = "FF";
  bool ok;

  ushort hex = str.toUShort(&ok, 16);     // hex == 255, ok == true
  ushort dec = str.toUShort(&ok, 10);     // dec == 0, ok == false



See also number(), toShort(), and QLocale::toUShort().
*/
impl /*struct*/ QStringRef {
  pub fn toUShort_0<RetType, T: QStringRef_toUShort_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toUShort_0(self);
    // return 1;
  }
}
pub trait QStringRef_toUShort_0<RetType> {
  fn toUShort_0(self , rsthis: & QStringRef) -> RetType;
}
impl<'a> /*trait*/ QStringRef_toUShort_0<u16> for (usize,i32) {
  fn toUShort_0(self , rsthis: & QStringRef) -> u16 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QStringRef8toUShortEPbi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u16 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1581
// index:0
// Public Visibility=Default Availability=Available
// [4] int toInt(bool *, int) const

/*
Returns the string converted to an int using base base, which is 10 by default and must be between 2 and 36, or 0. Returns 0 if the conversion fails.

If a conversion error occurs, *ok is set to false; otherwise *ok is set to true.

If base is 0, the C language convention is used: If the string begins with "0x", base 16 is used; if the string begins with "0", base 8 is used; otherwise, base 10 is used.

The string conversion will always happen in the 'C' locale. For locale dependent conversion use QLocale::toInt()

Example:


  QString str = "FF";
  bool ok;
  int hex = str.toInt(&ok, 16);       // hex == 255, ok == true
  int dec = str.toInt(&ok, 10);       // dec == 0, ok == false



See also number(), toUInt(), toDouble(), and QLocale::toInt().
*/
impl /*struct*/ QStringRef {
  pub fn toInt_0<RetType, T: QStringRef_toInt_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toInt_0(self);
    // return 1;
  }
}
pub trait QStringRef_toInt_0<RetType> {
  fn toInt_0(self , rsthis: & QStringRef) -> RetType;
}
impl<'a> /*trait*/ QStringRef_toInt_0<i32> for (usize,i32) {
  fn toInt_0(self , rsthis: & QStringRef) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QStringRef5toIntEPbi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1582
// index:0
// Public Visibility=Default Availability=Available
// [4] uint toUInt(bool *, int) const

/*
Returns the string converted to an unsigned int using base base, which is 10 by default and must be between 2 and 36, or 0. Returns 0 if the conversion fails.

If a conversion error occurs, *ok is set to false; otherwise *ok is set to true.

If base is 0, the C language convention is used: If the string begins with "0x", base 16 is used; if the string begins with "0", base 8 is used; otherwise, base 10 is used.

The string conversion will always happen in the 'C' locale. For locale dependent conversion use QLocale::toUInt()

Example:


  QString str = "FF";
  bool ok;

  uint hex = str.toUInt(&ok, 16);     // hex == 255, ok == true
  uint dec = str.toUInt(&ok, 10);     // dec == 0, ok == false



See also number(), toInt(), and QLocale::toUInt().
*/
impl /*struct*/ QStringRef {
  pub fn toUInt_0<RetType, T: QStringRef_toUInt_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toUInt_0(self);
    // return 1;
  }
}
pub trait QStringRef_toUInt_0<RetType> {
  fn toUInt_0(self , rsthis: & QStringRef) -> RetType;
}
impl<'a> /*trait*/ QStringRef_toUInt_0<u32> for (usize,i32) {
  fn toUInt_0(self , rsthis: & QStringRef) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QStringRef6toUIntEPbi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1583
// index:0
// Public Visibility=Default Availability=Available
// [8] long toLong(bool *, int) const

/*
Returns the string converted to a long using base base, which is 10 by default and must be between 2 and 36, or 0. Returns 0 if the conversion fails.

If a conversion error occurs, *ok is set to false; otherwise *ok is set to true.

If base is 0, the C language convention is used: If the string begins with "0x", base 16 is used; if the string begins with "0", base 8 is used; otherwise, base 10 is used.

The string conversion will always happen in the 'C' locale. For locale dependent conversion use QLocale::toLongLong()

Example:


  QString str = "FF";
  bool ok;

  long hex = str.toLong(&ok, 16);     // hex == 255, ok == true
  long dec = str.toLong(&ok, 10);     // dec == 0, ok == false



See also number(), toULong(), toInt(), and QLocale::toInt().
*/
impl /*struct*/ QStringRef {
  pub fn toLong_0<RetType, T: QStringRef_toLong_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toLong_0(self);
    // return 1;
  }
}
pub trait QStringRef_toLong_0<RetType> {
  fn toLong_0(self , rsthis: & QStringRef) -> RetType;
}
impl<'a> /*trait*/ QStringRef_toLong_0<i64> for (usize,i32) {
  fn toLong_0(self , rsthis: & QStringRef) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QStringRef6toLongEPbi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1584
// index:0
// Public Visibility=Default Availability=Available
// [8] ulong toULong(bool *, int) const

/*
Returns the string converted to an unsigned long using base base, which is 10 by default and must be between 2 and 36, or 0. Returns 0 if the conversion fails.

If a conversion error occurs, *ok is set to false; otherwise *ok is set to true.

If base is 0, the C language convention is used: If the string begins with "0x", base 16 is used; if the string begins with "0", base 8 is used; otherwise, base 10 is used.

The string conversion will always happen in the 'C' locale. For locale dependent conversion use QLocale::toULongLong()

Example:


  QString str = "FF";
  bool ok;

  ulong hex = str.toULong(&ok, 16);   // hex == 255, ok == true
  ulong dec = str.toULong(&ok, 10);   // dec == 0, ok == false



See also number() and QLocale::toUInt().
*/
impl /*struct*/ QStringRef {
  pub fn toULong_0<RetType, T: QStringRef_toULong_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toULong_0(self);
    // return 1;
  }
}
pub trait QStringRef_toULong_0<RetType> {
  fn toULong_0(self , rsthis: & QStringRef) -> RetType;
}
impl<'a> /*trait*/ QStringRef_toULong_0<u64> for (usize,i32) {
  fn toULong_0(self , rsthis: & QStringRef) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QStringRef7toULongEPbi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1585
// index:0
// Public Visibility=Default Availability=Available
// [8] qlonglong toLongLong(bool *, int) const

/*
Returns the string converted to a long long using base base, which is 10 by default and must be between 2 and 36, or 0. Returns 0 if the conversion fails.

If a conversion error occurs, *ok is set to false; otherwise *ok is set to true.

If base is 0, the C language convention is used: If the string begins with "0x", base 16 is used; if the string begins with "0", base 8 is used; otherwise, base 10 is used.

The string conversion will always happen in the 'C' locale. For locale dependent conversion use QLocale::toLongLong()

Example:


  QString str = "FF";
  bool ok;

  qint64 hex = str.toLongLong(&ok, 16);      // hex == 255, ok == true
  qint64 dec = str.toLongLong(&ok, 10);      // dec == 0, ok == false



See also number(), toULongLong(), toInt(), and QLocale::toLongLong().
*/
impl /*struct*/ QStringRef {
  pub fn toLongLong_0<RetType, T: QStringRef_toLongLong_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toLongLong_0(self);
    // return 1;
  }
}
pub trait QStringRef_toLongLong_0<RetType> {
  fn toLongLong_0(self , rsthis: & QStringRef) -> RetType;
}
impl<'a> /*trait*/ QStringRef_toLongLong_0<i64> for (usize,i32) {
  fn toLongLong_0(self , rsthis: & QStringRef) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QStringRef10toLongLongEPbi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1586
// index:0
// Public Visibility=Default Availability=Available
// [8] qulonglong toULongLong(bool *, int) const

/*
Returns the string converted to an unsigned long long using base base, which is 10 by default and must be between 2 and 36, or 0. Returns 0 if the conversion fails.

If a conversion error occurs, *ok is set to false; otherwise *ok is set to true.

If base is 0, the C language convention is used: If the string begins with "0x", base 16 is used; if the string begins with "0", base 8 is used; otherwise, base 10 is used.

The string conversion will always happen in the 'C' locale. For locale dependent conversion use QLocale::toULongLong()

Example:


  QString str = "FF";
  bool ok;

  quint64 hex = str.toULongLong(&ok, 16);    // hex == 255, ok == true
  quint64 dec = str.toULongLong(&ok, 10);    // dec == 0, ok == false



See also number(), toLongLong(), and QLocale::toULongLong().
*/
impl /*struct*/ QStringRef {
  pub fn toULongLong_0<RetType, T: QStringRef_toULongLong_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toULongLong_0(self);
    // return 1;
  }
}
pub trait QStringRef_toULongLong_0<RetType> {
  fn toULongLong_0(self , rsthis: & QStringRef) -> RetType;
}
impl<'a> /*trait*/ QStringRef_toULongLong_0<u64> for (usize,i32) {
  fn toULongLong_0(self , rsthis: & QStringRef) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QStringRef11toULongLongEPbi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1587
// index:0
// Public Visibility=Default Availability=Available
// [4] float toFloat(bool *) const

/*
Returns the string converted to a float value.

If a conversion error occurs, *ok is set to false; otherwise *ok is set to true. Returns 0.0 if the conversion fails.

The string conversion will always happen in the 'C' locale. For locale dependent conversion use QLocale::toFloat()

Example:


  QString str1 = "1234.56";
  str1.toFloat();             // returns 1234.56

  bool ok;
  QString str2 = "R2D2";
  str2.toFloat(&ok);          // returns 0.0, sets ok to false



See also number(), toDouble(), toInt(), and QLocale::toFloat().
*/
impl /*struct*/ QStringRef {
  pub fn toFloat_0<RetType, T: QStringRef_toFloat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toFloat_0(self);
    // return 1;
  }
}
pub trait QStringRef_toFloat_0<RetType> {
  fn toFloat_0(self , rsthis: & QStringRef) -> RetType;
}
impl<'a> /*trait*/ QStringRef_toFloat_0<f32> for (usize) {
  fn toFloat_0(self , rsthis: & QStringRef) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QStringRef7toFloatEPb", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1588
// index:0
// Public Visibility=Default Availability=Available
// [8] double toDouble(bool *) const

/*
Returns the string converted to a double value.

Returns 0.0 if the conversion fails.

If a conversion error occurs, *ok is set to false; otherwise *ok is set to true.


  QString str = "1234.56";
  double val = str.toDouble();   // val == 1234.56



Warning: The QString content may only contain valid numerical characters which includes the plus/minus sign, the characters g and e used in scientific notation, and the decimal point. Including the unit or additional characters leads to a conversion error.


  bool ok;
  double d;

  d = QString( "1234.56e-02" ).toDouble(&ok); // ok == true, d == 12.3456



The string conversion will always happen in the 'C' locale. For locale dependent conversion use QLocale::toDouble()


  d = QString( "1234,56" ).toDouble(&ok); // ok == false
  d = QString( "1234.56" ).toDouble(&ok); // ok == true, d == 1234.56



For historical reasons, this function does not handle thousands group separators. If you need to convert such numbers, use QLocale::toDouble().


  d = QString( "1,234,567.89" ).toDouble(&ok); // ok == false
  d = QString( "1234567.89" ).toDouble(&ok); // ok == true



See also number(), QLocale::setDefault(), QLocale::toDouble(), and trimmed().
*/
impl /*struct*/ QStringRef {
  pub fn toDouble_0<RetType, T: QStringRef_toDouble_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toDouble_0(self);
    // return 1;
  }
}
pub trait QStringRef_toDouble_0<RetType> {
  fn toDouble_0(self , rsthis: & QStringRef) -> RetType;
}
impl<'a> /*trait*/ QStringRef_toDouble_0<f64> for (usize) {
  fn toDouble_0(self , rsthis: & QStringRef) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QStringRef8toDoubleEPb", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
