

// mod ::core::QMetaMethod
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
// extern C begin: 9
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
#[derive(Default)] // class sizeof(QMetaMethod)=16
pub struct QMetaMethod {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QMetaMethod_ITF interface {
//    QMetaMethod_PTR() *QMetaMethod
//}
//func (ptr *QMetaMethod) QMetaMethod_PTR() *QMetaMethod { return ptr }

impl /*struct*/ QMetaMethod {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QMetaMethod {
    return QMetaMethod{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QMetaMethod {
//  type Target = QMetaMethodBASE;
//
//  fn deref(&self) -> &QMetaMethodBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QMetaMethodBASE> for QMetaMethod {
//  fn as_ref(& self) -> & QMetaMethodBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qmetaobject.h:57
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void QMetaMethod()

/*

*/
// QMetaMethod() ctx.fn_proto_cpp
impl /*struct*/ QMetaMethod {
  pub fn QMetaMethod_0<T: QMetaMethod_QMetaMethod_0>(value: T) -> QMetaMethod {
    let rsthis = value.QMetaMethod_0();
    return rsthis;
    // return 1;
  }
}

pub trait QMetaMethod_QMetaMethod_0 {
  fn QMetaMethod_0(self) -> QMetaMethod;
}
// QMetaMethod() ctx.fn_proto_cpp
impl<'a> /*trait*/ QMetaMethod_QMetaMethod_0 for () {
  fn QMetaMethod_0(self) -> QMetaMethod {
    // unsafe{_ZN11QMetaMethodC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QMetaMethodC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QMetaMethod{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qmetaobject.h:59
// index:0
// Public Visibility=Default Availability=Available
// [8] QByteArray methodSignature() const

/*

*/
impl /*struct*/ QMetaMethod {
  pub fn methodSignature_0<RetType, T: QMetaMethod_methodSignature_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.methodSignature_0(self);
    // return 1;
  }
}
pub trait QMetaMethod_methodSignature_0<RetType> {
  fn methodSignature_0(self , rsthis: & QMetaMethod) -> RetType;
}
impl<'a> /*trait*/ QMetaMethod_methodSignature_0<usize> for () {
  fn methodSignature_0(self , rsthis: & QMetaMethod) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QMetaMethod15methodSignatureEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmetaobject.h:60
// index:0
// Public Visibility=Default Availability=Available
// [8] QByteArray name() const

/*

*/
impl /*struct*/ QMetaMethod {
  pub fn name_0<RetType, T: QMetaMethod_name_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.name_0(self);
    // return 1;
  }
}
pub trait QMetaMethod_name_0<RetType> {
  fn name_0(self , rsthis: & QMetaMethod) -> RetType;
}
impl<'a> /*trait*/ QMetaMethod_name_0<usize> for () {
  fn name_0(self , rsthis: & QMetaMethod) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QMetaMethod4nameEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmetaobject.h:61
// index:0
// Public Visibility=Default Availability=Available
// [8] const char * typeName() const

/*

*/
impl /*struct*/ QMetaMethod {
  pub fn typeName_0<RetType, T: QMetaMethod_typeName_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.typeName_0(self);
    // return 1;
  }
}
pub trait QMetaMethod_typeName_0<RetType> {
  fn typeName_0(self , rsthis: & QMetaMethod) -> RetType;
}
impl<'a> /*trait*/ QMetaMethod_typeName_0<usize> for () {
  fn typeName_0(self , rsthis: & QMetaMethod) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QMetaMethod8typeNameEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmetaobject.h:62
// index:0
// Public Visibility=Default Availability=Available
// [4] int returnType() const

/*

*/
impl /*struct*/ QMetaMethod {
  pub fn returnType_0<RetType, T: QMetaMethod_returnType_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.returnType_0(self);
    // return 1;
  }
}
pub trait QMetaMethod_returnType_0<RetType> {
  fn returnType_0(self , rsthis: & QMetaMethod) -> RetType;
}
impl<'a> /*trait*/ QMetaMethod_returnType_0<i32> for () {
  fn returnType_0(self , rsthis: & QMetaMethod) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QMetaMethod10returnTypeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmetaobject.h:63
// index:0
// Public Visibility=Default Availability=Available
// [4] int parameterCount() const

/*

*/
impl /*struct*/ QMetaMethod {
  pub fn parameterCount_0<RetType, T: QMetaMethod_parameterCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.parameterCount_0(self);
    // return 1;
  }
}
pub trait QMetaMethod_parameterCount_0<RetType> {
  fn parameterCount_0(self , rsthis: & QMetaMethod) -> RetType;
}
impl<'a> /*trait*/ QMetaMethod_parameterCount_0<i32> for () {
  fn parameterCount_0(self , rsthis: & QMetaMethod) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QMetaMethod14parameterCountEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmetaobject.h:64
// index:0
// Public Visibility=Default Availability=Available
// [4] int parameterType(int) const

/*

*/
impl /*struct*/ QMetaMethod {
  pub fn parameterType_0<RetType, T: QMetaMethod_parameterType_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.parameterType_0(self);
    // return 1;
  }
}
pub trait QMetaMethod_parameterType_0<RetType> {
  fn parameterType_0(self , rsthis: & QMetaMethod) -> RetType;
}
impl<'a> /*trait*/ QMetaMethod_parameterType_0<i32> for (i32) {
  fn parameterType_0(self , rsthis: & QMetaMethod) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QMetaMethod13parameterTypeEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmetaobject.h:65
// index:0
// Public Visibility=Default Availability=Available
// [-2] void getParameterTypes(int *) const

/*

*/
impl /*struct*/ QMetaMethod {
  pub fn getParameterTypes_0<RetType, T: QMetaMethod_getParameterTypes_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.getParameterTypes_0(self);
    // return 1;
  }
}
pub trait QMetaMethod_getParameterTypes_0<RetType> {
  fn getParameterTypes_0(self , rsthis: & QMetaMethod) -> RetType;
}
impl<'a> /*trait*/ QMetaMethod_getParameterTypes_0<(/*void*/)> for (usize) {
  fn getParameterTypes_0(self , rsthis: & QMetaMethod) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK11QMetaMethod17getParameterTypesEPi", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qmetaobject.h:68
