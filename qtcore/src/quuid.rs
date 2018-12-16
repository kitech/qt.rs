

// mod ::core::QUuid
// package qtcore
// /usr/include/qt/QtCore/quuid.h
// #include <quuid.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 22
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
#[derive(Default)] // class sizeof(QUuid)=16
pub struct QUuid {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QUuid_ITF interface {
//    QUuid_PTR() *QUuid
//}
//func (ptr *QUuid) QUuid_PTR() *QUuid { return ptr }

impl /*struct*/ QUuid {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QUuid {
    return QUuid{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QUuid {
//  type Target = QUuidBASE;
//
//  fn deref(&self) -> &QUuidBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QUuidBASE> for QUuid {
//  fn as_ref(& self) -> & QUuidBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/quuid.h:89
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void QUuid()

/*
Creates the null UUID. toString() will output the null UUID as "{00000000-0000-0000-0000-000000000000}".
*/
// QUuid() ctx.fn_proto_cpp
impl /*struct*/ QUuid {
  pub fn QUuid_0<T: QUuid_QUuid_0>(value: T) -> QUuid {
    let rsthis = value.QUuid_0();
    return rsthis;
    // return 1;
  }
}

pub trait QUuid_QUuid_0 {
  fn QUuid_0(self) -> QUuid;
}
// QUuid() ctx.fn_proto_cpp
impl<'a> /*trait*/ QUuid_QUuid_0 for () {
  fn QUuid_0(self) -> QUuid {
    // unsafe{_ZN5QUuidC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN5QUuidC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QUuid{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/quuid.h:91
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void QUuid(uint, ushort, ushort, uchar, uchar, uchar, uchar, uchar, uchar, uchar, uchar)

/*
Creates the null UUID. toString() will output the null UUID as "{00000000-0000-0000-0000-000000000000}".
*/
// QUuid(uint, ushort, ushort, uchar, uchar, uchar, uchar, uchar, uchar, uchar, uchar) ctx.fn_proto_cpp
impl /*struct*/ QUuid {
  pub fn QUuid_1<T: QUuid_QUuid_1>(value: T) -> QUuid {
    let rsthis = value.QUuid_1();
    return rsthis;
    // return 1;
  }
}

pub trait QUuid_QUuid_1 {
  fn QUuid_1(self) -> QUuid;
}
// QUuid(uint, ushort, ushort, uchar, uchar, uchar, uchar, uchar, uchar, uchar, uchar) ctx.fn_proto_cpp
impl<'a> /*trait*/ QUuid_QUuid_1 for (u32,u16,u16,u8,u8,u8,u8,u8,u8,u8,u8) {
  fn QUuid_1(self) -> QUuid {
    // unsafe{_ZN5QUuidC2Ejtthhhhhhhh()};
    let arg0 = (&self.0) as *const u32 as usize;
    let arg1 = (&self.1) as *const u16 as usize;
    let arg2 = (&self.2) as *const u16 as usize;
    let arg3 = (&self.3) as *const u8 as usize;
    let arg4 = (&self.4) as *const u8 as usize;
    let arg5 = (&self.5) as *const u8 as usize;
    let arg6 = (&self.6) as *const u8 as usize;
    let arg7 = (&self.7) as *const u8 as usize;
    let arg8 = (&self.8) as *const u8 as usize;
    let arg9 = (&self.9) as *const u8 as usize;
    let arg10 = (&self.10) as *const u8 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN5QUuidC2Ejtthhhhhhhh", 11,qtrt::FFITY_UINT32,qtrt::FFITY_UINT16,qtrt::FFITY_UINT16,qtrt::FFITY_UINT8,qtrt::FFITY_UINT8,qtrt::FFITY_UINT8,qtrt::FFITY_UINT8,qtrt::FFITY_UINT8,qtrt::FFITY_UINT8,qtrt::FFITY_UINT8,qtrt::FFITY_UINT8,0,0,0,0,arg0,arg1,arg2,arg3,arg4,arg5,arg6,arg7,arg8,arg9,arg10,0,0,0,0);
    let rsthis = QUuid{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/quuid.h:119
// index:2
// Public Visibility=Default Availability=Available
// [-2] void QUuid(const QString &)

/*
Creates the null UUID. toString() will output the null UUID as "{00000000-0000-0000-0000-000000000000}".
*/
// QUuid(const QString &) ctx.fn_proto_cpp
impl /*struct*/ QUuid {
  pub fn QUuid_2<T: QUuid_QUuid_2>(value: T) -> QUuid {
    let rsthis = value.QUuid_2();
    return rsthis;
    // return 1;
  }
}

pub trait QUuid_QUuid_2 {
  fn QUuid_2(self) -> QUuid;
}
// QUuid(const QString &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QUuid_QUuid_2 for (usize) {
  fn QUuid_2(self) -> QUuid {
    // unsafe{_ZN5QUuidC2ERK7QString()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN5QUuidC2ERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QUuid{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/quuid.h:122
// index:3
// Public Visibility=Default Availability=Available
// [-2] void QUuid(const char *)

/*
Creates the null UUID. toString() will output the null UUID as "{00000000-0000-0000-0000-000000000000}".
*/
// QUuid(const char *) ctx.fn_proto_cpp
impl /*struct*/ QUuid {
  pub fn QUuid_3<T: QUuid_QUuid_3>(value: T) -> QUuid {
    let rsthis = value.QUuid_3();
    return rsthis;
    // return 1;
  }
}

pub trait QUuid_QUuid_3 {
  fn QUuid_3(self) -> QUuid;
}
// QUuid(const char *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QUuid_QUuid_3 for (usize) {
  fn QUuid_3(self) -> QUuid {
    // unsafe{_ZN5QUuidC2EPKc()};
    let arg0 = (self) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN5QUuidC2EPKc", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QUuid{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/quuid.h:124
// index:4
// Public Visibility=Default Availability=Available
// [-2] void QUuid(const QByteArray &)

/*
Creates the null UUID. toString() will output the null UUID as "{00000000-0000-0000-0000-000000000000}".
*/
// QUuid(const QByteArray &) ctx.fn_proto_cpp
impl /*struct*/ QUuid {
  pub fn QUuid_4<T: QUuid_QUuid_4>(value: T) -> QUuid {
    let rsthis = value.QUuid_4();
    return rsthis;
    // return 1;
  }
}

pub trait QUuid_QUuid_4 {
  fn QUuid_4(self) -> QUuid;
}
// QUuid(const QByteArray &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QUuid_QUuid_4 for (usize) {
  fn QUuid_4(self) -> QUuid {
    // unsafe{_ZN5QUuidC2ERK10QByteArray()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN5QUuidC2ERK10QByteArray", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QUuid{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/quuid.h:120
// index:0
// Public static Visibility=Default Availability=Available
// [16] QUuid fromString(QStringView)

/*
Creates a QUuid object from the string text, which must be formatted as five hex fields separated by '-', e.g., "{xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx}" where each 'x' is a hex digit. The curly braces shown here are optional, but it is normal to include them. If the conversion fails, a null UUID is returned. See toString() for an explanation of how the five hex fields map to the public data members in QUuid.

This function was introduced in  Qt 5.10.

See also toString() and QUuid().
*/
impl /*struct*/ QUuid {
  pub fn fromString_0<RetType, T: QUuid_fromString_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromString_0();
    // return 1;
  }
}
pub trait QUuid_fromString_0<RetType> {
  fn fromString_0(self ) -> RetType;
}
impl<'a> /*trait*/ QUuid_fromString_0<usize> for (usize) {
  fn fromString_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QUuid10fromStringE11QStringView", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/quuid.h:121
// index:1
// Public static Visibility=Default Availability=Available
// [16] QUuid fromString(QLatin1String)

/*
Creates a QUuid object from the string text, which must be formatted as five hex fields separated by '-', e.g., "{xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx}" where each 'x' is a hex digit. The curly braces shown here are optional, but it is normal to include them. If the conversion fails, a null UUID is returned. See toString() for an explanation of how the five hex fields map to the public data members in QUuid.

This function was introduced in  Qt 5.10.

See also toString() and QUuid().
*/
impl /*struct*/ QUuid {
  pub fn fromString_1<RetType, T: QUuid_fromString_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromString_1();
    // return 1;
  }
}
pub trait QUuid_fromString_1<RetType> {
  fn fromString_1(self ) -> RetType;
}
impl<'a> /*trait*/ QUuid_fromString_1<usize> for (usize) {
  fn fromString_1(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QUuid10fromStringE13QLatin1String", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/quuid.h:123
// index:0
// Public Visibility=Default Availability=Available
// [8] QString toString() const

/*
Returns the string representation of this QUuid. The string is formatted as five hex fields separated by '-' and enclosed in curly braces, i.e., "{xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx}" where 'x' is a hex digit. From left to right, the five hex fields are obtained from the four public data members in QUuid as follows:


 Field #Source
1data1
2data2
3data3
4data4[0] .. data4[1]
5data4[2] .. data4[7]
*/
impl /*struct*/ QUuid {
  pub fn toString_0<RetType, T: QUuid_toString_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toString_0(self);
    // return 1;
  }
}
pub trait QUuid_toString_0<RetType> {
  fn toString_0(self , rsthis: & QUuid) -> RetType;
}
impl<'a> /*trait*/ QUuid_toString_0<usize> for () {
  fn toString_0(self , rsthis: & QUuid) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QUuid8toStringEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/quuid.h:125
// index:0
// Public Visibility=Default Availability=Available
// [8] QByteArray toByteArray() const

/*
Returns the binary representation of this QUuid. The byte array is formatted as five hex fields separated by '-' and enclosed in curly braces, i.e., "{xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx}" where 'x' is a hex digit. From left to right, the five hex fields are obtained from the four public data members in QUuid as follows:


 Field #Source
1data1
2data2
3data3
4data4[0] .. data4[1]
5data4[2] .. data4[7]


This function was introduced in  Qt 4.8.
*/
impl /*struct*/ QUuid {
  pub fn toByteArray_0<RetType, T: QUuid_toByteArray_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toByteArray_0(self);
    // return 1;
  }
}
pub trait QUuid_toByteArray_0<RetType> {
  fn toByteArray_0(self , rsthis: & QUuid) -> RetType;
}
impl<'a> /*trait*/ QUuid_toByteArray_0<usize> for () {
  fn toByteArray_0(self , rsthis: & QUuid) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QUuid11toByteArrayEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/quuid.h:126
// index:0
// Public Visibility=Default Availability=Available
// [8] QByteArray toRfc4122() const

/*
Returns the binary representation of this QUuid. The byte array is in big endian format, and formatted according to RFC 4122, section 4.1.2 - "Layout and byte order".

The order is as follows:


 Field #Source
1data1
2data2
3data3
4data4[0] .. data4[7]


This function was introduced in  Qt 4.8.
*/
impl /*struct*/ QUuid {
  pub fn toRfc4122_0<RetType, T: QUuid_toRfc4122_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toRfc4122_0(self);
    // return 1;
  }
}
pub trait QUuid_toRfc4122_0<RetType> {
  fn toRfc4122_0(self , rsthis: & QUuid) -> RetType;
}
impl<'a> /*trait*/ QUuid_toRfc4122_0<usize> for () {
  fn toRfc4122_0(self , rsthis: & QUuid) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QUuid9toRfc4122Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/quuid.h:127
// index:0
// Public static Visibility=Default Availability=Available
// [16] QUuid fromRfc4122(const QByteArray &)

/*
Creates a QUuid object from the binary representation of the UUID, as specified by RFC 4122 section 4.1.2. See toRfc4122() for a further explanation of the order of bytes required.

The byte array accepted is NOT a human readable format.

If the conversion fails, a null UUID is created.

This function was introduced in  Qt 4.8.

See also toRfc4122() and QUuid().
*/
impl /*struct*/ QUuid {
  pub fn fromRfc4122_0<RetType, T: QUuid_fromRfc4122_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromRfc4122_0();
    // return 1;
  }
}
pub trait QUuid_fromRfc4122_0<RetType> {
  fn fromRfc4122_0(self ) -> RetType;
}
impl<'a> /*trait*/ QUuid_fromRfc4122_0<usize> for (usize) {
  fn fromRfc4122_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QUuid11fromRfc4122ERK10QByteArray", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/quuid.h:128
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isNull() const

/*
Returns true if this is the null UUID {00000000-0000-0000-0000-000000000000}; otherwise returns false.
*/
impl /*struct*/ QUuid {
  pub fn isNull_0<RetType, T: QUuid_isNull_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isNull_0(self);
    // return 1;
  }
}
pub trait QUuid_isNull_0<RetType> {
  fn isNull_0(self , rsthis: & QUuid) -> RetType;
}
impl<'a> /*trait*/ QUuid_isNull_0<bool> for () {
  fn isNull_0(self , rsthis: & QUuid) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QUuid6isNullEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/quuid.h:130
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator==(const QUuid &) const

/*

*/
impl /*struct*/ QUuid {
  pub fn operator_equal_equal_0<RetType, T: QUuid_operator_equal_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_equal_0(self);
    // return 1;
  }
}
pub trait QUuid_operator_equal_equal_0<RetType> {
  fn operator_equal_equal_0(self , rsthis: & QUuid) -> RetType;
}
impl<'a> /*trait*/ QUuid_operator_equal_equal_0<bool> for (usize) {
  fn operator_equal_equal_0(self , rsthis: & QUuid) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QUuideqERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/quuid.h:143
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator!=(const QUuid &) const

/*

*/
impl /*struct*/ QUuid {
  pub fn operator_not_equal_0<RetType, T: QUuid_operator_not_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_not_equal_0(self);
    // return 1;
  }
}
pub trait QUuid_operator_not_equal_0<RetType> {
  fn operator_not_equal_0(self , rsthis: & QUuid) -> RetType;
}
impl<'a> /*trait*/ QUuid_operator_not_equal_0<bool> for (usize) {
  fn operator_not_equal_0(self , rsthis: & QUuid) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QUuidneERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/quuid.h:148
// index:0
// Public Visibility=Default Availability=Available
// [1] bool operator<(const QUuid &) const

/*

*/
impl /*struct*/ QUuid {
  pub fn operator_less_than_0<RetType, T: QUuid_operator_less_than_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_less_than_0(self);
    // return 1;
  }
}
pub trait QUuid_operator_less_than_0<RetType> {
  fn operator_less_than_0(self , rsthis: & QUuid) -> RetType;
}
impl<'a> /*trait*/ QUuid_operator_less_than_0<bool> for (usize) {
  fn operator_less_than_0(self , rsthis: & QUuid) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QUuidltERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/quuid.h:149
// index:0
// Public Visibility=Default Availability=Available
// [1] bool operator>(const QUuid &) const

/*

*/
impl /*struct*/ QUuid {
  pub fn operator_greater_than_0<RetType, T: QUuid_operator_greater_than_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_greater_than_0(self);
    // return 1;
  }
}
pub trait QUuid_operator_greater_than_0<RetType> {
  fn operator_greater_than_0(self , rsthis: & QUuid) -> RetType;
}
impl<'a> /*trait*/ QUuid_operator_greater_than_0<bool> for (usize) {
  fn operator_greater_than_0(self , rsthis: & QUuid) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QUuidgtERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/quuid.h:192
// index:0
// Public static Visibility=Default Availability=Available
// [16] QUuid createUuid()

/*
On any platform other than Windows, this function returns a new UUID with variant QUuid::DCE and version QUuid::Random. On Windows, a GUID is generated using the Windows API and will be of the type that the API decides to create.

See also variant() and version().
*/
impl /*struct*/ QUuid {
  pub fn createUuid_0<RetType, T: QUuid_createUuid_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.createUuid_0();
    // return 1;
  }
}
pub trait QUuid_createUuid_0<RetType> {
  fn createUuid_0(self ) -> RetType;
}
impl<'a> /*trait*/ QUuid_createUuid_0<usize> for () {
  fn createUuid_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QUuid10createUuidEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/quuid.h:194
// index:0
// Public static Visibility=Default Availability=Available
// [16] QUuid createUuidV3(const QUuid &, const QByteArray &)

/*
This function returns a new UUID with variant QUuid::DCE and version QUuid::Md5. ns is the namespace and baseData is the basic data as described by RFC 4122.

This function was introduced in  Qt 5.0.

See also variant(), version(), and createUuidV5().
*/
impl /*struct*/ QUuid {
  pub fn createUuidV3_0<RetType, T: QUuid_createUuidV3_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.createUuidV3_0();
    // return 1;
  }
}
pub trait QUuid_createUuidV3_0<RetType> {
  fn createUuidV3_0(self ) -> RetType;
}
impl<'a> /*trait*/ QUuid_createUuidV3_0<usize> for (usize,usize) {
  fn createUuidV3_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QUuid12createUuidV3ERKS_RK10QByteArray", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/quuid.h:196
