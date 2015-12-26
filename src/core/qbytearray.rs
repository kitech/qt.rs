// auto generated, do not modify.
// created: Sat Dec 26 10:52:38 2015
// src-file: /QtCore/qbytearray.h
// dst-file: /src/core/qbytearray.rs
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
// use super::qbytearray::QByteArray; // 773
use super::qstring::QString; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QByteRef_Class_Size() -> c_int;
  // proto:  void QByteRef::QByteRef(QByteArray & array, int idx);
  fn dector_ZN8QByteRefC1ER10QByteArrayi(arg0: *mut c_void, arg1: c_int) -> *mut c_void;
  fn _ZN8QByteRefC1ER10QByteArrayi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int);
  fn QByteArray_Class_Size() -> c_int;
  // proto:  QByteArray & QByteArray::insert(int i, char c);
  fn _ZN10QByteArray6insertEic(qthis: *mut c_void, arg0: c_int, arg1: c_char) -> *mut c_void;
  // proto:  const_iterator QByteArray::cend();
  fn _ZNK10QByteArray4cendEv(qthis: *mut c_void) -> *mut c_char;
  // proto:  int QByteArray::lastIndexOf(const QByteArray & a, int from);
  fn _ZNK10QByteArray11lastIndexOfERKS_i(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) -> c_int;
  // proto:  void QByteArray::push_front(const QByteArray & a);
  fn _ZN10QByteArray10push_frontERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  ulong QByteArray::toULong(bool * ok, int base);
  fn _ZNK10QByteArray7toULongEPbi(qthis: *mut c_void, arg0: *mut c_char, arg1: c_int) -> c_ulong;
  // proto:  QByteArray & QByteArray::replace(const char * before, const char * after);
  fn _ZN10QByteArray7replaceEPKcS1_(qthis: *mut c_void, arg0: *mut c_char, arg1: *mut c_char) -> *mut c_void;
  // proto:  QByteArray & QByteArray::replace(const QByteArray & before, const QByteArray & after);
  fn _ZN10QByteArray7replaceERKS_S1_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto: static QByteArray QByteArray::fromHex(const QByteArray & hexEncoded);
  fn _ZN10QByteArray7fromHexERKS_(arg0: *mut c_void) -> *mut c_void;
  // proto:  QByteArray & QByteArray::prepend(const char * s);
  fn _ZN10QByteArray7prependEPKc(qthis: *mut c_void, arg0: *mut c_char) -> *mut c_void;
  // proto:  int QByteArray::count(const QByteArray & a);
  fn _ZNK10QByteArray5countERKS_(qthis: *mut c_void, arg0: *mut c_void) -> c_int;
  // proto:  void QByteArray::~QByteArray();
  fn _ZN10QByteArrayD0Ev(qthis: *mut c_void);
  // proto:  iterator QByteArray::end();
  fn _ZN10QByteArray3endEv(qthis: *mut c_void) -> *mut c_char;
  // proto:  void QByteArray::QByteArray();
  fn dector_ZN10QByteArrayC1Ev() -> *mut c_void;
  fn _ZN10QByteArrayC1Ev(qthis: *mut c_void);
  // proto:  QByteArray & QByteArray::replace(const char * before, const QByteArray & after);
  fn _ZN10QByteArray7replaceEPKcRKS_(qthis: *mut c_void, arg0: *mut c_char, arg1: *mut c_void) -> *mut c_void;
  // proto:  float QByteArray::toFloat(bool * ok);
  fn _ZNK10QByteArray7toFloatEPb(qthis: *mut c_void, arg0: *mut c_char) -> c_float;
  // proto:  void QByteArray::truncate(int pos);
  fn _ZN10QByteArray8truncateEi(qthis: *mut c_void, arg0: c_int);
  // proto:  QByteArray QByteArray::toBase64();
  fn _ZNK10QByteArray8toBase64Ev(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QByteArray::isEmpty();
  fn _ZNK10QByteArray7isEmptyEv(qthis: *mut c_void) -> c_char;
  // proto:  QByteArray & QByteArray::insert(int i, const char * s, int len);
  fn _ZN10QByteArray6insertEiPKci(qthis: *mut c_void, arg0: c_int, arg1: *mut c_char, arg2: c_int) -> *mut c_void;
  // proto:  QByteArray & QByteArray::insert(int i, const QString & s);
  fn _ZN10QByteArray6insertEiRK7QString(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) -> *mut c_void;
  // proto:  void QByteArray::resize(int size);
  fn _ZN10QByteArray6resizeEi(qthis: *mut c_void, arg0: c_int);
  // proto:  QByteArray & QByteArray::replace(int index, int len, const char * s, int alen);
  fn _ZN10QByteArray7replaceEiiPKci(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *mut c_char, arg3: c_int) -> *mut c_void;
  // proto:  int QByteArray::lastIndexOf(const QString & s, int from);
  fn _ZNK10QByteArray11lastIndexOfERK7QStringi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) -> c_int;
  // proto:  QByteArray QByteArray::toHex();
  fn _ZNK10QByteArray5toHexEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QByteArray::indexOf(const char * c, int from);
  fn _ZNK10QByteArray7indexOfEPKci(qthis: *mut c_void, arg0: *mut c_char, arg1: c_int) -> c_int;
  // proto:  QByteArray & QByteArray::replace(char before, const QByteArray & after);
  fn _ZN10QByteArray7replaceEcRKS_(qthis: *mut c_void, arg0: c_char, arg1: *mut c_void) -> *mut c_void;
  // proto:  uint QByteArray::toUInt(bool * ok, int base);
  fn _ZNK10QByteArray6toUIntEPbi(qthis: *mut c_void, arg0: *mut c_char, arg1: c_int) -> c_uint;
  // proto: static QByteArray QByteArray::fromStdString(const std::string & s);
  fn _ZN10QByteArray13fromStdStringERKi(arg0: *mut c_int) -> *mut c_void;
  // proto:  bool QByteArray::isNull();
  fn _ZNK10QByteArray6isNullEv(qthis: *mut c_void) -> c_char;
  // proto:  void QByteArray::reserve(int size);
  fn _ZN10QByteArray7reserveEi(qthis: *mut c_void, arg0: c_int);
  // proto:  const_iterator QByteArray::cbegin();
  fn _ZNK10QByteArray6cbeginEv(qthis: *mut c_void) -> *mut c_char;
  // proto: static QByteArray QByteArray::fromRawData(const char * , int size);
  fn _ZN10QByteArray11fromRawDataEPKci(arg0: *mut c_char, arg1: c_int) -> *mut c_void;
  // proto:  bool QByteArray::contains(char c);
  fn _ZNK10QByteArray8containsEc(qthis: *mut c_void, arg0: c_char) -> c_char;
  // proto:  void QByteArray::QByteArray(int size, char c);
  fn dector_ZN10QByteArrayC1Eic(arg0: c_int, arg1: c_char) -> *mut c_void;
  fn _ZN10QByteArrayC1Eic(qthis: *mut c_void, arg0: c_int, arg1: c_char);
  // proto:  int QByteArray::indexOf(const QByteArray & a, int from);
  fn _ZNK10QByteArray7indexOfERKS_i(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) -> c_int;
  // proto:  long QByteArray::toLong(bool * ok, int base);
  fn _ZNK10QByteArray6toLongEPbi(qthis: *mut c_void, arg0: *mut c_char, arg1: c_int) -> c_long;
  // proto:  int QByteArray::indexOf(char c, int from);
  fn _ZNK10QByteArray7indexOfEci(qthis: *mut c_void, arg0: c_char, arg1: c_int) -> c_int;
  // proto:  char * QByteArray::data();
  fn _ZN10QByteArray4dataEv(qthis: *mut c_void) -> *mut c_char;
  // proto: static QByteArray QByteArray::number(double , char f, int prec);
  fn _ZN10QByteArray6numberEdci(arg0: c_double, arg1: c_char, arg2: c_int) -> *mut c_void;
  // proto:  int QByteArray::capacity();
  fn _ZNK10QByteArray8capacityEv(qthis: *mut c_void) -> c_int;
  // proto:  int QByteArray::count();
  fn _ZNK10QByteArray5countEv(qthis: *mut c_void) -> c_int;
  // proto: static QByteArray QByteArray::fromBase64(const QByteArray & base64);
  fn _ZN10QByteArray10fromBase64ERKS_(arg0: *mut c_void) -> *mut c_void;
  // proto:  QByteArray QByteArray::left(int len);
  fn _ZNK10QByteArray4leftEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  QByteArray & QByteArray::replace(char before, char after);
  fn _ZN10QByteArray7replaceEcc(qthis: *mut c_void, arg0: c_char, arg1: c_char) -> *mut c_void;
  // proto:  QByteArray & QByteArray::append(char c);
  fn _ZN10QByteArray6appendEc(qthis: *mut c_void, arg0: c_char) -> *mut c_void;
  // proto:  bool QByteArray::startsWith(const char * c);
  fn _ZNK10QByteArray10startsWithEPKc(qthis: *mut c_void, arg0: *mut c_char) -> c_char;
  // proto:  QByteArray & QByteArray::remove(int index, int len);
  fn _ZN10QByteArray6removeEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) -> *mut c_void;
  // proto:  int QByteArray::lastIndexOf(char c, int from);
  fn _ZNK10QByteArray11lastIndexOfEci(qthis: *mut c_void, arg0: c_char, arg1: c_int) -> c_int;
  // proto:  bool QByteArray::startsWith(const QByteArray & a);
  fn _ZNK10QByteArray10startsWithERKS_(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  bool QByteArray::contains(const char * a);
  fn _ZNK10QByteArray8containsEPKc(qthis: *mut c_void, arg0: *mut c_char) -> c_char;
  // proto:  bool QByteArray::endsWith(const char * c);
  fn _ZNK10QByteArray8endsWithEPKc(qthis: *mut c_void, arg0: *mut c_char) -> c_char;
  // proto:  void QByteArray::squeeze();
  fn _ZN10QByteArray7squeezeEv(qthis: *mut c_void);
  // proto:  void QByteArray::QByteArray(const char * , int size);
  fn dector_ZN10QByteArrayC1EPKci(arg0: *mut c_char, arg1: c_int) -> *mut c_void;
  fn _ZN10QByteArrayC1EPKci(qthis: *mut c_void, arg0: *mut c_char, arg1: c_int);
  // proto:  int QByteArray::indexOf(const QString & s, int from);
  fn _ZNK10QByteArray7indexOfERK7QStringi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) -> c_int;
  // proto:  void QByteArray::detach();
  fn _ZN10QByteArray6detachEv(qthis: *mut c_void);
  // proto:  QByteArray QByteArray::repeated(int times);
  fn _ZNK10QByteArray8repeatedEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  QByteArray & QByteArray::setNum(float , char f, int prec);
  fn _ZN10QByteArray6setNumEfci(qthis: *mut c_void, arg0: c_float, arg1: c_char, arg2: c_int) -> *mut c_void;
  // proto:  short QByteArray::toShort(bool * ok, int base);
  fn _ZNK10QByteArray7toShortEPbi(qthis: *mut c_void, arg0: *mut c_char, arg1: c_int) -> c_short;
  // proto:  QByteArray & QByteArray::prepend(char c);
  fn _ZN10QByteArray7prependEc(qthis: *mut c_void, arg0: c_char) -> *mut c_void;
  // proto:  int QByteArray::toInt(bool * ok, int base);
  fn _ZNK10QByteArray5toIntEPbi(qthis: *mut c_void, arg0: *mut c_char, arg1: c_int) -> c_int;
  // proto:  const_iterator QByteArray::constBegin();
  fn _ZNK10QByteArray10constBeginEv(qthis: *mut c_void) -> *mut c_char;
  // proto:  void QByteArray::push_back(char c);
  fn _ZN10QByteArray9push_backEc(qthis: *mut c_void, arg0: c_char);
  // proto:  bool QByteArray::isSharedWith(const QByteArray & other);
  fn _ZNK10QByteArray12isSharedWithERKS_(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  int QByteArray::size();
  fn _ZNK10QByteArray4sizeEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QByteArray::endsWith(char c);
  fn _ZNK10QByteArray8endsWithEc(qthis: *mut c_void, arg0: c_char) -> c_char;
  // proto: static QByteArray QByteArray::number(uint , int base);
  fn _ZN10QByteArray6numberEji(arg0: c_uint, arg1: c_int) -> *mut c_void;
  // proto:  void QByteArray::push_front(char c);
  fn _ZN10QByteArray10push_frontEc(qthis: *mut c_void, arg0: c_char);
  // proto:  QByteArray QByteArray::leftJustified(int width, char fill, bool truncate);
  fn _ZNK10QByteArray13leftJustifiedEicb(qthis: *mut c_void, arg0: c_int, arg1: c_char, arg2: c_char) -> *mut c_void;
  // proto:  iterator QByteArray::begin();
  fn _ZN10QByteArray5beginEv(qthis: *mut c_void) -> *mut c_char;
  // proto: static QByteArray QByteArray::number(qulonglong , int base);
  fn _ZN10QByteArray6numberEyi(arg0: c_ulonglong, arg1: c_int) -> *mut c_void;
  // proto:  int QByteArray::count(char c);
  fn _ZNK10QByteArray5countEc(qthis: *mut c_void, arg0: c_char) -> c_int;
  // proto:  double QByteArray::toDouble(bool * ok);
  fn _ZNK10QByteArray8toDoubleEPb(qthis: *mut c_void, arg0: *mut c_char) -> c_double;
  // proto:  QByteArray & QByteArray::replace(int index, int len, const QByteArray & s);
  fn _ZN10QByteArray7replaceEiiRKS_(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *mut c_void) -> *mut c_void;
  // proto:  QByteArray & QByteArray::setNum(short , int base);
  fn _ZN10QByteArray6setNumEsi(qthis: *mut c_void, arg0: c_short, arg1: c_int) -> *mut c_void;
  // proto:  QByteArray & QByteArray::prepend(const QByteArray & a);
  fn _ZN10QByteArray7prependERKS_(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  qulonglong QByteArray::toULongLong(bool * ok, int base);
  fn _ZNK10QByteArray11toULongLongEPbi(qthis: *mut c_void, arg0: *mut c_char, arg1: c_int) -> c_ulonglong;
  // proto:  QByteArray & QByteArray::replace(char c, const QString & after);
  fn _ZN10QByteArray7replaceEcRK7QString(qthis: *mut c_void, arg0: c_char, arg1: *mut c_void) -> *mut c_void;
  // proto: static QByteArray QByteArray::fromPercentEncoding(const QByteArray & pctEncoded, char percent);
  fn _ZN10QByteArray19fromPercentEncodingERKS_c(arg0: *mut c_void, arg1: c_char) -> *mut c_void;
  // proto:  void QByteArray::push_front(const char * c);
  fn _ZN10QByteArray10push_frontEPKc(qthis: *mut c_void, arg0: *mut c_char);
  // proto:  void QByteArray::clear();
  fn _ZN10QByteArray5clearEv(qthis: *mut c_void);
  // proto:  qlonglong QByteArray::toLongLong(bool * ok, int base);
  fn _ZNK10QByteArray10toLongLongEPbi(qthis: *mut c_void, arg0: *mut c_char, arg1: c_int) -> c_longlong;
  // proto:  QByteArray & QByteArray::prepend(const char * s, int len);
  fn _ZN10QByteArray7prependEPKci(qthis: *mut c_void, arg0: *mut c_char, arg1: c_int) -> *mut c_void;
  // proto:  const char * QByteArray::constData();
  fn _ZNK10QByteArray9constDataEv(qthis: *mut c_void) -> *mut c_char;
  // proto:  void QByteArray::QByteArray(const QByteArray & );
  fn dector_ZN10QByteArrayC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN10QByteArrayC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  int QByteArray::length();
  fn _ZNK10QByteArray6lengthEv(qthis: *mut c_void) -> c_int;
  // proto: static QByteArray QByteArray::number(int , int base);
  fn _ZN10QByteArray6numberEii(arg0: c_int, arg1: c_int) -> *mut c_void;
  // proto:  bool QByteArray::startsWith(char c);
  fn _ZNK10QByteArray10startsWithEc(qthis: *mut c_void, arg0: c_char) -> c_char;
  // proto:  QByteArray & QByteArray::setNum(double , char f, int prec);
  fn _ZN10QByteArray6setNumEdci(qthis: *mut c_void, arg0: c_double, arg1: c_char, arg2: c_int) -> *mut c_void;
  // proto: static QByteArray QByteArray::number(qlonglong , int base);
  fn _ZN10QByteArray6numberExi(arg0: c_longlong, arg1: c_int) -> *mut c_void;
  // proto:  char QByteArray::at(int i);
  fn _ZNK10QByteArray2atEi(qthis: *mut c_void, arg0: c_int) -> c_char;
  // proto:  QByteArray & QByteArray::setNum(ushort , int base);
  fn _ZN10QByteArray6setNumEti(qthis: *mut c_void, arg0: c_ushort, arg1: c_int) -> *mut c_void;
  // proto:  void QByteArray::swap(QByteArray & other);
  fn _ZN10QByteArray4swapERS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QByteArray & QByteArray::replace(const QString & before, const char * after);
  fn _ZN10QByteArray7replaceERK7QStringPKc(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_char) -> *mut c_void;
  // proto:  QByteArray & QByteArray::append(const QByteArray & a);
  fn _ZN10QByteArray6appendERKS_(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  bool QByteArray::endsWith(const QByteArray & a);
  fn _ZNK10QByteArray8endsWithERKS_(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  int QByteArray::count(const char * a);
  fn _ZNK10QByteArray5countEPKc(qthis: *mut c_void, arg0: *mut c_char) -> c_int;
  // proto:  QByteArray & QByteArray::replace(const char * before, int bsize, const char * after, int asize);
  fn _ZN10QByteArray7replaceEPKciS1_i(qthis: *mut c_void, arg0: *mut c_char, arg1: c_int, arg2: *mut c_char, arg3: c_int) -> *mut c_void;
  // proto:  QList<QByteArray> QByteArray::split(char sep);
  fn _ZNK10QByteArray5splitEc(qthis: *mut c_void, arg0: c_char);
  // proto:  QByteArray & QByteArray::setNum(qlonglong , int base);
  fn _ZN10QByteArray6setNumExi(qthis: *mut c_void, arg0: c_longlong, arg1: c_int) -> *mut c_void;
  // proto:  QByteArray & QByteArray::replace(char before, const char * after);
  fn _ZN10QByteArray7replaceEcPKc(qthis: *mut c_void, arg0: c_char, arg1: *mut c_char) -> *mut c_void;
  // proto:  QByteArray & QByteArray::append(const char * s);
  fn _ZN10QByteArray6appendEPKc(qthis: *mut c_void, arg0: *mut c_char) -> *mut c_void;
  // proto:  QByteArray QByteArray::right(int len);
  fn _ZNK10QByteArray5rightEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  QByteArray & QByteArray::append(const QString & s);
  fn _ZN10QByteArray6appendERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QByteArray::chop(int n);
  fn _ZN10QByteArray4chopEi(qthis: *mut c_void, arg0: c_int);
  // proto:  int QByteArray::lastIndexOf(const char * c, int from);
  fn _ZNK10QByteArray11lastIndexOfEPKci(qthis: *mut c_void, arg0: *mut c_char, arg1: c_int) -> c_int;
  // proto:  QByteArray & QByteArray::replace(int index, int len, const char * s);
  fn _ZN10QByteArray7replaceEiiPKc(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *mut c_char) -> *mut c_void;
  // proto:  void QByteArray::push_back(const QByteArray & a);
  fn _ZN10QByteArray9push_backERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QByteArray QByteArray::toPercentEncoding(const QByteArray & exclude, const QByteArray & include, char percent);
  fn _ZNK10QByteArray17toPercentEncodingERKS_S1_c(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: c_char) -> *mut c_void;
  // proto:  bool QByteArray::isDetached();
  fn _ZNK10QByteArray10isDetachedEv(qthis: *mut c_void) -> c_char;
  // proto:  QByteArray & QByteArray::append(const char * s, int len);
  fn _ZN10QByteArray6appendEPKci(qthis: *mut c_void, arg0: *mut c_char, arg1: c_int) -> *mut c_void;
  // proto:  const_iterator QByteArray::constEnd();
  fn _ZNK10QByteArray8constEndEv(qthis: *mut c_void) -> *mut c_char;
  // proto:  QByteArray & QByteArray::replace(const QByteArray & before, const char * after);
  fn _ZN10QByteArray7replaceERKS_PKc(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_char) -> *mut c_void;
  // proto:  QByteArray & QByteArray::setNum(qulonglong , int base);
  fn _ZN10QByteArray6setNumEyi(qthis: *mut c_void, arg0: c_ulonglong, arg1: c_int) -> *mut c_void;
  // proto:  QByteArray & QByteArray::setRawData(const char * a, uint n);
  fn _ZN10QByteArray10setRawDataEPKcj(qthis: *mut c_void, arg0: *mut c_char, arg1: c_uint) -> *mut c_void;
  // proto:  QByteArray & QByteArray::replace(const QString & before, const QByteArray & after);
  fn _ZN10QByteArray7replaceERK7QStringRKS_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  QByteArray & QByteArray::setNum(uint , int base);
  fn _ZN10QByteArray6setNumEji(qthis: *mut c_void, arg0: c_uint, arg1: c_int) -> *mut c_void;
  // proto:  QByteArray QByteArray::mid(int index, int len);
  fn _ZNK10QByteArray3midEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) -> *mut c_void;
  // proto:  QByteArray & QByteArray::setNum(int , int base);
  fn _ZN10QByteArray6setNumEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) -> *mut c_void;
  // proto:  QByteArray & QByteArray::insert(int i, const QByteArray & a);
  fn _ZN10QByteArray6insertEiRKS_(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) -> *mut c_void;
  // proto:  QByteArray & QByteArray::insert(int i, const char * s);
  fn _ZN10QByteArray6insertEiPKc(qthis: *mut c_void, arg0: c_int, arg1: *mut c_char) -> *mut c_void;
  // proto:  QByteArray & QByteArray::fill(char c, int size);
  fn _ZN10QByteArray4fillEci(qthis: *mut c_void, arg0: c_char, arg1: c_int) -> *mut c_void;
  // proto:  ushort QByteArray::toUShort(bool * ok, int base);
  fn _ZNK10QByteArray8toUShortEPbi(qthis: *mut c_void, arg0: *mut c_char, arg1: c_int) -> c_ushort;
  // proto:  void QByteArray::push_back(const char * c);
  fn _ZN10QByteArray9push_backEPKc(qthis: *mut c_void, arg0: *mut c_char);
  // proto:  QByteArray QByteArray::rightJustified(int width, char fill, bool truncate);
  fn _ZNK10QByteArray14rightJustifiedEicb(qthis: *mut c_void, arg0: c_int, arg1: c_char, arg2: c_char) -> *mut c_void;
  // proto:  bool QByteArray::contains(const QByteArray & a);
  fn _ZNK10QByteArray8containsERKS_(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  fn QByteArrayDataPtr_Class_Size() -> c_int;
} // <= ext block end

// body block begin =>
// class sizeof(QByteRef)=16
pub struct QByteRef {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

// class sizeof(QByteArray)=8
pub struct QByteArray {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

// class sizeof(QByteArrayDataPtr)=8
pub struct QByteArrayDataPtr {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QByteRef {
  pub fn inheritFrom(qthis: *mut c_void) -> QByteRef {
    return QByteRef{qclsinst: qthis};
  }
}
  // proto:  void QByteRef::QByteRef(QByteArray & array, int idx);
impl /*struct*/ QByteRef {
  pub fn New<T: QByteRef_New>(value: T) -> QByteRef {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QByteRef_New {
  fn New(self) -> QByteRef;
}

  // proto:  void QByteRef::QByteRef(QByteArray & array, int idx);
impl<'a> /*trait*/ QByteRef_New for (&'a QByteArray, i32) {
  fn New(self) -> QByteRef {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QByteRefC1ER10QByteArrayi()};
    let ctysz: c_int = unsafe{QByteRef_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    // unsafe {_ZN8QByteRefC1ER10QByteArrayi(qthis, arg0, arg1)};
    let qthis: *mut c_void = unsafe {dector_ZN8QByteRefC1ER10QByteArrayi(arg0, arg1)};
    let rsthis = QByteRef{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QByteArray {
  pub fn inheritFrom(qthis: *mut c_void) -> QByteArray {
    return QByteArray{qclsinst: qthis};
  }
}
  // proto:  QByteArray & QByteArray::insert(int i, char c);
impl /*struct*/ QByteArray {
  pub fn insert<RetType, T: QByteArray_insert<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.insert(self);
    // return 1;
  }
}

pub trait QByteArray_insert<RetType> {
  fn insert(self , rsthis: & QByteArray) -> RetType;
}

  // proto:  QByteArray & QByteArray::insert(int i, char c);
impl<'a> /*trait*/ QByteArray_insert<QByteArray> for (i32, i8) {
  fn insert(self , rsthis: & QByteArray) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray6insertEic()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_char;
    let mut ret = unsafe {_ZN10QByteArray6insertEic(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QByteArray::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  const_iterator QByteArray::cend();
impl /*struct*/ QByteArray {
  pub fn cend<RetType, T: QByteArray_cend<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.cend(self);
    // return 1;
  }
}

pub trait QByteArray_cend<RetType> {
  fn cend(self , rsthis: & QByteArray) -> RetType;
}

  // proto:  const_iterator QByteArray::cend();
impl<'a> /*trait*/ QByteArray_cend<String> for () {
  fn cend(self , rsthis: & QByteArray) -> String {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray4cendEv()};
    let mut ret = unsafe {_ZNK10QByteArray4cendEv(rsthis.qclsinst)};
    let slen = unsafe {strlen(ret as *const i8)} as usize;
    return unsafe{String::from_raw_parts(ret as *mut u8, slen, slen+1)};
    // return 1;
  }
}

  // proto:  int QByteArray::lastIndexOf(const QByteArray & a, int from);
impl /*struct*/ QByteArray {
  pub fn lastIndexOf<RetType, T: QByteArray_lastIndexOf<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.lastIndexOf(self);
    // return 1;
  }
}

pub trait QByteArray_lastIndexOf<RetType> {
  fn lastIndexOf(self , rsthis: & QByteArray) -> RetType;
}

  // proto:  int QByteArray::lastIndexOf(const QByteArray & a, int from);
impl<'a> /*trait*/ QByteArray_lastIndexOf<i32> for (&'a QByteArray, i32) {
  fn lastIndexOf(self , rsthis: & QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray11lastIndexOfERKS_i()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK10QByteArray11lastIndexOfERKS_i(rsthis.qclsinst, arg0, arg1)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QByteArray::push_front(const QByteArray & a);
impl /*struct*/ QByteArray {
  pub fn push_front<RetType, T: QByteArray_push_front<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.push_front(self);
    // return 1;
  }
}

pub trait QByteArray_push_front<RetType> {
  fn push_front(self , rsthis: & QByteArray) -> RetType;
}

  // proto:  void QByteArray::push_front(const QByteArray & a);
impl<'a> /*trait*/ QByteArray_push_front<()> for (&'a QByteArray) {
  fn push_front(self , rsthis: & QByteArray) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray10push_frontERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QByteArray10push_frontERKS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  ulong QByteArray::toULong(bool * ok, int base);
impl /*struct*/ QByteArray {
  pub fn toULong<RetType, T: QByteArray_toULong<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toULong(self);
    // return 1;
  }
}

pub trait QByteArray_toULong<RetType> {
  fn toULong(self , rsthis: & QByteArray) -> RetType;
}

  // proto:  ulong QByteArray::toULong(bool * ok, int base);
impl<'a> /*trait*/ QByteArray_toULong<u64> for (&'a mut Vec<i8>, i32) {
  fn toULong(self , rsthis: & QByteArray) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray7toULongEPbi()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK10QByteArray7toULongEPbi(rsthis.qclsinst, arg0, arg1)};
    return ret as u64;
    // return 1;
  }
}

  // proto:  QByteArray & QByteArray::replace(const char * before, const char * after);
impl /*struct*/ QByteArray {
  pub fn replace<RetType, T: QByteArray_replace<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.replace(self);
    // return 1;
  }
}

pub trait QByteArray_replace<RetType> {
  fn replace(self , rsthis: & QByteArray) -> RetType;
}

  // proto:  QByteArray & QByteArray::replace(const char * before, const char * after);
impl<'a> /*trait*/ QByteArray_replace<QByteArray> for (&'a  String, &'a  String) {
  fn replace(self , rsthis: & QByteArray) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray7replaceEPKcS1_()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1.as_ptr()  as *mut c_char;
    let mut ret = unsafe {_ZN10QByteArray7replaceEPKcS1_(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QByteArray::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QByteArray & QByteArray::replace(const QByteArray & before, const QByteArray & after);
impl<'a> /*trait*/ QByteArray_replace<QByteArray> for (&'a QByteArray, &'a QByteArray) {
  fn replace(self , rsthis: & QByteArray) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray7replaceERKS_S1_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN10QByteArray7replaceERKS_S1_(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QByteArray::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto: static QByteArray QByteArray::fromHex(const QByteArray & hexEncoded);
impl /*struct*/ QByteArray {
  pub fn fromHex_s<RetType, T: QByteArray_fromHex_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromHex_s();
    // return 1;
  }
}

pub trait QByteArray_fromHex_s<RetType> {
  fn fromHex_s(self ) -> RetType;
}

  // proto: static QByteArray QByteArray::fromHex(const QByteArray & hexEncoded);
impl<'a> /*trait*/ QByteArray_fromHex_s<QByteArray> for (&'a QByteArray) {
  fn fromHex_s(self ) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray7fromHexERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN10QByteArray7fromHexERKS_(arg0)};
    let mut ret1 = QByteArray::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QByteArray & QByteArray::prepend(const char * s);
impl /*struct*/ QByteArray {
  pub fn prepend<RetType, T: QByteArray_prepend<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.prepend(self);
    // return 1;
  }
}

pub trait QByteArray_prepend<RetType> {
  fn prepend(self , rsthis: & QByteArray) -> RetType;
}

  // proto:  QByteArray & QByteArray::prepend(const char * s);
impl<'a> /*trait*/ QByteArray_prepend<QByteArray> for (&'a  String) {
  fn prepend(self , rsthis: & QByteArray) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray7prependEPKc()};
    let arg0 = self.as_ptr()  as *mut c_char;
    let mut ret = unsafe {_ZN10QByteArray7prependEPKc(rsthis.qclsinst, arg0)};
    let mut ret1 = QByteArray::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  int QByteArray::count(const QByteArray & a);
