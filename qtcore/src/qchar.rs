

// mod ::core::QChar
// package qtcore
// /usr/include/qt/QtCore/qchar.h
// #include <qchar.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 3
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
#[derive(Default)] // class sizeof(QChar)=2
pub struct QChar {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QChar_ITF interface {
//    QChar_PTR() *QChar
//}
//func (ptr *QChar) QChar_PTR() *QChar { return ptr }

impl /*struct*/ QChar {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QChar {
    return QChar{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QChar {
//  type Target = QCharBASE;
//
//  fn deref(&self) -> &QCharBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QCharBASE> for QChar {
//  fn as_ref(& self) -> & QCharBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qchar.h:81
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void QChar()

/*
Constructs a null QChar ('\0').

See also isNull().
*/
// QChar() ctx.fn_proto_cpp
impl /*struct*/ QChar {
  pub fn QChar_0<T: QChar_QChar_0>(value: T) -> QChar {
    let rsthis = value.QChar_0();
    return rsthis;
    // return 1;
  }
}

pub trait QChar_QChar_0 {
  fn QChar_0(self) -> QChar;
}
// QChar() ctx.fn_proto_cpp
impl<'a> /*trait*/ QChar_QChar_0 for () {
  fn QChar_0(self) -> QChar {
    // unsafe{_ZN5QCharC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN5QCharC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QChar{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qchar.h:82
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void QChar(ushort)

/*
Constructs a null QChar ('\0').

See also isNull().
*/
// QChar(ushort) ctx.fn_proto_cpp
impl /*struct*/ QChar {
  pub fn QChar_1<T: QChar_QChar_1>(value: T) -> QChar {
    let rsthis = value.QChar_1();
    return rsthis;
    // return 1;
  }
}

pub trait QChar_QChar_1 {
  fn QChar_1(self) -> QChar;
}
// QChar(ushort) ctx.fn_proto_cpp
impl<'a> /*trait*/ QChar_QChar_1 for (u16) {
  fn QChar_1(self) -> QChar {
    // unsafe{_ZN5QCharC2Et()};
    let arg0 = (&self) as *const u16 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN5QCharC2Et", 1,qtrt::FFITY_UINT16,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QChar{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qchar.h:83
// index:2
// Public inline Visibility=Default Availability=Available
// [-2] void QChar(uchar, uchar)

/*
Constructs a null QChar ('\0').

See also isNull().
*/
// QChar(uchar, uchar) ctx.fn_proto_cpp
impl /*struct*/ QChar {
  pub fn QChar_2<T: QChar_QChar_2>(value: T) -> QChar {
    let rsthis = value.QChar_2();
    return rsthis;
    // return 1;
  }
}

pub trait QChar_QChar_2 {
  fn QChar_2(self) -> QChar;
}
// QChar(uchar, uchar) ctx.fn_proto_cpp
impl<'a> /*trait*/ QChar_QChar_2 for (u8,u8) {
  fn QChar_2(self) -> QChar {
    // unsafe{_ZN5QCharC2Ehh()};
    let arg0 = (&self.0) as *const u8 as usize;
    let arg1 = (&self.1) as *const u8 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN5QCharC2Ehh", 2,qtrt::FFITY_UINT8,qtrt::FFITY_UINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QChar{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qchar.h:84
// index:3
// Public inline Visibility=Default Availability=Available
// [-2] void QChar(short)

/*
Constructs a null QChar ('\0').

See also isNull().
*/
// QChar(short) ctx.fn_proto_cpp
impl /*struct*/ QChar {
  pub fn QChar_3<T: QChar_QChar_3>(value: T) -> QChar {
    let rsthis = value.QChar_3();
    return rsthis;
    // return 1;
  }
}

pub trait QChar_QChar_3 {
  fn QChar_3(self) -> QChar;
}
// QChar(short) ctx.fn_proto_cpp
impl<'a> /*trait*/ QChar_QChar_3 for (i16) {
  fn QChar_3(self) -> QChar {
    // unsafe{_ZN5QCharC2Es()};
    let arg0 = (&self) as *const i16 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN5QCharC2Es", 1,qtrt::FFITY_SINT16,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QChar{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qchar.h:85
// index:4
// Public inline Visibility=Default Availability=Available
// [-2] void QChar(uint)

/*
Constructs a null QChar ('\0').

See also isNull().
*/
// QChar(uint) ctx.fn_proto_cpp
impl /*struct*/ QChar {
  pub fn QChar_4<T: QChar_QChar_4>(value: T) -> QChar {
    let rsthis = value.QChar_4();
    return rsthis;
    // return 1;
  }
}

pub trait QChar_QChar_4 {
  fn QChar_4(self) -> QChar;
}
// QChar(uint) ctx.fn_proto_cpp
impl<'a> /*trait*/ QChar_QChar_4 for (u32) {
  fn QChar_4(self) -> QChar {
    // unsafe{_ZN5QCharC2Ej()};
    let arg0 = (&self) as *const u32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN5QCharC2Ej", 1,qtrt::FFITY_UINT32,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QChar{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qchar.h:86
// index:5
// Public inline Visibility=Default Availability=Available
// [-2] void QChar(int)

/*
Constructs a null QChar ('\0').

See also isNull().
*/
// QChar(int) ctx.fn_proto_cpp
impl /*struct*/ QChar {
  pub fn QChar_5<T: QChar_QChar_5>(value: T) -> QChar {
    let rsthis = value.QChar_5();
    return rsthis;
    // return 1;
  }
}

pub trait QChar_QChar_5 {
  fn QChar_5(self) -> QChar;
}
// QChar(int) ctx.fn_proto_cpp
impl<'a> /*trait*/ QChar_QChar_5 for (i32) {
  fn QChar_5(self) -> QChar {
    // unsafe{_ZN5QCharC2Ei()};
    let arg0 = (&self) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN5QCharC2Ei", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QChar{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qchar.h:87
// index:6
// Public inline Visibility=Default Availability=Available
// [-2] void QChar(QChar::SpecialCharacter)

/*
Constructs a null QChar ('\0').

See also isNull().
*/
// QChar(QChar::SpecialCharacter) ctx.fn_proto_cpp
impl /*struct*/ QChar {
  pub fn QChar_6<T: QChar_QChar_6>(value: T) -> QChar {
    let rsthis = value.QChar_6();
    return rsthis;
    // return 1;
  }
}

pub trait QChar_QChar_6 {
  fn QChar_6(self) -> QChar;
}
// QChar(QChar::SpecialCharacter) ctx.fn_proto_cpp
impl<'a> /*trait*/ QChar_QChar_6 for (i32) {
  fn QChar_6(self) -> QChar {
    // unsafe{_ZN5QCharC2ENS_16SpecialCharacterE()};
    let arg0 = (&self) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN5QCharC2ENS_16SpecialCharacterE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QChar{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qchar.h:90
// index:7
// Public inline Visibility=Default Availability=Available
// [-2] void QChar(char16_t)

/*
Constructs a null QChar ('\0').

See also isNull().
*/
// QChar(char16_t) ctx.fn_proto_cpp
impl /*struct*/ QChar {
  pub fn QChar_7<T: QChar_QChar_7>(value: T) -> QChar {
    let rsthis = value.QChar_7();
    return rsthis;
    // return 1;
  }
}

pub trait QChar_QChar_7 {
  fn QChar_7(self) -> QChar;
}
// QChar(char16_t) ctx.fn_proto_cpp
impl<'a> /*trait*/ QChar_QChar_7 for (i16) {
  fn QChar_7(self) -> QChar {
    // unsafe{_ZN5QCharC2EDs()};
    let arg0 = (&self) as *const i16 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN5QCharC2EDs", 1,qtrt::FFITY_SINT16,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QChar{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qchar.h:98
// index:8
// Public inline Visibility=Default Availability=Available
// [-2] void QChar(char)

/*
Constructs a null QChar ('\0').

See also isNull().
*/
// QChar(char) ctx.fn_proto_cpp
impl /*struct*/ QChar {
  pub fn QChar_8<T: QChar_QChar_8>(value: T) -> QChar {
    let rsthis = value.QChar_8();
    return rsthis;
    // return 1;
  }
}

pub trait QChar_QChar_8 {
  fn QChar_8(self) -> QChar;
}
// QChar(char) ctx.fn_proto_cpp
impl<'a> /*trait*/ QChar_QChar_8 for (i8) {
  fn QChar_8(self) -> QChar {
    // unsafe{_ZN5QCharC2Ec()};
    let arg0 = (&self) as *const i8 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN5QCharC2Ec", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QChar{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qchar.h:100
// index:9
// Public inline Visibility=Default Availability=Available
// [-2] void QChar(uchar)

/*
Constructs a null QChar ('\0').

See also isNull().
*/
// QChar(uchar) ctx.fn_proto_cpp
impl /*struct*/ QChar {
  pub fn QChar_9<T: QChar_QChar_9>(value: T) -> QChar {
    let rsthis = value.QChar_9();
    return rsthis;
    // return 1;
  }
}

pub trait QChar_QChar_9 {
  fn QChar_9(self) -> QChar;
}
// QChar(uchar) ctx.fn_proto_cpp
impl<'a> /*trait*/ QChar_QChar_9 for (u8) {
  fn QChar_9(self) -> QChar {
    // unsafe{_ZN5QCharC2Eh()};
    let arg0 = (&self) as *const u8 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN5QCharC2Eh", 1,qtrt::FFITY_UINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QChar{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qchar.h:394
// index:0
// Public inline Visibility=Default Availability=Available
// [4] QChar::Category category() const

/*
Returns the character's category.
*/
impl /*struct*/ QChar {
  pub fn category_0<RetType, T: QChar_category_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.category_0(self);
    // return 1;
  }
}
pub trait QChar_category_0<RetType> {
  fn category_0(self , rsthis: & QChar) -> RetType;
}
impl<'a> /*trait*/ QChar_category_0<i32> for () {
  fn category_0(self , rsthis: & QChar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QChar8categoryEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qchar.h:504
// index:1
// Public static Visibility=Default Availability=Available
// [4] QChar::Category category(uint)

/*
Returns the character's category.
*/
impl /*struct*/ QChar {
  pub fn category_1<RetType, T: QChar_category_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.category_1();
    // return 1;
  }
}
pub trait QChar_category_1<RetType> {
  fn category_1(self ) -> RetType;
}
impl<'a> /*trait*/ QChar_category_1<i32> for (u32) {
  fn category_1(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const u32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QChar8categoryEj", 1,qtrt::FFITY_UINT32,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qchar.h:395
// index:0
// Public inline Visibility=Default Availability=Available
// [4] QChar::Direction direction() const

/*
Returns the character's direction.
*/
impl /*struct*/ QChar {
  pub fn direction_0<RetType, T: QChar_direction_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.direction_0(self);
    // return 1;
  }
}
pub trait QChar_direction_0<RetType> {
  fn direction_0(self , rsthis: & QChar) -> RetType;
}
impl<'a> /*trait*/ QChar_direction_0<i32> for () {
  fn direction_0(self , rsthis: & QChar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QChar9directionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qchar.h:505
// index:1
// Public static Visibility=Default Availability=Available
// [4] QChar::Direction direction(uint)

/*
Returns the character's direction.
*/
impl /*struct*/ QChar {
  pub fn direction_1<RetType, T: QChar_direction_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.direction_1();
    // return 1;
  }
}
pub trait QChar_direction_1<RetType> {
  fn direction_1(self ) -> RetType;
}
impl<'a> /*trait*/ QChar_direction_1<i32> for (u32) {
  fn direction_1(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const u32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QChar9directionEj", 1,qtrt::FFITY_UINT32,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qchar.h:396
// index:0
// Public inline Visibility=Default Availability=Available
// [4] QChar::JoiningType joiningType() const

/*
Returns information about the joining type attributes of the character (needed for certain languages such as Arabic or Syriac).

This function was introduced in  Qt 5.3.
*/
impl /*struct*/ QChar {
  pub fn joiningType_0<RetType, T: QChar_joiningType_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.joiningType_0(self);
    // return 1;
  }
}
pub trait QChar_joiningType_0<RetType> {
  fn joiningType_0(self , rsthis: & QChar) -> RetType;
}
impl<'a> /*trait*/ QChar_joiningType_0<i32> for () {
  fn joiningType_0(self , rsthis: & QChar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QChar11joiningTypeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qchar.h:506
// index:1
// Public static Visibility=Default Availability=Available
// [4] QChar::JoiningType joiningType(uint)

/*
Returns information about the joining type attributes of the character (needed for certain languages such as Arabic or Syriac).

This function was introduced in  Qt 5.3.
*/
impl /*struct*/ QChar {
  pub fn joiningType_1<RetType, T: QChar_joiningType_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.joiningType_1();
    // return 1;
  }
}
pub trait QChar_joiningType_1<RetType> {
  fn joiningType_1(self ) -> RetType;
}
impl<'a> /*trait*/ QChar_joiningType_1<i32> for (u32) {
  fn joiningType_1(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const u32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QChar11joiningTypeEj", 1,qtrt::FFITY_UINT32,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qchar.h:398
// index:0
// Public inline Visibility=Default Availability=Available
// [4] QChar::Joining joining() const

/*

*/
impl /*struct*/ QChar {
  pub fn joining_0<RetType, T: QChar_joining_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.joining_0(self);
    // return 1;
  }
}
pub trait QChar_joining_0<RetType> {
  fn joining_0(self , rsthis: & QChar) -> RetType;
}
impl<'a> /*trait*/ QChar_joining_0<i32> for () {
  fn joining_0(self , rsthis: & QChar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QChar7joiningEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qchar.h:508
// index:1
// Public static Visibility=Default Availability=Available
// [4] QChar::Joining joining(uint)

/*

*/
impl /*struct*/ QChar {
  pub fn joining_1<RetType, T: QChar_joining_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.joining_1();
    // return 1;
  }
}
pub trait QChar_joining_1<RetType> {
  fn joining_1(self ) -> RetType;
}
impl<'a> /*trait*/ QChar_joining_1<i32> for (u32) {
  fn joining_1(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const u32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QChar7joiningEj", 1,qtrt::FFITY_UINT32,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qchar.h:411
// index:0
// Public inline Visibility=Default Availability=Available
// [1] unsigned char combiningClass() const

/*
Returns the combining class for the character as defined in the Unicode standard. This is mainly useful as a positioning hint for marks attached to a base character.

The Qt text rendering engine uses this information to correctly position non-spacing marks around a base character.
*/
impl /*struct*/ QChar {
  pub fn combiningClass_0<RetType, T: QChar_combiningClass_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.combiningClass_0(self);
    // return 1;
  }
}
pub trait QChar_combiningClass_0<RetType> {
  fn combiningClass_0(self , rsthis: & QChar) -> RetType;
}
impl<'a> /*trait*/ QChar_combiningClass_0<u8> for () {
  fn combiningClass_0(self , rsthis: & QChar) -> u8 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QChar14combiningClassEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u8 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qchar.h:510
// index:1
// Public static Visibility=Default Availability=Available
// [1] unsigned char combiningClass(uint)

/*
Returns the combining class for the character as defined in the Unicode standard. This is mainly useful as a positioning hint for marks attached to a base character.

The Qt text rendering engine uses this information to correctly position non-spacing marks around a base character.
*/
impl /*struct*/ QChar {
  pub fn combiningClass_1<RetType, T: QChar_combiningClass_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.combiningClass_1();
    // return 1;
  }
}
pub trait QChar_combiningClass_1<RetType> {
  fn combiningClass_1(self ) -> RetType;
}
impl<'a> /*trait*/ QChar_combiningClass_1<u8> for (u32) {
  fn combiningClass_1(self ) -> u8 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const u32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QChar14combiningClassEj", 1,qtrt::FFITY_UINT32,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u8 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qchar.h:413
// index:0
// Public inline Visibility=Default Availability=Available
// [2] QChar mirroredChar() const

/*
Returns the mirrored character if this character is a mirrored character; otherwise returns the character itself.

See also hasMirrored().
*/
impl /*struct*/ QChar {
  pub fn mirroredChar_0<RetType, T: QChar_mirroredChar_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mirroredChar_0(self);
    // return 1;
  }
}
pub trait QChar_mirroredChar_0<RetType> {
  fn mirroredChar_0(self , rsthis: & QChar) -> RetType;
}
impl<'a> /*trait*/ QChar_mirroredChar_0<usize> for () {
  fn mirroredChar_0(self , rsthis: & QChar) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QChar12mirroredCharEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qchar.h:512
// index:1
// Public static Visibility=Default Availability=Available
// [4] uint mirroredChar(uint)

/*
Returns the mirrored character if this character is a mirrored character; otherwise returns the character itself.

See also hasMirrored().
*/
impl /*struct*/ QChar {
  pub fn mirroredChar_1<RetType, T: QChar_mirroredChar_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.mirroredChar_1();
    // return 1;
  }
}
pub trait QChar_mirroredChar_1<RetType> {
  fn mirroredChar_1(self ) -> RetType;
}
impl<'a> /*trait*/ QChar_mirroredChar_1<u32> for (u32) {
  fn mirroredChar_1(self ) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const u32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QChar12mirroredCharEj", 1,qtrt::FFITY_UINT32,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qchar.h:414
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool hasMirrored() const

/*
Returns true if the character should be reversed if the text direction is reversed; otherwise returns false.

A bit faster equivalent of (ch.mirroredChar() != ch).

See also mirroredChar().
*/
impl /*struct*/ QChar {
  pub fn hasMirrored_0<RetType, T: QChar_hasMirrored_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hasMirrored_0(self);
    // return 1;
  }
}
pub trait QChar_hasMirrored_0<RetType> {
  fn hasMirrored_0(self , rsthis: & QChar) -> RetType;
}
impl<'a> /*trait*/ QChar_hasMirrored_0<bool> for () {
  fn hasMirrored_0(self , rsthis: & QChar) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QChar11hasMirroredEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qchar.h:513
// index:1
// Public static Visibility=Default Availability=Available
// [1] bool hasMirrored(uint)

/*
Returns true if the character should be reversed if the text direction is reversed; otherwise returns false.

A bit faster equivalent of (ch.mirroredChar() != ch).

See also mirroredChar().
*/
impl /*struct*/ QChar {
  pub fn hasMirrored_1<RetType, T: QChar_hasMirrored_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.hasMirrored_1();
    // return 1;
  }
}
pub trait QChar_hasMirrored_1<RetType> {
  fn hasMirrored_1(self ) -> RetType;
}
impl<'a> /*trait*/ QChar_hasMirrored_1<bool> for (u32) {
  fn hasMirrored_1(self ) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const u32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QChar11hasMirroredEj", 1,qtrt::FFITY_UINT32,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qchar.h:416
// index:0
// Public Visibility=Default Availability=Available
// [8] QString decomposition() const

/*
Decomposes a character into it's constituent parts. Returns an empty string if no decomposition exists.
*/
impl /*struct*/ QChar {
  pub fn decomposition_0<RetType, T: QChar_decomposition_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.decomposition_0(self);
    // return 1;
  }
}
pub trait QChar_decomposition_0<RetType> {
  fn decomposition_0(self , rsthis: & QChar) -> RetType;
}
impl<'a> /*trait*/ QChar_decomposition_0<usize> for () {
  fn decomposition_0(self , rsthis: & QChar) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QChar13decompositionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qchar.h:515
// index:1
// Public static Visibility=Default Availability=Available
// [8] QString decomposition(uint)

/*
Decomposes a character into it's constituent parts. Returns an empty string if no decomposition exists.
*/
impl /*struct*/ QChar {
  pub fn decomposition_1<RetType, T: QChar_decomposition_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.decomposition_1();
    // return 1;
  }
}
pub trait QChar_decomposition_1<RetType> {
  fn decomposition_1(self ) -> RetType;
}
impl<'a> /*trait*/ QChar_decomposition_1<usize> for (u32) {
  fn decomposition_1(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const u32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QChar13decompositionEj", 1,qtrt::FFITY_UINT32,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qchar.h:417
// index:0
// Public inline Visibility=Default Availability=Available
// [4] QChar::Decomposition decompositionTag() const

/*
Returns the tag defining the composition of the character. Returns QChar::NoDecomposition if no decomposition exists.
*/
impl /*struct*/ QChar {
  pub fn decompositionTag_0<RetType, T: QChar_decompositionTag_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.decompositionTag_0(self);
    // return 1;
  }
}
pub trait QChar_decompositionTag_0<RetType> {
  fn decompositionTag_0(self , rsthis: & QChar) -> RetType;
}
impl<'a> /*trait*/ QChar_decompositionTag_0<i32> for () {
  fn decompositionTag_0(self , rsthis: & QChar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QChar16decompositionTagEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qchar.h:516
// index:1
// Public static Visibility=Default Availability=Available
// [4] QChar::Decomposition decompositionTag(uint)

/*
Returns the tag defining the composition of the character. Returns QChar::NoDecomposition if no decomposition exists.
*/
impl /*struct*/ QChar {
  pub fn decompositionTag_1<RetType, T: QChar_decompositionTag_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.decompositionTag_1();
    // return 1;
  }
}
pub trait QChar_decompositionTag_1<RetType> {
  fn decompositionTag_1(self ) -> RetType;
}
impl<'a> /*trait*/ QChar_decompositionTag_1<i32> for (u32) {
  fn decompositionTag_1(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const u32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QChar16decompositionTagEj", 1,qtrt::FFITY_UINT32,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qchar.h:419
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int digitValue() const

/*
Returns the numeric value of the digit, or -1 if the character is not a digit.
*/
impl /*struct*/ QChar {
  pub fn digitValue_0<RetType, T: QChar_digitValue_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.digitValue_0(self);
    // return 1;
  }
}
pub trait QChar_digitValue_0<RetType> {
  fn digitValue_0(self , rsthis: & QChar) -> RetType;
}
impl<'a> /*trait*/ QChar_digitValue_0<i32> for () {
  fn digitValue_0(self , rsthis: & QChar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QChar10digitValueEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qchar.h:518
// index:1
// Public static Visibility=Default Availability=Available
// [4] int digitValue(uint)

/*
Returns the numeric value of the digit, or -1 if the character is not a digit.
*/
impl /*struct*/ QChar {
  pub fn digitValue_1<RetType, T: QChar_digitValue_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.digitValue_1();
    // return 1;
  }
}
pub trait QChar_digitValue_1<RetType> {
  fn digitValue_1(self ) -> RetType;
}
impl<'a> /*trait*/ QChar_digitValue_1<i32> for (u32) {
  fn digitValue_1(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const u32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QChar10digitValueEj", 1,qtrt::FFITY_UINT32,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qchar.h:420
// index:0
// Public inline Visibility=Default Availability=Available
// [2] QChar toLower() const

/*
Returns the lowercase equivalent if the character is uppercase or titlecase; otherwise returns the character itself.
*/
impl /*struct*/ QChar {
  pub fn toLower_0<RetType, T: QChar_toLower_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toLower_0(self);
    // return 1;
  }
}
pub trait QChar_toLower_0<RetType> {
  fn toLower_0(self , rsthis: & QChar) -> RetType;
}
impl<'a> /*trait*/ QChar_toLower_0<usize> for () {
  fn toLower_0(self , rsthis: & QChar) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QChar7toLowerEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qchar.h:519
// index:1
// Public static Visibility=Default Availability=Available
// [4] uint toLower(uint)

/*
Returns the lowercase equivalent if the character is uppercase or titlecase; otherwise returns the character itself.
*/
impl /*struct*/ QChar {
  pub fn toLower_1<RetType, T: QChar_toLower_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.toLower_1();
    // return 1;
  }
}
pub trait QChar_toLower_1<RetType> {
  fn toLower_1(self ) -> RetType;
}
impl<'a> /*trait*/ QChar_toLower_1<u32> for (u32) {
  fn toLower_1(self ) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const u32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QChar7toLowerEj", 1,qtrt::FFITY_UINT32,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qchar.h:421
// index:0
// Public inline Visibility=Default Availability=Available
// [2] QChar toUpper() const

/*
Returns the uppercase equivalent if the character is lowercase or titlecase; otherwise returns the character itself.
*/
impl /*struct*/ QChar {
  pub fn toUpper_0<RetType, T: QChar_toUpper_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toUpper_0(self);
    // return 1;
  }
}
pub trait QChar_toUpper_0<RetType> {
  fn toUpper_0(self , rsthis: & QChar) -> RetType;
}
impl<'a> /*trait*/ QChar_toUpper_0<usize> for () {
  fn toUpper_0(self , rsthis: & QChar) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QChar7toUpperEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qchar.h:520
// index:1
// Public static Visibility=Default Availability=Available
// [4] uint toUpper(uint)

/*
Returns the uppercase equivalent if the character is lowercase or titlecase; otherwise returns the character itself.
*/
impl /*struct*/ QChar {
  pub fn toUpper_1<RetType, T: QChar_toUpper_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.toUpper_1();
    // return 1;
  }
}
pub trait QChar_toUpper_1<RetType> {
  fn toUpper_1(self ) -> RetType;
}
impl<'a> /*trait*/ QChar_toUpper_1<u32> for (u32) {
  fn toUpper_1(self ) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const u32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QChar7toUpperEj", 1,qtrt::FFITY_UINT32,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qchar.h:422
// index:0
// Public inline Visibility=Default Availability=Available
// [2] QChar toTitleCase() const

/*
Returns the title case equivalent if the character is lowercase or uppercase; otherwise returns the character itself.
*/
impl /*struct*/ QChar {
  pub fn toTitleCase_0<RetType, T: QChar_toTitleCase_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toTitleCase_0(self);
    // return 1;
  }
}
pub trait QChar_toTitleCase_0<RetType> {
  fn toTitleCase_0(self , rsthis: & QChar) -> RetType;
}
impl<'a> /*trait*/ QChar_toTitleCase_0<usize> for () {
  fn toTitleCase_0(self , rsthis: & QChar) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QChar11toTitleCaseEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qchar.h:521
// index:1
// Public static Visibility=Default Availability=Available
// [4] uint toTitleCase(uint)

/*
Returns the title case equivalent if the character is lowercase or uppercase; otherwise returns the character itself.
*/
impl /*struct*/ QChar {
  pub fn toTitleCase_1<RetType, T: QChar_toTitleCase_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.toTitleCase_1();
    // return 1;
  }
}
pub trait QChar_toTitleCase_1<RetType> {
  fn toTitleCase_1(self ) -> RetType;
}
impl<'a> /*trait*/ QChar_toTitleCase_1<u32> for (u32) {
  fn toTitleCase_1(self ) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const u32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QChar11toTitleCaseEj", 1,qtrt::FFITY_UINT32,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qchar.h:423
// index:0
// Public inline Visibility=Default Availability=Available
// [2] QChar toCaseFolded() const

/*
Returns the case folded equivalent of the character. For most Unicode characters this is the same as toLower().
*/
impl /*struct*/ QChar {
  pub fn toCaseFolded_0<RetType, T: QChar_toCaseFolded_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toCaseFolded_0(self);
    // return 1;
  }
}
pub trait QChar_toCaseFolded_0<RetType> {
  fn toCaseFolded_0(self , rsthis: & QChar) -> RetType;
}
impl<'a> /*trait*/ QChar_toCaseFolded_0<usize> for () {
  fn toCaseFolded_0(self , rsthis: & QChar) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QChar12toCaseFoldedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qchar.h:522
// index:1
// Public static Visibility=Default Availability=Available
// [4] uint toCaseFolded(uint)

/*
Returns the case folded equivalent of the character. For most Unicode characters this is the same as toLower().
*/
impl /*struct*/ QChar {
  pub fn toCaseFolded_1<RetType, T: QChar_toCaseFolded_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.toCaseFolded_1();
    // return 1;
  }
}
pub trait QChar_toCaseFolded_1<RetType> {
  fn toCaseFolded_1(self ) -> RetType;
}
impl<'a> /*trait*/ QChar_toCaseFolded_1<u32> for (u32) {
  fn toCaseFolded_1(self ) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const u32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QChar12toCaseFoldedEj", 1,qtrt::FFITY_UINT32,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qchar.h:425
// index:0
// Public inline Visibility=Default Availability=Available
// [4] QChar::Script script() const

/*
Returns the Unicode script property value for this character.

This function was introduced in  Qt 5.1.
*/
impl /*struct*/ QChar {
  pub fn script_0<RetType, T: QChar_script_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.script_0(self);
    // return 1;
  }
}
pub trait QChar_script_0<RetType> {
  fn script_0(self , rsthis: & QChar) -> RetType;
}
impl<'a> /*trait*/ QChar_script_0<i32> for () {
  fn script_0(self , rsthis: & QChar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QChar6scriptEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qchar.h:524
// index:1
// Public static Visibility=Default Availability=Available
// [4] QChar::Script script(uint)

/*
Returns the Unicode script property value for this character.

This function was introduced in  Qt 5.1.
*/
impl /*struct*/ QChar {
  pub fn script_1<RetType, T: QChar_script_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.script_1();
    // return 1;
  }
}
pub trait QChar_script_1<RetType> {
  fn script_1(self ) -> RetType;
}
impl<'a> /*trait*/ QChar_script_1<i32> for (u32) {
  fn script_1(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const u32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QChar6scriptEj", 1,qtrt::FFITY_UINT32,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qchar.h:427
// index:0
// Public inline Visibility=Default Availability=Available
// [4] QChar::UnicodeVersion unicodeVersion() const

/*
Returns the Unicode version that introduced this character.
*/
impl /*struct*/ QChar {
  pub fn unicodeVersion_0<RetType, T: QChar_unicodeVersion_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.unicodeVersion_0(self);
    // return 1;
  }
}
pub trait QChar_unicodeVersion_0<RetType> {
  fn unicodeVersion_0(self , rsthis: & QChar) -> RetType;
}
impl<'a> /*trait*/ QChar_unicodeVersion_0<i32> for () {
  fn unicodeVersion_0(self , rsthis: & QChar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QChar14unicodeVersionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qchar.h:526
// index:1
// Public static Visibility=Default Availability=Available
// [4] QChar::UnicodeVersion unicodeVersion(uint)

/*
Returns the Unicode version that introduced this character.
*/
impl /*struct*/ QChar {
  pub fn unicodeVersion_1<RetType, T: QChar_unicodeVersion_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.unicodeVersion_1();
    // return 1;
  }
}
pub trait QChar_unicodeVersion_1<RetType> {
  fn unicodeVersion_1(self ) -> RetType;
}
impl<'a> /*trait*/ QChar_unicodeVersion_1<i32> for (u32) {
  fn unicodeVersion_1(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const u32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QChar14unicodeVersionEj", 1,qtrt::FFITY_UINT32,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qchar.h:432
// index:0
// Public inline Visibility=Default Availability=Available
// [1] char toLatin1() const

/*
Returns the Latin-1 character equivalent to the QChar, or 0. This is mainly useful for non-internationalized software.

Note: It is not possible to distinguish a non-Latin-1 character from a Latin-1 0 (NUL) character. Prefer to use unicode(), which does not have this ambiguity.

See also unicode().
*/
impl /*struct*/ QChar {
  pub fn toLatin1_0<RetType, T: QChar_toLatin1_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toLatin1_0(self);
    // return 1;
  }
}
pub trait QChar_toLatin1_0<RetType> {
  fn toLatin1_0(self , rsthis: & QChar) -> RetType;
}
impl<'a> /*trait*/ QChar_toLatin1_0<i8> for () {
  fn toLatin1_0(self , rsthis: & QChar) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QChar8toLatin1Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i8 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qchar.h:433
// index:0
// Public inline Visibility=Default Availability=Available
// [2] ushort unicode() const

/*
Returns the numeric Unicode value of the QChar.
*/
impl /*struct*/ QChar {
  pub fn unicode_0<RetType, T: QChar_unicode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.unicode_0(self);
    // return 1;
  }
}
pub trait QChar_unicode_0<RetType> {
  fn unicode_0(self , rsthis: & QChar) -> RetType;
}
impl<'a> /*trait*/ QChar_unicode_0<u16> for () {
  fn unicode_0(self , rsthis: & QChar) -> u16 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QChar7unicodeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u16 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qchar.h:434
