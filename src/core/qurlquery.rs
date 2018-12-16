

// mod ::core::QUrlQuery
// package qtcore
// /usr/include/qt/QtCore/qurlquery.h
// #include <qurlquery.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 48
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
#[derive(Default)] // class sizeof(QUrlQuery)=8
pub struct QUrlQuery {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QUrlQuery_ITF interface {
//    QUrlQuery_PTR() *QUrlQuery
//}
//func (ptr *QUrlQuery) QUrlQuery_PTR() *QUrlQuery { return ptr }

impl /*struct*/ QUrlQuery {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QUrlQuery {
    return QUrlQuery{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QUrlQuery {
//  type Target = QUrlQueryBASE;
//
//  fn deref(&self) -> &QUrlQueryBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QUrlQueryBASE> for QUrlQuery {
//  fn as_ref(& self) -> & QUrlQueryBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qurlquery.h:59
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QUrlQuery()

/*
Constructs an empty QUrlQuery object. A query can be set afterwards by calling setQuery() or items can be added by using addQueryItem().

See also setQuery() and addQueryItem().
*/
// QUrlQuery() ctx.fn_proto_cpp
impl /*struct*/ QUrlQuery {
  pub fn QUrlQuery_0<T: QUrlQuery_QUrlQuery_0>(value: T) -> QUrlQuery {
    let rsthis = value.QUrlQuery_0();
    return rsthis;
    // return 1;
  }
}

pub trait QUrlQuery_QUrlQuery_0 {
  fn QUrlQuery_0(self) -> QUrlQuery;
}
// QUrlQuery() ctx.fn_proto_cpp
impl<'a> /*trait*/ QUrlQuery_QUrlQuery_0 for () {
  fn QUrlQuery_0(self) -> QUrlQuery {
    // unsafe{_ZN9QUrlQueryC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QUrlQueryC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QUrlQuery{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qurlquery.h:60
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QUrlQuery(const QUrl &)

/*
Constructs an empty QUrlQuery object. A query can be set afterwards by calling setQuery() or items can be added by using addQueryItem().

See also setQuery() and addQueryItem().
*/
// QUrlQuery(const QUrl &) ctx.fn_proto_cpp
impl /*struct*/ QUrlQuery {
  pub fn QUrlQuery_1<T: QUrlQuery_QUrlQuery_1>(value: T) -> QUrlQuery {
    let rsthis = value.QUrlQuery_1();
    return rsthis;
    // return 1;
  }
}

pub trait QUrlQuery_QUrlQuery_1 {
  fn QUrlQuery_1(self) -> QUrlQuery;
}
// QUrlQuery(const QUrl &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QUrlQuery_QUrlQuery_1 for (usize) {
  fn QUrlQuery_1(self) -> QUrlQuery {
    // unsafe{_ZN9QUrlQueryC2ERK4QUrl()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QUrlQueryC2ERK4QUrl", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QUrlQuery{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qurlquery.h:61
// index:2
// Public Visibility=Default Availability=Available
// [-2] void QUrlQuery(const QString &)

/*
Constructs an empty QUrlQuery object. A query can be set afterwards by calling setQuery() or items can be added by using addQueryItem().

See also setQuery() and addQueryItem().
*/
// QUrlQuery(const QString &) ctx.fn_proto_cpp
impl /*struct*/ QUrlQuery {
  pub fn QUrlQuery_2<T: QUrlQuery_QUrlQuery_2>(value: T) -> QUrlQuery {
    let rsthis = value.QUrlQuery_2();
    return rsthis;
    // return 1;
  }
}

pub trait QUrlQuery_QUrlQuery_2 {
  fn QUrlQuery_2(self) -> QUrlQuery;
}
// QUrlQuery(const QString &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QUrlQuery_QUrlQuery_2 for (usize) {
  fn QUrlQuery_2(self) -> QUrlQuery {
    // unsafe{_ZN9QUrlQueryC2ERK7QString()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QUrlQueryC2ERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QUrlQuery{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qurlquery.h:63
// index:0
// Public Visibility=Default Availability=Available
// [8] QUrlQuery & operator=(const QUrlQuery &)

/*

*/
impl /*struct*/ QUrlQuery {
  pub fn operator_equal_0<RetType, T: QUrlQuery_operator_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_0(self);
    // return 1;
  }
}
pub trait QUrlQuery_operator_equal_0<RetType> {
  fn operator_equal_0(self , rsthis: & QUrlQuery) -> RetType;
}
impl<'a> /*trait*/ QUrlQuery_operator_equal_0<usize> for (usize) {
  fn operator_equal_0(self , rsthis: & QUrlQuery) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QUrlQueryaSERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qurlquery.h:65
// index:1
// Public inline Visibility=Default Availability=Available
// [8] QUrlQuery & operator=(QUrlQuery &&)

/*

*/
impl /*struct*/ QUrlQuery {
  pub fn operator_equal_1<RetType, T: QUrlQuery_operator_equal_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_1(self);
    // return 1;
  }
}
pub trait QUrlQuery_operator_equal_1<RetType> {
  fn operator_equal_1(self , rsthis: & QUrlQuery) -> RetType;
}
impl<'a> /*trait*/ QUrlQuery_operator_equal_1<usize> for (usize) {
  fn operator_equal_1(self , rsthis: & QUrlQuery) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QUrlQueryaSEOS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qurlquery.h:67
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ~QUrlQuery()

/*

*/
pub fn DeleteQUrlQuery(this :*mut QUrlQuery) {
    // let rv = qtrt::InvokeQtFunc6("_ZN9QUrlQueryD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 8)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qurlquery.h:69
// index:0
// Public Visibility=Default Availability=Available
// [1] bool operator==(const QUrlQuery &) const

/*

*/
impl /*struct*/ QUrlQuery {
  pub fn operator_equal_equal_0<RetType, T: QUrlQuery_operator_equal_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_equal_0(self);
    // return 1;
  }
}
pub trait QUrlQuery_operator_equal_equal_0<RetType> {
  fn operator_equal_equal_0(self , rsthis: & QUrlQuery) -> RetType;
}
impl<'a> /*trait*/ QUrlQuery_operator_equal_equal_0<bool> for (usize) {
  fn operator_equal_equal_0(self , rsthis: & QUrlQuery) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QUrlQueryeqERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qurlquery.h:70
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator!=(const QUrlQuery &) const

/*

*/
impl /*struct*/ QUrlQuery {
  pub fn operator_not_equal_0<RetType, T: QUrlQuery_operator_not_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_not_equal_0(self);
    // return 1;
  }
}
pub trait QUrlQuery_operator_not_equal_0<RetType> {
  fn operator_not_equal_0(self , rsthis: & QUrlQuery) -> RetType;
}
impl<'a> /*trait*/ QUrlQuery_operator_not_equal_0<bool> for (usize) {
  fn operator_not_equal_0(self , rsthis: & QUrlQuery) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QUrlQueryneERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qurlquery.h:73
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void swap(QUrlQuery &)

/*
Swaps this URL query instance with other. This function is very fast and never fails.
*/
impl /*struct*/ QUrlQuery {
  pub fn swap_0<RetType, T: QUrlQuery_swap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.swap_0(self);
    // return 1;
  }
}
pub trait QUrlQuery_swap_0<RetType> {
  fn swap_0(self , rsthis: & QUrlQuery) -> RetType;
}
impl<'a> /*trait*/ QUrlQuery_swap_0<(/*void*/)> for (usize) {
  fn swap_0(self , rsthis: & QUrlQuery) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QUrlQuery4swapERS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qurlquery.h:75
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isEmpty() const

/*
Returns true if this QUrlQuery object contains no key-value pairs, such as after being default-constructed or after parsing an empty query string.

See also setQuery() and clear().
*/
impl /*struct*/ QUrlQuery {
  pub fn isEmpty_0<RetType, T: QUrlQuery_isEmpty_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isEmpty_0(self);
    // return 1;
  }
}
pub trait QUrlQuery_isEmpty_0<RetType> {
  fn isEmpty_0(self , rsthis: & QUrlQuery) -> RetType;
}
impl<'a> /*trait*/ QUrlQuery_isEmpty_0<bool> for () {
  fn isEmpty_0(self , rsthis: & QUrlQuery) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QUrlQuery7isEmptyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qurlquery.h:76
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isDetached() const

/*

*/
impl /*struct*/ QUrlQuery {
  pub fn isDetached_0<RetType, T: QUrlQuery_isDetached_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isDetached_0(self);
    // return 1;
  }
}
pub trait QUrlQuery_isDetached_0<RetType> {
  fn isDetached_0(self , rsthis: & QUrlQuery) -> RetType;
}
impl<'a> /*trait*/ QUrlQuery_isDetached_0<bool> for () {
  fn isDetached_0(self , rsthis: & QUrlQuery) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QUrlQuery10isDetachedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qurlquery.h:77
// index:0
// Public Visibility=Default Availability=Available
// [-2] void clear()

/*
Clears this QUrlQuery object by removing all of the key-value pairs currently stored. If the query delimiters have been changed, this function will leave them with their changed values.

See also isEmpty() and setQueryDelimiters().
*/
impl /*struct*/ QUrlQuery {
  pub fn clear_0<RetType, T: QUrlQuery_clear_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.clear_0(self);
    // return 1;
  }
}
pub trait QUrlQuery_clear_0<RetType> {
  fn clear_0(self , rsthis: & QUrlQuery) -> RetType;
}
impl<'a> /*trait*/ QUrlQuery_clear_0<(/*void*/)> for () {
  fn clear_0(self , rsthis: & QUrlQuery) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN9QUrlQuery5clearEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qurlquery.h:80
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setQuery(const QString &)

/*
Parses the query string in queryString and sets the internal items to the values found there. If any delimiters have been specified with setQueryDelimiters(), this function will use them instead of the default delimiters to parse the string.

See also query().
*/
impl /*struct*/ QUrlQuery {
  pub fn setQuery_0<RetType, T: QUrlQuery_setQuery_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setQuery_0(self);
    // return 1;
  }
}
pub trait QUrlQuery_setQuery_0<RetType> {
  fn setQuery_0(self , rsthis: & QUrlQuery) -> RetType;
}
impl<'a> /*trait*/ QUrlQuery_setQuery_0<(/*void*/)> for (usize) {
  fn setQuery_0(self , rsthis: & QUrlQuery) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QUrlQuery8setQueryERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qurlquery.h:84
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setQueryDelimiters(QChar, QChar)

/*
Sets the characters used for delimiting between keys and values, and between key-value pairs in the URL's query string. The default value delimiter is '=' and the default pair delimiter is '&'.



valueDelimiter will be used for separating keys from values, and pairDelimiter will be used to separate key-value pairs. Any occurrences of these delimiting characters in the encoded representation of the keys and values of the query string are percent encoded when returned in query().

If valueDelimiter is set to '(' and pairDelimiter is ')', the above query string would instead be represented like this:


  http://www.example.com/cgi-bin/drawgraph.cgi?type(pie)color(green)



Note: Non-standard delimiters should be chosen from among what RFC 3986 calls "sub-delimiters". They are:


  sub-delims    = "!" / "$" / "&" / "'" / "(" / ")"
                / "*" / "+" / "," / ";" / "="



Use of other characters is not supported and may result in unexpected behaviour. This method does not verify that you passed a valid delimiter.

See also queryValueDelimiter() and queryPairDelimiter().
*/
impl /*struct*/ QUrlQuery {
  pub fn setQueryDelimiters_0<RetType, T: QUrlQuery_setQueryDelimiters_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setQueryDelimiters_0(self);
    // return 1;
  }
}
pub trait QUrlQuery_setQueryDelimiters_0<RetType> {
  fn setQueryDelimiters_0(self , rsthis: & QUrlQuery) -> RetType;
}
impl<'a> /*trait*/ QUrlQuery_setQueryDelimiters_0<(/*void*/)> for (usize,usize) {
  fn setQueryDelimiters_0(self , rsthis: & QUrlQuery) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QUrlQuery18setQueryDelimitersE5QCharS0_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qurlquery.h:85