impl /*struct*/ QByteArray {
  pub fn count<RetType, T: QByteArray_count<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.count(self);
    // return 1;
  }
}

pub trait QByteArray_count<RetType> {
  fn count(self , rsthis: & QByteArray) -> RetType;
}

  // proto:  int QByteArray::count(const QByteArray & a);
impl<'a> /*trait*/ QByteArray_count<i32> for (&'a QByteArray) {
  fn count(self , rsthis: & QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray5countERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK10QByteArray5countERKS_(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QByteArray::~QByteArray();
impl /*struct*/ QByteArray {
  pub fn Free<RetType, T: QByteArray_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QByteArray_Free<RetType> {
  fn Free(self , rsthis: & QByteArray) -> RetType;
}

  // proto:  void QByteArray::~QByteArray();
impl<'a> /*trait*/ QByteArray_Free<()> for () {
  fn Free(self , rsthis: & QByteArray) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArrayD0Ev()};
     unsafe {_ZN10QByteArrayD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  iterator QByteArray::end();
impl /*struct*/ QByteArray {
  pub fn end<RetType, T: QByteArray_end<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.end(self);
    // return 1;
  }
}

pub trait QByteArray_end<RetType> {
  fn end(self , rsthis: & QByteArray) -> RetType;
}

  // proto:  iterator QByteArray::end();
impl<'a> /*trait*/ QByteArray_end<String> for () {
  fn end(self , rsthis: & QByteArray) -> String {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray3endEv()};
    let mut ret = unsafe {_ZN10QByteArray3endEv(rsthis.qclsinst)};
    let slen = unsafe {strlen(ret as *const i8)} as usize;
    return unsafe{String::from_raw_parts(ret as *mut u8, slen, slen+1)};
    // return 1;
  }
}

