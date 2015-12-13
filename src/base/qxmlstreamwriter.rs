// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstring::QString;
use super::qtextcodec::QTextCodec;
use super::qiodevice::QIODevice;
use super::qxmlstreamattribute::QXmlStreamAttribute;
use super::qxmlstreamattributes::QXmlStreamAttributes;
use super::qxmlstreamreader::QXmlStreamReader;
use super::qbytearray::QByteArray;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  fn _ZN16QXmlStreamWriter15writeEndElementEv() -> i32;
  fn _ZN16QXmlStreamWriterC1Ev(qthis: *mut c_void) -> i32;
  fn _ZN16QXmlStreamWriter16writeEndDocumentEv() -> i32;
  fn _ZNK16QXmlStreamWriter14autoFormattingEv() -> i32;
  fn _ZN16QXmlStreamWriter18writeStartDocumentERK7QStringb(arg0: *const c_void, arg1: int8_t) -> i32;
  fn _ZN16QXmlStreamWriter8setCodecEP10QTextCodec(arg0: *mut c_void) -> i32;
  fn _ZN16QXmlStreamWriter26writeProcessingInstructionERK7QStringS2_(arg0: *const c_void, arg1: *const c_void) -> i32;
  fn _ZN16QXmlStreamWriter15writeCharactersERK7QString(arg0: *const c_void) -> i32;
  fn _ZN16QXmlStreamWriter9setDeviceEP9QIODevice(arg0: *mut c_void) -> i32;
  fn _ZN16QXmlStreamWriter18writeStartDocumentERK7QString(arg0: *const c_void) -> i32;
  fn _ZN16QXmlStreamWriter16writeTextElementERK7QStringS2_S2_(arg0: *const c_void, arg1: *const c_void, arg2: *const c_void) -> i32;
  fn _ZN16QXmlStreamWriter14writeAttributeERK7QStringS2_(arg0: *const c_void, arg1: *const c_void) -> i32;
  fn _ZN16QXmlStreamWriter17writeEmptyElementERK7QString(arg0: *const c_void) -> i32;
  fn _ZN16QXmlStreamWriter8writeDTDERK7QString(arg0: *const c_void) -> i32;
  fn _ZN16QXmlStreamWriter23setAutoFormattingIndentEi(arg0: c_int) -> i32;
  fn _ZN16QXmlStreamWriter14writeAttributeERK19QXmlStreamAttribute(arg0: *const c_void) -> i32;
  fn _ZN16QXmlStreamWriter17writeStartElementERK7QStringS2_(arg0: *const c_void, arg1: *const c_void) -> i32;
  fn _ZN16QXmlStreamWriterC1EP7QString(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  fn _ZN16QXmlStreamWriter12writeCommentERK7QString(arg0: *const c_void) -> i32;
  fn _ZNK16QXmlStreamWriter5codecEv() -> i32;
  fn _ZN16QXmlStreamWriter14writeAttributeERK7QStringS2_S2_(arg0: *const c_void, arg1: *const c_void, arg2: *const c_void) -> i32;
  fn _ZN16QXmlStreamWriter14writeNamespaceERK7QStringS2_(arg0: *const c_void, arg1: *const c_void) -> i32;
  fn _ZN16QXmlStreamWriterC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZNK16QXmlStreamWriter8hasErrorEv() -> i32;
  fn _ZN16QXmlStreamWriter10writeCDATAERK7QString(arg0: *const c_void) -> i32;
  fn _ZN16QXmlStreamWriter18writeStartDocumentEv() -> i32;
  fn _ZN16QXmlStreamWriter20writeEntityReferenceERK7QString(arg0: *const c_void) -> i32;
  fn _ZN16QXmlStreamWriter17setAutoFormattingEb(arg0: int8_t) -> i32;
  fn _ZN16QXmlStreamWriter8setCodecEPKc(arg0: *const c_char) -> i32;
  fn _ZNK16QXmlStreamWriter20autoFormattingIndentEv() -> i32;
  fn _ZN16QXmlStreamWriterD0Ev() -> i32;
  fn _ZN16QXmlStreamWriter15writeAttributesERK20QXmlStreamAttributes(arg0: *const c_void) -> i32;
  fn _ZN16QXmlStreamWriter21writeDefaultNamespaceERK7QString(arg0: *const c_void) -> i32;
  fn _ZNK16QXmlStreamWriter6deviceEv() -> i32;
  fn _ZN16QXmlStreamWriter17writeCurrentTokenERK16QXmlStreamReader(arg0: *const c_void) -> i32;
  fn _ZN16QXmlStreamWriterC1EP10QByteArray(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  fn _ZN16QXmlStreamWriter16writeTextElementERK7QStringS2_(arg0: *const c_void, arg1: *const c_void) -> i32;
  fn _ZN16QXmlStreamWriter17writeEmptyElementERK7QStringS2_(arg0: *const c_void, arg1: *const c_void) -> i32;
  fn _ZN16QXmlStreamWriterC1EP9QIODevice(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  fn _ZN16QXmlStreamWriter17writeStartElementERK7QString(arg0: *const c_void) -> i32;
}

// body block begin
// class sizeof(QXmlStreamWriter)=1
pub struct QXmlStreamWriter {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QXmlStreamWriter {
  pub fn writeEndElement<T: QXmlStreamWriter_writeEndElement>(&mut self, value: T) -> i32 {
    value.writeEndElement(self);
    return 1;
  }
}

pub trait QXmlStreamWriter_writeEndElement {
  fn writeEndElement(self, this: &mut QXmlStreamWriter) -> i32;
}

// proto: void QXmlStreamWriter::writeEndElement();
impl<'a> /*trait*/ QXmlStreamWriter_writeEndElement for () {
  fn writeEndElement(self, this: &mut QXmlStreamWriter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriter15writeEndElementEv()};
    unsafe {_ZN16QXmlStreamWriter15writeEndElementEv()};
    return 1;
  }
}

impl /*struct*/ QXmlStreamWriter {
  pub fn NewQXmlStreamWriter<T: QXmlStreamWriter_NewQXmlStreamWriter>(value: T) -> QXmlStreamWriter {
    let rsthis = value.NewQXmlStreamWriter();
    return rsthis;
    // return 1;
  }
}

pub trait QXmlStreamWriter_NewQXmlStreamWriter {
  fn NewQXmlStreamWriter(self) -> QXmlStreamWriter;
}

// proto: void QXmlStreamWriter::NewQXmlStreamWriter();
impl<'a> /*trait*/ QXmlStreamWriter_NewQXmlStreamWriter for () {
  fn NewQXmlStreamWriter(self) -> QXmlStreamWriter {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriterC1Ev()};
    unsafe {_ZN16QXmlStreamWriterC1Ev(qthis)};
    let rsthis = QXmlStreamWriter{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QXmlStreamWriter {
  pub fn writeEndDocument<T: QXmlStreamWriter_writeEndDocument>(&mut self, value: T) -> i32 {
    value.writeEndDocument(self);
    return 1;
  }
}

pub trait QXmlStreamWriter_writeEndDocument {
  fn writeEndDocument(self, this: &mut QXmlStreamWriter) -> i32;
}

// proto: void QXmlStreamWriter::writeEndDocument();
impl<'a> /*trait*/ QXmlStreamWriter_writeEndDocument for () {
  fn writeEndDocument(self, this: &mut QXmlStreamWriter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriter16writeEndDocumentEv()};
    unsafe {_ZN16QXmlStreamWriter16writeEndDocumentEv()};
    return 1;
  }
}

