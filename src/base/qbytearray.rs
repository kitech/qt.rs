// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstring::QString;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  fn _ZN10QByteArray6insertEic(arg0: c_int, arg1: c_char) -> i32;
  fn _ZNK10QByteArray4cendEv() -> i32;
  fn _ZNK10QByteArray8toBase64E6QFlagsINS_12Base64OptionEE(arg0: c_int) -> i32;
  fn _ZNK10QByteArray11lastIndexOfERKS_i(arg0: *const c_void, arg1: c_int) -> i32;
  fn _ZN10QByteArray10push_frontERKS_(arg0: *const c_void) -> i32;
  fn _ZNK10QByteArray7toULongEPbi(arg0: *mut int8_t, arg1: c_int) -> i32;
  fn _ZN10QByteArray7replaceEPKcS1_(arg0: *const c_char, arg1: *const c_char) -> i32;
  fn _ZN10QByteArray7replaceERKS_S1_(arg0: *const c_void, arg1: *const c_void) -> i32;
  fn _ZN10QByteArray7fromHexERKS_(arg0: *const c_void) -> i32;
  fn _ZN10QByteArray7prependEPKc(arg0: *const c_char) -> i32;
  fn _ZNK10QByteArray5countERKS_(arg0: *const c_void) -> i32;
  fn _ZN10QByteArrayD0Ev() -> i32;
  fn _ZN10QByteArray3endEv() -> i32;
  fn _ZN10QByteArrayC1Ev(qthis: *mut c_void) -> i32;
  fn _ZN10QByteArray7replaceEPKcRKS_(arg0: *const c_char, arg1: *const c_void) -> i32;
  fn _ZNK10QByteArray7toFloatEPb(arg0: *mut int8_t) -> i32;
  fn _ZN10QByteArray8truncateEi(arg0: c_int) -> i32;
  fn _ZNK10QByteArray8toBase64Ev() -> i32;
  fn _ZNK10QByteArray7isEmptyEv() -> i32;
  fn _ZN10QByteArray6insertEiPKci(arg0: c_int, arg1: *const c_char, arg2: c_int) -> i32;
  fn _ZN10QByteArray6insertEiRK7QString(arg0: c_int, arg1: *const c_void) -> i32;
  fn _ZN10QByteArray6resizeEi(arg0: c_int) -> i32;
  fn _ZN10QByteArray7replaceEiiPKci(arg0: c_int, arg1: c_int, arg2: *const c_char, arg3: c_int) -> i32;
  fn _ZNK10QByteArray11lastIndexOfERK7QStringi(arg0: *const c_void, arg1: c_int) -> i32;
  fn _ZNK10QByteArray5toHexEv() -> i32;
  fn _ZNK10QByteArray7indexOfEPKci(arg0: *const c_char, arg1: c_int) -> i32;
  fn _ZN10QByteArray7replaceEcRKS_(arg0: c_char, arg1: *const c_void) -> i32;
  fn _ZNK10QByteArray6toUIntEPbi(arg0: *mut int8_t, arg1: c_int) -> i32;
  fn _ZN10QByteArray13fromStdStringERKi(arg0: *const c_int) -> i32;
  fn _ZNK10QByteArray6isNullEv() -> i32;
  fn _ZN10QByteArray7reserveEi(arg0: c_int) -> i32;
  fn _ZNK10QByteArray6cbeginEv() -> i32;
  fn _ZN10QByteArray11fromRawDataEPKci(arg0: *const c_char, arg1: c_int) -> i32;
  fn _ZNK10QByteArray8containsEc(arg0: c_char) -> i32;
  fn _ZN10QByteArrayC1Eic(qthis: *mut c_void, arg0: c_int, arg1: c_char) -> i32;
  fn _ZNK10QByteArray7indexOfERKS_i(arg0: *const c_void, arg1: c_int) -> i32;
  fn _ZNK10QByteArray6toLongEPbi(arg0: *mut int8_t, arg1: c_int) -> i32;
  fn _ZNK10QByteArray7indexOfEci(arg0: c_char, arg1: c_int) -> i32;
  fn _ZN10QByteArray4dataEv() -> i32;
  fn _ZN10QByteArray6numberEdci(arg0: c_double, arg1: c_char, arg2: c_int) -> i32;
  fn _ZNK10QByteArray8capacityEv() -> i32;
  fn _ZNK10QByteArray5countEv() -> i32;
  fn _ZN10QByteArray10fromBase64ERKS_(arg0: *const c_void) -> i32;
  fn _ZNK10QByteArray4leftEi(arg0: c_int) -> i32;
  fn _ZN10QByteArray7replaceEcc(arg0: c_char, arg1: c_char) -> i32;
  fn _ZN10QByteArray6appendEc(arg0: c_char) -> i32;
  fn _ZNK10QByteArray10startsWithEPKc(arg0: *const c_char) -> i32;
  fn _ZN10QByteArray6removeEii(arg0: c_int, arg1: c_int) -> i32;
  fn _ZNK10QByteArray11lastIndexOfEci(arg0: c_char, arg1: c_int) -> i32;
  fn _ZNK10QByteArray10startsWithERKS_(arg0: *const c_void) -> i32;
  fn _ZNK10QByteArray8containsEPKc(arg0: *const c_char) -> i32;
  fn _ZNK10QByteArray8endsWithEPKc(arg0: *const c_char) -> i32;
  fn _ZN10QByteArray7squeezeEv() -> i32;
  fn _ZN10QByteArrayC1EPKci(qthis: *mut c_void, arg0: *const c_char, arg1: c_int) -> i32;
  fn _ZNK10QByteArray7indexOfERK7QStringi(arg0: *const c_void, arg1: c_int) -> i32;
  fn _ZN10QByteArray6detachEv() -> i32;
  fn _ZNK10QByteArray8repeatedEi(arg0: c_int) -> i32;
  fn _ZN10QByteArray6setNumEfci(arg0: c_float, arg1: c_char, arg2: c_int) -> i32;
  fn _ZNK10QByteArray7toShortEPbi(arg0: *mut int8_t, arg1: c_int) -> i32;
  fn _ZN10QByteArray7prependEc(arg0: c_char) -> i32;
  fn _ZNK10QByteArray5toIntEPbi(arg0: *mut int8_t, arg1: c_int) -> i32;
  fn _ZNK10QByteArray10constBeginEv() -> i32;
  fn _ZN10QByteArray9push_backEc(arg0: c_char) -> i32;
  fn _ZNK10QByteArray12isSharedWithERKS_(arg0: *const c_void) -> i32;
  fn _ZN10QByteArray10fromBase64ERKS_6QFlagsINS_12Base64OptionEE(arg0: *const c_void, arg1: c_int) -> i32;
  fn _ZNK10QByteArray4sizeEv() -> i32;
  fn _ZNK10QByteArray8endsWithEc(arg0: c_char) -> i32;
  fn _ZN10QByteArray6numberEji(arg0: c_uint, arg1: c_int) -> i32;
  fn _ZN10QByteArray10push_frontEc(arg0: c_char) -> i32;
  fn _ZNK10QByteArray13leftJustifiedEicb(arg0: c_int, arg1: c_char, arg2: int8_t) -> i32;
  fn _ZN10QByteArray5beginEv() -> i32;
  fn _ZN10QByteArray6numberEyi(arg0: uint64_t, arg1: c_int) -> i32;
  fn _ZNK10QByteArray5countEc(arg0: c_char) -> i32;
  fn _ZNK10QByteArray8toDoubleEPb(arg0: *mut int8_t) -> i32;
  fn _ZN10QByteArray7replaceEiiRKS_(arg0: c_int, arg1: c_int, arg2: *const c_void) -> i32;
  fn _ZN10QByteArray6setNumEsi(arg0: c_short, arg1: c_int) -> i32;
  fn _ZN10QByteArray7prependERKS_(arg0: *const c_void) -> i32;
  fn _ZNK10QByteArray11toULongLongEPbi(arg0: *mut int8_t, arg1: c_int) -> i32;
  fn _ZN10QByteArray7replaceEcRK7QString(arg0: c_char, arg1: *const c_void) -> i32;
  fn _ZN10QByteArray19fromPercentEncodingERKS_c(arg0: *const c_void, arg1: c_char) -> i32;
  fn _ZN10QByteArray10push_frontEPKc(arg0: *const c_char) -> i32;
  fn _ZN10QByteArray5clearEv() -> i32;
  fn _ZNK10QByteArray10toLongLongEPbi(arg0: *mut int8_t, arg1: c_int) -> i32;
  fn _ZN10QByteArray7prependEPKci(arg0: *const c_char, arg1: c_int) -> i32;
  fn _ZNK10QByteArray9constDataEv() -> i32;
  fn _ZN10QByteArrayC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZNK10QByteArray6lengthEv() -> i32;
  fn _ZN10QByteArray6numberEii(arg0: c_int, arg1: c_int) -> i32;
  fn _ZNK10QByteArray10startsWithEc(arg0: c_char) -> i32;
  fn _ZN10QByteArray6setNumEdci(arg0: c_double, arg1: c_char, arg2: c_int) -> i32;
  fn _ZN10QByteArray6numberExi(arg0: c_longlong, arg1: c_int) -> i32;
  fn _ZNK10QByteArray2atEi(arg0: c_int) -> i32;
  fn _ZN10QByteArray6setNumEti(arg0: c_ushort, arg1: c_int) -> i32;
  fn _ZN10QByteArray4swapERS_(arg0: *mut c_void) -> i32;
  fn _ZN10QByteArray7replaceERK7QStringPKc(arg0: *const c_void, arg1: *const c_char) -> i32;
  fn _ZN10QByteArray6appendERKS_(arg0: *const c_void) -> i32;
  fn _ZNK10QByteArray8endsWithERKS_(arg0: *const c_void) -> i32;
  fn _ZNK10QByteArray5countEPKc(arg0: *const c_char) -> i32;
  fn _ZN10QByteArray7replaceEPKciS1_i(arg0: *const c_char, arg1: c_int, arg2: *const c_char, arg3: c_int) -> i32;
  fn _ZNK10QByteArray5splitEc(arg0: c_char) -> i32;
  fn _ZN10QByteArray6setNumExi(arg0: c_longlong, arg1: c_int) -> i32;
  fn _ZN10QByteArray7replaceEcPKc(arg0: c_char, arg1: *const c_char) -> i32;
  fn _ZN10QByteArray6appendEPKc(arg0: *const c_char) -> i32;
  fn _ZNK10QByteArray5rightEi(arg0: c_int) -> i32;
  fn _ZN10QByteArray6appendERK7QString(arg0: *const c_void) -> i32;
  fn _ZN10QByteArray4chopEi(arg0: c_int) -> i32;
  fn _ZNK10QByteArray11lastIndexOfEPKci(arg0: *const c_char, arg1: c_int) -> i32;
  fn _ZN10QByteArray7replaceEiiPKc(arg0: c_int, arg1: c_int, arg2: *const c_char) -> i32;
  fn _ZN10QByteArray9push_backERKS_(arg0: *const c_void) -> i32;
  fn _ZNK10QByteArray17toPercentEncodingERKS_S1_c(arg0: *const c_void, arg1: *const c_void, arg2: c_char) -> i32;
  fn _ZNK10QByteArray10isDetachedEv() -> i32;
  fn _ZN10QByteArray6appendEPKci(arg0: *const c_char, arg1: c_int) -> i32;
  fn _ZNK10QByteArray8constEndEv() -> i32;
  fn _ZN10QByteArray7replaceERKS_PKc(arg0: *const c_void, arg1: *const c_char) -> i32;
  fn _ZN10QByteArray6setNumEyi(arg0: uint64_t, arg1: c_int) -> i32;
  fn _ZN10QByteArray10setRawDataEPKcj(arg0: *const c_char, arg1: c_uint) -> i32;
  fn _ZN10QByteArray7replaceERK7QStringRKS_(arg0: *const c_void, arg1: *const c_void) -> i32;
  fn _ZN10QByteArray6setNumEji(arg0: c_uint, arg1: c_int) -> i32;
  fn _ZNK10QByteArray3midEii(arg0: c_int, arg1: c_int) -> i32;
  fn _ZN10QByteArray6setNumEii(arg0: c_int, arg1: c_int) -> i32;
  fn _ZN10QByteArray6insertEiRKS_(arg0: c_int, arg1: *const c_void) -> i32;
  fn _ZN10QByteArray6insertEiPKc(arg0: c_int, arg1: *const c_char) -> i32;
  fn _ZN10QByteArray4fillEci(arg0: c_char, arg1: c_int) -> i32;
  fn _ZNK10QByteArray8toUShortEPbi(arg0: *mut int8_t, arg1: c_int) -> i32;
  fn _ZN10QByteArray9push_backEPKc(arg0: *const c_char) -> i32;
  fn _ZNK10QByteArray14rightJustifiedEicb(arg0: c_int, arg1: c_char, arg2: int8_t) -> i32;
  fn _ZNK10QByteArray8containsERKS_(arg0: *const c_void) -> i32;
}

