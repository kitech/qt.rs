// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstring::QString;
use super::qxmlstreamnamespacedeclaration::QXmlStreamNamespaceDeclaration;
use super::qbytearray::QByteArray;
use super::qiodevice::QIODevice;
use super::qxmlstreamentityresolver::QXmlStreamEntityResolver;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  fn _ZNK16QXmlStreamReader4nameEv() -> i32;
  fn _ZNK16QXmlStreamReader14entityResolverEv() -> i32;
  fn _ZNK16QXmlStreamReader19namespaceProcessingEv() -> i32;
  fn _ZNK16QXmlStreamReader14isStartElementEv() -> i32;
  fn _ZNK16QXmlStreamReader20isStandaloneDocumentEv() -> i32;
  fn _ZNK16QXmlStreamReader10lineNumberEv() -> i32;
  fn _ZN16QXmlStreamReader5clearEv() -> i32;
  fn _ZNK16QXmlStreamReader25processingInstructionDataEv() -> i32;
  fn _ZN16QXmlStreamReader7addDataERK7QString(arg0: *const c_void) -> i32;
  fn _ZNK16QXmlStreamReader11dtdPublicIdEv() -> i32;
  fn _ZNK16QXmlStreamReader16documentEncodingEv() -> i32;
  fn _ZNK16QXmlStreamReader15characterOffsetEv() -> i32;
  fn _ZNK16QXmlStreamReader10attributesEv() -> i32;
  fn _ZNK16QXmlStreamReader11tokenStringEv() -> i32;
  fn _ZN16QXmlStreamReader28addExtraNamespaceDeclarationERK30QXmlStreamNamespaceDeclaration(arg0: *const c_void) -> i32;
  fn _ZN16QXmlStreamReaderC1ERK10QByteArray(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZNK16QXmlStreamReader13qualifiedNameEv() -> i32;
  fn _ZNK16QXmlStreamReader6deviceEv() -> i32;
  fn _ZNK16QXmlStreamReader12namespaceUriEv() -> i32;
  fn _ZNK16QXmlStreamReader4textEv() -> i32;
  fn _ZN16QXmlStreamReader9setDeviceEP9QIODevice(arg0: *mut c_void) -> i32;
  fn _ZN16QXmlStreamReaderC1EP9QIODevice(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  fn _ZNK16QXmlStreamReader15documentVersionEv() -> i32;
  fn _ZNK16QXmlStreamReader5isDTDEv() -> i32;
  fn _ZNK16QXmlStreamReader15isStartDocumentEv() -> i32;
  fn _ZNK16QXmlStreamReader11errorStringEv() -> i32;
  fn _ZNK16QXmlStreamReader23isProcessingInstructionEv() -> i32;
  fn _ZN16QXmlStreamReader17setEntityResolverEP24QXmlStreamEntityResolver(arg0: *mut c_void) -> i32;
  fn _ZNK16QXmlStreamReader12isCharactersEv() -> i32;
  fn _ZN16QXmlStreamReaderC1Ev(qthis: *mut c_void) -> i32;
  fn _ZN16QXmlStreamReaderC1ERK7QString(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZNK16QXmlStreamReader18entityDeclarationsEv() -> i32;
  fn _ZNK16QXmlStreamReader12isWhitespaceEv() -> i32;
  fn _ZN16QXmlStreamReaderC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZNK16QXmlStreamReader12columnNumberEv() -> i32;
  fn _ZNK16QXmlStreamReader8hasErrorEv() -> i32;
  fn _ZNK16QXmlStreamReader7isCDATAEv() -> i32;
  fn _ZN16QXmlStreamReaderD0Ev() -> i32;
  fn _ZNK16QXmlStreamReader27processingInstructionTargetEv() -> i32;
  fn _ZN16QXmlStreamReader7addDataEPKc(arg0: *const c_char) -> i32;
  fn _ZNK16QXmlStreamReader11dtdSystemIdEv() -> i32;
  fn _ZNK16QXmlStreamReader6prefixEv() -> i32;
  fn _ZNK16QXmlStreamReader12isEndElementEv() -> i32;
  fn _ZNK16QXmlStreamReader20notationDeclarationsEv() -> i32;
  fn _ZNK16QXmlStreamReader21namespaceDeclarationsEv() -> i32;
  fn _ZN16QXmlStreamReader22setNamespaceProcessingEb(arg0: int8_t) -> i32;
  fn _ZN16QXmlStreamReader10raiseErrorERK7QString(arg0: *const c_void) -> i32;
  fn _ZNK16QXmlStreamReader7dtdNameEv() -> i32;
  fn _ZNK16QXmlStreamReader13isEndDocumentEv() -> i32;
  fn _ZN16QXmlStreamReader20readNextStartElementEv() -> i32;
  fn _ZNK16QXmlStreamReader9isCommentEv() -> i32;
  fn _ZN16QXmlStreamReaderC1EPKc(qthis: *mut c_void, arg0: *const c_char) -> i32;
  fn _ZN16QXmlStreamReader18skipCurrentElementEv() -> i32;
  fn _ZNK16QXmlStreamReader17isEntityReferenceEv() -> i32;
  fn _ZN16QXmlStreamReader7addDataERK10QByteArray(arg0: *const c_void) -> i32;
  fn _ZNK16QXmlStreamReader5atEndEv() -> i32;
}

// body block begin
// class sizeof(QXmlStreamReader)=1
pub struct QXmlStreamReader {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QXmlStreamReader {
  pub fn name<T: QXmlStreamReader_name>(&mut self, value: T) -> i32 {
    value.name(self);
    return 1;
  }
}

pub trait QXmlStreamReader_name {
  fn name(self, this: &mut QXmlStreamReader) -> i32;
}

// proto: QStringRef QXmlStreamReader::name();
impl<'a> /*trait*/ QXmlStreamReader_name for () {
  fn name(self, this: &mut QXmlStreamReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader4nameEv()};
    unsafe {_ZNK16QXmlStreamReader4nameEv()};
    return 1;
  }
}

impl /*struct*/ QXmlStreamReader {
  pub fn entityResolver<T: QXmlStreamReader_entityResolver>(&mut self, value: T) -> i32 {
    value.entityResolver(self);
    return 1;
  }
}

pub trait QXmlStreamReader_entityResolver {
  fn entityResolver(self, this: &mut QXmlStreamReader) -> i32;
}

// proto: QXmlStreamEntityResolver * QXmlStreamReader::entityResolver();
impl<'a> /*trait*/ QXmlStreamReader_entityResolver for () {
  fn entityResolver(self, this: &mut QXmlStreamReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader14entityResolverEv()};
    unsafe {_ZNK16QXmlStreamReader14entityResolverEv()};
    return 1;
  }
}

