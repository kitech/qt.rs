

// mod ::core::QLatin1String
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
#[derive(Default)] // class sizeof(QLatin1String)=16
pub struct QLatin1String {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QLatin1String_ITF interface {
//    QLatin1String_PTR() *QLatin1String
//}
//func (ptr *QLatin1String) QLatin1String_PTR() *QLatin1String { return ptr }

impl /*struct*/ QLatin1String {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QLatin1String {
    return QLatin1String{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QLatin1String {
//  type Target = QLatin1StringBASE;
//
//  fn deref(&self) -> &QLatin1StringBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QLatin1StringBASE> for QLatin1String {
//  fn as_ref(& self) -> & QLatin1StringBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qstring.h:94
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void QLatin1String()

/*

*/
// QLatin1String() ctx.fn_proto_cpp
impl /*struct*/ QLatin1String {
  pub fn QLatin1String_0<T: QLatin1String_QLatin1String_0>(value: T) -> QLatin1String {
    let rsthis = value.QLatin1String_0();
    return rsthis;
    // return 1;
  }
}

pub trait QLatin1String_QLatin1String_0 {
  fn QLatin1String_0(self) -> QLatin1String;
}
// QLatin1String() ctx.fn_proto_cpp
impl<'a> /*trait*/ QLatin1String_QLatin1String_0 for () {
  fn QLatin1String_0(self) -> QLatin1String {
    // unsafe{_ZN13QLatin1StringC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN13QLatin1StringC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QLatin1String{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qstring.h:95
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void QLatin1String(const char *)

/*

*/
// QLatin1String(const char *) ctx.fn_proto_cpp
impl /*struct*/ QLatin1String {
  pub fn QLatin1String_1<T: QLatin1String_QLatin1String_1>(value: T) -> QLatin1String {
    let rsthis = value.QLatin1String_1();
    return rsthis;
    // return 1;
  }
}

pub trait QLatin1String_QLatin1String_1 {
  fn QLatin1String_1(self) -> QLatin1String;
}
// QLatin1String(const char *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QLatin1String_QLatin1String_1 for (usize) {
  fn QLatin1String_1(self) -> QLatin1String {
    // unsafe{_ZN13QLatin1StringC2EPKc()};
    let arg0 = (self) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN13QLatin1StringC2EPKc", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QLatin1String{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qstring.h:96
// index:2
// Public inline Visibility=Default Availability=Available
// [-2] void QLatin1String(const char *, const char *)

/*

*/
// QLatin1String(const char *, const char *) ctx.fn_proto_cpp
impl /*struct*/ QLatin1String {
  pub fn QLatin1String_2<T: QLatin1String_QLatin1String_2>(value: T) -> QLatin1String {
    let rsthis = value.QLatin1String_2();
    return rsthis;
    // return 1;
  }
}

pub trait QLatin1String_QLatin1String_2 {
  fn QLatin1String_2(self) -> QLatin1String;
}
// QLatin1String(const char *, const char *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QLatin1String_QLatin1String_2 for (usize,usize) {
  fn QLatin1String_2(self) -> QLatin1String {
    // unsafe{_ZN13QLatin1StringC2EPKcS1_()};
    let arg0 = (self.0) as *const usize as usize;
    let arg1 = (self.1) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN13QLatin1StringC2EPKcS1_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QLatin1String{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qstring.h:98
// index:3
// Public inline Visibility=Default Availability=Available
// [-2] void QLatin1String(const char *, int)

/*

*/
// QLatin1String(const char *, int) ctx.fn_proto_cpp
impl /*struct*/ QLatin1String {
  pub fn QLatin1String_3<T: QLatin1String_QLatin1String_3>(value: T) -> QLatin1String {
    let rsthis = value.QLatin1String_3();
    return rsthis;
    // return 1;
  }
}

pub trait QLatin1String_QLatin1String_3 {
  fn QLatin1String_3(self) -> QLatin1String;
}
// QLatin1String(const char *, int) ctx.fn_proto_cpp
impl<'a> /*trait*/ QLatin1String_QLatin1String_3 for (usize,i32) {
  fn QLatin1String_3(self) -> QLatin1String {
    // unsafe{_ZN13QLatin1StringC2EPKci()};
    let arg0 = (self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN13QLatin1StringC2EPKci", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QLatin1String{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qstring.h:99
// index:4
// Public inline Visibility=Default Availability=Available
// [-2] void QLatin1String(const QByteArray &)

/*

*/
// QLatin1String(const QByteArray &) ctx.fn_proto_cpp
impl /*struct*/ QLatin1String {
  pub fn QLatin1String_4<T: QLatin1String_QLatin1String_4>(value: T) -> QLatin1String {
    let rsthis = value.QLatin1String_4();
    return rsthis;
    // return 1;
  }
}

pub trait QLatin1String_QLatin1String_4 {
  fn QLatin1String_4(self) -> QLatin1String;
}
// QLatin1String(const QByteArray &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QLatin1String_QLatin1String_4 for (usize) {
  fn QLatin1String_4(self) -> QLatin1String {
    // unsafe{_ZN13QLatin1StringC2ERK10QByteArray()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN13QLatin1StringC2ERK10QByteArray", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QLatin1String{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qstring.h:101
// index:0
// Public inline Visibility=Default Availability=Available
// [8] const char * latin1() const

/*

*/
impl /*struct*/ QLatin1String {
  pub fn latin1_0<RetType, T: QLatin1String_latin1_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.latin1_0(self);
    // return 1;
  }
}
pub trait QLatin1String_latin1_0<RetType> {
  fn latin1_0(self , rsthis: & QLatin1String) -> RetType;
}
impl<'a> /*trait*/ QLatin1String_latin1_0<usize> for () {
  fn latin1_0(self , rsthis: & QLatin1String) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QLatin1String6latin1Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:102
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
impl /*struct*/ QLatin1String {
  pub fn size_0<RetType, T: QLatin1String_size_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.size_0(self);
    // return 1;
  }
}
pub trait QLatin1String_size_0<RetType> {
  fn size_0(self , rsthis: & QLatin1String) -> RetType;
}
impl<'a> /*trait*/ QLatin1String_size_0<i32> for () {
  fn size_0(self , rsthis: & QLatin1String) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QLatin1String4sizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:103
// index:0
// Public inline Visibility=Default Availability=Available
// [8] const char * data() const

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
impl /*struct*/ QLatin1String {
  pub fn data_0<RetType, T: QLatin1String_data_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.data_0(self);
    // return 1;
  }
}
pub trait QLatin1String_data_0<RetType> {
  fn data_0(self , rsthis: & QLatin1String) -> RetType;
}
impl<'a> /*trait*/ QLatin1String_data_0<usize> for () {
  fn data_0(self , rsthis: & QLatin1String) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QLatin1String4dataEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:105
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
impl /*struct*/ QLatin1String {
  pub fn isNull_0<RetType, T: QLatin1String_isNull_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isNull_0(self);
    // return 1;
  }
}
pub trait QLatin1String_isNull_0<RetType> {
  fn isNull_0(self , rsthis: & QLatin1String) -> RetType;
}
impl<'a> /*trait*/ QLatin1String_isNull_0<bool> for () {
  fn isNull_0(self , rsthis: & QLatin1String) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QLatin1String6isNullEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:106
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
impl /*struct*/ QLatin1String {
  pub fn isEmpty_0<RetType, T: QLatin1String_isEmpty_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isEmpty_0(self);
    // return 1;
  }
}
pub trait QLatin1String_isEmpty_0<RetType> {
  fn isEmpty_0(self , rsthis: & QLatin1String) -> RetType;
}
impl<'a> /*trait*/ QLatin1String_isEmpty_0<bool> for () {
  fn isEmpty_0(self , rsthis: & QLatin1String) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QLatin1String7isEmptyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:108
// index:0
// Public inline Visibility=Default Availability=Available
// [1] QLatin1Char at(int) const

/*
Returns the character at the given index position in the string.

The position must be a valid index position in the string (i.e., 0 <= position < size()).

See also operator[]().
*/
impl /*struct*/ QLatin1String {
  pub fn at_0<RetType, T: QLatin1String_at_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.at_0(self);
    // return 1;
  }
}
pub trait QLatin1String_at_0<RetType> {
  fn at_0(self , rsthis: & QLatin1String) -> RetType;
}
impl<'a> /*trait*/ QLatin1String_at_0<usize> for (i32) {
  fn at_0(self , rsthis: & QLatin1String) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QLatin1String2atEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:110
// index:0
// Public inline Visibility=Default Availability=Available
// [1] QLatin1Char operator[](int) const

/*

*/
impl /*struct*/ QLatin1String {
  pub fn operator_get_index_0<RetType, T: QLatin1String_operator_get_index_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_get_index_0(self);
    // return 1;
  }
}
pub trait QLatin1String_operator_get_index_0<RetType> {
  fn operator_get_index_0(self , rsthis: & QLatin1String) -> RetType;
}
impl<'a> /*trait*/ QLatin1String_operator_get_index_0<usize> for (i32) {
  fn operator_get_index_0(self , rsthis: & QLatin1String) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QLatin1StringixEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:112
// index:0
// Public inline Visibility=Default Availability=Available
// [1] QLatin1Char front() const

/*
Returns the first character in the string. Same as at(0).

This function is provided for STL compatibility.

Warning: Calling this function on an empty string constitutes undefined behavior.

This function was introduced in  Qt 5.10.

See also back(), at(), and operator[]().
*/
impl /*struct*/ QLatin1String {
  pub fn front_0<RetType, T: QLatin1String_front_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.front_0(self);
    // return 1;
  }
}
pub trait QLatin1String_front_0<RetType> {
  fn front_0(self , rsthis: & QLatin1String) -> RetType;
}
impl<'a> /*trait*/ QLatin1String_front_0<usize> for () {
  fn front_0(self , rsthis: & QLatin1String) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QLatin1String5frontEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:113
// index:0
// Public inline Visibility=Default Availability=Available
// [1] QLatin1Char back() const

/*
Returns the last character in the string. Same as at(size() - 1).

This function is provided for STL compatibility.

Warning: Calling this function on an empty string constitutes undefined behavior.

This function was introduced in  Qt 5.10.

See also front(), at(), and operator[]().
*/
impl /*struct*/ QLatin1String {
  pub fn back_0<RetType, T: QLatin1String_back_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.back_0(self);
    // return 1;
  }
}
pub trait QLatin1String_back_0<RetType> {
  fn back_0(self , rsthis: & QLatin1String) -> RetType;
}
impl<'a> /*trait*/ QLatin1String_back_0<usize> for () {
  fn back_0(self , rsthis: & QLatin1String) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QLatin1String4backEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:115
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
impl /*struct*/ QLatin1String {
  pub fn startsWith_0<RetType, T: QLatin1String_startsWith_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.startsWith_0(self);
    // return 1;
  }
}
pub trait QLatin1String_startsWith_0<RetType> {
  fn startsWith_0(self , rsthis: & QLatin1String) -> RetType;
}
impl<'a> /*trait*/ QLatin1String_startsWith_0<bool> for (usize,i32) {
  fn startsWith_0(self , rsthis: & QLatin1String) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QLatin1String10startsWithE11QStringViewN2Qt15CaseSensitivityE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:117
// index:1
// Public inline Visibility=Default Availability=Available
// [1] bool startsWith(QLatin1String, Qt::CaseSensitivity) const

/*
Returns true if the string starts with s; otherwise returns false.

If cs is Qt::CaseSensitive (default), the search is case sensitive; otherwise the search is case insensitive.


  QString str = "Bananas";
  str.startsWith("Ban");     // returns true
  str.startsWith("Car");     // returns false



See also endsWith().
*/
impl /*struct*/ QLatin1String {
  pub fn startsWith_1<RetType, T: QLatin1String_startsWith_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.startsWith_1(self);
    // return 1;
  }
}
pub trait QLatin1String_startsWith_1<RetType> {
  fn startsWith_1(self , rsthis: & QLatin1String) -> RetType;
}
impl<'a> /*trait*/ QLatin1String_startsWith_1<bool> for (usize,i32) {
  fn startsWith_1(self , rsthis: & QLatin1String) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QLatin1String10startsWithES_N2Qt15CaseSensitivityE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:119
// index:2
// Public inline Visibility=Default Availability=Available
// [1] bool startsWith(QChar) const

/*
Returns true if the string starts with s; otherwise returns false.

If cs is Qt::CaseSensitive (default), the search is case sensitive; otherwise the search is case insensitive.


  QString str = "Bananas";
  str.startsWith("Ban");     // returns true
  str.startsWith("Car");     // returns false



See also endsWith().
*/
impl /*struct*/ QLatin1String {
  pub fn startsWith_2<RetType, T: QLatin1String_startsWith_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.startsWith_2(self);
    // return 1;
  }
}
pub trait QLatin1String_startsWith_2<RetType> {
  fn startsWith_2(self , rsthis: & QLatin1String) -> RetType;
}
impl<'a> /*trait*/ QLatin1String_startsWith_2<bool> for (usize) {
  fn startsWith_2(self , rsthis: & QLatin1String) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QLatin1String10startsWithE5QChar", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:121
// index:3
// Public inline Visibility=Default Availability=Available
// [1] bool startsWith(QChar, Qt::CaseSensitivity) const

/*
Returns true if the string starts with s; otherwise returns false.

If cs is Qt::CaseSensitive (default), the search is case sensitive; otherwise the search is case insensitive.


  QString str = "Bananas";
  str.startsWith("Ban");     // returns true
  str.startsWith("Car");     // returns false



See also endsWith().
*/
impl /*struct*/ QLatin1String {
  pub fn startsWith_3<RetType, T: QLatin1String_startsWith_3<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.startsWith_3(self);
    // return 1;
  }
}
pub trait QLatin1String_startsWith_3<RetType> {
  fn startsWith_3(self , rsthis: & QLatin1String) -> RetType;
}
impl<'a> /*trait*/ QLatin1String_startsWith_3<bool> for (usize,i32) {
  fn startsWith_3(self , rsthis: & QLatin1String) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QLatin1String10startsWithE5QCharN2Qt15CaseSensitivityE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:124
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
impl /*struct*/ QLatin1String {
  pub fn endsWith_0<RetType, T: QLatin1String_endsWith_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.endsWith_0(self);
    // return 1;
  }
}
pub trait QLatin1String_endsWith_0<RetType> {
  fn endsWith_0(self , rsthis: & QLatin1String) -> RetType;
}
impl<'a> /*trait*/ QLatin1String_endsWith_0<bool> for (usize,i32) {
  fn endsWith_0(self , rsthis: & QLatin1String) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QLatin1String8endsWithE11QStringViewN2Qt15CaseSensitivityE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:126
