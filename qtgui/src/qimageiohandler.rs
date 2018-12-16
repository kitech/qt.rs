

// mod ::gui::QImageIOHandler
// package qtgui
// /usr/include/qt/QtGui/qimageiohandler.h
// #include <qimageiohandler.h>
// #include <QtGui>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 4
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
#[derive(Default)] // class sizeof(QImageIOHandler)=16
pub struct QImageIOHandler {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QImageIOHandler_ITF interface {
//    QImageIOHandler_PTR() *QImageIOHandler
//}
//func (ptr *QImageIOHandler) QImageIOHandler_PTR() *QImageIOHandler { return ptr }

impl /*struct*/ QImageIOHandler {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QImageIOHandler {
    return QImageIOHandler{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QImageIOHandler {
//  type Target = QImageIOHandlerBASE;
//
//  fn deref(&self) -> &QImageIOHandlerBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QImageIOHandlerBASE> for QImageIOHandler {
//  fn as_ref(& self) -> & QImageIOHandlerBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qimageiohandler.h:62
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QImageIOHandler()

/*
Constructs a QImageIOHandler object.
*/
// QImageIOHandler() ctx.fn_proto_cpp
impl /*struct*/ QImageIOHandler {
  pub fn QImageIOHandler_0<T: QImageIOHandler_QImageIOHandler_0>(value: T) -> QImageIOHandler {
    let rsthis = value.QImageIOHandler_0();
    return rsthis;
    // return 1;
  }
}

pub trait QImageIOHandler_QImageIOHandler_0 {
  fn QImageIOHandler_0(self) -> QImageIOHandler;
}
// QImageIOHandler() ctx.fn_proto_cpp
impl<'a> /*trait*/ QImageIOHandler_QImageIOHandler_0 for () {
  fn QImageIOHandler_0(self) -> QImageIOHandler {
    // unsafe{_ZN15QImageIOHandlerC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN15QImageIOHandlerC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QImageIOHandler{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qimageiohandler.h:63
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QImageIOHandler()

/*

*/
pub fn DeleteQImageIOHandler(this :*mut QImageIOHandler) {
    // let rv = qtrt::InvokeQtFunc6("_ZN15QImageIOHandlerD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qimageiohandler.h:65
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDevice(QIODevice *)

/*
Sets the device of the QImageIOHandler to device. The image handler will use this device when reading and writing images.

The device can only be set once and must be set before calling canRead(), read(), write(), etc. If you need to read multiple files, construct multiple instances of the appropriate QImageIOHandler subclass.

See also device().
*/
impl /*struct*/ QImageIOHandler {
  pub fn setDevice_0<RetType, T: QImageIOHandler_setDevice_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDevice_0(self);
    // return 1;
  }
}
pub trait QImageIOHandler_setDevice_0<RetType> {
  fn setDevice_0(self , rsthis: & QImageIOHandler) -> RetType;
}
impl<'a> /*trait*/ QImageIOHandler_setDevice_0<(/*void*/)> for (usize) {
  fn setDevice_0(self , rsthis: & QImageIOHandler) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QImageIOHandler9setDeviceEP9QIODevice", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qimageiohandler.h:66
// index:0
// Public Visibility=Default Availability=Available
// [8] QIODevice * device() const

/*
Returns the device currently assigned to the QImageIOHandler. If not device has been assigned, 0 is returned.

See also setDevice().
*/
impl /*struct*/ QImageIOHandler {
  pub fn device_0<RetType, T: QImageIOHandler_device_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.device_0(self);
    // return 1;
  }
}
pub trait QImageIOHandler_device_0<RetType> {
  fn device_0(self , rsthis: & QImageIOHandler) -> RetType;
}
impl<'a> /*trait*/ QImageIOHandler_device_0<usize> for () {
  fn device_0(self , rsthis: & QImageIOHandler) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QImageIOHandler6deviceEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimageiohandler.h:68
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFormat(const QByteArray &)

/*
Sets the format of the QImageIOHandler to format. The format is most useful for handlers that support multiple image formats.

See also format().
*/
impl /*struct*/ QImageIOHandler {
  pub fn setFormat_0<RetType, T: QImageIOHandler_setFormat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFormat_0(self);
    // return 1;
  }
}
pub trait QImageIOHandler_setFormat_0<RetType> {
  fn setFormat_0(self , rsthis: & QImageIOHandler) -> RetType;
}
impl<'a> /*trait*/ QImageIOHandler_setFormat_0<(/*void*/)> for (usize) {
  fn setFormat_0(self , rsthis: & QImageIOHandler) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QImageIOHandler9setFormatERK10QByteArray", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qimageiohandler.h:69
// index:1
// Public Visibility=Default Availability=Available
// [-2] void setFormat(const QByteArray &) const

/*
Sets the format of the QImageIOHandler to format. The format is most useful for handlers that support multiple image formats.

See also format().
*/
impl /*struct*/ QImageIOHandler {
  pub fn setFormat_1<RetType, T: QImageIOHandler_setFormat_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFormat_1(self);
    // return 1;
  }
}
pub trait QImageIOHandler_setFormat_1<RetType> {
  fn setFormat_1(self , rsthis: & QImageIOHandler) -> RetType;
}
impl<'a> /*trait*/ QImageIOHandler_setFormat_1<(/*void*/)> for (usize) {
  fn setFormat_1(self , rsthis: & QImageIOHandler) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK15QImageIOHandler9setFormatERK10QByteArray", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qimageiohandler.h:70
// index:0
// Public Visibility=Default Availability=Available
// [8] QByteArray format() const

/*
Returns the format that is currently assigned to QImageIOHandler. If no format has been assigned, an empty string is returned.

See also setFormat().
*/
impl /*struct*/ QImageIOHandler {
  pub fn format_0<RetType, T: QImageIOHandler_format_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.format_0(self);
    // return 1;
  }
}
pub trait QImageIOHandler_format_0<RetType> {
  fn format_0(self , rsthis: & QImageIOHandler) -> RetType;
}
impl<'a> /*trait*/ QImageIOHandler_format_0<usize> for () {
  fn format_0(self , rsthis: & QImageIOHandler) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QImageIOHandler6formatEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimageiohandler.h:72
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QByteArray name() const

/*

*/
impl /*struct*/ QImageIOHandler {
  pub fn name_0<RetType, T: QImageIOHandler_name_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.name_0(self);
    // return 1;
  }
}
pub trait QImageIOHandler_name_0<RetType> {
  fn name_0(self , rsthis: & QImageIOHandler) -> RetType;
}
impl<'a> /*trait*/ QImageIOHandler_name_0<usize> for () {
  fn name_0(self , rsthis: & QImageIOHandler) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QImageIOHandler4nameEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimageiohandler.h:74
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [1] bool canRead() const

/*
Returns true if an image can be read from the device (i.e., the image format is supported, the device can be read from and the initial header information suggests that the image can be read); otherwise returns false.

When reimplementing canRead(), make sure that the I/O device (device()) is left in its original state (e.g., by using peek() rather than read()).

See also read() and QIODevice::peek().
*/
impl /*struct*/ QImageIOHandler {
  pub fn canRead_0<RetType, T: QImageIOHandler_canRead_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.canRead_0(self);
    // return 1;
  }
}
pub trait QImageIOHandler_canRead_0<RetType> {
  fn canRead_0(self , rsthis: & QImageIOHandler) -> RetType;
}
impl<'a> /*trait*/ QImageIOHandler_canRead_0<bool> for () {
  fn canRead_0(self , rsthis: & QImageIOHandler) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QImageIOHandler7canReadEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimageiohandler.h:75
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [1] bool read(QImage *)

/*
Read an image from the device, and stores it in image. Returns true if the image is successfully read; otherwise returns false.

For image formats that support incremental loading, and for animation formats, the image handler can assume that image points to the previous frame.

See also canRead().
*/
impl /*struct*/ QImageIOHandler {
  pub fn read_0<RetType, T: QImageIOHandler_read_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.read_0(self);
    // return 1;
  }
}
pub trait QImageIOHandler_read_0<RetType> {
  fn read_0(self , rsthis: & QImageIOHandler) -> RetType;
}
impl<'a> /*trait*/ QImageIOHandler_read_0<bool> for (usize) {
  fn read_0(self , rsthis: & QImageIOHandler) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN15QImageIOHandler4readEP6QImage", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimageiohandler.h:76
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool write(const QImage &)

