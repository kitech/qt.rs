

// mod ::core::QMetaProperty
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
// extern C begin: 14
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
#[derive(Default)] // class sizeof(QMetaProperty)=32
pub struct QMetaProperty {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QMetaProperty_ITF interface {
//    QMetaProperty_PTR() *QMetaProperty
//}
//func (ptr *QMetaProperty) QMetaProperty_PTR() *QMetaProperty { return ptr }

impl /*struct*/ QMetaProperty {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QMetaProperty {
    return QMetaProperty{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QMetaProperty {
//  type Target = QMetaPropertyBASE;
//
//  fn deref(&self) -> &QMetaPropertyBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QMetaPropertyBASE> for QMetaProperty {
//  fn as_ref(& self) -> & QMetaPropertyBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qmetaobject.h:248
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QMetaProperty()

/*

*/
// QMetaProperty() ctx.fn_proto_cpp
impl /*struct*/ QMetaProperty {
  pub fn QMetaProperty_0<T: QMetaProperty_QMetaProperty_0>(value: T) -> QMetaProperty {
    let rsthis = value.QMetaProperty_0();
    return rsthis;
    // return 1;
  }
}

pub trait QMetaProperty_QMetaProperty_0 {
  fn QMetaProperty_0(self) -> QMetaProperty;
}
// QMetaProperty() ctx.fn_proto_cpp
impl<'a> /*trait*/ QMetaProperty_QMetaProperty_0 for () {
  fn QMetaProperty_0(self) -> QMetaProperty {
    // unsafe{_ZN13QMetaPropertyC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN13QMetaPropertyC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QMetaProperty{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qmetaobject.h:250
// index:0
// Public Visibility=Default Availability=Available
// [8] const char * name() const

/*

*/
impl /*struct*/ QMetaProperty {
  pub fn name_0<RetType, T: QMetaProperty_name_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.name_0(self);
    // return 1;
  }
}
pub trait QMetaProperty_name_0<RetType> {
  fn name_0(self , rsthis: & QMetaProperty) -> RetType;
}
impl<'a> /*trait*/ QMetaProperty_name_0<usize> for () {
  fn name_0(self , rsthis: & QMetaProperty) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QMetaProperty4nameEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmetaobject.h:251
// index:0
// Public Visibility=Default Availability=Available
// [8] const char * typeName() const

/*

*/
impl /*struct*/ QMetaProperty {
  pub fn typeName_0<RetType, T: QMetaProperty_typeName_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.typeName_0(self);
    // return 1;
  }
}
pub trait QMetaProperty_typeName_0<RetType> {
  fn typeName_0(self , rsthis: & QMetaProperty) -> RetType;
}
impl<'a> /*trait*/ QMetaProperty_typeName_0<usize> for () {
  fn typeName_0(self , rsthis: & QMetaProperty) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QMetaProperty8typeNameEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmetaobject.h:252
// index:0
// Public Visibility=Default Availability=Available
// [4] QVariant::Type type() const

/*

*/
impl /*struct*/ QMetaProperty {
  pub fn type__0<RetType, T: QMetaProperty_type__0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.type__0(self);
    // return 1;
  }
}
pub trait QMetaProperty_type__0<RetType> {
  fn type__0(self , rsthis: & QMetaProperty) -> RetType;
}
impl<'a> /*trait*/ QMetaProperty_type__0<i32> for () {
  fn type__0(self , rsthis: & QMetaProperty) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QMetaProperty4typeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmetaobject.h:253
// index:0
// Public Visibility=Default Availability=Available
// [4] int userType() const

/*

*/
impl /*struct*/ QMetaProperty {
  pub fn userType_0<RetType, T: QMetaProperty_userType_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.userType_0(self);
    // return 1;
  }
}
pub trait QMetaProperty_userType_0<RetType> {
  fn userType_0(self , rsthis: & QMetaProperty) -> RetType;
}
impl<'a> /*trait*/ QMetaProperty_userType_0<i32> for () {
  fn userType_0(self , rsthis: & QMetaProperty) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QMetaProperty8userTypeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmetaobject.h:254
// index:0
// Public Visibility=Default Availability=Available
// [4] int propertyIndex() const

/*

*/
impl /*struct*/ QMetaProperty {
  pub fn propertyIndex_0<RetType, T: QMetaProperty_propertyIndex_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.propertyIndex_0(self);
    // return 1;
  }
}
pub trait QMetaProperty_propertyIndex_0<RetType> {
  fn propertyIndex_0(self , rsthis: & QMetaProperty) -> RetType;
}
impl<'a> /*trait*/ QMetaProperty_propertyIndex_0<i32> for () {
  fn propertyIndex_0(self , rsthis: & QMetaProperty) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QMetaProperty13propertyIndexEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmetaobject.h:256
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isReadable() const

/*

*/
impl /*struct*/ QMetaProperty {
  pub fn isReadable_0<RetType, T: QMetaProperty_isReadable_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isReadable_0(self);
    // return 1;
  }
}
pub trait QMetaProperty_isReadable_0<RetType> {
  fn isReadable_0(self , rsthis: & QMetaProperty) -> RetType;
}
impl<'a> /*trait*/ QMetaProperty_isReadable_0<bool> for () {
  fn isReadable_0(self , rsthis: & QMetaProperty) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QMetaProperty10isReadableEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmetaobject.h:257
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isWritable() const

/*

*/
impl /*struct*/ QMetaProperty {
  pub fn isWritable_0<RetType, T: QMetaProperty_isWritable_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isWritable_0(self);
    // return 1;
  }
}
pub trait QMetaProperty_isWritable_0<RetType> {
  fn isWritable_0(self , rsthis: & QMetaProperty) -> RetType;
}
impl<'a> /*trait*/ QMetaProperty_isWritable_0<bool> for () {
  fn isWritable_0(self , rsthis: & QMetaProperty) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QMetaProperty10isWritableEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmetaobject.h:258
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isResettable() const

