

// mod ::gui::QImageReader
// package qtgui
// /usr/include/qt/QtGui/qimagereader.h
// #include <qimagereader.h>
// #include <QtGui>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 5
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
#[derive(Default)] // class sizeof(QImageReader)=8
pub struct QImageReader {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QImageReader_ITF interface {
//    QImageReader_PTR() *QImageReader
//}
//func (ptr *QImageReader) QImageReader_PTR() *QImageReader { return ptr }

impl /*struct*/ QImageReader {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QImageReader {
    return QImageReader{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QImageReader {
//  type Target = QImageReaderBASE;
//
//  fn deref(&self) -> &QImageReaderBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QImageReaderBASE> for QImageReader {
//  fn as_ref(& self) -> & QImageReaderBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qimagereader.h:71
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QImageReader()

/*
Constructs an empty QImageReader object. Before reading an image, call setDevice() or setFileName().
*/
// QImageReader() ctx.fn_proto_cpp
impl /*struct*/ QImageReader {
  pub fn QImageReader_0<T: QImageReader_QImageReader_0>(value: T) -> QImageReader {
    let rsthis = value.QImageReader_0();
    return rsthis;
    // return 1;
  }
}

pub trait QImageReader_QImageReader_0 {
  fn QImageReader_0(self) -> QImageReader;
}
// QImageReader() ctx.fn_proto_cpp
impl<'a> /*trait*/ QImageReader_QImageReader_0 for () {
  fn QImageReader_0(self) -> QImageReader {
    // unsafe{_ZN12QImageReaderC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN12QImageReaderC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QImageReader{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qimagereader.h:72
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QImageReader(QIODevice *, const QByteArray &)

/*
Constructs an empty QImageReader object. Before reading an image, call setDevice() or setFileName().
*/
// QImageReader(QIODevice *, const QByteArray &) ctx.fn_proto_cpp
impl /*struct*/ QImageReader {
  pub fn QImageReader_1<T: QImageReader_QImageReader_1>(value: T) -> QImageReader {
    let rsthis = value.QImageReader_1();
    return rsthis;
    // return 1;
  }
}

pub trait QImageReader_QImageReader_1 {
  fn QImageReader_1(self) -> QImageReader;
}
// QImageReader(QIODevice *, const QByteArray &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QImageReader_QImageReader_1 for (usize,usize) {
  fn QImageReader_1(self) -> QImageReader {
    // unsafe{_ZN12QImageReaderC2EP9QIODeviceRK10QByteArray()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN12QImageReaderC2EP9QIODeviceRK10QByteArray", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QImageReader{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qimagereader.h:73
// index:2
// Public Visibility=Default Availability=Available
// [-2] void QImageReader(const QString &, const QByteArray &)

/*
Constructs an empty QImageReader object. Before reading an image, call setDevice() or setFileName().
*/
// QImageReader(const QString &, const QByteArray &) ctx.fn_proto_cpp
impl /*struct*/ QImageReader {
  pub fn QImageReader_2<T: QImageReader_QImageReader_2>(value: T) -> QImageReader {
    let rsthis = value.QImageReader_2();
    return rsthis;
    // return 1;
  }
}

pub trait QImageReader_QImageReader_2 {
  fn QImageReader_2(self) -> QImageReader;
}
// QImageReader(const QString &, const QByteArray &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QImageReader_QImageReader_2 for (usize,usize) {
  fn QImageReader_2(self) -> QImageReader {
    // unsafe{_ZN12QImageReaderC2ERK7QStringRK10QByteArray()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN12QImageReaderC2ERK7QStringRK10QByteArray", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QImageReader{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qimagereader.h:74
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ~QImageReader()

/*

*/
pub fn DeleteQImageReader(this :*mut QImageReader) {
    // let rv = qtrt::InvokeQtFunc6("_ZN12QImageReaderD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 8)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qimagereader.h:76
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFormat(const QByteArray &)

/*
Sets the format QImageReader will use when reading images, to format. format is a case insensitive text string. Example:


  QImageReader reader;
  reader.setFormat("png"); // same as reader.setFormat("PNG");



You can call supportedImageFormats() for the full list of formats QImageReader supports.

See also format().
*/
impl /*struct*/ QImageReader {
  pub fn setFormat_0<RetType, T: QImageReader_setFormat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFormat_0(self);
    // return 1;
  }
}
pub trait QImageReader_setFormat_0<RetType> {
  fn setFormat_0(self , rsthis: & QImageReader) -> RetType;
}
impl<'a> /*trait*/ QImageReader_setFormat_0<(/*void*/)> for (usize) {
  fn setFormat_0(self , rsthis: & QImageReader) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QImageReader9setFormatERK10QByteArray", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qimagereader.h:77
// index:0
// Public Visibility=Default Availability=Available
// [8] QByteArray format() const

/*
Returns the format QImageReader uses for reading images.

You can call this function after assigning a device to the reader to determine the format of the device. For example:


  QImageReader reader("image.png");
  // reader.format() == "png"



If the reader cannot read any image from the device (e.g., there is no image there, or the image has already been read), or if the format is unsupported, this function returns an empty QByteArray().

See also setFormat() and supportedImageFormats().
*/
impl /*struct*/ QImageReader {
  pub fn format_0<RetType, T: QImageReader_format_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.format_0(self);
    // return 1;
  }
}
pub trait QImageReader_format_0<RetType> {
  fn format_0(self , rsthis: & QImageReader) -> RetType;
}
impl<'a> /*trait*/ QImageReader_format_0<usize> for () {
  fn format_0(self , rsthis: & QImageReader) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QImageReader6formatEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimagereader.h:79
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setAutoDetectImageFormat(bool)

/*
If enabled is true, image format autodetection is enabled; otherwise, it is disabled. By default, autodetection is enabled.

QImageReader uses an extensive approach to detecting the image format; firstly, if you pass a file name to QImageReader, it will attempt to detect the file extension if the given file name does not point to an existing file, by appending supported default extensions to the given file name, one at a time. It then uses the following approach to detect the image format:


Image plugins are queried first, based on either the optional format string, or the file name suffix (if the source device is a file). No content detection is done at this stage. QImageReader will choose the first plugin that supports reading for this format.
If no plugin supports the image format, Qt's built-in handlers are checked based on either the optional format string, or the file name suffix.
If no capable plugins or built-in handlers are found, each plugin is tested by inspecting the content of the data stream.
If no plugins could detect the image format based on data contents, each built-in image handler is tested by inspecting the contents.
Finally, if all above approaches fail, QImageReader will report failure when trying to read the image.


By disabling image format autodetection, QImageReader will only query the plugins and built-in handlers based on the format string (i.e., no file name extensions are tested).

See also autoDetectImageFormat(), QImageIOHandler::canRead(), and QImageIOPlugin::capabilities().
*/
impl /*struct*/ QImageReader {
  pub fn setAutoDetectImageFormat_0<RetType, T: QImageReader_setAutoDetectImageFormat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setAutoDetectImageFormat_0(self);
    // return 1;
  }
}
pub trait QImageReader_setAutoDetectImageFormat_0<RetType> {
  fn setAutoDetectImageFormat_0(self , rsthis: & QImageReader) -> RetType;
}
impl<'a> /*trait*/ QImageReader_setAutoDetectImageFormat_0<(/*void*/)> for (bool) {
  fn setAutoDetectImageFormat_0(self , rsthis: & QImageReader) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN12QImageReader24setAutoDetectImageFormatEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qimagereader.h:80
// index:0
// Public Visibility=Default Availability=Available
// [1] bool autoDetectImageFormat() const

/*
Returns true if image format autodetection is enabled on this image reader; otherwise returns false. By default, autodetection is enabled.

See also setAutoDetectImageFormat().
*/
impl /*struct*/ QImageReader {
  pub fn autoDetectImageFormat_0<RetType, T: QImageReader_autoDetectImageFormat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.autoDetectImageFormat_0(self);
    // return 1;
  }
}
pub trait QImageReader_autoDetectImageFormat_0<RetType> {
  fn autoDetectImageFormat_0(self , rsthis: & QImageReader) -> RetType;
}
impl<'a> /*trait*/ QImageReader_autoDetectImageFormat_0<bool> for () {
  fn autoDetectImageFormat_0(self , rsthis: & QImageReader) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QImageReader21autoDetectImageFormatEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimagereader.h:82
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDecideFormatFromContent(bool)

/*
If ignored is set to true, then the image reader will ignore specified formats or file extensions and decide which plugin to use only based on the contents in the datastream.

Setting this flag means that all image plugins gets loaded. Each plugin will read the first bytes in the image data and decide if the plugin is compatible or not.

This also disables auto detecting the image format.

See also decideFormatFromContent().
*/
impl /*struct*/ QImageReader {
  pub fn setDecideFormatFromContent_0<RetType, T: QImageReader_setDecideFormatFromContent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDecideFormatFromContent_0(self);
    // return 1;
  }
}
pub trait QImageReader_setDecideFormatFromContent_0<RetType> {
  fn setDecideFormatFromContent_0(self , rsthis: & QImageReader) -> RetType;
}
impl<'a> /*trait*/ QImageReader_setDecideFormatFromContent_0<(/*void*/)> for (bool) {
  fn setDecideFormatFromContent_0(self , rsthis: & QImageReader) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN12QImageReader26setDecideFormatFromContentEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qimagereader.h:83
// index:0
// Public Visibility=Default Availability=Available
// [1] bool decideFormatFromContent() const

/*
Returns whether the image reader should decide which plugin to use only based on the contents of the datastream rather than on the file extension.

See also setDecideFormatFromContent().
*/
impl /*struct*/ QImageReader {
  pub fn decideFormatFromContent_0<RetType, T: QImageReader_decideFormatFromContent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.decideFormatFromContent_0(self);
    // return 1;
  }
}
pub trait QImageReader_decideFormatFromContent_0<RetType> {
  fn decideFormatFromContent_0(self , rsthis: & QImageReader) -> RetType;
}
impl<'a> /*trait*/ QImageReader_decideFormatFromContent_0<bool> for () {
  fn decideFormatFromContent_0(self , rsthis: & QImageReader) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QImageReader23decideFormatFromContentEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimagereader.h:85
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDevice(QIODevice *)

/*
Sets QImageReader's device to device. If a device has already been set, the old device is removed from QImageReader and is otherwise left unchanged.

If the device is not already open, QImageReader will attempt to open the device in QIODevice::ReadOnly mode by calling open(). Note that this does not work for certain devices, such as QProcess, QTcpSocket and QUdpSocket, where more logic is required to open the device.

See also device() and setFileName().
*/
impl /*struct*/ QImageReader {
  pub fn setDevice_0<RetType, T: QImageReader_setDevice_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDevice_0(self);
    // return 1;
  }
}
pub trait QImageReader_setDevice_0<RetType> {
  fn setDevice_0(self , rsthis: & QImageReader) -> RetType;
}
impl<'a> /*trait*/ QImageReader_setDevice_0<(/*void*/)> for (usize) {
  fn setDevice_0(self , rsthis: & QImageReader) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QImageReader9setDeviceEP9QIODevice", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qimagereader.h:86
// index:0
// Public Visibility=Default Availability=Available
// [8] QIODevice * device() const

/*
Returns the device currently assigned to QImageReader, or 0 if no device has been assigned.

See also setDevice().
*/
impl /*struct*/ QImageReader {
  pub fn device_0<RetType, T: QImageReader_device_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.device_0(self);
    // return 1;
  }
}
pub trait QImageReader_device_0<RetType> {
  fn device_0(self , rsthis: & QImageReader) -> RetType;
}
impl<'a> /*trait*/ QImageReader_device_0<usize> for () {
  fn device_0(self , rsthis: & QImageReader) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QImageReader6deviceEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimagereader.h:88
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFileName(const QString &)

/*
Sets the file name of QImageReader to fileName. Internally, QImageReader will create a QFile object and open it in QIODevice::ReadOnly mode, and use this when reading images.

If fileName does not include a file extension (e.g., .png or .bmp), QImageReader will cycle through all supported extensions until it finds a matching file.

See also fileName(), setDevice(), and supportedImageFormats().
*/
impl /*struct*/ QImageReader {
  pub fn setFileName_0<RetType, T: QImageReader_setFileName_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFileName_0(self);
    // return 1;
  }
}
pub trait QImageReader_setFileName_0<RetType> {
  fn setFileName_0(self , rsthis: & QImageReader) -> RetType;
}
impl<'a> /*trait*/ QImageReader_setFileName_0<(/*void*/)> for (usize) {
  fn setFileName_0(self , rsthis: & QImageReader) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QImageReader11setFileNameERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qimagereader.h:89
// index:0
// Public Visibility=Default Availability=Available
// [8] QString fileName() const

/*
If the currently assigned device is a QFile, or if setFileName() has been called, this function returns the name of the file QImageReader reads from. Otherwise (i.e., if no device has been assigned or the device is not a QFile), an empty QString is returned.

See also setFileName() and setDevice().
*/
impl /*struct*/ QImageReader {
  pub fn fileName_0<RetType, T: QImageReader_fileName_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fileName_0(self);
    // return 1;
  }
}
pub trait QImageReader_fileName_0<RetType> {
  fn fileName_0(self , rsthis: & QImageReader) -> RetType;
}
impl<'a> /*trait*/ QImageReader_fileName_0<usize> for () {
  fn fileName_0(self , rsthis: & QImageReader) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QImageReader8fileNameEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimagereader.h:91