impl /*struct*/ QXmlStreamWriter {
  pub fn autoFormatting<T: QXmlStreamWriter_autoFormatting>(&mut self, value: T) -> i32 {
    value.autoFormatting(self);
    return 1;
  }
}

pub trait QXmlStreamWriter_autoFormatting {
  fn autoFormatting(self, this: &mut QXmlStreamWriter) -> i32;
}

// proto: bool QXmlStreamWriter::autoFormatting();
impl<'a> /*trait*/ QXmlStreamWriter_autoFormatting for () {
  fn autoFormatting(self, this: &mut QXmlStreamWriter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamWriter14autoFormattingEv()};
    unsafe {_ZNK16QXmlStreamWriter14autoFormattingEv()};
    return 1;
  }
}

impl /*struct*/ QXmlStreamWriter {
  pub fn writeStartDocument<T: QXmlStreamWriter_writeStartDocument>(&mut self, value: T) -> i32 {
    value.writeStartDocument(self);
    return 1;
  }
}

pub trait QXmlStreamWriter_writeStartDocument {
  fn writeStartDocument(self, this: &mut QXmlStreamWriter) -> i32;
}

// proto: void QXmlStreamWriter::writeStartDocument(const QString & version, bool standalone);
impl<'a> /*trait*/ QXmlStreamWriter_writeStartDocument for (&'a  QString, i8) {
  fn writeStartDocument(self, this: &mut QXmlStreamWriter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriter18writeStartDocumentERK7QStringb()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as int8_t;
    unsafe {_ZN16QXmlStreamWriter18writeStartDocumentERK7QStringb(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QXmlStreamWriter {
  pub fn setCodec<T: QXmlStreamWriter_setCodec>(&mut self, value: T) -> i32 {
    value.setCodec(self);
    return 1;
  }
}

pub trait QXmlStreamWriter_setCodec {
  fn setCodec(self, this: &mut QXmlStreamWriter) -> i32;
}

// proto: void QXmlStreamWriter::setCodec(QTextCodec * codec);
impl<'a> /*trait*/ QXmlStreamWriter_setCodec for (&'a mut QTextCodec) {
  fn setCodec(self, this: &mut QXmlStreamWriter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriter8setCodecEP10QTextCodec()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN16QXmlStreamWriter8setCodecEP10QTextCodec(arg0)};
    return 1;
  }
}

impl /*struct*/ QXmlStreamWriter {
  pub fn writeProcessingInstruction<T: QXmlStreamWriter_writeProcessingInstruction>(&mut self, value: T) -> i32 {
    value.writeProcessingInstruction(self);
    return 1;
  }
}

pub trait QXmlStreamWriter_writeProcessingInstruction {
  fn writeProcessingInstruction(self, this: &mut QXmlStreamWriter) -> i32;
}

// proto: void QXmlStreamWriter::writeProcessingInstruction(const QString & target, const QString & data);
impl<'a> /*trait*/ QXmlStreamWriter_writeProcessingInstruction for (&'a  QString, &'a  QString) {
  fn writeProcessingInstruction(self, this: &mut QXmlStreamWriter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriter26writeProcessingInstructionERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN16QXmlStreamWriter26writeProcessingInstructionERK7QStringS2_(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QXmlStreamWriter {
  pub fn writeCharacters<T: QXmlStreamWriter_writeCharacters>(&mut self, value: T) -> i32 {
    value.writeCharacters(self);
    return 1;
  }
}

pub trait QXmlStreamWriter_writeCharacters {
  fn writeCharacters(self, this: &mut QXmlStreamWriter) -> i32;
}

// proto: void QXmlStreamWriter::writeCharacters(const QString & text);
impl<'a> /*trait*/ QXmlStreamWriter_writeCharacters for (&'a  QString) {
  fn writeCharacters(self, this: &mut QXmlStreamWriter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriter15writeCharactersERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN16QXmlStreamWriter15writeCharactersERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QXmlStreamWriter {
  pub fn setDevice<T: QXmlStreamWriter_setDevice>(&mut self, value: T) -> i32 {
    value.setDevice(self);
    return 1;
  }
}

pub trait QXmlStreamWriter_setDevice {
  fn setDevice(self, this: &mut QXmlStreamWriter) -> i32;
}

// proto: void QXmlStreamWriter::setDevice(QIODevice * device);
impl<'a> /*trait*/ QXmlStreamWriter_setDevice for (&'a mut QIODevice) {
  fn setDevice(self, this: &mut QXmlStreamWriter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriter9setDeviceEP9QIODevice()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN16QXmlStreamWriter9setDeviceEP9QIODevice(arg0)};
    return 1;
  }
}

// proto: void QXmlStreamWriter::writeStartDocument(const QString & version);
impl<'a> /*trait*/ QXmlStreamWriter_writeStartDocument for (&'a  QString) {
  fn writeStartDocument(self, this: &mut QXmlStreamWriter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriter18writeStartDocumentERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN16QXmlStreamWriter18writeStartDocumentERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QXmlStreamWriter {
  pub fn writeTextElement<T: QXmlStreamWriter_writeTextElement>(&mut self, value: T) -> i32 {
    value.writeTextElement(self);
    return 1;
  }
}

pub trait QXmlStreamWriter_writeTextElement {
  fn writeTextElement(self, this: &mut QXmlStreamWriter) -> i32;
}

// proto: void QXmlStreamWriter::writeTextElement(const QString & namespaceUri, const QString & name, const QString & text);
impl<'a> /*trait*/ QXmlStreamWriter_writeTextElement for (&'a  QString, &'a  QString, &'a  QString) {
  fn writeTextElement(self, this: &mut QXmlStreamWriter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriter16writeTextElementERK7QStringS2_S2_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2.qclsinst  as *const c_void;
    unsafe {_ZN16QXmlStreamWriter16writeTextElementERK7QStringS2_S2_(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QXmlStreamWriter {
  pub fn writeAttribute<T: QXmlStreamWriter_writeAttribute>(&mut self, value: T) -> i32 {
    value.writeAttribute(self);
    return 1;
  }
}

pub trait QXmlStreamWriter_writeAttribute {
  fn writeAttribute(self, this: &mut QXmlStreamWriter) -> i32;
}

// proto: void QXmlStreamWriter::writeAttribute(const QString & qualifiedName, const QString & value);
impl<'a> /*trait*/ QXmlStreamWriter_writeAttribute for (&'a  QString, &'a  QString) {
  fn writeAttribute(self, this: &mut QXmlStreamWriter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriter14writeAttributeERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN16QXmlStreamWriter14writeAttributeERK7QStringS2_(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QXmlStreamWriter {
  pub fn writeEmptyElement<T: QXmlStreamWriter_writeEmptyElement>(&mut self, value: T) -> i32 {
    value.writeEmptyElement(self);
    return 1;
  }
}

pub trait QXmlStreamWriter_writeEmptyElement {
  fn writeEmptyElement(self, this: &mut QXmlStreamWriter) -> i32;
}

// proto: void QXmlStreamWriter::writeEmptyElement(const QString & qualifiedName);
impl<'a> /*trait*/ QXmlStreamWriter_writeEmptyElement for (&'a  QString) {
  fn writeEmptyElement(self, this: &mut QXmlStreamWriter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriter17writeEmptyElementERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN16QXmlStreamWriter17writeEmptyElementERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QXmlStreamWriter {
  pub fn writeDTD<T: QXmlStreamWriter_writeDTD>(&mut self, value: T) -> i32 {
    value.writeDTD(self);
    return 1;
  }
}

pub trait QXmlStreamWriter_writeDTD {
  fn writeDTD(self, this: &mut QXmlStreamWriter) -> i32;
}

// proto: void QXmlStreamWriter::writeDTD(const QString & dtd);
impl<'a> /*trait*/ QXmlStreamWriter_writeDTD for (&'a  QString) {
  fn writeDTD(self, this: &mut QXmlStreamWriter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriter8writeDTDERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN16QXmlStreamWriter8writeDTDERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QXmlStreamWriter {
  pub fn setAutoFormattingIndent<T: QXmlStreamWriter_setAutoFormattingIndent>(&mut self, value: T) -> i32 {
    value.setAutoFormattingIndent(self);
    return 1;
  }
}

pub trait QXmlStreamWriter_setAutoFormattingIndent {
  fn setAutoFormattingIndent(self, this: &mut QXmlStreamWriter) -> i32;
}

// proto: void QXmlStreamWriter::setAutoFormattingIndent(int spacesOrTabs);
impl<'a> /*trait*/ QXmlStreamWriter_setAutoFormattingIndent for (i32) {
  fn setAutoFormattingIndent(self, this: &mut QXmlStreamWriter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriter23setAutoFormattingIndentEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN16QXmlStreamWriter23setAutoFormattingIndentEi(arg0)};
    return 1;
  }
}

// proto: void QXmlStreamWriter::writeAttribute(const QXmlStreamAttribute & attribute);
impl<'a> /*trait*/ QXmlStreamWriter_writeAttribute for (&'a  QXmlStreamAttribute) {
  fn writeAttribute(self, this: &mut QXmlStreamWriter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriter14writeAttributeERK19QXmlStreamAttribute()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN16QXmlStreamWriter14writeAttributeERK19QXmlStreamAttribute(arg0)};
    return 1;
  }
}

impl /*struct*/ QXmlStreamWriter {
  pub fn writeStartElement<T: QXmlStreamWriter_writeStartElement>(&mut self, value: T) -> i32 {
    value.writeStartElement(self);
    return 1;
  }
}

pub trait QXmlStreamWriter_writeStartElement {
  fn writeStartElement(self, this: &mut QXmlStreamWriter) -> i32;
}

// proto: void QXmlStreamWriter::writeStartElement(const QString & namespaceUri, const QString & name);
impl<'a> /*trait*/ QXmlStreamWriter_writeStartElement for (&'a  QString, &'a  QString) {
  fn writeStartElement(self, this: &mut QXmlStreamWriter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriter17writeStartElementERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN16QXmlStreamWriter17writeStartElementERK7QStringS2_(arg0, arg1)};
    return 1;
  }
}

// proto: void QXmlStreamWriter::NewQXmlStreamWriter(QString * string);
impl<'a> /*trait*/ QXmlStreamWriter_NewQXmlStreamWriter for (&'a mut QString) {
  fn NewQXmlStreamWriter(self) -> QXmlStreamWriter {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriterC1EP7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN16QXmlStreamWriterC1EP7QString(qthis, arg0)};
    let rsthis = QXmlStreamWriter{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QXmlStreamWriter {
  pub fn writeComment<T: QXmlStreamWriter_writeComment>(&mut self, value: T) -> i32 {
    value.writeComment(self);
    return 1;
  }
}

pub trait QXmlStreamWriter_writeComment {
  fn writeComment(self, this: &mut QXmlStreamWriter) -> i32;
}

// proto: void QXmlStreamWriter::writeComment(const QString & text);
impl<'a> /*trait*/ QXmlStreamWriter_writeComment for (&'a  QString) {
  fn writeComment(self, this: &mut QXmlStreamWriter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriter12writeCommentERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN16QXmlStreamWriter12writeCommentERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QXmlStreamWriter {
  pub fn codec<T: QXmlStreamWriter_codec>(&mut self, value: T) -> i32 {
    value.codec(self);
    return 1;
  }
}

pub trait QXmlStreamWriter_codec {
  fn codec(self, this: &mut QXmlStreamWriter) -> i32;
}

// proto: QTextCodec * QXmlStreamWriter::codec();
impl<'a> /*trait*/ QXmlStreamWriter_codec for () {
  fn codec(self, this: &mut QXmlStreamWriter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamWriter5codecEv()};
    unsafe {_ZNK16QXmlStreamWriter5codecEv()};
    return 1;
  }
}

// proto: void QXmlStreamWriter::writeAttribute(const QString & namespaceUri, const QString & name, const QString & value);
impl<'a> /*trait*/ QXmlStreamWriter_writeAttribute for (&'a  QString, &'a  QString, &'a  QString) {
  fn writeAttribute(self, this: &mut QXmlStreamWriter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriter14writeAttributeERK7QStringS2_S2_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2.qclsinst  as *const c_void;
    unsafe {_ZN16QXmlStreamWriter14writeAttributeERK7QStringS2_S2_(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QXmlStreamWriter {
  pub fn writeNamespace<T: QXmlStreamWriter_writeNamespace>(&mut self, value: T) -> i32 {
    value.writeNamespace(self);
    return 1;
  }
}

pub trait QXmlStreamWriter_writeNamespace {
  fn writeNamespace(self, this: &mut QXmlStreamWriter) -> i32;
}

// proto: void QXmlStreamWriter::writeNamespace(const QString & namespaceUri, const QString & prefix);
impl<'a> /*trait*/ QXmlStreamWriter_writeNamespace for (&'a  QString, &'a  QString) {
  fn writeNamespace(self, this: &mut QXmlStreamWriter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriter14writeNamespaceERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN16QXmlStreamWriter14writeNamespaceERK7QStringS2_(arg0, arg1)};
    return 1;
  }
}

// proto: void QXmlStreamWriter::NewQXmlStreamWriter(const QXmlStreamWriter & );
impl<'a> /*trait*/ QXmlStreamWriter_NewQXmlStreamWriter for (&'a  QXmlStreamWriter) {
  fn NewQXmlStreamWriter(self) -> QXmlStreamWriter {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriterC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN16QXmlStreamWriterC1ERKS_(qthis, arg0)};
    let rsthis = QXmlStreamWriter{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QXmlStreamWriter {
  pub fn hasError<T: QXmlStreamWriter_hasError>(&mut self, value: T) -> i32 {
    value.hasError(self);
    return 1;
  }
}

pub trait QXmlStreamWriter_hasError {
  fn hasError(self, this: &mut QXmlStreamWriter) -> i32;
}

// proto: bool QXmlStreamWriter::hasError();
impl<'a> /*trait*/ QXmlStreamWriter_hasError for () {
  fn hasError(self, this: &mut QXmlStreamWriter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamWriter8hasErrorEv()};
    unsafe {_ZNK16QXmlStreamWriter8hasErrorEv()};
    return 1;
  }
}

impl /*struct*/ QXmlStreamWriter {
  pub fn writeCDATA<T: QXmlStreamWriter_writeCDATA>(&mut self, value: T) -> i32 {
    value.writeCDATA(self);
    return 1;
  }
}

pub trait QXmlStreamWriter_writeCDATA {
  fn writeCDATA(self, this: &mut QXmlStreamWriter) -> i32;
}

// proto: void QXmlStreamWriter::writeCDATA(const QString & text);
impl<'a> /*trait*/ QXmlStreamWriter_writeCDATA for (&'a  QString) {
  fn writeCDATA(self, this: &mut QXmlStreamWriter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriter10writeCDATAERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN16QXmlStreamWriter10writeCDATAERK7QString(arg0)};
    return 1;
  }
}

// proto: void QXmlStreamWriter::writeStartDocument();
impl<'a> /*trait*/ QXmlStreamWriter_writeStartDocument for () {
  fn writeStartDocument(self, this: &mut QXmlStreamWriter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriter18writeStartDocumentEv()};
    unsafe {_ZN16QXmlStreamWriter18writeStartDocumentEv()};
    return 1;
  }
}

impl /*struct*/ QXmlStreamWriter {
  pub fn writeEntityReference<T: QXmlStreamWriter_writeEntityReference>(&mut self, value: T) -> i32 {
    value.writeEntityReference(self);
    return 1;
  }
}

pub trait QXmlStreamWriter_writeEntityReference {
  fn writeEntityReference(self, this: &mut QXmlStreamWriter) -> i32;
}

// proto: void QXmlStreamWriter::writeEntityReference(const QString & name);
impl<'a> /*trait*/ QXmlStreamWriter_writeEntityReference for (&'a  QString) {
  fn writeEntityReference(self, this: &mut QXmlStreamWriter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriter20writeEntityReferenceERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN16QXmlStreamWriter20writeEntityReferenceERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QXmlStreamWriter {
  pub fn setAutoFormatting<T: QXmlStreamWriter_setAutoFormatting>(&mut self, value: T) -> i32 {
    value.setAutoFormatting(self);
    return 1;
  }
}

pub trait QXmlStreamWriter_setAutoFormatting {
  fn setAutoFormatting(self, this: &mut QXmlStreamWriter) -> i32;
}

// proto: void QXmlStreamWriter::setAutoFormatting(bool );
impl<'a> /*trait*/ QXmlStreamWriter_setAutoFormatting for (i8) {
  fn setAutoFormatting(self, this: &mut QXmlStreamWriter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriter17setAutoFormattingEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN16QXmlStreamWriter17setAutoFormattingEb(arg0)};
    return 1;
  }
}

