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
  // proto:  void QPdfWriter::FreeQPdfWriter();
  fn _ZN10QPdfWriterD0Ev(qthis: *mut c_void) ;
  // proto:  void QPdfWriter::setCreator(const QString & creator);
  fn _ZN10QPdfWriter10setCreatorERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QPdfWriter::setPageSizeMM(const QSizeF & size);
  fn _ZN10QPdfWriter13setPageSizeMMERK6QSizeF(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QPdfWriter::setResolution(int resolution);
  fn _ZN10QPdfWriter13setResolutionEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QPdfWriter::NewQPdfWriter(const QPdfWriter & );
  fn _ZN10QPdfWriterC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QPdfWriter::newPage();
  fn _ZN10QPdfWriter7newPageEv(qthis: *mut c_void) -> int8_t;
  // proto:  QString QPdfWriter::title();
  fn _ZNK10QPdfWriter5titleEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QString QPdfWriter::creator();
  fn _ZNK10QPdfWriter7creatorEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QPdfWriter::resolution();
  fn _ZNK10QPdfWriter10resolutionEv(qthis: *mut c_void) -> c_int;
  // proto:  const QMetaObject * QPdfWriter::metaObject();
  fn _ZNK10QPdfWriter10metaObjectEv(qthis: *mut c_void) ;
  // proto:  void QPdfWriter::NewQPdfWriter(const QString & filename);
  fn _ZN10QPdfWriterC1ERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QPdfWriter::NewQPdfWriter(QIODevice * device);
  fn _ZN10QPdfWriterC1EP9QIODevice(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QPdfWriter::setTitle(const QString & title);
  fn _ZN10QPdfWriter8setTitleERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
}

// body block begin
// class sizeof(QPdfWriter)=1
pub struct QPdfWriter {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QPdfWriter {
  pub fn FreeQPdfWriter<RetType, T: QPdfWriter_FreeQPdfWriter<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQPdfWriter(self);
    // return 1;
  }
}

pub trait QPdfWriter_FreeQPdfWriter<RetType> {
  fn FreeQPdfWriter(self, rsthis: &mut QPdfWriter) -> RetType;
}

// proto:  void QPdfWriter::FreeQPdfWriter();
impl<'a> /*trait*/ QPdfWriter_FreeQPdfWriter<()> for () {
  fn FreeQPdfWriter(self, rsthis: &mut QPdfWriter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPdfWriterD0Ev()};
     unsafe {_ZN10QPdfWriterD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QPdfWriter {
  pub fn setCreator<RetType, T: QPdfWriter_setCreator<RetType>>(&mut self, value: T) -> RetType {
    return value.setCreator(self);
    // return 1;
  }
}

pub trait QPdfWriter_setCreator<RetType> {
  fn setCreator(self, rsthis: &mut QPdfWriter) -> RetType;
}

// proto:  void QPdfWriter::setCreator(const QString & creator);
impl<'a> /*trait*/ QPdfWriter_setCreator<()> for (&'a  QString) {
  fn setCreator(self, rsthis: &mut QPdfWriter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPdfWriter10setCreatorERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QPdfWriter10setCreatorERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QPdfWriter {
  pub fn setPageSizeMM<RetType, T: QPdfWriter_setPageSizeMM<RetType>>(&mut self, value: T) -> RetType {
    return value.setPageSizeMM(self);
    // return 1;
  }
}

pub trait QPdfWriter_setPageSizeMM<RetType> {
  fn setPageSizeMM(self, rsthis: &mut QPdfWriter) -> RetType;
}

// proto:  void QPdfWriter::setPageSizeMM(const QSizeF & size);
impl<'a> /*trait*/ QPdfWriter_setPageSizeMM<()> for (&'a  QSizeF) {
  fn setPageSizeMM(self, rsthis: &mut QPdfWriter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPdfWriter13setPageSizeMMERK6QSizeF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QPdfWriter13setPageSizeMMERK6QSizeF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QPdfWriter {
  pub fn setResolution<RetType, T: QPdfWriter_setResolution<RetType>>(&mut self, value: T) -> RetType {
    return value.setResolution(self);
    // return 1;
  }
}

pub trait QPdfWriter_setResolution<RetType> {
  fn setResolution(self, rsthis: &mut QPdfWriter) -> RetType;
}

// proto:  void QPdfWriter::setResolution(int resolution);
impl<'a> /*trait*/ QPdfWriter_setResolution<()> for (i32) {
  fn setResolution(self, rsthis: &mut QPdfWriter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPdfWriter13setResolutionEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN10QPdfWriter13setResolutionEi(rsthis.qclsinst, arg0)};
    // return 1;
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
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN10QPdfWriterC1ERKS_(qthis, arg0)};
    let rsthis = QPdfWriter{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QPdfWriter {
  pub fn newPage<RetType, T: QPdfWriter_newPage<RetType>>(&mut self, value: T) -> RetType {
    return value.newPage(self);
    // return 1;
  }
}

pub trait QPdfWriter_newPage<RetType> {
  fn newPage(self, rsthis: &mut QPdfWriter) -> RetType;
}

// proto:  bool QPdfWriter::newPage();
impl<'a> /*trait*/ QPdfWriter_newPage<i8> for () {
  fn newPage(self, rsthis: &mut QPdfWriter) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPdfWriter7newPageEv()};
    let mut ret = unsafe {_ZN10QPdfWriter7newPageEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QPdfWriter {
  pub fn title<RetType, T: QPdfWriter_title<RetType>>(&mut self, value: T) -> RetType {
    return value.title(self);
    // return 1;
  }
}

pub trait QPdfWriter_title<RetType> {
  fn title(self, rsthis: &mut QPdfWriter) -> RetType;
}

// proto:  QString QPdfWriter::title();
impl<'a> /*trait*/ QPdfWriter_title<QString> for () {
  fn title(self, rsthis: &mut QPdfWriter) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QPdfWriter5titleEv()};
    let mut ret = unsafe {_ZNK10QPdfWriter5titleEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPdfWriter {
  pub fn creator<RetType, T: QPdfWriter_creator<RetType>>(&mut self, value: T) -> RetType {
    return value.creator(self);
    // return 1;
  }
}

pub trait QPdfWriter_creator<RetType> {
  fn creator(self, rsthis: &mut QPdfWriter) -> RetType;
}

// proto:  QString QPdfWriter::creator();
impl<'a> /*trait*/ QPdfWriter_creator<QString> for () {
  fn creator(self, rsthis: &mut QPdfWriter) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QPdfWriter7creatorEv()};
    let mut ret = unsafe {_ZNK10QPdfWriter7creatorEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPdfWriter {
  pub fn resolution<RetType, T: QPdfWriter_resolution<RetType>>(&mut self, value: T) -> RetType {
    return value.resolution(self);
    // return 1;
  }
}

pub trait QPdfWriter_resolution<RetType> {
  fn resolution(self, rsthis: &mut QPdfWriter) -> RetType;
}

// proto:  int QPdfWriter::resolution();
impl<'a> /*trait*/ QPdfWriter_resolution<i32> for () {
  fn resolution(self, rsthis: &mut QPdfWriter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QPdfWriter10resolutionEv()};
    let mut ret = unsafe {_ZNK10QPdfWriter10resolutionEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QPdfWriter {
  pub fn metaObject<RetType, T: QPdfWriter_metaObject<RetType>>(&mut self, value: T) -> RetType {
    return value.metaObject(self);
    // return 1;
  }
}

pub trait QPdfWriter_metaObject<RetType> {
  fn metaObject(self, rsthis: &mut QPdfWriter) -> RetType;
}

// proto:  const QMetaObject * QPdfWriter::metaObject();
impl<'a> /*trait*/ QPdfWriter_metaObject<()> for () {
  fn metaObject(self, rsthis: &mut QPdfWriter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QPdfWriter10metaObjectEv()};
     unsafe {_ZNK10QPdfWriter10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto: void QPdfWriter::NewQPdfWriter(const QString & filename);
impl<'a> /*trait*/ QPdfWriter_NewQPdfWriter for (&'a  QString) {
  fn NewQPdfWriter(self) -> QPdfWriter {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPdfWriterC1ERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
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
  pub fn setTitle<RetType, T: QPdfWriter_setTitle<RetType>>(&mut self, value: T) -> RetType {
    return value.setTitle(self);
    // return 1;
  }
}

pub trait QPdfWriter_setTitle<RetType> {
  fn setTitle(self, rsthis: &mut QPdfWriter) -> RetType;
}

// proto:  void QPdfWriter::setTitle(const QString & title);
impl<'a> /*trait*/ QPdfWriter_setTitle<()> for (&'a  QString) {
  fn setTitle(self, rsthis: &mut QPdfWriter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPdfWriter8setTitleERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QPdfWriter8setTitleERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

