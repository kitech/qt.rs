// auto generated, do not modify.
// created: Wed Dec 23 22:29:56 2015
// src-file: /QtCore/qsettings.h
// dst-file: /src/core/qsettings.rs
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
use super::qobject::QObject; // 773
use std::ops::Deref;
use super::qstring::QString; // 773
use super::qvariant::QVariant; // 773
use super::qtextcodec::QTextCodec; // 773
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  void QSettings::QSettings(QObject * parent);
  fn _ZN9QSettingsC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QSettings::isWritable();
  fn _ZNK9QSettings10isWritableEv(qthis: *mut c_void) -> c_char;
  // proto:  QString QSettings::fileName();
  fn _ZNK9QSettings8fileNameEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QSettings::fallbacksEnabled();
  fn _ZNK9QSettings16fallbacksEnabledEv(qthis: *mut c_void) -> c_char;
  // proto:  QString QSettings::applicationName();
  fn _ZNK9QSettings15applicationNameEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QSettings::sync();
  fn _ZN9QSettings4syncEv(qthis: *mut c_void);
  // proto:  void QSettings::setValue(const QString & key, const QVariant & value);
  fn _ZN9QSettings8setValueERK7QStringRK8QVariant(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QSettings::setArrayIndex(int i);
  fn _ZN9QSettings13setArrayIndexEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QSettings::QSettings(const QString & organization, const QString & application, QObject * parent);
  fn _ZN9QSettingsC1ERK7QStringS2_P7QObject(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void);
  // proto:  void QSettings::setIniCodec(QTextCodec * codec);
  fn _ZN9QSettings11setIniCodecEP10QTextCodec(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QSettings::setIniCodec(const char * codecName);
  fn _ZN9QSettings11setIniCodecEPKc(qthis: *mut c_void, arg0: *mut c_char);
  // proto:  int QSettings::beginReadArray(const QString & prefix);
  fn _ZN9QSettings14beginReadArrayERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> c_int;
  // proto:  void QSettings::clear();
  fn _ZN9QSettings5clearEv(qthis: *mut c_void);
  // proto:  void QSettings::~QSettings();
  fn _ZN9QSettingsD0Ev(qthis: *mut c_void);
  // proto:  QTextCodec * QSettings::iniCodec();
  fn _ZNK9QSettings8iniCodecEv(qthis: *mut c_void) -> *mut c_void;
  // proto: static void QSettings::setUserIniPath(const QString & dir);
  fn _ZN9QSettings14setUserIniPathERK7QString(arg0: *mut c_void);
  // proto:  QStringList QSettings::childGroups();
  fn _ZNK9QSettings11childGroupsEv(qthis: *mut c_void);
  // proto:  QVariant QSettings::value(const QString & key, const QVariant & defaultValue);
  fn _ZNK9QSettings5valueERK7QStringRK8QVariant(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  QString QSettings::organizationName();
  fn _ZNK9QSettings16organizationNameEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  const QMetaObject * QSettings::metaObject();
  fn _ZNK9QSettings10metaObjectEv(qthis: *mut c_void);
  // proto:  void QSettings::setFallbacksEnabled(bool b);
  fn _ZN9QSettings19setFallbacksEnabledEb(qthis: *mut c_void, arg0: c_char);
  // proto:  bool QSettings::contains(const QString & key);
  fn _ZNK9QSettings8containsERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  void QSettings::remove(const QString & key);
  fn _ZN9QSettings6removeERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QSettings::endGroup();
  fn _ZN9QSettings8endGroupEv(qthis: *mut c_void);
  // proto:  void QSettings::beginWriteArray(const QString & prefix, int size);
  fn _ZN9QSettings15beginWriteArrayERK7QStringi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int);
  // proto:  void QSettings::beginGroup(const QString & prefix);
  fn _ZN9QSettings10beginGroupERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QStringList QSettings::childKeys();
  fn _ZNK9QSettings9childKeysEv(qthis: *mut c_void);
  // proto:  void QSettings::QSettings(const QSettings & );
  fn _ZN9QSettingsC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QSettings::endArray();
  fn _ZN9QSettings8endArrayEv(qthis: *mut c_void);
  // proto: static void QSettings::setSystemIniPath(const QString & dir);
  fn _ZN9QSettings16setSystemIniPathERK7QString(arg0: *mut c_void);
  // proto:  QStringList QSettings::allKeys();
  fn _ZNK9QSettings7allKeysEv(qthis: *mut c_void);
  // proto:  QString QSettings::group();
  fn _ZNK9QSettings5groupEv(qthis: *mut c_void) -> *mut c_void;
} // <= ext block end

// body block begin =>
// class sizeof(QSettings)=1
pub struct QSettings {
  qbase: QObject,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QSettings {
  pub fn inheritFrom(qthis: *mut c_void) -> QSettings {
    return QSettings{qbase: QObject::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QSettings {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QSettings {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  void QSettings::QSettings(QObject * parent);
impl /*struct*/ QSettings {
  pub fn New<T: QSettings_New>(value: T) -> QSettings {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QSettings_New {
  fn New(self) -> QSettings;
}

  // proto:  void QSettings::QSettings(QObject * parent);
impl<'a> /*trait*/ QSettings_New for (&'a QObject) {
  fn New(self) -> QSettings {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSettingsC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QSettingsC1EP7QObject(qthis, arg0)};
    let rsthis = QSettings{/**/qbase: QObject::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QSettings::isWritable();
impl /*struct*/ QSettings {
  pub fn isWritable<RetType, T: QSettings_isWritable<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isWritable(self);
    // return 1;
  }
}

pub trait QSettings_isWritable<RetType> {
  fn isWritable(self , rsthis: & QSettings) -> RetType;
}

  // proto:  bool QSettings::isWritable();
impl<'a> /*trait*/ QSettings_isWritable<i8> for () {
  fn isWritable(self , rsthis: & QSettings) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QSettings10isWritableEv()};
    let mut ret = unsafe {_ZNK9QSettings10isWritableEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QString QSettings::fileName();
impl /*struct*/ QSettings {
  pub fn fileName<RetType, T: QSettings_fileName<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.fileName(self);
    // return 1;
  }
}

pub trait QSettings_fileName<RetType> {
  fn fileName(self , rsthis: & QSettings) -> RetType;
}

  // proto:  QString QSettings::fileName();
impl<'a> /*trait*/ QSettings_fileName<QString> for () {
  fn fileName(self , rsthis: & QSettings) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QSettings8fileNameEv()};
    let mut ret = unsafe {_ZNK9QSettings8fileNameEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QSettings::fallbacksEnabled();
impl /*struct*/ QSettings {
  pub fn fallbacksEnabled<RetType, T: QSettings_fallbacksEnabled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.fallbacksEnabled(self);
    // return 1;
  }
}

pub trait QSettings_fallbacksEnabled<RetType> {
  fn fallbacksEnabled(self , rsthis: & QSettings) -> RetType;
}

  // proto:  bool QSettings::fallbacksEnabled();
impl<'a> /*trait*/ QSettings_fallbacksEnabled<i8> for () {
  fn fallbacksEnabled(self , rsthis: & QSettings) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QSettings16fallbacksEnabledEv()};
    let mut ret = unsafe {_ZNK9QSettings16fallbacksEnabledEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QString QSettings::applicationName();
impl /*struct*/ QSettings {
  pub fn applicationName<RetType, T: QSettings_applicationName<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.applicationName(self);
    // return 1;
  }
}

pub trait QSettings_applicationName<RetType> {
  fn applicationName(self , rsthis: & QSettings) -> RetType;
}

  // proto:  QString QSettings::applicationName();
impl<'a> /*trait*/ QSettings_applicationName<QString> for () {
  fn applicationName(self , rsthis: & QSettings) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QSettings15applicationNameEv()};
    let mut ret = unsafe {_ZNK9QSettings15applicationNameEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QSettings::sync();
impl /*struct*/ QSettings {
  pub fn sync<RetType, T: QSettings_sync<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sync(self);
    // return 1;
  }
}

pub trait QSettings_sync<RetType> {
  fn sync(self , rsthis: & QSettings) -> RetType;
}

  // proto:  void QSettings::sync();
impl<'a> /*trait*/ QSettings_sync<()> for () {
  fn sync(self , rsthis: & QSettings) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSettings4syncEv()};
     unsafe {_ZN9QSettings4syncEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QSettings::setValue(const QString & key, const QVariant & value);
impl /*struct*/ QSettings {
  pub fn setValue<RetType, T: QSettings_setValue<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setValue(self);
    // return 1;
  }
}

pub trait QSettings_setValue<RetType> {
  fn setValue(self , rsthis: & QSettings) -> RetType;
}

  // proto:  void QSettings::setValue(const QString & key, const QVariant & value);
impl<'a> /*trait*/ QSettings_setValue<()> for (&'a QString, &'a QVariant) {
  fn setValue(self , rsthis: & QSettings) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSettings8setValueERK7QStringRK8QVariant()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN9QSettings8setValueERK7QStringRK8QVariant(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QSettings::setArrayIndex(int i);
impl /*struct*/ QSettings {
  pub fn setArrayIndex<RetType, T: QSettings_setArrayIndex<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setArrayIndex(self);
    // return 1;
  }
}

pub trait QSettings_setArrayIndex<RetType> {
  fn setArrayIndex(self , rsthis: & QSettings) -> RetType;
}

  // proto:  void QSettings::setArrayIndex(int i);
impl<'a> /*trait*/ QSettings_setArrayIndex<()> for (i32) {
  fn setArrayIndex(self , rsthis: & QSettings) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSettings13setArrayIndexEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QSettings13setArrayIndexEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QSettings::QSettings(const QString & organization, const QString & application, QObject * parent);
impl<'a> /*trait*/ QSettings_New for (&'a QString, &'a QString, &'a QObject) {
  fn New(self) -> QSettings {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSettingsC1ERK7QStringS2_P7QObject()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    unsafe {_ZN9QSettingsC1ERK7QStringS2_P7QObject(qthis, arg0, arg1, arg2)};
    let rsthis = QSettings{/**/qbase: QObject::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QSettings::setIniCodec(QTextCodec * codec);
impl /*struct*/ QSettings {
  pub fn setIniCodec<RetType, T: QSettings_setIniCodec<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setIniCodec(self);
    // return 1;
  }
}

pub trait QSettings_setIniCodec<RetType> {
  fn setIniCodec(self , rsthis: & QSettings) -> RetType;
}

  // proto:  void QSettings::setIniCodec(QTextCodec * codec);
impl<'a> /*trait*/ QSettings_setIniCodec<()> for (&'a QTextCodec) {
  fn setIniCodec(self , rsthis: & QSettings) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSettings11setIniCodecEP10QTextCodec()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QSettings11setIniCodecEP10QTextCodec(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QSettings::setIniCodec(const char * codecName);
impl<'a> /*trait*/ QSettings_setIniCodec<()> for (&'a  String) {
  fn setIniCodec(self , rsthis: & QSettings) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSettings11setIniCodecEPKc()};
    let arg0 = self.as_ptr()  as *mut c_char;
     unsafe {_ZN9QSettings11setIniCodecEPKc(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QSettings::beginReadArray(const QString & prefix);
impl /*struct*/ QSettings {
  pub fn beginReadArray<RetType, T: QSettings_beginReadArray<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.beginReadArray(self);
    // return 1;
  }
}

pub trait QSettings_beginReadArray<RetType> {
  fn beginReadArray(self , rsthis: & QSettings) -> RetType;
}

  // proto:  int QSettings::beginReadArray(const QString & prefix);
impl<'a> /*trait*/ QSettings_beginReadArray<i32> for (&'a QString) {
  fn beginReadArray(self , rsthis: & QSettings) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSettings14beginReadArrayERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN9QSettings14beginReadArrayERK7QString(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QSettings::clear();
impl /*struct*/ QSettings {
  pub fn clear<RetType, T: QSettings_clear<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clear(self);
    // return 1;
  }
}

pub trait QSettings_clear<RetType> {
  fn clear(self , rsthis: & QSettings) -> RetType;
}

  // proto:  void QSettings::clear();
impl<'a> /*trait*/ QSettings_clear<()> for () {
  fn clear(self , rsthis: & QSettings) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSettings5clearEv()};
     unsafe {_ZN9QSettings5clearEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QSettings::~QSettings();
impl /*struct*/ QSettings {
  pub fn Free<RetType, T: QSettings_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QSettings_Free<RetType> {
  fn Free(self , rsthis: & QSettings) -> RetType;
}

  // proto:  void QSettings::~QSettings();
impl<'a> /*trait*/ QSettings_Free<()> for () {
  fn Free(self , rsthis: & QSettings) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSettingsD0Ev()};
     unsafe {_ZN9QSettingsD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QTextCodec * QSettings::iniCodec();
impl /*struct*/ QSettings {
  pub fn iniCodec<RetType, T: QSettings_iniCodec<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.iniCodec(self);
    // return 1;
  }
}

pub trait QSettings_iniCodec<RetType> {
  fn iniCodec(self , rsthis: & QSettings) -> RetType;
}

  // proto:  QTextCodec * QSettings::iniCodec();
impl<'a> /*trait*/ QSettings_iniCodec<QTextCodec> for () {
  fn iniCodec(self , rsthis: & QSettings) -> QTextCodec {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QSettings8iniCodecEv()};
    let mut ret = unsafe {_ZNK9QSettings8iniCodecEv(rsthis.qclsinst)};
    let mut ret1 = QTextCodec::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto: static void QSettings::setUserIniPath(const QString & dir);
impl /*struct*/ QSettings {
  pub fn setUserIniPath_s<RetType, T: QSettings_setUserIniPath_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.setUserIniPath_s();
    // return 1;
  }
}

pub trait QSettings_setUserIniPath_s<RetType> {
  fn setUserIniPath_s(self ) -> RetType;
}

  // proto: static void QSettings::setUserIniPath(const QString & dir);
impl<'a> /*trait*/ QSettings_setUserIniPath_s<()> for (&'a QString) {
  fn setUserIniPath_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSettings14setUserIniPathERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QSettings14setUserIniPathERK7QString(arg0)};
    // return 1;
  }
}

  // proto:  QStringList QSettings::childGroups();
impl /*struct*/ QSettings {
  pub fn childGroups<RetType, T: QSettings_childGroups<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.childGroups(self);
    // return 1;
  }
}

pub trait QSettings_childGroups<RetType> {
  fn childGroups(self , rsthis: & QSettings) -> RetType;
}

  // proto:  QStringList QSettings::childGroups();
impl<'a> /*trait*/ QSettings_childGroups<()> for () {
  fn childGroups(self , rsthis: & QSettings) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QSettings11childGroupsEv()};
     unsafe {_ZNK9QSettings11childGroupsEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QVariant QSettings::value(const QString & key, const QVariant & defaultValue);
impl /*struct*/ QSettings {
  pub fn value<RetType, T: QSettings_value<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.value(self);
    // return 1;
  }
}

