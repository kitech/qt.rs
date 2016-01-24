// auto generated, do not modify.
// created: Sun Jan 24 17:41:38 2016
// src-file: /QtCore/qstringbuilder.h
// dst-file: /src/core/qstringbuilder.rs
//

// header block begin =>
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;


// <= header block end

// main block begin =>
// <= main block end

// use block begin =>
// use super::qstringbuilder::QAbstractConcatenable; // 773
use std::ops::Deref;
use super::qchar::QChar; // 773
use super::qstring::QCharRef; // 773
use super::qstring::QString; // 773
use super::qbytearray::QByteArray; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QConcatenable_QLatin1String__Class_Size() -> c_int;
  fn QConcatenable_QCharRef__Class_Size() -> c_int;
  // proto: static void QConcatenable<QCharRef>::appendTo(QCharRef c, QChar *& out);
  fn C_ZN13QConcatenableI8QCharRefE8appendToES0_RP5QChar(arg0: *mut c_void, arg1: *mut c_void);
  // proto: static int QConcatenable<QCharRef>::size(QCharRef );
  fn C_ZN13QConcatenableI8QCharRefE4sizeES0_(arg0: *mut c_void) -> c_int;
  fn QConcatenable_QStringRef__Class_Size() -> c_int;
  fn QAbstractConcatenable_Class_Size() -> c_int;
  fn QConcatenable_QString__Class_Size() -> c_int;
  // proto: static int QConcatenable<QString>::size(const QString & a);
  fn C_ZN13QConcatenableI7QStringE4sizeERKS0_(arg0: *mut c_void) -> c_int;
  // proto: static void QConcatenable<QString>::appendTo(const QString & a, QChar *& out);
  fn C_ZN13QConcatenableI7QStringE8appendToERKS0_RP5QChar(arg0: *mut c_void, arg1: *mut c_void);
  fn QConcatenable_QLatin1Char__Class_Size() -> c_int;
  fn QConcatenable_char__Class_Size() -> c_int;
  // proto: static void QConcatenable<char>::appendTo(const char c, QChar *& out);
  fn C_ZN13QConcatenableIcE8appendToEcRP5QChar(arg0: c_char, arg1: *mut c_void);
  // proto: static int QConcatenable<char>::size(const char );
  fn C_ZN13QConcatenableIcE4sizeEc(arg0: c_char) -> c_int;
  // proto: static void QConcatenable<char>::appendTo(const char c, char *& out);
  fn C_ZN13QConcatenableIcE8appendToEcRPc(arg0: c_char, arg1: *mut c_char);
  fn QConcatenable_QByteArray__Class_Size() -> c_int;
  // proto: static void QConcatenable<QByteArray>::appendTo(const QByteArray & ba, QChar *& out);
  fn C_ZN13QConcatenableI10QByteArrayE8appendToERKS0_RP5QChar(arg0: *mut c_void, arg1: *mut c_void);
  // proto: static void QConcatenable<QByteArray>::appendTo(const QByteArray & ba, char *& out);
  fn C_ZN13QConcatenableI10QByteArrayE8appendToERKS0_RPc(arg0: *mut c_void, arg1: *mut c_char);
  // proto: static int QConcatenable<QByteArray>::size(const QByteArray & ba);
  fn C_ZN13QConcatenableI10QByteArrayE4sizeERKS0_(arg0: *mut c_void) -> c_int;
  fn QConcatenable_QChar__SpecialCharacter__Class_Size() -> c_int;
  fn QConcatenable_QChar__Class_Size() -> c_int;
  // proto: static int QConcatenable<QChar>::size(const QChar );
  fn C_ZN13QConcatenableI5QCharE4sizeES0_(arg0: *mut c_void) -> c_int;
  // proto: static void QConcatenable<QChar>::appendTo(const QChar c, QChar *& out);
  fn C_ZN13QConcatenableI5QCharE8appendToES0_RPS0_(arg0: *mut c_void, arg1: *mut c_void);
  fn QStringBuilder_QByteArray_QByteArray__Class_Size() -> c_int;
  // proto:  void QStringBuilder<QByteArray, QByteArray>::QStringBuilder(const QByteArray & a_, const QByteArray & b_);
  fn C_ZN14QStringBuilderI10QByteArrayS0_EC2ERKS0_S3_(arg0: *mut c_void, arg1: *mut c_void) -> u64;
  fn QConcatenable_constchar___Class_Size() -> c_int;
  // proto: static void QConcatenable<const char *>::appendTo(const char * a, QChar *& out);
  fn C_ZN13QConcatenableIPKcE8appendToES1_RP5QChar(arg0: *mut c_char, arg1: *mut c_void);
  // proto: static void QConcatenable<const char *>::appendTo(const char * a, char *& out);
  fn C_ZN13QConcatenableIPKcE8appendToES1_RPc(arg0: *mut c_char, arg1: *mut c_char);
  // proto: static int QConcatenable<const char *>::size(const char * a);
  fn C_ZN13QConcatenableIPKcE4sizeES1_(arg0: *mut c_char) -> c_int;
  fn QStringBuilder_QString_QString__Class_Size() -> c_int;
  // proto:  void QStringBuilder<QString, QString>::QStringBuilder(const QString & a_, const QString & b_);
  fn C_ZN14QStringBuilderI7QStringS0_EC2ERKS0_S3_(arg0: *mut c_void, arg1: *mut c_void) -> u64;
} // <= ext block end