// index:1
// Public static inline Visibility=Default Availability=Available
// [16] QUuid createUuidV3(const QUuid &, const QString &)

/*
This function returns a new UUID with variant QUuid::DCE and version QUuid::Md5. ns is the namespace and baseData is the basic data as described by RFC 4122.

This function was introduced in  Qt 5.0.

See also variant(), version(), and createUuidV5().
*/
impl /*struct*/ QUuid {
  pub fn createUuidV3_1<RetType, T: QUuid_createUuidV3_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.createUuidV3_1();
    // return 1;
  }
}
pub trait QUuid_createUuidV3_1<RetType> {
  fn createUuidV3_1(self ) -> RetType;
}
impl<'a> /*trait*/ QUuid_createUuidV3_1<usize> for (usize,usize) {
  fn createUuidV3_1(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QUuid12createUuidV3ERKS_RK7QString", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/quuid.h:195
// index:0
// Public static Visibility=Default Availability=Available
// [16] QUuid createUuidV5(const QUuid &, const QByteArray &)

/*
This function returns a new UUID with variant QUuid::DCE and version QUuid::Sha1. ns is the namespace and baseData is the basic data as described by RFC 4122.

This function was introduced in  Qt 5.0.

See also variant(), version(), and createUuidV3().
*/
impl /*struct*/ QUuid {
  pub fn createUuidV5_0<RetType, T: QUuid_createUuidV5_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.createUuidV5_0();
    // return 1;
  }
}
pub trait QUuid_createUuidV5_0<RetType> {
  fn createUuidV5_0(self ) -> RetType;
}
impl<'a> /*trait*/ QUuid_createUuidV5_0<usize> for (usize,usize) {
  fn createUuidV5_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QUuid12createUuidV5ERKS_RK10QByteArray", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/quuid.h:201
