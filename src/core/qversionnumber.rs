

// mod ::core::QVersionNumber
// package qtcore
// /usr/include/qt/QtCore/qversionnumber.h
// #include <qversionnumber.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 21
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
#[derive(Default)] // class sizeof(QVersionNumber)=8
pub struct QVersionNumber {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QVersionNumber_ITF interface {
//    QVersionNumber_PTR() *QVersionNumber
//}
//func (ptr *QVersionNumber) QVersionNumber_PTR() *QVersionNumber { return ptr }

impl /*struct*/ QVersionNumber {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QVersionNumber {
    return QVersionNumber{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QVersionNumber {
//  type Target = QVersionNumberBASE;
//
//  fn deref(&self) -> &QVersionNumberBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QVersionNumberBASE> for QVersionNumber {
//  fn as_ref(& self) -> & QVersionNumberBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qversionnumber.h:221
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void QVersionNumber()

/*
Produces a null version.

See also isNull().
*/
// QVersionNumber() ctx.fn_proto_cpp
impl /*struct*/ QVersionNumber {
  pub fn QVersionNumber_0<T: QVersionNumber_QVersionNumber_0>(value: T) -> QVersionNumber {
    let rsthis = value.QVersionNumber_0();
    return rsthis;
    // return 1;
  }
}

pub trait QVersionNumber_QVersionNumber_0 {
  fn QVersionNumber_0(self) -> QVersionNumber;
}
// QVersionNumber() ctx.fn_proto_cpp
impl<'a> /*trait*/ QVersionNumber_QVersionNumber_0 for () {
  fn QVersionNumber_0(self) -> QVersionNumber {
    // unsafe{_ZN14QVersionNumberC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN14QVersionNumberC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QVersionNumber{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qversionnumber.h:242
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void QVersionNumber(int)

/*
Produces a null version.

See also isNull().
*/
// QVersionNumber(int) ctx.fn_proto_cpp
impl /*struct*/ QVersionNumber {
  pub fn QVersionNumber_1<T: QVersionNumber_QVersionNumber_1>(value: T) -> QVersionNumber {
    let rsthis = value.QVersionNumber_1();
    return rsthis;
    // return 1;
  }
}

pub trait QVersionNumber_QVersionNumber_1 {
  fn QVersionNumber_1(self) -> QVersionNumber;
}
// QVersionNumber(int) ctx.fn_proto_cpp
impl<'a> /*trait*/ QVersionNumber_QVersionNumber_1 for (i32) {
  fn QVersionNumber_1(self) -> QVersionNumber {
    // unsafe{_ZN14QVersionNumberC2Ei()};
    let arg0 = (&self) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN14QVersionNumberC2Ei", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QVersionNumber{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qversionnumber.h:245
// index:2
// Public inline Visibility=Default Availability=Available
// [-2] void QVersionNumber(int, int)

/*
Produces a null version.

See also isNull().
*/
// QVersionNumber(int, int) ctx.fn_proto_cpp
impl /*struct*/ QVersionNumber {
  pub fn QVersionNumber_2<T: QVersionNumber_QVersionNumber_2>(value: T) -> QVersionNumber {
    let rsthis = value.QVersionNumber_2();
    return rsthis;
    // return 1;
  }
}

pub trait QVersionNumber_QVersionNumber_2 {
  fn QVersionNumber_2(self) -> QVersionNumber;
}
// QVersionNumber(int, int) ctx.fn_proto_cpp
impl<'a> /*trait*/ QVersionNumber_QVersionNumber_2 for (i32,i32) {
  fn QVersionNumber_2(self) -> QVersionNumber {
    // unsafe{_ZN14QVersionNumberC2Eii()};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN14QVersionNumberC2Eii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QVersionNumber{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qversionnumber.h:248
// index:3
// Public inline Visibility=Default Availability=Available
// [-2] void QVersionNumber(int, int, int)

/*
Produces a null version.

See also isNull().
*/
// QVersionNumber(int, int, int) ctx.fn_proto_cpp
impl /*struct*/ QVersionNumber {
  pub fn QVersionNumber_3<T: QVersionNumber_QVersionNumber_3>(value: T) -> QVersionNumber {
    let rsthis = value.QVersionNumber_3();
    return rsthis;
    // return 1;
  }
}

pub trait QVersionNumber_QVersionNumber_3 {
  fn QVersionNumber_3(self) -> QVersionNumber;
}
// QVersionNumber(int, int, int) ctx.fn_proto_cpp
impl<'a> /*trait*/ QVersionNumber_QVersionNumber_3 for (i32,i32,i32) {
  fn QVersionNumber_3(self) -> QVersionNumber {
    // unsafe{_ZN14QVersionNumberC2Eiii()};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN14QVersionNumberC2Eiii", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QVersionNumber{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qversionnumber.h:251
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isNull() const

/*
Returns true if there are zero numerical segments, otherwise returns false.

See also segments().
*/
impl /*struct*/ QVersionNumber {
  pub fn isNull_0<RetType, T: QVersionNumber_isNull_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isNull_0(self);
    // return 1;
  }
}
pub trait QVersionNumber_isNull_0<RetType> {
  fn isNull_0(self , rsthis: & QVersionNumber) -> RetType;
}
impl<'a> /*trait*/ QVersionNumber_isNull_0<bool> for () {
  fn isNull_0(self , rsthis: & QVersionNumber) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QVersionNumber6isNullEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qversionnumber.h:254
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isNormalized() const

/*
Returns true if the version number does not contain any trailing zeros, otherwise returns false.

See also normalized().
*/
impl /*struct*/ QVersionNumber {
  pub fn isNormalized_0<RetType, T: QVersionNumber_isNormalized_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isNormalized_0(self);
    // return 1;
  }
}
pub trait QVersionNumber_isNormalized_0<RetType> {
  fn isNormalized_0(self , rsthis: & QVersionNumber) -> RetType;
}
impl<'a> /*trait*/ QVersionNumber_isNormalized_0<bool> for () {
  fn isNormalized_0(self , rsthis: & QVersionNumber) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QVersionNumber12isNormalizedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qversionnumber.h:257
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int majorVersion() const

/*
Returns the major version number, that is, the first segment. This function is equivalent to segmentAt(0). If this QVersionNumber object is null, this function returns 0.

See also isNull() and segmentAt().
*/
impl /*struct*/ QVersionNumber {
  pub fn majorVersion_0<RetType, T: QVersionNumber_majorVersion_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.majorVersion_0(self);
    // return 1;
  }
}
pub trait QVersionNumber_majorVersion_0<RetType> {
  fn majorVersion_0(self , rsthis: & QVersionNumber) -> RetType;
}
impl<'a> /*trait*/ QVersionNumber_majorVersion_0<i32> for () {
  fn majorVersion_0(self , rsthis: & QVersionNumber) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QVersionNumber12majorVersionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qversionnumber.h:260
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int minorVersion() const

/*
Returns the minor version number, that is, the second segment. This function is equivalent to segmentAt(1). If this QVersionNumber object does not contain a minor number, this function returns 0.

See also isNull() and segmentAt().
*/
impl /*struct*/ QVersionNumber {
  pub fn minorVersion_0<RetType, T: QVersionNumber_minorVersion_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.minorVersion_0(self);
    // return 1;
  }
}
pub trait QVersionNumber_minorVersion_0<RetType> {
  fn minorVersion_0(self , rsthis: & QVersionNumber) -> RetType;
}
impl<'a> /*trait*/ QVersionNumber_minorVersion_0<i32> for () {
  fn minorVersion_0(self , rsthis: & QVersionNumber) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QVersionNumber12minorVersionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qversionnumber.h:263
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int microVersion() const

/*
Returns the micro version number, that is, the third segment. This function is equivalent to segmentAt(2). If this QVersionNumber object does not contain a micro number, this function returns 0.

See also isNull() and segmentAt().
*/
impl /*struct*/ QVersionNumber {
  pub fn microVersion_0<RetType, T: QVersionNumber_microVersion_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.microVersion_0(self);
    // return 1;
  }
}
pub trait QVersionNumber_microVersion_0<RetType> {
  fn microVersion_0(self , rsthis: & QVersionNumber) -> RetType;
}
impl<'a> /*trait*/ QVersionNumber_microVersion_0<i32> for () {
  fn microVersion_0(self , rsthis: & QVersionNumber) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QVersionNumber12microVersionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qversionnumber.h:266
// index:0
// Public Visibility=Default Availability=Available
// [8] QVersionNumber normalized() const

/*
Returns an equivalent version number but with all trailing zeros removed.

To check if two numbers are equivalent, use normalized() on both version numbers before performing the compare.


  QVersionNumber v1(5, 4);
  QVersionNumber v2(5, 4, 0);
  bool equivalent = v1.normalized() == v2.normalized();
  bool equal = v1 == v2;
  // equivalent is true
  // equal is false
*/
impl /*struct*/ QVersionNumber {
  pub fn normalized_0<RetType, T: QVersionNumber_normalized_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.normalized_0(self);
    // return 1;
  }
}
pub trait QVersionNumber_normalized_0<RetType> {
  fn normalized_0(self , rsthis: & QVersionNumber) -> RetType;
}
impl<'a> /*trait*/ QVersionNumber_normalized_0<usize> for () {
  fn normalized_0(self , rsthis: & QVersionNumber) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QVersionNumber10normalizedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qversionnumber.h:270
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int segmentAt(int) const

/*
Returns the segement value at index. If the index does not exist, returns 0.

See also segments() and segmentCount().
*/
impl /*struct*/ QVersionNumber {
  pub fn segmentAt_0<RetType, T: QVersionNumber_segmentAt_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.segmentAt_0(self);
    // return 1;
  }
}
pub trait QVersionNumber_segmentAt_0<RetType> {
  fn segmentAt_0(self , rsthis: & QVersionNumber) -> RetType;
}
impl<'a> /*trait*/ QVersionNumber_segmentAt_0<i32> for (i32) {
  fn segmentAt_0(self , rsthis: & QVersionNumber) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QVersionNumber9segmentAtEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qversionnumber.h:273
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int segmentCount() const

/*
Returns the number of integers stored in segments().

See also segments().
*/
impl /*struct*/ QVersionNumber {
  pub fn segmentCount_0<RetType, T: QVersionNumber_segmentCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.segmentCount_0(self);
    // return 1;
  }
}
pub trait QVersionNumber_segmentCount_0<RetType> {
  fn segmentCount_0(self , rsthis: & QVersionNumber) -> RetType;
}
impl<'a> /*trait*/ QVersionNumber_segmentCount_0<i32> for () {
  fn segmentCount_0(self , rsthis: & QVersionNumber) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QVersionNumber12segmentCountEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qversionnumber.h:276
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isPrefixOf(const QVersionNumber &) const

/*
Returns true if the current version number is contained in the other version number, otherwise returns false.


  QVersionNumber v1(5, 3);
  QVersionNumber v2(5, 3, 1);
  bool value = v1.isPrefixOf(v2); // true



See also commonPrefix().
*/
impl /*struct*/ QVersionNumber {
  pub fn isPrefixOf_0<RetType, T: QVersionNumber_isPrefixOf_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isPrefixOf_0(self);
    // return 1;
  }
}
pub trait QVersionNumber_isPrefixOf_0<RetType> {
  fn isPrefixOf_0(self , rsthis: & QVersionNumber) -> RetType;
}
impl<'a> /*trait*/ QVersionNumber_isPrefixOf_0<bool> for (usize) {
  fn isPrefixOf_0(self , rsthis: & QVersionNumber) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QVersionNumber10isPrefixOfERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qversionnumber.h:278
// index:0
// Public static Visibility=Default Availability=Available
// [4] int compare(const QVersionNumber &, const QVersionNumber &)

/*
Compares v1 with v2 and returns an integer less than, equal to, or greater than zero, depending on whether v1 is less than, equal to, or greater than v2, respectively.

Comparisons are performed by comparing the segments of v1 and v2 starting at index 0 and working towards the end of the longer list.


  QVersionNumber v1(1, 2);
  QVersionNumber v2(1, 2, 0);
  int compare = QVersionNumber::compare(v1, v2); // compare == -1
*/
impl /*struct*/ QVersionNumber {
  pub fn compare_0<RetType, T: QVersionNumber_compare_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.compare_0();
    // return 1;
  }
}
pub trait QVersionNumber_compare_0<RetType> {
  fn compare_0(self ) -> RetType;
}
impl<'a> /*trait*/ QVersionNumber_compare_0<i32> for (usize,usize) {
  fn compare_0(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN14QVersionNumber7compareERKS_S1_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qversionnumber.h:280
// index:0
// Public static Visibility=Default Availability=Available
// [8] QVersionNumber commonPrefix(const QVersionNumber &, const QVersionNumber &)

/*
QVersionNumber QVersionNumber::commonPrefix(const QVersionNumber &v1, const QVersionNumber &v2)

Returns a version number that is a parent version of both v1 and v2.

See also isPrefixOf().
*/
impl /*struct*/ QVersionNumber {
  pub fn commonPrefix_0<RetType, T: QVersionNumber_commonPrefix_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.commonPrefix_0();
    // return 1;
  }
}
pub trait QVersionNumber_commonPrefix_0<RetType> {
  fn commonPrefix_0(self ) -> RetType;
}
impl<'a> /*trait*/ QVersionNumber_commonPrefix_0<usize> for (usize,usize) {
  fn commonPrefix_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN14QVersionNumber12commonPrefixERKS_S1_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qversionnumber.h:282
