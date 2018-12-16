

// mod ::gui::QPictureIO
// package qtgui
// /usr/include/qt/QtGui/qpicture.h
// #include <qpicture.h>
// #include <QtGui>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 24
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
#[derive(Default)] // class sizeof(QPictureIO)=8
pub struct QPictureIO {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QPictureIO_ITF interface {
//    QPictureIO_PTR() *QPictureIO
//}
//func (ptr *QPictureIO) QPictureIO_PTR() *QPictureIO { return ptr }

impl /*struct*/ QPictureIO {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QPictureIO {
    return QPictureIO{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QPictureIO {
//  type Target = QPictureIOBASE;
//
//  fn deref(&self) -> &QPictureIOBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QPictureIOBASE> for QPictureIO {
//  fn as_ref(& self) -> & QPictureIOBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qpicture.h:134
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QPictureIO()

/*

*/
// QPictureIO() ctx.fn_proto_cpp
impl /*struct*/ QPictureIO {
  pub fn QPictureIO_0<T: QPictureIO_QPictureIO_0>(value: T) -> QPictureIO {
    let rsthis = value.QPictureIO_0();
    return rsthis;
    // return 1;
  }
}

pub trait QPictureIO_QPictureIO_0 {
  fn QPictureIO_0(self) -> QPictureIO;
}
// QPictureIO() ctx.fn_proto_cpp
impl<'a> /*trait*/ QPictureIO_QPictureIO_0 for () {
  fn QPictureIO_0(self) -> QPictureIO {
    // unsafe{_ZN10QPictureIOC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN10QPictureIOC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QPictureIO{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpicture.h:135
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QPictureIO(QIODevice *, const char *)

/*

*/
// QPictureIO(QIODevice *, const char *) ctx.fn_proto_cpp
impl /*struct*/ QPictureIO {
  pub fn QPictureIO_1<T: QPictureIO_QPictureIO_1>(value: T) -> QPictureIO {
    let rsthis = value.QPictureIO_1();
    return rsthis;
    // return 1;
  }
}

pub trait QPictureIO_QPictureIO_1 {
  fn QPictureIO_1(self) -> QPictureIO;
}
// QPictureIO(QIODevice *, const char *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QPictureIO_QPictureIO_1 for (usize,usize) {
  fn QPictureIO_1(self) -> QPictureIO {
    // unsafe{_ZN10QPictureIOC2EP9QIODevicePKc()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (self.1) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN10QPictureIOC2EP9QIODevicePKc", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QPictureIO{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpicture.h:136
// index:2
// Public Visibility=Default Availability=Available
// [-2] void QPictureIO(const QString &, const char *)

/*

*/
// QPictureIO(const QString &, const char *) ctx.fn_proto_cpp
impl /*struct*/ QPictureIO {
  pub fn QPictureIO_2<T: QPictureIO_QPictureIO_2>(value: T) -> QPictureIO {
    let rsthis = value.QPictureIO_2();
    return rsthis;
    // return 1;
  }
}

pub trait QPictureIO_QPictureIO_2 {
  fn QPictureIO_2(self) -> QPictureIO;
}
// QPictureIO(const QString &, const char *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QPictureIO_QPictureIO_2 for (usize,usize) {
  fn QPictureIO_2(self) -> QPictureIO {
    // unsafe{_ZN10QPictureIOC2ERK7QStringPKc()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (self.1) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN10QPictureIOC2ERK7QStringPKc", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QPictureIO{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpicture.h:137
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ~QPictureIO()

/*

*/
pub fn DeleteQPictureIO(this :*mut QPictureIO) {
    // let rv = qtrt::InvokeQtFunc6("_ZN10QPictureIOD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 8)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qpicture.h:139
// index:0
// Public Visibility=Default Availability=Available
// [32] const QPicture & picture() const

/*

*/
impl /*struct*/ QPictureIO {
  pub fn picture_0<RetType, T: QPictureIO_picture_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.picture_0(self);
    // return 1;
  }
}
pub trait QPictureIO_picture_0<RetType> {
  fn picture_0(self , rsthis: & QPictureIO) -> RetType;
}
impl<'a> /*trait*/ QPictureIO_picture_0<usize> for () {
  fn picture_0(self , rsthis: & QPictureIO) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QPictureIO7pictureEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpicture.h:140
// index:0
// Public Visibility=Default Availability=Available
// [4] int status() const

/*

*/
impl /*struct*/ QPictureIO {
  pub fn status_0<RetType, T: QPictureIO_status_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.status_0(self);
    // return 1;
  }
}
pub trait QPictureIO_status_0<RetType> {
  fn status_0(self , rsthis: & QPictureIO) -> RetType;
}
impl<'a> /*trait*/ QPictureIO_status_0<i32> for () {
  fn status_0(self , rsthis: & QPictureIO) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QPictureIO6statusEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpicture.h:141
// index:0
// Public Visibility=Default Availability=Available
// [8] const char * format() const

/*

*/
impl /*struct*/ QPictureIO {
  pub fn format_0<RetType, T: QPictureIO_format_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.format_0(self);
    // return 1;
  }
}
pub trait QPictureIO_format_0<RetType> {
  fn format_0(self , rsthis: & QPictureIO) -> RetType;
}
impl<'a> /*trait*/ QPictureIO_format_0<usize> for () {
  fn format_0(self , rsthis: & QPictureIO) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QPictureIO6formatEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpicture.h:142
// index:0
// Public Visibility=Default Availability=Available
// [8] QIODevice * ioDevice() const

/*

*/
impl /*struct*/ QPictureIO {
  pub fn ioDevice_0<RetType, T: QPictureIO_ioDevice_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.ioDevice_0(self);
    // return 1;
  }
}
pub trait QPictureIO_ioDevice_0<RetType> {
  fn ioDevice_0(self , rsthis: & QPictureIO) -> RetType;
}
impl<'a> /*trait*/ QPictureIO_ioDevice_0<usize> for () {
  fn ioDevice_0(self , rsthis: & QPictureIO) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QPictureIO8ioDeviceEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpicture.h:143
// index:0
// Public Visibility=Default Availability=Available
// [8] QString fileName() const

/*

*/
impl /*struct*/ QPictureIO {
  pub fn fileName_0<RetType, T: QPictureIO_fileName_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fileName_0(self);
    // return 1;
  }
}
pub trait QPictureIO_fileName_0<RetType> {
  fn fileName_0(self , rsthis: & QPictureIO) -> RetType;
}
impl<'a> /*trait*/ QPictureIO_fileName_0<usize> for () {
  fn fileName_0(self , rsthis: & QPictureIO) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QPictureIO8fileNameEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpicture.h:144
// index:0
// Public Visibility=Default Availability=Available
// [4] int quality() const

/*

*/
impl /*struct*/ QPictureIO {
  pub fn quality_0<RetType, T: QPictureIO_quality_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.quality_0(self);
    // return 1;
  }
}
pub trait QPictureIO_quality_0<RetType> {
  fn quality_0(self , rsthis: & QPictureIO) -> RetType;
}
impl<'a> /*trait*/ QPictureIO_quality_0<i32> for () {
  fn quality_0(self , rsthis: & QPictureIO) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QPictureIO7qualityEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpicture.h:145
// index:0
// Public Visibility=Default Availability=Available
// [8] QString description() const

/*

*/
impl /*struct*/ QPictureIO {
  pub fn description_0<RetType, T: QPictureIO_description_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.description_0(self);
    // return 1;
  }
}
pub trait QPictureIO_description_0<RetType> {
  fn description_0(self , rsthis: & QPictureIO) -> RetType;
}
impl<'a> /*trait*/ QPictureIO_description_0<usize> for () {
  fn description_0(self , rsthis: & QPictureIO) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QPictureIO11descriptionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpicture.h:146
// index:0
// Public Visibility=Default Availability=Available
// [8] const char * parameters() const

/*

*/
impl /*struct*/ QPictureIO {
  pub fn parameters_0<RetType, T: QPictureIO_parameters_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.parameters_0(self);
    // return 1;
  }
}
pub trait QPictureIO_parameters_0<RetType> {
  fn parameters_0(self , rsthis: & QPictureIO) -> RetType;
}
impl<'a> /*trait*/ QPictureIO_parameters_0<usize> for () {
  fn parameters_0(self , rsthis: & QPictureIO) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QPictureIO10parametersEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpicture.h:147
// index:0
// Public Visibility=Default Availability=Available
// [4] float gamma() const

/*

*/
impl /*struct*/ QPictureIO {
  pub fn gamma_0<RetType, T: QPictureIO_gamma_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.gamma_0(self);
    // return 1;
  }
}
pub trait QPictureIO_gamma_0<RetType> {
  fn gamma_0(self , rsthis: & QPictureIO) -> RetType;
}
impl<'a> /*trait*/ QPictureIO_gamma_0<f32> for () {
  fn gamma_0(self , rsthis: & QPictureIO) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QPictureIO5gammaEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpicture.h:149
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setPicture(const QPicture &)

/*

*/
impl /*struct*/ QPictureIO {
  pub fn setPicture_0<RetType, T: QPictureIO_setPicture_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPicture_0(self);
    // return 1;
  }
}
pub trait QPictureIO_setPicture_0<RetType> {
  fn setPicture_0(self , rsthis: & QPictureIO) -> RetType;
}
impl<'a> /*trait*/ QPictureIO_setPicture_0<(/*void*/)> for (usize) {
  fn setPicture_0(self , rsthis: & QPictureIO) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN10QPictureIO10setPictureERK8QPicture", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpicture.h:150
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setStatus(int)

