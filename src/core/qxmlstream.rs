// auto generated, do not modify.
// created: Mon Dec 21 22:54:38 2015
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
use super::qstring::QString; // 773
// use super::qxmlstream::QXmlStreamEntityResolver; // 773
// use super::qxmlstream::QXmlStreamAttributes; // 773
// use super::qxmlstream::QXmlStreamNamespaceDeclaration; // 773
use super::qbytearray::QByteArray; // 773
use super::qiodevice::QIODevice; // 773
use super::qtextcodec::QTextCodec; // 773
// use super::qxmlstream::QXmlStreamAttribute; // 773
// use super::qxmlstream::QXmlStreamReader; // 773
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  int QXmlStreamStringRef::size();
  fn _ZNK19QXmlStreamStringRef4sizeEv(qthis: *mut c_void) -> c_int;
  // proto:  void QXmlStreamStringRef::clear();
  fn _ZN19QXmlStreamStringRef5clearEv(qthis: *mut c_void);
  // proto:  const QString * QXmlStreamStringRef::string();
  fn _ZNK19QXmlStreamStringRef6stringEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QXmlStreamStringRef::QXmlStreamStringRef(const QString & aString);
  fn _ZN19QXmlStreamStringRefC1ERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QXmlStreamStringRef::~QXmlStreamStringRef();
  fn _ZN19QXmlStreamStringRefD0Ev(qthis: *mut c_void);
  // proto:  void QXmlStreamStringRef::QXmlStreamStringRef();
  fn _ZN19QXmlStreamStringRefC1Ev(qthis: *mut c_void);
  // proto:  int QXmlStreamStringRef::position();
  fn _ZNK19QXmlStreamStringRef8positionEv(qthis: *mut c_void) -> c_int;
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
  // proto:  QString QXmlStreamEntityResolver::resolveEntity(const QString & publicId, const QString & systemId);
  fn _ZN24QXmlStreamEntityResolver13resolveEntityERK7QStringS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  QString QXmlStreamEntityResolver::resolveUndeclaredEntity(const QString & name);
  fn _ZN24QXmlStreamEntityResolver23resolveUndeclaredEntityERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QXmlStreamEntityResolver::~QXmlStreamEntityResolver();
  fn _ZN24QXmlStreamEntityResolverD0Ev(qthis: *mut c_void);
  // proto:  QStringRef QXmlStreamNamespaceDeclaration::namespaceUri();
  fn _ZNK30QXmlStreamNamespaceDeclaration12namespaceUriEv(qthis: *mut c_void);
  // proto:  void QXmlStreamNamespaceDeclaration::QXmlStreamNamespaceDeclaration(const QXmlStreamNamespaceDeclaration & );
  fn _ZN30QXmlStreamNamespaceDeclarationC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QXmlStreamNamespaceDeclaration::QXmlStreamNamespaceDeclaration();
  fn _ZN30QXmlStreamNamespaceDeclarationC1Ev(qthis: *mut c_void);
  // proto:  QStringRef QXmlStreamNamespaceDeclaration::prefix();
  fn _ZNK30QXmlStreamNamespaceDeclaration6prefixEv(qthis: *mut c_void);
  // proto:  void QXmlStreamNamespaceDeclaration::QXmlStreamNamespaceDeclaration(const QString & prefix, const QString & namespaceUri);
  fn _ZN30QXmlStreamNamespaceDeclarationC1ERK7QStringS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QXmlStreamNamespaceDeclaration::~QXmlStreamNamespaceDeclaration();
  fn _ZN30QXmlStreamNamespaceDeclarationD0Ev(qthis: *mut c_void);
  // proto:  void QXmlStreamEntityDeclaration::~QXmlStreamEntityDeclaration();
  fn _ZN27QXmlStreamEntityDeclarationD0Ev(qthis: *mut c_void);
  // proto:  QStringRef QXmlStreamEntityDeclaration::publicId();
  fn _ZNK27QXmlStreamEntityDeclaration8publicIdEv(qthis: *mut c_void);
  // proto:  QStringRef QXmlStreamEntityDeclaration::name();
  fn _ZNK27QXmlStreamEntityDeclaration4nameEv(qthis: *mut c_void);
  // proto:  void QXmlStreamEntityDeclaration::QXmlStreamEntityDeclaration();
  fn _ZN27QXmlStreamEntityDeclarationC1Ev(qthis: *mut c_void);
  // proto:  QStringRef QXmlStreamEntityDeclaration::value();
  fn _ZNK27QXmlStreamEntityDeclaration5valueEv(qthis: *mut c_void);
  // proto:  QStringRef QXmlStreamEntityDeclaration::notationName();
  fn _ZNK27QXmlStreamEntityDeclaration12notationNameEv(qthis: *mut c_void);
  // proto:  void QXmlStreamEntityDeclaration::QXmlStreamEntityDeclaration(const QXmlStreamEntityDeclaration & );
  fn _ZN27QXmlStreamEntityDeclarationC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QStringRef QXmlStreamEntityDeclaration::systemId();
  fn _ZNK27QXmlStreamEntityDeclaration8systemIdEv(qthis: *mut c_void);
  // proto:  QStringRef QXmlStreamAttributes::value(const QString & qualifiedName);
  fn _ZNK20QXmlStreamAttributes5valueERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QXmlStreamAttributes::QXmlStreamAttributes();
  fn _ZN20QXmlStreamAttributesC1Ev(qthis: *mut c_void);
  // proto:  bool QXmlStreamAttributes::hasAttribute(const QString & qualifiedName);
  fn _ZNK20QXmlStreamAttributes12hasAttributeERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  bool QXmlStreamAttributes::hasAttribute(const QString & namespaceUri, const QString & name);
  fn _ZNK20QXmlStreamAttributes12hasAttributeERK7QStringS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> c_char;
  // proto:  void QXmlStreamAttributes::append(const QString & namespaceUri, const QString & name, const QString & value);
  fn _ZN20QXmlStreamAttributes6appendERK7QStringS2_S2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void);
  // proto:  void QXmlStreamAttributes::append(const QString & qualifiedName, const QString & value);
  fn _ZN20QXmlStreamAttributes6appendERK7QStringS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  QStringRef QXmlStreamAttributes::value(const QString & namespaceUri, const QString & name);
  fn _ZNK20QXmlStreamAttributes5valueERK7QStringS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QXmlStreamWriter::writeEndElement();
  fn _ZN16QXmlStreamWriter15writeEndElementEv(qthis: *mut c_void);
  // proto:  void QXmlStreamWriter::QXmlStreamWriter();
  fn _ZN16QXmlStreamWriterC1Ev(qthis: *mut c_void);
  // proto:  void QXmlStreamWriter::writeEndDocument();
  fn _ZN16QXmlStreamWriter16writeEndDocumentEv(qthis: *mut c_void);
  // proto:  bool QXmlStreamWriter::autoFormatting();
  fn _ZNK16QXmlStreamWriter14autoFormattingEv(qthis: *mut c_void) -> c_char;
  // proto:  void QXmlStreamWriter::writeStartDocument(const QString & version, bool standalone);
  fn _ZN16QXmlStreamWriter18writeStartDocumentERK7QStringb(qthis: *mut c_void, arg0: *mut c_void, arg1: c_char);
  // proto:  void QXmlStreamWriter::setCodec(QTextCodec * codec);
  fn _ZN16QXmlStreamWriter8setCodecEP10QTextCodec(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QXmlStreamWriter::writeProcessingInstruction(const QString & target, const QString & data);
  fn _ZN16QXmlStreamWriter26writeProcessingInstructionERK7QStringS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QXmlStreamWriter::writeCharacters(const QString & text);
  fn _ZN16QXmlStreamWriter15writeCharactersERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QXmlStreamWriter::setDevice(QIODevice * device);
  fn _ZN16QXmlStreamWriter9setDeviceEP9QIODevice(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QXmlStreamWriter::writeStartDocument(const QString & version);
  fn _ZN16QXmlStreamWriter18writeStartDocumentERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QXmlStreamWriter::writeTextElement(const QString & namespaceUri, const QString & name, const QString & text);
  fn _ZN16QXmlStreamWriter16writeTextElementERK7QStringS2_S2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void);
  // proto:  void QXmlStreamWriter::writeAttribute(const QString & qualifiedName, const QString & value);
  fn _ZN16QXmlStreamWriter14writeAttributeERK7QStringS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QXmlStreamWriter::writeEmptyElement(const QString & qualifiedName);
  fn _ZN16QXmlStreamWriter17writeEmptyElementERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QXmlStreamWriter::writeDTD(const QString & dtd);
  fn _ZN16QXmlStreamWriter8writeDTDERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QXmlStreamWriter::setAutoFormattingIndent(int spacesOrTabs);
  fn _ZN16QXmlStreamWriter23setAutoFormattingIndentEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QXmlStreamWriter::writeAttribute(const QXmlStreamAttribute & attribute);
  fn _ZN16QXmlStreamWriter14writeAttributeERK19QXmlStreamAttribute(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QXmlStreamWriter::writeStartElement(const QString & namespaceUri, const QString & name);
  fn _ZN16QXmlStreamWriter17writeStartElementERK7QStringS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QXmlStreamWriter::QXmlStreamWriter(QString * string);
  fn _ZN16QXmlStreamWriterC1EP7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QXmlStreamWriter::writeComment(const QString & text);
  fn _ZN16QXmlStreamWriter12writeCommentERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QTextCodec * QXmlStreamWriter::codec();
  fn _ZNK16QXmlStreamWriter5codecEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QXmlStreamWriter::writeAttribute(const QString & namespaceUri, const QString & name, const QString & value);
  fn _ZN16QXmlStreamWriter14writeAttributeERK7QStringS2_S2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void);
  // proto:  void QXmlStreamWriter::writeNamespace(const QString & namespaceUri, const QString & prefix);
  fn _ZN16QXmlStreamWriter14writeNamespaceERK7QStringS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QXmlStreamWriter::QXmlStreamWriter(const QXmlStreamWriter & );
  fn _ZN16QXmlStreamWriterC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QXmlStreamWriter::hasError();
  fn _ZNK16QXmlStreamWriter8hasErrorEv(qthis: *mut c_void) -> c_char;
  // proto:  void QXmlStreamWriter::writeCDATA(const QString & text);
  fn _ZN16QXmlStreamWriter10writeCDATAERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QXmlStreamWriter::writeStartDocument();
  fn _ZN16QXmlStreamWriter18writeStartDocumentEv(qthis: *mut c_void);
  // proto:  void QXmlStreamWriter::writeEntityReference(const QString & name);
  fn _ZN16QXmlStreamWriter20writeEntityReferenceERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QXmlStreamWriter::setAutoFormatting(bool );
  fn _ZN16QXmlStreamWriter17setAutoFormattingEb(qthis: *mut c_void, arg0: c_char);
  // proto:  void QXmlStreamWriter::setCodec(const char * codecName);
  fn _ZN16QXmlStreamWriter8setCodecEPKc(qthis: *mut c_void, arg0: *mut c_char);
  // proto:  int QXmlStreamWriter::autoFormattingIndent();
  fn _ZNK16QXmlStreamWriter20autoFormattingIndentEv(qthis: *mut c_void) -> c_int;
  // proto:  void QXmlStreamWriter::~QXmlStreamWriter();
  fn _ZN16QXmlStreamWriterD0Ev(qthis: *mut c_void);
  // proto:  void QXmlStreamWriter::writeAttributes(const QXmlStreamAttributes & attributes);
  fn _ZN16QXmlStreamWriter15writeAttributesERK20QXmlStreamAttributes(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QXmlStreamWriter::writeDefaultNamespace(const QString & namespaceUri);
  fn _ZN16QXmlStreamWriter21writeDefaultNamespaceERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QIODevice * QXmlStreamWriter::device();
  fn _ZNK16QXmlStreamWriter6deviceEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QXmlStreamWriter::writeCurrentToken(const QXmlStreamReader & reader);
  fn _ZN16QXmlStreamWriter17writeCurrentTokenERK16QXmlStreamReader(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QXmlStreamWriter::QXmlStreamWriter(QByteArray * array);
  fn _ZN16QXmlStreamWriterC1EP10QByteArray(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QXmlStreamWriter::writeTextElement(const QString & qualifiedName, const QString & text);
  fn _ZN16QXmlStreamWriter16writeTextElementERK7QStringS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QXmlStreamWriter::writeEmptyElement(const QString & namespaceUri, const QString & name);
  fn _ZN16QXmlStreamWriter17writeEmptyElementERK7QStringS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QXmlStreamWriter::QXmlStreamWriter(QIODevice * device);
  fn _ZN16QXmlStreamWriterC1EP9QIODevice(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QXmlStreamWriter::writeStartElement(const QString & qualifiedName);
  fn _ZN16QXmlStreamWriter17writeStartElementERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QXmlStreamNotationDeclaration::QXmlStreamNotationDeclaration(const QXmlStreamNotationDeclaration & );
  fn _ZN29QXmlStreamNotationDeclarationC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QStringRef QXmlStreamNotationDeclaration::publicId();
  fn _ZNK29QXmlStreamNotationDeclaration8publicIdEv(qthis: *mut c_void);
  // proto:  QStringRef QXmlStreamNotationDeclaration::name();
  fn _ZNK29QXmlStreamNotationDeclaration4nameEv(qthis: *mut c_void);
  // proto:  void QXmlStreamNotationDeclaration::~QXmlStreamNotationDeclaration();
  fn _ZN29QXmlStreamNotationDeclarationD0Ev(qthis: *mut c_void);
  // proto:  QStringRef QXmlStreamNotationDeclaration::systemId();
  fn _ZNK29QXmlStreamNotationDeclaration8systemIdEv(qthis: *mut c_void);
  // proto:  void QXmlStreamNotationDeclaration::QXmlStreamNotationDeclaration();
  fn _ZN29QXmlStreamNotationDeclarationC1Ev(qthis: *mut c_void);
  // proto:  void QXmlStreamAttribute::QXmlStreamAttribute(const QString & qualifiedName, const QString & value);
  fn _ZN19QXmlStreamAttributeC1ERK7QStringS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  QStringRef QXmlStreamAttribute::qualifiedName();
  fn _ZNK19QXmlStreamAttribute13qualifiedNameEv(qthis: *mut c_void);
  // proto:  void QXmlStreamAttribute::~QXmlStreamAttribute();
  fn _ZN19QXmlStreamAttributeD0Ev(qthis: *mut c_void);
  // proto:  QStringRef QXmlStreamAttribute::value();
  fn _ZNK19QXmlStreamAttribute5valueEv(qthis: *mut c_void);
  // proto:  QStringRef QXmlStreamAttribute::namespaceUri();
  fn _ZNK19QXmlStreamAttribute12namespaceUriEv(qthis: *mut c_void);
  // proto:  void QXmlStreamAttribute::QXmlStreamAttribute();
  fn _ZN19QXmlStreamAttributeC1Ev(qthis: *mut c_void);
  // proto:  void QXmlStreamAttribute::QXmlStreamAttribute(const QXmlStreamAttribute & );
  fn _ZN19QXmlStreamAttributeC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QXmlStreamAttribute::QXmlStreamAttribute(const QString & namespaceUri, const QString & name, const QString & value);
  fn _ZN19QXmlStreamAttributeC1ERK7QStringS2_S2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void);
  // proto:  bool QXmlStreamAttribute::isDefault();
  fn _ZNK19QXmlStreamAttribute9isDefaultEv(qthis: *mut c_void) -> c_char;
  // proto:  QStringRef QXmlStreamAttribute::prefix();
  fn _ZNK19QXmlStreamAttribute6prefixEv(qthis: *mut c_void);
  // proto:  QStringRef QXmlStreamAttribute::name();
  fn _ZNK19QXmlStreamAttribute4nameEv(qthis: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QXmlStreamStringRef)=16
pub struct QXmlStreamStringRef {
  pub qclsinst: *mut c_void,
}

// class sizeof(QXmlStreamReader)=1
pub struct QXmlStreamReader {
  pub qclsinst: *mut c_void,
}

// class sizeof(QXmlStreamEntityResolver)=8
pub struct QXmlStreamEntityResolver {
  pub qclsinst: *mut c_void,
}

// class sizeof(QXmlStreamNamespaceDeclaration)=40
pub struct QXmlStreamNamespaceDeclaration {
  pub qclsinst: *mut c_void,
}

// class sizeof(QXmlStreamEntityDeclaration)=88
pub struct QXmlStreamEntityDeclaration {
  pub qclsinst: *mut c_void,
}

// class sizeof(QXmlStreamAttributes)=1
pub struct QXmlStreamAttributes {
  pub qclsinst: *mut c_void,
}

// class sizeof(QXmlStreamWriter)=1
pub struct QXmlStreamWriter {
  pub qclsinst: *mut c_void,
}

// class sizeof(QXmlStreamNotationDeclaration)=56
pub struct QXmlStreamNotationDeclaration {
  pub qclsinst: *mut c_void,
}

// class sizeof(QXmlStreamAttribute)=80
pub struct QXmlStreamAttribute {
  pub qclsinst: *mut c_void,
}

  // proto:  int QXmlStreamStringRef::size();
impl /*struct*/ QXmlStreamStringRef {
  pub fn size<RetType, T: QXmlStreamStringRef_size<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.size(self);
    // return 1;
  }
}

pub trait QXmlStreamStringRef_size<RetType> {
  fn size(self , rsthis: &mut QXmlStreamStringRef) -> RetType;
}

  // proto:  int QXmlStreamStringRef::size();
impl<'a> /*trait*/ QXmlStreamStringRef_size<i32> for () {
  fn size(self , rsthis: &mut QXmlStreamStringRef) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QXmlStreamStringRef4sizeEv()};
    let mut ret = unsafe {_ZNK19QXmlStreamStringRef4sizeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QXmlStreamStringRef::clear();
impl /*struct*/ QXmlStreamStringRef {
  pub fn clear<RetType, T: QXmlStreamStringRef_clear<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.clear(self);
    // return 1;
  }
}

pub trait QXmlStreamStringRef_clear<RetType> {
  fn clear(self , rsthis: &mut QXmlStreamStringRef) -> RetType;
}

  // proto:  void QXmlStreamStringRef::clear();
impl<'a> /*trait*/ QXmlStreamStringRef_clear<()> for () {
  fn clear(self , rsthis: &mut QXmlStreamStringRef) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QXmlStreamStringRef5clearEv()};
     unsafe {_ZN19QXmlStreamStringRef5clearEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  const QString * QXmlStreamStringRef::string();
impl /*struct*/ QXmlStreamStringRef {
  pub fn string<RetType, T: QXmlStreamStringRef_string<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.string(self);
    // return 1;
  }
}

pub trait QXmlStreamStringRef_string<RetType> {
  fn string(self , rsthis: &mut QXmlStreamStringRef) -> RetType;
}

  // proto:  const QString * QXmlStreamStringRef::string();
impl<'a> /*trait*/ QXmlStreamStringRef_string<QString> for () {
  fn string(self , rsthis: &mut QXmlStreamStringRef) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QXmlStreamStringRef6stringEv()};
    let mut ret = unsafe {_ZNK19QXmlStreamStringRef6stringEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QXmlStreamStringRef::QXmlStreamStringRef(const QString & aString);
impl /*struct*/ QXmlStreamStringRef {
  pub fn NewQXmlStreamStringRef<T: QXmlStreamStringRef_NewQXmlStreamStringRef>(value: T) -> QXmlStreamStringRef {
    let rsthis = value.NewQXmlStreamStringRef();
    return rsthis;
    // return 1;
  }
}

pub trait QXmlStreamStringRef_NewQXmlStreamStringRef {
  fn NewQXmlStreamStringRef(self) -> QXmlStreamStringRef;
}

  // proto:  void QXmlStreamStringRef::QXmlStreamStringRef(const QString & aString);
impl<'a> /*trait*/ QXmlStreamStringRef_NewQXmlStreamStringRef for (QString) {
  fn NewQXmlStreamStringRef(self) -> QXmlStreamStringRef {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QXmlStreamStringRefC1ERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN19QXmlStreamStringRefC1ERK7QString(qthis, arg0)};
    let rsthis = QXmlStreamStringRef{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QXmlStreamStringRef::~QXmlStreamStringRef();
impl /*struct*/ QXmlStreamStringRef {
  pub fn FreeQXmlStreamStringRef<RetType, T: QXmlStreamStringRef_FreeQXmlStreamStringRef<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQXmlStreamStringRef(self);
    // return 1;
  }
}

pub trait QXmlStreamStringRef_FreeQXmlStreamStringRef<RetType> {
  fn FreeQXmlStreamStringRef(self , rsthis: &mut QXmlStreamStringRef) -> RetType;
}

  // proto:  void QXmlStreamStringRef::~QXmlStreamStringRef();
impl<'a> /*trait*/ QXmlStreamStringRef_FreeQXmlStreamStringRef<()> for () {
  fn FreeQXmlStreamStringRef(self , rsthis: &mut QXmlStreamStringRef) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QXmlStreamStringRefD0Ev()};
     unsafe {_ZN19QXmlStreamStringRefD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QXmlStreamStringRef::QXmlStreamStringRef();
impl<'a> /*trait*/ QXmlStreamStringRef_NewQXmlStreamStringRef for () {
  fn NewQXmlStreamStringRef(self) -> QXmlStreamStringRef {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QXmlStreamStringRefC1Ev()};
    unsafe {_ZN19QXmlStreamStringRefC1Ev(qthis)};
    let rsthis = QXmlStreamStringRef{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  int QXmlStreamStringRef::position();
impl /*struct*/ QXmlStreamStringRef {
  pub fn position<RetType, T: QXmlStreamStringRef_position<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.position(self);
    // return 1;
  }
}

pub trait QXmlStreamStringRef_position<RetType> {
  fn position(self , rsthis: &mut QXmlStreamStringRef) -> RetType;
}

  // proto:  int QXmlStreamStringRef::position();
impl<'a> /*trait*/ QXmlStreamStringRef_position<i32> for () {
  fn position(self , rsthis: &mut QXmlStreamStringRef) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QXmlStreamStringRef8positionEv()};
    let mut ret = unsafe {_ZNK19QXmlStreamStringRef8positionEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
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

  // proto:  QString QXmlStreamEntityResolver::resolveEntity(const QString & publicId, const QString & systemId);
impl /*struct*/ QXmlStreamEntityResolver {
  pub fn resolveEntity<RetType, T: QXmlStreamEntityResolver_resolveEntity<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.resolveEntity(self);
    // return 1;
  }
}

