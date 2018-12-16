

// mod ::core::QTextStream
// package qtcore
// /usr/include/qt/QtCore/qtextstream.h
// #include <qtextstream.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 21
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
#[derive(Default)] // class sizeof(QTextStream)=16
pub struct QTextStream {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QTextStream_ITF interface {
//    QTextStream_PTR() *QTextStream
//}
//func (ptr *QTextStream) QTextStream_PTR() *QTextStream { return ptr }

impl /*struct*/ QTextStream {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QTextStream {
    return QTextStream{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QTextStream {
//  type Target = QTextStreamBASE;
//
//  fn deref(&self) -> &QTextStreamBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QTextStreamBASE> for QTextStream {
//  fn as_ref(& self) -> & QTextStreamBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qtextstream.h:93
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QTextStream()

/*
Constructs a QTextStream. Before you can use it for reading or writing, you must assign a device or a string.

See also setDevice() and setString().
*/
// QTextStream() ctx.fn_proto_cpp
impl /*struct*/ QTextStream {
  pub fn QTextStream_0<T: QTextStream_QTextStream_0>(value: T) -> QTextStream {
    let rsthis = value.QTextStream_0();
    return rsthis;
    // return 1;
  }
}

pub trait QTextStream_QTextStream_0 {
  fn QTextStream_0(self) -> QTextStream;
}
// QTextStream() ctx.fn_proto_cpp
impl<'a> /*trait*/ QTextStream_QTextStream_0 for () {
  fn QTextStream_0(self) -> QTextStream {
    // unsafe{_ZN11QTextStreamC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QTextStreamC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTextStream{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qtextstream.h:94
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QTextStream(QIODevice *)

/*
Constructs a QTextStream. Before you can use it for reading or writing, you must assign a device or a string.

See also setDevice() and setString().
*/
// QTextStream(QIODevice *) ctx.fn_proto_cpp
impl /*struct*/ QTextStream {
  pub fn QTextStream_1<T: QTextStream_QTextStream_1>(value: T) -> QTextStream {
    let rsthis = value.QTextStream_1();
    return rsthis;
    // return 1;
  }
}

pub trait QTextStream_QTextStream_1 {
  fn QTextStream_1(self) -> QTextStream;
}
// QTextStream(QIODevice *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QTextStream_QTextStream_1 for (usize) {
  fn QTextStream_1(self) -> QTextStream {
    // unsafe{_ZN11QTextStreamC2EP9QIODevice()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QTextStreamC2EP9QIODevice", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTextStream{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qtextstream.h:96
// index:2
// Public Visibility=Default Availability=Available
// [-2] void QTextStream(QString *, QIODevice::OpenMode)

/*
Constructs a QTextStream. Before you can use it for reading or writing, you must assign a device or a string.

See also setDevice() and setString().
*/
// QTextStream(QString *, QIODevice::OpenMode) ctx.fn_proto_cpp
impl /*struct*/ QTextStream {
  pub fn QTextStream_2<T: QTextStream_QTextStream_2>(value: T) -> QTextStream {
    let rsthis = value.QTextStream_2();
    return rsthis;
    // return 1;
  }
}

pub trait QTextStream_QTextStream_2 {
  fn QTextStream_2(self) -> QTextStream;
}
// QTextStream(QString *, QIODevice::OpenMode) ctx.fn_proto_cpp
impl<'a> /*trait*/ QTextStream_QTextStream_2 for (usize,i32) {
  fn QTextStream_2(self) -> QTextStream {
    // unsafe{_ZN11QTextStreamC2EP7QString6QFlagsIN9QIODevice12OpenModeFlagEE()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QTextStreamC2EP7QString6QFlagsIN9QIODevice12OpenModeFlagEE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTextStream{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qtextstream.h:97
// index:3
// Public Visibility=Default Availability=Available
// [-2] void QTextStream(QByteArray *, QIODevice::OpenMode)

/*
Constructs a QTextStream. Before you can use it for reading or writing, you must assign a device or a string.

See also setDevice() and setString().
*/
// QTextStream(QByteArray *, QIODevice::OpenMode) ctx.fn_proto_cpp
impl /*struct*/ QTextStream {
  pub fn QTextStream_3<T: QTextStream_QTextStream_3>(value: T) -> QTextStream {
    let rsthis = value.QTextStream_3();
    return rsthis;
    // return 1;
  }
}

pub trait QTextStream_QTextStream_3 {
  fn QTextStream_3(self) -> QTextStream;
}
// QTextStream(QByteArray *, QIODevice::OpenMode) ctx.fn_proto_cpp
impl<'a> /*trait*/ QTextStream_QTextStream_3 for (usize,i32) {
  fn QTextStream_3(self) -> QTextStream {
    // unsafe{_ZN11QTextStreamC2EP10QByteArray6QFlagsIN9QIODevice12OpenModeFlagEE()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QTextStreamC2EP10QByteArray6QFlagsIN9QIODevice12OpenModeFlagEE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTextStream{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qtextstream.h:98
// index:4
// Public Visibility=Default Availability=Available
// [-2] void QTextStream(const QByteArray &, QIODevice::OpenMode)

/*
Constructs a QTextStream. Before you can use it for reading or writing, you must assign a device or a string.

See also setDevice() and setString().
*/
// QTextStream(const QByteArray &, QIODevice::OpenMode) ctx.fn_proto_cpp
impl /*struct*/ QTextStream {
  pub fn QTextStream_4<T: QTextStream_QTextStream_4>(value: T) -> QTextStream {
    let rsthis = value.QTextStream_4();
    return rsthis;
    // return 1;
  }
}

pub trait QTextStream_QTextStream_4 {
  fn QTextStream_4(self) -> QTextStream;
}
// QTextStream(const QByteArray &, QIODevice::OpenMode) ctx.fn_proto_cpp
impl<'a> /*trait*/ QTextStream_QTextStream_4 for (usize,i32) {
  fn QTextStream_4(self) -> QTextStream {
    // unsafe{_ZN11QTextStreamC2ERK10QByteArray6QFlagsIN9QIODevice12OpenModeFlagEE()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QTextStreamC2ERK10QByteArray6QFlagsIN9QIODevice12OpenModeFlagEE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTextStream{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qtextstream.h:99
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QTextStream()

/*

*/
pub fn DeleteQTextStream(this :*mut QTextStream) {
    // let rv = qtrt::InvokeQtFunc6("_ZN11QTextStreamD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qtextstream.h:102
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCodec(QTextCodec *)

/*
Sets the codec for this stream to codec. The codec is used for decoding any data that is read from the assigned device, and for encoding any data that is written. By default, QTextCodec::codecForLocale() is used, and automatic unicode detection is enabled.

If QTextStream operates on a string, this function does nothing.

Warning: If you call this function while the text stream is reading from an open sequential socket, the internal buffer may still contain text decoded using the old codec.

See also codec(), setAutoDetectUnicode(), and setLocale().
*/
impl /*struct*/ QTextStream {
  pub fn setCodec_0<RetType, T: QTextStream_setCodec_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCodec_0(self);
    // return 1;
  }
}
pub trait QTextStream_setCodec_0<RetType> {
  fn setCodec_0(self , rsthis: & QTextStream) -> RetType;
}
impl<'a> /*trait*/ QTextStream_setCodec_0<(/*void*/)> for (usize) {
  fn setCodec_0(self , rsthis: & QTextStream) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QTextStream8setCodecEP10QTextCodec", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qtextstream.h:103
// index:1
// Public Visibility=Default Availability=Available
// [-2] void setCodec(const char *)

/*
Sets the codec for this stream to codec. The codec is used for decoding any data that is read from the assigned device, and for encoding any data that is written. By default, QTextCodec::codecForLocale() is used, and automatic unicode detection is enabled.

If QTextStream operates on a string, this function does nothing.

Warning: If you call this function while the text stream is reading from an open sequential socket, the internal buffer may still contain text decoded using the old codec.

See also codec(), setAutoDetectUnicode(), and setLocale().
*/
impl /*struct*/ QTextStream {
  pub fn setCodec_1<RetType, T: QTextStream_setCodec_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCodec_1(self);
    // return 1;
  }
}
pub trait QTextStream_setCodec_1<RetType> {
  fn setCodec_1(self , rsthis: & QTextStream) -> RetType;
}
impl<'a> /*trait*/ QTextStream_setCodec_1<(/*void*/)> for (usize) {
  fn setCodec_1(self , rsthis: & QTextStream) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QTextStream8setCodecEPKc", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qtextstream.h:104
// index:0
// Public Visibility=Default Availability=Available
// [8] QTextCodec * codec() const

/*
Returns the codec that is current assigned to the stream.

See also setCodec(), setAutoDetectUnicode(), and locale().
*/
impl /*struct*/ QTextStream {
  pub fn codec_0<RetType, T: QTextStream_codec_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.codec_0(self);
    // return 1;
  }
}
pub trait QTextStream_codec_0<RetType> {
  fn codec_0(self , rsthis: & QTextStream) -> RetType;
}
impl<'a> /*trait*/ QTextStream_codec_0<usize> for () {
  fn codec_0(self , rsthis: & QTextStream) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextStream5codecEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtextstream.h:105
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setAutoDetectUnicode(bool)

/*
If enabled is true, QTextStream will attempt to detect Unicode encoding by peeking into the stream data to see if it can find the UTF-16 or UTF-32 BOM (Byte Order Mark). If this mark is found, QTextStream will replace the current codec with the UTF codec.

This function can be used together with setCodec(). It is common to set the codec to UTF-8, and then enable UTF-16 detection.

See also autoDetectUnicode() and setCodec().
*/
impl /*struct*/ QTextStream {
  pub fn setAutoDetectUnicode_0<RetType, T: QTextStream_setAutoDetectUnicode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setAutoDetectUnicode_0(self);
    // return 1;
  }
}
pub trait QTextStream_setAutoDetectUnicode_0<RetType> {
  fn setAutoDetectUnicode_0(self , rsthis: & QTextStream) -> RetType;
}
impl<'a> /*trait*/ QTextStream_setAutoDetectUnicode_0<(/*void*/)> for (bool) {
  fn setAutoDetectUnicode_0(self , rsthis: & QTextStream) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN11QTextStream20setAutoDetectUnicodeEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qtextstream.h:106
// index:0
// Public Visibility=Default Availability=Available
// [1] bool autoDetectUnicode() const

/*
Returns true if automatic Unicode detection is enabled, otherwise returns false. Automatic Unicode detection is enabled by default.

See also setAutoDetectUnicode() and setCodec().
*/
impl /*struct*/ QTextStream {
  pub fn autoDetectUnicode_0<RetType, T: QTextStream_autoDetectUnicode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.autoDetectUnicode_0(self);
    // return 1;
  }
}
pub trait QTextStream_autoDetectUnicode_0<RetType> {
  fn autoDetectUnicode_0(self , rsthis: & QTextStream) -> RetType;
}
impl<'a> /*trait*/ QTextStream_autoDetectUnicode_0<bool> for () {
  fn autoDetectUnicode_0(self , rsthis: & QTextStream) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextStream17autoDetectUnicodeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtextstream.h:107
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setGenerateByteOrderMark(bool)

/*
If generate is true and a UTF codec is used, QTextStream will insert the BOM (Byte Order Mark) before any data has been written to the device. If generate is false, no BOM will be inserted. This function must be called before any data is written. Otherwise, it does nothing.

See also generateByteOrderMark() and bom().
*/
impl /*struct*/ QTextStream {
  pub fn setGenerateByteOrderMark_0<RetType, T: QTextStream_setGenerateByteOrderMark_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setGenerateByteOrderMark_0(self);
    // return 1;
  }
}
pub trait QTextStream_setGenerateByteOrderMark_0<RetType> {
  fn setGenerateByteOrderMark_0(self , rsthis: & QTextStream) -> RetType;
}
impl<'a> /*trait*/ QTextStream_setGenerateByteOrderMark_0<(/*void*/)> for (bool) {
  fn setGenerateByteOrderMark_0(self , rsthis: & QTextStream) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN11QTextStream24setGenerateByteOrderMarkEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qtextstream.h:108
// index:0
// Public Visibility=Default Availability=Available
// [1] bool generateByteOrderMark() const

/*
Returns true if QTextStream is set to generate the UTF BOM (Byte Order Mark) when using a UTF codec; otherwise returns false. UTF BOM generation is set to false by default.

See also setGenerateByteOrderMark().
*/
impl /*struct*/ QTextStream {
  pub fn generateByteOrderMark_0<RetType, T: QTextStream_generateByteOrderMark_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.generateByteOrderMark_0(self);
    // return 1;
  }
}
pub trait QTextStream_generateByteOrderMark_0<RetType> {
  fn generateByteOrderMark_0(self , rsthis: & QTextStream) -> RetType;
}
impl<'a> /*trait*/ QTextStream_generateByteOrderMark_0<bool> for () {
  fn generateByteOrderMark_0(self , rsthis: & QTextStream) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextStream21generateByteOrderMarkEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtextstream.h:111
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setLocale(const QLocale &)

/*
Sets the locale for this stream to locale. The specified locale is used for conversions between numbers and their string representations.

The default locale is C and it is a special case - the thousands group separator is not used for backward compatibility reasons.

This function was introduced in  Qt 4.5.

See also locale().
*/
impl /*struct*/ QTextStream {
  pub fn setLocale_0<RetType, T: QTextStream_setLocale_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setLocale_0(self);
    // return 1;
  }
}
pub trait QTextStream_setLocale_0<RetType> {
  fn setLocale_0(self , rsthis: & QTextStream) -> RetType;
}
impl<'a> /*trait*/ QTextStream_setLocale_0<(/*void*/)> for (usize) {
  fn setLocale_0(self , rsthis: & QTextStream) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QTextStream9setLocaleERK7QLocale", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qtextstream.h:112
// index:0
// Public Visibility=Default Availability=Available
// [8] QLocale locale() const

/*
Returns the locale for this stream. The default locale is C.

This function was introduced in  Qt 4.5.

See also setLocale().
*/
impl /*struct*/ QTextStream {
  pub fn locale_0<RetType, T: QTextStream_locale_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.locale_0(self);
    // return 1;
  }
}
pub trait QTextStream_locale_0<RetType> {
  fn locale_0(self , rsthis: & QTextStream) -> RetType;
}
impl<'a> /*trait*/ QTextStream_locale_0<usize> for () {
  fn locale_0(self , rsthis: & QTextStream) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextStream6localeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtextstream.h:114
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDevice(QIODevice *)

/*
Sets the current device to device. If a device has already been assigned, QTextStream will call flush() before the old device is replaced.

Note: This function resets locale to the default locale ('C') and codec to the default codec, QTextCodec::codecForLocale().

See also device() and setString().
*/
impl /*struct*/ QTextStream {
  pub fn setDevice_0<RetType, T: QTextStream_setDevice_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDevice_0(self);
    // return 1;
  }
}
pub trait QTextStream_setDevice_0<RetType> {
  fn setDevice_0(self , rsthis: & QTextStream) -> RetType;
}
impl<'a> /*trait*/ QTextStream_setDevice_0<(/*void*/)> for (usize) {
  fn setDevice_0(self , rsthis: & QTextStream) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QTextStream9setDeviceEP9QIODevice", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qtextstream.h:115
// index:0
// Public Visibility=Default Availability=Available
// [8] QIODevice * device() const

/*
Returns the current device associated with the QTextStream, or 0 if no device has been assigned.

See also setDevice() and string().
*/
impl /*struct*/ QTextStream {
  pub fn device_0<RetType, T: QTextStream_device_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.device_0(self);
    // return 1;
  }
}
pub trait QTextStream_device_0<RetType> {
  fn device_0(self , rsthis: & QTextStream) -> RetType;
}
impl<'a> /*trait*/ QTextStream_device_0<usize> for () {
  fn device_0(self , rsthis: & QTextStream) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextStream6deviceEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtextstream.h:117
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setString(QString *, QIODevice::OpenMode)

/*
Sets the current string to string, using the given openMode. If a device has already been assigned, QTextStream will call flush() before replacing it.

See also string() and setDevice().
*/
impl /*struct*/ QTextStream {
  pub fn setString_0<RetType, T: QTextStream_setString_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setString_0(self);
    // return 1;
  }
}
pub trait QTextStream_setString_0<RetType> {
  fn setString_0(self , rsthis: & QTextStream) -> RetType;
}
impl<'a> /*trait*/ QTextStream_setString_0<(/*void*/)> for (usize,i32) {
  fn setString_0(self , rsthis: & QTextStream) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QTextStream9setStringEP7QString6QFlagsIN9QIODevice12OpenModeFlagEE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qtextstream.h:118