// proto: void QXmlStreamWriter::setCodec(const char * codecName);
impl<'a> /*trait*/ QXmlStreamWriter_setCodec for (&'a  String) {
  fn setCodec(self, this: &mut QXmlStreamWriter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriter8setCodecEPKc()};
    let arg0 = self.as_ptr()  as *const c_char;
    unsafe {_ZN16QXmlStreamWriter8setCodecEPKc(arg0)};
    return 1;
  }
}

impl /*struct*/ QXmlStreamWriter {
  pub fn autoFormattingIndent<T: QXmlStreamWriter_autoFormattingIndent>(&mut self, value: T) -> i32 {
    value.autoFormattingIndent(self);
    return 1;
  }
}

pub trait QXmlStreamWriter_autoFormattingIndent {
  fn autoFormattingIndent(self, this: &mut QXmlStreamWriter) -> i32;
}

// proto: int QXmlStreamWriter::autoFormattingIndent();
impl<'a> /*trait*/ QXmlStreamWriter_autoFormattingIndent for () {
  fn autoFormattingIndent(self, this: &mut QXmlStreamWriter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamWriter20autoFormattingIndentEv()};
    unsafe {_ZNK16QXmlStreamWriter20autoFormattingIndentEv()};
    return 1;
  }
}

impl /*struct*/ QXmlStreamWriter {
  pub fn FreeQXmlStreamWriter<T: QXmlStreamWriter_FreeQXmlStreamWriter>(&mut self, value: T) -> i32 {
    value.FreeQXmlStreamWriter(self);
    return 1;
  }
}