/*

*/
impl /*struct*/ QMetaProperty {
  pub fn isResettable_0<RetType, T: QMetaProperty_isResettable_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isResettable_0(self);
    // return 1;
  }
}
pub trait QMetaProperty_isResettable_0<RetType> {
  fn isResettable_0(self , rsthis: & QMetaProperty) -> RetType;
}
impl<'a> /*trait*/ QMetaProperty_isResettable_0<bool> for () {
  fn isResettable_0(self , rsthis: & QMetaProperty) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QMetaProperty12isResettableEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmetaobject.h:259
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isDesignable(const QObject *) const

/*

*/
impl /*struct*/ QMetaProperty {
  pub fn isDesignable_0<RetType, T: QMetaProperty_isDesignable_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isDesignable_0(self);
    // return 1;
  }
}
pub trait QMetaProperty_isDesignable_0<RetType> {
  fn isDesignable_0(self , rsthis: & QMetaProperty) -> RetType;
}
impl<'a> /*trait*/ QMetaProperty_isDesignable_0<bool> for (usize) {
  fn isDesignable_0(self , rsthis: & QMetaProperty) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QMetaProperty12isDesignableEPK7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmetaobject.h:260
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isScriptable(const QObject *) const

/*

*/
impl /*struct*/ QMetaProperty {
  pub fn isScriptable_0<RetType, T: QMetaProperty_isScriptable_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isScriptable_0(self);
    // return 1;
  }
}
pub trait QMetaProperty_isScriptable_0<RetType> {
  fn isScriptable_0(self , rsthis: & QMetaProperty) -> RetType;
}
impl<'a> /*trait*/ QMetaProperty_isScriptable_0<bool> for (usize) {
  fn isScriptable_0(self , rsthis: & QMetaProperty) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QMetaProperty12isScriptableEPK7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmetaobject.h:261
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isStored(const QObject *) const

/*

*/
impl /*struct*/ QMetaProperty {
  pub fn isStored_0<RetType, T: QMetaProperty_isStored_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isStored_0(self);
    // return 1;
  }
}
pub trait QMetaProperty_isStored_0<RetType> {
  fn isStored_0(self , rsthis: & QMetaProperty) -> RetType;
}
impl<'a> /*trait*/ QMetaProperty_isStored_0<bool> for (usize) {
  fn isStored_0(self , rsthis: & QMetaProperty) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QMetaProperty8isStoredEPK7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmetaobject.h:262
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isEditable(const QObject *) const

/*

*/
impl /*struct*/ QMetaProperty {
  pub fn isEditable_0<RetType, T: QMetaProperty_isEditable_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isEditable_0(self);
    // return 1;
  }
}
pub trait QMetaProperty_isEditable_0<RetType> {
  fn isEditable_0(self , rsthis: & QMetaProperty) -> RetType;
}
impl<'a> /*trait*/ QMetaProperty_isEditable_0<bool> for (usize) {
  fn isEditable_0(self , rsthis: & QMetaProperty) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QMetaProperty10isEditableEPK7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmetaobject.h:263
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isUser(const QObject *) const