// index:0
// Public Visibility=Default Availability=Available
// [8] QString * string() const

/*
Returns the current string assigned to the QTextStream, or 0 if no string has been assigned.

See also setString() and device().
*/
impl /*struct*/ QTextStream {
  pub fn string_0<RetType, T: QTextStream_string_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.string_0(self);
    // return 1;
  }
}
pub trait QTextStream_string_0<RetType> {
  fn string_0(self , rsthis: & QTextStream) -> RetType;
}
impl<'a> /*trait*/ QTextStream_string_0<usize> for () {
  fn string_0(self , rsthis: & QTextStream) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextStream6stringEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtextstream.h:120
// index:0
// Public Visibility=Default Availability=Available
// [4] QTextStream::Status status() const

/*
Returns the status of the text stream.

See also QTextStream::Status, setStatus(), and resetStatus().
*/
impl /*struct*/ QTextStream {
  pub fn status_0<RetType, T: QTextStream_status_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.status_0(self);
    // return 1;
  }
}
pub trait QTextStream_status_0<RetType> {
  fn status_0(self , rsthis: & QTextStream) -> RetType;
}
impl<'a> /*trait*/ QTextStream_status_0<i32> for () {
  fn status_0(self , rsthis: & QTextStream) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextStream6statusEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtextstream.h:121
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setStatus(QTextStream::Status)

