

// mod ::core::QBuffer
// package qtcore
// /usr/include/qt/QtCore/qbuffer.h
// #include <qbuffer.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 59
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

// void connectNotify(const QMetaMethod &)
// func (this *QBuffer) InheritConnectNotify(f func(arg0 *QMetaMethod) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "connectNotify", f)
// }

// void disconnectNotify(const QMetaMethod &)
// func (this *QBuffer) InheritDisconnectNotify(f func(arg0 *QMetaMethod) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "disconnectNotify", f)
// }

// long long readData(char *, qint64)
// func (this *QBuffer) InheritReadData(f func(data string, maxlen int64) int64) {
//  qtrt.SetAllInheritCallback(this, "readData", f)
// }

// long long writeData(const char *, qint64)
// func (this *QBuffer) InheritWriteData(f func(data string, len_ int64) int64) {
//  qtrt.SetAllInheritCallback(this, "writeData", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QBuffer)=16
pub struct QBuffer {
  qbase: QIODevice,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QBuffer_ITF interface {
//    QIODevice_ITF
//    QBuffer_PTR() *QBuffer
//}
//func (ptr *QBuffer) QBuffer_PTR() *QBuffer { return ptr }

impl /*struct*/ QBuffer {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QBuffer {
    return QBuffer{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QBuffer {
//  type Target = QBufferBASE;
//
//  fn deref(&self) -> &QBufferBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QBufferBASE> for QBuffer {
//  fn as_ref(& self) -> & QBufferBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qbuffer.h:55
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QBuffer {
  pub fn metaObject_0<RetType, T: QBuffer_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QBuffer_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QBuffer) -> RetType;
}
impl<'a> /*trait*/ QBuffer_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QBuffer) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QBuffer10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbuffer.h:60
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QBuffer(QObject *)

/*
Constructs an empty buffer with the given parent. You can call setData() to fill the buffer with data, or you can open it in write mode and use write().

See also open().
*/
// QBuffer(QObject *) ctx.fn_proto_cpp
impl /*struct*/ QBuffer {
  pub fn QBuffer_0<T: QBuffer_QBuffer_0>(value: T) -> QBuffer {
    let rsthis = value.QBuffer_0();
    return rsthis;
    // return 1;
  }
}

pub trait QBuffer_QBuffer_0 {
  fn QBuffer_0(self) -> QBuffer;
}
// QBuffer(QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QBuffer_QBuffer_0 for (usize) {
  fn QBuffer_0(self) -> QBuffer {
    // unsafe{_ZN7QBufferC2EP7QObject()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN7QBufferC2EP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QBuffer{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qbuffer.h:61
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QBuffer(QByteArray *, QObject *)

/*
Constructs an empty buffer with the given parent. You can call setData() to fill the buffer with data, or you can open it in write mode and use write().

See also open().
*/
// QBuffer(QByteArray *, QObject *) ctx.fn_proto_cpp
impl /*struct*/ QBuffer {
  pub fn QBuffer_1<T: QBuffer_QBuffer_1>(value: T) -> QBuffer {
    let rsthis = value.QBuffer_1();
    return rsthis;
    // return 1;
  }
}

pub trait QBuffer_QBuffer_1 {
  fn QBuffer_1(self) -> QBuffer;
}
// QBuffer(QByteArray *, QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QBuffer_QBuffer_1 for (usize,usize) {
  fn QBuffer_1(self) -> QBuffer {
    // unsafe{_ZN7QBufferC2EP10QByteArrayP7QObject()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN7QBufferC2EP10QByteArrayP7QObject", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QBuffer{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qbuffer.h:66
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QBuffer()

/*

*/
pub fn DeleteQBuffer(this :*mut QBuffer) {
    // let rv = qtrt::InvokeQtFunc6("_ZN7QBufferD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qbuffer.h:68
// index:0
// Public Visibility=Default Availability=Available
// [8] QByteArray & buffer()

/*
Returns a reference to the QBuffer's internal buffer. You can use it to modify the QByteArray behind the QBuffer's back.

See also setBuffer() and data().
*/
impl /*struct*/ QBuffer {
  pub fn buffer_0<RetType, T: QBuffer_buffer_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.buffer_0(self);
    // return 1;
  }
}
pub trait QBuffer_buffer_0<RetType> {
  fn buffer_0(self , rsthis: & QBuffer) -> RetType;
}
impl<'a> /*trait*/ QBuffer_buffer_0<usize> for () {
  fn buffer_0(self , rsthis: & QBuffer) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QBuffer6bufferEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbuffer.h:69
// index:1
// Public Visibility=Default Availability=Available
// [8] const QByteArray & buffer() const

/*
Returns a reference to the QBuffer's internal buffer. You can use it to modify the QByteArray behind the QBuffer's back.

See also setBuffer() and data().
*/
impl /*struct*/ QBuffer {
  pub fn buffer_1<RetType, T: QBuffer_buffer_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.buffer_1(self);
    // return 1;
  }
}
pub trait QBuffer_buffer_1<RetType> {
  fn buffer_1(self , rsthis: & QBuffer) -> RetType;
}
impl<'a> /*trait*/ QBuffer_buffer_1<usize> for () {
  fn buffer_1(self , rsthis: & QBuffer) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QBuffer6bufferEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbuffer.h:70
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setBuffer(QByteArray *)

/*
Makes QBuffer uses the QByteArray pointed to by byteArray as its internal buffer. The caller is responsible for ensuring that byteArray remains valid until the QBuffer is destroyed, or until setBuffer() is called to change the buffer. QBuffer doesn't take ownership of the QByteArray.

Does nothing if isOpen() is true.

If you open the buffer in write-only mode or read-write mode and write something into the QBuffer, byteArray will be modified.

Example:


      QByteArray byteArray("abc");
      QBuffer buffer;
      buffer.setBuffer(&byteArray);
      buffer.open(QIODevice::WriteOnly);
      buffer.seek(3);
      buffer.write("def", 3);
      buffer.close();
      // byteArray == "abcdef"



If byteArray is 0, the buffer creates its own internal QByteArray to work on. This byte array is initially empty.

See also buffer(), setData(), and open().
*/
impl /*struct*/ QBuffer {
  pub fn setBuffer_0<RetType, T: QBuffer_setBuffer_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setBuffer_0(self);
    // return 1;
  }
}
pub trait QBuffer_setBuffer_0<RetType> {
  fn setBuffer_0(self , rsthis: & QBuffer) -> RetType;
}
impl<'a> /*trait*/ QBuffer_setBuffer_0<(/*void*/)> for (usize) {
  fn setBuffer_0(self , rsthis: & QBuffer) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QBuffer9setBufferEP10QByteArray", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qbuffer.h:72
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setData(const QByteArray &)

/*
Sets the contents of the internal buffer to be data. This is the same as assigning data to buffer().

Does nothing if isOpen() is true.

See also data() and setBuffer().
*/
impl /*struct*/ QBuffer {
  pub fn setData_0<RetType, T: QBuffer_setData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setData_0(self);
    // return 1;
  }
}
pub trait QBuffer_setData_0<RetType> {
  fn setData_0(self , rsthis: & QBuffer) -> RetType;
}
impl<'a> /*trait*/ QBuffer_setData_0<(/*void*/)> for (usize) {
  fn setData_0(self , rsthis: & QBuffer) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QBuffer7setDataERK10QByteArray", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qbuffer.h:73
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void setData(const char *, int)

/*
Sets the contents of the internal buffer to be data. This is the same as assigning data to buffer().

Does nothing if isOpen() is true.

See also data() and setBuffer().
*/
impl /*struct*/ QBuffer {
  pub fn setData_1<RetType, T: QBuffer_setData_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setData_1(self);
    // return 1;
  }
}
pub trait QBuffer_setData_1<RetType> {
  fn setData_1(self , rsthis: & QBuffer) -> RetType;
}
impl<'a> /*trait*/ QBuffer_setData_1<(/*void*/)> for (usize,i32) {
  fn setData_1(self , rsthis: & QBuffer) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QBuffer7setDataEPKci", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qbuffer.h:74
// index:0
// Public Visibility=Default Availability=Available
// [8] const QByteArray & data() const

/*
Returns the data contained in the buffer.

This is the same as buffer().

See also setData() and setBuffer().
*/
impl /*struct*/ QBuffer {
  pub fn data_0<RetType, T: QBuffer_data_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.data_0(self);
    // return 1;
  }
}
pub trait QBuffer_data_0<RetType> {
  fn data_0(self , rsthis: & QBuffer) -> RetType;
}
impl<'a> /*trait*/ QBuffer_data_0<usize> for () {
  fn data_0(self , rsthis: & QBuffer) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QBuffer4dataEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbuffer.h:76
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool open(QIODevice::OpenMode)

/*
Reimplemented from QIODevice::open().
*/
impl /*struct*/ QBuffer {
  pub fn open_0<RetType, T: QBuffer_open_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.open_0(self);
    // return 1;
  }
}
pub trait QBuffer_open_0<RetType> {
  fn open_0(self , rsthis: & QBuffer) -> RetType;
}
impl<'a> /*trait*/ QBuffer_open_0<bool> for (i32) {
  fn open_0(self , rsthis: & QBuffer) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QBuffer4openE6QFlagsIN9QIODevice12OpenModeFlagEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbuffer.h:78
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void close()

/*
Reimplemented from QIODevice::close().
*/
impl /*struct*/ QBuffer {
  pub fn close_0<RetType, T: QBuffer_close_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.close_0(self);
    // return 1;
  }
}
pub trait QBuffer_close_0<RetType> {
  fn close_0(self , rsthis: & QBuffer) -> RetType;
}
impl<'a> /*trait*/ QBuffer_close_0<(/*void*/)> for () {
  fn close_0(self , rsthis: & QBuffer) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN7QBuffer5closeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qbuffer.h:79
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] qint64 size() const

