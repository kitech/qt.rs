

// mod ::core::QVariant
// package qtcore
// /usr/include/qt/QtCore/qvariant.h
// #include <qvariant.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 0
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

// void create(int, const void *)
// func (this *QVariant) InheritCreate(f func(type_ int, copy unsafe.Pointer /*666*/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "create", f)
// }

// bool cmp(const QVariant &)
// func (this *QVariant) InheritCmp(f func(other *QVariant) bool) {
//  qtrt.SetAllInheritCallback(this, "cmp", f)
// }

// int compare(const QVariant &)
// func (this *QVariant) InheritCompare(f func(other *QVariant) int) {
//  qtrt.SetAllInheritCallback(this, "compare", f)
// }

// bool convert(const int, void *)
// func (this *QVariant) InheritConvert(f func(t int, ptr unsafe.Pointer /*666*/) bool) {
//  qtrt.SetAllInheritCallback(this, "convert", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QVariant)=16
pub struct QVariant {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QVariant_ITF interface {
//    QVariant_PTR() *QVariant
//}
//func (ptr *QVariant) QVariant_PTR() *QVariant { return ptr }

impl /*struct*/ QVariant {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QVariant {
    return QVariant{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QVariant {
//  type Target = QVariantBASE;
//
//  fn deref(&self) -> &QVariantBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QVariantBASE> for QVariant {
//  fn as_ref(& self) -> & QVariantBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qvariant.h:199
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void QVariant()

/*
Constructs an invalid variant.
*/
// QVariant() ctx.fn_proto_cpp
impl /*struct*/ QVariant {
  pub fn QVariant_0<T: QVariant_QVariant_0>(value: T) -> QVariant {
    let rsthis = value.QVariant_0();
    return rsthis;
    // return 1;
  }
}

pub trait QVariant_QVariant_0 {
  fn QVariant_0(self) -> QVariant;
}
// QVariant() ctx.fn_proto_cpp
impl<'a> /*trait*/ QVariant_QVariant_0 for () {
  fn QVariant_0(self) -> QVariant {
    // unsafe{_ZN8QVariantC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN8QVariantC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QVariant{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qvariant.h:201
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QVariant(QVariant::Type)

/*
Constructs an invalid variant.
*/
// QVariant(QVariant::Type) ctx.fn_proto_cpp
impl /*struct*/ QVariant {
  pub fn QVariant_1<T: QVariant_QVariant_1>(value: T) -> QVariant {
    let rsthis = value.QVariant_1();
    return rsthis;
    // return 1;
  }
}

pub trait QVariant_QVariant_1 {
  fn QVariant_1(self) -> QVariant;
}
// QVariant(QVariant::Type) ctx.fn_proto_cpp
impl<'a> /*trait*/ QVariant_QVariant_1 for (i32) {
  fn QVariant_1(self) -> QVariant {
    // unsafe{_ZN8QVariantC2ENS_4TypeE()};
    let arg0 = (&self) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN8QVariantC2ENS_4TypeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QVariant{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qvariant.h:202
// index:2
// Public Visibility=Default Availability=Available
// [-2] void QVariant(int, const void *)

/*
Constructs an invalid variant.
*/
// QVariant(int, const void *) ctx.fn_proto_cpp
impl /*struct*/ QVariant {
  pub fn QVariant_2<T: QVariant_QVariant_2>(value: T) -> QVariant {
    let rsthis = value.QVariant_2();
    return rsthis;
    // return 1;
  }
}

pub trait QVariant_QVariant_2 {
  fn QVariant_2(self) -> QVariant;
}
// QVariant(int, const void *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QVariant_QVariant_2 for (i32,usize) {
  fn QVariant_2(self) -> QVariant {
    // unsafe{_ZN8QVariantC2EiPKv()};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN8QVariantC2EiPKv", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QVariant{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qvariant.h:203
// index:3
// Public Visibility=Default Availability=Available
// [-2] void QVariant(int, const void *, uint)

/*
Constructs an invalid variant.
*/
// QVariant(int, const void *, uint) ctx.fn_proto_cpp
impl /*struct*/ QVariant {
  pub fn QVariant_3<T: QVariant_QVariant_3>(value: T) -> QVariant {
    let rsthis = value.QVariant_3();
    return rsthis;
    // return 1;
  }
}

pub trait QVariant_QVariant_3 {
  fn QVariant_3(self) -> QVariant;
}
// QVariant(int, const void *, uint) ctx.fn_proto_cpp
impl<'a> /*trait*/ QVariant_QVariant_3 for (i32,usize,u32) {
  fn QVariant_3(self) -> QVariant {
    // unsafe{_ZN8QVariantC2EiPKvj()};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const usize as usize;
    let arg2 = (&self.2) as *const u32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN8QVariantC2EiPKvj", 3,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_UINT32,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QVariant{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qvariant.h:207
// index:4
// Public Visibility=Default Availability=Available
// [-2] void QVariant(QDataStream &)

/*
Constructs an invalid variant.
*/
// QVariant(QDataStream &) ctx.fn_proto_cpp
impl /*struct*/ QVariant {
  pub fn QVariant_4<T: QVariant_QVariant_4>(value: T) -> QVariant {
    let rsthis = value.QVariant_4();
    return rsthis;
    // return 1;
  }
}

pub trait QVariant_QVariant_4 {
  fn QVariant_4(self) -> QVariant;
}
// QVariant(QDataStream &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QVariant_QVariant_4 for (usize) {
  fn QVariant_4(self) -> QVariant {
    // unsafe{_ZN8QVariantC2ER11QDataStream()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN8QVariantC2ER11QDataStream", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QVariant{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qvariant.h:210
// index:5
// Public Visibility=Default Availability=Available
// [-2] void QVariant(int)

/*
Constructs an invalid variant.
*/
// QVariant(int) ctx.fn_proto_cpp
impl /*struct*/ QVariant {
  pub fn QVariant_5<T: QVariant_QVariant_5>(value: T) -> QVariant {
    let rsthis = value.QVariant_5();
    return rsthis;
    // return 1;
  }
}

pub trait QVariant_QVariant_5 {
  fn QVariant_5(self) -> QVariant;
}
// QVariant(int) ctx.fn_proto_cpp
impl<'a> /*trait*/ QVariant_QVariant_5 for (i32) {
  fn QVariant_5(self) -> QVariant {
    // unsafe{_ZN8QVariantC2Ei()};
    let arg0 = (&self) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN8QVariantC2Ei", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QVariant{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qvariant.h:211
// index:6
// Public Visibility=Default Availability=Available
// [-2] void QVariant(uint)

/*
Constructs an invalid variant.
*/
// QVariant(uint) ctx.fn_proto_cpp
impl /*struct*/ QVariant {
  pub fn QVariant_6<T: QVariant_QVariant_6>(value: T) -> QVariant {
    let rsthis = value.QVariant_6();
    return rsthis;
    // return 1;
  }
}

pub trait QVariant_QVariant_6 {
  fn QVariant_6(self) -> QVariant;
}
// QVariant(uint) ctx.fn_proto_cpp
impl<'a> /*trait*/ QVariant_QVariant_6 for (u32) {
  fn QVariant_6(self) -> QVariant {
    // unsafe{_ZN8QVariantC2Ej()};
    let arg0 = (&self) as *const u32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN8QVariantC2Ej", 1,qtrt::FFITY_UINT32,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QVariant{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qvariant.h:212
// index:7
// Public Visibility=Default Availability=Available
// [-2] void QVariant(qlonglong)

/*
Constructs an invalid variant.
*/
// QVariant(qlonglong) ctx.fn_proto_cpp
impl /*struct*/ QVariant {
  pub fn QVariant_7<T: QVariant_QVariant_7>(value: T) -> QVariant {
    let rsthis = value.QVariant_7();
    return rsthis;
    // return 1;
  }
}

pub trait QVariant_QVariant_7 {
  fn QVariant_7(self) -> QVariant;
}
// QVariant(qlonglong) ctx.fn_proto_cpp
impl<'a> /*trait*/ QVariant_QVariant_7 for (i64) {
  fn QVariant_7(self) -> QVariant {
    // unsafe{_ZN8QVariantC2Ex()};
    let arg0 = (&self) as *const i64 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN8QVariantC2Ex", 1,qtrt::FFITY_SINT64,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QVariant{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qvariant.h:213
// index:8
// Public Visibility=Default Availability=Available
// [-2] void QVariant(qulonglong)

/*
Constructs an invalid variant.
*/
// QVariant(qulonglong) ctx.fn_proto_cpp
impl /*struct*/ QVariant {
  pub fn QVariant_8<T: QVariant_QVariant_8>(value: T) -> QVariant {
    let rsthis = value.QVariant_8();
    return rsthis;
    // return 1;
  }
}

pub trait QVariant_QVariant_8 {
  fn QVariant_8(self) -> QVariant;
}
// QVariant(qulonglong) ctx.fn_proto_cpp
impl<'a> /*trait*/ QVariant_QVariant_8 for (u64) {
  fn QVariant_8(self) -> QVariant {
    // unsafe{_ZN8QVariantC2Ey()};
    let arg0 = (&self) as *const u64 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN8QVariantC2Ey", 1,qtrt::FFITY_UINT64,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QVariant{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qvariant.h:214
// index:9
// Public Visibility=Default Availability=Available
// [-2] void QVariant(bool)

/*
Constructs an invalid variant.
*/
// QVariant(bool) ctx.fn_proto_cpp
impl /*struct*/ QVariant {
  pub fn QVariant_9<T: QVariant_QVariant_9>(value: T) -> QVariant {
    let rsthis = value.QVariant_9();
    return rsthis;
    // return 1;
  }
}

pub trait QVariant_QVariant_9 {
  fn QVariant_9(self) -> QVariant;
}
// QVariant(bool) ctx.fn_proto_cpp
impl<'a> /*trait*/ QVariant_QVariant_9 for (bool) {
  fn QVariant_9(self) -> QVariant {
    // unsafe{_ZN8QVariantC2Eb()};
    let arg0 = (&self) as *const bool as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN8QVariantC2Eb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QVariant{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qvariant.h:215
// index:10
// Public Visibility=Default Availability=Available
// [-2] void QVariant(double)

/*
Constructs an invalid variant.
*/
// QVariant(double) ctx.fn_proto_cpp
impl /*struct*/ QVariant {
  pub fn QVariant_10<T: QVariant_QVariant_10>(value: T) -> QVariant {
    let rsthis = value.QVariant_10();
    return rsthis;
    // return 1;
  }
}

pub trait QVariant_QVariant_10 {
  fn QVariant_10(self) -> QVariant;
}
// QVariant(double) ctx.fn_proto_cpp
impl<'a> /*trait*/ QVariant_QVariant_10 for (f64) {
  fn QVariant_10(self) -> QVariant {
    // unsafe{_ZN8QVariantC2Ed()};
    let arg0 = (&self) as *const f64 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN8QVariantC2Ed", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QVariant{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qvariant.h:216
// index:11
// Public Visibility=Default Availability=Available
// [-2] void QVariant(float)

/*
Constructs an invalid variant.
*/
// QVariant(float) ctx.fn_proto_cpp
impl /*struct*/ QVariant {
  pub fn QVariant_11<T: QVariant_QVariant_11>(value: T) -> QVariant {
    let rsthis = value.QVariant_11();
    return rsthis;
    // return 1;
  }
}

pub trait QVariant_QVariant_11 {
  fn QVariant_11(self) -> QVariant;
}
// QVariant(float) ctx.fn_proto_cpp
impl<'a> /*trait*/ QVariant_QVariant_11 for (f32) {
  fn QVariant_11(self) -> QVariant {
    // unsafe{_ZN8QVariantC2Ef()};
    let arg0 = (&self) as *const f32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN8QVariantC2Ef", 1,qtrt::FFITY_FLOAT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QVariant{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qvariant.h:218
// index:12
// Public Visibility=Default Availability=Available
// [-2] void QVariant(const char *)

/*
Constructs an invalid variant.
*/
// QVariant(const char *) ctx.fn_proto_cpp
impl /*struct*/ QVariant {
  pub fn QVariant_12<T: QVariant_QVariant_12>(value: T) -> QVariant {
    let rsthis = value.QVariant_12();
    return rsthis;
    // return 1;
  }
}

pub trait QVariant_QVariant_12 {
  fn QVariant_12(self) -> QVariant;
}
// QVariant(const char *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QVariant_QVariant_12 for (usize) {
  fn QVariant_12(self) -> QVariant {
    // unsafe{_ZN8QVariantC2EPKc()};
    let arg0 = (self) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN8QVariantC2EPKc", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QVariant{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qvariant.h:221
// index:13
// Public Visibility=Default Availability=Available
// [-2] void QVariant(const QByteArray &)

/*
Constructs an invalid variant.
*/
// QVariant(const QByteArray &) ctx.fn_proto_cpp
impl /*struct*/ QVariant {
  pub fn QVariant_13<T: QVariant_QVariant_13>(value: T) -> QVariant {
    let rsthis = value.QVariant_13();
    return rsthis;
    // return 1;
  }
}

pub trait QVariant_QVariant_13 {
  fn QVariant_13(self) -> QVariant;
}
// QVariant(const QByteArray &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QVariant_QVariant_13 for (usize) {
  fn QVariant_13(self) -> QVariant {
    // unsafe{_ZN8QVariantC2ERK10QByteArray()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN8QVariantC2ERK10QByteArray", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QVariant{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qvariant.h:222
// index:14
// Public Visibility=Default Availability=Available
// [-2] void QVariant(const QBitArray &)

/*
Constructs an invalid variant.
*/
// QVariant(const QBitArray &) ctx.fn_proto_cpp
impl /*struct*/ QVariant {
  pub fn QVariant_14<T: QVariant_QVariant_14>(value: T) -> QVariant {
    let rsthis = value.QVariant_14();
    return rsthis;
    // return 1;
  }
}

pub trait QVariant_QVariant_14 {
  fn QVariant_14(self) -> QVariant;
}
// QVariant(const QBitArray &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QVariant_QVariant_14 for (usize) {
  fn QVariant_14(self) -> QVariant {
    // unsafe{_ZN8QVariantC2ERK9QBitArray()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN8QVariantC2ERK9QBitArray", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QVariant{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qvariant.h:223
// index:15
// Public Visibility=Default Availability=Available
// [-2] void QVariant(const QString &)

/*
Constructs an invalid variant.
*/
// QVariant(const QString &) ctx.fn_proto_cpp
impl /*struct*/ QVariant {
  pub fn QVariant_15<T: QVariant_QVariant_15>(value: T) -> QVariant {
    let rsthis = value.QVariant_15();
    return rsthis;
    // return 1;
  }
}

pub trait QVariant_QVariant_15 {
  fn QVariant_15(self) -> QVariant;
}
// QVariant(const QString &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QVariant_QVariant_15 for (usize) {
  fn QVariant_15(self) -> QVariant {
    // unsafe{_ZN8QVariantC2ERK7QString()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN8QVariantC2ERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QVariant{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qvariant.h:224
// index:16
// Public Visibility=Default Availability=Available
// [-2] void QVariant(QLatin1String)

/*
Constructs an invalid variant.
*/
// QVariant(QLatin1String) ctx.fn_proto_cpp
impl /*struct*/ QVariant {
  pub fn QVariant_16<T: QVariant_QVariant_16>(value: T) -> QVariant {
    let rsthis = value.QVariant_16();
    return rsthis;
    // return 1;
  }
}

pub trait QVariant_QVariant_16 {
  fn QVariant_16(self) -> QVariant;
}
// QVariant(QLatin1String) ctx.fn_proto_cpp
impl<'a> /*trait*/ QVariant_QVariant_16 for (usize) {
  fn QVariant_16(self) -> QVariant {
    // unsafe{_ZN8QVariantC2E13QLatin1String()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN8QVariantC2E13QLatin1String", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QVariant{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qvariant.h:225
// index:17
// Public Visibility=Default Availability=Available
// [-2] void QVariant(const QStringList &)

/*
Constructs an invalid variant.
*/
// QVariant(const QStringList &) ctx.fn_proto_cpp
impl /*struct*/ QVariant {
  pub fn QVariant_17<T: QVariant_QVariant_17>(value: T) -> QVariant {
    let rsthis = value.QVariant_17();
    return rsthis;
    // return 1;
  }
}

pub trait QVariant_QVariant_17 {
  fn QVariant_17(self) -> QVariant;
}
// QVariant(const QStringList &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QVariant_QVariant_17 for (usize) {
  fn QVariant_17(self) -> QVariant {
    // unsafe{_ZN8QVariantC2ERK11QStringList()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN8QVariantC2ERK11QStringList", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QVariant{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qvariant.h:226
// index:18
// Public Visibility=Default Availability=Available
// [-2] void QVariant(QChar)

/*
Constructs an invalid variant.
*/
// QVariant(QChar) ctx.fn_proto_cpp
impl /*struct*/ QVariant {
  pub fn QVariant_18<T: QVariant_QVariant_18>(value: T) -> QVariant {
    let rsthis = value.QVariant_18();
    return rsthis;
    // return 1;
  }
}

pub trait QVariant_QVariant_18 {
  fn QVariant_18(self) -> QVariant;
}
// QVariant(QChar) ctx.fn_proto_cpp
impl<'a> /*trait*/ QVariant_QVariant_18 for (usize) {
  fn QVariant_18(self) -> QVariant {
    // unsafe{_ZN8QVariantC2E5QChar()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN8QVariantC2E5QChar", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QVariant{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qvariant.h:227
// index:19
// Public Visibility=Default Availability=Available
// [-2] void QVariant(const QDate &)

/*
Constructs an invalid variant.
*/
// QVariant(const QDate &) ctx.fn_proto_cpp
impl /*struct*/ QVariant {
  pub fn QVariant_19<T: QVariant_QVariant_19>(value: T) -> QVariant {
    let rsthis = value.QVariant_19();
    return rsthis;
    // return 1;
  }
}

pub trait QVariant_QVariant_19 {
  fn QVariant_19(self) -> QVariant;
}
// QVariant(const QDate &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QVariant_QVariant_19 for (usize) {
  fn QVariant_19(self) -> QVariant {
    // unsafe{_ZN8QVariantC2ERK5QDate()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN8QVariantC2ERK5QDate", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QVariant{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qvariant.h:228
// index:20
// Public Visibility=Default Availability=Available
// [-2] void QVariant(const QTime &)

/*
Constructs an invalid variant.
*/
// QVariant(const QTime &) ctx.fn_proto_cpp
impl /*struct*/ QVariant {
  pub fn QVariant_20<T: QVariant_QVariant_20>(value: T) -> QVariant {
    let rsthis = value.QVariant_20();
    return rsthis;
    // return 1;
  }
}

pub trait QVariant_QVariant_20 {
  fn QVariant_20(self) -> QVariant;
}
// QVariant(const QTime &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QVariant_QVariant_20 for (usize) {
  fn QVariant_20(self) -> QVariant {
    // unsafe{_ZN8QVariantC2ERK5QTime()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN8QVariantC2ERK5QTime", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QVariant{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qvariant.h:229
// index:21
// Public Visibility=Default Availability=Available
// [-2] void QVariant(const QDateTime &)

/*
Constructs an invalid variant.
*/
// QVariant(const QDateTime &) ctx.fn_proto_cpp
impl /*struct*/ QVariant {
  pub fn QVariant_21<T: QVariant_QVariant_21>(value: T) -> QVariant {
    let rsthis = value.QVariant_21();
    return rsthis;
    // return 1;
  }
}

pub trait QVariant_QVariant_21 {
  fn QVariant_21(self) -> QVariant;
}
// QVariant(const QDateTime &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QVariant_QVariant_21 for (usize) {
  fn QVariant_21(self) -> QVariant {
    // unsafe{_ZN8QVariantC2ERK9QDateTime()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN8QVariantC2ERK9QDateTime", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QVariant{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qvariant.h:234
// index:22
// Public Visibility=Default Availability=Available
// [-2] void QVariant(const QSize &)

/*
Constructs an invalid variant.
*/
// QVariant(const QSize &) ctx.fn_proto_cpp
impl /*struct*/ QVariant {
  pub fn QVariant_22<T: QVariant_QVariant_22>(value: T) -> QVariant {
    let rsthis = value.QVariant_22();
    return rsthis;
    // return 1;
  }
}

pub trait QVariant_QVariant_22 {
  fn QVariant_22(self) -> QVariant;
}
// QVariant(const QSize &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QVariant_QVariant_22 for (usize) {
  fn QVariant_22(self) -> QVariant {
    // unsafe{_ZN8QVariantC2ERK5QSize()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN8QVariantC2ERK5QSize", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QVariant{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qvariant.h:235
// index:23
// Public Visibility=Default Availability=Available
// [-2] void QVariant(const QSizeF &)

/*
Constructs an invalid variant.
*/
// QVariant(const QSizeF &) ctx.fn_proto_cpp
impl /*struct*/ QVariant {
  pub fn QVariant_23<T: QVariant_QVariant_23>(value: T) -> QVariant {
    let rsthis = value.QVariant_23();
    return rsthis;
    // return 1;
  }
}

pub trait QVariant_QVariant_23 {
  fn QVariant_23(self) -> QVariant;
}
// QVariant(const QSizeF &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QVariant_QVariant_23 for (usize) {
  fn QVariant_23(self) -> QVariant {
    // unsafe{_ZN8QVariantC2ERK6QSizeF()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN8QVariantC2ERK6QSizeF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QVariant{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qvariant.h:236
// index:24
// Public Visibility=Default Availability=Available
// [-2] void QVariant(const QPoint &)

/*
Constructs an invalid variant.
*/
// QVariant(const QPoint &) ctx.fn_proto_cpp
impl /*struct*/ QVariant {
  pub fn QVariant_24<T: QVariant_QVariant_24>(value: T) -> QVariant {
    let rsthis = value.QVariant_24();
    return rsthis;
    // return 1;
  }
}

pub trait QVariant_QVariant_24 {
  fn QVariant_24(self) -> QVariant;
}
// QVariant(const QPoint &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QVariant_QVariant_24 for (usize) {
  fn QVariant_24(self) -> QVariant {
    // unsafe{_ZN8QVariantC2ERK6QPoint()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN8QVariantC2ERK6QPoint", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QVariant{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qvariant.h:237
// index:25
// Public Visibility=Default Availability=Available
// [-2] void QVariant(const QPointF &)

/*
Constructs an invalid variant.
*/
// QVariant(const QPointF &) ctx.fn_proto_cpp
impl /*struct*/ QVariant {
  pub fn QVariant_25<T: QVariant_QVariant_25>(value: T) -> QVariant {
    let rsthis = value.QVariant_25();
    return rsthis;
    // return 1;
  }
}

pub trait QVariant_QVariant_25 {
  fn QVariant_25(self) -> QVariant;
}
// QVariant(const QPointF &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QVariant_QVariant_25 for (usize) {
  fn QVariant_25(self) -> QVariant {
    // unsafe{_ZN8QVariantC2ERK7QPointF()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN8QVariantC2ERK7QPointF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QVariant{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qvariant.h:238
// index:26
// Public Visibility=Default Availability=Available
// [-2] void QVariant(const QLine &)

/*
Constructs an invalid variant.
*/
// QVariant(const QLine &) ctx.fn_proto_cpp
impl /*struct*/ QVariant {
  pub fn QVariant_26<T: QVariant_QVariant_26>(value: T) -> QVariant {
    let rsthis = value.QVariant_26();
    return rsthis;
    // return 1;
  }
}

pub trait QVariant_QVariant_26 {
  fn QVariant_26(self) -> QVariant;
}
// QVariant(const QLine &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QVariant_QVariant_26 for (usize) {
  fn QVariant_26(self) -> QVariant {
    // unsafe{_ZN8QVariantC2ERK5QLine()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN8QVariantC2ERK5QLine", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QVariant{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qvariant.h:239
// index:27
// Public Visibility=Default Availability=Available
// [-2] void QVariant(const QLineF &)

/*
Constructs an invalid variant.
*/
// QVariant(const QLineF &) ctx.fn_proto_cpp
impl /*struct*/ QVariant {
  pub fn QVariant_27<T: QVariant_QVariant_27>(value: T) -> QVariant {
    let rsthis = value.QVariant_27();
    return rsthis;
    // return 1;
  }
}

pub trait QVariant_QVariant_27 {
  fn QVariant_27(self) -> QVariant;
}
// QVariant(const QLineF &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QVariant_QVariant_27 for (usize) {
  fn QVariant_27(self) -> QVariant {
    // unsafe{_ZN8QVariantC2ERK6QLineF()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN8QVariantC2ERK6QLineF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QVariant{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qvariant.h:240
// index:28
// Public Visibility=Default Availability=Available
// [-2] void QVariant(const QRect &)

/*
Constructs an invalid variant.
*/
// QVariant(const QRect &) ctx.fn_proto_cpp
impl /*struct*/ QVariant {
  pub fn QVariant_28<T: QVariant_QVariant_28>(value: T) -> QVariant {
    let rsthis = value.QVariant_28();
    return rsthis;
    // return 1;
  }
}

pub trait QVariant_QVariant_28 {
  fn QVariant_28(self) -> QVariant;
}
// QVariant(const QRect &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QVariant_QVariant_28 for (usize) {
  fn QVariant_28(self) -> QVariant {
    // unsafe{_ZN8QVariantC2ERK5QRect()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN8QVariantC2ERK5QRect", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QVariant{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qvariant.h:241
// index:29
// Public Visibility=Default Availability=Available
// [-2] void QVariant(const QRectF &)

/*
Constructs an invalid variant.
*/
// QVariant(const QRectF &) ctx.fn_proto_cpp
impl /*struct*/ QVariant {
  pub fn QVariant_29<T: QVariant_QVariant_29>(value: T) -> QVariant {
    let rsthis = value.QVariant_29();
    return rsthis;
    // return 1;
  }
}

pub trait QVariant_QVariant_29 {
  fn QVariant_29(self) -> QVariant;
}
// QVariant(const QRectF &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QVariant_QVariant_29 for (usize) {
  fn QVariant_29(self) -> QVariant {
    // unsafe{_ZN8QVariantC2ERK6QRectF()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN8QVariantC2ERK6QRectF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QVariant{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qvariant.h:243
// index:30
// Public Visibility=Default Availability=Available
// [-2] void QVariant(const QLocale &)

/*
Constructs an invalid variant.
*/
// QVariant(const QLocale &) ctx.fn_proto_cpp
impl /*struct*/ QVariant {
  pub fn QVariant_30<T: QVariant_QVariant_30>(value: T) -> QVariant {
    let rsthis = value.QVariant_30();
    return rsthis;
    // return 1;
  }
}

pub trait QVariant_QVariant_30 {
  fn QVariant_30(self) -> QVariant;
}
// QVariant(const QLocale &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QVariant_QVariant_30 for (usize) {
  fn QVariant_30(self) -> QVariant {
    // unsafe{_ZN8QVariantC2ERK7QLocale()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN8QVariantC2ERK7QLocale", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QVariant{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qvariant.h:245
// index:31
// Public Visibility=Default Availability=Available
// [-2] void QVariant(const QRegExp &)

/*
Constructs an invalid variant.
*/
// QVariant(const QRegExp &) ctx.fn_proto_cpp
impl /*struct*/ QVariant {
  pub fn QVariant_31<T: QVariant_QVariant_31>(value: T) -> QVariant {
    let rsthis = value.QVariant_31();
    return rsthis;
    // return 1;
  }
}

pub trait QVariant_QVariant_31 {
  fn QVariant_31(self) -> QVariant;
}
// QVariant(const QRegExp &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QVariant_QVariant_31 for (usize) {
  fn QVariant_31(self) -> QVariant {
    // unsafe{_ZN8QVariantC2ERK7QRegExp()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN8QVariantC2ERK7QRegExp", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QVariant{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qvariant.h:249
// index:32
// Public Visibility=Default Availability=Available
// [-2] void QVariant(const QRegularExpression &)

/*
Constructs an invalid variant.
*/
// QVariant(const QRegularExpression &) ctx.fn_proto_cpp
impl /*struct*/ QVariant {
  pub fn QVariant_32<T: QVariant_QVariant_32>(value: T) -> QVariant {
    let rsthis = value.QVariant_32();
    return rsthis;
    // return 1;
  }
}

pub trait QVariant_QVariant_32 {
  fn QVariant_32(self) -> QVariant;
}
// QVariant(const QRegularExpression &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QVariant_QVariant_32 for (usize) {
  fn QVariant_32(self) -> QVariant {
    // unsafe{_ZN8QVariantC2ERK18QRegularExpression()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN8QVariantC2ERK18QRegularExpression", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QVariant{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qvariant.h:251
// index:33
// Public Visibility=Default Availability=Available
// [-2] void QVariant(const QUrl &)

/*
Constructs an invalid variant.
*/
// QVariant(const QUrl &) ctx.fn_proto_cpp
impl /*struct*/ QVariant {
  pub fn QVariant_33<T: QVariant_QVariant_33>(value: T) -> QVariant {
    let rsthis = value.QVariant_33();
    return rsthis;
    // return 1;
  }
}

pub trait QVariant_QVariant_33 {
  fn QVariant_33(self) -> QVariant;
}
// QVariant(const QUrl &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QVariant_QVariant_33 for (usize) {
  fn QVariant_33(self) -> QVariant {
    // unsafe{_ZN8QVariantC2ERK4QUrl()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN8QVariantC2ERK4QUrl", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QVariant{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qvariant.h:252
// index:34
// Public Visibility=Default Availability=Available
// [-2] void QVariant(const QEasingCurve &)

/*
Constructs an invalid variant.
*/
// QVariant(const QEasingCurve &) ctx.fn_proto_cpp
impl /*struct*/ QVariant {
  pub fn QVariant_34<T: QVariant_QVariant_34>(value: T) -> QVariant {
    let rsthis = value.QVariant_34();
    return rsthis;
    // return 1;
  }
}

pub trait QVariant_QVariant_34 {
  fn QVariant_34(self) -> QVariant;
}
// QVariant(const QEasingCurve &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QVariant_QVariant_34 for (usize) {
  fn QVariant_34(self) -> QVariant {
    // unsafe{_ZN8QVariantC2ERK12QEasingCurve()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN8QVariantC2ERK12QEasingCurve", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QVariant{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qvariant.h:253
// index:35
// Public Visibility=Default Availability=Available
// [-2] void QVariant(const QUuid &)

/*
Constructs an invalid variant.
*/
// QVariant(const QUuid &) ctx.fn_proto_cpp
impl /*struct*/ QVariant {
  pub fn QVariant_35<T: QVariant_QVariant_35>(value: T) -> QVariant {
    let rsthis = value.QVariant_35();
    return rsthis;
    // return 1;
  }
}

pub trait QVariant_QVariant_35 {
  fn QVariant_35(self) -> QVariant;
}
// QVariant(const QUuid &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QVariant_QVariant_35 for (usize) {
  fn QVariant_35(self) -> QVariant {
    // unsafe{_ZN8QVariantC2ERK5QUuid()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN8QVariantC2ERK5QUuid", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QVariant{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qvariant.h:254
// index:36
// Public Visibility=Default Availability=Available
// [-2] void QVariant(const QModelIndex &)

/*
Constructs an invalid variant.
*/
// QVariant(const QModelIndex &) ctx.fn_proto_cpp
impl /*struct*/ QVariant {
  pub fn QVariant_36<T: QVariant_QVariant_36>(value: T) -> QVariant {
    let rsthis = value.QVariant_36();
    return rsthis;
    // return 1;
  }
}

pub trait QVariant_QVariant_36 {
  fn QVariant_36(self) -> QVariant;
}
// QVariant(const QModelIndex &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QVariant_QVariant_36 for (usize) {
  fn QVariant_36(self) -> QVariant {
    // unsafe{_ZN8QVariantC2ERK11QModelIndex()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN8QVariantC2ERK11QModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QVariant{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qvariant.h:255
// index:37
// Public Visibility=Default Availability=Available
// [-2] void QVariant(const QPersistentModelIndex &)

/*
Constructs an invalid variant.
*/
// QVariant(const QPersistentModelIndex &) ctx.fn_proto_cpp
impl /*struct*/ QVariant {
  pub fn QVariant_37<T: QVariant_QVariant_37>(value: T) -> QVariant {
    let rsthis = value.QVariant_37();
    return rsthis;
    // return 1;
  }
}

pub trait QVariant_QVariant_37 {
  fn QVariant_37(self) -> QVariant;
}
// QVariant(const QPersistentModelIndex &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QVariant_QVariant_37 for (usize) {
  fn QVariant_37(self) -> QVariant {
    // unsafe{_ZN8QVariantC2ERK21QPersistentModelIndex()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN8QVariantC2ERK21QPersistentModelIndex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QVariant{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qvariant.h:256
// index:38
// Public Visibility=Default Availability=Available
// [-2] void QVariant(const QJsonValue &)

/*
Constructs an invalid variant.
*/
// QVariant(const QJsonValue &) ctx.fn_proto_cpp
impl /*struct*/ QVariant {
  pub fn QVariant_38<T: QVariant_QVariant_38>(value: T) -> QVariant {
    let rsthis = value.QVariant_38();
    return rsthis;
    // return 1;
  }
}

pub trait QVariant_QVariant_38 {
  fn QVariant_38(self) -> QVariant;
}
// QVariant(const QJsonValue &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QVariant_QVariant_38 for (usize) {
  fn QVariant_38(self) -> QVariant {
    // unsafe{_ZN8QVariantC2ERK10QJsonValue()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN8QVariantC2ERK10QJsonValue", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QVariant{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qvariant.h:257
// index:39
// Public Visibility=Default Availability=Available
// [-2] void QVariant(const QJsonObject &)

/*
Constructs an invalid variant.
*/
// QVariant(const QJsonObject &) ctx.fn_proto_cpp
impl /*struct*/ QVariant {
  pub fn QVariant_39<T: QVariant_QVariant_39>(value: T) -> QVariant {
    let rsthis = value.QVariant_39();
    return rsthis;
    // return 1;
  }
}

pub trait QVariant_QVariant_39 {
  fn QVariant_39(self) -> QVariant;
}
// QVariant(const QJsonObject &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QVariant_QVariant_39 for (usize) {
  fn QVariant_39(self) -> QVariant {
    // unsafe{_ZN8QVariantC2ERK11QJsonObject()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN8QVariantC2ERK11QJsonObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QVariant{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qvariant.h:258
// index:40
// Public Visibility=Default Availability=Available
// [-2] void QVariant(const QJsonArray &)

/*
Constructs an invalid variant.
*/
// QVariant(const QJsonArray &) ctx.fn_proto_cpp
impl /*struct*/ QVariant {
  pub fn QVariant_40<T: QVariant_QVariant_40>(value: T) -> QVariant {
    let rsthis = value.QVariant_40();
    return rsthis;
    // return 1;
  }
}

pub trait QVariant_QVariant_40 {
  fn QVariant_40(self) -> QVariant;
}
// QVariant(const QJsonArray &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QVariant_QVariant_40 for (usize) {
  fn QVariant_40(self) -> QVariant {
    // unsafe{_ZN8QVariantC2ERK10QJsonArray()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN8QVariantC2ERK10QJsonArray", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QVariant{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qvariant.h:259
// index:41
// Public Visibility=Default Availability=Available
// [-2] void QVariant(const QJsonDocument &)

/*
Constructs an invalid variant.
*/
// QVariant(const QJsonDocument &) ctx.fn_proto_cpp
impl /*struct*/ QVariant {
  pub fn QVariant_41<T: QVariant_QVariant_41>(value: T) -> QVariant {
    let rsthis = value.QVariant_41();
    return rsthis;
    // return 1;
  }
}

pub trait QVariant_QVariant_41 {
  fn QVariant_41(self) -> QVariant;
}
// QVariant(const QJsonDocument &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QVariant_QVariant_41 for (usize) {
  fn QVariant_41(self) -> QVariant {
    // unsafe{_ZN8QVariantC2ERK13QJsonDocument()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN8QVariantC2ERK13QJsonDocument", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QVariant{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qvariant.h:200
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ~QVariant()

/*

*/
pub fn DeleteQVariant(this :*mut QVariant) {
    // let rv = qtrt::InvokeQtFunc6("_ZN8QVariantD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qvariant.h:262
// index:0
// Public Visibility=Default Availability=Available
// [16] QVariant & operator=(const QVariant &)

/*

*/
impl /*struct*/ QVariant {
  pub fn operator_equal_0<RetType, T: QVariant_operator_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_0(self);
    // return 1;
  }
}
pub trait QVariant_operator_equal_0<RetType> {
  fn operator_equal_0(self , rsthis: & QVariant) -> RetType;
}
impl<'a> /*trait*/ QVariant_operator_equal_0<usize> for (usize) {
  fn operator_equal_0(self , rsthis: & QVariant) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QVariantaSERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qvariant.h:266
// index:1
// Public inline Visibility=Default Availability=Available
// [16] QVariant & operator=(QVariant &&)

/*

*/
impl /*struct*/ QVariant {
  pub fn operator_equal_1<RetType, T: QVariant_operator_equal_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_1(self);
    // return 1;
  }
}
pub trait QVariant_operator_equal_1<RetType> {
  fn operator_equal_1(self , rsthis: & QVariant) -> RetType;
}
impl<'a> /*trait*/ QVariant_operator_equal_1<usize> for (usize) {
  fn operator_equal_1(self , rsthis: & QVariant) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QVariantaSEOS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qvariant.h:270
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void swap(QVariant &)

/*
Swaps variant other with this variant. This operation is very fast and never fails.

This function was introduced in  Qt 4.8.
*/
impl /*struct*/ QVariant {
  pub fn swap_0<RetType, T: QVariant_swap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.swap_0(self);
    // return 1;
  }
}
pub trait QVariant_swap_0<RetType> {
  fn swap_0(self , rsthis: & QVariant) -> RetType;
}
impl<'a> /*trait*/ QVariant_swap_0<(/*void*/)> for (usize) {
  fn swap_0(self , rsthis: & QVariant) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QVariant4swapERS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qvariant.h:272
// index:0
// Public Visibility=Default Availability=Available
// [4] QVariant::Type type() const

/*
Returns the storage type of the value stored in the variant. Although this function is declared as returning QVariant::Type, the return value should be interpreted as QMetaType::Type. In particular, QVariant::UserType is returned here only if the value is equal or greater than QMetaType::User.

Note that return values in the ranges QVariant::Char through QVariant::RegExp and QVariant::Font through QVariant::Transform correspond to the values in the ranges QMetaType::QChar through QMetaType::QRegExp and QMetaType::QFont through QMetaType::QQuaternion.

Pay particular attention when working with char and QChar variants. Note that there is no QVariant constructor specifically for type char, but there is one for QChar. For a variant of type QChar, this function returns QVariant::Char, which is the same as QMetaType::QChar, but for a variant of type char, this function returns QMetaType::Char, which is not the same as QVariant::Char.

Also note that the types void*, long, short, unsigned long, unsigned short, unsigned char, float, QObject*, and QWidget* are represented in QMetaType::Type but not in QVariant::Type, and they can be returned by this function. However, they are considered to be user defined types when tested against QVariant::Type.

To test whether an instance of QVariant contains a data type that is compatible with the data type you are interested in, use canConvert().
*/
impl /*struct*/ QVariant {
  pub fn type__0<RetType, T: QVariant_type__0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.type__0(self);
    // return 1;
  }
}
pub trait QVariant_type__0<RetType> {
  fn type__0(self , rsthis: & QVariant) -> RetType;
}
impl<'a> /*trait*/ QVariant_type__0<i32> for () {
  fn type__0(self , rsthis: & QVariant) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QVariant4typeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qvariant.h:273
// index:0
// Public Visibility=Default Availability=Available
// [4] int userType() const

/*
Returns the storage type of the value stored in the variant. For non-user types, this is the same as type().

See also type().
*/
impl /*struct*/ QVariant {
  pub fn userType_0<RetType, T: QVariant_userType_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.userType_0(self);
    // return 1;
  }
}
pub trait QVariant_userType_0<RetType> {
  fn userType_0(self , rsthis: & QVariant) -> RetType;
}
impl<'a> /*trait*/ QVariant_userType_0<i32> for () {
  fn userType_0(self , rsthis: & QVariant) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QVariant8userTypeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qvariant.h:274
// index:0
// Public Visibility=Default Availability=Available
// [8] const char * typeName() const

/*
Returns the name of the type stored in the variant. The returned strings describe the C++ datatype used to store the data: for example, "QFont", "QString", or "QVariantList". An Invalid variant returns 0.
*/
impl /*struct*/ QVariant {
  pub fn typeName_0<RetType, T: QVariant_typeName_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.typeName_0(self);
    // return 1;
  }
}
pub trait QVariant_typeName_0<RetType> {
  fn typeName_0(self , rsthis: & QVariant) -> RetType;
}
impl<'a> /*trait*/ QVariant_typeName_0<usize> for () {
  fn typeName_0(self , rsthis: & QVariant) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QVariant8typeNameEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qvariant.h:276
// index:0
// Public Visibility=Default Availability=Available
// [1] bool canConvert(int) const

/*
Returns true if the variant's type can be cast to the requested type, targetTypeId. Such casting is done automatically when calling the toInt(), toBool(), ... methods.

The following casts are done automatically:


 TypeAutomatically Cast To
QMetaType::BoolQMetaType::QChar, QMetaType::Double, QMetaType::Int, QMetaType::LongLong, QMetaType::QString, QMetaType::UInt, QMetaType::ULongLong
QMetaType::QByteArrayQMetaType::Double, QMetaType::Int, QMetaType::LongLong, QMetaType::QString, QMetaType::UInt, QMetaType::ULongLong, QMetaType::QUuid
QMetaType::QCharQMetaType::Bool, QMetaType::Int, QMetaType::UInt, QMetaType::LongLong, QMetaType::ULongLong
QMetaType::QColorQMetaType::QString
QMetaType::QDateQMetaType::QDateTime, QMetaType::QString
QMetaType::QDateTimeQMetaType::QDate, QMetaType::QString, QMetaType::QTime
QMetaType::DoubleQMetaType::Bool, QMetaType::Int, QMetaType::LongLong, QMetaType::QString, QMetaType::UInt, QMetaType::ULongLong
QMetaType::QFontQMetaType::QString
QMetaType::IntQMetaType::Bool, QMetaType::QChar, QMetaType::Double, QMetaType::LongLong, QMetaType::QString, QMetaType::UInt, QMetaType::ULongLong
QMetaType::QKeySequenceQMetaType::Int, QMetaType::QString
QMetaType::QVariantListQMetaType::QStringList (if the list's items can be converted to QStrings)
QMetaType::LongLongQMetaType::Bool, QMetaType::QByteArray, QMetaType::QChar, QMetaType::Double, QMetaType::Int, QMetaType::QString, QMetaType::UInt, QMetaType::ULongLong
QMetaType::QPointQMetaType::QPointF
QMetaType::QRectQMetaType::QRectF
QMetaType::QStringQMetaType::Bool, QMetaType::QByteArray, QMetaType::QChar, QMetaType::QColor, QMetaType::QDate, QMetaType::QDateTime, QMetaType::Double, QMetaType::QFont, QMetaType::Int, QMetaType::QKeySequence, QMetaType::LongLong, QMetaType::QStringList, QMetaType::QTime, QMetaType::UInt, QMetaType::ULongLong, QMetaType::QUuid
QMetaType::QStringListQMetaType::QVariantList, QMetaType::QString (if the list contains exactly one item)
QMetaType::QTimeQMetaType::QString
QMetaType::UIntQMetaType::Bool, QMetaType::QChar, QMetaType::Double, QMetaType::Int, QMetaType::LongLong, QMetaType::QString, QMetaType::ULongLong
QMetaType::ULongLongQMetaType::Bool, QMetaType::QChar, QMetaType::Double, QMetaType::Int, QMetaType::LongLong, QMetaType::QString, QMetaType::UInt
QMetaType::QUuidQMetaType::QByteArray, QMetaType::QString


A QVariant containing a pointer to a type derived from QObject will also return true for this function if a qobject_cast to the type described by targetTypeId would succeed. Note that this only works for QObject subclasses which use the Q_OBJECT macro.

A QVariant containing a sequential container will also return true for this function if the targetTypeId is QVariantList. It is possible to iterate over the contents of the container without extracting it as a (copied) QVariantList:


  QList<int> intList = {7, 11, 42};

  QVariant variant = QVariant::fromValue(intList);
  if (variant.canConvert<QVariantList>()) {
      QSequentialIterable iterable = variant.value<QSequentialIterable>();
      // Can use foreach:
      foreach (const QVariant &v, iterable) {
          qDebug() << v;
      }
      // Can use C++11 range-for:
      for (const QVariant &v : iterable) {
          qDebug() << v;
      }
      // Can use iterators:
      QSequentialIterable::const_iterator it = iterable.begin();
      const QSequentialIterable::const_iterator end = iterable.end();
      for ( ; it != end; ++it) {
          qDebug() << *it;
      }
  }



This requires that the value_type of the container is itself a metatype.

Similarly, a QVariant containing a sequential container will also return true for this function the targetTypeId is QVariantHash or QVariantMap. It is possible to iterate over the contents of the container without extracting it as a (copied) QVariantHash or QVariantMap:


  QHash<int, QString> mapping;
  mapping.insert(7, "Seven");
  mapping.insert(11, "Eleven");
  mapping.insert(42, "Forty-two");

  QVariant variant = QVariant::fromValue(mapping);
  if (variant.canConvert<QVariantHash>()) {
      QAssociativeIterable iterable = variant.value<QAssociativeIterable>();
      // Can use foreach over the values:
      foreach (const QVariant &v, iterable) {
          qDebug() << v;
      }
      // Can use C++11 range-for over the values:
      for (const QVariant &v : iterable) {
          qDebug() << v;
      }
      // Can use iterators:
      QAssociativeIterable::const_iterator it = iterable.begin();
      const QAssociativeIterable::const_iterator end = iterable.end();
      for ( ; it != end; ++it) {
          qDebug() << *it; // The current value
          qDebug() << it.key();
          qDebug() << it.value();
      }
  }



See also convert(), QSequentialIterable, Q_DECLARE_SEQUENTIAL_CONTAINER_METATYPE(), QAssociativeIterable, and Q_DECLARE_ASSOCIATIVE_CONTAINER_METATYPE().
*/
impl /*struct*/ QVariant {
  pub fn canConvert_0<RetType, T: QVariant_canConvert_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.canConvert_0(self);
    // return 1;
  }
}
pub trait QVariant_canConvert_0<RetType> {
  fn canConvert_0(self , rsthis: & QVariant) -> RetType;
}
impl<'a> /*trait*/ QVariant_canConvert_0<bool> for (i32) {
  fn canConvert_0(self , rsthis: & QVariant) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QVariant10canConvertEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qvariant.h:277
// index:0
// Public Visibility=Default Availability=Available
// [1] bool convert(int)

/*
Casts the variant to the requested type, targetTypeId. If the cast cannot be done, the variant is still changed to the requested type, but is left in a cleared null state similar to that constructed by QVariant(Type).

Returns true if the current type of the variant was successfully cast; otherwise returns false.

A QVariant containing a pointer to a type derived from QObject will also convert and return true for this function if a qobject_cast to the type described by targetTypeId would succeed. Note that this only works for QObject subclasses which use the Q_OBJECT macro.

Note: converting QVariants that are null due to not being initialized or having failed a previous conversion will always fail, changing the type, remaining null, and returning false.

See also canConvert() and clear().
*/
impl /*struct*/ QVariant {
  pub fn convert_0<RetType, T: QVariant_convert_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.convert_0(self);
    // return 1;
  }
}
pub trait QVariant_convert_0<RetType> {
  fn convert_0(self , rsthis: & QVariant) -> RetType;
}
impl<'a> /*trait*/ QVariant_convert_0<bool> for (i32) {
  fn convert_0(self , rsthis: & QVariant) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QVariant7convertEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qvariant.h:467
// index:1
// Protected Visibility=Default Availability=Available
// [1] bool convert(const int, void *) const

/*
Casts the variant to the requested type, targetTypeId. If the cast cannot be done, the variant is still changed to the requested type, but is left in a cleared null state similar to that constructed by QVariant(Type).

Returns true if the current type of the variant was successfully cast; otherwise returns false.

A QVariant containing a pointer to a type derived from QObject will also convert and return true for this function if a qobject_cast to the type described by targetTypeId would succeed. Note that this only works for QObject subclasses which use the Q_OBJECT macro.

Note: converting QVariants that are null due to not being initialized or having failed a previous conversion will always fail, changing the type, remaining null, and returning false.

See also canConvert() and clear().
*/
impl /*struct*/ QVariant {
  pub fn convert_1<RetType, T: QVariant_convert_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.convert_1(self);
    // return 1;
  }
}
pub trait QVariant_convert_1<RetType> {
  fn convert_1(self , rsthis: & QVariant) -> RetType;
}
impl<'a> /*trait*/ QVariant_convert_1<bool> for (i32,usize) {
  fn convert_1(self , rsthis: & QVariant) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QVariant7convertEiPv", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qvariant.h:279
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isValid() const

/*
Returns true if the storage type of this variant is not QMetaType::UnknownType; otherwise returns false.
*/
impl /*struct*/ QVariant {
  pub fn isValid_0<RetType, T: QVariant_isValid_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isValid_0(self);
    // return 1;
  }
}
pub trait QVariant_isValid_0<RetType> {
  fn isValid_0(self , rsthis: & QVariant) -> RetType;
}
impl<'a> /*trait*/ QVariant_isValid_0<bool> for () {
  fn isValid_0(self , rsthis: & QVariant) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QVariant7isValidEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qvariant.h:280
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isNull() const

/*
Returns true if this is a null variant, false otherwise. A variant is considered null if it contains no initialized value, or the contained value is a null pointer or is an instance of a built-in type that has an isNull method, in which case the result would be the same as calling isNull on the wrapped object.

Warning: Null variants is not a single state and two null variants may easily return false on the == operator if they do not contain similar null values.

See also QVariant(Type) and convert(int).
*/
impl /*struct*/ QVariant {
  pub fn isNull_0<RetType, T: QVariant_isNull_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isNull_0(self);
    // return 1;
  }
}
pub trait QVariant_isNull_0<RetType> {
  fn isNull_0(self , rsthis: & QVariant) -> RetType;
}
impl<'a> /*trait*/ QVariant_isNull_0<bool> for () {
  fn isNull_0(self , rsthis: & QVariant) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QVariant6isNullEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qvariant.h:282
// index:0
// Public Visibility=Default Availability=Available
// [-2] void clear()

/*
Convert this variant to type QMetaType::UnknownType and free up any resources used.
*/
impl /*struct*/ QVariant {
  pub fn clear_0<RetType, T: QVariant_clear_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.clear_0(self);
    // return 1;
  }
}
pub trait QVariant_clear_0<RetType> {
  fn clear_0(self , rsthis: & QVariant) -> RetType;
}
impl<'a> /*trait*/ QVariant_clear_0<(/*void*/)> for () {
  fn clear_0(self , rsthis: & QVariant) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN8QVariant5clearEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qvariant.h:284
// index:0
// Public Visibility=Default Availability=Available
// [-2] void detach()

/*

*/
impl /*struct*/ QVariant {
  pub fn detach_0<RetType, T: QVariant_detach_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.detach_0(self);
    // return 1;
  }
}
pub trait QVariant_detach_0<RetType> {
  fn detach_0(self , rsthis: & QVariant) -> RetType;
}
impl<'a> /*trait*/ QVariant_detach_0<(/*void*/)> for () {
  fn detach_0(self , rsthis: & QVariant) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN8QVariant6detachEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qvariant.h:285
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isDetached() const

/*

*/
impl /*struct*/ QVariant {
  pub fn isDetached_0<RetType, T: QVariant_isDetached_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isDetached_0(self);
    // return 1;
  }
}
pub trait QVariant_isDetached_0<RetType> {
  fn isDetached_0(self , rsthis: & QVariant) -> RetType;
}
impl<'a> /*trait*/ QVariant_isDetached_0<bool> for () {
  fn isDetached_0(self , rsthis: & QVariant) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QVariant10isDetachedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qvariant.h:287
// index:0
// Public Visibility=Default Availability=Available
// [4] int toInt(bool *) const

/*
Returns the variant as an int if the variant has userType() QMetaType::Int, QMetaType::Bool, QMetaType::QByteArray, QMetaType::QChar, QMetaType::Double, QMetaType::LongLong, QMetaType::QString, QMetaType::UInt, or QMetaType::ULongLong; otherwise returns 0.

If ok is non-null: *ok is set to true if the value could be converted to an int; otherwise *ok is set to false.

Warning: If the value is convertible to a QMetaType::LongLong but is too large to be represented in an int, the resulting arithmetic overflow will not be reflected in ok. A simple workaround is to use QString::toInt().

See also canConvert() and convert().
*/
impl /*struct*/ QVariant {
  pub fn toInt_0<RetType, T: QVariant_toInt_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toInt_0(self);
    // return 1;
  }
}
pub trait QVariant_toInt_0<RetType> {
  fn toInt_0(self , rsthis: & QVariant) -> RetType;
}
impl<'a> /*trait*/ QVariant_toInt_0<i32> for (usize) {
  fn toInt_0(self , rsthis: & QVariant) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QVariant5toIntEPb", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qvariant.h:288
// index:0
// Public Visibility=Default Availability=Available
// [4] uint toUInt(bool *) const

/*
Returns the variant as an unsigned int if the variant has userType() QMetaType::UInt, QMetaType::Bool, QMetaType::QByteArray, QMetaType::QChar, QMetaType::Double, QMetaType::Int, QMetaType::LongLong, QMetaType::QString, or QMetaType::ULongLong; otherwise returns 0.

If ok is non-null: *ok is set to true if the value could be converted to an unsigned int; otherwise *ok is set to false.

Warning: If the value is convertible to a QMetaType::ULongLong but is too large to be represented in an unsigned int, the resulting arithmetic overflow will not be reflected in ok. A simple workaround is to use QString::toUInt().

See also canConvert() and convert().
*/
impl /*struct*/ QVariant {
  pub fn toUInt_0<RetType, T: QVariant_toUInt_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toUInt_0(self);
    // return 1;
  }
}
pub trait QVariant_toUInt_0<RetType> {
  fn toUInt_0(self , rsthis: & QVariant) -> RetType;
}
impl<'a> /*trait*/ QVariant_toUInt_0<u32> for (usize) {
  fn toUInt_0(self , rsthis: & QVariant) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QVariant6toUIntEPb", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qvariant.h:289
// index:0
// Public Visibility=Default Availability=Available
// [8] qlonglong toLongLong(bool *) const

/*
Returns the variant as a long long int if the variant has userType() QMetaType::LongLong, QMetaType::Bool, QMetaType::QByteArray, QMetaType::QChar, QMetaType::Double, QMetaType::Int, QMetaType::QString, QMetaType::UInt, or QMetaType::ULongLong; otherwise returns 0.

If ok is non-null: *ok is set to true if the value could be converted to an int; otherwise *ok is set to false.

See also canConvert() and convert().
*/
impl /*struct*/ QVariant {
  pub fn toLongLong_0<RetType, T: QVariant_toLongLong_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toLongLong_0(self);
    // return 1;
  }
}
pub trait QVariant_toLongLong_0<RetType> {
  fn toLongLong_0(self , rsthis: & QVariant) -> RetType;
}
impl<'a> /*trait*/ QVariant_toLongLong_0<i64> for (usize) {
  fn toLongLong_0(self , rsthis: & QVariant) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QVariant10toLongLongEPb", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qvariant.h:290
// index:0
// Public Visibility=Default Availability=Available
// [8] qulonglong toULongLong(bool *) const

/*
Returns the variant as an unsigned long long int if the variant has type() QMetaType::ULongLong, QMetaType::Bool, QMetaType::QByteArray, QMetaType::QChar, QMetaType::Double, QMetaType::Int, QMetaType::LongLong, QMetaType::QString, or QMetaType::UInt; otherwise returns 0.

If ok is non-null: *ok is set to true if the value could be converted to an int; otherwise *ok is set to false.

See also canConvert() and convert().
*/
impl /*struct*/ QVariant {
  pub fn toULongLong_0<RetType, T: QVariant_toULongLong_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toULongLong_0(self);
    // return 1;
  }
}
pub trait QVariant_toULongLong_0<RetType> {
  fn toULongLong_0(self , rsthis: & QVariant) -> RetType;
}
impl<'a> /*trait*/ QVariant_toULongLong_0<u64> for (usize) {
  fn toULongLong_0(self , rsthis: & QVariant) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QVariant11toULongLongEPb", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qvariant.h:291
// index:0
// Public Visibility=Default Availability=Available
// [1] bool toBool() const

/*
Returns the variant as a bool if the variant has userType() Bool.

Returns true if the variant has userType() QMetaType::Bool, QMetaType::QChar, QMetaType::Double, QMetaType::Int, QMetaType::LongLong, QMetaType::UInt, or QMetaType::ULongLong and the value is non-zero, or if the variant has type QMetaType::QString or QMetaType::QByteArray and its lower-case content is not one of the following: empty, "0" or "false"; otherwise returns false.

See also canConvert() and convert().
*/
impl /*struct*/ QVariant {
  pub fn toBool_0<RetType, T: QVariant_toBool_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toBool_0(self);
    // return 1;
  }
}
pub trait QVariant_toBool_0<RetType> {
  fn toBool_0(self , rsthis: & QVariant) -> RetType;
}
impl<'a> /*trait*/ QVariant_toBool_0<bool> for () {
  fn toBool_0(self , rsthis: & QVariant) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QVariant6toBoolEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qvariant.h:292
// index:0
// Public Visibility=Default Availability=Available
// [8] double toDouble(bool *) const

/*
Returns the variant as a double if the variant has userType() QMetaType::Double, QMetaType::Float, QMetaType::Bool, QMetaType::QByteArray, QMetaType::Int, QMetaType::LongLong, QMetaType::QString, QMetaType::UInt, or QMetaType::ULongLong; otherwise returns 0.0.

If ok is non-null: *ok is set to true if the value could be converted to a double; otherwise *ok is set to false.

See also canConvert() and convert().
*/
impl /*struct*/ QVariant {
  pub fn toDouble_0<RetType, T: QVariant_toDouble_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toDouble_0(self);
    // return 1;
  }
}
pub trait QVariant_toDouble_0<RetType> {
  fn toDouble_0(self , rsthis: & QVariant) -> RetType;
}
impl<'a> /*trait*/ QVariant_toDouble_0<f64> for (usize) {
  fn toDouble_0(self , rsthis: & QVariant) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QVariant8toDoubleEPb", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qvariant.h:293
// index:0
// Public Visibility=Default Availability=Available
// [4] float toFloat(bool *) const

/*
Returns the variant as a float if the variant has userType() QMetaType::Double, QMetaType::Float, QMetaType::Bool, QMetaType::QByteArray, QMetaType::Int, QMetaType::LongLong, QMetaType::QString, QMetaType::UInt, or QMetaType::ULongLong; otherwise returns 0.0.

If ok is non-null: *ok is set to true if the value could be converted to a double; otherwise *ok is set to false.

This function was introduced in  Qt 4.6.

See also canConvert() and convert().
*/
impl /*struct*/ QVariant {
  pub fn toFloat_0<RetType, T: QVariant_toFloat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toFloat_0(self);
    // return 1;
  }
}
pub trait QVariant_toFloat_0<RetType> {
  fn toFloat_0(self , rsthis: & QVariant) -> RetType;
}
impl<'a> /*trait*/ QVariant_toFloat_0<f32> for (usize) {
  fn toFloat_0(self , rsthis: & QVariant) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QVariant7toFloatEPb", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qvariant.h:294
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal toReal(bool *) const

/*
Returns the variant as a qreal if the variant has userType() QMetaType::Double, QMetaType::Float, QMetaType::Bool, QMetaType::QByteArray, QMetaType::Int, QMetaType::LongLong, QMetaType::QString, QMetaType::UInt, or QMetaType::ULongLong; otherwise returns 0.0.

If ok is non-null: *ok is set to true if the value could be converted to a double; otherwise *ok is set to false.

This function was introduced in  Qt 4.6.

See also canConvert() and convert().
*/
impl /*struct*/ QVariant {
  pub fn toReal_0<RetType, T: QVariant_toReal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toReal_0(self);
    // return 1;
  }
}
pub trait QVariant_toReal_0<RetType> {
  fn toReal_0(self , rsthis: & QVariant) -> RetType;
}
impl<'a> /*trait*/ QVariant_toReal_0<f64> for (usize) {
  fn toReal_0(self , rsthis: & QVariant) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QVariant6toRealEPb", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qvariant.h:295
// index:0
// Public Visibility=Default Availability=Available
// [8] QByteArray toByteArray() const

/*
Returns the variant as a QByteArray if the variant has userType() QMetaType::QByteArray or QMetaType::QString (converted using QString::fromUtf8()); otherwise returns an empty byte array.

See also canConvert() and convert().
*/
impl /*struct*/ QVariant {
  pub fn toByteArray_0<RetType, T: QVariant_toByteArray_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toByteArray_0(self);
    // return 1;
  }
}
pub trait QVariant_toByteArray_0<RetType> {
  fn toByteArray_0(self , rsthis: & QVariant) -> RetType;
}
impl<'a> /*trait*/ QVariant_toByteArray_0<usize> for () {
  fn toByteArray_0(self , rsthis: & QVariant) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QVariant11toByteArrayEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qvariant.h:296
// index:0
// Public Visibility=Default Availability=Available
// [8] QBitArray toBitArray() const

/*
Returns the variant as a QBitArray if the variant has userType() QMetaType::QBitArray; otherwise returns an empty bit array.

See also canConvert() and convert().
*/
impl /*struct*/ QVariant {
  pub fn toBitArray_0<RetType, T: QVariant_toBitArray_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toBitArray_0(self);
    // return 1;
  }
}
pub trait QVariant_toBitArray_0<RetType> {
  fn toBitArray_0(self , rsthis: & QVariant) -> RetType;
}
impl<'a> /*trait*/ QVariant_toBitArray_0<usize> for () {
  fn toBitArray_0(self , rsthis: & QVariant) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QVariant10toBitArrayEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qvariant.h:297
// index:0
// Public Visibility=Default Availability=Available
// [8] QString toString() const

/*
Returns the variant as a QString if the variant has userType() QMetaType::QString, QMetaType::Bool, QMetaType::QByteArray, QMetaType::QChar, QMetaType::QDate, QMetaType::QDateTime, QMetaType::Double, QMetaType::Int, QMetaType::LongLong, QMetaType::QStringList, QMetaType::QTime, QMetaType::UInt, or QMetaType::ULongLong; otherwise returns an empty string.

See also canConvert() and convert().
*/
impl /*struct*/ QVariant {
  pub fn toString_0<RetType, T: QVariant_toString_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toString_0(self);
    // return 1;
  }
}
pub trait QVariant_toString_0<RetType> {
  fn toString_0(self , rsthis: & QVariant) -> RetType;
}
impl<'a> /*trait*/ QVariant_toString_0<usize> for () {
  fn toString_0(self , rsthis: & QVariant) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QVariant8toStringEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qvariant.h:298
// index:0
// Public Visibility=Default Availability=Available
// [8] QStringList toStringList() const

/*
Returns the variant as a QStringList if the variant has userType() QMetaType::QStringList, QMetaType::QString, or QMetaType::QVariantList of a type that can be converted to QString; otherwise returns an empty list.

See also canConvert() and convert().
*/
impl /*struct*/ QVariant {
  pub fn toStringList_0<RetType, T: QVariant_toStringList_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toStringList_0(self);
    // return 1;
  }
}
pub trait QVariant_toStringList_0<RetType> {
  fn toStringList_0(self , rsthis: & QVariant) -> RetType;
}
impl<'a> /*trait*/ QVariant_toStringList_0<usize> for () {
  fn toStringList_0(self , rsthis: & QVariant) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QVariant12toStringListEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qvariant.h:299
// index:0
// Public Visibility=Default Availability=Available
// [2] QChar toChar() const

/*
Returns the variant as a QChar if the variant has userType() QMetaType::QChar, QMetaType::Int, or QMetaType::UInt; otherwise returns an invalid QChar.

See also canConvert() and convert().
*/
impl /*struct*/ QVariant {
  pub fn toChar_0<RetType, T: QVariant_toChar_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toChar_0(self);
    // return 1;
  }
}
pub trait QVariant_toChar_0<RetType> {
  fn toChar_0(self , rsthis: & QVariant) -> RetType;
}
impl<'a> /*trait*/ QVariant_toChar_0<usize> for () {
  fn toChar_0(self , rsthis: & QVariant) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QVariant6toCharEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qvariant.h:300
// index:0
// Public Visibility=Default Availability=Available
// [8] QDate toDate() const

/*
Returns the variant as a QDate if the variant has userType() QMetaType::QDate, QMetaType::QDateTime, or QMetaType::QString; otherwise returns an invalid date.

If the type() is QMetaType::QString, an invalid date will be returned if the string cannot be parsed as a Qt::ISODate format date.

See also canConvert() and convert().
*/
impl /*struct*/ QVariant {
  pub fn toDate_0<RetType, T: QVariant_toDate_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toDate_0(self);
    // return 1;
  }
}
pub trait QVariant_toDate_0<RetType> {
  fn toDate_0(self , rsthis: & QVariant) -> RetType;
}
impl<'a> /*trait*/ QVariant_toDate_0<usize> for () {
  fn toDate_0(self , rsthis: & QVariant) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QVariant6toDateEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qvariant.h:301
// index:0
// Public Visibility=Default Availability=Available
// [4] QTime toTime() const

/*
Returns the variant as a QTime if the variant has userType() QMetaType::QTime, QMetaType::QDateTime, or QMetaType::QString; otherwise returns an invalid time.

If the type() is QMetaType::QString, an invalid time will be returned if the string cannot be parsed as a Qt::ISODate format time.

See also canConvert() and convert().
*/
impl /*struct*/ QVariant {
  pub fn toTime_0<RetType, T: QVariant_toTime_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toTime_0(self);
    // return 1;
  }
}
pub trait QVariant_toTime_0<RetType> {
  fn toTime_0(self , rsthis: & QVariant) -> RetType;
}
impl<'a> /*trait*/ QVariant_toTime_0<usize> for () {
  fn toTime_0(self , rsthis: & QVariant) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QVariant6toTimeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qvariant.h:302
// index:0
// Public Visibility=Default Availability=Available
// [8] QDateTime toDateTime() const

/*
Returns the variant as a QDateTime if the variant has userType() QMetaType::QDateTime, QMetaType::QDate, or QMetaType::QString; otherwise returns an invalid date/time.

If the type() is QMetaType::QString, an invalid date/time will be returned if the string cannot be parsed as a Qt::ISODate format date/time.

See also canConvert() and convert().
*/
impl /*struct*/ QVariant {
  pub fn toDateTime_0<RetType, T: QVariant_toDateTime_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toDateTime_0(self);
    // return 1;
  }
}
pub trait QVariant_toDateTime_0<RetType> {
  fn toDateTime_0(self , rsthis: & QVariant) -> RetType;
}
impl<'a> /*trait*/ QVariant_toDateTime_0<usize> for () {
  fn toDateTime_0(self , rsthis: & QVariant) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QVariant10toDateTimeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qvariant.h:308
// index:0
// Public Visibility=Default Availability=Available
// [8] QPoint toPoint() const

/*
Returns the variant as a QPoint if the variant has userType() QMetaType::QPoint or QMetaType::QPointF; otherwise returns a null QPoint.

See also canConvert() and convert().
*/
impl /*struct*/ QVariant {
  pub fn toPoint_0<RetType, T: QVariant_toPoint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toPoint_0(self);
    // return 1;
  }
}
pub trait QVariant_toPoint_0<RetType> {
  fn toPoint_0(self , rsthis: & QVariant) -> RetType;
}
impl<'a> /*trait*/ QVariant_toPoint_0<usize> for () {
  fn toPoint_0(self , rsthis: & QVariant) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QVariant7toPointEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qvariant.h:309
// index:0
// Public Visibility=Default Availability=Available
// [16] QPointF toPointF() const

/*
Returns the variant as a QPointF if the variant has userType() QMetaType::QPoint or QMetaType::QPointF; otherwise returns a null QPointF.

See also canConvert() and convert().
*/
impl /*struct*/ QVariant {
  pub fn toPointF_0<RetType, T: QVariant_toPointF_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toPointF_0(self);
    // return 1;
  }
}
pub trait QVariant_toPointF_0<RetType> {
  fn toPointF_0(self , rsthis: & QVariant) -> RetType;
}
impl<'a> /*trait*/ QVariant_toPointF_0<usize> for () {
  fn toPointF_0(self , rsthis: & QVariant) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QVariant8toPointFEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qvariant.h:310
// index:0
// Public Visibility=Default Availability=Available
// [16] QRect toRect() const

/*
Returns the variant as a QRect if the variant has userType() QMetaType::QRect; otherwise returns an invalid QRect.

See also canConvert() and convert().
*/
impl /*struct*/ QVariant {
  pub fn toRect_0<RetType, T: QVariant_toRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toRect_0(self);
    // return 1;
  }
}
pub trait QVariant_toRect_0<RetType> {
  fn toRect_0(self , rsthis: & QVariant) -> RetType;
}
impl<'a> /*trait*/ QVariant_toRect_0<usize> for () {
  fn toRect_0(self , rsthis: & QVariant) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QVariant6toRectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qvariant.h:311
// index:0
// Public Visibility=Default Availability=Available
// [8] QSize toSize() const

/*
Returns the variant as a QSize if the variant has userType() QMetaType::QSize; otherwise returns an invalid QSize.

See also canConvert() and convert().
*/
impl /*struct*/ QVariant {
  pub fn toSize_0<RetType, T: QVariant_toSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toSize_0(self);
    // return 1;
  }
}
pub trait QVariant_toSize_0<RetType> {
  fn toSize_0(self , rsthis: & QVariant) -> RetType;
}
impl<'a> /*trait*/ QVariant_toSize_0<usize> for () {
  fn toSize_0(self , rsthis: & QVariant) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QVariant6toSizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qvariant.h:312
// index:0
// Public Visibility=Default Availability=Available
// [16] QSizeF toSizeF() const

/*
Returns the variant as a QSizeF if the variant has userType() QMetaType::QSizeF; otherwise returns an invalid QSizeF.

See also canConvert() and convert().
*/
impl /*struct*/ QVariant {
  pub fn toSizeF_0<RetType, T: QVariant_toSizeF_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toSizeF_0(self);
    // return 1;
  }
}
pub trait QVariant_toSizeF_0<RetType> {
  fn toSizeF_0(self , rsthis: & QVariant) -> RetType;
}
impl<'a> /*trait*/ QVariant_toSizeF_0<usize> for () {
  fn toSizeF_0(self , rsthis: & QVariant) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QVariant7toSizeFEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qvariant.h:313
// index:0
// Public Visibility=Default Availability=Available
// [16] QLine toLine() const

/*
Returns the variant as a QLine if the variant has userType() QMetaType::QLine; otherwise returns an invalid QLine.

See also canConvert() and convert().
*/
impl /*struct*/ QVariant {
  pub fn toLine_0<RetType, T: QVariant_toLine_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toLine_0(self);
    // return 1;
  }
}
pub trait QVariant_toLine_0<RetType> {
  fn toLine_0(self , rsthis: & QVariant) -> RetType;
}
impl<'a> /*trait*/ QVariant_toLine_0<usize> for () {
  fn toLine_0(self , rsthis: & QVariant) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QVariant6toLineEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qvariant.h:314
// index:0
// Public Visibility=Default Availability=Available
// [32] QLineF toLineF() const

/*
Returns the variant as a QLineF if the variant has userType() QMetaType::QLineF; otherwise returns an invalid QLineF.

See also canConvert() and convert().
*/
impl /*struct*/ QVariant {
  pub fn toLineF_0<RetType, T: QVariant_toLineF_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toLineF_0(self);
    // return 1;
  }
}
pub trait QVariant_toLineF_0<RetType> {
  fn toLineF_0(self , rsthis: & QVariant) -> RetType;
}
impl<'a> /*trait*/ QVariant_toLineF_0<usize> for () {
  fn toLineF_0(self , rsthis: & QVariant) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QVariant7toLineFEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qvariant.h:315
// index:0
// Public Visibility=Default Availability=Available
// [32] QRectF toRectF() const

/*
Returns the variant as a QRectF if the variant has userType() QMetaType::QRect or QMetaType::QRectF; otherwise returns an invalid QRectF.

See also canConvert() and convert().
*/
impl /*struct*/ QVariant {
  pub fn toRectF_0<RetType, T: QVariant_toRectF_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toRectF_0(self);
    // return 1;
  }
}
pub trait QVariant_toRectF_0<RetType> {
  fn toRectF_0(self , rsthis: & QVariant) -> RetType;
}
impl<'a> /*trait*/ QVariant_toRectF_0<usize> for () {
  fn toRectF_0(self , rsthis: & QVariant) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QVariant7toRectFEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qvariant.h:317
// index:0
// Public Visibility=Default Availability=Available
// [8] QLocale toLocale() const

/*
Returns the variant as a QLocale if the variant has userType() QMetaType::QLocale; otherwise returns an invalid QLocale.

See also canConvert() and convert().
*/
impl /*struct*/ QVariant {
  pub fn toLocale_0<RetType, T: QVariant_toLocale_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toLocale_0(self);
    // return 1;
  }
}
pub trait QVariant_toLocale_0<RetType> {
  fn toLocale_0(self , rsthis: & QVariant) -> RetType;
}
impl<'a> /*trait*/ QVariant_toLocale_0<usize> for () {
  fn toLocale_0(self , rsthis: & QVariant) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QVariant8toLocaleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qvariant.h:319
// index:0
// Public Visibility=Default Availability=Available
// [8] QRegExp toRegExp() const

/*
Returns the variant as a QRegExp if the variant has userType() QMetaType::QRegExp; otherwise returns an empty QRegExp.

This function was introduced in  Qt 4.1.

See also canConvert() and convert().
*/
impl /*struct*/ QVariant {
  pub fn toRegExp_0<RetType, T: QVariant_toRegExp_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toRegExp_0(self);
    // return 1;
  }
}
pub trait QVariant_toRegExp_0<RetType> {
  fn toRegExp_0(self , rsthis: & QVariant) -> RetType;
}
impl<'a> /*trait*/ QVariant_toRegExp_0<usize> for () {
  fn toRegExp_0(self , rsthis: & QVariant) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QVariant8toRegExpEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qvariant.h:323
// index:0
// Public Visibility=Default Availability=Available
// [8] QRegularExpression toRegularExpression() const

/*
Returns the variant as a QRegularExpression if the variant has userType() QRegularExpression; otherwise returns an empty QRegularExpression.

This function was introduced in  Qt 5.0.

See also canConvert() and convert().
*/
impl /*struct*/ QVariant {
  pub fn toRegularExpression_0<RetType, T: QVariant_toRegularExpression_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toRegularExpression_0(self);
    // return 1;
  }
}
pub trait QVariant_toRegularExpression_0<RetType> {
  fn toRegularExpression_0(self , rsthis: & QVariant) -> RetType;
}
impl<'a> /*trait*/ QVariant_toRegularExpression_0<usize> for () {
  fn toRegularExpression_0(self , rsthis: & QVariant) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QVariant19toRegularExpressionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qvariant.h:325
// index:0
// Public Visibility=Default Availability=Available
// [8] QUrl toUrl() const

/*
Returns the variant as a QUrl if the variant has userType() QMetaType::QUrl; otherwise returns an invalid QUrl.

See also canConvert() and convert().
*/
impl /*struct*/ QVariant {
  pub fn toUrl_0<RetType, T: QVariant_toUrl_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toUrl_0(self);
    // return 1;
  }
}
pub trait QVariant_toUrl_0<RetType> {
  fn toUrl_0(self , rsthis: & QVariant) -> RetType;
}
impl<'a> /*trait*/ QVariant_toUrl_0<usize> for () {
  fn toUrl_0(self , rsthis: & QVariant) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QVariant5toUrlEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qvariant.h:326
// index:0
// Public Visibility=Default Availability=Available
// [8] QEasingCurve toEasingCurve() const

/*
Returns the variant as a QEasingCurve if the variant has userType() QMetaType::QEasingCurve; otherwise returns a default easing curve.

This function was introduced in  Qt 4.7.

See also canConvert() and convert().
*/
impl /*struct*/ QVariant {
  pub fn toEasingCurve_0<RetType, T: QVariant_toEasingCurve_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toEasingCurve_0(self);
    // return 1;
  }
}
pub trait QVariant_toEasingCurve_0<RetType> {
  fn toEasingCurve_0(self , rsthis: & QVariant) -> RetType;
}
impl<'a> /*trait*/ QVariant_toEasingCurve_0<usize> for () {
  fn toEasingCurve_0(self , rsthis: & QVariant) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QVariant13toEasingCurveEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qvariant.h:327
// index:0
// Public Visibility=Default Availability=Available
// [16] QUuid toUuid() const

/*
Returns the variant as a QUuid if the variant has type() QMetaType::QUuid, QMetaType::QByteArray or QMetaType::QString; otherwise returns a default-constructed QUuid.

This function was introduced in  Qt 5.0.

See also canConvert() and convert().
*/
impl /*struct*/ QVariant {
  pub fn toUuid_0<RetType, T: QVariant_toUuid_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toUuid_0(self);
    // return 1;
  }
}
pub trait QVariant_toUuid_0<RetType> {
  fn toUuid_0(self , rsthis: & QVariant) -> RetType;
}
impl<'a> /*trait*/ QVariant_toUuid_0<usize> for () {
  fn toUuid_0(self , rsthis: & QVariant) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QVariant6toUuidEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qvariant.h:328
// index:0
// Public Visibility=Default Availability=Available
// [24] QModelIndex toModelIndex() const

/*
Returns the variant as a QModelIndex if the variant has userType() QModelIndex; otherwise returns a default constructed QModelIndex.

This function was introduced in  Qt 5.0.

See also canConvert(), convert(), and toPersistentModelIndex().
*/
impl /*struct*/ QVariant {
  pub fn toModelIndex_0<RetType, T: QVariant_toModelIndex_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toModelIndex_0(self);
    // return 1;
  }
}
pub trait QVariant_toModelIndex_0<RetType> {
  fn toModelIndex_0(self , rsthis: & QVariant) -> RetType;
}
impl<'a> /*trait*/ QVariant_toModelIndex_0<usize> for () {
  fn toModelIndex_0(self , rsthis: & QVariant) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QVariant12toModelIndexEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qvariant.h:329
// index:0
// Public Visibility=Default Availability=Available
// [8] QPersistentModelIndex toPersistentModelIndex() const

/*
Returns the variant as a QPersistentModelIndex if the variant has userType() QPersistentModelIndex; otherwise returns a default constructed QPersistentModelIndex.

This function was introduced in  Qt 5.5.

See also canConvert(), convert(), and toModelIndex().
*/
impl /*struct*/ QVariant {
  pub fn toPersistentModelIndex_0<RetType, T: QVariant_toPersistentModelIndex_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toPersistentModelIndex_0(self);
    // return 1;
  }
}
pub trait QVariant_toPersistentModelIndex_0<RetType> {
  fn toPersistentModelIndex_0(self , rsthis: & QVariant) -> RetType;
}
impl<'a> /*trait*/ QVariant_toPersistentModelIndex_0<usize> for () {
  fn toPersistentModelIndex_0(self , rsthis: & QVariant) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QVariant22toPersistentModelIndexEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qvariant.h:330
// index:0
// Public Visibility=Default Availability=Available
// [24] QJsonValue toJsonValue() const

/*
Returns the variant as a QJsonValue if the variant has userType() QJsonValue; otherwise returns a default constructed QJsonValue.

This function was introduced in  Qt 5.0.

See also canConvert() and convert().
*/
impl /*struct*/ QVariant {
  pub fn toJsonValue_0<RetType, T: QVariant_toJsonValue_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toJsonValue_0(self);
    // return 1;
  }
}
pub trait QVariant_toJsonValue_0<RetType> {
  fn toJsonValue_0(self , rsthis: & QVariant) -> RetType;
}
impl<'a> /*trait*/ QVariant_toJsonValue_0<usize> for () {
  fn toJsonValue_0(self , rsthis: & QVariant) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QVariant11toJsonValueEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qvariant.h:331
// index:0
// Public Visibility=Default Availability=Available
// [16] QJsonObject toJsonObject() const

/*
Returns the variant as a QJsonObject if the variant has userType() QJsonObject; otherwise returns a default constructed QJsonObject.

This function was introduced in  Qt 5.0.

See also canConvert() and convert().
*/
impl /*struct*/ QVariant {
  pub fn toJsonObject_0<RetType, T: QVariant_toJsonObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toJsonObject_0(self);
    // return 1;
  }
}
pub trait QVariant_toJsonObject_0<RetType> {
  fn toJsonObject_0(self , rsthis: & QVariant) -> RetType;
}
impl<'a> /*trait*/ QVariant_toJsonObject_0<usize> for () {
  fn toJsonObject_0(self , rsthis: & QVariant) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QVariant12toJsonObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qvariant.h:332
// index:0
// Public Visibility=Default Availability=Available
// [16] QJsonArray toJsonArray() const

/*
Returns the variant as a QJsonArray if the variant has userType() QJsonArray; otherwise returns a default constructed QJsonArray.

This function was introduced in  Qt 5.0.

See also canConvert() and convert().
*/
impl /*struct*/ QVariant {
  pub fn toJsonArray_0<RetType, T: QVariant_toJsonArray_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toJsonArray_0(self);
    // return 1;
  }
}
pub trait QVariant_toJsonArray_0<RetType> {
  fn toJsonArray_0(self , rsthis: & QVariant) -> RetType;
}
impl<'a> /*trait*/ QVariant_toJsonArray_0<usize> for () {
  fn toJsonArray_0(self , rsthis: & QVariant) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QVariant11toJsonArrayEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qvariant.h:333
// index:0
// Public Visibility=Default Availability=Available
// [8] QJsonDocument toJsonDocument() const

/*
Returns the variant as a QJsonDocument if the variant has userType() QJsonDocument; otherwise returns a default constructed QJsonDocument.

This function was introduced in  Qt 5.0.

See also canConvert() and convert().
*/
impl /*struct*/ QVariant {
  pub fn toJsonDocument_0<RetType, T: QVariant_toJsonDocument_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toJsonDocument_0(self);
    // return 1;
  }
}
pub trait QVariant_toJsonDocument_0<RetType> {
  fn toJsonDocument_0(self , rsthis: & QVariant) -> RetType;
}
impl<'a> /*trait*/ QVariant_toJsonDocument_0<usize> for () {
  fn toJsonDocument_0(self , rsthis: & QVariant) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QVariant14toJsonDocumentEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qvariant.h:337
// index:0
// Public Visibility=Default Availability=Available
// [-2] void load(QDataStream &)

/*

*/
impl /*struct*/ QVariant {
  pub fn load_0<RetType, T: QVariant_load_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.load_0(self);
    // return 1;
  }
}
pub trait QVariant_load_0<RetType> {
  fn load_0(self , rsthis: & QVariant) -> RetType;
}
impl<'a> /*trait*/ QVariant_load_0<(/*void*/)> for (usize) {
  fn load_0(self , rsthis: & QVariant) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QVariant4loadER11QDataStream", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qvariant.h:338