// index:0
// Public Visibility=Default Availability=Available
// [2] QChar queryValueDelimiter() const

/*
Returns the character used to delimit between keys and values when reconstructing the query string in query() or when parsing in setQuery().

See also setQueryDelimiters() and queryPairDelimiter().
*/
impl /*struct*/ QUrlQuery {
  pub fn queryValueDelimiter_0<RetType, T: QUrlQuery_queryValueDelimiter_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.queryValueDelimiter_0(self);
    // return 1;
  }
}
pub trait QUrlQuery_queryValueDelimiter_0<RetType> {
  fn queryValueDelimiter_0(self , rsthis: & QUrlQuery) -> RetType;
}
impl<'a> /*trait*/ QUrlQuery_queryValueDelimiter_0<usize> for () {
  fn queryValueDelimiter_0(self , rsthis: & QUrlQuery) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QUrlQuery19queryValueDelimiterEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qurlquery.h:86
// index:0
// Public Visibility=Default Availability=Available
// [2] QChar queryPairDelimiter() const

/*
Returns the character used to delimit between keys-value pairs when reconstructing the query string in query() or when parsing in setQuery().

See also setQueryDelimiters() and queryValueDelimiter().
*/
impl /*struct*/ QUrlQuery {
  pub fn queryPairDelimiter_0<RetType, T: QUrlQuery_queryPairDelimiter_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.queryPairDelimiter_0(self);
    // return 1;
  }
}
pub trait QUrlQuery_queryPairDelimiter_0<RetType> {
  fn queryPairDelimiter_0(self , rsthis: & QUrlQuery) -> RetType;
}
impl<'a> /*trait*/ QUrlQuery_queryPairDelimiter_0<usize> for () {
  fn queryPairDelimiter_0(self , rsthis: & QUrlQuery) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QUrlQuery18queryPairDelimiterEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qurlquery.h:91