// body block begin =>
// class sizeof(QConcatenable_QLatin1String_)=1
#[derive(Default)]
pub struct QConcatenable_QLatin1String_ {
  qbase: QAbstractConcatenable,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QConcatenable_QCharRef_)=1
#[derive(Default)]
pub struct QConcatenable_QCharRef_ {
  qbase: QAbstractConcatenable,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QConcatenable_QStringRef_)=1
#[derive(Default)]
pub struct QConcatenable_QStringRef_ {
  qbase: QAbstractConcatenable,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QAbstractConcatenable)=1
#[derive(Default)]
pub struct QAbstractConcatenable {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QConcatenable_QString_)=1
#[derive(Default)]
pub struct QConcatenable_QString_ {
  qbase: QAbstractConcatenable,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QConcatenable_QLatin1Char_)=1
#[derive(Default)]
pub struct QConcatenable_QLatin1Char_ {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QConcatenable_char_)=1
#[derive(Default)]
pub struct QConcatenable_char_ {
  qbase: QAbstractConcatenable,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QConcatenable_QByteArray_)=1
#[derive(Default)]
pub struct QConcatenable_QByteArray_ {
  qbase: QAbstractConcatenable,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QConcatenable_QChar__SpecialCharacter_)=1
#[derive(Default)]
pub struct QConcatenable_QChar__SpecialCharacter_ {
  qbase: QAbstractConcatenable,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QConcatenable_QChar_)=1
#[derive(Default)]
pub struct QConcatenable_QChar_ {
  qbase: QAbstractConcatenable,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QStringBuilder_QByteArray_QByteArray_)=1
#[derive(Default)]
pub struct QStringBuilder_QByteArray_QByteArray_ {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QConcatenable_constchar__)=1
#[derive(Default)]
pub struct QConcatenable_constchar__ {
  qbase: QAbstractConcatenable,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QStringBuilder_QString_QString_)=1
#[derive(Default)]
pub struct QStringBuilder_QString_QString_ {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QConcatenable_QLatin1String_ {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QConcatenable_QLatin1String_ {
    return QConcatenable_QLatin1String_{qbase: QAbstractConcatenable::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QConcatenable_QLatin1String_ {
  type Target = QAbstractConcatenable;

  fn deref(&self) -> &QAbstractConcatenable {
    return & self.qbase;
  }
}
impl AsRef<QAbstractConcatenable> for QConcatenable_QLatin1String_ {
  fn as_ref(& self) -> & QAbstractConcatenable {
    return & self.qbase;
  }
}
impl /*struct*/ QConcatenable_QCharRef_ {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QConcatenable_QCharRef_ {
    return QConcatenable_QCharRef_{qbase: QAbstractConcatenable::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QConcatenable_QCharRef_ {
  type Target = QAbstractConcatenable;

  fn deref(&self) -> &QAbstractConcatenable {
    return & self.qbase;
  }
}
impl AsRef<QAbstractConcatenable> for QConcatenable_QCharRef_ {
  fn as_ref(& self) -> & QAbstractConcatenable {
    return & self.qbase;
  }
}
  // proto: static void QConcatenable<QCharRef>::appendTo(QCharRef c, QChar *& out);
impl /*struct*/ QConcatenable_QCharRef_ {
  pub fn appendTo_s<RetType, T: QConcatenable_QCharRef__appendTo_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.appendTo_s();
    // return 1;
  }
}

pub trait QConcatenable_QCharRef__appendTo_s<RetType> {
  fn appendTo_s(self ) -> RetType;
}

  // proto: static void QConcatenable<QCharRef>::appendTo(QCharRef c, QChar *& out);
impl<'a> /*trait*/ QConcatenable_QCharRef__appendTo_s<()> for (QCharRef, &'a QChar) {
  fn appendTo_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QConcatenableI8QCharRefE8appendToES0_RP5QChar()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {C_ZN13QConcatenableI8QCharRefE8appendToES0_RP5QChar(arg0, arg1)};
    // return 1;
  }
}

  // proto: static int QConcatenable<QCharRef>::size(QCharRef );
impl /*struct*/ QConcatenable_QCharRef_ {
  pub fn size_s<RetType, T: QConcatenable_QCharRef__size_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.size_s();
    // return 1;
  }
}

pub trait QConcatenable_QCharRef__size_s<RetType> {
  fn size_s(self ) -> RetType;
}