/*
Sets the status of the text stream to the status given.

Subsequent calls to setStatus() are ignored until resetStatus() is called.

This function was introduced in  Qt 4.1.

See also Status, status(), and resetStatus().
*/
impl /*struct*/ QTextStream {
  pub fn setStatus_0<RetType, T: QTextStream_setStatus_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setStatus_0(self);
    // return 1;
  }
}
pub trait QTextStream_setStatus_0<RetType> {
  fn setStatus_0(self , rsthis: & QTextStream) -> RetType;
}
impl<'a> /*trait*/ QTextStream_setStatus_0<(/*void*/)> for (i32) {
  fn setStatus_0(self , rsthis: & QTextStream) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QTextStream9setStatusENS_6StatusE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qtextstream.h:122
// index:0
// Public Visibility=Default Availability=Available
// [-2] void resetStatus()

/*
Resets the status of the text stream.

This function was introduced in  Qt 4.1.

See also QTextStream::Status, status(), and setStatus().
*/
impl /*struct*/ QTextStream {
  pub fn resetStatus_0<RetType, T: QTextStream_resetStatus_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resetStatus_0(self);
    // return 1;
  }
}
pub trait QTextStream_resetStatus_0<RetType> {
  fn resetStatus_0(self , rsthis: & QTextStream) -> RetType;
}
impl<'a> /*trait*/ QTextStream_resetStatus_0<(/*void*/)> for () {
  fn resetStatus_0(self , rsthis: & QTextStream) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN11QTextStream11resetStatusEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qtextstream.h:124
// index:0
// Public Visibility=Default Availability=Available
// [1] bool atEnd() const

/*
Returns true if there is no more data to be read from the QTextStream; otherwise returns false. This is similar to, but not the same as calling QIODevice::atEnd(), as QTextStream also takes into account its internal Unicode buffer.
*/
impl /*struct*/ QTextStream {
  pub fn atEnd_0<RetType, T: QTextStream_atEnd_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.atEnd_0(self);
    // return 1;
  }
}
pub trait QTextStream_atEnd_0<RetType> {
  fn atEnd_0(self , rsthis: & QTextStream) -> RetType;
}
impl<'a> /*trait*/ QTextStream_atEnd_0<bool> for () {
  fn atEnd_0(self , rsthis: & QTextStream) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextStream5atEndEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtextstream.h:125
// index:0
// Public Visibility=Default Availability=Available
// [-2] void reset()

/*
Resets QTextStream's formatting options, bringing it back to its original constructed state. The device, string and any buffered data is left untouched.
*/
impl /*struct*/ QTextStream {
  pub fn reset_0<RetType, T: QTextStream_reset_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.reset_0(self);
    // return 1;
  }
}
pub trait QTextStream_reset_0<RetType> {
  fn reset_0(self , rsthis: & QTextStream) -> RetType;
}
impl<'a> /*trait*/ QTextStream_reset_0<(/*void*/)> for () {
  fn reset_0(self , rsthis: & QTextStream) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN11QTextStream5resetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qtextstream.h:126
// index:0
// Public Visibility=Default Availability=Available
// [-2] void flush()

/*
Flushes any buffered data waiting to be written to the device.

If QTextStream operates on a string, this function does nothing.
*/
impl /*struct*/ QTextStream {
  pub fn flush_0<RetType, T: QTextStream_flush_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.flush_0(self);
    // return 1;
  }
}
pub trait QTextStream_flush_0<RetType> {
  fn flush_0(self , rsthis: & QTextStream) -> RetType;
}
impl<'a> /*trait*/ QTextStream_flush_0<(/*void*/)> for () {
  fn flush_0(self , rsthis: & QTextStream) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN11QTextStream5flushEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qtextstream.h:127
// index:0
// Public Visibility=Default Availability=Available
// [1] bool seek(qint64)

/*
Seeks to the position pos in the device. Returns true on success; otherwise returns false.
*/
impl /*struct*/ QTextStream {
  pub fn seek_0<RetType, T: QTextStream_seek_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.seek_0(self);
    // return 1;
  }
}
pub trait QTextStream_seek_0<RetType> {
  fn seek_0(self , rsthis: & QTextStream) -> RetType;
}
impl<'a> /*trait*/ QTextStream_seek_0<bool> for (i64) {
  fn seek_0(self , rsthis: & QTextStream) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QTextStream4seekEx", 1,qtrt::FFITY_SINT64,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtextstream.h:128
// index:0
// Public Visibility=Default Availability=Available
// [8] qint64 pos() const

/*
Returns the device position corresponding to the current position of the stream, or -1 if an error occurs (e.g., if there is no device or string, or if there's a device error).

Because QTextStream is buffered, this function may have to seek the device to reconstruct a valid device position. This operation can be expensive, so you may want to avoid calling this function in a tight loop.

This function was introduced in  Qt 4.2.

See also seek().
*/
impl /*struct*/ QTextStream {
  pub fn pos_0<RetType, T: QTextStream_pos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.pos_0(self);
    // return 1;
  }
}
pub trait QTextStream_pos_0<RetType> {
  fn pos_0(self , rsthis: & QTextStream) -> RetType;
}
impl<'a> /*trait*/ QTextStream_pos_0<i64> for () {
  fn pos_0(self , rsthis: & QTextStream) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextStream3posEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtextstream.h:130
// index:0
// Public Visibility=Default Availability=Available
// [-2] void skipWhiteSpace()

/*
Reads and discards whitespace from the stream until either a non-space character is detected, or until atEnd() returns true. This function is useful when reading a stream character by character.

Whitespace characters are all characters for which QChar::isSpace() returns true.

See also operator>>().
*/
impl /*struct*/ QTextStream {
  pub fn skipWhiteSpace_0<RetType, T: QTextStream_skipWhiteSpace_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.skipWhiteSpace_0(self);
    // return 1;
  }
}
pub trait QTextStream_skipWhiteSpace_0<RetType> {
  fn skipWhiteSpace_0(self , rsthis: & QTextStream) -> RetType;
}
impl<'a> /*trait*/ QTextStream_skipWhiteSpace_0<(/*void*/)> for () {
  fn skipWhiteSpace_0(self , rsthis: & QTextStream) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN11QTextStream14skipWhiteSpaceEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qtextstream.h:132
// index:0
// Public Visibility=Default Availability=Available
// [8] QString readLine(qint64)

/*
Reads one line of text from the stream, and returns it as a QString. The maximum allowed line length is set to maxlen. If the stream contains lines longer than this, then the lines will be split after maxlen characters and returned in parts.

If maxlen is 0, the lines can be of any length.

The returned line has no trailing end-of-line characters ("\n" or "\r\n"), so calling QString::trimmed() can be unnecessary.

If the stream has read to the end of the file, readLine() will return a null QString. For strings, or for devices that support it, you can explicitly test for the end of the stream using atEnd().

See also readAll() and QIODevice::readLine().
*/
impl /*struct*/ QTextStream {
  pub fn readLine_0<RetType, T: QTextStream_readLine_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.readLine_0(self);
    // return 1;
  }
}
pub trait QTextStream_readLine_0<RetType> {
  fn readLine_0(self , rsthis: & QTextStream) -> RetType;
}
impl<'a> /*trait*/ QTextStream_readLine_0<usize> for (i64) {
  fn readLine_0(self , rsthis: & QTextStream) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QTextStream8readLineEx", 1,qtrt::FFITY_SINT64,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtextstream.h:133
// index:0
// Public Visibility=Default Availability=Available
// [1] bool readLineInto(QString *, qint64)

