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
  fn _ZNK7QString10toLongLongEPbi(arg0: *mut int8_t, arg1: c_int) -> i32;
  fn _ZNK7QString6isNullEv() -> i32;
  fn _ZN7QString6appendEPK5QChari(arg0: *const c_void, arg1: c_int) -> i32;
  fn _ZN7QString7prependE5QChar(arg0: *mut c_void) -> i32;
  fn _ZN7QString6insertEi5QChar(arg0: c_int, arg1: *mut c_void) -> i32;
  fn _ZNK7QString4leftEi(arg0: c_int) -> i32;
  fn _ZN7QStringC1E5QChar(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  fn _ZNK7QString7isEmptyEv() -> i32;
  fn _ZN7QString7prependEPKc(arg0: *const c_char) -> i32;
  fn _ZNK7QString11lastIndexOfERK18QRegularExpressioni(arg0: *const c_void, arg1: c_int) -> i32;
  fn _ZN7QString6numberEii(arg0: c_int, arg1: c_int) -> i32;
  fn _ZN7QString6resizeEi(arg0: c_int) -> i32;
  fn _ZN7QString10push_frontE5QChar(arg0: *mut c_void) -> i32;
  fn _ZN7QStringC1Ev(qthis: *mut c_void) -> i32;
  fn _ZNK7QString8toDoubleEPb(arg0: *mut int8_t) -> i32;
  fn _ZNK7QString3argERKS_S1_S1_S1_S1_S1_S1_S1_(arg0: *const c_void, arg1: *const c_void, arg2: *const c_void, arg3: *const c_void, arg4: *const c_void, arg5: *const c_void, arg6: *const c_void, arg7: *const c_void) -> i32;
  fn _ZNK7QString8rightRefEi(arg0: c_int) -> i32;
  fn _ZN7QString6setNumEsi(arg0: c_short, arg1: c_int) -> i32;
  fn _ZN7QStringC1EPK5QChari(qthis: *mut c_void, arg0: *const c_void, arg1: c_int) -> i32;
  fn _ZNK7QString7sectionE5QCharii6QFlagsINS_11SectionFlagEE(arg0: *mut c_void, arg1: c_int, arg2: c_int, arg3: c_int) -> i32;
  fn _ZNK7QString7toFloatEPb(arg0: *mut int8_t) -> i32;
  fn _ZNK7QString5countERK18QRegularExpression(arg0: *const c_void) -> i32;
  fn _ZNK7QString6midRefEii(arg0: c_int, arg1: c_int) -> i32;
  fn _ZN7QString6detachEv() -> i32;
  fn _ZNK7QString3argERKS_S1_S1_S1_(arg0: *const c_void, arg1: *const c_void, arg2: *const c_void, arg3: *const c_void) -> i32;
  fn _ZNK7QString3argElii5QChar(arg0: c_long, arg1: c_int, arg2: c_int, arg3: *mut c_void) -> i32;
  fn _ZNK7QString5countEv() -> i32;
  fn _ZN7QString6setNumEyi(arg0: uint64_t, arg1: c_int) -> i32;
  fn _ZN7QString14fromStdWStringERKi(arg0: *const c_int) -> i32;
  fn _ZN7QString9push_backE5QChar(arg0: *mut c_void) -> i32;
  fn _ZN7QString6setNumEfci(arg0: c_float, arg1: c_char, arg2: c_int) -> i32;
  fn _ZNK7QString5countERK7QRegExp(arg0: *const c_void) -> i32;
  fn _ZNK7QString4sizeEv() -> i32;
  fn _ZN7QString6insertEiPK5QChari(arg0: c_int, arg1: *const c_void, arg2: c_int) -> i32;
  fn _ZN7QString7replaceEiiPK5QChari(arg0: c_int, arg1: c_int, arg2: *const c_void, arg3: c_int) -> i32;
  fn _ZN7QString11fromRawDataEPK5QChari(arg0: *const c_void, arg1: c_int) -> i32;
  fn _ZN7QString6insertEiRKS_(arg0: c_int, arg1: *const c_void) -> i32;
  fn _ZNK7QString3argERKS_S1_S1_S1_S1_(arg0: *const c_void, arg1: *const c_void, arg2: *const c_void, arg3: *const c_void, arg4: *const c_void) -> i32;
  fn _ZN7QString10setRawDataEPK5QChari(arg0: *const c_void, arg1: c_int) -> i32;
  fn _ZN7QString7prependERKS_(arg0: *const c_void) -> i32;
  fn _ZNK7QString7sectionERKS_ii6QFlagsINS_11SectionFlagEE(arg0: *const c_void, arg1: c_int, arg2: c_int, arg3: c_int) -> i32;
  fn _ZN7QString6setNumEii(arg0: c_int, arg1: c_int) -> i32;
  fn _ZNK7QString7toULongEPbi(arg0: *mut int8_t, arg1: c_int) -> i32;
  fn _ZN7QString4chopEi(arg0: c_int) -> i32;
  fn _ZN7QString9fromUtf16EPKti(arg0: *const c_ushort, arg1: c_int) -> i32;
  fn _ZNK7QString10isDetachedEv() -> i32;
  fn _ZN7QString6setNumExi(arg0: c_longlong, arg1: c_int) -> i32;
  fn _ZNK7QString3midEii(arg0: c_int, arg1: c_int) -> i32;
  fn _ZN7QString13fromLocal8BitEPKci(arg0: *const c_char, arg1: c_int) -> i32;
  fn _ZN7QString4swapERS_(arg0: *mut c_void) -> i32;
  fn _ZN7QString8fromUtf8ERK10QByteArray(arg0: *const c_void) -> i32;
  fn _ZN7QString8fromUcs4EPKDii(arg0: *const int32_t, arg1: c_int) -> i32;
  fn _ZNK7QString13leftJustifiedEi5QCharb(arg0: c_int, arg1: *mut c_void, arg2: int8_t) -> i32;
  fn _ZNK7QString7indexOfERK7QRegExpi(arg0: *const c_void, arg1: c_int) -> i32;
  fn _ZN7QString9push_backERKS_(arg0: *const c_void) -> i32;
  fn _ZNK7QString11lastIndexOfER7QRegExpi(arg0: *mut c_void, arg1: c_int) -> i32;
  fn _ZNK7QString3argERKS_S1_S1_(arg0: *const c_void, arg1: *const c_void, arg2: *const c_void) -> i32;
  fn _ZNK7QString5utf16Ev() -> i32;
  fn _ZNK7QString5toIntEPbi(arg0: *mut int8_t, arg1: c_int) -> i32;
  fn _ZNK7QString3argEdici5QChar(arg0: c_double, arg1: c_int, arg2: c_char, arg3: c_int, arg4: *mut c_void) -> i32;
  fn _ZN7QString4dataEv() -> i32;
  fn _ZN7QString6setNumEji(arg0: c_uint, arg1: c_int) -> i32;
  fn _ZN7QString18localeAwareCompareERKS_S1_(arg0: *const c_void, arg1: *const c_void) -> i32;
  fn _ZN7QStringC1EPKc(qthis: *mut c_void, arg0: *const c_char) -> i32;
  fn _ZN7QString9fromUtf16EPKDsi(arg0: *const int16_t, arg1: c_int) -> i32;
  fn _ZN7QString7replaceERK7QRegExpRKS_(arg0: *const c_void, arg1: *const c_void) -> i32;
  fn _ZNK7QString8repeatedEi(arg0: c_int) -> i32;
  fn _ZN7QString8setUtf16EPKti(arg0: *const c_ushort, arg1: c_int) -> i32;
  fn _ZN7QString16fromStdU32StringERKi(arg0: *const c_int) -> i32;
  fn _ZN7QString5clearEv() -> i32;
  fn _ZNK7QString8containsERK7QRegExp(arg0: *const c_void) -> i32;
  fn _ZNK7QString12isSharedWithERKS_(arg0: *const c_void) -> i32;
  fn _ZN7QString10fromLatin1ERK10QByteArray(arg0: *const c_void) -> i32;
  fn _ZN7QStringD0Ev() -> i32;
  fn _ZN7QString6removeERK18QRegularExpression(arg0: *const c_void) -> i32;
  fn _ZNK7QString4cendEv() -> i32;
  fn _ZNK7QString13toHtmlEscapedEv() -> i32;
  fn _ZN7QString6appendERK10QByteArray(arg0: *const c_void) -> i32;
  fn _ZN7QString10fromLatin1EPKci(arg0: *const c_char, arg1: c_int) -> i32;
  fn _ZNK7QString8containsERK18QRegularExpressionP23QRegularExpressionMatch(arg0: *const c_void, arg1: *mut c_void) -> i32;
  fn _ZNK7QString7indexOfERK18QRegularExpressioniP23QRegularExpressionMatch(arg0: *const c_void, arg1: c_int, arg2: *mut c_void) -> i32;
  fn _ZNK7QString11lastIndexOfERK7QRegExpi(arg0: *const c_void, arg1: c_int) -> i32;
  fn _ZNK7QString12toWCharArrayEPw(arg0: *mut wchar_t) -> i32;
  fn _ZNK7QString6cbeginEv() -> i32;
  fn _ZN7QString7prependERK10QByteArray(arg0: *const c_void) -> i32;
  fn _ZN7QString7replaceEiiRKS_(arg0: c_int, arg1: c_int, arg2: *const c_void) -> i32;
  fn _ZN7QString13fromStdStringERKi(arg0: *const c_int) -> i32;
  fn _ZNK7QString3argERKS_S1_S1_S1_S1_S1_S1_(arg0: *const c_void, arg1: *const c_void, arg2: *const c_void, arg3: *const c_void, arg4: *const c_void, arg5: *const c_void, arg6: *const c_void) -> i32;
  fn _ZN7QString14fromWCharArrayEPKwi(arg0: *const wchar_t, arg1: c_int) -> i32;
  fn _ZN7QString4fillE5QChari(arg0: *mut c_void, arg1: c_int) -> i32;
  fn _ZNK7QString9constDataEv() -> i32;
  fn _ZN7QString6numberEmi(arg0: c_ulong, arg1: c_int) -> i32;
  fn _ZNK7QString6toLongEPbi(arg0: *mut int8_t, arg1: c_int) -> i32;
  fn _ZNK7QString8constEndEv() -> i32;
  fn _ZNK7QString6lengthEv() -> i32;
  fn _ZN7QString8fromUtf8EPKci(arg0: *const c_char, arg1: c_int) -> i32;
  fn _ZNK7QString7sectionERK18QRegularExpressionii6QFlagsINS_11SectionFlagEE(arg0: *const c_void, arg1: c_int, arg2: c_int, arg3: c_int) -> i32;
  fn _ZN7QString6numberExi(arg0: c_longlong, arg1: c_int) -> i32;
  fn _ZNK7QString7leftRefEi(arg0: c_int) -> i32;
  fn _ZN7QString6setNumEli(arg0: c_long, arg1: c_int) -> i32;
  fn _ZNK7QString3argERKS_S1_(arg0: *const c_void, arg1: *const c_void) -> i32;
  fn _ZNK7QString12isSimpleTextEv() -> i32;
  fn _ZN7QString8fromUcs4EPKji(arg0: *const c_uint, arg1: c_int) -> i32;
  fn _ZN7QString10setUnicodeEPK5QChari(arg0: *const c_void, arg1: c_int) -> i32;
  fn _ZNK7QString8containsER7QRegExp(arg0: *mut c_void) -> i32;
  fn _ZNK7QString10constBeginEv() -> i32;
  fn _ZNK7QString7unicodeEv() -> i32;
  fn _ZNK7QString3argERKS_S1_S1_S1_S1_S1_S1_S1_S1_(arg0: *const c_void, arg1: *const c_void, arg2: *const c_void, arg3: *const c_void, arg4: *const c_void, arg5: *const c_void, arg6: *const c_void, arg7: *const c_void, arg8: *const c_void) -> i32;
  fn _ZNK7QString7indexOfERK18QRegularExpressioni(arg0: *const c_void, arg1: c_int) -> i32;
  fn _ZN7QString6numberEli(arg0: c_long, arg1: c_int) -> i32;
  fn _ZN7QString6numberEji(arg0: c_uint, arg1: c_int) -> i32;
  fn _ZN7QString13fromLocal8BitERK10QByteArray(arg0: *const c_void) -> i32;
  fn _ZNK7QString2atEi(arg0: c_int) -> i32;
  fn _ZN7QStringC1Ei5QChar(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) -> i32;
  fn _ZN7QString6setNumEmi(arg0: c_ulong, arg1: c_int) -> i32;
  fn _ZN7QString10push_frontERKS_(arg0: *const c_void) -> i32;
  fn _ZNK7QString7sectionERK7QRegExpii6QFlagsINS_11SectionFlagEE(arg0: *const c_void, arg1: c_int, arg2: c_int, arg3: c_int) -> i32;
  fn _ZNK7QString3argERKS_S1_S1_S1_S1_S1_(arg0: *const c_void, arg1: *const c_void, arg2: *const c_void, arg3: *const c_void, arg4: *const c_void, arg5: *const c_void) -> i32;
  fn _ZN7QString5beginEv() -> i32;
  fn _ZN7QString6numberEdci(arg0: c_double, arg1: c_char, arg2: c_int) -> i32;
  fn _ZN7QString3endEv() -> i32;
  fn _ZN7QString6appendE5QChar(arg0: *mut c_void) -> i32;
  fn _ZNK7QString6toUIntEPbi(arg0: *mut int8_t, arg1: c_int) -> i32;
  fn _ZN7QString6appendERKS_(arg0: *const c_void) -> i32;
  fn _ZN7QString16fromStdU16StringERKi(arg0: *const c_int) -> i32;
  fn _ZNK7QString3argExii5QChar(arg0: c_longlong, arg1: c_int, arg2: c_int, arg3: *mut c_void) -> i32;
  fn _ZNK7QString8toUShortEPbi(arg0: *mut int8_t, arg1: c_int) -> i32;
  fn _ZNK7QString3argEjii5QChar(arg0: c_uint, arg1: c_int, arg2: c_int, arg3: *mut c_void) -> i32;
  fn _ZN7QString6setNumEti(arg0: c_ushort, arg1: c_int) -> i32;
  fn _ZN7QString7replaceERK18QRegularExpressionRKS_(arg0: *const c_void, arg1: *const c_void) -> i32;
  fn _ZN7QString6setNumEdci(arg0: c_double, arg1: c_char, arg2: c_int) -> i32;
  fn _ZN7QString6numberEyi(arg0: uint64_t, arg1: c_int) -> i32;
  fn _ZNK7QString3argEtii5QChar(arg0: c_ushort, arg1: c_int, arg2: c_int, arg3: *mut c_void) -> i32;
  fn _ZN7QStringC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZNK7QString3argEsii5QChar(arg0: c_short, arg1: c_int, arg2: c_int, arg3: *mut c_void) -> i32;
  fn _ZN7QStringC1ERK10QByteArray(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZNK7QString11toULongLongEPbi(arg0: *mut int8_t, arg1: c_int) -> i32;
  fn _ZN7QString6appendEPKc(arg0: *const c_char) -> i32;
  fn _ZNK7QString8capacityEv() -> i32;
  fn _ZN7QString7squeezeEv() -> i32;
  fn _ZN7QString8truncateEi(arg0: c_int) -> i32;
  fn _ZNK7QString3argEiii5QChar(arg0: c_int, arg1: c_int, arg2: c_int, arg3: *mut c_void) -> i32;
  fn _ZNK7QString3argE5QChariS0_(arg0: *mut c_void, arg1: c_int, arg2: *mut c_void) -> i32;
  fn _ZNK7QString18localeAwareCompareERKS_(arg0: *const c_void) -> i32;
  fn _ZN7QString6removeERK7QRegExp(arg0: *const c_void) -> i32;
  fn _ZNK7QString8containsERK18QRegularExpression(arg0: *const c_void) -> i32;
  fn _ZNK7QString7indexOfER7QRegExpi(arg0: *mut c_void, arg1: c_int) -> i32;
  fn _ZN7QString7replaceEii5QChar(arg0: c_int, arg1: c_int, arg2: *mut c_void) -> i32;
  fn _ZNK7QString13isRightToLeftEv() -> i32;
  fn _ZNK7QString3argEci5QChar(arg0: c_char, arg1: c_int, arg2: *mut c_void) -> i32;
  fn _ZNK7QString6toUcs4Ev() -> i32;
  fn _ZN7QString6removeEii(arg0: c_int, arg1: c_int) -> i32;
  fn _ZNK7QString11lastIndexOfERK18QRegularExpressioniP23QRegularExpressionMatch(arg0: *const c_void, arg1: c_int, arg2: *mut c_void) -> i32;
  fn _ZNK7QString5rightEi(arg0: c_int) -> i32;
  fn _ZNK7QString14rightJustifiedEi5QCharb(arg0: c_int, arg1: *mut c_void, arg2: int8_t) -> i32;
  fn _ZNK7QString3argERKS_i5QChar(arg0: *const c_void, arg1: c_int, arg2: *mut c_void) -> i32;
  fn _ZNK7QString3argEyii5QChar(arg0: uint64_t, arg1: c_int, arg2: c_int, arg3: *mut c_void) -> i32;
  fn _ZN7QString7reserveEi(arg0: c_int) -> i32;
  fn _ZNK7QString7toShortEPbi(arg0: *mut int8_t, arg1: c_int) -> i32;
  fn _ZNK7QString3argEmii5QChar(arg0: c_ulong, arg1: c_int, arg2: c_int, arg3: *mut c_void) -> i32;
}

// body block begin
// class sizeof(QString)=8
pub struct QString {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QString {
  pub fn toLongLong<T: QString_toLongLong>(&mut self, value: T) -> i32 {
    value.toLongLong(self);
    return 1;
  }
}

pub trait QString_toLongLong {
  fn toLongLong(self, this: &mut QString) -> i32;
}

// proto: qint64 QString::toLongLong(bool * ok, int base);
impl<'a> /*trait*/ QString_toLongLong for (&'a mut i8, i32) {
  fn toLongLong(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString10toLongLongEPbi()};
    let arg0 = self.0  as *mut int8_t;
    let arg1 = self.1  as c_int;
    unsafe {_ZNK7QString10toLongLongEPbi(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QString {
  pub fn isNull<T: QString_isNull>(&mut self, value: T) -> i32 {
    value.isNull(self);
    return 1;
  }
}

pub trait QString_isNull {
  fn isNull(self, this: &mut QString) -> i32;
}

// proto: bool QString::isNull();
impl<'a> /*trait*/ QString_isNull for () {
  fn isNull(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString6isNullEv()};
    unsafe {_ZNK7QString6isNullEv()};
    return 1;
  }
}

impl /*struct*/ QString {
  pub fn append<T: QString_append>(&mut self, value: T) -> i32 {
    value.append(self);
    return 1;
  }
}

pub trait QString_append {
  fn append(self, this: &mut QString) -> i32;
}

// proto: QString & QString::append(const QChar * uc, int len);
impl<'a> /*trait*/ QString_append for (&'a  QChar, i32) {
  fn append(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString6appendEPK5QChari()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_int;
    unsafe {_ZN7QString6appendEPK5QChari(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QString {
  pub fn prepend<T: QString_prepend>(&mut self, value: T) -> i32 {
    value.prepend(self);
    return 1;
  }
}

pub trait QString_prepend {
  fn prepend(self, this: &mut QString) -> i32;
}

// proto: QString & QString::prepend(QChar c);
impl<'a> /*trait*/ QString_prepend for (QChar) {
  fn prepend(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString7prependE5QChar()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN7QString7prependE5QChar(arg0)};
    return 1;
  }
}

impl /*struct*/ QString {
  pub fn insert<T: QString_insert>(&mut self, value: T) -> i32 {
    value.insert(self);
    return 1;
  }
}

pub trait QString_insert {
  fn insert(self, this: &mut QString) -> i32;
}

// proto: QString & QString::insert(int i, QChar c);
impl<'a> /*trait*/ QString_insert for (i32, QChar) {
  fn insert(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString6insertEi5QChar()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN7QString6insertEi5QChar(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QString {
  pub fn left<T: QString_left>(&mut self, value: T) -> i32 {
    value.left(self);
    return 1;
  }
}

pub trait QString_left {
  fn left(self, this: &mut QString) -> i32;
}

// proto: QString QString::left(int n);
impl<'a> /*trait*/ QString_left for (i32) {
  fn left(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString4leftEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK7QString4leftEi(arg0)};
    return 1;
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
  pub fn isEmpty<T: QString_isEmpty>(&mut self, value: T) -> i32 {
    value.isEmpty(self);
    return 1;
  }
}

pub trait QString_isEmpty {
  fn isEmpty(self, this: &mut QString) -> i32;
}

// proto: bool QString::isEmpty();
impl<'a> /*trait*/ QString_isEmpty for () {
  fn isEmpty(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString7isEmptyEv()};
    unsafe {_ZNK7QString7isEmptyEv()};
    return 1;
  }
}

// proto: QString & QString::prepend(const char * s);
impl<'a> /*trait*/ QString_prepend for (&'a  String) {
  fn prepend(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString7prependEPKc()};
    let arg0 = self.as_ptr()  as *const c_char;
    unsafe {_ZN7QString7prependEPKc(arg0)};
    return 1;
  }
}