// body block begin
// class sizeof(QByteArray)=8
pub struct QByteArray {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QByteArray {
  pub fn insert<T: QByteArray_insert>(&mut self, value: T) -> i32 {
    value.insert(self);
    return 1;
  }
}

pub trait QByteArray_insert {
  fn insert(self, this: &mut QByteArray) -> i32;
}

// proto: QByteArray & QByteArray::insert(int i, char c);
impl<'a> /*trait*/ QByteArray_insert for (i32, i8) {
  fn insert(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray6insertEic()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_char;
    unsafe {_ZN10QByteArray6insertEic(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QByteArray {
  pub fn cend<T: QByteArray_cend>(&mut self, value: T) -> i32 {
    value.cend(self);
    return 1;
  }
}

pub trait QByteArray_cend {
  fn cend(self, this: &mut QByteArray) -> i32;
}

// proto: const char * QByteArray::cend();
impl<'a> /*trait*/ QByteArray_cend for () {
  fn cend(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray4cendEv()};
    unsafe {_ZNK10QByteArray4cendEv()};
    return 1;
  }
}

impl /*struct*/ QByteArray {
  pub fn toBase64<T: QByteArray_toBase64>(&mut self, value: T) -> i32 {
    value.toBase64(self);
    return 1;
  }
}

pub trait QByteArray_toBase64 {
  fn toBase64(self, this: &mut QByteArray) -> i32;
}

// proto: QByteArray QByteArray::toBase64(Base64Options options);
impl<'a> /*trait*/ QByteArray_toBase64 for (i32) {
  fn toBase64(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray8toBase64E6QFlagsINS_12Base64OptionEE()};
    let arg0 = self  as c_int;
    unsafe {_ZNK10QByteArray8toBase64E6QFlagsINS_12Base64OptionEE(arg0)};
    return 1;
  }
}

impl /*struct*/ QByteArray {
  pub fn lastIndexOf<T: QByteArray_lastIndexOf>(&mut self, value: T) -> i32 {
    value.lastIndexOf(self);
    return 1;
  }
}

pub trait QByteArray_lastIndexOf {
  fn lastIndexOf(self, this: &mut QByteArray) -> i32;
}

