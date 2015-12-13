// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qtextcodec::QTextCodec;
use super::qiodevice::QIODevice;
use super::qbytearray::QByteArray;
use super::qstring::QString;
use super::qtextdocument::QTextDocument;
use super::qtextdocumentfragment::QTextDocumentFragment;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QTextDocumentWriter::setCodec(QTextCodec * codec);
  fn _ZN19QTextDocumentWriter8setCodecEP10QTextCodec(arg0: *mut c_void) -> i32;
  // proto: void QTextDocumentWriter::NewQTextDocumentWriter(QIODevice * device, const QByteArray & format);
  fn _ZN19QTextDocumentWriterC1EP9QIODeviceRK10QByteArray(qthis: *mut c_void, arg0: *mut c_void, arg1: *const c_void) -> i32;
  // proto: void QTextDocumentWriter::setFileName(const QString & fileName);
  fn _ZN19QTextDocumentWriter11setFileNameERK7QString(arg0: *const c_void) -> i32;
  // proto: QByteArray QTextDocumentWriter::format();
  fn _ZNK19QTextDocumentWriter6formatEv() -> i32;
  // proto: void QTextDocumentWriter::setDevice(QIODevice * device);
  fn _ZN19QTextDocumentWriter9setDeviceEP9QIODevice(arg0: *mut c_void) -> i32;
  // proto: void QTextDocumentWriter::NewQTextDocumentWriter(const QString & fileName, const QByteArray & format);
  fn _ZN19QTextDocumentWriterC1ERK7QStringRK10QByteArray(qthis: *mut c_void, arg0: *const c_void, arg1: *const c_void) -> i32;
  // proto: void QTextDocumentWriter::setFormat(const QByteArray & format);
  fn _ZN19QTextDocumentWriter9setFormatERK10QByteArray(arg0: *const c_void) -> i32;
  // proto: bool QTextDocumentWriter::write(const QTextDocument * document);
  fn _ZN19QTextDocumentWriter5writeEPK13QTextDocument(arg0: *const c_void) -> i32;
  // proto: bool QTextDocumentWriter::write(const QTextDocumentFragment & fragment);
  fn _ZN19QTextDocumentWriter5writeERK21QTextDocumentFragment(arg0: *const c_void) -> i32;
  // proto: void QTextDocumentWriter::NewQTextDocumentWriter();
  fn _ZN19QTextDocumentWriterC1Ev(qthis: *mut c_void) -> i32;
  // proto: QTextCodec * QTextDocumentWriter::codec();
  fn _ZNK19QTextDocumentWriter5codecEv() -> i32;
  // proto: QString QTextDocumentWriter::fileName();
  fn _ZNK19QTextDocumentWriter8fileNameEv() -> i32;
  // proto: QList<QByteArray> QTextDocumentWriter::supportedDocumentFormats();
  fn _ZN19QTextDocumentWriter24supportedDocumentFormatsEv() -> i32;
  // proto: QIODevice * QTextDocumentWriter::device();
  fn _ZNK19QTextDocumentWriter6deviceEv() -> i32;
  // proto: void QTextDocumentWriter::FreeQTextDocumentWriter();
  fn _ZN19QTextDocumentWriterD0Ev() -> i32;
  // proto: void QTextDocumentWriter::NewQTextDocumentWriter(const QTextDocumentWriter & );
  fn _ZN19QTextDocumentWriterC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
}