impl /*struct*/ QXmlStreamReader {
  pub fn namespaceProcessing<T: QXmlStreamReader_namespaceProcessing>(&mut self, value: T) -> i32 {
    value.namespaceProcessing(self);
    return 1;
  }
}

pub trait QXmlStreamReader_namespaceProcessing {
  fn namespaceProcessing(self, this: &mut QXmlStreamReader) -> i32;
}

// proto: bool QXmlStreamReader::namespaceProcessing();
impl<'a> /*trait*/ QXmlStreamReader_namespaceProcessing for () {
  fn namespaceProcessing(self, this: &mut QXmlStreamReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader19namespaceProcessingEv()};
    unsafe {_ZNK16QXmlStreamReader19namespaceProcessingEv()};
    return 1;
  }
}

impl /*struct*/ QXmlStreamReader {
  pub fn isStartElement<T: QXmlStreamReader_isStartElement>(&mut self, value: T) -> i32 {
    value.isStartElement(self);
    return 1;
  }
}

pub trait QXmlStreamReader_isStartElement {
  fn isStartElement(self, this: &mut QXmlStreamReader) -> i32;
}

// proto: bool QXmlStreamReader::isStartElement();
impl<'a> /*trait*/ QXmlStreamReader_isStartElement for () {
  fn isStartElement(self, this: &mut QXmlStreamReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader14isStartElementEv()};
    unsafe {_ZNK16QXmlStreamReader14isStartElementEv()};
    return 1;
  }
}

impl /*struct*/ QXmlStreamReader {
  pub fn isStandaloneDocument<T: QXmlStreamReader_isStandaloneDocument>(&mut self, value: T) -> i32 {
    value.isStandaloneDocument(self);
    return 1;
  }
}

pub trait QXmlStreamReader_isStandaloneDocument {
  fn isStandaloneDocument(self, this: &mut QXmlStreamReader) -> i32;
}

// proto: bool QXmlStreamReader::isStandaloneDocument();
impl<'a> /*trait*/ QXmlStreamReader_isStandaloneDocument for () {
  fn isStandaloneDocument(self, this: &mut QXmlStreamReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader20isStandaloneDocumentEv()};
    unsafe {_ZNK16QXmlStreamReader20isStandaloneDocumentEv()};
    return 1;
  }
}

impl /*struct*/ QXmlStreamReader {
  pub fn lineNumber<T: QXmlStreamReader_lineNumber>(&mut self, value: T) -> i32 {
    value.lineNumber(self);
    return 1;
  }
}

pub trait QXmlStreamReader_lineNumber {
  fn lineNumber(self, this: &mut QXmlStreamReader) -> i32;
}

// proto: long long QXmlStreamReader::lineNumber();
impl<'a> /*trait*/ QXmlStreamReader_lineNumber for () {
  fn lineNumber(self, this: &mut QXmlStreamReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader10lineNumberEv()};
    unsafe {_ZNK16QXmlStreamReader10lineNumberEv()};
    return 1;
  }
}

impl /*struct*/ QXmlStreamReader {
  pub fn clear<T: QXmlStreamReader_clear>(&mut self, value: T) -> i32 {
    value.clear(self);
    return 1;
  }
}

pub trait QXmlStreamReader_clear {
  fn clear(self, this: &mut QXmlStreamReader) -> i32;
}

// proto: void QXmlStreamReader::clear();
impl<'a> /*trait*/ QXmlStreamReader_clear for () {
  fn clear(self, this: &mut QXmlStreamReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamReader5clearEv()};
    unsafe {_ZN16QXmlStreamReader5clearEv()};
    return 1;
  }
}