/*
Writes the image image to the assigned device. Returns true on success; otherwise returns false.

The default implementation does nothing, and simply returns false.
*/
impl /*struct*/ QImageIOHandler {
  pub fn write_0<RetType, T: QImageIOHandler_write_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.write_0(self);
    // return 1;
  }
}
pub trait QImageIOHandler_write_0<RetType> {
  fn write_0(self , rsthis: & QImageIOHandler) -> RetType;
}
impl<'a> /*trait*/ QImageIOHandler_write_0<bool> for (usize) {
  fn write_0(self , rsthis: & QImageIOHandler) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN15QImageIOHandler5writeERK6QImage", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimageiohandler.h:115
// index:0
// Public virtual Visibility=Default Availability=Available
// [16] QVariant option(QImageIOHandler::ImageOption) const

/*
Returns the value assigned to option as a QVariant. The type of the value depends on the option. For example, option(Size) returns a QSize variant.

See also setOption() and supportsOption().
*/
impl /*struct*/ QImageIOHandler {
  pub fn option_0<RetType, T: QImageIOHandler_option_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.option_0(self);
    // return 1;
  }
}
pub trait QImageIOHandler_option_0<RetType> {
  fn option_0(self , rsthis: & QImageIOHandler) -> RetType;
}
impl<'a> /*trait*/ QImageIOHandler_option_0<usize> for (i32) {
  fn option_0(self , rsthis: & QImageIOHandler) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QImageIOHandler6optionENS_11ImageOptionE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimageiohandler.h:116
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void setOption(QImageIOHandler::ImageOption, const QVariant &)