// proto: int QByteArray::lastIndexOf(const QByteArray & a, int from);
impl<'a> /*trait*/ QByteArray_lastIndexOf for (&'a  QByteArray, i32) {
  fn lastIndexOf(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray11lastIndexOfERKS_i()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_int;
    unsafe {_ZNK10QByteArray11lastIndexOfERKS_i(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QByteArray {
  pub fn push_front<T: QByteArray_push_front>(&mut self, value: T) -> i32 {
    value.push_front(self);
    return 1;
  }
}

pub trait QByteArray_push_front {
  fn push_front(self, this: &mut QByteArray) -> i32;
}

// proto: void QByteArray::push_front(const QByteArray & a);
impl<'a> /*trait*/ QByteArray_push_front for (&'a  QByteArray) {
  fn push_front(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray10push_frontERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN10QByteArray10push_frontERKS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QByteArray {
  pub fn toULong<T: QByteArray_toULong>(&mut self, value: T) -> i32 {
    value.toULong(self);
    return 1;
  }
}

pub trait QByteArray_toULong {
  fn toULong(self, this: &mut QByteArray) -> i32;
}

// proto: unsigned long QByteArray::toULong(bool * ok, int base);
impl<'a> /*trait*/ QByteArray_toULong for (&'a mut i8, i32) {
  fn toULong(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray7toULongEPbi()};
    let arg0 = self.0  as *mut int8_t;
    let arg1 = self.1  as c_int;
    unsafe {_ZNK10QByteArray7toULongEPbi(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QByteArray {
  pub fn replace<T: QByteArray_replace>(&mut self, value: T) -> i32 {
    value.replace(self);
    return 1;
  }
}

pub trait QByteArray_replace {
  fn replace(self, this: &mut QByteArray) -> i32;
}

// proto: QByteArray & QByteArray::replace(const char * before, const char * after);
impl<'a> /*trait*/ QByteArray_replace for (&'a  String, &'a  String) {
  fn replace(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray7replaceEPKcS1_()};
    let arg0 = self.0.as_ptr()  as *const c_char;
    let arg1 = self.1.as_ptr()  as *const c_char;
    unsafe {_ZN10QByteArray7replaceEPKcS1_(arg0, arg1)};
    return 1;
  }
}

// proto: QByteArray & QByteArray::replace(const QByteArray & before, const QByteArray & after);
impl<'a> /*trait*/ QByteArray_replace for (&'a  QByteArray, &'a  QByteArray) {
  fn replace(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray7replaceERKS_S1_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN10QByteArray7replaceERKS_S1_(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QByteArray {
  pub fn fromHex<T: QByteArray_fromHex>(&mut self, value: T) -> i32 {
    value.fromHex(self);
    return 1;
  }
}

pub trait QByteArray_fromHex {
  fn fromHex(self, this: &mut QByteArray) -> i32;
}

// proto: QByteArray QByteArray::fromHex(const QByteArray & hexEncoded);
impl<'a> /*trait*/ QByteArray_fromHex for (&'a  QByteArray) {
  fn fromHex(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray7fromHexERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN10QByteArray7fromHexERKS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QByteArray {
  pub fn prepend<T: QByteArray_prepend>(&mut self, value: T) -> i32 {
    value.prepend(self);
    return 1;
  }
}

pub trait QByteArray_prepend {
  fn prepend(self, this: &mut QByteArray) -> i32;
}

// proto: QByteArray & QByteArray::prepend(const char * s);
impl<'a> /*trait*/ QByteArray_prepend for (&'a  String) {
  fn prepend(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray7prependEPKc()};
    let arg0 = self.as_ptr()  as *const c_char;
    unsafe {_ZN10QByteArray7prependEPKc(arg0)};
    return 1;
  }
}

impl /*struct*/ QByteArray {
  pub fn count<T: QByteArray_count>(&mut self, value: T) -> i32 {
    value.count(self);
    return 1;
  }
}

pub trait QByteArray_count {
  fn count(self, this: &mut QByteArray) -> i32;
}

// proto: int QByteArray::count(const QByteArray & a);
impl<'a> /*trait*/ QByteArray_count for (&'a  QByteArray) {
  fn count(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray5countERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK10QByteArray5countERKS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QByteArray {
  pub fn FreeQByteArray<T: QByteArray_FreeQByteArray>(&mut self, value: T) -> i32 {
    value.FreeQByteArray(self);
    return 1;
  }
}

pub trait QByteArray_FreeQByteArray {
  fn FreeQByteArray(self, this: &mut QByteArray) -> i32;
}

// proto: void QByteArray::FreeQByteArray();
impl<'a> /*trait*/ QByteArray_FreeQByteArray for () {
  fn FreeQByteArray(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArrayD0Ev()};
    unsafe {_ZN10QByteArrayD0Ev()};
    return 1;
  }
}

impl /*struct*/ QByteArray {
  pub fn end<T: QByteArray_end>(&mut self, value: T) -> i32 {
    value.end(self);
    return 1;
  }
}

pub trait QByteArray_end {
  fn end(self, this: &mut QByteArray) -> i32;
}

// proto: char * QByteArray::end();
impl<'a> /*trait*/ QByteArray_end for () {
  fn end(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray3endEv()};
    unsafe {_ZN10QByteArray3endEv()};
    return 1;
  }
}

impl /*struct*/ QByteArray {
  pub fn NewQByteArray<T: QByteArray_NewQByteArray>(value: T) -> QByteArray {
    let rsthis = value.NewQByteArray();
    return rsthis;
    // return 1;
  }
}

pub trait QByteArray_NewQByteArray {
  fn NewQByteArray(self) -> QByteArray;
}

// proto: void QByteArray::NewQByteArray();
impl<'a> /*trait*/ QByteArray_NewQByteArray for () {
  fn NewQByteArray(self) -> QByteArray {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArrayC1Ev()};
    unsafe {_ZN10QByteArrayC1Ev(qthis)};
    let rsthis = QByteArray{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: QByteArray & QByteArray::replace(const char * before, const QByteArray & after);
impl<'a> /*trait*/ QByteArray_replace for (&'a  String, &'a  QByteArray) {
  fn replace(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray7replaceEPKcRKS_()};
    let arg0 = self.0.as_ptr()  as *const c_char;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN10QByteArray7replaceEPKcRKS_(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QByteArray {
  pub fn toFloat<T: QByteArray_toFloat>(&mut self, value: T) -> i32 {
    value.toFloat(self);
    return 1;
  }
}

pub trait QByteArray_toFloat {
  fn toFloat(self, this: &mut QByteArray) -> i32;
}

// proto: float QByteArray::toFloat(bool * ok);
impl<'a> /*trait*/ QByteArray_toFloat for (&'a mut i8) {
  fn toFloat(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray7toFloatEPb()};
    let arg0 = self  as *mut int8_t;
    unsafe {_ZNK10QByteArray7toFloatEPb(arg0)};
    return 1;
  }
}

impl /*struct*/ QByteArray {
  pub fn truncate<T: QByteArray_truncate>(&mut self, value: T) -> i32 {
    value.truncate(self);
    return 1;
  }
}

pub trait QByteArray_truncate {
  fn truncate(self, this: &mut QByteArray) -> i32;
}

// proto: void QByteArray::truncate(int pos);
impl<'a> /*trait*/ QByteArray_truncate for (i32) {
  fn truncate(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray8truncateEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN10QByteArray8truncateEi(arg0)};
    return 1;
  }
}

// proto: QByteArray QByteArray::toBase64();
impl<'a> /*trait*/ QByteArray_toBase64 for () {
  fn toBase64(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray8toBase64Ev()};
    unsafe {_ZNK10QByteArray8toBase64Ev()};
    return 1;
  }
}

impl /*struct*/ QByteArray {
  pub fn isEmpty<T: QByteArray_isEmpty>(&mut self, value: T) -> i32 {
    value.isEmpty(self);
    return 1;
  }
}

pub trait QByteArray_isEmpty {
  fn isEmpty(self, this: &mut QByteArray) -> i32;
}

// proto: bool QByteArray::isEmpty();
impl<'a> /*trait*/ QByteArray_isEmpty for () {
  fn isEmpty(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray7isEmptyEv()};
    unsafe {_ZNK10QByteArray7isEmptyEv()};
    return 1;
  }
}

// proto: QByteArray & QByteArray::insert(int i, const char * s, int len);
impl<'a> /*trait*/ QByteArray_insert for (i32, &'a  String, i32) {
  fn insert(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray6insertEiPKci()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.as_ptr()  as *const c_char;
    let arg2 = self.2  as c_int;
    unsafe {_ZN10QByteArray6insertEiPKci(arg0, arg1, arg2)};
    return 1;
  }
}

