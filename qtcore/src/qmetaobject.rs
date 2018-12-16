

// mod ::core::QMetaObject
// package qtcore
// /usr/include/qt/QtCore/qobjectdefs.h
// #include <qobjectdefs.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 1
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
#[derive(Default)] // class sizeof(QMetaObject)=48
pub struct QMetaObject {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QMetaObject_ITF interface {
//    QMetaObject_PTR() *QMetaObject
//}
//func (ptr *QMetaObject) QMetaObject_PTR() *QMetaObject { return ptr }

impl /*struct*/ QMetaObject {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QMetaObject {
    return QMetaObject{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QMetaObject {
//  type Target = QMetaObjectBASE;
//
//  fn deref(&self) -> &QMetaObjectBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QMetaObjectBASE> for QMetaObject {
//  fn as_ref(& self) -> & QMetaObjectBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qobjectdefs.h:345
// index:0
// Public Visibility=Default Availability=Available
// [8] const char * className() const

/*

*/
impl /*struct*/ QMetaObject {
  pub fn className_0<RetType, T: QMetaObject_className_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.className_0(self);
    // return 1;
  }
}
pub trait QMetaObject_className_0<RetType> {
  fn className_0(self , rsthis: & QMetaObject) -> RetType;
}
impl<'a> /*trait*/ QMetaObject_className_0<usize> for () {
  fn className_0(self , rsthis: & QMetaObject) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QMetaObject9classNameEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qobjectdefs.h:346
// index:0
// Public Visibility=Default Availability=Available
// [8] const QMetaObject * superClass() const

/*

*/
impl /*struct*/ QMetaObject {
  pub fn superClass_0<RetType, T: QMetaObject_superClass_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.superClass_0(self);
    // return 1;
  }
}
pub trait QMetaObject_superClass_0<RetType> {
  fn superClass_0(self , rsthis: & QMetaObject) -> RetType;
}
impl<'a> /*trait*/ QMetaObject_superClass_0<usize> for () {
  fn superClass_0(self , rsthis: & QMetaObject) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QMetaObject10superClassEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qobjectdefs.h:349
// index:0
// Public Visibility=Default Availability=Available
// [8] QObject * cast(QObject *) const

/*

*/
impl /*struct*/ QMetaObject {
  pub fn cast_0<RetType, T: QMetaObject_cast_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cast_0(self);
    // return 1;
  }
}
pub trait QMetaObject_cast_0<RetType> {
  fn cast_0(self , rsthis: & QMetaObject) -> RetType;
}
impl<'a> /*trait*/ QMetaObject_cast_0<usize> for (usize) {
  fn cast_0(self , rsthis: & QMetaObject) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QMetaObject4castEP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qobjectdefs.h:350
// index:1
// Public Visibility=Default Availability=Available
// [8] const QObject * cast(const QObject *) const

/*

*/
impl /*struct*/ QMetaObject {
  pub fn cast_1<RetType, T: QMetaObject_cast_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cast_1(self);
    // return 1;
  }
}
pub trait QMetaObject_cast_1<RetType> {
  fn cast_1(self , rsthis: & QMetaObject) -> RetType;
}
impl<'a> /*trait*/ QMetaObject_cast_1<usize> for (usize) {
  fn cast_1(self , rsthis: & QMetaObject) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QMetaObject4castEPK7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qobjectdefs.h:356
// index:0
// Public Visibility=Default Availability=Available
// [4] int methodOffset() const

/*

*/
impl /*struct*/ QMetaObject {
  pub fn methodOffset_0<RetType, T: QMetaObject_methodOffset_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.methodOffset_0(self);
    // return 1;
  }
}
pub trait QMetaObject_methodOffset_0<RetType> {
  fn methodOffset_0(self , rsthis: & QMetaObject) -> RetType;
}
impl<'a> /*trait*/ QMetaObject_methodOffset_0<i32> for () {
  fn methodOffset_0(self , rsthis: & QMetaObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QMetaObject12methodOffsetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qobjectdefs.h:357
// index:0
// Public Visibility=Default Availability=Available
// [4] int enumeratorOffset() const

/*

*/
impl /*struct*/ QMetaObject {
  pub fn enumeratorOffset_0<RetType, T: QMetaObject_enumeratorOffset_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.enumeratorOffset_0(self);
    // return 1;
  }
}
pub trait QMetaObject_enumeratorOffset_0<RetType> {
  fn enumeratorOffset_0(self , rsthis: & QMetaObject) -> RetType;
}
impl<'a> /*trait*/ QMetaObject_enumeratorOffset_0<i32> for () {
  fn enumeratorOffset_0(self , rsthis: & QMetaObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QMetaObject16enumeratorOffsetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qobjectdefs.h:358
// index:0
// Public Visibility=Default Availability=Available
// [4] int propertyOffset() const

/*

*/
impl /*struct*/ QMetaObject {
  pub fn propertyOffset_0<RetType, T: QMetaObject_propertyOffset_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.propertyOffset_0(self);
    // return 1;
  }
}
pub trait QMetaObject_propertyOffset_0<RetType> {
  fn propertyOffset_0(self , rsthis: & QMetaObject) -> RetType;
}
impl<'a> /*trait*/ QMetaObject_propertyOffset_0<i32> for () {
  fn propertyOffset_0(self , rsthis: & QMetaObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QMetaObject14propertyOffsetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qobjectdefs.h:359
// index:0
// Public Visibility=Default Availability=Available
// [4] int classInfoOffset() const

/*

*/
impl /*struct*/ QMetaObject {
  pub fn classInfoOffset_0<RetType, T: QMetaObject_classInfoOffset_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.classInfoOffset_0(self);
    // return 1;
  }
}
pub trait QMetaObject_classInfoOffset_0<RetType> {
  fn classInfoOffset_0(self , rsthis: & QMetaObject) -> RetType;
}
impl<'a> /*trait*/ QMetaObject_classInfoOffset_0<i32> for () {
  fn classInfoOffset_0(self , rsthis: & QMetaObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QMetaObject15classInfoOffsetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qobjectdefs.h:361
// index:0
// Public Visibility=Default Availability=Available
// [4] int constructorCount() const

/*

*/
impl /*struct*/ QMetaObject {
  pub fn constructorCount_0<RetType, T: QMetaObject_constructorCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.constructorCount_0(self);
    // return 1;
  }
}
pub trait QMetaObject_constructorCount_0<RetType> {
  fn constructorCount_0(self , rsthis: & QMetaObject) -> RetType;
}
impl<'a> /*trait*/ QMetaObject_constructorCount_0<i32> for () {
  fn constructorCount_0(self , rsthis: & QMetaObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QMetaObject16constructorCountEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qobjectdefs.h:362
// index:0
// Public Visibility=Default Availability=Available
// [4] int methodCount() const

/*

*/
impl /*struct*/ QMetaObject {
  pub fn methodCount_0<RetType, T: QMetaObject_methodCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.methodCount_0(self);
    // return 1;
  }
}
pub trait QMetaObject_methodCount_0<RetType> {
  fn methodCount_0(self , rsthis: & QMetaObject) -> RetType;
}
impl<'a> /*trait*/ QMetaObject_methodCount_0<i32> for () {
  fn methodCount_0(self , rsthis: & QMetaObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QMetaObject11methodCountEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qobjectdefs.h:363
// index:0
// Public Visibility=Default Availability=Available
// [4] int enumeratorCount() const

/*

*/
impl /*struct*/ QMetaObject {
  pub fn enumeratorCount_0<RetType, T: QMetaObject_enumeratorCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.enumeratorCount_0(self);
    // return 1;
  }
}
pub trait QMetaObject_enumeratorCount_0<RetType> {
  fn enumeratorCount_0(self , rsthis: & QMetaObject) -> RetType;
}
impl<'a> /*trait*/ QMetaObject_enumeratorCount_0<i32> for () {
  fn enumeratorCount_0(self , rsthis: & QMetaObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QMetaObject15enumeratorCountEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qobjectdefs.h:364
// index:0
// Public Visibility=Default Availability=Available
// [4] int propertyCount() const

/*

*/
impl /*struct*/ QMetaObject {
  pub fn propertyCount_0<RetType, T: QMetaObject_propertyCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.propertyCount_0(self);
    // return 1;
  }
}
pub trait QMetaObject_propertyCount_0<RetType> {
  fn propertyCount_0(self , rsthis: & QMetaObject) -> RetType;
}
impl<'a> /*trait*/ QMetaObject_propertyCount_0<i32> for () {
  fn propertyCount_0(self , rsthis: & QMetaObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QMetaObject13propertyCountEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qobjectdefs.h:365
// index:0
// Public Visibility=Default Availability=Available
// [4] int classInfoCount() const

/*

*/
impl /*struct*/ QMetaObject {
  pub fn classInfoCount_0<RetType, T: QMetaObject_classInfoCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.classInfoCount_0(self);
    // return 1;
  }
}
pub trait QMetaObject_classInfoCount_0<RetType> {
  fn classInfoCount_0(self , rsthis: & QMetaObject) -> RetType;
}
impl<'a> /*trait*/ QMetaObject_classInfoCount_0<i32> for () {
  fn classInfoCount_0(self , rsthis: & QMetaObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QMetaObject14classInfoCountEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qobjectdefs.h:367
// index:0
// Public Visibility=Default Availability=Available
// [4] int indexOfConstructor(const char *) const

/*

*/
impl /*struct*/ QMetaObject {
  pub fn indexOfConstructor_0<RetType, T: QMetaObject_indexOfConstructor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.indexOfConstructor_0(self);
    // return 1;
  }
}
pub trait QMetaObject_indexOfConstructor_0<RetType> {
  fn indexOfConstructor_0(self , rsthis: & QMetaObject) -> RetType;
}
impl<'a> /*trait*/ QMetaObject_indexOfConstructor_0<i32> for (usize) {
  fn indexOfConstructor_0(self , rsthis: & QMetaObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QMetaObject18indexOfConstructorEPKc", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qobjectdefs.h:368
// index:0
// Public Visibility=Default Availability=Available
// [4] int indexOfMethod(const char *) const

/*

*/
impl /*struct*/ QMetaObject {
  pub fn indexOfMethod_0<RetType, T: QMetaObject_indexOfMethod_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.indexOfMethod_0(self);
    // return 1;
  }
}
pub trait QMetaObject_indexOfMethod_0<RetType> {
  fn indexOfMethod_0(self , rsthis: & QMetaObject) -> RetType;
}
impl<'a> /*trait*/ QMetaObject_indexOfMethod_0<i32> for (usize) {
  fn indexOfMethod_0(self , rsthis: & QMetaObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QMetaObject13indexOfMethodEPKc", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qobjectdefs.h:369
// index:0
// Public Visibility=Default Availability=Available
// [4] int indexOfSignal(const char *) const

/*

*/
impl /*struct*/ QMetaObject {
  pub fn indexOfSignal_0<RetType, T: QMetaObject_indexOfSignal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.indexOfSignal_0(self);
    // return 1;
  }
}
pub trait QMetaObject_indexOfSignal_0<RetType> {
  fn indexOfSignal_0(self , rsthis: & QMetaObject) -> RetType;
}
impl<'a> /*trait*/ QMetaObject_indexOfSignal_0<i32> for (usize) {
  fn indexOfSignal_0(self , rsthis: & QMetaObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QMetaObject13indexOfSignalEPKc", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qobjectdefs.h:370
// index:0
// Public Visibility=Default Availability=Available
// [4] int indexOfSlot(const char *) const

/*

*/
impl /*struct*/ QMetaObject {
  pub fn indexOfSlot_0<RetType, T: QMetaObject_indexOfSlot_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.indexOfSlot_0(self);
    // return 1;
  }
}
pub trait QMetaObject_indexOfSlot_0<RetType> {
  fn indexOfSlot_0(self , rsthis: & QMetaObject) -> RetType;
}
impl<'a> /*trait*/ QMetaObject_indexOfSlot_0<i32> for (usize) {
  fn indexOfSlot_0(self , rsthis: & QMetaObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QMetaObject11indexOfSlotEPKc", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qobjectdefs.h:371
// index:0
// Public Visibility=Default Availability=Available
// [4] int indexOfEnumerator(const char *) const

/*

*/
impl /*struct*/ QMetaObject {
  pub fn indexOfEnumerator_0<RetType, T: QMetaObject_indexOfEnumerator_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.indexOfEnumerator_0(self);
    // return 1;
  }
}
pub trait QMetaObject_indexOfEnumerator_0<RetType> {
  fn indexOfEnumerator_0(self , rsthis: & QMetaObject) -> RetType;
}
impl<'a> /*trait*/ QMetaObject_indexOfEnumerator_0<i32> for (usize) {
  fn indexOfEnumerator_0(self , rsthis: & QMetaObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QMetaObject17indexOfEnumeratorEPKc", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qobjectdefs.h:372
// index:0
// Public Visibility=Default Availability=Available
// [4] int indexOfProperty(const char *) const

/*

*/
impl /*struct*/ QMetaObject {
  pub fn indexOfProperty_0<RetType, T: QMetaObject_indexOfProperty_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.indexOfProperty_0(self);
    // return 1;
  }
}
pub trait QMetaObject_indexOfProperty_0<RetType> {
  fn indexOfProperty_0(self , rsthis: & QMetaObject) -> RetType;
}
impl<'a> /*trait*/ QMetaObject_indexOfProperty_0<i32> for (usize) {
  fn indexOfProperty_0(self , rsthis: & QMetaObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QMetaObject15indexOfPropertyEPKc", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qobjectdefs.h:373
// index:0
// Public Visibility=Default Availability=Available
// [4] int indexOfClassInfo(const char *) const

/*

*/
impl /*struct*/ QMetaObject {
  pub fn indexOfClassInfo_0<RetType, T: QMetaObject_indexOfClassInfo_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.indexOfClassInfo_0(self);
    // return 1;
  }
}
pub trait QMetaObject_indexOfClassInfo_0<RetType> {
  fn indexOfClassInfo_0(self , rsthis: & QMetaObject) -> RetType;
}
impl<'a> /*trait*/ QMetaObject_indexOfClassInfo_0<i32> for (usize) {
  fn indexOfClassInfo_0(self , rsthis: & QMetaObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QMetaObject16indexOfClassInfoEPKc", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qobjectdefs.h:375
// index:0
// Public Visibility=Default Availability=Available
// [16] QMetaMethod constructor(int) const

/*

*/
impl /*struct*/ QMetaObject {
  pub fn constructor_0<RetType, T: QMetaObject_constructor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.constructor_0(self);
    // return 1;
  }
}
pub trait QMetaObject_constructor_0<RetType> {
  fn constructor_0(self , rsthis: & QMetaObject) -> RetType;
}
impl<'a> /*trait*/ QMetaObject_constructor_0<usize> for (i32) {
  fn constructor_0(self , rsthis: & QMetaObject) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QMetaObject11constructorEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qobjectdefs.h:376
// index:0
// Public Visibility=Default Availability=Available
// [16] QMetaMethod method(int) const

/*

*/
impl /*struct*/ QMetaObject {
  pub fn method_0<RetType, T: QMetaObject_method_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.method_0(self);
    // return 1;
  }
}
pub trait QMetaObject_method_0<RetType> {
  fn method_0(self , rsthis: & QMetaObject) -> RetType;
}
impl<'a> /*trait*/ QMetaObject_method_0<usize> for (i32) {
  fn method_0(self , rsthis: & QMetaObject) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QMetaObject6methodEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qobjectdefs.h:377
