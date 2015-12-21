// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qchar::QChar;
use super::qregularexpression::QRegularExpression;
use super::qregexp::QRegExp;
use super::qbytearray::QByteArray;
use super::qregularexpressionmatch::QRegularExpressionMatch;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  qlonglong QString::toLongLong(bool * ok, int base);
  fn _ZNK7QString10toLongLongEPbi(qthis: *mut c_void, arg0: *mut c_char, arg1: c_int) -> c_longlong;
  // proto:  bool QString::isNull();
  fn _ZNK7QString6isNullEv(qthis: *mut c_void) -> c_char;
  // proto:  QString & QString::append(const QChar * uc, int len);
  fn _ZN7QString6appendEPK5QChari(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) -> *mut c_void;
  // proto:  QString & QString::prepend(QChar c);
  fn _ZN7QString7prependE5QChar(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QString & QString::insert(int i, QChar c);
  fn _ZN7QString6insertEi5QChar(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) -> *mut c_void;
  // proto:  QString QString::left(int n);
  fn _ZNK7QString4leftEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  void QString::QString(QChar c);
  fn _ZN7QStringC1E5QChar(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QString::isEmpty();
  fn _ZNK7QString7isEmptyEv(qthis: *mut c_void) -> c_char;
  // proto:  QString & QString::prepend(const char * s);
  fn _ZN7QString7prependEPKc(qthis: *mut c_void, arg0: *mut c_char) -> *mut c_void;
  // proto:  int QString::lastIndexOf(const QRegularExpression & re, int from);
  fn _ZNK7QString11lastIndexOfERK18QRegularExpressioni(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) -> c_int;
  // proto: static QString QString::number(int , int base);
  fn _ZN7QString6numberEii(arg0: c_int, arg1: c_int) -> *mut c_void;
  // proto:  void QString::resize(int size);
  fn _ZN7QString6resizeEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QString::push_front(QChar c);
  fn _ZN7QString10push_frontE5QChar(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QString::QString();
  fn _ZN7QStringC1Ev(qthis: *mut c_void);
  // proto:  double QString::toDouble(bool * ok);
  fn _ZNK7QString8toDoubleEPb(qthis: *mut c_void, arg0: *mut c_char) -> c_double;
  // proto:  QString QString::arg(const QString & a1, const QString & a2, const QString & a3, const QString & a4, const QString & a5, const QString & a6, const QString & a7, const QString & a8);
  fn _ZNK7QString3argERKS_S1_S1_S1_S1_S1_S1_S1_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void, arg3: *mut c_void, arg4: *mut c_void, arg5: *mut c_void, arg6: *mut c_void, arg7: *mut c_void) -> *mut c_void;
  // proto:  QStringRef QString::rightRef(int n);
  fn _ZNK7QString8rightRefEi(qthis: *mut c_void, arg0: c_int);
  // proto:  QString & QString::setNum(short , int base);
  fn _ZN7QString6setNumEsi(qthis: *mut c_void, arg0: c_short, arg1: c_int) -> *mut c_void;
  // proto:  void QString::QString(const QChar * unicode, int size);
  fn _ZN7QStringC1EPK5QChari(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int);
  // proto:  float QString::toFloat(bool * ok);
  fn _ZNK7QString7toFloatEPb(qthis: *mut c_void, arg0: *mut c_char) -> c_float;
  // proto:  int QString::count(const QRegularExpression & re);
  fn _ZNK7QString5countERK18QRegularExpression(qthis: *mut c_void, arg0: *mut c_void) -> c_int;
  // proto:  QStringRef QString::midRef(int position, int n);
  fn _ZNK7QString6midRefEii(qthis: *mut c_void, arg0: c_int, arg1: c_int);
  // proto:  void QString::detach();
  fn _ZN7QString6detachEv(qthis: *mut c_void);
  // proto:  QString QString::arg(const QString & a1, const QString & a2, const QString & a3, const QString & a4);
  fn _ZNK7QString3argERKS_S1_S1_S1_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void, arg3: *mut c_void) -> *mut c_void;
  // proto:  QString QString::arg(long a, int fieldwidth, int base, QChar fillChar);
  fn _ZNK7QString3argElii5QChar(qthis: *mut c_void, arg0: c_long, arg1: c_int, arg2: c_int, arg3: *mut c_void) -> *mut c_void;
  // proto:  int QString::count();
  fn _ZNK7QString5countEv(qthis: *mut c_void) -> c_int;
  // proto:  QString & QString::setNum(qulonglong , int base);
  fn _ZN7QString6setNumEyi(qthis: *mut c_void, arg0: c_ulonglong, arg1: c_int) -> *mut c_void;
  // proto: static QString QString::fromStdWString(const std::wstring & s);
  fn _ZN7QString14fromStdWStringERKi(arg0: *mut c_int) -> *mut c_void;
  // proto:  void QString::push_back(QChar c);
  fn _ZN7QString9push_backE5QChar(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QString & QString::setNum(float , char f, int prec);
  fn _ZN7QString6setNumEfci(qthis: *mut c_void, arg0: c_float, arg1: c_char, arg2: c_int) -> *mut c_void;
  // proto:  int QString::count(const QRegExp & );
  fn _ZNK7QString5countERK7QRegExp(qthis: *mut c_void, arg0: *mut c_void) -> c_int;
  // proto:  int QString::size();
  fn _ZNK7QString4sizeEv(qthis: *mut c_void) -> c_int;
  // proto:  QString & QString::insert(int i, const QChar * uc, int len);
  fn _ZN7QString6insertEiPK5QChari(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void, arg2: c_int) -> *mut c_void;
  // proto:  QString & QString::replace(int i, int len, const QChar * s, int slen);
  fn _ZN7QString7replaceEiiPK5QChari(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *mut c_void, arg3: c_int) -> *mut c_void;
  // proto: static QString QString::fromRawData(const QChar * , int size);
  fn _ZN7QString11fromRawDataEPK5QChari(arg0: *mut c_void, arg1: c_int) -> *mut c_void;
  // proto:  QString & QString::insert(int i, const QString & s);
  fn _ZN7QString6insertEiRKS_(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) -> *mut c_void;
  // proto:  QString QString::arg(const QString & a1, const QString & a2, const QString & a3, const QString & a4, const QString & a5);
  fn _ZNK7QString3argERKS_S1_S1_S1_S1_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void, arg3: *mut c_void, arg4: *mut c_void) -> *mut c_void;
  // proto:  QString & QString::setRawData(const QChar * unicode, int size);
  fn _ZN7QString10setRawDataEPK5QChari(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) -> *mut c_void;
  // proto:  QString & QString::prepend(const QString & s);
  fn _ZN7QString7prependERKS_(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QString & QString::setNum(int , int base);
  fn _ZN7QString6setNumEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) -> *mut c_void;
  // proto:  ulong QString::toULong(bool * ok, int base);
  fn _ZNK7QString7toULongEPbi(qthis: *mut c_void, arg0: *mut c_char, arg1: c_int) -> c_ulong;
  // proto:  void QString::chop(int n);
  fn _ZN7QString4chopEi(qthis: *mut c_void, arg0: c_int);
  // proto: static QString QString::fromUtf16(const ushort * , int size);
  fn _ZN7QString9fromUtf16EPKti(arg0: *mut c_ushort, arg1: c_int) -> *mut c_void;
  // proto:  bool QString::isDetached();
  fn _ZNK7QString10isDetachedEv(qthis: *mut c_void) -> c_char;
  // proto:  QString & QString::setNum(qlonglong , int base);
  fn _ZN7QString6setNumExi(qthis: *mut c_void, arg0: c_longlong, arg1: c_int) -> *mut c_void;
  // proto:  QString QString::mid(int position, int n);
  fn _ZNK7QString3midEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) -> *mut c_void;
  // proto: static QString QString::fromLocal8Bit(const char * str, int size);
  fn _ZN7QString13fromLocal8BitEPKci(arg0: *mut c_char, arg1: c_int) -> *mut c_void;
  // proto:  void QString::swap(QString & other);
  fn _ZN7QString4swapERS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto: static QString QString::fromUtf8(const QByteArray & str);
  fn _ZN7QString8fromUtf8ERK10QByteArray(arg0: *mut c_void) -> *mut c_void;
  // proto: static QString QString::fromUcs4(const char32_t * str, int size);
  fn _ZN7QString8fromUcs4EPKDii(arg0: *mut c_char, arg1: c_int) -> *mut c_void;
  // proto:  QString QString::leftJustified(int width, QChar fill, bool trunc);
  fn _ZNK7QString13leftJustifiedEi5QCharb(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void, arg2: c_char) -> *mut c_void;
  // proto:  int QString::indexOf(const QRegExp & , int from);
  fn _ZNK7QString7indexOfERK7QRegExpi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) -> c_int;
  // proto:  void QString::push_back(const QString & s);
  fn _ZN7QString9push_backERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  int QString::lastIndexOf(QRegExp & , int from);
  fn _ZNK7QString11lastIndexOfER7QRegExpi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) -> c_int;
  // proto:  QString QString::arg(const QString & a1, const QString & a2, const QString & a3);
  fn _ZNK7QString3argERKS_S1_S1_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) -> *mut c_void;
  // proto:  const ushort * QString::utf16();
  fn _ZNK7QString5utf16Ev(qthis: *mut c_void);
  // proto:  int QString::toInt(bool * ok, int base);
  fn _ZNK7QString5toIntEPbi(qthis: *mut c_void, arg0: *mut c_char, arg1: c_int) -> c_int;
  // proto:  QString QString::arg(double a, int fieldWidth, char fmt, int prec, QChar fillChar);
  fn _ZNK7QString3argEdici5QChar(qthis: *mut c_void, arg0: c_double, arg1: c_int, arg2: c_char, arg3: c_int, arg4: *mut c_void) -> *mut c_void;
  // proto:  QChar * QString::data();
  fn _ZN7QString4dataEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QString & QString::setNum(uint , int base);
  fn _ZN7QString6setNumEji(qthis: *mut c_void, arg0: c_uint, arg1: c_int) -> *mut c_void;
  // proto: static int QString::localeAwareCompare(const QString & s1, const QString & s2);
  fn _ZN7QString18localeAwareCompareERKS_S1_(arg0: *mut c_void, arg1: *mut c_void) -> c_int;
  // proto:  void QString::QString(const char * ch);
  fn _ZN7QStringC1EPKc(qthis: *mut c_void, arg0: *mut c_char);
  // proto: static QString QString::fromUtf16(const char16_t * str, int size);
  fn _ZN7QString9fromUtf16EPKDsi(arg0: *mut c_char, arg1: c_int) -> *mut c_void;
  // proto:  QString & QString::replace(const QRegExp & rx, const QString & after);
  fn _ZN7QString7replaceERK7QRegExpRKS_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  QString QString::repeated(int times);
  fn _ZNK7QString8repeatedEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  QString & QString::setUtf16(const ushort * utf16, int size);
  fn _ZN7QString8setUtf16EPKti(qthis: *mut c_void, arg0: *mut c_ushort, arg1: c_int) -> *mut c_void;
  // proto: static QString QString::fromStdU32String(const std::u32string & s);
  fn _ZN7QString16fromStdU32StringERKi(arg0: *mut c_int) -> *mut c_void;
  // proto:  void QString::clear();
  fn _ZN7QString5clearEv(qthis: *mut c_void);
  // proto:  bool QString::contains(const QRegExp & rx);
  fn _ZNK7QString8containsERK7QRegExp(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  bool QString::isSharedWith(const QString & other);
  fn _ZNK7QString12isSharedWithERKS_(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto: static QString QString::fromLatin1(const QByteArray & str);
  fn _ZN7QString10fromLatin1ERK10QByteArray(arg0: *mut c_void) -> *mut c_void;
  // proto:  void QString::~QString();
  fn _ZN7QStringD0Ev(qthis: *mut c_void);
  // proto:  QString & QString::remove(const QRegularExpression & re);
  fn _ZN7QString6removeERK18QRegularExpression(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  const_iterator QString::cend();
  fn _ZNK7QString4cendEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QString QString::toHtmlEscaped();
  fn _ZNK7QString13toHtmlEscapedEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QString & QString::append(const QByteArray & s);
  fn _ZN7QString6appendERK10QByteArray(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto: static QString QString::fromLatin1(const char * str, int size);
  fn _ZN7QString10fromLatin1EPKci(arg0: *mut c_char, arg1: c_int) -> *mut c_void;
  // proto:  bool QString::contains(const QRegularExpression & re, QRegularExpressionMatch * match);
  fn _ZNK7QString8containsERK18QRegularExpressionP23QRegularExpressionMatch(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> c_char;
  // proto:  int QString::indexOf(const QRegularExpression & re, int from, QRegularExpressionMatch * rmatch);
  fn _ZNK7QString7indexOfERK18QRegularExpressioniP23QRegularExpressionMatch(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int, arg2: *mut c_void) -> c_int;
  // proto:  int QString::lastIndexOf(const QRegExp & , int from);
  fn _ZNK7QString11lastIndexOfERK7QRegExpi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) -> c_int;
  // proto:  int QString::toWCharArray(wchar_t * array);
  fn _ZNK7QString12toWCharArrayEPw(qthis: *mut c_void, arg0: *mut wchar_t) -> c_int;
  // proto:  const_iterator QString::cbegin();
  fn _ZNK7QString6cbeginEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QString & QString::prepend(const QByteArray & s);
  fn _ZN7QString7prependERK10QByteArray(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QString & QString::replace(int i, int len, const QString & after);
  fn _ZN7QString7replaceEiiRKS_(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *mut c_void) -> *mut c_void;
  // proto: static QString QString::fromStdString(const std::string & s);
  fn _ZN7QString13fromStdStringERKi(arg0: *mut c_int) -> *mut c_void;
  // proto:  QString QString::arg(const QString & a1, const QString & a2, const QString & a3, const QString & a4, const QString & a5, const QString & a6, const QString & a7);
  fn _ZNK7QString3argERKS_S1_S1_S1_S1_S1_S1_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void, arg3: *mut c_void, arg4: *mut c_void, arg5: *mut c_void, arg6: *mut c_void) -> *mut c_void;
  // proto: static QString QString::fromWCharArray(const wchar_t * string, int size);
  fn _ZN7QString14fromWCharArrayEPKwi(arg0: *mut wchar_t, arg1: c_int) -> *mut c_void;
  // proto:  QString & QString::fill(QChar c, int size);
  fn _ZN7QString4fillE5QChari(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) -> *mut c_void;
  // proto:  const QChar * QString::constData();
  fn _ZNK7QString9constDataEv(qthis: *mut c_void) -> *mut c_void;
  // proto: static QString QString::number(ulong , int base);
  fn _ZN7QString6numberEmi(arg0: c_ulong, arg1: c_int) -> *mut c_void;
  // proto:  long QString::toLong(bool * ok, int base);
  fn _ZNK7QString6toLongEPbi(qthis: *mut c_void, arg0: *mut c_char, arg1: c_int) -> c_long;
  // proto:  const_iterator QString::constEnd();
  fn _ZNK7QString8constEndEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QString::length();
  fn _ZNK7QString6lengthEv(qthis: *mut c_void) -> c_int;
  // proto: static QString QString::fromUtf8(const char * str, int size);
  fn _ZN7QString8fromUtf8EPKci(arg0: *mut c_char, arg1: c_int) -> *mut c_void;
  // proto: static QString QString::number(qlonglong , int base);
  fn _ZN7QString6numberExi(arg0: c_longlong, arg1: c_int) -> *mut c_void;
  // proto:  QStringRef QString::leftRef(int n);
  fn _ZNK7QString7leftRefEi(qthis: *mut c_void, arg0: c_int);
  // proto:  QString & QString::setNum(long , int base);
  fn _ZN7QString6setNumEli(qthis: *mut c_void, arg0: c_long, arg1: c_int) -> *mut c_void;
  // proto:  QString QString::arg(const QString & a1, const QString & a2);
  fn _ZNK7QString3argERKS_S1_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  bool QString::isSimpleText();
  fn _ZNK7QString12isSimpleTextEv(qthis: *mut c_void) -> c_char;
  // proto: static QString QString::fromUcs4(const uint * , int size);
  fn _ZN7QString8fromUcs4EPKji(arg0: *mut c_uint, arg1: c_int) -> *mut c_void;
  // proto:  QString & QString::setUnicode(const QChar * unicode, int size);
  fn _ZN7QString10setUnicodeEPK5QChari(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) -> *mut c_void;
  // proto:  bool QString::contains(QRegExp & rx);
  fn _ZNK7QString8containsER7QRegExp(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  const_iterator QString::constBegin();
  fn _ZNK7QString10constBeginEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  const QChar * QString::unicode();
  fn _ZNK7QString7unicodeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QString QString::arg(const QString & a1, const QString & a2, const QString & a3, const QString & a4, const QString & a5, const QString & a6, const QString & a7, const QString & a8, const QString & a9);
  fn _ZNK7QString3argERKS_S1_S1_S1_S1_S1_S1_S1_S1_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void, arg3: *mut c_void, arg4: *mut c_void, arg5: *mut c_void, arg6: *mut c_void, arg7: *mut c_void, arg8: *mut c_void) -> *mut c_void;
  // proto:  int QString::indexOf(const QRegularExpression & re, int from);
  fn _ZNK7QString7indexOfERK18QRegularExpressioni(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) -> c_int;
  // proto: static QString QString::number(long , int base);
  fn _ZN7QString6numberEli(arg0: c_long, arg1: c_int) -> *mut c_void;
  // proto: static QString QString::number(uint , int base);
  fn _ZN7QString6numberEji(arg0: c_uint, arg1: c_int) -> *mut c_void;
  // proto: static QString QString::fromLocal8Bit(const QByteArray & str);
  fn _ZN7QString13fromLocal8BitERK10QByteArray(arg0: *mut c_void) -> *mut c_void;
  // proto:  const QChar QString::at(int i);
  fn _ZNK7QString2atEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  void QString::QString(int size, QChar c);
  fn _ZN7QStringC1Ei5QChar(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void);
  // proto:  QString & QString::setNum(ulong , int base);
  fn _ZN7QString6setNumEmi(qthis: *mut c_void, arg0: c_ulong, arg1: c_int) -> *mut c_void;
  // proto:  void QString::push_front(const QString & s);
  fn _ZN7QString10push_frontERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QString QString::arg(const QString & a1, const QString & a2, const QString & a3, const QString & a4, const QString & a5, const QString & a6);
  fn _ZNK7QString3argERKS_S1_S1_S1_S1_S1_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void, arg3: *mut c_void, arg4: *mut c_void, arg5: *mut c_void) -> *mut c_void;
  // proto:  iterator QString::begin();
  fn _ZN7QString5beginEv(qthis: *mut c_void) -> *mut c_void;
  // proto: static QString QString::number(double , char f, int prec);
  fn _ZN7QString6numberEdci(arg0: c_double, arg1: c_char, arg2: c_int) -> *mut c_void;
  // proto:  iterator QString::end();
  fn _ZN7QString3endEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QString & QString::append(QChar c);
  fn _ZN7QString6appendE5QChar(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  uint QString::toUInt(bool * ok, int base);
  fn _ZNK7QString6toUIntEPbi(qthis: *mut c_void, arg0: *mut c_char, arg1: c_int) -> c_uint;
  // proto:  QString & QString::append(const QString & s);
  fn _ZN7QString6appendERKS_(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto: static QString QString::fromStdU16String(const std::u16string & s);
  fn _ZN7QString16fromStdU16StringERKi(arg0: *mut c_int) -> *mut c_void;
  // proto:  QString QString::arg(qlonglong a, int fieldwidth, int base, QChar fillChar);
  fn _ZNK7QString3argExii5QChar(qthis: *mut c_void, arg0: c_longlong, arg1: c_int, arg2: c_int, arg3: *mut c_void) -> *mut c_void;
  // proto:  ushort QString::toUShort(bool * ok, int base);
  fn _ZNK7QString8toUShortEPbi(qthis: *mut c_void, arg0: *mut c_char, arg1: c_int) -> c_ushort;
  // proto:  QString QString::arg(uint a, int fieldWidth, int base, QChar fillChar);
  fn _ZNK7QString3argEjii5QChar(qthis: *mut c_void, arg0: c_uint, arg1: c_int, arg2: c_int, arg3: *mut c_void) -> *mut c_void;
  // proto:  QString & QString::setNum(ushort , int base);
  fn _ZN7QString6setNumEti(qthis: *mut c_void, arg0: c_ushort, arg1: c_int) -> *mut c_void;
  // proto:  QString & QString::replace(const QRegularExpression & re, const QString & after);
  fn _ZN7QString7replaceERK18QRegularExpressionRKS_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  QString & QString::setNum(double , char f, int prec);
  fn _ZN7QString6setNumEdci(qthis: *mut c_void, arg0: c_double, arg1: c_char, arg2: c_int) -> *mut c_void;
  // proto: static QString QString::number(qulonglong , int base);
  fn _ZN7QString6numberEyi(arg0: c_ulonglong, arg1: c_int) -> *mut c_void;
  // proto:  QString QString::arg(ushort a, int fieldWidth, int base, QChar fillChar);
  fn _ZNK7QString3argEtii5QChar(qthis: *mut c_void, arg0: c_ushort, arg1: c_int, arg2: c_int, arg3: *mut c_void) -> *mut c_void;
  // proto:  void QString::QString(const QString & );
  fn _ZN7QStringC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QString QString::arg(short a, int fieldWidth, int base, QChar fillChar);
  fn _ZNK7QString3argEsii5QChar(qthis: *mut c_void, arg0: c_short, arg1: c_int, arg2: c_int, arg3: *mut c_void) -> *mut c_void;
  // proto:  void QString::QString(const QByteArray & a);
  fn _ZN7QStringC1ERK10QByteArray(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  qulonglong QString::toULongLong(bool * ok, int base);
  fn _ZNK7QString11toULongLongEPbi(qthis: *mut c_void, arg0: *mut c_char, arg1: c_int) -> c_ulonglong;
  // proto:  QString & QString::append(const char * s);
  fn _ZN7QString6appendEPKc(qthis: *mut c_void, arg0: *mut c_char) -> *mut c_void;
  // proto:  int QString::capacity();
  fn _ZNK7QString8capacityEv(qthis: *mut c_void) -> c_int;
  // proto:  void QString::squeeze();
  fn _ZN7QString7squeezeEv(qthis: *mut c_void);
  // proto:  void QString::truncate(int pos);
  fn _ZN7QString8truncateEi(qthis: *mut c_void, arg0: c_int);
  // proto:  QString QString::arg(int a, int fieldWidth, int base, QChar fillChar);
  fn _ZNK7QString3argEiii5QChar(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: *mut c_void) -> *mut c_void;
  // proto:  QString QString::arg(QChar a, int fieldWidth, QChar fillChar);
  fn _ZNK7QString3argE5QChariS0_(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int, arg2: *mut c_void) -> *mut c_void;
  // proto:  int QString::localeAwareCompare(const QString & s);
  fn _ZNK7QString18localeAwareCompareERKS_(qthis: *mut c_void, arg0: *mut c_void) -> c_int;
  // proto:  QString & QString::remove(const QRegExp & rx);
  fn _ZN7QString6removeERK7QRegExp(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  bool QString::contains(const QRegularExpression & re);
  fn _ZNK7QString8containsERK18QRegularExpression(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  int QString::indexOf(QRegExp & , int from);
  fn _ZNK7QString7indexOfER7QRegExpi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) -> c_int;
  // proto:  QString & QString::replace(int i, int len, QChar after);
  fn _ZN7QString7replaceEii5QChar(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *mut c_void) -> *mut c_void;
  // proto:  bool QString::isRightToLeft();
  fn _ZNK7QString13isRightToLeftEv(qthis: *mut c_void) -> c_char;
  // proto:  QString QString::arg(char a, int fieldWidth, QChar fillChar);
  fn _ZNK7QString3argEci5QChar(qthis: *mut c_void, arg0: c_char, arg1: c_int, arg2: *mut c_void) -> *mut c_void;
  // proto:  QVector<uint> QString::toUcs4();
  fn _ZNK7QString6toUcs4Ev(qthis: *mut c_void);
  // proto:  QString & QString::remove(int i, int len);
  fn _ZN7QString6removeEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) -> *mut c_void;
  // proto:  int QString::lastIndexOf(const QRegularExpression & re, int from, QRegularExpressionMatch * rmatch);
  fn _ZNK7QString11lastIndexOfERK18QRegularExpressioniP23QRegularExpressionMatch(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int, arg2: *mut c_void) -> c_int;
  // proto:  QString QString::right(int n);
  fn _ZNK7QString5rightEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  QString QString::rightJustified(int width, QChar fill, bool trunc);
  fn _ZNK7QString14rightJustifiedEi5QCharb(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void, arg2: c_char) -> *mut c_void;
  // proto:  QString QString::arg(const QString & a, int fieldWidth, QChar fillChar);
  fn _ZNK7QString3argERKS_i5QChar(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int, arg2: *mut c_void) -> *mut c_void;
  // proto:  QString QString::arg(qulonglong a, int fieldwidth, int base, QChar fillChar);
  fn _ZNK7QString3argEyii5QChar(qthis: *mut c_void, arg0: c_ulonglong, arg1: c_int, arg2: c_int, arg3: *mut c_void) -> *mut c_void;
  // proto:  void QString::reserve(int size);
  fn _ZN7QString7reserveEi(qthis: *mut c_void, arg0: c_int);
  // proto:  short QString::toShort(bool * ok, int base);
  fn _ZNK7QString7toShortEPbi(qthis: *mut c_void, arg0: *mut c_char, arg1: c_int) -> c_short;
  // proto:  QString QString::arg(ulong a, int fieldwidth, int base, QChar fillChar);
  fn _ZNK7QString3argEmii5QChar(qthis: *mut c_void, arg0: c_ulong, arg1: c_int, arg2: c_int, arg3: *mut c_void) -> *mut c_void;
}

// body block begin
// class sizeof(QString)=8
pub struct QString {
  pub qclsinst: *mut c_void,
}

  // proto:  qlonglong QString::toLongLong(bool * ok, int base);
impl /*struct*/ QString {
  pub fn toLongLong<RetType, T: QString_toLongLong<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toLongLong(self);
    // return 1;
  }
}

pub trait QString_toLongLong<RetType> {
  fn toLongLong(self , rsthis: &mut QString) -> RetType;
}

  // proto:  qlonglong QString::toLongLong(bool * ok, int base);
impl<'a> /*trait*/ QString_toLongLong<i64> for (&'a mut Vec<i8>, i32) {
  fn toLongLong(self , rsthis: &mut QString) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString10toLongLongEPbi()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK7QString10toLongLongEPbi(rsthis.qclsinst, arg0, arg1)};
    return ret as i64;
    // return 1;
  }
}

  // proto:  bool QString::isNull();
impl /*struct*/ QString {
  pub fn isNull<RetType, T: QString_isNull<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isNull(self);
    // return 1;
  }
}