/*

*/
impl /*struct*/ QPictureIO {
  pub fn setStatus_0<RetType, T: QPictureIO_setStatus_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setStatus_0(self);
    // return 1;
  }
}
pub trait QPictureIO_setStatus_0<RetType> {
  fn setStatus_0(self , rsthis: & QPictureIO) -> RetType;
}
impl<'a> /*trait*/ QPictureIO_setStatus_0<(/*void*/)> for (i32) {
  fn setStatus_0(self , rsthis: & QPictureIO) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QPictureIO9setStatusEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpicture.h:151
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFormat(const char *)

/*

*/
impl /*struct*/ QPictureIO {
  pub fn setFormat_0<RetType, T: QPictureIO_setFormat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFormat_0(self);
    // return 1;
  }
}
pub trait QPictureIO_setFormat_0<RetType> {
  fn setFormat_0(self , rsthis: & QPictureIO) -> RetType;
}
impl<'a> /*trait*/ QPictureIO_setFormat_0<(/*void*/)> for (usize) {
  fn setFormat_0(self , rsthis: & QPictureIO) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN10QPictureIO9setFormatEPKc", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpicture.h:152
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setIODevice(QIODevice *)

/*

*/
impl /*struct*/ QPictureIO {
  pub fn setIODevice_0<RetType, T: QPictureIO_setIODevice_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setIODevice_0(self);
    // return 1;
  }
}
pub trait QPictureIO_setIODevice_0<RetType> {
  fn setIODevice_0(self , rsthis: & QPictureIO) -> RetType;
}
impl<'a> /*trait*/ QPictureIO_setIODevice_0<(/*void*/)> for (usize) {
  fn setIODevice_0(self , rsthis: & QPictureIO) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN10QPictureIO11setIODeviceEP9QIODevice", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpicture.h:153
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFileName(const QString &)