// proto: QByteArray & QByteArray::insert(int i, const QString & s);
impl<'a> /*trait*/ QByteArray_insert for (i32, &'a  QString) {
  fn insert(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray6insertEiRK7QString()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN10QByteArray6insertEiRK7QString(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QByteArray {
  pub fn resize<T: QByteArray_resize>(&mut self, value: T) -> i32 {
    value.resize(self);
    return 1;
  }
}

pub trait QByteArray_resize {
  fn resize(self, this: &mut QByteArray) -> i32;
}

// proto: void QByteArray::resize(int size);
impl<'a> /*trait*/ QByteArray_resize for (i32) {
  fn resize(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray6resizeEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN10QByteArray6resizeEi(arg0)};
    return 1;
  }
}

// proto: QByteArray & QByteArray::replace(int index, int len, const char * s, int alen);
impl<'a> /*trait*/ QByteArray_replace for (i32, i32, &'a  String, i32) {
  fn replace(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray7replaceEiiPKci()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.as_ptr()  as *const c_char;
    let arg3 = self.3  as c_int;
    unsafe {_ZN10QByteArray7replaceEiiPKci(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

// proto: int QByteArray::lastIndexOf(const QString & s, int from);
impl<'a> /*trait*/ QByteArray_lastIndexOf for (&'a  QString, i32) {
  fn lastIndexOf(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray11lastIndexOfERK7QStringi()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_int;
    unsafe {_ZNK10QByteArray11lastIndexOfERK7QStringi(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QByteArray {
  pub fn toHex<T: QByteArray_toHex>(&mut self, value: T) -> i32 {
    value.toHex(self);
    return 1;
  }
}

pub trait QByteArray_toHex {
  fn toHex(self, this: &mut QByteArray) -> i32;
}

// proto: QByteArray QByteArray::toHex();
impl<'a> /*trait*/ QByteArray_toHex for () {
  fn toHex(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray5toHexEv()};
    unsafe {_ZNK10QByteArray5toHexEv()};
    return 1;
  }
}

impl /*struct*/ QByteArray {
  pub fn indexOf<T: QByteArray_indexOf>(&mut self, value: T) -> i32 {
    value.indexOf(self);
    return 1;
  }
}

pub trait QByteArray_indexOf {
  fn indexOf(self, this: &mut QByteArray) -> i32;
}

// proto: int QByteArray::indexOf(const char * c, int from);
impl<'a> /*trait*/ QByteArray_indexOf for (&'a  String, i32) {
  fn indexOf(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray7indexOfEPKci()};
    let arg0 = self.0.as_ptr()  as *const c_char;
    let arg1 = self.1  as c_int;
    unsafe {_ZNK10QByteArray7indexOfEPKci(arg0, arg1)};
    return 1;
  }
}

// proto: QByteArray & QByteArray::replace(char before, const QByteArray & after);
impl<'a> /*trait*/ QByteArray_replace for (i8, &'a  QByteArray) {
  fn replace(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray7replaceEcRKS_()};
    let arg0 = self.0  as c_char;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN10QByteArray7replaceEcRKS_(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QByteArray {
  pub fn toUInt<T: QByteArray_toUInt>(&mut self, value: T) -> i32 {
    value.toUInt(self);
    return 1;
  }
}

pub trait QByteArray_toUInt {
  fn toUInt(self, this: &mut QByteArray) -> i32;
}

// proto: unsigned int QByteArray::toUInt(bool * ok, int base);
impl<'a> /*trait*/ QByteArray_toUInt for (&'a mut i8, i32) {
  fn toUInt(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray6toUIntEPbi()};
    let arg0 = self.0  as *mut int8_t;
    let arg1 = self.1  as c_int;
    unsafe {_ZNK10QByteArray6toUIntEPbi(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QByteArray {
  pub fn fromStdString<T: QByteArray_fromStdString>(&mut self, value: T) -> i32 {
    value.fromStdString(self);
    return 1;
  }
}

pub trait QByteArray_fromStdString {
  fn fromStdString(self, this: &mut QByteArray) -> i32;
}

// proto: QByteArray QByteArray::fromStdString(const std::string & s);
impl<'a> /*trait*/ QByteArray_fromStdString for (&'a  i32) {
  fn fromStdString(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray13fromStdStringERKi()};
    let arg0 = self  as *const c_int;
    unsafe {_ZN10QByteArray13fromStdStringERKi(arg0)};
    return 1;
  }
}

impl /*struct*/ QByteArray {
  pub fn isNull<T: QByteArray_isNull>(&mut self, value: T) -> i32 {
    value.isNull(self);
    return 1;
  }
}

pub trait QByteArray_isNull {
  fn isNull(self, this: &mut QByteArray) -> i32;
}

// proto: bool QByteArray::isNull();
impl<'a> /*trait*/ QByteArray_isNull for () {
  fn isNull(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray6isNullEv()};
    unsafe {_ZNK10QByteArray6isNullEv()};
    return 1;
  }
}

impl /*struct*/ QByteArray {
  pub fn reserve<T: QByteArray_reserve>(&mut self, value: T) -> i32 {
    value.reserve(self);
    return 1;
  }
}

pub trait QByteArray_reserve {
  fn reserve(self, this: &mut QByteArray) -> i32;
}

// proto: void QByteArray::reserve(int size);
impl<'a> /*trait*/ QByteArray_reserve for (i32) {
  fn reserve(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray7reserveEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN10QByteArray7reserveEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QByteArray {
  pub fn cbegin<T: QByteArray_cbegin>(&mut self, value: T) -> i32 {
    value.cbegin(self);
    return 1;
  }
}

pub trait QByteArray_cbegin {
  fn cbegin(self, this: &mut QByteArray) -> i32;
}

// proto: const char * QByteArray::cbegin();
impl<'a> /*trait*/ QByteArray_cbegin for () {
  fn cbegin(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray6cbeginEv()};
    unsafe {_ZNK10QByteArray6cbeginEv()};
    return 1;
  }
}

impl /*struct*/ QByteArray {
  pub fn fromRawData<T: QByteArray_fromRawData>(&mut self, value: T) -> i32 {
    value.fromRawData(self);
    return 1;
  }
}

pub trait QByteArray_fromRawData {
  fn fromRawData(self, this: &mut QByteArray) -> i32;
}

// proto: QByteArray QByteArray::fromRawData(const char * , int size);
impl<'a> /*trait*/ QByteArray_fromRawData for (&'a  String, i32) {
  fn fromRawData(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray11fromRawDataEPKci()};
    let arg0 = self.0.as_ptr()  as *const c_char;
    let arg1 = self.1  as c_int;
    unsafe {_ZN10QByteArray11fromRawDataEPKci(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QByteArray {
  pub fn contains<T: QByteArray_contains>(&mut self, value: T) -> i32 {
    value.contains(self);
    return 1;
  }
}

pub trait QByteArray_contains {
  fn contains(self, this: &mut QByteArray) -> i32;
}

// proto: bool QByteArray::contains(char c);
impl<'a> /*trait*/ QByteArray_contains for (i8) {
  fn contains(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray8containsEc()};
    let arg0 = self  as c_char;
    unsafe {_ZNK10QByteArray8containsEc(arg0)};
    return 1;
  }
}

// proto: void QByteArray::NewQByteArray(int size, char c);
impl<'a> /*trait*/ QByteArray_NewQByteArray for (i32, i8) {
  fn NewQByteArray(self) -> QByteArray {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArrayC1Eic()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_char;
    unsafe {_ZN10QByteArrayC1Eic(qthis, arg0, arg1)};
    let rsthis = QByteArray{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: int QByteArray::indexOf(const QByteArray & a, int from);
impl<'a> /*trait*/ QByteArray_indexOf for (&'a  QByteArray, i32) {
  fn indexOf(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray7indexOfERKS_i()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_int;
    unsafe {_ZNK10QByteArray7indexOfERKS_i(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QByteArray {
  pub fn toLong<T: QByteArray_toLong>(&mut self, value: T) -> i32 {
    value.toLong(self);
    return 1;
  }
}

pub trait QByteArray_toLong {
  fn toLong(self, this: &mut QByteArray) -> i32;
}

// proto: long QByteArray::toLong(bool * ok, int base);
impl<'a> /*trait*/ QByteArray_toLong for (&'a mut i8, i32) {
  fn toLong(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray6toLongEPbi()};
    let arg0 = self.0  as *mut int8_t;
    let arg1 = self.1  as c_int;
    unsafe {_ZNK10QByteArray6toLongEPbi(arg0, arg1)};
    return 1;
  }
}

// proto: int QByteArray::indexOf(char c, int from);
impl<'a> /*trait*/ QByteArray_indexOf for (i8, i32) {
  fn indexOf(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray7indexOfEci()};
    let arg0 = self.0  as c_char;
    let arg1 = self.1  as c_int;
    unsafe {_ZNK10QByteArray7indexOfEci(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QByteArray {
  pub fn data<T: QByteArray_data>(&mut self, value: T) -> i32 {
    value.data(self);
    return 1;
  }
}

pub trait QByteArray_data {
  fn data(self, this: &mut QByteArray) -> i32;
}

// proto: char * QByteArray::data();
impl<'a> /*trait*/ QByteArray_data for () {
  fn data(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray4dataEv()};
    unsafe {_ZN10QByteArray4dataEv()};
    return 1;
  }
}

impl /*struct*/ QByteArray {
  pub fn number<T: QByteArray_number>(&mut self, value: T) -> i32 {
    value.number(self);
    return 1;
  }
}

pub trait QByteArray_number {
  fn number(self, this: &mut QByteArray) -> i32;
}

// proto: QByteArray QByteArray::number(double , char f, int prec);
impl<'a> /*trait*/ QByteArray_number for (f64, i8, i32) {
  fn number(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray6numberEdci()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_char;
    let arg2 = self.2  as c_int;
    unsafe {_ZN10QByteArray6numberEdci(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QByteArray {
  pub fn capacity<T: QByteArray_capacity>(&mut self, value: T) -> i32 {
    value.capacity(self);
    return 1;
  }
}

pub trait QByteArray_capacity {
  fn capacity(self, this: &mut QByteArray) -> i32;
}

// proto: int QByteArray::capacity();
impl<'a> /*trait*/ QByteArray_capacity for () {
  fn capacity(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray8capacityEv()};
    unsafe {_ZNK10QByteArray8capacityEv()};
    return 1;
  }
}

// proto: int QByteArray::count();
impl<'a> /*trait*/ QByteArray_count for () {
  fn count(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray5countEv()};
    unsafe {_ZNK10QByteArray5countEv()};
    return 1;
  }
}

impl /*struct*/ QByteArray {
  pub fn fromBase64<T: QByteArray_fromBase64>(&mut self, value: T) -> i32 {
    value.fromBase64(self);
    return 1;
  }
}

pub trait QByteArray_fromBase64 {
  fn fromBase64(self, this: &mut QByteArray) -> i32;
}

// proto: QByteArray QByteArray::fromBase64(const QByteArray & base64);
impl<'a> /*trait*/ QByteArray_fromBase64 for (&'a  QByteArray) {
  fn fromBase64(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray10fromBase64ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN10QByteArray10fromBase64ERKS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QByteArray {
  pub fn left<T: QByteArray_left>(&mut self, value: T) -> i32 {
    value.left(self);
    return 1;
  }
}

pub trait QByteArray_left {
  fn left(self, this: &mut QByteArray) -> i32;
}

// proto: QByteArray QByteArray::left(int len);
impl<'a> /*trait*/ QByteArray_left for (i32) {
  fn left(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray4leftEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK10QByteArray4leftEi(arg0)};
    return 1;
  }
}

// proto: QByteArray & QByteArray::replace(char before, char after);
impl<'a> /*trait*/ QByteArray_replace for (i8, i8) {
  fn replace(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray7replaceEcc()};
    let arg0 = self.0  as c_char;
    let arg1 = self.1  as c_char;
    unsafe {_ZN10QByteArray7replaceEcc(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QByteArray {
  pub fn append<T: QByteArray_append>(&mut self, value: T) -> i32 {
    value.append(self);
    return 1;
  }
}

pub trait QByteArray_append {
  fn append(self, this: &mut QByteArray) -> i32;
}

// proto: QByteArray & QByteArray::append(char c);
impl<'a> /*trait*/ QByteArray_append for (i8) {
  fn append(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray6appendEc()};
    let arg0 = self  as c_char;
    unsafe {_ZN10QByteArray6appendEc(arg0)};
    return 1;
  }
}

impl /*struct*/ QByteArray {
  pub fn startsWith<T: QByteArray_startsWith>(&mut self, value: T) -> i32 {
    value.startsWith(self);
    return 1;
  }
}

pub trait QByteArray_startsWith {
  fn startsWith(self, this: &mut QByteArray) -> i32;
}

// proto: bool QByteArray::startsWith(const char * c);
impl<'a> /*trait*/ QByteArray_startsWith for (&'a  String) {
  fn startsWith(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray10startsWithEPKc()};
    let arg0 = self.as_ptr()  as *const c_char;
    unsafe {_ZNK10QByteArray10startsWithEPKc(arg0)};
    return 1;
  }
}