/*

*/
impl /*struct*/ QMetaProperty {
  pub fn isUser_0<RetType, T: QMetaProperty_isUser_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isUser_0(self);
    // return 1;
  }
}
pub trait QMetaProperty_isUser_0<RetType> {
  fn isUser_0(self , rsthis: & QMetaProperty) -> RetType;
}
impl<'a> /*trait*/ QMetaProperty_isUser_0<bool> for (usize) {
  fn isUser_0(self , rsthis: & QMetaProperty) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QMetaProperty6isUserEPK7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmetaobject.h:264
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isConstant() const

/*

*/
impl /*struct*/ QMetaProperty {
  pub fn isConstant_0<RetType, T: QMetaProperty_isConstant_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isConstant_0(self);
    // return 1;
  }
}
pub trait QMetaProperty_isConstant_0<RetType> {
  fn isConstant_0(self , rsthis: & QMetaProperty) -> RetType;
}
impl<'a> /*trait*/ QMetaProperty_isConstant_0<bool> for () {
  fn isConstant_0(self , rsthis: & QMetaProperty) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QMetaProperty10isConstantEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmetaobject.h:265
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isFinal() const

/*

*/
impl /*struct*/ QMetaProperty {
  pub fn isFinal_0<RetType, T: QMetaProperty_isFinal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isFinal_0(self);
    // return 1;
  }
}
pub trait QMetaProperty_isFinal_0<RetType> {
  fn isFinal_0(self , rsthis: & QMetaProperty) -> RetType;
}
impl<'a> /*trait*/ QMetaProperty_isFinal_0<bool> for () {
  fn isFinal_0(self , rsthis: & QMetaProperty) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QMetaProperty7isFinalEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmetaobject.h:267
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isFlagType() const

/*

*/
impl /*struct*/ QMetaProperty {
  pub fn isFlagType_0<RetType, T: QMetaProperty_isFlagType_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isFlagType_0(self);
    // return 1;
  }
}
pub trait QMetaProperty_isFlagType_0<RetType> {
  fn isFlagType_0(self , rsthis: & QMetaProperty) -> RetType;
}
impl<'a> /*trait*/ QMetaProperty_isFlagType_0<bool> for () {
  fn isFlagType_0(self , rsthis: & QMetaProperty) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QMetaProperty10isFlagTypeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmetaobject.h:268
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isEnumType() const

/*

*/
impl /*struct*/ QMetaProperty {
  pub fn isEnumType_0<RetType, T: QMetaProperty_isEnumType_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isEnumType_0(self);
    // return 1;
  }
}
pub trait QMetaProperty_isEnumType_0<RetType> {
  fn isEnumType_0(self , rsthis: & QMetaProperty) -> RetType;
}
impl<'a> /*trait*/ QMetaProperty_isEnumType_0<bool> for () {
  fn isEnumType_0(self , rsthis: & QMetaProperty) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QMetaProperty10isEnumTypeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmetaobject.h:269
// index:0
// Public Visibility=Default Availability=Available
// [16] QMetaEnum enumerator() const

/*
Returns the meta-data for the enumerator with the given index.

See also enumeratorCount(), enumeratorOffset(), and indexOfEnumerator().
*/
impl /*struct*/ QMetaProperty {
  pub fn enumerator_0<RetType, T: QMetaProperty_enumerator_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.enumerator_0(self);
    // return 1;
  }
}
pub trait QMetaProperty_enumerator_0<RetType> {
  fn enumerator_0(self , rsthis: & QMetaProperty) -> RetType;
}
impl<'a> /*trait*/ QMetaProperty_enumerator_0<usize> for () {
  fn enumerator_0(self , rsthis: & QMetaProperty) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QMetaProperty10enumeratorEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmetaobject.h:271
// index:0
// Public Visibility=Default Availability=Available
// [1] bool hasNotifySignal() const

