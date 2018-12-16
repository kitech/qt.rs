

// mod ::core::QDataStream
// package qtcore
// /usr/include/qt/QtCore/qdatastream.h
// #include <qdatastream.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 8
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
#[derive(Default)] // class sizeof(QDataStream)=32
pub struct QDataStream {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QDataStream_ITF interface {
//    QDataStream_PTR() *QDataStream
//}
//func (ptr *QDataStream) QDataStream_PTR() *QDataStream { return ptr }

impl /*struct*/ QDataStream {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QDataStream {
    return QDataStream{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QDataStream {
//  type Target = QDataStreamBASE;
//
//  fn deref(&self) -> &QDataStreamBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QDataStreamBASE> for QDataStream {
//  fn as_ref(& self) -> & QDataStreamBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qdatastream.h:123
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QDataStream()

/*
Constructs a data stream that has no I/O device.

See also setDevice().
*/
// QDataStream() ctx.fn_proto_cpp
impl /*struct*/ QDataStream {
  pub fn QDataStream_0<T: QDataStream_QDataStream_0>(value: T) -> QDataStream {
    let rsthis = value.QDataStream_0();
    return rsthis;
    // return 1;
  }
}

pub trait QDataStream_QDataStream_0 {
  fn QDataStream_0(self) -> QDataStream;
}
// QDataStream() ctx.fn_proto_cpp
impl<'a> /*trait*/ QDataStream_QDataStream_0 for () {
  fn QDataStream_0(self) -> QDataStream {
    // unsafe{_ZN11QDataStreamC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QDataStreamC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QDataStream{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qdatastream.h:124
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QDataStream(QIODevice *)

/*
Constructs a data stream that has no I/O device.

See also setDevice().
*/
// QDataStream(QIODevice *) ctx.fn_proto_cpp
impl /*struct*/ QDataStream {
  pub fn QDataStream_1<T: QDataStream_QDataStream_1>(value: T) -> QDataStream {
    let rsthis = value.QDataStream_1();
    return rsthis;
    // return 1;
  }
}

pub trait QDataStream_QDataStream_1 {
  fn QDataStream_1(self) -> QDataStream;
}
// QDataStream(QIODevice *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QDataStream_QDataStream_1 for (usize) {
  fn QDataStream_1(self) -> QDataStream {
    // unsafe{_ZN11QDataStreamC2EP9QIODevice()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QDataStreamC2EP9QIODevice", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QDataStream{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qdatastream.h:125
// index:2
// Public Visibility=Default Availability=Available
// [-2] void QDataStream(QByteArray *, QIODevice::OpenMode)

/*
Constructs a data stream that has no I/O device.

See also setDevice().
*/
// QDataStream(QByteArray *, QIODevice::OpenMode) ctx.fn_proto_cpp
impl /*struct*/ QDataStream {
  pub fn QDataStream_2<T: QDataStream_QDataStream_2>(value: T) -> QDataStream {
    let rsthis = value.QDataStream_2();
    return rsthis;
    // return 1;
  }
}

pub trait QDataStream_QDataStream_2 {
  fn QDataStream_2(self) -> QDataStream;
}
// QDataStream(QByteArray *, QIODevice::OpenMode) ctx.fn_proto_cpp
impl<'a> /*trait*/ QDataStream_QDataStream_2 for (usize,i32) {
  fn QDataStream_2(self) -> QDataStream {
    // unsafe{_ZN11QDataStreamC2EP10QByteArray6QFlagsIN9QIODevice12OpenModeFlagEE()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QDataStreamC2EP10QByteArray6QFlagsIN9QIODevice12OpenModeFlagEE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QDataStream{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qdatastream.h:126
// index:3
// Public Visibility=Default Availability=Available
// [-2] void QDataStream(const QByteArray &)

/*
Constructs a data stream that has no I/O device.

See also setDevice().
*/
// QDataStream(const QByteArray &) ctx.fn_proto_cpp
impl /*struct*/ QDataStream {
  pub fn QDataStream_3<T: QDataStream_QDataStream_3>(value: T) -> QDataStream {
    let rsthis = value.QDataStream_3();
    return rsthis;
    // return 1;
  }
}

pub trait QDataStream_QDataStream_3 {
  fn QDataStream_3(self) -> QDataStream;
}
// QDataStream(const QByteArray &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QDataStream_QDataStream_3 for (usize) {
  fn QDataStream_3(self) -> QDataStream {
    // unsafe{_ZN11QDataStreamC2ERK10QByteArray()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QDataStreamC2ERK10QByteArray", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QDataStream{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qdatastream.h:127
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ~QDataStream()

/*

*/
pub fn DeleteQDataStream(this :*mut QDataStream) {
    // let rv = qtrt::InvokeQtFunc6("_ZN11QDataStreamD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 32)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qdatastream.h:129
// index:0
// Public Visibility=Default Availability=Available
// [8] QIODevice * device() const

/*
Returns the I/O device currently set, or 0 if no device is currently set.

See also setDevice().
*/
impl /*struct*/ QDataStream {
  pub fn device_0<RetType, T: QDataStream_device_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.device_0(self);
    // return 1;
  }
}
pub trait QDataStream_device_0<RetType> {
  fn device_0(self , rsthis: & QDataStream) -> RetType;
}
impl<'a> /*trait*/ QDataStream_device_0<usize> for () {
  fn device_0(self , rsthis: & QDataStream) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QDataStream6deviceEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatastream.h:130
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDevice(QIODevice *)

/*
void QDataStream::setDevice(QIODevice *d)

Sets the I/O device to d, which can be 0 to unset to current I/O device.

See also device().
*/
impl /*struct*/ QDataStream {
  pub fn setDevice_0<RetType, T: QDataStream_setDevice_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDevice_0(self);
    // return 1;
  }
}
pub trait QDataStream_setDevice_0<RetType> {
  fn setDevice_0(self , rsthis: & QDataStream) -> RetType;
}
impl<'a> /*trait*/ QDataStream_setDevice_0<(/*void*/)> for (usize) {
  fn setDevice_0(self , rsthis: & QDataStream) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QDataStream9setDeviceEP9QIODevice", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qdatastream.h:131
// index:0
// Public Visibility=Default Availability=Available
// [-2] void unsetDevice()

/*

*/
impl /*struct*/ QDataStream {
  pub fn unsetDevice_0<RetType, T: QDataStream_unsetDevice_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.unsetDevice_0(self);
    // return 1;
  }
}
pub trait QDataStream_unsetDevice_0<RetType> {
  fn unsetDevice_0(self , rsthis: & QDataStream) -> RetType;
}
impl<'a> /*trait*/ QDataStream_unsetDevice_0<(/*void*/)> for () {
  fn unsetDevice_0(self , rsthis: & QDataStream) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN11QDataStream11unsetDeviceEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qdatastream.h:133
// index:0
// Public Visibility=Default Availability=Available
// [1] bool atEnd() const

/*
Returns true if the I/O device has reached the end position (end of the stream or file) or if there is no I/O device set; otherwise returns false.

See also QIODevice::atEnd().
*/
impl /*struct*/ QDataStream {
  pub fn atEnd_0<RetType, T: QDataStream_atEnd_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.atEnd_0(self);
    // return 1;
  }
}
pub trait QDataStream_atEnd_0<RetType> {
  fn atEnd_0(self , rsthis: & QDataStream) -> RetType;
}
impl<'a> /*trait*/ QDataStream_atEnd_0<bool> for () {
  fn atEnd_0(self , rsthis: & QDataStream) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QDataStream5atEndEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatastream.h:135
// index:0
// Public Visibility=Default Availability=Available
// [4] QDataStream::Status status() const

/*
Returns the status of the data stream.

See also Status, setStatus(), and resetStatus().
*/
impl /*struct*/ QDataStream {
  pub fn status_0<RetType, T: QDataStream_status_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.status_0(self);
    // return 1;
  }
}
pub trait QDataStream_status_0<RetType> {
  fn status_0(self , rsthis: & QDataStream) -> RetType;
}
impl<'a> /*trait*/ QDataStream_status_0<i32> for () {
  fn status_0(self , rsthis: & QDataStream) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QDataStream6statusEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatastream.h:136
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setStatus(QDataStream::Status)

/*
Sets the status of the data stream to the status given.

Subsequent calls to setStatus() are ignored until resetStatus() is called.

See also Status, status(), and resetStatus().
*/
impl /*struct*/ QDataStream {
  pub fn setStatus_0<RetType, T: QDataStream_setStatus_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setStatus_0(self);
    // return 1;
  }
}
pub trait QDataStream_setStatus_0<RetType> {
  fn setStatus_0(self , rsthis: & QDataStream) -> RetType;
}
impl<'a> /*trait*/ QDataStream_setStatus_0<(/*void*/)> for (i32) {
  fn setStatus_0(self , rsthis: & QDataStream) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QDataStream9setStatusENS_6StatusE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qdatastream.h:137
// index:0
// Public Visibility=Default Availability=Available
// [-2] void resetStatus()

/*
Resets the status of the data stream.

See also Status, status(), and setStatus().
*/
impl /*struct*/ QDataStream {
  pub fn resetStatus_0<RetType, T: QDataStream_resetStatus_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resetStatus_0(self);
    // return 1;
  }
}
pub trait QDataStream_resetStatus_0<RetType> {
  fn resetStatus_0(self , rsthis: & QDataStream) -> RetType;
}
impl<'a> /*trait*/ QDataStream_resetStatus_0<(/*void*/)> for () {
  fn resetStatus_0(self , rsthis: & QDataStream) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN11QDataStream11resetStatusEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qdatastream.h:139
// index:0
// Public Visibility=Default Availability=Available
// [4] QDataStream::FloatingPointPrecision floatingPointPrecision() const

/*
Returns the floating point precision of the data stream.

This function was introduced in  Qt 4.6.

See also FloatingPointPrecision and setFloatingPointPrecision().
*/
impl /*struct*/ QDataStream {
  pub fn floatingPointPrecision_0<RetType, T: QDataStream_floatingPointPrecision_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.floatingPointPrecision_0(self);
    // return 1;
  }
}
pub trait QDataStream_floatingPointPrecision_0<RetType> {
  fn floatingPointPrecision_0(self , rsthis: & QDataStream) -> RetType;
}
impl<'a> /*trait*/ QDataStream_floatingPointPrecision_0<i32> for () {
  fn floatingPointPrecision_0(self , rsthis: & QDataStream) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QDataStream22floatingPointPrecisionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatastream.h:140
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFloatingPointPrecision(QDataStream::FloatingPointPrecision)

/*
Sets the floating point precision of the data stream to precision. If the floating point precision is DoublePrecision and the version of the data stream is Qt_4_6 or higher, all floating point numbers will be written and read with 64-bit precision. If the floating point precision is SinglePrecision and the version is Qt_4_6 or higher, all floating point numbers will be written and read with 32-bit precision.

For versions prior to Qt_4_6, the precision of floating point numbers in the data stream depends on the stream operator called.

The default is DoublePrecision.

Note that this property does not affect the serialization or deserialization of qfloat16 instances.

Warning: This property must be set to the same value on the object that writes and the object that reads the data stream.

This function was introduced in  Qt 4.6.

See also floatingPointPrecision().
*/
impl /*struct*/ QDataStream {
  pub fn setFloatingPointPrecision_0<RetType, T: QDataStream_setFloatingPointPrecision_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFloatingPointPrecision_0(self);
    // return 1;
  }
}
pub trait QDataStream_setFloatingPointPrecision_0<RetType> {
  fn setFloatingPointPrecision_0(self , rsthis: & QDataStream) -> RetType;
}
impl<'a> /*trait*/ QDataStream_setFloatingPointPrecision_0<(/*void*/)> for (i32) {
  fn setFloatingPointPrecision_0(self , rsthis: & QDataStream) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QDataStream25setFloatingPointPrecisionENS_22FloatingPointPrecisionE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qdatastream.h:142
// index:0
// Public Visibility=Default Availability=Available
// [4] QDataStream::ByteOrder byteOrder() const

/*
Returns the current byte order setting -- either BigEndian or LittleEndian.

See also setByteOrder().
*/
impl /*struct*/ QDataStream {
  pub fn byteOrder_0<RetType, T: QDataStream_byteOrder_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.byteOrder_0(self);
    // return 1;
  }
}
pub trait QDataStream_byteOrder_0<RetType> {
  fn byteOrder_0(self , rsthis: & QDataStream) -> RetType;
}
impl<'a> /*trait*/ QDataStream_byteOrder_0<i32> for () {
  fn byteOrder_0(self , rsthis: & QDataStream) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QDataStream9byteOrderEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatastream.h:143
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setByteOrder(QDataStream::ByteOrder)

/*
Sets the serialization byte order to bo.

The bo parameter can be QDataStream::BigEndian or QDataStream::LittleEndian.

The default setting is big endian. We recommend leaving this setting unless you have special requirements.

See also byteOrder().
*/
impl /*struct*/ QDataStream {
  pub fn setByteOrder_0<RetType, T: QDataStream_setByteOrder_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setByteOrder_0(self);
    // return 1;
  }
}
pub trait QDataStream_setByteOrder_0<RetType> {
  fn setByteOrder_0(self , rsthis: & QDataStream) -> RetType;
}
impl<'a> /*trait*/ QDataStream_setByteOrder_0<(/*void*/)> for (i32) {
  fn setByteOrder_0(self , rsthis: & QDataStream) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QDataStream12setByteOrderENS_9ByteOrderE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qdatastream.h:145
