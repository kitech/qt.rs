// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstring::QString;
use super::qurl::QUrl;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QFileOpenEvent::NewQFileOpenEvent(const QString & file);
  fn _ZN14QFileOpenEventC1ERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QFileOpenEvent::FreeQFileOpenEvent();
  fn _ZN14QFileOpenEventD0Ev(qthis: *mut c_void) ;
  // proto:  void QFileOpenEvent::NewQFileOpenEvent(const QUrl & url);
  fn _ZN14QFileOpenEventC1ERK4QUrl(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QString QFileOpenEvent::file();
  fn _ZNK14QFileOpenEvent4fileEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QUrl QFileOpenEvent::url();
  fn _ZNK14QFileOpenEvent3urlEv(qthis: *mut c_void) -> *mut c_void;
}

// body block begin
// class sizeof(QFileOpenEvent)=40
pub struct QFileOpenEvent {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QFileOpenEvent {
  pub fn NewQFileOpenEvent<T: QFileOpenEvent_NewQFileOpenEvent>(value: T) -> QFileOpenEvent {
    let rsthis = value.NewQFileOpenEvent();
    return rsthis;
    // return 1;
  }
}

pub trait QFileOpenEvent_NewQFileOpenEvent {
  fn NewQFileOpenEvent(self) -> QFileOpenEvent;
}

// proto: void QFileOpenEvent::NewQFileOpenEvent(const QString & file);
impl<'a> /*trait*/ QFileOpenEvent_NewQFileOpenEvent for (&'a  QString) {
  fn NewQFileOpenEvent(self) -> QFileOpenEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZN14QFileOpenEventC1ERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN14QFileOpenEventC1ERK7QString(qthis, arg0)};
    let rsthis = QFileOpenEvent{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QFileOpenEvent {
  pub fn FreeQFileOpenEvent<T: QFileOpenEvent_FreeQFileOpenEvent>(&mut self, value: T)  {
     value.FreeQFileOpenEvent(self);
    // return 1;
  }
}

pub trait QFileOpenEvent_FreeQFileOpenEvent {
  fn FreeQFileOpenEvent(self, rsthis: &mut QFileOpenEvent) ;
}

// proto:  void QFileOpenEvent::FreeQFileOpenEvent();
impl<'a> /*trait*/ QFileOpenEvent_FreeQFileOpenEvent for () {
  fn FreeQFileOpenEvent(self, rsthis: &mut QFileOpenEvent)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZN14QFileOpenEventD0Ev()};
     unsafe {_ZN14QFileOpenEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// proto: void QFileOpenEvent::NewQFileOpenEvent(const QUrl & url);
impl<'a> /*trait*/ QFileOpenEvent_NewQFileOpenEvent for (&'a  QUrl) {
  fn NewQFileOpenEvent(self) -> QFileOpenEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZN14QFileOpenEventC1ERK4QUrl()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN14QFileOpenEventC1ERK4QUrl(qthis, arg0)};
    let rsthis = QFileOpenEvent{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QFileOpenEvent {
  pub fn file<T: QFileOpenEvent_file>(&mut self, value: T) -> QString {
    return value.file(self);
    // return 1;
  }
}

pub trait QFileOpenEvent_file {
  fn file(self, rsthis: &mut QFileOpenEvent) -> QString;
}

// proto:  QString QFileOpenEvent::file();
impl<'a> /*trait*/ QFileOpenEvent_file for () {
  fn file(self, rsthis: &mut QFileOpenEvent) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZNK14QFileOpenEvent4fileEv()};
    let mut ret = unsafe {_ZNK14QFileOpenEvent4fileEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QFileOpenEvent {
  pub fn url<T: QFileOpenEvent_url>(&mut self, value: T) -> QUrl {
    return value.url(self);
    // return 1;
  }
}

pub trait QFileOpenEvent_url {
  fn url(self, rsthis: &mut QFileOpenEvent) -> QUrl;
}

// proto:  QUrl QFileOpenEvent::url();
impl<'a> /*trait*/ QFileOpenEvent_url for () {
  fn url(self, rsthis: &mut QFileOpenEvent) -> QUrl {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZNK14QFileOpenEvent3urlEv()};
    let mut ret = unsafe {_ZNK14QFileOpenEvent3urlEv(rsthis.qclsinst)};
    let mut ret1 = QUrl{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