// index:0
// Public Visibility=Default Availability=Available
// [16] QMetaEnum enumerator(int) const

/*

*/
impl /*struct*/ QMetaObject {
  pub fn enumerator_0<RetType, T: QMetaObject_enumerator_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.enumerator_0(self);
    // return 1;
  }
}
pub trait QMetaObject_enumerator_0<RetType> {
  fn enumerator_0(self , rsthis: & QMetaObject) -> RetType;
}
impl<'a> /*trait*/ QMetaObject_enumerator_0<usize> for (i32) {
  fn enumerator_0(self , rsthis: & QMetaObject) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QMetaObject10enumeratorEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qobjectdefs.h:378
// index:0
// Public Visibility=Default Availability=Available
// [32] QMetaProperty property(int) const

/*

*/
impl /*struct*/ QMetaObject {
  pub fn property_0<RetType, T: QMetaObject_property_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.property_0(self);
    // return 1;
  }
}
pub trait QMetaObject_property_0<RetType> {
  fn property_0(self , rsthis: & QMetaObject) -> RetType;
}
impl<'a> /*trait*/ QMetaObject_property_0<usize> for (i32) {
  fn property_0(self , rsthis: & QMetaObject) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QMetaObject8propertyEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qobjectdefs.h:379
// index:0
// Public Visibility=Default Availability=Available
// [16] QMetaClassInfo classInfo(int) const

