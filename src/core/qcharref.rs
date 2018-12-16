

// mod ::core::QCharRef
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
// extern C begin: 262
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
#[derive(Default)] // class sizeof(QCharRef)=16
pub struct QCharRef {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QCharRef_ITF interface {
//    QCharRef_PTR() *QCharRef
//}
//func (ptr *QCharRef) QCharRef_PTR() *QCharRef { return ptr }

impl /*struct*/ QCharRef {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QCharRef {
    return QCharRef{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QCharRef {
//  type Target = QCharRefBASE;
//
//  fn deref(&self) -> &QCharRefBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QCharRefBASE> for QCharRef {
//  fn as_ref(& self) -> & QCharRefBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qstring.h:1036
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QCharRef & operator=(QChar)

/*

*/
impl /*struct*/ QCharRef {
  pub fn operator_equal_0<RetType, T: QCharRef_operator_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_0(self);
    // return 1;
  }
}
pub trait QCharRef_operator_equal_0<RetType> {
  fn operator_equal_0(self , rsthis: & QCharRef) -> RetType;
}
impl<'a> /*trait*/ QCharRef_operator_equal_0<usize> for (usize) {
  fn operator_equal_0(self , rsthis: & QCharRef) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QCharRefaSE5QChar", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1042
// index:1
// Public inline Visibility=Default Availability=Available
// [16] QCharRef & operator=(char)

/*

*/
impl /*struct*/ QCharRef {
  pub fn operator_equal_1<RetType, T: QCharRef_operator_equal_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_1(self);
    // return 1;
  }
}
pub trait QCharRef_operator_equal_1<RetType> {
  fn operator_equal_1(self , rsthis: & QCharRef) -> RetType;
}
impl<'a> /*trait*/ QCharRef_operator_equal_1<usize> for (i8) {
  fn operator_equal_1(self , rsthis: & QCharRef) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i8 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QCharRefaSEc", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1044
// index:2
// Public inline Visibility=Default Availability=Available
// [16] QCharRef & operator=(uchar)

/*

*/
impl /*struct*/ QCharRef {
  pub fn operator_equal_2<RetType, T: QCharRef_operator_equal_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_2(self);
    // return 1;
  }
}
pub trait QCharRef_operator_equal_2<RetType> {
  fn operator_equal_2(self , rsthis: & QCharRef) -> RetType;
}
impl<'a> /*trait*/ QCharRef_operator_equal_2<usize> for (u8) {
  fn operator_equal_2(self , rsthis: & QCharRef) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const u8 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QCharRefaSEh", 1,qtrt::FFITY_UINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1047
// index:3
// Public inline Visibility=Default Availability=Available
// [16] QCharRef & operator=(const QCharRef &)

/*

*/
impl /*struct*/ QCharRef {
  pub fn operator_equal_3<RetType, T: QCharRef_operator_equal_3<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_3(self);
    // return 1;
  }
}
pub trait QCharRef_operator_equal_3<RetType> {
  fn operator_equal_3(self , rsthis: & QCharRef) -> RetType;
}
impl<'a> /*trait*/ QCharRef_operator_equal_3<usize> for (usize) {
  fn operator_equal_3(self , rsthis: & QCharRef) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QCharRefaSERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1048
// index:4
// Public inline Visibility=Default Availability=Available
// [16] QCharRef & operator=(ushort)

/*

*/
impl /*struct*/ QCharRef {
  pub fn operator_equal_4<RetType, T: QCharRef_operator_equal_4<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_4(self);
    // return 1;
  }
}
pub trait QCharRef_operator_equal_4<RetType> {
  fn operator_equal_4(self , rsthis: & QCharRef) -> RetType;
}
impl<'a> /*trait*/ QCharRef_operator_equal_4<usize> for (u16) {
  fn operator_equal_4(self , rsthis: & QCharRef) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const u16 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QCharRefaSEt", 1,qtrt::FFITY_UINT16,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1049
// index:5
// Public inline Visibility=Default Availability=Available
// [16] QCharRef & operator=(short)

/*

*/
impl /*struct*/ QCharRef {
  pub fn operator_equal_5<RetType, T: QCharRef_operator_equal_5<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_5(self);
    // return 1;
  }
}
pub trait QCharRef_operator_equal_5<RetType> {
  fn operator_equal_5(self , rsthis: & QCharRef) -> RetType;
}
impl<'a> /*trait*/ QCharRef_operator_equal_5<usize> for (i16) {
  fn operator_equal_5(self , rsthis: & QCharRef) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i16 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QCharRefaSEs", 1,qtrt::FFITY_SINT16,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1050
