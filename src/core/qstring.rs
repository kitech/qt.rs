// auto generated, do not modify.
// created: Thu Jan 28 22:38:45 2016
// src-file: /QtCore/qstring.h
// dst-file: /src/core/qstring.rs
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
use super::qchar::*; // 773
use super::qregularexpression::*; // 773
// use super::qstring::QStringRef; // 773
use super::qregexp::*; // 773
use super::qbytearray::*; // 773
// use super::qvector::*; // 775
// use super::qstring::QString; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QStringDataPtr_Class_Size() -> c_int;
  fn QString_Class_Size() -> c_int;
  // proto:  qlonglong QString::toLongLong(bool * ok, int base);
  fn C_ZNK7QString10toLongLongEPbi(qthis: u64 /* *mut c_void*/, arg0: *mut c_char, arg1: c_int) -> c_longlong;
  // proto:  bool QString::isNull();
  fn C_ZNK7QString6isNullEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QString & QString::append(const QChar * uc, int len);
  fn C_ZN7QString6appendEPK5QChari(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_int) -> *mut c_void;
  // proto:  QString & QString::prepend(QChar c);
  fn C_ZN7QString7prependE5QChar(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  QString & QString::insert(int i, QChar c);
  fn C_ZN7QString6insertEi5QChar(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void) -> *mut c_void;
  // proto:  QString QString::left(int n);
  fn C_ZNK7QString4leftEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> *mut c_void;
  // proto:  void QString::QString(QChar c);
  fn C_ZN7QStringC2E5QChar(arg0: *mut c_void) -> u64;
  // proto:  QString & QString::prepend(const char * s);
  fn C_ZN7QString7prependEPKc(qthis: u64 /* *mut c_void*/, arg0: *mut c_char) -> *mut c_void;
  // proto:  int QString::lastIndexOf(const QRegularExpression & re, int from);
  fn C_ZNK7QString11lastIndexOfERK18QRegularExpressioni(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_int) -> c_int;
  // proto: static QString QString::number(int , int base);
  fn C_ZN7QString6numberEii(arg0: c_int, arg1: c_int) -> *mut c_void;
  // proto:  void QString::resize(int size);
  fn C_ZN7QString6resizeEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QString::push_front(QChar c);
  fn C_ZN7QString10push_frontE5QChar(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QString::QString();
  fn C_ZN7QStringC2Ev() -> u64;
  // proto:  double QString::toDouble(bool * ok);
  fn C_ZNK7QString8toDoubleEPb(qthis: u64 /* *mut c_void*/, arg0: *mut c_char) -> c_double;
  // proto:  QString QString::arg(const QString & a1, const QString & a2, const QString & a3, const QString & a4, const QString & a5, const QString & a6, const QString & a7, const QString & a8);
  fn C_ZNK7QString3argERKS_S1_S1_S1_S1_S1_S1_S1_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void, arg3: *mut c_void, arg4: *mut c_void, arg5: *mut c_void, arg6: *mut c_void, arg7: *mut c_void) -> *mut c_void;
  // proto:  QStringRef QString::rightRef(int n);
  fn C_ZNK7QString8rightRefEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> *mut c_void;
  // proto:  QString & QString::setNum(short , int base);
  fn C_ZN7QString6setNumEsi(qthis: u64 /* *mut c_void*/, arg0: c_short, arg1: c_int) -> *mut c_void;
  // proto:  void QString::QString(const QChar * unicode, int size);
  fn C_ZN7QStringC2EPK5QChari(arg0: *mut c_void, arg1: c_int) -> u64;
  // proto:  float QString::toFloat(bool * ok);
  fn C_ZNK7QString7toFloatEPb(qthis: u64 /* *mut c_void*/, arg0: *mut c_char) -> c_float;
  // proto:  int QString::count(const QRegularExpression & re);
  fn C_ZNK7QString5countERK18QRegularExpression(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_int;
  // proto:  QStringRef QString::midRef(int position, int n);
  fn C_ZNK7QString6midRefEii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int) -> *mut c_void;
  // proto:  void QString::detach();
  fn C_ZN7QString6detachEv(qthis: u64 /* *mut c_void*/);
  // proto:  QString QString::arg(const QString & a1, const QString & a2, const QString & a3, const QString & a4);
  fn C_ZNK7QString3argERKS_S1_S1_S1_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void, arg3: *mut c_void) -> *mut c_void;
  // proto:  QString QString::arg(long a, int fieldwidth, int base, QChar fillChar);
  fn C_ZNK7QString3argElii5QChar(qthis: u64 /* *mut c_void*/, arg0: c_long, arg1: c_int, arg2: c_int, arg3: *mut c_void) -> *mut c_void;
  // proto:  int QString::count();
  fn C_ZNK7QString5countEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  QString & QString::setNum(qulonglong , int base);
  fn C_ZN7QString6setNumEyi(qthis: u64 /* *mut c_void*/, arg0: c_ulonglong, arg1: c_int) -> *mut c_void;
  // proto:  void QString::push_back(QChar c);
  fn C_ZN7QString9push_backE5QChar(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QString & QString::setNum(float , char f, int prec);
  fn C_ZN7QString6setNumEfci(qthis: u64 /* *mut c_void*/, arg0: c_float, arg1: c_char, arg2: c_int) -> *mut c_void;
  // proto:  int QString::count(const QRegExp & );
  fn C_ZNK7QString5countERK7QRegExp(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_int;
  // proto:  int QString::size();
  fn C_ZNK7QString4sizeEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  QString & QString::insert(int i, const QChar * uc, int len);
  fn C_ZN7QString6insertEiPK5QChari(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void, arg2: c_int) -> *mut c_void;
  // proto:  QString & QString::replace(int i, int len, const QChar * s, int slen);
  fn C_ZN7QString7replaceEiiPK5QChari(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int, arg2: *mut c_void, arg3: c_int) -> *mut c_void;
  // proto: static QString QString::fromRawData(const QChar * , int size);
  fn C_ZN7QString11fromRawDataEPK5QChari(arg0: *mut c_void, arg1: c_int) -> *mut c_void;
  // proto:  QString QString::trimmed();
  fn C_ZNO7QString7trimmedEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QString & QString::insert(int i, const QString & s);
  fn C_ZN7QString6insertEiRKS_(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void) -> *mut c_void;
  // proto:  QString QString::arg(const QString & a1, const QString & a2, const QString & a3, const QString & a4, const QString & a5);
  fn C_ZNK7QString3argERKS_S1_S1_S1_S1_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void, arg3: *mut c_void, arg4: *mut c_void) -> *mut c_void;
  // proto:  QString & QString::setRawData(const QChar * unicode, int size);
  fn C_ZN7QString10setRawDataEPK5QChari(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_int) -> *mut c_void;
  // proto:  QString & QString::prepend(const QString & s);
  fn C_ZN7QString7prependERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  QString & QString::sprintf(const char * format);
  fn C_ZN7QString7sprintfEPKcz(qthis: u64 /* *mut c_void*/, arg0: *mut c_char) -> *mut c_void;
  // proto:  ulong QString::toULong(bool * ok, int base);
  fn C_ZNK7QString7toULongEPbi(qthis: u64 /* *mut c_void*/, arg0: *mut c_char, arg1: c_int) -> c_ulong;
  // proto:  void QString::chop(int n);
  fn C_ZN7QString4chopEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto: static QString QString::fromUtf16(const ushort * , int size);
  fn C_ZN7QString9fromUtf16EPKti(arg0: *mut c_ushort, arg1: c_int) -> *mut c_void;
  // proto:  bool QString::isDetached();
  fn C_ZNK7QString10isDetachedEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QString & QString::setNum(qlonglong , int base);
  fn C_ZN7QString6setNumExi(qthis: u64 /* *mut c_void*/, arg0: c_longlong, arg1: c_int) -> *mut c_void;
  // proto:  QString QString::mid(int position, int n);
  fn C_ZNK7QString3midEii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int) -> *mut c_void;
  // proto: static QString QString::fromLocal8Bit(const char * str, int size);
  fn C_ZN7QString13fromLocal8BitEPKci(arg0: *mut c_char, arg1: c_int) -> *mut c_void;
  // proto:  void QString::swap(QString & other);
  fn C_ZN7QString4swapERS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QString & QString::vsprintf(const char * format, va_list ap);
  fn C_ZN7QString8vsprintfEPKci(qthis: u64 /* *mut c_void*/, arg0: *mut c_char, arg1: c_int) -> *mut c_void;
  // proto: static QString QString::fromUtf8(const QByteArray & str);
  fn C_ZN7QString8fromUtf8ERK10QByteArray(arg0: *mut c_void) -> *mut c_void;
  // proto: static QString QString::fromUcs4(const char32_t * str, int size);
  fn C_ZN7QString8fromUcs4EPKDii(arg0: *mut c_char, arg1: c_int) -> *mut c_void;
  // proto:  QString QString::leftJustified(int width, QChar fill, bool trunc);
  fn C_ZNK7QString13leftJustifiedEi5QCharb(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void, arg2: c_char) -> *mut c_void;
  // proto:  int QString::indexOf(const QRegExp & , int from);
  fn C_ZNK7QString7indexOfERK7QRegExpi(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_int) -> c_int;
  // proto:  void QString::push_back(const QString & s);
  fn C_ZN7QString9push_backERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  int QString::lastIndexOf(QRegExp & , int from);
  fn C_ZNK7QString11lastIndexOfER7QRegExpi(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_int) -> c_int;
  // proto:  QString QString::arg(const QString & a1, const QString & a2, const QString & a3);
  fn C_ZNK7QString3argERKS_S1_S1_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) -> *mut c_void;
  // proto:  const ushort * QString::utf16();
  fn C_ZNK7QString5utf16Ev(qthis: u64 /* *mut c_void*/) -> *mut c_ushort;
  // proto:  int QString::toInt(bool * ok, int base);
  fn C_ZNK7QString5toIntEPbi(qthis: u64 /* *mut c_void*/, arg0: *mut c_char, arg1: c_int) -> c_int;
  // proto:  QByteArray QString::toUtf8();
  fn C_ZNO7QString6toUtf8Ev(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QString QString::arg(double a, int fieldWidth, char fmt, int prec, QChar fillChar);
  fn C_ZNK7QString3argEdici5QChar(qthis: u64 /* *mut c_void*/, arg0: c_double, arg1: c_int, arg2: c_char, arg3: c_int, arg4: *mut c_void) -> *mut c_void;
  // proto:  QChar * QString::data();
  fn C_ZN7QString4dataEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QString QString::toCaseFolded();
  fn C_ZNO7QString12toCaseFoldedEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QString & QString::setNum(uint , int base);
  fn C_ZN7QString6setNumEji(qthis: u64 /* *mut c_void*/, arg0: c_uint, arg1: c_int) -> *mut c_void;
  // proto: static int QString::localeAwareCompare(const QString & s1, const QString & s2);
  fn C_ZN7QString18localeAwareCompareERKS_S1_(arg0: *mut c_void, arg1: *mut c_void) -> c_int;
  // proto:  void QString::QString(const char * ch);
  fn C_ZN7QStringC2EPKc(arg0: *mut c_char) -> u64;
  // proto: static QString QString::fromUtf16(const char16_t * str, int size);
  fn C_ZN7QString9fromUtf16EPKDsi(arg0: *mut c_char, arg1: c_int) -> *mut c_void;
  // proto:  QString & QString::replace(const QRegExp & rx, const QString & after);
  fn C_ZN7QString7replaceERK7QRegExpRKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  QString QString::repeated(int times);
  fn C_ZNK7QString8repeatedEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> *mut c_void;
  // proto:  QString QString::toLower();
  fn C_ZNO7QString7toLowerEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QString & QString::setUtf16(const ushort * utf16, int size);
  fn C_ZN7QString8setUtf16EPKti(qthis: u64 /* *mut c_void*/, arg0: *mut c_ushort, arg1: c_int) -> *mut c_void;
  // proto:  void QString::clear();
  fn C_ZN7QString5clearEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QString::contains(const QRegExp & rx);
  fn C_ZNK7QString8containsERK7QRegExp(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  bool QString::isSharedWith(const QString & other);
  fn C_ZNK7QString12isSharedWithERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto: static QString QString::fromLatin1(const QByteArray & str);
  fn C_ZN7QString10fromLatin1ERK10QByteArray(arg0: *mut c_void) -> *mut c_void;
  // proto:  void QString::~QString();
  fn C_ZN7QStringD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QString & QString::remove(const QRegularExpression & re);
  fn C_ZN7QString6removeERK18QRegularExpression(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  QString & QString::setNum(int , int base);
  fn C_ZN7QString6setNumEii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int) -> *mut c_void;
  // proto:  const_iterator QString::cend();
  fn C_ZNK7QString4cendEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QString QString::toHtmlEscaped();
  fn C_ZNK7QString13toHtmlEscapedEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  int QString::lastIndexOf(const QRegularExpression & re, int from, QRegularExpressionMatch * rmatch);
  fn C_ZNK7QString11lastIndexOfERK18QRegularExpressioniP23QRegularExpressionMatch(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_int, arg2: *mut c_void) -> c_int;
  // proto:  QString & QString::append(const QByteArray & s);
  fn C_ZN7QString6appendERK10QByteArray(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto: static QString QString::fromLatin1(const char * str, int size);
  fn C_ZN7QString10fromLatin1EPKci(arg0: *mut c_char, arg1: c_int) -> *mut c_void;
  // proto:  bool QString::contains(const QRegularExpression & re, QRegularExpressionMatch * match);
  fn C_ZNK7QString8containsERK18QRegularExpressionP23QRegularExpressionMatch(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void) -> c_char;
  // proto:  int QString::indexOf(const QRegularExpression & re, int from, QRegularExpressionMatch * rmatch);
  fn C_ZNK7QString7indexOfERK18QRegularExpressioniP23QRegularExpressionMatch(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_int, arg2: *mut c_void) -> c_int;
  // proto:  int QString::lastIndexOf(const QRegExp & , int from);
  fn C_ZNK7QString11lastIndexOfERK7QRegExpi(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_int) -> c_int;
  // proto:  int QString::toWCharArray(wchar_t * array);
  fn C_ZNK7QString12toWCharArrayEPw(qthis: u64 /* *mut c_void*/, arg0: *mut wchar_t) -> c_int;
  // proto:  const_iterator QString::cbegin();
  fn C_ZNK7QString6cbeginEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QString & QString::prepend(const QByteArray & s);
  fn C_ZN7QString7prependERK10QByteArray(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  QString & QString::replace(int i, int len, const QString & after);
  fn C_ZN7QString7replaceEiiRKS_(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int, arg2: *mut c_void) -> *mut c_void;
  // proto:  QString QString::arg(const QString & a1, const QString & a2, const QString & a3, const QString & a4, const QString & a5, const QString & a6, const QString & a7);
  fn C_ZNK7QString3argERKS_S1_S1_S1_S1_S1_S1_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void, arg3: *mut c_void, arg4: *mut c_void, arg5: *mut c_void, arg6: *mut c_void) -> *mut c_void;
  // proto: static QString QString::fromWCharArray(const wchar_t * string, int size);
  fn C_ZN7QString14fromWCharArrayEPKwi(arg0: *mut wchar_t, arg1: c_int) -> *mut c_void;
  // proto:  QString & QString::fill(QChar c, int size);
  fn C_ZN7QString4fillE5QChari(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_int) -> *mut c_void;
  // proto:  const QChar * QString::constData();
  fn C_ZNK7QString9constDataEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto: static QString QString::number(ulong , int base);
  fn C_ZN7QString6numberEmi(arg0: c_ulong, arg1: c_int) -> *mut c_void;
  // proto:  long QString::toLong(bool * ok, int base);
  fn C_ZNK7QString6toLongEPbi(qthis: u64 /* *mut c_void*/, arg0: *mut c_char, arg1: c_int) -> c_long;
  // proto:  QString QString::toUpper();
  fn C_ZNO7QString7toUpperEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  const_iterator QString::constEnd();
  fn C_ZNK7QString8constEndEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  int QString::length();
  fn C_ZNK7QString6lengthEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto: static QString QString::fromUtf8(const char * str, int size);
  fn C_ZN7QString8fromUtf8EPKci(arg0: *mut c_char, arg1: c_int) -> *mut c_void;
  // proto:  QString QString::simplified();
  fn C_ZNO7QString10simplifiedEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto: static QString QString::number(qlonglong , int base);
  fn C_ZN7QString6numberExi(arg0: c_longlong, arg1: c_int) -> *mut c_void;
  // proto:  QStringRef QString::leftRef(int n);
  fn C_ZNK7QString7leftRefEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> *mut c_void;
  // proto:  QString & QString::setNum(long , int base);
  fn C_ZN7QString6setNumEli(qthis: u64 /* *mut c_void*/, arg0: c_long, arg1: c_int) -> *mut c_void;
  // proto:  QString QString::arg(const QString & a1, const QString & a2);
  fn C_ZNK7QString3argERKS_S1_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  bool QString::isSimpleText();
  fn C_ZNK7QString12isSimpleTextEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto: static QString QString::fromUcs4(const uint * , int size);
  fn C_ZN7QString8fromUcs4EPKji(arg0: *mut c_uint, arg1: c_int) -> *mut c_void;
  // proto:  QString & QString::setUnicode(const QChar * unicode, int size);
  fn C_ZN7QString10setUnicodeEPK5QChari(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_int) -> *mut c_void;
  // proto:  bool QString::contains(QRegExp & rx);
  fn C_ZNK7QString8containsER7QRegExp(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  const_iterator QString::constBegin();
  fn C_ZNK7QString10constBeginEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  const QChar * QString::unicode();
  fn C_ZNK7QString7unicodeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QString QString::arg(const QString & a1, const QString & a2, const QString & a3, const QString & a4, const QString & a5, const QString & a6, const QString & a7, const QString & a8, const QString & a9);
  fn C_ZNK7QString3argERKS_S1_S1_S1_S1_S1_S1_S1_S1_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void, arg3: *mut c_void, arg4: *mut c_void, arg5: *mut c_void, arg6: *mut c_void, arg7: *mut c_void, arg8: *mut c_void) -> *mut c_void;
  // proto:  int QString::indexOf(const QRegularExpression & re, int from);
  fn C_ZNK7QString7indexOfERK18QRegularExpressioni(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_int) -> c_int;
  // proto: static QString QString::number(long , int base);
  fn C_ZN7QString6numberEli(arg0: c_long, arg1: c_int) -> *mut c_void;
  // proto: static QString QString::number(uint , int base);
  fn C_ZN7QString6numberEji(arg0: c_uint, arg1: c_int) -> *mut c_void;
  // proto: static QString QString::fromLocal8Bit(const QByteArray & str);
  fn C_ZN7QString13fromLocal8BitERK10QByteArray(arg0: *mut c_void) -> *mut c_void;
  // proto:  const QChar QString::at(int i);
  fn C_ZNK7QString2atEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> *mut c_void;
  // proto: static QString QString::asprintf(const char * format);
  fn C_ZN7QString8asprintfEPKcz(arg0: *mut c_char) -> *mut c_void;
  // proto:  void QString::QString(int size, QChar c);
  fn C_ZN7QStringC2Ei5QChar(arg0: c_int, arg1: *mut c_void) -> u64;
  // proto:  QByteArray QString::toLatin1();
  fn C_ZNO7QString8toLatin1Ev(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QString & QString::setNum(ulong , int base);
  fn C_ZN7QString6setNumEmi(qthis: u64 /* *mut c_void*/, arg0: c_ulong, arg1: c_int) -> *mut c_void;
  // proto:  void QString::push_front(const QString & s);
  fn C_ZN7QString10push_frontERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QString QString::arg(const QString & a1, const QString & a2, const QString & a3, const QString & a4, const QString & a5, const QString & a6);
  fn C_ZNK7QString3argERKS_S1_S1_S1_S1_S1_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void, arg3: *mut c_void, arg4: *mut c_void, arg5: *mut c_void) -> *mut c_void;
  // proto:  iterator QString::begin();
  fn C_ZN7QString5beginEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto: static QString QString::number(double , char f, int prec);
  fn C_ZN7QString6numberEdci(arg0: c_double, arg1: c_char, arg2: c_int) -> *mut c_void;
  // proto:  iterator QString::end();
  fn C_ZN7QString3endEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QString & QString::append(QChar c);
  fn C_ZN7QString6appendE5QChar(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  uint QString::toUInt(bool * ok, int base);
  fn C_ZNK7QString6toUIntEPbi(qthis: u64 /* *mut c_void*/, arg0: *mut c_char, arg1: c_int) -> c_uint;
  // proto:  QString & QString::append(const QString & s);
  fn C_ZN7QString6appendERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  QString QString::arg(qlonglong a, int fieldwidth, int base, QChar fillChar);
  fn C_ZNK7QString3argExii5QChar(qthis: u64 /* *mut c_void*/, arg0: c_longlong, arg1: c_int, arg2: c_int, arg3: *mut c_void) -> *mut c_void;
  // proto:  ushort QString::toUShort(bool * ok, int base);
  fn C_ZNK7QString8toUShortEPbi(qthis: u64 /* *mut c_void*/, arg0: *mut c_char, arg1: c_int) -> c_ushort;
  // proto:  QString QString::arg(uint a, int fieldWidth, int base, QChar fillChar);
  fn C_ZNK7QString3argEjii5QChar(qthis: u64 /* *mut c_void*/, arg0: c_uint, arg1: c_int, arg2: c_int, arg3: *mut c_void) -> *mut c_void;
  // proto:  QString & QString::setNum(ushort , int base);
  fn C_ZN7QString6setNumEti(qthis: u64 /* *mut c_void*/, arg0: c_ushort, arg1: c_int) -> *mut c_void;
  // proto:  QByteArray QString::toLocal8Bit();
  fn C_ZNO7QString11toLocal8BitEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QString & QString::replace(const QRegularExpression & re, const QString & after);
  fn C_ZN7QString7replaceERK18QRegularExpressionRKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  QString & QString::setNum(double , char f, int prec);
  fn C_ZN7QString6setNumEdci(qthis: u64 /* *mut c_void*/, arg0: c_double, arg1: c_char, arg2: c_int) -> *mut c_void;
  // proto: static QString QString::number(qulonglong , int base);
  fn C_ZN7QString6numberEyi(arg0: c_ulonglong, arg1: c_int) -> *mut c_void;
  // proto:  QString QString::arg(ushort a, int fieldWidth, int base, QChar fillChar);
  fn C_ZNK7QString3argEtii5QChar(qthis: u64 /* *mut c_void*/, arg0: c_ushort, arg1: c_int, arg2: c_int, arg3: *mut c_void) -> *mut c_void;
  // proto:  void QString::QString(const QString & );
  fn C_ZN7QStringC2ERKS_(arg0: *mut c_void) -> u64;
  // proto:  QString QString::arg(short a, int fieldWidth, int base, QChar fillChar);
  fn C_ZNK7QString3argEsii5QChar(qthis: u64 /* *mut c_void*/, arg0: c_short, arg1: c_int, arg2: c_int, arg3: *mut c_void) -> *mut c_void;
  // proto:  void QString::QString(const QByteArray & a);
  fn C_ZN7QStringC2ERK10QByteArray(arg0: *mut c_void) -> u64;
  // proto: static QString QString::vasprintf(const char * format, va_list ap);
  fn C_ZN7QString9vasprintfEPKci(arg0: *mut c_char, arg1: c_int) -> *mut c_void;
  // proto:  qulonglong QString::toULongLong(bool * ok, int base);
  fn C_ZNK7QString11toULongLongEPbi(qthis: u64 /* *mut c_void*/, arg0: *mut c_char, arg1: c_int) -> c_ulonglong;
  // proto:  QString & QString::append(const char * s);
  fn C_ZN7QString6appendEPKc(qthis: u64 /* *mut c_void*/, arg0: *mut c_char) -> *mut c_void;
  // proto:  int QString::capacity();
  fn C_ZNK7QString8capacityEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QString::squeeze();
  fn C_ZN7QString7squeezeEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QString::truncate(int pos);
  fn C_ZN7QString8truncateEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  QString QString::arg(int a, int fieldWidth, int base, QChar fillChar);
  fn C_ZNK7QString3argEiii5QChar(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int, arg2: c_int, arg3: *mut c_void) -> *mut c_void;
  // proto:  QString QString::arg(QChar a, int fieldWidth, QChar fillChar);
  fn C_ZNK7QString3argE5QChariS0_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_int, arg2: *mut c_void) -> *mut c_void;
  // proto:  int QString::localeAwareCompare(const QString & s);
  fn C_ZNK7QString18localeAwareCompareERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_int;
  // proto:  QString & QString::remove(const QRegExp & rx);
  fn C_ZN7QString6removeERK7QRegExp(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  bool QString::contains(const QRegularExpression & re);
  fn C_ZNK7QString8containsERK18QRegularExpression(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  int QString::indexOf(QRegExp & , int from);
  fn C_ZNK7QString7indexOfER7QRegExpi(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_int) -> c_int;
  // proto:  QString & QString::replace(int i, int len, QChar after);
  fn C_ZN7QString7replaceEii5QChar(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int, arg2: *mut c_void) -> *mut c_void;
  // proto:  bool QString::isRightToLeft();
  fn C_ZNK7QString13isRightToLeftEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QString QString::arg(char a, int fieldWidth, QChar fillChar);
  fn C_ZNK7QString3argEci5QChar(qthis: u64 /* *mut c_void*/, arg0: c_char, arg1: c_int, arg2: *mut c_void) -> *mut c_void;
  // proto:  QVector<uint> QString::toUcs4();
  fn C_ZNK7QString6toUcs4Ev(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QString & QString::remove(int i, int len);
  fn C_ZN7QString6removeEii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int) -> *mut c_void;
  // proto:  bool QString::isEmpty();
  fn C_ZNK7QString7isEmptyEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QString QString::right(int n);
  fn C_ZNK7QString5rightEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> *mut c_void;
  // proto:  QString QString::rightJustified(int width, QChar fill, bool trunc);
  fn C_ZNK7QString14rightJustifiedEi5QCharb(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void, arg2: c_char) -> *mut c_void;
  // proto:  QString QString::arg(const QString & a, int fieldWidth, QChar fillChar);
  fn C_ZNK7QString3argERKS_i5QChar(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_int, arg2: *mut c_void) -> *mut c_void;
  // proto:  QString QString::arg(qulonglong a, int fieldwidth, int base, QChar fillChar);
  fn C_ZNK7QString3argEyii5QChar(qthis: u64 /* *mut c_void*/, arg0: c_ulonglong, arg1: c_int, arg2: c_int, arg3: *mut c_void) -> *mut c_void;
  // proto:  void QString::reserve(int size);
  fn C_ZN7QString7reserveEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  short QString::toShort(bool * ok, int base);
  fn C_ZNK7QString7toShortEPbi(qthis: u64 /* *mut c_void*/, arg0: *mut c_char, arg1: c_int) -> c_short;
  // proto:  QString QString::arg(ulong a, int fieldwidth, int base, QChar fillChar);
  fn C_ZNK7QString3argEmii5QChar(qthis: u64 /* *mut c_void*/, arg0: c_ulong, arg1: c_int, arg2: c_int, arg3: *mut c_void) -> *mut c_void;
  fn QLatin1String_Class_Size() -> c_int;
  // proto:  const char * QLatin1String::data();
  fn C_ZNK13QLatin1String4dataEv(qthis: u64 /* *mut c_void*/) -> *mut c_char;
  // proto:  void QLatin1String::QLatin1String(const char * s);
  fn C_ZN13QLatin1StringC2EPKc(arg0: *mut c_char) -> u64;
  // proto:  int QLatin1String::size();
  fn C_ZNK13QLatin1String4sizeEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QLatin1String::QLatin1String(const QByteArray & s);
  fn C_ZN13QLatin1StringC2ERK10QByteArray(arg0: *mut c_void) -> u64;
  // proto:  const char * QLatin1String::latin1();
  fn C_ZNK13QLatin1String6latin1Ev(qthis: u64 /* *mut c_void*/) -> *mut c_char;
  // proto:  void QLatin1String::QLatin1String(const char * s, int sz);
  fn C_ZN13QLatin1StringC2EPKci(arg0: *mut c_char, arg1: c_int) -> u64;
  fn QCharRef_Class_Size() -> c_int;
  // proto:  bool QCharRef::isLetterOrNumber();
  fn C_ZN8QCharRef16isLetterOrNumberEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QCharRef::isDigit();
  fn C_ZNK8QCharRef7isDigitEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  char QCharRef::toLatin1();
  fn C_ZNK8QCharRef8toLatin1Ev(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QCharRef::setCell(uchar cell);
  fn C_ZN8QCharRef7setCellEh(qthis: u64 /* *mut c_void*/, arg0: c_uchar);
  // proto:  bool QCharRef::isMark();
  fn C_ZNK8QCharRef6isMarkEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  int QCharRef::digitValue();
  fn C_ZNK8QCharRef10digitValueEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  bool QCharRef::isLetter();
  fn C_ZNK8QCharRef8isLetterEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QCharRef::isNumber();
  fn C_ZNK8QCharRef8isNumberEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QCharRef::isPrint();
  fn C_ZNK8QCharRef7isPrintEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QChar QCharRef::toLower();
  fn C_ZNK8QCharRef7toLowerEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QCharRef::setRow(uchar row);
  fn C_ZN8QCharRef6setRowEh(qthis: u64 /* *mut c_void*/, arg0: c_uchar);
  // proto:  bool QCharRef::isNull();
  fn C_ZNK8QCharRef6isNullEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QChar QCharRef::toTitleCase();
  fn C_ZNK8QCharRef11toTitleCaseEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QCharRef::hasMirrored();
  fn C_ZNK8QCharRef11hasMirroredEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  uchar QCharRef::row();
  fn C_ZNK8QCharRef3rowEv(qthis: u64 /* *mut c_void*/) -> c_uchar;
  // proto:  ushort & QCharRef::unicode();
  fn C_ZN8QCharRef7unicodeEv(qthis: u64 /* *mut c_void*/) -> c_ushort;
  // proto:  bool QCharRef::isTitleCase();
  fn C_ZNK8QCharRef11isTitleCaseEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QCharRef::isUpper();
  fn C_ZNK8QCharRef7isUpperEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  uchar QCharRef::cell();
  fn C_ZNK8QCharRef4cellEv(qthis: u64 /* *mut c_void*/) -> c_uchar;
  // proto:  QString QCharRef::decomposition();
  fn C_ZNK8QCharRef13decompositionEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  uchar QCharRef::combiningClass();
  fn C_ZNK8QCharRef14combiningClassEv(qthis: u64 /* *mut c_void*/) -> c_uchar;
  // proto:  QChar QCharRef::mirroredChar();
  fn C_ZNK8QCharRef12mirroredCharEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QCharRef::isSpace();
  fn C_ZNK8QCharRef7isSpaceEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QCharRef::isPunct();
  fn C_ZNK8QCharRef7isPunctEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QChar QCharRef::toUpper();
  fn C_ZNK8QCharRef7toUpperEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QCharRef::isLower();
  fn C_ZNK8QCharRef7isLowerEv(qthis: u64 /* *mut c_void*/) -> c_char;
  fn QStringRef_Class_Size() -> c_int;
  // proto:  short QStringRef::toShort(bool * ok, int base);
  fn C_ZNK10QStringRef7toShortEPbi(qthis: u64 /* *mut c_void*/, arg0: *mut c_char, arg1: c_int) -> c_short;
  // proto:  void QStringRef::QStringRef(const QString * string);
  fn C_ZN10QStringRefC2EPK7QString(arg0: *mut c_void) -> u64;
  // proto:  qulonglong QStringRef::toULongLong(bool * ok, int base);
  fn C_ZNK10QStringRef11toULongLongEPbi(qthis: u64 /* *mut c_void*/, arg0: *mut c_char, arg1: c_int) -> c_ulonglong;
  // proto:  void QStringRef::clear();
  fn C_ZN10QStringRef5clearEv(qthis: u64 /* *mut c_void*/);
  // proto:  int QStringRef::position();
  fn C_ZNK10QStringRef8positionEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  long QStringRef::toLong(bool * ok, int base);
  fn C_ZNK10QStringRef6toLongEPbi(qthis: u64 /* *mut c_void*/, arg0: *mut c_char, arg1: c_int) -> c_long;
  // proto:  const QChar * QStringRef::cbegin();
  fn C_ZNK10QStringRef6cbeginEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  ushort QStringRef::toUShort(bool * ok, int base);
  fn C_ZNK10QStringRef8toUShortEPbi(qthis: u64 /* *mut c_void*/, arg0: *mut c_char, arg1: c_int) -> c_ushort;
  // proto:  uint QStringRef::toUInt(bool * ok, int base);
  fn C_ZNK10QStringRef6toUIntEPbi(qthis: u64 /* *mut c_void*/, arg0: *mut c_char, arg1: c_int) -> c_uint;
  // proto:  bool QStringRef::isEmpty();
  fn C_ZNK10QStringRef7isEmptyEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  int QStringRef::localeAwareCompare(const QString & s);
  fn C_ZNK10QStringRef18localeAwareCompareERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_int;
  // proto:  QByteArray QStringRef::toUtf8();
  fn C_ZNK10QStringRef6toUtf8Ev(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  int QStringRef::size();
  fn C_ZNK10QStringRef4sizeEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  const QChar * QStringRef::constData();
  fn C_ZNK10QStringRef9constDataEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QStringRef QStringRef::left(int n);
  fn C_ZNK10QStringRef4leftEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> *mut c_void;
  // proto:  QVector<uint> QStringRef::toUcs4();
  fn C_ZNK10QStringRef6toUcs4Ev(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  int QStringRef::count();
  fn C_ZNK10QStringRef5countEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QStringRef::QStringRef(const QString * string, int position, int size);
  fn C_ZN10QStringRefC2EPK7QStringii(arg0: *mut c_void, arg1: c_int, arg2: c_int) -> u64;
  // proto:  void QStringRef::QStringRef();
  fn C_ZN10QStringRefC2Ev() -> u64;
  // proto:  QStringRef QStringRef::right(int n);
  fn C_ZNK10QStringRef5rightEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> *mut c_void;
  // proto:  const QChar QStringRef::at(int i);
  fn C_ZNK10QStringRef2atEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> *mut c_void;
  // proto:  double QStringRef::toDouble(bool * ok);
  fn C_ZNK10QStringRef8toDoubleEPb(qthis: u64 /* *mut c_void*/, arg0: *mut c_char) -> c_double;
  // proto:  bool QStringRef::isNull();
  fn C_ZNK10QStringRef6isNullEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  const QChar * QStringRef::data();
  fn C_ZNK10QStringRef4dataEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  qlonglong QStringRef::toLongLong(bool * ok, int base);
  fn C_ZNK10QStringRef10toLongLongEPbi(qthis: u64 /* *mut c_void*/, arg0: *mut c_char, arg1: c_int) -> c_longlong;
  // proto:  QByteArray QStringRef::toLatin1();
  fn C_ZNK10QStringRef8toLatin1Ev(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  const QChar * QStringRef::begin();
  fn C_ZNK10QStringRef5beginEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  const QChar * QStringRef::unicode();
  fn C_ZNK10QStringRef7unicodeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QStringRef QStringRef::mid(int pos, int n);
  fn C_ZNK10QStringRef3midEii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int) -> *mut c_void;
  // proto:  float QStringRef::toFloat(bool * ok);
  fn C_ZNK10QStringRef7toFloatEPb(qthis: u64 /* *mut c_void*/, arg0: *mut c_char) -> c_float;
  // proto:  const QString * QStringRef::string();
  fn C_ZNK10QStringRef6stringEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QString QStringRef::toString();
  fn C_ZNK10QStringRef8toStringEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QStringRef QStringRef::trimmed();
  fn C_ZNK10QStringRef7trimmedEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  int QStringRef::toInt(bool * ok, int base);
  fn C_ZNK10QStringRef5toIntEPbi(qthis: u64 /* *mut c_void*/, arg0: *mut c_char, arg1: c_int) -> c_int;
  // proto:  const QChar * QStringRef::cend();
  fn C_ZNK10QStringRef4cendEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QStringRef QStringRef::appendTo(QString * string);
  fn C_ZNK10QStringRef8appendToEP7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  int QStringRef::length();
  fn C_ZNK10QStringRef6lengthEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QStringRef::~QStringRef();
  fn C_ZN10QStringRefD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QByteArray QStringRef::toLocal8Bit();
  fn C_ZNK10QStringRef11toLocal8BitEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  ulong QStringRef::toULong(bool * ok, int base);
  fn C_ZNK10QStringRef7toULongEPbi(qthis: u64 /* *mut c_void*/, arg0: *mut c_char, arg1: c_int) -> c_ulong;
  // proto:  const QChar * QStringRef::end();
  fn C_ZNK10QStringRef3endEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
} // <= ext block end

// body block begin =>
// class sizeof(QStringDataPtr)=8
#[derive(Default)]
pub struct QStringDataPtr {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QString)=8
#[derive(Default)]
pub struct QString {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QLatin1String)=16
#[derive(Default)]
pub struct QLatin1String {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QCharRef)=16
#[derive(Default)]
pub struct QCharRef {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QStringRef)=16
#[derive(Default)]
pub struct QStringRef {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QStringDataPtr {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QStringDataPtr {
    return QStringDataPtr{qclsinst: qthis, ..Default::default()};
  }
}
impl /*struct*/ QString {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QString {
    return QString{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  qlonglong QString::toLongLong(bool * ok, int base);
impl /*struct*/ QString {
  pub fn toLongLong<RetType, T: QString_toLongLong<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toLongLong(self);
    // return 1;
  }
}

pub trait QString_toLongLong<RetType> {
  fn toLongLong(self , rsthis: & QString) -> RetType;
}

  // proto:  qlonglong QString::toLongLong(bool * ok, int base);
impl<'a> /*trait*/ QString_toLongLong<i64> for (&'a mut Vec<i8>, i32) {
  fn toLongLong(self , rsthis: & QString) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString10toLongLongEPbi()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {C_ZNK7QString10toLongLongEPbi(rsthis.qclsinst, arg0, arg1)};
    return ret as i64; // 1
    // return 1;
  }
}

  // proto:  bool QString::isNull();
impl /*struct*/ QString {
  pub fn isNull<RetType, T: QString_isNull<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isNull(self);
    // return 1;
  }
}

pub trait QString_isNull<RetType> {
  fn isNull(self , rsthis: & QString) -> RetType;
}

  // proto:  bool QString::isNull();
impl<'a> /*trait*/ QString_isNull<i8> for () {
  fn isNull(self , rsthis: & QString) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString6isNullEv()};
    let mut ret = unsafe {C_ZNK7QString6isNullEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  QString & QString::append(const QChar * uc, int len);
impl /*struct*/ QString {
  pub fn append<RetType, T: QString_append<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.append(self);
    // return 1;
  }
}

pub trait QString_append<RetType> {
  fn append(self , rsthis: & QString) -> RetType;
}

  // proto:  QString & QString::append(const QChar * uc, int len);
impl<'a> /*trait*/ QString_append<QString> for (&'a QChar, i32) {
  fn append(self , rsthis: & QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString6appendEPK5QChari()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {C_ZN7QString6appendEPK5QChari(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString & QString::prepend(QChar c);
impl /*struct*/ QString {
  pub fn prepend<RetType, T: QString_prepend<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.prepend(self);
    // return 1;
  }
}

pub trait QString_prepend<RetType> {
  fn prepend(self , rsthis: & QString) -> RetType;
}

  // proto:  QString & QString::prepend(QChar c);
impl<'a> /*trait*/ QString_prepend<QString> for (QChar) {
  fn prepend(self , rsthis: & QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString7prependE5QChar()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN7QString7prependE5QChar(rsthis.qclsinst, arg0)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString & QString::insert(int i, QChar c);
impl /*struct*/ QString {
  pub fn insert<RetType, T: QString_insert<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.insert(self);
    // return 1;
  }
}

pub trait QString_insert<RetType> {
  fn insert(self , rsthis: & QString) -> RetType;
}

  // proto:  QString & QString::insert(int i, QChar c);
impl<'a> /*trait*/ QString_insert<QString> for (i32, QChar) {
  fn insert(self , rsthis: & QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString6insertEi5QChar()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN7QString6insertEi5QChar(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QString::left(int n);
impl /*struct*/ QString {
  pub fn left<RetType, T: QString_left<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.left(self);
    // return 1;
  }
}

pub trait QString_left<RetType> {
  fn left(self , rsthis: & QString) -> RetType;
}

  // proto:  QString QString::left(int n);
impl<'a> /*trait*/ QString_left<QString> for (i32) {
  fn left(self , rsthis: & QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString4leftEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {C_ZNK7QString4leftEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QString::QString(QChar c);
impl /*struct*/ QString {
  pub fn new<T: QString_new>(value: T) -> QString {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QString_new {
  fn new(self) -> QString;
}

  // proto:  void QString::QString(QChar c);
impl<'a> /*trait*/ QString_new for (QChar) {
  fn new(self) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QStringC2E5QChar()};
    let ctysz: c_int = unsafe{QString_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN7QStringC2E5QChar(arg0)};
    let rsthis = QString{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QString & QString::prepend(const char * s);
impl<'a> /*trait*/ QString_prepend<QString> for (&'a  String) {
  fn prepend(self , rsthis: & QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString7prependEPKc()};
    let arg0 = self.as_ptr()  as *mut c_char;
    let mut ret = unsafe {C_ZN7QString7prependEPKc(rsthis.qclsinst, arg0)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QString::lastIndexOf(const QRegularExpression & re, int from);
impl /*struct*/ QString {
  pub fn lastIndexOf<RetType, T: QString_lastIndexOf<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.lastIndexOf(self);
    // return 1;
  }
}

pub trait QString_lastIndexOf<RetType> {
  fn lastIndexOf(self , rsthis: & QString) -> RetType;
}

  // proto:  int QString::lastIndexOf(const QRegularExpression & re, int from);
impl<'a> /*trait*/ QString_lastIndexOf<i32> for (&'a QRegularExpression, i32) {
  fn lastIndexOf(self , rsthis: & QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString11lastIndexOfERK18QRegularExpressioni()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {C_ZNK7QString11lastIndexOfERK18QRegularExpressioni(rsthis.qclsinst, arg0, arg1)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto: static QString QString::number(int , int base);
impl /*struct*/ QString {
  pub fn number_s<RetType, T: QString_number_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.number_s();
    // return 1;
  }
}

pub trait QString_number_s<RetType> {
  fn number_s(self ) -> RetType;
}

  // proto: static QString QString::number(int , int base);
impl<'a> /*trait*/ QString_number_s<QString> for (i32, i32) {
  fn number_s(self ) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString6numberEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {C_ZN7QString6numberEii(arg0, arg1)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QString::resize(int size);
impl /*struct*/ QString {
  pub fn resize<RetType, T: QString_resize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.resize(self);
    // return 1;
  }
}

pub trait QString_resize<RetType> {
  fn resize(self , rsthis: & QString) -> RetType;
}

  // proto:  void QString::resize(int size);
impl<'a> /*trait*/ QString_resize<()> for (i32) {
  fn resize(self , rsthis: & QString) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString6resizeEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN7QString6resizeEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QString::push_front(QChar c);
impl /*struct*/ QString {
  pub fn push_front<RetType, T: QString_push_front<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.push_front(self);
    // return 1;
  }
}

pub trait QString_push_front<RetType> {
  fn push_front(self , rsthis: & QString) -> RetType;
}

  // proto:  void QString::push_front(QChar c);
impl<'a> /*trait*/ QString_push_front<()> for (QChar) {
  fn push_front(self , rsthis: & QString) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString10push_frontE5QChar()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN7QString10push_frontE5QChar(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QString::QString();
impl<'a> /*trait*/ QString_new for () {
  fn new(self) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QStringC2Ev()};
    let ctysz: c_int = unsafe{QString_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let qthis: u64 = unsafe {C_ZN7QStringC2Ev()};
    let rsthis = QString{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  double QString::toDouble(bool * ok);
impl /*struct*/ QString {
  pub fn toDouble<RetType, T: QString_toDouble<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toDouble(self);
    // return 1;
  }
}

pub trait QString_toDouble<RetType> {
  fn toDouble(self , rsthis: & QString) -> RetType;
}

  // proto:  double QString::toDouble(bool * ok);
impl<'a> /*trait*/ QString_toDouble<f64> for (&'a mut Vec<i8>) {
  fn toDouble(self , rsthis: & QString) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString8toDoubleEPb()};
    let arg0 = self.as_ptr()  as *mut c_char;
    let mut ret = unsafe {C_ZNK7QString8toDoubleEPb(rsthis.qclsinst, arg0)};
    return ret as f64; // 1
    // return 1;
  }
}

  // proto:  QString QString::arg(const QString & a1, const QString & a2, const QString & a3, const QString & a4, const QString & a5, const QString & a6, const QString & a7, const QString & a8);
impl /*struct*/ QString {
  pub fn arg<RetType, T: QString_arg<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.arg(self);
    // return 1;
  }
}

pub trait QString_arg<RetType> {
  fn arg(self , rsthis: & QString) -> RetType;
}

  // proto:  QString QString::arg(const QString & a1, const QString & a2, const QString & a3, const QString & a4, const QString & a5, const QString & a6, const QString & a7, const QString & a8);
impl<'a> /*trait*/ QString_arg<QString> for (&'a QString, &'a QString, &'a QString, &'a QString, &'a QString, &'a QString, &'a QString, &'a QString) {
  fn arg(self , rsthis: & QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString3argERKS_S1_S1_S1_S1_S1_S1_S1_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let arg3 = self.3.qclsinst  as *mut c_void;
    let arg4 = self.4.qclsinst  as *mut c_void;
    let arg5 = self.5.qclsinst  as *mut c_void;
    let arg6 = self.6.qclsinst  as *mut c_void;
    let arg7 = self.7.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK7QString3argERKS_S1_S1_S1_S1_S1_S1_S1_(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QStringRef QString::rightRef(int n);
impl /*struct*/ QString {
  pub fn rightRef<RetType, T: QString_rightRef<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rightRef(self);
    // return 1;
  }
}

pub trait QString_rightRef<RetType> {
  fn rightRef(self , rsthis: & QString) -> RetType;
}

  // proto:  QStringRef QString::rightRef(int n);
impl<'a> /*trait*/ QString_rightRef<QStringRef> for (i32) {
  fn rightRef(self , rsthis: & QString) -> QStringRef {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString8rightRefEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {C_ZNK7QString8rightRefEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QStringRef::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString & QString::setNum(short , int base);
impl /*struct*/ QString {
  pub fn setNum<RetType, T: QString_setNum<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setNum(self);
    // return 1;
  }
}

pub trait QString_setNum<RetType> {
  fn setNum(self , rsthis: & QString) -> RetType;
}

  // proto:  QString & QString::setNum(short , int base);
impl<'a> /*trait*/ QString_setNum<QString> for (i16, i32) {
  fn setNum(self , rsthis: & QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString6setNumEsi()};
    let arg0 = self.0  as c_short;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {C_ZN7QString6setNumEsi(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QString::QString(const QChar * unicode, int size);
impl<'a> /*trait*/ QString_new for (&'a QChar, i32) {
  fn new(self) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QStringC2EPK5QChari()};
    let ctysz: c_int = unsafe{QString_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let qthis: u64 = unsafe {C_ZN7QStringC2EPK5QChari(arg0, arg1)};
    let rsthis = QString{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  float QString::toFloat(bool * ok);
impl /*struct*/ QString {
  pub fn toFloat<RetType, T: QString_toFloat<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toFloat(self);
    // return 1;
  }
}

pub trait QString_toFloat<RetType> {
  fn toFloat(self , rsthis: & QString) -> RetType;
}

  // proto:  float QString::toFloat(bool * ok);
impl<'a> /*trait*/ QString_toFloat<f32> for (&'a mut Vec<i8>) {
  fn toFloat(self , rsthis: & QString) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString7toFloatEPb()};
    let arg0 = self.as_ptr()  as *mut c_char;
    let mut ret = unsafe {C_ZNK7QString7toFloatEPb(rsthis.qclsinst, arg0)};
    return ret as f32; // 1
    // return 1;
  }
}

  // proto:  int QString::count(const QRegularExpression & re);
impl /*struct*/ QString {
  pub fn count<RetType, T: QString_count<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.count(self);
    // return 1;
  }
}

pub trait QString_count<RetType> {
  fn count(self , rsthis: & QString) -> RetType;
}

  // proto:  int QString::count(const QRegularExpression & re);
impl<'a> /*trait*/ QString_count<i32> for (&'a QRegularExpression) {
  fn count(self , rsthis: & QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString5countERK18QRegularExpression()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK7QString5countERK18QRegularExpression(rsthis.qclsinst, arg0)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  QStringRef QString::midRef(int position, int n);
impl /*struct*/ QString {
  pub fn midRef<RetType, T: QString_midRef<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.midRef(self);
    // return 1;
  }
}

pub trait QString_midRef<RetType> {
  fn midRef(self , rsthis: & QString) -> RetType;
}

  // proto:  QStringRef QString::midRef(int position, int n);
impl<'a> /*trait*/ QString_midRef<QStringRef> for (i32, i32) {
  fn midRef(self , rsthis: & QString) -> QStringRef {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString6midRefEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {C_ZNK7QString6midRefEii(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QStringRef::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QString::detach();
impl /*struct*/ QString {
  pub fn detach<RetType, T: QString_detach<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.detach(self);
    // return 1;
  }
}

pub trait QString_detach<RetType> {
  fn detach(self , rsthis: & QString) -> RetType;
}

  // proto:  void QString::detach();
impl<'a> /*trait*/ QString_detach<()> for () {
  fn detach(self , rsthis: & QString) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString6detachEv()};
     unsafe {C_ZN7QString6detachEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QString QString::arg(const QString & a1, const QString & a2, const QString & a3, const QString & a4);
impl<'a> /*trait*/ QString_arg<QString> for (&'a QString, &'a QString, &'a QString, &'a QString) {
  fn arg(self , rsthis: & QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString3argERKS_S1_S1_S1_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let arg3 = self.3.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK7QString3argERKS_S1_S1_S1_(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QString::arg(long a, int fieldwidth, int base, QChar fillChar);
impl<'a> /*trait*/ QString_arg<QString> for (i64, i32, i32, QChar) {
  fn arg(self , rsthis: & QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString3argElii5QChar()};
    let arg0 = self.0  as c_long;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK7QString3argElii5QChar(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QString::count();
impl<'a> /*trait*/ QString_count<i32> for () {
  fn count(self , rsthis: & QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString5countEv()};
    let mut ret = unsafe {C_ZNK7QString5countEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  QString & QString::setNum(qulonglong , int base);
impl<'a> /*trait*/ QString_setNum<QString> for (u64, i32) {
  fn setNum(self , rsthis: & QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString6setNumEyi()};
    let arg0 = self.0  as c_ulonglong;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {C_ZN7QString6setNumEyi(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QString::push_back(QChar c);
impl /*struct*/ QString {
  pub fn push_back<RetType, T: QString_push_back<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.push_back(self);
    // return 1;
  }
}

pub trait QString_push_back<RetType> {
  fn push_back(self , rsthis: & QString) -> RetType;
}

  // proto:  void QString::push_back(QChar c);
impl<'a> /*trait*/ QString_push_back<()> for (QChar) {
  fn push_back(self , rsthis: & QString) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString9push_backE5QChar()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN7QString9push_backE5QChar(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QString & QString::setNum(float , char f, int prec);
impl<'a> /*trait*/ QString_setNum<QString> for (f32, i8, i32) {
  fn setNum(self , rsthis: & QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString6setNumEfci()};
    let arg0 = self.0  as c_float;
    let arg1 = self.1  as c_char;
    let arg2 = self.2  as c_int;
    let mut ret = unsafe {C_ZN7QString6setNumEfci(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QString::count(const QRegExp & );
impl<'a> /*trait*/ QString_count<i32> for (&'a QRegExp) {
  fn count(self , rsthis: & QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString5countERK7QRegExp()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK7QString5countERK7QRegExp(rsthis.qclsinst, arg0)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  int QString::size();
impl /*struct*/ QString {
  pub fn size<RetType, T: QString_size<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.size(self);
    // return 1;
  }
}

pub trait QString_size<RetType> {
  fn size(self , rsthis: & QString) -> RetType;
}

  // proto:  int QString::size();
impl<'a> /*trait*/ QString_size<i32> for () {
  fn size(self , rsthis: & QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString4sizeEv()};
    let mut ret = unsafe {C_ZNK7QString4sizeEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  QString & QString::insert(int i, const QChar * uc, int len);
impl<'a> /*trait*/ QString_insert<QString> for (i32, &'a QChar, i32) {
  fn insert(self , rsthis: & QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString6insertEiPK5QChari()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_int;
    let mut ret = unsafe {C_ZN7QString6insertEiPK5QChari(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString & QString::replace(int i, int len, const QChar * s, int slen);
impl /*struct*/ QString {
  pub fn replace<RetType, T: QString_replace<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.replace(self);
    // return 1;
  }
}

pub trait QString_replace<RetType> {
  fn replace(self , rsthis: & QString) -> RetType;
}

  // proto:  QString & QString::replace(int i, int len, const QChar * s, int slen);
impl<'a> /*trait*/ QString_replace<QString> for (i32, i32, &'a QChar, i32) {
  fn replace(self , rsthis: & QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString7replaceEiiPK5QChari()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let arg3 = self.3  as c_int;
    let mut ret = unsafe {C_ZN7QString7replaceEiiPK5QChari(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto: static QString QString::fromRawData(const QChar * , int size);
impl /*struct*/ QString {
  pub fn fromRawData_s<RetType, T: QString_fromRawData_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromRawData_s();
    // return 1;
  }
}

pub trait QString_fromRawData_s<RetType> {
  fn fromRawData_s(self ) -> RetType;
}

  // proto: static QString QString::fromRawData(const QChar * , int size);
impl<'a> /*trait*/ QString_fromRawData_s<QString> for (&'a QChar, i32) {
  fn fromRawData_s(self ) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString11fromRawDataEPK5QChari()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {C_ZN7QString11fromRawDataEPK5QChari(arg0, arg1)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QString::trimmed();
impl /*struct*/ QString {
  pub fn trimmed<RetType, T: QString_trimmed<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.trimmed(self);
    // return 1;
  }
}

pub trait QString_trimmed<RetType> {
  fn trimmed(self , rsthis: & QString) -> RetType;
}

  // proto:  QString QString::trimmed();
impl<'a> /*trait*/ QString_trimmed<QString> for () {
  fn trimmed(self , rsthis: & QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNO7QString7trimmedEv()};
    let mut ret = unsafe {C_ZNO7QString7trimmedEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString & QString::insert(int i, const QString & s);
impl<'a> /*trait*/ QString_insert<QString> for (i32, &'a QString) {
  fn insert(self , rsthis: & QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString6insertEiRKS_()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN7QString6insertEiRKS_(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QString::arg(const QString & a1, const QString & a2, const QString & a3, const QString & a4, const QString & a5);
impl<'a> /*trait*/ QString_arg<QString> for (&'a QString, &'a QString, &'a QString, &'a QString, &'a QString) {
  fn arg(self , rsthis: & QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString3argERKS_S1_S1_S1_S1_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let arg3 = self.3.qclsinst  as *mut c_void;
    let arg4 = self.4.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK7QString3argERKS_S1_S1_S1_S1_(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString & QString::setRawData(const QChar * unicode, int size);
impl /*struct*/ QString {
  pub fn setRawData<RetType, T: QString_setRawData<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setRawData(self);
    // return 1;
  }
}

pub trait QString_setRawData<RetType> {
  fn setRawData(self , rsthis: & QString) -> RetType;
}

  // proto:  QString & QString::setRawData(const QChar * unicode, int size);
impl<'a> /*trait*/ QString_setRawData<QString> for (&'a QChar, i32) {
  fn setRawData(self , rsthis: & QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString10setRawDataEPK5QChari()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {C_ZN7QString10setRawDataEPK5QChari(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString & QString::prepend(const QString & s);
impl<'a> /*trait*/ QString_prepend<QString> for (&'a QString) {
  fn prepend(self , rsthis: & QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString7prependERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN7QString7prependERKS_(rsthis.qclsinst, arg0)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString & QString::sprintf(const char * format);
impl /*struct*/ QString {
  pub fn sprintf<RetType, T: QString_sprintf<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sprintf(self);
    // return 1;
  }
}

pub trait QString_sprintf<RetType> {
  fn sprintf(self , rsthis: & QString) -> RetType;
}

  // proto:  QString & QString::sprintf(const char * format);
impl<'a> /*trait*/ QString_sprintf<QString> for (&'a  String) {
  fn sprintf(self , rsthis: & QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString7sprintfEPKcz()};
    let arg0 = self.as_ptr()  as *mut c_char;
    let mut ret = unsafe {C_ZN7QString7sprintfEPKcz(rsthis.qclsinst, arg0)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  ulong QString::toULong(bool * ok, int base);
impl /*struct*/ QString {
  pub fn toULong<RetType, T: QString_toULong<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toULong(self);
    // return 1;
  }
}

pub trait QString_toULong<RetType> {
  fn toULong(self , rsthis: & QString) -> RetType;
}

  // proto:  ulong QString::toULong(bool * ok, int base);
impl<'a> /*trait*/ QString_toULong<u64> for (&'a mut Vec<i8>, i32) {
  fn toULong(self , rsthis: & QString) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString7toULongEPbi()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {C_ZNK7QString7toULongEPbi(rsthis.qclsinst, arg0, arg1)};
    return ret as u64; // 1
    // return 1;
  }
}

  // proto:  void QString::chop(int n);
impl /*struct*/ QString {
  pub fn chop<RetType, T: QString_chop<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.chop(self);
    // return 1;
  }
}

pub trait QString_chop<RetType> {
  fn chop(self , rsthis: & QString) -> RetType;
}

  // proto:  void QString::chop(int n);
impl<'a> /*trait*/ QString_chop<()> for (i32) {
  fn chop(self , rsthis: & QString) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString4chopEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN7QString4chopEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto: static QString QString::fromUtf16(const ushort * , int size);
impl /*struct*/ QString {
  pub fn fromUtf16_s<RetType, T: QString_fromUtf16_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromUtf16_s();
    // return 1;
  }
}

pub trait QString_fromUtf16_s<RetType> {
  fn fromUtf16_s(self ) -> RetType;
}

  // proto: static QString QString::fromUtf16(const ushort * , int size);
impl<'a> /*trait*/ QString_fromUtf16_s<QString> for (&'a  Vec<u16>, i32) {
  fn fromUtf16_s(self ) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString9fromUtf16EPKti()};
    let arg0 = self.0.as_ptr()  as *mut c_ushort;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {C_ZN7QString9fromUtf16EPKti(arg0, arg1)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QString::isDetached();
impl /*struct*/ QString {
  pub fn isDetached<RetType, T: QString_isDetached<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isDetached(self);
    // return 1;
  }
}

pub trait QString_isDetached<RetType> {
  fn isDetached(self , rsthis: & QString) -> RetType;
}

  // proto:  bool QString::isDetached();
impl<'a> /*trait*/ QString_isDetached<i8> for () {
  fn isDetached(self , rsthis: & QString) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString10isDetachedEv()};
    let mut ret = unsafe {C_ZNK7QString10isDetachedEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  QString & QString::setNum(qlonglong , int base);
impl<'a> /*trait*/ QString_setNum<QString> for (i64, i32) {
  fn setNum(self , rsthis: & QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString6setNumExi()};
    let arg0 = self.0  as c_longlong;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {C_ZN7QString6setNumExi(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QString::mid(int position, int n);
impl /*struct*/ QString {
  pub fn mid<RetType, T: QString_mid<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.mid(self);
    // return 1;
  }
}

pub trait QString_mid<RetType> {
  fn mid(self , rsthis: & QString) -> RetType;
}

  // proto:  QString QString::mid(int position, int n);
impl<'a> /*trait*/ QString_mid<QString> for (i32, i32) {
  fn mid(self , rsthis: & QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString3midEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {C_ZNK7QString3midEii(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto: static QString QString::fromLocal8Bit(const char * str, int size);
impl /*struct*/ QString {
  pub fn fromLocal8Bit_s<RetType, T: QString_fromLocal8Bit_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromLocal8Bit_s();
    // return 1;
  }
}

pub trait QString_fromLocal8Bit_s<RetType> {
  fn fromLocal8Bit_s(self ) -> RetType;
}

  // proto: static QString QString::fromLocal8Bit(const char * str, int size);
impl<'a> /*trait*/ QString_fromLocal8Bit_s<QString> for (&'a  String, i32) {
  fn fromLocal8Bit_s(self ) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString13fromLocal8BitEPKci()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {C_ZN7QString13fromLocal8BitEPKci(arg0, arg1)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QString::swap(QString & other);
impl /*struct*/ QString {
  pub fn swap<RetType, T: QString_swap<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.swap(self);
    // return 1;
  }
}

pub trait QString_swap<RetType> {
  fn swap(self , rsthis: & QString) -> RetType;
}

  // proto:  void QString::swap(QString & other);
impl<'a> /*trait*/ QString_swap<()> for (&'a QString) {
  fn swap(self , rsthis: & QString) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN7QString4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QString & QString::vsprintf(const char * format, va_list ap);
impl /*struct*/ QString {
  pub fn vsprintf<RetType, T: QString_vsprintf<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.vsprintf(self);
    // return 1;
  }
}

pub trait QString_vsprintf<RetType> {
  fn vsprintf(self , rsthis: & QString) -> RetType;
}

  // proto:  QString & QString::vsprintf(const char * format, va_list ap);
impl<'a> /*trait*/ QString_vsprintf<QString> for (&'a  String, i32) {
  fn vsprintf(self , rsthis: & QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString8vsprintfEPKci()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {C_ZN7QString8vsprintfEPKci(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto: static QString QString::fromUtf8(const QByteArray & str);
impl /*struct*/ QString {
  pub fn fromUtf8_s<RetType, T: QString_fromUtf8_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromUtf8_s();
    // return 1;
  }
}

pub trait QString_fromUtf8_s<RetType> {
  fn fromUtf8_s(self ) -> RetType;
}

  // proto: static QString QString::fromUtf8(const QByteArray & str);
impl<'a> /*trait*/ QString_fromUtf8_s<QString> for (&'a QByteArray) {
  fn fromUtf8_s(self ) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString8fromUtf8ERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN7QString8fromUtf8ERK10QByteArray(arg0)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto: static QString QString::fromUcs4(const char32_t * str, int size);
impl /*struct*/ QString {
  pub fn fromUcs4_s<RetType, T: QString_fromUcs4_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromUcs4_s();
    // return 1;
  }
}

pub trait QString_fromUcs4_s<RetType> {
  fn fromUcs4_s(self ) -> RetType;
}

  // proto: static QString QString::fromUcs4(const char32_t * str, int size);
impl<'a> /*trait*/ QString_fromUcs4_s<QString> for (&'a  String, i32) {
  fn fromUcs4_s(self ) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString8fromUcs4EPKDii()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {C_ZN7QString8fromUcs4EPKDii(arg0, arg1)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QString::leftJustified(int width, QChar fill, bool trunc);
impl /*struct*/ QString {
  pub fn leftJustified<RetType, T: QString_leftJustified<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.leftJustified(self);
    // return 1;
  }
}

pub trait QString_leftJustified<RetType> {
  fn leftJustified(self , rsthis: & QString) -> RetType;
}

  // proto:  QString QString::leftJustified(int width, QChar fill, bool trunc);
impl<'a> /*trait*/ QString_leftJustified<QString> for (i32, QChar, i8) {
  fn leftJustified(self , rsthis: & QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString13leftJustifiedEi5QCharb()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_char;
    let mut ret = unsafe {C_ZNK7QString13leftJustifiedEi5QCharb(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QString::indexOf(const QRegExp & , int from);
impl /*struct*/ QString {
  pub fn indexOf<RetType, T: QString_indexOf<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.indexOf(self);
    // return 1;
  }
}

pub trait QString_indexOf<RetType> {
  fn indexOf(self , rsthis: & QString) -> RetType;
}

  // proto:  int QString::indexOf(const QRegExp & , int from);
impl<'a> /*trait*/ QString_indexOf<i32> for (&'a QRegExp, i32) {
  fn indexOf(self , rsthis: & QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString7indexOfERK7QRegExpi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {C_ZNK7QString7indexOfERK7QRegExpi(rsthis.qclsinst, arg0, arg1)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  void QString::push_back(const QString & s);
impl<'a> /*trait*/ QString_push_back<()> for (&'a QString) {
  fn push_back(self , rsthis: & QString) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString9push_backERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN7QString9push_backERKS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QString::lastIndexOf(QRegExp & , int from);
impl<'a> /*trait*/ QString_lastIndexOf<i32> for (&'a QRegExp, i32) {
  fn lastIndexOf(self , rsthis: & QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString11lastIndexOfER7QRegExpi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {C_ZNK7QString11lastIndexOfER7QRegExpi(rsthis.qclsinst, arg0, arg1)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  QString QString::arg(const QString & a1, const QString & a2, const QString & a3);
impl<'a> /*trait*/ QString_arg<QString> for (&'a QString, &'a QString, &'a QString) {
  fn arg(self , rsthis: & QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString3argERKS_S1_S1_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK7QString3argERKS_S1_S1_(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  const ushort * QString::utf16();
impl /*struct*/ QString {
  pub fn utf16<RetType, T: QString_utf16<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.utf16(self);
    // return 1;
  }
}

pub trait QString_utf16<RetType> {
  fn utf16(self , rsthis: & QString) -> RetType;
}

  // proto:  const ushort * QString::utf16();
impl<'a> /*trait*/ QString_utf16<*mut u16> for () {
  fn utf16(self , rsthis: & QString) -> *mut u16 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString5utf16Ev()};
    let mut ret = unsafe {C_ZNK7QString5utf16Ev(rsthis.qclsinst)};
    return ret as *mut u16; // 1
    // return 1;
  }
}

  // proto:  int QString::toInt(bool * ok, int base);
impl /*struct*/ QString {
  pub fn toInt<RetType, T: QString_toInt<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toInt(self);
    // return 1;
  }
}

pub trait QString_toInt<RetType> {
  fn toInt(self , rsthis: & QString) -> RetType;
}

  // proto:  int QString::toInt(bool * ok, int base);
impl<'a> /*trait*/ QString_toInt<i32> for (&'a mut Vec<i8>, i32) {
  fn toInt(self , rsthis: & QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString5toIntEPbi()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {C_ZNK7QString5toIntEPbi(rsthis.qclsinst, arg0, arg1)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  QByteArray QString::toUtf8();
impl /*struct*/ QString {
  pub fn toUtf8<RetType, T: QString_toUtf8<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toUtf8(self);
    // return 1;
  }
}

pub trait QString_toUtf8<RetType> {
  fn toUtf8(self , rsthis: & QString) -> RetType;
}

  // proto:  QByteArray QString::toUtf8();
impl<'a> /*trait*/ QString_toUtf8<QByteArray> for () {
  fn toUtf8(self , rsthis: & QString) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNO7QString6toUtf8Ev()};
    let mut ret = unsafe {C_ZNO7QString6toUtf8Ev(rsthis.qclsinst)};
    let mut ret1 = QByteArray::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QString::arg(double a, int fieldWidth, char fmt, int prec, QChar fillChar);
impl<'a> /*trait*/ QString_arg<QString> for (f64, i32, i8, i32, QChar) {
  fn arg(self , rsthis: & QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString3argEdici5QChar()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_char;
    let arg3 = self.3  as c_int;
    let arg4 = self.4.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK7QString3argEdici5QChar(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QChar * QString::data();
impl /*struct*/ QString {
  pub fn data<RetType, T: QString_data<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.data(self);
    // return 1;
  }
}

pub trait QString_data<RetType> {
  fn data(self , rsthis: & QString) -> RetType;
}

  // proto:  QChar * QString::data();
impl<'a> /*trait*/ QString_data<QChar> for () {
  fn data(self , rsthis: & QString) -> QChar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString4dataEv()};
    let mut ret = unsafe {C_ZN7QString4dataEv(rsthis.qclsinst)};
    let mut ret1 = QChar::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QString::toCaseFolded();
impl /*struct*/ QString {
  pub fn toCaseFolded<RetType, T: QString_toCaseFolded<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toCaseFolded(self);
    // return 1;
  }
}

pub trait QString_toCaseFolded<RetType> {
  fn toCaseFolded(self , rsthis: & QString) -> RetType;
}

  // proto:  QString QString::toCaseFolded();
impl<'a> /*trait*/ QString_toCaseFolded<QString> for () {
  fn toCaseFolded(self , rsthis: & QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNO7QString12toCaseFoldedEv()};
    let mut ret = unsafe {C_ZNO7QString12toCaseFoldedEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString & QString::setNum(uint , int base);
impl<'a> /*trait*/ QString_setNum<QString> for (u32, i32) {
  fn setNum(self , rsthis: & QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString6setNumEji()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {C_ZN7QString6setNumEji(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto: static int QString::localeAwareCompare(const QString & s1, const QString & s2);
impl /*struct*/ QString {
  pub fn localeAwareCompare_s<RetType, T: QString_localeAwareCompare_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.localeAwareCompare_s();
    // return 1;
  }
}

pub trait QString_localeAwareCompare_s<RetType> {
  fn localeAwareCompare_s(self ) -> RetType;
}

  // proto: static int QString::localeAwareCompare(const QString & s1, const QString & s2);
impl<'a> /*trait*/ QString_localeAwareCompare_s<i32> for (&'a QString, &'a QString) {
  fn localeAwareCompare_s(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString18localeAwareCompareERKS_S1_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN7QString18localeAwareCompareERKS_S1_(arg0, arg1)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  void QString::QString(const char * ch);
impl<'a> /*trait*/ QString_new for (&'a  String) {
  fn new(self) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QStringC2EPKc()};
    let ctysz: c_int = unsafe{QString_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.as_ptr()  as *mut c_char;
    let qthis: u64 = unsafe {C_ZN7QStringC2EPKc(arg0)};
    let rsthis = QString{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto: static QString QString::fromUtf16(const char16_t * str, int size);
impl<'a> /*trait*/ QString_fromUtf16_s<QString> for (&'a  String, i32) {
  fn fromUtf16_s(self ) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString9fromUtf16EPKDsi()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {C_ZN7QString9fromUtf16EPKDsi(arg0, arg1)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString & QString::replace(const QRegExp & rx, const QString & after);
impl<'a> /*trait*/ QString_replace<QString> for (&'a QRegExp, &'a QString) {
  fn replace(self , rsthis: & QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString7replaceERK7QRegExpRKS_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN7QString7replaceERK7QRegExpRKS_(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QString::repeated(int times);
impl /*struct*/ QString {
  pub fn repeated<RetType, T: QString_repeated<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.repeated(self);
    // return 1;
  }
}

pub trait QString_repeated<RetType> {
  fn repeated(self , rsthis: & QString) -> RetType;
}

  // proto:  QString QString::repeated(int times);
impl<'a> /*trait*/ QString_repeated<QString> for (i32) {
  fn repeated(self , rsthis: & QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString8repeatedEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {C_ZNK7QString8repeatedEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QString::toLower();
impl /*struct*/ QString {
  pub fn toLower<RetType, T: QString_toLower<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toLower(self);
    // return 1;
  }
}

pub trait QString_toLower<RetType> {
  fn toLower(self , rsthis: & QString) -> RetType;
}

  // proto:  QString QString::toLower();
impl<'a> /*trait*/ QString_toLower<QString> for () {
  fn toLower(self , rsthis: & QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNO7QString7toLowerEv()};
    let mut ret = unsafe {C_ZNO7QString7toLowerEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString & QString::setUtf16(const ushort * utf16, int size);
impl /*struct*/ QString {
  pub fn setUtf16<RetType, T: QString_setUtf16<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setUtf16(self);
    // return 1;
  }
}

pub trait QString_setUtf16<RetType> {
  fn setUtf16(self , rsthis: & QString) -> RetType;
}

  // proto:  QString & QString::setUtf16(const ushort * utf16, int size);
impl<'a> /*trait*/ QString_setUtf16<QString> for (&'a  Vec<u16>, i32) {
  fn setUtf16(self , rsthis: & QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString8setUtf16EPKti()};
    let arg0 = self.0.as_ptr()  as *mut c_ushort;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {C_ZN7QString8setUtf16EPKti(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QString::clear();
impl /*struct*/ QString {
  pub fn clear<RetType, T: QString_clear<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clear(self);
    // return 1;
  }
}

pub trait QString_clear<RetType> {
  fn clear(self , rsthis: & QString) -> RetType;
}

  // proto:  void QString::clear();
impl<'a> /*trait*/ QString_clear<()> for () {
  fn clear(self , rsthis: & QString) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString5clearEv()};
     unsafe {C_ZN7QString5clearEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QString::contains(const QRegExp & rx);
impl /*struct*/ QString {
  pub fn contains<RetType, T: QString_contains<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.contains(self);
    // return 1;
  }
}

pub trait QString_contains<RetType> {
  fn contains(self , rsthis: & QString) -> RetType;
}

  // proto:  bool QString::contains(const QRegExp & rx);
impl<'a> /*trait*/ QString_contains<i8> for (&'a QRegExp) {
  fn contains(self , rsthis: & QString) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString8containsERK7QRegExp()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK7QString8containsERK7QRegExp(rsthis.qclsinst, arg0)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  bool QString::isSharedWith(const QString & other);
impl /*struct*/ QString {
  pub fn isSharedWith<RetType, T: QString_isSharedWith<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isSharedWith(self);
    // return 1;
  }
}

pub trait QString_isSharedWith<RetType> {
  fn isSharedWith(self , rsthis: & QString) -> RetType;
}

  // proto:  bool QString::isSharedWith(const QString & other);
impl<'a> /*trait*/ QString_isSharedWith<i8> for (&'a QString) {
  fn isSharedWith(self , rsthis: & QString) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString12isSharedWithERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK7QString12isSharedWithERKS_(rsthis.qclsinst, arg0)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto: static QString QString::fromLatin1(const QByteArray & str);
impl /*struct*/ QString {
  pub fn fromLatin1_s<RetType, T: QString_fromLatin1_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromLatin1_s();
    // return 1;
  }
}

pub trait QString_fromLatin1_s<RetType> {
  fn fromLatin1_s(self ) -> RetType;
}

  // proto: static QString QString::fromLatin1(const QByteArray & str);
impl<'a> /*trait*/ QString_fromLatin1_s<QString> for (&'a QByteArray) {
  fn fromLatin1_s(self ) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString10fromLatin1ERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN7QString10fromLatin1ERK10QByteArray(arg0)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QString::~QString();
impl /*struct*/ QString {
  pub fn free<RetType, T: QString_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QString_free<RetType> {
  fn free(self , rsthis: & QString) -> RetType;
}

  // proto:  void QString::~QString();
impl<'a> /*trait*/ QString_free<()> for () {
  fn free(self , rsthis: & QString) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QStringD2Ev()};
     unsafe {C_ZN7QStringD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QString & QString::remove(const QRegularExpression & re);
impl /*struct*/ QString {
  pub fn remove<RetType, T: QString_remove<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.remove(self);
    // return 1;
  }
}

pub trait QString_remove<RetType> {
  fn remove(self , rsthis: & QString) -> RetType;
}

  // proto:  QString & QString::remove(const QRegularExpression & re);
impl<'a> /*trait*/ QString_remove<QString> for (&'a QRegularExpression) {
  fn remove(self , rsthis: & QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString6removeERK18QRegularExpression()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN7QString6removeERK18QRegularExpression(rsthis.qclsinst, arg0)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString & QString::setNum(int , int base);
impl<'a> /*trait*/ QString_setNum<QString> for (i32, i32) {
  fn setNum(self , rsthis: & QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString6setNumEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {C_ZN7QString6setNumEii(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  const_iterator QString::cend();
impl /*struct*/ QString {
  pub fn cend<RetType, T: QString_cend<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.cend(self);
    // return 1;
  }
}

pub trait QString_cend<RetType> {
  fn cend(self , rsthis: & QString) -> RetType;
}

  // proto:  const_iterator QString::cend();
impl<'a> /*trait*/ QString_cend<QChar> for () {
  fn cend(self , rsthis: & QString) -> QChar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString4cendEv()};
    let mut ret = unsafe {C_ZNK7QString4cendEv(rsthis.qclsinst)};
    let mut ret1 = QChar::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QString::toHtmlEscaped();
impl /*struct*/ QString {
  pub fn toHtmlEscaped<RetType, T: QString_toHtmlEscaped<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toHtmlEscaped(self);
    // return 1;
  }
}

pub trait QString_toHtmlEscaped<RetType> {
  fn toHtmlEscaped(self , rsthis: & QString) -> RetType;
}

  // proto:  QString QString::toHtmlEscaped();
impl<'a> /*trait*/ QString_toHtmlEscaped<QString> for () {
  fn toHtmlEscaped(self , rsthis: & QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString13toHtmlEscapedEv()};
    let mut ret = unsafe {C_ZNK7QString13toHtmlEscapedEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QString::lastIndexOf(const QRegularExpression & re, int from, QRegularExpressionMatch * rmatch);
impl<'a> /*trait*/ QString_lastIndexOf<i32> for (&'a QRegularExpression, i32, &'a QRegularExpressionMatch) {
  fn lastIndexOf(self , rsthis: & QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString11lastIndexOfERK18QRegularExpressioniP23QRegularExpressionMatch()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK7QString11lastIndexOfERK18QRegularExpressioniP23QRegularExpressionMatch(rsthis.qclsinst, arg0, arg1, arg2)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  QString & QString::append(const QByteArray & s);
impl<'a> /*trait*/ QString_append<QString> for (&'a QByteArray) {
  fn append(self , rsthis: & QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString6appendERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN7QString6appendERK10QByteArray(rsthis.qclsinst, arg0)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto: static QString QString::fromLatin1(const char * str, int size);
impl<'a> /*trait*/ QString_fromLatin1_s<QString> for (&'a  String, i32) {
  fn fromLatin1_s(self ) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString10fromLatin1EPKci()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {C_ZN7QString10fromLatin1EPKci(arg0, arg1)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QString::contains(const QRegularExpression & re, QRegularExpressionMatch * match);
impl<'a> /*trait*/ QString_contains<i8> for (&'a QRegularExpression, &'a QRegularExpressionMatch) {
  fn contains(self , rsthis: & QString) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString8containsERK18QRegularExpressionP23QRegularExpressionMatch()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK7QString8containsERK18QRegularExpressionP23QRegularExpressionMatch(rsthis.qclsinst, arg0, arg1)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  int QString::indexOf(const QRegularExpression & re, int from, QRegularExpressionMatch * rmatch);
impl<'a> /*trait*/ QString_indexOf<i32> for (&'a QRegularExpression, i32, &'a QRegularExpressionMatch) {
  fn indexOf(self , rsthis: & QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString7indexOfERK18QRegularExpressioniP23QRegularExpressionMatch()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK7QString7indexOfERK18QRegularExpressioniP23QRegularExpressionMatch(rsthis.qclsinst, arg0, arg1, arg2)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  int QString::toWCharArray(wchar_t * array);
impl /*struct*/ QString {
  pub fn toWCharArray<RetType, T: QString_toWCharArray<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toWCharArray(self);
    // return 1;
  }
}

pub trait QString_toWCharArray<RetType> {
  fn toWCharArray(self , rsthis: & QString) -> RetType;
}

  // proto:  int QString::toWCharArray(wchar_t * array);
impl<'a> /*trait*/ QString_toWCharArray<i32> for (&'a  String) {
  fn toWCharArray(self , rsthis: & QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString12toWCharArrayEPw()};
    let arg0 = self.as_ptr()  as *mut wchar_t;
    let mut ret = unsafe {C_ZNK7QString12toWCharArrayEPw(rsthis.qclsinst, arg0)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  const_iterator QString::cbegin();
impl /*struct*/ QString {
  pub fn cbegin<RetType, T: QString_cbegin<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.cbegin(self);
    // return 1;
  }
}

pub trait QString_cbegin<RetType> {
  fn cbegin(self , rsthis: & QString) -> RetType;
}

  // proto:  const_iterator QString::cbegin();
impl<'a> /*trait*/ QString_cbegin<QChar> for () {
  fn cbegin(self , rsthis: & QString) -> QChar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString6cbeginEv()};
    let mut ret = unsafe {C_ZNK7QString6cbeginEv(rsthis.qclsinst)};
    let mut ret1 = QChar::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString & QString::prepend(const QByteArray & s);
impl<'a> /*trait*/ QString_prepend<QString> for (&'a QByteArray) {
  fn prepend(self , rsthis: & QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString7prependERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN7QString7prependERK10QByteArray(rsthis.qclsinst, arg0)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString & QString::replace(int i, int len, const QString & after);
impl<'a> /*trait*/ QString_replace<QString> for (i32, i32, &'a QString) {
  fn replace(self , rsthis: & QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString7replaceEiiRKS_()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN7QString7replaceEiiRKS_(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QString::arg(const QString & a1, const QString & a2, const QString & a3, const QString & a4, const QString & a5, const QString & a6, const QString & a7);
impl<'a> /*trait*/ QString_arg<QString> for (&'a QString, &'a QString, &'a QString, &'a QString, &'a QString, &'a QString, &'a QString) {
  fn arg(self , rsthis: & QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString3argERKS_S1_S1_S1_S1_S1_S1_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let arg3 = self.3.qclsinst  as *mut c_void;
    let arg4 = self.4.qclsinst  as *mut c_void;
    let arg5 = self.5.qclsinst  as *mut c_void;
    let arg6 = self.6.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK7QString3argERKS_S1_S1_S1_S1_S1_S1_(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4, arg5, arg6)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto: static QString QString::fromWCharArray(const wchar_t * string, int size);
impl /*struct*/ QString {
  pub fn fromWCharArray_s<RetType, T: QString_fromWCharArray_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromWCharArray_s();
    // return 1;
  }
}

pub trait QString_fromWCharArray_s<RetType> {
  fn fromWCharArray_s(self ) -> RetType;
}

  // proto: static QString QString::fromWCharArray(const wchar_t * string, int size);
impl<'a> /*trait*/ QString_fromWCharArray_s<QString> for (&'a  String, i32) {
  fn fromWCharArray_s(self ) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString14fromWCharArrayEPKwi()};
    let arg0 = self.0.as_ptr()  as *mut wchar_t;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {C_ZN7QString14fromWCharArrayEPKwi(arg0, arg1)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString & QString::fill(QChar c, int size);
impl /*struct*/ QString {
  pub fn fill<RetType, T: QString_fill<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.fill(self);
    // return 1;
  }
}

pub trait QString_fill<RetType> {
  fn fill(self , rsthis: & QString) -> RetType;
}

  // proto:  QString & QString::fill(QChar c, int size);
impl<'a> /*trait*/ QString_fill<QString> for (QChar, i32) {
  fn fill(self , rsthis: & QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString4fillE5QChari()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {C_ZN7QString4fillE5QChari(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  const QChar * QString::constData();
impl /*struct*/ QString {
  pub fn constData<RetType, T: QString_constData<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.constData(self);
    // return 1;
  }
}

pub trait QString_constData<RetType> {
  fn constData(self , rsthis: & QString) -> RetType;
}

  // proto:  const QChar * QString::constData();
impl<'a> /*trait*/ QString_constData<QChar> for () {
  fn constData(self , rsthis: & QString) -> QChar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString9constDataEv()};
    let mut ret = unsafe {C_ZNK7QString9constDataEv(rsthis.qclsinst)};
    let mut ret1 = QChar::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto: static QString QString::number(ulong , int base);
impl<'a> /*trait*/ QString_number_s<QString> for (u64, i32) {
  fn number_s(self ) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString6numberEmi()};
    let arg0 = self.0  as c_ulong;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {C_ZN7QString6numberEmi(arg0, arg1)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  long QString::toLong(bool * ok, int base);
impl /*struct*/ QString {
  pub fn toLong<RetType, T: QString_toLong<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toLong(self);
    // return 1;
  }
}

pub trait QString_toLong<RetType> {
  fn toLong(self , rsthis: & QString) -> RetType;
}

  // proto:  long QString::toLong(bool * ok, int base);
impl<'a> /*trait*/ QString_toLong<i64> for (&'a mut Vec<i8>, i32) {
  fn toLong(self , rsthis: & QString) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString6toLongEPbi()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {C_ZNK7QString6toLongEPbi(rsthis.qclsinst, arg0, arg1)};
    return ret as i64; // 1
    // return 1;
  }
}

  // proto:  QString QString::toUpper();
impl /*struct*/ QString {
  pub fn toUpper<RetType, T: QString_toUpper<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toUpper(self);
    // return 1;
  }
}

pub trait QString_toUpper<RetType> {
  fn toUpper(self , rsthis: & QString) -> RetType;
}

  // proto:  QString QString::toUpper();
impl<'a> /*trait*/ QString_toUpper<QString> for () {
  fn toUpper(self , rsthis: & QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNO7QString7toUpperEv()};
    let mut ret = unsafe {C_ZNO7QString7toUpperEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  const_iterator QString::constEnd();
impl /*struct*/ QString {
  pub fn constEnd<RetType, T: QString_constEnd<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.constEnd(self);
    // return 1;
  }
}

pub trait QString_constEnd<RetType> {
  fn constEnd(self , rsthis: & QString) -> RetType;
}

  // proto:  const_iterator QString::constEnd();
impl<'a> /*trait*/ QString_constEnd<QChar> for () {
  fn constEnd(self , rsthis: & QString) -> QChar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString8constEndEv()};
    let mut ret = unsafe {C_ZNK7QString8constEndEv(rsthis.qclsinst)};
    let mut ret1 = QChar::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QString::length();
impl /*struct*/ QString {
  pub fn length<RetType, T: QString_length<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.length(self);
    // return 1;
  }
}

pub trait QString_length<RetType> {
  fn length(self , rsthis: & QString) -> RetType;
}

  // proto:  int QString::length();
impl<'a> /*trait*/ QString_length<i32> for () {
  fn length(self , rsthis: & QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString6lengthEv()};
    let mut ret = unsafe {C_ZNK7QString6lengthEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto: static QString QString::fromUtf8(const char * str, int size);
impl<'a> /*trait*/ QString_fromUtf8_s<QString> for (&'a  String, i32) {
  fn fromUtf8_s(self ) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString8fromUtf8EPKci()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {C_ZN7QString8fromUtf8EPKci(arg0, arg1)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QString::simplified();
impl /*struct*/ QString {
  pub fn simplified<RetType, T: QString_simplified<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.simplified(self);
    // return 1;
  }
}

pub trait QString_simplified<RetType> {
  fn simplified(self , rsthis: & QString) -> RetType;
}

  // proto:  QString QString::simplified();
impl<'a> /*trait*/ QString_simplified<QString> for () {
  fn simplified(self , rsthis: & QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNO7QString10simplifiedEv()};
    let mut ret = unsafe {C_ZNO7QString10simplifiedEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto: static QString QString::number(qlonglong , int base);
impl<'a> /*trait*/ QString_number_s<QString> for (i64, i32) {
  fn number_s(self ) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString6numberExi()};
    let arg0 = self.0  as c_longlong;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {C_ZN7QString6numberExi(arg0, arg1)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QStringRef QString::leftRef(int n);
impl /*struct*/ QString {
  pub fn leftRef<RetType, T: QString_leftRef<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.leftRef(self);
    // return 1;
  }
}

pub trait QString_leftRef<RetType> {
  fn leftRef(self , rsthis: & QString) -> RetType;
}

  // proto:  QStringRef QString::leftRef(int n);
impl<'a> /*trait*/ QString_leftRef<QStringRef> for (i32) {
  fn leftRef(self , rsthis: & QString) -> QStringRef {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString7leftRefEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {C_ZNK7QString7leftRefEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QStringRef::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QString::arg(const QString & a1, const QString & a2);
impl<'a> /*trait*/ QString_arg<QString> for (&'a QString, &'a QString) {
  fn arg(self , rsthis: & QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString3argERKS_S1_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK7QString3argERKS_S1_(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QString::isSimpleText();
impl /*struct*/ QString {
  pub fn isSimpleText<RetType, T: QString_isSimpleText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isSimpleText(self);
    // return 1;
  }
}

pub trait QString_isSimpleText<RetType> {
  fn isSimpleText(self , rsthis: & QString) -> RetType;
}

  // proto:  bool QString::isSimpleText();
impl<'a> /*trait*/ QString_isSimpleText<i8> for () {
  fn isSimpleText(self , rsthis: & QString) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString12isSimpleTextEv()};
    let mut ret = unsafe {C_ZNK7QString12isSimpleTextEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto: static QString QString::fromUcs4(const uint * , int size);
impl<'a> /*trait*/ QString_fromUcs4_s<QString> for (&'a  Vec<u32>, i32) {
  fn fromUcs4_s(self ) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString8fromUcs4EPKji()};
    let arg0 = self.0.as_ptr()  as *mut c_uint;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {C_ZN7QString8fromUcs4EPKji(arg0, arg1)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString & QString::setUnicode(const QChar * unicode, int size);
impl /*struct*/ QString {
  pub fn setUnicode<RetType, T: QString_setUnicode<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setUnicode(self);
    // return 1;
  }
}

pub trait QString_setUnicode<RetType> {
  fn setUnicode(self , rsthis: & QString) -> RetType;
}

  // proto:  QString & QString::setUnicode(const QChar * unicode, int size);
impl<'a> /*trait*/ QString_setUnicode<QString> for (&'a QChar, i32) {
  fn setUnicode(self , rsthis: & QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString10setUnicodeEPK5QChari()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {C_ZN7QString10setUnicodeEPK5QChari(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  const_iterator QString::constBegin();
impl /*struct*/ QString {
  pub fn constBegin<RetType, T: QString_constBegin<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.constBegin(self);
    // return 1;
  }
}

pub trait QString_constBegin<RetType> {
  fn constBegin(self , rsthis: & QString) -> RetType;
}

  // proto:  const_iterator QString::constBegin();
impl<'a> /*trait*/ QString_constBegin<QChar> for () {
  fn constBegin(self , rsthis: & QString) -> QChar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString10constBeginEv()};
    let mut ret = unsafe {C_ZNK7QString10constBeginEv(rsthis.qclsinst)};
    let mut ret1 = QChar::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  const QChar * QString::unicode();
impl /*struct*/ QString {
  pub fn unicode<RetType, T: QString_unicode<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.unicode(self);
    // return 1;
  }
}

pub trait QString_unicode<RetType> {
  fn unicode(self , rsthis: & QString) -> RetType;
}

  // proto:  const QChar * QString::unicode();
impl<'a> /*trait*/ QString_unicode<QChar> for () {
  fn unicode(self , rsthis: & QString) -> QChar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString7unicodeEv()};
    let mut ret = unsafe {C_ZNK7QString7unicodeEv(rsthis.qclsinst)};
    let mut ret1 = QChar::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QString::arg(const QString & a1, const QString & a2, const QString & a3, const QString & a4, const QString & a5, const QString & a6, const QString & a7, const QString & a8, const QString & a9);
impl<'a> /*trait*/ QString_arg<QString> for (&'a QString, &'a QString, &'a QString, &'a QString, &'a QString, &'a QString, &'a QString, &'a QString, &'a QString) {
  fn arg(self , rsthis: & QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString3argERKS_S1_S1_S1_S1_S1_S1_S1_S1_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let arg3 = self.3.qclsinst  as *mut c_void;
    let arg4 = self.4.qclsinst  as *mut c_void;
    let arg5 = self.5.qclsinst  as *mut c_void;
    let arg6 = self.6.qclsinst  as *mut c_void;
    let arg7 = self.7.qclsinst  as *mut c_void;
    let arg8 = self.8.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK7QString3argERKS_S1_S1_S1_S1_S1_S1_S1_S1_(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QString::indexOf(const QRegularExpression & re, int from);
impl<'a> /*trait*/ QString_indexOf<i32> for (&'a QRegularExpression, i32) {
  fn indexOf(self , rsthis: & QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString7indexOfERK18QRegularExpressioni()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {C_ZNK7QString7indexOfERK18QRegularExpressioni(rsthis.qclsinst, arg0, arg1)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto: static QString QString::number(uint , int base);
impl<'a> /*trait*/ QString_number_s<QString> for (u32, i32) {
  fn number_s(self ) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString6numberEji()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {C_ZN7QString6numberEji(arg0, arg1)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto: static QString QString::fromLocal8Bit(const QByteArray & str);
impl<'a> /*trait*/ QString_fromLocal8Bit_s<QString> for (&'a QByteArray) {
  fn fromLocal8Bit_s(self ) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString13fromLocal8BitERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN7QString13fromLocal8BitERK10QByteArray(arg0)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  const QChar QString::at(int i);
impl /*struct*/ QString {
  pub fn at<RetType, T: QString_at<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.at(self);
    // return 1;
  }
}

pub trait QString_at<RetType> {
  fn at(self , rsthis: & QString) -> RetType;
}

  // proto:  const QChar QString::at(int i);
impl<'a> /*trait*/ QString_at<QChar> for (i32) {
  fn at(self , rsthis: & QString) -> QChar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString2atEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {C_ZNK7QString2atEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QChar::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto: static QString QString::asprintf(const char * format);
impl /*struct*/ QString {
  pub fn asprintf_s<RetType, T: QString_asprintf_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.asprintf_s();
    // return 1;
  }
}

pub trait QString_asprintf_s<RetType> {
  fn asprintf_s(self ) -> RetType;
}

  // proto: static QString QString::asprintf(const char * format);
impl<'a> /*trait*/ QString_asprintf_s<QString> for (&'a  String) {
  fn asprintf_s(self ) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString8asprintfEPKcz()};
    let arg0 = self.as_ptr()  as *mut c_char;
    let mut ret = unsafe {C_ZN7QString8asprintfEPKcz(arg0)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QString::QString(int size, QChar c);
impl<'a> /*trait*/ QString_new for (i32, QChar) {
  fn new(self) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QStringC2Ei5QChar()};
    let ctysz: c_int = unsafe{QString_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN7QStringC2Ei5QChar(arg0, arg1)};
    let rsthis = QString{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QByteArray QString::toLatin1();
impl /*struct*/ QString {
  pub fn toLatin1<RetType, T: QString_toLatin1<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toLatin1(self);
    // return 1;
  }
}

pub trait QString_toLatin1<RetType> {
  fn toLatin1(self , rsthis: & QString) -> RetType;
}

  // proto:  QByteArray QString::toLatin1();
impl<'a> /*trait*/ QString_toLatin1<QByteArray> for () {
  fn toLatin1(self , rsthis: & QString) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNO7QString8toLatin1Ev()};
    let mut ret = unsafe {C_ZNO7QString8toLatin1Ev(rsthis.qclsinst)};
    let mut ret1 = QByteArray::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QString::push_front(const QString & s);
impl<'a> /*trait*/ QString_push_front<()> for (&'a QString) {
  fn push_front(self , rsthis: & QString) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString10push_frontERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN7QString10push_frontERKS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QString QString::arg(const QString & a1, const QString & a2, const QString & a3, const QString & a4, const QString & a5, const QString & a6);
impl<'a> /*trait*/ QString_arg<QString> for (&'a QString, &'a QString, &'a QString, &'a QString, &'a QString, &'a QString) {
  fn arg(self , rsthis: & QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString3argERKS_S1_S1_S1_S1_S1_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let arg3 = self.3.qclsinst  as *mut c_void;
    let arg4 = self.4.qclsinst  as *mut c_void;
    let arg5 = self.5.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK7QString3argERKS_S1_S1_S1_S1_S1_(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4, arg5)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  iterator QString::begin();
impl /*struct*/ QString {
  pub fn begin<RetType, T: QString_begin<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.begin(self);
    // return 1;
  }
}

pub trait QString_begin<RetType> {
  fn begin(self , rsthis: & QString) -> RetType;
}

  // proto:  iterator QString::begin();
impl<'a> /*trait*/ QString_begin<QChar> for () {
  fn begin(self , rsthis: & QString) -> QChar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString5beginEv()};
    let mut ret = unsafe {C_ZN7QString5beginEv(rsthis.qclsinst)};
    let mut ret1 = QChar::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto: static QString QString::number(double , char f, int prec);
impl<'a> /*trait*/ QString_number_s<QString> for (f64, i8, i32) {
  fn number_s(self ) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString6numberEdci()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_char;
    let arg2 = self.2  as c_int;
    let mut ret = unsafe {C_ZN7QString6numberEdci(arg0, arg1, arg2)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  iterator QString::end();
impl /*struct*/ QString {
  pub fn end<RetType, T: QString_end<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.end(self);
    // return 1;
  }
}

pub trait QString_end<RetType> {
  fn end(self , rsthis: & QString) -> RetType;
}

  // proto:  iterator QString::end();
impl<'a> /*trait*/ QString_end<QChar> for () {
  fn end(self , rsthis: & QString) -> QChar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString3endEv()};
    let mut ret = unsafe {C_ZN7QString3endEv(rsthis.qclsinst)};
    let mut ret1 = QChar::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString & QString::append(QChar c);
impl<'a> /*trait*/ QString_append<QString> for (QChar) {
  fn append(self , rsthis: & QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString6appendE5QChar()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN7QString6appendE5QChar(rsthis.qclsinst, arg0)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  uint QString::toUInt(bool * ok, int base);
impl /*struct*/ QString {
  pub fn toUInt<RetType, T: QString_toUInt<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toUInt(self);
    // return 1;
  }
}

pub trait QString_toUInt<RetType> {
  fn toUInt(self , rsthis: & QString) -> RetType;
}

  // proto:  uint QString::toUInt(bool * ok, int base);
impl<'a> /*trait*/ QString_toUInt<u32> for (&'a mut Vec<i8>, i32) {
  fn toUInt(self , rsthis: & QString) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString6toUIntEPbi()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {C_ZNK7QString6toUIntEPbi(rsthis.qclsinst, arg0, arg1)};
    return ret as u32; // 1
    // return 1;
  }
}

  // proto:  QString & QString::append(const QString & s);
impl<'a> /*trait*/ QString_append<QString> for (&'a QString) {
  fn append(self , rsthis: & QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString6appendERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN7QString6appendERKS_(rsthis.qclsinst, arg0)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  ushort QString::toUShort(bool * ok, int base);
impl /*struct*/ QString {
  pub fn toUShort<RetType, T: QString_toUShort<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toUShort(self);
    // return 1;
  }
}

pub trait QString_toUShort<RetType> {
  fn toUShort(self , rsthis: & QString) -> RetType;
}

  // proto:  ushort QString::toUShort(bool * ok, int base);
impl<'a> /*trait*/ QString_toUShort<u16> for (&'a mut Vec<i8>, i32) {
  fn toUShort(self , rsthis: & QString) -> u16 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString8toUShortEPbi()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {C_ZNK7QString8toUShortEPbi(rsthis.qclsinst, arg0, arg1)};
    return ret as u16; // 1
    // return 1;
  }
}

  // proto:  QString QString::arg(uint a, int fieldWidth, int base, QChar fillChar);
impl<'a> /*trait*/ QString_arg<QString> for (u32, i32, i32, QChar) {
  fn arg(self , rsthis: & QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString3argEjii5QChar()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK7QString3argEjii5QChar(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString & QString::setNum(ushort , int base);
impl<'a> /*trait*/ QString_setNum<QString> for (u16, i32) {
  fn setNum(self , rsthis: & QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString6setNumEti()};
    let arg0 = self.0  as c_ushort;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {C_ZN7QString6setNumEti(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QByteArray QString::toLocal8Bit();
impl /*struct*/ QString {
  pub fn toLocal8Bit<RetType, T: QString_toLocal8Bit<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toLocal8Bit(self);
    // return 1;
  }
}

pub trait QString_toLocal8Bit<RetType> {
  fn toLocal8Bit(self , rsthis: & QString) -> RetType;
}

  // proto:  QByteArray QString::toLocal8Bit();
impl<'a> /*trait*/ QString_toLocal8Bit<QByteArray> for () {
  fn toLocal8Bit(self , rsthis: & QString) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNO7QString11toLocal8BitEv()};
    let mut ret = unsafe {C_ZNO7QString11toLocal8BitEv(rsthis.qclsinst)};
    let mut ret1 = QByteArray::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString & QString::replace(const QRegularExpression & re, const QString & after);
impl<'a> /*trait*/ QString_replace<QString> for (&'a QRegularExpression, &'a QString) {
  fn replace(self , rsthis: & QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString7replaceERK18QRegularExpressionRKS_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN7QString7replaceERK18QRegularExpressionRKS_(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString & QString::setNum(double , char f, int prec);
impl<'a> /*trait*/ QString_setNum<QString> for (f64, i8, i32) {
  fn setNum(self , rsthis: & QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString6setNumEdci()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_char;
    let arg2 = self.2  as c_int;
    let mut ret = unsafe {C_ZN7QString6setNumEdci(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QString::arg(ushort a, int fieldWidth, int base, QChar fillChar);
impl<'a> /*trait*/ QString_arg<QString> for (u16, i32, i32, QChar) {
  fn arg(self , rsthis: & QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString3argEtii5QChar()};
    let arg0 = self.0  as c_ushort;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK7QString3argEtii5QChar(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QString::QString(const QString & );
impl<'a> /*trait*/ QString_new for (&'a QString) {
  fn new(self) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QStringC2ERKS_()};
    let ctysz: c_int = unsafe{QString_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN7QStringC2ERKS_(arg0)};
    let rsthis = QString{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QString QString::arg(short a, int fieldWidth, int base, QChar fillChar);
impl<'a> /*trait*/ QString_arg<QString> for (i16, i32, i32, QChar) {
  fn arg(self , rsthis: & QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString3argEsii5QChar()};
    let arg0 = self.0  as c_short;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK7QString3argEsii5QChar(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QString::QString(const QByteArray & a);
impl<'a> /*trait*/ QString_new for (&'a QByteArray) {
  fn new(self) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QStringC2ERK10QByteArray()};
    let ctysz: c_int = unsafe{QString_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN7QStringC2ERK10QByteArray(arg0)};
    let rsthis = QString{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto: static QString QString::vasprintf(const char * format, va_list ap);
impl /*struct*/ QString {
  pub fn vasprintf_s<RetType, T: QString_vasprintf_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.vasprintf_s();
    // return 1;
  }
}

pub trait QString_vasprintf_s<RetType> {
  fn vasprintf_s(self ) -> RetType;
}

  // proto: static QString QString::vasprintf(const char * format, va_list ap);
impl<'a> /*trait*/ QString_vasprintf_s<QString> for (&'a  String, i32) {
  fn vasprintf_s(self ) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString9vasprintfEPKci()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {C_ZN7QString9vasprintfEPKci(arg0, arg1)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  qulonglong QString::toULongLong(bool * ok, int base);
impl /*struct*/ QString {
  pub fn toULongLong<RetType, T: QString_toULongLong<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toULongLong(self);
    // return 1;
  }
}

pub trait QString_toULongLong<RetType> {
  fn toULongLong(self , rsthis: & QString) -> RetType;
}

  // proto:  qulonglong QString::toULongLong(bool * ok, int base);
impl<'a> /*trait*/ QString_toULongLong<u64> for (&'a mut Vec<i8>, i32) {
  fn toULongLong(self , rsthis: & QString) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString11toULongLongEPbi()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {C_ZNK7QString11toULongLongEPbi(rsthis.qclsinst, arg0, arg1)};
    return ret as u64; // 1
    // return 1;
  }
}

  // proto:  QString & QString::append(const char * s);
impl<'a> /*trait*/ QString_append<QString> for (&'a  String) {
  fn append(self , rsthis: & QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString6appendEPKc()};
    let arg0 = self.as_ptr()  as *mut c_char;
    let mut ret = unsafe {C_ZN7QString6appendEPKc(rsthis.qclsinst, arg0)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QString::capacity();
impl /*struct*/ QString {
  pub fn capacity<RetType, T: QString_capacity<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.capacity(self);
    // return 1;
  }
}

pub trait QString_capacity<RetType> {
  fn capacity(self , rsthis: & QString) -> RetType;
}

  // proto:  int QString::capacity();
impl<'a> /*trait*/ QString_capacity<i32> for () {
  fn capacity(self , rsthis: & QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString8capacityEv()};
    let mut ret = unsafe {C_ZNK7QString8capacityEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  void QString::squeeze();
impl /*struct*/ QString {
  pub fn squeeze<RetType, T: QString_squeeze<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.squeeze(self);
    // return 1;
  }
}

pub trait QString_squeeze<RetType> {
  fn squeeze(self , rsthis: & QString) -> RetType;
}

  // proto:  void QString::squeeze();
impl<'a> /*trait*/ QString_squeeze<()> for () {
  fn squeeze(self , rsthis: & QString) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString7squeezeEv()};
     unsafe {C_ZN7QString7squeezeEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QString::truncate(int pos);
impl /*struct*/ QString {
  pub fn truncate<RetType, T: QString_truncate<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.truncate(self);
    // return 1;
  }
}

pub trait QString_truncate<RetType> {
  fn truncate(self , rsthis: & QString) -> RetType;
}

  // proto:  void QString::truncate(int pos);
impl<'a> /*trait*/ QString_truncate<()> for (i32) {
  fn truncate(self , rsthis: & QString) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString8truncateEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN7QString8truncateEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QString QString::arg(int a, int fieldWidth, int base, QChar fillChar);
impl<'a> /*trait*/ QString_arg<QString> for (i32, i32, i32, QChar) {
  fn arg(self , rsthis: & QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString3argEiii5QChar()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK7QString3argEiii5QChar(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QString::arg(QChar a, int fieldWidth, QChar fillChar);
impl<'a> /*trait*/ QString_arg<QString> for (QChar, i32, QChar) {
  fn arg(self , rsthis: & QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString3argE5QChariS0_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK7QString3argE5QChariS0_(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QString::localeAwareCompare(const QString & s);
impl /*struct*/ QString {
  pub fn localeAwareCompare<RetType, T: QString_localeAwareCompare<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.localeAwareCompare(self);
    // return 1;
  }
}

pub trait QString_localeAwareCompare<RetType> {
  fn localeAwareCompare(self , rsthis: & QString) -> RetType;
}

  // proto:  int QString::localeAwareCompare(const QString & s);
impl<'a> /*trait*/ QString_localeAwareCompare<i32> for (&'a QString) {
  fn localeAwareCompare(self , rsthis: & QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString18localeAwareCompareERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK7QString18localeAwareCompareERKS_(rsthis.qclsinst, arg0)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  QString & QString::remove(const QRegExp & rx);
impl<'a> /*trait*/ QString_remove<QString> for (&'a QRegExp) {
  fn remove(self , rsthis: & QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString6removeERK7QRegExp()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN7QString6removeERK7QRegExp(rsthis.qclsinst, arg0)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QString::contains(const QRegularExpression & re);
impl<'a> /*trait*/ QString_contains<i8> for (&'a QRegularExpression) {
  fn contains(self , rsthis: & QString) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString8containsERK18QRegularExpression()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK7QString8containsERK18QRegularExpression(rsthis.qclsinst, arg0)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  QString & QString::replace(int i, int len, QChar after);
impl<'a> /*trait*/ QString_replace<QString> for (i32, i32, QChar) {
  fn replace(self , rsthis: & QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString7replaceEii5QChar()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN7QString7replaceEii5QChar(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QString::isRightToLeft();
impl /*struct*/ QString {
  pub fn isRightToLeft<RetType, T: QString_isRightToLeft<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isRightToLeft(self);
    // return 1;
  }
}

pub trait QString_isRightToLeft<RetType> {
  fn isRightToLeft(self , rsthis: & QString) -> RetType;
}

  // proto:  bool QString::isRightToLeft();
impl<'a> /*trait*/ QString_isRightToLeft<i8> for () {
  fn isRightToLeft(self , rsthis: & QString) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString13isRightToLeftEv()};
    let mut ret = unsafe {C_ZNK7QString13isRightToLeftEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  QString QString::arg(char a, int fieldWidth, QChar fillChar);
impl<'a> /*trait*/ QString_arg<QString> for (i8, i32, QChar) {
  fn arg(self , rsthis: & QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString3argEci5QChar()};
    let arg0 = self.0  as c_char;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK7QString3argEci5QChar(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QVector<uint> QString::toUcs4();
impl /*struct*/ QString {
  pub fn toUcs4<RetType, T: QString_toUcs4<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toUcs4(self);
    // return 1;
  }
}

pub trait QString_toUcs4<RetType> {
  fn toUcs4(self , rsthis: & QString) -> RetType;
}

  // proto:  QVector<uint> QString::toUcs4();
impl<'a> /*trait*/ QString_toUcs4<u64> for () {
  fn toUcs4(self , rsthis: & QString) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString6toUcs4Ev()};
    let mut ret = unsafe {C_ZNK7QString6toUcs4Ev(rsthis.qclsinst)};
    return ret as u64; // 5
    // return 1;
  }
}

  // proto:  QString & QString::remove(int i, int len);
impl<'a> /*trait*/ QString_remove<QString> for (i32, i32) {
  fn remove(self , rsthis: & QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString6removeEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {C_ZN7QString6removeEii(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QString::isEmpty();
impl /*struct*/ QString {
  pub fn isEmpty<RetType, T: QString_isEmpty<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isEmpty(self);
    // return 1;
  }
}

pub trait QString_isEmpty<RetType> {
  fn isEmpty(self , rsthis: & QString) -> RetType;
}

  // proto:  bool QString::isEmpty();
impl<'a> /*trait*/ QString_isEmpty<i8> for () {
  fn isEmpty(self , rsthis: & QString) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString7isEmptyEv()};
    let mut ret = unsafe {C_ZNK7QString7isEmptyEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  QString QString::right(int n);
impl /*struct*/ QString {
  pub fn right<RetType, T: QString_right<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.right(self);
    // return 1;
  }
}

pub trait QString_right<RetType> {
  fn right(self , rsthis: & QString) -> RetType;
}

  // proto:  QString QString::right(int n);
impl<'a> /*trait*/ QString_right<QString> for (i32) {
  fn right(self , rsthis: & QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString5rightEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {C_ZNK7QString5rightEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QString::rightJustified(int width, QChar fill, bool trunc);
impl /*struct*/ QString {
  pub fn rightJustified<RetType, T: QString_rightJustified<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rightJustified(self);
    // return 1;
  }
}

pub trait QString_rightJustified<RetType> {
  fn rightJustified(self , rsthis: & QString) -> RetType;
}

  // proto:  QString QString::rightJustified(int width, QChar fill, bool trunc);
impl<'a> /*trait*/ QString_rightJustified<QString> for (i32, QChar, i8) {
  fn rightJustified(self , rsthis: & QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString14rightJustifiedEi5QCharb()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_char;
    let mut ret = unsafe {C_ZNK7QString14rightJustifiedEi5QCharb(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QString::arg(const QString & a, int fieldWidth, QChar fillChar);
impl<'a> /*trait*/ QString_arg<QString> for (&'a QString, i32, QChar) {
  fn arg(self , rsthis: & QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString3argERKS_i5QChar()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK7QString3argERKS_i5QChar(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QString::arg(qulonglong a, int fieldwidth, int base, QChar fillChar);
impl<'a> /*trait*/ QString_arg<QString> for (u64, i32, i32, QChar) {
  fn arg(self , rsthis: & QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString3argEyii5QChar()};
    let arg0 = self.0  as c_ulonglong;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK7QString3argEyii5QChar(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QString::reserve(int size);
impl /*struct*/ QString {
  pub fn reserve<RetType, T: QString_reserve<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.reserve(self);
    // return 1;
  }
}

pub trait QString_reserve<RetType> {
  fn reserve(self , rsthis: & QString) -> RetType;
}

  // proto:  void QString::reserve(int size);
impl<'a> /*trait*/ QString_reserve<()> for (i32) {
  fn reserve(self , rsthis: & QString) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString7reserveEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN7QString7reserveEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  short QString::toShort(bool * ok, int base);
impl /*struct*/ QString {
  pub fn toShort<RetType, T: QString_toShort<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toShort(self);
    // return 1;
  }
}

pub trait QString_toShort<RetType> {
  fn toShort(self , rsthis: & QString) -> RetType;
}

  // proto:  short QString::toShort(bool * ok, int base);
impl<'a> /*trait*/ QString_toShort<i16> for (&'a mut Vec<i8>, i32) {
  fn toShort(self , rsthis: & QString) -> i16 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString7toShortEPbi()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {C_ZNK7QString7toShortEPbi(rsthis.qclsinst, arg0, arg1)};
    return ret as i16; // 1
    // return 1;
  }
}

impl /*struct*/ QLatin1String {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QLatin1String {
    return QLatin1String{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  const char * QLatin1String::data();
impl /*struct*/ QLatin1String {
  pub fn data<RetType, T: QLatin1String_data<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.data(self);
    // return 1;
  }
}

pub trait QLatin1String_data<RetType> {
  fn data(self , rsthis: & QLatin1String) -> RetType;
}

  // proto:  const char * QLatin1String::data();
impl<'a> /*trait*/ QLatin1String_data<String> for () {
  fn data(self , rsthis: & QLatin1String) -> String {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QLatin1String4dataEv()};
    let mut ret = unsafe {C_ZNK13QLatin1String4dataEv(rsthis.qclsinst)};
    let slen = unsafe {strlen(ret as *const i8)} as usize;
    return unsafe{String::from_raw_parts(ret as *mut u8, slen, slen+1)};
    // return 1;
  }
}

  // proto:  void QLatin1String::QLatin1String(const char * s);
impl /*struct*/ QLatin1String {
  pub fn new<T: QLatin1String_new>(value: T) -> QLatin1String {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QLatin1String_new {
  fn new(self) -> QLatin1String;
}

  // proto:  void QLatin1String::QLatin1String(const char * s);
impl<'a> /*trait*/ QLatin1String_new for (&'a  String) {
  fn new(self) -> QLatin1String {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QLatin1StringC2EPKc()};
    let ctysz: c_int = unsafe{QLatin1String_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.as_ptr()  as *mut c_char;
    let qthis: u64 = unsafe {C_ZN13QLatin1StringC2EPKc(arg0)};
    let rsthis = QLatin1String{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  int QLatin1String::size();
impl /*struct*/ QLatin1String {
  pub fn size<RetType, T: QLatin1String_size<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.size(self);
    // return 1;
  }
}

pub trait QLatin1String_size<RetType> {
  fn size(self , rsthis: & QLatin1String) -> RetType;
}

  // proto:  int QLatin1String::size();
impl<'a> /*trait*/ QLatin1String_size<i32> for () {
  fn size(self , rsthis: & QLatin1String) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QLatin1String4sizeEv()};
    let mut ret = unsafe {C_ZNK13QLatin1String4sizeEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  void QLatin1String::QLatin1String(const QByteArray & s);
impl<'a> /*trait*/ QLatin1String_new for (&'a QByteArray) {
  fn new(self) -> QLatin1String {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QLatin1StringC2ERK10QByteArray()};
    let ctysz: c_int = unsafe{QLatin1String_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN13QLatin1StringC2ERK10QByteArray(arg0)};
    let rsthis = QLatin1String{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  const char * QLatin1String::latin1();
impl /*struct*/ QLatin1String {
  pub fn latin1<RetType, T: QLatin1String_latin1<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.latin1(self);
    // return 1;
  }
}

pub trait QLatin1String_latin1<RetType> {
  fn latin1(self , rsthis: & QLatin1String) -> RetType;
}

  // proto:  const char * QLatin1String::latin1();
impl<'a> /*trait*/ QLatin1String_latin1<String> for () {
  fn latin1(self , rsthis: & QLatin1String) -> String {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QLatin1String6latin1Ev()};
    let mut ret = unsafe {C_ZNK13QLatin1String6latin1Ev(rsthis.qclsinst)};
    let slen = unsafe {strlen(ret as *const i8)} as usize;
    return unsafe{String::from_raw_parts(ret as *mut u8, slen, slen+1)};
    // return 1;
  }
}

  // proto:  void QLatin1String::QLatin1String(const char * s, int sz);
impl<'a> /*trait*/ QLatin1String_new for (&'a  String, i32) {
  fn new(self) -> QLatin1String {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QLatin1StringC2EPKci()};
    let ctysz: c_int = unsafe{QLatin1String_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1  as c_int;
    let qthis: u64 = unsafe {C_ZN13QLatin1StringC2EPKci(arg0, arg1)};
    let rsthis = QLatin1String{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QCharRef {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QCharRef {
    return QCharRef{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  bool QCharRef::isLetterOrNumber();
impl /*struct*/ QCharRef {
  pub fn isLetterOrNumber<RetType, T: QCharRef_isLetterOrNumber<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isLetterOrNumber(self);
    // return 1;
  }
}

pub trait QCharRef_isLetterOrNumber<RetType> {
  fn isLetterOrNumber(self , rsthis: & QCharRef) -> RetType;
}

  // proto:  bool QCharRef::isLetterOrNumber();
impl<'a> /*trait*/ QCharRef_isLetterOrNumber<i8> for () {
  fn isLetterOrNumber(self , rsthis: & QCharRef) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QCharRef16isLetterOrNumberEv()};
    let mut ret = unsafe {C_ZN8QCharRef16isLetterOrNumberEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  bool QCharRef::isDigit();
impl /*struct*/ QCharRef {
  pub fn isDigit<RetType, T: QCharRef_isDigit<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isDigit(self);
    // return 1;
  }
}

pub trait QCharRef_isDigit<RetType> {
  fn isDigit(self , rsthis: & QCharRef) -> RetType;
}

  // proto:  bool QCharRef::isDigit();
impl<'a> /*trait*/ QCharRef_isDigit<i8> for () {
  fn isDigit(self , rsthis: & QCharRef) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QCharRef7isDigitEv()};
    let mut ret = unsafe {C_ZNK8QCharRef7isDigitEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  char QCharRef::toLatin1();
impl /*struct*/ QCharRef {
  pub fn toLatin1<RetType, T: QCharRef_toLatin1<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toLatin1(self);
    // return 1;
  }
}

pub trait QCharRef_toLatin1<RetType> {
  fn toLatin1(self , rsthis: & QCharRef) -> RetType;
}

  // proto:  char QCharRef::toLatin1();
impl<'a> /*trait*/ QCharRef_toLatin1<i8> for () {
  fn toLatin1(self , rsthis: & QCharRef) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QCharRef8toLatin1Ev()};
    let mut ret = unsafe {C_ZNK8QCharRef8toLatin1Ev(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QCharRef::setCell(uchar cell);
impl /*struct*/ QCharRef {
  pub fn setCell<RetType, T: QCharRef_setCell<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setCell(self);
    // return 1;
  }
}

pub trait QCharRef_setCell<RetType> {
  fn setCell(self , rsthis: & QCharRef) -> RetType;
}

  // proto:  void QCharRef::setCell(uchar cell);
impl<'a> /*trait*/ QCharRef_setCell<()> for (u8) {
  fn setCell(self , rsthis: & QCharRef) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QCharRef7setCellEh()};
    let arg0 = self  as c_uchar;
     unsafe {C_ZN8QCharRef7setCellEh(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QCharRef::isMark();
impl /*struct*/ QCharRef {
  pub fn isMark<RetType, T: QCharRef_isMark<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isMark(self);
    // return 1;
  }
}

pub trait QCharRef_isMark<RetType> {
  fn isMark(self , rsthis: & QCharRef) -> RetType;
}

  // proto:  bool QCharRef::isMark();
impl<'a> /*trait*/ QCharRef_isMark<i8> for () {
  fn isMark(self , rsthis: & QCharRef) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QCharRef6isMarkEv()};
    let mut ret = unsafe {C_ZNK8QCharRef6isMarkEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  int QCharRef::digitValue();
impl /*struct*/ QCharRef {
  pub fn digitValue<RetType, T: QCharRef_digitValue<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.digitValue(self);
    // return 1;
  }
}

pub trait QCharRef_digitValue<RetType> {
  fn digitValue(self , rsthis: & QCharRef) -> RetType;
}

  // proto:  int QCharRef::digitValue();
impl<'a> /*trait*/ QCharRef_digitValue<i32> for () {
  fn digitValue(self , rsthis: & QCharRef) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QCharRef10digitValueEv()};
    let mut ret = unsafe {C_ZNK8QCharRef10digitValueEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  bool QCharRef::isLetter();
impl /*struct*/ QCharRef {
  pub fn isLetter<RetType, T: QCharRef_isLetter<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isLetter(self);
    // return 1;
  }
}

pub trait QCharRef_isLetter<RetType> {
  fn isLetter(self , rsthis: & QCharRef) -> RetType;
}

  // proto:  bool QCharRef::isLetter();
impl<'a> /*trait*/ QCharRef_isLetter<i8> for () {
  fn isLetter(self , rsthis: & QCharRef) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QCharRef8isLetterEv()};
    let mut ret = unsafe {C_ZNK8QCharRef8isLetterEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  bool QCharRef::isNumber();
impl /*struct*/ QCharRef {
  pub fn isNumber<RetType, T: QCharRef_isNumber<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isNumber(self);
    // return 1;
  }
}

pub trait QCharRef_isNumber<RetType> {
  fn isNumber(self , rsthis: & QCharRef) -> RetType;
}

  // proto:  bool QCharRef::isNumber();
impl<'a> /*trait*/ QCharRef_isNumber<i8> for () {
  fn isNumber(self , rsthis: & QCharRef) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QCharRef8isNumberEv()};
    let mut ret = unsafe {C_ZNK8QCharRef8isNumberEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  bool QCharRef::isPrint();
impl /*struct*/ QCharRef {
  pub fn isPrint<RetType, T: QCharRef_isPrint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isPrint(self);
    // return 1;
  }
}

pub trait QCharRef_isPrint<RetType> {
  fn isPrint(self , rsthis: & QCharRef) -> RetType;
}

  // proto:  bool QCharRef::isPrint();
impl<'a> /*trait*/ QCharRef_isPrint<i8> for () {
  fn isPrint(self , rsthis: & QCharRef) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QCharRef7isPrintEv()};
    let mut ret = unsafe {C_ZNK8QCharRef7isPrintEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  QChar QCharRef::toLower();
impl /*struct*/ QCharRef {
  pub fn toLower<RetType, T: QCharRef_toLower<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toLower(self);
    // return 1;
  }
}

pub trait QCharRef_toLower<RetType> {
  fn toLower(self , rsthis: & QCharRef) -> RetType;
}

  // proto:  QChar QCharRef::toLower();
impl<'a> /*trait*/ QCharRef_toLower<QChar> for () {
  fn toLower(self , rsthis: & QCharRef) -> QChar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QCharRef7toLowerEv()};
    let mut ret = unsafe {C_ZNK8QCharRef7toLowerEv(rsthis.qclsinst)};
    let mut ret1 = QChar::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QCharRef::setRow(uchar row);
impl /*struct*/ QCharRef {
  pub fn setRow<RetType, T: QCharRef_setRow<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setRow(self);
    // return 1;
  }
}

pub trait QCharRef_setRow<RetType> {
  fn setRow(self , rsthis: & QCharRef) -> RetType;
}

  // proto:  void QCharRef::setRow(uchar row);
impl<'a> /*trait*/ QCharRef_setRow<()> for (u8) {
  fn setRow(self , rsthis: & QCharRef) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QCharRef6setRowEh()};
    let arg0 = self  as c_uchar;
     unsafe {C_ZN8QCharRef6setRowEh(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QCharRef::isNull();
impl /*struct*/ QCharRef {
  pub fn isNull<RetType, T: QCharRef_isNull<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isNull(self);
    // return 1;
  }
}

pub trait QCharRef_isNull<RetType> {
  fn isNull(self , rsthis: & QCharRef) -> RetType;
}

  // proto:  bool QCharRef::isNull();
impl<'a> /*trait*/ QCharRef_isNull<i8> for () {
  fn isNull(self , rsthis: & QCharRef) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QCharRef6isNullEv()};
    let mut ret = unsafe {C_ZNK8QCharRef6isNullEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  QChar QCharRef::toTitleCase();
impl /*struct*/ QCharRef {
  pub fn toTitleCase<RetType, T: QCharRef_toTitleCase<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toTitleCase(self);
    // return 1;
  }
}

pub trait QCharRef_toTitleCase<RetType> {
  fn toTitleCase(self , rsthis: & QCharRef) -> RetType;
}

  // proto:  QChar QCharRef::toTitleCase();
impl<'a> /*trait*/ QCharRef_toTitleCase<QChar> for () {
  fn toTitleCase(self , rsthis: & QCharRef) -> QChar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QCharRef11toTitleCaseEv()};
    let mut ret = unsafe {C_ZNK8QCharRef11toTitleCaseEv(rsthis.qclsinst)};
    let mut ret1 = QChar::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QCharRef::hasMirrored();
impl /*struct*/ QCharRef {
  pub fn hasMirrored<RetType, T: QCharRef_hasMirrored<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hasMirrored(self);
    // return 1;
  }
}

pub trait QCharRef_hasMirrored<RetType> {
  fn hasMirrored(self , rsthis: & QCharRef) -> RetType;
}

  // proto:  bool QCharRef::hasMirrored();
impl<'a> /*trait*/ QCharRef_hasMirrored<i8> for () {
  fn hasMirrored(self , rsthis: & QCharRef) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QCharRef11hasMirroredEv()};
    let mut ret = unsafe {C_ZNK8QCharRef11hasMirroredEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  uchar QCharRef::row();
impl /*struct*/ QCharRef {
  pub fn row<RetType, T: QCharRef_row<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.row(self);
    // return 1;
  }
}

pub trait QCharRef_row<RetType> {
  fn row(self , rsthis: & QCharRef) -> RetType;
}

  // proto:  uchar QCharRef::row();
impl<'a> /*trait*/ QCharRef_row<u8> for () {
  fn row(self , rsthis: & QCharRef) -> u8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QCharRef3rowEv()};
    let mut ret = unsafe {C_ZNK8QCharRef3rowEv(rsthis.qclsinst)};
    return ret as u8; // 1
    // return 1;
  }
}

  // proto:  ushort & QCharRef::unicode();
impl /*struct*/ QCharRef {
  pub fn unicode<RetType, T: QCharRef_unicode<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.unicode(self);
    // return 1;
  }
}

pub trait QCharRef_unicode<RetType> {
  fn unicode(self , rsthis: & QCharRef) -> RetType;
}

  // proto:  ushort & QCharRef::unicode();
impl<'a> /*trait*/ QCharRef_unicode<u16> for () {
  fn unicode(self , rsthis: & QCharRef) -> u16 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QCharRef7unicodeEv()};
    let mut ret = unsafe {C_ZN8QCharRef7unicodeEv(rsthis.qclsinst)};
    return ret as u16; // 1
    // return 1;
  }
}

  // proto:  bool QCharRef::isTitleCase();
impl /*struct*/ QCharRef {
  pub fn isTitleCase<RetType, T: QCharRef_isTitleCase<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isTitleCase(self);
    // return 1;
  }
}

pub trait QCharRef_isTitleCase<RetType> {
  fn isTitleCase(self , rsthis: & QCharRef) -> RetType;
}

  // proto:  bool QCharRef::isTitleCase();
impl<'a> /*trait*/ QCharRef_isTitleCase<i8> for () {
  fn isTitleCase(self , rsthis: & QCharRef) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QCharRef11isTitleCaseEv()};
    let mut ret = unsafe {C_ZNK8QCharRef11isTitleCaseEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  bool QCharRef::isUpper();
impl /*struct*/ QCharRef {
  pub fn isUpper<RetType, T: QCharRef_isUpper<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isUpper(self);
    // return 1;
  }
}

pub trait QCharRef_isUpper<RetType> {
  fn isUpper(self , rsthis: & QCharRef) -> RetType;
}

  // proto:  bool QCharRef::isUpper();
impl<'a> /*trait*/ QCharRef_isUpper<i8> for () {
  fn isUpper(self , rsthis: & QCharRef) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QCharRef7isUpperEv()};
    let mut ret = unsafe {C_ZNK8QCharRef7isUpperEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  uchar QCharRef::cell();
impl /*struct*/ QCharRef {
  pub fn cell<RetType, T: QCharRef_cell<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.cell(self);
    // return 1;
  }
}

pub trait QCharRef_cell<RetType> {
  fn cell(self , rsthis: & QCharRef) -> RetType;
}

  // proto:  uchar QCharRef::cell();
impl<'a> /*trait*/ QCharRef_cell<u8> for () {
  fn cell(self , rsthis: & QCharRef) -> u8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QCharRef4cellEv()};
    let mut ret = unsafe {C_ZNK8QCharRef4cellEv(rsthis.qclsinst)};
    return ret as u8; // 1
    // return 1;
  }
}

  // proto:  QString QCharRef::decomposition();
impl /*struct*/ QCharRef {
  pub fn decomposition<RetType, T: QCharRef_decomposition<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.decomposition(self);
    // return 1;
  }
}

pub trait QCharRef_decomposition<RetType> {
  fn decomposition(self , rsthis: & QCharRef) -> RetType;
}

  // proto:  QString QCharRef::decomposition();
impl<'a> /*trait*/ QCharRef_decomposition<QString> for () {
  fn decomposition(self , rsthis: & QCharRef) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QCharRef13decompositionEv()};
    let mut ret = unsafe {C_ZNK8QCharRef13decompositionEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  uchar QCharRef::combiningClass();
impl /*struct*/ QCharRef {
  pub fn combiningClass<RetType, T: QCharRef_combiningClass<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.combiningClass(self);
    // return 1;
  }
}

pub trait QCharRef_combiningClass<RetType> {
  fn combiningClass(self , rsthis: & QCharRef) -> RetType;
}

  // proto:  uchar QCharRef::combiningClass();
impl<'a> /*trait*/ QCharRef_combiningClass<u8> for () {
  fn combiningClass(self , rsthis: & QCharRef) -> u8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QCharRef14combiningClassEv()};
    let mut ret = unsafe {C_ZNK8QCharRef14combiningClassEv(rsthis.qclsinst)};
    return ret as u8; // 1
    // return 1;
  }
}

  // proto:  QChar QCharRef::mirroredChar();
impl /*struct*/ QCharRef {
  pub fn mirroredChar<RetType, T: QCharRef_mirroredChar<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.mirroredChar(self);
    // return 1;
  }
}

pub trait QCharRef_mirroredChar<RetType> {
  fn mirroredChar(self , rsthis: & QCharRef) -> RetType;
}

  // proto:  QChar QCharRef::mirroredChar();
impl<'a> /*trait*/ QCharRef_mirroredChar<QChar> for () {
  fn mirroredChar(self , rsthis: & QCharRef) -> QChar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QCharRef12mirroredCharEv()};
    let mut ret = unsafe {C_ZNK8QCharRef12mirroredCharEv(rsthis.qclsinst)};
    let mut ret1 = QChar::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QCharRef::isSpace();
impl /*struct*/ QCharRef {
  pub fn isSpace<RetType, T: QCharRef_isSpace<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isSpace(self);
    // return 1;
  }
}

pub trait QCharRef_isSpace<RetType> {
  fn isSpace(self , rsthis: & QCharRef) -> RetType;
}

  // proto:  bool QCharRef::isSpace();
impl<'a> /*trait*/ QCharRef_isSpace<i8> for () {
  fn isSpace(self , rsthis: & QCharRef) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QCharRef7isSpaceEv()};
    let mut ret = unsafe {C_ZNK8QCharRef7isSpaceEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  bool QCharRef::isPunct();
impl /*struct*/ QCharRef {
  pub fn isPunct<RetType, T: QCharRef_isPunct<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isPunct(self);
    // return 1;
  }
}

pub trait QCharRef_isPunct<RetType> {
  fn isPunct(self , rsthis: & QCharRef) -> RetType;
}

  // proto:  bool QCharRef::isPunct();
impl<'a> /*trait*/ QCharRef_isPunct<i8> for () {
  fn isPunct(self , rsthis: & QCharRef) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QCharRef7isPunctEv()};
    let mut ret = unsafe {C_ZNK8QCharRef7isPunctEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  QChar QCharRef::toUpper();
impl /*struct*/ QCharRef {
  pub fn toUpper<RetType, T: QCharRef_toUpper<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toUpper(self);
    // return 1;
  }
}

pub trait QCharRef_toUpper<RetType> {
  fn toUpper(self , rsthis: & QCharRef) -> RetType;
}

  // proto:  QChar QCharRef::toUpper();
impl<'a> /*trait*/ QCharRef_toUpper<QChar> for () {
  fn toUpper(self , rsthis: & QCharRef) -> QChar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QCharRef7toUpperEv()};
    let mut ret = unsafe {C_ZNK8QCharRef7toUpperEv(rsthis.qclsinst)};
    let mut ret1 = QChar::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QCharRef::isLower();
impl /*struct*/ QCharRef {
  pub fn isLower<RetType, T: QCharRef_isLower<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isLower(self);
    // return 1;
  }
}

pub trait QCharRef_isLower<RetType> {
  fn isLower(self , rsthis: & QCharRef) -> RetType;
}

  // proto:  bool QCharRef::isLower();
impl<'a> /*trait*/ QCharRef_isLower<i8> for () {
  fn isLower(self , rsthis: & QCharRef) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QCharRef7isLowerEv()};
    let mut ret = unsafe {C_ZNK8QCharRef7isLowerEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

impl /*struct*/ QStringRef {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QStringRef {
    return QStringRef{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  short QStringRef::toShort(bool * ok, int base);
impl /*struct*/ QStringRef {
  pub fn toShort<RetType, T: QStringRef_toShort<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toShort(self);
    // return 1;
  }
}

pub trait QStringRef_toShort<RetType> {
  fn toShort(self , rsthis: & QStringRef) -> RetType;
}

  // proto:  short QStringRef::toShort(bool * ok, int base);
impl<'a> /*trait*/ QStringRef_toShort<i16> for (&'a mut Vec<i8>, i32) {
  fn toShort(self , rsthis: & QStringRef) -> i16 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QStringRef7toShortEPbi()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {C_ZNK10QStringRef7toShortEPbi(rsthis.qclsinst, arg0, arg1)};
    return ret as i16; // 1
    // return 1;
  }
}

  // proto:  void QStringRef::QStringRef(const QString * string);
impl /*struct*/ QStringRef {
  pub fn new<T: QStringRef_new>(value: T) -> QStringRef {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QStringRef_new {
  fn new(self) -> QStringRef;
}

  // proto:  void QStringRef::QStringRef(const QString * string);
impl<'a> /*trait*/ QStringRef_new for (&'a QString) {
  fn new(self) -> QStringRef {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QStringRefC2EPK7QString()};
    let ctysz: c_int = unsafe{QStringRef_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN10QStringRefC2EPK7QString(arg0)};
    let rsthis = QStringRef{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  qulonglong QStringRef::toULongLong(bool * ok, int base);
impl /*struct*/ QStringRef {
  pub fn toULongLong<RetType, T: QStringRef_toULongLong<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toULongLong(self);
    // return 1;
  }
}

pub trait QStringRef_toULongLong<RetType> {
  fn toULongLong(self , rsthis: & QStringRef) -> RetType;
}

  // proto:  qulonglong QStringRef::toULongLong(bool * ok, int base);
impl<'a> /*trait*/ QStringRef_toULongLong<u64> for (&'a mut Vec<i8>, i32) {
  fn toULongLong(self , rsthis: & QStringRef) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QStringRef11toULongLongEPbi()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {C_ZNK10QStringRef11toULongLongEPbi(rsthis.qclsinst, arg0, arg1)};
    return ret as u64; // 1
    // return 1;
  }
}

  // proto:  void QStringRef::clear();
impl /*struct*/ QStringRef {
  pub fn clear<RetType, T: QStringRef_clear<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clear(self);
    // return 1;
  }
}

pub trait QStringRef_clear<RetType> {
  fn clear(self , rsthis: & QStringRef) -> RetType;
}

  // proto:  void QStringRef::clear();
impl<'a> /*trait*/ QStringRef_clear<()> for () {
  fn clear(self , rsthis: & QStringRef) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QStringRef5clearEv()};
     unsafe {C_ZN10QStringRef5clearEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QStringRef::position();
impl /*struct*/ QStringRef {
  pub fn position<RetType, T: QStringRef_position<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.position(self);
    // return 1;
  }
}

pub trait QStringRef_position<RetType> {
  fn position(self , rsthis: & QStringRef) -> RetType;
}

  // proto:  int QStringRef::position();
impl<'a> /*trait*/ QStringRef_position<i32> for () {
  fn position(self , rsthis: & QStringRef) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QStringRef8positionEv()};
    let mut ret = unsafe {C_ZNK10QStringRef8positionEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  long QStringRef::toLong(bool * ok, int base);
impl /*struct*/ QStringRef {
  pub fn toLong<RetType, T: QStringRef_toLong<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toLong(self);
    // return 1;
  }
}

pub trait QStringRef_toLong<RetType> {
  fn toLong(self , rsthis: & QStringRef) -> RetType;
}

  // proto:  long QStringRef::toLong(bool * ok, int base);
impl<'a> /*trait*/ QStringRef_toLong<i64> for (&'a mut Vec<i8>, i32) {
  fn toLong(self , rsthis: & QStringRef) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QStringRef6toLongEPbi()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {C_ZNK10QStringRef6toLongEPbi(rsthis.qclsinst, arg0, arg1)};
    return ret as i64; // 1
    // return 1;
  }
}

  // proto:  const QChar * QStringRef::cbegin();
impl /*struct*/ QStringRef {
  pub fn cbegin<RetType, T: QStringRef_cbegin<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.cbegin(self);
    // return 1;
  }
}

pub trait QStringRef_cbegin<RetType> {
  fn cbegin(self , rsthis: & QStringRef) -> RetType;
}

  // proto:  const QChar * QStringRef::cbegin();
impl<'a> /*trait*/ QStringRef_cbegin<QChar> for () {
  fn cbegin(self , rsthis: & QStringRef) -> QChar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QStringRef6cbeginEv()};
    let mut ret = unsafe {C_ZNK10QStringRef6cbeginEv(rsthis.qclsinst)};
    let mut ret1 = QChar::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  ushort QStringRef::toUShort(bool * ok, int base);
impl /*struct*/ QStringRef {
  pub fn toUShort<RetType, T: QStringRef_toUShort<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toUShort(self);
    // return 1;
  }
}

pub trait QStringRef_toUShort<RetType> {
  fn toUShort(self , rsthis: & QStringRef) -> RetType;
}

  // proto:  ushort QStringRef::toUShort(bool * ok, int base);
impl<'a> /*trait*/ QStringRef_toUShort<u16> for (&'a mut Vec<i8>, i32) {
  fn toUShort(self , rsthis: & QStringRef) -> u16 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QStringRef8toUShortEPbi()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {C_ZNK10QStringRef8toUShortEPbi(rsthis.qclsinst, arg0, arg1)};
    return ret as u16; // 1
    // return 1;
  }
}

  // proto:  uint QStringRef::toUInt(bool * ok, int base);
impl /*struct*/ QStringRef {
  pub fn toUInt<RetType, T: QStringRef_toUInt<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toUInt(self);
    // return 1;
  }
}

pub trait QStringRef_toUInt<RetType> {
  fn toUInt(self , rsthis: & QStringRef) -> RetType;
}

  // proto:  uint QStringRef::toUInt(bool * ok, int base);
impl<'a> /*trait*/ QStringRef_toUInt<u32> for (&'a mut Vec<i8>, i32) {
  fn toUInt(self , rsthis: & QStringRef) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QStringRef6toUIntEPbi()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {C_ZNK10QStringRef6toUIntEPbi(rsthis.qclsinst, arg0, arg1)};
    return ret as u32; // 1
    // return 1;
  }
}

  // proto:  bool QStringRef::isEmpty();
impl /*struct*/ QStringRef {
  pub fn isEmpty<RetType, T: QStringRef_isEmpty<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isEmpty(self);
    // return 1;
  }
}

pub trait QStringRef_isEmpty<RetType> {
  fn isEmpty(self , rsthis: & QStringRef) -> RetType;
}

  // proto:  bool QStringRef::isEmpty();
impl<'a> /*trait*/ QStringRef_isEmpty<i8> for () {
  fn isEmpty(self , rsthis: & QStringRef) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QStringRef7isEmptyEv()};
    let mut ret = unsafe {C_ZNK10QStringRef7isEmptyEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  int QStringRef::localeAwareCompare(const QString & s);
impl /*struct*/ QStringRef {
  pub fn localeAwareCompare<RetType, T: QStringRef_localeAwareCompare<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.localeAwareCompare(self);
    // return 1;
  }
}

pub trait QStringRef_localeAwareCompare<RetType> {
  fn localeAwareCompare(self , rsthis: & QStringRef) -> RetType;
}

  // proto:  int QStringRef::localeAwareCompare(const QString & s);
impl<'a> /*trait*/ QStringRef_localeAwareCompare<i32> for (&'a QString) {
  fn localeAwareCompare(self , rsthis: & QStringRef) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QStringRef18localeAwareCompareERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK10QStringRef18localeAwareCompareERK7QString(rsthis.qclsinst, arg0)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  QByteArray QStringRef::toUtf8();
impl /*struct*/ QStringRef {
  pub fn toUtf8<RetType, T: QStringRef_toUtf8<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toUtf8(self);
    // return 1;
  }
}

