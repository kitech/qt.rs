

// mod ::gui::QImageWriter
// package qtgui
// /usr/include/qt/QtGui/qimagewriter.h
// #include <qimagewriter.h>
// #include <QtGui>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 50
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
#[derive(Default)] // class sizeof(QImageWriter)=8
pub struct QImageWriter {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QImageWriter_ITF interface {
//    QImageWriter_PTR() *QImageWriter
//}
//func (ptr *QImageWriter) QImageWriter_PTR() *QImageWriter { return ptr }

impl /*struct*/ QImageWriter {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QImageWriter {
    return QImageWriter{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QImageWriter {
//  type Target = QImageWriterBASE;
//
//  fn deref(&self) -> &QImageWriterBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QImageWriterBASE> for QImageWriter {
//  fn as_ref(& self) -> & QImageWriterBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qimagewriter.h:67
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QImageWriter()

/*
Constructs an empty QImageWriter object. Before writing, you must call setFormat() to set an image format, then setDevice() or setFileName().
*/
// QImageWriter() ctx.fn_proto_cpp
impl /*struct*/ QImageWriter {
  pub fn QImageWriter_0<T: QImageWriter_QImageWriter_0>(value: T) -> QImageWriter {
    let rsthis = value.QImageWriter_0();
    return rsthis;
    // return 1;
  }
}

pub trait QImageWriter_QImageWriter_0 {
  fn QImageWriter_0(self) -> QImageWriter;
}
// QImageWriter() ctx.fn_proto_cpp
impl<'a> /*trait*/ QImageWriter_QImageWriter_0 for () {
  fn QImageWriter_0(self) -> QImageWriter {
    // unsafe{_ZN12QImageWriterC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN12QImageWriterC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QImageWriter{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qimagewriter.h:68
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QImageWriter(QIODevice *, const QByteArray &)

/*
Constructs an empty QImageWriter object. Before writing, you must call setFormat() to set an image format, then setDevice() or setFileName().
*/
// QImageWriter(QIODevice *, const QByteArray &) ctx.fn_proto_cpp
impl /*struct*/ QImageWriter {
  pub fn QImageWriter_1<T: QImageWriter_QImageWriter_1>(value: T) -> QImageWriter {
    let rsthis = value.QImageWriter_1();
    return rsthis;
    // return 1;
  }
}

pub trait QImageWriter_QImageWriter_1 {
  fn QImageWriter_1(self) -> QImageWriter;
}
// QImageWriter(QIODevice *, const QByteArray &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QImageWriter_QImageWriter_1 for (usize,usize) {
  fn QImageWriter_1(self) -> QImageWriter {
    // unsafe{_ZN12QImageWriterC2EP9QIODeviceRK10QByteArray()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN12QImageWriterC2EP9QIODeviceRK10QByteArray", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QImageWriter{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qimagewriter.h:69
// index:2
// Public Visibility=Default Availability=Available
// [-2] void QImageWriter(const QString &, const QByteArray &)

/*
Constructs an empty QImageWriter object. Before writing, you must call setFormat() to set an image format, then setDevice() or setFileName().
*/
// QImageWriter(const QString &, const QByteArray &) ctx.fn_proto_cpp
impl /*struct*/ QImageWriter {
  pub fn QImageWriter_2<T: QImageWriter_QImageWriter_2>(value: T) -> QImageWriter {
    let rsthis = value.QImageWriter_2();
    return rsthis;
    // return 1;
  }
}

pub trait QImageWriter_QImageWriter_2 {
  fn QImageWriter_2(self) -> QImageWriter;
}
// QImageWriter(const QString &, const QByteArray &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QImageWriter_QImageWriter_2 for (usize,usize) {
  fn QImageWriter_2(self) -> QImageWriter {
    // unsafe{_ZN12QImageWriterC2ERK7QStringRK10QByteArray()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN12QImageWriterC2ERK7QStringRK10QByteArray", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QImageWriter{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qimagewriter.h:70
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ~QImageWriter()

/*

*/
pub fn DeleteQImageWriter(this :*mut QImageWriter) {
    // let rv = qtrt::InvokeQtFunc6("_ZN12QImageWriterD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 8)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qimagewriter.h:72
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFormat(const QByteArray &)

/*
Sets the format QImageWriter will use when writing images, to format. format is a case insensitive text string. Example:


  QImageWriter writer;
  writer.setFormat("png"); // same as writer.setFormat("PNG");



You can call supportedImageFormats() for the full list of formats QImageWriter supports.

See also format().
*/
impl /*struct*/ QImageWriter {
  pub fn setFormat_0<RetType, T: QImageWriter_setFormat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFormat_0(self);
    // return 1;
  }
}
pub trait QImageWriter_setFormat_0<RetType> {
  fn setFormat_0(self , rsthis: & QImageWriter) -> RetType;
}
impl<'a> /*trait*/ QImageWriter_setFormat_0<(/*void*/)> for (usize) {
  fn setFormat_0(self , rsthis: & QImageWriter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QImageWriter9setFormatERK10QByteArray", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qimagewriter.h:73
// index:0
// Public Visibility=Default Availability=Available
// [8] QByteArray format() const

/*
Returns the format QImageWriter uses for writing images.

See also setFormat().
*/
impl /*struct*/ QImageWriter {
  pub fn format_0<RetType, T: QImageWriter_format_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.format_0(self);
    // return 1;
  }
}
pub trait QImageWriter_format_0<RetType> {
  fn format_0(self , rsthis: & QImageWriter) -> RetType;
}
impl<'a> /*trait*/ QImageWriter_format_0<usize> for () {
  fn format_0(self , rsthis: & QImageWriter) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QImageWriter6formatEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimagewriter.h:75
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDevice(QIODevice *)

/*
Sets QImageWriter's device to device. If a device has already been set, the old device is removed from QImageWriter and is otherwise left unchanged.

If the device is not already open, QImageWriter will attempt to open the device in QIODevice::WriteOnly mode by calling open(). Note that this does not work for certain devices, such as QProcess, QTcpSocket and QUdpSocket, where more logic is required to open the device.

See also device() and setFileName().
*/
impl /*struct*/ QImageWriter {
  pub fn setDevice_0<RetType, T: QImageWriter_setDevice_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDevice_0(self);
    // return 1;
  }
}
pub trait QImageWriter_setDevice_0<RetType> {
  fn setDevice_0(self , rsthis: & QImageWriter) -> RetType;
}
impl<'a> /*trait*/ QImageWriter_setDevice_0<(/*void*/)> for (usize) {
  fn setDevice_0(self , rsthis: & QImageWriter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QImageWriter9setDeviceEP9QIODevice", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qimagewriter.h:76
// index:0
// Public Visibility=Default Availability=Available
// [8] QIODevice * device() const

/*
Returns the device currently assigned to QImageWriter, or 0 if no device has been assigned.

See also setDevice().
*/
impl /*struct*/ QImageWriter {
  pub fn device_0<RetType, T: QImageWriter_device_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.device_0(self);
    // return 1;
  }
}
pub trait QImageWriter_device_0<RetType> {
  fn device_0(self , rsthis: & QImageWriter) -> RetType;
}
impl<'a> /*trait*/ QImageWriter_device_0<usize> for () {
  fn device_0(self , rsthis: & QImageWriter) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QImageWriter6deviceEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimagewriter.h:78
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFileName(const QString &)

/*
Sets the file name of QImageWriter to fileName. Internally, QImageWriter will create a QFile and open it in QIODevice::WriteOnly mode, and use this file when writing images.

See also fileName() and setDevice().
*/
impl /*struct*/ QImageWriter {
  pub fn setFileName_0<RetType, T: QImageWriter_setFileName_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFileName_0(self);
    // return 1;
  }
}
pub trait QImageWriter_setFileName_0<RetType> {
  fn setFileName_0(self , rsthis: & QImageWriter) -> RetType;
}
impl<'a> /*trait*/ QImageWriter_setFileName_0<(/*void*/)> for (usize) {
  fn setFileName_0(self , rsthis: & QImageWriter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QImageWriter11setFileNameERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qimagewriter.h:79
// index:0
// Public Visibility=Default Availability=Available
// [8] QString fileName() const

/*
If the currently assigned device is a QFile, or if setFileName() has been called, this function returns the name of the file QImageWriter writes to. Otherwise (i.e., if no device has been assigned or the device is not a QFile), an empty QString is returned.

See also setFileName() and setDevice().
*/
impl /*struct*/ QImageWriter {
  pub fn fileName_0<RetType, T: QImageWriter_fileName_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fileName_0(self);
    // return 1;
  }
}
pub trait QImageWriter_fileName_0<RetType> {
  fn fileName_0(self , rsthis: & QImageWriter) -> RetType;
}
impl<'a> /*trait*/ QImageWriter_fileName_0<usize> for () {
  fn fileName_0(self , rsthis: & QImageWriter) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QImageWriter8fileNameEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimagewriter.h:81
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setQuality(int)

/*
Sets the quality setting of the image format to quality.

Some image formats, in particular lossy ones, entail a tradeoff between a) visual quality of the resulting image, and b) encoding execution time and compression level. This function sets the level of that tradeoff for image formats that support it. For other formats, this value is ignored.

The value range of quality depends on the image format. For example, the "jpeg" format supports a quality range from 0 (low visual quality, high compression) to 100 (high visual quality, low compression).

See also quality().
*/
impl /*struct*/ QImageWriter {
  pub fn setQuality_0<RetType, T: QImageWriter_setQuality_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setQuality_0(self);
    // return 1;
  }
}
pub trait QImageWriter_setQuality_0<RetType> {
  fn setQuality_0(self , rsthis: & QImageWriter) -> RetType;
}
impl<'a> /*trait*/ QImageWriter_setQuality_0<(/*void*/)> for (i32) {
  fn setQuality_0(self , rsthis: & QImageWriter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN12QImageWriter10setQualityEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qimagewriter.h:82
// index:0
// Public Visibility=Default Availability=Available
// [4] int quality() const

/*
Returns the quality setting of the image format.

See also setQuality().
*/
impl /*struct*/ QImageWriter {
  pub fn quality_0<RetType, T: QImageWriter_quality_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.quality_0(self);
    // return 1;
  }
}
pub trait QImageWriter_quality_0<RetType> {
  fn quality_0(self , rsthis: & QImageWriter) -> RetType;
}
impl<'a> /*trait*/ QImageWriter_quality_0<i32> for () {
  fn quality_0(self , rsthis: & QImageWriter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QImageWriter7qualityEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimagewriter.h:84
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCompression(int)

/*
This is an image format specific function that set the compression of an image. For image formats that do not support setting the compression, this value is ignored.

The value range of compression depends on the image format. For example, the "tiff" format supports two values, 0(no compression) and 1(LZW-compression).

See also compression().
*/
impl /*struct*/ QImageWriter {
  pub fn setCompression_0<RetType, T: QImageWriter_setCompression_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCompression_0(self);
    // return 1;
  }
}
pub trait QImageWriter_setCompression_0<RetType> {
  fn setCompression_0(self , rsthis: & QImageWriter) -> RetType;
}
impl<'a> /*trait*/ QImageWriter_setCompression_0<(/*void*/)> for (i32) {
  fn setCompression_0(self , rsthis: & QImageWriter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN12QImageWriter14setCompressionEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qimagewriter.h:85
// index:0
// Public Visibility=Default Availability=Available
// [4] int compression() const

/*
Returns the compression of the image.

See also setCompression().
*/
impl /*struct*/ QImageWriter {
  pub fn compression_0<RetType, T: QImageWriter_compression_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.compression_0(self);
    // return 1;
  }
}
pub trait QImageWriter_compression_0<RetType> {
  fn compression_0(self , rsthis: & QImageWriter) -> RetType;
}
impl<'a> /*trait*/ QImageWriter_compression_0<i32> for () {
  fn compression_0(self , rsthis: & QImageWriter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QImageWriter11compressionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimagewriter.h:87
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setGamma(float)

/*
This is an image format specific function that sets the gamma level of the image to gamma. For image formats that do not support setting the gamma level, this value is ignored.

The value range of gamma depends on the image format. For example, the "png" format supports a gamma range from 0.0 to 1.0.

See also gamma() and quality().
*/
impl /*struct*/ QImageWriter {
  pub fn setGamma_0<RetType, T: QImageWriter_setGamma_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setGamma_0(self);
    // return 1;
  }
}
pub trait QImageWriter_setGamma_0<RetType> {
  fn setGamma_0(self , rsthis: & QImageWriter) -> RetType;
}
impl<'a> /*trait*/ QImageWriter_setGamma_0<(/*void*/)> for (f32) {
  fn setGamma_0(self , rsthis: & QImageWriter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f32 as usize;
     qtrt::InvokeQtFunc6("_ZN12QImageWriter8setGammaEf", 1,qtrt::FFITY_FLOAT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qimagewriter.h:88
// index:0
// Public Visibility=Default Availability=Available
// [4] float gamma() const

/*
Returns the gamma level of the image.

See also setGamma().
*/
impl /*struct*/ QImageWriter {
  pub fn gamma_0<RetType, T: QImageWriter_gamma_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.gamma_0(self);
    // return 1;
  }
}
pub trait QImageWriter_gamma_0<RetType> {
  fn gamma_0(self , rsthis: & QImageWriter) -> RetType;
}
impl<'a> /*trait*/ QImageWriter_gamma_0<f32> for () {
  fn gamma_0(self , rsthis: & QImageWriter) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QImageWriter5gammaEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimagewriter.h:90
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setSubType(const QByteArray &)

/*
This is an image format specific function that sets the subtype of the image to type. Subtype can be used by a handler to determine which format it should use while saving the image.

For example, saving an image in DDS format with A8R8G8R8 subtype:


  QImageWriter writer("some/image.dds");
  if (writer.supportsOption(QImageIOHandler::SubType))
      writer.setSubType("A8R8G8B8");
  writer.write(image);



This function was introduced in  Qt 5.4.

See also subType().
*/
impl /*struct*/ QImageWriter {
  pub fn setSubType_0<RetType, T: QImageWriter_setSubType_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSubType_0(self);
    // return 1;
  }
}
pub trait QImageWriter_setSubType_0<RetType> {
  fn setSubType_0(self , rsthis: & QImageWriter) -> RetType;
}
impl<'a> /*trait*/ QImageWriter_setSubType_0<(/*void*/)> for (usize) {
  fn setSubType_0(self , rsthis: & QImageWriter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QImageWriter10setSubTypeERK10QByteArray", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qimagewriter.h:91
// index:0
// Public Visibility=Default Availability=Available
// [8] QByteArray subType() const

/*
Returns the subtype of the image.

This function was introduced in  Qt 5.4.

See also setSubType().
*/
impl /*struct*/ QImageWriter {
  pub fn subType_0<RetType, T: QImageWriter_subType_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.subType_0(self);
    // return 1;
  }
}
pub trait QImageWriter_subType_0<RetType> {
  fn subType_0(self , rsthis: & QImageWriter) -> RetType;
}
impl<'a> /*trait*/ QImageWriter_subType_0<usize> for () {
  fn subType_0(self , rsthis: & QImageWriter) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QImageWriter7subTypeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimagewriter.h:94
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setOptimizedWrite(bool)

/*
This is an image format-specific function which sets the optimize flags when writing images. For image formats that do not support setting an optimize flag, this value is ignored.

The default is false.

This function was introduced in  Qt 5.5.

See also optimizedWrite().
*/
impl /*struct*/ QImageWriter {
  pub fn setOptimizedWrite_0<RetType, T: QImageWriter_setOptimizedWrite_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setOptimizedWrite_0(self);
    // return 1;
  }
}
pub trait QImageWriter_setOptimizedWrite_0<RetType> {
  fn setOptimizedWrite_0(self , rsthis: & QImageWriter) -> RetType;
}
impl<'a> /*trait*/ QImageWriter_setOptimizedWrite_0<(/*void*/)> for (bool) {
  fn setOptimizedWrite_0(self , rsthis: & QImageWriter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN12QImageWriter17setOptimizedWriteEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qimagewriter.h:95
// index:0
// Public Visibility=Default Availability=Available
// [1] bool optimizedWrite() const

/*
Returns whether optimization has been turned on for writing the image.

This function was introduced in  Qt 5.5.

See also setOptimizedWrite().
*/
impl /*struct*/ QImageWriter {
  pub fn optimizedWrite_0<RetType, T: QImageWriter_optimizedWrite_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.optimizedWrite_0(self);
    // return 1;
  }
}
pub trait QImageWriter_optimizedWrite_0<RetType> {
  fn optimizedWrite_0(self , rsthis: & QImageWriter) -> RetType;
}
impl<'a> /*trait*/ QImageWriter_optimizedWrite_0<bool> for () {
  fn optimizedWrite_0(self , rsthis: & QImageWriter) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QImageWriter14optimizedWriteEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimagewriter.h:97
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setProgressiveScanWrite(bool)

/*
This is an image format-specific function which turns on progressive scanning when writing images. For image formats that do not support setting a progressive scan flag, this value is ignored.

The default is false.

This function was introduced in  Qt 5.5.

See also progressiveScanWrite().
*/
impl /*struct*/ QImageWriter {
  pub fn setProgressiveScanWrite_0<RetType, T: QImageWriter_setProgressiveScanWrite_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setProgressiveScanWrite_0(self);
    // return 1;
  }
}
pub trait QImageWriter_setProgressiveScanWrite_0<RetType> {
  fn setProgressiveScanWrite_0(self , rsthis: & QImageWriter) -> RetType;
}
impl<'a> /*trait*/ QImageWriter_setProgressiveScanWrite_0<(/*void*/)> for (bool) {
  fn setProgressiveScanWrite_0(self , rsthis: & QImageWriter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN12QImageWriter23setProgressiveScanWriteEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qimagewriter.h:98
// index:0
// Public Visibility=Default Availability=Available
// [1] bool progressiveScanWrite() const

/*
Returns whether the image should be written as a progressive image.

This function was introduced in  Qt 5.5.

See also setProgressiveScanWrite().
*/
impl /*struct*/ QImageWriter {
  pub fn progressiveScanWrite_0<RetType, T: QImageWriter_progressiveScanWrite_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.progressiveScanWrite_0(self);
    // return 1;
  }
}
pub trait QImageWriter_progressiveScanWrite_0<RetType> {
  fn progressiveScanWrite_0(self , rsthis: & QImageWriter) -> RetType;
}
impl<'a> /*trait*/ QImageWriter_progressiveScanWrite_0<bool> for () {
  fn progressiveScanWrite_0(self , rsthis: & QImageWriter) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QImageWriter20progressiveScanWriteEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimagewriter.h:100
// index:0
// Public Visibility=Default Availability=Available
// [-2] QImageIOHandler::Transformations transformation() const

/*
Returns the transformation and orientation the image has been set to written with.

This function was introduced in  Qt 5.5.

See also setTransformation().
*/
impl /*struct*/ QImageWriter {
  pub fn transformation_0<RetType, T: QImageWriter_transformation_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.transformation_0(self);
    // return 1;
  }
}
pub trait QImageWriter_transformation_0<RetType> {
  fn transformation_0(self , rsthis: & QImageWriter) -> RetType;
}
impl<'a> /*trait*/ QImageWriter_transformation_0<i32> for () {
  fn transformation_0(self , rsthis: & QImageWriter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QImageWriter14transformationEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimagewriter.h:101
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTransformation(QImageIOHandler::Transformations)

/*
Sets the image transformations metadata including orientation to transform.

If transformation metadata is not supported by the image format, the transform is applied before writing.

This function was introduced in  Qt 5.5.

See also transformation() and write().
*/
impl /*struct*/ QImageWriter {
  pub fn setTransformation_0<RetType, T: QImageWriter_setTransformation_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTransformation_0(self);
    // return 1;
  }
}
pub trait QImageWriter_setTransformation_0<RetType> {
  fn setTransformation_0(self , rsthis: & QImageWriter) -> RetType;
}
impl<'a> /*trait*/ QImageWriter_setTransformation_0<(/*void*/)> for (i32) {
  fn setTransformation_0(self , rsthis: & QImageWriter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN12QImageWriter17setTransformationE6QFlagsIN15QImageIOHandler14TransformationEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qimagewriter.h:104
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDescription(const QString &)

/*

*/
impl /*struct*/ QImageWriter {
  pub fn setDescription_0<RetType, T: QImageWriter_setDescription_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDescription_0(self);
    // return 1;
  }
}
pub trait QImageWriter_setDescription_0<RetType> {
  fn setDescription_0(self , rsthis: & QImageWriter) -> RetType;
}
impl<'a> /*trait*/ QImageWriter_setDescription_0<(/*void*/)> for (usize) {
  fn setDescription_0(self , rsthis: & QImageWriter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QImageWriter14setDescriptionERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qimagewriter.h:105
// index:0
// Public Visibility=Default Availability=Available
// [8] QString description() const

/*

*/
impl /*struct*/ QImageWriter {
  pub fn description_0<RetType, T: QImageWriter_description_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.description_0(self);
    // return 1;
  }
}
pub trait QImageWriter_description_0<RetType> {
  fn description_0(self , rsthis: & QImageWriter) -> RetType;
}
impl<'a> /*trait*/ QImageWriter_description_0<usize> for () {
  fn description_0(self , rsthis: & QImageWriter) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QImageWriter11descriptionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimagewriter.h:107
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setText(const QString &, const QString &)

/*
Sets the image text associated with the key key to text. This is useful for storing copyright information or other information about the image. Example:


  QImage image("some/image.jpeg");
  QImageWriter writer("images/outimage.png", "png");
  writer.setText("Author", "John Smith");
  writer.write(image);



If you want to store a single block of data (e.g., a comment), you can pass an empty key, or use a generic key like "Description".

The key and text will be embedded into the image data after calling write().

Support for this option is implemented through QImageIOHandler::Description.

This function was introduced in  Qt 4.1.

See also QImage::setText() and QImageReader::text().
*/
impl /*struct*/ QImageWriter {
  pub fn setText_0<RetType, T: QImageWriter_setText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setText_0(self);
    // return 1;
  }
}
pub trait QImageWriter_setText_0<RetType> {
  fn setText_0(self , rsthis: & QImageWriter) -> RetType;
}
impl<'a> /*trait*/ QImageWriter_setText_0<(/*void*/)> for (usize,usize) {
  fn setText_0(self , rsthis: & QImageWriter) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QImageWriter7setTextERK7QStringS2_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qimagewriter.h:109
// index:0
// Public Visibility=Default Availability=Available
// [1] bool canWrite() const

/*
Returns true if QImageWriter can write the image; i.e., the image format is supported and the assigned device is open for reading.

See also write(), setDevice(), and setFormat().
*/
impl /*struct*/ QImageWriter {
  pub fn canWrite_0<RetType, T: QImageWriter_canWrite_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.canWrite_0(self);
    // return 1;
  }
}
pub trait QImageWriter_canWrite_0<RetType> {
  fn canWrite_0(self , rsthis: & QImageWriter) -> RetType;
}
impl<'a> /*trait*/ QImageWriter_canWrite_0<bool> for () {
  fn canWrite_0(self , rsthis: & QImageWriter) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QImageWriter8canWriteEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimagewriter.h:110
// index:0
// Public Visibility=Default Availability=Available
// [1] bool write(const QImage &)

/*
Writes the image image to the assigned device or file name. Returns true on success; otherwise returns false. If the operation fails, you can call error() to find the type of error that occurred, or errorString() to get a human readable description of the error.

See also canWrite(), error(), and errorString().
*/
impl /*struct*/ QImageWriter {
  pub fn write_0<RetType, T: QImageWriter_write_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.write_0(self);
    // return 1;
  }
}
pub trait QImageWriter_write_0<RetType> {
  fn write_0(self , rsthis: & QImageWriter) -> RetType;
}
impl<'a> /*trait*/ QImageWriter_write_0<bool> for (usize) {
  fn write_0(self , rsthis: & QImageWriter) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN12QImageWriter5writeERK6QImage", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimagewriter.h:112
