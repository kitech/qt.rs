// auto generated, do not modify.
// created: Mon Feb 22 23:57:02 2016
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
use super::qchar::*; // 773
use super::qstring::*; // 773
use super::qbytearray::*; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QConcatenableLQLatin1StringG_Class_Size() -> c_int;
  fn QConcatenableLQCharRefG_Class_Size() -> c_int;
  // proto: static void QConcatenable<QCharRef>::appendTo(QCharRef c, QChar *& out);
  fn C_ZN13QConcatenableI8QCharRefE8appendToES0_RP5QChar(arg0: *mut c_void, arg1: *mut c_void);
  // proto: static int QConcatenable<QCharRef>::size(QCharRef );
  fn C_ZN13QConcatenableI8QCharRefE4sizeES0_(arg0: *mut c_void) -> c_int;
  fn QConcatenableLQStringRefG_Class_Size() -> c_int;
  fn QAbstractConcatenable_Class_Size() -> c_int;
  fn QConcatenableLQStringG_Class_Size() -> c_int;
  // proto: static int QConcatenable<QString>::size(const QString & a);
  fn C_ZN13QConcatenableI7QStringE4sizeERKS0_(arg0: *mut c_void) -> c_int;
  // proto: static void QConcatenable<QString>::appendTo(const QString & a, QChar *& out);
  fn C_ZN13QConcatenableI7QStringE8appendToERKS0_RP5QChar(arg0: *mut c_void, arg1: *mut c_void);
  fn QConcatenableLQLatin1CharG_Class_Size() -> c_int;
  fn QConcatenableLcharG_Class_Size() -> c_int;
  // proto: static void QConcatenable<char>::appendTo(const char c, QChar *& out);
  fn C_ZN13QConcatenableIcE8appendToEcRP5QChar(arg0: c_char, arg1: *mut c_void);
  // proto: static int QConcatenable<char>::size(const char );
  fn C_ZN13QConcatenableIcE4sizeEc(arg0: c_char) -> c_int;
  // proto: static void QConcatenable<char>::appendTo(const char c, char *& out);
  fn C_ZN13QConcatenableIcE8appendToEcRPc(arg0: c_char, arg1: *mut c_char);
  fn QConcatenableLQByteArrayG_Class_Size() -> c_int;
  // proto: static void QConcatenable<QByteArray>::appendTo(const QByteArray & ba, QChar *& out);
  fn C_ZN13QConcatenableI10QByteArrayE8appendToERKS0_RP5QChar(arg0: *mut c_void, arg1: *mut c_void);
  // proto: static void QConcatenable<QByteArray>::appendTo(const QByteArray & ba, char *& out);
  fn C_ZN13QConcatenableI10QByteArrayE8appendToERKS0_RPc(arg0: *mut c_void, arg1: *mut c_char);
  // proto: static int QConcatenable<QByteArray>::size(const QByteArray & ba);
  fn C_ZN13QConcatenableI10QByteArrayE4sizeERKS0_(arg0: *mut c_void) -> c_int;
  fn QConcatenableLQChar__SpecialCharacterG_Class_Size() -> c_int;
  fn QConcatenableLQCharG_Class_Size() -> c_int;
  // proto: static int QConcatenable<QChar>::size(const QChar );
  fn C_ZN13QConcatenableI5QCharE4sizeES0_(arg0: *mut c_void) -> c_int;
  // proto: static void QConcatenable<QChar>::appendTo(const QChar c, QChar *& out);
  fn C_ZN13QConcatenableI5QCharE8appendToES0_RPS0_(arg0: *mut c_void, arg1: *mut c_void);
  fn QStringBuilderLQByteArray_EQByteArrayG_Class_Size() -> c_int;
  // proto:  void QStringBuilder<QByteArray, QByteArray>::QStringBuilder(const QByteArray & a_, const QByteArray & b_);
  fn C_ZN14QStringBuilderI10QByteArrayS0_EC2ERKS0_S3_(arg0: *mut c_void, arg1: *mut c_void) -> u64;
  fn QConcatenableLconstEcharEPG_Class_Size() -> c_int;
  // proto: static void QConcatenable<const char *>::appendTo(const char * a, QChar *& out);
  fn C_ZN13QConcatenableIPKcE8appendToES1_RP5QChar(arg0: *mut c_char, arg1: *mut c_void);
  // proto: static void QConcatenable<const char *>::appendTo(const char * a, char *& out);
  fn C_ZN13QConcatenableIPKcE8appendToES1_RPc(arg0: *mut c_char, arg1: *mut c_char);
  // proto: static int QConcatenable<const char *>::size(const char * a);
  fn C_ZN13QConcatenableIPKcE4sizeES1_(arg0: *mut c_char) -> c_int;
  fn QStringBuilderLQString_EQStringG_Class_Size() -> c_int;
  // proto:  void QStringBuilder<QString, QString>::QStringBuilder(const QString & a_, const QString & b_);
  fn C_ZN14QStringBuilderI7QStringS0_EC2ERKS0_S3_(arg0: *mut c_void, arg1: *mut c_void) -> u64;
} // <= ext block end