// index:6
// Public inline Visibility=Default Availability=Available
// [16] QCharRef & operator=(uint)

/*

*/
impl /*struct*/ QCharRef {
  pub fn operator_equal_6<RetType, T: QCharRef_operator_equal_6<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_6(self);
    // return 1;
  }
}
pub trait QCharRef_operator_equal_6<RetType> {
  fn operator_equal_6(self , rsthis: & QCharRef) -> RetType;
}
impl<'a> /*trait*/ QCharRef_operator_equal_6<usize> for (u32) {
  fn operator_equal_6(self , rsthis: & QCharRef) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const u32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QCharRefaSEj", 1,qtrt::FFITY_UINT32,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1051
// index:7
// Public inline Visibility=Default Availability=Available
// [16] QCharRef & operator=(int)

/*

*/
impl /*struct*/ QCharRef {
  pub fn operator_equal_7<RetType, T: QCharRef_operator_equal_7<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_7(self);
    // return 1;
  }
}
pub trait QCharRef_operator_equal_7<RetType> {
  fn operator_equal_7(self , rsthis: & QCharRef) -> RetType;
}
impl<'a> /*trait*/ QCharRef_operator_equal_7<usize> for (i32) {
  fn operator_equal_7(self , rsthis: & QCharRef) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QCharRefaSEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1054
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
impl /*struct*/ QCharRef {
  pub fn isNull_0<RetType, T: QCharRef_isNull_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isNull_0(self);
    // return 1;
  }
}
pub trait QCharRef_isNull_0<RetType> {
  fn isNull_0(self , rsthis: & QCharRef) -> RetType;
}
impl<'a> /*trait*/ QCharRef_isNull_0<bool> for () {
  fn isNull_0(self , rsthis: & QCharRef) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QCharRef6isNullEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1055
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isPrint() const

/*

*/
impl /*struct*/ QCharRef {
  pub fn isPrint_0<RetType, T: QCharRef_isPrint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isPrint_0(self);
    // return 1;
  }
}
pub trait QCharRef_isPrint_0<RetType> {
  fn isPrint_0(self , rsthis: & QCharRef) -> RetType;
}
impl<'a> /*trait*/ QCharRef_isPrint_0<bool> for () {
  fn isPrint_0(self , rsthis: & QCharRef) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QCharRef7isPrintEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1056
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isPunct() const

/*

*/
impl /*struct*/ QCharRef {
  pub fn isPunct_0<RetType, T: QCharRef_isPunct_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isPunct_0(self);
    // return 1;
  }
}
pub trait QCharRef_isPunct_0<RetType> {
  fn isPunct_0(self , rsthis: & QCharRef) -> RetType;
}
impl<'a> /*trait*/ QCharRef_isPunct_0<bool> for () {
  fn isPunct_0(self , rsthis: & QCharRef) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QCharRef7isPunctEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1057
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isSpace() const