pub trait QXmlStreamEntityResolver_resolveEntity<RetType> {
  fn resolveEntity(self , rsthis: &mut QXmlStreamEntityResolver) -> RetType;
}

  // proto:  QString QXmlStreamEntityResolver::resolveEntity(const QString & publicId, const QString & systemId);
impl<'a> /*trait*/ QXmlStreamEntityResolver_resolveEntity<QString> for (QString, QString) {
  fn resolveEntity(self , rsthis: &mut QXmlStreamEntityResolver) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QXmlStreamEntityResolver13resolveEntityERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN24QXmlStreamEntityResolver13resolveEntityERK7QStringS2_(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QString QXmlStreamEntityResolver::resolveUndeclaredEntity(const QString & name);
impl /*struct*/ QXmlStreamEntityResolver {
  pub fn resolveUndeclaredEntity<RetType, T: QXmlStreamEntityResolver_resolveUndeclaredEntity<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.resolveUndeclaredEntity(self);
    // return 1;
  }
}

pub trait QXmlStreamEntityResolver_resolveUndeclaredEntity<RetType> {
  fn resolveUndeclaredEntity(self , rsthis: &mut QXmlStreamEntityResolver) -> RetType;
}

  // proto:  QString QXmlStreamEntityResolver::resolveUndeclaredEntity(const QString & name);
