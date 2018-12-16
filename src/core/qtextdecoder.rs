

// mod ::core::QTextDecoder
// package qtcore
// /usr/include/qt/QtCore/qtextcodec.h
// #include <qtextcodec.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 7
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
#[derive(Default)] // class sizeof(QTextDecoder)=40
pub struct QTextDecoder {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QTextDecoder_ITF interface {
//    QTextDecoder_PTR() *QTextDecoder
//}
//func (ptr *QTextDecoder) QTextDecoder_PTR() *QTextDecoder { return ptr }

impl /*struct*/ QTextDecoder {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QTextDecoder {
    return QTextDecoder{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QTextDecoder {
//  type Target = QTextDecoderBASE;
//
//  fn deref(&self) -> &QTextDecoderBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QTextDecoderBASE> for QTextDecoder {
//  fn as_ref(& self) -> & QTextDecoderBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qtextcodec.h:158
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void QTextDecoder(const QTextCodec *)

/*

*/
// QTextDecoder(const QTextCodec *) ctx.fn_proto_cpp
impl /*struct*/ QTextDecoder {
  pub fn QTextDecoder_0<T: QTextDecoder_QTextDecoder_0>(value: T) -> QTextDecoder {
    let rsthis = value.QTextDecoder_0();
    return rsthis;
    // return 1;
  }
}

pub trait QTextDecoder_QTextDecoder_0 {
  fn QTextDecoder_0(self) -> QTextDecoder;
}
// QTextDecoder(const QTextCodec *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QTextDecoder_QTextDecoder_0 for (usize) {
  fn QTextDecoder_0(self) -> QTextDecoder {
    // unsafe{_ZN12QTextDecoderC2EPK10QTextCodec()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN12QTextDecoderC2EPK10QTextCodec", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTextDecoder{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qtextcodec.h:159
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QTextDecoder(const QTextCodec *, QTextCodec::ConversionFlags)

/*

*/
// QTextDecoder(const QTextCodec *, QTextCodec::ConversionFlags) ctx.fn_proto_cpp
impl /*struct*/ QTextDecoder {
  pub fn QTextDecoder_1<T: QTextDecoder_QTextDecoder_1>(value: T) -> QTextDecoder {
    let rsthis = value.QTextDecoder_1();
    return rsthis;
    // return 1;
  }
}

pub trait QTextDecoder_QTextDecoder_1 {
  fn QTextDecoder_1(self) -> QTextDecoder;
}
// QTextDecoder(const QTextCodec *, QTextCodec::ConversionFlags) ctx.fn_proto_cpp
impl<'a> /*trait*/ QTextDecoder_QTextDecoder_1 for (usize,i32) {
  fn QTextDecoder_1(self) -> QTextDecoder {
    // unsafe{_ZN12QTextDecoderC2EPK10QTextCodec6QFlagsINS0_14ConversionFlagEE()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN12QTextDecoderC2EPK10QTextCodec6QFlagsINS0_14ConversionFlagEE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTextDecoder{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qtextcodec.h:160
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ~QTextDecoder()

/*

*/
pub fn DeleteQTextDecoder(this :*mut QTextDecoder) {
    // let rv = qtrt::InvokeQtFunc6("_ZN12QTextDecoderD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 40)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qtextcodec.h:161
// index:0
// Public Visibility=Default Availability=Available
// [8] QString toUnicode(const char *, int)

/*
Converts a from the encoding of this codec to Unicode, and returns the result in a QString.
*/
impl /*struct*/ QTextDecoder {
  pub fn toUnicode_0<RetType, T: QTextDecoder_toUnicode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toUnicode_0(self);
    // return 1;
  }
}
pub trait QTextDecoder_toUnicode_0<RetType> {
  fn toUnicode_0(self , rsthis: & QTextDecoder) -> RetType;
}
impl<'a> /*trait*/ QTextDecoder_toUnicode_0<usize> for (usize,i32) {
  fn toUnicode_0(self , rsthis: & QTextDecoder) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN12QTextDecoder9toUnicodeEPKci", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtextcodec.h:162
// index:1
// Public Visibility=Default Availability=Available
// [8] QString toUnicode(const QByteArray &)

/*
Converts a from the encoding of this codec to Unicode, and returns the result in a QString.
*/
impl /*struct*/ QTextDecoder {
  pub fn toUnicode_1<RetType, T: QTextDecoder_toUnicode_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toUnicode_1(self);
    // return 1;
  }
}
pub trait QTextDecoder_toUnicode_1<RetType> {
  fn toUnicode_1(self , rsthis: & QTextDecoder) -> RetType;
}
impl<'a> /*trait*/ QTextDecoder_toUnicode_1<usize> for (usize) {
  fn toUnicode_1(self , rsthis: & QTextDecoder) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN12QTextDecoder9toUnicodeERK10QByteArray", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtextcodec.h:163
// index:2
// Public Visibility=Default Availability=Available
// [-2] void toUnicode(QString *, const char *, int)

/*
Converts a from the encoding of this codec to Unicode, and returns the result in a QString.
*/
impl /*struct*/ QTextDecoder {
  pub fn toUnicode_2<RetType, T: QTextDecoder_toUnicode_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toUnicode_2(self);
    // return 1;
  }
}
pub trait QTextDecoder_toUnicode_2<RetType> {
  fn toUnicode_2(self , rsthis: & QTextDecoder) -> RetType;
}
impl<'a> /*trait*/ QTextDecoder_toUnicode_2<(/*void*/)> for (usize,usize,i32) {
  fn toUnicode_2(self , rsthis: & QTextDecoder) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (self.1) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN12QTextDecoder9toUnicodeEP7QStringPKci", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qtextcodec.h:164
// index:0
// Public Visibility=Default Availability=Available
// [1] bool hasFailure() const

/*

*/
impl /*struct*/ QTextDecoder {
  pub fn hasFailure_0<RetType, T: QTextDecoder_hasFailure_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hasFailure_0(self);
    // return 1;
  }
}
pub trait QTextDecoder_hasFailure_0<RetType> {
  fn hasFailure_0(self , rsthis: & QTextDecoder) -> RetType;
}
impl<'a> /*trait*/ QTextDecoder_hasFailure_0<bool> for () {
  fn hasFailure_0(self , rsthis: & QTextDecoder) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QTextDecoder10hasFailureEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
