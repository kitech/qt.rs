

// mod ::core::QMimeType
// package qtcore
// /usr/include/qt/QtCore/qmimetype.h
// #include <qmimetype.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 24
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
#[derive(Default)] // class sizeof(QMimeType)=8
pub struct QMimeType {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QMimeType_ITF interface {
//    QMimeType_PTR() *QMimeType
//}
//func (ptr *QMimeType) QMimeType_PTR() *QMimeType { return ptr }

impl /*struct*/ QMimeType {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QMimeType {
    return QMimeType{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QMimeType {
//  type Target = QMimeTypeBASE;
//
//  fn deref(&self) -> &QMimeTypeBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QMimeTypeBASE> for QMimeType {
//  fn as_ref(& self) -> & QMimeTypeBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qmimetype.h:78
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QMimeType()

/*
Constructs this QMimeType object initialized with default property values that indicate an invalid MIME type.
*/
// QMimeType() ctx.fn_proto_cpp
impl /*struct*/ QMimeType {
  pub fn QMimeType_0<T: QMimeType_QMimeType_0>(value: T) -> QMimeType {
    let rsthis = value.QMimeType_0();
    return rsthis;
    // return 1;
  }
}

pub trait QMimeType_QMimeType_0 {
  fn QMimeType_0(self) -> QMimeType;
}
// QMimeType() ctx.fn_proto_cpp
impl<'a> /*trait*/ QMimeType_QMimeType_0 for () {
  fn QMimeType_0(self) -> QMimeType {
    // unsafe{_ZN9QMimeTypeC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QMimeTypeC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QMimeType{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qmimetype.h:80
// index:0
// Public Visibility=Default Availability=Available
// [8] QMimeType & operator=(const QMimeType &)

/*

*/
impl /*struct*/ QMimeType {
  pub fn operator_equal_0<RetType, T: QMimeType_operator_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_0(self);
    // return 1;
  }
}
pub trait QMimeType_operator_equal_0<RetType> {
  fn operator_equal_0(self , rsthis: & QMimeType) -> RetType;
}
impl<'a> /*trait*/ QMimeType_operator_equal_0<usize> for (usize) {
  fn operator_equal_0(self , rsthis: & QMimeType) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QMimeTypeaSERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmimetype.h:82
// index:1
// Public inline Visibility=Default Availability=Available
// [8] QMimeType & operator=(QMimeType &&)

/*

*/
impl /*struct*/ QMimeType {
  pub fn operator_equal_1<RetType, T: QMimeType_operator_equal_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_1(self);
    // return 1;
  }
}
pub trait QMimeType_operator_equal_1<RetType> {
  fn operator_equal_1(self , rsthis: & QMimeType) -> RetType;
}
impl<'a> /*trait*/ QMimeType_operator_equal_1<usize> for (usize) {
  fn operator_equal_1(self , rsthis: & QMimeType) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QMimeTypeaSEOS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmimetype.h:84
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void swap(QMimeType &)

/*
Swaps QMimeType other with this QMimeType object.

This operation is very fast and never fails.

The swap() method helps with the implementation of assignment operators in an exception-safe way. For more information consult More C++ Idioms - Copy-and-swap.
*/
impl /*struct*/ QMimeType {
  pub fn swap_0<RetType, T: QMimeType_swap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.swap_0(self);
    // return 1;
  }
}
pub trait QMimeType_swap_0<RetType> {
  fn swap_0(self , rsthis: & QMimeType) -> RetType;
}
impl<'a> /*trait*/ QMimeType_swap_0<(/*void*/)> for (usize) {
  fn swap_0(self , rsthis: & QMimeType) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QMimeType4swapERS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qmimetype.h:89
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ~QMimeType()

/*

*/
pub fn DeleteQMimeType(this :*mut QMimeType) {
    // let rv = qtrt::InvokeQtFunc6("_ZN9QMimeTypeD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 8)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qmimetype.h:91
// index:0
// Public Visibility=Default Availability=Available
// [1] bool operator==(const QMimeType &) const

/*

*/
impl /*struct*/ QMimeType {
  pub fn operator_equal_equal_0<RetType, T: QMimeType_operator_equal_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_equal_0(self);
    // return 1;
  }
}
pub trait QMimeType_operator_equal_equal_0<RetType> {
  fn operator_equal_equal_0(self , rsthis: & QMimeType) -> RetType;
}
impl<'a> /*trait*/ QMimeType_operator_equal_equal_0<bool> for (usize) {
  fn operator_equal_equal_0(self , rsthis: & QMimeType) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QMimeTypeeqERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmimetype.h:93
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator!=(const QMimeType &) const

/*

*/
impl /*struct*/ QMimeType {
  pub fn operator_not_equal_0<RetType, T: QMimeType_operator_not_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_not_equal_0(self);
    // return 1;
  }
}
pub trait QMimeType_operator_not_equal_0<RetType> {
  fn operator_not_equal_0(self , rsthis: & QMimeType) -> RetType;
}
impl<'a> /*trait*/ QMimeType_operator_not_equal_0<bool> for (usize) {
  fn operator_not_equal_0(self , rsthis: & QMimeType) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QMimeTypeneERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmimetype.h:98
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isValid() const

/*

*/
impl /*struct*/ QMimeType {
  pub fn isValid_0<RetType, T: QMimeType_isValid_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isValid_0(self);
    // return 1;
  }
}
pub trait QMimeType_isValid_0<RetType> {
  fn isValid_0(self , rsthis: & QMimeType) -> RetType;
}
impl<'a> /*trait*/ QMimeType_isValid_0<bool> for () {
  fn isValid_0(self , rsthis: & QMimeType) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QMimeType7isValidEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmimetype.h:100
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isDefault() const

/*

*/
impl /*struct*/ QMimeType {
  pub fn isDefault_0<RetType, T: QMimeType_isDefault_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isDefault_0(self);
    // return 1;
  }
}
pub trait QMimeType_isDefault_0<RetType> {
  fn isDefault_0(self , rsthis: & QMimeType) -> RetType;
}
impl<'a> /*trait*/ QMimeType_isDefault_0<bool> for () {
  fn isDefault_0(self , rsthis: & QMimeType) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QMimeType9isDefaultEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmimetype.h:102
// index:0
// Public Visibility=Default Availability=Available
// [8] QString name() const

/*

*/
impl /*struct*/ QMimeType {
  pub fn name_0<RetType, T: QMimeType_name_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.name_0(self);
    // return 1;
  }
}
pub trait QMimeType_name_0<RetType> {
  fn name_0(self , rsthis: & QMimeType) -> RetType;
}
impl<'a> /*trait*/ QMimeType_name_0<usize> for () {
  fn name_0(self , rsthis: & QMimeType) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QMimeType4nameEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmimetype.h:103