impl /*struct*/ QByteArray {
  pub fn remove<T: QByteArray_remove>(&mut self, value: T) -> i32 {
    value.remove(self);
    return 1;
  }
}

pub trait QByteArray_remove {
  fn remove(self, this: &mut QByteArray) -> i32;
}

// proto: QByteArray & QByteArray::remove(int index, int len);
impl<'a> /*trait*/ QByteArray_remove for (i32, i32) {
  fn remove(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray6removeEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZN10QByteArray6removeEii(arg0, arg1)};
    return 1;
  }
}

// proto: int QByteArray::lastIndexOf(char c, int from);
impl<'a> /*trait*/ QByteArray_lastIndexOf for (i8, i32) {
  fn lastIndexOf(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray11lastIndexOfEci()};
    let arg0 = self.0  as c_char;
    let arg1 = self.1  as c_int;
    unsafe {_ZNK10QByteArray11lastIndexOfEci(arg0, arg1)};
    return 1;
  }
}

// proto: bool QByteArray::startsWith(const QByteArray & a);
impl<'a> /*trait*/ QByteArray_startsWith for (&'a  QByteArray) {
  fn startsWith(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray10startsWithERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK10QByteArray10startsWithERKS_(arg0)};
    return 1;
  }
}

// proto: bool QByteArray::contains(const char * a);
impl<'a> /*trait*/ QByteArray_contains for (&'a  String) {
  fn contains(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray8containsEPKc()};
    let arg0 = self.as_ptr()  as *const c_char;
    unsafe {_ZNK10QByteArray8containsEPKc(arg0)};
    return 1;
  }
}

impl /*struct*/ QByteArray {
  pub fn endsWith<T: QByteArray_endsWith>(&mut self, value: T) -> i32 {
    value.endsWith(self);
    return 1;
  }
}

pub trait QByteArray_endsWith {
  fn endsWith(self, this: &mut QByteArray) -> i32;
}

// proto: bool QByteArray::endsWith(const char * c);
impl<'a> /*trait*/ QByteArray_endsWith for (&'a  String) {
  fn endsWith(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray8endsWithEPKc()};
    let arg0 = self.as_ptr()  as *const c_char;
    unsafe {_ZNK10QByteArray8endsWithEPKc(arg0)};
    return 1;
  }
}

impl /*struct*/ QByteArray {
  pub fn squeeze<T: QByteArray_squeeze>(&mut self, value: T) -> i32 {
    value.squeeze(self);
    return 1;
  }
}

pub trait QByteArray_squeeze {
  fn squeeze(self, this: &mut QByteArray) -> i32;
}

// proto: void QByteArray::squeeze();
impl<'a> /*trait*/ QByteArray_squeeze for () {
  fn squeeze(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray7squeezeEv()};
    unsafe {_ZN10QByteArray7squeezeEv()};
    return 1;
  }
}

// proto: void QByteArray::NewQByteArray(const char * , int size);
impl<'a> /*trait*/ QByteArray_NewQByteArray for (&'a  String, i32) {
  fn NewQByteArray(self) -> QByteArray {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArrayC1EPKci()};
    let arg0 = self.0.as_ptr()  as *const c_char;
    let arg1 = self.1  as c_int;
    unsafe {_ZN10QByteArrayC1EPKci(qthis, arg0, arg1)};
    let rsthis = QByteArray{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: int QByteArray::indexOf(const QString & s, int from);
impl<'a> /*trait*/ QByteArray_indexOf for (&'a  QString, i32) {
  fn indexOf(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray7indexOfERK7QStringi()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_int;
    unsafe {_ZNK10QByteArray7indexOfERK7QStringi(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QByteArray {
  pub fn detach<T: QByteArray_detach>(&mut self, value: T) -> i32 {
    value.detach(self);
    return 1;
  }
}

pub trait QByteArray_detach {
  fn detach(self, this: &mut QByteArray) -> i32;
}

// proto: void QByteArray::detach();
impl<'a> /*trait*/ QByteArray_detach for () {
  fn detach(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray6detachEv()};
    unsafe {_ZN10QByteArray6detachEv()};
    return 1;
  }
}

impl /*struct*/ QByteArray {
  pub fn repeated<T: QByteArray_repeated>(&mut self, value: T) -> i32 {
    value.repeated(self);
    return 1;
  }
}

pub trait QByteArray_repeated {
  fn repeated(self, this: &mut QByteArray) -> i32;
}

// proto: QByteArray QByteArray::repeated(int times);
impl<'a> /*trait*/ QByteArray_repeated for (i32) {
  fn repeated(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray8repeatedEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK10QByteArray8repeatedEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QByteArray {
  pub fn setNum<T: QByteArray_setNum>(&mut self, value: T) -> i32 {
    value.setNum(self);
    return 1;
  }
}

pub trait QByteArray_setNum {
  fn setNum(self, this: &mut QByteArray) -> i32;
}

// proto: QByteArray & QByteArray::setNum(float , char f, int prec);
impl<'a> /*trait*/ QByteArray_setNum for (f32, i8, i32) {
  fn setNum(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray6setNumEfci()};
    let arg0 = self.0  as c_float;
    let arg1 = self.1  as c_char;
    let arg2 = self.2  as c_int;
    unsafe {_ZN10QByteArray6setNumEfci(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QByteArray {
  pub fn toShort<T: QByteArray_toShort>(&mut self, value: T) -> i32 {
    value.toShort(self);
    return 1;
  }
}

pub trait QByteArray_toShort {
  fn toShort(self, this: &mut QByteArray) -> i32;
}

// proto: short QByteArray::toShort(bool * ok, int base);
impl<'a> /*trait*/ QByteArray_toShort for (&'a mut i8, i32) {
  fn toShort(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray7toShortEPbi()};
    let arg0 = self.0  as *mut int8_t;
    let arg1 = self.1  as c_int;
    unsafe {_ZNK10QByteArray7toShortEPbi(arg0, arg1)};
    return 1;
  }
}

// proto: QByteArray & QByteArray::prepend(char c);
impl<'a> /*trait*/ QByteArray_prepend for (i8) {
  fn prepend(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray7prependEc()};
    let arg0 = self  as c_char;
    unsafe {_ZN10QByteArray7prependEc(arg0)};
    return 1;
  }
}

impl /*struct*/ QByteArray {
  pub fn toInt<T: QByteArray_toInt>(&mut self, value: T) -> i32 {
    value.toInt(self);
    return 1;
  }
}

pub trait QByteArray_toInt {
  fn toInt(self, this: &mut QByteArray) -> i32;
}

// proto: int QByteArray::toInt(bool * ok, int base);
impl<'a> /*trait*/ QByteArray_toInt for (&'a mut i8, i32) {
  fn toInt(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray5toIntEPbi()};
    let arg0 = self.0  as *mut int8_t;
    let arg1 = self.1  as c_int;
    unsafe {_ZNK10QByteArray5toIntEPbi(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QByteArray {
  pub fn constBegin<T: QByteArray_constBegin>(&mut self, value: T) -> i32 {
    value.constBegin(self);
    return 1;
  }
}

pub trait QByteArray_constBegin {
  fn constBegin(self, this: &mut QByteArray) -> i32;
}

// proto: const char * QByteArray::constBegin();
impl<'a> /*trait*/ QByteArray_constBegin for () {
  fn constBegin(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray10constBeginEv()};
    unsafe {_ZNK10QByteArray10constBeginEv()};
    return 1;
  }
}

impl /*struct*/ QByteArray {
  pub fn push_back<T: QByteArray_push_back>(&mut self, value: T) -> i32 {
    value.push_back(self);
    return 1;
  }
}

pub trait QByteArray_push_back {
  fn push_back(self, this: &mut QByteArray) -> i32;
}

// proto: void QByteArray::push_back(char c);
impl<'a> /*trait*/ QByteArray_push_back for (i8) {
  fn push_back(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray9push_backEc()};
    let arg0 = self  as c_char;
    unsafe {_ZN10QByteArray9push_backEc(arg0)};
    return 1;
  }
}

impl /*struct*/ QByteArray {
  pub fn isSharedWith<T: QByteArray_isSharedWith>(&mut self, value: T) -> i32 {
    value.isSharedWith(self);
    return 1;
  }
}

pub trait QByteArray_isSharedWith {
  fn isSharedWith(self, this: &mut QByteArray) -> i32;
}