// index:0
// Public Visibility=Default Availability=Available
// [8] const char * tag() const

/*

*/
impl /*struct*/ QMetaMethod {
  pub fn tag_0<RetType, T: QMetaMethod_tag_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.tag_0(self);
    // return 1;
  }
}
pub trait QMetaMethod_tag_0<RetType> {
  fn tag_0(self , rsthis: & QMetaMethod) -> RetType;
}
impl<'a> /*trait*/ QMetaMethod_tag_0<usize> for () {
  fn tag_0(self , rsthis: & QMetaMethod) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QMetaMethod3tagEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmetaobject.h:70
// index:0
// Public Visibility=Default Availability=Available
// [4] QMetaMethod::Access access() const

/*

*/
impl /*struct*/ QMetaMethod {
  pub fn access_0<RetType, T: QMetaMethod_access_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.access_0(self);
    // return 1;
  }
}
pub trait QMetaMethod_access_0<RetType> {
  fn access_0(self , rsthis: & QMetaMethod) -> RetType;
}
impl<'a> /*trait*/ QMetaMethod_access_0<i32> for () {
  fn access_0(self , rsthis: & QMetaMethod) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QMetaMethod6accessEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmetaobject.h:72
// index:0
// Public Visibility=Default Availability=Available
// [4] QMetaMethod::MethodType methodType() const

/*

*/
impl /*struct*/ QMetaMethod {
  pub fn methodType_0<RetType, T: QMetaMethod_methodType_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.methodType_0(self);
    // return 1;
  }
}
pub trait QMetaMethod_methodType_0<RetType> {
  fn methodType_0(self , rsthis: & QMetaMethod) -> RetType;
}
impl<'a> /*trait*/ QMetaMethod_methodType_0<i32> for () {
  fn methodType_0(self , rsthis: & QMetaMethod) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QMetaMethod10methodTypeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmetaobject.h:74
// index:0
// Public Visibility=Default Availability=Available
// [4] int attributes() const

/*

*/
impl /*struct*/ QMetaMethod {
  pub fn attributes_0<RetType, T: QMetaMethod_attributes_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.attributes_0(self);
    // return 1;
  }
}
pub trait QMetaMethod_attributes_0<RetType> {
  fn attributes_0(self , rsthis: & QMetaMethod) -> RetType;
}
impl<'a> /*trait*/ QMetaMethod_attributes_0<i32> for () {
  fn attributes_0(self , rsthis: & QMetaMethod) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QMetaMethod10attributesEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmetaobject.h:75
// index:0
// Public Visibility=Default Availability=Available
// [4] int methodIndex() const