impl<'a> /*trait*/ QXmlStreamEntityResolver_resolveUndeclaredEntity<QString> for (QString) {
  fn resolveUndeclaredEntity(self , rsthis: &mut QXmlStreamEntityResolver) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QXmlStreamEntityResolver23resolveUndeclaredEntityERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN24QXmlStreamEntityResolver23resolveUndeclaredEntityERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QXmlStreamEntityResolver::~QXmlStreamEntityResolver();
impl /*struct*/ QXmlStreamEntityResolver {
  pub fn FreeQXmlStreamEntityResolver<RetType, T: QXmlStreamEntityResolver_FreeQXmlStreamEntityResolver<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQXmlStreamEntityResolver(self);
    // return 1;
  }
}

pub trait QXmlStreamEntityResolver_FreeQXmlStreamEntityResolver<RetType> {
  fn FreeQXmlStreamEntityResolver(self , rsthis: &mut QXmlStreamEntityResolver) -> RetType;
}

  // proto:  void QXmlStreamEntityResolver::~QXmlStreamEntityResolver();
impl<'a> /*trait*/ QXmlStreamEntityResolver_FreeQXmlStreamEntityResolver<()> for () {
  fn FreeQXmlStreamEntityResolver(self , rsthis: &mut QXmlStreamEntityResolver) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QXmlStreamEntityResolverD0Ev()};
     unsafe {_ZN24QXmlStreamEntityResolverD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QStringRef QXmlStreamNamespaceDeclaration::namespaceUri();
impl /*struct*/ QXmlStreamNamespaceDeclaration {
  pub fn namespaceUri<RetType, T: QXmlStreamNamespaceDeclaration_namespaceUri<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.namespaceUri(self);
    // return 1;
  }
}

pub trait QXmlStreamNamespaceDeclaration_namespaceUri<RetType> {
  fn namespaceUri(self , rsthis: &mut QXmlStreamNamespaceDeclaration) -> RetType;
}

  // proto:  QStringRef QXmlStreamNamespaceDeclaration::namespaceUri();