/*

*/
impl /*struct*/ QCharRef {
  pub fn isSpace_0<RetType, T: QCharRef_isSpace_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isSpace_0(self);
    // return 1;
  }
}
pub trait QCharRef_isSpace_0<RetType> {
  fn isSpace_0(self , rsthis: & QCharRef) -> RetType;
}
impl<'a> /*trait*/ QCharRef_isSpace_0<bool> for () {
  fn isSpace_0(self , rsthis: & QCharRef) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QCharRef7isSpaceEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1058
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isMark() const

/*

*/
impl /*struct*/ QCharRef {
  pub fn isMark_0<RetType, T: QCharRef_isMark_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isMark_0(self);
    // return 1;
  }
}
pub trait QCharRef_isMark_0<RetType> {
  fn isMark_0(self , rsthis: & QCharRef) -> RetType;
}
impl<'a> /*trait*/ QCharRef_isMark_0<bool> for () {
  fn isMark_0(self , rsthis: & QCharRef) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QCharRef6isMarkEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1059
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isLetter() const

/*

*/
impl /*struct*/ QCharRef {
  pub fn isLetter_0<RetType, T: QCharRef_isLetter_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isLetter_0(self);
    // return 1;
  }
}
pub trait QCharRef_isLetter_0<RetType> {
  fn isLetter_0(self , rsthis: & QCharRef) -> RetType;
}
impl<'a> /*trait*/ QCharRef_isLetter_0<bool> for () {
  fn isLetter_0(self , rsthis: & QCharRef) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QCharRef8isLetterEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1060
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isNumber() const

/*

*/
impl /*struct*/ QCharRef {
  pub fn isNumber_0<RetType, T: QCharRef_isNumber_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isNumber_0(self);
    // return 1;
  }
}
pub trait QCharRef_isNumber_0<RetType> {
  fn isNumber_0(self , rsthis: & QCharRef) -> RetType;
}
impl<'a> /*trait*/ QCharRef_isNumber_0<bool> for () {
  fn isNumber_0(self , rsthis: & QCharRef) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QCharRef8isNumberEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1061
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isLetterOrNumber()

/*

*/
impl /*struct*/ QCharRef {
  pub fn isLetterOrNumber_0<RetType, T: QCharRef_isLetterOrNumber_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isLetterOrNumber_0(self);
    // return 1;
  }
}
pub trait QCharRef_isLetterOrNumber_0<RetType> {
  fn isLetterOrNumber_0(self , rsthis: & QCharRef) -> RetType;
}
impl<'a> /*trait*/ QCharRef_isLetterOrNumber_0<bool> for () {
  fn isLetterOrNumber_0(self , rsthis: & QCharRef) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QCharRef16isLetterOrNumberEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1062
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isDigit() const

/*

*/
impl /*struct*/ QCharRef {
  pub fn isDigit_0<RetType, T: QCharRef_isDigit_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isDigit_0(self);
    // return 1;
  }
}
pub trait QCharRef_isDigit_0<RetType> {
  fn isDigit_0(self , rsthis: & QCharRef) -> RetType;
}
impl<'a> /*trait*/ QCharRef_isDigit_0<bool> for () {
  fn isDigit_0(self , rsthis: & QCharRef) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QCharRef7isDigitEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1063
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isLower() const

/*

*/
impl /*struct*/ QCharRef {
  pub fn isLower_0<RetType, T: QCharRef_isLower_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isLower_0(self);
    // return 1;
  }
}
pub trait QCharRef_isLower_0<RetType> {
  fn isLower_0(self , rsthis: & QCharRef) -> RetType;
}
impl<'a> /*trait*/ QCharRef_isLower_0<bool> for () {
  fn isLower_0(self , rsthis: & QCharRef) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QCharRef7isLowerEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1064
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isUpper() const

/*

*/
impl /*struct*/ QCharRef {
  pub fn isUpper_0<RetType, T: QCharRef_isUpper_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isUpper_0(self);
    // return 1;
  }
}
pub trait QCharRef_isUpper_0<RetType> {
  fn isUpper_0(self , rsthis: & QCharRef) -> RetType;
}
impl<'a> /*trait*/ QCharRef_isUpper_0<bool> for () {
  fn isUpper_0(self , rsthis: & QCharRef) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QCharRef7isUpperEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1065
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isTitleCase() const