// index:1
// Public inline Visibility=Default Availability=Available
// [2] ushort & unicode()

/*
Returns the numeric Unicode value of the QChar.
*/
impl /*struct*/ QChar {
  pub fn unicode_1<RetType, T: QChar_unicode_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.unicode_1(self);
    // return 1;
  }
}
pub trait QChar_unicode_1<RetType> {
  fn unicode_1(self , rsthis: & QChar) -> RetType;
}
impl<'a> /*trait*/ QChar_unicode_1<usize> for () {
  fn unicode_1(self , rsthis: & QChar) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QChar7unicodeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qchar.h:440
// index:0
// Public static inline Visibility=Default Availability=Available
// [2] QChar fromLatin1(char)

/*
Converts the Latin-1 character c to its equivalent QChar. This is mainly useful for non-internationalized software.

An alternative is to use QLatin1Char.

See also toLatin1() and unicode().
*/
impl /*struct*/ QChar {
  pub fn fromLatin1_0<RetType, T: QChar_fromLatin1_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromLatin1_0();
    // return 1;
  }
}
pub trait QChar_fromLatin1_0<RetType> {
  fn fromLatin1_0(self ) -> RetType;
}
impl<'a> /*trait*/ QChar_fromLatin1_0<usize> for (i8) {
  fn fromLatin1_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i8 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QChar10fromLatin1Ec", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qchar.h:442
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isNull() const