// index:0
// Public Visibility=Default Availability=Available
// [8] QSize size() const

/*
Returns the size of the image, without actually reading the image contents.

If the image format does not support this feature, this function returns an invalid size. Qt's built-in image handlers all support this feature, but custom image format plugins are not required to do so.

See also QImageIOHandler::ImageOption, QImageIOHandler::option(), and QImageIOHandler::supportsOption().
*/
impl /*struct*/ QImageReader {
  pub fn size_0<RetType, T: QImageReader_size_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.size_0(self);
    // return 1;
  }
}
pub trait QImageReader_size_0<RetType> {
  fn size_0(self , rsthis: & QImageReader) -> RetType;
}
impl<'a> /*trait*/ QImageReader_size_0<usize> for () {
  fn size_0(self , rsthis: & QImageReader) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QImageReader4sizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimagereader.h:93
// index:0
// Public Visibility=Default Availability=Available
// [4] QImage::Format imageFormat() const

/*
Returns the format of the image, without actually reading the image contents. The format describes the image format QImageReader::read() returns, not the format of the actual image.

If the image format does not support this feature, this function returns an invalid format.

This function was introduced in  Qt 4.5.

See also QImageIOHandler::ImageOption, QImageIOHandler::option(), and QImageIOHandler::supportsOption().
*/
impl /*struct*/ QImageReader {
  pub fn imageFormat_0<RetType, T: QImageReader_imageFormat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.imageFormat_0(self);
    // return 1;
  }
}
pub trait QImageReader_imageFormat_0<RetType> {
  fn imageFormat_0(self , rsthis: & QImageReader) -> RetType;
}
impl<'a> /*trait*/ QImageReader_imageFormat_0<i32> for () {
  fn imageFormat_0(self , rsthis: & QImageReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QImageReader11imageFormatEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimagereader.h:143
// index:1
// Public static Visibility=Default Availability=Available
// [8] QByteArray imageFormat(const QString &)

/*
Returns the format of the image, without actually reading the image contents. The format describes the image format QImageReader::read() returns, not the format of the actual image.

If the image format does not support this feature, this function returns an invalid format.

This function was introduced in  Qt 4.5.

See also QImageIOHandler::ImageOption, QImageIOHandler::option(), and QImageIOHandler::supportsOption().
*/
impl /*struct*/ QImageReader {
  pub fn imageFormat_1<RetType, T: QImageReader_imageFormat_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.imageFormat_1();
    // return 1;
  }
}
pub trait QImageReader_imageFormat_1<RetType> {
  fn imageFormat_1(self ) -> RetType;
}
impl<'a> /*trait*/ QImageReader_imageFormat_1<usize> for (usize) {
  fn imageFormat_1(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN12QImageReader11imageFormatERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimagereader.h:144
// index:2
// Public static Visibility=Default Availability=Available
// [8] QByteArray imageFormat(QIODevice *)

/*
Returns the format of the image, without actually reading the image contents. The format describes the image format QImageReader::read() returns, not the format of the actual image.

If the image format does not support this feature, this function returns an invalid format.

This function was introduced in  Qt 4.5.

See also QImageIOHandler::ImageOption, QImageIOHandler::option(), and QImageIOHandler::supportsOption().
*/
impl /*struct*/ QImageReader {
  pub fn imageFormat_2<RetType, T: QImageReader_imageFormat_2<RetType>>( overload_args: T) -> RetType {
    return overload_args.imageFormat_2();
    // return 1;
  }
}
pub trait QImageReader_imageFormat_2<RetType> {
  fn imageFormat_2(self ) -> RetType;
}
impl<'a> /*trait*/ QImageReader_imageFormat_2<usize> for (usize) {
  fn imageFormat_2(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN12QImageReader11imageFormatEP9QIODevice", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimagereader.h:95
// index:0
// Public Visibility=Default Availability=Available
// [8] QStringList textKeys() const

/*
Returns the text keys for this image. You can use these keys with text() to list the image text for a certain key.

Support for this option is implemented through QImageIOHandler::Description.

This function was introduced in  Qt 4.1.

See also text(), QImageWriter::setText(), and QImage::textKeys().
*/
impl /*struct*/ QImageReader {
  pub fn textKeys_0<RetType, T: QImageReader_textKeys_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.textKeys_0(self);
    // return 1;
  }
}
pub trait QImageReader_textKeys_0<RetType> {
  fn textKeys_0(self , rsthis: & QImageReader) -> RetType;
}
impl<'a> /*trait*/ QImageReader_textKeys_0<usize> for () {
  fn textKeys_0(self , rsthis: & QImageReader) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QImageReader8textKeysEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimagereader.h:96
// index:0
// Public Visibility=Default Availability=Available
// [8] QString text(const QString &) const

/*
Returns the image text associated with key.

Support for this option is implemented through QImageIOHandler::Description.

This function was introduced in  Qt 4.1.

See also textKeys() and QImageWriter::setText().
*/
impl /*struct*/ QImageReader {
  pub fn text_0<RetType, T: QImageReader_text_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.text_0(self);
    // return 1;
  }
}
pub trait QImageReader_text_0<RetType> {
  fn text_0(self , rsthis: & QImageReader) -> RetType;
}
impl<'a> /*trait*/ QImageReader_text_0<usize> for (usize) {
  fn text_0(self , rsthis: & QImageReader) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QImageReader4textERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimagereader.h:98
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setClipRect(const QRect &)