impl /*struct*/ QString {
  pub fn lastIndexOf<T: QString_lastIndexOf>(&mut self, value: T) -> i32 {
    value.lastIndexOf(self);
    return 1;
  }
}

pub trait QString_lastIndexOf {
  fn lastIndexOf(self, this: &mut QString) -> i32;
}

// proto: int QString::lastIndexOf(const QRegularExpression & re, int from);
impl<'a> /*trait*/ QString_lastIndexOf for (&'a  QRegularExpression, i32) {
  fn lastIndexOf(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString11lastIndexOfERK18QRegularExpressioni()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_int;
    unsafe {_ZNK7QString11lastIndexOfERK18QRegularExpressioni(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QString {
  pub fn number<T: QString_number>(&mut self, value: T) -> i32 {
    value.number(self);
    return 1;
  }
}

pub trait QString_number {
  fn number(self, this: &mut QString) -> i32;
}

// proto: QString QString::number(int , int base);
impl<'a> /*trait*/ QString_number for (i32, i32) {
  fn number(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString6numberEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZN7QString6numberEii(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QString {
  pub fn resize<T: QString_resize>(&mut self, value: T) -> i32 {
    value.resize(self);
    return 1;
  }
}

pub trait QString_resize {
  fn resize(self, this: &mut QString) -> i32;
}

// proto: void QString::resize(int size);
impl<'a> /*trait*/ QString_resize for (i32) {
  fn resize(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString6resizeEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN7QString6resizeEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QString {
  pub fn push_front<T: QString_push_front>(&mut self, value: T) -> i32 {
    value.push_front(self);
    return 1;
  }
}

pub trait QString_push_front {
  fn push_front(self, this: &mut QString) -> i32;
}

// proto: void QString::push_front(QChar c);
impl<'a> /*trait*/ QString_push_front for (QChar) {
  fn push_front(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString10push_frontE5QChar()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN7QString10push_frontE5QChar(arg0)};
    return 1;
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
  pub fn toDouble<T: QString_toDouble>(&mut self, value: T) -> i32 {
    value.toDouble(self);
    return 1;
  }
}

pub trait QString_toDouble {
  fn toDouble(self, this: &mut QString) -> i32;
}

// proto: double QString::toDouble(bool * ok);
impl<'a> /*trait*/ QString_toDouble for (&'a mut i8) {
  fn toDouble(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString8toDoubleEPb()};
    let arg0 = self  as *mut int8_t;
    unsafe {_ZNK7QString8toDoubleEPb(arg0)};
    return 1;
  }
}

impl /*struct*/ QString {
  pub fn arg<T: QString_arg>(&mut self, value: T) -> i32 {
    value.arg(self);
    return 1;
  }
}

pub trait QString_arg {
  fn arg(self, this: &mut QString) -> i32;
}

// proto: QString QString::arg(const QString & a1, const QString & a2, const QString & a3, const QString & a4, const QString & a5, const QString & a6, const QString & a7, const QString & a8);
impl<'a> /*trait*/ QString_arg for (&'a  QString, &'a  QString, &'a  QString, &'a  QString, &'a  QString, &'a  QString, &'a  QString, &'a  QString) {
  fn arg(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString3argERKS_S1_S1_S1_S1_S1_S1_S1_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2.qclsinst  as *const c_void;
    let arg3 = self.3.qclsinst  as *const c_void;
    let arg4 = self.4.qclsinst  as *const c_void;
    let arg5 = self.5.qclsinst  as *const c_void;
    let arg6 = self.6.qclsinst  as *const c_void;
    let arg7 = self.7.qclsinst  as *const c_void;
    unsafe {_ZNK7QString3argERKS_S1_S1_S1_S1_S1_S1_S1_(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7)};
    return 1;
  }
}

impl /*struct*/ QString {
  pub fn rightRef<T: QString_rightRef>(&mut self, value: T) -> i32 {
    value.rightRef(self);
    return 1;
  }
}

pub trait QString_rightRef {
  fn rightRef(self, this: &mut QString) -> i32;
}

// proto: QStringRef QString::rightRef(int n);
impl<'a> /*trait*/ QString_rightRef for (i32) {
  fn rightRef(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString8rightRefEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK7QString8rightRefEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QString {
  pub fn setNum<T: QString_setNum>(&mut self, value: T) -> i32 {
    value.setNum(self);
    return 1;
  }
}

pub trait QString_setNum {
  fn setNum(self, this: &mut QString) -> i32;
}

// proto: QString & QString::setNum(short , int base);
impl<'a> /*trait*/ QString_setNum for (i16, i32) {
  fn setNum(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString6setNumEsi()};
    let arg0 = self.0  as c_short;
    let arg1 = self.1  as c_int;
    unsafe {_ZN7QString6setNumEsi(arg0, arg1)};
    return 1;
  }
}

// proto: void QString::NewQString(const QChar * unicode, int size);
impl<'a> /*trait*/ QString_NewQString for (&'a  QChar, i32) {
  fn NewQString(self) -> QString {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QStringC1EPK5QChari()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_int;
    unsafe {_ZN7QStringC1EPK5QChari(qthis, arg0, arg1)};
    let rsthis = QString{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QString {
  pub fn section<T: QString_section>(&mut self, value: T) -> i32 {
    value.section(self);
    return 1;
  }
}

pub trait QString_section {
  fn section(self, this: &mut QString) -> i32;
}

// proto: QString QString::section(QChar sep, int start, int end, SectionFlags flags);
impl<'a> /*trait*/ QString_section for (QChar, i32, i32, i32) {
  fn section(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString7sectionE5QCharii6QFlagsINS_11SectionFlagEE()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    unsafe {_ZNK7QString7sectionE5QCharii6QFlagsINS_11SectionFlagEE(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QString {
  pub fn toFloat<T: QString_toFloat>(&mut self, value: T) -> i32 {
    value.toFloat(self);
    return 1;
  }
}

pub trait QString_toFloat {
  fn toFloat(self, this: &mut QString) -> i32;
}

// proto: float QString::toFloat(bool * ok);
impl<'a> /*trait*/ QString_toFloat for (&'a mut i8) {
  fn toFloat(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString7toFloatEPb()};
    let arg0 = self  as *mut int8_t;
    unsafe {_ZNK7QString7toFloatEPb(arg0)};
    return 1;
  }
}

impl /*struct*/ QString {
  pub fn count<T: QString_count>(&mut self, value: T) -> i32 {
    value.count(self);
    return 1;
  }
}

pub trait QString_count {
  fn count(self, this: &mut QString) -> i32;
}

// proto: int QString::count(const QRegularExpression & re);
impl<'a> /*trait*/ QString_count for (&'a  QRegularExpression) {
  fn count(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString5countERK18QRegularExpression()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK7QString5countERK18QRegularExpression(arg0)};
    return 1;
  }
}

impl /*struct*/ QString {
  pub fn midRef<T: QString_midRef>(&mut self, value: T) -> i32 {
    value.midRef(self);
    return 1;
  }
}

pub trait QString_midRef {
  fn midRef(self, this: &mut QString) -> i32;
}

// proto: QStringRef QString::midRef(int position, int n);
impl<'a> /*trait*/ QString_midRef for (i32, i32) {
  fn midRef(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString6midRefEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZNK7QString6midRefEii(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QString {
  pub fn detach<T: QString_detach>(&mut self, value: T) -> i32 {
    value.detach(self);
    return 1;
  }
}

pub trait QString_detach {
  fn detach(self, this: &mut QString) -> i32;
}

// proto: void QString::detach();
impl<'a> /*trait*/ QString_detach for () {
  fn detach(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString6detachEv()};
    unsafe {_ZN7QString6detachEv()};
    return 1;
  }
}

// proto: QString QString::arg(const QString & a1, const QString & a2, const QString & a3, const QString & a4);
impl<'a> /*trait*/ QString_arg for (&'a  QString, &'a  QString, &'a  QString, &'a  QString) {
  fn arg(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString3argERKS_S1_S1_S1_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2.qclsinst  as *const c_void;
    let arg3 = self.3.qclsinst  as *const c_void;
    unsafe {_ZNK7QString3argERKS_S1_S1_S1_(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

// proto: QString QString::arg(long a, int fieldwidth, int base, QChar fillChar);
impl<'a> /*trait*/ QString_arg for (i32, i32, i32, QChar) {
  fn arg(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString3argElii5QChar()};
    let arg0 = self.0  as c_long;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3.qclsinst  as *mut c_void;
    unsafe {_ZNK7QString3argElii5QChar(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

// proto: int QString::count();
impl<'a> /*trait*/ QString_count for () {
  fn count(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString5countEv()};
    unsafe {_ZNK7QString5countEv()};
    return 1;
  }
}

// proto: QString & QString::setNum(qulonglong , int base);
impl<'a> /*trait*/ QString_setNum for (u64, i32) {
  fn setNum(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString6setNumEyi()};
    let arg0 = self.0  as uint64_t;
    let arg1 = self.1  as c_int;
    unsafe {_ZN7QString6setNumEyi(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QString {
  pub fn fromStdWString<T: QString_fromStdWString>(&mut self, value: T) -> i32 {
    value.fromStdWString(self);
    return 1;
  }
}

pub trait QString_fromStdWString {
  fn fromStdWString(self, this: &mut QString) -> i32;
}

// proto: QString QString::fromStdWString(const std::wstring & s);
impl<'a> /*trait*/ QString_fromStdWString for (&'a  i32) {
  fn fromStdWString(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString14fromStdWStringERKi()};
    let arg0 = self  as *const c_int;
    unsafe {_ZN7QString14fromStdWStringERKi(arg0)};
    return 1;
  }
}

impl /*struct*/ QString {
  pub fn push_back<T: QString_push_back>(&mut self, value: T) -> i32 {
    value.push_back(self);
    return 1;
  }
}

pub trait QString_push_back {
  fn push_back(self, this: &mut QString) -> i32;
}

// proto: void QString::push_back(QChar c);
impl<'a> /*trait*/ QString_push_back for (QChar) {
  fn push_back(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString9push_backE5QChar()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN7QString9push_backE5QChar(arg0)};
    return 1;
  }
}

// proto: QString & QString::setNum(float , char f, int prec);
impl<'a> /*trait*/ QString_setNum for (f32, i8, i32) {
  fn setNum(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString6setNumEfci()};
    let arg0 = self.0  as c_float;
    let arg1 = self.1  as c_char;
    let arg2 = self.2  as c_int;
    unsafe {_ZN7QString6setNumEfci(arg0, arg1, arg2)};
    return 1;
  }
}

// proto: int QString::count(const QRegExp & );
impl<'a> /*trait*/ QString_count for (&'a  QRegExp) {
  fn count(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString5countERK7QRegExp()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK7QString5countERK7QRegExp(arg0)};
    return 1;
  }
}