impl<'a> /*trait*/ QXmlStreamNamespaceDeclaration_namespaceUri<()> for () {
  fn namespaceUri(self , rsthis: &mut QXmlStreamNamespaceDeclaration) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZNK30QXmlStreamNamespaceDeclaration12namespaceUriEv()};
     unsafe {_ZNK30QXmlStreamNamespaceDeclaration12namespaceUriEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QXmlStreamNamespaceDeclaration::QXmlStreamNamespaceDeclaration(const QXmlStreamNamespaceDeclaration & );
impl /*struct*/ QXmlStreamNamespaceDeclaration {
  pub fn NewQXmlStreamNamespaceDeclaration<T: QXmlStreamNamespaceDeclaration_NewQXmlStreamNamespaceDeclaration>(value: T) -> QXmlStreamNamespaceDeclaration {
    let rsthis = value.NewQXmlStreamNamespaceDeclaration();
    return rsthis;
    // return 1;
  }
}

pub trait QXmlStreamNamespaceDeclaration_NewQXmlStreamNamespaceDeclaration {
  fn NewQXmlStreamNamespaceDeclaration(self) -> QXmlStreamNamespaceDeclaration;
}

  // proto:  void QXmlStreamNamespaceDeclaration::QXmlStreamNamespaceDeclaration(const QXmlStreamNamespaceDeclaration & );
impl<'a> /*trait*/ QXmlStreamNamespaceDeclaration_NewQXmlStreamNamespaceDeclaration for (QXmlStreamNamespaceDeclaration) {
  fn NewQXmlStreamNamespaceDeclaration(self) -> QXmlStreamNamespaceDeclaration {
    let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZN30QXmlStreamNamespaceDeclarationC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN30QXmlStreamNamespaceDeclarationC1ERKS_(qthis, arg0)};
    let rsthis = QXmlStreamNamespaceDeclaration{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QXmlStreamNamespaceDeclaration::QXmlStreamNamespaceDeclaration();
impl<'a> /*trait*/ QXmlStreamNamespaceDeclaration_NewQXmlStreamNamespaceDeclaration for () {
  fn NewQXmlStreamNamespaceDeclaration(self) -> QXmlStreamNamespaceDeclaration {
    let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZN30QXmlStreamNamespaceDeclarationC1Ev()};
    unsafe {_ZN30QXmlStreamNamespaceDeclarationC1Ev(qthis)};
    let rsthis = QXmlStreamNamespaceDeclaration{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QStringRef QXmlStreamNamespaceDeclaration::prefix();
impl /*struct*/ QXmlStreamNamespaceDeclaration {
  pub fn prefix<RetType, T: QXmlStreamNamespaceDeclaration_prefix<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.prefix(self);
    // return 1;
  }
}

pub trait QXmlStreamNamespaceDeclaration_prefix<RetType> {
  fn prefix(self , rsthis: &mut QXmlStreamNamespaceDeclaration) -> RetType;
}

  // proto:  QStringRef QXmlStreamNamespaceDeclaration::prefix();
impl<'a> /*trait*/ QXmlStreamNamespaceDeclaration_prefix<()> for () {
  fn prefix(self , rsthis: &mut QXmlStreamNamespaceDeclaration) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZNK30QXmlStreamNamespaceDeclaration6prefixEv()};
     unsafe {_ZNK30QXmlStreamNamespaceDeclaration6prefixEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QXmlStreamNamespaceDeclaration::QXmlStreamNamespaceDeclaration(const QString & prefix, const QString & namespaceUri);
impl<'a> /*trait*/ QXmlStreamNamespaceDeclaration_NewQXmlStreamNamespaceDeclaration for (QString, QString) {
  fn NewQXmlStreamNamespaceDeclaration(self) -> QXmlStreamNamespaceDeclaration {
    let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZN30QXmlStreamNamespaceDeclarationC1ERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN30QXmlStreamNamespaceDeclarationC1ERK7QStringS2_(qthis, arg0, arg1)};
    let rsthis = QXmlStreamNamespaceDeclaration{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QXmlStreamNamespaceDeclaration::~QXmlStreamNamespaceDeclaration();
impl /*struct*/ QXmlStreamNamespaceDeclaration {
  pub fn FreeQXmlStreamNamespaceDeclaration<RetType, T: QXmlStreamNamespaceDeclaration_FreeQXmlStreamNamespaceDeclaration<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQXmlStreamNamespaceDeclaration(self);
    // return 1;
  }
}

pub trait QXmlStreamNamespaceDeclaration_FreeQXmlStreamNamespaceDeclaration<RetType> {
  fn FreeQXmlStreamNamespaceDeclaration(self , rsthis: &mut QXmlStreamNamespaceDeclaration) -> RetType;
}

  // proto:  void QXmlStreamNamespaceDeclaration::~QXmlStreamNamespaceDeclaration();
impl<'a> /*trait*/ QXmlStreamNamespaceDeclaration_FreeQXmlStreamNamespaceDeclaration<()> for () {
  fn FreeQXmlStreamNamespaceDeclaration(self , rsthis: &mut QXmlStreamNamespaceDeclaration) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZN30QXmlStreamNamespaceDeclarationD0Ev()};
     unsafe {_ZN30QXmlStreamNamespaceDeclarationD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QXmlStreamEntityDeclaration::~QXmlStreamEntityDeclaration();
impl /*struct*/ QXmlStreamEntityDeclaration {
  pub fn FreeQXmlStreamEntityDeclaration<RetType, T: QXmlStreamEntityDeclaration_FreeQXmlStreamEntityDeclaration<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQXmlStreamEntityDeclaration(self);
    // return 1;
  }
}

pub trait QXmlStreamEntityDeclaration_FreeQXmlStreamEntityDeclaration<RetType> {
  fn FreeQXmlStreamEntityDeclaration(self , rsthis: &mut QXmlStreamEntityDeclaration) -> RetType;
}

  // proto:  void QXmlStreamEntityDeclaration::~QXmlStreamEntityDeclaration();
impl<'a> /*trait*/ QXmlStreamEntityDeclaration_FreeQXmlStreamEntityDeclaration<()> for () {
  fn FreeQXmlStreamEntityDeclaration(self , rsthis: &mut QXmlStreamEntityDeclaration) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZN27QXmlStreamEntityDeclarationD0Ev()};
     unsafe {_ZN27QXmlStreamEntityDeclarationD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QStringRef QXmlStreamEntityDeclaration::publicId();
impl /*struct*/ QXmlStreamEntityDeclaration {
  pub fn publicId<RetType, T: QXmlStreamEntityDeclaration_publicId<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.publicId(self);
    // return 1;
  }
}

pub trait QXmlStreamEntityDeclaration_publicId<RetType> {
  fn publicId(self , rsthis: &mut QXmlStreamEntityDeclaration) -> RetType;
}

  // proto:  QStringRef QXmlStreamEntityDeclaration::publicId();
impl<'a> /*trait*/ QXmlStreamEntityDeclaration_publicId<()> for () {
  fn publicId(self , rsthis: &mut QXmlStreamEntityDeclaration) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK27QXmlStreamEntityDeclaration8publicIdEv()};
     unsafe {_ZNK27QXmlStreamEntityDeclaration8publicIdEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QStringRef QXmlStreamEntityDeclaration::name();
impl /*struct*/ QXmlStreamEntityDeclaration {
  pub fn name<RetType, T: QXmlStreamEntityDeclaration_name<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.name(self);
    // return 1;
  }
}

pub trait QXmlStreamEntityDeclaration_name<RetType> {
  fn name(self , rsthis: &mut QXmlStreamEntityDeclaration) -> RetType;
}

  // proto:  QStringRef QXmlStreamEntityDeclaration::name();
impl<'a> /*trait*/ QXmlStreamEntityDeclaration_name<()> for () {
  fn name(self , rsthis: &mut QXmlStreamEntityDeclaration) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK27QXmlStreamEntityDeclaration4nameEv()};
     unsafe {_ZNK27QXmlStreamEntityDeclaration4nameEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QXmlStreamEntityDeclaration::QXmlStreamEntityDeclaration();
impl /*struct*/ QXmlStreamEntityDeclaration {
  pub fn NewQXmlStreamEntityDeclaration<T: QXmlStreamEntityDeclaration_NewQXmlStreamEntityDeclaration>(value: T) -> QXmlStreamEntityDeclaration {
    let rsthis = value.NewQXmlStreamEntityDeclaration();
    return rsthis;
    // return 1;
  }
}

pub trait QXmlStreamEntityDeclaration_NewQXmlStreamEntityDeclaration {
  fn NewQXmlStreamEntityDeclaration(self) -> QXmlStreamEntityDeclaration;
}

  // proto:  void QXmlStreamEntityDeclaration::QXmlStreamEntityDeclaration();
impl<'a> /*trait*/ QXmlStreamEntityDeclaration_NewQXmlStreamEntityDeclaration for () {
  fn NewQXmlStreamEntityDeclaration(self) -> QXmlStreamEntityDeclaration {
    let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZN27QXmlStreamEntityDeclarationC1Ev()};
    unsafe {_ZN27QXmlStreamEntityDeclarationC1Ev(qthis)};
    let rsthis = QXmlStreamEntityDeclaration{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QStringRef QXmlStreamEntityDeclaration::value();
impl /*struct*/ QXmlStreamEntityDeclaration {
  pub fn value<RetType, T: QXmlStreamEntityDeclaration_value<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.value(self);
    // return 1;
  }
}

pub trait QXmlStreamEntityDeclaration_value<RetType> {
  fn value(self , rsthis: &mut QXmlStreamEntityDeclaration) -> RetType;
}

  // proto:  QStringRef QXmlStreamEntityDeclaration::value();
impl<'a> /*trait*/ QXmlStreamEntityDeclaration_value<()> for () {
  fn value(self , rsthis: &mut QXmlStreamEntityDeclaration) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK27QXmlStreamEntityDeclaration5valueEv()};
     unsafe {_ZNK27QXmlStreamEntityDeclaration5valueEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QStringRef QXmlStreamEntityDeclaration::notationName();
impl /*struct*/ QXmlStreamEntityDeclaration {
  pub fn notationName<RetType, T: QXmlStreamEntityDeclaration_notationName<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.notationName(self);
    // return 1;
  }
}