// body block begin =>
// class sizeof(QConcatenableLQLatin1StringG)=1
#[derive(Default)]
pub struct QConcatenableLQLatin1StringG {
  qbase: QAbstractConcatenable,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QConcatenableLQCharRefG)=1
#[derive(Default)]
pub struct QConcatenableLQCharRefG {
  qbase: QAbstractConcatenable,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QConcatenableLQStringRefG)=1
#[derive(Default)]
pub struct QConcatenableLQStringRefG {
  qbase: QAbstractConcatenable,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QAbstractConcatenable)=1
#[derive(Default)]
pub struct QAbstractConcatenable {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QConcatenableLQStringG)=1
#[derive(Default)]
pub struct QConcatenableLQStringG {
  qbase: QAbstractConcatenable,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QConcatenableLQLatin1CharG)=1
#[derive(Default)]
pub struct QConcatenableLQLatin1CharG {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QConcatenableLcharG)=1
#[derive(Default)]
pub struct QConcatenableLcharG {
  qbase: QAbstractConcatenable,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QConcatenableLQByteArrayG)=1
#[derive(Default)]
pub struct QConcatenableLQByteArrayG {
  qbase: QAbstractConcatenable,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QConcatenableLQChar__SpecialCharacterG)=1
#[derive(Default)]
pub struct QConcatenableLQChar__SpecialCharacterG {
  qbase: QAbstractConcatenable,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QConcatenableLQCharG)=1
#[derive(Default)]
pub struct QConcatenableLQCharG {
  qbase: QAbstractConcatenable,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QStringBuilderLQByteArray_EQByteArrayG)=1
#[derive(Default)]
pub struct QStringBuilderLQByteArray_EQByteArrayG {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QConcatenableLconstEcharEPG)=1
#[derive(Default)]
pub struct QConcatenableLconstEcharEPG {
  qbase: QAbstractConcatenable,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QStringBuilderLQString_EQStringG)=1