/*

*/
impl /*struct*/ QMetaMethod {
  pub fn methodIndex_0<RetType, T: QMetaMethod_methodIndex_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.methodIndex_0(self);
    // return 1;
  }
}
pub trait QMetaMethod_methodIndex_0<RetType> {
  fn methodIndex_0(self , rsthis: & QMetaMethod) -> RetType;
}
impl<'a> /*trait*/ QMetaMethod_methodIndex_0<i32> for () {
  fn methodIndex_0(self , rsthis: & QMetaMethod) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QMetaMethod11methodIndexEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmetaobject.h:76
// index:0
// Public Visibility=Default Availability=Available
// [4] int revision() const

/*

*/
impl /*struct*/ QMetaMethod {
  pub fn revision_0<RetType, T: QMetaMethod_revision_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.revision_0(self);
    // return 1;
  }
}
pub trait QMetaMethod_revision_0<RetType> {
  fn revision_0(self , rsthis: & QMetaMethod) -> RetType;
}
impl<'a> /*trait*/ QMetaMethod_revision_0<i32> for () {
  fn revision_0(self , rsthis: & QMetaMethod) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QMetaMethod8revisionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmetaobject.h:78
// index:0
// Public inline Visibility=Default Availability=Available
// [8] const QMetaObject * enclosingMetaObject() const

/*

*/
impl /*struct*/ QMetaMethod {
  pub fn enclosingMetaObject_0<RetType, T: QMetaMethod_enclosingMetaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.enclosingMetaObject_0(self);
    // return 1;
  }
}
pub trait QMetaMethod_enclosingMetaObject_0<RetType> {
  fn enclosingMetaObject_0(self , rsthis: & QMetaMethod) -> RetType;
}
impl<'a> /*trait*/ QMetaMethod_enclosingMetaObject_0<usize> for () {
  fn enclosingMetaObject_0(self , rsthis: & QMetaMethod) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QMetaMethod19enclosingMetaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmetaobject.h:80
// index:0
// Public Visibility=Default Availability=Available
// [1] bool invoke(QObject *, Qt::ConnectionType, QGenericReturnArgument, QGenericArgument, QGenericArgument, QGenericArgument, QGenericArgument, QGenericArgument, QGenericArgument, QGenericArgument, QGenericArgument, QGenericArgument, QGenericArgument) const

/*

*/
impl /*struct*/ QMetaMethod {
  pub fn invoke_0<RetType, T: QMetaMethod_invoke_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.invoke_0(self);
    // return 1;
  }
}
pub trait QMetaMethod_invoke_0<RetType> {
  fn invoke_0(self , rsthis: & QMetaMethod) -> RetType;
}
impl<'a> /*trait*/ QMetaMethod_invoke_0<bool> for (usize,i32,usize,usize,usize,usize,usize,usize,usize,usize,usize,usize,usize) {
  fn invoke_0(self , rsthis: & QMetaMethod) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
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
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QMetaMethod6invokeEP7QObjectN2Qt14ConnectionTypeE22QGenericReturnArgument16QGenericArgumentS5_S5_S5_S5_S5_S5_S5_S5_S5_", 13,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,arg0,arg1,arg2,arg3,arg4,arg5,arg6,arg7,arg8,arg9,arg10,arg11,arg12,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmetaobject.h:93
// index:1
// Public inline Visibility=Default Availability=Available
// [1] bool invoke(QObject *, QGenericReturnArgument, QGenericArgument, QGenericArgument, QGenericArgument, QGenericArgument, QGenericArgument, QGenericArgument, QGenericArgument, QGenericArgument, QGenericArgument, QGenericArgument) const