  // proto:  void QByteArray::QByteArray();
impl /*struct*/ QByteArray {
  pub fn New<T: QByteArray_New>(value: T) -> QByteArray {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QByteArray_New {
  fn New(self) -> QByteArray;
}

  // proto:  void QByteArray::QByteArray();
impl<'a> /*trait*/ QByteArray_New for () {
  fn New(self) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArrayC1Ev()};
    let ctysz: c_int = unsafe{QByteArray_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    // unsafe {_ZN10QByteArrayC1Ev(qthis)};
    let qthis: *mut c_void = unsafe {dector_ZN10QByteArrayC1Ev()};
    let rsthis = QByteArray{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QByteArray & QByteArray::replace(const char * before, const QByteArray & after);
impl<'a> /*trait*/ QByteArray_replace<QByteArray> for (&'a  String, &'a QByteArray) {
  fn replace(self , rsthis: & QByteArray) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray7replaceEPKcRKS_()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN10QByteArray7replaceEPKcRKS_(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QByteArray::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  float QByteArray::toFloat(bool * ok);
impl /*struct*/ QByteArray {
  pub fn toFloat<RetType, T: QByteArray_toFloat<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toFloat(self);
    // return 1;
  }
}

pub trait QByteArray_toFloat<RetType> {
  fn toFloat(self , rsthis: & QByteArray) -> RetType;
}

  // proto:  float QByteArray::toFloat(bool * ok);
impl<'a> /*trait*/ QByteArray_toFloat<f32> for (&'a mut Vec<i8>) {
  fn toFloat(self , rsthis: & QByteArray) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray7toFloatEPb()};
    let arg0 = self.as_ptr()  as *mut c_char;
    let mut ret = unsafe {_ZNK10QByteArray7toFloatEPb(rsthis.qclsinst, arg0)};
    return ret as f32;
    // return 1;
  }
}

