// auto generated, do not modify.
// created: Sat Dec 26 12:15:38 2015
// src-file: /QtGui/qpdfwriter.h
// dst-file: /src/gui/qpdfwriter.rs
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
use super::super::core::qobject::QObject; // 771
use std::ops::Deref;
use super::super::core::qstring::QString; // 771
use super::super::core::qsize::QSizeF; // 771
use super::super::core::qiodevice::QIODevice; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QPdfWriter_Class_Size() -> c_int;
  // proto:  void QPdfWriter::~QPdfWriter();
  fn _ZN10QPdfWriterD0Ev(qthis: *mut c_void);
  // proto:  void QPdfWriter::setCreator(const QString & creator);
  fn _ZN10QPdfWriter10setCreatorERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QPdfWriter::setPageSizeMM(const QSizeF & size);
  fn _ZN10QPdfWriter13setPageSizeMMERK6QSizeF(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QPdfWriter::setResolution(int resolution);
  fn _ZN10QPdfWriter13setResolutionEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QPdfWriter::QPdfWriter(const QPdfWriter & );
  fn dector_ZN10QPdfWriterC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN10QPdfWriterC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QPdfWriter::newPage();
  fn _ZN10QPdfWriter7newPageEv(qthis: *mut c_void) -> c_char;
  // proto:  QString QPdfWriter::title();
  fn _ZNK10QPdfWriter5titleEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QString QPdfWriter::creator();
  fn _ZNK10QPdfWriter7creatorEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QPdfWriter::resolution();
  fn _ZNK10QPdfWriter10resolutionEv(qthis: *mut c_void) -> c_int;
  // proto:  const QMetaObject * QPdfWriter::metaObject();
  fn _ZNK10QPdfWriter10metaObjectEv(qthis: *mut c_void);
  // proto:  void QPdfWriter::QPdfWriter(const QString & filename);
  fn dector_ZN10QPdfWriterC1ERK7QString(arg0: *mut c_void) -> *mut c_void;
  fn _ZN10QPdfWriterC1ERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QPdfWriter::QPdfWriter(QIODevice * device);
  fn dector_ZN10QPdfWriterC1EP9QIODevice(arg0: *mut c_void) -> *mut c_void;
  fn _ZN10QPdfWriterC1EP9QIODevice(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QPdfWriter::setTitle(const QString & title);
  fn _ZN10QPdfWriter8setTitleERK7QString(qthis: *mut c_void, arg0: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QPdfWriter)=1
pub struct QPdfWriter {
  qbase: QObject,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QPdfWriter {
  pub fn inheritFrom(qthis: *mut c_void) -> QPdfWriter {
    return QPdfWriter{qbase: QObject::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QPdfWriter {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QPdfWriter {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  void QPdfWriter::~QPdfWriter();
impl /*struct*/ QPdfWriter {
  pub fn Free<RetType, T: QPdfWriter_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QPdfWriter_Free<RetType> {
  fn Free(self , rsthis: & QPdfWriter) -> RetType;
}

  // proto:  void QPdfWriter::~QPdfWriter();
impl<'a> /*trait*/ QPdfWriter_Free<()> for () {
  fn Free(self , rsthis: & QPdfWriter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPdfWriterD0Ev()};
     unsafe {_ZN10QPdfWriterD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QPdfWriter::setCreator(const QString & creator);
impl /*struct*/ QPdfWriter {
  pub fn setCreator<RetType, T: QPdfWriter_setCreator<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setCreator(self);
    // return 1;
  }
}

pub trait QPdfWriter_setCreator<RetType> {
  fn setCreator(self , rsthis: & QPdfWriter) -> RetType;
}

  // proto:  void QPdfWriter::setCreator(const QString & creator);
impl<'a> /*trait*/ QPdfWriter_setCreator<()> for (&'a QString) {
  fn setCreator(self , rsthis: & QPdfWriter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPdfWriter10setCreatorERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QPdfWriter10setCreatorERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPdfWriter::setPageSizeMM(const QSizeF & size);
impl /*struct*/ QPdfWriter {
  pub fn setPageSizeMM<RetType, T: QPdfWriter_setPageSizeMM<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setPageSizeMM(self);
    // return 1;
  }
}

pub trait QPdfWriter_setPageSizeMM<RetType> {
  fn setPageSizeMM(self , rsthis: & QPdfWriter) -> RetType;
}

  // proto:  void QPdfWriter::setPageSizeMM(const QSizeF & size);
impl<'a> /*trait*/ QPdfWriter_setPageSizeMM<()> for (&'a QSizeF) {
  fn setPageSizeMM(self , rsthis: & QPdfWriter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPdfWriter13setPageSizeMMERK6QSizeF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QPdfWriter13setPageSizeMMERK6QSizeF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPdfWriter::setResolution(int resolution);
impl /*struct*/ QPdfWriter {
  pub fn setResolution<RetType, T: QPdfWriter_setResolution<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setResolution(self);
    // return 1;
  }
}

pub trait QPdfWriter_setResolution<RetType> {
  fn setResolution(self , rsthis: & QPdfWriter) -> RetType;
}

  // proto:  void QPdfWriter::setResolution(int resolution);
impl<'a> /*trait*/ QPdfWriter_setResolution<()> for (i32) {
  fn setResolution(self , rsthis: & QPdfWriter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPdfWriter13setResolutionEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN10QPdfWriter13setResolutionEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPdfWriter::QPdfWriter(const QPdfWriter & );
impl /*struct*/ QPdfWriter {
  pub fn New<T: QPdfWriter_New>(value: T) -> QPdfWriter {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QPdfWriter_New {
  fn New(self) -> QPdfWriter;
}

  // proto:  void QPdfWriter::QPdfWriter(const QPdfWriter & );
impl<'a> /*trait*/ QPdfWriter_New for (&'a QPdfWriter) {
  fn New(self) -> QPdfWriter {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPdfWriterC1ERKS_()};
    let ctysz: c_int = unsafe{QPdfWriter_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN10QPdfWriterC1ERKS_(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN10QPdfWriterC1ERKS_(arg0)};
    let rsthis = QPdfWriter{/**/qbase: QObject::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QPdfWriter::newPage();
impl /*struct*/ QPdfWriter {
  pub fn newPage<RetType, T: QPdfWriter_newPage<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.newPage(self);
    // return 1;
  }
}

pub trait QPdfWriter_newPage<RetType> {
  fn newPage(self , rsthis: & QPdfWriter) -> RetType;
}

  // proto:  bool QPdfWriter::newPage();
impl<'a> /*trait*/ QPdfWriter_newPage<i8> for () {
  fn newPage(self , rsthis: & QPdfWriter) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPdfWriter7newPageEv()};
    let mut ret = unsafe {_ZN10QPdfWriter7newPageEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QString QPdfWriter::title();
impl /*struct*/ QPdfWriter {
  pub fn title<RetType, T: QPdfWriter_title<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.title(self);
    // return 1;
  }
}

pub trait QPdfWriter_title<RetType> {
  fn title(self , rsthis: & QPdfWriter) -> RetType;
}

  // proto:  QString QPdfWriter::title();
impl<'a> /*trait*/ QPdfWriter_title<QString> for () {
  fn title(self , rsthis: & QPdfWriter) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QPdfWriter5titleEv()};
    let mut ret = unsafe {_ZNK10QPdfWriter5titleEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QPdfWriter::creator();
impl /*struct*/ QPdfWriter {
  pub fn creator<RetType, T: QPdfWriter_creator<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.creator(self);
    // return 1;
  }
}

pub trait QPdfWriter_creator<RetType> {
  fn creator(self , rsthis: & QPdfWriter) -> RetType;
}

  // proto:  QString QPdfWriter::creator();
impl<'a> /*trait*/ QPdfWriter_creator<QString> for () {
  fn creator(self , rsthis: & QPdfWriter) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QPdfWriter7creatorEv()};
    let mut ret = unsafe {_ZNK10QPdfWriter7creatorEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  int QPdfWriter::resolution();
impl /*struct*/ QPdfWriter {
  pub fn resolution<RetType, T: QPdfWriter_resolution<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.resolution(self);
    // return 1;
  }
}

pub trait QPdfWriter_resolution<RetType> {
  fn resolution(self , rsthis: & QPdfWriter) -> RetType;
}

  // proto:  int QPdfWriter::resolution();
impl<'a> /*trait*/ QPdfWriter_resolution<i32> for () {
  fn resolution(self , rsthis: & QPdfWriter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QPdfWriter10resolutionEv()};
    let mut ret = unsafe {_ZNK10QPdfWriter10resolutionEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  const QMetaObject * QPdfWriter::metaObject();
impl /*struct*/ QPdfWriter {
  pub fn metaObject<RetType, T: QPdfWriter_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QPdfWriter_metaObject<RetType> {
  fn metaObject(self , rsthis: & QPdfWriter) -> RetType;
}

  // proto:  const QMetaObject * QPdfWriter::metaObject();
impl<'a> /*trait*/ QPdfWriter_metaObject<()> for () {
  fn metaObject(self , rsthis: & QPdfWriter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QPdfWriter10metaObjectEv()};
     unsafe {_ZNK10QPdfWriter10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QPdfWriter::QPdfWriter(const QString & filename);
impl<'a> /*trait*/ QPdfWriter_New for (&'a QString) {
  fn New(self) -> QPdfWriter {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPdfWriterC1ERK7QString()};
    let ctysz: c_int = unsafe{QPdfWriter_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN10QPdfWriterC1ERK7QString(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN10QPdfWriterC1ERK7QString(arg0)};
    let rsthis = QPdfWriter{/**/qbase: QObject::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QPdfWriter::QPdfWriter(QIODevice * device);
impl<'a> /*trait*/ QPdfWriter_New for (&'a QIODevice) {
  fn New(self) -> QPdfWriter {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPdfWriterC1EP9QIODevice()};
    let ctysz: c_int = unsafe{QPdfWriter_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN10QPdfWriterC1EP9QIODevice(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN10QPdfWriterC1EP9QIODevice(arg0)};
    let rsthis = QPdfWriter{/**/qbase: QObject::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QPdfWriter::setTitle(const QString & title);
impl /*struct*/ QPdfWriter {
  pub fn setTitle<RetType, T: QPdfWriter_setTitle<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setTitle(self);
    // return 1;
  }
}

pub trait QPdfWriter_setTitle<RetType> {
  fn setTitle(self , rsthis: & QPdfWriter) -> RetType;
}

  // proto:  void QPdfWriter::setTitle(const QString & title);
impl<'a> /*trait*/ QPdfWriter_setTitle<()> for (&'a QString) {
  fn setTitle(self , rsthis: & QPdfWriter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPdfWriter8setTitleERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QPdfWriter8setTitleERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// <= body block end