impl /*struct*/ QString {
  pub fn size<T: QString_size>(&mut self, value: T) -> i32 {
    value.size(self);
    return 1;
  }
}

pub trait QString_size {
  fn size(self, this: &mut QString) -> i32;
}

// proto: int QString::size();
impl<'a> /*trait*/ QString_size for () {
  fn size(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString4sizeEv()};
    unsafe {_ZNK7QString4sizeEv()};
    return 1;
  }
}

// proto: QString & QString::insert(int i, const QChar * uc, int len);
impl<'a> /*trait*/ QString_insert for (i32, &'a  QChar, i32) {
  fn insert(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString6insertEiPK5QChari()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2  as c_int;
    unsafe {_ZN7QString6insertEiPK5QChari(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QString {
  pub fn replace<T: QString_replace>(&mut self, value: T) -> i32 {
    value.replace(self);
    return 1;
  }
}

pub trait QString_replace {
  fn replace(self, this: &mut QString) -> i32;
}

// proto: QString & QString::replace(int i, int len, const QChar * s, int slen);
impl<'a> /*trait*/ QString_replace for (i32, i32, &'a  QChar, i32) {
  fn replace(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString7replaceEiiPK5QChari()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *const c_void;
    let arg3 = self.3  as c_int;
    unsafe {_ZN7QString7replaceEiiPK5QChari(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QString {
  pub fn fromRawData<T: QString_fromRawData>(&mut self, value: T) -> i32 {
    value.fromRawData(self);
    return 1;
  }
}

pub trait QString_fromRawData {
  fn fromRawData(self, this: &mut QString) -> i32;
}

// proto: QString QString::fromRawData(const QChar * , int size);
impl<'a> /*trait*/ QString_fromRawData for (&'a  QChar, i32) {
  fn fromRawData(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString11fromRawDataEPK5QChari()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_int;
    unsafe {_ZN7QString11fromRawDataEPK5QChari(arg0, arg1)};
    return 1;
  }
}

// proto: QString & QString::insert(int i, const QString & s);
impl<'a> /*trait*/ QString_insert for (i32, &'a  QString) {
  fn insert(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString6insertEiRKS_()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN7QString6insertEiRKS_(arg0, arg1)};
    return 1;
  }
}

// proto: QString QString::arg(const QString & a1, const QString & a2, const QString & a3, const QString & a4, const QString & a5);
impl<'a> /*trait*/ QString_arg for (&'a  QString, &'a  QString, &'a  QString, &'a  QString, &'a  QString) {
  fn arg(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString3argERKS_S1_S1_S1_S1_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2.qclsinst  as *const c_void;
    let arg3 = self.3.qclsinst  as *const c_void;
    let arg4 = self.4.qclsinst  as *const c_void;
    unsafe {_ZNK7QString3argERKS_S1_S1_S1_S1_(arg0, arg1, arg2, arg3, arg4)};
    return 1;
  }
}

impl /*struct*/ QString {
  pub fn setRawData<T: QString_setRawData>(&mut self, value: T) -> i32 {
    value.setRawData(self);
    return 1;
  }
}

pub trait QString_setRawData {
  fn setRawData(self, this: &mut QString) -> i32;
}

// proto: QString & QString::setRawData(const QChar * unicode, int size);
impl<'a> /*trait*/ QString_setRawData for (&'a  QChar, i32) {
  fn setRawData(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString10setRawDataEPK5QChari()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_int;
    unsafe {_ZN7QString10setRawDataEPK5QChari(arg0, arg1)};
    return 1;
  }
}

// proto: QString & QString::prepend(const QString & s);
impl<'a> /*trait*/ QString_prepend for (&'a  QString) {
  fn prepend(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString7prependERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QString7prependERKS_(arg0)};
    return 1;
  }
}