// index:0
// Public Visibility=Default Availability=Available
// [-2] void save(QDataStream &) const

/*

*/
impl /*struct*/ QVariant {
  pub fn save_0<RetType, T: QVariant_save_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.save_0(self);
    // return 1;
  }
}
pub trait QVariant_save_0<RetType> {
  fn save_0(self , rsthis: & QVariant) -> RetType;
}
impl<'a> /*trait*/ QVariant_save_0<(/*void*/)> for (usize) {
  fn save_0(self , rsthis: & QVariant) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK8QVariant4saveER11QDataStream", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qvariant.h:340
// index:0
// Public static Visibility=Default Availability=Available
// [8] const char * typeToName(int)

/*
Converts the int representation of the storage type, typeId, to its string representation.

Returns a null pointer if the type is QMetaType::UnknownType or doesn't exist.
*/
impl /*struct*/ QVariant {
  pub fn typeToName_0<RetType, T: QVariant_typeToName_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.typeToName_0();
    // return 1;
  }
}
pub trait QVariant_typeToName_0<RetType> {
  fn typeToName_0(self ) -> RetType;
}
impl<'a> /*trait*/ QVariant_typeToName_0<usize> for (i32) {
  fn typeToName_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QVariant10typeToNameEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qvariant.h:341
// index:0
// Public static Visibility=Default Availability=Available
// [4] QVariant::Type nameToType(const char *)