/*

*/
impl /*struct*/ QCharRef {
  pub fn isTitleCase_0<RetType, T: QCharRef_isTitleCase_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isTitleCase_0(self);
    // return 1;
  }
}
pub trait QCharRef_isTitleCase_0<RetType> {
  fn isTitleCase_0(self , rsthis: & QCharRef) -> RetType;
}
impl<'a> /*trait*/ QCharRef_isTitleCase_0<bool> for () {
  fn isTitleCase_0(self , rsthis: & QCharRef) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QCharRef11isTitleCaseEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1067
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int digitValue() const

/*

*/
impl /*struct*/ QCharRef {
  pub fn digitValue_0<RetType, T: QCharRef_digitValue_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.digitValue_0(self);
    // return 1;
  }
}
pub trait QCharRef_digitValue_0<RetType> {
  fn digitValue_0(self , rsthis: & QCharRef) -> RetType;
}
impl<'a> /*trait*/ QCharRef_digitValue_0<i32> for () {
  fn digitValue_0(self , rsthis: & QCharRef) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QCharRef10digitValueEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1068
// index:0
// Public inline Visibility=Default Availability=Available
// [2] QChar toLower() const

/*
Returns a lowercase copy of the string.


  QString str = "The Qt PROJECT";
  str = str.toLower();        // str == "the qt project"



The case conversion will always happen in the 'C' locale. For locale dependent case folding use QLocale::toLower()

See also toUpper() and QLocale::toLower().
*/
impl /*struct*/ QCharRef {
  pub fn toLower_0<RetType, T: QCharRef_toLower_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toLower_0(self);
    // return 1;
  }
}
pub trait QCharRef_toLower_0<RetType> {
  fn toLower_0(self , rsthis: & QCharRef) -> RetType;
}
impl<'a> /*trait*/ QCharRef_toLower_0<usize> for () {
  fn toLower_0(self , rsthis: & QCharRef) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QCharRef7toLowerEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1069
// index:0
// Public inline Visibility=Default Availability=Available
// [2] QChar toUpper() const

/*
Returns an uppercase copy of the string.


  QString str = "TeXt";
  str = str.toUpper();        // str == "TEXT"



The case conversion will always happen in the 'C' locale. For locale dependent case folding use QLocale::toUpper()

See also toLower() and QLocale::toLower().
*/
impl /*struct*/ QCharRef {
  pub fn toUpper_0<RetType, T: QCharRef_toUpper_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toUpper_0(self);
    // return 1;
  }
}
pub trait QCharRef_toUpper_0<RetType> {
  fn toUpper_0(self , rsthis: & QCharRef) -> RetType;
}
impl<'a> /*trait*/ QCharRef_toUpper_0<usize> for () {
  fn toUpper_0(self , rsthis: & QCharRef) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QCharRef7toUpperEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1070
// index:0
// Public inline Visibility=Default Availability=Available
// [2] QChar toTitleCase() const

/*

*/
impl /*struct*/ QCharRef {
  pub fn toTitleCase_0<RetType, T: QCharRef_toTitleCase_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toTitleCase_0(self);
    // return 1;
  }
}
pub trait QCharRef_toTitleCase_0<RetType> {
  fn toTitleCase_0(self , rsthis: & QCharRef) -> RetType;
}
impl<'a> /*trait*/ QCharRef_toTitleCase_0<usize> for () {
  fn toTitleCase_0(self , rsthis: & QCharRef) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QCharRef11toTitleCaseEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1072
// index:0
// Public inline Visibility=Default Availability=Available
// [4] QChar::Category category() const

