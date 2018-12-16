

// mod ::gui::QMovie
// package qtgui
// /usr/include/qt/QtGui/qmovie.h
// #include <qmovie.h>
// #include <QtGui>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 61
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
#[derive(Default)] // class sizeof(QMovie)=16
pub struct QMovie {
  qbase: QObject,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QMovie_ITF interface {
//    qtcore.QObject_ITF
//    QMovie_PTR() *QMovie
//}
//func (ptr *QMovie) QMovie_PTR() *QMovie { return ptr }

impl /*struct*/ QMovie {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QMovie {
    return QMovie{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QMovie {
//  type Target = QMovieBASE;
//
//  fn deref(&self) -> &QMovieBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QMovieBASE> for QMovie {
//  fn as_ref(& self) -> & QMovieBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qmovie.h:65
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QMovie {
  pub fn metaObject_0<RetType, T: QMovie_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QMovie_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QMovie) -> RetType;
}
impl<'a> /*trait*/ QMovie_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QMovie) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QMovie10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qmovie.h:82
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QMovie(QObject *)

/*
Constructs a QMovie object, passing the parent object to QObject's constructor.

See also setFileName(), setDevice(), and setFormat().
*/
// QMovie(QObject *) ctx.fn_proto_cpp
impl /*struct*/ QMovie {
  pub fn QMovie_0<T: QMovie_QMovie_0>(value: T) -> QMovie {
    let rsthis = value.QMovie_0();
    return rsthis;
    // return 1;
  }
}

pub trait QMovie_QMovie_0 {
  fn QMovie_0(self) -> QMovie;
}
// QMovie(QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QMovie_QMovie_0 for (usize) {
  fn QMovie_0(self) -> QMovie {
    // unsafe{_ZN6QMovieC2EP7QObject()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN6QMovieC2EP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QMovie{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qmovie.h:83
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QMovie(QIODevice *, const QByteArray &, QObject *)

/*
Constructs a QMovie object, passing the parent object to QObject's constructor.

See also setFileName(), setDevice(), and setFormat().
*/
// QMovie(QIODevice *, const QByteArray &, QObject *) ctx.fn_proto_cpp
impl /*struct*/ QMovie {
  pub fn QMovie_1<T: QMovie_QMovie_1>(value: T) -> QMovie {
    let rsthis = value.QMovie_1();
    return rsthis;
    // return 1;
  }
}

pub trait QMovie_QMovie_1 {
  fn QMovie_1(self) -> QMovie;
}
// QMovie(QIODevice *, const QByteArray &, QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QMovie_QMovie_1 for (usize,usize,usize) {
  fn QMovie_1(self) -> QMovie {
    // unsafe{_ZN6QMovieC2EP9QIODeviceRK10QByteArrayP7QObject()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN6QMovieC2EP9QIODeviceRK10QByteArrayP7QObject", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QMovie{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qmovie.h:84
// index:2
// Public Visibility=Default Availability=Available
// [-2] void QMovie(const QString &, const QByteArray &, QObject *)

/*
Constructs a QMovie object, passing the parent object to QObject's constructor.

See also setFileName(), setDevice(), and setFormat().
*/
// QMovie(const QString &, const QByteArray &, QObject *) ctx.fn_proto_cpp
impl /*struct*/ QMovie {
  pub fn QMovie_2<T: QMovie_QMovie_2>(value: T) -> QMovie {
    let rsthis = value.QMovie_2();
    return rsthis;
    // return 1;
  }
}

pub trait QMovie_QMovie_2 {
  fn QMovie_2(self) -> QMovie;
}
// QMovie(const QString &, const QByteArray &, QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QMovie_QMovie_2 for (usize,usize,usize) {
  fn QMovie_2(self) -> QMovie {
    // unsafe{_ZN6QMovieC2ERK7QStringRK10QByteArrayP7QObject()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN6QMovieC2ERK7QStringRK10QByteArrayP7QObject", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QMovie{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qmovie.h:85
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QMovie()

/*

*/
pub fn DeleteQMovie(this :*mut QMovie) {
    // let rv = qtrt::InvokeQtFunc6("_ZN6QMovieD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qmovie.h:89
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDevice(QIODevice *)

/*
Sets the current device to device. QMovie will read image data from this device when the movie is running.

See also device() and setFormat().
*/
impl /*struct*/ QMovie {
  pub fn setDevice_0<RetType, T: QMovie_setDevice_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDevice_0(self);
    // return 1;
  }
}
pub trait QMovie_setDevice_0<RetType> {
  fn setDevice_0(self , rsthis: & QMovie) -> RetType;
}
impl<'a> /*trait*/ QMovie_setDevice_0<(/*void*/)> for (usize) {
  fn setDevice_0(self , rsthis: & QMovie) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN6QMovie9setDeviceEP9QIODevice", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qmovie.h:90
// index:0
// Public Visibility=Default Availability=Available
// [8] QIODevice * device() const

/*
Returns the device QMovie reads image data from. If no device has currently been assigned, 0 is returned.

See also setDevice() and fileName().
*/
impl /*struct*/ QMovie {
  pub fn device_0<RetType, T: QMovie_device_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.device_0(self);
    // return 1;
  }
}
pub trait QMovie_device_0<RetType> {
  fn device_0(self , rsthis: & QMovie) -> RetType;
}
impl<'a> /*trait*/ QMovie_device_0<usize> for () {
  fn device_0(self , rsthis: & QMovie) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QMovie6deviceEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qmovie.h:92
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFileName(const QString &)

/*
Sets the name of the file that QMovie reads image data from, to fileName.

See also fileName(), setDevice(), and setFormat().
*/
impl /*struct*/ QMovie {
  pub fn setFileName_0<RetType, T: QMovie_setFileName_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFileName_0(self);
    // return 1;
  }
}
pub trait QMovie_setFileName_0<RetType> {
  fn setFileName_0(self , rsthis: & QMovie) -> RetType;
}
impl<'a> /*trait*/ QMovie_setFileName_0<(/*void*/)> for (usize) {
  fn setFileName_0(self , rsthis: & QMovie) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN6QMovie11setFileNameERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qmovie.h:93
// index:0
// Public Visibility=Default Availability=Available
// [8] QString fileName() const

/*
Returns the name of the file that QMovie reads image data from. If no file name has been assigned, or if the assigned device is not a file, an empty QString is returned.

See also setFileName() and device().
*/
impl /*struct*/ QMovie {
  pub fn fileName_0<RetType, T: QMovie_fileName_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fileName_0(self);
    // return 1;
  }
}
pub trait QMovie_fileName_0<RetType> {
  fn fileName_0(self , rsthis: & QMovie) -> RetType;
}
impl<'a> /*trait*/ QMovie_fileName_0<usize> for () {
  fn fileName_0(self , rsthis: & QMovie) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QMovie8fileNameEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qmovie.h:95
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFormat(const QByteArray &)

/*
Sets the format that QMovie will use when decoding image data, to format. By default, QMovie will attempt to guess the format of the image data.

You can call supportedFormats() for the full list of formats QMovie supports.

See also format() and QImageReader::supportedImageFormats().
*/
impl /*struct*/ QMovie {
  pub fn setFormat_0<RetType, T: QMovie_setFormat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFormat_0(self);
    // return 1;
  }
}
pub trait QMovie_setFormat_0<RetType> {
  fn setFormat_0(self , rsthis: & QMovie) -> RetType;
}
impl<'a> /*trait*/ QMovie_setFormat_0<(/*void*/)> for (usize) {
  fn setFormat_0(self , rsthis: & QMovie) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN6QMovie9setFormatERK10QByteArray", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qmovie.h:96
// index:0
// Public Visibility=Default Availability=Available
// [8] QByteArray format() const

/*
Returns the format that QMovie uses when decoding image data. If no format has been assigned, an empty QByteArray() is returned.

See also setFormat().
*/
impl /*struct*/ QMovie {
  pub fn format_0<RetType, T: QMovie_format_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.format_0(self);
    // return 1;
  }
}
pub trait QMovie_format_0<RetType> {
  fn format_0(self , rsthis: & QMovie) -> RetType;
}
impl<'a> /*trait*/ QMovie_format_0<usize> for () {
  fn format_0(self , rsthis: & QMovie) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QMovie6formatEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qmovie.h:98
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setBackgroundColor(const QColor &)

/*
For image formats that support it, this function sets the background color to color.

See also backgroundColor().
*/
impl /*struct*/ QMovie {
  pub fn setBackgroundColor_0<RetType, T: QMovie_setBackgroundColor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setBackgroundColor_0(self);
    // return 1;
  }
}
pub trait QMovie_setBackgroundColor_0<RetType> {
  fn setBackgroundColor_0(self , rsthis: & QMovie) -> RetType;
}
impl<'a> /*trait*/ QMovie_setBackgroundColor_0<(/*void*/)> for (usize) {
  fn setBackgroundColor_0(self , rsthis: & QMovie) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN6QMovie18setBackgroundColorERK6QColor", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qmovie.h:99
// index:0
// Public Visibility=Default Availability=Available
// [16] QColor backgroundColor() const

/*
Returns the background color of the movie. If no background color has been assigned, an invalid QColor is returned.

See also setBackgroundColor().
*/
impl /*struct*/ QMovie {
  pub fn backgroundColor_0<RetType, T: QMovie_backgroundColor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.backgroundColor_0(self);
    // return 1;
  }
}
pub trait QMovie_backgroundColor_0<RetType> {
  fn backgroundColor_0(self , rsthis: & QMovie) -> RetType;
}
impl<'a> /*trait*/ QMovie_backgroundColor_0<usize> for () {
  fn backgroundColor_0(self , rsthis: & QMovie) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QMovie15backgroundColorEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qmovie.h:101
// index:0
// Public Visibility=Default Availability=Available
// [4] QMovie::MovieState state() const

/*
Returns the current state of QMovie.

See also MovieState and stateChanged().
*/
impl /*struct*/ QMovie {
  pub fn state_0<RetType, T: QMovie_state_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.state_0(self);
    // return 1;
  }
}
pub trait QMovie_state_0<RetType> {
  fn state_0(self , rsthis: & QMovie) -> RetType;
}
impl<'a> /*trait*/ QMovie_state_0<i32> for () {
  fn state_0(self , rsthis: & QMovie) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QMovie5stateEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qmovie.h:103