impl /*struct*/ QXmlStreamReader {
  pub fn processingInstructionData<T: QXmlStreamReader_processingInstructionData>(&mut self, value: T) -> i32 {
    value.processingInstructionData(self);
    return 1;
  }
}

pub trait QXmlStreamReader_processingInstructionData {
  fn processingInstructionData(self, this: &mut QXmlStreamReader) -> i32;
}

// proto: QStringRef QXmlStreamReader::processingInstructionData();
impl<'a> /*trait*/ QXmlStreamReader_processingInstructionData for () {
  fn processingInstructionData(self, this: &mut QXmlStreamReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader25processingInstructionDataEv()};
    unsafe {_ZNK16QXmlStreamReader25processingInstructionDataEv()};
    return 1;
  }
}

impl /*struct*/ QXmlStreamReader {
  pub fn addData<T: QXmlStreamReader_addData>(&mut self, value: T) -> i32 {
    value.addData(self);
    return 1;
  }
}

pub trait QXmlStreamReader_addData {
  fn addData(self, this: &mut QXmlStreamReader) -> i32;
}

// proto: void QXmlStreamReader::addData(const QString & data);
impl<'a> /*trait*/ QXmlStreamReader_addData for (&'a  QString) {
  fn addData(self, this: &mut QXmlStreamReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamReader7addDataERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN16QXmlStreamReader7addDataERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QXmlStreamReader {
  pub fn dtdPublicId<T: QXmlStreamReader_dtdPublicId>(&mut self, value: T) -> i32 {
    value.dtdPublicId(self);
    return 1;
  }
}

pub trait QXmlStreamReader_dtdPublicId {
  fn dtdPublicId(self, this: &mut QXmlStreamReader) -> i32;
}

// proto: QStringRef QXmlStreamReader::dtdPublicId();
impl<'a> /*trait*/ QXmlStreamReader_dtdPublicId for () {
  fn dtdPublicId(self, this: &mut QXmlStreamReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader11dtdPublicIdEv()};
    unsafe {_ZNK16QXmlStreamReader11dtdPublicIdEv()};
    return 1;
  }
}

impl /*struct*/ QXmlStreamReader {
  pub fn documentEncoding<T: QXmlStreamReader_documentEncoding>(&mut self, value: T) -> i32 {
    value.documentEncoding(self);
    return 1;
  }
}

pub trait QXmlStreamReader_documentEncoding {
  fn documentEncoding(self, this: &mut QXmlStreamReader) -> i32;
}

// proto: QStringRef QXmlStreamReader::documentEncoding();
impl<'a> /*trait*/ QXmlStreamReader_documentEncoding for () {
  fn documentEncoding(self, this: &mut QXmlStreamReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader16documentEncodingEv()};
    unsafe {_ZNK16QXmlStreamReader16documentEncodingEv()};
    return 1;
  }
}

impl /*struct*/ QXmlStreamReader {
  pub fn characterOffset<T: QXmlStreamReader_characterOffset>(&mut self, value: T) -> i32 {
    value.characterOffset(self);
    return 1;
  }
}

pub trait QXmlStreamReader_characterOffset {
  fn characterOffset(self, this: &mut QXmlStreamReader) -> i32;
}

// proto: long long QXmlStreamReader::characterOffset();
impl<'a> /*trait*/ QXmlStreamReader_characterOffset for () {
  fn characterOffset(self, this: &mut QXmlStreamReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader15characterOffsetEv()};
    unsafe {_ZNK16QXmlStreamReader15characterOffsetEv()};
    return 1;
  }
}

impl /*struct*/ QXmlStreamReader {
  pub fn attributes<T: QXmlStreamReader_attributes>(&mut self, value: T) -> i32 {
    value.attributes(self);
    return 1;
  }
}

pub trait QXmlStreamReader_attributes {
  fn attributes(self, this: &mut QXmlStreamReader) -> i32;
}

// proto: QXmlStreamAttributes QXmlStreamReader::attributes();
impl<'a> /*trait*/ QXmlStreamReader_attributes for () {
  fn attributes(self, this: &mut QXmlStreamReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader10attributesEv()};
    unsafe {_ZNK16QXmlStreamReader10attributesEv()};
    return 1;
  }
}

impl /*struct*/ QXmlStreamReader {
  pub fn tokenString<T: QXmlStreamReader_tokenString>(&mut self, value: T) -> i32 {
    value.tokenString(self);
    return 1;
  }
}

pub trait QXmlStreamReader_tokenString {
  fn tokenString(self, this: &mut QXmlStreamReader) -> i32;
}

// proto: QString QXmlStreamReader::tokenString();
impl<'a> /*trait*/ QXmlStreamReader_tokenString for () {
  fn tokenString(self, this: &mut QXmlStreamReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader11tokenStringEv()};
    unsafe {_ZNK16QXmlStreamReader11tokenStringEv()};
    return 1;
  }
}

impl /*struct*/ QXmlStreamReader {
  pub fn addExtraNamespaceDeclaration<T: QXmlStreamReader_addExtraNamespaceDeclaration>(&mut self, value: T) -> i32 {
    value.addExtraNamespaceDeclaration(self);
    return 1;
  }
}

pub trait QXmlStreamReader_addExtraNamespaceDeclaration {
  fn addExtraNamespaceDeclaration(self, this: &mut QXmlStreamReader) -> i32;
}