/*

*/
impl /*struct*/ QCharRef {
  pub fn category_0<RetType, T: QCharRef_category_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.category_0(self);
    // return 1;
  }
}
pub trait QCharRef_category_0<RetType> {
  fn category_0(self , rsthis: & QCharRef) -> RetType;
}
impl<'a> /*trait*/ QCharRef_category_0<i32> for () {
  fn category_0(self , rsthis: & QCharRef) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QCharRef8categoryEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1073
// index:0
// Public inline Visibility=Default Availability=Available
// [4] QChar::Direction direction() const

/*

*/
impl /*struct*/ QCharRef {
  pub fn direction_0<RetType, T: QCharRef_direction_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.direction_0(self);
    // return 1;
  }
}
pub trait QCharRef_direction_0<RetType> {
  fn direction_0(self , rsthis: & QCharRef) -> RetType;
}
impl<'a> /*trait*/ QCharRef_direction_0<i32> for () {
  fn direction_0(self , rsthis: & QCharRef) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QCharRef9directionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1074
// index:0
// Public inline Visibility=Default Availability=Available
// [4] QChar::JoiningType joiningType() const

/*

*/
impl /*struct*/ QCharRef {
  pub fn joiningType_0<RetType, T: QCharRef_joiningType_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.joiningType_0(self);
    // return 1;
  }
}
pub trait QCharRef_joiningType_0<RetType> {
  fn joiningType_0(self , rsthis: & QCharRef) -> RetType;
}
impl<'a> /*trait*/ QCharRef_joiningType_0<i32> for () {
  fn joiningType_0(self , rsthis: & QCharRef) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QCharRef11joiningTypeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1076
// index:0
// Public inline Visibility=Default Availability=Available
// [4] QChar::Joining joining() const

/*

*/
impl /*struct*/ QCharRef {
  pub fn joining_0<RetType, T: QCharRef_joining_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.joining_0(self);
    // return 1;
  }
}
pub trait QCharRef_joining_0<RetType> {
  fn joining_0(self , rsthis: & QCharRef) -> RetType;
}
impl<'a> /*trait*/ QCharRef_joining_0<i32> for () {
  fn joining_0(self , rsthis: & QCharRef) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QCharRef7joiningEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1089
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool hasMirrored() const

/*

*/
impl /*struct*/ QCharRef {
  pub fn hasMirrored_0<RetType, T: QCharRef_hasMirrored_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hasMirrored_0(self);
    // return 1;
  }
}
pub trait QCharRef_hasMirrored_0<RetType> {
  fn hasMirrored_0(self , rsthis: & QCharRef) -> RetType;
}
impl<'a> /*trait*/ QCharRef_hasMirrored_0<bool> for () {
  fn hasMirrored_0(self , rsthis: & QCharRef) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QCharRef11hasMirroredEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1090
// index:0
// Public inline Visibility=Default Availability=Available
// [2] QChar mirroredChar() const

/*

*/
impl /*struct*/ QCharRef {
  pub fn mirroredChar_0<RetType, T: QCharRef_mirroredChar_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mirroredChar_0(self);
    // return 1;
  }
}
pub trait QCharRef_mirroredChar_0<RetType> {
  fn mirroredChar_0(self , rsthis: & QCharRef) -> RetType;
}
impl<'a> /*trait*/ QCharRef_mirroredChar_0<usize> for () {
  fn mirroredChar_0(self , rsthis: & QCharRef) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QCharRef12mirroredCharEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1091
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QString decomposition() const

/*

*/
impl /*struct*/ QCharRef {
  pub fn decomposition_0<RetType, T: QCharRef_decomposition_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.decomposition_0(self);
    // return 1;
  }
}
pub trait QCharRef_decomposition_0<RetType> {
  fn decomposition_0(self , rsthis: & QCharRef) -> RetType;
}
impl<'a> /*trait*/ QCharRef_decomposition_0<usize> for () {
  fn decomposition_0(self , rsthis: & QCharRef) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QCharRef13decompositionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1092