pub trait QStringRef_toUtf8<RetType> {
  fn toUtf8(self , rsthis: & QStringRef) -> RetType;
}

  // proto:  QByteArray QStringRef::toUtf8();
impl<'a> /*trait*/ QStringRef_toUtf8<QByteArray> for () {
  fn toUtf8(self , rsthis: & QStringRef) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QStringRef6toUtf8Ev()};
    let mut ret = unsafe {C_ZNK10QStringRef6toUtf8Ev(rsthis.qclsinst)};
    let mut ret1 = QByteArray::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QStringRef::size();
impl /*struct*/ QStringRef {
  pub fn size<RetType, T: QStringRef_size<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.size(self);
    // return 1;
  }
}

pub trait QStringRef_size<RetType> {
  fn size(self , rsthis: & QStringRef) -> RetType;
}

  // proto:  int QStringRef::size();
impl<'a> /*trait*/ QStringRef_size<i32> for () {
  fn size(self , rsthis: & QStringRef) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QStringRef4sizeEv()};
    let mut ret = unsafe {C_ZNK10QStringRef4sizeEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  const QChar * QStringRef::constData();
impl /*struct*/ QStringRef {
  pub fn constData<RetType, T: QStringRef_constData<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.constData(self);
    // return 1;
  }
}

pub trait QStringRef_constData<RetType> {
  fn constData(self , rsthis: & QStringRef) -> RetType;
}

  // proto:  const QChar * QStringRef::constData();