  // proto:  void QByteArray::truncate(int pos);
impl /*struct*/ QByteArray {
  pub fn truncate<RetType, T: QByteArray_truncate<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.truncate(self);
    // return 1;
  }
}

pub trait QByteArray_truncate<RetType> {
  fn truncate(self , rsthis: & QByteArray) -> RetType;
}

  // proto:  void QByteArray::truncate(int pos);
impl<'a> /*trait*/ QByteArray_truncate<()> for (i32) {
  fn truncate(self , rsthis: & QByteArray) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray8truncateEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN10QByteArray8truncateEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QByteArray QByteArray::toBase64();
impl /*struct*/ QByteArray {
  pub fn toBase64<RetType, T: QByteArray_toBase64<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toBase64(self);
    // return 1;
  }
}

pub trait QByteArray_toBase64<RetType> {
  fn toBase64(self , rsthis: & QByteArray) -> RetType;
}

  // proto:  QByteArray QByteArray::toBase64();
impl<'a> /*trait*/ QByteArray_toBase64<QByteArray> for () {
  fn toBase64(self , rsthis: & QByteArray) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray8toBase64Ev()};
    let mut ret = unsafe {_ZNK10QByteArray8toBase64Ev(rsthis.qclsinst)};
    let mut ret1 = QByteArray::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QByteArray::isEmpty();
impl /*struct*/ QByteArray {
  pub fn isEmpty<RetType, T: QByteArray_isEmpty<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isEmpty(self);
    // return 1;
  }
}

pub trait QByteArray_isEmpty<RetType> {
  fn isEmpty(self , rsthis: & QByteArray) -> RetType;
}

  // proto:  bool QByteArray::isEmpty();
impl<'a> /*trait*/ QByteArray_isEmpty<i8> for () {
  fn isEmpty(self , rsthis: & QByteArray) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray7isEmptyEv()};
    let mut ret = unsafe {_ZNK10QByteArray7isEmptyEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QByteArray & QByteArray::insert(int i, const char * s, int len);
impl<'a> /*trait*/ QByteArray_insert<QByteArray> for (i32, &'a  String, i32) {
  fn insert(self , rsthis: & QByteArray) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray6insertEiPKci()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.as_ptr()  as *mut c_char;
    let arg2 = self.2  as c_int;
    let mut ret = unsafe {_ZN10QByteArray6insertEiPKci(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QByteArray::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QByteArray & QByteArray::insert(int i, const QString & s);
impl<'a> /*trait*/ QByteArray_insert<QByteArray> for (i32, &'a QString) {
  fn insert(self , rsthis: & QByteArray) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray6insertEiRK7QString()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN10QByteArray6insertEiRK7QString(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QByteArray::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QByteArray::resize(int size);
impl /*struct*/ QByteArray {
  pub fn resize<RetType, T: QByteArray_resize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.resize(self);
    // return 1;
  }
}

pub trait QByteArray_resize<RetType> {
  fn resize(self , rsthis: & QByteArray) -> RetType;
}

  // proto:  void QByteArray::resize(int size);
impl<'a> /*trait*/ QByteArray_resize<()> for (i32) {
  fn resize(self , rsthis: & QByteArray) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray6resizeEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN10QByteArray6resizeEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QByteArray & QByteArray::replace(int index, int len, const char * s, int alen);
impl<'a> /*trait*/ QByteArray_replace<QByteArray> for (i32, i32, &'a  String, i32) {
  fn replace(self , rsthis: & QByteArray) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray7replaceEiiPKci()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.as_ptr()  as *mut c_char;
    let arg3 = self.3  as c_int;
    let mut ret = unsafe {_ZN10QByteArray7replaceEiiPKci(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    let mut ret1 = QByteArray::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  int QByteArray::lastIndexOf(const QString & s, int from);
impl<'a> /*trait*/ QByteArray_lastIndexOf<i32> for (&'a QString, i32) {
  fn lastIndexOf(self , rsthis: & QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray11lastIndexOfERK7QStringi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK10QByteArray11lastIndexOfERK7QStringi(rsthis.qclsinst, arg0, arg1)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QByteArray QByteArray::toHex();
impl /*struct*/ QByteArray {
  pub fn toHex<RetType, T: QByteArray_toHex<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toHex(self);
    // return 1;
  }
}

pub trait QByteArray_toHex<RetType> {
  fn toHex(self , rsthis: & QByteArray) -> RetType;
}

  // proto:  QByteArray QByteArray::toHex();
impl<'a> /*trait*/ QByteArray_toHex<QByteArray> for () {
  fn toHex(self , rsthis: & QByteArray) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray5toHexEv()};
    let mut ret = unsafe {_ZNK10QByteArray5toHexEv(rsthis.qclsinst)};
    let mut ret1 = QByteArray::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  int QByteArray::indexOf(const char * c, int from);
impl /*struct*/ QByteArray {
  pub fn indexOf<RetType, T: QByteArray_indexOf<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.indexOf(self);
    // return 1;
  }
}

pub trait QByteArray_indexOf<RetType> {
  fn indexOf(self , rsthis: & QByteArray) -> RetType;
}

  // proto:  int QByteArray::indexOf(const char * c, int from);
impl<'a> /*trait*/ QByteArray_indexOf<i32> for (&'a  String, i32) {
  fn indexOf(self , rsthis: & QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray7indexOfEPKci()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK10QByteArray7indexOfEPKci(rsthis.qclsinst, arg0, arg1)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QByteArray & QByteArray::replace(char before, const QByteArray & after);
impl<'a> /*trait*/ QByteArray_replace<QByteArray> for (i8, &'a QByteArray) {
  fn replace(self , rsthis: & QByteArray) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray7replaceEcRKS_()};
    let arg0 = self.0  as c_char;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN10QByteArray7replaceEcRKS_(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QByteArray::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  uint QByteArray::toUInt(bool * ok, int base);
impl /*struct*/ QByteArray {
  pub fn toUInt<RetType, T: QByteArray_toUInt<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toUInt(self);
    // return 1;
  }
}

pub trait QByteArray_toUInt<RetType> {
  fn toUInt(self , rsthis: & QByteArray) -> RetType;
}

  // proto:  uint QByteArray::toUInt(bool * ok, int base);
impl<'a> /*trait*/ QByteArray_toUInt<u32> for (&'a mut Vec<i8>, i32) {
  fn toUInt(self , rsthis: & QByteArray) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray6toUIntEPbi()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK10QByteArray6toUIntEPbi(rsthis.qclsinst, arg0, arg1)};
    return ret as u32;
    // return 1;
  }
}

  // proto: static QByteArray QByteArray::fromStdString(const std::string & s);
impl /*struct*/ QByteArray {
  pub fn fromStdString_s<RetType, T: QByteArray_fromStdString_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromStdString_s();
    // return 1;
  }
}

pub trait QByteArray_fromStdString_s<RetType> {
  fn fromStdString_s(self ) -> RetType;
}

  // proto: static QByteArray QByteArray::fromStdString(const std::string & s);
impl<'a> /*trait*/ QByteArray_fromStdString_s<QByteArray> for (&'a  String) {
  fn fromStdString_s(self ) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray13fromStdStringERKi()};
    let arg0 = self.as_ptr()  as *mut c_int;
    let mut ret = unsafe {_ZN10QByteArray13fromStdStringERKi(arg0)};
    let mut ret1 = QByteArray::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QByteArray::isNull();
impl /*struct*/ QByteArray {
  pub fn isNull<RetType, T: QByteArray_isNull<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isNull(self);
    // return 1;
  }
}

pub trait QByteArray_isNull<RetType> {
  fn isNull(self , rsthis: & QByteArray) -> RetType;
}

  // proto:  bool QByteArray::isNull();
impl<'a> /*trait*/ QByteArray_isNull<i8> for () {
  fn isNull(self , rsthis: & QByteArray) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray6isNullEv()};
    let mut ret = unsafe {_ZNK10QByteArray6isNullEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QByteArray::reserve(int size);
impl /*struct*/ QByteArray {
  pub fn reserve<RetType, T: QByteArray_reserve<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.reserve(self);
    // return 1;
  }
}

pub trait QByteArray_reserve<RetType> {
  fn reserve(self , rsthis: & QByteArray) -> RetType;
}

  // proto:  void QByteArray::reserve(int size);
impl<'a> /*trait*/ QByteArray_reserve<()> for (i32) {
  fn reserve(self , rsthis: & QByteArray) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray7reserveEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN10QByteArray7reserveEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  const_iterator QByteArray::cbegin();
impl /*struct*/ QByteArray {
  pub fn cbegin<RetType, T: QByteArray_cbegin<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.cbegin(self);
    // return 1;
  }
}

pub trait QByteArray_cbegin<RetType> {
  fn cbegin(self , rsthis: & QByteArray) -> RetType;
}

  // proto:  const_iterator QByteArray::cbegin();
impl<'a> /*trait*/ QByteArray_cbegin<String> for () {
  fn cbegin(self , rsthis: & QByteArray) -> String {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray6cbeginEv()};
    let mut ret = unsafe {_ZNK10QByteArray6cbeginEv(rsthis.qclsinst)};
    let slen = unsafe {strlen(ret as *const i8)} as usize;
    return unsafe{String::from_raw_parts(ret as *mut u8, slen, slen+1)};
    // return 1;
  }
}

  // proto: static QByteArray QByteArray::fromRawData(const char * , int size);
impl /*struct*/ QByteArray {
  pub fn fromRawData_s<RetType, T: QByteArray_fromRawData_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromRawData_s();
    // return 1;
  }
}

pub trait QByteArray_fromRawData_s<RetType> {
  fn fromRawData_s(self ) -> RetType;
}

  // proto: static QByteArray QByteArray::fromRawData(const char * , int size);
impl<'a> /*trait*/ QByteArray_fromRawData_s<QByteArray> for (&'a  String, i32) {
  fn fromRawData_s(self ) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray11fromRawDataEPKci()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZN10QByteArray11fromRawDataEPKci(arg0, arg1)};
    let mut ret1 = QByteArray::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QByteArray::contains(char c);
impl /*struct*/ QByteArray {
  pub fn contains<RetType, T: QByteArray_contains<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.contains(self);
    // return 1;
  }
}

pub trait QByteArray_contains<RetType> {
  fn contains(self , rsthis: & QByteArray) -> RetType;
}

  // proto:  bool QByteArray::contains(char c);
impl<'a> /*trait*/ QByteArray_contains<i8> for (i8) {
  fn contains(self , rsthis: & QByteArray) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray8containsEc()};
    let arg0 = self  as c_char;
    let mut ret = unsafe {_ZNK10QByteArray8containsEc(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QByteArray::QByteArray(int size, char c);
impl<'a> /*trait*/ QByteArray_New for (i32, i8) {
  fn New(self) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArrayC1Eic()};
    let ctysz: c_int = unsafe{QByteArray_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_char;
    // unsafe {_ZN10QByteArrayC1Eic(qthis, arg0, arg1)};
    let qthis: *mut c_void = unsafe {dector_ZN10QByteArrayC1Eic(arg0, arg1)};
    let rsthis = QByteArray{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  int QByteArray::indexOf(const QByteArray & a, int from);
impl<'a> /*trait*/ QByteArray_indexOf<i32> for (&'a QByteArray, i32) {
  fn indexOf(self , rsthis: & QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray7indexOfERKS_i()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK10QByteArray7indexOfERKS_i(rsthis.qclsinst, arg0, arg1)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  long QByteArray::toLong(bool * ok, int base);
impl /*struct*/ QByteArray {
  pub fn toLong<RetType, T: QByteArray_toLong<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toLong(self);
    // return 1;
  }
}

pub trait QByteArray_toLong<RetType> {
  fn toLong(self , rsthis: & QByteArray) -> RetType;
}

  // proto:  long QByteArray::toLong(bool * ok, int base);
impl<'a> /*trait*/ QByteArray_toLong<i64> for (&'a mut Vec<i8>, i32) {
  fn toLong(self , rsthis: & QByteArray) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray6toLongEPbi()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK10QByteArray6toLongEPbi(rsthis.qclsinst, arg0, arg1)};
    return ret as i64;
    // return 1;
  }
}

  // proto:  int QByteArray::indexOf(char c, int from);