// proto: QString QString::section(const QString & in_sep, int start, int end, SectionFlags flags);
impl<'a> /*trait*/ QString_section for (&'a  QString, i32, i32, i32) {
  fn section(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString7sectionERKS_ii6QFlagsINS_11SectionFlagEE()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    unsafe {_ZNK7QString7sectionERKS_ii6QFlagsINS_11SectionFlagEE(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

// proto: QString & QString::setNum(int , int base);
impl<'a> /*trait*/ QString_setNum for (i32, i32) {
  fn setNum(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString6setNumEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZN7QString6setNumEii(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QString {
  pub fn toULong<T: QString_toULong>(&mut self, value: T) -> i32 {
    value.toULong(self);
    return 1;
  }
}

pub trait QString_toULong {
  fn toULong(self, this: &mut QString) -> i32;
}

// proto: unsigned long QString::toULong(bool * ok, int base);
impl<'a> /*trait*/ QString_toULong for (&'a mut i8, i32) {
  fn toULong(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString7toULongEPbi()};
    let arg0 = self.0  as *mut int8_t;
    let arg1 = self.1  as c_int;
    unsafe {_ZNK7QString7toULongEPbi(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QString {
  pub fn chop<T: QString_chop>(&mut self, value: T) -> i32 {
    value.chop(self);
    return 1;
  }
}

pub trait QString_chop {
  fn chop(self, this: &mut QString) -> i32;
}

// proto: void QString::chop(int n);
impl<'a> /*trait*/ QString_chop for (i32) {
  fn chop(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString4chopEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN7QString4chopEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QString {
  pub fn fromUtf16<T: QString_fromUtf16>(&mut self, value: T) -> i32 {
    value.fromUtf16(self);
    return 1;
  }
}

pub trait QString_fromUtf16 {
  fn fromUtf16(self, this: &mut QString) -> i32;
}

// proto: QString QString::fromUtf16(const ushort * , int size);
impl<'a> /*trait*/ QString_fromUtf16 for (&'a  u16, i32) {
  fn fromUtf16(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString9fromUtf16EPKti()};
    let arg0 = self.0  as *const c_ushort;
    let arg1 = self.1  as c_int;
    unsafe {_ZN7QString9fromUtf16EPKti(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QString {
  pub fn isDetached<T: QString_isDetached>(&mut self, value: T) -> i32 {
    value.isDetached(self);
    return 1;
  }
}

pub trait QString_isDetached {
  fn isDetached(self, this: &mut QString) -> i32;
}

// proto: bool QString::isDetached();
impl<'a> /*trait*/ QString_isDetached for () {
  fn isDetached(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString10isDetachedEv()};
    unsafe {_ZNK7QString10isDetachedEv()};
    return 1;
  }
}

// proto: QString & QString::setNum(qlonglong , int base);
impl<'a> /*trait*/ QString_setNum for (i64, i32) {
  fn setNum(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString6setNumExi()};
    let arg0 = self.0  as c_longlong;
    let arg1 = self.1  as c_int;
    unsafe {_ZN7QString6setNumExi(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QString {
  pub fn mid<T: QString_mid>(&mut self, value: T) -> i32 {
    value.mid(self);
    return 1;
  }
}

pub trait QString_mid {
  fn mid(self, this: &mut QString) -> i32;
}

// proto: QString QString::mid(int position, int n);
impl<'a> /*trait*/ QString_mid for (i32, i32) {
  fn mid(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString3midEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZNK7QString3midEii(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QString {
  pub fn fromLocal8Bit<T: QString_fromLocal8Bit>(&mut self, value: T) -> i32 {
    value.fromLocal8Bit(self);
    return 1;
  }
}

pub trait QString_fromLocal8Bit {
  fn fromLocal8Bit(self, this: &mut QString) -> i32;
}

// proto: QString QString::fromLocal8Bit(const char * str, int size);
impl<'a> /*trait*/ QString_fromLocal8Bit for (&'a  String, i32) {
  fn fromLocal8Bit(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString13fromLocal8BitEPKci()};
    let arg0 = self.0.as_ptr()  as *const c_char;
    let arg1 = self.1  as c_int;
    unsafe {_ZN7QString13fromLocal8BitEPKci(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QString {
  pub fn swap<T: QString_swap>(&mut self, value: T) -> i32 {
    value.swap(self);
    return 1;
  }
}

pub trait QString_swap {
  fn swap(self, this: &mut QString) -> i32;
}

// proto: void QString::swap(QString & other);
impl<'a> /*trait*/ QString_swap for (&'a mut QString) {
  fn swap(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN7QString4swapERS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QString {
  pub fn fromUtf8<T: QString_fromUtf8>(&mut self, value: T) -> i32 {
    value.fromUtf8(self);
    return 1;
  }
}

pub trait QString_fromUtf8 {
  fn fromUtf8(self, this: &mut QString) -> i32;
}

// proto: QString QString::fromUtf8(const QByteArray & str);
impl<'a> /*trait*/ QString_fromUtf8 for (&'a  QByteArray) {
  fn fromUtf8(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString8fromUtf8ERK10QByteArray()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QString8fromUtf8ERK10QByteArray(arg0)};
    return 1;
  }
}

impl /*struct*/ QString {
  pub fn fromUcs4<T: QString_fromUcs4>(&mut self, value: T) -> i32 {
    value.fromUcs4(self);
    return 1;
  }
}

pub trait QString_fromUcs4 {
  fn fromUcs4(self, this: &mut QString) -> i32;
}

// proto: QString QString::fromUcs4(const char32_t * str, int size);
impl<'a> /*trait*/ QString_fromUcs4 for (&'a  String, i32) {
  fn fromUcs4(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString8fromUcs4EPKDii()};
    let arg0 = self.0.as_ptr()  as *const int32_t;
    let arg1 = self.1  as c_int;
    unsafe {_ZN7QString8fromUcs4EPKDii(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QString {
  pub fn leftJustified<T: QString_leftJustified>(&mut self, value: T) -> i32 {
    value.leftJustified(self);
    return 1;
  }
}

pub trait QString_leftJustified {
  fn leftJustified(self, this: &mut QString) -> i32;
}

// proto: QString QString::leftJustified(int width, QChar fill, bool trunc);
impl<'a> /*trait*/ QString_leftJustified for (i32, QChar, i8) {
  fn leftJustified(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString13leftJustifiedEi5QCharb()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as int8_t;
    unsafe {_ZNK7QString13leftJustifiedEi5QCharb(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QString {
  pub fn indexOf<T: QString_indexOf>(&mut self, value: T) -> i32 {
    value.indexOf(self);
    return 1;
  }
}

pub trait QString_indexOf {
  fn indexOf(self, this: &mut QString) -> i32;
}

// proto: int QString::indexOf(const QRegExp & , int from);
impl<'a> /*trait*/ QString_indexOf for (&'a  QRegExp, i32) {
  fn indexOf(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString7indexOfERK7QRegExpi()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_int;
    unsafe {_ZNK7QString7indexOfERK7QRegExpi(arg0, arg1)};
    return 1;
  }
}

// proto: void QString::push_back(const QString & s);
impl<'a> /*trait*/ QString_push_back for (&'a  QString) {
  fn push_back(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString9push_backERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QString9push_backERKS_(arg0)};
    return 1;
  }
}

// proto: int QString::lastIndexOf(QRegExp & , int from);
impl<'a> /*trait*/ QString_lastIndexOf for (&'a mut QRegExp, i32) {
  fn lastIndexOf(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString11lastIndexOfER7QRegExpi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    unsafe {_ZNK7QString11lastIndexOfER7QRegExpi(arg0, arg1)};
    return 1;
  }
}

// proto: QString QString::arg(const QString & a1, const QString & a2, const QString & a3);
impl<'a> /*trait*/ QString_arg for (&'a  QString, &'a  QString, &'a  QString) {
  fn arg(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString3argERKS_S1_S1_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2.qclsinst  as *const c_void;
    unsafe {_ZNK7QString3argERKS_S1_S1_(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QString {
  pub fn utf16<T: QString_utf16>(&mut self, value: T) -> i32 {
    value.utf16(self);
    return 1;
  }
}

pub trait QString_utf16 {
  fn utf16(self, this: &mut QString) -> i32;
}

// proto: const ushort * QString::utf16();
impl<'a> /*trait*/ QString_utf16 for () {
  fn utf16(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString5utf16Ev()};
    unsafe {_ZNK7QString5utf16Ev()};
    return 1;
  }
}

impl /*struct*/ QString {
  pub fn toInt<T: QString_toInt>(&mut self, value: T) -> i32 {
    value.toInt(self);
    return 1;
  }
}

pub trait QString_toInt {
  fn toInt(self, this: &mut QString) -> i32;
}

// proto: int QString::toInt(bool * ok, int base);
impl<'a> /*trait*/ QString_toInt for (&'a mut i8, i32) {
  fn toInt(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString5toIntEPbi()};
    let arg0 = self.0  as *mut int8_t;
    let arg1 = self.1  as c_int;
    unsafe {_ZNK7QString5toIntEPbi(arg0, arg1)};
    return 1;
  }
}

// proto: QString QString::arg(double a, int fieldWidth, char fmt, int prec, QChar fillChar);
impl<'a> /*trait*/ QString_arg for (f64, i32, i8, i32, QChar) {
  fn arg(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString3argEdici5QChar()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_char;
    let arg3 = self.3  as c_int;
    let arg4 = self.4.qclsinst  as *mut c_void;
    unsafe {_ZNK7QString3argEdici5QChar(arg0, arg1, arg2, arg3, arg4)};
    return 1;
  }
}

impl /*struct*/ QString {
  pub fn data<T: QString_data>(&mut self, value: T) -> i32 {
    value.data(self);
    return 1;
  }
}

pub trait QString_data {
  fn data(self, this: &mut QString) -> i32;
}

// proto: QChar * QString::data();
impl<'a> /*trait*/ QString_data for () {
  fn data(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString4dataEv()};
    unsafe {_ZN7QString4dataEv()};
    return 1;
  }
}

// proto: QString & QString::setNum(uint , int base);
impl<'a> /*trait*/ QString_setNum for (u32, i32) {
  fn setNum(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString6setNumEji()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_int;
    unsafe {_ZN7QString6setNumEji(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QString {
  pub fn localeAwareCompare<T: QString_localeAwareCompare>(&mut self, value: T) -> i32 {
    value.localeAwareCompare(self);
    return 1;
  }
}

pub trait QString_localeAwareCompare {
  fn localeAwareCompare(self, this: &mut QString) -> i32;
}

// proto: int QString::localeAwareCompare(const QString & s1, const QString & s2);
impl<'a> /*trait*/ QString_localeAwareCompare for (&'a  QString, &'a  QString) {
  fn localeAwareCompare(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString18localeAwareCompareERKS_S1_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN7QString18localeAwareCompareERKS_S1_(arg0, arg1)};
    return 1;
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

// proto: QString QString::fromUtf16(const char16_t * str, int size);
impl<'a> /*trait*/ QString_fromUtf16 for (&'a  String, i32) {
  fn fromUtf16(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString9fromUtf16EPKDsi()};
    let arg0 = self.0.as_ptr()  as *const int16_t;
    let arg1 = self.1  as c_int;
    unsafe {_ZN7QString9fromUtf16EPKDsi(arg0, arg1)};
    return 1;
  }
}

// proto: QString & QString::replace(const QRegExp & rx, const QString & after);
impl<'a> /*trait*/ QString_replace for (&'a  QRegExp, &'a  QString) {
  fn replace(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString7replaceERK7QRegExpRKS_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN7QString7replaceERK7QRegExpRKS_(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QString {
  pub fn repeated<T: QString_repeated>(&mut self, value: T) -> i32 {
    value.repeated(self);
    return 1;
  }
}

pub trait QString_repeated {
  fn repeated(self, this: &mut QString) -> i32;
}

// proto: QString QString::repeated(int times);
impl<'a> /*trait*/ QString_repeated for (i32) {
  fn repeated(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString8repeatedEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK7QString8repeatedEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QString {
  pub fn setUtf16<T: QString_setUtf16>(&mut self, value: T) -> i32 {
    value.setUtf16(self);
    return 1;
  }
}

pub trait QString_setUtf16 {
  fn setUtf16(self, this: &mut QString) -> i32;
}

// proto: QString & QString::setUtf16(const ushort * utf16, int size);
impl<'a> /*trait*/ QString_setUtf16 for (&'a  u16, i32) {
  fn setUtf16(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString8setUtf16EPKti()};
    let arg0 = self.0  as *const c_ushort;
    let arg1 = self.1  as c_int;
    unsafe {_ZN7QString8setUtf16EPKti(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QString {
  pub fn fromStdU32String<T: QString_fromStdU32String>(&mut self, value: T) -> i32 {
    value.fromStdU32String(self);
    return 1;
  }
}

pub trait QString_fromStdU32String {
  fn fromStdU32String(self, this: &mut QString) -> i32;
}

// proto: QString QString::fromStdU32String(const std::u32string & s);
impl<'a> /*trait*/ QString_fromStdU32String for (&'a  i32) {
  fn fromStdU32String(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString16fromStdU32StringERKi()};
    let arg0 = self  as *const c_int;
    unsafe {_ZN7QString16fromStdU32StringERKi(arg0)};
    return 1;
  }
}

impl /*struct*/ QString {
  pub fn clear<T: QString_clear>(&mut self, value: T) -> i32 {
    value.clear(self);
    return 1;
  }
}

pub trait QString_clear {
  fn clear(self, this: &mut QString) -> i32;
}

// proto: void QString::clear();
impl<'a> /*trait*/ QString_clear for () {
  fn clear(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString5clearEv()};
    unsafe {_ZN7QString5clearEv()};
    return 1;
  }
}

impl /*struct*/ QString {
  pub fn contains<T: QString_contains>(&mut self, value: T) -> i32 {
    value.contains(self);
    return 1;
  }
}

pub trait QString_contains {
  fn contains(self, this: &mut QString) -> i32;
}

// proto: bool QString::contains(const QRegExp & rx);
impl<'a> /*trait*/ QString_contains for (&'a  QRegExp) {
  fn contains(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString8containsERK7QRegExp()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK7QString8containsERK7QRegExp(arg0)};
    return 1;
  }
}

impl /*struct*/ QString {
  pub fn isSharedWith<T: QString_isSharedWith>(&mut self, value: T) -> i32 {
    value.isSharedWith(self);
    return 1;
  }
}

pub trait QString_isSharedWith {
  fn isSharedWith(self, this: &mut QString) -> i32;
}

// proto: bool QString::isSharedWith(const QString & other);
impl<'a> /*trait*/ QString_isSharedWith for (&'a  QString) {
  fn isSharedWith(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString12isSharedWithERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK7QString12isSharedWithERKS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QString {
  pub fn fromLatin1<T: QString_fromLatin1>(&mut self, value: T) -> i32 {
    value.fromLatin1(self);
    return 1;
  }
}

pub trait QString_fromLatin1 {
  fn fromLatin1(self, this: &mut QString) -> i32;
}

// proto: QString QString::fromLatin1(const QByteArray & str);
impl<'a> /*trait*/ QString_fromLatin1 for (&'a  QByteArray) {
  fn fromLatin1(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString10fromLatin1ERK10QByteArray()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QString10fromLatin1ERK10QByteArray(arg0)};
    return 1;
  }
}

impl /*struct*/ QString {
  pub fn FreeQString<T: QString_FreeQString>(&mut self, value: T) -> i32 {
    value.FreeQString(self);
    return 1;
  }
}

pub trait QString_FreeQString {
  fn FreeQString(self, this: &mut QString) -> i32;
}

// proto: void QString::FreeQString();
impl<'a> /*trait*/ QString_FreeQString for () {
  fn FreeQString(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QStringD0Ev()};
    unsafe {_ZN7QStringD0Ev()};
    return 1;
  }
}

impl /*struct*/ QString {
  pub fn remove<T: QString_remove>(&mut self, value: T) -> i32 {
    value.remove(self);
    return 1;
  }
}

pub trait QString_remove {
  fn remove(self, this: &mut QString) -> i32;
}

// proto: QString & QString::remove(const QRegularExpression & re);
impl<'a> /*trait*/ QString_remove for (&'a  QRegularExpression) {
  fn remove(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString6removeERK18QRegularExpression()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QString6removeERK18QRegularExpression(arg0)};
    return 1;
  }
}

impl /*struct*/ QString {
  pub fn cend<T: QString_cend>(&mut self, value: T) -> i32 {
    value.cend(self);
    return 1;
  }
}

pub trait QString_cend {
  fn cend(self, this: &mut QString) -> i32;
}

// proto: const QChar * QString::cend();
impl<'a> /*trait*/ QString_cend for () {
  fn cend(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString4cendEv()};
    unsafe {_ZNK7QString4cendEv()};
    return 1;
  }
}

impl /*struct*/ QString {
  pub fn toHtmlEscaped<T: QString_toHtmlEscaped>(&mut self, value: T) -> i32 {
    value.toHtmlEscaped(self);
    return 1;
  }
}

pub trait QString_toHtmlEscaped {
  fn toHtmlEscaped(self, this: &mut QString) -> i32;
}

// proto: QString QString::toHtmlEscaped();
impl<'a> /*trait*/ QString_toHtmlEscaped for () {
  fn toHtmlEscaped(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString13toHtmlEscapedEv()};
    unsafe {_ZNK7QString13toHtmlEscapedEv()};
    return 1;
  }
}

// proto: QString & QString::append(const QByteArray & s);
impl<'a> /*trait*/ QString_append for (&'a  QByteArray) {
  fn append(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString6appendERK10QByteArray()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QString6appendERK10QByteArray(arg0)};
    return 1;
  }
}