/*
Converts the string representation of the storage type given in name, to its enum representation.

If the string representation cannot be converted to any enum representation, the variant is set to Invalid.
*/
impl /*struct*/ QVariant {
  pub fn nameToType_0<RetType, T: QVariant_nameToType_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.nameToType_0();
    // return 1;
  }
}
pub trait QVariant_nameToType_0<RetType> {
  fn nameToType_0(self ) -> RetType;
}
impl<'a> /*trait*/ QVariant_nameToType_0<i32> for (usize) {
  fn nameToType_0(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QVariant10nameToTypeEPKc", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qvariant.h:343
// index:0
// Public Visibility=Default Availability=Available
// [8] void * data()

/*

*/
impl /*struct*/ QVariant {
  pub fn data_0<RetType, T: QVariant_data_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.data_0(self);
    // return 1;
  }
}
pub trait QVariant_data_0<RetType> {
  fn data_0(self , rsthis: & QVariant) -> RetType;
}
impl<'a> /*trait*/ QVariant_data_0<usize> for () {
  fn data_0(self , rsthis: & QVariant) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QVariant4dataEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qvariant.h:345
// index:1
// Public inline Visibility=Default Availability=Available
// [8] const void * data() const

/*

*/
impl /*struct*/ QVariant {
  pub fn data_1<RetType, T: QVariant_data_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.data_1(self);
    // return 1;
  }
}
pub trait QVariant_data_1<RetType> {
  fn data_1(self , rsthis: & QVariant) -> RetType;
}
impl<'a> /*trait*/ QVariant_data_1<usize> for () {
  fn data_1(self , rsthis: & QVariant) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QVariant4dataEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qvariant.h:344