// proto: void QXmlStreamReader::addExtraNamespaceDeclaration(const QXmlStreamNamespaceDeclaration & extraNamespaceDeclaraction);
impl<'a> /*trait*/ QXmlStreamReader_addExtraNamespaceDeclaration for (&'a  QXmlStreamNamespaceDeclaration) {
  fn addExtraNamespaceDeclaration(self, this: &mut QXmlStreamReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamReader28addExtraNamespaceDeclarationERK30QXmlStreamNamespaceDeclaration()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN16QXmlStreamReader28addExtraNamespaceDeclarationERK30QXmlStreamNamespaceDeclaration(arg0)};
    return 1;
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
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN16QXmlStreamReaderC1ERK10QByteArray(qthis, arg0)};
    let rsthis = QXmlStreamReader{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QXmlStreamReader {
  pub fn qualifiedName<T: QXmlStreamReader_qualifiedName>(&mut self, value: T) -> i32 {
    value.qualifiedName(self);
    return 1;
  }
}

pub trait QXmlStreamReader_qualifiedName {
  fn qualifiedName(self, this: &mut QXmlStreamReader) -> i32;
}

// proto: QStringRef QXmlStreamReader::qualifiedName();
impl<'a> /*trait*/ QXmlStreamReader_qualifiedName for () {
  fn qualifiedName(self, this: &mut QXmlStreamReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader13qualifiedNameEv()};
    unsafe {_ZNK16QXmlStreamReader13qualifiedNameEv()};
    return 1;
  }
}

impl /*struct*/ QXmlStreamReader {
  pub fn device<T: QXmlStreamReader_device>(&mut self, value: T) -> i32 {
    value.device(self);
    return 1;
  }
}

pub trait QXmlStreamReader_device {
  fn device(self, this: &mut QXmlStreamReader) -> i32;
}

// proto: QIODevice * QXmlStreamReader::device();
impl<'a> /*trait*/ QXmlStreamReader_device for () {
  fn device(self, this: &mut QXmlStreamReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader6deviceEv()};
    unsafe {_ZNK16QXmlStreamReader6deviceEv()};
    return 1;
  }
}

impl /*struct*/ QXmlStreamReader {
  pub fn namespaceUri<T: QXmlStreamReader_namespaceUri>(&mut self, value: T) -> i32 {
    value.namespaceUri(self);
    return 1;
  }
}

pub trait QXmlStreamReader_namespaceUri {
  fn namespaceUri(self, this: &mut QXmlStreamReader) -> i32;
}

// proto: QStringRef QXmlStreamReader::namespaceUri();
impl<'a> /*trait*/ QXmlStreamReader_namespaceUri for () {
  fn namespaceUri(self, this: &mut QXmlStreamReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader12namespaceUriEv()};
    unsafe {_ZNK16QXmlStreamReader12namespaceUriEv()};
    return 1;
  }
}

impl /*struct*/ QXmlStreamReader {
  pub fn text<T: QXmlStreamReader_text>(&mut self, value: T) -> i32 {
    value.text(self);
    return 1;
  }
}

pub trait QXmlStreamReader_text {
  fn text(self, this: &mut QXmlStreamReader) -> i32;
}

// proto: QStringRef QXmlStreamReader::text();
impl<'a> /*trait*/ QXmlStreamReader_text for () {
  fn text(self, this: &mut QXmlStreamReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader4textEv()};
    unsafe {_ZNK16QXmlStreamReader4textEv()};
    return 1;
  }
}

impl /*struct*/ QXmlStreamReader {
  pub fn setDevice<T: QXmlStreamReader_setDevice>(&mut self, value: T) -> i32 {
    value.setDevice(self);
    return 1;
  }
}

pub trait QXmlStreamReader_setDevice {
  fn setDevice(self, this: &mut QXmlStreamReader) -> i32;
}

// proto: void QXmlStreamReader::setDevice(QIODevice * device);
impl<'a> /*trait*/ QXmlStreamReader_setDevice for (&'a mut QIODevice) {
  fn setDevice(self, this: &mut QXmlStreamReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamReader9setDeviceEP9QIODevice()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN16QXmlStreamReader9setDeviceEP9QIODevice(arg0)};
    return 1;
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
  pub fn documentVersion<T: QXmlStreamReader_documentVersion>(&mut self, value: T) -> i32 {
    value.documentVersion(self);
    return 1;
  }
}

pub trait QXmlStreamReader_documentVersion {
  fn documentVersion(self, this: &mut QXmlStreamReader) -> i32;
}

// proto: QStringRef QXmlStreamReader::documentVersion();
impl<'a> /*trait*/ QXmlStreamReader_documentVersion for () {
  fn documentVersion(self, this: &mut QXmlStreamReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader15documentVersionEv()};
    unsafe {_ZNK16QXmlStreamReader15documentVersionEv()};
    return 1;
  }
}

impl /*struct*/ QXmlStreamReader {
  pub fn isDTD<T: QXmlStreamReader_isDTD>(&mut self, value: T) -> i32 {
    value.isDTD(self);
    return 1;
  }
}