// proto: QString QString::fromLatin1(const char * str, int size);
impl<'a> /*trait*/ QString_fromLatin1 for (&'a  String, i32) {
  fn fromLatin1(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString10fromLatin1EPKci()};
    let arg0 = self.0.as_ptr()  as *const c_char;
    let arg1 = self.1  as c_int;
    unsafe {_ZN7QString10fromLatin1EPKci(arg0, arg1)};
    return 1;
  }
}

// proto: bool QString::contains(const QRegularExpression & re, QRegularExpressionMatch * match);
impl<'a> /*trait*/ QString_contains for (&'a  QRegularExpression, &'a mut QRegularExpressionMatch) {
  fn contains(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString8containsERK18QRegularExpressionP23QRegularExpressionMatch()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZNK7QString8containsERK18QRegularExpressionP23QRegularExpressionMatch(arg0, arg1)};
    return 1;
  }
}

// proto: int QString::indexOf(const QRegularExpression & re, int from, QRegularExpressionMatch * rmatch);
impl<'a> /*trait*/ QString_indexOf for (&'a  QRegularExpression, i32, &'a mut QRegularExpressionMatch) {
  fn indexOf(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString7indexOfERK18QRegularExpressioniP23QRegularExpressionMatch()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    unsafe {_ZNK7QString7indexOfERK18QRegularExpressioniP23QRegularExpressionMatch(arg0, arg1, arg2)};
    return 1;
  }
}