// index:1
// Public static inline Visibility=Default Availability=Available
// [16] QUuid createUuidV5(const QUuid &, const QString &)

/*
This function returns a new UUID with variant QUuid::DCE and version QUuid::Sha1. ns is the namespace and baseData is the basic data as described by RFC 4122.

This function was introduced in  Qt 5.0.

See also variant(), version(), and createUuidV3().
*/
impl /*struct*/ QUuid {
  pub fn createUuidV5_1<RetType, T: QUuid_createUuidV5_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.createUuidV5_1();
    // return 1;
  }
}
pub trait QUuid_createUuidV5_1<RetType> {
  fn createUuidV5_1(self ) -> RetType;
}
impl<'a> /*trait*/ QUuid_createUuidV5_1<usize> for (usize,usize) {
  fn createUuidV5_1(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QUuid12createUuidV5ERKS_RK7QString", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/quuid.h:208
// index:0
// Public Visibility=Default Availability=Available
// [4] QUuid::Variant variant() const

/*
Returns the value in the variant field of the UUID. If the return value is QUuid::DCE, call version() to see which layout it uses. The null UUID is considered to be of an unknown variant.

See also version().
*/
impl /*struct*/ QUuid {
  pub fn variant_0<RetType, T: QUuid_variant_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.variant_0(self);
    // return 1;
  }
}
pub trait QUuid_variant_0<RetType> {
  fn variant_0(self , rsthis: & QUuid) -> RetType;
}
impl<'a> /*trait*/ QUuid_variant_0<i32> for () {
  fn variant_0(self , rsthis: & QUuid) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QUuid7variantEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/quuid.h:209