/*
Returns true if the character is the Unicode character 0x0000 ('\0'); otherwise returns false.
*/
impl /*struct*/ QChar {
  pub fn isNull_0<RetType, T: QChar_isNull_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isNull_0(self);
    // return 1;
  }
}
pub trait QChar_isNull_0<RetType> {
  fn isNull_0(self , rsthis: & QChar) -> RetType;
}
impl<'a> /*trait*/ QChar_isNull_0<bool> for () {
  fn isNull_0(self , rsthis: & QChar) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QChar6isNullEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qchar.h:444
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isPrint() const

/*
Returns true if the character is a printable character; otherwise returns false. This is any character not of category Other_*.

Note that this gives no indication of whether the character is available in a particular font.
*/
impl /*struct*/ QChar {
  pub fn isPrint_0<RetType, T: QChar_isPrint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isPrint_0(self);
    // return 1;
  }
}
pub trait QChar_isPrint_0<RetType> {
  fn isPrint_0(self , rsthis: & QChar) -> RetType;
}
impl<'a> /*trait*/ QChar_isPrint_0<bool> for () {
  fn isPrint_0(self , rsthis: & QChar) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QChar7isPrintEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qchar.h:530
// index:1
// Public static Visibility=Default Availability=Available
// [1] bool isPrint(uint)

/*
Returns true if the character is a printable character; otherwise returns false. This is any character not of category Other_*.

Note that this gives no indication of whether the character is available in a particular font.
*/
impl /*struct*/ QChar {
  pub fn isPrint_1<RetType, T: QChar_isPrint_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.isPrint_1();
    // return 1;
  }
}
pub trait QChar_isPrint_1<RetType> {
  fn isPrint_1(self ) -> RetType;
}
impl<'a> /*trait*/ QChar_isPrint_1<bool> for (u32) {
  fn isPrint_1(self ) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const u32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QChar7isPrintEj", 1,qtrt::FFITY_UINT32,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qchar.h:445
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isSpace() const

/*
Returns true if the character is a separator character (Separator_* categories or certain code points from Other_Control category); otherwise returns false.
*/
impl /*struct*/ QChar {
  pub fn isSpace_0<RetType, T: QChar_isSpace_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isSpace_0(self);
    // return 1;
  }
}
pub trait QChar_isSpace_0<RetType> {
  fn isSpace_0(self , rsthis: & QChar) -> RetType;
}
impl<'a> /*trait*/ QChar_isSpace_0<bool> for () {
  fn isSpace_0(self , rsthis: & QChar) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QChar7isSpaceEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qchar.h:531
// index:1
// Public static inline Visibility=Default Availability=Available
// [1] bool isSpace(uint)

/*
Returns true if the character is a separator character (Separator_* categories or certain code points from Other_Control category); otherwise returns false.
*/
impl /*struct*/ QChar {
  pub fn isSpace_1<RetType, T: QChar_isSpace_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.isSpace_1();
    // return 1;
  }
}
pub trait QChar_isSpace_1<RetType> {
  fn isSpace_1(self ) -> RetType;
}
impl<'a> /*trait*/ QChar_isSpace_1<bool> for (u32) {
  fn isSpace_1(self ) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const u32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QChar7isSpaceEj", 1,qtrt::FFITY_UINT32,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qchar.h:446
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isMark() const

/*
Returns true if the character is a mark (Mark_* categories); otherwise returns false.

See QChar::Category for more information regarding marks.
*/
impl /*struct*/ QChar {
  pub fn isMark_0<RetType, T: QChar_isMark_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isMark_0(self);
    // return 1;
  }
}
pub trait QChar_isMark_0<RetType> {
  fn isMark_0(self , rsthis: & QChar) -> RetType;
}
impl<'a> /*trait*/ QChar_isMark_0<bool> for () {
  fn isMark_0(self , rsthis: & QChar) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QChar6isMarkEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qchar.h:537
// index:1
// Public static Visibility=Default Availability=Available
// [1] bool isMark(uint)

/*
Returns true if the character is a mark (Mark_* categories); otherwise returns false.

See QChar::Category for more information regarding marks.
*/
impl /*struct*/ QChar {
  pub fn isMark_1<RetType, T: QChar_isMark_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.isMark_1();
    // return 1;
  }
}
pub trait QChar_isMark_1<RetType> {
  fn isMark_1(self ) -> RetType;
}
impl<'a> /*trait*/ QChar_isMark_1<bool> for (u32) {
  fn isMark_1(self ) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const u32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QChar6isMarkEj", 1,qtrt::FFITY_UINT32,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qchar.h:447
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isPunct() const

/*
Returns true if the character is a punctuation mark (Punctuation_* categories); otherwise returns false.
*/
impl /*struct*/ QChar {
  pub fn isPunct_0<RetType, T: QChar_isPunct_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isPunct_0(self);
    // return 1;
  }
}
pub trait QChar_isPunct_0<RetType> {
  fn isPunct_0(self , rsthis: & QChar) -> RetType;
}
impl<'a> /*trait*/ QChar_isPunct_0<bool> for () {
  fn isPunct_0(self , rsthis: & QChar) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QChar7isPunctEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qchar.h:538
// index:1
// Public static Visibility=Default Availability=Available
// [1] bool isPunct(uint)

/*
Returns true if the character is a punctuation mark (Punctuation_* categories); otherwise returns false.
*/
impl /*struct*/ QChar {
  pub fn isPunct_1<RetType, T: QChar_isPunct_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.isPunct_1();
    // return 1;
  }
}
pub trait QChar_isPunct_1<RetType> {
  fn isPunct_1(self ) -> RetType;
}
impl<'a> /*trait*/ QChar_isPunct_1<bool> for (u32) {
  fn isPunct_1(self ) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const u32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QChar7isPunctEj", 1,qtrt::FFITY_UINT32,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qchar.h:448
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isSymbol() const

/*
Returns true if the character is a symbol (Symbol_* categories); otherwise returns false.
*/
impl /*struct*/ QChar {
  pub fn isSymbol_0<RetType, T: QChar_isSymbol_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isSymbol_0(self);
    // return 1;
  }
}
pub trait QChar_isSymbol_0<RetType> {
  fn isSymbol_0(self , rsthis: & QChar) -> RetType;
}
impl<'a> /*trait*/ QChar_isSymbol_0<bool> for () {
  fn isSymbol_0(self , rsthis: & QChar) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QChar8isSymbolEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qchar.h:539
// index:1
// Public static Visibility=Default Availability=Available
// [1] bool isSymbol(uint)

/*
Returns true if the character is a symbol (Symbol_* categories); otherwise returns false.
*/
impl /*struct*/ QChar {
  pub fn isSymbol_1<RetType, T: QChar_isSymbol_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.isSymbol_1();
    // return 1;
  }
}
pub trait QChar_isSymbol_1<RetType> {
  fn isSymbol_1(self ) -> RetType;
}
impl<'a> /*trait*/ QChar_isSymbol_1<bool> for (u32) {
  fn isSymbol_1(self ) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const u32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QChar8isSymbolEj", 1,qtrt::FFITY_UINT32,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qchar.h:449
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isLetter() const

/*
Returns true if the character is a letter (Letter_* categories); otherwise returns false.
*/
impl /*struct*/ QChar {
  pub fn isLetter_0<RetType, T: QChar_isLetter_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isLetter_0(self);
    // return 1;
  }
}
pub trait QChar_isLetter_0<RetType> {
  fn isLetter_0(self , rsthis: & QChar) -> RetType;
}
impl<'a> /*trait*/ QChar_isLetter_0<bool> for () {
  fn isLetter_0(self , rsthis: & QChar) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QChar8isLetterEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qchar.h:540
// index:1
// Public static inline Visibility=Default Availability=Available
// [1] bool isLetter(uint)

/*
Returns true if the character is a letter (Letter_* categories); otherwise returns false.
*/
impl /*struct*/ QChar {
  pub fn isLetter_1<RetType, T: QChar_isLetter_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.isLetter_1();
    // return 1;
  }
}
pub trait QChar_isLetter_1<RetType> {
  fn isLetter_1(self ) -> RetType;
}
impl<'a> /*trait*/ QChar_isLetter_1<bool> for (u32) {
  fn isLetter_1(self ) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const u32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QChar8isLetterEj", 1,qtrt::FFITY_UINT32,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qchar.h:450
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isNumber() const

