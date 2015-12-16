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
  fn _ZNK16QXmlStreamReader4nameEv(qthis: *mut c_void) ;
  // proto:  QXmlStreamEntityResolver * QXmlStreamReader::entityResolver();
  fn _ZNK16QXmlStreamReader14entityResolverEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QXmlStreamReader::namespaceProcessing();
  fn _ZNK16QXmlStreamReader19namespaceProcessingEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QXmlStreamReader::isStartElement();
  fn _ZNK16QXmlStreamReader14isStartElementEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QXmlStreamReader::isStandaloneDocument();
  fn _ZNK16QXmlStreamReader20isStandaloneDocumentEv(qthis: *mut c_void) -> int8_t;
  // proto:  long long QXmlStreamReader::lineNumber();
  fn _ZNK16QXmlStreamReader10lineNumberEv(qthis: *mut c_void) -> c_longlong;
  // proto:  void QXmlStreamReader::clear();
  fn _ZN16QXmlStreamReader5clearEv(qthis: *mut c_void) ;
  // proto:  QStringRef QXmlStreamReader::processingInstructionData();
  fn _ZNK16QXmlStreamReader25processingInstructionDataEv(qthis: *mut c_void) ;
  // proto:  void QXmlStreamReader::addData(const QString & data);
  fn _ZN16QXmlStreamReader7addDataERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QStringRef QXmlStreamReader::dtdPublicId();
  fn _ZNK16QXmlStreamReader11dtdPublicIdEv(qthis: *mut c_void) ;
  // proto:  QStringRef QXmlStreamReader::documentEncoding();
  fn _ZNK16QXmlStreamReader16documentEncodingEv(qthis: *mut c_void) ;
  // proto:  long long QXmlStreamReader::characterOffset();
  fn _ZNK16QXmlStreamReader15characterOffsetEv(qthis: *mut c_void) -> c_longlong;
  // proto:  QXmlStreamAttributes QXmlStreamReader::attributes();
  fn _ZNK16QXmlStreamReader10attributesEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QString QXmlStreamReader::tokenString();
  fn _ZNK16QXmlStreamReader11tokenStringEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QXmlStreamReader::addExtraNamespaceDeclaration(const QXmlStreamNamespaceDeclaration & extraNamespaceDeclaraction);
  fn _ZN16QXmlStreamReader28addExtraNamespaceDeclarationERK30QXmlStreamNamespaceDeclaration(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QXmlStreamReader::NewQXmlStreamReader(const QByteArray & data);
  fn _ZN16QXmlStreamReaderC1ERK10QByteArray(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QStringRef QXmlStreamReader::qualifiedName();
  fn _ZNK16QXmlStreamReader13qualifiedNameEv(qthis: *mut c_void) ;
  // proto:  QIODevice * QXmlStreamReader::device();
  fn _ZNK16QXmlStreamReader6deviceEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QStringRef QXmlStreamReader::namespaceUri();
  fn _ZNK16QXmlStreamReader12namespaceUriEv(qthis: *mut c_void) ;
  // proto:  QStringRef QXmlStreamReader::text();
  fn _ZNK16QXmlStreamReader4textEv(qthis: *mut c_void) ;
  // proto:  void QXmlStreamReader::setDevice(QIODevice * device);
  fn _ZN16QXmlStreamReader9setDeviceEP9QIODevice(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QXmlStreamReader::NewQXmlStreamReader(QIODevice * device);
  fn _ZN16QXmlStreamReaderC1EP9QIODevice(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QStringRef QXmlStreamReader::documentVersion();
  fn _ZNK16QXmlStreamReader15documentVersionEv(qthis: *mut c_void) ;
  // proto:  bool QXmlStreamReader::isDTD();
  fn _ZNK16QXmlStreamReader5isDTDEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QXmlStreamReader::isStartDocument();
  fn _ZNK16QXmlStreamReader15isStartDocumentEv(qthis: *mut c_void) -> int8_t;
  // proto:  QString QXmlStreamReader::errorString();
  fn _ZNK16QXmlStreamReader11errorStringEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QXmlStreamReader::isProcessingInstruction();
  fn _ZNK16QXmlStreamReader23isProcessingInstructionEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QXmlStreamReader::setEntityResolver(QXmlStreamEntityResolver * resolver);
  fn _ZN16QXmlStreamReader17setEntityResolverEP24QXmlStreamEntityResolver(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QXmlStreamReader::isCharacters();
  fn _ZNK16QXmlStreamReader12isCharactersEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QXmlStreamReader::NewQXmlStreamReader();
  fn _ZN16QXmlStreamReaderC1Ev(qthis: *mut c_void) ;
  // proto:  void QXmlStreamReader::NewQXmlStreamReader(const QString & data);
  fn _ZN16QXmlStreamReaderC1ERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QVector<QXmlStreamEntityDeclaration> QXmlStreamReader::entityDeclarations();
  fn _ZNK16QXmlStreamReader18entityDeclarationsEv(qthis: *mut c_void) ;
  // proto:  bool QXmlStreamReader::isWhitespace();
  fn _ZNK16QXmlStreamReader12isWhitespaceEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QXmlStreamReader::NewQXmlStreamReader(const QXmlStreamReader & );
  fn _ZN16QXmlStreamReaderC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  long long QXmlStreamReader::columnNumber();
  fn _ZNK16QXmlStreamReader12columnNumberEv(qthis: *mut c_void) -> c_longlong;
  // proto:  bool QXmlStreamReader::hasError();
  fn _ZNK16QXmlStreamReader8hasErrorEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QXmlStreamReader::isCDATA();
  fn _ZNK16QXmlStreamReader7isCDATAEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QXmlStreamReader::FreeQXmlStreamReader();
  fn _ZN16QXmlStreamReaderD0Ev(qthis: *mut c_void) ;
  // proto:  QStringRef QXmlStreamReader::processingInstructionTarget();
  fn _ZNK16QXmlStreamReader27processingInstructionTargetEv(qthis: *mut c_void) ;
  // proto:  void QXmlStreamReader::addData(const char * data);
  fn _ZN16QXmlStreamReader7addDataEPKc(qthis: *mut c_void, arg0: *const c_char) ;
  // proto:  QStringRef QXmlStreamReader::dtdSystemId();
  fn _ZNK16QXmlStreamReader11dtdSystemIdEv(qthis: *mut c_void) ;
  // proto:  QStringRef QXmlStreamReader::prefix();
  fn _ZNK16QXmlStreamReader6prefixEv(qthis: *mut c_void) ;
  // proto:  bool QXmlStreamReader::isEndElement();
  fn _ZNK16QXmlStreamReader12isEndElementEv(qthis: *mut c_void) -> int8_t;
  // proto:  QVector<QXmlStreamNotationDeclaration> QXmlStreamReader::notationDeclarations();
  fn _ZNK16QXmlStreamReader20notationDeclarationsEv(qthis: *mut c_void) ;
  // proto:  QVector<QXmlStreamNamespaceDeclaration> QXmlStreamReader::namespaceDeclarations();
  fn _ZNK16QXmlStreamReader21namespaceDeclarationsEv(qthis: *mut c_void) ;
  // proto:  void QXmlStreamReader::setNamespaceProcessing(bool );
  fn _ZN16QXmlStreamReader22setNamespaceProcessingEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QXmlStreamReader::raiseError(const QString & message);
  fn _ZN16QXmlStreamReader10raiseErrorERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QStringRef QXmlStreamReader::dtdName();
  fn _ZNK16QXmlStreamReader7dtdNameEv(qthis: *mut c_void) ;
  // proto:  bool QXmlStreamReader::isEndDocument();
  fn _ZNK16QXmlStreamReader13isEndDocumentEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QXmlStreamReader::readNextStartElement();
  fn _ZN16QXmlStreamReader20readNextStartElementEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QXmlStreamReader::isComment();
  fn _ZNK16QXmlStreamReader9isCommentEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QXmlStreamReader::NewQXmlStreamReader(const char * data);
  fn _ZN16QXmlStreamReaderC1EPKc(qthis: *mut c_void, arg0: *const c_char) ;
  // proto:  void QXmlStreamReader::skipCurrentElement();
  fn _ZN16QXmlStreamReader18skipCurrentElementEv(qthis: *mut c_void) ;
  // proto:  bool QXmlStreamReader::isEntityReference();
  fn _ZNK16QXmlStreamReader17isEntityReferenceEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QXmlStreamReader::addData(const QByteArray & data);
  fn _ZN16QXmlStreamReader7addDataERK10QByteArray(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QXmlStreamReader::atEnd();
  fn _ZNK16QXmlStreamReader5atEndEv(qthis: *mut c_void) -> int8_t;
}

// body block begin
// class sizeof(QXmlStreamReader)=1
pub struct QXmlStreamReader {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QXmlStreamReader {
  pub fn name<T: QXmlStreamReader_name>(&mut self, value: T)  {
     value.name(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_name {
  fn name(self, rsthis: &mut QXmlStreamReader) ;
}

// proto:  QStringRef QXmlStreamReader::name();
impl<'a> /*trait*/ QXmlStreamReader_name for () {
  fn name(self, rsthis: &mut QXmlStreamReader)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader4nameEv()};
     unsafe {_ZNK16QXmlStreamReader4nameEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QXmlStreamReader {
  pub fn entityResolver<T: QXmlStreamReader_entityResolver>(&mut self, value: T) -> QXmlStreamEntityResolver {
    return value.entityResolver(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_entityResolver {
  fn entityResolver(self, rsthis: &mut QXmlStreamReader) -> QXmlStreamEntityResolver;
}

// proto:  QXmlStreamEntityResolver * QXmlStreamReader::entityResolver();
impl<'a> /*trait*/ QXmlStreamReader_entityResolver for () {
  fn entityResolver(self, rsthis: &mut QXmlStreamReader) -> QXmlStreamEntityResolver {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader14entityResolverEv()};
    let mut ret = unsafe {_ZNK16QXmlStreamReader14entityResolverEv(rsthis.qclsinst)};
    let mut ret1 = QXmlStreamEntityResolver{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QXmlStreamReader {
  pub fn namespaceProcessing<T: QXmlStreamReader_namespaceProcessing>(&mut self, value: T) -> i8 {
    return value.namespaceProcessing(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_namespaceProcessing {
  fn namespaceProcessing(self, rsthis: &mut QXmlStreamReader) -> i8;
}

// proto:  bool QXmlStreamReader::namespaceProcessing();
impl<'a> /*trait*/ QXmlStreamReader_namespaceProcessing for () {
  fn namespaceProcessing(self, rsthis: &mut QXmlStreamReader) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader19namespaceProcessingEv()};
    let mut ret = unsafe {_ZNK16QXmlStreamReader19namespaceProcessingEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QXmlStreamReader {
  pub fn isStartElement<T: QXmlStreamReader_isStartElement>(&mut self, value: T) -> i8 {
    return value.isStartElement(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_isStartElement {
  fn isStartElement(self, rsthis: &mut QXmlStreamReader) -> i8;
}

// proto:  bool QXmlStreamReader::isStartElement();
impl<'a> /*trait*/ QXmlStreamReader_isStartElement for () {
  fn isStartElement(self, rsthis: &mut QXmlStreamReader) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader14isStartElementEv()};
    let mut ret = unsafe {_ZNK16QXmlStreamReader14isStartElementEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QXmlStreamReader {
  pub fn isStandaloneDocument<T: QXmlStreamReader_isStandaloneDocument>(&mut self, value: T) -> i8 {
    return value.isStandaloneDocument(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_isStandaloneDocument {
  fn isStandaloneDocument(self, rsthis: &mut QXmlStreamReader) -> i8;
}

// proto:  bool QXmlStreamReader::isStandaloneDocument();
impl<'a> /*trait*/ QXmlStreamReader_isStandaloneDocument for () {
  fn isStandaloneDocument(self, rsthis: &mut QXmlStreamReader) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader20isStandaloneDocumentEv()};
    let mut ret = unsafe {_ZNK16QXmlStreamReader20isStandaloneDocumentEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QXmlStreamReader {
  pub fn lineNumber<T: QXmlStreamReader_lineNumber>(&mut self, value: T) -> i64 {
    return value.lineNumber(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_lineNumber {
  fn lineNumber(self, rsthis: &mut QXmlStreamReader) -> i64;
}

// proto:  long long QXmlStreamReader::lineNumber();
impl<'a> /*trait*/ QXmlStreamReader_lineNumber for () {
  fn lineNumber(self, rsthis: &mut QXmlStreamReader) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader10lineNumberEv()};
    let mut ret = unsafe {_ZNK16QXmlStreamReader10lineNumberEv(rsthis.qclsinst)};
    return ret as i64;
    // return 1;
  }
}

impl /*struct*/ QXmlStreamReader {
  pub fn clear<T: QXmlStreamReader_clear>(&mut self, value: T)  {
     value.clear(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_clear {
  fn clear(self, rsthis: &mut QXmlStreamReader) ;
}

// proto:  void QXmlStreamReader::clear();
impl<'a> /*trait*/ QXmlStreamReader_clear for () {
  fn clear(self, rsthis: &mut QXmlStreamReader)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamReader5clearEv()};
     unsafe {_ZN16QXmlStreamReader5clearEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QXmlStreamReader {
  pub fn processingInstructionData<T: QXmlStreamReader_processingInstructionData>(&mut self, value: T)  {
     value.processingInstructionData(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_processingInstructionData {
  fn processingInstructionData(self, rsthis: &mut QXmlStreamReader) ;
}

// proto:  QStringRef QXmlStreamReader::processingInstructionData();
impl<'a> /*trait*/ QXmlStreamReader_processingInstructionData for () {
  fn processingInstructionData(self, rsthis: &mut QXmlStreamReader)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader25processingInstructionDataEv()};
     unsafe {_ZNK16QXmlStreamReader25processingInstructionDataEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QXmlStreamReader {
  pub fn addData<T: QXmlStreamReader_addData>(&mut self, value: T)  {
     value.addData(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_addData {
  fn addData(self, rsthis: &mut QXmlStreamReader) ;
}

// proto:  void QXmlStreamReader::addData(const QString & data);
impl<'a> /*trait*/ QXmlStreamReader_addData for (&'a  QString) {
  fn addData(self, rsthis: &mut QXmlStreamReader)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamReader7addDataERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QXmlStreamReader7addDataERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QXmlStreamReader {
  pub fn dtdPublicId<T: QXmlStreamReader_dtdPublicId>(&mut self, value: T)  {
     value.dtdPublicId(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_dtdPublicId {
  fn dtdPublicId(self, rsthis: &mut QXmlStreamReader) ;
}

// proto:  QStringRef QXmlStreamReader::dtdPublicId();
impl<'a> /*trait*/ QXmlStreamReader_dtdPublicId for () {
  fn dtdPublicId(self, rsthis: &mut QXmlStreamReader)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader11dtdPublicIdEv()};
     unsafe {_ZNK16QXmlStreamReader11dtdPublicIdEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QXmlStreamReader {
  pub fn documentEncoding<T: QXmlStreamReader_documentEncoding>(&mut self, value: T)  {
     value.documentEncoding(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_documentEncoding {
  fn documentEncoding(self, rsthis: &mut QXmlStreamReader) ;
}

// proto:  QStringRef QXmlStreamReader::documentEncoding();
impl<'a> /*trait*/ QXmlStreamReader_documentEncoding for () {
  fn documentEncoding(self, rsthis: &mut QXmlStreamReader)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader16documentEncodingEv()};
     unsafe {_ZNK16QXmlStreamReader16documentEncodingEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QXmlStreamReader {
  pub fn characterOffset<T: QXmlStreamReader_characterOffset>(&mut self, value: T) -> i64 {
    return value.characterOffset(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_characterOffset {
  fn characterOffset(self, rsthis: &mut QXmlStreamReader) -> i64;
}

// proto:  long long QXmlStreamReader::characterOffset();
impl<'a> /*trait*/ QXmlStreamReader_characterOffset for () {
  fn characterOffset(self, rsthis: &mut QXmlStreamReader) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader15characterOffsetEv()};
    let mut ret = unsafe {_ZNK16QXmlStreamReader15characterOffsetEv(rsthis.qclsinst)};
    return ret as i64;
    // return 1;
  }
}

impl /*struct*/ QXmlStreamReader {
  pub fn attributes<T: QXmlStreamReader_attributes>(&mut self, value: T) -> QXmlStreamAttributes {
    return value.attributes(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_attributes {
  fn attributes(self, rsthis: &mut QXmlStreamReader) -> QXmlStreamAttributes;
}

// proto:  QXmlStreamAttributes QXmlStreamReader::attributes();
impl<'a> /*trait*/ QXmlStreamReader_attributes for () {
  fn attributes(self, rsthis: &mut QXmlStreamReader) -> QXmlStreamAttributes {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader10attributesEv()};
    let mut ret = unsafe {_ZNK16QXmlStreamReader10attributesEv(rsthis.qclsinst)};
    let mut ret1 = QXmlStreamAttributes{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QXmlStreamReader {
  pub fn tokenString<T: QXmlStreamReader_tokenString>(&mut self, value: T) -> QString {
    return value.tokenString(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_tokenString {
  fn tokenString(self, rsthis: &mut QXmlStreamReader) -> QString;
}

// proto:  QString QXmlStreamReader::tokenString();
impl<'a> /*trait*/ QXmlStreamReader_tokenString for () {
  fn tokenString(self, rsthis: &mut QXmlStreamReader) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader11tokenStringEv()};
    let mut ret = unsafe {_ZNK16QXmlStreamReader11tokenStringEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QXmlStreamReader {
  pub fn addExtraNamespaceDeclaration<T: QXmlStreamReader_addExtraNamespaceDeclaration>(&mut self, value: T)  {
     value.addExtraNamespaceDeclaration(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_addExtraNamespaceDeclaration {
  fn addExtraNamespaceDeclaration(self, rsthis: &mut QXmlStreamReader) ;
}

// proto:  void QXmlStreamReader::addExtraNamespaceDeclaration(const QXmlStreamNamespaceDeclaration & extraNamespaceDeclaraction);
impl<'a> /*trait*/ QXmlStreamReader_addExtraNamespaceDeclaration for (&'a  QXmlStreamNamespaceDeclaration) {
  fn addExtraNamespaceDeclaration(self, rsthis: &mut QXmlStreamReader)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamReader28addExtraNamespaceDeclarationERK30QXmlStreamNamespaceDeclaration()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QXmlStreamReader28addExtraNamespaceDeclarationERK30QXmlStreamNamespaceDeclaration(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

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

// proto: void QXmlStreamReader::NewQXmlStreamReader(const QByteArray & data);
impl<'a> /*trait*/ QXmlStreamReader_NewQXmlStreamReader for (&'a  QByteArray) {
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

impl /*struct*/ QXmlStreamReader {
  pub fn qualifiedName<T: QXmlStreamReader_qualifiedName>(&mut self, value: T)  {
     value.qualifiedName(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_qualifiedName {
  fn qualifiedName(self, rsthis: &mut QXmlStreamReader) ;
}

// proto:  QStringRef QXmlStreamReader::qualifiedName();
impl<'a> /*trait*/ QXmlStreamReader_qualifiedName for () {
  fn qualifiedName(self, rsthis: &mut QXmlStreamReader)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader13qualifiedNameEv()};
     unsafe {_ZNK16QXmlStreamReader13qualifiedNameEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QXmlStreamReader {
  pub fn device<T: QXmlStreamReader_device>(&mut self, value: T) -> QIODevice {
    return value.device(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_device {
  fn device(self, rsthis: &mut QXmlStreamReader) -> QIODevice;
}

// proto:  QIODevice * QXmlStreamReader::device();
impl<'a> /*trait*/ QXmlStreamReader_device for () {
  fn device(self, rsthis: &mut QXmlStreamReader) -> QIODevice {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader6deviceEv()};
    let mut ret = unsafe {_ZNK16QXmlStreamReader6deviceEv(rsthis.qclsinst)};
    let mut ret1 = QIODevice{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QXmlStreamReader {
  pub fn namespaceUri<T: QXmlStreamReader_namespaceUri>(&mut self, value: T)  {
     value.namespaceUri(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_namespaceUri {
  fn namespaceUri(self, rsthis: &mut QXmlStreamReader) ;
}

// proto:  QStringRef QXmlStreamReader::namespaceUri();
impl<'a> /*trait*/ QXmlStreamReader_namespaceUri for () {
  fn namespaceUri(self, rsthis: &mut QXmlStreamReader)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader12namespaceUriEv()};
     unsafe {_ZNK16QXmlStreamReader12namespaceUriEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QXmlStreamReader {
  pub fn text<T: QXmlStreamReader_text>(&mut self, value: T)  {
     value.text(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_text {
  fn text(self, rsthis: &mut QXmlStreamReader) ;
}

// proto:  QStringRef QXmlStreamReader::text();
impl<'a> /*trait*/ QXmlStreamReader_text for () {
  fn text(self, rsthis: &mut QXmlStreamReader)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader4textEv()};
     unsafe {_ZNK16QXmlStreamReader4textEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QXmlStreamReader {
  pub fn setDevice<T: QXmlStreamReader_setDevice>(&mut self, value: T)  {
     value.setDevice(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_setDevice {
  fn setDevice(self, rsthis: &mut QXmlStreamReader) ;
}

// proto:  void QXmlStreamReader::setDevice(QIODevice * device);
impl<'a> /*trait*/ QXmlStreamReader_setDevice for (&'a mut QIODevice) {
  fn setDevice(self, rsthis: &mut QXmlStreamReader)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamReader9setDeviceEP9QIODevice()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QXmlStreamReader9setDeviceEP9QIODevice(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto: void QXmlStreamReader::NewQXmlStreamReader(QIODevice * device);
impl<'a> /*trait*/ QXmlStreamReader_NewQXmlStreamReader for (&'a mut QIODevice) {
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

impl /*struct*/ QXmlStreamReader {
  pub fn documentVersion<T: QXmlStreamReader_documentVersion>(&mut self, value: T)  {
     value.documentVersion(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_documentVersion {
  fn documentVersion(self, rsthis: &mut QXmlStreamReader) ;
}

// proto:  QStringRef QXmlStreamReader::documentVersion();
impl<'a> /*trait*/ QXmlStreamReader_documentVersion for () {
  fn documentVersion(self, rsthis: &mut QXmlStreamReader)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader15documentVersionEv()};
     unsafe {_ZNK16QXmlStreamReader15documentVersionEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QXmlStreamReader {
  pub fn isDTD<T: QXmlStreamReader_isDTD>(&mut self, value: T) -> i8 {
    return value.isDTD(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_isDTD {
  fn isDTD(self, rsthis: &mut QXmlStreamReader) -> i8;
}

// proto:  bool QXmlStreamReader::isDTD();
impl<'a> /*trait*/ QXmlStreamReader_isDTD for () {
  fn isDTD(self, rsthis: &mut QXmlStreamReader) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader5isDTDEv()};
    let mut ret = unsafe {_ZNK16QXmlStreamReader5isDTDEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QXmlStreamReader {
  pub fn isStartDocument<T: QXmlStreamReader_isStartDocument>(&mut self, value: T) -> i8 {
    return value.isStartDocument(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_isStartDocument {
  fn isStartDocument(self, rsthis: &mut QXmlStreamReader) -> i8;
}

// proto:  bool QXmlStreamReader::isStartDocument();
impl<'a> /*trait*/ QXmlStreamReader_isStartDocument for () {
  fn isStartDocument(self, rsthis: &mut QXmlStreamReader) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader15isStartDocumentEv()};
    let mut ret = unsafe {_ZNK16QXmlStreamReader15isStartDocumentEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QXmlStreamReader {
  pub fn errorString<T: QXmlStreamReader_errorString>(&mut self, value: T) -> QString {
    return value.errorString(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_errorString {
  fn errorString(self, rsthis: &mut QXmlStreamReader) -> QString;
}

// proto:  QString QXmlStreamReader::errorString();
impl<'a> /*trait*/ QXmlStreamReader_errorString for () {
  fn errorString(self, rsthis: &mut QXmlStreamReader) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader11errorStringEv()};
    let mut ret = unsafe {_ZNK16QXmlStreamReader11errorStringEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QXmlStreamReader {
  pub fn isProcessingInstruction<T: QXmlStreamReader_isProcessingInstruction>(&mut self, value: T) -> i8 {
    return value.isProcessingInstruction(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_isProcessingInstruction {
  fn isProcessingInstruction(self, rsthis: &mut QXmlStreamReader) -> i8;
}

// proto:  bool QXmlStreamReader::isProcessingInstruction();
impl<'a> /*trait*/ QXmlStreamReader_isProcessingInstruction for () {
  fn isProcessingInstruction(self, rsthis: &mut QXmlStreamReader) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader23isProcessingInstructionEv()};
    let mut ret = unsafe {_ZNK16QXmlStreamReader23isProcessingInstructionEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QXmlStreamReader {
  pub fn setEntityResolver<T: QXmlStreamReader_setEntityResolver>(&mut self, value: T)  {
     value.setEntityResolver(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_setEntityResolver {
  fn setEntityResolver(self, rsthis: &mut QXmlStreamReader) ;
}

// proto:  void QXmlStreamReader::setEntityResolver(QXmlStreamEntityResolver * resolver);
impl<'a> /*trait*/ QXmlStreamReader_setEntityResolver for (&'a mut QXmlStreamEntityResolver) {
  fn setEntityResolver(self, rsthis: &mut QXmlStreamReader)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamReader17setEntityResolverEP24QXmlStreamEntityResolver()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QXmlStreamReader17setEntityResolverEP24QXmlStreamEntityResolver(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QXmlStreamReader {
  pub fn isCharacters<T: QXmlStreamReader_isCharacters>(&mut self, value: T) -> i8 {
    return value.isCharacters(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_isCharacters {
  fn isCharacters(self, rsthis: &mut QXmlStreamReader) -> i8;
}

// proto:  bool QXmlStreamReader::isCharacters();
impl<'a> /*trait*/ QXmlStreamReader_isCharacters for () {
  fn isCharacters(self, rsthis: &mut QXmlStreamReader) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader12isCharactersEv()};
    let mut ret = unsafe {_ZNK16QXmlStreamReader12isCharactersEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto: void QXmlStreamReader::NewQXmlStreamReader();
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

// proto: void QXmlStreamReader::NewQXmlStreamReader(const QString & data);
impl<'a> /*trait*/ QXmlStreamReader_NewQXmlStreamReader for (&'a  QString) {
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

impl /*struct*/ QXmlStreamReader {
  pub fn entityDeclarations<T: QXmlStreamReader_entityDeclarations>(&mut self, value: T)  {
     value.entityDeclarations(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_entityDeclarations {
  fn entityDeclarations(self, rsthis: &mut QXmlStreamReader) ;
}

// proto:  QVector<QXmlStreamEntityDeclaration> QXmlStreamReader::entityDeclarations();
impl<'a> /*trait*/ QXmlStreamReader_entityDeclarations for () {
  fn entityDeclarations(self, rsthis: &mut QXmlStreamReader)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader18entityDeclarationsEv()};
     unsafe {_ZNK16QXmlStreamReader18entityDeclarationsEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QXmlStreamReader {
  pub fn isWhitespace<T: QXmlStreamReader_isWhitespace>(&mut self, value: T) -> i8 {
    return value.isWhitespace(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_isWhitespace {
  fn isWhitespace(self, rsthis: &mut QXmlStreamReader) -> i8;
}

// proto:  bool QXmlStreamReader::isWhitespace();
impl<'a> /*trait*/ QXmlStreamReader_isWhitespace for () {
  fn isWhitespace(self, rsthis: &mut QXmlStreamReader) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader12isWhitespaceEv()};
    let mut ret = unsafe {_ZNK16QXmlStreamReader12isWhitespaceEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto: void QXmlStreamReader::NewQXmlStreamReader(const QXmlStreamReader & );
impl<'a> /*trait*/ QXmlStreamReader_NewQXmlStreamReader for (&'a  QXmlStreamReader) {
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

impl /*struct*/ QXmlStreamReader {
  pub fn columnNumber<T: QXmlStreamReader_columnNumber>(&mut self, value: T) -> i64 {
    return value.columnNumber(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_columnNumber {
  fn columnNumber(self, rsthis: &mut QXmlStreamReader) -> i64;
}

// proto:  long long QXmlStreamReader::columnNumber();
impl<'a> /*trait*/ QXmlStreamReader_columnNumber for () {
  fn columnNumber(self, rsthis: &mut QXmlStreamReader) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader12columnNumberEv()};
    let mut ret = unsafe {_ZNK16QXmlStreamReader12columnNumberEv(rsthis.qclsinst)};
    return ret as i64;
    // return 1;
  }
}

impl /*struct*/ QXmlStreamReader {
  pub fn hasError<T: QXmlStreamReader_hasError>(&mut self, value: T) -> i8 {
    return value.hasError(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_hasError {
  fn hasError(self, rsthis: &mut QXmlStreamReader) -> i8;
}

// proto:  bool QXmlStreamReader::hasError();
impl<'a> /*trait*/ QXmlStreamReader_hasError for () {
  fn hasError(self, rsthis: &mut QXmlStreamReader) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader8hasErrorEv()};
    let mut ret = unsafe {_ZNK16QXmlStreamReader8hasErrorEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QXmlStreamReader {
  pub fn isCDATA<T: QXmlStreamReader_isCDATA>(&mut self, value: T) -> i8 {
    return value.isCDATA(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_isCDATA {
  fn isCDATA(self, rsthis: &mut QXmlStreamReader) -> i8;
}

// proto:  bool QXmlStreamReader::isCDATA();
impl<'a> /*trait*/ QXmlStreamReader_isCDATA for () {
  fn isCDATA(self, rsthis: &mut QXmlStreamReader) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader7isCDATAEv()};
    let mut ret = unsafe {_ZNK16QXmlStreamReader7isCDATAEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QXmlStreamReader {
  pub fn FreeQXmlStreamReader<T: QXmlStreamReader_FreeQXmlStreamReader>(&mut self, value: T)  {
     value.FreeQXmlStreamReader(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_FreeQXmlStreamReader {
  fn FreeQXmlStreamReader(self, rsthis: &mut QXmlStreamReader) ;
}

// proto:  void QXmlStreamReader::FreeQXmlStreamReader();
impl<'a> /*trait*/ QXmlStreamReader_FreeQXmlStreamReader for () {
  fn FreeQXmlStreamReader(self, rsthis: &mut QXmlStreamReader)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamReaderD0Ev()};
     unsafe {_ZN16QXmlStreamReaderD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QXmlStreamReader {
  pub fn processingInstructionTarget<T: QXmlStreamReader_processingInstructionTarget>(&mut self, value: T)  {
     value.processingInstructionTarget(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_processingInstructionTarget {
  fn processingInstructionTarget(self, rsthis: &mut QXmlStreamReader) ;
}

// proto:  QStringRef QXmlStreamReader::processingInstructionTarget();
impl<'a> /*trait*/ QXmlStreamReader_processingInstructionTarget for () {
  fn processingInstructionTarget(self, rsthis: &mut QXmlStreamReader)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader27processingInstructionTargetEv()};
     unsafe {_ZNK16QXmlStreamReader27processingInstructionTargetEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  void QXmlStreamReader::addData(const char * data);
impl<'a> /*trait*/ QXmlStreamReader_addData for (&'a  String) {
  fn addData(self, rsthis: &mut QXmlStreamReader)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamReader7addDataEPKc()};
    let arg0 = self.as_ptr()  as *const c_char;
     unsafe {_ZN16QXmlStreamReader7addDataEPKc(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QXmlStreamReader {
  pub fn dtdSystemId<T: QXmlStreamReader_dtdSystemId>(&mut self, value: T)  {
     value.dtdSystemId(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_dtdSystemId {
  fn dtdSystemId(self, rsthis: &mut QXmlStreamReader) ;
}

// proto:  QStringRef QXmlStreamReader::dtdSystemId();
impl<'a> /*trait*/ QXmlStreamReader_dtdSystemId for () {
  fn dtdSystemId(self, rsthis: &mut QXmlStreamReader)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader11dtdSystemIdEv()};
     unsafe {_ZNK16QXmlStreamReader11dtdSystemIdEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QXmlStreamReader {
  pub fn prefix<T: QXmlStreamReader_prefix>(&mut self, value: T)  {
     value.prefix(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_prefix {
  fn prefix(self, rsthis: &mut QXmlStreamReader) ;
}

// proto:  QStringRef QXmlStreamReader::prefix();
impl<'a> /*trait*/ QXmlStreamReader_prefix for () {
  fn prefix(self, rsthis: &mut QXmlStreamReader)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader6prefixEv()};
     unsafe {_ZNK16QXmlStreamReader6prefixEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QXmlStreamReader {
  pub fn isEndElement<T: QXmlStreamReader_isEndElement>(&mut self, value: T) -> i8 {
    return value.isEndElement(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_isEndElement {
  fn isEndElement(self, rsthis: &mut QXmlStreamReader) -> i8;
}

// proto:  bool QXmlStreamReader::isEndElement();
impl<'a> /*trait*/ QXmlStreamReader_isEndElement for () {
  fn isEndElement(self, rsthis: &mut QXmlStreamReader) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader12isEndElementEv()};
    let mut ret = unsafe {_ZNK16QXmlStreamReader12isEndElementEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QXmlStreamReader {
  pub fn notationDeclarations<T: QXmlStreamReader_notationDeclarations>(&mut self, value: T)  {
     value.notationDeclarations(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_notationDeclarations {
  fn notationDeclarations(self, rsthis: &mut QXmlStreamReader) ;
}

// proto:  QVector<QXmlStreamNotationDeclaration> QXmlStreamReader::notationDeclarations();
impl<'a> /*trait*/ QXmlStreamReader_notationDeclarations for () {
  fn notationDeclarations(self, rsthis: &mut QXmlStreamReader)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader20notationDeclarationsEv()};
     unsafe {_ZNK16QXmlStreamReader20notationDeclarationsEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QXmlStreamReader {
  pub fn namespaceDeclarations<T: QXmlStreamReader_namespaceDeclarations>(&mut self, value: T)  {
     value.namespaceDeclarations(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_namespaceDeclarations {
  fn namespaceDeclarations(self, rsthis: &mut QXmlStreamReader) ;
}

// proto:  QVector<QXmlStreamNamespaceDeclaration> QXmlStreamReader::namespaceDeclarations();
impl<'a> /*trait*/ QXmlStreamReader_namespaceDeclarations for () {
  fn namespaceDeclarations(self, rsthis: &mut QXmlStreamReader)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader21namespaceDeclarationsEv()};
     unsafe {_ZNK16QXmlStreamReader21namespaceDeclarationsEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QXmlStreamReader {
  pub fn setNamespaceProcessing<T: QXmlStreamReader_setNamespaceProcessing>(&mut self, value: T)  {
     value.setNamespaceProcessing(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_setNamespaceProcessing {
  fn setNamespaceProcessing(self, rsthis: &mut QXmlStreamReader) ;
}

// proto:  void QXmlStreamReader::setNamespaceProcessing(bool );
impl<'a> /*trait*/ QXmlStreamReader_setNamespaceProcessing for (i8) {
  fn setNamespaceProcessing(self, rsthis: &mut QXmlStreamReader)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamReader22setNamespaceProcessingEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN16QXmlStreamReader22setNamespaceProcessingEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QXmlStreamReader {
  pub fn raiseError<T: QXmlStreamReader_raiseError>(&mut self, value: T)  {
     value.raiseError(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_raiseError {
  fn raiseError(self, rsthis: &mut QXmlStreamReader) ;
}

// proto:  void QXmlStreamReader::raiseError(const QString & message);
impl<'a> /*trait*/ QXmlStreamReader_raiseError for (&'a  QString) {
  fn raiseError(self, rsthis: &mut QXmlStreamReader)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamReader10raiseErrorERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QXmlStreamReader10raiseErrorERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QXmlStreamReader {
  pub fn dtdName<T: QXmlStreamReader_dtdName>(&mut self, value: T)  {
     value.dtdName(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_dtdName {
  fn dtdName(self, rsthis: &mut QXmlStreamReader) ;
}

// proto:  QStringRef QXmlStreamReader::dtdName();
impl<'a> /*trait*/ QXmlStreamReader_dtdName for () {
  fn dtdName(self, rsthis: &mut QXmlStreamReader)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader7dtdNameEv()};
     unsafe {_ZNK16QXmlStreamReader7dtdNameEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QXmlStreamReader {
  pub fn isEndDocument<T: QXmlStreamReader_isEndDocument>(&mut self, value: T) -> i8 {
    return value.isEndDocument(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_isEndDocument {
  fn isEndDocument(self, rsthis: &mut QXmlStreamReader) -> i8;
}

// proto:  bool QXmlStreamReader::isEndDocument();
impl<'a> /*trait*/ QXmlStreamReader_isEndDocument for () {
  fn isEndDocument(self, rsthis: &mut QXmlStreamReader) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader13isEndDocumentEv()};
    let mut ret = unsafe {_ZNK16QXmlStreamReader13isEndDocumentEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QXmlStreamReader {
  pub fn readNextStartElement<T: QXmlStreamReader_readNextStartElement>(&mut self, value: T) -> i8 {
    return value.readNextStartElement(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_readNextStartElement {
  fn readNextStartElement(self, rsthis: &mut QXmlStreamReader) -> i8;
}

// proto:  bool QXmlStreamReader::readNextStartElement();
impl<'a> /*trait*/ QXmlStreamReader_readNextStartElement for () {
  fn readNextStartElement(self, rsthis: &mut QXmlStreamReader) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamReader20readNextStartElementEv()};
    let mut ret = unsafe {_ZN16QXmlStreamReader20readNextStartElementEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QXmlStreamReader {
  pub fn isComment<T: QXmlStreamReader_isComment>(&mut self, value: T) -> i8 {
    return value.isComment(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_isComment {
  fn isComment(self, rsthis: &mut QXmlStreamReader) -> i8;
}

// proto:  bool QXmlStreamReader::isComment();
impl<'a> /*trait*/ QXmlStreamReader_isComment for () {
  fn isComment(self, rsthis: &mut QXmlStreamReader) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader9isCommentEv()};
    let mut ret = unsafe {_ZNK16QXmlStreamReader9isCommentEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto: void QXmlStreamReader::NewQXmlStreamReader(const char * data);
impl<'a> /*trait*/ QXmlStreamReader_NewQXmlStreamReader for (&'a  String) {
  fn NewQXmlStreamReader(self) -> QXmlStreamReader {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamReaderC1EPKc()};
    let arg0 = self.as_ptr()  as *const c_char;
    unsafe {_ZN16QXmlStreamReaderC1EPKc(qthis, arg0)};
    let rsthis = QXmlStreamReader{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QXmlStreamReader {
  pub fn skipCurrentElement<T: QXmlStreamReader_skipCurrentElement>(&mut self, value: T)  {
     value.skipCurrentElement(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_skipCurrentElement {
  fn skipCurrentElement(self, rsthis: &mut QXmlStreamReader) ;
}

// proto:  void QXmlStreamReader::skipCurrentElement();
impl<'a> /*trait*/ QXmlStreamReader_skipCurrentElement for () {
  fn skipCurrentElement(self, rsthis: &mut QXmlStreamReader)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamReader18skipCurrentElementEv()};
     unsafe {_ZN16QXmlStreamReader18skipCurrentElementEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QXmlStreamReader {
  pub fn isEntityReference<T: QXmlStreamReader_isEntityReference>(&mut self, value: T) -> i8 {
    return value.isEntityReference(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_isEntityReference {
  fn isEntityReference(self, rsthis: &mut QXmlStreamReader) -> i8;
}

// proto:  bool QXmlStreamReader::isEntityReference();
impl<'a> /*trait*/ QXmlStreamReader_isEntityReference for () {
  fn isEntityReference(self, rsthis: &mut QXmlStreamReader) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader17isEntityReferenceEv()};
    let mut ret = unsafe {_ZNK16QXmlStreamReader17isEntityReferenceEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto:  void QXmlStreamReader::addData(const QByteArray & data);
impl<'a> /*trait*/ QXmlStreamReader_addData for (&'a  QByteArray) {
  fn addData(self, rsthis: &mut QXmlStreamReader)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamReader7addDataERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QXmlStreamReader7addDataERK10QByteArray(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QXmlStreamReader {
  pub fn atEnd<T: QXmlStreamReader_atEnd>(&mut self, value: T) -> i8 {
    return value.atEnd(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_atEnd {
  fn atEnd(self, rsthis: &mut QXmlStreamReader) -> i8;
}

// proto:  bool QXmlStreamReader::atEnd();
impl<'a> /*trait*/ QXmlStreamReader_atEnd for () {
  fn atEnd(self, rsthis: &mut QXmlStreamReader) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader5atEndEv()};
    let mut ret = unsafe {_ZNK16QXmlStreamReader5atEndEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