pub trait QXmlStreamReader_isDTD {
  fn isDTD(self, this: &mut QXmlStreamReader) -> i32;
}

// proto: bool QXmlStreamReader::isDTD();
impl<'a> /*trait*/ QXmlStreamReader_isDTD for () {
  fn isDTD(self, this: &mut QXmlStreamReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader5isDTDEv()};
    unsafe {_ZNK16QXmlStreamReader5isDTDEv()};
    return 1;
  }
}

impl /*struct*/ QXmlStreamReader {
  pub fn isStartDocument<T: QXmlStreamReader_isStartDocument>(&mut self, value: T) -> i32 {
    value.isStartDocument(self);
    return 1;
  }
}

pub trait QXmlStreamReader_isStartDocument {
  fn isStartDocument(self, this: &mut QXmlStreamReader) -> i32;
}

// proto: bool QXmlStreamReader::isStartDocument();
impl<'a> /*trait*/ QXmlStreamReader_isStartDocument for () {
  fn isStartDocument(self, this: &mut QXmlStreamReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader15isStartDocumentEv()};
    unsafe {_ZNK16QXmlStreamReader15isStartDocumentEv()};
    return 1;
  }
}

impl /*struct*/ QXmlStreamReader {
  pub fn errorString<T: QXmlStreamReader_errorString>(&mut self, value: T) -> i32 {
    value.errorString(self);
    return 1;
  }
}

pub trait QXmlStreamReader_errorString {
  fn errorString(self, this: &mut QXmlStreamReader) -> i32;
}

// proto: QString QXmlStreamReader::errorString();
impl<'a> /*trait*/ QXmlStreamReader_errorString for () {
  fn errorString(self, this: &mut QXmlStreamReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader11errorStringEv()};
    unsafe {_ZNK16QXmlStreamReader11errorStringEv()};
    return 1;
  }
}

impl /*struct*/ QXmlStreamReader {
  pub fn isProcessingInstruction<T: QXmlStreamReader_isProcessingInstruction>(&mut self, value: T) -> i32 {
    value.isProcessingInstruction(self);
    return 1;
  }
}

pub trait QXmlStreamReader_isProcessingInstruction {
  fn isProcessingInstruction(self, this: &mut QXmlStreamReader) -> i32;
}

// proto: bool QXmlStreamReader::isProcessingInstruction();
impl<'a> /*trait*/ QXmlStreamReader_isProcessingInstruction for () {
  fn isProcessingInstruction(self, this: &mut QXmlStreamReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader23isProcessingInstructionEv()};
    unsafe {_ZNK16QXmlStreamReader23isProcessingInstructionEv()};
    return 1;
  }
}

impl /*struct*/ QXmlStreamReader {
  pub fn setEntityResolver<T: QXmlStreamReader_setEntityResolver>(&mut self, value: T) -> i32 {
    value.setEntityResolver(self);
    return 1;
  }
}

pub trait QXmlStreamReader_setEntityResolver {
  fn setEntityResolver(self, this: &mut QXmlStreamReader) -> i32;
}

// proto: void QXmlStreamReader::setEntityResolver(QXmlStreamEntityResolver * resolver);
impl<'a> /*trait*/ QXmlStreamReader_setEntityResolver for (&'a mut QXmlStreamEntityResolver) {
  fn setEntityResolver(self, this: &mut QXmlStreamReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamReader17setEntityResolverEP24QXmlStreamEntityResolver()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN16QXmlStreamReader17setEntityResolverEP24QXmlStreamEntityResolver(arg0)};
    return 1;
  }
}

impl /*struct*/ QXmlStreamReader {
  pub fn isCharacters<T: QXmlStreamReader_isCharacters>(&mut self, value: T) -> i32 {
    value.isCharacters(self);
    return 1;
  }
}

pub trait QXmlStreamReader_isCharacters {
  fn isCharacters(self, this: &mut QXmlStreamReader) -> i32;
}

// proto: bool QXmlStreamReader::isCharacters();
impl<'a> /*trait*/ QXmlStreamReader_isCharacters for () {
  fn isCharacters(self, this: &mut QXmlStreamReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader12isCharactersEv()};
    unsafe {_ZNK16QXmlStreamReader12isCharactersEv()};
    return 1;
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
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN16QXmlStreamReaderC1ERK7QString(qthis, arg0)};
    let rsthis = QXmlStreamReader{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QXmlStreamReader {
  pub fn entityDeclarations<T: QXmlStreamReader_entityDeclarations>(&mut self, value: T) -> i32 {
    value.entityDeclarations(self);
    return 1;
  }
}

pub trait QXmlStreamReader_entityDeclarations {
  fn entityDeclarations(self, this: &mut QXmlStreamReader) -> i32;
}

// proto: QVector<QXmlStreamEntityDeclaration> QXmlStreamReader::entityDeclarations();
impl<'a> /*trait*/ QXmlStreamReader_entityDeclarations for () {
  fn entityDeclarations(self, this: &mut QXmlStreamReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader18entityDeclarationsEv()};
    unsafe {_ZNK16QXmlStreamReader18entityDeclarationsEv()};
    return 1;
  }
}

impl /*struct*/ QXmlStreamReader {
  pub fn isWhitespace<T: QXmlStreamReader_isWhitespace>(&mut self, value: T) -> i32 {
    value.isWhitespace(self);
    return 1;
  }
}