/*
Sets the option option with the value value.

See also option() and ImageOption.
*/
impl /*struct*/ QImageIOHandler {
  pub fn setOption_0<RetType, T: QImageIOHandler_setOption_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setOption_0(self);
    // return 1;
  }
}
pub trait QImageIOHandler_setOption_0<RetType> {
  fn setOption_0(self , rsthis: & QImageIOHandler) -> RetType;
}
impl<'a> /*trait*/ QImageIOHandler_setOption_0<(/*void*/)> for (i32,usize) {
  fn setOption_0(self , rsthis: & QImageIOHandler) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QImageIOHandler9setOptionENS_11ImageOptionERK8QVariant", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qimageiohandler.h:117
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool supportsOption(QImageIOHandler::ImageOption) const

/*
Returns true if the QImageIOHandler supports the option option; otherwise returns false. For example, if the QImageIOHandler supports the Size option, supportsOption(Size) must return true.

See also setOption() and option().
*/
impl /*struct*/ QImageIOHandler {
  pub fn supportsOption_0<RetType, T: QImageIOHandler_supportsOption_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.supportsOption_0(self);
    // return 1;
  }
}
pub trait QImageIOHandler_supportsOption_0<RetType> {
  fn supportsOption_0(self , rsthis: & QImageIOHandler) -> RetType;
}
impl<'a> /*trait*/ QImageIOHandler_supportsOption_0<bool> for (i32) {
  fn supportsOption_0(self , rsthis: & QImageIOHandler) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QImageIOHandler14supportsOptionENS_11ImageOptionE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimageiohandler.h:120
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool jumpToNextImage()

/*
For image formats that support animation, this function jumps to the next image.

The default implementation does nothing, and returns false.
*/
impl /*struct*/ QImageIOHandler {
  pub fn jumpToNextImage_0<RetType, T: QImageIOHandler_jumpToNextImage_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.jumpToNextImage_0(self);
    // return 1;
  }
}
pub trait QImageIOHandler_jumpToNextImage_0<RetType> {
  fn jumpToNextImage_0(self , rsthis: & QImageIOHandler) -> RetType;
}
impl<'a> /*trait*/ QImageIOHandler_jumpToNextImage_0<bool> for () {
  fn jumpToNextImage_0(self , rsthis: & QImageIOHandler) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN15QImageIOHandler15jumpToNextImageEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimageiohandler.h:121
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool jumpToImage(int)

/*
For image formats that support animation, this function jumps to the image whose sequence number is imageNumber. The next call to read() will attempt to read this image.

The default implementation does nothing, and returns false.
*/
impl /*struct*/ QImageIOHandler {
  pub fn jumpToImage_0<RetType, T: QImageIOHandler_jumpToImage_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.jumpToImage_0(self);
    // return 1;
  }
}
pub trait QImageIOHandler_jumpToImage_0<RetType> {
  fn jumpToImage_0(self , rsthis: & QImageIOHandler) -> RetType;
}
impl<'a> /*trait*/ QImageIOHandler_jumpToImage_0<bool> for (i32) {
  fn jumpToImage_0(self , rsthis: & QImageIOHandler) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN15QImageIOHandler11jumpToImageEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimageiohandler.h:122
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] int loopCount() const