// index:0
// Public Visibility=Default Availability=Available
// [16] QRect frameRect() const

/*
Returns the rect of the last frame. If no frame has yet been updated, an invalid QRect is returned.

See also currentImage() and currentPixmap().
*/
impl /*struct*/ QMovie {
  pub fn frameRect_0<RetType, T: QMovie_frameRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.frameRect_0(self);
    // return 1;
  }
}
pub trait QMovie_frameRect_0<RetType> {
  fn frameRect_0(self , rsthis: & QMovie) -> RetType;
}
impl<'a> /*trait*/ QMovie_frameRect_0<usize> for () {
  fn frameRect_0(self , rsthis: & QMovie) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QMovie9frameRectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qmovie.h:104
// index:0
// Public Visibility=Default Availability=Available
// [32] QImage currentImage() const

/*
Returns the current frame as a QImage.

See also currentPixmap() and updated().
*/
impl /*struct*/ QMovie {
  pub fn currentImage_0<RetType, T: QMovie_currentImage_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.currentImage_0(self);
    // return 1;
  }
}
pub trait QMovie_currentImage_0<RetType> {
  fn currentImage_0(self , rsthis: & QMovie) -> RetType;
}
impl<'a> /*trait*/ QMovie_currentImage_0<usize> for () {
  fn currentImage_0(self , rsthis: & QMovie) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QMovie12currentImageEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qmovie.h:105