/*
Sets the image clip rect (also known as the ROI, or Region Of Interest) to rect. The coordinates of rect are relative to the untransformed image size, as returned by size().

See also clipRect(), setScaledSize(), and setScaledClipRect().
*/
impl /*struct*/ QImageReader {
  pub fn setClipRect_0<RetType, T: QImageReader_setClipRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setClipRect_0(self);
    // return 1;
  }
}
pub trait QImageReader_setClipRect_0<RetType> {
  fn setClipRect_0(self , rsthis: & QImageReader) -> RetType;
}
impl<'a> /*trait*/ QImageReader_setClipRect_0<(/*void*/)> for (usize) {
  fn setClipRect_0(self , rsthis: & QImageReader) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QImageReader11setClipRectERK5QRect", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qimagereader.h:99
// index:0
// Public Visibility=Default Availability=Available
// [16] QRect clipRect() const

/*
Returns the clip rect (also known as the ROI, or Region Of Interest) of the image. If no clip rect has been set, an invalid QRect is returned.

See also setClipRect().
*/
impl /*struct*/ QImageReader {
  pub fn clipRect_0<RetType, T: QImageReader_clipRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.clipRect_0(self);
    // return 1;
  }
}
pub trait QImageReader_clipRect_0<RetType> {
  fn clipRect_0(self , rsthis: & QImageReader) -> RetType;
}
impl<'a> /*trait*/ QImageReader_clipRect_0<usize> for () {
  fn clipRect_0(self , rsthis: & QImageReader) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QImageReader8clipRectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimagereader.h:101
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setScaledSize(const QSize &)