/*
For image formats that support animation, this function returns the number of times the animation should loop. If the image format does not support animation, 0 is returned.
*/
impl /*struct*/ QImageIOHandler {
  pub fn loopCount_0<RetType, T: QImageIOHandler_loopCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.loopCount_0(self);
    // return 1;
  }
}
pub trait QImageIOHandler_loopCount_0<RetType> {
  fn loopCount_0(self , rsthis: & QImageIOHandler) -> RetType;
}
impl<'a> /*trait*/ QImageIOHandler_loopCount_0<i32> for () {
  fn loopCount_0(self , rsthis: & QImageIOHandler) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QImageIOHandler9loopCountEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimageiohandler.h:123
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] int imageCount() const

/*
For image formats that support animation, this function returns the number of images in the animation. If the image format does not support animation, or if it is unable to determine the number of images, 0 is returned.

The default implementation returns 1 if canRead() returns true; otherwise 0 is returned.
*/
impl /*struct*/ QImageIOHandler {
  pub fn imageCount_0<RetType, T: QImageIOHandler_imageCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.imageCount_0(self);
    // return 1;
  }
}
pub trait QImageIOHandler_imageCount_0<RetType> {
  fn imageCount_0(self , rsthis: & QImageIOHandler) -> RetType;
}
impl<'a> /*trait*/ QImageIOHandler_imageCount_0<i32> for () {
  fn imageCount_0(self , rsthis: & QImageIOHandler) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QImageIOHandler10imageCountEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimageiohandler.h:124
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] int nextImageDelay() const

/*
For image formats that support animation, this function returns the number of milliseconds to wait until reading the next image. If the image format does not support animation, 0 is returned.
*/
impl /*struct*/ QImageIOHandler {
  pub fn nextImageDelay_0<RetType, T: QImageIOHandler_nextImageDelay_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.nextImageDelay_0(self);
    // return 1;
  }
}
pub trait QImageIOHandler_nextImageDelay_0<RetType> {
  fn nextImageDelay_0(self , rsthis: & QImageIOHandler) -> RetType;
}
impl<'a> /*trait*/ QImageIOHandler_nextImageDelay_0<i32> for () {
  fn nextImageDelay_0(self , rsthis: & QImageIOHandler) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QImageIOHandler14nextImageDelayEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimageiohandler.h:125
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] int currentImageNumber() const

/*
For image formats that support animation, this function returns the sequence number of the current image in the animation. If this function is called before any image is read(), -1 is returned. The number of the first image in the sequence is 0.

If the image format does not support animation, 0 is returned.

See also read().
*/
impl /*struct*/ QImageIOHandler {
  pub fn currentImageNumber_0<RetType, T: QImageIOHandler_currentImageNumber_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.currentImageNumber_0(self);
    // return 1;
  }
}
pub trait QImageIOHandler_currentImageNumber_0<RetType> {
  fn currentImageNumber_0(self , rsthis: & QImageIOHandler) -> RetType;
}
impl<'a> /*trait*/ QImageIOHandler_currentImageNumber_0<i32> for () {
  fn currentImageNumber_0(self , rsthis: & QImageIOHandler) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QImageIOHandler18currentImageNumberEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qimageiohandler.h:126
// index:0
// Public virtual Visibility=Default Availability=Available
// [16] QRect currentImageRect() const

/*
Returns the rect of the current image. If no rect is defined for the image, and empty QRect() is returned.

This function is useful for animations, where only parts of the frame may be updated at a time.
*/
impl /*struct*/ QImageIOHandler {
  pub fn currentImageRect_0<RetType, T: QImageIOHandler_currentImageRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.currentImageRect_0(self);
    // return 1;
  }
}
pub trait QImageIOHandler_currentImageRect_0<RetType> {
  fn currentImageRect_0(self , rsthis: & QImageIOHandler) -> RetType;
}
impl<'a> /*trait*/ QImageIOHandler_currentImageRect_0<usize> for () {
  fn currentImageRect_0(self , rsthis: & QImageIOHandler) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QImageIOHandler16currentImageRectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}