impl<'a> /*trait*/ QStringRef_constData<QChar> for () {
  fn constData(self , rsthis: & QStringRef) -> QChar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QStringRef9constDataEv()};
    let mut ret = unsafe {C_ZNK10QStringRef9constDataEv(rsthis.qclsinst)};
    let mut ret1 = QChar::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QStringRef QStringRef::left(int n);
impl /*struct*/ QStringRef {
  pub fn left<RetType, T: QStringRef_left<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.left(self);
    // return 1;
  }
}

pub trait QStringRef_left<RetType> {
  fn left(self , rsthis: & QStringRef) -> RetType;
}

  // proto:  QStringRef QStringRef::left(int n);
impl<'a> /*trait*/ QStringRef_left<QStringRef> for (i32) {
  fn left(self , rsthis: & QStringRef) -> QStringRef {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QStringRef4leftEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {C_ZNK10QStringRef4leftEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QStringRef::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QVector<uint> QStringRef::toUcs4();
impl /*struct*/ QStringRef {
  pub fn toUcs4<RetType, T: QStringRef_toUcs4<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toUcs4(self);
    // return 1;
  }
}

pub trait QStringRef_toUcs4<RetType> {
  fn toUcs4(self , rsthis: & QStringRef) -> RetType;
}

  // proto:  QVector<uint> QStringRef::toUcs4();
impl<'a> /*trait*/ QStringRef_toUcs4<u64> for () {
  fn toUcs4(self , rsthis: & QStringRef) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QStringRef6toUcs4Ev()};
    let mut ret = unsafe {C_ZNK10QStringRef6toUcs4Ev(rsthis.qclsinst)};
    return ret as u64; // 5
    // return 1;
  }
}

  // proto:  int QStringRef::count();