pub trait QSettings_value<RetType> {
  fn value(self , rsthis: & QSettings) -> RetType;
}

  // proto:  QVariant QSettings::value(const QString & key, const QVariant & defaultValue);
impl<'a> /*trait*/ QSettings_value<QVariant> for (&'a QString, &'a QVariant) {
  fn value(self , rsthis: & QSettings) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QSettings5valueERK7QStringRK8QVariant()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QSettings5valueERK7QStringRK8QVariant(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QVariant::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QSettings::organizationName();
impl /*struct*/ QSettings {
  pub fn organizationName<RetType, T: QSettings_organizationName<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.organizationName(self);
    // return 1;
  }
}

pub trait QSettings_organizationName<RetType> {
  fn organizationName(self , rsthis: & QSettings) -> RetType;
}

  // proto:  QString QSettings::organizationName();
impl<'a> /*trait*/ QSettings_organizationName<QString> for () {
  fn organizationName(self , rsthis: & QSettings) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QSettings16organizationNameEv()};
    let mut ret = unsafe {_ZNK9QSettings16organizationNameEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  const QMetaObject * QSettings::metaObject();
impl /*struct*/ QSettings {
  pub fn metaObject<RetType, T: QSettings_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QSettings_metaObject<RetType> {
  fn metaObject(self , rsthis: & QSettings) -> RetType;
}

  // proto:  const QMetaObject * QSettings::metaObject();
impl<'a> /*trait*/ QSettings_metaObject<()> for () {
  fn metaObject(self , rsthis: & QSettings) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QSettings10metaObjectEv()};
     unsafe {_ZNK9QSettings10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QSettings::setFallbacksEnabled(bool b);
impl /*struct*/ QSettings {
  pub fn setFallbacksEnabled<RetType, T: QSettings_setFallbacksEnabled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setFallbacksEnabled(self);
    // return 1;
  }
}

pub trait QSettings_setFallbacksEnabled<RetType> {
  fn setFallbacksEnabled(self , rsthis: & QSettings) -> RetType;
}

  // proto:  void QSettings::setFallbacksEnabled(bool b);
impl<'a> /*trait*/ QSettings_setFallbacksEnabled<()> for (i8) {
  fn setFallbacksEnabled(self , rsthis: & QSettings) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSettings19setFallbacksEnabledEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN9QSettings19setFallbacksEnabledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QSettings::contains(const QString & key);
impl /*struct*/ QSettings {
  pub fn contains<RetType, T: QSettings_contains<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.contains(self);
    // return 1;
  }
}

