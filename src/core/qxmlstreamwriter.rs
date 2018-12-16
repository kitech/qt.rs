

// mod ::core::QXmlStreamWriter
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
// extern C begin: 56
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
#[derive(Default)] // class sizeof(QXmlStreamWriter)=8
pub struct QXmlStreamWriter {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QXmlStreamWriter_ITF interface {
//    QXmlStreamWriter_PTR() *QXmlStreamWriter
//}
//func (ptr *QXmlStreamWriter) QXmlStreamWriter_PTR() *QXmlStreamWriter { return ptr }

impl /*struct*/ QXmlStreamWriter {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QXmlStreamWriter {
    return QXmlStreamWriter{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QXmlStreamWriter {
//  type Target = QXmlStreamWriterBASE;
//
//  fn deref(&self) -> &QXmlStreamWriterBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QXmlStreamWriterBASE> for QXmlStreamWriter {
//  fn as_ref(& self) -> & QXmlStreamWriterBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qxmlstream.h:472
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QXmlStreamWriter()

/*

*/
// QXmlStreamWriter() ctx.fn_proto_cpp
impl /*struct*/ QXmlStreamWriter {
  pub fn QXmlStreamWriter_0<T: QXmlStreamWriter_QXmlStreamWriter_0>(value: T) -> QXmlStreamWriter {
    let rsthis = value.QXmlStreamWriter_0();
    return rsthis;
    // return 1;
  }
}

pub trait QXmlStreamWriter_QXmlStreamWriter_0 {
  fn QXmlStreamWriter_0(self) -> QXmlStreamWriter;
}
// QXmlStreamWriter() ctx.fn_proto_cpp
impl<'a> /*trait*/ QXmlStreamWriter_QXmlStreamWriter_0 for () {
  fn QXmlStreamWriter_0(self) -> QXmlStreamWriter {
    // unsafe{_ZN16QXmlStreamWriterC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN16QXmlStreamWriterC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QXmlStreamWriter{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:473
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QXmlStreamWriter(QIODevice *)

/*

*/
// QXmlStreamWriter(QIODevice *) ctx.fn_proto_cpp
impl /*struct*/ QXmlStreamWriter {
  pub fn QXmlStreamWriter_1<T: QXmlStreamWriter_QXmlStreamWriter_1>(value: T) -> QXmlStreamWriter {
    let rsthis = value.QXmlStreamWriter_1();
    return rsthis;
    // return 1;
  }
}

pub trait QXmlStreamWriter_QXmlStreamWriter_1 {
  fn QXmlStreamWriter_1(self) -> QXmlStreamWriter;
}
// QXmlStreamWriter(QIODevice *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QXmlStreamWriter_QXmlStreamWriter_1 for (usize) {
  fn QXmlStreamWriter_1(self) -> QXmlStreamWriter {
    // unsafe{_ZN16QXmlStreamWriterC2EP9QIODevice()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN16QXmlStreamWriterC2EP9QIODevice", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QXmlStreamWriter{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:474
// index:2
// Public Visibility=Default Availability=Available
// [-2] void QXmlStreamWriter(QByteArray *)

/*

*/
// QXmlStreamWriter(QByteArray *) ctx.fn_proto_cpp
impl /*struct*/ QXmlStreamWriter {
  pub fn QXmlStreamWriter_2<T: QXmlStreamWriter_QXmlStreamWriter_2>(value: T) -> QXmlStreamWriter {
    let rsthis = value.QXmlStreamWriter_2();
    return rsthis;
    // return 1;
  }
}

pub trait QXmlStreamWriter_QXmlStreamWriter_2 {
  fn QXmlStreamWriter_2(self) -> QXmlStreamWriter;
}
// QXmlStreamWriter(QByteArray *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QXmlStreamWriter_QXmlStreamWriter_2 for (usize) {
  fn QXmlStreamWriter_2(self) -> QXmlStreamWriter {
    // unsafe{_ZN16QXmlStreamWriterC2EP10QByteArray()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN16QXmlStreamWriterC2EP10QByteArray", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QXmlStreamWriter{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:475
// index:3
// Public Visibility=Default Availability=Available
// [-2] void QXmlStreamWriter(QString *)

/*

*/
// QXmlStreamWriter(QString *) ctx.fn_proto_cpp
impl /*struct*/ QXmlStreamWriter {
  pub fn QXmlStreamWriter_3<T: QXmlStreamWriter_QXmlStreamWriter_3>(value: T) -> QXmlStreamWriter {
    let rsthis = value.QXmlStreamWriter_3();
    return rsthis;
    // return 1;
  }
}

pub trait QXmlStreamWriter_QXmlStreamWriter_3 {
  fn QXmlStreamWriter_3(self) -> QXmlStreamWriter;
}
// QXmlStreamWriter(QString *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QXmlStreamWriter_QXmlStreamWriter_3 for (usize) {
  fn QXmlStreamWriter_3(self) -> QXmlStreamWriter {
    // unsafe{_ZN16QXmlStreamWriterC2EP7QString()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN16QXmlStreamWriterC2EP7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QXmlStreamWriter{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:476
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ~QXmlStreamWriter()

/*

*/
pub fn DeleteQXmlStreamWriter(this :*mut QXmlStreamWriter) {
    // let rv = qtrt::InvokeQtFunc6("_ZN16QXmlStreamWriterD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 8)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qxmlstream.h:478
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDevice(QIODevice *)

/*

*/
impl /*struct*/ QXmlStreamWriter {
  pub fn setDevice_0<RetType, T: QXmlStreamWriter_setDevice_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDevice_0(self);
    // return 1;
  }
}
pub trait QXmlStreamWriter_setDevice_0<RetType> {
  fn setDevice_0(self , rsthis: & QXmlStreamWriter) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamWriter_setDevice_0<(/*void*/)> for (usize) {
  fn setDevice_0(self , rsthis: & QXmlStreamWriter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN16QXmlStreamWriter9setDeviceEP9QIODevice", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:479
// index:0
// Public Visibility=Default Availability=Available
// [8] QIODevice * device() const

/*

*/
impl /*struct*/ QXmlStreamWriter {
  pub fn device_0<RetType, T: QXmlStreamWriter_device_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.device_0(self);
    // return 1;
  }
}
pub trait QXmlStreamWriter_device_0<RetType> {
  fn device_0(self , rsthis: & QXmlStreamWriter) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamWriter_device_0<usize> for () {
  fn device_0(self , rsthis: & QXmlStreamWriter) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QXmlStreamWriter6deviceEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:482
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCodec(QTextCodec *)

/*

*/
impl /*struct*/ QXmlStreamWriter {
  pub fn setCodec_0<RetType, T: QXmlStreamWriter_setCodec_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCodec_0(self);
    // return 1;
  }
}
pub trait QXmlStreamWriter_setCodec_0<RetType> {
  fn setCodec_0(self , rsthis: & QXmlStreamWriter) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamWriter_setCodec_0<(/*void*/)> for (usize) {
  fn setCodec_0(self , rsthis: & QXmlStreamWriter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN16QXmlStreamWriter8setCodecEP10QTextCodec", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:483
// index:1
// Public Visibility=Default Availability=Available
// [-2] void setCodec(const char *)

/*

*/
impl /*struct*/ QXmlStreamWriter {
  pub fn setCodec_1<RetType, T: QXmlStreamWriter_setCodec_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCodec_1(self);
    // return 1;
  }
}
pub trait QXmlStreamWriter_setCodec_1<RetType> {
  fn setCodec_1(self , rsthis: & QXmlStreamWriter) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamWriter_setCodec_1<(/*void*/)> for (usize) {
  fn setCodec_1(self , rsthis: & QXmlStreamWriter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN16QXmlStreamWriter8setCodecEPKc", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:484
// index:0
// Public Visibility=Default Availability=Available
// [8] QTextCodec * codec() const

/*

*/
impl /*struct*/ QXmlStreamWriter {
  pub fn codec_0<RetType, T: QXmlStreamWriter_codec_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.codec_0(self);
    // return 1;
  }
}
pub trait QXmlStreamWriter_codec_0<RetType> {
  fn codec_0(self , rsthis: & QXmlStreamWriter) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamWriter_codec_0<usize> for () {
  fn codec_0(self , rsthis: & QXmlStreamWriter) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QXmlStreamWriter5codecEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:487
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setAutoFormatting(bool)

/*

*/
impl /*struct*/ QXmlStreamWriter {
  pub fn setAutoFormatting_0<RetType, T: QXmlStreamWriter_setAutoFormatting_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setAutoFormatting_0(self);
    // return 1;
  }
}
pub trait QXmlStreamWriter_setAutoFormatting_0<RetType> {
  fn setAutoFormatting_0(self , rsthis: & QXmlStreamWriter) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamWriter_setAutoFormatting_0<(/*void*/)> for (bool) {
  fn setAutoFormatting_0(self , rsthis: & QXmlStreamWriter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN16QXmlStreamWriter17setAutoFormattingEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:488
// index:0
// Public Visibility=Default Availability=Available
// [1] bool autoFormatting() const

/*

*/
impl /*struct*/ QXmlStreamWriter {
  pub fn autoFormatting_0<RetType, T: QXmlStreamWriter_autoFormatting_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.autoFormatting_0(self);
    // return 1;
  }
}
pub trait QXmlStreamWriter_autoFormatting_0<RetType> {
  fn autoFormatting_0(self , rsthis: & QXmlStreamWriter) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamWriter_autoFormatting_0<bool> for () {
  fn autoFormatting_0(self , rsthis: & QXmlStreamWriter) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QXmlStreamWriter14autoFormattingEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:490
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setAutoFormattingIndent(int)

/*

*/
impl /*struct*/ QXmlStreamWriter {
  pub fn setAutoFormattingIndent_0<RetType, T: QXmlStreamWriter_setAutoFormattingIndent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setAutoFormattingIndent_0(self);
    // return 1;
  }
}
pub trait QXmlStreamWriter_setAutoFormattingIndent_0<RetType> {
  fn setAutoFormattingIndent_0(self , rsthis: & QXmlStreamWriter) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamWriter_setAutoFormattingIndent_0<(/*void*/)> for (i32) {
  fn setAutoFormattingIndent_0(self , rsthis: & QXmlStreamWriter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN16QXmlStreamWriter23setAutoFormattingIndentEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:491
// index:0
// Public Visibility=Default Availability=Available
// [4] int autoFormattingIndent() const

/*

*/
impl /*struct*/ QXmlStreamWriter {
  pub fn autoFormattingIndent_0<RetType, T: QXmlStreamWriter_autoFormattingIndent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.autoFormattingIndent_0(self);
    // return 1;
  }
}
pub trait QXmlStreamWriter_autoFormattingIndent_0<RetType> {
  fn autoFormattingIndent_0(self , rsthis: & QXmlStreamWriter) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamWriter_autoFormattingIndent_0<i32> for () {
  fn autoFormattingIndent_0(self , rsthis: & QXmlStreamWriter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QXmlStreamWriter20autoFormattingIndentEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:493
// index:0
// Public Visibility=Default Availability=Available
// [-2] void writeAttribute(const QString &, const QString &)

/*

*/
impl /*struct*/ QXmlStreamWriter {
  pub fn writeAttribute_0<RetType, T: QXmlStreamWriter_writeAttribute_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.writeAttribute_0(self);
    // return 1;
  }
}
pub trait QXmlStreamWriter_writeAttribute_0<RetType> {
  fn writeAttribute_0(self , rsthis: & QXmlStreamWriter) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamWriter_writeAttribute_0<(/*void*/)> for (usize,usize) {
  fn writeAttribute_0(self , rsthis: & QXmlStreamWriter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN16QXmlStreamWriter14writeAttributeERK7QStringS2_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:494
// index:1
// Public Visibility=Default Availability=Available
// [-2] void writeAttribute(const QString &, const QString &, const QString &)

/*

*/
impl /*struct*/ QXmlStreamWriter {
  pub fn writeAttribute_1<RetType, T: QXmlStreamWriter_writeAttribute_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.writeAttribute_1(self);
    // return 1;
  }
}
pub trait QXmlStreamWriter_writeAttribute_1<RetType> {
  fn writeAttribute_1(self , rsthis: & QXmlStreamWriter) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamWriter_writeAttribute_1<(/*void*/)> for (usize,usize,usize) {
  fn writeAttribute_1(self , rsthis: & QXmlStreamWriter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN16QXmlStreamWriter14writeAttributeERK7QStringS2_S2_", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:495
// index:2
// Public Visibility=Default Availability=Available
// [-2] void writeAttribute(const QXmlStreamAttribute &)

/*

*/
impl /*struct*/ QXmlStreamWriter {
  pub fn writeAttribute_2<RetType, T: QXmlStreamWriter_writeAttribute_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.writeAttribute_2(self);
    // return 1;
  }
}
pub trait QXmlStreamWriter_writeAttribute_2<RetType> {
  fn writeAttribute_2(self , rsthis: & QXmlStreamWriter) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamWriter_writeAttribute_2<(/*void*/)> for (usize) {
  fn writeAttribute_2(self , rsthis: & QXmlStreamWriter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN16QXmlStreamWriter14writeAttributeERK19QXmlStreamAttribute", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:496
// index:0
// Public Visibility=Default Availability=Available
// [-2] void writeAttributes(const QXmlStreamAttributes &)

/*

*/
impl /*struct*/ QXmlStreamWriter {
  pub fn writeAttributes_0<RetType, T: QXmlStreamWriter_writeAttributes_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.writeAttributes_0(self);
    // return 1;
  }
}
pub trait QXmlStreamWriter_writeAttributes_0<RetType> {
  fn writeAttributes_0(self , rsthis: & QXmlStreamWriter) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamWriter_writeAttributes_0<(/*void*/)> for (usize) {
  fn writeAttributes_0(self , rsthis: & QXmlStreamWriter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN16QXmlStreamWriter15writeAttributesERK20QXmlStreamAttributes", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:498
// index:0
// Public Visibility=Default Availability=Available
// [-2] void writeCDATA(const QString &)

/*

*/
impl /*struct*/ QXmlStreamWriter {
  pub fn writeCDATA_0<RetType, T: QXmlStreamWriter_writeCDATA_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.writeCDATA_0(self);
    // return 1;
  }
}
pub trait QXmlStreamWriter_writeCDATA_0<RetType> {
  fn writeCDATA_0(self , rsthis: & QXmlStreamWriter) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamWriter_writeCDATA_0<(/*void*/)> for (usize) {
  fn writeCDATA_0(self , rsthis: & QXmlStreamWriter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN16QXmlStreamWriter10writeCDATAERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:499
// index:0
// Public Visibility=Default Availability=Available
// [-2] void writeCharacters(const QString &)

/*

*/
impl /*struct*/ QXmlStreamWriter {
  pub fn writeCharacters_0<RetType, T: QXmlStreamWriter_writeCharacters_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.writeCharacters_0(self);
    // return 1;
  }
}
pub trait QXmlStreamWriter_writeCharacters_0<RetType> {
  fn writeCharacters_0(self , rsthis: & QXmlStreamWriter) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamWriter_writeCharacters_0<(/*void*/)> for (usize) {
  fn writeCharacters_0(self , rsthis: & QXmlStreamWriter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN16QXmlStreamWriter15writeCharactersERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:500
// index:0
// Public Visibility=Default Availability=Available
// [-2] void writeComment(const QString &)

/*

*/
impl /*struct*/ QXmlStreamWriter {
  pub fn writeComment_0<RetType, T: QXmlStreamWriter_writeComment_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.writeComment_0(self);
    // return 1;
  }
}
pub trait QXmlStreamWriter_writeComment_0<RetType> {
  fn writeComment_0(self , rsthis: & QXmlStreamWriter) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamWriter_writeComment_0<(/*void*/)> for (usize) {
  fn writeComment_0(self , rsthis: & QXmlStreamWriter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN16QXmlStreamWriter12writeCommentERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:502
// index:0
// Public Visibility=Default Availability=Available
// [-2] void writeDTD(const QString &)

/*

*/
impl /*struct*/ QXmlStreamWriter {
  pub fn writeDTD_0<RetType, T: QXmlStreamWriter_writeDTD_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.writeDTD_0(self);
    // return 1;
  }
}
pub trait QXmlStreamWriter_writeDTD_0<RetType> {
  fn writeDTD_0(self , rsthis: & QXmlStreamWriter) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamWriter_writeDTD_0<(/*void*/)> for (usize) {
  fn writeDTD_0(self , rsthis: & QXmlStreamWriter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN16QXmlStreamWriter8writeDTDERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:504
// index:0
// Public Visibility=Default Availability=Available
// [-2] void writeEmptyElement(const QString &)

/*

*/
impl /*struct*/ QXmlStreamWriter {
  pub fn writeEmptyElement_0<RetType, T: QXmlStreamWriter_writeEmptyElement_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.writeEmptyElement_0(self);
    // return 1;
  }
}
pub trait QXmlStreamWriter_writeEmptyElement_0<RetType> {
  fn writeEmptyElement_0(self , rsthis: & QXmlStreamWriter) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamWriter_writeEmptyElement_0<(/*void*/)> for (usize) {
  fn writeEmptyElement_0(self , rsthis: & QXmlStreamWriter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN16QXmlStreamWriter17writeEmptyElementERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:505
// index:1
// Public Visibility=Default Availability=Available
// [-2] void writeEmptyElement(const QString &, const QString &)

/*

*/
impl /*struct*/ QXmlStreamWriter {
  pub fn writeEmptyElement_1<RetType, T: QXmlStreamWriter_writeEmptyElement_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.writeEmptyElement_1(self);
    // return 1;
  }
}
pub trait QXmlStreamWriter_writeEmptyElement_1<RetType> {
  fn writeEmptyElement_1(self , rsthis: & QXmlStreamWriter) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamWriter_writeEmptyElement_1<(/*void*/)> for (usize,usize) {
  fn writeEmptyElement_1(self , rsthis: & QXmlStreamWriter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN16QXmlStreamWriter17writeEmptyElementERK7QStringS2_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:507
// index:0
// Public Visibility=Default Availability=Available
// [-2] void writeTextElement(const QString &, const QString &)

/*

*/
impl /*struct*/ QXmlStreamWriter {
  pub fn writeTextElement_0<RetType, T: QXmlStreamWriter_writeTextElement_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.writeTextElement_0(self);
    // return 1;
  }
}
pub trait QXmlStreamWriter_writeTextElement_0<RetType> {
  fn writeTextElement_0(self , rsthis: & QXmlStreamWriter) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamWriter_writeTextElement_0<(/*void*/)> for (usize,usize) {
  fn writeTextElement_0(self , rsthis: & QXmlStreamWriter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN16QXmlStreamWriter16writeTextElementERK7QStringS2_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:508
// index:1
// Public Visibility=Default Availability=Available
// [-2] void writeTextElement(const QString &, const QString &, const QString &)

/*

*/
impl /*struct*/ QXmlStreamWriter {
  pub fn writeTextElement_1<RetType, T: QXmlStreamWriter_writeTextElement_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.writeTextElement_1(self);
    // return 1;
  }
}
pub trait QXmlStreamWriter_writeTextElement_1<RetType> {
  fn writeTextElement_1(self , rsthis: & QXmlStreamWriter) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamWriter_writeTextElement_1<(/*void*/)> for (usize,usize,usize) {
  fn writeTextElement_1(self , rsthis: & QXmlStreamWriter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN16QXmlStreamWriter16writeTextElementERK7QStringS2_S2_", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:510
// index:0
// Public Visibility=Default Availability=Available
// [-2] void writeEndDocument()

/*

*/
impl /*struct*/ QXmlStreamWriter {
  pub fn writeEndDocument_0<RetType, T: QXmlStreamWriter_writeEndDocument_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.writeEndDocument_0(self);
    // return 1;
  }
}
pub trait QXmlStreamWriter_writeEndDocument_0<RetType> {
  fn writeEndDocument_0(self , rsthis: & QXmlStreamWriter) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamWriter_writeEndDocument_0<(/*void*/)> for () {
  fn writeEndDocument_0(self , rsthis: & QXmlStreamWriter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN16QXmlStreamWriter16writeEndDocumentEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:511
// index:0
// Public Visibility=Default Availability=Available
// [-2] void writeEndElement()

/*

*/
impl /*struct*/ QXmlStreamWriter {
  pub fn writeEndElement_0<RetType, T: QXmlStreamWriter_writeEndElement_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.writeEndElement_0(self);
    // return 1;
  }
}
pub trait QXmlStreamWriter_writeEndElement_0<RetType> {
  fn writeEndElement_0(self , rsthis: & QXmlStreamWriter) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamWriter_writeEndElement_0<(/*void*/)> for () {
  fn writeEndElement_0(self , rsthis: & QXmlStreamWriter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN16QXmlStreamWriter15writeEndElementEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:513
// index:0
// Public Visibility=Default Availability=Available
// [-2] void writeEntityReference(const QString &)

/*

*/
impl /*struct*/ QXmlStreamWriter {
  pub fn writeEntityReference_0<RetType, T: QXmlStreamWriter_writeEntityReference_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.writeEntityReference_0(self);
    // return 1;
  }
}
pub trait QXmlStreamWriter_writeEntityReference_0<RetType> {
  fn writeEntityReference_0(self , rsthis: & QXmlStreamWriter) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamWriter_writeEntityReference_0<(/*void*/)> for (usize) {
  fn writeEntityReference_0(self , rsthis: & QXmlStreamWriter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN16QXmlStreamWriter20writeEntityReferenceERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:514
// index:0
// Public Visibility=Default Availability=Available
// [-2] void writeNamespace(const QString &, const QString &)

/*

*/
impl /*struct*/ QXmlStreamWriter {
  pub fn writeNamespace_0<RetType, T: QXmlStreamWriter_writeNamespace_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.writeNamespace_0(self);
    // return 1;
  }
}
pub trait QXmlStreamWriter_writeNamespace_0<RetType> {
  fn writeNamespace_0(self , rsthis: & QXmlStreamWriter) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamWriter_writeNamespace_0<(/*void*/)> for (usize,usize) {
  fn writeNamespace_0(self , rsthis: & QXmlStreamWriter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN16QXmlStreamWriter14writeNamespaceERK7QStringS2_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:515
// index:0
// Public Visibility=Default Availability=Available
// [-2] void writeDefaultNamespace(const QString &)

/*

*/
impl /*struct*/ QXmlStreamWriter {
  pub fn writeDefaultNamespace_0<RetType, T: QXmlStreamWriter_writeDefaultNamespace_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.writeDefaultNamespace_0(self);
    // return 1;
  }
}
pub trait QXmlStreamWriter_writeDefaultNamespace_0<RetType> {
  fn writeDefaultNamespace_0(self , rsthis: & QXmlStreamWriter) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamWriter_writeDefaultNamespace_0<(/*void*/)> for (usize) {
  fn writeDefaultNamespace_0(self , rsthis: & QXmlStreamWriter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN16QXmlStreamWriter21writeDefaultNamespaceERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:516
// index:0
// Public Visibility=Default Availability=Available
// [-2] void writeProcessingInstruction(const QString &, const QString &)

/*

*/
impl /*struct*/ QXmlStreamWriter {
  pub fn writeProcessingInstruction_0<RetType, T: QXmlStreamWriter_writeProcessingInstruction_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.writeProcessingInstruction_0(self);
    // return 1;
  }
}
pub trait QXmlStreamWriter_writeProcessingInstruction_0<RetType> {
  fn writeProcessingInstruction_0(self , rsthis: & QXmlStreamWriter) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamWriter_writeProcessingInstruction_0<(/*void*/)> for (usize,usize) {
  fn writeProcessingInstruction_0(self , rsthis: & QXmlStreamWriter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN16QXmlStreamWriter26writeProcessingInstructionERK7QStringS2_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:518
// index:0
// Public Visibility=Default Availability=Available
// [-2] void writeStartDocument()

/*

*/
impl /*struct*/ QXmlStreamWriter {
  pub fn writeStartDocument_0<RetType, T: QXmlStreamWriter_writeStartDocument_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.writeStartDocument_0(self);
    // return 1;
  }
}
pub trait QXmlStreamWriter_writeStartDocument_0<RetType> {
  fn writeStartDocument_0(self , rsthis: & QXmlStreamWriter) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamWriter_writeStartDocument_0<(/*void*/)> for () {
  fn writeStartDocument_0(self , rsthis: & QXmlStreamWriter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN16QXmlStreamWriter18writeStartDocumentEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:519
// index:1
// Public Visibility=Default Availability=Available
// [-2] void writeStartDocument(const QString &)

/*

*/
impl /*struct*/ QXmlStreamWriter {
  pub fn writeStartDocument_1<RetType, T: QXmlStreamWriter_writeStartDocument_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.writeStartDocument_1(self);
    // return 1;
  }
}
pub trait QXmlStreamWriter_writeStartDocument_1<RetType> {
  fn writeStartDocument_1(self , rsthis: & QXmlStreamWriter) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamWriter_writeStartDocument_1<(/*void*/)> for (usize) {
  fn writeStartDocument_1(self , rsthis: & QXmlStreamWriter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN16QXmlStreamWriter18writeStartDocumentERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:520
// index:2
// Public Visibility=Default Availability=Available
// [-2] void writeStartDocument(const QString &, bool)

/*

*/
impl /*struct*/ QXmlStreamWriter {
  pub fn writeStartDocument_2<RetType, T: QXmlStreamWriter_writeStartDocument_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.writeStartDocument_2(self);
    // return 1;
  }
}
pub trait QXmlStreamWriter_writeStartDocument_2<RetType> {
  fn writeStartDocument_2(self , rsthis: & QXmlStreamWriter) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamWriter_writeStartDocument_2<(/*void*/)> for (usize,bool) {
  fn writeStartDocument_2(self , rsthis: & QXmlStreamWriter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN16QXmlStreamWriter18writeStartDocumentERK7QStringb", 2,qtrt::FFITY_POINTER,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:521
// index:0
// Public Visibility=Default Availability=Available
// [-2] void writeStartElement(const QString &)

/*

*/
impl /*struct*/ QXmlStreamWriter {
  pub fn writeStartElement_0<RetType, T: QXmlStreamWriter_writeStartElement_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.writeStartElement_0(self);
    // return 1;
  }
}
pub trait QXmlStreamWriter_writeStartElement_0<RetType> {
  fn writeStartElement_0(self , rsthis: & QXmlStreamWriter) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamWriter_writeStartElement_0<(/*void*/)> for (usize) {
  fn writeStartElement_0(self , rsthis: & QXmlStreamWriter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN16QXmlStreamWriter17writeStartElementERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:522
// index:1
// Public Visibility=Default Availability=Available
// [-2] void writeStartElement(const QString &, const QString &)

/*

*/
impl /*struct*/ QXmlStreamWriter {
  pub fn writeStartElement_1<RetType, T: QXmlStreamWriter_writeStartElement_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.writeStartElement_1(self);
    // return 1;
  }
}
pub trait QXmlStreamWriter_writeStartElement_1<RetType> {
  fn writeStartElement_1(self , rsthis: & QXmlStreamWriter) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamWriter_writeStartElement_1<(/*void*/)> for (usize,usize) {
  fn writeStartElement_1(self , rsthis: & QXmlStreamWriter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN16QXmlStreamWriter17writeStartElementERK7QStringS2_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:525
// index:0
// Public Visibility=Default Availability=Available
// [-2] void writeCurrentToken(const QXmlStreamReader &)

/*

*/
impl /*struct*/ QXmlStreamWriter {
  pub fn writeCurrentToken_0<RetType, T: QXmlStreamWriter_writeCurrentToken_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.writeCurrentToken_0(self);
    // return 1;
  }
}
pub trait QXmlStreamWriter_writeCurrentToken_0<RetType> {
  fn writeCurrentToken_0(self , rsthis: & QXmlStreamWriter) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamWriter_writeCurrentToken_0<(/*void*/)> for (usize) {
  fn writeCurrentToken_0(self , rsthis: & QXmlStreamWriter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN16QXmlStreamWriter17writeCurrentTokenERK16QXmlStreamReader", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qxmlstream.h:528
// index:0
// Public Visibility=Default Availability=Available
// [1] bool hasError() const

/*

*/
impl /*struct*/ QXmlStreamWriter {
  pub fn hasError_0<RetType, T: QXmlStreamWriter_hasError_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hasError_0(self);
    // return 1;
  }
}
pub trait QXmlStreamWriter_hasError_0<RetType> {
  fn hasError_0(self , rsthis: & QXmlStreamWriter) -> RetType;
}
impl<'a> /*trait*/ QXmlStreamWriter_hasError_0<bool> for () {
  fn hasError_0(self , rsthis: & QXmlStreamWriter) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QXmlStreamWriter8hasErrorEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