pub trait QXmlStreamWriter_FreeQXmlStreamWriter {
  fn FreeQXmlStreamWriter(self, this: &mut QXmlStreamWriter) -> i32;
}

// proto: void QXmlStreamWriter::FreeQXmlStreamWriter();
impl<'a> /*trait*/ QXmlStreamWriter_FreeQXmlStreamWriter for () {
  fn FreeQXmlStreamWriter(self, this: &mut QXmlStreamWriter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriterD0Ev()};
    unsafe {_ZN16QXmlStreamWriterD0Ev()};
    return 1;
  }
}

impl /*struct*/ QXmlStreamWriter {
  pub fn writeAttributes<T: QXmlStreamWriter_writeAttributes>(&mut self, value: T) -> i32 {
    value.writeAttributes(self);
    return 1;
  }
}

pub trait QXmlStreamWriter_writeAttributes {
  fn writeAttributes(self, this: &mut QXmlStreamWriter) -> i32;
}

// proto: void QXmlStreamWriter::writeAttributes(const QXmlStreamAttributes & attributes);
impl<'a> /*trait*/ QXmlStreamWriter_writeAttributes for (&'a  QXmlStreamAttributes) {
  fn writeAttributes(self, this: &mut QXmlStreamWriter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriter15writeAttributesERK20QXmlStreamAttributes()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN16QXmlStreamWriter15writeAttributesERK20QXmlStreamAttributes(arg0)};
    return 1;
  }
}