impl<'a> /*trait*/ QByteArray_indexOf<i32> for (i8, i32) {
  fn indexOf(self , rsthis: & QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray7indexOfEci()};
    let arg0 = self.0  as c_char;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK10QByteArray7indexOfEci(rsthis.qclsinst, arg0, arg1)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  char * QByteArray::data();
impl /*struct*/ QByteArray {
  pub fn data<RetType, T: QByteArray_data<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.data(self);
    // return 1;
  }
}

pub trait QByteArray_data<RetType> {
  fn data(self , rsthis: & QByteArray) -> RetType;
}

  // proto:  char * QByteArray::data();
impl<'a> /*trait*/ QByteArray_data<String> for () {
  fn data(self , rsthis: & QByteArray) -> String {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray4dataEv()};
    let mut ret = unsafe {_ZN10QByteArray4dataEv(rsthis.qclsinst)};
    let slen = unsafe {strlen(ret as *const i8)} as usize;
    return unsafe{String::from_raw_parts(ret as *mut u8, slen, slen+1)};
    // return 1;
  }
}

  // proto: static QByteArray QByteArray::number(double , char f, int prec);
impl /*struct*/ QByteArray {
  pub fn number_s<RetType, T: QByteArray_number_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.number_s();
    // return 1;
  }
}

pub trait QByteArray_number_s<RetType> {
  fn number_s(self ) -> RetType;
}

  // proto: static QByteArray QByteArray::number(double , char f, int prec);
impl<'a> /*trait*/ QByteArray_number_s<QByteArray> for (f64, i8, i32) {
  fn number_s(self ) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray6numberEdci()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_char;
    let arg2 = self.2  as c_int;
    let mut ret = unsafe {_ZN10QByteArray6numberEdci(arg0, arg1, arg2)};
    let mut ret1 = QByteArray::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  int QByteArray::capacity();
impl /*struct*/ QByteArray {
  pub fn capacity<RetType, T: QByteArray_capacity<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.capacity(self);
    // return 1;
  }
}

pub trait QByteArray_capacity<RetType> {
  fn capacity(self , rsthis: & QByteArray) -> RetType;
}

  // proto:  int QByteArray::capacity();
impl<'a> /*trait*/ QByteArray_capacity<i32> for () {
  fn capacity(self , rsthis: & QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray8capacityEv()};
    let mut ret = unsafe {_ZNK10QByteArray8capacityEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QByteArray::count();
impl<'a> /*trait*/ QByteArray_count<i32> for () {
  fn count(self , rsthis: & QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray5countEv()};
    let mut ret = unsafe {_ZNK10QByteArray5countEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto: static QByteArray QByteArray::fromBase64(const QByteArray & base64);
impl /*struct*/ QByteArray {
  pub fn fromBase64_s<RetType, T: QByteArray_fromBase64_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromBase64_s();
    // return 1;
  }
}

pub trait QByteArray_fromBase64_s<RetType> {
  fn fromBase64_s(self ) -> RetType;
}

  // proto: static QByteArray QByteArray::fromBase64(const QByteArray & base64);
impl<'a> /*trait*/ QByteArray_fromBase64_s<QByteArray> for (&'a QByteArray) {
  fn fromBase64_s(self ) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray10fromBase64ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN10QByteArray10fromBase64ERKS_(arg0)};
    let mut ret1 = QByteArray::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QByteArray QByteArray::left(int len);
impl /*struct*/ QByteArray {
  pub fn left<RetType, T: QByteArray_left<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.left(self);
    // return 1;
  }
}

pub trait QByteArray_left<RetType> {
  fn left(self , rsthis: & QByteArray) -> RetType;
}

  // proto:  QByteArray QByteArray::left(int len);
impl<'a> /*trait*/ QByteArray_left<QByteArray> for (i32) {
  fn left(self , rsthis: & QByteArray) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray4leftEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK10QByteArray4leftEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QByteArray::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QByteArray & QByteArray::replace(char before, char after);
impl<'a> /*trait*/ QByteArray_replace<QByteArray> for (i8, i8) {
  fn replace(self , rsthis: & QByteArray) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray7replaceEcc()};
    let arg0 = self.0  as c_char;
    let arg1 = self.1  as c_char;
    let mut ret = unsafe {_ZN10QByteArray7replaceEcc(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QByteArray::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QByteArray & QByteArray::append(char c);
impl /*struct*/ QByteArray {
  pub fn append<RetType, T: QByteArray_append<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.append(self);
    // return 1;
  }
}

pub trait QByteArray_append<RetType> {
  fn append(self , rsthis: & QByteArray) -> RetType;
}

  // proto:  QByteArray & QByteArray::append(char c);
impl<'a> /*trait*/ QByteArray_append<QByteArray> for (i8) {
  fn append(self , rsthis: & QByteArray) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray6appendEc()};
    let arg0 = self  as c_char;
    let mut ret = unsafe {_ZN10QByteArray6appendEc(rsthis.qclsinst, arg0)};
    let mut ret1 = QByteArray::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QByteArray::startsWith(const char * c);
impl /*struct*/ QByteArray {
  pub fn startsWith<RetType, T: QByteArray_startsWith<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.startsWith(self);
    // return 1;
  }
}

pub trait QByteArray_startsWith<RetType> {
  fn startsWith(self , rsthis: & QByteArray) -> RetType;
}

  // proto:  bool QByteArray::startsWith(const char * c);
impl<'a> /*trait*/ QByteArray_startsWith<i8> for (&'a  String) {
  fn startsWith(self , rsthis: & QByteArray) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray10startsWithEPKc()};
    let arg0 = self.as_ptr()  as *mut c_char;
    let mut ret = unsafe {_ZNK10QByteArray10startsWithEPKc(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QByteArray & QByteArray::remove(int index, int len);
impl /*struct*/ QByteArray {
  pub fn remove<RetType, T: QByteArray_remove<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.remove(self);
    // return 1;
  }
}

pub trait QByteArray_remove<RetType> {
  fn remove(self , rsthis: & QByteArray) -> RetType;
}

  // proto:  QByteArray & QByteArray::remove(int index, int len);
impl<'a> /*trait*/ QByteArray_remove<QByteArray> for (i32, i32) {
  fn remove(self , rsthis: & QByteArray) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray6removeEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZN10QByteArray6removeEii(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QByteArray::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  int QByteArray::lastIndexOf(char c, int from);
impl<'a> /*trait*/ QByteArray_lastIndexOf<i32> for (i8, i32) {
  fn lastIndexOf(self , rsthis: & QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray11lastIndexOfEci()};
    let arg0 = self.0  as c_char;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK10QByteArray11lastIndexOfEci(rsthis.qclsinst, arg0, arg1)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  bool QByteArray::startsWith(const QByteArray & a);
impl<'a> /*trait*/ QByteArray_startsWith<i8> for (&'a QByteArray) {
  fn startsWith(self , rsthis: & QByteArray) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray10startsWithERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK10QByteArray10startsWithERKS_(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QByteArray::contains(const char * a);
impl<'a> /*trait*/ QByteArray_contains<i8> for (&'a  String) {
  fn contains(self , rsthis: & QByteArray) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray8containsEPKc()};
    let arg0 = self.as_ptr()  as *mut c_char;
    let mut ret = unsafe {_ZNK10QByteArray8containsEPKc(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QByteArray::endsWith(const char * c);
impl /*struct*/ QByteArray {
  pub fn endsWith<RetType, T: QByteArray_endsWith<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.endsWith(self);
    // return 1;
  }
}

pub trait QByteArray_endsWith<RetType> {
  fn endsWith(self , rsthis: & QByteArray) -> RetType;
}

  // proto:  bool QByteArray::endsWith(const char * c);
impl<'a> /*trait*/ QByteArray_endsWith<i8> for (&'a  String) {
  fn endsWith(self , rsthis: & QByteArray) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray8endsWithEPKc()};
    let arg0 = self.as_ptr()  as *mut c_char;
    let mut ret = unsafe {_ZNK10QByteArray8endsWithEPKc(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QByteArray::squeeze();
impl /*struct*/ QByteArray {
  pub fn squeeze<RetType, T: QByteArray_squeeze<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.squeeze(self);
    // return 1;
  }
}

pub trait QByteArray_squeeze<RetType> {
  fn squeeze(self , rsthis: & QByteArray) -> RetType;
}

  // proto:  void QByteArray::squeeze();
impl<'a> /*trait*/ QByteArray_squeeze<()> for () {
  fn squeeze(self , rsthis: & QByteArray) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray7squeezeEv()};
     unsafe {_ZN10QByteArray7squeezeEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QByteArray::QByteArray(const char * , int size);
impl<'a> /*trait*/ QByteArray_New for (&'a  String, i32) {
  fn New(self) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArrayC1EPKci()};
    let ctysz: c_int = unsafe{QByteArray_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1  as c_int;
    // unsafe {_ZN10QByteArrayC1EPKci(qthis, arg0, arg1)};
    let qthis: *mut c_void = unsafe {dector_ZN10QByteArrayC1EPKci(arg0, arg1)};
    let rsthis = QByteArray{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  int QByteArray::indexOf(const QString & s, int from);
impl<'a> /*trait*/ QByteArray_indexOf<i32> for (&'a QString, i32) {
  fn indexOf(self , rsthis: & QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray7indexOfERK7QStringi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK10QByteArray7indexOfERK7QStringi(rsthis.qclsinst, arg0, arg1)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QByteArray::detach();
impl /*struct*/ QByteArray {
  pub fn detach<RetType, T: QByteArray_detach<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.detach(self);
    // return 1;
  }
}

pub trait QByteArray_detach<RetType> {
  fn detach(self , rsthis: & QByteArray) -> RetType;
}

  // proto:  void QByteArray::detach();
impl<'a> /*trait*/ QByteArray_detach<()> for () {
  fn detach(self , rsthis: & QByteArray) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray6detachEv()};
     unsafe {_ZN10QByteArray6detachEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QByteArray QByteArray::repeated(int times);
impl /*struct*/ QByteArray {
  pub fn repeated<RetType, T: QByteArray_repeated<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.repeated(self);
    // return 1;
  }
}

pub trait QByteArray_repeated<RetType> {
  fn repeated(self , rsthis: & QByteArray) -> RetType;
}

  // proto:  QByteArray QByteArray::repeated(int times);
impl<'a> /*trait*/ QByteArray_repeated<QByteArray> for (i32) {
  fn repeated(self , rsthis: & QByteArray) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray8repeatedEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK10QByteArray8repeatedEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QByteArray::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QByteArray & QByteArray::setNum(float , char f, int prec);
impl /*struct*/ QByteArray {
  pub fn setNum<RetType, T: QByteArray_setNum<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setNum(self);
    // return 1;
  }
}

pub trait QByteArray_setNum<RetType> {
  fn setNum(self , rsthis: & QByteArray) -> RetType;
}

  // proto:  QByteArray & QByteArray::setNum(float , char f, int prec);
impl<'a> /*trait*/ QByteArray_setNum<QByteArray> for (f32, i8, i32) {
  fn setNum(self , rsthis: & QByteArray) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray6setNumEfci()};
    let arg0 = self.0  as c_float;
    let arg1 = self.1  as c_char;
    let arg2 = self.2  as c_int;
    let mut ret = unsafe {_ZN10QByteArray6setNumEfci(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QByteArray::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  short QByteArray::toShort(bool * ok, int base);
impl /*struct*/ QByteArray {
  pub fn toShort<RetType, T: QByteArray_toShort<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toShort(self);
    // return 1;
  }
}

pub trait QByteArray_toShort<RetType> {
  fn toShort(self , rsthis: & QByteArray) -> RetType;
}

  // proto:  short QByteArray::toShort(bool * ok, int base);
impl<'a> /*trait*/ QByteArray_toShort<i16> for (&'a mut Vec<i8>, i32) {
  fn toShort(self , rsthis: & QByteArray) -> i16 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray7toShortEPbi()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK10QByteArray7toShortEPbi(rsthis.qclsinst, arg0, arg1)};
    return ret as i16;
    // return 1;
  }
}

  // proto:  QByteArray & QByteArray::prepend(char c);