// index:0
// Public Visibility=Default Availability=Available
// [8] QString comment() const

/*

*/
impl /*struct*/ QMimeType {
  pub fn comment_0<RetType, T: QMimeType_comment_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.comment_0(self);
    // return 1;
  }
}
pub trait QMimeType_comment_0<RetType> {
  fn comment_0(self , rsthis: & QMimeType) -> RetType;
}
impl<'a> /*trait*/ QMimeType_comment_0<usize> for () {
  fn comment_0(self , rsthis: & QMimeType) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QMimeType7commentEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmimetype.h:104
// index:0
// Public Visibility=Default Availability=Available
// [8] QString genericIconName() const

/*

*/
impl /*struct*/ QMimeType {
  pub fn genericIconName_0<RetType, T: QMimeType_genericIconName_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.genericIconName_0(self);
    // return 1;
  }
}
pub trait QMimeType_genericIconName_0<RetType> {
  fn genericIconName_0(self , rsthis: & QMimeType) -> RetType;
}
impl<'a> /*trait*/ QMimeType_genericIconName_0<usize> for () {
  fn genericIconName_0(self , rsthis: & QMimeType) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QMimeType15genericIconNameEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmimetype.h:105
// index:0
// Public Visibility=Default Availability=Available
// [8] QString iconName() const

/*

*/
impl /*struct*/ QMimeType {
  pub fn iconName_0<RetType, T: QMimeType_iconName_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.iconName_0(self);
    // return 1;
  }
}
pub trait QMimeType_iconName_0<RetType> {
  fn iconName_0(self , rsthis: & QMimeType) -> RetType;
}
impl<'a> /*trait*/ QMimeType_iconName_0<usize> for () {
  fn iconName_0(self , rsthis: & QMimeType) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QMimeType8iconNameEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmimetype.h:106
// index:0
// Public Visibility=Default Availability=Available
// [8] QStringList globPatterns() const

/*

*/
impl /*struct*/ QMimeType {
  pub fn globPatterns_0<RetType, T: QMimeType_globPatterns_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.globPatterns_0(self);
    // return 1;
  }
}
pub trait QMimeType_globPatterns_0<RetType> {
  fn globPatterns_0(self , rsthis: & QMimeType) -> RetType;
}
impl<'a> /*trait*/ QMimeType_globPatterns_0<usize> for () {
  fn globPatterns_0(self , rsthis: & QMimeType) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QMimeType12globPatternsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmimetype.h:107