impl /*struct*/ QXmlStreamWriter {
  pub fn writeDefaultNamespace<T: QXmlStreamWriter_writeDefaultNamespace>(&mut self, value: T) -> i32 {
    value.writeDefaultNamespace(self);
    return 1;
  }
}

pub trait QXmlStreamWriter_writeDefaultNamespace {
  fn writeDefaultNamespace(self, this: &mut QXmlStreamWriter) -> i32;
}

// proto: void QXmlStreamWriter::writeDefaultNamespace(const QString & namespaceUri);
impl<'a> /*trait*/ QXmlStreamWriter_writeDefaultNamespace for (&'a  QString) {
  fn writeDefaultNamespace(self, this: &mut QXmlStreamWriter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriter21writeDefaultNamespaceERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN16QXmlStreamWriter21writeDefaultNamespaceERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QXmlStreamWriter {
  pub fn device<T: QXmlStreamWriter_device>(&mut self, value: T) -> i32 {
    value.device(self);
    return 1;
  }
}

pub trait QXmlStreamWriter_device {
  fn device(self, this: &mut QXmlStreamWriter) -> i32;
}

// proto: QIODevice * QXmlStreamWriter::device();
impl<'a> /*trait*/ QXmlStreamWriter_device for () {
  fn device(self, this: &mut QXmlStreamWriter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamWriter6deviceEv()};
    unsafe {_ZNK16QXmlStreamWriter6deviceEv()};
    return 1;
  }
}

