

// mod ::core::QIODevice
// package qtcore
// /usr/include/qt/QtCore/qiodevice.h
// #include <qiodevice.h>
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

// long long readData(char *, qint64)
// func (this *QIODevice) InheritReadData(f func(data string, maxlen int64) int64) {
//  qtrt.SetAllInheritCallback(this, "readData", f)
// }

// long long readLineData(char *, qint64)
// func (this *QIODevice) InheritReadLineData(f func(data string, maxlen int64) int64) {
//  qtrt.SetAllInheritCallback(this, "readLineData", f)
// }

// long long writeData(const char *, qint64)
// func (this *QIODevice) InheritWriteData(f func(data string, len_ int64) int64) {
//  qtrt.SetAllInheritCallback(this, "writeData", f)
// }

// void setOpenMode(QIODevice::OpenMode)
// func (this *QIODevice) InheritSetOpenMode(f func(openMode int) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "setOpenMode", f)
// }

// void setErrorString(const QString &)
// func (this *QIODevice) InheritSetErrorString(f func(errorString string) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "setErrorString", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QIODevice)=16
pub struct QIODevice {
  qbase: QObject,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QIODevice_ITF interface {
//    QObject_ITF
//    QIODevice_PTR() *QIODevice
//}
//func (ptr *QIODevice) QIODevice_PTR() *QIODevice { return ptr }

impl /*struct*/ QIODevice {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QIODevice {
    return QIODevice{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QIODevice {
//  type Target = QIODeviceBASE;
//
//  fn deref(&self) -> &QIODeviceBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QIODeviceBASE> for QIODevice {
//  fn as_ref(& self) -> & QIODeviceBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qiodevice.h:68
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QIODevice {
  pub fn metaObject_0<RetType, T: QIODevice_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QIODevice_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QIODevice) -> RetType;
}
impl<'a> /*trait*/ QIODevice_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QIODevice) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QIODevice10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qiodevice.h:83
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QIODevice()

/*
Constructs a QIODevice object.
*/
// QIODevice() ctx.fn_proto_cpp
impl /*struct*/ QIODevice {
  pub fn QIODevice_0<T: QIODevice_QIODevice_0>(value: T) -> QIODevice {
    let rsthis = value.QIODevice_0();
    return rsthis;
    // return 1;
  }
}

pub trait QIODevice_QIODevice_0 {
  fn QIODevice_0(self) -> QIODevice;
}
// QIODevice() ctx.fn_proto_cpp
impl<'a> /*trait*/ QIODevice_QIODevice_0 for () {
  fn QIODevice_0(self) -> QIODevice {
    // unsafe{_ZN9QIODeviceC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QIODeviceC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QIODevice{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qiodevice.h:85
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QIODevice(QObject *)

/*
Constructs a QIODevice object.
*/
// QIODevice(QObject *) ctx.fn_proto_cpp
impl /*struct*/ QIODevice {
  pub fn QIODevice_1<T: QIODevice_QIODevice_1>(value: T) -> QIODevice {
    let rsthis = value.QIODevice_1();
    return rsthis;
    // return 1;
  }
}

pub trait QIODevice_QIODevice_1 {
  fn QIODevice_1(self) -> QIODevice;
}
// QIODevice(QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QIODevice_QIODevice_1 for (usize) {
  fn QIODevice_1(self) -> QIODevice {
    // unsafe{_ZN9QIODeviceC2EP7QObject()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QIODeviceC2EP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QIODevice{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qiodevice.h:87
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QIODevice()

/*

*/
pub fn DeleteQIODevice(this :*mut QIODevice) {
    // let rv = qtrt::InvokeQtFunc6("_ZN9QIODeviceD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qiodevice.h:89
// index:0
// Public Visibility=Default Availability=Available
// [4] QIODevice::OpenMode openMode() const

/*
Returns the mode in which the device has been opened; i.e. ReadOnly or WriteOnly.

See also setOpenMode() and OpenMode.
*/
impl /*struct*/ QIODevice {
  pub fn openMode_0<RetType, T: QIODevice_openMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.openMode_0(self);
    // return 1;
  }
}
pub trait QIODevice_openMode_0<RetType> {
  fn openMode_0(self , rsthis: & QIODevice) -> RetType;
}
impl<'a> /*trait*/ QIODevice_openMode_0<i32> for () {
  fn openMode_0(self , rsthis: & QIODevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QIODevice8openModeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qiodevice.h:91
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTextModeEnabled(bool)

/*
If enabled is true, this function sets the Text flag on the device; otherwise the Text flag is removed. This feature is useful for classes that provide custom end-of-line handling on a QIODevice.

The IO device should be opened before calling this function.

See also isTextModeEnabled(), open(), and setOpenMode().
*/
impl /*struct*/ QIODevice {
  pub fn setTextModeEnabled_0<RetType, T: QIODevice_setTextModeEnabled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTextModeEnabled_0(self);
    // return 1;
  }
}
pub trait QIODevice_setTextModeEnabled_0<RetType> {
  fn setTextModeEnabled_0(self , rsthis: & QIODevice) -> RetType;
}
impl<'a> /*trait*/ QIODevice_setTextModeEnabled_0<(/*void*/)> for (bool) {
  fn setTextModeEnabled_0(self , rsthis: & QIODevice) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN9QIODevice18setTextModeEnabledEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qiodevice.h:92
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isTextModeEnabled() const

/*
Returns true if the Text flag is enabled; otherwise returns false.

See also setTextModeEnabled().
*/
impl /*struct*/ QIODevice {
  pub fn isTextModeEnabled_0<RetType, T: QIODevice_isTextModeEnabled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isTextModeEnabled_0(self);
    // return 1;
  }
}
pub trait QIODevice_isTextModeEnabled_0<RetType> {
  fn isTextModeEnabled_0(self , rsthis: & QIODevice) -> RetType;
}
impl<'a> /*trait*/ QIODevice_isTextModeEnabled_0<bool> for () {
  fn isTextModeEnabled_0(self , rsthis: & QIODevice) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QIODevice17isTextModeEnabledEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qiodevice.h:94
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isOpen() const

/*
Returns true if the device is open; otherwise returns false. A device is open if it can be read from and/or written to. By default, this function returns false if openMode() returns NotOpen.

See also openMode() and OpenMode.
*/
impl /*struct*/ QIODevice {
  pub fn isOpen_0<RetType, T: QIODevice_isOpen_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isOpen_0(self);
    // return 1;
  }
}
pub trait QIODevice_isOpen_0<RetType> {
  fn isOpen_0(self , rsthis: & QIODevice) -> RetType;
}
impl<'a> /*trait*/ QIODevice_isOpen_0<bool> for () {
  fn isOpen_0(self , rsthis: & QIODevice) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QIODevice6isOpenEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qiodevice.h:95
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isReadable() const

/*
Returns true if data can be read from the device; otherwise returns false. Use bytesAvailable() to determine how many bytes can be read.

This is a convenience function which checks if the OpenMode of the device contains the ReadOnly flag.

See also openMode() and OpenMode.
*/
impl /*struct*/ QIODevice {
  pub fn isReadable_0<RetType, T: QIODevice_isReadable_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isReadable_0(self);
    // return 1;
  }
}
pub trait QIODevice_isReadable_0<RetType> {
  fn isReadable_0(self , rsthis: & QIODevice) -> RetType;
}
impl<'a> /*trait*/ QIODevice_isReadable_0<bool> for () {
  fn isReadable_0(self , rsthis: & QIODevice) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QIODevice10isReadableEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qiodevice.h:96
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isWritable() const

/*
Returns true if data can be written to the device; otherwise returns false.

This is a convenience function which checks if the OpenMode of the device contains the WriteOnly flag.

See also openMode() and OpenMode.
*/
impl /*struct*/ QIODevice {
  pub fn isWritable_0<RetType, T: QIODevice_isWritable_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isWritable_0(self);
    // return 1;
  }
}
pub trait QIODevice_isWritable_0<RetType> {
  fn isWritable_0(self , rsthis: & QIODevice) -> RetType;
}
impl<'a> /*trait*/ QIODevice_isWritable_0<bool> for () {
  fn isWritable_0(self , rsthis: & QIODevice) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QIODevice10isWritableEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qiodevice.h:97
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool isSequential() const

/*
Returns true if this device is sequential; otherwise returns false.

Sequential devices, as opposed to a random-access devices, have no concept of a start, an end, a size, or a current position, and they do not support seeking. You can only read from the device when it reports that data is available. The most common example of a sequential device is a network socket. On Unix, special files such as /dev/zero and fifo pipes are sequential.

Regular files, on the other hand, do support random access. They have both a size and a current position, and they also support seeking backwards and forwards in the data stream. Regular files are non-sequential.

See also bytesAvailable().
*/
impl /*struct*/ QIODevice {
  pub fn isSequential_0<RetType, T: QIODevice_isSequential_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isSequential_0(self);
    // return 1;
  }
}
pub trait QIODevice_isSequential_0<RetType> {
  fn isSequential_0(self , rsthis: & QIODevice) -> RetType;
}
impl<'a> /*trait*/ QIODevice_isSequential_0<bool> for () {
  fn isSequential_0(self , rsthis: & QIODevice) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QIODevice12isSequentialEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qiodevice.h:99
// index:0
// Public Visibility=Default Availability=Available
// [4] int readChannelCount() const

/*
Returns the number of available read channels if the device is open; otherwise returns 0.

This function was introduced in  Qt 5.7.

See also writeChannelCount() and QProcess.
*/
impl /*struct*/ QIODevice {
  pub fn readChannelCount_0<RetType, T: QIODevice_readChannelCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.readChannelCount_0(self);
    // return 1;
  }
}
pub trait QIODevice_readChannelCount_0<RetType> {
  fn readChannelCount_0(self , rsthis: & QIODevice) -> RetType;
}
impl<'a> /*trait*/ QIODevice_readChannelCount_0<i32> for () {
  fn readChannelCount_0(self , rsthis: & QIODevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QIODevice16readChannelCountEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qiodevice.h:100
// index:0
// Public Visibility=Default Availability=Available
// [4] int writeChannelCount() const

/*
Returns the number of available write channels if the device is open; otherwise returns 0.

This function was introduced in  Qt 5.7.

See also readChannelCount().
*/
impl /*struct*/ QIODevice {
  pub fn writeChannelCount_0<RetType, T: QIODevice_writeChannelCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.writeChannelCount_0(self);
    // return 1;
  }
}
pub trait QIODevice_writeChannelCount_0<RetType> {
  fn writeChannelCount_0(self , rsthis: & QIODevice) -> RetType;
}
impl<'a> /*trait*/ QIODevice_writeChannelCount_0<i32> for () {
  fn writeChannelCount_0(self , rsthis: & QIODevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QIODevice17writeChannelCountEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qiodevice.h:101
// index:0
// Public Visibility=Default Availability=Available
// [4] int currentReadChannel() const

/*
Returns the index of the current read channel.

This function was introduced in  Qt 5.7.

See also setCurrentReadChannel(), readChannelCount(), and QProcess.
*/
impl /*struct*/ QIODevice {
  pub fn currentReadChannel_0<RetType, T: QIODevice_currentReadChannel_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.currentReadChannel_0(self);
    // return 1;
  }
}
pub trait QIODevice_currentReadChannel_0<RetType> {
  fn currentReadChannel_0(self , rsthis: & QIODevice) -> RetType;
}
impl<'a> /*trait*/ QIODevice_currentReadChannel_0<i32> for () {
  fn currentReadChannel_0(self , rsthis: & QIODevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QIODevice18currentReadChannelEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qiodevice.h:102
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCurrentReadChannel(int)

/*
Sets the current read channel of the QIODevice to the given channel. The current input channel is used by the functions read(), readAll(), readLine(), and getChar(). It also determines which channel triggers QIODevice to emit readyRead().

This function was introduced in  Qt 5.7.

See also currentReadChannel(), readChannelCount(), and QProcess.
*/
impl /*struct*/ QIODevice {
  pub fn setCurrentReadChannel_0<RetType, T: QIODevice_setCurrentReadChannel_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCurrentReadChannel_0(self);
    // return 1;
  }
}
pub trait QIODevice_setCurrentReadChannel_0<RetType> {
  fn setCurrentReadChannel_0(self , rsthis: & QIODevice) -> RetType;
}
impl<'a> /*trait*/ QIODevice_setCurrentReadChannel_0<(/*void*/)> for (i32) {
  fn setCurrentReadChannel_0(self , rsthis: & QIODevice) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QIODevice21setCurrentReadChannelEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qiodevice.h:103
// index:0
// Public Visibility=Default Availability=Available
// [4] int currentWriteChannel() const

/*
Returns the the index of the current write channel.

This function was introduced in  Qt 5.7.

See also setCurrentWriteChannel() and writeChannelCount().
*/
impl /*struct*/ QIODevice {
  pub fn currentWriteChannel_0<RetType, T: QIODevice_currentWriteChannel_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.currentWriteChannel_0(self);
    // return 1;
  }
}
pub trait QIODevice_currentWriteChannel_0<RetType> {
  fn currentWriteChannel_0(self , rsthis: & QIODevice) -> RetType;
}
impl<'a> /*trait*/ QIODevice_currentWriteChannel_0<i32> for () {
  fn currentWriteChannel_0(self , rsthis: & QIODevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QIODevice19currentWriteChannelEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qiodevice.h:104
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCurrentWriteChannel(int)

/*
Sets the current write channel of the QIODevice to the given channel. The current output channel is used by the functions write(), putChar(). It also determines which channel triggers QIODevice to emit bytesWritten().

This function was introduced in  Qt 5.7.

See also currentWriteChannel() and writeChannelCount().
*/
impl /*struct*/ QIODevice {
  pub fn setCurrentWriteChannel_0<RetType, T: QIODevice_setCurrentWriteChannel_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCurrentWriteChannel_0(self);
    // return 1;
  }
}
pub trait QIODevice_setCurrentWriteChannel_0<RetType> {
  fn setCurrentWriteChannel_0(self , rsthis: & QIODevice) -> RetType;
}
impl<'a> /*trait*/ QIODevice_setCurrentWriteChannel_0<(/*void*/)> for (i32) {
  fn setCurrentWriteChannel_0(self , rsthis: & QIODevice) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QIODevice22setCurrentWriteChannelEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qiodevice.h:106
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool open(QIODevice::OpenMode)