// index:1
// Public inline Visibility=Default Availability=Available
// [1] bool endsWith(QLatin1String, Qt::CaseSensitivity) const

/*
Returns true if the string ends with s; otherwise returns false.

If cs is Qt::CaseSensitive (default), the search is case sensitive; otherwise the search is case insensitive.


  QString str = "Bananas";
  str.endsWith("anas");         // returns true
  str.endsWith("pple");         // returns false



See also startsWith().
*/
impl /*struct*/ QLatin1String {
  pub fn endsWith_1<RetType, T: QLatin1String_endsWith_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.endsWith_1(self);
    // return 1;
  }
}
pub trait QLatin1String_endsWith_1<RetType> {
  fn endsWith_1(self , rsthis: & QLatin1String) -> RetType;
}
impl<'a> /*trait*/ QLatin1String_endsWith_1<bool> for (usize,i32) {
  fn endsWith_1(self , rsthis: & QLatin1String) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QLatin1String8endsWithES_N2Qt15CaseSensitivityE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:128
// index:2
// Public inline Visibility=Default Availability=Available
// [1] bool endsWith(QChar) const

/*
Returns true if the string ends with s; otherwise returns false.

If cs is Qt::CaseSensitive (default), the search is case sensitive; otherwise the search is case insensitive.


  QString str = "Bananas";
  str.endsWith("anas");         // returns true
  str.endsWith("pple");         // returns false



See also startsWith().
*/
impl /*struct*/ QLatin1String {
  pub fn endsWith_2<RetType, T: QLatin1String_endsWith_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.endsWith_2(self);
    // return 1;
  }
}
pub trait QLatin1String_endsWith_2<RetType> {
  fn endsWith_2(self , rsthis: & QLatin1String) -> RetType;
}
impl<'a> /*trait*/ QLatin1String_endsWith_2<bool> for (usize) {
  fn endsWith_2(self , rsthis: & QLatin1String) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QLatin1String8endsWithE5QChar", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:130