pub trait QXmlStreamReader_isWhitespace {
  fn isWhitespace(self, this: &mut QXmlStreamReader) -> i32;
}

// proto: bool QXmlStreamReader::isWhitespace();
impl<'a> /*trait*/ QXmlStreamReader_isWhitespace for () {
  fn isWhitespace(self, this: &mut QXmlStreamReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader12isWhitespaceEv()};
    unsafe {_ZNK16QXmlStreamReader12isWhitespaceEv()};
    return 1;
  }
}

// proto: void QXmlStreamReader::NewQXmlStreamReader(const QXmlStreamReader & );
impl<'a> /*trait*/ QXmlStreamReader_NewQXmlStreamReader for (&'a  QXmlStreamReader) {
  fn NewQXmlStreamReader(self) -> QXmlStreamReader {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamReaderC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN16QXmlStreamReaderC1ERKS_(qthis, arg0)};
    let rsthis = QXmlStreamReader{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QXmlStreamReader {
  pub fn columnNumber<T: QXmlStreamReader_columnNumber>(&mut self, value: T) -> i32 {
    value.columnNumber(self);
    return 1;
  }
}

pub trait QXmlStreamReader_columnNumber {
  fn columnNumber(self, this: &mut QXmlStreamReader) -> i32;
}

// proto: long long QXmlStreamReader::columnNumber();
impl<'a> /*trait*/ QXmlStreamReader_columnNumber for () {
  fn columnNumber(self, this: &mut QXmlStreamReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader12columnNumberEv()};
    unsafe {_ZNK16QXmlStreamReader12columnNumberEv()};
    return 1;
  }
}

impl /*struct*/ QXmlStreamReader {
  pub fn hasError<T: QXmlStreamReader_hasError>(&mut self, value: T) -> i32 {
    value.hasError(self);
    return 1;
  }
}

pub trait QXmlStreamReader_hasError {
  fn hasError(self, this: &mut QXmlStreamReader) -> i32;
}

// proto: bool QXmlStreamReader::hasError();
impl<'a> /*trait*/ QXmlStreamReader_hasError for () {
  fn hasError(self, this: &mut QXmlStreamReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader8hasErrorEv()};
    unsafe {_ZNK16QXmlStreamReader8hasErrorEv()};
    return 1;
  }
}

impl /*struct*/ QXmlStreamReader {
  pub fn isCDATA<T: QXmlStreamReader_isCDATA>(&mut self, value: T) -> i32 {
    value.isCDATA(self);
    return 1;
  }
}

pub trait QXmlStreamReader_isCDATA {
  fn isCDATA(self, this: &mut QXmlStreamReader) -> i32;
}

// proto: bool QXmlStreamReader::isCDATA();
impl<'a> /*trait*/ QXmlStreamReader_isCDATA for () {
  fn isCDATA(self, this: &mut QXmlStreamReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader7isCDATAEv()};
    unsafe {_ZNK16QXmlStreamReader7isCDATAEv()};
    return 1;
  }
}

impl /*struct*/ QXmlStreamReader {
  pub fn FreeQXmlStreamReader<T: QXmlStreamReader_FreeQXmlStreamReader>(&mut self, value: T) -> i32 {
    value.FreeQXmlStreamReader(self);
    return 1;
  }
}

pub trait QXmlStreamReader_FreeQXmlStreamReader {
  fn FreeQXmlStreamReader(self, this: &mut QXmlStreamReader) -> i32;
}

// proto: void QXmlStreamReader::FreeQXmlStreamReader();
impl<'a> /*trait*/ QXmlStreamReader_FreeQXmlStreamReader for () {
  fn FreeQXmlStreamReader(self, this: &mut QXmlStreamReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamReaderD0Ev()};
    unsafe {_ZN16QXmlStreamReaderD0Ev()};
    return 1;
  }
}

impl /*struct*/ QXmlStreamReader {
  pub fn processingInstructionTarget<T: QXmlStreamReader_processingInstructionTarget>(&mut self, value: T) -> i32 {
    value.processingInstructionTarget(self);
    return 1;
  }
}

pub trait QXmlStreamReader_processingInstructionTarget {
  fn processingInstructionTarget(self, this: &mut QXmlStreamReader) -> i32;
}

// proto: QStringRef QXmlStreamReader::processingInstructionTarget();
impl<'a> /*trait*/ QXmlStreamReader_processingInstructionTarget for () {
  fn processingInstructionTarget(self, this: &mut QXmlStreamReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader27processingInstructionTargetEv()};
    unsafe {_ZNK16QXmlStreamReader27processingInstructionTargetEv()};
    return 1;
  }
}

// proto: void QXmlStreamReader::addData(const char * data);
impl<'a> /*trait*/ QXmlStreamReader_addData for (&'a  String) {
  fn addData(self, this: &mut QXmlStreamReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamReader7addDataEPKc()};
    let arg0 = self.as_ptr()  as *const c_char;
    unsafe {_ZN16QXmlStreamReader7addDataEPKc(arg0)};
    return 1;
  }
}