/*
Opens the device and sets its OpenMode to mode. Returns true if successful; otherwise returns false. This function should be called from any reimplementations of open() or other functions that open the device.

See also openMode() and OpenMode.
*/
impl /*struct*/ QIODevice {
  pub fn open_0<RetType, T: QIODevice_open_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.open_0(self);
    // return 1;
  }
}
pub trait QIODevice_open_0<RetType> {
  fn open_0(self , rsthis: & QIODevice) -> RetType;
}
impl<'a> /*trait*/ QIODevice_open_0<bool> for (i32) {
  fn open_0(self , rsthis: & QIODevice) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QIODevice4openE6QFlagsINS_12OpenModeFlagEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qiodevice.h:107
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void close()

/*
First emits aboutToClose(), then closes the device and sets its OpenMode to NotOpen. The error string is also reset.

See also setOpenMode() and OpenMode.
*/
impl /*struct*/ QIODevice {
  pub fn close_0<RetType, T: QIODevice_close_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.close_0(self);
    // return 1;
  }
}
pub trait QIODevice_close_0<RetType> {
  fn close_0(self , rsthis: & QIODevice) -> RetType;
}
impl<'a> /*trait*/ QIODevice_close_0<(/*void*/)> for () {
  fn close_0(self , rsthis: & QIODevice) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN9QIODevice5closeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qiodevice.h:111
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] qint64 pos() const