// index:0
// Public inline Visibility=Default Availability=Available
// [4] QChar::Decomposition decompositionTag() const

/*

*/
impl /*struct*/ QCharRef {
  pub fn decompositionTag_0<RetType, T: QCharRef_decompositionTag_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.decompositionTag_0(self);
    // return 1;
  }
}
pub trait QCharRef_decompositionTag_0<RetType> {
  fn decompositionTag_0(self , rsthis: & QCharRef) -> RetType;
}
impl<'a> /*trait*/ QCharRef_decompositionTag_0<i32> for () {
  fn decompositionTag_0(self , rsthis: & QCharRef) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QCharRef16decompositionTagEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1093
// index:0
// Public inline Visibility=Default Availability=Available
// [1] uchar combiningClass() const

/*

*/
impl /*struct*/ QCharRef {
  pub fn combiningClass_0<RetType, T: QCharRef_combiningClass_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.combiningClass_0(self);
    // return 1;
  }
}
pub trait QCharRef_combiningClass_0<RetType> {
  fn combiningClass_0(self , rsthis: & QCharRef) -> RetType;
}
impl<'a> /*trait*/ QCharRef_combiningClass_0<u8> for () {
  fn combiningClass_0(self , rsthis: & QCharRef) -> u8 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QCharRef14combiningClassEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u8 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1095
// index:0
// Public inline Visibility=Default Availability=Available
// [4] QChar::Script script() const

/*

*/
impl /*struct*/ QCharRef {
  pub fn script_0<RetType, T: QCharRef_script_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.script_0(self);
    // return 1;
  }
}
pub trait QCharRef_script_0<RetType> {
  fn script_0(self , rsthis: & QCharRef) -> RetType;
}
impl<'a> /*trait*/ QCharRef_script_0<i32> for () {
  fn script_0(self , rsthis: & QCharRef) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QCharRef6scriptEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1097
// index:0
// Public inline Visibility=Default Availability=Available
// [4] QChar::UnicodeVersion unicodeVersion() const

/*

*/
impl /*struct*/ QCharRef {
  pub fn unicodeVersion_0<RetType, T: QCharRef_unicodeVersion_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.unicodeVersion_0(self);
    // return 1;
  }
}
pub trait QCharRef_unicodeVersion_0<RetType> {
  fn unicodeVersion_0(self , rsthis: & QCharRef) -> RetType;
}
impl<'a> /*trait*/ QCharRef_unicodeVersion_0<i32> for () {
  fn unicodeVersion_0(self , rsthis: & QCharRef) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QCharRef14unicodeVersionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1099
// index:0
// Public inline Visibility=Default Availability=Available
// [1] uchar cell() const

/*

*/
impl /*struct*/ QCharRef {
  pub fn cell_0<RetType, T: QCharRef_cell_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cell_0(self);
    // return 1;
  }
}
pub trait QCharRef_cell_0<RetType> {
  fn cell_0(self , rsthis: & QCharRef) -> RetType;
}
impl<'a> /*trait*/ QCharRef_cell_0<u8> for () {
  fn cell_0(self , rsthis: & QCharRef) -> u8 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QCharRef4cellEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u8 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1100
// index:0
// Public inline Visibility=Default Availability=Available
// [1] uchar row() const

/*

*/
impl /*struct*/ QCharRef {
  pub fn row_0<RetType, T: QCharRef_row_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.row_0(self);
    // return 1;
  }
}
pub trait QCharRef_row_0<RetType> {
  fn row_0(self , rsthis: & QCharRef) -> RetType;
}
impl<'a> /*trait*/ QCharRef_row_0<u8> for () {
  fn row_0(self , rsthis: & QCharRef) -> u8 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QCharRef3rowEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u8 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1101
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setCell(uchar)