/*
Returns true if the character is a number (Number_* categories, not just 0-9); otherwise returns false.

See also isDigit().
*/
impl /*struct*/ QChar {
  pub fn isNumber_0<RetType, T: QChar_isNumber_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isNumber_0(self);
    // return 1;
  }
}
pub trait QChar_isNumber_0<RetType> {
  fn isNumber_0(self , rsthis: & QChar) -> RetType;
}
impl<'a> /*trait*/ QChar_isNumber_0<bool> for () {
  fn isNumber_0(self , rsthis: & QChar) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QChar8isNumberEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qchar.h:545
// index:1
// Public static inline Visibility=Default Availability=Available
// [1] bool isNumber(uint)

/*
Returns true if the character is a number (Number_* categories, not just 0-9); otherwise returns false.

See also isDigit().
*/
impl /*struct*/ QChar {
  pub fn isNumber_1<RetType, T: QChar_isNumber_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.isNumber_1();
    // return 1;
  }
}
pub trait QChar_isNumber_1<RetType> {
  fn isNumber_1(self ) -> RetType;
}
impl<'a> /*trait*/ QChar_isNumber_1<bool> for (u32) {
  fn isNumber_1(self ) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const u32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QChar8isNumberEj", 1,qtrt::FFITY_UINT32,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qchar.h:451
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isLetterOrNumber() const

/*
Returns true if the character is a letter or number (Letter_* or Number_* categories); otherwise returns false.
*/
impl /*struct*/ QChar {
  pub fn isLetterOrNumber_0<RetType, T: QChar_isLetterOrNumber_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isLetterOrNumber_0(self);
    // return 1;
  }
}
pub trait QChar_isLetterOrNumber_0<RetType> {
  fn isLetterOrNumber_0(self , rsthis: & QChar) -> RetType;
}
impl<'a> /*trait*/ QChar_isLetterOrNumber_0<bool> for () {
  fn isLetterOrNumber_0(self , rsthis: & QChar) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QChar16isLetterOrNumberEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qchar.h:547
// index:1
// Public static inline Visibility=Default Availability=Available
// [1] bool isLetterOrNumber(uint)

/*
Returns true if the character is a letter or number (Letter_* or Number_* categories); otherwise returns false.
*/
impl /*struct*/ QChar {
  pub fn isLetterOrNumber_1<RetType, T: QChar_isLetterOrNumber_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.isLetterOrNumber_1();
    // return 1;
  }
}
pub trait QChar_isLetterOrNumber_1<RetType> {
  fn isLetterOrNumber_1(self ) -> RetType;
}
impl<'a> /*trait*/ QChar_isLetterOrNumber_1<bool> for (u32) {
  fn isLetterOrNumber_1(self ) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const u32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QChar16isLetterOrNumberEj", 1,qtrt::FFITY_UINT32,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qchar.h:452
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isDigit() const

/*
Returns true if the character is a decimal digit (Number_DecimalDigit); otherwise returns false.

See also isNumber().
*/
impl /*struct*/ QChar {
  pub fn isDigit_0<RetType, T: QChar_isDigit_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isDigit_0(self);
    // return 1;
  }
}
pub trait QChar_isDigit_0<RetType> {
  fn isDigit_0(self , rsthis: & QChar) -> RetType;
}
impl<'a> /*trait*/ QChar_isDigit_0<bool> for () {
  fn isDigit_0(self , rsthis: & QChar) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QChar7isDigitEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qchar.h:553
// index:1
// Public static inline Visibility=Default Availability=Available
// [1] bool isDigit(uint)

/*
Returns true if the character is a decimal digit (Number_DecimalDigit); otherwise returns false.

See also isNumber().
*/
impl /*struct*/ QChar {
  pub fn isDigit_1<RetType, T: QChar_isDigit_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.isDigit_1();
    // return 1;
  }
}
pub trait QChar_isDigit_1<RetType> {
  fn isDigit_1(self ) -> RetType;
}
impl<'a> /*trait*/ QChar_isDigit_1<bool> for (u32) {
  fn isDigit_1(self ) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const u32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QChar7isDigitEj", 1,qtrt::FFITY_UINT32,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qchar.h:453
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isLower() const

/*
Returns true if the character is a lowercase letter, for example category() is Letter_Lowercase.

See also isUpper(), toLower(), and toUpper().
*/
impl /*struct*/ QChar {
  pub fn isLower_0<RetType, T: QChar_isLower_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isLower_0(self);
    // return 1;
  }
}
pub trait QChar_isLower_0<RetType> {
  fn isLower_0(self , rsthis: & QChar) -> RetType;
}
impl<'a> /*trait*/ QChar_isLower_0<bool> for () {
  fn isLower_0(self , rsthis: & QChar) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QChar7isLowerEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qchar.h:555
// index:1
// Public static inline Visibility=Default Availability=Available
// [1] bool isLower(uint)

/*
Returns true if the character is a lowercase letter, for example category() is Letter_Lowercase.

See also isUpper(), toLower(), and toUpper().
*/
impl /*struct*/ QChar {
  pub fn isLower_1<RetType, T: QChar_isLower_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.isLower_1();
    // return 1;
  }
}
pub trait QChar_isLower_1<RetType> {
  fn isLower_1(self ) -> RetType;
}
impl<'a> /*trait*/ QChar_isLower_1<bool> for (u32) {
  fn isLower_1(self ) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const u32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QChar7isLowerEj", 1,qtrt::FFITY_UINT32,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qchar.h:454
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isUpper() const

/*
Returns true if the character is an uppercase letter, for example category() is Letter_Uppercase.

See also isLower(), toUpper(), and toLower().
*/
impl /*struct*/ QChar {
  pub fn isUpper_0<RetType, T: QChar_isUpper_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isUpper_0(self);
    // return 1;
  }
}
pub trait QChar_isUpper_0<RetType> {
  fn isUpper_0(self , rsthis: & QChar) -> RetType;
}
impl<'a> /*trait*/ QChar_isUpper_0<bool> for () {
  fn isUpper_0(self , rsthis: & QChar) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QChar7isUpperEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qchar.h:557
// index:1
// Public static inline Visibility=Default Availability=Available
// [1] bool isUpper(uint)

/*
Returns true if the character is an uppercase letter, for example category() is Letter_Uppercase.

See also isLower(), toUpper(), and toLower().
*/
impl /*struct*/ QChar {
  pub fn isUpper_1<RetType, T: QChar_isUpper_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.isUpper_1();
    // return 1;
  }
}
pub trait QChar_isUpper_1<RetType> {
  fn isUpper_1(self ) -> RetType;
}
impl<'a> /*trait*/ QChar_isUpper_1<bool> for (u32) {
  fn isUpper_1(self ) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const u32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QChar7isUpperEj", 1,qtrt::FFITY_UINT32,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qchar.h:455
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isTitleCase() const

/*
Returns true if the character is a titlecase letter, for example category() is Letter_Titlecase.

See also isLower(), toUpper(), toLower(), and toTitleCase().
*/
impl /*struct*/ QChar {
  pub fn isTitleCase_0<RetType, T: QChar_isTitleCase_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isTitleCase_0(self);
    // return 1;
  }
}
pub trait QChar_isTitleCase_0<RetType> {
  fn isTitleCase_0(self , rsthis: & QChar) -> RetType;
}
impl<'a> /*trait*/ QChar_isTitleCase_0<bool> for () {
  fn isTitleCase_0(self , rsthis: & QChar) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QChar11isTitleCaseEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qchar.h:559
// index:1
// Public static inline Visibility=Default Availability=Available
// [1] bool isTitleCase(uint)

/*
Returns true if the character is a titlecase letter, for example category() is Letter_Titlecase.

See also isLower(), toUpper(), toLower(), and toTitleCase().
*/
impl /*struct*/ QChar {
  pub fn isTitleCase_1<RetType, T: QChar_isTitleCase_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.isTitleCase_1();
    // return 1;
  }
}
pub trait QChar_isTitleCase_1<RetType> {
  fn isTitleCase_1(self ) -> RetType;
}
impl<'a> /*trait*/ QChar_isTitleCase_1<bool> for (u32) {
  fn isTitleCase_1(self ) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const u32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QChar11isTitleCaseEj", 1,qtrt::FFITY_UINT32,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qchar.h:457
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isNonCharacter() const

/*
Returns true if the QChar is a non-character; false otherwise.

Unicode has a certain number of code points that are classified as "non-characters:" that is, they can be used for internal purposes in applications but cannot be used for text interchange. Those are the last two entries each Unicode Plane ([0xfffe..0xffff], [0x1fffe..0x1ffff], etc.) as well as the entries in range [0xfdd0..0xfdef].

This function was introduced in  Qt 5.0.
*/
impl /*struct*/ QChar {
  pub fn isNonCharacter_0<RetType, T: QChar_isNonCharacter_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isNonCharacter_0(self);
    // return 1;
  }
}
pub trait QChar_isNonCharacter_0<RetType> {
  fn isNonCharacter_0(self , rsthis: & QChar) -> RetType;
}
impl<'a> /*trait*/ QChar_isNonCharacter_0<bool> for () {
  fn isNonCharacter_0(self , rsthis: & QChar) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QChar14isNonCharacterEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qchar.h:467
// index:1
// Public static inline Visibility=Default Availability=Available
// [1] bool isNonCharacter(uint)

/*
Returns true if the QChar is a non-character; false otherwise.

Unicode has a certain number of code points that are classified as "non-characters:" that is, they can be used for internal purposes in applications but cannot be used for text interchange. Those are the last two entries each Unicode Plane ([0xfffe..0xffff], [0x1fffe..0x1ffff], etc.) as well as the entries in range [0xfdd0..0xfdef].

This function was introduced in  Qt 5.0.
*/
impl /*struct*/ QChar {
  pub fn isNonCharacter_1<RetType, T: QChar_isNonCharacter_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.isNonCharacter_1();
    // return 1;
  }
}
pub trait QChar_isNonCharacter_1<RetType> {
  fn isNonCharacter_1(self ) -> RetType;
}
impl<'a> /*trait*/ QChar_isNonCharacter_1<bool> for (u32) {
  fn isNonCharacter_1(self ) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const u32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QChar14isNonCharacterEj", 1,qtrt::FFITY_UINT32,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qchar.h:458
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isHighSurrogate() const

/*
Returns true if the QChar is the high part of a UTF16 surrogate (for example if its code point is in range [0xd800..0xdbff]); false otherwise.
*/
impl /*struct*/ QChar {
  pub fn isHighSurrogate_0<RetType, T: QChar_isHighSurrogate_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isHighSurrogate_0(self);
    // return 1;
  }
}
pub trait QChar_isHighSurrogate_0<RetType> {
  fn isHighSurrogate_0(self , rsthis: & QChar) -> RetType;
}
impl<'a> /*trait*/ QChar_isHighSurrogate_0<bool> for () {
  fn isHighSurrogate_0(self , rsthis: & QChar) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QChar15isHighSurrogateEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qchar.h:471
// index:1
// Public static inline Visibility=Default Availability=Available
// [1] bool isHighSurrogate(uint)

/*
Returns true if the QChar is the high part of a UTF16 surrogate (for example if its code point is in range [0xd800..0xdbff]); false otherwise.
*/
impl /*struct*/ QChar {
  pub fn isHighSurrogate_1<RetType, T: QChar_isHighSurrogate_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.isHighSurrogate_1();
    // return 1;
  }
}
pub trait QChar_isHighSurrogate_1<RetType> {
  fn isHighSurrogate_1(self ) -> RetType;
}
impl<'a> /*trait*/ QChar_isHighSurrogate_1<bool> for (u32) {
  fn isHighSurrogate_1(self ) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const u32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QChar15isHighSurrogateEj", 1,qtrt::FFITY_UINT32,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qchar.h:459
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isLowSurrogate() const

/*
Returns true if the QChar is the low part of a UTF16 surrogate (for example if its code point is in range [0xdc00..0xdfff]); false otherwise.
*/
impl /*struct*/ QChar {
  pub fn isLowSurrogate_0<RetType, T: QChar_isLowSurrogate_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isLowSurrogate_0(self);
    // return 1;
  }
}
pub trait QChar_isLowSurrogate_0<RetType> {
  fn isLowSurrogate_0(self , rsthis: & QChar) -> RetType;
}
impl<'a> /*trait*/ QChar_isLowSurrogate_0<bool> for () {
  fn isLowSurrogate_0(self , rsthis: & QChar) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QChar14isLowSurrogateEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qchar.h:475
// index:1
// Public static inline Visibility=Default Availability=Available
// [1] bool isLowSurrogate(uint)

/*
Returns true if the QChar is the low part of a UTF16 surrogate (for example if its code point is in range [0xdc00..0xdfff]); false otherwise.
*/
impl /*struct*/ QChar {
  pub fn isLowSurrogate_1<RetType, T: QChar_isLowSurrogate_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.isLowSurrogate_1();
    // return 1;
  }
}
pub trait QChar_isLowSurrogate_1<RetType> {
  fn isLowSurrogate_1(self ) -> RetType;
}
impl<'a> /*trait*/ QChar_isLowSurrogate_1<bool> for (u32) {
  fn isLowSurrogate_1(self ) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const u32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QChar14isLowSurrogateEj", 1,qtrt::FFITY_UINT32,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qchar.h:460
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isSurrogate() const

/*
Returns true if the QChar contains a code point that is in either the high or the low part of the UTF-16 surrogate range (for example if its code point is in range [0xd800..0xdfff]); false otherwise.

This function was introduced in  Qt 5.0.
*/
impl /*struct*/ QChar {
  pub fn isSurrogate_0<RetType, T: QChar_isSurrogate_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isSurrogate_0(self);
    // return 1;
  }
}
pub trait QChar_isSurrogate_0<RetType> {
  fn isSurrogate_0(self , rsthis: & QChar) -> RetType;
}
impl<'a> /*trait*/ QChar_isSurrogate_0<bool> for () {
  fn isSurrogate_0(self , rsthis: & QChar) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QChar11isSurrogateEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qchar.h:479
// index:1
// Public static inline Visibility=Default Availability=Available
// [1] bool isSurrogate(uint)