/*
For random-access devices, this function returns the position that data is written to or read from. For sequential devices or closed devices, where there is no concept of a "current position", 0 is returned.

The current read/write position of the device is maintained internally by QIODevice, so reimplementing this function is not necessary. When subclassing QIODevice, use QIODevice::seek() to notify QIODevice about changes in the device position.

See also isSequential() and seek().
*/
impl /*struct*/ QIODevice {
  pub fn pos_0<RetType, T: QIODevice_pos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.pos_0(self);
    // return 1;
  }
}
pub trait QIODevice_pos_0<RetType> {
  fn pos_0(self , rsthis: & QIODevice) -> RetType;
}
impl<'a> /*trait*/ QIODevice_pos_0<i64> for () {
  fn pos_0(self , rsthis: & QIODevice) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QIODevice3posEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qiodevice.h:112
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] qint64 size() const

/*
For open random-access devices, this function returns the size of the device. For open sequential devices, bytesAvailable() is returned.

If the device is closed, the size returned will not reflect the actual size of the device.

See also isSequential() and pos().
*/
impl /*struct*/ QIODevice {
  pub fn size_0<RetType, T: QIODevice_size_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.size_0(self);
    // return 1;
  }
}
pub trait QIODevice_size_0<RetType> {
  fn size_0(self , rsthis: & QIODevice) -> RetType;
}
impl<'a> /*trait*/ QIODevice_size_0<i64> for () {
  fn size_0(self , rsthis: & QIODevice) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QIODevice4sizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qiodevice.h:113
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool seek(qint64)

/*
For random-access devices, this function sets the current position to pos, returning true on success, or false if an error occurred. For sequential devices, the default behavior is to produce a warning and return false.

When subclassing QIODevice, you must call QIODevice::seek() at the start of your function to ensure integrity with QIODevice's built-in buffer.

See also pos() and isSequential().
*/
impl /*struct*/ QIODevice {
  pub fn seek_0<RetType, T: QIODevice_seek_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.seek_0(self);
    // return 1;
  }
}
pub trait QIODevice_seek_0<RetType> {
  fn seek_0(self , rsthis: & QIODevice) -> RetType;
}
impl<'a> /*trait*/ QIODevice_seek_0<bool> for (i64) {
  fn seek_0(self , rsthis: & QIODevice) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QIODevice4seekEx", 1,qtrt::FFITY_SINT64,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qiodevice.h:114
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool atEnd() const

/*
Returns true if the current read and write position is at the end of the device (i.e. there is no more data available for reading on the device); otherwise returns false.

For some devices, atEnd() can return true even though there is more data to read. This special case only applies to devices that generate data in direct response to you calling read() (e.g., /dev or /proc files on Unix and macOS, or console input / stdin on all platforms).

See also bytesAvailable(), read(), and isSequential().
*/
impl /*struct*/ QIODevice {
  pub fn atEnd_0<RetType, T: QIODevice_atEnd_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.atEnd_0(self);
    // return 1;
  }
}
pub trait QIODevice_atEnd_0<RetType> {
  fn atEnd_0(self , rsthis: & QIODevice) -> RetType;
}
impl<'a> /*trait*/ QIODevice_atEnd_0<bool> for () {
  fn atEnd_0(self , rsthis: & QIODevice) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QIODevice5atEndEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qiodevice.h:115
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool reset()

/*
Seeks to the start of input for random-access devices. Returns true on success; otherwise returns false (for example, if the device is not open).

Note that when using a QTextStream on a QFile, calling reset() on the QFile will not have the expected result because QTextStream buffers the file. Use the QTextStream::seek() function instead.

See also seek().
*/
impl /*struct*/ QIODevice {
  pub fn reset_0<RetType, T: QIODevice_reset_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.reset_0(self);
    // return 1;
  }
}
pub trait QIODevice_reset_0<RetType> {
  fn reset_0(self , rsthis: & QIODevice) -> RetType;
}
impl<'a> /*trait*/ QIODevice_reset_0<bool> for () {
  fn reset_0(self , rsthis: & QIODevice) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QIODevice5resetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qiodevice.h:117
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] qint64 bytesAvailable() const

/*
Returns the number of bytes that are available for reading. This function is commonly used with sequential devices to determine the number of bytes to allocate in a buffer before reading.

Subclasses that reimplement this function must call the base implementation in order to include the size of the buffer of QIODevice. Example:


  qint64 CustomDevice::bytesAvailable() const
  {
      return buffer.size() + QIODevice::bytesAvailable();
  }



See also bytesToWrite(), readyRead(), and isSequential().
*/
impl /*struct*/ QIODevice {
  pub fn bytesAvailable_0<RetType, T: QIODevice_bytesAvailable_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.bytesAvailable_0(self);
    // return 1;
  }
}
pub trait QIODevice_bytesAvailable_0<RetType> {
  fn bytesAvailable_0(self , rsthis: & QIODevice) -> RetType;
}
impl<'a> /*trait*/ QIODevice_bytesAvailable_0<i64> for () {
  fn bytesAvailable_0(self , rsthis: & QIODevice) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QIODevice14bytesAvailableEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qiodevice.h:118
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] qint64 bytesToWrite() const

/*
For buffered devices, this function returns the number of bytes waiting to be written. For devices with no buffer, this function returns 0.

Subclasses that reimplement this function must call the base implementation in order to include the size of the buffer of QIODevice.

See also bytesAvailable(), bytesWritten(), and isSequential().
*/
impl /*struct*/ QIODevice {
  pub fn bytesToWrite_0<RetType, T: QIODevice_bytesToWrite_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.bytesToWrite_0(self);
    // return 1;
  }
}
pub trait QIODevice_bytesToWrite_0<RetType> {
  fn bytesToWrite_0(self , rsthis: & QIODevice) -> RetType;
}
impl<'a> /*trait*/ QIODevice_bytesToWrite_0<i64> for () {
  fn bytesToWrite_0(self , rsthis: & QIODevice) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QIODevice12bytesToWriteEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qiodevice.h:120
