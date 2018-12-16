

// mod ::core::QRandomGenerator
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
// extern C begin: 11
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
#[derive(Default)] // class sizeof(QRandomGenerator)=2512
pub struct QRandomGenerator {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QRandomGenerator_ITF interface {
//    QRandomGenerator_PTR() *QRandomGenerator
//}
//func (ptr *QRandomGenerator) QRandomGenerator_PTR() *QRandomGenerator { return ptr }

impl /*struct*/ QRandomGenerator {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QRandomGenerator {
    return QRandomGenerator{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QRandomGenerator {
//  type Target = QRandomGeneratorBASE;
//
//  fn deref(&self) -> &QRandomGeneratorBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QRandomGeneratorBASE> for QRandomGenerator {
//  fn as_ref(& self) -> & QRandomGeneratorBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qrandom.h:62
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void QRandomGenerator(quint32)

/*

*/
// QRandomGenerator(quint32) ctx.fn_proto_cpp
impl /*struct*/ QRandomGenerator {
  pub fn QRandomGenerator_0<T: QRandomGenerator_QRandomGenerator_0>(value: T) -> QRandomGenerator {
    let rsthis = value.QRandomGenerator_0();
    return rsthis;
    // return 1;
  }
}

pub trait QRandomGenerator_QRandomGenerator_0 {
  fn QRandomGenerator_0(self) -> QRandomGenerator;
}
// QRandomGenerator(quint32) ctx.fn_proto_cpp
impl<'a> /*trait*/ QRandomGenerator_QRandomGenerator_0 for (u32) {
  fn QRandomGenerator_0(self) -> QRandomGenerator {
    // unsafe{_ZN16QRandomGeneratorC2Ej()};
    let arg0 = (&self) as *const u32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN16QRandomGeneratorC2Ej", 1,qtrt::FFITY_UINT32,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QRandomGenerator{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qrandom.h:68
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void QRandomGenerator(const quint32 *, qsizetype)

/*

*/
// QRandomGenerator(const quint32 *, qsizetype) ctx.fn_proto_cpp
impl /*struct*/ QRandomGenerator {
  pub fn QRandomGenerator_1<T: QRandomGenerator_QRandomGenerator_1>(value: T) -> QRandomGenerator {
    let rsthis = value.QRandomGenerator_1();
    return rsthis;
    // return 1;
  }
}

pub trait QRandomGenerator_QRandomGenerator_1 {
  fn QRandomGenerator_1(self) -> QRandomGenerator;
}
// QRandomGenerator(const quint32 *, qsizetype) ctx.fn_proto_cpp
impl<'a> /*trait*/ QRandomGenerator_QRandomGenerator_1 for (usize,i64) {
  fn QRandomGenerator_1(self) -> QRandomGenerator {
    // unsafe{_ZN16QRandomGeneratorC2EPKjx()};
    let arg0 = (&self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const i64 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN16QRandomGeneratorC2EPKjx", 2,qtrt::FFITY_POINTER,qtrt::FFITY_SINT64,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QRandomGenerator{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qrandom.h:72
// index:2
// Public Visibility=Default Availability=Available
// [-2] void QRandomGenerator(const quint32 *, const quint32 *)

/*

*/
// QRandomGenerator(const quint32 *, const quint32 *) ctx.fn_proto_cpp
impl /*struct*/ QRandomGenerator {
  pub fn QRandomGenerator_2<T: QRandomGenerator_QRandomGenerator_2>(value: T) -> QRandomGenerator {
    let rsthis = value.QRandomGenerator_2();
    return rsthis;
    // return 1;
  }
}

pub trait QRandomGenerator_QRandomGenerator_2 {
  fn QRandomGenerator_2(self) -> QRandomGenerator;
}
// QRandomGenerator(const quint32 *, const quint32 *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QRandomGenerator_QRandomGenerator_2 for (usize,usize) {
  fn QRandomGenerator_2(self) -> QRandomGenerator {
    // unsafe{_ZN16QRandomGeneratorC2EPKjS1_()};
    let arg0 = (&self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN16QRandomGeneratorC2EPKjS1_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QRandomGenerator{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qrandom.h:179
// index:3
// Protected Visibility=Default Availability=Available
// [-2] void QRandomGenerator(QRandomGenerator::System)

/*

*/
// QRandomGenerator(QRandomGenerator::System) ctx.fn_proto_cpp
impl /*struct*/ QRandomGenerator {
  pub fn QRandomGenerator_3<T: QRandomGenerator_QRandomGenerator_3>(value: T) -> QRandomGenerator {
    let rsthis = value.QRandomGenerator_3();
    return rsthis;
    // return 1;
  }
}

pub trait QRandomGenerator_QRandomGenerator_3 {
  fn QRandomGenerator_3(self) -> QRandomGenerator;
}
// QRandomGenerator(QRandomGenerator::System) ctx.fn_proto_cpp
impl<'a> /*trait*/ QRandomGenerator_QRandomGenerator_3 for (i32) {
  fn QRandomGenerator_3(self) -> QRandomGenerator {
    // unsafe{_ZN16QRandomGeneratorC2ENS_6SystemE()};
    let arg0 = (&self) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN16QRandomGeneratorC2ENS_6SystemE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QRandomGenerator{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qrandom.h:76
// index:0
// Public Visibility=Default Availability=Available
// [2512] QRandomGenerator & operator=(const QRandomGenerator &)

/*

*/
impl /*struct*/ QRandomGenerator {
  pub fn operator_equal_0<RetType, T: QRandomGenerator_operator_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_0(self);
    // return 1;
  }
}
pub trait QRandomGenerator_operator_equal_0<RetType> {
  fn operator_equal_0(self , rsthis: & QRandomGenerator) -> RetType;
}
impl<'a> /*trait*/ QRandomGenerator_operator_equal_0<usize> for (usize) {
  fn operator_equal_0(self , rsthis: & QRandomGenerator) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN16QRandomGeneratoraSERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qrandom.h:84
// index:0
// Public inline Visibility=Default Availability=Available
// [4] quint32 generate()

/*

*/
impl /*struct*/ QRandomGenerator {
  pub fn generate_0<RetType, T: QRandomGenerator_generate_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.generate_0(self);
    // return 1;
  }
}
pub trait QRandomGenerator_generate_0<RetType> {
  fn generate_0(self , rsthis: & QRandomGenerator) -> RetType;
}
impl<'a> /*trait*/ QRandomGenerator_generate_0<u32> for () {
  fn generate_0(self , rsthis: & QRandomGenerator) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN16QRandomGenerator8generateEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qrandom.h:159
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void generate(quint32 *, quint32 *)

/*

*/
impl /*struct*/ QRandomGenerator {
  pub fn generate_1<RetType, T: QRandomGenerator_generate_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.generate_1(self);
    // return 1;
  }
}
pub trait QRandomGenerator_generate_1<RetType> {
  fn generate_1(self , rsthis: & QRandomGenerator) -> RetType;
}
impl<'a> /*trait*/ QRandomGenerator_generate_1<(/*void*/)> for (usize,usize) {
  fn generate_1(self , rsthis: & QRandomGenerator) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN16QRandomGenerator8generateEPjS0_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qrandom.h:91
// index:0
// Public inline Visibility=Default Availability=Available
// [8] quint64 generate64()

/*

*/
impl /*struct*/ QRandomGenerator {
  pub fn generate64_0<RetType, T: QRandomGenerator_generate64_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.generate64_0(self);
    // return 1;
  }
}
pub trait QRandomGenerator_generate64_0<RetType> {
  fn generate64_0(self , rsthis: & QRandomGenerator) -> RetType;
}
impl<'a> /*trait*/ QRandomGenerator_generate64_0<u64> for () {
  fn generate64_0(self , rsthis: & QRandomGenerator) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN16QRandomGenerator10generate64Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qrandom.h:98
// index:0
// Public inline Visibility=Default Availability=Available
// [8] double generateDouble()

/*

*/
impl /*struct*/ QRandomGenerator {
  pub fn generateDouble_0<RetType, T: QRandomGenerator_generateDouble_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.generateDouble_0(self);
    // return 1;
  }
}
pub trait QRandomGenerator_generateDouble_0<RetType> {
  fn generateDouble_0(self , rsthis: & QRandomGenerator) -> RetType;
}
impl<'a> /*trait*/ QRandomGenerator_generateDouble_0<f64> for () {
  fn generateDouble_0(self , rsthis: & QRandomGenerator) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN16QRandomGenerator14generateDoubleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qrandom.h:112
// index:0
// Public inline Visibility=Default Availability=Available
// [8] double bounded(double)

/*

*/
impl /*struct*/ QRandomGenerator {
  pub fn bounded_0<RetType, T: QRandomGenerator_bounded_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.bounded_0(self);
    // return 1;
  }
}
pub trait QRandomGenerator_bounded_0<RetType> {
  fn bounded_0(self , rsthis: & QRandomGenerator) -> RetType;
}
impl<'a> /*trait*/ QRandomGenerator_bounded_0<f64> for (f64) {
  fn bounded_0(self , rsthis: & QRandomGenerator) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN16QRandomGenerator7boundedEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qrandom.h:117
// index:1
// Public inline Visibility=Default Availability=Available
// [4] quint32 bounded(quint32)

/*

*/
impl /*struct*/ QRandomGenerator {
  pub fn bounded_1<RetType, T: QRandomGenerator_bounded_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.bounded_1(self);
    // return 1;
  }
}
pub trait QRandomGenerator_bounded_1<RetType> {
  fn bounded_1(self , rsthis: & QRandomGenerator) -> RetType;
}
impl<'a> /*trait*/ QRandomGenerator_bounded_1<u32> for (u32) {
  fn bounded_1(self , rsthis: & QRandomGenerator) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const u32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN16QRandomGenerator7boundedEj", 1,qtrt::FFITY_UINT32,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qrandom.h:125
// index:2
// Public inline Visibility=Default Availability=Available
// [4] int bounded(int)

/*

*/
impl /*struct*/ QRandomGenerator {
  pub fn bounded_2<RetType, T: QRandomGenerator_bounded_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.bounded_2(self);
    // return 1;
  }
}
pub trait QRandomGenerator_bounded_2<RetType> {
  fn bounded_2(self , rsthis: & QRandomGenerator) -> RetType;
}
impl<'a> /*trait*/ QRandomGenerator_bounded_2<i32> for (i32) {
  fn bounded_2(self , rsthis: & QRandomGenerator) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN16QRandomGenerator7boundedEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qrandom.h:130
// index:3
// Public inline Visibility=Default Availability=Available
// [4] quint32 bounded(quint32, quint32)

/*

*/
impl /*struct*/ QRandomGenerator {
  pub fn bounded_3<RetType, T: QRandomGenerator_bounded_3<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.bounded_3(self);
    // return 1;
  }
}
pub trait QRandomGenerator_bounded_3<RetType> {
  fn bounded_3(self , rsthis: & QRandomGenerator) -> RetType;
}
impl<'a> /*trait*/ QRandomGenerator_bounded_3<u32> for (u32,u32) {
  fn bounded_3(self , rsthis: & QRandomGenerator) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const u32 as usize;
    let arg1 = (&self.1) as *const u32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN16QRandomGenerator7boundedEjj", 2,qtrt::FFITY_UINT32,qtrt::FFITY_UINT32,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qrandom.h:135
// index:4
// Public inline Visibility=Default Availability=Available
// [4] int bounded(int, int)

/*

*/
impl /*struct*/ QRandomGenerator {
  pub fn bounded_4<RetType, T: QRandomGenerator_bounded_4<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.bounded_4(self);
    // return 1;
  }
}
pub trait QRandomGenerator_bounded_4<RetType> {
  fn bounded_4(self , rsthis: & QRandomGenerator) -> RetType;
}
impl<'a> /*trait*/ QRandomGenerator_bounded_4<i32> for (i32,i32) {
  fn bounded_4(self , rsthis: & QRandomGenerator) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN16QRandomGenerator7boundedEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qrandom.h:166
// index:0
// Public inline Visibility=Default Availability=Available
// [4] QRandomGenerator::result_type operator()()

/*

*/
impl /*struct*/ QRandomGenerator {
  pub fn operator_fncall_0<RetType, T: QRandomGenerator_operator_fncall_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_fncall_0(self);
    // return 1;
  }
}
pub trait QRandomGenerator_operator_fncall_0<RetType> {
  fn operator_fncall_0(self , rsthis: & QRandomGenerator) -> RetType;
}
impl<'a> /*trait*/ QRandomGenerator_operator_fncall_0<u32> for () {
  fn operator_fncall_0(self , rsthis: & QRandomGenerator) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN16QRandomGeneratorclEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qrandom.h:167
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void seed(quint32)

/*

*/
impl /*struct*/ QRandomGenerator {
  pub fn seed_0<RetType, T: QRandomGenerator_seed_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.seed_0(self);
    // return 1;
  }
}
pub trait QRandomGenerator_seed_0<RetType> {
  fn seed_0(self , rsthis: & QRandomGenerator) -> RetType;
}
impl<'a> /*trait*/ QRandomGenerator_seed_0<(/*void*/)> for (u32) {
  fn seed_0(self , rsthis: & QRandomGenerator) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const u32 as usize;
     qtrt::InvokeQtFunc6("_ZN16QRandomGenerator4seedEj", 1,qtrt::FFITY_UINT32,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qrandom.h:169
// index:0
// Public Visibility=Default Availability=Available
// [-2] void discard(unsigned long long)

/*

*/
impl /*struct*/ QRandomGenerator {
  pub fn discard_0<RetType, T: QRandomGenerator_discard_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.discard_0(self);
    // return 1;
  }
}
pub trait QRandomGenerator_discard_0<RetType> {
  fn discard_0(self , rsthis: & QRandomGenerator) -> RetType;
}
impl<'a> /*trait*/ QRandomGenerator_discard_0<(/*void*/)> for (u64) {
  fn discard_0(self , rsthis: & QRandomGenerator) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const u64 as usize;
     qtrt::InvokeQtFunc6("_ZN16QRandomGenerator7discardEy", 1,qtrt::FFITY_UINT64,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qrandom.h:170
// index:0
// Public static inline Visibility=Default Availability=Available
// [4] QRandomGenerator::result_type min()

/*

*/
impl /*struct*/ QRandomGenerator {
  pub fn min_0<RetType, T: QRandomGenerator_min_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.min_0();
    // return 1;
  }
}
pub trait QRandomGenerator_min_0<RetType> {
  fn min_0(self ) -> RetType;
}
impl<'a> /*trait*/ QRandomGenerator_min_0<u32> for () {
  fn min_0(self ) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN16QRandomGenerator3minEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qrandom.h:171
// index:0
// Public static inline Visibility=Default Availability=Available
// [4] QRandomGenerator::result_type max()

/*

*/
impl /*struct*/ QRandomGenerator {
  pub fn max_0<RetType, T: QRandomGenerator_max_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.max_0();
    // return 1;
  }
}
pub trait QRandomGenerator_max_0<RetType> {
  fn max_0(self ) -> RetType;
}
impl<'a> /*trait*/ QRandomGenerator_max_0<u32> for () {
  fn max_0(self ) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN16QRandomGenerator3maxEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qrandom.h:173
// index:0
// Public static inline Visibility=Default Availability=Available
// [8] QRandomGenerator * system()

/*

*/
impl /*struct*/ QRandomGenerator {
  pub fn system_0<RetType, T: QRandomGenerator_system_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.system_0();
    // return 1;
  }
}
pub trait QRandomGenerator_system_0<RetType> {
  fn system_0(self ) -> RetType;
}
impl<'a> /*trait*/ QRandomGenerator_system_0<usize> for () {
  fn system_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN16QRandomGenerator6systemEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qrandom.h:174