pub trait QSettings_contains<RetType> {
  fn contains(self , rsthis: & QSettings) -> RetType;
}

  // proto:  bool QSettings::contains(const QString & key);
impl<'a> /*trait*/ QSettings_contains<i8> for (&'a QString) {
  fn contains(self , rsthis: & QSettings) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QSettings8containsERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QSettings8containsERK7QString(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QSettings::remove(const QString & key);
impl /*struct*/ QSettings {
  pub fn remove<RetType, T: QSettings_remove<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.remove(self);
    // return 1;
  }
}

pub trait QSettings_remove<RetType> {
  fn remove(self , rsthis: & QSettings) -> RetType;
}

  // proto:  void QSettings::remove(const QString & key);
impl<'a> /*trait*/ QSettings_remove<()> for (&'a QString) {
  fn remove(self , rsthis: & QSettings) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSettings6removeERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QSettings6removeERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QSettings::endGroup();
impl /*struct*/ QSettings {
  pub fn endGroup<RetType, T: QSettings_endGroup<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.endGroup(self);
    // return 1;
  }
}

pub trait QSettings_endGroup<RetType> {
  fn endGroup(self , rsthis: & QSettings) -> RetType;
}

  // proto:  void QSettings::endGroup();
impl<'a> /*trait*/ QSettings_endGroup<()> for () {
  fn endGroup(self , rsthis: & QSettings) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSettings8endGroupEv()};
     unsafe {_ZN9QSettings8endGroupEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QSettings::beginWriteArray(const QString & prefix, int size);
impl /*struct*/ QSettings {
  pub fn beginWriteArray<RetType, T: QSettings_beginWriteArray<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.beginWriteArray(self);
    // return 1;
  }
}