/*
Sets the scaled size of the image to size. The scaling is performed after the initial clip rect, but before the scaled clip rect is applied. The algorithm used for scaling depends on the image format. By default (i.e., if the image format does not support scaling), QImageReader will use QImage::scale() with Qt::SmoothScaling.

See also scaledSize(), setClipRect(), and setScaledClipRect().
*/
impl /*struct*/ QImageReader {
  pub fn setScaledSize_0<RetType, T: QImageReader_setScaledSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setScaledSize_0(self);
    // return 1;
  }
}
pub trait QImageReader_setScaledSize_0<RetType> {
  fn setScaledSize_0(self , rsthis: & QImageReader) -> RetType;
}
impl<'a> /*trait*/ QImageReader_setScaledSize_0<(/*void*/)> for (usize) {
  fn setScaledSize_0(self , rsthis: & QImageReader) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QImageReader13setScaledSizeERK5QSize", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qimagereader.h:102
// index:0
// Public Visibility=Default Availability=Available
// [8] QSize scaledSize() const

/*
Returns the scaled size of the image.

See also setScaledSize().
*/
impl /*struct*/ QImageReader {
  pub fn scaledSize_0<RetType, T: QImageReader_scaledSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.scaledSize_0(self);
    // return 1;
  }
}
pub trait QImageReader_scaledSize_0<RetType> {
  fn scaledSize_0(self , rsthis: & QImageReader) -> RetType;
}
impl<'a> /*trait*/ QImageReader_scaledSize_0<usize> for () {
  fn scaledSize_0(self , rsthis: & QImageReader) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QImageReader10scaledSizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimagereader.h:104
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setQuality(int)

/*
Sets the quality setting of the image format to quality.

Some image formats, in particular lossy ones, entail a tradeoff between a) visual quality of the resulting image, and b) decoding execution time. This function sets the level of that tradeoff for image formats that support it.

In case of scaled image reading, the quality setting may also influence the tradeoff level between visual quality and execution speed of the scaling algorithm.

The value range of quality depends on the image format. For example, the "jpeg" format supports a quality range from 0 (low visual quality) to 100 (high visual quality).

This function was introduced in  Qt 4.2.

See also quality() and setScaledSize().
*/
impl /*struct*/ QImageReader {
  pub fn setQuality_0<RetType, T: QImageReader_setQuality_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setQuality_0(self);
    // return 1;
  }
}
pub trait QImageReader_setQuality_0<RetType> {
  fn setQuality_0(self , rsthis: & QImageReader) -> RetType;
}
impl<'a> /*trait*/ QImageReader_setQuality_0<(/*void*/)> for (i32) {
  fn setQuality_0(self , rsthis: & QImageReader) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN12QImageReader10setQualityEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qimagereader.h:105
