

// mod ::core::QLocale
// package qtcore
// /usr/include/qt/QtCore/qlocale.h
// #include <qlocale.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 1
*/
// import "C"
// import "unsafe"
// import "reflect"
// import "fmt"
// import "log"
// import "github.com/kitech/qt.go/qtrt"
use std::default::Default;
use std::ops::Deref;
use super::super::qtrt;
use super::*;
//  ext block end

//  body block begin



/*

*/
#[derive(Default)] // class sizeof(QLocale)=8
pub struct QLocale {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QLocale_ITF interface {
//    QLocale_PTR() *QLocale
//}
//func (ptr *QLocale) QLocale_PTR() *QLocale { return ptr }

impl /*struct*/ QLocale {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QLocale {
    return QLocale{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QLocale {
//  type Target = QLocaleBASE;
//
//  fn deref(&self) -> &QLocaleBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QLocaleBASE> for QLocale {
//  fn as_ref(& self) -> & QLocaleBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qlocale.h:929
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QLocale()

/*
Constructs a QLocale object initialized with the default locale. If no default locale was set using setDefault(), this locale will be the same as the one returned by system().

See also setDefault().
*/
// QLocale() ctx.fn_proto_cpp
impl /*struct*/ QLocale {
  pub fn QLocale_0<T: QLocale_QLocale_0>(value: T) -> QLocale {
    let rsthis = value.QLocale_0();
    return rsthis;
    // return 1;
  }
}

pub trait QLocale_QLocale_0 {
  fn QLocale_0(self) -> QLocale;
}
// QLocale() ctx.fn_proto_cpp
impl<'a> /*trait*/ QLocale_QLocale_0 for () {
  fn QLocale_0(self) -> QLocale {
    // unsafe{_ZN7QLocaleC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN7QLocaleC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QLocale{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qlocale.h:930
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QLocale(const QString &)

/*
Constructs a QLocale object initialized with the default locale. If no default locale was set using setDefault(), this locale will be the same as the one returned by system().

See also setDefault().
*/
// QLocale(const QString &) ctx.fn_proto_cpp
impl /*struct*/ QLocale {
  pub fn QLocale_1<T: QLocale_QLocale_1>(value: T) -> QLocale {
    let rsthis = value.QLocale_1();
    return rsthis;
    // return 1;
  }
}

pub trait QLocale_QLocale_1 {
  fn QLocale_1(self) -> QLocale;
}
// QLocale(const QString &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QLocale_QLocale_1 for (usize) {
  fn QLocale_1(self) -> QLocale {
    // unsafe{_ZN7QLocaleC2ERK7QString()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN7QLocaleC2ERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QLocale{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qlocale.h:931
// index:2
// Public Visibility=Default Availability=Available
// [-2] void QLocale(QLocale::Language, QLocale::Country)

/*
Constructs a QLocale object initialized with the default locale. If no default locale was set using setDefault(), this locale will be the same as the one returned by system().

See also setDefault().
*/
// QLocale(QLocale::Language, QLocale::Country) ctx.fn_proto_cpp
impl /*struct*/ QLocale {
  pub fn QLocale_2<T: QLocale_QLocale_2>(value: T) -> QLocale {
    let rsthis = value.QLocale_2();
    return rsthis;
    // return 1;
  }
}

pub trait QLocale_QLocale_2 {
  fn QLocale_2(self) -> QLocale;
}
// QLocale(QLocale::Language, QLocale::Country) ctx.fn_proto_cpp
impl<'a> /*trait*/ QLocale_QLocale_2 for (i32,i32) {
  fn QLocale_2(self) -> QLocale {
    // unsafe{_ZN7QLocaleC2ENS_8LanguageENS_7CountryE()};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN7QLocaleC2ENS_8LanguageENS_7CountryE", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QLocale{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qlocale.h:932
// index:3
// Public Visibility=Default Availability=Available
// [-2] void QLocale(QLocale::Language, QLocale::Script, QLocale::Country)

/*
Constructs a QLocale object initialized with the default locale. If no default locale was set using setDefault(), this locale will be the same as the one returned by system().

See also setDefault().
*/
// QLocale(QLocale::Language, QLocale::Script, QLocale::Country) ctx.fn_proto_cpp
impl /*struct*/ QLocale {
  pub fn QLocale_3<T: QLocale_QLocale_3>(value: T) -> QLocale {
    let rsthis = value.QLocale_3();
    return rsthis;
    // return 1;
  }
}

pub trait QLocale_QLocale_3 {
  fn QLocale_3(self) -> QLocale;
}
// QLocale(QLocale::Language, QLocale::Script, QLocale::Country) ctx.fn_proto_cpp
impl<'a> /*trait*/ QLocale_QLocale_3 for (i32,i32,i32) {
  fn QLocale_3(self) -> QLocale {
    // unsafe{_ZN7QLocaleC2ENS_8LanguageENS_6ScriptENS_7CountryE()};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN7QLocaleC2ENS_8LanguageENS_6ScriptENS_7CountryE", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QLocale{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qlocale.h:935
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QLocale & operator=(QLocale &&)

/*

*/
impl /*struct*/ QLocale {
  pub fn operator_equal_0<RetType, T: QLocale_operator_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_0(self);
    // return 1;
  }
}
pub trait QLocale_operator_equal_0<RetType> {
  fn operator_equal_0(self , rsthis: & QLocale) -> RetType;
}
impl<'a> /*trait*/ QLocale_operator_equal_0<usize> for (usize) {
  fn operator_equal_0(self , rsthis: & QLocale) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QLocaleaSEOS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlocale.h:937
// index:1
// Public Visibility=Default Availability=Available
// [8] QLocale & operator=(const QLocale &)

/*

*/
impl /*struct*/ QLocale {
  pub fn operator_equal_1<RetType, T: QLocale_operator_equal_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_1(self);
    // return 1;
  }
}
pub trait QLocale_operator_equal_1<RetType> {
  fn operator_equal_1(self , rsthis: & QLocale) -> RetType;
}
impl<'a> /*trait*/ QLocale_operator_equal_1<usize> for (usize) {
  fn operator_equal_1(self , rsthis: & QLocale) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QLocaleaSERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlocale.h:938
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ~QLocale()

/*

*/
pub fn DeleteQLocale(this :*mut QLocale) {
    // let rv = qtrt::InvokeQtFunc6("_ZN7QLocaleD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 8)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qlocale.h:940
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void swap(QLocale &)

/*
Swaps locale other with this locale. This operation is very fast and never fails.

This function was introduced in  Qt 5.6.
*/
impl /*struct*/ QLocale {
  pub fn swap_0<RetType, T: QLocale_swap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.swap_0(self);
    // return 1;
  }
}
pub trait QLocale_swap_0<RetType> {
  fn swap_0(self , rsthis: & QLocale) -> RetType;
}
impl<'a> /*trait*/ QLocale_swap_0<(/*void*/)> for (usize) {
  fn swap_0(self , rsthis: & QLocale) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QLocale4swapERS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qlocale.h:942
// index:0
// Public Visibility=Default Availability=Available
// [4] QLocale::Language language() const

/*
Returns the language of this locale.

See also script(), country(), languageToString(), and bcp47Name().
*/
impl /*struct*/ QLocale {
  pub fn language_0<RetType, T: QLocale_language_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.language_0(self);
    // return 1;
  }
}
pub trait QLocale_language_0<RetType> {
  fn language_0(self , rsthis: & QLocale) -> RetType;
}
impl<'a> /*trait*/ QLocale_language_0<i32> for () {
  fn language_0(self , rsthis: & QLocale) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLocale8languageEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlocale.h:943
// index:0
// Public Visibility=Default Availability=Available
// [4] QLocale::Script script() const

/*
Returns the script of this locale.

This function was introduced in  Qt 4.8.

See also language(), country(), languageToString(), scriptToString(), and bcp47Name().
*/
impl /*struct*/ QLocale {
  pub fn script_0<RetType, T: QLocale_script_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.script_0(self);
    // return 1;
  }
}
pub trait QLocale_script_0<RetType> {
  fn script_0(self , rsthis: & QLocale) -> RetType;
}
impl<'a> /*trait*/ QLocale_script_0<i32> for () {
  fn script_0(self , rsthis: & QLocale) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLocale6scriptEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlocale.h:944
// index:0
// Public Visibility=Default Availability=Available
// [4] QLocale::Country country() const

/*
Returns the country of this locale.

See also language(), script(), countryToString(), and bcp47Name().
*/
impl /*struct*/ QLocale {
  pub fn country_0<RetType, T: QLocale_country_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.country_0(self);
    // return 1;
  }
}
pub trait QLocale_country_0<RetType> {
  fn country_0(self , rsthis: & QLocale) -> RetType;
}
impl<'a> /*trait*/ QLocale_country_0<i32> for () {
  fn country_0(self , rsthis: & QLocale) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLocale7countryEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlocale.h:945
// index:0
// Public Visibility=Default Availability=Available
// [8] QString name() const

/*
Returns the language and country of this locale as a string of the form "language_country", where language is a lowercase, two-letter ISO 639 language code, and country is an uppercase, two- or three-letter ISO 3166 country code.

Note that even if QLocale object was constructed with an explicit script, name() will not contain it for compatibility reasons. Use bcp47Name() instead if you need a full locale name.

See also QLocale(), language(), script(), country(), and bcp47Name().
*/
impl /*struct*/ QLocale {
  pub fn name_0<RetType, T: QLocale_name_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.name_0(self);
    // return 1;
  }
}
pub trait QLocale_name_0<RetType> {
  fn name_0(self , rsthis: & QLocale) -> RetType;
}
impl<'a> /*trait*/ QLocale_name_0<usize> for () {
  fn name_0(self , rsthis: & QLocale) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLocale4nameEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlocale.h:947
// index:0
// Public Visibility=Default Availability=Available
// [8] QString bcp47Name() const

/*
Returns the dash-separated language, script and country (and possibly other BCP47 fields) of this locale as a string.

Unlike the uiLanguages() the returned value of the bcp47Name() represents the locale name of the QLocale data but not the language the user-interface should be in.

This function tries to conform the locale name to BCP47.

This function was introduced in  Qt 4.8.

See also language(), country(), script(), and uiLanguages().
*/
impl /*struct*/ QLocale {
  pub fn bcp47Name_0<RetType, T: QLocale_bcp47Name_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.bcp47Name_0(self);
    // return 1;
  }
}
pub trait QLocale_bcp47Name_0<RetType> {
  fn bcp47Name_0(self , rsthis: & QLocale) -> RetType;
}
impl<'a> /*trait*/ QLocale_bcp47Name_0<usize> for () {
  fn bcp47Name_0(self , rsthis: & QLocale) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLocale9bcp47NameEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlocale.h:948
// index:0
// Public Visibility=Default Availability=Available
// [8] QString nativeLanguageName() const

/*
Returns a native name of the language for the locale. For example "Schwiizertüütsch" for Swiss-German locale.

This function was introduced in  Qt 4.8.

See also nativeCountryName() and languageToString().
*/
impl /*struct*/ QLocale {
  pub fn nativeLanguageName_0<RetType, T: QLocale_nativeLanguageName_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.nativeLanguageName_0(self);
    // return 1;
  }
}
pub trait QLocale_nativeLanguageName_0<RetType> {
  fn nativeLanguageName_0(self , rsthis: & QLocale) -> RetType;
}
impl<'a> /*trait*/ QLocale_nativeLanguageName_0<usize> for () {
  fn nativeLanguageName_0(self , rsthis: & QLocale) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLocale18nativeLanguageNameEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlocale.h:949
// index:0
// Public Visibility=Default Availability=Available
// [8] QString nativeCountryName() const

/*
Returns a native name of the country for the locale. For example "España" for Spanish/Spain locale.

This function was introduced in  Qt 4.8.

See also nativeLanguageName() and countryToString().
*/
impl /*struct*/ QLocale {
  pub fn nativeCountryName_0<RetType, T: QLocale_nativeCountryName_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.nativeCountryName_0(self);
    // return 1;
  }
}
pub trait QLocale_nativeCountryName_0<RetType> {
  fn nativeCountryName_0(self , rsthis: & QLocale) -> RetType;
}
impl<'a> /*trait*/ QLocale_nativeCountryName_0<usize> for () {
  fn nativeCountryName_0(self , rsthis: & QLocale) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLocale17nativeCountryNameEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlocale.h:952
// index:0
// Public Visibility=Default Availability=Available
// [2] short toShort(const QString &, bool *) const

/*
Returns the short int represented by the localized string s.

If the conversion fails the function returns 0.

If ok is not 0, failure is reported by setting *ok to false, and success by setting *ok to true.

This function ignores leading and trailing whitespace.

See also toUShort() and toString().
*/
impl /*struct*/ QLocale {
  pub fn toShort_0<RetType, T: QLocale_toShort_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toShort_0(self);
    // return 1;
  }
}
pub trait QLocale_toShort_0<RetType> {
  fn toShort_0(self , rsthis: & QLocale) -> RetType;
}
impl<'a> /*trait*/ QLocale_toShort_0<i16> for (usize,usize) {
  fn toShort_0(self , rsthis: & QLocale) -> i16 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLocale7toShortERK7QStringPb", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i16 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlocale.h:961
// index:1
// Public Visibility=Default Availability=Available
// [2] short toShort(const QStringRef &, bool *) const

/*
Returns the short int represented by the localized string s.

If the conversion fails the function returns 0.

If ok is not 0, failure is reported by setting *ok to false, and success by setting *ok to true.

This function ignores leading and trailing whitespace.

See also toUShort() and toString().
*/
impl /*struct*/ QLocale {
  pub fn toShort_1<RetType, T: QLocale_toShort_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toShort_1(self);
    // return 1;
  }
}
pub trait QLocale_toShort_1<RetType> {
  fn toShort_1(self , rsthis: & QLocale) -> RetType;
}
impl<'a> /*trait*/ QLocale_toShort_1<i16> for (usize,usize) {
  fn toShort_1(self , rsthis: & QLocale) -> i16 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLocale7toShortERK10QStringRefPb", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i16 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlocale.h:971
// index:2
// Public Visibility=Default Availability=Available
// [2] short toShort(QStringView, bool *) const

/*
Returns the short int represented by the localized string s.

If the conversion fails the function returns 0.

If ok is not 0, failure is reported by setting *ok to false, and success by setting *ok to true.

This function ignores leading and trailing whitespace.

See also toUShort() and toString().
*/
impl /*struct*/ QLocale {
  pub fn toShort_2<RetType, T: QLocale_toShort_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toShort_2(self);
    // return 1;
  }
}
pub trait QLocale_toShort_2<RetType> {
  fn toShort_2(self , rsthis: & QLocale) -> RetType;
}
impl<'a> /*trait*/ QLocale_toShort_2<i16> for (usize,usize) {
  fn toShort_2(self , rsthis: & QLocale) -> i16 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLocale7toShortE11QStringViewPb", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i16 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlocale.h:953
// index:0
// Public Visibility=Default Availability=Available
// [2] ushort toUShort(const QString &, bool *) const

/*
Returns the unsigned short int represented by the localized string s.

If the conversion fails the function returns 0.

If ok is not 0, failure is reported by setting *ok to false, and success by setting *ok to true.

This function ignores leading and trailing whitespace.

See also toShort() and toString().
*/
impl /*struct*/ QLocale {
  pub fn toUShort_0<RetType, T: QLocale_toUShort_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toUShort_0(self);
    // return 1;
  }
}
pub trait QLocale_toUShort_0<RetType> {
  fn toUShort_0(self , rsthis: & QLocale) -> RetType;
}
impl<'a> /*trait*/ QLocale_toUShort_0<u16> for (usize,usize) {
  fn toUShort_0(self , rsthis: & QLocale) -> u16 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLocale8toUShortERK7QStringPb", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u16 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlocale.h:962
// index:1
// Public Visibility=Default Availability=Available
// [2] ushort toUShort(const QStringRef &, bool *) const

/*
Returns the unsigned short int represented by the localized string s.

If the conversion fails the function returns 0.

If ok is not 0, failure is reported by setting *ok to false, and success by setting *ok to true.

This function ignores leading and trailing whitespace.

See also toShort() and toString().
*/
impl /*struct*/ QLocale {
  pub fn toUShort_1<RetType, T: QLocale_toUShort_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toUShort_1(self);
    // return 1;
  }
}
pub trait QLocale_toUShort_1<RetType> {
  fn toUShort_1(self , rsthis: & QLocale) -> RetType;
}
impl<'a> /*trait*/ QLocale_toUShort_1<u16> for (usize,usize) {
  fn toUShort_1(self , rsthis: & QLocale) -> u16 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLocale8toUShortERK10QStringRefPb", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u16 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlocale.h:972
// index:2
// Public Visibility=Default Availability=Available
// [2] ushort toUShort(QStringView, bool *) const

/*
Returns the unsigned short int represented by the localized string s.

If the conversion fails the function returns 0.

If ok is not 0, failure is reported by setting *ok to false, and success by setting *ok to true.

This function ignores leading and trailing whitespace.

See also toShort() and toString().
*/
impl /*struct*/ QLocale {
  pub fn toUShort_2<RetType, T: QLocale_toUShort_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toUShort_2(self);
    // return 1;
  }
}
pub trait QLocale_toUShort_2<RetType> {
  fn toUShort_2(self , rsthis: & QLocale) -> RetType;
}
impl<'a> /*trait*/ QLocale_toUShort_2<u16> for (usize,usize) {
  fn toUShort_2(self , rsthis: & QLocale) -> u16 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLocale8toUShortE11QStringViewPb", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u16 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlocale.h:954
// index:0
// Public Visibility=Default Availability=Available
// [4] int toInt(const QString &, bool *) const

/*
Returns the int represented by the localized string s.

If the conversion fails the function returns 0.

If ok is not 0, failure is reported by setting *ok to false, and success by setting *ok to true.

This function ignores leading and trailing whitespace.

See also toUInt() and toString().
*/
impl /*struct*/ QLocale {
  pub fn toInt_0<RetType, T: QLocale_toInt_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toInt_0(self);
    // return 1;
  }
}
pub trait QLocale_toInt_0<RetType> {
  fn toInt_0(self , rsthis: & QLocale) -> RetType;
}
impl<'a> /*trait*/ QLocale_toInt_0<i32> for (usize,usize) {
  fn toInt_0(self , rsthis: & QLocale) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLocale5toIntERK7QStringPb", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlocale.h:963
// index:1
// Public Visibility=Default Availability=Available
// [4] int toInt(const QStringRef &, bool *) const

/*
Returns the int represented by the localized string s.

If the conversion fails the function returns 0.

If ok is not 0, failure is reported by setting *ok to false, and success by setting *ok to true.

This function ignores leading and trailing whitespace.

See also toUInt() and toString().
*/
impl /*struct*/ QLocale {
  pub fn toInt_1<RetType, T: QLocale_toInt_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toInt_1(self);
    // return 1;
  }
}
pub trait QLocale_toInt_1<RetType> {
  fn toInt_1(self , rsthis: & QLocale) -> RetType;
}
impl<'a> /*trait*/ QLocale_toInt_1<i32> for (usize,usize) {
  fn toInt_1(self , rsthis: & QLocale) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLocale5toIntERK10QStringRefPb", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlocale.h:973
// index:2
// Public Visibility=Default Availability=Available
// [4] int toInt(QStringView, bool *) const

/*
Returns the int represented by the localized string s.

If the conversion fails the function returns 0.

If ok is not 0, failure is reported by setting *ok to false, and success by setting *ok to true.

This function ignores leading and trailing whitespace.

See also toUInt() and toString().
*/
impl /*struct*/ QLocale {
  pub fn toInt_2<RetType, T: QLocale_toInt_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toInt_2(self);
    // return 1;
  }
}
pub trait QLocale_toInt_2<RetType> {
  fn toInt_2(self , rsthis: & QLocale) -> RetType;
}
impl<'a> /*trait*/ QLocale_toInt_2<i32> for (usize,usize) {
  fn toInt_2(self , rsthis: & QLocale) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLocale5toIntE11QStringViewPb", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlocale.h:955
// index:0
// Public Visibility=Default Availability=Available
// [4] uint toUInt(const QString &, bool *) const

/*
Returns the unsigned int represented by the localized string s.

If the conversion fails the function returns 0.

If ok is not 0, failure is reported by setting *ok to false, and success by setting *ok to true.

This function ignores leading and trailing whitespace.

See also toInt() and toString().
*/
impl /*struct*/ QLocale {
  pub fn toUInt_0<RetType, T: QLocale_toUInt_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toUInt_0(self);
    // return 1;
  }
}
pub trait QLocale_toUInt_0<RetType> {
  fn toUInt_0(self , rsthis: & QLocale) -> RetType;
}
impl<'a> /*trait*/ QLocale_toUInt_0<u32> for (usize,usize) {
  fn toUInt_0(self , rsthis: & QLocale) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLocale6toUIntERK7QStringPb", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlocale.h:964
// index:1
// Public Visibility=Default Availability=Available
// [4] uint toUInt(const QStringRef &, bool *) const

/*
Returns the unsigned int represented by the localized string s.

If the conversion fails the function returns 0.

If ok is not 0, failure is reported by setting *ok to false, and success by setting *ok to true.

This function ignores leading and trailing whitespace.

See also toInt() and toString().
*/
impl /*struct*/ QLocale {
  pub fn toUInt_1<RetType, T: QLocale_toUInt_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toUInt_1(self);
    // return 1;
  }
}
pub trait QLocale_toUInt_1<RetType> {
  fn toUInt_1(self , rsthis: & QLocale) -> RetType;
}
impl<'a> /*trait*/ QLocale_toUInt_1<u32> for (usize,usize) {
  fn toUInt_1(self , rsthis: & QLocale) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLocale6toUIntERK10QStringRefPb", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlocale.h:974
// index:2
// Public Visibility=Default Availability=Available
// [4] uint toUInt(QStringView, bool *) const

/*
Returns the unsigned int represented by the localized string s.

If the conversion fails the function returns 0.

If ok is not 0, failure is reported by setting *ok to false, and success by setting *ok to true.

This function ignores leading and trailing whitespace.

See also toInt() and toString().
*/
impl /*struct*/ QLocale {
  pub fn toUInt_2<RetType, T: QLocale_toUInt_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toUInt_2(self);
    // return 1;
  }
}
pub trait QLocale_toUInt_2<RetType> {
  fn toUInt_2(self , rsthis: & QLocale) -> RetType;
}
impl<'a> /*trait*/ QLocale_toUInt_2<u32> for (usize,usize) {
  fn toUInt_2(self , rsthis: & QLocale) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLocale6toUIntE11QStringViewPb", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlocale.h:956
// index:0
// Public Visibility=Default Availability=Available
// [8] qlonglong toLongLong(const QString &, bool *) const

/*
Returns the long long int represented by the localized string s.

If the conversion fails the function returns 0.

If ok is not 0, failure is reported by setting *ok to false, and success by setting *ok to true.

This function ignores leading and trailing whitespace.

See also toInt(), toULongLong(), toDouble(), and toString().
*/
impl /*struct*/ QLocale {
  pub fn toLongLong_0<RetType, T: QLocale_toLongLong_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toLongLong_0(self);
    // return 1;
  }
}
pub trait QLocale_toLongLong_0<RetType> {
  fn toLongLong_0(self , rsthis: & QLocale) -> RetType;
}
impl<'a> /*trait*/ QLocale_toLongLong_0<i64> for (usize,usize) {
  fn toLongLong_0(self , rsthis: & QLocale) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLocale10toLongLongERK7QStringPb", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlocale.h:965
// index:1
// Public Visibility=Default Availability=Available
// [8] qlonglong toLongLong(const QStringRef &, bool *) const

/*
Returns the long long int represented by the localized string s.

If the conversion fails the function returns 0.

If ok is not 0, failure is reported by setting *ok to false, and success by setting *ok to true.

This function ignores leading and trailing whitespace.

See also toInt(), toULongLong(), toDouble(), and toString().
*/
impl /*struct*/ QLocale {
  pub fn toLongLong_1<RetType, T: QLocale_toLongLong_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toLongLong_1(self);
    // return 1;
  }
}
pub trait QLocale_toLongLong_1<RetType> {
  fn toLongLong_1(self , rsthis: & QLocale) -> RetType;
}
impl<'a> /*trait*/ QLocale_toLongLong_1<i64> for (usize,usize) {
  fn toLongLong_1(self , rsthis: & QLocale) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLocale10toLongLongERK10QStringRefPb", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlocale.h:975
// index:2
// Public Visibility=Default Availability=Available
// [8] qlonglong toLongLong(QStringView, bool *) const

/*
Returns the long long int represented by the localized string s.

If the conversion fails the function returns 0.

If ok is not 0, failure is reported by setting *ok to false, and success by setting *ok to true.

This function ignores leading and trailing whitespace.

See also toInt(), toULongLong(), toDouble(), and toString().
*/
impl /*struct*/ QLocale {
  pub fn toLongLong_2<RetType, T: QLocale_toLongLong_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toLongLong_2(self);
    // return 1;
  }
}
pub trait QLocale_toLongLong_2<RetType> {
  fn toLongLong_2(self , rsthis: & QLocale) -> RetType;
}
impl<'a> /*trait*/ QLocale_toLongLong_2<i64> for (usize,usize) {
  fn toLongLong_2(self , rsthis: & QLocale) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLocale10toLongLongE11QStringViewPb", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlocale.h:957
// index:0
// Public Visibility=Default Availability=Available
// [8] qulonglong toULongLong(const QString &, bool *) const

/*
Returns the unsigned long long int represented by the localized string s.

If the conversion fails the function returns 0.

If ok is not 0, failure is reported by setting *ok to false, and success by setting *ok to true.

This function ignores leading and trailing whitespace.

See also toLongLong(), toInt(), toDouble(), and toString().
*/
impl /*struct*/ QLocale {
  pub fn toULongLong_0<RetType, T: QLocale_toULongLong_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toULongLong_0(self);
    // return 1;
  }
}
pub trait QLocale_toULongLong_0<RetType> {
  fn toULongLong_0(self , rsthis: & QLocale) -> RetType;
}
impl<'a> /*trait*/ QLocale_toULongLong_0<u64> for (usize,usize) {
  fn toULongLong_0(self , rsthis: & QLocale) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLocale11toULongLongERK7QStringPb", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlocale.h:966
// index:1
// Public Visibility=Default Availability=Available
// [8] qulonglong toULongLong(const QStringRef &, bool *) const

/*
Returns the unsigned long long int represented by the localized string s.

If the conversion fails the function returns 0.

If ok is not 0, failure is reported by setting *ok to false, and success by setting *ok to true.

This function ignores leading and trailing whitespace.

See also toLongLong(), toInt(), toDouble(), and toString().
*/
impl /*struct*/ QLocale {
  pub fn toULongLong_1<RetType, T: QLocale_toULongLong_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toULongLong_1(self);
    // return 1;
  }
}
pub trait QLocale_toULongLong_1<RetType> {
  fn toULongLong_1(self , rsthis: & QLocale) -> RetType;
}
impl<'a> /*trait*/ QLocale_toULongLong_1<u64> for (usize,usize) {
  fn toULongLong_1(self , rsthis: & QLocale) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLocale11toULongLongERK10QStringRefPb", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlocale.h:976
// index:2
// Public Visibility=Default Availability=Available
// [8] qulonglong toULongLong(QStringView, bool *) const