// index:0
// Public Visibility=Default Availability=Available
// [8] const void * constData() const

/*

*/
impl /*struct*/ QVariant {
  pub fn constData_0<RetType, T: QVariant_constData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.constData_0(self);
    // return 1;
  }
}
pub trait QVariant_constData_0<RetType> {
  fn constData_0(self , rsthis: & QVariant) -> RetType;
}
impl<'a> /*trait*/ QVariant_constData_0<usize> for () {
  fn constData_0(self , rsthis: & QVariant) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QVariant9constDataEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qvariant.h:436
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator==(const QVariant &) const

/*

*/
impl /*struct*/ QVariant {
  pub fn operator_equal_equal_0<RetType, T: QVariant_operator_equal_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_equal_0(self);
    // return 1;
  }
}
pub trait QVariant_operator_equal_equal_0<RetType> {
  fn operator_equal_equal_0(self , rsthis: & QVariant) -> RetType;
}
impl<'a> /*trait*/ QVariant_operator_equal_equal_0<bool> for (usize) {
  fn operator_equal_equal_0(self , rsthis: & QVariant) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QVarianteqERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qvariant.h:438
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator!=(const QVariant &) const

/*

*/
impl /*struct*/ QVariant {
  pub fn operator_not_equal_0<RetType, T: QVariant_operator_not_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_not_equal_0(self);
    // return 1;
  }
}
pub trait QVariant_operator_not_equal_0<RetType> {
  fn operator_not_equal_0(self , rsthis: & QVariant) -> RetType;
}
impl<'a> /*trait*/ QVariant_operator_not_equal_0<bool> for (usize) {
  fn operator_not_equal_0(self , rsthis: & QVariant) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QVariantneERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qvariant.h:440
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator<(const QVariant &) const