impl /*struct*/ QStringRef {
  pub fn count<RetType, T: QStringRef_count<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.count(self);
    // return 1;
  }
}

pub trait QStringRef_count<RetType> {
  fn count(self , rsthis: & QStringRef) -> RetType;
}

  // proto:  int QStringRef::count();
impl<'a> /*trait*/ QStringRef_count<i32> for () {
  fn count(self , rsthis: & QStringRef) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QStringRef5countEv()};
    let mut ret = unsafe {C_ZNK10QStringRef5countEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  void QStringRef::QStringRef(const QString * string, int position, int size);
impl<'a> /*trait*/ QStringRef_new for (&'a QString, i32, i32) {
  fn new(self) -> QStringRef {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QStringRefC2EPK7QStringii()};
    let ctysz: c_int = unsafe{QStringRef_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let qthis: u64 = unsafe {C_ZN10QStringRefC2EPK7QStringii(arg0, arg1, arg2)};
    let rsthis = QStringRef{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStringRef::QStringRef();
impl<'a> /*trait*/ QStringRef_new for () {
  fn new(self) -> QStringRef {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QStringRefC2Ev()};
    let ctysz: c_int = unsafe{QStringRef_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let qthis: u64 = unsafe {C_ZN10QStringRefC2Ev()};
    let rsthis = QStringRef{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QStringRef QStringRef::right(int n);
impl /*struct*/ QStringRef {
  pub fn right<RetType, T: QStringRef_right<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.right(self);
    // return 1;
  }
}

pub trait QStringRef_right<RetType> {
  fn right(self , rsthis: & QStringRef) -> RetType;
}

  // proto:  QStringRef QStringRef::right(int n);
impl<'a> /*trait*/ QStringRef_right<QStringRef> for (i32) {
  fn right(self , rsthis: & QStringRef) -> QStringRef {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QStringRef5rightEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {C_ZNK10QStringRef5rightEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QStringRef::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  const QChar QStringRef::at(int i);
impl /*struct*/ QStringRef {
  pub fn at<RetType, T: QStringRef_at<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.at(self);
    // return 1;
  }
}

pub trait QStringRef_at<RetType> {
  fn at(self , rsthis: & QStringRef) -> RetType;
}

  // proto:  const QChar QStringRef::at(int i);
impl<'a> /*trait*/ QStringRef_at<QChar> for (i32) {
  fn at(self , rsthis: & QStringRef) -> QChar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QStringRef2atEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {C_ZNK10QStringRef2atEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QChar::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  double QStringRef::toDouble(bool * ok);
impl /*struct*/ QStringRef {
  pub fn toDouble<RetType, T: QStringRef_toDouble<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toDouble(self);
    // return 1;
  }
}

pub trait QStringRef_toDouble<RetType> {
  fn toDouble(self , rsthis: & QStringRef) -> RetType;
}

  // proto:  double QStringRef::toDouble(bool * ok);
impl<'a> /*trait*/ QStringRef_toDouble<f64> for (&'a mut Vec<i8>) {
  fn toDouble(self , rsthis: & QStringRef) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QStringRef8toDoubleEPb()};
    let arg0 = self.as_ptr()  as *mut c_char;
    let mut ret = unsafe {C_ZNK10QStringRef8toDoubleEPb(rsthis.qclsinst, arg0)};
    return ret as f64; // 1
    // return 1;
  }
}

  // proto:  bool QStringRef::isNull();
impl /*struct*/ QStringRef {
  pub fn isNull<RetType, T: QStringRef_isNull<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isNull(self);
    // return 1;
  }
}

pub trait QStringRef_isNull<RetType> {
  fn isNull(self , rsthis: & QStringRef) -> RetType;
}

