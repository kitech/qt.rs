

// mod ::gui::QTextDocumentWriter
// package qtgui
// /usr/include/qt/QtGui/qtextdocumentwriter.h
// #include <qtextdocumentwriter.h>
// #include <QtGui>

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
// import "github.com/kitech/qt.go/qtcore"
use qtcore::*; // super::super::%!s(MISSING)::*;
use std::default::Default;
use std::ops::Deref;
use qtrt; // super::super::qtrt;
use super::*;
//  ext block end

//  body block begin



/*

*/
#[derive(Default)] // class sizeof(QTextDocumentWriter)=8
pub struct QTextDocumentWriter {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QTextDocumentWriter_ITF interface {
//    QTextDocumentWriter_PTR() *QTextDocumentWriter
//}
//func (ptr *QTextDocumentWriter) QTextDocumentWriter_PTR() *QTextDocumentWriter { return ptr }

impl /*struct*/ QTextDocumentWriter {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QTextDocumentWriter {
    return QTextDocumentWriter{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QTextDocumentWriter {
//  type Target = QTextDocumentWriterBASE;
//
//  fn deref(&self) -> &QTextDocumentWriterBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QTextDocumentWriterBASE> for QTextDocumentWriter {
//  fn as_ref(& self) -> & QTextDocumentWriterBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qtextdocumentwriter.h:57
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QTextDocumentWriter()

/*
Constructs an empty QTextDocumentWriter object. Before writing, you must call setFormat() to set a document format, then setDevice() or setFileName().
*/
// QTextDocumentWriter() ctx.fn_proto_cpp
impl /*struct*/ QTextDocumentWriter {
  pub fn QTextDocumentWriter_0<T: QTextDocumentWriter_QTextDocumentWriter_0>(value: T) -> QTextDocumentWriter {
    let rsthis = value.QTextDocumentWriter_0();
    return rsthis;
    // return 1;
  }
}

pub trait QTextDocumentWriter_QTextDocumentWriter_0 {
  fn QTextDocumentWriter_0(self) -> QTextDocumentWriter;
}
// QTextDocumentWriter() ctx.fn_proto_cpp
impl<'a> /*trait*/ QTextDocumentWriter_QTextDocumentWriter_0 for () {
  fn QTextDocumentWriter_0(self) -> QTextDocumentWriter {
    // unsafe{_ZN19QTextDocumentWriterC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN19QTextDocumentWriterC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTextDocumentWriter{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextdocumentwriter.h:58
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QTextDocumentWriter(QIODevice *, const QByteArray &)

/*
Constructs an empty QTextDocumentWriter object. Before writing, you must call setFormat() to set a document format, then setDevice() or setFileName().
*/
// QTextDocumentWriter(QIODevice *, const QByteArray &) ctx.fn_proto_cpp
impl /*struct*/ QTextDocumentWriter {
  pub fn QTextDocumentWriter_1<T: QTextDocumentWriter_QTextDocumentWriter_1>(value: T) -> QTextDocumentWriter {
    let rsthis = value.QTextDocumentWriter_1();
    return rsthis;
    // return 1;
  }
}

pub trait QTextDocumentWriter_QTextDocumentWriter_1 {
  fn QTextDocumentWriter_1(self) -> QTextDocumentWriter;
}
// QTextDocumentWriter(QIODevice *, const QByteArray &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QTextDocumentWriter_QTextDocumentWriter_1 for (usize,usize) {
  fn QTextDocumentWriter_1(self) -> QTextDocumentWriter {
    // unsafe{_ZN19QTextDocumentWriterC2EP9QIODeviceRK10QByteArray()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN19QTextDocumentWriterC2EP9QIODeviceRK10QByteArray", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTextDocumentWriter{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextdocumentwriter.h:59
// index:2
// Public Visibility=Default Availability=Available
// [-2] void QTextDocumentWriter(const QString &, const QByteArray &)

/*
Constructs an empty QTextDocumentWriter object. Before writing, you must call setFormat() to set a document format, then setDevice() or setFileName().
*/
// QTextDocumentWriter(const QString &, const QByteArray &) ctx.fn_proto_cpp
impl /*struct*/ QTextDocumentWriter {
  pub fn QTextDocumentWriter_2<T: QTextDocumentWriter_QTextDocumentWriter_2>(value: T) -> QTextDocumentWriter {
    let rsthis = value.QTextDocumentWriter_2();
    return rsthis;
    // return 1;
  }
}

pub trait QTextDocumentWriter_QTextDocumentWriter_2 {
  fn QTextDocumentWriter_2(self) -> QTextDocumentWriter;
}
// QTextDocumentWriter(const QString &, const QByteArray &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QTextDocumentWriter_QTextDocumentWriter_2 for (usize,usize) {
  fn QTextDocumentWriter_2(self) -> QTextDocumentWriter {
    // unsafe{_ZN19QTextDocumentWriterC2ERK7QStringRK10QByteArray()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN19QTextDocumentWriterC2ERK7QStringRK10QByteArray", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTextDocumentWriter{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextdocumentwriter.h:60
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ~QTextDocumentWriter()

/*

*/
pub fn DeleteQTextDocumentWriter(this :*mut QTextDocumentWriter) {
    // let rv = qtrt::InvokeQtFunc6("_ZN19QTextDocumentWriterD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 8)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qtextdocumentwriter.h:62
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFormat(const QByteArray &)

/*
Sets the format used to write documents to the format specified. format is a case insensitive text string. For example:


          QTextDocumentWriter writer;
          writer.setFormat("odf"); // same as writer.setFormat("ODF");



You can call supportedDocumentFormats() for the full list of formats QTextDocumentWriter supports.

See also format().
*/
impl /*struct*/ QTextDocumentWriter {
  pub fn setFormat_0<RetType, T: QTextDocumentWriter_setFormat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFormat_0(self);
    // return 1;
  }
}
pub trait QTextDocumentWriter_setFormat_0<RetType> {
  fn setFormat_0(self , rsthis: & QTextDocumentWriter) -> RetType;
}
impl<'a> /*trait*/ QTextDocumentWriter_setFormat_0<(/*void*/)> for (usize) {
  fn setFormat_0(self , rsthis: & QTextDocumentWriter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN19QTextDocumentWriter9setFormatERK10QByteArray", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextdocumentwriter.h:63
// index:0
// Public Visibility=Default Availability=Available
// [8] QByteArray format() const

/*
Returns the format used for writing documents.

See also setFormat().
*/
impl /*struct*/ QTextDocumentWriter {
  pub fn format_0<RetType, T: QTextDocumentWriter_format_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.format_0(self);
    // return 1;
  }
}
pub trait QTextDocumentWriter_format_0<RetType> {
  fn format_0(self , rsthis: & QTextDocumentWriter) -> RetType;
}
impl<'a> /*trait*/ QTextDocumentWriter_format_0<usize> for () {
  fn format_0(self , rsthis: & QTextDocumentWriter) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QTextDocumentWriter6formatEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextdocumentwriter.h:65
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDevice(QIODevice *)

/*
Sets the writer's device to the device specified. If a device has already been set, the old device is removed but otherwise left unchanged.

If the device is not already open, QTextDocumentWriter will attempt to open the device in QIODevice::WriteOnly mode by calling open().

Note: This will not work for certain devices, such as QProcess, QTcpSocket and QUdpSocket, where some configuration is required before the device can be opened.

See also device() and setFileName().
*/
impl /*struct*/ QTextDocumentWriter {
  pub fn setDevice_0<RetType, T: QTextDocumentWriter_setDevice_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDevice_0(self);
    // return 1;
  }
}
pub trait QTextDocumentWriter_setDevice_0<RetType> {
  fn setDevice_0(self , rsthis: & QTextDocumentWriter) -> RetType;
}
impl<'a> /*trait*/ QTextDocumentWriter_setDevice_0<(/*void*/)> for (usize) {
  fn setDevice_0(self , rsthis: & QTextDocumentWriter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN19QTextDocumentWriter9setDeviceEP9QIODevice", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextdocumentwriter.h:66
// index:0
// Public Visibility=Default Availability=Available
// [8] QIODevice * device() const

/*
Returns the device currently assigned, or 0 if no device has been assigned.

See also setDevice().
*/
impl /*struct*/ QTextDocumentWriter {
  pub fn device_0<RetType, T: QTextDocumentWriter_device_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.device_0(self);
    // return 1;
  }
}
pub trait QTextDocumentWriter_device_0<RetType> {
  fn device_0(self , rsthis: & QTextDocumentWriter) -> RetType;
}
impl<'a> /*trait*/ QTextDocumentWriter_device_0<usize> for () {
  fn device_0(self , rsthis: & QTextDocumentWriter) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QTextDocumentWriter6deviceEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextdocumentwriter.h:67
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFileName(const QString &)

/*
Sets the name of the file to be written to fileName. Internally, QTextDocumentWriter will create a QFile and open it in QIODevice::WriteOnly mode, and use this file when writing the document.

See also fileName() and setDevice().
*/
impl /*struct*/ QTextDocumentWriter {
  pub fn setFileName_0<RetType, T: QTextDocumentWriter_setFileName_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFileName_0(self);
    // return 1;
  }
}
pub trait QTextDocumentWriter_setFileName_0<RetType> {
  fn setFileName_0(self , rsthis: & QTextDocumentWriter) -> RetType;
}
impl<'a> /*trait*/ QTextDocumentWriter_setFileName_0<(/*void*/)> for (usize) {
  fn setFileName_0(self , rsthis: & QTextDocumentWriter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN19QTextDocumentWriter11setFileNameERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextdocumentwriter.h:68
// index:0
// Public Visibility=Default Availability=Available
// [8] QString fileName() const

/*
If the currently assigned device is a QFile, or if setFileName() has been called, this function returns the name of the file to be written to. In all other cases, it returns an empty string.

See also setFileName() and setDevice().
*/
impl /*struct*/ QTextDocumentWriter {
  pub fn fileName_0<RetType, T: QTextDocumentWriter_fileName_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fileName_0(self);
    // return 1;
  }
}
pub trait QTextDocumentWriter_fileName_0<RetType> {
  fn fileName_0(self , rsthis: & QTextDocumentWriter) -> RetType;
}
impl<'a> /*trait*/ QTextDocumentWriter_fileName_0<usize> for () {
  fn fileName_0(self , rsthis: & QTextDocumentWriter) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QTextDocumentWriter8fileNameEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextdocumentwriter.h:70
// index:0
// Public Visibility=Default Availability=Available
// [1] bool write(const QTextDocument *)

/*
Writes the given document to the assigned device or file and returns true if successful; otherwise returns false.
*/
impl /*struct*/ QTextDocumentWriter {
  pub fn write_0<RetType, T: QTextDocumentWriter_write_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.write_0(self);
    // return 1;
  }
}
pub trait QTextDocumentWriter_write_0<RetType> {
  fn write_0(self , rsthis: & QTextDocumentWriter) -> RetType;
}
impl<'a> /*trait*/ QTextDocumentWriter_write_0<bool> for (usize) {
  fn write_0(self , rsthis: & QTextDocumentWriter) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN19QTextDocumentWriter5writeEPK13QTextDocument", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextdocumentwriter.h:71
// index:1
// Public Visibility=Default Availability=Available
// [1] bool write(const QTextDocumentFragment &)

/*
Writes the given document to the assigned device or file and returns true if successful; otherwise returns false.
*/
impl /*struct*/ QTextDocumentWriter {
  pub fn write_1<RetType, T: QTextDocumentWriter_write_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.write_1(self);
    // return 1;
  }
}
pub trait QTextDocumentWriter_write_1<RetType> {
  fn write_1(self , rsthis: & QTextDocumentWriter) -> RetType;
}
impl<'a> /*trait*/ QTextDocumentWriter_write_1<bool> for (usize) {
  fn write_1(self , rsthis: & QTextDocumentWriter) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN19QTextDocumentWriter5writeERK21QTextDocumentFragment", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextdocumentwriter.h:74
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCodec(QTextCodec *)

/*
Sets the codec for this stream to codec. The codec is used for encoding any data that is written. By default, QTextDocumentWriter uses UTF-8.

See also codec().
*/
impl /*struct*/ QTextDocumentWriter {
  pub fn setCodec_0<RetType, T: QTextDocumentWriter_setCodec_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCodec_0(self);
    // return 1;
  }
}
pub trait QTextDocumentWriter_setCodec_0<RetType> {
  fn setCodec_0(self , rsthis: & QTextDocumentWriter) -> RetType;
}
impl<'a> /*trait*/ QTextDocumentWriter_setCodec_0<(/*void*/)> for (usize) {
  fn setCodec_0(self , rsthis: & QTextDocumentWriter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN19QTextDocumentWriter8setCodecEP10QTextCodec", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextdocumentwriter.h:75
// index:0
// Public Visibility=Default Availability=Available
// [8] QTextCodec * codec() const

/*
Returns the codec that is currently assigned to the writer.

See also setCodec().
*/
impl /*struct*/ QTextDocumentWriter {
  pub fn codec_0<RetType, T: QTextDocumentWriter_codec_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.codec_0(self);
    // return 1;
  }
}
pub trait QTextDocumentWriter_codec_0<RetType> {
  fn codec_0(self , rsthis: & QTextDocumentWriter) -> RetType;
}
impl<'a> /*trait*/ QTextDocumentWriter_codec_0<usize> for () {
  fn codec_0(self , rsthis: & QTextDocumentWriter) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QTextDocumentWriter5codecEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
