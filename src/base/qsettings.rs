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
  pub fn isWritable<T: QSettings_isWritable>(&mut self, value: T) -> i8 {
    return value.isWritable(self);
    // return 1;
  }
}

pub trait QSettings_isWritable {
  fn isWritable(self, rsthis: &mut QSettings) -> i8;
}

// proto:  bool QSettings::isWritable();
impl<'a> /*trait*/ QSettings_isWritable for () {
  fn isWritable(self, rsthis: &mut QSettings) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QSettings10isWritableEv()};
    let mut ret = unsafe {_ZNK9QSettings10isWritableEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QSettings {
  pub fn fileName<T: QSettings_fileName>(&mut self, value: T) -> QString {
    return value.fileName(self);
    // return 1;
  }
}

pub trait QSettings_fileName {
  fn fileName(self, rsthis: &mut QSettings) -> QString;
}

// proto:  QString QSettings::fileName();
impl<'a> /*trait*/ QSettings_fileName for () {
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
  pub fn fallbacksEnabled<T: QSettings_fallbacksEnabled>(&mut self, value: T) -> i8 {
    return value.fallbacksEnabled(self);
    // return 1;
  }
}

pub trait QSettings_fallbacksEnabled {
  fn fallbacksEnabled(self, rsthis: &mut QSettings) -> i8;
}

// proto:  bool QSettings::fallbacksEnabled();
impl<'a> /*trait*/ QSettings_fallbacksEnabled for () {
  fn fallbacksEnabled(self, rsthis: &mut QSettings) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QSettings16fallbacksEnabledEv()};
    let mut ret = unsafe {_ZNK9QSettings16fallbacksEnabledEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QSettings {
  pub fn applicationName<T: QSettings_applicationName>(&mut self, value: T) -> QString {
    return value.applicationName(self);
    // return 1;
  }
}

pub trait QSettings_applicationName {
  fn applicationName(self, rsthis: &mut QSettings) -> QString;
}

// proto:  QString QSettings::applicationName();
impl<'a> /*trait*/ QSettings_applicationName for () {
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
  pub fn sync<T: QSettings_sync>(&mut self, value: T)  {
     value.sync(self);
    // return 1;
  }
}

pub trait QSettings_sync {
  fn sync(self, rsthis: &mut QSettings) ;
}

// proto:  void QSettings::sync();
impl<'a> /*trait*/ QSettings_sync for () {
  fn sync(self, rsthis: &mut QSettings)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSettings4syncEv()};
     unsafe {_ZN9QSettings4syncEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QSettings {
  pub fn setValue<T: QSettings_setValue>(&mut self, value: T)  {
     value.setValue(self);
    // return 1;
  }
}

pub trait QSettings_setValue {
  fn setValue(self, rsthis: &mut QSettings) ;
}

// proto:  void QSettings::setValue(const QString & key, const QVariant & value);
impl<'a> /*trait*/ QSettings_setValue for (&'a  QString, &'a  QVariant) {
  fn setValue(self, rsthis: &mut QSettings)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSettings8setValueERK7QStringRK8QVariant()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN9QSettings8setValueERK7QStringRK8QVariant(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QSettings {
  pub fn setArrayIndex<T: QSettings_setArrayIndex>(&mut self, value: T)  {
     value.setArrayIndex(self);
    // return 1;
  }
}

pub trait QSettings_setArrayIndex {
  fn setArrayIndex(self, rsthis: &mut QSettings) ;
}

// proto:  void QSettings::setArrayIndex(int i);
impl<'a> /*trait*/ QSettings_setArrayIndex for (i32) {
  fn setArrayIndex(self, rsthis: &mut QSettings)  {
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
  pub fn setIniCodec<T: QSettings_setIniCodec>(&mut self, value: T)  {
     value.setIniCodec(self);
    // return 1;
  }
}

pub trait QSettings_setIniCodec {
  fn setIniCodec(self, rsthis: &mut QSettings) ;
}

// proto:  void QSettings::setIniCodec(QTextCodec * codec);
impl<'a> /*trait*/ QSettings_setIniCodec for (&'a mut QTextCodec) {
  fn setIniCodec(self, rsthis: &mut QSettings)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSettings11setIniCodecEP10QTextCodec()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QSettings11setIniCodecEP10QTextCodec(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QSettings::setIniCodec(const char * codecName);
impl<'a> /*trait*/ QSettings_setIniCodec for (&'a  String) {
  fn setIniCodec(self, rsthis: &mut QSettings)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSettings11setIniCodecEPKc()};
    let arg0 = self.as_ptr()  as *const c_char;
     unsafe {_ZN9QSettings11setIniCodecEPKc(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QSettings {
  pub fn beginReadArray<T: QSettings_beginReadArray>(&mut self, value: T) -> i32 {
    return value.beginReadArray(self);
    // return 1;
  }
}

pub trait QSettings_beginReadArray {
  fn beginReadArray(self, rsthis: &mut QSettings) -> i32;
}

// proto:  int QSettings::beginReadArray(const QString & prefix);
impl<'a> /*trait*/ QSettings_beginReadArray for (&'a  QString) {
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
  pub fn clear<T: QSettings_clear>(&mut self, value: T)  {
     value.clear(self);
    // return 1;
  }
}

pub trait QSettings_clear {
  fn clear(self, rsthis: &mut QSettings) ;
}

// proto:  void QSettings::clear();
impl<'a> /*trait*/ QSettings_clear for () {
  fn clear(self, rsthis: &mut QSettings)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSettings5clearEv()};
     unsafe {_ZN9QSettings5clearEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QSettings {
  pub fn FreeQSettings<T: QSettings_FreeQSettings>(&mut self, value: T)  {
     value.FreeQSettings(self);
    // return 1;
  }
}

pub trait QSettings_FreeQSettings {
  fn FreeQSettings(self, rsthis: &mut QSettings) ;
}

// proto:  void QSettings::FreeQSettings();
impl<'a> /*trait*/ QSettings_FreeQSettings for () {
  fn FreeQSettings(self, rsthis: &mut QSettings)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSettingsD0Ev()};
     unsafe {_ZN9QSettingsD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QSettings {
  pub fn iniCodec<T: QSettings_iniCodec>(&mut self, value: T) -> QTextCodec {
    return value.iniCodec(self);
    // return 1;
  }
}

pub trait QSettings_iniCodec {
  fn iniCodec(self, rsthis: &mut QSettings) -> QTextCodec;
}

// proto:  QTextCodec * QSettings::iniCodec();
impl<'a> /*trait*/ QSettings_iniCodec for () {
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
  pub fn setUserIniPath<T: QSettings_setUserIniPath>(&mut self, value: T)  {
     value.setUserIniPath(self);
    // return 1;
  }
}

pub trait QSettings_setUserIniPath {
  fn setUserIniPath(self, rsthis: &mut QSettings) ;
}

// proto: static void QSettings::setUserIniPath(const QString & dir);
impl<'a> /*trait*/ QSettings_setUserIniPath for (&'a  QString) {
  fn setUserIniPath(self, rsthis: &mut QSettings)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSettings14setUserIniPathERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QSettings14setUserIniPathERK7QString(arg0)};
    // return 1;
  }
}