/*
Reimplemented from QIODevice::size().
*/
impl /*struct*/ QBuffer {
  pub fn size_0<RetType, T: QBuffer_size_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.size_0(self);
    // return 1;
  }
}
pub trait QBuffer_size_0<RetType> {
  fn size_0(self , rsthis: & QBuffer) -> RetType;
}
impl<'a> /*trait*/ QBuffer_size_0<i64> for () {
  fn size_0(self , rsthis: & QBuffer) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QBuffer4sizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbuffer.h:80
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] qint64 pos() const

/*
Reimplemented from QIODevice::pos().
*/
impl /*struct*/ QBuffer {
  pub fn pos_0<RetType, T: QBuffer_pos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.pos_0(self);
    // return 1;
  }
}
pub trait QBuffer_pos_0<RetType> {
  fn pos_0(self , rsthis: & QBuffer) -> RetType;
}
impl<'a> /*trait*/ QBuffer_pos_0<i64> for () {
  fn pos_0(self , rsthis: & QBuffer) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QBuffer3posEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbuffer.h:81
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool seek(qint64)

/*
Reimplemented from QIODevice::seek().
*/
impl /*struct*/ QBuffer {
  pub fn seek_0<RetType, T: QBuffer_seek_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.seek_0(self);
    // return 1;
  }
}
pub trait QBuffer_seek_0<RetType> {
  fn seek_0(self , rsthis: & QBuffer) -> RetType;
}
impl<'a> /*trait*/ QBuffer_seek_0<bool> for (i64) {
  fn seek_0(self , rsthis: & QBuffer) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QBuffer4seekEx", 1,qtrt::FFITY_SINT64,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbuffer.h:82
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool atEnd() const