// index:0
// Public Visibility=Default Availability=Available
// [32] QPixmap currentPixmap() const

/*
Returns the current frame as a QPixmap.

See also currentImage() and updated().
*/
impl /*struct*/ QMovie {
  pub fn currentPixmap_0<RetType, T: QMovie_currentPixmap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.currentPixmap_0(self);
    // return 1;
  }
}
pub trait QMovie_currentPixmap_0<RetType> {
  fn currentPixmap_0(self , rsthis: & QMovie) -> RetType;
}
impl<'a> /*trait*/ QMovie_currentPixmap_0<usize> for () {
  fn currentPixmap_0(self , rsthis: & QMovie) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QMovie13currentPixmapEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qmovie.h:107
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isValid() const

/*
Returns true if the movie is valid (e.g., the image data is readable and the image format is supported); otherwise returns false.

For information about why the movie is not valid, see lastError().
*/
impl /*struct*/ QMovie {
  pub fn isValid_0<RetType, T: QMovie_isValid_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isValid_0(self);
    // return 1;
  }
}
pub trait QMovie_isValid_0<RetType> {
  fn isValid_0(self , rsthis: & QMovie) -> RetType;
}
impl<'a> /*trait*/ QMovie_isValid_0<bool> for () {
  fn isValid_0(self , rsthis: & QMovie) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QMovie7isValidEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qmovie.h:108