// index:0
// Public Visibility=Default Availability=Available
// [8] QStringList parentMimeTypes() const

/*

*/
impl /*struct*/ QMimeType {
  pub fn parentMimeTypes_0<RetType, T: QMimeType_parentMimeTypes_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.parentMimeTypes_0(self);
    // return 1;
  }
}
pub trait QMimeType_parentMimeTypes_0<RetType> {
  fn parentMimeTypes_0(self , rsthis: & QMimeType) -> RetType;
}
impl<'a> /*trait*/ QMimeType_parentMimeTypes_0<usize> for () {
  fn parentMimeTypes_0(self , rsthis: & QMimeType) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QMimeType15parentMimeTypesEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmimetype.h:108
// index:0
// Public Visibility=Default Availability=Available
// [8] QStringList allAncestors() const

/*

*/
impl /*struct*/ QMimeType {
  pub fn allAncestors_0<RetType, T: QMimeType_allAncestors_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.allAncestors_0(self);
    // return 1;
  }
}
pub trait QMimeType_allAncestors_0<RetType> {
  fn allAncestors_0(self , rsthis: & QMimeType) -> RetType;
}
impl<'a> /*trait*/ QMimeType_allAncestors_0<usize> for () {
  fn allAncestors_0(self , rsthis: & QMimeType) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QMimeType12allAncestorsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmimetype.h:109
// index:0
// Public Visibility=Default Availability=Available
// [8] QStringList aliases() const

/*

*/
impl /*struct*/ QMimeType {
  pub fn aliases_0<RetType, T: QMimeType_aliases_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.aliases_0(self);
    // return 1;
  }
}
pub trait QMimeType_aliases_0<RetType> {
  fn aliases_0(self , rsthis: & QMimeType) -> RetType;
}
impl<'a> /*trait*/ QMimeType_aliases_0<usize> for () {
  fn aliases_0(self , rsthis: & QMimeType) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QMimeType7aliasesEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmimetype.h:110
// index:0
// Public Visibility=Default Availability=Available
// [8] QStringList suffixes() const

/*

*/
impl /*struct*/ QMimeType {
  pub fn suffixes_0<RetType, T: QMimeType_suffixes_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.suffixes_0(self);
    // return 1;
  }
}
pub trait QMimeType_suffixes_0<RetType> {
  fn suffixes_0(self , rsthis: & QMimeType) -> RetType;
}
impl<'a> /*trait*/ QMimeType_suffixes_0<usize> for () {
  fn suffixes_0(self , rsthis: & QMimeType) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QMimeType8suffixesEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmimetype.h:111
// index:0
// Public Visibility=Default Availability=Available
// [8] QString preferredSuffix() const

/*

*/
impl /*struct*/ QMimeType {
  pub fn preferredSuffix_0<RetType, T: QMimeType_preferredSuffix_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.preferredSuffix_0(self);
    // return 1;
  }
}
pub trait QMimeType_preferredSuffix_0<RetType> {
  fn preferredSuffix_0(self , rsthis: & QMimeType) -> RetType;
}
impl<'a> /*trait*/ QMimeType_preferredSuffix_0<usize> for () {
  fn preferredSuffix_0(self , rsthis: & QMimeType) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QMimeType15preferredSuffixEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmimetype.h:113
// index:0
// Public Visibility=Default Availability=Available
// [1] bool inherits(const QString &) const

/*
Returns true if this mimetype is mimeTypeName, or inherits mimeTypeName (see parentMimeTypes()), or mimeTypeName is an alias for this mimetype.

This method has been made invokable from QML since 5.10.
*/
impl /*struct*/ QMimeType {
  pub fn inherits_0<RetType, T: QMimeType_inherits_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.inherits_0(self);
    // return 1;
  }
}
pub trait QMimeType_inherits_0<RetType> {
  fn inherits_0(self , rsthis: & QMimeType) -> RetType;
}
impl<'a> /*trait*/ QMimeType_inherits_0<bool> for (usize) {
  fn inherits_0(self , rsthis: & QMimeType) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QMimeType8inheritsERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmimetype.h:115
// index:0
// Public Visibility=Default Availability=Available
// [8] QString filterString() const

/*

*/
impl /*struct*/ QMimeType {
  pub fn filterString_0<RetType, T: QMimeType_filterString_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.filterString_0(self);
    // return 1;
  }
}
pub trait QMimeType_filterString_0<RetType> {
  fn filterString_0(self , rsthis: & QMimeType) -> RetType;
}
impl<'a> /*trait*/ QMimeType_filterString_0<usize> for () {
  fn filterString_0(self , rsthis: & QMimeType) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QMimeType12filterStringEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