/*

*/
impl /*struct*/ QMetaMethod {
  pub fn invoke_1<RetType, T: QMetaMethod_invoke_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.invoke_1(self);
    // return 1;
  }
}
pub trait QMetaMethod_invoke_1<RetType> {
  fn invoke_1(self , rsthis: & QMetaMethod) -> RetType;
}
impl<'a> /*trait*/ QMetaMethod_invoke_1<bool> for (usize,usize,usize,usize,usize,usize,usize,usize,usize,usize,usize,usize) {
  fn invoke_1(self , rsthis: & QMetaMethod) -> bool {
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
    let arg10 = (&self.10/*.qclsinst*/) as *const usize as usize;
    let arg11 = (&self.11/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QMetaMethod6invokeEP7QObject22QGenericReturnArgument16QGenericArgumentS3_S3_S3_S3_S3_S3_S3_S3_S3_", 12,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,arg0,arg1,arg2,arg3,arg4,arg5,arg6,arg7,arg8,arg9,arg10,arg11,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmetaobject.h:109
// index:2
// Public inline Visibility=Default Availability=Available
// [1] bool invoke(QObject *, Qt::ConnectionType, QGenericArgument, QGenericArgument, QGenericArgument, QGenericArgument, QGenericArgument, QGenericArgument, QGenericArgument, QGenericArgument, QGenericArgument, QGenericArgument) const

/*

*/
impl /*struct*/ QMetaMethod {
  pub fn invoke_2<RetType, T: QMetaMethod_invoke_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.invoke_2(self);
    // return 1;
  }
}
pub trait QMetaMethod_invoke_2<RetType> {
  fn invoke_2(self , rsthis: & QMetaMethod) -> RetType;
}
impl<'a> /*trait*/ QMetaMethod_invoke_2<bool> for (usize,i32,usize,usize,usize,usize,usize,usize,usize,usize,usize,usize) {
  fn invoke_2(self , rsthis: & QMetaMethod) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
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
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QMetaMethod6invokeEP7QObjectN2Qt14ConnectionTypeE16QGenericArgumentS4_S4_S4_S4_S4_S4_S4_S4_S4_", 12,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,arg0,arg1,arg2,arg3,arg4,arg5,arg6,arg7,arg8,arg9,arg10,arg11,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmetaobject.h:125
// index:3
// Public inline Visibility=Default Availability=Available
// [1] bool invoke(QObject *, QGenericArgument, QGenericArgument, QGenericArgument, QGenericArgument, QGenericArgument, QGenericArgument, QGenericArgument, QGenericArgument, QGenericArgument, QGenericArgument) const

/*

*/
impl /*struct*/ QMetaMethod {
  pub fn invoke_3<RetType, T: QMetaMethod_invoke_3<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.invoke_3(self);
    // return 1;
  }
}
pub trait QMetaMethod_invoke_3<RetType> {
  fn invoke_3(self , rsthis: & QMetaMethod) -> RetType;
}
impl<'a> /*trait*/ QMetaMethod_invoke_3<bool> for (usize,usize,usize,usize,usize,usize,usize,usize,usize,usize,usize) {
  fn invoke_3(self , rsthis: & QMetaMethod) -> bool {
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
    let arg10 = (&self.10/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QMetaMethod6invokeEP7QObject16QGenericArgumentS2_S2_S2_S2_S2_S2_S2_S2_S2_", 11,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,arg0,arg1,arg2,arg3,arg4,arg5,arg6,arg7,arg8,arg9,arg10,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmetaobject.h:141
// index:0
// Public Visibility=Default Availability=Available
// [1] bool invokeOnGadget(void *, QGenericReturnArgument, QGenericArgument, QGenericArgument, QGenericArgument, QGenericArgument, QGenericArgument, QGenericArgument, QGenericArgument, QGenericArgument, QGenericArgument, QGenericArgument) const

/*

*/
impl /*struct*/ QMetaMethod {
  pub fn invokeOnGadget_0<RetType, T: QMetaMethod_invokeOnGadget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.invokeOnGadget_0(self);
    // return 1;
  }
}
pub trait QMetaMethod_invokeOnGadget_0<RetType> {
  fn invokeOnGadget_0(self , rsthis: & QMetaMethod) -> RetType;
}
impl<'a> /*trait*/ QMetaMethod_invokeOnGadget_0<bool> for (usize,usize,usize,usize,usize,usize,usize,usize,usize,usize,usize,usize) {
  fn invokeOnGadget_0(self , rsthis: & QMetaMethod) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
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
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QMetaMethod14invokeOnGadgetEPv22QGenericReturnArgument16QGenericArgumentS2_S2_S2_S2_S2_S2_S2_S2_S2_", 12,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,arg0,arg1,arg2,arg3,arg4,arg5,arg6,arg7,arg8,arg9,arg10,arg11,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmetaobject.h:153
// index:1
// Public inline Visibility=Default Availability=Available
// [1] bool invokeOnGadget(void *, QGenericArgument, QGenericArgument, QGenericArgument, QGenericArgument, QGenericArgument, QGenericArgument, QGenericArgument, QGenericArgument, QGenericArgument, QGenericArgument) const

/*

*/
impl /*struct*/ QMetaMethod {
  pub fn invokeOnGadget_1<RetType, T: QMetaMethod_invokeOnGadget_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.invokeOnGadget_1(self);
    // return 1;
  }
}
pub trait QMetaMethod_invokeOnGadget_1<RetType> {
  fn invokeOnGadget_1(self , rsthis: & QMetaMethod) -> RetType;
}
impl<'a> /*trait*/ QMetaMethod_invokeOnGadget_1<bool> for (usize,usize,usize,usize,usize,usize,usize,usize,usize,usize,usize) {
  fn invokeOnGadget_1(self , rsthis: & QMetaMethod) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
    let arg4 = (&self.4/*.qclsinst*/) as *const usize as usize;
    let arg5 = (&self.5/*.qclsinst*/) as *const usize as usize;
    let arg6 = (&self.6/*.qclsinst*/) as *const usize as usize;
    let arg7 = (&self.7/*.qclsinst*/) as *const usize as usize;
    let arg8 = (&self.8/*.qclsinst*/) as *const usize as usize;
    let arg9 = (&self.9/*.qclsinst*/) as *const usize as usize;
    let arg10 = (&self.10/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QMetaMethod14invokeOnGadgetEPv16QGenericArgumentS1_S1_S1_S1_S1_S1_S1_S1_S1_", 11,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,arg0,arg1,arg2,arg3,arg4,arg5,arg6,arg7,arg8,arg9,arg10,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmetaobject.h:169
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isValid() const

/*

*/
impl /*struct*/ QMetaMethod {
  pub fn isValid_0<RetType, T: QMetaMethod_isValid_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isValid_0(self);
    // return 1;
  }
}
pub trait QMetaMethod_isValid_0<RetType> {
  fn isValid_0(self , rsthis: & QMetaMethod) -> RetType;
}
impl<'a> /*trait*/ QMetaMethod_isValid_0<bool> for () {
  fn isValid_0(self , rsthis: & QMetaMethod) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QMetaMethod7isValidEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}