// index:0
// Public Visibility=Default Availability=Available
// [4] QImageReader::ImageReaderError lastError() const

/*
Returns the most recent error that occurred while attempting to read image data.

See also lastErrorString().
*/
impl /*struct*/ QMovie {
  pub fn lastError_0<RetType, T: QMovie_lastError_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.lastError_0(self);
    // return 1;
  }
}
pub trait QMovie_lastError_0<RetType> {
  fn lastError_0(self , rsthis: & QMovie) -> RetType;
}
impl<'a> /*trait*/ QMovie_lastError_0<i32> for () {
  fn lastError_0(self , rsthis: & QMovie) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QMovie9lastErrorEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qmovie.h:109
// index:0
// Public Visibility=Default Availability=Available
// [8] QString lastErrorString() const

/*
Returns a human-readable representation of the most recent error that occurred while attempting to read image data.

See also lastError().
*/
impl /*struct*/ QMovie {
  pub fn lastErrorString_0<RetType, T: QMovie_lastErrorString_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.lastErrorString_0(self);
    // return 1;
  }
}
pub trait QMovie_lastErrorString_0<RetType> {
  fn lastErrorString_0(self , rsthis: & QMovie) -> RetType;
}
impl<'a> /*trait*/ QMovie_lastErrorString_0<usize> for () {
  fn lastErrorString_0(self , rsthis: & QMovie) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QMovie15lastErrorStringEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qmovie.h:111
// index:0
// Public Visibility=Default Availability=Available
// [1] bool jumpToFrame(int)

/*
Jumps to frame number frameNumber. Returns true on success; otherwise returns false.
*/
impl /*struct*/ QMovie {
  pub fn jumpToFrame_0<RetType, T: QMovie_jumpToFrame_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.jumpToFrame_0(self);
    // return 1;
  }
}
pub trait QMovie_jumpToFrame_0<RetType> {
  fn jumpToFrame_0(self , rsthis: & QMovie) -> RetType;
}
impl<'a> /*trait*/ QMovie_jumpToFrame_0<bool> for (i32) {
  fn jumpToFrame_0(self , rsthis: & QMovie) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN6QMovie11jumpToFrameEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qmovie.h:112
// index:0
// Public Visibility=Default Availability=Available
// [4] int loopCount() const

/*
Returns the number of times the movie will loop before it finishes. If the movie will only play once (no looping), loopCount returns 0. If the movie loops forever, loopCount returns -1.

Note that, if the image data comes from a sequential device (e.g. a socket), QMovie can only loop the movie if the cacheMode is set to QMovie::CacheAll.
*/
impl /*struct*/ QMovie {
  pub fn loopCount_0<RetType, T: QMovie_loopCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.loopCount_0(self);
    // return 1;
  }
}
pub trait QMovie_loopCount_0<RetType> {
  fn loopCount_0(self , rsthis: & QMovie) -> RetType;
}
impl<'a> /*trait*/ QMovie_loopCount_0<i32> for () {
  fn loopCount_0(self , rsthis: & QMovie) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QMovie9loopCountEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qmovie.h:113