/*
Reads one line of text from the stream into line. If line is 0, the read line is not stored.

The maximum allowed line length is set to maxlen. If the stream contains lines longer than this, then the lines will be split after maxlen characters and returned in parts.

If maxlen is 0, the lines can be of any length.

The resulting line has no trailing end-of-line characters ("\n" or "\r\n"), so calling QString::trimmed() can be unnecessary.

If line has sufficient capacity for the data that is about to be read, this function may not need to allocate new memory. Because of this, it can be faster than readLine().

Returns false if the stream has read to the end of the file or an error has occurred; otherwise returns true. The contents in line before the call are discarded in any case.

This function was introduced in  Qt 5.5.

See also readAll() and QIODevice::readLine().
*/
impl /*struct*/ QTextStream {
  pub fn readLineInto_0<RetType, T: QTextStream_readLineInto_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.readLineInto_0(self);
    // return 1;
  }
}
pub trait QTextStream_readLineInto_0<RetType> {
  fn readLineInto_0(self , rsthis: & QTextStream) -> RetType;
}
impl<'a> /*trait*/ QTextStream_readLineInto_0<bool> for (usize,i64) {
  fn readLineInto_0(self , rsthis: & QTextStream) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QTextStream12readLineIntoEP7QStringx", 2,qtrt::FFITY_POINTER,qtrt::FFITY_SINT64,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtextstream.h:134
// index:0
// Public Visibility=Default Availability=Available
// [8] QString readAll()

/*
Reads the entire content of the stream, and returns it as a QString. Avoid this function when working on large files, as it will consume a significant amount of memory.

Calling readLine() is better if you do not know how much data is available.

See also readLine().
*/
impl /*struct*/ QTextStream {
  pub fn readAll_0<RetType, T: QTextStream_readAll_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.readAll_0(self);
    // return 1;
  }
}
pub trait QTextStream_readAll_0<RetType> {
  fn readAll_0(self , rsthis: & QTextStream) -> RetType;
}
impl<'a> /*trait*/ QTextStream_readAll_0<usize> for () {
  fn readAll_0(self , rsthis: & QTextStream) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QTextStream7readAllEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtextstream.h:135
// index:0
// Public Visibility=Default Availability=Available
// [8] QString read(qint64)

/*
Reads at most maxlen characters from the stream, and returns the data read as a QString.

This function was introduced in  Qt 4.1.

See also readAll(), readLine(), and QIODevice::read().
*/
impl /*struct*/ QTextStream {
  pub fn read_0<RetType, T: QTextStream_read_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.read_0(self);
    // return 1;
  }
}
pub trait QTextStream_read_0<RetType> {
  fn read_0(self , rsthis: & QTextStream) -> RetType;
}
impl<'a> /*trait*/ QTextStream_read_0<usize> for (i64) {
  fn read_0(self , rsthis: & QTextStream) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QTextStream4readEx", 1,qtrt::FFITY_SINT64,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtextstream.h:137
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFieldAlignment(QTextStream::FieldAlignment)

/*
Sets the field alignment to mode. When used together with setFieldWidth(), this function allows you to generate formatted output with text aligned to the left, to the right or center aligned.

See also fieldAlignment() and setFieldWidth().
*/
impl /*struct*/ QTextStream {
  pub fn setFieldAlignment_0<RetType, T: QTextStream_setFieldAlignment_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFieldAlignment_0(self);
    // return 1;
  }
}
pub trait QTextStream_setFieldAlignment_0<RetType> {
  fn setFieldAlignment_0(self , rsthis: & QTextStream) -> RetType;
}
impl<'a> /*trait*/ QTextStream_setFieldAlignment_0<(/*void*/)> for (i32) {
  fn setFieldAlignment_0(self , rsthis: & QTextStream) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QTextStream17setFieldAlignmentENS_14FieldAlignmentE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qtextstream.h:138
// index:0
// Public Visibility=Default Availability=Available
// [4] QTextStream::FieldAlignment fieldAlignment() const

/*
Returns the current field alignment.

See also setFieldAlignment() and fieldWidth().
*/
impl /*struct*/ QTextStream {
  pub fn fieldAlignment_0<RetType, T: QTextStream_fieldAlignment_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fieldAlignment_0(self);
    // return 1;
  }
}
pub trait QTextStream_fieldAlignment_0<RetType> {
  fn fieldAlignment_0(self , rsthis: & QTextStream) -> RetType;
}
impl<'a> /*trait*/ QTextStream_fieldAlignment_0<i32> for () {
  fn fieldAlignment_0(self , rsthis: & QTextStream) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextStream14fieldAlignmentEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtextstream.h:140
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setPadChar(QChar)

/*
Sets the pad character to ch. The default value is the ASCII space character (' '), or QChar(0x20). This character is used to fill in the space in fields when generating text.

Example:


  QString s;
  QTextStream out(&s);
  out.setFieldWidth(10);
  out.setFieldAlignment(QTextStream::AlignCenter);
  out.setPadChar('-');
  out << "Qt" << "rocks!";



The string s contains:


  ----Qt------rocks!--



See also padChar() and setFieldWidth().
*/
impl /*struct*/ QTextStream {
  pub fn setPadChar_0<RetType, T: QTextStream_setPadChar_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPadChar_0(self);
    // return 1;
  }
}
pub trait QTextStream_setPadChar_0<RetType> {
  fn setPadChar_0(self , rsthis: & QTextStream) -> RetType;
}
impl<'a> /*trait*/ QTextStream_setPadChar_0<(/*void*/)> for (usize) {
  fn setPadChar_0(self , rsthis: & QTextStream) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QTextStream10setPadCharE5QChar", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qtextstream.h:141
// index:0
// Public Visibility=Default Availability=Available
// [2] QChar padChar() const

/*
Returns the current pad character.

See also setPadChar() and setFieldWidth().
*/
impl /*struct*/ QTextStream {
  pub fn padChar_0<RetType, T: QTextStream_padChar_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.padChar_0(self);
    // return 1;
  }
}
pub trait QTextStream_padChar_0<RetType> {
  fn padChar_0(self , rsthis: & QTextStream) -> RetType;
}
impl<'a> /*trait*/ QTextStream_padChar_0<usize> for () {
  fn padChar_0(self , rsthis: & QTextStream) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextStream7padCharEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtextstream.h:143
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFieldWidth(int)

/*
Sets the current field width to width. If width is 0 (the default), the field width is equal to the length of the generated text.

Note: The field width applies to every element appended to this stream after this function has been called (e.g., it also pads endl). This behavior is different from similar classes in the STL, where the field width only applies to the next element.

See also fieldWidth() and setPadChar().
*/
impl /*struct*/ QTextStream {
  pub fn setFieldWidth_0<RetType, T: QTextStream_setFieldWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFieldWidth_0(self);
    // return 1;
  }
}
pub trait QTextStream_setFieldWidth_0<RetType> {
  fn setFieldWidth_0(self , rsthis: & QTextStream) -> RetType;
}
impl<'a> /*trait*/ QTextStream_setFieldWidth_0<(/*void*/)> for (i32) {
  fn setFieldWidth_0(self , rsthis: & QTextStream) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QTextStream13setFieldWidthEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qtextstream.h:144
// index:0
// Public Visibility=Default Availability=Available
// [4] int fieldWidth() const

/*
Returns the current field width.

See also setFieldWidth().
*/
impl /*struct*/ QTextStream {
  pub fn fieldWidth_0<RetType, T: QTextStream_fieldWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fieldWidth_0(self);
    // return 1;
  }
}
pub trait QTextStream_fieldWidth_0<RetType> {
  fn fieldWidth_0(self , rsthis: & QTextStream) -> RetType;
}
impl<'a> /*trait*/ QTextStream_fieldWidth_0<i32> for () {
  fn fieldWidth_0(self , rsthis: & QTextStream) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextStream10fieldWidthEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtextstream.h:146
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setNumberFlags(QTextStream::NumberFlags)

/*
Sets the current number flags to flags. flags is a set of flags from the NumberFlag enum, and describes options for formatting generated code (e.g., whether or not to always write the base or sign of a number).

See also numberFlags(), setIntegerBase(), and setRealNumberNotation().
*/
impl /*struct*/ QTextStream {
  pub fn setNumberFlags_0<RetType, T: QTextStream_setNumberFlags_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setNumberFlags_0(self);
    // return 1;
  }
}
pub trait QTextStream_setNumberFlags_0<RetType> {
  fn setNumberFlags_0(self , rsthis: & QTextStream) -> RetType;
}
impl<'a> /*trait*/ QTextStream_setNumberFlags_0<(/*void*/)> for (i32) {
  fn setNumberFlags_0(self , rsthis: & QTextStream) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QTextStream14setNumberFlagsE6QFlagsINS_10NumberFlagEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qtextstream.h:147