// index:0
// Public Visibility=Default Availability=Available
// [8] qint64 read(char *, qint64)

/*
Reads at most maxSize bytes from the device into data, and returns the number of bytes read. If an error occurs, such as when attempting to read from a device opened in WriteOnly mode, this function returns -1.

0 is returned when no more data is available for reading. However, reading past the end of the stream is considered an error, so this function returns -1 in those cases (that is, reading on a closed socket or after a process has died).

See also readData(), readLine(), and write().
*/
impl /*struct*/ QIODevice {
  pub fn read_0<RetType, T: QIODevice_read_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.read_0(self);
    // return 1;
  }
}
pub trait QIODevice_read_0<RetType> {
  fn read_0(self , rsthis: & QIODevice) -> RetType;
}
impl<'a> /*trait*/ QIODevice_read_0<i64> for (usize,i64) {
  fn read_0(self , rsthis: & QIODevice) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const i64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QIODevice4readEPcx", 2,qtrt::FFITY_POINTER,qtrt::FFITY_SINT64,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qiodevice.h:121
// index:1
// Public Visibility=Default Availability=Available
// [8] QByteArray read(qint64)

/*
Reads at most maxSize bytes from the device into data, and returns the number of bytes read. If an error occurs, such as when attempting to read from a device opened in WriteOnly mode, this function returns -1.

0 is returned when no more data is available for reading. However, reading past the end of the stream is considered an error, so this function returns -1 in those cases (that is, reading on a closed socket or after a process has died).

See also readData(), readLine(), and write().
*/
impl /*struct*/ QIODevice {
  pub fn read_1<RetType, T: QIODevice_read_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.read_1(self);
    // return 1;
  }
}
pub trait QIODevice_read_1<RetType> {
  fn read_1(self , rsthis: & QIODevice) -> RetType;
}
impl<'a> /*trait*/ QIODevice_read_1<usize> for (i64) {
  fn read_1(self , rsthis: & QIODevice) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QIODevice4readEx", 1,qtrt::FFITY_SINT64,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qiodevice.h:122
// index:0
// Public Visibility=Default Availability=Available
// [8] QByteArray readAll()

/*
Reads all remaining data from the device, and returns it as a byte array.

This function has no way of reporting errors; returning an empty QByteArray can mean either that no data was currently available for reading, or that an error occurred.
*/
impl /*struct*/ QIODevice {
  pub fn readAll_0<RetType, T: QIODevice_readAll_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.readAll_0(self);
    // return 1;
  }
}
pub trait QIODevice_readAll_0<RetType> {
  fn readAll_0(self , rsthis: & QIODevice) -> RetType;
}
impl<'a> /*trait*/ QIODevice_readAll_0<usize> for () {
  fn readAll_0(self , rsthis: & QIODevice) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QIODevice7readAllEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qiodevice.h:123
// index:0
// Public Visibility=Default Availability=Available
// [8] qint64 readLine(char *, qint64)

/*
This function reads a line of ASCII characters from the device, up to a maximum of maxSize - 1 bytes, stores the characters in data, and returns the number of bytes read. If a line could not be read but no error ocurred, this function returns 0. If an error occurs, this function returns the length of what could be read, or -1 if nothing was read.

A terminating '\0' byte is always appended to data, so maxSize must be larger than 1.

Data is read until either of the following conditions are met:


The first '\n' character is read.
maxSize - 1 bytes are read.
The end of the device data is detected.


For example, the following code reads a line of characters from a file:


  QFile file("box.txt");
  if (file.open(QFile::ReadOnly)) {
      char buf[1024];
      qint64 lineLength = file.readLine(buf, sizeof(buf));
      if (lineLength != -1) {
          // the line is available in buf
      }
  }



The newline character ('\n') is included in the buffer. If a newline is not encountered before maxSize - 1 bytes are read, a newline will not be inserted into the buffer. On windows newline characters are replaced with '\n'.

This function calls readLineData(), which is implemented using repeated calls to getChar(). You can provide a more efficient implementation by reimplementing readLineData() in your own subclass.

See also getChar(), read(), and write().
*/
impl /*struct*/ QIODevice {
  pub fn readLine_0<RetType, T: QIODevice_readLine_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.readLine_0(self);
    // return 1;
  }
}
pub trait QIODevice_readLine_0<RetType> {
  fn readLine_0(self , rsthis: & QIODevice) -> RetType;
}
impl<'a> /*trait*/ QIODevice_readLine_0<i64> for (usize,i64) {
  fn readLine_0(self , rsthis: & QIODevice) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const i64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QIODevice8readLineEPcx", 2,qtrt::FFITY_POINTER,qtrt::FFITY_SINT64,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qiodevice.h:124
// index:1
// Public Visibility=Default Availability=Available
// [8] QByteArray readLine(qint64)

/*
This function reads a line of ASCII characters from the device, up to a maximum of maxSize - 1 bytes, stores the characters in data, and returns the number of bytes read. If a line could not be read but no error ocurred, this function returns 0. If an error occurs, this function returns the length of what could be read, or -1 if nothing was read.

A terminating '\0' byte is always appended to data, so maxSize must be larger than 1.

Data is read until either of the following conditions are met:


The first '\n' character is read.
maxSize - 1 bytes are read.
The end of the device data is detected.


For example, the following code reads a line of characters from a file:


  QFile file("box.txt");
  if (file.open(QFile::ReadOnly)) {
      char buf[1024];
      qint64 lineLength = file.readLine(buf, sizeof(buf));
      if (lineLength != -1) {
          // the line is available in buf
      }
  }



The newline character ('\n') is included in the buffer. If a newline is not encountered before maxSize - 1 bytes are read, a newline will not be inserted into the buffer. On windows newline characters are replaced with '\n'.

This function calls readLineData(), which is implemented using repeated calls to getChar(). You can provide a more efficient implementation by reimplementing readLineData() in your own subclass.

See also getChar(), read(), and write().
*/
impl /*struct*/ QIODevice {
  pub fn readLine_1<RetType, T: QIODevice_readLine_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.readLine_1(self);
    // return 1;
  }
}
pub trait QIODevice_readLine_1<RetType> {
  fn readLine_1(self , rsthis: & QIODevice) -> RetType;
}
impl<'a> /*trait*/ QIODevice_readLine_1<usize> for (i64) {
  fn readLine_1(self , rsthis: & QIODevice) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QIODevice8readLineEx", 1,qtrt::FFITY_SINT64,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qiodevice.h:125
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool canReadLine() const