// index:0
// Public Visibility=Default Availability=Available
// [1] bool hasQueryItem(const QString &) const

/*
Returns true if there is a query string pair whose key is equal to key from the URL.

See also addQueryItem() and queryItemValue().
*/
impl /*struct*/ QUrlQuery {
  pub fn hasQueryItem_0<RetType, T: QUrlQuery_hasQueryItem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hasQueryItem_0(self);
    // return 1;
  }
}
pub trait QUrlQuery_hasQueryItem_0<RetType> {
  fn hasQueryItem_0(self , rsthis: & QUrlQuery) -> RetType;
}
impl<'a> /*trait*/ QUrlQuery_hasQueryItem_0<bool> for (usize) {
  fn hasQueryItem_0(self , rsthis: & QUrlQuery) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QUrlQuery12hasQueryItemERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qurlquery.h:92
// index:0
// Public Visibility=Default Availability=Available
// [-2] void addQueryItem(const QString &, const QString &)

/*
Appends the pair key = value to the end of the query string of the URL. This method does not overwrite existing items that might exist with the same key.

Note: This method does not treat spaces (ASCII 0x20) and plus ("+") signs as the same, like HTML forms do. If you need spaces to be represented as plus signs, use actual plus signs.

See also hasQueryItem() and queryItemValue().
*/
impl /*struct*/ QUrlQuery {
  pub fn addQueryItem_0<RetType, T: QUrlQuery_addQueryItem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addQueryItem_0(self);
    // return 1;
  }
}
pub trait QUrlQuery_addQueryItem_0<RetType> {
  fn addQueryItem_0(self , rsthis: & QUrlQuery) -> RetType;
}
impl<'a> /*trait*/ QUrlQuery_addQueryItem_0<(/*void*/)> for (usize,usize) {
  fn addQueryItem_0(self , rsthis: & QUrlQuery) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QUrlQuery12addQueryItemERK7QStringS2_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qurlquery.h:93