impl /*struct*/ QXmlStreamReader {
  pub fn dtdSystemId<T: QXmlStreamReader_dtdSystemId>(&mut self, value: T) -> i32 {
    value.dtdSystemId(self);
    return 1;
  }
}

pub trait QXmlStreamReader_dtdSystemId {
  fn dtdSystemId(self, this: &mut QXmlStreamReader) -> i32;
}

// proto: QStringRef QXmlStreamReader::dtdSystemId();
impl<'a> /*trait*/ QXmlStreamReader_dtdSystemId for () {
  fn dtdSystemId(self, this: &mut QXmlStreamReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader11dtdSystemIdEv()};
    unsafe {_ZNK16QXmlStreamReader11dtdSystemIdEv()};
    return 1;
  }
}

impl /*struct*/ QXmlStreamReader {
  pub fn prefix<T: QXmlStreamReader_prefix>(&mut self, value: T) -> i32 {
    value.prefix(self);
    return 1;
  }
}

pub trait QXmlStreamReader_prefix {
  fn prefix(self, this: &mut QXmlStreamReader) -> i32;
}

// proto: QStringRef QXmlStreamReader::prefix();
impl<'a> /*trait*/ QXmlStreamReader_prefix for () {
  fn prefix(self, this: &mut QXmlStreamReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader6prefixEv()};
    unsafe {_ZNK16QXmlStreamReader6prefixEv()};
    return 1;
  }
}

impl /*struct*/ QXmlStreamReader {
  pub fn isEndElement<T: QXmlStreamReader_isEndElement>(&mut self, value: T) -> i32 {
    value.isEndElement(self);
    return 1;
  }
}

pub trait QXmlStreamReader_isEndElement {
  fn isEndElement(self, this: &mut QXmlStreamReader) -> i32;
}

// proto: bool QXmlStreamReader::isEndElement();
impl<'a> /*trait*/ QXmlStreamReader_isEndElement for () {
  fn isEndElement(self, this: &mut QXmlStreamReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader12isEndElementEv()};
    unsafe {_ZNK16QXmlStreamReader12isEndElementEv()};
    return 1;
  }
}

impl /*struct*/ QXmlStreamReader {
  pub fn notationDeclarations<T: QXmlStreamReader_notationDeclarations>(&mut self, value: T) -> i32 {
    value.notationDeclarations(self);
    return 1;
  }
}

pub trait QXmlStreamReader_notationDeclarations {
  fn notationDeclarations(self, this: &mut QXmlStreamReader) -> i32;
}

// proto: QVector<QXmlStreamNotationDeclaration> QXmlStreamReader::notationDeclarations();
impl<'a> /*trait*/ QXmlStreamReader_notationDeclarations for () {
  fn notationDeclarations(self, this: &mut QXmlStreamReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader20notationDeclarationsEv()};
    unsafe {_ZNK16QXmlStreamReader20notationDeclarationsEv()};
    return 1;
  }
}

impl /*struct*/ QXmlStreamReader {
  pub fn namespaceDeclarations<T: QXmlStreamReader_namespaceDeclarations>(&mut self, value: T) -> i32 {
    value.namespaceDeclarations(self);
    return 1;
  }
}

pub trait QXmlStreamReader_namespaceDeclarations {
  fn namespaceDeclarations(self, this: &mut QXmlStreamReader) -> i32;
}

// proto: QVector<QXmlStreamNamespaceDeclaration> QXmlStreamReader::namespaceDeclarations();
impl<'a> /*trait*/ QXmlStreamReader_namespaceDeclarations for () {
  fn namespaceDeclarations(self, this: &mut QXmlStreamReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader21namespaceDeclarationsEv()};
    unsafe {_ZNK16QXmlStreamReader21namespaceDeclarationsEv()};
    return 1;
  }
}

impl /*struct*/ QXmlStreamReader {
  pub fn setNamespaceProcessing<T: QXmlStreamReader_setNamespaceProcessing>(&mut self, value: T) -> i32 {
    value.setNamespaceProcessing(self);
    return 1;
  }
}

pub trait QXmlStreamReader_setNamespaceProcessing {
  fn setNamespaceProcessing(self, this: &mut QXmlStreamReader) -> i32;
}

// proto: void QXmlStreamReader::setNamespaceProcessing(bool );
impl<'a> /*trait*/ QXmlStreamReader_setNamespaceProcessing for (i8) {
  fn setNamespaceProcessing(self, this: &mut QXmlStreamReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamReader22setNamespaceProcessingEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN16QXmlStreamReader22setNamespaceProcessingEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QXmlStreamReader {
  pub fn raiseError<T: QXmlStreamReader_raiseError>(&mut self, value: T) -> i32 {
    value.raiseError(self);
    return 1;
  }
}

pub trait QXmlStreamReader_raiseError {
  fn raiseError(self, this: &mut QXmlStreamReader) -> i32;
}

// proto: void QXmlStreamReader::raiseError(const QString & message);
impl<'a> /*trait*/ QXmlStreamReader_raiseError for (&'a  QString) {
  fn raiseError(self, this: &mut QXmlStreamReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamReader10raiseErrorERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN16QXmlStreamReader10raiseErrorERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QXmlStreamReader {
  pub fn dtdName<T: QXmlStreamReader_dtdName>(&mut self, value: T) -> i32 {
    value.dtdName(self);
    return 1;
  }
}

pub trait QXmlStreamReader_dtdName {
  fn dtdName(self, this: &mut QXmlStreamReader) -> i32;
}

// proto: QStringRef QXmlStreamReader::dtdName();
impl<'a> /*trait*/ QXmlStreamReader_dtdName for () {
  fn dtdName(self, this: &mut QXmlStreamReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader7dtdNameEv()};
    unsafe {_ZNK16QXmlStreamReader7dtdNameEv()};
    return 1;
  }
}