/*

*/
impl /*struct*/ QMetaProperty {
  pub fn hasNotifySignal_0<RetType, T: QMetaProperty_hasNotifySignal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hasNotifySignal_0(self);
    // return 1;
  }
}
pub trait QMetaProperty_hasNotifySignal_0<RetType> {
  fn hasNotifySignal_0(self , rsthis: & QMetaProperty) -> RetType;
}
impl<'a> /*trait*/ QMetaProperty_hasNotifySignal_0<bool> for () {
  fn hasNotifySignal_0(self , rsthis: & QMetaProperty) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QMetaProperty15hasNotifySignalEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmetaobject.h:272
// index:0
// Public Visibility=Default Availability=Available
// [16] QMetaMethod notifySignal() const

/*

*/
impl /*struct*/ QMetaProperty {
  pub fn notifySignal_0<RetType, T: QMetaProperty_notifySignal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.notifySignal_0(self);
    // return 1;
  }
}
pub trait QMetaProperty_notifySignal_0<RetType> {
  fn notifySignal_0(self , rsthis: & QMetaProperty) -> RetType;
}
impl<'a> /*trait*/ QMetaProperty_notifySignal_0<usize> for () {
  fn notifySignal_0(self , rsthis: & QMetaProperty) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QMetaProperty12notifySignalEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmetaobject.h:273
// index:0
// Public Visibility=Default Availability=Available
// [4] int notifySignalIndex() const

/*

*/
impl /*struct*/ QMetaProperty {
  pub fn notifySignalIndex_0<RetType, T: QMetaProperty_notifySignalIndex_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.notifySignalIndex_0(self);
    // return 1;
  }
}
pub trait QMetaProperty_notifySignalIndex_0<RetType> {
  fn notifySignalIndex_0(self , rsthis: & QMetaProperty) -> RetType;
}
impl<'a> /*trait*/ QMetaProperty_notifySignalIndex_0<i32> for () {
  fn notifySignalIndex_0(self , rsthis: & QMetaProperty) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QMetaProperty17notifySignalIndexEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmetaobject.h:275
// index:0
// Public Visibility=Default Availability=Available
// [4] int revision() const

/*

*/
impl /*struct*/ QMetaProperty {
  pub fn revision_0<RetType, T: QMetaProperty_revision_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.revision_0(self);
    // return 1;
  }
}
pub trait QMetaProperty_revision_0<RetType> {
  fn revision_0(self , rsthis: & QMetaProperty) -> RetType;
}
impl<'a> /*trait*/ QMetaProperty_revision_0<i32> for () {
  fn revision_0(self , rsthis: & QMetaProperty) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QMetaProperty8revisionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmetaobject.h:277
// index:0
// Public Visibility=Default Availability=Available
// [16] QVariant read(const QObject *) const

/*

*/
impl /*struct*/ QMetaProperty {
  pub fn read_0<RetType, T: QMetaProperty_read_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.read_0(self);
    // return 1;
  }
}
pub trait QMetaProperty_read_0<RetType> {
  fn read_0(self , rsthis: & QMetaProperty) -> RetType;
}
impl<'a> /*trait*/ QMetaProperty_read_0<usize> for (usize) {
  fn read_0(self , rsthis: & QMetaProperty) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QMetaProperty4readEPK7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmetaobject.h:278
// index:0
// Public Visibility=Default Availability=Available
// [1] bool write(QObject *, const QVariant &) const

/*

*/
impl /*struct*/ QMetaProperty {
  pub fn write_0<RetType, T: QMetaProperty_write_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.write_0(self);
    // return 1;
  }
}
pub trait QMetaProperty_write_0<RetType> {
  fn write_0(self , rsthis: & QMetaProperty) -> RetType;
}
impl<'a> /*trait*/ QMetaProperty_write_0<bool> for (usize,usize) {
  fn write_0(self , rsthis: & QMetaProperty) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QMetaProperty5writeEP7QObjectRK8QVariant", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmetaobject.h:279
// index:0
// Public Visibility=Default Availability=Available
// [1] bool reset(QObject *) const

/*

*/
impl /*struct*/ QMetaProperty {
  pub fn reset_0<RetType, T: QMetaProperty_reset_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.reset_0(self);
    // return 1;
  }
}
pub trait QMetaProperty_reset_0<RetType> {
  fn reset_0(self , rsthis: & QMetaProperty) -> RetType;
}
impl<'a> /*trait*/ QMetaProperty_reset_0<bool> for (usize) {
  fn reset_0(self , rsthis: & QMetaProperty) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QMetaProperty5resetEP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmetaobject.h:281