/*
Returns true if a complete line of data can be read from the device; otherwise returns false.

Note that unbuffered devices, which have no way of determining what can be read, always return false.

This function is often called in conjunction with the readyRead() signal.

Subclasses that reimplement this function must call the base implementation in order to include the contents of the QIODevice's buffer. Example:


  bool CustomDevice::canReadLine() const
  {
      return buffer.contains('\n') || QIODevice::canReadLine();
  }



See also readyRead() and readLine().
*/
impl /*struct*/ QIODevice {
  pub fn canReadLine_0<RetType, T: QIODevice_canReadLine_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.canReadLine_0(self);
    // return 1;
  }
}
pub trait QIODevice_canReadLine_0<RetType> {
  fn canReadLine_0(self , rsthis: & QIODevice) -> RetType;
}
impl<'a> /*trait*/ QIODevice_canReadLine_0<bool> for () {
  fn canReadLine_0(self , rsthis: & QIODevice) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QIODevice11canReadLineEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qiodevice.h:127
// index:0
// Public Visibility=Default Availability=Available
// [-2] void startTransaction()

/*
Starts a new read transaction on the device.

Defines a restorable point within the sequence of read operations. For sequential devices, read data will be duplicated internally to allow recovery in case of incomplete reads. For random-access devices, this function saves the current position. Call commitTransaction() or rollbackTransaction() to finish the transaction.

Note: Nesting transactions is not supported.

This function was introduced in  Qt 5.7.

See also commitTransaction() and rollbackTransaction().
*/
impl /*struct*/ QIODevice {
  pub fn startTransaction_0<RetType, T: QIODevice_startTransaction_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.startTransaction_0(self);
    // return 1;
  }
}
pub trait QIODevice_startTransaction_0<RetType> {
  fn startTransaction_0(self , rsthis: & QIODevice) -> RetType;
}
impl<'a> /*trait*/ QIODevice_startTransaction_0<(/*void*/)> for () {
  fn startTransaction_0(self , rsthis: & QIODevice) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN9QIODevice16startTransactionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qiodevice.h:128
// index:0
// Public Visibility=Default Availability=Available
// [-2] void commitTransaction()

/*
Completes a read transaction.

For sequential devices, all data recorded in the internal buffer during the transaction will be discarded.

This function was introduced in  Qt 5.7.

See also startTransaction() and rollbackTransaction().
*/
impl /*struct*/ QIODevice {
  pub fn commitTransaction_0<RetType, T: QIODevice_commitTransaction_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.commitTransaction_0(self);
    // return 1;
  }
}
pub trait QIODevice_commitTransaction_0<RetType> {
  fn commitTransaction_0(self , rsthis: & QIODevice) -> RetType;
}
impl<'a> /*trait*/ QIODevice_commitTransaction_0<(/*void*/)> for () {
  fn commitTransaction_0(self , rsthis: & QIODevice) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN9QIODevice17commitTransactionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qiodevice.h:129
// index:0
// Public Visibility=Default Availability=Available
// [-2] void rollbackTransaction()

/*
Rolls back a read transaction.

Restores the input stream to the point of the startTransaction() call. This function is commonly used to rollback the transaction when an incomplete read was detected prior to committing the transaction.

This function was introduced in  Qt 5.7.

See also startTransaction() and commitTransaction().
*/
impl /*struct*/ QIODevice {
  pub fn rollbackTransaction_0<RetType, T: QIODevice_rollbackTransaction_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rollbackTransaction_0(self);
    // return 1;
  }
}
pub trait QIODevice_rollbackTransaction_0<RetType> {
  fn rollbackTransaction_0(self , rsthis: & QIODevice) -> RetType;
}
impl<'a> /*trait*/ QIODevice_rollbackTransaction_0<(/*void*/)> for () {
  fn rollbackTransaction_0(self , rsthis: & QIODevice) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN9QIODevice19rollbackTransactionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qiodevice.h:130
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isTransactionStarted() const

/*
Returns true if a transaction is in progress on the device, otherwise false.

This function was introduced in  Qt 5.7.

See also startTransaction().
*/
impl /*struct*/ QIODevice {
  pub fn isTransactionStarted_0<RetType, T: QIODevice_isTransactionStarted_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isTransactionStarted_0(self);
    // return 1;
  }
}
pub trait QIODevice_isTransactionStarted_0<RetType> {
  fn isTransactionStarted_0(self , rsthis: & QIODevice) -> RetType;
}
impl<'a> /*trait*/ QIODevice_isTransactionStarted_0<bool> for () {
  fn isTransactionStarted_0(self , rsthis: & QIODevice) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QIODevice20isTransactionStartedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qiodevice.h:132
// index:0
// Public Visibility=Default Availability=Available
// [8] qint64 write(const char *, qint64)

/*
Writes at most maxSize bytes of data from data to the device. Returns the number of bytes that were actually written, or -1 if an error occurred.

See also read() and writeData().
*/
impl /*struct*/ QIODevice {
  pub fn write_0<RetType, T: QIODevice_write_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.write_0(self);
    // return 1;
  }
}
pub trait QIODevice_write_0<RetType> {
  fn write_0(self , rsthis: & QIODevice) -> RetType;
}
impl<'a> /*trait*/ QIODevice_write_0<i64> for (usize,i64) {
  fn write_0(self , rsthis: & QIODevice) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const i64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QIODevice5writeEPKcx", 2,qtrt::FFITY_POINTER,qtrt::FFITY_SINT64,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qiodevice.h:133
// index:1
// Public Visibility=Default Availability=Available
// [8] qint64 write(const char *)

/*
Writes at most maxSize bytes of data from data to the device. Returns the number of bytes that were actually written, or -1 if an error occurred.

See also read() and writeData().
*/
impl /*struct*/ QIODevice {
  pub fn write_1<RetType, T: QIODevice_write_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.write_1(self);
    // return 1;
  }
}
pub trait QIODevice_write_1<RetType> {
  fn write_1(self , rsthis: & QIODevice) -> RetType;
}
impl<'a> /*trait*/ QIODevice_write_1<i64> for (usize) {
  fn write_1(self , rsthis: & QIODevice) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QIODevice5writeEPKc", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qiodevice.h:134
// index:2
// Public inline Visibility=Default Availability=Available
// [8] qint64 write(const QByteArray &)

/*
Writes at most maxSize bytes of data from data to the device. Returns the number of bytes that were actually written, or -1 if an error occurred.

See also read() and writeData().
*/
impl /*struct*/ QIODevice {
  pub fn write_2<RetType, T: QIODevice_write_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.write_2(self);
    // return 1;
  }
}
pub trait QIODevice_write_2<RetType> {
  fn write_2(self , rsthis: & QIODevice) -> RetType;
}
impl<'a> /*trait*/ QIODevice_write_2<i64> for (usize) {
  fn write_2(self , rsthis: & QIODevice) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QIODevice5writeERK10QByteArray", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qiodevice.h:137
// index:0
// Public Visibility=Default Availability=Available
// [8] qint64 peek(char *, qint64)