// index:3
// Public inline Visibility=Default Availability=Available
// [1] bool endsWith(QChar, Qt::CaseSensitivity) const

/*
Returns true if the string ends with s; otherwise returns false.

If cs is Qt::CaseSensitive (default), the search is case sensitive; otherwise the search is case insensitive.


  QString str = "Bananas";
  str.endsWith("anas");         // returns true
  str.endsWith("pple");         // returns false



See also startsWith().
*/
impl /*struct*/ QLatin1String {
  pub fn endsWith_3<RetType, T: QLatin1String_endsWith_3<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.endsWith_3(self);
    // return 1;
  }
}
pub trait QLatin1String_endsWith_3<RetType> {
  fn endsWith_3(self , rsthis: & QLatin1String) -> RetType;
}
impl<'a> /*trait*/ QLatin1String_endsWith_3<bool> for (usize,i32) {
  fn endsWith_3(self , rsthis: & QLatin1String) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QLatin1String8endsWithE5QCharN2Qt15CaseSensitivityE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:141
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QLatin1String::const_iterator begin() const

/*
Returns an STL-style iterator pointing to the first character in the string.

See also constBegin() and end().
*/
impl /*struct*/ QLatin1String {
  pub fn begin_0<RetType, T: QLatin1String_begin_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.begin_0(self);
    // return 1;
  }
}
pub trait QLatin1String_begin_0<RetType> {
  fn begin_0(self , rsthis: & QLatin1String) -> RetType;
}
impl<'a> /*trait*/ QLatin1String_begin_0<usize> for () {
  fn begin_0(self , rsthis: & QLatin1String) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QLatin1String5beginEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:142
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QLatin1String::const_iterator cbegin() const