/*
Returns the unsigned long long int represented by the localized string s.

If the conversion fails the function returns 0.

If ok is not 0, failure is reported by setting *ok to false, and success by setting *ok to true.

This function ignores leading and trailing whitespace.

See also toLongLong(), toInt(), toDouble(), and toString().
*/
impl /*struct*/ QLocale {
  pub fn toULongLong_2<RetType, T: QLocale_toULongLong_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toULongLong_2(self);
    // return 1;
  }
}
pub trait QLocale_toULongLong_2<RetType> {
  fn toULongLong_2(self , rsthis: & QLocale) -> RetType;
}
impl<'a> /*trait*/ QLocale_toULongLong_2<u64> for (usize,usize) {
  fn toULongLong_2(self , rsthis: & QLocale) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLocale11toULongLongE11QStringViewPb", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlocale.h:958
// index:0
// Public Visibility=Default Availability=Available
// [4] float toFloat(const QString &, bool *) const

/*
Returns the float represented by the localized string s, or 0.0 if the conversion failed.

If ok is not 0, reports failure by setting *ok to false and success by setting *ok to true.

This function ignores leading and trailing whitespace.

See also toDouble(), toInt(), and toString().
*/
impl /*struct*/ QLocale {
  pub fn toFloat_0<RetType, T: QLocale_toFloat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toFloat_0(self);
    // return 1;
  }
}
pub trait QLocale_toFloat_0<RetType> {
  fn toFloat_0(self , rsthis: & QLocale) -> RetType;
}
impl<'a> /*trait*/ QLocale_toFloat_0<f32> for (usize,usize) {
  fn toFloat_0(self , rsthis: & QLocale) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLocale7toFloatERK7QStringPb", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlocale.h:967
// index:1
// Public Visibility=Default Availability=Available
// [4] float toFloat(const QStringRef &, bool *) const

/*
Returns the float represented by the localized string s, or 0.0 if the conversion failed.

If ok is not 0, reports failure by setting *ok to false and success by setting *ok to true.

This function ignores leading and trailing whitespace.

See also toDouble(), toInt(), and toString().
*/
impl /*struct*/ QLocale {
  pub fn toFloat_1<RetType, T: QLocale_toFloat_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toFloat_1(self);
    // return 1;
  }
}
pub trait QLocale_toFloat_1<RetType> {
  fn toFloat_1(self , rsthis: & QLocale) -> RetType;
}
impl<'a> /*trait*/ QLocale_toFloat_1<f32> for (usize,usize) {
  fn toFloat_1(self , rsthis: & QLocale) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLocale7toFloatERK10QStringRefPb", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlocale.h:977
// index:2
// Public Visibility=Default Availability=Available
// [4] float toFloat(QStringView, bool *) const

/*
Returns the float represented by the localized string s, or 0.0 if the conversion failed.

If ok is not 0, reports failure by setting *ok to false and success by setting *ok to true.

This function ignores leading and trailing whitespace.

See also toDouble(), toInt(), and toString().
*/
impl /*struct*/ QLocale {
  pub fn toFloat_2<RetType, T: QLocale_toFloat_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toFloat_2(self);
    // return 1;
  }
}
pub trait QLocale_toFloat_2<RetType> {
  fn toFloat_2(self , rsthis: & QLocale) -> RetType;
}
impl<'a> /*trait*/ QLocale_toFloat_2<f32> for (usize,usize) {
  fn toFloat_2(self , rsthis: & QLocale) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLocale7toFloatE11QStringViewPb", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlocale.h:959
// index:0
// Public Visibility=Default Availability=Available
// [8] double toDouble(const QString &, bool *) const

/*
Returns the double represented by the localized string s, or 0.0 if the conversion failed.

If ok is not 0, reports failure by setting *ok to false and success by setting *ok to true.

Unlike QString::toDouble(), this function does not use the 'C' locale if the string cannot be interpreted in this locale.


  bool ok;
  double d;

  QLocale c(QLocale::C);
  d = c.toDouble( "1234.56", &ok );  // ok == true, d == 1234.56
  d = c.toDouble( "1,234.56", &ok ); // ok == true, d == 1234.56
  d = c.toDouble( "1234,56", &ok );  // ok == false

  QLocale german(QLocale::German);
  d = german.toDouble( "1234,56", &ok );  // ok == true, d == 1234.56
  d = german.toDouble( "1.234,56", &ok ); // ok == true, d == 1234.56
  d = german.toDouble( "1234.56", &ok );  // ok == false

  d = german.toDouble( "1.234", &ok );    // ok == true, d == 1234.0



Notice that the last conversion returns 1234.0, because '.' is the thousands group separator in the German locale.

This function ignores leading and trailing whitespace.

See also toFloat(), toInt(), and toString().
*/
impl /*struct*/ QLocale {
  pub fn toDouble_0<RetType, T: QLocale_toDouble_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toDouble_0(self);
    // return 1;
  }
}
pub trait QLocale_toDouble_0<RetType> {
  fn toDouble_0(self , rsthis: & QLocale) -> RetType;
}
impl<'a> /*trait*/ QLocale_toDouble_0<f64> for (usize,usize) {
  fn toDouble_0(self , rsthis: & QLocale) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLocale8toDoubleERK7QStringPb", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlocale.h:968
// index:1
// Public Visibility=Default Availability=Available
// [8] double toDouble(const QStringRef &, bool *) const

/*
Returns the double represented by the localized string s, or 0.0 if the conversion failed.

If ok is not 0, reports failure by setting *ok to false and success by setting *ok to true.

Unlike QString::toDouble(), this function does not use the 'C' locale if the string cannot be interpreted in this locale.


  bool ok;
  double d;

  QLocale c(QLocale::C);
  d = c.toDouble( "1234.56", &ok );  // ok == true, d == 1234.56
  d = c.toDouble( "1,234.56", &ok ); // ok == true, d == 1234.56
  d = c.toDouble( "1234,56", &ok );  // ok == false

  QLocale german(QLocale::German);
  d = german.toDouble( "1234,56", &ok );  // ok == true, d == 1234.56
  d = german.toDouble( "1.234,56", &ok ); // ok == true, d == 1234.56
  d = german.toDouble( "1234.56", &ok );  // ok == false

  d = german.toDouble( "1.234", &ok );    // ok == true, d == 1234.0



Notice that the last conversion returns 1234.0, because '.' is the thousands group separator in the German locale.

This function ignores leading and trailing whitespace.

See also toFloat(), toInt(), and toString().
*/
impl /*struct*/ QLocale {
  pub fn toDouble_1<RetType, T: QLocale_toDouble_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toDouble_1(self);
    // return 1;
  }
}
pub trait QLocale_toDouble_1<RetType> {
  fn toDouble_1(self , rsthis: & QLocale) -> RetType;
}
impl<'a> /*trait*/ QLocale_toDouble_1<f64> for (usize,usize) {
  fn toDouble_1(self , rsthis: & QLocale) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLocale8toDoubleERK10QStringRefPb", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlocale.h:978
// index:2
// Public Visibility=Default Availability=Available
// [8] double toDouble(QStringView, bool *) const

/*
Returns the double represented by the localized string s, or 0.0 if the conversion failed.

If ok is not 0, reports failure by setting *ok to false and success by setting *ok to true.

Unlike QString::toDouble(), this function does not use the 'C' locale if the string cannot be interpreted in this locale.


  bool ok;
  double d;

  QLocale c(QLocale::C);
  d = c.toDouble( "1234.56", &ok );  // ok == true, d == 1234.56
  d = c.toDouble( "1,234.56", &ok ); // ok == true, d == 1234.56
  d = c.toDouble( "1234,56", &ok );  // ok == false

  QLocale german(QLocale::German);
  d = german.toDouble( "1234,56", &ok );  // ok == true, d == 1234.56
  d = german.toDouble( "1.234,56", &ok ); // ok == true, d == 1234.56
  d = german.toDouble( "1234.56", &ok );  // ok == false

  d = german.toDouble( "1.234", &ok );    // ok == true, d == 1234.0



Notice that the last conversion returns 1234.0, because '.' is the thousands group separator in the German locale.

This function ignores leading and trailing whitespace.

See also toFloat(), toInt(), and toString().
*/
impl /*struct*/ QLocale {
  pub fn toDouble_2<RetType, T: QLocale_toDouble_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toDouble_2(self);
    // return 1;
  }
}
pub trait QLocale_toDouble_2<RetType> {
  fn toDouble_2(self , rsthis: & QLocale) -> RetType;
}
impl<'a> /*trait*/ QLocale_toDouble_2<f64> for (usize,usize) {
  fn toDouble_2(self , rsthis: & QLocale) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLocale8toDoubleE11QStringViewPb", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlocale.h:980
// index:0
// Public Visibility=Default Availability=Available
// [8] QString toString(qlonglong) const

/*
Returns a localized string representation of i.

See also toLongLong().
*/
impl /*struct*/ QLocale {
  pub fn toString_0<RetType, T: QLocale_toString_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toString_0(self);
    // return 1;
  }
}
pub trait QLocale_toString_0<RetType> {
  fn toString_0(self , rsthis: & QLocale) -> RetType;
}
impl<'a> /*trait*/ QLocale_toString_0<usize> for (i64) {
  fn toString_0(self , rsthis: & QLocale) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLocale8toStringEx", 1,qtrt::FFITY_SINT64,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlocale.h:981
// index:1
// Public Visibility=Default Availability=Available
// [8] QString toString(qulonglong) const

/*
Returns a localized string representation of i.

See also toLongLong().
*/
impl /*struct*/ QLocale {
  pub fn toString_1<RetType, T: QLocale_toString_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toString_1(self);
    // return 1;
  }
}
pub trait QLocale_toString_1<RetType> {
  fn toString_1(self , rsthis: & QLocale) -> RetType;
}
impl<'a> /*trait*/ QLocale_toString_1<usize> for (u64) {
  fn toString_1(self , rsthis: & QLocale) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const u64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLocale8toStringEy", 1,qtrt::FFITY_UINT64,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlocale.h:982
// index:2
// Public inline Visibility=Default Availability=Available
// [8] QString toString(short) const

/*
Returns a localized string representation of i.

See also toLongLong().
*/
impl /*struct*/ QLocale {
  pub fn toString_2<RetType, T: QLocale_toString_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toString_2(self);
    // return 1;
  }
}
pub trait QLocale_toString_2<RetType> {
  fn toString_2(self , rsthis: & QLocale) -> RetType;
}
impl<'a> /*trait*/ QLocale_toString_2<usize> for (i16) {
  fn toString_2(self , rsthis: & QLocale) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i16 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLocale8toStringEs", 1,qtrt::FFITY_SINT16,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlocale.h:983
// index:3
// Public inline Visibility=Default Availability=Available
// [8] QString toString(ushort) const

/*
Returns a localized string representation of i.

See also toLongLong().
*/
impl /*struct*/ QLocale {
  pub fn toString_3<RetType, T: QLocale_toString_3<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toString_3(self);
    // return 1;
  }
}
pub trait QLocale_toString_3<RetType> {
  fn toString_3(self , rsthis: & QLocale) -> RetType;
}
impl<'a> /*trait*/ QLocale_toString_3<usize> for (u16) {
  fn toString_3(self , rsthis: & QLocale) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const u16 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLocale8toStringEt", 1,qtrt::FFITY_UINT16,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlocale.h:984
// index:4
// Public inline Visibility=Default Availability=Available
// [8] QString toString(int) const

/*
Returns a localized string representation of i.

See also toLongLong().
*/
impl /*struct*/ QLocale {
  pub fn toString_4<RetType, T: QLocale_toString_4<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toString_4(self);
    // return 1;
  }
}
pub trait QLocale_toString_4<RetType> {
  fn toString_4(self , rsthis: & QLocale) -> RetType;
}
impl<'a> /*trait*/ QLocale_toString_4<usize> for (i32) {
  fn toString_4(self , rsthis: & QLocale) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLocale8toStringEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlocale.h:985
// index:5
// Public inline Visibility=Default Availability=Available
// [8] QString toString(uint) const

/*
Returns a localized string representation of i.

See also toLongLong().
*/
impl /*struct*/ QLocale {
  pub fn toString_5<RetType, T: QLocale_toString_5<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toString_5(self);
    // return 1;
  }
}
pub trait QLocale_toString_5<RetType> {
  fn toString_5(self , rsthis: & QLocale) -> RetType;
}
impl<'a> /*trait*/ QLocale_toString_5<usize> for (u32) {
  fn toString_5(self , rsthis: & QLocale) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const u32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLocale8toStringEj", 1,qtrt::FFITY_UINT32,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlocale.h:986
// index:6
// Public Visibility=Default Availability=Available
// [8] QString toString(double, char, int) const

/*
Returns a localized string representation of i.

See also toLongLong().
*/
impl /*struct*/ QLocale {
  pub fn toString_6<RetType, T: QLocale_toString_6<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toString_6(self);
    // return 1;
  }
}
pub trait QLocale_toString_6<RetType> {
  fn toString_6(self , rsthis: & QLocale) -> RetType;
}
impl<'a> /*trait*/ QLocale_toString_6<usize> for (f64,i8,i32) {
  fn toString_6(self , rsthis: & QLocale) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const i8 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLocale8toStringEdci", 3,qtrt::FFITY_DOUBLE,qtrt::FFITY_SINT8,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlocale.h:987
// index:7
// Public inline Visibility=Default Availability=Available
// [8] QString toString(float, char, int) const

/*
Returns a localized string representation of i.

See also toLongLong().
*/
impl /*struct*/ QLocale {
  pub fn toString_7<RetType, T: QLocale_toString_7<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toString_7(self);
    // return 1;
  }
}
pub trait QLocale_toString_7<RetType> {
  fn toString_7(self , rsthis: & QLocale) -> RetType;
}
impl<'a> /*trait*/ QLocale_toString_7<usize> for (f32,i8,i32) {
  fn toString_7(self , rsthis: & QLocale) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f32 as usize;
    let arg1 = (&self.1) as *const i8 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLocale8toStringEfci", 3,qtrt::FFITY_FLOAT,qtrt::FFITY_SINT8,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlocale.h:990
// index:8
// Public Visibility=Default Availability=Available
// [8] QString toString(const QDate &, const QString &) const

/*
Returns a localized string representation of i.

See also toLongLong().
*/
impl /*struct*/ QLocale {
  pub fn toString_8<RetType, T: QLocale_toString_8<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toString_8(self);
    // return 1;
  }
}
pub trait QLocale_toString_8<RetType> {
  fn toString_8(self , rsthis: & QLocale) -> RetType;
}
impl<'a> /*trait*/ QLocale_toString_8<usize> for (usize,usize) {
  fn toString_8(self , rsthis: & QLocale) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLocale8toStringERK5QDateRK7QString", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlocale.h:991
// index:9
// Public Visibility=Default Availability=Available
// [8] QString toString(const QTime &, const QString &) const

/*
Returns a localized string representation of i.

See also toLongLong().
*/
impl /*struct*/ QLocale {
  pub fn toString_9<RetType, T: QLocale_toString_9<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toString_9(self);
    // return 1;
  }
}
pub trait QLocale_toString_9<RetType> {
  fn toString_9(self , rsthis: & QLocale) -> RetType;
}
impl<'a> /*trait*/ QLocale_toString_9<usize> for (usize,usize) {
  fn toString_9(self , rsthis: & QLocale) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLocale8toStringERK5QTimeRK7QString", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlocale.h:992
// index:10
// Public Visibility=Default Availability=Available
// [8] QString toString(const QDateTime &, const QString &) const

/*
Returns a localized string representation of i.

See also toLongLong().
*/
impl /*struct*/ QLocale {
  pub fn toString_10<RetType, T: QLocale_toString_10<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toString_10(self);
    // return 1;
  }
}
pub trait QLocale_toString_10<RetType> {
  fn toString_10(self , rsthis: & QLocale) -> RetType;
}
impl<'a> /*trait*/ QLocale_toString_10<usize> for (usize,usize) {
  fn toString_10(self , rsthis: & QLocale) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLocale8toStringERK9QDateTimeRK7QString", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlocale.h:994
// index:11
// Public Visibility=Default Availability=Available
// [8] QString toString(const QDate &, QStringView) const

/*
Returns a localized string representation of i.

See also toLongLong().
*/
impl /*struct*/ QLocale {
  pub fn toString_11<RetType, T: QLocale_toString_11<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toString_11(self);
    // return 1;
  }
}
pub trait QLocale_toString_11<RetType> {
  fn toString_11(self , rsthis: & QLocale) -> RetType;
}
impl<'a> /*trait*/ QLocale_toString_11<usize> for (usize,usize) {
  fn toString_11(self , rsthis: & QLocale) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLocale8toStringERK5QDate11QStringView", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlocale.h:995
// index:12
// Public Visibility=Default Availability=Available
// [8] QString toString(const QTime &, QStringView) const

/*
Returns a localized string representation of i.

See also toLongLong().
*/
impl /*struct*/ QLocale {
  pub fn toString_12<RetType, T: QLocale_toString_12<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toString_12(self);
    // return 1;
  }
}
pub trait QLocale_toString_12<RetType> {
  fn toString_12(self , rsthis: & QLocale) -> RetType;
}
impl<'a> /*trait*/ QLocale_toString_12<usize> for (usize,usize) {
  fn toString_12(self , rsthis: & QLocale) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLocale8toStringERK5QTime11QStringView", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlocale.h:996
// index:13
// Public Visibility=Default Availability=Available
// [8] QString toString(const QDateTime &, QStringView) const

/*
Returns a localized string representation of i.

See also toLongLong().
*/
impl /*struct*/ QLocale {
  pub fn toString_13<RetType, T: QLocale_toString_13<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toString_13(self);
    // return 1;
  }
}
pub trait QLocale_toString_13<RetType> {
  fn toString_13(self , rsthis: & QLocale) -> RetType;
}
impl<'a> /*trait*/ QLocale_toString_13<usize> for (usize,usize) {
  fn toString_13(self , rsthis: & QLocale) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLocale8toStringERK9QDateTime11QStringView", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlocale.h:997
// index:14
// Public Visibility=Default Availability=Available
// [8] QString toString(const QDate &, QLocale::FormatType) const

/*
Returns a localized string representation of i.

See also toLongLong().
*/
impl /*struct*/ QLocale {
  pub fn toString_14<RetType, T: QLocale_toString_14<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toString_14(self);
    // return 1;
  }
}
pub trait QLocale_toString_14<RetType> {
  fn toString_14(self , rsthis: & QLocale) -> RetType;
}
impl<'a> /*trait*/ QLocale_toString_14<usize> for (usize,i32) {
  fn toString_14(self , rsthis: & QLocale) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLocale8toStringERK5QDateNS_10FormatTypeE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlocale.h:998
// index:15
// Public Visibility=Default Availability=Available
// [8] QString toString(const QTime &, QLocale::FormatType) const

/*
Returns a localized string representation of i.

See also toLongLong().
*/
impl /*struct*/ QLocale {
  pub fn toString_15<RetType, T: QLocale_toString_15<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toString_15(self);
    // return 1;
  }
}
pub trait QLocale_toString_15<RetType> {
  fn toString_15(self , rsthis: & QLocale) -> RetType;
}
impl<'a> /*trait*/ QLocale_toString_15<usize> for (usize,i32) {
  fn toString_15(self , rsthis: & QLocale) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLocale8toStringERK5QTimeNS_10FormatTypeE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlocale.h:999
// index:16
// Public Visibility=Default Availability=Available
// [8] QString toString(const QDateTime &, QLocale::FormatType) const

/*
Returns a localized string representation of i.

See also toLongLong().
*/
impl /*struct*/ QLocale {
  pub fn toString_16<RetType, T: QLocale_toString_16<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toString_16(self);
    // return 1;
  }
}
pub trait QLocale_toString_16<RetType> {
  fn toString_16(self , rsthis: & QLocale) -> RetType;
}
impl<'a> /*trait*/ QLocale_toString_16<usize> for (usize,i32) {
  fn toString_16(self , rsthis: & QLocale) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLocale8toStringERK9QDateTimeNS_10FormatTypeE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlocale.h:1001
// index:0
// Public Visibility=Default Availability=Available
// [8] QString dateFormat(QLocale::FormatType) const

/*
Returns the date format used for the current locale.

If format is LongFormat the format will be a long version. Otherwise it uses a shorter version.

This function was introduced in  Qt 4.1.

See also QDate::toString() and QDate::fromString().
*/
impl /*struct*/ QLocale {
  pub fn dateFormat_0<RetType, T: QLocale_dateFormat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dateFormat_0(self);
    // return 1;
  }
}
pub trait QLocale_dateFormat_0<RetType> {
  fn dateFormat_0(self , rsthis: & QLocale) -> RetType;
}
impl<'a> /*trait*/ QLocale_dateFormat_0<usize> for (i32) {
  fn dateFormat_0(self , rsthis: & QLocale) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLocale10dateFormatENS_10FormatTypeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlocale.h:1002
// index:0
// Public Visibility=Default Availability=Available
// [8] QString timeFormat(QLocale::FormatType) const

/*
Returns the time format used for the current locale.

If format is LongFormat the format will be a long version. Otherwise it uses a shorter version.

This function was introduced in  Qt 4.1.

See also QTime::toString() and QTime::fromString().
*/
impl /*struct*/ QLocale {
  pub fn timeFormat_0<RetType, T: QLocale_timeFormat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.timeFormat_0(self);
    // return 1;
  }
}
pub trait QLocale_timeFormat_0<RetType> {
  fn timeFormat_0(self , rsthis: & QLocale) -> RetType;
}
impl<'a> /*trait*/ QLocale_timeFormat_0<usize> for (i32) {
  fn timeFormat_0(self , rsthis: & QLocale) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLocale10timeFormatENS_10FormatTypeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlocale.h:1003
// index:0
// Public Visibility=Default Availability=Available
// [8] QString dateTimeFormat(QLocale::FormatType) const

/*
Returns the date time format used for the current locale.

If format is ShortFormat the format will be a short version. Otherwise it uses a longer version.

This function was introduced in  Qt 4.4.

See also QDateTime::toString() and QDateTime::fromString().
*/
impl /*struct*/ QLocale {
  pub fn dateTimeFormat_0<RetType, T: QLocale_dateTimeFormat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dateTimeFormat_0(self);
    // return 1;
  }
}
pub trait QLocale_dateTimeFormat_0<RetType> {
  fn dateTimeFormat_0(self , rsthis: & QLocale) -> RetType;
}
impl<'a> /*trait*/ QLocale_dateTimeFormat_0<usize> for (i32) {
  fn dateTimeFormat_0(self , rsthis: & QLocale) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLocale14dateTimeFormatENS_10FormatTypeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlocale.h:1005
// index:0
// Public Visibility=Default Availability=Available
// [8] QDate toDate(const QString &, QLocale::FormatType) const

/*
Parses the date string given in string and returns the date. The format of the date string is chosen according to the format parameter (see dateFormat()).

If the date could not be parsed, returns an invalid date.

This function was introduced in  Qt 4.4.

See also dateFormat(), toTime(), toDateTime(), and QDate::fromString().
*/
impl /*struct*/ QLocale {
  pub fn toDate_0<RetType, T: QLocale_toDate_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toDate_0(self);
    // return 1;
  }
}
pub trait QLocale_toDate_0<RetType> {
  fn toDate_0(self , rsthis: & QLocale) -> RetType;
}
impl<'a> /*trait*/ QLocale_toDate_0<usize> for (usize,i32) {
  fn toDate_0(self , rsthis: & QLocale) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLocale6toDateERK7QStringNS_10FormatTypeE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlocale.h:1008
// index:1
// Public Visibility=Default Availability=Available
// [8] QDate toDate(const QString &, const QString &) const

/*
Parses the date string given in string and returns the date. The format of the date string is chosen according to the format parameter (see dateFormat()).

If the date could not be parsed, returns an invalid date.

This function was introduced in  Qt 4.4.

See also dateFormat(), toTime(), toDateTime(), and QDate::fromString().
*/
impl /*struct*/ QLocale {
  pub fn toDate_1<RetType, T: QLocale_toDate_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toDate_1(self);
    // return 1;
  }
}
pub trait QLocale_toDate_1<RetType> {
  fn toDate_1(self , rsthis: & QLocale) -> RetType;
}
impl<'a> /*trait*/ QLocale_toDate_1<usize> for (usize,usize) {
  fn toDate_1(self , rsthis: & QLocale) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLocale6toDateERK7QStringS2_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlocale.h:1006
// index:0
// Public Visibility=Default Availability=Available
// [4] QTime toTime(const QString &, QLocale::FormatType) const

/*
Parses the time string given in string and returns the time. The format of the time string is chosen according to the format parameter (see timeFormat()).

If the time could not be parsed, returns an invalid time.

This function was introduced in  Qt 4.4.

See also timeFormat(), toDate(), toDateTime(), and QTime::fromString().
*/
impl /*struct*/ QLocale {
  pub fn toTime_0<RetType, T: QLocale_toTime_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toTime_0(self);
    // return 1;
  }
}
pub trait QLocale_toTime_0<RetType> {
  fn toTime_0(self , rsthis: & QLocale) -> RetType;
}
impl<'a> /*trait*/ QLocale_toTime_0<usize> for (usize,i32) {
  fn toTime_0(self , rsthis: & QLocale) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLocale6toTimeERK7QStringNS_10FormatTypeE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlocale.h:1009
// index:1
// Public Visibility=Default Availability=Available
// [4] QTime toTime(const QString &, const QString &) const

/*
Parses the time string given in string and returns the time. The format of the time string is chosen according to the format parameter (see timeFormat()).

If the time could not be parsed, returns an invalid time.

This function was introduced in  Qt 4.4.

See also timeFormat(), toDate(), toDateTime(), and QTime::fromString().
*/
impl /*struct*/ QLocale {
  pub fn toTime_1<RetType, T: QLocale_toTime_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toTime_1(self);
    // return 1;
  }
}
pub trait QLocale_toTime_1<RetType> {
  fn toTime_1(self , rsthis: & QLocale) -> RetType;
}
impl<'a> /*trait*/ QLocale_toTime_1<usize> for (usize,usize) {
  fn toTime_1(self , rsthis: & QLocale) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLocale6toTimeERK7QStringS2_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlocale.h:1007
// index:0
// Public Visibility=Default Availability=Available
// [8] QDateTime toDateTime(const QString &, QLocale::FormatType) const

/*
Parses the date/time string given in string and returns the time. The format of the date/time string is chosen according to the format parameter (see dateTimeFormat()).

If the string could not be parsed, returns an invalid QDateTime.

This function was introduced in  Qt 4.4.

See also dateTimeFormat(), toTime(), toDate(), and QDateTime::fromString().
*/
impl /*struct*/ QLocale {
  pub fn toDateTime_0<RetType, T: QLocale_toDateTime_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toDateTime_0(self);
    // return 1;
  }
}
pub trait QLocale_toDateTime_0<RetType> {
  fn toDateTime_0(self , rsthis: & QLocale) -> RetType;
}
impl<'a> /*trait*/ QLocale_toDateTime_0<usize> for (usize,i32) {
  fn toDateTime_0(self , rsthis: & QLocale) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLocale10toDateTimeERK7QStringNS_10FormatTypeE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlocale.h:1010
// index:1
// Public Visibility=Default Availability=Available
// [8] QDateTime toDateTime(const QString &, const QString &) const

/*
Parses the date/time string given in string and returns the time. The format of the date/time string is chosen according to the format parameter (see dateTimeFormat()).

If the string could not be parsed, returns an invalid QDateTime.

This function was introduced in  Qt 4.4.

See also dateTimeFormat(), toTime(), toDate(), and QDateTime::fromString().
*/
impl /*struct*/ QLocale {
  pub fn toDateTime_1<RetType, T: QLocale_toDateTime_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toDateTime_1(self);
    // return 1;
  }
}
pub trait QLocale_toDateTime_1<RetType> {
  fn toDateTime_1(self , rsthis: & QLocale) -> RetType;
}
impl<'a> /*trait*/ QLocale_toDateTime_1<usize> for (usize,usize) {
  fn toDateTime_1(self , rsthis: & QLocale) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLocale10toDateTimeERK7QStringS2_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlocale.h:1015