// proto: int QString::lastIndexOf(const QRegExp & , int from);
impl<'a> /*trait*/ QString_lastIndexOf for (&'a  QRegExp, i32) {
  fn lastIndexOf(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString11lastIndexOfERK7QRegExpi()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_int;
    unsafe {_ZNK7QString11lastIndexOfERK7QRegExpi(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QString {
  pub fn toWCharArray<T: QString_toWCharArray>(&mut self, value: T) -> i32 {
    value.toWCharArray(self);
    return 1;
  }
}

pub trait QString_toWCharArray {
  fn toWCharArray(self, this: &mut QString) -> i32;
}

// proto: int QString::toWCharArray(wchar_t * array);
impl<'a> /*trait*/ QString_toWCharArray for (&'a mut String) {
  fn toWCharArray(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString12toWCharArrayEPw()};
    let arg0 = self.as_ptr()  as *mut wchar_t;
    unsafe {_ZNK7QString12toWCharArrayEPw(arg0)};
    return 1;
  }
}

impl /*struct*/ QString {
  pub fn cbegin<T: QString_cbegin>(&mut self, value: T) -> i32 {
    value.cbegin(self);
    return 1;
  }
}

pub trait QString_cbegin {
  fn cbegin(self, this: &mut QString) -> i32;
}

// proto: const QChar * QString::cbegin();
impl<'a> /*trait*/ QString_cbegin for () {
  fn cbegin(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString6cbeginEv()};
    unsafe {_ZNK7QString6cbeginEv()};
    return 1;
  }
}

// proto: QString & QString::prepend(const QByteArray & s);
impl<'a> /*trait*/ QString_prepend for (&'a  QByteArray) {
  fn prepend(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString7prependERK10QByteArray()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QString7prependERK10QByteArray(arg0)};
    return 1;
  }
}

// proto: QString & QString::replace(int i, int len, const QString & after);
impl<'a> /*trait*/ QString_replace for (i32, i32, &'a  QString) {
  fn replace(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString7replaceEiiRKS_()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *const c_void;
    unsafe {_ZN7QString7replaceEiiRKS_(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QString {
  pub fn fromStdString<T: QString_fromStdString>(&mut self, value: T) -> i32 {
    value.fromStdString(self);
    return 1;
  }
}

pub trait QString_fromStdString {
  fn fromStdString(self, this: &mut QString) -> i32;
}

// proto: QString QString::fromStdString(const std::string & s);
impl<'a> /*trait*/ QString_fromStdString for (&'a  i32) {
  fn fromStdString(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString13fromStdStringERKi()};
    let arg0 = self  as *const c_int;
    unsafe {_ZN7QString13fromStdStringERKi(arg0)};
    return 1;
  }
}

// proto: QString QString::arg(const QString & a1, const QString & a2, const QString & a3, const QString & a4, const QString & a5, const QString & a6, const QString & a7);
impl<'a> /*trait*/ QString_arg for (&'a  QString, &'a  QString, &'a  QString, &'a  QString, &'a  QString, &'a  QString, &'a  QString) {
  fn arg(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString3argERKS_S1_S1_S1_S1_S1_S1_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2.qclsinst  as *const c_void;
    let arg3 = self.3.qclsinst  as *const c_void;
    let arg4 = self.4.qclsinst  as *const c_void;
    let arg5 = self.5.qclsinst  as *const c_void;
    let arg6 = self.6.qclsinst  as *const c_void;
    unsafe {_ZNK7QString3argERKS_S1_S1_S1_S1_S1_S1_(arg0, arg1, arg2, arg3, arg4, arg5, arg6)};
    return 1;
  }
}

impl /*struct*/ QString {
  pub fn fromWCharArray<T: QString_fromWCharArray>(&mut self, value: T) -> i32 {
    value.fromWCharArray(self);
    return 1;
  }
}

pub trait QString_fromWCharArray {
  fn fromWCharArray(self, this: &mut QString) -> i32;
}

// proto: QString QString::fromWCharArray(const wchar_t * string, int size);
impl<'a> /*trait*/ QString_fromWCharArray for (&'a  String, i32) {
  fn fromWCharArray(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString14fromWCharArrayEPKwi()};
    let arg0 = self.0.as_ptr()  as *const wchar_t;
    let arg1 = self.1  as c_int;
    unsafe {_ZN7QString14fromWCharArrayEPKwi(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QString {
  pub fn fill<T: QString_fill>(&mut self, value: T) -> i32 {
    value.fill(self);
    return 1;
  }
}

pub trait QString_fill {
  fn fill(self, this: &mut QString) -> i32;
}

// proto: QString & QString::fill(QChar c, int size);
impl<'a> /*trait*/ QString_fill for (QChar, i32) {
  fn fill(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString4fillE5QChari()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    unsafe {_ZN7QString4fillE5QChari(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QString {
  pub fn constData<T: QString_constData>(&mut self, value: T) -> i32 {
    value.constData(self);
    return 1;
  }
}

pub trait QString_constData {
  fn constData(self, this: &mut QString) -> i32;
}

// proto: const QChar * QString::constData();
impl<'a> /*trait*/ QString_constData for () {
  fn constData(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString9constDataEv()};
    unsafe {_ZNK7QString9constDataEv()};
    return 1;
  }
}

// proto: QString QString::number(ulong , int base);
impl<'a> /*trait*/ QString_number for (u32, i32) {
  fn number(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString6numberEmi()};
    let arg0 = self.0  as c_ulong;
    let arg1 = self.1  as c_int;
    unsafe {_ZN7QString6numberEmi(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QString {
  pub fn toLong<T: QString_toLong>(&mut self, value: T) -> i32 {
    value.toLong(self);
    return 1;
  }
}

pub trait QString_toLong {
  fn toLong(self, this: &mut QString) -> i32;
}

// proto: long QString::toLong(bool * ok, int base);
impl<'a> /*trait*/ QString_toLong for (&'a mut i8, i32) {
  fn toLong(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString6toLongEPbi()};
    let arg0 = self.0  as *mut int8_t;
    let arg1 = self.1  as c_int;
    unsafe {_ZNK7QString6toLongEPbi(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QString {
  pub fn constEnd<T: QString_constEnd>(&mut self, value: T) -> i32 {
    value.constEnd(self);
    return 1;
  }
}

pub trait QString_constEnd {
  fn constEnd(self, this: &mut QString) -> i32;
}

// proto: const QChar * QString::constEnd();
impl<'a> /*trait*/ QString_constEnd for () {
  fn constEnd(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString8constEndEv()};
    unsafe {_ZNK7QString8constEndEv()};
    return 1;
  }
}

impl /*struct*/ QString {
  pub fn length<T: QString_length>(&mut self, value: T) -> i32 {
    value.length(self);
    return 1;
  }
}

pub trait QString_length {
  fn length(self, this: &mut QString) -> i32;
}

// proto: int QString::length();
impl<'a> /*trait*/ QString_length for () {
  fn length(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString6lengthEv()};
    unsafe {_ZNK7QString6lengthEv()};
    return 1;
  }
}

// proto: QString QString::fromUtf8(const char * str, int size);
impl<'a> /*trait*/ QString_fromUtf8 for (&'a  String, i32) {
  fn fromUtf8(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString8fromUtf8EPKci()};
    let arg0 = self.0.as_ptr()  as *const c_char;
    let arg1 = self.1  as c_int;
    unsafe {_ZN7QString8fromUtf8EPKci(arg0, arg1)};
    return 1;
  }
}

// proto: QString QString::section(const QRegularExpression & re, int start, int end, SectionFlags flags);
impl<'a> /*trait*/ QString_section for (&'a  QRegularExpression, i32, i32, i32) {
  fn section(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString7sectionERK18QRegularExpressionii6QFlagsINS_11SectionFlagEE()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    unsafe {_ZNK7QString7sectionERK18QRegularExpressionii6QFlagsINS_11SectionFlagEE(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

// proto: QString QString::number(qlonglong , int base);
impl<'a> /*trait*/ QString_number for (i64, i32) {
  fn number(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString6numberExi()};
    let arg0 = self.0  as c_longlong;
    let arg1 = self.1  as c_int;
    unsafe {_ZN7QString6numberExi(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QString {
  pub fn leftRef<T: QString_leftRef>(&mut self, value: T) -> i32 {
    value.leftRef(self);
    return 1;
  }
}

pub trait QString_leftRef {
  fn leftRef(self, this: &mut QString) -> i32;
}

// proto: QStringRef QString::leftRef(int n);
impl<'a> /*trait*/ QString_leftRef for (i32) {
  fn leftRef(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString7leftRefEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK7QString7leftRefEi(arg0)};
    return 1;
  }
}

// proto: QString QString::arg(const QString & a1, const QString & a2);
impl<'a> /*trait*/ QString_arg for (&'a  QString, &'a  QString) {
  fn arg(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString3argERKS_S1_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZNK7QString3argERKS_S1_(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QString {
  pub fn isSimpleText<T: QString_isSimpleText>(&mut self, value: T) -> i32 {
    value.isSimpleText(self);
    return 1;
  }
}

pub trait QString_isSimpleText {
  fn isSimpleText(self, this: &mut QString) -> i32;
}

// proto: bool QString::isSimpleText();
impl<'a> /*trait*/ QString_isSimpleText for () {
  fn isSimpleText(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString12isSimpleTextEv()};
    unsafe {_ZNK7QString12isSimpleTextEv()};
    return 1;
  }
}

// proto: QString QString::fromUcs4(const uint * , int size);
impl<'a> /*trait*/ QString_fromUcs4 for (&'a  u32, i32) {
  fn fromUcs4(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString8fromUcs4EPKji()};
    let arg0 = self.0  as *const c_uint;
    let arg1 = self.1  as c_int;
    unsafe {_ZN7QString8fromUcs4EPKji(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QString {
  pub fn setUnicode<T: QString_setUnicode>(&mut self, value: T) -> i32 {
    value.setUnicode(self);
    return 1;
  }
}

pub trait QString_setUnicode {
  fn setUnicode(self, this: &mut QString) -> i32;
}

// proto: QString & QString::setUnicode(const QChar * unicode, int size);
impl<'a> /*trait*/ QString_setUnicode for (&'a  QChar, i32) {
  fn setUnicode(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString10setUnicodeEPK5QChari()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_int;
    unsafe {_ZN7QString10setUnicodeEPK5QChari(arg0, arg1)};
    return 1;
  }
}

// proto: bool QString::contains(QRegExp & rx);
impl<'a> /*trait*/ QString_contains for (&'a mut QRegExp) {
  fn contains(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString8containsER7QRegExp()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZNK7QString8containsER7QRegExp(arg0)};
    return 1;
  }
}

impl /*struct*/ QString {
  pub fn constBegin<T: QString_constBegin>(&mut self, value: T) -> i32 {
    value.constBegin(self);
    return 1;
  }
}

pub trait QString_constBegin {
  fn constBegin(self, this: &mut QString) -> i32;
}

// proto: const QChar * QString::constBegin();
impl<'a> /*trait*/ QString_constBegin for () {
  fn constBegin(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString10constBeginEv()};
    unsafe {_ZNK7QString10constBeginEv()};
    return 1;
  }
}

impl /*struct*/ QString {
  pub fn unicode<T: QString_unicode>(&mut self, value: T) -> i32 {
    value.unicode(self);
    return 1;
  }
}

pub trait QString_unicode {
  fn unicode(self, this: &mut QString) -> i32;
}

// proto: const QChar * QString::unicode();
impl<'a> /*trait*/ QString_unicode for () {
  fn unicode(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString7unicodeEv()};
    unsafe {_ZNK7QString7unicodeEv()};
    return 1;
  }
}