/*

*/
impl /*struct*/ QPictureIO {
  pub fn setFileName_0<RetType, T: QPictureIO_setFileName_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFileName_0(self);
    // return 1;
  }
}
pub trait QPictureIO_setFileName_0<RetType> {
  fn setFileName_0(self , rsthis: & QPictureIO) -> RetType;
}
impl<'a> /*trait*/ QPictureIO_setFileName_0<(/*void*/)> for (usize) {
  fn setFileName_0(self , rsthis: & QPictureIO) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN10QPictureIO11setFileNameERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpicture.h:154
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setQuality(int)

/*

*/
impl /*struct*/ QPictureIO {
  pub fn setQuality_0<RetType, T: QPictureIO_setQuality_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setQuality_0(self);
    // return 1;
  }
}
pub trait QPictureIO_setQuality_0<RetType> {
  fn setQuality_0(self , rsthis: & QPictureIO) -> RetType;
}
impl<'a> /*trait*/ QPictureIO_setQuality_0<(/*void*/)> for (i32) {
  fn setQuality_0(self , rsthis: & QPictureIO) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QPictureIO10setQualityEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpicture.h:155
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDescription(const QString &)

/*

*/
impl /*struct*/ QPictureIO {
  pub fn setDescription_0<RetType, T: QPictureIO_setDescription_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDescription_0(self);
    // return 1;
  }
}
pub trait QPictureIO_setDescription_0<RetType> {
  fn setDescription_0(self , rsthis: & QPictureIO) -> RetType;
}
impl<'a> /*trait*/ QPictureIO_setDescription_0<(/*void*/)> for (usize) {
  fn setDescription_0(self , rsthis: & QPictureIO) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN10QPictureIO14setDescriptionERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpicture.h:156
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setParameters(const char *)