// index:0
// Public Visibility=Default Availability=Available
// [4] int version() const

/*
Returns the version number of the data serialization format.

See also setVersion() and Version.
*/
impl /*struct*/ QDataStream {
  pub fn version_0<RetType, T: QDataStream_version_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.version_0(self);
    // return 1;
  }
}
pub trait QDataStream_version_0<RetType> {
  fn version_0(self , rsthis: & QDataStream) -> RetType;
}
impl<'a> /*trait*/ QDataStream_version_0<i32> for () {
  fn version_0(self , rsthis: & QDataStream) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QDataStream7versionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatastream.h:146
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setVersion(int)

/*
Sets the version number of the data serialization format to v, a value of the Version enum.

You don't have to set a version if you are using the current version of Qt, but for your own custom binary formats we recommend that you do; see Versioning in the Detailed Description.

To accommodate new functionality, the datastream serialization format of some Qt classes has changed in some versions of Qt. If you want to read data that was created by an earlier version of Qt, or write data that can be read by a program that was compiled with an earlier version of Qt, use this function to modify the serialization format used by QDataStream.

The Version enum provides symbolic constants for the different versions of Qt. For example:


  QDataStream out(file);
  out.setVersion(QDataStream::Qt_4_0);



See also version() and Version.
*/
impl /*struct*/ QDataStream {
  pub fn setVersion_0<RetType, T: QDataStream_setVersion_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setVersion_0(self);
    // return 1;
  }
}
pub trait QDataStream_setVersion_0<RetType> {
  fn setVersion_0(self , rsthis: & QDataStream) -> RetType;
}
impl<'a> /*trait*/ QDataStream_setVersion_0<(/*void*/)> for (i32) {
  fn setVersion_0(self , rsthis: & QDataStream) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QDataStream10setVersionEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qdatastream.h:149