// proto: bool QByteArray::isSharedWith(const QByteArray & other);
impl<'a> /*trait*/ QByteArray_isSharedWith for (&'a  QByteArray) {
  fn isSharedWith(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray12isSharedWithERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK10QByteArray12isSharedWithERKS_(arg0)};
    return 1;
  }
}

// proto: QByteArray QByteArray::fromBase64(const QByteArray & base64, Base64Options options);
impl<'a> /*trait*/ QByteArray_fromBase64 for (&'a  QByteArray, i32) {
  fn fromBase64(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray10fromBase64ERKS_6QFlagsINS_12Base64OptionEE()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_int;
    unsafe {_ZN10QByteArray10fromBase64ERKS_6QFlagsINS_12Base64OptionEE(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QByteArray {
  pub fn size<T: QByteArray_size>(&mut self, value: T) -> i32 {
    value.size(self);
    return 1;
  }
}

pub trait QByteArray_size {
  fn size(self, this: &mut QByteArray) -> i32;
}

// proto: int QByteArray::size();
impl<'a> /*trait*/ QByteArray_size for () {
  fn size(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray4sizeEv()};
    unsafe {_ZNK10QByteArray4sizeEv()};
    return 1;
  }
}

// proto: bool QByteArray::endsWith(char c);
impl<'a> /*trait*/ QByteArray_endsWith for (i8) {
  fn endsWith(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray8endsWithEc()};
    let arg0 = self  as c_char;
    unsafe {_ZNK10QByteArray8endsWithEc(arg0)};
    return 1;
  }
}

// proto: QByteArray QByteArray::number(uint , int base);
impl<'a> /*trait*/ QByteArray_number for (u32, i32) {
  fn number(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray6numberEji()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_int;
    unsafe {_ZN10QByteArray6numberEji(arg0, arg1)};
    return 1;
  }
}

// proto: void QByteArray::push_front(char c);
impl<'a> /*trait*/ QByteArray_push_front for (i8) {
  fn push_front(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray10push_frontEc()};
    let arg0 = self  as c_char;
    unsafe {_ZN10QByteArray10push_frontEc(arg0)};
    return 1;
  }
}

impl /*struct*/ QByteArray {
  pub fn leftJustified<T: QByteArray_leftJustified>(&mut self, value: T) -> i32 {
    value.leftJustified(self);
    return 1;
  }
}

pub trait QByteArray_leftJustified {
  fn leftJustified(self, this: &mut QByteArray) -> i32;
}

// proto: QByteArray QByteArray::leftJustified(int width, char fill, bool truncate);
impl<'a> /*trait*/ QByteArray_leftJustified for (i32, i8, i8) {
  fn leftJustified(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray13leftJustifiedEicb()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_char;
    let arg2 = self.2  as int8_t;
    unsafe {_ZNK10QByteArray13leftJustifiedEicb(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QByteArray {
  pub fn begin<T: QByteArray_begin>(&mut self, value: T) -> i32 {
    value.begin(self);
    return 1;
  }
}

pub trait QByteArray_begin {
  fn begin(self, this: &mut QByteArray) -> i32;
}

// proto: char * QByteArray::begin();
impl<'a> /*trait*/ QByteArray_begin for () {
  fn begin(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray5beginEv()};
    unsafe {_ZN10QByteArray5beginEv()};
    return 1;
  }
}

// proto: QByteArray QByteArray::number(qulonglong , int base);
impl<'a> /*trait*/ QByteArray_number for (u64, i32) {
  fn number(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray6numberEyi()};
    let arg0 = self.0  as uint64_t;
    let arg1 = self.1  as c_int;
    unsafe {_ZN10QByteArray6numberEyi(arg0, arg1)};
    return 1;
  }
}

// proto: int QByteArray::count(char c);
impl<'a> /*trait*/ QByteArray_count for (i8) {
  fn count(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray5countEc()};
    let arg0 = self  as c_char;
    unsafe {_ZNK10QByteArray5countEc(arg0)};
    return 1;
  }
}

impl /*struct*/ QByteArray {
  pub fn toDouble<T: QByteArray_toDouble>(&mut self, value: T) -> i32 {
    value.toDouble(self);
    return 1;
  }
}

pub trait QByteArray_toDouble {
  fn toDouble(self, this: &mut QByteArray) -> i32;
}

// proto: double QByteArray::toDouble(bool * ok);
impl<'a> /*trait*/ QByteArray_toDouble for (&'a mut i8) {
  fn toDouble(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray8toDoubleEPb()};
    let arg0 = self  as *mut int8_t;
    unsafe {_ZNK10QByteArray8toDoubleEPb(arg0)};
    return 1;
  }
}

// proto: QByteArray & QByteArray::replace(int index, int len, const QByteArray & s);
impl<'a> /*trait*/ QByteArray_replace for (i32, i32, &'a  QByteArray) {
  fn replace(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray7replaceEiiRKS_()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *const c_void;
    unsafe {_ZN10QByteArray7replaceEiiRKS_(arg0, arg1, arg2)};
    return 1;
  }
}

// proto: QByteArray & QByteArray::setNum(short , int base);
impl<'a> /*trait*/ QByteArray_setNum for (i16, i32) {
  fn setNum(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray6setNumEsi()};
    let arg0 = self.0  as c_short;
    let arg1 = self.1  as c_int;
    unsafe {_ZN10QByteArray6setNumEsi(arg0, arg1)};
    return 1;
  }
}

// proto: QByteArray & QByteArray::prepend(const QByteArray & a);
impl<'a> /*trait*/ QByteArray_prepend for (&'a  QByteArray) {
  fn prepend(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray7prependERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN10QByteArray7prependERKS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QByteArray {
  pub fn toULongLong<T: QByteArray_toULongLong>(&mut self, value: T) -> i32 {
    value.toULongLong(self);
    return 1;
  }
}

pub trait QByteArray_toULongLong {
  fn toULongLong(self, this: &mut QByteArray) -> i32;
}

// proto: quint64 QByteArray::toULongLong(bool * ok, int base);
impl<'a> /*trait*/ QByteArray_toULongLong for (&'a mut i8, i32) {
  fn toULongLong(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray11toULongLongEPbi()};
    let arg0 = self.0  as *mut int8_t;
    let arg1 = self.1  as c_int;
    unsafe {_ZNK10QByteArray11toULongLongEPbi(arg0, arg1)};
    return 1;
  }
}

// proto: QByteArray & QByteArray::replace(char c, const QString & after);
impl<'a> /*trait*/ QByteArray_replace for (i8, &'a  QString) {
  fn replace(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray7replaceEcRK7QString()};
    let arg0 = self.0  as c_char;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN10QByteArray7replaceEcRK7QString(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QByteArray {
  pub fn fromPercentEncoding<T: QByteArray_fromPercentEncoding>(&mut self, value: T) -> i32 {
    value.fromPercentEncoding(self);
    return 1;
  }
}

pub trait QByteArray_fromPercentEncoding {
  fn fromPercentEncoding(self, this: &mut QByteArray) -> i32;
}

// proto: QByteArray QByteArray::fromPercentEncoding(const QByteArray & pctEncoded, char percent);
impl<'a> /*trait*/ QByteArray_fromPercentEncoding for (&'a  QByteArray, i8) {
  fn fromPercentEncoding(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray19fromPercentEncodingERKS_c()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_char;
    unsafe {_ZN10QByteArray19fromPercentEncodingERKS_c(arg0, arg1)};
    return 1;
  }
}

// proto: void QByteArray::push_front(const char * c);
impl<'a> /*trait*/ QByteArray_push_front for (&'a  String) {
  fn push_front(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray10push_frontEPKc()};
    let arg0 = self.as_ptr()  as *const c_char;
    unsafe {_ZN10QByteArray10push_frontEPKc(arg0)};
    return 1;
  }
}

impl /*struct*/ QByteArray {
  pub fn clear<T: QByteArray_clear>(&mut self, value: T) -> i32 {
    value.clear(self);
    return 1;
  }
}

pub trait QByteArray_clear {
  fn clear(self, this: &mut QByteArray) -> i32;
}

// proto: void QByteArray::clear();
impl<'a> /*trait*/ QByteArray_clear for () {
  fn clear(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray5clearEv()};
    unsafe {_ZN10QByteArray5clearEv()};
    return 1;
  }
}

impl /*struct*/ QByteArray {
  pub fn toLongLong<T: QByteArray_toLongLong>(&mut self, value: T) -> i32 {
    value.toLongLong(self);
    return 1;
  }
}

pub trait QByteArray_toLongLong {
  fn toLongLong(self, this: &mut QByteArray) -> i32;
}

// proto: qint64 QByteArray::toLongLong(bool * ok, int base);
impl<'a> /*trait*/ QByteArray_toLongLong for (&'a mut i8, i32) {
  fn toLongLong(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray10toLongLongEPbi()};
    let arg0 = self.0  as *mut int8_t;
    let arg1 = self.1  as c_int;
    unsafe {_ZNK10QByteArray10toLongLongEPbi(arg0, arg1)};
    return 1;
  }
}