impl<'a> /*trait*/ QByteArray_prepend<QByteArray> for (i8) {
  fn prepend(self , rsthis: & QByteArray) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray7prependEc()};
    let arg0 = self  as c_char;
    let mut ret = unsafe {_ZN10QByteArray7prependEc(rsthis.qclsinst, arg0)};
    let mut ret1 = QByteArray::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  int QByteArray::toInt(bool * ok, int base);
impl /*struct*/ QByteArray {
  pub fn toInt<RetType, T: QByteArray_toInt<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toInt(self);
    // return 1;
  }
}

pub trait QByteArray_toInt<RetType> {
  fn toInt(self , rsthis: & QByteArray) -> RetType;
}

  // proto:  int QByteArray::toInt(bool * ok, int base);
impl<'a> /*trait*/ QByteArray_toInt<i32> for (&'a mut Vec<i8>, i32) {
  fn toInt(self , rsthis: & QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray5toIntEPbi()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK10QByteArray5toIntEPbi(rsthis.qclsinst, arg0, arg1)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  const_iterator QByteArray::constBegin();
impl /*struct*/ QByteArray {
  pub fn constBegin<RetType, T: QByteArray_constBegin<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.constBegin(self);
    // return 1;
  }
}

pub trait QByteArray_constBegin<RetType> {
  fn constBegin(self , rsthis: & QByteArray) -> RetType;
}

  // proto:  const_iterator QByteArray::constBegin();
impl<'a> /*trait*/ QByteArray_constBegin<String> for () {
  fn constBegin(self , rsthis: & QByteArray) -> String {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray10constBeginEv()};
    let mut ret = unsafe {_ZNK10QByteArray10constBeginEv(rsthis.qclsinst)};
    let slen = unsafe {strlen(ret as *const i8)} as usize;
    return unsafe{String::from_raw_parts(ret as *mut u8, slen, slen+1)};
    // return 1;
  }
}

  // proto:  void QByteArray::push_back(char c);
impl /*struct*/ QByteArray {
  pub fn push_back<RetType, T: QByteArray_push_back<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.push_back(self);
    // return 1;
  }
}

pub trait QByteArray_push_back<RetType> {
  fn push_back(self , rsthis: & QByteArray) -> RetType;
}

  // proto:  void QByteArray::push_back(char c);
impl<'a> /*trait*/ QByteArray_push_back<()> for (i8) {
  fn push_back(self , rsthis: & QByteArray) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray9push_backEc()};
    let arg0 = self  as c_char;
     unsafe {_ZN10QByteArray9push_backEc(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QByteArray::isSharedWith(const QByteArray & other);
impl /*struct*/ QByteArray {
  pub fn isSharedWith<RetType, T: QByteArray_isSharedWith<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isSharedWith(self);
    // return 1;
  }
}

pub trait QByteArray_isSharedWith<RetType> {
  fn isSharedWith(self , rsthis: & QByteArray) -> RetType;
}

  // proto:  bool QByteArray::isSharedWith(const QByteArray & other);
impl<'a> /*trait*/ QByteArray_isSharedWith<i8> for (&'a QByteArray) {
  fn isSharedWith(self , rsthis: & QByteArray) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray12isSharedWithERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK10QByteArray12isSharedWithERKS_(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  int QByteArray::size();
impl /*struct*/ QByteArray {
  pub fn size<RetType, T: QByteArray_size<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.size(self);
    // return 1;
  }
}

pub trait QByteArray_size<RetType> {
  fn size(self , rsthis: & QByteArray) -> RetType;
}

  // proto:  int QByteArray::size();
impl<'a> /*trait*/ QByteArray_size<i32> for () {
  fn size(self , rsthis: & QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray4sizeEv()};
    let mut ret = unsafe {_ZNK10QByteArray4sizeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  bool QByteArray::endsWith(char c);
impl<'a> /*trait*/ QByteArray_endsWith<i8> for (i8) {
  fn endsWith(self , rsthis: & QByteArray) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray8endsWithEc()};
    let arg0 = self  as c_char;
    let mut ret = unsafe {_ZNK10QByteArray8endsWithEc(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto: static QByteArray QByteArray::number(uint , int base);
impl<'a> /*trait*/ QByteArray_number_s<QByteArray> for (u32, i32) {
  fn number_s(self ) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray6numberEji()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZN10QByteArray6numberEji(arg0, arg1)};
    let mut ret1 = QByteArray::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QByteArray::push_front(char c);
impl<'a> /*trait*/ QByteArray_push_front<()> for (i8) {
  fn push_front(self , rsthis: & QByteArray) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray10push_frontEc()};
    let arg0 = self  as c_char;
     unsafe {_ZN10QByteArray10push_frontEc(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QByteArray QByteArray::leftJustified(int width, char fill, bool truncate);
impl /*struct*/ QByteArray {
  pub fn leftJustified<RetType, T: QByteArray_leftJustified<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.leftJustified(self);
    // return 1;
  }
}

pub trait QByteArray_leftJustified<RetType> {
  fn leftJustified(self , rsthis: & QByteArray) -> RetType;
}

  // proto:  QByteArray QByteArray::leftJustified(int width, char fill, bool truncate);
impl<'a> /*trait*/ QByteArray_leftJustified<QByteArray> for (i32, i8, i8) {
  fn leftJustified(self , rsthis: & QByteArray) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray13leftJustifiedEicb()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_char;
    let arg2 = self.2  as c_char;
    let mut ret = unsafe {_ZNK10QByteArray13leftJustifiedEicb(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QByteArray::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  iterator QByteArray::begin();
impl /*struct*/ QByteArray {
  pub fn begin<RetType, T: QByteArray_begin<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.begin(self);
    // return 1;
  }
}

pub trait QByteArray_begin<RetType> {
  fn begin(self , rsthis: & QByteArray) -> RetType;
}

  // proto:  iterator QByteArray::begin();
impl<'a> /*trait*/ QByteArray_begin<String> for () {
  fn begin(self , rsthis: & QByteArray) -> String {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray5beginEv()};
    let mut ret = unsafe {_ZN10QByteArray5beginEv(rsthis.qclsinst)};
    let slen = unsafe {strlen(ret as *const i8)} as usize;
    return unsafe{String::from_raw_parts(ret as *mut u8, slen, slen+1)};
    // return 1;
  }
}

  // proto: static QByteArray QByteArray::number(qulonglong , int base);
impl<'a> /*trait*/ QByteArray_number_s<QByteArray> for (u64, i32) {
  fn number_s(self ) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray6numberEyi()};
    let arg0 = self.0  as c_ulonglong;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZN10QByteArray6numberEyi(arg0, arg1)};
    let mut ret1 = QByteArray::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  int QByteArray::count(char c);
impl<'a> /*trait*/ QByteArray_count<i32> for (i8) {
  fn count(self , rsthis: & QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray5countEc()};
    let arg0 = self  as c_char;
    let mut ret = unsafe {_ZNK10QByteArray5countEc(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  double QByteArray::toDouble(bool * ok);
impl /*struct*/ QByteArray {
  pub fn toDouble<RetType, T: QByteArray_toDouble<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toDouble(self);
    // return 1;
  }
}

pub trait QByteArray_toDouble<RetType> {
  fn toDouble(self , rsthis: & QByteArray) -> RetType;
}

  // proto:  double QByteArray::toDouble(bool * ok);
impl<'a> /*trait*/ QByteArray_toDouble<f64> for (&'a mut Vec<i8>) {
  fn toDouble(self , rsthis: & QByteArray) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray8toDoubleEPb()};
    let arg0 = self.as_ptr()  as *mut c_char;
    let mut ret = unsafe {_ZNK10QByteArray8toDoubleEPb(rsthis.qclsinst, arg0)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  QByteArray & QByteArray::replace(int index, int len, const QByteArray & s);
impl<'a> /*trait*/ QByteArray_replace<QByteArray> for (i32, i32, &'a QByteArray) {
  fn replace(self , rsthis: & QByteArray) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray7replaceEiiRKS_()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN10QByteArray7replaceEiiRKS_(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QByteArray::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QByteArray & QByteArray::setNum(short , int base);
impl<'a> /*trait*/ QByteArray_setNum<QByteArray> for (i16, i32) {
  fn setNum(self , rsthis: & QByteArray) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray6setNumEsi()};
    let arg0 = self.0  as c_short;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZN10QByteArray6setNumEsi(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QByteArray::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QByteArray & QByteArray::prepend(const QByteArray & a);
impl<'a> /*trait*/ QByteArray_prepend<QByteArray> for (&'a QByteArray) {
  fn prepend(self , rsthis: & QByteArray) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray7prependERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN10QByteArray7prependERKS_(rsthis.qclsinst, arg0)};
    let mut ret1 = QByteArray::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  qulonglong QByteArray::toULongLong(bool * ok, int base);
impl /*struct*/ QByteArray {
  pub fn toULongLong<RetType, T: QByteArray_toULongLong<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toULongLong(self);
    // return 1;
  }
}

pub trait QByteArray_toULongLong<RetType> {
  fn toULongLong(self , rsthis: & QByteArray) -> RetType;
}

  // proto:  qulonglong QByteArray::toULongLong(bool * ok, int base);
impl<'a> /*trait*/ QByteArray_toULongLong<u64> for (&'a mut Vec<i8>, i32) {
  fn toULongLong(self , rsthis: & QByteArray) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray11toULongLongEPbi()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK10QByteArray11toULongLongEPbi(rsthis.qclsinst, arg0, arg1)};
    return ret as u64;
    // return 1;
  }
}

  // proto:  QByteArray & QByteArray::replace(char c, const QString & after);