pub trait QXmlStreamEntityDeclaration_notationName<RetType> {
  fn notationName(self , rsthis: &mut QXmlStreamEntityDeclaration) -> RetType;
}

  // proto:  QStringRef QXmlStreamEntityDeclaration::notationName();
impl<'a> /*trait*/ QXmlStreamEntityDeclaration_notationName<()> for () {
  fn notationName(self , rsthis: &mut QXmlStreamEntityDeclaration) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK27QXmlStreamEntityDeclaration12notationNameEv()};
     unsafe {_ZNK27QXmlStreamEntityDeclaration12notationNameEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QXmlStreamEntityDeclaration::QXmlStreamEntityDeclaration(const QXmlStreamEntityDeclaration & );
impl<'a> /*trait*/ QXmlStreamEntityDeclaration_NewQXmlStreamEntityDeclaration for (QXmlStreamEntityDeclaration) {
  fn NewQXmlStreamEntityDeclaration(self) -> QXmlStreamEntityDeclaration {
    let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZN27QXmlStreamEntityDeclarationC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN27QXmlStreamEntityDeclarationC1ERKS_(qthis, arg0)};
    let rsthis = QXmlStreamEntityDeclaration{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QStringRef QXmlStreamEntityDeclaration::systemId();
impl /*struct*/ QXmlStreamEntityDeclaration {
  pub fn systemId<RetType, T: QXmlStreamEntityDeclaration_systemId<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.systemId(self);
    // return 1;
  }
}

pub trait QXmlStreamEntityDeclaration_systemId<RetType> {
  fn systemId(self , rsthis: &mut QXmlStreamEntityDeclaration) -> RetType;
}

  // proto:  QStringRef QXmlStreamEntityDeclaration::systemId();
impl<'a> /*trait*/ QXmlStreamEntityDeclaration_systemId<()> for () {
  fn systemId(self , rsthis: &mut QXmlStreamEntityDeclaration) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 88)};
    // unsafe{_ZNK27QXmlStreamEntityDeclaration8systemIdEv()};
     unsafe {_ZNK27QXmlStreamEntityDeclaration8systemIdEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QStringRef QXmlStreamAttributes::value(const QString & qualifiedName);
impl /*struct*/ QXmlStreamAttributes {
  pub fn value<RetType, T: QXmlStreamAttributes_value<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.value(self);
    // return 1;
  }
}

pub trait QXmlStreamAttributes_value<RetType> {
  fn value(self , rsthis: &mut QXmlStreamAttributes) -> RetType;
}

  // proto:  QStringRef QXmlStreamAttributes::value(const QString & qualifiedName);
impl<'a> /*trait*/ QXmlStreamAttributes_value<()> for (QString) {
  fn value(self , rsthis: &mut QXmlStreamAttributes) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QXmlStreamAttributes5valueERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZNK20QXmlStreamAttributes5valueERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QXmlStreamAttributes::QXmlStreamAttributes();
impl /*struct*/ QXmlStreamAttributes {
  pub fn NewQXmlStreamAttributes<T: QXmlStreamAttributes_NewQXmlStreamAttributes>(value: T) -> QXmlStreamAttributes {
    let rsthis = value.NewQXmlStreamAttributes();
    return rsthis;
    // return 1;
  }
}

pub trait QXmlStreamAttributes_NewQXmlStreamAttributes {
  fn NewQXmlStreamAttributes(self) -> QXmlStreamAttributes;
}

  // proto:  void QXmlStreamAttributes::QXmlStreamAttributes();
impl<'a> /*trait*/ QXmlStreamAttributes_NewQXmlStreamAttributes for () {
  fn NewQXmlStreamAttributes(self) -> QXmlStreamAttributes {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QXmlStreamAttributesC1Ev()};
    unsafe {_ZN20QXmlStreamAttributesC1Ev(qthis)};
    let rsthis = QXmlStreamAttributes{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QXmlStreamAttributes::hasAttribute(const QString & qualifiedName);
impl /*struct*/ QXmlStreamAttributes {
  pub fn hasAttribute<RetType, T: QXmlStreamAttributes_hasAttribute<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.hasAttribute(self);
    // return 1;
  }
}

pub trait QXmlStreamAttributes_hasAttribute<RetType> {
  fn hasAttribute(self , rsthis: &mut QXmlStreamAttributes) -> RetType;
}

  // proto:  bool QXmlStreamAttributes::hasAttribute(const QString & qualifiedName);
impl<'a> /*trait*/ QXmlStreamAttributes_hasAttribute<i8> for (QString) {
  fn hasAttribute(self , rsthis: &mut QXmlStreamAttributes) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QXmlStreamAttributes12hasAttributeERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK20QXmlStreamAttributes12hasAttributeERK7QString(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QXmlStreamAttributes::hasAttribute(const QString & namespaceUri, const QString & name);
impl<'a> /*trait*/ QXmlStreamAttributes_hasAttribute<i8> for (QString, QString) {
  fn hasAttribute(self , rsthis: &mut QXmlStreamAttributes) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QXmlStreamAttributes12hasAttributeERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK20QXmlStreamAttributes12hasAttributeERK7QStringS2_(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QXmlStreamAttributes::append(const QString & namespaceUri, const QString & name, const QString & value);
impl /*struct*/ QXmlStreamAttributes {
  pub fn append<RetType, T: QXmlStreamAttributes_append<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.append(self);
    // return 1;
  }
}

pub trait QXmlStreamAttributes_append<RetType> {
  fn append(self , rsthis: &mut QXmlStreamAttributes) -> RetType;
}

  // proto:  void QXmlStreamAttributes::append(const QString & namespaceUri, const QString & name, const QString & value);
impl<'a> /*trait*/ QXmlStreamAttributes_append<()> for (QString, QString, QString) {
  fn append(self , rsthis: &mut QXmlStreamAttributes) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QXmlStreamAttributes6appendERK7QStringS2_S2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN20QXmlStreamAttributes6appendERK7QStringS2_S2_(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QXmlStreamAttributes::append(const QString & qualifiedName, const QString & value);
impl<'a> /*trait*/ QXmlStreamAttributes_append<()> for (QString, QString) {
  fn append(self , rsthis: &mut QXmlStreamAttributes) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QXmlStreamAttributes6appendERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN20QXmlStreamAttributes6appendERK7QStringS2_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QStringRef QXmlStreamAttributes::value(const QString & namespaceUri, const QString & name);
impl<'a> /*trait*/ QXmlStreamAttributes_value<()> for (QString, QString) {
  fn value(self , rsthis: &mut QXmlStreamAttributes) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QXmlStreamAttributes5valueERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZNK20QXmlStreamAttributes5valueERK7QStringS2_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QXmlStreamWriter::writeEndElement();
impl /*struct*/ QXmlStreamWriter {
  pub fn writeEndElement<RetType, T: QXmlStreamWriter_writeEndElement<RetType>>(&mut self,  overload_args: T) -> RetType {
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

  // proto:  void QXmlStreamWriter::QXmlStreamWriter();
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

  // proto:  void QXmlStreamWriter::QXmlStreamWriter();
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
  pub fn writeEndDocument<RetType, T: QXmlStreamWriter_writeEndDocument<RetType>>(&mut self,  overload_args: T) -> RetType {
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
  pub fn autoFormatting<RetType, T: QXmlStreamWriter_autoFormatting<RetType>>(&mut self,  overload_args: T) -> RetType {
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
  pub fn writeStartDocument<RetType, T: QXmlStreamWriter_writeStartDocument<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.writeStartDocument(self);
    // return 1;
  }
}

pub trait QXmlStreamWriter_writeStartDocument<RetType> {
  fn writeStartDocument(self , rsthis: &mut QXmlStreamWriter) -> RetType;
}

  // proto:  void QXmlStreamWriter::writeStartDocument(const QString & version, bool standalone);
impl<'a> /*trait*/ QXmlStreamWriter_writeStartDocument<()> for (QString, i8) {
  fn writeStartDocument(self , rsthis: &mut QXmlStreamWriter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriter18writeStartDocumentERK7QStringb()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_char;
     unsafe {_ZN16QXmlStreamWriter18writeStartDocumentERK7QStringb(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QXmlStreamWriter::setCodec(QTextCodec * codec);
impl /*struct*/ QXmlStreamWriter {
  pub fn setCodec<RetType, T: QXmlStreamWriter_setCodec<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setCodec(self);
    // return 1;
  }
}

pub trait QXmlStreamWriter_setCodec<RetType> {
  fn setCodec(self , rsthis: &mut QXmlStreamWriter) -> RetType;
}

  // proto:  void QXmlStreamWriter::setCodec(QTextCodec * codec);
impl<'a> /*trait*/ QXmlStreamWriter_setCodec<()> for (QTextCodec) {
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
  pub fn writeProcessingInstruction<RetType, T: QXmlStreamWriter_writeProcessingInstruction<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.writeProcessingInstruction(self);
    // return 1;
  }
}

pub trait QXmlStreamWriter_writeProcessingInstruction<RetType> {
  fn writeProcessingInstruction(self , rsthis: &mut QXmlStreamWriter) -> RetType;
}

  // proto:  void QXmlStreamWriter::writeProcessingInstruction(const QString & target, const QString & data);
impl<'a> /*trait*/ QXmlStreamWriter_writeProcessingInstruction<()> for (QString, QString) {
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
  pub fn writeCharacters<RetType, T: QXmlStreamWriter_writeCharacters<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.writeCharacters(self);
    // return 1;
  }
}

pub trait QXmlStreamWriter_writeCharacters<RetType> {
  fn writeCharacters(self , rsthis: &mut QXmlStreamWriter) -> RetType;
}

  // proto:  void QXmlStreamWriter::writeCharacters(const QString & text);
impl<'a> /*trait*/ QXmlStreamWriter_writeCharacters<()> for (QString) {
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
  pub fn setDevice<RetType, T: QXmlStreamWriter_setDevice<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setDevice(self);
    // return 1;
  }
}

