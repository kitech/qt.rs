

// mod ::core::QMetaEnum
// package qtcore
// /usr/include/qt/QtCore/qmetaobject.h
// #include <qmetaobject.h>
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
#[derive(Default)] // class sizeof(QMetaEnum)=16
pub struct QMetaEnum {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QMetaEnum_ITF interface {
//    QMetaEnum_PTR() *QMetaEnum
//}
//func (ptr *QMetaEnum) QMetaEnum_PTR() *QMetaEnum { return ptr }

impl /*struct*/ QMetaEnum {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QMetaEnum {
    return QMetaEnum{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QMetaEnum {
//  type Target = QMetaEnumBASE;
//
//  fn deref(&self) -> &QMetaEnumBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QMetaEnumBASE> for QMetaEnum {
//  fn as_ref(& self) -> & QMetaEnumBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qmetaobject.h:209
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void QMetaEnum()

/*

*/
// QMetaEnum() ctx.fn_proto_cpp
impl /*struct*/ QMetaEnum {
  pub fn QMetaEnum_0<T: QMetaEnum_QMetaEnum_0>(value: T) -> QMetaEnum {
    let rsthis = value.QMetaEnum_0();
    return rsthis;
    // return 1;
  }
}

pub trait QMetaEnum_QMetaEnum_0 {
  fn QMetaEnum_0(self) -> QMetaEnum;
}
// QMetaEnum() ctx.fn_proto_cpp
impl<'a> /*trait*/ QMetaEnum_QMetaEnum_0 for () {
  fn QMetaEnum_0(self) -> QMetaEnum {
    // unsafe{_ZN9QMetaEnumC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QMetaEnumC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QMetaEnum{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qmetaobject.h:211
// index:0
// Public Visibility=Default Availability=Available
// [8] const char * name() const

/*

*/
impl /*struct*/ QMetaEnum {
  pub fn name_0<RetType, T: QMetaEnum_name_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.name_0(self);
    // return 1;
  }
}
pub trait QMetaEnum_name_0<RetType> {
  fn name_0(self , rsthis: & QMetaEnum) -> RetType;
}
impl<'a> /*trait*/ QMetaEnum_name_0<usize> for () {
  fn name_0(self , rsthis: & QMetaEnum) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QMetaEnum4nameEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmetaobject.h:212
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isFlag() const

/*

*/
impl /*struct*/ QMetaEnum {
  pub fn isFlag_0<RetType, T: QMetaEnum_isFlag_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isFlag_0(self);
    // return 1;
  }
}
pub trait QMetaEnum_isFlag_0<RetType> {
  fn isFlag_0(self , rsthis: & QMetaEnum) -> RetType;
}
impl<'a> /*trait*/ QMetaEnum_isFlag_0<bool> for () {
  fn isFlag_0(self , rsthis: & QMetaEnum) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QMetaEnum6isFlagEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmetaobject.h:213
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isScoped() const

/*

*/
impl /*struct*/ QMetaEnum {
  pub fn isScoped_0<RetType, T: QMetaEnum_isScoped_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isScoped_0(self);
    // return 1;
  }
}
pub trait QMetaEnum_isScoped_0<RetType> {
  fn isScoped_0(self , rsthis: & QMetaEnum) -> RetType;
}
impl<'a> /*trait*/ QMetaEnum_isScoped_0<bool> for () {
  fn isScoped_0(self , rsthis: & QMetaEnum) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QMetaEnum8isScopedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmetaobject.h:215
// index:0
// Public Visibility=Default Availability=Available
// [4] int keyCount() const

/*

*/
impl /*struct*/ QMetaEnum {
  pub fn keyCount_0<RetType, T: QMetaEnum_keyCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.keyCount_0(self);
    // return 1;
  }
}
pub trait QMetaEnum_keyCount_0<RetType> {
  fn keyCount_0(self , rsthis: & QMetaEnum) -> RetType;
}
impl<'a> /*trait*/ QMetaEnum_keyCount_0<i32> for () {
  fn keyCount_0(self , rsthis: & QMetaEnum) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QMetaEnum8keyCountEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmetaobject.h:216
// index:0
// Public Visibility=Default Availability=Available
// [8] const char * key(int) const

/*

*/
impl /*struct*/ QMetaEnum {
  pub fn key_0<RetType, T: QMetaEnum_key_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.key_0(self);
    // return 1;
  }
}
pub trait QMetaEnum_key_0<RetType> {
  fn key_0(self , rsthis: & QMetaEnum) -> RetType;
}
impl<'a> /*trait*/ QMetaEnum_key_0<usize> for (i32) {
  fn key_0(self , rsthis: & QMetaEnum) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QMetaEnum3keyEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmetaobject.h:217
// index:0
// Public Visibility=Default Availability=Available
// [4] int value(int) const

/*

*/
impl /*struct*/ QMetaEnum {
  pub fn value_0<RetType, T: QMetaEnum_value_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.value_0(self);
    // return 1;
  }
}
pub trait QMetaEnum_value_0<RetType> {
  fn value_0(self , rsthis: & QMetaEnum) -> RetType;
}
impl<'a> /*trait*/ QMetaEnum_value_0<i32> for (i32) {
  fn value_0(self , rsthis: & QMetaEnum) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QMetaEnum5valueEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmetaobject.h:219
// index:0
// Public Visibility=Default Availability=Available
// [8] const char * scope() const

/*

*/
impl /*struct*/ QMetaEnum {
  pub fn scope_0<RetType, T: QMetaEnum_scope_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.scope_0(self);
    // return 1;
  }
}
pub trait QMetaEnum_scope_0<RetType> {
  fn scope_0(self , rsthis: & QMetaEnum) -> RetType;
}
impl<'a> /*trait*/ QMetaEnum_scope_0<usize> for () {
  fn scope_0(self , rsthis: & QMetaEnum) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QMetaEnum5scopeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmetaobject.h:221