/*
Returns true if the QChar contains a code point that is in either the high or the low part of the UTF-16 surrogate range (for example if its code point is in range [0xd800..0xdfff]); false otherwise.

This function was introduced in  Qt 5.0.
*/
impl /*struct*/ QChar {
  pub fn isSurrogate_1<RetType, T: QChar_isSurrogate_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.isSurrogate_1();
    // return 1;
  }
}
pub trait QChar_isSurrogate_1<RetType> {
  fn isSurrogate_1(self ) -> RetType;
}
impl<'a> /*trait*/ QChar_isSurrogate_1<bool> for (u32) {
  fn isSurrogate_1(self ) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const u32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QChar11isSurrogateEj", 1,qtrt::FFITY_UINT32,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qchar.h:462
// index:0
// Public inline Visibility=Default Availability=Available
// [1] uchar cell() const

/*
Returns the cell (least significant byte) of the Unicode character.

See also row().
*/
impl /*struct*/ QChar {
  pub fn cell_0<RetType, T: QChar_cell_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cell_0(self);
    // return 1;
  }
}
pub trait QChar_cell_0<RetType> {
  fn cell_0(self , rsthis: & QChar) -> RetType;
}
impl<'a> /*trait*/ QChar_cell_0<u8> for () {
  fn cell_0(self , rsthis: & QChar) -> u8 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QChar4cellEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u8 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qchar.h:463
// index:0
// Public inline Visibility=Default Availability=Available
// [1] uchar row() const

/*
Returns the row (most significant byte) of the Unicode character.

See also cell().
*/
impl /*struct*/ QChar {
  pub fn row_0<RetType, T: QChar_row_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.row_0(self);
    // return 1;
  }
}
pub trait QChar_row_0<RetType> {
  fn row_0(self , rsthis: & QChar) -> RetType;
}
impl<'a> /*trait*/ QChar_row_0<u8> for () {
  fn row_0(self , rsthis: & QChar) -> u8 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK5QChar3rowEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u8 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qchar.h:464
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setCell(uchar)

/*

*/
impl /*struct*/ QChar {
  pub fn setCell_0<RetType, T: QChar_setCell_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCell_0(self);
    // return 1;
  }
}
pub trait QChar_setCell_0<RetType> {
  fn setCell_0(self , rsthis: & QChar) -> RetType;
}
impl<'a> /*trait*/ QChar_setCell_0<(/*void*/)> for (u8) {
  fn setCell_0(self , rsthis: & QChar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const u8 as usize;
     qtrt::InvokeQtFunc6("_ZN5QChar7setCellEh", 1,qtrt::FFITY_UINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qchar.h:465
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setRow(uchar)

/*

*/
impl /*struct*/ QChar {
  pub fn setRow_0<RetType, T: QChar_setRow_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setRow_0(self);
    // return 1;
  }
}
pub trait QChar_setRow_0<RetType> {
  fn setRow_0(self , rsthis: & QChar) -> RetType;
}
impl<'a> /*trait*/ QChar_setRow_0<(/*void*/)> for (u8) {
  fn setRow_0(self , rsthis: & QChar) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const u8 as usize;
     qtrt::InvokeQtFunc6("_ZN5QChar6setRowEh", 1,qtrt::FFITY_UINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qchar.h:483
// index:0
// Public static inline Visibility=Default Availability=Available
// [1] bool requiresSurrogates(uint)

/*
Returns true if the UCS-4-encoded character specified by ucs4 can be split into the high and low parts of a UTF16 surrogate (for example if its code point is greater than or equals to 0x10000); false otherwise.
*/
impl /*struct*/ QChar {
  pub fn requiresSurrogates_0<RetType, T: QChar_requiresSurrogates_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.requiresSurrogates_0();
    // return 1;
  }
}
pub trait QChar_requiresSurrogates_0<RetType> {
  fn requiresSurrogates_0(self ) -> RetType;
}
impl<'a> /*trait*/ QChar_requiresSurrogates_0<bool> for (u32) {
  fn requiresSurrogates_0(self ) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const u32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QChar18requiresSurrogatesEj", 1,qtrt::FFITY_UINT32,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qchar.h:487
// index:0
// Public static inline Visibility=Default Availability=Available
// [4] uint surrogateToUcs4(ushort, ushort)

/*
Converts a UTF16 surrogate pair with the given high and low values to it's UCS-4-encoded code point.
*/
impl /*struct*/ QChar {
  pub fn surrogateToUcs4_0<RetType, T: QChar_surrogateToUcs4_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.surrogateToUcs4_0();
    // return 1;
  }
}
pub trait QChar_surrogateToUcs4_0<RetType> {
  fn surrogateToUcs4_0(self ) -> RetType;
}
impl<'a> /*trait*/ QChar_surrogateToUcs4_0<u32> for (u16,u16) {
  fn surrogateToUcs4_0(self ) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const u16 as usize;
    let arg1 = (&self.1) as *const u16 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QChar15surrogateToUcs4Ett", 2,qtrt::FFITY_UINT16,qtrt::FFITY_UINT16,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qchar.h:491
// index:1
// Public static inline Visibility=Default Availability=Available
// [4] uint surrogateToUcs4(QChar, QChar)

/*
Converts a UTF16 surrogate pair with the given high and low values to it's UCS-4-encoded code point.
*/
impl /*struct*/ QChar {
  pub fn surrogateToUcs4_1<RetType, T: QChar_surrogateToUcs4_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.surrogateToUcs4_1();
    // return 1;
  }
}
pub trait QChar_surrogateToUcs4_1<RetType> {
  fn surrogateToUcs4_1(self ) -> RetType;
}
impl<'a> /*trait*/ QChar_surrogateToUcs4_1<u32> for (usize,usize) {
  fn surrogateToUcs4_1(self ) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QChar15surrogateToUcs4ES_S_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qchar.h:495
// index:0
// Public static inline Visibility=Default Availability=Available
// [2] ushort highSurrogate(uint)

/*
Returns the high surrogate part of a UCS-4-encoded code point. The returned result is undefined if ucs4 is smaller than 0x10000.
*/
impl /*struct*/ QChar {
  pub fn highSurrogate_0<RetType, T: QChar_highSurrogate_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.highSurrogate_0();
    // return 1;
  }
}
pub trait QChar_highSurrogate_0<RetType> {
  fn highSurrogate_0(self ) -> RetType;
}
impl<'a> /*trait*/ QChar_highSurrogate_0<u16> for (u32) {
  fn highSurrogate_0(self ) -> u16 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const u32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QChar13highSurrogateEj", 1,qtrt::FFITY_UINT32,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u16 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qchar.h:499
// index:0
// Public static inline Visibility=Default Availability=Available
// [2] ushort lowSurrogate(uint)

/*
Returns the low surrogate part of a UCS-4-encoded code point. The returned result is undefined if ucs4 is smaller than 0x10000.
*/
impl /*struct*/ QChar {
  pub fn lowSurrogate_0<RetType, T: QChar_lowSurrogate_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.lowSurrogate_0();
    // return 1;
  }
}
pub trait QChar_lowSurrogate_0<RetType> {
  fn lowSurrogate_0(self ) -> RetType;
}
impl<'a> /*trait*/ QChar_lowSurrogate_0<u16> for (u32) {
  fn lowSurrogate_0(self ) -> u16 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const u32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QChar12lowSurrogateEj", 1,qtrt::FFITY_UINT32,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u16 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qchar.h:528
// index:0
// Public static Visibility=Default Availability=Available
// [4] QChar::UnicodeVersion currentUnicodeVersion()

/*
Returns the most recent supported Unicode version.
*/
impl /*struct*/ QChar {
  pub fn currentUnicodeVersion_0<RetType, T: QChar_currentUnicodeVersion_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.currentUnicodeVersion_0();
    // return 1;
  }
}
pub trait QChar_currentUnicodeVersion_0<RetType> {
  fn currentUnicodeVersion_0(self ) -> RetType;
}
impl<'a> /*trait*/ QChar_currentUnicodeVersion_0<i32> for () {
  fn currentUnicodeVersion_0(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN5QChar21currentUnicodeVersionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}


pub fn DeleteQChar(this :*mut QChar) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN5QCharD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}

/*

*/
pub type QChar__SpecialCharacter = i32;
// 
pub const QChar__Null :QChar__SpecialCharacter = 0;
// 
pub const QChar__Tabulation :QChar__SpecialCharacter = 9;
// 
pub const QChar__LineFeed :QChar__SpecialCharacter = 10;
// 
pub const QChar__CarriageReturn :QChar__SpecialCharacter = 13;
// 
pub const QChar__Space :QChar__SpecialCharacter = 32;
// 
pub const QChar__Nbsp :QChar__SpecialCharacter = 160;
// 
pub const QChar__SoftHyphen :QChar__SpecialCharacter = 173;
// xfffdThe character shown when a font has no glyph for a certain codepoint. A special question mark character is often used. Codecs use this codepoint when input data cannot be represented in Unicode.
pub const QChar__ReplacementCharacter :QChar__SpecialCharacter = 65533;
// xfffcUsed to represent an object such as an image when such objects cannot be presented.
pub const QChar__ObjectReplacementCharacter :QChar__SpecialCharacter = 65532;
// xfeff 
pub const QChar__ByteOrderMark :QChar__SpecialCharacter = 65279;
// xfffe 
pub const QChar__ByteOrderSwapped :QChar__SpecialCharacter = 65534;
// 
pub const QChar__ParagraphSeparator :QChar__SpecialCharacter = 8233;
// 
pub const QChar__LineSeparator :QChar__SpecialCharacter = 8232;
// 
pub const QChar__LastValidCodePoint :QChar__SpecialCharacter = 1114111;
pub fn QChar_SpecialCharacterItemName(val: i32) ->String {
  match val {
     QChar__Null => // 0
     {return String::from("Null");}
     QChar__Tabulation => // 9
     {return String::from("Tabulation");}
     QChar__LineFeed => // 10
     {return String::from("LineFeed");}
     QChar__CarriageReturn => // 13
     {return String::from("CarriageReturn");}
     QChar__Space => // 32
     {return String::from("Space");}
     QChar__Nbsp => // 160
     {return String::from("Nbsp");}
     QChar__SoftHyphen => // 173
     {return String::from("SoftHyphen");}
     QChar__ReplacementCharacter => // 65533
     {return String::from("ReplacementCharacter");}
     QChar__ObjectReplacementCharacter => // 65532
     {return String::from("ObjectReplacementCharacter");}
     QChar__ByteOrderMark => // 65279
     {return String::from("ByteOrderMark");}
     QChar__ByteOrderSwapped => // 65534
     {return String::from("ByteOrderSwapped");}
     QChar__ParagraphSeparator => // 8233
     {return String::from("ParagraphSeparator");}
     QChar__LineSeparator => // 8232
     {return String::from("LineSeparator");}
     QChar__LastValidCodePoint => // 1114111
     {return String::from("LastValidCodePoint");}
  _ => {return format!("{}", val);}
}
}
pub fn QChar_SpecialCharacterItemName_s(val: i32) ->String {
  //var nilthis *QChar
  //return nilthis.SpecialCharacterItemName(val);
  return QChar_SpecialCharacterItemName(val);
}


/*
This enum maps the Unicode character categories.

The following characters are normative in Unicode:



The following categories are informative in Unicode:



See also category().

*/
pub type QChar__Category = i32;
// Unicode class name Mn
pub const QChar__Mark_NonSpacing :QChar__Category = 0;
// Unicode class name Mc
pub const QChar__Mark_SpacingCombining :QChar__Category = 1;
// Unicode class name Me
pub const QChar__Mark_Enclosing :QChar__Category = 2;
// Unicode class name Nd
pub const QChar__Number_DecimalDigit :QChar__Category = 3;
// Unicode class name Nl
pub const QChar__Number_Letter :QChar__Category = 4;
// Unicode class name No
pub const QChar__Number_Other :QChar__Category = 5;
// Unicode class name Zs
pub const QChar__Separator_Space :QChar__Category = 6;
// Unicode class name Zl
pub const QChar__Separator_Line :QChar__Category = 7;
// Unicode class name Zp
pub const QChar__Separator_Paragraph :QChar__Category = 8;
// Unicode class name Cc
pub const QChar__Other_Control :QChar__Category = 9;
// 
pub const QChar__Other_Format :QChar__Category = 10;
// 
pub const QChar__Other_Surrogate :QChar__Category = 11;
// 
pub const QChar__Other_PrivateUse :QChar__Category = 12;
// 
pub const QChar__Other_NotAssigned :QChar__Category = 13;
// 
pub const QChar__Letter_Uppercase :QChar__Category = 14;
// 
pub const QChar__Letter_Lowercase :QChar__Category = 15;
// 
pub const QChar__Letter_Titlecase :QChar__Category = 16;
// 
pub const QChar__Letter_Modifier :QChar__Category = 17;
// 
pub const QChar__Letter_Other :QChar__Category = 18;
// 
pub const QChar__Punctuation_Connector :QChar__Category = 19;
// 
pub const QChar__Punctuation_Dash :QChar__Category = 20;
// 
pub const QChar__Punctuation_Open :QChar__Category = 21;
// 
pub const QChar__Punctuation_Close :QChar__Category = 22;
// 
pub const QChar__Punctuation_InitialQuote :QChar__Category = 23;
// 
pub const QChar__Punctuation_FinalQuote :QChar__Category = 24;
// 
pub const QChar__Punctuation_Other :QChar__Category = 25;
// 
pub const QChar__Symbol_Math :QChar__Category = 26;
// 
pub const QChar__Symbol_Currency :QChar__Category = 27;
// 
pub const QChar__Symbol_Modifier :QChar__Category = 28;
// 
pub const QChar__Symbol_Other :QChar__Category = 29;
pub fn QChar_CategoryItemName(val: i32) ->String {
  match val {
     QChar__Mark_NonSpacing => // 0
     {return String::from("Mark_NonSpacing");}
     QChar__Mark_SpacingCombining => // 1
     {return String::from("Mark_SpacingCombining");}
     QChar__Mark_Enclosing => // 2
     {return String::from("Mark_Enclosing");}
     QChar__Number_DecimalDigit => // 3
     {return String::from("Number_DecimalDigit");}
     QChar__Number_Letter => // 4
     {return String::from("Number_Letter");}
     QChar__Number_Other => // 5
     {return String::from("Number_Other");}
     QChar__Separator_Space => // 6
     {return String::from("Separator_Space");}
     QChar__Separator_Line => // 7
     {return String::from("Separator_Line");}
     QChar__Separator_Paragraph => // 8
     {return String::from("Separator_Paragraph");}
     QChar__Other_Control => // 9
     {return String::from("Other_Control");}
     QChar__Other_Format => // 10
     {return String::from("Other_Format");}
     QChar__Other_Surrogate => // 11
     {return String::from("Other_Surrogate");}
     QChar__Other_PrivateUse => // 12
     {return String::from("Other_PrivateUse");}
     QChar__Other_NotAssigned => // 13
     {return String::from("Other_NotAssigned");}
     QChar__Letter_Uppercase => // 14
     {return String::from("Letter_Uppercase");}
     QChar__Letter_Lowercase => // 15
     {return String::from("Letter_Lowercase");}
     QChar__Letter_Titlecase => // 16
     {return String::from("Letter_Titlecase");}
     QChar__Letter_Modifier => // 17
     {return String::from("Letter_Modifier");}
     QChar__Letter_Other => // 18
     {return String::from("Letter_Other");}
     QChar__Punctuation_Connector => // 19
     {return String::from("Punctuation_Connector");}
     QChar__Punctuation_Dash => // 20
     {return String::from("Punctuation_Dash");}
     QChar__Punctuation_Open => // 21
     {return String::from("Punctuation_Open");}
     QChar__Punctuation_Close => // 22
     {return String::from("Punctuation_Close");}
     QChar__Punctuation_InitialQuote => // 23
     {return String::from("Punctuation_InitialQuote");}
     QChar__Punctuation_FinalQuote => // 24
     {return String::from("Punctuation_FinalQuote");}
     QChar__Punctuation_Other => // 25
     {return String::from("Punctuation_Other");}
     QChar__Symbol_Math => // 26
     {return String::from("Symbol_Math");}
     QChar__Symbol_Currency => // 27
     {return String::from("Symbol_Currency");}
     QChar__Symbol_Modifier => // 28
     {return String::from("Symbol_Modifier");}
     QChar__Symbol_Other => // 29
     {return String::from("Symbol_Other");}
  _ => {return format!("{}", val);}
}
}
pub fn QChar_CategoryItemName_s(val: i32) ->String {
  //var nilthis *QChar
  //return nilthis.CategoryItemName(val);
  return QChar_CategoryItemName(val);
}