// index:0
// Public Visibility=Default Availability=Available
// [32] QDataStream & operator>>(quint8 &)

/*

*/
impl /*struct*/ QDataStream {
  pub fn operator_right_shift_0<RetType, T: QDataStream_operator_right_shift_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_right_shift_0(self);
    // return 1;
  }
}
pub trait QDataStream_operator_right_shift_0<RetType> {
  fn operator_right_shift_0(self , rsthis: & QDataStream) -> RetType;
}
impl<'a> /*trait*/ QDataStream_operator_right_shift_0<usize> for (usize) {
  fn operator_right_shift_0(self , rsthis: & QDataStream) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QDataStreamrsERh", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatastream.h:150
// index:1
// Public Visibility=Default Availability=Available
// [32] QDataStream & operator>>(qint16 &)

/*

*/
impl /*struct*/ QDataStream {
  pub fn operator_right_shift_1<RetType, T: QDataStream_operator_right_shift_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_right_shift_1(self);
    // return 1;
  }
}
pub trait QDataStream_operator_right_shift_1<RetType> {
  fn operator_right_shift_1(self , rsthis: & QDataStream) -> RetType;
}
impl<'a> /*trait*/ QDataStream_operator_right_shift_1<usize> for (usize) {
  fn operator_right_shift_1(self , rsthis: & QDataStream) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QDataStreamrsERs", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatastream.h:151
// index:2
// Public Visibility=Default Availability=Available
// [32] QDataStream & operator>>(quint16 &)

/*

*/
impl /*struct*/ QDataStream {
  pub fn operator_right_shift_2<RetType, T: QDataStream_operator_right_shift_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_right_shift_2(self);
    // return 1;
  }
}
pub trait QDataStream_operator_right_shift_2<RetType> {
  fn operator_right_shift_2(self , rsthis: & QDataStream) -> RetType;
}
impl<'a> /*trait*/ QDataStream_operator_right_shift_2<usize> for (usize) {
  fn operator_right_shift_2(self , rsthis: & QDataStream) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QDataStreamrsERt", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatastream.h:152
// index:3
// Public Visibility=Default Availability=Available
// [32] QDataStream & operator>>(qint32 &)

/*

*/
impl /*struct*/ QDataStream {
  pub fn operator_right_shift_3<RetType, T: QDataStream_operator_right_shift_3<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_right_shift_3(self);
    // return 1;
  }
}
pub trait QDataStream_operator_right_shift_3<RetType> {
  fn operator_right_shift_3(self , rsthis: & QDataStream) -> RetType;
}
impl<'a> /*trait*/ QDataStream_operator_right_shift_3<usize> for (usize) {
  fn operator_right_shift_3(self , rsthis: & QDataStream) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QDataStreamrsERi", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatastream.h:153
// index:4
// Public Visibility=Default Availability=Available
// [32] QDataStream & operator>>(quint32 &)

/*

*/
impl /*struct*/ QDataStream {
  pub fn operator_right_shift_4<RetType, T: QDataStream_operator_right_shift_4<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_right_shift_4(self);
    // return 1;
  }
}
pub trait QDataStream_operator_right_shift_4<RetType> {
  fn operator_right_shift_4(self , rsthis: & QDataStream) -> RetType;
}
impl<'a> /*trait*/ QDataStream_operator_right_shift_4<usize> for (usize) {
  fn operator_right_shift_4(self , rsthis: & QDataStream) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QDataStreamrsERj", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatastream.h:154