// index:0
// Public Visibility=Default Availability=Available
// [2] QChar decimalPoint() const

/*
Returns the decimal point character of this locale.

This function was introduced in  Qt 4.1.
*/
impl /*struct*/ QLocale {
  pub fn decimalPoint_0<RetType, T: QLocale_decimalPoint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.decimalPoint_0(self);
    // return 1;
  }
}
pub trait QLocale_decimalPoint_0<RetType> {
  fn decimalPoint_0(self , rsthis: & QLocale) -> RetType;
}
impl<'a> /*trait*/ QLocale_decimalPoint_0<usize> for () {
  fn decimalPoint_0(self , rsthis: & QLocale) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLocale12decimalPointEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlocale.h:1016
// index:0
// Public Visibility=Default Availability=Available
// [2] QChar groupSeparator() const

/*
Returns the group separator character of this locale.

This function was introduced in  Qt 4.1.
*/
impl /*struct*/ QLocale {
  pub fn groupSeparator_0<RetType, T: QLocale_groupSeparator_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.groupSeparator_0(self);
    // return 1;
  }
}
pub trait QLocale_groupSeparator_0<RetType> {
  fn groupSeparator_0(self , rsthis: & QLocale) -> RetType;
}
impl<'a> /*trait*/ QLocale_groupSeparator_0<usize> for () {
  fn groupSeparator_0(self , rsthis: & QLocale) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLocale14groupSeparatorEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlocale.h:1017
// index:0
// Public Visibility=Default Availability=Available
// [2] QChar percent() const

/*
Returns the percent character of this locale.

This function was introduced in  Qt 4.1.
*/
impl /*struct*/ QLocale {
  pub fn percent_0<RetType, T: QLocale_percent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.percent_0(self);
    // return 1;
  }
}
pub trait QLocale_percent_0<RetType> {
  fn percent_0(self , rsthis: & QLocale) -> RetType;
}
impl<'a> /*trait*/ QLocale_percent_0<usize> for () {
  fn percent_0(self , rsthis: & QLocale) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLocale7percentEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlocale.h:1018
// index:0
// Public Visibility=Default Availability=Available
// [2] QChar zeroDigit() const

/*
Returns the zero digit character of this locale.

This function was introduced in  Qt 4.1.
*/
impl /*struct*/ QLocale {
  pub fn zeroDigit_0<RetType, T: QLocale_zeroDigit_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.zeroDigit_0(self);
    // return 1;
  }
}
pub trait QLocale_zeroDigit_0<RetType> {
  fn zeroDigit_0(self , rsthis: & QLocale) -> RetType;
}
impl<'a> /*trait*/ QLocale_zeroDigit_0<usize> for () {
  fn zeroDigit_0(self , rsthis: & QLocale) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLocale9zeroDigitEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlocale.h:1019
// index:0
// Public Visibility=Default Availability=Available
// [2] QChar negativeSign() const

/*
Returns the negative sign character of this locale.

This function was introduced in  Qt 4.1.
*/
impl /*struct*/ QLocale {
  pub fn negativeSign_0<RetType, T: QLocale_negativeSign_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.negativeSign_0(self);
    // return 1;
  }
}
pub trait QLocale_negativeSign_0<RetType> {
  fn negativeSign_0(self , rsthis: & QLocale) -> RetType;
}
impl<'a> /*trait*/ QLocale_negativeSign_0<usize> for () {
  fn negativeSign_0(self , rsthis: & QLocale) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLocale12negativeSignEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlocale.h:1020
// index:0
// Public Visibility=Default Availability=Available
// [2] QChar positiveSign() const

/*
Returns the positive sign character of this locale.

This function was introduced in  Qt 4.5.
*/
impl /*struct*/ QLocale {
  pub fn positiveSign_0<RetType, T: QLocale_positiveSign_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.positiveSign_0(self);
    // return 1;
  }
}
pub trait QLocale_positiveSign_0<RetType> {
  fn positiveSign_0(self , rsthis: & QLocale) -> RetType;
}
impl<'a> /*trait*/ QLocale_positiveSign_0<usize> for () {
  fn positiveSign_0(self , rsthis: & QLocale) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLocale12positiveSignEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlocale.h:1021
// index:0
// Public Visibility=Default Availability=Available
// [2] QChar exponential() const

/*
Returns the exponential character of this locale.

This function was introduced in  Qt 4.1.
*/
impl /*struct*/ QLocale {
  pub fn exponential_0<RetType, T: QLocale_exponential_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.exponential_0(self);
    // return 1;
  }
}
pub trait QLocale_exponential_0<RetType> {
  fn exponential_0(self , rsthis: & QLocale) -> RetType;
}
impl<'a> /*trait*/ QLocale_exponential_0<usize> for () {
  fn exponential_0(self , rsthis: & QLocale) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLocale11exponentialEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlocale.h:1023
// index:0
// Public Visibility=Default Availability=Available
// [8] QString monthName(int, QLocale::FormatType) const

/*
Returns the localized name of month, in the format specified by type.

This function was introduced in  Qt 4.2.

See also dayName() and standaloneMonthName().
*/
impl /*struct*/ QLocale {
  pub fn monthName_0<RetType, T: QLocale_monthName_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.monthName_0(self);
    // return 1;
  }
}
pub trait QLocale_monthName_0<RetType> {
  fn monthName_0(self , rsthis: & QLocale) -> RetType;
}
impl<'a> /*trait*/ QLocale_monthName_0<usize> for (i32,i32) {
  fn monthName_0(self , rsthis: & QLocale) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLocale9monthNameEiNS_10FormatTypeE", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlocale.h:1024
// index:0
// Public Visibility=Default Availability=Available
// [8] QString standaloneMonthName(int, QLocale::FormatType) const

/*
Returns the localized name of month that is used as a standalone text, in the format specified by type.

If the locale information doesn't specify the standalone month name then return value is the same as in monthName().

This function was introduced in  Qt 4.5.

See also monthName() and standaloneDayName().
*/
impl /*struct*/ QLocale {
  pub fn standaloneMonthName_0<RetType, T: QLocale_standaloneMonthName_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.standaloneMonthName_0(self);
    // return 1;
  }
}
pub trait QLocale_standaloneMonthName_0<RetType> {
  fn standaloneMonthName_0(self , rsthis: & QLocale) -> RetType;
}
impl<'a> /*trait*/ QLocale_standaloneMonthName_0<usize> for (i32,i32) {
  fn standaloneMonthName_0(self , rsthis: & QLocale) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLocale19standaloneMonthNameEiNS_10FormatTypeE", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlocale.h:1025
// index:0
// Public Visibility=Default Availability=Available
// [8] QString dayName(int, QLocale::FormatType) const

/*
Returns the localized name of the day (where 1 represents Monday, 2 represents Tuesday and so on), in the format specified by type.

This function was introduced in  Qt 4.2.

See also monthName() and standaloneDayName().
*/
impl /*struct*/ QLocale {
  pub fn dayName_0<RetType, T: QLocale_dayName_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dayName_0(self);
    // return 1;
  }
}
pub trait QLocale_dayName_0<RetType> {
  fn dayName_0(self , rsthis: & QLocale) -> RetType;
}
impl<'a> /*trait*/ QLocale_dayName_0<usize> for (i32,i32) {
  fn dayName_0(self , rsthis: & QLocale) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLocale7dayNameEiNS_10FormatTypeE", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlocale.h:1026
// index:0
// Public Visibility=Default Availability=Available
// [8] QString standaloneDayName(int, QLocale::FormatType) const

/*
Returns the localized name of the day (where 1 represents Monday, 2 represents Tuesday and so on) that is used as a standalone text, in the format specified by type.

If the locale information does not specify the standalone day name then return value is the same as in dayName().

This function was introduced in  Qt 4.5.

See also dayName() and standaloneMonthName().
*/
impl /*struct*/ QLocale {
  pub fn standaloneDayName_0<RetType, T: QLocale_standaloneDayName_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.standaloneDayName_0(self);
    // return 1;
  }
}
pub trait QLocale_standaloneDayName_0<RetType> {
  fn standaloneDayName_0(self , rsthis: & QLocale) -> RetType;
}
impl<'a> /*trait*/ QLocale_standaloneDayName_0<usize> for (i32,i32) {
  fn standaloneDayName_0(self , rsthis: & QLocale) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLocale17standaloneDayNameEiNS_10FormatTypeE", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlocale.h:1028
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::DayOfWeek firstDayOfWeek() const

/*
Returns the first day of the week according to the current locale.

This function was introduced in  Qt 4.8.
*/
impl /*struct*/ QLocale {
  pub fn firstDayOfWeek_0<RetType, T: QLocale_firstDayOfWeek_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.firstDayOfWeek_0(self);
    // return 1;
  }
}
pub trait QLocale_firstDayOfWeek_0<RetType> {
  fn firstDayOfWeek_0(self , rsthis: & QLocale) -> RetType;
}
impl<'a> /*trait*/ QLocale_firstDayOfWeek_0<i32> for () {
  fn firstDayOfWeek_0(self , rsthis: & QLocale) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLocale14firstDayOfWeekEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlocale.h:1031
// index:0
// Public Visibility=Default Availability=Available
// [8] QString amText() const

/*
Returns the localized name of the "AM" suffix for times specified using the conventions of the 12-hour clock.

This function was introduced in  Qt 4.5.

See also pmText().
*/
impl /*struct*/ QLocale {
  pub fn amText_0<RetType, T: QLocale_amText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.amText_0(self);
    // return 1;
  }
}
pub trait QLocale_amText_0<RetType> {
  fn amText_0(self , rsthis: & QLocale) -> RetType;
}
impl<'a> /*trait*/ QLocale_amText_0<usize> for () {
  fn amText_0(self , rsthis: & QLocale) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLocale6amTextEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlocale.h:1032
// index:0
// Public Visibility=Default Availability=Available
// [8] QString pmText() const

/*
Returns the localized name of the "PM" suffix for times specified using the conventions of the 12-hour clock.

This function was introduced in  Qt 4.5.

See also amText().
*/
impl /*struct*/ QLocale {
  pub fn pmText_0<RetType, T: QLocale_pmText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.pmText_0(self);
    // return 1;
  }
}
pub trait QLocale_pmText_0<RetType> {
  fn pmText_0(self , rsthis: & QLocale) -> RetType;
}
impl<'a> /*trait*/ QLocale_pmText_0<usize> for () {
  fn pmText_0(self , rsthis: & QLocale) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLocale6pmTextEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlocale.h:1034
// index:0
// Public Visibility=Default Availability=Available
// [4] QLocale::MeasurementSystem measurementSystem() const

/*
Returns the measurement system for the locale.

This function was introduced in  Qt 4.4.
*/
impl /*struct*/ QLocale {
  pub fn measurementSystem_0<RetType, T: QLocale_measurementSystem_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.measurementSystem_0(self);
    // return 1;
  }
}
pub trait QLocale_measurementSystem_0<RetType> {
  fn measurementSystem_0(self , rsthis: & QLocale) -> RetType;
}
impl<'a> /*trait*/ QLocale_measurementSystem_0<i32> for () {
  fn measurementSystem_0(self , rsthis: & QLocale) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLocale17measurementSystemEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlocale.h:1036
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::LayoutDirection textDirection() const

/*
Returns the text direction of the language.

This function was introduced in  Qt 4.7.
*/
impl /*struct*/ QLocale {
  pub fn textDirection_0<RetType, T: QLocale_textDirection_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.textDirection_0(self);
    // return 1;
  }
}
pub trait QLocale_textDirection_0<RetType> {
  fn textDirection_0(self , rsthis: & QLocale) -> RetType;
}
impl<'a> /*trait*/ QLocale_textDirection_0<i32> for () {
  fn textDirection_0(self , rsthis: & QLocale) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLocale13textDirectionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlocale.h:1038
// index:0
// Public Visibility=Default Availability=Available
// [8] QString toUpper(const QString &) const

/*
Returns an uppercase copy of str.

If Qt Core is using the ICU libraries, they will be used to perform the transformation according to the rules of the current locale. Otherwise the conversion may be done in a platform-dependent manner, with QString::toUpper() as a generic fallback.

This function was introduced in  Qt 4.8.

See also QString::toUpper().
*/
impl /*struct*/ QLocale {
  pub fn toUpper_0<RetType, T: QLocale_toUpper_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toUpper_0(self);
    // return 1;
  }
}
pub trait QLocale_toUpper_0<RetType> {
  fn toUpper_0(self , rsthis: & QLocale) -> RetType;
}
impl<'a> /*trait*/ QLocale_toUpper_0<usize> for (usize) {
  fn toUpper_0(self , rsthis: & QLocale) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLocale7toUpperERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlocale.h:1039
// index:0
// Public Visibility=Default Availability=Available
// [8] QString toLower(const QString &) const

/*
Returns a lowercase copy of str.

If Qt Core is using the ICU libraries, they will be used to perform the transformation according to the rules of the current locale. Otherwise the conversion may be done in a platform-dependent manner, with QString::toLower() as a generic fallback.

This function was introduced in  Qt 4.8.

See also QString::toLower().
*/
impl /*struct*/ QLocale {
  pub fn toLower_0<RetType, T: QLocale_toLower_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toLower_0(self);
    // return 1;
  }
}
pub trait QLocale_toLower_0<RetType> {
  fn toLower_0(self , rsthis: & QLocale) -> RetType;
}
impl<'a> /*trait*/ QLocale_toLower_0<usize> for (usize) {
  fn toLower_0(self , rsthis: & QLocale) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLocale7toLowerERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlocale.h:1041
// index:0
// Public Visibility=Default Availability=Available
// [8] QString currencySymbol(QLocale::CurrencySymbolFormat) const

/*
Returns a currency symbol according to the format.

This function was introduced in  Qt 4.8.
*/
impl /*struct*/ QLocale {
  pub fn currencySymbol_0<RetType, T: QLocale_currencySymbol_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.currencySymbol_0(self);
    // return 1;
  }
}
pub trait QLocale_currencySymbol_0<RetType> {
  fn currencySymbol_0(self , rsthis: & QLocale) -> RetType;
}
impl<'a> /*trait*/ QLocale_currencySymbol_0<usize> for (i32) {
  fn currencySymbol_0(self , rsthis: & QLocale) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLocale14currencySymbolENS_20CurrencySymbolFormatE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlocale.h:1042
// index:0
// Public Visibility=Default Availability=Available
// [8] QString toCurrencyString(qlonglong, const QString &) const

/*
Returns a localized string representation of value as a currency. If the symbol is provided it is used instead of the default currency symbol.

This function was introduced in  Qt 4.8.

See also currencySymbol().
*/
impl /*struct*/ QLocale {
  pub fn toCurrencyString_0<RetType, T: QLocale_toCurrencyString_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toCurrencyString_0(self);
    // return 1;
  }
}
pub trait QLocale_toCurrencyString_0<RetType> {
  fn toCurrencyString_0(self , rsthis: & QLocale) -> RetType;
}
impl<'a> /*trait*/ QLocale_toCurrencyString_0<usize> for (i64,usize) {
  fn toCurrencyString_0(self , rsthis: & QLocale) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i64 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLocale16toCurrencyStringExRK7QString", 2,qtrt::FFITY_SINT64,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlocale.h:1043
// index:1
// Public Visibility=Default Availability=Available
// [8] QString toCurrencyString(qulonglong, const QString &) const

/*
Returns a localized string representation of value as a currency. If the symbol is provided it is used instead of the default currency symbol.

This function was introduced in  Qt 4.8.

See also currencySymbol().
*/
impl /*struct*/ QLocale {
  pub fn toCurrencyString_1<RetType, T: QLocale_toCurrencyString_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toCurrencyString_1(self);
    // return 1;
  }
}
pub trait QLocale_toCurrencyString_1<RetType> {
  fn toCurrencyString_1(self , rsthis: & QLocale) -> RetType;
}
impl<'a> /*trait*/ QLocale_toCurrencyString_1<usize> for (u64,usize) {
  fn toCurrencyString_1(self , rsthis: & QLocale) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const u64 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLocale16toCurrencyStringEyRK7QString", 2,qtrt::FFITY_UINT64,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlocale.h:1044
// index:2
// Public inline Visibility=Default Availability=Available
// [8] QString toCurrencyString(short, const QString &) const

/*
Returns a localized string representation of value as a currency. If the symbol is provided it is used instead of the default currency symbol.

This function was introduced in  Qt 4.8.

See also currencySymbol().
*/
impl /*struct*/ QLocale {
  pub fn toCurrencyString_2<RetType, T: QLocale_toCurrencyString_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toCurrencyString_2(self);
    // return 1;
  }
}
pub trait QLocale_toCurrencyString_2<RetType> {
  fn toCurrencyString_2(self , rsthis: & QLocale) -> RetType;
}
impl<'a> /*trait*/ QLocale_toCurrencyString_2<usize> for (i16,usize) {
  fn toCurrencyString_2(self , rsthis: & QLocale) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i16 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLocale16toCurrencyStringEsRK7QString", 2,qtrt::FFITY_SINT16,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlocale.h:1045
// index:3
// Public inline Visibility=Default Availability=Available
// [8] QString toCurrencyString(ushort, const QString &) const

/*
Returns a localized string representation of value as a currency. If the symbol is provided it is used instead of the default currency symbol.

This function was introduced in  Qt 4.8.

See also currencySymbol().
*/
impl /*struct*/ QLocale {
  pub fn toCurrencyString_3<RetType, T: QLocale_toCurrencyString_3<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toCurrencyString_3(self);
    // return 1;
  }
}
pub trait QLocale_toCurrencyString_3<RetType> {
  fn toCurrencyString_3(self , rsthis: & QLocale) -> RetType;
}
impl<'a> /*trait*/ QLocale_toCurrencyString_3<usize> for (u16,usize) {
  fn toCurrencyString_3(self , rsthis: & QLocale) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const u16 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLocale16toCurrencyStringEtRK7QString", 2,qtrt::FFITY_UINT16,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlocale.h:1046
// index:4
// Public inline Visibility=Default Availability=Available
// [8] QString toCurrencyString(int, const QString &) const

/*
Returns a localized string representation of value as a currency. If the symbol is provided it is used instead of the default currency symbol.

This function was introduced in  Qt 4.8.

See also currencySymbol().
*/
impl /*struct*/ QLocale {
  pub fn toCurrencyString_4<RetType, T: QLocale_toCurrencyString_4<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toCurrencyString_4(self);
    // return 1;
  }
}
pub trait QLocale_toCurrencyString_4<RetType> {
  fn toCurrencyString_4(self , rsthis: & QLocale) -> RetType;
}
impl<'a> /*trait*/ QLocale_toCurrencyString_4<usize> for (i32,usize) {
  fn toCurrencyString_4(self , rsthis: & QLocale) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLocale16toCurrencyStringEiRK7QString", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlocale.h:1047
// index:5
// Public inline Visibility=Default Availability=Available
// [8] QString toCurrencyString(uint, const QString &) const

/*
Returns a localized string representation of value as a currency. If the symbol is provided it is used instead of the default currency symbol.

This function was introduced in  Qt 4.8.

See also currencySymbol().
*/
impl /*struct*/ QLocale {
  pub fn toCurrencyString_5<RetType, T: QLocale_toCurrencyString_5<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toCurrencyString_5(self);
    // return 1;
  }
}
pub trait QLocale_toCurrencyString_5<RetType> {
  fn toCurrencyString_5(self , rsthis: & QLocale) -> RetType;
}
impl<'a> /*trait*/ QLocale_toCurrencyString_5<usize> for (u32,usize) {
  fn toCurrencyString_5(self , rsthis: & QLocale) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const u32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLocale16toCurrencyStringEjRK7QString", 2,qtrt::FFITY_UINT32,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlocale.h:1053
// index:6
// Public Visibility=Default Availability=Available
// [8] QString toCurrencyString(double, const QString &) const

/*
Returns a localized string representation of value as a currency. If the symbol is provided it is used instead of the default currency symbol.

This function was introduced in  Qt 4.8.

See also currencySymbol().
*/
impl /*struct*/ QLocale {
  pub fn toCurrencyString_6<RetType, T: QLocale_toCurrencyString_6<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toCurrencyString_6(self);
    // return 1;
  }
}
pub trait QLocale_toCurrencyString_6<RetType> {
  fn toCurrencyString_6(self , rsthis: & QLocale) -> RetType;
}
impl<'a> /*trait*/ QLocale_toCurrencyString_6<usize> for (f64,usize) {
  fn toCurrencyString_6(self , rsthis: & QLocale) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLocale16toCurrencyStringEdRK7QString", 2,qtrt::FFITY_DOUBLE,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlocale.h:1054
// index:7
// Public Visibility=Default Availability=Available
// [8] QString toCurrencyString(double, const QString &, int) const

/*
Returns a localized string representation of value as a currency. If the symbol is provided it is used instead of the default currency symbol.

This function was introduced in  Qt 4.8.

See also currencySymbol().
*/
impl /*struct*/ QLocale {
  pub fn toCurrencyString_7<RetType, T: QLocale_toCurrencyString_7<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toCurrencyString_7(self);
    // return 1;
  }
}
pub trait QLocale_toCurrencyString_7<RetType> {
  fn toCurrencyString_7(self , rsthis: & QLocale) -> RetType;
}
impl<'a> /*trait*/ QLocale_toCurrencyString_7<usize> for (f64,usize,i32) {
  fn toCurrencyString_7(self , rsthis: & QLocale) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLocale16toCurrencyStringEdRK7QStringi", 3,qtrt::FFITY_DOUBLE,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlocale.h:1055
// index:8
// Public inline Visibility=Default Availability=Available
// [8] QString toCurrencyString(float, const QString &) const

/*
Returns a localized string representation of value as a currency. If the symbol is provided it is used instead of the default currency symbol.

This function was introduced in  Qt 4.8.

See also currencySymbol().
*/
impl /*struct*/ QLocale {
  pub fn toCurrencyString_8<RetType, T: QLocale_toCurrencyString_8<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toCurrencyString_8(self);
    // return 1;
  }
}
pub trait QLocale_toCurrencyString_8<RetType> {
  fn toCurrencyString_8(self , rsthis: & QLocale) -> RetType;
}
impl<'a> /*trait*/ QLocale_toCurrencyString_8<usize> for (f32,usize) {
  fn toCurrencyString_8(self , rsthis: & QLocale) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLocale16toCurrencyStringEfRK7QString", 2,qtrt::FFITY_FLOAT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlocale.h:1057
// index:9
// Public inline Visibility=Default Availability=Available
// [8] QString toCurrencyString(float, const QString &, int) const

/*
Returns a localized string representation of value as a currency. If the symbol is provided it is used instead of the default currency symbol.

This function was introduced in  Qt 4.8.

See also currencySymbol().
*/
impl /*struct*/ QLocale {
  pub fn toCurrencyString_9<RetType, T: QLocale_toCurrencyString_9<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toCurrencyString_9(self);
    // return 1;
  }
}
pub trait QLocale_toCurrencyString_9<RetType> {
  fn toCurrencyString_9(self , rsthis: & QLocale) -> RetType;
}
impl<'a> /*trait*/ QLocale_toCurrencyString_9<usize> for (f32,usize,i32) {
  fn toCurrencyString_9(self , rsthis: & QLocale) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLocale16toCurrencyStringEfRK7QStringi", 3,qtrt::FFITY_FLOAT,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlocale.h:1061
// index:0
// Public Visibility=Default Availability=Available
// [8] QString formattedDataSize(qint64, int, QLocale::DataSizeFormats)

/*
Converts a size in bytes to a human-readable localized string, comprising a number and a quantified unit. The quantifier is chosen such that the number is at least one, and as small as possible. For example if bytes is 16384, precision is 2, and format is DataSizeIecFormat (the default), this function returns "16.00 KiB"; for 1330409069609 bytes it returns "1.21 GiB"; and so on. If format is DataSizeIecFormat or DataSizeTraditionalFormat, the given number of bytes is divided by a power of 1024, with result less than 1024; for DataSizeSIFormat, it is divided by a power of 1000, with result less than 1000. DataSizeIecFormat uses the new IEC standard quantifiers Ki, Mi and so on, whereas DataSizeSIFormat uses the older SI quantifiers k, M, etc., and DataSizeTraditionalFormat abuses them.

This function was introduced in  Qt 5.10.
*/
impl /*struct*/ QLocale {
  pub fn formattedDataSize_0<RetType, T: QLocale_formattedDataSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.formattedDataSize_0(self);
    // return 1;
  }
}
pub trait QLocale_formattedDataSize_0<RetType> {
  fn formattedDataSize_0(self , rsthis: & QLocale) -> RetType;
}
impl<'a> /*trait*/ QLocale_formattedDataSize_0<usize> for (i64,i32,i32) {
  fn formattedDataSize_0(self , rsthis: & QLocale) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i64 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QLocale17formattedDataSizeExi6QFlagsINS_14DataSizeFormatEE", 3,qtrt::FFITY_SINT64,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlocale.h:1063
// index:0
// Public Visibility=Default Availability=Available
// [8] QStringList uiLanguages() const

/*
Returns an ordered list of locale names for translation purposes in preference order (like "en-Latn-US", "en-US", "en").

The return value represents locale names that the user expects to see the UI translation in.

Most like you do not need to use this function directly, but just pass the QLocale object to the QTranslator::load() function.

The first item in the list is the most preferred one.

This function was introduced in  Qt 4.8.

See also QTranslator and bcp47Name().
*/
impl /*struct*/ QLocale {
  pub fn uiLanguages_0<RetType, T: QLocale_uiLanguages_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.uiLanguages_0(self);
    // return 1;
  }
}
pub trait QLocale_uiLanguages_0<RetType> {
  fn uiLanguages_0(self , rsthis: & QLocale) -> RetType;
}
impl<'a> /*trait*/ QLocale_uiLanguages_0<usize> for () {
  fn uiLanguages_0(self , rsthis: & QLocale) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLocale11uiLanguagesEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlocale.h:1065
// index:0
// Public Visibility=Default Availability=Available
// [1] bool operator==(const QLocale &) const

/*

*/
impl /*struct*/ QLocale {
  pub fn operator_equal_equal_0<RetType, T: QLocale_operator_equal_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_equal_0(self);
    // return 1;
  }
}
pub trait QLocale_operator_equal_equal_0<RetType> {
  fn operator_equal_equal_0(self , rsthis: & QLocale) -> RetType;
}
impl<'a> /*trait*/ QLocale_operator_equal_equal_0<bool> for (usize) {
  fn operator_equal_equal_0(self , rsthis: & QLocale) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLocaleeqERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlocale.h:1066
// index:0
// Public Visibility=Default Availability=Available
// [1] bool operator!=(const QLocale &) const

/*

*/
impl /*struct*/ QLocale {
  pub fn operator_not_equal_0<RetType, T: QLocale_operator_not_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_not_equal_0(self);
    // return 1;
  }
}
pub trait QLocale_operator_not_equal_0<RetType> {
  fn operator_not_equal_0(self , rsthis: & QLocale) -> RetType;
}
impl<'a> /*trait*/ QLocale_operator_not_equal_0<bool> for (usize) {
  fn operator_not_equal_0(self , rsthis: & QLocale) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLocaleneERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlocale.h:1068
// index:0
// Public static Visibility=Default Availability=Available
// [8] QString languageToString(QLocale::Language)

/*
Returns a QString containing the name of language.

See also countryToString(), scriptToString(), and bcp47Name().
*/
impl /*struct*/ QLocale {
  pub fn languageToString_0<RetType, T: QLocale_languageToString_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.languageToString_0();
    // return 1;
  }
}
pub trait QLocale_languageToString_0<RetType> {
  fn languageToString_0(self ) -> RetType;
}
impl<'a> /*trait*/ QLocale_languageToString_0<usize> for (i32) {
  fn languageToString_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QLocale16languageToStringENS_8LanguageE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlocale.h:1069
