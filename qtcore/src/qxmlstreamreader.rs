

// mod ::core::QXmlStreamReader
// package qtcore
// /usr/include/qt/QtCore/qxmlstream.h
// #include <qxmlstream.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 3
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
#[derive(Default)] // class sizeof(QXmlStreamReader)=8
pub struct QXmlStreamReader {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QXmlStreamReader_ITF interface {
//    QXmlStreamReader_PTR() *QXmlStreamReader
//}
//func (ptr *QXmlStreamReader) QXmlStreamReader_PTR() *QXmlStreamReader { return ptr }

impl /*struct*/ QXmlStreamReader {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QXmlStreamReader {
    return QXmlStreamReader{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QXmlStreamReader {
//  type Target = QXmlStreamReaderBASE;
//
//  fn deref(&self) -> &QXmlStreamReaderBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QXmlStreamReaderBASE> for QXmlStreamReader {
//  fn as_ref(& self) -> & QXmlStreamReaderBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qxmlstream.h:360
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QXmlStreamReader()

/*

*/
// QXmlStreamReader() ctx.fn_proto_cpp
impl /*struct*/ QXmlStreamReader {
  pub fn QXmlStreamReader_0<T: QXmlStreamReader_QXmlStreamReader_0>(value: T) -> QXmlStreamReader {
    let rsthis = value.QXmlStreamReader_0();
    return rsthis;
    // return 1;
  }
}

pub trait QXmlStreamReader_QXmlStreamReader_0 {
  fn QXmlStreamReader_0(self) -> QXmlStreamReader;
}
// QXmlStreamReader() ctx.fn_proto_cpp
impl<'a> /*trait*/ QXmlStreamReader_QXmlStreamReader_0 for () {
  fn QXmlStreamReader_0(self) -> QXmlStreamReader {
    // unsafe{_ZN16QXmlStreamReaderC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN16QXmlStreamReaderC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QXmlStreamReader{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:361
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QXmlStreamReader(QIODevice *)

/*

*/
// QXmlStreamReader(QIODevice *) ctx.fn_proto_cpp
impl /*struct*/ QXmlStreamReader {
  pub fn QXmlStreamReader_1<T: QXmlStreamReader_QXmlStreamReader_1>(value: T) -> QXmlStreamReader {
    let rsthis = value.QXmlStreamReader_1();
    return rsthis;
    // return 1;
  }
}

pub trait QXmlStreamReader_QXmlStreamReader_1 {
  fn QXmlStreamReader_1(self) -> QXmlStreamReader;
}
// QXmlStreamReader(QIODevice *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QXmlStreamReader_QXmlStreamReader_1 for (usize) {
  fn QXmlStreamReader_1(self) -> QXmlStreamReader {
    // unsafe{_ZN16QXmlStreamReaderC2EP9QIODevice()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN16QXmlStreamReaderC2EP9QIODevice", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QXmlStreamReader{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:362
// index:2
// Public Visibility=Default Availability=Available
// [-2] void QXmlStreamReader(const QByteArray &)

/*

*/
// QXmlStreamReader(const QByteArray &) ctx.fn_proto_cpp
impl /*struct*/ QXmlStreamReader {
  pub fn QXmlStreamReader_2<T: QXmlStreamReader_QXmlStreamReader_2>(value: T) -> QXmlStreamReader {
    let rsthis = value.QXmlStreamReader_2();
    return rsthis;
    // return 1;
  }
}

pub trait QXmlStreamReader_QXmlStreamReader_2 {
  fn QXmlStreamReader_2(self) -> QXmlStreamReader;
}
// QXmlStreamReader(const QByteArray &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QXmlStreamReader_QXmlStreamReader_2 for (usize) {
  fn QXmlStreamReader_2(self) -> QXmlStreamReader {
    // unsafe{_ZN16QXmlStreamReaderC2ERK10QByteArray()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN16QXmlStreamReaderC2ERK10QByteArray", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QXmlStreamReader{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:363
// index:3
// Public Visibility=Default Availability=Available
// [-2] void QXmlStreamReader(const QString &)

/*

*/
// QXmlStreamReader(const QString &) ctx.fn_proto_cpp
impl /*struct*/ QXmlStreamReader {
  pub fn QXmlStreamReader_3<T: QXmlStreamReader_QXmlStreamReader_3>(value: T) -> QXmlStreamReader {
    let rsthis = value.QXmlStreamReader_3();
    return rsthis;
    // return 1;
  }
}

pub trait QXmlStreamReader_QXmlStreamReader_3 {
  fn QXmlStreamReader_3(self) -> QXmlStreamReader;
}
// QXmlStreamReader(const QString &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QXmlStreamReader_QXmlStreamReader_3 for (usize) {
  fn QXmlStreamReader_3(self) -> QXmlStreamReader {
    // unsafe{_ZN16QXmlStreamReaderC2ERK7QString()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN16QXmlStreamReaderC2ERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QXmlStreamReader{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:364
// index:4
// Public Visibility=Default Availability=Available
// [-2] void QXmlStreamReader(const char *)

/*

*/
// QXmlStreamReader(const char *) ctx.fn_proto_cpp
impl /*struct*/ QXmlStreamReader {
  pub fn QXmlStreamReader_4<T: QXmlStreamReader_QXmlStreamReader_4>(value: T) -> QXmlStreamReader {
    let rsthis = value.QXmlStreamReader_4();
    return rsthis;
    // return 1;
  }
}

pub trait QXmlStreamReader_QXmlStreamReader_4 {
  fn QXmlStreamReader_4(self) -> QXmlStreamReader;
}
// QXmlStreamReader(const char *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QXmlStreamReader_QXmlStreamReader_4 for (usize) {
  fn QXmlStreamReader_4(self) -> QXmlStreamReader {
    // unsafe{_ZN16QXmlStreamReaderC2EPKc()};
    let arg0 = (self) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN16QXmlStreamReaderC2EPKc", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QXmlStreamReader{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:365
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ~QXmlStreamReader()

/*

*/
pub fn DeleteQXmlStreamReader(this :*mut QXmlStreamReader) {
    // let rv = qtrt::InvokeQtFunc6("_ZN16QXmlStreamReaderD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 8)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qxmlstream.h:367
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDevice(QIODevice *)

/*

*/
impl /*struct*/ QXmlStreamReader {
  pub fn setDevice_0<RetType, T: QXmlStreamReader_setDevice_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDevice_0(self);
    // return 1;
  }
}
pub trait QXmlStreamReader_setDevice_0<RetType> {
  fn setDevice_0(self , rsthis: & QXmlStreamReader) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamReader_setDevice_0<(/*void*/)> for (usize) {
  fn setDevice_0(self , rsthis: & QXmlStreamReader) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN16QXmlStreamReader9setDeviceEP9QIODevice", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:368
// index:0
// Public Visibility=Default Availability=Available
// [8] QIODevice * device() const

/*

*/
impl /*struct*/ QXmlStreamReader {
  pub fn device_0<RetType, T: QXmlStreamReader_device_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.device_0(self);
    // return 1;
  }
}
pub trait QXmlStreamReader_device_0<RetType> {
  fn device_0(self , rsthis: & QXmlStreamReader) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamReader_device_0<usize> for () {
  fn device_0(self , rsthis: & QXmlStreamReader) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QXmlStreamReader6deviceEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:369
// index:0
// Public Visibility=Default Availability=Available
// [-2] void addData(const QByteArray &)

/*

*/
impl /*struct*/ QXmlStreamReader {
  pub fn addData_0<RetType, T: QXmlStreamReader_addData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addData_0(self);
    // return 1;
  }
}
pub trait QXmlStreamReader_addData_0<RetType> {
  fn addData_0(self , rsthis: & QXmlStreamReader) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamReader_addData_0<(/*void*/)> for (usize) {
  fn addData_0(self , rsthis: & QXmlStreamReader) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN16QXmlStreamReader7addDataERK10QByteArray", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:370
// index:1
// Public Visibility=Default Availability=Available
// [-2] void addData(const QString &)

/*

*/
impl /*struct*/ QXmlStreamReader {
  pub fn addData_1<RetType, T: QXmlStreamReader_addData_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addData_1(self);
    // return 1;
  }
}
pub trait QXmlStreamReader_addData_1<RetType> {
  fn addData_1(self , rsthis: & QXmlStreamReader) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamReader_addData_1<(/*void*/)> for (usize) {
  fn addData_1(self , rsthis: & QXmlStreamReader) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN16QXmlStreamReader7addDataERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:371
// index:2
// Public Visibility=Default Availability=Available
// [-2] void addData(const char *)

/*

*/
impl /*struct*/ QXmlStreamReader {
  pub fn addData_2<RetType, T: QXmlStreamReader_addData_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addData_2(self);
    // return 1;
  }
}
pub trait QXmlStreamReader_addData_2<RetType> {
  fn addData_2(self , rsthis: & QXmlStreamReader) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamReader_addData_2<(/*void*/)> for (usize) {
  fn addData_2(self , rsthis: & QXmlStreamReader) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN16QXmlStreamReader7addDataEPKc", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:372
// index:0
// Public Visibility=Default Availability=Available
// [-2] void clear()

/*

*/
impl /*struct*/ QXmlStreamReader {
  pub fn clear_0<RetType, T: QXmlStreamReader_clear_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.clear_0(self);
    // return 1;
  }
}
pub trait QXmlStreamReader_clear_0<RetType> {
  fn clear_0(self , rsthis: & QXmlStreamReader) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamReader_clear_0<(/*void*/)> for () {
  fn clear_0(self , rsthis: & QXmlStreamReader) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN16QXmlStreamReader5clearEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:375
// index:0
// Public Visibility=Default Availability=Available
// [1] bool atEnd() const

/*

*/
impl /*struct*/ QXmlStreamReader {
  pub fn atEnd_0<RetType, T: QXmlStreamReader_atEnd_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.atEnd_0(self);
    // return 1;
  }
}
pub trait QXmlStreamReader_atEnd_0<RetType> {
  fn atEnd_0(self , rsthis: & QXmlStreamReader) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamReader_atEnd_0<bool> for () {
  fn atEnd_0(self , rsthis: & QXmlStreamReader) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QXmlStreamReader5atEndEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:376
// index:0
// Public Visibility=Default Availability=Available
// [4] QXmlStreamReader::TokenType readNext()

/*

*/
impl /*struct*/ QXmlStreamReader {
  pub fn readNext_0<RetType, T: QXmlStreamReader_readNext_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.readNext_0(self);
    // return 1;
  }
}
pub trait QXmlStreamReader_readNext_0<RetType> {
  fn readNext_0(self , rsthis: & QXmlStreamReader) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamReader_readNext_0<i32> for () {
  fn readNext_0(self , rsthis: & QXmlStreamReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN16QXmlStreamReader8readNextEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:378
// index:0
// Public Visibility=Default Availability=Available
// [1] bool readNextStartElement()

/*

*/
impl /*struct*/ QXmlStreamReader {
  pub fn readNextStartElement_0<RetType, T: QXmlStreamReader_readNextStartElement_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.readNextStartElement_0(self);
    // return 1;
  }
}
pub trait QXmlStreamReader_readNextStartElement_0<RetType> {
  fn readNextStartElement_0(self , rsthis: & QXmlStreamReader) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamReader_readNextStartElement_0<bool> for () {
  fn readNextStartElement_0(self , rsthis: & QXmlStreamReader) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN16QXmlStreamReader20readNextStartElementEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:379
// index:0
// Public Visibility=Default Availability=Available
// [-2] void skipCurrentElement()

/*

*/
impl /*struct*/ QXmlStreamReader {
  pub fn skipCurrentElement_0<RetType, T: QXmlStreamReader_skipCurrentElement_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.skipCurrentElement_0(self);
    // return 1;
  }
}
pub trait QXmlStreamReader_skipCurrentElement_0<RetType> {
  fn skipCurrentElement_0(self , rsthis: & QXmlStreamReader) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamReader_skipCurrentElement_0<(/*void*/)> for () {
  fn skipCurrentElement_0(self , rsthis: & QXmlStreamReader) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN16QXmlStreamReader18skipCurrentElementEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:381
// index:0
// Public Visibility=Default Availability=Available
// [4] QXmlStreamReader::TokenType tokenType() const

/*

*/
impl /*struct*/ QXmlStreamReader {
  pub fn tokenType_0<RetType, T: QXmlStreamReader_tokenType_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.tokenType_0(self);
    // return 1;
  }
}
pub trait QXmlStreamReader_tokenType_0<RetType> {
  fn tokenType_0(self , rsthis: & QXmlStreamReader) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamReader_tokenType_0<i32> for () {
  fn tokenType_0(self , rsthis: & QXmlStreamReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QXmlStreamReader9tokenTypeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:382
// index:0
// Public Visibility=Default Availability=Available
// [8] QString tokenString() const

/*

*/
impl /*struct*/ QXmlStreamReader {
  pub fn tokenString_0<RetType, T: QXmlStreamReader_tokenString_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.tokenString_0(self);
    // return 1;
  }
}
pub trait QXmlStreamReader_tokenString_0<RetType> {
  fn tokenString_0(self , rsthis: & QXmlStreamReader) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamReader_tokenString_0<usize> for () {
  fn tokenString_0(self , rsthis: & QXmlStreamReader) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QXmlStreamReader11tokenStringEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:384
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setNamespaceProcessing(bool)

/*

*/
impl /*struct*/ QXmlStreamReader {
  pub fn setNamespaceProcessing_0<RetType, T: QXmlStreamReader_setNamespaceProcessing_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setNamespaceProcessing_0(self);
    // return 1;
  }
}
pub trait QXmlStreamReader_setNamespaceProcessing_0<RetType> {
  fn setNamespaceProcessing_0(self , rsthis: & QXmlStreamReader) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamReader_setNamespaceProcessing_0<(/*void*/)> for (bool) {
  fn setNamespaceProcessing_0(self , rsthis: & QXmlStreamReader) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN16QXmlStreamReader22setNamespaceProcessingEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:385
// index:0
// Public Visibility=Default Availability=Available
// [1] bool namespaceProcessing() const

/*

*/
impl /*struct*/ QXmlStreamReader {
  pub fn namespaceProcessing_0<RetType, T: QXmlStreamReader_namespaceProcessing_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.namespaceProcessing_0(self);
    // return 1;
  }
}
pub trait QXmlStreamReader_namespaceProcessing_0<RetType> {
  fn namespaceProcessing_0(self , rsthis: & QXmlStreamReader) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamReader_namespaceProcessing_0<bool> for () {
  fn namespaceProcessing_0(self , rsthis: & QXmlStreamReader) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QXmlStreamReader19namespaceProcessingEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:387
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isStartDocument() const

/*

*/
impl /*struct*/ QXmlStreamReader {
  pub fn isStartDocument_0<RetType, T: QXmlStreamReader_isStartDocument_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isStartDocument_0(self);
    // return 1;
  }
}
pub trait QXmlStreamReader_isStartDocument_0<RetType> {
  fn isStartDocument_0(self , rsthis: & QXmlStreamReader) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamReader_isStartDocument_0<bool> for () {
  fn isStartDocument_0(self , rsthis: & QXmlStreamReader) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QXmlStreamReader15isStartDocumentEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:388
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isEndDocument() const

/*

*/
impl /*struct*/ QXmlStreamReader {
  pub fn isEndDocument_0<RetType, T: QXmlStreamReader_isEndDocument_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isEndDocument_0(self);
    // return 1;
  }
}
pub trait QXmlStreamReader_isEndDocument_0<RetType> {
  fn isEndDocument_0(self , rsthis: & QXmlStreamReader) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamReader_isEndDocument_0<bool> for () {
  fn isEndDocument_0(self , rsthis: & QXmlStreamReader) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QXmlStreamReader13isEndDocumentEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:389
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isStartElement() const

/*

*/
impl /*struct*/ QXmlStreamReader {
  pub fn isStartElement_0<RetType, T: QXmlStreamReader_isStartElement_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isStartElement_0(self);
    // return 1;
  }
}
pub trait QXmlStreamReader_isStartElement_0<RetType> {
  fn isStartElement_0(self , rsthis: & QXmlStreamReader) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamReader_isStartElement_0<bool> for () {
  fn isStartElement_0(self , rsthis: & QXmlStreamReader) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QXmlStreamReader14isStartElementEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:390
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isEndElement() const

/*

*/
impl /*struct*/ QXmlStreamReader {
  pub fn isEndElement_0<RetType, T: QXmlStreamReader_isEndElement_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isEndElement_0(self);
    // return 1;
  }
}
pub trait QXmlStreamReader_isEndElement_0<RetType> {
  fn isEndElement_0(self , rsthis: & QXmlStreamReader) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamReader_isEndElement_0<bool> for () {
  fn isEndElement_0(self , rsthis: & QXmlStreamReader) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QXmlStreamReader12isEndElementEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:391
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isCharacters() const

/*

*/
impl /*struct*/ QXmlStreamReader {
  pub fn isCharacters_0<RetType, T: QXmlStreamReader_isCharacters_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isCharacters_0(self);
    // return 1;
  }
}
pub trait QXmlStreamReader_isCharacters_0<RetType> {
  fn isCharacters_0(self , rsthis: & QXmlStreamReader) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamReader_isCharacters_0<bool> for () {
  fn isCharacters_0(self , rsthis: & QXmlStreamReader) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QXmlStreamReader12isCharactersEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:392
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isWhitespace() const

/*

*/
impl /*struct*/ QXmlStreamReader {
  pub fn isWhitespace_0<RetType, T: QXmlStreamReader_isWhitespace_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isWhitespace_0(self);
    // return 1;
  }
}
pub trait QXmlStreamReader_isWhitespace_0<RetType> {
  fn isWhitespace_0(self , rsthis: & QXmlStreamReader) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamReader_isWhitespace_0<bool> for () {
  fn isWhitespace_0(self , rsthis: & QXmlStreamReader) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QXmlStreamReader12isWhitespaceEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:393
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isCDATA() const

/*

*/
impl /*struct*/ QXmlStreamReader {
  pub fn isCDATA_0<RetType, T: QXmlStreamReader_isCDATA_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isCDATA_0(self);
    // return 1;
  }
}
pub trait QXmlStreamReader_isCDATA_0<RetType> {
  fn isCDATA_0(self , rsthis: & QXmlStreamReader) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamReader_isCDATA_0<bool> for () {
  fn isCDATA_0(self , rsthis: & QXmlStreamReader) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QXmlStreamReader7isCDATAEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:394
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isComment() const

/*

*/
impl /*struct*/ QXmlStreamReader {
  pub fn isComment_0<RetType, T: QXmlStreamReader_isComment_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isComment_0(self);
    // return 1;
  }
}
pub trait QXmlStreamReader_isComment_0<RetType> {
  fn isComment_0(self , rsthis: & QXmlStreamReader) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamReader_isComment_0<bool> for () {
  fn isComment_0(self , rsthis: & QXmlStreamReader) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QXmlStreamReader9isCommentEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:395
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isDTD() const

/*

*/
impl /*struct*/ QXmlStreamReader {
  pub fn isDTD_0<RetType, T: QXmlStreamReader_isDTD_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isDTD_0(self);
    // return 1;
  }
}
pub trait QXmlStreamReader_isDTD_0<RetType> {
  fn isDTD_0(self , rsthis: & QXmlStreamReader) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamReader_isDTD_0<bool> for () {
  fn isDTD_0(self , rsthis: & QXmlStreamReader) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QXmlStreamReader5isDTDEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:396
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isEntityReference() const

/*

*/
impl /*struct*/ QXmlStreamReader {
  pub fn isEntityReference_0<RetType, T: QXmlStreamReader_isEntityReference_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isEntityReference_0(self);
    // return 1;
  }
}
pub trait QXmlStreamReader_isEntityReference_0<RetType> {
  fn isEntityReference_0(self , rsthis: & QXmlStreamReader) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamReader_isEntityReference_0<bool> for () {
  fn isEntityReference_0(self , rsthis: & QXmlStreamReader) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QXmlStreamReader17isEntityReferenceEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:397
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isProcessingInstruction() const

/*

*/
impl /*struct*/ QXmlStreamReader {
  pub fn isProcessingInstruction_0<RetType, T: QXmlStreamReader_isProcessingInstruction_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isProcessingInstruction_0(self);
    // return 1;
  }
}
pub trait QXmlStreamReader_isProcessingInstruction_0<RetType> {
  fn isProcessingInstruction_0(self , rsthis: & QXmlStreamReader) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamReader_isProcessingInstruction_0<bool> for () {
  fn isProcessingInstruction_0(self , rsthis: & QXmlStreamReader) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QXmlStreamReader23isProcessingInstructionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:399
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isStandaloneDocument() const

/*

*/
impl /*struct*/ QXmlStreamReader {
  pub fn isStandaloneDocument_0<RetType, T: QXmlStreamReader_isStandaloneDocument_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isStandaloneDocument_0(self);
    // return 1;
  }
}
pub trait QXmlStreamReader_isStandaloneDocument_0<RetType> {
  fn isStandaloneDocument_0(self , rsthis: & QXmlStreamReader) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamReader_isStandaloneDocument_0<bool> for () {
  fn isStandaloneDocument_0(self , rsthis: & QXmlStreamReader) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QXmlStreamReader20isStandaloneDocumentEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:400
// index:0
// Public Visibility=Default Availability=Available
// [16] QStringRef documentVersion() const

/*

*/
impl /*struct*/ QXmlStreamReader {
  pub fn documentVersion_0<RetType, T: QXmlStreamReader_documentVersion_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.documentVersion_0(self);
    // return 1;
  }
}
pub trait QXmlStreamReader_documentVersion_0<RetType> {
  fn documentVersion_0(self , rsthis: & QXmlStreamReader) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamReader_documentVersion_0<usize> for () {
  fn documentVersion_0(self , rsthis: & QXmlStreamReader) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QXmlStreamReader15documentVersionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:401
// index:0
// Public Visibility=Default Availability=Available
// [16] QStringRef documentEncoding() const

/*

*/
impl /*struct*/ QXmlStreamReader {
  pub fn documentEncoding_0<RetType, T: QXmlStreamReader_documentEncoding_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.documentEncoding_0(self);
    // return 1;
  }
}
pub trait QXmlStreamReader_documentEncoding_0<RetType> {
  fn documentEncoding_0(self , rsthis: & QXmlStreamReader) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamReader_documentEncoding_0<usize> for () {
  fn documentEncoding_0(self , rsthis: & QXmlStreamReader) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QXmlStreamReader16documentEncodingEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:403
// index:0
// Public Visibility=Default Availability=Available
// [8] qint64 lineNumber() const

/*

*/
impl /*struct*/ QXmlStreamReader {
  pub fn lineNumber_0<RetType, T: QXmlStreamReader_lineNumber_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.lineNumber_0(self);
    // return 1;
  }
}
pub trait QXmlStreamReader_lineNumber_0<RetType> {
  fn lineNumber_0(self , rsthis: & QXmlStreamReader) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamReader_lineNumber_0<i64> for () {
  fn lineNumber_0(self , rsthis: & QXmlStreamReader) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QXmlStreamReader10lineNumberEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:404
// index:0
// Public Visibility=Default Availability=Available
// [8] qint64 columnNumber() const

/*

*/
impl /*struct*/ QXmlStreamReader {
  pub fn columnNumber_0<RetType, T: QXmlStreamReader_columnNumber_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.columnNumber_0(self);
    // return 1;
  }
}
pub trait QXmlStreamReader_columnNumber_0<RetType> {
  fn columnNumber_0(self , rsthis: & QXmlStreamReader) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamReader_columnNumber_0<i64> for () {
  fn columnNumber_0(self , rsthis: & QXmlStreamReader) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QXmlStreamReader12columnNumberEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:405
// index:0
// Public Visibility=Default Availability=Available
// [8] qint64 characterOffset() const

/*

*/
impl /*struct*/ QXmlStreamReader {
  pub fn characterOffset_0<RetType, T: QXmlStreamReader_characterOffset_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.characterOffset_0(self);
    // return 1;
  }
}
pub trait QXmlStreamReader_characterOffset_0<RetType> {
  fn characterOffset_0(self , rsthis: & QXmlStreamReader) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamReader_characterOffset_0<i64> for () {
  fn characterOffset_0(self , rsthis: & QXmlStreamReader) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QXmlStreamReader15characterOffsetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:407
// index:0
// Public Visibility=Default Availability=Available
// [8] QXmlStreamAttributes attributes() const

/*

*/
impl /*struct*/ QXmlStreamReader {
  pub fn attributes_0<RetType, T: QXmlStreamReader_attributes_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.attributes_0(self);
    // return 1;
  }
}
pub trait QXmlStreamReader_attributes_0<RetType> {
  fn attributes_0(self , rsthis: & QXmlStreamReader) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamReader_attributes_0<usize> for () {
  fn attributes_0(self , rsthis: & QXmlStreamReader) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QXmlStreamReader10attributesEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:414
// index:0
// Public Visibility=Default Availability=Available
// [8] QString readElementText(QXmlStreamReader::ReadElementTextBehaviour)

/*

*/
impl /*struct*/ QXmlStreamReader {
  pub fn readElementText_0<RetType, T: QXmlStreamReader_readElementText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.readElementText_0(self);
    // return 1;
  }
}
pub trait QXmlStreamReader_readElementText_0<RetType> {
  fn readElementText_0(self , rsthis: & QXmlStreamReader) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamReader_readElementText_0<usize> for (i32) {
  fn readElementText_0(self , rsthis: & QXmlStreamReader) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN16QXmlStreamReader15readElementTextENS_24ReadElementTextBehaviourE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:416
// index:0
// Public Visibility=Default Availability=Available
// [16] QStringRef name() const

/*

*/
impl /*struct*/ QXmlStreamReader {
  pub fn name_0<RetType, T: QXmlStreamReader_name_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.name_0(self);
    // return 1;
  }
}
pub trait QXmlStreamReader_name_0<RetType> {
  fn name_0(self , rsthis: & QXmlStreamReader) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamReader_name_0<usize> for () {
  fn name_0(self , rsthis: & QXmlStreamReader) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QXmlStreamReader4nameEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:417
// index:0
// Public Visibility=Default Availability=Available
// [16] QStringRef namespaceUri() const

/*

*/
impl /*struct*/ QXmlStreamReader {
  pub fn namespaceUri_0<RetType, T: QXmlStreamReader_namespaceUri_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.namespaceUri_0(self);
    // return 1;
  }
}
pub trait QXmlStreamReader_namespaceUri_0<RetType> {
  fn namespaceUri_0(self , rsthis: & QXmlStreamReader) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamReader_namespaceUri_0<usize> for () {
  fn namespaceUri_0(self , rsthis: & QXmlStreamReader) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QXmlStreamReader12namespaceUriEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:418
// index:0
// Public Visibility=Default Availability=Available
// [16] QStringRef qualifiedName() const

/*

*/
impl /*struct*/ QXmlStreamReader {
  pub fn qualifiedName_0<RetType, T: QXmlStreamReader_qualifiedName_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.qualifiedName_0(self);
    // return 1;
  }
}
pub trait QXmlStreamReader_qualifiedName_0<RetType> {
  fn qualifiedName_0(self , rsthis: & QXmlStreamReader) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamReader_qualifiedName_0<usize> for () {
  fn qualifiedName_0(self , rsthis: & QXmlStreamReader) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QXmlStreamReader13qualifiedNameEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:419
// index:0
// Public Visibility=Default Availability=Available
// [16] QStringRef prefix() const

/*

*/
impl /*struct*/ QXmlStreamReader {
  pub fn prefix_0<RetType, T: QXmlStreamReader_prefix_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.prefix_0(self);
    // return 1;
  }
}
pub trait QXmlStreamReader_prefix_0<RetType> {
  fn prefix_0(self , rsthis: & QXmlStreamReader) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamReader_prefix_0<usize> for () {
  fn prefix_0(self , rsthis: & QXmlStreamReader) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QXmlStreamReader6prefixEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:421
// index:0
// Public Visibility=Default Availability=Available
// [16] QStringRef processingInstructionTarget() const

/*

*/
impl /*struct*/ QXmlStreamReader {
  pub fn processingInstructionTarget_0<RetType, T: QXmlStreamReader_processingInstructionTarget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.processingInstructionTarget_0(self);
    // return 1;
  }
}
pub trait QXmlStreamReader_processingInstructionTarget_0<RetType> {
  fn processingInstructionTarget_0(self , rsthis: & QXmlStreamReader) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamReader_processingInstructionTarget_0<usize> for () {
  fn processingInstructionTarget_0(self , rsthis: & QXmlStreamReader) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QXmlStreamReader27processingInstructionTargetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:422
// index:0
// Public Visibility=Default Availability=Available
// [16] QStringRef processingInstructionData() const

/*

*/
impl /*struct*/ QXmlStreamReader {
  pub fn processingInstructionData_0<RetType, T: QXmlStreamReader_processingInstructionData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.processingInstructionData_0(self);
    // return 1;
  }
}
pub trait QXmlStreamReader_processingInstructionData_0<RetType> {
  fn processingInstructionData_0(self , rsthis: & QXmlStreamReader) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamReader_processingInstructionData_0<usize> for () {
  fn processingInstructionData_0(self , rsthis: & QXmlStreamReader) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QXmlStreamReader25processingInstructionDataEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:424
// index:0
// Public Visibility=Default Availability=Available
// [16] QStringRef text() const

/*

*/
impl /*struct*/ QXmlStreamReader {
  pub fn text_0<RetType, T: QXmlStreamReader_text_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.text_0(self);
    // return 1;
  }
}
pub trait QXmlStreamReader_text_0<RetType> {
  fn text_0(self , rsthis: & QXmlStreamReader) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamReader_text_0<usize> for () {
  fn text_0(self , rsthis: & QXmlStreamReader) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QXmlStreamReader4textEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:427
// index:0
// Public Visibility=Default Availability=Available
// [-2] void addExtraNamespaceDeclaration(const QXmlStreamNamespaceDeclaration &)

/*

*/
impl /*struct*/ QXmlStreamReader {
  pub fn addExtraNamespaceDeclaration_0<RetType, T: QXmlStreamReader_addExtraNamespaceDeclaration_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addExtraNamespaceDeclaration_0(self);
    // return 1;
  }
}
pub trait QXmlStreamReader_addExtraNamespaceDeclaration_0<RetType> {
  fn addExtraNamespaceDeclaration_0(self , rsthis: & QXmlStreamReader) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamReader_addExtraNamespaceDeclaration_0<(/*void*/)> for (usize) {
  fn addExtraNamespaceDeclaration_0(self , rsthis: & QXmlStreamReader) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN16QXmlStreamReader28addExtraNamespaceDeclarationERK30QXmlStreamNamespaceDeclaration", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:431
// index:0
// Public Visibility=Default Availability=Available
// [16] QStringRef dtdName() const

/*

*/
impl /*struct*/ QXmlStreamReader {
  pub fn dtdName_0<RetType, T: QXmlStreamReader_dtdName_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dtdName_0(self);
    // return 1;
  }
}
pub trait QXmlStreamReader_dtdName_0<RetType> {
  fn dtdName_0(self , rsthis: & QXmlStreamReader) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamReader_dtdName_0<usize> for () {
  fn dtdName_0(self , rsthis: & QXmlStreamReader) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QXmlStreamReader7dtdNameEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:432
// index:0
// Public Visibility=Default Availability=Available
// [16] QStringRef dtdPublicId() const

/*

*/
impl /*struct*/ QXmlStreamReader {
  pub fn dtdPublicId_0<RetType, T: QXmlStreamReader_dtdPublicId_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dtdPublicId_0(self);
    // return 1;
  }
}
pub trait QXmlStreamReader_dtdPublicId_0<RetType> {
  fn dtdPublicId_0(self , rsthis: & QXmlStreamReader) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamReader_dtdPublicId_0<usize> for () {
  fn dtdPublicId_0(self , rsthis: & QXmlStreamReader) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QXmlStreamReader11dtdPublicIdEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:433
// index:0
// Public Visibility=Default Availability=Available
// [16] QStringRef dtdSystemId() const

/*

*/
impl /*struct*/ QXmlStreamReader {
  pub fn dtdSystemId_0<RetType, T: QXmlStreamReader_dtdSystemId_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dtdSystemId_0(self);
    // return 1;
  }
}
pub trait QXmlStreamReader_dtdSystemId_0<RetType> {
  fn dtdSystemId_0(self , rsthis: & QXmlStreamReader) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamReader_dtdSystemId_0<usize> for () {
  fn dtdSystemId_0(self , rsthis: & QXmlStreamReader) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QXmlStreamReader11dtdSystemIdEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:443
// index:0
// Public Visibility=Default Availability=Available
// [-2] void raiseError(const QString &)

/*

*/
impl /*struct*/ QXmlStreamReader {
  pub fn raiseError_0<RetType, T: QXmlStreamReader_raiseError_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.raiseError_0(self);
    // return 1;
  }
}
pub trait QXmlStreamReader_raiseError_0<RetType> {
  fn raiseError_0(self , rsthis: & QXmlStreamReader) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamReader_raiseError_0<(/*void*/)> for (usize) {
  fn raiseError_0(self , rsthis: & QXmlStreamReader) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN16QXmlStreamReader10raiseErrorERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:444
// index:0
// Public Visibility=Default Availability=Available
// [8] QString errorString() const

/*

*/
impl /*struct*/ QXmlStreamReader {
  pub fn errorString_0<RetType, T: QXmlStreamReader_errorString_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.errorString_0(self);
    // return 1;
  }
}
pub trait QXmlStreamReader_errorString_0<RetType> {
  fn errorString_0(self , rsthis: & QXmlStreamReader) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamReader_errorString_0<usize> for () {
  fn errorString_0(self , rsthis: & QXmlStreamReader) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QXmlStreamReader11errorStringEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:445
// index:0
// Public Visibility=Default Availability=Available
// [4] QXmlStreamReader::Error error() const

/*

*/
impl /*struct*/ QXmlStreamReader {
  pub fn error_0<RetType, T: QXmlStreamReader_error_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.error_0(self);
    // return 1;
  }
}
pub trait QXmlStreamReader_error_0<RetType> {
  fn error_0(self , rsthis: & QXmlStreamReader) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamReader_error_0<i32> for () {
  fn error_0(self , rsthis: & QXmlStreamReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QXmlStreamReader5errorEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:447
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool hasError() const

/*

*/
impl /*struct*/ QXmlStreamReader {
  pub fn hasError_0<RetType, T: QXmlStreamReader_hasError_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hasError_0(self);
    // return 1;
  }
}
pub trait QXmlStreamReader_hasError_0<RetType> {
  fn hasError_0(self , rsthis: & QXmlStreamReader) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamReader_hasError_0<bool> for () {
  fn hasError_0(self , rsthis: & QXmlStreamReader) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QXmlStreamReader8hasErrorEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:452
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setEntityResolver(QXmlStreamEntityResolver *)

/*

*/
impl /*struct*/ QXmlStreamReader {
  pub fn setEntityResolver_0<RetType, T: QXmlStreamReader_setEntityResolver_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setEntityResolver_0(self);
    // return 1;
  }
}
pub trait QXmlStreamReader_setEntityResolver_0<RetType> {
  fn setEntityResolver_0(self , rsthis: & QXmlStreamReader) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamReader_setEntityResolver_0<(/*void*/)> for (usize) {
  fn setEntityResolver_0(self , rsthis: & QXmlStreamReader) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN16QXmlStreamReader17setEntityResolverEP24QXmlStreamEntityResolver", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:453
// index:0
// Public Visibility=Default Availability=Available
// [8] QXmlStreamEntityResolver * entityResolver() const

/*

*/
impl /*struct*/ QXmlStreamReader {
  pub fn entityResolver_0<RetType, T: QXmlStreamReader_entityResolver_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.entityResolver_0(self);
    // return 1;
  }
}
pub trait QXmlStreamReader_entityResolver_0<RetType> {
  fn entityResolver_0(self , rsthis: & QXmlStreamReader) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamReader_entityResolver_0<usize> for () {
  fn entityResolver_0(self , rsthis: & QXmlStreamReader) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QXmlStreamReader14entityResolverEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}


/*


*/
pub type QXmlStreamReader__TokenType = i32;
// 
pub const QXmlStreamReader__NoToken :QXmlStreamReader__TokenType = 0;
// 
pub const QXmlStreamReader__Invalid :QXmlStreamReader__TokenType = 1;
// 
pub const QXmlStreamReader__StartDocument :QXmlStreamReader__TokenType = 2;
// 
pub const QXmlStreamReader__EndDocument :QXmlStreamReader__TokenType = 3;
// 
pub const QXmlStreamReader__StartElement :QXmlStreamReader__TokenType = 4;
// 
pub const QXmlStreamReader__EndElement :QXmlStreamReader__TokenType = 5;
// 
pub const QXmlStreamReader__Characters :QXmlStreamReader__TokenType = 6;
// 
pub const QXmlStreamReader__Comment :QXmlStreamReader__TokenType = 7;
// 
pub const QXmlStreamReader__DTD :QXmlStreamReader__TokenType = 8;
// 
pub const QXmlStreamReader__EntityReference :QXmlStreamReader__TokenType = 9;
// 
pub const QXmlStreamReader__ProcessingInstruction :QXmlStreamReader__TokenType = 10;
pub fn QXmlStreamReader_TokenTypeItemName(val: i32) ->String {
  match val {
     QXmlStreamReader__NoToken => // 0
     {return String::from("NoToken");}
     QXmlStreamReader__Invalid => // 1
     {return String::from("Invalid");}
     QXmlStreamReader__StartDocument => // 2
     {return String::from("StartDocument");}
     QXmlStreamReader__EndDocument => // 3
     {return String::from("EndDocument");}
     QXmlStreamReader__StartElement => // 4
     {return String::from("StartElement");}
     QXmlStreamReader__EndElement => // 5
     {return String::from("EndElement");}
     QXmlStreamReader__Characters => // 6
     {return String::from("Characters");}
     QXmlStreamReader__Comment => // 7
     {return String::from("Comment");}
     QXmlStreamReader__DTD => // 8
     {return String::from("DTD");}
     QXmlStreamReader__EntityReference => // 9
     {return String::from("EntityReference");}
     QXmlStreamReader__ProcessingInstruction => // 10
     {return String::from("ProcessingInstruction");}
  _ => {return format!("{}", val);}
}
}
pub fn QXmlStreamReader_TokenTypeItemName_s(val: i32) ->String {
  //var nilthis *QXmlStreamReader
  //return nilthis.TokenTypeItemName(val);
  return QXmlStreamReader_TokenTypeItemName(val);
}


/*


*/
pub type QXmlStreamReader__ReadElementTextBehaviour = i32;
// 
pub const QXmlStreamReader__ErrorOnUnexpectedElement :QXmlStreamReader__ReadElementTextBehaviour = 0;
// 
pub const QXmlStreamReader__IncludeChildElements :QXmlStreamReader__ReadElementTextBehaviour = 1;
// 
pub const QXmlStreamReader__SkipChildElements :QXmlStreamReader__ReadElementTextBehaviour = 2;
pub fn QXmlStreamReader_ReadElementTextBehaviourItemName(val: i32) ->String {
  match val {
     QXmlStreamReader__ErrorOnUnexpectedElement => // 0
     {return String::from("ErrorOnUnexpectedElement");}
     QXmlStreamReader__IncludeChildElements => // 1
     {return String::from("IncludeChildElements");}
     QXmlStreamReader__SkipChildElements => // 2
     {return String::from("SkipChildElements");}
  _ => {return format!("{}", val);}
}
}
pub fn QXmlStreamReader_ReadElementTextBehaviourItemName_s(val: i32) ->String {
  //var nilthis *QXmlStreamReader
  //return nilthis.ReadElementTextBehaviourItemName(val);
  return QXmlStreamReader_ReadElementTextBehaviourItemName(val);
}


/*


*/
pub type QXmlStreamReader__Error = i32;
// 
pub const QXmlStreamReader__NoError :QXmlStreamReader__Error = 0;
// 
pub const QXmlStreamReader__UnexpectedElementError :QXmlStreamReader__Error = 1;
// 
pub const QXmlStreamReader__CustomError :QXmlStreamReader__Error = 2;
// 
pub const QXmlStreamReader__NotWellFormedError :QXmlStreamReader__Error = 3;
// 
pub const QXmlStreamReader__PrematureEndOfDocumentError :QXmlStreamReader__Error = 4;
pub fn QXmlStreamReader_ErrorItemName(val: i32) ->String {
  match val {
     QXmlStreamReader__NoError => // 0
     {return String::from("NoError");}
     QXmlStreamReader__UnexpectedElementError => // 1
     {return String::from("UnexpectedElementError");}
     QXmlStreamReader__CustomError => // 2
     {return String::from("CustomError");}
     QXmlStreamReader__NotWellFormedError => // 3
     {return String::from("NotWellFormedError");}
     QXmlStreamReader__PrematureEndOfDocumentError => // 4
     {return String::from("PrematureEndOfDocumentError");}
  _ => {return format!("{}", val);}
}
}
pub fn QXmlStreamReader_ErrorItemName_s(val: i32) ->String {
  //var nilthis *QXmlStreamReader
  //return nilthis.ErrorItemName(val);
  return QXmlStreamReader_ErrorItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