pub trait QXmlStreamWriter_setDevice<RetType> {
  fn setDevice(self , rsthis: &mut QXmlStreamWriter) -> RetType;
}

  // proto:  void QXmlStreamWriter::setDevice(QIODevice * device);
impl<'a> /*trait*/ QXmlStreamWriter_setDevice<()> for (QIODevice) {
  fn setDevice(self , rsthis: &mut QXmlStreamWriter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriter9setDeviceEP9QIODevice()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QXmlStreamWriter9setDeviceEP9QIODevice(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QXmlStreamWriter::writeStartDocument(const QString & version);
impl<'a> /*trait*/ QXmlStreamWriter_writeStartDocument<()> for (QString) {
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
  pub fn writeTextElement<RetType, T: QXmlStreamWriter_writeTextElement<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.writeTextElement(self);
    // return 1;
  }
}

pub trait QXmlStreamWriter_writeTextElement<RetType> {
  fn writeTextElement(self , rsthis: &mut QXmlStreamWriter) -> RetType;
}

  // proto:  void QXmlStreamWriter::writeTextElement(const QString & namespaceUri, const QString & name, const QString & text);
impl<'a> /*trait*/ QXmlStreamWriter_writeTextElement<()> for (QString, QString, QString) {
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
  pub fn writeAttribute<RetType, T: QXmlStreamWriter_writeAttribute<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.writeAttribute(self);
    // return 1;
  }
}

pub trait QXmlStreamWriter_writeAttribute<RetType> {
  fn writeAttribute(self , rsthis: &mut QXmlStreamWriter) -> RetType;
}

  // proto:  void QXmlStreamWriter::writeAttribute(const QString & qualifiedName, const QString & value);
impl<'a> /*trait*/ QXmlStreamWriter_writeAttribute<()> for (QString, QString) {
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
  pub fn writeEmptyElement<RetType, T: QXmlStreamWriter_writeEmptyElement<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.writeEmptyElement(self);
    // return 1;
  }
}

pub trait QXmlStreamWriter_writeEmptyElement<RetType> {
  fn writeEmptyElement(self , rsthis: &mut QXmlStreamWriter) -> RetType;
}

  // proto:  void QXmlStreamWriter::writeEmptyElement(const QString & qualifiedName);
impl<'a> /*trait*/ QXmlStreamWriter_writeEmptyElement<()> for (QString) {
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
  pub fn writeDTD<RetType, T: QXmlStreamWriter_writeDTD<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.writeDTD(self);
    // return 1;
  }
}

pub trait QXmlStreamWriter_writeDTD<RetType> {
  fn writeDTD(self , rsthis: &mut QXmlStreamWriter) -> RetType;
}

  // proto:  void QXmlStreamWriter::writeDTD(const QString & dtd);
impl<'a> /*trait*/ QXmlStreamWriter_writeDTD<()> for (QString) {
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
  pub fn setAutoFormattingIndent<RetType, T: QXmlStreamWriter_setAutoFormattingIndent<RetType>>(&mut self,  overload_args: T) -> RetType {
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
impl<'a> /*trait*/ QXmlStreamWriter_writeAttribute<()> for (QXmlStreamAttribute) {
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
  pub fn writeStartElement<RetType, T: QXmlStreamWriter_writeStartElement<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.writeStartElement(self);
    // return 1;
  }
}

pub trait QXmlStreamWriter_writeStartElement<RetType> {
  fn writeStartElement(self , rsthis: &mut QXmlStreamWriter) -> RetType;
}

  // proto:  void QXmlStreamWriter::writeStartElement(const QString & namespaceUri, const QString & name);
impl<'a> /*trait*/ QXmlStreamWriter_writeStartElement<()> for (QString, QString) {
  fn writeStartElement(self , rsthis: &mut QXmlStreamWriter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriter17writeStartElementERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN16QXmlStreamWriter17writeStartElementERK7QStringS2_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QXmlStreamWriter::QXmlStreamWriter(QString * string);
impl<'a> /*trait*/ QXmlStreamWriter_NewQXmlStreamWriter for (QString) {
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
  pub fn writeComment<RetType, T: QXmlStreamWriter_writeComment<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.writeComment(self);
    // return 1;
  }
}

pub trait QXmlStreamWriter_writeComment<RetType> {
  fn writeComment(self , rsthis: &mut QXmlStreamWriter) -> RetType;
}

  // proto:  void QXmlStreamWriter::writeComment(const QString & text);
impl<'a> /*trait*/ QXmlStreamWriter_writeComment<()> for (QString) {
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
  pub fn codec<RetType, T: QXmlStreamWriter_codec<RetType>>(&mut self,  overload_args: T) -> RetType {
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
impl<'a> /*trait*/ QXmlStreamWriter_writeAttribute<()> for (QString, QString, QString) {
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
  pub fn writeNamespace<RetType, T: QXmlStreamWriter_writeNamespace<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.writeNamespace(self);
    // return 1;
  }
}

pub trait QXmlStreamWriter_writeNamespace<RetType> {
  fn writeNamespace(self , rsthis: &mut QXmlStreamWriter) -> RetType;
}

  // proto:  void QXmlStreamWriter::writeNamespace(const QString & namespaceUri, const QString & prefix);
impl<'a> /*trait*/ QXmlStreamWriter_writeNamespace<()> for (QString, QString) {
  fn writeNamespace(self , rsthis: &mut QXmlStreamWriter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriter14writeNamespaceERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN16QXmlStreamWriter14writeNamespaceERK7QStringS2_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QXmlStreamWriter::QXmlStreamWriter(const QXmlStreamWriter & );
impl<'a> /*trait*/ QXmlStreamWriter_NewQXmlStreamWriter for (QXmlStreamWriter) {
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
  pub fn hasError<RetType, T: QXmlStreamWriter_hasError<RetType>>(&mut self,  overload_args: T) -> RetType {
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
  pub fn writeCDATA<RetType, T: QXmlStreamWriter_writeCDATA<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.writeCDATA(self);
    // return 1;
  }
}

pub trait QXmlStreamWriter_writeCDATA<RetType> {
  fn writeCDATA(self , rsthis: &mut QXmlStreamWriter) -> RetType;
}

  // proto:  void QXmlStreamWriter::writeCDATA(const QString & text);
impl<'a> /*trait*/ QXmlStreamWriter_writeCDATA<()> for (QString) {
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
  pub fn writeEntityReference<RetType, T: QXmlStreamWriter_writeEntityReference<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.writeEntityReference(self);
    // return 1;
  }
}

pub trait QXmlStreamWriter_writeEntityReference<RetType> {
  fn writeEntityReference(self , rsthis: &mut QXmlStreamWriter) -> RetType;
}

  // proto:  void QXmlStreamWriter::writeEntityReference(const QString & name);