/*
Reimplemented from QIODevice::atEnd().
*/
impl /*struct*/ QBuffer {
  pub fn atEnd_0<RetType, T: QBuffer_atEnd_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.atEnd_0(self);
    // return 1;
  }
}
pub trait QBuffer_atEnd_0<RetType> {
  fn atEnd_0(self , rsthis: & QBuffer) -> RetType;
}
impl<'a> /*trait*/ QBuffer_atEnd_0<bool> for () {
  fn atEnd_0(self , rsthis: & QBuffer) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QBuffer5atEndEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbuffer.h:83
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool canReadLine() const

/*
Reimplemented from QIODevice::canReadLine().
*/
impl /*struct*/ QBuffer {
  pub fn canReadLine_0<RetType, T: QBuffer_canReadLine_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.canReadLine_0(self);
    // return 1;
  }
}
pub trait QBuffer_canReadLine_0<RetType> {
  fn canReadLine_0(self , rsthis: & QBuffer) -> RetType;
}
impl<'a> /*trait*/ QBuffer_canReadLine_0<bool> for () {
  fn canReadLine_0(self , rsthis: & QBuffer) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QBuffer11canReadLineEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbuffer.h:87
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void connectNotify(const QMetaMethod &)

/*

*/
impl /*struct*/ QBuffer {
  pub fn connectNotify_0<RetType, T: QBuffer_connectNotify_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.connectNotify_0(self);
    // return 1;
  }
}
pub trait QBuffer_connectNotify_0<RetType> {
  fn connectNotify_0(self , rsthis: & QBuffer) -> RetType;
}
impl<'a> /*trait*/ QBuffer_connectNotify_0<(/*void*/)> for (usize) {
  fn connectNotify_0(self , rsthis: & QBuffer) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QBuffer13connectNotifyERK11QMetaMethod", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qbuffer.h:88
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void disconnectNotify(const QMetaMethod &)