pub trait QSettings_beginWriteArray<RetType> {
  fn beginWriteArray(self , rsthis: & QSettings) -> RetType;
}

  // proto:  void QSettings::beginWriteArray(const QString & prefix, int size);
impl<'a> /*trait*/ QSettings_beginWriteArray<()> for (&'a QString, i32) {
  fn beginWriteArray(self , rsthis: & QSettings) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSettings15beginWriteArrayERK7QStringi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN9QSettings15beginWriteArrayERK7QStringi(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QSettings::beginGroup(const QString & prefix);
impl /*struct*/ QSettings {
  pub fn beginGroup<RetType, T: QSettings_beginGroup<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.beginGroup(self);
    // return 1;
  }
}

pub trait QSettings_beginGroup<RetType> {
  fn beginGroup(self , rsthis: & QSettings) -> RetType;
}

  // proto:  void QSettings::beginGroup(const QString & prefix);
impl<'a> /*trait*/ QSettings_beginGroup<()> for (&'a QString) {
  fn beginGroup(self , rsthis: & QSettings) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSettings10beginGroupERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QSettings10beginGroupERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QStringList QSettings::childKeys();
impl /*struct*/ QSettings {
  pub fn childKeys<RetType, T: QSettings_childKeys<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.childKeys(self);
    // return 1;
  }
}