/*

*/
impl /*struct*/ QVariant {
  pub fn operator_less_than_0<RetType, T: QVariant_operator_less_than_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_less_than_0(self);
    // return 1;
  }
}
pub trait QVariant_operator_less_than_0<RetType> {
  fn operator_less_than_0(self , rsthis: & QVariant) -> RetType;
}
impl<'a> /*trait*/ QVariant_operator_less_than_0<bool> for (usize) {
  fn operator_less_than_0(self , rsthis: & QVariant) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QVariantltERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qvariant.h:442
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator<=(const QVariant &) const

/*

*/
impl /*struct*/ QVariant {
  pub fn operator_less_than_equal_0<RetType, T: QVariant_operator_less_than_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_less_than_equal_0(self);
    // return 1;
  }
}
pub trait QVariant_operator_less_than_equal_0<RetType> {
  fn operator_less_than_equal_0(self , rsthis: & QVariant) -> RetType;
}
impl<'a> /*trait*/ QVariant_operator_less_than_equal_0<bool> for (usize) {
  fn operator_less_than_equal_0(self , rsthis: & QVariant) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QVariantleERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qvariant.h:444
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator>(const QVariant &) const

/*

*/
impl /*struct*/ QVariant {
  pub fn operator_greater_than_0<RetType, T: QVariant_operator_greater_than_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_greater_than_0(self);
    // return 1;
  }
}
pub trait QVariant_operator_greater_than_0<RetType> {
  fn operator_greater_than_0(self , rsthis: & QVariant) -> RetType;
}
impl<'a> /*trait*/ QVariant_operator_greater_than_0<bool> for (usize) {
  fn operator_greater_than_0(self , rsthis: & QVariant) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QVariantgtERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qvariant.h:446
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator>=(const QVariant &) const