// index:0
// Public Visibility=Default Availability=Available
// [4] int frameCount() const

/*
Returns the number of frames in the movie.

Certain animation formats do not support this feature, in which case 0 is returned.
*/
impl /*struct*/ QMovie {
  pub fn frameCount_0<RetType, T: QMovie_frameCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.frameCount_0(self);
    // return 1;
  }
}
pub trait QMovie_frameCount_0<RetType> {
  fn frameCount_0(self , rsthis: & QMovie) -> RetType;
}
impl<'a> /*trait*/ QMovie_frameCount_0<i32> for () {
  fn frameCount_0(self , rsthis: & QMovie) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QMovie10frameCountEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qmovie.h:114
// index:0
// Public Visibility=Default Availability=Available
// [4] int nextFrameDelay() const

/*
Returns the number of milliseconds QMovie will wait before updating the next frame in the animation.
*/
impl /*struct*/ QMovie {
  pub fn nextFrameDelay_0<RetType, T: QMovie_nextFrameDelay_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.nextFrameDelay_0(self);
    // return 1;
  }
}
pub trait QMovie_nextFrameDelay_0<RetType> {
  fn nextFrameDelay_0(self , rsthis: & QMovie) -> RetType;
}
impl<'a> /*trait*/ QMovie_nextFrameDelay_0<i32> for () {
  fn nextFrameDelay_0(self , rsthis: & QMovie) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QMovie14nextFrameDelayEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qmovie.h:115
// index:0
// Public Visibility=Default Availability=Available
// [4] int currentFrameNumber() const

/*
Returns the sequence number of the current frame. The number of the first frame in the movie is 0.
*/
impl /*struct*/ QMovie {
  pub fn currentFrameNumber_0<RetType, T: QMovie_currentFrameNumber_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.currentFrameNumber_0(self);
    // return 1;
  }
}
pub trait QMovie_currentFrameNumber_0<RetType> {
  fn currentFrameNumber_0(self , rsthis: & QMovie) -> RetType;
}
impl<'a> /*trait*/ QMovie_currentFrameNumber_0<i32> for () {
  fn currentFrameNumber_0(self , rsthis: & QMovie) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QMovie18currentFrameNumberEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qmovie.h:117
// index:0
// Public Visibility=Default Availability=Available
// [4] int speed() const

/*

*/
impl /*struct*/ QMovie {
  pub fn speed_0<RetType, T: QMovie_speed_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.speed_0(self);
    // return 1;
  }
}
pub trait QMovie_speed_0<RetType> {
  fn speed_0(self , rsthis: & QMovie) -> RetType;
}
impl<'a> /*trait*/ QMovie_speed_0<i32> for () {
  fn speed_0(self , rsthis: & QMovie) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QMovie5speedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qmovie.h:119
// index:0
// Public Visibility=Default Availability=Available
// [8] QSize scaledSize()

/*
Returns the scaled size of frames.

This function was introduced in  Qt 4.1.

See also setScaledSize() and QImageReader::scaledSize().
*/
impl /*struct*/ QMovie {
  pub fn scaledSize_0<RetType, T: QMovie_scaledSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.scaledSize_0(self);
    // return 1;
  }
}
pub trait QMovie_scaledSize_0<RetType> {
  fn scaledSize_0(self , rsthis: & QMovie) -> RetType;
}
impl<'a> /*trait*/ QMovie_scaledSize_0<usize> for () {
  fn scaledSize_0(self , rsthis: & QMovie) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN6QMovie10scaledSizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qmovie.h:120
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setScaledSize(const QSize &)