// index:0
// Public Visibility=Default Availability=Available
// [4] QUuid::Version version() const

/*
Returns the version field of the UUID, if the UUID's variant field is QUuid::DCE. Otherwise it returns QUuid::VerUnknown.

See also variant().
*/
impl /*struct*/ QUuid {
  pub fn version_0<RetType, T: QUuid_version_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.version_0(self);
    // return 1;
  }
}
pub trait QUuid_version_0<RetType> {
  fn version_0(self , rsthis: & QUuid) -> RetType;
}
impl<'a> /*trait*/ QUuid_version_0<i32> for () {
  fn version_0(self , rsthis: & QUuid) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QUuid7versionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}


pub fn DeleteQUuid(this :*mut QUuid) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN5QUuidD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}

/*
This enum defines the values used in the variant field of the UUID. The value in the variant field determines the layout of the 128-bit value.


*/
pub type QUuid__Variant = i32;
// 
pub const QUuid__VarUnknown :QUuid__Variant = -1;
// Reserved for NCS (Network Computing System) backward compatibility
pub const QUuid__NCS :QUuid__Variant = 0;
// Distributed Computing Environment, the scheme used by QUuid
pub const QUuid__DCE :QUuid__Variant = 2;
// Reserved for Microsoft backward compatibility (GUID)
pub const QUuid__Microsoft :QUuid__Variant = 6;
// Reserved for future definition
pub const QUuid__Reserved :QUuid__Variant = 7;
pub fn QUuid_VariantItemName(val: i32) ->String {
  match val {
     QUuid__VarUnknown => // -1
     {return String::from("VarUnknown");}
     QUuid__NCS => // 0
     {return String::from("NCS");}
     QUuid__DCE => // 2
     {return String::from("DCE");}
     QUuid__Microsoft => // 6
     {return String::from("Microsoft");}
     QUuid__Reserved => // 7
     {return String::from("Reserved");}
  _ => {return format!("{}", val);}
}
}
pub fn QUuid_VariantItemName_s(val: i32) ->String {
  //var nilthis *QUuid
  //return nilthis.VariantItemName(val);
  return QUuid_VariantItemName(val);
}