impl /*struct*/ QXmlStreamWriter {
  pub fn writeCurrentToken<T: QXmlStreamWriter_writeCurrentToken>(&mut self, value: T) -> i32 {
    value.writeCurrentToken(self);
    return 1;
  }
}

pub trait QXmlStreamWriter_writeCurrentToken {
  fn writeCurrentToken(self, this: &mut QXmlStreamWriter) -> i32;
}

// proto: void QXmlStreamWriter::writeCurrentToken(const QXmlStreamReader & reader);
impl<'a> /*trait*/ QXmlStreamWriter_writeCurrentToken for (&'a  QXmlStreamReader) {
  fn writeCurrentToken(self, this: &mut QXmlStreamWriter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriter17writeCurrentTokenERK16QXmlStreamReader()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN16QXmlStreamWriter17writeCurrentTokenERK16QXmlStreamReader(arg0)};
    return 1;
  }
}

// proto: void QXmlStreamWriter::NewQXmlStreamWriter(QByteArray * array);
impl<'a> /*trait*/ QXmlStreamWriter_NewQXmlStreamWriter for (&'a mut QByteArray) {
  fn NewQXmlStreamWriter(self) -> QXmlStreamWriter {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriterC1EP10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN16QXmlStreamWriterC1EP10QByteArray(qthis, arg0)};
    let rsthis = QXmlStreamWriter{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QXmlStreamWriter::writeTextElement(const QString & qualifiedName, const QString & text);
impl<'a> /*trait*/ QXmlStreamWriter_writeTextElement for (&'a  QString, &'a  QString) {
  fn writeTextElement(self, this: &mut QXmlStreamWriter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriter16writeTextElementERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN16QXmlStreamWriter16writeTextElementERK7QStringS2_(arg0, arg1)};
    return 1;
  }
}

