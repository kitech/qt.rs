// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstring::QString;
use super::qsizef::QSizeF;
use super::qiodevice::QIODevice;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QPdfWriter::FreeQPdfWriter();
  fn _ZN10QPdfWriterD0Ev() -> i32;
  // proto: void QPdfWriter::setCreator(const QString & creator);
  fn _ZN10QPdfWriter10setCreatorERK7QString(arg0: *const c_void) -> i32;
  // proto: void QPdfWriter::setPageSizeMM(const QSizeF & size);
  fn _ZN10QPdfWriter13setPageSizeMMERK6QSizeF(arg0: *const c_void) -> i32;
  // proto: void QPdfWriter::setResolution(int resolution);
  fn _ZN10QPdfWriter13setResolutionEi(arg0: c_int) -> i32;
  // proto: void QPdfWriter::NewQPdfWriter(const QPdfWriter & );
  fn _ZN10QPdfWriterC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: bool QPdfWriter::newPage();
  fn _ZN10QPdfWriter7newPageEv() -> i32;
  // proto: QString QPdfWriter::title();
  fn _ZNK10QPdfWriter5titleEv() -> i32;
  // proto: QString QPdfWriter::creator();
  fn _ZNK10QPdfWriter7creatorEv() -> i32;
  // proto: int QPdfWriter::resolution();
  fn _ZNK10QPdfWriter10resolutionEv() -> i32;
  // proto: const QMetaObject * QPdfWriter::metaObject();
  fn _ZNK10QPdfWriter10metaObjectEv() -> i32;
  // proto: void QPdfWriter::NewQPdfWriter(const QString & filename);
  fn _ZN10QPdfWriterC1ERK7QString(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: void QPdfWriter::NewQPdfWriter(QIODevice * device);
  fn _ZN10QPdfWriterC1EP9QIODevice(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: void QPdfWriter::setTitle(const QString & title);
  fn _ZN10QPdfWriter8setTitleERK7QString(arg0: *const c_void) -> i32;
}

// body block begin
// class sizeof(QPdfWriter)=1
pub struct QPdfWriter {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QPdfWriter {
  pub fn FreeQPdfWriter<T: QPdfWriter_FreeQPdfWriter>(&mut self, value: T) -> i32 {
    value.FreeQPdfWriter(self);
    return 1;
  }
}

pub trait QPdfWriter_FreeQPdfWriter {
  fn FreeQPdfWriter(self, this: &mut QPdfWriter) -> i32;
}

// proto: void QPdfWriter::FreeQPdfWriter();
impl<'a> /*trait*/ QPdfWriter_FreeQPdfWriter for () {
  fn FreeQPdfWriter(self, this: &mut QPdfWriter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPdfWriterD0Ev()};
    unsafe {_ZN10QPdfWriterD0Ev()};
    return 1;
  }
}

impl /*struct*/ QPdfWriter {
  pub fn setCreator<T: QPdfWriter_setCreator>(&mut self, value: T) -> i32 {
    value.setCreator(self);
    return 1;
  }
}

pub trait QPdfWriter_setCreator {
  fn setCreator(self, this: &mut QPdfWriter) -> i32;
}

// proto: void QPdfWriter::setCreator(const QString & creator);
impl<'a> /*trait*/ QPdfWriter_setCreator for (&'a  QString) {
  fn setCreator(self, this: &mut QPdfWriter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPdfWriter10setCreatorERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN10QPdfWriter10setCreatorERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QPdfWriter {
  pub fn setPageSizeMM<T: QPdfWriter_setPageSizeMM>(&mut self, value: T) -> i32 {
    value.setPageSizeMM(self);
    return 1;
  }
}

pub trait QPdfWriter_setPageSizeMM {
  fn setPageSizeMM(self, this: &mut QPdfWriter) -> i32;
}

// proto: void QPdfWriter::setPageSizeMM(const QSizeF & size);
impl<'a> /*trait*/ QPdfWriter_setPageSizeMM for (&'a  QSizeF) {
  fn setPageSizeMM(self, this: &mut QPdfWriter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPdfWriter13setPageSizeMMERK6QSizeF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN10QPdfWriter13setPageSizeMMERK6QSizeF(arg0)};
    return 1;
  }
}

impl /*struct*/ QPdfWriter {
  pub fn setResolution<T: QPdfWriter_setResolution>(&mut self, value: T) -> i32 {
    value.setResolution(self);
    return 1;
  }
}

pub trait QPdfWriter_setResolution {
  fn setResolution(self, this: &mut QPdfWriter) -> i32;
}