/*
This enum describes the different options supported by QImageIOHandler. Some options are used to query an image for properties, and others are used to toggle the way in which an image should be written.


*/
pub type QImageIOHandler__ImageOption = i32;
// The original size of an image. A handler that supports this option is expected to read the size of the image from the image metadata, and return this size from option() as a QSize.
pub const QImageIOHandler__Size :QImageIOHandler__ImageOption = 0;
// The clip rect, or ROI (Region Of Interest). A handler that supports this option is expected to only read the provided QRect area from the original image in read(), before any other transformation is applied.
pub const QImageIOHandler__ClipRect :QImageIOHandler__ImageOption = 1;
// The image description. Some image formats, such as GIF and PNG, allow embedding of text or comments into the image data (e.g., for storing copyright information). It's common that the text is stored in key-value pairs, but some formats store all text in one continuous block. QImageIOHandler returns the text as one QString, where keys and values are separated by a ':', and keys-value pairs are separated by two newlines (\n\n). For example, "Title: Sunset\n\nAuthor: Jim Smith\nSarah Jones\n\n". Formats that store text in a single block can use "Description" as the key.
pub const QImageIOHandler__Description :QImageIOHandler__ImageOption = 2;
// The scaled clip rect (or ROI, Region Of Interest) of the image. A handler that supports this option is expected to apply the provided clip rect (a QRect), after applying any scaling (ScaleSize) or regular clipping (ClipRect). If the handler does not support this option, QImageReader will apply the scaled clip rect after the image has been read.
pub const QImageIOHandler__ScaledClipRect :QImageIOHandler__ImageOption = 3;
// The scaled size of the image. A handler that supports this option is expected to scale the image to the provided size (a QSize), after applying any clip rect transformation (ClipRect). If the handler does not support this option, QImageReader will perform the scaling after the image has been read.
pub const QImageIOHandler__ScaledSize :QImageIOHandler__ImageOption = 4;
// The compression ratio of the image data. A handler that supports this option is expected to set its compression rate depending on the value of this option (an int) when writing.
pub const QImageIOHandler__CompressionRatio :QImageIOHandler__ImageOption = 5;
// The gamma level of the image. A handler that supports this option is expected to set the image gamma level depending on the value of this option (a float) when writing.
pub const QImageIOHandler__Gamma :QImageIOHandler__ImageOption = 6;
// The quality level of the image. A handler that supports this option is expected to set the image quality level depending on the value of this option (an int) when writing.
pub const QImageIOHandler__Quality :QImageIOHandler__ImageOption = 7;
// The name of the image. A handler that supports this option is expected to read the name from the image metadata and return this as a QString, or when writing an image it is expected to store the name in the image metadata.
pub const QImageIOHandler__Name :QImageIOHandler__ImageOption = 8;
// The subtype of the image. A handler that supports this option can use the subtype value to help when reading and writing images. For example, a PPM handler may have a subtype value of "ppm" or "ppmraw".
pub const QImageIOHandler__SubType :QImageIOHandler__ImageOption = 9;
// 
pub const QImageIOHandler__IncrementalReading :QImageIOHandler__ImageOption = 10;
// 
pub const QImageIOHandler__Endianness :QImageIOHandler__ImageOption = 11;
// 
pub const QImageIOHandler__Animation :QImageIOHandler__ImageOption = 12;
// 
pub const QImageIOHandler__BackgroundColor :QImageIOHandler__ImageOption = 13;
// 
pub const QImageIOHandler__ImageFormat :QImageIOHandler__ImageOption = 14;
// 
pub const QImageIOHandler__SupportedSubTypes :QImageIOHandler__ImageOption = 15;
// 
pub const QImageIOHandler__OptimizedWrite :QImageIOHandler__ImageOption = 16;
// 
pub const QImageIOHandler__ProgressiveScanWrite :QImageIOHandler__ImageOption = 17;
// 
pub const QImageIOHandler__ImageTransformation :QImageIOHandler__ImageOption = 18;
// 
pub const QImageIOHandler__TransformedByDefault :QImageIOHandler__ImageOption = 19;
pub fn QImageIOHandler_ImageOptionItemName(val: i32) ->String {
  match val {
     QImageIOHandler__Size => // 0
     {return String::from("Size");}
     QImageIOHandler__ClipRect => // 1
     {return String::from("ClipRect");}
     QImageIOHandler__Description => // 2
     {return String::from("Description");}
     QImageIOHandler__ScaledClipRect => // 3
     {return String::from("ScaledClipRect");}
     QImageIOHandler__ScaledSize => // 4
     {return String::from("ScaledSize");}
     QImageIOHandler__CompressionRatio => // 5
     {return String::from("CompressionRatio");}
     QImageIOHandler__Gamma => // 6
     {return String::from("Gamma");}
     QImageIOHandler__Quality => // 7
     {return String::from("Quality");}
     QImageIOHandler__Name => // 8
     {return String::from("Name");}
     QImageIOHandler__SubType => // 9
     {return String::from("SubType");}
     QImageIOHandler__IncrementalReading => // 10
     {return String::from("IncrementalReading");}
     QImageIOHandler__Endianness => // 11
     {return String::from("Endianness");}
     QImageIOHandler__Animation => // 12
     {return String::from("Animation");}
     QImageIOHandler__BackgroundColor => // 13
     {return String::from("BackgroundColor");}
     QImageIOHandler__ImageFormat => // 14
     {return String::from("ImageFormat");}
     QImageIOHandler__SupportedSubTypes => // 15
     {return String::from("SupportedSubTypes");}
     QImageIOHandler__OptimizedWrite => // 16
     {return String::from("OptimizedWrite");}
     QImageIOHandler__ProgressiveScanWrite => // 17
     {return String::from("ProgressiveScanWrite");}
     QImageIOHandler__ImageTransformation => // 18
     {return String::from("ImageTransformation");}
     QImageIOHandler__TransformedByDefault => // 19
     {return String::from("TransformedByDefault");}
  _ => {return format!("{}", val);}
}
}
pub fn QImageIOHandler_ImageOptionItemName_s(val: i32) ->String {
  //var nilthis *QImageIOHandler
  //return nilthis.ImageOptionItemName(val);
  return QImageIOHandler_ImageOptionItemName(val);
}