/*

*/
impl /*struct*/ QMetaObject {
  pub fn classInfo_0<RetType, T: QMetaObject_classInfo_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.classInfo_0(self);
    // return 1;
  }
}
pub trait QMetaObject_classInfo_0<RetType> {
  fn classInfo_0(self , rsthis: & QMetaObject) -> RetType;
}
impl<'a> /*trait*/ QMetaObject_classInfo_0<usize> for (i32) {
  fn classInfo_0(self , rsthis: & QMetaObject) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QMetaObject9classInfoEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qobjectdefs.h:380
// index:0
// Public Visibility=Default Availability=Available
// [32] QMetaProperty userProperty() const

/*

*/
impl /*struct*/ QMetaObject {
  pub fn userProperty_0<RetType, T: QMetaObject_userProperty_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.userProperty_0(self);
    // return 1;
  }
}
pub trait QMetaObject_userProperty_0<RetType> {
  fn userProperty_0(self , rsthis: & QMetaObject) -> RetType;
}
impl<'a> /*trait*/ QMetaObject_userProperty_0<usize> for () {
  fn userProperty_0(self , rsthis: & QMetaObject) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QMetaObject12userPropertyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qobjectdefs.h:382
// index:0
// Public static Visibility=Default Availability=Available
// [1] bool checkConnectArgs(const char *, const char *)