// index:0
// Public Visibility=Default Availability=Available
// [4] QImageWriter::ImageWriterError error() const

/*
Returns the type of error that last occurred.

See also ImageWriterError and errorString().
*/
impl /*struct*/ QImageWriter {
  pub fn error_0<RetType, T: QImageWriter_error_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.error_0(self);
    // return 1;
  }
}
pub trait QImageWriter_error_0<RetType> {
  fn error_0(self , rsthis: & QImageWriter) -> RetType;
}
impl<'a> /*trait*/ QImageWriter_error_0<i32> for () {
  fn error_0(self , rsthis: & QImageWriter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QImageWriter5errorEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimagewriter.h:113
// index:0
// Public Visibility=Default Availability=Available
// [8] QString errorString() const

/*
Returns a human readable description of the last error that occurred.

See also error().
*/
impl /*struct*/ QImageWriter {
  pub fn errorString_0<RetType, T: QImageWriter_errorString_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.errorString_0(self);
    // return 1;
  }
}
pub trait QImageWriter_errorString_0<RetType> {
  fn errorString_0(self , rsthis: & QImageWriter) -> RetType;
}
impl<'a> /*trait*/ QImageWriter_errorString_0<usize> for () {
  fn errorString_0(self , rsthis: & QImageWriter) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QImageWriter11errorStringEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimagewriter.h:115