// index:0
// Public Visibility=Default Availability=Available
// [4] int quality() const

/*
Returns the quality setting of the image format.

This function was introduced in  Qt 4.2.

See also setQuality().
*/
impl /*struct*/ QImageReader {
  pub fn quality_0<RetType, T: QImageReader_quality_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.quality_0(self);
    // return 1;
  }
}
pub trait QImageReader_quality_0<RetType> {
  fn quality_0(self , rsthis: & QImageReader) -> RetType;
}
impl<'a> /*trait*/ QImageReader_quality_0<i32> for () {
  fn quality_0(self , rsthis: & QImageReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QImageReader7qualityEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimagereader.h:107
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setScaledClipRect(const QRect &)

/*
Sets the scaled clip rect to rect. The scaled clip rect is the clip rect (also known as ROI, or Region Of Interest) that is applied after the image has been scaled.

See also scaledClipRect() and setScaledSize().
*/
impl /*struct*/ QImageReader {
  pub fn setScaledClipRect_0<RetType, T: QImageReader_setScaledClipRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setScaledClipRect_0(self);
    // return 1;
  }
}
pub trait QImageReader_setScaledClipRect_0<RetType> {
  fn setScaledClipRect_0(self , rsthis: & QImageReader) -> RetType;
}
impl<'a> /*trait*/ QImageReader_setScaledClipRect_0<(/*void*/)> for (usize) {
  fn setScaledClipRect_0(self , rsthis: & QImageReader) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QImageReader17setScaledClipRectERK5QRect", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qimagereader.h:108
// index:0
// Public Visibility=Default Availability=Available
// [16] QRect scaledClipRect() const

/*
Returns the scaled clip rect of the image.

See also setScaledClipRect().
*/
impl /*struct*/ QImageReader {
  pub fn scaledClipRect_0<RetType, T: QImageReader_scaledClipRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.scaledClipRect_0(self);
    // return 1;
  }
}
pub trait QImageReader_scaledClipRect_0<RetType> {
  fn scaledClipRect_0(self , rsthis: & QImageReader) -> RetType;
}
impl<'a> /*trait*/ QImageReader_scaledClipRect_0<usize> for () {
  fn scaledClipRect_0(self , rsthis: & QImageReader) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QImageReader14scaledClipRectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimagereader.h:110
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setBackgroundColor(const QColor &)

/*
Sets the background color to color. Image formats that support this operation are expected to initialize the background to color before reading an image.

This function was introduced in  Qt 4.1.

See also backgroundColor() and read().
*/
impl /*struct*/ QImageReader {
  pub fn setBackgroundColor_0<RetType, T: QImageReader_setBackgroundColor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setBackgroundColor_0(self);
    // return 1;
  }
}
pub trait QImageReader_setBackgroundColor_0<RetType> {
  fn setBackgroundColor_0(self , rsthis: & QImageReader) -> RetType;
}
impl<'a> /*trait*/ QImageReader_setBackgroundColor_0<(/*void*/)> for (usize) {
  fn setBackgroundColor_0(self , rsthis: & QImageReader) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QImageReader18setBackgroundColorERK6QColor", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qimagereader.h:111
// index:0
// Public Visibility=Default Availability=Available
// [16] QColor backgroundColor() const

/*
Returns the background color that's used when reading an image. If the image format does not support setting the background color an invalid color is returned.

This function was introduced in  Qt 4.1.

See also setBackgroundColor() and read().
*/
impl /*struct*/ QImageReader {
  pub fn backgroundColor_0<RetType, T: QImageReader_backgroundColor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.backgroundColor_0(self);
    // return 1;
  }
}
pub trait QImageReader_backgroundColor_0<RetType> {
  fn backgroundColor_0(self , rsthis: & QImageReader) -> RetType;
}
impl<'a> /*trait*/ QImageReader_backgroundColor_0<usize> for () {
  fn backgroundColor_0(self , rsthis: & QImageReader) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QImageReader15backgroundColorEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimagereader.h:113
// index:0
// Public Visibility=Default Availability=Available
// [1] bool supportsAnimation() const

/*
Returns true if the image format supports animation; otherwise, false is returned.

This function was introduced in  Qt 4.1.

See also QMovie::supportedFormats().
*/
impl /*struct*/ QImageReader {
  pub fn supportsAnimation_0<RetType, T: QImageReader_supportsAnimation_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.supportsAnimation_0(self);
    // return 1;
  }
}
pub trait QImageReader_supportsAnimation_0<RetType> {
  fn supportsAnimation_0(self , rsthis: & QImageReader) -> RetType;
}
impl<'a> /*trait*/ QImageReader_supportsAnimation_0<bool> for () {
  fn supportsAnimation_0(self , rsthis: & QImageReader) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QImageReader17supportsAnimationEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimagereader.h:115
// index:0
// Public Visibility=Default Availability=Available
// [-2] QImageIOHandler::Transformations transformation() const

/*
Returns the transformation metadata of the image, including image orientation. If the format does not support transformation metadata QImageIOHandler::Transformation_None is returned.

This function was introduced in  Qt 5.5.

See also setAutoTransform() and autoTransform().
*/
impl /*struct*/ QImageReader {
  pub fn transformation_0<RetType, T: QImageReader_transformation_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.transformation_0(self);
    // return 1;
  }
}
pub trait QImageReader_transformation_0<RetType> {
  fn transformation_0(self , rsthis: & QImageReader) -> RetType;
}
impl<'a> /*trait*/ QImageReader_transformation_0<i32> for () {
  fn transformation_0(self , rsthis: & QImageReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QImageReader14transformationEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimagereader.h:117
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setAutoTransform(bool)

/*
Determines that images returned by read() should have transformation metadata automatically applied if enabled is true.

This function was introduced in  Qt 5.5.

See also autoTransform(), transformation(), and read().
*/
impl /*struct*/ QImageReader {
  pub fn setAutoTransform_0<RetType, T: QImageReader_setAutoTransform_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setAutoTransform_0(self);
    // return 1;
  }
}
pub trait QImageReader_setAutoTransform_0<RetType> {
  fn setAutoTransform_0(self , rsthis: & QImageReader) -> RetType;
}
impl<'a> /*trait*/ QImageReader_setAutoTransform_0<(/*void*/)> for (bool) {
  fn setAutoTransform_0(self , rsthis: & QImageReader) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN12QImageReader16setAutoTransformEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qimagereader.h:118