/*
Returns a const STL-style iterator pointing to the first character in the string.

This function was introduced in  Qt 5.0.

See also begin() and cend().
*/
impl /*struct*/ QLatin1String {
  pub fn cbegin_0<RetType, T: QLatin1String_cbegin_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cbegin_0(self);
    // return 1;
  }
}
pub trait QLatin1String_cbegin_0<RetType> {
  fn cbegin_0(self , rsthis: & QLatin1String) -> RetType;
}
impl<'a> /*trait*/ QLatin1String_cbegin_0<usize> for () {
  fn cbegin_0(self , rsthis: & QLatin1String) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QLatin1String6cbeginEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:143
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QLatin1String::const_iterator end() const

/*
Returns an STL-style iterator pointing to the imaginary character after the last character in the string.

See also begin() and constEnd().
*/
impl /*struct*/ QLatin1String {
  pub fn end_0<RetType, T: QLatin1String_end_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.end_0(self);
    // return 1;
  }
}
pub trait QLatin1String_end_0<RetType> {
  fn end_0(self , rsthis: & QLatin1String) -> RetType;
}
impl<'a> /*trait*/ QLatin1String_end_0<usize> for () {
  fn end_0(self , rsthis: & QLatin1String) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QLatin1String3endEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:144
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QLatin1String::const_iterator cend() const