impl /*struct*/ QXmlStreamReader {
  pub fn isEndDocument<T: QXmlStreamReader_isEndDocument>(&mut self, value: T) -> i32 {
    value.isEndDocument(self);
    return 1;
  }
}

pub trait QXmlStreamReader_isEndDocument {
  fn isEndDocument(self, this: &mut QXmlStreamReader) -> i32;
}

// proto: bool QXmlStreamReader::isEndDocument();
impl<'a> /*trait*/ QXmlStreamReader_isEndDocument for () {
  fn isEndDocument(self, this: &mut QXmlStreamReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader13isEndDocumentEv()};
    unsafe {_ZNK16QXmlStreamReader13isEndDocumentEv()};
    return 1;
  }
}

impl /*struct*/ QXmlStreamReader {
  pub fn readNextStartElement<T: QXmlStreamReader_readNextStartElement>(&mut self, value: T) -> i32 {
    value.readNextStartElement(self);
    return 1;
  }
}

pub trait QXmlStreamReader_readNextStartElement {
  fn readNextStartElement(self, this: &mut QXmlStreamReader) -> i32;
}

// proto: bool QXmlStreamReader::readNextStartElement();
impl<'a> /*trait*/ QXmlStreamReader_readNextStartElement for () {
  fn readNextStartElement(self, this: &mut QXmlStreamReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamReader20readNextStartElementEv()};
    unsafe {_ZN16QXmlStreamReader20readNextStartElementEv()};
    return 1;
  }
}

impl /*struct*/ QXmlStreamReader {
  pub fn isComment<T: QXmlStreamReader_isComment>(&mut self, value: T) -> i32 {
    value.isComment(self);
    return 1;
  }
}

pub trait QXmlStreamReader_isComment {
  fn isComment(self, this: &mut QXmlStreamReader) -> i32;
}

// proto: bool QXmlStreamReader::isComment();
impl<'a> /*trait*/ QXmlStreamReader_isComment for () {
  fn isComment(self, this: &mut QXmlStreamReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader9isCommentEv()};
    unsafe {_ZNK16QXmlStreamReader9isCommentEv()};
    return 1;
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
  pub fn skipCurrentElement<T: QXmlStreamReader_skipCurrentElement>(&mut self, value: T) -> i32 {
    value.skipCurrentElement(self);
    return 1;
  }
}

pub trait QXmlStreamReader_skipCurrentElement {
  fn skipCurrentElement(self, this: &mut QXmlStreamReader) -> i32;
}

// proto: void QXmlStreamReader::skipCurrentElement();
impl<'a> /*trait*/ QXmlStreamReader_skipCurrentElement for () {
  fn skipCurrentElement(self, this: &mut QXmlStreamReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamReader18skipCurrentElementEv()};
    unsafe {_ZN16QXmlStreamReader18skipCurrentElementEv()};
    return 1;
  }
}

impl /*struct*/ QXmlStreamReader {
  pub fn isEntityReference<T: QXmlStreamReader_isEntityReference>(&mut self, value: T) -> i32 {
    value.isEntityReference(self);
    return 1;
  }
}

pub trait QXmlStreamReader_isEntityReference {
  fn isEntityReference(self, this: &mut QXmlStreamReader) -> i32;
}

// proto: bool QXmlStreamReader::isEntityReference();
impl<'a> /*trait*/ QXmlStreamReader_isEntityReference for () {
  fn isEntityReference(self, this: &mut QXmlStreamReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader17isEntityReferenceEv()};
    unsafe {_ZNK16QXmlStreamReader17isEntityReferenceEv()};
    return 1;
  }
}

// proto: void QXmlStreamReader::addData(const QByteArray & data);
impl<'a> /*trait*/ QXmlStreamReader_addData for (&'a  QByteArray) {
  fn addData(self, this: &mut QXmlStreamReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamReader7addDataERK10QByteArray()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN16QXmlStreamReader7addDataERK10QByteArray(arg0)};
    return 1;
  }
}

impl /*struct*/ QXmlStreamReader {
  pub fn atEnd<T: QXmlStreamReader_atEnd>(&mut self, value: T) -> i32 {
    value.atEnd(self);
    return 1;
  }
}

pub trait QXmlStreamReader_atEnd {
  fn atEnd(self, this: &mut QXmlStreamReader) -> i32;
}

// proto: bool QXmlStreamReader::atEnd();
impl<'a> /*trait*/ QXmlStreamReader_atEnd for () {
  fn atEnd(self, this: &mut QXmlStreamReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader5atEndEv()};
    unsafe {_ZNK16QXmlStreamReader5atEndEv()};
    return 1;
  }
}