// index:0
// Public Visibility=Default Availability=Available
// [1] bool autoTransform() const

/*
Returns true if the image handler will apply transformation metadata on read().

This function was introduced in  Qt 5.5.

See also setAutoTransform(), transformation(), and read().
*/
impl /*struct*/ QImageReader {
  pub fn autoTransform_0<RetType, T: QImageReader_autoTransform_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.autoTransform_0(self);
    // return 1;
  }
}
pub trait QImageReader_autoTransform_0<RetType> {
  fn autoTransform_0(self , rsthis: & QImageReader) -> RetType;
}
impl<'a> /*trait*/ QImageReader_autoTransform_0<bool> for () {
  fn autoTransform_0(self , rsthis: & QImageReader) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QImageReader13autoTransformEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimagereader.h:120
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setGamma(float)

/*
This is an image format specific function that forces images with gamma information to be gamma corrected to gamma. For image formats that do not support gamma correction, this value is ignored.

To gamma correct to a standard PC color-space, set gamma to 1/2.2.

This function was introduced in  Qt 5.6.

See also gamma().
*/
impl /*struct*/ QImageReader {
  pub fn setGamma_0<RetType, T: QImageReader_setGamma_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setGamma_0(self);
    // return 1;
  }
}
pub trait QImageReader_setGamma_0<RetType> {
  fn setGamma_0(self , rsthis: & QImageReader) -> RetType;
}
impl<'a> /*trait*/ QImageReader_setGamma_0<(/*void*/)> for (f32) {
  fn setGamma_0(self , rsthis: & QImageReader) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f32 as usize;
     qtrt::InvokeQtFunc6("_ZN12QImageReader8setGammaEf", 1,qtrt::FFITY_FLOAT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qimagereader.h:121
// index:0
// Public Visibility=Default Availability=Available
// [4] float gamma() const

/*
Returns the gamma level of the decoded image. If setGamma() has been called and gamma correction is supported it will return the gamma set. If gamma level is not supported by the image format, 0.0 is returned.

This function was introduced in  Qt 5.6.

See also setGamma().
*/
impl /*struct*/ QImageReader {
  pub fn gamma_0<RetType, T: QImageReader_gamma_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.gamma_0(self);
    // return 1;
  }
}
pub trait QImageReader_gamma_0<RetType> {
  fn gamma_0(self , rsthis: & QImageReader) -> RetType;
}
impl<'a> /*trait*/ QImageReader_gamma_0<f32> for () {
  fn gamma_0(self , rsthis: & QImageReader) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QImageReader5gammaEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimagereader.h:123
// index:0
// Public Visibility=Default Availability=Available
// [8] QByteArray subType() const

/*
Returns the subtype of the image.

This function was introduced in  Qt 5.4.
*/
impl /*struct*/ QImageReader {
  pub fn subType_0<RetType, T: QImageReader_subType_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.subType_0(self);
    // return 1;
  }
}
pub trait QImageReader_subType_0<RetType> {
  fn subType_0(self , rsthis: & QImageReader) -> RetType;
}
impl<'a> /*trait*/ QImageReader_subType_0<usize> for () {
  fn subType_0(self , rsthis: & QImageReader) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QImageReader7subTypeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimagereader.h:126
// index:0
// Public Visibility=Default Availability=Available
// [1] bool canRead() const

/*
Returns true if an image can be read for the device (i.e., the image format is supported, and the device seems to contain valid data); otherwise returns false.

canRead() is a lightweight function that only does a quick test to see if the image data is valid. read() may still return false after canRead() returns true, if the image data is corrupt.

Note: A QMimeDatabase lookup is normally a better approach than this function for identifying potentially non-image files or data.

For images that support animation, canRead() returns false when all frames have been read.

See also read(), supportedImageFormats(), and QMimeDatabase.
*/
impl /*struct*/ QImageReader {
  pub fn canRead_0<RetType, T: QImageReader_canRead_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.canRead_0(self);
    // return 1;
  }
}
pub trait QImageReader_canRead_0<RetType> {
  fn canRead_0(self , rsthis: & QImageReader) -> RetType;
}
impl<'a> /*trait*/ QImageReader_canRead_0<bool> for () {
  fn canRead_0(self , rsthis: & QImageReader) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QImageReader7canReadEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimagereader.h:127
// index:0
// Public Visibility=Default Availability=Available
// [32] QImage read()

/*
Reads an image from the device. On success, the image that was read is returned; otherwise, a null QImage is returned. You can then call error() to find the type of error that occurred, or errorString() to get a human readable description of the error.

For image formats that support animation, calling read() repeatedly will return the next frame. When all frames have been read, a null image will be returned.

See also canRead(), supportedImageFormats(), supportsAnimation(), and QMovie.
*/
impl /*struct*/ QImageReader {
  pub fn read_0<RetType, T: QImageReader_read_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.read_0(self);
    // return 1;
  }
}
pub trait QImageReader_read_0<RetType> {
  fn read_0(self , rsthis: & QImageReader) -> RetType;
}
impl<'a> /*trait*/ QImageReader_read_0<usize> for () {
  fn read_0(self , rsthis: & QImageReader) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN12QImageReader4readEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimagereader.h:128
