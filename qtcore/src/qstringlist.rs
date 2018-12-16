

// mod ::core::QStringList
// package qtcore
// /usr/include/qt/QtCore/qstringlist.h
// #include <qstringlist.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 11
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
#[derive(Default)] // class sizeof(QStringList)=8
pub struct QStringList {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QStringList_ITF interface {
//    QStringList_PTR() *QStringList
//}
//func (ptr *QStringList) QStringList_PTR() *QStringList { return ptr }

impl /*struct*/ QStringList {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QStringList {
    return QStringList{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QStringList {
//  type Target = QStringListBASE;
//
//  fn deref(&self) -> &QStringListBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QStringListBASE> for QStringList {
//  fn as_ref(& self) -> & QStringListBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qstringlist.h:105
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void QStringList()

/*
Constructs an empty string list.
*/
// QStringList() ctx.fn_proto_cpp
impl /*struct*/ QStringList {
  pub fn QStringList_0<T: QStringList_QStringList_0>(value: T) -> QStringList {
    let rsthis = value.QStringList_0();
    return rsthis;
    // return 1;
  }
}

pub trait QStringList_QStringList_0 {
  fn QStringList_0(self) -> QStringList;
}
// QStringList() ctx.fn_proto_cpp
impl<'a> /*trait*/ QStringList_QStringList_0 for () {
  fn QStringList_0(self) -> QStringList {
    // unsafe{_ZN11QStringListC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QStringListC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QStringList{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qstringlist.h:106
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void QStringList(const QString &)

/*
Constructs an empty string list.
*/
// QStringList(const QString &) ctx.fn_proto_cpp
impl /*struct*/ QStringList {
  pub fn QStringList_1<T: QStringList_QStringList_1>(value: T) -> QStringList {
    let rsthis = value.QStringList_1();
    return rsthis;
    // return 1;
  }
}

pub trait QStringList_QStringList_1 {
  fn QStringList_1(self) -> QStringList;
}
// QStringList(const QString &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QStringList_QStringList_1 for (usize) {
  fn QStringList_1(self) -> QStringList {
    // unsafe{_ZN11QStringListC2ERK7QString()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QStringListC2ERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QStringList{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qstringlist.h:122
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool contains(const QString &, Qt::CaseSensitivity) const

/*
Returns true if the list contains the string str; otherwise returns false. The search is case insensitive if cs is Qt::CaseInsensitive; the search is case sensitive by default.

See also indexOf(), lastIndexOf(), and QString::contains().
*/
impl /*struct*/ QStringList {
  pub fn contains_0<RetType, T: QStringList_contains_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.contains_0(self);
    // return 1;
  }
}
pub trait QStringList_contains_0<RetType> {
  fn contains_0(self , rsthis: & QStringList) -> RetType;
}
impl<'a> /*trait*/ QStringList_contains_0<bool> for (usize,i32) {
  fn contains_0(self , rsthis: & QStringList) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QStringList8containsERK7QStringN2Qt15CaseSensitivityE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstringlist.h:123
// index:1
// Public inline Visibility=Default Availability=Available
// [1] bool contains(QLatin1String, Qt::CaseSensitivity) const

/*
Returns true if the list contains the string str; otherwise returns false. The search is case insensitive if cs is Qt::CaseInsensitive; the search is case sensitive by default.

See also indexOf(), lastIndexOf(), and QString::contains().
*/
impl /*struct*/ QStringList {
  pub fn contains_1<RetType, T: QStringList_contains_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.contains_1(self);
    // return 1;
  }
}
pub trait QStringList_contains_1<RetType> {
  fn contains_1(self , rsthis: & QStringList) -> RetType;
}
impl<'a> /*trait*/ QStringList_contains_1<bool> for (usize,i32) {
  fn contains_1(self , rsthis: & QStringList) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QStringList8containsE13QLatin1StringN2Qt15CaseSensitivityE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstringlist.h:125
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QStringList operator+(const QStringList &) const

/*

*/
impl /*struct*/ QStringList {
  pub fn operator_add_0<RetType, T: QStringList_operator_add_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_add_0(self);
    // return 1;
  }
}
pub trait QStringList_operator_add_0<RetType> {
  fn operator_add_0(self , rsthis: & QStringList) -> RetType;
}
impl<'a> /*trait*/ QStringList_operator_add_0<usize> for (usize) {
  fn operator_add_0(self , rsthis: & QStringList) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QStringListplERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstringlist.h:127
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QStringList & operator<<(const QString &)

/*

*/
impl /*struct*/ QStringList {
  pub fn operator_left_shift_0<RetType, T: QStringList_operator_left_shift_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_left_shift_0(self);
    // return 1;
  }
}
pub trait QStringList_operator_left_shift_0<RetType> {
  fn operator_left_shift_0(self , rsthis: & QStringList) -> RetType;
}
impl<'a> /*trait*/ QStringList_operator_left_shift_0<usize> for (usize) {
  fn operator_left_shift_0(self , rsthis: & QStringList) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QStringListlsERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstringlist.h:129
// index:1
// Public inline Visibility=Default Availability=Available
// [8] QStringList & operator<<(const QStringList &)

/*

*/
impl /*struct*/ QStringList {
  pub fn operator_left_shift_1<RetType, T: QStringList_operator_left_shift_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_left_shift_1(self);
    // return 1;
  }
}
pub trait QStringList_operator_left_shift_1<RetType> {
  fn operator_left_shift_1(self , rsthis: & QStringList) -> RetType;
}
impl<'a> /*trait*/ QStringList_operator_left_shift_1<usize> for (usize) {
  fn operator_left_shift_1(self , rsthis: & QStringList) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QStringListlsERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstringlist.h:135
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int indexOf(const QRegExp &, int) const

/*
Returns the index position of the first exact match of rx in the list, searching forward from index position from. Returns -1 if no item matched.

By default, this function is case sensitive.

See also lastIndexOf(), contains(), and QRegExp::exactMatch().
*/
impl /*struct*/ QStringList {
  pub fn indexOf_0<RetType, T: QStringList_indexOf_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.indexOf_0(self);
    // return 1;
  }
}
pub trait QStringList_indexOf_0<RetType> {
  fn indexOf_0(self , rsthis: & QStringList) -> RetType;
}
impl<'a> /*trait*/ QStringList_indexOf_0<i32> for (usize,i32) {
  fn indexOf_0(self , rsthis: & QStringList) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QStringList7indexOfERK7QRegExpi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstringlist.h:137
// index:1
// Public inline Visibility=Default Availability=Available
// [4] int indexOf(QRegExp &, int) const

/*
Returns the index position of the first exact match of rx in the list, searching forward from index position from. Returns -1 if no item matched.

By default, this function is case sensitive.

See also lastIndexOf(), contains(), and QRegExp::exactMatch().
*/
impl /*struct*/ QStringList {
  pub fn indexOf_1<RetType, T: QStringList_indexOf_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.indexOf_1(self);
    // return 1;
  }
}
pub trait QStringList_indexOf_1<RetType> {
  fn indexOf_1(self , rsthis: & QStringList) -> RetType;
}
impl<'a> /*trait*/ QStringList_indexOf_1<i32> for (usize,i32) {
  fn indexOf_1(self , rsthis: & QStringList) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QStringList7indexOfER7QRegExpi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstringlist.h:143
// index:2
// Public inline Visibility=Default Availability=Available
// [4] int indexOf(const QRegularExpression &, int) const

/*
Returns the index position of the first exact match of rx in the list, searching forward from index position from. Returns -1 if no item matched.

By default, this function is case sensitive.

See also lastIndexOf(), contains(), and QRegExp::exactMatch().
*/
impl /*struct*/ QStringList {
  pub fn indexOf_2<RetType, T: QStringList_indexOf_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.indexOf_2(self);
    // return 1;
  }
}
pub trait QStringList_indexOf_2<RetType> {
  fn indexOf_2(self , rsthis: & QStringList) -> RetType;
}
impl<'a> /*trait*/ QStringList_indexOf_2<i32> for (usize,i32) {
  fn indexOf_2(self , rsthis: & QStringList) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QStringList7indexOfERK18QRegularExpressioni", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstringlist.h:136
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int lastIndexOf(const QRegExp &, int) const

