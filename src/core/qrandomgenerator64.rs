

// mod ::core::QRandomGenerator64
// package qtcore
// /usr/include/qt/QtCore/qrandom.h
// #include <qrandom.h>
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
#[derive(Default)] // class sizeof(QRandomGenerator64)=2512
pub struct QRandomGenerator64 {
  qbase: QRandomGenerator,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QRandomGenerator64_ITF interface {
//    QRandomGenerator_ITF
//    QRandomGenerator64_PTR() *QRandomGenerator64
//}
//func (ptr *QRandomGenerator64) QRandomGenerator64_PTR() *QRandomGenerator64 { return ptr }

impl /*struct*/ QRandomGenerator64 {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QRandomGenerator64 {
    return QRandomGenerator64{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QRandomGenerator64 {
//  type Target = QRandomGenerator64BASE;
//
//  fn deref(&self) -> &QRandomGenerator64BASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QRandomGenerator64BASE> for QRandomGenerator64 {
//  fn as_ref(& self) -> & QRandomGenerator64BASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qrandom.h:216
// index:0
// Public inline Visibility=Default Availability=Available
// [8] quint64 generate()

/*

*/
impl /*struct*/ QRandomGenerator64 {
  pub fn generate_0<RetType, T: QRandomGenerator64_generate_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.generate_0(self);
    // return 1;
  }
}
pub trait QRandomGenerator64_generate_0<RetType> {
  fn generate_0(self , rsthis: & QRandomGenerator64) -> RetType;
}
impl<'a> /*trait*/ QRandomGenerator64_generate_0<u64> for () {
  fn generate_0(self , rsthis: & QRandomGenerator64) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN18QRandomGenerator648generateEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qrandom.h:219
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QRandomGenerator64::result_type operator()()

/*

*/
impl /*struct*/ QRandomGenerator64 {
  pub fn operator_fncall_0<RetType, T: QRandomGenerator64_operator_fncall_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_fncall_0(self);
    // return 1;
  }
}
pub trait QRandomGenerator64_operator_fncall_0<RetType> {
  fn operator_fncall_0(self , rsthis: & QRandomGenerator64) -> RetType;
}
impl<'a> /*trait*/ QRandomGenerator64_operator_fncall_0<u64> for () {
  fn operator_fncall_0(self , rsthis: & QRandomGenerator64) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN18QRandomGenerator64clEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qrandom.h:222
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void QRandomGenerator64(quint32)

/*

*/
// QRandomGenerator64(quint32) ctx.fn_proto_cpp
impl /*struct*/ QRandomGenerator64 {
  pub fn QRandomGenerator64_0<T: QRandomGenerator64_QRandomGenerator64_0>(value: T) -> QRandomGenerator64 {
    let rsthis = value.QRandomGenerator64_0();
    return rsthis;
    // return 1;
  }
}

pub trait QRandomGenerator64_QRandomGenerator64_0 {
  fn QRandomGenerator64_0(self) -> QRandomGenerator64;
}
// QRandomGenerator64(quint32) ctx.fn_proto_cpp
impl<'a> /*trait*/ QRandomGenerator64_QRandomGenerator64_0 for (u32) {
  fn QRandomGenerator64_0(self) -> QRandomGenerator64 {
    // unsafe{_ZN18QRandomGenerator64C2Ej()};
    let arg0 = (&self) as *const u32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN18QRandomGenerator64C2Ej", 1,qtrt::FFITY_UINT32,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QRandomGenerator64{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qrandom.h:228
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void QRandomGenerator64(const quint32 *, qsizetype)

/*

*/
// QRandomGenerator64(const quint32 *, qsizetype) ctx.fn_proto_cpp
impl /*struct*/ QRandomGenerator64 {
  pub fn QRandomGenerator64_1<T: QRandomGenerator64_QRandomGenerator64_1>(value: T) -> QRandomGenerator64 {
    let rsthis = value.QRandomGenerator64_1();
    return rsthis;
    // return 1;
  }
}

pub trait QRandomGenerator64_QRandomGenerator64_1 {
  fn QRandomGenerator64_1(self) -> QRandomGenerator64;
}
// QRandomGenerator64(const quint32 *, qsizetype) ctx.fn_proto_cpp
impl<'a> /*trait*/ QRandomGenerator64_QRandomGenerator64_1 for (usize,i64) {
  fn QRandomGenerator64_1(self) -> QRandomGenerator64 {
    // unsafe{_ZN18QRandomGenerator64C2EPKjx()};
    let arg0 = (&self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const i64 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN18QRandomGenerator64C2EPKjx", 2,qtrt::FFITY_POINTER,qtrt::FFITY_SINT64,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QRandomGenerator64{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qrandom.h:234
// index:2
// Public inline Visibility=Default Availability=Available
// [-2] void QRandomGenerator64(const quint32 *, const quint32 *)

/*

*/
// QRandomGenerator64(const quint32 *, const quint32 *) ctx.fn_proto_cpp
impl /*struct*/ QRandomGenerator64 {
  pub fn QRandomGenerator64_2<T: QRandomGenerator64_QRandomGenerator64_2>(value: T) -> QRandomGenerator64 {
    let rsthis = value.QRandomGenerator64_2();
    return rsthis;
    // return 1;
  }
}

pub trait QRandomGenerator64_QRandomGenerator64_2 {
  fn QRandomGenerator64_2(self) -> QRandomGenerator64;
}
// QRandomGenerator64(const quint32 *, const quint32 *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QRandomGenerator64_QRandomGenerator64_2 for (usize,usize) {
  fn QRandomGenerator64_2(self) -> QRandomGenerator64 {
    // unsafe{_ZN18QRandomGenerator64C2EPKjS1_()};
    let arg0 = (&self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN18QRandomGenerator64C2EPKjS1_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QRandomGenerator64{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qrandom.h:237
// index:3
// Public inline Visibility=Default Availability=Available
// [-2] void QRandomGenerator64(const QRandomGenerator &)

/*

*/
// QRandomGenerator64(const QRandomGenerator &) ctx.fn_proto_cpp
impl /*struct*/ QRandomGenerator64 {
  pub fn QRandomGenerator64_3<T: QRandomGenerator64_QRandomGenerator64_3>(value: T) -> QRandomGenerator64 {
    let rsthis = value.QRandomGenerator64_3();
    return rsthis;
    // return 1;
  }
}

pub trait QRandomGenerator64_QRandomGenerator64_3 {
  fn QRandomGenerator64_3(self) -> QRandomGenerator64;
}
// QRandomGenerator64(const QRandomGenerator &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QRandomGenerator64_QRandomGenerator64_3 for (usize) {
  fn QRandomGenerator64_3(self) -> QRandomGenerator64 {
    // unsafe{_ZN18QRandomGenerator64C2ERK16QRandomGenerator()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN18QRandomGenerator64C2ERK16QRandomGenerator", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QRandomGenerator64{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qrandom.h:239
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void discard(unsigned long long)

/*

*/
impl /*struct*/ QRandomGenerator64 {
  pub fn discard_0<RetType, T: QRandomGenerator64_discard_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.discard_0(self);
    // return 1;
  }
}
pub trait QRandomGenerator64_discard_0<RetType> {
  fn discard_0(self , rsthis: & QRandomGenerator64) -> RetType;
}
impl<'a> /*trait*/ QRandomGenerator64_discard_0<(/*void*/)> for (u64) {
  fn discard_0(self , rsthis: & QRandomGenerator64) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const u64 as usize;
     qtrt::InvokeQtFunc6("_ZN18QRandomGenerator647discardEy", 1,qtrt::FFITY_UINT64,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qrandom.h:246
// index:0
// Public static inline Visibility=Default Availability=Available
// [8] QRandomGenerator64::result_type min()

/*

*/
impl /*struct*/ QRandomGenerator64 {
  pub fn min_0<RetType, T: QRandomGenerator64_min_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.min_0();
    // return 1;
  }
}
pub trait QRandomGenerator64_min_0<RetType> {
  fn min_0(self ) -> RetType;
}
impl<'a> /*trait*/ QRandomGenerator64_min_0<u64> for () {
  fn min_0(self ) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN18QRandomGenerator643minEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qrandom.h:247
// index:0
// Public static inline Visibility=Default Availability=Available
// [8] QRandomGenerator64::result_type max()

/*

*/
impl /*struct*/ QRandomGenerator64 {
  pub fn max_0<RetType, T: QRandomGenerator64_max_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.max_0();
    // return 1;
  }
}
pub trait QRandomGenerator64_max_0<RetType> {
  fn max_0(self ) -> RetType;
}
impl<'a> /*trait*/ QRandomGenerator64_max_0<u64> for () {
  fn max_0(self ) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN18QRandomGenerator643maxEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qrandom.h:248
// index:0
// Public static Visibility=Default Availability=Available
// [8] QRandomGenerator64 * system()

/*

*/
impl /*struct*/ QRandomGenerator64 {
  pub fn system_0<RetType, T: QRandomGenerator64_system_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.system_0();
    // return 1;
  }
}
pub trait QRandomGenerator64_system_0<RetType> {
  fn system_0(self ) -> RetType;
}
impl<'a> /*trait*/ QRandomGenerator64_system_0<usize> for () {
  fn system_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN18QRandomGenerator646systemEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qrandom.h:249
// index:0
// Public static Visibility=Default Availability=Available
// [8] QRandomGenerator64 * global()

/*

*/
impl /*struct*/ QRandomGenerator64 {
  pub fn global_0<RetType, T: QRandomGenerator64_global_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.global_0();
    // return 1;
  }
}
pub trait QRandomGenerator64_global_0<RetType> {
  fn global_0(self ) -> RetType;
}
impl<'a> /*trait*/ QRandomGenerator64_global_0<usize> for () {
  fn global_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN18QRandomGenerator646globalEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qrandom.h:250
// index:0
// Public static Visibility=Default Availability=Available
// [2512] QRandomGenerator64 securelySeeded()

/*

*/
impl /*struct*/ QRandomGenerator64 {
  pub fn securelySeeded_0<RetType, T: QRandomGenerator64_securelySeeded_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.securelySeeded_0();
    // return 1;
  }
}
pub trait QRandomGenerator64_securelySeeded_0<RetType> {
  fn securelySeeded_0(self ) -> RetType;
}
impl<'a> /*trait*/ QRandomGenerator64_securelySeeded_0<usize> for () {
  fn securelySeeded_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN18QRandomGenerator6414securelySeededEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}


pub fn DeleteQRandomGenerator64(this :*mut QRandomGenerator64) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN18QRandomGenerator64D2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