// index:0
// Public static Visibility=Default Availability=Available
// [8] QString countryToString(QLocale::Country)

/*
Returns a QString containing the name of country.

See also languageToString(), scriptToString(), country(), and bcp47Name().
*/
impl /*struct*/ QLocale {
  pub fn countryToString_0<RetType, T: QLocale_countryToString_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.countryToString_0();
    // return 1;
  }
}
pub trait QLocale_countryToString_0<RetType> {
  fn countryToString_0(self ) -> RetType;
}
impl<'a> /*trait*/ QLocale_countryToString_0<usize> for (i32) {
  fn countryToString_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QLocale15countryToStringENS_7CountryE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlocale.h:1070
// index:0
// Public static Visibility=Default Availability=Available
// [8] QString scriptToString(QLocale::Script)

/*
Returns a QString containing the name of script.

This function was introduced in  Qt 4.8.

See also languageToString(), countryToString(), script(), and bcp47Name().
*/
impl /*struct*/ QLocale {
  pub fn scriptToString_0<RetType, T: QLocale_scriptToString_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.scriptToString_0();
    // return 1;
  }
}
pub trait QLocale_scriptToString_0<RetType> {
  fn scriptToString_0(self ) -> RetType;
}
impl<'a> /*trait*/ QLocale_scriptToString_0<usize> for (i32) {
  fn scriptToString_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QLocale14scriptToStringENS_6ScriptE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlocale.h:1071
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void setDefault(const QLocale &)

/*
Sets the global default locale to locale. These values are used when a QLocale object is constructed with no arguments. If this function is not called, the system's locale is used.

Warning: In a multithreaded application, the default locale should be set at application startup, before any non-GUI threads are created.

Warning: This function is not reentrant.

See also system() and c().
*/
impl /*struct*/ QLocale {
  pub fn setDefault_0<RetType, T: QLocale_setDefault_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.setDefault_0();
    // return 1;
  }
}
pub trait QLocale_setDefault_0<RetType> {
  fn setDefault_0(self ) -> RetType;
}
impl<'a> /*trait*/ QLocale_setDefault_0<(/*void*/)> for (usize) {
  fn setDefault_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QLocale10setDefaultERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qlocale.h:1073
// index:0
// Public static inline Visibility=Default Availability=Available
// [8] QLocale c()

/*
Returns a QLocale object initialized to the "C" locale.

See also system().
*/
impl /*struct*/ QLocale {
  pub fn c_0<RetType, T: QLocale_c_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.c_0();
    // return 1;
  }
}
pub trait QLocale_c_0<RetType> {
  fn c_0(self ) -> RetType;
}
impl<'a> /*trait*/ QLocale_c_0<usize> for () {
  fn c_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QLocale1cEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlocale.h:1074
// index:0
// Public static Visibility=Default Availability=Available
// [8] QLocale system()

/*
Returns a QLocale object initialized to the system locale.

On Windows and Mac, this locale will use the decimal/grouping characters and date/time formats specified in the system configuration panel.

See also c().
*/
impl /*struct*/ QLocale {
  pub fn system_0<RetType, T: QLocale_system_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.system_0();
    // return 1;
  }
}
pub trait QLocale_system_0<RetType> {
  fn system_0(self ) -> RetType;
}
impl<'a> /*trait*/ QLocale_system_0<usize> for () {
  fn system_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QLocale6systemEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlocale.h:1079
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setNumberOptions(QLocale::NumberOptions)

/*
Sets the options related to number conversions for this QLocale instance.

This function was introduced in  Qt 4.2.

See also numberOptions().
*/
impl /*struct*/ QLocale {
  pub fn setNumberOptions_0<RetType, T: QLocale_setNumberOptions_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setNumberOptions_0(self);
    // return 1;
  }
}
pub trait QLocale_setNumberOptions_0<RetType> {
  fn setNumberOptions_0(self , rsthis: & QLocale) -> RetType;
}
impl<'a> /*trait*/ QLocale_setNumberOptions_0<(/*void*/)> for (i32) {
  fn setNumberOptions_0(self , rsthis: & QLocale) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QLocale16setNumberOptionsE6QFlagsINS_12NumberOptionEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qlocale.h:1080
// index:0
// Public Visibility=Default Availability=Available
// [4] QLocale::NumberOptions numberOptions() const

/*
Returns the options related to number conversions for this QLocale instance.

By default, no options are set for the standard locales.

This function was introduced in  Qt 4.2.

See also setNumberOptions().
*/
impl /*struct*/ QLocale {
  pub fn numberOptions_0<RetType, T: QLocale_numberOptions_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.numberOptions_0(self);
    // return 1;
  }
}
pub trait QLocale_numberOptions_0<RetType> {
  fn numberOptions_0(self , rsthis: & QLocale) -> RetType;
}
impl<'a> /*trait*/ QLocale_numberOptions_0<i32> for () {
  fn numberOptions_0(self , rsthis: & QLocale) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLocale13numberOptionsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlocale.h:1083
// index:0
// Public Visibility=Default Availability=Available
// [8] QString quoteString(const QString &, QLocale::QuotationStyle) const

/*
Returns str quoted according to the current locale using the given quotation style.

This function was introduced in  Qt 4.8.
*/
impl /*struct*/ QLocale {
  pub fn quoteString_0<RetType, T: QLocale_quoteString_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.quoteString_0(self);
    // return 1;
  }
}
pub trait QLocale_quoteString_0<RetType> {
  fn quoteString_0(self , rsthis: & QLocale) -> RetType;
}
impl<'a> /*trait*/ QLocale_quoteString_0<usize> for (usize,i32) {
  fn quoteString_0(self , rsthis: & QLocale) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLocale11quoteStringERK7QStringNS_14QuotationStyleE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlocale.h:1084
// index:1
// Public Visibility=Default Availability=Available
// [8] QString quoteString(const QStringRef &, QLocale::QuotationStyle) const

/*
Returns str quoted according to the current locale using the given quotation style.

This function was introduced in  Qt 4.8.
*/
impl /*struct*/ QLocale {
  pub fn quoteString_1<RetType, T: QLocale_quoteString_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.quoteString_1(self);
    // return 1;
  }
}
pub trait QLocale_quoteString_1<RetType> {
  fn quoteString_1(self , rsthis: & QLocale) -> RetType;
}
impl<'a> /*trait*/ QLocale_quoteString_1<usize> for (usize,i32) {
  fn quoteString_1(self , rsthis: & QLocale) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLocale11quoteStringERK10QStringRefNS_14QuotationStyleE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlocale.h:1086
// index:0
// Public Visibility=Default Availability=Available
// [8] QString createSeparatedList(const QStringList &) const

/*
Returns a string that represents a join of a given list of strings with a separator defined by the locale.

This function was introduced in  Qt 4.8.
*/
impl /*struct*/ QLocale {
  pub fn createSeparatedList_0<RetType, T: QLocale_createSeparatedList_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.createSeparatedList_0(self);
    // return 1;
  }
}
pub trait QLocale_createSeparatedList_0<RetType> {
  fn createSeparatedList_0(self , rsthis: & QLocale) -> RetType;
}
impl<'a> /*trait*/ QLocale_createSeparatedList_0<usize> for (usize) {
  fn createSeparatedList_0(self , rsthis: & QLocale) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QLocale19createSeparatedListERK11QStringList", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}