/*

*/
impl /*struct*/ QCharRef {
  pub fn setCell_0<RetType, T: QCharRef_setCell_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCell_0(self);
    // return 1;
  }
}
pub trait QCharRef_setCell_0<RetType> {
  fn setCell_0(self , rsthis: & QCharRef) -> RetType;
}
impl<'a> /*trait*/ QCharRef_setCell_0<(/*void*/)> for (u8) {
  fn setCell_0(self , rsthis: & QCharRef) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const u8 as usize;
     qtrt::InvokeQtFunc6("_ZN8QCharRef7setCellEh", 1,qtrt::FFITY_UINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qstring.h:1102
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setRow(uchar)

/*

*/
impl /*struct*/ QCharRef {
  pub fn setRow_0<RetType, T: QCharRef_setRow_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setRow_0(self);
    // return 1;
  }
}
pub trait QCharRef_setRow_0<RetType> {
  fn setRow_0(self , rsthis: & QCharRef) -> RetType;
}
impl<'a> /*trait*/ QCharRef_setRow_0<(/*void*/)> for (u8) {
  fn setRow_0(self , rsthis: & QCharRef) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const u8 as usize;
     qtrt::InvokeQtFunc6("_ZN8QCharRef6setRowEh", 1,qtrt::FFITY_UINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qstring.h:1107
// index:0
// Public inline Visibility=Default Availability=Available
// [1] char toLatin1() const

/*
Returns a Latin-1 representation of the string as a QByteArray.

The returned byte array is undefined if the string contains non-Latin1 characters. Those characters may be suppressed or replaced with a question mark.

See also fromLatin1(), toUtf8(), toLocal8Bit(), QTextCodec, and qConvertToLatin1().
*/
impl /*struct*/ QCharRef {
  pub fn toLatin1_0<RetType, T: QCharRef_toLatin1_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toLatin1_0(self);
    // return 1;
  }
}
pub trait QCharRef_toLatin1_0<RetType> {
  fn toLatin1_0(self , rsthis: & QCharRef) -> RetType;
}
impl<'a> /*trait*/ QCharRef_toLatin1_0<i8> for () {
  fn toLatin1_0(self , rsthis: & QCharRef) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QCharRef8toLatin1Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i8 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1108
// index:0
// Public inline Visibility=Default Availability=Available
// [2] ushort unicode() const

/*
Returns a Unicode representation of the string. The result remains valid until the string is modified.

Note: The returned string may not be '\0'-terminated. Use size() to determine the length of the array.

See also setUnicode(), utf16(), and fromRawData().
*/
impl /*struct*/ QCharRef {
  pub fn unicode_0<RetType, T: QCharRef_unicode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.unicode_0(self);
    // return 1;
  }
}
pub trait QCharRef_unicode_0<RetType> {
  fn unicode_0(self , rsthis: & QCharRef) -> RetType;
}
impl<'a> /*trait*/ QCharRef_unicode_0<u16> for () {
  fn unicode_0(self , rsthis: & QCharRef) -> u16 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QCharRef7unicodeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u16 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstring.h:1109
// index:1
// Public inline Visibility=Default Availability=Available
// [2] ushort & unicode()

/*
Returns a Unicode representation of the string. The result remains valid until the string is modified.

Note: The returned string may not be '\0'-terminated. Use size() to determine the length of the array.

See also setUnicode(), utf16(), and fromRawData().
*/
impl /*struct*/ QCharRef {
  pub fn unicode_1<RetType, T: QCharRef_unicode_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.unicode_1(self);
    // return 1;
  }
}
pub trait QCharRef_unicode_1<RetType> {
  fn unicode_1(self , rsthis: & QCharRef) -> RetType;
}
impl<'a> /*trait*/ QCharRef_unicode_1<usize> for () {
  fn unicode_1(self , rsthis: & QCharRef) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QCharRef7unicodeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}


pub fn DeleteQCharRef(this :*mut QCharRef) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN8QCharRefD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