/*


*/
pub type QImageIOHandler__Transformation = i32;
// 
pub const QImageIOHandler__TransformationNone :QImageIOHandler__Transformation = 0;
// 
pub const QImageIOHandler__TransformationMirror :QImageIOHandler__Transformation = 1;
// 
pub const QImageIOHandler__TransformationFlip :QImageIOHandler__Transformation = 2;
// 
pub const QImageIOHandler__TransformationRotate180 :QImageIOHandler__Transformation = 3;
// 
pub const QImageIOHandler__TransformationRotate90 :QImageIOHandler__Transformation = 4;
// 
pub const QImageIOHandler__TransformationMirrorAndRotate90 :QImageIOHandler__Transformation = 5;
// 
pub const QImageIOHandler__TransformationFlipAndRotate90 :QImageIOHandler__Transformation = 6;
// 
pub const QImageIOHandler__TransformationRotate270 :QImageIOHandler__Transformation = 7;
pub fn QImageIOHandler_TransformationItemName(val: i32) ->String {
  match val {
     QImageIOHandler__TransformationNone => // 0
     {return String::from("TransformationNone");}
     QImageIOHandler__TransformationMirror => // 1
     {return String::from("TransformationMirror");}
     QImageIOHandler__TransformationFlip => // 2
     {return String::from("TransformationFlip");}
     QImageIOHandler__TransformationRotate180 => // 3
     {return String::from("TransformationRotate180");}
     QImageIOHandler__TransformationRotate90 => // 4
     {return String::from("TransformationRotate90");}
     QImageIOHandler__TransformationMirrorAndRotate90 => // 5
     {return String::from("TransformationMirrorAndRotate90");}
     QImageIOHandler__TransformationFlipAndRotate90 => // 6
     {return String::from("TransformationFlipAndRotate90");}
     QImageIOHandler__TransformationRotate270 => // 7
     {return String::from("TransformationRotate270");}
  _ => {return format!("{}", val);}
}
}
pub fn QImageIOHandler_TransformationItemName_s(val: i32) ->String {
  //var nilthis *QImageIOHandler
  //return nilthis.TransformationItemName(val);
  return QImageIOHandler_TransformationItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
