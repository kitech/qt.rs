

// mod ::core::QHashData
// package qtcore
// /usr/include/qt/QtCore/qhash.h
// #include <qhash.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 2
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
#[derive(Default)] // class sizeof(QHashData)=48
pub struct QHashData {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QHashData_ITF interface {
//    QHashData_PTR() *QHashData
//}
//func (ptr *QHashData) QHashData_PTR() *QHashData { return ptr }

impl /*struct*/ QHashData {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QHashData {
    return QHashData{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QHashData {
//  type Target = QHashDataBASE;
//
//  fn deref(&self) -> &QHashDataBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QHashDataBASE> for QHashData {
//  fn as_ref(& self) -> & QHashDataBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qhash.h:84
// index:0
// Public Visibility=Default Availability=Available
// [8] void * allocateNode(int)

/*

*/
impl /*struct*/ QHashData {
  pub fn allocateNode_0<RetType, T: QHashData_allocateNode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.allocateNode_0(self);
    // return 1;
  }
}
pub trait QHashData_allocateNode_0<RetType> {
  fn allocateNode_0(self , rsthis: & QHashData) -> RetType;
}
impl<'a> /*trait*/ QHashData_allocateNode_0<usize> for (i32) {
  fn allocateNode_0(self , rsthis: & QHashData) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QHashData12allocateNodeEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qhash.h:85
// index:0
// Public Visibility=Default Availability=Available
// [-2] void freeNode(void *)

/*

*/
impl /*struct*/ QHashData {
  pub fn freeNode_0<RetType, T: QHashData_freeNode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.freeNode_0(self);
    // return 1;
  }
}
pub trait QHashData_freeNode_0<RetType> {
  fn freeNode_0(self , rsthis: & QHashData) -> RetType;
}
impl<'a> /*trait*/ QHashData_freeNode_0<(/*void*/)> for (usize) {
  fn freeNode_0(self , rsthis: & QHashData) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QHashData8freeNodeEPv", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qhash.h:86
// index:0
// Public Visibility=Default Availability=Available
// [8] QHashData * detach_helper(void (*)(QHashData::Node *, void *), void (*)(QHashData::Node *), int, int)

/*

*/
impl /*struct*/ QHashData {
  pub fn detach_helper_0<RetType, T: QHashData_detach_helper_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.detach_helper_0(self);
    // return 1;
  }
}
pub trait QHashData_detach_helper_0<RetType> {
  fn detach_helper_0(self , rsthis: & QHashData) -> RetType;
}
impl<'a> /*trait*/ QHashData_detach_helper_0<usize> for (usize,usize,i32,i32) {
  fn detach_helper_0(self , rsthis: & QHashData) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QHashData13detach_helperEPFvPNS_4NodeEPvEPFvS1_Eii", 4,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qhash.h:88
// index:0
// Public Visibility=Default Availability=Available
// [1] bool willGrow()

/*

*/
impl /*struct*/ QHashData {
  pub fn willGrow_0<RetType, T: QHashData_willGrow_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.willGrow_0(self);
    // return 1;
  }
}
pub trait QHashData_willGrow_0<RetType> {
  fn willGrow_0(self , rsthis: & QHashData) -> RetType;
}
impl<'a> /*trait*/ QHashData_willGrow_0<bool> for () {
  fn willGrow_0(self , rsthis: & QHashData) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QHashData8willGrowEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qhash.h:89
// index:0
// Public Visibility=Default Availability=Available
// [-2] void hasShrunk()

/*

*/
impl /*struct*/ QHashData {
  pub fn hasShrunk_0<RetType, T: QHashData_hasShrunk_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hasShrunk_0(self);
    // return 1;
  }
}
pub trait QHashData_hasShrunk_0<RetType> {
  fn hasShrunk_0(self , rsthis: & QHashData) -> RetType;
}
impl<'a> /*trait*/ QHashData_hasShrunk_0<(/*void*/)> for () {
  fn hasShrunk_0(self , rsthis: & QHashData) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN9QHashData9hasShrunkEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qhash.h:90
// index:0
// Public Visibility=Default Availability=Available
// [-2] void rehash(int)

/*

*/
impl /*struct*/ QHashData {
  pub fn rehash_0<RetType, T: QHashData_rehash_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rehash_0(self);
    // return 1;
  }
}
pub trait QHashData_rehash_0<RetType> {
  fn rehash_0(self , rsthis: & QHashData) -> RetType;
}
impl<'a> /*trait*/ QHashData_rehash_0<(/*void*/)> for (i32) {
  fn rehash_0(self , rsthis: & QHashData) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QHashData6rehashEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qhash.h:91
// index:0
// Public Visibility=Default Availability=Available
// [-2] void free_helper(void (*)(QHashData::Node *))

/*

*/
impl /*struct*/ QHashData {
  pub fn free_helper_0<RetType, T: QHashData_free_helper_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.free_helper_0(self);
    // return 1;
  }
}
pub trait QHashData_free_helper_0<RetType> {
  fn free_helper_0(self , rsthis: & QHashData) -> RetType;
}
impl<'a> /*trait*/ QHashData_free_helper_0<(/*void*/)> for (usize) {
  fn free_helper_0(self , rsthis: & QHashData) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QHashData11free_helperEPFvPNS_4NodeEE", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qhash.h:92
// index:0
// Public Visibility=Default Availability=Available
// [8] QHashData::Node * firstNode()

/*

*/
impl /*struct*/ QHashData {
  pub fn firstNode_0<RetType, T: QHashData_firstNode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.firstNode_0(self);
    // return 1;
  }
}
pub trait QHashData_firstNode_0<RetType> {
  fn firstNode_0(self , rsthis: & QHashData) -> RetType;
}
impl<'a> /*trait*/ QHashData_firstNode_0<usize> for () {
  fn firstNode_0(self , rsthis: & QHashData) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QHashData9firstNodeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}


pub fn DeleteQHashData(this :*mut QHashData) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN9QHashDataD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