  // proto:  bool QStringRef::isNull();
impl<'a> /*trait*/ QStringRef_isNull<i8> for () {
  fn isNull(self , rsthis: & QStringRef) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QStringRef6isNullEv()};
    let mut ret = unsafe {C_ZNK10QStringRef6isNullEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  const QChar * QStringRef::data();
impl /*struct*/ QStringRef {
  pub fn data<RetType, T: QStringRef_data<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.data(self);
    // return 1;
  }
}

pub trait QStringRef_data<RetType> {
  fn data(self , rsthis: & QStringRef) -> RetType;
}

  // proto:  const QChar * QStringRef::data();
impl<'a> /*trait*/ QStringRef_data<QChar> for () {
  fn data(self , rsthis: & QStringRef) -> QChar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QStringRef4dataEv()};
    let mut ret = unsafe {C_ZNK10QStringRef4dataEv(rsthis.qclsinst)};
    let mut ret1 = QChar::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  qlonglong QStringRef::toLongLong(bool * ok, int base);
impl /*struct*/ QStringRef {
  pub fn toLongLong<RetType, T: QStringRef_toLongLong<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toLongLong(self);
    // return 1;
  }
}

pub trait QStringRef_toLongLong<RetType> {
  fn toLongLong(self , rsthis: & QStringRef) -> RetType;
}

  // proto:  qlonglong QStringRef::toLongLong(bool * ok, int base);