// index:0
// Public Visibility=Default Availability=Available
// [4] QTextStream::NumberFlags numberFlags() const

/*
Returns the current number flags.

See also setNumberFlags(), integerBase(), and realNumberNotation().
*/
impl /*struct*/ QTextStream {
  pub fn numberFlags_0<RetType, T: QTextStream_numberFlags_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.numberFlags_0(self);
    // return 1;
  }
}
pub trait QTextStream_numberFlags_0<RetType> {
  fn numberFlags_0(self , rsthis: & QTextStream) -> RetType;
}
impl<'a> /*trait*/ QTextStream_numberFlags_0<i32> for () {
  fn numberFlags_0(self , rsthis: & QTextStream) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextStream11numberFlagsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtextstream.h:149
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setIntegerBase(int)

/*
Sets the base of integers to base, both for reading and for generating numbers. base can be either 2 (binary), 8 (octal), 10 (decimal) or 16 (hexadecimal). If base is 0, QTextStream will attempt to detect the base by inspecting the data on the stream. When generating numbers, QTextStream assumes base is 10 unless the base has been set explicitly.

See also integerBase(), QString::number(), and setNumberFlags().
*/
impl /*struct*/ QTextStream {
  pub fn setIntegerBase_0<RetType, T: QTextStream_setIntegerBase_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setIntegerBase_0(self);
    // return 1;
  }
}
pub trait QTextStream_setIntegerBase_0<RetType> {
  fn setIntegerBase_0(self , rsthis: & QTextStream) -> RetType;
}
impl<'a> /*trait*/ QTextStream_setIntegerBase_0<(/*void*/)> for (i32) {
  fn setIntegerBase_0(self , rsthis: & QTextStream) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QTextStream14setIntegerBaseEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qtextstream.h:150
// index:0
// Public Visibility=Default Availability=Available
// [4] int integerBase() const

/*
Returns the current base of integers. 0 means that the base is detected when reading, or 10 (decimal) when generating numbers.

See also setIntegerBase(), QString::number(), and numberFlags().
*/
impl /*struct*/ QTextStream {
  pub fn integerBase_0<RetType, T: QTextStream_integerBase_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.integerBase_0(self);
    // return 1;
  }
}
pub trait QTextStream_integerBase_0<RetType> {
  fn integerBase_0(self , rsthis: & QTextStream) -> RetType;
}
impl<'a> /*trait*/ QTextStream_integerBase_0<i32> for () {
  fn integerBase_0(self , rsthis: & QTextStream) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextStream11integerBaseEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtextstream.h:152
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setRealNumberNotation(QTextStream::RealNumberNotation)

/*
Sets the real number notation to notation (SmartNotation, FixedNotation, ScientificNotation). When reading and generating numbers, QTextStream uses this value to detect the formatting of real numbers.

See also realNumberNotation(), setRealNumberPrecision(), setNumberFlags(), and setIntegerBase().
*/
impl /*struct*/ QTextStream {
  pub fn setRealNumberNotation_0<RetType, T: QTextStream_setRealNumberNotation_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setRealNumberNotation_0(self);
    // return 1;
  }
}
pub trait QTextStream_setRealNumberNotation_0<RetType> {
  fn setRealNumberNotation_0(self , rsthis: & QTextStream) -> RetType;
}
impl<'a> /*trait*/ QTextStream_setRealNumberNotation_0<(/*void*/)> for (i32) {
  fn setRealNumberNotation_0(self , rsthis: & QTextStream) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QTextStream21setRealNumberNotationENS_18RealNumberNotationE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qtextstream.h:153
// index:0
// Public Visibility=Default Availability=Available
// [4] QTextStream::RealNumberNotation realNumberNotation() const

/*
Returns the current real number notation.

See also setRealNumberNotation(), realNumberPrecision(), numberFlags(), and integerBase().
*/
impl /*struct*/ QTextStream {
  pub fn realNumberNotation_0<RetType, T: QTextStream_realNumberNotation_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.realNumberNotation_0(self);
    // return 1;
  }
}
pub trait QTextStream_realNumberNotation_0<RetType> {
  fn realNumberNotation_0(self , rsthis: & QTextStream) -> RetType;
}
impl<'a> /*trait*/ QTextStream_realNumberNotation_0<i32> for () {
  fn realNumberNotation_0(self , rsthis: & QTextStream) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextStream18realNumberNotationEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtextstream.h:155
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setRealNumberPrecision(int)

/*
Sets the precision of real numbers to precision. This value describes the number of fraction digits QTextStream should write when generating real numbers.

The precision cannot be a negative value. The default value is 6.

See also realNumberPrecision() and setRealNumberNotation().
*/
impl /*struct*/ QTextStream {
  pub fn setRealNumberPrecision_0<RetType, T: QTextStream_setRealNumberPrecision_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setRealNumberPrecision_0(self);
    // return 1;
  }
}
pub trait QTextStream_setRealNumberPrecision_0<RetType> {
  fn setRealNumberPrecision_0(self , rsthis: & QTextStream) -> RetType;
}
impl<'a> /*trait*/ QTextStream_setRealNumberPrecision_0<(/*void*/)> for (i32) {
  fn setRealNumberPrecision_0(self , rsthis: & QTextStream) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QTextStream22setRealNumberPrecisionEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qtextstream.h:156
// index:0
// Public Visibility=Default Availability=Available
// [4] int realNumberPrecision() const

/*
Returns the current real number precision, or the number of fraction digits QTextStream will write when generating real numbers.

See also setRealNumberPrecision(), setRealNumberNotation(), realNumberNotation(), numberFlags(), and integerBase().
*/
impl /*struct*/ QTextStream {
  pub fn realNumberPrecision_0<RetType, T: QTextStream_realNumberPrecision_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.realNumberPrecision_0(self);
    // return 1;
  }
}
pub trait QTextStream_realNumberPrecision_0<RetType> {
  fn realNumberPrecision_0(self , rsthis: & QTextStream) -> RetType;
}
impl<'a> /*trait*/ QTextStream_realNumberPrecision_0<i32> for () {
  fn realNumberPrecision_0(self , rsthis: & QTextStream) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextStream19realNumberPrecisionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtextstream.h:158
// index:0
// Public Visibility=Default Availability=Available
// [16] QTextStream & operator>>(QChar &)

/*

*/
impl /*struct*/ QTextStream {
  pub fn operator_right_shift_0<RetType, T: QTextStream_operator_right_shift_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_right_shift_0(self);
    // return 1;
  }
}
pub trait QTextStream_operator_right_shift_0<RetType> {
  fn operator_right_shift_0(self , rsthis: & QTextStream) -> RetType;
}
impl<'a> /*trait*/ QTextStream_operator_right_shift_0<usize> for (usize) {
  fn operator_right_shift_0(self , rsthis: & QTextStream) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QTextStreamrsER5QChar", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtextstream.h:159
// index:1
// Public Visibility=Default Availability=Available
// [16] QTextStream & operator>>(char &)

/*

*/
impl /*struct*/ QTextStream {
  pub fn operator_right_shift_1<RetType, T: QTextStream_operator_right_shift_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_right_shift_1(self);
    // return 1;
  }
}
pub trait QTextStream_operator_right_shift_1<RetType> {
  fn operator_right_shift_1(self , rsthis: & QTextStream) -> RetType;
}
impl<'a> /*trait*/ QTextStream_operator_right_shift_1<usize> for (usize) {
  fn operator_right_shift_1(self , rsthis: & QTextStream) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QTextStreamrsERc", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtextstream.h:160
// index:2
// Public Visibility=Default Availability=Available
// [16] QTextStream & operator>>(short &)

/*

*/
impl /*struct*/ QTextStream {
  pub fn operator_right_shift_2<RetType, T: QTextStream_operator_right_shift_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_right_shift_2(self);
    // return 1;
  }
}
pub trait QTextStream_operator_right_shift_2<RetType> {
  fn operator_right_shift_2(self , rsthis: & QTextStream) -> RetType;
}
impl<'a> /*trait*/ QTextStream_operator_right_shift_2<usize> for (usize) {
  fn operator_right_shift_2(self , rsthis: & QTextStream) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QTextStreamrsERs", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtextstream.h:161
// index:3
// Public Visibility=Default Availability=Available
// [16] QTextStream & operator>>(unsigned short &)