// index:0
// Public Visibility=Default Availability=Available
// [-2] void removeQueryItem(const QString &)

/*
Removes the query string pair whose key is equal to key from the URL. If there are multiple items with a key equal to key, it removes the first item in the order they were present in the query string or added with addQueryItem().

See also removeAllQueryItems().
*/
impl /*struct*/ QUrlQuery {
  pub fn removeQueryItem_0<RetType, T: QUrlQuery_removeQueryItem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.removeQueryItem_0(self);
    // return 1;
  }
}
pub trait QUrlQuery_removeQueryItem_0<RetType> {
  fn removeQueryItem_0(self , rsthis: & QUrlQuery) -> RetType;
}
impl<'a> /*trait*/ QUrlQuery_removeQueryItem_0<(/*void*/)> for (usize) {
  fn removeQueryItem_0(self , rsthis: & QUrlQuery) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QUrlQuery15removeQueryItemERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qurlquery.h:96
// index:0
// Public Visibility=Default Availability=Available
// [-2] void removeAllQueryItems(const QString &)

/*
Removes all the query string pairs whose key is equal to key from the URL.

See also removeQueryItem().
*/
impl /*struct*/ QUrlQuery {
  pub fn removeAllQueryItems_0<RetType, T: QUrlQuery_removeAllQueryItems_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.removeAllQueryItems_0(self);
    // return 1;
  }
}
pub trait QUrlQuery_removeAllQueryItems_0<RetType> {
  fn removeAllQueryItems_0(self , rsthis: & QUrlQuery) -> RetType;
}
impl<'a> /*trait*/ QUrlQuery_removeAllQueryItems_0<(/*void*/)> for (usize) {
  fn removeAllQueryItems_0(self , rsthis: & QUrlQuery) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QUrlQuery19removeAllQueryItemsERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qurlquery.h:98