// proto: QString QString::arg(const QString & a1, const QString & a2, const QString & a3, const QString & a4, const QString & a5, const QString & a6, const QString & a7, const QString & a8, const QString & a9);
impl<'a> /*trait*/ QString_arg for (&'a  QString, &'a  QString, &'a  QString, &'a  QString, &'a  QString, &'a  QString, &'a  QString, &'a  QString, &'a  QString) {
  fn arg(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString3argERKS_S1_S1_S1_S1_S1_S1_S1_S1_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2.qclsinst  as *const c_void;
    let arg3 = self.3.qclsinst  as *const c_void;
    let arg4 = self.4.qclsinst  as *const c_void;
    let arg5 = self.5.qclsinst  as *const c_void;
    let arg6 = self.6.qclsinst  as *const c_void;
    let arg7 = self.7.qclsinst  as *const c_void;
    let arg8 = self.8.qclsinst  as *const c_void;
    unsafe {_ZNK7QString3argERKS_S1_S1_S1_S1_S1_S1_S1_S1_(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8)};
    return 1;
  }
}

// proto: int QString::indexOf(const QRegularExpression & re, int from);
impl<'a> /*trait*/ QString_indexOf for (&'a  QRegularExpression, i32) {
  fn indexOf(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString7indexOfERK18QRegularExpressioni()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_int;
    unsafe {_ZNK7QString7indexOfERK18QRegularExpressioni(arg0, arg1)};
    return 1;
  }
}

// proto: QString QString::fromLocal8Bit(const QByteArray & str);
impl<'a> /*trait*/ QString_fromLocal8Bit for (&'a  QByteArray) {
  fn fromLocal8Bit(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString13fromLocal8BitERK10QByteArray()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QString13fromLocal8BitERK10QByteArray(arg0)};
    return 1;
  }
}

impl /*struct*/ QString {
  pub fn at<T: QString_at>(&mut self, value: T) -> i32 {
    value.at(self);
    return 1;
  }
}

pub trait QString_at {
  fn at(self, this: &mut QString) -> i32;
}

// proto: const QChar QString::at(int i);
impl<'a> /*trait*/ QString_at for (i32) {
  fn at(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString2atEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK7QString2atEi(arg0)};
    return 1;
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

// proto: void QString::push_front(const QString & s);
impl<'a> /*trait*/ QString_push_front for (&'a  QString) {
  fn push_front(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString10push_frontERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QString10push_frontERKS_(arg0)};
    return 1;
  }
}

// proto: QString QString::section(const QRegExp & reg, int start, int end, SectionFlags flags);
impl<'a> /*trait*/ QString_section for (&'a  QRegExp, i32, i32, i32) {
  fn section(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString7sectionERK7QRegExpii6QFlagsINS_11SectionFlagEE()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    unsafe {_ZNK7QString7sectionERK7QRegExpii6QFlagsINS_11SectionFlagEE(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

// proto: QString QString::arg(const QString & a1, const QString & a2, const QString & a3, const QString & a4, const QString & a5, const QString & a6);
impl<'a> /*trait*/ QString_arg for (&'a  QString, &'a  QString, &'a  QString, &'a  QString, &'a  QString, &'a  QString) {
  fn arg(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString3argERKS_S1_S1_S1_S1_S1_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2.qclsinst  as *const c_void;
    let arg3 = self.3.qclsinst  as *const c_void;
    let arg4 = self.4.qclsinst  as *const c_void;
    let arg5 = self.5.qclsinst  as *const c_void;
    unsafe {_ZNK7QString3argERKS_S1_S1_S1_S1_S1_(arg0, arg1, arg2, arg3, arg4, arg5)};
    return 1;
  }
}

impl /*struct*/ QString {
  pub fn begin<T: QString_begin>(&mut self, value: T) -> i32 {
    value.begin(self);
    return 1;
  }
}

pub trait QString_begin {
  fn begin(self, this: &mut QString) -> i32;
}

// proto: QChar * QString::begin();
impl<'a> /*trait*/ QString_begin for () {
  fn begin(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString5beginEv()};
    unsafe {_ZN7QString5beginEv()};
    return 1;
  }
}

// proto: QString QString::number(double , char f, int prec);
impl<'a> /*trait*/ QString_number for (f64, i8, i32) {
  fn number(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString6numberEdci()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_char;
    let arg2 = self.2  as c_int;
    unsafe {_ZN7QString6numberEdci(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QString {
  pub fn end<T: QString_end>(&mut self, value: T) -> i32 {
    value.end(self);
    return 1;
  }
}

pub trait QString_end {
  fn end(self, this: &mut QString) -> i32;
}

// proto: QChar * QString::end();
impl<'a> /*trait*/ QString_end for () {
  fn end(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString3endEv()};
    unsafe {_ZN7QString3endEv()};
    return 1;
  }
}

// proto: QString & QString::append(QChar c);
impl<'a> /*trait*/ QString_append for (QChar) {
  fn append(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString6appendE5QChar()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN7QString6appendE5QChar(arg0)};
    return 1;
  }
}

impl /*struct*/ QString {
  pub fn toUInt<T: QString_toUInt>(&mut self, value: T) -> i32 {
    value.toUInt(self);
    return 1;
  }
}

pub trait QString_toUInt {
  fn toUInt(self, this: &mut QString) -> i32;
}

// proto: unsigned int QString::toUInt(bool * ok, int base);
impl<'a> /*trait*/ QString_toUInt for (&'a mut i8, i32) {
  fn toUInt(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString6toUIntEPbi()};
    let arg0 = self.0  as *mut int8_t;
    let arg1 = self.1  as c_int;
    unsafe {_ZNK7QString6toUIntEPbi(arg0, arg1)};
    return 1;
  }
}

// proto: QString & QString::append(const QString & s);
impl<'a> /*trait*/ QString_append for (&'a  QString) {
  fn append(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString6appendERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QString6appendERKS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QString {
  pub fn fromStdU16String<T: QString_fromStdU16String>(&mut self, value: T) -> i32 {
    value.fromStdU16String(self);
    return 1;
  }
}

pub trait QString_fromStdU16String {
  fn fromStdU16String(self, this: &mut QString) -> i32;
}

// proto: QString QString::fromStdU16String(const std::u16string & s);
impl<'a> /*trait*/ QString_fromStdU16String for (&'a  i32) {
  fn fromStdU16String(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString16fromStdU16StringERKi()};
    let arg0 = self  as *const c_int;
    unsafe {_ZN7QString16fromStdU16StringERKi(arg0)};
    return 1;
  }
}

// proto: QString QString::arg(qlonglong a, int fieldwidth, int base, QChar fillChar);
impl<'a> /*trait*/ QString_arg for (i64, i32, i32, QChar) {
  fn arg(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString3argExii5QChar()};
    let arg0 = self.0  as c_longlong;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3.qclsinst  as *mut c_void;
    unsafe {_ZNK7QString3argExii5QChar(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QString {
  pub fn toUShort<T: QString_toUShort>(&mut self, value: T) -> i32 {
    value.toUShort(self);
    return 1;
  }
}

pub trait QString_toUShort {
  fn toUShort(self, this: &mut QString) -> i32;
}

// proto: unsigned short QString::toUShort(bool * ok, int base);
impl<'a> /*trait*/ QString_toUShort for (&'a mut i8, i32) {
  fn toUShort(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString8toUShortEPbi()};
    let arg0 = self.0  as *mut int8_t;
    let arg1 = self.1  as c_int;
    unsafe {_ZNK7QString8toUShortEPbi(arg0, arg1)};
    return 1;
  }
}

// proto: QString QString::arg(uint a, int fieldWidth, int base, QChar fillChar);
impl<'a> /*trait*/ QString_arg for (u32, i32, i32, QChar) {
  fn arg(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString3argEjii5QChar()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3.qclsinst  as *mut c_void;
    unsafe {_ZNK7QString3argEjii5QChar(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

// proto: QString & QString::setNum(ushort , int base);
impl<'a> /*trait*/ QString_setNum for (u16, i32) {
  fn setNum(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString6setNumEti()};
    let arg0 = self.0  as c_ushort;
    let arg1 = self.1  as c_int;
    unsafe {_ZN7QString6setNumEti(arg0, arg1)};
    return 1;
  }
}

// proto: QString & QString::replace(const QRegularExpression & re, const QString & after);
impl<'a> /*trait*/ QString_replace for (&'a  QRegularExpression, &'a  QString) {
  fn replace(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString7replaceERK18QRegularExpressionRKS_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN7QString7replaceERK18QRegularExpressionRKS_(arg0, arg1)};
    return 1;
  }
}

// proto: QString & QString::setNum(double , char f, int prec);
impl<'a> /*trait*/ QString_setNum for (f64, i8, i32) {
  fn setNum(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString6setNumEdci()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_char;
    let arg2 = self.2  as c_int;
    unsafe {_ZN7QString6setNumEdci(arg0, arg1, arg2)};
    return 1;
  }
}