/*

*/
impl /*struct*/ QPictureIO {
  pub fn setParameters_0<RetType, T: QPictureIO_setParameters_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setParameters_0(self);
    // return 1;
  }
}
pub trait QPictureIO_setParameters_0<RetType> {
  fn setParameters_0(self , rsthis: & QPictureIO) -> RetType;
}
impl<'a> /*trait*/ QPictureIO_setParameters_0<(/*void*/)> for (usize) {
  fn setParameters_0(self , rsthis: & QPictureIO) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN10QPictureIO13setParametersEPKc", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpicture.h:157
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setGamma(float)

/*

*/
impl /*struct*/ QPictureIO {
  pub fn setGamma_0<RetType, T: QPictureIO_setGamma_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setGamma_0(self);
    // return 1;
  }
}
pub trait QPictureIO_setGamma_0<RetType> {
  fn setGamma_0(self , rsthis: & QPictureIO) -> RetType;
}
impl<'a> /*trait*/ QPictureIO_setGamma_0<(/*void*/)> for (f32) {
  fn setGamma_0(self , rsthis: & QPictureIO) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QPictureIO8setGammaEf", 1,qtrt::FFITY_FLOAT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpicture.h:159
// index:0
// Public Visibility=Default Availability=Available
// [1] bool read()

/*

*/
impl /*struct*/ QPictureIO {
  pub fn read_0<RetType, T: QPictureIO_read_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.read_0(self);
    // return 1;
  }
}
pub trait QPictureIO_read_0<RetType> {
  fn read_0(self , rsthis: & QPictureIO) -> RetType;
}
impl<'a> /*trait*/ QPictureIO_read_0<bool> for () {
  fn read_0(self , rsthis: & QPictureIO) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QPictureIO4readEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpicture.h:160
// index:0
// Public Visibility=Default Availability=Available
// [1] bool write()

/*

*/
impl /*struct*/ QPictureIO {
  pub fn write_0<RetType, T: QPictureIO_write_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.write_0(self);
    // return 1;
  }
}
pub trait QPictureIO_write_0<RetType> {
  fn write_0(self , rsthis: & QPictureIO) -> RetType;
}
impl<'a> /*trait*/ QPictureIO_write_0<bool> for () {
  fn write_0(self , rsthis: & QPictureIO) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QPictureIO5writeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpicture.h:162
// index:0
// Public static Visibility=Default Availability=Available
// [8] QByteArray pictureFormat(const QString &)

/*

*/
impl /*struct*/ QPictureIO {
  pub fn pictureFormat_0<RetType, T: QPictureIO_pictureFormat_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.pictureFormat_0();
    // return 1;
  }
}
pub trait QPictureIO_pictureFormat_0<RetType> {
  fn pictureFormat_0(self ) -> RetType;
}
impl<'a> /*trait*/ QPictureIO_pictureFormat_0<usize> for (usize) {
  fn pictureFormat_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QPictureIO13pictureFormatERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpicture.h:163
// index:1
// Public static Visibility=Default Availability=Available
// [8] QByteArray pictureFormat(QIODevice *)

/*

*/
impl /*struct*/ QPictureIO {
  pub fn pictureFormat_1<RetType, T: QPictureIO_pictureFormat_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.pictureFormat_1();
    // return 1;
  }
}
pub trait QPictureIO_pictureFormat_1<RetType> {
  fn pictureFormat_1(self ) -> RetType;
}
impl<'a> /*trait*/ QPictureIO_pictureFormat_1<usize> for (usize) {
  fn pictureFormat_1(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QPictureIO13pictureFormatEP9QIODevice", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