/*

*/
impl /*struct*/ QMetaObject {
  pub fn checkConnectArgs_0<RetType, T: QMetaObject_checkConnectArgs_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.checkConnectArgs_0();
    // return 1;
  }
}
pub trait QMetaObject_checkConnectArgs_0<RetType> {
  fn checkConnectArgs_0(self ) -> RetType;
}
impl<'a> /*trait*/ QMetaObject_checkConnectArgs_0<bool> for (usize,usize) {
  fn checkConnectArgs_0(self ) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self.0) as *const usize as usize;
    let arg1 = (self.1) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QMetaObject16checkConnectArgsEPKcS1_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qobjectdefs.h:383
// index:1
// Public static Visibility=Default Availability=Available
// [1] bool checkConnectArgs(const QMetaMethod &, const QMetaMethod &)

/*

*/
impl /*struct*/ QMetaObject {
  pub fn checkConnectArgs_1<RetType, T: QMetaObject_checkConnectArgs_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.checkConnectArgs_1();
    // return 1;
  }
}
pub trait QMetaObject_checkConnectArgs_1<RetType> {
  fn checkConnectArgs_1(self ) -> RetType;
}
impl<'a> /*trait*/ QMetaObject_checkConnectArgs_1<bool> for (usize,usize) {
  fn checkConnectArgs_1(self ) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QMetaObject16checkConnectArgsERK11QMetaMethodS2_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qobjectdefs.h:385