// index:1
// Public Visibility=Default Availability=Available
// [1] bool read(QImage *)

/*
Reads an image from the device. On success, the image that was read is returned; otherwise, a null QImage is returned. You can then call error() to find the type of error that occurred, or errorString() to get a human readable description of the error.

For image formats that support animation, calling read() repeatedly will return the next frame. When all frames have been read, a null image will be returned.

See also canRead(), supportedImageFormats(), supportsAnimation(), and QMovie.
*/
impl /*struct*/ QImageReader {
  pub fn read_1<RetType, T: QImageReader_read_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.read_1(self);
    // return 1;
  }
}
pub trait QImageReader_read_1<RetType> {
  fn read_1(self , rsthis: & QImageReader) -> RetType;
}
impl<'a> /*trait*/ QImageReader_read_1<bool> for (usize) {
  fn read_1(self , rsthis: & QImageReader) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN12QImageReader4readEP6QImage", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimagereader.h:130
// index:0
// Public Visibility=Default Availability=Available
// [1] bool jumpToNextImage()

/*
For image formats that support animation, this function steps over the current image, returning true if successful or false if there is no following image in the animation.

The default implementation calls read(), then discards the resulting image, but the image handler may have a more efficient way of implementing this operation.

See also jumpToImage() and QImageIOHandler::jumpToNextImage().
*/
impl /*struct*/ QImageReader {
  pub fn jumpToNextImage_0<RetType, T: QImageReader_jumpToNextImage_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.jumpToNextImage_0(self);
    // return 1;
  }
}
pub trait QImageReader_jumpToNextImage_0<RetType> {
  fn jumpToNextImage_0(self , rsthis: & QImageReader) -> RetType;
}
impl<'a> /*trait*/ QImageReader_jumpToNextImage_0<bool> for () {
  fn jumpToNextImage_0(self , rsthis: & QImageReader) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN12QImageReader15jumpToNextImageEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimagereader.h:131
// index:0
// Public Visibility=Default Availability=Available
// [1] bool jumpToImage(int)

/*
For image formats that support animation, this function skips to the image whose sequence number is imageNumber, returning true if successful or false if the corresponding image cannot be found.

The next call to read() will attempt to read this image.

See also jumpToNextImage() and QImageIOHandler::jumpToImage().
*/
impl /*struct*/ QImageReader {
  pub fn jumpToImage_0<RetType, T: QImageReader_jumpToImage_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.jumpToImage_0(self);
    // return 1;
  }
}
pub trait QImageReader_jumpToImage_0<RetType> {
  fn jumpToImage_0(self , rsthis: & QImageReader) -> RetType;
}
impl<'a> /*trait*/ QImageReader_jumpToImage_0<bool> for (i32) {
  fn jumpToImage_0(self , rsthis: & QImageReader) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN12QImageReader11jumpToImageEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimagereader.h:132
// index:0
// Public Visibility=Default Availability=Available
// [4] int loopCount() const

/*
For image formats that support animation, this function returns the number of times the animation should loop. If this function returns -1, it can either mean the animation should loop forever, or that an error occurred. If an error occurred, canRead() will return false.

See also supportsAnimation(), QImageIOHandler::loopCount(), and canRead().
*/
impl /*struct*/ QImageReader {
  pub fn loopCount_0<RetType, T: QImageReader_loopCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.loopCount_0(self);
    // return 1;
  }
}
pub trait QImageReader_loopCount_0<RetType> {
  fn loopCount_0(self , rsthis: & QImageReader) -> RetType;
}
impl<'a> /*trait*/ QImageReader_loopCount_0<i32> for () {
  fn loopCount_0(self , rsthis: & QImageReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QImageReader9loopCountEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimagereader.h:133
// index:0
// Public Visibility=Default Availability=Available
// [4] int imageCount() const

/*
For image formats that support animation, this function returns the total number of images in the animation. If the format does not support animation, 0 is returned.

This function returns -1 if an error occurred.

See also supportsAnimation(), QImageIOHandler::imageCount(), and canRead().
*/
impl /*struct*/ QImageReader {
  pub fn imageCount_0<RetType, T: QImageReader_imageCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.imageCount_0(self);
    // return 1;
  }
}
pub trait QImageReader_imageCount_0<RetType> {
  fn imageCount_0(self , rsthis: & QImageReader) -> RetType;
}
impl<'a> /*trait*/ QImageReader_imageCount_0<i32> for () {
  fn imageCount_0(self , rsthis: & QImageReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QImageReader10imageCountEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimagereader.h:134
// index:0
// Public Visibility=Default Availability=Available
// [4] int nextImageDelay() const

/*
For image formats that support animation, this function returns the number of milliseconds to wait until displaying the next frame in the animation. If the image format doesn't support animation, 0 is returned.

This function returns -1 if an error occurred.

See also supportsAnimation(), QImageIOHandler::nextImageDelay(), and canRead().
*/
impl /*struct*/ QImageReader {
  pub fn nextImageDelay_0<RetType, T: QImageReader_nextImageDelay_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.nextImageDelay_0(self);
    // return 1;
  }
}
pub trait QImageReader_nextImageDelay_0<RetType> {
  fn nextImageDelay_0(self , rsthis: & QImageReader) -> RetType;
}
impl<'a> /*trait*/ QImageReader_nextImageDelay_0<i32> for () {
  fn nextImageDelay_0(self , rsthis: & QImageReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QImageReader14nextImageDelayEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimagereader.h:135
// index:0
// Public Visibility=Default Availability=Available
// [4] int currentImageNumber() const

/*
For image formats that support animation, this function returns the sequence number of the current frame. If the image format doesn't support animation, 0 is returned.

This function returns -1 if an error occurred.

See also supportsAnimation(), QImageIOHandler::currentImageNumber(), and canRead().
*/
impl /*struct*/ QImageReader {
  pub fn currentImageNumber_0<RetType, T: QImageReader_currentImageNumber_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.currentImageNumber_0(self);
    // return 1;
  }
}
pub trait QImageReader_currentImageNumber_0<RetType> {
  fn currentImageNumber_0(self , rsthis: & QImageReader) -> RetType;
}
impl<'a> /*trait*/ QImageReader_currentImageNumber_0<i32> for () {
  fn currentImageNumber_0(self , rsthis: & QImageReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QImageReader18currentImageNumberEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimagereader.h:136
// index:0
// Public Visibility=Default Availability=Available
// [16] QRect currentImageRect() const

/*
For image formats that support animation, this function returns the rect for the current frame. Otherwise, a null rect is returned.

See also supportsAnimation() and QImageIOHandler::currentImageRect().
*/
impl /*struct*/ QImageReader {
  pub fn currentImageRect_0<RetType, T: QImageReader_currentImageRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.currentImageRect_0(self);
    // return 1;
  }
}
pub trait QImageReader_currentImageRect_0<RetType> {
  fn currentImageRect_0(self , rsthis: & QImageReader) -> RetType;
}
impl<'a> /*trait*/ QImageReader_currentImageRect_0<usize> for () {
  fn currentImageRect_0(self , rsthis: & QImageReader) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QImageReader16currentImageRectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimagereader.h:138