  // proto: static int QConcatenable<QCharRef>::size(QCharRef );
impl<'a> /*trait*/ QConcatenable_QCharRef__size_s<i32> for (QCharRef) {
  fn size_s(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QConcatenableI8QCharRefE4sizeES0_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN13QConcatenableI8QCharRefE4sizeES0_(arg0)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QConcatenable_QStringRef_ {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QConcatenable_QStringRef_ {
    return QConcatenable_QStringRef_{qbase: QAbstractConcatenable::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QConcatenable_QStringRef_ {
  type Target = QAbstractConcatenable;

  fn deref(&self) -> &QAbstractConcatenable {
    return & self.qbase;
  }
}
impl AsRef<QAbstractConcatenable> for QConcatenable_QStringRef_ {
  fn as_ref(& self) -> & QAbstractConcatenable {
    return & self.qbase;
  }
}
impl /*struct*/ QAbstractConcatenable {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QAbstractConcatenable {
    return QAbstractConcatenable{qclsinst: qthis, ..Default::default()};
  }
}
impl /*struct*/ QConcatenable_QString_ {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QConcatenable_QString_ {
    return QConcatenable_QString_{qbase: QAbstractConcatenable::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QConcatenable_QString_ {
  type Target = QAbstractConcatenable;

  fn deref(&self) -> &QAbstractConcatenable {
    return & self.qbase;
  }
}
impl AsRef<QAbstractConcatenable> for QConcatenable_QString_ {
  fn as_ref(& self) -> & QAbstractConcatenable {
    return & self.qbase;
  }
}
  // proto: static int QConcatenable<QString>::size(const QString & a);
impl /*struct*/ QConcatenable_QString_ {
  pub fn size_s<RetType, T: QConcatenable_QString__size_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.size_s();
    // return 1;
  }
}

pub trait QConcatenable_QString__size_s<RetType> {
  fn size_s(self ) -> RetType;
}

  // proto: static int QConcatenable<QString>::size(const QString & a);
impl<'a> /*trait*/ QConcatenable_QString__size_s<i32> for (&'a QString) {
  fn size_s(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QConcatenableI7QStringE4sizeERKS0_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN13QConcatenableI7QStringE4sizeERKS0_(arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto: static void QConcatenable<QString>::appendTo(const QString & a, QChar *& out);
impl /*struct*/ QConcatenable_QString_ {
  pub fn appendTo_s<RetType, T: QConcatenable_QString__appendTo_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.appendTo_s();
    // return 1;
  }
}

pub trait QConcatenable_QString__appendTo_s<RetType> {
  fn appendTo_s(self ) -> RetType;
}

  // proto: static void QConcatenable<QString>::appendTo(const QString & a, QChar *& out);
impl<'a> /*trait*/ QConcatenable_QString__appendTo_s<()> for (&'a QString, &'a QChar) {
  fn appendTo_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QConcatenableI7QStringE8appendToERKS0_RP5QChar()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {C_ZN13QConcatenableI7QStringE8appendToERKS0_RP5QChar(arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QConcatenable_QLatin1Char_ {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QConcatenable_QLatin1Char_ {
    return QConcatenable_QLatin1Char_{qclsinst: qthis, ..Default::default()};
  }
}
impl /*struct*/ QConcatenable_char_ {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QConcatenable_char_ {
    return QConcatenable_char_{qbase: QAbstractConcatenable::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QConcatenable_char_ {
  type Target = QAbstractConcatenable;

  fn deref(&self) -> &QAbstractConcatenable {
    return & self.qbase;
  }
}
impl AsRef<QAbstractConcatenable> for QConcatenable_char_ {
  fn as_ref(& self) -> & QAbstractConcatenable {
    return & self.qbase;
  }
}
  // proto: static void QConcatenable<char>::appendTo(const char c, QChar *& out);
impl /*struct*/ QConcatenable_char_ {
  pub fn appendTo_s<RetType, T: QConcatenable_char__appendTo_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.appendTo_s();
    // return 1;
  }
}

pub trait QConcatenable_char__appendTo_s<RetType> {
  fn appendTo_s(self ) -> RetType;
}

  // proto: static void QConcatenable<char>::appendTo(const char c, QChar *& out);
impl<'a> /*trait*/ QConcatenable_char__appendTo_s<()> for (i8, &'a QChar) {
  fn appendTo_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QConcatenableIcE8appendToEcRP5QChar()};
    let arg0 = self.0  as c_char;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {C_ZN13QConcatenableIcE8appendToEcRP5QChar(arg0, arg1)};
    // return 1;
  }
}

  // proto: static int QConcatenable<char>::size(const char );
impl /*struct*/ QConcatenable_char_ {
  pub fn size_s<RetType, T: QConcatenable_char__size_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.size_s();
    // return 1;
  }
}

pub trait QConcatenable_char__size_s<RetType> {
  fn size_s(self ) -> RetType;
}

  // proto: static int QConcatenable<char>::size(const char );
impl<'a> /*trait*/ QConcatenable_char__size_s<i32> for (i8) {
  fn size_s(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QConcatenableIcE4sizeEc()};
    let arg0 = self  as c_char;
    let mut ret = unsafe {C_ZN13QConcatenableIcE4sizeEc(arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto: static void QConcatenable<char>::appendTo(const char c, char *& out);
impl<'a> /*trait*/ QConcatenable_char__appendTo_s<()> for (i8, &'a mut String) {
  fn appendTo_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QConcatenableIcE8appendToEcRPc()};
    let arg0 = self.0  as c_char;
    let arg1 = self.1.as_ptr()  as *mut c_char;
     unsafe {C_ZN13QConcatenableIcE8appendToEcRPc(arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QConcatenable_QByteArray_ {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QConcatenable_QByteArray_ {
    return QConcatenable_QByteArray_{qbase: QAbstractConcatenable::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QConcatenable_QByteArray_ {
  type Target = QAbstractConcatenable;

  fn deref(&self) -> &QAbstractConcatenable {
    return & self.qbase;
  }
}
impl AsRef<QAbstractConcatenable> for QConcatenable_QByteArray_ {
  fn as_ref(& self) -> & QAbstractConcatenable {
    return & self.qbase;
  }
}
  // proto: static void QConcatenable<QByteArray>::appendTo(const QByteArray & ba, QChar *& out);
impl /*struct*/ QConcatenable_QByteArray_ {
  pub fn appendTo_s<RetType, T: QConcatenable_QByteArray__appendTo_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.appendTo_s();
    // return 1;
  }
}

pub trait QConcatenable_QByteArray__appendTo_s<RetType> {
  fn appendTo_s(self ) -> RetType;
}

  // proto: static void QConcatenable<QByteArray>::appendTo(const QByteArray & ba, QChar *& out);
impl<'a> /*trait*/ QConcatenable_QByteArray__appendTo_s<()> for (&'a QByteArray, &'a QChar) {
  fn appendTo_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QConcatenableI10QByteArrayE8appendToERKS0_RP5QChar()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {C_ZN13QConcatenableI10QByteArrayE8appendToERKS0_RP5QChar(arg0, arg1)};
    // return 1;
  }
}

  // proto: static void QConcatenable<QByteArray>::appendTo(const QByteArray & ba, char *& out);
impl<'a> /*trait*/ QConcatenable_QByteArray__appendTo_s<()> for (&'a QByteArray, &'a mut String) {
  fn appendTo_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QConcatenableI10QByteArrayE8appendToERKS0_RPc()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *mut c_char;
     unsafe {C_ZN13QConcatenableI10QByteArrayE8appendToERKS0_RPc(arg0, arg1)};
    // return 1;
  }
}

  // proto: static int QConcatenable<QByteArray>::size(const QByteArray & ba);
impl /*struct*/ QConcatenable_QByteArray_ {
  pub fn size_s<RetType, T: QConcatenable_QByteArray__size_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.size_s();
    // return 1;
  }
}

pub trait QConcatenable_QByteArray__size_s<RetType> {
  fn size_s(self ) -> RetType;
}

  // proto: static int QConcatenable<QByteArray>::size(const QByteArray & ba);
impl<'a> /*trait*/ QConcatenable_QByteArray__size_s<i32> for (&'a QByteArray) {
  fn size_s(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QConcatenableI10QByteArrayE4sizeERKS0_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN13QConcatenableI10QByteArrayE4sizeERKS0_(arg0)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QConcatenable_QChar__SpecialCharacter_ {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QConcatenable_QChar__SpecialCharacter_ {
    return QConcatenable_QChar__SpecialCharacter_{qbase: QAbstractConcatenable::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QConcatenable_QChar__SpecialCharacter_ {
  type Target = QAbstractConcatenable;

  fn deref(&self) -> &QAbstractConcatenable {
    return & self.qbase;
  }
}
impl AsRef<QAbstractConcatenable> for QConcatenable_QChar__SpecialCharacter_ {
  fn as_ref(& self) -> & QAbstractConcatenable {
    return & self.qbase;
  }
}
impl /*struct*/ QConcatenable_QChar_ {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QConcatenable_QChar_ {
    return QConcatenable_QChar_{qbase: QAbstractConcatenable::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QConcatenable_QChar_ {
  type Target = QAbstractConcatenable;

  fn deref(&self) -> &QAbstractConcatenable {
    return & self.qbase;
  }
}
impl AsRef<QAbstractConcatenable> for QConcatenable_QChar_ {
  fn as_ref(& self) -> & QAbstractConcatenable {
    return & self.qbase;
  }
}
  // proto: static int QConcatenable<QChar>::size(const QChar );
impl /*struct*/ QConcatenable_QChar_ {
  pub fn size_s<RetType, T: QConcatenable_QChar__size_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.size_s();
    // return 1;
  }
}

pub trait QConcatenable_QChar__size_s<RetType> {
  fn size_s(self ) -> RetType;
}

  // proto: static int QConcatenable<QChar>::size(const QChar );
impl<'a> /*trait*/ QConcatenable_QChar__size_s<i32> for (QChar) {
  fn size_s(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QConcatenableI5QCharE4sizeES0_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN13QConcatenableI5QCharE4sizeES0_(arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto: static void QConcatenable<QChar>::appendTo(const QChar c, QChar *& out);
impl /*struct*/ QConcatenable_QChar_ {
  pub fn appendTo_s<RetType, T: QConcatenable_QChar__appendTo_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.appendTo_s();
    // return 1;
  }
}

pub trait QConcatenable_QChar__appendTo_s<RetType> {
  fn appendTo_s(self ) -> RetType;
}

  // proto: static void QConcatenable<QChar>::appendTo(const QChar c, QChar *& out);
impl<'a> /*trait*/ QConcatenable_QChar__appendTo_s<()> for (QChar, &'a QChar) {
  fn appendTo_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QConcatenableI5QCharE8appendToES0_RPS0_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {C_ZN13QConcatenableI5QCharE8appendToES0_RPS0_(arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QStringBuilder_QByteArray_QByteArray_ {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QStringBuilder_QByteArray_QByteArray_ {
    return QStringBuilder_QByteArray_QByteArray_{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void QStringBuilder<QByteArray, QByteArray>::QStringBuilder(const QByteArray & a_, const QByteArray & b_);
impl /*struct*/ QStringBuilder_QByteArray_QByteArray_ {
  pub fn new<T: QStringBuilder_QByteArray_QByteArray__new>(value: T) -> QStringBuilder_QByteArray_QByteArray_ {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QStringBuilder_QByteArray_QByteArray__new {
  fn new(self) -> QStringBuilder_QByteArray_QByteArray_;
}

  // proto:  void QStringBuilder<QByteArray, QByteArray>::QStringBuilder(const QByteArray & a_, const QByteArray & b_);
impl<'a> /*trait*/ QStringBuilder_QByteArray_QByteArray__new for (&'a QByteArray, &'a QByteArray) {
  fn new(self) -> QStringBuilder_QByteArray_QByteArray_ {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QStringBuilderI10QByteArrayS0_EC2ERKS0_S3_()};
    let ctysz: c_int = unsafe{QStringBuilder_QByteArray_QByteArray__Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN14QStringBuilderI10QByteArrayS0_EC2ERKS0_S3_(arg0, arg1)};
    let rsthis = QStringBuilder_QByteArray_QByteArray_{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QConcatenable_constchar__ {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QConcatenable_constchar__ {
    return QConcatenable_constchar__{qbase: QAbstractConcatenable::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QConcatenable_constchar__ {
  type Target = QAbstractConcatenable;

  fn deref(&self) -> &QAbstractConcatenable {
    return & self.qbase;
  }
}
impl AsRef<QAbstractConcatenable> for QConcatenable_constchar__ {
  fn as_ref(& self) -> & QAbstractConcatenable {
    return & self.qbase;
  }
}
  // proto: static void QConcatenable<const char *>::appendTo(const char * a, QChar *& out);
impl /*struct*/ QConcatenable_constchar__ {
  pub fn appendTo_s<RetType, T: QConcatenable_constchar___appendTo_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.appendTo_s();
    // return 1;
  }
}

pub trait QConcatenable_constchar___appendTo_s<RetType> {
  fn appendTo_s(self ) -> RetType;
}

  // proto: static void QConcatenable<const char *>::appendTo(const char * a, QChar *& out);
impl<'a> /*trait*/ QConcatenable_constchar___appendTo_s<()> for (&'a  String, &'a QChar) {
  fn appendTo_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QConcatenableIPKcE8appendToES1_RP5QChar()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {C_ZN13QConcatenableIPKcE8appendToES1_RP5QChar(arg0, arg1)};
    // return 1;
  }
}

  // proto: static void QConcatenable<const char *>::appendTo(const char * a, char *& out);
impl<'a> /*trait*/ QConcatenable_constchar___appendTo_s<()> for (&'a  String, &'a mut String) {
  fn appendTo_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QConcatenableIPKcE8appendToES1_RPc()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1.as_ptr()  as *mut c_char;
     unsafe {C_ZN13QConcatenableIPKcE8appendToES1_RPc(arg0, arg1)};
    // return 1;
  }
}

  // proto: static int QConcatenable<const char *>::size(const char * a);
impl /*struct*/ QConcatenable_constchar__ {
  pub fn size_s<RetType, T: QConcatenable_constchar___size_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.size_s();
    // return 1;
  }
}

pub trait QConcatenable_constchar___size_s<RetType> {
  fn size_s(self ) -> RetType;
}

  // proto: static int QConcatenable<const char *>::size(const char * a);
impl<'a> /*trait*/ QConcatenable_constchar___size_s<i32> for (&'a  String) {
  fn size_s(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QConcatenableIPKcE4sizeES1_()};
    let arg0 = self.as_ptr()  as *mut c_char;
    let mut ret = unsafe {C_ZN13QConcatenableIPKcE4sizeES1_(arg0)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QStringBuilder_QString_QString_ {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QStringBuilder_QString_QString_ {
    return QStringBuilder_QString_QString_{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void QStringBuilder<QString, QString>::QStringBuilder(const QString & a_, const QString & b_);
impl /*struct*/ QStringBuilder_QString_QString_ {
  pub fn new<T: QStringBuilder_QString_QString__new>(value: T) -> QStringBuilder_QString_QString_ {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QStringBuilder_QString_QString__new {
  fn new(self) -> QStringBuilder_QString_QString_;
}

  // proto:  void QStringBuilder<QString, QString>::QStringBuilder(const QString & a_, const QString & b_);
impl<'a> /*trait*/ QStringBuilder_QString_QString__new for (&'a QString, &'a QString) {
  fn new(self) -> QStringBuilder_QString_QString_ {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QStringBuilderI7QStringS0_EC2ERKS0_S3_()};
    let ctysz: c_int = unsafe{QStringBuilder_QString_QString__Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN14QStringBuilderI7QStringS0_EC2ERKS0_S3_(arg0, arg1)};
    let rsthis = QStringBuilder_QString_QString_{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// <= body block end