impl<'a> /*trait*/ QXmlStreamWriter_writeEntityReference<()> for (QString) {
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
  pub fn setAutoFormatting<RetType, T: QXmlStreamWriter_setAutoFormatting<RetType>>(&mut self,  overload_args: T) -> RetType {
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
    let arg0 = self  as c_char;
     unsafe {_ZN16QXmlStreamWriter17setAutoFormattingEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QXmlStreamWriter::setCodec(const char * codecName);
impl<'a> /*trait*/ QXmlStreamWriter_setCodec<()> for (&'a  String) {
  fn setCodec(self , rsthis: &mut QXmlStreamWriter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriter8setCodecEPKc()};
    let arg0 = self.as_ptr()  as *mut c_char;
     unsafe {_ZN16QXmlStreamWriter8setCodecEPKc(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QXmlStreamWriter::autoFormattingIndent();
impl /*struct*/ QXmlStreamWriter {
  pub fn autoFormattingIndent<RetType, T: QXmlStreamWriter_autoFormattingIndent<RetType>>(&mut self,  overload_args: T) -> RetType {
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

  // proto:  void QXmlStreamWriter::~QXmlStreamWriter();
impl /*struct*/ QXmlStreamWriter {
  pub fn FreeQXmlStreamWriter<RetType, T: QXmlStreamWriter_FreeQXmlStreamWriter<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQXmlStreamWriter(self);
    // return 1;
  }
}

pub trait QXmlStreamWriter_FreeQXmlStreamWriter<RetType> {
  fn FreeQXmlStreamWriter(self , rsthis: &mut QXmlStreamWriter) -> RetType;
}

  // proto:  void QXmlStreamWriter::~QXmlStreamWriter();
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
  pub fn writeAttributes<RetType, T: QXmlStreamWriter_writeAttributes<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.writeAttributes(self);
    // return 1;
  }
}

pub trait QXmlStreamWriter_writeAttributes<RetType> {
  fn writeAttributes(self , rsthis: &mut QXmlStreamWriter) -> RetType;
}

  // proto:  void QXmlStreamWriter::writeAttributes(const QXmlStreamAttributes & attributes);
impl<'a> /*trait*/ QXmlStreamWriter_writeAttributes<()> for (QXmlStreamAttributes) {
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
  pub fn writeDefaultNamespace<RetType, T: QXmlStreamWriter_writeDefaultNamespace<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.writeDefaultNamespace(self);
    // return 1;
  }
}

pub trait QXmlStreamWriter_writeDefaultNamespace<RetType> {
  fn writeDefaultNamespace(self , rsthis: &mut QXmlStreamWriter) -> RetType;
}

  // proto:  void QXmlStreamWriter::writeDefaultNamespace(const QString & namespaceUri);
impl<'a> /*trait*/ QXmlStreamWriter_writeDefaultNamespace<()> for (QString) {
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
  pub fn device<RetType, T: QXmlStreamWriter_device<RetType>>(&mut self,  overload_args: T) -> RetType {
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
  pub fn writeCurrentToken<RetType, T: QXmlStreamWriter_writeCurrentToken<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.writeCurrentToken(self);
    // return 1;
  }
}

pub trait QXmlStreamWriter_writeCurrentToken<RetType> {
  fn writeCurrentToken(self , rsthis: &mut QXmlStreamWriter) -> RetType;
}

  // proto:  void QXmlStreamWriter::writeCurrentToken(const QXmlStreamReader & reader);
impl<'a> /*trait*/ QXmlStreamWriter_writeCurrentToken<()> for (QXmlStreamReader) {
  fn writeCurrentToken(self , rsthis: &mut QXmlStreamWriter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriter17writeCurrentTokenERK16QXmlStreamReader()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QXmlStreamWriter17writeCurrentTokenERK16QXmlStreamReader(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QXmlStreamWriter::QXmlStreamWriter(QByteArray * array);
impl<'a> /*trait*/ QXmlStreamWriter_NewQXmlStreamWriter for (QByteArray) {
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
impl<'a> /*trait*/ QXmlStreamWriter_writeTextElement<()> for (QString, QString) {
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
impl<'a> /*trait*/ QXmlStreamWriter_writeEmptyElement<()> for (QString, QString) {
  fn writeEmptyElement(self , rsthis: &mut QXmlStreamWriter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriter17writeEmptyElementERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN16QXmlStreamWriter17writeEmptyElementERK7QStringS2_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QXmlStreamWriter::QXmlStreamWriter(QIODevice * device);
impl<'a> /*trait*/ QXmlStreamWriter_NewQXmlStreamWriter for (QIODevice) {
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
impl<'a> /*trait*/ QXmlStreamWriter_writeStartElement<()> for (QString) {
  fn writeStartElement(self , rsthis: &mut QXmlStreamWriter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QXmlStreamWriter17writeStartElementERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QXmlStreamWriter17writeStartElementERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QXmlStreamNotationDeclaration::QXmlStreamNotationDeclaration(const QXmlStreamNotationDeclaration & );
impl /*struct*/ QXmlStreamNotationDeclaration {
  pub fn NewQXmlStreamNotationDeclaration<T: QXmlStreamNotationDeclaration_NewQXmlStreamNotationDeclaration>(value: T) -> QXmlStreamNotationDeclaration {
    let rsthis = value.NewQXmlStreamNotationDeclaration();
    return rsthis;
    // return 1;
  }
}

pub trait QXmlStreamNotationDeclaration_NewQXmlStreamNotationDeclaration {
  fn NewQXmlStreamNotationDeclaration(self) -> QXmlStreamNotationDeclaration;
}

  // proto:  void QXmlStreamNotationDeclaration::QXmlStreamNotationDeclaration(const QXmlStreamNotationDeclaration & );
impl<'a> /*trait*/ QXmlStreamNotationDeclaration_NewQXmlStreamNotationDeclaration for (QXmlStreamNotationDeclaration) {
  fn NewQXmlStreamNotationDeclaration(self) -> QXmlStreamNotationDeclaration {
    let qthis: *mut c_void = unsafe{calloc(1, 56)};
    // unsafe{_ZN29QXmlStreamNotationDeclarationC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN29QXmlStreamNotationDeclarationC1ERKS_(qthis, arg0)};
    let rsthis = QXmlStreamNotationDeclaration{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QStringRef QXmlStreamNotationDeclaration::publicId();
impl /*struct*/ QXmlStreamNotationDeclaration {
  pub fn publicId<RetType, T: QXmlStreamNotationDeclaration_publicId<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.publicId(self);
    // return 1;
  }
}

pub trait QXmlStreamNotationDeclaration_publicId<RetType> {
  fn publicId(self , rsthis: &mut QXmlStreamNotationDeclaration) -> RetType;
}

  // proto:  QStringRef QXmlStreamNotationDeclaration::publicId();
impl<'a> /*trait*/ QXmlStreamNotationDeclaration_publicId<()> for () {
  fn publicId(self , rsthis: &mut QXmlStreamNotationDeclaration) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 56)};
    // unsafe{_ZNK29QXmlStreamNotationDeclaration8publicIdEv()};
     unsafe {_ZNK29QXmlStreamNotationDeclaration8publicIdEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QStringRef QXmlStreamNotationDeclaration::name();
impl /*struct*/ QXmlStreamNotationDeclaration {
  pub fn name<RetType, T: QXmlStreamNotationDeclaration_name<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.name(self);
    // return 1;
  }
}

pub trait QXmlStreamNotationDeclaration_name<RetType> {
  fn name(self , rsthis: &mut QXmlStreamNotationDeclaration) -> RetType;
}

  // proto:  QStringRef QXmlStreamNotationDeclaration::name();
impl<'a> /*trait*/ QXmlStreamNotationDeclaration_name<()> for () {
  fn name(self , rsthis: &mut QXmlStreamNotationDeclaration) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 56)};
    // unsafe{_ZNK29QXmlStreamNotationDeclaration4nameEv()};
     unsafe {_ZNK29QXmlStreamNotationDeclaration4nameEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QXmlStreamNotationDeclaration::~QXmlStreamNotationDeclaration();
impl /*struct*/ QXmlStreamNotationDeclaration {
  pub fn FreeQXmlStreamNotationDeclaration<RetType, T: QXmlStreamNotationDeclaration_FreeQXmlStreamNotationDeclaration<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQXmlStreamNotationDeclaration(self);
    // return 1;
  }
}

pub trait QXmlStreamNotationDeclaration_FreeQXmlStreamNotationDeclaration<RetType> {
  fn FreeQXmlStreamNotationDeclaration(self , rsthis: &mut QXmlStreamNotationDeclaration) -> RetType;
}

  // proto:  void QXmlStreamNotationDeclaration::~QXmlStreamNotationDeclaration();
impl<'a> /*trait*/ QXmlStreamNotationDeclaration_FreeQXmlStreamNotationDeclaration<()> for () {
  fn FreeQXmlStreamNotationDeclaration(self , rsthis: &mut QXmlStreamNotationDeclaration) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 56)};
    // unsafe{_ZN29QXmlStreamNotationDeclarationD0Ev()};
     unsafe {_ZN29QXmlStreamNotationDeclarationD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QStringRef QXmlStreamNotationDeclaration::systemId();
impl /*struct*/ QXmlStreamNotationDeclaration {
  pub fn systemId<RetType, T: QXmlStreamNotationDeclaration_systemId<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.systemId(self);
    // return 1;
  }
}

pub trait QXmlStreamNotationDeclaration_systemId<RetType> {
  fn systemId(self , rsthis: &mut QXmlStreamNotationDeclaration) -> RetType;
}

  // proto:  QStringRef QXmlStreamNotationDeclaration::systemId();