impl<'a> /*trait*/ QByteArray_replace<QByteArray> for (i8, &'a QString) {
  fn replace(self , rsthis: & QByteArray) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray7replaceEcRK7QString()};
    let arg0 = self.0  as c_char;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN10QByteArray7replaceEcRK7QString(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QByteArray::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto: static QByteArray QByteArray::fromPercentEncoding(const QByteArray & pctEncoded, char percent);
impl /*struct*/ QByteArray {
  pub fn fromPercentEncoding_s<RetType, T: QByteArray_fromPercentEncoding_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromPercentEncoding_s();
    // return 1;
  }
}

pub trait QByteArray_fromPercentEncoding_s<RetType> {
  fn fromPercentEncoding_s(self ) -> RetType;
}

  // proto: static QByteArray QByteArray::fromPercentEncoding(const QByteArray & pctEncoded, char percent);
impl<'a> /*trait*/ QByteArray_fromPercentEncoding_s<QByteArray> for (&'a QByteArray, i8) {
  fn fromPercentEncoding_s(self ) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray19fromPercentEncodingERKS_c()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_char;
    let mut ret = unsafe {_ZN10QByteArray19fromPercentEncodingERKS_c(arg0, arg1)};
    let mut ret1 = QByteArray::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QByteArray::push_front(const char * c);
impl<'a> /*trait*/ QByteArray_push_front<()> for (&'a  String) {
  fn push_front(self , rsthis: & QByteArray) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray10push_frontEPKc()};
    let arg0 = self.as_ptr()  as *mut c_char;
     unsafe {_ZN10QByteArray10push_frontEPKc(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QByteArray::clear();
impl /*struct*/ QByteArray {
  pub fn clear<RetType, T: QByteArray_clear<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clear(self);
    // return 1;
  }
}

pub trait QByteArray_clear<RetType> {
  fn clear(self , rsthis: & QByteArray) -> RetType;
}

  // proto:  void QByteArray::clear();
impl<'a> /*trait*/ QByteArray_clear<()> for () {
  fn clear(self , rsthis: & QByteArray) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray5clearEv()};
     unsafe {_ZN10QByteArray5clearEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  qlonglong QByteArray::toLongLong(bool * ok, int base);
impl /*struct*/ QByteArray {
  pub fn toLongLong<RetType, T: QByteArray_toLongLong<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toLongLong(self);
    // return 1;
  }
}

pub trait QByteArray_toLongLong<RetType> {
  fn toLongLong(self , rsthis: & QByteArray) -> RetType;
}

  // proto:  qlonglong QByteArray::toLongLong(bool * ok, int base);
impl<'a> /*trait*/ QByteArray_toLongLong<i64> for (&'a mut Vec<i8>, i32) {
  fn toLongLong(self , rsthis: & QByteArray) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray10toLongLongEPbi()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK10QByteArray10toLongLongEPbi(rsthis.qclsinst, arg0, arg1)};
    return ret as i64;
    // return 1;
  }
}

  // proto:  QByteArray & QByteArray::prepend(const char * s, int len);
impl<'a> /*trait*/ QByteArray_prepend<QByteArray> for (&'a  String, i32) {
  fn prepend(self , rsthis: & QByteArray) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray7prependEPKci()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZN10QByteArray7prependEPKci(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QByteArray::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  const char * QByteArray::constData();
impl /*struct*/ QByteArray {
  pub fn constData<RetType, T: QByteArray_constData<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.constData(self);
    // return 1;
  }
}

pub trait QByteArray_constData<RetType> {
  fn constData(self , rsthis: & QByteArray) -> RetType;
}

  // proto:  const char * QByteArray::constData();
impl<'a> /*trait*/ QByteArray_constData<String> for () {
  fn constData(self , rsthis: & QByteArray) -> String {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray9constDataEv()};
    let mut ret = unsafe {_ZNK10QByteArray9constDataEv(rsthis.qclsinst)};
    let slen = unsafe {strlen(ret as *const i8)} as usize;
    return unsafe{String::from_raw_parts(ret as *mut u8, slen, slen+1)};
    // return 1;
  }
}

  // proto:  void QByteArray::QByteArray(const QByteArray & );
impl<'a> /*trait*/ QByteArray_New for (&'a QByteArray) {
  fn New(self) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArrayC1ERKS_()};
    let ctysz: c_int = unsafe{QByteArray_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN10QByteArrayC1ERKS_(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN10QByteArrayC1ERKS_(arg0)};
    let rsthis = QByteArray{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  int QByteArray::length();
impl /*struct*/ QByteArray {
  pub fn length<RetType, T: QByteArray_length<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.length(self);
    // return 1;
  }
}

pub trait QByteArray_length<RetType> {
  fn length(self , rsthis: & QByteArray) -> RetType;
}

  // proto:  int QByteArray::length();
impl<'a> /*trait*/ QByteArray_length<i32> for () {
  fn length(self , rsthis: & QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray6lengthEv()};
    let mut ret = unsafe {_ZNK10QByteArray6lengthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto: static QByteArray QByteArray::number(int , int base);
impl<'a> /*trait*/ QByteArray_number_s<QByteArray> for (i32, i32) {
  fn number_s(self ) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray6numberEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZN10QByteArray6numberEii(arg0, arg1)};
    let mut ret1 = QByteArray::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QByteArray::startsWith(char c);
impl<'a> /*trait*/ QByteArray_startsWith<i8> for (i8) {
  fn startsWith(self , rsthis: & QByteArray) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray10startsWithEc()};
    let arg0 = self  as c_char;
    let mut ret = unsafe {_ZNK10QByteArray10startsWithEc(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QByteArray & QByteArray::setNum(double , char f, int prec);
impl<'a> /*trait*/ QByteArray_setNum<QByteArray> for (f64, i8, i32) {
  fn setNum(self , rsthis: & QByteArray) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray6setNumEdci()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_char;
    let arg2 = self.2  as c_int;
    let mut ret = unsafe {_ZN10QByteArray6setNumEdci(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QByteArray::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto: static QByteArray QByteArray::number(qlonglong , int base);
impl<'a> /*trait*/ QByteArray_number_s<QByteArray> for (i64, i32) {
  fn number_s(self ) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray6numberExi()};
    let arg0 = self.0  as c_longlong;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZN10QByteArray6numberExi(arg0, arg1)};
    let mut ret1 = QByteArray::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  char QByteArray::at(int i);
impl /*struct*/ QByteArray {
  pub fn at<RetType, T: QByteArray_at<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.at(self);
    // return 1;
  }
}

pub trait QByteArray_at<RetType> {
  fn at(self , rsthis: & QByteArray) -> RetType;
}

  // proto:  char QByteArray::at(int i);
impl<'a> /*trait*/ QByteArray_at<i8> for (i32) {
  fn at(self , rsthis: & QByteArray) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray2atEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK10QByteArray2atEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QByteArray & QByteArray::setNum(ushort , int base);
impl<'a> /*trait*/ QByteArray_setNum<QByteArray> for (u16, i32) {
  fn setNum(self , rsthis: & QByteArray) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray6setNumEti()};
    let arg0 = self.0  as c_ushort;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZN10QByteArray6setNumEti(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QByteArray::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QByteArray::swap(QByteArray & other);
impl /*struct*/ QByteArray {
  pub fn swap<RetType, T: QByteArray_swap<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.swap(self);
    // return 1;
  }
}

pub trait QByteArray_swap<RetType> {
  fn swap(self , rsthis: & QByteArray) -> RetType;
}

  // proto:  void QByteArray::swap(QByteArray & other);