/*
Sets the scaled frame size to size.

This function was introduced in  Qt 4.1.

See also scaledSize() and QImageReader::setScaledSize().
*/
impl /*struct*/ QMovie {
  pub fn setScaledSize_0<RetType, T: QMovie_setScaledSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setScaledSize_0(self);
    // return 1;
  }
}
pub trait QMovie_setScaledSize_0<RetType> {
  fn setScaledSize_0(self , rsthis: & QMovie) -> RetType;
}
impl<'a> /*trait*/ QMovie_setScaledSize_0<(/*void*/)> for (usize) {
  fn setScaledSize_0(self , rsthis: & QMovie) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN6QMovie13setScaledSizeERK5QSize", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qmovie.h:122
// index:0
// Public Visibility=Default Availability=Available
// [4] QMovie::CacheMode cacheMode() const

/*

*/
impl /*struct*/ QMovie {
  pub fn cacheMode_0<RetType, T: QMovie_cacheMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cacheMode_0(self);
    // return 1;
  }
}
pub trait QMovie_cacheMode_0<RetType> {
  fn cacheMode_0(self , rsthis: & QMovie) -> RetType;
}
impl<'a> /*trait*/ QMovie_cacheMode_0<i32> for () {
  fn cacheMode_0(self , rsthis: & QMovie) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QMovie9cacheModeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qmovie.h:123
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCacheMode(QMovie::CacheMode)

/*

*/
impl /*struct*/ QMovie {
  pub fn setCacheMode_0<RetType, T: QMovie_setCacheMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCacheMode_0(self);
    // return 1;
  }
}
pub trait QMovie_setCacheMode_0<RetType> {
  fn setCacheMode_0(self , rsthis: & QMovie) -> RetType;
}
impl<'a> /*trait*/ QMovie_setCacheMode_0<(/*void*/)> for (i32) {
  fn setCacheMode_0(self , rsthis: & QMovie) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN6QMovie12setCacheModeENS_9CacheModeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qmovie.h:126
// index:0
// Public Visibility=Default Availability=Available
// [-2] void started()

/*
This signal is emitted after QMovie::start() has been called, and QMovie has entered QMovie::Running state.
*/
impl /*struct*/ QMovie {
  pub fn started_0<RetType, T: QMovie_started_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.started_0(self);
    // return 1;
  }
}
pub trait QMovie_started_0<RetType> {
  fn started_0(self , rsthis: & QMovie) -> RetType;
}
impl<'a> /*trait*/ QMovie_started_0<(/*void*/)> for () {
  fn started_0(self , rsthis: & QMovie) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN6QMovie7startedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qmovie.h:127
// index:0
// Public Visibility=Default Availability=Available
// [-2] void resized(const QSize &)

/*
This signal is emitted when the current frame has been resized to size. This effect is sometimes used in animations as an alternative to replacing the frame. You can call currentImage() or currentPixmap() to get a copy of the updated frame.
*/
impl /*struct*/ QMovie {
  pub fn resized_0<RetType, T: QMovie_resized_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resized_0(self);
    // return 1;
  }
}
pub trait QMovie_resized_0<RetType> {
  fn resized_0(self , rsthis: & QMovie) -> RetType;
}
impl<'a> /*trait*/ QMovie_resized_0<(/*void*/)> for (usize) {
  fn resized_0(self , rsthis: & QMovie) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN6QMovie7resizedERK5QSize", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qmovie.h:128
// index:0
// Public Visibility=Default Availability=Available
// [-2] void updated(const QRect &)

/*
This signal is emitted when the rect rect in the current frame has been updated. You can call currentImage() or currentPixmap() to get a copy of the updated frame.
*/
impl /*struct*/ QMovie {
  pub fn updated_0<RetType, T: QMovie_updated_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.updated_0(self);
    // return 1;
  }
}
pub trait QMovie_updated_0<RetType> {
  fn updated_0(self , rsthis: & QMovie) -> RetType;
}
impl<'a> /*trait*/ QMovie_updated_0<(/*void*/)> for (usize) {
  fn updated_0(self , rsthis: & QMovie) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN6QMovie7updatedERK5QRect", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qmovie.h:129
// index:0
// Public Visibility=Default Availability=Available
// [-2] void stateChanged(QMovie::MovieState)