// index:5
// Public Visibility=Default Availability=Available
// [32] QDataStream & operator>>(qint64 &)

/*

*/
impl /*struct*/ QDataStream {
  pub fn operator_right_shift_5<RetType, T: QDataStream_operator_right_shift_5<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_right_shift_5(self);
    // return 1;
  }
}
pub trait QDataStream_operator_right_shift_5<RetType> {
  fn operator_right_shift_5(self , rsthis: & QDataStream) -> RetType;
}
impl<'a> /*trait*/ QDataStream_operator_right_shift_5<usize> for (usize) {
  fn operator_right_shift_5(self , rsthis: & QDataStream) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QDataStreamrsERx", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatastream.h:155
// index:6
// Public Visibility=Default Availability=Available
// [32] QDataStream & operator>>(quint64 &)

/*

*/
impl /*struct*/ QDataStream {
  pub fn operator_right_shift_6<RetType, T: QDataStream_operator_right_shift_6<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_right_shift_6(self);
    // return 1;
  }
}
pub trait QDataStream_operator_right_shift_6<RetType> {
  fn operator_right_shift_6(self , rsthis: & QDataStream) -> RetType;
}
impl<'a> /*trait*/ QDataStream_operator_right_shift_6<usize> for (usize) {
  fn operator_right_shift_6(self , rsthis: & QDataStream) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QDataStreamrsERy", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatastream.h:158
// index:7
// Public Visibility=Default Availability=Available
// [32] QDataStream & operator>>(bool &)

/*

*/
impl /*struct*/ QDataStream {
  pub fn operator_right_shift_7<RetType, T: QDataStream_operator_right_shift_7<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_right_shift_7(self);
    // return 1;
  }
}
pub trait QDataStream_operator_right_shift_7<RetType> {
  fn operator_right_shift_7(self , rsthis: & QDataStream) -> RetType;
}
impl<'a> /*trait*/ QDataStream_operator_right_shift_7<usize> for (usize) {
  fn operator_right_shift_7(self , rsthis: & QDataStream) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QDataStreamrsERb", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatastream.h:160
// index:8
// Public Visibility=Default Availability=Available
// [32] QDataStream & operator>>(float &)

/*

*/
impl /*struct*/ QDataStream {
  pub fn operator_right_shift_8<RetType, T: QDataStream_operator_right_shift_8<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_right_shift_8(self);
    // return 1;
  }
}
pub trait QDataStream_operator_right_shift_8<RetType> {
  fn operator_right_shift_8(self , rsthis: & QDataStream) -> RetType;
}
impl<'a> /*trait*/ QDataStream_operator_right_shift_8<usize> for (usize) {
  fn operator_right_shift_8(self , rsthis: & QDataStream) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QDataStreamrsERf", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatastream.h:161
// index:9
// Public Visibility=Default Availability=Available
// [32] QDataStream & operator>>(double &)

/*

*/
impl /*struct*/ QDataStream {
  pub fn operator_right_shift_9<RetType, T: QDataStream_operator_right_shift_9<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_right_shift_9(self);
    // return 1;
  }
}
pub trait QDataStream_operator_right_shift_9<RetType> {
  fn operator_right_shift_9(self , rsthis: & QDataStream) -> RetType;
}
impl<'a> /*trait*/ QDataStream_operator_right_shift_9<usize> for (usize) {
  fn operator_right_shift_9(self , rsthis: & QDataStream) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QDataStreamrsERd", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatastream.h:162
// index:10
// Public Visibility=Default Availability=Available
// [32] QDataStream & operator>>(char *&)

/*

*/
impl /*struct*/ QDataStream {
  pub fn operator_right_shift_10<RetType, T: QDataStream_operator_right_shift_10<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_right_shift_10(self);
    // return 1;
  }
}
pub trait QDataStream_operator_right_shift_10<RetType> {
  fn operator_right_shift_10(self , rsthis: & QDataStream) -> RetType;
}
impl<'a> /*trait*/ QDataStream_operator_right_shift_10<usize> for (usize) {
  fn operator_right_shift_10(self , rsthis: & QDataStream) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QDataStreamrsERPc", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatastream.h:165
// index:0
// Public Visibility=Default Availability=Available
// [32] QDataStream & operator<<(quint8)

/*

*/
impl /*struct*/ QDataStream {
  pub fn operator_left_shift_0<RetType, T: QDataStream_operator_left_shift_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_left_shift_0(self);
    // return 1;
  }
}
pub trait QDataStream_operator_left_shift_0<RetType> {
  fn operator_left_shift_0(self , rsthis: & QDataStream) -> RetType;
}
impl<'a> /*trait*/ QDataStream_operator_left_shift_0<usize> for (u8) {
  fn operator_left_shift_0(self , rsthis: & QDataStream) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const u8 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QDataStreamlsEh", 1,qtrt::FFITY_UINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatastream.h:166
// index:1
// Public Visibility=Default Availability=Available
// [32] QDataStream & operator<<(qint16)

/*

*/
impl /*struct*/ QDataStream {
  pub fn operator_left_shift_1<RetType, T: QDataStream_operator_left_shift_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_left_shift_1(self);
    // return 1;
  }
}
pub trait QDataStream_operator_left_shift_1<RetType> {
  fn operator_left_shift_1(self , rsthis: & QDataStream) -> RetType;
}
impl<'a> /*trait*/ QDataStream_operator_left_shift_1<usize> for (i16) {
  fn operator_left_shift_1(self , rsthis: & QDataStream) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i16 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QDataStreamlsEs", 1,qtrt::FFITY_SINT16,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatastream.h:167
// index:2
// Public Visibility=Default Availability=Available
// [32] QDataStream & operator<<(quint16)