// index:0
// Public static inline Visibility=Default Availability=Available
// [8] QRandomGenerator * global()

/*

*/
impl /*struct*/ QRandomGenerator {
  pub fn global_0<RetType, T: QRandomGenerator_global_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.global_0();
    // return 1;
  }
}
pub trait QRandomGenerator_global_0<RetType> {
  fn global_0(self ) -> RetType;
}
impl<'a> /*trait*/ QRandomGenerator_global_0<usize> for () {
  fn global_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN16QRandomGenerator6globalEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qrandom.h:175
// index:0
// Public static inline Visibility=Default Availability=Available
// [2512] QRandomGenerator securelySeeded()

/*

*/
impl /*struct*/ QRandomGenerator {
  pub fn securelySeeded_0<RetType, T: QRandomGenerator_securelySeeded_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.securelySeeded_0();
    // return 1;
  }
}
pub trait QRandomGenerator_securelySeeded_0<RetType> {
  fn securelySeeded_0(self ) -> RetType;
}
impl<'a> /*trait*/ QRandomGenerator_securelySeeded_0<usize> for () {
  fn securelySeeded_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN16QRandomGenerator14securelySeededEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}


pub fn DeleteQRandomGenerator(this :*mut QRandomGenerator) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN16QRandomGeneratorD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}

/*


*/
pub type QRandomGenerator__System = i32;
pub fn QRandomGenerator_SystemItemName(val: i32) ->String {
  match val {
  _ => {return format!("{}", val);}
}
}
pub fn QRandomGenerator_SystemItemName_s(val: i32) ->String {
  //var nilthis *QRandomGenerator
  //return nilthis.SystemItemName(val);
  return QRandomGenerator_SystemItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