/*

*/
impl /*struct*/ QVariant {
  pub fn operator_greater_than_equal_0<RetType, T: QVariant_operator_greater_than_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_greater_than_equal_0(self);
    // return 1;
  }
}
pub trait QVariant_operator_greater_than_equal_0<RetType> {
  fn operator_greater_than_equal_0(self , rsthis: & QVariant) -> RetType;
}
impl<'a> /*trait*/ QVariant_operator_greater_than_equal_0<bool> for (usize) {
  fn operator_greater_than_equal_0(self , rsthis: & QVariant) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QVariantgeERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qvariant.h:464
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void create(int, const void *)

/*

*/
impl /*struct*/ QVariant {
  pub fn create_0<RetType, T: QVariant_create_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.create_0(self);
    // return 1;
  }
}
pub trait QVariant_create_0<RetType> {
  fn create_0(self , rsthis: & QVariant) -> RetType;
}
impl<'a> /*trait*/ QVariant_create_0<(/*void*/)> for (i32,usize) {
  fn create_0(self , rsthis: & QVariant) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QVariant6createEiPKv", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qvariant.h:465
// index:0
// Protected Visibility=Default Availability=Available
// [1] bool cmp(const QVariant &) const

/*

*/
impl /*struct*/ QVariant {
  pub fn cmp_0<RetType, T: QVariant_cmp_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cmp_0(self);
    // return 1;
  }
}
pub trait QVariant_cmp_0<RetType> {
  fn cmp_0(self , rsthis: & QVariant) -> RetType;
}
impl<'a> /*trait*/ QVariant_cmp_0<bool> for (usize) {
  fn cmp_0(self , rsthis: & QVariant) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QVariant3cmpERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qvariant.h:466