/*

*/
impl /*struct*/ QTextStream {
  pub fn operator_right_shift_3<RetType, T: QTextStream_operator_right_shift_3<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_right_shift_3(self);
    // return 1;
  }
}
pub trait QTextStream_operator_right_shift_3<RetType> {
  fn operator_right_shift_3(self , rsthis: & QTextStream) -> RetType;
}
impl<'a> /*trait*/ QTextStream_operator_right_shift_3<usize> for (usize) {
  fn operator_right_shift_3(self , rsthis: & QTextStream) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QTextStreamrsERt", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtextstream.h:162
// index:4
// Public Visibility=Default Availability=Available
// [16] QTextStream & operator>>(int &)

/*

*/
impl /*struct*/ QTextStream {
  pub fn operator_right_shift_4<RetType, T: QTextStream_operator_right_shift_4<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_right_shift_4(self);
    // return 1;
  }
}
pub trait QTextStream_operator_right_shift_4<RetType> {
  fn operator_right_shift_4(self , rsthis: & QTextStream) -> RetType;
}
impl<'a> /*trait*/ QTextStream_operator_right_shift_4<usize> for (usize) {
  fn operator_right_shift_4(self , rsthis: & QTextStream) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QTextStreamrsERi", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtextstream.h:163
// index:5
// Public Visibility=Default Availability=Available
// [16] QTextStream & operator>>(unsigned int &)

/*

*/
impl /*struct*/ QTextStream {
  pub fn operator_right_shift_5<RetType, T: QTextStream_operator_right_shift_5<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_right_shift_5(self);
    // return 1;
  }
}
pub trait QTextStream_operator_right_shift_5<RetType> {
  fn operator_right_shift_5(self , rsthis: & QTextStream) -> RetType;
}
impl<'a> /*trait*/ QTextStream_operator_right_shift_5<usize> for (usize) {
  fn operator_right_shift_5(self , rsthis: & QTextStream) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QTextStreamrsERj", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtextstream.h:164
// index:6
// Public Visibility=Default Availability=Available
// [16] QTextStream & operator>>(long &)

/*

*/
impl /*struct*/ QTextStream {
  pub fn operator_right_shift_6<RetType, T: QTextStream_operator_right_shift_6<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_right_shift_6(self);
    // return 1;
  }
}
pub trait QTextStream_operator_right_shift_6<RetType> {
  fn operator_right_shift_6(self , rsthis: & QTextStream) -> RetType;
}
impl<'a> /*trait*/ QTextStream_operator_right_shift_6<usize> for (usize) {
  fn operator_right_shift_6(self , rsthis: & QTextStream) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QTextStreamrsERl", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtextstream.h:165
// index:7
// Public Visibility=Default Availability=Available
// [16] QTextStream & operator>>(unsigned long &)

/*

*/
impl /*struct*/ QTextStream {
  pub fn operator_right_shift_7<RetType, T: QTextStream_operator_right_shift_7<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_right_shift_7(self);
    // return 1;
  }
}
pub trait QTextStream_operator_right_shift_7<RetType> {
  fn operator_right_shift_7(self , rsthis: & QTextStream) -> RetType;
}
impl<'a> /*trait*/ QTextStream_operator_right_shift_7<usize> for (usize) {
  fn operator_right_shift_7(self , rsthis: & QTextStream) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QTextStreamrsERm", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtextstream.h:166
// index:8
// Public Visibility=Default Availability=Available
// [16] QTextStream & operator>>(qlonglong &)

/*

*/
impl /*struct*/ QTextStream {
  pub fn operator_right_shift_8<RetType, T: QTextStream_operator_right_shift_8<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_right_shift_8(self);
    // return 1;
  }
}
pub trait QTextStream_operator_right_shift_8<RetType> {
  fn operator_right_shift_8(self , rsthis: & QTextStream) -> RetType;
}
impl<'a> /*trait*/ QTextStream_operator_right_shift_8<usize> for (usize) {
  fn operator_right_shift_8(self , rsthis: & QTextStream) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QTextStreamrsERx", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtextstream.h:167
// index:9
// Public Visibility=Default Availability=Available
// [16] QTextStream & operator>>(qulonglong &)

/*

*/
impl /*struct*/ QTextStream {
  pub fn operator_right_shift_9<RetType, T: QTextStream_operator_right_shift_9<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_right_shift_9(self);
    // return 1;
  }
}
pub trait QTextStream_operator_right_shift_9<RetType> {
  fn operator_right_shift_9(self , rsthis: & QTextStream) -> RetType;
}
impl<'a> /*trait*/ QTextStream_operator_right_shift_9<usize> for (usize) {
  fn operator_right_shift_9(self , rsthis: & QTextStream) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QTextStreamrsERy", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtextstream.h:168
// index:10
// Public Visibility=Default Availability=Available
// [16] QTextStream & operator>>(float &)

/*

*/
impl /*struct*/ QTextStream {
  pub fn operator_right_shift_10<RetType, T: QTextStream_operator_right_shift_10<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_right_shift_10(self);
    // return 1;
  }
}
pub trait QTextStream_operator_right_shift_10<RetType> {
  fn operator_right_shift_10(self , rsthis: & QTextStream) -> RetType;
}
impl<'a> /*trait*/ QTextStream_operator_right_shift_10<usize> for (usize) {
  fn operator_right_shift_10(self , rsthis: & QTextStream) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QTextStreamrsERf", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtextstream.h:169
// index:11
// Public Visibility=Default Availability=Available
// [16] QTextStream & operator>>(double &)

/*

*/
impl /*struct*/ QTextStream {
  pub fn operator_right_shift_11<RetType, T: QTextStream_operator_right_shift_11<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_right_shift_11(self);
    // return 1;
  }
}
pub trait QTextStream_operator_right_shift_11<RetType> {
  fn operator_right_shift_11(self , rsthis: & QTextStream) -> RetType;
}
impl<'a> /*trait*/ QTextStream_operator_right_shift_11<usize> for (usize) {
  fn operator_right_shift_11(self , rsthis: & QTextStream) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QTextStreamrsERd", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtextstream.h:170
// index:12
// Public Visibility=Default Availability=Available
// [16] QTextStream & operator>>(QString &)

/*

*/
impl /*struct*/ QTextStream {
  pub fn operator_right_shift_12<RetType, T: QTextStream_operator_right_shift_12<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_right_shift_12(self);
    // return 1;
  }
}
pub trait QTextStream_operator_right_shift_12<RetType> {
  fn operator_right_shift_12(self , rsthis: & QTextStream) -> RetType;
}
impl<'a> /*trait*/ QTextStream_operator_right_shift_12<usize> for (usize) {
  fn operator_right_shift_12(self , rsthis: & QTextStream) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QTextStreamrsER7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtextstream.h:171
// index:13
// Public Visibility=Default Availability=Available
// [16] QTextStream & operator>>(QByteArray &)

/*

*/
impl /*struct*/ QTextStream {
  pub fn operator_right_shift_13<RetType, T: QTextStream_operator_right_shift_13<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_right_shift_13(self);
    // return 1;
  }
}
pub trait QTextStream_operator_right_shift_13<RetType> {
  fn operator_right_shift_13(self , rsthis: & QTextStream) -> RetType;
}
impl<'a> /*trait*/ QTextStream_operator_right_shift_13<usize> for (usize) {
  fn operator_right_shift_13(self , rsthis: & QTextStream) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QTextStreamrsER10QByteArray", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtextstream.h:172
// index:14
// Public Visibility=Default Availability=Available
// [16] QTextStream & operator>>(char *)

/*

*/
impl /*struct*/ QTextStream {
  pub fn operator_right_shift_14<RetType, T: QTextStream_operator_right_shift_14<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_right_shift_14(self);
    // return 1;
  }
}
pub trait QTextStream_operator_right_shift_14<RetType> {
  fn operator_right_shift_14(self , rsthis: & QTextStream) -> RetType;
}
impl<'a> /*trait*/ QTextStream_operator_right_shift_14<usize> for (usize) {
  fn operator_right_shift_14(self , rsthis: & QTextStream) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QTextStreamrsEPc", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtextstream.h:174
// index:0
// Public Visibility=Default Availability=Available
// [16] QTextStream & operator<<(QChar)

/*

*/
impl /*struct*/ QTextStream {
  pub fn operator_left_shift_0<RetType, T: QTextStream_operator_left_shift_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_left_shift_0(self);
    // return 1;
  }
}
pub trait QTextStream_operator_left_shift_0<RetType> {
  fn operator_left_shift_0(self , rsthis: & QTextStream) -> RetType;
}
impl<'a> /*trait*/ QTextStream_operator_left_shift_0<usize> for (usize) {
  fn operator_left_shift_0(self , rsthis: & QTextStream) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QTextStreamlsE5QChar", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtextstream.h:175
