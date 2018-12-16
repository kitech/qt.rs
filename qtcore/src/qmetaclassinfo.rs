

// mod ::core::QMetaClassInfo
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
// extern C begin: 32
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
#[derive(Default)] // class sizeof(QMetaClassInfo)=16
pub struct QMetaClassInfo {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QMetaClassInfo_ITF interface {
//    QMetaClassInfo_PTR() *QMetaClassInfo
//}
//func (ptr *QMetaClassInfo) QMetaClassInfo_PTR() *QMetaClassInfo { return ptr }

impl /*struct*/ QMetaClassInfo {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QMetaClassInfo {
    return QMetaClassInfo{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QMetaClassInfo {
//  type Target = QMetaClassInfoBASE;
//
//  fn deref(&self) -> &QMetaClassInfoBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QMetaClassInfoBASE> for QMetaClassInfo {
//  fn as_ref(& self) -> & QMetaClassInfoBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qmetaobject.h:303
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void QMetaClassInfo()

/*

*/
// QMetaClassInfo() ctx.fn_proto_cpp
impl /*struct*/ QMetaClassInfo {
  pub fn QMetaClassInfo_0<T: QMetaClassInfo_QMetaClassInfo_0>(value: T) -> QMetaClassInfo {
    let rsthis = value.QMetaClassInfo_0();
    return rsthis;
    // return 1;
  }
}

pub trait QMetaClassInfo_QMetaClassInfo_0 {
  fn QMetaClassInfo_0(self) -> QMetaClassInfo;
}
// QMetaClassInfo() ctx.fn_proto_cpp
impl<'a> /*trait*/ QMetaClassInfo_QMetaClassInfo_0 for () {
  fn QMetaClassInfo_0(self) -> QMetaClassInfo {
    // unsafe{_ZN14QMetaClassInfoC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN14QMetaClassInfoC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QMetaClassInfo{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qmetaobject.h:304
// index:0
// Public Visibility=Default Availability=Available
// [8] const char * name() const

/*

*/
impl /*struct*/ QMetaClassInfo {
  pub fn name_0<RetType, T: QMetaClassInfo_name_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.name_0(self);
    // return 1;
  }
}
pub trait QMetaClassInfo_name_0<RetType> {
  fn name_0(self , rsthis: & QMetaClassInfo) -> RetType;
}
impl<'a> /*trait*/ QMetaClassInfo_name_0<usize> for () {
  fn name_0(self , rsthis: & QMetaClassInfo) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QMetaClassInfo4nameEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmetaobject.h:305
// index:0
// Public Visibility=Default Availability=Available
// [8] const char * value() const

/*

*/
impl /*struct*/ QMetaClassInfo {
  pub fn value_0<RetType, T: QMetaClassInfo_value_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.value_0(self);
    // return 1;
  }
}
pub trait QMetaClassInfo_value_0<RetType> {
  fn value_0(self , rsthis: & QMetaClassInfo) -> RetType;
}
impl<'a> /*trait*/ QMetaClassInfo_value_0<usize> for () {
  fn value_0(self , rsthis: & QMetaClassInfo) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QMetaClassInfo5valueEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmetaobject.h:306
// index:0
// Public inline Visibility=Default Availability=Available
// [8] const QMetaObject * enclosingMetaObject() const

/*

*/
impl /*struct*/ QMetaClassInfo {
  pub fn enclosingMetaObject_0<RetType, T: QMetaClassInfo_enclosingMetaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.enclosingMetaObject_0(self);
    // return 1;
  }
}
pub trait QMetaClassInfo_enclosingMetaObject_0<RetType> {
  fn enclosingMetaObject_0(self , rsthis: & QMetaClassInfo) -> RetType;
}
impl<'a> /*trait*/ QMetaClassInfo_enclosingMetaObject_0<usize> for () {
  fn enclosingMetaObject_0(self , rsthis: & QMetaClassInfo) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QMetaClassInfo19enclosingMetaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}


pub fn DeleteQMetaClassInfo(this :*mut QMetaClassInfo) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN14QMetaClassInfoD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