pub trait QString_isNull<RetType> {
  fn isNull(self , rsthis: &mut QString) -> RetType;
}

  // proto:  bool QString::isNull();
impl<'a> /*trait*/ QString_isNull<i8> for () {
  fn isNull(self , rsthis: &mut QString) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString6isNullEv()};
    let mut ret = unsafe {_ZNK7QString6isNullEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QString & QString::append(const QChar * uc, int len);
impl /*struct*/ QString {
  pub fn append<RetType, T: QString_append<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.append(self);
    // return 1;
  }
}

pub trait QString_append<RetType> {
  fn append(self , rsthis: &mut QString) -> RetType;
}

  // proto:  QString & QString::append(const QChar * uc, int len);
impl<'a> /*trait*/ QString_append<QString> for (QChar, i32) {
  fn append(self , rsthis: &mut QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString6appendEPK5QChari()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZN7QString6appendEPK5QChari(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QString & QString::prepend(QChar c);
impl /*struct*/ QString {
  pub fn prepend<RetType, T: QString_prepend<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.prepend(self);
    // return 1;
  }
}

pub trait QString_prepend<RetType> {
  fn prepend(self , rsthis: &mut QString) -> RetType;
}

  // proto:  QString & QString::prepend(QChar c);
impl<'a> /*trait*/ QString_prepend<QString> for (QChar) {
  fn prepend(self , rsthis: &mut QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString7prependE5QChar()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN7QString7prependE5QChar(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QString & QString::insert(int i, QChar c);
impl /*struct*/ QString {
  pub fn insert<RetType, T: QString_insert<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.insert(self);
    // return 1;
  }
}

pub trait QString_insert<RetType> {
  fn insert(self , rsthis: &mut QString) -> RetType;
}

  // proto:  QString & QString::insert(int i, QChar c);
impl<'a> /*trait*/ QString_insert<QString> for (i32, QChar) {
  fn insert(self , rsthis: &mut QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString6insertEi5QChar()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN7QString6insertEi5QChar(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QString QString::left(int n);
impl /*struct*/ QString {
  pub fn left<RetType, T: QString_left<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.left(self);
    // return 1;
  }
}

pub trait QString_left<RetType> {
  fn left(self , rsthis: &mut QString) -> RetType;
}

  // proto:  QString QString::left(int n);
impl<'a> /*trait*/ QString_left<QString> for (i32) {
  fn left(self , rsthis: &mut QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString4leftEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK7QString4leftEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QString::QString(QChar c);
impl /*struct*/ QString {
  pub fn NewQString<T: QString_NewQString>(value: T) -> QString {
    let rsthis = value.NewQString();
    return rsthis;
    // return 1;
  }
}

pub trait QString_NewQString {
  fn NewQString(self) -> QString;
}

  // proto:  void QString::QString(QChar c);
impl<'a> /*trait*/ QString_NewQString for (QChar) {
  fn NewQString(self) -> QString {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QStringC1E5QChar()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN7QStringC1E5QChar(qthis, arg0)};
    let rsthis = QString{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QString::isEmpty();
impl /*struct*/ QString {
  pub fn isEmpty<RetType, T: QString_isEmpty<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isEmpty(self);
    // return 1;
  }
}

pub trait QString_isEmpty<RetType> {
  fn isEmpty(self , rsthis: &mut QString) -> RetType;
}

  // proto:  bool QString::isEmpty();
impl<'a> /*trait*/ QString_isEmpty<i8> for () {
  fn isEmpty(self , rsthis: &mut QString) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString7isEmptyEv()};
    let mut ret = unsafe {_ZNK7QString7isEmptyEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QString & QString::prepend(const char * s);
impl<'a> /*trait*/ QString_prepend<QString> for (&'a  String) {
  fn prepend(self , rsthis: &mut QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString7prependEPKc()};
    let arg0 = self.as_ptr()  as *mut c_char;
    let mut ret = unsafe {_ZN7QString7prependEPKc(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  int QString::lastIndexOf(const QRegularExpression & re, int from);
impl /*struct*/ QString {
  pub fn lastIndexOf<RetType, T: QString_lastIndexOf<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.lastIndexOf(self);
    // return 1;
  }
}

pub trait QString_lastIndexOf<RetType> {
  fn lastIndexOf(self , rsthis: &mut QString) -> RetType;
}

  // proto:  int QString::lastIndexOf(const QRegularExpression & re, int from);
impl<'a> /*trait*/ QString_lastIndexOf<i32> for (QRegularExpression, i32) {
  fn lastIndexOf(self , rsthis: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString11lastIndexOfERK18QRegularExpressioni()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK7QString11lastIndexOfERK18QRegularExpressioni(rsthis.qclsinst, arg0, arg1)};
    return ret as i32;
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
    let mut ret = unsafe {_ZN7QString6numberEii(arg0, arg1)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QString::resize(int size);
impl /*struct*/ QString {
  pub fn resize<RetType, T: QString_resize<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.resize(self);
    // return 1;
  }
}

pub trait QString_resize<RetType> {
  fn resize(self , rsthis: &mut QString) -> RetType;
}

  // proto:  void QString::resize(int size);
impl<'a> /*trait*/ QString_resize<()> for (i32) {
  fn resize(self , rsthis: &mut QString) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString6resizeEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN7QString6resizeEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QString::push_front(QChar c);
impl /*struct*/ QString {
  pub fn push_front<RetType, T: QString_push_front<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.push_front(self);
    // return 1;
  }
}

pub trait QString_push_front<RetType> {
  fn push_front(self , rsthis: &mut QString) -> RetType;
}

  // proto:  void QString::push_front(QChar c);
impl<'a> /*trait*/ QString_push_front<()> for (QChar) {
  fn push_front(self , rsthis: &mut QString) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString10push_frontE5QChar()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QString10push_frontE5QChar(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QString::QString();
impl<'a> /*trait*/ QString_NewQString for () {
  fn NewQString(self) -> QString {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QStringC1Ev()};
    unsafe {_ZN7QStringC1Ev(qthis)};
    let rsthis = QString{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  double QString::toDouble(bool * ok);
impl /*struct*/ QString {
  pub fn toDouble<RetType, T: QString_toDouble<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toDouble(self);
    // return 1;
  }
}

pub trait QString_toDouble<RetType> {
  fn toDouble(self , rsthis: &mut QString) -> RetType;
}

  // proto:  double QString::toDouble(bool * ok);
impl<'a> /*trait*/ QString_toDouble<f64> for (&'a mut Vec<i8>) {
  fn toDouble(self , rsthis: &mut QString) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString8toDoubleEPb()};
    let arg0 = self.as_ptr()  as *mut c_char;
    let mut ret = unsafe {_ZNK7QString8toDoubleEPb(rsthis.qclsinst, arg0)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  QString QString::arg(const QString & a1, const QString & a2, const QString & a3, const QString & a4, const QString & a5, const QString & a6, const QString & a7, const QString & a8);
impl /*struct*/ QString {
  pub fn arg<RetType, T: QString_arg<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.arg(self);
    // return 1;
  }
}

pub trait QString_arg<RetType> {
  fn arg(self , rsthis: &mut QString) -> RetType;
}

  // proto:  QString QString::arg(const QString & a1, const QString & a2, const QString & a3, const QString & a4, const QString & a5, const QString & a6, const QString & a7, const QString & a8);
impl<'a> /*trait*/ QString_arg<QString> for (QString, QString, QString, QString, QString, QString, QString, QString) {
  fn arg(self , rsthis: &mut QString) -> QString {
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
    let mut ret = unsafe {_ZNK7QString3argERKS_S1_S1_S1_S1_S1_S1_S1_(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QStringRef QString::rightRef(int n);
impl /*struct*/ QString {
  pub fn rightRef<RetType, T: QString_rightRef<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.rightRef(self);
    // return 1;
  }
}

pub trait QString_rightRef<RetType> {
  fn rightRef(self , rsthis: &mut QString) -> RetType;
}

  // proto:  QStringRef QString::rightRef(int n);
impl<'a> /*trait*/ QString_rightRef<()> for (i32) {
  fn rightRef(self , rsthis: &mut QString) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString8rightRefEi()};
    let arg0 = self  as c_int;
     unsafe {_ZNK7QString8rightRefEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QString & QString::setNum(short , int base);
impl /*struct*/ QString {
  pub fn setNum<RetType, T: QString_setNum<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setNum(self);
    // return 1;
  }
}

pub trait QString_setNum<RetType> {
  fn setNum(self , rsthis: &mut QString) -> RetType;
}

  // proto:  QString & QString::setNum(short , int base);
impl<'a> /*trait*/ QString_setNum<QString> for (i16, i32) {
  fn setNum(self , rsthis: &mut QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString6setNumEsi()};
    let arg0 = self.0  as c_short;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZN7QString6setNumEsi(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QString::QString(const QChar * unicode, int size);
impl<'a> /*trait*/ QString_NewQString for (QChar, i32) {
  fn NewQString(self) -> QString {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QStringC1EPK5QChari()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    unsafe {_ZN7QStringC1EPK5QChari(qthis, arg0, arg1)};
    let rsthis = QString{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  float QString::toFloat(bool * ok);
impl /*struct*/ QString {
  pub fn toFloat<RetType, T: QString_toFloat<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toFloat(self);
    // return 1;
  }
}

pub trait QString_toFloat<RetType> {
  fn toFloat(self , rsthis: &mut QString) -> RetType;
}

  // proto:  float QString::toFloat(bool * ok);
impl<'a> /*trait*/ QString_toFloat<f32> for (&'a mut Vec<i8>) {
  fn toFloat(self , rsthis: &mut QString) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString7toFloatEPb()};
    let arg0 = self.as_ptr()  as *mut c_char;
    let mut ret = unsafe {_ZNK7QString7toFloatEPb(rsthis.qclsinst, arg0)};
    return ret as f32;
    // return 1;
  }
}

  // proto:  int QString::count(const QRegularExpression & re);
impl /*struct*/ QString {
  pub fn count<RetType, T: QString_count<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.count(self);
    // return 1;
  }
}

pub trait QString_count<RetType> {
  fn count(self , rsthis: &mut QString) -> RetType;
}

  // proto:  int QString::count(const QRegularExpression & re);
impl<'a> /*trait*/ QString_count<i32> for (QRegularExpression) {
  fn count(self , rsthis: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString5countERK18QRegularExpression()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QString5countERK18QRegularExpression(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QStringRef QString::midRef(int position, int n);
impl /*struct*/ QString {
  pub fn midRef<RetType, T: QString_midRef<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.midRef(self);
    // return 1;
  }
}

pub trait QString_midRef<RetType> {
  fn midRef(self , rsthis: &mut QString) -> RetType;
}

  // proto:  QStringRef QString::midRef(int position, int n);
impl<'a> /*trait*/ QString_midRef<()> for (i32, i32) {
  fn midRef(self , rsthis: &mut QString) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString6midRefEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZNK7QString6midRefEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QString::detach();
impl /*struct*/ QString {
  pub fn detach<RetType, T: QString_detach<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.detach(self);
    // return 1;
  }
}

pub trait QString_detach<RetType> {
  fn detach(self , rsthis: &mut QString) -> RetType;
}

  // proto:  void QString::detach();
impl<'a> /*trait*/ QString_detach<()> for () {
  fn detach(self , rsthis: &mut QString) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString6detachEv()};
     unsafe {_ZN7QString6detachEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QString QString::arg(const QString & a1, const QString & a2, const QString & a3, const QString & a4);
impl<'a> /*trait*/ QString_arg<QString> for (QString, QString, QString, QString) {
  fn arg(self , rsthis: &mut QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString3argERKS_S1_S1_S1_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let arg3 = self.3.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QString3argERKS_S1_S1_S1_(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QString QString::arg(long a, int fieldwidth, int base, QChar fillChar);
impl<'a> /*trait*/ QString_arg<QString> for (i64, i32, i32, QChar) {
  fn arg(self , rsthis: &mut QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString3argElii5QChar()};
    let arg0 = self.0  as c_long;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QString3argElii5QChar(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  int QString::count();
impl<'a> /*trait*/ QString_count<i32> for () {
  fn count(self , rsthis: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString5countEv()};
    let mut ret = unsafe {_ZNK7QString5countEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QString & QString::setNum(qulonglong , int base);
impl<'a> /*trait*/ QString_setNum<QString> for (u64, i32) {
  fn setNum(self , rsthis: &mut QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString6setNumEyi()};
    let arg0 = self.0  as c_ulonglong;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZN7QString6setNumEyi(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto: static QString QString::fromStdWString(const std::wstring & s);
impl /*struct*/ QString {
  pub fn fromStdWString_s<RetType, T: QString_fromStdWString_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromStdWString_s();
    // return 1;
  }
}

pub trait QString_fromStdWString_s<RetType> {
  fn fromStdWString_s(self ) -> RetType;
}

  // proto: static QString QString::fromStdWString(const std::wstring & s);
impl<'a> /*trait*/ QString_fromStdWString_s<QString> for (&'a  Vec<i32>) {
  fn fromStdWString_s(self ) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString14fromStdWStringERKi()};
    let arg0 = self.as_ptr()  as *mut c_int;
    let mut ret = unsafe {_ZN7QString14fromStdWStringERKi(arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QString::push_back(QChar c);
impl /*struct*/ QString {
  pub fn push_back<RetType, T: QString_push_back<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.push_back(self);
    // return 1;
  }
}

pub trait QString_push_back<RetType> {
  fn push_back(self , rsthis: &mut QString) -> RetType;
}

  // proto:  void QString::push_back(QChar c);
impl<'a> /*trait*/ QString_push_back<()> for (QChar) {
  fn push_back(self , rsthis: &mut QString) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString9push_backE5QChar()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QString9push_backE5QChar(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QString & QString::setNum(float , char f, int prec);
impl<'a> /*trait*/ QString_setNum<QString> for (f32, i8, i32) {
  fn setNum(self , rsthis: &mut QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString6setNumEfci()};
    let arg0 = self.0  as c_float;
    let arg1 = self.1  as c_char;
    let arg2 = self.2  as c_int;
    let mut ret = unsafe {_ZN7QString6setNumEfci(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  int QString::count(const QRegExp & );
impl<'a> /*trait*/ QString_count<i32> for (QRegExp) {
  fn count(self , rsthis: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString5countERK7QRegExp()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QString5countERK7QRegExp(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QString::size();
impl /*struct*/ QString {
  pub fn size<RetType, T: QString_size<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.size(self);
    // return 1;
  }
}

pub trait QString_size<RetType> {
  fn size(self , rsthis: &mut QString) -> RetType;
}

  // proto:  int QString::size();
impl<'a> /*trait*/ QString_size<i32> for () {
  fn size(self , rsthis: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString4sizeEv()};
    let mut ret = unsafe {_ZNK7QString4sizeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QString & QString::insert(int i, const QChar * uc, int len);
impl<'a> /*trait*/ QString_insert<QString> for (i32, QChar, i32) {
  fn insert(self , rsthis: &mut QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString6insertEiPK5QChari()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_int;
    let mut ret = unsafe {_ZN7QString6insertEiPK5QChari(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QString & QString::replace(int i, int len, const QChar * s, int slen);
impl /*struct*/ QString {
  pub fn replace<RetType, T: QString_replace<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.replace(self);
    // return 1;
  }
}

pub trait QString_replace<RetType> {
  fn replace(self , rsthis: &mut QString) -> RetType;
}

  // proto:  QString & QString::replace(int i, int len, const QChar * s, int slen);
impl<'a> /*trait*/ QString_replace<QString> for (i32, i32, QChar, i32) {
  fn replace(self , rsthis: &mut QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString7replaceEiiPK5QChari()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let arg3 = self.3  as c_int;
    let mut ret = unsafe {_ZN7QString7replaceEiiPK5QChari(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    let mut ret1 = QString{qclsinst: ret};
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
impl<'a> /*trait*/ QString_fromRawData_s<QString> for (QChar, i32) {
  fn fromRawData_s(self ) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString11fromRawDataEPK5QChari()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZN7QString11fromRawDataEPK5QChari(arg0, arg1)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QString & QString::insert(int i, const QString & s);
impl<'a> /*trait*/ QString_insert<QString> for (i32, QString) {
  fn insert(self , rsthis: &mut QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString6insertEiRKS_()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN7QString6insertEiRKS_(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QString QString::arg(const QString & a1, const QString & a2, const QString & a3, const QString & a4, const QString & a5);
impl<'a> /*trait*/ QString_arg<QString> for (QString, QString, QString, QString, QString) {
  fn arg(self , rsthis: &mut QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString3argERKS_S1_S1_S1_S1_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let arg3 = self.3.qclsinst  as *mut c_void;
    let arg4 = self.4.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QString3argERKS_S1_S1_S1_S1_(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QString & QString::setRawData(const QChar * unicode, int size);
impl /*struct*/ QString {
  pub fn setRawData<RetType, T: QString_setRawData<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setRawData(self);
    // return 1;
  }
}

pub trait QString_setRawData<RetType> {
  fn setRawData(self , rsthis: &mut QString) -> RetType;
}

  // proto:  QString & QString::setRawData(const QChar * unicode, int size);
impl<'a> /*trait*/ QString_setRawData<QString> for (QChar, i32) {
  fn setRawData(self , rsthis: &mut QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString10setRawDataEPK5QChari()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZN7QString10setRawDataEPK5QChari(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QString & QString::prepend(const QString & s);
impl<'a> /*trait*/ QString_prepend<QString> for (QString) {
  fn prepend(self , rsthis: &mut QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString7prependERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN7QString7prependERKS_(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QString & QString::setNum(int , int base);
impl<'a> /*trait*/ QString_setNum<QString> for (i32, i32) {
  fn setNum(self , rsthis: &mut QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString6setNumEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZN7QString6setNumEii(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  ulong QString::toULong(bool * ok, int base);
impl /*struct*/ QString {
  pub fn toULong<RetType, T: QString_toULong<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toULong(self);
    // return 1;
  }
}

pub trait QString_toULong<RetType> {
  fn toULong(self , rsthis: &mut QString) -> RetType;
}

  // proto:  ulong QString::toULong(bool * ok, int base);
impl<'a> /*trait*/ QString_toULong<u64> for (&'a mut Vec<i8>, i32) {
  fn toULong(self , rsthis: &mut QString) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString7toULongEPbi()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK7QString7toULongEPbi(rsthis.qclsinst, arg0, arg1)};
    return ret as u64;
    // return 1;
  }
}

  // proto:  void QString::chop(int n);
impl /*struct*/ QString {
  pub fn chop<RetType, T: QString_chop<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.chop(self);
    // return 1;
  }
}

pub trait QString_chop<RetType> {
  fn chop(self , rsthis: &mut QString) -> RetType;
}

  // proto:  void QString::chop(int n);
impl<'a> /*trait*/ QString_chop<()> for (i32) {
  fn chop(self , rsthis: &mut QString) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString4chopEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN7QString4chopEi(rsthis.qclsinst, arg0)};
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
    let mut ret = unsafe {_ZN7QString9fromUtf16EPKti(arg0, arg1)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  bool QString::isDetached();
impl /*struct*/ QString {
  pub fn isDetached<RetType, T: QString_isDetached<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isDetached(self);
    // return 1;
  }
}

pub trait QString_isDetached<RetType> {
  fn isDetached(self , rsthis: &mut QString) -> RetType;
}

  // proto:  bool QString::isDetached();
impl<'a> /*trait*/ QString_isDetached<i8> for () {
  fn isDetached(self , rsthis: &mut QString) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString10isDetachedEv()};
    let mut ret = unsafe {_ZNK7QString10isDetachedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QString & QString::setNum(qlonglong , int base);
impl<'a> /*trait*/ QString_setNum<QString> for (i64, i32) {
  fn setNum(self , rsthis: &mut QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString6setNumExi()};
    let arg0 = self.0  as c_longlong;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZN7QString6setNumExi(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QString QString::mid(int position, int n);
impl /*struct*/ QString {
  pub fn mid<RetType, T: QString_mid<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.mid(self);
    // return 1;
  }
}

pub trait QString_mid<RetType> {
  fn mid(self , rsthis: &mut QString) -> RetType;
}

  // proto:  QString QString::mid(int position, int n);
impl<'a> /*trait*/ QString_mid<QString> for (i32, i32) {
  fn mid(self , rsthis: &mut QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString3midEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK7QString3midEii(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QString{qclsinst: ret};
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
    let mut ret = unsafe {_ZN7QString13fromLocal8BitEPKci(arg0, arg1)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QString::swap(QString & other);
impl /*struct*/ QString {
  pub fn swap<RetType, T: QString_swap<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.swap(self);
    // return 1;
  }
}

pub trait QString_swap<RetType> {
  fn swap(self , rsthis: &mut QString) -> RetType;
}

  // proto:  void QString::swap(QString & other);
impl<'a> /*trait*/ QString_swap<()> for (QString) {
  fn swap(self , rsthis: &mut QString) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QString4swapERS_(rsthis.qclsinst, arg0)};
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
impl<'a> /*trait*/ QString_fromUtf8_s<QString> for (QByteArray) {
  fn fromUtf8_s(self ) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString8fromUtf8ERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN7QString8fromUtf8ERK10QByteArray(arg0)};
    let mut ret1 = QString{qclsinst: ret};
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
    let mut ret = unsafe {_ZN7QString8fromUcs4EPKDii(arg0, arg1)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QString QString::leftJustified(int width, QChar fill, bool trunc);
impl /*struct*/ QString {
  pub fn leftJustified<RetType, T: QString_leftJustified<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.leftJustified(self);
    // return 1;
  }
}

pub trait QString_leftJustified<RetType> {
  fn leftJustified(self , rsthis: &mut QString) -> RetType;
}

  // proto:  QString QString::leftJustified(int width, QChar fill, bool trunc);
impl<'a> /*trait*/ QString_leftJustified<QString> for (i32, QChar, i8) {
  fn leftJustified(self , rsthis: &mut QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString13leftJustifiedEi5QCharb()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_char;
    let mut ret = unsafe {_ZNK7QString13leftJustifiedEi5QCharb(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  int QString::indexOf(const QRegExp & , int from);
impl /*struct*/ QString {
  pub fn indexOf<RetType, T: QString_indexOf<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.indexOf(self);
    // return 1;
  }
}

pub trait QString_indexOf<RetType> {
  fn indexOf(self , rsthis: &mut QString) -> RetType;
}

  // proto:  int QString::indexOf(const QRegExp & , int from);
impl<'a> /*trait*/ QString_indexOf<i32> for (QRegExp, i32) {
  fn indexOf(self , rsthis: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString7indexOfERK7QRegExpi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK7QString7indexOfERK7QRegExpi(rsthis.qclsinst, arg0, arg1)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QString::push_back(const QString & s);
impl<'a> /*trait*/ QString_push_back<()> for (QString) {
  fn push_back(self , rsthis: &mut QString) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString9push_backERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QString9push_backERKS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QString::lastIndexOf(QRegExp & , int from);
impl<'a> /*trait*/ QString_lastIndexOf<i32> for (QRegExp, i32) {
  fn lastIndexOf(self , rsthis: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString11lastIndexOfER7QRegExpi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK7QString11lastIndexOfER7QRegExpi(rsthis.qclsinst, arg0, arg1)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QString QString::arg(const QString & a1, const QString & a2, const QString & a3);
impl<'a> /*trait*/ QString_arg<QString> for (QString, QString, QString) {
  fn arg(self , rsthis: &mut QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString3argERKS_S1_S1_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QString3argERKS_S1_S1_(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  const ushort * QString::utf16();
impl /*struct*/ QString {
  pub fn utf16<RetType, T: QString_utf16<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.utf16(self);
    // return 1;
  }
}

pub trait QString_utf16<RetType> {
  fn utf16(self , rsthis: &mut QString) -> RetType;
}

  // proto:  const ushort * QString::utf16();
impl<'a> /*trait*/ QString_utf16<()> for () {
  fn utf16(self , rsthis: &mut QString) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString5utf16Ev()};
     unsafe {_ZNK7QString5utf16Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QString::toInt(bool * ok, int base);
impl /*struct*/ QString {
  pub fn toInt<RetType, T: QString_toInt<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toInt(self);
    // return 1;
  }
}

pub trait QString_toInt<RetType> {
  fn toInt(self , rsthis: &mut QString) -> RetType;
}

  // proto:  int QString::toInt(bool * ok, int base);
impl<'a> /*trait*/ QString_toInt<i32> for (&'a mut Vec<i8>, i32) {
  fn toInt(self , rsthis: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString5toIntEPbi()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK7QString5toIntEPbi(rsthis.qclsinst, arg0, arg1)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QString QString::arg(double a, int fieldWidth, char fmt, int prec, QChar fillChar);
impl<'a> /*trait*/ QString_arg<QString> for (f64, i32, i8, i32, QChar) {
  fn arg(self , rsthis: &mut QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString3argEdici5QChar()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_char;
    let arg3 = self.3  as c_int;
    let arg4 = self.4.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QString3argEdici5QChar(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QChar * QString::data();
impl /*struct*/ QString {
  pub fn data<RetType, T: QString_data<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.data(self);
    // return 1;
  }
}

pub trait QString_data<RetType> {
  fn data(self , rsthis: &mut QString) -> RetType;
}

  // proto:  QChar * QString::data();
impl<'a> /*trait*/ QString_data<QChar> for () {
  fn data(self , rsthis: &mut QString) -> QChar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString4dataEv()};
    let mut ret = unsafe {_ZN7QString4dataEv(rsthis.qclsinst)};
    let mut ret1 = QChar{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QString & QString::setNum(uint , int base);
impl<'a> /*trait*/ QString_setNum<QString> for (u32, i32) {
  fn setNum(self , rsthis: &mut QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString6setNumEji()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZN7QString6setNumEji(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QString{qclsinst: ret};
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
impl<'a> /*trait*/ QString_localeAwareCompare_s<i32> for (QString, QString) {
  fn localeAwareCompare_s(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString18localeAwareCompareERKS_S1_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN7QString18localeAwareCompareERKS_S1_(arg0, arg1)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QString::QString(const char * ch);
impl<'a> /*trait*/ QString_NewQString for (&'a  String) {
  fn NewQString(self) -> QString {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QStringC1EPKc()};
    let arg0 = self.as_ptr()  as *mut c_char;
    unsafe {_ZN7QStringC1EPKc(qthis, arg0)};
    let rsthis = QString{qclsinst: qthis};
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
    let mut ret = unsafe {_ZN7QString9fromUtf16EPKDsi(arg0, arg1)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QString & QString::replace(const QRegExp & rx, const QString & after);
impl<'a> /*trait*/ QString_replace<QString> for (QRegExp, QString) {
  fn replace(self , rsthis: &mut QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString7replaceERK7QRegExpRKS_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN7QString7replaceERK7QRegExpRKS_(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QString QString::repeated(int times);
impl /*struct*/ QString {
  pub fn repeated<RetType, T: QString_repeated<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.repeated(self);
    // return 1;
  }
}

pub trait QString_repeated<RetType> {
  fn repeated(self , rsthis: &mut QString) -> RetType;
}

  // proto:  QString QString::repeated(int times);
impl<'a> /*trait*/ QString_repeated<QString> for (i32) {
  fn repeated(self , rsthis: &mut QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString8repeatedEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK7QString8repeatedEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QString & QString::setUtf16(const ushort * utf16, int size);
impl /*struct*/ QString {
  pub fn setUtf16<RetType, T: QString_setUtf16<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setUtf16(self);
    // return 1;
  }
}

pub trait QString_setUtf16<RetType> {
  fn setUtf16(self , rsthis: &mut QString) -> RetType;
}

  // proto:  QString & QString::setUtf16(const ushort * utf16, int size);
impl<'a> /*trait*/ QString_setUtf16<QString> for (&'a  Vec<u16>, i32) {
  fn setUtf16(self , rsthis: &mut QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString8setUtf16EPKti()};
    let arg0 = self.0.as_ptr()  as *mut c_ushort;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZN7QString8setUtf16EPKti(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto: static QString QString::fromStdU32String(const std::u32string & s);
impl /*struct*/ QString {
  pub fn fromStdU32String_s<RetType, T: QString_fromStdU32String_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromStdU32String_s();
    // return 1;
  }
}

pub trait QString_fromStdU32String_s<RetType> {
  fn fromStdU32String_s(self ) -> RetType;
}

  // proto: static QString QString::fromStdU32String(const std::u32string & s);
impl<'a> /*trait*/ QString_fromStdU32String_s<QString> for (&'a  Vec<i32>) {
  fn fromStdU32String_s(self ) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString16fromStdU32StringERKi()};
    let arg0 = self.as_ptr()  as *mut c_int;
    let mut ret = unsafe {_ZN7QString16fromStdU32StringERKi(arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QString::clear();
impl /*struct*/ QString {
  pub fn clear<RetType, T: QString_clear<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.clear(self);
    // return 1;
  }
}

pub trait QString_clear<RetType> {
  fn clear(self , rsthis: &mut QString) -> RetType;
}

  // proto:  void QString::clear();
impl<'a> /*trait*/ QString_clear<()> for () {
  fn clear(self , rsthis: &mut QString) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString5clearEv()};
     unsafe {_ZN7QString5clearEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QString::contains(const QRegExp & rx);
impl /*struct*/ QString {
  pub fn contains<RetType, T: QString_contains<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.contains(self);
    // return 1;
  }
}

pub trait QString_contains<RetType> {
  fn contains(self , rsthis: &mut QString) -> RetType;
}

  // proto:  bool QString::contains(const QRegExp & rx);
impl<'a> /*trait*/ QString_contains<i8> for (QRegExp) {
  fn contains(self , rsthis: &mut QString) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString8containsERK7QRegExp()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QString8containsERK7QRegExp(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QString::isSharedWith(const QString & other);
impl /*struct*/ QString {
  pub fn isSharedWith<RetType, T: QString_isSharedWith<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isSharedWith(self);
    // return 1;
  }
}

pub trait QString_isSharedWith<RetType> {
  fn isSharedWith(self , rsthis: &mut QString) -> RetType;
}

  // proto:  bool QString::isSharedWith(const QString & other);
impl<'a> /*trait*/ QString_isSharedWith<i8> for (QString) {
  fn isSharedWith(self , rsthis: &mut QString) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString12isSharedWithERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QString12isSharedWithERKS_(rsthis.qclsinst, arg0)};
    return ret as i8;
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
impl<'a> /*trait*/ QString_fromLatin1_s<QString> for (QByteArray) {
  fn fromLatin1_s(self ) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString10fromLatin1ERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN7QString10fromLatin1ERK10QByteArray(arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QString::~QString();
impl /*struct*/ QString {
  pub fn FreeQString<RetType, T: QString_FreeQString<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQString(self);
    // return 1;
  }
}

pub trait QString_FreeQString<RetType> {
  fn FreeQString(self , rsthis: &mut QString) -> RetType;
}

  // proto:  void QString::~QString();
impl<'a> /*trait*/ QString_FreeQString<()> for () {
  fn FreeQString(self , rsthis: &mut QString) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QStringD0Ev()};
     unsafe {_ZN7QStringD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QString & QString::remove(const QRegularExpression & re);
impl /*struct*/ QString {
  pub fn remove<RetType, T: QString_remove<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.remove(self);
    // return 1;
  }
}

pub trait QString_remove<RetType> {
  fn remove(self , rsthis: &mut QString) -> RetType;
}

  // proto:  QString & QString::remove(const QRegularExpression & re);
impl<'a> /*trait*/ QString_remove<QString> for (QRegularExpression) {
  fn remove(self , rsthis: &mut QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString6removeERK18QRegularExpression()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN7QString6removeERK18QRegularExpression(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  const_iterator QString::cend();
impl /*struct*/ QString {
  pub fn cend<RetType, T: QString_cend<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.cend(self);
    // return 1;
  }
}

pub trait QString_cend<RetType> {
  fn cend(self , rsthis: &mut QString) -> RetType;
}

  // proto:  const_iterator QString::cend();
impl<'a> /*trait*/ QString_cend<QChar> for () {
  fn cend(self , rsthis: &mut QString) -> QChar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString4cendEv()};
    let mut ret = unsafe {_ZNK7QString4cendEv(rsthis.qclsinst)};
    let mut ret1 = QChar{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QString QString::toHtmlEscaped();
impl /*struct*/ QString {
  pub fn toHtmlEscaped<RetType, T: QString_toHtmlEscaped<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toHtmlEscaped(self);
    // return 1;
  }
}

pub trait QString_toHtmlEscaped<RetType> {
  fn toHtmlEscaped(self , rsthis: &mut QString) -> RetType;
}

  // proto:  QString QString::toHtmlEscaped();
impl<'a> /*trait*/ QString_toHtmlEscaped<QString> for () {
  fn toHtmlEscaped(self , rsthis: &mut QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString13toHtmlEscapedEv()};
    let mut ret = unsafe {_ZNK7QString13toHtmlEscapedEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QString & QString::append(const QByteArray & s);
impl<'a> /*trait*/ QString_append<QString> for (QByteArray) {
  fn append(self , rsthis: &mut QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString6appendERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN7QString6appendERK10QByteArray(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
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
    let mut ret = unsafe {_ZN7QString10fromLatin1EPKci(arg0, arg1)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  bool QString::contains(const QRegularExpression & re, QRegularExpressionMatch * match);
impl<'a> /*trait*/ QString_contains<i8> for (QRegularExpression, QRegularExpressionMatch) {
  fn contains(self , rsthis: &mut QString) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString8containsERK18QRegularExpressionP23QRegularExpressionMatch()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QString8containsERK18QRegularExpressionP23QRegularExpressionMatch(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  int QString::indexOf(const QRegularExpression & re, int from, QRegularExpressionMatch * rmatch);
impl<'a> /*trait*/ QString_indexOf<i32> for (QRegularExpression, i32, QRegularExpressionMatch) {
  fn indexOf(self , rsthis: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString7indexOfERK18QRegularExpressioniP23QRegularExpressionMatch()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QString7indexOfERK18QRegularExpressioniP23QRegularExpressionMatch(rsthis.qclsinst, arg0, arg1, arg2)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QString::toWCharArray(wchar_t * array);
impl /*struct*/ QString {
  pub fn toWCharArray<RetType, T: QString_toWCharArray<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toWCharArray(self);
    // return 1;
  }
}

pub trait QString_toWCharArray<RetType> {
  fn toWCharArray(self , rsthis: &mut QString) -> RetType;
}

  // proto:  int QString::toWCharArray(wchar_t * array);
impl<'a> /*trait*/ QString_toWCharArray<i32> for (&'a  String) {
  fn toWCharArray(self , rsthis: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString12toWCharArrayEPw()};
    let arg0 = self.as_ptr()  as *mut wchar_t;
    let mut ret = unsafe {_ZNK7QString12toWCharArrayEPw(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  const_iterator QString::cbegin();
impl /*struct*/ QString {
  pub fn cbegin<RetType, T: QString_cbegin<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.cbegin(self);
    // return 1;
  }
}

pub trait QString_cbegin<RetType> {
  fn cbegin(self , rsthis: &mut QString) -> RetType;
}

  // proto:  const_iterator QString::cbegin();
impl<'a> /*trait*/ QString_cbegin<QChar> for () {
  fn cbegin(self , rsthis: &mut QString) -> QChar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString6cbeginEv()};
    let mut ret = unsafe {_ZNK7QString6cbeginEv(rsthis.qclsinst)};
    let mut ret1 = QChar{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QString & QString::prepend(const QByteArray & s);
impl<'a> /*trait*/ QString_prepend<QString> for (QByteArray) {
  fn prepend(self , rsthis: &mut QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString7prependERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN7QString7prependERK10QByteArray(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QString & QString::replace(int i, int len, const QString & after);
impl<'a> /*trait*/ QString_replace<QString> for (i32, i32, QString) {
  fn replace(self , rsthis: &mut QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString7replaceEiiRKS_()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN7QString7replaceEiiRKS_(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto: static QString QString::fromStdString(const std::string & s);
impl /*struct*/ QString {
  pub fn fromStdString_s<RetType, T: QString_fromStdString_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromStdString_s();
    // return 1;
  }
}

pub trait QString_fromStdString_s<RetType> {
  fn fromStdString_s(self ) -> RetType;
}

  // proto: static QString QString::fromStdString(const std::string & s);
impl<'a> /*trait*/ QString_fromStdString_s<QString> for (&'a  Vec<i32>) {
  fn fromStdString_s(self ) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString13fromStdStringERKi()};
    let arg0 = self.as_ptr()  as *mut c_int;
    let mut ret = unsafe {_ZN7QString13fromStdStringERKi(arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QString QString::arg(const QString & a1, const QString & a2, const QString & a3, const QString & a4, const QString & a5, const QString & a6, const QString & a7);
impl<'a> /*trait*/ QString_arg<QString> for (QString, QString, QString, QString, QString, QString, QString) {
  fn arg(self , rsthis: &mut QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString3argERKS_S1_S1_S1_S1_S1_S1_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let arg3 = self.3.qclsinst  as *mut c_void;
    let arg4 = self.4.qclsinst  as *mut c_void;
    let arg5 = self.5.qclsinst  as *mut c_void;
    let arg6 = self.6.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QString3argERKS_S1_S1_S1_S1_S1_S1_(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4, arg5, arg6)};
    let mut ret1 = QString{qclsinst: ret};
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
    let mut ret = unsafe {_ZN7QString14fromWCharArrayEPKwi(arg0, arg1)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QString & QString::fill(QChar c, int size);
impl /*struct*/ QString {
  pub fn fill<RetType, T: QString_fill<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.fill(self);
    // return 1;
  }
}

pub trait QString_fill<RetType> {
  fn fill(self , rsthis: &mut QString) -> RetType;
}

  // proto:  QString & QString::fill(QChar c, int size);
impl<'a> /*trait*/ QString_fill<QString> for (QChar, i32) {
  fn fill(self , rsthis: &mut QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString4fillE5QChari()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZN7QString4fillE5QChari(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  const QChar * QString::constData();
impl /*struct*/ QString {
  pub fn constData<RetType, T: QString_constData<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.constData(self);
    // return 1;
  }
}

pub trait QString_constData<RetType> {
  fn constData(self , rsthis: &mut QString) -> RetType;
}

  // proto:  const QChar * QString::constData();
impl<'a> /*trait*/ QString_constData<QChar> for () {
  fn constData(self , rsthis: &mut QString) -> QChar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString9constDataEv()};
    let mut ret = unsafe {_ZNK7QString9constDataEv(rsthis.qclsinst)};
    let mut ret1 = QChar{qclsinst: ret};
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
    let mut ret = unsafe {_ZN7QString6numberEmi(arg0, arg1)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  long QString::toLong(bool * ok, int base);
impl /*struct*/ QString {
  pub fn toLong<RetType, T: QString_toLong<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toLong(self);
    // return 1;
  }
}

pub trait QString_toLong<RetType> {
  fn toLong(self , rsthis: &mut QString) -> RetType;
}

  // proto:  long QString::toLong(bool * ok, int base);
impl<'a> /*trait*/ QString_toLong<i64> for (&'a mut Vec<i8>, i32) {
  fn toLong(self , rsthis: &mut QString) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString6toLongEPbi()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK7QString6toLongEPbi(rsthis.qclsinst, arg0, arg1)};
    return ret as i64;
    // return 1;
  }
}

  // proto:  const_iterator QString::constEnd();
impl /*struct*/ QString {
  pub fn constEnd<RetType, T: QString_constEnd<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.constEnd(self);
    // return 1;
  }
}

pub trait QString_constEnd<RetType> {
  fn constEnd(self , rsthis: &mut QString) -> RetType;
}

  // proto:  const_iterator QString::constEnd();
impl<'a> /*trait*/ QString_constEnd<QChar> for () {
  fn constEnd(self , rsthis: &mut QString) -> QChar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString8constEndEv()};
    let mut ret = unsafe {_ZNK7QString8constEndEv(rsthis.qclsinst)};
    let mut ret1 = QChar{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  int QString::length();
impl /*struct*/ QString {
  pub fn length<RetType, T: QString_length<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.length(self);
    // return 1;
  }
}

pub trait QString_length<RetType> {
  fn length(self , rsthis: &mut QString) -> RetType;
}

  // proto:  int QString::length();
impl<'a> /*trait*/ QString_length<i32> for () {
  fn length(self , rsthis: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString6lengthEv()};
    let mut ret = unsafe {_ZNK7QString6lengthEv(rsthis.qclsinst)};
    return ret as i32;
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
    let mut ret = unsafe {_ZN7QString8fromUtf8EPKci(arg0, arg1)};
    let mut ret1 = QString{qclsinst: ret};
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
    let mut ret = unsafe {_ZN7QString6numberExi(arg0, arg1)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QStringRef QString::leftRef(int n);
impl /*struct*/ QString {
  pub fn leftRef<RetType, T: QString_leftRef<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.leftRef(self);
    // return 1;
  }
}

pub trait QString_leftRef<RetType> {
  fn leftRef(self , rsthis: &mut QString) -> RetType;
}

  // proto:  QStringRef QString::leftRef(int n);
impl<'a> /*trait*/ QString_leftRef<()> for (i32) {
  fn leftRef(self , rsthis: &mut QString) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString7leftRefEi()};
    let arg0 = self  as c_int;
     unsafe {_ZNK7QString7leftRefEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QString QString::arg(const QString & a1, const QString & a2);
impl<'a> /*trait*/ QString_arg<QString> for (QString, QString) {
  fn arg(self , rsthis: &mut QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString3argERKS_S1_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QString3argERKS_S1_(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  bool QString::isSimpleText();
impl /*struct*/ QString {
  pub fn isSimpleText<RetType, T: QString_isSimpleText<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isSimpleText(self);
    // return 1;
  }
}

pub trait QString_isSimpleText<RetType> {
  fn isSimpleText(self , rsthis: &mut QString) -> RetType;
}

  // proto:  bool QString::isSimpleText();
impl<'a> /*trait*/ QString_isSimpleText<i8> for () {
  fn isSimpleText(self , rsthis: &mut QString) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString12isSimpleTextEv()};
    let mut ret = unsafe {_ZNK7QString12isSimpleTextEv(rsthis.qclsinst)};
    return ret as i8;
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
    let mut ret = unsafe {_ZN7QString8fromUcs4EPKji(arg0, arg1)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QString & QString::setUnicode(const QChar * unicode, int size);
impl /*struct*/ QString {
  pub fn setUnicode<RetType, T: QString_setUnicode<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setUnicode(self);
    // return 1;
  }
}

pub trait QString_setUnicode<RetType> {
  fn setUnicode(self , rsthis: &mut QString) -> RetType;
}

  // proto:  QString & QString::setUnicode(const QChar * unicode, int size);
impl<'a> /*trait*/ QString_setUnicode<QString> for (QChar, i32) {
  fn setUnicode(self , rsthis: &mut QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString10setUnicodeEPK5QChari()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZN7QString10setUnicodeEPK5QChari(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  const_iterator QString::constBegin();
impl /*struct*/ QString {
  pub fn constBegin<RetType, T: QString_constBegin<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.constBegin(self);
    // return 1;
  }
}

pub trait QString_constBegin<RetType> {
  fn constBegin(self , rsthis: &mut QString) -> RetType;
}

  // proto:  const_iterator QString::constBegin();
impl<'a> /*trait*/ QString_constBegin<QChar> for () {
  fn constBegin(self , rsthis: &mut QString) -> QChar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString10constBeginEv()};
    let mut ret = unsafe {_ZNK7QString10constBeginEv(rsthis.qclsinst)};
    let mut ret1 = QChar{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  const QChar * QString::unicode();
impl /*struct*/ QString {
  pub fn unicode<RetType, T: QString_unicode<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.unicode(self);
    // return 1;
  }
}

pub trait QString_unicode<RetType> {
  fn unicode(self , rsthis: &mut QString) -> RetType;
}

  // proto:  const QChar * QString::unicode();
impl<'a> /*trait*/ QString_unicode<QChar> for () {
  fn unicode(self , rsthis: &mut QString) -> QChar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString7unicodeEv()};
    let mut ret = unsafe {_ZNK7QString7unicodeEv(rsthis.qclsinst)};
    let mut ret1 = QChar{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QString QString::arg(const QString & a1, const QString & a2, const QString & a3, const QString & a4, const QString & a5, const QString & a6, const QString & a7, const QString & a8, const QString & a9);
impl<'a> /*trait*/ QString_arg<QString> for (QString, QString, QString, QString, QString, QString, QString, QString, QString) {
  fn arg(self , rsthis: &mut QString) -> QString {
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
    let mut ret = unsafe {_ZNK7QString3argERKS_S1_S1_S1_S1_S1_S1_S1_S1_(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  int QString::indexOf(const QRegularExpression & re, int from);
impl<'a> /*trait*/ QString_indexOf<i32> for (QRegularExpression, i32) {
  fn indexOf(self , rsthis: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString7indexOfERK18QRegularExpressioni()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK7QString7indexOfERK18QRegularExpressioni(rsthis.qclsinst, arg0, arg1)};
    return ret as i32;
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
    let mut ret = unsafe {_ZN7QString6numberEji(arg0, arg1)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto: static QString QString::fromLocal8Bit(const QByteArray & str);
impl<'a> /*trait*/ QString_fromLocal8Bit_s<QString> for (QByteArray) {
  fn fromLocal8Bit_s(self ) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString13fromLocal8BitERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN7QString13fromLocal8BitERK10QByteArray(arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  const QChar QString::at(int i);
impl /*struct*/ QString {
  pub fn at<RetType, T: QString_at<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.at(self);
    // return 1;
  }
}

pub trait QString_at<RetType> {
  fn at(self , rsthis: &mut QString) -> RetType;
}

  // proto:  const QChar QString::at(int i);
impl<'a> /*trait*/ QString_at<QChar> for (i32) {
  fn at(self , rsthis: &mut QString) -> QChar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString2atEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK7QString2atEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QChar{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QString::QString(int size, QChar c);
impl<'a> /*trait*/ QString_NewQString for (i32, QChar) {
  fn NewQString(self) -> QString {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QStringC1Ei5QChar()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN7QStringC1Ei5QChar(qthis, arg0, arg1)};
    let rsthis = QString{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QString::push_front(const QString & s);
impl<'a> /*trait*/ QString_push_front<()> for (QString) {
  fn push_front(self , rsthis: &mut QString) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString10push_frontERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QString10push_frontERKS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QString QString::arg(const QString & a1, const QString & a2, const QString & a3, const QString & a4, const QString & a5, const QString & a6);
impl<'a> /*trait*/ QString_arg<QString> for (QString, QString, QString, QString, QString, QString) {
  fn arg(self , rsthis: &mut QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString3argERKS_S1_S1_S1_S1_S1_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let arg3 = self.3.qclsinst  as *mut c_void;
    let arg4 = self.4.qclsinst  as *mut c_void;
    let arg5 = self.5.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QString3argERKS_S1_S1_S1_S1_S1_(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4, arg5)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  iterator QString::begin();
impl /*struct*/ QString {
  pub fn begin<RetType, T: QString_begin<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.begin(self);
    // return 1;
  }
}

pub trait QString_begin<RetType> {
  fn begin(self , rsthis: &mut QString) -> RetType;
}

  // proto:  iterator QString::begin();
impl<'a> /*trait*/ QString_begin<QChar> for () {
  fn begin(self , rsthis: &mut QString) -> QChar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString5beginEv()};
    let mut ret = unsafe {_ZN7QString5beginEv(rsthis.qclsinst)};
    let mut ret1 = QChar{qclsinst: ret};
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
    let mut ret = unsafe {_ZN7QString6numberEdci(arg0, arg1, arg2)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  iterator QString::end();
impl /*struct*/ QString {
  pub fn end<RetType, T: QString_end<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.end(self);
    // return 1;
  }
}

pub trait QString_end<RetType> {
  fn end(self , rsthis: &mut QString) -> RetType;
}

  // proto:  iterator QString::end();
impl<'a> /*trait*/ QString_end<QChar> for () {
  fn end(self , rsthis: &mut QString) -> QChar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString3endEv()};
    let mut ret = unsafe {_ZN7QString3endEv(rsthis.qclsinst)};
    let mut ret1 = QChar{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QString & QString::append(QChar c);
impl<'a> /*trait*/ QString_append<QString> for (QChar) {
  fn append(self , rsthis: &mut QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString6appendE5QChar()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN7QString6appendE5QChar(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  uint QString::toUInt(bool * ok, int base);
impl /*struct*/ QString {
  pub fn toUInt<RetType, T: QString_toUInt<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toUInt(self);
    // return 1;
  }
}

pub trait QString_toUInt<RetType> {
  fn toUInt(self , rsthis: &mut QString) -> RetType;
}

  // proto:  uint QString::toUInt(bool * ok, int base);
impl<'a> /*trait*/ QString_toUInt<u32> for (&'a mut Vec<i8>, i32) {
  fn toUInt(self , rsthis: &mut QString) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString6toUIntEPbi()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK7QString6toUIntEPbi(rsthis.qclsinst, arg0, arg1)};
    return ret as u32;
    // return 1;
  }
}

  // proto:  QString & QString::append(const QString & s);
impl<'a> /*trait*/ QString_append<QString> for (QString) {
  fn append(self , rsthis: &mut QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString6appendERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN7QString6appendERKS_(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto: static QString QString::fromStdU16String(const std::u16string & s);
impl /*struct*/ QString {
  pub fn fromStdU16String_s<RetType, T: QString_fromStdU16String_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromStdU16String_s();
    // return 1;
  }
}

pub trait QString_fromStdU16String_s<RetType> {
  fn fromStdU16String_s(self ) -> RetType;
}

  // proto: static QString QString::fromStdU16String(const std::u16string & s);
impl<'a> /*trait*/ QString_fromStdU16String_s<QString> for (&'a  Vec<i32>) {
  fn fromStdU16String_s(self ) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString16fromStdU16StringERKi()};
    let arg0 = self.as_ptr()  as *mut c_int;
    let mut ret = unsafe {_ZN7QString16fromStdU16StringERKi(arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  ushort QString::toUShort(bool * ok, int base);
impl /*struct*/ QString {
  pub fn toUShort<RetType, T: QString_toUShort<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toUShort(self);
    // return 1;
  }
}

pub trait QString_toUShort<RetType> {
  fn toUShort(self , rsthis: &mut QString) -> RetType;
}

  // proto:  ushort QString::toUShort(bool * ok, int base);
impl<'a> /*trait*/ QString_toUShort<u16> for (&'a mut Vec<i8>, i32) {
  fn toUShort(self , rsthis: &mut QString) -> u16 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString8toUShortEPbi()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK7QString8toUShortEPbi(rsthis.qclsinst, arg0, arg1)};
    return ret as u16;
    // return 1;
  }
}

  // proto:  QString QString::arg(uint a, int fieldWidth, int base, QChar fillChar);
impl<'a> /*trait*/ QString_arg<QString> for (u32, i32, i32, QChar) {
  fn arg(self , rsthis: &mut QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString3argEjii5QChar()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QString3argEjii5QChar(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QString & QString::setNum(ushort , int base);
impl<'a> /*trait*/ QString_setNum<QString> for (u16, i32) {
  fn setNum(self , rsthis: &mut QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString6setNumEti()};
    let arg0 = self.0  as c_ushort;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZN7QString6setNumEti(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QString & QString::replace(const QRegularExpression & re, const QString & after);
impl<'a> /*trait*/ QString_replace<QString> for (QRegularExpression, QString) {
  fn replace(self , rsthis: &mut QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString7replaceERK18QRegularExpressionRKS_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN7QString7replaceERK18QRegularExpressionRKS_(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QString & QString::setNum(double , char f, int prec);
impl<'a> /*trait*/ QString_setNum<QString> for (f64, i8, i32) {
  fn setNum(self , rsthis: &mut QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString6setNumEdci()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_char;
    let arg2 = self.2  as c_int;
    let mut ret = unsafe {_ZN7QString6setNumEdci(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QString QString::arg(ushort a, int fieldWidth, int base, QChar fillChar);
impl<'a> /*trait*/ QString_arg<QString> for (u16, i32, i32, QChar) {
  fn arg(self , rsthis: &mut QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString3argEtii5QChar()};
    let arg0 = self.0  as c_ushort;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QString3argEtii5QChar(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QString::QString(const QString & );
impl<'a> /*trait*/ QString_NewQString for (QString) {
  fn NewQString(self) -> QString {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QStringC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN7QStringC1ERKS_(qthis, arg0)};
    let rsthis = QString{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QString QString::arg(short a, int fieldWidth, int base, QChar fillChar);
impl<'a> /*trait*/ QString_arg<QString> for (i16, i32, i32, QChar) {
  fn arg(self , rsthis: &mut QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString3argEsii5QChar()};
    let arg0 = self.0  as c_short;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QString3argEsii5QChar(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QString::QString(const QByteArray & a);
impl<'a> /*trait*/ QString_NewQString for (QByteArray) {
  fn NewQString(self) -> QString {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QStringC1ERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN7QStringC1ERK10QByteArray(qthis, arg0)};
    let rsthis = QString{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  qulonglong QString::toULongLong(bool * ok, int base);
impl /*struct*/ QString {
  pub fn toULongLong<RetType, T: QString_toULongLong<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toULongLong(self);
    // return 1;
  }
}

pub trait QString_toULongLong<RetType> {
  fn toULongLong(self , rsthis: &mut QString) -> RetType;
}

  // proto:  qulonglong QString::toULongLong(bool * ok, int base);
impl<'a> /*trait*/ QString_toULongLong<u64> for (&'a mut Vec<i8>, i32) {
  fn toULongLong(self , rsthis: &mut QString) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString11toULongLongEPbi()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK7QString11toULongLongEPbi(rsthis.qclsinst, arg0, arg1)};
    return ret as u64;
    // return 1;
  }
}

  // proto:  QString & QString::append(const char * s);
impl<'a> /*trait*/ QString_append<QString> for (&'a  String) {
  fn append(self , rsthis: &mut QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString6appendEPKc()};
    let arg0 = self.as_ptr()  as *mut c_char;
    let mut ret = unsafe {_ZN7QString6appendEPKc(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  int QString::capacity();
impl /*struct*/ QString {
  pub fn capacity<RetType, T: QString_capacity<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.capacity(self);
    // return 1;
  }
}

pub trait QString_capacity<RetType> {
  fn capacity(self , rsthis: &mut QString) -> RetType;
}

  // proto:  int QString::capacity();
impl<'a> /*trait*/ QString_capacity<i32> for () {
  fn capacity(self , rsthis: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString8capacityEv()};
    let mut ret = unsafe {_ZNK7QString8capacityEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QString::squeeze();
impl /*struct*/ QString {
  pub fn squeeze<RetType, T: QString_squeeze<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.squeeze(self);
    // return 1;
  }
}

pub trait QString_squeeze<RetType> {
  fn squeeze(self , rsthis: &mut QString) -> RetType;
}

  // proto:  void QString::squeeze();
impl<'a> /*trait*/ QString_squeeze<()> for () {
  fn squeeze(self , rsthis: &mut QString) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString7squeezeEv()};
     unsafe {_ZN7QString7squeezeEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QString::truncate(int pos);
impl /*struct*/ QString {
  pub fn truncate<RetType, T: QString_truncate<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.truncate(self);
    // return 1;
  }
}

pub trait QString_truncate<RetType> {
  fn truncate(self , rsthis: &mut QString) -> RetType;
}

  // proto:  void QString::truncate(int pos);
impl<'a> /*trait*/ QString_truncate<()> for (i32) {
  fn truncate(self , rsthis: &mut QString) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString8truncateEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN7QString8truncateEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QString QString::arg(int a, int fieldWidth, int base, QChar fillChar);
impl<'a> /*trait*/ QString_arg<QString> for (i32, i32, i32, QChar) {
  fn arg(self , rsthis: &mut QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString3argEiii5QChar()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QString3argEiii5QChar(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QString QString::arg(QChar a, int fieldWidth, QChar fillChar);
impl<'a> /*trait*/ QString_arg<QString> for (QChar, i32, QChar) {
  fn arg(self , rsthis: &mut QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString3argE5QChariS0_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QString3argE5QChariS0_(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  int QString::localeAwareCompare(const QString & s);
impl /*struct*/ QString {
  pub fn localeAwareCompare<RetType, T: QString_localeAwareCompare<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.localeAwareCompare(self);
    // return 1;
  }
}

pub trait QString_localeAwareCompare<RetType> {
  fn localeAwareCompare(self , rsthis: &mut QString) -> RetType;
}

  // proto:  int QString::localeAwareCompare(const QString & s);
impl<'a> /*trait*/ QString_localeAwareCompare<i32> for (QString) {
  fn localeAwareCompare(self , rsthis: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString18localeAwareCompareERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QString18localeAwareCompareERKS_(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QString & QString::remove(const QRegExp & rx);
impl<'a> /*trait*/ QString_remove<QString> for (QRegExp) {
  fn remove(self , rsthis: &mut QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString6removeERK7QRegExp()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN7QString6removeERK7QRegExp(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  bool QString::contains(const QRegularExpression & re);
impl<'a> /*trait*/ QString_contains<i8> for (QRegularExpression) {
  fn contains(self , rsthis: &mut QString) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString8containsERK18QRegularExpression()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QString8containsERK18QRegularExpression(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QString & QString::replace(int i, int len, QChar after);
impl<'a> /*trait*/ QString_replace<QString> for (i32, i32, QChar) {
  fn replace(self , rsthis: &mut QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString7replaceEii5QChar()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN7QString7replaceEii5QChar(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  bool QString::isRightToLeft();
impl /*struct*/ QString {
  pub fn isRightToLeft<RetType, T: QString_isRightToLeft<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isRightToLeft(self);
    // return 1;
  }
}

pub trait QString_isRightToLeft<RetType> {
  fn isRightToLeft(self , rsthis: &mut QString) -> RetType;
}

  // proto:  bool QString::isRightToLeft();
impl<'a> /*trait*/ QString_isRightToLeft<i8> for () {
  fn isRightToLeft(self , rsthis: &mut QString) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString13isRightToLeftEv()};
    let mut ret = unsafe {_ZNK7QString13isRightToLeftEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QString QString::arg(char a, int fieldWidth, QChar fillChar);
impl<'a> /*trait*/ QString_arg<QString> for (i8, i32, QChar) {
  fn arg(self , rsthis: &mut QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString3argEci5QChar()};
    let arg0 = self.0  as c_char;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QString3argEci5QChar(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QVector<uint> QString::toUcs4();
impl /*struct*/ QString {
  pub fn toUcs4<RetType, T: QString_toUcs4<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toUcs4(self);
    // return 1;
  }
}

pub trait QString_toUcs4<RetType> {
  fn toUcs4(self , rsthis: &mut QString) -> RetType;
}

  // proto:  QVector<uint> QString::toUcs4();
impl<'a> /*trait*/ QString_toUcs4<()> for () {
  fn toUcs4(self , rsthis: &mut QString) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString6toUcs4Ev()};
     unsafe {_ZNK7QString6toUcs4Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QString & QString::remove(int i, int len);
impl<'a> /*trait*/ QString_remove<QString> for (i32, i32) {
  fn remove(self , rsthis: &mut QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString6removeEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZN7QString6removeEii(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  int QString::lastIndexOf(const QRegularExpression & re, int from, QRegularExpressionMatch * rmatch);
impl<'a> /*trait*/ QString_lastIndexOf<i32> for (QRegularExpression, i32, QRegularExpressionMatch) {
  fn lastIndexOf(self , rsthis: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString11lastIndexOfERK18QRegularExpressioniP23QRegularExpressionMatch()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QString11lastIndexOfERK18QRegularExpressioniP23QRegularExpressionMatch(rsthis.qclsinst, arg0, arg1, arg2)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QString QString::right(int n);
impl /*struct*/ QString {
  pub fn right<RetType, T: QString_right<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.right(self);
    // return 1;
  }
}

pub trait QString_right<RetType> {
  fn right(self , rsthis: &mut QString) -> RetType;
}

  // proto:  QString QString::right(int n);
impl<'a> /*trait*/ QString_right<QString> for (i32) {
  fn right(self , rsthis: &mut QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString5rightEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK7QString5rightEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QString QString::rightJustified(int width, QChar fill, bool trunc);
impl /*struct*/ QString {
  pub fn rightJustified<RetType, T: QString_rightJustified<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.rightJustified(self);
    // return 1;
  }
}

pub trait QString_rightJustified<RetType> {
  fn rightJustified(self , rsthis: &mut QString) -> RetType;
}

  // proto:  QString QString::rightJustified(int width, QChar fill, bool trunc);
impl<'a> /*trait*/ QString_rightJustified<QString> for (i32, QChar, i8) {
  fn rightJustified(self , rsthis: &mut QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString14rightJustifiedEi5QCharb()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_char;
    let mut ret = unsafe {_ZNK7QString14rightJustifiedEi5QCharb(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QString QString::arg(const QString & a, int fieldWidth, QChar fillChar);
impl<'a> /*trait*/ QString_arg<QString> for (QString, i32, QChar) {
  fn arg(self , rsthis: &mut QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString3argERKS_i5QChar()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QString3argERKS_i5QChar(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QString QString::arg(qulonglong a, int fieldwidth, int base, QChar fillChar);
impl<'a> /*trait*/ QString_arg<QString> for (u64, i32, i32, QChar) {
  fn arg(self , rsthis: &mut QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString3argEyii5QChar()};
    let arg0 = self.0  as c_ulonglong;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QString3argEyii5QChar(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QString::reserve(int size);
impl /*struct*/ QString {
  pub fn reserve<RetType, T: QString_reserve<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.reserve(self);
    // return 1;
  }
}

pub trait QString_reserve<RetType> {
  fn reserve(self , rsthis: &mut QString) -> RetType;
}

  // proto:  void QString::reserve(int size);
impl<'a> /*trait*/ QString_reserve<()> for (i32) {
  fn reserve(self , rsthis: &mut QString) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString7reserveEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN7QString7reserveEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  short QString::toShort(bool * ok, int base);
impl /*struct*/ QString {
  pub fn toShort<RetType, T: QString_toShort<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toShort(self);
    // return 1;
  }
}

pub trait QString_toShort<RetType> {
  fn toShort(self , rsthis: &mut QString) -> RetType;
}

  // proto:  short QString::toShort(bool * ok, int base);
impl<'a> /*trait*/ QString_toShort<i16> for (&'a mut Vec<i8>, i32) {
  fn toShort(self , rsthis: &mut QString) -> i16 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString7toShortEPbi()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK7QString7toShortEPbi(rsthis.qclsinst, arg0, arg1)};
    return ret as i16;
    // return 1;
  }
}