pub trait QSettings_childKeys<RetType> {
  fn childKeys(self , rsthis: & QSettings) -> RetType;
}

  // proto:  QStringList QSettings::childKeys();
impl<'a> /*trait*/ QSettings_childKeys<()> for () {
  fn childKeys(self , rsthis: & QSettings) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QSettings9childKeysEv()};
     unsafe {_ZNK9QSettings9childKeysEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QSettings::QSettings(const QSettings & );
impl<'a> /*trait*/ QSettings_New for (&'a QSettings) {
  fn New(self) -> QSettings {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSettingsC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QSettingsC1ERKS_(qthis, arg0)};
    let rsthis = QSettings{/**/qbase: QObject::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QSettings::endArray();
impl /*struct*/ QSettings {
  pub fn endArray<RetType, T: QSettings_endArray<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.endArray(self);
    // return 1;
  }
}

pub trait QSettings_endArray<RetType> {
  fn endArray(self , rsthis: & QSettings) -> RetType;
}

  // proto:  void QSettings::endArray();
impl<'a> /*trait*/ QSettings_endArray<()> for () {
  fn endArray(self , rsthis: & QSettings) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSettings8endArrayEv()};
     unsafe {_ZN9QSettings8endArrayEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto: static void QSettings::setSystemIniPath(const QString & dir);
impl /*struct*/ QSettings {
  pub fn setSystemIniPath_s<RetType, T: QSettings_setSystemIniPath_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.setSystemIniPath_s();
    // return 1;
  }
}