/*
This enum type defines the Unicode script property values.

For details about the Unicode script property values see Unicode Standard Annex #24.

In order to conform to C/C++ naming conventions "Script_" is prepended to the codes used in the Unicode Standard.



This enum was introduced or modified in  Qt 5.1.

See also script().

*/
pub type QChar__Script = i32;
// For unassigned, private-use, noncharacter, and surrogate code points.
pub const QChar__Script_Unknown :QChar__Script = 0;
// For characters that may be used with multiple scripts and that inherit their script from the preceding characters. These include nonspacing marks, enclosing marks, and zero width joiner/non-joiner characters.
pub const QChar__Script_Inherited :QChar__Script = 1;
// For characters that may be used with multiple scripts and that do not inherit their script from the preceding characters.
pub const QChar__Script_Common :QChar__Script = 2;
//  
pub const QChar__Script_Latin :QChar__Script = 3;
//  
pub const QChar__Script_Greek :QChar__Script = 4;
//  
pub const QChar__Script_Cyrillic :QChar__Script = 5;
//  
pub const QChar__Script_Armenian :QChar__Script = 6;
//  
pub const QChar__Script_Hebrew :QChar__Script = 7;
//  
pub const QChar__Script_Arabic :QChar__Script = 8;
//  
pub const QChar__Script_Syriac :QChar__Script = 9;
// 
pub const QChar__Script_Thaana :QChar__Script = 10;
// 
pub const QChar__Script_Devanagari :QChar__Script = 11;
// 
pub const QChar__Script_Bengali :QChar__Script = 12;
// 
pub const QChar__Script_Gurmukhi :QChar__Script = 13;
// 
pub const QChar__Script_Gujarati :QChar__Script = 14;
// 
pub const QChar__Script_Oriya :QChar__Script = 15;
// 
pub const QChar__Script_Tamil :QChar__Script = 16;
// 
pub const QChar__Script_Telugu :QChar__Script = 17;
// 
pub const QChar__Script_Kannada :QChar__Script = 18;
// 
pub const QChar__Script_Malayalam :QChar__Script = 19;
// 
pub const QChar__Script_Sinhala :QChar__Script = 20;
// 
pub const QChar__Script_Thai :QChar__Script = 21;
// 
pub const QChar__Script_Lao :QChar__Script = 22;
// 
pub const QChar__Script_Tibetan :QChar__Script = 23;
// 
pub const QChar__Script_Myanmar :QChar__Script = 24;
// 
pub const QChar__Script_Georgian :QChar__Script = 25;
// 
pub const QChar__Script_Hangul :QChar__Script = 26;
// 
pub const QChar__Script_Ethiopic :QChar__Script = 27;
// 
pub const QChar__Script_Cherokee :QChar__Script = 28;
// 
pub const QChar__Script_CanadianAboriginal :QChar__Script = 29;
// 
pub const QChar__Script_Ogham :QChar__Script = 30;
// 
pub const QChar__Script_Runic :QChar__Script = 31;
// 
pub const QChar__Script_Khmer :QChar__Script = 32;
// 
pub const QChar__Script_Mongolian :QChar__Script = 33;
// 
pub const QChar__Script_Hiragana :QChar__Script = 34;
// 
pub const QChar__Script_Katakana :QChar__Script = 35;
// 
pub const QChar__Script_Bopomofo :QChar__Script = 36;
// 
pub const QChar__Script_Han :QChar__Script = 37;
// 
pub const QChar__Script_Yi :QChar__Script = 38;
// 
pub const QChar__Script_OldItalic :QChar__Script = 39;
// 
pub const QChar__Script_Gothic :QChar__Script = 40;
// 
pub const QChar__Script_Deseret :QChar__Script = 41;
// 
pub const QChar__Script_Tagalog :QChar__Script = 42;
// 
pub const QChar__Script_Hanunoo :QChar__Script = 43;
// 
pub const QChar__Script_Buhid :QChar__Script = 44;
// 
pub const QChar__Script_Tagbanwa :QChar__Script = 45;
// 
pub const QChar__Script_Coptic :QChar__Script = 46;
// 
pub const QChar__Script_Limbu :QChar__Script = 47;
// 
pub const QChar__Script_TaiLe :QChar__Script = 48;
// 
pub const QChar__Script_LinearB :QChar__Script = 49;
// 
pub const QChar__Script_Ugaritic :QChar__Script = 50;
// 
pub const QChar__Script_Shavian :QChar__Script = 51;
// 
pub const QChar__Script_Osmanya :QChar__Script = 52;
// 
pub const QChar__Script_Cypriot :QChar__Script = 53;
// 
pub const QChar__Script_Braille :QChar__Script = 54;
// 
pub const QChar__Script_Buginese :QChar__Script = 55;
// 
pub const QChar__Script_NewTaiLue :QChar__Script = 56;
// 
pub const QChar__Script_Glagolitic :QChar__Script = 57;
// 
pub const QChar__Script_Tifinagh :QChar__Script = 58;
// 
pub const QChar__Script_SylotiNagri :QChar__Script = 59;
// 
pub const QChar__Script_OldPersian :QChar__Script = 60;
// 
pub const QChar__Script_Kharoshthi :QChar__Script = 61;
// 
pub const QChar__Script_Balinese :QChar__Script = 62;
// 
pub const QChar__Script_Cuneiform :QChar__Script = 63;
// 
pub const QChar__Script_Phoenician :QChar__Script = 64;
// 
pub const QChar__Script_PhagsPa :QChar__Script = 65;
// 
pub const QChar__Script_Nko :QChar__Script = 66;
// 
pub const QChar__Script_Sundanese :QChar__Script = 67;
// 
pub const QChar__Script_Lepcha :QChar__Script = 68;
// 
pub const QChar__Script_OlChiki :QChar__Script = 69;
// 
pub const QChar__Script_Vai :QChar__Script = 70;
// 
pub const QChar__Script_Saurashtra :QChar__Script = 71;
// 
pub const QChar__Script_KayahLi :QChar__Script = 72;
// 
pub const QChar__Script_Rejang :QChar__Script = 73;
// 
pub const QChar__Script_Lycian :QChar__Script = 74;
// 
pub const QChar__Script_Carian :QChar__Script = 75;
// 
pub const QChar__Script_Lydian :QChar__Script = 76;
// 
pub const QChar__Script_Cham :QChar__Script = 77;
// 
pub const QChar__Script_TaiTham :QChar__Script = 78;
// 
pub const QChar__Script_TaiViet :QChar__Script = 79;
// 
pub const QChar__Script_Avestan :QChar__Script = 80;
// 
pub const QChar__Script_EgyptianHieroglyphs :QChar__Script = 81;
// 
pub const QChar__Script_Samaritan :QChar__Script = 82;
// 
pub const QChar__Script_Lisu :QChar__Script = 83;
// 
pub const QChar__Script_Bamum :QChar__Script = 84;
// 
pub const QChar__Script_Javanese :QChar__Script = 85;
// 
pub const QChar__Script_MeeteiMayek :QChar__Script = 86;
// 
pub const QChar__Script_ImperialAramaic :QChar__Script = 87;
// 
pub const QChar__Script_OldSouthArabian :QChar__Script = 88;
// 
pub const QChar__Script_InscriptionalParthian :QChar__Script = 89;
// 
pub const QChar__Script_InscriptionalPahlavi :QChar__Script = 90;
// 
pub const QChar__Script_OldTurkic :QChar__Script = 91;
// 
pub const QChar__Script_Kaithi :QChar__Script = 92;
// 
pub const QChar__Script_Batak :QChar__Script = 93;
// 
pub const QChar__Script_Brahmi :QChar__Script = 94;
// 
pub const QChar__Script_Mandaic :QChar__Script = 95;
// 
pub const QChar__Script_Chakma :QChar__Script = 96;
// 
pub const QChar__Script_MeroiticCursive :QChar__Script = 97;
// 
pub const QChar__Script_MeroiticHieroglyphs :QChar__Script = 98;
// 
pub const QChar__Script_Miao :QChar__Script = 99;
// 
pub const QChar__Script_Sharada :QChar__Script = 100;
// 
pub const QChar__Script_SoraSompeng :QChar__Script = 101;
// 
pub const QChar__Script_Takri :QChar__Script = 102;
// 
pub const QChar__Script_CaucasianAlbanian :QChar__Script = 103;
// 
pub const QChar__Script_BassaVah :QChar__Script = 104;
// 
pub const QChar__Script_Duployan :QChar__Script = 105;
// 
pub const QChar__Script_Elbasan :QChar__Script = 106;
// 
pub const QChar__Script_Grantha :QChar__Script = 107;
// 
pub const QChar__Script_PahawhHmong :QChar__Script = 108;
// 
pub const QChar__Script_Khojki :QChar__Script = 109;
// 
pub const QChar__Script_LinearA :QChar__Script = 110;
// 
pub const QChar__Script_Mahajani :QChar__Script = 111;
// 
pub const QChar__Script_Manichaean :QChar__Script = 112;
// 
pub const QChar__Script_MendeKikakui :QChar__Script = 113;
// 
pub const QChar__Script_Modi :QChar__Script = 114;
// 
pub const QChar__Script_Mro :QChar__Script = 115;
// 
pub const QChar__Script_OldNorthArabian :QChar__Script = 116;
// 
pub const QChar__Script_Nabataean :QChar__Script = 117;
// 
pub const QChar__Script_Palmyrene :QChar__Script = 118;
// 
pub const QChar__Script_PauCinHau :QChar__Script = 119;
// 
pub const QChar__Script_OldPermic :QChar__Script = 120;
// 
pub const QChar__Script_PsalterPahlavi :QChar__Script = 121;
// 
pub const QChar__Script_Siddham :QChar__Script = 122;
// 
pub const QChar__Script_Khudawadi :QChar__Script = 123;
// 
pub const QChar__Script_Tirhuta :QChar__Script = 124;
// 
pub const QChar__Script_WarangCiti :QChar__Script = 125;
// 
pub const QChar__Script_Ahom :QChar__Script = 126;
// 
pub const QChar__Script_AnatolianHieroglyphs :QChar__Script = 127;
// 
pub const QChar__Script_Hatran :QChar__Script = 128;
// 
pub const QChar__Script_Multani :QChar__Script = 129;
// 
pub const QChar__Script_OldHungarian :QChar__Script = 130;
// 
pub const QChar__Script_SignWriting :QChar__Script = 131;
// 
pub const QChar__ScriptCount :QChar__Script = 132;
pub fn QChar_ScriptItemName(val: i32) ->String {
  match val {
     QChar__Script_Unknown => // 0
     {return String::from("Script_Unknown");}
     QChar__Script_Inherited => // 1
     {return String::from("Script_Inherited");}
     QChar__Script_Common => // 2
     {return String::from("Script_Common");}
     QChar__Script_Latin => // 3
     {return String::from("Script_Latin");}
     QChar__Script_Greek => // 4
     {return String::from("Script_Greek");}
     QChar__Script_Cyrillic => // 5
     {return String::from("Script_Cyrillic");}
     QChar__Script_Armenian => // 6
     {return String::from("Script_Armenian");}
     QChar__Script_Hebrew => // 7
     {return String::from("Script_Hebrew");}
     QChar__Script_Arabic => // 8
     {return String::from("Script_Arabic");}
     QChar__Script_Syriac => // 9
     {return String::from("Script_Syriac");}
     QChar__Script_Thaana => // 10
     {return String::from("Script_Thaana");}
     QChar__Script_Devanagari => // 11
     {return String::from("Script_Devanagari");}
     QChar__Script_Bengali => // 12
     {return String::from("Script_Bengali");}
     QChar__Script_Gurmukhi => // 13
     {return String::from("Script_Gurmukhi");}
     QChar__Script_Gujarati => // 14
     {return String::from("Script_Gujarati");}
     QChar__Script_Oriya => // 15
     {return String::from("Script_Oriya");}
     QChar__Script_Tamil => // 16
     {return String::from("Script_Tamil");}
     QChar__Script_Telugu => // 17
     {return String::from("Script_Telugu");}
     QChar__Script_Kannada => // 18
     {return String::from("Script_Kannada");}
     QChar__Script_Malayalam => // 19
     {return String::from("Script_Malayalam");}
     QChar__Script_Sinhala => // 20
     {return String::from("Script_Sinhala");}
     QChar__Script_Thai => // 21
     {return String::from("Script_Thai");}
     QChar__Script_Lao => // 22
     {return String::from("Script_Lao");}
     QChar__Script_Tibetan => // 23
     {return String::from("Script_Tibetan");}
     QChar__Script_Myanmar => // 24
     {return String::from("Script_Myanmar");}
     QChar__Script_Georgian => // 25
     {return String::from("Script_Georgian");}
     QChar__Script_Hangul => // 26
     {return String::from("Script_Hangul");}
     QChar__Script_Ethiopic => // 27
     {return String::from("Script_Ethiopic");}
     QChar__Script_Cherokee => // 28
     {return String::from("Script_Cherokee");}
     QChar__Script_CanadianAboriginal => // 29
     {return String::from("Script_CanadianAboriginal");}
     QChar__Script_Ogham => // 30
     {return String::from("Script_Ogham");}
     QChar__Script_Runic => // 31
     {return String::from("Script_Runic");}
     QChar__Script_Khmer => // 32
     {return String::from("Script_Khmer");}
     QChar__Script_Mongolian => // 33
     {return String::from("Script_Mongolian");}
     QChar__Script_Hiragana => // 34
     {return String::from("Script_Hiragana");}
     QChar__Script_Katakana => // 35
     {return String::from("Script_Katakana");}
     QChar__Script_Bopomofo => // 36
     {return String::from("Script_Bopomofo");}
     QChar__Script_Han => // 37
     {return String::from("Script_Han");}
     QChar__Script_Yi => // 38
     {return String::from("Script_Yi");}
     QChar__Script_OldItalic => // 39
     {return String::from("Script_OldItalic");}
     QChar__Script_Gothic => // 40
     {return String::from("Script_Gothic");}
     QChar__Script_Deseret => // 41
     {return String::from("Script_Deseret");}
     QChar__Script_Tagalog => // 42
     {return String::from("Script_Tagalog");}
     QChar__Script_Hanunoo => // 43
     {return String::from("Script_Hanunoo");}
     QChar__Script_Buhid => // 44
     {return String::from("Script_Buhid");}
     QChar__Script_Tagbanwa => // 45
     {return String::from("Script_Tagbanwa");}
     QChar__Script_Coptic => // 46
     {return String::from("Script_Coptic");}
     QChar__Script_Limbu => // 47
     {return String::from("Script_Limbu");}
     QChar__Script_TaiLe => // 48
     {return String::from("Script_TaiLe");}
     QChar__Script_LinearB => // 49
     {return String::from("Script_LinearB");}
     QChar__Script_Ugaritic => // 50
     {return String::from("Script_Ugaritic");}
     QChar__Script_Shavian => // 51
     {return String::from("Script_Shavian");}
     QChar__Script_Osmanya => // 52
     {return String::from("Script_Osmanya");}
     QChar__Script_Cypriot => // 53
     {return String::from("Script_Cypriot");}
     QChar__Script_Braille => // 54
     {return String::from("Script_Braille");}
     QChar__Script_Buginese => // 55
     {return String::from("Script_Buginese");}
     QChar__Script_NewTaiLue => // 56
     {return String::from("Script_NewTaiLue");}
     QChar__Script_Glagolitic => // 57
     {return String::from("Script_Glagolitic");}
     QChar__Script_Tifinagh => // 58
     {return String::from("Script_Tifinagh");}
     QChar__Script_SylotiNagri => // 59
     {return String::from("Script_SylotiNagri");}
     QChar__Script_OldPersian => // 60
     {return String::from("Script_OldPersian");}
     QChar__Script_Kharoshthi => // 61
     {return String::from("Script_Kharoshthi");}
     QChar__Script_Balinese => // 62
     {return String::from("Script_Balinese");}
     QChar__Script_Cuneiform => // 63
     {return String::from("Script_Cuneiform");}
     QChar__Script_Phoenician => // 64
     {return String::from("Script_Phoenician");}
     QChar__Script_PhagsPa => // 65
     {return String::from("Script_PhagsPa");}
     QChar__Script_Nko => // 66
     {return String::from("Script_Nko");}
     QChar__Script_Sundanese => // 67
     {return String::from("Script_Sundanese");}
     QChar__Script_Lepcha => // 68
     {return String::from("Script_Lepcha");}
     QChar__Script_OlChiki => // 69
     {return String::from("Script_OlChiki");}
     QChar__Script_Vai => // 70
     {return String::from("Script_Vai");}
     QChar__Script_Saurashtra => // 71
     {return String::from("Script_Saurashtra");}
     QChar__Script_KayahLi => // 72
     {return String::from("Script_KayahLi");}
     QChar__Script_Rejang => // 73
     {return String::from("Script_Rejang");}
     QChar__Script_Lycian => // 74
     {return String::from("Script_Lycian");}
     QChar__Script_Carian => // 75
     {return String::from("Script_Carian");}
     QChar__Script_Lydian => // 76
     {return String::from("Script_Lydian");}
     QChar__Script_Cham => // 77
     {return String::from("Script_Cham");}
     QChar__Script_TaiTham => // 78
     {return String::from("Script_TaiTham");}
     QChar__Script_TaiViet => // 79
     {return String::from("Script_TaiViet");}
     QChar__Script_Avestan => // 80
     {return String::from("Script_Avestan");}
     QChar__Script_EgyptianHieroglyphs => // 81
     {return String::from("Script_EgyptianHieroglyphs");}
     QChar__Script_Samaritan => // 82
     {return String::from("Script_Samaritan");}
     QChar__Script_Lisu => // 83
     {return String::from("Script_Lisu");}
     QChar__Script_Bamum => // 84
     {return String::from("Script_Bamum");}
     QChar__Script_Javanese => // 85
     {return String::from("Script_Javanese");}
     QChar__Script_MeeteiMayek => // 86
     {return String::from("Script_MeeteiMayek");}
     QChar__Script_ImperialAramaic => // 87
     {return String::from("Script_ImperialAramaic");}
     QChar__Script_OldSouthArabian => // 88
     {return String::from("Script_OldSouthArabian");}
     QChar__Script_InscriptionalParthian => // 89
     {return String::from("Script_InscriptionalParthian");}
     QChar__Script_InscriptionalPahlavi => // 90
     {return String::from("Script_InscriptionalPahlavi");}
     QChar__Script_OldTurkic => // 91
     {return String::from("Script_OldTurkic");}
     QChar__Script_Kaithi => // 92
     {return String::from("Script_Kaithi");}
     QChar__Script_Batak => // 93
     {return String::from("Script_Batak");}
     QChar__Script_Brahmi => // 94
     {return String::from("Script_Brahmi");}
     QChar__Script_Mandaic => // 95
     {return String::from("Script_Mandaic");}
     QChar__Script_Chakma => // 96
     {return String::from("Script_Chakma");}
     QChar__Script_MeroiticCursive => // 97
     {return String::from("Script_MeroiticCursive");}
     QChar__Script_MeroiticHieroglyphs => // 98
     {return String::from("Script_MeroiticHieroglyphs");}
     QChar__Script_Miao => // 99
     {return String::from("Script_Miao");}
     QChar__Script_Sharada => // 100
     {return String::from("Script_Sharada");}
     QChar__Script_SoraSompeng => // 101
     {return String::from("Script_SoraSompeng");}
     QChar__Script_Takri => // 102
     {return String::from("Script_Takri");}
     QChar__Script_CaucasianAlbanian => // 103
     {return String::from("Script_CaucasianAlbanian");}
     QChar__Script_BassaVah => // 104
     {return String::from("Script_BassaVah");}
     QChar__Script_Duployan => // 105
     {return String::from("Script_Duployan");}
     QChar__Script_Elbasan => // 106
     {return String::from("Script_Elbasan");}
     QChar__Script_Grantha => // 107
     {return String::from("Script_Grantha");}
     QChar__Script_PahawhHmong => // 108
     {return String::from("Script_PahawhHmong");}
     QChar__Script_Khojki => // 109
     {return String::from("Script_Khojki");}
     QChar__Script_LinearA => // 110
     {return String::from("Script_LinearA");}
     QChar__Script_Mahajani => // 111
     {return String::from("Script_Mahajani");}
     QChar__Script_Manichaean => // 112
     {return String::from("Script_Manichaean");}
     QChar__Script_MendeKikakui => // 113
     {return String::from("Script_MendeKikakui");}
     QChar__Script_Modi => // 114
     {return String::from("Script_Modi");}
     QChar__Script_Mro => // 115
     {return String::from("Script_Mro");}
     QChar__Script_OldNorthArabian => // 116
     {return String::from("Script_OldNorthArabian");}
     QChar__Script_Nabataean => // 117
     {return String::from("Script_Nabataean");}
     QChar__Script_Palmyrene => // 118
     {return String::from("Script_Palmyrene");}
     QChar__Script_PauCinHau => // 119
     {return String::from("Script_PauCinHau");}
     QChar__Script_OldPermic => // 120
     {return String::from("Script_OldPermic");}
     QChar__Script_PsalterPahlavi => // 121
     {return String::from("Script_PsalterPahlavi");}
     QChar__Script_Siddham => // 122
     {return String::from("Script_Siddham");}
     QChar__Script_Khudawadi => // 123
     {return String::from("Script_Khudawadi");}
     QChar__Script_Tirhuta => // 124
     {return String::from("Script_Tirhuta");}
     QChar__Script_WarangCiti => // 125
     {return String::from("Script_WarangCiti");}
     QChar__Script_Ahom => // 126
     {return String::from("Script_Ahom");}
     QChar__Script_AnatolianHieroglyphs => // 127
     {return String::from("Script_AnatolianHieroglyphs");}
     QChar__Script_Hatran => // 128
     {return String::from("Script_Hatran");}
     QChar__Script_Multani => // 129
     {return String::from("Script_Multani");}
     QChar__Script_OldHungarian => // 130
     {return String::from("Script_OldHungarian");}
     QChar__Script_SignWriting => // 131
     {return String::from("Script_SignWriting");}
     QChar__ScriptCount => // 132
     {return String::from("ScriptCount");}
  _ => {return format!("{}", val);}
}
}
pub fn QChar_ScriptItemName_s(val: i32) ->String {
  //var nilthis *QChar
  //return nilthis.ScriptItemName(val);
  return QChar_ScriptItemName(val);
}