/*
Returns a const STL-style iterator pointing to the imaginary character after the last character in the list.

This function was introduced in  Qt 5.0.

See also cbegin() and end().
*/
impl /*struct*/ QLatin1String {
  pub fn cend_0<RetType, T: QLatin1String_cend_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cend_0(self);
    // return 1;
  }
}
pub trait QLatin1String_cend_0<RetType> {
  fn cend_0(self , rsthis: & QLatin1String) -> RetType;
}
impl<'a> /*trait*/ QLatin1String_cend_0<usize> for () {
  fn cend_0(self , rsthis: & QLatin1String) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QLatin1String4cendEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:154
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QLatin1String mid(int) const

/*
Returns a string that contains n characters of this string, starting at the specified position index.

Returns a null string if the position index exceeds the length of the string. If there are less than n characters available in the string starting at the given position, or if n is -1 (default), the function returns all characters that are available from the specified position.

Example:


  QString x = "Nine pineapples";
  QString y = x.mid(5, 4);            // y == "pine"
  QString z = x.mid(5);               // z == "pineapples"



See also left(), right(), chopped(), chop(), and truncate().
*/
impl /*struct*/ QLatin1String {
  pub fn mid_0<RetType, T: QLatin1String_mid_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mid_0(self);
    // return 1;
  }
}
pub trait QLatin1String_mid_0<RetType> {
  fn mid_0(self , rsthis: & QLatin1String) -> RetType;
}
impl<'a> /*trait*/ QLatin1String_mid_0<usize> for (i32) {
  fn mid_0(self , rsthis: & QLatin1String) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QLatin1String3midEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:156
// index:1
// Public inline Visibility=Default Availability=Available
// [16] QLatin1String mid(int, int) const

/*
Returns a string that contains n characters of this string, starting at the specified position index.

Returns a null string if the position index exceeds the length of the string. If there are less than n characters available in the string starting at the given position, or if n is -1 (default), the function returns all characters that are available from the specified position.

Example:


  QString x = "Nine pineapples";
  QString y = x.mid(5, 4);            // y == "pine"
  QString z = x.mid(5);               // z == "pineapples"



See also left(), right(), chopped(), chop(), and truncate().
*/
impl /*struct*/ QLatin1String {
  pub fn mid_1<RetType, T: QLatin1String_mid_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mid_1(self);
    // return 1;
  }
}
pub trait QLatin1String_mid_1<RetType> {
  fn mid_1(self , rsthis: & QLatin1String) -> RetType;
}
impl<'a> /*trait*/ QLatin1String_mid_1<usize> for (i32,i32) {
  fn mid_1(self , rsthis: & QLatin1String) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QLatin1String3midEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:158
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QLatin1String left(int) const

/*
Returns a substring that contains the n leftmost characters of the string.

The entire string is returned if n is greater than or equal to size(), or less than zero.


  QString x = "Pineapple";
  QString y = x.left(4);      // y == "Pine"



See also right(), mid(), startsWith(), chopped(), chop(), and truncate().
*/
impl /*struct*/ QLatin1String {
  pub fn left_0<RetType, T: QLatin1String_left_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.left_0(self);
    // return 1;
  }
}
pub trait QLatin1String_left_0<RetType> {
  fn left_0(self , rsthis: & QLatin1String) -> RetType;
}
impl<'a> /*trait*/ QLatin1String_left_0<usize> for (i32) {
  fn left_0(self , rsthis: & QLatin1String) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QLatin1String4leftEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:160
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QLatin1String right(int) const

/*
Returns a substring that contains the n rightmost characters of the string.

The entire string is returned if n is greater than or equal to size(), or less than zero.


  QString x = "Pineapple";
  QString y = x.right(5);      // y == "apple"



See also left(), mid(), endsWith(), chopped(), chop(), and truncate().
*/
impl /*struct*/ QLatin1String {
  pub fn right_0<RetType, T: QLatin1String_right_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.right_0(self);
    // return 1;
  }
}
pub trait QLatin1String_right_0<RetType> {
  fn right_0(self , rsthis: & QLatin1String) -> RetType;
}
impl<'a> /*trait*/ QLatin1String_right_0<usize> for (i32) {
  fn right_0(self , rsthis: & QLatin1String) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QLatin1String5rightEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:162
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QLatin1String chopped(int) const