impl<'a> /*trait*/ QXmlStreamNotationDeclaration_systemId<()> for () {
  fn systemId(self , rsthis: &mut QXmlStreamNotationDeclaration) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 56)};
    // unsafe{_ZNK29QXmlStreamNotationDeclaration8systemIdEv()};
     unsafe {_ZNK29QXmlStreamNotationDeclaration8systemIdEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QXmlStreamNotationDeclaration::QXmlStreamNotationDeclaration();
impl<'a> /*trait*/ QXmlStreamNotationDeclaration_NewQXmlStreamNotationDeclaration for () {
  fn NewQXmlStreamNotationDeclaration(self) -> QXmlStreamNotationDeclaration {
    let qthis: *mut c_void = unsafe{calloc(1, 56)};
    // unsafe{_ZN29QXmlStreamNotationDeclarationC1Ev()};
    unsafe {_ZN29QXmlStreamNotationDeclarationC1Ev(qthis)};
    let rsthis = QXmlStreamNotationDeclaration{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QXmlStreamAttribute::QXmlStreamAttribute(const QString & qualifiedName, const QString & value);
impl /*struct*/ QXmlStreamAttribute {
  pub fn NewQXmlStreamAttribute<T: QXmlStreamAttribute_NewQXmlStreamAttribute>(value: T) -> QXmlStreamAttribute {
    let rsthis = value.NewQXmlStreamAttribute();
    return rsthis;
    // return 1;
  }
}

pub trait QXmlStreamAttribute_NewQXmlStreamAttribute {
  fn NewQXmlStreamAttribute(self) -> QXmlStreamAttribute;
}

  // proto:  void QXmlStreamAttribute::QXmlStreamAttribute(const QString & qualifiedName, const QString & value);
impl<'a> /*trait*/ QXmlStreamAttribute_NewQXmlStreamAttribute for (QString, QString) {
  fn NewQXmlStreamAttribute(self) -> QXmlStreamAttribute {
    let qthis: *mut c_void = unsafe{calloc(1, 80)};
    // unsafe{_ZN19QXmlStreamAttributeC1ERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN19QXmlStreamAttributeC1ERK7QStringS2_(qthis, arg0, arg1)};
    let rsthis = QXmlStreamAttribute{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QStringRef QXmlStreamAttribute::qualifiedName();
impl /*struct*/ QXmlStreamAttribute {
  pub fn qualifiedName<RetType, T: QXmlStreamAttribute_qualifiedName<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.qualifiedName(self);
    // return 1;
  }
}

pub trait QXmlStreamAttribute_qualifiedName<RetType> {
  fn qualifiedName(self , rsthis: &mut QXmlStreamAttribute) -> RetType;
}

  // proto:  QStringRef QXmlStreamAttribute::qualifiedName();
impl<'a> /*trait*/ QXmlStreamAttribute_qualifiedName<()> for () {
  fn qualifiedName(self , rsthis: &mut QXmlStreamAttribute) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 80)};
    // unsafe{_ZNK19QXmlStreamAttribute13qualifiedNameEv()};
     unsafe {_ZNK19QXmlStreamAttribute13qualifiedNameEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QXmlStreamAttribute::~QXmlStreamAttribute();
impl /*struct*/ QXmlStreamAttribute {
  pub fn FreeQXmlStreamAttribute<RetType, T: QXmlStreamAttribute_FreeQXmlStreamAttribute<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQXmlStreamAttribute(self);
    // return 1;
  }
}

pub trait QXmlStreamAttribute_FreeQXmlStreamAttribute<RetType> {
  fn FreeQXmlStreamAttribute(self , rsthis: &mut QXmlStreamAttribute) -> RetType;
}

  // proto:  void QXmlStreamAttribute::~QXmlStreamAttribute();
impl<'a> /*trait*/ QXmlStreamAttribute_FreeQXmlStreamAttribute<()> for () {
  fn FreeQXmlStreamAttribute(self , rsthis: &mut QXmlStreamAttribute) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 80)};
    // unsafe{_ZN19QXmlStreamAttributeD0Ev()};
     unsafe {_ZN19QXmlStreamAttributeD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QStringRef QXmlStreamAttribute::value();
impl /*struct*/ QXmlStreamAttribute {
  pub fn value<RetType, T: QXmlStreamAttribute_value<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.value(self);
    // return 1;
  }
}

pub trait QXmlStreamAttribute_value<RetType> {
  fn value(self , rsthis: &mut QXmlStreamAttribute) -> RetType;
}

  // proto:  QStringRef QXmlStreamAttribute::value();
impl<'a> /*trait*/ QXmlStreamAttribute_value<()> for () {
  fn value(self , rsthis: &mut QXmlStreamAttribute) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 80)};
    // unsafe{_ZNK19QXmlStreamAttribute5valueEv()};
     unsafe {_ZNK19QXmlStreamAttribute5valueEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QStringRef QXmlStreamAttribute::namespaceUri();
impl /*struct*/ QXmlStreamAttribute {
  pub fn namespaceUri<RetType, T: QXmlStreamAttribute_namespaceUri<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.namespaceUri(self);
    // return 1;
  }
}

pub trait QXmlStreamAttribute_namespaceUri<RetType> {
  fn namespaceUri(self , rsthis: &mut QXmlStreamAttribute) -> RetType;
}

  // proto:  QStringRef QXmlStreamAttribute::namespaceUri();
impl<'a> /*trait*/ QXmlStreamAttribute_namespaceUri<()> for () {
  fn namespaceUri(self , rsthis: &mut QXmlStreamAttribute) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 80)};
    // unsafe{_ZNK19QXmlStreamAttribute12namespaceUriEv()};
     unsafe {_ZNK19QXmlStreamAttribute12namespaceUriEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QXmlStreamAttribute::QXmlStreamAttribute();
impl<'a> /*trait*/ QXmlStreamAttribute_NewQXmlStreamAttribute for () {
  fn NewQXmlStreamAttribute(self) -> QXmlStreamAttribute {
    let qthis: *mut c_void = unsafe{calloc(1, 80)};
    // unsafe{_ZN19QXmlStreamAttributeC1Ev()};
    unsafe {_ZN19QXmlStreamAttributeC1Ev(qthis)};
    let rsthis = QXmlStreamAttribute{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QXmlStreamAttribute::QXmlStreamAttribute(const QXmlStreamAttribute & );
impl<'a> /*trait*/ QXmlStreamAttribute_NewQXmlStreamAttribute for (QXmlStreamAttribute) {
  fn NewQXmlStreamAttribute(self) -> QXmlStreamAttribute {
    let qthis: *mut c_void = unsafe{calloc(1, 80)};
    // unsafe{_ZN19QXmlStreamAttributeC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN19QXmlStreamAttributeC1ERKS_(qthis, arg0)};
    let rsthis = QXmlStreamAttribute{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QXmlStreamAttribute::QXmlStreamAttribute(const QString & namespaceUri, const QString & name, const QString & value);
impl<'a> /*trait*/ QXmlStreamAttribute_NewQXmlStreamAttribute for (QString, QString, QString) {
  fn NewQXmlStreamAttribute(self) -> QXmlStreamAttribute {
    let qthis: *mut c_void = unsafe{calloc(1, 80)};
    // unsafe{_ZN19QXmlStreamAttributeC1ERK7QStringS2_S2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    unsafe {_ZN19QXmlStreamAttributeC1ERK7QStringS2_S2_(qthis, arg0, arg1, arg2)};
    let rsthis = QXmlStreamAttribute{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QXmlStreamAttribute::isDefault();
impl /*struct*/ QXmlStreamAttribute {
  pub fn isDefault<RetType, T: QXmlStreamAttribute_isDefault<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isDefault(self);
    // return 1;
  }
}

pub trait QXmlStreamAttribute_isDefault<RetType> {
  fn isDefault(self , rsthis: &mut QXmlStreamAttribute) -> RetType;
}

  // proto:  bool QXmlStreamAttribute::isDefault();
impl<'a> /*trait*/ QXmlStreamAttribute_isDefault<i8> for () {
  fn isDefault(self , rsthis: &mut QXmlStreamAttribute) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 80)};
    // unsafe{_ZNK19QXmlStreamAttribute9isDefaultEv()};
    let mut ret = unsafe {_ZNK19QXmlStreamAttribute9isDefaultEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QStringRef QXmlStreamAttribute::prefix();
impl /*struct*/ QXmlStreamAttribute {
  pub fn prefix<RetType, T: QXmlStreamAttribute_prefix<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.prefix(self);
    // return 1;
  }
}

pub trait QXmlStreamAttribute_prefix<RetType> {
  fn prefix(self , rsthis: &mut QXmlStreamAttribute) -> RetType;
}

  // proto:  QStringRef QXmlStreamAttribute::prefix();
impl<'a> /*trait*/ QXmlStreamAttribute_prefix<()> for () {
  fn prefix(self , rsthis: &mut QXmlStreamAttribute) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 80)};
    // unsafe{_ZNK19QXmlStreamAttribute6prefixEv()};
     unsafe {_ZNK19QXmlStreamAttribute6prefixEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QStringRef QXmlStreamAttribute::name();
impl /*struct*/ QXmlStreamAttribute {
  pub fn name<RetType, T: QXmlStreamAttribute_name<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.name(self);
    // return 1;
  }
}

pub trait QXmlStreamAttribute_name<RetType> {
  fn name(self , rsthis: &mut QXmlStreamAttribute) -> RetType;
}

  // proto:  QStringRef QXmlStreamAttribute::name();
impl<'a> /*trait*/ QXmlStreamAttribute_name<()> for () {
  fn name(self , rsthis: &mut QXmlStreamAttribute) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 80)};
    // unsafe{_ZNK19QXmlStreamAttribute4nameEv()};
     unsafe {_ZNK19QXmlStreamAttribute4nameEv(rsthis.qclsinst)};
    // return 1;
  }
}

// <= body block end