// proto: void QPdfWriter::setResolution(int resolution);
impl<'a> /*trait*/ QPdfWriter_setResolution for (i32) {
  fn setResolution(self, this: &mut QPdfWriter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPdfWriter13setResolutionEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN10QPdfWriter13setResolutionEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QPdfWriter {
  pub fn NewQPdfWriter<T: QPdfWriter_NewQPdfWriter>(value: T) -> QPdfWriter {
    let rsthis = value.NewQPdfWriter();
    return rsthis;
    // return 1;
  }
}

pub trait QPdfWriter_NewQPdfWriter {
  fn NewQPdfWriter(self) -> QPdfWriter;
}

// proto: void QPdfWriter::NewQPdfWriter(const QPdfWriter & );
impl<'a> /*trait*/ QPdfWriter_NewQPdfWriter for (&'a  QPdfWriter) {
  fn NewQPdfWriter(self) -> QPdfWriter {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPdfWriterC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN10QPdfWriterC1ERKS_(qthis, arg0)};
    let rsthis = QPdfWriter{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QPdfWriter {
  pub fn newPage<T: QPdfWriter_newPage>(&mut self, value: T) -> i32 {
    value.newPage(self);
    return 1;
  }
}

pub trait QPdfWriter_newPage {
  fn newPage(self, this: &mut QPdfWriter) -> i32;
}

// proto: bool QPdfWriter::newPage();
impl<'a> /*trait*/ QPdfWriter_newPage for () {
  fn newPage(self, this: &mut QPdfWriter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPdfWriter7newPageEv()};
    unsafe {_ZN10QPdfWriter7newPageEv()};
    return 1;
  }
}

impl /*struct*/ QPdfWriter {
  pub fn title<T: QPdfWriter_title>(&mut self, value: T) -> i32 {
    value.title(self);
    return 1;
  }
}

pub trait QPdfWriter_title {
  fn title(self, this: &mut QPdfWriter) -> i32;
}

// proto: QString QPdfWriter::title();
impl<'a> /*trait*/ QPdfWriter_title for () {
  fn title(self, this: &mut QPdfWriter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QPdfWriter5titleEv()};
    unsafe {_ZNK10QPdfWriter5titleEv()};
    return 1;
  }
}

impl /*struct*/ QPdfWriter {
  pub fn creator<T: QPdfWriter_creator>(&mut self, value: T) -> i32 {
    value.creator(self);
    return 1;
  }
}

pub trait QPdfWriter_creator {
  fn creator(self, this: &mut QPdfWriter) -> i32;
}

// proto: QString QPdfWriter::creator();
impl<'a> /*trait*/ QPdfWriter_creator for () {
  fn creator(self, this: &mut QPdfWriter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QPdfWriter7creatorEv()};
    unsafe {_ZNK10QPdfWriter7creatorEv()};
    return 1;
  }
}

impl /*struct*/ QPdfWriter {
  pub fn resolution<T: QPdfWriter_resolution>(&mut self, value: T) -> i32 {
    value.resolution(self);
    return 1;
  }
}

pub trait QPdfWriter_resolution {
  fn resolution(self, this: &mut QPdfWriter) -> i32;
}

// proto: int QPdfWriter::resolution();
impl<'a> /*trait*/ QPdfWriter_resolution for () {
  fn resolution(self, this: &mut QPdfWriter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QPdfWriter10resolutionEv()};
    unsafe {_ZNK10QPdfWriter10resolutionEv()};
    return 1;
  }
}

impl /*struct*/ QPdfWriter {
  pub fn metaObject<T: QPdfWriter_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QPdfWriter_metaObject {
  fn metaObject(self, this: &mut QPdfWriter) -> i32;
}

// proto: const QMetaObject * QPdfWriter::metaObject();
impl<'a> /*trait*/ QPdfWriter_metaObject for () {
  fn metaObject(self, this: &mut QPdfWriter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QPdfWriter10metaObjectEv()};
    unsafe {_ZNK10QPdfWriter10metaObjectEv()};
    return 1;
  }
}

// proto: void QPdfWriter::NewQPdfWriter(const QString & filename);
impl<'a> /*trait*/ QPdfWriter_NewQPdfWriter for (&'a  QString) {
  fn NewQPdfWriter(self) -> QPdfWriter {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPdfWriterC1ERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN10QPdfWriterC1ERK7QString(qthis, arg0)};
    let rsthis = QPdfWriter{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QPdfWriter::NewQPdfWriter(QIODevice * device);
impl<'a> /*trait*/ QPdfWriter_NewQPdfWriter for (&'a mut QIODevice) {
  fn NewQPdfWriter(self) -> QPdfWriter {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPdfWriterC1EP9QIODevice()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN10QPdfWriterC1EP9QIODevice(qthis, arg0)};
    let rsthis = QPdfWriter{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QPdfWriter {
  pub fn setTitle<T: QPdfWriter_setTitle>(&mut self, value: T) -> i32 {
    value.setTitle(self);
    return 1;
  }
}

pub trait QPdfWriter_setTitle {
  fn setTitle(self, this: &mut QPdfWriter) -> i32;
}

// proto: void QPdfWriter::setTitle(const QString & title);
impl<'a> /*trait*/ QPdfWriter_setTitle for (&'a  QString) {
  fn setTitle(self, this: &mut QPdfWriter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPdfWriter8setTitleERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN10QPdfWriter8setTitleERK7QString(arg0)};
    return 1;
  }
}