/*
Returns a substring that contains the size() - len leftmost characters of this string.

Note: The behavior is undefined if len is negative or greater than size().

This function was introduced in  Qt 5.10.

See also endsWith(), left(), right(), mid(), chop(), and truncate().
*/
impl /*struct*/ QLatin1String {
  pub fn chopped_0<RetType, T: QLatin1String_chopped_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.chopped_0(self);
    // return 1;
  }
}
pub trait QLatin1String_chopped_0<RetType> {
  fn chopped_0(self , rsthis: & QLatin1String) -> RetType;
}
impl<'a> /*trait*/ QLatin1String_chopped_0<usize> for (i32) {
  fn chopped_0(self , rsthis: & QLatin1String) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QLatin1String7choppedEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:165
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
impl /*struct*/ QLatin1String {
  pub fn chop_0<RetType, T: QLatin1String_chop_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.chop_0(self);
    // return 1;
  }
}
pub trait QLatin1String_chop_0<RetType> {
  fn chop_0(self , rsthis: & QLatin1String) -> RetType;
}
impl<'a> /*trait*/ QLatin1String_chop_0<(/*void*/)> for (i32) {
  fn chop_0(self , rsthis: & QLatin1String) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN13QLatin1String4chopEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qstring.h:167
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
impl /*struct*/ QLatin1String {
  pub fn truncate_0<RetType, T: QLatin1String_truncate_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.truncate_0(self);
    // return 1;
  }
}
pub trait QLatin1String_truncate_0<RetType> {
  fn truncate_0(self , rsthis: & QLatin1String) -> RetType;
}
impl<'a> /*trait*/ QLatin1String_truncate_0<(/*void*/)> for (i32) {
  fn truncate_0(self , rsthis: & QLatin1String) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN13QLatin1String8truncateEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qstring.h:170
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QLatin1String trimmed() const

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
impl /*struct*/ QLatin1String {
  pub fn trimmed_0<RetType, T: QLatin1String_trimmed_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.trimmed_0(self);
    // return 1;
  }
}
pub trait QLatin1String_trimmed_0<RetType> {
  fn trimmed_0(self , rsthis: & QLatin1String) -> RetType;
}
impl<'a> /*trait*/ QLatin1String_trimmed_0<usize> for () {
  fn trimmed_0(self , rsthis: & QLatin1String) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QLatin1String7trimmedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:172
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator==(const QString &) const

/*

*/
impl /*struct*/ QLatin1String {
  pub fn operator_equal_equal_0<RetType, T: QLatin1String_operator_equal_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_equal_0(self);
    // return 1;
  }
}
pub trait QLatin1String_operator_equal_equal_0<RetType> {
  fn operator_equal_equal_0(self , rsthis: & QLatin1String) -> RetType;
}
impl<'a> /*trait*/ QLatin1String_operator_equal_equal_0<bool> for (usize) {
  fn operator_equal_equal_0(self , rsthis: & QLatin1String) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QLatin1StringeqERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:180
// index:1
// Public inline Visibility=Default Availability=Available
// [1] bool operator==(const char *) const

/*

*/
impl /*struct*/ QLatin1String {
  pub fn operator_equal_equal_1<RetType, T: QLatin1String_operator_equal_equal_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_equal_1(self);
    // return 1;
  }
}
pub trait QLatin1String_operator_equal_equal_1<RetType> {
  fn operator_equal_equal_1(self , rsthis: & QLatin1String) -> RetType;
}
impl<'a> /*trait*/ QLatin1String_operator_equal_equal_1<bool> for (usize) {
  fn operator_equal_equal_1(self , rsthis: & QLatin1String) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QLatin1StringeqEPKc", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:187
// index:2
// Public inline Visibility=Default Availability=Available
// [1] bool operator==(const QByteArray &) const

/*

*/
impl /*struct*/ QLatin1String {
  pub fn operator_equal_equal_2<RetType, T: QLatin1String_operator_equal_equal_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_equal_2(self);
    // return 1;
  }
}
pub trait QLatin1String_operator_equal_equal_2<RetType> {
  fn operator_equal_equal_2(self , rsthis: & QLatin1String) -> RetType;
}
impl<'a> /*trait*/ QLatin1String_operator_equal_equal_2<bool> for (usize) {
  fn operator_equal_equal_2(self , rsthis: & QLatin1String) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QLatin1StringeqERK10QByteArray", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:173
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator!=(const QString &) const

/*

*/
impl /*struct*/ QLatin1String {
  pub fn operator_not_equal_0<RetType, T: QLatin1String_operator_not_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_not_equal_0(self);
    // return 1;
  }
}
pub trait QLatin1String_operator_not_equal_0<RetType> {
  fn operator_not_equal_0(self , rsthis: & QLatin1String) -> RetType;
}
impl<'a> /*trait*/ QLatin1String_operator_not_equal_0<bool> for (usize) {
  fn operator_not_equal_0(self , rsthis: & QLatin1String) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QLatin1StringneERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:181