// index:0
// Public Visibility=Default Availability=Available
// [8] QString toString() const

/*
Returns a string with all of the segments delimited by a period (.).

See also majorVersion(), minorVersion(), microVersion(), and segments().
*/
impl /*struct*/ QVersionNumber {
  pub fn toString_0<RetType, T: QVersionNumber_toString_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toString_0(self);
    // return 1;
  }
}
pub trait QVersionNumber_toString_0<RetType> {
  fn toString_0(self , rsthis: & QVersionNumber) -> RetType;
}
impl<'a> /*trait*/ QVersionNumber_toString_0<usize> for () {
  fn toString_0(self , rsthis: & QVersionNumber) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QVersionNumber8toStringEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qversionnumber.h:284
// index:0
// Public static Visibility=Default Availability=Available
// [8] QVersionNumber fromString(const QString &, int *)

/*
Constructs a QVersionNumber from a specially formatted string of non-negative decimal numbers delimited by a period (.).

Once the numerical segments have been parsed, the remainder of the string is considered to be the suffix string. The start index of that string will be stored in suffixIndex if it is not null.


  QString string("5.4.0-alpha");
  int suffixIndex;
  QVersionNumber version = QVersionNumber::fromString(string, &suffixIndex);
  // version is 5.4.0
  // suffixIndex is 5



See also isNull().
*/
impl /*struct*/ QVersionNumber {
  pub fn fromString_0<RetType, T: QVersionNumber_fromString_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromString_0();
    // return 1;
  }
}
pub trait QVersionNumber_fromString_0<RetType> {
  fn fromString_0(self ) -> RetType;
}
impl<'a> /*trait*/ QVersionNumber_fromString_0<usize> for (usize,usize) {
  fn fromString_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN14QVersionNumber10fromStringERK7QStringPi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qversionnumber.h:286