// proto: QByteArray & QByteArray::prepend(const char * s, int len);
impl<'a> /*trait*/ QByteArray_prepend for (&'a  String, i32) {
  fn prepend(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray7prependEPKci()};
    let arg0 = self.0.as_ptr()  as *const c_char;
    let arg1 = self.1  as c_int;
    unsafe {_ZN10QByteArray7prependEPKci(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QByteArray {
  pub fn constData<T: QByteArray_constData>(&mut self, value: T) -> i32 {
    value.constData(self);
    return 1;
  }
}

pub trait QByteArray_constData {
  fn constData(self, this: &mut QByteArray) -> i32;
}

// proto: const char * QByteArray::constData();
impl<'a> /*trait*/ QByteArray_constData for () {
  fn constData(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray9constDataEv()};
    unsafe {_ZNK10QByteArray9constDataEv()};
    return 1;
  }
}

// proto: void QByteArray::NewQByteArray(const QByteArray & );
impl<'a> /*trait*/ QByteArray_NewQByteArray for (&'a  QByteArray) {
  fn NewQByteArray(self) -> QByteArray {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArrayC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN10QByteArrayC1ERKS_(qthis, arg0)};
    let rsthis = QByteArray{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QByteArray {
  pub fn length<T: QByteArray_length>(&mut self, value: T) -> i32 {
    value.length(self);
    return 1;
  }
}

pub trait QByteArray_length {
  fn length(self, this: &mut QByteArray) -> i32;
}

// proto: int QByteArray::length();
impl<'a> /*trait*/ QByteArray_length for () {
  fn length(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray6lengthEv()};
    unsafe {_ZNK10QByteArray6lengthEv()};
    return 1;
  }
}

// proto: QByteArray QByteArray::number(int , int base);
impl<'a> /*trait*/ QByteArray_number for (i32, i32) {
  fn number(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray6numberEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZN10QByteArray6numberEii(arg0, arg1)};
    return 1;
  }
}

// proto: bool QByteArray::startsWith(char c);
impl<'a> /*trait*/ QByteArray_startsWith for (i8) {
  fn startsWith(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray10startsWithEc()};
    let arg0 = self  as c_char;
    unsafe {_ZNK10QByteArray10startsWithEc(arg0)};
    return 1;
  }
}

// proto: QByteArray & QByteArray::setNum(double , char f, int prec);
impl<'a> /*trait*/ QByteArray_setNum for (f64, i8, i32) {
  fn setNum(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray6setNumEdci()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_char;
    let arg2 = self.2  as c_int;
    unsafe {_ZN10QByteArray6setNumEdci(arg0, arg1, arg2)};
    return 1;
  }
}

// proto: QByteArray QByteArray::number(qlonglong , int base);
impl<'a> /*trait*/ QByteArray_number for (i64, i32) {
  fn number(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray6numberExi()};
    let arg0 = self.0  as c_longlong;
    let arg1 = self.1  as c_int;
    unsafe {_ZN10QByteArray6numberExi(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QByteArray {
  pub fn at<T: QByteArray_at>(&mut self, value: T) -> i32 {
    value.at(self);
    return 1;
  }
}

pub trait QByteArray_at {
  fn at(self, this: &mut QByteArray) -> i32;
}

// proto: char QByteArray::at(int i);
impl<'a> /*trait*/ QByteArray_at for (i32) {
  fn at(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray2atEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK10QByteArray2atEi(arg0)};
    return 1;
  }
}

// proto: QByteArray & QByteArray::setNum(ushort , int base);
impl<'a> /*trait*/ QByteArray_setNum for (u16, i32) {
  fn setNum(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray6setNumEti()};
    let arg0 = self.0  as c_ushort;
    let arg1 = self.1  as c_int;
    unsafe {_ZN10QByteArray6setNumEti(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QByteArray {
  pub fn swap<T: QByteArray_swap>(&mut self, value: T) -> i32 {
    value.swap(self);
    return 1;
  }
}

pub trait QByteArray_swap {
  fn swap(self, this: &mut QByteArray) -> i32;
}

// proto: void QByteArray::swap(QByteArray & other);
impl<'a> /*trait*/ QByteArray_swap for (&'a mut QByteArray) {
  fn swap(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN10QByteArray4swapERS_(arg0)};
    return 1;
  }
}

// proto: QByteArray & QByteArray::replace(const QString & before, const char * after);
impl<'a> /*trait*/ QByteArray_replace for (&'a  QString, &'a  String) {
  fn replace(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray7replaceERK7QStringPKc()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.as_ptr()  as *const c_char;
    unsafe {_ZN10QByteArray7replaceERK7QStringPKc(arg0, arg1)};
    return 1;
  }
}

// proto: QByteArray & QByteArray::append(const QByteArray & a);
impl<'a> /*trait*/ QByteArray_append for (&'a  QByteArray) {
  fn append(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray6appendERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN10QByteArray6appendERKS_(arg0)};
    return 1;
  }
}

// proto: bool QByteArray::endsWith(const QByteArray & a);
impl<'a> /*trait*/ QByteArray_endsWith for (&'a  QByteArray) {
  fn endsWith(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray8endsWithERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK10QByteArray8endsWithERKS_(arg0)};
    return 1;
  }
}

// proto: int QByteArray::count(const char * a);
impl<'a> /*trait*/ QByteArray_count for (&'a  String) {
  fn count(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray5countEPKc()};
    let arg0 = self.as_ptr()  as *const c_char;
    unsafe {_ZNK10QByteArray5countEPKc(arg0)};
    return 1;
  }
}

// proto: QByteArray & QByteArray::replace(const char * before, int bsize, const char * after, int asize);
impl<'a> /*trait*/ QByteArray_replace for (&'a  String, i32, &'a  String, i32) {
  fn replace(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray7replaceEPKciS1_i()};
    let arg0 = self.0.as_ptr()  as *const c_char;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.as_ptr()  as *const c_char;
    let arg3 = self.3  as c_int;
    unsafe {_ZN10QByteArray7replaceEPKciS1_i(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QByteArray {
  pub fn split<T: QByteArray_split>(&mut self, value: T) -> i32 {
    value.split(self);
    return 1;
  }
}

pub trait QByteArray_split {
  fn split(self, this: &mut QByteArray) -> i32;
}

// proto: QList<QByteArray> QByteArray::split(char sep);
impl<'a> /*trait*/ QByteArray_split for (i8) {
  fn split(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray5splitEc()};
    let arg0 = self  as c_char;
    unsafe {_ZNK10QByteArray5splitEc(arg0)};
    return 1;
  }
}

// proto: QByteArray & QByteArray::setNum(qlonglong , int base);
impl<'a> /*trait*/ QByteArray_setNum for (i64, i32) {
  fn setNum(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray6setNumExi()};
    let arg0 = self.0  as c_longlong;
    let arg1 = self.1  as c_int;
    unsafe {_ZN10QByteArray6setNumExi(arg0, arg1)};
    return 1;
  }
}

// proto: QByteArray & QByteArray::replace(char before, const char * after);
impl<'a> /*trait*/ QByteArray_replace for (i8, &'a  String) {
  fn replace(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray7replaceEcPKc()};
    let arg0 = self.0  as c_char;
    let arg1 = self.1.as_ptr()  as *const c_char;
    unsafe {_ZN10QByteArray7replaceEcPKc(arg0, arg1)};
    return 1;
  }
}

// proto: QByteArray & QByteArray::append(const char * s);
impl<'a> /*trait*/ QByteArray_append for (&'a  String) {
  fn append(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray6appendEPKc()};
    let arg0 = self.as_ptr()  as *const c_char;
    unsafe {_ZN10QByteArray6appendEPKc(arg0)};
    return 1;
  }
}

impl /*struct*/ QByteArray {
  pub fn right<T: QByteArray_right>(&mut self, value: T) -> i32 {
    value.right(self);
    return 1;
  }
}

pub trait QByteArray_right {
  fn right(self, this: &mut QByteArray) -> i32;
}

// proto: QByteArray QByteArray::right(int len);
impl<'a> /*trait*/ QByteArray_right for (i32) {
  fn right(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray5rightEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK10QByteArray5rightEi(arg0)};
    return 1;
  }
}

// proto: QByteArray & QByteArray::append(const QString & s);
impl<'a> /*trait*/ QByteArray_append for (&'a  QString) {
  fn append(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray6appendERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN10QByteArray6appendERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QByteArray {
  pub fn chop<T: QByteArray_chop>(&mut self, value: T) -> i32 {
    value.chop(self);
    return 1;
  }
}

pub trait QByteArray_chop {
  fn chop(self, this: &mut QByteArray) -> i32;
}

// proto: void QByteArray::chop(int n);
impl<'a> /*trait*/ QByteArray_chop for (i32) {
  fn chop(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray4chopEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN10QByteArray4chopEi(arg0)};
    return 1;
  }
}

// proto: int QByteArray::lastIndexOf(const char * c, int from);
impl<'a> /*trait*/ QByteArray_lastIndexOf for (&'a  String, i32) {
  fn lastIndexOf(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray11lastIndexOfEPKci()};
    let arg0 = self.0.as_ptr()  as *const c_char;
    let arg1 = self.1  as c_int;
    unsafe {_ZNK10QByteArray11lastIndexOfEPKci(arg0, arg1)};
    return 1;
  }
}

