

// mod ::core::QTextEncoder
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
#[derive(Default)] // class sizeof(QTextEncoder)=40
pub struct QTextEncoder {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QTextEncoder_ITF interface {
//    QTextEncoder_PTR() *QTextEncoder
//}
//func (ptr *QTextEncoder) QTextEncoder_PTR() *QTextEncoder { return ptr }

impl /*struct*/ QTextEncoder {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QTextEncoder {
    return QTextEncoder{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QTextEncoder {
//  type Target = QTextEncoderBASE;
//
//  fn deref(&self) -> &QTextEncoderBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QTextEncoderBASE> for QTextEncoder {
//  fn as_ref(& self) -> & QTextEncoderBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qtextcodec.h:141
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void QTextEncoder(const QTextCodec *)

/*

*/
// QTextEncoder(const QTextCodec *) ctx.fn_proto_cpp
impl /*struct*/ QTextEncoder {
  pub fn QTextEncoder_0<T: QTextEncoder_QTextEncoder_0>(value: T) -> QTextEncoder {
    let rsthis = value.QTextEncoder_0();
    return rsthis;
    // return 1;
  }
}

pub trait QTextEncoder_QTextEncoder_0 {
  fn QTextEncoder_0(self) -> QTextEncoder;
}
// QTextEncoder(const QTextCodec *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QTextEncoder_QTextEncoder_0 for (usize) {
  fn QTextEncoder_0(self) -> QTextEncoder {
    // unsafe{_ZN12QTextEncoderC2EPK10QTextCodec()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN12QTextEncoderC2EPK10QTextCodec", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTextEncoder{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qtextcodec.h:142
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QTextEncoder(const QTextCodec *, QTextCodec::ConversionFlags)

/*

*/
// QTextEncoder(const QTextCodec *, QTextCodec::ConversionFlags) ctx.fn_proto_cpp
impl /*struct*/ QTextEncoder {
  pub fn QTextEncoder_1<T: QTextEncoder_QTextEncoder_1>(value: T) -> QTextEncoder {
    let rsthis = value.QTextEncoder_1();
    return rsthis;
    // return 1;
  }
}

pub trait QTextEncoder_QTextEncoder_1 {
  fn QTextEncoder_1(self) -> QTextEncoder;
}
// QTextEncoder(const QTextCodec *, QTextCodec::ConversionFlags) ctx.fn_proto_cpp
impl<'a> /*trait*/ QTextEncoder_QTextEncoder_1 for (usize,i32) {
  fn QTextEncoder_1(self) -> QTextEncoder {
    // unsafe{_ZN12QTextEncoderC2EPK10QTextCodec6QFlagsINS0_14ConversionFlagEE()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN12QTextEncoderC2EPK10QTextCodec6QFlagsINS0_14ConversionFlagEE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTextEncoder{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qtextcodec.h:143
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ~QTextEncoder()

/*

*/
pub fn DeleteQTextEncoder(this :*mut QTextEncoder) {
    // let rv = qtrt::InvokeQtFunc6("_ZN12QTextEncoderD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 40)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qtextcodec.h:145
// index:0
// Public Visibility=Default Availability=Available
// [8] QByteArray fromUnicode(const QString &)

/*
Converts str from Unicode to the encoding of this codec, and returns the result in a QByteArray.
*/
impl /*struct*/ QTextEncoder {
  pub fn fromUnicode_0<RetType, T: QTextEncoder_fromUnicode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fromUnicode_0(self);
    // return 1;
  }
}
pub trait QTextEncoder_fromUnicode_0<RetType> {
  fn fromUnicode_0(self , rsthis: & QTextEncoder) -> RetType;
}
impl<'a> /*trait*/ QTextEncoder_fromUnicode_0<usize> for (usize) {
  fn fromUnicode_0(self , rsthis: & QTextEncoder) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN12QTextEncoder11fromUnicodeERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtextcodec.h:147
// index:1
// Public Visibility=Default Availability=Available
// [8] QByteArray fromUnicode(QStringView)

/*
Converts str from Unicode to the encoding of this codec, and returns the result in a QByteArray.
*/
impl /*struct*/ QTextEncoder {
  pub fn fromUnicode_1<RetType, T: QTextEncoder_fromUnicode_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fromUnicode_1(self);
    // return 1;
  }
}
pub trait QTextEncoder_fromUnicode_1<RetType> {
  fn fromUnicode_1(self , rsthis: & QTextEncoder) -> RetType;
}
impl<'a> /*trait*/ QTextEncoder_fromUnicode_1<usize> for (usize) {
  fn fromUnicode_1(self , rsthis: & QTextEncoder) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN12QTextEncoder11fromUnicodeE11QStringView", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtextcodec.h:148
// index:2
// Public Visibility=Default Availability=Available
// [8] QByteArray fromUnicode(const QChar *, int)

/*
Converts str from Unicode to the encoding of this codec, and returns the result in a QByteArray.
*/
impl /*struct*/ QTextEncoder {
  pub fn fromUnicode_2<RetType, T: QTextEncoder_fromUnicode_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fromUnicode_2(self);
    // return 1;
  }
}
pub trait QTextEncoder_fromUnicode_2<RetType> {
  fn fromUnicode_2(self , rsthis: & QTextEncoder) -> RetType;
}
impl<'a> /*trait*/ QTextEncoder_fromUnicode_2<usize> for (usize,i32) {
  fn fromUnicode_2(self , rsthis: & QTextEncoder) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN12QTextEncoder11fromUnicodeEPK5QChari", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtextcodec.h:149
// index:0
// Public Visibility=Default Availability=Available
// [1] bool hasFailure() const

/*

*/
impl /*struct*/ QTextEncoder {
  pub fn hasFailure_0<RetType, T: QTextEncoder_hasFailure_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hasFailure_0(self);
    // return 1;
  }
}
pub trait QTextEncoder_hasFailure_0<RetType> {
  fn hasFailure_0(self , rsthis: & QTextEncoder) -> RetType;
}
impl<'a> /*trait*/ QTextEncoder_hasFailure_0<bool> for () {
  fn hasFailure_0(self , rsthis: & QTextEncoder) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QTextEncoder10hasFailureEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