// index:1
// Public Visibility=Default Availability=Available
// [16] QTextStream & operator<<(char)

/*

*/
impl /*struct*/ QTextStream {
  pub fn operator_left_shift_1<RetType, T: QTextStream_operator_left_shift_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_left_shift_1(self);
    // return 1;
  }
}
pub trait QTextStream_operator_left_shift_1<RetType> {
  fn operator_left_shift_1(self , rsthis: & QTextStream) -> RetType;
}
impl<'a> /*trait*/ QTextStream_operator_left_shift_1<usize> for (i8) {
  fn operator_left_shift_1(self , rsthis: & QTextStream) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i8 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QTextStreamlsEc", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtextstream.h:176
// index:2
// Public Visibility=Default Availability=Available
// [16] QTextStream & operator<<(short)

/*

*/
impl /*struct*/ QTextStream {
  pub fn operator_left_shift_2<RetType, T: QTextStream_operator_left_shift_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_left_shift_2(self);
    // return 1;
  }
}
pub trait QTextStream_operator_left_shift_2<RetType> {
  fn operator_left_shift_2(self , rsthis: & QTextStream) -> RetType;
}
impl<'a> /*trait*/ QTextStream_operator_left_shift_2<usize> for (i16) {
  fn operator_left_shift_2(self , rsthis: & QTextStream) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i16 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QTextStreamlsEs", 1,qtrt::FFITY_SINT16,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtextstream.h:177
// index:3
// Public Visibility=Default Availability=Available
// [16] QTextStream & operator<<(unsigned short)

/*

*/
impl /*struct*/ QTextStream {
  pub fn operator_left_shift_3<RetType, T: QTextStream_operator_left_shift_3<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_left_shift_3(self);
    // return 1;
  }
}
pub trait QTextStream_operator_left_shift_3<RetType> {
  fn operator_left_shift_3(self , rsthis: & QTextStream) -> RetType;
}
impl<'a> /*trait*/ QTextStream_operator_left_shift_3<usize> for (u16) {
  fn operator_left_shift_3(self , rsthis: & QTextStream) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const u16 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QTextStreamlsEt", 1,qtrt::FFITY_UINT16,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtextstream.h:178
// index:4
// Public Visibility=Default Availability=Available
// [16] QTextStream & operator<<(int)

/*

*/
impl /*struct*/ QTextStream {
  pub fn operator_left_shift_4<RetType, T: QTextStream_operator_left_shift_4<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_left_shift_4(self);
    // return 1;
  }
}
pub trait QTextStream_operator_left_shift_4<RetType> {
  fn operator_left_shift_4(self , rsthis: & QTextStream) -> RetType;
}
impl<'a> /*trait*/ QTextStream_operator_left_shift_4<usize> for (i32) {
  fn operator_left_shift_4(self , rsthis: & QTextStream) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QTextStreamlsEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtextstream.h:179
// index:5
// Public Visibility=Default Availability=Available
// [16] QTextStream & operator<<(unsigned int)

/*

*/
impl /*struct*/ QTextStream {
  pub fn operator_left_shift_5<RetType, T: QTextStream_operator_left_shift_5<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_left_shift_5(self);
    // return 1;
  }
}
pub trait QTextStream_operator_left_shift_5<RetType> {
  fn operator_left_shift_5(self , rsthis: & QTextStream) -> RetType;
}
impl<'a> /*trait*/ QTextStream_operator_left_shift_5<usize> for (u32) {
  fn operator_left_shift_5(self , rsthis: & QTextStream) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const u32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QTextStreamlsEj", 1,qtrt::FFITY_UINT32,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtextstream.h:180
// index:6
// Public Visibility=Default Availability=Available
// [16] QTextStream & operator<<(long)

/*

*/
impl /*struct*/ QTextStream {
  pub fn operator_left_shift_6<RetType, T: QTextStream_operator_left_shift_6<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_left_shift_6(self);
    // return 1;
  }
}
pub trait QTextStream_operator_left_shift_6<RetType> {
  fn operator_left_shift_6(self , rsthis: & QTextStream) -> RetType;
}
impl<'a> /*trait*/ QTextStream_operator_left_shift_6<usize> for (i64) {
  fn operator_left_shift_6(self , rsthis: & QTextStream) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QTextStreamlsEl", 1,qtrt::FFITY_SINT64,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtextstream.h:181
// index:7
// Public Visibility=Default Availability=Available
// [16] QTextStream & operator<<(unsigned long)

/*

*/
impl /*struct*/ QTextStream {
  pub fn operator_left_shift_7<RetType, T: QTextStream_operator_left_shift_7<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_left_shift_7(self);
    // return 1;
  }
}
pub trait QTextStream_operator_left_shift_7<RetType> {
  fn operator_left_shift_7(self , rsthis: & QTextStream) -> RetType;
}
impl<'a> /*trait*/ QTextStream_operator_left_shift_7<usize> for (u64) {
  fn operator_left_shift_7(self , rsthis: & QTextStream) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const u64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QTextStreamlsEm", 1,qtrt::FFITY_UINT64,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtextstream.h:182
// index:8
// Public Visibility=Default Availability=Available
// [16] QTextStream & operator<<(qlonglong)

/*

*/
impl /*struct*/ QTextStream {
  pub fn operator_left_shift_8<RetType, T: QTextStream_operator_left_shift_8<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_left_shift_8(self);
    // return 1;
  }
}
pub trait QTextStream_operator_left_shift_8<RetType> {
  fn operator_left_shift_8(self , rsthis: & QTextStream) -> RetType;
}
impl<'a> /*trait*/ QTextStream_operator_left_shift_8<usize> for (i64) {
  fn operator_left_shift_8(self , rsthis: & QTextStream) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QTextStreamlsEx", 1,qtrt::FFITY_SINT64,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtextstream.h:183
// index:9
// Public Visibility=Default Availability=Available
// [16] QTextStream & operator<<(qulonglong)

/*

*/
impl /*struct*/ QTextStream {
  pub fn operator_left_shift_9<RetType, T: QTextStream_operator_left_shift_9<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_left_shift_9(self);
    // return 1;
  }
}
pub trait QTextStream_operator_left_shift_9<RetType> {
  fn operator_left_shift_9(self , rsthis: & QTextStream) -> RetType;
}
impl<'a> /*trait*/ QTextStream_operator_left_shift_9<usize> for (u64) {
  fn operator_left_shift_9(self , rsthis: & QTextStream) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const u64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QTextStreamlsEy", 1,qtrt::FFITY_UINT64,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtextstream.h:184
// index:10
// Public Visibility=Default Availability=Available
// [16] QTextStream & operator<<(float)

/*

*/
impl /*struct*/ QTextStream {
  pub fn operator_left_shift_10<RetType, T: QTextStream_operator_left_shift_10<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_left_shift_10(self);
    // return 1;
  }
}
pub trait QTextStream_operator_left_shift_10<RetType> {
  fn operator_left_shift_10(self , rsthis: & QTextStream) -> RetType;
}
impl<'a> /*trait*/ QTextStream_operator_left_shift_10<usize> for (f32) {
  fn operator_left_shift_10(self , rsthis: & QTextStream) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QTextStreamlsEf", 1,qtrt::FFITY_FLOAT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtextstream.h:185
// index:11
// Public Visibility=Default Availability=Available
// [16] QTextStream & operator<<(double)

/*

*/
impl /*struct*/ QTextStream {
  pub fn operator_left_shift_11<RetType, T: QTextStream_operator_left_shift_11<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_left_shift_11(self);
    // return 1;
  }
}
pub trait QTextStream_operator_left_shift_11<RetType> {
  fn operator_left_shift_11(self , rsthis: & QTextStream) -> RetType;
}
impl<'a> /*trait*/ QTextStream_operator_left_shift_11<usize> for (f64) {
  fn operator_left_shift_11(self , rsthis: & QTextStream) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QTextStreamlsEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtextstream.h:186
// index:12
// Public Visibility=Default Availability=Available
// [16] QTextStream & operator<<(const QString &)

/*

*/
impl /*struct*/ QTextStream {
  pub fn operator_left_shift_12<RetType, T: QTextStream_operator_left_shift_12<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_left_shift_12(self);
    // return 1;
  }
}
pub trait QTextStream_operator_left_shift_12<RetType> {
  fn operator_left_shift_12(self , rsthis: & QTextStream) -> RetType;
}
impl<'a> /*trait*/ QTextStream_operator_left_shift_12<usize> for (usize) {
  fn operator_left_shift_12(self , rsthis: & QTextStream) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QTextStreamlsERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtextstream.h:187