/*
This enum type defines the Unicode direction attributes. See the Unicode Standard for a description of the values.

In order to conform to C/C++ naming conventions "Dir" is prepended to the codes used in the Unicode Standard.



See also direction().

*/
pub type QChar__Direction = i32;
//  
pub const QChar__DirL :QChar__Direction = 0;
//  
pub const QChar__DirR :QChar__Direction = 1;
//  
pub const QChar__DirEN :QChar__Direction = 2;
//  
pub const QChar__DirES :QChar__Direction = 3;
//  
pub const QChar__DirET :QChar__Direction = 4;
//  
pub const QChar__DirAN :QChar__Direction = 5;
//  
pub const QChar__DirCS :QChar__Direction = 6;
//  
pub const QChar__DirB :QChar__Direction = 7;
//  
pub const QChar__DirS :QChar__Direction = 8;
//  
pub const QChar__DirWS :QChar__Direction = 9;
// 
pub const QChar__DirON :QChar__Direction = 10;
// 
pub const QChar__DirLRE :QChar__Direction = 11;
// 
pub const QChar__DirLRO :QChar__Direction = 12;
// 
pub const QChar__DirAL :QChar__Direction = 13;
// 
pub const QChar__DirRLE :QChar__Direction = 14;
// 
pub const QChar__DirRLO :QChar__Direction = 15;
// 
pub const QChar__DirPDF :QChar__Direction = 16;
// 
pub const QChar__DirNSM :QChar__Direction = 17;
// 
pub const QChar__DirBN :QChar__Direction = 18;
// 
pub const QChar__DirLRI :QChar__Direction = 19;
// 
pub const QChar__DirRLI :QChar__Direction = 20;
// 
pub const QChar__DirFSI :QChar__Direction = 21;
// 
pub const QChar__DirPDI :QChar__Direction = 22;
pub fn QChar_DirectionItemName(val: i32) ->String {
  match val {
     QChar__DirL => // 0
     {return String::from("DirL");}
     QChar__DirR => // 1
     {return String::from("DirR");}
     QChar__DirEN => // 2
     {return String::from("DirEN");}
     QChar__DirES => // 3
     {return String::from("DirES");}
     QChar__DirET => // 4
     {return String::from("DirET");}
     QChar__DirAN => // 5
     {return String::from("DirAN");}
     QChar__DirCS => // 6
     {return String::from("DirCS");}
     QChar__DirB => // 7
     {return String::from("DirB");}
     QChar__DirS => // 8
     {return String::from("DirS");}
     QChar__DirWS => // 9
     {return String::from("DirWS");}
     QChar__DirON => // 10
     {return String::from("DirON");}
     QChar__DirLRE => // 11
     {return String::from("DirLRE");}
     QChar__DirLRO => // 12
     {return String::from("DirLRO");}
     QChar__DirAL => // 13
     {return String::from("DirAL");}
     QChar__DirRLE => // 14
     {return String::from("DirRLE");}
     QChar__DirRLO => // 15
     {return String::from("DirRLO");}
     QChar__DirPDF => // 16
     {return String::from("DirPDF");}
     QChar__DirNSM => // 17
     {return String::from("DirNSM");}
     QChar__DirBN => // 18
     {return String::from("DirBN");}
     QChar__DirLRI => // 19
     {return String::from("DirLRI");}
     QChar__DirRLI => // 20
     {return String::from("DirRLI");}
     QChar__DirFSI => // 21
     {return String::from("DirFSI");}
     QChar__DirPDI => // 22
     {return String::from("DirPDI");}
  _ => {return format!("{}", val);}
}
}
pub fn QChar_DirectionItemName_s(val: i32) ->String {
  //var nilthis *QChar
  //return nilthis.DirectionItemName(val);
  return QChar_DirectionItemName(val);
}