/*
This enumerated type is used to specify a language.

QLocale::AfanOromoObsolete, please use Oromo
QLocale::BhutaniDzongkhaObsolete, please use Dzongkha
QLocale::ByelorussianBelarusianObsolete, please use Belarusian
QLocale::CambodianKhmerObsolete, please use Khmer
QLocale::FrisianWesternFrisiansame as WesternFrisian
QLocale::KurundiRundiObsolete, please use Rundi
QLocale::MoldavianRomanianObsolete, please use Romanian
QLocale::NorwegianNorwegianBokmalsame as NorwegianBokmal
QLocale::RhaetoRomanceRomanshObsolete, please use Romansh
QLocale::SerboCroatianSerbianObsolete, please use Serbian
QLocale::TagalogFilipinoObsolete, please use Filipino
QLocale::TwiAkanObsolete, please use Akan
QLocale::UigurUighurObsolete, please use Uighur
QLocale::ChewaNyanjaObsolete, please use Nyanja


See also language() and languageToString().

*/
pub type QLocale__Language = i32;
//  
pub const QLocale__AnyLanguage :QLocale__Language = 0;
// The "C" locale is identical in behavior to English/UnitedStates.
pub const QLocale__C :QLocale__Language = 1;
//  
pub const QLocale__Abkhazian :QLocale__Language = 2;
//  
pub const QLocale__Oromo :QLocale__Language = 3;
//  
pub const QLocale__Afar :QLocale__Language = 4;
//  
pub const QLocale__Afrikaans :QLocale__Language = 5;
//  
pub const QLocale__Albanian :QLocale__Language = 6;
//  
pub const QLocale__Amharic :QLocale__Language = 7;
//  
pub const QLocale__Arabic :QLocale__Language = 8;
//  
pub const QLocale__Armenian :QLocale__Language = 9;
// 
pub const QLocale__Assamese :QLocale__Language = 10;
// 
pub const QLocale__Aymara :QLocale__Language = 11;
// 
pub const QLocale__Azerbaijani :QLocale__Language = 12;
// 
pub const QLocale__Bashkir :QLocale__Language = 13;
// 
pub const QLocale__Basque :QLocale__Language = 14;
// 
pub const QLocale__Bengali :QLocale__Language = 15;
// 
pub const QLocale__Dzongkha :QLocale__Language = 16;
// 
pub const QLocale__Bihari :QLocale__Language = 17;
// 
pub const QLocale__Bislama :QLocale__Language = 18;
// 
pub const QLocale__Breton :QLocale__Language = 19;
// 
pub const QLocale__Bulgarian :QLocale__Language = 20;
// 
pub const QLocale__Burmese :QLocale__Language = 21;
// 
pub const QLocale__Belarusian :QLocale__Language = 22;
// 
pub const QLocale__Khmer :QLocale__Language = 23;
// 
pub const QLocale__Catalan :QLocale__Language = 24;
// 
pub const QLocale__Chinese :QLocale__Language = 25;
// 
pub const QLocale__Corsican :QLocale__Language = 26;
// 
pub const QLocale__Croatian :QLocale__Language = 27;
// 
pub const QLocale__Czech :QLocale__Language = 28;
// 
pub const QLocale__Danish :QLocale__Language = 29;
// 
pub const QLocale__Dutch :QLocale__Language = 30;
// 
pub const QLocale__English :QLocale__Language = 31;
// 
pub const QLocale__Esperanto :QLocale__Language = 32;
// 
pub const QLocale__Estonian :QLocale__Language = 33;
// 
pub const QLocale__Faroese :QLocale__Language = 34;
// 
pub const QLocale__Fijian :QLocale__Language = 35;
// 
pub const QLocale__Finnish :QLocale__Language = 36;
// 
pub const QLocale__French :QLocale__Language = 37;
// 
pub const QLocale__WesternFrisian :QLocale__Language = 38;
// 
pub const QLocale__Gaelic :QLocale__Language = 39;
// 
pub const QLocale__Galician :QLocale__Language = 40;
// 
pub const QLocale__Georgian :QLocale__Language = 41;
// 
pub const QLocale__German :QLocale__Language = 42;
// 
pub const QLocale__Greek :QLocale__Language = 43;
// 
pub const QLocale__Greenlandic :QLocale__Language = 44;
// 
pub const QLocale__Guarani :QLocale__Language = 45;
// 
pub const QLocale__Gujarati :QLocale__Language = 46;
// 
pub const QLocale__Hausa :QLocale__Language = 47;
// 
pub const QLocale__Hebrew :QLocale__Language = 48;
// 
pub const QLocale__Hindi :QLocale__Language = 49;
// 
pub const QLocale__Hungarian :QLocale__Language = 50;
// 
pub const QLocale__Icelandic :QLocale__Language = 51;
// 
pub const QLocale__Indonesian :QLocale__Language = 52;
// 
pub const QLocale__Interlingua :QLocale__Language = 53;
// 
pub const QLocale__Interlingue :QLocale__Language = 54;
// 
pub const QLocale__Inuktitut :QLocale__Language = 55;
// 
pub const QLocale__Inupiak :QLocale__Language = 56;
// 
pub const QLocale__Irish :QLocale__Language = 57;
// 
pub const QLocale__Italian :QLocale__Language = 58;
// 
pub const QLocale__Japanese :QLocale__Language = 59;
// 
pub const QLocale__Javanese :QLocale__Language = 60;
// 
pub const QLocale__Kannada :QLocale__Language = 61;
// 
pub const QLocale__Kashmiri :QLocale__Language = 62;
// 
pub const QLocale__Kazakh :QLocale__Language = 63;
// 
pub const QLocale__Kinyarwanda :QLocale__Language = 64;
// 
pub const QLocale__Kirghiz :QLocale__Language = 65;
// 
pub const QLocale__Korean :QLocale__Language = 66;
// 
pub const QLocale__Kurdish :QLocale__Language = 67;
// 
pub const QLocale__Rundi :QLocale__Language = 68;
// 
pub const QLocale__Lao :QLocale__Language = 69;
// 
pub const QLocale__Latin :QLocale__Language = 70;
// 
pub const QLocale__Latvian :QLocale__Language = 71;
// 
pub const QLocale__Lingala :QLocale__Language = 72;
// 
pub const QLocale__Lithuanian :QLocale__Language = 73;
// 
pub const QLocale__Macedonian :QLocale__Language = 74;
// 
pub const QLocale__Malagasy :QLocale__Language = 75;
// 
pub const QLocale__Malay :QLocale__Language = 76;
// 
pub const QLocale__Malayalam :QLocale__Language = 77;
// 
pub const QLocale__Maltese :QLocale__Language = 78;
// 
pub const QLocale__Maori :QLocale__Language = 79;
// 
pub const QLocale__Marathi :QLocale__Language = 80;
// 
pub const QLocale__Marshallese :QLocale__Language = 81;
// 
pub const QLocale__Mongolian :QLocale__Language = 82;
// 
pub const QLocale__NauruLanguage :QLocale__Language = 83;
// 
pub const QLocale__Nepali :QLocale__Language = 84;
// 
pub const QLocale__NorwegianBokmal :QLocale__Language = 85;
// 
pub const QLocale__Occitan :QLocale__Language = 86;
// 
pub const QLocale__Oriya :QLocale__Language = 87;
// 
pub const QLocale__Pashto :QLocale__Language = 88;
// 
pub const QLocale__Persian :QLocale__Language = 89;
// 
pub const QLocale__Polish :QLocale__Language = 90;
// 
pub const QLocale__Portuguese :QLocale__Language = 91;
// 
pub const QLocale__Punjabi :QLocale__Language = 92;
// 
pub const QLocale__Quechua :QLocale__Language = 93;
// 
pub const QLocale__Romansh :QLocale__Language = 94;
// 
pub const QLocale__Romanian :QLocale__Language = 95;
// 
pub const QLocale__Russian :QLocale__Language = 96;
// 
pub const QLocale__Samoan :QLocale__Language = 97;
// 
pub const QLocale__Sango :QLocale__Language = 98;
// 
pub const QLocale__Sanskrit :QLocale__Language = 99;
// 
pub const QLocale__Serbian :QLocale__Language = 100;
// 
pub const QLocale__Ossetic :QLocale__Language = 101;
// 
pub const QLocale__SouthernSotho :QLocale__Language = 102;
// 
pub const QLocale__Tswana :QLocale__Language = 103;
// 
pub const QLocale__Shona :QLocale__Language = 104;
// 
pub const QLocale__Sindhi :QLocale__Language = 105;
// 
pub const QLocale__Sinhala :QLocale__Language = 106;
// 
pub const QLocale__Swati :QLocale__Language = 107;
// 
pub const QLocale__Slovak :QLocale__Language = 108;
// 
pub const QLocale__Slovenian :QLocale__Language = 109;
// 
pub const QLocale__Somali :QLocale__Language = 110;
// 
pub const QLocale__Spanish :QLocale__Language = 111;
// 
pub const QLocale__Sundanese :QLocale__Language = 112;
// 
pub const QLocale__Swahili :QLocale__Language = 113;
// 
pub const QLocale__Swedish :QLocale__Language = 114;
// 
pub const QLocale__Sardinian :QLocale__Language = 115;
// 
pub const QLocale__Tajik :QLocale__Language = 116;
// 
pub const QLocale__Tamil :QLocale__Language = 117;
// 
pub const QLocale__Tatar :QLocale__Language = 118;
// 
pub const QLocale__Telugu :QLocale__Language = 119;
// 
pub const QLocale__Thai :QLocale__Language = 120;
// 
pub const QLocale__Tibetan :QLocale__Language = 121;
// 
pub const QLocale__Tigrinya :QLocale__Language = 122;
// 
pub const QLocale__Tongan :QLocale__Language = 123;
// 
pub const QLocale__Tsonga :QLocale__Language = 124;
// 
pub const QLocale__Turkish :QLocale__Language = 125;
// 
pub const QLocale__Turkmen :QLocale__Language = 126;
// 
pub const QLocale__Tahitian :QLocale__Language = 127;
// 
pub const QLocale__Uighur :QLocale__Language = 128;
// 
pub const QLocale__Ukrainian :QLocale__Language = 129;
// 
pub const QLocale__Urdu :QLocale__Language = 130;
// 
pub const QLocale__Uzbek :QLocale__Language = 131;
// 
pub const QLocale__Vietnamese :QLocale__Language = 132;
// 
pub const QLocale__Volapuk :QLocale__Language = 133;
// 
pub const QLocale__Welsh :QLocale__Language = 134;
// 
pub const QLocale__Wolof :QLocale__Language = 135;
// 
pub const QLocale__Xhosa :QLocale__Language = 136;
// 
pub const QLocale__Yiddish :QLocale__Language = 137;
// 
pub const QLocale__Yoruba :QLocale__Language = 138;
// 
pub const QLocale__Zhuang :QLocale__Language = 139;
// 
pub const QLocale__Zulu :QLocale__Language = 140;
// 
pub const QLocale__NorwegianNynorsk :QLocale__Language = 141;
// 
pub const QLocale__Bosnian :QLocale__Language = 142;
// 
pub const QLocale__Divehi :QLocale__Language = 143;
// 
pub const QLocale__Manx :QLocale__Language = 144;
// 
pub const QLocale__Cornish :QLocale__Language = 145;
// 
pub const QLocale__Akan :QLocale__Language = 146;
// 
pub const QLocale__Konkani :QLocale__Language = 147;
// 
pub const QLocale__Ga :QLocale__Language = 148;
// 
pub const QLocale__Igbo :QLocale__Language = 149;
// 
pub const QLocale__Kamba :QLocale__Language = 150;
// 
pub const QLocale__Syriac :QLocale__Language = 151;
// 
pub const QLocale__Blin :QLocale__Language = 152;
// 
pub const QLocale__Geez :QLocale__Language = 153;
// 
pub const QLocale__Koro :QLocale__Language = 154;
// 
pub const QLocale__Sidamo :QLocale__Language = 155;
// 
pub const QLocale__Atsam :QLocale__Language = 156;
// 
pub const QLocale__Tigre :QLocale__Language = 157;
// 
pub const QLocale__Jju :QLocale__Language = 158;
// 
pub const QLocale__Friulian :QLocale__Language = 159;
// 
pub const QLocale__Venda :QLocale__Language = 160;
// 
pub const QLocale__Ewe :QLocale__Language = 161;
// 
pub const QLocale__Walamo :QLocale__Language = 162;
// 
pub const QLocale__Hawaiian :QLocale__Language = 163;
// 
pub const QLocale__Tyap :QLocale__Language = 164;
// 
pub const QLocale__Nyanja :QLocale__Language = 165;
// 
pub const QLocale__Filipino :QLocale__Language = 166;
// 
pub const QLocale__SwissGerman :QLocale__Language = 167;
// 
pub const QLocale__SichuanYi :QLocale__Language = 168;
// 
pub const QLocale__Kpelle :QLocale__Language = 169;
// 
pub const QLocale__LowGerman :QLocale__Language = 170;
// 
pub const QLocale__SouthNdebele :QLocale__Language = 171;
// 
pub const QLocale__NorthernSotho :QLocale__Language = 172;
// 
pub const QLocale__NorthernSami :QLocale__Language = 173;
// 
pub const QLocale__Taroko :QLocale__Language = 174;
// 
pub const QLocale__Gusii :QLocale__Language = 175;
// 
pub const QLocale__Taita :QLocale__Language = 176;
// 
pub const QLocale__Fulah :QLocale__Language = 177;
// 
pub const QLocale__Kikuyu :QLocale__Language = 178;
// 
pub const QLocale__Samburu :QLocale__Language = 179;
// 
pub const QLocale__Sena :QLocale__Language = 180;
// 
pub const QLocale__NorthNdebele :QLocale__Language = 181;
// 
pub const QLocale__Rombo :QLocale__Language = 182;
// 
pub const QLocale__Tachelhit :QLocale__Language = 183;
// 
pub const QLocale__Kabyle :QLocale__Language = 184;
// 
pub const QLocale__Nyankole :QLocale__Language = 185;
// 
pub const QLocale__Bena :QLocale__Language = 186;
// 
pub const QLocale__Vunjo :QLocale__Language = 187;
// 
pub const QLocale__Bambara :QLocale__Language = 188;
// 
pub const QLocale__Embu :QLocale__Language = 189;
// 
pub const QLocale__Cherokee :QLocale__Language = 190;
// 
pub const QLocale__Morisyen :QLocale__Language = 191;
// 
pub const QLocale__Makonde :QLocale__Language = 192;
// 
pub const QLocale__Langi :QLocale__Language = 193;
// 
pub const QLocale__Ganda :QLocale__Language = 194;
// 
pub const QLocale__Bemba :QLocale__Language = 195;
// 
pub const QLocale__Kabuverdianu :QLocale__Language = 196;
// 
pub const QLocale__Meru :QLocale__Language = 197;
// 
pub const QLocale__Kalenjin :QLocale__Language = 198;
// 
pub const QLocale__Nama :QLocale__Language = 199;
// 
pub const QLocale__Machame :QLocale__Language = 200;
// 
pub const QLocale__Colognian :QLocale__Language = 201;
// 
pub const QLocale__Masai :QLocale__Language = 202;
// 
pub const QLocale__Soga :QLocale__Language = 203;
// 
pub const QLocale__Luyia :QLocale__Language = 204;
// 
pub const QLocale__Asu :QLocale__Language = 205;
// 
pub const QLocale__Teso :QLocale__Language = 206;
// 
pub const QLocale__Saho :QLocale__Language = 207;
// 
pub const QLocale__KoyraChiini :QLocale__Language = 208;
// 
pub const QLocale__Rwa :QLocale__Language = 209;
// 
pub const QLocale__Luo :QLocale__Language = 210;
// 
pub const QLocale__Chiga :QLocale__Language = 211;
// 
pub const QLocale__CentralMoroccoTamazight :QLocale__Language = 212;
// 
pub const QLocale__KoyraboroSenni :QLocale__Language = 213;
// 
pub const QLocale__Shambala :QLocale__Language = 214;
// 
pub const QLocale__Bodo :QLocale__Language = 215;
// 
pub const QLocale__Avaric :QLocale__Language = 216;
// 
pub const QLocale__Chamorro :QLocale__Language = 217;
// 
pub const QLocale__Chechen :QLocale__Language = 218;
// 
pub const QLocale__Church :QLocale__Language = 219;
// 
pub const QLocale__Chuvash :QLocale__Language = 220;
// 
pub const QLocale__Cree :QLocale__Language = 221;
// 
pub const QLocale__Haitian :QLocale__Language = 222;
// 
pub const QLocale__Herero :QLocale__Language = 223;
// 
pub const QLocale__HiriMotu :QLocale__Language = 224;
// 
pub const QLocale__Kanuri :QLocale__Language = 225;
// 
pub const QLocale__Komi :QLocale__Language = 226;
// 
pub const QLocale__Kongo :QLocale__Language = 227;
// 
pub const QLocale__Kwanyama :QLocale__Language = 228;
// 
pub const QLocale__Limburgish :QLocale__Language = 229;
// 
pub const QLocale__LubaKatanga :QLocale__Language = 230;
// 
pub const QLocale__Luxembourgish :QLocale__Language = 231;
// 
pub const QLocale__Navaho :QLocale__Language = 232;
// 
pub const QLocale__Ndonga :QLocale__Language = 233;
// 
pub const QLocale__Ojibwa :QLocale__Language = 234;
// 
pub const QLocale__Pali :QLocale__Language = 235;
// 
pub const QLocale__Walloon :QLocale__Language = 236;
// 
pub const QLocale__Aghem :QLocale__Language = 237;
// 
pub const QLocale__Basaa :QLocale__Language = 238;
// 
pub const QLocale__Zarma :QLocale__Language = 239;
// 
pub const QLocale__Duala :QLocale__Language = 240;
// 
pub const QLocale__JolaFonyi :QLocale__Language = 241;
// 
pub const QLocale__Ewondo :QLocale__Language = 242;
// 
pub const QLocale__Bafia :QLocale__Language = 243;
// 
pub const QLocale__MakhuwaMeetto :QLocale__Language = 244;
// 
pub const QLocale__Mundang :QLocale__Language = 245;
// 
pub const QLocale__Kwasio :QLocale__Language = 246;
// 
pub const QLocale__Nuer :QLocale__Language = 247;
// 
pub const QLocale__Sakha :QLocale__Language = 248;
// 
pub const QLocale__Sangu :QLocale__Language = 249;
// 
pub const QLocale__CongoSwahili :QLocale__Language = 250;
// 
pub const QLocale__Tasawaq :QLocale__Language = 251;
// 
pub const QLocale__Vai :QLocale__Language = 252;
// 
pub const QLocale__Walser :QLocale__Language = 253;
// 
pub const QLocale__Yangben :QLocale__Language = 254;
// 
pub const QLocale__Avestan :QLocale__Language = 255;
// 
pub const QLocale__Asturian :QLocale__Language = 256;
// 
pub const QLocale__Ngomba :QLocale__Language = 257;
// 
pub const QLocale__Kako :QLocale__Language = 258;
// 
pub const QLocale__Meta :QLocale__Language = 259;
// 
pub const QLocale__Ngiemboon :QLocale__Language = 260;
// 
pub const QLocale__Aragonese :QLocale__Language = 261;
// 
pub const QLocale__Akkadian :QLocale__Language = 262;
// 
pub const QLocale__AncientEgyptian :QLocale__Language = 263;
// 
pub const QLocale__AncientGreek :QLocale__Language = 264;
// 
pub const QLocale__Aramaic :QLocale__Language = 265;
// 
pub const QLocale__Balinese :QLocale__Language = 266;
// 
pub const QLocale__Bamun :QLocale__Language = 267;
// 
pub const QLocale__BatakToba :QLocale__Language = 268;
// 
pub const QLocale__Buginese :QLocale__Language = 269;
// 
pub const QLocale__Buhid :QLocale__Language = 270;
// 
pub const QLocale__Carian :QLocale__Language = 271;
// 
pub const QLocale__Chakma :QLocale__Language = 272;
// 
pub const QLocale__ClassicalMandaic :QLocale__Language = 273;
// 
pub const QLocale__Coptic :QLocale__Language = 274;
// 
pub const QLocale__Dogri :QLocale__Language = 275;
// 
pub const QLocale__EasternCham :QLocale__Language = 276;
// 
pub const QLocale__EasternKayah :QLocale__Language = 277;
// 
pub const QLocale__Etruscan :QLocale__Language = 278;
// 
pub const QLocale__Gothic :QLocale__Language = 279;
// 
pub const QLocale__Hanunoo :QLocale__Language = 280;
// 
pub const QLocale__Ingush :QLocale__Language = 281;
// 
pub const QLocale__LargeFloweryMiao :QLocale__Language = 282;
// 
pub const QLocale__Lepcha :QLocale__Language = 283;
// 
pub const QLocale__Limbu :QLocale__Language = 284;
// 
pub const QLocale__Lisu :QLocale__Language = 285;
// 
pub const QLocale__Lu :QLocale__Language = 286;
// 
pub const QLocale__Lycian :QLocale__Language = 287;
// 
pub const QLocale__Lydian :QLocale__Language = 288;
// 
pub const QLocale__Mandingo :QLocale__Language = 289;
// 
pub const QLocale__Manipuri :QLocale__Language = 290;
// 
pub const QLocale__Meroitic :QLocale__Language = 291;
// 
pub const QLocale__NorthernThai :QLocale__Language = 292;
// 
pub const QLocale__OldIrish :QLocale__Language = 293;
// 
pub const QLocale__OldNorse :QLocale__Language = 294;
// 
pub const QLocale__OldPersian :QLocale__Language = 295;
// 
pub const QLocale__OldTurkish :QLocale__Language = 296;
// 
pub const QLocale__Pahlavi :QLocale__Language = 297;
// 
pub const QLocale__Parthian :QLocale__Language = 298;
// 
pub const QLocale__Phoenician :QLocale__Language = 299;
// 
pub const QLocale__PrakritLanguage :QLocale__Language = 300;
// 
pub const QLocale__Rejang :QLocale__Language = 301;
// 
pub const QLocale__Sabaean :QLocale__Language = 302;
// 
pub const QLocale__Samaritan :QLocale__Language = 303;
// 
pub const QLocale__Santali :QLocale__Language = 304;
// 
pub const QLocale__Saurashtra :QLocale__Language = 305;
// 
pub const QLocale__Sora :QLocale__Language = 306;
// 
pub const QLocale__Sylheti :QLocale__Language = 307;
// 
pub const QLocale__Tagbanwa :QLocale__Language = 308;
// 
pub const QLocale__TaiDam :QLocale__Language = 309;
// 
pub const QLocale__TaiNua :QLocale__Language = 310;
// 
pub const QLocale__Ugaritic :QLocale__Language = 311;
// 
pub const QLocale__Akoose :QLocale__Language = 312;
// 
pub const QLocale__Lakota :QLocale__Language = 313;
// 
pub const QLocale__StandardMoroccanTamazight :QLocale__Language = 314;
// 
pub const QLocale__Mapuche :QLocale__Language = 315;
// 
pub const QLocale__CentralKurdish :QLocale__Language = 316;
// 
pub const QLocale__LowerSorbian :QLocale__Language = 317;
// 
pub const QLocale__UpperSorbian :QLocale__Language = 318;
// 
pub const QLocale__Kenyang :QLocale__Language = 319;
// 
pub const QLocale__Mohawk :QLocale__Language = 320;
// 
pub const QLocale__Nko :QLocale__Language = 321;
// 
pub const QLocale__Prussian :QLocale__Language = 322;
// 
pub const QLocale__Kiche :QLocale__Language = 323;
// 
pub const QLocale__SouthernSami :QLocale__Language = 324;
// 
pub const QLocale__LuleSami :QLocale__Language = 325;
// 
pub const QLocale__InariSami :QLocale__Language = 326;
// 
pub const QLocale__SkoltSami :QLocale__Language = 327;
// 
pub const QLocale__Warlpiri :QLocale__Language = 328;
// 
pub const QLocale__ManichaeanMiddlePersian :QLocale__Language = 329;
// 
pub const QLocale__Mende :QLocale__Language = 330;
// 
pub const QLocale__AncientNorthArabian :QLocale__Language = 331;
// 
pub const QLocale__LinearA :QLocale__Language = 332;
// 
pub const QLocale__HmongNjua :QLocale__Language = 333;
// 
pub const QLocale__Ho :QLocale__Language = 334;
// 
pub const QLocale__Lezghian :QLocale__Language = 335;
// 
pub const QLocale__Bassa :QLocale__Language = 336;
// 
pub const QLocale__Mono :QLocale__Language = 337;
// 
pub const QLocale__TedimChin :QLocale__Language = 338;
// 
pub const QLocale__Maithili :QLocale__Language = 339;
// 
pub const QLocale__Ahom :QLocale__Language = 340;
// 
pub const QLocale__AmericanSignLanguage :QLocale__Language = 341;
// 
pub const QLocale__ArdhamagadhiPrakrit :QLocale__Language = 342;
// 
pub const QLocale__Bhojpuri :QLocale__Language = 343;
// 
pub const QLocale__HieroglyphicLuwian :QLocale__Language = 344;
// 
pub const QLocale__LiteraryChinese :QLocale__Language = 345;
// 
pub const QLocale__Mazanderani :QLocale__Language = 346;
// 
pub const QLocale__Mru :QLocale__Language = 347;
// 
pub const QLocale__Newari :QLocale__Language = 348;
// 
pub const QLocale__NorthernLuri :QLocale__Language = 349;
// 
pub const QLocale__Palauan :QLocale__Language = 350;
// 
pub const QLocale__Papiamento :QLocale__Language = 351;
// 
pub const QLocale__Saraiki :QLocale__Language = 352;
// 
pub const QLocale__TokelauLanguage :QLocale__Language = 353;
// 
pub const QLocale__TokPisin :QLocale__Language = 354;
// 
pub const QLocale__TuvaluLanguage :QLocale__Language = 355;
// 
pub const QLocale__UncodedLanguages :QLocale__Language = 356;
// 
pub const QLocale__Cantonese :QLocale__Language = 357;
// 
pub const QLocale__Osage :QLocale__Language = 358;
// 
pub const QLocale__Tangut :QLocale__Language = 359;
// 
pub const QLocale__Norwegian :QLocale__Language = 85;
// 
pub const QLocale__Moldavian :QLocale__Language = 95;
// 
pub const QLocale__SerboCroatian :QLocale__Language = 100;
// 
pub const QLocale__Tagalog :QLocale__Language = 166;
// 
pub const QLocale__Twi :QLocale__Language = 146;
// 
pub const QLocale__Afan :QLocale__Language = 3;
// 
pub const QLocale__Byelorussian :QLocale__Language = 22;
// 
pub const QLocale__Bhutani :QLocale__Language = 16;
// 
pub const QLocale__Cambodian :QLocale__Language = 23;
// 
pub const QLocale__Kurundi :QLocale__Language = 68;
// 
pub const QLocale__RhaetoRomance :QLocale__Language = 94;
// 
pub const QLocale__Chewa :QLocale__Language = 165;
// 
pub const QLocale__Frisian :QLocale__Language = 38;
// 
pub const QLocale__Uigur :QLocale__Language = 128;
// 
pub const QLocale__LastLanguage :QLocale__Language = 359;
pub fn QLocale_LanguageItemName(val: i32) ->String {
  match val {
     QLocale__AnyLanguage => // 0
     {return String::from("AnyLanguage");}
     QLocale__C => // 1
     {return String::from("C");}
     QLocale__Abkhazian => // 2
     {return String::from("Abkhazian");}
     QLocale__Oromo => // 3
     {return String::from("Oromo,Afan");}
     QLocale__Afar => // 4
     {return String::from("Afar");}
     QLocale__Afrikaans => // 5
     {return String::from("Afrikaans");}
     QLocale__Albanian => // 6
     {return String::from("Albanian");}
     QLocale__Amharic => // 7
     {return String::from("Amharic");}
     QLocale__Arabic => // 8
     {return String::from("Arabic");}
     QLocale__Armenian => // 9
     {return String::from("Armenian");}
     QLocale__Assamese => // 10
     {return String::from("Assamese");}
     QLocale__Aymara => // 11
     {return String::from("Aymara");}
     QLocale__Azerbaijani => // 12
     {return String::from("Azerbaijani");}
     QLocale__Bashkir => // 13
     {return String::from("Bashkir");}
     QLocale__Basque => // 14
     {return String::from("Basque");}
     QLocale__Bengali => // 15
     {return String::from("Bengali");}
     QLocale__Dzongkha => // 16
     {return String::from("Dzongkha,Bhutani");}
     QLocale__Bihari => // 17
     {return String::from("Bihari");}
     QLocale__Bislama => // 18
     {return String::from("Bislama");}
     QLocale__Breton => // 19
     {return String::from("Breton");}
     QLocale__Bulgarian => // 20
     {return String::from("Bulgarian");}
     QLocale__Burmese => // 21
     {return String::from("Burmese");}
     QLocale__Belarusian => // 22
     {return String::from("Belarusian,Byelorussian");}
     QLocale__Khmer => // 23
     {return String::from("Khmer,Cambodian");}
     QLocale__Catalan => // 24
     {return String::from("Catalan");}
     QLocale__Chinese => // 25
     {return String::from("Chinese");}
     QLocale__Corsican => // 26
     {return String::from("Corsican");}
     QLocale__Croatian => // 27
     {return String::from("Croatian");}
     QLocale__Czech => // 28
     {return String::from("Czech");}
     QLocale__Danish => // 29
     {return String::from("Danish");}
     QLocale__Dutch => // 30
     {return String::from("Dutch");}
     QLocale__English => // 31
     {return String::from("English");}
     QLocale__Esperanto => // 32
     {return String::from("Esperanto");}
     QLocale__Estonian => // 33
     {return String::from("Estonian");}
     QLocale__Faroese => // 34
     {return String::from("Faroese");}
     QLocale__Fijian => // 35
     {return String::from("Fijian");}
     QLocale__Finnish => // 36
     {return String::from("Finnish");}
     QLocale__French => // 37
     {return String::from("French");}
     QLocale__WesternFrisian => // 38
     {return String::from("WesternFrisian,Frisian");}
     QLocale__Gaelic => // 39
     {return String::from("Gaelic");}
     QLocale__Galician => // 40
     {return String::from("Galician");}
     QLocale__Georgian => // 41
     {return String::from("Georgian");}
     QLocale__German => // 42
     {return String::from("German");}
     QLocale__Greek => // 43
     {return String::from("Greek");}
     QLocale__Greenlandic => // 44
     {return String::from("Greenlandic");}
     QLocale__Guarani => // 45
     {return String::from("Guarani");}
     QLocale__Gujarati => // 46
     {return String::from("Gujarati");}
     QLocale__Hausa => // 47
     {return String::from("Hausa");}
     QLocale__Hebrew => // 48
     {return String::from("Hebrew");}
     QLocale__Hindi => // 49
     {return String::from("Hindi");}
     QLocale__Hungarian => // 50
     {return String::from("Hungarian");}
     QLocale__Icelandic => // 51
     {return String::from("Icelandic");}
     QLocale__Indonesian => // 52
     {return String::from("Indonesian");}
     QLocale__Interlingua => // 53
     {return String::from("Interlingua");}
     QLocale__Interlingue => // 54
     {return String::from("Interlingue");}
     QLocale__Inuktitut => // 55
     {return String::from("Inuktitut");}
     QLocale__Inupiak => // 56
     {return String::from("Inupiak");}
     QLocale__Irish => // 57
     {return String::from("Irish");}
     QLocale__Italian => // 58
     {return String::from("Italian");}
     QLocale__Japanese => // 59
     {return String::from("Japanese");}
     QLocale__Javanese => // 60
     {return String::from("Javanese");}
     QLocale__Kannada => // 61
     {return String::from("Kannada");}
     QLocale__Kashmiri => // 62
     {return String::from("Kashmiri");}
     QLocale__Kazakh => // 63
     {return String::from("Kazakh");}
     QLocale__Kinyarwanda => // 64
     {return String::from("Kinyarwanda");}
     QLocale__Kirghiz => // 65
     {return String::from("Kirghiz");}
     QLocale__Korean => // 66
     {return String::from("Korean");}
     QLocale__Kurdish => // 67
     {return String::from("Kurdish");}
     QLocale__Rundi => // 68
     {return String::from("Rundi,Kurundi");}
     QLocale__Lao => // 69
     {return String::from("Lao");}
     QLocale__Latin => // 70
     {return String::from("Latin");}
     QLocale__Latvian => // 71
     {return String::from("Latvian");}
     QLocale__Lingala => // 72
     {return String::from("Lingala");}
     QLocale__Lithuanian => // 73
     {return String::from("Lithuanian");}
     QLocale__Macedonian => // 74
     {return String::from("Macedonian");}
     QLocale__Malagasy => // 75
     {return String::from("Malagasy");}
     QLocale__Malay => // 76
     {return String::from("Malay");}
     QLocale__Malayalam => // 77
     {return String::from("Malayalam");}
     QLocale__Maltese => // 78
     {return String::from("Maltese");}
     QLocale__Maori => // 79
     {return String::from("Maori");}
     QLocale__Marathi => // 80
     {return String::from("Marathi");}
     QLocale__Marshallese => // 81
     {return String::from("Marshallese");}
     QLocale__Mongolian => // 82
     {return String::from("Mongolian");}
     QLocale__NauruLanguage => // 83
     {return String::from("NauruLanguage");}
     QLocale__Nepali => // 84
     {return String::from("Nepali");}
     QLocale__NorwegianBokmal => // 85
     {return String::from("NorwegianBokmal,Norwegian");}
     QLocale__Occitan => // 86
     {return String::from("Occitan");}
     QLocale__Oriya => // 87
     {return String::from("Oriya");}
     QLocale__Pashto => // 88
     {return String::from("Pashto");}
     QLocale__Persian => // 89
     {return String::from("Persian");}
     QLocale__Polish => // 90
     {return String::from("Polish");}
     QLocale__Portuguese => // 91
     {return String::from("Portuguese");}
     QLocale__Punjabi => // 92
     {return String::from("Punjabi");}
     QLocale__Quechua => // 93
     {return String::from("Quechua");}
     QLocale__Romansh => // 94
     {return String::from("Romansh,RhaetoRomance");}
     QLocale__Romanian => // 95
     {return String::from("Romanian,Moldavian");}
     QLocale__Russian => // 96
     {return String::from("Russian");}
     QLocale__Samoan => // 97
     {return String::from("Samoan");}
     QLocale__Sango => // 98
     {return String::from("Sango");}
     QLocale__Sanskrit => // 99
     {return String::from("Sanskrit");}
     QLocale__Serbian => // 100
     {return String::from("Serbian,SerboCroatian");}
     QLocale__Ossetic => // 101
     {return String::from("Ossetic");}
     QLocale__SouthernSotho => // 102
     {return String::from("SouthernSotho");}
     QLocale__Tswana => // 103
     {return String::from("Tswana");}
     QLocale__Shona => // 104
     {return String::from("Shona");}
     QLocale__Sindhi => // 105
     {return String::from("Sindhi");}
     QLocale__Sinhala => // 106
     {return String::from("Sinhala");}
     QLocale__Swati => // 107
     {return String::from("Swati");}
     QLocale__Slovak => // 108
     {return String::from("Slovak");}
     QLocale__Slovenian => // 109
     {return String::from("Slovenian");}
     QLocale__Somali => // 110
     {return String::from("Somali");}
     QLocale__Spanish => // 111
     {return String::from("Spanish");}
     QLocale__Sundanese => // 112
     {return String::from("Sundanese");}
     QLocale__Swahili => // 113
     {return String::from("Swahili");}
     QLocale__Swedish => // 114
     {return String::from("Swedish");}
     QLocale__Sardinian => // 115
     {return String::from("Sardinian");}
     QLocale__Tajik => // 116
     {return String::from("Tajik");}
     QLocale__Tamil => // 117
     {return String::from("Tamil");}
     QLocale__Tatar => // 118
     {return String::from("Tatar");}
     QLocale__Telugu => // 119
     {return String::from("Telugu");}
     QLocale__Thai => // 120
     {return String::from("Thai");}
     QLocale__Tibetan => // 121
     {return String::from("Tibetan");}
     QLocale__Tigrinya => // 122
     {return String::from("Tigrinya");}
     QLocale__Tongan => // 123
     {return String::from("Tongan");}
     QLocale__Tsonga => // 124
     {return String::from("Tsonga");}
     QLocale__Turkish => // 125
     {return String::from("Turkish");}
     QLocale__Turkmen => // 126
     {return String::from("Turkmen");}
     QLocale__Tahitian => // 127
     {return String::from("Tahitian");}
     QLocale__Uighur => // 128
     {return String::from("Uighur,Uigur");}
     QLocale__Ukrainian => // 129
     {return String::from("Ukrainian");}
     QLocale__Urdu => // 130
     {return String::from("Urdu");}
     QLocale__Uzbek => // 131
     {return String::from("Uzbek");}
     QLocale__Vietnamese => // 132
     {return String::from("Vietnamese");}
     QLocale__Volapuk => // 133
     {return String::from("Volapuk");}
     QLocale__Welsh => // 134
     {return String::from("Welsh");}
     QLocale__Wolof => // 135
     {return String::from("Wolof");}
     QLocale__Xhosa => // 136
     {return String::from("Xhosa");}
     QLocale__Yiddish => // 137
     {return String::from("Yiddish");}
     QLocale__Yoruba => // 138
     {return String::from("Yoruba");}
     QLocale__Zhuang => // 139
     {return String::from("Zhuang");}
     QLocale__Zulu => // 140
     {return String::from("Zulu");}
     QLocale__NorwegianNynorsk => // 141
     {return String::from("NorwegianNynorsk");}
     QLocale__Bosnian => // 142
     {return String::from("Bosnian");}
     QLocale__Divehi => // 143
     {return String::from("Divehi");}
     QLocale__Manx => // 144
     {return String::from("Manx");}
     QLocale__Cornish => // 145
     {return String::from("Cornish");}
     QLocale__Akan => // 146
     {return String::from("Akan,Twi");}
     QLocale__Konkani => // 147
     {return String::from("Konkani");}
     QLocale__Ga => // 148
     {return String::from("Ga");}
     QLocale__Igbo => // 149
     {return String::from("Igbo");}
     QLocale__Kamba => // 150
     {return String::from("Kamba");}
     QLocale__Syriac => // 151
     {return String::from("Syriac");}
     QLocale__Blin => // 152
     {return String::from("Blin");}
     QLocale__Geez => // 153
     {return String::from("Geez");}
     QLocale__Koro => // 154
     {return String::from("Koro");}
     QLocale__Sidamo => // 155
     {return String::from("Sidamo");}
     QLocale__Atsam => // 156
     {return String::from("Atsam");}
     QLocale__Tigre => // 157
     {return String::from("Tigre");}
     QLocale__Jju => // 158
     {return String::from("Jju");}
     QLocale__Friulian => // 159
     {return String::from("Friulian");}
     QLocale__Venda => // 160
     {return String::from("Venda");}
     QLocale__Ewe => // 161
     {return String::from("Ewe");}
     QLocale__Walamo => // 162
     {return String::from("Walamo");}
     QLocale__Hawaiian => // 163
     {return String::from("Hawaiian");}
     QLocale__Tyap => // 164
     {return String::from("Tyap");}
     QLocale__Nyanja => // 165
     {return String::from("Nyanja,Chewa");}
     QLocale__Filipino => // 166
     {return String::from("Filipino,Tagalog");}
     QLocale__SwissGerman => // 167
     {return String::from("SwissGerman");}
     QLocale__SichuanYi => // 168
     {return String::from("SichuanYi");}
     QLocale__Kpelle => // 169
     {return String::from("Kpelle");}
     QLocale__LowGerman => // 170
     {return String::from("LowGerman");}
     QLocale__SouthNdebele => // 171
     {return String::from("SouthNdebele");}
     QLocale__NorthernSotho => // 172
     {return String::from("NorthernSotho");}
     QLocale__NorthernSami => // 173
     {return String::from("NorthernSami");}
     QLocale__Taroko => // 174
     {return String::from("Taroko");}
     QLocale__Gusii => // 175
     {return String::from("Gusii");}
     QLocale__Taita => // 176
     {return String::from("Taita");}
     QLocale__Fulah => // 177
     {return String::from("Fulah");}
     QLocale__Kikuyu => // 178
     {return String::from("Kikuyu");}
     QLocale__Samburu => // 179
     {return String::from("Samburu");}
     QLocale__Sena => // 180
     {return String::from("Sena");}
     QLocale__NorthNdebele => // 181
     {return String::from("NorthNdebele");}
     QLocale__Rombo => // 182
     {return String::from("Rombo");}
     QLocale__Tachelhit => // 183
     {return String::from("Tachelhit");}
     QLocale__Kabyle => // 184
     {return String::from("Kabyle");}
     QLocale__Nyankole => // 185
     {return String::from("Nyankole");}
     QLocale__Bena => // 186
     {return String::from("Bena");}
     QLocale__Vunjo => // 187
     {return String::from("Vunjo");}
     QLocale__Bambara => // 188
     {return String::from("Bambara");}
     QLocale__Embu => // 189
     {return String::from("Embu");}
     QLocale__Cherokee => // 190
     {return String::from("Cherokee");}
     QLocale__Morisyen => // 191
     {return String::from("Morisyen");}
     QLocale__Makonde => // 192
     {return String::from("Makonde");}
     QLocale__Langi => // 193
     {return String::from("Langi");}
     QLocale__Ganda => // 194
     {return String::from("Ganda");}
     QLocale__Bemba => // 195
     {return String::from("Bemba");}
     QLocale__Kabuverdianu => // 196
     {return String::from("Kabuverdianu");}
     QLocale__Meru => // 197
     {return String::from("Meru");}
     QLocale__Kalenjin => // 198
     {return String::from("Kalenjin");}
     QLocale__Nama => // 199
     {return String::from("Nama");}
     QLocale__Machame => // 200
     {return String::from("Machame");}
     QLocale__Colognian => // 201
     {return String::from("Colognian");}
     QLocale__Masai => // 202
     {return String::from("Masai");}
     QLocale__Soga => // 203
     {return String::from("Soga");}
     QLocale__Luyia => // 204
     {return String::from("Luyia");}
     QLocale__Asu => // 205
     {return String::from("Asu");}
     QLocale__Teso => // 206
     {return String::from("Teso");}
     QLocale__Saho => // 207
     {return String::from("Saho");}
     QLocale__KoyraChiini => // 208
     {return String::from("KoyraChiini");}
     QLocale__Rwa => // 209
     {return String::from("Rwa");}
     QLocale__Luo => // 210
     {return String::from("Luo");}
     QLocale__Chiga => // 211
     {return String::from("Chiga");}
     QLocale__CentralMoroccoTamazight => // 212
     {return String::from("CentralMoroccoTamazight");}
     QLocale__KoyraboroSenni => // 213
     {return String::from("KoyraboroSenni");}
     QLocale__Shambala => // 214
     {return String::from("Shambala");}
     QLocale__Bodo => // 215
     {return String::from("Bodo");}
     QLocale__Avaric => // 216
     {return String::from("Avaric");}
     QLocale__Chamorro => // 217
     {return String::from("Chamorro");}
     QLocale__Chechen => // 218
     {return String::from("Chechen");}
     QLocale__Church => // 219
     {return String::from("Church");}
     QLocale__Chuvash => // 220
     {return String::from("Chuvash");}
     QLocale__Cree => // 221
     {return String::from("Cree");}
     QLocale__Haitian => // 222
     {return String::from("Haitian");}
     QLocale__Herero => // 223
     {return String::from("Herero");}
     QLocale__HiriMotu => // 224
     {return String::from("HiriMotu");}
     QLocale__Kanuri => // 225
     {return String::from("Kanuri");}
     QLocale__Komi => // 226
     {return String::from("Komi");}
     QLocale__Kongo => // 227
     {return String::from("Kongo");}
     QLocale__Kwanyama => // 228
     {return String::from("Kwanyama");}
     QLocale__Limburgish => // 229
     {return String::from("Limburgish");}
     QLocale__LubaKatanga => // 230
     {return String::from("LubaKatanga");}
     QLocale__Luxembourgish => // 231
     {return String::from("Luxembourgish");}
     QLocale__Navaho => // 232
     {return String::from("Navaho");}
     QLocale__Ndonga => // 233
     {return String::from("Ndonga");}
     QLocale__Ojibwa => // 234
     {return String::from("Ojibwa");}
     QLocale__Pali => // 235
     {return String::from("Pali");}
     QLocale__Walloon => // 236
     {return String::from("Walloon");}
     QLocale__Aghem => // 237
     {return String::from("Aghem");}
     QLocale__Basaa => // 238
     {return String::from("Basaa");}
     QLocale__Zarma => // 239
     {return String::from("Zarma");}
     QLocale__Duala => // 240
     {return String::from("Duala");}
     QLocale__JolaFonyi => // 241
     {return String::from("JolaFonyi");}
     QLocale__Ewondo => // 242
     {return String::from("Ewondo");}
     QLocale__Bafia => // 243
     {return String::from("Bafia");}
     QLocale__MakhuwaMeetto => // 244
     {return String::from("MakhuwaMeetto");}
     QLocale__Mundang => // 245
     {return String::from("Mundang");}
     QLocale__Kwasio => // 246
     {return String::from("Kwasio");}
     QLocale__Nuer => // 247
     {return String::from("Nuer");}
     QLocale__Sakha => // 248
     {return String::from("Sakha");}
     QLocale__Sangu => // 249
     {return String::from("Sangu");}
     QLocale__CongoSwahili => // 250
     {return String::from("CongoSwahili");}
     QLocale__Tasawaq => // 251
     {return String::from("Tasawaq");}
     QLocale__Vai => // 252
     {return String::from("Vai");}
     QLocale__Walser => // 253
     {return String::from("Walser");}
     QLocale__Yangben => // 254
     {return String::from("Yangben");}
     QLocale__Avestan => // 255
     {return String::from("Avestan");}
     QLocale__Asturian => // 256
     {return String::from("Asturian");}
     QLocale__Ngomba => // 257
     {return String::from("Ngomba");}
     QLocale__Kako => // 258
     {return String::from("Kako");}
     QLocale__Meta => // 259
     {return String::from("Meta");}
     QLocale__Ngiemboon => // 260
     {return String::from("Ngiemboon");}
     QLocale__Aragonese => // 261
     {return String::from("Aragonese");}
     QLocale__Akkadian => // 262
     {return String::from("Akkadian");}
     QLocale__AncientEgyptian => // 263
     {return String::from("AncientEgyptian");}
     QLocale__AncientGreek => // 264
     {return String::from("AncientGreek");}
     QLocale__Aramaic => // 265
     {return String::from("Aramaic");}
     QLocale__Balinese => // 266
     {return String::from("Balinese");}
     QLocale__Bamun => // 267
     {return String::from("Bamun");}
     QLocale__BatakToba => // 268
     {return String::from("BatakToba");}
     QLocale__Buginese => // 269
     {return String::from("Buginese");}
     QLocale__Buhid => // 270
     {return String::from("Buhid");}
     QLocale__Carian => // 271
     {return String::from("Carian");}
     QLocale__Chakma => // 272
     {return String::from("Chakma");}
     QLocale__ClassicalMandaic => // 273
     {return String::from("ClassicalMandaic");}
     QLocale__Coptic => // 274
     {return String::from("Coptic");}
     QLocale__Dogri => // 275
     {return String::from("Dogri");}
     QLocale__EasternCham => // 276
     {return String::from("EasternCham");}
     QLocale__EasternKayah => // 277
     {return String::from("EasternKayah");}
     QLocale__Etruscan => // 278
     {return String::from("Etruscan");}
     QLocale__Gothic => // 279
     {return String::from("Gothic");}
     QLocale__Hanunoo => // 280
     {return String::from("Hanunoo");}
     QLocale__Ingush => // 281
     {return String::from("Ingush");}
     QLocale__LargeFloweryMiao => // 282
     {return String::from("LargeFloweryMiao");}
     QLocale__Lepcha => // 283
     {return String::from("Lepcha");}
     QLocale__Limbu => // 284
     {return String::from("Limbu");}
     QLocale__Lisu => // 285
     {return String::from("Lisu");}
     QLocale__Lu => // 286
     {return String::from("Lu");}
     QLocale__Lycian => // 287
     {return String::from("Lycian");}
     QLocale__Lydian => // 288
     {return String::from("Lydian");}
     QLocale__Mandingo => // 289
     {return String::from("Mandingo");}
     QLocale__Manipuri => // 290
     {return String::from("Manipuri");}
     QLocale__Meroitic => // 291
     {return String::from("Meroitic");}
     QLocale__NorthernThai => // 292
     {return String::from("NorthernThai");}
     QLocale__OldIrish => // 293
     {return String::from("OldIrish");}
     QLocale__OldNorse => // 294
     {return String::from("OldNorse");}
     QLocale__OldPersian => // 295
     {return String::from("OldPersian");}
     QLocale__OldTurkish => // 296
     {return String::from("OldTurkish");}
     QLocale__Pahlavi => // 297
     {return String::from("Pahlavi");}
     QLocale__Parthian => // 298
     {return String::from("Parthian");}
     QLocale__Phoenician => // 299
     {return String::from("Phoenician");}
     QLocale__PrakritLanguage => // 300
     {return String::from("PrakritLanguage");}
     QLocale__Rejang => // 301
     {return String::from("Rejang");}
     QLocale__Sabaean => // 302
     {return String::from("Sabaean");}
     QLocale__Samaritan => // 303
     {return String::from("Samaritan");}
     QLocale__Santali => // 304
     {return String::from("Santali");}
     QLocale__Saurashtra => // 305
     {return String::from("Saurashtra");}
     QLocale__Sora => // 306
     {return String::from("Sora");}
     QLocale__Sylheti => // 307
     {return String::from("Sylheti");}
     QLocale__Tagbanwa => // 308
     {return String::from("Tagbanwa");}
     QLocale__TaiDam => // 309
     {return String::from("TaiDam");}
     QLocale__TaiNua => // 310
     {return String::from("TaiNua");}
     QLocale__Ugaritic => // 311
     {return String::from("Ugaritic");}
     QLocale__Akoose => // 312
     {return String::from("Akoose");}
     QLocale__Lakota => // 313
     {return String::from("Lakota");}
     QLocale__StandardMoroccanTamazight => // 314
     {return String::from("StandardMoroccanTamazight");}
     QLocale__Mapuche => // 315
     {return String::from("Mapuche");}
     QLocale__CentralKurdish => // 316
     {return String::from("CentralKurdish");}
     QLocale__LowerSorbian => // 317
     {return String::from("LowerSorbian");}
     QLocale__UpperSorbian => // 318
     {return String::from("UpperSorbian");}
     QLocale__Kenyang => // 319
     {return String::from("Kenyang");}
     QLocale__Mohawk => // 320
     {return String::from("Mohawk");}
     QLocale__Nko => // 321
     {return String::from("Nko");}
     QLocale__Prussian => // 322
     {return String::from("Prussian");}
     QLocale__Kiche => // 323
     {return String::from("Kiche");}
     QLocale__SouthernSami => // 324
     {return String::from("SouthernSami");}
     QLocale__LuleSami => // 325
     {return String::from("LuleSami");}
     QLocale__InariSami => // 326
     {return String::from("InariSami");}
     QLocale__SkoltSami => // 327
     {return String::from("SkoltSami");}
     QLocale__Warlpiri => // 328
     {return String::from("Warlpiri");}
     QLocale__ManichaeanMiddlePersian => // 329
     {return String::from("ManichaeanMiddlePersian");}
     QLocale__Mende => // 330
     {return String::from("Mende");}
     QLocale__AncientNorthArabian => // 331
     {return String::from("AncientNorthArabian");}
     QLocale__LinearA => // 332
     {return String::from("LinearA");}
     QLocale__HmongNjua => // 333
     {return String::from("HmongNjua");}
     QLocale__Ho => // 334
     {return String::from("Ho");}
     QLocale__Lezghian => // 335
     {return String::from("Lezghian");}
     QLocale__Bassa => // 336
     {return String::from("Bassa");}
     QLocale__Mono => // 337
     {return String::from("Mono");}
     QLocale__TedimChin => // 338
     {return String::from("TedimChin");}
     QLocale__Maithili => // 339
     {return String::from("Maithili");}
     QLocale__Ahom => // 340
     {return String::from("Ahom");}
     QLocale__AmericanSignLanguage => // 341
     {return String::from("AmericanSignLanguage");}
     QLocale__ArdhamagadhiPrakrit => // 342
     {return String::from("ArdhamagadhiPrakrit");}
     QLocale__Bhojpuri => // 343
     {return String::from("Bhojpuri");}
     QLocale__HieroglyphicLuwian => // 344
     {return String::from("HieroglyphicLuwian");}
     QLocale__LiteraryChinese => // 345
     {return String::from("LiteraryChinese");}
     QLocale__Mazanderani => // 346
     {return String::from("Mazanderani");}
     QLocale__Mru => // 347
     {return String::from("Mru");}
     QLocale__Newari => // 348
     {return String::from("Newari");}
     QLocale__NorthernLuri => // 349
     {return String::from("NorthernLuri");}
     QLocale__Palauan => // 350
     {return String::from("Palauan");}
     QLocale__Papiamento => // 351
     {return String::from("Papiamento");}
     QLocale__Saraiki => // 352
     {return String::from("Saraiki");}
     QLocale__TokelauLanguage => // 353
     {return String::from("TokelauLanguage");}
     QLocale__TokPisin => // 354
     {return String::from("TokPisin");}
     QLocale__TuvaluLanguage => // 355
     {return String::from("TuvaluLanguage");}
     QLocale__UncodedLanguages => // 356
     {return String::from("UncodedLanguages");}
     QLocale__Cantonese => // 357
     {return String::from("Cantonese");}
     QLocale__Osage => // 358
     {return String::from("Osage");}
     QLocale__Tangut => // 359
     {return String::from("Tangut,LastLanguage");}
    // QLocale__Norwegian => // 85
    // {return String::from("");}
    // QLocale__Moldavian => // 95
    // {return String::from("");}
    // QLocale__SerboCroatian => // 100
    // {return String::from("");}
    // QLocale__Tagalog => // 166
    // {return String::from("");}
    // QLocale__Twi => // 146
    // {return String::from("");}
    // QLocale__Afan => // 3
    // {return String::from("");}
    // QLocale__Byelorussian => // 22
    // {return String::from("");}
    // QLocale__Bhutani => // 16
    // {return String::from("");}
    // QLocale__Cambodian => // 23
    // {return String::from("");}
    // QLocale__Kurundi => // 68
    // {return String::from("");}
    // QLocale__RhaetoRomance => // 94
    // {return String::from("");}
    // QLocale__Chewa => // 165
    // {return String::from("");}
    // QLocale__Frisian => // 38
    // {return String::from("");}
    // QLocale__Uigur => // 128
    // {return String::from("");}
    // QLocale__LastLanguage => // 359
    // {return String::from("");}
  _ => {return format!("{}", val);}
}
}
pub fn QLocale_LanguageItemName_s(val: i32) ->String {
  //var nilthis *QLocale
  //return nilthis.LanguageItemName(val);
  return QLocale_LanguageItemName(val);
}


