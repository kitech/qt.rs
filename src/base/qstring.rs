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
  // proto:  qint64 QString::toLongLong(bool * ok, int base);
  fn _ZNK7QString10toLongLongEPbi(qthis: *mut c_void, arg0: *mut int8_t, arg1: c_int) -> c_longlong;
  // proto:  bool QString::isNull();
  fn _ZNK7QString6isNullEv(qthis: *mut c_void) -> int8_t;
  // proto:  QString & QString::append(const QChar * uc, int len);
  fn _ZN7QString6appendEPK5QChari(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) -> *mut c_void;
  // proto:  QString & QString::prepend(QChar c);
  fn _ZN7QString7prependE5QChar(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QString & QString::insert(int i, QChar c);
  fn _ZN7QString6insertEi5QChar(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) -> *mut c_void;
  // proto:  QString QString::left(int n);
  fn _ZNK7QString4leftEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  void QString::NewQString(QChar c);
  fn _ZN7QStringC1E5QChar(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QString::isEmpty();
  fn _ZNK7QString7isEmptyEv(qthis: *mut c_void) -> int8_t;
  // proto:  QString & QString::prepend(const char * s);
  fn _ZN7QString7prependEPKc(qthis: *mut c_void, arg0: *const c_char) -> *mut c_void;
  // proto:  int QString::lastIndexOf(const QRegularExpression & re, int from);
  fn _ZNK7QString11lastIndexOfERK18QRegularExpressioni(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) -> c_int;
  // proto: static QString QString::number(int , int base);
  fn _ZN7QString6numberEii(arg0: c_int, arg1: c_int) -> *mut c_void;
  // proto:  void QString::resize(int size);
  fn _ZN7QString6resizeEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QString::push_front(QChar c);
  fn _ZN7QString10push_frontE5QChar(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QString::NewQString();
  fn _ZN7QStringC1Ev(qthis: *mut c_void) ;
  // proto:  double QString::toDouble(bool * ok);
  fn _ZNK7QString8toDoubleEPb(qthis: *mut c_void, arg0: *mut int8_t) -> c_double;
  // proto:  QString QString::arg(const QString & a1, const QString & a2, const QString & a3, const QString & a4, const QString & a5, const QString & a6, const QString & a7, const QString & a8);
  fn _ZNK7QString3argERKS_S1_S1_S1_S1_S1_S1_S1_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void, arg3: *mut c_void, arg4: *mut c_void, arg5: *mut c_void, arg6: *mut c_void, arg7: *mut c_void) -> *mut c_void;
  // proto:  QStringRef QString::rightRef(int n);
  fn _ZNK7QString8rightRefEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  QString & QString::setNum(short , int base);
  fn _ZN7QString6setNumEsi(qthis: *mut c_void, arg0: c_short, arg1: c_int) -> *mut c_void;
  // proto:  void QString::NewQString(const QChar * unicode, int size);
  fn _ZN7QStringC1EPK5QChari(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) ;
  // proto:  float QString::toFloat(bool * ok);
  fn _ZNK7QString7toFloatEPb(qthis: *mut c_void, arg0: *mut int8_t) -> c_float;
  // proto:  int QString::count(const QRegularExpression & re);
  fn _ZNK7QString5countERK18QRegularExpression(qthis: *mut c_void, arg0: *mut c_void) -> c_int;
  // proto:  QStringRef QString::midRef(int position, int n);
  fn _ZNK7QString6midRefEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) ;
  // proto:  void QString::detach();
  fn _ZN7QString6detachEv(qthis: *mut c_void) ;
  // proto:  QString QString::arg(const QString & a1, const QString & a2, const QString & a3, const QString & a4);
  fn _ZNK7QString3argERKS_S1_S1_S1_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void, arg3: *mut c_void) -> *mut c_void;
  // proto:  QString QString::arg(long a, int fieldwidth, int base, QChar fillChar);
  fn _ZNK7QString3argElii5QChar(qthis: *mut c_void, arg0: c_long, arg1: c_int, arg2: c_int, arg3: *mut c_void) -> *mut c_void;
  // proto:  int QString::count();
  fn _ZNK7QString5countEv(qthis: *mut c_void) -> c_int;
  // proto:  QString & QString::setNum(qulonglong , int base);
  fn _ZN7QString6setNumEyi(qthis: *mut c_void, arg0: uint64_t, arg1: c_int) -> *mut c_void;
  // proto: static QString QString::fromStdWString(const std::wstring & s);
  fn _ZN7QString14fromStdWStringERKi(arg0: *const c_int) -> *mut c_void;
  // proto:  void QString::push_back(QChar c);
  fn _ZN7QString9push_backE5QChar(qthis: *mut c_void, arg0: *mut c_void) ;
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
  // proto:  unsigned long QString::toULong(bool * ok, int base);
  fn _ZNK7QString7toULongEPbi(qthis: *mut c_void, arg0: *mut int8_t, arg1: c_int) -> c_ulong;
  // proto:  void QString::chop(int n);
  fn _ZN7QString4chopEi(qthis: *mut c_void, arg0: c_int) ;
  // proto: static QString QString::fromUtf16(const ushort * , int size);
  fn _ZN7QString9fromUtf16EPKti(arg0: *const c_ushort, arg1: c_int) -> *mut c_void;
  // proto:  bool QString::isDetached();
  fn _ZNK7QString10isDetachedEv(qthis: *mut c_void) -> int8_t;
  // proto:  QString & QString::setNum(qlonglong , int base);
  fn _ZN7QString6setNumExi(qthis: *mut c_void, arg0: c_longlong, arg1: c_int) -> *mut c_void;
  // proto:  QString QString::mid(int position, int n);
  fn _ZNK7QString3midEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) -> *mut c_void;
  // proto: static QString QString::fromLocal8Bit(const char * str, int size);
  fn _ZN7QString13fromLocal8BitEPKci(arg0: *const c_char, arg1: c_int) -> *mut c_void;
  // proto:  void QString::swap(QString & other);
  fn _ZN7QString4swapERS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto: static QString QString::fromUtf8(const QByteArray & str);
  fn _ZN7QString8fromUtf8ERK10QByteArray(arg0: *mut c_void) -> *mut c_void;
  // proto: static QString QString::fromUcs4(const char32_t * str, int size);
  fn _ZN7QString8fromUcs4EPKDii(arg0: *const int32_t, arg1: c_int) -> *mut c_void;
  // proto:  QString QString::leftJustified(int width, QChar fill, bool trunc);
  fn _ZNK7QString13leftJustifiedEi5QCharb(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void, arg2: int8_t) -> *mut c_void;
  // proto:  int QString::indexOf(const QRegExp & , int from);
  fn _ZNK7QString7indexOfERK7QRegExpi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) -> c_int;
  // proto:  void QString::push_back(const QString & s);
  fn _ZN7QString9push_backERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  int QString::lastIndexOf(QRegExp & , int from);
  fn _ZNK7QString11lastIndexOfER7QRegExpi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) -> c_int;
  // proto:  QString QString::arg(const QString & a1, const QString & a2, const QString & a3);
  fn _ZNK7QString3argERKS_S1_S1_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) -> *mut c_void;
  // proto:  const ushort * QString::utf16();
  fn _ZNK7QString5utf16Ev(qthis: *mut c_void) ;
  // proto:  int QString::toInt(bool * ok, int base);
  fn _ZNK7QString5toIntEPbi(qthis: *mut c_void, arg0: *mut int8_t, arg1: c_int) -> c_int;
  // proto:  QString QString::arg(double a, int fieldWidth, char fmt, int prec, QChar fillChar);
  fn _ZNK7QString3argEdici5QChar(qthis: *mut c_void, arg0: c_double, arg1: c_int, arg2: c_char, arg3: c_int, arg4: *mut c_void) -> *mut c_void;
  // proto:  QChar * QString::data();
  fn _ZN7QString4dataEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QString & QString::setNum(uint , int base);
  fn _ZN7QString6setNumEji(qthis: *mut c_void, arg0: c_uint, arg1: c_int) -> *mut c_void;
  // proto: static int QString::localeAwareCompare(const QString & s1, const QString & s2);
  fn _ZN7QString18localeAwareCompareERKS_S1_(arg0: *mut c_void, arg1: *mut c_void) -> c_int;
  // proto:  void QString::NewQString(const char * ch);
  fn _ZN7QStringC1EPKc(qthis: *mut c_void, arg0: *const c_char) ;
  // proto: static QString QString::fromUtf16(const char16_t * str, int size);
  fn _ZN7QString9fromUtf16EPKDsi(arg0: *const int16_t, arg1: c_int) -> *mut c_void;
  // proto:  QString & QString::replace(const QRegExp & rx, const QString & after);
  fn _ZN7QString7replaceERK7QRegExpRKS_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  QString QString::repeated(int times);
  fn _ZNK7QString8repeatedEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  QString & QString::setUtf16(const ushort * utf16, int size);
  fn _ZN7QString8setUtf16EPKti(qthis: *mut c_void, arg0: *const c_ushort, arg1: c_int) -> *mut c_void;
  // proto: static QString QString::fromStdU32String(const std::u32string & s);
  fn _ZN7QString16fromStdU32StringERKi(arg0: *const c_int) -> *mut c_void;
  // proto:  void QString::clear();
  fn _ZN7QString5clearEv(qthis: *mut c_void) ;
  // proto:  bool QString::contains(const QRegExp & rx);
  fn _ZNK7QString8containsERK7QRegExp(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  bool QString::isSharedWith(const QString & other);
  fn _ZNK7QString12isSharedWithERKS_(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto: static QString QString::fromLatin1(const QByteArray & str);
  fn _ZN7QString10fromLatin1ERK10QByteArray(arg0: *mut c_void) -> *mut c_void;
  // proto:  void QString::FreeQString();
  fn _ZN7QStringD0Ev(qthis: *mut c_void) ;
  // proto:  QString & QString::remove(const QRegularExpression & re);
  fn _ZN7QString6removeERK18QRegularExpression(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  const QChar * QString::cend();
  fn _ZNK7QString4cendEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QString QString::toHtmlEscaped();
  fn _ZNK7QString13toHtmlEscapedEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QString & QString::append(const QByteArray & s);
  fn _ZN7QString6appendERK10QByteArray(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto: static QString QString::fromLatin1(const char * str, int size);
  fn _ZN7QString10fromLatin1EPKci(arg0: *const c_char, arg1: c_int) -> *mut c_void;
  // proto:  bool QString::contains(const QRegularExpression & re, QRegularExpressionMatch * match);
  fn _ZNK7QString8containsERK18QRegularExpressionP23QRegularExpressionMatch(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> int8_t;
  // proto:  int QString::indexOf(const QRegularExpression & re, int from, QRegularExpressionMatch * rmatch);
  fn _ZNK7QString7indexOfERK18QRegularExpressioniP23QRegularExpressionMatch(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int, arg2: *mut c_void) -> c_int;
  // proto:  int QString::lastIndexOf(const QRegExp & , int from);
  fn _ZNK7QString11lastIndexOfERK7QRegExpi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) -> c_int;
  // proto:  int QString::toWCharArray(wchar_t * array);
  fn _ZNK7QString12toWCharArrayEPw(qthis: *mut c_void, arg0: *mut wchar_t) -> c_int;
  // proto:  const QChar * QString::cbegin();
  fn _ZNK7QString6cbeginEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QString & QString::prepend(const QByteArray & s);
  fn _ZN7QString7prependERK10QByteArray(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QString & QString::replace(int i, int len, const QString & after);
  fn _ZN7QString7replaceEiiRKS_(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *mut c_void) -> *mut c_void;
  // proto: static QString QString::fromStdString(const std::string & s);
  fn _ZN7QString13fromStdStringERKi(arg0: *const c_int) -> *mut c_void;
  // proto:  QString QString::arg(const QString & a1, const QString & a2, const QString & a3, const QString & a4, const QString & a5, const QString & a6, const QString & a7);
  fn _ZNK7QString3argERKS_S1_S1_S1_S1_S1_S1_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void, arg3: *mut c_void, arg4: *mut c_void, arg5: *mut c_void, arg6: *mut c_void) -> *mut c_void;
  // proto: static QString QString::fromWCharArray(const wchar_t * string, int size);
  fn _ZN7QString14fromWCharArrayEPKwi(arg0: *const wchar_t, arg1: c_int) -> *mut c_void;
  // proto:  QString & QString::fill(QChar c, int size);
  fn _ZN7QString4fillE5QChari(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) -> *mut c_void;
  // proto:  const QChar * QString::constData();
  fn _ZNK7QString9constDataEv(qthis: *mut c_void) -> *mut c_void;
  // proto: static QString QString::number(ulong , int base);
  fn _ZN7QString6numberEmi(arg0: c_ulong, arg1: c_int) -> *mut c_void;
  // proto:  long QString::toLong(bool * ok, int base);
  fn _ZNK7QString6toLongEPbi(qthis: *mut c_void, arg0: *mut int8_t, arg1: c_int) -> c_long;
  // proto:  const QChar * QString::constEnd();
  fn _ZNK7QString8constEndEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QString::length();
  fn _ZNK7QString6lengthEv(qthis: *mut c_void) -> c_int;
  // proto: static QString QString::fromUtf8(const char * str, int size);
  fn _ZN7QString8fromUtf8EPKci(arg0: *const c_char, arg1: c_int) -> *mut c_void;
  // proto: static QString QString::number(qlonglong , int base);
  fn _ZN7QString6numberExi(arg0: c_longlong, arg1: c_int) -> *mut c_void;
  // proto:  QStringRef QString::leftRef(int n);
  fn _ZNK7QString7leftRefEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  QString & QString::setNum(long , int base);
  fn _ZN7QString6setNumEli(qthis: *mut c_void, arg0: c_long, arg1: c_int) -> *mut c_void;
  // proto:  QString QString::arg(const QString & a1, const QString & a2);
  fn _ZNK7QString3argERKS_S1_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  bool QString::isSimpleText();
  fn _ZNK7QString12isSimpleTextEv(qthis: *mut c_void) -> int8_t;
  // proto: static QString QString::fromUcs4(const uint * , int size);
  fn _ZN7QString8fromUcs4EPKji(arg0: *const c_uint, arg1: c_int) -> *mut c_void;
  // proto:  QString & QString::setUnicode(const QChar * unicode, int size);
  fn _ZN7QString10setUnicodeEPK5QChari(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) -> *mut c_void;
  // proto:  bool QString::contains(QRegExp & rx);
  fn _ZNK7QString8containsER7QRegExp(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  const QChar * QString::constBegin();
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
  // proto:  void QString::NewQString(int size, QChar c);
  fn _ZN7QStringC1Ei5QChar(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) ;
  // proto:  QString & QString::setNum(ulong , int base);
  fn _ZN7QString6setNumEmi(qthis: *mut c_void, arg0: c_ulong, arg1: c_int) -> *mut c_void;
  // proto:  void QString::push_front(const QString & s);
  fn _ZN7QString10push_frontERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QString QString::arg(const QString & a1, const QString & a2, const QString & a3, const QString & a4, const QString & a5, const QString & a6);
  fn _ZNK7QString3argERKS_S1_S1_S1_S1_S1_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void, arg3: *mut c_void, arg4: *mut c_void, arg5: *mut c_void) -> *mut c_void;
  // proto:  QChar * QString::begin();
  fn _ZN7QString5beginEv(qthis: *mut c_void) -> *mut c_void;
  // proto: static QString QString::number(double , char f, int prec);
  fn _ZN7QString6numberEdci(arg0: c_double, arg1: c_char, arg2: c_int) -> *mut c_void;
  // proto:  QChar * QString::end();
  fn _ZN7QString3endEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QString & QString::append(QChar c);
  fn _ZN7QString6appendE5QChar(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  unsigned int QString::toUInt(bool * ok, int base);
  fn _ZNK7QString6toUIntEPbi(qthis: *mut c_void, arg0: *mut int8_t, arg1: c_int) -> c_uint;
  // proto:  QString & QString::append(const QString & s);
  fn _ZN7QString6appendERKS_(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto: static QString QString::fromStdU16String(const std::u16string & s);
  fn _ZN7QString16fromStdU16StringERKi(arg0: *const c_int) -> *mut c_void;
  // proto:  QString QString::arg(qlonglong a, int fieldwidth, int base, QChar fillChar);
  fn _ZNK7QString3argExii5QChar(qthis: *mut c_void, arg0: c_longlong, arg1: c_int, arg2: c_int, arg3: *mut c_void) -> *mut c_void;
  // proto:  unsigned short QString::toUShort(bool * ok, int base);
  fn _ZNK7QString8toUShortEPbi(qthis: *mut c_void, arg0: *mut int8_t, arg1: c_int) -> c_ushort;
  // proto:  QString QString::arg(uint a, int fieldWidth, int base, QChar fillChar);
  fn _ZNK7QString3argEjii5QChar(qthis: *mut c_void, arg0: c_uint, arg1: c_int, arg2: c_int, arg3: *mut c_void) -> *mut c_void;
  // proto:  QString & QString::setNum(ushort , int base);
  fn _ZN7QString6setNumEti(qthis: *mut c_void, arg0: c_ushort, arg1: c_int) -> *mut c_void;
  // proto:  QString & QString::replace(const QRegularExpression & re, const QString & after);
  fn _ZN7QString7replaceERK18QRegularExpressionRKS_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  QString & QString::setNum(double , char f, int prec);
  fn _ZN7QString6setNumEdci(qthis: *mut c_void, arg0: c_double, arg1: c_char, arg2: c_int) -> *mut c_void;
  // proto: static QString QString::number(qulonglong , int base);
  fn _ZN7QString6numberEyi(arg0: uint64_t, arg1: c_int) -> *mut c_void;
  // proto:  QString QString::arg(ushort a, int fieldWidth, int base, QChar fillChar);
  fn _ZNK7QString3argEtii5QChar(qthis: *mut c_void, arg0: c_ushort, arg1: c_int, arg2: c_int, arg3: *mut c_void) -> *mut c_void;
  // proto:  void QString::NewQString(const QString & );
  fn _ZN7QStringC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QString QString::arg(short a, int fieldWidth, int base, QChar fillChar);
  fn _ZNK7QString3argEsii5QChar(qthis: *mut c_void, arg0: c_short, arg1: c_int, arg2: c_int, arg3: *mut c_void) -> *mut c_void;
  // proto:  void QString::NewQString(const QByteArray & a);
  fn _ZN7QStringC1ERK10QByteArray(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  quint64 QString::toULongLong(bool * ok, int base);
  fn _ZNK7QString11toULongLongEPbi(qthis: *mut c_void, arg0: *mut int8_t, arg1: c_int) -> uint64_t;
  // proto:  QString & QString::append(const char * s);
  fn _ZN7QString6appendEPKc(qthis: *mut c_void, arg0: *const c_char) -> *mut c_void;
  // proto:  int QString::capacity();
  fn _ZNK7QString8capacityEv(qthis: *mut c_void) -> c_int;
  // proto:  void QString::squeeze();
  fn _ZN7QString7squeezeEv(qthis: *mut c_void) ;
  // proto:  void QString::truncate(int pos);
  fn _ZN7QString8truncateEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  QString QString::arg(int a, int fieldWidth, int base, QChar fillChar);
  fn _ZNK7QString3argEiii5QChar(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: *mut c_void) -> *mut c_void;
  // proto:  QString QString::arg(QChar a, int fieldWidth, QChar fillChar);
  fn _ZNK7QString3argE5QChariS0_(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int, arg2: *mut c_void) -> *mut c_void;
  // proto:  int QString::localeAwareCompare(const QString & s);
  fn _ZNK7QString18localeAwareCompareERKS_(qthis: *mut c_void, arg0: *mut c_void) -> c_int;
  // proto:  QString & QString::remove(const QRegExp & rx);
  fn _ZN7QString6removeERK7QRegExp(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  bool QString::contains(const QRegularExpression & re);
  fn _ZNK7QString8containsERK18QRegularExpression(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  int QString::indexOf(QRegExp & , int from);
  fn _ZNK7QString7indexOfER7QRegExpi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) -> c_int;
  // proto:  QString & QString::replace(int i, int len, QChar after);
  fn _ZN7QString7replaceEii5QChar(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *mut c_void) -> *mut c_void;
  // proto:  bool QString::isRightToLeft();
  fn _ZNK7QString13isRightToLeftEv(qthis: *mut c_void) -> int8_t;
  // proto:  QString QString::arg(char a, int fieldWidth, QChar fillChar);
  fn _ZNK7QString3argEci5QChar(qthis: *mut c_void, arg0: c_char, arg1: c_int, arg2: *mut c_void) -> *mut c_void;
  // proto:  QVector<uint> QString::toUcs4();
  fn _ZNK7QString6toUcs4Ev(qthis: *mut c_void) ;
  // proto:  QString & QString::remove(int i, int len);
  fn _ZN7QString6removeEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) -> *mut c_void;
  // proto:  int QString::lastIndexOf(const QRegularExpression & re, int from, QRegularExpressionMatch * rmatch);
  fn _ZNK7QString11lastIndexOfERK18QRegularExpressioniP23QRegularExpressionMatch(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int, arg2: *mut c_void) -> c_int;
  // proto:  QString QString::right(int n);
  fn _ZNK7QString5rightEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  QString QString::rightJustified(int width, QChar fill, bool trunc);
  fn _ZNK7QString14rightJustifiedEi5QCharb(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void, arg2: int8_t) -> *mut c_void;
  // proto:  QString QString::arg(const QString & a, int fieldWidth, QChar fillChar);
  fn _ZNK7QString3argERKS_i5QChar(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int, arg2: *mut c_void) -> *mut c_void;
  // proto:  QString QString::arg(qulonglong a, int fieldwidth, int base, QChar fillChar);
  fn _ZNK7QString3argEyii5QChar(qthis: *mut c_void, arg0: uint64_t, arg1: c_int, arg2: c_int, arg3: *mut c_void) -> *mut c_void;
  // proto:  void QString::reserve(int size);
  fn _ZN7QString7reserveEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  short QString::toShort(bool * ok, int base);
  fn _ZNK7QString7toShortEPbi(qthis: *mut c_void, arg0: *mut int8_t, arg1: c_int) -> c_short;
  // proto:  QString QString::arg(ulong a, int fieldwidth, int base, QChar fillChar);
  fn _ZNK7QString3argEmii5QChar(qthis: *mut c_void, arg0: c_ulong, arg1: c_int, arg2: c_int, arg3: *mut c_void) -> *mut c_void;
}

// body block begin
// class sizeof(QString)=8
pub struct QString {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QString {
  pub fn toLongLong<T: QString_toLongLong>(&mut self, value: T) -> i64 {
    return value.toLongLong(self);
    // return 1;
  }
}

pub trait QString_toLongLong {
  fn toLongLong(self, rsthis: &mut QString) -> i64;
}

// proto:  qint64 QString::toLongLong(bool * ok, int base);
impl<'a> /*trait*/ QString_toLongLong for (&'a mut i8, i32) {
  fn toLongLong(self, rsthis: &mut QString) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString10toLongLongEPbi()};
    let arg0 = self.0  as *mut int8_t;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK7QString10toLongLongEPbi(rsthis.qclsinst, arg0, arg1)};
    return ret as i64;
    // return 1;
  }
}

impl /*struct*/ QString {
  pub fn isNull<T: QString_isNull>(&mut self, value: T) -> i8 {
    return value.isNull(self);
    // return 1;
  }
}

pub trait QString_isNull {
  fn isNull(self, rsthis: &mut QString) -> i8;
}

// proto:  bool QString::isNull();
impl<'a> /*trait*/ QString_isNull for () {
  fn isNull(self, rsthis: &mut QString) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString6isNullEv()};
    let mut ret = unsafe {_ZNK7QString6isNullEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QString {
  pub fn append<T: QString_append>(&mut self, value: T) -> QString {
    return value.append(self);
    // return 1;
  }
}

pub trait QString_append {
  fn append(self, rsthis: &mut QString) -> QString;
}

// proto:  QString & QString::append(const QChar * uc, int len);
impl<'a> /*trait*/ QString_append for (&'a  QChar, i32) {
  fn append(self, rsthis: &mut QString) -> QString {
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

impl /*struct*/ QString {
  pub fn prepend<T: QString_prepend>(&mut self, value: T) -> QString {
    return value.prepend(self);
    // return 1;
  }
}

pub trait QString_prepend {
  fn prepend(self, rsthis: &mut QString) -> QString;
}

// proto:  QString & QString::prepend(QChar c);
impl<'a> /*trait*/ QString_prepend for (QChar) {
  fn prepend(self, rsthis: &mut QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString7prependE5QChar()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN7QString7prependE5QChar(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QString {
  pub fn insert<T: QString_insert>(&mut self, value: T) -> QString {
    return value.insert(self);
    // return 1;
  }
}

pub trait QString_insert {
  fn insert(self, rsthis: &mut QString) -> QString;
}

// proto:  QString & QString::insert(int i, QChar c);
impl<'a> /*trait*/ QString_insert for (i32, QChar) {
  fn insert(self, rsthis: &mut QString) -> QString {
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

impl /*struct*/ QString {
  pub fn left<T: QString_left>(&mut self, value: T) -> QString {
    return value.left(self);
    // return 1;
  }
}

pub trait QString_left {
  fn left(self, rsthis: &mut QString) -> QString;
}

// proto:  QString QString::left(int n);
impl<'a> /*trait*/ QString_left for (i32) {
  fn left(self, rsthis: &mut QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString4leftEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK7QString4leftEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

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

// proto: void QString::NewQString(QChar c);
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

impl /*struct*/ QString {
  pub fn isEmpty<T: QString_isEmpty>(&mut self, value: T) -> i8 {
    return value.isEmpty(self);
    // return 1;
  }
}

pub trait QString_isEmpty {
  fn isEmpty(self, rsthis: &mut QString) -> i8;
}

// proto:  bool QString::isEmpty();
impl<'a> /*trait*/ QString_isEmpty for () {
  fn isEmpty(self, rsthis: &mut QString) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString7isEmptyEv()};
    let mut ret = unsafe {_ZNK7QString7isEmptyEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto:  QString & QString::prepend(const char * s);
impl<'a> /*trait*/ QString_prepend for (&'a  String) {
  fn prepend(self, rsthis: &mut QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString7prependEPKc()};
    let arg0 = self.as_ptr()  as *const c_char;
    let mut ret = unsafe {_ZN7QString7prependEPKc(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QString {
  pub fn lastIndexOf<T: QString_lastIndexOf>(&mut self, value: T) -> i32 {
    return value.lastIndexOf(self);
    // return 1;
  }
}

pub trait QString_lastIndexOf {
  fn lastIndexOf(self, rsthis: &mut QString) -> i32;
}

// proto:  int QString::lastIndexOf(const QRegularExpression & re, int from);
impl<'a> /*trait*/ QString_lastIndexOf for (&'a  QRegularExpression, i32) {
  fn lastIndexOf(self, rsthis: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString11lastIndexOfERK18QRegularExpressioni()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK7QString11lastIndexOfERK18QRegularExpressioni(rsthis.qclsinst, arg0, arg1)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QString {
  pub fn number<T: QString_number>(&mut self, value: T) -> QString {
    return value.number(self);
    // return 1;
  }
}

pub trait QString_number {
  fn number(self, rsthis: &mut QString) -> QString;
}

// proto: static QString QString::number(int , int base);
impl<'a> /*trait*/ QString_number for (i32, i32) {
  fn number(self, rsthis: &mut QString) -> QString {
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

impl /*struct*/ QString {
  pub fn resize<T: QString_resize>(&mut self, value: T)  {
     value.resize(self);
    // return 1;
  }
}

pub trait QString_resize {
  fn resize(self, rsthis: &mut QString) ;
}

// proto:  void QString::resize(int size);
impl<'a> /*trait*/ QString_resize for (i32) {
  fn resize(self, rsthis: &mut QString)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString6resizeEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN7QString6resizeEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QString {
  pub fn push_front<T: QString_push_front>(&mut self, value: T)  {
     value.push_front(self);
    // return 1;
  }
}

pub trait QString_push_front {
  fn push_front(self, rsthis: &mut QString) ;
}

// proto:  void QString::push_front(QChar c);
impl<'a> /*trait*/ QString_push_front for (QChar) {
  fn push_front(self, rsthis: &mut QString)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString10push_frontE5QChar()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QString10push_frontE5QChar(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto: void QString::NewQString();
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

impl /*struct*/ QString {
  pub fn toDouble<T: QString_toDouble>(&mut self, value: T) -> f64 {
    return value.toDouble(self);
    // return 1;
  }
}

pub trait QString_toDouble {
  fn toDouble(self, rsthis: &mut QString) -> f64;
}

// proto:  double QString::toDouble(bool * ok);
impl<'a> /*trait*/ QString_toDouble for (&'a mut i8) {
  fn toDouble(self, rsthis: &mut QString) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString8toDoubleEPb()};
    let arg0 = self  as *mut int8_t;
    let mut ret = unsafe {_ZNK7QString8toDoubleEPb(rsthis.qclsinst, arg0)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QString {
  pub fn arg<T: QString_arg>(&mut self, value: T) -> QString {
    return value.arg(self);
    // return 1;
  }
}

pub trait QString_arg {
  fn arg(self, rsthis: &mut QString) -> QString;
}

// proto:  QString QString::arg(const QString & a1, const QString & a2, const QString & a3, const QString & a4, const QString & a5, const QString & a6, const QString & a7, const QString & a8);
impl<'a> /*trait*/ QString_arg for (&'a  QString, &'a  QString, &'a  QString, &'a  QString, &'a  QString, &'a  QString, &'a  QString, &'a  QString) {
  fn arg(self, rsthis: &mut QString) -> QString {
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

impl /*struct*/ QString {
  pub fn rightRef<T: QString_rightRef>(&mut self, value: T)  {
     value.rightRef(self);
    // return 1;
  }
}

pub trait QString_rightRef {
  fn rightRef(self, rsthis: &mut QString) ;
}

// proto:  QStringRef QString::rightRef(int n);
impl<'a> /*trait*/ QString_rightRef for (i32) {
  fn rightRef(self, rsthis: &mut QString)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString8rightRefEi()};
    let arg0 = self  as c_int;
     unsafe {_ZNK7QString8rightRefEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QString {
  pub fn setNum<T: QString_setNum>(&mut self, value: T) -> QString {
    return value.setNum(self);
    // return 1;
  }
}

pub trait QString_setNum {
  fn setNum(self, rsthis: &mut QString) -> QString;
}

// proto:  QString & QString::setNum(short , int base);
impl<'a> /*trait*/ QString_setNum for (i16, i32) {
  fn setNum(self, rsthis: &mut QString) -> QString {
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

// proto: void QString::NewQString(const QChar * unicode, int size);
impl<'a> /*trait*/ QString_NewQString for (&'a  QChar, i32) {
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

impl /*struct*/ QString {
  pub fn toFloat<T: QString_toFloat>(&mut self, value: T) -> f32 {
    return value.toFloat(self);
    // return 1;
  }
}

pub trait QString_toFloat {
  fn toFloat(self, rsthis: &mut QString) -> f32;
}

// proto:  float QString::toFloat(bool * ok);
impl<'a> /*trait*/ QString_toFloat for (&'a mut i8) {
  fn toFloat(self, rsthis: &mut QString) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString7toFloatEPb()};
    let arg0 = self  as *mut int8_t;
    let mut ret = unsafe {_ZNK7QString7toFloatEPb(rsthis.qclsinst, arg0)};
    return ret as f32;
    // return 1;
  }
}

impl /*struct*/ QString {
  pub fn count<T: QString_count>(&mut self, value: T) -> i32 {
    return value.count(self);
    // return 1;
  }
}

pub trait QString_count {
  fn count(self, rsthis: &mut QString) -> i32;
}

// proto:  int QString::count(const QRegularExpression & re);
impl<'a> /*trait*/ QString_count for (&'a  QRegularExpression) {
  fn count(self, rsthis: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString5countERK18QRegularExpression()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QString5countERK18QRegularExpression(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QString {
  pub fn midRef<T: QString_midRef>(&mut self, value: T)  {
     value.midRef(self);
    // return 1;
  }
}

pub trait QString_midRef {
  fn midRef(self, rsthis: &mut QString) ;
}

// proto:  QStringRef QString::midRef(int position, int n);
impl<'a> /*trait*/ QString_midRef for (i32, i32) {
  fn midRef(self, rsthis: &mut QString)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString6midRefEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZNK7QString6midRefEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QString {
  pub fn detach<T: QString_detach>(&mut self, value: T)  {
     value.detach(self);
    // return 1;
  }
}

pub trait QString_detach {
  fn detach(self, rsthis: &mut QString) ;
}

// proto:  void QString::detach();
impl<'a> /*trait*/ QString_detach for () {
  fn detach(self, rsthis: &mut QString)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString6detachEv()};
     unsafe {_ZN7QString6detachEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  QString QString::arg(const QString & a1, const QString & a2, const QString & a3, const QString & a4);
impl<'a> /*trait*/ QString_arg for (&'a  QString, &'a  QString, &'a  QString, &'a  QString) {
  fn arg(self, rsthis: &mut QString) -> QString {
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
impl<'a> /*trait*/ QString_arg for (i32, i32, i32, QChar) {
  fn arg(self, rsthis: &mut QString) -> QString {
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
impl<'a> /*trait*/ QString_count for () {
  fn count(self, rsthis: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString5countEv()};
    let mut ret = unsafe {_ZNK7QString5countEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto:  QString & QString::setNum(qulonglong , int base);
impl<'a> /*trait*/ QString_setNum for (u64, i32) {
  fn setNum(self, rsthis: &mut QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString6setNumEyi()};
    let arg0 = self.0  as uint64_t;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZN7QString6setNumEyi(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QString {
  pub fn fromStdWString<T: QString_fromStdWString>(&mut self, value: T) -> QString {
    return value.fromStdWString(self);
    // return 1;
  }
}

pub trait QString_fromStdWString {
  fn fromStdWString(self, rsthis: &mut QString) -> QString;
}

// proto: static QString QString::fromStdWString(const std::wstring & s);
impl<'a> /*trait*/ QString_fromStdWString for (&'a  i32) {
  fn fromStdWString(self, rsthis: &mut QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString14fromStdWStringERKi()};
    let arg0 = self  as *const c_int;
    let mut ret = unsafe {_ZN7QString14fromStdWStringERKi(arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QString {
  pub fn push_back<T: QString_push_back>(&mut self, value: T)  {
     value.push_back(self);
    // return 1;
  }
}

pub trait QString_push_back {
  fn push_back(self, rsthis: &mut QString) ;
}

// proto:  void QString::push_back(QChar c);
impl<'a> /*trait*/ QString_push_back for (QChar) {
  fn push_back(self, rsthis: &mut QString)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString9push_backE5QChar()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QString9push_backE5QChar(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  QString & QString::setNum(float , char f, int prec);
impl<'a> /*trait*/ QString_setNum for (f32, i8, i32) {
  fn setNum(self, rsthis: &mut QString) -> QString {
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
impl<'a> /*trait*/ QString_count for (&'a  QRegExp) {
  fn count(self, rsthis: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString5countERK7QRegExp()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QString5countERK7QRegExp(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QString {
  pub fn size<T: QString_size>(&mut self, value: T) -> i32 {
    return value.size(self);
    // return 1;
  }
}

pub trait QString_size {
  fn size(self, rsthis: &mut QString) -> i32;
}

// proto:  int QString::size();
impl<'a> /*trait*/ QString_size for () {
  fn size(self, rsthis: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString4sizeEv()};
    let mut ret = unsafe {_ZNK7QString4sizeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto:  QString & QString::insert(int i, const QChar * uc, int len);
impl<'a> /*trait*/ QString_insert for (i32, &'a  QChar, i32) {
  fn insert(self, rsthis: &mut QString) -> QString {
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

impl /*struct*/ QString {
  pub fn replace<T: QString_replace>(&mut self, value: T) -> QString {
    return value.replace(self);
    // return 1;
  }
}

pub trait QString_replace {
  fn replace(self, rsthis: &mut QString) -> QString;
}

// proto:  QString & QString::replace(int i, int len, const QChar * s, int slen);
impl<'a> /*trait*/ QString_replace for (i32, i32, &'a  QChar, i32) {
  fn replace(self, rsthis: &mut QString) -> QString {
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

impl /*struct*/ QString {
  pub fn fromRawData<T: QString_fromRawData>(&mut self, value: T) -> QString {
    return value.fromRawData(self);
    // return 1;
  }
}

pub trait QString_fromRawData {
  fn fromRawData(self, rsthis: &mut QString) -> QString;
}

// proto: static QString QString::fromRawData(const QChar * , int size);
impl<'a> /*trait*/ QString_fromRawData for (&'a  QChar, i32) {
  fn fromRawData(self, rsthis: &mut QString) -> QString {
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
impl<'a> /*trait*/ QString_insert for (i32, &'a  QString) {
  fn insert(self, rsthis: &mut QString) -> QString {
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
impl<'a> /*trait*/ QString_arg for (&'a  QString, &'a  QString, &'a  QString, &'a  QString, &'a  QString) {
  fn arg(self, rsthis: &mut QString) -> QString {
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

impl /*struct*/ QString {
  pub fn setRawData<T: QString_setRawData>(&mut self, value: T) -> QString {
    return value.setRawData(self);
    // return 1;
  }
}

pub trait QString_setRawData {
  fn setRawData(self, rsthis: &mut QString) -> QString;
}

// proto:  QString & QString::setRawData(const QChar * unicode, int size);
impl<'a> /*trait*/ QString_setRawData for (&'a  QChar, i32) {
  fn setRawData(self, rsthis: &mut QString) -> QString {
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
impl<'a> /*trait*/ QString_prepend for (&'a  QString) {
  fn prepend(self, rsthis: &mut QString) -> QString {
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
impl<'a> /*trait*/ QString_setNum for (i32, i32) {
  fn setNum(self, rsthis: &mut QString) -> QString {
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

impl /*struct*/ QString {
  pub fn toULong<T: QString_toULong>(&mut self, value: T) -> u32 {
    return value.toULong(self);
    // return 1;
  }
}

pub trait QString_toULong {
  fn toULong(self, rsthis: &mut QString) -> u32;
}

// proto:  unsigned long QString::toULong(bool * ok, int base);
impl<'a> /*trait*/ QString_toULong for (&'a mut i8, i32) {
  fn toULong(self, rsthis: &mut QString) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString7toULongEPbi()};
    let arg0 = self.0  as *mut int8_t;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK7QString7toULongEPbi(rsthis.qclsinst, arg0, arg1)};
    return ret as u32;
    // return 1;
  }
}

impl /*struct*/ QString {
  pub fn chop<T: QString_chop>(&mut self, value: T)  {
     value.chop(self);
    // return 1;
  }
}

pub trait QString_chop {
  fn chop(self, rsthis: &mut QString) ;
}

// proto:  void QString::chop(int n);
impl<'a> /*trait*/ QString_chop for (i32) {
  fn chop(self, rsthis: &mut QString)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString4chopEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN7QString4chopEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QString {
  pub fn fromUtf16<T: QString_fromUtf16>(&mut self, value: T) -> QString {
    return value.fromUtf16(self);
    // return 1;
  }
}

pub trait QString_fromUtf16 {
  fn fromUtf16(self, rsthis: &mut QString) -> QString;
}

// proto: static QString QString::fromUtf16(const ushort * , int size);
impl<'a> /*trait*/ QString_fromUtf16 for (&'a  u16, i32) {
  fn fromUtf16(self, rsthis: &mut QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString9fromUtf16EPKti()};
    let arg0 = self.0  as *const c_ushort;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZN7QString9fromUtf16EPKti(arg0, arg1)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QString {
  pub fn isDetached<T: QString_isDetached>(&mut self, value: T) -> i8 {
    return value.isDetached(self);
    // return 1;
  }
}

pub trait QString_isDetached {
  fn isDetached(self, rsthis: &mut QString) -> i8;
}

// proto:  bool QString::isDetached();
impl<'a> /*trait*/ QString_isDetached for () {
  fn isDetached(self, rsthis: &mut QString) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString10isDetachedEv()};
    let mut ret = unsafe {_ZNK7QString10isDetachedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto:  QString & QString::setNum(qlonglong , int base);
impl<'a> /*trait*/ QString_setNum for (i64, i32) {
  fn setNum(self, rsthis: &mut QString) -> QString {
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

impl /*struct*/ QString {
  pub fn mid<T: QString_mid>(&mut self, value: T) -> QString {
    return value.mid(self);
    // return 1;
  }
}

pub trait QString_mid {
  fn mid(self, rsthis: &mut QString) -> QString;
}

// proto:  QString QString::mid(int position, int n);
impl<'a> /*trait*/ QString_mid for (i32, i32) {
  fn mid(self, rsthis: &mut QString) -> QString {
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

impl /*struct*/ QString {
  pub fn fromLocal8Bit<T: QString_fromLocal8Bit>(&mut self, value: T) -> QString {
    return value.fromLocal8Bit(self);
    // return 1;
  }
}

pub trait QString_fromLocal8Bit {
  fn fromLocal8Bit(self, rsthis: &mut QString) -> QString;
}

// proto: static QString QString::fromLocal8Bit(const char * str, int size);
impl<'a> /*trait*/ QString_fromLocal8Bit for (&'a  String, i32) {
  fn fromLocal8Bit(self, rsthis: &mut QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString13fromLocal8BitEPKci()};
    let arg0 = self.0.as_ptr()  as *const c_char;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZN7QString13fromLocal8BitEPKci(arg0, arg1)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QString {
  pub fn swap<T: QString_swap>(&mut self, value: T)  {
     value.swap(self);
    // return 1;
  }
}

pub trait QString_swap {
  fn swap(self, rsthis: &mut QString) ;
}

// proto:  void QString::swap(QString & other);
impl<'a> /*trait*/ QString_swap for (&'a mut QString) {
  fn swap(self, rsthis: &mut QString)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QString4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QString {
  pub fn fromUtf8<T: QString_fromUtf8>(&mut self, value: T) -> QString {
    return value.fromUtf8(self);
    // return 1;
  }
}

pub trait QString_fromUtf8 {
  fn fromUtf8(self, rsthis: &mut QString) -> QString;
}

// proto: static QString QString::fromUtf8(const QByteArray & str);
impl<'a> /*trait*/ QString_fromUtf8 for (&'a  QByteArray) {
  fn fromUtf8(self, rsthis: &mut QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString8fromUtf8ERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN7QString8fromUtf8ERK10QByteArray(arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QString {
  pub fn fromUcs4<T: QString_fromUcs4>(&mut self, value: T) -> QString {
    return value.fromUcs4(self);
    // return 1;
  }
}

pub trait QString_fromUcs4 {
  fn fromUcs4(self, rsthis: &mut QString) -> QString;
}

// proto: static QString QString::fromUcs4(const char32_t * str, int size);
impl<'a> /*trait*/ QString_fromUcs4 for (&'a  String, i32) {
  fn fromUcs4(self, rsthis: &mut QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString8fromUcs4EPKDii()};
    let arg0 = self.0.as_ptr()  as *const int32_t;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZN7QString8fromUcs4EPKDii(arg0, arg1)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QString {
  pub fn leftJustified<T: QString_leftJustified>(&mut self, value: T) -> QString {
    return value.leftJustified(self);
    // return 1;
  }
}

pub trait QString_leftJustified {
  fn leftJustified(self, rsthis: &mut QString) -> QString;
}

// proto:  QString QString::leftJustified(int width, QChar fill, bool trunc);
impl<'a> /*trait*/ QString_leftJustified for (i32, QChar, i8) {
  fn leftJustified(self, rsthis: &mut QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString13leftJustifiedEi5QCharb()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as int8_t;
    let mut ret = unsafe {_ZNK7QString13leftJustifiedEi5QCharb(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QString {
  pub fn indexOf<T: QString_indexOf>(&mut self, value: T) -> i32 {
    return value.indexOf(self);
    // return 1;
  }
}

pub trait QString_indexOf {
  fn indexOf(self, rsthis: &mut QString) -> i32;
}

// proto:  int QString::indexOf(const QRegExp & , int from);
impl<'a> /*trait*/ QString_indexOf for (&'a  QRegExp, i32) {
  fn indexOf(self, rsthis: &mut QString) -> i32 {
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
impl<'a> /*trait*/ QString_push_back for (&'a  QString) {
  fn push_back(self, rsthis: &mut QString)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString9push_backERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QString9push_backERKS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  int QString::lastIndexOf(QRegExp & , int from);
impl<'a> /*trait*/ QString_lastIndexOf for (&'a mut QRegExp, i32) {
  fn lastIndexOf(self, rsthis: &mut QString) -> i32 {
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
impl<'a> /*trait*/ QString_arg for (&'a  QString, &'a  QString, &'a  QString) {
  fn arg(self, rsthis: &mut QString) -> QString {
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

impl /*struct*/ QString {
  pub fn utf16<T: QString_utf16>(&mut self, value: T)  {
     value.utf16(self);
    // return 1;
  }
}

pub trait QString_utf16 {
  fn utf16(self, rsthis: &mut QString) ;
}

// proto:  const ushort * QString::utf16();
impl<'a> /*trait*/ QString_utf16 for () {
  fn utf16(self, rsthis: &mut QString)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString5utf16Ev()};
     unsafe {_ZNK7QString5utf16Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QString {
  pub fn toInt<T: QString_toInt>(&mut self, value: T) -> i32 {
    return value.toInt(self);
    // return 1;
  }
}

pub trait QString_toInt {
  fn toInt(self, rsthis: &mut QString) -> i32;
}

// proto:  int QString::toInt(bool * ok, int base);
impl<'a> /*trait*/ QString_toInt for (&'a mut i8, i32) {
  fn toInt(self, rsthis: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString5toIntEPbi()};
    let arg0 = self.0  as *mut int8_t;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK7QString5toIntEPbi(rsthis.qclsinst, arg0, arg1)};
    return ret as i32;
    // return 1;
  }
}

// proto:  QString QString::arg(double a, int fieldWidth, char fmt, int prec, QChar fillChar);
impl<'a> /*trait*/ QString_arg for (f64, i32, i8, i32, QChar) {
  fn arg(self, rsthis: &mut QString) -> QString {
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

impl /*struct*/ QString {
  pub fn data<T: QString_data>(&mut self, value: T) -> QChar {
    return value.data(self);
    // return 1;
  }
}

pub trait QString_data {
  fn data(self, rsthis: &mut QString) -> QChar;
}

// proto:  QChar * QString::data();
impl<'a> /*trait*/ QString_data for () {
  fn data(self, rsthis: &mut QString) -> QChar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString4dataEv()};
    let mut ret = unsafe {_ZN7QString4dataEv(rsthis.qclsinst)};
    let mut ret1 = QChar{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QString & QString::setNum(uint , int base);
impl<'a> /*trait*/ QString_setNum for (u32, i32) {
  fn setNum(self, rsthis: &mut QString) -> QString {
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

impl /*struct*/ QString {
  pub fn localeAwareCompare<T: QString_localeAwareCompare>(&mut self, value: T) -> i32 {
    return value.localeAwareCompare(self);
    // return 1;
  }
}

pub trait QString_localeAwareCompare {
  fn localeAwareCompare(self, rsthis: &mut QString) -> i32;
}

// proto: static int QString::localeAwareCompare(const QString & s1, const QString & s2);
impl<'a> /*trait*/ QString_localeAwareCompare for (&'a  QString, &'a  QString) {
  fn localeAwareCompare(self, rsthis: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString18localeAwareCompareERKS_S1_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN7QString18localeAwareCompareERKS_S1_(arg0, arg1)};
    return ret as i32;
    // return 1;
  }
}

// proto: void QString::NewQString(const char * ch);
impl<'a> /*trait*/ QString_NewQString for (&'a  String) {
  fn NewQString(self) -> QString {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QStringC1EPKc()};
    let arg0 = self.as_ptr()  as *const c_char;
    unsafe {_ZN7QStringC1EPKc(qthis, arg0)};
    let rsthis = QString{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: static QString QString::fromUtf16(const char16_t * str, int size);
impl<'a> /*trait*/ QString_fromUtf16 for (&'a  String, i32) {
  fn fromUtf16(self, rsthis: &mut QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString9fromUtf16EPKDsi()};
    let arg0 = self.0.as_ptr()  as *const int16_t;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZN7QString9fromUtf16EPKDsi(arg0, arg1)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QString & QString::replace(const QRegExp & rx, const QString & after);
impl<'a> /*trait*/ QString_replace for (&'a  QRegExp, &'a  QString) {
  fn replace(self, rsthis: &mut QString) -> QString {
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

impl /*struct*/ QString {
  pub fn repeated<T: QString_repeated>(&mut self, value: T) -> QString {
    return value.repeated(self);
    // return 1;
  }
}

pub trait QString_repeated {
  fn repeated(self, rsthis: &mut QString) -> QString;
}

// proto:  QString QString::repeated(int times);
impl<'a> /*trait*/ QString_repeated for (i32) {
  fn repeated(self, rsthis: &mut QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString8repeatedEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK7QString8repeatedEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QString {
  pub fn setUtf16<T: QString_setUtf16>(&mut self, value: T) -> QString {
    return value.setUtf16(self);
    // return 1;
  }
}

pub trait QString_setUtf16 {
  fn setUtf16(self, rsthis: &mut QString) -> QString;
}

// proto:  QString & QString::setUtf16(const ushort * utf16, int size);
impl<'a> /*trait*/ QString_setUtf16 for (&'a  u16, i32) {
  fn setUtf16(self, rsthis: &mut QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString8setUtf16EPKti()};
    let arg0 = self.0  as *const c_ushort;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZN7QString8setUtf16EPKti(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QString {
  pub fn fromStdU32String<T: QString_fromStdU32String>(&mut self, value: T) -> QString {
    return value.fromStdU32String(self);
    // return 1;
  }
}

pub trait QString_fromStdU32String {
  fn fromStdU32String(self, rsthis: &mut QString) -> QString;
}

// proto: static QString QString::fromStdU32String(const std::u32string & s);
impl<'a> /*trait*/ QString_fromStdU32String for (&'a  i32) {
  fn fromStdU32String(self, rsthis: &mut QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString16fromStdU32StringERKi()};
    let arg0 = self  as *const c_int;
    let mut ret = unsafe {_ZN7QString16fromStdU32StringERKi(arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QString {
  pub fn clear<T: QString_clear>(&mut self, value: T)  {
     value.clear(self);
    // return 1;
  }
}

pub trait QString_clear {
  fn clear(self, rsthis: &mut QString) ;
}

// proto:  void QString::clear();
impl<'a> /*trait*/ QString_clear for () {
  fn clear(self, rsthis: &mut QString)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString5clearEv()};
     unsafe {_ZN7QString5clearEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QString {
  pub fn contains<T: QString_contains>(&mut self, value: T) -> i8 {
    return value.contains(self);
    // return 1;
  }
}

pub trait QString_contains {
  fn contains(self, rsthis: &mut QString) -> i8;
}

// proto:  bool QString::contains(const QRegExp & rx);
impl<'a> /*trait*/ QString_contains for (&'a  QRegExp) {
  fn contains(self, rsthis: &mut QString) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString8containsERK7QRegExp()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QString8containsERK7QRegExp(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QString {
  pub fn isSharedWith<T: QString_isSharedWith>(&mut self, value: T) -> i8 {
    return value.isSharedWith(self);
    // return 1;
  }
}

pub trait QString_isSharedWith {
  fn isSharedWith(self, rsthis: &mut QString) -> i8;
}

// proto:  bool QString::isSharedWith(const QString & other);
impl<'a> /*trait*/ QString_isSharedWith for (&'a  QString) {
  fn isSharedWith(self, rsthis: &mut QString) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString12isSharedWithERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QString12isSharedWithERKS_(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QString {
  pub fn fromLatin1<T: QString_fromLatin1>(&mut self, value: T) -> QString {
    return value.fromLatin1(self);
    // return 1;
  }
}

pub trait QString_fromLatin1 {
  fn fromLatin1(self, rsthis: &mut QString) -> QString;
}

// proto: static QString QString::fromLatin1(const QByteArray & str);
impl<'a> /*trait*/ QString_fromLatin1 for (&'a  QByteArray) {
  fn fromLatin1(self, rsthis: &mut QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString10fromLatin1ERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN7QString10fromLatin1ERK10QByteArray(arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QString {
  pub fn FreeQString<T: QString_FreeQString>(&mut self, value: T)  {
     value.FreeQString(self);
    // return 1;
  }
}

pub trait QString_FreeQString {
  fn FreeQString(self, rsthis: &mut QString) ;
}

// proto:  void QString::FreeQString();
impl<'a> /*trait*/ QString_FreeQString for () {
  fn FreeQString(self, rsthis: &mut QString)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QStringD0Ev()};
     unsafe {_ZN7QStringD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QString {
  pub fn remove<T: QString_remove>(&mut self, value: T) -> QString {
    return value.remove(self);
    // return 1;
  }
}

pub trait QString_remove {
  fn remove(self, rsthis: &mut QString) -> QString;
}

// proto:  QString & QString::remove(const QRegularExpression & re);
impl<'a> /*trait*/ QString_remove for (&'a  QRegularExpression) {
  fn remove(self, rsthis: &mut QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString6removeERK18QRegularExpression()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN7QString6removeERK18QRegularExpression(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QString {
  pub fn cend<T: QString_cend>(&mut self, value: T) -> QChar {
    return value.cend(self);
    // return 1;
  }
}

pub trait QString_cend {
  fn cend(self, rsthis: &mut QString) -> QChar;
}

// proto:  const QChar * QString::cend();
impl<'a> /*trait*/ QString_cend for () {
  fn cend(self, rsthis: &mut QString) -> QChar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString4cendEv()};
    let mut ret = unsafe {_ZNK7QString4cendEv(rsthis.qclsinst)};
    let mut ret1 = QChar{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QString {
  pub fn toHtmlEscaped<T: QString_toHtmlEscaped>(&mut self, value: T) -> QString {
    return value.toHtmlEscaped(self);
    // return 1;
  }
}

pub trait QString_toHtmlEscaped {
  fn toHtmlEscaped(self, rsthis: &mut QString) -> QString;
}

// proto:  QString QString::toHtmlEscaped();
impl<'a> /*trait*/ QString_toHtmlEscaped for () {
  fn toHtmlEscaped(self, rsthis: &mut QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString13toHtmlEscapedEv()};
    let mut ret = unsafe {_ZNK7QString13toHtmlEscapedEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QString & QString::append(const QByteArray & s);
impl<'a> /*trait*/ QString_append for (&'a  QByteArray) {
  fn append(self, rsthis: &mut QString) -> QString {
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
impl<'a> /*trait*/ QString_fromLatin1 for (&'a  String, i32) {
  fn fromLatin1(self, rsthis: &mut QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString10fromLatin1EPKci()};
    let arg0 = self.0.as_ptr()  as *const c_char;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZN7QString10fromLatin1EPKci(arg0, arg1)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  bool QString::contains(const QRegularExpression & re, QRegularExpressionMatch * match);
impl<'a> /*trait*/ QString_contains for (&'a  QRegularExpression, &'a mut QRegularExpressionMatch) {
  fn contains(self, rsthis: &mut QString) -> i8 {
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
impl<'a> /*trait*/ QString_indexOf for (&'a  QRegularExpression, i32, &'a mut QRegularExpressionMatch) {
  fn indexOf(self, rsthis: &mut QString) -> i32 {
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

// proto:  int QString::lastIndexOf(const QRegExp & , int from);
impl<'a> /*trait*/ QString_lastIndexOf for (&'a  QRegExp, i32) {
  fn lastIndexOf(self, rsthis: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString11lastIndexOfERK7QRegExpi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK7QString11lastIndexOfERK7QRegExpi(rsthis.qclsinst, arg0, arg1)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QString {
  pub fn toWCharArray<T: QString_toWCharArray>(&mut self, value: T) -> i32 {
    return value.toWCharArray(self);
    // return 1;
  }
}

pub trait QString_toWCharArray {
  fn toWCharArray(self, rsthis: &mut QString) -> i32;
}

// proto:  int QString::toWCharArray(wchar_t * array);
impl<'a> /*trait*/ QString_toWCharArray for (&'a mut String) {
  fn toWCharArray(self, rsthis: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString12toWCharArrayEPw()};
    let arg0 = self.as_ptr()  as *mut wchar_t;
    let mut ret = unsafe {_ZNK7QString12toWCharArrayEPw(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QString {
  pub fn cbegin<T: QString_cbegin>(&mut self, value: T) -> QChar {
    return value.cbegin(self);
    // return 1;
  }
}

pub trait QString_cbegin {
  fn cbegin(self, rsthis: &mut QString) -> QChar;
}

// proto:  const QChar * QString::cbegin();
impl<'a> /*trait*/ QString_cbegin for () {
  fn cbegin(self, rsthis: &mut QString) -> QChar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString6cbeginEv()};
    let mut ret = unsafe {_ZNK7QString6cbeginEv(rsthis.qclsinst)};
    let mut ret1 = QChar{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QString & QString::prepend(const QByteArray & s);
impl<'a> /*trait*/ QString_prepend for (&'a  QByteArray) {
  fn prepend(self, rsthis: &mut QString) -> QString {
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
impl<'a> /*trait*/ QString_replace for (i32, i32, &'a  QString) {
  fn replace(self, rsthis: &mut QString) -> QString {
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

impl /*struct*/ QString {
  pub fn fromStdString<T: QString_fromStdString>(&mut self, value: T) -> QString {
    return value.fromStdString(self);
    // return 1;
  }
}

pub trait QString_fromStdString {
  fn fromStdString(self, rsthis: &mut QString) -> QString;
}

// proto: static QString QString::fromStdString(const std::string & s);
impl<'a> /*trait*/ QString_fromStdString for (&'a  i32) {
  fn fromStdString(self, rsthis: &mut QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString13fromStdStringERKi()};
    let arg0 = self  as *const c_int;
    let mut ret = unsafe {_ZN7QString13fromStdStringERKi(arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QString QString::arg(const QString & a1, const QString & a2, const QString & a3, const QString & a4, const QString & a5, const QString & a6, const QString & a7);
impl<'a> /*trait*/ QString_arg for (&'a  QString, &'a  QString, &'a  QString, &'a  QString, &'a  QString, &'a  QString, &'a  QString) {
  fn arg(self, rsthis: &mut QString) -> QString {
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

impl /*struct*/ QString {
  pub fn fromWCharArray<T: QString_fromWCharArray>(&mut self, value: T) -> QString {
    return value.fromWCharArray(self);
    // return 1;
  }
}

pub trait QString_fromWCharArray {
  fn fromWCharArray(self, rsthis: &mut QString) -> QString;
}

// proto: static QString QString::fromWCharArray(const wchar_t * string, int size);
impl<'a> /*trait*/ QString_fromWCharArray for (&'a  String, i32) {
  fn fromWCharArray(self, rsthis: &mut QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString14fromWCharArrayEPKwi()};
    let arg0 = self.0.as_ptr()  as *const wchar_t;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZN7QString14fromWCharArrayEPKwi(arg0, arg1)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QString {
  pub fn fill<T: QString_fill>(&mut self, value: T) -> QString {
    return value.fill(self);
    // return 1;
  }
}

pub trait QString_fill {
  fn fill(self, rsthis: &mut QString) -> QString;
}

// proto:  QString & QString::fill(QChar c, int size);
impl<'a> /*trait*/ QString_fill for (QChar, i32) {
  fn fill(self, rsthis: &mut QString) -> QString {
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

impl /*struct*/ QString {
  pub fn constData<T: QString_constData>(&mut self, value: T) -> QChar {
    return value.constData(self);
    // return 1;
  }
}

pub trait QString_constData {
  fn constData(self, rsthis: &mut QString) -> QChar;
}

// proto:  const QChar * QString::constData();
impl<'a> /*trait*/ QString_constData for () {
  fn constData(self, rsthis: &mut QString) -> QChar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString9constDataEv()};
    let mut ret = unsafe {_ZNK7QString9constDataEv(rsthis.qclsinst)};
    let mut ret1 = QChar{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: static QString QString::number(ulong , int base);
impl<'a> /*trait*/ QString_number for (u32, i32) {
  fn number(self, rsthis: &mut QString) -> QString {
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

impl /*struct*/ QString {
  pub fn toLong<T: QString_toLong>(&mut self, value: T) -> i32 {
    return value.toLong(self);
    // return 1;
  }
}

pub trait QString_toLong {
  fn toLong(self, rsthis: &mut QString) -> i32;
}

// proto:  long QString::toLong(bool * ok, int base);
impl<'a> /*trait*/ QString_toLong for (&'a mut i8, i32) {
  fn toLong(self, rsthis: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString6toLongEPbi()};
    let arg0 = self.0  as *mut int8_t;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK7QString6toLongEPbi(rsthis.qclsinst, arg0, arg1)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QString {
  pub fn constEnd<T: QString_constEnd>(&mut self, value: T) -> QChar {
    return value.constEnd(self);
    // return 1;
  }
}

pub trait QString_constEnd {
  fn constEnd(self, rsthis: &mut QString) -> QChar;
}

// proto:  const QChar * QString::constEnd();
impl<'a> /*trait*/ QString_constEnd for () {
  fn constEnd(self, rsthis: &mut QString) -> QChar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString8constEndEv()};
    let mut ret = unsafe {_ZNK7QString8constEndEv(rsthis.qclsinst)};
    let mut ret1 = QChar{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QString {
  pub fn length<T: QString_length>(&mut self, value: T) -> i32 {
    return value.length(self);
    // return 1;
  }
}

pub trait QString_length {
  fn length(self, rsthis: &mut QString) -> i32;
}

// proto:  int QString::length();
impl<'a> /*trait*/ QString_length for () {
  fn length(self, rsthis: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString6lengthEv()};
    let mut ret = unsafe {_ZNK7QString6lengthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto: static QString QString::fromUtf8(const char * str, int size);
impl<'a> /*trait*/ QString_fromUtf8 for (&'a  String, i32) {
  fn fromUtf8(self, rsthis: &mut QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString8fromUtf8EPKci()};
    let arg0 = self.0.as_ptr()  as *const c_char;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZN7QString8fromUtf8EPKci(arg0, arg1)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: static QString QString::number(qlonglong , int base);
impl<'a> /*trait*/ QString_number for (i64, i32) {
  fn number(self, rsthis: &mut QString) -> QString {
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

impl /*struct*/ QString {
  pub fn leftRef<T: QString_leftRef>(&mut self, value: T)  {
     value.leftRef(self);
    // return 1;
  }
}

pub trait QString_leftRef {
  fn leftRef(self, rsthis: &mut QString) ;
}

// proto:  QStringRef QString::leftRef(int n);
impl<'a> /*trait*/ QString_leftRef for (i32) {
  fn leftRef(self, rsthis: &mut QString)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString7leftRefEi()};
    let arg0 = self  as c_int;
     unsafe {_ZNK7QString7leftRefEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  QString QString::arg(const QString & a1, const QString & a2);
impl<'a> /*trait*/ QString_arg for (&'a  QString, &'a  QString) {
  fn arg(self, rsthis: &mut QString) -> QString {
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

impl /*struct*/ QString {
  pub fn isSimpleText<T: QString_isSimpleText>(&mut self, value: T) -> i8 {
    return value.isSimpleText(self);
    // return 1;
  }
}

pub trait QString_isSimpleText {
  fn isSimpleText(self, rsthis: &mut QString) -> i8;
}

// proto:  bool QString::isSimpleText();
impl<'a> /*trait*/ QString_isSimpleText for () {
  fn isSimpleText(self, rsthis: &mut QString) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString12isSimpleTextEv()};
    let mut ret = unsafe {_ZNK7QString12isSimpleTextEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto: static QString QString::fromUcs4(const uint * , int size);
impl<'a> /*trait*/ QString_fromUcs4 for (&'a  u32, i32) {
  fn fromUcs4(self, rsthis: &mut QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString8fromUcs4EPKji()};
    let arg0 = self.0  as *const c_uint;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZN7QString8fromUcs4EPKji(arg0, arg1)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QString {
  pub fn setUnicode<T: QString_setUnicode>(&mut self, value: T) -> QString {
    return value.setUnicode(self);
    // return 1;
  }
}

pub trait QString_setUnicode {
  fn setUnicode(self, rsthis: &mut QString) -> QString;
}

// proto:  QString & QString::setUnicode(const QChar * unicode, int size);
impl<'a> /*trait*/ QString_setUnicode for (&'a  QChar, i32) {
  fn setUnicode(self, rsthis: &mut QString) -> QString {
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

// proto:  bool QString::contains(QRegExp & rx);
impl<'a> /*trait*/ QString_contains for (&'a mut QRegExp) {
  fn contains(self, rsthis: &mut QString) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString8containsER7QRegExp()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QString8containsER7QRegExp(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QString {
  pub fn constBegin<T: QString_constBegin>(&mut self, value: T) -> QChar {
    return value.constBegin(self);
    // return 1;
  }
}

pub trait QString_constBegin {
  fn constBegin(self, rsthis: &mut QString) -> QChar;
}

// proto:  const QChar * QString::constBegin();
impl<'a> /*trait*/ QString_constBegin for () {
  fn constBegin(self, rsthis: &mut QString) -> QChar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString10constBeginEv()};
    let mut ret = unsafe {_ZNK7QString10constBeginEv(rsthis.qclsinst)};
    let mut ret1 = QChar{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QString {
  pub fn unicode<T: QString_unicode>(&mut self, value: T) -> QChar {
    return value.unicode(self);
    // return 1;
  }
}

pub trait QString_unicode {
  fn unicode(self, rsthis: &mut QString) -> QChar;
}

// proto:  const QChar * QString::unicode();
impl<'a> /*trait*/ QString_unicode for () {
  fn unicode(self, rsthis: &mut QString) -> QChar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString7unicodeEv()};
    let mut ret = unsafe {_ZNK7QString7unicodeEv(rsthis.qclsinst)};
    let mut ret1 = QChar{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QString QString::arg(const QString & a1, const QString & a2, const QString & a3, const QString & a4, const QString & a5, const QString & a6, const QString & a7, const QString & a8, const QString & a9);
impl<'a> /*trait*/ QString_arg for (&'a  QString, &'a  QString, &'a  QString, &'a  QString, &'a  QString, &'a  QString, &'a  QString, &'a  QString, &'a  QString) {
  fn arg(self, rsthis: &mut QString) -> QString {
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
impl<'a> /*trait*/ QString_indexOf for (&'a  QRegularExpression, i32) {
  fn indexOf(self, rsthis: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString7indexOfERK18QRegularExpressioni()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK7QString7indexOfERK18QRegularExpressioni(rsthis.qclsinst, arg0, arg1)};
    return ret as i32;
    // return 1;
  }
}

// proto: static QString QString::fromLocal8Bit(const QByteArray & str);
impl<'a> /*trait*/ QString_fromLocal8Bit for (&'a  QByteArray) {
  fn fromLocal8Bit(self, rsthis: &mut QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString13fromLocal8BitERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN7QString13fromLocal8BitERK10QByteArray(arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QString {
  pub fn at<T: QString_at>(&mut self, value: T) -> QChar {
    return value.at(self);
    // return 1;
  }
}

pub trait QString_at {
  fn at(self, rsthis: &mut QString) -> QChar;
}

// proto:  const QChar QString::at(int i);
impl<'a> /*trait*/ QString_at for (i32) {
  fn at(self, rsthis: &mut QString) -> QChar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString2atEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK7QString2atEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QChar{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: void QString::NewQString(int size, QChar c);
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
impl<'a> /*trait*/ QString_push_front for (&'a  QString) {
  fn push_front(self, rsthis: &mut QString)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString10push_frontERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QString10push_frontERKS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  QString QString::arg(const QString & a1, const QString & a2, const QString & a3, const QString & a4, const QString & a5, const QString & a6);
impl<'a> /*trait*/ QString_arg for (&'a  QString, &'a  QString, &'a  QString, &'a  QString, &'a  QString, &'a  QString) {
  fn arg(self, rsthis: &mut QString) -> QString {
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

impl /*struct*/ QString {
  pub fn begin<T: QString_begin>(&mut self, value: T) -> QChar {
    return value.begin(self);
    // return 1;
  }
}

pub trait QString_begin {
  fn begin(self, rsthis: &mut QString) -> QChar;
}

// proto:  QChar * QString::begin();
impl<'a> /*trait*/ QString_begin for () {
  fn begin(self, rsthis: &mut QString) -> QChar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString5beginEv()};
    let mut ret = unsafe {_ZN7QString5beginEv(rsthis.qclsinst)};
    let mut ret1 = QChar{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: static QString QString::number(double , char f, int prec);
impl<'a> /*trait*/ QString_number for (f64, i8, i32) {
  fn number(self, rsthis: &mut QString) -> QString {
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

impl /*struct*/ QString {
  pub fn end<T: QString_end>(&mut self, value: T) -> QChar {
    return value.end(self);
    // return 1;
  }
}

pub trait QString_end {
  fn end(self, rsthis: &mut QString) -> QChar;
}

// proto:  QChar * QString::end();
impl<'a> /*trait*/ QString_end for () {
  fn end(self, rsthis: &mut QString) -> QChar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString3endEv()};
    let mut ret = unsafe {_ZN7QString3endEv(rsthis.qclsinst)};
    let mut ret1 = QChar{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QString & QString::append(QChar c);
impl<'a> /*trait*/ QString_append for (QChar) {
  fn append(self, rsthis: &mut QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString6appendE5QChar()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN7QString6appendE5QChar(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QString {
  pub fn toUInt<T: QString_toUInt>(&mut self, value: T) -> u32 {
    return value.toUInt(self);
    // return 1;
  }
}

pub trait QString_toUInt {
  fn toUInt(self, rsthis: &mut QString) -> u32;
}

// proto:  unsigned int QString::toUInt(bool * ok, int base);
impl<'a> /*trait*/ QString_toUInt for (&'a mut i8, i32) {
  fn toUInt(self, rsthis: &mut QString) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString6toUIntEPbi()};
    let arg0 = self.0  as *mut int8_t;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK7QString6toUIntEPbi(rsthis.qclsinst, arg0, arg1)};
    return ret as u32;
    // return 1;
  }
}

// proto:  QString & QString::append(const QString & s);
impl<'a> /*trait*/ QString_append for (&'a  QString) {
  fn append(self, rsthis: &mut QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString6appendERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN7QString6appendERKS_(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QString {
  pub fn fromStdU16String<T: QString_fromStdU16String>(&mut self, value: T) -> QString {
    return value.fromStdU16String(self);
    // return 1;
  }
}

pub trait QString_fromStdU16String {
  fn fromStdU16String(self, rsthis: &mut QString) -> QString;
}

// proto: static QString QString::fromStdU16String(const std::u16string & s);
impl<'a> /*trait*/ QString_fromStdU16String for (&'a  i32) {
  fn fromStdU16String(self, rsthis: &mut QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString16fromStdU16StringERKi()};
    let arg0 = self  as *const c_int;
    let mut ret = unsafe {_ZN7QString16fromStdU16StringERKi(arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QString QString::arg(qlonglong a, int fieldwidth, int base, QChar fillChar);
impl<'a> /*trait*/ QString_arg for (i64, i32, i32, QChar) {
  fn arg(self, rsthis: &mut QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString3argExii5QChar()};
    let arg0 = self.0  as c_longlong;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QString3argExii5QChar(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QString {
  pub fn toUShort<T: QString_toUShort>(&mut self, value: T) -> u16 {
    return value.toUShort(self);
    // return 1;
  }
}

pub trait QString_toUShort {
  fn toUShort(self, rsthis: &mut QString) -> u16;
}

// proto:  unsigned short QString::toUShort(bool * ok, int base);
impl<'a> /*trait*/ QString_toUShort for (&'a mut i8, i32) {
  fn toUShort(self, rsthis: &mut QString) -> u16 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString8toUShortEPbi()};
    let arg0 = self.0  as *mut int8_t;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK7QString8toUShortEPbi(rsthis.qclsinst, arg0, arg1)};
    return ret as u16;
    // return 1;
  }
}

// proto:  QString QString::arg(uint a, int fieldWidth, int base, QChar fillChar);
impl<'a> /*trait*/ QString_arg for (u32, i32, i32, QChar) {
  fn arg(self, rsthis: &mut QString) -> QString {
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
impl<'a> /*trait*/ QString_setNum for (u16, i32) {
  fn setNum(self, rsthis: &mut QString) -> QString {
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
impl<'a> /*trait*/ QString_replace for (&'a  QRegularExpression, &'a  QString) {
  fn replace(self, rsthis: &mut QString) -> QString {
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
impl<'a> /*trait*/ QString_setNum for (f64, i8, i32) {
  fn setNum(self, rsthis: &mut QString) -> QString {
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

// proto: static QString QString::number(qulonglong , int base);
impl<'a> /*trait*/ QString_number for (u64, i32) {
  fn number(self, rsthis: &mut QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString6numberEyi()};
    let arg0 = self.0  as uint64_t;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZN7QString6numberEyi(arg0, arg1)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QString QString::arg(ushort a, int fieldWidth, int base, QChar fillChar);
impl<'a> /*trait*/ QString_arg for (u16, i32, i32, QChar) {
  fn arg(self, rsthis: &mut QString) -> QString {
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

// proto: void QString::NewQString(const QString & );
impl<'a> /*trait*/ QString_NewQString for (&'a  QString) {
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
impl<'a> /*trait*/ QString_arg for (i16, i32, i32, QChar) {
  fn arg(self, rsthis: &mut QString) -> QString {
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

// proto: void QString::NewQString(const QByteArray & a);
impl<'a> /*trait*/ QString_NewQString for (&'a  QByteArray) {
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

impl /*struct*/ QString {
  pub fn toULongLong<T: QString_toULongLong>(&mut self, value: T) -> u64 {
    return value.toULongLong(self);
    // return 1;
  }
}

pub trait QString_toULongLong {
  fn toULongLong(self, rsthis: &mut QString) -> u64;
}

// proto:  quint64 QString::toULongLong(bool * ok, int base);
impl<'a> /*trait*/ QString_toULongLong for (&'a mut i8, i32) {
  fn toULongLong(self, rsthis: &mut QString) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString11toULongLongEPbi()};
    let arg0 = self.0  as *mut int8_t;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK7QString11toULongLongEPbi(rsthis.qclsinst, arg0, arg1)};
    return ret as u64;
    // return 1;
  }
}

// proto:  QString & QString::append(const char * s);
impl<'a> /*trait*/ QString_append for (&'a  String) {
  fn append(self, rsthis: &mut QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString6appendEPKc()};
    let arg0 = self.as_ptr()  as *const c_char;
    let mut ret = unsafe {_ZN7QString6appendEPKc(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QString {
  pub fn capacity<T: QString_capacity>(&mut self, value: T) -> i32 {
    return value.capacity(self);
    // return 1;
  }
}

pub trait QString_capacity {
  fn capacity(self, rsthis: &mut QString) -> i32;
}

// proto:  int QString::capacity();
impl<'a> /*trait*/ QString_capacity for () {
  fn capacity(self, rsthis: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString8capacityEv()};
    let mut ret = unsafe {_ZNK7QString8capacityEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QString {
  pub fn squeeze<T: QString_squeeze>(&mut self, value: T)  {
     value.squeeze(self);
    // return 1;
  }
}

pub trait QString_squeeze {
  fn squeeze(self, rsthis: &mut QString) ;
}

// proto:  void QString::squeeze();
impl<'a> /*trait*/ QString_squeeze for () {
  fn squeeze(self, rsthis: &mut QString)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString7squeezeEv()};
     unsafe {_ZN7QString7squeezeEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QString {
  pub fn truncate<T: QString_truncate>(&mut self, value: T)  {
     value.truncate(self);
    // return 1;
  }
}

pub trait QString_truncate {
  fn truncate(self, rsthis: &mut QString) ;
}

// proto:  void QString::truncate(int pos);
impl<'a> /*trait*/ QString_truncate for (i32) {
  fn truncate(self, rsthis: &mut QString)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString8truncateEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN7QString8truncateEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  QString QString::arg(QChar a, int fieldWidth, QChar fillChar);
impl<'a> /*trait*/ QString_arg for (QChar, i32, QChar) {
  fn arg(self, rsthis: &mut QString) -> QString {
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
impl<'a> /*trait*/ QString_localeAwareCompare for (&'a  QString) {
  fn localeAwareCompare(self, rsthis: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString18localeAwareCompareERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QString18localeAwareCompareERKS_(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

// proto:  QString & QString::remove(const QRegExp & rx);
impl<'a> /*trait*/ QString_remove for (&'a  QRegExp) {
  fn remove(self, rsthis: &mut QString) -> QString {
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
impl<'a> /*trait*/ QString_contains for (&'a  QRegularExpression) {
  fn contains(self, rsthis: &mut QString) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString8containsERK18QRegularExpression()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QString8containsERK18QRegularExpression(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

// proto:  int QString::indexOf(QRegExp & , int from);
impl<'a> /*trait*/ QString_indexOf for (&'a mut QRegExp, i32) {
  fn indexOf(self, rsthis: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString7indexOfER7QRegExpi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK7QString7indexOfER7QRegExpi(rsthis.qclsinst, arg0, arg1)};
    return ret as i32;
    // return 1;
  }
}

// proto:  QString & QString::replace(int i, int len, QChar after);
impl<'a> /*trait*/ QString_replace for (i32, i32, QChar) {
  fn replace(self, rsthis: &mut QString) -> QString {
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

impl /*struct*/ QString {
  pub fn isRightToLeft<T: QString_isRightToLeft>(&mut self, value: T) -> i8 {
    return value.isRightToLeft(self);
    // return 1;
  }
}

pub trait QString_isRightToLeft {
  fn isRightToLeft(self, rsthis: &mut QString) -> i8;
}

// proto:  bool QString::isRightToLeft();
impl<'a> /*trait*/ QString_isRightToLeft for () {
  fn isRightToLeft(self, rsthis: &mut QString) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString13isRightToLeftEv()};
    let mut ret = unsafe {_ZNK7QString13isRightToLeftEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto:  QString QString::arg(char a, int fieldWidth, QChar fillChar);
impl<'a> /*trait*/ QString_arg for (i8, i32, QChar) {
  fn arg(self, rsthis: &mut QString) -> QString {
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

impl /*struct*/ QString {
  pub fn toUcs4<T: QString_toUcs4>(&mut self, value: T)  {
     value.toUcs4(self);
    // return 1;
  }
}

pub trait QString_toUcs4 {
  fn toUcs4(self, rsthis: &mut QString) ;
}

// proto:  QVector<uint> QString::toUcs4();
impl<'a> /*trait*/ QString_toUcs4 for () {
  fn toUcs4(self, rsthis: &mut QString)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString6toUcs4Ev()};
     unsafe {_ZNK7QString6toUcs4Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  QString & QString::remove(int i, int len);
impl<'a> /*trait*/ QString_remove for (i32, i32) {
  fn remove(self, rsthis: &mut QString) -> QString {
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
impl<'a> /*trait*/ QString_lastIndexOf for (&'a  QRegularExpression, i32, &'a mut QRegularExpressionMatch) {
  fn lastIndexOf(self, rsthis: &mut QString) -> i32 {
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

impl /*struct*/ QString {
  pub fn right<T: QString_right>(&mut self, value: T) -> QString {
    return value.right(self);
    // return 1;
  }
}

pub trait QString_right {
  fn right(self, rsthis: &mut QString) -> QString;
}

// proto:  QString QString::right(int n);
impl<'a> /*trait*/ QString_right for (i32) {
  fn right(self, rsthis: &mut QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString5rightEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK7QString5rightEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QString {
  pub fn rightJustified<T: QString_rightJustified>(&mut self, value: T) -> QString {
    return value.rightJustified(self);
    // return 1;
  }
}

pub trait QString_rightJustified {
  fn rightJustified(self, rsthis: &mut QString) -> QString;
}

// proto:  QString QString::rightJustified(int width, QChar fill, bool trunc);
impl<'a> /*trait*/ QString_rightJustified for (i32, QChar, i8) {
  fn rightJustified(self, rsthis: &mut QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString14rightJustifiedEi5QCharb()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as int8_t;
    let mut ret = unsafe {_ZNK7QString14rightJustifiedEi5QCharb(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QString QString::arg(const QString & a, int fieldWidth, QChar fillChar);
impl<'a> /*trait*/ QString_arg for (&'a  QString, i32, QChar) {
  fn arg(self, rsthis: &mut QString) -> QString {
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
impl<'a> /*trait*/ QString_arg for (u64, i32, i32, QChar) {
  fn arg(self, rsthis: &mut QString) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString3argEyii5QChar()};
    let arg0 = self.0  as uint64_t;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QString3argEyii5QChar(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QString {
  pub fn reserve<T: QString_reserve>(&mut self, value: T)  {
     value.reserve(self);
    // return 1;
  }
}

pub trait QString_reserve {
  fn reserve(self, rsthis: &mut QString) ;
}

// proto:  void QString::reserve(int size);
impl<'a> /*trait*/ QString_reserve for (i32) {
  fn reserve(self, rsthis: &mut QString)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString7reserveEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN7QString7reserveEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QString {
  pub fn toShort<T: QString_toShort>(&mut self, value: T) -> i16 {
    return value.toShort(self);
    // return 1;
  }
}

pub trait QString_toShort {
  fn toShort(self, rsthis: &mut QString) -> i16;
}

// proto:  short QString::toShort(bool * ok, int base);
impl<'a> /*trait*/ QString_toShort for (&'a mut i8, i32) {
  fn toShort(self, rsthis: &mut QString) -> i16 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString7toShortEPbi()};
    let arg0 = self.0  as *mut int8_t;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK7QString7toShortEPbi(rsthis.qclsinst, arg0, arg1)};
    return ret as i16;
    // return 1;
  }
}