/*
This enum type defines the Unicode decomposition attributes. See the Unicode Standard for a description of the values.

ConstantValue
QChar::NoDecomposition0
QChar::Canonical1
QChar::Circle8
QChar::Final6
QChar::Font2
QChar::Initial4
QChar::Isolated7
QChar::Medial5
QChar::NoBreak3
QChar::Super9


See also decomposition().

*/
pub type QChar__Decomposition = i32;
// 
pub const QChar__NoDecomposition :QChar__Decomposition = 0;
// 
pub const QChar__Canonical :QChar__Decomposition = 1;
// 
pub const QChar__Font :QChar__Decomposition = 2;
// 
pub const QChar__NoBreak :QChar__Decomposition = 3;
// 
pub const QChar__Initial :QChar__Decomposition = 4;
// 
pub const QChar__Medial :QChar__Decomposition = 5;
// 
pub const QChar__Final :QChar__Decomposition = 6;
// 
pub const QChar__Isolated :QChar__Decomposition = 7;
// 
pub const QChar__Circle :QChar__Decomposition = 8;
// 
pub const QChar__Super :QChar__Decomposition = 9;
// 0
pub const QChar__Sub :QChar__Decomposition = 10;
// 1
pub const QChar__Vertical :QChar__Decomposition = 11;
// 2
pub const QChar__Wide :QChar__Decomposition = 12;
// 3
pub const QChar__Narrow :QChar__Decomposition = 13;
// 4
pub const QChar__Small :QChar__Decomposition = 14;
// 5
pub const QChar__Square :QChar__Decomposition = 15;
// 6
pub const QChar__Compat :QChar__Decomposition = 16;
// 7
pub const QChar__Fraction :QChar__Decomposition = 17;
pub fn QChar_DecompositionItemName(val: i32) ->String {
  match val {
     QChar__NoDecomposition => // 0
     {return String::from("NoDecomposition");}
     QChar__Canonical => // 1
     {return String::from("Canonical");}
     QChar__Font => // 2
     {return String::from("Font");}
     QChar__NoBreak => // 3
     {return String::from("NoBreak");}
     QChar__Initial => // 4
     {return String::from("Initial");}
     QChar__Medial => // 5
     {return String::from("Medial");}
     QChar__Final => // 6
     {return String::from("Final");}
     QChar__Isolated => // 7
     {return String::from("Isolated");}
     QChar__Circle => // 8
     {return String::from("Circle");}
     QChar__Super => // 9
     {return String::from("Super");}
     QChar__Sub => // 10
     {return String::from("Sub");}
     QChar__Vertical => // 11
     {return String::from("Vertical");}
     QChar__Wide => // 12
     {return String::from("Wide");}
     QChar__Narrow => // 13
     {return String::from("Narrow");}
     QChar__Small => // 14
     {return String::from("Small");}
     QChar__Square => // 15
     {return String::from("Square");}
     QChar__Compat => // 16
     {return String::from("Compat");}
     QChar__Fraction => // 17
     {return String::from("Fraction");}
  _ => {return format!("{}", val);}
}
}
pub fn QChar_DecompositionItemName_s(val: i32) ->String {
  //var nilthis *QChar
  //return nilthis.DecompositionItemName(val);
  return QChar_DecompositionItemName(val);
}


/*
since 5.3

This enum type defines the Unicode joining type attributes. See the Unicode Standard for a description of the values.

In order to conform to C/C++ naming conventions "Joining_" is prepended to the codes used in the Unicode Standard.

ConstantValue
QChar::Joining_None0
QChar::Joining_Causing1
QChar::Joining_Dual2
QChar::Joining_Right3
QChar::Joining_Left4
QChar::Joining_Transparent5


See also joiningType().

*/
pub type QChar__JoiningType = i32;
// 
pub const QChar__Joining_None :QChar__JoiningType = 0;
// 
pub const QChar__Joining_Causing :QChar__JoiningType = 1;
// 
pub const QChar__Joining_Dual :QChar__JoiningType = 2;
// 
pub const QChar__Joining_Right :QChar__JoiningType = 3;
// 
pub const QChar__Joining_Left :QChar__JoiningType = 4;
// 
pub const QChar__Joining_Transparent :QChar__JoiningType = 5;
pub fn QChar_JoiningTypeItemName(val: i32) ->String {
  match val {
     QChar__Joining_None => // 0
     {return String::from("Joining_None");}
     QChar__Joining_Causing => // 1
     {return String::from("Joining_Causing");}
     QChar__Joining_Dual => // 2
     {return String::from("Joining_Dual");}
     QChar__Joining_Right => // 3
     {return String::from("Joining_Right");}
     QChar__Joining_Left => // 4
     {return String::from("Joining_Left");}
     QChar__Joining_Transparent => // 5
     {return String::from("Joining_Transparent");}
  _ => {return format!("{}", val);}
}
}
pub fn QChar_JoiningTypeItemName_s(val: i32) ->String {
  //var nilthis *QChar
  //return nilthis.JoiningTypeItemName(val);
  return QChar_JoiningTypeItemName(val);
}


/*


*/
pub type QChar__Joining = i32;
// 
pub const QChar__OtherJoining :QChar__Joining = 0;
// 
pub const QChar__Dual :QChar__Joining = 1;
// 
pub const QChar__Right :QChar__Joining = 2;
// 
pub const QChar__Center :QChar__Joining = 3;
pub fn QChar_JoiningItemName(val: i32) ->String {
  match val {
     QChar__OtherJoining => // 0
     {return String::from("OtherJoining");}
     QChar__Dual => // 1
     {return String::from("Dual");}
     QChar__Right => // 2
     {return String::from("Right");}
     QChar__Center => // 3
     {return String::from("Center");}
  _ => {return format!("{}", val);}
}
}
pub fn QChar_JoiningItemName_s(val: i32) ->String {
  //var nilthis *QChar
  //return nilthis.JoiningItemName(val);
  return QChar_JoiningItemName(val);
}


/*


*/
pub type QChar__CombiningClass = i32;
// 
pub const QChar__Combining_BelowLeftAttached :QChar__CombiningClass = 200;
// 
pub const QChar__Combining_BelowAttached :QChar__CombiningClass = 202;
// 
pub const QChar__Combining_BelowRightAttached :QChar__CombiningClass = 204;
// 
pub const QChar__Combining_LeftAttached :QChar__CombiningClass = 208;
// 
pub const QChar__Combining_RightAttached :QChar__CombiningClass = 210;
// 
pub const QChar__Combining_AboveLeftAttached :QChar__CombiningClass = 212;
// 
pub const QChar__Combining_AboveAttached :QChar__CombiningClass = 214;
// 
pub const QChar__Combining_AboveRightAttached :QChar__CombiningClass = 216;
// 
pub const QChar__Combining_BelowLeft :QChar__CombiningClass = 218;
// 
pub const QChar__Combining_Below :QChar__CombiningClass = 220;
// 
pub const QChar__Combining_BelowRight :QChar__CombiningClass = 222;
// 
pub const QChar__Combining_Left :QChar__CombiningClass = 224;
// 
pub const QChar__Combining_Right :QChar__CombiningClass = 226;
// 
pub const QChar__Combining_AboveLeft :QChar__CombiningClass = 228;
// 
pub const QChar__Combining_Above :QChar__CombiningClass = 230;
// 
pub const QChar__Combining_AboveRight :QChar__CombiningClass = 232;
// 
pub const QChar__Combining_DoubleBelow :QChar__CombiningClass = 233;
// 
pub const QChar__Combining_DoubleAbove :QChar__CombiningClass = 234;
// 
pub const QChar__Combining_IotaSubscript :QChar__CombiningClass = 240;
pub fn QChar_CombiningClassItemName(val: i32) ->String {
  match val {
     QChar__Combining_BelowLeftAttached => // 200
     {return String::from("Combining_BelowLeftAttached");}
     QChar__Combining_BelowAttached => // 202
     {return String::from("Combining_BelowAttached");}
     QChar__Combining_BelowRightAttached => // 204
     {return String::from("Combining_BelowRightAttached");}
     QChar__Combining_LeftAttached => // 208
     {return String::from("Combining_LeftAttached");}
     QChar__Combining_RightAttached => // 210
     {return String::from("Combining_RightAttached");}
     QChar__Combining_AboveLeftAttached => // 212
     {return String::from("Combining_AboveLeftAttached");}
     QChar__Combining_AboveAttached => // 214
     {return String::from("Combining_AboveAttached");}
     QChar__Combining_AboveRightAttached => // 216
     {return String::from("Combining_AboveRightAttached");}
     QChar__Combining_BelowLeft => // 218
     {return String::from("Combining_BelowLeft");}
     QChar__Combining_Below => // 220
     {return String::from("Combining_Below");}
     QChar__Combining_BelowRight => // 222
     {return String::from("Combining_BelowRight");}
     QChar__Combining_Left => // 224
     {return String::from("Combining_Left");}
     QChar__Combining_Right => // 226
     {return String::from("Combining_Right");}
     QChar__Combining_AboveLeft => // 228
     {return String::from("Combining_AboveLeft");}
     QChar__Combining_Above => // 230
     {return String::from("Combining_Above");}
     QChar__Combining_AboveRight => // 232
     {return String::from("Combining_AboveRight");}
     QChar__Combining_DoubleBelow => // 233
     {return String::from("Combining_DoubleBelow");}
     QChar__Combining_DoubleAbove => // 234
     {return String::from("Combining_DoubleAbove");}
     QChar__Combining_IotaSubscript => // 240
     {return String::from("Combining_IotaSubscript");}
  _ => {return format!("{}", val);}
}
}
pub fn QChar_CombiningClassItemName_s(val: i32) ->String {
  //var nilthis *QChar
  //return nilthis.CombiningClassItemName(val);
  return QChar_CombiningClassItemName(val);
}


/*
Specifies which version of the Unicode standard introduced a certain character.



See also unicodeVersion() and currentUnicodeVersion().

*/
pub type QChar__UnicodeVersion = i32;
// 
pub const QChar__Unicode_Unassigned :QChar__UnicodeVersion = 0;
// 
pub const QChar__Unicode_1_1 :QChar__UnicodeVersion = 1;
// 
pub const QChar__Unicode_2_0 :QChar__UnicodeVersion = 2;
// 
pub const QChar__Unicode_2_1_2 :QChar__UnicodeVersion = 3;
// 
pub const QChar__Unicode_3_0 :QChar__UnicodeVersion = 4;
// 
pub const QChar__Unicode_3_1 :QChar__UnicodeVersion = 5;
// 
pub const QChar__Unicode_3_2 :QChar__UnicodeVersion = 6;
// 
pub const QChar__Unicode_4_0 :QChar__UnicodeVersion = 7;
// 
pub const QChar__Unicode_4_1 :QChar__UnicodeVersion = 8;
// 
pub const QChar__Unicode_5_0 :QChar__UnicodeVersion = 9;
// 
pub const QChar__Unicode_5_1 :QChar__UnicodeVersion = 10;
// 
pub const QChar__Unicode_5_2 :QChar__UnicodeVersion = 11;
// 
pub const QChar__Unicode_6_0 :QChar__UnicodeVersion = 12;
// 
pub const QChar__Unicode_6_1 :QChar__UnicodeVersion = 13;
// 
pub const QChar__Unicode_6_2 :QChar__UnicodeVersion = 14;
// 
pub const QChar__Unicode_6_3 :QChar__UnicodeVersion = 15;
// 
pub const QChar__Unicode_7_0 :QChar__UnicodeVersion = 16;
// 
pub const QChar__Unicode_8_0 :QChar__UnicodeVersion = 17;
pub fn QChar_UnicodeVersionItemName(val: i32) ->String {
  match val {
     QChar__Unicode_Unassigned => // 0
     {return String::from("Unicode_Unassigned");}
     QChar__Unicode_1_1 => // 1
     {return String::from("Unicode_1_1");}
     QChar__Unicode_2_0 => // 2
     {return String::from("Unicode_2_0");}
     QChar__Unicode_2_1_2 => // 3
     {return String::from("Unicode_2_1_2");}
     QChar__Unicode_3_0 => // 4
     {return String::from("Unicode_3_0");}
     QChar__Unicode_3_1 => // 5
     {return String::from("Unicode_3_1");}
     QChar__Unicode_3_2 => // 6
     {return String::from("Unicode_3_2");}
     QChar__Unicode_4_0 => // 7
     {return String::from("Unicode_4_0");}
     QChar__Unicode_4_1 => // 8
     {return String::from("Unicode_4_1");}
     QChar__Unicode_5_0 => // 9
     {return String::from("Unicode_5_0");}
     QChar__Unicode_5_1 => // 10
     {return String::from("Unicode_5_1");}
     QChar__Unicode_5_2 => // 11
     {return String::from("Unicode_5_2");}
     QChar__Unicode_6_0 => // 12
     {return String::from("Unicode_6_0");}
     QChar__Unicode_6_1 => // 13
     {return String::from("Unicode_6_1");}
     QChar__Unicode_6_2 => // 14
     {return String::from("Unicode_6_2");}
     QChar__Unicode_6_3 => // 15
     {return String::from("Unicode_6_3");}
     QChar__Unicode_7_0 => // 16
     {return String::from("Unicode_7_0");}
     QChar__Unicode_8_0 => // 17
     {return String::from("Unicode_8_0");}
  _ => {return format!("{}", val);}
}
}
pub fn QChar_UnicodeVersionItemName_s(val: i32) ->String {
  //var nilthis *QChar
  //return nilthis.UnicodeVersionItemName(val);
  return QChar_UnicodeVersionItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