#[derive(Default)]
pub struct QStringBuilderLQString_EQStringG {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QConcatenableLQLatin1StringG {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QConcatenableLQLatin1StringG {
    return QConcatenableLQLatin1StringG{qbase: QAbstractConcatenable::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QConcatenableLQLatin1StringG {
  type Target = QAbstractConcatenable;

  fn deref(&self) -> &QAbstractConcatenable {
    return & self.qbase;
  }
}
impl AsRef<QAbstractConcatenable> for QConcatenableLQLatin1StringG {
  fn as_ref(& self) -> & QAbstractConcatenable {
    return & self.qbase;
  }
}
impl /*struct*/ QConcatenableLQCharRefG {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QConcatenableLQCharRefG {
    return QConcatenableLQCharRefG{qbase: QAbstractConcatenable::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QConcatenableLQCharRefG {
  type Target = QAbstractConcatenable;

  fn deref(&self) -> &QAbstractConcatenable {
    return & self.qbase;
  }
}
impl AsRef<QAbstractConcatenable> for QConcatenableLQCharRefG {
  fn as_ref(& self) -> & QAbstractConcatenable {
    return & self.qbase;
  }
}
  // proto: static void QConcatenable<QCharRef>::appendTo(QCharRef c, QChar *& out);
impl /*struct*/ QConcatenableLQCharRefG {
  pub fn appendTo_s<RetType, T: QConcatenableLQCharRefG_appendTo_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.appendTo_s();
    // return 1;
  }
}

pub trait QConcatenableLQCharRefG_appendTo_s<RetType> {
  fn appendTo_s(self ) -> RetType;
}

  // proto: static void QConcatenable<QCharRef>::appendTo(QCharRef c, QChar *& out);
impl<'a> /*trait*/ QConcatenableLQCharRefG_appendTo_s<()> for (QCharRef, &'a QChar) {
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
impl /*struct*/ QConcatenableLQCharRefG {
  pub fn size_s<RetType, T: QConcatenableLQCharRefG_size_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.size_s();
    // return 1;
  }
}

pub trait QConcatenableLQCharRefG_size_s<RetType> {
  fn size_s(self ) -> RetType;
}

  // proto: static int QConcatenable<QCharRef>::size(QCharRef );
impl<'a> /*trait*/ QConcatenableLQCharRefG_size_s<i32> for (QCharRef) {
  fn size_s(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QConcatenableI8QCharRefE4sizeES0_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN13QConcatenableI8QCharRefE4sizeES0_(arg0)};
    return ret as i32; // 1
    // return 1;
  }
}

impl /*struct*/ QConcatenableLQStringRefG {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QConcatenableLQStringRefG {
    return QConcatenableLQStringRefG{qbase: QAbstractConcatenable::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QConcatenableLQStringRefG {
  type Target = QAbstractConcatenable;

  fn deref(&self) -> &QAbstractConcatenable {
    return & self.qbase;
  }
}
impl AsRef<QAbstractConcatenable> for QConcatenableLQStringRefG {
  fn as_ref(& self) -> & QAbstractConcatenable {
    return & self.qbase;
  }
}
impl /*struct*/ QAbstractConcatenable {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QAbstractConcatenable {
    return QAbstractConcatenable{qclsinst: qthis, ..Default::default()};
  }
}
impl /*struct*/ QConcatenableLQStringG {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QConcatenableLQStringG {
    return QConcatenableLQStringG{qbase: QAbstractConcatenable::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QConcatenableLQStringG {
  type Target = QAbstractConcatenable;

  fn deref(&self) -> &QAbstractConcatenable {
    return & self.qbase;
  }
}
impl AsRef<QAbstractConcatenable> for QConcatenableLQStringG {
  fn as_ref(& self) -> & QAbstractConcatenable {
    return & self.qbase;
  }
}
  // proto: static int QConcatenable<QString>::size(const QString & a);
impl /*struct*/ QConcatenableLQStringG {
  pub fn size_s<RetType, T: QConcatenableLQStringG_size_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.size_s();
    // return 1;
  }
}

pub trait QConcatenableLQStringG_size_s<RetType> {
  fn size_s(self ) -> RetType;
}

  // proto: static int QConcatenable<QString>::size(const QString & a);
impl<'a> /*trait*/ QConcatenableLQStringG_size_s<i32> for (&'a QString) {
  fn size_s(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QConcatenableI7QStringE4sizeERKS0_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN13QConcatenableI7QStringE4sizeERKS0_(arg0)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto: static void QConcatenable<QString>::appendTo(const QString & a, QChar *& out);
impl /*struct*/ QConcatenableLQStringG {
  pub fn appendTo_s<RetType, T: QConcatenableLQStringG_appendTo_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.appendTo_s();
    // return 1;
  }
}

pub trait QConcatenableLQStringG_appendTo_s<RetType> {
  fn appendTo_s(self ) -> RetType;
}

  // proto: static void QConcatenable<QString>::appendTo(const QString & a, QChar *& out);
impl<'a> /*trait*/ QConcatenableLQStringG_appendTo_s<()> for (&'a QString, &'a QChar) {
  fn appendTo_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QConcatenableI7QStringE8appendToERKS0_RP5QChar()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {C_ZN13QConcatenableI7QStringE8appendToERKS0_RP5QChar(arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QConcatenableLQLatin1CharG {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QConcatenableLQLatin1CharG {
    return QConcatenableLQLatin1CharG{qclsinst: qthis, ..Default::default()};
  }
}
impl /*struct*/ QConcatenableLcharG {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QConcatenableLcharG {
    return QConcatenableLcharG{qbase: QAbstractConcatenable::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QConcatenableLcharG {
  type Target = QAbstractConcatenable;

  fn deref(&self) -> &QAbstractConcatenable {
    return & self.qbase;
  }
}
impl AsRef<QAbstractConcatenable> for QConcatenableLcharG {
  fn as_ref(& self) -> & QAbstractConcatenable {
    return & self.qbase;
  }
}
  // proto: static void QConcatenable<char>::appendTo(const char c, QChar *& out);
impl /*struct*/ QConcatenableLcharG {
  pub fn appendTo_s<RetType, T: QConcatenableLcharG_appendTo_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.appendTo_s();
    // return 1;
  }
}

pub trait QConcatenableLcharG_appendTo_s<RetType> {
  fn appendTo_s(self ) -> RetType;
}

  // proto: static void QConcatenable<char>::appendTo(const char c, QChar *& out);
impl<'a> /*trait*/ QConcatenableLcharG_appendTo_s<()> for (i8, &'a QChar) {
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
impl /*struct*/ QConcatenableLcharG {
  pub fn size_s<RetType, T: QConcatenableLcharG_size_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.size_s();
    // return 1;
  }
}

pub trait QConcatenableLcharG_size_s<RetType> {
  fn size_s(self ) -> RetType;
}

  // proto: static int QConcatenable<char>::size(const char );
impl<'a> /*trait*/ QConcatenableLcharG_size_s<i32> for (i8) {
  fn size_s(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QConcatenableIcE4sizeEc()};
    let arg0 = self  as c_char;
    let mut ret = unsafe {C_ZN13QConcatenableIcE4sizeEc(arg0)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto: static void QConcatenable<char>::appendTo(const char c, char *& out);
impl<'a> /*trait*/ QConcatenableLcharG_appendTo_s<()> for (i8, &'a mut String) {
  fn appendTo_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QConcatenableIcE8appendToEcRPc()};
    let arg0 = self.0  as c_char;
    let arg1 = self.1.as_ptr()  as *mut c_char;
     unsafe {C_ZN13QConcatenableIcE8appendToEcRPc(arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QConcatenableLQByteArrayG {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QConcatenableLQByteArrayG {
    return QConcatenableLQByteArrayG{qbase: QAbstractConcatenable::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QConcatenableLQByteArrayG {
  type Target = QAbstractConcatenable;

  fn deref(&self) -> &QAbstractConcatenable {
    return & self.qbase;
  }
}
impl AsRef<QAbstractConcatenable> for QConcatenableLQByteArrayG {
  fn as_ref(& self) -> & QAbstractConcatenable {
    return & self.qbase;
  }
}
  // proto: static void QConcatenable<QByteArray>::appendTo(const QByteArray & ba, QChar *& out);
impl /*struct*/ QConcatenableLQByteArrayG {
  pub fn appendTo_s<RetType, T: QConcatenableLQByteArrayG_appendTo_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.appendTo_s();
    // return 1;
  }
}

pub trait QConcatenableLQByteArrayG_appendTo_s<RetType> {
  fn appendTo_s(self ) -> RetType;
}

  // proto: static void QConcatenable<QByteArray>::appendTo(const QByteArray & ba, QChar *& out);
impl<'a> /*trait*/ QConcatenableLQByteArrayG_appendTo_s<()> for (&'a QByteArray, &'a QChar) {
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
impl<'a> /*trait*/ QConcatenableLQByteArrayG_appendTo_s<()> for (&'a QByteArray, &'a mut String) {
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
impl /*struct*/ QConcatenableLQByteArrayG {
  pub fn size_s<RetType, T: QConcatenableLQByteArrayG_size_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.size_s();
    // return 1;
  }
}

pub trait QConcatenableLQByteArrayG_size_s<RetType> {
  fn size_s(self ) -> RetType;
}

  // proto: static int QConcatenable<QByteArray>::size(const QByteArray & ba);
impl<'a> /*trait*/ QConcatenableLQByteArrayG_size_s<i32> for (&'a QByteArray) {
  fn size_s(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QConcatenableI10QByteArrayE4sizeERKS0_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN13QConcatenableI10QByteArrayE4sizeERKS0_(arg0)};
    return ret as i32; // 1
    // return 1;
  }
}

impl /*struct*/ QConcatenableLQChar__SpecialCharacterG {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QConcatenableLQChar__SpecialCharacterG {
    return QConcatenableLQChar__SpecialCharacterG{qbase: QAbstractConcatenable::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QConcatenableLQChar__SpecialCharacterG {
  type Target = QAbstractConcatenable;

  fn deref(&self) -> &QAbstractConcatenable {
    return & self.qbase;
  }
}
impl AsRef<QAbstractConcatenable> for QConcatenableLQChar__SpecialCharacterG {
  fn as_ref(& self) -> & QAbstractConcatenable {
    return & self.qbase;
  }
}
impl /*struct*/ QConcatenableLQCharG {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QConcatenableLQCharG {
    return QConcatenableLQCharG{qbase: QAbstractConcatenable::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QConcatenableLQCharG {
  type Target = QAbstractConcatenable;

  fn deref(&self) -> &QAbstractConcatenable {
    return & self.qbase;
  }
}
impl AsRef<QAbstractConcatenable> for QConcatenableLQCharG {
  fn as_ref(& self) -> & QAbstractConcatenable {
    return & self.qbase;
  }
}
  // proto: static int QConcatenable<QChar>::size(const QChar );
impl /*struct*/ QConcatenableLQCharG {
  pub fn size_s<RetType, T: QConcatenableLQCharG_size_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.size_s();
    // return 1;
  }
}

pub trait QConcatenableLQCharG_size_s<RetType> {
  fn size_s(self ) -> RetType;
}

  // proto: static int QConcatenable<QChar>::size(const QChar );
impl<'a> /*trait*/ QConcatenableLQCharG_size_s<i32> for (QChar) {
  fn size_s(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QConcatenableI5QCharE4sizeES0_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN13QConcatenableI5QCharE4sizeES0_(arg0)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto: static void QConcatenable<QChar>::appendTo(const QChar c, QChar *& out);
impl /*struct*/ QConcatenableLQCharG {
  pub fn appendTo_s<RetType, T: QConcatenableLQCharG_appendTo_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.appendTo_s();
    // return 1;
  }
}

pub trait QConcatenableLQCharG_appendTo_s<RetType> {
  fn appendTo_s(self ) -> RetType;
}

  // proto: static void QConcatenable<QChar>::appendTo(const QChar c, QChar *& out);
impl<'a> /*trait*/ QConcatenableLQCharG_appendTo_s<()> for (QChar, &'a QChar) {
  fn appendTo_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QConcatenableI5QCharE8appendToES0_RPS0_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {C_ZN13QConcatenableI5QCharE8appendToES0_RPS0_(arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QStringBuilderLQByteArray_EQByteArrayG {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QStringBuilderLQByteArray_EQByteArrayG {
    return QStringBuilderLQByteArray_EQByteArrayG{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void QStringBuilder<QByteArray, QByteArray>::QStringBuilder(const QByteArray & a_, const QByteArray & b_);
impl /*struct*/ QStringBuilderLQByteArray_EQByteArrayG {
  pub fn new<T: QStringBuilderLQByteArray_EQByteArrayG_new>(value: T) -> QStringBuilderLQByteArray_EQByteArrayG {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QStringBuilderLQByteArray_EQByteArrayG_new {
  fn new(self) -> QStringBuilderLQByteArray_EQByteArrayG;
}

  // proto:  void QStringBuilder<QByteArray, QByteArray>::QStringBuilder(const QByteArray & a_, const QByteArray & b_);
impl<'a> /*trait*/ QStringBuilderLQByteArray_EQByteArrayG_new for (&'a QByteArray, &'a QByteArray) {
  fn new(self) -> QStringBuilderLQByteArray_EQByteArrayG {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QStringBuilderI10QByteArrayS0_EC2ERKS0_S3_()};
    let ctysz: c_int = unsafe{QStringBuilderLQByteArray_EQByteArrayG_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN14QStringBuilderI10QByteArrayS0_EC2ERKS0_S3_(arg0, arg1)};
    let rsthis = QStringBuilderLQByteArray_EQByteArrayG{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QConcatenableLconstEcharEPG {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QConcatenableLconstEcharEPG {
    return QConcatenableLconstEcharEPG{qbase: QAbstractConcatenable::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QConcatenableLconstEcharEPG {
  type Target = QAbstractConcatenable;

  fn deref(&self) -> &QAbstractConcatenable {
    return & self.qbase;
  }
}
impl AsRef<QAbstractConcatenable> for QConcatenableLconstEcharEPG {
  fn as_ref(& self) -> & QAbstractConcatenable {
    return & self.qbase;
  }
}
  // proto: static void QConcatenable<const char *>::appendTo(const char * a, QChar *& out);
impl /*struct*/ QConcatenableLconstEcharEPG {
  pub fn appendTo_s<RetType, T: QConcatenableLconstEcharEPG_appendTo_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.appendTo_s();
    // return 1;
  }
}

pub trait QConcatenableLconstEcharEPG_appendTo_s<RetType> {
  fn appendTo_s(self ) -> RetType;
}

  // proto: static void QConcatenable<const char *>::appendTo(const char * a, QChar *& out);
impl<'a> /*trait*/ QConcatenableLconstEcharEPG_appendTo_s<()> for (&'a  String, &'a QChar) {
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
impl<'a> /*trait*/ QConcatenableLconstEcharEPG_appendTo_s<()> for (&'a  String, &'a mut String) {
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
impl /*struct*/ QConcatenableLconstEcharEPG {
  pub fn size_s<RetType, T: QConcatenableLconstEcharEPG_size_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.size_s();
    // return 1;
  }
}

pub trait QConcatenableLconstEcharEPG_size_s<RetType> {
  fn size_s(self ) -> RetType;
}

  // proto: static int QConcatenable<const char *>::size(const char * a);
impl<'a> /*trait*/ QConcatenableLconstEcharEPG_size_s<i32> for (&'a  String) {
  fn size_s(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QConcatenableIPKcE4sizeES1_()};
    let arg0 = self.as_ptr()  as *mut c_char;
    let mut ret = unsafe {C_ZN13QConcatenableIPKcE4sizeES1_(arg0)};
    return ret as i32; // 1
    // return 1;
  }
}

impl /*struct*/ QStringBuilderLQString_EQStringG {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QStringBuilderLQString_EQStringG {
    return QStringBuilderLQString_EQStringG{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void QStringBuilder<QString, QString>::QStringBuilder(const QString & a_, const QString & b_);
impl /*struct*/ QStringBuilderLQString_EQStringG {
  pub fn new<T: QStringBuilderLQString_EQStringG_new>(value: T) -> QStringBuilderLQString_EQStringG {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QStringBuilderLQString_EQStringG_new {
  fn new(self) -> QStringBuilderLQString_EQStringG;
}

  // proto:  void QStringBuilder<QString, QString>::QStringBuilder(const QString & a_, const QString & b_);
impl<'a> /*trait*/ QStringBuilderLQString_EQStringG_new for (&'a QString, &'a QString) {
  fn new(self) -> QStringBuilderLQString_EQStringG {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QStringBuilderI7QStringS0_EC2ERKS0_S3_()};
    let ctysz: c_int = unsafe{QStringBuilderLQString_EQStringG_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN14QStringBuilderI7QStringS0_EC2ERKS0_S3_(arg0, arg1)};
    let rsthis = QStringBuilderLQString_EQStringG{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// <= body block end