impl<'a> /*trait*/ QByteArray_swap<()> for (&'a QByteArray) {
  fn swap(self , rsthis: & QByteArray) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QByteArray4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QByteArray & QByteArray::replace(const QString & before, const char * after);
impl<'a> /*trait*/ QByteArray_replace<QByteArray> for (&'a QString, &'a  String) {
  fn replace(self , rsthis: & QByteArray) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray7replaceERK7QStringPKc()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *mut c_char;
    let mut ret = unsafe {_ZN10QByteArray7replaceERK7QStringPKc(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QByteArray::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QByteArray & QByteArray::append(const QByteArray & a);
impl<'a> /*trait*/ QByteArray_append<QByteArray> for (&'a QByteArray) {
  fn append(self , rsthis: & QByteArray) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray6appendERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN10QByteArray6appendERKS_(rsthis.qclsinst, arg0)};
    let mut ret1 = QByteArray::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QByteArray::endsWith(const QByteArray & a);
impl<'a> /*trait*/ QByteArray_endsWith<i8> for (&'a QByteArray) {
  fn endsWith(self , rsthis: & QByteArray) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray8endsWithERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK10QByteArray8endsWithERKS_(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  int QByteArray::count(const char * a);
impl<'a> /*trait*/ QByteArray_count<i32> for (&'a  String) {
  fn count(self , rsthis: & QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray5countEPKc()};
    let arg0 = self.as_ptr()  as *mut c_char;
    let mut ret = unsafe {_ZNK10QByteArray5countEPKc(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QByteArray & QByteArray::replace(const char * before, int bsize, const char * after, int asize);
impl<'a> /*trait*/ QByteArray_replace<QByteArray> for (&'a  String, i32, &'a  String, i32) {
  fn replace(self , rsthis: & QByteArray) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray7replaceEPKciS1_i()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.as_ptr()  as *mut c_char;
    let arg3 = self.3  as c_int;
    let mut ret = unsafe {_ZN10QByteArray7replaceEPKciS1_i(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    let mut ret1 = QByteArray::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QList<QByteArray> QByteArray::split(char sep);
impl /*struct*/ QByteArray {
  pub fn split<RetType, T: QByteArray_split<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.split(self);
    // return 1;
  }
}

pub trait QByteArray_split<RetType> {
  fn split(self , rsthis: & QByteArray) -> RetType;
}

  // proto:  QList<QByteArray> QByteArray::split(char sep);
impl<'a> /*trait*/ QByteArray_split<()> for (i8) {
  fn split(self , rsthis: & QByteArray) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray5splitEc()};
    let arg0 = self  as c_char;
     unsafe {_ZNK10QByteArray5splitEc(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QByteArray & QByteArray::setNum(qlonglong , int base);
impl<'a> /*trait*/ QByteArray_setNum<QByteArray> for (i64, i32) {
  fn setNum(self , rsthis: & QByteArray) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray6setNumExi()};
    let arg0 = self.0  as c_longlong;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZN10QByteArray6setNumExi(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QByteArray::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QByteArray & QByteArray::replace(char before, const char * after);
impl<'a> /*trait*/ QByteArray_replace<QByteArray> for (i8, &'a  String) {
  fn replace(self , rsthis: & QByteArray) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray7replaceEcPKc()};
    let arg0 = self.0  as c_char;
    let arg1 = self.1.as_ptr()  as *mut c_char;
    let mut ret = unsafe {_ZN10QByteArray7replaceEcPKc(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QByteArray::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QByteArray & QByteArray::append(const char * s);
impl<'a> /*trait*/ QByteArray_append<QByteArray> for (&'a  String) {
  fn append(self , rsthis: & QByteArray) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray6appendEPKc()};
    let arg0 = self.as_ptr()  as *mut c_char;
    let mut ret = unsafe {_ZN10QByteArray6appendEPKc(rsthis.qclsinst, arg0)};
    let mut ret1 = QByteArray::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QByteArray QByteArray::right(int len);
impl /*struct*/ QByteArray {
  pub fn right<RetType, T: QByteArray_right<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.right(self);
    // return 1;
  }
}

pub trait QByteArray_right<RetType> {
  fn right(self , rsthis: & QByteArray) -> RetType;
}

  // proto:  QByteArray QByteArray::right(int len);
impl<'a> /*trait*/ QByteArray_right<QByteArray> for (i32) {
  fn right(self , rsthis: & QByteArray) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray5rightEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK10QByteArray5rightEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QByteArray::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QByteArray & QByteArray::append(const QString & s);
impl<'a> /*trait*/ QByteArray_append<QByteArray> for (&'a QString) {
  fn append(self , rsthis: & QByteArray) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray6appendERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN10QByteArray6appendERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QByteArray::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QByteArray::chop(int n);
impl /*struct*/ QByteArray {
  pub fn chop<RetType, T: QByteArray_chop<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.chop(self);
    // return 1;
  }
}

pub trait QByteArray_chop<RetType> {
  fn chop(self , rsthis: & QByteArray) -> RetType;
}

  // proto:  void QByteArray::chop(int n);
impl<'a> /*trait*/ QByteArray_chop<()> for (i32) {
  fn chop(self , rsthis: & QByteArray) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray4chopEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN10QByteArray4chopEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QByteArray::lastIndexOf(const char * c, int from);
impl<'a> /*trait*/ QByteArray_lastIndexOf<i32> for (&'a  String, i32) {
  fn lastIndexOf(self , rsthis: & QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray11lastIndexOfEPKci()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK10QByteArray11lastIndexOfEPKci(rsthis.qclsinst, arg0, arg1)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QByteArray & QByteArray::replace(int index, int len, const char * s);
impl<'a> /*trait*/ QByteArray_replace<QByteArray> for (i32, i32, &'a  String) {
  fn replace(self , rsthis: & QByteArray) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray7replaceEiiPKc()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.as_ptr()  as *mut c_char;
    let mut ret = unsafe {_ZN10QByteArray7replaceEiiPKc(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QByteArray::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QByteArray::push_back(const QByteArray & a);
impl<'a> /*trait*/ QByteArray_push_back<()> for (&'a QByteArray) {
  fn push_back(self , rsthis: & QByteArray) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray9push_backERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QByteArray9push_backERKS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QByteArray QByteArray::toPercentEncoding(const QByteArray & exclude, const QByteArray & include, char percent);
impl /*struct*/ QByteArray {
  pub fn toPercentEncoding<RetType, T: QByteArray_toPercentEncoding<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toPercentEncoding(self);
    // return 1;
  }
}

pub trait QByteArray_toPercentEncoding<RetType> {
  fn toPercentEncoding(self , rsthis: & QByteArray) -> RetType;
}

  // proto:  QByteArray QByteArray::toPercentEncoding(const QByteArray & exclude, const QByteArray & include, char percent);
impl<'a> /*trait*/ QByteArray_toPercentEncoding<QByteArray> for (&'a QByteArray, &'a QByteArray, i8) {
  fn toPercentEncoding(self , rsthis: & QByteArray) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray17toPercentEncodingERKS_S1_c()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_char;
    let mut ret = unsafe {_ZNK10QByteArray17toPercentEncodingERKS_S1_c(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QByteArray::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QByteArray::isDetached();
impl /*struct*/ QByteArray {
  pub fn isDetached<RetType, T: QByteArray_isDetached<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isDetached(self);
    // return 1;
  }
}

pub trait QByteArray_isDetached<RetType> {
  fn isDetached(self , rsthis: & QByteArray) -> RetType;
}

  // proto:  bool QByteArray::isDetached();
impl<'a> /*trait*/ QByteArray_isDetached<i8> for () {
  fn isDetached(self , rsthis: & QByteArray) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray10isDetachedEv()};
    let mut ret = unsafe {_ZNK10QByteArray10isDetachedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QByteArray & QByteArray::append(const char * s, int len);
impl<'a> /*trait*/ QByteArray_append<QByteArray> for (&'a  String, i32) {
  fn append(self , rsthis: & QByteArray) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray6appendEPKci()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZN10QByteArray6appendEPKci(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QByteArray::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  const_iterator QByteArray::constEnd();
impl /*struct*/ QByteArray {
  pub fn constEnd<RetType, T: QByteArray_constEnd<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.constEnd(self);
    // return 1;
  }
}

pub trait QByteArray_constEnd<RetType> {
  fn constEnd(self , rsthis: & QByteArray) -> RetType;
}

  // proto:  const_iterator QByteArray::constEnd();
impl<'a> /*trait*/ QByteArray_constEnd<String> for () {
  fn constEnd(self , rsthis: & QByteArray) -> String {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray8constEndEv()};
    let mut ret = unsafe {_ZNK10QByteArray8constEndEv(rsthis.qclsinst)};
    let slen = unsafe {strlen(ret as *const i8)} as usize;
    return unsafe{String::from_raw_parts(ret as *mut u8, slen, slen+1)};
    // return 1;
  }
}

  // proto:  QByteArray & QByteArray::replace(const QByteArray & before, const char * after);
impl<'a> /*trait*/ QByteArray_replace<QByteArray> for (&'a QByteArray, &'a  String) {
  fn replace(self , rsthis: & QByteArray) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray7replaceERKS_PKc()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *mut c_char;
    let mut ret = unsafe {_ZN10QByteArray7replaceERKS_PKc(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QByteArray::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QByteArray & QByteArray::setNum(qulonglong , int base);
impl<'a> /*trait*/ QByteArray_setNum<QByteArray> for (u64, i32) {
  fn setNum(self , rsthis: & QByteArray) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray6setNumEyi()};
    let arg0 = self.0  as c_ulonglong;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZN10QByteArray6setNumEyi(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QByteArray::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QByteArray & QByteArray::setRawData(const char * a, uint n);
impl /*struct*/ QByteArray {
  pub fn setRawData<RetType, T: QByteArray_setRawData<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setRawData(self);
    // return 1;
  }
}

pub trait QByteArray_setRawData<RetType> {
  fn setRawData(self , rsthis: & QByteArray) -> RetType;
}

  // proto:  QByteArray & QByteArray::setRawData(const char * a, uint n);
impl<'a> /*trait*/ QByteArray_setRawData<QByteArray> for (&'a  String, u32) {
  fn setRawData(self , rsthis: & QByteArray) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray10setRawDataEPKcj()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1  as c_uint;
    let mut ret = unsafe {_ZN10QByteArray10setRawDataEPKcj(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QByteArray::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QByteArray & QByteArray::replace(const QString & before, const QByteArray & after);
impl<'a> /*trait*/ QByteArray_replace<QByteArray> for (&'a QString, &'a QByteArray) {
  fn replace(self , rsthis: & QByteArray) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray7replaceERK7QStringRKS_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN10QByteArray7replaceERK7QStringRKS_(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QByteArray::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QByteArray & QByteArray::setNum(uint , int base);
impl<'a> /*trait*/ QByteArray_setNum<QByteArray> for (u32, i32) {
  fn setNum(self , rsthis: & QByteArray) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray6setNumEji()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZN10QByteArray6setNumEji(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QByteArray::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QByteArray QByteArray::mid(int index, int len);
impl /*struct*/ QByteArray {
  pub fn mid<RetType, T: QByteArray_mid<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.mid(self);
    // return 1;
  }
}

pub trait QByteArray_mid<RetType> {
  fn mid(self , rsthis: & QByteArray) -> RetType;
}

  // proto:  QByteArray QByteArray::mid(int index, int len);
impl<'a> /*trait*/ QByteArray_mid<QByteArray> for (i32, i32) {
  fn mid(self , rsthis: & QByteArray) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray3midEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK10QByteArray3midEii(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QByteArray::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QByteArray & QByteArray::setNum(int , int base);
impl<'a> /*trait*/ QByteArray_setNum<QByteArray> for (i32, i32) {
  fn setNum(self , rsthis: & QByteArray) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray6setNumEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZN10QByteArray6setNumEii(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QByteArray::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QByteArray & QByteArray::insert(int i, const QByteArray & a);
impl<'a> /*trait*/ QByteArray_insert<QByteArray> for (i32, &'a QByteArray) {
  fn insert(self , rsthis: & QByteArray) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray6insertEiRKS_()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN10QByteArray6insertEiRKS_(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QByteArray::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QByteArray & QByteArray::insert(int i, const char * s);
impl<'a> /*trait*/ QByteArray_insert<QByteArray> for (i32, &'a  String) {
  fn insert(self , rsthis: & QByteArray) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray6insertEiPKc()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.as_ptr()  as *mut c_char;
    let mut ret = unsafe {_ZN10QByteArray6insertEiPKc(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QByteArray::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QByteArray & QByteArray::fill(char c, int size);
impl /*struct*/ QByteArray {
  pub fn fill<RetType, T: QByteArray_fill<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.fill(self);
    // return 1;
  }
}

pub trait QByteArray_fill<RetType> {
  fn fill(self , rsthis: & QByteArray) -> RetType;
}

  // proto:  QByteArray & QByteArray::fill(char c, int size);
impl<'a> /*trait*/ QByteArray_fill<QByteArray> for (i8, i32) {
  fn fill(self , rsthis: & QByteArray) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray4fillEci()};
    let arg0 = self.0  as c_char;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZN10QByteArray4fillEci(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QByteArray::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  ushort QByteArray::toUShort(bool * ok, int base);
impl /*struct*/ QByteArray {
  pub fn toUShort<RetType, T: QByteArray_toUShort<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toUShort(self);
    // return 1;
  }
}

pub trait QByteArray_toUShort<RetType> {
  fn toUShort(self , rsthis: & QByteArray) -> RetType;
}

  // proto:  ushort QByteArray::toUShort(bool * ok, int base);
impl<'a> /*trait*/ QByteArray_toUShort<u16> for (&'a mut Vec<i8>, i32) {
  fn toUShort(self , rsthis: & QByteArray) -> u16 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray8toUShortEPbi()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK10QByteArray8toUShortEPbi(rsthis.qclsinst, arg0, arg1)};
    return ret as u16;
    // return 1;
  }
}

  // proto:  void QByteArray::push_back(const char * c);
impl<'a> /*trait*/ QByteArray_push_back<()> for (&'a  String) {
  fn push_back(self , rsthis: & QByteArray) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray9push_backEPKc()};
    let arg0 = self.as_ptr()  as *mut c_char;
     unsafe {_ZN10QByteArray9push_backEPKc(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QByteArray QByteArray::rightJustified(int width, char fill, bool truncate);
impl /*struct*/ QByteArray {
  pub fn rightJustified<RetType, T: QByteArray_rightJustified<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rightJustified(self);
    // return 1;
  }
}

pub trait QByteArray_rightJustified<RetType> {
  fn rightJustified(self , rsthis: & QByteArray) -> RetType;
}

  // proto:  QByteArray QByteArray::rightJustified(int width, char fill, bool truncate);
impl<'a> /*trait*/ QByteArray_rightJustified<QByteArray> for (i32, i8, i8) {
  fn rightJustified(self , rsthis: & QByteArray) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray14rightJustifiedEicb()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_char;
    let arg2 = self.2  as c_char;
    let mut ret = unsafe {_ZNK10QByteArray14rightJustifiedEicb(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QByteArray::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QByteArray::contains(const QByteArray & a);
impl<'a> /*trait*/ QByteArray_contains<i8> for (&'a QByteArray) {
  fn contains(self , rsthis: & QByteArray) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray8containsERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK10QByteArray8containsERKS_(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QByteArrayDataPtr {
  pub fn inheritFrom(qthis: *mut c_void) -> QByteArrayDataPtr {
    return QByteArrayDataPtr{qclsinst: qthis};
  }
}
// <= body block end