// index:1
// Public inline Visibility=Default Availability=Available
// [1] bool operator!=(const char *) const

/*

*/
impl /*struct*/ QLatin1String {
  pub fn operator_not_equal_1<RetType, T: QLatin1String_operator_not_equal_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_not_equal_1(self);
    // return 1;
  }
}
pub trait QLatin1String_operator_not_equal_1<RetType> {
  fn operator_not_equal_1(self , rsthis: & QLatin1String) -> RetType;
}
impl<'a> /*trait*/ QLatin1String_operator_not_equal_1<bool> for (usize) {
  fn operator_not_equal_1(self , rsthis: & QLatin1String) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QLatin1StringneEPKc", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:188
// index:2
// Public inline Visibility=Default Availability=Available
// [1] bool operator!=(const QByteArray &) const

/*

*/
impl /*struct*/ QLatin1String {
  pub fn operator_not_equal_2<RetType, T: QLatin1String_operator_not_equal_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_not_equal_2(self);
    // return 1;
  }
}
pub trait QLatin1String_operator_not_equal_2<RetType> {
  fn operator_not_equal_2(self , rsthis: & QLatin1String) -> RetType;
}
impl<'a> /*trait*/ QLatin1String_operator_not_equal_2<bool> for (usize) {
  fn operator_not_equal_2(self , rsthis: & QLatin1String) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QLatin1StringneERK10QByteArray", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:174
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator>(const QString &) const

/*

*/
impl /*struct*/ QLatin1String {
  pub fn operator_greater_than_0<RetType, T: QLatin1String_operator_greater_than_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_greater_than_0(self);
    // return 1;
  }
}
pub trait QLatin1String_operator_greater_than_0<RetType> {
  fn operator_greater_than_0(self , rsthis: & QLatin1String) -> RetType;
}
impl<'a> /*trait*/ QLatin1String_operator_greater_than_0<bool> for (usize) {
  fn operator_greater_than_0(self , rsthis: & QLatin1String) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QLatin1StringgtERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:183
// index:1
// Public inline Visibility=Default Availability=Available
// [1] bool operator>(const char *) const

/*

*/
impl /*struct*/ QLatin1String {
  pub fn operator_greater_than_1<RetType, T: QLatin1String_operator_greater_than_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_greater_than_1(self);
    // return 1;
  }
}
pub trait QLatin1String_operator_greater_than_1<RetType> {
  fn operator_greater_than_1(self , rsthis: & QLatin1String) -> RetType;
}
impl<'a> /*trait*/ QLatin1String_operator_greater_than_1<bool> for (usize) {
  fn operator_greater_than_1(self , rsthis: & QLatin1String) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QLatin1StringgtEPKc", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:190
// index:2
// Public inline Visibility=Default Availability=Available
// [1] bool operator>(const QByteArray &) const

/*

*/
impl /*struct*/ QLatin1String {
  pub fn operator_greater_than_2<RetType, T: QLatin1String_operator_greater_than_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_greater_than_2(self);
    // return 1;
  }
}
pub trait QLatin1String_operator_greater_than_2<RetType> {
  fn operator_greater_than_2(self , rsthis: & QLatin1String) -> RetType;
}
impl<'a> /*trait*/ QLatin1String_operator_greater_than_2<bool> for (usize) {
  fn operator_greater_than_2(self , rsthis: & QLatin1String) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QLatin1StringgtERK10QByteArray", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:175
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator<(const QString &) const

/*

*/
impl /*struct*/ QLatin1String {
  pub fn operator_less_than_0<RetType, T: QLatin1String_operator_less_than_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_less_than_0(self);
    // return 1;
  }
}
pub trait QLatin1String_operator_less_than_0<RetType> {
  fn operator_less_than_0(self , rsthis: & QLatin1String) -> RetType;
}
impl<'a> /*trait*/ QLatin1String_operator_less_than_0<bool> for (usize) {
  fn operator_less_than_0(self , rsthis: & QLatin1String) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QLatin1StringltERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:182
// index:1
// Public inline Visibility=Default Availability=Available
// [1] bool operator<(const char *) const

/*

*/
impl /*struct*/ QLatin1String {
  pub fn operator_less_than_1<RetType, T: QLatin1String_operator_less_than_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_less_than_1(self);
    // return 1;
  }
}
pub trait QLatin1String_operator_less_than_1<RetType> {
  fn operator_less_than_1(self , rsthis: & QLatin1String) -> RetType;
}
impl<'a> /*trait*/ QLatin1String_operator_less_than_1<bool> for (usize) {
  fn operator_less_than_1(self , rsthis: & QLatin1String) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QLatin1StringltEPKc", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:189
// index:2
// Public inline Visibility=Default Availability=Available
// [1] bool operator<(const QByteArray &) const