// index:0
// Public Visibility=Default Availability=Available
// [1] bool supportsOption(QImageIOHandler::ImageOption) const

/*
Returns true if the writer supports option; otherwise returns false.

Different image formats support different options. Call this function to determine whether a certain option is supported by the current format. For example, the PNG format allows you to embed text into the image's metadata (see text()).


  QImageWriter writer(fileName);
  if (writer.supportsOption(QImageIOHandler::Description))
      writer.setText("Author", "John Smith");



Options can be tested after the writer has been associated with a format.

This function was introduced in  Qt 4.2.

See also QImageReader::supportsOption() and setFormat().
*/
impl /*struct*/ QImageWriter {
  pub fn supportsOption_0<RetType, T: QImageWriter_supportsOption_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.supportsOption_0(self);
    // return 1;
  }
}
pub trait QImageWriter_supportsOption_0<RetType> {
  fn supportsOption_0(self , rsthis: & QImageWriter) -> RetType;
}
impl<'a> /*trait*/ QImageWriter_supportsOption_0<bool> for (i32) {
  fn supportsOption_0(self , rsthis: & QImageWriter) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QImageWriter14supportsOptionEN15QImageIOHandler11ImageOptionE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}


/*
This enum describes errors that can occur when writing images with QImageWriter.


*/
pub type QImageWriter__ImageWriterError = i32;
// An unknown error occurred. If you get this value after calling write(), it is most likely caused by a bug in QImageWriter.
pub const QImageWriter__UnknownError :QImageWriter__ImageWriterError = 0;
// QImageWriter encountered a device error when writing the image data. Consult your device for more details on what went wrong.
pub const QImageWriter__DeviceError :QImageWriter__ImageWriterError = 1;
// Qt does not support the requested image format.
pub const QImageWriter__UnsupportedFormatError :QImageWriter__ImageWriterError = 2;
// An attempt was made to write an invalid QImage. An example of an invalid image would be a null QImage.
pub const QImageWriter__InvalidImageError :QImageWriter__ImageWriterError = 3;
pub fn QImageWriter_ImageWriterErrorItemName(val: i32) ->String {
  match val {
     QImageWriter__UnknownError => // 0
     {return String::from("UnknownError");}
     QImageWriter__DeviceError => // 1
     {return String::from("DeviceError");}
     QImageWriter__UnsupportedFormatError => // 2
     {return String::from("UnsupportedFormatError");}
     QImageWriter__InvalidImageError => // 3
     {return String::from("InvalidImageError");}
  _ => {return format!("{}", val);}
}
}
pub fn QImageWriter_ImageWriterErrorItemName_s(val: i32) ->String {
  //var nilthis *QImageWriter
  //return nilthis.ImageWriterErrorItemName(val);
  return QImageWriter_ImageWriterErrorItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