/*

*/
impl /*struct*/ QDataStream {
  pub fn operator_left_shift_2<RetType, T: QDataStream_operator_left_shift_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_left_shift_2(self);
    // return 1;
  }
}
pub trait QDataStream_operator_left_shift_2<RetType> {
  fn operator_left_shift_2(self , rsthis: & QDataStream) -> RetType;
}
impl<'a> /*trait*/ QDataStream_operator_left_shift_2<usize> for (u16) {
  fn operator_left_shift_2(self , rsthis: & QDataStream) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const u16 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QDataStreamlsEt", 1,qtrt::FFITY_UINT16,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatastream.h:168
// index:3
// Public Visibility=Default Availability=Available
// [32] QDataStream & operator<<(qint32)

/*

*/
impl /*struct*/ QDataStream {
  pub fn operator_left_shift_3<RetType, T: QDataStream_operator_left_shift_3<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_left_shift_3(self);
    // return 1;
  }
}
pub trait QDataStream_operator_left_shift_3<RetType> {
  fn operator_left_shift_3(self , rsthis: & QDataStream) -> RetType;
}
impl<'a> /*trait*/ QDataStream_operator_left_shift_3<usize> for (i32) {
  fn operator_left_shift_3(self , rsthis: & QDataStream) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QDataStreamlsEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatastream.h:169
// index:4
// Public Visibility=Default Availability=Available
// [32] QDataStream & operator<<(quint32)

/*

*/
impl /*struct*/ QDataStream {
  pub fn operator_left_shift_4<RetType, T: QDataStream_operator_left_shift_4<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_left_shift_4(self);
    // return 1;
  }
}
pub trait QDataStream_operator_left_shift_4<RetType> {
  fn operator_left_shift_4(self , rsthis: & QDataStream) -> RetType;
}
impl<'a> /*trait*/ QDataStream_operator_left_shift_4<usize> for (u32) {
  fn operator_left_shift_4(self , rsthis: & QDataStream) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const u32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QDataStreamlsEj", 1,qtrt::FFITY_UINT32,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatastream.h:170
// index:5
// Public Visibility=Default Availability=Available
// [32] QDataStream & operator<<(qint64)

/*

*/
impl /*struct*/ QDataStream {
  pub fn operator_left_shift_5<RetType, T: QDataStream_operator_left_shift_5<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_left_shift_5(self);
    // return 1;
  }
}
pub trait QDataStream_operator_left_shift_5<RetType> {
  fn operator_left_shift_5(self , rsthis: & QDataStream) -> RetType;
}
impl<'a> /*trait*/ QDataStream_operator_left_shift_5<usize> for (i64) {
  fn operator_left_shift_5(self , rsthis: & QDataStream) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QDataStreamlsEx", 1,qtrt::FFITY_SINT64,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatastream.h:171
// index:6
// Public Visibility=Default Availability=Available
// [32] QDataStream & operator<<(quint64)

/*

*/
impl /*struct*/ QDataStream {
  pub fn operator_left_shift_6<RetType, T: QDataStream_operator_left_shift_6<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_left_shift_6(self);
    // return 1;
  }
}
pub trait QDataStream_operator_left_shift_6<RetType> {
  fn operator_left_shift_6(self , rsthis: & QDataStream) -> RetType;
}
impl<'a> /*trait*/ QDataStream_operator_left_shift_6<usize> for (u64) {
  fn operator_left_shift_6(self , rsthis: & QDataStream) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const u64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QDataStreamlsEy", 1,qtrt::FFITY_UINT64,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatastream.h:173
// index:7
// Public Visibility=Default Availability=Available
// [32] QDataStream & operator<<(bool)

/*

*/
impl /*struct*/ QDataStream {
  pub fn operator_left_shift_7<RetType, T: QDataStream_operator_left_shift_7<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_left_shift_7(self);
    // return 1;
  }
}
pub trait QDataStream_operator_left_shift_7<RetType> {
  fn operator_left_shift_7(self , rsthis: & QDataStream) -> RetType;
}
impl<'a> /*trait*/ QDataStream_operator_left_shift_7<usize> for (bool) {
  fn operator_left_shift_7(self , rsthis: & QDataStream) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QDataStreamlsEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatastream.h:175
// index:8
// Public Visibility=Default Availability=Available
// [32] QDataStream & operator<<(float)

/*

*/
impl /*struct*/ QDataStream {
  pub fn operator_left_shift_8<RetType, T: QDataStream_operator_left_shift_8<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_left_shift_8(self);
    // return 1;
  }
}
pub trait QDataStream_operator_left_shift_8<RetType> {
  fn operator_left_shift_8(self , rsthis: & QDataStream) -> RetType;
}
impl<'a> /*trait*/ QDataStream_operator_left_shift_8<usize> for (f32) {
  fn operator_left_shift_8(self , rsthis: & QDataStream) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QDataStreamlsEf", 1,qtrt::FFITY_FLOAT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatastream.h:176
// index:9
// Public Visibility=Default Availability=Available
// [32] QDataStream & operator<<(double)

/*

*/
impl /*struct*/ QDataStream {
  pub fn operator_left_shift_9<RetType, T: QDataStream_operator_left_shift_9<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_left_shift_9(self);
    // return 1;
  }
}
pub trait QDataStream_operator_left_shift_9<RetType> {
  fn operator_left_shift_9(self , rsthis: & QDataStream) -> RetType;
}
impl<'a> /*trait*/ QDataStream_operator_left_shift_9<usize> for (f64) {
  fn operator_left_shift_9(self , rsthis: & QDataStream) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QDataStreamlsEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatastream.h:177
// index:10
// Public Visibility=Default Availability=Available
// [32] QDataStream & operator<<(const char *)

/*

*/
impl /*struct*/ QDataStream {
  pub fn operator_left_shift_10<RetType, T: QDataStream_operator_left_shift_10<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_left_shift_10(self);
    // return 1;
  }
}
pub trait QDataStream_operator_left_shift_10<RetType> {
  fn operator_left_shift_10(self , rsthis: & QDataStream) -> RetType;
}
impl<'a> /*trait*/ QDataStream_operator_left_shift_10<usize> for (usize) {
  fn operator_left_shift_10(self , rsthis: & QDataStream) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QDataStreamlsEPKc", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatastream.h:179
// index:0
// Public Visibility=Default Availability=Available
// [32] QDataStream & readBytes(char *&, uint &)

