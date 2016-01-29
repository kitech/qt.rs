// auto generated, do not modify.
// created: Thu Jan 28 22:38:45 2016
// src-file: /QtCore/qxmlstream.h
// dst-file: /src/core/qxmlstream.rs
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
use std::ops::Deref;
use super::qstring::*; // 773
// use super::qxmlstream::QXmlStreamEntityResolver; // 773
// use super::qxmlstream::QXmlStreamAttributes; // 773
// use super::qxmlstream::QXmlStreamNamespaceDeclaration; // 773
use super::qbytearray::*; // 773
use super::qiodevice::*; // 773
// use super::qvector::*; // 775
use super::qtextcodec::*; // 773
// use super::qxmlstream::QXmlStreamAttribute; // 773
// use super::qxmlstream::QXmlStreamReader; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QXmlStreamStringRef_Class_Size() -> c_int;
  // proto:  int QXmlStreamStringRef::size();
  fn C_ZNK19QXmlStreamStringRef4sizeEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QXmlStreamStringRef::clear();
  fn C_ZN19QXmlStreamStringRef5clearEv(qthis: u64 /* *mut c_void*/);
  // proto:  const QString * QXmlStreamStringRef::string();
  fn C_ZNK19QXmlStreamStringRef6stringEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QXmlStreamStringRef::QXmlStreamStringRef(const QString & aString);
  fn C_ZN19QXmlStreamStringRefC2ERK7QString(arg0: *mut c_void) -> u64;
  // proto:  void QXmlStreamStringRef::~QXmlStreamStringRef();
  fn C_ZN19QXmlStreamStringRefD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QXmlStreamStringRef::QXmlStreamStringRef();
  fn C_ZN19QXmlStreamStringRefC2Ev() -> u64;
  // proto:  int QXmlStreamStringRef::position();
  fn C_ZNK19QXmlStreamStringRef8positionEv(qthis: u64 /* *mut c_void*/) -> c_int;
  fn QXmlStreamReader_Class_Size() -> c_int;
  // proto:  QStringRef QXmlStreamReader::name();
  fn C_ZNK16QXmlStreamReader4nameEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QXmlStreamEntityResolver * QXmlStreamReader::entityResolver();
  fn C_ZNK16QXmlStreamReader14entityResolverEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QXmlStreamReader::namespaceProcessing();
  fn C_ZNK16QXmlStreamReader19namespaceProcessingEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QXmlStreamReader::isStartElement();
  fn C_ZNK16QXmlStreamReader14isStartElementEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QXmlStreamReader::isStandaloneDocument();
  fn C_ZNK16QXmlStreamReader20isStandaloneDocumentEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  qint64 QXmlStreamReader::lineNumber();
  fn C_ZNK16QXmlStreamReader10lineNumberEv(qthis: u64 /* *mut c_void*/) -> c_longlong;
  // proto:  void QXmlStreamReader::clear();
  fn C_ZN16QXmlStreamReader5clearEv(qthis: u64 /* *mut c_void*/);
  // proto:  QStringRef QXmlStreamReader::processingInstructionData();
  fn C_ZNK16QXmlStreamReader25processingInstructionDataEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QXmlStreamReader::addData(const QString & data);
  fn C_ZN16QXmlStreamReader7addDataERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QStringRef QXmlStreamReader::dtdPublicId();
  fn C_ZNK16QXmlStreamReader11dtdPublicIdEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QStringRef QXmlStreamReader::documentEncoding();
  fn C_ZNK16QXmlStreamReader16documentEncodingEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  qint64 QXmlStreamReader::characterOffset();
  fn C_ZNK16QXmlStreamReader15characterOffsetEv(qthis: u64 /* *mut c_void*/) -> c_longlong;
  // proto:  QXmlStreamAttributes QXmlStreamReader::attributes();
  fn C_ZNK16QXmlStreamReader10attributesEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QString QXmlStreamReader::tokenString();
  fn C_ZNK16QXmlStreamReader11tokenStringEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QXmlStreamReader::addExtraNamespaceDeclaration(const QXmlStreamNamespaceDeclaration & extraNamespaceDeclaraction);
  fn C_ZN16QXmlStreamReader28addExtraNamespaceDeclarationERK30QXmlStreamNamespaceDeclaration(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QXmlStreamReader::QXmlStreamReader(const QByteArray & data);
  fn C_ZN16QXmlStreamReaderC2ERK10QByteArray(arg0: *mut c_void) -> u64;
  // proto:  QStringRef QXmlStreamReader::qualifiedName();
  fn C_ZNK16QXmlStreamReader13qualifiedNameEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QIODevice * QXmlStreamReader::device();
  fn C_ZNK16QXmlStreamReader6deviceEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QStringRef QXmlStreamReader::namespaceUri();
  fn C_ZNK16QXmlStreamReader12namespaceUriEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QStringRef QXmlStreamReader::text();
  fn C_ZNK16QXmlStreamReader4textEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QXmlStreamReader::setDevice(QIODevice * device);
  fn C_ZN16QXmlStreamReader9setDeviceEP9QIODevice(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QXmlStreamReader::QXmlStreamReader(QIODevice * device);
  fn C_ZN16QXmlStreamReaderC2EP9QIODevice(arg0: *mut c_void) -> u64;
  // proto:  QStringRef QXmlStreamReader::documentVersion();
  fn C_ZNK16QXmlStreamReader15documentVersionEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QXmlStreamReader::isDTD();
  fn C_ZNK16QXmlStreamReader5isDTDEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QXmlStreamReader::isStartDocument();
  fn C_ZNK16QXmlStreamReader15isStartDocumentEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QString QXmlStreamReader::errorString();
  fn C_ZNK16QXmlStreamReader11errorStringEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QXmlStreamReader::isProcessingInstruction();
  fn C_ZNK16QXmlStreamReader23isProcessingInstructionEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QXmlStreamReader::setEntityResolver(QXmlStreamEntityResolver * resolver);
  fn C_ZN16QXmlStreamReader17setEntityResolverEP24QXmlStreamEntityResolver(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QXmlStreamReader::isCharacters();
  fn C_ZNK16QXmlStreamReader12isCharactersEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QXmlStreamReader::QXmlStreamReader();
  fn C_ZN16QXmlStreamReaderC2Ev() -> u64;
  // proto:  void QXmlStreamReader::QXmlStreamReader(const QString & data);
  fn C_ZN16QXmlStreamReaderC2ERK7QString(arg0: *mut c_void) -> u64;
  // proto:  QXmlStreamEntityDeclarations QXmlStreamReader::entityDeclarations();
  fn C_ZNK16QXmlStreamReader18entityDeclarationsEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QXmlStreamReader::isWhitespace();
  fn C_ZNK16QXmlStreamReader12isWhitespaceEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  qint64 QXmlStreamReader::columnNumber();
  fn C_ZNK16QXmlStreamReader12columnNumberEv(qthis: u64 /* *mut c_void*/) -> c_longlong;
  // proto:  bool QXmlStreamReader::hasError();
  fn C_ZNK16QXmlStreamReader8hasErrorEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QXmlStreamReader::isCDATA();
  fn C_ZNK16QXmlStreamReader7isCDATAEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QXmlStreamReader::~QXmlStreamReader();
  fn C_ZN16QXmlStreamReaderD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QStringRef QXmlStreamReader::processingInstructionTarget();
  fn C_ZNK16QXmlStreamReader27processingInstructionTargetEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QXmlStreamReader::addData(const char * data);
  fn C_ZN16QXmlStreamReader7addDataEPKc(qthis: u64 /* *mut c_void*/, arg0: *mut c_char);
  // proto:  QStringRef QXmlStreamReader::dtdSystemId();
  fn C_ZNK16QXmlStreamReader11dtdSystemIdEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QStringRef QXmlStreamReader::prefix();
  fn C_ZNK16QXmlStreamReader6prefixEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QXmlStreamReader::isEndElement();
  fn C_ZNK16QXmlStreamReader12isEndElementEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QXmlStreamNotationDeclarations QXmlStreamReader::notationDeclarations();
  fn C_ZNK16QXmlStreamReader20notationDeclarationsEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QXmlStreamNamespaceDeclarations QXmlStreamReader::namespaceDeclarations();
  fn C_ZNK16QXmlStreamReader21namespaceDeclarationsEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QXmlStreamReader::setNamespaceProcessing(bool );
  fn C_ZN16QXmlStreamReader22setNamespaceProcessingEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QXmlStreamReader::raiseError(const QString & message);
  fn C_ZN16QXmlStreamReader10raiseErrorERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QStringRef QXmlStreamReader::dtdName();
  fn C_ZNK16QXmlStreamReader7dtdNameEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QXmlStreamReader::isEndDocument();
  fn C_ZNK16QXmlStreamReader13isEndDocumentEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QXmlStreamReader::readNextStartElement();
  fn C_ZN16QXmlStreamReader20readNextStartElementEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QXmlStreamReader::isComment();
  fn C_ZNK16QXmlStreamReader9isCommentEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QXmlStreamReader::QXmlStreamReader(const char * data);
  fn C_ZN16QXmlStreamReaderC2EPKc(arg0: *mut c_char) -> u64;
  // proto:  void QXmlStreamReader::skipCurrentElement();
  fn C_ZN16QXmlStreamReader18skipCurrentElementEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QXmlStreamReader::isEntityReference();
  fn C_ZNK16QXmlStreamReader17isEntityReferenceEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QXmlStreamReader::addData(const QByteArray & data);
  fn C_ZN16QXmlStreamReader7addDataERK10QByteArray(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QXmlStreamReader::atEnd();
  fn C_ZNK16QXmlStreamReader5atEndEv(qthis: u64 /* *mut c_void*/) -> c_char;
  fn QXmlStreamEntityResolver_Class_Size() -> c_int;
  // proto:  QString QXmlStreamEntityResolver::resolveEntity(const QString & publicId, const QString & systemId);
  fn C_ZN24QXmlStreamEntityResolver13resolveEntityERK7QStringS2_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  QString QXmlStreamEntityResolver::resolveUndeclaredEntity(const QString & name);
  fn C_ZN24QXmlStreamEntityResolver23resolveUndeclaredEntityERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QXmlStreamEntityResolver::~QXmlStreamEntityResolver();
  fn C_ZN24QXmlStreamEntityResolverD2Ev(qthis: u64 /* *mut c_void*/);
  fn QXmlStreamNamespaceDeclaration_Class_Size() -> c_int;
  // proto:  QStringRef QXmlStreamNamespaceDeclaration::namespaceUri();
  fn C_ZNK30QXmlStreamNamespaceDeclaration12namespaceUriEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QXmlStreamNamespaceDeclaration::QXmlStreamNamespaceDeclaration(const QXmlStreamNamespaceDeclaration & );
  fn C_ZN30QXmlStreamNamespaceDeclarationC2ERKS_(arg0: *mut c_void) -> u64;
  // proto:  void QXmlStreamNamespaceDeclaration::QXmlStreamNamespaceDeclaration();
  fn C_ZN30QXmlStreamNamespaceDeclarationC2Ev() -> u64;
  // proto:  QStringRef QXmlStreamNamespaceDeclaration::prefix();
  fn C_ZNK30QXmlStreamNamespaceDeclaration6prefixEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QXmlStreamNamespaceDeclaration::QXmlStreamNamespaceDeclaration(const QString & prefix, const QString & namespaceUri);
  fn C_ZN30QXmlStreamNamespaceDeclarationC2ERK7QStringS2_(arg0: *mut c_void, arg1: *mut c_void) -> u64;
  // proto:  void QXmlStreamNamespaceDeclaration::~QXmlStreamNamespaceDeclaration();
  fn C_ZN30QXmlStreamNamespaceDeclarationD2Ev(qthis: u64 /* *mut c_void*/);
  fn QXmlStreamEntityDeclaration_Class_Size() -> c_int;
  // proto:  void QXmlStreamEntityDeclaration::~QXmlStreamEntityDeclaration();
  fn C_ZN27QXmlStreamEntityDeclarationD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QStringRef QXmlStreamEntityDeclaration::publicId();
  fn C_ZNK27QXmlStreamEntityDeclaration8publicIdEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QStringRef QXmlStreamEntityDeclaration::name();
  fn C_ZNK27QXmlStreamEntityDeclaration4nameEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QXmlStreamEntityDeclaration::QXmlStreamEntityDeclaration();
  fn C_ZN27QXmlStreamEntityDeclarationC2Ev() -> u64;
  // proto:  QStringRef QXmlStreamEntityDeclaration::value();
  fn C_ZNK27QXmlStreamEntityDeclaration5valueEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QStringRef QXmlStreamEntityDeclaration::notationName();
  fn C_ZNK27QXmlStreamEntityDeclaration12notationNameEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QXmlStreamEntityDeclaration::QXmlStreamEntityDeclaration(const QXmlStreamEntityDeclaration & );
  fn C_ZN27QXmlStreamEntityDeclarationC2ERKS_(arg0: *mut c_void) -> u64;
  // proto:  QStringRef QXmlStreamEntityDeclaration::systemId();
  fn C_ZNK27QXmlStreamEntityDeclaration8systemIdEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  fn QXmlStreamAttributes_Class_Size() -> c_int;
  // proto:  QStringRef QXmlStreamAttributes::value(const QString & qualifiedName);
  fn C_ZNK20QXmlStreamAttributes5valueERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QXmlStreamAttributes::QXmlStreamAttributes();
  fn C_ZN20QXmlStreamAttributesC2Ev() -> u64;
  // proto:  bool QXmlStreamAttributes::hasAttribute(const QString & qualifiedName);
  fn C_ZNK20QXmlStreamAttributes12hasAttributeERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  bool QXmlStreamAttributes::hasAttribute(const QString & namespaceUri, const QString & name);
  fn C_ZNK20QXmlStreamAttributes12hasAttributeERK7QStringS2_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void) -> c_char;
  // proto:  void QXmlStreamAttributes::append(const QString & namespaceUri, const QString & name, const QString & value);
  fn C_ZN20QXmlStreamAttributes6appendERK7QStringS2_S2_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void);
  // proto:  void QXmlStreamAttributes::append(const QString & qualifiedName, const QString & value);
  fn C_ZN20QXmlStreamAttributes6appendERK7QStringS2_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  QStringRef QXmlStreamAttributes::value(const QString & namespaceUri, const QString & name);
  fn C_ZNK20QXmlStreamAttributes5valueERK7QStringS2_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  fn QXmlStreamWriter_Class_Size() -> c_int;
  // proto:  void QXmlStreamWriter::writeEndElement();
  fn C_ZN16QXmlStreamWriter15writeEndElementEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QXmlStreamWriter::QXmlStreamWriter();
  fn C_ZN16QXmlStreamWriterC2Ev() -> u64;
  // proto:  void QXmlStreamWriter::writeEndDocument();
  fn C_ZN16QXmlStreamWriter16writeEndDocumentEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QXmlStreamWriter::autoFormatting();
  fn C_ZNK16QXmlStreamWriter14autoFormattingEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QXmlStreamWriter::writeStartDocument(const QString & version, bool standalone);
  fn C_ZN16QXmlStreamWriter18writeStartDocumentERK7QStringb(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_char);
  // proto:  void QXmlStreamWriter::setCodec(QTextCodec * codec);
  fn C_ZN16QXmlStreamWriter8setCodecEP10QTextCodec(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QXmlStreamWriter::writeProcessingInstruction(const QString & target, const QString & data);
  fn C_ZN16QXmlStreamWriter26writeProcessingInstructionERK7QStringS2_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QXmlStreamWriter::writeCharacters(const QString & text);
  fn C_ZN16QXmlStreamWriter15writeCharactersERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QXmlStreamWriter::setDevice(QIODevice * device);
  fn C_ZN16QXmlStreamWriter9setDeviceEP9QIODevice(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QXmlStreamWriter::writeStartDocument(const QString & version);
  fn C_ZN16QXmlStreamWriter18writeStartDocumentERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QXmlStreamWriter::writeTextElement(const QString & namespaceUri, const QString & name, const QString & text);
  fn C_ZN16QXmlStreamWriter16writeTextElementERK7QStringS2_S2_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void);
  // proto:  void QXmlStreamWriter::writeAttribute(const QString & qualifiedName, const QString & value);
  fn C_ZN16QXmlStreamWriter14writeAttributeERK7QStringS2_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QXmlStreamWriter::writeEmptyElement(const QString & qualifiedName);
  fn C_ZN16QXmlStreamWriter17writeEmptyElementERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QXmlStreamWriter::writeDTD(const QString & dtd);
  fn C_ZN16QXmlStreamWriter8writeDTDERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QXmlStreamWriter::setAutoFormattingIndent(int spacesOrTabs);
  fn C_ZN16QXmlStreamWriter23setAutoFormattingIndentEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QXmlStreamWriter::writeAttribute(const QXmlStreamAttribute & attribute);
  fn C_ZN16QXmlStreamWriter14writeAttributeERK19QXmlStreamAttribute(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QXmlStreamWriter::writeStartElement(const QString & namespaceUri, const QString & name);
  fn C_ZN16QXmlStreamWriter17writeStartElementERK7QStringS2_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QXmlStreamWriter::QXmlStreamWriter(QString * string);
  fn C_ZN16QXmlStreamWriterC2EP7QString(arg0: *mut c_void) -> u64;
  // proto:  void QXmlStreamWriter::writeComment(const QString & text);
  fn C_ZN16QXmlStreamWriter12writeCommentERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QTextCodec * QXmlStreamWriter::codec();
  fn C_ZNK16QXmlStreamWriter5codecEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QXmlStreamWriter::writeAttribute(const QString & namespaceUri, const QString & name, const QString & value);
  fn C_ZN16QXmlStreamWriter14writeAttributeERK7QStringS2_S2_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void);
  // proto:  void QXmlStreamWriter::writeNamespace(const QString & namespaceUri, const QString & prefix);
  fn C_ZN16QXmlStreamWriter14writeNamespaceERK7QStringS2_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  bool QXmlStreamWriter::hasError();
  fn C_ZNK16QXmlStreamWriter8hasErrorEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QXmlStreamWriter::writeCDATA(const QString & text);
  fn C_ZN16QXmlStreamWriter10writeCDATAERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QXmlStreamWriter::writeStartDocument();
  fn C_ZN16QXmlStreamWriter18writeStartDocumentEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QXmlStreamWriter::writeEntityReference(const QString & name);
  fn C_ZN16QXmlStreamWriter20writeEntityReferenceERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QXmlStreamWriter::setAutoFormatting(bool );
  fn C_ZN16QXmlStreamWriter17setAutoFormattingEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QXmlStreamWriter::setCodec(const char * codecName);
  fn C_ZN16QXmlStreamWriter8setCodecEPKc(qthis: u64 /* *mut c_void*/, arg0: *mut c_char);
  // proto:  int QXmlStreamWriter::autoFormattingIndent();
  fn C_ZNK16QXmlStreamWriter20autoFormattingIndentEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QXmlStreamWriter::~QXmlStreamWriter();
  fn C_ZN16QXmlStreamWriterD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QXmlStreamWriter::writeAttributes(const QXmlStreamAttributes & attributes);
  fn C_ZN16QXmlStreamWriter15writeAttributesERK20QXmlStreamAttributes(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QXmlStreamWriter::writeDefaultNamespace(const QString & namespaceUri);
  fn C_ZN16QXmlStreamWriter21writeDefaultNamespaceERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QIODevice * QXmlStreamWriter::device();
  fn C_ZNK16QXmlStreamWriter6deviceEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QXmlStreamWriter::writeCurrentToken(const QXmlStreamReader & reader);
  fn C_ZN16QXmlStreamWriter17writeCurrentTokenERK16QXmlStreamReader(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QXmlStreamWriter::QXmlStreamWriter(QByteArray * array);
  fn C_ZN16QXmlStreamWriterC2EP10QByteArray(arg0: *mut c_void) -> u64;
  // proto:  void QXmlStreamWriter::writeTextElement(const QString & qualifiedName, const QString & text);
  fn C_ZN16QXmlStreamWriter16writeTextElementERK7QStringS2_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QXmlStreamWriter::writeEmptyElement(const QString & namespaceUri, const QString & name);
  fn C_ZN16QXmlStreamWriter17writeEmptyElementERK7QStringS2_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QXmlStreamWriter::QXmlStreamWriter(QIODevice * device);
  fn C_ZN16QXmlStreamWriterC2EP9QIODevice(arg0: *mut c_void) -> u64;
  // proto:  void QXmlStreamWriter::writeStartElement(const QString & qualifiedName);
  fn C_ZN16QXmlStreamWriter17writeStartElementERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  fn QXmlStreamNotationDeclaration_Class_Size() -> c_int;
  // proto:  void QXmlStreamNotationDeclaration::QXmlStreamNotationDeclaration(const QXmlStreamNotationDeclaration & );
  fn C_ZN29QXmlStreamNotationDeclarationC2ERKS_(arg0: *mut c_void) -> u64;
  // proto:  QStringRef QXmlStreamNotationDeclaration::publicId();
  fn C_ZNK29QXmlStreamNotationDeclaration8publicIdEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QStringRef QXmlStreamNotationDeclaration::name();
  fn C_ZNK29QXmlStreamNotationDeclaration4nameEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QXmlStreamNotationDeclaration::~QXmlStreamNotationDeclaration();
  fn C_ZN29QXmlStreamNotationDeclarationD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QStringRef QXmlStreamNotationDeclaration::systemId();
  fn C_ZNK29QXmlStreamNotationDeclaration8systemIdEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QXmlStreamNotationDeclaration::QXmlStreamNotationDeclaration();
  fn C_ZN29QXmlStreamNotationDeclarationC2Ev() -> u64;
  fn QXmlStreamAttribute_Class_Size() -> c_int;
  // proto:  void QXmlStreamAttribute::QXmlStreamAttribute(const QString & qualifiedName, const QString & value);
  fn C_ZN19QXmlStreamAttributeC2ERK7QStringS2_(arg0: *mut c_void, arg1: *mut c_void) -> u64;
  // proto:  QStringRef QXmlStreamAttribute::qualifiedName();
  fn C_ZNK19QXmlStreamAttribute13qualifiedNameEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QXmlStreamAttribute::~QXmlStreamAttribute();
  fn C_ZN19QXmlStreamAttributeD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QStringRef QXmlStreamAttribute::value();
  fn C_ZNK19QXmlStreamAttribute5valueEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QStringRef QXmlStreamAttribute::namespaceUri();
  fn C_ZNK19QXmlStreamAttribute12namespaceUriEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QXmlStreamAttribute::QXmlStreamAttribute();
  fn C_ZN19QXmlStreamAttributeC2Ev() -> u64;
  // proto:  void QXmlStreamAttribute::QXmlStreamAttribute(const QXmlStreamAttribute & );
  fn C_ZN19QXmlStreamAttributeC2ERKS_(arg0: *mut c_void) -> u64;
  // proto:  void QXmlStreamAttribute::QXmlStreamAttribute(const QString & namespaceUri, const QString & name, const QString & value);
  fn C_ZN19QXmlStreamAttributeC2ERK7QStringS2_S2_(arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) -> u64;
  // proto:  bool QXmlStreamAttribute::isDefault();
  fn C_ZNK19QXmlStreamAttribute9isDefaultEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QStringRef QXmlStreamAttribute::prefix();
  fn C_ZNK19QXmlStreamAttribute6prefixEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QStringRef QXmlStreamAttribute::name();
  fn C_ZNK19QXmlStreamAttribute4nameEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
} // <= ext block end

// body block begin =>
// class sizeof(QXmlStreamStringRef)=16
#[derive(Default)]
pub struct QXmlStreamStringRef {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QXmlStreamReader)=1
#[derive(Default)]
pub struct QXmlStreamReader {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QXmlStreamEntityResolver)=8
#[derive(Default)]
pub struct QXmlStreamEntityResolver {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QXmlStreamNamespaceDeclaration)=40
#[derive(Default)]
pub struct QXmlStreamNamespaceDeclaration {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QXmlStreamEntityDeclaration)=88
#[derive(Default)]
pub struct QXmlStreamEntityDeclaration {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QXmlStreamAttributes)=1
#[derive(Default)]
pub struct QXmlStreamAttributes {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QXmlStreamWriter)=1
#[derive(Default)]
pub struct QXmlStreamWriter {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QXmlStreamNotationDeclaration)=56
#[derive(Default)]
pub struct QXmlStreamNotationDeclaration {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QXmlStreamAttribute)=80
#[derive(Default)]
pub struct QXmlStreamAttribute {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QXmlStreamStringRef {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QXmlStreamStringRef {
    return QXmlStreamStringRef{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  int QXmlStreamStringRef::size();
impl /*struct*/ QXmlStreamStringRef {
  pub fn size<RetType, T: QXmlStreamStringRef_size<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.size(self);
    // return 1;
  }
}

pub trait QXmlStreamStringRef_size<RetType> {
  fn size(self , rsthis: & QXmlStreamStringRef) -> RetType;
}

  // proto:  int QXmlStreamStringRef::size();
impl<'a> /*trait*/ QXmlStreamStringRef_size<i32> for () {
  fn size(self , rsthis: & QXmlStreamStringRef) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QXmlStreamStringRef4sizeEv()};
    let mut ret = unsafe {C_ZNK19QXmlStreamStringRef4sizeEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  void QXmlStreamStringRef::clear();
impl /*struct*/ QXmlStreamStringRef {
  pub fn clear<RetType, T: QXmlStreamStringRef_clear<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clear(self);
    // return 1;
  }
}

pub trait QXmlStreamStringRef_clear<RetType> {
  fn clear(self , rsthis: & QXmlStreamStringRef) -> RetType;
}

  // proto:  void QXmlStreamStringRef::clear();
impl<'a> /*trait*/ QXmlStreamStringRef_clear<()> for () {
  fn clear(self , rsthis: & QXmlStreamStringRef) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QXmlStreamStringRef5clearEv()};
     unsafe {C_ZN19QXmlStreamStringRef5clearEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  const QString * QXmlStreamStringRef::string();
impl /*struct*/ QXmlStreamStringRef {
  pub fn string<RetType, T: QXmlStreamStringRef_string<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.string(self);
    // return 1;
  }
}

pub trait QXmlStreamStringRef_string<RetType> {
  fn string(self , rsthis: & QXmlStreamStringRef) -> RetType;
}

  // proto:  const QString * QXmlStreamStringRef::string();
impl<'a> /*trait*/ QXmlStreamStringRef_string<QString> for () {
  fn string(self , rsthis: & QXmlStreamStringRef) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QXmlStreamStringRef6stringEv()};
    let mut ret = unsafe {C_ZNK19QXmlStreamStringRef6stringEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QXmlStreamStringRef::QXmlStreamStringRef(const QString & aString);
impl /*struct*/ QXmlStreamStringRef {
  pub fn new<T: QXmlStreamStringRef_new>(value: T) -> QXmlStreamStringRef {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QXmlStreamStringRef_new {
  fn new(self) -> QXmlStreamStringRef;
}

  // proto:  void QXmlStreamStringRef::QXmlStreamStringRef(const QString & aString);
impl<'a> /*trait*/ QXmlStreamStringRef_new for (&'a QString) {
  fn new(self) -> QXmlStreamStringRef {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QXmlStreamStringRefC2ERK7QString()};
    let ctysz: c_int = unsafe{QXmlStreamStringRef_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN19QXmlStreamStringRefC2ERK7QString(arg0)};
    let rsthis = QXmlStreamStringRef{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QXmlStreamStringRef::~QXmlStreamStringRef();
impl /*struct*/ QXmlStreamStringRef {
  pub fn free<RetType, T: QXmlStreamStringRef_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QXmlStreamStringRef_free<RetType> {
  fn free(self , rsthis: & QXmlStreamStringRef) -> RetType;
}

  // proto:  void QXmlStreamStringRef::~QXmlStreamStringRef();
impl<'a> /*trait*/ QXmlStreamStringRef_free<()> for () {
  fn free(self , rsthis: & QXmlStreamStringRef) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QXmlStreamStringRefD2Ev()};
     unsafe {C_ZN19QXmlStreamStringRefD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QXmlStreamStringRef::QXmlStreamStringRef();
impl<'a> /*trait*/ QXmlStreamStringRef_new for () {
  fn new(self) -> QXmlStreamStringRef {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QXmlStreamStringRefC2Ev()};
    let ctysz: c_int = unsafe{QXmlStreamStringRef_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let qthis: u64 = unsafe {C_ZN19QXmlStreamStringRefC2Ev()};
    let rsthis = QXmlStreamStringRef{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  int QXmlStreamStringRef::position();
impl /*struct*/ QXmlStreamStringRef {
  pub fn position<RetType, T: QXmlStreamStringRef_position<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.position(self);
    // return 1;
  }
}

pub trait QXmlStreamStringRef_position<RetType> {
  fn position(self , rsthis: & QXmlStreamStringRef) -> RetType;
}

  // proto:  int QXmlStreamStringRef::position();
impl<'a> /*trait*/ QXmlStreamStringRef_position<i32> for () {
  fn position(self , rsthis: & QXmlStreamStringRef) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QXmlStreamStringRef8positionEv()};
    let mut ret = unsafe {C_ZNK19QXmlStreamStringRef8positionEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

impl /*struct*/ QXmlStreamReader {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QXmlStreamReader {
    return QXmlStreamReader{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  QStringRef QXmlStreamReader::name();
impl /*struct*/ QXmlStreamReader {
  pub fn name<RetType, T: QXmlStreamReader_name<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.name(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_name<RetType> {
  fn name(self , rsthis: & QXmlStreamReader) -> RetType;
}

  // proto:  QStringRef QXmlStreamReader::name();
impl<'a> /*trait*/ QXmlStreamReader_name<QStringRef> for () {
  fn name(self , rsthis: & QXmlStreamReader) -> QStringRef {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader4nameEv()};
    let mut ret = unsafe {C_ZNK16QXmlStreamReader4nameEv(rsthis.qclsinst)};
    let mut ret1 = QStringRef::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QXmlStreamEntityResolver * QXmlStreamReader::entityResolver();
impl /*struct*/ QXmlStreamReader {
  pub fn entityResolver<RetType, T: QXmlStreamReader_entityResolver<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.entityResolver(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_entityResolver<RetType> {
  fn entityResolver(self , rsthis: & QXmlStreamReader) -> RetType;
}

  // proto:  QXmlStreamEntityResolver * QXmlStreamReader::entityResolver();
impl<'a> /*trait*/ QXmlStreamReader_entityResolver<QXmlStreamEntityResolver> for () {
  fn entityResolver(self , rsthis: & QXmlStreamReader) -> QXmlStreamEntityResolver {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader14entityResolverEv()};
    let mut ret = unsafe {C_ZNK16QXmlStreamReader14entityResolverEv(rsthis.qclsinst)};
    let mut ret1 = QXmlStreamEntityResolver::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QXmlStreamReader::namespaceProcessing();
impl /*struct*/ QXmlStreamReader {
  pub fn namespaceProcessing<RetType, T: QXmlStreamReader_namespaceProcessing<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.namespaceProcessing(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_namespaceProcessing<RetType> {
  fn namespaceProcessing(self , rsthis: & QXmlStreamReader) -> RetType;
}

  // proto:  bool QXmlStreamReader::namespaceProcessing();
impl<'a> /*trait*/ QXmlStreamReader_namespaceProcessing<i8> for () {
  fn namespaceProcessing(self , rsthis: & QXmlStreamReader) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader19namespaceProcessingEv()};
    let mut ret = unsafe {C_ZNK16QXmlStreamReader19namespaceProcessingEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  bool QXmlStreamReader::isStartElement();
impl /*struct*/ QXmlStreamReader {
  pub fn isStartElement<RetType, T: QXmlStreamReader_isStartElement<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isStartElement(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_isStartElement<RetType> {
  fn isStartElement(self , rsthis: & QXmlStreamReader) -> RetType;
}

  // proto:  bool QXmlStreamReader::isStartElement();
impl<'a> /*trait*/ QXmlStreamReader_isStartElement<i8> for () {
  fn isStartElement(self , rsthis: & QXmlStreamReader) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader14isStartElementEv()};
    let mut ret = unsafe {C_ZNK16QXmlStreamReader14isStartElementEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  bool QXmlStreamReader::isStandaloneDocument();
impl /*struct*/ QXmlStreamReader {
  pub fn isStandaloneDocument<RetType, T: QXmlStreamReader_isStandaloneDocument<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isStandaloneDocument(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_isStandaloneDocument<RetType> {
  fn isStandaloneDocument(self , rsthis: & QXmlStreamReader) -> RetType;
}

  // proto:  bool QXmlStreamReader::isStandaloneDocument();
impl<'a> /*trait*/ QXmlStreamReader_isStandaloneDocument<i8> for () {
  fn isStandaloneDocument(self , rsthis: & QXmlStreamReader) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader20isStandaloneDocumentEv()};
    let mut ret = unsafe {C_ZNK16QXmlStreamReader20isStandaloneDocumentEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  qint64 QXmlStreamReader::lineNumber();
impl /*struct*/ QXmlStreamReader {
  pub fn lineNumber<RetType, T: QXmlStreamReader_lineNumber<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.lineNumber(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_lineNumber<RetType> {
  fn lineNumber(self , rsthis: & QXmlStreamReader) -> RetType;
}

  // proto:  qint64 QXmlStreamReader::lineNumber();
impl<'a> /*trait*/ QXmlStreamReader_lineNumber<i64> for () {
  fn lineNumber(self , rsthis: & QXmlStreamReader) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader10lineNumberEv()};
    let mut ret = unsafe {C_ZNK16QXmlStreamReader10lineNumberEv(rsthis.qclsinst)};
    return ret as i64; // 1
    // return 1;
  }
}

  // proto:  void QXmlStreamReader::clear();
impl /*struct*/ QXmlStreamReader {
  pub fn clear<RetType, T: QXmlStreamReader_clear<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clear(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_clear<RetType> {
  fn clear(self , rsthis: & QXmlStreamReader) -> RetType;
}

  // proto:  void QXmlStreamReader::clear();
impl<'a> /*trait*/ QXmlStreamReader_clear<()> for () {
  fn clear(self , rsthis: & QXmlStreamReader) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamReader5clearEv()};
     unsafe {C_ZN16QXmlStreamReader5clearEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QStringRef QXmlStreamReader::processingInstructionData();
impl /*struct*/ QXmlStreamReader {
  pub fn processingInstructionData<RetType, T: QXmlStreamReader_processingInstructionData<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.processingInstructionData(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_processingInstructionData<RetType> {
  fn processingInstructionData(self , rsthis: & QXmlStreamReader) -> RetType;
}

  // proto:  QStringRef QXmlStreamReader::processingInstructionData();
impl<'a> /*trait*/ QXmlStreamReader_processingInstructionData<QStringRef> for () {
  fn processingInstructionData(self , rsthis: & QXmlStreamReader) -> QStringRef {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader25processingInstructionDataEv()};
    let mut ret = unsafe {C_ZNK16QXmlStreamReader25processingInstructionDataEv(rsthis.qclsinst)};
    let mut ret1 = QStringRef::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QXmlStreamReader::addData(const QString & data);
impl /*struct*/ QXmlStreamReader {
  pub fn addData<RetType, T: QXmlStreamReader_addData<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.addData(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_addData<RetType> {
  fn addData(self , rsthis: & QXmlStreamReader) -> RetType;
}

  // proto:  void QXmlStreamReader::addData(const QString & data);
impl<'a> /*trait*/ QXmlStreamReader_addData<()> for (&'a QString) {
  fn addData(self , rsthis: & QXmlStreamReader) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamReader7addDataERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN16QXmlStreamReader7addDataERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QStringRef QXmlStreamReader::dtdPublicId();
impl /*struct*/ QXmlStreamReader {
  pub fn dtdPublicId<RetType, T: QXmlStreamReader_dtdPublicId<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.dtdPublicId(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_dtdPublicId<RetType> {
  fn dtdPublicId(self , rsthis: & QXmlStreamReader) -> RetType;
}

  // proto:  QStringRef QXmlStreamReader::dtdPublicId();
impl<'a> /*trait*/ QXmlStreamReader_dtdPublicId<QStringRef> for () {
  fn dtdPublicId(self , rsthis: & QXmlStreamReader) -> QStringRef {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader11dtdPublicIdEv()};
    let mut ret = unsafe {C_ZNK16QXmlStreamReader11dtdPublicIdEv(rsthis.qclsinst)};
    let mut ret1 = QStringRef::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QStringRef QXmlStreamReader::documentEncoding();
impl /*struct*/ QXmlStreamReader {
  pub fn documentEncoding<RetType, T: QXmlStreamReader_documentEncoding<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.documentEncoding(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_documentEncoding<RetType> {
  fn documentEncoding(self , rsthis: & QXmlStreamReader) -> RetType;
}

  // proto:  QStringRef QXmlStreamReader::documentEncoding();
impl<'a> /*trait*/ QXmlStreamReader_documentEncoding<QStringRef> for () {
  fn documentEncoding(self , rsthis: & QXmlStreamReader) -> QStringRef {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader16documentEncodingEv()};
    let mut ret = unsafe {C_ZNK16QXmlStreamReader16documentEncodingEv(rsthis.qclsinst)};
    let mut ret1 = QStringRef::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  qint64 QXmlStreamReader::characterOffset();
impl /*struct*/ QXmlStreamReader {
  pub fn characterOffset<RetType, T: QXmlStreamReader_characterOffset<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.characterOffset(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_characterOffset<RetType> {
  fn characterOffset(self , rsthis: & QXmlStreamReader) -> RetType;
}

  // proto:  qint64 QXmlStreamReader::characterOffset();
impl<'a> /*trait*/ QXmlStreamReader_characterOffset<i64> for () {
  fn characterOffset(self , rsthis: & QXmlStreamReader) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader15characterOffsetEv()};
    let mut ret = unsafe {C_ZNK16QXmlStreamReader15characterOffsetEv(rsthis.qclsinst)};
    return ret as i64; // 1
    // return 1;
  }
}

  // proto:  QXmlStreamAttributes QXmlStreamReader::attributes();
impl /*struct*/ QXmlStreamReader {
  pub fn attributes<RetType, T: QXmlStreamReader_attributes<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.attributes(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_attributes<RetType> {
  fn attributes(self , rsthis: & QXmlStreamReader) -> RetType;
}

  // proto:  QXmlStreamAttributes QXmlStreamReader::attributes();
impl<'a> /*trait*/ QXmlStreamReader_attributes<QXmlStreamAttributes> for () {
  fn attributes(self , rsthis: & QXmlStreamReader) -> QXmlStreamAttributes {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader10attributesEv()};
    let mut ret = unsafe {C_ZNK16QXmlStreamReader10attributesEv(rsthis.qclsinst)};
    let mut ret1 = QXmlStreamAttributes::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QXmlStreamReader::tokenString();
impl /*struct*/ QXmlStreamReader {
  pub fn tokenString<RetType, T: QXmlStreamReader_tokenString<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.tokenString(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_tokenString<RetType> {
  fn tokenString(self , rsthis: & QXmlStreamReader) -> RetType;
}

  // proto:  QString QXmlStreamReader::tokenString();
impl<'a> /*trait*/ QXmlStreamReader_tokenString<QString> for () {
  fn tokenString(self , rsthis: & QXmlStreamReader) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader11tokenStringEv()};
    let mut ret = unsafe {C_ZNK16QXmlStreamReader11tokenStringEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QXmlStreamReader::addExtraNamespaceDeclaration(const QXmlStreamNamespaceDeclaration & extraNamespaceDeclaraction);
impl /*struct*/ QXmlStreamReader {
  pub fn addExtraNamespaceDeclaration<RetType, T: QXmlStreamReader_addExtraNamespaceDeclaration<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.addExtraNamespaceDeclaration(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_addExtraNamespaceDeclaration<RetType> {
  fn addExtraNamespaceDeclaration(self , rsthis: & QXmlStreamReader) -> RetType;
}

  // proto:  void QXmlStreamReader::addExtraNamespaceDeclaration(const QXmlStreamNamespaceDeclaration & extraNamespaceDeclaraction);
impl<'a> /*trait*/ QXmlStreamReader_addExtraNamespaceDeclaration<()> for (&'a QXmlStreamNamespaceDeclaration) {
  fn addExtraNamespaceDeclaration(self , rsthis: & QXmlStreamReader) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamReader28addExtraNamespaceDeclarationERK30QXmlStreamNamespaceDeclaration()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN16QXmlStreamReader28addExtraNamespaceDeclarationERK30QXmlStreamNamespaceDeclaration(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QXmlStreamReader::QXmlStreamReader(const QByteArray & data);
impl /*struct*/ QXmlStreamReader {
  pub fn new<T: QXmlStreamReader_new>(value: T) -> QXmlStreamReader {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QXmlStreamReader_new {
  fn new(self) -> QXmlStreamReader;
}

  // proto:  void QXmlStreamReader::QXmlStreamReader(const QByteArray & data);
impl<'a> /*trait*/ QXmlStreamReader_new for (&'a QByteArray) {
  fn new(self) -> QXmlStreamReader {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamReaderC2ERK10QByteArray()};
    let ctysz: c_int = unsafe{QXmlStreamReader_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN16QXmlStreamReaderC2ERK10QByteArray(arg0)};
    let rsthis = QXmlStreamReader{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QStringRef QXmlStreamReader::qualifiedName();
impl /*struct*/ QXmlStreamReader {
  pub fn qualifiedName<RetType, T: QXmlStreamReader_qualifiedName<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.qualifiedName(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_qualifiedName<RetType> {
  fn qualifiedName(self , rsthis: & QXmlStreamReader) -> RetType;
}

  // proto:  QStringRef QXmlStreamReader::qualifiedName();
impl<'a> /*trait*/ QXmlStreamReader_qualifiedName<QStringRef> for () {
  fn qualifiedName(self , rsthis: & QXmlStreamReader) -> QStringRef {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader13qualifiedNameEv()};
    let mut ret = unsafe {C_ZNK16QXmlStreamReader13qualifiedNameEv(rsthis.qclsinst)};
    let mut ret1 = QStringRef::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QIODevice * QXmlStreamReader::device();
impl /*struct*/ QXmlStreamReader {
  pub fn device<RetType, T: QXmlStreamReader_device<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.device(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_device<RetType> {
  fn device(self , rsthis: & QXmlStreamReader) -> RetType;
}

  // proto:  QIODevice * QXmlStreamReader::device();
impl<'a> /*trait*/ QXmlStreamReader_device<QIODevice> for () {
  fn device(self , rsthis: & QXmlStreamReader) -> QIODevice {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader6deviceEv()};
    let mut ret = unsafe {C_ZNK16QXmlStreamReader6deviceEv(rsthis.qclsinst)};
    let mut ret1 = QIODevice::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QStringRef QXmlStreamReader::namespaceUri();
impl /*struct*/ QXmlStreamReader {
  pub fn namespaceUri<RetType, T: QXmlStreamReader_namespaceUri<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.namespaceUri(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_namespaceUri<RetType> {
  fn namespaceUri(self , rsthis: & QXmlStreamReader) -> RetType;
}

  // proto:  QStringRef QXmlStreamReader::namespaceUri();
impl<'a> /*trait*/ QXmlStreamReader_namespaceUri<QStringRef> for () {
  fn namespaceUri(self , rsthis: & QXmlStreamReader) -> QStringRef {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader12namespaceUriEv()};
    let mut ret = unsafe {C_ZNK16QXmlStreamReader12namespaceUriEv(rsthis.qclsinst)};
    let mut ret1 = QStringRef::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QStringRef QXmlStreamReader::text();
impl /*struct*/ QXmlStreamReader {
  pub fn text<RetType, T: QXmlStreamReader_text<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.text(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_text<RetType> {
  fn text(self , rsthis: & QXmlStreamReader) -> RetType;
}

  // proto:  QStringRef QXmlStreamReader::text();
impl<'a> /*trait*/ QXmlStreamReader_text<QStringRef> for () {
  fn text(self , rsthis: & QXmlStreamReader) -> QStringRef {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader4textEv()};
    let mut ret = unsafe {C_ZNK16QXmlStreamReader4textEv(rsthis.qclsinst)};
    let mut ret1 = QStringRef::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QXmlStreamReader::setDevice(QIODevice * device);
impl /*struct*/ QXmlStreamReader {
  pub fn setDevice<RetType, T: QXmlStreamReader_setDevice<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setDevice(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_setDevice<RetType> {
  fn setDevice(self , rsthis: & QXmlStreamReader) -> RetType;
}

  // proto:  void QXmlStreamReader::setDevice(QIODevice * device);
impl<'a> /*trait*/ QXmlStreamReader_setDevice<()> for (&'a QIODevice) {
  fn setDevice(self , rsthis: & QXmlStreamReader) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamReader9setDeviceEP9QIODevice()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN16QXmlStreamReader9setDeviceEP9QIODevice(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QXmlStreamReader::QXmlStreamReader(QIODevice * device);
impl<'a> /*trait*/ QXmlStreamReader_new for (&'a QIODevice) {
  fn new(self) -> QXmlStreamReader {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamReaderC2EP9QIODevice()};
    let ctysz: c_int = unsafe{QXmlStreamReader_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN16QXmlStreamReaderC2EP9QIODevice(arg0)};
    let rsthis = QXmlStreamReader{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QStringRef QXmlStreamReader::documentVersion();
impl /*struct*/ QXmlStreamReader {
  pub fn documentVersion<RetType, T: QXmlStreamReader_documentVersion<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.documentVersion(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_documentVersion<RetType> {
  fn documentVersion(self , rsthis: & QXmlStreamReader) -> RetType;
}

  // proto:  QStringRef QXmlStreamReader::documentVersion();
impl<'a> /*trait*/ QXmlStreamReader_documentVersion<QStringRef> for () {
  fn documentVersion(self , rsthis: & QXmlStreamReader) -> QStringRef {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader15documentVersionEv()};
    let mut ret = unsafe {C_ZNK16QXmlStreamReader15documentVersionEv(rsthis.qclsinst)};
    let mut ret1 = QStringRef::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QXmlStreamReader::isDTD();
impl /*struct*/ QXmlStreamReader {
  pub fn isDTD<RetType, T: QXmlStreamReader_isDTD<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isDTD(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_isDTD<RetType> {
  fn isDTD(self , rsthis: & QXmlStreamReader) -> RetType;
}

  // proto:  bool QXmlStreamReader::isDTD();
impl<'a> /*trait*/ QXmlStreamReader_isDTD<i8> for () {
  fn isDTD(self , rsthis: & QXmlStreamReader) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader5isDTDEv()};
    let mut ret = unsafe {C_ZNK16QXmlStreamReader5isDTDEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  bool QXmlStreamReader::isStartDocument();
impl /*struct*/ QXmlStreamReader {
  pub fn isStartDocument<RetType, T: QXmlStreamReader_isStartDocument<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isStartDocument(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_isStartDocument<RetType> {
  fn isStartDocument(self , rsthis: & QXmlStreamReader) -> RetType;
}

  // proto:  bool QXmlStreamReader::isStartDocument();
impl<'a> /*trait*/ QXmlStreamReader_isStartDocument<i8> for () {
  fn isStartDocument(self , rsthis: & QXmlStreamReader) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader15isStartDocumentEv()};
    let mut ret = unsafe {C_ZNK16QXmlStreamReader15isStartDocumentEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  QString QXmlStreamReader::errorString();
impl /*struct*/ QXmlStreamReader {
  pub fn errorString<RetType, T: QXmlStreamReader_errorString<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.errorString(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_errorString<RetType> {
  fn errorString(self , rsthis: & QXmlStreamReader) -> RetType;
}

  // proto:  QString QXmlStreamReader::errorString();
impl<'a> /*trait*/ QXmlStreamReader_errorString<QString> for () {
  fn errorString(self , rsthis: & QXmlStreamReader) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader11errorStringEv()};
    let mut ret = unsafe {C_ZNK16QXmlStreamReader11errorStringEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QXmlStreamReader::isProcessingInstruction();
impl /*struct*/ QXmlStreamReader {
  pub fn isProcessingInstruction<RetType, T: QXmlStreamReader_isProcessingInstruction<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isProcessingInstruction(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_isProcessingInstruction<RetType> {
  fn isProcessingInstruction(self , rsthis: & QXmlStreamReader) -> RetType;
}

  // proto:  bool QXmlStreamReader::isProcessingInstruction();
impl<'a> /*trait*/ QXmlStreamReader_isProcessingInstruction<i8> for () {
  fn isProcessingInstruction(self , rsthis: & QXmlStreamReader) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader23isProcessingInstructionEv()};
    let mut ret = unsafe {C_ZNK16QXmlStreamReader23isProcessingInstructionEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QXmlStreamReader::setEntityResolver(QXmlStreamEntityResolver * resolver);
impl /*struct*/ QXmlStreamReader {
  pub fn setEntityResolver<RetType, T: QXmlStreamReader_setEntityResolver<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setEntityResolver(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_setEntityResolver<RetType> {
  fn setEntityResolver(self , rsthis: & QXmlStreamReader) -> RetType;
}

  // proto:  void QXmlStreamReader::setEntityResolver(QXmlStreamEntityResolver * resolver);
impl<'a> /*trait*/ QXmlStreamReader_setEntityResolver<()> for (&'a QXmlStreamEntityResolver) {
  fn setEntityResolver(self , rsthis: & QXmlStreamReader) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamReader17setEntityResolverEP24QXmlStreamEntityResolver()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN16QXmlStreamReader17setEntityResolverEP24QXmlStreamEntityResolver(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QXmlStreamReader::isCharacters();
impl /*struct*/ QXmlStreamReader {
  pub fn isCharacters<RetType, T: QXmlStreamReader_isCharacters<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isCharacters(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_isCharacters<RetType> {
  fn isCharacters(self , rsthis: & QXmlStreamReader) -> RetType;
}

  // proto:  bool QXmlStreamReader::isCharacters();
impl<'a> /*trait*/ QXmlStreamReader_isCharacters<i8> for () {
  fn isCharacters(self , rsthis: & QXmlStreamReader) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader12isCharactersEv()};
    let mut ret = unsafe {C_ZNK16QXmlStreamReader12isCharactersEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QXmlStreamReader::QXmlStreamReader();
impl<'a> /*trait*/ QXmlStreamReader_new for () {
  fn new(self) -> QXmlStreamReader {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamReaderC2Ev()};
    let ctysz: c_int = unsafe{QXmlStreamReader_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let qthis: u64 = unsafe {C_ZN16QXmlStreamReaderC2Ev()};
    let rsthis = QXmlStreamReader{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QXmlStreamReader::QXmlStreamReader(const QString & data);
impl<'a> /*trait*/ QXmlStreamReader_new for (&'a QString) {
  fn new(self) -> QXmlStreamReader {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamReaderC2ERK7QString()};
    let ctysz: c_int = unsafe{QXmlStreamReader_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN16QXmlStreamReaderC2ERK7QString(arg0)};
    let rsthis = QXmlStreamReader{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QXmlStreamEntityDeclarations QXmlStreamReader::entityDeclarations();
impl /*struct*/ QXmlStreamReader {
  pub fn entityDeclarations<RetType, T: QXmlStreamReader_entityDeclarations<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.entityDeclarations(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_entityDeclarations<RetType> {
  fn entityDeclarations(self , rsthis: & QXmlStreamReader) -> RetType;
}

  // proto:  QXmlStreamEntityDeclarations QXmlStreamReader::entityDeclarations();
impl<'a> /*trait*/ QXmlStreamReader_entityDeclarations<u64> for () {
  fn entityDeclarations(self , rsthis: & QXmlStreamReader) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader18entityDeclarationsEv()};
    let mut ret = unsafe {C_ZNK16QXmlStreamReader18entityDeclarationsEv(rsthis.qclsinst)};
    return ret as u64; // 5
    // return 1;
  }
}

  // proto:  bool QXmlStreamReader::isWhitespace();
impl /*struct*/ QXmlStreamReader {
  pub fn isWhitespace<RetType, T: QXmlStreamReader_isWhitespace<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isWhitespace(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_isWhitespace<RetType> {
  fn isWhitespace(self , rsthis: & QXmlStreamReader) -> RetType;
}

  // proto:  bool QXmlStreamReader::isWhitespace();
impl<'a> /*trait*/ QXmlStreamReader_isWhitespace<i8> for () {
  fn isWhitespace(self , rsthis: & QXmlStreamReader) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader12isWhitespaceEv()};
    let mut ret = unsafe {C_ZNK16QXmlStreamReader12isWhitespaceEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  qint64 QXmlStreamReader::columnNumber();
impl /*struct*/ QXmlStreamReader {
  pub fn columnNumber<RetType, T: QXmlStreamReader_columnNumber<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.columnNumber(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_columnNumber<RetType> {
  fn columnNumber(self , rsthis: & QXmlStreamReader) -> RetType;
}

  // proto:  qint64 QXmlStreamReader::columnNumber();
impl<'a> /*trait*/ QXmlStreamReader_columnNumber<i64> for () {
  fn columnNumber(self , rsthis: & QXmlStreamReader) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader12columnNumberEv()};
    let mut ret = unsafe {C_ZNK16QXmlStreamReader12columnNumberEv(rsthis.qclsinst)};
    return ret as i64; // 1
    // return 1;
  }
}

  // proto:  bool QXmlStreamReader::hasError();
impl /*struct*/ QXmlStreamReader {
  pub fn hasError<RetType, T: QXmlStreamReader_hasError<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hasError(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_hasError<RetType> {
  fn hasError(self , rsthis: & QXmlStreamReader) -> RetType;
}

  // proto:  bool QXmlStreamReader::hasError();
impl<'a> /*trait*/ QXmlStreamReader_hasError<i8> for () {
  fn hasError(self , rsthis: & QXmlStreamReader) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader8hasErrorEv()};
    let mut ret = unsafe {C_ZNK16QXmlStreamReader8hasErrorEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  bool QXmlStreamReader::isCDATA();
impl /*struct*/ QXmlStreamReader {
  pub fn isCDATA<RetType, T: QXmlStreamReader_isCDATA<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isCDATA(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_isCDATA<RetType> {
  fn isCDATA(self , rsthis: & QXmlStreamReader) -> RetType;
}

  // proto:  bool QXmlStreamReader::isCDATA();
impl<'a> /*trait*/ QXmlStreamReader_isCDATA<i8> for () {
  fn isCDATA(self , rsthis: & QXmlStreamReader) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader7isCDATAEv()};
    let mut ret = unsafe {C_ZNK16QXmlStreamReader7isCDATAEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QXmlStreamReader::~QXmlStreamReader();
impl /*struct*/ QXmlStreamReader {
  pub fn free<RetType, T: QXmlStreamReader_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_free<RetType> {
  fn free(self , rsthis: & QXmlStreamReader) -> RetType;
}

  // proto:  void QXmlStreamReader::~QXmlStreamReader();
impl<'a> /*trait*/ QXmlStreamReader_free<()> for () {
  fn free(self , rsthis: & QXmlStreamReader) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamReaderD2Ev()};
     unsafe {C_ZN16QXmlStreamReaderD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QStringRef QXmlStreamReader::processingInstructionTarget();
impl /*struct*/ QXmlStreamReader {
  pub fn processingInstructionTarget<RetType, T: QXmlStreamReader_processingInstructionTarget<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.processingInstructionTarget(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_processingInstructionTarget<RetType> {
  fn processingInstructionTarget(self , rsthis: & QXmlStreamReader) -> RetType;
}

  // proto:  QStringRef QXmlStreamReader::processingInstructionTarget();
impl<'a> /*trait*/ QXmlStreamReader_processingInstructionTarget<QStringRef> for () {
  fn processingInstructionTarget(self , rsthis: & QXmlStreamReader) -> QStringRef {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader27processingInstructionTargetEv()};
    let mut ret = unsafe {C_ZNK16QXmlStreamReader27processingInstructionTargetEv(rsthis.qclsinst)};
    let mut ret1 = QStringRef::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QXmlStreamReader::addData(const char * data);
impl<'a> /*trait*/ QXmlStreamReader_addData<()> for (&'a  String) {
  fn addData(self , rsthis: & QXmlStreamReader) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamReader7addDataEPKc()};
    let arg0 = self.as_ptr()  as *mut c_char;
     unsafe {C_ZN16QXmlStreamReader7addDataEPKc(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QStringRef QXmlStreamReader::dtdSystemId();
impl /*struct*/ QXmlStreamReader {
  pub fn dtdSystemId<RetType, T: QXmlStreamReader_dtdSystemId<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.dtdSystemId(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_dtdSystemId<RetType> {
  fn dtdSystemId(self , rsthis: & QXmlStreamReader) -> RetType;
}

  // proto:  QStringRef QXmlStreamReader::dtdSystemId();
impl<'a> /*trait*/ QXmlStreamReader_dtdSystemId<QStringRef> for () {
  fn dtdSystemId(self , rsthis: & QXmlStreamReader) -> QStringRef {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader11dtdSystemIdEv()};
    let mut ret = unsafe {C_ZNK16QXmlStreamReader11dtdSystemIdEv(rsthis.qclsinst)};
    let mut ret1 = QStringRef::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QStringRef QXmlStreamReader::prefix();
impl /*struct*/ QXmlStreamReader {
  pub fn prefix<RetType, T: QXmlStreamReader_prefix<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.prefix(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_prefix<RetType> {
  fn prefix(self , rsthis: & QXmlStreamReader) -> RetType;
}

  // proto:  QStringRef QXmlStreamReader::prefix();
impl<'a> /*trait*/ QXmlStreamReader_prefix<QStringRef> for () {
  fn prefix(self , rsthis: & QXmlStreamReader) -> QStringRef {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader6prefixEv()};
    let mut ret = unsafe {C_ZNK16QXmlStreamReader6prefixEv(rsthis.qclsinst)};
    let mut ret1 = QStringRef::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QXmlStreamReader::isEndElement();
impl /*struct*/ QXmlStreamReader {
  pub fn isEndElement<RetType, T: QXmlStreamReader_isEndElement<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isEndElement(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_isEndElement<RetType> {
  fn isEndElement(self , rsthis: & QXmlStreamReader) -> RetType;
}

  // proto:  bool QXmlStreamReader::isEndElement();
impl<'a> /*trait*/ QXmlStreamReader_isEndElement<i8> for () {
  fn isEndElement(self , rsthis: & QXmlStreamReader) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader12isEndElementEv()};
    let mut ret = unsafe {C_ZNK16QXmlStreamReader12isEndElementEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  QXmlStreamNotationDeclarations QXmlStreamReader::notationDeclarations();
impl /*struct*/ QXmlStreamReader {
  pub fn notationDeclarations<RetType, T: QXmlStreamReader_notationDeclarations<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.notationDeclarations(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_notationDeclarations<RetType> {
  fn notationDeclarations(self , rsthis: & QXmlStreamReader) -> RetType;
}

  // proto:  QXmlStreamNotationDeclarations QXmlStreamReader::notationDeclarations();
impl<'a> /*trait*/ QXmlStreamReader_notationDeclarations<u64> for () {
  fn notationDeclarations(self , rsthis: & QXmlStreamReader) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader20notationDeclarationsEv()};
    let mut ret = unsafe {C_ZNK16QXmlStreamReader20notationDeclarationsEv(rsthis.qclsinst)};
    return ret as u64; // 5
    // return 1;
  }
}

  // proto:  QXmlStreamNamespaceDeclarations QXmlStreamReader::namespaceDeclarations();
impl /*struct*/ QXmlStreamReader {
  pub fn namespaceDeclarations<RetType, T: QXmlStreamReader_namespaceDeclarations<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.namespaceDeclarations(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_namespaceDeclarations<RetType> {
  fn namespaceDeclarations(self , rsthis: & QXmlStreamReader) -> RetType;
}

  // proto:  QXmlStreamNamespaceDeclarations QXmlStreamReader::namespaceDeclarations();
impl<'a> /*trait*/ QXmlStreamReader_namespaceDeclarations<u64> for () {
  fn namespaceDeclarations(self , rsthis: & QXmlStreamReader) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader21namespaceDeclarationsEv()};
    let mut ret = unsafe {C_ZNK16QXmlStreamReader21namespaceDeclarationsEv(rsthis.qclsinst)};
    return ret as u64; // 5
    // return 1;
  }
}

  // proto:  void QXmlStreamReader::setNamespaceProcessing(bool );
impl /*struct*/ QXmlStreamReader {
  pub fn setNamespaceProcessing<RetType, T: QXmlStreamReader_setNamespaceProcessing<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setNamespaceProcessing(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_setNamespaceProcessing<RetType> {
  fn setNamespaceProcessing(self , rsthis: & QXmlStreamReader) -> RetType;
}

  // proto:  void QXmlStreamReader::setNamespaceProcessing(bool );
impl<'a> /*trait*/ QXmlStreamReader_setNamespaceProcessing<()> for (i8) {
  fn setNamespaceProcessing(self , rsthis: & QXmlStreamReader) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamReader22setNamespaceProcessingEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN16QXmlStreamReader22setNamespaceProcessingEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QXmlStreamReader::raiseError(const QString & message);
impl /*struct*/ QXmlStreamReader {
  pub fn raiseError<RetType, T: QXmlStreamReader_raiseError<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.raiseError(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_raiseError<RetType> {
  fn raiseError(self , rsthis: & QXmlStreamReader) -> RetType;
}

  // proto:  void QXmlStreamReader::raiseError(const QString & message);
impl<'a> /*trait*/ QXmlStreamReader_raiseError<()> for (&'a QString) {
  fn raiseError(self , rsthis: & QXmlStreamReader) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamReader10raiseErrorERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN16QXmlStreamReader10raiseErrorERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QStringRef QXmlStreamReader::dtdName();
impl /*struct*/ QXmlStreamReader {
  pub fn dtdName<RetType, T: QXmlStreamReader_dtdName<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.dtdName(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_dtdName<RetType> {
  fn dtdName(self , rsthis: & QXmlStreamReader) -> RetType;
}

  // proto:  QStringRef QXmlStreamReader::dtdName();
impl<'a> /*trait*/ QXmlStreamReader_dtdName<QStringRef> for () {
  fn dtdName(self , rsthis: & QXmlStreamReader) -> QStringRef {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader7dtdNameEv()};
    let mut ret = unsafe {C_ZNK16QXmlStreamReader7dtdNameEv(rsthis.qclsinst)};
    let mut ret1 = QStringRef::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QXmlStreamReader::isEndDocument();
impl /*struct*/ QXmlStreamReader {
  pub fn isEndDocument<RetType, T: QXmlStreamReader_isEndDocument<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isEndDocument(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_isEndDocument<RetType> {
  fn isEndDocument(self , rsthis: & QXmlStreamReader) -> RetType;
}

  // proto:  bool QXmlStreamReader::isEndDocument();
impl<'a> /*trait*/ QXmlStreamReader_isEndDocument<i8> for () {
  fn isEndDocument(self , rsthis: & QXmlStreamReader) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader13isEndDocumentEv()};
    let mut ret = unsafe {C_ZNK16QXmlStreamReader13isEndDocumentEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  bool QXmlStreamReader::readNextStartElement();
impl /*struct*/ QXmlStreamReader {
  pub fn readNextStartElement<RetType, T: QXmlStreamReader_readNextStartElement<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.readNextStartElement(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_readNextStartElement<RetType> {
  fn readNextStartElement(self , rsthis: & QXmlStreamReader) -> RetType;
}

  // proto:  bool QXmlStreamReader::readNextStartElement();
impl<'a> /*trait*/ QXmlStreamReader_readNextStartElement<i8> for () {
  fn readNextStartElement(self , rsthis: & QXmlStreamReader) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamReader20readNextStartElementEv()};
    let mut ret = unsafe {C_ZN16QXmlStreamReader20readNextStartElementEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  bool QXmlStreamReader::isComment();
impl /*struct*/ QXmlStreamReader {
  pub fn isComment<RetType, T: QXmlStreamReader_isComment<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isComment(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_isComment<RetType> {
  fn isComment(self , rsthis: & QXmlStreamReader) -> RetType;
}

  // proto:  bool QXmlStreamReader::isComment();
impl<'a> /*trait*/ QXmlStreamReader_isComment<i8> for () {
  fn isComment(self , rsthis: & QXmlStreamReader) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader9isCommentEv()};
    let mut ret = unsafe {C_ZNK16QXmlStreamReader9isCommentEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QXmlStreamReader::QXmlStreamReader(const char * data);
impl<'a> /*trait*/ QXmlStreamReader_new for (&'a  String) {
  fn new(self) -> QXmlStreamReader {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamReaderC2EPKc()};
    let ctysz: c_int = unsafe{QXmlStreamReader_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.as_ptr()  as *mut c_char;
    let qthis: u64 = unsafe {C_ZN16QXmlStreamReaderC2EPKc(arg0)};
    let rsthis = QXmlStreamReader{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QXmlStreamReader::skipCurrentElement();
impl /*struct*/ QXmlStreamReader {
  pub fn skipCurrentElement<RetType, T: QXmlStreamReader_skipCurrentElement<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.skipCurrentElement(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_skipCurrentElement<RetType> {
  fn skipCurrentElement(self , rsthis: & QXmlStreamReader) -> RetType;
}

  // proto:  void QXmlStreamReader::skipCurrentElement();
impl<'a> /*trait*/ QXmlStreamReader_skipCurrentElement<()> for () {
  fn skipCurrentElement(self , rsthis: & QXmlStreamReader) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamReader18skipCurrentElementEv()};
     unsafe {C_ZN16QXmlStreamReader18skipCurrentElementEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QXmlStreamReader::isEntityReference();
impl /*struct*/ QXmlStreamReader {
  pub fn isEntityReference<RetType, T: QXmlStreamReader_isEntityReference<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isEntityReference(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_isEntityReference<RetType> {
  fn isEntityReference(self , rsthis: & QXmlStreamReader) -> RetType;
}

  // proto:  bool QXmlStreamReader::isEntityReference();
impl<'a> /*trait*/ QXmlStreamReader_isEntityReference<i8> for () {
  fn isEntityReference(self , rsthis: & QXmlStreamReader) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader17isEntityReferenceEv()};
    let mut ret = unsafe {C_ZNK16QXmlStreamReader17isEntityReferenceEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QXmlStreamReader::addData(const QByteArray & data);
impl<'a> /*trait*/ QXmlStreamReader_addData<()> for (&'a QByteArray) {
  fn addData(self , rsthis: & QXmlStreamReader) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamReader7addDataERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN16QXmlStreamReader7addDataERK10QByteArray(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QXmlStreamReader::atEnd();
impl /*struct*/ QXmlStreamReader {
  pub fn atEnd<RetType, T: QXmlStreamReader_atEnd<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.atEnd(self);
    // return 1;
  }
}

pub trait QXmlStreamReader_atEnd<RetType> {
  fn atEnd(self , rsthis: & QXmlStreamReader) -> RetType;
}

  // proto:  bool QXmlStreamReader::atEnd();
impl<'a> /*trait*/ QXmlStreamReader_atEnd<i8> for () {
  fn atEnd(self , rsthis: & QXmlStreamReader) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamReader5atEndEv()};
    let mut ret = unsafe {C_ZNK16QXmlStreamReader5atEndEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

impl /*struct*/ QXmlStreamEntityResolver {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QXmlStreamEntityResolver {
    return QXmlStreamEntityResolver{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  QString QXmlStreamEntityResolver::resolveEntity(const QString & publicId, const QString & systemId);
impl /*struct*/ QXmlStreamEntityResolver {
  pub fn resolveEntity<RetType, T: QXmlStreamEntityResolver_resolveEntity<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.resolveEntity(self);
    // return 1;
  }
}

pub trait QXmlStreamEntityResolver_resolveEntity<RetType> {
  fn resolveEntity(self , rsthis: & QXmlStreamEntityResolver) -> RetType;
}

  // proto:  QString QXmlStreamEntityResolver::resolveEntity(const QString & publicId, const QString & systemId);
impl<'a> /*trait*/ QXmlStreamEntityResolver_resolveEntity<QString> for (&'a QString, &'a QString) {
  fn resolveEntity(self , rsthis: & QXmlStreamEntityResolver) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QXmlStreamEntityResolver13resolveEntityERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN24QXmlStreamEntityResolver13resolveEntityERK7QStringS2_(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QXmlStreamEntityResolver::resolveUndeclaredEntity(const QString & name);
impl /*struct*/ QXmlStreamEntityResolver {
  pub fn resolveUndeclaredEntity<RetType, T: QXmlStreamEntityResolver_resolveUndeclaredEntity<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.resolveUndeclaredEntity(self);
    // return 1;
  }
}

pub trait QXmlStreamEntityResolver_resolveUndeclaredEntity<RetType> {
  fn resolveUndeclaredEntity(self , rsthis: & QXmlStreamEntityResolver) -> RetType;
}

  // proto:  QString QXmlStreamEntityResolver::resolveUndeclaredEntity(const QString & name);
impl<'a> /*trait*/ QXmlStreamEntityResolver_resolveUndeclaredEntity<QString> for (&'a QString) {
  fn resolveUndeclaredEntity(self , rsthis: & QXmlStreamEntityResolver) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QXmlStreamEntityResolver23resolveUndeclaredEntityERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN24QXmlStreamEntityResolver23resolveUndeclaredEntityERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QXmlStreamEntityResolver::~QXmlStreamEntityResolver();
impl /*struct*/ QXmlStreamEntityResolver {
  pub fn free<RetType, T: QXmlStreamEntityResolver_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QXmlStreamEntityResolver_free<RetType> {
  fn free(self , rsthis: & QXmlStreamEntityResolver) -> RetType;
}

  // proto:  void QXmlStreamEntityResolver::~QXmlStreamEntityResolver();
impl<'a> /*trait*/ QXmlStreamEntityResolver_free<()> for () {
  fn free(self , rsthis: & QXmlStreamEntityResolver) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QXmlStreamEntityResolverD2Ev()};
     unsafe {C_ZN24QXmlStreamEntityResolverD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QXmlStreamNamespaceDeclaration {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QXmlStreamNamespaceDeclaration {
    return QXmlStreamNamespaceDeclaration{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  QStringRef QXmlStreamNamespaceDeclaration::namespaceUri();
impl /*struct*/ QXmlStreamNamespaceDeclaration {
  pub fn namespaceUri<RetType, T: QXmlStreamNamespaceDeclaration_namespaceUri<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.namespaceUri(self);
    // return 1;
  }
}

pub trait QXmlStreamNamespaceDeclaration_namespaceUri<RetType> {
  fn namespaceUri(self , rsthis: & QXmlStreamNamespaceDeclaration) -> RetType;
}

  // proto:  QStringRef QXmlStreamNamespaceDeclaration::namespaceUri();
impl<'a> /*trait*/ QXmlStreamNamespaceDeclaration_namespaceUri<QStringRef> for () {
  fn namespaceUri(self , rsthis: & QXmlStreamNamespaceDeclaration) -> QStringRef {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZNK30QXmlStreamNamespaceDeclaration12namespaceUriEv()};
    let mut ret = unsafe {C_ZNK30QXmlStreamNamespaceDeclaration12namespaceUriEv(rsthis.qclsinst)};
    let mut ret1 = QStringRef::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QXmlStreamNamespaceDeclaration::QXmlStreamNamespaceDeclaration(const QXmlStreamNamespaceDeclaration & );
impl /*struct*/ QXmlStreamNamespaceDeclaration {
  pub fn new<T: QXmlStreamNamespaceDeclaration_new>(value: T) -> QXmlStreamNamespaceDeclaration {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QXmlStreamNamespaceDeclaration_new {
  fn new(self) -> QXmlStreamNamespaceDeclaration;
}

  // proto:  void QXmlStreamNamespaceDeclaration::QXmlStreamNamespaceDeclaration(const QXmlStreamNamespaceDeclaration & );
impl<'a> /*trait*/ QXmlStreamNamespaceDeclaration_new for (&'a QXmlStreamNamespaceDeclaration) {
  fn new(self) -> QXmlStreamNamespaceDeclaration {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZN30QXmlStreamNamespaceDeclarationC2ERKS_()};
    let ctysz: c_int = unsafe{QXmlStreamNamespaceDeclaration_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN30QXmlStreamNamespaceDeclarationC2ERKS_(arg0)};
    let rsthis = QXmlStreamNamespaceDeclaration{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QXmlStreamNamespaceDeclaration::QXmlStreamNamespaceDeclaration();
impl<'a> /*trait*/ QXmlStreamNamespaceDeclaration_new for () {
  fn new(self) -> QXmlStreamNamespaceDeclaration {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZN30QXmlStreamNamespaceDeclarationC2Ev()};
    let ctysz: c_int = unsafe{QXmlStreamNamespaceDeclaration_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let qthis: u64 = unsafe {C_ZN30QXmlStreamNamespaceDeclarationC2Ev()};
    let rsthis = QXmlStreamNamespaceDeclaration{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QStringRef QXmlStreamNamespaceDeclaration::prefix();
impl /*struct*/ QXmlStreamNamespaceDeclaration {
  pub fn prefix<RetType, T: QXmlStreamNamespaceDeclaration_prefix<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.prefix(self);
    // return 1;
  }
}

pub trait QXmlStreamNamespaceDeclaration_prefix<RetType> {
  fn prefix(self , rsthis: & QXmlStreamNamespaceDeclaration) -> RetType;
}

  // proto:  QStringRef QXmlStreamNamespaceDeclaration::prefix();
impl<'a> /*trait*/ QXmlStreamNamespaceDeclaration_prefix<QStringRef> for () {
  fn prefix(self , rsthis: & QXmlStreamNamespaceDeclaration) -> QStringRef {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZNK30QXmlStreamNamespaceDeclaration6prefixEv()};
    let mut ret = unsafe {C_ZNK30QXmlStreamNamespaceDeclaration6prefixEv(rsthis.qclsinst)};
    let mut ret1 = QStringRef::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QXmlStreamNamespaceDeclaration::QXmlStreamNamespaceDeclaration(const QString & prefix, const QString & namespaceUri);
impl<'a> /*trait*/ QXmlStreamNamespaceDeclaration_new for (&'a QString, &'a QString) {
  fn new(self) -> QXmlStreamNamespaceDeclaration {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZN30QXmlStreamNamespaceDeclarationC2ERK7QStringS2_()};
    let ctysz: c_int = unsafe{QXmlStreamNamespaceDeclaration_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN30QXmlStreamNamespaceDeclarationC2ERK7QStringS2_(arg0, arg1)};
    let rsthis = QXmlStreamNamespaceDeclaration{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QXmlStreamNamespaceDeclaration::~QXmlStreamNamespaceDeclaration();
impl /*struct*/ QXmlStreamNamespaceDeclaration {
  pub fn free<RetType, T: QXmlStreamNamespaceDeclaration_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QXmlStreamNamespaceDeclaration_free<RetType> {
  fn free(self , rsthis: & QXmlStreamNamespaceDeclaration) -> RetType;
}

  // proto:  void QXmlStreamNamespaceDeclaration::~QXmlStreamNamespaceDeclaration();
impl<'a> /*trait*/ QXmlStreamNamespaceDeclaration_free<()> for () {
  fn free(self , rsthis: & QXmlStreamNamespaceDeclaration) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZN30QXmlStreamNamespaceDeclarationD2Ev()};
     unsafe {C_ZN30QXmlStreamNamespaceDeclarationD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QXmlStreamEntityDeclaration {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QXmlStreamEntityDeclaration {
    return QXmlStreamEntityDeclaration{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void QXmlStreamEntityDeclaration::~QXmlStreamEntityDeclaration();
impl /*struct*/ QXmlStreamEntityDeclaration {
  pub fn free<RetType, T: QXmlStreamEntityDeclaration_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QXmlStreamEntityDeclaration_free<RetType> {
  fn free(self , rsthis: & QXmlStreamEntityDeclaration) -> RetType;
}

  // proto:  void QXmlStreamEntityDeclaration::~QXmlStreamEntityDeclaration();
impl<'a> /*trait*/ QXmlStreamEntityDeclaration_free<()> for () {
  fn free(self , rsthis: & QXmlStreamEntityDeclaration) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZN27QXmlStreamEntityDeclarationD2Ev()};
     unsafe {C_ZN27QXmlStreamEntityDeclarationD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QStringRef QXmlStreamEntityDeclaration::publicId();
impl /*struct*/ QXmlStreamEntityDeclaration {
  pub fn publicId<RetType, T: QXmlStreamEntityDeclaration_publicId<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.publicId(self);
    // return 1;
  }
}

pub trait QXmlStreamEntityDeclaration_publicId<RetType> {
  fn publicId(self , rsthis: & QXmlStreamEntityDeclaration) -> RetType;
}

  // proto:  QStringRef QXmlStreamEntityDeclaration::publicId();
impl<'a> /*trait*/ QXmlStreamEntityDeclaration_publicId<QStringRef> for () {
  fn publicId(self , rsthis: & QXmlStreamEntityDeclaration) -> QStringRef {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK27QXmlStreamEntityDeclaration8publicIdEv()};
    let mut ret = unsafe {C_ZNK27QXmlStreamEntityDeclaration8publicIdEv(rsthis.qclsinst)};
    let mut ret1 = QStringRef::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QStringRef QXmlStreamEntityDeclaration::name();
impl /*struct*/ QXmlStreamEntityDeclaration {
  pub fn name<RetType, T: QXmlStreamEntityDeclaration_name<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.name(self);
    // return 1;
  }
}

pub trait QXmlStreamEntityDeclaration_name<RetType> {
  fn name(self , rsthis: & QXmlStreamEntityDeclaration) -> RetType;
}

  // proto:  QStringRef QXmlStreamEntityDeclaration::name();
impl<'a> /*trait*/ QXmlStreamEntityDeclaration_name<QStringRef> for () {
  fn name(self , rsthis: & QXmlStreamEntityDeclaration) -> QStringRef {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK27QXmlStreamEntityDeclaration4nameEv()};
    let mut ret = unsafe {C_ZNK27QXmlStreamEntityDeclaration4nameEv(rsthis.qclsinst)};
    let mut ret1 = QStringRef::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QXmlStreamEntityDeclaration::QXmlStreamEntityDeclaration();
impl /*struct*/ QXmlStreamEntityDeclaration {
  pub fn new<T: QXmlStreamEntityDeclaration_new>(value: T) -> QXmlStreamEntityDeclaration {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QXmlStreamEntityDeclaration_new {
  fn new(self) -> QXmlStreamEntityDeclaration;
}

  // proto:  void QXmlStreamEntityDeclaration::QXmlStreamEntityDeclaration();
impl<'a> /*trait*/ QXmlStreamEntityDeclaration_new for () {
  fn new(self) -> QXmlStreamEntityDeclaration {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZN27QXmlStreamEntityDeclarationC2Ev()};
    let ctysz: c_int = unsafe{QXmlStreamEntityDeclaration_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let qthis: u64 = unsafe {C_ZN27QXmlStreamEntityDeclarationC2Ev()};
    let rsthis = QXmlStreamEntityDeclaration{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QStringRef QXmlStreamEntityDeclaration::value();
impl /*struct*/ QXmlStreamEntityDeclaration {
  pub fn value<RetType, T: QXmlStreamEntityDeclaration_value<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.value(self);
    // return 1;
  }
}

pub trait QXmlStreamEntityDeclaration_value<RetType> {
  fn value(self , rsthis: & QXmlStreamEntityDeclaration) -> RetType;
}

  // proto:  QStringRef QXmlStreamEntityDeclaration::value();
impl<'a> /*trait*/ QXmlStreamEntityDeclaration_value<QStringRef> for () {
  fn value(self , rsthis: & QXmlStreamEntityDeclaration) -> QStringRef {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK27QXmlStreamEntityDeclaration5valueEv()};
    let mut ret = unsafe {C_ZNK27QXmlStreamEntityDeclaration5valueEv(rsthis.qclsinst)};
    let mut ret1 = QStringRef::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QStringRef QXmlStreamEntityDeclaration::notationName();
impl /*struct*/ QXmlStreamEntityDeclaration {
  pub fn notationName<RetType, T: QXmlStreamEntityDeclaration_notationName<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.notationName(self);
    // return 1;
  }
}

pub trait QXmlStreamEntityDeclaration_notationName<RetType> {
  fn notationName(self , rsthis: & QXmlStreamEntityDeclaration) -> RetType;
}

  // proto:  QStringRef QXmlStreamEntityDeclaration::notationName();
impl<'a> /*trait*/ QXmlStreamEntityDeclaration_notationName<QStringRef> for () {
  fn notationName(self , rsthis: & QXmlStreamEntityDeclaration) -> QStringRef {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK27QXmlStreamEntityDeclaration12notationNameEv()};
    let mut ret = unsafe {C_ZNK27QXmlStreamEntityDeclaration12notationNameEv(rsthis.qclsinst)};
    let mut ret1 = QStringRef::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QXmlStreamEntityDeclaration::QXmlStreamEntityDeclaration(const QXmlStreamEntityDeclaration & );
impl<'a> /*trait*/ QXmlStreamEntityDeclaration_new for (&'a QXmlStreamEntityDeclaration) {
  fn new(self) -> QXmlStreamEntityDeclaration {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZN27QXmlStreamEntityDeclarationC2ERKS_()};
    let ctysz: c_int = unsafe{QXmlStreamEntityDeclaration_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN27QXmlStreamEntityDeclarationC2ERKS_(arg0)};
    let rsthis = QXmlStreamEntityDeclaration{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QStringRef QXmlStreamEntityDeclaration::systemId();
impl /*struct*/ QXmlStreamEntityDeclaration {
  pub fn systemId<RetType, T: QXmlStreamEntityDeclaration_systemId<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.systemId(self);
    // return 1;
  }
}

pub trait QXmlStreamEntityDeclaration_systemId<RetType> {
  fn systemId(self , rsthis: & QXmlStreamEntityDeclaration) -> RetType;
}

  // proto:  QStringRef QXmlStreamEntityDeclaration::systemId();
impl<'a> /*trait*/ QXmlStreamEntityDeclaration_systemId<QStringRef> for () {
  fn systemId(self , rsthis: & QXmlStreamEntityDeclaration) -> QStringRef {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK27QXmlStreamEntityDeclaration8systemIdEv()};
    let mut ret = unsafe {C_ZNK27QXmlStreamEntityDeclaration8systemIdEv(rsthis.qclsinst)};
    let mut ret1 = QStringRef::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QXmlStreamAttributes {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QXmlStreamAttributes {
    return QXmlStreamAttributes{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  QStringRef QXmlStreamAttributes::value(const QString & qualifiedName);
impl /*struct*/ QXmlStreamAttributes {
  pub fn value<RetType, T: QXmlStreamAttributes_value<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.value(self);
    // return 1;
  }
}

pub trait QXmlStreamAttributes_value<RetType> {
  fn value(self , rsthis: & QXmlStreamAttributes) -> RetType;
}

  // proto:  QStringRef QXmlStreamAttributes::value(const QString & qualifiedName);
impl<'a> /*trait*/ QXmlStreamAttributes_value<QStringRef> for (&'a QString) {
  fn value(self , rsthis: & QXmlStreamAttributes) -> QStringRef {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QXmlStreamAttributes5valueERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK20QXmlStreamAttributes5valueERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QStringRef::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QXmlStreamAttributes::QXmlStreamAttributes();
impl /*struct*/ QXmlStreamAttributes {
  pub fn new<T: QXmlStreamAttributes_new>(value: T) -> QXmlStreamAttributes {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QXmlStreamAttributes_new {
  fn new(self) -> QXmlStreamAttributes;
}

  // proto:  void QXmlStreamAttributes::QXmlStreamAttributes();
impl<'a> /*trait*/ QXmlStreamAttributes_new for () {
  fn new(self) -> QXmlStreamAttributes {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QXmlStreamAttributesC2Ev()};
    let ctysz: c_int = unsafe{QXmlStreamAttributes_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let qthis: u64 = unsafe {C_ZN20QXmlStreamAttributesC2Ev()};
    let rsthis = QXmlStreamAttributes{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QXmlStreamAttributes::hasAttribute(const QString & qualifiedName);
impl /*struct*/ QXmlStreamAttributes {
  pub fn hasAttribute<RetType, T: QXmlStreamAttributes_hasAttribute<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hasAttribute(self);
    // return 1;
  }
}

pub trait QXmlStreamAttributes_hasAttribute<RetType> {
  fn hasAttribute(self , rsthis: & QXmlStreamAttributes) -> RetType;
}

  // proto:  bool QXmlStreamAttributes::hasAttribute(const QString & qualifiedName);
impl<'a> /*trait*/ QXmlStreamAttributes_hasAttribute<i8> for (&'a QString) {
  fn hasAttribute(self , rsthis: & QXmlStreamAttributes) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QXmlStreamAttributes12hasAttributeERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK20QXmlStreamAttributes12hasAttributeERK7QString(rsthis.qclsinst, arg0)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  bool QXmlStreamAttributes::hasAttribute(const QString & namespaceUri, const QString & name);
impl<'a> /*trait*/ QXmlStreamAttributes_hasAttribute<i8> for (&'a QString, &'a QString) {
  fn hasAttribute(self , rsthis: & QXmlStreamAttributes) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QXmlStreamAttributes12hasAttributeERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK20QXmlStreamAttributes12hasAttributeERK7QStringS2_(rsthis.qclsinst, arg0, arg1)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QXmlStreamAttributes::append(const QString & namespaceUri, const QString & name, const QString & value);
impl /*struct*/ QXmlStreamAttributes {
  pub fn append<RetType, T: QXmlStreamAttributes_append<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.append(self);
    // return 1;
  }
}

pub trait QXmlStreamAttributes_append<RetType> {
  fn append(self , rsthis: & QXmlStreamAttributes) -> RetType;
}

  // proto:  void QXmlStreamAttributes::append(const QString & namespaceUri, const QString & name, const QString & value);
impl<'a> /*trait*/ QXmlStreamAttributes_append<()> for (&'a QString, &'a QString, &'a QString) {
  fn append(self , rsthis: & QXmlStreamAttributes) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QXmlStreamAttributes6appendERK7QStringS2_S2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {C_ZN20QXmlStreamAttributes6appendERK7QStringS2_S2_(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QXmlStreamAttributes::append(const QString & qualifiedName, const QString & value);
impl<'a> /*trait*/ QXmlStreamAttributes_append<()> for (&'a QString, &'a QString) {
  fn append(self , rsthis: & QXmlStreamAttributes) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QXmlStreamAttributes6appendERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {C_ZN20QXmlStreamAttributes6appendERK7QStringS2_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QStringRef QXmlStreamAttributes::value(const QString & namespaceUri, const QString & name);
impl<'a> /*trait*/ QXmlStreamAttributes_value<QStringRef> for (&'a QString, &'a QString) {
  fn value(self , rsthis: & QXmlStreamAttributes) -> QStringRef {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QXmlStreamAttributes5valueERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK20QXmlStreamAttributes5valueERK7QStringS2_(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QStringRef::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QXmlStreamWriter {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QXmlStreamWriter {
    return QXmlStreamWriter{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void QXmlStreamWriter::writeEndElement();
impl /*struct*/ QXmlStreamWriter {
  pub fn writeEndElement<RetType, T: QXmlStreamWriter_writeEndElement<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.writeEndElement(self);
    // return 1;
  }
}

pub trait QXmlStreamWriter_writeEndElement<RetType> {
  fn writeEndElement(self , rsthis: & QXmlStreamWriter) -> RetType;
}

  // proto:  void QXmlStreamWriter::writeEndElement();
impl<'a> /*trait*/ QXmlStreamWriter_writeEndElement<()> for () {
  fn writeEndElement(self , rsthis: & QXmlStreamWriter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriter15writeEndElementEv()};
     unsafe {C_ZN16QXmlStreamWriter15writeEndElementEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QXmlStreamWriter::QXmlStreamWriter();
impl /*struct*/ QXmlStreamWriter {
  pub fn new<T: QXmlStreamWriter_new>(value: T) -> QXmlStreamWriter {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QXmlStreamWriter_new {
  fn new(self) -> QXmlStreamWriter;
}

  // proto:  void QXmlStreamWriter::QXmlStreamWriter();
impl<'a> /*trait*/ QXmlStreamWriter_new for () {
  fn new(self) -> QXmlStreamWriter {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriterC2Ev()};
    let ctysz: c_int = unsafe{QXmlStreamWriter_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let qthis: u64 = unsafe {C_ZN16QXmlStreamWriterC2Ev()};
    let rsthis = QXmlStreamWriter{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QXmlStreamWriter::writeEndDocument();
impl /*struct*/ QXmlStreamWriter {
  pub fn writeEndDocument<RetType, T: QXmlStreamWriter_writeEndDocument<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.writeEndDocument(self);
    // return 1;
  }
}

pub trait QXmlStreamWriter_writeEndDocument<RetType> {
  fn writeEndDocument(self , rsthis: & QXmlStreamWriter) -> RetType;
}

  // proto:  void QXmlStreamWriter::writeEndDocument();
impl<'a> /*trait*/ QXmlStreamWriter_writeEndDocument<()> for () {
  fn writeEndDocument(self , rsthis: & QXmlStreamWriter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriter16writeEndDocumentEv()};
     unsafe {C_ZN16QXmlStreamWriter16writeEndDocumentEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QXmlStreamWriter::autoFormatting();
impl /*struct*/ QXmlStreamWriter {
  pub fn autoFormatting<RetType, T: QXmlStreamWriter_autoFormatting<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.autoFormatting(self);
    // return 1;
  }
}

pub trait QXmlStreamWriter_autoFormatting<RetType> {
  fn autoFormatting(self , rsthis: & QXmlStreamWriter) -> RetType;
}

  // proto:  bool QXmlStreamWriter::autoFormatting();
impl<'a> /*trait*/ QXmlStreamWriter_autoFormatting<i8> for () {
  fn autoFormatting(self , rsthis: & QXmlStreamWriter) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamWriter14autoFormattingEv()};
    let mut ret = unsafe {C_ZNK16QXmlStreamWriter14autoFormattingEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QXmlStreamWriter::writeStartDocument(const QString & version, bool standalone);
impl /*struct*/ QXmlStreamWriter {
  pub fn writeStartDocument<RetType, T: QXmlStreamWriter_writeStartDocument<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.writeStartDocument(self);
    // return 1;
  }
}

pub trait QXmlStreamWriter_writeStartDocument<RetType> {
  fn writeStartDocument(self , rsthis: & QXmlStreamWriter) -> RetType;
}

  // proto:  void QXmlStreamWriter::writeStartDocument(const QString & version, bool standalone);
impl<'a> /*trait*/ QXmlStreamWriter_writeStartDocument<()> for (&'a QString, i8) {
  fn writeStartDocument(self , rsthis: & QXmlStreamWriter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriter18writeStartDocumentERK7QStringb()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_char;
     unsafe {C_ZN16QXmlStreamWriter18writeStartDocumentERK7QStringb(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QXmlStreamWriter::setCodec(QTextCodec * codec);
impl /*struct*/ QXmlStreamWriter {
  pub fn setCodec<RetType, T: QXmlStreamWriter_setCodec<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setCodec(self);
    // return 1;
  }
}

pub trait QXmlStreamWriter_setCodec<RetType> {
  fn setCodec(self , rsthis: & QXmlStreamWriter) -> RetType;
}

  // proto:  void QXmlStreamWriter::setCodec(QTextCodec * codec);
impl<'a> /*trait*/ QXmlStreamWriter_setCodec<()> for (&'a QTextCodec) {
  fn setCodec(self , rsthis: & QXmlStreamWriter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriter8setCodecEP10QTextCodec()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN16QXmlStreamWriter8setCodecEP10QTextCodec(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QXmlStreamWriter::writeProcessingInstruction(const QString & target, const QString & data);
impl /*struct*/ QXmlStreamWriter {
  pub fn writeProcessingInstruction<RetType, T: QXmlStreamWriter_writeProcessingInstruction<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.writeProcessingInstruction(self);
    // return 1;
  }
}

pub trait QXmlStreamWriter_writeProcessingInstruction<RetType> {
  fn writeProcessingInstruction(self , rsthis: & QXmlStreamWriter) -> RetType;
}

  // proto:  void QXmlStreamWriter::writeProcessingInstruction(const QString & target, const QString & data);
impl<'a> /*trait*/ QXmlStreamWriter_writeProcessingInstruction<()> for (&'a QString, &'a QString) {
  fn writeProcessingInstruction(self , rsthis: & QXmlStreamWriter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriter26writeProcessingInstructionERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {C_ZN16QXmlStreamWriter26writeProcessingInstructionERK7QStringS2_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QXmlStreamWriter::writeCharacters(const QString & text);
impl /*struct*/ QXmlStreamWriter {
  pub fn writeCharacters<RetType, T: QXmlStreamWriter_writeCharacters<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.writeCharacters(self);
    // return 1;
  }
}

pub trait QXmlStreamWriter_writeCharacters<RetType> {
  fn writeCharacters(self , rsthis: & QXmlStreamWriter) -> RetType;
}

  // proto:  void QXmlStreamWriter::writeCharacters(const QString & text);
impl<'a> /*trait*/ QXmlStreamWriter_writeCharacters<()> for (&'a QString) {
  fn writeCharacters(self , rsthis: & QXmlStreamWriter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriter15writeCharactersERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN16QXmlStreamWriter15writeCharactersERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QXmlStreamWriter::setDevice(QIODevice * device);
impl /*struct*/ QXmlStreamWriter {
  pub fn setDevice<RetType, T: QXmlStreamWriter_setDevice<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setDevice(self);
    // return 1;
  }
}

pub trait QXmlStreamWriter_setDevice<RetType> {
  fn setDevice(self , rsthis: & QXmlStreamWriter) -> RetType;
}

  // proto:  void QXmlStreamWriter::setDevice(QIODevice * device);
impl<'a> /*trait*/ QXmlStreamWriter_setDevice<()> for (&'a QIODevice) {
  fn setDevice(self , rsthis: & QXmlStreamWriter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriter9setDeviceEP9QIODevice()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN16QXmlStreamWriter9setDeviceEP9QIODevice(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QXmlStreamWriter::writeStartDocument(const QString & version);
impl<'a> /*trait*/ QXmlStreamWriter_writeStartDocument<()> for (&'a QString) {
  fn writeStartDocument(self , rsthis: & QXmlStreamWriter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriter18writeStartDocumentERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN16QXmlStreamWriter18writeStartDocumentERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QXmlStreamWriter::writeTextElement(const QString & namespaceUri, const QString & name, const QString & text);
impl /*struct*/ QXmlStreamWriter {
  pub fn writeTextElement<RetType, T: QXmlStreamWriter_writeTextElement<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.writeTextElement(self);
    // return 1;
  }
}

pub trait QXmlStreamWriter_writeTextElement<RetType> {
  fn writeTextElement(self , rsthis: & QXmlStreamWriter) -> RetType;
}

  // proto:  void QXmlStreamWriter::writeTextElement(const QString & namespaceUri, const QString & name, const QString & text);
impl<'a> /*trait*/ QXmlStreamWriter_writeTextElement<()> for (&'a QString, &'a QString, &'a QString) {
  fn writeTextElement(self , rsthis: & QXmlStreamWriter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriter16writeTextElementERK7QStringS2_S2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {C_ZN16QXmlStreamWriter16writeTextElementERK7QStringS2_S2_(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QXmlStreamWriter::writeAttribute(const QString & qualifiedName, const QString & value);
impl /*struct*/ QXmlStreamWriter {
  pub fn writeAttribute<RetType, T: QXmlStreamWriter_writeAttribute<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.writeAttribute(self);
    // return 1;
  }
}

pub trait QXmlStreamWriter_writeAttribute<RetType> {
  fn writeAttribute(self , rsthis: & QXmlStreamWriter) -> RetType;
}

  // proto:  void QXmlStreamWriter::writeAttribute(const QString & qualifiedName, const QString & value);
impl<'a> /*trait*/ QXmlStreamWriter_writeAttribute<()> for (&'a QString, &'a QString) {
  fn writeAttribute(self , rsthis: & QXmlStreamWriter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriter14writeAttributeERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {C_ZN16QXmlStreamWriter14writeAttributeERK7QStringS2_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QXmlStreamWriter::writeEmptyElement(const QString & qualifiedName);
impl /*struct*/ QXmlStreamWriter {
  pub fn writeEmptyElement<RetType, T: QXmlStreamWriter_writeEmptyElement<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.writeEmptyElement(self);
    // return 1;
  }
}

pub trait QXmlStreamWriter_writeEmptyElement<RetType> {
  fn writeEmptyElement(self , rsthis: & QXmlStreamWriter) -> RetType;
}

  // proto:  void QXmlStreamWriter::writeEmptyElement(const QString & qualifiedName);
impl<'a> /*trait*/ QXmlStreamWriter_writeEmptyElement<()> for (&'a QString) {
  fn writeEmptyElement(self , rsthis: & QXmlStreamWriter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriter17writeEmptyElementERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN16QXmlStreamWriter17writeEmptyElementERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QXmlStreamWriter::writeDTD(const QString & dtd);
impl /*struct*/ QXmlStreamWriter {
  pub fn writeDTD<RetType, T: QXmlStreamWriter_writeDTD<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.writeDTD(self);
    // return 1;
  }
}

pub trait QXmlStreamWriter_writeDTD<RetType> {
  fn writeDTD(self , rsthis: & QXmlStreamWriter) -> RetType;
}

  // proto:  void QXmlStreamWriter::writeDTD(const QString & dtd);
impl<'a> /*trait*/ QXmlStreamWriter_writeDTD<()> for (&'a QString) {
  fn writeDTD(self , rsthis: & QXmlStreamWriter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriter8writeDTDERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN16QXmlStreamWriter8writeDTDERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QXmlStreamWriter::setAutoFormattingIndent(int spacesOrTabs);
impl /*struct*/ QXmlStreamWriter {
  pub fn setAutoFormattingIndent<RetType, T: QXmlStreamWriter_setAutoFormattingIndent<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setAutoFormattingIndent(self);
    // return 1;
  }
}

pub trait QXmlStreamWriter_setAutoFormattingIndent<RetType> {
  fn setAutoFormattingIndent(self , rsthis: & QXmlStreamWriter) -> RetType;
}

  // proto:  void QXmlStreamWriter::setAutoFormattingIndent(int spacesOrTabs);
impl<'a> /*trait*/ QXmlStreamWriter_setAutoFormattingIndent<()> for (i32) {
  fn setAutoFormattingIndent(self , rsthis: & QXmlStreamWriter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriter23setAutoFormattingIndentEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN16QXmlStreamWriter23setAutoFormattingIndentEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QXmlStreamWriter::writeAttribute(const QXmlStreamAttribute & attribute);
impl<'a> /*trait*/ QXmlStreamWriter_writeAttribute<()> for (&'a QXmlStreamAttribute) {
  fn writeAttribute(self , rsthis: & QXmlStreamWriter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriter14writeAttributeERK19QXmlStreamAttribute()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN16QXmlStreamWriter14writeAttributeERK19QXmlStreamAttribute(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QXmlStreamWriter::writeStartElement(const QString & namespaceUri, const QString & name);
impl /*struct*/ QXmlStreamWriter {
  pub fn writeStartElement<RetType, T: QXmlStreamWriter_writeStartElement<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.writeStartElement(self);
    // return 1;
  }
}

pub trait QXmlStreamWriter_writeStartElement<RetType> {
  fn writeStartElement(self , rsthis: & QXmlStreamWriter) -> RetType;
}

  // proto:  void QXmlStreamWriter::writeStartElement(const QString & namespaceUri, const QString & name);
impl<'a> /*trait*/ QXmlStreamWriter_writeStartElement<()> for (&'a QString, &'a QString) {
  fn writeStartElement(self , rsthis: & QXmlStreamWriter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriter17writeStartElementERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {C_ZN16QXmlStreamWriter17writeStartElementERK7QStringS2_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QXmlStreamWriter::QXmlStreamWriter(QString * string);
impl<'a> /*trait*/ QXmlStreamWriter_new for (&'a QString) {
  fn new(self) -> QXmlStreamWriter {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriterC2EP7QString()};
    let ctysz: c_int = unsafe{QXmlStreamWriter_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN16QXmlStreamWriterC2EP7QString(arg0)};
    let rsthis = QXmlStreamWriter{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QXmlStreamWriter::writeComment(const QString & text);
impl /*struct*/ QXmlStreamWriter {
  pub fn writeComment<RetType, T: QXmlStreamWriter_writeComment<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.writeComment(self);
    // return 1;
  }
}

pub trait QXmlStreamWriter_writeComment<RetType> {
  fn writeComment(self , rsthis: & QXmlStreamWriter) -> RetType;
}

  // proto:  void QXmlStreamWriter::writeComment(const QString & text);
impl<'a> /*trait*/ QXmlStreamWriter_writeComment<()> for (&'a QString) {
  fn writeComment(self , rsthis: & QXmlStreamWriter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriter12writeCommentERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN16QXmlStreamWriter12writeCommentERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QTextCodec * QXmlStreamWriter::codec();
impl /*struct*/ QXmlStreamWriter {
  pub fn codec<RetType, T: QXmlStreamWriter_codec<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.codec(self);
    // return 1;
  }
}

pub trait QXmlStreamWriter_codec<RetType> {
  fn codec(self , rsthis: & QXmlStreamWriter) -> RetType;
}

  // proto:  QTextCodec * QXmlStreamWriter::codec();
impl<'a> /*trait*/ QXmlStreamWriter_codec<QTextCodec> for () {
  fn codec(self , rsthis: & QXmlStreamWriter) -> QTextCodec {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamWriter5codecEv()};
    let mut ret = unsafe {C_ZNK16QXmlStreamWriter5codecEv(rsthis.qclsinst)};
    let mut ret1 = QTextCodec::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QXmlStreamWriter::writeAttribute(const QString & namespaceUri, const QString & name, const QString & value);
impl<'a> /*trait*/ QXmlStreamWriter_writeAttribute<()> for (&'a QString, &'a QString, &'a QString) {
  fn writeAttribute(self , rsthis: & QXmlStreamWriter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriter14writeAttributeERK7QStringS2_S2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {C_ZN16QXmlStreamWriter14writeAttributeERK7QStringS2_S2_(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QXmlStreamWriter::writeNamespace(const QString & namespaceUri, const QString & prefix);
impl /*struct*/ QXmlStreamWriter {
  pub fn writeNamespace<RetType, T: QXmlStreamWriter_writeNamespace<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.writeNamespace(self);
    // return 1;
  }
}

pub trait QXmlStreamWriter_writeNamespace<RetType> {
  fn writeNamespace(self , rsthis: & QXmlStreamWriter) -> RetType;
}

  // proto:  void QXmlStreamWriter::writeNamespace(const QString & namespaceUri, const QString & prefix);
impl<'a> /*trait*/ QXmlStreamWriter_writeNamespace<()> for (&'a QString, &'a QString) {
  fn writeNamespace(self , rsthis: & QXmlStreamWriter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriter14writeNamespaceERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {C_ZN16QXmlStreamWriter14writeNamespaceERK7QStringS2_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  bool QXmlStreamWriter::hasError();
impl /*struct*/ QXmlStreamWriter {
  pub fn hasError<RetType, T: QXmlStreamWriter_hasError<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hasError(self);
    // return 1;
  }
}

pub trait QXmlStreamWriter_hasError<RetType> {
  fn hasError(self , rsthis: & QXmlStreamWriter) -> RetType;
}

  // proto:  bool QXmlStreamWriter::hasError();
impl<'a> /*trait*/ QXmlStreamWriter_hasError<i8> for () {
  fn hasError(self , rsthis: & QXmlStreamWriter) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamWriter8hasErrorEv()};
    let mut ret = unsafe {C_ZNK16QXmlStreamWriter8hasErrorEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QXmlStreamWriter::writeCDATA(const QString & text);
impl /*struct*/ QXmlStreamWriter {
  pub fn writeCDATA<RetType, T: QXmlStreamWriter_writeCDATA<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.writeCDATA(self);
    // return 1;
  }
}

pub trait QXmlStreamWriter_writeCDATA<RetType> {
  fn writeCDATA(self , rsthis: & QXmlStreamWriter) -> RetType;
}

  // proto:  void QXmlStreamWriter::writeCDATA(const QString & text);
impl<'a> /*trait*/ QXmlStreamWriter_writeCDATA<()> for (&'a QString) {
  fn writeCDATA(self , rsthis: & QXmlStreamWriter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriter10writeCDATAERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN16QXmlStreamWriter10writeCDATAERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QXmlStreamWriter::writeStartDocument();
impl<'a> /*trait*/ QXmlStreamWriter_writeStartDocument<()> for () {
  fn writeStartDocument(self , rsthis: & QXmlStreamWriter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriter18writeStartDocumentEv()};
     unsafe {C_ZN16QXmlStreamWriter18writeStartDocumentEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QXmlStreamWriter::writeEntityReference(const QString & name);
impl /*struct*/ QXmlStreamWriter {
  pub fn writeEntityReference<RetType, T: QXmlStreamWriter_writeEntityReference<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.writeEntityReference(self);
    // return 1;
  }
}

pub trait QXmlStreamWriter_writeEntityReference<RetType> {
  fn writeEntityReference(self , rsthis: & QXmlStreamWriter) -> RetType;
}

  // proto:  void QXmlStreamWriter::writeEntityReference(const QString & name);
impl<'a> /*trait*/ QXmlStreamWriter_writeEntityReference<()> for (&'a QString) {
  fn writeEntityReference(self , rsthis: & QXmlStreamWriter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriter20writeEntityReferenceERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN16QXmlStreamWriter20writeEntityReferenceERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QXmlStreamWriter::setAutoFormatting(bool );
impl /*struct*/ QXmlStreamWriter {
  pub fn setAutoFormatting<RetType, T: QXmlStreamWriter_setAutoFormatting<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setAutoFormatting(self);
    // return 1;
  }
}

pub trait QXmlStreamWriter_setAutoFormatting<RetType> {
  fn setAutoFormatting(self , rsthis: & QXmlStreamWriter) -> RetType;
}

  // proto:  void QXmlStreamWriter::setAutoFormatting(bool );
impl<'a> /*trait*/ QXmlStreamWriter_setAutoFormatting<()> for (i8) {
  fn setAutoFormatting(self , rsthis: & QXmlStreamWriter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriter17setAutoFormattingEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN16QXmlStreamWriter17setAutoFormattingEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QXmlStreamWriter::setCodec(const char * codecName);
impl<'a> /*trait*/ QXmlStreamWriter_setCodec<()> for (&'a  String) {
  fn setCodec(self , rsthis: & QXmlStreamWriter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriter8setCodecEPKc()};
    let arg0 = self.as_ptr()  as *mut c_char;
     unsafe {C_ZN16QXmlStreamWriter8setCodecEPKc(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QXmlStreamWriter::autoFormattingIndent();
impl /*struct*/ QXmlStreamWriter {
  pub fn autoFormattingIndent<RetType, T: QXmlStreamWriter_autoFormattingIndent<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.autoFormattingIndent(self);
    // return 1;
  }
}

pub trait QXmlStreamWriter_autoFormattingIndent<RetType> {
  fn autoFormattingIndent(self , rsthis: & QXmlStreamWriter) -> RetType;
}

  // proto:  int QXmlStreamWriter::autoFormattingIndent();
impl<'a> /*trait*/ QXmlStreamWriter_autoFormattingIndent<i32> for () {
  fn autoFormattingIndent(self , rsthis: & QXmlStreamWriter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamWriter20autoFormattingIndentEv()};
    let mut ret = unsafe {C_ZNK16QXmlStreamWriter20autoFormattingIndentEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  void QXmlStreamWriter::~QXmlStreamWriter();
impl /*struct*/ QXmlStreamWriter {
  pub fn free<RetType, T: QXmlStreamWriter_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QXmlStreamWriter_free<RetType> {
  fn free(self , rsthis: & QXmlStreamWriter) -> RetType;
}

  // proto:  void QXmlStreamWriter::~QXmlStreamWriter();
impl<'a> /*trait*/ QXmlStreamWriter_free<()> for () {
  fn free(self , rsthis: & QXmlStreamWriter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriterD2Ev()};
     unsafe {C_ZN16QXmlStreamWriterD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QXmlStreamWriter::writeAttributes(const QXmlStreamAttributes & attributes);
impl /*struct*/ QXmlStreamWriter {
  pub fn writeAttributes<RetType, T: QXmlStreamWriter_writeAttributes<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.writeAttributes(self);
    // return 1;
  }
}

pub trait QXmlStreamWriter_writeAttributes<RetType> {
  fn writeAttributes(self , rsthis: & QXmlStreamWriter) -> RetType;
}

  // proto:  void QXmlStreamWriter::writeAttributes(const QXmlStreamAttributes & attributes);
impl<'a> /*trait*/ QXmlStreamWriter_writeAttributes<()> for (&'a QXmlStreamAttributes) {
  fn writeAttributes(self , rsthis: & QXmlStreamWriter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriter15writeAttributesERK20QXmlStreamAttributes()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN16QXmlStreamWriter15writeAttributesERK20QXmlStreamAttributes(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QXmlStreamWriter::writeDefaultNamespace(const QString & namespaceUri);
impl /*struct*/ QXmlStreamWriter {
  pub fn writeDefaultNamespace<RetType, T: QXmlStreamWriter_writeDefaultNamespace<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.writeDefaultNamespace(self);
    // return 1;
  }
}

pub trait QXmlStreamWriter_writeDefaultNamespace<RetType> {
  fn writeDefaultNamespace(self , rsthis: & QXmlStreamWriter) -> RetType;
}

  // proto:  void QXmlStreamWriter::writeDefaultNamespace(const QString & namespaceUri);
impl<'a> /*trait*/ QXmlStreamWriter_writeDefaultNamespace<()> for (&'a QString) {
  fn writeDefaultNamespace(self , rsthis: & QXmlStreamWriter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriter21writeDefaultNamespaceERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN16QXmlStreamWriter21writeDefaultNamespaceERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QIODevice * QXmlStreamWriter::device();
impl /*struct*/ QXmlStreamWriter {
  pub fn device<RetType, T: QXmlStreamWriter_device<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.device(self);
    // return 1;
  }
}

pub trait QXmlStreamWriter_device<RetType> {
  fn device(self , rsthis: & QXmlStreamWriter) -> RetType;
}

  // proto:  QIODevice * QXmlStreamWriter::device();
impl<'a> /*trait*/ QXmlStreamWriter_device<QIODevice> for () {
  fn device(self , rsthis: & QXmlStreamWriter) -> QIODevice {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QXmlStreamWriter6deviceEv()};
    let mut ret = unsafe {C_ZNK16QXmlStreamWriter6deviceEv(rsthis.qclsinst)};
    let mut ret1 = QIODevice::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QXmlStreamWriter::writeCurrentToken(const QXmlStreamReader & reader);
impl /*struct*/ QXmlStreamWriter {
  pub fn writeCurrentToken<RetType, T: QXmlStreamWriter_writeCurrentToken<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.writeCurrentToken(self);
    // return 1;
  }
}

pub trait QXmlStreamWriter_writeCurrentToken<RetType> {
  fn writeCurrentToken(self , rsthis: & QXmlStreamWriter) -> RetType;
}

  // proto:  void QXmlStreamWriter::writeCurrentToken(const QXmlStreamReader & reader);
impl<'a> /*trait*/ QXmlStreamWriter_writeCurrentToken<()> for (&'a QXmlStreamReader) {
  fn writeCurrentToken(self , rsthis: & QXmlStreamWriter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriter17writeCurrentTokenERK16QXmlStreamReader()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN16QXmlStreamWriter17writeCurrentTokenERK16QXmlStreamReader(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QXmlStreamWriter::QXmlStreamWriter(QByteArray * array);
impl<'a> /*trait*/ QXmlStreamWriter_new for (&'a QByteArray) {
  fn new(self) -> QXmlStreamWriter {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriterC2EP10QByteArray()};
    let ctysz: c_int = unsafe{QXmlStreamWriter_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN16QXmlStreamWriterC2EP10QByteArray(arg0)};
    let rsthis = QXmlStreamWriter{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QXmlStreamWriter::writeTextElement(const QString & qualifiedName, const QString & text);
impl<'a> /*trait*/ QXmlStreamWriter_writeTextElement<()> for (&'a QString, &'a QString) {
  fn writeTextElement(self , rsthis: & QXmlStreamWriter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriter16writeTextElementERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {C_ZN16QXmlStreamWriter16writeTextElementERK7QStringS2_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QXmlStreamWriter::writeEmptyElement(const QString & namespaceUri, const QString & name);
impl<'a> /*trait*/ QXmlStreamWriter_writeEmptyElement<()> for (&'a QString, &'a QString) {
  fn writeEmptyElement(self , rsthis: & QXmlStreamWriter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriter17writeEmptyElementERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {C_ZN16QXmlStreamWriter17writeEmptyElementERK7QStringS2_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QXmlStreamWriter::QXmlStreamWriter(QIODevice * device);
impl<'a> /*trait*/ QXmlStreamWriter_new for (&'a QIODevice) {
  fn new(self) -> QXmlStreamWriter {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriterC2EP9QIODevice()};
    let ctysz: c_int = unsafe{QXmlStreamWriter_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN16QXmlStreamWriterC2EP9QIODevice(arg0)};
    let rsthis = QXmlStreamWriter{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QXmlStreamWriter::writeStartElement(const QString & qualifiedName);
impl<'a> /*trait*/ QXmlStreamWriter_writeStartElement<()> for (&'a QString) {
  fn writeStartElement(self , rsthis: & QXmlStreamWriter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriter17writeStartElementERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN16QXmlStreamWriter17writeStartElementERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QXmlStreamNotationDeclaration {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QXmlStreamNotationDeclaration {
    return QXmlStreamNotationDeclaration{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void QXmlStreamNotationDeclaration::QXmlStreamNotationDeclaration(const QXmlStreamNotationDeclaration & );
impl /*struct*/ QXmlStreamNotationDeclaration {
  pub fn new<T: QXmlStreamNotationDeclaration_new>(value: T) -> QXmlStreamNotationDeclaration {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QXmlStreamNotationDeclaration_new {
  fn new(self) -> QXmlStreamNotationDeclaration;
}

  // proto:  void QXmlStreamNotationDeclaration::QXmlStreamNotationDeclaration(const QXmlStreamNotationDeclaration & );
impl<'a> /*trait*/ QXmlStreamNotationDeclaration_new for (&'a QXmlStreamNotationDeclaration) {
  fn new(self) -> QXmlStreamNotationDeclaration {
    // let qthis: *mut c_void = unsafe{calloc(1, 56)};
    // unsafe{_ZN29QXmlStreamNotationDeclarationC2ERKS_()};
    let ctysz: c_int = unsafe{QXmlStreamNotationDeclaration_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN29QXmlStreamNotationDeclarationC2ERKS_(arg0)};
    let rsthis = QXmlStreamNotationDeclaration{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QStringRef QXmlStreamNotationDeclaration::publicId();
impl /*struct*/ QXmlStreamNotationDeclaration {
  pub fn publicId<RetType, T: QXmlStreamNotationDeclaration_publicId<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.publicId(self);
    // return 1;
  }
}

pub trait QXmlStreamNotationDeclaration_publicId<RetType> {
  fn publicId(self , rsthis: & QXmlStreamNotationDeclaration) -> RetType;
}

  // proto:  QStringRef QXmlStreamNotationDeclaration::publicId();
impl<'a> /*trait*/ QXmlStreamNotationDeclaration_publicId<QStringRef> for () {
  fn publicId(self , rsthis: & QXmlStreamNotationDeclaration) -> QStringRef {
    // let qthis: *mut c_void = unsafe{calloc(1, 56)};
    // unsafe{_ZNK29QXmlStreamNotationDeclaration8publicIdEv()};
    let mut ret = unsafe {C_ZNK29QXmlStreamNotationDeclaration8publicIdEv(rsthis.qclsinst)};
    let mut ret1 = QStringRef::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QStringRef QXmlStreamNotationDeclaration::name();
impl /*struct*/ QXmlStreamNotationDeclaration {
  pub fn name<RetType, T: QXmlStreamNotationDeclaration_name<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.name(self);
    // return 1;
  }
}

pub trait QXmlStreamNotationDeclaration_name<RetType> {
  fn name(self , rsthis: & QXmlStreamNotationDeclaration) -> RetType;
}

  // proto:  QStringRef QXmlStreamNotationDeclaration::name();
impl<'a> /*trait*/ QXmlStreamNotationDeclaration_name<QStringRef> for () {
  fn name(self , rsthis: & QXmlStreamNotationDeclaration) -> QStringRef {
    // let qthis: *mut c_void = unsafe{calloc(1, 56)};
    // unsafe{_ZNK29QXmlStreamNotationDeclaration4nameEv()};
    let mut ret = unsafe {C_ZNK29QXmlStreamNotationDeclaration4nameEv(rsthis.qclsinst)};
    let mut ret1 = QStringRef::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QXmlStreamNotationDeclaration::~QXmlStreamNotationDeclaration();
impl /*struct*/ QXmlStreamNotationDeclaration {
  pub fn free<RetType, T: QXmlStreamNotationDeclaration_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QXmlStreamNotationDeclaration_free<RetType> {
  fn free(self , rsthis: & QXmlStreamNotationDeclaration) -> RetType;
}

  // proto:  void QXmlStreamNotationDeclaration::~QXmlStreamNotationDeclaration();
impl<'a> /*trait*/ QXmlStreamNotationDeclaration_free<()> for () {
  fn free(self , rsthis: & QXmlStreamNotationDeclaration) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 56)};
    // unsafe{_ZN29QXmlStreamNotationDeclarationD2Ev()};
     unsafe {C_ZN29QXmlStreamNotationDeclarationD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QStringRef QXmlStreamNotationDeclaration::systemId();
impl /*struct*/ QXmlStreamNotationDeclaration {
  pub fn systemId<RetType, T: QXmlStreamNotationDeclaration_systemId<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.systemId(self);
    // return 1;
  }
}

pub trait QXmlStreamNotationDeclaration_systemId<RetType> {
  fn systemId(self , rsthis: & QXmlStreamNotationDeclaration) -> RetType;
}

  // proto:  QStringRef QXmlStreamNotationDeclaration::systemId();
impl<'a> /*trait*/ QXmlStreamNotationDeclaration_systemId<QStringRef> for () {
  fn systemId(self , rsthis: & QXmlStreamNotationDeclaration) -> QStringRef {
    // let qthis: *mut c_void = unsafe{calloc(1, 56)};
    // unsafe{_ZNK29QXmlStreamNotationDeclaration8systemIdEv()};
    let mut ret = unsafe {C_ZNK29QXmlStreamNotationDeclaration8systemIdEv(rsthis.qclsinst)};
    let mut ret1 = QStringRef::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QXmlStreamNotationDeclaration::QXmlStreamNotationDeclaration();
impl<'a> /*trait*/ QXmlStreamNotationDeclaration_new for () {
  fn new(self) -> QXmlStreamNotationDeclaration {
    // let qthis: *mut c_void = unsafe{calloc(1, 56)};
    // unsafe{_ZN29QXmlStreamNotationDeclarationC2Ev()};
    let ctysz: c_int = unsafe{QXmlStreamNotationDeclaration_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let qthis: u64 = unsafe {C_ZN29QXmlStreamNotationDeclarationC2Ev()};
    let rsthis = QXmlStreamNotationDeclaration{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QXmlStreamAttribute {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QXmlStreamAttribute {
    return QXmlStreamAttribute{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void QXmlStreamAttribute::QXmlStreamAttribute(const QString & qualifiedName, const QString & value);
impl /*struct*/ QXmlStreamAttribute {
  pub fn new<T: QXmlStreamAttribute_new>(value: T) -> QXmlStreamAttribute {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QXmlStreamAttribute_new {
  fn new(self) -> QXmlStreamAttribute;
}

  // proto:  void QXmlStreamAttribute::QXmlStreamAttribute(const QString & qualifiedName, const QString & value);
impl<'a> /*trait*/ QXmlStreamAttribute_new for (&'a QString, &'a QString) {
  fn new(self) -> QXmlStreamAttribute {
    // let qthis: *mut c_void = unsafe{calloc(1, 80)};
    // unsafe{_ZN19QXmlStreamAttributeC2ERK7QStringS2_()};
    let ctysz: c_int = unsafe{QXmlStreamAttribute_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN19QXmlStreamAttributeC2ERK7QStringS2_(arg0, arg1)};
    let rsthis = QXmlStreamAttribute{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QStringRef QXmlStreamAttribute::qualifiedName();
impl /*struct*/ QXmlStreamAttribute {
  pub fn qualifiedName<RetType, T: QXmlStreamAttribute_qualifiedName<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.qualifiedName(self);
    // return 1;
  }
}

pub trait QXmlStreamAttribute_qualifiedName<RetType> {
  fn qualifiedName(self , rsthis: & QXmlStreamAttribute) -> RetType;
}

  // proto:  QStringRef QXmlStreamAttribute::qualifiedName();
impl<'a> /*trait*/ QXmlStreamAttribute_qualifiedName<QStringRef> for () {
  fn qualifiedName(self , rsthis: & QXmlStreamAttribute) -> QStringRef {
    // let qthis: *mut c_void = unsafe{calloc(1, 80)};
    // unsafe{_ZNK19QXmlStreamAttribute13qualifiedNameEv()};
    let mut ret = unsafe {C_ZNK19QXmlStreamAttribute13qualifiedNameEv(rsthis.qclsinst)};
    let mut ret1 = QStringRef::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QXmlStreamAttribute::~QXmlStreamAttribute();
impl /*struct*/ QXmlStreamAttribute {
  pub fn free<RetType, T: QXmlStreamAttribute_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QXmlStreamAttribute_free<RetType> {
  fn free(self , rsthis: & QXmlStreamAttribute) -> RetType;
}

  // proto:  void QXmlStreamAttribute::~QXmlStreamAttribute();
impl<'a> /*trait*/ QXmlStreamAttribute_free<()> for () {
  fn free(self , rsthis: & QXmlStreamAttribute) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 80)};
    // unsafe{_ZN19QXmlStreamAttributeD2Ev()};
     unsafe {C_ZN19QXmlStreamAttributeD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QStringRef QXmlStreamAttribute::value();
impl /*struct*/ QXmlStreamAttribute {
  pub fn value<RetType, T: QXmlStreamAttribute_value<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.value(self);
    // return 1;
  }
}

pub trait QXmlStreamAttribute_value<RetType> {
  fn value(self , rsthis: & QXmlStreamAttribute) -> RetType;
}

  // proto:  QStringRef QXmlStreamAttribute::value();
impl<'a> /*trait*/ QXmlStreamAttribute_value<QStringRef> for () {
  fn value(self , rsthis: & QXmlStreamAttribute) -> QStringRef {
    // let qthis: *mut c_void = unsafe{calloc(1, 80)};
    // unsafe{_ZNK19QXmlStreamAttribute5valueEv()};
    let mut ret = unsafe {C_ZNK19QXmlStreamAttribute5valueEv(rsthis.qclsinst)};
    let mut ret1 = QStringRef::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QStringRef QXmlStreamAttribute::namespaceUri();
impl /*struct*/ QXmlStreamAttribute {
  pub fn namespaceUri<RetType, T: QXmlStreamAttribute_namespaceUri<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.namespaceUri(self);
    // return 1;
  }
}

pub trait QXmlStreamAttribute_namespaceUri<RetType> {
  fn namespaceUri(self , rsthis: & QXmlStreamAttribute) -> RetType;
}

  // proto:  QStringRef QXmlStreamAttribute::namespaceUri();
impl<'a> /*trait*/ QXmlStreamAttribute_namespaceUri<QStringRef> for () {
  fn namespaceUri(self , rsthis: & QXmlStreamAttribute) -> QStringRef {
    // let qthis: *mut c_void = unsafe{calloc(1, 80)};
    // unsafe{_ZNK19QXmlStreamAttribute12namespaceUriEv()};
    let mut ret = unsafe {C_ZNK19QXmlStreamAttribute12namespaceUriEv(rsthis.qclsinst)};
    let mut ret1 = QStringRef::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QXmlStreamAttribute::QXmlStreamAttribute();
impl<'a> /*trait*/ QXmlStreamAttribute_new for () {
  fn new(self) -> QXmlStreamAttribute {
    // let qthis: *mut c_void = unsafe{calloc(1, 80)};
    // unsafe{_ZN19QXmlStreamAttributeC2Ev()};
    let ctysz: c_int = unsafe{QXmlStreamAttribute_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let qthis: u64 = unsafe {C_ZN19QXmlStreamAttributeC2Ev()};
    let rsthis = QXmlStreamAttribute{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QXmlStreamAttribute::QXmlStreamAttribute(const QXmlStreamAttribute & );
impl<'a> /*trait*/ QXmlStreamAttribute_new for (&'a QXmlStreamAttribute) {
  fn new(self) -> QXmlStreamAttribute {
    // let qthis: *mut c_void = unsafe{calloc(1, 80)};
    // unsafe{_ZN19QXmlStreamAttributeC2ERKS_()};
    let ctysz: c_int = unsafe{QXmlStreamAttribute_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN19QXmlStreamAttributeC2ERKS_(arg0)};
    let rsthis = QXmlStreamAttribute{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QXmlStreamAttribute::QXmlStreamAttribute(const QString & namespaceUri, const QString & name, const QString & value);
impl<'a> /*trait*/ QXmlStreamAttribute_new for (&'a QString, &'a QString, &'a QString) {
  fn new(self) -> QXmlStreamAttribute {
    // let qthis: *mut c_void = unsafe{calloc(1, 80)};
    // unsafe{_ZN19QXmlStreamAttributeC2ERK7QStringS2_S2_()};
    let ctysz: c_int = unsafe{QXmlStreamAttribute_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN19QXmlStreamAttributeC2ERK7QStringS2_S2_(arg0, arg1, arg2)};
    let rsthis = QXmlStreamAttribute{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QXmlStreamAttribute::isDefault();
impl /*struct*/ QXmlStreamAttribute {
  pub fn isDefault<RetType, T: QXmlStreamAttribute_isDefault<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isDefault(self);
    // return 1;
  }
}

pub trait QXmlStreamAttribute_isDefault<RetType> {
  fn isDefault(self , rsthis: & QXmlStreamAttribute) -> RetType;
}

  // proto:  bool QXmlStreamAttribute::isDefault();
impl<'a> /*trait*/ QXmlStreamAttribute_isDefault<i8> for () {
  fn isDefault(self , rsthis: & QXmlStreamAttribute) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 80)};
    // unsafe{_ZNK19QXmlStreamAttribute9isDefaultEv()};
    let mut ret = unsafe {C_ZNK19QXmlStreamAttribute9isDefaultEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  QStringRef QXmlStreamAttribute::prefix();
impl /*struct*/ QXmlStreamAttribute {
  pub fn prefix<RetType, T: QXmlStreamAttribute_prefix<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.prefix(self);
    // return 1;
  }
}

pub trait QXmlStreamAttribute_prefix<RetType> {
  fn prefix(self , rsthis: & QXmlStreamAttribute) -> RetType;
}

  // proto:  QStringRef QXmlStreamAttribute::prefix();
impl<'a> /*trait*/ QXmlStreamAttribute_prefix<QStringRef> for () {
  fn prefix(self , rsthis: & QXmlStreamAttribute) -> QStringRef {
    // let qthis: *mut c_void = unsafe{calloc(1, 80)};
    // unsafe{_ZNK19QXmlStreamAttribute6prefixEv()};
    let mut ret = unsafe {C_ZNK19QXmlStreamAttribute6prefixEv(rsthis.qclsinst)};
    let mut ret1 = QStringRef::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QStringRef QXmlStreamAttribute::name();
impl /*struct*/ QXmlStreamAttribute {
  pub fn name<RetType, T: QXmlStreamAttribute_name<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.name(self);
    // return 1;
  }
}

pub trait QXmlStreamAttribute_name<RetType> {
  fn name(self , rsthis: & QXmlStreamAttribute) -> RetType;
}

  // proto:  QStringRef QXmlStreamAttribute::name();
impl<'a> /*trait*/ QXmlStreamAttribute_name<QStringRef> for () {
  fn name(self , rsthis: & QXmlStreamAttribute) -> QStringRef {
    // let qthis: *mut c_void = unsafe{calloc(1, 80)};
    // unsafe{_ZNK19QXmlStreamAttribute4nameEv()};
    let mut ret = unsafe {C_ZNK19QXmlStreamAttribute4nameEv(rsthis.qclsinst)};
    let mut ret1 = QStringRef::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

// <= body block end