// index:0
// Public static Visibility=Default Availability=Available
// [8] QByteArray normalizedSignature(const char *)

/*

*/
impl /*struct*/ QMetaObject {
  pub fn normalizedSignature_0<RetType, T: QMetaObject_normalizedSignature_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.normalizedSignature_0();
    // return 1;
  }
}
pub trait QMetaObject_normalizedSignature_0<RetType> {
  fn normalizedSignature_0(self ) -> RetType;
}
impl<'a> /*trait*/ QMetaObject_normalizedSignature_0<usize> for (usize) {
  fn normalizedSignature_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QMetaObject19normalizedSignatureEPKc", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qobjectdefs.h:386
// index:0
// Public static Visibility=Default Availability=Available
// [8] QByteArray normalizedType(const char *)

/*

*/
impl /*struct*/ QMetaObject {
  pub fn normalizedType_0<RetType, T: QMetaObject_normalizedType_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.normalizedType_0();
    // return 1;
  }
}
pub trait QMetaObject_normalizedType_0<RetType> {
  fn normalizedType_0(self ) -> RetType;
}
impl<'a> /*trait*/ QMetaObject_normalizedType_0<usize> for (usize) {
  fn normalizedType_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QMetaObject14normalizedTypeEPKc", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qobjectdefs.h:389
// index:0
// Public static Visibility=Default Availability=Available
// [8] QMetaObject::Connection connect(const QObject *, int, const QObject *, int, int, int *)

/*

*/
impl /*struct*/ QMetaObject {
  pub fn connect_0<RetType, T: QMetaObject_connect_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.connect_0();
    // return 1;
  }
}
pub trait QMetaObject_connect_0<RetType> {
  fn connect_0(self ) -> RetType;
}
impl<'a> /*trait*/ QMetaObject_connect_0<usize> for (usize,i32,usize,i32,i32,usize) {
  fn connect_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let arg4 = (&self.4) as *const i32 as usize;
    let arg5 = (&self.5) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QMetaObject7connectEPK7QObjectiS2_iiPi", 6,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,arg5,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qobjectdefs.h:393
// index:0
// Public static Visibility=Default Availability=Available
// [1] bool disconnect(const QObject *, int, const QObject *, int)

/*

*/
impl /*struct*/ QMetaObject {
  pub fn disconnect_0<RetType, T: QMetaObject_disconnect_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.disconnect_0();
    // return 1;
  }
}
pub trait QMetaObject_disconnect_0<RetType> {
  fn disconnect_0(self ) -> RetType;
}
impl<'a> /*trait*/ QMetaObject_disconnect_0<bool> for (usize,i32,usize,i32) {
  fn disconnect_0(self ) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QMetaObject10disconnectEPK7QObjectiS2_i", 4,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qobjectdefs.h:395
// index:0
// Public static Visibility=Default Availability=Available
// [1] bool disconnectOne(const QObject *, int, const QObject *, int)

/*

*/
impl /*struct*/ QMetaObject {
  pub fn disconnectOne_0<RetType, T: QMetaObject_disconnectOne_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.disconnectOne_0();
    // return 1;
  }
}
pub trait QMetaObject_disconnectOne_0<RetType> {
  fn disconnectOne_0(self ) -> RetType;
}
impl<'a> /*trait*/ QMetaObject_disconnectOne_0<bool> for (usize,i32,usize,i32) {
  fn disconnectOne_0(self ) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QMetaObject13disconnectOneEPK7QObjectiS2_i", 4,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qobjectdefs.h:398
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void connectSlotsByName(QObject *)

/*

*/
impl /*struct*/ QMetaObject {
  pub fn connectSlotsByName_0<RetType, T: QMetaObject_connectSlotsByName_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.connectSlotsByName_0();
    // return 1;
  }
}
pub trait QMetaObject_connectSlotsByName_0<RetType> {
  fn connectSlotsByName_0(self ) -> RetType;
}
impl<'a> /*trait*/ QMetaObject_connectSlotsByName_0<(/*void*/)> for (usize) {
  fn connectSlotsByName_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QMetaObject18connectSlotsByNameEP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qobjectdefs.h:401
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void activate(QObject *, int, void **)