/*
Reads at most maxSize bytes from the device into data, without side effects (i.e., if you call read() after peek(), you will get the same data). Returns the number of bytes read. If an error occurs, such as when attempting to peek a device opened in WriteOnly mode, this function returns -1.

0 is returned when no more data is available for reading.

Example:


  bool isExeFile(QFile *file)
  {
      char buf[2];
      if (file->peek(buf, sizeof(buf)) == sizeof(buf))
          return (buf[0] == 'M' && buf[1] == 'Z');
      return false;
  }



This function was introduced in  Qt 4.1.

See also read().
*/
impl /*struct*/ QIODevice {
  pub fn peek_0<RetType, T: QIODevice_peek_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.peek_0(self);
    // return 1;
  }
}
pub trait QIODevice_peek_0<RetType> {
  fn peek_0(self , rsthis: & QIODevice) -> RetType;
}
impl<'a> /*trait*/ QIODevice_peek_0<i64> for (usize,i64) {
  fn peek_0(self , rsthis: & QIODevice) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const i64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QIODevice4peekEPcx", 2,qtrt::FFITY_POINTER,qtrt::FFITY_SINT64,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qiodevice.h:138
// index:1
// Public Visibility=Default Availability=Available
// [8] QByteArray peek(qint64)

/*
Reads at most maxSize bytes from the device into data, without side effects (i.e., if you call read() after peek(), you will get the same data). Returns the number of bytes read. If an error occurs, such as when attempting to peek a device opened in WriteOnly mode, this function returns -1.

0 is returned when no more data is available for reading.

Example:


  bool isExeFile(QFile *file)
  {
      char buf[2];
      if (file->peek(buf, sizeof(buf)) == sizeof(buf))
          return (buf[0] == 'M' && buf[1] == 'Z');
      return false;
  }



This function was introduced in  Qt 4.1.

See also read().
*/
impl /*struct*/ QIODevice {
  pub fn peek_1<RetType, T: QIODevice_peek_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.peek_1(self);
    // return 1;
  }
}
pub trait QIODevice_peek_1<RetType> {
  fn peek_1(self , rsthis: & QIODevice) -> RetType;
}
impl<'a> /*trait*/ QIODevice_peek_1<usize> for (i64) {
  fn peek_1(self , rsthis: & QIODevice) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QIODevice4peekEx", 1,qtrt::FFITY_SINT64,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qiodevice.h:139
// index:0
// Public Visibility=Default Availability=Available
// [8] qint64 skip(qint64)

/*
Skips up to maxSize bytes from the device. Returns the number of bytes actually skipped, or -1 on error.

This function does not wait and only discards the data that is already available for reading.

If the device is opened in text mode, end-of-line terminators are translated to '\n' symbols and count as a single byte identically to the read() and peek() behavior.

This function works for all devices, including sequential ones that cannot seek(). It is optimized to skip unwanted data after a peek() call.

For random-access devices, skip() can be used to seek forward from the current position. Negative maxSize values are not allowed.

This function was introduced in  Qt 5.10.

See also peek(), seek(), and read().
*/
impl /*struct*/ QIODevice {
  pub fn skip_0<RetType, T: QIODevice_skip_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.skip_0(self);
    // return 1;
  }
}
pub trait QIODevice_skip_0<RetType> {
  fn skip_0(self , rsthis: & QIODevice) -> RetType;
}
impl<'a> /*trait*/ QIODevice_skip_0<i64> for (i64) {
  fn skip_0(self , rsthis: & QIODevice) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QIODevice4skipEx", 1,qtrt::FFITY_SINT64,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qiodevice.h:141
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool waitForReadyRead(int)

/*
Blocks until new data is available for reading and the readyRead() signal has been emitted, or until msecs milliseconds have passed. If msecs is -1, this function will not time out.

Returns true if new data is available for reading; otherwise returns false (if the operation timed out or if an error occurred).

This function can operate without an event loop. It is useful when writing non-GUI applications and when performing I/O operations in a non-GUI thread.

If called from within a slot connected to the readyRead() signal, readyRead() will not be reemitted.

Reimplement this function to provide a blocking API for a custom device. The default implementation does nothing, and returns false.

Warning: Calling this function from the main (GUI) thread might cause your user interface to freeze.

See also waitForBytesWritten().
*/
impl /*struct*/ QIODevice {
  pub fn waitForReadyRead_0<RetType, T: QIODevice_waitForReadyRead_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.waitForReadyRead_0(self);
    // return 1;
  }
}
pub trait QIODevice_waitForReadyRead_0<RetType> {
  fn waitForReadyRead_0(self , rsthis: & QIODevice) -> RetType;
}
impl<'a> /*trait*/ QIODevice_waitForReadyRead_0<bool> for (i32) {
  fn waitForReadyRead_0(self , rsthis: & QIODevice) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QIODevice16waitForReadyReadEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qiodevice.h:142
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool waitForBytesWritten(int)

/*
For buffered devices, this function waits until a payload of buffered written data has been written to the device and the bytesWritten() signal has been emitted, or until msecs milliseconds have passed. If msecs is -1, this function will not time out. For unbuffered devices, it returns immediately.

Returns true if a payload of data was written to the device; otherwise returns false (i.e. if the operation timed out, or if an error occurred).

This function can operate without an event loop. It is useful when writing non-GUI applications and when performing I/O operations in a non-GUI thread.

If called from within a slot connected to the bytesWritten() signal, bytesWritten() will not be reemitted.

Reimplement this function to provide a blocking API for a custom device. The default implementation does nothing, and returns false.

Warning: Calling this function from the main (GUI) thread might cause your user interface to freeze.

See also waitForReadyRead().
*/
impl /*struct*/ QIODevice {
  pub fn waitForBytesWritten_0<RetType, T: QIODevice_waitForBytesWritten_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.waitForBytesWritten_0(self);
    // return 1;
  }
}
pub trait QIODevice_waitForBytesWritten_0<RetType> {
  fn waitForBytesWritten_0(self , rsthis: & QIODevice) -> RetType;
}
impl<'a> /*trait*/ QIODevice_waitForBytesWritten_0<bool> for (i32) {
  fn waitForBytesWritten_0(self , rsthis: & QIODevice) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QIODevice19waitForBytesWrittenEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qiodevice.h:144
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ungetChar(char)

/*
Puts the character c back into the device, and decrements the current position unless the position is 0. This function is usually called to "undo" a getChar() operation, such as when writing a backtracking parser.

If c was not previously read from the device, the behavior is undefined.

Note: This function is not available while a transaction is in progress.
*/
impl /*struct*/ QIODevice {
  pub fn ungetChar_0<RetType, T: QIODevice_ungetChar_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.ungetChar_0(self);
    // return 1;
  }
}
pub trait QIODevice_ungetChar_0<RetType> {
  fn ungetChar_0(self , rsthis: & QIODevice) -> RetType;
}
impl<'a> /*trait*/ QIODevice_ungetChar_0<(/*void*/)> for (i8) {
  fn ungetChar_0(self , rsthis: & QIODevice) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i8 as usize;
     qtrt::InvokeQtFunc6("_ZN9QIODevice9ungetCharEc", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qiodevice.h:145
// index:0
// Public Visibility=Default Availability=Available
// [1] bool putChar(char)

/*
Writes the character c to the device. Returns true on success; otherwise returns false.

See also write(), getChar(), and ungetChar().
*/
impl /*struct*/ QIODevice {
  pub fn putChar_0<RetType, T: QIODevice_putChar_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.putChar_0(self);
    // return 1;
  }
}
pub trait QIODevice_putChar_0<RetType> {
  fn putChar_0(self , rsthis: & QIODevice) -> RetType;
}
impl<'a> /*trait*/ QIODevice_putChar_0<bool> for (i8) {
  fn putChar_0(self , rsthis: & QIODevice) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i8 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QIODevice7putCharEc", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qiodevice.h:146
