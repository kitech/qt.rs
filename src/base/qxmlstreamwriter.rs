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
  // proto:  void QXmlStreamWriter::writeEndElement();
  fn _ZN16QXmlStreamWriter15writeEndElementEv(qthis: *mut c_void) ;
  // proto:  void QXmlStreamWriter::NewQXmlStreamWriter();
  fn _ZN16QXmlStreamWriterC1Ev(qthis: *mut c_void) ;
  // proto:  void QXmlStreamWriter::writeEndDocument();
  fn _ZN16QXmlStreamWriter16writeEndDocumentEv(qthis: *mut c_void) ;
  // proto:  bool QXmlStreamWriter::autoFormatting();
  fn _ZNK16QXmlStreamWriter14autoFormattingEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QXmlStreamWriter::writeStartDocument(const QString & version, bool standalone);
  fn _ZN16QXmlStreamWriter18writeStartDocumentERK7QStringb(qthis: *mut c_void, arg0: *mut c_void, arg1: int8_t) ;
  // proto:  void QXmlStreamWriter::setCodec(QTextCodec * codec);
  fn _ZN16QXmlStreamWriter8setCodecEP10QTextCodec(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QXmlStreamWriter::writeProcessingInstruction(const QString & target, const QString & data);
  fn _ZN16QXmlStreamWriter26writeProcessingInstructionERK7QStringS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  void QXmlStreamWriter::writeCharacters(const QString & text);
  fn _ZN16QXmlStreamWriter15writeCharactersERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QXmlStreamWriter::setDevice(QIODevice * device);
  fn _ZN16QXmlStreamWriter9setDeviceEP9QIODevice(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QXmlStreamWriter::writeStartDocument(const QString & version);
  fn _ZN16QXmlStreamWriter18writeStartDocumentERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QXmlStreamWriter::writeTextElement(const QString & namespaceUri, const QString & name, const QString & text);
  fn _ZN16QXmlStreamWriter16writeTextElementERK7QStringS2_S2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) ;
  // proto:  void QXmlStreamWriter::writeAttribute(const QString & qualifiedName, const QString & value);
  fn _ZN16QXmlStreamWriter14writeAttributeERK7QStringS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  void QXmlStreamWriter::writeEmptyElement(const QString & qualifiedName);
  fn _ZN16QXmlStreamWriter17writeEmptyElementERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QXmlStreamWriter::writeDTD(const QString & dtd);
  fn _ZN16QXmlStreamWriter8writeDTDERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QXmlStreamWriter::setAutoFormattingIndent(int spacesOrTabs);
  fn _ZN16QXmlStreamWriter23setAutoFormattingIndentEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QXmlStreamWriter::writeAttribute(const QXmlStreamAttribute & attribute);
  fn _ZN16QXmlStreamWriter14writeAttributeERK19QXmlStreamAttribute(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QXmlStreamWriter::writeStartElement(const QString & namespaceUri, const QString & name);
  fn _ZN16QXmlStreamWriter17writeStartElementERK7QStringS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  void QXmlStreamWriter::NewQXmlStreamWriter(QString * string);
  fn _ZN16QXmlStreamWriterC1EP7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QXmlStreamWriter::writeComment(const QString & text);
  fn _ZN16QXmlStreamWriter12writeCommentERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QTextCodec * QXmlStreamWriter::codec();
  fn _ZNK16QXmlStreamWriter5codecEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QXmlStreamWriter::writeAttribute(const QString & namespaceUri, const QString & name, const QString & value);
  fn _ZN16QXmlStreamWriter14writeAttributeERK7QStringS2_S2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) ;
  // proto:  void QXmlStreamWriter::writeNamespace(const QString & namespaceUri, const QString & prefix);
  fn _ZN16QXmlStreamWriter14writeNamespaceERK7QStringS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  void QXmlStreamWriter::NewQXmlStreamWriter(const QXmlStreamWriter & );
  fn _ZN16QXmlStreamWriterC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QXmlStreamWriter::hasError();
  fn _ZNK16QXmlStreamWriter8hasErrorEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QXmlStreamWriter::writeCDATA(const QString & text);
  fn _ZN16QXmlStreamWriter10writeCDATAERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QXmlStreamWriter::writeStartDocument();
  fn _ZN16QXmlStreamWriter18writeStartDocumentEv(qthis: *mut c_void) ;
  // proto:  void QXmlStreamWriter::writeEntityReference(const QString & name);
  fn _ZN16QXmlStreamWriter20writeEntityReferenceERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QXmlStreamWriter::setAutoFormatting(bool );
  fn _ZN16QXmlStreamWriter17setAutoFormattingEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QXmlStreamWriter::setCodec(const char * codecName);
  fn _ZN16QXmlStreamWriter8setCodecEPKc(qthis: *mut c_void, arg0: *const c_char) ;
  // proto:  int QXmlStreamWriter::autoFormattingIndent();
  fn _ZNK16QXmlStreamWriter20autoFormattingIndentEv(qthis: *mut c_void) -> c_int;
  // proto:  void QXmlStreamWriter::FreeQXmlStreamWriter();
  fn _ZN16QXmlStreamWriterD0Ev(qthis: *mut c_void) ;
  // proto:  void QXmlStreamWriter::writeAttributes(const QXmlStreamAttributes & attributes);
  fn _ZN16QXmlStreamWriter15writeAttributesERK20QXmlStreamAttributes(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QXmlStreamWriter::writeDefaultNamespace(const QString & namespaceUri);
  fn _ZN16QXmlStreamWriter21writeDefaultNamespaceERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QIODevice * QXmlStreamWriter::device();
  fn _ZNK16QXmlStreamWriter6deviceEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QXmlStreamWriter::writeCurrentToken(const QXmlStreamReader & reader);
  fn _ZN16QXmlStreamWriter17writeCurrentTokenERK16QXmlStreamReader(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QXmlStreamWriter::NewQXmlStreamWriter(QByteArray * array);
  fn _ZN16QXmlStreamWriterC1EP10QByteArray(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QXmlStreamWriter::writeTextElement(const QString & qualifiedName, const QString & text);
  fn _ZN16QXmlStreamWriter16writeTextElementERK7QStringS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  void QXmlStreamWriter::writeEmptyElement(const QString & namespaceUri, const QString & name);
  fn _ZN16QXmlStreamWriter17writeEmptyElementERK7QStringS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  void QXmlStreamWriter::NewQXmlStreamWriter(QIODevice * device);
  fn _ZN16QXmlStreamWriterC1EP9QIODevice(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QXmlStreamWriter::writeStartElement(const QString & qualifiedName);
  fn _ZN16QXmlStreamWriter17writeStartElementERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
}

// body block begin
// class sizeof(QXmlStreamWriter)=1
pub struct QXmlStreamWriter {
  pub qclsinst: *mut c_void,
}

// proto:  void QXmlStreamWriter::writeEndElement();
impl /*struct*/ QXmlStreamWriter {
  pub fn writeEndElement<RetType, T: QXmlStreamWriter_writeEndElement<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.writeEndElement(self);
    // return 1;
  }
}

pub trait QXmlStreamWriter_writeEndElement<RetType> {
  fn writeEndElement(self , rsthis: &mut QXmlStreamWriter) -> RetType;
}

// proto:  void QXmlStreamWriter::writeEndElement();
impl<'a> /*trait*/ QXmlStreamWriter_writeEndElement<()> for () {
  fn writeEndElement(self , rsthis: &mut QXmlStreamWriter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriter15writeEndElementEv()};
     unsafe {_ZN16QXmlStreamWriter15writeEndElementEv(rsthis.qclsinst)};
    // return 1;
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

// proto:  void QXmlStreamWriter::writeEndDocument();
impl /*struct*/ QXmlStreamWriter {
  pub fn writeEndDocument<RetType, T: QXmlStreamWriter_writeEndDocument<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.writeEndDocument(self);
    // return 1;
  }
}

pub trait QXmlStreamWriter_writeEndDocument<RetType> {
  fn writeEndDocument(self , rsthis: &mut QXmlStreamWriter) -> RetType;
}

// proto:  void QXmlStreamWriter::writeEndDocument();
impl<'a> /*trait*/ QXmlStreamWriter_writeEndDocument<()> for () {
  fn writeEndDocument(self , rsthis: &mut QXmlStreamWriter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriter16writeEndDocumentEv()};
     unsafe {_ZN16QXmlStreamWriter16writeEndDocumentEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  bool QXmlStreamWriter::autoFormatting();
impl /*struct*/ QXmlStreamWriter {
  pub fn autoFormatting<RetType, T: QXmlStreamWriter_autoFormatting<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.autoFormatting(self);
    // return 1;
  }
}

pub trait QXmlStreamWriter_autoFormatting<RetType> {
  fn autoFormatting(self , rsthis: &mut QXmlStreamWriter) -> RetType;
}

// proto:  bool QXmlStreamWriter::autoFormatting();
impl<'a> /*trait*/ QXmlStreamWriter_autoFormatting<i8> for () {
  fn autoFormatting(self , rsthis: &mut QXmlStreamWriter) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamWriter14autoFormattingEv()};
    let mut ret = unsafe {_ZNK16QXmlStreamWriter14autoFormattingEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto:  void QXmlStreamWriter::writeStartDocument(const QString & version, bool standalone);
impl /*struct*/ QXmlStreamWriter {
  pub fn writeStartDocument<RetType, T: QXmlStreamWriter_writeStartDocument<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.writeStartDocument(self);
    // return 1;
  }
}

pub trait QXmlStreamWriter_writeStartDocument<RetType> {
  fn writeStartDocument(self , rsthis: &mut QXmlStreamWriter) -> RetType;
}

// proto:  void QXmlStreamWriter::writeStartDocument(const QString & version, bool standalone);
impl<'a> /*trait*/ QXmlStreamWriter_writeStartDocument<()> for (&'a  QString, i8) {
  fn writeStartDocument(self , rsthis: &mut QXmlStreamWriter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriter18writeStartDocumentERK7QStringb()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as int8_t;
     unsafe {_ZN16QXmlStreamWriter18writeStartDocumentERK7QStringb(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  void QXmlStreamWriter::setCodec(QTextCodec * codec);
impl /*struct*/ QXmlStreamWriter {
  pub fn setCodec<RetType, T: QXmlStreamWriter_setCodec<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setCodec(self);
    // return 1;
  }
}

pub trait QXmlStreamWriter_setCodec<RetType> {
  fn setCodec(self , rsthis: &mut QXmlStreamWriter) -> RetType;
}

// proto:  void QXmlStreamWriter::setCodec(QTextCodec * codec);
impl<'a> /*trait*/ QXmlStreamWriter_setCodec<()> for (&'a mut QTextCodec) {
  fn setCodec(self , rsthis: &mut QXmlStreamWriter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriter8setCodecEP10QTextCodec()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QXmlStreamWriter8setCodecEP10QTextCodec(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QXmlStreamWriter::writeProcessingInstruction(const QString & target, const QString & data);
impl /*struct*/ QXmlStreamWriter {
  pub fn writeProcessingInstruction<RetType, T: QXmlStreamWriter_writeProcessingInstruction<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.writeProcessingInstruction(self);
    // return 1;
  }
}

pub trait QXmlStreamWriter_writeProcessingInstruction<RetType> {
  fn writeProcessingInstruction(self , rsthis: &mut QXmlStreamWriter) -> RetType;
}

// proto:  void QXmlStreamWriter::writeProcessingInstruction(const QString & target, const QString & data);
impl<'a> /*trait*/ QXmlStreamWriter_writeProcessingInstruction<()> for (&'a  QString, &'a  QString) {
  fn writeProcessingInstruction(self , rsthis: &mut QXmlStreamWriter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriter26writeProcessingInstructionERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN16QXmlStreamWriter26writeProcessingInstructionERK7QStringS2_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  void QXmlStreamWriter::writeCharacters(const QString & text);
impl /*struct*/ QXmlStreamWriter {
  pub fn writeCharacters<RetType, T: QXmlStreamWriter_writeCharacters<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.writeCharacters(self);
    // return 1;
  }
}

pub trait QXmlStreamWriter_writeCharacters<RetType> {
  fn writeCharacters(self , rsthis: &mut QXmlStreamWriter) -> RetType;
}

// proto:  void QXmlStreamWriter::writeCharacters(const QString & text);
impl<'a> /*trait*/ QXmlStreamWriter_writeCharacters<()> for (&'a  QString) {
  fn writeCharacters(self , rsthis: &mut QXmlStreamWriter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriter15writeCharactersERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QXmlStreamWriter15writeCharactersERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QXmlStreamWriter::setDevice(QIODevice * device);
impl /*struct*/ QXmlStreamWriter {
  pub fn setDevice<RetType, T: QXmlStreamWriter_setDevice<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setDevice(self);
    // return 1;
  }
}

pub trait QXmlStreamWriter_setDevice<RetType> {
  fn setDevice(self , rsthis: &mut QXmlStreamWriter) -> RetType;
}

// proto:  void QXmlStreamWriter::setDevice(QIODevice * device);
impl<'a> /*trait*/ QXmlStreamWriter_setDevice<()> for (&'a mut QIODevice) {
  fn setDevice(self , rsthis: &mut QXmlStreamWriter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriter9setDeviceEP9QIODevice()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QXmlStreamWriter9setDeviceEP9QIODevice(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QXmlStreamWriter::writeStartDocument(const QString & version);
impl<'a> /*trait*/ QXmlStreamWriter_writeStartDocument<()> for (&'a  QString) {
  fn writeStartDocument(self , rsthis: &mut QXmlStreamWriter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriter18writeStartDocumentERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QXmlStreamWriter18writeStartDocumentERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QXmlStreamWriter::writeTextElement(const QString & namespaceUri, const QString & name, const QString & text);
impl /*struct*/ QXmlStreamWriter {
  pub fn writeTextElement<RetType, T: QXmlStreamWriter_writeTextElement<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.writeTextElement(self);
    // return 1;
  }
}

pub trait QXmlStreamWriter_writeTextElement<RetType> {
  fn writeTextElement(self , rsthis: &mut QXmlStreamWriter) -> RetType;
}

// proto:  void QXmlStreamWriter::writeTextElement(const QString & namespaceUri, const QString & name, const QString & text);
impl<'a> /*trait*/ QXmlStreamWriter_writeTextElement<()> for (&'a  QString, &'a  QString, &'a  QString) {
  fn writeTextElement(self , rsthis: &mut QXmlStreamWriter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriter16writeTextElementERK7QStringS2_S2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN16QXmlStreamWriter16writeTextElementERK7QStringS2_S2_(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

// proto:  void QXmlStreamWriter::writeAttribute(const QString & qualifiedName, const QString & value);
impl /*struct*/ QXmlStreamWriter {
  pub fn writeAttribute<RetType, T: QXmlStreamWriter_writeAttribute<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.writeAttribute(self);
    // return 1;
  }
}

pub trait QXmlStreamWriter_writeAttribute<RetType> {
  fn writeAttribute(self , rsthis: &mut QXmlStreamWriter) -> RetType;
}

// proto:  void QXmlStreamWriter::writeAttribute(const QString & qualifiedName, const QString & value);
impl<'a> /*trait*/ QXmlStreamWriter_writeAttribute<()> for (&'a  QString, &'a  QString) {
  fn writeAttribute(self , rsthis: &mut QXmlStreamWriter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriter14writeAttributeERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN16QXmlStreamWriter14writeAttributeERK7QStringS2_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  void QXmlStreamWriter::writeEmptyElement(const QString & qualifiedName);
impl /*struct*/ QXmlStreamWriter {
  pub fn writeEmptyElement<RetType, T: QXmlStreamWriter_writeEmptyElement<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.writeEmptyElement(self);
    // return 1;
  }
}

pub trait QXmlStreamWriter_writeEmptyElement<RetType> {
  fn writeEmptyElement(self , rsthis: &mut QXmlStreamWriter) -> RetType;
}

// proto:  void QXmlStreamWriter::writeEmptyElement(const QString & qualifiedName);
impl<'a> /*trait*/ QXmlStreamWriter_writeEmptyElement<()> for (&'a  QString) {
  fn writeEmptyElement(self , rsthis: &mut QXmlStreamWriter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriter17writeEmptyElementERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QXmlStreamWriter17writeEmptyElementERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QXmlStreamWriter::writeDTD(const QString & dtd);
impl /*struct*/ QXmlStreamWriter {
  pub fn writeDTD<RetType, T: QXmlStreamWriter_writeDTD<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.writeDTD(self);
    // return 1;
  }
}

pub trait QXmlStreamWriter_writeDTD<RetType> {
  fn writeDTD(self , rsthis: &mut QXmlStreamWriter) -> RetType;
}

// proto:  void QXmlStreamWriter::writeDTD(const QString & dtd);
impl<'a> /*trait*/ QXmlStreamWriter_writeDTD<()> for (&'a  QString) {
  fn writeDTD(self , rsthis: &mut QXmlStreamWriter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriter8writeDTDERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QXmlStreamWriter8writeDTDERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QXmlStreamWriter::setAutoFormattingIndent(int spacesOrTabs);
impl /*struct*/ QXmlStreamWriter {
  pub fn setAutoFormattingIndent<RetType, T: QXmlStreamWriter_setAutoFormattingIndent<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setAutoFormattingIndent(self);
    // return 1;
  }
}

pub trait QXmlStreamWriter_setAutoFormattingIndent<RetType> {
  fn setAutoFormattingIndent(self , rsthis: &mut QXmlStreamWriter) -> RetType;
}

// proto:  void QXmlStreamWriter::setAutoFormattingIndent(int spacesOrTabs);
impl<'a> /*trait*/ QXmlStreamWriter_setAutoFormattingIndent<()> for (i32) {
  fn setAutoFormattingIndent(self , rsthis: &mut QXmlStreamWriter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriter23setAutoFormattingIndentEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN16QXmlStreamWriter23setAutoFormattingIndentEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QXmlStreamWriter::writeAttribute(const QXmlStreamAttribute & attribute);
impl<'a> /*trait*/ QXmlStreamWriter_writeAttribute<()> for (&'a  QXmlStreamAttribute) {
  fn writeAttribute(self , rsthis: &mut QXmlStreamWriter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriter14writeAttributeERK19QXmlStreamAttribute()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QXmlStreamWriter14writeAttributeERK19QXmlStreamAttribute(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QXmlStreamWriter::writeStartElement(const QString & namespaceUri, const QString & name);
impl /*struct*/ QXmlStreamWriter {
  pub fn writeStartElement<RetType, T: QXmlStreamWriter_writeStartElement<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.writeStartElement(self);
    // return 1;
  }
}

pub trait QXmlStreamWriter_writeStartElement<RetType> {
  fn writeStartElement(self , rsthis: &mut QXmlStreamWriter) -> RetType;
}

// proto:  void QXmlStreamWriter::writeStartElement(const QString & namespaceUri, const QString & name);
impl<'a> /*trait*/ QXmlStreamWriter_writeStartElement<()> for (&'a  QString, &'a  QString) {
  fn writeStartElement(self , rsthis: &mut QXmlStreamWriter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriter17writeStartElementERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN16QXmlStreamWriter17writeStartElementERK7QStringS2_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
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

// proto:  void QXmlStreamWriter::writeComment(const QString & text);
impl /*struct*/ QXmlStreamWriter {
  pub fn writeComment<RetType, T: QXmlStreamWriter_writeComment<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.writeComment(self);
    // return 1;
  }
}

pub trait QXmlStreamWriter_writeComment<RetType> {
  fn writeComment(self , rsthis: &mut QXmlStreamWriter) -> RetType;
}

// proto:  void QXmlStreamWriter::writeComment(const QString & text);
impl<'a> /*trait*/ QXmlStreamWriter_writeComment<()> for (&'a  QString) {
  fn writeComment(self , rsthis: &mut QXmlStreamWriter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriter12writeCommentERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QXmlStreamWriter12writeCommentERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  QTextCodec * QXmlStreamWriter::codec();
impl /*struct*/ QXmlStreamWriter {
  pub fn codec<RetType, T: QXmlStreamWriter_codec<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.codec(self);
    // return 1;
  }
}

pub trait QXmlStreamWriter_codec<RetType> {
  fn codec(self , rsthis: &mut QXmlStreamWriter) -> RetType;
}

// proto:  QTextCodec * QXmlStreamWriter::codec();
impl<'a> /*trait*/ QXmlStreamWriter_codec<QTextCodec> for () {
  fn codec(self , rsthis: &mut QXmlStreamWriter) -> QTextCodec {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamWriter5codecEv()};
    let mut ret = unsafe {_ZNK16QXmlStreamWriter5codecEv(rsthis.qclsinst)};
    let mut ret1 = QTextCodec{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QXmlStreamWriter::writeAttribute(const QString & namespaceUri, const QString & name, const QString & value);
impl<'a> /*trait*/ QXmlStreamWriter_writeAttribute<()> for (&'a  QString, &'a  QString, &'a  QString) {
  fn writeAttribute(self , rsthis: &mut QXmlStreamWriter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriter14writeAttributeERK7QStringS2_S2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN16QXmlStreamWriter14writeAttributeERK7QStringS2_S2_(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

// proto:  void QXmlStreamWriter::writeNamespace(const QString & namespaceUri, const QString & prefix);
impl /*struct*/ QXmlStreamWriter {
  pub fn writeNamespace<RetType, T: QXmlStreamWriter_writeNamespace<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.writeNamespace(self);
    // return 1;
  }
}

pub trait QXmlStreamWriter_writeNamespace<RetType> {
  fn writeNamespace(self , rsthis: &mut QXmlStreamWriter) -> RetType;
}

// proto:  void QXmlStreamWriter::writeNamespace(const QString & namespaceUri, const QString & prefix);
impl<'a> /*trait*/ QXmlStreamWriter_writeNamespace<()> for (&'a  QString, &'a  QString) {
  fn writeNamespace(self , rsthis: &mut QXmlStreamWriter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriter14writeNamespaceERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN16QXmlStreamWriter14writeNamespaceERK7QStringS2_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto: void QXmlStreamWriter::NewQXmlStreamWriter(const QXmlStreamWriter & );
impl<'a> /*trait*/ QXmlStreamWriter_NewQXmlStreamWriter for (&'a  QXmlStreamWriter) {
  fn NewQXmlStreamWriter(self) -> QXmlStreamWriter {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriterC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN16QXmlStreamWriterC1ERKS_(qthis, arg0)};
    let rsthis = QXmlStreamWriter{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto:  bool QXmlStreamWriter::hasError();
impl /*struct*/ QXmlStreamWriter {
  pub fn hasError<RetType, T: QXmlStreamWriter_hasError<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.hasError(self);
    // return 1;
  }
}

pub trait QXmlStreamWriter_hasError<RetType> {
  fn hasError(self , rsthis: &mut QXmlStreamWriter) -> RetType;
}

// proto:  bool QXmlStreamWriter::hasError();
impl<'a> /*trait*/ QXmlStreamWriter_hasError<i8> for () {
  fn hasError(self , rsthis: &mut QXmlStreamWriter) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamWriter8hasErrorEv()};
    let mut ret = unsafe {_ZNK16QXmlStreamWriter8hasErrorEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto:  void QXmlStreamWriter::writeCDATA(const QString & text);
impl /*struct*/ QXmlStreamWriter {
  pub fn writeCDATA<RetType, T: QXmlStreamWriter_writeCDATA<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.writeCDATA(self);
    // return 1;
  }
}

pub trait QXmlStreamWriter_writeCDATA<RetType> {
  fn writeCDATA(self , rsthis: &mut QXmlStreamWriter) -> RetType;
}

// proto:  void QXmlStreamWriter::writeCDATA(const QString & text);
impl<'a> /*trait*/ QXmlStreamWriter_writeCDATA<()> for (&'a  QString) {
  fn writeCDATA(self , rsthis: &mut QXmlStreamWriter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriter10writeCDATAERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QXmlStreamWriter10writeCDATAERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QXmlStreamWriter::writeStartDocument();
impl<'a> /*trait*/ QXmlStreamWriter_writeStartDocument<()> for () {
  fn writeStartDocument(self , rsthis: &mut QXmlStreamWriter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriter18writeStartDocumentEv()};
     unsafe {_ZN16QXmlStreamWriter18writeStartDocumentEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  void QXmlStreamWriter::writeEntityReference(const QString & name);
impl /*struct*/ QXmlStreamWriter {
  pub fn writeEntityReference<RetType, T: QXmlStreamWriter_writeEntityReference<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.writeEntityReference(self);
    // return 1;
  }
}

pub trait QXmlStreamWriter_writeEntityReference<RetType> {
  fn writeEntityReference(self , rsthis: &mut QXmlStreamWriter) -> RetType;
}

// proto:  void QXmlStreamWriter::writeEntityReference(const QString & name);
impl<'a> /*trait*/ QXmlStreamWriter_writeEntityReference<()> for (&'a  QString) {
  fn writeEntityReference(self , rsthis: &mut QXmlStreamWriter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriter20writeEntityReferenceERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QXmlStreamWriter20writeEntityReferenceERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QXmlStreamWriter::setAutoFormatting(bool );
impl /*struct*/ QXmlStreamWriter {
  pub fn setAutoFormatting<RetType, T: QXmlStreamWriter_setAutoFormatting<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setAutoFormatting(self);
    // return 1;
  }
}

pub trait QXmlStreamWriter_setAutoFormatting<RetType> {
  fn setAutoFormatting(self , rsthis: &mut QXmlStreamWriter) -> RetType;
}

// proto:  void QXmlStreamWriter::setAutoFormatting(bool );
impl<'a> /*trait*/ QXmlStreamWriter_setAutoFormatting<()> for (i8) {
  fn setAutoFormatting(self , rsthis: &mut QXmlStreamWriter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriter17setAutoFormattingEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN16QXmlStreamWriter17setAutoFormattingEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QXmlStreamWriter::setCodec(const char * codecName);
impl<'a> /*trait*/ QXmlStreamWriter_setCodec<()> for (&'a  String) {
  fn setCodec(self , rsthis: &mut QXmlStreamWriter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriter8setCodecEPKc()};
    let arg0 = self.as_ptr()  as *const c_char;
     unsafe {_ZN16QXmlStreamWriter8setCodecEPKc(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  int QXmlStreamWriter::autoFormattingIndent();
impl /*struct*/ QXmlStreamWriter {
  pub fn autoFormattingIndent<RetType, T: QXmlStreamWriter_autoFormattingIndent<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.autoFormattingIndent(self);
    // return 1;
  }
}

pub trait QXmlStreamWriter_autoFormattingIndent<RetType> {
  fn autoFormattingIndent(self , rsthis: &mut QXmlStreamWriter) -> RetType;
}

// proto:  int QXmlStreamWriter::autoFormattingIndent();
impl<'a> /*trait*/ QXmlStreamWriter_autoFormattingIndent<i32> for () {
  fn autoFormattingIndent(self , rsthis: &mut QXmlStreamWriter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamWriter20autoFormattingIndentEv()};
    let mut ret = unsafe {_ZNK16QXmlStreamWriter20autoFormattingIndentEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto:  void QXmlStreamWriter::FreeQXmlStreamWriter();
impl /*struct*/ QXmlStreamWriter {
  pub fn FreeQXmlStreamWriter<RetType, T: QXmlStreamWriter_FreeQXmlStreamWriter<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.FreeQXmlStreamWriter(self);
    // return 1;
  }
}

pub trait QXmlStreamWriter_FreeQXmlStreamWriter<RetType> {
  fn FreeQXmlStreamWriter(self , rsthis: &mut QXmlStreamWriter) -> RetType;
}

// proto:  void QXmlStreamWriter::FreeQXmlStreamWriter();
impl<'a> /*trait*/ QXmlStreamWriter_FreeQXmlStreamWriter<()> for () {
  fn FreeQXmlStreamWriter(self , rsthis: &mut QXmlStreamWriter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriterD0Ev()};
     unsafe {_ZN16QXmlStreamWriterD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  void QXmlStreamWriter::writeAttributes(const QXmlStreamAttributes & attributes);
impl /*struct*/ QXmlStreamWriter {
  pub fn writeAttributes<RetType, T: QXmlStreamWriter_writeAttributes<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.writeAttributes(self);
    // return 1;
  }
}

pub trait QXmlStreamWriter_writeAttributes<RetType> {
  fn writeAttributes(self , rsthis: &mut QXmlStreamWriter) -> RetType;
}

// proto:  void QXmlStreamWriter::writeAttributes(const QXmlStreamAttributes & attributes);
impl<'a> /*trait*/ QXmlStreamWriter_writeAttributes<()> for (&'a  QXmlStreamAttributes) {
  fn writeAttributes(self , rsthis: &mut QXmlStreamWriter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriter15writeAttributesERK20QXmlStreamAttributes()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QXmlStreamWriter15writeAttributesERK20QXmlStreamAttributes(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QXmlStreamWriter::writeDefaultNamespace(const QString & namespaceUri);
impl /*struct*/ QXmlStreamWriter {
  pub fn writeDefaultNamespace<RetType, T: QXmlStreamWriter_writeDefaultNamespace<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.writeDefaultNamespace(self);
    // return 1;
  }
}

pub trait QXmlStreamWriter_writeDefaultNamespace<RetType> {
  fn writeDefaultNamespace(self , rsthis: &mut QXmlStreamWriter) -> RetType;
}

// proto:  void QXmlStreamWriter::writeDefaultNamespace(const QString & namespaceUri);
impl<'a> /*trait*/ QXmlStreamWriter_writeDefaultNamespace<()> for (&'a  QString) {
  fn writeDefaultNamespace(self , rsthis: &mut QXmlStreamWriter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriter21writeDefaultNamespaceERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QXmlStreamWriter21writeDefaultNamespaceERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  QIODevice * QXmlStreamWriter::device();
impl /*struct*/ QXmlStreamWriter {
  pub fn device<RetType, T: QXmlStreamWriter_device<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.device(self);
    // return 1;
  }
}

pub trait QXmlStreamWriter_device<RetType> {
  fn device(self , rsthis: &mut QXmlStreamWriter) -> RetType;
}

// proto:  QIODevice * QXmlStreamWriter::device();
impl<'a> /*trait*/ QXmlStreamWriter_device<QIODevice> for () {
  fn device(self , rsthis: &mut QXmlStreamWriter) -> QIODevice {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamWriter6deviceEv()};
    let mut ret = unsafe {_ZNK16QXmlStreamWriter6deviceEv(rsthis.qclsinst)};
    let mut ret1 = QIODevice{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QXmlStreamWriter::writeCurrentToken(const QXmlStreamReader & reader);
impl /*struct*/ QXmlStreamWriter {
  pub fn writeCurrentToken<RetType, T: QXmlStreamWriter_writeCurrentToken<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.writeCurrentToken(self);
    // return 1;
  }
}

pub trait QXmlStreamWriter_writeCurrentToken<RetType> {
  fn writeCurrentToken(self , rsthis: &mut QXmlStreamWriter) -> RetType;
}

// proto:  void QXmlStreamWriter::writeCurrentToken(const QXmlStreamReader & reader);
impl<'a> /*trait*/ QXmlStreamWriter_writeCurrentToken<()> for (&'a  QXmlStreamReader) {
  fn writeCurrentToken(self , rsthis: &mut QXmlStreamWriter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriter17writeCurrentTokenERK16QXmlStreamReader()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QXmlStreamWriter17writeCurrentTokenERK16QXmlStreamReader(rsthis.qclsinst, arg0)};
    // return 1;
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

// proto:  void QXmlStreamWriter::writeTextElement(const QString & qualifiedName, const QString & text);
impl<'a> /*trait*/ QXmlStreamWriter_writeTextElement<()> for (&'a  QString, &'a  QString) {
  fn writeTextElement(self , rsthis: &mut QXmlStreamWriter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriter16writeTextElementERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN16QXmlStreamWriter16writeTextElementERK7QStringS2_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  void QXmlStreamWriter::writeEmptyElement(const QString & namespaceUri, const QString & name);
impl<'a> /*trait*/ QXmlStreamWriter_writeEmptyElement<()> for (&'a  QString, &'a  QString) {
  fn writeEmptyElement(self , rsthis: &mut QXmlStreamWriter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriter17writeEmptyElementERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN16QXmlStreamWriter17writeEmptyElementERK7QStringS2_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
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

// proto:  void QXmlStreamWriter::writeStartElement(const QString & qualifiedName);
impl<'a> /*trait*/ QXmlStreamWriter_writeStartElement<()> for (&'a  QString) {
  fn writeStartElement(self , rsthis: &mut QXmlStreamWriter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriter17writeStartElementERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QXmlStreamWriter17writeStartElementERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