// index:1
// Public static Visibility=Default Availability=Available
// [8] QVersionNumber fromString(QLatin1String, int *)

/*
Constructs a QVersionNumber from a specially formatted string of non-negative decimal numbers delimited by a period (.).

Once the numerical segments have been parsed, the remainder of the string is considered to be the suffix string. The start index of that string will be stored in suffixIndex if it is not null.


  QString string("5.4.0-alpha");
  int suffixIndex;
  QVersionNumber version = QVersionNumber::fromString(string, &suffixIndex);
  // version is 5.4.0
  // suffixIndex is 5



See also isNull().
*/
impl /*struct*/ QVersionNumber {
  pub fn fromString_1<RetType, T: QVersionNumber_fromString_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromString_1();
    // return 1;
  }
}
pub trait QVersionNumber_fromString_1<RetType> {
  fn fromString_1(self ) -> RetType;
}
impl<'a> /*trait*/ QVersionNumber_fromString_1<usize> for (usize,usize) {
  fn fromString_1(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN14QVersionNumber10fromStringE13QLatin1StringPi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qversionnumber.h:287
// index:2
// Public static Visibility=Default Availability=Available
// [8] QVersionNumber fromString(QStringView, int *)

/*
Constructs a QVersionNumber from a specially formatted string of non-negative decimal numbers delimited by a period (.).

Once the numerical segments have been parsed, the remainder of the string is considered to be the suffix string. The start index of that string will be stored in suffixIndex if it is not null.


  QString string("5.4.0-alpha");
  int suffixIndex;
  QVersionNumber version = QVersionNumber::fromString(string, &suffixIndex);
  // version is 5.4.0
  // suffixIndex is 5



See also isNull().
*/
impl /*struct*/ QVersionNumber {
  pub fn fromString_2<RetType, T: QVersionNumber_fromString_2<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromString_2();
    // return 1;
  }
}
pub trait QVersionNumber_fromString_2<RetType> {
  fn fromString_2(self ) -> RetType;
}
impl<'a> /*trait*/ QVersionNumber_fromString_2<usize> for (usize,usize) {
  fn fromString_2(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN14QVersionNumber10fromStringE11QStringViewPi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}


pub fn DeleteQVersionNumber(this :*mut QVersionNumber) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN14QVersionNumberD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}

/*


*/
pub type QVersionNumber__ = i32;
// 
pub const QVersionNumber__InlineSegmentMarker :QVersionNumber__ = 0;
// 
pub const QVersionNumber__InlineSegmentStartIdx :QVersionNumber__ = 1;
// 
pub const QVersionNumber__InlineSegmentCount :QVersionNumber__ = 7;
pub fn QVersionNumber_ItemName(val: i32) ->String {
  match val {
     QVersionNumber__InlineSegmentMarker => // 0
     {return String::from("InlineSegmentMarker");}
     QVersionNumber__InlineSegmentStartIdx => // 1
     {return String::from("InlineSegmentStartIdx");}
     QVersionNumber__InlineSegmentCount => // 7
     {return String::from("InlineSegmentCount");}
  _ => {return format!("{}", val);}
}
}
pub fn QVersionNumber_ItemName_s(val: i32) ->String {
  //var nilthis *QVersionNumber
  //return nilthis.ItemName(val);
  return QVersionNumber_ItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