/*
Returns the index position of the last exact match of rx in the list, searching backward from index position from. If from is -1 (the default), the search starts at the last item. Returns -1 if no item matched.

By default, this function is case sensitive.

See also indexOf(), contains(), and QRegExp::exactMatch().
*/
impl /*struct*/ QStringList {
  pub fn lastIndexOf_0<RetType, T: QStringList_lastIndexOf_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.lastIndexOf_0(self);
    // return 1;
  }
}
pub trait QStringList_lastIndexOf_0<RetType> {
  fn lastIndexOf_0(self , rsthis: & QStringList) -> RetType;
}
impl<'a> /*trait*/ QStringList_lastIndexOf_0<i32> for (usize,i32) {
  fn lastIndexOf_0(self , rsthis: & QStringList) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QStringList11lastIndexOfERK7QRegExpi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstringlist.h:138
// index:1
// Public inline Visibility=Default Availability=Available
// [4] int lastIndexOf(QRegExp &, int) const

/*
Returns the index position of the last exact match of rx in the list, searching backward from index position from. If from is -1 (the default), the search starts at the last item. Returns -1 if no item matched.

By default, this function is case sensitive.

See also indexOf(), contains(), and QRegExp::exactMatch().
*/
impl /*struct*/ QStringList {
  pub fn lastIndexOf_1<RetType, T: QStringList_lastIndexOf_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.lastIndexOf_1(self);
    // return 1;
  }
}
pub trait QStringList_lastIndexOf_1<RetType> {
  fn lastIndexOf_1(self , rsthis: & QStringList) -> RetType;
}
impl<'a> /*trait*/ QStringList_lastIndexOf_1<i32> for (usize,i32) {
  fn lastIndexOf_1(self , rsthis: & QStringList) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QStringList11lastIndexOfER7QRegExpi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstringlist.h:144
// index:2
// Public inline Visibility=Default Availability=Available
// [4] int lastIndexOf(const QRegularExpression &, int) const

/*
Returns the index position of the last exact match of rx in the list, searching backward from index position from. If from is -1 (the default), the search starts at the last item. Returns -1 if no item matched.

By default, this function is case sensitive.

See also indexOf(), contains(), and QRegExp::exactMatch().
*/
impl /*struct*/ QStringList {
  pub fn lastIndexOf_2<RetType, T: QStringList_lastIndexOf_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.lastIndexOf_2(self);
    // return 1;
  }
}
pub trait QStringList_lastIndexOf_2<RetType> {
  fn lastIndexOf_2(self , rsthis: & QStringList) -> RetType;
}
impl<'a> /*trait*/ QStringList_lastIndexOf_2<i32> for (usize,i32) {
  fn lastIndexOf_2(self , rsthis: & QStringList) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QStringList11lastIndexOfERK18QRegularExpressioni", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}


pub fn DeleteQStringList(this :*mut QStringList) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN11QStringListD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
