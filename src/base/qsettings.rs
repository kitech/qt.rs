// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qobject::QObject;
use super::qstring::QString;
use super::qvariant::QVariant;
use super::qtextcodec::QTextCodec;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QSettings::NewQSettings(QObject * parent);
  fn _ZN9QSettingsC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QSettings::isWritable();
  fn _ZNK9QSettings10isWritableEv(qthis: *mut c_void) -> int8_t;
  // proto:  QString QSettings::fileName();
  fn _ZNK9QSettings8fileNameEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QSettings::fallbacksEnabled();
  fn _ZNK9QSettings16fallbacksEnabledEv(qthis: *mut c_void) -> int8_t;
  // proto:  QString QSettings::applicationName();
  fn _ZNK9QSettings15applicationNameEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QSettings::sync();
  fn _ZN9QSettings4syncEv(qthis: *mut c_void) ;
  // proto:  void QSettings::setValue(const QString & key, const QVariant & value);
  fn _ZN9QSettings8setValueERK7QStringRK8QVariant(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  void QSettings::setArrayIndex(int i);
  fn _ZN9QSettings13setArrayIndexEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QSettings::NewQSettings(const QString & organization, const QString & application, QObject * parent);
  fn _ZN9QSettingsC1ERK7QStringS2_P7QObject(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) ;
  // proto:  void QSettings::setIniCodec(QTextCodec * codec);
  fn _ZN9QSettings11setIniCodecEP10QTextCodec(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QSettings::setIniCodec(const char * codecName);
  fn _ZN9QSettings11setIniCodecEPKc(qthis: *mut c_void, arg0: *const c_char) ;
  // proto:  int QSettings::beginReadArray(const QString & prefix);
  fn _ZN9QSettings14beginReadArrayERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> c_int;
  // proto:  void QSettings::clear();
  fn _ZN9QSettings5clearEv(qthis: *mut c_void) ;
  // proto:  void QSettings::FreeQSettings();
  fn _ZN9QSettingsD0Ev(qthis: *mut c_void) ;
  // proto:  QTextCodec * QSettings::iniCodec();
  fn _ZNK9QSettings8iniCodecEv(qthis: *mut c_void) -> *mut c_void;
  // proto: static void QSettings::setUserIniPath(const QString & dir);
  fn _ZN9QSettings14setUserIniPathERK7QString(arg0: *mut c_void) ;
  // proto:  QStringList QSettings::childGroups();
  fn _ZNK9QSettings11childGroupsEv(qthis: *mut c_void) ;
  // proto:  QVariant QSettings::value(const QString & key, const QVariant & defaultValue);
  fn _ZNK9QSettings5valueERK7QStringRK8QVariant(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  QString QSettings::organizationName();
  fn _ZNK9QSettings16organizationNameEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  const QMetaObject * QSettings::metaObject();
  fn _ZNK9QSettings10metaObjectEv(qthis: *mut c_void) ;
  // proto:  void QSettings::setFallbacksEnabled(bool b);
  fn _ZN9QSettings19setFallbacksEnabledEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  bool QSettings::contains(const QString & key);
  fn _ZNK9QSettings8containsERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  void QSettings::remove(const QString & key);
  fn _ZN9QSettings6removeERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QSettings::endGroup();
  fn _ZN9QSettings8endGroupEv(qthis: *mut c_void) ;
  // proto:  void QSettings::beginWriteArray(const QString & prefix, int size);
  fn _ZN9QSettings15beginWriteArrayERK7QStringi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) ;
  // proto:  void QSettings::beginGroup(const QString & prefix);
  fn _ZN9QSettings10beginGroupERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QStringList QSettings::childKeys();
  fn _ZNK9QSettings9childKeysEv(qthis: *mut c_void) ;
  // proto:  void QSettings::NewQSettings(const QSettings & );
  fn _ZN9QSettingsC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QSettings::endArray();
  fn _ZN9QSettings8endArrayEv(qthis: *mut c_void) ;
  // proto: static void QSettings::setSystemIniPath(const QString & dir);
  fn _ZN9QSettings16setSystemIniPathERK7QString(arg0: *mut c_void) ;
  // proto:  QStringList QSettings::allKeys();
  fn _ZNK9QSettings7allKeysEv(qthis: *mut c_void) ;
  // proto:  QString QSettings::group();
  fn _ZNK9QSettings5groupEv(qthis: *mut c_void) -> *mut c_void;
}

// body block begin
// class sizeof(QSettings)=1
pub struct QSettings {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QSettings {
  pub fn NewQSettings<T: QSettings_NewQSettings>(value: T) -> QSettings {
    let rsthis = value.NewQSettings();
    return rsthis;
    // return 1;
  }
}

pub trait QSettings_NewQSettings {
  fn NewQSettings(self) -> QSettings;
}

// proto: void QSettings::NewQSettings(QObject * parent);
impl<'a> /*trait*/ QSettings_NewQSettings for (&'a mut QObject) {
  fn NewQSettings(self) -> QSettings {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSettingsC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QSettingsC1EP7QObject(qthis, arg0)};
    let rsthis = QSettings{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QSettings {
  pub fn isWritable<RetType, T: QSettings_isWritable<RetType>>(&mut self, value: T) -> RetType {
    return value.isWritable(self);
    // return 1;
  }
}

pub trait QSettings_isWritable<RetType> {
  fn isWritable(self, rsthis: &mut QSettings) -> RetType;
}

// proto:  bool QSettings::isWritable();
impl<'a> /*trait*/ QSettings_isWritable<i8> for () {
  fn isWritable(self, rsthis: &mut QSettings) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QSettings10isWritableEv()};
    let mut ret = unsafe {_ZNK9QSettings10isWritableEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QSettings {
  pub fn fileName<RetType, T: QSettings_fileName<RetType>>(&mut self, value: T) -> RetType {
    return value.fileName(self);
    // return 1;
  }
}

pub trait QSettings_fileName<RetType> {
  fn fileName(self, rsthis: &mut QSettings) -> RetType;
}

// proto:  QString QSettings::fileName();
impl<'a> /*trait*/ QSettings_fileName<QString> for () {
  fn fileName(self, rsthis: &mut QSettings) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QSettings8fileNameEv()};
    let mut ret = unsafe {_ZNK9QSettings8fileNameEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QSettings {
  pub fn fallbacksEnabled<RetType, T: QSettings_fallbacksEnabled<RetType>>(&mut self, value: T) -> RetType {
    return value.fallbacksEnabled(self);
    // return 1;
  }
}

pub trait QSettings_fallbacksEnabled<RetType> {
  fn fallbacksEnabled(self, rsthis: &mut QSettings) -> RetType;
}

// proto:  bool QSettings::fallbacksEnabled();
impl<'a> /*trait*/ QSettings_fallbacksEnabled<i8> for () {
  fn fallbacksEnabled(self, rsthis: &mut QSettings) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QSettings16fallbacksEnabledEv()};
    let mut ret = unsafe {_ZNK9QSettings16fallbacksEnabledEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QSettings {
  pub fn applicationName<RetType, T: QSettings_applicationName<RetType>>(&mut self, value: T) -> RetType {
    return value.applicationName(self);
    // return 1;
  }
}

pub trait QSettings_applicationName<RetType> {
  fn applicationName(self, rsthis: &mut QSettings) -> RetType;
}

// proto:  QString QSettings::applicationName();
impl<'a> /*trait*/ QSettings_applicationName<QString> for () {
  fn applicationName(self, rsthis: &mut QSettings) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QSettings15applicationNameEv()};
    let mut ret = unsafe {_ZNK9QSettings15applicationNameEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QSettings {
  pub fn sync<RetType, T: QSettings_sync<RetType>>(&mut self, value: T) -> RetType {
    return value.sync(self);
    // return 1;
  }
}

pub trait QSettings_sync<RetType> {
  fn sync(self, rsthis: &mut QSettings) -> RetType;
}

// proto:  void QSettings::sync();
impl<'a> /*trait*/ QSettings_sync<()> for () {
  fn sync(self, rsthis: &mut QSettings) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSettings4syncEv()};
     unsafe {_ZN9QSettings4syncEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QSettings {
  pub fn setValue<RetType, T: QSettings_setValue<RetType>>(&mut self, value: T) -> RetType {
    return value.setValue(self);
    // return 1;
  }
}

pub trait QSettings_setValue<RetType> {
  fn setValue(self, rsthis: &mut QSettings) -> RetType;
}

// proto:  void QSettings::setValue(const QString & key, const QVariant & value);
impl<'a> /*trait*/ QSettings_setValue<()> for (&'a  QString, &'a  QVariant) {
  fn setValue(self, rsthis: &mut QSettings) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSettings8setValueERK7QStringRK8QVariant()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN9QSettings8setValueERK7QStringRK8QVariant(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QSettings {
  pub fn setArrayIndex<RetType, T: QSettings_setArrayIndex<RetType>>(&mut self, value: T) -> RetType {
    return value.setArrayIndex(self);
    // return 1;
  }
}

pub trait QSettings_setArrayIndex<RetType> {
  fn setArrayIndex(self, rsthis: &mut QSettings) -> RetType;
}

// proto:  void QSettings::setArrayIndex(int i);
impl<'a> /*trait*/ QSettings_setArrayIndex<()> for (i32) {
  fn setArrayIndex(self, rsthis: &mut QSettings) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSettings13setArrayIndexEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QSettings13setArrayIndexEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto: void QSettings::NewQSettings(const QString & organization, const QString & application, QObject * parent);
impl<'a> /*trait*/ QSettings_NewQSettings for (&'a  QString, &'a  QString, &'a mut QObject) {
  fn NewQSettings(self) -> QSettings {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSettingsC1ERK7QStringS2_P7QObject()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    unsafe {_ZN9QSettingsC1ERK7QStringS2_P7QObject(qthis, arg0, arg1, arg2)};
    let rsthis = QSettings{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QSettings {
  pub fn setIniCodec<RetType, T: QSettings_setIniCodec<RetType>>(&mut self, value: T) -> RetType {
    return value.setIniCodec(self);
    // return 1;
  }
}

pub trait QSettings_setIniCodec<RetType> {
  fn setIniCodec(self, rsthis: &mut QSettings) -> RetType;
}

// proto:  void QSettings::setIniCodec(QTextCodec * codec);
impl<'a> /*trait*/ QSettings_setIniCodec<()> for (&'a mut QTextCodec) {
  fn setIniCodec(self, rsthis: &mut QSettings) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSettings11setIniCodecEP10QTextCodec()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QSettings11setIniCodecEP10QTextCodec(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QSettings::setIniCodec(const char * codecName);
impl<'a> /*trait*/ QSettings_setIniCodec<()> for (&'a  String) {
  fn setIniCodec(self, rsthis: &mut QSettings) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSettings11setIniCodecEPKc()};
    let arg0 = self.as_ptr()  as *const c_char;
     unsafe {_ZN9QSettings11setIniCodecEPKc(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QSettings {
  pub fn beginReadArray<RetType, T: QSettings_beginReadArray<RetType>>(&mut self, value: T) -> RetType {
    return value.beginReadArray(self);
    // return 1;
  }
}

pub trait QSettings_beginReadArray<RetType> {
  fn beginReadArray(self, rsthis: &mut QSettings) -> RetType;
}

// proto:  int QSettings::beginReadArray(const QString & prefix);
impl<'a> /*trait*/ QSettings_beginReadArray<i32> for (&'a  QString) {
  fn beginReadArray(self, rsthis: &mut QSettings) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSettings14beginReadArrayERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN9QSettings14beginReadArrayERK7QString(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QSettings {
  pub fn clear<RetType, T: QSettings_clear<RetType>>(&mut self, value: T) -> RetType {
    return value.clear(self);
    // return 1;
  }
}

pub trait QSettings_clear<RetType> {
  fn clear(self, rsthis: &mut QSettings) -> RetType;
}

// proto:  void QSettings::clear();
impl<'a> /*trait*/ QSettings_clear<()> for () {
  fn clear(self, rsthis: &mut QSettings) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSettings5clearEv()};
     unsafe {_ZN9QSettings5clearEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QSettings {
  pub fn FreeQSettings<RetType, T: QSettings_FreeQSettings<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQSettings(self);
    // return 1;
  }
}

pub trait QSettings_FreeQSettings<RetType> {
  fn FreeQSettings(self, rsthis: &mut QSettings) -> RetType;
}

// proto:  void QSettings::FreeQSettings();
impl<'a> /*trait*/ QSettings_FreeQSettings<()> for () {
  fn FreeQSettings(self, rsthis: &mut QSettings) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSettingsD0Ev()};
     unsafe {_ZN9QSettingsD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QSettings {
  pub fn iniCodec<RetType, T: QSettings_iniCodec<RetType>>(&mut self, value: T) -> RetType {
    return value.iniCodec(self);
    // return 1;
  }
}

pub trait QSettings_iniCodec<RetType> {
  fn iniCodec(self, rsthis: &mut QSettings) -> RetType;
}

// proto:  QTextCodec * QSettings::iniCodec();
impl<'a> /*trait*/ QSettings_iniCodec<QTextCodec> for () {
  fn iniCodec(self, rsthis: &mut QSettings) -> QTextCodec {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QSettings8iniCodecEv()};
    let mut ret = unsafe {_ZNK9QSettings8iniCodecEv(rsthis.qclsinst)};
    let mut ret1 = QTextCodec{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QSettings {
  pub fn setUserIniPath<RetType, T: QSettings_setUserIniPath<RetType>>(&mut self, value: T) -> RetType {
    return value.setUserIniPath(self);
    // return 1;
  }
}

pub trait QSettings_setUserIniPath<RetType> {
  fn setUserIniPath(self, rsthis: &mut QSettings) -> RetType;
}

// proto: static void QSettings::setUserIniPath(const QString & dir);
impl<'a> /*trait*/ QSettings_setUserIniPath<()> for (&'a  QString) {
  fn setUserIniPath(self, rsthis: &mut QSettings) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSettings14setUserIniPathERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QSettings14setUserIniPathERK7QString(arg0)};
    // return 1;
  }
}

impl /*struct*/ QSettings {
  pub fn childGroups<RetType, T: QSettings_childGroups<RetType>>(&mut self, value: T) -> RetType {
    return value.childGroups(self);
    // return 1;
  }
}

pub trait QSettings_childGroups<RetType> {
  fn childGroups(self, rsthis: &mut QSettings) -> RetType;
}

// proto:  QStringList QSettings::childGroups();
impl<'a> /*trait*/ QSettings_childGroups<()> for () {
  fn childGroups(self, rsthis: &mut QSettings) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QSettings11childGroupsEv()};
     unsafe {_ZNK9QSettings11childGroupsEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QSettings {
  pub fn value<RetType, T: QSettings_value<RetType>>(&mut self, value: T) -> RetType {
    return value.value(self);
    // return 1;
  }
}

pub trait QSettings_value<RetType> {
  fn value(self, rsthis: &mut QSettings) -> RetType;
}

// proto:  QVariant QSettings::value(const QString & key, const QVariant & defaultValue);
impl<'a> /*trait*/ QSettings_value<QVariant> for (&'a  QString, &'a  QVariant) {
  fn value(self, rsthis: &mut QSettings) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QSettings5valueERK7QStringRK8QVariant()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QSettings5valueERK7QStringRK8QVariant(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QVariant{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QSettings {
  pub fn organizationName<RetType, T: QSettings_organizationName<RetType>>(&mut self, value: T) -> RetType {
    return value.organizationName(self);
    // return 1;
  }
}

pub trait QSettings_organizationName<RetType> {
  fn organizationName(self, rsthis: &mut QSettings) -> RetType;
}

// proto:  QString QSettings::organizationName();
impl<'a> /*trait*/ QSettings_organizationName<QString> for () {
  fn organizationName(self, rsthis: &mut QSettings) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QSettings16organizationNameEv()};
    let mut ret = unsafe {_ZNK9QSettings16organizationNameEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QSettings {
  pub fn metaObject<RetType, T: QSettings_metaObject<RetType>>(&mut self, value: T) -> RetType {
    return value.metaObject(self);
    // return 1;
  }
}

pub trait QSettings_metaObject<RetType> {
  fn metaObject(self, rsthis: &mut QSettings) -> RetType;
}

// proto:  const QMetaObject * QSettings::metaObject();
impl<'a> /*trait*/ QSettings_metaObject<()> for () {
  fn metaObject(self, rsthis: &mut QSettings) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QSettings10metaObjectEv()};
     unsafe {_ZNK9QSettings10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QSettings {
  pub fn setFallbacksEnabled<RetType, T: QSettings_setFallbacksEnabled<RetType>>(&mut self, value: T) -> RetType {
    return value.setFallbacksEnabled(self);
    // return 1;
  }
}

pub trait QSettings_setFallbacksEnabled<RetType> {
  fn setFallbacksEnabled(self, rsthis: &mut QSettings) -> RetType;
}

// proto:  void QSettings::setFallbacksEnabled(bool b);
impl<'a> /*trait*/ QSettings_setFallbacksEnabled<()> for (i8) {
  fn setFallbacksEnabled(self, rsthis: &mut QSettings) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSettings19setFallbacksEnabledEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN9QSettings19setFallbacksEnabledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QSettings {
  pub fn contains<RetType, T: QSettings_contains<RetType>>(&mut self, value: T) -> RetType {
    return value.contains(self);
    // return 1;
  }
}

pub trait QSettings_contains<RetType> {
  fn contains(self, rsthis: &mut QSettings) -> RetType;
}

// proto:  bool QSettings::contains(const QString & key);
impl<'a> /*trait*/ QSettings_contains<i8> for (&'a  QString) {
  fn contains(self, rsthis: &mut QSettings) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QSettings8containsERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QSettings8containsERK7QString(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QSettings {
  pub fn remove<RetType, T: QSettings_remove<RetType>>(&mut self, value: T) -> RetType {
    return value.remove(self);
    // return 1;
  }
}

pub trait QSettings_remove<RetType> {
  fn remove(self, rsthis: &mut QSettings) -> RetType;
}

// proto:  void QSettings::remove(const QString & key);
impl<'a> /*trait*/ QSettings_remove<()> for (&'a  QString) {
  fn remove(self, rsthis: &mut QSettings) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSettings6removeERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QSettings6removeERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QSettings {
  pub fn endGroup<RetType, T: QSettings_endGroup<RetType>>(&mut self, value: T) -> RetType {
    return value.endGroup(self);
    // return 1;
  }
}

pub trait QSettings_endGroup<RetType> {
  fn endGroup(self, rsthis: &mut QSettings) -> RetType;
}

// proto:  void QSettings::endGroup();
impl<'a> /*trait*/ QSettings_endGroup<()> for () {
  fn endGroup(self, rsthis: &mut QSettings) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSettings8endGroupEv()};
     unsafe {_ZN9QSettings8endGroupEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QSettings {
  pub fn beginWriteArray<RetType, T: QSettings_beginWriteArray<RetType>>(&mut self, value: T) -> RetType {
    return value.beginWriteArray(self);
    // return 1;
  }
}

pub trait QSettings_beginWriteArray<RetType> {
  fn beginWriteArray(self, rsthis: &mut QSettings) -> RetType;
}

// proto:  void QSettings::beginWriteArray(const QString & prefix, int size);
impl<'a> /*trait*/ QSettings_beginWriteArray<()> for (&'a  QString, i32) {
  fn beginWriteArray(self, rsthis: &mut QSettings) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSettings15beginWriteArrayERK7QStringi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN9QSettings15beginWriteArrayERK7QStringi(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QSettings {
  pub fn beginGroup<RetType, T: QSettings_beginGroup<RetType>>(&mut self, value: T) -> RetType {
    return value.beginGroup(self);
    // return 1;
  }
}

pub trait QSettings_beginGroup<RetType> {
  fn beginGroup(self, rsthis: &mut QSettings) -> RetType;
}

// proto:  void QSettings::beginGroup(const QString & prefix);
impl<'a> /*trait*/ QSettings_beginGroup<()> for (&'a  QString) {
  fn beginGroup(self, rsthis: &mut QSettings) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSettings10beginGroupERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QSettings10beginGroupERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QSettings {
  pub fn childKeys<RetType, T: QSettings_childKeys<RetType>>(&mut self, value: T) -> RetType {
    return value.childKeys(self);
    // return 1;
  }
}

pub trait QSettings_childKeys<RetType> {
  fn childKeys(self, rsthis: &mut QSettings) -> RetType;
}

// proto:  QStringList QSettings::childKeys();
impl<'a> /*trait*/ QSettings_childKeys<()> for () {
  fn childKeys(self, rsthis: &mut QSettings) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QSettings9childKeysEv()};
     unsafe {_ZNK9QSettings9childKeysEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto: void QSettings::NewQSettings(const QSettings & );
impl<'a> /*trait*/ QSettings_NewQSettings for (&'a  QSettings) {
  fn NewQSettings(self) -> QSettings {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSettingsC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QSettingsC1ERKS_(qthis, arg0)};
    let rsthis = QSettings{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QSettings {
  pub fn endArray<RetType, T: QSettings_endArray<RetType>>(&mut self, value: T) -> RetType {
    return value.endArray(self);
    // return 1;
  }
}

pub trait QSettings_endArray<RetType> {
  fn endArray(self, rsthis: &mut QSettings) -> RetType;
}

// proto:  void QSettings::endArray();
impl<'a> /*trait*/ QSettings_endArray<()> for () {
  fn endArray(self, rsthis: &mut QSettings) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSettings8endArrayEv()};
     unsafe {_ZN9QSettings8endArrayEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QSettings {
  pub fn setSystemIniPath<RetType, T: QSettings_setSystemIniPath<RetType>>(&mut self, value: T) -> RetType {
    return value.setSystemIniPath(self);
    // return 1;
  }
}

pub trait QSettings_setSystemIniPath<RetType> {
  fn setSystemIniPath(self, rsthis: &mut QSettings) -> RetType;
}

// proto: static void QSettings::setSystemIniPath(const QString & dir);
impl<'a> /*trait*/ QSettings_setSystemIniPath<()> for (&'a  QString) {
  fn setSystemIniPath(self, rsthis: &mut QSettings) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSettings16setSystemIniPathERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QSettings16setSystemIniPathERK7QString(arg0)};
    // return 1;
  }
}

impl /*struct*/ QSettings {
  pub fn allKeys<RetType, T: QSettings_allKeys<RetType>>(&mut self, value: T) -> RetType {
    return value.allKeys(self);
    // return 1;
  }
}

pub trait QSettings_allKeys<RetType> {
  fn allKeys(self, rsthis: &mut QSettings) -> RetType;
}

// proto:  QStringList QSettings::allKeys();
impl<'a> /*trait*/ QSettings_allKeys<()> for () {
  fn allKeys(self, rsthis: &mut QSettings) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QSettings7allKeysEv()};
     unsafe {_ZNK9QSettings7allKeysEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QSettings {
  pub fn group<RetType, T: QSettings_group<RetType>>(&mut self, value: T) -> RetType {
    return value.group(self);
    // return 1;
  }
}

pub trait QSettings_group<RetType> {
  fn group(self, rsthis: &mut QSettings) -> RetType;
}

// proto:  QString QSettings::group();
impl<'a> /*trait*/ QSettings_group<QString> for () {
  fn group(self, rsthis: &mut QSettings) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QSettings5groupEv()};
    let mut ret = unsafe {_ZNK9QSettings5groupEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