// proto: QByteArray & QByteArray::replace(int index, int len, const char * s);
impl<'a> /*trait*/ QByteArray_replace for (i32, i32, &'a  String) {
  fn replace(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray7replaceEiiPKc()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.as_ptr()  as *const c_char;
    unsafe {_ZN10QByteArray7replaceEiiPKc(arg0, arg1, arg2)};
    return 1;
  }
}

// proto: void QByteArray::push_back(const QByteArray & a);
impl<'a> /*trait*/ QByteArray_push_back for (&'a  QByteArray) {
  fn push_back(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray9push_backERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN10QByteArray9push_backERKS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QByteArray {
  pub fn toPercentEncoding<T: QByteArray_toPercentEncoding>(&mut self, value: T) -> i32 {
    value.toPercentEncoding(self);
    return 1;
  }
}

pub trait QByteArray_toPercentEncoding {
  fn toPercentEncoding(self, this: &mut QByteArray) -> i32;
}

// proto: QByteArray QByteArray::toPercentEncoding(const QByteArray & exclude, const QByteArray & include, char percent);
impl<'a> /*trait*/ QByteArray_toPercentEncoding for (&'a  QByteArray, &'a  QByteArray, i8) {
  fn toPercentEncoding(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray17toPercentEncodingERKS_S1_c()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2  as c_char;
    unsafe {_ZNK10QByteArray17toPercentEncodingERKS_S1_c(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QByteArray {
  pub fn isDetached<T: QByteArray_isDetached>(&mut self, value: T) -> i32 {
    value.isDetached(self);
    return 1;
  }
}

pub trait QByteArray_isDetached {
  fn isDetached(self, this: &mut QByteArray) -> i32;
}

// proto: bool QByteArray::isDetached();
impl<'a> /*trait*/ QByteArray_isDetached for () {
  fn isDetached(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray10isDetachedEv()};
    unsafe {_ZNK10QByteArray10isDetachedEv()};
    return 1;
  }
}

// proto: QByteArray & QByteArray::append(const char * s, int len);
impl<'a> /*trait*/ QByteArray_append for (&'a  String, i32) {
  fn append(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray6appendEPKci()};
    let arg0 = self.0.as_ptr()  as *const c_char;
    let arg1 = self.1  as c_int;
    unsafe {_ZN10QByteArray6appendEPKci(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QByteArray {
  pub fn constEnd<T: QByteArray_constEnd>(&mut self, value: T) -> i32 {
    value.constEnd(self);
    return 1;
  }
}

pub trait QByteArray_constEnd {
  fn constEnd(self, this: &mut QByteArray) -> i32;
}

// proto: const char * QByteArray::constEnd();
impl<'a> /*trait*/ QByteArray_constEnd for () {
  fn constEnd(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray8constEndEv()};
    unsafe {_ZNK10QByteArray8constEndEv()};
    return 1;
  }
}

// proto: QByteArray & QByteArray::replace(const QByteArray & before, const char * after);
impl<'a> /*trait*/ QByteArray_replace for (&'a  QByteArray, &'a  String) {
  fn replace(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray7replaceERKS_PKc()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.as_ptr()  as *const c_char;
    unsafe {_ZN10QByteArray7replaceERKS_PKc(arg0, arg1)};
    return 1;
  }
}

// proto: QByteArray & QByteArray::setNum(qulonglong , int base);
impl<'a> /*trait*/ QByteArray_setNum for (u64, i32) {
  fn setNum(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray6setNumEyi()};
    let arg0 = self.0  as uint64_t;
    let arg1 = self.1  as c_int;
    unsafe {_ZN10QByteArray6setNumEyi(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QByteArray {
  pub fn setRawData<T: QByteArray_setRawData>(&mut self, value: T) -> i32 {
    value.setRawData(self);
    return 1;
  }
}

pub trait QByteArray_setRawData {
  fn setRawData(self, this: &mut QByteArray) -> i32;
}

// proto: QByteArray & QByteArray::setRawData(const char * a, uint n);
impl<'a> /*trait*/ QByteArray_setRawData for (&'a  String, u32) {
  fn setRawData(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray10setRawDataEPKcj()};
    let arg0 = self.0.as_ptr()  as *const c_char;
    let arg1 = self.1  as c_uint;
    unsafe {_ZN10QByteArray10setRawDataEPKcj(arg0, arg1)};
    return 1;
  }
}

// proto: QByteArray & QByteArray::replace(const QString & before, const QByteArray & after);
impl<'a> /*trait*/ QByteArray_replace for (&'a  QString, &'a  QByteArray) {
  fn replace(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray7replaceERK7QStringRKS_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN10QByteArray7replaceERK7QStringRKS_(arg0, arg1)};
    return 1;
  }
}

// proto: QByteArray & QByteArray::setNum(uint , int base);
impl<'a> /*trait*/ QByteArray_setNum for (u32, i32) {
  fn setNum(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray6setNumEji()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_int;
    unsafe {_ZN10QByteArray6setNumEji(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QByteArray {
  pub fn mid<T: QByteArray_mid>(&mut self, value: T) -> i32 {
    value.mid(self);
    return 1;
  }
}

pub trait QByteArray_mid {
  fn mid(self, this: &mut QByteArray) -> i32;
}

// proto: QByteArray QByteArray::mid(int index, int len);
impl<'a> /*trait*/ QByteArray_mid for (i32, i32) {
  fn mid(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray3midEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZNK10QByteArray3midEii(arg0, arg1)};
    return 1;
  }
}

// proto: QByteArray & QByteArray::setNum(int , int base);
impl<'a> /*trait*/ QByteArray_setNum for (i32, i32) {
  fn setNum(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray6setNumEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZN10QByteArray6setNumEii(arg0, arg1)};
    return 1;
  }
}

// proto: QByteArray & QByteArray::insert(int i, const QByteArray & a);
impl<'a> /*trait*/ QByteArray_insert for (i32, &'a  QByteArray) {
  fn insert(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray6insertEiRKS_()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN10QByteArray6insertEiRKS_(arg0, arg1)};
    return 1;
  }
}

// proto: QByteArray & QByteArray::insert(int i, const char * s);
impl<'a> /*trait*/ QByteArray_insert for (i32, &'a  String) {
  fn insert(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray6insertEiPKc()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.as_ptr()  as *const c_char;
    unsafe {_ZN10QByteArray6insertEiPKc(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QByteArray {
  pub fn fill<T: QByteArray_fill>(&mut self, value: T) -> i32 {
    value.fill(self);
    return 1;
  }
}

pub trait QByteArray_fill {
  fn fill(self, this: &mut QByteArray) -> i32;
}

// proto: QByteArray & QByteArray::fill(char c, int size);
impl<'a> /*trait*/ QByteArray_fill for (i8, i32) {
  fn fill(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray4fillEci()};
    let arg0 = self.0  as c_char;
    let arg1 = self.1  as c_int;
    unsafe {_ZN10QByteArray4fillEci(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QByteArray {
  pub fn toUShort<T: QByteArray_toUShort>(&mut self, value: T) -> i32 {
    value.toUShort(self);
    return 1;
  }
}

pub trait QByteArray_toUShort {
  fn toUShort(self, this: &mut QByteArray) -> i32;
}

// proto: unsigned short QByteArray::toUShort(bool * ok, int base);
impl<'a> /*trait*/ QByteArray_toUShort for (&'a mut i8, i32) {
  fn toUShort(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray8toUShortEPbi()};
    let arg0 = self.0  as *mut int8_t;
    let arg1 = self.1  as c_int;
    unsafe {_ZNK10QByteArray8toUShortEPbi(arg0, arg1)};
    return 1;
  }
}

// proto: void QByteArray::push_back(const char * c);
impl<'a> /*trait*/ QByteArray_push_back for (&'a  String) {
  fn push_back(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QByteArray9push_backEPKc()};
    let arg0 = self.as_ptr()  as *const c_char;
    unsafe {_ZN10QByteArray9push_backEPKc(arg0)};
    return 1;
  }
}

impl /*struct*/ QByteArray {
  pub fn rightJustified<T: QByteArray_rightJustified>(&mut self, value: T) -> i32 {
    value.rightJustified(self);
    return 1;
  }
}

pub trait QByteArray_rightJustified {
  fn rightJustified(self, this: &mut QByteArray) -> i32;
}

// proto: QByteArray QByteArray::rightJustified(int width, char fill, bool truncate);
impl<'a> /*trait*/ QByteArray_rightJustified for (i32, i8, i8) {
  fn rightJustified(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray14rightJustifiedEicb()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_char;
    let arg2 = self.2  as int8_t;
    unsafe {_ZNK10QByteArray14rightJustifiedEicb(arg0, arg1, arg2)};
    return 1;
  }
}

// proto: bool QByteArray::contains(const QByteArray & a);
impl<'a> /*trait*/ QByteArray_contains for (&'a  QByteArray) {
  fn contains(self, this: &mut QByteArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QByteArray8containsERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK10QByteArray8containsERKS_(arg0)};
    return 1;
  }
}