impl /*struct*/ QSettings {
  pub fn childGroups<T: QSettings_childGroups>(&mut self, value: T)  {
     value.childGroups(self);
    // return 1;
  }
}

pub trait QSettings_childGroups {
  fn childGroups(self, rsthis: &mut QSettings) ;
}

// proto:  QStringList QSettings::childGroups();
impl<'a> /*trait*/ QSettings_childGroups for () {
  fn childGroups(self, rsthis: &mut QSettings)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QSettings11childGroupsEv()};
     unsafe {_ZNK9QSettings11childGroupsEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QSettings {
  pub fn value<T: QSettings_value>(&mut self, value: T) -> QVariant {
    return value.value(self);
    // return 1;
  }
}

pub trait QSettings_value {
  fn value(self, rsthis: &mut QSettings) -> QVariant;
}

// proto:  QVariant QSettings::value(const QString & key, const QVariant & defaultValue);
impl<'a> /*trait*/ QSettings_value for (&'a  QString, &'a  QVariant) {
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
  pub fn organizationName<T: QSettings_organizationName>(&mut self, value: T) -> QString {
    return value.organizationName(self);
    // return 1;
  }
}

pub trait QSettings_organizationName {
  fn organizationName(self, rsthis: &mut QSettings) -> QString;
}

// proto:  QString QSettings::organizationName();
impl<'a> /*trait*/ QSettings_organizationName for () {
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
  pub fn metaObject<T: QSettings_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QSettings_metaObject {
  fn metaObject(self, rsthis: &mut QSettings) ;
}

// proto:  const QMetaObject * QSettings::metaObject();
impl<'a> /*trait*/ QSettings_metaObject for () {
  fn metaObject(self, rsthis: &mut QSettings)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QSettings10metaObjectEv()};
     unsafe {_ZNK9QSettings10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QSettings {
  pub fn setFallbacksEnabled<T: QSettings_setFallbacksEnabled>(&mut self, value: T)  {
     value.setFallbacksEnabled(self);
    // return 1;
  }
}

pub trait QSettings_setFallbacksEnabled {
  fn setFallbacksEnabled(self, rsthis: &mut QSettings) ;
}

// proto:  void QSettings::setFallbacksEnabled(bool b);
impl<'a> /*trait*/ QSettings_setFallbacksEnabled for (i8) {
  fn setFallbacksEnabled(self, rsthis: &mut QSettings)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSettings19setFallbacksEnabledEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN9QSettings19setFallbacksEnabledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QSettings {
  pub fn contains<T: QSettings_contains>(&mut self, value: T) -> i8 {
    return value.contains(self);
    // return 1;
  }
}

pub trait QSettings_contains {
  fn contains(self, rsthis: &mut QSettings) -> i8;
}

// proto:  bool QSettings::contains(const QString & key);
impl<'a> /*trait*/ QSettings_contains for (&'a  QString) {
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
  pub fn remove<T: QSettings_remove>(&mut self, value: T)  {
     value.remove(self);
    // return 1;
  }
}