// index:0
// Public static inline Visibility=Default Availability=Available
// [2] QChar defaultQueryValueDelimiter()

/*
Returns the default character for separating keys from values in the query, an equal sign ("=").

See also setQueryDelimiters(), queryValueDelimiter(), and defaultQueryPairDelimiter().
*/
impl /*struct*/ QUrlQuery {
  pub fn defaultQueryValueDelimiter_0<RetType, T: QUrlQuery_defaultQueryValueDelimiter_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.defaultQueryValueDelimiter_0();
    // return 1;
  }
}
pub trait QUrlQuery_defaultQueryValueDelimiter_0<RetType> {
  fn defaultQueryValueDelimiter_0(self ) -> RetType;
}
impl<'a> /*trait*/ QUrlQuery_defaultQueryValueDelimiter_0<usize> for () {
  fn defaultQueryValueDelimiter_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QUrlQuery26defaultQueryValueDelimiterEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qurlquery.h:100
// index:0
// Public static inline Visibility=Default Availability=Available
// [2] QChar defaultQueryPairDelimiter()

/*
Returns the default character for separating keys-value pairs from each other, an ampersand ("&").

See also setQueryDelimiters(), queryPairDelimiter(), and defaultQueryValueDelimiter().
*/
impl /*struct*/ QUrlQuery {
  pub fn defaultQueryPairDelimiter_0<RetType, T: QUrlQuery_defaultQueryPairDelimiter_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.defaultQueryPairDelimiter_0();
    // return 1;
  }
}
pub trait QUrlQuery_defaultQueryPairDelimiter_0<RetType> {
  fn defaultQueryPairDelimiter_0(self ) -> RetType;
}
impl<'a> /*trait*/ QUrlQuery_defaultQueryPairDelimiter_0<usize> for () {
  fn defaultQueryPairDelimiter_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QUrlQuery25defaultQueryPairDelimiterEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