/*
Reads the buffer s from the stream and returns a reference to the stream.

The buffer s is allocated using new []. Destroy it with the delete [] operator.

The l parameter is set to the length of the buffer. If the string read is empty, l is set to 0 and s is set to a null pointer.

The serialization format is a quint32 length specifier first, then l bytes of data.

See also readRawData() and writeBytes().
*/
impl /*struct*/ QDataStream {
  pub fn readBytes_0<RetType, T: QDataStream_readBytes_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.readBytes_0(self);
    // return 1;
  }
}
pub trait QDataStream_readBytes_0<RetType> {
  fn readBytes_0(self , rsthis: & QDataStream) -> RetType;
}
impl<'a> /*trait*/ QDataStream_readBytes_0<usize> for (usize,usize) {
  fn readBytes_0(self , rsthis: & QDataStream) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QDataStream9readBytesERPcRj", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatastream.h:180
// index:0
// Public Visibility=Default Availability=Available
// [4] int readRawData(char *, int)

/*
Reads at most len bytes from the stream into s and returns the number of bytes read. If an error occurs, this function returns -1.

The buffer s must be preallocated. The data is not encoded.

See also readBytes(), QIODevice::read(), and writeRawData().
*/
impl /*struct*/ QDataStream {
  pub fn readRawData_0<RetType, T: QDataStream_readRawData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.readRawData_0(self);
    // return 1;
  }
}
pub trait QDataStream_readRawData_0<RetType> {
  fn readRawData_0(self , rsthis: & QDataStream) -> RetType;
}
impl<'a> /*trait*/ QDataStream_readRawData_0<i32> for (usize,i32) {
  fn readRawData_0(self , rsthis: & QDataStream) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QDataStream11readRawDataEPci", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatastream.h:182
// index:0
// Public Visibility=Default Availability=Available
// [32] QDataStream & writeBytes(const char *, uint)

/*
Writes the length specifier len and the buffer s to the stream and returns a reference to the stream.

The len is serialized as a quint32, followed by len bytes from s. Note that the data is not encoded.

See also writeRawData() and readBytes().
*/
impl /*struct*/ QDataStream {
  pub fn writeBytes_0<RetType, T: QDataStream_writeBytes_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.writeBytes_0(self);
    // return 1;
  }
}
pub trait QDataStream_writeBytes_0<RetType> {
  fn writeBytes_0(self , rsthis: & QDataStream) -> RetType;
}
impl<'a> /*trait*/ QDataStream_writeBytes_0<usize> for (usize,u32) {
  fn writeBytes_0(self , rsthis: & QDataStream) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const u32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QDataStream10writeBytesEPKcj", 2,qtrt::FFITY_POINTER,qtrt::FFITY_UINT32,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatastream.h:183
// index:0
// Public Visibility=Default Availability=Available
// [4] int writeRawData(const char *, int)

/*
Writes len bytes from s to the stream. Returns the number of bytes actually written, or -1 on error. The data is not encoded.

See also writeBytes(), QIODevice::write(), and readRawData().
*/
impl /*struct*/ QDataStream {
  pub fn writeRawData_0<RetType, T: QDataStream_writeRawData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.writeRawData_0(self);
    // return 1;
  }
}
pub trait QDataStream_writeRawData_0<RetType> {
  fn writeRawData_0(self , rsthis: & QDataStream) -> RetType;
}
impl<'a> /*trait*/ QDataStream_writeRawData_0<i32> for (usize,i32) {
  fn writeRawData_0(self , rsthis: & QDataStream) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QDataStream12writeRawDataEPKci", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatastream.h:185
// index:0
// Public Visibility=Default Availability=Available
// [4] int skipRawData(int)

/*
Skips len bytes from the device. Returns the number of bytes actually skipped, or -1 on error.

This is equivalent to calling readRawData() on a buffer of length len and ignoring the buffer.

This function was introduced in  Qt 4.1.

See also QIODevice::seek().
*/
impl /*struct*/ QDataStream {
  pub fn skipRawData_0<RetType, T: QDataStream_skipRawData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.skipRawData_0(self);
    // return 1;
  }
}
pub trait QDataStream_skipRawData_0<RetType> {
  fn skipRawData_0(self , rsthis: & QDataStream) -> RetType;
}
impl<'a> /*trait*/ QDataStream_skipRawData_0<i32> for (i32) {
  fn skipRawData_0(self , rsthis: & QDataStream) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QDataStream11skipRawDataEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatastream.h:187
// index:0
// Public Visibility=Default Availability=Available
// [-2] void startTransaction()

/*
Starts a new read transaction on the stream.

Defines a restorable point within the sequence of read operations. For sequential devices, read data will be duplicated internally to allow recovery in case of incomplete reads. For random-access devices, this function saves the current position of the stream. Call commitTransaction(), rollbackTransaction(), or abortTransaction() to finish the current transaction.

Once a transaction is started, subsequent calls to this function will make the transaction recursive. Inner transactions act as agents of the outermost transaction (i.e., report the status of read operations to the outermost transaction, which can restore the position of the stream).

Note: Restoring to the point of the nested startTransaction() call is not supported.

When an error occurs during a transaction (including an inner transaction failing), reading from the data stream is suspended (all subsequent read operations return empty/zero values) and subsequent inner transactions are forced to fail. Starting a new outermost transaction recovers from this state. This behavior makes it unnecessary to error-check every read operation separately.

This function was introduced in  Qt 5.7.

See also commitTransaction(), rollbackTransaction(), and abortTransaction().
*/
impl /*struct*/ QDataStream {
  pub fn startTransaction_0<RetType, T: QDataStream_startTransaction_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.startTransaction_0(self);
    // return 1;
  }
}
pub trait QDataStream_startTransaction_0<RetType> {
  fn startTransaction_0(self , rsthis: & QDataStream) -> RetType;
}
impl<'a> /*trait*/ QDataStream_startTransaction_0<(/*void*/)> for () {
  fn startTransaction_0(self , rsthis: & QDataStream) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN11QDataStream16startTransactionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qdatastream.h:188
// index:0
// Public Visibility=Default Availability=Available
// [1] bool commitTransaction()