// body block begin
// class sizeof(QTextDocumentWriter)=8
pub struct QTextDocumentWriter {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTextDocumentWriter {
  pub fn setCodec<T: QTextDocumentWriter_setCodec>(&mut self, value: T) -> i32 {
    value.setCodec(self);
    return 1;
  }
}

pub trait QTextDocumentWriter_setCodec {
  fn setCodec(self, this: &mut QTextDocumentWriter) -> i32;
}

// proto: void QTextDocumentWriter::setCodec(QTextCodec * codec);
impl<'a> /*trait*/ QTextDocumentWriter_setCodec for (&'a mut QTextCodec) {
  fn setCodec(self, this: &mut QTextDocumentWriter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QTextDocumentWriter8setCodecEP10QTextCodec()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN19QTextDocumentWriter8setCodecEP10QTextCodec(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextDocumentWriter {
  pub fn NewQTextDocumentWriter<T: QTextDocumentWriter_NewQTextDocumentWriter>(value: T) -> QTextDocumentWriter {
    let rsthis = value.NewQTextDocumentWriter();
    return rsthis;
    // return 1;
  }
}

pub trait QTextDocumentWriter_NewQTextDocumentWriter {
  fn NewQTextDocumentWriter(self) -> QTextDocumentWriter;
}

// proto: void QTextDocumentWriter::NewQTextDocumentWriter(QIODevice * device, const QByteArray & format);
impl<'a> /*trait*/ QTextDocumentWriter_NewQTextDocumentWriter for (&'a mut QIODevice, &'a  QByteArray) {
  fn NewQTextDocumentWriter(self) -> QTextDocumentWriter {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QTextDocumentWriterC1EP9QIODeviceRK10QByteArray()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN19QTextDocumentWriterC1EP9QIODeviceRK10QByteArray(qthis, arg0, arg1)};
    let rsthis = QTextDocumentWriter{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTextDocumentWriter {
  pub fn setFileName<T: QTextDocumentWriter_setFileName>(&mut self, value: T) -> i32 {
    value.setFileName(self);
    return 1;
  }
}

pub trait QTextDocumentWriter_setFileName {
  fn setFileName(self, this: &mut QTextDocumentWriter) -> i32;
}

// proto: void QTextDocumentWriter::setFileName(const QString & fileName);
impl<'a> /*trait*/ QTextDocumentWriter_setFileName for (&'a  QString) {
  fn setFileName(self, this: &mut QTextDocumentWriter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QTextDocumentWriter11setFileNameERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN19QTextDocumentWriter11setFileNameERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextDocumentWriter {
  pub fn format<T: QTextDocumentWriter_format>(&mut self, value: T) -> i32 {
    value.format(self);
    return 1;
  }
}

pub trait QTextDocumentWriter_format {
  fn format(self, this: &mut QTextDocumentWriter) -> i32;
}

// proto: QByteArray QTextDocumentWriter::format();
impl<'a> /*trait*/ QTextDocumentWriter_format for () {
  fn format(self, this: &mut QTextDocumentWriter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QTextDocumentWriter6formatEv()};
    unsafe {_ZNK19QTextDocumentWriter6formatEv()};
    return 1;
  }
}

impl /*struct*/ QTextDocumentWriter {
  pub fn setDevice<T: QTextDocumentWriter_setDevice>(&mut self, value: T) -> i32 {
    value.setDevice(self);
    return 1;
  }
}

pub trait QTextDocumentWriter_setDevice {
  fn setDevice(self, this: &mut QTextDocumentWriter) -> i32;
}

// proto: void QTextDocumentWriter::setDevice(QIODevice * device);
impl<'a> /*trait*/ QTextDocumentWriter_setDevice for (&'a mut QIODevice) {
  fn setDevice(self, this: &mut QTextDocumentWriter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QTextDocumentWriter9setDeviceEP9QIODevice()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN19QTextDocumentWriter9setDeviceEP9QIODevice(arg0)};
    return 1;
  }
}

// proto: void QTextDocumentWriter::NewQTextDocumentWriter(const QString & fileName, const QByteArray & format);
impl<'a> /*trait*/ QTextDocumentWriter_NewQTextDocumentWriter for (&'a  QString, &'a  QByteArray) {
  fn NewQTextDocumentWriter(self) -> QTextDocumentWriter {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QTextDocumentWriterC1ERK7QStringRK10QByteArray()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN19QTextDocumentWriterC1ERK7QStringRK10QByteArray(qthis, arg0, arg1)};
    let rsthis = QTextDocumentWriter{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTextDocumentWriter {
  pub fn setFormat<T: QTextDocumentWriter_setFormat>(&mut self, value: T) -> i32 {
    value.setFormat(self);
    return 1;
  }
}

pub trait QTextDocumentWriter_setFormat {
  fn setFormat(self, this: &mut QTextDocumentWriter) -> i32;
}

// proto: void QTextDocumentWriter::setFormat(const QByteArray & format);
impl<'a> /*trait*/ QTextDocumentWriter_setFormat for (&'a  QByteArray) {
  fn setFormat(self, this: &mut QTextDocumentWriter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QTextDocumentWriter9setFormatERK10QByteArray()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN19QTextDocumentWriter9setFormatERK10QByteArray(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextDocumentWriter {
  pub fn write<T: QTextDocumentWriter_write>(&mut self, value: T) -> i32 {
    value.write(self);
    return 1;
  }
}

pub trait QTextDocumentWriter_write {
  fn write(self, this: &mut QTextDocumentWriter) -> i32;
}

// proto: bool QTextDocumentWriter::write(const QTextDocument * document);
impl<'a> /*trait*/ QTextDocumentWriter_write for (&'a  QTextDocument) {
  fn write(self, this: &mut QTextDocumentWriter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QTextDocumentWriter5writeEPK13QTextDocument()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN19QTextDocumentWriter5writeEPK13QTextDocument(arg0)};
    return 1;
  }
}

// proto: bool QTextDocumentWriter::write(const QTextDocumentFragment & fragment);
impl<'a> /*trait*/ QTextDocumentWriter_write for (&'a  QTextDocumentFragment) {
  fn write(self, this: &mut QTextDocumentWriter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QTextDocumentWriter5writeERK21QTextDocumentFragment()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN19QTextDocumentWriter5writeERK21QTextDocumentFragment(arg0)};
    return 1;
  }
}