impl<'a> /*trait*/ QStringRef_toLongLong<i64> for (&'a mut Vec<i8>, i32) {
  fn toLongLong(self , rsthis: & QStringRef) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QStringRef10toLongLongEPbi()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {C_ZNK10QStringRef10toLongLongEPbi(rsthis.qclsinst, arg0, arg1)};
    return ret as i64; // 1
    // return 1;
  }
}

  // proto:  QByteArray QStringRef::toLatin1();
impl /*struct*/ QStringRef {
  pub fn toLatin1<RetType, T: QStringRef_toLatin1<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toLatin1(self);
    // return 1;
  }
}

pub trait QStringRef_toLatin1<RetType> {
  fn toLatin1(self , rsthis: & QStringRef) -> RetType;
}

  // proto:  QByteArray QStringRef::toLatin1();
impl<'a> /*trait*/ QStringRef_toLatin1<QByteArray> for () {
  fn toLatin1(self , rsthis: & QStringRef) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QStringRef8toLatin1Ev()};
    let mut ret = unsafe {C_ZNK10QStringRef8toLatin1Ev(rsthis.qclsinst)};
    let mut ret1 = QByteArray::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  const QChar * QStringRef::begin();
impl /*struct*/ QStringRef {
  pub fn begin<RetType, T: QStringRef_begin<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.begin(self);
    // return 1;
  }
}