// index:0
// Protected Visibility=Default Availability=Available
// [4] int compare(const QVariant &) const

/*

*/
impl /*struct*/ QVariant {
  pub fn compare_0<RetType, T: QVariant_compare_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.compare_0(self);
    // return 1;
  }
}
pub trait QVariant_compare_0<RetType> {
  fn compare_0(self , rsthis: & QVariant) -> RetType;
}
impl<'a> /*trait*/ QVariant_compare_0<i32> for (usize) {
  fn compare_0(self , rsthis: & QVariant) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QVariant7compareERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}


/*


*/
pub type QVariant__Type = i32;
// 
pub const QVariant__Invalid :QVariant__Type = 0;
// 
pub const QVariant__Bool :QVariant__Type = 1;
// 
pub const QVariant__Int :QVariant__Type = 2;
// 
pub const QVariant__UInt :QVariant__Type = 3;
// 
pub const QVariant__LongLong :QVariant__Type = 4;
// 
pub const QVariant__ULongLong :QVariant__Type = 5;
// 
pub const QVariant__Double :QVariant__Type = 6;
// 
pub const QVariant__Char :QVariant__Type = 7;
// 
pub const QVariant__Map :QVariant__Type = 8;
// 
pub const QVariant__List :QVariant__Type = 9;
// 
pub const QVariant__String :QVariant__Type = 10;
// 
pub const QVariant__StringList :QVariant__Type = 11;
// 
pub const QVariant__ByteArray :QVariant__Type = 12;
// 
pub const QVariant__BitArray :QVariant__Type = 13;
// 
pub const QVariant__Date :QVariant__Type = 14;
// 
pub const QVariant__Time :QVariant__Type = 15;
// 
pub const QVariant__DateTime :QVariant__Type = 16;
// 
pub const QVariant__Url :QVariant__Type = 17;
// 
pub const QVariant__Locale :QVariant__Type = 18;
// 
pub const QVariant__Rect :QVariant__Type = 19;
// 
pub const QVariant__RectF :QVariant__Type = 20;
// 
pub const QVariant__Size :QVariant__Type = 21;
// 
pub const QVariant__SizeF :QVariant__Type = 22;
// 
pub const QVariant__Line :QVariant__Type = 23;
// 
pub const QVariant__LineF :QVariant__Type = 24;
// 
pub const QVariant__Point :QVariant__Type = 25;
// 
pub const QVariant__PointF :QVariant__Type = 26;
// 
pub const QVariant__RegExp :QVariant__Type = 27;
// 
pub const QVariant__RegularExpression :QVariant__Type = 44;
// 
pub const QVariant__Hash :QVariant__Type = 28;
// 
pub const QVariant__EasingCurve :QVariant__Type = 29;
// 
pub const QVariant__Uuid :QVariant__Type = 30;
// 
pub const QVariant__ModelIndex :QVariant__Type = 42;
// 
pub const QVariant__PersistentModelIndex :QVariant__Type = 50;
// 
pub const QVariant__LastCoreType :QVariant__Type = 51;
// 
pub const QVariant__Font :QVariant__Type = 64;
// 
pub const QVariant__Pixmap :QVariant__Type = 65;
// 
pub const QVariant__Brush :QVariant__Type = 66;
// 
pub const QVariant__Color :QVariant__Type = 67;
// 
pub const QVariant__Palette :QVariant__Type = 68;
// 
pub const QVariant__Image :QVariant__Type = 70;
// 
pub const QVariant__Polygon :QVariant__Type = 71;
// 
pub const QVariant__Region :QVariant__Type = 72;
// 
pub const QVariant__Bitmap :QVariant__Type = 73;
// 
pub const QVariant__Cursor :QVariant__Type = 74;
// 
pub const QVariant__KeySequence :QVariant__Type = 75;
// 
pub const QVariant__Pen :QVariant__Type = 76;
// 
pub const QVariant__TextLength :QVariant__Type = 77;
// 
pub const QVariant__TextFormat :QVariant__Type = 78;
// 
pub const QVariant__Matrix :QVariant__Type = 79;
// 
pub const QVariant__Transform :QVariant__Type = 80;
// 
pub const QVariant__Matrix4x4 :QVariant__Type = 81;
// 
pub const QVariant__Vector2D :QVariant__Type = 82;
// 
pub const QVariant__Vector3D :QVariant__Type = 83;
// 
pub const QVariant__Vector4D :QVariant__Type = 84;
// 
pub const QVariant__Quaternion :QVariant__Type = 85;
// 
pub const QVariant__PolygonF :QVariant__Type = 86;
// 
pub const QVariant__Icon :QVariant__Type = 69;
// 
pub const QVariant__LastGuiType :QVariant__Type = 86;
// 
pub const QVariant__SizePolicy :QVariant__Type = 121;
// 
pub const QVariant__UserType :QVariant__Type = 1024;
// 
pub const QVariant__LastType :QVariant__Type = -1;
pub fn QVariant_TypeItemName(val: i32) ->String {
  match val {
     QVariant__Invalid => // 0
     {return String::from("Invalid");}
     QVariant__Bool => // 1
     {return String::from("Bool");}
     QVariant__Int => // 2
     {return String::from("Int");}
     QVariant__UInt => // 3
     {return String::from("UInt");}
     QVariant__LongLong => // 4
     {return String::from("LongLong");}
     QVariant__ULongLong => // 5
     {return String::from("ULongLong");}
     QVariant__Double => // 6
     {return String::from("Double");}
     QVariant__Char => // 7
     {return String::from("Char");}
     QVariant__Map => // 8
     {return String::from("Map");}
     QVariant__List => // 9
     {return String::from("List");}
     QVariant__String => // 10
     {return String::from("String");}
     QVariant__StringList => // 11
     {return String::from("StringList");}
     QVariant__ByteArray => // 12
     {return String::from("ByteArray");}
     QVariant__BitArray => // 13
     {return String::from("BitArray");}
     QVariant__Date => // 14
     {return String::from("Date");}
     QVariant__Time => // 15
     {return String::from("Time");}
     QVariant__DateTime => // 16
     {return String::from("DateTime");}
     QVariant__Url => // 17
     {return String::from("Url");}
     QVariant__Locale => // 18
     {return String::from("Locale");}
     QVariant__Rect => // 19
     {return String::from("Rect");}
     QVariant__RectF => // 20
     {return String::from("RectF");}
     QVariant__Size => // 21
     {return String::from("Size");}
     QVariant__SizeF => // 22
     {return String::from("SizeF");}
     QVariant__Line => // 23
     {return String::from("Line");}
     QVariant__LineF => // 24
     {return String::from("LineF");}
     QVariant__Point => // 25
     {return String::from("Point");}
     QVariant__PointF => // 26
     {return String::from("PointF");}
     QVariant__RegExp => // 27
     {return String::from("RegExp");}
     QVariant__RegularExpression => // 44
     {return String::from("RegularExpression");}
     QVariant__Hash => // 28
     {return String::from("Hash");}
     QVariant__EasingCurve => // 29
     {return String::from("EasingCurve");}
     QVariant__Uuid => // 30
     {return String::from("Uuid");}
     QVariant__ModelIndex => // 42
     {return String::from("ModelIndex");}
     QVariant__PersistentModelIndex => // 50
     {return String::from("PersistentModelIndex");}
     QVariant__LastCoreType => // 51
     {return String::from("LastCoreType");}
     QVariant__Font => // 64
     {return String::from("Font");}
     QVariant__Pixmap => // 65
     {return String::from("Pixmap");}
     QVariant__Brush => // 66
     {return String::from("Brush");}
     QVariant__Color => // 67
     {return String::from("Color");}
     QVariant__Palette => // 68
     {return String::from("Palette");}
     QVariant__Image => // 70
     {return String::from("Image");}
     QVariant__Polygon => // 71
     {return String::from("Polygon");}
     QVariant__Region => // 72
     {return String::from("Region");}
     QVariant__Bitmap => // 73
     {return String::from("Bitmap");}
     QVariant__Cursor => // 74
     {return String::from("Cursor");}
     QVariant__KeySequence => // 75
     {return String::from("KeySequence");}
     QVariant__Pen => // 76
     {return String::from("Pen");}
     QVariant__TextLength => // 77
     {return String::from("TextLength");}
     QVariant__TextFormat => // 78
     {return String::from("TextFormat");}
     QVariant__Matrix => // 79
     {return String::from("Matrix");}
     QVariant__Transform => // 80
     {return String::from("Transform");}
     QVariant__Matrix4x4 => // 81
     {return String::from("Matrix4x4");}
     QVariant__Vector2D => // 82
     {return String::from("Vector2D");}
     QVariant__Vector3D => // 83
     {return String::from("Vector3D");}
     QVariant__Vector4D => // 84
     {return String::from("Vector4D");}
     QVariant__Quaternion => // 85
     {return String::from("Quaternion");}
     QVariant__PolygonF => // 86
     {return String::from("PolygonF,LastGuiType");}
     QVariant__Icon => // 69
     {return String::from("Icon");}
    // QVariant__LastGuiType => // 86
    // {return String::from("");}
     QVariant__SizePolicy => // 121
     {return String::from("SizePolicy");}
     QVariant__UserType => // 1024
     {return String::from("UserType");}
     QVariant__LastType => // -1
     {return String::from("LastType");}
  _ => {return format!("{}", val);}
}
}
pub fn QVariant_TypeItemName_s(val: i32) ->String {
  //var nilthis *QVariant
  //return nilthis.TypeItemName(val);
  return QVariant_TypeItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