/*
Completes a read transaction. Returns true if no read errors have occurred during the transaction; otherwise returns false.

If called on an inner transaction, committing will be postponed until the outermost commitTransaction(), rollbackTransaction(), or abortTransaction() call occurs.

Otherwise, if the stream status indicates reading past the end of the data, this function restores the stream data to the point of the startTransaction() call. When this situation occurs, you need to wait for more data to arrive, after which you start a new transaction. If the data stream has read corrupt data or any of the inner transactions was aborted, this function aborts the transaction.

This function was introduced in  Qt 5.7.

See also startTransaction(), rollbackTransaction(), and abortTransaction().
*/
impl /*struct*/ QDataStream {
  pub fn commitTransaction_0<RetType, T: QDataStream_commitTransaction_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.commitTransaction_0(self);
    // return 1;
  }
}
pub trait QDataStream_commitTransaction_0<RetType> {
  fn commitTransaction_0(self , rsthis: & QDataStream) -> RetType;
}
impl<'a> /*trait*/ QDataStream_commitTransaction_0<bool> for () {
  fn commitTransaction_0(self , rsthis: & QDataStream) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QDataStream17commitTransactionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdatastream.h:189
// index:0
// Public Visibility=Default Availability=Available
// [-2] void rollbackTransaction()

/*
Reverts a read transaction.

This function is commonly used to rollback the transaction when an incomplete read was detected prior to committing the transaction.

If called on an inner transaction, reverting is delegated to the outermost transaction, and subsequently started inner transactions are forced to fail.

For the outermost transaction, restores the stream data to the point of the startTransaction() call. If the data stream has read corrupt data or any of the inner transactions was aborted, this function aborts the transaction.

If the preceding stream operations were successful, sets the status of the data stream to

ConstantDescription
ReadPastEnd.


This function was introduced in  Qt 5.7.

See also startTransaction(), commitTransaction(), and abortTransaction().
*/
impl /*struct*/ QDataStream {
  pub fn rollbackTransaction_0<RetType, T: QDataStream_rollbackTransaction_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rollbackTransaction_0(self);
    // return 1;
  }
}
pub trait QDataStream_rollbackTransaction_0<RetType> {
  fn rollbackTransaction_0(self , rsthis: & QDataStream) -> RetType;
}
impl<'a> /*trait*/ QDataStream_rollbackTransaction_0<(/*void*/)> for () {
  fn rollbackTransaction_0(self , rsthis: & QDataStream) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN11QDataStream19rollbackTransactionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qdatastream.h:190
// index:0
// Public Visibility=Default Availability=Available
// [-2] void abortTransaction()

/*
Aborts a read transaction.

This function is commonly used to discard the transaction after higher-level protocol errors or loss of stream synchronization.

If called on an inner transaction, aborting is delegated to the outermost transaction, and subsequently started inner transactions are forced to fail.

For the outermost transaction, discards the restoration point and any internally duplicated data of the stream. Will not affect the current read position of the stream.

Sets the status of the data stream to

ConstantDescription
ReadCorruptData.


This function was introduced in  Qt 5.7.

See also startTransaction(), commitTransaction(), and rollbackTransaction().
*/
impl /*struct*/ QDataStream {
  pub fn abortTransaction_0<RetType, T: QDataStream_abortTransaction_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.abortTransaction_0(self);
    // return 1;
  }
}
pub trait QDataStream_abortTransaction_0<RetType> {
  fn abortTransaction_0(self , rsthis: & QDataStream) -> RetType;
}
impl<'a> /*trait*/ QDataStream_abortTransaction_0<(/*void*/)> for () {
  fn abortTransaction_0(self , rsthis: & QDataStream) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN11QDataStream16abortTransactionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}