pub trait QStringRef_begin<RetType> {
  fn begin(self , rsthis: & QStringRef) -> RetType;
}

  // proto:  const QChar * QStringRef::begin();
impl<'a> /*trait*/ QStringRef_begin<QChar> for () {
  fn begin(self , rsthis: & QStringRef) -> QChar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QStringRef5beginEv()};
    let mut ret = unsafe {C_ZNK10QStringRef5beginEv(rsthis.qclsinst)};
    let mut ret1 = QChar::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  const QChar * QStringRef::unicode();
impl /*struct*/ QStringRef {
  pub fn unicode<RetType, T: QStringRef_unicode<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.unicode(self);
    // return 1;
  }
}

pub trait QStringRef_unicode<RetType> {
  fn unicode(self , rsthis: & QStringRef) -> RetType;
}

  // proto:  const QChar * QStringRef::unicode();
impl<'a> /*trait*/ QStringRef_unicode<QChar> for () {
  fn unicode(self , rsthis: & QStringRef) -> QChar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QStringRef7unicodeEv()};
    let mut ret = unsafe {C_ZNK10QStringRef7unicodeEv(rsthis.qclsinst)};
    let mut ret1 = QChar::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QStringRef QStringRef::mid(int pos, int n);
impl /*struct*/ QStringRef {
  pub fn mid<RetType, T: QStringRef_mid<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.mid(self);
    // return 1;
  }
}