/*
This enum defines the values used in the version field of the UUID. The version field is meaningful only if the value in the variant field is QUuid::DCE.


*/
pub type QUuid__Version = i32;
// 
pub const QUuid__VerUnknown :QUuid__Version = -1;
// Time-based, by using timestamp, clock sequence, and MAC network card address (if available) for the node sections
pub const QUuid__Time :QUuid__Version = 1;
// DCE Security version, with embedded POSIX UUIDs
pub const QUuid__EmbeddedPOSIX :QUuid__Version = 2;
// Alias for Name
pub const QUuid__Md5 :QUuid__Version = 3;
// 
pub const QUuid__Name :QUuid__Version = 3;
// Random-based, by using random numbers for all sections
pub const QUuid__Random :QUuid__Version = 4;
// 
pub const QUuid__Sha1 :QUuid__Version = 5;
pub fn QUuid_VersionItemName(val: i32) ->String {
  match val {
     QUuid__VerUnknown => // -1
     {return String::from("VerUnknown");}
     QUuid__Time => // 1
     {return String::from("Time");}
     QUuid__EmbeddedPOSIX => // 2
     {return String::from("EmbeddedPOSIX");}
     QUuid__Md5 => // 3
     {return String::from("Md5,Name");}
    // QUuid__Name => // 3
    // {return String::from("");}
     QUuid__Random => // 4
     {return String::from("Random");}
     QUuid__Sha1 => // 5
     {return String::from("Sha1");}
  _ => {return format!("{}", val);}
}
}
pub fn QUuid_VersionItemName_s(val: i32) ->String {
  //var nilthis *QUuid
  //return nilthis.VersionItemName(val);
  return QUuid_VersionItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
