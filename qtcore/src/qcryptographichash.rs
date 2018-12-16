

// mod ::core::QCryptographicHash
// package qtcore
// /usr/include/qt/QtCore/qcryptographichash.h
// #include <qcryptographichash.h>
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
#[derive(Default)] // class sizeof(QCryptographicHash)=8
pub struct QCryptographicHash {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QCryptographicHash_ITF interface {
//    QCryptographicHash_PTR() *QCryptographicHash
//}
//func (ptr *QCryptographicHash) QCryptographicHash_PTR() *QCryptographicHash { return ptr }

impl /*struct*/ QCryptographicHash {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QCryptographicHash {
    return QCryptographicHash{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QCryptographicHash {
//  type Target = QCryptographicHashBASE;
//
//  fn deref(&self) -> &QCryptographicHashBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QCryptographicHashBASE> for QCryptographicHash {
//  fn as_ref(& self) -> & QCryptographicHashBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qcryptographichash.h:92
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QCryptographicHash(QCryptographicHash::Algorithm)

/*
Constructs an object that can be used to create a cryptographic hash from data using method.
*/
// QCryptographicHash(QCryptographicHash::Algorithm) ctx.fn_proto_cpp
impl /*struct*/ QCryptographicHash {
  pub fn QCryptographicHash_0<T: QCryptographicHash_QCryptographicHash_0>(value: T) -> QCryptographicHash {
    let rsthis = value.QCryptographicHash_0();
    return rsthis;
    // return 1;
  }
}

pub trait QCryptographicHash_QCryptographicHash_0 {
  fn QCryptographicHash_0(self) -> QCryptographicHash;
}
// QCryptographicHash(QCryptographicHash::Algorithm) ctx.fn_proto_cpp
impl<'a> /*trait*/ QCryptographicHash_QCryptographicHash_0 for (i32) {
  fn QCryptographicHash_0(self) -> QCryptographicHash {
    // unsafe{_ZN18QCryptographicHashC2ENS_9AlgorithmE()};
    let arg0 = (&self) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN18QCryptographicHashC2ENS_9AlgorithmE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QCryptographicHash{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qcryptographichash.h:93
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ~QCryptographicHash()

/*

*/
pub fn DeleteQCryptographicHash(this :*mut QCryptographicHash) {
    // let rv = qtrt::InvokeQtFunc6("_ZN18QCryptographicHashD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 8)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qcryptographichash.h:95
// index:0
// Public Visibility=Default Availability=Available
// [-2] void reset()

/*
Resets the object.
*/
impl /*struct*/ QCryptographicHash {
  pub fn reset_0<RetType, T: QCryptographicHash_reset_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.reset_0(self);
    // return 1;
  }
}
pub trait QCryptographicHash_reset_0<RetType> {
  fn reset_0(self , rsthis: & QCryptographicHash) -> RetType;
}
impl<'a> /*trait*/ QCryptographicHash_reset_0<(/*void*/)> for () {
  fn reset_0(self , rsthis: & QCryptographicHash) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN18QCryptographicHash5resetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qcryptographichash.h:97
// index:0
// Public Visibility=Default Availability=Available
// [-2] void addData(const char *, int)

/*
Adds the first length chars of data to the cryptographic hash.
*/
impl /*struct*/ QCryptographicHash {
  pub fn addData_0<RetType, T: QCryptographicHash_addData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addData_0(self);
    // return 1;
  }
}
pub trait QCryptographicHash_addData_0<RetType> {
  fn addData_0(self , rsthis: & QCryptographicHash) -> RetType;
}
impl<'a> /*trait*/ QCryptographicHash_addData_0<(/*void*/)> for (usize,i32) {
  fn addData_0(self , rsthis: & QCryptographicHash) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN18QCryptographicHash7addDataEPKci", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qcryptographichash.h:98
// index:1
// Public Visibility=Default Availability=Available
// [-2] void addData(const QByteArray &)

/*
Adds the first length chars of data to the cryptographic hash.
*/
impl /*struct*/ QCryptographicHash {
  pub fn addData_1<RetType, T: QCryptographicHash_addData_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addData_1(self);
    // return 1;
  }
}
pub trait QCryptographicHash_addData_1<RetType> {
  fn addData_1(self , rsthis: & QCryptographicHash) -> RetType;
}
impl<'a> /*trait*/ QCryptographicHash_addData_1<(/*void*/)> for (usize) {
  fn addData_1(self , rsthis: & QCryptographicHash) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN18QCryptographicHash7addDataERK10QByteArray", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qcryptographichash.h:99
// index:2
// Public Visibility=Default Availability=Available
// [1] bool addData(QIODevice *)

/*
Adds the first length chars of data to the cryptographic hash.
*/
impl /*struct*/ QCryptographicHash {
  pub fn addData_2<RetType, T: QCryptographicHash_addData_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addData_2(self);
    // return 1;
  }
}
pub trait QCryptographicHash_addData_2<RetType> {
  fn addData_2(self , rsthis: & QCryptographicHash) -> RetType;
}
impl<'a> /*trait*/ QCryptographicHash_addData_2<bool> for (usize) {
  fn addData_2(self , rsthis: & QCryptographicHash) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN18QCryptographicHash7addDataEP9QIODevice", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qcryptographichash.h:101
// index:0
// Public Visibility=Default Availability=Available
// [8] QByteArray result() const

/*
Returns the final hash value.

See also QByteArray::toHex().
*/
impl /*struct*/ QCryptographicHash {
  pub fn result_0<RetType, T: QCryptographicHash_result_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.result_0(self);
    // return 1;
  }
}
pub trait QCryptographicHash_result_0<RetType> {
  fn result_0(self , rsthis: & QCryptographicHash) -> RetType;
}
impl<'a> /*trait*/ QCryptographicHash_result_0<usize> for () {
  fn result_0(self , rsthis: & QCryptographicHash) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QCryptographicHash6resultEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qcryptographichash.h:103
// index:0
// Public static Visibility=Default Availability=Available
// [8] QByteArray hash(const QByteArray &, QCryptographicHash::Algorithm)

/*
Returns the hash of data using method.
*/
impl /*struct*/ QCryptographicHash {
  pub fn hash_0<RetType, T: QCryptographicHash_hash_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.hash_0();
    // return 1;
  }
}
pub trait QCryptographicHash_hash_0<RetType> {
  fn hash_0(self ) -> RetType;
}
impl<'a> /*trait*/ QCryptographicHash_hash_0<usize> for (usize,i32) {
  fn hash_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN18QCryptographicHash4hashERK10QByteArrayNS_9AlgorithmE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}


/*
Note: In Qt versions before 5.9, when asked to generate a SHA3 hash sum, QCryptographicHash actually calculated Keccak. If you need compatibility with SHA-3 hashes produced by those versions of Qt, use the Keccak_ enumerators. Alternatively, if source compatibility is required, define the macro QT_SHA3_KECCAK_COMPAT.


*/
pub type QCryptographicHash__Algorithm = i32;
// 
pub const QCryptographicHash__Md4 :QCryptographicHash__Algorithm = 0;
// 
pub const QCryptographicHash__Md5 :QCryptographicHash__Algorithm = 1;
// 
pub const QCryptographicHash__Sha1 :QCryptographicHash__Algorithm = 2;
// 
pub const QCryptographicHash__Sha224 :QCryptographicHash__Algorithm = 3;
// 
pub const QCryptographicHash__Sha256 :QCryptographicHash__Algorithm = 4;
// 
pub const QCryptographicHash__Sha384 :QCryptographicHash__Algorithm = 5;
// 
pub const QCryptographicHash__Sha512 :QCryptographicHash__Algorithm = 6;
// 
pub const QCryptographicHash__Keccak_224 :QCryptographicHash__Algorithm = 7;
// 
pub const QCryptographicHash__Keccak_256 :QCryptographicHash__Algorithm = 8;
// 
pub const QCryptographicHash__Keccak_384 :QCryptographicHash__Algorithm = 9;
// 
pub const QCryptographicHash__Keccak_512 :QCryptographicHash__Algorithm = 10;
// 
pub const QCryptographicHash__RealSha3_224 :QCryptographicHash__Algorithm = 11;
// 
pub const QCryptographicHash__RealSha3_256 :QCryptographicHash__Algorithm = 12;
// 
pub const QCryptographicHash__RealSha3_384 :QCryptographicHash__Algorithm = 13;
// 
pub const QCryptographicHash__RealSha3_512 :QCryptographicHash__Algorithm = 14;
// 
pub const QCryptographicHash__Sha3_224 :QCryptographicHash__Algorithm = 11;
// 
pub const QCryptographicHash__Sha3_256 :QCryptographicHash__Algorithm = 12;
// 
pub const QCryptographicHash__Sha3_384 :QCryptographicHash__Algorithm = 13;
// 
pub const QCryptographicHash__Sha3_512 :QCryptographicHash__Algorithm = 14;
pub fn QCryptographicHash_AlgorithmItemName(val: i32) ->String {
  match val {
     QCryptographicHash__Md4 => // 0
     {return String::from("Md4");}
     QCryptographicHash__Md5 => // 1
     {return String::from("Md5");}
     QCryptographicHash__Sha1 => // 2
     {return String::from("Sha1");}
     QCryptographicHash__Sha224 => // 3
     {return String::from("Sha224");}
     QCryptographicHash__Sha256 => // 4
     {return String::from("Sha256");}
     QCryptographicHash__Sha384 => // 5
     {return String::from("Sha384");}
     QCryptographicHash__Sha512 => // 6
     {return String::from("Sha512");}
     QCryptographicHash__Keccak_224 => // 7
     {return String::from("Keccak_224");}
     QCryptographicHash__Keccak_256 => // 8
     {return String::from("Keccak_256");}
     QCryptographicHash__Keccak_384 => // 9
     {return String::from("Keccak_384");}
     QCryptographicHash__Keccak_512 => // 10
     {return String::from("Keccak_512");}
     QCryptographicHash__RealSha3_224 => // 11
     {return String::from("RealSha3_224,Sha3_224");}
     QCryptographicHash__RealSha3_256 => // 12
     {return String::from("RealSha3_256,Sha3_256");}
     QCryptographicHash__RealSha3_384 => // 13
     {return String::from("RealSha3_384,Sha3_384");}
     QCryptographicHash__RealSha3_512 => // 14
     {return String::from("RealSha3_512,Sha3_512");}
    // QCryptographicHash__Sha3_224 => // 11
    // {return String::from("");}
    // QCryptographicHash__Sha3_256 => // 12
    // {return String::from("");}
    // QCryptographicHash__Sha3_384 => // 13
    // {return String::from("");}
    // QCryptographicHash__Sha3_512 => // 14
    // {return String::from("");}
  _ => {return format!("{}", val);}
}
}
pub fn QCryptographicHash_AlgorithmItemName_s(val: i32) ->String {
  //var nilthis *QCryptographicHash
  //return nilthis.AlgorithmItemName(val);
  return QCryptographicHash_AlgorithmItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