/*

*/
impl /*struct*/ QLatin1String {
  pub fn operator_less_than_2<RetType, T: QLatin1String_operator_less_than_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_less_than_2(self);
    // return 1;
  }
}
pub trait QLatin1String_operator_less_than_2<RetType> {
  fn operator_less_than_2(self , rsthis: & QLatin1String) -> RetType;
}
impl<'a> /*trait*/ QLatin1String_operator_less_than_2<bool> for (usize) {
  fn operator_less_than_2(self , rsthis: & QLatin1String) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QLatin1StringltERK10QByteArray", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:176
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator>=(const QString &) const

/*

*/
impl /*struct*/ QLatin1String {
  pub fn operator_greater_than_equal_0<RetType, T: QLatin1String_operator_greater_than_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_greater_than_equal_0(self);
    // return 1;
  }
}
pub trait QLatin1String_operator_greater_than_equal_0<RetType> {
  fn operator_greater_than_equal_0(self , rsthis: & QLatin1String) -> RetType;
}
impl<'a> /*trait*/ QLatin1String_operator_greater_than_equal_0<bool> for (usize) {
  fn operator_greater_than_equal_0(self , rsthis: & QLatin1String) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QLatin1StringgeERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:185
// index:1
// Public inline Visibility=Default Availability=Available
// [1] bool operator>=(const char *) const

/*

*/
impl /*struct*/ QLatin1String {
  pub fn operator_greater_than_equal_1<RetType, T: QLatin1String_operator_greater_than_equal_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_greater_than_equal_1(self);
    // return 1;
  }
}
pub trait QLatin1String_operator_greater_than_equal_1<RetType> {
  fn operator_greater_than_equal_1(self , rsthis: & QLatin1String) -> RetType;
}
impl<'a> /*trait*/ QLatin1String_operator_greater_than_equal_1<bool> for (usize) {
  fn operator_greater_than_equal_1(self , rsthis: & QLatin1String) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QLatin1StringgeEPKc", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:192
// index:2
// Public inline Visibility=Default Availability=Available
// [1] bool operator>=(const QByteArray &) const

/*

*/
impl /*struct*/ QLatin1String {
  pub fn operator_greater_than_equal_2<RetType, T: QLatin1String_operator_greater_than_equal_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_greater_than_equal_2(self);
    // return 1;
  }
}
pub trait QLatin1String_operator_greater_than_equal_2<RetType> {
  fn operator_greater_than_equal_2(self , rsthis: & QLatin1String) -> RetType;
}
impl<'a> /*trait*/ QLatin1String_operator_greater_than_equal_2<bool> for (usize) {
  fn operator_greater_than_equal_2(self , rsthis: & QLatin1String) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QLatin1StringgeERK10QByteArray", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:177
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator<=(const QString &) const

/*

*/
impl /*struct*/ QLatin1String {
  pub fn operator_less_than_equal_0<RetType, T: QLatin1String_operator_less_than_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_less_than_equal_0(self);
    // return 1;
  }
}
pub trait QLatin1String_operator_less_than_equal_0<RetType> {
  fn operator_less_than_equal_0(self , rsthis: & QLatin1String) -> RetType;
}
impl<'a> /*trait*/ QLatin1String_operator_less_than_equal_0<bool> for (usize) {
  fn operator_less_than_equal_0(self , rsthis: & QLatin1String) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QLatin1StringleERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:184
// index:1
// Public inline Visibility=Default Availability=Available
// [1] bool operator<=(const char *) const

/*

*/
impl /*struct*/ QLatin1String {
  pub fn operator_less_than_equal_1<RetType, T: QLatin1String_operator_less_than_equal_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_less_than_equal_1(self);
    // return 1;
  }
}
pub trait QLatin1String_operator_less_than_equal_1<RetType> {
  fn operator_less_than_equal_1(self , rsthis: & QLatin1String) -> RetType;
}
impl<'a> /*trait*/ QLatin1String_operator_less_than_equal_1<bool> for (usize) {
  fn operator_less_than_equal_1(self , rsthis: & QLatin1String) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QLatin1StringleEPKc", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:191
// index:2
// Public inline Visibility=Default Availability=Available
// [1] bool operator<=(const QByteArray &) const

/*

*/
impl /*struct*/ QLatin1String {
  pub fn operator_less_than_equal_2<RetType, T: QLatin1String_operator_less_than_equal_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_less_than_equal_2(self);
    // return 1;
  }
}
pub trait QLatin1String_operator_less_than_equal_2<RetType> {
  fn operator_less_than_equal_2(self , rsthis: & QLatin1String) -> RetType;
}
impl<'a> /*trait*/ QLatin1String_operator_less_than_equal_2<bool> for (usize) {
  fn operator_less_than_equal_2(self , rsthis: & QLatin1String) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QLatin1StringleERK10QByteArray", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}


pub fn DeleteQLatin1String(this :*mut QLatin1String) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN13QLatin1StringD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