/*
This signal is emitted every time the state of the movie changes. The new state is specified by state.

See also QMovie::state().
*/
impl /*struct*/ QMovie {
  pub fn stateChanged_0<RetType, T: QMovie_stateChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.stateChanged_0(self);
    // return 1;
  }
}
pub trait QMovie_stateChanged_0<RetType> {
  fn stateChanged_0(self , rsthis: & QMovie) -> RetType;
}
impl<'a> /*trait*/ QMovie_stateChanged_0<(/*void*/)> for (i32) {
  fn stateChanged_0(self , rsthis: & QMovie) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN6QMovie12stateChangedENS_10MovieStateE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qmovie.h:130
// index:0
// Public Visibility=Default Availability=Available
// [-2] void error(QImageReader::ImageReaderError)

/*
This signal is emitted by QMovie when the error error occurred during playback. QMovie will stop the movie, and enter QMovie::NotRunning state.

See also lastError() and lastErrorString().
*/
impl /*struct*/ QMovie {
  pub fn error_0<RetType, T: QMovie_error_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.error_0(self);
    // return 1;
  }
}
pub trait QMovie_error_0<RetType> {
  fn error_0(self , rsthis: & QMovie) -> RetType;
}
impl<'a> /*trait*/ QMovie_error_0<(/*void*/)> for (i32) {
  fn error_0(self , rsthis: & QMovie) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN6QMovie5errorEN12QImageReader16ImageReaderErrorE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qmovie.h:131
// index:0
// Public Visibility=Default Availability=Available
// [-2] void finished()

/*
This signal is emitted when the movie has finished.

See also QMovie::stop().
*/
impl /*struct*/ QMovie {
  pub fn finished_0<RetType, T: QMovie_finished_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.finished_0(self);
    // return 1;
  }
}
pub trait QMovie_finished_0<RetType> {
  fn finished_0(self , rsthis: & QMovie) -> RetType;
}
impl<'a> /*trait*/ QMovie_finished_0<(/*void*/)> for () {
  fn finished_0(self , rsthis: & QMovie) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN6QMovie8finishedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qmovie.h:132
// index:0
// Public Visibility=Default Availability=Available
// [-2] void frameChanged(int)

/*
This signal is emitted when the frame number has changed to frameNumber. You can call currentImage() or currentPixmap() to get a copy of the frame.

This function was introduced in  Qt 4.1.
*/
impl /*struct*/ QMovie {
  pub fn frameChanged_0<RetType, T: QMovie_frameChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.frameChanged_0(self);
    // return 1;
  }
}
pub trait QMovie_frameChanged_0<RetType> {
  fn frameChanged_0(self , rsthis: & QMovie) -> RetType;
}
impl<'a> /*trait*/ QMovie_frameChanged_0<(/*void*/)> for (i32) {
  fn frameChanged_0(self , rsthis: & QMovie) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN6QMovie12frameChangedEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qmovie.h:135
// index:0
// Public Visibility=Default Availability=Available
// [-2] void start()

/*
Starts the movie. QMovie will enter Running state, and start emitting updated() and resized() as the movie progresses.

If QMovie is in the Paused state, this function is equivalent to calling setPaused(false). If QMovie is already in the Running state, this function does nothing.

See also stop() and setPaused().
*/
impl /*struct*/ QMovie {
  pub fn start_0<RetType, T: QMovie_start_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.start_0(self);
    // return 1;
  }
}
pub trait QMovie_start_0<RetType> {
  fn start_0(self , rsthis: & QMovie) -> RetType;
}
impl<'a> /*trait*/ QMovie_start_0<(/*void*/)> for () {
  fn start_0(self , rsthis: & QMovie) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN6QMovie5startEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qmovie.h:136
// index:0
// Public Visibility=Default Availability=Available
// [1] bool jumpToNextFrame()