/*
This enum provides symbolic synonyms for the data serialization format version numbers.



See also setVersion() and version().

*/
pub type QDataStream__Version = i32;
// 
pub const QDataStream__Qt_1_0 :QDataStream__Version = 1;
// 
pub const QDataStream__Qt_2_0 :QDataStream__Version = 2;
// 
pub const QDataStream__Qt_2_1 :QDataStream__Version = 3;
// 
pub const QDataStream__Qt_3_0 :QDataStream__Version = 4;
// 
pub const QDataStream__Qt_3_1 :QDataStream__Version = 5;
// 
pub const QDataStream__Qt_3_3 :QDataStream__Version = 6;
// 
pub const QDataStream__Qt_4_0 :QDataStream__Version = 7;
// 
pub const QDataStream__Qt_4_1 :QDataStream__Version = 7;
// 
pub const QDataStream__Qt_4_2 :QDataStream__Version = 8;
// 
pub const QDataStream__Qt_4_3 :QDataStream__Version = 9;
// 
pub const QDataStream__Qt_4_4 :QDataStream__Version = 10;
// 
pub const QDataStream__Qt_4_5 :QDataStream__Version = 11;
// 
pub const QDataStream__Qt_4_6 :QDataStream__Version = 12;
// 
pub const QDataStream__Qt_4_7 :QDataStream__Version = 12;
// 
pub const QDataStream__Qt_4_8 :QDataStream__Version = 12;
// 
pub const QDataStream__Qt_4_9 :QDataStream__Version = 12;
// 
pub const QDataStream__Qt_5_0 :QDataStream__Version = 13;
// 
pub const QDataStream__Qt_5_1 :QDataStream__Version = 14;
// 
pub const QDataStream__Qt_5_2 :QDataStream__Version = 15;
// 
pub const QDataStream__Qt_5_3 :QDataStream__Version = 15;
// 
pub const QDataStream__Qt_5_4 :QDataStream__Version = 16;
// 
pub const QDataStream__Qt_5_5 :QDataStream__Version = 16;
// 
pub const QDataStream__Qt_5_6 :QDataStream__Version = 17;
// 
pub const QDataStream__Qt_5_7 :QDataStream__Version = 17;
// 
pub const QDataStream__Qt_5_8 :QDataStream__Version = 17;
// 
pub const QDataStream__Qt_5_9 :QDataStream__Version = 17;
// 
pub const QDataStream__Qt_5_10 :QDataStream__Version = 17;
// 
pub const QDataStream__Qt_DefaultCompiledVersion :QDataStream__Version = 17;
pub fn QDataStream_VersionItemName(val: i32) ->String {
  match val {
     QDataStream__Qt_1_0 => // 1
     {return String::from("Qt_1_0");}
     QDataStream__Qt_2_0 => // 2
     {return String::from("Qt_2_0");}
     QDataStream__Qt_2_1 => // 3
     {return String::from("Qt_2_1");}
     QDataStream__Qt_3_0 => // 4
     {return String::from("Qt_3_0");}
     QDataStream__Qt_3_1 => // 5
     {return String::from("Qt_3_1");}
     QDataStream__Qt_3_3 => // 6
     {return String::from("Qt_3_3");}
     QDataStream__Qt_4_0 => // 7
     {return String::from("Qt_4_0,Qt_4_1");}
    // QDataStream__Qt_4_1 => // 7
    // {return String::from("");}
     QDataStream__Qt_4_2 => // 8
     {return String::from("Qt_4_2");}
     QDataStream__Qt_4_3 => // 9
     {return String::from("Qt_4_3");}
     QDataStream__Qt_4_4 => // 10
     {return String::from("Qt_4_4");}
     QDataStream__Qt_4_5 => // 11
     {return String::from("Qt_4_5");}
     QDataStream__Qt_4_6 => // 12
     {return String::from("Qt_4_6,Qt_4_7,Qt_4_8,Qt_4_9");}
    // QDataStream__Qt_4_7 => // 12
    // {return String::from("");}
    // QDataStream__Qt_4_8 => // 12
    // {return String::from("");}
    // QDataStream__Qt_4_9 => // 12
    // {return String::from("");}
     QDataStream__Qt_5_0 => // 13
     {return String::from("Qt_5_0");}
     QDataStream__Qt_5_1 => // 14
     {return String::from("Qt_5_1");}
     QDataStream__Qt_5_2 => // 15
     {return String::from("Qt_5_2,Qt_5_3");}
    // QDataStream__Qt_5_3 => // 15
    // {return String::from("");}
     QDataStream__Qt_5_4 => // 16
     {return String::from("Qt_5_4,Qt_5_5");}
    // QDataStream__Qt_5_5 => // 16
    // {return String::from("");}
     QDataStream__Qt_5_6 => // 17
     {return String::from("Qt_5_6,Qt_5_7,Qt_5_8,Qt_5_9,Qt_5_10,Qt_DefaultCompiledVersion");}
    // QDataStream__Qt_5_7 => // 17
    // {return String::from("");}
    // QDataStream__Qt_5_8 => // 17
    // {return String::from("");}
    // QDataStream__Qt_5_9 => // 17
    // {return String::from("");}
    // QDataStream__Qt_5_10 => // 17
    // {return String::from("");}
    // QDataStream__Qt_DefaultCompiledVersion => // 17
    // {return String::from("");}
  _ => {return format!("{}", val);}
}
}
pub fn QDataStream_VersionItemName_s(val: i32) ->String {
  //var nilthis *QDataStream
  //return nilthis.VersionItemName(val);
  return QDataStream_VersionItemName(val);
}


/*
The byte order used for reading/writing the data.

QDataStream::BigEndianQSysInfo::BigEndianMost significant byte first (the default)
QDataStream::LittleEndianQSysInfo::LittleEndianLeast significant byte first

*/
pub type QDataStream__ByteOrder = i32;
// 
pub const QDataStream__BigEndian :QDataStream__ByteOrder = 0;
// 
pub const QDataStream__LittleEndian :QDataStream__ByteOrder = 1;
pub fn QDataStream_ByteOrderItemName(val: i32) ->String {
  match val {
     QDataStream__BigEndian => // 0
     {return String::from("BigEndian");}
     QDataStream__LittleEndian => // 1
     {return String::from("LittleEndian");}
  _ => {return format!("{}", val);}
}
}
pub fn QDataStream_ByteOrderItemName_s(val: i32) ->String {
  //var nilthis *QDataStream
  //return nilthis.ByteOrderItemName(val);
  return QDataStream_ByteOrderItemName(val);
}


/*
This enum describes the current status of the data stream.


*/
pub type QDataStream__Status = i32;
// The data stream is operating normally.
pub const QDataStream__Ok :QDataStream__Status = 0;
// The data stream has read past the end of the data in the underlying device.
pub const QDataStream__ReadPastEnd :QDataStream__Status = 1;
// The data stream has read corrupt data.
pub const QDataStream__ReadCorruptData :QDataStream__Status = 2;
// The data stream cannot write to the underlying device.
pub const QDataStream__WriteFailed :QDataStream__Status = 3;
pub fn QDataStream_StatusItemName(val: i32) ->String {
  match val {
     QDataStream__Ok => // 0
     {return String::from("Ok");}
     QDataStream__ReadPastEnd => // 1
     {return String::from("ReadPastEnd");}
     QDataStream__ReadCorruptData => // 2
     {return String::from("ReadCorruptData");}
     QDataStream__WriteFailed => // 3
     {return String::from("WriteFailed");}
  _ => {return format!("{}", val);}
}
}
pub fn QDataStream_StatusItemName_s(val: i32) ->String {
  //var nilthis *QDataStream
  //return nilthis.StatusItemName(val);
  return QDataStream_StatusItemName(val);
}


/*
The precision of floating point numbers used for reading/writing the data. This will only have an effect if the version of the data stream is Qt_4_6 or higher.

Warning: The floating point precision must be set to the same value on the object that writes and the object that reads the data stream.



See also setFloatingPointPrecision() and floatingPointPrecision().

*/
pub type QDataStream__FloatingPointPrecision = i32;
// 
pub const QDataStream__SinglePrecision :QDataStream__FloatingPointPrecision = 0;
// 
pub const QDataStream__DoublePrecision :QDataStream__FloatingPointPrecision = 1;
pub fn QDataStream_FloatingPointPrecisionItemName(val: i32) ->String {
  match val {
     QDataStream__SinglePrecision => // 0
     {return String::from("SinglePrecision");}
     QDataStream__DoublePrecision => // 1
     {return String::from("DoublePrecision");}
  _ => {return format!("{}", val);}
}
}
pub fn QDataStream_FloatingPointPrecisionItemName_s(val: i32) ->String {
  //var nilthis *QDataStream
  //return nilthis.FloatingPointPrecisionItemName(val);
  return QDataStream_FloatingPointPrecisionItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