/*
This enumerated type is used to specify a script.

QLocale::SimplifiedChineseScriptSimplifiedHanScriptsame as SimplifiedHanScript
QLocale::TraditionalChineseScriptTraditionalHanScriptsame as TraditionalHanScript


See also script(), scriptToString(), and languageToString().

*/
pub type QLocale__Script = i32;
//  
pub const QLocale__AnyScript :QLocale__Script = 0;
//  
pub const QLocale__ArabicScript :QLocale__Script = 1;
//  
pub const QLocale__CyrillicScript :QLocale__Script = 2;
//  
pub const QLocale__DeseretScript :QLocale__Script = 3;
//  
pub const QLocale__GurmukhiScript :QLocale__Script = 4;
// same as SimplifiedChineseScript
pub const QLocale__SimplifiedHanScript :QLocale__Script = 5;
// same as TraditionalChineseScript
pub const QLocale__TraditionalHanScript :QLocale__Script = 6;
//  
pub const QLocale__LatinScript :QLocale__Script = 7;
//  
pub const QLocale__MongolianScript :QLocale__Script = 8;
//  
pub const QLocale__TifinaghScript :QLocale__Script = 9;
// 
pub const QLocale__ArmenianScript :QLocale__Script = 10;
// 
pub const QLocale__BengaliScript :QLocale__Script = 11;
// 
pub const QLocale__CherokeeScript :QLocale__Script = 12;
// 
pub const QLocale__DevanagariScript :QLocale__Script = 13;
// 
pub const QLocale__EthiopicScript :QLocale__Script = 14;
// 
pub const QLocale__GeorgianScript :QLocale__Script = 15;
// 
pub const QLocale__GreekScript :QLocale__Script = 16;
// 
pub const QLocale__GujaratiScript :QLocale__Script = 17;
// 
pub const QLocale__HebrewScript :QLocale__Script = 18;
// 
pub const QLocale__JapaneseScript :QLocale__Script = 19;
// 
pub const QLocale__KhmerScript :QLocale__Script = 20;
// 
pub const QLocale__KannadaScript :QLocale__Script = 21;
// 
pub const QLocale__KoreanScript :QLocale__Script = 22;
// 
pub const QLocale__LaoScript :QLocale__Script = 23;
// 
pub const QLocale__MalayalamScript :QLocale__Script = 24;
// 
pub const QLocale__MyanmarScript :QLocale__Script = 25;
// 
pub const QLocale__OriyaScript :QLocale__Script = 26;
// 
pub const QLocale__TamilScript :QLocale__Script = 27;
// 
pub const QLocale__TeluguScript :QLocale__Script = 28;
// 
pub const QLocale__ThaanaScript :QLocale__Script = 29;
// 
pub const QLocale__ThaiScript :QLocale__Script = 30;
// 
pub const QLocale__TibetanScript :QLocale__Script = 31;
// 
pub const QLocale__SinhalaScript :QLocale__Script = 32;
// 
pub const QLocale__SyriacScript :QLocale__Script = 33;
// 
pub const QLocale__YiScript :QLocale__Script = 34;
// 
pub const QLocale__VaiScript :QLocale__Script = 35;
// 
pub const QLocale__AvestanScript :QLocale__Script = 36;
// 
pub const QLocale__BalineseScript :QLocale__Script = 37;
// 
pub const QLocale__BamumScript :QLocale__Script = 38;
// 
pub const QLocale__BatakScript :QLocale__Script = 39;
// 
pub const QLocale__BopomofoScript :QLocale__Script = 40;
// 
pub const QLocale__BrahmiScript :QLocale__Script = 41;
// 
pub const QLocale__BugineseScript :QLocale__Script = 42;
// 
pub const QLocale__BuhidScript :QLocale__Script = 43;
// 
pub const QLocale__CanadianAboriginalScript :QLocale__Script = 44;
// 
pub const QLocale__CarianScript :QLocale__Script = 45;
// 
pub const QLocale__ChakmaScript :QLocale__Script = 46;
// 
pub const QLocale__ChamScript :QLocale__Script = 47;
// 
pub const QLocale__CopticScript :QLocale__Script = 48;
// 
pub const QLocale__CypriotScript :QLocale__Script = 49;
// 
pub const QLocale__EgyptianHieroglyphsScript :QLocale__Script = 50;
// 
pub const QLocale__FraserScript :QLocale__Script = 51;
// 
pub const QLocale__GlagoliticScript :QLocale__Script = 52;
// 
pub const QLocale__GothicScript :QLocale__Script = 53;
// 
pub const QLocale__HanScript :QLocale__Script = 54;
// 
pub const QLocale__HangulScript :QLocale__Script = 55;
// 
pub const QLocale__HanunooScript :QLocale__Script = 56;
// 
pub const QLocale__ImperialAramaicScript :QLocale__Script = 57;
// 
pub const QLocale__InscriptionalPahlaviScript :QLocale__Script = 58;
// 
pub const QLocale__InscriptionalParthianScript :QLocale__Script = 59;
// 
pub const QLocale__JavaneseScript :QLocale__Script = 60;
// 
pub const QLocale__KaithiScript :QLocale__Script = 61;
// 
pub const QLocale__KatakanaScript :QLocale__Script = 62;
// 
pub const QLocale__KayahLiScript :QLocale__Script = 63;
// 
pub const QLocale__KharoshthiScript :QLocale__Script = 64;
// 
pub const QLocale__LannaScript :QLocale__Script = 65;
// 
pub const QLocale__LepchaScript :QLocale__Script = 66;
// 
pub const QLocale__LimbuScript :QLocale__Script = 67;
// 
pub const QLocale__LinearBScript :QLocale__Script = 68;
// 
pub const QLocale__LycianScript :QLocale__Script = 69;
// 
pub const QLocale__LydianScript :QLocale__Script = 70;
// 
pub const QLocale__MandaeanScript :QLocale__Script = 71;
// 
pub const QLocale__MeiteiMayekScript :QLocale__Script = 72;
// 
pub const QLocale__MeroiticScript :QLocale__Script = 73;
// 
pub const QLocale__MeroiticCursiveScript :QLocale__Script = 74;
// 
pub const QLocale__NkoScript :QLocale__Script = 75;
// 
pub const QLocale__NewTaiLueScript :QLocale__Script = 76;
// 
pub const QLocale__OghamScript :QLocale__Script = 77;
// 
pub const QLocale__OlChikiScript :QLocale__Script = 78;
// 
pub const QLocale__OldItalicScript :QLocale__Script = 79;
// 
pub const QLocale__OldPersianScript :QLocale__Script = 80;
// 
pub const QLocale__OldSouthArabianScript :QLocale__Script = 81;
// 
pub const QLocale__OrkhonScript :QLocale__Script = 82;
// 
pub const QLocale__OsmanyaScript :QLocale__Script = 83;
// 
pub const QLocale__PhagsPaScript :QLocale__Script = 84;
// 
pub const QLocale__PhoenicianScript :QLocale__Script = 85;
// 
pub const QLocale__PollardPhoneticScript :QLocale__Script = 86;
// 
pub const QLocale__RejangScript :QLocale__Script = 87;
// 
pub const QLocale__RunicScript :QLocale__Script = 88;
// 
pub const QLocale__SamaritanScript :QLocale__Script = 89;
// 
pub const QLocale__SaurashtraScript :QLocale__Script = 90;
// 
pub const QLocale__SharadaScript :QLocale__Script = 91;
// 
pub const QLocale__ShavianScript :QLocale__Script = 92;
// 
pub const QLocale__SoraSompengScript :QLocale__Script = 93;
// 
pub const QLocale__CuneiformScript :QLocale__Script = 94;
// 
pub const QLocale__SundaneseScript :QLocale__Script = 95;
// 
pub const QLocale__SylotiNagriScript :QLocale__Script = 96;
// 
pub const QLocale__TagalogScript :QLocale__Script = 97;
// 
pub const QLocale__TagbanwaScript :QLocale__Script = 98;
// 
pub const QLocale__TaiLeScript :QLocale__Script = 99;
// 
pub const QLocale__TaiVietScript :QLocale__Script = 100;
// 
pub const QLocale__TakriScript :QLocale__Script = 101;
// 
pub const QLocale__UgariticScript :QLocale__Script = 102;
// 
pub const QLocale__BrailleScript :QLocale__Script = 103;
// 
pub const QLocale__HiraganaScript :QLocale__Script = 104;
// 
pub const QLocale__CaucasianAlbanianScript :QLocale__Script = 105;
// 
pub const QLocale__BassaVahScript :QLocale__Script = 106;
// 
pub const QLocale__DuployanScript :QLocale__Script = 107;
// 
pub const QLocale__ElbasanScript :QLocale__Script = 108;
// 
pub const QLocale__GranthaScript :QLocale__Script = 109;
// 
pub const QLocale__PahawhHmongScript :QLocale__Script = 110;
// 
pub const QLocale__KhojkiScript :QLocale__Script = 111;
// 
pub const QLocale__LinearAScript :QLocale__Script = 112;
// 
pub const QLocale__MahajaniScript :QLocale__Script = 113;
// 
pub const QLocale__ManichaeanScript :QLocale__Script = 114;
// 
pub const QLocale__MendeKikakuiScript :QLocale__Script = 115;
// 
pub const QLocale__ModiScript :QLocale__Script = 116;
// 
pub const QLocale__MroScript :QLocale__Script = 117;
// 
pub const QLocale__OldNorthArabianScript :QLocale__Script = 118;
// 
pub const QLocale__NabataeanScript :QLocale__Script = 119;
// 
pub const QLocale__PalmyreneScript :QLocale__Script = 120;
// 
pub const QLocale__PauCinHauScript :QLocale__Script = 121;
// 
pub const QLocale__OldPermicScript :QLocale__Script = 122;
// 
pub const QLocale__PsalterPahlaviScript :QLocale__Script = 123;
// 
pub const QLocale__SiddhamScript :QLocale__Script = 124;
// 
pub const QLocale__KhudawadiScript :QLocale__Script = 125;
// 
pub const QLocale__TirhutaScript :QLocale__Script = 126;
// 
pub const QLocale__VarangKshitiScript :QLocale__Script = 127;
// 
pub const QLocale__AhomScript :QLocale__Script = 128;
// 
pub const QLocale__AnatolianHieroglyphsScript :QLocale__Script = 129;
// 
pub const QLocale__HatranScript :QLocale__Script = 130;
// 
pub const QLocale__MultaniScript :QLocale__Script = 131;
// 
pub const QLocale__OldHungarianScript :QLocale__Script = 132;
// 
pub const QLocale__SignWritingScript :QLocale__Script = 133;
// 
pub const QLocale__AdlamScript :QLocale__Script = 134;
// 
pub const QLocale__BhaiksukiScript :QLocale__Script = 135;
// 
pub const QLocale__MarchenScript :QLocale__Script = 136;
// 
pub const QLocale__NewaScript :QLocale__Script = 137;
// 
pub const QLocale__OsageScript :QLocale__Script = 138;
// 
pub const QLocale__TangutScript :QLocale__Script = 139;
// 
pub const QLocale__HanWithBopomofoScript :QLocale__Script = 140;
// 
pub const QLocale__JamoScript :QLocale__Script = 141;
// 
pub const QLocale__SimplifiedChineseScript :QLocale__Script = 5;
// 
pub const QLocale__TraditionalChineseScript :QLocale__Script = 6;
// 
pub const QLocale__LastScript :QLocale__Script = 141;
pub fn QLocale_ScriptItemName(val: i32) ->String {
  match val {
     QLocale__AnyScript => // 0
     {return String::from("AnyScript");}
     QLocale__ArabicScript => // 1
     {return String::from("ArabicScript");}
     QLocale__CyrillicScript => // 2
     {return String::from("CyrillicScript");}
     QLocale__DeseretScript => // 3
     {return String::from("DeseretScript");}
     QLocale__GurmukhiScript => // 4
     {return String::from("GurmukhiScript");}
     QLocale__SimplifiedHanScript => // 5
     {return String::from("SimplifiedHanScript,SimplifiedChineseScript");}
     QLocale__TraditionalHanScript => // 6
     {return String::from("TraditionalHanScript,TraditionalChineseScript");}
     QLocale__LatinScript => // 7
     {return String::from("LatinScript");}
     QLocale__MongolianScript => // 8
     {return String::from("MongolianScript");}
     QLocale__TifinaghScript => // 9
     {return String::from("TifinaghScript");}
     QLocale__ArmenianScript => // 10
     {return String::from("ArmenianScript");}
     QLocale__BengaliScript => // 11
     {return String::from("BengaliScript");}
     QLocale__CherokeeScript => // 12
     {return String::from("CherokeeScript");}
     QLocale__DevanagariScript => // 13
     {return String::from("DevanagariScript");}
     QLocale__EthiopicScript => // 14
     {return String::from("EthiopicScript");}
     QLocale__GeorgianScript => // 15
     {return String::from("GeorgianScript");}
     QLocale__GreekScript => // 16
     {return String::from("GreekScript");}
     QLocale__GujaratiScript => // 17
     {return String::from("GujaratiScript");}
     QLocale__HebrewScript => // 18
     {return String::from("HebrewScript");}
     QLocale__JapaneseScript => // 19
     {return String::from("JapaneseScript");}
     QLocale__KhmerScript => // 20
     {return String::from("KhmerScript");}
     QLocale__KannadaScript => // 21
     {return String::from("KannadaScript");}
     QLocale__KoreanScript => // 22
     {return String::from("KoreanScript");}
     QLocale__LaoScript => // 23
     {return String::from("LaoScript");}
     QLocale__MalayalamScript => // 24
     {return String::from("MalayalamScript");}
     QLocale__MyanmarScript => // 25
     {return String::from("MyanmarScript");}
     QLocale__OriyaScript => // 26
     {return String::from("OriyaScript");}
     QLocale__TamilScript => // 27
     {return String::from("TamilScript");}
     QLocale__TeluguScript => // 28
     {return String::from("TeluguScript");}
     QLocale__ThaanaScript => // 29
     {return String::from("ThaanaScript");}
     QLocale__ThaiScript => // 30
     {return String::from("ThaiScript");}
     QLocale__TibetanScript => // 31
     {return String::from("TibetanScript");}
     QLocale__SinhalaScript => // 32
     {return String::from("SinhalaScript");}
     QLocale__SyriacScript => // 33
     {return String::from("SyriacScript");}
     QLocale__YiScript => // 34
     {return String::from("YiScript");}
     QLocale__VaiScript => // 35
     {return String::from("VaiScript");}
     QLocale__AvestanScript => // 36
     {return String::from("AvestanScript");}
     QLocale__BalineseScript => // 37
     {return String::from("BalineseScript");}
     QLocale__BamumScript => // 38
     {return String::from("BamumScript");}
     QLocale__BatakScript => // 39
     {return String::from("BatakScript");}
     QLocale__BopomofoScript => // 40
     {return String::from("BopomofoScript");}
     QLocale__BrahmiScript => // 41
     {return String::from("BrahmiScript");}
     QLocale__BugineseScript => // 42
     {return String::from("BugineseScript");}
     QLocale__BuhidScript => // 43
     {return String::from("BuhidScript");}
     QLocale__CanadianAboriginalScript => // 44
     {return String::from("CanadianAboriginalScript");}
     QLocale__CarianScript => // 45
     {return String::from("CarianScript");}
     QLocale__ChakmaScript => // 46
     {return String::from("ChakmaScript");}
     QLocale__ChamScript => // 47
     {return String::from("ChamScript");}
     QLocale__CopticScript => // 48
     {return String::from("CopticScript");}
     QLocale__CypriotScript => // 49
     {return String::from("CypriotScript");}
     QLocale__EgyptianHieroglyphsScript => // 50
     {return String::from("EgyptianHieroglyphsScript");}
     QLocale__FraserScript => // 51
     {return String::from("FraserScript");}
     QLocale__GlagoliticScript => // 52
     {return String::from("GlagoliticScript");}
     QLocale__GothicScript => // 53
     {return String::from("GothicScript");}
     QLocale__HanScript => // 54
     {return String::from("HanScript");}
     QLocale__HangulScript => // 55
     {return String::from("HangulScript");}
     QLocale__HanunooScript => // 56
     {return String::from("HanunooScript");}
     QLocale__ImperialAramaicScript => // 57
     {return String::from("ImperialAramaicScript");}
     QLocale__InscriptionalPahlaviScript => // 58
     {return String::from("InscriptionalPahlaviScript");}
     QLocale__InscriptionalParthianScript => // 59
     {return String::from("InscriptionalParthianScript");}
     QLocale__JavaneseScript => // 60
     {return String::from("JavaneseScript");}
     QLocale__KaithiScript => // 61
     {return String::from("KaithiScript");}
     QLocale__KatakanaScript => // 62
     {return String::from("KatakanaScript");}
     QLocale__KayahLiScript => // 63
     {return String::from("KayahLiScript");}
     QLocale__KharoshthiScript => // 64
     {return String::from("KharoshthiScript");}
     QLocale__LannaScript => // 65
     {return String::from("LannaScript");}
     QLocale__LepchaScript => // 66
     {return String::from("LepchaScript");}
     QLocale__LimbuScript => // 67
     {return String::from("LimbuScript");}
     QLocale__LinearBScript => // 68
     {return String::from("LinearBScript");}
     QLocale__LycianScript => // 69
     {return String::from("LycianScript");}
     QLocale__LydianScript => // 70
     {return String::from("LydianScript");}
     QLocale__MandaeanScript => // 71
     {return String::from("MandaeanScript");}
     QLocale__MeiteiMayekScript => // 72
     {return String::from("MeiteiMayekScript");}
     QLocale__MeroiticScript => // 73
     {return String::from("MeroiticScript");}
     QLocale__MeroiticCursiveScript => // 74
     {return String::from("MeroiticCursiveScript");}
     QLocale__NkoScript => // 75
     {return String::from("NkoScript");}
     QLocale__NewTaiLueScript => // 76
     {return String::from("NewTaiLueScript");}
     QLocale__OghamScript => // 77
     {return String::from("OghamScript");}
     QLocale__OlChikiScript => // 78
     {return String::from("OlChikiScript");}
     QLocale__OldItalicScript => // 79
     {return String::from("OldItalicScript");}
     QLocale__OldPersianScript => // 80
     {return String::from("OldPersianScript");}
     QLocale__OldSouthArabianScript => // 81
     {return String::from("OldSouthArabianScript");}
     QLocale__OrkhonScript => // 82
     {return String::from("OrkhonScript");}
     QLocale__OsmanyaScript => // 83
     {return String::from("OsmanyaScript");}
     QLocale__PhagsPaScript => // 84
     {return String::from("PhagsPaScript");}
     QLocale__PhoenicianScript => // 85
     {return String::from("PhoenicianScript");}
     QLocale__PollardPhoneticScript => // 86
     {return String::from("PollardPhoneticScript");}
     QLocale__RejangScript => // 87
     {return String::from("RejangScript");}
     QLocale__RunicScript => // 88
     {return String::from("RunicScript");}
     QLocale__SamaritanScript => // 89
     {return String::from("SamaritanScript");}
     QLocale__SaurashtraScript => // 90
     {return String::from("SaurashtraScript");}
     QLocale__SharadaScript => // 91
     {return String::from("SharadaScript");}
     QLocale__ShavianScript => // 92
     {return String::from("ShavianScript");}
     QLocale__SoraSompengScript => // 93
     {return String::from("SoraSompengScript");}
     QLocale__CuneiformScript => // 94
     {return String::from("CuneiformScript");}
     QLocale__SundaneseScript => // 95
     {return String::from("SundaneseScript");}
     QLocale__SylotiNagriScript => // 96
     {return String::from("SylotiNagriScript");}
     QLocale__TagalogScript => // 97
     {return String::from("TagalogScript");}
     QLocale__TagbanwaScript => // 98
     {return String::from("TagbanwaScript");}
     QLocale__TaiLeScript => // 99
     {return String::from("TaiLeScript");}
     QLocale__TaiVietScript => // 100
     {return String::from("TaiVietScript");}
     QLocale__TakriScript => // 101
     {return String::from("TakriScript");}
     QLocale__UgariticScript => // 102
     {return String::from("UgariticScript");}
     QLocale__BrailleScript => // 103
     {return String::from("BrailleScript");}
     QLocale__HiraganaScript => // 104
     {return String::from("HiraganaScript");}
     QLocale__CaucasianAlbanianScript => // 105
     {return String::from("CaucasianAlbanianScript");}
     QLocale__BassaVahScript => // 106
     {return String::from("BassaVahScript");}
     QLocale__DuployanScript => // 107
     {return String::from("DuployanScript");}
     QLocale__ElbasanScript => // 108
     {return String::from("ElbasanScript");}
     QLocale__GranthaScript => // 109
     {return String::from("GranthaScript");}
     QLocale__PahawhHmongScript => // 110
     {return String::from("PahawhHmongScript");}
     QLocale__KhojkiScript => // 111
     {return String::from("KhojkiScript");}
     QLocale__LinearAScript => // 112
     {return String::from("LinearAScript");}
     QLocale__MahajaniScript => // 113
     {return String::from("MahajaniScript");}
     QLocale__ManichaeanScript => // 114
     {return String::from("ManichaeanScript");}
     QLocale__MendeKikakuiScript => // 115
     {return String::from("MendeKikakuiScript");}
     QLocale__ModiScript => // 116
     {return String::from("ModiScript");}
     QLocale__MroScript => // 117
     {return String::from("MroScript");}
     QLocale__OldNorthArabianScript => // 118
     {return String::from("OldNorthArabianScript");}
     QLocale__NabataeanScript => // 119
     {return String::from("NabataeanScript");}
     QLocale__PalmyreneScript => // 120
     {return String::from("PalmyreneScript");}
     QLocale__PauCinHauScript => // 121
     {return String::from("PauCinHauScript");}
     QLocale__OldPermicScript => // 122
     {return String::from("OldPermicScript");}
     QLocale__PsalterPahlaviScript => // 123
     {return String::from("PsalterPahlaviScript");}
     QLocale__SiddhamScript => // 124
     {return String::from("SiddhamScript");}
     QLocale__KhudawadiScript => // 125
     {return String::from("KhudawadiScript");}
     QLocale__TirhutaScript => // 126
     {return String::from("TirhutaScript");}
     QLocale__VarangKshitiScript => // 127
     {return String::from("VarangKshitiScript");}
     QLocale__AhomScript => // 128
     {return String::from("AhomScript");}
     QLocale__AnatolianHieroglyphsScript => // 129
     {return String::from("AnatolianHieroglyphsScript");}
     QLocale__HatranScript => // 130
     {return String::from("HatranScript");}
     QLocale__MultaniScript => // 131
     {return String::from("MultaniScript");}
     QLocale__OldHungarianScript => // 132
     {return String::from("OldHungarianScript");}
     QLocale__SignWritingScript => // 133
     {return String::from("SignWritingScript");}
     QLocale__AdlamScript => // 134
     {return String::from("AdlamScript");}
     QLocale__BhaiksukiScript => // 135
     {return String::from("BhaiksukiScript");}
     QLocale__MarchenScript => // 136
     {return String::from("MarchenScript");}
     QLocale__NewaScript => // 137
     {return String::from("NewaScript");}
     QLocale__OsageScript => // 138
     {return String::from("OsageScript");}
     QLocale__TangutScript => // 139
     {return String::from("TangutScript");}
     QLocale__HanWithBopomofoScript => // 140
     {return String::from("HanWithBopomofoScript");}
     QLocale__JamoScript => // 141
     {return String::from("JamoScript,LastScript");}
    // QLocale__SimplifiedChineseScript => // 5
    // {return String::from("");}
    // QLocale__TraditionalChineseScript => // 6
    // {return String::from("");}
    // QLocale__LastScript => // 141
    // {return String::from("");}
  _ => {return format!("{}", val);}
}
}
pub fn QLocale_ScriptItemName_s(val: i32) ->String {
  //var nilthis *QLocale
  //return nilthis.ScriptItemName(val);
  return QLocale_ScriptItemName(val);
}