// index:0
// Public Visibility=Default Availability=Available
// [1] bool getChar(char *)

/*
Reads one character from the device and stores it in c. If c is 0, the character is discarded. Returns true on success; otherwise returns false.

See also read(), putChar(), and ungetChar().
*/
impl /*struct*/ QIODevice {
  pub fn getChar_0<RetType, T: QIODevice_getChar_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.getChar_0(self);
    // return 1;
  }
}
pub trait QIODevice_getChar_0<RetType> {
  fn getChar_0(self , rsthis: & QIODevice) -> RetType;
}
impl<'a> /*trait*/ QIODevice_getChar_0<bool> for (usize) {
  fn getChar_0(self , rsthis: & QIODevice) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QIODevice7getCharEPc", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qiodevice.h:148
// index:0
// Public Visibility=Default Availability=Available
// [8] QString errorString() const

/*
Returns a human-readable description of the last device error that occurred.

See also setErrorString().
*/
impl /*struct*/ QIODevice {
  pub fn errorString_0<RetType, T: QIODevice_errorString_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.errorString_0(self);
    // return 1;
  }
}
pub trait QIODevice_errorString_0<RetType> {
  fn errorString_0(self , rsthis: & QIODevice) -> RetType;
}
impl<'a> /*trait*/ QIODevice_errorString_0<usize> for () {
  fn errorString_0(self , rsthis: & QIODevice) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QIODevice11errorStringEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qiodevice.h:152
// index:0
// Public Visibility=Default Availability=Available
// [-2] void readyRead()

/*
This signal is emitted once every time new data is available for reading from the device's current read channel. It will only be emitted again once new data is available, such as when a new payload of network data has arrived on your network socket, or when a new block of data has been appended to your device.

readyRead() is not emitted recursively; if you reenter the event loop or call waitForReadyRead() inside a slot connected to the readyRead() signal, the signal will not be reemitted (although waitForReadyRead() may still return true).

Note for developers implementing classes derived from QIODevice: you should always emit readyRead() when new data has arrived (do not emit it only because there's data still to be read in your buffers). Do not emit readyRead() in other conditions.

See also bytesWritten().
*/
impl /*struct*/ QIODevice {
  pub fn readyRead_0<RetType, T: QIODevice_readyRead_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.readyRead_0(self);
    // return 1;
  }
}
pub trait QIODevice_readyRead_0<RetType> {
  fn readyRead_0(self , rsthis: & QIODevice) -> RetType;
}
impl<'a> /*trait*/ QIODevice_readyRead_0<(/*void*/)> for () {
  fn readyRead_0(self , rsthis: & QIODevice) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN9QIODevice9readyReadEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qiodevice.h:153
// index:0
// Public Visibility=Default Availability=Available
// [-2] void channelReadyRead(int)

/*
This signal is emitted when new data is available for reading from the device. The channel argument is set to the index of the read channel on which the data has arrived. Unlike readyRead(), it is emitted regardless of the current read channel.

channelReadyRead() can be emitted recursively - even for the same channel.

This function was introduced in  Qt 5.7.

See also readyRead() and channelBytesWritten().
*/
impl /*struct*/ QIODevice {
  pub fn channelReadyRead_0<RetType, T: QIODevice_channelReadyRead_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.channelReadyRead_0(self);
    // return 1;
  }
}
pub trait QIODevice_channelReadyRead_0<RetType> {
  fn channelReadyRead_0(self , rsthis: & QIODevice) -> RetType;
}
impl<'a> /*trait*/ QIODevice_channelReadyRead_0<(/*void*/)> for (i32) {
  fn channelReadyRead_0(self , rsthis: & QIODevice) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QIODevice16channelReadyReadEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qiodevice.h:154
// index:0
// Public Visibility=Default Availability=Available
// [-2] void bytesWritten(qint64)

/*
This signal is emitted every time a payload of data has been written to the device's current write channel. The bytes argument is set to the number of bytes that were written in this payload.

bytesWritten() is not emitted recursively; if you reenter the event loop or call waitForBytesWritten() inside a slot connected to the bytesWritten() signal, the signal will not be reemitted (although waitForBytesWritten() may still return true).

See also readyRead().
*/
impl /*struct*/ QIODevice {
  pub fn bytesWritten_0<RetType, T: QIODevice_bytesWritten_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.bytesWritten_0(self);
    // return 1;
  }
}
pub trait QIODevice_bytesWritten_0<RetType> {
  fn bytesWritten_0(self , rsthis: & QIODevice) -> RetType;
}
impl<'a> /*trait*/ QIODevice_bytesWritten_0<(/*void*/)> for (i64) {
  fn bytesWritten_0(self , rsthis: & QIODevice) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i64 as usize;
     qtrt::InvokeQtFunc6("_ZN9QIODevice12bytesWrittenEx", 1,qtrt::FFITY_SINT64,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qiodevice.h:155
// index:0
// Public Visibility=Default Availability=Available
// [-2] void channelBytesWritten(int, qint64)

/*
This signal is emitted every time a payload of data has been written to the device. The bytes argument is set to the number of bytes that were written in this payload, while channel is the channel they were written to. Unlike bytesWritten(), it is emitted regardless of the current write channel.

channelBytesWritten() can be emitted recursively - even for the same channel.

This function was introduced in  Qt 5.7.

See also bytesWritten() and channelReadyRead().
*/
impl /*struct*/ QIODevice {
  pub fn channelBytesWritten_0<RetType, T: QIODevice_channelBytesWritten_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.channelBytesWritten_0(self);
    // return 1;
  }
}
pub trait QIODevice_channelBytesWritten_0<RetType> {
  fn channelBytesWritten_0(self , rsthis: & QIODevice) -> RetType;
}
impl<'a> /*trait*/ QIODevice_channelBytesWritten_0<(/*void*/)> for (i32,i64) {
  fn channelBytesWritten_0(self , rsthis: & QIODevice) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i64 as usize;
     qtrt::InvokeQtFunc6("_ZN9QIODevice19channelBytesWrittenEix", 2,qtrt::FFITY_INT,qtrt::FFITY_SINT64,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qiodevice.h:156
// index:0
// Public Visibility=Default Availability=Available
// [-2] void aboutToClose()

/*
This signal is emitted when the device is about to close. Connect this signal if you have operations that need to be performed before the device closes (e.g., if you have data in a separate buffer that needs to be written to the device).
*/
impl /*struct*/ QIODevice {
  pub fn aboutToClose_0<RetType, T: QIODevice_aboutToClose_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.aboutToClose_0(self);
    // return 1;
  }
}
pub trait QIODevice_aboutToClose_0<RetType> {
  fn aboutToClose_0(self , rsthis: & QIODevice) -> RetType;
}
impl<'a> /*trait*/ QIODevice_aboutToClose_0<(/*void*/)> for () {
  fn aboutToClose_0(self , rsthis: & QIODevice) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN9QIODevice12aboutToCloseEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qiodevice.h:157
