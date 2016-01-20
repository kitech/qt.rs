// auto generated, do not modify.
// created: Wed Jan 20 00:44:03 2016
// src-file: /QtQml/qqmlcomponent.h
// dst-file: /src/qml/qqmlcomponent.rs
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
use super::super::core::qobject::QObject; // 771
use std::ops::Deref;
use super::qqmlcontext::QQmlContext; // 773
use super::qqmlengine::QQmlEngine; // 773
use super::super::core::qurl::QUrl; // 771
use super::super::core::qstring::QString; // 771
use super::super::core::qbytearray::QByteArray; // 771
use super::qqmlincubator::QQmlIncubator; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QQmlComponent_Class_Size() -> c_int;
  // proto:  void QQmlComponent::~QQmlComponent();
  fn _ZN13QQmlComponentD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QQmlContext * QQmlComponent::creationContext();
  fn _ZNK13QQmlComponent15creationContextEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QObject * QQmlComponent::create(QQmlContext * context);
  fn _ZN13QQmlComponent6createEP11QQmlContext(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QQmlComponent::QQmlComponent(QQmlEngine * , QObject * parent);
  fn _ZN13QQmlComponentC2EP10QQmlEngineP7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  bool QQmlComponent::isReady();
  fn _ZNK13QQmlComponent7isReadyEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QQmlComponent::QQmlComponent(QObject * parent);
  fn _ZN13QQmlComponentC2EP7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  qreal QQmlComponent::progress();
  fn _ZNK13QQmlComponent8progressEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  QUrl QQmlComponent::url();
  fn _ZNK13QQmlComponent3urlEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto: static QQmlComponentAttached * QQmlComponent::qmlAttachedProperties(QObject * );
  fn _ZN13QQmlComponent21qmlAttachedPropertiesEP7QObject(arg0: *mut c_void);
  // proto:  bool QQmlComponent::isError();
  fn _ZNK13QQmlComponent7isErrorEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QQmlComponent::completeCreate();
  fn _ZN13QQmlComponent14completeCreateEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QQmlComponent::isLoading();
  fn _ZNK13QQmlComponent9isLoadingEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QQmlComponent::loadUrl(const QUrl & url);
  fn _ZN13QQmlComponent7loadUrlERK4QUrl(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QQmlComponent::QQmlComponent(QQmlEngine * , const QUrl & url, QObject * parent);
  fn _ZN13QQmlComponentC2EP10QQmlEngineRK4QUrlP7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void);
  // proto:  QObject * QQmlComponent::beginCreate(QQmlContext * );
  fn _ZN13QQmlComponent11beginCreateEP11QQmlContext(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QQmlComponent::setData(const QByteArray & , const QUrl & baseUrl);
  fn _ZN13QQmlComponent7setDataERK10QByteArrayRK4QUrl(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  QString QQmlComponent::errorString();
  fn _ZNK13QQmlComponent11errorStringEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QList<QQmlError> QQmlComponent::errors();
  fn _ZNK13QQmlComponent6errorsEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QQmlComponent::QQmlComponent(const QQmlComponent & );
  fn _ZN13QQmlComponentC2ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  const QMetaObject * QQmlComponent::metaObject();
  fn _ZNK13QQmlComponent10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QQmlComponent::create(QQmlIncubator & , QQmlContext * context, QQmlContext * forContext);
  fn _ZN13QQmlComponent6createER13QQmlIncubatorP11QQmlContextS3_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void);
  // proto:  void QQmlComponent::QQmlComponent(QQmlEngine * , const QString & fileName, QObject * parent);
  fn _ZN13QQmlComponentC2EP10QQmlEngineRK7QStringP7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void);
  // proto:  bool QQmlComponent::isNull();
  fn _ZNK13QQmlComponent6isNullEv(qthis: u64 /* *mut c_void*/) -> c_char;
  fn QQmlComponent_SlotProxy_connect__ZN13QQmlComponent15progressChangedEd(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QQmlComponent)=1
#[derive(Default)]
pub struct QQmlComponent {
  qbase: QObject,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _progressChanged: QQmlComponent_progressChanged_signal,
  pub _statusChanged: QQmlComponent_statusChanged_signal,
}

impl /*struct*/ QQmlComponent {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QQmlComponent {
    return QQmlComponent{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QQmlComponent {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QQmlComponent {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  void QQmlComponent::~QQmlComponent();
impl /*struct*/ QQmlComponent {
  pub fn free<RetType, T: QQmlComponent_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QQmlComponent_free<RetType> {
  fn free(self , rsthis: & QQmlComponent) -> RetType;
}

  // proto:  void QQmlComponent::~QQmlComponent();
impl<'a> /*trait*/ QQmlComponent_free<()> for () {
  fn free(self , rsthis: & QQmlComponent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QQmlComponentD2Ev()};
     unsafe {_ZN13QQmlComponentD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QQmlContext * QQmlComponent::creationContext();
impl /*struct*/ QQmlComponent {
  pub fn creationContext<RetType, T: QQmlComponent_creationContext<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.creationContext(self);
    // return 1;
  }
}

pub trait QQmlComponent_creationContext<RetType> {
  fn creationContext(self , rsthis: & QQmlComponent) -> RetType;
}

  // proto:  QQmlContext * QQmlComponent::creationContext();
impl<'a> /*trait*/ QQmlComponent_creationContext<QQmlContext> for () {
  fn creationContext(self , rsthis: & QQmlComponent) -> QQmlContext {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QQmlComponent15creationContextEv()};
    let mut ret = unsafe {_ZNK13QQmlComponent15creationContextEv(rsthis.qclsinst)};
    let mut ret1 = QQmlContext::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QObject * QQmlComponent::create(QQmlContext * context);
impl /*struct*/ QQmlComponent {
  pub fn create<RetType, T: QQmlComponent_create<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.create(self);
    // return 1;
  }
}

pub trait QQmlComponent_create<RetType> {
  fn create(self , rsthis: & QQmlComponent) -> RetType;
}

  // proto:  QObject * QQmlComponent::create(QQmlContext * context);
impl<'a> /*trait*/ QQmlComponent_create<QObject> for (&'a QQmlContext) {
  fn create(self , rsthis: & QQmlComponent) -> QObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QQmlComponent6createEP11QQmlContext()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN13QQmlComponent6createEP11QQmlContext(rsthis.qclsinst, arg0)};
    let mut ret1 = QObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QQmlComponent::QQmlComponent(QQmlEngine * , QObject * parent);
impl /*struct*/ QQmlComponent {
  pub fn new<T: QQmlComponent_new>(value: T) -> QQmlComponent {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QQmlComponent_new {
  fn new(self) -> QQmlComponent;
}

  // proto:  void QQmlComponent::QQmlComponent(QQmlEngine * , QObject * parent);
impl<'a> /*trait*/ QQmlComponent_new for (&'a QQmlEngine, &'a QObject) {
  fn new(self) -> QQmlComponent {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QQmlComponentC2EP10QQmlEngineP7QObject()};
    let ctysz: c_int = unsafe{QQmlComponent_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN13QQmlComponentC2EP10QQmlEngineP7QObject(qthis_ph, arg0, arg1)};
    let qthis: u64 = qthis_ph;
    let rsthis = QQmlComponent{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QQmlComponent::isReady();
impl /*struct*/ QQmlComponent {
  pub fn isReady<RetType, T: QQmlComponent_isReady<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isReady(self);
    // return 1;
  }
}

pub trait QQmlComponent_isReady<RetType> {
  fn isReady(self , rsthis: & QQmlComponent) -> RetType;
}

  // proto:  bool QQmlComponent::isReady();
impl<'a> /*trait*/ QQmlComponent_isReady<i8> for () {
  fn isReady(self , rsthis: & QQmlComponent) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QQmlComponent7isReadyEv()};
    let mut ret = unsafe {_ZNK13QQmlComponent7isReadyEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QQmlComponent::QQmlComponent(QObject * parent);
impl<'a> /*trait*/ QQmlComponent_new for (&'a QObject) {
  fn new(self) -> QQmlComponent {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QQmlComponentC2EP7QObject()};
    let ctysz: c_int = unsafe{QQmlComponent_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QQmlComponentC2EP7QObject(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QQmlComponent{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  qreal QQmlComponent::progress();
impl /*struct*/ QQmlComponent {
  pub fn progress<RetType, T: QQmlComponent_progress<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.progress(self);
    // return 1;
  }
}

pub trait QQmlComponent_progress<RetType> {
  fn progress(self , rsthis: & QQmlComponent) -> RetType;
}

  // proto:  qreal QQmlComponent::progress();
impl<'a> /*trait*/ QQmlComponent_progress<f64> for () {
  fn progress(self , rsthis: & QQmlComponent) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QQmlComponent8progressEv()};
    let mut ret = unsafe {_ZNK13QQmlComponent8progressEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  QUrl QQmlComponent::url();
impl /*struct*/ QQmlComponent {
  pub fn url<RetType, T: QQmlComponent_url<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.url(self);
    // return 1;
  }
}

pub trait QQmlComponent_url<RetType> {
  fn url(self , rsthis: & QQmlComponent) -> RetType;
}

  // proto:  QUrl QQmlComponent::url();
impl<'a> /*trait*/ QQmlComponent_url<QUrl> for () {
  fn url(self , rsthis: & QQmlComponent) -> QUrl {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QQmlComponent3urlEv()};
    let mut ret = unsafe {_ZNK13QQmlComponent3urlEv(rsthis.qclsinst)};
    let mut ret1 = QUrl::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto: static QQmlComponentAttached * QQmlComponent::qmlAttachedProperties(QObject * );
impl /*struct*/ QQmlComponent {
  pub fn qmlAttachedProperties_s<RetType, T: QQmlComponent_qmlAttachedProperties_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.qmlAttachedProperties_s();
    // return 1;
  }
}

pub trait QQmlComponent_qmlAttachedProperties_s<RetType> {
  fn qmlAttachedProperties_s(self ) -> RetType;
}

  // proto: static QQmlComponentAttached * QQmlComponent::qmlAttachedProperties(QObject * );
impl<'a> /*trait*/ QQmlComponent_qmlAttachedProperties_s<()> for (&'a QObject) {
  fn qmlAttachedProperties_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QQmlComponent21qmlAttachedPropertiesEP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QQmlComponent21qmlAttachedPropertiesEP7QObject(arg0)};
    // return 1;
  }
}

  // proto:  bool QQmlComponent::isError();
impl /*struct*/ QQmlComponent {
  pub fn isError<RetType, T: QQmlComponent_isError<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isError(self);
    // return 1;
  }
}

pub trait QQmlComponent_isError<RetType> {
  fn isError(self , rsthis: & QQmlComponent) -> RetType;
}

  // proto:  bool QQmlComponent::isError();
impl<'a> /*trait*/ QQmlComponent_isError<i8> for () {
  fn isError(self , rsthis: & QQmlComponent) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QQmlComponent7isErrorEv()};
    let mut ret = unsafe {_ZNK13QQmlComponent7isErrorEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QQmlComponent::completeCreate();
impl /*struct*/ QQmlComponent {
  pub fn completeCreate<RetType, T: QQmlComponent_completeCreate<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.completeCreate(self);
    // return 1;
  }
}

pub trait QQmlComponent_completeCreate<RetType> {
  fn completeCreate(self , rsthis: & QQmlComponent) -> RetType;
}

  // proto:  void QQmlComponent::completeCreate();
impl<'a> /*trait*/ QQmlComponent_completeCreate<()> for () {
  fn completeCreate(self , rsthis: & QQmlComponent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QQmlComponent14completeCreateEv()};
     unsafe {_ZN13QQmlComponent14completeCreateEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QQmlComponent::isLoading();
impl /*struct*/ QQmlComponent {
  pub fn isLoading<RetType, T: QQmlComponent_isLoading<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isLoading(self);
    // return 1;
  }
}

pub trait QQmlComponent_isLoading<RetType> {
  fn isLoading(self , rsthis: & QQmlComponent) -> RetType;
}

  // proto:  bool QQmlComponent::isLoading();
impl<'a> /*trait*/ QQmlComponent_isLoading<i8> for () {
  fn isLoading(self , rsthis: & QQmlComponent) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QQmlComponent9isLoadingEv()};
    let mut ret = unsafe {_ZNK13QQmlComponent9isLoadingEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QQmlComponent::loadUrl(const QUrl & url);
impl /*struct*/ QQmlComponent {
  pub fn loadUrl<RetType, T: QQmlComponent_loadUrl<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.loadUrl(self);
    // return 1;
  }
}

pub trait QQmlComponent_loadUrl<RetType> {
  fn loadUrl(self , rsthis: & QQmlComponent) -> RetType;
}

  // proto:  void QQmlComponent::loadUrl(const QUrl & url);
impl<'a> /*trait*/ QQmlComponent_loadUrl<()> for (&'a QUrl) {
  fn loadUrl(self , rsthis: & QQmlComponent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QQmlComponent7loadUrlERK4QUrl()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QQmlComponent7loadUrlERK4QUrl(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QQmlComponent::QQmlComponent(QQmlEngine * , const QUrl & url, QObject * parent);
impl<'a> /*trait*/ QQmlComponent_new for (&'a QQmlEngine, &'a QUrl, &'a QObject) {
  fn new(self) -> QQmlComponent {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QQmlComponentC2EP10QQmlEngineRK4QUrlP7QObject()};
    let ctysz: c_int = unsafe{QQmlComponent_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    unsafe {_ZN13QQmlComponentC2EP10QQmlEngineRK4QUrlP7QObject(qthis_ph, arg0, arg1, arg2)};
    let qthis: u64 = qthis_ph;
    let rsthis = QQmlComponent{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QObject * QQmlComponent::beginCreate(QQmlContext * );
impl /*struct*/ QQmlComponent {
  pub fn beginCreate<RetType, T: QQmlComponent_beginCreate<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.beginCreate(self);
    // return 1;
  }
}

pub trait QQmlComponent_beginCreate<RetType> {
  fn beginCreate(self , rsthis: & QQmlComponent) -> RetType;
}

  // proto:  QObject * QQmlComponent::beginCreate(QQmlContext * );
impl<'a> /*trait*/ QQmlComponent_beginCreate<QObject> for (&'a QQmlContext) {
  fn beginCreate(self , rsthis: & QQmlComponent) -> QObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QQmlComponent11beginCreateEP11QQmlContext()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN13QQmlComponent11beginCreateEP11QQmlContext(rsthis.qclsinst, arg0)};
    let mut ret1 = QObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QQmlComponent::setData(const QByteArray & , const QUrl & baseUrl);
impl /*struct*/ QQmlComponent {
  pub fn setData<RetType, T: QQmlComponent_setData<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setData(self);
    // return 1;
  }
}

pub trait QQmlComponent_setData<RetType> {
  fn setData(self , rsthis: & QQmlComponent) -> RetType;
}

  // proto:  void QQmlComponent::setData(const QByteArray & , const QUrl & baseUrl);
impl<'a> /*trait*/ QQmlComponent_setData<()> for (&'a QByteArray, &'a QUrl) {
  fn setData(self , rsthis: & QQmlComponent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QQmlComponent7setDataERK10QByteArrayRK4QUrl()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN13QQmlComponent7setDataERK10QByteArrayRK4QUrl(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QString QQmlComponent::errorString();
impl /*struct*/ QQmlComponent {
  pub fn errorString<RetType, T: QQmlComponent_errorString<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.errorString(self);
    // return 1;
  }
}

pub trait QQmlComponent_errorString<RetType> {
  fn errorString(self , rsthis: & QQmlComponent) -> RetType;
}

  // proto:  QString QQmlComponent::errorString();
impl<'a> /*trait*/ QQmlComponent_errorString<QString> for () {
  fn errorString(self , rsthis: & QQmlComponent) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QQmlComponent11errorStringEv()};
    let mut ret = unsafe {_ZNK13QQmlComponent11errorStringEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QList<QQmlError> QQmlComponent::errors();
impl /*struct*/ QQmlComponent {
  pub fn errors<RetType, T: QQmlComponent_errors<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.errors(self);
    // return 1;
  }
}

pub trait QQmlComponent_errors<RetType> {
  fn errors(self , rsthis: & QQmlComponent) -> RetType;
}

  // proto:  QList<QQmlError> QQmlComponent::errors();
impl<'a> /*trait*/ QQmlComponent_errors<()> for () {
  fn errors(self , rsthis: & QQmlComponent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QQmlComponent6errorsEv()};
     unsafe {_ZNK13QQmlComponent6errorsEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QQmlComponent::QQmlComponent(const QQmlComponent & );
impl<'a> /*trait*/ QQmlComponent_new for (&'a QQmlComponent) {
  fn new(self) -> QQmlComponent {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QQmlComponentC2ERKS_()};
    let ctysz: c_int = unsafe{QQmlComponent_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QQmlComponentC2ERKS_(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QQmlComponent{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  const QMetaObject * QQmlComponent::metaObject();
impl /*struct*/ QQmlComponent {
  pub fn metaObject<RetType, T: QQmlComponent_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QQmlComponent_metaObject<RetType> {
  fn metaObject(self , rsthis: & QQmlComponent) -> RetType;
}

  // proto:  const QMetaObject * QQmlComponent::metaObject();
impl<'a> /*trait*/ QQmlComponent_metaObject<()> for () {
  fn metaObject(self , rsthis: & QQmlComponent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QQmlComponent10metaObjectEv()};
     unsafe {_ZNK13QQmlComponent10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QQmlComponent::create(QQmlIncubator & , QQmlContext * context, QQmlContext * forContext);
impl<'a> /*trait*/ QQmlComponent_create<()> for (&'a QQmlIncubator, &'a QQmlContext, &'a QQmlContext) {
  fn create(self , rsthis: & QQmlComponent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QQmlComponent6createER13QQmlIncubatorP11QQmlContextS3_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN13QQmlComponent6createER13QQmlIncubatorP11QQmlContextS3_(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QQmlComponent::QQmlComponent(QQmlEngine * , const QString & fileName, QObject * parent);
impl<'a> /*trait*/ QQmlComponent_new for (&'a QQmlEngine, &'a QString, &'a QObject) {
  fn new(self) -> QQmlComponent {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QQmlComponentC2EP10QQmlEngineRK7QStringP7QObject()};
    let ctysz: c_int = unsafe{QQmlComponent_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    unsafe {_ZN13QQmlComponentC2EP10QQmlEngineRK7QStringP7QObject(qthis_ph, arg0, arg1, arg2)};
    let qthis: u64 = qthis_ph;
    let rsthis = QQmlComponent{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QQmlComponent::isNull();
impl /*struct*/ QQmlComponent {
  pub fn isNull<RetType, T: QQmlComponent_isNull<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isNull(self);
    // return 1;
  }
}

pub trait QQmlComponent_isNull<RetType> {
  fn isNull(self , rsthis: & QQmlComponent) -> RetType;
}

  // proto:  bool QQmlComponent::isNull();
impl<'a> /*trait*/ QQmlComponent_isNull<i8> for () {
  fn isNull(self , rsthis: & QQmlComponent) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QQmlComponent6isNullEv()};
    let mut ret = unsafe {_ZNK13QQmlComponent6isNullEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

#[derive(Default)] // for QQmlComponent_progressChanged
pub struct QQmlComponent_progressChanged_signal{poi:u64}
impl /* struct */ QQmlComponent {
  pub fn progressChanged(&self) -> QQmlComponent_progressChanged_signal {
     return QQmlComponent_progressChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QQmlComponent_progressChanged_signal {
  pub fn connect<T: QQmlComponent_progressChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QQmlComponent_progressChanged_signal_connect {
  fn connect(self, sigthis: QQmlComponent_progressChanged_signal);
}

#[derive(Default)] // for QQmlComponent_statusChanged
pub struct QQmlComponent_statusChanged_signal{poi:u64}
impl /* struct */ QQmlComponent {
  pub fn statusChanged(&self) -> QQmlComponent_statusChanged_signal {
     return QQmlComponent_statusChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QQmlComponent_statusChanged_signal {
  pub fn connect<T: QQmlComponent_statusChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QQmlComponent_statusChanged_signal_connect {
  fn connect(self, sigthis: QQmlComponent_statusChanged_signal);
}

// progressChanged(qreal)
extern fn QQmlComponent_progressChanged_signal_connect_cb_0(rsfptr:fn(f64), arg0: c_double) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as f64;
  rsfptr(rsarg0);
}
extern fn QQmlComponent_progressChanged_signal_connect_cb_box_0(rsfptr_raw:*mut Box<Fn(f64)>, arg0: c_double) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as f64;
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QQmlComponent_progressChanged_signal_connect for fn(f64) {
  fn connect(self, sigthis: QQmlComponent_progressChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QQmlComponent_progressChanged_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QQmlComponent_SlotProxy_connect__ZN13QQmlComponent15progressChangedEd(arg0, arg1, arg2)};
  }
}
impl /* trait */ QQmlComponent_progressChanged_signal_connect for Box<Fn(f64)> {
  fn connect(self, sigthis: QQmlComponent_progressChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QQmlComponent_progressChanged_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QQmlComponent_SlotProxy_connect__ZN13QQmlComponent15progressChangedEd(arg0, arg1, arg2)};
  }
}
// <= body block end