pub fn DeleteQMetaMethod(this :*mut QMetaMethod) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN11QMetaMethodD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}

/*


*/
pub type QMetaMethod__Access = i32;
// 
pub const QMetaMethod__Private :QMetaMethod__Access = 0;
// 
pub const QMetaMethod__Protected :QMetaMethod__Access = 1;
// 
pub const QMetaMethod__Public :QMetaMethod__Access = 2;
pub fn QMetaMethod_AccessItemName(val: i32) ->String {
  match val {
     QMetaMethod__Private => // 0
     {return String::from("Private");}
     QMetaMethod__Protected => // 1
     {return String::from("Protected");}
     QMetaMethod__Public => // 2
     {return String::from("Public");}
  _ => {return format!("{}", val);}
}
}
pub fn QMetaMethod_AccessItemName_s(val: i32) ->String {
  //var nilthis *QMetaMethod
  //return nilthis.AccessItemName(val);
  return QMetaMethod_AccessItemName(val);
}


/*


*/
pub type QMetaMethod__MethodType = i32;
// 
pub const QMetaMethod__Method :QMetaMethod__MethodType = 0;
// 
pub const QMetaMethod__Signal :QMetaMethod__MethodType = 1;
// 
pub const QMetaMethod__Slot :QMetaMethod__MethodType = 2;
// 
pub const QMetaMethod__Constructor :QMetaMethod__MethodType = 3;
pub fn QMetaMethod_MethodTypeItemName(val: i32) ->String {
  match val {
     QMetaMethod__Method => // 0
     {return String::from("Method");}
     QMetaMethod__Signal => // 1
     {return String::from("Signal");}
     QMetaMethod__Slot => // 2
     {return String::from("Slot");}
     QMetaMethod__Constructor => // 3
     {return String::from("Constructor");}
  _ => {return format!("{}", val);}
}
}
pub fn QMetaMethod_MethodTypeItemName_s(val: i32) ->String {
  //var nilthis *QMetaMethod
  //return nilthis.MethodTypeItemName(val);
  return QMetaMethod_MethodTypeItemName(val);
}


/*


*/
pub type QMetaMethod__Attributes = i32;
// 
pub const QMetaMethod__Compatibility :QMetaMethod__Attributes = 1;
// 
pub const QMetaMethod__Cloned :QMetaMethod__Attributes = 2;
// 
pub const QMetaMethod__Scriptable :QMetaMethod__Attributes = 4;
pub fn QMetaMethod_AttributesItemName(val: i32) ->String {
  match val {
     QMetaMethod__Compatibility => // 1
     {return String::from("Compatibility");}
     QMetaMethod__Cloned => // 2
     {return String::from("Cloned");}
     QMetaMethod__Scriptable => // 4
     {return String::from("Scriptable");}
  _ => {return format!("{}", val);}
}
}
pub fn QMetaMethod_AttributesItemName_s(val: i32) ->String {
  //var nilthis *QMetaMethod
  //return nilthis.AttributesItemName(val);
  return QMetaMethod_AttributesItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
