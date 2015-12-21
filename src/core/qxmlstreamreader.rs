// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qxmlstreamentityresolver::QXmlStreamEntityResolver;
use super::qstring::QString;
use super::qxmlstreamattributes::QXmlStreamAttributes;
use super::qxmlstreamnamespacedeclaration::QXmlStreamNamespaceDeclaration;
use super::qbytearray::QByteArray;
use super::qiodevice::QIODevice;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  QStringRef QXmlStreamReader::name();
  fn _ZNK16QXmlStreamReader4nameEv(qthis: *mut c_void);
  // proto:  QXmlStreamEntityResolver * QXmlStreamReader::entityResolver();
  fn _ZNK16QXmlStreamReader14entityResolverEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QXmlStreamReader::namespaceProcessing();
  fn _ZNK16QXmlStreamReader19namespaceProcessingEv(qthis: *mut c_void) -> c_char;
  // proto:  bool QXmlStreamReader::isStartElement();
  fn _ZNK16QXmlStreamReader14isStartElementEv(qthis: *mut c_void) -> c_char;
  // proto:  bool QXmlStreamReader::isStandaloneDocument();
  fn _ZNK16QXmlStreamReader20isStandaloneDocumentEv(qthis: *mut c_void) -> c_char;
  // proto:  qint64 QXmlStreamReader::lineNumber();
  fn _ZNK16QXmlStreamReader10lineNumberEv(qthis: *mut c_void) -> c_longlong;
  // proto:  void QXmlStreamReader::clear();
  fn _ZN16QXmlStreamReader5clearEv(qthis: *mut c_void);
  // proto:  QStringRef QXmlStreamReader::processingInstructionData();
  fn _ZNK16QXmlStreamReader25processingInstructionDataEv(qthis: *mut c_void);
  // proto:  void QXmlStreamReader::addData(const QString & data);
  fn _ZN16QXmlStreamReader7addDataERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QStringRef QXmlStreamReader::dtdPublicId();
  fn _ZNK16QXmlStreamReader11dtdPublicIdEv(qthis: *mut c_void);
  // proto:  QStringRef QXmlStreamReader::documentEncoding();
  fn _ZNK16QXmlStreamReader16documentEncodingEv(qthis: *mut c_void);
  // proto:  qint64 QXmlStreamReader::characterOffset();
  fn _ZNK16QXmlStreamReader15characterOffsetEv(qthis: *mut c_void) -> c_longlong;
  // proto:  QXmlStreamAttributes QXmlStreamReader::attributes();
  fn _ZNK16QXmlStreamReader10attributesEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QString QXmlStreamReader::tokenString();
  fn _ZNK16QXmlStreamReader11tokenStringEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QXmlStreamReader::addExtraNamespaceDeclaration(const QXmlStreamNamespaceDeclaration & extraNamespaceDeclaraction);
  fn _ZN16QXmlStreamReader28addExtraNamespaceDeclarationERK30QXmlStreamNamespaceDeclaration(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QXmlStreamReader::QXmlStreamReader(const QByteArray & data);
  fn _ZN16QXmlStreamReaderC1ERK10QByteArray(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QStringRef QXmlStreamReader::qualifiedName();
  fn _ZNK16QXmlStreamReader13qualifiedNameEv(qthis: *mut c_void);
  // proto:  QIODevice * QXmlStreamReader::device();
  fn _ZNK16QXmlStreamReader6deviceEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QStringRef QXmlStreamReader::namespaceUri();
  fn _ZNK16QXmlStreamReader12namespaceUriEv(qthis: *mut c_void);
  // proto:  QStringRef QXmlStreamReader::text();
  fn _ZNK16QXmlStreamReader4textEv(qthis: *mut c_void);
  // proto:  void QXmlStreamReader::setDevice(QIODevice * device);
  fn _ZN16QXmlStreamReader9setDeviceEP9QIODevice(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QXmlStreamReader::QXmlStreamReader(QIODevice * device);
  fn _ZN16QXmlStreamReaderC1EP9QIODevice(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QStringRef QXmlStreamReader::documentVersion();
  fn _ZNK16QXmlStreamReader15documentVersionEv(qthis: *mut c_void);
  // proto:  bool QXmlStreamReader::isDTD();
  fn _ZNK16QXmlStreamReader5isDTDEv(qthis: *mut c_void) -> c_char;
  // proto:  bool QXmlStreamReader::isStartDocument();
  fn _ZNK16QXmlStreamReader15isStartDocumentEv(qthis: *mut c_void) -> c_char;
  // proto:  QString QXmlStreamReader::errorString();
  fn _ZNK16QXmlStreamReader11errorStringEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QXmlStreamReader::isProcessingInstruction();
  fn _ZNK16QXmlStreamReader23isProcessingInstructionEv(qthis: *mut c_void) -> c_char;
  // proto:  void QXmlStreamReader::setEntityResolver(QXmlStreamEntityResolver * resolver);
  fn _ZN16QXmlStreamReader17setEntityResolverEP24QXmlStreamEntityResolver(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QXmlStreamReader::isCharacters();
  fn _ZNK16QXmlStreamReader12isCharactersEv(qthis: *mut c_void) -> c_char;
  // proto:  void QXmlStreamReader::QXmlStreamReader();
  fn _ZN16QXmlStreamReaderC1Ev(qthis: *mut c_void);
  // proto:  void QXmlStreamReader::QXmlStreamReader(const QString & data);
  fn _ZN16QXmlStreamReaderC1ERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QXmlStreamEntityDeclarations QXmlStreamReader::entityDeclarations();
  fn _ZNK16QXmlStreamReader18entityDeclarationsEv(qthis: *mut c_void);
  // proto:  bool QXmlStreamReader::isWhitespace();
  fn _ZNK16QXmlStreamReader12isWhitespaceEv(qthis: *mut c_void) -> c_char;
  // proto:  void QXmlStreamReader::QXmlStreamReader(const QXmlStreamReader & );
  fn _ZN16QXmlStreamReaderC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  qint64 QXmlStreamReader::columnNumber();
  fn _ZNK16QXmlStreamReader12columnNumberEv(qthis: *mut c_void) -> c_longlong;
  // proto:  bool QXmlStreamReader::hasError();
  fn _ZNK16QXmlStreamReader8hasErrorEv(qthis: *mut c_void) -> c_char;
  // proto:  bool QXmlStreamReader::isCDATA();
  fn _ZNK16QXmlStreamReader7isCDATAEv(qthis: *mut c_void) -> c_char;
  // proto:  void QXmlStreamReader::~QXmlStreamReader();
  fn _ZN16QXmlStreamReaderD0Ev(qthis: *mut c_void);
  // proto:  QStringRef QXmlStreamReader::processingInstructionTarget();
  fn _ZNK16QXmlStreamReader27processingInstructionTargetEv(qthis: *mut c_void);
  // proto:  void QXmlStreamReader::addData(const char * data);
  fn _ZN16QXmlStreamReader7addDataEPKc(qthis: *mut c_void, arg0: *mut c_char);
  // proto:  QStringRef QXmlStreamReader::dtdSystemId();
  fn _ZNK16QXmlStreamReader11dtdSystemIdEv(qthis: *mut c_void);
  // proto:  QStringRef QXmlStreamReader::prefix();
  fn _ZNK16QXmlStreamReader6prefixEv(qthis: *mut c_void);
  // proto:  bool QXmlStreamReader::isEndElement();
  fn _ZNK16QXmlStreamReader12isEndElementEv(qthis: *mut c_void) -> c_char;
  // proto:  QXmlStreamNotationDeclarations QXmlStreamReader::notationDeclarations();
  fn _ZNK16QXmlStreamReader20notationDeclarationsEv(qthis: *mut c_void);
  // proto:  QXmlStreamNamespaceDeclarations QXmlStreamReader::namespaceDeclarations();
  fn _ZNK16QXmlStreamReader21namespaceDeclarationsEv(qthis: *mut c_void);
  // proto:  void QXmlStreamReader::setNamespaceProcessing(bool );
  fn _ZN16QXmlStreamReader22setNamespaceProcessingEb(qthis: *mut c_void, arg0: c_char);
  // proto:  void QXmlStreamReader::raiseError(const QString & message);
  fn _ZN16QXmlStreamReader10raiseErrorERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QStringRef QXmlStreamReader::dtdName();
  fn _ZNK16QXmlStreamReader7dtdNameEv(qthis: *mut c_void);
  // proto:  bool QXmlStreamReader::isEndDocument();
  fn _ZNK16QXmlStreamReader13isEndDocumentEv(qthis: *mut c_void) -> c_char;
  // proto:  bool QXmlStreamReader::readNextStartElement();
  fn _ZN16QXmlStreamReader20readNextStartElementEv(qthis: *mut c_void) -> c_char;
  // proto:  bool QXmlStreamReader::isComment();
  fn _ZNK16QXmlStreamReader9isCommentEv(qthis: *mut c_void) -> c_char;
  // proto:  void QXmlStreamReader::QXmlStreamReader(const char * data);
  fn _ZN16QXmlStreamReaderC1EPKc(qthis: *mut c_void, arg0: *mut c_char);
  // proto:  void QXmlStreamReader::skipCurrentElement();
  fn _ZN16QXmlStreamReader18skipCurrentElementEv(qthis: *mut c_void);
  // proto:  bool QXmlStreamReader::isEntityReference();
  fn _ZNK16QXmlStreamReader17isEntityReferenceEv(qthis: *mut c_void) -> c_char;
  // proto:  void QXmlStreamReader::addData(const QByteArray & data);
  fn _ZN16QXmlStreamReader7addDataERK10QByteArray(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QXmlStreamReader::atEnd();
  fn _ZNK16QXmlStreamReader5atEndEv(qthis: *mut c_void) -> c_char;
}

// body block begin
// class sizeof(QXmlStreamReader)=1
pub struct QXmlStreamReader {
  pub qclsinst: *mut c_void,
}

  // proto:  QStringRef QXmlStreamReader::name();
impl /*struct*/ QXmlStreamReader {
  pub fn name<RetType, T: QXmlStreamReader_name<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.name(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_name<RetType> {
  fn name(self , rsthis: &mut QXmlStreamReader) -> RetType;
}

  // proto:  QStringRef QXmlStreamReader::name();
impl<'a> /*trait*/ QXmlStreamReader_name<()> for () {
  fn name(self , rsthis: &mut QXmlStreamReader) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader4nameEv()};
     unsafe {_ZNK16QXmlStreamReader4nameEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QXmlStreamEntityResolver * QXmlStreamReader::entityResolver();
impl /*struct*/ QXmlStreamReader {
  pub fn entityResolver<RetType, T: QXmlStreamReader_entityResolver<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.entityResolver(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_entityResolver<RetType> {
  fn entityResolver(self , rsthis: &mut QXmlStreamReader) -> RetType;
}

  // proto:  QXmlStreamEntityResolver * QXmlStreamReader::entityResolver();
impl<'a> /*trait*/ QXmlStreamReader_entityResolver<QXmlStreamEntityResolver> for () {
  fn entityResolver(self , rsthis: &mut QXmlStreamReader) -> QXmlStreamEntityResolver {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader14entityResolverEv()};
    let mut ret = unsafe {_ZNK16QXmlStreamReader14entityResolverEv(rsthis.qclsinst)};
    let mut ret1 = QXmlStreamEntityResolver{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  bool QXmlStreamReader::namespaceProcessing();
impl /*struct*/ QXmlStreamReader {
  pub fn namespaceProcessing<RetType, T: QXmlStreamReader_namespaceProcessing<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.namespaceProcessing(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_namespaceProcessing<RetType> {
  fn namespaceProcessing(self , rsthis: &mut QXmlStreamReader) -> RetType;
}

  // proto:  bool QXmlStreamReader::namespaceProcessing();
impl<'a> /*trait*/ QXmlStreamReader_namespaceProcessing<i8> for () {
  fn namespaceProcessing(self , rsthis: &mut QXmlStreamReader) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader19namespaceProcessingEv()};
    let mut ret = unsafe {_ZNK16QXmlStreamReader19namespaceProcessingEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QXmlStreamReader::isStartElement();
impl /*struct*/ QXmlStreamReader {
  pub fn isStartElement<RetType, T: QXmlStreamReader_isStartElement<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isStartElement(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_isStartElement<RetType> {
  fn isStartElement(self , rsthis: &mut QXmlStreamReader) -> RetType;
}

  // proto:  bool QXmlStreamReader::isStartElement();
impl<'a> /*trait*/ QXmlStreamReader_isStartElement<i8> for () {
  fn isStartElement(self , rsthis: &mut QXmlStreamReader) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader14isStartElementEv()};
    let mut ret = unsafe {_ZNK16QXmlStreamReader14isStartElementEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QXmlStreamReader::isStandaloneDocument();
impl /*struct*/ QXmlStreamReader {
  pub fn isStandaloneDocument<RetType, T: QXmlStreamReader_isStandaloneDocument<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isStandaloneDocument(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_isStandaloneDocument<RetType> {
  fn isStandaloneDocument(self , rsthis: &mut QXmlStreamReader) -> RetType;
}

  // proto:  bool QXmlStreamReader::isStandaloneDocument();
impl<'a> /*trait*/ QXmlStreamReader_isStandaloneDocument<i8> for () {
  fn isStandaloneDocument(self , rsthis: &mut QXmlStreamReader) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader20isStandaloneDocumentEv()};
    let mut ret = unsafe {_ZNK16QXmlStreamReader20isStandaloneDocumentEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  qint64 QXmlStreamReader::lineNumber();
impl /*struct*/ QXmlStreamReader {
  pub fn lineNumber<RetType, T: QXmlStreamReader_lineNumber<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.lineNumber(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_lineNumber<RetType> {
  fn lineNumber(self , rsthis: &mut QXmlStreamReader) -> RetType;
}

  // proto:  qint64 QXmlStreamReader::lineNumber();
impl<'a> /*trait*/ QXmlStreamReader_lineNumber<i64> for () {
  fn lineNumber(self , rsthis: &mut QXmlStreamReader) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader10lineNumberEv()};
    let mut ret = unsafe {_ZNK16QXmlStreamReader10lineNumberEv(rsthis.qclsinst)};
    return ret as i64;
    // return 1;
  }
}

  // proto:  void QXmlStreamReader::clear();
impl /*struct*/ QXmlStreamReader {
  pub fn clear<RetType, T: QXmlStreamReader_clear<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.clear(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_clear<RetType> {
  fn clear(self , rsthis: &mut QXmlStreamReader) -> RetType;
}

  // proto:  void QXmlStreamReader::clear();
impl<'a> /*trait*/ QXmlStreamReader_clear<()> for () {
  fn clear(self , rsthis: &mut QXmlStreamReader) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamReader5clearEv()};
     unsafe {_ZN16QXmlStreamReader5clearEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QStringRef QXmlStreamReader::processingInstructionData();
impl /*struct*/ QXmlStreamReader {
  pub fn processingInstructionData<RetType, T: QXmlStreamReader_processingInstructionData<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.processingInstructionData(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_processingInstructionData<RetType> {
  fn processingInstructionData(self , rsthis: &mut QXmlStreamReader) -> RetType;
}

  // proto:  QStringRef QXmlStreamReader::processingInstructionData();
impl<'a> /*trait*/ QXmlStreamReader_processingInstructionData<()> for () {
  fn processingInstructionData(self , rsthis: &mut QXmlStreamReader) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader25processingInstructionDataEv()};
     unsafe {_ZNK16QXmlStreamReader25processingInstructionDataEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QXmlStreamReader::addData(const QString & data);
impl /*struct*/ QXmlStreamReader {
  pub fn addData<RetType, T: QXmlStreamReader_addData<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.addData(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_addData<RetType> {
  fn addData(self , rsthis: &mut QXmlStreamReader) -> RetType;
}

  // proto:  void QXmlStreamReader::addData(const QString & data);
impl<'a> /*trait*/ QXmlStreamReader_addData<()> for (QString) {
  fn addData(self , rsthis: &mut QXmlStreamReader) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamReader7addDataERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QXmlStreamReader7addDataERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QStringRef QXmlStreamReader::dtdPublicId();
impl /*struct*/ QXmlStreamReader {
  pub fn dtdPublicId<RetType, T: QXmlStreamReader_dtdPublicId<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.dtdPublicId(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_dtdPublicId<RetType> {
  fn dtdPublicId(self , rsthis: &mut QXmlStreamReader) -> RetType;
}

  // proto:  QStringRef QXmlStreamReader::dtdPublicId();
impl<'a> /*trait*/ QXmlStreamReader_dtdPublicId<()> for () {
  fn dtdPublicId(self , rsthis: &mut QXmlStreamReader) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader11dtdPublicIdEv()};
     unsafe {_ZNK16QXmlStreamReader11dtdPublicIdEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QStringRef QXmlStreamReader::documentEncoding();
impl /*struct*/ QXmlStreamReader {
  pub fn documentEncoding<RetType, T: QXmlStreamReader_documentEncoding<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.documentEncoding(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_documentEncoding<RetType> {
  fn documentEncoding(self , rsthis: &mut QXmlStreamReader) -> RetType;
}

  // proto:  QStringRef QXmlStreamReader::documentEncoding();
impl<'a> /*trait*/ QXmlStreamReader_documentEncoding<()> for () {
  fn documentEncoding(self , rsthis: &mut QXmlStreamReader) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader16documentEncodingEv()};
     unsafe {_ZNK16QXmlStreamReader16documentEncodingEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  qint64 QXmlStreamReader::characterOffset();
impl /*struct*/ QXmlStreamReader {
  pub fn characterOffset<RetType, T: QXmlStreamReader_characterOffset<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.characterOffset(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_characterOffset<RetType> {
  fn characterOffset(self , rsthis: &mut QXmlStreamReader) -> RetType;
}

  // proto:  qint64 QXmlStreamReader::characterOffset();
impl<'a> /*trait*/ QXmlStreamReader_characterOffset<i64> for () {
  fn characterOffset(self , rsthis: &mut QXmlStreamReader) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader15characterOffsetEv()};
    let mut ret = unsafe {_ZNK16QXmlStreamReader15characterOffsetEv(rsthis.qclsinst)};
    return ret as i64;
    // return 1;
  }
}

  // proto:  QXmlStreamAttributes QXmlStreamReader::attributes();
impl /*struct*/ QXmlStreamReader {
  pub fn attributes<RetType, T: QXmlStreamReader_attributes<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.attributes(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_attributes<RetType> {
  fn attributes(self , rsthis: &mut QXmlStreamReader) -> RetType;
}

  // proto:  QXmlStreamAttributes QXmlStreamReader::attributes();
impl<'a> /*trait*/ QXmlStreamReader_attributes<QXmlStreamAttributes> for () {
  fn attributes(self , rsthis: &mut QXmlStreamReader) -> QXmlStreamAttributes {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader10attributesEv()};
    let mut ret = unsafe {_ZNK16QXmlStreamReader10attributesEv(rsthis.qclsinst)};
    let mut ret1 = QXmlStreamAttributes{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QString QXmlStreamReader::tokenString();
impl /*struct*/ QXmlStreamReader {
  pub fn tokenString<RetType, T: QXmlStreamReader_tokenString<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.tokenString(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_tokenString<RetType> {
  fn tokenString(self , rsthis: &mut QXmlStreamReader) -> RetType;
}

  // proto:  QString QXmlStreamReader::tokenString();
impl<'a> /*trait*/ QXmlStreamReader_tokenString<QString> for () {
  fn tokenString(self , rsthis: &mut QXmlStreamReader) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader11tokenStringEv()};
    let mut ret = unsafe {_ZNK16QXmlStreamReader11tokenStringEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QXmlStreamReader::addExtraNamespaceDeclaration(const QXmlStreamNamespaceDeclaration & extraNamespaceDeclaraction);
impl /*struct*/ QXmlStreamReader {
  pub fn addExtraNamespaceDeclaration<RetType, T: QXmlStreamReader_addExtraNamespaceDeclaration<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.addExtraNamespaceDeclaration(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_addExtraNamespaceDeclaration<RetType> {
  fn addExtraNamespaceDeclaration(self , rsthis: &mut QXmlStreamReader) -> RetType;
}

  // proto:  void QXmlStreamReader::addExtraNamespaceDeclaration(const QXmlStreamNamespaceDeclaration & extraNamespaceDeclaraction);
impl<'a> /*trait*/ QXmlStreamReader_addExtraNamespaceDeclaration<()> for (QXmlStreamNamespaceDeclaration) {
  fn addExtraNamespaceDeclaration(self , rsthis: &mut QXmlStreamReader) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamReader28addExtraNamespaceDeclarationERK30QXmlStreamNamespaceDeclaration()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QXmlStreamReader28addExtraNamespaceDeclarationERK30QXmlStreamNamespaceDeclaration(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QXmlStreamReader::QXmlStreamReader(const QByteArray & data);
impl /*struct*/ QXmlStreamReader {
  pub fn NewQXmlStreamReader<T: QXmlStreamReader_NewQXmlStreamReader>(value: T) -> QXmlStreamReader {
    let rsthis = value.NewQXmlStreamReader();
    return rsthis;
    // return 1;
  }
}

pub trait QXmlStreamReader_NewQXmlStreamReader {
  fn NewQXmlStreamReader(self) -> QXmlStreamReader;
}

  // proto:  void QXmlStreamReader::QXmlStreamReader(const QByteArray & data);
impl<'a> /*trait*/ QXmlStreamReader_NewQXmlStreamReader for (QByteArray) {
  fn NewQXmlStreamReader(self) -> QXmlStreamReader {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamReaderC1ERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN16QXmlStreamReaderC1ERK10QByteArray(qthis, arg0)};
    let rsthis = QXmlStreamReader{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QStringRef QXmlStreamReader::qualifiedName();
impl /*struct*/ QXmlStreamReader {
  pub fn qualifiedName<RetType, T: QXmlStreamReader_qualifiedName<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.qualifiedName(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_qualifiedName<RetType> {
  fn qualifiedName(self , rsthis: &mut QXmlStreamReader) -> RetType;
}

  // proto:  QStringRef QXmlStreamReader::qualifiedName();
impl<'a> /*trait*/ QXmlStreamReader_qualifiedName<()> for () {
  fn qualifiedName(self , rsthis: &mut QXmlStreamReader) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader13qualifiedNameEv()};
     unsafe {_ZNK16QXmlStreamReader13qualifiedNameEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QIODevice * QXmlStreamReader::device();
impl /*struct*/ QXmlStreamReader {
  pub fn device<RetType, T: QXmlStreamReader_device<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.device(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_device<RetType> {
  fn device(self , rsthis: &mut QXmlStreamReader) -> RetType;
}

  // proto:  QIODevice * QXmlStreamReader::device();
impl<'a> /*trait*/ QXmlStreamReader_device<QIODevice> for () {
  fn device(self , rsthis: &mut QXmlStreamReader) -> QIODevice {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader6deviceEv()};
    let mut ret = unsafe {_ZNK16QXmlStreamReader6deviceEv(rsthis.qclsinst)};
    let mut ret1 = QIODevice{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QStringRef QXmlStreamReader::namespaceUri();
impl /*struct*/ QXmlStreamReader {
  pub fn namespaceUri<RetType, T: QXmlStreamReader_namespaceUri<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.namespaceUri(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_namespaceUri<RetType> {
  fn namespaceUri(self , rsthis: &mut QXmlStreamReader) -> RetType;
}

  // proto:  QStringRef QXmlStreamReader::namespaceUri();
impl<'a> /*trait*/ QXmlStreamReader_namespaceUri<()> for () {
  fn namespaceUri(self , rsthis: &mut QXmlStreamReader) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader12namespaceUriEv()};
     unsafe {_ZNK16QXmlStreamReader12namespaceUriEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QStringRef QXmlStreamReader::text();
impl /*struct*/ QXmlStreamReader {
  pub fn text<RetType, T: QXmlStreamReader_text<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.text(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_text<RetType> {
  fn text(self , rsthis: &mut QXmlStreamReader) -> RetType;
}

  // proto:  QStringRef QXmlStreamReader::text();
impl<'a> /*trait*/ QXmlStreamReader_text<()> for () {
  fn text(self , rsthis: &mut QXmlStreamReader) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader4textEv()};
     unsafe {_ZNK16QXmlStreamReader4textEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QXmlStreamReader::setDevice(QIODevice * device);
impl /*struct*/ QXmlStreamReader {
  pub fn setDevice<RetType, T: QXmlStreamReader_setDevice<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setDevice(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_setDevice<RetType> {
  fn setDevice(self , rsthis: &mut QXmlStreamReader) -> RetType;
}

  // proto:  void QXmlStreamReader::setDevice(QIODevice * device);
impl<'a> /*trait*/ QXmlStreamReader_setDevice<()> for (QIODevice) {
  fn setDevice(self , rsthis: &mut QXmlStreamReader) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamReader9setDeviceEP9QIODevice()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QXmlStreamReader9setDeviceEP9QIODevice(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QXmlStreamReader::QXmlStreamReader(QIODevice * device);
impl<'a> /*trait*/ QXmlStreamReader_NewQXmlStreamReader for (QIODevice) {
  fn NewQXmlStreamReader(self) -> QXmlStreamReader {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamReaderC1EP9QIODevice()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN16QXmlStreamReaderC1EP9QIODevice(qthis, arg0)};
    let rsthis = QXmlStreamReader{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QStringRef QXmlStreamReader::documentVersion();
impl /*struct*/ QXmlStreamReader {
  pub fn documentVersion<RetType, T: QXmlStreamReader_documentVersion<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.documentVersion(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_documentVersion<RetType> {
  fn documentVersion(self , rsthis: &mut QXmlStreamReader) -> RetType;
}

  // proto:  QStringRef QXmlStreamReader::documentVersion();
impl<'a> /*trait*/ QXmlStreamReader_documentVersion<()> for () {
  fn documentVersion(self , rsthis: &mut QXmlStreamReader) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader15documentVersionEv()};
     unsafe {_ZNK16QXmlStreamReader15documentVersionEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QXmlStreamReader::isDTD();
impl /*struct*/ QXmlStreamReader {
  pub fn isDTD<RetType, T: QXmlStreamReader_isDTD<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isDTD(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_isDTD<RetType> {
  fn isDTD(self , rsthis: &mut QXmlStreamReader) -> RetType;
}

  // proto:  bool QXmlStreamReader::isDTD();
impl<'a> /*trait*/ QXmlStreamReader_isDTD<i8> for () {
  fn isDTD(self , rsthis: &mut QXmlStreamReader) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader5isDTDEv()};
    let mut ret = unsafe {_ZNK16QXmlStreamReader5isDTDEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QXmlStreamReader::isStartDocument();
impl /*struct*/ QXmlStreamReader {
  pub fn isStartDocument<RetType, T: QXmlStreamReader_isStartDocument<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isStartDocument(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_isStartDocument<RetType> {
  fn isStartDocument(self , rsthis: &mut QXmlStreamReader) -> RetType;
}

  // proto:  bool QXmlStreamReader::isStartDocument();
impl<'a> /*trait*/ QXmlStreamReader_isStartDocument<i8> for () {
  fn isStartDocument(self , rsthis: &mut QXmlStreamReader) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader15isStartDocumentEv()};
    let mut ret = unsafe {_ZNK16QXmlStreamReader15isStartDocumentEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QString QXmlStreamReader::errorString();
impl /*struct*/ QXmlStreamReader {
  pub fn errorString<RetType, T: QXmlStreamReader_errorString<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.errorString(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_errorString<RetType> {
  fn errorString(self , rsthis: &mut QXmlStreamReader) -> RetType;
}

  // proto:  QString QXmlStreamReader::errorString();
impl<'a> /*trait*/ QXmlStreamReader_errorString<QString> for () {
  fn errorString(self , rsthis: &mut QXmlStreamReader) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader11errorStringEv()};
    let mut ret = unsafe {_ZNK16QXmlStreamReader11errorStringEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  bool QXmlStreamReader::isProcessingInstruction();
impl /*struct*/ QXmlStreamReader {
  pub fn isProcessingInstruction<RetType, T: QXmlStreamReader_isProcessingInstruction<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isProcessingInstruction(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_isProcessingInstruction<RetType> {
  fn isProcessingInstruction(self , rsthis: &mut QXmlStreamReader) -> RetType;
}

  // proto:  bool QXmlStreamReader::isProcessingInstruction();
impl<'a> /*trait*/ QXmlStreamReader_isProcessingInstruction<i8> for () {
  fn isProcessingInstruction(self , rsthis: &mut QXmlStreamReader) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader23isProcessingInstructionEv()};
    let mut ret = unsafe {_ZNK16QXmlStreamReader23isProcessingInstructionEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QXmlStreamReader::setEntityResolver(QXmlStreamEntityResolver * resolver);
impl /*struct*/ QXmlStreamReader {
  pub fn setEntityResolver<RetType, T: QXmlStreamReader_setEntityResolver<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setEntityResolver(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_setEntityResolver<RetType> {
  fn setEntityResolver(self , rsthis: &mut QXmlStreamReader) -> RetType;
}

  // proto:  void QXmlStreamReader::setEntityResolver(QXmlStreamEntityResolver * resolver);
impl<'a> /*trait*/ QXmlStreamReader_setEntityResolver<()> for (QXmlStreamEntityResolver) {
  fn setEntityResolver(self , rsthis: &mut QXmlStreamReader) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamReader17setEntityResolverEP24QXmlStreamEntityResolver()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QXmlStreamReader17setEntityResolverEP24QXmlStreamEntityResolver(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QXmlStreamReader::isCharacters();
impl /*struct*/ QXmlStreamReader {
  pub fn isCharacters<RetType, T: QXmlStreamReader_isCharacters<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isCharacters(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_isCharacters<RetType> {
  fn isCharacters(self , rsthis: &mut QXmlStreamReader) -> RetType;
}

  // proto:  bool QXmlStreamReader::isCharacters();
impl<'a> /*trait*/ QXmlStreamReader_isCharacters<i8> for () {
  fn isCharacters(self , rsthis: &mut QXmlStreamReader) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader12isCharactersEv()};
    let mut ret = unsafe {_ZNK16QXmlStreamReader12isCharactersEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QXmlStreamReader::QXmlStreamReader();
impl<'a> /*trait*/ QXmlStreamReader_NewQXmlStreamReader for () {
  fn NewQXmlStreamReader(self) -> QXmlStreamReader {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamReaderC1Ev()};
    unsafe {_ZN16QXmlStreamReaderC1Ev(qthis)};
    let rsthis = QXmlStreamReader{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QXmlStreamReader::QXmlStreamReader(const QString & data);
impl<'a> /*trait*/ QXmlStreamReader_NewQXmlStreamReader for (QString) {
  fn NewQXmlStreamReader(self) -> QXmlStreamReader {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamReaderC1ERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN16QXmlStreamReaderC1ERK7QString(qthis, arg0)};
    let rsthis = QXmlStreamReader{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QXmlStreamEntityDeclarations QXmlStreamReader::entityDeclarations();
impl /*struct*/ QXmlStreamReader {
  pub fn entityDeclarations<RetType, T: QXmlStreamReader_entityDeclarations<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.entityDeclarations(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_entityDeclarations<RetType> {
  fn entityDeclarations(self , rsthis: &mut QXmlStreamReader) -> RetType;
}

  // proto:  QXmlStreamEntityDeclarations QXmlStreamReader::entityDeclarations();
impl<'a> /*trait*/ QXmlStreamReader_entityDeclarations<()> for () {
  fn entityDeclarations(self , rsthis: &mut QXmlStreamReader) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader18entityDeclarationsEv()};
     unsafe {_ZNK16QXmlStreamReader18entityDeclarationsEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QXmlStreamReader::isWhitespace();
impl /*struct*/ QXmlStreamReader {
  pub fn isWhitespace<RetType, T: QXmlStreamReader_isWhitespace<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isWhitespace(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_isWhitespace<RetType> {
  fn isWhitespace(self , rsthis: &mut QXmlStreamReader) -> RetType;
}

  // proto:  bool QXmlStreamReader::isWhitespace();
impl<'a> /*trait*/ QXmlStreamReader_isWhitespace<i8> for () {
  fn isWhitespace(self , rsthis: &mut QXmlStreamReader) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader12isWhitespaceEv()};
    let mut ret = unsafe {_ZNK16QXmlStreamReader12isWhitespaceEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QXmlStreamReader::QXmlStreamReader(const QXmlStreamReader & );
impl<'a> /*trait*/ QXmlStreamReader_NewQXmlStreamReader for (QXmlStreamReader) {
  fn NewQXmlStreamReader(self) -> QXmlStreamReader {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamReaderC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN16QXmlStreamReaderC1ERKS_(qthis, arg0)};
    let rsthis = QXmlStreamReader{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  qint64 QXmlStreamReader::columnNumber();
impl /*struct*/ QXmlStreamReader {
  pub fn columnNumber<RetType, T: QXmlStreamReader_columnNumber<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.columnNumber(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_columnNumber<RetType> {
  fn columnNumber(self , rsthis: &mut QXmlStreamReader) -> RetType;
}

  // proto:  qint64 QXmlStreamReader::columnNumber();
impl<'a> /*trait*/ QXmlStreamReader_columnNumber<i64> for () {
  fn columnNumber(self , rsthis: &mut QXmlStreamReader) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader12columnNumberEv()};
    let mut ret = unsafe {_ZNK16QXmlStreamReader12columnNumberEv(rsthis.qclsinst)};
    return ret as i64;
    // return 1;
  }
}

  // proto:  bool QXmlStreamReader::hasError();
impl /*struct*/ QXmlStreamReader {
  pub fn hasError<RetType, T: QXmlStreamReader_hasError<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.hasError(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_hasError<RetType> {
  fn hasError(self , rsthis: &mut QXmlStreamReader) -> RetType;
}

  // proto:  bool QXmlStreamReader::hasError();
impl<'a> /*trait*/ QXmlStreamReader_hasError<i8> for () {
  fn hasError(self , rsthis: &mut QXmlStreamReader) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader8hasErrorEv()};
    let mut ret = unsafe {_ZNK16QXmlStreamReader8hasErrorEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QXmlStreamReader::isCDATA();
impl /*struct*/ QXmlStreamReader {
  pub fn isCDATA<RetType, T: QXmlStreamReader_isCDATA<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isCDATA(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_isCDATA<RetType> {
  fn isCDATA(self , rsthis: &mut QXmlStreamReader) -> RetType;
}

  // proto:  bool QXmlStreamReader::isCDATA();
impl<'a> /*trait*/ QXmlStreamReader_isCDATA<i8> for () {
  fn isCDATA(self , rsthis: &mut QXmlStreamReader) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader7isCDATAEv()};
    let mut ret = unsafe {_ZNK16QXmlStreamReader7isCDATAEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QXmlStreamReader::~QXmlStreamReader();
impl /*struct*/ QXmlStreamReader {
  pub fn FreeQXmlStreamReader<RetType, T: QXmlStreamReader_FreeQXmlStreamReader<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQXmlStreamReader(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_FreeQXmlStreamReader<RetType> {
  fn FreeQXmlStreamReader(self , rsthis: &mut QXmlStreamReader) -> RetType;
}

  // proto:  void QXmlStreamReader::~QXmlStreamReader();
impl<'a> /*trait*/ QXmlStreamReader_FreeQXmlStreamReader<()> for () {
  fn FreeQXmlStreamReader(self , rsthis: &mut QXmlStreamReader) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamReaderD0Ev()};
     unsafe {_ZN16QXmlStreamReaderD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QStringRef QXmlStreamReader::processingInstructionTarget();
impl /*struct*/ QXmlStreamReader {
  pub fn processingInstructionTarget<RetType, T: QXmlStreamReader_processingInstructionTarget<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.processingInstructionTarget(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_processingInstructionTarget<RetType> {
  fn processingInstructionTarget(self , rsthis: &mut QXmlStreamReader) -> RetType;
}

  // proto:  QStringRef QXmlStreamReader::processingInstructionTarget();
impl<'a> /*trait*/ QXmlStreamReader_processingInstructionTarget<()> for () {
  fn processingInstructionTarget(self , rsthis: &mut QXmlStreamReader) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader27processingInstructionTargetEv()};
     unsafe {_ZNK16QXmlStreamReader27processingInstructionTargetEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QXmlStreamReader::addData(const char * data);
impl<'a> /*trait*/ QXmlStreamReader_addData<()> for (&'a  String) {
  fn addData(self , rsthis: &mut QXmlStreamReader) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamReader7addDataEPKc()};
    let arg0 = self.as_ptr()  as *mut c_char;
     unsafe {_ZN16QXmlStreamReader7addDataEPKc(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QStringRef QXmlStreamReader::dtdSystemId();
impl /*struct*/ QXmlStreamReader {
  pub fn dtdSystemId<RetType, T: QXmlStreamReader_dtdSystemId<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.dtdSystemId(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_dtdSystemId<RetType> {
  fn dtdSystemId(self , rsthis: &mut QXmlStreamReader) -> RetType;
}

  // proto:  QStringRef QXmlStreamReader::dtdSystemId();
impl<'a> /*trait*/ QXmlStreamReader_dtdSystemId<()> for () {
  fn dtdSystemId(self , rsthis: &mut QXmlStreamReader) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader11dtdSystemIdEv()};
     unsafe {_ZNK16QXmlStreamReader11dtdSystemIdEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QStringRef QXmlStreamReader::prefix();
impl /*struct*/ QXmlStreamReader {
  pub fn prefix<RetType, T: QXmlStreamReader_prefix<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.prefix(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_prefix<RetType> {
  fn prefix(self , rsthis: &mut QXmlStreamReader) -> RetType;
}

  // proto:  QStringRef QXmlStreamReader::prefix();
impl<'a> /*trait*/ QXmlStreamReader_prefix<()> for () {
  fn prefix(self , rsthis: &mut QXmlStreamReader) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader6prefixEv()};
     unsafe {_ZNK16QXmlStreamReader6prefixEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QXmlStreamReader::isEndElement();
impl /*struct*/ QXmlStreamReader {
  pub fn isEndElement<RetType, T: QXmlStreamReader_isEndElement<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isEndElement(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_isEndElement<RetType> {
  fn isEndElement(self , rsthis: &mut QXmlStreamReader) -> RetType;
}

  // proto:  bool QXmlStreamReader::isEndElement();
impl<'a> /*trait*/ QXmlStreamReader_isEndElement<i8> for () {
  fn isEndElement(self , rsthis: &mut QXmlStreamReader) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader12isEndElementEv()};
    let mut ret = unsafe {_ZNK16QXmlStreamReader12isEndElementEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QXmlStreamNotationDeclarations QXmlStreamReader::notationDeclarations();
impl /*struct*/ QXmlStreamReader {
  pub fn notationDeclarations<RetType, T: QXmlStreamReader_notationDeclarations<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.notationDeclarations(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_notationDeclarations<RetType> {
  fn notationDeclarations(self , rsthis: &mut QXmlStreamReader) -> RetType;
}

  // proto:  QXmlStreamNotationDeclarations QXmlStreamReader::notationDeclarations();
impl<'a> /*trait*/ QXmlStreamReader_notationDeclarations<()> for () {
  fn notationDeclarations(self , rsthis: &mut QXmlStreamReader) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader20notationDeclarationsEv()};
     unsafe {_ZNK16QXmlStreamReader20notationDeclarationsEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QXmlStreamNamespaceDeclarations QXmlStreamReader::namespaceDeclarations();
impl /*struct*/ QXmlStreamReader {
  pub fn namespaceDeclarations<RetType, T: QXmlStreamReader_namespaceDeclarations<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.namespaceDeclarations(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_namespaceDeclarations<RetType> {
  fn namespaceDeclarations(self , rsthis: &mut QXmlStreamReader) -> RetType;
}

  // proto:  QXmlStreamNamespaceDeclarations QXmlStreamReader::namespaceDeclarations();
impl<'a> /*trait*/ QXmlStreamReader_namespaceDeclarations<()> for () {
  fn namespaceDeclarations(self , rsthis: &mut QXmlStreamReader) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader21namespaceDeclarationsEv()};
     unsafe {_ZNK16QXmlStreamReader21namespaceDeclarationsEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QXmlStreamReader::setNamespaceProcessing(bool );
impl /*struct*/ QXmlStreamReader {
  pub fn setNamespaceProcessing<RetType, T: QXmlStreamReader_setNamespaceProcessing<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setNamespaceProcessing(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_setNamespaceProcessing<RetType> {
  fn setNamespaceProcessing(self , rsthis: &mut QXmlStreamReader) -> RetType;
}

  // proto:  void QXmlStreamReader::setNamespaceProcessing(bool );
impl<'a> /*trait*/ QXmlStreamReader_setNamespaceProcessing<()> for (i8) {
  fn setNamespaceProcessing(self , rsthis: &mut QXmlStreamReader) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamReader22setNamespaceProcessingEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN16QXmlStreamReader22setNamespaceProcessingEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QXmlStreamReader::raiseError(const QString & message);
impl /*struct*/ QXmlStreamReader {
  pub fn raiseError<RetType, T: QXmlStreamReader_raiseError<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.raiseError(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_raiseError<RetType> {
  fn raiseError(self , rsthis: &mut QXmlStreamReader) -> RetType;
}

  // proto:  void QXmlStreamReader::raiseError(const QString & message);
impl<'a> /*trait*/ QXmlStreamReader_raiseError<()> for (QString) {
  fn raiseError(self , rsthis: &mut QXmlStreamReader) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamReader10raiseErrorERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QXmlStreamReader10raiseErrorERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QStringRef QXmlStreamReader::dtdName();
impl /*struct*/ QXmlStreamReader {
  pub fn dtdName<RetType, T: QXmlStreamReader_dtdName<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.dtdName(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_dtdName<RetType> {
  fn dtdName(self , rsthis: &mut QXmlStreamReader) -> RetType;
}

  // proto:  QStringRef QXmlStreamReader::dtdName();
impl<'a> /*trait*/ QXmlStreamReader_dtdName<()> for () {
  fn dtdName(self , rsthis: &mut QXmlStreamReader) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader7dtdNameEv()};
     unsafe {_ZNK16QXmlStreamReader7dtdNameEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QXmlStreamReader::isEndDocument();
impl /*struct*/ QXmlStreamReader {
  pub fn isEndDocument<RetType, T: QXmlStreamReader_isEndDocument<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isEndDocument(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_isEndDocument<RetType> {
  fn isEndDocument(self , rsthis: &mut QXmlStreamReader) -> RetType;
}

  // proto:  bool QXmlStreamReader::isEndDocument();
impl<'a> /*trait*/ QXmlStreamReader_isEndDocument<i8> for () {
  fn isEndDocument(self , rsthis: &mut QXmlStreamReader) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader13isEndDocumentEv()};
    let mut ret = unsafe {_ZNK16QXmlStreamReader13isEndDocumentEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QXmlStreamReader::readNextStartElement();
impl /*struct*/ QXmlStreamReader {
  pub fn readNextStartElement<RetType, T: QXmlStreamReader_readNextStartElement<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.readNextStartElement(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_readNextStartElement<RetType> {
  fn readNextStartElement(self , rsthis: &mut QXmlStreamReader) -> RetType;
}

  // proto:  bool QXmlStreamReader::readNextStartElement();
impl<'a> /*trait*/ QXmlStreamReader_readNextStartElement<i8> for () {
  fn readNextStartElement(self , rsthis: &mut QXmlStreamReader) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamReader20readNextStartElementEv()};
    let mut ret = unsafe {_ZN16QXmlStreamReader20readNextStartElementEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QXmlStreamReader::isComment();
impl /*struct*/ QXmlStreamReader {
  pub fn isComment<RetType, T: QXmlStreamReader_isComment<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isComment(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_isComment<RetType> {
  fn isComment(self , rsthis: &mut QXmlStreamReader) -> RetType;
}

  // proto:  bool QXmlStreamReader::isComment();
impl<'a> /*trait*/ QXmlStreamReader_isComment<i8> for () {
  fn isComment(self , rsthis: &mut QXmlStreamReader) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader9isCommentEv()};
    let mut ret = unsafe {_ZNK16QXmlStreamReader9isCommentEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QXmlStreamReader::QXmlStreamReader(const char * data);
impl<'a> /*trait*/ QXmlStreamReader_NewQXmlStreamReader for (&'a  String) {
  fn NewQXmlStreamReader(self) -> QXmlStreamReader {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamReaderC1EPKc()};
    let arg0 = self.as_ptr()  as *mut c_char;
    unsafe {_ZN16QXmlStreamReaderC1EPKc(qthis, arg0)};
    let rsthis = QXmlStreamReader{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QXmlStreamReader::skipCurrentElement();
impl /*struct*/ QXmlStreamReader {
  pub fn skipCurrentElement<RetType, T: QXmlStreamReader_skipCurrentElement<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.skipCurrentElement(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_skipCurrentElement<RetType> {
  fn skipCurrentElement(self , rsthis: &mut QXmlStreamReader) -> RetType;
}

  // proto:  void QXmlStreamReader::skipCurrentElement();
impl<'a> /*trait*/ QXmlStreamReader_skipCurrentElement<()> for () {
  fn skipCurrentElement(self , rsthis: &mut QXmlStreamReader) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamReader18skipCurrentElementEv()};
     unsafe {_ZN16QXmlStreamReader18skipCurrentElementEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QXmlStreamReader::isEntityReference();
impl /*struct*/ QXmlStreamReader {
  pub fn isEntityReference<RetType, T: QXmlStreamReader_isEntityReference<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isEntityReference(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_isEntityReference<RetType> {
  fn isEntityReference(self , rsthis: &mut QXmlStreamReader) -> RetType;
}

  // proto:  bool QXmlStreamReader::isEntityReference();
impl<'a> /*trait*/ QXmlStreamReader_isEntityReference<i8> for () {
  fn isEntityReference(self , rsthis: &mut QXmlStreamReader) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader17isEntityReferenceEv()};
    let mut ret = unsafe {_ZNK16QXmlStreamReader17isEntityReferenceEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QXmlStreamReader::addData(const QByteArray & data);
impl<'a> /*trait*/ QXmlStreamReader_addData<()> for (QByteArray) {
  fn addData(self , rsthis: &mut QXmlStreamReader) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamReader7addDataERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QXmlStreamReader7addDataERK10QByteArray(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QXmlStreamReader::atEnd();
impl /*struct*/ QXmlStreamReader {
  pub fn atEnd<RetType, T: QXmlStreamReader_atEnd<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.atEnd(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_atEnd<RetType> {
  fn atEnd(self , rsthis: &mut QXmlStreamReader) -> RetType;
}

  // proto:  bool QXmlStreamReader::atEnd();
impl<'a> /*trait*/ QXmlStreamReader_atEnd<i8> for () {
  fn atEnd(self , rsthis: &mut QXmlStreamReader) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader5atEndEv()};
    let mut ret = unsafe {_ZNK16QXmlStreamReader5atEndEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