// index:13
// Public Visibility=Default Availability=Available
// [16] QTextStream & operator<<(QLatin1String)

/*

*/
impl /*struct*/ QTextStream {
  pub fn operator_left_shift_13<RetType, T: QTextStream_operator_left_shift_13<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_left_shift_13(self);
    // return 1;
  }
}
pub trait QTextStream_operator_left_shift_13<RetType> {
  fn operator_left_shift_13(self , rsthis: & QTextStream) -> RetType;
}
impl<'a> /*trait*/ QTextStream_operator_left_shift_13<usize> for (usize) {
  fn operator_left_shift_13(self , rsthis: & QTextStream) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QTextStreamlsE13QLatin1String", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtextstream.h:188
// index:14
// Public Visibility=Default Availability=Available
// [16] QTextStream & operator<<(const QStringRef &)

/*

*/
impl /*struct*/ QTextStream {
  pub fn operator_left_shift_14<RetType, T: QTextStream_operator_left_shift_14<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_left_shift_14(self);
    // return 1;
  }
}
pub trait QTextStream_operator_left_shift_14<RetType> {
  fn operator_left_shift_14(self , rsthis: & QTextStream) -> RetType;
}
impl<'a> /*trait*/ QTextStream_operator_left_shift_14<usize> for (usize) {
  fn operator_left_shift_14(self , rsthis: & QTextStream) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QTextStreamlsERK10QStringRef", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtextstream.h:189
// index:15
// Public Visibility=Default Availability=Available
// [16] QTextStream & operator<<(const QByteArray &)

/*

*/
impl /*struct*/ QTextStream {
  pub fn operator_left_shift_15<RetType, T: QTextStream_operator_left_shift_15<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_left_shift_15(self);
    // return 1;
  }
}
pub trait QTextStream_operator_left_shift_15<RetType> {
  fn operator_left_shift_15(self , rsthis: & QTextStream) -> RetType;
}
impl<'a> /*trait*/ QTextStream_operator_left_shift_15<usize> for (usize) {
  fn operator_left_shift_15(self , rsthis: & QTextStream) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QTextStreamlsERK10QByteArray", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtextstream.h:190
// index:16
// Public Visibility=Default Availability=Available
// [16] QTextStream & operator<<(const char *)

/*

*/
impl /*struct*/ QTextStream {
  pub fn operator_left_shift_16<RetType, T: QTextStream_operator_left_shift_16<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_left_shift_16(self);
    // return 1;
  }
}
pub trait QTextStream_operator_left_shift_16<RetType> {
  fn operator_left_shift_16(self , rsthis: & QTextStream) -> RetType;
}
impl<'a> /*trait*/ QTextStream_operator_left_shift_16<usize> for (usize) {
  fn operator_left_shift_16(self , rsthis: & QTextStream) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QTextStreamlsEPKc", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtextstream.h:191
// index:17
// Public Visibility=Default Availability=Available
// [16] QTextStream & operator<<(const void *)

/*

*/
impl /*struct*/ QTextStream {
  pub fn operator_left_shift_17<RetType, T: QTextStream_operator_left_shift_17<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_left_shift_17(self);
    // return 1;
  }
}
pub trait QTextStream_operator_left_shift_17<RetType> {
  fn operator_left_shift_17(self , rsthis: & QTextStream) -> RetType;
}
impl<'a> /*trait*/ QTextStream_operator_left_shift_17<usize> for (usize) {
  fn operator_left_shift_17(self , rsthis: & QTextStream) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QTextStreamlsEPKv", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}


/*
This enum specifies which notations to use for expressing float and double as strings.



See also setRealNumberNotation().

*/
pub type QTextStream__RealNumberNotation = i32;
// Scientific or fixed-point notation, depending on which makes most sense (printf()'s %g flag).
pub const QTextStream__SmartNotation :QTextStream__RealNumberNotation = 0;
// Fixed-point notation (printf()'s %f flag).
pub const QTextStream__FixedNotation :QTextStream__RealNumberNotation = 1;
// Scientific notation (printf()'s %e flag).
pub const QTextStream__ScientificNotation :QTextStream__RealNumberNotation = 2;
pub fn QTextStream_RealNumberNotationItemName(val: i32) ->String {
  match val {
     QTextStream__SmartNotation => // 0
     {return String::from("SmartNotation");}
     QTextStream__FixedNotation => // 1
     {return String::from("FixedNotation");}
     QTextStream__ScientificNotation => // 2
     {return String::from("ScientificNotation");}
  _ => {return format!("{}", val);}
}
}
pub fn QTextStream_RealNumberNotationItemName_s(val: i32) ->String {
  //var nilthis *QTextStream
  //return nilthis.RealNumberNotationItemName(val);
  return QTextStream_RealNumberNotationItemName(val);
}


/*
This enum specifies how to align text in fields when the field is wider than the text that occupies it.



See also setFieldAlignment().

*/
pub type QTextStream__FieldAlignment = i32;
// Pad on the right side of fields.
pub const QTextStream__AlignLeft :QTextStream__FieldAlignment = 0;
// Pad on the left side of fields.
pub const QTextStream__AlignRight :QTextStream__FieldAlignment = 1;
// Pad on both sides of field.
pub const QTextStream__AlignCenter :QTextStream__FieldAlignment = 2;
// Same as AlignRight, except that the sign of a number is flush left.
pub const QTextStream__AlignAccountingStyle :QTextStream__FieldAlignment = 3;
pub fn QTextStream_FieldAlignmentItemName(val: i32) ->String {
  match val {
     QTextStream__AlignLeft => // 0
     {return String::from("AlignLeft");}
     QTextStream__AlignRight => // 1
     {return String::from("AlignRight");}
     QTextStream__AlignCenter => // 2
     {return String::from("AlignCenter");}
     QTextStream__AlignAccountingStyle => // 3
     {return String::from("AlignAccountingStyle");}
  _ => {return format!("{}", val);}
}
}
pub fn QTextStream_FieldAlignmentItemName_s(val: i32) ->String {
  //var nilthis *QTextStream
  //return nilthis.FieldAlignmentItemName(val);
  return QTextStream_FieldAlignmentItemName(val);
}


/*
This enum describes the current status of the text stream.



See also status().

*/
pub type QTextStream__Status = i32;
// The text stream is operating normally.
pub const QTextStream__Ok :QTextStream__Status = 0;
// The text stream has read past the end of the data in the underlying device.
pub const QTextStream__ReadPastEnd :QTextStream__Status = 1;
// The text stream has read corrupt data.
pub const QTextStream__ReadCorruptData :QTextStream__Status = 2;
// The text stream cannot write to the underlying device.
pub const QTextStream__WriteFailed :QTextStream__Status = 3;
pub fn QTextStream_StatusItemName(val: i32) ->String {
  match val {
     QTextStream__Ok => // 0
     {return String::from("Ok");}
     QTextStream__ReadPastEnd => // 1
     {return String::from("ReadPastEnd");}
     QTextStream__ReadCorruptData => // 2
     {return String::from("ReadCorruptData");}
     QTextStream__WriteFailed => // 3
     {return String::from("WriteFailed");}
  _ => {return format!("{}", val);}
}
}
pub fn QTextStream_StatusItemName_s(val: i32) ->String {
  //var nilthis *QTextStream
  //return nilthis.StatusItemName(val);
  return QTextStream_StatusItemName(val);
}


/*


*/
pub type QTextStream__NumberFlag = i32;
// 
pub const QTextStream__ShowBase :QTextStream__NumberFlag = 1;
// 
pub const QTextStream__ForcePoint :QTextStream__NumberFlag = 2;
// 
pub const QTextStream__ForceSign :QTextStream__NumberFlag = 4;
// 
pub const QTextStream__UppercaseBase :QTextStream__NumberFlag = 8;
// 
pub const QTextStream__UppercaseDigits :QTextStream__NumberFlag = 16;
pub fn QTextStream_NumberFlagItemName(val: i32) ->String {
  match val {
     QTextStream__ShowBase => // 1
     {return String::from("ShowBase");}
     QTextStream__ForcePoint => // 2
     {return String::from("ForcePoint");}
     QTextStream__ForceSign => // 4
     {return String::from("ForceSign");}
     QTextStream__UppercaseBase => // 8
     {return String::from("UppercaseBase");}
     QTextStream__UppercaseDigits => // 16
     {return String::from("UppercaseDigits");}
  _ => {return format!("{}", val);}
}
}
pub fn QTextStream_NumberFlagItemName_s(val: i32) ->String {
  //var nilthis *QTextStream
  //return nilthis.NumberFlagItemName(val);
  return QTextStream_NumberFlagItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