// proto: void QXmlStreamWriter::writeEmptyElement(const QString & namespaceUri, const QString & name);
impl<'a> /*trait*/ QXmlStreamWriter_writeEmptyElement for (&'a  QString, &'a  QString) {
  fn writeEmptyElement(self, this: &mut QXmlStreamWriter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriter17writeEmptyElementERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN16QXmlStreamWriter17writeEmptyElementERK7QStringS2_(arg0, arg1)};
    return 1;
  }
}

// proto: void QXmlStreamWriter::NewQXmlStreamWriter(QIODevice * device);
impl<'a> /*trait*/ QXmlStreamWriter_NewQXmlStreamWriter for (&'a mut QIODevice) {
  fn NewQXmlStreamWriter(self) -> QXmlStreamWriter {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriterC1EP9QIODevice()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN16QXmlStreamWriterC1EP9QIODevice(qthis, arg0)};
    let rsthis = QXmlStreamWriter{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QXmlStreamWriter::writeStartElement(const QString & qualifiedName);
impl<'a> /*trait*/ QXmlStreamWriter_writeStartElement for (&'a  QString) {
  fn writeStartElement(self, this: &mut QXmlStreamWriter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriter17writeStartElementERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN16QXmlStreamWriter17writeStartElementERK7QString(arg0)};
    return 1;
  }
}