/*

*/
impl /*struct*/ QBuffer {
  pub fn disconnectNotify_0<RetType, T: QBuffer_disconnectNotify_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.disconnectNotify_0(self);
    // return 1;
  }
}
pub trait QBuffer_disconnectNotify_0<RetType> {
  fn disconnectNotify_0(self , rsthis: & QBuffer) -> RetType;
}
impl<'a> /*trait*/ QBuffer_disconnectNotify_0<(/*void*/)> for (usize) {
  fn disconnectNotify_0(self , rsthis: & QBuffer) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QBuffer16disconnectNotifyERK11QMetaMethod", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qbuffer.h:90
// index:0
// Protected virtual Visibility=Default Availability=Available
// [8] qint64 readData(char *, qint64)

/*
Reimplemented from QIODevice::readData().
*/
impl /*struct*/ QBuffer {
  pub fn readData_0<RetType, T: QBuffer_readData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.readData_0(self);
    // return 1;
  }
}
pub trait QBuffer_readData_0<RetType> {
  fn readData_0(self , rsthis: & QBuffer) -> RetType;
}
impl<'a> /*trait*/ QBuffer_readData_0<i64> for (usize,i64) {
  fn readData_0(self , rsthis: & QBuffer) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const i64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QBuffer8readDataEPcx", 2,qtrt::FFITY_POINTER,qtrt::FFITY_SINT64,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbuffer.h:91
// index:0
// Protected virtual Visibility=Default Availability=Available
// [8] qint64 writeData(const char *, qint64)

/*
Reimplemented from QIODevice::writeData().
*/
impl /*struct*/ QBuffer {
  pub fn writeData_0<RetType, T: QBuffer_writeData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.writeData_0(self);
    // return 1;
  }
}
pub trait QBuffer_writeData_0<RetType> {
  fn writeData_0(self , rsthis: & QBuffer) -> RetType;
}
impl<'a> /*trait*/ QBuffer_writeData_0<i64> for (usize,i64) {
  fn writeData_0(self , rsthis: & QBuffer) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const i64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QBuffer9writeDataEPKcx", 2,qtrt::FFITY_POINTER,qtrt::FFITY_SINT64,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i64 = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
