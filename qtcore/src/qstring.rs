

// mod ::core::QString
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
// extern C begin: 52
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
#[derive(Default)] // class sizeof(QString)=8
pub struct QString {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QString_ITF interface {
//    QString_PTR() *QString
//}
//func (ptr *QString) QString_PTR() *QString { return ptr }

impl /*struct*/ QString {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QString {
    return QString{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QString {
//  type Target = QStringBASE;
//
//  fn deref(&self) -> &QStringBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QStringBASE> for QString {
//  fn as_ref(& self) -> & QStringBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qstring.h:217
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void QString()

/*
Constructs a null string. Null strings are also empty.

See also isEmpty().
*/
// QString() ctx.fn_proto_cpp
impl /*struct*/ QString {
  pub fn QString_0<T: QString_QString_0>(value: T) -> QString {
    let rsthis = value.QString_0();
    return rsthis;
    // return 1;
  }
}

pub trait QString_QString_0 {
  fn QString_0(self) -> QString;
}
// QString() ctx.fn_proto_cpp
impl<'a> /*trait*/ QString_QString_0 for () {
  fn QString_0(self) -> QString {
    // unsafe{_ZN7QStringC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN7QStringC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QString{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qstring.h:218
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QString(const QChar *, int)

/*
Constructs a null string. Null strings are also empty.

See also isEmpty().
*/
// QString(const QChar *, int) ctx.fn_proto_cpp
impl /*struct*/ QString {
  pub fn QString_1<T: QString_QString_1>(value: T) -> QString {
    let rsthis = value.QString_1();
    return rsthis;
    // return 1;
  }
}

pub trait QString_QString_1 {
  fn QString_1(self) -> QString;
}
// QString(const QChar *, int) ctx.fn_proto_cpp
impl<'a> /*trait*/ QString_QString_1 for (usize,i32) {
  fn QString_1(self) -> QString {
    // unsafe{_ZN7QStringC2EPK5QChari()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN7QStringC2EPK5QChari", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QString{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qstring.h:219
// index:2
// Public Visibility=Default Availability=Available
// [-2] void QString(QChar)

/*
Constructs a null string. Null strings are also empty.

See also isEmpty().
*/
// QString(QChar) ctx.fn_proto_cpp
impl /*struct*/ QString {
  pub fn QString_2<T: QString_QString_2>(value: T) -> QString {
    let rsthis = value.QString_2();
    return rsthis;
    // return 1;
  }
}

pub trait QString_QString_2 {
  fn QString_2(self) -> QString;
}
// QString(QChar) ctx.fn_proto_cpp
impl<'a> /*trait*/ QString_QString_2 for (usize) {
  fn QString_2(self) -> QString {
    // unsafe{_ZN7QStringC2E5QChar()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN7QStringC2E5QChar", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QString{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qstring.h:220
// index:3
// Public Visibility=Default Availability=Available
// [-2] void QString(int, QChar)

/*
Constructs a null string. Null strings are also empty.

See also isEmpty().
*/
// QString(int, QChar) ctx.fn_proto_cpp
impl /*struct*/ QString {
  pub fn QString_3<T: QString_QString_3>(value: T) -> QString {
    let rsthis = value.QString_3();
    return rsthis;
    // return 1;
  }
}

pub trait QString_QString_3 {
  fn QString_3(self) -> QString;
}
// QString(int, QChar) ctx.fn_proto_cpp
impl<'a> /*trait*/ QString_QString_3 for (i32,usize) {
  fn QString_3(self) -> QString {
    // unsafe{_ZN7QStringC2Ei5QChar()};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN7QStringC2Ei5QChar", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QString{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qstring.h:221
// index:4
// Public inline Visibility=Default Availability=Available
// [-2] void QString(QLatin1String)

/*
Constructs a null string. Null strings are also empty.

See also isEmpty().
*/
// QString(QLatin1String) ctx.fn_proto_cpp
impl /*struct*/ QString {
  pub fn QString_4<T: QString_QString_4>(value: T) -> QString {
    let rsthis = value.QString_4();
    return rsthis;
    // return 1;
  }
}

pub trait QString_QString_4 {
  fn QString_4(self) -> QString;
}
// QString(QLatin1String) ctx.fn_proto_cpp
impl<'a> /*trait*/ QString_QString_4 for (usize) {
  fn QString_4(self) -> QString {
    // unsafe{_ZN7QStringC2E13QLatin1String()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN7QStringC2E13QLatin1String", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QString{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qstring.h:682
// index:5
// Public inline Visibility=Default Availability=Available
// [-2] void QString(const char *)

/*
Constructs a null string. Null strings are also empty.

See also isEmpty().
*/
// QString(const char *) ctx.fn_proto_cpp
impl /*struct*/ QString {
  pub fn QString_5<T: QString_QString_5>(value: T) -> QString {
    let rsthis = value.QString_5();
    return rsthis;
    // return 1;
  }
}

pub trait QString_QString_5 {
  fn QString_5(self) -> QString;
}
// QString(const char *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QString_QString_5 for (usize) {
  fn QString_5(self) -> QString {
    // unsafe{_ZN7QStringC2EPKc()};
    let arg0 = (self) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN7QStringC2EPKc", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QString{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qstring.h:685
// index:6
// Public inline Visibility=Default Availability=Available
// [-2] void QString(const QByteArray &)

/*
Constructs a null string. Null strings are also empty.

See also isEmpty().
*/
// QString(const QByteArray &) ctx.fn_proto_cpp
impl /*struct*/ QString {
  pub fn QString_6<T: QString_QString_6>(value: T) -> QString {
    let rsthis = value.QString_6();
    return rsthis;
    // return 1;
  }
}

pub trait QString_QString_6 {
  fn QString_6(self) -> QString;
}
// QString(const QByteArray &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QString_QString_6 for (usize) {
  fn QString_6(self) -> QString {
    // unsafe{_ZN7QStringC2ERK10QByteArray()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN7QStringC2ERK10QByteArray", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QString{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qstring.h:811
// index:7
// Public Visibility=Default Availability=Available
// [-2] void QString(int, Qt::Initialization)

/*
Constructs a null string. Null strings are also empty.

See also isEmpty().
*/
// QString(int, Qt::Initialization) ctx.fn_proto_cpp
impl /*struct*/ QString {
  pub fn QString_7<T: QString_QString_7>(value: T) -> QString {
    let rsthis = value.QString_7();
    return rsthis;
    // return 1;
  }
}

pub trait QString_QString_7 {
  fn QString_7(self) -> QString;
}
// QString(int, Qt::Initialization) ctx.fn_proto_cpp
impl<'a> /*trait*/ QString_QString_7 for (i32,i32) {
  fn QString_7(self) -> QString {
    // unsafe{_ZN7QStringC2EiN2Qt14InitializationE()};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN7QStringC2EiN2Qt14InitializationE", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QString{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qstring.h:223
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void ~QString()

/*

*/
pub fn DeleteQString(this :*mut QString) {
    // let rv = qtrt::InvokeQtFunc6("_ZN7QStringD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 8)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qstring.h:224
// index:0
// Public Visibility=Default Availability=Available
// [8] QString & operator=(QChar)

/*

*/
impl /*struct*/ QString {
  pub fn operator_equal_0<RetType, T: QString_operator_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_0(self);
    // return 1;
  }
}
pub trait QString_operator_equal_0<RetType> {
  fn operator_equal_0(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_operator_equal_0<usize> for (usize) {
  fn operator_equal_0(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QStringaSE5QChar", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:225
// index:1
// Public Visibility=Default Availability=Available
// [8] QString & operator=(const QString &)

/*

*/
impl /*struct*/ QString {
  pub fn operator_equal_1<RetType, T: QString_operator_equal_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_1(self);
    // return 1;
  }
}
pub trait QString_operator_equal_1<RetType> {
  fn operator_equal_1(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_operator_equal_1<usize> for (usize) {
  fn operator_equal_1(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QStringaSERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:226
// index:2
// Public Visibility=Default Availability=Available
// [8] QString & operator=(QLatin1String)

/*

*/
impl /*struct*/ QString {
  pub fn operator_equal_2<RetType, T: QString_operator_equal_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_2(self);
    // return 1;
  }
}
pub trait QString_operator_equal_2<RetType> {
  fn operator_equal_2(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_operator_equal_2<usize> for (usize) {
  fn operator_equal_2(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QStringaSE13QLatin1String", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:229
// index:3
// Public inline Visibility=Default Availability=Available
// [8] QString & operator=(QString &&)

/*

*/
impl /*struct*/ QString {
  pub fn operator_equal_3<RetType, T: QString_operator_equal_3<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_3(self);
    // return 1;
  }
}
pub trait QString_operator_equal_3<RetType> {
  fn operator_equal_3(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_operator_equal_3<usize> for (usize) {
  fn operator_equal_3(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QStringaSEOS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:688
// index:4
// Public inline Visibility=Default Availability=Available
// [8] QString & operator=(const char *)

/*

*/
impl /*struct*/ QString {
  pub fn operator_equal_4<RetType, T: QString_operator_equal_4<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_4(self);
    // return 1;
  }
}
pub trait QString_operator_equal_4<RetType> {
  fn operator_equal_4(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_operator_equal_4<usize> for (usize) {
  fn operator_equal_4(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QStringaSEPKc", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:690
// index:5
// Public inline Visibility=Default Availability=Available
// [8] QString & operator=(const QByteArray &)

/*

*/
impl /*struct*/ QString {
  pub fn operator_equal_5<RetType, T: QString_operator_equal_5<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_5(self);
    // return 1;
  }
}
pub trait QString_operator_equal_5<RetType> {
  fn operator_equal_5(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_operator_equal_5<usize> for (usize) {
  fn operator_equal_5(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QStringaSERK10QByteArray", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:692
// index:6
// Public inline Visibility=Default Availability=Available
// [8] QString & operator=(char)

/*

*/
impl /*struct*/ QString {
  pub fn operator_equal_6<RetType, T: QString_operator_equal_6<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_6(self);
    // return 1;
  }
}
pub trait QString_operator_equal_6<RetType> {
  fn operator_equal_6(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_operator_equal_6<usize> for (i8) {
  fn operator_equal_6(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i8 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QStringaSEc", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:232
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void swap(QString &)

/*
Swaps string other with this string. This operation is very fast and never fails.

This function was introduced in  Qt 4.8.
*/
impl /*struct*/ QString {
  pub fn swap_0<RetType, T: QString_swap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.swap_0(self);
    // return 1;
  }
}
pub trait QString_swap_0<RetType> {
  fn swap_0(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_swap_0<(/*void*/)> for (usize) {
  fn swap_0(self , rsthis: & QString) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QString4swapERS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qstring.h:233
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
impl /*struct*/ QString {
  pub fn size_0<RetType, T: QString_size_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.size_0(self);
    // return 1;
  }
}
pub trait QString_size_0<RetType> {
  fn size_0(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_size_0<i32> for () {
  fn size_0(self , rsthis: & QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString4sizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:234
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int count() const

/*
Returns the number of (potentially overlapping) occurrences of the string str in this string.

If cs is Qt::CaseSensitive (default), the search is case sensitive; otherwise the search is case insensitive.

See also contains() and indexOf().
*/
impl /*struct*/ QString {
  pub fn count_0<RetType, T: QString_count_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.count_0(self);
    // return 1;
  }
}
pub trait QString_count_0<RetType> {
  fn count_0(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_count_0<i32> for () {
  fn count_0(self , rsthis: & QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString5countEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:335
// index:1
// Public Visibility=Default Availability=Available
// [4] int count(QChar, Qt::CaseSensitivity) const

/*
Returns the number of (potentially overlapping) occurrences of the string str in this string.

If cs is Qt::CaseSensitive (default), the search is case sensitive; otherwise the search is case insensitive.

See also contains() and indexOf().
*/
impl /*struct*/ QString {
  pub fn count_1<RetType, T: QString_count_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.count_1(self);
    // return 1;
  }
}
pub trait QString_count_1<RetType> {
  fn count_1(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_count_1<i32> for (usize,i32) {
  fn count_1(self , rsthis: & QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString5countE5QCharN2Qt15CaseSensitivityE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:336
// index:2
// Public Visibility=Default Availability=Available
// [4] int count(const QString &, Qt::CaseSensitivity) const

/*
Returns the number of (potentially overlapping) occurrences of the string str in this string.

If cs is Qt::CaseSensitive (default), the search is case sensitive; otherwise the search is case insensitive.

See also contains() and indexOf().
*/
impl /*struct*/ QString {
  pub fn count_2<RetType, T: QString_count_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.count_2(self);
    // return 1;
  }
}
pub trait QString_count_2<RetType> {
  fn count_2(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_count_2<i32> for (usize,i32) {
  fn count_2(self , rsthis: & QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString5countERKS_N2Qt15CaseSensitivityE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:337
// index:3
// Public Visibility=Default Availability=Available
// [4] int count(const QStringRef &, Qt::CaseSensitivity) const

/*
Returns the number of (potentially overlapping) occurrences of the string str in this string.

If cs is Qt::CaseSensitive (default), the search is case sensitive; otherwise the search is case insensitive.

See also contains() and indexOf().
*/
impl /*struct*/ QString {
  pub fn count_3<RetType, T: QString_count_3<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.count_3(self);
    // return 1;
  }
}
pub trait QString_count_3<RetType> {
  fn count_3(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_count_3<i32> for (usize,i32) {
  fn count_3(self , rsthis: & QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString5countERK10QStringRefN2Qt15CaseSensitivityE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:343
// index:4
// Public Visibility=Default Availability=Available
// [4] int count(const QRegExp &) const

/*
Returns the number of (potentially overlapping) occurrences of the string str in this string.

If cs is Qt::CaseSensitive (default), the search is case sensitive; otherwise the search is case insensitive.

See also contains() and indexOf().
*/
impl /*struct*/ QString {
  pub fn count_4<RetType, T: QString_count_4<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.count_4(self);
    // return 1;
  }
}
pub trait QString_count_4<RetType> {
  fn count_4(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_count_4<i32> for (usize) {
  fn count_4(self , rsthis: & QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString5countERK7QRegExp", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:357
// index:5
// Public Visibility=Default Availability=Available
// [4] int count(const QRegularExpression &) const

/*
Returns the number of (potentially overlapping) occurrences of the string str in this string.

If cs is Qt::CaseSensitive (default), the search is case sensitive; otherwise the search is case insensitive.

See also contains() and indexOf().
*/
impl /*struct*/ QString {
  pub fn count_5<RetType, T: QString_count_5<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.count_5(self);
    // return 1;
  }
}
pub trait QString_count_5<RetType> {
  fn count_5(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_count_5<i32> for (usize) {
  fn count_5(self , rsthis: & QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString5countERK18QRegularExpression", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:235
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int length() const

/*
Returns the number of characters in this string. Equivalent to size().

See also resize().
*/
impl /*struct*/ QString {
  pub fn length_0<RetType, T: QString_length_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.length_0(self);
    // return 1;
  }
}
pub trait QString_length_0<RetType> {
  fn length_0(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_length_0<i32> for () {
  fn length_0(self , rsthis: & QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString6lengthEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:236
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
impl /*struct*/ QString {
  pub fn isEmpty_0<RetType, T: QString_isEmpty_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isEmpty_0(self);
    // return 1;
  }
}
pub trait QString_isEmpty_0<RetType> {
  fn isEmpty_0(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_isEmpty_0<bool> for () {
  fn isEmpty_0(self , rsthis: & QString) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString7isEmptyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:237
// index:0
// Public Visibility=Default Availability=Available
// [-2] void resize(int)

/*
Sets the size of the string to size characters.

If size is greater than the current size, the string is extended to make it size characters long with the extra characters added to the end. The new characters are uninitialized.

If size is less than the current size, characters are removed from the end.

Example:


  QString s = "Hello world";
  s.resize(5);
  // s == "Hello"

  s.resize(8);
  // s == "Hello???" (where ? stands for any character)



If you want to append a certain number of identical characters to the string, use the resize(int, QChar) overload.

If you want to expand the string so that it reaches a certain width and fill the new positions with a particular character, use the leftJustified() function:

If size is negative, it is equivalent to passing zero.


  QString r = "Hello";
  r = r.leftJustified(10, ' ');
  // r == "Hello     "



See also truncate() and reserve().
*/
impl /*struct*/ QString {
  pub fn resize_0<RetType, T: QString_resize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resize_0(self);
    // return 1;
  }
}
pub trait QString_resize_0<RetType> {
  fn resize_0(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_resize_0<(/*void*/)> for (i32) {
  fn resize_0(self , rsthis: & QString) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QString6resizeEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qstring.h:238
// index:1
// Public Visibility=Default Availability=Available
// [-2] void resize(int, QChar)

/*
Sets the size of the string to size characters.

If size is greater than the current size, the string is extended to make it size characters long with the extra characters added to the end. The new characters are uninitialized.

If size is less than the current size, characters are removed from the end.

Example:


  QString s = "Hello world";
  s.resize(5);
  // s == "Hello"

  s.resize(8);
  // s == "Hello???" (where ? stands for any character)



If you want to append a certain number of identical characters to the string, use the resize(int, QChar) overload.

If you want to expand the string so that it reaches a certain width and fill the new positions with a particular character, use the leftJustified() function:

If size is negative, it is equivalent to passing zero.


  QString r = "Hello";
  r = r.leftJustified(10, ' ');
  // r == "Hello     "



See also truncate() and reserve().
*/
impl /*struct*/ QString {
  pub fn resize_1<RetType, T: QString_resize_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resize_1(self);
    // return 1;
  }
}
pub trait QString_resize_1<RetType> {
  fn resize_1(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_resize_1<(/*void*/)> for (i32,usize) {
  fn resize_1(self , rsthis: & QString) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QString6resizeEi5QChar", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qstring.h:240
// index:0
// Public Visibility=Default Availability=Available
// [8] QString & fill(QChar, int)

/*
Sets every character in the string to character ch. If size is different from -1 (default), the string is resized to size beforehand.

Example:


  QString str = "Berlin";
  str.fill('z');
  // str == "zzzzzz"

  str.fill('A', 2);
  // str == "AA"



See also resize().
*/
impl /*struct*/ QString {
  pub fn fill_0<RetType, T: QString_fill_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fill_0(self);
    // return 1;
  }
}
pub trait QString_fill_0<RetType> {
  fn fill_0(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_fill_0<usize> for (usize,i32) {
  fn fill_0(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QString4fillE5QChari", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:241
// index:0
// Public Visibility=Default Availability=Available
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
impl /*struct*/ QString {
  pub fn truncate_0<RetType, T: QString_truncate_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.truncate_0(self);
    // return 1;
  }
}
pub trait QString_truncate_0<RetType> {
  fn truncate_0(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_truncate_0<(/*void*/)> for (i32) {
  fn truncate_0(self , rsthis: & QString) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QString8truncateEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qstring.h:242
// index:0
// Public Visibility=Default Availability=Available
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
impl /*struct*/ QString {
  pub fn chop_0<RetType, T: QString_chop_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.chop_0(self);
    // return 1;
  }
}
pub trait QString_chop_0<RetType> {
  fn chop_0(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_chop_0<(/*void*/)> for (i32) {
  fn chop_0(self , rsthis: & QString) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QString4chopEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qstring.h:244
// index:0
// Public Visibility=Default Availability=Available
// [4] int capacity() const

/*
Returns the maximum number of characters that can be stored in the string without forcing a reallocation.

The sole purpose of this function is to provide a means of fine tuning QString's memory usage. In general, you will rarely ever need to call this function. If you want to know how many characters are in the string, call size().

See also reserve() and squeeze().
*/
impl /*struct*/ QString {
  pub fn capacity_0<RetType, T: QString_capacity_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.capacity_0(self);
    // return 1;
  }
}
pub trait QString_capacity_0<RetType> {
  fn capacity_0(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_capacity_0<i32> for () {
  fn capacity_0(self , rsthis: & QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString8capacityEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:245
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void reserve(int)

/*
Attempts to allocate memory for at least size characters. If you know in advance how large the string will be, you can call this function, and if you resize the string often you are likely to get better performance. If size is an underestimate, the worst that will happen is that the QString will be a bit slower.

The sole purpose of this function is to provide a means of fine tuning QString's memory usage. In general, you will rarely ever need to call this function. If you want to change the size of the string, call resize().

This function is useful for code that needs to build up a long string and wants to avoid repeated reallocation. In this example, we want to add to the string until some condition is true, and we're fairly sure that size is large enough to make a call to reserve() worthwhile:


  QString result;
  int maxSize;
  bool condition;
  QChar nextChar;

  result.reserve(maxSize);

  while (condition)
      result.append(nextChar);

  result.squeeze();



See also squeeze() and capacity().
*/
impl /*struct*/ QString {
  pub fn reserve_0<RetType, T: QString_reserve_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.reserve_0(self);
    // return 1;
  }
}
pub trait QString_reserve_0<RetType> {
  fn reserve_0(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_reserve_0<(/*void*/)> for (i32) {
  fn reserve_0(self , rsthis: & QString) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QString7reserveEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qstring.h:246
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void squeeze()

/*
Releases any memory not required to store the character data.

The sole purpose of this function is to provide a means of fine tuning QString's memory usage. In general, you will rarely ever need to call this function.

See also reserve() and capacity().
*/
impl /*struct*/ QString {
  pub fn squeeze_0<RetType, T: QString_squeeze_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.squeeze_0(self);
    // return 1;
  }
}
pub trait QString_squeeze_0<RetType> {
  fn squeeze_0(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_squeeze_0<(/*void*/)> for () {
  fn squeeze_0(self , rsthis: & QString) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN7QString7squeezeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qstring.h:248
// index:0
// Public inline Visibility=Default Availability=Available
// [8] const QChar * unicode() const

/*
Returns a Unicode representation of the string. The result remains valid until the string is modified.

Note: The returned string may not be '\0'-terminated. Use size() to determine the length of the array.

See also setUnicode(), utf16(), and fromRawData().
*/
impl /*struct*/ QString {
  pub fn unicode_0<RetType, T: QString_unicode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.unicode_0(self);
    // return 1;
  }
}
pub trait QString_unicode_0<RetType> {
  fn unicode_0(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_unicode_0<usize> for () {
  fn unicode_0(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString7unicodeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:249
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QChar * data()

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
impl /*struct*/ QString {
  pub fn data_0<RetType, T: QString_data_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.data_0(self);
    // return 1;
  }
}
pub trait QString_data_0<RetType> {
  fn data_0(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_data_0<usize> for () {
  fn data_0(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QString4dataEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:250
// index:1
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
impl /*struct*/ QString {
  pub fn data_1<RetType, T: QString_data_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.data_1(self);
    // return 1;
  }
}
pub trait QString_data_1<RetType> {
  fn data_1(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_data_1<usize> for () {
  fn data_1(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString4dataEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:251
// index:0
// Public inline Visibility=Default Availability=Available
// [8] const QChar * constData() const

/*
Returns a pointer to the data stored in the QString. The pointer can be used to access the characters that compose the string.

Note that the pointer remains valid only as long as the string is not modified.

Note: The returned string may not be '\0'-terminated. Use size() to determine the length of the array.

See also data(), operator[](), and fromRawData().
*/
impl /*struct*/ QString {
  pub fn constData_0<RetType, T: QString_constData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.constData_0(self);
    // return 1;
  }
}
pub trait QString_constData_0<RetType> {
  fn constData_0(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_constData_0<usize> for () {
  fn constData_0(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString9constDataEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:253
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void detach()

/*

*/
impl /*struct*/ QString {
  pub fn detach_0<RetType, T: QString_detach_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.detach_0(self);
    // return 1;
  }
}
pub trait QString_detach_0<RetType> {
  fn detach_0(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_detach_0<(/*void*/)> for () {
  fn detach_0(self , rsthis: & QString) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN7QString6detachEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qstring.h:254
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isDetached() const

/*

*/
impl /*struct*/ QString {
  pub fn isDetached_0<RetType, T: QString_isDetached_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isDetached_0(self);
    // return 1;
  }
}
pub trait QString_isDetached_0<RetType> {
  fn isDetached_0(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_isDetached_0<bool> for () {
  fn isDetached_0(self , rsthis: & QString) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString10isDetachedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:255
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isSharedWith(const QString &) const

/*

*/
impl /*struct*/ QString {
  pub fn isSharedWith_0<RetType, T: QString_isSharedWith_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isSharedWith_0(self);
    // return 1;
  }
}
pub trait QString_isSharedWith_0<RetType> {
  fn isSharedWith_0(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_isSharedWith_0<bool> for (usize) {
  fn isSharedWith_0(self , rsthis: & QString) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString12isSharedWithERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:256
// index:0
// Public Visibility=Default Availability=Available
// [-2] void clear()

/*
Clears the contents of the string and makes it null.

See also resize() and isNull().
*/
impl /*struct*/ QString {
  pub fn clear_0<RetType, T: QString_clear_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.clear_0(self);
    // return 1;
  }
}
pub trait QString_clear_0<RetType> {
  fn clear_0(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_clear_0<(/*void*/)> for () {
  fn clear_0(self , rsthis: & QString) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN7QString5clearEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qstring.h:258
// index:0
// Public inline Visibility=Default Availability=Available
// [2] const QChar at(int) const

/*
Returns the character at the given index position in the string.

The position must be a valid index position in the string (i.e., 0 <= position < size()).

See also operator[]().
*/
impl /*struct*/ QString {
  pub fn at_0<RetType, T: QString_at_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.at_0(self);
    // return 1;
  }
}
pub trait QString_at_0<RetType> {
  fn at_0(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_at_0<usize> for (i32) {
  fn at_0(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString2atEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:259
// index:0
// Public Visibility=Default Availability=Available
// [2] const QChar operator[](int) const

/*

*/
impl /*struct*/ QString {
  pub fn operator_get_index_0<RetType, T: QString_operator_get_index_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_get_index_0(self);
    // return 1;
  }
}
pub trait QString_operator_get_index_0<RetType> {
  fn operator_get_index_0(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_operator_get_index_0<usize> for (i32) {
  fn operator_get_index_0(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QStringixEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:260
// index:1
// Public Visibility=Default Availability=Available
// [16] QCharRef operator[](int)

/*

*/
impl /*struct*/ QString {
  pub fn operator_get_index_1<RetType, T: QString_operator_get_index_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_get_index_1(self);
    // return 1;
  }
}
pub trait QString_operator_get_index_1<RetType> {
  fn operator_get_index_1(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_operator_get_index_1<usize> for (i32) {
  fn operator_get_index_1(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QStringixEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:261
// index:2
// Public Visibility=Default Availability=Available
// [2] const QChar operator[](uint) const

/*

*/
impl /*struct*/ QString {
  pub fn operator_get_index_2<RetType, T: QString_operator_get_index_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_get_index_2(self);
    // return 1;
  }
}
pub trait QString_operator_get_index_2<RetType> {
  fn operator_get_index_2(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_operator_get_index_2<usize> for (u32) {
  fn operator_get_index_2(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const u32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QStringixEj", 1,qtrt::FFITY_UINT32,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:262
// index:3
// Public Visibility=Default Availability=Available
// [16] QCharRef operator[](uint)

/*

*/
impl /*struct*/ QString {
  pub fn operator_get_index_3<RetType, T: QString_operator_get_index_3<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_get_index_3(self);
    // return 1;
  }
}
pub trait QString_operator_get_index_3<RetType> {
  fn operator_get_index_3(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_operator_get_index_3<usize> for (u32) {
  fn operator_get_index_3(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const u32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QStringixEj", 1,qtrt::FFITY_UINT32,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:264
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
impl /*struct*/ QString {
  pub fn front_0<RetType, T: QString_front_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.front_0(self);
    // return 1;
  }
}
pub trait QString_front_0<RetType> {
  fn front_0(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_front_0<usize> for () {
  fn front_0(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString5frontEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:265
// index:1
// Public inline Visibility=Default Availability=Available
// [16] QCharRef front()

/*
Returns the first character in the string. Same as at(0).

This function is provided for STL compatibility.

Warning: Calling this function on an empty string constitutes undefined behavior.

This function was introduced in  Qt 5.10.

See also back(), at(), and operator[]().
*/
impl /*struct*/ QString {
  pub fn front_1<RetType, T: QString_front_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.front_1(self);
    // return 1;
  }
}
pub trait QString_front_1<RetType> {
  fn front_1(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_front_1<usize> for () {
  fn front_1(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QString5frontEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:266
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
impl /*struct*/ QString {
  pub fn back_0<RetType, T: QString_back_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.back_0(self);
    // return 1;
  }
}
pub trait QString_back_0<RetType> {
  fn back_0(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_back_0<usize> for () {
  fn back_0(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString4backEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:267
// index:1
// Public inline Visibility=Default Availability=Available
// [16] QCharRef back()

/*
Returns the last character in the string. Same as at(size() - 1).

This function is provided for STL compatibility.

Warning: Calling this function on an empty string constitutes undefined behavior.

This function was introduced in  Qt 5.10.

See also front(), at(), and operator[]().
*/
impl /*struct*/ QString {
  pub fn back_1<RetType, T: QString_back_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.back_1(self);
    // return 1;
  }
}
pub trait QString_back_1<RetType> {
  fn back_1(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_back_1<usize> for () {
  fn back_1(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QString4backEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:269
// index:0
// Public Visibility=Default Availability=Available
// [8] QString arg(qlonglong, int, int, QChar) const

/*
Returns a copy of this string with the lowest numbered place marker replaced by string a, i.e., %1, %2, ..., %99.

fieldWidth specifies the minimum amount of space that argument a shall occupy. If a requires less space than fieldWidth, it is padded to fieldWidth with character fillChar. A positive fieldWidth produces right-aligned text. A negative fieldWidth produces left-aligned text.

This example shows how we might create a status string for reporting progress while processing a list of files:


  QString i;           // current file's number
  QString total;       // number of files to process
  QString fileName;    // current file's name

  QString status = QString("Processing file %1 of %2: %3")
                  .arg(i).arg(total).arg(fileName);



First, arg(i) replaces %1. Then arg(total) replaces %2. Finally, arg(fileName) replaces %3.

One advantage of using arg() over asprintf() is that the order of the numbered place markers can change, if the application's strings are translated into other languages, but each arg() will still replace the lowest numbered unreplaced place marker, no matter where it appears. Also, if place marker %i appears more than once in the string, the arg() replaces all of them.

If there is no unreplaced place marker remaining, a warning message is output and the result is undefined. Place marker numbers must be in the range 1 to 99.
*/
impl /*struct*/ QString {
  pub fn arg_0<RetType, T: QString_arg_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.arg_0(self);
    // return 1;
  }
}
pub trait QString_arg_0<RetType> {
  fn arg_0(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_arg_0<usize> for (i64,i32,i32,usize) {
  fn arg_0(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i64 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString3argExii5QChar", 4,qtrt::FFITY_SINT64,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:271
// index:1
// Public Visibility=Default Availability=Available
// [8] QString arg(qulonglong, int, int, QChar) const

/*
Returns a copy of this string with the lowest numbered place marker replaced by string a, i.e., %1, %2, ..., %99.

fieldWidth specifies the minimum amount of space that argument a shall occupy. If a requires less space than fieldWidth, it is padded to fieldWidth with character fillChar. A positive fieldWidth produces right-aligned text. A negative fieldWidth produces left-aligned text.

This example shows how we might create a status string for reporting progress while processing a list of files:


  QString i;           // current file's number
  QString total;       // number of files to process
  QString fileName;    // current file's name

  QString status = QString("Processing file %1 of %2: %3")
                  .arg(i).arg(total).arg(fileName);



First, arg(i) replaces %1. Then arg(total) replaces %2. Finally, arg(fileName) replaces %3.

One advantage of using arg() over asprintf() is that the order of the numbered place markers can change, if the application's strings are translated into other languages, but each arg() will still replace the lowest numbered unreplaced place marker, no matter where it appears. Also, if place marker %i appears more than once in the string, the arg() replaces all of them.

If there is no unreplaced place marker remaining, a warning message is output and the result is undefined. Place marker numbers must be in the range 1 to 99.
*/
impl /*struct*/ QString {
  pub fn arg_1<RetType, T: QString_arg_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.arg_1(self);
    // return 1;
  }
}
pub trait QString_arg_1<RetType> {
  fn arg_1(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_arg_1<usize> for (u64,i32,i32,usize) {
  fn arg_1(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const u64 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString3argEyii5QChar", 4,qtrt::FFITY_UINT64,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:273
// index:2
// Public Visibility=Default Availability=Available
// [8] QString arg(long, int, int, QChar) const

/*
Returns a copy of this string with the lowest numbered place marker replaced by string a, i.e., %1, %2, ..., %99.

fieldWidth specifies the minimum amount of space that argument a shall occupy. If a requires less space than fieldWidth, it is padded to fieldWidth with character fillChar. A positive fieldWidth produces right-aligned text. A negative fieldWidth produces left-aligned text.

This example shows how we might create a status string for reporting progress while processing a list of files:


  QString i;           // current file's number
  QString total;       // number of files to process
  QString fileName;    // current file's name

  QString status = QString("Processing file %1 of %2: %3")
                  .arg(i).arg(total).arg(fileName);



First, arg(i) replaces %1. Then arg(total) replaces %2. Finally, arg(fileName) replaces %3.

One advantage of using arg() over asprintf() is that the order of the numbered place markers can change, if the application's strings are translated into other languages, but each arg() will still replace the lowest numbered unreplaced place marker, no matter where it appears. Also, if place marker %i appears more than once in the string, the arg() replaces all of them.

If there is no unreplaced place marker remaining, a warning message is output and the result is undefined. Place marker numbers must be in the range 1 to 99.
*/
impl /*struct*/ QString {
  pub fn arg_2<RetType, T: QString_arg_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.arg_2(self);
    // return 1;
  }
}
pub trait QString_arg_2<RetType> {
  fn arg_2(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_arg_2<usize> for (i64,i32,i32,usize) {
  fn arg_2(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i64 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString3argElii5QChar", 4,qtrt::FFITY_SINT64,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:275
// index:3
// Public Visibility=Default Availability=Available
// [8] QString arg(ulong, int, int, QChar) const

/*
Returns a copy of this string with the lowest numbered place marker replaced by string a, i.e., %1, %2, ..., %99.

fieldWidth specifies the minimum amount of space that argument a shall occupy. If a requires less space than fieldWidth, it is padded to fieldWidth with character fillChar. A positive fieldWidth produces right-aligned text. A negative fieldWidth produces left-aligned text.

This example shows how we might create a status string for reporting progress while processing a list of files:


  QString i;           // current file's number
  QString total;       // number of files to process
  QString fileName;    // current file's name

  QString status = QString("Processing file %1 of %2: %3")
                  .arg(i).arg(total).arg(fileName);



First, arg(i) replaces %1. Then arg(total) replaces %2. Finally, arg(fileName) replaces %3.

One advantage of using arg() over asprintf() is that the order of the numbered place markers can change, if the application's strings are translated into other languages, but each arg() will still replace the lowest numbered unreplaced place marker, no matter where it appears. Also, if place marker %i appears more than once in the string, the arg() replaces all of them.

If there is no unreplaced place marker remaining, a warning message is output and the result is undefined. Place marker numbers must be in the range 1 to 99.
*/
impl /*struct*/ QString {
  pub fn arg_3<RetType, T: QString_arg_3<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.arg_3(self);
    // return 1;
  }
}
pub trait QString_arg_3<RetType> {
  fn arg_3(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_arg_3<usize> for (u64,i32,i32,usize) {
  fn arg_3(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const u64 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString3argEmii5QChar", 4,qtrt::FFITY_UINT64,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:277
// index:4
// Public Visibility=Default Availability=Available
// [8] QString arg(int, int, int, QChar) const

/*
Returns a copy of this string with the lowest numbered place marker replaced by string a, i.e., %1, %2, ..., %99.

fieldWidth specifies the minimum amount of space that argument a shall occupy. If a requires less space than fieldWidth, it is padded to fieldWidth with character fillChar. A positive fieldWidth produces right-aligned text. A negative fieldWidth produces left-aligned text.

This example shows how we might create a status string for reporting progress while processing a list of files:


  QString i;           // current file's number
  QString total;       // number of files to process
  QString fileName;    // current file's name

  QString status = QString("Processing file %1 of %2: %3")
                  .arg(i).arg(total).arg(fileName);



First, arg(i) replaces %1. Then arg(total) replaces %2. Finally, arg(fileName) replaces %3.

One advantage of using arg() over asprintf() is that the order of the numbered place markers can change, if the application's strings are translated into other languages, but each arg() will still replace the lowest numbered unreplaced place marker, no matter where it appears. Also, if place marker %i appears more than once in the string, the arg() replaces all of them.

If there is no unreplaced place marker remaining, a warning message is output and the result is undefined. Place marker numbers must be in the range 1 to 99.
*/
impl /*struct*/ QString {
  pub fn arg_4<RetType, T: QString_arg_4<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.arg_4(self);
    // return 1;
  }
}
pub trait QString_arg_4<RetType> {
  fn arg_4(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_arg_4<usize> for (i32,i32,i32,usize) {
  fn arg_4(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString3argEiii5QChar", 4,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:279
// index:5
// Public Visibility=Default Availability=Available
// [8] QString arg(uint, int, int, QChar) const

/*
Returns a copy of this string with the lowest numbered place marker replaced by string a, i.e., %1, %2, ..., %99.

fieldWidth specifies the minimum amount of space that argument a shall occupy. If a requires less space than fieldWidth, it is padded to fieldWidth with character fillChar. A positive fieldWidth produces right-aligned text. A negative fieldWidth produces left-aligned text.

This example shows how we might create a status string for reporting progress while processing a list of files:


  QString i;           // current file's number
  QString total;       // number of files to process
  QString fileName;    // current file's name

  QString status = QString("Processing file %1 of %2: %3")
                  .arg(i).arg(total).arg(fileName);



First, arg(i) replaces %1. Then arg(total) replaces %2. Finally, arg(fileName) replaces %3.

One advantage of using arg() over asprintf() is that the order of the numbered place markers can change, if the application's strings are translated into other languages, but each arg() will still replace the lowest numbered unreplaced place marker, no matter where it appears. Also, if place marker %i appears more than once in the string, the arg() replaces all of them.

If there is no unreplaced place marker remaining, a warning message is output and the result is undefined. Place marker numbers must be in the range 1 to 99.
*/
impl /*struct*/ QString {
  pub fn arg_5<RetType, T: QString_arg_5<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.arg_5(self);
    // return 1;
  }
}
pub trait QString_arg_5<RetType> {
  fn arg_5(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_arg_5<usize> for (u32,i32,i32,usize) {
  fn arg_5(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const u32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString3argEjii5QChar", 4,qtrt::FFITY_UINT32,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:281
// index:6
// Public Visibility=Default Availability=Available
// [8] QString arg(short, int, int, QChar) const

/*
Returns a copy of this string with the lowest numbered place marker replaced by string a, i.e., %1, %2, ..., %99.

fieldWidth specifies the minimum amount of space that argument a shall occupy. If a requires less space than fieldWidth, it is padded to fieldWidth with character fillChar. A positive fieldWidth produces right-aligned text. A negative fieldWidth produces left-aligned text.

This example shows how we might create a status string for reporting progress while processing a list of files:


  QString i;           // current file's number
  QString total;       // number of files to process
  QString fileName;    // current file's name

  QString status = QString("Processing file %1 of %2: %3")
                  .arg(i).arg(total).arg(fileName);



First, arg(i) replaces %1. Then arg(total) replaces %2. Finally, arg(fileName) replaces %3.

One advantage of using arg() over asprintf() is that the order of the numbered place markers can change, if the application's strings are translated into other languages, but each arg() will still replace the lowest numbered unreplaced place marker, no matter where it appears. Also, if place marker %i appears more than once in the string, the arg() replaces all of them.

If there is no unreplaced place marker remaining, a warning message is output and the result is undefined. Place marker numbers must be in the range 1 to 99.
*/
impl /*struct*/ QString {
  pub fn arg_6<RetType, T: QString_arg_6<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.arg_6(self);
    // return 1;
  }
}
pub trait QString_arg_6<RetType> {
  fn arg_6(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_arg_6<usize> for (i16,i32,i32,usize) {
  fn arg_6(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i16 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString3argEsii5QChar", 4,qtrt::FFITY_SINT16,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:283
// index:7
// Public Visibility=Default Availability=Available
// [8] QString arg(ushort, int, int, QChar) const

/*
Returns a copy of this string with the lowest numbered place marker replaced by string a, i.e., %1, %2, ..., %99.

fieldWidth specifies the minimum amount of space that argument a shall occupy. If a requires less space than fieldWidth, it is padded to fieldWidth with character fillChar. A positive fieldWidth produces right-aligned text. A negative fieldWidth produces left-aligned text.

This example shows how we might create a status string for reporting progress while processing a list of files:


  QString i;           // current file's number
  QString total;       // number of files to process
  QString fileName;    // current file's name

  QString status = QString("Processing file %1 of %2: %3")
                  .arg(i).arg(total).arg(fileName);



First, arg(i) replaces %1. Then arg(total) replaces %2. Finally, arg(fileName) replaces %3.

One advantage of using arg() over asprintf() is that the order of the numbered place markers can change, if the application's strings are translated into other languages, but each arg() will still replace the lowest numbered unreplaced place marker, no matter where it appears. Also, if place marker %i appears more than once in the string, the arg() replaces all of them.

If there is no unreplaced place marker remaining, a warning message is output and the result is undefined. Place marker numbers must be in the range 1 to 99.
*/
impl /*struct*/ QString {
  pub fn arg_7<RetType, T: QString_arg_7<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.arg_7(self);
    // return 1;
  }
}
pub trait QString_arg_7<RetType> {
  fn arg_7(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_arg_7<usize> for (u16,i32,i32,usize) {
  fn arg_7(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const u16 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString3argEtii5QChar", 4,qtrt::FFITY_UINT16,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:285
// index:8
// Public Visibility=Default Availability=Available
// [8] QString arg(double, int, char, int, QChar) const

/*
Returns a copy of this string with the lowest numbered place marker replaced by string a, i.e., %1, %2, ..., %99.

fieldWidth specifies the minimum amount of space that argument a shall occupy. If a requires less space than fieldWidth, it is padded to fieldWidth with character fillChar. A positive fieldWidth produces right-aligned text. A negative fieldWidth produces left-aligned text.

This example shows how we might create a status string for reporting progress while processing a list of files:


  QString i;           // current file's number
  QString total;       // number of files to process
  QString fileName;    // current file's name

  QString status = QString("Processing file %1 of %2: %3")
                  .arg(i).arg(total).arg(fileName);



First, arg(i) replaces %1. Then arg(total) replaces %2. Finally, arg(fileName) replaces %3.

One advantage of using arg() over asprintf() is that the order of the numbered place markers can change, if the application's strings are translated into other languages, but each arg() will still replace the lowest numbered unreplaced place marker, no matter where it appears. Also, if place marker %i appears more than once in the string, the arg() replaces all of them.

If there is no unreplaced place marker remaining, a warning message is output and the result is undefined. Place marker numbers must be in the range 1 to 99.
*/
impl /*struct*/ QString {
  pub fn arg_8<RetType, T: QString_arg_8<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.arg_8(self);
    // return 1;
  }
}
pub trait QString_arg_8<RetType> {
  fn arg_8(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_arg_8<usize> for (f64,i32,i8,i32,usize) {
  fn arg_8(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i8 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let arg4 = (&self.4/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString3argEdici5QChar", 5,qtrt::FFITY_DOUBLE,qtrt::FFITY_INT,qtrt::FFITY_SINT8,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:287
// index:9
// Public Visibility=Default Availability=Available
// [8] QString arg(char, int, QChar) const

/*
Returns a copy of this string with the lowest numbered place marker replaced by string a, i.e., %1, %2, ..., %99.

fieldWidth specifies the minimum amount of space that argument a shall occupy. If a requires less space than fieldWidth, it is padded to fieldWidth with character fillChar. A positive fieldWidth produces right-aligned text. A negative fieldWidth produces left-aligned text.

This example shows how we might create a status string for reporting progress while processing a list of files:


  QString i;           // current file's number
  QString total;       // number of files to process
  QString fileName;    // current file's name

  QString status = QString("Processing file %1 of %2: %3")
                  .arg(i).arg(total).arg(fileName);



First, arg(i) replaces %1. Then arg(total) replaces %2. Finally, arg(fileName) replaces %3.

One advantage of using arg() over asprintf() is that the order of the numbered place markers can change, if the application's strings are translated into other languages, but each arg() will still replace the lowest numbered unreplaced place marker, no matter where it appears. Also, if place marker %i appears more than once in the string, the arg() replaces all of them.

If there is no unreplaced place marker remaining, a warning message is output and the result is undefined. Place marker numbers must be in the range 1 to 99.
*/
impl /*struct*/ QString {
  pub fn arg_9<RetType, T: QString_arg_9<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.arg_9(self);
    // return 1;
  }
}
pub trait QString_arg_9<RetType> {
  fn arg_9(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_arg_9<usize> for (i8,i32,usize) {
  fn arg_9(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i8 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString3argEci5QChar", 3,qtrt::FFITY_SINT8,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:289
// index:10
// Public Visibility=Default Availability=Available
// [8] QString arg(QChar, int, QChar) const

/*
Returns a copy of this string with the lowest numbered place marker replaced by string a, i.e., %1, %2, ..., %99.

fieldWidth specifies the minimum amount of space that argument a shall occupy. If a requires less space than fieldWidth, it is padded to fieldWidth with character fillChar. A positive fieldWidth produces right-aligned text. A negative fieldWidth produces left-aligned text.

This example shows how we might create a status string for reporting progress while processing a list of files:


  QString i;           // current file's number
  QString total;       // number of files to process
  QString fileName;    // current file's name

  QString status = QString("Processing file %1 of %2: %3")
                  .arg(i).arg(total).arg(fileName);



First, arg(i) replaces %1. Then arg(total) replaces %2. Finally, arg(fileName) replaces %3.

One advantage of using arg() over asprintf() is that the order of the numbered place markers can change, if the application's strings are translated into other languages, but each arg() will still replace the lowest numbered unreplaced place marker, no matter where it appears. Also, if place marker %i appears more than once in the string, the arg() replaces all of them.

If there is no unreplaced place marker remaining, a warning message is output and the result is undefined. Place marker numbers must be in the range 1 to 99.
*/
impl /*struct*/ QString {
  pub fn arg_10<RetType, T: QString_arg_10<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.arg_10(self);
    // return 1;
  }
}
pub trait QString_arg_10<RetType> {
  fn arg_10(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_arg_10<usize> for (usize,i32,usize) {
  fn arg_10(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString3argE5QChariS0_", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:292
// index:11
// Public Visibility=Default Availability=Available
// [8] QString arg(const QString &, int, QChar) const

/*
Returns a copy of this string with the lowest numbered place marker replaced by string a, i.e., %1, %2, ..., %99.

fieldWidth specifies the minimum amount of space that argument a shall occupy. If a requires less space than fieldWidth, it is padded to fieldWidth with character fillChar. A positive fieldWidth produces right-aligned text. A negative fieldWidth produces left-aligned text.

This example shows how we might create a status string for reporting progress while processing a list of files:


  QString i;           // current file's number
  QString total;       // number of files to process
  QString fileName;    // current file's name

  QString status = QString("Processing file %1 of %2: %3")
                  .arg(i).arg(total).arg(fileName);



First, arg(i) replaces %1. Then arg(total) replaces %2. Finally, arg(fileName) replaces %3.

One advantage of using arg() over asprintf() is that the order of the numbered place markers can change, if the application's strings are translated into other languages, but each arg() will still replace the lowest numbered unreplaced place marker, no matter where it appears. Also, if place marker %i appears more than once in the string, the arg() replaces all of them.

If there is no unreplaced place marker remaining, a warning message is output and the result is undefined. Place marker numbers must be in the range 1 to 99.
*/
impl /*struct*/ QString {
  pub fn arg_11<RetType, T: QString_arg_11<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.arg_11(self);
    // return 1;
  }
}
pub trait QString_arg_11<RetType> {
  fn arg_11(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_arg_11<usize> for (usize,i32,usize) {
  fn arg_11(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString3argERKS_i5QChar", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:295
// index:12
// Public Visibility=Default Availability=Available
// [8] QString arg(QStringView, int, QChar) const

/*
Returns a copy of this string with the lowest numbered place marker replaced by string a, i.e., %1, %2, ..., %99.

fieldWidth specifies the minimum amount of space that argument a shall occupy. If a requires less space than fieldWidth, it is padded to fieldWidth with character fillChar. A positive fieldWidth produces right-aligned text. A negative fieldWidth produces left-aligned text.

This example shows how we might create a status string for reporting progress while processing a list of files:


  QString i;           // current file's number
  QString total;       // number of files to process
  QString fileName;    // current file's name

  QString status = QString("Processing file %1 of %2: %3")
                  .arg(i).arg(total).arg(fileName);



First, arg(i) replaces %1. Then arg(total) replaces %2. Finally, arg(fileName) replaces %3.

One advantage of using arg() over asprintf() is that the order of the numbered place markers can change, if the application's strings are translated into other languages, but each arg() will still replace the lowest numbered unreplaced place marker, no matter where it appears. Also, if place marker %i appears more than once in the string, the arg() replaces all of them.

If there is no unreplaced place marker remaining, a warning message is output and the result is undefined. Place marker numbers must be in the range 1 to 99.
*/
impl /*struct*/ QString {
  pub fn arg_12<RetType, T: QString_arg_12<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.arg_12(self);
    // return 1;
  }
}
pub trait QString_arg_12<RetType> {
  fn arg_12(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_arg_12<usize> for (usize,i32,usize) {
  fn arg_12(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString3argE11QStringViewi5QChar", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:297
// index:13
// Public Visibility=Default Availability=Available
// [8] QString arg(QLatin1String, int, QChar) const

/*
Returns a copy of this string with the lowest numbered place marker replaced by string a, i.e., %1, %2, ..., %99.

fieldWidth specifies the minimum amount of space that argument a shall occupy. If a requires less space than fieldWidth, it is padded to fieldWidth with character fillChar. A positive fieldWidth produces right-aligned text. A negative fieldWidth produces left-aligned text.

This example shows how we might create a status string for reporting progress while processing a list of files:


  QString i;           // current file's number
  QString total;       // number of files to process
  QString fileName;    // current file's name

  QString status = QString("Processing file %1 of %2: %3")
                  .arg(i).arg(total).arg(fileName);



First, arg(i) replaces %1. Then arg(total) replaces %2. Finally, arg(fileName) replaces %3.

One advantage of using arg() over asprintf() is that the order of the numbered place markers can change, if the application's strings are translated into other languages, but each arg() will still replace the lowest numbered unreplaced place marker, no matter where it appears. Also, if place marker %i appears more than once in the string, the arg() replaces all of them.

If there is no unreplaced place marker remaining, a warning message is output and the result is undefined. Place marker numbers must be in the range 1 to 99.
*/
impl /*struct*/ QString {
  pub fn arg_13<RetType, T: QString_arg_13<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.arg_13(self);
    // return 1;
  }
}
pub trait QString_arg_13<RetType> {
  fn arg_13(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_arg_13<usize> for (usize,i32,usize) {
  fn arg_13(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString3argE13QLatin1Stringi5QChar", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:299
// index:14
// Public Visibility=Default Availability=Available
// [8] QString arg(const QString &, const QString &) const

/*
Returns a copy of this string with the lowest numbered place marker replaced by string a, i.e., %1, %2, ..., %99.

fieldWidth specifies the minimum amount of space that argument a shall occupy. If a requires less space than fieldWidth, it is padded to fieldWidth with character fillChar. A positive fieldWidth produces right-aligned text. A negative fieldWidth produces left-aligned text.

This example shows how we might create a status string for reporting progress while processing a list of files:


  QString i;           // current file's number
  QString total;       // number of files to process
  QString fileName;    // current file's name

  QString status = QString("Processing file %1 of %2: %3")
                  .arg(i).arg(total).arg(fileName);



First, arg(i) replaces %1. Then arg(total) replaces %2. Finally, arg(fileName) replaces %3.

One advantage of using arg() over asprintf() is that the order of the numbered place markers can change, if the application's strings are translated into other languages, but each arg() will still replace the lowest numbered unreplaced place marker, no matter where it appears. Also, if place marker %i appears more than once in the string, the arg() replaces all of them.

If there is no unreplaced place marker remaining, a warning message is output and the result is undefined. Place marker numbers must be in the range 1 to 99.
*/
impl /*struct*/ QString {
  pub fn arg_14<RetType, T: QString_arg_14<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.arg_14(self);
    // return 1;
  }
}
pub trait QString_arg_14<RetType> {
  fn arg_14(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_arg_14<usize> for (usize,usize) {
  fn arg_14(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString3argERKS_S1_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:300
// index:15
// Public Visibility=Default Availability=Available
// [8] QString arg(const QString &, const QString &, const QString &) const

/*
Returns a copy of this string with the lowest numbered place marker replaced by string a, i.e., %1, %2, ..., %99.

fieldWidth specifies the minimum amount of space that argument a shall occupy. If a requires less space than fieldWidth, it is padded to fieldWidth with character fillChar. A positive fieldWidth produces right-aligned text. A negative fieldWidth produces left-aligned text.

This example shows how we might create a status string for reporting progress while processing a list of files:


  QString i;           // current file's number
  QString total;       // number of files to process
  QString fileName;    // current file's name

  QString status = QString("Processing file %1 of %2: %3")
                  .arg(i).arg(total).arg(fileName);



First, arg(i) replaces %1. Then arg(total) replaces %2. Finally, arg(fileName) replaces %3.

One advantage of using arg() over asprintf() is that the order of the numbered place markers can change, if the application's strings are translated into other languages, but each arg() will still replace the lowest numbered unreplaced place marker, no matter where it appears. Also, if place marker %i appears more than once in the string, the arg() replaces all of them.

If there is no unreplaced place marker remaining, a warning message is output and the result is undefined. Place marker numbers must be in the range 1 to 99.
*/
impl /*struct*/ QString {
  pub fn arg_15<RetType, T: QString_arg_15<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.arg_15(self);
    // return 1;
  }
}
pub trait QString_arg_15<RetType> {
  fn arg_15(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_arg_15<usize> for (usize,usize,usize) {
  fn arg_15(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString3argERKS_S1_S1_", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:301
// index:16
// Public Visibility=Default Availability=Available
// [8] QString arg(const QString &, const QString &, const QString &, const QString &) const

/*
Returns a copy of this string with the lowest numbered place marker replaced by string a, i.e., %1, %2, ..., %99.

fieldWidth specifies the minimum amount of space that argument a shall occupy. If a requires less space than fieldWidth, it is padded to fieldWidth with character fillChar. A positive fieldWidth produces right-aligned text. A negative fieldWidth produces left-aligned text.

This example shows how we might create a status string for reporting progress while processing a list of files:


  QString i;           // current file's number
  QString total;       // number of files to process
  QString fileName;    // current file's name

  QString status = QString("Processing file %1 of %2: %3")
                  .arg(i).arg(total).arg(fileName);



First, arg(i) replaces %1. Then arg(total) replaces %2. Finally, arg(fileName) replaces %3.

One advantage of using arg() over asprintf() is that the order of the numbered place markers can change, if the application's strings are translated into other languages, but each arg() will still replace the lowest numbered unreplaced place marker, no matter where it appears. Also, if place marker %i appears more than once in the string, the arg() replaces all of them.

If there is no unreplaced place marker remaining, a warning message is output and the result is undefined. Place marker numbers must be in the range 1 to 99.
*/
impl /*struct*/ QString {
  pub fn arg_16<RetType, T: QString_arg_16<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.arg_16(self);
    // return 1;
  }
}
pub trait QString_arg_16<RetType> {
  fn arg_16(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_arg_16<usize> for (usize,usize,usize,usize) {
  fn arg_16(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString3argERKS_S1_S1_S1_", 4,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:303
// index:17
// Public Visibility=Default Availability=Available
// [8] QString arg(const QString &, const QString &, const QString &, const QString &, const QString &) const

/*
Returns a copy of this string with the lowest numbered place marker replaced by string a, i.e., %1, %2, ..., %99.

fieldWidth specifies the minimum amount of space that argument a shall occupy. If a requires less space than fieldWidth, it is padded to fieldWidth with character fillChar. A positive fieldWidth produces right-aligned text. A negative fieldWidth produces left-aligned text.

This example shows how we might create a status string for reporting progress while processing a list of files:


  QString i;           // current file's number
  QString total;       // number of files to process
  QString fileName;    // current file's name

  QString status = QString("Processing file %1 of %2: %3")
                  .arg(i).arg(total).arg(fileName);



First, arg(i) replaces %1. Then arg(total) replaces %2. Finally, arg(fileName) replaces %3.

One advantage of using arg() over asprintf() is that the order of the numbered place markers can change, if the application's strings are translated into other languages, but each arg() will still replace the lowest numbered unreplaced place marker, no matter where it appears. Also, if place marker %i appears more than once in the string, the arg() replaces all of them.

If there is no unreplaced place marker remaining, a warning message is output and the result is undefined. Place marker numbers must be in the range 1 to 99.
*/
impl /*struct*/ QString {
  pub fn arg_17<RetType, T: QString_arg_17<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.arg_17(self);
    // return 1;
  }
}
pub trait QString_arg_17<RetType> {
  fn arg_17(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_arg_17<usize> for (usize,usize,usize,usize,usize) {
  fn arg_17(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
    let arg4 = (&self.4/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString3argERKS_S1_S1_S1_S1_", 5,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:305
// index:18
// Public Visibility=Default Availability=Available
// [8] QString arg(const QString &, const QString &, const QString &, const QString &, const QString &, const QString &) const

/*
Returns a copy of this string with the lowest numbered place marker replaced by string a, i.e., %1, %2, ..., %99.

fieldWidth specifies the minimum amount of space that argument a shall occupy. If a requires less space than fieldWidth, it is padded to fieldWidth with character fillChar. A positive fieldWidth produces right-aligned text. A negative fieldWidth produces left-aligned text.

This example shows how we might create a status string for reporting progress while processing a list of files:


  QString i;           // current file's number
  QString total;       // number of files to process
  QString fileName;    // current file's name

  QString status = QString("Processing file %1 of %2: %3")
                  .arg(i).arg(total).arg(fileName);



First, arg(i) replaces %1. Then arg(total) replaces %2. Finally, arg(fileName) replaces %3.

One advantage of using arg() over asprintf() is that the order of the numbered place markers can change, if the application's strings are translated into other languages, but each arg() will still replace the lowest numbered unreplaced place marker, no matter where it appears. Also, if place marker %i appears more than once in the string, the arg() replaces all of them.

If there is no unreplaced place marker remaining, a warning message is output and the result is undefined. Place marker numbers must be in the range 1 to 99.
*/
impl /*struct*/ QString {
  pub fn arg_18<RetType, T: QString_arg_18<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.arg_18(self);
    // return 1;
  }
}
pub trait QString_arg_18<RetType> {
  fn arg_18(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_arg_18<usize> for (usize,usize,usize,usize,usize,usize) {
  fn arg_18(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
    let arg4 = (&self.4/*.qclsinst*/) as *const usize as usize;
    let arg5 = (&self.5/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString3argERKS_S1_S1_S1_S1_S1_", 6,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,arg5,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:307
// index:19
// Public Visibility=Default Availability=Available
// [8] QString arg(const QString &, const QString &, const QString &, const QString &, const QString &, const QString &, const QString &) const

/*
Returns a copy of this string with the lowest numbered place marker replaced by string a, i.e., %1, %2, ..., %99.

fieldWidth specifies the minimum amount of space that argument a shall occupy. If a requires less space than fieldWidth, it is padded to fieldWidth with character fillChar. A positive fieldWidth produces right-aligned text. A negative fieldWidth produces left-aligned text.

This example shows how we might create a status string for reporting progress while processing a list of files:


  QString i;           // current file's number
  QString total;       // number of files to process
  QString fileName;    // current file's name

  QString status = QString("Processing file %1 of %2: %3")
                  .arg(i).arg(total).arg(fileName);



First, arg(i) replaces %1. Then arg(total) replaces %2. Finally, arg(fileName) replaces %3.

One advantage of using arg() over asprintf() is that the order of the numbered place markers can change, if the application's strings are translated into other languages, but each arg() will still replace the lowest numbered unreplaced place marker, no matter where it appears. Also, if place marker %i appears more than once in the string, the arg() replaces all of them.

If there is no unreplaced place marker remaining, a warning message is output and the result is undefined. Place marker numbers must be in the range 1 to 99.
*/
impl /*struct*/ QString {
  pub fn arg_19<RetType, T: QString_arg_19<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.arg_19(self);
    // return 1;
  }
}
pub trait QString_arg_19<RetType> {
  fn arg_19(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_arg_19<usize> for (usize,usize,usize,usize,usize,usize,usize) {
  fn arg_19(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
    let arg4 = (&self.4/*.qclsinst*/) as *const usize as usize;
    let arg5 = (&self.5/*.qclsinst*/) as *const usize as usize;
    let arg6 = (&self.6/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString3argERKS_S1_S1_S1_S1_S1_S1_", 7,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,arg5,arg6,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:310
// index:20
// Public Visibility=Default Availability=Available
// [8] QString arg(const QString &, const QString &, const QString &, const QString &, const QString &, const QString &, const QString &, const QString &) const

/*
Returns a copy of this string with the lowest numbered place marker replaced by string a, i.e., %1, %2, ..., %99.

fieldWidth specifies the minimum amount of space that argument a shall occupy. If a requires less space than fieldWidth, it is padded to fieldWidth with character fillChar. A positive fieldWidth produces right-aligned text. A negative fieldWidth produces left-aligned text.

This example shows how we might create a status string for reporting progress while processing a list of files:


  QString i;           // current file's number
  QString total;       // number of files to process
  QString fileName;    // current file's name

  QString status = QString("Processing file %1 of %2: %3")
                  .arg(i).arg(total).arg(fileName);



First, arg(i) replaces %1. Then arg(total) replaces %2. Finally, arg(fileName) replaces %3.

One advantage of using arg() over asprintf() is that the order of the numbered place markers can change, if the application's strings are translated into other languages, but each arg() will still replace the lowest numbered unreplaced place marker, no matter where it appears. Also, if place marker %i appears more than once in the string, the arg() replaces all of them.

If there is no unreplaced place marker remaining, a warning message is output and the result is undefined. Place marker numbers must be in the range 1 to 99.
*/
impl /*struct*/ QString {
  pub fn arg_20<RetType, T: QString_arg_20<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.arg_20(self);
    // return 1;
  }
}
pub trait QString_arg_20<RetType> {
  fn arg_20(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_arg_20<usize> for (usize,usize,usize,usize,usize,usize,usize,usize) {
  fn arg_20(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
    let arg4 = (&self.4/*.qclsinst*/) as *const usize as usize;
    let arg5 = (&self.5/*.qclsinst*/) as *const usize as usize;
    let arg6 = (&self.6/*.qclsinst*/) as *const usize as usize;
    let arg7 = (&self.7/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString3argERKS_S1_S1_S1_S1_S1_S1_S1_", 8,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,arg5,arg6,arg7,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:313
// index:21
// Public Visibility=Default Availability=Available
// [8] QString arg(const QString &, const QString &, const QString &, const QString &, const QString &, const QString &, const QString &, const QString &, const QString &) const

/*
Returns a copy of this string with the lowest numbered place marker replaced by string a, i.e., %1, %2, ..., %99.

fieldWidth specifies the minimum amount of space that argument a shall occupy. If a requires less space than fieldWidth, it is padded to fieldWidth with character fillChar. A positive fieldWidth produces right-aligned text. A negative fieldWidth produces left-aligned text.

This example shows how we might create a status string for reporting progress while processing a list of files:


  QString i;           // current file's number
  QString total;       // number of files to process
  QString fileName;    // current file's name

  QString status = QString("Processing file %1 of %2: %3")
                  .arg(i).arg(total).arg(fileName);



First, arg(i) replaces %1. Then arg(total) replaces %2. Finally, arg(fileName) replaces %3.

One advantage of using arg() over asprintf() is that the order of the numbered place markers can change, if the application's strings are translated into other languages, but each arg() will still replace the lowest numbered unreplaced place marker, no matter where it appears. Also, if place marker %i appears more than once in the string, the arg() replaces all of them.

If there is no unreplaced place marker remaining, a warning message is output and the result is undefined. Place marker numbers must be in the range 1 to 99.
*/
impl /*struct*/ QString {
  pub fn arg_21<RetType, T: QString_arg_21<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.arg_21(self);
    // return 1;
  }
}
pub trait QString_arg_21<RetType> {
  fn arg_21(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_arg_21<usize> for (usize,usize,usize,usize,usize,usize,usize,usize,usize) {
  fn arg_21(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
    let arg4 = (&self.4/*.qclsinst*/) as *const usize as usize;
    let arg5 = (&self.5/*.qclsinst*/) as *const usize as usize;
    let arg6 = (&self.6/*.qclsinst*/) as *const usize as usize;
    let arg7 = (&self.7/*.qclsinst*/) as *const usize as usize;
    let arg8 = (&self.8/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString3argERKS_S1_S1_S1_S1_S1_S1_S1_S1_", 9,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,arg5,arg6,arg7,arg8,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:322
// index:0
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
impl /*struct*/ QString {
  pub fn indexOf_0<RetType, T: QString_indexOf_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.indexOf_0(self);
    // return 1;
  }
}
pub trait QString_indexOf_0<RetType> {
  fn indexOf_0(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_indexOf_0<i32> for (usize,i32,i32) {
  fn indexOf_0(self , rsthis: & QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString7indexOfE5QChariN2Qt15CaseSensitivityE", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:323
// index:1
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
impl /*struct*/ QString {
  pub fn indexOf_1<RetType, T: QString_indexOf_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.indexOf_1(self);
    // return 1;
  }
}
pub trait QString_indexOf_1<RetType> {
  fn indexOf_1(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_indexOf_1<i32> for (usize,i32,i32) {
  fn indexOf_1(self , rsthis: & QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString7indexOfERKS_iN2Qt15CaseSensitivityE", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:324
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
impl /*struct*/ QString {
  pub fn indexOf_2<RetType, T: QString_indexOf_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.indexOf_2(self);
    // return 1;
  }
}
pub trait QString_indexOf_2<RetType> {
  fn indexOf_2(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_indexOf_2<i32> for (usize,i32,i32) {
  fn indexOf_2(self , rsthis: & QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString7indexOfE13QLatin1StringiN2Qt15CaseSensitivityE", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:325
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
impl /*struct*/ QString {
  pub fn indexOf_3<RetType, T: QString_indexOf_3<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.indexOf_3(self);
    // return 1;
  }
}
pub trait QString_indexOf_3<RetType> {
  fn indexOf_3(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_indexOf_3<i32> for (usize,i32,i32) {
  fn indexOf_3(self , rsthis: & QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString7indexOfERK10QStringRefiN2Qt15CaseSensitivityE", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:340
// index:4
// Public Visibility=Default Availability=Available
// [4] int indexOf(const QRegExp &, int) const

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
impl /*struct*/ QString {
  pub fn indexOf_4<RetType, T: QString_indexOf_4<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.indexOf_4(self);
    // return 1;
  }
}
pub trait QString_indexOf_4<RetType> {
  fn indexOf_4(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_indexOf_4<i32> for (usize,i32) {
  fn indexOf_4(self , rsthis: & QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString7indexOfERK7QRegExpi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:345
// index:5
// Public Visibility=Default Availability=Available
// [4] int indexOf(QRegExp &, int) const

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
impl /*struct*/ QString {
  pub fn indexOf_5<RetType, T: QString_indexOf_5<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.indexOf_5(self);
    // return 1;
  }
}
pub trait QString_indexOf_5<RetType> {
  fn indexOf_5(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_indexOf_5<i32> for (usize,i32) {
  fn indexOf_5(self , rsthis: & QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString7indexOfER7QRegExpi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:351
// index:6
// Public Visibility=Default Availability=Available
// [4] int indexOf(const QRegularExpression &, int) const

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
impl /*struct*/ QString {
  pub fn indexOf_6<RetType, T: QString_indexOf_6<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.indexOf_6(self);
    // return 1;
  }
}
pub trait QString_indexOf_6<RetType> {
  fn indexOf_6(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_indexOf_6<i32> for (usize,i32) {
  fn indexOf_6(self , rsthis: & QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString7indexOfERK18QRegularExpressioni", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:352
// index:7
// Public Visibility=Default Availability=Available
// [4] int indexOf(const QRegularExpression &, int, QRegularExpressionMatch *) const

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
impl /*struct*/ QString {
  pub fn indexOf_7<RetType, T: QString_indexOf_7<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.indexOf_7(self);
    // return 1;
  }
}
pub trait QString_indexOf_7<RetType> {
  fn indexOf_7(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_indexOf_7<i32> for (usize,i32,usize) {
  fn indexOf_7(self , rsthis: & QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString7indexOfERK18QRegularExpressioniP23QRegularExpressionMatch", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:326
// index:0
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
impl /*struct*/ QString {
  pub fn lastIndexOf_0<RetType, T: QString_lastIndexOf_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.lastIndexOf_0(self);
    // return 1;
  }
}
pub trait QString_lastIndexOf_0<RetType> {
  fn lastIndexOf_0(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_lastIndexOf_0<i32> for (usize,i32,i32) {
  fn lastIndexOf_0(self , rsthis: & QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString11lastIndexOfE5QChariN2Qt15CaseSensitivityE", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:327
// index:1
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
impl /*struct*/ QString {
  pub fn lastIndexOf_1<RetType, T: QString_lastIndexOf_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.lastIndexOf_1(self);
    // return 1;
  }
}
pub trait QString_lastIndexOf_1<RetType> {
  fn lastIndexOf_1(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_lastIndexOf_1<i32> for (usize,i32,i32) {
  fn lastIndexOf_1(self , rsthis: & QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString11lastIndexOfERKS_iN2Qt15CaseSensitivityE", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:328
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
impl /*struct*/ QString {
  pub fn lastIndexOf_2<RetType, T: QString_lastIndexOf_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.lastIndexOf_2(self);
    // return 1;
  }
}
pub trait QString_lastIndexOf_2<RetType> {
  fn lastIndexOf_2(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_lastIndexOf_2<i32> for (usize,i32,i32) {
  fn lastIndexOf_2(self , rsthis: & QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString11lastIndexOfE13QLatin1StringiN2Qt15CaseSensitivityE", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:329
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
impl /*struct*/ QString {
  pub fn lastIndexOf_3<RetType, T: QString_lastIndexOf_3<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.lastIndexOf_3(self);
    // return 1;
  }
}
pub trait QString_lastIndexOf_3<RetType> {
  fn lastIndexOf_3(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_lastIndexOf_3<i32> for (usize,i32,i32) {
  fn lastIndexOf_3(self , rsthis: & QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString11lastIndexOfERK10QStringRefiN2Qt15CaseSensitivityE", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:341
// index:4
// Public Visibility=Default Availability=Available
// [4] int lastIndexOf(const QRegExp &, int) const

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
impl /*struct*/ QString {
  pub fn lastIndexOf_4<RetType, T: QString_lastIndexOf_4<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.lastIndexOf_4(self);
    // return 1;
  }
}
pub trait QString_lastIndexOf_4<RetType> {
  fn lastIndexOf_4(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_lastIndexOf_4<i32> for (usize,i32) {
  fn lastIndexOf_4(self , rsthis: & QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString11lastIndexOfERK7QRegExpi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:346
// index:5
// Public Visibility=Default Availability=Available
// [4] int lastIndexOf(QRegExp &, int) const

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
impl /*struct*/ QString {
  pub fn lastIndexOf_5<RetType, T: QString_lastIndexOf_5<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.lastIndexOf_5(self);
    // return 1;
  }
}
pub trait QString_lastIndexOf_5<RetType> {
  fn lastIndexOf_5(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_lastIndexOf_5<i32> for (usize,i32) {
  fn lastIndexOf_5(self , rsthis: & QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString11lastIndexOfER7QRegExpi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:353
// index:6
// Public Visibility=Default Availability=Available
// [4] int lastIndexOf(const QRegularExpression &, int) const

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
impl /*struct*/ QString {
  pub fn lastIndexOf_6<RetType, T: QString_lastIndexOf_6<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.lastIndexOf_6(self);
    // return 1;
  }
}
pub trait QString_lastIndexOf_6<RetType> {
  fn lastIndexOf_6(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_lastIndexOf_6<i32> for (usize,i32) {
  fn lastIndexOf_6(self , rsthis: & QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString11lastIndexOfERK18QRegularExpressioni", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:354
// index:7
// Public Visibility=Default Availability=Available
// [4] int lastIndexOf(const QRegularExpression &, int, QRegularExpressionMatch *) const

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
impl /*struct*/ QString {
  pub fn lastIndexOf_7<RetType, T: QString_lastIndexOf_7<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.lastIndexOf_7(self);
    // return 1;
  }
}
pub trait QString_lastIndexOf_7<RetType> {
  fn lastIndexOf_7(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_lastIndexOf_7<i32> for (usize,i32,usize) {
  fn lastIndexOf_7(self , rsthis: & QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString11lastIndexOfERK18QRegularExpressioniP23QRegularExpressionMatch", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:331
// index:0
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
impl /*struct*/ QString {
  pub fn contains_0<RetType, T: QString_contains_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.contains_0(self);
    // return 1;
  }
}
pub trait QString_contains_0<RetType> {
  fn contains_0(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_contains_0<bool> for (usize,i32) {
  fn contains_0(self , rsthis: & QString) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString8containsE5QCharN2Qt15CaseSensitivityE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:332
// index:1
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
impl /*struct*/ QString {
  pub fn contains_1<RetType, T: QString_contains_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.contains_1(self);
    // return 1;
  }
}
pub trait QString_contains_1<RetType> {
  fn contains_1(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_contains_1<bool> for (usize,i32) {
  fn contains_1(self , rsthis: & QString) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString8containsERKS_N2Qt15CaseSensitivityE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:333
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
impl /*struct*/ QString {
  pub fn contains_2<RetType, T: QString_contains_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.contains_2(self);
    // return 1;
  }
}
pub trait QString_contains_2<RetType> {
  fn contains_2(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_contains_2<bool> for (usize,i32) {
  fn contains_2(self , rsthis: & QString) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString8containsE13QLatin1StringN2Qt15CaseSensitivityE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:334
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
impl /*struct*/ QString {
  pub fn contains_3<RetType, T: QString_contains_3<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.contains_3(self);
    // return 1;
  }
}
pub trait QString_contains_3<RetType> {
  fn contains_3(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_contains_3<bool> for (usize,i32) {
  fn contains_3(self , rsthis: & QString) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString8containsERK10QStringRefN2Qt15CaseSensitivityE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:342
// index:4
// Public inline Visibility=Default Availability=Available
// [1] bool contains(const QRegExp &) const

/*
Returns true if this string contains an occurrence of the string str; otherwise returns false.

If cs is Qt::CaseSensitive (default), the search is case sensitive; otherwise the search is case insensitive.

Example:


  QString str = "Peter Pan";
  str.contains("peter", Qt::CaseInsensitive);    // returns true



See also indexOf() and count().
*/
impl /*struct*/ QString {
  pub fn contains_4<RetType, T: QString_contains_4<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.contains_4(self);
    // return 1;
  }
}
pub trait QString_contains_4<RetType> {
  fn contains_4(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_contains_4<bool> for (usize) {
  fn contains_4(self , rsthis: & QString) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString8containsERK7QRegExp", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:347
// index:5
// Public inline Visibility=Default Availability=Available
// [1] bool contains(QRegExp &) const

/*
Returns true if this string contains an occurrence of the string str; otherwise returns false.

If cs is Qt::CaseSensitive (default), the search is case sensitive; otherwise the search is case insensitive.

Example:


  QString str = "Peter Pan";
  str.contains("peter", Qt::CaseInsensitive);    // returns true



See also indexOf() and count().
*/
impl /*struct*/ QString {
  pub fn contains_5<RetType, T: QString_contains_5<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.contains_5(self);
    // return 1;
  }
}
pub trait QString_contains_5<RetType> {
  fn contains_5(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_contains_5<bool> for (usize) {
  fn contains_5(self , rsthis: & QString) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString8containsER7QRegExp", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:355
// index:6
// Public Visibility=Default Availability=Available
// [1] bool contains(const QRegularExpression &) const

/*
Returns true if this string contains an occurrence of the string str; otherwise returns false.

If cs is Qt::CaseSensitive (default), the search is case sensitive; otherwise the search is case insensitive.

Example:


  QString str = "Peter Pan";
  str.contains("peter", Qt::CaseInsensitive);    // returns true



See also indexOf() and count().
*/
impl /*struct*/ QString {
  pub fn contains_6<RetType, T: QString_contains_6<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.contains_6(self);
    // return 1;
  }
}
pub trait QString_contains_6<RetType> {
  fn contains_6(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_contains_6<bool> for (usize) {
  fn contains_6(self , rsthis: & QString) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString8containsERK18QRegularExpression", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:356
// index:7
// Public Visibility=Default Availability=Available
// [1] bool contains(const QRegularExpression &, QRegularExpressionMatch *) const

/*
Returns true if this string contains an occurrence of the string str; otherwise returns false.

If cs is Qt::CaseSensitive (default), the search is case sensitive; otherwise the search is case insensitive.

Example:


  QString str = "Peter Pan";
  str.contains("peter", Qt::CaseInsensitive);    // returns true



See also indexOf() and count().
*/
impl /*struct*/ QString {
  pub fn contains_7<RetType, T: QString_contains_7<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.contains_7(self);
    // return 1;
  }
}
pub trait QString_contains_7<RetType> {
  fn contains_7(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_contains_7<bool> for (usize,usize) {
  fn contains_7(self , rsthis: & QString) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString8containsERK18QRegularExpressionP23QRegularExpressionMatch", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:369
// index:0
// Public Visibility=Default Availability=Available
// [8] QString section(QChar, int, int, QString::SectionFlags) const

/*
This function returns a section of the string.

This string is treated as a sequence of fields separated by the character, sep. The returned string consists of the fields from position start to position end inclusive. If end is not specified, all fields from position start to the end of the string are included. Fields are numbered 0, 1, 2, etc., counting from the left, and -1, -2, etc., counting from right to left.

The flags argument can be used to affect some aspects of the function's behavior, e.g. whether to be case sensitive, whether to skip empty fields and how to deal with leading and trailing separators; see SectionFlags.


  QString str;
  QString csv = "forename,middlename,surname,phone";
  QString path = "/usr/local/bin/myapp"; // First field is empty
  QString::SectionFlag flag = QString::SectionSkipEmpty;

  str = csv.section(',', 2, 2);   // str == "surname"
  str = path.section('/', 3, 4);  // str == "bin/myapp"
  str = path.section('/', 3, 3, flag); // str == "myapp"



If start or end is negative, we count fields from the right of the string, the right-most field being -1, the one from right-most field being -2, and so on.


  str = csv.section(',', -3, -2);  // str == "middlename,surname"
  str = path.section('/', -1); // str == "myapp"



See also split().
*/
impl /*struct*/ QString {
  pub fn section_0<RetType, T: QString_section_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.section_0(self);
    // return 1;
  }
}
pub trait QString_section_0<RetType> {
  fn section_0(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_section_0<usize> for (usize,i32,i32,i32) {
  fn section_0(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString7sectionE5QCharii6QFlagsINS_11SectionFlagEE", 4,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:370
// index:1
// Public Visibility=Default Availability=Available
// [8] QString section(const QString &, int, int, QString::SectionFlags) const

/*
This function returns a section of the string.

This string is treated as a sequence of fields separated by the character, sep. The returned string consists of the fields from position start to position end inclusive. If end is not specified, all fields from position start to the end of the string are included. Fields are numbered 0, 1, 2, etc., counting from the left, and -1, -2, etc., counting from right to left.

The flags argument can be used to affect some aspects of the function's behavior, e.g. whether to be case sensitive, whether to skip empty fields and how to deal with leading and trailing separators; see SectionFlags.


  QString str;
  QString csv = "forename,middlename,surname,phone";
  QString path = "/usr/local/bin/myapp"; // First field is empty
  QString::SectionFlag flag = QString::SectionSkipEmpty;

  str = csv.section(',', 2, 2);   // str == "surname"
  str = path.section('/', 3, 4);  // str == "bin/myapp"
  str = path.section('/', 3, 3, flag); // str == "myapp"



If start or end is negative, we count fields from the right of the string, the right-most field being -1, the one from right-most field being -2, and so on.


  str = csv.section(',', -3, -2);  // str == "middlename,surname"
  str = path.section('/', -1); // str == "myapp"



See also split().
*/
impl /*struct*/ QString {
  pub fn section_1<RetType, T: QString_section_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.section_1(self);
    // return 1;
  }
}
pub trait QString_section_1<RetType> {
  fn section_1(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_section_1<usize> for (usize,i32,i32,i32) {
  fn section_1(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString7sectionERKS_ii6QFlagsINS_11SectionFlagEE", 4,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:372
// index:2
// Public Visibility=Default Availability=Available
// [8] QString section(const QRegExp &, int, int, QString::SectionFlags) const

/*
This function returns a section of the string.

This string is treated as a sequence of fields separated by the character, sep. The returned string consists of the fields from position start to position end inclusive. If end is not specified, all fields from position start to the end of the string are included. Fields are numbered 0, 1, 2, etc., counting from the left, and -1, -2, etc., counting from right to left.

The flags argument can be used to affect some aspects of the function's behavior, e.g. whether to be case sensitive, whether to skip empty fields and how to deal with leading and trailing separators; see SectionFlags.


  QString str;
  QString csv = "forename,middlename,surname,phone";
  QString path = "/usr/local/bin/myapp"; // First field is empty
  QString::SectionFlag flag = QString::SectionSkipEmpty;

  str = csv.section(',', 2, 2);   // str == "surname"
  str = path.section('/', 3, 4);  // str == "bin/myapp"
  str = path.section('/', 3, 3, flag); // str == "myapp"



If start or end is negative, we count fields from the right of the string, the right-most field being -1, the one from right-most field being -2, and so on.


  str = csv.section(',', -3, -2);  // str == "middlename,surname"
  str = path.section('/', -1); // str == "myapp"



See also split().
*/
impl /*struct*/ QString {
  pub fn section_2<RetType, T: QString_section_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.section_2(self);
    // return 1;
  }
}
pub trait QString_section_2<RetType> {
  fn section_2(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_section_2<usize> for (usize,i32,i32,i32) {
  fn section_2(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString7sectionERK7QRegExpii6QFlagsINS_11SectionFlagEE", 4,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:375
// index:3
// Public Visibility=Default Availability=Available
// [8] QString section(const QRegularExpression &, int, int, QString::SectionFlags) const

/*
This function returns a section of the string.

This string is treated as a sequence of fields separated by the character, sep. The returned string consists of the fields from position start to position end inclusive. If end is not specified, all fields from position start to the end of the string are included. Fields are numbered 0, 1, 2, etc., counting from the left, and -1, -2, etc., counting from right to left.

The flags argument can be used to affect some aspects of the function's behavior, e.g. whether to be case sensitive, whether to skip empty fields and how to deal with leading and trailing separators; see SectionFlags.


  QString str;
  QString csv = "forename,middlename,surname,phone";
  QString path = "/usr/local/bin/myapp"; // First field is empty
  QString::SectionFlag flag = QString::SectionSkipEmpty;

  str = csv.section(',', 2, 2);   // str == "surname"
  str = path.section('/', 3, 4);  // str == "bin/myapp"
  str = path.section('/', 3, 3, flag); // str == "myapp"



If start or end is negative, we count fields from the right of the string, the right-most field being -1, the one from right-most field being -2, and so on.


  str = csv.section(',', -3, -2);  // str == "middlename,surname"
  str = path.section('/', -1); // str == "myapp"



See also split().
*/
impl /*struct*/ QString {
  pub fn section_3<RetType, T: QString_section_3<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.section_3(self);
    // return 1;
  }
}
pub trait QString_section_3<RetType> {
  fn section_3(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_section_3<usize> for (usize,i32,i32,i32) {
  fn section_3(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString7sectionERK18QRegularExpressionii6QFlagsINS_11SectionFlagEE", 4,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:377
// index:0
// Public Visibility=Default Availability=Available
// [8] QString left(int) const

/*
Returns a substring that contains the n leftmost characters of the string.

The entire string is returned if n is greater than or equal to size(), or less than zero.


  QString x = "Pineapple";
  QString y = x.left(4);      // y == "Pine"



See also right(), mid(), startsWith(), chopped(), chop(), and truncate().
*/
impl /*struct*/ QString {
  pub fn left_0<RetType, T: QString_left_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.left_0(self);
    // return 1;
  }
}
pub trait QString_left_0<RetType> {
  fn left_0(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_left_0<usize> for (i32) {
  fn left_0(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString4leftEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:378
// index:0
// Public Visibility=Default Availability=Available
// [8] QString right(int) const

/*
Returns a substring that contains the n rightmost characters of the string.

The entire string is returned if n is greater than or equal to size(), or less than zero.


  QString x = "Pineapple";
  QString y = x.right(5);      // y == "apple"



See also left(), mid(), endsWith(), chopped(), chop(), and truncate().
*/
impl /*struct*/ QString {
  pub fn right_0<RetType, T: QString_right_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.right_0(self);
    // return 1;
  }
}
pub trait QString_right_0<RetType> {
  fn right_0(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_right_0<usize> for (i32) {
  fn right_0(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString5rightEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:379
// index:0
// Public Visibility=Default Availability=Available
// [8] QString mid(int, int) const

/*
Returns a string that contains n characters of this string, starting at the specified position index.

Returns a null string if the position index exceeds the length of the string. If there are less than n characters available in the string starting at the given position, or if n is -1 (default), the function returns all characters that are available from the specified position.

Example:


  QString x = "Nine pineapples";
  QString y = x.mid(5, 4);            // y == "pine"
  QString z = x.mid(5);               // z == "pineapples"



See also left(), right(), chopped(), chop(), and truncate().
*/
impl /*struct*/ QString {
  pub fn mid_0<RetType, T: QString_mid_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mid_0(self);
    // return 1;
  }
}
pub trait QString_mid_0<RetType> {
  fn mid_0(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_mid_0<usize> for (i32,i32) {
  fn mid_0(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString3midEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:380
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QString chopped(int) const

/*
Returns a substring that contains the size() - len leftmost characters of this string.

Note: The behavior is undefined if len is negative or greater than size().

This function was introduced in  Qt 5.10.

See also endsWith(), left(), right(), mid(), chop(), and truncate().
*/
impl /*struct*/ QString {
  pub fn chopped_0<RetType, T: QString_chopped_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.chopped_0(self);
    // return 1;
  }
}
pub trait QString_chopped_0<RetType> {
  fn chopped_0(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_chopped_0<usize> for (i32) {
  fn chopped_0(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString7choppedEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:384
// index:0
// Public Visibility=Default Availability=Available
// [16] QStringRef leftRef(int) const

/*
Returns a substring reference to the n leftmost characters of the string.

If n is greater than or equal to size(), or less than zero, a reference to the entire string is returned.


  QString x = "Pineapple";
  QStringRef y = x.leftRef(4);        // y == "Pine"



This function was introduced in  Qt 4.4.

See also left(), rightRef(), midRef(), and startsWith().
*/
impl /*struct*/ QString {
  pub fn leftRef_0<RetType, T: QString_leftRef_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.leftRef_0(self);
    // return 1;
  }
}
pub trait QString_leftRef_0<RetType> {
  fn leftRef_0(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_leftRef_0<usize> for (i32) {
  fn leftRef_0(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString7leftRefEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:385
// index:0
// Public Visibility=Default Availability=Available
// [16] QStringRef rightRef(int) const

/*
Returns a substring reference to the n rightmost characters of the string.

If n is greater than or equal to size(), or less than zero, a reference to the entire string is returned.


  QString x = "Pineapple";
  QStringRef y = x.rightRef(5);       // y == "apple"



This function was introduced in  Qt 4.4.

See also right(), leftRef(), midRef(), and endsWith().
*/
impl /*struct*/ QString {
  pub fn rightRef_0<RetType, T: QString_rightRef_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rightRef_0(self);
    // return 1;
  }
}
pub trait QString_rightRef_0<RetType> {
  fn rightRef_0(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_rightRef_0<usize> for (i32) {
  fn rightRef_0(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString8rightRefEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:386
// index:0
// Public Visibility=Default Availability=Available
// [16] QStringRef midRef(int, int) const

/*
Returns a substring reference to n characters of this string, starting at the specified position.

If the position exceeds the length of the string, a null reference is returned.

If there are less than n characters available in the string, starting at the given position, or if n is -1 (default), the function returns all characters from the specified position onwards.

Example:


  QString x = "Nine pineapples";
  QStringRef y = x.midRef(5, 4);      // y == "pine"
  QStringRef z = x.midRef(5);         // z == "pineapples"



This function was introduced in  Qt 4.4.

See also mid(), leftRef(), and rightRef().
*/
impl /*struct*/ QString {
  pub fn midRef_0<RetType, T: QString_midRef_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.midRef_0(self);
    // return 1;
  }
}
pub trait QString_midRef_0<RetType> {
  fn midRef_0(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_midRef_0<usize> for (i32,i32) {
  fn midRef_0(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString6midRefEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:389
// index:0
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
impl /*struct*/ QString {
  pub fn startsWith_0<RetType, T: QString_startsWith_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.startsWith_0(self);
    // return 1;
  }
}
pub trait QString_startsWith_0<RetType> {
  fn startsWith_0(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_startsWith_0<bool> for (usize,i32) {
  fn startsWith_0(self , rsthis: & QString) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString10startsWithERKS_N2Qt15CaseSensitivityE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:390
// index:1
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
impl /*struct*/ QString {
  pub fn startsWith_1<RetType, T: QString_startsWith_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.startsWith_1(self);
    // return 1;
  }
}
pub trait QString_startsWith_1<RetType> {
  fn startsWith_1(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_startsWith_1<bool> for (usize,i32) {
  fn startsWith_1(self , rsthis: & QString) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString10startsWithERK10QStringRefN2Qt15CaseSensitivityE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:392
// index:2
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
impl /*struct*/ QString {
  pub fn startsWith_2<RetType, T: QString_startsWith_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.startsWith_2(self);
    // return 1;
  }
}
pub trait QString_startsWith_2<RetType> {
  fn startsWith_2(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_startsWith_2<bool> for (usize,i32) {
  fn startsWith_2(self , rsthis: & QString) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString10startsWithE11QStringViewN2Qt15CaseSensitivityE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:394
// index:3
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
impl /*struct*/ QString {
  pub fn startsWith_3<RetType, T: QString_startsWith_3<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.startsWith_3(self);
    // return 1;
  }
}
pub trait QString_startsWith_3<RetType> {
  fn startsWith_3(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_startsWith_3<bool> for (usize,i32) {
  fn startsWith_3(self , rsthis: & QString) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString10startsWithE13QLatin1StringN2Qt15CaseSensitivityE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:395
// index:4
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
impl /*struct*/ QString {
  pub fn startsWith_4<RetType, T: QString_startsWith_4<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.startsWith_4(self);
    // return 1;
  }
}
pub trait QString_startsWith_4<RetType> {
  fn startsWith_4(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_startsWith_4<bool> for (usize,i32) {
  fn startsWith_4(self , rsthis: & QString) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString10startsWithE5QCharN2Qt15CaseSensitivityE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:398
// index:0
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
impl /*struct*/ QString {
  pub fn endsWith_0<RetType, T: QString_endsWith_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.endsWith_0(self);
    // return 1;
  }
}
pub trait QString_endsWith_0<RetType> {
  fn endsWith_0(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_endsWith_0<bool> for (usize,i32) {
  fn endsWith_0(self , rsthis: & QString) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString8endsWithERKS_N2Qt15CaseSensitivityE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:399
// index:1
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
impl /*struct*/ QString {
  pub fn endsWith_1<RetType, T: QString_endsWith_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.endsWith_1(self);
    // return 1;
  }
}
pub trait QString_endsWith_1<RetType> {
  fn endsWith_1(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_endsWith_1<bool> for (usize,i32) {
  fn endsWith_1(self , rsthis: & QString) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString8endsWithERK10QStringRefN2Qt15CaseSensitivityE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:401
// index:2
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
impl /*struct*/ QString {
  pub fn endsWith_2<RetType, T: QString_endsWith_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.endsWith_2(self);
    // return 1;
  }
}
pub trait QString_endsWith_2<RetType> {
  fn endsWith_2(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_endsWith_2<bool> for (usize,i32) {
  fn endsWith_2(self , rsthis: & QString) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString8endsWithE11QStringViewN2Qt15CaseSensitivityE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:403
// index:3
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
impl /*struct*/ QString {
  pub fn endsWith_3<RetType, T: QString_endsWith_3<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.endsWith_3(self);
    // return 1;
  }
}
pub trait QString_endsWith_3<RetType> {
  fn endsWith_3(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_endsWith_3<bool> for (usize,i32) {
  fn endsWith_3(self , rsthis: & QString) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString8endsWithE13QLatin1StringN2Qt15CaseSensitivityE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:404
// index:4
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
impl /*struct*/ QString {
  pub fn endsWith_4<RetType, T: QString_endsWith_4<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.endsWith_4(self);
    // return 1;
  }
}
pub trait QString_endsWith_4<RetType> {
  fn endsWith_4(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_endsWith_4<bool> for (usize,i32) {
  fn endsWith_4(self , rsthis: & QString) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString8endsWithE5QCharN2Qt15CaseSensitivityE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:406
// index:0
// Public Visibility=Default Availability=Available
// [8] QString leftJustified(int, QChar, bool) const

/*
Returns a string of size width that contains this string padded by the fill character.

If truncate is false and the size() of the string is more than width, then the returned string is a copy of the string.


  QString s = "apple";
  QString t = s.leftJustified(8, '.');    // t == "apple..."



If truncate is true and the size() of the string is more than width, then any characters in a copy of the string after position width are removed, and the copy is returned.


  QString str = "Pineapple";
  str = str.leftJustified(5, '.', true);    // str == "Pinea"



See also rightJustified().
*/
impl /*struct*/ QString {
  pub fn leftJustified_0<RetType, T: QString_leftJustified_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.leftJustified_0(self);
    // return 1;
  }
}
pub trait QString_leftJustified_0<RetType> {
  fn leftJustified_0(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_leftJustified_0<usize> for (i32,usize,bool) {
  fn leftJustified_0(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const bool as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString13leftJustifiedEi5QCharb", 3,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:407
// index:0
// Public Visibility=Default Availability=Available
// [8] QString rightJustified(int, QChar, bool) const

/*
Returns a string of size() width that contains the fill character followed by the string. For example:


  QString s = "apple";
  QString t = s.rightJustified(8, '.');    // t == "...apple"



If truncate is false and the size() of the string is more than width, then the returned string is a copy of the string.

If truncate is true and the size() of the string is more than width, then the resulting string is truncated at position width.


  QString str = "Pineapple";
  str = str.rightJustified(5, '.', true);    // str == "Pinea"



See also leftJustified().
*/
impl /*struct*/ QString {
  pub fn rightJustified_0<RetType, T: QString_rightJustified_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rightJustified_0(self);
    // return 1;
  }
}
pub trait QString_rightJustified_0<RetType> {
  fn rightJustified_0(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_rightJustified_0<usize> for (i32,usize,bool) {
  fn rightJustified_0(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const bool as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString14rightJustifiedEi5QCharb", 3,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:417
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QString toLower() const

/*
Returns a lowercase copy of the string.


  QString str = "The Qt PROJECT";
  str = str.toLower();        // str == "the qt project"



The case conversion will always happen in the 'C' locale. For locale dependent case folding use QLocale::toLower()

See also toUpper() and QLocale::toLower().
*/
impl /*struct*/ QString {
  pub fn toLower_0<RetType, T: QString_toLower_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toLower_0(self);
    // return 1;
  }
}
pub trait QString_toLower_0<RetType> {
  fn toLower_0(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_toLower_0<usize> for () {
  fn toLower_0(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNKR7QString7toLowerEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:419
// index:1
// Public inline Visibility=Default Availability=Available
// [8] QString toLower()

/*
Returns a lowercase copy of the string.


  QString str = "The Qt PROJECT";
  str = str.toLower();        // str == "the qt project"



The case conversion will always happen in the 'C' locale. For locale dependent case folding use QLocale::toLower()

See also toUpper() and QLocale::toLower().
*/
impl /*struct*/ QString {
  pub fn toLower_1<RetType, T: QString_toLower_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toLower_1(self);
    // return 1;
  }
}
pub trait QString_toLower_1<RetType> {
  fn toLower_1(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_toLower_1<usize> for () {
  fn toLower_1(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNO7QString7toLowerEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:421
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QString toUpper() const

/*
Returns an uppercase copy of the string.


  QString str = "TeXt";
  str = str.toUpper();        // str == "TEXT"



The case conversion will always happen in the 'C' locale. For locale dependent case folding use QLocale::toUpper()

See also toLower() and QLocale::toLower().
*/
impl /*struct*/ QString {
  pub fn toUpper_0<RetType, T: QString_toUpper_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toUpper_0(self);
    // return 1;
  }
}
pub trait QString_toUpper_0<RetType> {
  fn toUpper_0(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_toUpper_0<usize> for () {
  fn toUpper_0(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNKR7QString7toUpperEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:423
// index:1
// Public inline Visibility=Default Availability=Available
// [8] QString toUpper()

/*
Returns an uppercase copy of the string.


  QString str = "TeXt";
  str = str.toUpper();        // str == "TEXT"



The case conversion will always happen in the 'C' locale. For locale dependent case folding use QLocale::toUpper()

See also toLower() and QLocale::toLower().
*/
impl /*struct*/ QString {
  pub fn toUpper_1<RetType, T: QString_toUpper_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toUpper_1(self);
    // return 1;
  }
}
pub trait QString_toUpper_1<RetType> {
  fn toUpper_1(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_toUpper_1<usize> for () {
  fn toUpper_1(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNO7QString7toUpperEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:425
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QString toCaseFolded() const

/*
Returns the case folded equivalent of the string. For most Unicode characters this is the same as toLower().
*/
impl /*struct*/ QString {
  pub fn toCaseFolded_0<RetType, T: QString_toCaseFolded_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toCaseFolded_0(self);
    // return 1;
  }
}
pub trait QString_toCaseFolded_0<RetType> {
  fn toCaseFolded_0(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_toCaseFolded_0<usize> for () {
  fn toCaseFolded_0(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNKR7QString12toCaseFoldedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:427
// index:1
// Public inline Visibility=Default Availability=Available
// [8] QString toCaseFolded()

/*
Returns the case folded equivalent of the string. For most Unicode characters this is the same as toLower().
*/
impl /*struct*/ QString {
  pub fn toCaseFolded_1<RetType, T: QString_toCaseFolded_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toCaseFolded_1(self);
    // return 1;
  }
}
pub trait QString_toCaseFolded_1<RetType> {
  fn toCaseFolded_1(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_toCaseFolded_1<usize> for () {
  fn toCaseFolded_1(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNO7QString12toCaseFoldedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:429
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QString trimmed() const

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
impl /*struct*/ QString {
  pub fn trimmed_0<RetType, T: QString_trimmed_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.trimmed_0(self);
    // return 1;
  }
}
pub trait QString_trimmed_0<RetType> {
  fn trimmed_0(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_trimmed_0<usize> for () {
  fn trimmed_0(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNKR7QString7trimmedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:431
// index:1
// Public inline Visibility=Default Availability=Available
// [8] QString trimmed()

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
impl /*struct*/ QString {
  pub fn trimmed_1<RetType, T: QString_trimmed_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.trimmed_1(self);
    // return 1;
  }
}
pub trait QString_trimmed_1<RetType> {
  fn trimmed_1(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_trimmed_1<usize> for () {
  fn trimmed_1(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNO7QString7trimmedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:433
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QString simplified() const

/*
Returns a string that has whitespace removed from the start and the end, and that has each sequence of internal whitespace replaced with a single space.

Whitespace means any character for which QChar::isSpace() returns true. This includes the ASCII characters '\t', '\n', '\v', '\f', '\r', and ' '.

Example:


  QString str = "  lots\t of\nwhitespace\r\n ";
  str = str.simplified();
  // str == "lots of whitespace";



See also trimmed().
*/
impl /*struct*/ QString {
  pub fn simplified_0<RetType, T: QString_simplified_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.simplified_0(self);
    // return 1;
  }
}
pub trait QString_simplified_0<RetType> {
  fn simplified_0(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_simplified_0<usize> for () {
  fn simplified_0(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNKR7QString10simplifiedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:435
// index:1
// Public inline Visibility=Default Availability=Available
// [8] QString simplified()

/*
Returns a string that has whitespace removed from the start and the end, and that has each sequence of internal whitespace replaced with a single space.

Whitespace means any character for which QChar::isSpace() returns true. This includes the ASCII characters '\t', '\n', '\v', '\f', '\r', and ' '.

Example:


  QString str = "  lots\t of\nwhitespace\r\n ";
  str = str.simplified();
  // str == "lots of whitespace";



See also trimmed().
*/
impl /*struct*/ QString {
  pub fn simplified_1<RetType, T: QString_simplified_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.simplified_1(self);
    // return 1;
  }
}
pub trait QString_simplified_1<RetType> {
  fn simplified_1(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_simplified_1<usize> for () {
  fn simplified_1(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNO7QString10simplifiedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:447
// index:0
// Public Visibility=Default Availability=Available
// [8] QString toHtmlEscaped() const

/*
Converts a plain text string to an HTML string with HTML metacharacters <, >, &, and " replaced by HTML entities.

Example:


  QString plain = "#include <QtCore>"
  QString html = plain.toHtmlEscaped();
  // html == "#include &lt;QtCore&gt;"



This function was introduced in  Qt 5.0.
*/
impl /*struct*/ QString {
  pub fn toHtmlEscaped_0<RetType, T: QString_toHtmlEscaped_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toHtmlEscaped_0(self);
    // return 1;
  }
}
pub trait QString_toHtmlEscaped_0<RetType> {
  fn toHtmlEscaped_0(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_toHtmlEscaped_0<usize> for () {
  fn toHtmlEscaped_0(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString13toHtmlEscapedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:465
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QString & operator+=(QChar)

/*

*/
impl /*struct*/ QString {
  pub fn operator_add_equal_0<RetType, T: QString_operator_add_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_add_equal_0(self);
    // return 1;
  }
}
pub trait QString_operator_add_equal_0<RetType> {
  fn operator_add_equal_0(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_operator_add_equal_0<usize> for (usize) {
  fn operator_add_equal_0(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QStringpLE5QChar", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:473
// index:1
// Public inline Visibility=Default Availability=Available
// [8] QString & operator+=(QChar::SpecialCharacter)

/*

*/
impl /*struct*/ QString {
  pub fn operator_add_equal_1<RetType, T: QString_operator_add_equal_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_add_equal_1(self);
    // return 1;
  }
}
pub trait QString_operator_add_equal_1<RetType> {
  fn operator_add_equal_1(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_operator_add_equal_1<usize> for (i32) {
  fn operator_add_equal_1(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QStringpLEN5QChar16SpecialCharacterE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:474
// index:2
// Public inline Visibility=Default Availability=Available
// [8] QString & operator+=(const QString &)

/*

*/
impl /*struct*/ QString {
  pub fn operator_add_equal_2<RetType, T: QString_operator_add_equal_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_add_equal_2(self);
    // return 1;
  }
}
pub trait QString_operator_add_equal_2<RetType> {
  fn operator_add_equal_2(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_operator_add_equal_2<usize> for (usize) {
  fn operator_add_equal_2(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QStringpLERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:475
// index:3
// Public inline Visibility=Default Availability=Available
// [8] QString & operator+=(const QStringRef &)

/*

*/
impl /*struct*/ QString {
  pub fn operator_add_equal_3<RetType, T: QString_operator_add_equal_3<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_add_equal_3(self);
    // return 1;
  }
}
pub trait QString_operator_add_equal_3<RetType> {
  fn operator_add_equal_3(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_operator_add_equal_3<usize> for (usize) {
  fn operator_add_equal_3(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QStringpLERK10QStringRef", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:476
// index:4
// Public inline Visibility=Default Availability=Available
// [8] QString & operator+=(QLatin1String)

/*

*/
impl /*struct*/ QString {
  pub fn operator_add_equal_4<RetType, T: QString_operator_add_equal_4<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_add_equal_4(self);
    // return 1;
  }
}
pub trait QString_operator_add_equal_4<RetType> {
  fn operator_add_equal_4(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_operator_add_equal_4<usize> for (usize) {
  fn operator_add_equal_4(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QStringpLE13QLatin1String", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:708
// index:5
// Public inline Visibility=Default Availability=Available
// [8] QString & operator+=(const char *)

/*

*/
impl /*struct*/ QString {
  pub fn operator_add_equal_5<RetType, T: QString_operator_add_equal_5<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_add_equal_5(self);
    // return 1;
  }
}
pub trait QString_operator_add_equal_5<RetType> {
  fn operator_add_equal_5(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_operator_add_equal_5<usize> for (usize) {
  fn operator_add_equal_5(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QStringpLEPKc", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:710
// index:6
// Public inline Visibility=Default Availability=Available
// [8] QString & operator+=(const QByteArray &)

/*

*/
impl /*struct*/ QString {
  pub fn operator_add_equal_6<RetType, T: QString_operator_add_equal_6<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_add_equal_6(self);
    // return 1;
  }
}
pub trait QString_operator_add_equal_6<RetType> {
  fn operator_add_equal_6(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_operator_add_equal_6<usize> for (usize) {
  fn operator_add_equal_6(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QStringpLERK10QByteArray", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:712
// index:7
// Public inline Visibility=Default Availability=Available
// [8] QString & operator+=(char)

/*

*/
impl /*struct*/ QString {
  pub fn operator_add_equal_7<RetType, T: QString_operator_add_equal_7<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_add_equal_7(self);
    // return 1;
  }
}
pub trait QString_operator_add_equal_7<RetType> {
  fn operator_add_equal_7(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_operator_add_equal_7<usize> for (i8) {
  fn operator_add_equal_7(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i8 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QStringpLEc", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:478
// index:0
// Public Visibility=Default Availability=Available
// [8] QString & remove(int, int)

/*
Removes n characters from the string, starting at the given position index, and returns a reference to the string.

If the specified position index is within the string, but position + n is beyond the end of the string, the string is truncated at the specified position.


  QString s = "Montreal";
  s.remove(1, 4);
  // s == "Meal"



See also insert() and replace().
*/
impl /*struct*/ QString {
  pub fn remove_0<RetType, T: QString_remove_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.remove_0(self);
    // return 1;
  }
}
pub trait QString_remove_0<RetType> {
  fn remove_0(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_remove_0<usize> for (i32,i32) {
  fn remove_0(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QString6removeEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:479
// index:1
// Public Visibility=Default Availability=Available
// [8] QString & remove(QChar, Qt::CaseSensitivity)

/*
Removes n characters from the string, starting at the given position index, and returns a reference to the string.

If the specified position index is within the string, but position + n is beyond the end of the string, the string is truncated at the specified position.


  QString s = "Montreal";
  s.remove(1, 4);
  // s == "Meal"



See also insert() and replace().
*/
impl /*struct*/ QString {
  pub fn remove_1<RetType, T: QString_remove_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.remove_1(self);
    // return 1;
  }
}
pub trait QString_remove_1<RetType> {
  fn remove_1(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_remove_1<usize> for (usize,i32) {
  fn remove_1(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QString6removeE5QCharN2Qt15CaseSensitivityE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:480
// index:2
// Public Visibility=Default Availability=Available
// [8] QString & remove(const QString &, Qt::CaseSensitivity)

/*
Removes n characters from the string, starting at the given position index, and returns a reference to the string.

If the specified position index is within the string, but position + n is beyond the end of the string, the string is truncated at the specified position.


  QString s = "Montreal";
  s.remove(1, 4);
  // s == "Meal"



See also insert() and replace().
*/
impl /*struct*/ QString {
  pub fn remove_2<RetType, T: QString_remove_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.remove_2(self);
    // return 1;
  }
}
pub trait QString_remove_2<RetType> {
  fn remove_2(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_remove_2<usize> for (usize,i32) {
  fn remove_2(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QString6removeERKS_N2Qt15CaseSensitivityE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:495
// index:3
// Public inline Visibility=Default Availability=Available
// [8] QString & remove(const QRegExp &)

/*
Removes n characters from the string, starting at the given position index, and returns a reference to the string.

If the specified position index is within the string, but position + n is beyond the end of the string, the string is truncated at the specified position.


  QString s = "Montreal";
  s.remove(1, 4);
  // s == "Meal"



See also insert() and replace().
*/
impl /*struct*/ QString {
  pub fn remove_3<RetType, T: QString_remove_3<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.remove_3(self);
    // return 1;
  }
}
pub trait QString_remove_3<RetType> {
  fn remove_3(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_remove_3<usize> for (usize) {
  fn remove_3(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QString6removeERK7QRegExp", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:500
// index:4
// Public inline Visibility=Default Availability=Available
// [8] QString & remove(const QRegularExpression &)

/*
Removes n characters from the string, starting at the given position index, and returns a reference to the string.

If the specified position index is within the string, but position + n is beyond the end of the string, the string is truncated at the specified position.


  QString s = "Montreal";
  s.remove(1, 4);
  // s == "Meal"



See also insert() and replace().
*/
impl /*struct*/ QString {
  pub fn remove_4<RetType, T: QString_remove_4<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.remove_4(self);
    // return 1;
  }
}
pub trait QString_remove_4<RetType> {
  fn remove_4(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_remove_4<usize> for (usize) {
  fn remove_4(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QString6removeERK18QRegularExpression", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:481
// index:0
// Public Visibility=Default Availability=Available
// [8] QString & replace(int, int, QChar)

/*
Replaces n characters beginning at index position with the string after and returns a reference to this string.

Note: If the specified position index is within the string, but position + n goes outside the strings range, then n will be adjusted to stop at the end of the string.

Example:


  QString x = "Say yes!";
  QString y = "no";
  x.replace(4, 3, y);
  // x == "Say no!"



See also insert() and remove().
*/
impl /*struct*/ QString {
  pub fn replace_0<RetType, T: QString_replace_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.replace_0(self);
    // return 1;
  }
}
pub trait QString_replace_0<RetType> {
  fn replace_0(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_replace_0<usize> for (i32,i32,usize) {
  fn replace_0(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QString7replaceEii5QChar", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:482
// index:1
// Public Visibility=Default Availability=Available
// [8] QString & replace(int, int, const QChar *, int)

/*
Replaces n characters beginning at index position with the string after and returns a reference to this string.

Note: If the specified position index is within the string, but position + n goes outside the strings range, then n will be adjusted to stop at the end of the string.

Example:


  QString x = "Say yes!";
  QString y = "no";
  x.replace(4, 3, y);
  // x == "Say no!"



See also insert() and remove().
*/
impl /*struct*/ QString {
  pub fn replace_1<RetType, T: QString_replace_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.replace_1(self);
    // return 1;
  }
}
pub trait QString_replace_1<RetType> {
  fn replace_1(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_replace_1<usize> for (i32,i32,usize,i32) {
  fn replace_1(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QString7replaceEiiPK5QChari", 4,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:483
// index:2
// Public Visibility=Default Availability=Available
// [8] QString & replace(int, int, const QString &)

/*
Replaces n characters beginning at index position with the string after and returns a reference to this string.

Note: If the specified position index is within the string, but position + n goes outside the strings range, then n will be adjusted to stop at the end of the string.

Example:


  QString x = "Say yes!";
  QString y = "no";
  x.replace(4, 3, y);
  // x == "Say no!"



See also insert() and remove().
*/
impl /*struct*/ QString {
  pub fn replace_2<RetType, T: QString_replace_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.replace_2(self);
    // return 1;
  }
}
pub trait QString_replace_2<RetType> {
  fn replace_2(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_replace_2<usize> for (i32,i32,usize) {
  fn replace_2(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QString7replaceEiiRKS_", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:484
// index:3
// Public Visibility=Default Availability=Available
// [8] QString & replace(QChar, QChar, Qt::CaseSensitivity)

/*
Replaces n characters beginning at index position with the string after and returns a reference to this string.

Note: If the specified position index is within the string, but position + n goes outside the strings range, then n will be adjusted to stop at the end of the string.

Example:


  QString x = "Say yes!";
  QString y = "no";
  x.replace(4, 3, y);
  // x == "Say no!"



See also insert() and remove().
*/
impl /*struct*/ QString {
  pub fn replace_3<RetType, T: QString_replace_3<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.replace_3(self);
    // return 1;
  }
}
pub trait QString_replace_3<RetType> {
  fn replace_3(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_replace_3<usize> for (usize,usize,i32) {
  fn replace_3(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QString7replaceE5QCharS0_N2Qt15CaseSensitivityE", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:485
// index:4
// Public Visibility=Default Availability=Available
// [8] QString & replace(const QChar *, int, const QChar *, int, Qt::CaseSensitivity)

/*
Replaces n characters beginning at index position with the string after and returns a reference to this string.

Note: If the specified position index is within the string, but position + n goes outside the strings range, then n will be adjusted to stop at the end of the string.

Example:


  QString x = "Say yes!";
  QString y = "no";
  x.replace(4, 3, y);
  // x == "Say no!"



See also insert() and remove().
*/
impl /*struct*/ QString {
  pub fn replace_4<RetType, T: QString_replace_4<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.replace_4(self);
    // return 1;
  }
}
pub trait QString_replace_4<RetType> {
  fn replace_4(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_replace_4<usize> for (usize,i32,usize,i32,i32) {
  fn replace_4(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let arg4 = (&self.4) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QString7replaceEPK5QChariS2_iN2Qt15CaseSensitivityE", 5,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:486
// index:5
// Public Visibility=Default Availability=Available
// [8] QString & replace(QLatin1String, QLatin1String, Qt::CaseSensitivity)

/*
Replaces n characters beginning at index position with the string after and returns a reference to this string.

Note: If the specified position index is within the string, but position + n goes outside the strings range, then n will be adjusted to stop at the end of the string.

Example:


  QString x = "Say yes!";
  QString y = "no";
  x.replace(4, 3, y);
  // x == "Say no!"



See also insert() and remove().
*/
impl /*struct*/ QString {
  pub fn replace_5<RetType, T: QString_replace_5<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.replace_5(self);
    // return 1;
  }
}
pub trait QString_replace_5<RetType> {
  fn replace_5(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_replace_5<usize> for (usize,usize,i32) {
  fn replace_5(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QString7replaceE13QLatin1StringS0_N2Qt15CaseSensitivityE", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:487
// index:6
// Public Visibility=Default Availability=Available
// [8] QString & replace(QLatin1String, const QString &, Qt::CaseSensitivity)

/*
Replaces n characters beginning at index position with the string after and returns a reference to this string.

Note: If the specified position index is within the string, but position + n goes outside the strings range, then n will be adjusted to stop at the end of the string.

Example:


  QString x = "Say yes!";
  QString y = "no";
  x.replace(4, 3, y);
  // x == "Say no!"



See also insert() and remove().
*/
impl /*struct*/ QString {
  pub fn replace_6<RetType, T: QString_replace_6<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.replace_6(self);
    // return 1;
  }
}
pub trait QString_replace_6<RetType> {
  fn replace_6(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_replace_6<usize> for (usize,usize,i32) {
  fn replace_6(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QString7replaceE13QLatin1StringRKS_N2Qt15CaseSensitivityE", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:488
// index:7
// Public Visibility=Default Availability=Available
// [8] QString & replace(const QString &, QLatin1String, Qt::CaseSensitivity)

/*
Replaces n characters beginning at index position with the string after and returns a reference to this string.

Note: If the specified position index is within the string, but position + n goes outside the strings range, then n will be adjusted to stop at the end of the string.

Example:


  QString x = "Say yes!";
  QString y = "no";
  x.replace(4, 3, y);
  // x == "Say no!"



See also insert() and remove().
*/
impl /*struct*/ QString {
  pub fn replace_7<RetType, T: QString_replace_7<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.replace_7(self);
    // return 1;
  }
}
pub trait QString_replace_7<RetType> {
  fn replace_7(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_replace_7<usize> for (usize,usize,i32) {
  fn replace_7(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QString7replaceERKS_13QLatin1StringN2Qt15CaseSensitivityE", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:489
// index:8
// Public Visibility=Default Availability=Available
// [8] QString & replace(const QString &, const QString &, Qt::CaseSensitivity)

/*
Replaces n characters beginning at index position with the string after and returns a reference to this string.

Note: If the specified position index is within the string, but position + n goes outside the strings range, then n will be adjusted to stop at the end of the string.

Example:


  QString x = "Say yes!";
  QString y = "no";
  x.replace(4, 3, y);
  // x == "Say no!"



See also insert() and remove().
*/
impl /*struct*/ QString {
  pub fn replace_8<RetType, T: QString_replace_8<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.replace_8(self);
    // return 1;
  }
}
pub trait QString_replace_8<RetType> {
  fn replace_8(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_replace_8<usize> for (usize,usize,i32) {
  fn replace_8(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QString7replaceERKS_S1_N2Qt15CaseSensitivityE", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:491
// index:9
// Public Visibility=Default Availability=Available
// [8] QString & replace(QChar, const QString &, Qt::CaseSensitivity)

/*
Replaces n characters beginning at index position with the string after and returns a reference to this string.

Note: If the specified position index is within the string, but position + n goes outside the strings range, then n will be adjusted to stop at the end of the string.

Example:


  QString x = "Say yes!";
  QString y = "no";
  x.replace(4, 3, y);
  // x == "Say no!"



See also insert() and remove().
*/
impl /*struct*/ QString {
  pub fn replace_9<RetType, T: QString_replace_9<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.replace_9(self);
    // return 1;
  }
}
pub trait QString_replace_9<RetType> {
  fn replace_9(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_replace_9<usize> for (usize,usize,i32) {
  fn replace_9(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QString7replaceE5QCharRKS_N2Qt15CaseSensitivityE", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:492
// index:10
// Public Visibility=Default Availability=Available
// [8] QString & replace(QChar, QLatin1String, Qt::CaseSensitivity)

/*
Replaces n characters beginning at index position with the string after and returns a reference to this string.

Note: If the specified position index is within the string, but position + n goes outside the strings range, then n will be adjusted to stop at the end of the string.

Example:


  QString x = "Say yes!";
  QString y = "no";
  x.replace(4, 3, y);
  // x == "Say no!"



See also insert() and remove().
*/
impl /*struct*/ QString {
  pub fn replace_10<RetType, T: QString_replace_10<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.replace_10(self);
    // return 1;
  }
}
pub trait QString_replace_10<RetType> {
  fn replace_10(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_replace_10<usize> for (usize,usize,i32) {
  fn replace_10(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QString7replaceE5QChar13QLatin1StringN2Qt15CaseSensitivityE", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:494
// index:11
// Public Visibility=Default Availability=Available
// [8] QString & replace(const QRegExp &, const QString &)

/*
Replaces n characters beginning at index position with the string after and returns a reference to this string.

Note: If the specified position index is within the string, but position + n goes outside the strings range, then n will be adjusted to stop at the end of the string.

Example:


  QString x = "Say yes!";
  QString y = "no";
  x.replace(4, 3, y);
  // x == "Say no!"



See also insert() and remove().
*/
impl /*struct*/ QString {
  pub fn replace_11<RetType, T: QString_replace_11<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.replace_11(self);
    // return 1;
  }
}
pub trait QString_replace_11<RetType> {
  fn replace_11(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_replace_11<usize> for (usize,usize) {
  fn replace_11(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QString7replaceERK7QRegExpRKS_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:499
// index:12
// Public Visibility=Default Availability=Available
// [8] QString & replace(const QRegularExpression &, const QString &)

/*
Replaces n characters beginning at index position with the string after and returns a reference to this string.

Note: If the specified position index is within the string, but position + n goes outside the strings range, then n will be adjusted to stop at the end of the string.

Example:


  QString x = "Say yes!";
  QString y = "no";
  x.replace(4, 3, y);
  // x == "Say no!"



See also insert() and remove().
*/
impl /*struct*/ QString {
  pub fn replace_12<RetType, T: QString_replace_12<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.replace_12(self);
    // return 1;
  }
}
pub trait QString_replace_12<RetType> {
  fn replace_12(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_replace_12<usize> for (usize,usize) {
  fn replace_12(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QString7replaceERK18QRegularExpressionRKS_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:506
// index:0
// Public Visibility=Default Availability=Available
// [8] QStringList split(const QString &, QString::SplitBehavior, Qt::CaseSensitivity) const

/*
Splits the string into substrings wherever sep occurs, and returns the list of those strings. If sep does not match anywhere in the string, split() returns a single-element list containing this string.

cs specifies whether sep should be matched case sensitively or case insensitively.

If behavior is QString::SkipEmptyParts, empty entries don't appear in the result. By default, empty entries are kept.

Example:


  QString str = "a,,b,c";

  QStringList list1 = str.split(',');
  // list1: [ "a", "", "b", "c" ]

  QStringList list2 = str.split(',', QString::SkipEmptyParts);
  // list2: [ "a", "b", "c" ]



If sep is empty, split() returns an empty string, followed by each of the string's characters, followed by another empty string:


  QString str = "abc";
  auto parts = str.split("");
  // parts: {"", "a", "b", "c", ""}



To understand this behavior, recall that the empty string matches everywhere, so the above is qualitatively the same as:


  QString str = "/a/b/c/";
  auto parts = str.split('/');
  // parts: {"", "a", "b", "c", ""}



See also QStringList::join() and section().
*/
impl /*struct*/ QString {
  pub fn split_0<RetType, T: QString_split_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.split_0(self);
    // return 1;
  }
}
pub trait QString_split_0<RetType> {
  fn split_0(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_split_0<usize> for (usize,i32,i32) {
  fn split_0(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString5splitERKS_NS_13SplitBehaviorEN2Qt15CaseSensitivityE", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:510
// index:1
// Public Visibility=Default Availability=Available
// [8] QStringList split(QChar, QString::SplitBehavior, Qt::CaseSensitivity) const

/*
Splits the string into substrings wherever sep occurs, and returns the list of those strings. If sep does not match anywhere in the string, split() returns a single-element list containing this string.

cs specifies whether sep should be matched case sensitively or case insensitively.

If behavior is QString::SkipEmptyParts, empty entries don't appear in the result. By default, empty entries are kept.

Example:


  QString str = "a,,b,c";

  QStringList list1 = str.split(',');
  // list1: [ "a", "", "b", "c" ]

  QStringList list2 = str.split(',', QString::SkipEmptyParts);
  // list2: [ "a", "b", "c" ]



If sep is empty, split() returns an empty string, followed by each of the string's characters, followed by another empty string:


  QString str = "abc";
  auto parts = str.split("");
  // parts: {"", "a", "b", "c", ""}



To understand this behavior, recall that the empty string matches everywhere, so the above is qualitatively the same as:


  QString str = "/a/b/c/";
  auto parts = str.split('/');
  // parts: {"", "a", "b", "c", ""}



See also QStringList::join() and section().
*/
impl /*struct*/ QString {
  pub fn split_1<RetType, T: QString_split_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.split_1(self);
    // return 1;
  }
}
pub trait QString_split_1<RetType> {
  fn split_1(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_split_1<usize> for (usize,i32,i32) {
  fn split_1(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString5splitE5QCharNS_13SplitBehaviorEN2Qt15CaseSensitivityE", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:515
// index:2
// Public Visibility=Default Availability=Available
// [8] QStringList split(const QRegExp &, QString::SplitBehavior) const

/*
Splits the string into substrings wherever sep occurs, and returns the list of those strings. If sep does not match anywhere in the string, split() returns a single-element list containing this string.

cs specifies whether sep should be matched case sensitively or case insensitively.

If behavior is QString::SkipEmptyParts, empty entries don't appear in the result. By default, empty entries are kept.

Example:


  QString str = "a,,b,c";

  QStringList list1 = str.split(',');
  // list1: [ "a", "", "b", "c" ]

  QStringList list2 = str.split(',', QString::SkipEmptyParts);
  // list2: [ "a", "b", "c" ]



If sep is empty, split() returns an empty string, followed by each of the string's characters, followed by another empty string:


  QString str = "abc";
  auto parts = str.split("");
  // parts: {"", "a", "b", "c", ""}



To understand this behavior, recall that the empty string matches everywhere, so the above is qualitatively the same as:


  QString str = "/a/b/c/";
  auto parts = str.split('/');
  // parts: {"", "a", "b", "c", ""}



See also QStringList::join() and section().
*/
impl /*struct*/ QString {
  pub fn split_2<RetType, T: QString_split_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.split_2(self);
    // return 1;
  }
}
pub trait QString_split_2<RetType> {
  fn split_2(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_split_2<usize> for (usize,i32) {
  fn split_2(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString5splitERK7QRegExpNS_13SplitBehaviorE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:519
// index:3
// Public Visibility=Default Availability=Available
// [8] QStringList split(const QRegularExpression &, QString::SplitBehavior) const

/*
Splits the string into substrings wherever sep occurs, and returns the list of those strings. If sep does not match anywhere in the string, split() returns a single-element list containing this string.

cs specifies whether sep should be matched case sensitively or case insensitively.

If behavior is QString::SkipEmptyParts, empty entries don't appear in the result. By default, empty entries are kept.

Example:


  QString str = "a,,b,c";

  QStringList list1 = str.split(',');
  // list1: [ "a", "", "b", "c" ]

  QStringList list2 = str.split(',', QString::SkipEmptyParts);
  // list2: [ "a", "b", "c" ]



If sep is empty, split() returns an empty string, followed by each of the string's characters, followed by another empty string:


  QString str = "abc";
  auto parts = str.split("");
  // parts: {"", "a", "b", "c", ""}



To understand this behavior, recall that the empty string matches everywhere, so the above is qualitatively the same as:


  QString str = "/a/b/c/";
  auto parts = str.split('/');
  // parts: {"", "a", "b", "c", ""}



See also QStringList::join() and section().
*/
impl /*struct*/ QString {
  pub fn split_3<RetType, T: QString_split_3<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.split_3(self);
    // return 1;
  }
}
pub trait QString_split_3<RetType> {
  fn split_3(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_split_3<usize> for (usize,i32) {
  fn split_3(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString5splitERK18QRegularExpressionNS_13SplitBehaviorE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:528
// index:0
// Public Visibility=Default Availability=Available
// [8] QString normalized(QString::NormalizationForm, QChar::UnicodeVersion) const

/*
Returns the string in the given Unicode normalization mode, according to the given version of the Unicode standard.
*/
impl /*struct*/ QString {
  pub fn normalized_0<RetType, T: QString_normalized_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.normalized_0(self);
    // return 1;
  }
}
pub trait QString_normalized_0<RetType> {
  fn normalized_0(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_normalized_0<usize> for (i32,i32) {
  fn normalized_0(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString10normalizedENS_17NormalizationFormEN5QChar14UnicodeVersionE", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:530
// index:0
// Public Visibility=Default Availability=Available
// [8] QString repeated(int) const

/*
Returns a copy of this string repeated the specified number of times.

If times is less than 1, an empty string is returned.

Example:


  QString str("ab");
  str.repeated(4);            // returns "abababab"



This function was introduced in  Qt 4.5.
*/
impl /*struct*/ QString {
  pub fn repeated_0<RetType, T: QString_repeated_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.repeated_0(self);
    // return 1;
  }
}
pub trait QString_repeated_0<RetType> {
  fn repeated_0(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_repeated_0<usize> for (i32) {
  fn repeated_0(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString8repeatedEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:532
// index:0
// Public Visibility=Default Availability=Available
// [8] const ushort * utf16() const

/*
Returns the QString as a '\0'-terminated array of unsigned shorts. The result remains valid until the string is modified.

The returned string is in host byte order.

See also setUtf16() and unicode().
*/
impl /*struct*/ QString {
  pub fn utf16_0<RetType, T: QString_utf16_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.utf16_0(self);
    // return 1;
  }
}
pub trait QString_utf16_0<RetType> {
  fn utf16_0(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_utf16_0<usize> for () {
  fn utf16_0(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString5utf16Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:535
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QByteArray toLatin1() const

/*
Returns a Latin-1 representation of the string as a QByteArray.

The returned byte array is undefined if the string contains non-Latin1 characters. Those characters may be suppressed or replaced with a question mark.

See also fromLatin1(), toUtf8(), toLocal8Bit(), QTextCodec, and qConvertToLatin1().
*/
impl /*struct*/ QString {
  pub fn toLatin1_0<RetType, T: QString_toLatin1_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toLatin1_0(self);
    // return 1;
  }
}
pub trait QString_toLatin1_0<RetType> {
  fn toLatin1_0(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_toLatin1_0<usize> for () {
  fn toLatin1_0(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNKR7QString8toLatin1Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:537
// index:1
// Public inline Visibility=Default Availability=Available
// [8] QByteArray toLatin1()

/*
Returns a Latin-1 representation of the string as a QByteArray.

The returned byte array is undefined if the string contains non-Latin1 characters. Those characters may be suppressed or replaced with a question mark.

See also fromLatin1(), toUtf8(), toLocal8Bit(), QTextCodec, and qConvertToLatin1().
*/
impl /*struct*/ QString {
  pub fn toLatin1_1<RetType, T: QString_toLatin1_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toLatin1_1(self);
    // return 1;
  }
}
pub trait QString_toLatin1_1<RetType> {
  fn toLatin1_1(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_toLatin1_1<usize> for () {
  fn toLatin1_1(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNO7QString8toLatin1Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:539
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QByteArray toUtf8() const

/*
Returns a UTF-8 representation of the string as a QByteArray.

UTF-8 is a Unicode codec and can represent all characters in a Unicode string like QString.

See also fromUtf8(), toLatin1(), toLocal8Bit(), QTextCodec, and qConvertToUtf8().
*/
impl /*struct*/ QString {
  pub fn toUtf8_0<RetType, T: QString_toUtf8_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toUtf8_0(self);
    // return 1;
  }
}
pub trait QString_toUtf8_0<RetType> {
  fn toUtf8_0(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_toUtf8_0<usize> for () {
  fn toUtf8_0(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNKR7QString6toUtf8Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:541
// index:1
// Public inline Visibility=Default Availability=Available
// [8] QByteArray toUtf8()

/*
Returns a UTF-8 representation of the string as a QByteArray.

UTF-8 is a Unicode codec and can represent all characters in a Unicode string like QString.

See also fromUtf8(), toLatin1(), toLocal8Bit(), QTextCodec, and qConvertToUtf8().
*/
impl /*struct*/ QString {
  pub fn toUtf8_1<RetType, T: QString_toUtf8_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toUtf8_1(self);
    // return 1;
  }
}
pub trait QString_toUtf8_1<RetType> {
  fn toUtf8_1(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_toUtf8_1<usize> for () {
  fn toUtf8_1(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNO7QString6toUtf8Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:543
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QByteArray toLocal8Bit() const

/*
Returns the local 8-bit representation of the string as a QByteArray. The returned byte array is undefined if the string contains characters not supported by the local 8-bit encoding.

QTextCodec::codecForLocale() is used to perform the conversion from Unicode. If the locale encoding could not be determined, this function does the same as toLatin1().

If this string contains any characters that cannot be encoded in the locale, the returned byte array is undefined. Those characters may be suppressed or replaced by another.

See also fromLocal8Bit(), toLatin1(), toUtf8(), and QTextCodec.
*/
impl /*struct*/ QString {
  pub fn toLocal8Bit_0<RetType, T: QString_toLocal8Bit_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toLocal8Bit_0(self);
    // return 1;
  }
}
pub trait QString_toLocal8Bit_0<RetType> {
  fn toLocal8Bit_0(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_toLocal8Bit_0<usize> for () {
  fn toLocal8Bit_0(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNKR7QString11toLocal8BitEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:545
// index:1
// Public inline Visibility=Default Availability=Available
// [8] QByteArray toLocal8Bit()

/*
Returns the local 8-bit representation of the string as a QByteArray. The returned byte array is undefined if the string contains characters not supported by the local 8-bit encoding.

QTextCodec::codecForLocale() is used to perform the conversion from Unicode. If the locale encoding could not be determined, this function does the same as toLatin1().

If this string contains any characters that cannot be encoded in the locale, the returned byte array is undefined. Those characters may be suppressed or replaced by another.

See also fromLocal8Bit(), toLatin1(), toUtf8(), and QTextCodec.
*/
impl /*struct*/ QString {
  pub fn toLocal8Bit_1<RetType, T: QString_toLocal8Bit_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toLocal8Bit_1(self);
    // return 1;
  }
}
pub trait QString_toLocal8Bit_1<RetType> {
  fn toLocal8Bit_1(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_toLocal8Bit_1<usize> for () {
  fn toLocal8Bit_1(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNO7QString11toLocal8BitEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:555
// index:0
// Public static inline Visibility=Default Availability=Available
// [8] QString fromLatin1(const char *, int)

/*
Returns a QString initialized with the first size characters of the Latin-1 string str.

If size is -1 (default), it is taken to be strlen(str).

See also toLatin1(), fromUtf8(), and fromLocal8Bit().
*/
impl /*struct*/ QString {
  pub fn fromLatin1_0<RetType, T: QString_fromLatin1_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromLatin1_0();
    // return 1;
  }
}
pub trait QString_fromLatin1_0<RetType> {
  fn fromLatin1_0(self ) -> RetType;
}
impl<'a> /*trait*/ QString_fromLatin1_0<usize> for (usize,i32) {
  fn fromLatin1_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QString10fromLatin1EPKci", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:568
// index:1
// Public static inline Visibility=Default Availability=Available
// [8] QString fromLatin1(const QByteArray &)

/*
Returns a QString initialized with the first size characters of the Latin-1 string str.

If size is -1 (default), it is taken to be strlen(str).

See also toLatin1(), fromUtf8(), and fromLocal8Bit().
*/
impl /*struct*/ QString {
  pub fn fromLatin1_1<RetType, T: QString_fromLatin1_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromLatin1_1();
    // return 1;
  }
}
pub trait QString_fromLatin1_1<RetType> {
  fn fromLatin1_1(self ) -> RetType;
}
impl<'a> /*trait*/ QString_fromLatin1_1<usize> for (usize) {
  fn fromLatin1_1(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QString10fromLatin1ERK10QByteArray", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:560
// index:0
// Public static inline Visibility=Default Availability=Available
// [8] QString fromUtf8(const char *, int)

/*
Returns a QString initialized with the first size bytes of the UTF-8 string str.

If size is -1 (default), it is taken to be strlen(str).

UTF-8 is a Unicode codec and can represent all characters in a Unicode string like QString. However, invalid sequences are possible with UTF-8 and, if any such are found, they will be replaced with one or more "replacement characters", or suppressed. These include non-Unicode sequences, non-characters, overlong sequences or surrogate codepoints encoded into UTF-8.

This function can be used to process incoming data incrementally as long as all UTF-8 characters are terminated within the incoming data. Any unterminated characters at the end of the string will be replaced or suppressed. In order to do stateful decoding, please use QTextDecoder.

See also toUtf8(), fromLatin1(), and fromLocal8Bit().
*/
impl /*struct*/ QString {
  pub fn fromUtf8_0<RetType, T: QString_fromUtf8_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromUtf8_0();
    // return 1;
  }
}
pub trait QString_fromUtf8_0<RetType> {
  fn fromUtf8_0(self ) -> RetType;
}
impl<'a> /*trait*/ QString_fromUtf8_0<usize> for (usize,i32) {
  fn fromUtf8_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QString8fromUtf8EPKci", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:570
// index:1
// Public static inline Visibility=Default Availability=Available
// [8] QString fromUtf8(const QByteArray &)

/*
Returns a QString initialized with the first size bytes of the UTF-8 string str.

If size is -1 (default), it is taken to be strlen(str).

UTF-8 is a Unicode codec and can represent all characters in a Unicode string like QString. However, invalid sequences are possible with UTF-8 and, if any such are found, they will be replaced with one or more "replacement characters", or suppressed. These include non-Unicode sequences, non-characters, overlong sequences or surrogate codepoints encoded into UTF-8.

This function can be used to process incoming data incrementally as long as all UTF-8 characters are terminated within the incoming data. Any unterminated characters at the end of the string will be replaced or suppressed. In order to do stateful decoding, please use QTextDecoder.

See also toUtf8(), fromLatin1(), and fromLocal8Bit().
*/
impl /*struct*/ QString {
  pub fn fromUtf8_1<RetType, T: QString_fromUtf8_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromUtf8_1();
    // return 1;
  }
}
pub trait QString_fromUtf8_1<RetType> {
  fn fromUtf8_1(self ) -> RetType;
}
impl<'a> /*trait*/ QString_fromUtf8_1<usize> for (usize) {
  fn fromUtf8_1(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QString8fromUtf8ERK10QByteArray", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:564
// index:0
// Public static inline Visibility=Default Availability=Available
// [8] QString fromLocal8Bit(const char *, int)

/*
Returns a QString initialized with the first size characters of the 8-bit string str.

If size is -1 (default), it is taken to be strlen(str).

QTextCodec::codecForLocale() is used to perform the conversion.

See also toLocal8Bit(), fromLatin1(), and fromUtf8().
*/
impl /*struct*/ QString {
  pub fn fromLocal8Bit_0<RetType, T: QString_fromLocal8Bit_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromLocal8Bit_0();
    // return 1;
  }
}
pub trait QString_fromLocal8Bit_0<RetType> {
  fn fromLocal8Bit_0(self ) -> RetType;
}
impl<'a> /*trait*/ QString_fromLocal8Bit_0<usize> for (usize,i32) {
  fn fromLocal8Bit_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QString13fromLocal8BitEPKci", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:572
// index:1
// Public static inline Visibility=Default Availability=Available
// [8] QString fromLocal8Bit(const QByteArray &)

/*
Returns a QString initialized with the first size characters of the 8-bit string str.

If size is -1 (default), it is taken to be strlen(str).

QTextCodec::codecForLocale() is used to perform the conversion.

See also toLocal8Bit(), fromLatin1(), and fromUtf8().
*/
impl /*struct*/ QString {
  pub fn fromLocal8Bit_1<RetType, T: QString_fromLocal8Bit_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromLocal8Bit_1();
    // return 1;
  }
}
pub trait QString_fromLocal8Bit_1<RetType> {
  fn fromLocal8Bit_1(self ) -> RetType;
}
impl<'a> /*trait*/ QString_fromLocal8Bit_1<usize> for (usize) {
  fn fromLocal8Bit_1(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QString13fromLocal8BitERK10QByteArray", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:574
// index:0
// Public static Visibility=Default Availability=Available
// [8] QString fromUtf16(const ushort *, int)

/*
Returns a QString initialized with the first size characters of the Unicode string unicode (ISO-10646-UTF-16 encoded).

If size is -1 (default), unicode must be terminated with a 0.

This function checks for a Byte Order Mark (BOM). If it is missing, host byte order is assumed.

This function is slow compared to the other Unicode conversions. Use QString(const QChar *, int) or QString(const QChar *) if possible.

QString makes a deep copy of the Unicode data.

See also utf16(), setUtf16(), and fromStdU16String().
*/
impl /*struct*/ QString {
  pub fn fromUtf16_0<RetType, T: QString_fromUtf16_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromUtf16_0();
    // return 1;
  }
}
pub trait QString_fromUtf16_0<RetType> {
  fn fromUtf16_0(self ) -> RetType;
}
impl<'a> /*trait*/ QString_fromUtf16_0<usize> for (usize,i32) {
  fn fromUtf16_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QString9fromUtf16EPKti", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:579
// index:1
// Public static inline Visibility=Default Availability=Available
// [8] QString fromUtf16(const char16_t *, int)

/*
Returns a QString initialized with the first size characters of the Unicode string unicode (ISO-10646-UTF-16 encoded).

If size is -1 (default), unicode must be terminated with a 0.

This function checks for a Byte Order Mark (BOM). If it is missing, host byte order is assumed.

This function is slow compared to the other Unicode conversions. Use QString(const QChar *, int) or QString(const QChar *) if possible.

QString makes a deep copy of the Unicode data.

See also utf16(), setUtf16(), and fromStdU16String().
*/
impl /*struct*/ QString {
  pub fn fromUtf16_1<RetType, T: QString_fromUtf16_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromUtf16_1();
    // return 1;
  }
}
pub trait QString_fromUtf16_1<RetType> {
  fn fromUtf16_1(self ) -> RetType;
}
impl<'a> /*trait*/ QString_fromUtf16_1<usize> for (usize,i32) {
  fn fromUtf16_1(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QString9fromUtf16EPKDsi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:575
// index:0
// Public static Visibility=Default Availability=Available
// [8] QString fromUcs4(const uint *, int)

/*
Returns a QString initialized with the first size characters of the Unicode string unicode (ISO-10646-UCS-4 encoded).

If size is -1 (default), unicode must be terminated with a 0.

This function was introduced in  Qt 4.2.

See also toUcs4(), fromUtf16(), utf16(), setUtf16(), fromWCharArray(), and fromStdU32String().
*/
impl /*struct*/ QString {
  pub fn fromUcs4_0<RetType, T: QString_fromUcs4_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromUcs4_0();
    // return 1;
  }
}
pub trait QString_fromUcs4_0<RetType> {
  fn fromUcs4_0(self ) -> RetType;
}
impl<'a> /*trait*/ QString_fromUcs4_0<usize> for (usize,i32) {
  fn fromUcs4_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QString8fromUcs4EPKji", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:581
// index:1
// Public static inline Visibility=Default Availability=Available
// [8] QString fromUcs4(const char32_t *, int)

/*
Returns a QString initialized with the first size characters of the Unicode string unicode (ISO-10646-UCS-4 encoded).

If size is -1 (default), unicode must be terminated with a 0.

This function was introduced in  Qt 4.2.

See also toUcs4(), fromUtf16(), utf16(), setUtf16(), fromWCharArray(), and fromStdU32String().
*/
impl /*struct*/ QString {
  pub fn fromUcs4_1<RetType, T: QString_fromUcs4_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromUcs4_1();
    // return 1;
  }
}
pub trait QString_fromUcs4_1<RetType> {
  fn fromUcs4_1(self ) -> RetType;
}
impl<'a> /*trait*/ QString_fromUcs4_1<usize> for (usize,i32) {
  fn fromUcs4_1(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QString8fromUcs4EPKDii", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:576
// index:0
// Public static Visibility=Default Availability=Available
// [8] QString fromRawData(const QChar *, int)

/*
Constructs a QString that uses the first size Unicode characters in the array unicode. The data in unicode is not copied. The caller must be able to guarantee that unicode will not be deleted or modified as long as the QString (or an unmodified copy of it) exists.

Any attempts to modify the QString or copies of it will cause it to create a deep copy of the data, ensuring that the raw data isn't modified.

Here's an example of how we can use a QRegularExpression on raw data in memory without requiring to copy the data into a QString:


  QRegularExpression pattern("\u00A4");
  static const QChar unicode[] = {
          0x005A, 0x007F, 0x00A4, 0x0060,
          0x1009, 0x0020, 0x0020};
  int size = sizeof(unicode) / sizeof(QChar);

  QString str = QString::fromRawData(unicode, size);
  if (str.contains(pattern) {
      // ...
  }



Warning: A string created with fromRawData() is not '\0'-terminated, unless the raw data contains a '\0' character at position size. This means unicode() will not return a '\0'-terminated string (although utf16() does, at the cost of copying the raw data).

See also fromUtf16() and setRawData().
*/
impl /*struct*/ QString {
  pub fn fromRawData_0<RetType, T: QString_fromRawData_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromRawData_0();
    // return 1;
  }
}
pub trait QString_fromRawData_0<RetType> {
  fn fromRawData_0(self ) -> RetType;
}
impl<'a> /*trait*/ QString_fromRawData_0<usize> for (usize,i32) {
  fn fromRawData_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QString11fromRawDataEPK5QChari", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:594
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int toWCharArray(wchar_t *) const

/*
Fills the array with the data contained in this QString object. The array is encoded in UTF-16 on platforms where wchar_t is 2 bytes wide (e.g. windows) and in UCS-4 on platforms where wchar_t is 4 bytes wide (most Unix systems).

array has to be allocated by the caller and contain enough space to hold the complete string (allocating the array with the same length as the string is always sufficient).

This function returns the actual length of the string in array.

Note: This function does not append a null character to the array.

This function was introduced in  Qt 4.2.

See also utf16(), toUcs4(), toLatin1(), toUtf8(), toLocal8Bit(), and toStdWString().
*/
impl /*struct*/ QString {
  pub fn toWCharArray_0<RetType, T: QString_toWCharArray_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toWCharArray_0(self);
    // return 1;
  }
}
pub trait QString_toWCharArray_0<RetType> {
  fn toWCharArray_0(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_toWCharArray_0<i32> for (usize) {
  fn toWCharArray_0(self , rsthis: & QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString12toWCharArrayEPw", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:595
// index:0
// Public static inline Visibility=Default Availability=Available
// [8] QString fromWCharArray(const wchar_t *, int)

/*
Returns a copy of the string, where the encoding of string depends on the size of wchar. If wchar is 4 bytes, the string is interpreted as UCS-4, if wchar is 2 bytes it is interpreted as UTF-16.

If size is -1 (default), the string has to be 0 terminated.

This function was introduced in  Qt 4.2.

See also fromUtf16(), fromLatin1(), fromLocal8Bit(), fromUtf8(), fromUcs4(), and fromStdWString().
*/
impl /*struct*/ QString {
  pub fn fromWCharArray_0<RetType, T: QString_fromWCharArray_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromWCharArray_0();
    // return 1;
  }
}
pub trait QString_fromWCharArray_0<RetType> {
  fn fromWCharArray_0(self ) -> RetType;
}
impl<'a> /*trait*/ QString_fromWCharArray_0<usize> for (usize,i32) {
  fn fromWCharArray_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QString14fromWCharArrayEPKwi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:597
// index:0
// Public Visibility=Default Availability=Available
// [8] QString & setRawData(const QChar *, int)

/*
Resets the QString to use the first size Unicode characters in the array unicode. The data in unicode is not copied. The caller must be able to guarantee that unicode will not be deleted or modified as long as the QString (or an unmodified copy of it) exists.

This function can be used instead of fromRawData() to re-use existings QString objects to save memory re-allocations.

This function was introduced in  Qt 4.7.

See also fromRawData().
*/
impl /*struct*/ QString {
  pub fn setRawData_0<RetType, T: QString_setRawData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setRawData_0(self);
    // return 1;
  }
}
pub trait QString_setRawData_0<RetType> {
  fn setRawData_0(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_setRawData_0<usize> for (usize,i32) {
  fn setRawData_0(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QString10setRawDataEPK5QChari", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:598
// index:0
// Public Visibility=Default Availability=Available
// [8] QString & setUnicode(const QChar *, int)

/*
Resizes the string to size characters and copies unicode into the string.

If unicode is 0, nothing is copied, but the string is still resized to size.

See also unicode() and setUtf16().
*/
impl /*struct*/ QString {
  pub fn setUnicode_0<RetType, T: QString_setUnicode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setUnicode_0(self);
    // return 1;
  }
}
pub trait QString_setUnicode_0<RetType> {
  fn setUnicode_0(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_setUnicode_0<usize> for (usize,i32) {
  fn setUnicode_0(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QString10setUnicodeEPK5QChari", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:599
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QString & setUtf16(const ushort *, int)

/*
Resizes the string to size characters and copies unicode into the string.

If unicode is 0, nothing is copied, but the string is still resized to size.

Note that unlike fromUtf16(), this function does not consider BOMs and possibly differing byte ordering.

See also utf16() and setUnicode().
*/
impl /*struct*/ QString {
  pub fn setUtf16_0<RetType, T: QString_setUtf16_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setUtf16_0(self);
    // return 1;
  }
}
pub trait QString_setUtf16_0<RetType> {
  fn setUtf16_0(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_setUtf16_0<usize> for (usize,i32) {
  fn setUtf16_0(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QString8setUtf16EPKti", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:601
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
impl /*struct*/ QString {
  pub fn compare_0<RetType, T: QString_compare_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.compare_0(self);
    // return 1;
  }
}
pub trait QString_compare_0<RetType> {
  fn compare_0(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_compare_0<i32> for (usize,i32) {
  fn compare_0(self , rsthis: & QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString7compareERKS_N2Qt15CaseSensitivityE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:602
// index:1
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
impl /*struct*/ QString {
  pub fn compare_1<RetType, T: QString_compare_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.compare_1(self);
    // return 1;
  }
}
pub trait QString_compare_1<RetType> {
  fn compare_1(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_compare_1<i32> for (usize,i32) {
  fn compare_1(self , rsthis: & QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString7compareE13QLatin1StringN2Qt15CaseSensitivityE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:604
// index:2
// Public static inline Visibility=Default Availability=Available
// [4] int compare(const QString &, const QString &, Qt::CaseSensitivity)

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
impl /*struct*/ QString {
  pub fn compare_2<RetType, T: QString_compare_2<RetType>>( overload_args: T) -> RetType {
    return overload_args.compare_2();
    // return 1;
  }
}
pub trait QString_compare_2<RetType> {
  fn compare_2(self ) -> RetType;
}
impl<'a> /*trait*/ QString_compare_2<i32> for (usize,usize,i32) {
  fn compare_2(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QString7compareERKS_S1_N2Qt15CaseSensitivityE", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:608
// index:3
// Public static inline Visibility=Default Availability=Available
// [4] int compare(const QString &, QLatin1String, Qt::CaseSensitivity)

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
impl /*struct*/ QString {
  pub fn compare_3<RetType, T: QString_compare_3<RetType>>( overload_args: T) -> RetType {
    return overload_args.compare_3();
    // return 1;
  }
}
pub trait QString_compare_3<RetType> {
  fn compare_3(self ) -> RetType;
}
impl<'a> /*trait*/ QString_compare_3<i32> for (usize,usize,i32) {
  fn compare_3(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QString7compareERKS_13QLatin1StringN2Qt15CaseSensitivityE", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:611
// index:4
// Public static inline Visibility=Default Availability=Available
// [4] int compare(QLatin1String, const QString &, Qt::CaseSensitivity)

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
impl /*struct*/ QString {
  pub fn compare_4<RetType, T: QString_compare_4<RetType>>( overload_args: T) -> RetType {
    return overload_args.compare_4();
    // return 1;
  }
}
pub trait QString_compare_4<RetType> {
  fn compare_4(self ) -> RetType;
}
impl<'a> /*trait*/ QString_compare_4<i32> for (usize,usize,i32) {
  fn compare_4(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QString7compareE13QLatin1StringRKS_N2Qt15CaseSensitivityE", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:615
// index:5
// Public inline Visibility=Default Availability=Available
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
impl /*struct*/ QString {
  pub fn compare_5<RetType, T: QString_compare_5<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.compare_5(self);
    // return 1;
  }
}
pub trait QString_compare_5<RetType> {
  fn compare_5(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_compare_5<i32> for (usize,i32) {
  fn compare_5(self , rsthis: & QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString7compareERK10QStringRefN2Qt15CaseSensitivityE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:616
// index:6
// Public static Visibility=Default Availability=Available
// [4] int compare(const QString &, const QStringRef &, Qt::CaseSensitivity)

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
impl /*struct*/ QString {
  pub fn compare_6<RetType, T: QString_compare_6<RetType>>( overload_args: T) -> RetType {
    return overload_args.compare_6();
    // return 1;
  }
}
pub trait QString_compare_6<RetType> {
  fn compare_6(self ) -> RetType;
}
impl<'a> /*trait*/ QString_compare_6<i32> for (usize,usize,i32) {
  fn compare_6(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QString7compareERKS_RK10QStringRefN2Qt15CaseSensitivityE", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:619
// index:0
// Public Visibility=Default Availability=Available
// [4] int localeAwareCompare(const QString &) const

/*
Compares s1 with s2 and returns an integer less than, equal to, or greater than zero if s1 is less than, equal to, or greater than s2.

The comparison is performed in a locale- and also platform-dependent manner. Use this function to present sorted lists of strings to the user.

On macOS and iOS this function compares according the "Order for sorted lists" setting in the International preferences panel.

See also compare() and QLocale.
*/
impl /*struct*/ QString {
  pub fn localeAwareCompare_0<RetType, T: QString_localeAwareCompare_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.localeAwareCompare_0(self);
    // return 1;
  }
}
pub trait QString_localeAwareCompare_0<RetType> {
  fn localeAwareCompare_0(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_localeAwareCompare_0<i32> for (usize) {
  fn localeAwareCompare_0(self , rsthis: & QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString18localeAwareCompareERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:620
// index:1
// Public static inline Visibility=Default Availability=Available
// [4] int localeAwareCompare(const QString &, const QString &)

/*
Compares s1 with s2 and returns an integer less than, equal to, or greater than zero if s1 is less than, equal to, or greater than s2.

The comparison is performed in a locale- and also platform-dependent manner. Use this function to present sorted lists of strings to the user.

On macOS and iOS this function compares according the "Order for sorted lists" setting in the International preferences panel.

See also compare() and QLocale.
*/
impl /*struct*/ QString {
  pub fn localeAwareCompare_1<RetType, T: QString_localeAwareCompare_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.localeAwareCompare_1();
    // return 1;
  }
}
pub trait QString_localeAwareCompare_1<RetType> {
  fn localeAwareCompare_1(self ) -> RetType;
}
impl<'a> /*trait*/ QString_localeAwareCompare_1<i32> for (usize,usize) {
  fn localeAwareCompare_1(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QString18localeAwareCompareERKS_S1_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:623
// index:2
// Public Visibility=Default Availability=Available
// [4] int localeAwareCompare(const QStringRef &) const

/*
Compares s1 with s2 and returns an integer less than, equal to, or greater than zero if s1 is less than, equal to, or greater than s2.

The comparison is performed in a locale- and also platform-dependent manner. Use this function to present sorted lists of strings to the user.

On macOS and iOS this function compares according the "Order for sorted lists" setting in the International preferences panel.

See also compare() and QLocale.
*/
impl /*struct*/ QString {
  pub fn localeAwareCompare_2<RetType, T: QString_localeAwareCompare_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.localeAwareCompare_2(self);
    // return 1;
  }
}
pub trait QString_localeAwareCompare_2<RetType> {
  fn localeAwareCompare_2(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_localeAwareCompare_2<i32> for (usize) {
  fn localeAwareCompare_2(self , rsthis: & QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString18localeAwareCompareERK10QStringRef", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:624
// index:3
// Public static Visibility=Default Availability=Available
// [4] int localeAwareCompare(const QString &, const QStringRef &)

/*
Compares s1 with s2 and returns an integer less than, equal to, or greater than zero if s1 is less than, equal to, or greater than s2.

The comparison is performed in a locale- and also platform-dependent manner. Use this function to present sorted lists of strings to the user.

On macOS and iOS this function compares according the "Order for sorted lists" setting in the International preferences panel.

See also compare() and QLocale.
*/
impl /*struct*/ QString {
  pub fn localeAwareCompare_3<RetType, T: QString_localeAwareCompare_3<RetType>>( overload_args: T) -> RetType {
    return overload_args.localeAwareCompare_3();
    // return 1;
  }
}
pub trait QString_localeAwareCompare_3<RetType> {
  fn localeAwareCompare_3(self ) -> RetType;
}
impl<'a> /*trait*/ QString_localeAwareCompare_3<i32> for (usize,usize) {
  fn localeAwareCompare_3(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QString18localeAwareCompareERKS_RK10QStringRef", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:627
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
impl /*struct*/ QString {
  pub fn toShort_0<RetType, T: QString_toShort_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toShort_0(self);
    // return 1;
  }
}
pub trait QString_toShort_0<RetType> {
  fn toShort_0(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_toShort_0<i16> for (usize,i32) {
  fn toShort_0(self , rsthis: & QString) -> i16 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString7toShortEPbi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i16 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:628
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
impl /*struct*/ QString {
  pub fn toUShort_0<RetType, T: QString_toUShort_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toUShort_0(self);
    // return 1;
  }
}
pub trait QString_toUShort_0<RetType> {
  fn toUShort_0(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_toUShort_0<u16> for (usize,i32) {
  fn toUShort_0(self , rsthis: & QString) -> u16 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString8toUShortEPbi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u16 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:629
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
impl /*struct*/ QString {
  pub fn toInt_0<RetType, T: QString_toInt_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toInt_0(self);
    // return 1;
  }
}
pub trait QString_toInt_0<RetType> {
  fn toInt_0(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_toInt_0<i32> for (usize,i32) {
  fn toInt_0(self , rsthis: & QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString5toIntEPbi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:630
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
impl /*struct*/ QString {
  pub fn toUInt_0<RetType, T: QString_toUInt_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toUInt_0(self);
    // return 1;
  }
}
pub trait QString_toUInt_0<RetType> {
  fn toUInt_0(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_toUInt_0<u32> for (usize,i32) {
  fn toUInt_0(self , rsthis: & QString) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString6toUIntEPbi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:631
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
impl /*struct*/ QString {
  pub fn toLong_0<RetType, T: QString_toLong_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toLong_0(self);
    // return 1;
  }
}
pub trait QString_toLong_0<RetType> {
  fn toLong_0(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_toLong_0<i64> for (usize,i32) {
  fn toLong_0(self , rsthis: & QString) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString6toLongEPbi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:632
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
impl /*struct*/ QString {
  pub fn toULong_0<RetType, T: QString_toULong_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toULong_0(self);
    // return 1;
  }
}
pub trait QString_toULong_0<RetType> {
  fn toULong_0(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_toULong_0<u64> for (usize,i32) {
  fn toULong_0(self , rsthis: & QString) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString7toULongEPbi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:633
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
impl /*struct*/ QString {
  pub fn toLongLong_0<RetType, T: QString_toLongLong_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toLongLong_0(self);
    // return 1;
  }
}
pub trait QString_toLongLong_0<RetType> {
  fn toLongLong_0(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_toLongLong_0<i64> for (usize,i32) {
  fn toLongLong_0(self , rsthis: & QString) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString10toLongLongEPbi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:634
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
impl /*struct*/ QString {
  pub fn toULongLong_0<RetType, T: QString_toULongLong_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toULongLong_0(self);
    // return 1;
  }
}
pub trait QString_toULongLong_0<RetType> {
  fn toULongLong_0(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_toULongLong_0<u64> for (usize,i32) {
  fn toULongLong_0(self , rsthis: & QString) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString11toULongLongEPbi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:635
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
impl /*struct*/ QString {
  pub fn toFloat_0<RetType, T: QString_toFloat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toFloat_0(self);
    // return 1;
  }
}
pub trait QString_toFloat_0<RetType> {
  fn toFloat_0(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_toFloat_0<f32> for (usize) {
  fn toFloat_0(self , rsthis: & QString) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString7toFloatEPb", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:636
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
impl /*struct*/ QString {
  pub fn toDouble_0<RetType, T: QString_toDouble_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toDouble_0(self);
    // return 1;
  }
}
pub trait QString_toDouble_0<RetType> {
  fn toDouble_0(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_toDouble_0<f64> for (usize) {
  fn toDouble_0(self , rsthis: & QString) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString8toDoubleEPb", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:638
// index:0
// Public Visibility=Default Availability=Available
// [8] QString & setNum(short, int)

/*
Sets the string to the printed value of n in the specified base, and returns a reference to the string.

The base is 10 by default and must be between 2 and 36. For bases other than 10, n is treated as an unsigned integer.


  QString str;
  str.setNum(1234);       // str == "1234"



The formatting always uses QLocale::C, i.e., English/UnitedStates. To get a localized string representation of a number, use QLocale::toString() with the appropriate locale.
*/
impl /*struct*/ QString {
  pub fn setNum_0<RetType, T: QString_setNum_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setNum_0(self);
    // return 1;
  }
}
pub trait QString_setNum_0<RetType> {
  fn setNum_0(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_setNum_0<usize> for (i16,i32) {
  fn setNum_0(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i16 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QString6setNumEsi", 2,qtrt::FFITY_SINT16,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:639
// index:1
// Public Visibility=Default Availability=Available
// [8] QString & setNum(ushort, int)

/*
Sets the string to the printed value of n in the specified base, and returns a reference to the string.

The base is 10 by default and must be between 2 and 36. For bases other than 10, n is treated as an unsigned integer.


  QString str;
  str.setNum(1234);       // str == "1234"



The formatting always uses QLocale::C, i.e., English/UnitedStates. To get a localized string representation of a number, use QLocale::toString() with the appropriate locale.
*/
impl /*struct*/ QString {
  pub fn setNum_1<RetType, T: QString_setNum_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setNum_1(self);
    // return 1;
  }
}
pub trait QString_setNum_1<RetType> {
  fn setNum_1(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_setNum_1<usize> for (u16,i32) {
  fn setNum_1(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const u16 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QString6setNumEti", 2,qtrt::FFITY_UINT16,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:640
// index:2
// Public Visibility=Default Availability=Available
// [8] QString & setNum(int, int)

/*
Sets the string to the printed value of n in the specified base, and returns a reference to the string.

The base is 10 by default and must be between 2 and 36. For bases other than 10, n is treated as an unsigned integer.


  QString str;
  str.setNum(1234);       // str == "1234"



The formatting always uses QLocale::C, i.e., English/UnitedStates. To get a localized string representation of a number, use QLocale::toString() with the appropriate locale.
*/
impl /*struct*/ QString {
  pub fn setNum_2<RetType, T: QString_setNum_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setNum_2(self);
    // return 1;
  }
}
pub trait QString_setNum_2<RetType> {
  fn setNum_2(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_setNum_2<usize> for (i32,i32) {
  fn setNum_2(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QString6setNumEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:641
// index:3
// Public Visibility=Default Availability=Available
// [8] QString & setNum(uint, int)

/*
Sets the string to the printed value of n in the specified base, and returns a reference to the string.

The base is 10 by default and must be between 2 and 36. For bases other than 10, n is treated as an unsigned integer.


  QString str;
  str.setNum(1234);       // str == "1234"



The formatting always uses QLocale::C, i.e., English/UnitedStates. To get a localized string representation of a number, use QLocale::toString() with the appropriate locale.
*/
impl /*struct*/ QString {
  pub fn setNum_3<RetType, T: QString_setNum_3<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setNum_3(self);
    // return 1;
  }
}
pub trait QString_setNum_3<RetType> {
  fn setNum_3(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_setNum_3<usize> for (u32,i32) {
  fn setNum_3(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const u32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QString6setNumEji", 2,qtrt::FFITY_UINT32,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:642
// index:4
// Public Visibility=Default Availability=Available
// [8] QString & setNum(long, int)

/*
Sets the string to the printed value of n in the specified base, and returns a reference to the string.

The base is 10 by default and must be between 2 and 36. For bases other than 10, n is treated as an unsigned integer.


  QString str;
  str.setNum(1234);       // str == "1234"



The formatting always uses QLocale::C, i.e., English/UnitedStates. To get a localized string representation of a number, use QLocale::toString() with the appropriate locale.
*/
impl /*struct*/ QString {
  pub fn setNum_4<RetType, T: QString_setNum_4<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setNum_4(self);
    // return 1;
  }
}
pub trait QString_setNum_4<RetType> {
  fn setNum_4(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_setNum_4<usize> for (i64,i32) {
  fn setNum_4(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i64 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QString6setNumEli", 2,qtrt::FFITY_SINT64,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:643
// index:5
// Public Visibility=Default Availability=Available
// [8] QString & setNum(ulong, int)

/*
Sets the string to the printed value of n in the specified base, and returns a reference to the string.

The base is 10 by default and must be between 2 and 36. For bases other than 10, n is treated as an unsigned integer.


  QString str;
  str.setNum(1234);       // str == "1234"



The formatting always uses QLocale::C, i.e., English/UnitedStates. To get a localized string representation of a number, use QLocale::toString() with the appropriate locale.
*/
impl /*struct*/ QString {
  pub fn setNum_5<RetType, T: QString_setNum_5<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setNum_5(self);
    // return 1;
  }
}
pub trait QString_setNum_5<RetType> {
  fn setNum_5(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_setNum_5<usize> for (u64,i32) {
  fn setNum_5(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const u64 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QString6setNumEmi", 2,qtrt::FFITY_UINT64,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:644
// index:6
// Public Visibility=Default Availability=Available
// [8] QString & setNum(qlonglong, int)

/*
Sets the string to the printed value of n in the specified base, and returns a reference to the string.

The base is 10 by default and must be between 2 and 36. For bases other than 10, n is treated as an unsigned integer.


  QString str;
  str.setNum(1234);       // str == "1234"



The formatting always uses QLocale::C, i.e., English/UnitedStates. To get a localized string representation of a number, use QLocale::toString() with the appropriate locale.
*/
impl /*struct*/ QString {
  pub fn setNum_6<RetType, T: QString_setNum_6<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setNum_6(self);
    // return 1;
  }
}
pub trait QString_setNum_6<RetType> {
  fn setNum_6(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_setNum_6<usize> for (i64,i32) {
  fn setNum_6(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i64 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QString6setNumExi", 2,qtrt::FFITY_SINT64,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:645
// index:7
// Public Visibility=Default Availability=Available
// [8] QString & setNum(qulonglong, int)

/*
Sets the string to the printed value of n in the specified base, and returns a reference to the string.

The base is 10 by default and must be between 2 and 36. For bases other than 10, n is treated as an unsigned integer.


  QString str;
  str.setNum(1234);       // str == "1234"



The formatting always uses QLocale::C, i.e., English/UnitedStates. To get a localized string representation of a number, use QLocale::toString() with the appropriate locale.
*/
impl /*struct*/ QString {
  pub fn setNum_7<RetType, T: QString_setNum_7<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setNum_7(self);
    // return 1;
  }
}
pub trait QString_setNum_7<RetType> {
  fn setNum_7(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_setNum_7<usize> for (u64,i32) {
  fn setNum_7(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const u64 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QString6setNumEyi", 2,qtrt::FFITY_UINT64,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:646
// index:8
// Public Visibility=Default Availability=Available
// [8] QString & setNum(float, char, int)

/*
Sets the string to the printed value of n in the specified base, and returns a reference to the string.

The base is 10 by default and must be between 2 and 36. For bases other than 10, n is treated as an unsigned integer.


  QString str;
  str.setNum(1234);       // str == "1234"



The formatting always uses QLocale::C, i.e., English/UnitedStates. To get a localized string representation of a number, use QLocale::toString() with the appropriate locale.
*/
impl /*struct*/ QString {
  pub fn setNum_8<RetType, T: QString_setNum_8<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setNum_8(self);
    // return 1;
  }
}
pub trait QString_setNum_8<RetType> {
  fn setNum_8(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_setNum_8<usize> for (f32,i8,i32) {
  fn setNum_8(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f32 as usize;
    let arg1 = (&self.1) as *const i8 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QString6setNumEfci", 3,qtrt::FFITY_FLOAT,qtrt::FFITY_SINT8,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:647
// index:9
// Public Visibility=Default Availability=Available
// [8] QString & setNum(double, char, int)

/*
Sets the string to the printed value of n in the specified base, and returns a reference to the string.

The base is 10 by default and must be between 2 and 36. For bases other than 10, n is treated as an unsigned integer.


  QString str;
  str.setNum(1234);       // str == "1234"



The formatting always uses QLocale::C, i.e., English/UnitedStates. To get a localized string representation of a number, use QLocale::toString() with the appropriate locale.
*/
impl /*struct*/ QString {
  pub fn setNum_9<RetType, T: QString_setNum_9<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setNum_9(self);
    // return 1;
  }
}
pub trait QString_setNum_9<RetType> {
  fn setNum_9(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_setNum_9<usize> for (f64,i8,i32) {
  fn setNum_9(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const i8 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QString6setNumEdci", 3,qtrt::FFITY_DOUBLE,qtrt::FFITY_SINT8,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:649
// index:0
// Public static Visibility=Default Availability=Available
// [8] QString number(int, int)

/*
Returns a string equivalent of the number n according to the specified base.

The base is 10 by default and must be between 2 and 36. For bases other than 10, n is treated as an unsigned integer.

The formatting always uses QLocale::C, i.e., English/UnitedStates. To get a localized string representation of a number, use QLocale::toString() with the appropriate locale.


  long a = 63;
  QString s = QString::number(a, 16);             // s == "3f"
  QString t = QString::number(a, 16).toUpper();     // t == "3F"



See also setNum().
*/
impl /*struct*/ QString {
  pub fn number_0<RetType, T: QString_number_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.number_0();
    // return 1;
  }
}
pub trait QString_number_0<RetType> {
  fn number_0(self ) -> RetType;
}
impl<'a> /*trait*/ QString_number_0<usize> for (i32,i32) {
  fn number_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QString6numberEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:650
// index:1
// Public static Visibility=Default Availability=Available
// [8] QString number(uint, int)

/*
Returns a string equivalent of the number n according to the specified base.

The base is 10 by default and must be between 2 and 36. For bases other than 10, n is treated as an unsigned integer.

The formatting always uses QLocale::C, i.e., English/UnitedStates. To get a localized string representation of a number, use QLocale::toString() with the appropriate locale.


  long a = 63;
  QString s = QString::number(a, 16);             // s == "3f"
  QString t = QString::number(a, 16).toUpper();     // t == "3F"



See also setNum().
*/
impl /*struct*/ QString {
  pub fn number_1<RetType, T: QString_number_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.number_1();
    // return 1;
  }
}
pub trait QString_number_1<RetType> {
  fn number_1(self ) -> RetType;
}
impl<'a> /*trait*/ QString_number_1<usize> for (u32,i32) {
  fn number_1(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const u32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QString6numberEji", 2,qtrt::FFITY_UINT32,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:651
// index:2
// Public static Visibility=Default Availability=Available
// [8] QString number(long, int)

/*
Returns a string equivalent of the number n according to the specified base.

The base is 10 by default and must be between 2 and 36. For bases other than 10, n is treated as an unsigned integer.

The formatting always uses QLocale::C, i.e., English/UnitedStates. To get a localized string representation of a number, use QLocale::toString() with the appropriate locale.


  long a = 63;
  QString s = QString::number(a, 16);             // s == "3f"
  QString t = QString::number(a, 16).toUpper();     // t == "3F"



See also setNum().
*/
impl /*struct*/ QString {
  pub fn number_2<RetType, T: QString_number_2<RetType>>( overload_args: T) -> RetType {
    return overload_args.number_2();
    // return 1;
  }
}
pub trait QString_number_2<RetType> {
  fn number_2(self ) -> RetType;
}
impl<'a> /*trait*/ QString_number_2<usize> for (i64,i32) {
  fn number_2(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i64 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QString6numberEli", 2,qtrt::FFITY_SINT64,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:652
// index:3
// Public static Visibility=Default Availability=Available
// [8] QString number(ulong, int)

/*
Returns a string equivalent of the number n according to the specified base.

The base is 10 by default and must be between 2 and 36. For bases other than 10, n is treated as an unsigned integer.

The formatting always uses QLocale::C, i.e., English/UnitedStates. To get a localized string representation of a number, use QLocale::toString() with the appropriate locale.


  long a = 63;
  QString s = QString::number(a, 16);             // s == "3f"
  QString t = QString::number(a, 16).toUpper();     // t == "3F"



See also setNum().
*/
impl /*struct*/ QString {
  pub fn number_3<RetType, T: QString_number_3<RetType>>( overload_args: T) -> RetType {
    return overload_args.number_3();
    // return 1;
  }
}
pub trait QString_number_3<RetType> {
  fn number_3(self ) -> RetType;
}
impl<'a> /*trait*/ QString_number_3<usize> for (u64,i32) {
  fn number_3(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const u64 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QString6numberEmi", 2,qtrt::FFITY_UINT64,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:653
// index:4
// Public static Visibility=Default Availability=Available
// [8] QString number(qlonglong, int)

/*
Returns a string equivalent of the number n according to the specified base.

The base is 10 by default and must be between 2 and 36. For bases other than 10, n is treated as an unsigned integer.

The formatting always uses QLocale::C, i.e., English/UnitedStates. To get a localized string representation of a number, use QLocale::toString() with the appropriate locale.


  long a = 63;
  QString s = QString::number(a, 16);             // s == "3f"
  QString t = QString::number(a, 16).toUpper();     // t == "3F"



See also setNum().
*/
impl /*struct*/ QString {
  pub fn number_4<RetType, T: QString_number_4<RetType>>( overload_args: T) -> RetType {
    return overload_args.number_4();
    // return 1;
  }
}
pub trait QString_number_4<RetType> {
  fn number_4(self ) -> RetType;
}
impl<'a> /*trait*/ QString_number_4<usize> for (i64,i32) {
  fn number_4(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i64 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QString6numberExi", 2,qtrt::FFITY_SINT64,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:654
// index:5
// Public static Visibility=Default Availability=Available
// [8] QString number(qulonglong, int)

/*
Returns a string equivalent of the number n according to the specified base.

The base is 10 by default and must be between 2 and 36. For bases other than 10, n is treated as an unsigned integer.

The formatting always uses QLocale::C, i.e., English/UnitedStates. To get a localized string representation of a number, use QLocale::toString() with the appropriate locale.


  long a = 63;
  QString s = QString::number(a, 16);             // s == "3f"
  QString t = QString::number(a, 16).toUpper();     // t == "3F"



See also setNum().
*/
impl /*struct*/ QString {
  pub fn number_5<RetType, T: QString_number_5<RetType>>( overload_args: T) -> RetType {
    return overload_args.number_5();
    // return 1;
  }
}
pub trait QString_number_5<RetType> {
  fn number_5(self ) -> RetType;
}
impl<'a> /*trait*/ QString_number_5<usize> for (u64,i32) {
  fn number_5(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const u64 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QString6numberEyi", 2,qtrt::FFITY_UINT64,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:655
// index:6
// Public static Visibility=Default Availability=Available
// [8] QString number(double, char, int)

/*
Returns a string equivalent of the number n according to the specified base.

The base is 10 by default and must be between 2 and 36. For bases other than 10, n is treated as an unsigned integer.

The formatting always uses QLocale::C, i.e., English/UnitedStates. To get a localized string representation of a number, use QLocale::toString() with the appropriate locale.


  long a = 63;
  QString s = QString::number(a, 16);             // s == "3f"
  QString t = QString::number(a, 16).toUpper();     // t == "3F"



See also setNum().
*/
impl /*struct*/ QString {
  pub fn number_6<RetType, T: QString_number_6<RetType>>( overload_args: T) -> RetType {
    return overload_args.number_6();
    // return 1;
  }
}
pub trait QString_number_6<RetType> {
  fn number_6(self ) -> RetType;
}
impl<'a> /*trait*/ QString_number_6<usize> for (f64,i8,i32) {
  fn number_6(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const i8 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QString6numberEdci", 3,qtrt::FFITY_DOUBLE,qtrt::FFITY_SINT8,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:664
// index:0
// Public Visibility=Default Availability=Available
// [1] bool operator==(QLatin1String) const

/*

*/
impl /*struct*/ QString {
  pub fn operator_equal_equal_0<RetType, T: QString_operator_equal_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_equal_0(self);
    // return 1;
  }
}
pub trait QString_operator_equal_equal_0<RetType> {
  fn operator_equal_equal_0(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_operator_equal_equal_0<bool> for (usize) {
  fn operator_equal_equal_0(self , rsthis: & QString) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QStringeqE13QLatin1String", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:715
// index:1
// Public inline Visibility=Default Availability=Available
// [1] bool operator==(const char *) const

/*

*/
impl /*struct*/ QString {
  pub fn operator_equal_equal_1<RetType, T: QString_operator_equal_equal_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_equal_1(self);
    // return 1;
  }
}
pub trait QString_operator_equal_equal_1<RetType> {
  fn operator_equal_equal_1(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_operator_equal_equal_1<bool> for (usize) {
  fn operator_equal_equal_1(self , rsthis: & QString) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QStringeqEPKc", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:722
// index:2
// Public inline Visibility=Default Availability=Available
// [1] bool operator==(const QByteArray &) const

/*

*/
impl /*struct*/ QString {
  pub fn operator_equal_equal_2<RetType, T: QString_operator_equal_equal_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_equal_2(self);
    // return 1;
  }
}
pub trait QString_operator_equal_equal_2<RetType> {
  fn operator_equal_equal_2(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_operator_equal_equal_2<bool> for (usize) {
  fn operator_equal_equal_2(self , rsthis: & QString) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QStringeqERK10QByteArray", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:665
// index:0
// Public Visibility=Default Availability=Available
// [1] bool operator<(QLatin1String) const

/*

*/
impl /*struct*/ QString {
  pub fn operator_less_than_0<RetType, T: QString_operator_less_than_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_less_than_0(self);
    // return 1;
  }
}
pub trait QString_operator_less_than_0<RetType> {
  fn operator_less_than_0(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_operator_less_than_0<bool> for (usize) {
  fn operator_less_than_0(self , rsthis: & QString) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QStringltE13QLatin1String", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:717
// index:1
// Public inline Visibility=Default Availability=Available
// [1] bool operator<(const char *) const

/*

*/
impl /*struct*/ QString {
  pub fn operator_less_than_1<RetType, T: QString_operator_less_than_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_less_than_1(self);
    // return 1;
  }
}
pub trait QString_operator_less_than_1<RetType> {
  fn operator_less_than_1(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_operator_less_than_1<bool> for (usize) {
  fn operator_less_than_1(self , rsthis: & QString) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QStringltEPKc", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:724
// index:2
// Public inline Visibility=Default Availability=Available
// [1] bool operator<(const QByteArray &) const

/*

*/
impl /*struct*/ QString {
  pub fn operator_less_than_2<RetType, T: QString_operator_less_than_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_less_than_2(self);
    // return 1;
  }
}
pub trait QString_operator_less_than_2<RetType> {
  fn operator_less_than_2(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_operator_less_than_2<bool> for (usize) {
  fn operator_less_than_2(self , rsthis: & QString) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QStringltERK10QByteArray", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:666
// index:0
// Public Visibility=Default Availability=Available
// [1] bool operator>(QLatin1String) const

/*

*/
impl /*struct*/ QString {
  pub fn operator_greater_than_0<RetType, T: QString_operator_greater_than_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_greater_than_0(self);
    // return 1;
  }
}
pub trait QString_operator_greater_than_0<RetType> {
  fn operator_greater_than_0(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_operator_greater_than_0<bool> for (usize) {
  fn operator_greater_than_0(self , rsthis: & QString) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QStringgtE13QLatin1String", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:719
// index:1
// Public inline Visibility=Default Availability=Available
// [1] bool operator>(const char *) const

/*

*/
impl /*struct*/ QString {
  pub fn operator_greater_than_1<RetType, T: QString_operator_greater_than_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_greater_than_1(self);
    // return 1;
  }
}
pub trait QString_operator_greater_than_1<RetType> {
  fn operator_greater_than_1(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_operator_greater_than_1<bool> for (usize) {
  fn operator_greater_than_1(self , rsthis: & QString) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QStringgtEPKc", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:725
// index:2
// Public inline Visibility=Default Availability=Available
// [1] bool operator>(const QByteArray &) const

/*

*/
impl /*struct*/ QString {
  pub fn operator_greater_than_2<RetType, T: QString_operator_greater_than_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_greater_than_2(self);
    // return 1;
  }
}
pub trait QString_operator_greater_than_2<RetType> {
  fn operator_greater_than_2(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_operator_greater_than_2<bool> for (usize) {
  fn operator_greater_than_2(self , rsthis: & QString) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QStringgtERK10QByteArray", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:667
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator!=(QLatin1String) const

/*

*/
impl /*struct*/ QString {
  pub fn operator_not_equal_0<RetType, T: QString_operator_not_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_not_equal_0(self);
    // return 1;
  }
}
pub trait QString_operator_not_equal_0<RetType> {
  fn operator_not_equal_0(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_operator_not_equal_0<bool> for (usize) {
  fn operator_not_equal_0(self , rsthis: & QString) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QStringneE13QLatin1String", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:716
// index:1
// Public inline Visibility=Default Availability=Available
// [1] bool operator!=(const char *) const

/*

*/
impl /*struct*/ QString {
  pub fn operator_not_equal_1<RetType, T: QString_operator_not_equal_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_not_equal_1(self);
    // return 1;
  }
}
pub trait QString_operator_not_equal_1<RetType> {
  fn operator_not_equal_1(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_operator_not_equal_1<bool> for (usize) {
  fn operator_not_equal_1(self , rsthis: & QString) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QStringneEPKc", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:723
// index:2
// Public inline Visibility=Default Availability=Available
// [1] bool operator!=(const QByteArray &) const

/*

*/
impl /*struct*/ QString {
  pub fn operator_not_equal_2<RetType, T: QString_operator_not_equal_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_not_equal_2(self);
    // return 1;
  }
}
pub trait QString_operator_not_equal_2<RetType> {
  fn operator_not_equal_2(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_operator_not_equal_2<bool> for (usize) {
  fn operator_not_equal_2(self , rsthis: & QString) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QStringneERK10QByteArray", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:668
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator<=(QLatin1String) const

/*

*/
impl /*struct*/ QString {
  pub fn operator_less_than_equal_0<RetType, T: QString_operator_less_than_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_less_than_equal_0(self);
    // return 1;
  }
}
pub trait QString_operator_less_than_equal_0<RetType> {
  fn operator_less_than_equal_0(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_operator_less_than_equal_0<bool> for (usize) {
  fn operator_less_than_equal_0(self , rsthis: & QString) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QStringleE13QLatin1String", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:718
// index:1
// Public inline Visibility=Default Availability=Available
// [1] bool operator<=(const char *) const

/*

*/
impl /*struct*/ QString {
  pub fn operator_less_than_equal_1<RetType, T: QString_operator_less_than_equal_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_less_than_equal_1(self);
    // return 1;
  }
}
pub trait QString_operator_less_than_equal_1<RetType> {
  fn operator_less_than_equal_1(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_operator_less_than_equal_1<bool> for (usize) {
  fn operator_less_than_equal_1(self , rsthis: & QString) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QStringleEPKc", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:726
// index:2
// Public inline Visibility=Default Availability=Available
// [1] bool operator<=(const QByteArray &) const

/*

*/
impl /*struct*/ QString {
  pub fn operator_less_than_equal_2<RetType, T: QString_operator_less_than_equal_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_less_than_equal_2(self);
    // return 1;
  }
}
pub trait QString_operator_less_than_equal_2<RetType> {
  fn operator_less_than_equal_2(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_operator_less_than_equal_2<bool> for (usize) {
  fn operator_less_than_equal_2(self , rsthis: & QString) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QStringleERK10QByteArray", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:669
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator>=(QLatin1String) const

/*

*/
impl /*struct*/ QString {
  pub fn operator_greater_than_equal_0<RetType, T: QString_operator_greater_than_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_greater_than_equal_0(self);
    // return 1;
  }
}
pub trait QString_operator_greater_than_equal_0<RetType> {
  fn operator_greater_than_equal_0(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_operator_greater_than_equal_0<bool> for (usize) {
  fn operator_greater_than_equal_0(self , rsthis: & QString) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QStringgeE13QLatin1String", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:720
// index:1
// Public inline Visibility=Default Availability=Available
// [1] bool operator>=(const char *) const

/*

*/
impl /*struct*/ QString {
  pub fn operator_greater_than_equal_1<RetType, T: QString_operator_greater_than_equal_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_greater_than_equal_1(self);
    // return 1;
  }
}
pub trait QString_operator_greater_than_equal_1<RetType> {
  fn operator_greater_than_equal_1(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_operator_greater_than_equal_1<bool> for (usize) {
  fn operator_greater_than_equal_1(self , rsthis: & QString) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QStringgeEPKc", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:727
// index:2
// Public inline Visibility=Default Availability=Available
// [1] bool operator>=(const QByteArray &) const

/*

*/
impl /*struct*/ QString {
  pub fn operator_greater_than_equal_2<RetType, T: QString_operator_greater_than_equal_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_greater_than_equal_2(self);
    // return 1;
  }
}
pub trait QString_operator_greater_than_equal_2<RetType> {
  fn operator_greater_than_equal_2(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_operator_greater_than_equal_2<bool> for (usize) {
  fn operator_greater_than_equal_2(self , rsthis: & QString) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QStringgeERK10QByteArray", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:750
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QString::iterator begin()

/*
Returns an STL-style iterator pointing to the first character in the string.

See also constBegin() and end().
*/
impl /*struct*/ QString {
  pub fn begin_0<RetType, T: QString_begin_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.begin_0(self);
    // return 1;
  }
}
pub trait QString_begin_0<RetType> {
  fn begin_0(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_begin_0<usize> for () {
  fn begin_0(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QString5beginEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:751
// index:1
// Public inline Visibility=Default Availability=Available
// [8] QString::const_iterator begin() const

/*
Returns an STL-style iterator pointing to the first character in the string.

See also constBegin() and end().
*/
impl /*struct*/ QString {
  pub fn begin_1<RetType, T: QString_begin_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.begin_1(self);
    // return 1;
  }
}
pub trait QString_begin_1<RetType> {
  fn begin_1(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_begin_1<usize> for () {
  fn begin_1(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString5beginEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:752
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QString::const_iterator cbegin() const

/*
Returns a const STL-style iterator pointing to the first character in the string.

This function was introduced in  Qt 5.0.

See also begin() and cend().
*/
impl /*struct*/ QString {
  pub fn cbegin_0<RetType, T: QString_cbegin_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cbegin_0(self);
    // return 1;
  }
}
pub trait QString_cbegin_0<RetType> {
  fn cbegin_0(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_cbegin_0<usize> for () {
  fn cbegin_0(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString6cbeginEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:753
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QString::const_iterator constBegin() const

/*
Returns a const STL-style iterator pointing to the first character in the string.

See also begin() and constEnd().
*/
impl /*struct*/ QString {
  pub fn constBegin_0<RetType, T: QString_constBegin_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.constBegin_0(self);
    // return 1;
  }
}
pub trait QString_constBegin_0<RetType> {
  fn constBegin_0(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_constBegin_0<usize> for () {
  fn constBegin_0(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString10constBeginEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:754
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QString::iterator end()

/*
Returns an STL-style iterator pointing to the imaginary character after the last character in the string.

See also begin() and constEnd().
*/
impl /*struct*/ QString {
  pub fn end_0<RetType, T: QString_end_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.end_0(self);
    // return 1;
  }
}
pub trait QString_end_0<RetType> {
  fn end_0(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_end_0<usize> for () {
  fn end_0(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QString3endEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:755
// index:1
// Public inline Visibility=Default Availability=Available
// [8] QString::const_iterator end() const

/*
Returns an STL-style iterator pointing to the imaginary character after the last character in the string.

See also begin() and constEnd().
*/
impl /*struct*/ QString {
  pub fn end_1<RetType, T: QString_end_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.end_1(self);
    // return 1;
  }
}
pub trait QString_end_1<RetType> {
  fn end_1(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_end_1<usize> for () {
  fn end_1(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString3endEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:756
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QString::const_iterator cend() const

/*
Returns a const STL-style iterator pointing to the imaginary character after the last character in the list.

This function was introduced in  Qt 5.0.

See also cbegin() and end().
*/
impl /*struct*/ QString {
  pub fn cend_0<RetType, T: QString_cend_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cend_0(self);
    // return 1;
  }
}
pub trait QString_cend_0<RetType> {
  fn cend_0(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_cend_0<usize> for () {
  fn cend_0(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString4cendEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:757
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QString::const_iterator constEnd() const

/*
Returns a const STL-style iterator pointing to the imaginary character after the last character in the list.

See also constBegin() and end().
*/
impl /*struct*/ QString {
  pub fn constEnd_0<RetType, T: QString_constEnd_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.constEnd_0(self);
    // return 1;
  }
}
pub trait QString_constEnd_0<RetType> {
  fn constEnd_0(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_constEnd_0<usize> for () {
  fn constEnd_0(self , rsthis: & QString) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString8constEndEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:773
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void push_back(QChar)

/*
This function is provided for STL compatibility, appending the given other string onto the end of this string. It is equivalent to append(other).

See also append().
*/
impl /*struct*/ QString {
  pub fn push_back_0<RetType, T: QString_push_back_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.push_back_0(self);
    // return 1;
  }
}
pub trait QString_push_back_0<RetType> {
  fn push_back_0(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_push_back_0<(/*void*/)> for (usize) {
  fn push_back_0(self , rsthis: & QString) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QString9push_backE5QChar", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qstring.h:774
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void push_back(const QString &)

/*
This function is provided for STL compatibility, appending the given other string onto the end of this string. It is equivalent to append(other).

See also append().
*/
impl /*struct*/ QString {
  pub fn push_back_1<RetType, T: QString_push_back_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.push_back_1(self);
    // return 1;
  }
}
pub trait QString_push_back_1<RetType> {
  fn push_back_1(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_push_back_1<(/*void*/)> for (usize) {
  fn push_back_1(self , rsthis: & QString) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QString9push_backERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qstring.h:775
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void push_front(QChar)

/*
This function is provided for STL compatibility, prepending the given other string to the beginning of this string. It is equivalent to prepend(other).

See also prepend().
*/
impl /*struct*/ QString {
  pub fn push_front_0<RetType, T: QString_push_front_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.push_front_0(self);
    // return 1;
  }
}
pub trait QString_push_front_0<RetType> {
  fn push_front_0(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_push_front_0<(/*void*/)> for (usize) {
  fn push_front_0(self , rsthis: & QString) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QString10push_frontE5QChar", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qstring.h:776
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void push_front(const QString &)

/*
This function is provided for STL compatibility, prepending the given other string to the beginning of this string. It is equivalent to prepend(other).

See also prepend().
*/
impl /*struct*/ QString {
  pub fn push_front_1<RetType, T: QString_push_front_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.push_front_1(self);
    // return 1;
  }
}
pub trait QString_push_front_1<RetType> {
  fn push_front_1(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_push_front_1<(/*void*/)> for (usize) {
  fn push_front_1(self , rsthis: & QString) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QString10push_frontERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qstring.h:777
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void shrink_to_fit()

/*
This function is provided for STL compatibility. It is equivalent to squeeze().

This function was introduced in  Qt 5.10.

See also squeeze().
*/
impl /*struct*/ QString {
  pub fn shrink_to_fit_0<RetType, T: QString_shrink_to_fit_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.shrink_to_fit_0(self);
    // return 1;
  }
}
pub trait QString_shrink_to_fit_0<RetType> {
  fn shrink_to_fit_0(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_shrink_to_fit_0<(/*void*/)> for () {
  fn shrink_to_fit_0(self , rsthis: & QString) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN7QString13shrink_to_fitEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qstring.h:780
// index:0
// Public inline Visibility=Default Availability=Available
// [32] std::string toStdString() const

/*
Returns a std::string object with the data contained in this QString. The Unicode data is converted into 8-bit characters using the toUtf8() function.

This method is mostly useful to pass a QString to a function that accepts a std::string object.

See also toLatin1(), toUtf8(), toLocal8Bit(), and QByteArray::toStdString().
*/
impl /*struct*/ QString {
  pub fn toStdString_0<RetType, T: QString_toStdString_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toStdString_0(self);
    // return 1;
  }
}
pub trait QString_toStdString_0<RetType> {
  fn toStdString_0(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_toStdString_0<i32> for () {
  fn toStdString_0(self , rsthis: & QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString11toStdStringB5cxx11Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:782
// index:0
// Public inline Visibility=Default Availability=Available
// [32] std::wstring toStdWString() const

/*
Returns a std::wstring object with the data contained in this QString. The std::wstring is encoded in utf16 on platforms where wchar_t is 2 bytes wide (e.g. windows) and in ucs4 on platforms where wchar_t is 4 bytes wide (most Unix systems).

This method is mostly useful to pass a QString to a function that accepts a std::wstring object.

See also utf16(), toLatin1(), toUtf8(), toLocal8Bit(), toStdU16String(), and toStdU32String().
*/
impl /*struct*/ QString {
  pub fn toStdWString_0<RetType, T: QString_toStdWString_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toStdWString_0(self);
    // return 1;
  }
}
pub trait QString_toStdWString_0<RetType> {
  fn toStdWString_0(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_toStdWString_0<i32> for () {
  fn toStdWString_0(self , rsthis: & QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString12toStdWStringB5cxx11Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:786
// index:0
// Public inline Visibility=Default Availability=Available
// [32] std::u16string toStdU16String() const

/*
Returns a std::u16string object with the data contained in this QString. The Unicode data is the same as returned by the utf16() method.

This function was introduced in  Qt 5.5.

See also utf16(), toStdWString(), and toStdU32String().
*/
impl /*struct*/ QString {
  pub fn toStdU16String_0<RetType, T: QString_toStdU16String_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toStdU16String_0(self);
    // return 1;
  }
}
pub trait QString_toStdU16String_0<RetType> {
  fn toStdU16String_0(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_toStdU16String_0<i32> for () {
  fn toStdU16String_0(self , rsthis: & QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString14toStdU16StringB5cxx11Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:788
// index:0
// Public inline Visibility=Default Availability=Available
// [32] std::u32string toStdU32String() const

/*
Returns a std::u32string object with the data contained in this QString. The Unicode data is the same as returned by the toUcs4() method.

This function was introduced in  Qt 5.5.

See also toUcs4(), toStdWString(), and toStdU16String().
*/
impl /*struct*/ QString {
  pub fn toStdU32String_0<RetType, T: QString_toStdU32String_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toStdU32String_0(self);
    // return 1;
  }
}
pub trait QString_toStdU32String_0<RetType> {
  fn toStdU32String_0(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_toStdU32String_0<i32> for () {
  fn toStdU32String_0(self , rsthis: & QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString14toStdU32StringB5cxx11Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:805
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
impl /*struct*/ QString {
  pub fn isNull_0<RetType, T: QString_isNull_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isNull_0(self);
    // return 1;
  }
}
pub trait QString_isNull_0<RetType> {
  fn isNull_0(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_isNull_0<bool> for () {
  fn isNull_0(self , rsthis: & QString) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString6isNullEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:808
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isSimpleText() const

/*

*/
impl /*struct*/ QString {
  pub fn isSimpleText_0<RetType, T: QString_isSimpleText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isSimpleText_0(self);
    // return 1;
  }
}
pub trait QString_isSimpleText_0<RetType> {
  fn isSimpleText_0(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_isSimpleText_0<bool> for () {
  fn isSimpleText_0(self , rsthis: & QString) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString12isSimpleTextEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:809
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isRightToLeft() const

/*
Returns true if the string is read right to left.

See also QStringRef::isRightToLeft().
*/
impl /*struct*/ QString {
  pub fn isRightToLeft_0<RetType, T: QString_isRightToLeft_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isRightToLeft_0(self);
    // return 1;
  }
}
pub trait QString_isRightToLeft_0<RetType> {
  fn isRightToLeft_0(self , rsthis: & QString) -> RetType;
}
impl<'a> /*trait*/ QString_isRightToLeft_0<bool> for () {
  fn isRightToLeft_0(self , rsthis: & QString) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QString13isRightToLeftEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}


/*


*/
pub type QString__SectionFlag = i32;
// 
pub const QString__SectionDefault :QString__SectionFlag = 0;
// 
pub const QString__SectionSkipEmpty :QString__SectionFlag = 1;
// 
pub const QString__SectionIncludeLeadingSep :QString__SectionFlag = 2;
// 
pub const QString__SectionIncludeTrailingSep :QString__SectionFlag = 4;
// 
pub const QString__SectionCaseInsensitiveSeps :QString__SectionFlag = 8;
pub fn QString_SectionFlagItemName(val: i32) ->String {
  match val {
     QString__SectionDefault => // 0
     {return String::from("SectionDefault");}
     QString__SectionSkipEmpty => // 1
     {return String::from("SectionSkipEmpty");}
     QString__SectionIncludeLeadingSep => // 2
     {return String::from("SectionIncludeLeadingSep");}
     QString__SectionIncludeTrailingSep => // 4
     {return String::from("SectionIncludeTrailingSep");}
     QString__SectionCaseInsensitiveSeps => // 8
     {return String::from("SectionCaseInsensitiveSeps");}
  _ => {return format!("{}", val);}
}
}
pub fn QString_SectionFlagItemName_s(val: i32) ->String {
  //var nilthis *QString
  //return nilthis.SectionFlagItemName(val);
  return QString_SectionFlagItemName(val);
}


/*
This enum specifies how the split() function should behave with respect to empty strings.



See also split().

*/
pub type QString__SplitBehavior = i32;
// If a field is empty, keep it in the result.
pub const QString__KeepEmptyParts :QString__SplitBehavior = 0;
// If a field is empty, don't include it in the result.
pub const QString__SkipEmptyParts :QString__SplitBehavior = 1;
pub fn QString_SplitBehaviorItemName(val: i32) ->String {
  match val {
     QString__KeepEmptyParts => // 0
     {return String::from("KeepEmptyParts");}
     QString__SkipEmptyParts => // 1
     {return String::from("SkipEmptyParts");}
  _ => {return format!("{}", val);}
}
}
pub fn QString_SplitBehaviorItemName_s(val: i32) ->String {
  //var nilthis *QString
  //return nilthis.SplitBehaviorItemName(val);
  return QString_SplitBehaviorItemName(val);
}


/*
This enum describes the various normalized forms of Unicode text.



See also normalized() and Unicode Standard Annex #15.

*/
pub type QString__NormalizationForm = i32;
// Canonical Decomposition
pub const QString__NormalizationForm_D :QString__NormalizationForm = 0;
// Canonical Decomposition followed by Canonical Composition
pub const QString__NormalizationForm_C :QString__NormalizationForm = 1;
// Compatibility Decomposition
pub const QString__NormalizationForm_KD :QString__NormalizationForm = 2;
// Compatibility Decomposition followed by Canonical Composition
pub const QString__NormalizationForm_KC :QString__NormalizationForm = 3;
pub fn QString_NormalizationFormItemName(val: i32) ->String {
  match val {
     QString__NormalizationForm_D => // 0
     {return String::from("NormalizationForm_D");}
     QString__NormalizationForm_C => // 1
     {return String::from("NormalizationForm_C");}
     QString__NormalizationForm_KD => // 2
     {return String::from("NormalizationForm_KD");}
     QString__NormalizationForm_KC => // 3
     {return String::from("NormalizationForm_KC");}
  _ => {return format!("{}", val);}
}
}
pub fn QString_NormalizationFormItemName_s(val: i32) ->String {
  //var nilthis *QString
  //return nilthis.NormalizationFormItemName(val);
  return QString_NormalizationFormItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