pub trait QSettings_setSystemIniPath_s<RetType> {
  fn setSystemIniPath_s(self ) -> RetType;
}

  // proto: static void QSettings::setSystemIniPath(const QString & dir);
impl<'a> /*trait*/ QSettings_setSystemIniPath_s<()> for (&'a QString) {
  fn setSystemIniPath_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSettings16setSystemIniPathERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QSettings16setSystemIniPathERK7QString(arg0)};
    // return 1;
  }
}

  // proto:  QStringList QSettings::allKeys();
impl /*struct*/ QSettings {
  pub fn allKeys<RetType, T: QSettings_allKeys<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.allKeys(self);
    // return 1;
  }
}

pub trait QSettings_allKeys<RetType> {
  fn allKeys(self , rsthis: & QSettings) -> RetType;
}

  // proto:  QStringList QSettings::allKeys();
impl<'a> /*trait*/ QSettings_allKeys<()> for () {
  fn allKeys(self , rsthis: & QSettings) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QSettings7allKeysEv()};
     unsafe {_ZNK9QSettings7allKeysEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QString QSettings::group();
impl /*struct*/ QSettings {
  pub fn group<RetType, T: QSettings_group<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.group(self);
    // return 1;
  }
}

pub trait QSettings_group<RetType> {
  fn group(self , rsthis: & QSettings) -> RetType;
}

  // proto:  QString QSettings::group();
impl<'a> /*trait*/ QSettings_group<QString> for () {
  fn group(self , rsthis: & QSettings) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QSettings5groupEv()};
    let mut ret = unsafe {_ZNK9QSettings5groupEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

// <= body block end