/*
This enumerated type is used to specify a country.

QLocale::DemocraticRepublicOfCongoCongoKinshasaObsolete, please use CongoKinshasa
QLocale::PeoplesRepublicOfCongoCongoBrazzavilleObsolete, please use CongoBrazzaville
QLocale::DemocraticRepublicOfKoreaNorthKoreaObsolete, please use NorthKorea
QLocale::RepublicOfKoreaSouthKoreaObsolete, please use SouthKorea
QLocale::RussianFederationRussiasame as Russia
QLocale::SyrianArabRepublicSyriaObsolete, please use Syria
QLocale::TokelauTokelauCountryObsolete, please use TokelauCountry
QLocale::TuvaluTuvaluCountryObsolete, please use TuvaluCountry


See also country() and countryToString().

*/
pub type QLocale__Country = i32;
//  
pub const QLocale__AnyCountry :QLocale__Country = 0;
//  
pub const QLocale__Afghanistan :QLocale__Country = 1;
//  
pub const QLocale__Albania :QLocale__Country = 2;
//  
pub const QLocale__Algeria :QLocale__Country = 3;
//  
pub const QLocale__AmericanSamoa :QLocale__Country = 4;
//  
pub const QLocale__Andorra :QLocale__Country = 5;
//  
pub const QLocale__Angola :QLocale__Country = 6;
//  
pub const QLocale__Anguilla :QLocale__Country = 7;
//  
pub const QLocale__Antarctica :QLocale__Country = 8;
//  
pub const QLocale__AntiguaAndBarbuda :QLocale__Country = 9;
// 
pub const QLocale__Argentina :QLocale__Country = 10;
// 
pub const QLocale__Armenia :QLocale__Country = 11;
// 
pub const QLocale__Aruba :QLocale__Country = 12;
// 
pub const QLocale__Australia :QLocale__Country = 13;
// 
pub const QLocale__Austria :QLocale__Country = 14;
// 
pub const QLocale__Azerbaijan :QLocale__Country = 15;
// 
pub const QLocale__Bahamas :QLocale__Country = 16;
// 
pub const QLocale__Bahrain :QLocale__Country = 17;
// 
pub const QLocale__Bangladesh :QLocale__Country = 18;
// 
pub const QLocale__Barbados :QLocale__Country = 19;
// 
pub const QLocale__Belarus :QLocale__Country = 20;
// 
pub const QLocale__Belgium :QLocale__Country = 21;
// 
pub const QLocale__Belize :QLocale__Country = 22;
// 
pub const QLocale__Benin :QLocale__Country = 23;
// 
pub const QLocale__Bermuda :QLocale__Country = 24;
// 
pub const QLocale__Bhutan :QLocale__Country = 25;
// 
pub const QLocale__Bolivia :QLocale__Country = 26;
// 
pub const QLocale__BosniaAndHerzegowina :QLocale__Country = 27;
// 
pub const QLocale__Botswana :QLocale__Country = 28;
// 
pub const QLocale__BouvetIsland :QLocale__Country = 29;
// 
pub const QLocale__Brazil :QLocale__Country = 30;
// 
pub const QLocale__BritishIndianOceanTerritory :QLocale__Country = 31;
// 
pub const QLocale__Brunei :QLocale__Country = 32;
// 
pub const QLocale__Bulgaria :QLocale__Country = 33;
// 
pub const QLocale__BurkinaFaso :QLocale__Country = 34;
// 
pub const QLocale__Burundi :QLocale__Country = 35;
// 
pub const QLocale__Cambodia :QLocale__Country = 36;
// 
pub const QLocale__Cameroon :QLocale__Country = 37;
// 
pub const QLocale__Canada :QLocale__Country = 38;
// 
pub const QLocale__CapeVerde :QLocale__Country = 39;
// 
pub const QLocale__CaymanIslands :QLocale__Country = 40;
// 
pub const QLocale__CentralAfricanRepublic :QLocale__Country = 41;
// 
pub const QLocale__Chad :QLocale__Country = 42;
// 
pub const QLocale__Chile :QLocale__Country = 43;
// 
pub const QLocale__China :QLocale__Country = 44;
// 
pub const QLocale__ChristmasIsland :QLocale__Country = 45;
// 
pub const QLocale__CocosIslands :QLocale__Country = 46;
// 
pub const QLocale__Colombia :QLocale__Country = 47;
// 
pub const QLocale__Comoros :QLocale__Country = 48;
// 
pub const QLocale__CongoKinshasa :QLocale__Country = 49;
// 
pub const QLocale__CongoBrazzaville :QLocale__Country = 50;
// 
pub const QLocale__CookIslands :QLocale__Country = 51;
// 
pub const QLocale__CostaRica :QLocale__Country = 52;
// 
pub const QLocale__IvoryCoast :QLocale__Country = 53;
// 
pub const QLocale__Croatia :QLocale__Country = 54;
// 
pub const QLocale__Cuba :QLocale__Country = 55;
// 
pub const QLocale__Cyprus :QLocale__Country = 56;
// 
pub const QLocale__CzechRepublic :QLocale__Country = 57;
// 
pub const QLocale__Denmark :QLocale__Country = 58;
// 
pub const QLocale__Djibouti :QLocale__Country = 59;
// 
pub const QLocale__Dominica :QLocale__Country = 60;
// 
pub const QLocale__DominicanRepublic :QLocale__Country = 61;
// 
pub const QLocale__EastTimor :QLocale__Country = 62;
// 
pub const QLocale__Ecuador :QLocale__Country = 63;
// 
pub const QLocale__Egypt :QLocale__Country = 64;
// 
pub const QLocale__ElSalvador :QLocale__Country = 65;
// 
pub const QLocale__EquatorialGuinea :QLocale__Country = 66;
// 
pub const QLocale__Eritrea :QLocale__Country = 67;
// 
pub const QLocale__Estonia :QLocale__Country = 68;
// 
pub const QLocale__Ethiopia :QLocale__Country = 69;
// 
pub const QLocale__FalklandIslands :QLocale__Country = 70;
// 
pub const QLocale__FaroeIslands :QLocale__Country = 71;
// 
pub const QLocale__Fiji :QLocale__Country = 72;
// 
pub const QLocale__Finland :QLocale__Country = 73;
// 
pub const QLocale__France :QLocale__Country = 74;
// 
pub const QLocale__Guernsey :QLocale__Country = 75;
// 
pub const QLocale__FrenchGuiana :QLocale__Country = 76;
// 
pub const QLocale__FrenchPolynesia :QLocale__Country = 77;
// 
pub const QLocale__FrenchSouthernTerritories :QLocale__Country = 78;
// 
pub const QLocale__Gabon :QLocale__Country = 79;
// 
pub const QLocale__Gambia :QLocale__Country = 80;
// 
pub const QLocale__Georgia :QLocale__Country = 81;
// 
pub const QLocale__Germany :QLocale__Country = 82;
// 
pub const QLocale__Ghana :QLocale__Country = 83;
// 
pub const QLocale__Gibraltar :QLocale__Country = 84;
// 
pub const QLocale__Greece :QLocale__Country = 85;
// 
pub const QLocale__Greenland :QLocale__Country = 86;
// 
pub const QLocale__Grenada :QLocale__Country = 87;
// 
pub const QLocale__Guadeloupe :QLocale__Country = 88;
// 
pub const QLocale__Guam :QLocale__Country = 89;
// 
pub const QLocale__Guatemala :QLocale__Country = 90;
// 
pub const QLocale__Guinea :QLocale__Country = 91;
// 
pub const QLocale__GuineaBissau :QLocale__Country = 92;
// 
pub const QLocale__Guyana :QLocale__Country = 93;
// 
pub const QLocale__Haiti :QLocale__Country = 94;
// 
pub const QLocale__HeardAndMcDonaldIslands :QLocale__Country = 95;
// 
pub const QLocale__Honduras :QLocale__Country = 96;
// 
pub const QLocale__HongKong :QLocale__Country = 97;
// 
pub const QLocale__Hungary :QLocale__Country = 98;
// 
pub const QLocale__Iceland :QLocale__Country = 99;
// 
pub const QLocale__India :QLocale__Country = 100;
// 
pub const QLocale__Indonesia :QLocale__Country = 101;
// 
pub const QLocale__Iran :QLocale__Country = 102;
// 
pub const QLocale__Iraq :QLocale__Country = 103;
// 
pub const QLocale__Ireland :QLocale__Country = 104;
// 
pub const QLocale__Israel :QLocale__Country = 105;
// 
pub const QLocale__Italy :QLocale__Country = 106;
// 
pub const QLocale__Jamaica :QLocale__Country = 107;
// 
pub const QLocale__Japan :QLocale__Country = 108;
// 
pub const QLocale__Jordan :QLocale__Country = 109;
// 
pub const QLocale__Kazakhstan :QLocale__Country = 110;
// 
pub const QLocale__Kenya :QLocale__Country = 111;
// 
pub const QLocale__Kiribati :QLocale__Country = 112;
// 
pub const QLocale__NorthKorea :QLocale__Country = 113;
// 
pub const QLocale__SouthKorea :QLocale__Country = 114;
// 
pub const QLocale__Kuwait :QLocale__Country = 115;
// 
pub const QLocale__Kyrgyzstan :QLocale__Country = 116;
// 
pub const QLocale__Laos :QLocale__Country = 117;
// 
pub const QLocale__Latvia :QLocale__Country = 118;
// 
pub const QLocale__Lebanon :QLocale__Country = 119;
// 
pub const QLocale__Lesotho :QLocale__Country = 120;
// 
pub const QLocale__Liberia :QLocale__Country = 121;
// 
pub const QLocale__Libya :QLocale__Country = 122;
// 
pub const QLocale__Liechtenstein :QLocale__Country = 123;
// 
pub const QLocale__Lithuania :QLocale__Country = 124;
// 
pub const QLocale__Luxembourg :QLocale__Country = 125;
// 
pub const QLocale__Macau :QLocale__Country = 126;
// 
pub const QLocale__Macedonia :QLocale__Country = 127;
// 
pub const QLocale__Madagascar :QLocale__Country = 128;
// 
pub const QLocale__Malawi :QLocale__Country = 129;
// 
pub const QLocale__Malaysia :QLocale__Country = 130;
// 
pub const QLocale__Maldives :QLocale__Country = 131;
// 
pub const QLocale__Mali :QLocale__Country = 132;
// 
pub const QLocale__Malta :QLocale__Country = 133;
// 
pub const QLocale__MarshallIslands :QLocale__Country = 134;
// 
pub const QLocale__Martinique :QLocale__Country = 135;
// 
pub const QLocale__Mauritania :QLocale__Country = 136;
// 
pub const QLocale__Mauritius :QLocale__Country = 137;
// 
pub const QLocale__Mayotte :QLocale__Country = 138;
// 
pub const QLocale__Mexico :QLocale__Country = 139;
// 
pub const QLocale__Micronesia :QLocale__Country = 140;
// 
pub const QLocale__Moldova :QLocale__Country = 141;
// 
pub const QLocale__Monaco :QLocale__Country = 142;
// 
pub const QLocale__Mongolia :QLocale__Country = 143;
// 
pub const QLocale__Montserrat :QLocale__Country = 144;
// 
pub const QLocale__Morocco :QLocale__Country = 145;
// 
pub const QLocale__Mozambique :QLocale__Country = 146;
// 
pub const QLocale__Myanmar :QLocale__Country = 147;
// 
pub const QLocale__Namibia :QLocale__Country = 148;
// 
pub const QLocale__NauruCountry :QLocale__Country = 149;
// 
pub const QLocale__Nepal :QLocale__Country = 150;
// 
pub const QLocale__Netherlands :QLocale__Country = 151;
// 
pub const QLocale__CuraSao :QLocale__Country = 152;
// 
pub const QLocale__NewCaledonia :QLocale__Country = 153;
// 
pub const QLocale__NewZealand :QLocale__Country = 154;
// 
pub const QLocale__Nicaragua :QLocale__Country = 155;
// 
pub const QLocale__Niger :QLocale__Country = 156;
// 
pub const QLocale__Nigeria :QLocale__Country = 157;
// 
pub const QLocale__Niue :QLocale__Country = 158;
// 
pub const QLocale__NorfolkIsland :QLocale__Country = 159;
// 
pub const QLocale__NorthernMarianaIslands :QLocale__Country = 160;
// 
pub const QLocale__Norway :QLocale__Country = 161;
// 
pub const QLocale__Oman :QLocale__Country = 162;
// 
pub const QLocale__Pakistan :QLocale__Country = 163;
// 
pub const QLocale__Palau :QLocale__Country = 164;
// 
pub const QLocale__PalestinianTerritories :QLocale__Country = 165;
// 
pub const QLocale__Panama :QLocale__Country = 166;
// 
pub const QLocale__PapuaNewGuinea :QLocale__Country = 167;
// 
pub const QLocale__Paraguay :QLocale__Country = 168;
// 
pub const QLocale__Peru :QLocale__Country = 169;
// 
pub const QLocale__Philippines :QLocale__Country = 170;
// 
pub const QLocale__Pitcairn :QLocale__Country = 171;
// 
pub const QLocale__Poland :QLocale__Country = 172;
// 
pub const QLocale__Portugal :QLocale__Country = 173;
// 
pub const QLocale__PuertoRico :QLocale__Country = 174;
// 
pub const QLocale__Qatar :QLocale__Country = 175;
// 
pub const QLocale__Reunion :QLocale__Country = 176;
// 
pub const QLocale__Romania :QLocale__Country = 177;
// 
pub const QLocale__Russia :QLocale__Country = 178;
// 
pub const QLocale__Rwanda :QLocale__Country = 179;
// 
pub const QLocale__SaintKittsAndNevis :QLocale__Country = 180;
// 
pub const QLocale__SaintLucia :QLocale__Country = 181;
// 
pub const QLocale__SaintVincentAndTheGrenadines :QLocale__Country = 182;
// 
pub const QLocale__Samoa :QLocale__Country = 183;
// 
pub const QLocale__SanMarino :QLocale__Country = 184;
// 
pub const QLocale__SaoTomeAndPrincipe :QLocale__Country = 185;
// 
pub const QLocale__SaudiArabia :QLocale__Country = 186;
// 
pub const QLocale__Senegal :QLocale__Country = 187;
// 
pub const QLocale__Seychelles :QLocale__Country = 188;
// 
pub const QLocale__SierraLeone :QLocale__Country = 189;
// 
pub const QLocale__Singapore :QLocale__Country = 190;
// 
pub const QLocale__Slovakia :QLocale__Country = 191;
// 
pub const QLocale__Slovenia :QLocale__Country = 192;
// 
pub const QLocale__SolomonIslands :QLocale__Country = 193;
// 
pub const QLocale__Somalia :QLocale__Country = 194;
// 
pub const QLocale__SouthAfrica :QLocale__Country = 195;
// 
pub const QLocale__SouthGeorgiaAndTheSouthSandwichIslands :QLocale__Country = 196;
// 
pub const QLocale__Spain :QLocale__Country = 197;
// 
pub const QLocale__SriLanka :QLocale__Country = 198;
// 
pub const QLocale__SaintHelena :QLocale__Country = 199;
// 
pub const QLocale__SaintPierreAndMiquelon :QLocale__Country = 200;
// 
pub const QLocale__Sudan :QLocale__Country = 201;
// 
pub const QLocale__Suriname :QLocale__Country = 202;
// 
pub const QLocale__SvalbardAndJanMayenIslands :QLocale__Country = 203;
// 
pub const QLocale__Swaziland :QLocale__Country = 204;
// 
pub const QLocale__Sweden :QLocale__Country = 205;
// 
pub const QLocale__Switzerland :QLocale__Country = 206;
// 
pub const QLocale__Syria :QLocale__Country = 207;
// 
pub const QLocale__Taiwan :QLocale__Country = 208;
// 
pub const QLocale__Tajikistan :QLocale__Country = 209;
// 
pub const QLocale__Tanzania :QLocale__Country = 210;
// 
pub const QLocale__Thailand :QLocale__Country = 211;
// 
pub const QLocale__Togo :QLocale__Country = 212;
// 
pub const QLocale__TokelauCountry :QLocale__Country = 213;
// 
pub const QLocale__Tonga :QLocale__Country = 214;
// 
pub const QLocale__TrinidadAndTobago :QLocale__Country = 215;
// 
pub const QLocale__Tunisia :QLocale__Country = 216;
// 
pub const QLocale__Turkey :QLocale__Country = 217;
// 
pub const QLocale__Turkmenistan :QLocale__Country = 218;
// 
pub const QLocale__TurksAndCaicosIslands :QLocale__Country = 219;
// 
pub const QLocale__TuvaluCountry :QLocale__Country = 220;
// 
pub const QLocale__Uganda :QLocale__Country = 221;
// 
pub const QLocale__Ukraine :QLocale__Country = 222;
// 
pub const QLocale__UnitedArabEmirates :QLocale__Country = 223;
// 
pub const QLocale__UnitedKingdom :QLocale__Country = 224;
// 
pub const QLocale__UnitedStates :QLocale__Country = 225;
// 
pub const QLocale__UnitedStatesMinorOutlyingIslands :QLocale__Country = 226;
// 
pub const QLocale__Uruguay :QLocale__Country = 227;
// 
pub const QLocale__Uzbekistan :QLocale__Country = 228;
// 
pub const QLocale__Vanuatu :QLocale__Country = 229;
// 
pub const QLocale__VaticanCityState :QLocale__Country = 230;
// 
pub const QLocale__Venezuela :QLocale__Country = 231;
// 
pub const QLocale__Vietnam :QLocale__Country = 232;
// 
pub const QLocale__BritishVirginIslands :QLocale__Country = 233;
// 
pub const QLocale__UnitedStatesVirginIslands :QLocale__Country = 234;
// 
pub const QLocale__WallisAndFutunaIslands :QLocale__Country = 235;
// 
pub const QLocale__WesternSahara :QLocale__Country = 236;
// 
pub const QLocale__Yemen :QLocale__Country = 237;
// 
pub const QLocale__CanaryIslands :QLocale__Country = 238;
// 
pub const QLocale__Zambia :QLocale__Country = 239;
// 
pub const QLocale__Zimbabwe :QLocale__Country = 240;
// 
pub const QLocale__ClippertonIsland :QLocale__Country = 241;
// 
pub const QLocale__Montenegro :QLocale__Country = 242;
// 
pub const QLocale__Serbia :QLocale__Country = 243;
// 
pub const QLocale__SaintBarthelemy :QLocale__Country = 244;
// 
pub const QLocale__SaintMartin :QLocale__Country = 245;
// 
pub const QLocale__LatinAmericaAndTheCaribbean :QLocale__Country = 246;
// 
pub const QLocale__AscensionIsland :QLocale__Country = 247;
// 
pub const QLocale__AlandIslands :QLocale__Country = 248;
// 
pub const QLocale__DiegoGarcia :QLocale__Country = 249;
// 
pub const QLocale__CeutaAndMelilla :QLocale__Country = 250;
// 
pub const QLocale__IsleOfMan :QLocale__Country = 251;
// 
pub const QLocale__Jersey :QLocale__Country = 252;
// 
pub const QLocale__TristanDaCunha :QLocale__Country = 253;
// 
pub const QLocale__SouthSudan :QLocale__Country = 254;
// 
pub const QLocale__Bonaire :QLocale__Country = 255;
// 
pub const QLocale__SintMaarten :QLocale__Country = 256;
// 
pub const QLocale__Kosovo :QLocale__Country = 257;
// 
pub const QLocale__EuropeanUnion :QLocale__Country = 258;
// 
pub const QLocale__OutlyingOceania :QLocale__Country = 259;
// 
pub const QLocale__Tokelau :QLocale__Country = 213;
// 
pub const QLocale__Tuvalu :QLocale__Country = 220;
// 
pub const QLocale__DemocraticRepublicOfCongo :QLocale__Country = 49;
// 
pub const QLocale__PeoplesRepublicOfCongo :QLocale__Country = 50;
// 
pub const QLocale__DemocraticRepublicOfKorea :QLocale__Country = 113;
// 
pub const QLocale__RepublicOfKorea :QLocale__Country = 114;
// 
pub const QLocale__RussianFederation :QLocale__Country = 178;
// 
pub const QLocale__SyrianArabRepublic :QLocale__Country = 207;
// 
pub const QLocale__LastCountry :QLocale__Country = 259;
pub fn QLocale_CountryItemName(val: i32) ->String {
  match val {
     QLocale__AnyCountry => // 0
     {return String::from("AnyCountry");}
     QLocale__Afghanistan => // 1
     {return String::from("Afghanistan");}
     QLocale__Albania => // 2
     {return String::from("Albania");}
     QLocale__Algeria => // 3
     {return String::from("Algeria");}
     QLocale__AmericanSamoa => // 4
     {return String::from("AmericanSamoa");}
     QLocale__Andorra => // 5
     {return String::from("Andorra");}
     QLocale__Angola => // 6
     {return String::from("Angola");}
     QLocale__Anguilla => // 7
     {return String::from("Anguilla");}
     QLocale__Antarctica => // 8
     {return String::from("Antarctica");}
     QLocale__AntiguaAndBarbuda => // 9
     {return String::from("AntiguaAndBarbuda");}
     QLocale__Argentina => // 10
     {return String::from("Argentina");}
     QLocale__Armenia => // 11
     {return String::from("Armenia");}
     QLocale__Aruba => // 12
     {return String::from("Aruba");}
     QLocale__Australia => // 13
     {return String::from("Australia");}
     QLocale__Austria => // 14
     {return String::from("Austria");}
     QLocale__Azerbaijan => // 15
     {return String::from("Azerbaijan");}
     QLocale__Bahamas => // 16
     {return String::from("Bahamas");}
     QLocale__Bahrain => // 17
     {return String::from("Bahrain");}
     QLocale__Bangladesh => // 18
     {return String::from("Bangladesh");}
     QLocale__Barbados => // 19
     {return String::from("Barbados");}
     QLocale__Belarus => // 20
     {return String::from("Belarus");}
     QLocale__Belgium => // 21
     {return String::from("Belgium");}
     QLocale__Belize => // 22
     {return String::from("Belize");}
     QLocale__Benin => // 23
     {return String::from("Benin");}
     QLocale__Bermuda => // 24
     {return String::from("Bermuda");}
     QLocale__Bhutan => // 25
     {return String::from("Bhutan");}
     QLocale__Bolivia => // 26
     {return String::from("Bolivia");}
     QLocale__BosniaAndHerzegowina => // 27
     {return String::from("BosniaAndHerzegowina");}
     QLocale__Botswana => // 28
     {return String::from("Botswana");}
     QLocale__BouvetIsland => // 29
     {return String::from("BouvetIsland");}
     QLocale__Brazil => // 30
     {return String::from("Brazil");}
     QLocale__BritishIndianOceanTerritory => // 31
     {return String::from("BritishIndianOceanTerritory");}
     QLocale__Brunei => // 32
     {return String::from("Brunei");}
     QLocale__Bulgaria => // 33
     {return String::from("Bulgaria");}
     QLocale__BurkinaFaso => // 34
     {return String::from("BurkinaFaso");}
     QLocale__Burundi => // 35
     {return String::from("Burundi");}
     QLocale__Cambodia => // 36
     {return String::from("Cambodia");}
     QLocale__Cameroon => // 37
     {return String::from("Cameroon");}
     QLocale__Canada => // 38
     {return String::from("Canada");}
     QLocale__CapeVerde => // 39
     {return String::from("CapeVerde");}
     QLocale__CaymanIslands => // 40
     {return String::from("CaymanIslands");}
     QLocale__CentralAfricanRepublic => // 41
     {return String::from("CentralAfricanRepublic");}
     QLocale__Chad => // 42
     {return String::from("Chad");}
     QLocale__Chile => // 43
     {return String::from("Chile");}
     QLocale__China => // 44
     {return String::from("China");}
     QLocale__ChristmasIsland => // 45
     {return String::from("ChristmasIsland");}
     QLocale__CocosIslands => // 46
     {return String::from("CocosIslands");}
     QLocale__Colombia => // 47
     {return String::from("Colombia");}
     QLocale__Comoros => // 48
     {return String::from("Comoros");}
     QLocale__CongoKinshasa => // 49
     {return String::from("CongoKinshasa,DemocraticRepublicOfCongo");}
     QLocale__CongoBrazzaville => // 50
     {return String::from("CongoBrazzaville,PeoplesRepublicOfCongo");}
     QLocale__CookIslands => // 51
     {return String::from("CookIslands");}
     QLocale__CostaRica => // 52
     {return String::from("CostaRica");}
     QLocale__IvoryCoast => // 53
     {return String::from("IvoryCoast");}
     QLocale__Croatia => // 54
     {return String::from("Croatia");}
     QLocale__Cuba => // 55
     {return String::from("Cuba");}
     QLocale__Cyprus => // 56
     {return String::from("Cyprus");}
     QLocale__CzechRepublic => // 57
     {return String::from("CzechRepublic");}
     QLocale__Denmark => // 58
     {return String::from("Denmark");}
     QLocale__Djibouti => // 59
     {return String::from("Djibouti");}
     QLocale__Dominica => // 60
     {return String::from("Dominica");}
     QLocale__DominicanRepublic => // 61
     {return String::from("DominicanRepublic");}
     QLocale__EastTimor => // 62
     {return String::from("EastTimor");}
     QLocale__Ecuador => // 63
     {return String::from("Ecuador");}
     QLocale__Egypt => // 64
     {return String::from("Egypt");}
     QLocale__ElSalvador => // 65
     {return String::from("ElSalvador");}
     QLocale__EquatorialGuinea => // 66
     {return String::from("EquatorialGuinea");}
     QLocale__Eritrea => // 67
     {return String::from("Eritrea");}
     QLocale__Estonia => // 68
     {return String::from("Estonia");}
     QLocale__Ethiopia => // 69
     {return String::from("Ethiopia");}
     QLocale__FalklandIslands => // 70
     {return String::from("FalklandIslands");}
     QLocale__FaroeIslands => // 71
     {return String::from("FaroeIslands");}
     QLocale__Fiji => // 72
     {return String::from("Fiji");}
     QLocale__Finland => // 73
     {return String::from("Finland");}
     QLocale__France => // 74
     {return String::from("France");}
     QLocale__Guernsey => // 75
     {return String::from("Guernsey");}
     QLocale__FrenchGuiana => // 76
     {return String::from("FrenchGuiana");}
     QLocale__FrenchPolynesia => // 77
     {return String::from("FrenchPolynesia");}
     QLocale__FrenchSouthernTerritories => // 78
     {return String::from("FrenchSouthernTerritories");}
     QLocale__Gabon => // 79
     {return String::from("Gabon");}
     QLocale__Gambia => // 80
     {return String::from("Gambia");}
     QLocale__Georgia => // 81
     {return String::from("Georgia");}
     QLocale__Germany => // 82
     {return String::from("Germany");}
     QLocale__Ghana => // 83
     {return String::from("Ghana");}
     QLocale__Gibraltar => // 84
     {return String::from("Gibraltar");}
     QLocale__Greece => // 85
     {return String::from("Greece");}
     QLocale__Greenland => // 86
     {return String::from("Greenland");}
     QLocale__Grenada => // 87
     {return String::from("Grenada");}
     QLocale__Guadeloupe => // 88
     {return String::from("Guadeloupe");}
     QLocale__Guam => // 89
     {return String::from("Guam");}
     QLocale__Guatemala => // 90
     {return String::from("Guatemala");}
     QLocale__Guinea => // 91
     {return String::from("Guinea");}
     QLocale__GuineaBissau => // 92
     {return String::from("GuineaBissau");}
     QLocale__Guyana => // 93
     {return String::from("Guyana");}
     QLocale__Haiti => // 94
     {return String::from("Haiti");}
     QLocale__HeardAndMcDonaldIslands => // 95
     {return String::from("HeardAndMcDonaldIslands");}
     QLocale__Honduras => // 96
     {return String::from("Honduras");}
     QLocale__HongKong => // 97
     {return String::from("HongKong");}
     QLocale__Hungary => // 98
     {return String::from("Hungary");}
     QLocale__Iceland => // 99
     {return String::from("Iceland");}
     QLocale__India => // 100
     {return String::from("India");}
     QLocale__Indonesia => // 101
     {return String::from("Indonesia");}
     QLocale__Iran => // 102
     {return String::from("Iran");}
     QLocale__Iraq => // 103
     {return String::from("Iraq");}
     QLocale__Ireland => // 104
     {return String::from("Ireland");}
     QLocale__Israel => // 105
     {return String::from("Israel");}
     QLocale__Italy => // 106
     {return String::from("Italy");}
     QLocale__Jamaica => // 107
     {return String::from("Jamaica");}
     QLocale__Japan => // 108
     {return String::from("Japan");}
     QLocale__Jordan => // 109
     {return String::from("Jordan");}
     QLocale__Kazakhstan => // 110
     {return String::from("Kazakhstan");}
     QLocale__Kenya => // 111
     {return String::from("Kenya");}
     QLocale__Kiribati => // 112
     {return String::from("Kiribati");}
     QLocale__NorthKorea => // 113
     {return String::from("NorthKorea,DemocraticRepublicOfKorea");}
     QLocale__SouthKorea => // 114
     {return String::from("SouthKorea,RepublicOfKorea");}
     QLocale__Kuwait => // 115
     {return String::from("Kuwait");}
     QLocale__Kyrgyzstan => // 116
     {return String::from("Kyrgyzstan");}
     QLocale__Laos => // 117
     {return String::from("Laos");}
     QLocale__Latvia => // 118
     {return String::from("Latvia");}
     QLocale__Lebanon => // 119
     {return String::from("Lebanon");}
     QLocale__Lesotho => // 120
     {return String::from("Lesotho");}
     QLocale__Liberia => // 121
     {return String::from("Liberia");}
     QLocale__Libya => // 122
     {return String::from("Libya");}
     QLocale__Liechtenstein => // 123
     {return String::from("Liechtenstein");}
     QLocale__Lithuania => // 124
     {return String::from("Lithuania");}
     QLocale__Luxembourg => // 125
     {return String::from("Luxembourg");}
     QLocale__Macau => // 126
     {return String::from("Macau");}
     QLocale__Macedonia => // 127
     {return String::from("Macedonia");}
     QLocale__Madagascar => // 128
     {return String::from("Madagascar");}
     QLocale__Malawi => // 129
     {return String::from("Malawi");}
     QLocale__Malaysia => // 130
     {return String::from("Malaysia");}
     QLocale__Maldives => // 131
     {return String::from("Maldives");}
     QLocale__Mali => // 132
     {return String::from("Mali");}
     QLocale__Malta => // 133
     {return String::from("Malta");}
     QLocale__MarshallIslands => // 134
     {return String::from("MarshallIslands");}
     QLocale__Martinique => // 135
     {return String::from("Martinique");}
     QLocale__Mauritania => // 136
     {return String::from("Mauritania");}
     QLocale__Mauritius => // 137
     {return String::from("Mauritius");}
     QLocale__Mayotte => // 138
     {return String::from("Mayotte");}
     QLocale__Mexico => // 139
     {return String::from("Mexico");}
     QLocale__Micronesia => // 140
     {return String::from("Micronesia");}
     QLocale__Moldova => // 141
     {return String::from("Moldova");}
     QLocale__Monaco => // 142
     {return String::from("Monaco");}
     QLocale__Mongolia => // 143
     {return String::from("Mongolia");}
     QLocale__Montserrat => // 144
     {return String::from("Montserrat");}
     QLocale__Morocco => // 145
     {return String::from("Morocco");}
     QLocale__Mozambique => // 146
     {return String::from("Mozambique");}
     QLocale__Myanmar => // 147
     {return String::from("Myanmar");}
     QLocale__Namibia => // 148
     {return String::from("Namibia");}
     QLocale__NauruCountry => // 149
     {return String::from("NauruCountry");}
     QLocale__Nepal => // 150
     {return String::from("Nepal");}
     QLocale__Netherlands => // 151
     {return String::from("Netherlands");}
     QLocale__CuraSao => // 152
     {return String::from("CuraSao");}
     QLocale__NewCaledonia => // 153
     {return String::from("NewCaledonia");}
     QLocale__NewZealand => // 154
     {return String::from("NewZealand");}
     QLocale__Nicaragua => // 155
     {return String::from("Nicaragua");}
     QLocale__Niger => // 156
     {return String::from("Niger");}
     QLocale__Nigeria => // 157
     {return String::from("Nigeria");}
     QLocale__Niue => // 158
     {return String::from("Niue");}
     QLocale__NorfolkIsland => // 159
     {return String::from("NorfolkIsland");}
     QLocale__NorthernMarianaIslands => // 160
     {return String::from("NorthernMarianaIslands");}
     QLocale__Norway => // 161
     {return String::from("Norway");}
     QLocale__Oman => // 162
     {return String::from("Oman");}
     QLocale__Pakistan => // 163
     {return String::from("Pakistan");}
     QLocale__Palau => // 164
     {return String::from("Palau");}
     QLocale__PalestinianTerritories => // 165
     {return String::from("PalestinianTerritories");}
     QLocale__Panama => // 166
     {return String::from("Panama");}
     QLocale__PapuaNewGuinea => // 167
     {return String::from("PapuaNewGuinea");}
     QLocale__Paraguay => // 168
     {return String::from("Paraguay");}
     QLocale__Peru => // 169
     {return String::from("Peru");}
     QLocale__Philippines => // 170
     {return String::from("Philippines");}
     QLocale__Pitcairn => // 171
     {return String::from("Pitcairn");}
     QLocale__Poland => // 172
     {return String::from("Poland");}
     QLocale__Portugal => // 173
     {return String::from("Portugal");}
     QLocale__PuertoRico => // 174
     {return String::from("PuertoRico");}
     QLocale__Qatar => // 175
     {return String::from("Qatar");}
     QLocale__Reunion => // 176
     {return String::from("Reunion");}
     QLocale__Romania => // 177
     {return String::from("Romania");}
     QLocale__Russia => // 178
     {return String::from("Russia,RussianFederation");}
     QLocale__Rwanda => // 179
     {return String::from("Rwanda");}
     QLocale__SaintKittsAndNevis => // 180
     {return String::from("SaintKittsAndNevis");}
     QLocale__SaintLucia => // 181
     {return String::from("SaintLucia");}
     QLocale__SaintVincentAndTheGrenadines => // 182
     {return String::from("SaintVincentAndTheGrenadines");}
     QLocale__Samoa => // 183
     {return String::from("Samoa");}
     QLocale__SanMarino => // 184
     {return String::from("SanMarino");}
     QLocale__SaoTomeAndPrincipe => // 185
     {return String::from("SaoTomeAndPrincipe");}
     QLocale__SaudiArabia => // 186
     {return String::from("SaudiArabia");}
     QLocale__Senegal => // 187
     {return String::from("Senegal");}
     QLocale__Seychelles => // 188
     {return String::from("Seychelles");}
     QLocale__SierraLeone => // 189
     {return String::from("SierraLeone");}
     QLocale__Singapore => // 190
     {return String::from("Singapore");}
     QLocale__Slovakia => // 191
     {return String::from("Slovakia");}
     QLocale__Slovenia => // 192
     {return String::from("Slovenia");}
     QLocale__SolomonIslands => // 193
     {return String::from("SolomonIslands");}
     QLocale__Somalia => // 194
     {return String::from("Somalia");}
     QLocale__SouthAfrica => // 195
     {return String::from("SouthAfrica");}
     QLocale__SouthGeorgiaAndTheSouthSandwichIslands => // 196
     {return String::from("SouthGeorgiaAndTheSouthSandwichIslands");}
     QLocale__Spain => // 197
     {return String::from("Spain");}
     QLocale__SriLanka => // 198
     {return String::from("SriLanka");}
     QLocale__SaintHelena => // 199
     {return String::from("SaintHelena");}
     QLocale__SaintPierreAndMiquelon => // 200
     {return String::from("SaintPierreAndMiquelon");}
     QLocale__Sudan => // 201
     {return String::from("Sudan");}
     QLocale__Suriname => // 202
     {return String::from("Suriname");}
     QLocale__SvalbardAndJanMayenIslands => // 203
     {return String::from("SvalbardAndJanMayenIslands");}
     QLocale__Swaziland => // 204
     {return String::from("Swaziland");}
     QLocale__Sweden => // 205
     {return String::from("Sweden");}
     QLocale__Switzerland => // 206
     {return String::from("Switzerland");}
     QLocale__Syria => // 207
     {return String::from("Syria,SyrianArabRepublic");}
     QLocale__Taiwan => // 208
     {return String::from("Taiwan");}
     QLocale__Tajikistan => // 209
     {return String::from("Tajikistan");}
     QLocale__Tanzania => // 210
     {return String::from("Tanzania");}
     QLocale__Thailand => // 211
     {return String::from("Thailand");}
     QLocale__Togo => // 212
     {return String::from("Togo");}
     QLocale__TokelauCountry => // 213
     {return String::from("TokelauCountry,Tokelau");}
     QLocale__Tonga => // 214
     {return String::from("Tonga");}
     QLocale__TrinidadAndTobago => // 215
     {return String::from("TrinidadAndTobago");}
     QLocale__Tunisia => // 216
     {return String::from("Tunisia");}
     QLocale__Turkey => // 217
     {return String::from("Turkey");}
     QLocale__Turkmenistan => // 218
     {return String::from("Turkmenistan");}
     QLocale__TurksAndCaicosIslands => // 219
     {return String::from("TurksAndCaicosIslands");}
     QLocale__TuvaluCountry => // 220
     {return String::from("TuvaluCountry,Tuvalu");}
     QLocale__Uganda => // 221
     {return String::from("Uganda");}
     QLocale__Ukraine => // 222
     {return String::from("Ukraine");}
     QLocale__UnitedArabEmirates => // 223
     {return String::from("UnitedArabEmirates");}
     QLocale__UnitedKingdom => // 224
     {return String::from("UnitedKingdom");}
     QLocale__UnitedStates => // 225
     {return String::from("UnitedStates");}
     QLocale__UnitedStatesMinorOutlyingIslands => // 226
     {return String::from("UnitedStatesMinorOutlyingIslands");}
     QLocale__Uruguay => // 227
     {return String::from("Uruguay");}
     QLocale__Uzbekistan => // 228
     {return String::from("Uzbekistan");}
     QLocale__Vanuatu => // 229
     {return String::from("Vanuatu");}
     QLocale__VaticanCityState => // 230
     {return String::from("VaticanCityState");}
     QLocale__Venezuela => // 231
     {return String::from("Venezuela");}
     QLocale__Vietnam => // 232
     {return String::from("Vietnam");}
     QLocale__BritishVirginIslands => // 233
     {return String::from("BritishVirginIslands");}
     QLocale__UnitedStatesVirginIslands => // 234
     {return String::from("UnitedStatesVirginIslands");}
     QLocale__WallisAndFutunaIslands => // 235
     {return String::from("WallisAndFutunaIslands");}
     QLocale__WesternSahara => // 236
     {return String::from("WesternSahara");}
     QLocale__Yemen => // 237
     {return String::from("Yemen");}
     QLocale__CanaryIslands => // 238
     {return String::from("CanaryIslands");}
     QLocale__Zambia => // 239
     {return String::from("Zambia");}
     QLocale__Zimbabwe => // 240
     {return String::from("Zimbabwe");}
     QLocale__ClippertonIsland => // 241
     {return String::from("ClippertonIsland");}
     QLocale__Montenegro => // 242
     {return String::from("Montenegro");}
     QLocale__Serbia => // 243
     {return String::from("Serbia");}
     QLocale__SaintBarthelemy => // 244
     {return String::from("SaintBarthelemy");}
     QLocale__SaintMartin => // 245
     {return String::from("SaintMartin");}
     QLocale__LatinAmericaAndTheCaribbean => // 246
     {return String::from("LatinAmericaAndTheCaribbean");}
     QLocale__AscensionIsland => // 247
     {return String::from("AscensionIsland");}
     QLocale__AlandIslands => // 248
     {return String::from("AlandIslands");}
     QLocale__DiegoGarcia => // 249
     {return String::from("DiegoGarcia");}
     QLocale__CeutaAndMelilla => // 250
     {return String::from("CeutaAndMelilla");}
     QLocale__IsleOfMan => // 251
     {return String::from("IsleOfMan");}
     QLocale__Jersey => // 252
     {return String::from("Jersey");}
     QLocale__TristanDaCunha => // 253
     {return String::from("TristanDaCunha");}
     QLocale__SouthSudan => // 254
     {return String::from("SouthSudan");}
     QLocale__Bonaire => // 255
     {return String::from("Bonaire");}
     QLocale__SintMaarten => // 256
     {return String::from("SintMaarten");}
     QLocale__Kosovo => // 257
     {return String::from("Kosovo");}
     QLocale__EuropeanUnion => // 258
     {return String::from("EuropeanUnion");}
     QLocale__OutlyingOceania => // 259
     {return String::from("OutlyingOceania,LastCountry");}
    // QLocale__Tokelau => // 213
    // {return String::from("");}
    // QLocale__Tuvalu => // 220
    // {return String::from("");}
    // QLocale__DemocraticRepublicOfCongo => // 49
    // {return String::from("");}
    // QLocale__PeoplesRepublicOfCongo => // 50
    // {return String::from("");}
    // QLocale__DemocraticRepublicOfKorea => // 113
    // {return String::from("");}
    // QLocale__RepublicOfKorea => // 114
    // {return String::from("");}
    // QLocale__RussianFederation => // 178
    // {return String::from("");}
    // QLocale__SyrianArabRepublic => // 207
    // {return String::from("");}
    // QLocale__LastCountry => // 259
    // {return String::from("");}
  _ => {return format!("{}", val);}
}
}
pub fn QLocale_CountryItemName_s(val: i32) ->String {
  //var nilthis *QLocale
  //return nilthis.CountryItemName(val);
  return QLocale_CountryItemName(val);
}