// index:0
// Public Visibility=Default Availability=Available
// [4] QImageReader::ImageReaderError error() const

/*
Returns the type of error that occurred last.

See also ImageReaderError and errorString().
*/
impl /*struct*/ QImageReader {
  pub fn error_0<RetType, T: QImageReader_error_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.error_0(self);
    // return 1;
  }
}
pub trait QImageReader_error_0<RetType> {
  fn error_0(self , rsthis: & QImageReader) -> RetType;
}
impl<'a> /*trait*/ QImageReader_error_0<i32> for () {
  fn error_0(self , rsthis: & QImageReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QImageReader5errorEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimagereader.h:139
// index:0
// Public Visibility=Default Availability=Available
// [8] QString errorString() const

/*
Returns a human readable description of the last error that occurred.

See also error().
*/
impl /*struct*/ QImageReader {
  pub fn errorString_0<RetType, T: QImageReader_errorString_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.errorString_0(self);
    // return 1;
  }
}
pub trait QImageReader_errorString_0<RetType> {
  fn errorString_0(self , rsthis: & QImageReader) -> RetType;
}
impl<'a> /*trait*/ QImageReader_errorString_0<usize> for () {
  fn errorString_0(self , rsthis: & QImageReader) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QImageReader11errorStringEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimagereader.h:141
// index:0
// Public Visibility=Default Availability=Available
// [1] bool supportsOption(QImageIOHandler::ImageOption) const

/*
Returns true if the reader supports option; otherwise returns false.

Different image formats support different options. Call this function to determine whether a certain option is supported by the current format. For example, the PNG format allows you to embed text into the image's metadata (see text()), and the BMP format allows you to determine the image's size without loading the whole image into memory (see size()).


  QImageReader reader(":/image.png");
  if (reader.supportsOption(QImageIOHandler::Size))
      qDebug() << "Size:" << reader.size();



This function was introduced in  Qt 4.2.

See also QImageWriter::supportsOption().
*/
impl /*struct*/ QImageReader {
  pub fn supportsOption_0<RetType, T: QImageReader_supportsOption_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.supportsOption_0(self);
    // return 1;
  }
}
pub trait QImageReader_supportsOption_0<RetType> {
  fn supportsOption_0(self , rsthis: & QImageReader) -> RetType;
}
impl<'a> /*trait*/ QImageReader_supportsOption_0<bool> for (i32) {
  fn supportsOption_0(self , rsthis: & QImageReader) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QImageReader14supportsOptionEN15QImageIOHandler11ImageOptionE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}


/*
This enum describes the different types of errors that can occur when reading images with QImageReader.


*/
pub type QImageReader__ImageReaderError = i32;
// An unknown error occurred. If you get this value after calling read(), it is most likely caused by a bug in QImageReader.
pub const QImageReader__UnknownError :QImageReader__ImageReaderError = 0;
// QImageReader was used with a file name, but not file was found with that name. This can also happen if the file name contained no extension, and the file with the correct extension is not supported by Qt.
pub const QImageReader__FileNotFoundError :QImageReader__ImageReaderError = 1;
// QImageReader encountered a device error when reading the image. You can consult your particular device for more details on what went wrong.
pub const QImageReader__DeviceError :QImageReader__ImageReaderError = 2;
// Qt does not support the requested image format.
pub const QImageReader__UnsupportedFormatError :QImageReader__ImageReaderError = 3;
// The image data was invalid, and QImageReader was unable to read an image from it. The can happen if the image file is damaged.
pub const QImageReader__InvalidDataError :QImageReader__ImageReaderError = 4;
pub fn QImageReader_ImageReaderErrorItemName(val: i32) ->String {
  match val {
     QImageReader__UnknownError => // 0
     {return String::from("UnknownError");}
     QImageReader__FileNotFoundError => // 1
     {return String::from("FileNotFoundError");}
     QImageReader__DeviceError => // 2
     {return String::from("DeviceError");}
     QImageReader__UnsupportedFormatError => // 3
     {return String::from("UnsupportedFormatError");}
     QImageReader__InvalidDataError => // 4
     {return String::from("InvalidDataError");}
  _ => {return format!("{}", val);}
}
}
pub fn QImageReader_ImageReaderErrorItemName_s(val: i32) ->String {
  //var nilthis *QImageReader
  //return nilthis.ImageReaderErrorItemName(val);
  return QImageReader_ImageReaderErrorItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