// proto: QString QString::number(qulonglong , int base);
impl<'a> /*trait*/ QString_number for (u64, i32) {
  fn number(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString6numberEyi()};
    let arg0 = self.0  as uint64_t;
    let arg1 = self.1  as c_int;
    unsafe {_ZN7QString6numberEyi(arg0, arg1)};
    return 1;
  }
}

// proto: QString QString::arg(ushort a, int fieldWidth, int base, QChar fillChar);
impl<'a> /*trait*/ QString_arg for (u16, i32, i32, QChar) {
  fn arg(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString3argEtii5QChar()};
    let arg0 = self.0  as c_ushort;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3.qclsinst  as *mut c_void;
    unsafe {_ZNK7QString3argEtii5QChar(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

// proto: void QString::NewQString(const QString & );
impl<'a> /*trait*/ QString_NewQString for (&'a  QString) {
  fn NewQString(self) -> QString {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QStringC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QStringC1ERKS_(qthis, arg0)};
    let rsthis = QString{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: QString QString::arg(short a, int fieldWidth, int base, QChar fillChar);
impl<'a> /*trait*/ QString_arg for (i16, i32, i32, QChar) {
  fn arg(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString3argEsii5QChar()};
    let arg0 = self.0  as c_short;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3.qclsinst  as *mut c_void;
    unsafe {_ZNK7QString3argEsii5QChar(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

// proto: void QString::NewQString(const QByteArray & a);
impl<'a> /*trait*/ QString_NewQString for (&'a  QByteArray) {
  fn NewQString(self) -> QString {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QStringC1ERK10QByteArray()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QStringC1ERK10QByteArray(qthis, arg0)};
    let rsthis = QString{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QString {
  pub fn toULongLong<T: QString_toULongLong>(&mut self, value: T) -> i32 {
    value.toULongLong(self);
    return 1;
  }
}

pub trait QString_toULongLong {
  fn toULongLong(self, this: &mut QString) -> i32;
}

// proto: quint64 QString::toULongLong(bool * ok, int base);
impl<'a> /*trait*/ QString_toULongLong for (&'a mut i8, i32) {
  fn toULongLong(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString11toULongLongEPbi()};
    let arg0 = self.0  as *mut int8_t;
    let arg1 = self.1  as c_int;
    unsafe {_ZNK7QString11toULongLongEPbi(arg0, arg1)};
    return 1;
  }
}

// proto: QString & QString::append(const char * s);
impl<'a> /*trait*/ QString_append for (&'a  String) {
  fn append(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString6appendEPKc()};
    let arg0 = self.as_ptr()  as *const c_char;
    unsafe {_ZN7QString6appendEPKc(arg0)};
    return 1;
  }
}

impl /*struct*/ QString {
  pub fn capacity<T: QString_capacity>(&mut self, value: T) -> i32 {
    value.capacity(self);
    return 1;
  }
}

pub trait QString_capacity {
  fn capacity(self, this: &mut QString) -> i32;
}

// proto: int QString::capacity();
impl<'a> /*trait*/ QString_capacity for () {
  fn capacity(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString8capacityEv()};
    unsafe {_ZNK7QString8capacityEv()};
    return 1;
  }
}

impl /*struct*/ QString {
  pub fn squeeze<T: QString_squeeze>(&mut self, value: T) -> i32 {
    value.squeeze(self);
    return 1;
  }
}

pub trait QString_squeeze {
  fn squeeze(self, this: &mut QString) -> i32;
}

// proto: void QString::squeeze();
impl<'a> /*trait*/ QString_squeeze for () {
  fn squeeze(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString7squeezeEv()};
    unsafe {_ZN7QString7squeezeEv()};
    return 1;
  }
}

impl /*struct*/ QString {
  pub fn truncate<T: QString_truncate>(&mut self, value: T) -> i32 {
    value.truncate(self);
    return 1;
  }
}

pub trait QString_truncate {
  fn truncate(self, this: &mut QString) -> i32;
}

// proto: void QString::truncate(int pos);
impl<'a> /*trait*/ QString_truncate for (i32) {
  fn truncate(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString8truncateEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN7QString8truncateEi(arg0)};
    return 1;
  }
}

// proto: QString QString::arg(QChar a, int fieldWidth, QChar fillChar);
impl<'a> /*trait*/ QString_arg for (QChar, i32, QChar) {
  fn arg(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString3argE5QChariS0_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    unsafe {_ZNK7QString3argE5QChariS0_(arg0, arg1, arg2)};
    return 1;
  }
}

// proto: int QString::localeAwareCompare(const QString & s);
impl<'a> /*trait*/ QString_localeAwareCompare for (&'a  QString) {
  fn localeAwareCompare(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString18localeAwareCompareERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK7QString18localeAwareCompareERKS_(arg0)};
    return 1;
  }
}

// proto: QString & QString::remove(const QRegExp & rx);
impl<'a> /*trait*/ QString_remove for (&'a  QRegExp) {
  fn remove(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString6removeERK7QRegExp()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QString6removeERK7QRegExp(arg0)};
    return 1;
  }
}

// proto: bool QString::contains(const QRegularExpression & re);
impl<'a> /*trait*/ QString_contains for (&'a  QRegularExpression) {
  fn contains(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString8containsERK18QRegularExpression()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK7QString8containsERK18QRegularExpression(arg0)};
    return 1;
  }
}

// proto: int QString::indexOf(QRegExp & , int from);
impl<'a> /*trait*/ QString_indexOf for (&'a mut QRegExp, i32) {
  fn indexOf(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString7indexOfER7QRegExpi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    unsafe {_ZNK7QString7indexOfER7QRegExpi(arg0, arg1)};
    return 1;
  }
}

// proto: QString & QString::replace(int i, int len, QChar after);
impl<'a> /*trait*/ QString_replace for (i32, i32, QChar) {
  fn replace(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString7replaceEii5QChar()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    unsafe {_ZN7QString7replaceEii5QChar(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QString {
  pub fn isRightToLeft<T: QString_isRightToLeft>(&mut self, value: T) -> i32 {
    value.isRightToLeft(self);
    return 1;
  }
}

pub trait QString_isRightToLeft {
  fn isRightToLeft(self, this: &mut QString) -> i32;
}

// proto: bool QString::isRightToLeft();
impl<'a> /*trait*/ QString_isRightToLeft for () {
  fn isRightToLeft(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString13isRightToLeftEv()};
    unsafe {_ZNK7QString13isRightToLeftEv()};
    return 1;
  }
}

// proto: QString QString::arg(char a, int fieldWidth, QChar fillChar);
impl<'a> /*trait*/ QString_arg for (i8, i32, QChar) {
  fn arg(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString3argEci5QChar()};
    let arg0 = self.0  as c_char;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    unsafe {_ZNK7QString3argEci5QChar(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QString {
  pub fn toUcs4<T: QString_toUcs4>(&mut self, value: T) -> i32 {
    value.toUcs4(self);
    return 1;
  }
}

pub trait QString_toUcs4 {
  fn toUcs4(self, this: &mut QString) -> i32;
}

// proto: QVector<uint> QString::toUcs4();
impl<'a> /*trait*/ QString_toUcs4 for () {
  fn toUcs4(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString6toUcs4Ev()};
    unsafe {_ZNK7QString6toUcs4Ev()};
    return 1;
  }
}

// proto: QString & QString::remove(int i, int len);
impl<'a> /*trait*/ QString_remove for (i32, i32) {
  fn remove(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString6removeEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZN7QString6removeEii(arg0, arg1)};
    return 1;
  }
}

// proto: int QString::lastIndexOf(const QRegularExpression & re, int from, QRegularExpressionMatch * rmatch);
impl<'a> /*trait*/ QString_lastIndexOf for (&'a  QRegularExpression, i32, &'a mut QRegularExpressionMatch) {
  fn lastIndexOf(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString11lastIndexOfERK18QRegularExpressioniP23QRegularExpressionMatch()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    unsafe {_ZNK7QString11lastIndexOfERK18QRegularExpressioniP23QRegularExpressionMatch(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QString {
  pub fn right<T: QString_right>(&mut self, value: T) -> i32 {
    value.right(self);
    return 1;
  }
}

pub trait QString_right {
  fn right(self, this: &mut QString) -> i32;
}

// proto: QString QString::right(int n);
impl<'a> /*trait*/ QString_right for (i32) {
  fn right(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString5rightEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK7QString5rightEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QString {
  pub fn rightJustified<T: QString_rightJustified>(&mut self, value: T) -> i32 {
    value.rightJustified(self);
    return 1;
  }
}

pub trait QString_rightJustified {
  fn rightJustified(self, this: &mut QString) -> i32;
}

// proto: QString QString::rightJustified(int width, QChar fill, bool trunc);
impl<'a> /*trait*/ QString_rightJustified for (i32, QChar, i8) {
  fn rightJustified(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString14rightJustifiedEi5QCharb()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as int8_t;
    unsafe {_ZNK7QString14rightJustifiedEi5QCharb(arg0, arg1, arg2)};
    return 1;
  }
}

// proto: QString QString::arg(const QString & a, int fieldWidth, QChar fillChar);
impl<'a> /*trait*/ QString_arg for (&'a  QString, i32, QChar) {
  fn arg(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString3argERKS_i5QChar()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    unsafe {_ZNK7QString3argERKS_i5QChar(arg0, arg1, arg2)};
    return 1;
  }
}

// proto: QString QString::arg(qulonglong a, int fieldwidth, int base, QChar fillChar);
impl<'a> /*trait*/ QString_arg for (u64, i32, i32, QChar) {
  fn arg(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString3argEyii5QChar()};
    let arg0 = self.0  as uint64_t;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3.qclsinst  as *mut c_void;
    unsafe {_ZNK7QString3argEyii5QChar(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QString {
  pub fn reserve<T: QString_reserve>(&mut self, value: T) -> i32 {
    value.reserve(self);
    return 1;
  }
}

pub trait QString_reserve {
  fn reserve(self, this: &mut QString) -> i32;
}

// proto: void QString::reserve(int size);
impl<'a> /*trait*/ QString_reserve for (i32) {
  fn reserve(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QString7reserveEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN7QString7reserveEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QString {
  pub fn toShort<T: QString_toShort>(&mut self, value: T) -> i32 {
    value.toShort(self);
    return 1;
  }
}

pub trait QString_toShort {
  fn toShort(self, this: &mut QString) -> i32;
}

// proto: short QString::toShort(bool * ok, int base);
impl<'a> /*trait*/ QString_toShort for (&'a mut i8, i32) {
  fn toShort(self, this: &mut QString) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QString7toShortEPbi()};
    let arg0 = self.0  as *mut int8_t;
    let arg1 = self.1  as c_int;
    unsafe {_ZNK7QString7toShortEPbi(arg0, arg1)};
    return 1;
  }
}