pub trait QSettings_remove {
  fn remove(self, rsthis: &mut QSettings) ;
}

// proto:  void QSettings::remove(const QString & key);
impl<'a> /*trait*/ QSettings_remove for (&'a  QString) {
  fn remove(self, rsthis: &mut QSettings)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSettings6removeERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QSettings6removeERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QSettings {
  pub fn endGroup<T: QSettings_endGroup>(&mut self, value: T)  {
     value.endGroup(self);
    // return 1;
  }
}

pub trait QSettings_endGroup {
  fn endGroup(self, rsthis: &mut QSettings) ;
}

// proto:  void QSettings::endGroup();
impl<'a> /*trait*/ QSettings_endGroup for () {
  fn endGroup(self, rsthis: &mut QSettings)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSettings8endGroupEv()};
     unsafe {_ZN9QSettings8endGroupEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QSettings {
  pub fn beginWriteArray<T: QSettings_beginWriteArray>(&mut self, value: T)  {
     value.beginWriteArray(self);
    // return 1;
  }
}

pub trait QSettings_beginWriteArray {
  fn beginWriteArray(self, rsthis: &mut QSettings) ;
}

// proto:  void QSettings::beginWriteArray(const QString & prefix, int size);
impl<'a> /*trait*/ QSettings_beginWriteArray for (&'a  QString, i32) {
  fn beginWriteArray(self, rsthis: &mut QSettings)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSettings15beginWriteArrayERK7QStringi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN9QSettings15beginWriteArrayERK7QStringi(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QSettings {
  pub fn beginGroup<T: QSettings_beginGroup>(&mut self, value: T)  {
     value.beginGroup(self);
    // return 1;
  }
}

pub trait QSettings_beginGroup {
  fn beginGroup(self, rsthis: &mut QSettings) ;
}

// proto:  void QSettings::beginGroup(const QString & prefix);
impl<'a> /*trait*/ QSettings_beginGroup for (&'a  QString) {
  fn beginGroup(self, rsthis: &mut QSettings)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSettings10beginGroupERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QSettings10beginGroupERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QSettings {
  pub fn childKeys<T: QSettings_childKeys>(&mut self, value: T)  {
     value.childKeys(self);
    // return 1;
  }
}

pub trait QSettings_childKeys {
  fn childKeys(self, rsthis: &mut QSettings) ;
}

// proto:  QStringList QSettings::childKeys();
impl<'a> /*trait*/ QSettings_childKeys for () {
  fn childKeys(self, rsthis: &mut QSettings)  {
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
  pub fn endArray<T: QSettings_endArray>(&mut self, value: T)  {
     value.endArray(self);
    // return 1;
  }
}

pub trait QSettings_endArray {
  fn endArray(self, rsthis: &mut QSettings) ;
}

// proto:  void QSettings::endArray();
impl<'a> /*trait*/ QSettings_endArray for () {
  fn endArray(self, rsthis: &mut QSettings)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSettings8endArrayEv()};
     unsafe {_ZN9QSettings8endArrayEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QSettings {
  pub fn setSystemIniPath<T: QSettings_setSystemIniPath>(&mut self, value: T)  {
     value.setSystemIniPath(self);
    // return 1;
  }
}

pub trait QSettings_setSystemIniPath {
  fn setSystemIniPath(self, rsthis: &mut QSettings) ;
}

// proto: static void QSettings::setSystemIniPath(const QString & dir);
impl<'a> /*trait*/ QSettings_setSystemIniPath for (&'a  QString) {
  fn setSystemIniPath(self, rsthis: &mut QSettings)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSettings16setSystemIniPathERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QSettings16setSystemIniPathERK7QString(arg0)};
    // return 1;
  }
}

impl /*struct*/ QSettings {
  pub fn allKeys<T: QSettings_allKeys>(&mut self, value: T)  {
     value.allKeys(self);
    // return 1;
  }
}

pub trait QSettings_allKeys {
  fn allKeys(self, rsthis: &mut QSettings) ;
}

// proto:  QStringList QSettings::allKeys();
impl<'a> /*trait*/ QSettings_allKeys for () {
  fn allKeys(self, rsthis: &mut QSettings)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QSettings7allKeysEv()};
     unsafe {_ZNK9QSettings7allKeysEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QSettings {
  pub fn group<T: QSettings_group>(&mut self, value: T) -> QString {
    return value.group(self);
    // return 1;
  }
}

pub trait QSettings_group {
  fn group(self, rsthis: &mut QSettings) -> QString;
}

// proto:  QString QSettings::group();
impl<'a> /*trait*/ QSettings_group for () {
  fn group(self, rsthis: &mut QSettings) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QSettings5groupEv()};
    let mut ret = unsafe {_ZNK9QSettings5groupEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