// index:0
// Public Visibility=Default Availability=Available
// [16] QVariant readOnGadget(const void *) const

/*

*/
impl /*struct*/ QMetaProperty {
  pub fn readOnGadget_0<RetType, T: QMetaProperty_readOnGadget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.readOnGadget_0(self);
    // return 1;
  }
}
pub trait QMetaProperty_readOnGadget_0<RetType> {
  fn readOnGadget_0(self , rsthis: & QMetaProperty) -> RetType;
}
impl<'a> /*trait*/ QMetaProperty_readOnGadget_0<usize> for (usize) {
  fn readOnGadget_0(self , rsthis: & QMetaProperty) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QMetaProperty12readOnGadgetEPKv", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmetaobject.h:282
// index:0
// Public Visibility=Default Availability=Available
// [1] bool writeOnGadget(void *, const QVariant &) const

/*

*/
impl /*struct*/ QMetaProperty {
  pub fn writeOnGadget_0<RetType, T: QMetaProperty_writeOnGadget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.writeOnGadget_0(self);
    // return 1;
  }
}
pub trait QMetaProperty_writeOnGadget_0<RetType> {
  fn writeOnGadget_0(self , rsthis: & QMetaProperty) -> RetType;
}
impl<'a> /*trait*/ QMetaProperty_writeOnGadget_0<bool> for (usize,usize) {
  fn writeOnGadget_0(self , rsthis: & QMetaProperty) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QMetaProperty13writeOnGadgetEPvRK8QVariant", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmetaobject.h:283
// index:0
// Public Visibility=Default Availability=Available
// [1] bool resetOnGadget(void *) const

/*

*/
impl /*struct*/ QMetaProperty {
  pub fn resetOnGadget_0<RetType, T: QMetaProperty_resetOnGadget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resetOnGadget_0(self);
    // return 1;
  }
}
pub trait QMetaProperty_resetOnGadget_0<RetType> {
  fn resetOnGadget_0(self , rsthis: & QMetaProperty) -> RetType;
}
impl<'a> /*trait*/ QMetaProperty_resetOnGadget_0<bool> for (usize) {
  fn resetOnGadget_0(self , rsthis: & QMetaProperty) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QMetaProperty13resetOnGadgetEPv", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmetaobject.h:285
// index:0
// Public Visibility=Default Availability=Available
// [1] bool hasStdCppSet() const

/*

*/
impl /*struct*/ QMetaProperty {
  pub fn hasStdCppSet_0<RetType, T: QMetaProperty_hasStdCppSet_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hasStdCppSet_0(self);
    // return 1;
  }
}
pub trait QMetaProperty_hasStdCppSet_0<RetType> {
  fn hasStdCppSet_0(self , rsthis: & QMetaProperty) -> RetType;
}
impl<'a> /*trait*/ QMetaProperty_hasStdCppSet_0<bool> for () {
  fn hasStdCppSet_0(self , rsthis: & QMetaProperty) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QMetaProperty12hasStdCppSetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmetaobject.h:286
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isValid() const

/*

*/
impl /*struct*/ QMetaProperty {
  pub fn isValid_0<RetType, T: QMetaProperty_isValid_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isValid_0(self);
    // return 1;
  }
}
pub trait QMetaProperty_isValid_0<RetType> {
  fn isValid_0(self , rsthis: & QMetaProperty) -> RetType;
}
impl<'a> /*trait*/ QMetaProperty_isValid_0<bool> for () {
  fn isValid_0(self , rsthis: & QMetaProperty) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QMetaProperty7isValidEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmetaobject.h:287
// index:0
// Public inline Visibility=Default Availability=Available
// [8] const QMetaObject * enclosingMetaObject() const

/*

*/
impl /*struct*/ QMetaProperty {
  pub fn enclosingMetaObject_0<RetType, T: QMetaProperty_enclosingMetaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.enclosingMetaObject_0(self);
    // return 1;
  }
}
pub trait QMetaProperty_enclosingMetaObject_0<RetType> {
  fn enclosingMetaObject_0(self , rsthis: & QMetaProperty) -> RetType;
}
impl<'a> /*trait*/ QMetaProperty_enclosingMetaObject_0<usize> for () {
  fn enclosingMetaObject_0(self , rsthis: & QMetaProperty) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QMetaProperty19enclosingMetaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}


pub fn DeleteQMetaProperty(this :*mut QMetaProperty) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN13QMetaPropertyD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