/*

*/
impl /*struct*/ QMetaObject {
  pub fn activate_0<RetType, T: QMetaObject_activate_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.activate_0();
    // return 1;
  }
}
pub trait QMetaObject_activate_0<RetType> {
  fn activate_0(self ) -> RetType;
}
impl<'a> /*trait*/ QMetaObject_activate_0<(/*void*/)> for (usize,i32,usize) {
  fn activate_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QMetaObject8activateEP7QObjectiPPv", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qobjectdefs.h:403
// index:1
// Public static Visibility=Default Availability=Available
// [-2] void activate(QObject *, int, int, void **)

/*

*/
impl /*struct*/ QMetaObject {
  pub fn activate_1<RetType, T: QMetaObject_activate_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.activate_1();
    // return 1;
  }
}
pub trait QMetaObject_activate_1<RetType> {
  fn activate_1(self ) -> RetType;
}
impl<'a> /*trait*/ QMetaObject_activate_1<(/*void*/)> for (usize,i32,i32,usize) {
  fn activate_1(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QMetaObject8activateEP7QObjectiiPPv", 4,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qobjectdefs.h:405
// index:0
// Public static Visibility=Default Availability=Available
// [1] bool invokeMethod(QObject *, const char *, Qt::ConnectionType, QGenericReturnArgument, QGenericArgument, QGenericArgument, QGenericArgument, QGenericArgument, QGenericArgument, QGenericArgument, QGenericArgument, QGenericArgument, QGenericArgument, QGenericArgument)

/*

*/
impl /*struct*/ QMetaObject {
  pub fn invokeMethod_0<RetType, T: QMetaObject_invokeMethod_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.invokeMethod_0();
    // return 1;
  }
}
pub trait QMetaObject_invokeMethod_0<RetType> {
  fn invokeMethod_0(self ) -> RetType;
}
impl<'a> /*trait*/ QMetaObject_invokeMethod_0<bool> for (usize,usize,i32,usize,usize,usize,usize,usize,usize,usize,usize,usize,usize,usize) {
  fn invokeMethod_0(self ) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (self.1) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
    let arg4 = (&self.4/*.qclsinst*/) as *const usize as usize;
    let arg5 = (&self.5/*.qclsinst*/) as *const usize as usize;
    let arg6 = (&self.6/*.qclsinst*/) as *const usize as usize;
    let arg7 = (&self.7/*.qclsinst*/) as *const usize as usize;
    let arg8 = (&self.8/*.qclsinst*/) as *const usize as usize;
    let arg9 = (&self.9/*.qclsinst*/) as *const usize as usize;
    let arg10 = (&self.10/*.qclsinst*/) as *const usize as usize;
    let arg11 = (&self.11/*.qclsinst*/) as *const usize as usize;
    let arg12 = (&self.12/*.qclsinst*/) as *const usize as usize;
    let arg13 = (&self.13/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QMetaObject12invokeMethodEP7QObjectPKcN2Qt14ConnectionTypeE22QGenericReturnArgument16QGenericArgumentS7_S7_S7_S7_S7_S7_S7_S7_S7_", 14,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,arg0,arg1,arg2,arg3,arg4,arg5,arg6,arg7,arg8,arg9,arg10,arg11,arg12,arg13,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qobjectdefs.h:419
// index:1
// Public static inline Visibility=Default Availability=Available
// [1] bool invokeMethod(QObject *, const char *, QGenericReturnArgument, QGenericArgument, QGenericArgument, QGenericArgument, QGenericArgument, QGenericArgument, QGenericArgument, QGenericArgument, QGenericArgument, QGenericArgument, QGenericArgument)

/*

*/
impl /*struct*/ QMetaObject {
  pub fn invokeMethod_1<RetType, T: QMetaObject_invokeMethod_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.invokeMethod_1();
    // return 1;
  }
}
pub trait QMetaObject_invokeMethod_1<RetType> {
  fn invokeMethod_1(self ) -> RetType;
}
impl<'a> /*trait*/ QMetaObject_invokeMethod_1<bool> for (usize,usize,usize,usize,usize,usize,usize,usize,usize,usize,usize,usize,usize) {
  fn invokeMethod_1(self ) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (self.1) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
    let arg4 = (&self.4/*.qclsinst*/) as *const usize as usize;
    let arg5 = (&self.5/*.qclsinst*/) as *const usize as usize;
    let arg6 = (&self.6/*.qclsinst*/) as *const usize as usize;
    let arg7 = (&self.7/*.qclsinst*/) as *const usize as usize;
    let arg8 = (&self.8/*.qclsinst*/) as *const usize as usize;
    let arg9 = (&self.9/*.qclsinst*/) as *const usize as usize;
    let arg10 = (&self.10/*.qclsinst*/) as *const usize as usize;
    let arg11 = (&self.11/*.qclsinst*/) as *const usize as usize;
    let arg12 = (&self.12/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QMetaObject12invokeMethodEP7QObjectPKc22QGenericReturnArgument16QGenericArgumentS5_S5_S5_S5_S5_S5_S5_S5_S5_", 13,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,arg0,arg1,arg2,arg3,arg4,arg5,arg6,arg7,arg8,arg9,arg10,arg11,arg12,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qobjectdefs.h:436
// index:2
// Public static inline Visibility=Default Availability=Available
// [1] bool invokeMethod(QObject *, const char *, Qt::ConnectionType, QGenericArgument, QGenericArgument, QGenericArgument, QGenericArgument, QGenericArgument, QGenericArgument, QGenericArgument, QGenericArgument, QGenericArgument, QGenericArgument)

/*

*/
impl /*struct*/ QMetaObject {
  pub fn invokeMethod_2<RetType, T: QMetaObject_invokeMethod_2<RetType>>( overload_args: T) -> RetType {
    return overload_args.invokeMethod_2();
    // return 1;
  }
}
pub trait QMetaObject_invokeMethod_2<RetType> {
  fn invokeMethod_2(self ) -> RetType;
}
impl<'a> /*trait*/ QMetaObject_invokeMethod_2<bool> for (usize,usize,i32,usize,usize,usize,usize,usize,usize,usize,usize,usize,usize) {
  fn invokeMethod_2(self ) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (self.1) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
    let arg4 = (&self.4/*.qclsinst*/) as *const usize as usize;
    let arg5 = (&self.5/*.qclsinst*/) as *const usize as usize;
    let arg6 = (&self.6/*.qclsinst*/) as *const usize as usize;
    let arg7 = (&self.7/*.qclsinst*/) as *const usize as usize;
    let arg8 = (&self.8/*.qclsinst*/) as *const usize as usize;
    let arg9 = (&self.9/*.qclsinst*/) as *const usize as usize;
    let arg10 = (&self.10/*.qclsinst*/) as *const usize as usize;
    let arg11 = (&self.11/*.qclsinst*/) as *const usize as usize;
    let arg12 = (&self.12/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QMetaObject12invokeMethodEP7QObjectPKcN2Qt14ConnectionTypeE16QGenericArgumentS6_S6_S6_S6_S6_S6_S6_S6_S6_", 13,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,arg0,arg1,arg2,arg3,arg4,arg5,arg6,arg7,arg8,arg9,arg10,arg11,arg12,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qobjectdefs.h:453
// index:3
// Public static inline Visibility=Default Availability=Available
// [1] bool invokeMethod(QObject *, const char *, QGenericArgument, QGenericArgument, QGenericArgument, QGenericArgument, QGenericArgument, QGenericArgument, QGenericArgument, QGenericArgument, QGenericArgument, QGenericArgument)

/*

*/
impl /*struct*/ QMetaObject {
  pub fn invokeMethod_3<RetType, T: QMetaObject_invokeMethod_3<RetType>>( overload_args: T) -> RetType {
    return overload_args.invokeMethod_3();
    // return 1;
  }
}
pub trait QMetaObject_invokeMethod_3<RetType> {
  fn invokeMethod_3(self ) -> RetType;
}
impl<'a> /*trait*/ QMetaObject_invokeMethod_3<bool> for (usize,usize,usize,usize,usize,usize,usize,usize,usize,usize,usize,usize) {
  fn invokeMethod_3(self ) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (self.1) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
    let arg4 = (&self.4/*.qclsinst*/) as *const usize as usize;
    let arg5 = (&self.5/*.qclsinst*/) as *const usize as usize;
    let arg6 = (&self.6/*.qclsinst*/) as *const usize as usize;
    let arg7 = (&self.7/*.qclsinst*/) as *const usize as usize;
    let arg8 = (&self.8/*.qclsinst*/) as *const usize as usize;
    let arg9 = (&self.9/*.qclsinst*/) as *const usize as usize;
    let arg10 = (&self.10/*.qclsinst*/) as *const usize as usize;
    let arg11 = (&self.11/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QMetaObject12invokeMethodEP7QObjectPKc16QGenericArgumentS4_S4_S4_S4_S4_S4_S4_S4_S4_", 12,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,arg0,arg1,arg2,arg3,arg4,arg5,arg6,arg7,arg8,arg9,arg10,arg11,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qobjectdefs.h:554
// index:0
// Public Visibility=Default Availability=Available
// [8] QObject * newInstance(QGenericArgument, QGenericArgument, QGenericArgument, QGenericArgument, QGenericArgument, QGenericArgument, QGenericArgument, QGenericArgument, QGenericArgument, QGenericArgument) const

/*

*/
impl /*struct*/ QMetaObject {
  pub fn newInstance_0<RetType, T: QMetaObject_newInstance_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.newInstance_0(self);
    // return 1;
  }
}
pub trait QMetaObject_newInstance_0<RetType> {
  fn newInstance_0(self , rsthis: & QMetaObject) -> RetType;
}
impl<'a> /*trait*/ QMetaObject_newInstance_0<usize> for (usize,usize,usize,usize,usize,usize,usize,usize,usize,usize) {
  fn newInstance_0(self , rsthis: & QMetaObject) -> usize {
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
    let arg9 = (&self.9/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QMetaObject11newInstanceE16QGenericArgumentS0_S0_S0_S0_S0_S0_S0_S0_S0_", 10,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,arg5,arg6,arg7,arg8,arg9,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qobjectdefs.h:581
// index:0
// Public Visibility=Default Availability=Available
// [4] int static_metacall(QMetaObject::Call, int, void **) const

/*

*/
impl /*struct*/ QMetaObject {
  pub fn static_metacall_0<RetType, T: QMetaObject_static_metacall_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.static_metacall_0(self);
    // return 1;
  }
}
pub trait QMetaObject_static_metacall_0<RetType> {
  fn static_metacall_0(self , rsthis: & QMetaObject) -> RetType;
}
impl<'a> /*trait*/ QMetaObject_static_metacall_0<i32> for (i32,i32,usize) {
  fn static_metacall_0(self , rsthis: & QMetaObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QMetaObject15static_metacallENS_4CallEiPPv", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qobjectdefs.h:582
// index:0
// Public static Visibility=Default Availability=Available
// [4] int metacall(QObject *, QMetaObject::Call, int, void **)

/*

*/
impl /*struct*/ QMetaObject {
  pub fn metacall_0<RetType, T: QMetaObject_metacall_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.metacall_0();
    // return 1;
  }
}
pub trait QMetaObject_metacall_0<RetType> {
  fn metacall_0(self ) -> RetType;
}
impl<'a> /*trait*/ QMetaObject_metacall_0<i32> for (usize,i32,i32,usize) {
  fn metacall_0(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QMetaObject8metacallEP7QObjectNS_4CallEiPPv", 4,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}


pub fn DeleteQMetaObject(this :*mut QMetaObject) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN11QMetaObjectD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}

/*


*/
pub type QMetaObject__Call = i32;
// 
pub const QMetaObject__InvokeMetaMethod :QMetaObject__Call = 0;
// 
pub const QMetaObject__ReadProperty :QMetaObject__Call = 1;
// 
pub const QMetaObject__WriteProperty :QMetaObject__Call = 2;
// 
pub const QMetaObject__ResetProperty :QMetaObject__Call = 3;
// 
pub const QMetaObject__QueryPropertyDesignable :QMetaObject__Call = 4;
// 
pub const QMetaObject__QueryPropertyScriptable :QMetaObject__Call = 5;
// 
pub const QMetaObject__QueryPropertyStored :QMetaObject__Call = 6;
// 
pub const QMetaObject__QueryPropertyEditable :QMetaObject__Call = 7;
// 
pub const QMetaObject__QueryPropertyUser :QMetaObject__Call = 8;
// 
pub const QMetaObject__CreateInstance :QMetaObject__Call = 9;
// 
pub const QMetaObject__IndexOfMethod :QMetaObject__Call = 10;
// 
pub const QMetaObject__RegisterPropertyMetaType :QMetaObject__Call = 11;
// 
pub const QMetaObject__RegisterMethodArgumentMetaType :QMetaObject__Call = 12;
pub fn QMetaObject_CallItemName(val: i32) ->String {
  match val {
     QMetaObject__InvokeMetaMethod => // 0
     {return String::from("InvokeMetaMethod");}
     QMetaObject__ReadProperty => // 1
     {return String::from("ReadProperty");}
     QMetaObject__WriteProperty => // 2
     {return String::from("WriteProperty");}
     QMetaObject__ResetProperty => // 3
     {return String::from("ResetProperty");}
     QMetaObject__QueryPropertyDesignable => // 4
     {return String::from("QueryPropertyDesignable");}
     QMetaObject__QueryPropertyScriptable => // 5
     {return String::from("QueryPropertyScriptable");}
     QMetaObject__QueryPropertyStored => // 6
     {return String::from("QueryPropertyStored");}
     QMetaObject__QueryPropertyEditable => // 7
     {return String::from("QueryPropertyEditable");}
     QMetaObject__QueryPropertyUser => // 8
     {return String::from("QueryPropertyUser");}
     QMetaObject__CreateInstance => // 9
     {return String::from("CreateInstance");}
     QMetaObject__IndexOfMethod => // 10
     {return String::from("IndexOfMethod");}
     QMetaObject__RegisterPropertyMetaType => // 11
     {return String::from("RegisterPropertyMetaType");}
     QMetaObject__RegisterMethodArgumentMetaType => // 12
     {return String::from("RegisterMethodArgumentMetaType");}
  _ => {return format!("{}", val);}
}
}
pub fn QMetaObject_CallItemName_s(val: i32) ->String {
  //var nilthis *QMetaObject
  //return nilthis.CallItemName(val);
  return QMetaObject_CallItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