pub trait QStringRef_mid<RetType> {
  fn mid(self , rsthis: & QStringRef) -> RetType;
}

  // proto:  QStringRef QStringRef::mid(int pos, int n);
impl<'a> /*trait*/ QStringRef_mid<QStringRef> for (i32, i32) {
  fn mid(self , rsthis: & QStringRef) -> QStringRef {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QStringRef3midEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {C_ZNK10QStringRef3midEii(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QStringRef::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  float QStringRef::toFloat(bool * ok);
impl /*struct*/ QStringRef {
  pub fn toFloat<RetType, T: QStringRef_toFloat<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toFloat(self);
    // return 1;
  }
}

pub trait QStringRef_toFloat<RetType> {
  fn toFloat(self , rsthis: & QStringRef) -> RetType;
}

  // proto:  float QStringRef::toFloat(bool * ok);
impl<'a> /*trait*/ QStringRef_toFloat<f32> for (&'a mut Vec<i8>) {
  fn toFloat(self , rsthis: & QStringRef) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QStringRef7toFloatEPb()};
    let arg0 = self.as_ptr()  as *mut c_char;
    let mut ret = unsafe {C_ZNK10QStringRef7toFloatEPb(rsthis.qclsinst, arg0)};
    return ret as f32; // 1
    // return 1;
  }
}

  // proto:  const QString * QStringRef::string();
impl /*struct*/ QStringRef {
  pub fn string<RetType, T: QStringRef_string<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.string(self);
    // return 1;
  }
}

pub trait QStringRef_string<RetType> {
  fn string(self , rsthis: & QStringRef) -> RetType;
}

  // proto:  const QString * QStringRef::string();
impl<'a> /*trait*/ QStringRef_string<QString> for () {
  fn string(self , rsthis: & QStringRef) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QStringRef6stringEv()};
    let mut ret = unsafe {C_ZNK10QStringRef6stringEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QStringRef::toString();
impl /*struct*/ QStringRef {
  pub fn toString<RetType, T: QStringRef_toString<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toString(self);
    // return 1;
  }
}

pub trait QStringRef_toString<RetType> {
  fn toString(self , rsthis: & QStringRef) -> RetType;
}

  // proto:  QString QStringRef::toString();
impl<'a> /*trait*/ QStringRef_toString<QString> for () {
  fn toString(self , rsthis: & QStringRef) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QStringRef8toStringEv()};
    let mut ret = unsafe {C_ZNK10QStringRef8toStringEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QStringRef QStringRef::trimmed();
impl /*struct*/ QStringRef {
  pub fn trimmed<RetType, T: QStringRef_trimmed<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.trimmed(self);
    // return 1;
  }
}

pub trait QStringRef_trimmed<RetType> {
  fn trimmed(self , rsthis: & QStringRef) -> RetType;
}

  // proto:  QStringRef QStringRef::trimmed();
impl<'a> /*trait*/ QStringRef_trimmed<QStringRef> for () {
  fn trimmed(self , rsthis: & QStringRef) -> QStringRef {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QStringRef7trimmedEv()};
    let mut ret = unsafe {C_ZNK10QStringRef7trimmedEv(rsthis.qclsinst)};
    let mut ret1 = QStringRef::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QStringRef::toInt(bool * ok, int base);
impl /*struct*/ QStringRef {
  pub fn toInt<RetType, T: QStringRef_toInt<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toInt(self);
    // return 1;
  }
}

pub trait QStringRef_toInt<RetType> {
  fn toInt(self , rsthis: & QStringRef) -> RetType;
}

  // proto:  int QStringRef::toInt(bool * ok, int base);
impl<'a> /*trait*/ QStringRef_toInt<i32> for (&'a mut Vec<i8>, i32) {
  fn toInt(self , rsthis: & QStringRef) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QStringRef5toIntEPbi()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {C_ZNK10QStringRef5toIntEPbi(rsthis.qclsinst, arg0, arg1)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  const QChar * QStringRef::cend();
impl /*struct*/ QStringRef {
  pub fn cend<RetType, T: QStringRef_cend<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.cend(self);
    // return 1;
  }
}

pub trait QStringRef_cend<RetType> {
  fn cend(self , rsthis: & QStringRef) -> RetType;
}

  // proto:  const QChar * QStringRef::cend();
impl<'a> /*trait*/ QStringRef_cend<QChar> for () {
  fn cend(self , rsthis: & QStringRef) -> QChar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QStringRef4cendEv()};
    let mut ret = unsafe {C_ZNK10QStringRef4cendEv(rsthis.qclsinst)};
    let mut ret1 = QChar::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QStringRef QStringRef::appendTo(QString * string);
impl /*struct*/ QStringRef {
  pub fn appendTo<RetType, T: QStringRef_appendTo<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.appendTo(self);
    // return 1;
  }
}

pub trait QStringRef_appendTo<RetType> {
  fn appendTo(self , rsthis: & QStringRef) -> RetType;
}

  // proto:  QStringRef QStringRef::appendTo(QString * string);
impl<'a> /*trait*/ QStringRef_appendTo<QStringRef> for (&'a QString) {
  fn appendTo(self , rsthis: & QStringRef) -> QStringRef {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QStringRef8appendToEP7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK10QStringRef8appendToEP7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QStringRef::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QStringRef::length();
impl /*struct*/ QStringRef {
  pub fn length<RetType, T: QStringRef_length<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.length(self);
    // return 1;
  }
}

pub trait QStringRef_length<RetType> {
  fn length(self , rsthis: & QStringRef) -> RetType;
}

  // proto:  int QStringRef::length();
impl<'a> /*trait*/ QStringRef_length<i32> for () {
  fn length(self , rsthis: & QStringRef) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QStringRef6lengthEv()};
    let mut ret = unsafe {C_ZNK10QStringRef6lengthEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  void QStringRef::~QStringRef();
impl /*struct*/ QStringRef {
  pub fn free<RetType, T: QStringRef_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QStringRef_free<RetType> {
  fn free(self , rsthis: & QStringRef) -> RetType;
}

  // proto:  void QStringRef::~QStringRef();
impl<'a> /*trait*/ QStringRef_free<()> for () {
  fn free(self , rsthis: & QStringRef) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QStringRefD2Ev()};
     unsafe {C_ZN10QStringRefD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QByteArray QStringRef::toLocal8Bit();
impl /*struct*/ QStringRef {
  pub fn toLocal8Bit<RetType, T: QStringRef_toLocal8Bit<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toLocal8Bit(self);
    // return 1;
  }
}

pub trait QStringRef_toLocal8Bit<RetType> {
  fn toLocal8Bit(self , rsthis: & QStringRef) -> RetType;
}

  // proto:  QByteArray QStringRef::toLocal8Bit();
impl<'a> /*trait*/ QStringRef_toLocal8Bit<QByteArray> for () {
  fn toLocal8Bit(self , rsthis: & QStringRef) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QStringRef11toLocal8BitEv()};
    let mut ret = unsafe {C_ZNK10QStringRef11toLocal8BitEv(rsthis.qclsinst)};
    let mut ret1 = QByteArray::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  ulong QStringRef::toULong(bool * ok, int base);
impl /*struct*/ QStringRef {
  pub fn toULong<RetType, T: QStringRef_toULong<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toULong(self);
    // return 1;
  }
}

pub trait QStringRef_toULong<RetType> {
  fn toULong(self , rsthis: & QStringRef) -> RetType;
}

  // proto:  ulong QStringRef::toULong(bool * ok, int base);
impl<'a> /*trait*/ QStringRef_toULong<u64> for (&'a mut Vec<i8>, i32) {
  fn toULong(self , rsthis: & QStringRef) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QStringRef7toULongEPbi()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {C_ZNK10QStringRef7toULongEPbi(rsthis.qclsinst, arg0, arg1)};
    return ret as u64; // 1
    // return 1;
  }
}

  // proto:  const QChar * QStringRef::end();
impl /*struct*/ QStringRef {
  pub fn end<RetType, T: QStringRef_end<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.end(self);
    // return 1;
  }
}

pub trait QStringRef_end<RetType> {
  fn end(self , rsthis: & QStringRef) -> RetType;
}

  // proto:  const QChar * QStringRef::end();
impl<'a> /*trait*/ QStringRef_end<QChar> for () {
  fn end(self , rsthis: & QStringRef) -> QChar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QStringRef3endEv()};
    let mut ret = unsafe {C_ZNK10QStringRef3endEv(rsthis.qclsinst)};
    let mut ret1 = QChar::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

// <= body block end