// index:0
// Public Visibility=Default Availability=Available
// [4] int keyToValue(const char *, bool *) const

/*

*/
impl /*struct*/ QMetaEnum {
  pub fn keyToValue_0<RetType, T: QMetaEnum_keyToValue_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.keyToValue_0(self);
    // return 1;
  }
}
pub trait QMetaEnum_keyToValue_0<RetType> {
  fn keyToValue_0(self , rsthis: & QMetaEnum) -> RetType;
}
impl<'a> /*trait*/ QMetaEnum_keyToValue_0<i32> for (usize,usize) {
  fn keyToValue_0(self , rsthis: & QMetaEnum) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QMetaEnum10keyToValueEPKcPb", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmetaobject.h:222
// index:0
// Public Visibility=Default Availability=Available
// [8] const char * valueToKey(int) const

/*

*/
impl /*struct*/ QMetaEnum {
  pub fn valueToKey_0<RetType, T: QMetaEnum_valueToKey_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.valueToKey_0(self);
    // return 1;
  }
}
pub trait QMetaEnum_valueToKey_0<RetType> {
  fn valueToKey_0(self , rsthis: & QMetaEnum) -> RetType;
}
impl<'a> /*trait*/ QMetaEnum_valueToKey_0<usize> for (i32) {
  fn valueToKey_0(self , rsthis: & QMetaEnum) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QMetaEnum10valueToKeyEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmetaobject.h:223
// index:0
// Public Visibility=Default Availability=Available
// [4] int keysToValue(const char *, bool *) const

/*

*/
impl /*struct*/ QMetaEnum {
  pub fn keysToValue_0<RetType, T: QMetaEnum_keysToValue_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.keysToValue_0(self);
    // return 1;
  }
}
pub trait QMetaEnum_keysToValue_0<RetType> {
  fn keysToValue_0(self , rsthis: & QMetaEnum) -> RetType;
}
impl<'a> /*trait*/ QMetaEnum_keysToValue_0<i32> for (usize,usize) {
  fn keysToValue_0(self , rsthis: & QMetaEnum) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QMetaEnum11keysToValueEPKcPb", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmetaobject.h:224
// index:0
// Public Visibility=Default Availability=Available
// [8] QByteArray valueToKeys(int) const

/*

*/
impl /*struct*/ QMetaEnum {
  pub fn valueToKeys_0<RetType, T: QMetaEnum_valueToKeys_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.valueToKeys_0(self);
    // return 1;
  }
}
pub trait QMetaEnum_valueToKeys_0<RetType> {
  fn valueToKeys_0(self , rsthis: & QMetaEnum) -> RetType;
}
impl<'a> /*trait*/ QMetaEnum_valueToKeys_0<usize> for (i32) {
  fn valueToKeys_0(self , rsthis: & QMetaEnum) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QMetaEnum11valueToKeysEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmetaobject.h:226
// index:0
// Public inline Visibility=Default Availability=Available
// [8] const QMetaObject * enclosingMetaObject() const

/*

*/
impl /*struct*/ QMetaEnum {
  pub fn enclosingMetaObject_0<RetType, T: QMetaEnum_enclosingMetaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.enclosingMetaObject_0(self);
    // return 1;
  }
}
pub trait QMetaEnum_enclosingMetaObject_0<RetType> {
  fn enclosingMetaObject_0(self , rsthis: & QMetaEnum) -> RetType;
}
impl<'a> /*trait*/ QMetaEnum_enclosingMetaObject_0<usize> for () {
  fn enclosingMetaObject_0(self , rsthis: & QMetaEnum) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QMetaEnum19enclosingMetaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmetaobject.h:228
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isValid() const

/*

*/
impl /*struct*/ QMetaEnum {
  pub fn isValid_0<RetType, T: QMetaEnum_isValid_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isValid_0(self);
    // return 1;
  }
}
pub trait QMetaEnum_isValid_0<RetType> {
  fn isValid_0(self , rsthis: & QMetaEnum) -> RetType;
}
impl<'a> /*trait*/ QMetaEnum_isValid_0<bool> for () {
  fn isValid_0(self , rsthis: & QMetaEnum) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QMetaEnum7isValidEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}


pub fn DeleteQMetaEnum(this :*mut QMetaEnum) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN9QMetaEnumD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