// proto: void QTextDocumentWriter::NewQTextDocumentWriter();
impl<'a> /*trait*/ QTextDocumentWriter_NewQTextDocumentWriter for () {
  fn NewQTextDocumentWriter(self) -> QTextDocumentWriter {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QTextDocumentWriterC1Ev()};
    unsafe {_ZN19QTextDocumentWriterC1Ev(qthis)};
    let rsthis = QTextDocumentWriter{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTextDocumentWriter {
  pub fn codec<T: QTextDocumentWriter_codec>(&mut self, value: T) -> i32 {
    value.codec(self);
    return 1;
  }
}

pub trait QTextDocumentWriter_codec {
  fn codec(self, this: &mut QTextDocumentWriter) -> i32;
}

// proto: QTextCodec * QTextDocumentWriter::codec();
impl<'a> /*trait*/ QTextDocumentWriter_codec for () {
  fn codec(self, this: &mut QTextDocumentWriter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QTextDocumentWriter5codecEv()};
    unsafe {_ZNK19QTextDocumentWriter5codecEv()};
    return 1;
  }
}

impl /*struct*/ QTextDocumentWriter {
  pub fn fileName<T: QTextDocumentWriter_fileName>(&mut self, value: T) -> i32 {
    value.fileName(self);
    return 1;
  }
}

pub trait QTextDocumentWriter_fileName {
  fn fileName(self, this: &mut QTextDocumentWriter) -> i32;
}

// proto: QString QTextDocumentWriter::fileName();
impl<'a> /*trait*/ QTextDocumentWriter_fileName for () {
  fn fileName(self, this: &mut QTextDocumentWriter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QTextDocumentWriter8fileNameEv()};
    unsafe {_ZNK19QTextDocumentWriter8fileNameEv()};
    return 1;
  }
}

impl /*struct*/ QTextDocumentWriter {
  pub fn supportedDocumentFormats<T: QTextDocumentWriter_supportedDocumentFormats>(&mut self, value: T) -> i32 {
    value.supportedDocumentFormats(self);
    return 1;
  }
}

pub trait QTextDocumentWriter_supportedDocumentFormats {
  fn supportedDocumentFormats(self, this: &mut QTextDocumentWriter) -> i32;
}

// proto: QList<QByteArray> QTextDocumentWriter::supportedDocumentFormats();
impl<'a> /*trait*/ QTextDocumentWriter_supportedDocumentFormats for () {
  fn supportedDocumentFormats(self, this: &mut QTextDocumentWriter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QTextDocumentWriter24supportedDocumentFormatsEv()};
    unsafe {_ZN19QTextDocumentWriter24supportedDocumentFormatsEv()};
    return 1;
  }
}

impl /*struct*/ QTextDocumentWriter {
  pub fn device<T: QTextDocumentWriter_device>(&mut self, value: T) -> i32 {
    value.device(self);
    return 1;
  }
}

pub trait QTextDocumentWriter_device {
  fn device(self, this: &mut QTextDocumentWriter) -> i32;
}

// proto: QIODevice * QTextDocumentWriter::device();
impl<'a> /*trait*/ QTextDocumentWriter_device for () {
  fn device(self, this: &mut QTextDocumentWriter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QTextDocumentWriter6deviceEv()};
    unsafe {_ZNK19QTextDocumentWriter6deviceEv()};
    return 1;
  }
}

impl /*struct*/ QTextDocumentWriter {
  pub fn FreeQTextDocumentWriter<T: QTextDocumentWriter_FreeQTextDocumentWriter>(&mut self, value: T) -> i32 {
    value.FreeQTextDocumentWriter(self);
    return 1;
  }
}

pub trait QTextDocumentWriter_FreeQTextDocumentWriter {
  fn FreeQTextDocumentWriter(self, this: &mut QTextDocumentWriter) -> i32;
}

// proto: void QTextDocumentWriter::FreeQTextDocumentWriter();
impl<'a> /*trait*/ QTextDocumentWriter_FreeQTextDocumentWriter for () {
  fn FreeQTextDocumentWriter(self, this: &mut QTextDocumentWriter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QTextDocumentWriterD0Ev()};
    unsafe {_ZN19QTextDocumentWriterD0Ev()};
    return 1;
  }
}

// proto: void QTextDocumentWriter::NewQTextDocumentWriter(const QTextDocumentWriter & );
impl<'a> /*trait*/ QTextDocumentWriter_NewQTextDocumentWriter for (&'a  QTextDocumentWriter) {
  fn NewQTextDocumentWriter(self) -> QTextDocumentWriter {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QTextDocumentWriterC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN19QTextDocumentWriterC1ERKS_(qthis, arg0)};
    let rsthis = QTextDocumentWriter{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