/*
Jumps to the next frame. Returns true on success; otherwise returns false.
*/
impl /*struct*/ QMovie {
  pub fn jumpToNextFrame_0<RetType, T: QMovie_jumpToNextFrame_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.jumpToNextFrame_0(self);
    // return 1;
  }
}
pub trait QMovie_jumpToNextFrame_0<RetType> {
  fn jumpToNextFrame_0(self , rsthis: & QMovie) -> RetType;
}
impl<'a> /*trait*/ QMovie_jumpToNextFrame_0<bool> for () {
  fn jumpToNextFrame_0(self , rsthis: & QMovie) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN6QMovie15jumpToNextFrameEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qmovie.h:137
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setPaused(bool)

/*
If paused is true, QMovie will enter Paused state and emit stateChanged(Paused); otherwise it will enter Running state and emit stateChanged(Running).

See also state().
*/
impl /*struct*/ QMovie {
  pub fn setPaused_0<RetType, T: QMovie_setPaused_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPaused_0(self);
    // return 1;
  }
}
pub trait QMovie_setPaused_0<RetType> {
  fn setPaused_0(self , rsthis: & QMovie) -> RetType;
}
impl<'a> /*trait*/ QMovie_setPaused_0<(/*void*/)> for (bool) {
  fn setPaused_0(self , rsthis: & QMovie) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN6QMovie9setPausedEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qmovie.h:138
// index:0
// Public Visibility=Default Availability=Available
// [-2] void stop()

/*
Stops the movie. QMovie enters NotRunning state, and stops emitting updated() and resized(). If start() is called again, the movie will restart from the beginning.

If QMovie is already in the NotRunning state, this function does nothing.

See also start() and setPaused().
*/
impl /*struct*/ QMovie {
  pub fn stop_0<RetType, T: QMovie_stop_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.stop_0(self);
    // return 1;
  }
}
pub trait QMovie_stop_0<RetType> {
  fn stop_0(self , rsthis: & QMovie) -> RetType;
}
impl<'a> /*trait*/ QMovie_stop_0<(/*void*/)> for () {
  fn stop_0(self , rsthis: & QMovie) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN6QMovie4stopEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qmovie.h:139
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setSpeed(int)

/*

*/
impl /*struct*/ QMovie {
  pub fn setSpeed_0<RetType, T: QMovie_setSpeed_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSpeed_0(self);
    // return 1;
  }
}
pub trait QMovie_setSpeed_0<RetType> {
  fn setSpeed_0(self , rsthis: & QMovie) -> RetType;
}
impl<'a> /*trait*/ QMovie_setSpeed_0<(/*void*/)> for (i32) {
  fn setSpeed_0(self , rsthis: & QMovie) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN6QMovie8setSpeedEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}


/*
This enum describes the different states of QMovie.


*/
pub type QMovie__MovieState = i32;
// The movie is not running. This is QMovie's initial state, and the state it enters after stop() has been called or the movie is finished.
pub const QMovie__NotRunning :QMovie__MovieState = 0;
// The movie is paused, and QMovie stops emitting updated() or resized(). This state is entered after calling pause() or setPaused(true). The current frame number it kept, and the movie will continue with the next frame when unpause() or setPaused(false) is called.
pub const QMovie__Paused :QMovie__MovieState = 1;
// The movie is running.
pub const QMovie__Running :QMovie__MovieState = 2;
pub fn QMovie_MovieStateItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QMovie", val);
}
pub fn QMovie_MovieStateItemName_s(val: i32) ->String {
  //var nilthis *QMovie
  //return nilthis.MovieStateItemName(val);
  return QMovie_MovieStateItemName(val);
}


/*
This enum describes the different cache modes of QMovie.


*/
pub type QMovie__CacheMode = i32;
// No frames are cached (the default).
pub const QMovie__CacheNone :QMovie__CacheMode = 0;
// All frames are cached.
pub const QMovie__CacheAll :QMovie__CacheMode = 1;
pub fn QMovie_CacheModeItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QMovie", val);
}
pub fn QMovie_CacheModeItemName_s(val: i32) ->String {
  //var nilthis *QMovie
  //return nilthis.CacheModeItemName(val);
  return QMovie_CacheModeItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