/*
This enum defines which units are used for measurement.

QLocale::ImperialSystemImperialUSSystemProvided for compatibility. Same as ImperialUSSystem


This enum was introduced or modified in  Qt 4.4.

*/
pub type QLocale__MeasurementSystem = i32;
// This value indicates metric units, such as meters, centimeters and millimeters.
pub const QLocale__MetricSystem :QLocale__MeasurementSystem = 0;
// This value indicates imperial units, such as inches and miles as they are used in the United States.
pub const QLocale__ImperialUSSystem :QLocale__MeasurementSystem = 1;
// This value indicates imperial units, such as inches and miles as they are used in the United Kingdom.
pub const QLocale__ImperialUKSystem :QLocale__MeasurementSystem = 2;
// 
pub const QLocale__ImperialSystem :QLocale__MeasurementSystem = 1;
pub fn QLocale_MeasurementSystemItemName(val: i32) ->String {
  match val {
     QLocale__MetricSystem => // 0
     {return String::from("MetricSystem");}
     QLocale__ImperialUSSystem => // 1
     {return String::from("ImperialUSSystem,ImperialSystem");}
     QLocale__ImperialUKSystem => // 2
     {return String::from("ImperialUKSystem");}
    // QLocale__ImperialSystem => // 1
    // {return String::from("");}
  _ => {return format!("{}", val);}
}
}
pub fn QLocale_MeasurementSystemItemName_s(val: i32) ->String {
  //var nilthis *QLocale
  //return nilthis.MeasurementSystemItemName(val);
  return QLocale_MeasurementSystemItemName(val);
}


/*
This enum describes the types of format that can be used when converting QDate and QTime objects to strings.


*/
pub type QLocale__FormatType = i32;
// The long version of day and month names; for example, returning "January" as a month name.
pub const QLocale__LongFormat :QLocale__FormatType = 0;
// The short version of day and month names; for example, returning "Jan" as a month name.
pub const QLocale__ShortFormat :QLocale__FormatType = 1;
// A special version of day and month names for use when space is limited; for example, returning "J" as a month name. Note that the narrow format might contain the same text for different months and days or it can even be an empty string if the locale doesn't support narrow names, so you should avoid using it for date formatting. Also, for the system locale this format is the same as ShortFormat.
pub const QLocale__NarrowFormat :QLocale__FormatType = 2;
pub fn QLocale_FormatTypeItemName(val: i32) ->String {
  match val {
     QLocale__LongFormat => // 0
     {return String::from("LongFormat");}
     QLocale__ShortFormat => // 1
     {return String::from("ShortFormat");}
     QLocale__NarrowFormat => // 2
     {return String::from("NarrowFormat");}
  _ => {return format!("{}", val);}
}
}
pub fn QLocale_FormatTypeItemName_s(val: i32) ->String {
  //var nilthis *QLocale
  //return nilthis.FormatTypeItemName(val);
  return QLocale_FormatTypeItemName(val);
}


/*


*/
pub type QLocale__NumberOption = i32;
// 
pub const QLocale__DefaultNumberOptions :QLocale__NumberOption = 0;
// 
pub const QLocale__OmitGroupSeparator :QLocale__NumberOption = 1;
// 
pub const QLocale__RejectGroupSeparator :QLocale__NumberOption = 2;
// 
pub const QLocale__OmitLeadingZeroInExponent :QLocale__NumberOption = 4;
// 
pub const QLocale__RejectLeadingZeroInExponent :QLocale__NumberOption = 8;
// 
pub const QLocale__IncludeTrailingZeroesAfterDot :QLocale__NumberOption = 16;
// 
pub const QLocale__RejectTrailingZeroesAfterDot :QLocale__NumberOption = 32;
pub fn QLocale_NumberOptionItemName(val: i32) ->String {
  match val {
     QLocale__DefaultNumberOptions => // 0
     {return String::from("DefaultNumberOptions");}
     QLocale__OmitGroupSeparator => // 1
     {return String::from("OmitGroupSeparator");}
     QLocale__RejectGroupSeparator => // 2
     {return String::from("RejectGroupSeparator");}
     QLocale__OmitLeadingZeroInExponent => // 4
     {return String::from("OmitLeadingZeroInExponent");}
     QLocale__RejectLeadingZeroInExponent => // 8
     {return String::from("RejectLeadingZeroInExponent");}
     QLocale__IncludeTrailingZeroesAfterDot => // 16
     {return String::from("IncludeTrailingZeroesAfterDot");}
     QLocale__RejectTrailingZeroesAfterDot => // 32
     {return String::from("RejectTrailingZeroesAfterDot");}
  _ => {return format!("{}", val);}
}
}
pub fn QLocale_NumberOptionItemName_s(val: i32) ->String {
  //var nilthis *QLocale
  //return nilthis.NumberOptionItemName(val);
  return QLocale_NumberOptionItemName(val);
}


/*
This enum defines constants that can be given as precision to QString::number(), QByteArray::number(), and QLocale::toString() when converting floats or doubles, in order to express a variable number of digits as precision.



This enum was introduced or modified in  Qt 5.7.

See also toString(), QString, and QByteArray.

*/
pub type QLocale__FloatingPointPrecisionOption = i32;
// 
pub const QLocale__FloatingPointShortest :QLocale__FloatingPointPrecisionOption = -128;
pub fn QLocale_FloatingPointPrecisionOptionItemName(val: i32) ->String {
  match val {
     QLocale__FloatingPointShortest => // -128
     {return String::from("FloatingPointShortest");}
  _ => {return format!("{}", val);}
}
}
pub fn QLocale_FloatingPointPrecisionOptionItemName_s(val: i32) ->String {
  //var nilthis *QLocale
  //return nilthis.FloatingPointPrecisionOptionItemName(val);
  return QLocale_FloatingPointPrecisionOptionItemName(val);
}


/*
Specifies the format of the currency symbol.



This enum was introduced or modified in  Qt 4.8.

*/
pub type QLocale__CurrencySymbolFormat = i32;
// 
pub const QLocale__CurrencyIsoCode :QLocale__CurrencySymbolFormat = 0;
// a currency symbol.
pub const QLocale__CurrencySymbol :QLocale__CurrencySymbolFormat = 1;
// a user readable name of the currency.
pub const QLocale__CurrencyDisplayName :QLocale__CurrencySymbolFormat = 2;
pub fn QLocale_CurrencySymbolFormatItemName(val: i32) ->String {
  match val {
     QLocale__CurrencyIsoCode => // 0
     {return String::from("CurrencyIsoCode");}
     QLocale__CurrencySymbol => // 1
     {return String::from("CurrencySymbol");}
     QLocale__CurrencyDisplayName => // 2
     {return String::from("CurrencyDisplayName");}
  _ => {return format!("{}", val);}
}
}
pub fn QLocale_CurrencySymbolFormatItemName_s(val: i32) ->String {
  //var nilthis *QLocale
  //return nilthis.CurrencySymbolFormatItemName(val);
  return QLocale_CurrencySymbolFormatItemName(val);
}


/*


*/
pub type QLocale__DataSizeFormat = i32;
// 
pub const QLocale__DataSizeBase1000 :QLocale__DataSizeFormat = 1;
// 
pub const QLocale__DataSizeSIQuantifiers :QLocale__DataSizeFormat = 2;
// 
pub const QLocale__DataSizeIecFormat :QLocale__DataSizeFormat = 0;
// 
pub const QLocale__DataSizeTraditionalFormat :QLocale__DataSizeFormat = 2;
// 
pub const QLocale__DataSizeSIFormat :QLocale__DataSizeFormat = 3;
pub fn QLocale_DataSizeFormatItemName(val: i32) ->String {
  match val {
     QLocale__DataSizeBase1000 => // 1
     {return String::from("DataSizeBase1000");}
     QLocale__DataSizeSIQuantifiers => // 2
     {return String::from("DataSizeSIQuantifiers,DataSizeTraditionalFormat");}
     QLocale__DataSizeIecFormat => // 0
     {return String::from("DataSizeIecFormat");}
    // QLocale__DataSizeTraditionalFormat => // 2
    // {return String::from("");}
     QLocale__DataSizeSIFormat => // 3
     {return String::from("DataSizeSIFormat");}
  _ => {return format!("{}", val);}
}
}
pub fn QLocale_DataSizeFormatItemName_s(val: i32) ->String {
  //var nilthis *QLocale
  //return nilthis.DataSizeFormatItemName(val);
  return QLocale_DataSizeFormatItemName(val);
}


/*
This enum defines a set of possible styles for locale specific quotation.



This enum was introduced or modified in  Qt 4.8.

See also quoteString().

*/
pub type QLocale__QuotationStyle = i32;
// If this option is set, the standard quotation marks will be used to quote strings.
pub const QLocale__StandardQuotation :QLocale__QuotationStyle = 0;
// If this option is set, the alternate quotation marks will be used to quote strings.
pub const QLocale__AlternateQuotation :QLocale__QuotationStyle = 1;
pub fn QLocale_QuotationStyleItemName(val: i32) ->String {
  match val {
     QLocale__StandardQuotation => // 0
     {return String::from("StandardQuotation");}
     QLocale__AlternateQuotation => // 1
     {return String::from("AlternateQuotation");}
  _ => {return format!("{}", val);}
}
}
pub fn QLocale_QuotationStyleItemName_s(val: i32) ->String {
  //var nilthis *QLocale
  //return nilthis.QuotationStyleItemName(val);
  return QLocale_QuotationStyleItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