// index:0
// Public Visibility=Default Availability=Available
// [-2] void readChannelFinished()

/*
This signal is emitted when the input (reading) stream is closed in this device. It is emitted as soon as the closing is detected, which means that there might still be data available for reading with read().

This function was introduced in  Qt 4.4.

See also atEnd() and read().
*/
impl /*struct*/ QIODevice {
  pub fn readChannelFinished_0<RetType, T: QIODevice_readChannelFinished_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.readChannelFinished_0(self);
    // return 1;
  }
}
pub trait QIODevice_readChannelFinished_0<RetType> {
  fn readChannelFinished_0(self , rsthis: & QIODevice) -> RetType;
}
impl<'a> /*trait*/ QIODevice_readChannelFinished_0<(/*void*/)> for () {
  fn readChannelFinished_0(self , rsthis: & QIODevice) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN9QIODevice19readChannelFinishedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qiodevice.h:166
// index:0
// Protected purevirtual virtual Visibility=Default Availability=Available
// [8] qint64 readData(char *, qint64)

/*
Reads up to maxSize bytes from the device into data, and returns the number of bytes read or -1 if an error occurred.

If there are no bytes to be read and there can never be more bytes available (examples include socket closed, pipe closed, sub-process finished), this function returns -1.

This function is called by QIODevice. Reimplement this function when creating a subclass of QIODevice.

When reimplementing this function it is important that this function reads all the required data before returning. This is required in order for QDataStream to be able to operate on the class. QDataStream assumes all the requested information was read and therefore does not retry reading if there was a problem.

This function might be called with a maxSize of 0, which can be used to perform post-reading operations.

See also read(), readLine(), and writeData().
*/
impl /*struct*/ QIODevice {
  pub fn readData_0<RetType, T: QIODevice_readData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.readData_0(self);
    // return 1;
  }
}
pub trait QIODevice_readData_0<RetType> {
  fn readData_0(self , rsthis: & QIODevice) -> RetType;
}
impl<'a> /*trait*/ QIODevice_readData_0<i64> for (usize,i64) {
  fn readData_0(self , rsthis: & QIODevice) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const i64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QIODevice8readDataEPcx", 2,qtrt::FFITY_POINTER,qtrt::FFITY_SINT64,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qiodevice.h:167
// index:0
// Protected virtual Visibility=Default Availability=Available
// [8] qint64 readLineData(char *, qint64)

/*
Reads up to maxSize characters into data and returns the number of characters read.

This function is called by readLine(), and provides its base implementation, using getChar(). Buffered devices can improve the performance of readLine() by reimplementing this function.

readLine() appends a '\0' byte to data; readLineData() does not need to do this.

If you reimplement this function, be careful to return the correct value: it should return the number of bytes read in this line, including the terminating newline, or 0 if there is no line to be read at this point. If an error occurs, it should return -1 if and only if no bytes were read. Reading past EOF is considered an error.
*/
impl /*struct*/ QIODevice {
  pub fn readLineData_0<RetType, T: QIODevice_readLineData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.readLineData_0(self);
    // return 1;
  }
}
pub trait QIODevice_readLineData_0<RetType> {
  fn readLineData_0(self , rsthis: & QIODevice) -> RetType;
}
impl<'a> /*trait*/ QIODevice_readLineData_0<i64> for (usize,i64) {
  fn readLineData_0(self , rsthis: & QIODevice) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const i64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QIODevice12readLineDataEPcx", 2,qtrt::FFITY_POINTER,qtrt::FFITY_SINT64,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qiodevice.h:168
// index:0
// Protected purevirtual virtual Visibility=Default Availability=Available
// [8] qint64 writeData(const char *, qint64)

/*
Writes up to maxSize bytes from data to the device. Returns the number of bytes written, or -1 if an error occurred.

This function is called by QIODevice. Reimplement this function when creating a subclass of QIODevice.

When reimplementing this function it is important that this function writes all the data available before returning. This is required in order for QDataStream to be able to operate on the class. QDataStream assumes all the information was written and therefore does not retry writing if there was a problem.

See also read() and write().
*/
impl /*struct*/ QIODevice {
  pub fn writeData_0<RetType, T: QIODevice_writeData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.writeData_0(self);
    // return 1;
  }
}
pub trait QIODevice_writeData_0<RetType> {
  fn writeData_0(self , rsthis: & QIODevice) -> RetType;
}
impl<'a> /*trait*/ QIODevice_writeData_0<i64> for (usize,i64) {
  fn writeData_0(self , rsthis: & QIODevice) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const i64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QIODevice9writeDataEPKcx", 2,qtrt::FFITY_POINTER,qtrt::FFITY_SINT64,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qiodevice.h:170
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void setOpenMode(QIODevice::OpenMode)

/*
Sets the OpenMode of the device to openMode. Call this function to set the open mode if the flags change after the device has been opened.

See also openMode() and OpenMode.
*/
impl /*struct*/ QIODevice {
  pub fn setOpenMode_0<RetType, T: QIODevice_setOpenMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setOpenMode_0(self);
    // return 1;
  }
}
pub trait QIODevice_setOpenMode_0<RetType> {
  fn setOpenMode_0(self , rsthis: & QIODevice) -> RetType;
}
impl<'a> /*trait*/ QIODevice_setOpenMode_0<(/*void*/)> for (i32) {
  fn setOpenMode_0(self , rsthis: & QIODevice) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QIODevice11setOpenModeE6QFlagsINS_12OpenModeFlagEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qiodevice.h:172
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void setErrorString(const QString &)

/*
Sets the human readable description of the last device error that occurred to str.

See also errorString().
*/
impl /*struct*/ QIODevice {
  pub fn setErrorString_0<RetType, T: QIODevice_setErrorString_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setErrorString_0(self);
    // return 1;
  }
}
pub trait QIODevice_setErrorString_0<RetType> {
  fn setErrorString_0(self , rsthis: & QIODevice) -> RetType;
}
impl<'a> /*trait*/ QIODevice_setErrorString_0<(/*void*/)> for (usize) {
  fn setErrorString_0(self , rsthis: & QIODevice) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QIODevice14setErrorStringERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}


/*


*/
pub type QIODevice__OpenModeFlag = i32;
// 
pub const QIODevice__NotOpen :QIODevice__OpenModeFlag = 0;
// 
pub const QIODevice__ReadOnly :QIODevice__OpenModeFlag = 1;
// 
pub const QIODevice__WriteOnly :QIODevice__OpenModeFlag = 2;
// 
pub const QIODevice__ReadWrite :QIODevice__OpenModeFlag = 3;
// 
pub const QIODevice__Append :QIODevice__OpenModeFlag = 4;
// 
pub const QIODevice__Truncate :QIODevice__OpenModeFlag = 8;
// 
pub const QIODevice__Text :QIODevice__OpenModeFlag = 16;
// 
pub const QIODevice__Unbuffered :QIODevice__OpenModeFlag = 32;
pub fn QIODevice_OpenModeFlagItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QIODevice", val);
}
pub fn QIODevice_OpenModeFlagItemName_s(val: i32) ->String {
  //var nilthis *QIODevice
  //return nilthis.OpenModeFlagItemName(val);
  return QIODevice_OpenModeFlagItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
